pub mod types;
mod data_proc;

use std::{future::Future, sync::Arc};
use futures::lock::Mutex;
use tauri::{Window, State, AppHandle, Manager};
use types::RlDataOpChunk;

use crate::notify_block::notify_block;
use types::*;
use data_proc::*;

#[tauri::command]
pub fn get_ptprop(i: i32, chartproc_state: State<Arc<Mutex<ChartProc>>>) -> PtProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        chartproc_state.lock().await.pt_props[i]
    })
}

pub struct ChartProc {
    srcs: Vec<RlData>,
    mesh_props: Vec<CylProp>,
    pt_props: Vec<PtProp>,
    window: Window,
}

impl ChartProc {
    pub fn new(window: Window) -> ChartProc {
        ChartProc{
            srcs: vec![],
            mesh_props: vec![],
            pt_props: vec![],
            window,
        }
    }
}

pub async fn src_chunk_worker<F, const CHUNK_SIZE: usize>(app: AppHandle, get_rl: impl Fn() -> F)
where
    F: Future<Output = Option<RlDataOpChunk<CHUNK_SIZE>>>
{
    loop {
        let chartproc_state = app.state::<Arc<Mutex<ChartProc>>>();
        let data_chunk_op = get_rl().await;
        if let None = data_chunk_op {
            break;
        }
        let data_chunk = data_chunk_op.expect("you're in a bit of a pickle here");
        let data_chunk_iter = data_chunk.iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap());

        // process section
        let scale: Vec2 = [3.0, 2.0];
        let new_ptprops_iter = gen_ptprops_iter(data_chunk_iter.clone(), scale);
        let new_meshprops_iter = gen_meshprops_iter(new_ptprops_iter.clone(), [5.0,5.0]);

        let mut chartproc = chartproc_state.lock().await;

        chartproc.srcs.extend(data_chunk_iter);
        chartproc.pt_props.extend(new_ptprops_iter.clone());
        // chartproc.mesh_props.extend(new_meshprops_iter.clone());

        // notify frontend
        let pt_count = chartproc.pt_props.len();
        notify_new_data("pt_update", (pt_count-new_ptprops_iter.count())..pt_count
            , &chartproc.window);
        // notify_new_data("mesh_update", new_meshprops_iter, &chartproc.mesh_props, &chartproc.window);
    }
}

fn notify_new_data<I>(name: &str, new_index_iter: I, window: &Window)
where
    I: Iterator<Item = usize> + Clone,
{
    new_index_iter.for_each(|i| {
        notify_block(i, name, window).expect("Failure to notify goddammit");
    });
}