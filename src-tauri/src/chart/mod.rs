pub mod types;
use types::{ MetaData };

pub struct ChartProc {}

impl ChartProc {
    pub fn new_pt() {

    }
    pub fn output_handle<F>(handle: F)
    where
    F : Fn(MetaData) -> () {

    }
}