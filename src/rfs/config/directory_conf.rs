use serde::{Deserialize, Serialize};

use super::config_entry::ConfigEntry;

/// Structure for directory record in configuration
/// for example:
///
/// ## yaml:
///
/// ```yaml
/// ---
///   - !directory
///       name: test
///       content:
///         - !file
///             name: test.txt
///             content: empty
///         - !link
///             name: test_link
///             target: test.txt
/// ```
///
/// ## json:
///
/// ```json
/// {
///     [
///         "directory": {
///             "name": "test_dir",
///             "content": [
///                 "file": {
///                     "name": "test.txt",
///                     "content": "empty"
///                 },
///                 "link": {
///                     "name": "test_link",
///                     "target": "test.txt"
///                 }
///             ]
///         }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct DirectoryConf {
    /// A directory will be created with the given name. Root directory-container of sandbox has optional name.
    /// If name isn't specified than it has temporary generated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The directory content can contain a list of various entries.
    pub content: Vec<ConfigEntry>,
}
