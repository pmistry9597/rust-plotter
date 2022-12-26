use std::time::Duration;

use rand::{rngs::StdRng, Rng};
use tauri::async_runtime;

use crate::graph::types::{RlDataOpChunk, Vec2, RlData};

pub async fn shit_data<const N: usize>(raw_in: async_runtime::Sender<RlDataOpChunk<N>>) {
    let rate = 20;
    let (x_begin, x_end) = (0, 10);
    let x_len = x_end - x_begin;
    let count = rate * x_len;
    let interv = x_len as f32 / count as f32;
  
    let yfunc = f32::sin;
    let curr_chunk_vec: Vec<Vec2> = (0..count).map(
      move |i| {
        let x = i as f32 * interv;
        [x as f32 * 0.3, 
          yfunc(x)
           as f32 * 0.3
        ] as Vec2
      }).collect();
    let mut curr_chunk_iter = curr_chunk_vec.chunks(N);
  
    loop {
      let mut src_chunk: RlDataOpChunk<N> = [None; N];
      let chunk_iter = curr_chunk_iter.next().expect("ran out eh?").iter()
      .map(|pt| {
        let mut rand_wumb: StdRng = rand::SeedableRng::from_entropy();
        let pos_vec: Vec<f32> = pt.iter().chain([rand_wumb.gen::<f32>() - 0.5].iter()).map(|x| *x).collect();
        let pos: [f32; 3] = pos_vec[0..3].try_into().unwrap();
        RlData{pos}
      });
  
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