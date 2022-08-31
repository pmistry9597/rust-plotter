pub mod types;

use std::{future::Future};
use types::{ RlDataChunk };

pub async fn worker_proc_src_chunk<F, const CHUNK_SIZE: usize>(get_rl: impl Fn() -> F)
where
    F: Future<Output = Option<RlDataChunk<CHUNK_SIZE>>>
{
    loop {
        let data_chunk_op = get_rl().await;
        if let None = data_chunk_op {
            break;
        }
        println!("Here sum dat that jus came in hum dah: {:?}", data_chunk_op);
    }
}