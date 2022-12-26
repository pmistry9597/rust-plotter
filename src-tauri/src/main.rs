mod notify_block;
mod task_start;
mod single_consumable;
mod data_transform;
mod graph;
mod test_out;

use std::sync::Arc;
use futures::lock::Mutex;
use graph::{types::RlDataOpChunk, graph_1d::{GraphData1d, src_worker_1d}};
use tauri::{async_runtime, Manager, generate_handler};
use task_start::{ready, Task, generate_tasklist};

use crate::graph::cmd::{get_ptprop, get_cylprop};

fn main() {
  let buf_size: usize = 7;

  tauri::Builder::default()
    .setup(move |app| {
      let (raw_in, raw_out) = async_runtime::channel::<RlDataOpChunk<3>>(buf_size); // should move into closure if not used outside
      let raw_out_arc = Arc::new(Mutex::new(raw_out));
      app.manage(Arc::new(
        Mutex::new(
          GraphData1d::new_empty(
            [3.0,2.0,1.0],
            [1.0,1.0,1.0], 
            app.get_window("main").expect("couldn't get the window on Graph creation?")
          )
        )));

      let tasks_list: Vec<Task> = vec![
        Box::pin(src_worker_1d(
          app.handle(),
          move || {
            let raw_out_arc = raw_out_arc.clone();
            async move {
              raw_out_arc.lock().await.recv().await
            }
          }
        )),
        Box::pin(test_out::shit_data(raw_in)),
      ];
      app.manage(generate_tasklist(tasks_list.into_iter()));
      Ok(())
    })
    .invoke_handler(generate_handler![
      get_ptprop,
      get_cylprop,
      ready,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}