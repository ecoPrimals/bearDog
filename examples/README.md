# BearDog Examples

This directory holds **Cargo examples** (`cargo run --example <name>`). There are **20** `.rs` targets—API demos, entropy/HSM/CTAP exercises, discovery, and migration samples. They live alongside `README.md` only; there is **no** separate `broken/` tree.

## What’s here (by theme)

- **API / core** — `api_demo`, `basic_validation`, `simple_core_demo`, `MODERN_CONFIG_PATTERN_EXAMPLE`, `zero_hardcoding_migration`
- **Entropy & HSM** — `entropy_hardware_comparison`, `entropy_hardware_comparison_android`, `entropy_test_runner`, `universal_entropy_demo`, `cross_platform_hsm_unity`, `vendor_agnostic_multi_credential_demo`
- **Discovery** — `mdns_discovery_demo`
- **FIDO / SoloKey / Pixel** — `solokey_genetic_experiments`, `solokey_testing_suite`, `test_ctap2_getinfo`, `test_ctaphid_init_debug`, `test_fido2_hardware`, `test_pixel8a_detection`, `test_pixel8a_native`, `test_solo2_with_button`

## Running

```bash
cargo run --example api_demo
cargo run --example basic_validation
```

Other examples may need **features**, **hardware**, or **platform** support (Android-only sources, USB devices, etc.). Use `cargo run --example <name> -- --help` when the example defines a CLI.

## Note

Examples are maintained as references; not every target is expected to run on every machine. Prefer `cargo test --workspace` for CI-style verification of the library and binaries.
