# `cargo agents init`

Initialize `cargo-agents` for the current user, the current project, or both.

## Usage

```bash
cargo agents init [OPTIONS]
```

## Behavior

With no flags, `init` does whatever hasn't been done yet:

1. If no user-wide configuration exists (`~/.cargo-agents/config.toml`), runs user setup.
2. If no project configuration exists (`.cargo-agents/config.toml`), offers to set up the project.
3. If the project was initialized, runs [`cargo agents sync`](./cargo-agents-sync.md) automatically.

### `--user`

Set up user-wide configuration only. Can be run from any directory.

Prompts for:

- Which agent you use (e.g., Claude Code, Cursor)

Writes `~/.cargo-agents/config.toml` and, where applicable, registers a global hook so your agent automatically picks up project extensions on startup.

### `--project`

Set up the current project only. Must be run from within a Rust workspace.

Prompts for:

- Whether to set a project-level agent override (default: use each developer's own preference)

Scans workspace dependencies, discovers available extensions, and generates `.cargo-agents/config.toml`. Runs `cargo agents sync` afterward.

## Options

| Flag | Description |
|------|-------------|
| `--user` | Set up user-wide configuration only |
| `--project` | Set up project configuration only |
