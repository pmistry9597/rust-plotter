mod notify;
mod pts;
mod cyls;
mod data_helpers;

use std::{future::Future, sync::Arc};
use futures::lock::Mutex;
use tauri::{AppHandle, Manager, Window};

use crate::data_transform::{Transform, Identity, VecTransform, NotifyHook};
use self::{types::{RlData, PtProp, CylProp, RlDataOpChunk, Vec3}, pts::{PtNotify, PtMutate}, cyls::{CylNotify, CylMutate}};

pub mod cmd;
pub mod types;

pub async fn src_worker<F, const CHUNK_SIZE: usize>(app: AppHandle, get_rl: impl Fn() -> F)
where
    F: Future<Output = Option<RlDataOpChunk<CHUNK_SIZE>>>
{
    loop {
        let data_chunk_op = get_rl().await;
        if let None = data_chunk_op {
            break;
        }
        let graph_state = app.state::<Arc<Mutex<GraphData>>>();

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

pub struct GraphData {
    srcs: Mutex<Identity<RlData, Vec<RlData>>>,
    pts: Mutex<VecTransform<RlData, PtProp, PtMutate>>,
    cyls: Mutex<VecTransform<PtProp, CylProp, CylMutate>>,
    pt_notify: Mutex<PtNotify>,
    cyl_notify: Mutex<CylNotify>,
}

impl GraphData {
    pub fn new_empty(pt_scale: Vec3, cyl_scale: Vec3, window: Window) -> Self {
        GraphData{
            srcs: Mutex::new(Identity::new(vec![])),
            pts: Mutex::new(VecTransform::new(vec![], PtMutate::new(pt_scale))),
            cyls: Mutex::new(VecTransform::new(vec![], CylMutate::new(cyl_scale))),
            pt_notify: Mutex::new(PtNotify::new(window.clone())),
            cyl_notify: Mutex::new(CylNotify::new(window))
        }
    }
}