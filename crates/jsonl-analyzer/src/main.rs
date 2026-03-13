//! Analyzes Claude Code JSONL transcript files and produces documentation
//! of all node types, their field schemas, and the structural patterns observed.
//!
//! Designed for streaming — processes files of any size in a single pass
//! with bounded memory usage.

use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// Maximum number of distinct keys before an object is treated as a dynamic map.
const MAX_STRUCT_KEYS: usize = 40;

/// Tracks the observed JSON schema for a value position across many instances.
#[derive(Debug, Clone)]
struct FieldSchema {
    /// JSON types seen at this position ("string", "number", "boolean", "null", "array", "object")
    types_seen: BTreeSet<String>,
    /// For string fields: sample distinct values (capped)
    string_samples: BTreeSet<String>,
    /// Whether this field has been seen with a string value that looks like free-form text (long/variable)
    is_freeform_string: bool,
    /// For object fields: nested field schemas (only when `is_dynamic_map` is false)
    object_fields: BTreeMap<String, FieldSchema>,
    /// When true, this object has high-cardinality keys (dynamic map, not a struct).
    /// `map_value_schema` holds the merged schema of all values.
    is_dynamic_map: bool,
    /// Merged schema of all values when this is a dynamic map.
    map_value_schema: Option<Box<FieldSchema>>,
    /// For array fields: element schema (merged across all elements)
    array_element: Option<Box<FieldSchema>>,
    /// How many times this field was present
    present_count: u64,
    /// How many times this field was absent (for optional detection)
    absent_count: u64,
}

/// Detect strings that look like IDs/UUIDs/hashes rather than meaningful enum values.
fn looks_like_id(s: &str) -> bool {
    // UUIDs: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    if s.len() == 36
        && s.chars()
            .enumerate()
            .all(|(i, c)| matches!(i, 8 | 13 | 18 | 23) && c == '-' || c.is_ascii_hexdigit())
    {
        return true;
    }
    // Tool use IDs: toolu_*, srvtoolu_*
    if s.starts_with("toolu_") || s.starts_with("srvtoolu_") {
        return true;
    }
    // Message IDs: msg_*
    if s.starts_with("msg_") || s.starts_with("req_") {
        return true;
    }
    false
}

impl FieldSchema {
    fn new() -> Self {
        Self {
            types_seen: BTreeSet::new(),
            string_samples: BTreeSet::new(),
            is_freeform_string: false,
            object_fields: BTreeMap::new(),
            is_dynamic_map: false,
            map_value_schema: None,
            array_element: None,
            present_count: 0,
            absent_count: 0,
        }
    }

    fn record_absent(&mut self) {
        self.absent_count += 1;
    }

    /// Detect whether an object's keys look like dynamic/data keys vs fixed struct fields.
    fn looks_dynamic(map: &serde_json::Map<String, Value>) -> bool {
        // Any key longer than 60 chars is almost certainly data, not a field name
        if map.keys().any(|k| k.len() > 60) {
            return true;
        }
        // Keys with spaces are data (question text, etc)
        if map.keys().any(|k| k.contains(' ')) {
            return true;
        }
        false
    }

