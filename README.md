# Crate search_path

Provides a very simple search path file finder.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.50-green.svg)
[![crates.io](https://img.shields.io/crates/v/search_path.svg)](https://crates.io/crates/search_path)
[![docs.rs](https://docs.rs/search_path/badge.svg)](https://docs.rs/search_path)
![Build](https://github.com/johnstonskj/rust-search_path/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-search_path/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-search_path.svg)](https://github.com/johnstonskj/rust-search_path/stargazers)

-----

# Example

The following example shows the common pattern of finding an executable command on the command
line.

```rust
use search_path::SearchPath;
use std::path::PathBuf;

fn which_command(cmd_name: &str) -> Option<PathBuf> {
    let search_path = SearchPath::new("PATH").unwrap();
    search_path.find_file(&PathBuf::from(cmd_name))
}
```

-----

## Changes

**Version 0.1.0**

* Initial commit.
