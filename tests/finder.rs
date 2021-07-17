use search_path::SearchPath;
use std::path::PathBuf;

#[cfg(target_family = "windows")]
fn make_full_search_path() -> SearchPath {
    String::from("tests;tests/a;tests/b;tests/c;tests/e/f;tests/e/f/g").into()
}
#[cfg(not(target_family = "windows"))]
fn make_full_search_path() -> SearchPath {
    String::from("tests:tests/a:tests/b:tests/c:tests/e/f:tests/e/f/g").into()
}

#[cfg(target_family = "windows")]
fn make_partial_search_path() -> SearchPath {
    String::from("tests/a;tests/c;tests/e/f/g").into()
}
#[cfg(not(target_family = "windows"))]
fn make_partial_search_path() -> SearchPath {
    String::from("tests/a:tests/c:tests/e/f/g").into()
}

#[test]
fn find_file_full() {
    let search_path = make_full_search_path();
    let result = search_path.find(&PathBuf::from("a.txt"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/a.txt"))
}

#[test]
fn find_directory_full() {
    let search_path = make_full_search_path();
    let result = search_path.find(&PathBuf::from("d"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/b/d"))
}

#[test]
fn find_file_partial() {
    let search_path = make_partial_search_path();
    let result = search_path.find(&PathBuf::from("a.txt"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/e/f/g/a.txt"))
}

#[test]
fn find_no_file_full() {
    let search_path = make_full_search_path();
    let result = search_path.find(&PathBuf::from("none.txt"));
    assert!(result.is_none());
}

#[test]
fn find_all_files_full() {
    let search_path = make_full_search_path();
    let files = search_path.find_all(&PathBuf::from("a.txt"));
    assert_eq!(files.len(), 3);
    assert_eq!(
        files,
        vec![
            PathBuf::from("tests/a.txt"),
            PathBuf::from("tests/b/a.txt"),
            PathBuf::from("tests/e/f/g/a.txt")
        ]
    )
}

#[test]
fn find_file_only_full() {
    let search_path = make_full_search_path();
    let result = search_path.find_file(&PathBuf::from("a.txt"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/a.txt"))
}

#[test]
fn find_no_file_only_full() {
    let search_path = make_full_search_path();
    let result = search_path.find_file(&PathBuf::from("d"));
    assert!(result.is_none());
}

#[test]
fn find_directory_only_full() {
    let search_path = make_full_search_path();
    let result = search_path.find_directory(&PathBuf::from("d"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/b/d"))
}

#[test]
fn find_no_directory_only_full() {
    let search_path = make_full_search_path();
    let result = search_path.find_directory(&PathBuf::from("a.txt"));
    assert!(result.is_none());
}

#[test]
fn find_path_file_full() {
    let search_path = make_full_search_path();
    let result = search_path.find(&PathBuf::from("g/a.txt"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/e/f/g/a.txt"))
}

#[test]
fn find_filename_partial() {
    let search_path = make_partial_search_path();
    let result = search_path.find_if_name_only(&PathBuf::from("a.txt"));
    assert!(result.is_some());
    let file = result.unwrap();
    assert_eq!(file, PathBuf::from("tests/e/f/g/a.txt"))
}

#[test]
fn find_filename_with_directory_partial() {
    let search_path = make_partial_search_path();
    let result = search_path.find_if_name_only(&PathBuf::from("g/a.txt"));
    assert!(result.is_none());
}
