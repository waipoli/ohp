use std::fs;
use crate::utils;



// TODO
pub(crate) fn delete() {
    if !utils::exist_dir("./.ohp") {
        println!("Directory is empty");
        return;
    }
    fs::remove_dir_all("./.ohp").unwrap();
}