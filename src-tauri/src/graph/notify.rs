use tauri::Window;

use crate::notify_block::notify_block;

pub fn notify_new_data<I>(name: &str, new_index_iter: I, window: &Window)
where
    I: Iterator<Item = usize>,
{
    new_index_iter.for_each(|i| {
        notify_block(i, name, window).expect("Failure to notify goddammit");
    });
}