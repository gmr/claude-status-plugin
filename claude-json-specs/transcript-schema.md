# Claude Code JSONL Transcript Node Types

Auto-generated schema documentation from 1039 files (241894 lines total).

Each line in a Claude Code JSONL transcript is a JSON object with a `type` field
that determines its structure. This document describes every node type observed,
its fields, value types, and representative samples.

## Summary

| Category | Count | Description |
|----------|------:|-------------|
| `assistant:text` | 20019 | Assistant response containing only text content |
| `assistant:text+tool_use` | 3 | Assistant response with both text and a tool invocation |
| `assistant:thinking` | 1556 | Assistant response that includes a thinking/reasoning block |
| `assistant:tool_use` | 37837 | Assistant response containing only a tool invocation |
| `file-history-snapshot` | 3179 | Snapshot of tracked file states for undo/restore |
| `last-prompt` | 121 | Records the last user prompt for session resumption |
| `pr-link` | 54 | Links a session to a GitHub pull request |
| `progress:agent_progress` | 16558 | Progress update from a spawned sub-agent |
| `progress:bash_progress` | 81339 | Streaming output from a running Bash command |
| `progress:hook_progress` | 35296 | Progress update from a running hook script |
| `progress:mcp_progress` | 820 | Progress update from an MCP tool call |
| `progress:query_update` | 12 | Server-side web search query update during tool use |
| `progress:search_results_received` | 12 | Server-side web search results received notification |
| `progress:waiting_for_task` | 18 | Indicator that the system is waiting for a background task |
| `queue-operation` | 1779 | Background task queue operation (enqueue, dequeue, remove) |
| `system:api_error` | 23 | API-level error from the Claude service |
| `system:compact_boundary` | 58 | Marks where conversation context was compacted |
| `system:informational` | 2 | Informational system message (e.g. session metadata) |
| `system:local_command` | 46 | A local slash command invoked by the user |
| `system:stop_hook_summary` | 577 | Summary of post-turn hook execution results |
| `system:turn_duration` | 929 | Records the wall-clock duration of an assistant turn |
| `user:image` | 3 | User message containing an image |
| `user:image+text` | 51 | User message with image and text content |
| `user:text` | 3764 | User message with plain text content (string) |
| `user:text+tool_result` | 42 | User message with both text and tool result content |
| `user:tool_result` | 37796 | Tool execution result returned to the assistant |

---

## `assistant:text`

**Count:** 20019 instances

Assistant response containing only text content

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`error`**?: `string` — values: `"invalid_request"`, `"unknown"`
- **`gitBranch`**: `string`
- **`isApiErrorMessage`**?: `boolean`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`container`**?: `null`
  - **`content`**: `array`
    Array element:
      - **`text`**: `string` — free-form text
      - **`type`**: `string` — values: `"text"`
  - **`context_management`**?: `null | object`
    - **`applied_edits`**: `array`
      Array of: ``
  - **`id`**: `string` — free-form text
  - **`model`**: `string` — values: `"<synthetic>"`, `"claude-haiku-4-5-20251001"`, `"claude-opus-4-5-20251101"`, `"claude-opus-4-6"`, `"claude-sonnet-4-5-20250929"`, `"claude-sonnet-4-6"`
  - **`role`**: `string` — values: `"assistant"`
  - **`stop_reason`**: `null | string` — values: `"end_turn"`, `"stop_sequence"`
  - **`stop_sequence`**: `null | string` — values: `""`
  - **`type`**: `string` — values: `"message"`
  - **`usage`**: `object`
    - **`cache_creation`**: `object`
      - **`ephemeral_1h_input_tokens`**: `integer`
      - **`ephemeral_5m_input_tokens`**: `integer`
    - **`cache_creation_input_tokens`**: `integer`
    - **`cache_read_input_tokens`**: `integer`
    - **`inference_geo`**?: `null | string` — values: `""`, `"global"`, `"not_available"`
    - **`input_tokens`**: `integer`
    - **`iterations`**?: `array | null`
      Array of: ``
    - **`output_tokens`**: `integer`
    - **`server_tool_use`**?: `object`
      - **`web_fetch_requests`**: `integer`
      - **`web_search_requests`**: `integer`
    - **`service_tier`**: `null | string` — values: `"standard"`
    - **`speed`**?: `null | string` — values: `"standard"`
- **`parentUuid`**: `string` — free-form text
- **`requestId`**?: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"assistant"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "isSidechain": false,
  "message": {
    "content": [
      {
        "text": "\n\nI'll start by identifying the changes.",
        "type": "text"
      }
    ],
    "id": "msg_01Nj36dq6B8FQsc65wGkMqau",
    "model": "claude-opus-4-6",
    "role": "assistant",
    "stop_reason": null,
    "stop_sequence": null,
    "type": "message",
    "usage": {
      "cache_creation": {
        "ephemeral_1h_input_tokens": 19935,
        "ephemeral_5m_input_tokens": 0
      },
      "cache_creation_input_tokens": 19935,
      "cache_read_input_tokens": 0,
      "inference_geo": "not_available",
      "input_tokens": 2,
      "output_tokens": 2,
      "service_tier": "standard"
    }
  },
  "parentUuid": "d8551277-6864-4a04-929c-f3ec9e5cfa71",
  "requestId": "req_011CYytjRcVcCVeWJ91u1D4o",
  "sessionId": "2b9b74ae-8328-4e44-b5d8-9e4695afcd5d",
  "timestamp": "2026-03-12T21:25:53.969Z",
  "type": "assistant",
  "userType": "external",
  "uuid": "8d42f0bd-a86a-4b4a-b6a5-4754fd7b529b",
  "version": "2.1.71"
}
```

---

## `assistant:text+tool_use`

**Count:** 3 instances

Assistant response with both text and a tool invocation

### Fields

- **`agentId`**?: `string` — values: `"a69b0a451873226a9"`, `"a8e86f5"`
- **`cwd`**: `string` — values: `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/bots/aj/.claude/worktrees/abstract-snuggling-token"`, `"/Volumes/Source/gmr/claude-status"`
- **`gitBranch`**: `string` — values: `"feature/channel-prompts"`, `"feature/question-detection-and-version-fixes"`, `"main"`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`caller`**?: `object`
        - **`type`**: `string` — values: `"direct"`
      - **`id`**?: `string` — free-form text
      - **`input`**?: `object`
        - **`command`**: `string` — free-form text
        - **`description`**: `string` — values: `"Check transcript end_turn messages for question marks"`
        - **`file_path`**?: `string` — free-form text
        - **`pattern`**?: `string` — values: `"**/*.py"`
      - **`name`**?: `string` — values: `"Bash"`, `"Glob"`, `"Read"`
      - **`text`**?: `string` — free-form text
      - **`type`**: `string` — values: `"text"`, `"tool_use"`
  - **`context_management`**: `object`
    - **`applied_edits`**: `array`
      Array of: ``
  - **`id`**: `string` — free-form text
  - **`model`**: `string` — values: `"claude-haiku-4-5-20251001"`, `"claude-opus-4-6"`
  - **`role`**: `string` — values: `"assistant"`
  - **`stop_reason`**: `string` — values: `"tool_use"`
  - **`stop_sequence`**: `null`
  - **`type`**: `string` — values: `"message"`
  - **`usage`**: `object`
    - **`cache_creation`**: `object`
      - **`ephemeral_1h_input_tokens`**: `integer`
      - **`ephemeral_5m_input_tokens`**: `integer`
    - **`cache_creation_input_tokens`**: `integer`
    - **`cache_read_input_tokens`**: `integer`
    - **`inference_geo`**: `string` — values: `"not_available"`
    - **`input_tokens`**: `integer`
    - **`output_tokens`**: `integer`
    - **`service_tier`**: `string` — values: `"standard"`
- **`parentUuid`**: `string` — free-form text
- **`requestId`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"abstract-snuggling-token"`, `"serialized-tumbling-stroustrup"`
- **`timestamp`**: `string` — values: `"2026-02-23T14:17:10.373Z"`, `"2026-03-13T14:57:11.777Z"`, `"2026-03-13T17:40:52.651Z"`
- **`type`**: `string` — values: `"assistant"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.47"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "agentId": "a8e86f5",
  "cwd": "/Volumes/Source/bots/aj",
  "gitBranch": "main",
  "isSidechain": true,
  "message": {
    "content": [
      {
        "text": "I'll search the codebase thoroughly to understand the question detection, NO-RESPONSE handling, and bot response flow.",
        "type": "text"
      },
      {
        "caller": {
          "type": "[...]"
        },
        "id": "toolu_01Mh75UpKfCYHa5gsTasYE6E",
        "input": {
          "pattern": "[...]"
        },
        "name": "Glob",
        "type": "tool_use"
      }
    ],
    "context_management": {
      "applied_edits": []
    },
    "id": "msg_018bRCnoZiLK8SX8KBLEWtnY",
    "model": "claude-haiku-4-5-20251001",
    "role": "assistant",
    "stop_reason": "tool_use",
    "stop_sequence": null,
    "type": "message",
    "usage": {
      "cache_creation": {
        "ephemeral_1h_input_tokens": 60524,
        "ephemeral_5m_input_tokens": 0
      },
      "cache_creation_input_tokens": 60524,
      "cache_read_input_tokens": 0,
      "inference_geo": "not_available",
      "input_tokens": 3,
      "output_tokens": 80,
      "service_tier": "standard"
    }
  },
  "parentUuid": "778ed40f-09e8-4e8d-8ed0-d55af4013ca3",
  "requestId": "req_011CYR9H224vjpq9AKtsuW4A",
  "sessionId": "b4d6c74f-ed70-4b16-be79-7cf789e9a4ca",
  "timestamp": "2026-02-23T14:17:10.373Z",
  "type": "assistant",
  "userType": "external",
  "uuid": "780259aa-c740-4762-b7da-188e8b36b9d3",
  "version": "2.1.47"
}
```

---

## `assistant:thinking`

**Count:** 1556 instances

Assistant response that includes a thinking/reasoning block

### Fields

- **`agentId`**?: `string` — values: `"acompact-10ea9408a00175a0"`, `"acompact-21da4b8d4ddbfcec"`, `"acompact-2946bd3e32d321d8"`, `"acompact-5ee709d13d9967c1"`, `"acompact-74af726bf45c73f9"`, `"acompact-74e55408baff24f1"`, `"acompact-7f5cf2a7f3ba6ad6"`, `"acompact-99ab86a367d94b2a"`, `"acompact-b3914f2f711b83cd"`, `"acompact-d9621f0a65e49e00"`, `"acompact-f538e286804c8ad1"`, `"acompact-f64ce760c8b83321"`, `"acompact-ff06765bb152628c"`, `"aside_question-ad48711b92cb30ef"`
- **`cwd`**: `string`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`signature`**: `string` — free-form text
      - **`thinking`**: `string` — free-form text
      - **`type`**: `string` — values: `"thinking"`
  - **`id`**: `string` — free-form text
  - **`model`**: `string` — values: `"claude-haiku-4-5-20251001"`, `"claude-opus-4-6"`, `"claude-sonnet-4-6"`
  - **`role`**: `string` — values: `"assistant"`
  - **`stop_reason`**: `null`
  - **`stop_sequence`**: `null`
  - **`type`**: `string` — values: `"message"`
  - **`usage`**: `object`
    - **`cache_creation`**: `object`
      - **`ephemeral_1h_input_tokens`**: `integer`
      - **`ephemeral_5m_input_tokens`**: `integer`
    - **`cache_creation_input_tokens`**: `integer`
    - **`cache_read_input_tokens`**: `integer`
    - **`inference_geo`**: `string` — values: `"not_available"`
    - **`input_tokens`**: `integer`
    - **`output_tokens`**: `integer`
    - **`service_tier`**: `string` — values: `"standard"`
- **`parentUuid`**: `string` — free-form text
- **`requestId`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"assistant"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.44"`, `"2.1.45"`, `"2.1.47"`, `"2.1.49"`, `"2.1.50"`, `"2.1.53"`, `"2.1.56"`, `"2.1.58"`, `"2.1.59"`, `"2.1.62"`, `"2.1.63"`, `"2.1.66"`, `"2.1.68"`, `"2.1.69"`, `"2.1.70"`, `"2.1.71"`, `"2.1.72"`, `"2.1.73"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "isSidechain": false,
  "message": {
    "content": [
      {
        "signature": "ErYCCkYICxgCKkC2CQVV2nuKYPck+V15TnkP9/6C8lMqowlYGBMW14yTPIMstIK+RvK7OkCkBs6J13fx... [420b total]",
        "thinking": "We're in /Users/gmr which is not a git repository. Let me check if there are rec... [141b total]",
        "type": "thinking"
      }
    ],
    "id": "msg_01YHbBQHo4XcLdqsXu1aLzT8",
    "model": "claude-opus-4-6",
    "role": "assistant",
    "stop_reason": null,
    "stop_sequence": null,
    "type": "message",
    "usage": {
      "cache_creation": {
        "ephemeral_1h_input_tokens": 127,
        "ephemeral_5m_input_tokens": 0
      },
      "cache_creation_input_tokens": 127,
      "cache_read_input_tokens": 19935,
      "inference_geo": "not_available",
      "input_tokens": 1,
      "output_tokens": 8,
      "service_tier": "standard"
    }
  },
  "parentUuid": "bffe38b4-8361-42f0-8fb6-0eb60eb8518b",
  "requestId": "req_011CYytjhKohjdtENvbvqmXV",
  "sessionId": "2b9b74ae-8328-4e44-b5d8-9e4695afcd5d",
  "timestamp": "2026-03-12T21:25:57.964Z",
  "type": "assistant",
  "userType": "external",
  "uuid": "2d2d757f-7cdb-40d1-af78-ad7a267023cc",
  "version": "2.1.71"
}
```

