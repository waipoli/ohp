use crate::names::*;
use std::process::Command;
use crate::utils::to_executable;
use colored::Colorize;
fn compile_file(file: &str) {
    let output = Command::new("g++")
        .arg(file)
        .arg("-o")
        .arg(to_executable(file))
        .output()
        .expect("Failed to compile file");
    println!("Compile file: {}", file.green());
    if output.stdout.len() > 0 {
        println!("stdout:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("stderr:");
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

pub(crate) fn compile() {
    compile_file(SOL_FULL);
    compile_file(CORRECT_FULL);
    compile_file(GEN_FULL);
    compile_file(CHECKER_FULL);
}