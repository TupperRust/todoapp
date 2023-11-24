use super::entry::Entry;
use serde::{Deserialize, Serialize};

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

pub trait Repository {
    fn add(&mut self, entry: List);
    fn get(&self, name: &String) -> Option<List>;
    fn delete(&mut self, name: &String) -> Option<List>;
}
