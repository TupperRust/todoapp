pub mod list;
use domain::list::List;
use std::{fs::File, path::Path};

pub struct Memory {
    lists: Lists,
}

type Lists = Vec<List>;

impl Memory {
    pub fn load(path: &Path) -> Self {
        let file = File::options()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .unwrap();
        let lists = serde_json::from_reader(&file).unwrap_or(Lists::default());
        Memory { lists }
    }

    pub fn save(&self, path: &Path) {
        let file = File::options().write(true).open(path).unwrap();
        serde_json::to_writer(file, &self.lists).unwrap();
    }
}
