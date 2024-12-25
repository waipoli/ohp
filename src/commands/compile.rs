use crate::names::*;
use crate::utils::{create_hash, load_env, save_env, to_executable};
use colored::Colorize;
use std::fs;
use std::process::Command;
fn compile_file(file: &str) {
    let mut env = load_env();
    let compile;
    if !env.compiled_files.contains_key(file) {
        compile = false;
    } else {
        let file_md5 = env.compiled_files.get(file).unwrap();
        let md5 = create_hash(&fs::read_to_string(file).unwrap());
        compile = file_md5 == &md5;
    }
    if compile {
        println!("Already compiled file: {}", file.blue());
        return;
    }
    let output = Command::new("g++")
        .arg(file)
        .arg("-o")
        .arg(to_executable(file))
        .output()
        .expect("Failed to compile file");
    println!("Compile file: {}", file.green());
    env.compiled_files.insert(
        file.to_string(),
        create_hash(&fs::read_to_string(file).unwrap()),
    );
    save_env(&env);
    if output.stdout.len() > 0 {
        println!("stdout:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        println!("stderr:");
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }
}

pub fn compile() {
    compile_file(SOL_FULL);
    compile_file(CORRECT_FULL);
    compile_file(GEN_FULL);
    compile_file(CHECKER_FULL);
}
