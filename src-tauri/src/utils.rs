use std::hash::{Hash, Hasher};
use std::ops::DerefMut;
use tauri::State;

use crate::data::{Event, FEvent};
use crate::data::{List, Tag};
use crate::storage::{Entity, Repository, StorageState, Storage};
use crate::time::{date, time};

pub fn event_to_fevent(event: &Event) -> FEvent {
    FEvent {
        id: event.metadata.uuid.clone(),
        listid: match event.metadata.list {
            None => "Undefined".to_string(),
            Some(listid) => listid.to_string(),
        },
        time: time(event.task_time),
        date: date(event.task_time),
        tag: event.metadata.tag.clone(),
        title: event.title.clone(),
        create: date(event.metadata.timestamp),
        finished: event.finished,
        priority: event.priority.clone(),
        color: event.color.clone(),
        icon: event.icon.clone(),
    }
}

fn exists<T>(state: &State<'_, StorageState>, name: &str) -> bool
where
    T: Entity + 'static,
{
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut hash);
    let hash = hash.finish();
    let mut guard = state.0.lock().unwrap();
    let storage = guard.deref_mut();

    let item: Option<T> = Repository::<T>::get_by_name(storage, &hash.to_string())
        .map_err(|e| e.to_string())
        .unwrap();

    item.is_some()
}

pub fn tag_exists(state: &State<'_, StorageState>, name: &str) -> bool {
    exists::<Tag>(state, name)
}

pub fn list_exists(state: &State<'_, StorageState>, name: &str) -> bool {
    exists::<List>(state, name)
}

#[allow(dead_code)]
// 这一段代码是函数式编程的风格，写在这里装个逼
pub fn with_storage<F, T>(state: &State<'_, StorageState>, f: F) -> Result<T, String>
where
    F: FnOnce(&mut Storage) -> Result<T, String>,
{
    let mut guard = state.0.lock().unwrap();
    let storage = guard.deref_mut();
    f(storage)
}