#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod chart;

use std::{sync::Arc, time::Duration};
use chart::types::Vec2;
use futures::{lock::Mutex};
use chart::{types as chart_types, ChartProc};
use tauri::async_runtime;

async fn shit_data<const N: usize>(raw_in: async_runtime::Sender<chart_types::RlDataChunk<N>>) {
  let rate = 20;
  let (x_begin, x_end) = (0, 40);
  let x_len = x_end - x_begin;
  let count = rate * x_len;
  let interv = x_len as f32 / count as f32;

  let yfunc = f32::sin;
  let curr_chunk_vec: Vec<Vec2> = (0..count).map(
    move |i| {
      let x = i as f32 * interv;
      [x, yfunc(x)] as chart_types::Vec2
    }).collect();
  let mut curr_chunk_iter = curr_chunk_vec.chunks(N);
  loop {
    let mut src_chunk = [chart_types::RlData{pos:[-2.0,-2.0]}; N];
    let chunk_iter = curr_chunk_iter.next().expect("ran out eh?").iter()
    .map(|pt| chart_types::RlData{pos: pt.clone()});

    src_chunk.iter_mut().zip(chunk_iter)
    .for_each(|(arr, chunk)| *arr = chunk);

    if let Err(_err) = raw_in.send(src_chunk).await {
      println!("this fucker took a shit!");
      break;
    }
    
    tokio::time::sleep(Duration::from_millis(700)).await;
  }
}

fn main() {
  let buf_size: usize = 7;

  tauri::Builder::default()
    .setup(move |_app| {
      let (raw_in, raw_out) = async_runtime::channel::<chart_types::RlDataChunk<3>>(buf_size); // should move into closure if not used outside
      let raw_out_arc = Arc::new(Mutex::new(raw_out));

      let chart_proc = ChartProc::new();
      async_runtime::spawn(chart_proc.src_chunk_worker(
        move || {
          let raw_out_arc = raw_out_arc.clone();
          async move {
            raw_out_arc.lock().await.recv().await
          }
        }
      ));
      async_runtime::spawn(shit_data(raw_in));
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}