---

## `assistant:tool_use`

**Count:** 37837 instances

Assistant response containing only a tool invocation

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`caller`**?: `object`
        - **`type`**: `string` — values: `"direct"`
      - **`id`**: `string` — free-form text
      - **`input`**: `object`
        _Dynamic map:_ `Map<string, array | boolean | integer | string>`
      - **`name`**: `string`
      - **`type`**: `string` — values: `"tool_use"`
  - **`context_management`**?: `object`
    - **`applied_edits`**: `array`
      Array of: ``
  - **`id`**: `string` — free-form text
  - **`model`**: `string` — values: `"claude-haiku-4-5-20251001"`, `"claude-opus-4-5-20251101"`, `"claude-opus-4-6"`, `"claude-sonnet-4-5-20250929"`, `"claude-sonnet-4-6"`
  - **`role`**: `string` — values: `"assistant"`
  - **`stop_reason`**: `null | string` — values: `"tool_use"`
  - **`stop_sequence`**: `null`
  - **`type`**: `string` — values: `"message"`
  - **`usage`**: `object`
    - **`cache_creation`**: `object`
      - **`ephemeral_1h_input_tokens`**: `integer`
      - **`ephemeral_5m_input_tokens`**: `integer`
    - **`cache_creation_input_tokens`**: `integer`
    - **`cache_read_input_tokens`**: `integer`
    - **`inference_geo`**?: `string` — values: `""`, `"global"`, `"not_available"`
    - **`input_tokens`**: `integer`
    - **`iterations`**?: `array`
      Array of: ``
    - **`output_tokens`**: `integer`
    - **`server_tool_use`**?: `object`
      - **`web_fetch_requests`**: `integer`
      - **`web_search_requests`**: `integer`
    - **`service_tier`**: `string` — values: `"standard"`
    - **`speed`**?: `string` — values: `"standard"`
- **`parentUuid`**: `string` — free-form text
- **`requestId`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"assistant"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "isSidechain": false,
  "message": {
    "content": [
      {
        "caller": {
          "type": "[...]"
        },
        "id": "toolu_01Ky4ezCdB7CY8cnsHzfDxUU",
        "input": {
          "command": "[...]",
          "description": "[...]"
        },
        "name": "Bash",
        "type": "tool_use"
      }
    ],
    "id": "msg_01Nj36dq6B8FQsc65wGkMqau",
    "model": "claude-opus-4-6",
    "role": "assistant",
    "stop_reason": "tool_use",
    "stop_sequence": null,
    "type": "message",
    "usage": {
      "cache_creation": {
        "ephemeral_1h_input_tokens": 19935,
        "ephemeral_5m_input_tokens": 0
      },
      "cache_creation_input_tokens": 19935,
      "cache_read_input_tokens": 0,
      "inference_geo": "",
      "input_tokens": 2,
      "iterations": [],
      "output_tokens": 108,
      "server_tool_use": {
        "web_fetch_requests": 0,
        "web_search_requests": 0
      },
      "service_tier": "standard",
      "speed": "standard"
    }
  },
  "parentUuid": "8d42f0bd-a86a-4b4a-b6a5-4754fd7b529b",
  "requestId": "req_011CYytjRcVcCVeWJ91u1D4o",
  "sessionId": "2b9b74ae-8328-4e44-b5d8-9e4695afcd5d",
  "timestamp": "2026-03-12T21:25:54.583Z",
  "type": "assistant",
  "userType": "external",
  "uuid": "2122e954-bdb7-42ec-be10-285220a9eadd",
  "version": "2.1.71"
}
```

---

## `file-history-snapshot`

**Count:** 3179 instances

Snapshot of tracked file states for undo/restore

### Fields

- **`isSnapshotUpdate`**: `boolean`
- **`messageId`**: `string` — free-form text
- **`snapshot`**: `object`
  - **`messageId`**: `string` — free-form text
  - **`timestamp`**: `string`
  - **`trackedFileBackups`**: `object`
    _Dynamic map:_ `Map<string, object>`
      Value fields:
        - **`backupFileName`**: `null | string`
        - **`backupTime`**: `string`
        - **`version`**: `integer`
- **`type`**: `string` — values: `"file-history-snapshot"`

### Sample

```json
{
  "isSnapshotUpdate": false,
  "messageId": "60128b78-c3ff-4ac9-95d0-f9305e89af44",
  "snapshot": {
    "messageId": "60128b78-c3ff-4ac9-95d0-f9305e89af44",
    "timestamp": "2026-03-13T16:36:36.762Z",
    "trackedFileBackups": {}
  },
  "type": "file-history-snapshot"
}
```

---

## `last-prompt`

**Count:** 121 instances

Records the last user prompt for session resumption

### Fields

- **`lastPrompt`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`type`**: `string` — values: `"last-prompt"`

### Sample

```json
{
  "lastPrompt": "Sorry, this is different than the distribution certificate?",
  "sessionId": "bd7653e2-93a9-404e-be30-ad5718acbc1f",
  "type": "last-prompt"
}
```

---

## `pr-link`

**Count:** 54 instances

Links a session to a GitHub pull request

### Fields

- **`prNumber`**: `integer`
- **`prRepository`**: `string` — values: `"AWeber-Imbi/imbi"`, `"AWeber-Imbi/imbi-api"`, `"AWeber-Imbi/imbi-assistant"`, `"AWeber-Imbi/imbi-common"`, `"AWeber-Imbi/imbi-gateway"`, `"AWeber-Imbi/imbi-mcp"`, `"AWeber-Imbi/imbi-ui"`, `"gmr/aiorabbit"`, `"gmr/claude-status"`, `"gmr/email-normalize"`, `"gmr/flatdict"`
- **`prUrl`**: `string`
- **`sessionId`**: `string` — free-form text
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"pr-link"`

### Sample

```json
{
  "prNumber": 21,
  "prRepository": "gmr/aiorabbit",
  "prUrl": "https://github.com/gmr/aiorabbit/pull/21",
  "sessionId": "19f3a09b-39b4-4350-93a3-40a6b4a8794a",
  "timestamp": "2026-03-11T20:43:10.359Z",
  "type": "pr-link"
}
```

---

## `progress:agent_progress`

**Count:** 16558 instances

Progress update from a spawned sub-agent

### Fields

