# ZeroClaw Configuration Guide

A task-oriented guide to configuring ZeroClaw. For flat key/default tables, see [config-reference.md](config-reference.md).

Last updated: **February 28, 2026**.

## Configuration Overview

ZeroClaw is configured via a TOML file at `~/.zeroclaw/config.toml`. The resolution order at startup is:

1. `ZEROCLAW_WORKSPACE` env var (if set)
2. `~/.zeroclaw/active_workspace.toml` marker (if present)
3. `~/.zeroclaw/config.toml` (default)

You can export the full JSON Schema with `zeroclaw config schema` and validate your config with `zeroclaw doctor`.

## Quick Start — Minimal Config

After running `zeroclaw onboard`, you get a working config. Here is the minimal set of keys to get started:

```toml
# ~/.zeroclaw/config.toml

default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"
default_temperature = 0.7
api_key = "sk-or-v1-..."
```

That's it — you can now run `zeroclaw` in CLI mode.

## Configuration Sections at a Glance

| Section | What It Controls | Default State |
|---|---|---|
| [Core keys](#core-keys) | Provider, model, temperature, API key | `openrouter` / `claude-sonnet-4-6` / `0.7` |
| [`[agent]`](#agent-behavior) | Tool iterations, history, context size | Enabled, conservative defaults |
| [`[autonomy]`](#autonomy--permissions) | Shell permissions, approval gates, risk levels | `supervised`, deny-by-default |
| [`[security.*]`](#security) | OTP, emergency stop, URL access, syscall anomaly, perplexity filter | Mostly disabled by default |
| [`[memory]`](#memory) | Conversation memory backend, embeddings | `sqlite`, no embeddings |
| [`[channels_config.*]`](#channels) | Telegram, Discord, Slack, WhatsApp, and 15+ more | All disabled |
| [`[gateway]`](#gateway) | HTTP API server for external integrations | `127.0.0.1:42617`, pairing required |
| [`[providers]` / `[provider]`](#provider-configuration) | Provider-specific overrides, reasoning level | Provider defaults |
| [`[[model_routes]]`](#model-routing) | Route task hints to specific provider+model combos | No routes |
| [`[agents.<name>]`](#delegate-sub-agents) | Multi-agent delegation workflows | No delegates |
| [`[research]`](#research-phase) | Proactive info gathering before responses | Disabled |
| [`[skills]`](#skills) | Community skills, ClawhHub integration | Disabled |
| [`[cost]`](#cost-tracking) | Spending limits and budget enforcement | Disabled |
| [`[browser]`](#browser-automation) | Browser tools and automation backends | Disabled |
| [`[http_request]`](#http-requests) | Outbound HTTP API calls | Disabled |
| [`[web_search]` / `[web_fetch]`](#web-search--fetch) | Web search and page extraction | Disabled |
| [`[multimodal]`](#multimodal-images) | Image handling in messages | 4 images, 5 MB each |
| [`[runtime]`](#runtime) | Execution backend (native, Docker, WASM) | `native` |
| [`[observability]`](#observability) | Metrics, traces, OTLP export | `none` |
| [`[proxy]`](#proxy) | HTTP/HTTPS/SOCKS5 proxy for outbound traffic | Disabled |
| [`[storage]`](#storage) | Persistent storage backend | `sqlite` |
| [`[tunnel]`](#tunnel) | Public gateway exposure | Disabled |
| [`[hardware]` / `[peripherals]`](#hardware--peripherals) | Physical device access (STM32, GPIO, ESP32) | Disabled |
| [`[composio]`](#composio) | Managed OAuth tool integrations | Disabled |
| [`[hooks]` / `[plugins]`](#hooks--plugins) | Lifecycle hooks and plugin system | Defaults |
| [`[transcription]`](#voice-transcription) | Voice-to-text (Whisper via Groq) | Disabled |
| [`[agents_ipc]`](#inter-agent-communication) | Multi-agent IPC on same host | Disabled |
| [`[mcp]`](#mcp-servers) | External MCP server connections | None |
| [`[secrets]`](#secrets-encryption) | Config value encryption at rest | Enabled |
| [`[identity]`](#identity) | Agent identity format | `openclaw` |
| [`[cron]` / `[scheduler]`](#scheduled-tasks) | Periodic task execution | Disabled |
| [`[heartbeat]`](#heartbeat) | Periodic health pings | Disabled |
| [`[goal_loop]`](#goal-loop) | Autonomous long-term goal execution | Disabled |
| [`[coordination]`](#coordination) | Delegate agent runtime coordination | Defaults |

## Core Keys

The top-level keys control which AI provider and model ZeroClaw uses.

```toml
default_provider = "openrouter"          # provider ID or alias
default_model = "anthropic/claude-sonnet-4-6"  # model routed through provider
default_temperature = 0.7                # 0.0–2.0
api_key = "sk-or-v1-..."                 # provider API key
api_url = "http://10.0.0.1:11434"        # optional base URL override
model_support_vision = true              # force vision on/off (unset = auto)
```

Environment variable overrides (highest precedence first):

| Config Key | Env Var (primary) | Env Var (fallback) |
|---|---|---|
| `default_provider` | `ZEROCLAW_PROVIDER` | `PROVIDER` |
| `default_model` | `ZEROCLAW_MODEL` | — |
| `default_temperature` | `ZEROCLAW_TEMPERATURE` | — |
| `api_key` | `ZEROCLAW_API_KEY` | `API_KEY` |
| `model_support_vision` | `ZEROCLAW_MODEL_SUPPORT_VISION` | `MODEL_SUPPORT_VISION` |

Supported providers include: `openrouter`, `anthropic`, `openai`, `ollama`, `gemini`, `groq`, `mistral`, `deepseek`, `xai`, `together`, `fireworks`, `perplexity`, `cohere`, `venice`, `moonshot`, `glm`, `zai`, `minimax`, `qianfan`, `dashscope`, `cloudflare`, `vercel`, and `custom:<url>`. See [providers-reference.md](providers-reference.md) for the full matrix.

## Agent Behavior

Control the agent's conversation and tool-use behavior.

```toml
[agent]
compact_context = true          # reduce context for small models (≤13B)
max_tool_iterations = 20        # max tool-call turns per user message
max_history_messages = 50       # conversation history retention
parallel_tools = false          # parallel tool execution within a turn
```

## Autonomy & Permissions

The autonomy system controls what actions the agent can perform without asking.

```toml
[autonomy]
level = "supervised"            # read_only | supervised | full
workspace_only = true           # restrict to workspace directory
allowed_commands = ["git", "cargo", "ls", "cat"]  # shell command allowlist
forbidden_paths = ["/etc", "/root", "~/.ssh"]     # path denylist
allowed_roots = ["~/projects"]  # additional allowed directories
max_actions_per_hour = 20
auto_approve = []               # always-approved tool operations
always_ask = []                 # always-prompt tool operations
```

Key concepts:

- **`read_only`** — no writes, no shell execution
- **`supervised`** — medium-risk actions require approval
- **`full`** — skips medium-risk approval (guardrails still enforced)
- Use `allowed_commands = ["*"]` to allow any command (risk gates still apply)
- On non-CLI channels, use `/approve <tool>` to persist approvals

## Security

### OTP (One-Time Password) Gating

Protect sensitive tool actions with TOTP verification.

```toml
[security.otp]
enabled = true
method = "totp"                 # totp | pairing | cli-prompt
token_ttl_secs = 30
cache_valid_secs = 300
gated_actions = ["shell", "browser_open"]
gated_domains = ["*.chase.com", "accounts.google.com"]
gated_domain_categories = ["banking"]
```

### Emergency Stop

Hardware-style kill switch for the agent runtime.

```toml
[security.estop]
enabled = false
state_file = "~/.zeroclaw/estop-state.json"
require_otp_to_resume = true
```

Use `zeroclaw estop` to engage and `zeroclaw estop resume` to clear.

### URL Access Policy

Controls which network targets tools can reach.

```toml
[security.url_access]
block_private_ip = true                    # block local/private addresses
allow_loopback = false                     # permit localhost
allow_cidrs = ["100.64.0.0/10"]            # CIDR exceptions
allow_domains = ["internal.example"]       # domain exceptions
```

### Syscall Anomaly Detection

Monitors command output for suspicious syscall patterns.

```toml
[security.syscall_anomaly]
enabled = true
strict_mode = false
alert_on_unknown_syscall = true
max_denied_events_per_minute = 5
log_path = "syscall-anomalies.log"
```

### Adversarial Prompt Filter

Blocks adversarial suffix injection before provider calls.

```toml
[security.perplexity_filter]
enable_perplexity_filter = false    # opt-in
perplexity_threshold = 18.0
suffix_window_chars = 64
```

## Memory

Configure how ZeroClaw stores and retrieves conversation context.

```toml
[memory]
backend = "sqlite"              # sqlite | lucid | markdown | none
auto_save = true                # persist user inputs
embedding_provider = "none"     # none | openai | custom endpoint
embedding_model = "text-embedding-3-small"
embedding_dimensions = 1536
vector_weight = 0.7             # hybrid ranking weights
keyword_weight = 0.3
```

To enable semantic memory search, set `embedding_provider` and optionally configure `[[embedding_routes]]`.

## Channels

ZeroClaw can connect to messaging platforms as a bot. Each channel is configured under `[channels_config.<name>]`.

Supported channels: **Telegram**, **Discord**, **Slack**, **Mattermost**, **WhatsApp** (Cloud API + Web), **Matrix**, **Signal**, **iMessage** (via Linq), **Nostr**, **Nextcloud Talk**, **Email**, **IRC**, **Lark**, **Feishu**, **DingTalk**, **QQ**, **Webhook**, **ClawdTalk**.

### Global Channel Options

```toml
[channels_config]
message_timeout_secs = 300      # base timeout (scales with tool depth)
```

### Telegram Example

```toml
[channels_config.telegram]
bot_token = "123456:ABC..."
allowed_users = ["your_username"]
stream_mode = "edit"            # edit | chunked | disabled
interrupt_on_new_message = false
```

### Discord Example

```toml
[channels_config.discord]
bot_token = "MTIz..."
guild_id = "123456789"
allowed_users = []              # [] = deny all, ["*"] = allow all
listen_to_bots = false
```

### Group Reply Behavior

Telegram, Discord, Slack, Mattermost, Lark, and Feishu support group reply configuration:

```toml
[channels_config.telegram.group_reply]
mode = "mention_only"           # all_messages | mention_only
allowed_sender_ids = ["admin"]  # bypass mention gating
```

See [channels-reference.md](channels-reference.md) for the full channel matrix and all per-channel options.

## Gateway

The HTTP gateway exposes ZeroClaw as an API server.

```toml
[gateway]
host = "127.0.0.1"
port = 42617
require_pairing = true          # require pairing before bearer auth
allow_public_bind = false       # block accidental public exposure
```

## Provider Configuration

Override provider-specific behavior.

```toml
[provider]
reasoning_level = "high"        # minimal | low | medium | high | xhigh

# Custom provider API mode (for custom:<url> providers)
provider_api = "openai-chat-completions"  # or "openai-responses"
```

### Named Provider Profiles

```toml
[model_providers.my-openai]
name = "openai"
base_url = "https://api.openai.com/v1"
wire_api = "responses"
requires_openai_auth = true
```

## Model Routing

Route task hints to specific provider+model combinations. This lets integrations use stable names while models evolve.

```toml
[[model_routes]]
hint = "reasoning"
provider = "openrouter"
model = "anthropic/claude-sonnet-4-6"
max_tokens = 8192

[[model_routes]]
hint = "fast"
provider = "groq"
model = "llama-3.3-70b-versatile"
```

Use hints in memory config: `embedding_model = "hint:semantic"`.

### Query Classification

Automatically route user messages to model hints based on content patterns.

```toml
[query_classification]
enabled = true

[[query_classification.rules]]
hint = "reasoning"
keywords = ["explain", "analyze", "why"]
min_length = 200
priority = 10

[[query_classification.rules]]
hint = "fast"
keywords = ["hi", "hello", "thanks"]
max_length = 50
priority = 5
```

## Delegate Sub-Agents

Define named sub-agents for multi-agent delegation workflows.

```toml
[agents.researcher]
provider = "openrouter"
model = "anthropic/claude-sonnet-4-6"
system_prompt = "You are a research assistant."
max_depth = 2
agentic = true
allowed_tools = ["web_search", "http_request", "file_read"]
max_iterations = 8

[agents.coder]
provider = "ollama"
model = "qwen2.5-coder:32b"
temperature = 0.2
```

## Research Phase

Enable proactive information gathering before the agent responds.

```toml
[research]
enabled = true
trigger = "keywords"            # never | always | keywords | length | question
keywords = ["find", "show", "check", "how many"]
max_iterations = 3
show_progress = true
```

## Skills

Configure community skill loading and ClawhHub integration.

```toml
[skills]
open_skills_enabled = false         # opt-in community skills
prompt_injection_mode = "full"      # full | compact (use compact for small models)
clawhub_token = "your-token-here"   # optional ClawhHub bearer token
```

## Cost Tracking

Enforce spending limits per day/month.

```toml
[cost]
enabled = true
daily_limit_usd = 10.00
monthly_limit_usd = 100.00
warn_at_percent = 80
allow_override = false
```

## Browser Automation

Enable browser tools for web interaction.

```toml
[browser]
enabled = true
allowed_domains = ["docs.example.com"]  # or ["*"] for all public
backend = "agent_browser"       # agent_browser | rust_native | computer_use | auto
```

## HTTP Requests

Enable outbound HTTP API calls.

```toml
[http_request]
enabled = true
allowed_domains = ["api.example.com"]   # deny-by-default
max_response_size = 1000000             # 1 MB
timeout_secs = 30
```

## Web Search & Fetch

```toml
[web_search]
enabled = true
provider = "duckduckgo"         # duckduckgo | brave | firecrawl
max_results = 5

[web_fetch]
enabled = true
provider = "fast_html2md"       # fast_html2md | nanohtml2text | firecrawl
allowed_domains = ["*"]
```

## Multimodal (Images)

```toml
[multimodal]
max_images = 4
max_image_size_mb = 5
allow_remote_fetch = false
```

Image markers in messages: `[IMAGE:/path/to/screenshot.png]`.

## Runtime

```toml
[runtime]
kind = "native"                 # native | docker | wasm
reasoning_enabled = true        # enable/disable provider-side reasoning
```

### WASM Runtime

```toml
[runtime.wasm]
tools_dir = "tools/wasm"
fuel_limit = 1000000
memory_limit_mb = 64

[runtime.wasm.security]
require_workspace_relative_tools_dir = true
reject_symlink_modules = true
capability_escalation_mode = "deny"     # deny | clamp
module_hash_policy = "warn"             # disabled | warn | enforce
```

## Observability

```toml
[observability]
backend = "otel"                # none | noop | log | prometheus | otel
otel_endpoint = "http://localhost:4318"
otel_service_name = "zeroclaw"
runtime_trace_mode = "rolling"  # none | rolling | full
runtime_trace_max_entries = 200
```

Query traces with: `zeroclaw doctor traces --limit 20`.

## Proxy

Route outbound traffic through a proxy.

```toml
[proxy]
enabled = true
http_proxy = "http://proxy.example.com:8080"
https_proxy = "http://proxy.example.com:8080"
all_proxy = "socks5://proxy.example.com:1080"
no_proxy = ["localhost", "127.0.0.1"]
scope = "zeroclaw"              # environment | zeroclaw | services
services = ["openai", "anthropic"]
```

## Storage

```toml
[storage]
provider = "sqlite"             # sqlite | postgres
# db_url = "postgres://localhost/zeroclaw"
# connect_timeout_secs = 5
```

## Tunnel

Expose the gateway publicly via a tunnel service.

```toml
[tunnel]
# Tunnel configuration for public webhook endpoints
```

## Hardware & Peripherals

### Hardware Wizard

```toml
[hardware]
enabled = true
transport = "serial"            # none | native | serial | probe
serial_port = "/dev/ttyACM0"
baud_rate = 115200
```

### Peripheral Boards

```toml
[peripherals]
enabled = true
datasheet_dir = "docs/datasheets"

[[peripherals.boards]]
board = "nucleo-f401re"
transport = "serial"
path = "/dev/ttyACM0"
baud = 115200

[[peripherals.boards]]
board = "rpi-gpio"
transport = "native"
```

## Composio

Managed OAuth tool integrations.

```toml
[composio]
enabled = true
api_key = "your-composio-key"
entity_id = "default"
```

## Hooks & Plugins

```toml
[hooks]
# Lifecycle hook configurations

[plugins]
# Plugin discovery and loading
```

See [PLUGINS.md](PLUGINS.md) for plugin development documentation.

## Voice Transcription

```toml
[transcription]
# Whisper API via Groq for voice message transcription
# api_url = "https://api.groq.com/openai/v1/audio/transcriptions"
# model = "whisper-large-v3-turbo"
```

## Inter-Agent Communication

```toml
[agents_ipc]
enabled = true
db_path = "~/.zeroclaw/agents.db"
staleness_secs = 300
```

All agents sharing the same `db_path` can discover each other and exchange messages.

## MCP Servers

Connect to external Model Context Protocol servers.

```toml
[mcp]
# External MCP server connection configurations
```

## Secrets Encryption

```toml
[secrets]
encrypt = true                  # encrypt sensitive values in config at rest
```

## Identity

```toml
[identity]
format = "openclaw"             # openclaw | aieos
# aieos_path = "identity.json"  # for AIEOS format
```

## Scheduled Tasks

```toml
[cron]
# Cron job definitions for periodic task execution

[scheduler]
# Scheduler configuration
```

## Heartbeat

```toml
[heartbeat]
# Periodic health ping configuration
```

## Goal Loop

```toml
[goal_loop]
# Autonomous long-term goal execution configuration
```

## Coordination

```toml
[coordination]
# Delegate agent runtime coordination settings
```

## Validating Your Configuration

After editing `config.toml`, validate with:

```bash
zeroclaw status         # show resolved config summary
zeroclaw doctor         # comprehensive health check
zeroclaw channel doctor # channel-specific diagnostics
zeroclaw service restart # apply changes to running service
```

## Common Configuration Recipes

### Cloud API with Spending Limits

```toml
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"
api_key = "sk-or-v1-..."

[cost]
enabled = true
daily_limit_usd = 5.00
monthly_limit_usd = 50.00
```

### Local Ollama with Compact Context

```toml
default_provider = "ollama"
default_model = "qwen2.5-coder:32b"
api_url = "http://localhost:11434"

[agent]
compact_context = true
```

### Telegram Bot with Web Search

```toml
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"
api_key = "sk-or-v1-..."

[channels_config.telegram]
bot_token = "123456:ABC..."
allowed_users = ["your_username"]

[web_search]
enabled = true
provider = "duckduckgo"

[web_fetch]
enabled = true
```

### Multi-Model Routing

```toml
default_provider = "groq"
default_model = "llama-3.3-70b-versatile"

[[model_routes]]
hint = "reasoning"
provider = "openrouter"
model = "anthropic/claude-sonnet-4-6"

[[model_routes]]
hint = "fast"
provider = "groq"
model = "llama-3.3-70b-versatile"

[query_classification]
enabled = true

[[query_classification.rules]]
hint = "reasoning"
keywords = ["explain", "analyze", "debug"]
min_length = 200
priority = 10
```

### Hardened Security Profile

```toml
[autonomy]
level = "supervised"
workspace_only = true
allowed_commands = ["git", "cargo", "ls"]
block_high_risk_commands = true

[security.otp]
enabled = true
method = "totp"
gated_actions = ["shell", "file_write", "browser_open"]
gated_domain_categories = ["banking", "medical"]

[security.url_access]
block_private_ip = true
allow_loopback = false

[security.perplexity_filter]
enable_perplexity_filter = true
```

## Related Documentation

- [config-reference.md](config-reference.md) — flat key/default reference tables
- [providers-reference.md](providers-reference.md) — provider IDs, aliases, credential env vars
- [channels-reference.md](channels-reference.md) — channel capabilities and setup paths
- [commands-reference.md](commands-reference.md) — CLI command reference
- [operations-runbook.md](operations-runbook.md) — day-2 runtime operations
- [troubleshooting.md](troubleshooting.md) — common issues and recovery
- [custom-providers.md](custom-providers.md) — custom provider endpoint setup
