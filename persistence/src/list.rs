use super::Memory;
use domain::list::{self, Repository};

impl Repository for Memory {
    fn add(&mut self, list: list::List) {
        if let Some(l) = self.lists.iter_mut().find(|l| l.name == list.name) {
            l.entries = list.entries;
        } else {
            self.lists.push(list);
        }
    }

    fn get(&self, name: &String) -> Option<list::List> {
        self.lists.iter().find(|&list| list.name == *name).cloned()
    }
}
