#!/bin/bash
# devon - Bash wrapper for dev-on project switcher
# Source this file in your .bashrc to enable the devon command

# Main devon function - wrapper around dev-on binary
devon() {
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

  # Get project info from rust binary
  local result=$(dev-on get "$1" 2>&1)
  if [ $? -ne 0 ]; then
    echo "$result"
    return 1
  fi

  # Parse result: path|init_cmd1|init_cmd2|...
  IFS='|' read -ra PARTS <<< "$result"
  local project_path="${PARTS[0]}"

  # Change directory
  cd "$project_path" || return 1
  echo "Working on: $1 ($project_path)"

  # Run init commands
  for i in "${!PARTS[@]}"; do
    if [ $i -gt 0 ]; then
      eval "${PARTS[$i]}"
    fi
  done

  # Default: auto-activate .venv if present and no init commands
  if [ ${#PARTS[@]} -eq 1 ] && [ -f ".venv/bin/activate" ]; then
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
_devon_complete() {
  if ! command -v dev-on &> /dev/null; then
    return 0
  fi

  local projects=$(dev-on list 2>/dev/null)
  COMPREPLY=($(compgen -W "$projects" -- "${COMP_WORDS[1]}"))
}

complete -F _devon_complete devon
