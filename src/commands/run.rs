use std::process::exit;
use colored::Colorize;

use crate::names::*;
use crate::utils::to_executable;

fn run_program(path: &str, input_file: Option<&str>, output_file: Option<&str>) -> bool {
    let output = if input_file.is_some() {
        std::process::Command::new(to_executable(path))
            .stdin(std::fs::File::open(input_file.unwrap()).unwrap())
            .output()
            .expect(&*"Failed to execute command".red())

    } else {
        std::process::Command::new(to_executable(path))
            .output()
            .expect(&*"Failed to execute command".red())
    };
    if output_file.is_some() && output_file.is_some() {
        std::fs::write(output_file.unwrap(), &output.stdout).unwrap();
    }
    let code = output.status.code().unwrap();
    if code != 0 && code != 255 {
        println!("Something went wrong in program {}; Exit code: {}", path, code);
        exit(0)
    }
    return code == 0;
}


fn run_test() {
    run_program(GEN_FULL, None, Some(INPUT_FULL));
    run_program(SOL_FULL, Some(INPUT_FULL), Some(OUTPUT_SOL_FULL));
    run_program(CORRECT_FULL, Some(INPUT_FULL), Some(OUTPUT_CORRECT_FULL));
    let result = run_program(CHECKER_FULL, Some(INPUT_FULL), None);
    if result {
        println!("{}", "Test passed".green());
    } else {
        println!("{}", "Test failed".red());
    }
}

pub fn run(test_count: Option<usize>) {
    println!("Run tests");
    if test_count.is_some() {
        for _ in 0..test_count.unwrap() {
            run_test();
        }
    } else {
        run_test();
    }
}