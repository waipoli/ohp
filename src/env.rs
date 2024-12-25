use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Env {
    pub(crate) compiled_files: HashMap<String, String>,
}