- **`agentId`**?: `string` — values: `"acompact-21da4b8d4ddbfcec"`, `"acompact-2946bd3e32d321d8"`, `"acompact-7f5cf2a7f3ba6ad6"`, `"acompact-99ab86a367d94b2a"`, `"acompact-d9621f0a65e49e00"`, `"acompact-f538e286804c8ad1"`, `"acompact-f64ce760c8b83321"`, `"acompact-ff06765bb152628c"`
- **`cwd`**: `string` — values: `"/Volumes/Source/backend-libraries/awatlassian"`, `"/Volumes/Source/backend-libraries/awpromptlayer"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/bots/aj-workflows"`, `"/Volumes/Source/bots/aj/.claude/worktrees/abstract-snuggling-token"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/config-management/eks"`, `"/Volumes/Source/consumers/clickhouse-materialization"`, `"/Volumes/Source/data-processing/bi-pipeline"`, `"/Volumes/Source/docker-images/claude"`, `"/Volumes/Source/gmr/aiorabbit"`, `"/Volumes/Source/gmr/awagent"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/gmr/email-normalize"`, `"/Volumes/Source/imbi"`, `"/Volumes/Source/imbi/features/admin-env-pt/imbi-ui"`, `"/Volumes/Source/imbi/features/ai-assistant/imbi-api"`, `"/Volumes/Source/imbi/features/ai-assistant/imbi-ui"`, `"/Volumes/Source/imbi/features/slim-shared-models/imbi-api"`, `"/Volumes/Source/imbi/features/slim-shared-models/imbi-common"`, `"/Volumes/Source/imbi/main"`, `"/Volumes/Source/imbi/main/imbi"`, `"/Volumes/Source/imbi/main/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-assistant"`, `"/Volumes/Source/imbi/main/imbi-automations"`, `"/Volumes/Source/imbi/main/imbi-mcp"`, `"/Volumes/Source/imbi/main/imbi-ui"`
- **`data`**: `object`
  - **`agentId`**: `string`
  - **`message`**: `object`
    - **`message`**: `object`
      - **`content`**: `array`
        Array element:
          - **`caller`**?: `object`
          - **`content`**?: `array | string` — free-form text
          - **`id`**?: `string` — free-form text
          - **`input`**?: `object`
          - **`is_error`**?: `boolean`
          - **`name`**?: `string` — values: `"Bash"`, `"CronCreate"`, `"CronDelete"`, `"CronList"`, `"Edit"`, `"Glob"`, `"Grep"`, `"Read"`, `"Skill"`, `"TaskCreate"`, `"TaskUpdate"`, `"ToolSearch"`, `"WebFetch"`, `"WebSearch"`, `"Write"`, `"mcp__plugin_zoekt-mcp_zoekt-mcp__get_file"`, `"mcp__plugin_zoekt-mcp_zoekt-mcp__search"`, `"mcp__plugin_zoekt-mcp_zoekt-mcp_zoekt-mcp__search"`
          - **`text`**?: `string` — free-form text
          - **`tool_use_id`**?: `string` — free-form text
          - **`type`**: `string` — values: `"text"`, `"tool_result"`, `"tool_use"`
      - **`context_management`**?: `null | object`
        - **`applied_edits`**: `array`
          Array of: ``
      - **`id`**?: `string` — free-form text
      - **`model`**?: `string` — values: `"claude-haiku-4-5-20251001"`, `"claude-opus-4-6"`, `"claude-sonnet-4-5-20250929"`, `"claude-sonnet-4-6"`
      - **`role`**: `string` — values: `"assistant"`, `"user"`
      - **`stop_reason`**?: `null | string` — values: `"tool_use"`
      - **`stop_sequence`**?: `null`
      - **`type`**?: `string` — values: `"message"`
      - **`usage`**?: `object`
        - **`cache_creation`**: `object`
          - **`ephemeral_1h_input_tokens`**: `integer`
          - **`ephemeral_5m_input_tokens`**: `integer`
        - **`cache_creation_input_tokens`**: `integer`
        - **`cache_read_input_tokens`**: `integer`
        - **`inference_geo`**: `string` — values: `"not_available"`
        - **`input_tokens`**: `integer`
        - **`output_tokens`**: `integer`
        - **`service_tier`**: `string` — values: `"standard"`
    - **`requestId`**?: `string` — free-form text
    - **`timestamp`**: `string`
    - **`toolUseResult`**?: `string` — free-form text
    - **`type`**: `string` — values: `"assistant"`, `"user"`
    - **`uuid`**: `string` — free-form text
  - **`normalizedMessages`**?: `array`
    Array of: ``
  - **`prompt`**: `string` — free-form text
  - **`resume`**?: `string` — values: `"a417790e9016af838"`, `"a88555aa7598c6cc9"`, `"aa051e841cbbca2a9"`, `"aaaef87250851795c"`, `"ac83b5b3a8d8370f0"`
  - **`type`**: `string` — values: `"agent_progress"`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string`
- **`toolUseID`**: `string`
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.44"`, `"2.1.45"`, `"2.1.47"`, `"2.1.49"`, `"2.1.50"`, `"2.1.53"`, `"2.1.56"`, `"2.1.58"`, `"2.1.59"`, `"2.1.62"`, `"2.1.63"`, `"2.1.68"`, `"2.1.70"`, `"2.1.71"`, `"2.1.72"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "data": {
    "agentId": "ab80229",
    "message": {
      "message": {
        "content": [
          "[...]"
        ],
        "role": "user"
      },
      "timestamp": "2026-02-12T21:41:37.991Z",
      "type": "user",
      "uuid": "3e0363ab-a022-4907-982b-fb9d7c4ea629"
    },
    "normalizedMessages": [],
    "prompt": "Explore the Jira-related test data and CLI commands in the awatlassian project a... [680b total]",
    "type": "agent_progress"
  },
  "gitBranch": "feature/add-write-capabilities",
  "isSidechain": false,
  "parentToolUseID": "toolu_017ojJwWFuekBTyhXbwkjUMi",
  "parentUuid": "790c2aaa-2fef-4e4c-89d0-6f9be831ef6b",
  "sessionId": "0f5e5c23-3911-4875-a57a-06657a2058e9",
  "slug": "sequential-napping-duckling",
  "timestamp": "2026-02-12T21:41:37.992Z",
  "toolUseID": "agent_msg_01TJJQhFXLyfNFRN8zBwa37V",
  "type": "progress",
  "userType": "external",
  "uuid": "31f26d82-3ad1-4a11-bb04-ac8fe099c0e4",
  "version": "2.1.39"
}
```

---

## `progress:bash_progress`

**Count:** 81339 instances

Streaming output from a running Bash command

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`data`**: `object`
  - **`elapsedTimeSeconds`**: `integer`
  - **`fullOutput`**: `string` — free-form text
  - **`output`**: `string` — free-form text
  - **`taskId`**?: `string`
  - **`timeoutMs`**?: `integer`
  - **`totalBytes`**?: `integer`
  - **`totalLines`**: `integer`
  - **`type`**: `string` — values: `"bash_progress"`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string`
- **`toolUseID`**: `string`
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.12"`, `"2.1.14"`, `"2.1.17"`, `"2.1.32"`, `"2.1.34"`, `"2.1.37"`, `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.44"`, `"2.1.45"`, `"2.1.47"`, `"2.1.49"`, `"2.1.50"`, `"2.1.53"`, `"2.1.56"`, `"2.1.58"`, `"2.1.59"`, `"2.1.62"`, `"2.1.63"`, `"2.1.66"`, `"2.1.68"`, `"2.1.69"`, `"2.1.70"`, `"2.1.71"`

### Sample

```json
{
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "data": {
    "elapsedTimeSeconds": 2,
    "fullOutput": "Installing package...\nLooking in indexes: https://pypi.aweber.io/repository/pypi... [667b total]",
    "output": "  Attempting uninstall: pip\n    Found existing installation: pip 25.0.1\n    Unin... [176b total]",
    "timeoutMs": 120000,
    "totalLines": 12,
    "type": "bash_progress"
  },
  "gitBranch": "feature/add-write-capabilities",
  "isSidechain": false,
  "parentToolUseID": "toolu_0176dpYB6GjPkgsMYkVYWgfc",
  "parentUuid": "a19a8ca0-d27b-48c6-bbde-9155b8601056",
  "sessionId": "0f5e5c23-3911-4875-a57a-06657a2058e9",
  "slug": "sequential-napping-duckling",
  "timestamp": "2026-02-12T21:31:27.071Z",
  "toolUseID": "bash-progress-0",
  "type": "progress",
  "userType": "external",
  "uuid": "51dd0fea-156f-4397-8da7-4ad2c88ef61f",
  "version": "2.1.39"
}
```

---

## `progress:hook_progress`

**Count:** 35296 instances

Progress update from a running hook script

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`data`**: `object`
  - **`command`**: `string` — values: `"${CLAUDE_PLUGIN_ROOT}/hooks/validate-imports.py"`, `"${CLAUDE_PLUGIN_ROOT}/scripts/session-status.sh"`, `"bash ${CLAUDE_PLUGIN_ROOT}/scripts/setup-dependencies.sh"`, `"callback"`, `"python3 ${CLAUDE_PLUGIN_ROOT}/hooks/security_reminder_hook.py"`, `"~/.claude/hooks/session-status.sh"`
  - **`hookEvent`**: `string` — values: `"PostToolUse"`, `"PostToolUseFailure"`, `"PreToolUse"`, `"SessionStart"`, `"Stop"`, `"SubagentStop"`
  - **`hookName`**: `string`
  - **`type`**: `string` — values: `"hook_progress"`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `null | string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`timestamp`**: `string`
- **`toolUseID`**: `string` — free-form text
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "data": {
    "command": "bash ${CLAUDE_PLUGIN_ROOT}/scripts/setup-dependencies.sh",
    "hookEvent": "SessionStart",
    "hookName": "SessionStart:startup",
    "type": "hook_progress"
  },
  "gitBranch": "HEAD",
  "isSidechain": false,
  "parentToolUseID": "ff4690d9-1358-450f-aa98-aed6a1a0c82e",
  "parentUuid": null,
  "sessionId": "0be288e3-4239-4ef5-9656-92608edb7d49",
  "timestamp": "2026-03-13T16:36:25.694Z",
  "toolUseID": "ff4690d9-1358-450f-aa98-aed6a1a0c82e",
  "type": "progress",
  "userType": "external",
  "uuid": "1d27826f-6b73-4091-b2c1-f6fd53b0fdc6",
  "version": "2.1.74"
}
```

---

## `progress:mcp_progress`

**Count:** 820 instances

Progress update from an MCP tool call

### Fields

