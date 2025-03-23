use clap::{command, Parser, Subcommand};
mod commands;
mod utils;
mod names;
mod recourses;
mod env;

#[derive(Parser)]
#[command(
    name = "OHP",
    version = "0.1.1",
    author = "Maksym Shvedchenko <maximschved8@gmail.com>",
    about = "Does awesome things"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        force: bool,
    },
    Delete {},
    Compile {},
    Run {
        #[arg(short, long)]
        test_count: Option<usize>,
    },
}



fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init { force } => commands::init::init(force),
        Commands::Delete {} => commands::delete::delete(),
        Commands::Compile {} => commands::compile::compile(),
        Commands::Run { test_count } => commands::run::run(test_count),
    }
}
