pub use clap::Parser;
use clap::Subcommand;

/// A todo list manager that allows to create lists, put tasks in them and
/// update them
#[derive(Debug, Parser)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a todo list
    Create {
        /// The name of the list to create
        list: String,
    },
    /// Insets a task in the said list
    Insert {
        /// The name of the todo list in which there will be the task
        list: String,
        /// Name of the task
        task: String,
    },
    /// Marks said task as done
    Done {
        /// The name of the todo list in which there is the task
        list: String,
        /// Name of the task
        task: String,
    },
    /// Deletes either a todo list or a task in the todo list
    Delete {
        /// The name of the todo list to delete or in which there's a task to delete
        list: String,
        /// If indicated, then the said task will be deleted
        task: Option<String>,
    },
    /// Shows the contents of a todo-list
    Show {
        /// The name of the todo-list to show, shows all of them if not specified
        list: Option<String>,
    },
}
