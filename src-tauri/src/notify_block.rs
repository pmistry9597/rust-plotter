use serde::Serialize;
use tauri::window::Window;

pub fn notify_block(index: usize, name: &str, window: &Window) -> Result<(), tauri::Error>
{
    let blck_info = BlckInfo{index, name};
    window.emit(name, blck_info)
}

#[derive(Serialize, Clone)]
struct BlckInfo<'a> {
    index: usize,
    name: &'a str,
}