- **`agentId`**?: `string` — values: `"a09e4be"`, `"a22e2b43fc3689f49"`, `"a265a10"`, `"a2f58cf"`, `"a4040a2bc6b07ee74"`, `"a450a2d"`, `"a6fc123848270f9b5"`, `"a70b605"`, `"a72691f161a0e5b53"`, `"a73f8b8"`, `"aa87e7a"`, `"ac0da4a"`, `"acompact-7f5cf2a7f3ba6ad6"`, `"ada7b141095a4766d"`, `"afa2a39"`, `"afc5e27"`
- **`cwd`**: `string` — values: `"/Volumes/Source/backend-libraries/awatlassian"`, `"/Volumes/Source/backend-libraries/awpromptlayer"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/claude/gavinr-terminal-notifier"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/consumers/clickhouse-materialization"`, `"/Volumes/Source/data-processing/bi-pipeline"`, `"/Volumes/Source/imbi/main/imbi-ui"`, `"/Volumes/Source/schemata/clickhouse"`
- **`data`**: `object`
  - **`elapsedTimeMs`**?: `integer`
  - **`serverName`**: `string` — values: `"aj-neo4j"`, `"claude.ai Figma"`, `"claude.ai Slack"`, `"clickhouse-production"`, `"plugin:zoekt-mcp:zoekt-mcp"`, `"sentry"`
  - **`status`**: `string` — values: `"completed"`, `"failed"`, `"started"`
  - **`toolName`**: `string` — values: `"find_organizations"`, `"get_file"`, `"get_issue_details"`, `"get_metadata"`, `"get_screenshot"`, `"read_neo4j_cypher"`, `"run_select_query"`, `"search"`, `"search_issue_events"`, `"search_issues"`, `"slack_read_canvas"`, `"slack_read_channel"`, `"slack_read_thread"`, `"update_issue"`
  - **`type`**: `string` — values: `"mcp_progress"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"feature/add-write-capabilities"`, `"feature/enrich-delivery-geoip"`, `"feature/migrate-to-awpromptlayer"`, `"feature/research-subagent"`, `"feature/scheduled-tasks-m2-m3"`, `"feature/slack-file-tools-and-jira-attachments"`, `"feature/unsubscribe-enrichment-join-query"`, `"feature/update-api-documentation"`, `"fix/DBA-1462"`, `"fix/huddle-notes-message-changed"`, `"fix/ignore-attachments"`, `"fix/sentry-issues"`, `"main"`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string` — values: `"cuddly-jingling-pebble"`, `"enumerated-kindling-wilkes"`, `"giggly-imagining-lampson"`, `"gleaming-strolling-wren"`, `"glimmering-baking-hedgehog"`, `"glimmering-nibbling-lamport"`, `"glittery-dancing-cherny"`, `"hashed-dancing-token"`, `"hazy-mixing-panda"`, `"inherited-kindling-kettle"`, `"iterative-painting-whistle"`, `"moonlit-discovering-globe"`, `"optimized-wiggling-bird"`, `"reactive-foraging-spindle"`, `"robust-conjuring-robin"`, `"sequential-napping-duckling"`, `"shiny-munching-starlight"`, `"snazzy-purring-pelican"`, `"steady-riding-thacker"`, `"validated-wobbling-hellman"`, `"vivid-discovering-hammock"`, `"zany-juggling-stroustrup"`
- **`teamName`**?: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string`
- **`toolUseID`**: `string` — free-form text
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.34"`, `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.47"`, `"2.1.56"`, `"2.1.59"`, `"2.1.62"`, `"2.1.63"`, `"2.1.66"`, `"2.1.68"`, `"2.1.69"`, `"2.1.70"`, `"2.1.71"`

### Sample

```json
{
  "agentId": "aa87e7a",
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "data": {
    "serverName": "plugin:zoekt-mcp:zoekt-mcp",
    "status": "started",
    "toolName": "search",
    "type": "mcp_progress"
  },
  "gitBranch": "feature/add-write-capabilities",
  "isSidechain": true,
  "parentToolUseID": "toolu_01R8aipdXhGgMeqPnfGDtQms",
  "parentUuid": "59a1edaa-3e64-46fb-946f-56bed4ce04a4",
  "sessionId": "26963cd1-4e68-4327-80db-9369019d23a9",
  "slug": "sequential-napping-duckling",
  "timestamp": "2026-02-13T14:26:47.682Z",
  "toolUseID": "toolu_01R8aipdXhGgMeqPnfGDtQms",
  "type": "progress",
  "userType": "external",
  "uuid": "a8a57da7-a14f-4639-9e06-3820cc38cac2",
  "version": "2.1.39"
}
```

---

## `progress:query_update`

**Count:** 12 instances

Server-side web search query update during tool use

### Fields

- **`agentId`**?: `string` — values: `"a0f19937159495ca1"`, `"a440f6ff4b9b70f64"`, `"af0145420cd28b97c"`, `"afb4619"`
- **`cwd`**: `string` — values: `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/imbi/imbi-api"`, `"/Volumes/Source/imbi/main"`
- **`data`**: `object`
  - **`query`**: `string` — values: `"Claude Code hooks Notification notification_type values JSON schema"`, `"Claude Code hooks documentation JSON input format PreToolUse PostToolUse"`, `"Claude Code hooks events SessionStart UserPromptSubmit PreToolUse PostToolUse Stop"`, `"Claude Code hooks hook_event_name SubagentStart Notification"`, `"FastAPI custom OpenAPI schema dynamic models pydantic 2026"`, `"FastMCP GitHub repository documentation"`, `"FastMCP OpenAPI auto-generate tools"`, `"MCP OpenAPI auto-generation discovery"`, `"MCP server OpenAPI specification bridging"`, `"rollbar dSYM upload API curl post_server_item macOS"`, `"rollbar dsym upload API macOS CI github actions 2025"`, `"site:code.claude.com hooks reference PreToolUse PostToolUse JSON stdin input fields"`
  - **`type`**: `string` — values: `"query_update"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"main"`, `"rollbar-instrumentation"`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"frolicking-zooming-meerkat"`, `"groovy-tickling-snowflake"`, `"hazy-bubbling-galaxy"`, `"valiant-wibbling-pebble"`, `"witty-sleeping-pearl"`
- **`timestamp`**: `string` — values: `"2026-01-23T21:12:41.668Z"`, `"2026-03-10T14:32:10.456Z"`, `"2026-03-10T14:32:10.471Z"`, `"2026-03-10T14:32:10.739Z"`, `"2026-03-10T14:32:10.981Z"`, `"2026-03-11T00:02:37.653Z"`, `"2026-03-11T02:03:16.150Z"`, `"2026-03-11T02:03:19.238Z"`, `"2026-03-11T02:03:34.203Z"`, `"2026-03-11T02:03:35.617Z"`, `"2026-03-12T20:56:31.902Z"`, `"2026-03-12T20:56:42.679Z"`
- **`toolUseID`**: `string` — values: `"search-progress-1"`
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.17"`, `"2.1.71"`, `"2.1.72"`, `"2.1.74"`

### Sample

```json
{
  "agentId": "af0145420cd28b97c",
  "cwd": "/Volumes/Source/gmr/claude-status",
  "data": {
    "query": "Claude Code hooks documentation JSON input format PreToolUse PostToolUse",
    "type": "query_update"
  },
  "gitBranch": "main",
  "isSidechain": true,
  "parentToolUseID": "toolu_01SfwMQZAUdVM4pYg2eoXtvL",
  "parentUuid": "40098a45-d68b-40ea-a605-5d6981614606",
  "sessionId": "0797bb3d-c1ed-4b0a-bd89-26a356c57a70",
  "slug": "groovy-tickling-snowflake",
  "timestamp": "2026-03-11T02:03:16.150Z",
  "toolUseID": "search-progress-1",
  "type": "progress",
  "userType": "external",
  "uuid": "dba93055-2def-4cdc-bd6c-7d656d02a3d6",
  "version": "2.1.72"
}
```

---

## `progress:search_results_received`

**Count:** 12 instances

Server-side web search results received notification

### Fields

- **`agentId`**?: `string` — values: `"a0f19937159495ca1"`, `"a440f6ff4b9b70f64"`, `"af0145420cd28b97c"`, `"afb4619"`
- **`cwd`**: `string` — values: `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/imbi/imbi-api"`, `"/Volumes/Source/imbi/main"`
- **`data`**: `object`
  - **`query`**: `string` — values: `"Claude Code hooks Notification notification_type values JSON schema"`, `"Claude Code hooks documentation JSON input format PreToolUse PostToolUse"`, `"Claude Code hooks events SessionStart UserPromptSubmit PreToolUse PostToolUse Stop"`, `"Claude Code hooks hook_event_name SubagentStart Notification"`, `"FastAPI custom OpenAPI schema dynamic models pydantic 2026"`, `"FastMCP GitHub repository documentation"`, `"FastMCP OpenAPI auto-generate tools"`, `"MCP OpenAPI auto-generation discovery"`, `"MCP server OpenAPI specification bridging"`, `"rollbar dSYM upload API curl post_server_item macOS"`, `"rollbar dsym upload API macOS CI github actions 2025"`, `"site:code.claude.com hooks reference PreToolUse PostToolUse JSON stdin input fields"`
  - **`resultCount`**: `integer`
  - **`type`**: `string` — values: `"search_results_received"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"main"`, `"rollbar-instrumentation"`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"frolicking-zooming-meerkat"`, `"groovy-tickling-snowflake"`, `"hazy-bubbling-galaxy"`, `"valiant-wibbling-pebble"`, `"witty-sleeping-pearl"`
- **`timestamp`**: `string` — values: `"2026-01-23T21:12:43.909Z"`, `"2026-03-10T14:32:12.032Z"`, `"2026-03-10T14:32:12.271Z"`, `"2026-03-10T14:32:12.800Z"`, `"2026-03-10T14:32:13.403Z"`, `"2026-03-11T00:02:38.885Z"`, `"2026-03-11T02:03:17.523Z"`, `"2026-03-11T02:03:21.766Z"`, `"2026-03-11T02:03:35.278Z"`, `"2026-03-11T02:03:36.964Z"`, `"2026-03-12T20:56:33.576Z"`, `"2026-03-12T20:56:43.732Z"`
- **`toolUseID`**: `string` — free-form text
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.17"`, `"2.1.71"`, `"2.1.72"`, `"2.1.74"`

### Sample

```json
{
  "agentId": "af0145420cd28b97c",
  "cwd": "/Volumes/Source/gmr/claude-status",
  "data": {
    "query": "Claude Code hooks documentation JSON input format PreToolUse PostToolUse",
    "resultCount": 10,
    "type": "search_results_received"
  },
  "gitBranch": "main",
  "isSidechain": true,
  "parentToolUseID": "toolu_01SfwMQZAUdVM4pYg2eoXtvL",
  "parentUuid": "63639088-f3b3-4132-9540-60b3d013770d",
  "sessionId": "0797bb3d-c1ed-4b0a-bd89-26a356c57a70",
  "slug": "groovy-tickling-snowflake",
  "timestamp": "2026-03-11T02:03:17.523Z",
  "toolUseID": "srvtoolu_011DRT1E1wCbxMoQq2a8qnDN",
  "type": "progress",
  "userType": "external",
  "uuid": "3dc8e8c1-f91f-4b3b-a2f4-81ca10e780e1",
  "version": "2.1.72"
}
```

---

## `progress:waiting_for_task`

**Count:** 18 instances

Indicator that the system is waiting for a background task

### Fields

- **`cwd`**: `string` — values: `"/Volumes/Source/backend-libraries/awpromptlayer"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/imbi/features/admin-env-pt/imbi-ui"`, `"/Volumes/Source/imbi/main/imbi-api"`
- **`data`**: `object`
  - **`taskDescription`**: `string` — values: `"Check imbi-api PR #175 status"`, `"Code reuse review"`, `"Phase 7-8: Frontend changes"`, `"Re-run CodeRabbit review capturing output"`, `"Run CodeRabbit review on all changes"`, `"Run CodeRabbit review on changes"`, `"Run full test suite without -f flag"`, `"Run tests"`, `"Wait 2 more minutes"`, `"Wait 3 minutes for CI to complete"`, `"Wait 3.5min then check PR #22 status"`, `"Wait 3min then check PR #20 status"`, `"Wait for PR CI run to complete"`, `"Watch latest CI run"`, `"Write auth and permissions tests"`
  - **`taskType`**: `string` — values: `"local_agent"`, `"local_bash"`
  - **`type`**: `string` — values: `"waiting_for_task"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"feature/mcp-client-integration"`, `"feature/mcp-verify-ssl-option"`, `"feature/migrate-to-awpromptlayer"`, `"feature/scheduled-tasks-m2-m3"`, `"feature/sparkle-auto-updates"`, `"fix/waiting-state-priority"`, `"main"`
