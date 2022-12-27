mod notify_block;
mod task_start;
mod single_consumable;
mod data_transform;
mod graph;
mod test_out;

use std::sync::Arc;
use futures::lock::Mutex;
use graph::{types::{RlDataOpChunk, RlPointSlice}, graph_1d::{GraphData1d, src_worker_1d}, graph_mesh::{GraphDataMesh, src_worker_mesh}};
use tauri::{async_runtime, Manager, generate_handler};
use task_start::{ready, Task, generate_tasklist};

use crate::graph::cmd::{get_ptprop_1d, get_cylprop_1d, get_ptprop_mesh};

fn main() {
  let buf_size: usize = 7;

  tauri::Builder::default()
    .setup(move |app| {
      let (raw_in, raw_out) = async_runtime::channel::<RlDataOpChunk<3>>(buf_size);
      let raw_out_arc = Arc::new(Mutex::new(raw_out));
      app.manage(Arc::new(
        Mutex::new(
          GraphData1d::new_empty(
            [3.0,2.0,1.0],
            [1.0,1.0,1.0], 
            app.get_window("main").expect("couldn't get the window on Graph creation?")
          )
        )));

      let (in_mesh, out_mesh) = async_runtime::channel::<RlPointSlice>(buf_size);
      let out_mesh_arc = Arc::new(Mutex::new(out_mesh));
      app.manage(Arc::new(
        Mutex::new(
          GraphDataMesh::new_empty([3.0,2.0,1.0], 
            app.get_window("main").expect("couldn't get the window on Graph creation?"))
        )
      ));

      let tasks_list: Vec<Task> = vec![
        // Box::pin(src_worker_1d(
        //   app.handle(),
        //   move || {
        //     let raw_out_arc = raw_out_arc.clone();
        //     async move {
        //       raw_out_arc.lock().await.recv().await
        //     }
        //   }
        // )),
        Box::pin(test_out::shit_data_1d(raw_in)),
        Box::pin(test_out::shit_data_mesh(in_mesh)),
        Box::pin(src_worker_mesh(app.handle(), 
          move || {
            let out_mesh_arc = out_mesh_arc.clone();
            async move {
              out_mesh_arc.lock().await.recv().await
            }
          }
        )),
      ];
      app.manage(generate_tasklist(tasks_list.into_iter()));
      Ok(())
    })
    .invoke_handler(generate_handler![
      get_ptprop_1d,
      get_cylprop_1d,
      get_ptprop_mesh,
      // here goes mesh funcitno uwu
      ready,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}