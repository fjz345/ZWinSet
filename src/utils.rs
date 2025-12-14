use std::{fs, path::Path};

pub fn path_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}
