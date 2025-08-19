default:
    echo 'Hello, world!'

alias fmt := format
format:
    # Format nix
    alejandra .

    # Fix spelling
    codespell --write-changes

    # Fix rust formatting
    cargo fmt

# Basic checks
alias check := lint
lint:
    # Check rust
    cargo check

    # Check spelling
    codespell

    # Check nix formatting
    alejandra -q .

    # Check rust formatting
    cargo fmt --check

    # Check clippy
    cargo clippy -- --deny warnings

# Strict checks
audit: && lint
    # Audit rust code
    cargo-deny check

    # Check for outdated dependencies
    cargo outdated -R --exit-code 1

update:
    # Update devenv
    devenv update

    # Update flake
    nix flake update

    # Update rust
    cargo update

fix: && format update
    cargo clippy --fix --allow-staged --allow-dirty

build:
    cargo build

test:
    cargo test