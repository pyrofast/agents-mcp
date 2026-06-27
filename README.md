# mcplink

Universal MCP config sync daemon. One source of truth, all agents stay in sync.

## Problem

Every AI agent stores MCP config in its own path with its own schema. Add a server once → edit 6 files.

## Solution

`.agents/mcp.json` is the single source of truth. `mcplink` watches it and propagates every change to all agents automatically.

## Supported agents

| Agent | Config path | Key |
|---|---|---|
| Cursor | `.cursor/mcp.json` | `mcpServers` |
| Claude Code | `.mcp.json` | `mcpServers` |
| Copilot | `.github/mcp.json` | `mcpServers` |
| VS Code | `.vscode/mcp.json` | `servers` |
| Devin Desktop (formerly Windsurf) | `.windsurf/mcp.json` | `mcpServers` |
| OpenCode | `opencode.json` | `mcp` |

## Usage

```bash
# First run — installs as system service
mcplink

# Commands
mcplink status    # Show daemon status and synced agents
mcplink sync      # Force sync now
mcplink agents list  # Show agents installed on this OS
mcplink als          # Alias for agents list
mcplink stop      # Stop the daemon
mcplink uninstall # Remove service
```

## Source of truth

Create `.agents/mcp.json` in your project root:

```json
{
  "servers": {
    "memlink": {
      "transport": "http",
      "url": "https://memlink.cloud/mcp",
      "headers": {
        "Authorization": "Bearer TOKEN"
      }
    },
    "filesystem": {
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user"]
    }
  }
}
```

## Install

### Linux / macOS
```bash
curl -fsSL https://raw.githubusercontent.com/pyrofast/mcplink/main/install.sh | bash
```

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/pyrofast/mcplink/main/install.ps1 | iex
```

The binary auto-installs as a system daemon on first run.

## Build from source

```bash
cargo build --release
```
