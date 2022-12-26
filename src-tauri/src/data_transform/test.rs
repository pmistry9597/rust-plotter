use std::f32::consts::PI;

use itertools::Itertools;
use rustfft::{FftPlanner, num_complex::Complex, num_traits::Pow};

use crate::data_transform::{identity::Identity, transform::VecTransform, len::Len};

use super::{Transform, mutate_info::{Accessor, MutateInfo, Mutation}, Retrieve, mutator::Mutator};
type DFTOut = Complex<f32>;
type RawIn = f32;

struct DFT {
    planner: FftPlanner<f32>
}
impl Mutator<RawIn, Vec<DFTOut>> for DFT {
    fn mutate<Source: Retrieve<RawIn>>(self: &mut Self, src: &Source, out: &mut Vec<DFTOut>, change: &MutateInfo) -> MutateInfo {
        let full_access = Accessor::Range((0, src.len()));
        let mut buf: Vec<DFTOut> = src.get(&full_access).iter().map(|entry| Complex::new(entry.to_owned(), 0.0)).collect();
        
        let fft = self.planner.plan_fft_forward(src.len());
        fft.process(&mut buf);
        let buf_len = buf.len();
        *out = buf;
        MutateInfo::Change(vec![Mutation::Replace(Accessor::Range((0, buf_len)))])
    }
}

#[test]
fn dft_full_no_notify() {
    let mutator = DFT{planner: FftPlanner::new()};
    let mut transform = VecTransform::new_empty(mutator);

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
    // keep
    let mut time_dom = Identity::new(vec![] as Vec<f32>);
    let add_1 = time_dom.add(real_1.clone());
    transform.mutate(&time_dom, &add_1);

    // assertions woo
    let full_access = Accessor::Range((0, transform.len()));
    // println!("len: {}", out.len());
    let out_it = transform.get(&full_access);
    // find maximum magnitudes
    let mut max: Vec<(usize, (f32, Complex<f32>))> = out_it.iter().map(|comp| ((comp.re.pow(2) as f32 + comp.im.pow(2) as f32).pow(0.5), *comp))
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
}

struct Scaler {
    scale: f32,
}

impl Mutator<f32, Vec<f32>> for Scaler {
    fn mutate<Source: Retrieve<f32>>(self: &mut Self, src: &Source, out: &mut Vec<f32>, change: &MutateInfo) -> MutateInfo {
        match change {
            MutateInfo::Reset => {
                out.clear();
            },
            MutateInfo::Change(changes) => {
                changes.iter().for_each(|change| {
                    match change {
                        Mutation::Add(accessor) => {
                            out.extend(src.get(&accessor).iter().map(|entry| entry * self.scale));
                        },
                        Mutation::Remove(accessor) => {
                            accessor.to_indices().for_each(|index| {
                                out.remove(index);
                            });
                        },
                        _ => {}
                    }
                });
            }
            _ => {}
        }
        change.clone()
    }
//     fn change<StoreOut: Retrieve<RawOut>>(self: &mut Self, raw: &StoreOut, out: &mut Vec<RawOut>, change: &super::mutate_info::ChangeDescrip) -> ChangeDescrip {
//         match change {
//             ChangeDescrip::Reset => {
//                 out.clear();
//             },
//             ChangeDescrip::Change(changes) => {
//                 changes.iter().for_each(|change| {
//                     match change {
//                         Change::Add(accessor) => {
//                             out.extend(raw.get(&accessor).iter().map(|entry| entry * self.scale));
//                         },
//                         Change::Remove(accessor) => {
//                             accessor.to_indices().for_each(|index| {
//                                 out.remove(index);
//                             });
//                         },
//                         _ => {}
//                     }
//                 });
//             }
//             _ => {}
//         }
//         change.clone()
//     }
}

#[test]
fn scaling_add_remove() {
    let scale = 0.2;
    let mutator = Scaler{scale};
    let mut transform = VecTransform::new_empty(mutator);


    // pushing and removing signal lul
    let mut real_in_store = Identity::new(vec![] as Vec<f32>);
    let sig_in = (0..100).map(|elem| elem as f32);
    let chunk_n = 5;
    for chonk in sig_in.chunks(chunk_n).into_iter() {
        let change = real_in_store.add(chonk);
        transform.mutate(&real_in_store, &change);
    }

    // check if added entries are expected
    // for (raw_val, out_val) in raw.get(Accessor::Range((0, raw.len()))).zip(transform.get_out().iter()) {
    //     println!("raw: {}, out: {}", raw_val, out_val);
    // }
    let full_access = Accessor::Range((0, transform.len()));
    assert!(real_in_store.get(&Accessor::Range((0, real_in_store.len()))).iter().zip(transform.get(&full_access).iter()).all(|(raw_val, out_val)| {
        raw_val * scale == *out_val
    }));

    let remov_indices = [50, 5, 0];
    let remov_change = real_in_store.remove(remov_indices.iter().map(|index| *index as usize));
    transform.mutate(&real_in_store, &remov_change);
    let full_access = Accessor::Range((0, transform.len()));
    assert!(real_in_store.get(&Accessor::Range((0, real_in_store.len()))).iter().zip(transform.get(&full_access).iter()).all(|(raw_val, out_val)| {
        raw_val * scale == *out_val
    }));
    assert!(real_in_store.len() == 100 - remov_indices.len());
}