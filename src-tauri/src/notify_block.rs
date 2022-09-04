use serde::Serialize;
use tauri::window::Window;

pub trait NamedBlckRepo {
    fn get_name(&self) -> &'static str;
}

pub fn notify_block<BlockRepo>(index: i32, named_blckrepo: &BlockRepo, window: &Window) -> Result<(), tauri::Error>
where
    BlockRepo: NamedBlckRepo
{
    let name = named_blckrepo.get_name();
    let blck_info = BlckInfo{index, name};
    window.emit(name, blck_info)
}

#[derive(Serialize, Clone)]
struct BlckInfo<'a> {
    index: i32,
    name: &'a str,
}