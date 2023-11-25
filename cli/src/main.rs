mod options;

use std::path::Path;

use domain::{create_list, delete_list, delete_task, insert_into, mark_as_done, show_list, show_all};
use options::{Command, Options, Parser};
use persistence::Memory;

static MEMORY: &str = ".memory";

fn main() -> Result<(), main_error::MainError> {
    let options = Options::parse();

    let mut memory = Memory::load(Path::new(MEMORY))?;

    match options.command {
        Command::Create { list } => {
            create_list(list.clone(), &mut memory);
            println!("List '{list}' created.");
        }
        Command::Insert { list, task } => {
            insert_into(list.clone(), task.clone(), &mut memory)?;
            println!("Task '{task}' inserted in list '{list}'.");
        }
        Command::Done { list, task } => {
            mark_as_done(list.clone(), task.clone(), &mut memory)?;
            println!("Task '{task}' in list '{list}' is marked as done!.");
        }
        Command::Delete { list, task } => match task {
            Some(t) => {
                delete_task(list.clone(), t.clone(), &mut memory)?;
                println!("Deleted task '{t}' in list '{list}'.");
            }
            None => {
                delete_list(list.clone(), &mut memory);
                println!("Deleted list '{list}'.");
            }
        },
        Command::Show { list } => match list {
            Some(l) => {
                show_list(l.clone(), &mut memory)?;
            },
            None => {
                show_all(&mut memory);
            },
        },
    }

    memory.save(Path::new(MEMORY))?;

    Ok(())
}
