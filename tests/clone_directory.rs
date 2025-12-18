use std::fs;

use rfs_test_macro::rfs_test;

#[rfs_test(
    config = r#"
    - !clone_directory
      source: src
    "#
)]
fn test_clone_directory(dirname: &str) -> std::io::Result<()> {
    assert!(dirname.len() > 0);
    assert!(fs::read_dir(dirname)?.count() > 0);
    Ok(())
}