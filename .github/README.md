# BearDog GitHub Configuration

This directory contains GitHub-specific configuration files.

## Workflows

- **`workflows/ci.yml`** — Quality gates: fmt check, clippy (`-D warnings`), `cargo deny`, `cargo test --workspace`, llvm-cov 90% gate
- **`workflows/notify-plasmidbin.yml`** — Dispatch to plasmidBin for binary harvesting on push to main
- **`workflows/notify-sporeprint.yml`** — Dispatch to sporePrint for validation metrics on push to main

## Note

BearDog is a sovereign, privacy-focused project. GitHub is used for code hosting and CI/CD. Primary development may occur elsewhere for sovereignty reasons.
