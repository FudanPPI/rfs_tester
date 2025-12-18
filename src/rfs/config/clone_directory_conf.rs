use serde::{Deserialize, Serialize};

/// Structure for directory record in configuration
/// for example:
///
/// ## yaml:
///
/// ```yaml
/// ---
///   - !clone_directory
///       name: test_dir
///       source: data_dir
/// ```
///
/// ## json:
///
/// ```json
/// {
///     [
///         "clone_directory": {
///             "name": "test_dir",
///             "source": "data_dir"
///         }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct CloneDirectoryConf {
    /// A directory will be created with the given name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The name of the destination directory for the copy.
    pub source: String,
}
