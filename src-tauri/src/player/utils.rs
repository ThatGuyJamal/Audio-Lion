use std::path::Path;

/// Returns the file name without its extension
///
/// # Example
/// ```
/// let filename = "example-file.txt";
/// assert_eq!(from_path_to_name_without_ext(Path::new(filename)), "example-file");
/// ```
pub fn from_path_to_name_without_ext(path: &Path) -> String {
    let split: Vec<String> = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('.')
        .map(String::from)
        .collect();
    split[..split.len() - 1].to_vec().join(".")
}