use crate::env::Env;
use crate::names::*;
use crate::recourses::{CHECKER_SOURCE, CHECKER_UTILS_SOURCE, GEN_SOURCE, TESTLIB_SOURCE};
use crate::utils;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;

fn gen_checker_utils() -> String {
    CHECKER_UTILS_SOURCE
        .replace("{INPUT_FILE}", INPUT_FULL)
        .replace("{OUTPUT_CORRECT_FILE}", OUTPUT_CORRECT_FULL)
        .replace("{OUTPUT_SOL_FILE}", OUTPUT_SOL_FULL)
}

pub fn init(force: bool) {
    if utils::exist_dir(DIR_NAME_FULL) {
        if !force {
            println!(
                "{}",
                "Directory is not empty. Use --force to overwrite".red()
            );
            return;
        } else {
            fs::remove_dir_all(DIR_NAME_FULL).unwrap();
        }
    }
    fs::create_dir_all(DIR_NAME_FULL).unwrap();
    fs::create_dir_all(SRC_DIR_FULL).unwrap();
    fs::create_dir_all(BIN_DIR_FULL).unwrap();
    fs::create_dir_all(PIPE_DIR_FULL).unwrap();
    fs::write(TESTLIB_FULL, TESTLIB_SOURCE).unwrap();
    fs::write(CHECKER_UTILS_FULL, gen_checker_utils()).unwrap();
    fs::write(SOL_FULL, "").unwrap();
    fs::write(CORRECT_FULL, "").unwrap();
    fs::write(GEN_FULL, GEN_SOURCE).unwrap();
    fs::write(CHECKER_FULL, CHECKER_SOURCE).unwrap();
    fs::write(INPUT_FULL, "").unwrap();
    fs::write(OUTPUT_CORRECT_FULL, "").unwrap();
    fs::write(OUTPUT_SOL_FULL, "").unwrap();

    fs::write(
        ENV_FULL,
        serde_json::to_string(&Env {
            compiled_files: HashMap::new(),
        })
        .unwrap(),
    )
    .unwrap();
}
