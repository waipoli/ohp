use std::fs;

pub fn exist_dir(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn to_executable(path: &str) ->String {
    path.replace(".cpp", "").replace(crate::names::SRC_DIR, crate::names::BIN_DIR)
}