/*!
Provides a very simple search path file finder.

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
    let search_path = SearchPath::new("PATH").expect("How do you live with no $PATH?");
    search_path.find_file(&PathBuf::from(cmd_name))
}
```

*/

#![warn(
    // ---------- Stylistic
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    // ---------- Unused
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
)]

use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This is the search path itself, it wraps a list of file paths which can then be used to find
/// file system entries. See the [module](index.html) description for an overview.
///
#[derive(Clone, Debug, PartialEq)]
pub struct SearchPath {
    paths: Vec<PathBuf>,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[cfg(target_family = "windows")]
const PATH_SEPARATOR_CHAR: char = ';';

#[cfg(not(target_family = "windows"))]
const PATH_SEPARATOR_CHAR: char = ':';

const CURRENT_DIR_PATH: &str = ".";

// ------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
enum FindKind {
    Any,
    File,
    Directory,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SearchPath {
    fn default() -> Self {
        Self {
            paths: Default::default(),
        }
    }
}

impl Display for SearchPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(&PATH_SEPARATOR_CHAR.to_string())
        )
    }
}

impl From<PathBuf> for SearchPath {
    fn from(v: PathBuf) -> Self {
        Self { paths: vec![v] }
    }
}

impl From<Vec<PathBuf>> for SearchPath {
    fn from(vs: Vec<PathBuf>) -> Self {
        Self { paths: vs }
    }
}

impl From<Vec<&Path>> for SearchPath {
    fn from(vs: Vec<&Path>) -> Self {
        Self {
            paths: vs.iter().map(|p| PathBuf::from(p)).collect(),
        }
    }
}

impl From<Vec<&str>> for SearchPath {
    fn from(vs: Vec<&str>) -> Self {
        Self {
            paths: vs
                .iter()
                .filter_map(|p| {
                    if p.trim().is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(p))
                    }
                })
                .collect(),
        }
    }
}

impl From<String> for SearchPath {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for SearchPath {
    fn from(s: &str) -> Self {
        Self::from(s.split(PATH_SEPARATOR_CHAR).collect::<Vec<&str>>())
    }
}

impl From<SearchPath> for Vec<PathBuf> {
    fn from(p: SearchPath) -> Self {
        p.paths
    }
}

impl IntoIterator for SearchPath {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}

impl SearchPath {
    ///
    /// Construct a new search path by parsing the environment variable named `env_var` into
    /// separate paths. Paths are separated by the `';'` character on Windows, and the `':'`
    /// character on other platforms.
    ///
    /// If the environment variable is not present, or could not be read this function returns
    /// an error.
    ///
    /// ```rust,should_panic
    /// use search_path::SearchPath;
    ///
    /// let search_path = SearchPath::new("CMD_PATH").expect("No $CMD_PATH present");
    /// ```
    ///
    /// Constructors do not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn new(env_var: &str) -> Result<Self, Box<dyn Error>> {
        match env::var(env_var) {
            Ok(path) => Ok(Self::from(path)),
            Err(e) => Err(Box::new(e)),
        }
    }

    ///
    /// Construct a new search path by parsing the environment variable named `PATH`.
    ///
    pub fn path() -> Result<Self, Box<dyn Error>> {
        Self::new("PATH")
    }

