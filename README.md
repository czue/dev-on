# dev-on

A standalone project switcher for development environments, inspired by virtualenvwrapper's `workon` command.

## Features

- Quick switching between development projects
- Automatic virtual environment activation (.venv)
- Custom init commands per project
- Tab completion for project names
- Simple JSON configuration
- Cross-platform (Linux, macOS, Windows)

## Installation

### Install from source

```bash
cargo install --path .
```

### Install from crates.io (when published)

```bash
cargo install dev-on
```

## Setup

### 1. Install the Rust binary

The `dev-on` binary handles configuration management and project lookup.

### 2. Source the bash wrapper

Add to your `.bashrc` or `.bash_profile`:

```bash
source /path/to/dev-on/shell/devon.bash
```

Or copy the wrapper to your dotfiles and source it from there.

## Usage

### Initialize configuration

```bash
dev-on init
```

This creates `~/.devon.json` with an empty project list.

### Add projects

```bash
# Simple project (auto-activates .venv if present)
dev-on add myproject /path/to/myproject

# With custom init command
dev-on add myproject /path/to/myproject --init "source .venv/bin/activate"

# With multiple init commands
dev-on add myproject /path/to/myproject --init "source .venv/bin/activate" --init "export DEBUG=true"
```

### Switch to a project

```bash
devon myproject
```

This will:
1. Change to the project directory
2. Run any configured init commands
3. Auto-activate `.venv` if no init commands are specified

### List projects

```bash
devon-list
# or
dev-on list
```

### Edit configuration

```bash
devon-edit
# or
dev-on edit
```

Opens `~/.devon.json` in your `$EDITOR`.

### Remove a project

```bash
dev-on remove myproject
```

## Configuration

Configuration is stored in `~/.devon.json`:

```json
{
  "projects": {
    "myproject": {
      "path": "/home/user/projects/myproject",
      "init": [
        "source .venv/bin/activate",
        "export DJANGO_SETTINGS_MODULE=myproject.settings.dev"
      ]
    },
    "another": {
      "path": "/home/user/projects/another",
      "init": []
    }
  }
}
```

## Commands

### Binary commands (dev-on)

- `dev-on init` - Initialize config file
- `dev-on get <project>` - Get project path and init commands (used by shell wrapper)
- `dev-on list` - List all projects
- `dev-on add <alias> <path> [--init <cmd>]` - Add a project
- `dev-on remove <alias>` - Remove a project
- `dev-on edit` - Edit config file

### Shell wrapper commands (devon)

- `devon <project>` - Switch to project
- `devon-list` - List projects
- `devon-add` - Add project (passes through to dev-on add)
- `devon-edit` - Edit config (passes through to dev-on edit)

## Why Rust?

- Single fast binary with no runtime dependencies
- Cross-platform support
- Type-safe configuration handling
- Easy distribution via cargo

## License

MIT
