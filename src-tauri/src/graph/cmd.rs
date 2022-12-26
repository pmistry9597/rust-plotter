use std::sync::Arc;

use futures::lock::Mutex;
use tauri::State;

use crate::data_transform::{Retrieve, mutate_info::Accessor};
use super::{types::{PtProp, CylProp}, graph_1d::GraphData1d};

#[tauri::command]
pub fn get_ptprop_1d(i: i32, graph_state: State<Arc<Mutex<GraphData1d>>>) -> PtProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        let access = Accessor::Indices(vec![i]);
        graph_state.lock().await.pts.lock().await.get(&access.clone()).first()
            .expect("bro your index aint there, u better nize it")
            .clone()
    })
}
#[tauri::command]
pub fn get_cylprop_1d(i: i32, graph_state: State<Arc<Mutex<GraphData1d>>>) -> CylProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        let access = Accessor::Indices(vec![i]);
        graph_state.lock().await.cyls.lock().await.get(&access.clone()).first()
            .expect("bro your index aint there, u better nize it")
            .clone()
    })
}