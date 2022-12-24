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
    let real_freq_1 = vec![0.1, 0.2, 0.35, 0.4];
    let real_mag_1 = vec![4, 2, 5, 8];
    assert_eq!(real_freq_1.len(), real_mag_1.len());
    let real_1 = real_input_1.into_iter().map(|t|
        real_freq_1.iter().zip(real_mag_1.iter()).map(|(f, m)| 
            (*m as f32) * (2.0 * PI * f.to_owned() as f32 * t).cos())
                .fold(0.0, |acc, entry| acc + entry)
    );
    let insert_1 = real_in.add(real_1.clone());
    transform.change(&real_in, insert_1);

    let out = transform.get_out();
    let full_access = Accessor::Range((0, out.len()));
    // println!("len: {}", out.len());
    let out_it = out.get(full_access);
    // find maximum magnitudes
    let mut max: Vec<(usize, (f32, Complex<f32>))> = out_it.cloned().map(|comp| ((comp.re.pow(2) as f32 + comp.im.pow(2) as f32).pow(0.5), comp))
        .enumerate().collect();
    let max_len = max.len();
    let max_half = &mut max[0..(max_len as f32 * 0.5) as usize];
    max_half.sort_by(|(_, (a_len, _a_comp)), (_, (b_len, _b_comp))| {
        a_len.total_cmp(b_len)
    });
    max_half.reverse();

    // amplitude check
    let freq_count_1 = real_freq_1.len();
    assert!(max_half[0..freq_count_1].iter().all(|(found_f, (len, _))| {   
        real_freq_1.iter().zip(real_mag_1.iter()).any(|(real_f, mag)| {
            let unnorm_real_f = real_f * 1000.0;
            let f_within_5per = ((*found_f as f32 - unnorm_real_f) / unnorm_real_f).abs() < 0.05;
            let unnorm_real_mag = *mag as f32 * 1000.0 / 2.0;
            let m_within_5per = ((*len - unnorm_real_mag) / unnorm_real_mag).abs() < 0.05;
            println!("f - {} {}, m - {} {}", found_f, unnorm_real_f, len, unnorm_real_mag);
            f_within_5per && m_within_5per
        })
    }));
    // ensure remaining frequencies are just noise
    for (f, (len, _)) in &max_half[0..10] {
        println!("f: {}, len: {}", f, len);
    }
    let (_, (last_mag, _)) = max_half[freq_count_1 - 1];
    let (_, (noise_mag, _)) = max_half[freq_count_1];
    assert!(last_mag / noise_mag > 100.0);

    // todo!("check results of final product")
}