use std::fs;
use sha2::{Digest, Sha256};

pub fn exist_dir(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn to_executable(path: &str) -> String {
    path.replace(".cpp", "")
        .replace(crate::names::SRC_DIR, crate::names::BIN_DIR)
}

pub fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

pub fn load_env() -> crate::env::Env {
    let env = fs::read_to_string(crate::names::ENV_FULL).unwrap();
    serde_json::from_str(&env).unwrap()
}

pub fn save_env(env: &crate::env::Env) {
    let env = serde_json::to_string(env).unwrap();
    fs::write(crate::names::ENV_FULL, env).unwrap();
}