    fn record_value(&mut self, value: &Value, depth: usize) {
        self.present_count += 1;

        match value {
            Value::Null => {
                self.types_seen.insert("null".into());
            }
            Value::Bool(_) => {
                self.types_seen.insert("boolean".into());
            }
            Value::Number(n) => {
                if n.is_f64() && !n.is_i64() && !n.is_u64() {
                    self.types_seen.insert("float".into());
                } else {
                    self.types_seen.insert("integer".into());
                }
            }
            Value::String(s) => {
                self.types_seen.insert("string".into());
                if s.len() > 100 || s.contains('\n') || looks_like_id(s) {
                    self.is_freeform_string = true;
                } else if self.string_samples.len() < 50 {
                    self.string_samples.insert(s.clone());
                }
            }
            Value::Array(arr) => {
                self.types_seen.insert("array".into());
                if depth < MAX_DEPTH {
                    let elem = self
                        .array_element
                        .get_or_insert_with(|| Box::new(FieldSchema::new()));
                    for item in arr {
                        elem.record_value(item, depth + 1);
                    }
                }
            }
            Value::Object(map) => {
                self.types_seen.insert("object".into());
                if depth < MAX_DEPTH {
                    // Check if this object (or any prior instance) has dynamic keys
                    if self.is_dynamic_map || Self::looks_dynamic(map) {
                        // Treat as Map<string, ValueSchema> — merge all values into one schema
                        self.is_dynamic_map = true;
                        self.object_fields.clear(); // discard any struct fields from before
                        let val_schema = self
                            .map_value_schema
                            .get_or_insert_with(|| Box::new(FieldSchema::new()));
                        for v in map.values() {
                            val_schema.record_value(v, depth + 1);
                        }
                    } else {
                        // Fixed struct keys
                        let all_keys: BTreeSet<String> = self
                            .object_fields
                            .keys()
                            .cloned()
                            .chain(map.keys().cloned())
                            .collect();

                        // Promote to dynamic map if key cardinality is too high
                        if all_keys.len() > MAX_STRUCT_KEYS {
                            self.is_dynamic_map = true;
                            let val_schema = self
                                .map_value_schema
                                .get_or_insert_with(|| Box::new(FieldSchema::new()));
                            // Merge existing field schemas' type info
                            for field in self.object_fields.values() {
                                merge_types_into(val_schema, field);
                            }
                            self.object_fields.clear();
                            for v in map.values() {
                                val_schema.record_value(v, depth + 1);
                            }
                        } else {
                            for key in &all_keys {
                                let field = self
                                    .object_fields
                                    .entry(key.clone())
                                    .or_insert_with(FieldSchema::new);
                                match map.get(key) {
                                    Some(v) => field.record_value(v, depth + 1),
                                    None => field.record_absent(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Merge type information from `src` into `dst` (used when promoting struct to dynamic map).
fn merge_types_into(dst: &mut FieldSchema, src: &FieldSchema) {
    for t in &src.types_seen {
        dst.types_seen.insert(t.clone());
    }
    if src.is_freeform_string {
        dst.is_freeform_string = true;
    }
}

const MAX_DEPTH: usize = 6;
const MAX_SAMPLES_PER_CATEGORY: usize = 3;

/// A categorized node type with its schema and sample instances.
struct NodeCategory {
    /// Description key (e.g. "assistant", "progress:bash_progress")
    key: String,
    /// Merged field schema across all instances
    schema: FieldSchema,
    /// Count of instances seen
    count: u64,
    /// A few sample JSON objects (kept small)
    samples: Vec<Value>,
}

impl NodeCategory {
    fn new(key: String) -> Self {
        Self {
            key,
            schema: FieldSchema::new(),
            count: 0,
            samples: Vec::new(),
        }
    }

    fn record(&mut self, value: &Value) {
        self.count += 1;
        self.schema.record_value(value, 0);
        if self.samples.len() < MAX_SAMPLES_PER_CATEGORY {
            // Store a truncated sample (strip large string values)
            self.samples.push(truncate_sample(value, 0));
        }
    }
}

/// Classify a JSON line into a category key.
fn classify(value: &Value) -> String {
    let top_type = value
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    match top_type {
        "progress" => {
            let data_type = value
                .get("data")
                .and_then(|d| d.get("type"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            format!("progress:{data_type}")
        }
        "assistant" => {
            // Sub-classify by content type pattern
            let content = value
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_array());
            if let Some(items) = content {
                let has_type = |ty: &str| {
                    items
                        .iter()
                        .any(|i| i.get("type").and_then(|t| t.as_str()) == Some(ty))
                };

                if has_type("thinking") {
                    "assistant:thinking".into()
                } else if has_type("tool_use") && has_type("text") {
                    "assistant:text+tool_use".into()
                } else if has_type("tool_use") {
                    "assistant:tool_use".into()
                } else if has_type("text") {
                    "assistant:text".into()
                } else {
                    "assistant:other".into()
                }
            } else {
                "assistant:empty".into()
            }
        }
        "user" => {
            let msg = value.get("message");
            let content = msg.and_then(|m| m.get("content"));
            match content {
                Some(Value::String(_)) => "user:text".into(),
                Some(Value::Array(arr)) => {
                    let types: BTreeSet<String> = arr
                        .iter()
                        .filter_map(|i| i.get("type").and_then(|t| t.as_str()).map(String::from))
                        .collect();
                    if types.len() == 1 {
                        format!("user:{}", types.iter().next().unwrap())
                    } else {
                        let joined: Vec<&str> = types.iter().map(|s| s.as_str()).collect();
                        format!("user:{}", joined.join("+"))
                    }
                }
                _ => "user:other".into(),
            }
        }
        "system" => {
            let subtype = value
                .get("subtype")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            format!("system:{subtype}")
        }
        other => other.to_string(),
    }
}

/// Create a truncated copy of a JSON value for sample storage.
fn truncate_sample(value: &Value, depth: usize) -> Value {
    if depth > 4 {
        return Value::String("[...]".into());
    }
    match value {
        Value::String(s) if s.len() > 120 => {
            // Find a char boundary at or before byte 80 to avoid panicking on multi-byte UTF-8
            let truncate_at = s.floor_char_boundary(80);
            Value::String(format!("{}... [{}b total]", &s[..truncate_at], s.len()))
        }
        Value::Array(arr) if arr.len() > 5 => {
            let mut truncated: Vec<Value> = arr
                .iter()
                .take(3)
                .map(|v| truncate_sample(v, depth + 1))
                .collect();
            truncated.push(Value::String(format!("[...{} more items]", arr.len() - 3)));
            Value::Array(truncated)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| truncate_sample(v, depth + 1)).collect())
        }
        Value::Object(map) => {
            let truncated: serde_json::Map<String, Value> = map
                .iter()
                .map(|(k, v)| (k.clone(), truncate_sample(v, depth + 1)))
                .collect();
            Value::Object(truncated)
        }
        other => other.clone(),
    }
}

/// Collect all field paths from a schema into a flat set (e.g. "message.content.type").
fn collect_field_paths(schema: &FieldSchema, prefix: &str, paths: &mut BTreeSet<String>) {
    if schema.types_seen.contains("object") {
        if schema.is_dynamic_map {
            // Dynamic map: record a single "{}" path for the map itself
            let map_path = if prefix.is_empty() {
                "{}".to_string()
            } else {
                format!("{prefix}.{{}}")
            };
            paths.insert(map_path.clone());
            if let Some(val_schema) = &schema.map_value_schema {
                collect_field_paths(val_schema, &map_path, paths);
            }
        } else {
            for (key, field) in &schema.object_fields {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                paths.insert(path.clone());
                collect_field_paths(field, &path, paths);
            }
        }
    }
    if schema.types_seen.contains("array")
        && let Some(elem) = &schema.array_element
    {
        let arr_prefix = if prefix.is_empty() {
            "[]".to_string()
        } else {
            format!("{prefix}.[]")
        };
        collect_field_paths(elem, &arr_prefix, paths);
    }
}

/// A snapshot of known categories and their field paths, for diffing across runs.
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct KnownSchema {
    /// Map of category key -> set of dotted field paths
    categories: BTreeMap<String, BTreeSet<String>>,
}

/// Render the schema tree as markdown documentation.
/// If `known_fields` is provided, fields not in that set get a `[NEW]` marker.
fn render_schema(
    schema: &FieldSchema,
    indent: usize,
    path_prefix: &str,
    known_fields: Option<&BTreeSet<String>>,
) -> String {
    let mut out = String::new();
    let prefix = "  ".repeat(indent);

    if schema.types_seen.contains("object") {
        if schema.is_dynamic_map {
            // Render as Map<string, ValueType>
            if let Some(val_schema) = &schema.map_value_schema {
                let val_types: Vec<&str> =
                    val_schema.types_seen.iter().map(|s| s.as_str()).collect();
                let val_type_str = val_types.join(" | ");
                out.push_str(&format!(
                    "{prefix}_Dynamic map:_ `Map<string, {val_type_str}>`\n"
                ));
                // If the value is itself a struct, show its fields
                if val_schema.types_seen.contains("object")
                    && !val_schema.object_fields.is_empty()
                    && !val_schema.is_dynamic_map
                {
                    let map_path = if path_prefix.is_empty() {
                        "{}".to_string()
                    } else {
                        format!("{path_prefix}.{{}}")
                    };
                    out.push_str(&format!("{prefix}  Value fields:\n"));
                    out.push_str(&render_schema(
                        val_schema,
                        indent + 2,
                        &map_path,
                        known_fields,
                    ));
                }
            }
        } else if !schema.object_fields.is_empty() {
            for (key, field) in &schema.object_fields {
                let types: Vec<&str> = field.types_seen.iter().map(|s| s.as_str()).collect();
                let type_str = types.join(" | ");
                let optional = if field.absent_count > 0 { "?" } else { "" };

                let field_path = if path_prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{path_prefix}.{key}")
                };
                let new_marker = match known_fields {
                    Some(known) if !known.contains(&field_path) => " **[NEW]**",
                    _ => "",
                };

                out.push_str(&format!(
                    "{prefix}- **`{key}`**{optional}: `{type_str}`{new_marker}"
                ));

                // Show enum-like values for string fields with limited distinct values
                if field.types_seen.contains("string")
                    && !field.is_freeform_string
                    && !field.string_samples.is_empty()
                    && field.string_samples.len() <= 30
                {
                    let vals: Vec<String> = field
                        .string_samples
                        .iter()
                        .map(|s| format!("`\"{s}\"`"))
                        .collect();
                    out.push_str(&format!(" — values: {}", vals.join(", ")));
                } else if field.is_freeform_string {
                    out.push_str(" — free-form text");
                }

                out.push('\n');

                // Recurse into objects
                if field.types_seen.contains("object")
                    && (!field.object_fields.is_empty() || field.is_dynamic_map)
                {
                    out.push_str(&render_schema(field, indent + 1, &field_path, known_fields));
                }

                // Recurse into array elements
                if field.types_seen.contains("array")
                    && let Some(elem) = &field.array_element
                {
                    let arr_path = format!("{field_path}.[]");
                    if elem.types_seen.contains("object")
                        && (!elem.object_fields.is_empty() || elem.is_dynamic_map)
                    {
                        out.push_str(&format!("{prefix}  Array element:\n"));
                        out.push_str(&render_schema(elem, indent + 2, &arr_path, known_fields));
                    } else {
                        let elem_types: Vec<&str> =
                            elem.types_seen.iter().map(|s| s.as_str()).collect();
                        out.push_str(&format!(
                            "{prefix}  Array of: `{}`\n",
                            elem_types.join(" | ")
                        ));
                    }
                }
            }
        }
    }

    out
}

/// Recursively find all `.jsonl` files under a directory.
fn find_jsonl_files(dir: &std::path::Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(find_jsonl_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "jsonl") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// Process a single JSONL file, accumulating into shared state.
fn process_file(
    path: &std::path::Path,
    categories: &mut BTreeMap<String, NodeCategory>,
    total_lines: &mut u64,
    total_parse_errors: &mut u64,
) -> io::Result<()> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut file_lines: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        *total_lines += 1;
        file_lines += 1;

        if (*total_lines).is_multiple_of(500_000) {
            eprint!("\r  Processed {} lines...", total_lines);
        }

        if line.trim().is_empty() {
            continue;
        }

        let value: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                if *total_parse_errors < 5 {
                    eprintln!(
                        "  Parse error in {} line {}: {}",
                        path.display(),
                        file_lines,
                        e
                    );
                }
                *total_parse_errors += 1;
                continue;
            }
        };

        let key = classify(&value);
        let category = categories
            .entry(key)
            .or_insert_with_key(|k| NodeCategory::new(k.clone()));
        category.record(&value);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: jsonl-analyzer <path> [output.md]");
        eprintln!();
        eprintln!("  <path>       A .jsonl file or a directory to scan recursively");
        eprintln!("  [output.md]  Output file (default: <input>.md or transcript-schema.md)");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  jsonl-analyzer ~/.claude/projects/              # scan all transcripts");
        eprintln!("  jsonl-analyzer session.jsonl                    # single file");
        eprintln!("  jsonl-analyzer ~/.claude/projects/ schema.md    # custom output path");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);

    // Collect files to process
    let files: Vec<PathBuf> = if input_path.is_dir() {
        let found = find_jsonl_files(&input_path)?;
        if found.is_empty() {
            eprintln!("No .jsonl files found under {}", input_path.display());
            std::process::exit(1);
        }
        eprintln!(
            "Found {} .jsonl files under {}",
            found.len(),
            input_path.display()
        );
        found
    } else {
        vec![input_path.clone()]
    };

    let output_path = args.get(2).map(PathBuf::from).unwrap_or_else(|| {
        if input_path.is_dir() {
            PathBuf::from("transcript-schema.md")
        } else {
            let mut p = input_path.clone();
            p.set_extension("md");
            p
        }
    });

    let mut categories: BTreeMap<String, NodeCategory> = BTreeMap::new();
    let mut total_lines: u64 = 0;
    let mut total_parse_errors: u64 = 0;
    let file_count = files.len();

    for (i, file_path) in files.iter().enumerate() {
        eprintln!("[{}/{}] {}", i + 1, file_count, file_path.display());
        process_file(
            file_path,
            &mut categories,
            &mut total_lines,
            &mut total_parse_errors,
        )?;
    }

    eprintln!(
        "\r  Processed {} lines across {} files ({} parse errors)",
        total_lines, file_count, total_parse_errors
    );
    eprintln!("Writing: {}", output_path.display());

    // Load previous known schema for diffing (if it exists)
    let schema_json_path = output_path.with_extension("json");
    let previous_schema: Option<KnownSchema> = std::fs::read_to_string(&schema_json_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok());

    let has_previous = previous_schema.is_some();
    if has_previous {
        eprintln!(
            "  Loaded previous schema from {} for diffing",
            schema_json_path.display()
        );
    }

    // Build current known schema for saving
    let mut current_schema = KnownSchema::default();
    for cat in categories.values() {
        let mut paths = BTreeSet::new();
        collect_field_paths(&cat.schema, "", &mut paths);
        current_schema.categories.insert(cat.key.clone(), paths);
    }

    // Count new categories and fields for the summary
    let mut new_categories: Vec<String> = Vec::new();
    let mut new_field_count: usize = 0;
    if let Some(ref prev) = previous_schema {
        for (cat_key, cat_fields) in &current_schema.categories {
            if !prev.categories.contains_key(cat_key) {
                new_categories.push(cat_key.clone());
            } else {
                let prev_fields = &prev.categories[cat_key];
                new_field_count += cat_fields
                    .iter()
                    .filter(|f| !prev_fields.contains(*f))
                    .count();
            }
        }
    }

    // Generate markdown documentation
    let mut out = std::fs::File::create(&output_path)?;

    writeln!(out, "# Claude Code JSONL Transcript Node Types")?;
    writeln!(out)?;
    if file_count == 1 {
        writeln!(
            out,
            "Auto-generated schema documentation from `{}` ({} lines).",
            files[0].file_name().unwrap().to_string_lossy(),
            total_lines
        )?;
    } else {
        writeln!(
            out,
            "Auto-generated schema documentation from {} files ({} lines total).",
            file_count, total_lines
        )?;
    }
    writeln!(out)?;
    writeln!(
        out,
        "Each line in a Claude Code JSONL transcript is a JSON object with a `type` field"
    )?;
    writeln!(
        out,
        "that determines its structure. This document describes every node type observed,"
    )?;
    writeln!(out, "its fields, value types, and representative samples.")?;
    writeln!(out)?;

    if has_previous {
        if new_categories.is_empty() && new_field_count == 0 {
            writeln!(
                out,
                "> Compared with previous schema: **no changes detected.**"
            )?;
        } else {
            write!(out, "> Compared with previous schema:")?;
            if !new_categories.is_empty() {
                write!(out, " **{} new category(ies)**", new_categories.len())?;
            }
            if new_field_count > 0 {
                if !new_categories.is_empty() {
                    write!(out, ",")?;
                }
                write!(out, " **{new_field_count} new field(s)**")?;
            }
            writeln!(
                out,
                ". Items marked **[NEW]** were not in the previous run."
            )?;
        }
        writeln!(out)?;
    }

    // Summary table
    writeln!(out, "## Summary")?;
    writeln!(out)?;
    writeln!(out, "| Category | Count | Description |")?;
    writeln!(out, "|----------|------:|-------------|")?;
    for cat in categories.values() {
        let desc = category_description(&cat.key);
        let new_marker = if new_categories.contains(&cat.key) {
            " **[NEW]**"
        } else {
            ""
        };
        writeln!(
            out,
            "| `{}`{} | {} | {} |",
            cat.key, new_marker, cat.count, desc
        )?;
    }
    writeln!(out)?;

    // Detailed sections for each category
    for cat in categories.values() {
        let is_new_cat = new_categories.contains(&cat.key);
        let new_cat_marker = if is_new_cat { " **[NEW]**" } else { "" };

        writeln!(out, "---")?;
        writeln!(out)?;
        writeln!(out, "## `{}`{}", cat.key, new_cat_marker)?;
        writeln!(out)?;
        writeln!(out, "**Count:** {} instances", cat.count)?;
        writeln!(out)?;
        writeln!(out, "{}", category_description(&cat.key))?;
        writeln!(out)?;

        // Schema — pass known fields for diff markers (skip if entire category is new)
        let known_fields = if is_new_cat {
            None
        } else {
            previous_schema
                .as_ref()
                .and_then(|prev| prev.categories.get(&cat.key))
        };

        writeln!(out, "### Fields")?;
        writeln!(out)?;
        let schema_md = render_schema(&cat.schema, 0, "", known_fields);
        if schema_md.is_empty() {
            writeln!(out, "_No object fields observed._")?;
        } else {
            write!(out, "{schema_md}")?;
        }
        writeln!(out)?;

        // Samples
        writeln!(out, "### Sample")?;
        writeln!(out)?;
        if let Some(sample) = cat.samples.first() {
            writeln!(out, "```json")?;
            writeln!(out, "{}", serde_json::to_string_pretty(sample).unwrap())?;
            writeln!(out, "```")?;
        }
        writeln!(out)?;
    }

    // Save current schema for future diffing
    let schema_json =
        serde_json::to_string_pretty(&current_schema).expect("failed to serialize schema");
    std::fs::write(&schema_json_path, schema_json)?;
    eprintln!("  Saved schema snapshot to {}", schema_json_path.display());

    eprintln!("Done. {} categories documented.", categories.len());

    Ok(())
}

/// Provide a human-readable description for each category.
fn category_description(key: &str) -> &'static str {
    match key {
        "assistant:text" => "Assistant response containing only text content",
        "assistant:tool_use" => "Assistant response containing only a tool invocation",
        "assistant:text+tool_use" => "Assistant response with both text and a tool invocation",
        "assistant:thinking" => "Assistant response that includes a thinking/reasoning block",
        "assistant:other" => "Assistant response with unrecognized content types",
        "assistant:empty" => "Assistant response with no content array",

        "user:text" => "User message with plain text content (string)",
        "user:tool_result" => "Tool execution result returned to the assistant",
        "user:image" => "User message containing an image",
        "user:text+tool_result" => "User message with both text and tool result content",
        "user:image+text" => "User message with image and text content",
        "user:other" => "User message with unrecognized content structure",

        "progress:bash_progress" => "Streaming output from a running Bash command",
        "progress:hook_progress" => "Progress update from a running hook script",
        "progress:agent_progress" => "Progress update from a spawned sub-agent",
        "progress:mcp_progress" => "Progress update from an MCP tool call",
        "progress:waiting_for_task" => "Indicator that the system is waiting for a background task",
        "progress:query_update" => "Server-side web search query update during tool use",
        "progress:search_results_received" => {
            "Server-side web search results received notification"
        }

        "system:turn_duration" => "Records the wall-clock duration of an assistant turn",
        "system:stop_hook_summary" => "Summary of post-turn hook execution results",
        "system:compact_boundary" => "Marks where conversation context was compacted",
        "system:local_command" => "A local slash command invoked by the user",
        "system:api_error" => "API-level error from the Claude service",
        "system:informational" => "Informational system message (e.g. session metadata)",

        "file-history-snapshot" => "Snapshot of tracked file states for undo/restore",
        "queue-operation" => "Background task queue operation (enqueue, dequeue, remove)",
        "last-prompt" => "Records the last user prompt for session resumption",
        "pr-link" => "Links a session to a GitHub pull request",

        _ => "Uncategorized node type",
    }
}
