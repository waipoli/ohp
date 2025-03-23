use colored::Colorize;
use std::fs::File;
use std::process::{exit, Command, Stdio};

use crate::names::*;
use crate::utils::to_executable;

// fn run_program(
//     path: &str,
//     input_file: Option<&str>,
//     output_file: Option<&str>,
//     arguments: Option<&str>,
// ) -> bool {
//     let output = if input_file.is_some() {
//         if arguments.is_none() {
//             std::process::Command::new(to_executable(path))
//                 .stdin(std::fs::File::open(input_file.unwrap()).unwrap())
//                 .output()
//                 .expect(&*"Failed to execute command".red())
//         } else {
//             std::process::Command::new(to_executable(path))
//                 .stdin(std::fs::File::open(input_file.unwrap()).unwrap())
//                 .args(arguments.unwrap().split(" "))
//                 .output()
//                 .expect(&*"Failed to execute command".red())
//         }
//     } else {
//         if arguments.is_none() {
//             std::process::Command::new(to_executable(path))
//                 .output()
//                 .expect(&*"Failed to execute command".red())
//         } else {
//             std::process::Command::new(to_executable(path))
//                 .args(arguments.unwrap().split(" "))
//                 .output()
//                 .expect(&*"Failed to execute command".red())
//         }
//     };
//
//     if output_file.is_some() {
//         std::fs::write(output_file.unwrap(), &output.stdout).unwrap();
//     }
//     let code = match output.status.code() {
//         Some(code) => code,
//         None => {
//             println!("Failed to retrieve exit code");
//             exit(1);
//         }
//     };
//     if code != 0 && code != 255 {
//         println!(
//             "Something went wrong in program {}; Exit code: {}",
//             path, code
//         );
//         exit(0);
//     }
//     code == 0
// }

fn run_program(
    path: &str,
    input_file: Option<&str>,
    output_file: Option<&str>,
    arguments: Option<&str>,
) -> bool {
    let mut command = Command::new(to_executable(path));
    if let Some(input) = input_file {
        let file = File::open(input).unwrap_or_else(|_| {
            eprintln!("Failed to open input file: {}", input.red());
            exit(1);
        });
        command.stdin(Stdio::from(file));
    }
    if let Some(args) = arguments {
        command.args(args.split_whitespace());
    }
    let output = command.output().unwrap_or_else(|_| {
        eprintln!("{}", "Failed to execute command".red());
        exit(1);
    });
    if let Some(output_path) = output_file {
        if let Err(e) = std::fs::write(output_path, &output.stdout) {
            eprintln!(
                "Failed to write to output file: {}: {}",
                output_path.red(),
                e
            );
            exit(1);
        }
    }
    match output.status.code() {
        Some(code) if code == 0 => true,
        Some(255) => false,
        Some(code) => {
            eprintln!(
                "Something went wrong in program {}; Exit code: {}",
                path.red(),
                code
            );
            exit(0);
        }
        None => {
            eprintln!("{}", "Failed to retrieve exit code".red());
            exit(1);
        }
    }
}
fn run_test(id: usize) {
    run_program(
        GEN_FULL,
        None,
        Some(INPUT_FULL),
        Some(&format!("--id {}", id)),
    );
    run_program(SOL_FULL, Some(INPUT_FULL), Some(OUTPUT_SOL_FULL), None);
    run_program(
        CORRECT_FULL,
        Some(INPUT_FULL),
        Some(OUTPUT_CORRECT_FULL),
        None,
    );
    let result = run_program(CHECKER_FULL, Some(INPUT_FULL), None, None);
    if result {
        println!(
            "{}{} {}",
            "#".green(),
            id.to_string().green(),
            "Test passed".green()
        );
    } else {
        println!("{}", "Test failed".red());
        exit(0);
    }
}

pub fn run(test_count: Option<usize>) {
    println!("Run tests");
    if test_count.is_some() {
        for i in 0..test_count.unwrap() {
            run_test(i);
        }
    } else {
        run_test(1);
    }
}
