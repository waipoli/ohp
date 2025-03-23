use std::fs;
use colored::Colorize;
use crate::names::DIR_NAME_FULL;
use crate::utils;



pub fn delete() {
    if !utils::exist_dir(DIR_NAME_FULL) {
        println!("{}", "Directory is empty".red());
        return;
    }
    fs::remove_dir_all(DIR_NAME_FULL).unwrap();
}