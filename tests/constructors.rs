use search_path::SearchPath;
use std::path::{Path, PathBuf};

#[cfg(target_family = "windows")]
const SIMPLE_PATH: &str = ".;..";
#[cfg(not(target_family = "windows"))]
const SIMPLE_PATH: &str = ".:..";

#[cfg(target_family = "windows")]
const NOT_SO_SIMPLE_PATH: &str = ";.;..;";
#[cfg(not(target_family = "windows"))]
const NOT_SO_SIMPLE_PATH: &str = ":.:..:";

fn assert_correct(search_path: &SearchPath) {
    assert_eq!(search_path.len(), 2);
    assert!(search_path.contains(&PathBuf::from(".")));
    assert!(search_path.contains(&PathBuf::from("..")));
    assert!(search_path.contains_cwd());
    assert!(!search_path.contains(&PathBuf::from("tests")));
}

// ------------------------------------------------------------------------------------------------

#[test]
fn test_no_env_var() {
    let search_path = SearchPath::new("UNLIKELY_THIS_VAR_EXISTS");
    assert!(search_path.is_err());
}

#[test]
fn test_no_env_var_or() {
    let search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", SIMPLE_PATH);
    assert_correct(&search_path);
}

#[test]
fn test_no_env_var_or_default() {
    let search_path = SearchPath::new_or_default("UNLIKELY_THIS_VAR_EXISTS");
    assert_eq!(search_path.len(), 0);
}

#[test]
fn test_ignore_empty() {
    let search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", "");
    assert_eq!(search_path.len(), 0);
}

#[test]
fn test_ignore_empty_split() {
    let search_path = SearchPath::new_or("UNLIKELY_THIS_VAR_EXISTS", NOT_SO_SIMPLE_PATH);
    assert_correct(&search_path);
}

// ------------------------------------------------------------------------------------------------

#[test]
fn from_string() {
    let search_path: SearchPath = String::from(SIMPLE_PATH).into();
    assert_correct(&search_path);
}

#[test]
fn from_str() {
    let search_path: SearchPath = SIMPLE_PATH.into();
    assert_correct(&search_path);
}

#[test]
fn from_str_vec() {
    let search_path: SearchPath = vec![".", ".."].into();
    assert_correct(&search_path);
}

#[test]
fn from_pathbuf_vec() {
    let search_path: SearchPath = vec![PathBuf::from("."), PathBuf::from("..")].into();
    assert_correct(&search_path);
}

#[test]
fn from_path_vec() {
    let search_path: SearchPath = vec![Path::new("."), Path::new("..")].into();
    assert_correct(&search_path);
}

#[test]
fn from_pathbuf() {
    let search_path: SearchPath = PathBuf::from(".").into();
    assert_eq!(search_path.len(), 1);
    assert!(search_path.contains(&PathBuf::from(".")));
}
