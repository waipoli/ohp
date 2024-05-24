use crate::names::*;
use std::process::Command;
use crate::utils::to_executable;

fn compile_file(file: &str) {
    let output = Command::new("g++")
        .arg(file)
        .arg("-o")
        .arg(to_executable(file))
        .output()
        .expect("Failed to compile file");
    println!("Compile file: {}", file);
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}

pub(crate) fn compile() {
    compile_file(SOL_FULL);
    compile_file(CORRECT_FULL);
    compile_file(GEN_FULL);
    compile_file(CHECKER_FULL);
}