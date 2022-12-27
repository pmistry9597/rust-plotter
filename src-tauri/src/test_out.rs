use std::time::Duration;

use rand::{rngs::StdRng, Rng};
use tauri::async_runtime;
use crate::graph::types::{RlDataOpChunk, Vec2, RlData, RlPointSlice};

pub async fn shit_data_mesh(raw_in: async_runtime::Sender<RlPointSlice>) {
  let width_wise_bound = 10.0 as f32;
  let (count, interv) = gen_graph_param(5.0, (0.0, 40.0));

  let mut x_pts_it = (0..count as i32).map(|i| (i as f32) * interv);
  let fn_y = |x: f32, z: f32| {
    x.sin() + z.sin()
  };

  loop {
    let valid_slice = x_pts_it.next().and_then(|x| {
      let mut count_gen: StdRng = rand::SeedableRng::from_entropy();
      let slice_count = count_gen.gen_range(1..4);
      let slice_z_pos = (0..slice_count).map(|i| (i as f32 / slice_count as f32) * width_wise_bound);
      let slice_z = slice_z_pos.clone().chain(slice_z_pos.map(|z| -z));
      let mut noise_gen: StdRng = rand::SeedableRng::from_entropy();
      let pts_slice = slice_z.map(|z| {
        let x = x + noise_gen.gen::<f32>();
        let z = z + noise_gen.gen::<f32>();
        [x, fn_y(x,z), z]
      });
      // let pts_slice = pts_slice.map(|[x, y, z]| {
      //   [x + noise_gen.gen::<f32>() * 0.0, y + noise_gen.gen::<f32>() * 0.0, z + 0.0 * noise_gen.gen::<f32>()]
      // });
      let rl_point_slice = RlPointSlice{
        pts: pts_slice.map(|pos| RlData{pos}).collect()
      };
      Some(rl_point_slice)
    }).ok_or(());
    let sender = match valid_slice {
      Err(_) => {
        println!("no points left? or smthn else?");
        break;
      }
      Ok(slice) => raw_in.send(slice)
    };
    if let Err(fuck) = sender.await {
      println!("you slut, we couldn't send a damn thing!");
      println!("err {}", fuck);
    }

    tokio::time::sleep(Duration::from_millis(300)).await;
  }
}

pub async fn shit_data_1d<const N: usize>(raw_in: async_runtime::Sender<RlDataOpChunk<N>>) {
  let (count, interv) = gen_graph_param(20.0, (0.0, 10.0));
  
  let yfunc = f32::sin;
  let curr_chunk_vec: Vec<Vec2> = (0..count as i32).map(
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

fn gen_graph_param(rate: f32, x_bound: (f32, f32)) -> (f32, f32) {
  let (x_begin, x_end) = x_bound;
  let x_len = x_end - x_begin;
  let count = rate * x_len;
  let interv = x_len / count;
  (count, interv)
}