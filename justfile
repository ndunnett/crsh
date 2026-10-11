set no-exit-message
set dotenv-load
set script-interpreter := ["just", "script-runner"]

[windows]
set shell := ["powershell.exe", "-NoLogo", "-NoProfile"]

[private]
@default:
    just --list --list-submodules

[private]
@script-runner +script:
    PYTHONPATH={{ source_directory() }}/scripts uv run --quiet --no-project --script {{ script }}

# Format all rust files
@fmt:
    cargo fmt --all
alias f := fmt

# Run clippy fix
@fix:
    cargo clippy --quiet --workspace --all-features --all-targets --no-deps --fix --allow-dirty

# Run Clippy for all targets/features
@check:
    cargo clippy --quiet --workspace --all-features --all-targets --no-deps
alias c := check

# Run nextest for all targets/features with the given filter
@test *filter:
    cargo nextest run --cargo-quiet --workspace --all-targets --all-features -- {{ filter }}
alias t := test

# Run cargo deny to check dependencies
@deny:
    cargo deny --workspace --all-features check

# Full verification sweep + format
@verify: deny check test fmt
alias v := verify

# Print the size of the final binary in megabytes
[script]
size:
    import size
    size.run()
