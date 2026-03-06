<p align="center">
  <img src="assets/header.svg" alt="hyprsink" />
</p>

<p align="center">
  <strong>Strict Corporate Design Enforcement for your System.</strong>
</p>

> "Single Source of Truth". One config change propagates to Shells, Scripts, Logs, GUIs, and TUI apps instantly.

---

## Mission

hyprsink unifies the theming and configuration of your entire ecosystem (e.g., Hyprland, Waybar, Alacritty). Instead of editing 10 different config files to change a color or font, you edit **one** central configuration. hyprsink then propagates these changes to all your installed applications ("Templates") via powerful Tera templates.

## Installation

### Option A: One-liner (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/ryugen-io/hyprsink/master/install.sh | bash
```

### Option B: From Source
```bash
git clone https://github.com/ryugen-io/hyprsink.git
cd hyprsink
just install
```

All methods will:
1. Create `~/.config/hypr/hyprsink.conf` with default configuration.
2. Install binary (`hyprsink`) to `~/.local/bin/`.

> Ensure `~/.local/bin` is in your `$PATH`.

---

## Quick Start

Get up and running in 3 steps:

```bash
# 1. Install hyprsink
curl -fsSL https://raw.githubusercontent.com/ryugen-io/hyprsink/master/install.sh | bash

# 2. Add an example template
hyprsink add ./assets/templates/hyprsbar.tpl

# 3. Apply all templates
hyprsink apply
```

### Typical Workflow

1. **Edit your config** in `~/.config/hypr/hyprsink.conf`
2. **Run** `hyprsink apply` to apply changes
3. **Done!** All configured apps update automatically

### Creating Your Own Template

```bash
# Create a new template file
cat > my-app.tpl << 'EOF'
[manifest]
name = "my-app-theme"
version = "0.1.0"
authors = ["Your Name"]
description = "My app theming"

[[targets]]
target = "~/.config/my-app/colors.conf"
content = """
background = "{{ colors.bg }}"
foreground = "{{ colors.fg }}"
accent = "{{ colors.primary }}"
"""

[hooks]
reload = "pkill -USR1 my-app"
EOF

# Add and apply
hyprsink add my-app.tpl
hyprsink apply
```

---

## Project Structure

```bash
.
├── crates/
│   ├── hi_core/         # Core Logic (Rust 2024)
│   └── hi_cli/          # CLI wrapper (`hyprsink`)
├── assets/
│   └── templates/       # Example .tpl files
├── Cargo.toml           # Workspace config
└── justfile             # Command runner
```

### Core Architecture

- **Logic**: `hi_core` (Rust 2024) handles all processing, rendering, and logic.
- **Storage**: Templates are stored in a high-performance **binary database** located in `~/.local/share/hyprsink/`, ensuring instant access and clean storage.

---

## Commands

### Template Management
```bash
# Add a single template or .pkg package
hyprsink add ./assets/templates/hyprsbar.tpl
hyprsink add ./my-theme.pkg

# List all stored templates
hyprsink list

# Apply all templates to the system
hyprsink apply

# Clear all templates from store
hyprsink list clear

# Enable/Disable templates
hyprsink list disable hyprsbar
hyprsink list enable hyprsbar
```

### Packaging
```bash
# Pack multiple .tpl files into a portable .pkg package
hyprsink pack ./my-templates/

# Specify custom output path
hyprsink pack ./my-templates/ --output ./my-theme.pkg
```

### Performance Optimization
```bash
# Pre-compile config file into binary format for faster startup
hyprsink compile
```

> Run `hyprsink compile` after changing your configuration file to cache it for instant loading.

---

## Debugging

Use standard Rust log filters to get verbose output:

```bash
RUST_LOG=debug hyprsink apply
RUST_LOG=debug hyprsink compile
```

---

## Robustness

hyprsink enforces a **Single Instance Policy** using OS-level file locking (`flock`). This ensures that only one instance manages the store or system configuration at a time, preventing database corruption and conflicts.

- **Automatic Cleanup**: If hyprsink crashes, the kernel releases the lock immediately.
- **Non-Blocking**: A second instance will fail immediately with a clear error message instead of hanging.

---

## Templates (`.tpl`)

A **Template** is a single TOML file that teaches hyprsink how to theme a specific application. Templates are stored in the database upon installation.

### Structure
```toml
[manifest]
name = "waybar-theme"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "Waybar styling integration"
license = "MIT"

