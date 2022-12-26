use std::sync::Arc;

use futures::lock::Mutex;
use tauri::State;

use crate::data_transform::{Retrieve, mutate_info::Accessor};
use super::{GraphData, types::{PtProp, CylProp}};

#[tauri::command]
pub fn get_ptprop(i: i32, graph_state: State<Arc<Mutex<GraphData>>>) -> PtProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        let access = Accessor::Indices(vec![i]);
        graph_state.lock().await.pts.lock().await.get(&access.clone()).first()
            .expect("bro your index aint there, u better nize it")
            .clone()
    })
}
#[tauri::command]
pub fn get_cylprop(i: i32, graph_state: State<Arc<Mutex<GraphData>>>) -> CylProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        let access = Accessor::Indices(vec![i]);
        graph_state.lock().await.cyls.lock().await.get(&access.clone()).first()
            .expect("bro your index aint there, u better nize it")
            .clone()
    })
}