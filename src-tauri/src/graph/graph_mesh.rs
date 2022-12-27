use std::sync::Arc;

use futures::{lock::Mutex, Future};
use tauri::{Window, AppHandle, Manager};
use crate::data_transform::{Identity, VecTransform, Transform, NotifyHook, mutate_info::{MutateInfo, Mutation, Accessor}};
use super::{types::{RlPointSlice, PtProp, MeshProp, Vec3}, pts::{PtMeshMutate, PtNotify}, mesh::{MeshMutate, MeshNotify}};

pub async fn src_worker_mesh<F>(app: AppHandle, get_rl: impl Fn() -> F)
where
    F: Future<Output = Option<RlPointSlice>>
{
    loop {
        let data_op = get_rl().await;
        if let None = data_op {
            break;
        }
        let graph_state = app.state::<Arc<Mutex<GraphDataMesh>>>();

        let slice = data_op.expect("you're in a bit of a pickle here");

        // // feeding and fooding
        let graph = graph_state.lock().await;
        let rl_descrip = graph.srcs.lock().await.add(vec![slice].into_iter());
        let srcs = graph.srcs.lock().await;
        let pts_descrip = graph.pts.lock().await.mutate(&srcs, &rl_descrip);
        graph.pt_notify.lock().await.notify(&pts_descrip);
        let mesh_info = graph.mesh.lock().await.mutate(&graph.pts.lock().await, &pts_descrip);
        graph.mesh_notify.lock().await.notify(&mesh_info);
    }
}

pub struct GraphDataMesh {
    pub srcs: Mutex<Identity<RlPointSlice, Vec<RlPointSlice>>>,
    pub pts: Mutex<VecTransform<RlPointSlice, PtProp, PtMeshMutate>>,
    pub mesh: Mutex<VecTransform<PtProp, MeshProp, MeshMutate>>,
    pub pt_notify: Mutex<PtNotify>,
    pub mesh_notify: Mutex<MeshNotify>,
}

impl GraphDataMesh {
    pub fn new_empty(pt_scale: Vec3, window: Window) -> Self {
        Self{
            srcs: Mutex::new(Identity::new(vec![])),
            pts: Mutex::new(VecTransform::new(vec![], PtMeshMutate::new(pt_scale))),
            mesh: Mutex::new(VecTransform::new(vec![], MeshMutate::new())),
            pt_notify: Mutex::new(PtNotify::new_name(window.clone(), "pt_mesh_update")),
            mesh_notify: Mutex::new(MeshNotify::new(window))
        }
    }
}