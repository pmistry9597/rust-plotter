use std::{f32::consts::PI};

use rustfft::{FftPlanner, num_complex::Complex, num_traits::Pow};

use super::{transform::Transform, processor::Processor, notify_hook::EmptyNotifyHook, orig::Raw, len::Len, retrieve::Retrieve, change_desrip::Accessor};

type DFTOut = Complex<f32>;
type RawIn = f32;
struct DFTTransformer {
    planner: FftPlanner<f32>
}
impl DFTTransformer {
    pub fn new() -> Self {
        DFTTransformer{planner: FftPlanner::new()}
    }
}
impl Processor<Vec<DFTOut>, RawIn, Vec<RawIn>> for DFTTransformer {
    fn change(self: &mut Self, raw: &super::orig::Raw<RawIn, Vec<RawIn>>, out: &mut Vec<DFTOut>, _change: &super::change_desrip::ChangeDescrip) {
        let full_access = Accessor::Range((0, raw.len()));
        let mut buf: Vec<DFTOut> = raw.get(full_access).map(|entry| Complex::new(entry.to_owned(), 0.0)).collect();
        
        let fft = self.planner.plan_fft_forward(raw.len());
        fft.process(&mut buf);
        *out = buf;
    }
}

#[test]
fn dft_full_no_notify() {
    let real_in_store: Vec<RawIn> = vec![];
    let mut real_in = Raw::new(real_in_store);
    let processor = DFTTransformer::new();
    let mut transform = 
            Transform::new(vec![] as Vec<DFTOut>, processor, EmptyNotifyHook);

    // insert signals we like into real
    let real_1_len = 1000;
    let real_input_1 = (0..real_1_len).into_iter().map(|num| num as f32);
    let real_freq_1 = vec![0.2];
    let real_1 = real_input_1.into_iter().map(|t|
        real_freq_1.iter().map(|f| (2.0 * PI * f.to_owned() as f32 * t).cos()).fold(0.0, |acc, entry| acc + entry)
    );
    let insert_1 = real_in.add(real_1.clone());
    transform.change(&real_in, insert_1);

    // ASSERT WOo
    // transfer back into real frequency domain
    let out = transform.get_out();
    let full_access = Accessor::Range((0, out.len()));
    println!("len: {}", out.len());
    let out_it = out.get(full_access);
    // find maximum magnitudes
    let mut max: Vec<(usize, (f32, Complex<f32>))> = out_it.cloned().map(|comp| ((comp.re.pow(2) as f32 + comp.im.pow(2) as f32).pow(0.5), comp))
        .enumerate().collect();
    max.sort_by(|(_, (a_len, _a_comp)), (_, (b_len, _b_comp))| {
        a_len.total_cmp(b_len)
    });
    let max_len = max.len();
    // let ind = 0 ;// max.len() - 1;
    // println!("max freq: {}, max: {}, {}", max[ind].0, max[ind].1.0, max[ind].1.1);
    for (f, (len, comp)) in &max[max_len - 30 .. max_len] {
        println!("f: {}, len: {}, comp: {}", f, len, comp);
    }
    let avg_real_1 = real_1.fold(0.0, |acc, entry| acc + entry) / (real_1_len as f32);
    println!("avg real 1: {}", avg_real_1);
    // for entry in out_it {
    //     println!("{}", entry);
    // }
    todo!("figure out how dft works and why this shit is unintuitive");
    // let result: Vec<f32> = out_it.map(|comp| comp.re).collect();
}