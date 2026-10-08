#!/bin/bash
# devon - Shell wrapper for dev-on project switcher
# Source this file in your .bashrc or .zshrc to enable the devon command.
# Written to run under both bash (including macOS's bash 3.2) and zsh.

# Main devon function - wrapper around dev-on binary
devon() {
  local result project_path rest cmd nl=$'\n'

  if ! command -v dev-on &> /dev/null; then
    echo "Error: dev-on not installed"
    echo "Install with: cargo install --path /path/to/dev-on"
    return 1
  fi

  if [ -z "$1" ]; then
    echo "Usage: devon <project>"
    echo "Available projects:"
    dev-on list
    return 1
  fi

  # Get project info from rust binary. Assign separately from the
  # declaration so $? is dev-on's exit status, not local's.
  result=$(dev-on get "$1" 2>&1)
  if [ $? -ne 0 ]; then
    echo "$result"
    return 1
  fi

  # Parse result: the path, then one init command per line.
  # Uses plain parameter expansion rather than arrays, which differ
  # between bash and zsh.
  project_path="${result%%"$nl"*}"
  rest=""
  case "$result" in
    *"$nl"*) rest="${result#*"$nl"}" ;;
  esac

  # Change directory
  cd "$project_path" || return 1
  echo "Working on: $1 ($project_path)"

  if [ -n "$rest" ]; then
    # Run init commands
    while [ -n "$rest" ]; do
      cmd="${rest%%"$nl"*}"
      case "$rest" in
        *"$nl"*) rest="${rest#*"$nl"}" ;;
        *) rest="" ;;
      esac
      eval "$cmd"
    done
  elif [ -f ".venv/bin/activate" ]; then
    # Default: auto-activate .venv if present and no init commands
    source .venv/bin/activate
    echo "Activated .venv"
  fi
}

# Wrapper for list command
devon-list() {
  dev-on list
}

# Wrapper for add command
devon-add() {
  dev-on add "$@"
}

# Wrapper for edit command
devon-edit() {
  dev-on edit
}

# Tab completion
if [ -n "$ZSH_VERSION" ]; then
  _devon_complete() {
    command -v dev-on &> /dev/null || return 0
    compadd -- ${(f)"$(dev-on list 2>/dev/null)"}
  }
  # compdef only exists once compinit has run (most zsh setups, e.g.
  # oh-my-zsh, do this). Skip completion rather than error otherwise.
  if (( $+functions[compdef] )); then
    compdef _devon_complete devon
  fi
else
  _devon_complete() {
    if ! command -v dev-on &> /dev/null; then
      return 0
    fi

    local projects=$(dev-on list 2>/dev/null)
    COMPREPLY=($(compgen -W "$projects" -- "${COMP_WORDS[1]}"))
  }

  complete -F _devon_complete devon
fi
