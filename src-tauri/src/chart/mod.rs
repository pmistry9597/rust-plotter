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
#[tauri::command]
pub fn get_cylprop(i: i32, chartproc_state: State<Arc<Mutex<ChartProc>>>) -> CylProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        chartproc_state.lock().await.cyl_props[i]
    })
}

pub struct ChartProc {
    srcs: Vec<RlData>,
    cyl_props: Vec<CylProp>,
    pt_props: Vec<PtProp>,
    window: Window,
}

impl ChartProc {
    pub fn new(window: Window) -> ChartProc {
        ChartProc{
            srcs: vec![],
            cyl_props: vec![],
            pt_props: vec![],
            window,
        }
    }
}

pub async fn src_worker<F, const CHUNK_SIZE: usize>(app: AppHandle, get_rl: impl Fn() -> F)
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
        let scale: Vec3 = [3.0, 2.0, 1.0];
        let new_ptprops_iter = gen_ptprops_iter(data_chunk_iter.clone(), scale);

        let mut chartproc = chartproc_state.lock().await;
        let prev_pt_count = chartproc.pt_props.len();

        chartproc.srcs.extend(data_chunk_iter);
        chartproc.pt_props.extend(new_ptprops_iter.clone());

        let pt_props = &chartproc.pt_props;
        let pts_for_cyl: Vec<PtProp> = get_pts_for_cyl(pt_props, prev_pt_count); // get slice for cyl processing
        let new_cylprops_iter = gen_cylprops_iter(pts_for_cyl.into_iter(), [1.0, 1.0, 1.0]);

        chartproc.cyl_props.extend(new_cylprops_iter.clone());
        
        // notify concerned parties?
        notify_new_chartproc(new_ptprops_iter, new_cylprops_iter, &chartproc);
    }
}

fn notify_new_chartproc<PtIter, CylIter>(
    new_ptprops_iter: PtIter, new_cylprops_iter: CylIter, chartproc: &ChartProc)
where
    PtIter: Iterator<Item = PtProp> + Clone,
    CylIter: Iterator<Item = CylProp> + Clone,
{
    notify_new_data("pt_update", 
        new_index_iter(new_ptprops_iter.count(), chartproc.pt_props.len()), 
        &chartproc.window);
    notify_new_data("cyl_update", 
        new_index_iter(new_cylprops_iter.count(), chartproc.cyl_props.len()), 
        &chartproc.window);
}
fn get_pts_for_cyl(pt_props: &Vec<PtProp>, prev_pt_count: usize) -> Vec<PtProp> {
    if prev_pt_count == 0 {
        return vec![];
    }
    pt_props[prev_pt_count - 1..].iter().map(|pt_ref| pt_ref.clone()).collect()
}
fn new_index_iter(added_count: usize, count: usize) -> impl Iterator<Item = usize> + Clone {
    (count - added_count)..count
}
fn notify_new_data<I>(name: &str, new_index_iter: I, window: &Window)
where
    I: Iterator<Item = usize> + Clone,
{
    new_index_iter.for_each(|i| {
        notify_block(i, name, window).expect("Failure to notify goddammit");
    });
}