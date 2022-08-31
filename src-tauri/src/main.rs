#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod chart;

use std::sync::Arc;
use futures::lock::Mutex;
use chart::types as chart_types;
use chart::worker_proc_src_chunk;
use tauri::async_runtime;

async fn shit_data<const N: usize>(raw_in: async_runtime::Sender<chart_types::RlDataChunk<N>>) {
  let rl_data = chart_types::RlData{pos: [0.1,0.2,0.3]};
  let out: chart_types::RlDataChunk<N> = [rl_data; N];
  if let Err(_err) = raw_in.send(out).await {
    println!("this fucker took a shit!");
  }
}

fn main() {
  let buf_size: usize = 7;
  let (raw_in, raw_out) = async_runtime::channel::<chart_types::RlDataChunk<3>>(buf_size); // should move into closure if not used outside
  let raw_out_arc = Arc::new(Mutex::new(raw_out)); // finish fucker

  tauri::Builder::default()
    .setup(|_app| {
      async_runtime::spawn(shit_data(raw_in));
      async_runtime::spawn(worker_proc_src_chunk(
        move || {
          let raw_out_arc = raw_out_arc.clone();
          async move {
            raw_out_arc.lock().await.recv().await
          }
        }
      ));
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}