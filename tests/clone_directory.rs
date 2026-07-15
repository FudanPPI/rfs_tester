use std::fs;

use rfs_test_macro::rfs_test;

#[rfs_test(config = r#"
    - !clone_directory
      source: src
    "#)]
fn test_clone_directory(dirname: &str) -> std::io::Result<()> {
    assert!(!dirname.is_empty());
    assert!(fs::read_dir(dirname)?.count() > 0);
    Ok(())
}
