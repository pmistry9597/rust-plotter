use std::sync::Arc;
use futures::{lock::Mutex, Future};
use tauri::{AppHandle, Manager, Window};
use crate::data_transform::{Identity, VecTransform, Transform, NotifyHook};
use super::{types::{RlDataOpChunk, RlData, PtProp, CylProp, Vec3}, pts::{PtMutate, PtNotify}, cyls::{CylMutate, CylNotify}};

pub async fn src_worker_1d<F, const CHUNK_SIZE: usize>(app: AppHandle, get_rl: impl Fn() -> F)
where
    F: Future<Output = Option<RlDataOpChunk<CHUNK_SIZE>>>
{
    loop {
        let data_chunk_op = get_rl().await;
        if let None = data_chunk_op {
            break;
        }
        let graph_state = app.state::<Arc<Mutex<GraphData1d>>>();

        let data_chunk = data_chunk_op.expect("you're in a bit of a pickle here");
        let data_chunk_iter = data_chunk.iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

        // feeding and fooding
        let graph = graph_state.lock().await;
        let rl_descrip = graph.srcs.lock().await.add(data_chunk_iter);
        let srcs = graph.srcs.lock().await;
        let pts_descrip = graph.pts.lock().await.mutate(&srcs, &rl_descrip);
        graph.pt_notify.lock().await.notify(&pts_descrip);
        let cyl_descrip = graph.cyls.lock().await.mutate(&graph.pts.lock().await, &pts_descrip);
        graph.cyl_notify.lock().await.notify(&cyl_descrip);
    }
}

pub struct GraphData1d {
    pub srcs: Mutex<Identity<RlData, Vec<RlData>>>,
    pub pts: Mutex<VecTransform<RlData, PtProp, PtMutate>>,
    pub cyls: Mutex<VecTransform<PtProp, CylProp, CylMutate>>,
    pub pt_notify: Mutex<PtNotify>,
    pub cyl_notify: Mutex<CylNotify>,
}

impl GraphData1d {
    pub fn new_empty(pt_scale: Vec3, cyl_scale: Vec3, window: Window) -> Self {
        Self{
            srcs: Mutex::new(Identity::new(vec![])),
            pts: Mutex::new(VecTransform::new(vec![], PtMutate::new(pt_scale))),
            cyls: Mutex::new(VecTransform::new(vec![], CylMutate::new(cyl_scale))),
            pt_notify: Mutex::new(PtNotify::new(window.clone())),
            cyl_notify: Mutex::new(CylNotify::new(window))
        }
    }
}