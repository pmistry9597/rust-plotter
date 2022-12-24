mod types;
mod notify;
mod pts;
mod cyls;
mod data_helpers;

use std::{future::Future, sync::Arc, cell::{RefCell, Ref}};
use futures::lock::Mutex;
use tauri::{AppHandle, Manager};

use crate::data_transform::{Transform, Store};
use self::{types::{RlData, PtProp, CylProp, RlDataOpChunk}, pts::{PtNotify, PtProcess}, cyls::{CylNotify, CylProcess}};

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
        let mut graph = graph_state.lock().await;
        let rl_descrip = graph.srcs.borrow_mut().add(data_chunk_iter);
        let srcs = graph.srcs.borrow(); //Ref::map(graph.srcs.borrow(), |srcs: &'static Store<RlData, Vec<RlData>>| &srcs);
        let pts_descrip = graph.pts.borrow_mut().change(&srcs, &rl_descrip);
        // need cyls to read pts transform in graph
        let pts = graph.pts.borrow().get_out();
        // let pts_store = Store::new(pts);
        // graph.cyls.borrow_mut().change(&pts_store, &pts_descrip);
    }
}

pub struct GraphData<'p> {
    srcs: RefCell<Store<RlData, &'p mut Vec<RlData>>>,
    pts: RefCell<Transform<Vec<PtProp>, PtProcess, PtNotify, RlData, &'p mut Vec<RlData>>>,
    cyls: RefCell<Transform<Vec<CylProp>, CylProcess, CylNotify, PtProp, &'p mut Vec<PtProp>>,>
}