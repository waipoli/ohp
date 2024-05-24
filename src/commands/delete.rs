use std::fs;
use crate::names::DIR_NAME_FULL;
use crate::utils;



pub(crate) fn delete() {
    if !utils::exist_dir(DIR_NAME_FULL) {
        println!("Directory is empty");
        return;
    }
    fs::remove_dir_all(DIR_NAME_FULL).unwrap();
}