- **`isSidechain`**: `boolean`
- **`parentToolUseID`**: `string` — free-form text
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"clever-munching-minsky"`, `"gleaming-strolling-wren"`, `"glimmering-baking-hedgehog"`, `"greedy-twirling-cupcake"`, `"groovy-tickling-snowflake"`, `"hazy-mixing-panda"`, `"iterative-painting-whistle"`, `"nested-roaming-dawn"`, `"replicated-purring-pinwheel"`, `"robust-conjuring-robin"`, `"rustling-brewing-meerkat"`, `"structured-inventing-garden"`
- **`teamName`**: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string` — values: `"2026-02-12T22:11:40.445Z"`, `"2026-02-13T22:46:06.452Z"`, `"2026-02-16T00:53:34.990Z"`, `"2026-02-16T00:55:37.924Z"`, `"2026-02-16T01:00:47.398Z"`, `"2026-02-23T17:15:49.942Z"`, `"2026-02-23T17:19:08.249Z"`, `"2026-02-23T17:55:36.224Z"`, `"2026-02-23T18:39:26.122Z"`, `"2026-03-02T18:57:02.648Z"`, `"2026-03-02T22:45:00.394Z"`, `"2026-03-03T01:35:01.558Z"`, `"2026-03-03T01:39:55.627Z"`, `"2026-03-11T02:19:23.320Z"`, `"2026-03-12T16:17:25.469Z"`, `"2026-03-12T16:21:50.385Z"`, `"2026-03-12T16:48:43.099Z"`, `"2026-03-13T15:17:35.755Z"`
- **`toolUseID`**: `string` — values: `"task-output-waiting-1770934300445"`, `"task-output-waiting-1771022766452"`, `"task-output-waiting-1771203214990"`, `"task-output-waiting-1771203337924"`, `"task-output-waiting-1771203647398"`, `"task-output-waiting-1771866949942"`, `"task-output-waiting-1771867148248"`, `"task-output-waiting-1771869336224"`, `"task-output-waiting-1771871966122"`, `"task-output-waiting-1772477822648"`, `"task-output-waiting-1772491500394"`, `"task-output-waiting-1772501701558"`, `"task-output-waiting-1772501995627"`, `"task-output-waiting-1773195563320"`, `"task-output-waiting-1773332245468"`, `"task-output-waiting-1773332510385"`, `"task-output-waiting-1773334123099"`, `"task-output-waiting-1773415055755"`
- **`type`**: `string` — values: `"progress"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.47"`, `"2.1.62"`, `"2.1.63"`, `"2.1.71"`, `"2.1.72"`, `"2.1.74"`

### Sample

```json
{
  "cwd": "/Volumes/Source/backend-libraries/awpromptlayer",
  "data": {
    "taskDescription": "Run CodeRabbit review on all changes",
    "taskType": "local_bash",
    "type": "waiting_for_task"
  },
  "gitBranch": "main",
  "isSidechain": false,
  "parentToolUseID": "toolu_014jFDcQ9qqqJGsJ6rkojBH1",
  "parentUuid": "34a38a6f-b322-48a0-b95d-612010ec44f5",
  "sessionId": "42f571c5-08b6-465a-8128-aa39d03d92bd",
  "slug": "robust-conjuring-robin",
  "timestamp": "2026-02-13T22:46:06.452Z",
  "toolUseID": "task-output-waiting-1771022766452",
  "type": "progress",
  "userType": "external",
  "uuid": "9ff1e5a3-44ef-4abb-a1a6-e937f5c1f79b",
  "version": "2.1.41"
}
```

---

## `queue-operation`

**Count:** 1779 instances

Background task queue operation (enqueue, dequeue, remove)

### Fields

- **`content`**?: `string` — free-form text
- **`operation`**: `string` — values: `"dequeue"`, `"enqueue"`, `"popAll"`, `"remove"`
- **`sessionId`**: `string` — free-form text
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"queue-operation"`

### Sample

```json
{
  "operation": "enqueue",
  "sessionId": "2b9b74ae-8328-4e44-b5d8-9e4695afcd5d",
  "timestamp": "2026-03-12T21:25:51.396Z",
  "type": "queue-operation"
}
```

---

## `system:api_error`

**Count:** 23 instances

API-level error from the Claude service

### Fields

- **`agentId`**?: `string` — values: `"aside_question-ad48711b92cb30ef"`
- **`cause`**: `object`
  - **`code`**: `string` — values: `"ConnectionRefused"`, `"FailedToOpenSocket"`
  - **`errno`**: `integer`
  - **`path`**: `string` — values: `"https://api.anthropic.com/v1/messages?beta=true"`
- **`cwd`**: `string` — values: `"/Volumes/Source/gmr/claude-status"`
- **`error`**: `object`
  - **`cause`**: `object`
    - **`code`**: `string` — values: `"ConnectionRefused"`, `"FailedToOpenSocket"`
    - **`errno`**: `integer`
    - **`path`**: `string` — values: `"https://api.anthropic.com/v1/messages?beta=true"`
  - **`error`**?: `object`
    - **`error`**: `object`
      - **`message`**: `string` — values: `"Overloaded"`
      - **`type`**: `string` — values: `"overloaded_error"`
    - **`request_id`**: `string` — free-form text
    - **`type`**: `string` — values: `"error"`
  - **`headers`**?: `object`
    - **`anthropic-organization-id`**: `string` — free-form text
    - **`cf-cache-status`**: `string` — values: `"DYNAMIC"`
    - **`cf-ray`**: `string` — values: `"9dbcd2cf3a6a0575-IAD"`, `"9dbcd2e2da270575-IAD"`, `"9dbcd2fd0bb70575-IAD"`
    - **`connection`**: `string` — values: `"keep-alive"`
    - **`content-encoding`**: `string` — values: `"gzip"`
    - **`content-security-policy`**: `string` — values: `"default-src 'none'; frame-ancestors 'none'"`
    - **`content-type`**: `string` — values: `"application/json"`
    - **`date`**: `string` — values: `"Fri, 13 Mar 2026 17:40:26 GMT"`, `"Fri, 13 Mar 2026 17:40:29 GMT"`, `"Fri, 13 Mar 2026 17:40:32 GMT"`
    - **`request-id`**: `string` — free-form text
    - **`server`**: `string` — values: `"cloudflare"`
    - **`server-timing`**: `string` — values: `"proxy;dur=2001"`, `"proxy;dur=2564"`, `"proxy;dur=2914"`
    - **`strict-transport-security`**: `string` — values: `"max-age=31536000; includeSubDomains; preload"`
    - **`transfer-encoding`**: `string` — values: `"chunked"`
    - **`vary`**: `string` — values: `"Accept-Encoding"`
    - **`x-envoy-upstream-service-time`**: `string` — values: `"2000"`, `"2564"`, `"2913"`
    - **`x-robots-tag`**: `string` — values: `"none"`
    - **`x-should-retry`**: `string` — values: `"true"`
  - **`requestID`**?: `string` — free-form text
  - **`status`**?: `integer`
- **`gitBranch`**: `string` — values: `"feature/question-detection-and-version-fixes"`, `"main"`
- **`isSidechain`**: `boolean`
- **`level`**: `string` — values: `"error"`
- **`maxRetries`**: `integer`
- **`parentUuid`**: `string` — free-form text
- **`retryAttempt`**: `integer`
- **`retryInMs`**: `float`
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string` — values: `"drifting-gliding-wilkinson"`, `"serialized-tumbling-stroustrup"`
- **`subtype`**: `string` — values: `"api_error"`
- **`timestamp`**: `string` — values: `"2026-03-11T04:14:07.707Z"`, `"2026-03-11T04:14:08.216Z"`, `"2026-03-11T04:14:39.313Z"`, `"2026-03-11T04:15:11.462Z"`, `"2026-03-11T04:15:42.752Z"`, `"2026-03-11T04:16:07.865Z"`, `"2026-03-11T04:16:43.424Z"`, `"2026-03-11T04:17:23.445Z"`, `"2026-03-11T04:18:26.629Z"`, `"2026-03-11T04:19:34.195Z"`, `"2026-03-13T17:40:26.164Z"`, `"2026-03-13T17:40:29.649Z"`, `"2026-03-13T17:40:32.934Z"`
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.72"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Volumes/Source/gmr/claude-status",
  "error": {
    "error": {
      "error": {
        "message": "Overloaded",
        "type": "overloaded_error"
      },
      "request_id": "req_011CZ1VM8NsqK5TdECK9pAWx",
      "type": "error"
    },
    "headers": {
      "anthropic-organization-id": "893fd049-b3c9-4e65-ad0d-be0951a474e7",
      "cf-cache-status": "DYNAMIC",
      "cf-ray": "9dbcd2cf3a6a0575-IAD",
      "connection": "keep-alive",
      "content-encoding": "gzip",
      "content-security-policy": "default-src 'none'; frame-ancestors 'none'",
      "content-type": "application/json",
      "date": "Fri, 13 Mar 2026 17:40:26 GMT",
      "request-id": "req_011CZ1VM8NsqK5TdECK9pAWx",
      "server": "cloudflare",
      "server-timing": "proxy;dur=2564",
      "strict-transport-security": "max-age=31536000; includeSubDomains; preload",
      "transfer-encoding": "chunked",
      "vary": "Accept-Encoding",
      "x-envoy-upstream-service-time": "2564",
      "x-robots-tag": "none",
      "x-should-retry": "true"
    },
    "requestID": "req_011CZ1VM8NsqK5TdECK9pAWx",
    "status": 529
  },
  "gitBranch": "feature/question-detection-and-version-fixes",
  "isSidechain": false,
  "level": "error",
  "maxRetries": 10,
  "parentUuid": "d42acdbc-4ae1-4d45-8bcb-e58b60df8a0d",
  "retryAttempt": 1,
  "retryInMs": 504.8987858332398,
  "sessionId": "233c7845-c0b4-428a-b02a-3ba363192f34",
  "slug": "serialized-tumbling-stroustrup",
  "subtype": "api_error",
  "timestamp": "2026-03-13T17:40:26.164Z",
  "type": "system",
  "userType": "external",
  "uuid": "d9d90cbb-495c-4a7d-8391-56629a7992c7",
  "version": "2.1.75"
}
```

---

## `system:compact_boundary`

**Count:** 58 instances

Marks where conversation context was compacted

### Fields

- **`agentId`**?: `string` — values: `"a8527ee"`, `"acompact-21da4b8d4ddbfcec"`, `"acompact-2946bd3e32d321d8"`, `"acompact-d9621f0a65e49e00"`, `"acompact-f538e286804c8ad1"`, `"af8910f7e1f97b291"`
- **`compactMetadata`**: `object`
  - **`preTokens`**: `integer`
  - **`trigger`**: `string` — values: `"auto"`, `"manual"`
- **`content`**: `string` — values: `"Conversation compacted"`
- **`cwd`**: `string` — values: `"/Volumes/Source/backend-libraries/awatlassian"`, `"/Volumes/Source/backend-libraries/awpromptlayer"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/bots/aj-workflows"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/gmr/aiorabbit"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/imbi/features/admin-env-pt/imbi-ui"`, `"/Volumes/Source/imbi/features/ai-assistant/imbi-ui"`, `"/Volumes/Source/imbi/features/slim-shared-models/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-assistant"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"feature/abstract-provider-calls"`, `"feature/confluence-reindex-tool"`, `"feature/file-upload-system"`, `"feature/mcp-client-integration"`, `"feature/migrate-to-awpromptlayer"`, `"feature/productivity-tracking"`, `"feature/question-detection-and-version-fixes"`, `"feature/research-subagent"`, `"feature/run-callbacks-caching-betas-validators"`, `"feature/scheduled-tasks-m1"`, `"feature/scheduled-tasks-m2-m3"`, `"feature/simplify-awpromptlayer-integration"`, `"feature/slack-huddle-notes"`, `"feature/structured-output-boolean-classification"`, `"feature/update-api-documentation"`, `"feature/workflows"`, `"fix/code-review-bugs-and-optimizations"`, `"fix/indexer-processor-resilience"`, `"main"`, `"tmux-support"`
- **`isMeta`**: `boolean`
- **`isSidechain`**: `boolean`
- **`level`**: `string` — values: `"info"`
- **`logicalParentUuid`**: `string` — free-form text
- **`parentUuid`**: `null`
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"delegated-hopping-elephant"`, `"dynamic-seeking-stardust"`, `"eager-riding-hellman"`, `"elegant-splashing-church"`, `"glimmering-baking-hedgehog"`, `"glimmering-noodling-turtle"`, `"glimmering-spinning-pixel"`, `"golden-discovering-pearl"`, `"groovy-baking-badger"`, `"groovy-tickling-snowflake"`, `"hashed-dancing-token"`, `"hazy-mixing-panda"`, `"inherited-kindling-kettle"`, `"iterative-painting-whistle"`, `"melodic-pondering-rainbow"`, `"nested-roaming-dawn"`, `"reactive-foraging-spindle"`, `"reflective-giggling-conway"`, `"replicated-purring-pinwheel"`, `"resilient-baking-lemon"`, `"robust-conjuring-robin"`, `"sequential-napping-duckling"`, `"serene-spinning-fiddle"`, `"shiny-munching-starlight"`, `"snazzy-sleeping-hopper"`, `"structured-inventing-garden"`, `"swirling-orbiting-yao"`, `"validated-wobbling-hellman"`, `"wondrous-discovering-matsumoto"`
- **`subtype`**: `string` — values: `"compact_boundary"`
- **`teamName`**?: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.34"`, `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.45"`, `"2.1.47"`, `"2.1.56"`, `"2.1.58"`, `"2.1.59"`, `"2.1.63"`, `"2.1.68"`, `"2.1.71"`, `"2.1.72"`, `"2.1.75"`

