use search_path::SearchPath;
use std::path::PathBuf;

#[cfg(target_family = "windows")]
const SIMPLE_PATH: &str = "a;b;c";
#[cfg(not(target_family = "windows"))]
const SIMPLE_PATH: &str = "a:b:c";

#[test]
fn test_append() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert!(!search_path.contains(&PathBuf::from("d")));
    search_path.append(PathBuf::from("d"));
    assert!(search_path.contains(&PathBuf::from("d")));
    assert_eq!(search_path.iter().last(), Some(&PathBuf::from("d")));
}

#[test]
fn test_append_cwd() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert!(!search_path.contains(&PathBuf::from(".")));
    search_path.append_cwd();
    assert!(search_path.contains(&PathBuf::from(".")));
    assert_eq!(search_path.iter().last(), Some(&PathBuf::from(".")));
}

#[test]
fn test_prepend() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert!(!search_path.contains(&PathBuf::from("d")));
    search_path.prepend(PathBuf::from("d"));
    assert!(search_path.contains(&PathBuf::from("d")));
    assert_eq!(search_path.iter().next(), Some(&PathBuf::from("d")));
}

#[test]
fn test_prepend_cwd() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert!(!search_path.contains(&PathBuf::from(".")));
    search_path.prepend_cwd();
    assert!(search_path.contains(&PathBuf::from(".")));
    assert_eq!(search_path.iter().next(), Some(&PathBuf::from(".")));
}

#[test]
fn test_remove() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert!(search_path.contains(&PathBuf::from("a")));
    search_path.remove(&PathBuf::from("a"));
    assert!(!search_path.contains(&PathBuf::from("a")));
}

#[cfg(target_family = "windows")]
const DUPLICATES_PATH: &str = "a;b;a;b;c;a;c";
#[cfg(not(target_family = "windows"))]
const DUPLICATES_PATH: &str = "a:b:a:b:c:a:c";

#[test]
fn test_dedup() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", DUPLICATES_PATH);
    search_path.dedup();
    assert_eq!(search_path.len(), 3);
    assert!(search_path.contains(&PathBuf::from("a")));
    assert!(search_path.contains(&PathBuf::from("b")));
    assert!(search_path.contains(&PathBuf::from("c")));
}
