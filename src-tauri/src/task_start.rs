use crate::single_consumable::{Consume, SingleConsumable};
use tauri::{State, async_runtime};
use std::pin::Pin;
use futures::Future;
use futures::lock::Mutex;
use std::sync::Arc;

pub type Task = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>;
pub type TaskList = Vec<Arc<Mutex<SingleConsumable<Task>>>>;

#[tauri::command]
pub fn ready(tasks: State<TaskList>)
{
    tasks.iter().for_each(|task_lock| {
        let mut task_m = task_lock.try_lock().expect("u failed to lock meh? fuck you");
        async_runtime::spawn(task_m.try_consume().expect("already consumed or waht??"));
    });
}

pub fn get_tasklist(tasks: impl Iterator<Item = Task>) -> TaskList {
    tasks.map(|task| {
        Arc::new(Mutex::new(SingleConsumable::new(task)))
    }).collect()
}