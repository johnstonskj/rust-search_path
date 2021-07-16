use search_path::SearchPath;
use std::path::PathBuf;

#[test]
fn test_append() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "a:b:c");
    assert!(!search_path.contains(&PathBuf::from("d")));
    search_path.append(PathBuf::from("d"));
    assert!(search_path.contains(&PathBuf::from("d")));
    assert_eq!(search_path.iter().last(), Some(&PathBuf::from("d")));
}

#[test]
fn test_append_cwd() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "a:b:c");
    assert!(!search_path.contains(&PathBuf::from(".")));
    search_path.append_cwd();
    assert!(search_path.contains(&PathBuf::from(".")));
    assert_eq!(search_path.iter().last(), Some(&PathBuf::from(".")));
}

#[test]
fn test_prepend() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "a:b:c");
    assert!(!search_path.contains(&PathBuf::from("d")));
    search_path.prepend(PathBuf::from("d"));
    assert!(search_path.contains(&PathBuf::from("d")));
    assert_eq!(search_path.iter().next(), Some(&PathBuf::from("d")));
}

#[test]
fn test_prepend_cwd() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "a:b:c");
    assert!(!search_path.contains(&PathBuf::from(".")));
    search_path.prepend_cwd();
    assert!(search_path.contains(&PathBuf::from(".")));
    assert_eq!(search_path.iter().next(), Some(&PathBuf::from(".")));
}

#[test]
fn test_remove() {
    let mut search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "a:b:c");
    assert!(search_path.contains(&PathBuf::from("a")));
    search_path.remove(&PathBuf::from("a"));
    assert!(!search_path.contains(&PathBuf::from("a")));
}