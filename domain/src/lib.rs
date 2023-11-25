pub mod entry;
pub mod list;
pub mod status;

use list::List;
use status::Status;

#[derive(Debug, Clone, thiserror::Error)]
pub enum DomainError {
    #[error("The list '{0}' was not found.")]
    ListNotFound(String),
    #[error("The task '{0}' was not found.")]
    TaskNotFound(String),
}

pub type DomainResult = Result<(), DomainError>;

pub fn create_list(name: String, repository: &mut impl list::Repository) {
    if repository.get(&name).is_none() {
        let list = List::new(name);
        repository.add(list);
    }
}

pub fn insert_into(name: String, task: String, repository: &mut impl list::Repository) -> DomainResult {
    let Some(mut list) = repository.get(&name) else {
        return Err(DomainError::ListNotFound(name))
    };
    let entry = entry::Entry::new(task);
    list.todo(entry);
    repository.add(list);
    Ok(())
}

pub fn mark_as_done(name: String, task: String, repository: &mut impl list::Repository) -> DomainResult {
    let Some(mut list) = repository.get(&name) else {
        return Err(DomainError::ListNotFound(name))
    };
    if let Some(e) = list.entries.iter_mut().find(|e| e.task == task) {
        e.status = Status::Done
    }
    repository.add(list);
    Ok(())
}

pub fn delete_task(name: String, task: String, repository: &mut impl list::Repository) -> DomainResult {
    let Some(mut list) = repository.get(&name) else {
        return Err(DomainError::ListNotFound(name))
    };
    let task_index =
        list.entries
            .iter()
            .position(|l| l.task == task)
            .ok_or(DomainError::TaskNotFound(task))?;
    list.entries.remove(task_index);
    Ok(())
}

pub fn delete_list(name: String, repository: &mut impl list::Repository) {
    repository.delete(&name);
}
