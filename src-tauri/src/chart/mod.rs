pub mod types;

use std::{future::Future, sync::Arc};
use futures::lock::Mutex;
use tauri::{Window, State, AppHandle, Manager};
use types::RlDataOpChunk;

use crate::notify_block::notify_block;
use self::types::*;

#[tauri::command]
pub fn get_ptprop(i: i32, chartproc_state: State<Arc<Mutex<ChartProc>>>) -> PtProp {
    tauri::async_runtime::block_on(async {
        let i: usize = i.try_into().unwrap();
        chartproc_state.lock().await.pt_props[i]
    })
}

pub struct ChartProc {
    srcs: Vec<RlData>,
    // mesh_props: Vec<CylProp>,
    pt_props: Vec<PtProp>,
    window: Window,
}

impl ChartProc {
    pub fn new(window: Window) -> ChartProc {
        ChartProc{
            srcs: vec![],
            // mesh_props: vec![],
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
        // pt props
        let scale: Vec2 = [3.0, 2.0];
        let new_ptprops_iter = gen_ptprops_iter(data_chunk_iter.clone(), scale);
        // mesh props?

        let mut chartproc = chartproc_state.lock().await;

        // notify frontend
        notify_new_data(new_ptprops_iter.clone(), &chartproc.pt_props, &chartproc.window);

        chartproc.pt_props.extend(new_ptprops_iter);
        chartproc.srcs.extend(data_chunk_iter);
    }
}

fn notify_new_data<I>(new_ptprops_iter: I, pt_props: &Vec<PtProp>, window: &Window)
where
    I: Iterator<Item = PtProp> + Clone,
{
    let count = new_ptprops_iter.clone().count();
    if count > 0 {
        let begin_index = pt_props.len();
        let end_index = begin_index + count;
        (begin_index..end_index).for_each(|i| {
            notify_block(i.try_into().expect("weird af error eh"), "pt_update", window).expect("Failure to notify goddammit");
        })
    }
}

fn gen_ptprops_iter<VecIter>(data_chunk_iter: VecIter, scale: Vec2) -> impl Iterator<Item = PtProp> + Clone
where 
    VecIter: Iterator<Item = RlData> + Clone,
{
    data_chunk_iter.map(move |data| {
        let pos = scale_vec2(data.pos, scale);
        PtProp{pos, rl_data: data}
    })
}

fn scale_vec2(pt: Vec2, scale: Vec2) -> Vec2 {
    let scaled_vec: Vec<f32> = pt.iter().zip(scale.iter()).map(|(elem, scaler)| elem * scaler).collect();
    scaled_vec[0..2].try_into().expect("wow what a stupid failure")
}