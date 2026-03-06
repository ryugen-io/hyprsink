# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

hyprsink is a system-wide theming tool that unifies configuration across your Linux desktop ecosystem. The core concept is "Single Source of Truth" - edit one central configuration (`~/.config/hypr/hyprsink.conf`) and propagate changes to all applications via Tera templates.

## Build Commands

```bash
just build              # Build release binaries
just install            # Full installation (config + build)
just test               # Run all tests
just lint               # Clippy + format check
just pre-commit         # Full pre-commit checks
just bench              # Run all benchmarks
```

Single test operations:
```bash
cargo test -- test_name           # Run single test
cargo test -- test_name --nocapture  # With stdout
```

## Architecture

### Single Crate Structure
```
hyprsink/
├── src/
│   ├── lib.rs              # Library root
│   ├── config.rs           # Config loading
│   ├── template.rs         # Template parsing
│   ├── db.rs               # Store (wincode-based storage)
│   ├── processor.rs        # Tera rendering
│   ├── packager.rs         # .pkg archive handling
│   ├── logger.rs           # hyprslog integration
│   ├── factory.rs          # Factory patterns
│   ├── bin/
│   │   └── hyprsink.rs      # Binary entry point
│   └── cli/
│       ├── mod.rs          # CLI module root
│       ├── args.rs         # Clap argument parsing
│       ├── logging.rs      # CLI logging
│       ├── cli_config.rs   # CLI-specific config
│       └── commands/       # Subcommands
└── benches/                # Benchmarks
```

### Features
- `default = ["cli"]` - CLI binary enabled by default
- `cli` - Enables CLI dependencies (clap, tracing, etc.)

### Key Types
- `Config`: Unified config from `hyprsink.conf` (theme + icons + layout)
- `Template`: Parsed .tpl file representation
- `Store`: wincode-based template storage
- `ConfigError`: Typed error enum using thiserror

### Data Flow
1. **Config**: `hyprsink.conf` -> `Config::load()` -> binary cache (wincode)
2. **Templates**: `.tpl` file -> `Template` -> `Store::add()`
3. **Apply**: `Store::list()` -> Tera render -> target files -> hook execution

## Error Handling Pattern

- Library code: Use `thiserror` for typed error enums
- Binary code (cli): Use `anyhow::Result` for propagation

## Single Instance Policy

Uses `flock()` on `~/.cache/hyprsink/hyprsink.lock` to prevent concurrent modifications. Debug viewer is exempt.

## Config Locations

- Config: `~/.config/hypr/hyprsink.conf`
- Binary cache: `~/.cache/hyprsink/config.bin`
- Data/DB: `~/.local/share/hyprsink/`
- Logs: via hyprslog (`~/.local/state/hyprslog/logs/`)

## CLI Commands Reference

```bash
hyprsink add <path>      # Add template (.tpl) or package (.pkg)
hyprsink list            # List all stored templates
hyprsink list clear      # Remove all templates from store
hyprsink apply           # Apply all templates (render + run hooks)
hyprsink pack <dir>      # Package .tpl files into a .pkg archive
hyprsink compile         # Pre-compile config into binary cache
hyprsink --debug         # Spawn debug viewer in separate terminal
```

## Logging

hyprsink uses hyprslog for logging. See the hyprslog project for log presets and configuration.