### Sample

```json
{
  "compactMetadata": {
    "preTokens": 169337,
    "trigger": "auto"
  },
  "content": "Conversation compacted",
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "gitBranch": "feature/update-api-documentation",
  "isMeta": false,
  "isSidechain": false,
  "level": "info",
  "logicalParentUuid": "3775babd-531d-4400-a007-02c1b5b50575",
  "parentUuid": null,
  "sessionId": "26963cd1-4e68-4327-80db-9369019d23a9",
  "slug": "sequential-napping-duckling",
  "subtype": "compact_boundary",
  "timestamp": "2026-02-13T18:13:30.062Z",
  "type": "system",
  "userType": "external",
  "uuid": "a1832ed3-9c9a-4d6b-916f-b11620e26425",
  "version": "2.1.39"
}
```

---

## `system:informational`

**Count:** 2 instances

Informational system message (e.g. session metadata)

### Fields

- **`content`**: `string` — values: `"Args from unknown skill: claude-source-session-names"`
- **`cwd`**: `string` — values: `"/Volumes/Source/gmr/claude-status"`
- **`gitBranch`**: `string` — values: `"feature/session-naming"`
- **`isMeta`**: `boolean`
- **`isSidechain`**: `boolean`
- **`level`**: `string` — values: `"warning"`
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"stateless-wishing-moonbeam"`
- **`subtype`**: `string` — values: `"informational"`
- **`timestamp`**: `string` — values: `"2026-03-12T19:38:27.711Z"`, `"2026-03-12T19:38:43.762Z"`
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.74"`

### Sample

```json
{
  "content": "Args from unknown skill: claude-source-session-names",
  "cwd": "/Volumes/Source/gmr/claude-status",
  "gitBranch": "feature/session-naming",
  "isMeta": false,
  "isSidechain": false,
  "level": "warning",
  "parentUuid": "dec20951-3f62-4d31-a758-ccd1a5136dd1",
  "sessionId": "ac1cfa83-b9fa-4a6b-8430-deeadf36a831",
  "slug": "stateless-wishing-moonbeam",
  "subtype": "informational",
  "timestamp": "2026-03-12T19:38:27.711Z",
  "type": "system",
  "userType": "external",
  "uuid": "22b2570b-e94c-4524-942c-840319d6a541",
  "version": "2.1.74"
}
```

---

## `system:local_command`

**Count:** 46 instances

A local slash command invoked by the user

### Fields

- **`agentId`**?: `string` — values: `"aside_question-ad48711b92cb30ef"`
- **`content`**: `string` — free-form text
- **`cwd`**: `string` — values: `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/imbi/main"`, `"/Volumes/Source/imbi/main/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-automations"`, `"/Volumes/Source/imbi/main/imbi-mcp"`, `"/Volumes/Source/imbi/main/imbi-ui"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"feature/openapi-auto-discovery"`, `"feature/session-naming"`, `"feature/sparkle-auto-updates"`, `"fix/deploy-kustomize"`, `"fix/jira-escaped-newlines"`, `"main"`
- **`isMeta`**: `boolean`
- **`isSidechain`**: `boolean`
- **`level`**: `string` — values: `"info"`
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string` — values: `"drifting-gliding-wilkinson"`, `"enchanted-nibbling-wave"`, `"idempotent-bouncing-karp"`, `"inherited-kindling-kettle"`, `"radiant-exploring-rivest"`, `"reactive-foraging-spindle"`, `"stateless-wishing-moonbeam"`, `"structured-inventing-garden"`, `"validated-wobbling-hellman"`, `"zazzy-munching-ladybug"`
- **`subtype`**: `string` — values: `"local_command"`
- **`timestamp`**: `string` — values: `"2026-02-20T03:13:06.149Z"`, `"2026-02-20T22:47:30.840Z"`, `"2026-02-23T20:35:01.538Z"`, `"2026-02-24T20:20:47.438Z"`, `"2026-02-27T22:19:39.874Z"`, `"2026-03-02T21:03:14.991Z"`, `"2026-03-04T19:04:45.001Z"`, `"2026-03-07T03:24:48.493Z"`, `"2026-03-09T17:56:12.801Z"`, `"2026-03-09T19:51:31.108Z"`, `"2026-03-09T21:20:27.090Z"`, `"2026-03-09T21:31:59.951Z"`, `"2026-03-09T21:34:19.333Z"`, `"2026-03-09T22:37:13.083Z"`, `"2026-03-10T14:31:50.827Z"`, `"2026-03-10T15:18:51.148Z"`, `"2026-03-10T22:22:30.220Z"`, `"2026-03-11T02:45:09.591Z"`, `"2026-03-11T02:45:33.846Z"`, `"2026-03-11T13:39:24.813Z"`, `"2026-03-12T17:25:46.305Z"`, `"2026-03-12T17:27:15.060Z"`, `"2026-03-12T19:11:45.758Z"`, `"2026-03-12T19:12:42.354Z"`, `"2026-03-12T19:38:13.921Z"`, `"2026-03-12T20:15:46.427Z"`, `"2026-03-13T13:52:13.585Z"`, `"2026-03-13T14:18:18.080Z"`
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.47"`, `"2.1.49"`, `"2.1.50"`, `"2.1.59"`, `"2.1.63"`, `"2.1.68"`, `"2.1.71"`, `"2.1.72"`, `"2.1.74"`

### Sample

```json
{
  "content": "<command-name>/usage</command-name>\n            <command-message>usage</command-... [130b total]",
  "cwd": "/Volumes/Source/bots/aj",
  "gitBranch": "main",
  "isMeta": false,
  "isSidechain": false,
  "level": "info",
  "parentUuid": "13800923-ce65-4395-b438-54bc3b1361d5",
  "sessionId": "39a5d7b2-e7c3-4370-9024-0b9ea22e6a13",
  "slug": "reactive-foraging-spindle",
  "subtype": "local_command",
  "timestamp": "2026-02-27T22:19:39.874Z",
  "type": "system",
  "userType": "external",
  "uuid": "a312151d-ec06-4149-aafa-2a80a1a00d9d",
  "version": "2.1.59"
}
```

---

## `system:stop_hook_summary`

**Count:** 577 instances

Summary of post-turn hook execution results

### Fields

- **`agentId`**?: `string` — values: `"acompact-10ea9408a00175a0"`, `"acompact-21da4b8d4ddbfcec"`, `"acompact-2946bd3e32d321d8"`, `"acompact-74af726bf45c73f9"`, `"acompact-74e55408baff24f1"`, `"acompact-99ab86a367d94b2a"`, `"acompact-b3914f2f711b83cd"`, `"acompact-d9621f0a65e49e00"`, `"acompact-f538e286804c8ad1"`, `"acompact-f64ce760c8b83321"`, `"acompact-ff06765bb152628c"`, `"aside_question-ad48711b92cb30ef"`
- **`cwd`**: `string` — values: `"/Users/gmr"`, `"/Volumes/Source/backend-libraries/awatlassian"`, `"/Volumes/Source/backend-libraries/awpromptlayer"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/bots/aj/.claude/worktrees/abstract-snuggling-token"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/config-management/eks"`, `"/Volumes/Source/gmr/aiorabbit"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/gmr/email-normalize"`, `"/Volumes/Source/gmr/llm-cost"`, `"/Volumes/Source/imbi/main/imbi"`, `"/Volumes/Source/imbi/main/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-automations"`, `"/Volumes/Source/schemata/clickhouse"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"chore/bump-awatlassian-1.13"`, `"feature/channel-prompts"`, `"feature/modernize-repo"`, `"feature/productivity-tracking"`, `"feature/question-detection-and-version-fixes"`, `"feature/session-naming"`, `"feature/skip-dns"`, `"feature/sparkle-auto-updates"`, `"feature/v2-build-system"`, `"fix-multisegment-tld-normalization-fastmail"`, `"fix/INT-6988-text-like-mimetypes"`, `"fix/code-review-bugs-and-optimizations"`, `"fix/scheduled-task-response-leaking"`, `"fix/waiting-state-priority"`, `"main"`, `"master"`, `"rollbar-instrumentation"`, `"worktree-abstract-snuggling-token"`
- **`hasOutput`**: `boolean`
- **`hookCount`**: `integer`
- **`hookErrors`**: `array`
  Array of: `string`
- **`hookInfos`**: `array`
  Array element:
    - **`command`**: `string` — values: `"${CLAUDE_PLUGIN_ROOT}/scripts/session-status.sh"`, `"~/.claude/hooks/session-status.sh"`
    - **`durationMs`**: `integer`
