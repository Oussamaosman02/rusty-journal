use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Add a task -> rusty-journal add "Task"
    Add {
        #[structopt()]
        text: String,
    },
    /// Delete a task with id -> rusty-journal done 2
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all pending tasks -> rusty-journal list
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
