pub mod entry;
pub mod list;
pub mod status;

use list::List;
use status::Status;

pub fn create_list(name: String, repository: &mut impl list::Repository) {
    if repository.get(&name).is_none() {
        let list = List::new(name);
        repository.add(list);
    }
}

pub fn insert_into(name: String, task: String, repository: &mut impl list::Repository) {
    let mut list = repository.get(&name).unwrap();
    let entry = entry::Entry::new(task);
    list.todo(entry);
    repository.add(list);
}

pub fn mark_as_done(name: String, task: String, repository: &mut impl list::Repository) {
    let mut list = repository.get(&name).unwrap();
    if let Some(e) = list.entries.iter_mut().find(|e| e.task == task) {
        e.status = Status::Done
    }
    repository.add(list);
}

pub fn delete_task(name: String, task: String, repository: &mut impl list::Repository) {
    let mut list = repository.get(&name).unwrap();
    let i = list.entries.iter().position(|l| l.task == task).unwrap();
    list.entries.remove(i);
}

pub fn delete_list(name: String, repository: &mut impl list::Repository) {
    repository.delete(&name);
}
