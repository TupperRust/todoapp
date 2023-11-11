mod options;

use std::path::Path;

use domain::{create_list, insert_into, mark_as_done};
use options::{Command, Options, Parser};
use persistence::Memory;

static MEMORY: &'static str = ".memory";

fn main() {
    let options = Options::parse();

    let mut memory = Memory::load(&Path::new(MEMORY));

    match options.command {
        Command::Create { name } => {
            create_list(name, &mut memory);
        }
        Command::Insert { name, task } => {
            insert_into(name, task, &mut memory);
        }
        Command::Done { name, task } => {
            mark_as_done(name, task, &mut memory);
        }
    }

    memory.save(&Path::new(MEMORY));
}
