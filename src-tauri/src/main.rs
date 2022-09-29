mod chart;
mod notify_block;

use std::{sync::Arc, time::Duration};
use chart::types::Vec2;
use futures::lock::Mutex;
use chart::{types as chart_types, ChartProc, src_chunk_worker};
use tauri::{async_runtime, Manager, generate_handler};
use rand::rngs::StdRng;
use rand::Rng;
use chart::{get_ptprop, get_meshprop};

fn main() {
  let buf_size: usize = 7;

  tauri::Builder::default()
    .setup(move |app| {
      let (raw_in, raw_out) = async_runtime::channel::<chart_types::RlDataOpChunk<3>>(buf_size); // should move into closure if not used outside
      let raw_out_arc = Arc::new(Mutex::new(raw_out));

      app.manage(Arc::new(Mutex::new(ChartProc::new(app.get_window("main").unwrap()))));
      async_runtime::spawn(
        src_chunk_worker(
        app.handle(),
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
    .invoke_handler(generate_handler![
      get_ptprop,
      get_meshprop,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn shit_data<const N: usize>(raw_in: async_runtime::Sender<chart_types::RlDataOpChunk<N>>) {
  let rate = 20;
  let (x_begin, x_end) = (0, 40);
  let x_len = x_end - x_begin;
  let count = rate * x_len;
  let interv = x_len as f32 / count as f32;

  let yfunc = f32::sin;
  let curr_chunk_vec: Vec<Vec2> = (0..count).map(
    move |i| {
      let mut rand_cum: StdRng = rand::SeedableRng::from_entropy();
      let x = i as f32 * interv;
      [x + rand_cum.gen_range(0..2) as f32 * 0.3, yfunc(x) + rand_cum.gen_range(0..2) as f32 * 0.3] as chart_types::Vec2
    }).collect();
  let mut curr_chunk_iter = curr_chunk_vec.chunks(N);

  loop {
    let mut src_chunk: chart_types::RlDataOpChunk<N> = [None; N];
    let chunk_iter = curr_chunk_iter.next().expect("ran out eh?").iter()
    .map(|pt| chart_types::RlData{pos: pt.clone()});

    let mut rand_cum: StdRng = rand::SeedableRng::from_entropy();

    src_chunk.iter_mut().zip(chunk_iter)
    .filter(|_garbo| rand_cum.gen_range(0..10) as f32 / 10.0 > 0.5)
    .for_each(|(arr, chunk)| *arr = Some(chunk));

    if let Err(_err) = raw_in.send(src_chunk).await {
      println!("this fucker took a shit!");
      break;
    }
    
    tokio::time::sleep(Duration::from_millis(300)).await;
  }
}