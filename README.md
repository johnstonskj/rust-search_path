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

The `SearchPath` type allows for the finding of files using a series of search directories. This is
akin to the mechanism a shell uses to find executables using the `PATH` environment variable. It
can be constructed with a search path from an environment variable, from a string, or from a list
of either string or `Path`/`PathBuf` values. Typically the _find_ methods return the first
matching file or directory, but the `find_all` method specifically collects and returns a list of
all matching paths.

# Constructors

The `SearchPath` type also has `From<>` implementations for `PathBuf`, `Vec<PathBuf>`, `Vec<&Path>`,
`Vec<&str>`, `String`, and `&str`. In the case of vector values, or a single `PathBuf`, each path
value will be used as-is without trying to split it into components. In the case of individual
`String` and `&str` values the value will be split using the platform specific path separator
into individual paths components.

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

**Version 0.1.1**

* Added a dedup method.

**Version 0.1.1**

* Completed documentation.
* Added Github builds.
* Fixed bug in test cases for Windows builds.

**Version 0.1.0**

* Initial commit.
