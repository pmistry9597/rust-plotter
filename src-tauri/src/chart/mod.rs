pub mod types;

use std::{future::Future};
use types::{ RlDataChunk };

use self::types::*;

pub struct ChartProc {
    srcs: Vec<RlData>,
    mesh_props: Vec<CylProp>,
    pt_props: Vec<PtProp>,
}

impl ChartProc {
    pub fn new() -> ChartProc {
        ChartProc{
            srcs: vec![],
            mesh_props: vec![],
            pt_props: vec![],
        }
    }
    pub async fn src_chunk_worker<F, const CHUNK_SIZE: usize>(mut self: Self, get_rl: impl Fn() -> F)
    where
        F: Future<Output = Option<RlDataChunk<CHUNK_SIZE>>>
    {
        loop {
            let data_chunk_op = get_rl().await;
            if let None = data_chunk_op {
                break;
            }
            let data_chunk = &data_chunk_op.expect("you're in a bit of a pickle here");

            // process section
            // pt props
            let scale: [f32; 2] = [1.0, 1.0];
            let new_pt_props_iter = data_chunk.iter().map(|data| {
                let pos = scale_vec2(data.pos, scale);
                PtProp{pos, rl_data: *data}
            });
            // mesh props?

            self.pt_props.extend(new_pt_props_iter);
            self.srcs.extend(data_chunk.iter());
        }
    }
}

fn scale_vec2(pt: Vec2, scale: Vec2) -> Vec2 {
    let scaled_vec: Vec<f32> = pt.iter().zip(scale.iter()).map(|(elem, scaler)| elem * scaler).collect();
    scaled_vec[0..1].try_into().expect("wow what a stupid failure")
}