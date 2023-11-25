pub mod list;
use domain::list::List;
use std::{io, fs::File, path::Path};

pub struct Memory {
    lists: Lists,
}

type Lists = Vec<List>;

impl Memory {
    pub fn load(path: &Path) -> io::Result<Self> {
        let file = File::options()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;
        let lists = serde_json::from_reader(&file).unwrap_or_default();
        Ok(Memory { lists })
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let file = File::options().write(true).open(path)?;
        serde_json::to_writer(file, &self.lists)?;
        Ok(())
    }

    pub fn iter(&self) -> impl std::iter::Iterator<Item=&List> {
        self.lists.iter()
    }
}
