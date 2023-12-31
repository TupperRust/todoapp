use super::entry::Entry;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct List {
    pub name: String,
    pub entries: Entries,
}

type Entries = Vec<Entry>;

impl List {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entries: Entries::default(),
        }
    }

    pub fn todo(&mut self, entry: Entry) {
        if !self.entries.iter().any(|e| e.task == entry.task) {
            self.entries.push(entry);
        }
    }
}

impl Display for List {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(fmt, "# {} :", self.name)?;
        self.entries
            .iter()
            .try_for_each(|e| writeln!(fmt, "    - {} : {:?}", e.task, e.status))
    }
}

pub trait Repository {
    fn add(&mut self, entry: List);
    fn get(&self, name: &str) -> Option<List>;
    fn get_list_names(&self) -> Vec<String>;
    fn delete(&mut self, name: &str) -> Option<List>;
}
