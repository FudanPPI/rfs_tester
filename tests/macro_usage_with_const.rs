use std::path::PathBuf;

use rfs_test_macro::rfs_test;

const CONFIG: &str = r#"
    - !directory
        name: test
        content:
          - !file
              name: test.txt
              content:
                !inline_text "Hello, world!"
    "#;

const CONFIG_WO_DIRNAME: &str = r#"
    - !directory
        content:
          - !file
              name: test.txt
              content:
                !inline_text "Hello, world!"
    "#;

#[rfs_test(
    config = CONFIG,
)]
fn file_creation_test_macro_with_conf_in_const(dirname: &str) -> std::io::Result<()> {
    let file_path = PathBuf::from(dirname).join("test.txt");
    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, "Hello, world!");
    Ok(())
}

#[rfs_test(
    config = CONFIG_WO_DIRNAME,
)]
fn file_creation_test_macro_with_second_conf_in_const(dirname: &str) -> std::io::Result<()> {
    let file_path = PathBuf::from(dirname).join("test.txt");
    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, "Hello, world!");
    Ok(())
}
