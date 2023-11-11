pub use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Create { name: String },
    Insert { name: String, task: String },
    Done { name: String, task: String },
}
