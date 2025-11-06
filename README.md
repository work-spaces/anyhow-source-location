# anyhow-source-location

Two simple macros for adding the source location to `anyhow` error contexts.

To strip the full path to the source location, add this to `.cargo/config.toml`

```toml
[build]
rustflags = ["--remap-path-prefix=${CARGO_MANIFEST_DIR}=."]
```
