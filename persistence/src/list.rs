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

    /// Returns the list with the specified name, if it exists
    fn get(&self, name: &String) -> Option<list::List> {
        self.lists.iter().find(|&list| list.name == *name).cloned()
    }

    fn get_list_names(&self) -> Vec<String> {
        self.lists
            .iter()
            .map(|list| list.name.clone())
            .collect()
    }

    /// Removes the list with the specified name (if it exists) and return it.
    fn delete(&mut self, name: &String) -> Option<list::List> {
        let i = self.lists.iter().position(|l| &l.name == name)?;
        Some(self.lists.remove(i))
    }
}