    ///
    /// Construct a new search path by parsing the environment variable named `env_var` into
    /// separate paths. Paths are separated by the `';'` character on Windows, and the `':'`
    /// character on other platforms.
    ///
    /// If the environment variable is not present, or could not be read this function returns
    /// the default value provided instead. The default value may be any value that has an
    /// `Into` implementation for `SearchPath`.
    ///
    /// ```rust
    /// use search_path::SearchPath;
    ///
    /// let search_path = SearchPath::new_or("CMD_PATH", ".");
    /// ```
    ///
    /// Constructors do not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn new_or<T: Into<SearchPath>>(env_var: &str, default: T) -> Self {
        match Self::new(env_var) {
            Ok(search_path) => search_path,
            Err(_) => default.into(),
        }
    }

    ///
    /// Construct a new search path by parsing the environment variable named `env_var` into
    /// separate paths. Paths are separated by the `';'` character on Windows, and the `':'`
    /// character on other platforms.
    ///
    /// If the environment variable is not present, or could not be read this function returns
    /// the value of `Default::default()` implemented for `SearchPath` instead.
    ///
    /// ```rust
    /// use search_path::SearchPath;
    ///
    /// let search_path = SearchPath::new_or_default("CMD_PATH");
    /// ```
    ///
    /// Constructors do not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn new_or_default(env_var: &str) -> Self {
        Self::new_or(env_var, SearchPath::default())
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Return the first file system entity, either file or directory, found in the search path, or
    /// `None`.
    ///
    pub fn find(&self, file_name: &Path) -> Option<PathBuf> {
        self.find_something(file_name, FindKind::Any)
    }

    ///
    /// Return all the file system entities, either file or directory, found in the search path.
    ///
    pub fn find_all(&self, file_name: &Path) -> Vec<PathBuf> {
        let mut results: Vec<PathBuf> = Default::default();
        for path in &self.paths {
            let mut path = PathBuf::from(path);
            path.push(file_name);
            if path.exists() {
                results.push(path);
            }
        }
        results
    }

    ///
    /// Return the first _file_ found in the search path, or `None`.
    ///
    pub fn find_file(&self, file_name: &Path) -> Option<PathBuf> {
        self.find_something(file_name, FindKind::File)
    }

    ///
    /// Return the first _directory_ found in the search path, or `None`.
    ///
    pub fn find_directory(&self, file_name: &Path) -> Option<PathBuf> {
        self.find_something(file_name, FindKind::Directory)
    }

    ///
    /// Return the first file found in the search path, or `None`. This method will only
    /// consider `file_name` if it is not a path, if it has any path components the method
    /// will also return `None`.
    ///
    pub fn find_if_name_only(&self, file_name: &Path) -> Option<PathBuf> {
        if let Some(_) = file_name.parent() {
            self.find(file_name)
        } else {
            None
        }
    }

    fn find_something(&self, file_name: &Path, kind: FindKind) -> Option<PathBuf> {
        for path in &self.paths {
            let mut path = PathBuf::from(path);
            path.push(file_name);
            if (kind == FindKind::Any && path.exists())
                || (kind == FindKind::File && path.is_file())
                || (kind == FindKind::Directory && path.is_dir())
            {
                return Some(path);
            }
        }
        None
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Return `true` if this instance has no paths to search, else `false`.
    ///
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    ///
    /// Return the current number of paths in the list of paths to search.
    ///
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    ///
    /// Return `true` if the list of paths to search contains the `path` value, else `false`.
    ///
    pub fn contains(&self, path: &PathBuf) -> bool {
        self.paths.contains(path)
    }

    ///
    /// Return `true` if the list of paths to search contains the current directory path, `"."`,
    /// value, else `false`.
    ///
    pub fn contains_cwd(&self) -> bool {
        self.contains(&PathBuf::from(CURRENT_DIR_PATH))
    }

    ///
    /// Return an iterator over all the paths in the list of paths to search.
    ///
    pub fn iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.paths.iter()
    }

    // --------------------------------------------------------------------------------------------

    ///
    /// Append the provided `path` to the list of paths to search.
    /// This operation does not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn append(&mut self, path: PathBuf) {
        self.paths.push(path)
    }

    ///
    /// Append the current directory path, `"."`, to the list of paths to search.
    /// This operation does not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn append_cwd(&mut self) {
        self.append(PathBuf::from(CURRENT_DIR_PATH))
    }

    ///
    /// Prepend the provided `path` to the list of paths to search.
    /// This operation does not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn prepend(&mut self, path: PathBuf) {
        self.paths.insert(0, path)
    }

    ///
    /// Prepend the current directory path, `"."`, to the list of paths to search.
    /// This operation does not check for duplicate paths, to remove duplicates see the
    /// [`dedup`](struct.SearchPath.html#method.dedup)) method.
    ///
    pub fn prepend_cwd(&mut self) {
        self.prepend(PathBuf::from(CURRENT_DIR_PATH))
    }

    ///
    /// Remove the path from the list of paths to search, this has no effect if the path
    /// was not in the list.
    ///
    pub fn remove(&mut self, path: &PathBuf) {
        self.paths.retain(|p| p != path);
    }

    ///
    /// Ensure that only one copy of a path exists in the list of paths to search. This operation
    /// will ensure the ordering of paths remains the same and keep the first duplicate it finds
    /// and remove any subsequent ones.
    ///
    pub fn dedup(&mut self) {
        use std::collections::HashSet;
        let mut seen: HashSet<PathBuf> = Default::default();
        self.paths.retain(|p| seen.insert(p.clone()))
    }
}
