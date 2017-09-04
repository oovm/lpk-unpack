# lpk 

A Rust library for decrypting `.lkp` files from `Live2dViewerEx`.

## Features

- Decrypt `.lkp` files from Live2dViewerEx
- Simple and easy-to-use API
- Integration with `anyhow` for error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lpk = { path = "../lpk-core" }
anyhow = "1.0"
```

## Usage

```rust
use lpk::LpkLoader;
use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let loader = LpkLoader::new(Path::new("example.lkp"))?;
    // Process the decrypted data
    Ok(())
}
```
