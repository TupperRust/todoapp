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

pub fn insert_into(
    name: String,
    task: String,
    repository: &mut impl list::Repository,
) -> DomainResult {
    let mut list = repository
        .get(&name)
        .ok_or_else(|| DomainError::ListNotFound(name))?;

    let entry = entry::Entry::new(task);
    list.todo(entry);

    repository.add(list);

    Ok(())
}

pub fn mark_as_done(
    name: String,
    task: String,
    repository: &mut impl list::Repository,
) -> DomainResult {
    let mut list = repository
        .get(&name)
        .ok_or_else(|| DomainError::ListNotFound(name))?;

    if let Some(e) = list.entries.iter_mut().find(|e| e.task == task) {
        e.status = Status::Done
    }

    repository.add(list);

    Ok(())
}

pub fn delete_task(
    name: String,
    task: String,
    repository: &mut impl list::Repository,
) -> DomainResult {
    let mut list = repository
        .get(&name)
        .ok_or_else(|| DomainError::ListNotFound(name))?;

    let task_index = list
        .entries
        .iter()
        .position(|l| l.task == task)
        .ok_or(DomainError::TaskNotFound(task))?;

    list.entries.remove(task_index);

    Ok(())
}

pub fn delete_list(name: String, repository: &mut impl list::Repository) {
    repository.delete(&name);
}

pub fn show_list(name: String, repository: &mut impl list::Repository) -> DomainResult {
    let list = repository
        .get(&name)
        .ok_or_else(|| DomainError::ListNotFound(name))?;
    println!("{list}");

    Ok(())
}

pub fn show_all(repository: &mut impl list::Repository) {
    let list_names = repository.get_list_names();

    if list_names.is_empty() {
        println!("The todo-list is empty.");
    } else {
        for list_name in repository.get_list_names().iter() {
            // Unwrap is safe since we **know** the lists are present in the repo.
            show_list(list_name.clone(), repository).unwrap();
        }
    }
}