- **`isSidechain`**: `boolean`
- **`level`**: `string` — values: `"suggestion"`
- **`parentUuid`**: `string` — free-form text
- **`preventedContinuation`**: `boolean`
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`stopReason`**: `string` — values: `""`
- **`subtype`**: `string` — values: `"stop_hook_summary"`
- **`timestamp`**: `string`
- **`toolUseID`**: `string` — free-form text
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.71"`, `"2.1.72"`, `"2.1.73"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "hasOutput": false,
  "hookCount": 1,
  "hookErrors": [],
  "hookInfos": [
    {
      "command": "${CLAUDE_PLUGIN_ROOT}/scripts/session-status.sh",
      "durationMs": 178
    }
  ],
  "isSidechain": false,
  "level": "suggestion",
  "parentUuid": "8b62e177-9c6f-46bf-813f-8c1c4620f5c6",
  "preventedContinuation": false,
  "sessionId": "bd7653e2-93a9-404e-be30-ad5718acbc1f",
  "stopReason": "",
  "subtype": "stop_hook_summary",
  "timestamp": "2026-03-12T14:32:54.883Z",
  "toolUseID": "7942c263-83e8-4e53-be26-77d00595ca20",
  "type": "system",
  "userType": "external",
  "uuid": "416cbae5-dc76-40a7-ab01-68ad0ff1f6df",
  "version": "2.1.73"
}
```

---

## `system:turn_duration`

**Count:** 929 instances

Records the wall-clock duration of an assistant turn

### Fields

- **`agentId`**?: `string` — values: `"acompact-10ea9408a00175a0"`, `"acompact-5ee709d13d9967c1"`, `"acompact-74af726bf45c73f9"`, `"acompact-74e55408baff24f1"`, `"acompact-7f5cf2a7f3ba6ad6"`, `"acompact-99ab86a367d94b2a"`, `"aside_question-ad48711b92cb30ef"`
- **`cwd`**: `string`
- **`durationMs`**: `integer`
- **`gitBranch`**: `string`
- **`isMeta`**: `boolean`
- **`isSidechain`**: `boolean`
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`subtype`**: `string` — values: `"turn_duration"`
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`timestamp`**: `string`
- **`type`**: `string` — values: `"system"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.39"`, `"2.1.41"`, `"2.1.42"`, `"2.1.44"`, `"2.1.45"`, `"2.1.47"`, `"2.1.49"`, `"2.1.50"`, `"2.1.53"`, `"2.1.56"`, `"2.1.58"`, `"2.1.59"`, `"2.1.62"`, `"2.1.63"`, `"2.1.66"`, `"2.1.68"`, `"2.1.69"`, `"2.1.70"`, `"2.1.71"`, `"2.1.72"`, `"2.1.73"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "durationMs": 106758,
  "gitBranch": "feature/add-write-capabilities",
  "isMeta": false,
  "isSidechain": false,
  "parentUuid": "c760ab3c-4c18-4550-b437-7326af9fee24",
  "sessionId": "0f5e5c23-3911-4875-a57a-06657a2058e9",
  "slug": "sequential-napping-duckling",
  "subtype": "turn_duration",
  "timestamp": "2026-02-12T21:37:01.793Z",
  "type": "system",
  "userType": "external",
  "uuid": "11116f4c-8d8c-4fc3-9401-f49a989447c3",
  "version": "2.1.39"
}
```

---

## `user:image`

**Count:** 3 instances

User message containing an image

### Fields

- **`cwd`**: `string` — values: `"/Volumes/Source/imbi/features/ai-assistant/imbi-common"`, `"/Volumes/Source/imbi/main/imbi-api"`, `"/Volumes/Source/imbi/main/imbi-ui"`
- **`gitBranch`**: `string` — values: `"HEAD"`
- **`imagePasteIds`**: `array`
  Array of: `integer`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`source`**: `object`
        - **`data`**: `string` — free-form text
        - **`media_type`**: `string` — values: `"image/png"`
        - **`type`**: `string` — values: `"base64"`
      - **`type`**: `string` — values: `"image"`
  - **`role`**: `string` — values: `"user"`
- **`parentUuid`**: `string` — free-form text
- **`permissionMode`**: `string` — values: `"acceptEdits"`
- **`sessionId`**: `string` — free-form text
- **`slug`**: `string` — values: `"groovy-baking-badger"`, `"hazy-mixing-panda"`
- **`teamName`**: `string` — values: `"service-account-revamp"`
- **`timestamp`**: `string` — values: `"2026-03-02T21:34:25.461Z"`, `"2026-03-03T01:47:23.921Z"`, `"2026-03-03T01:52:28.501Z"`
- **`type`**: `string` — values: `"user"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.63"`

### Sample

```json
{
  "cwd": "/Volumes/Source/imbi/features/ai-assistant/imbi-common",
  "gitBranch": "HEAD",
  "imagePasteIds": [
    1
  ],
  "isSidechain": false,
  "message": {
    "content": [
      {
        "source": {
          "data": "[...]",
          "media_type": "[...]",
          "type": "[...]"
        },
        "type": "image"
      }
    ],
    "role": "user"
  },
  "parentUuid": "a333bc4d-7a17-4dad-999b-028e96d13b77",
  "permissionMode": "acceptEdits",
  "sessionId": "7e026d2f-4dd2-47b0-81d8-03c4631dbc43",
  "slug": "groovy-baking-badger",
  "timestamp": "2026-03-02T21:34:25.461Z",
  "type": "user",
  "userType": "external",
  "uuid": "95d3e1b5-f1b5-463d-93fe-5351e1b32249",
  "version": "2.1.63"
}
```

---

## `user:image+text`

**Count:** 51 instances

User message with image and text content

### Fields

- **`agentId`**?: `string` — values: `"acompact-10ea9408a00175a0"`, `"acompact-21da4b8d4ddbfcec"`, `"acompact-2946bd3e32d321d8"`, `"acompact-5ee709d13d9967c1"`, `"acompact-74e55408baff24f1"`, `"acompact-d9621f0a65e49e00"`, `"acompact-f538e286804c8ad1"`
- **`cwd`**: `string` — values: `"/Users/gmr"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/claude/git-repository"`, `"/Volumes/Source/gmr/claude-status"`, `"/Volumes/Source/gmr/email-normalize"`, `"/Volumes/Source/imbi/main/imbi-api"`
- **`gitBranch`**: `string` — values: `"HEAD"`, `"feature/confluence-reindex-tool"`, `"feature/modernize-repo"`, `"feature/productivity-tracking"`, `"feature/question-detection-and-version-fixes"`, `"feature/session-naming"`, `"feature/simplify-awpromptlayer-integration"`, `"feature/sparkle-auto-updates"`, `"feature/workflow-reply-in-thread"`, `"fix/jira-escaped-newlines"`, `"main"`
- **`imagePasteIds`**?: `array`
  Array of: `integer`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`source`**?: `object`
        - **`data`**: `string` — free-form text
        - **`media_type`**: `string` — values: `"image/png"`
        - **`type`**: `string` — values: `"base64"`
      - **`text`**?: `string` — free-form text
      - **`type`**: `string` — values: `"image"`, `"text"`
  - **`role`**: `string` — values: `"user"`
- **`parentUuid`**: `string` — free-form text
- **`permissionMode`**: `string` — values: `"acceptEdits"`, `"default"`
- **`promptId`**?: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string` — values: `"drifting-gliding-wilkinson"`, `"dynamic-seeking-stardust"`, `"glimmering-spinning-pixel"`, `"groovy-tickling-snowflake"`, `"hashed-dancing-token"`, `"hazy-mixing-panda"`, `"idempotent-bouncing-karp"`, `"imperative-discovering-mist"`, `"reactive-foraging-spindle"`, `"replicated-purring-pinwheel"`, `"resilient-baking-lemon"`, `"rustling-brewing-meerkat"`, `"shiny-munching-starlight"`, `"structured-inventing-garden"`, `"swirling-orbiting-yao"`, `"tingly-finding-owl"`, `"validated-wobbling-hellman"`, `"witty-sleeping-pearl"`, `"zesty-brewing-puppy"`
- **`teamName`**: `string` — values: `"service-account-revamp"`
- **`thinkingMetadata`**?: `object`
  - **`maxThinkingTokens`**: `integer`
- **`timestamp`**: `string`
- **`todos`**?: `array`
  Array of: ``
- **`type`**: `string` — values: `"user"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.42"`, `"2.1.45"`, `"2.1.50"`, `"2.1.59"`, `"2.1.63"`, `"2.1.70"`, `"2.1.71"`, `"2.1.72"`, `"2.1.73"`, `"2.1.74"`, `"2.1.75"`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "imagePasteIds": [
    3
  ],
  "isSidechain": false,
  "message": {
    "content": [
      {
        "text": "I'm trying to build a pkg file:\n\nsecurity find-identity -v\n  1) 6C05DC075A5E6285... [808b total]",
        "type": "text"
      },
      {
        "source": {
          "data": "[...]",
          "media_type": "[...]",
          "type": "[...]"
        },
        "type": "image"
      }
    ],
    "role": "user"
  },
  "parentUuid": "3574f619-b9cd-47bf-9c77-55f23ad8a3ce",
  "permissionMode": "default",
  "promptId": "3f8c0e5a-d853-4677-a3fc-a5cf2c560e2c",
  "sessionId": "bd7653e2-93a9-404e-be30-ad5718acbc1f",
  "timestamp": "2026-03-12T14:32:43.917Z",
  "type": "user",
  "userType": "external",
  "uuid": "5f4f3de7-6c33-4e4c-901f-6fae2a7713f9",
  "version": "2.1.73"
}
```

---

## `user:text`

**Count:** 3764 instances

User message with plain text content (string)

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`gitBranch`**: `string`
- **`isCompactSummary`**?: `boolean`
- **`isMeta`**?: `boolean`
- **`isSidechain`**: `boolean`
- **`isVisibleInTranscriptOnly`**?: `boolean`
- **`message`**: `object`
  - **`content`**: `array | string` — free-form text
    Array element:
      - **`text`**: `string` — free-form text
      - **`type`**: `string` — values: `"text"`
  - **`role`**: `string` — values: `"user"`
- **`parentUuid`**: `null | string` — free-form text
- **`permissionMode`**?: `string` — values: `"acceptEdits"`, `"default"`, `"plan"`
- **`planContent`**?: `string` — free-form text
- **`promptId`**?: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`sourceToolUseID`**?: `string` — free-form text
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`thinkingMetadata`**?: `object`
  - **`maxThinkingTokens`**: `integer`
- **`timestamp`**: `string`
- **`todos`**?: `array`
  Array of: ``
- **`type`**: `string` — values: `"user"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "isMeta": true,
  "isSidechain": false,
  "message": {
    "content": "<local-command-caveat>Caveat: The messages below were generated by the user whil... [245b total]",
    "role": "user"
  },
  "parentUuid": "25701410-9018-4bd8-92f1-0df76a493110",
  "sessionId": "0be288e3-4239-4ef5-9656-92608edb7d49",
  "timestamp": "2026-03-13T16:36:36.762Z",
  "type": "user",
  "userType": "external",
  "uuid": "b71eed32-66c2-46e7-a5cc-0f7c64aabc8a",
  "version": "2.1.74"
}
```

---

## `user:text+tool_result`

**Count:** 42 instances

User message with both text and tool result content

### Fields

- **`agentId`**?: `string` — values: `"a05ab2231a9ea7730"`, `"a96b62417a5f8e9f6"`, `"aba37a5672d7fb0e9"`, `"acompact-5ee709d13d9967c1"`, `"ad4a07fcd9c672504"`, `"ae20c13adf374aeb8"`
- **`cwd`**: `string` — values: `"/Volumes/Source/backend-libraries/awatlassian"`, `"/Volumes/Source/bots/aj"`, `"/Volumes/Source/consumers/clickhouse-materialization"`, `"/Volumes/Source/schemata/clickhouse"`
- **`gitBranch`**: `string` — values: `"feature/adf-pydantic-models-and-json-detection"`, `"feature/awatlassian-1.10"`, `"feature/ignore-clear-and-message-ts"`, `"main"`
- **`isSidechain`**: `boolean`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`content`**?: `array`
        Array element:
          - **`tool_name`**: `string` — values: `"Agent"`, `"Bash"`, `"Edit"`, `"Glob"`, `"Grep"`, `"Read"`, `"Skill"`, `"Write"`, `"mcp__claude_ai_Gmail__gmail_get_profile"`, `"mcp__claude_ai_Slack__slack_read_user_profile"`, `"mcp__claude_ai_Slack__slack_search_public_and_private"`, `"mcp__plugin_zoekt-mcp_zoekt-mcp__get_file"`, `"mcp__plugin_zoekt-mcp_zoekt-mcp__search"`, `"mcp__sentry__find_projects"`
          - **`type`**: `string` — values: `"tool_reference"`
      - **`text`**?: `string` — values: `"Tool loaded."`
      - **`tool_use_id`**?: `string` — free-form text
      - **`type`**: `string` — values: `"text"`, `"tool_result"`
  - **`role`**: `string` — values: `"user"`
- **`parentUuid`**: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string` — values: `"bubbly-orbiting-harbor"`, `"cuddly-bouncing-brook"`, `"hashed-dancing-token"`, `"moonlit-cuddling-crayon"`, `"smooth-bubbling-willow"`
- **`sourceToolAssistantUUID`**: `string` — free-form text
- **`timestamp`**: `string`
- **`toolUseResult`**?: `object`
  - **`matches`**: `array`
    Array of: `string`
  - **`query`**: `string` — values: `"select:Agent"`, `"select:Bash"`, `"select:Edit"`, `"select:Edit,Write"`, `"select:Grep"`, `"select:Read"`, `"select:Read,Bash,Glob"`, `"select:Read,Glob,Grep"`, `"select:Read,Grep,Glob,Bash,Agent"`, `"select:Skill"`, `"select:mcp__plugin_zoekt-mcp_zoekt-mcp__get_file"`, `"select:mcp__plugin_zoekt-mcp_zoekt-mcp__search,Edit,Write"`
  - **`total_deferred_tools`**: `integer`
