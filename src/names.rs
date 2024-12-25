use const_format::formatcp;

pub const DIR_NAME: &str = ".ohp";
pub const SRC_DIR: &str = "src";
pub const BIN_DIR: &str = "bin";
pub const PIPE_DIR: &str = "pipe";
pub const SOL: &str = "sol.cpp";
pub const CORRECT: &str = "correct.cpp";
pub const GEN: &str = "gen.cpp";
pub const TESTLIB: &str = "testlib.h";
pub const CHECKER: &str = "checker.cpp";
pub const CHECKER_UTILS: &str = "checker_utils.h";
pub const INPUT: &str = "input.in";
pub const OUTPUT_CORRECT: &str = "output-correct.out";
pub const OUTPUT_SOL: &str = "output-sol.out";
pub const ENV: &str = "env.json";

// build
pub const DIR_NAME_FULL: &str = formatcp!("{DIR_NAME}");
pub const SRC_DIR_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}");
pub const BIN_DIR_FULL: &str = formatcp!("{DIR_NAME}/{BIN_DIR}");
pub const PIPE_DIR_FULL: &str = formatcp!("{DIR_NAME}/{PIPE_DIR}");
pub const SOL_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{SOL}");
pub const CORRECT_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{CORRECT}");
pub const GEN_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{GEN}");
pub const TESTLIB_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{TESTLIB}");
pub const CHECKER_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{CHECKER}");
pub const CHECKER_UTILS_FULL: &str = formatcp!("{DIR_NAME}/{SRC_DIR}/{CHECKER_UTILS}");
pub const INPUT_FULL: &str = formatcp!("{DIR_NAME}/{PIPE_DIR}/{INPUT}");
pub const OUTPUT_CORRECT_FULL: &str = formatcp!("{DIR_NAME}/{PIPE_DIR}/{OUTPUT_CORRECT}");
pub const OUTPUT_SOL_FULL: &str = formatcp!("{DIR_NAME}/{PIPE_DIR}/{OUTPUT_SOL}");
pub const ENV_FULL: &str = formatcp!("{DIR_NAME}/{ENV}");