[[targets]]
target = "~/.config/waybar/style.css"
content = """
* {
    font-family: "{{ fonts.ui }}";
    font-size: {{ fonts.size_ui }}px;
}
window#waybar {
    background-color: {{ colors.bg }};
    border-bottom: 2px solid {{ colors.primary }};
}
"""

[hooks]
reload = "pkill -SIGUSR2 waybar"
```

### Manifest Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique identifier for the template |
| `version` | Yes | Semantic version (e.g., `0.1.0`) |
| `authors` | Yes | List of author names |
| `description` | Yes | Short description |
| `license` | No | License identifier (e.g., `MIT`, `GPL-3.0`) |
| `ignored` | No | Set to `true` to disable without deleting (Default: `false`) |

---

## Packages (`.pkg`)

A **Package** is a portable zip archive containing multiple `.tpl` files. Use packages to distribute complete theme collections.

### Creating a Package
```bash
# Package all .tpl files from a directory
hyprsink pack ./my-theme-templates/

# Creates: my-theme-templates.pkg
```

### Installing a Package
```bash
# Extract and add all templates from a package
hyprsink add ./my-theme.pkg
```

> Packages are simply ZIP files with a `.pkg` extension. You can inspect their contents with any archive tool.

---

## Template Variables

Templates use the [Tera](https://keats.github.io/tera/) templating engine. The following variables are available:

### Colors (`colors.*`)
All colors defined in `hyprsink.conf`:
```
{{ colors.bg }}         -> #161925
{{ colors.fg }}         -> #F8F8F2
{{ colors.primary }}    -> #BD93F9
{{ colors.secondary }}  -> #FF79C6
{{ colors.success }}    -> #50FA7B
{{ colors.error }}      -> #FF5555
{{ colors.warn }}       -> #FFB86C
{{ colors.info }}       -> #8BE9FD
```

### Fonts (`fonts.*`)
```
{{ fonts.mono }}        -> JetBrainsMono Nerd Font
{{ fonts.ui }}          -> Roboto
{{ fonts.size_mono }}   -> 10
{{ fonts.size_ui }}     -> 11
```

### Icons (`icons.*`)
Icons from the active icon set (configured in `hyprsink.conf`):
```
{{ icons.success }}     ->  (or * in ASCII mode)
{{ icons.error }}       ->  (or ! in ASCII mode)
{{ icons.warn }}        ->
{{ icons.info }}        ->
{{ icons.net }}         -> 󰖩
```

---

## Configuration

Located at `~/.config/hypr/hyprsink.conf` - a single file containing all settings.

### Example hyprsink.conf
```toml
[theme]
name = "Sweet Dracula"
active_icons = "nerdfont"

[theme.colors]
bg = "#161925"
fg = "#F8F8F2"
primary = "#BD93F9"
# ... more colors

[theme.fonts]
mono = "JetBrainsMono Nerd Font"
ui = "Roboto"
size_mono = "10"
size_ui = "11"

[icons]
[icons.nerdfont]
success = ""
error = ""
# ... more icons

[icons.ascii]
success = "*"
error = "!"
# ... more icons

[layout]
[layout.tag]
prefix = "["
suffix = "]"
transform = "lowercase"

[layout.structure]
terminal = "{tag} {scope} {icon} {msg}"
file = "{timestamp} {tag} {msg}"

[layout.logging]
base_dir = "~/.local/state/hyprsink/logs"
path_structure = "{year}/{month}/{scope}"
filename_structure = "{level}.{year}-{month}-{day}.log"
write_by_default = true

# Presets
[presets.boot_ok]
level = "success"
scope = "SYSTEM"
msg = "startup complete"
```

---

## Uninstall

```bash
just uninstall
```