- **`type`**: `string` — values: `"user"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string` — values: `"2.1.70"`

### Sample

```json
{
  "agentId": "ad4a07fcd9c672504",
  "cwd": "/Volumes/Source/backend-libraries/awatlassian",
  "gitBranch": "feature/adf-pydantic-models-and-json-detection",
  "isSidechain": true,
  "message": {
    "content": [
      {
        "content": [
          "[...]"
        ],
        "tool_use_id": "toolu_01MktJngmiypNQkKvT7j18g1",
        "type": "tool_result"
      },
      {
        "text": "Tool loaded.",
        "type": "text"
      }
    ],
    "role": "user"
  },
  "parentUuid": "24a5abf3-4d4f-4016-89f3-5d017fe0ccce",
  "sessionId": "e2f64615-7f40-4152-a25f-706841e60dfb",
  "slug": "smooth-bubbling-willow",
  "sourceToolAssistantUUID": "24a5abf3-4d4f-4016-89f3-5d017fe0ccce",
  "timestamp": "2026-03-06T21:44:14.062Z",
  "type": "user",
  "userType": "external",
  "uuid": "634705c7-ecbc-4f58-8a1d-bb221ae18b07",
  "version": "2.1.70"
}
```

---

## `user:tool_result`

**Count:** 37796 instances

Tool execution result returned to the assistant

### Fields

- **`agentId`**?: `string`
- **`cwd`**: `string`
- **`gitBranch`**: `string`
- **`isSidechain`**: `boolean`
- **`mcpMeta`**?: `object`
  - **`structuredContent`**: `object`
    - **`columns`**: `array`
      Array of: `string`
    - **`result`**?: `string` — free-form text
    - **`rows`**: `array`
      Array of: `array`
- **`message`**: `object`
  - **`content`**: `array`
    Array element:
      - **`content`**: `array | string` — free-form text
        Array element:
          - **`text`**?: `string` — free-form text
          - **`tool_name`**?: `string`
          - **`type`**: `string` — values: `"text"`, `"tool_reference"`
      - **`is_error`**?: `boolean`
      - **`tool_use_id`**: `string` — free-form text
      - **`type`**: `string` — values: `"tool_result"`
  - **`role`**: `string` — values: `"user"`
- **`parentUuid`**: `string` — free-form text
- **`promptId`**?: `string` — free-form text
- **`sessionId`**: `string` — free-form text
- **`slug`**?: `string`
- **`sourceToolAssistantUUID`**: `string` — free-form text
- **`teamName`**?: `string` — values: `"scheduled-tasks-m1"`, `"service-account-revamp"`
- **`timestamp`**: `string`
- **`toolUseResult`**?: `array | object | string` — free-form text
  _Dynamic map:_ `Map<string, array | boolean | float | integer | null | object | string>`
  Array element:
    - **`description`**?: `string` — values: `"Figma MCP docs: add custom rules"`, `"Figma MCP docs: avoid large frames"`, `"Figma MCP docs: bringing make context to your agent"`, `"Figma MCP docs: code connect integration"`, `"Figma MCP docs: getting 500 error"`, `"Figma MCP docs: images stopped loading"`, `"Figma MCP docs: intro"`, `"Figma MCP docs: local server installation"`, `"Figma MCP docs: mcp clients issues"`, `"Figma MCP docs: mcp vs agent"`, `"Figma MCP docs: plans access and permissions"`, `"Figma MCP docs: remote server installation"`, `"Figma MCP docs: server returning web code"`, `"Figma MCP docs: structure figma file"`, `"Figma MCP docs: stuck or slow"`, `"Figma MCP docs: tools and prompts"`, `"Figma MCP docs: tools not loading"`, `"Figma MCP docs: trigger specific tools"`, `"Figma MCP docs: variables vs code"`, `"Figma MCP docs: write effective prompts"`
    - **`mimeType`**?: `string` — values: `"text/markdown"`
    - **`name`**?: `string` — values: `"add-custom-rules"`, `"avoid-large-frames"`, `"bringing-make-context-to-your-agent"`, `"code-connect-integration"`, `"figjam-diagram-mcp-app"`, `"getting-500-error"`, `"images-stopped-loading"`, `"intro"`, `"local-server-installation"`, `"mcp-clients-issues"`, `"mcp-vs-agent"`, `"plans-access-and-permissions"`, `"remote-server-installation"`, `"server-returning-web-code"`, `"structure-figma-file"`, `"stuck-or-slow"`, `"tools-and-prompts"`, `"tools-not-loading"`, `"trigger-specific-tools"`, `"variables-vs-code"`, `"write-effective-prompts"`
    - **`server`**?: `string` — values: `"claude.ai Figma"`
    - **`text`**?: `string` — free-form text
    - **`title`**?: `string` — values: `"add custom rules"`, `"avoid large frames"`, `"bringing make context to your agent"`, `"code connect integration"`, `"getting 500 error"`, `"images stopped loading"`, `"intro"`, `"local server installation"`, `"mcp clients issues"`, `"mcp vs agent"`, `"plans access and permissions"`, `"remote server installation"`, `"server returning web code"`, `"structure figma file"`, `"stuck or slow"`, `"tools and prompts"`, `"tools not loading"`, `"trigger specific tools"`, `"variables vs code"`, `"write effective prompts"`
    - **`type`**?: `string` — values: `"text"`
    - **`uri`**?: `string` — values: `"file://figma/docs/add-custom-rules.md"`, `"file://figma/docs/avoid-large-frames.md"`, `"file://figma/docs/bringing-make-context-to-your-agent.md"`, `"file://figma/docs/code-connect-integration.md"`, `"file://figma/docs/getting-500-error.md"`, `"file://figma/docs/images-stopped-loading.md"`, `"file://figma/docs/intro.md"`, `"file://figma/docs/local-server-installation.md"`, `"file://figma/docs/mcp-clients-issues.md"`, `"file://figma/docs/mcp-vs-agent.md"`, `"file://figma/docs/plans-access-and-permissions.md"`, `"file://figma/docs/remote-server-installation.md"`, `"file://figma/docs/server-returning-web-code.md"`, `"file://figma/docs/structure-figma-file.md"`, `"file://figma/docs/stuck-or-slow.md"`, `"file://figma/docs/tools-and-prompts.md"`, `"file://figma/docs/tools-not-loading.md"`, `"file://figma/docs/trigger-specific-tools.md"`, `"file://figma/docs/variables-vs-code.md"`, `"file://figma/docs/write-effective-prompts.md"`, `"ui://widget/figjam-diagram-mcp-app.html"`
- **`type`**: `string` — values: `"user"`
- **`userType`**: `string` — values: `"external"`
- **`uuid`**: `string` — free-form text
- **`version`**: `string`

### Sample

```json
{
  "cwd": "/Users/gmr",
  "gitBranch": "HEAD",
  "isSidechain": false,
  "message": {
    "content": [
      {
        "content": "Exit code 129",
        "is_error": true,
        "tool_use_id": "toolu_01Ky4ezCdB7CY8cnsHzfDxUU",
        "type": "tool_result"
      }
    ],
    "role": "user"
  },
  "parentUuid": "2122e954-bdb7-42ec-be10-285220a9eadd",
  "sessionId": "2b9b74ae-8328-4e44-b5d8-9e4695afcd5d",
  "sourceToolAssistantUUID": "2122e954-bdb7-42ec-be10-285220a9eadd",
  "timestamp": "2026-03-12T21:25:55.274Z",
  "toolUseResult": "Error: Exit code 129",
  "type": "user",
  "userType": "external",
  "uuid": "bffe38b4-8361-42f0-8fb6-0eb60eb8518b",
  "version": "2.1.71"
}
```

