# hyprsink Project Overview

## Purpose
hyprsink is a system-wide theming tool that unifies configuration across a Linux desktop ecosystem. The core concept is "Single Source of Truth" - edit one central configuration (`~/.config/hypr/hyprsink.conf`) and propagate changes to all applications via Tera templates.

## Tech Stack
- **Language**: Rust (2021 edition)
- **Build System**: Cargo workspace with justfile
- **Template Engine**: Tera
- **Serialization**: bincode (binary cache), serde
- **Error Handling**: thiserror (library), anyhow (binary)
- **Logging**: hyprslog (hl_core)
- **CLI**: Clap

## Architecture

### Single Crate Structure
```
hyprsink/
├── src/
│   ├── lib.rs              # Library root
│   ├── config.rs           # Config loading
│   ├── template.rs         # Template parsing
│   ├── db.rs               # Store (bincode-based storage)
│   ├── processor.rs        # Tera rendering
│   ├── packager.rs         # .pkg archive handling
│   ├── logger.rs           # hyprslog integration
│   ├── factory.rs          # Factory patterns
│   ├── bin/
│   │   └── hyprsink.rs      # Binary entry point
│   └── cli/
│       ├── mod.rs          # CLI module root
│       ├── args.rs         # Clap argument parsing
│       └── commands/       # Subcommands
└── benches/                # Benchmarks
```

### Features
- `default = ["cli"]` - CLI binary enabled by default
- `cli` - Enables CLI dependencies (clap, tracing, etc.)

### Key Modules
- `config` - Config loading from hyprsink.conf
- `template` - Template parsing and representation
- `db` - bincode-based template storage (Store)
- `processor` - Template rendering with Tera
- `packager` - .pkg archive handling
- `logger` - hyprslog integration
- `factory` - Factory patterns

## Data Locations
- Config: `~/.config/hypr/hyprsink.conf`
- Binary cache: `~/.cache/hyprsink/config.bin`
- Data/DB: `~/.local/share/hyprsink/`
- Logs: `~/.local/state/hyprslog/logs/`
- Lock file: `~/.cache/hyprsink/hyprsink.lock`

## CLI Commands
- `hyprsink add <path>` - Add template (.tpl) or package (.pkg)
- `hyprsink list` - List stored templates
- `hyprsink list clear` - Remove all templates
- `hyprsink apply` - Apply all templates
- `hyprsink pack <dir>` - Create .pkg archive
- `hyprsink compile` - Pre-compile config to binary cache
- `hyprsink --debug` - Debug viewer
