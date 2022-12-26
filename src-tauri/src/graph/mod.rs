mod notify;
mod pts;
mod cyls;
mod data_helpers;

use std::{future::Future, sync::Arc, cell::RefCell};
use futures::lock::Mutex;
use tauri::{AppHandle, Manager, Window};

use crate::data_transform::{Transform, Store};
use self::{types::{RlData, PtProp, CylProp, RlDataOpChunk, Vec3}, pts::{PtNotify, PtProcess}, cyls::{CylNotify, CylProcess}};

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
        //.expect("nothing there??")
        let rl_descrip = graph.srcs.borrow_mut().as_mut().expect("fuck").add(data_chunk_iter);
        let srcs = graph.srcs.borrow(); //Ref::map(graph.srcs.borrow(), |srcs: &'static Store<RlData, Vec<RlData>>| &srcs);
        let pts_descrip = graph.pts.borrow_mut().change(srcs.as_ref().expect("fuck"), &rl_descrip);
        // need cyls to read pts transform in graph
        let pts_store = graph.pts_store.borrow();
        graph.cyls.borrow_mut().change(pts_store.as_ref().expect("fuck"), &pts_descrip);
    }
}

pub struct GraphData<'p> {
    srcs_vec: Mutex<Vec<RlData>>,
    pts: Mutex<PtTransform<'p>>,
    cyls: Mutex<CylTransform<'p>>,
    srcs: Mutex<Option<SrcStore<'p>>>,
    pts_store: Mutex<Option<PtStore<'p>>>,
}

type SrcStore<'p> = Store<RlData, &'p Mutex<Vec<RlData>>>;
type PtTransform<'p> = Transform<Vec<PtProp>, PtProcess, PtNotify, RlData, &'p mut Vec<RlData>>;
type PtStore<'p> = Store<PtProp, &'p mut Vec<PtProp>>;
type CylTransform<'p> = Transform<Vec<CylProp>, CylProcess, CylNotify, PtProp, &'p mut Vec<PtProp>>;

impl<'p> GraphData<'p> {
    pub fn new(srcs_vec_in: Vec<RlData>,
                pts_vec: Vec<PtProp>,
                cyls_vec: Vec<CylProp>,
                pt_scale: Vec3,
                cyl_scale: Vec3,
                window: Window,
            ) -> Self 
    {
        let srcs_vec = RefCell::new(srcs_vec_in);
        let pts = RefCell::new(Transform::new(pts_vec, PtProcess::new(pt_scale), PtNotify::new(window.clone())));
        let cyls = RefCell::new(Transform::new(cyls_vec, CylProcess::new(cyl_scale), CylNotify::new(window)));

        // let srcs = RefCell::new(Store::new(&mut srcs_vec.borrow_mut() as &mut Vec<RlData>));
        // let pts_store = RefCell::new(Store::new(pts.borrow_mut().get_out_mut()));
        let mut graph = GraphData{
            srcs_vec, 
            pts, 
            cyls,
            pts_store: RefCell::new(None), 
            srcs: RefCell::new(None), 
        };
        let srcs = RefCell::new(
            Some(
                Store::new(&graph.srcs_vec)
            )
        );
        graph.srcs = srcs;


        graph
    }
}