use std::f32::consts::PI;

use itertools::Itertools;
use rustfft::{FftPlanner, num_complex::Complex, num_traits::Pow};
use super::{transform::Transform, processor::Processor, notify_hook::{EmptyNotifyHook, NotifyHook}, store::Store, len::Len, retrieve::Retrieve, change_desrip::{Accessor, ChangeDescrip, Change}};

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
impl Processor<Vec<DFTOut>, RawIn, &mut Vec<RawIn>> for DFTTransformer {
    fn change<StoreType: Retrieve<RawIn>>(self: &mut Self, raw: &StoreType, out: &mut Vec<DFTOut>, _change: &super::change_desrip::ChangeDescrip) -> ChangeDescrip {
        let full_access = Accessor::Range((0, raw.len()));
        let mut buf: Vec<DFTOut> = raw.get(&full_access).map(|entry| Complex::new(entry.to_owned(), 0.0)).collect();
        
        let fft = self.planner.plan_fft_forward(raw.len());
        fft.process(&mut buf);
        let buf_len = buf.len();
        *out = buf;
        ChangeDescrip::Change(vec![Change::Replace(Accessor::Range((0, buf_len)))])
    }
}

#[test]
fn dft_full_no_notify() {
    let mut real_in_store: Vec<RawIn> = vec![];
    let mut real_in = Store::new(&mut real_in_store);
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
    transform.change(&real_in, &insert_1);
    let mut out = transform.get_out();

    // assertions woo
    let full_access = Accessor::Range((0, out.len()));
    // println!("len: {}", out.len());
    let out_it = out.get(&full_access);
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
}

type RawOut = f32;
struct ScaleTransformer {
    scale: f32,
}
impl Processor<Vec<RawOut>, RawOut, &mut Vec<RawIn>> for ScaleTransformer {
    fn change<StoreOut: Retrieve<RawOut>>(self: &mut Self, raw: &StoreOut, out: &mut Vec<RawOut>, change: &super::change_desrip::ChangeDescrip) -> ChangeDescrip {
        match change {
            ChangeDescrip::Reset => {
                out.clear();
            },
            ChangeDescrip::Change(changes) => {
                changes.iter().for_each(|change| {
                    match change {
                        Change::Add(accessor) => {
                            out.extend(raw.get(&accessor).map(|entry| entry * self.scale));
                        },
                        Change::Remove(accessor) => {
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
}

struct TestNotify<'a> {
    notifs: &'a mut Vec<ChangeDescrip>,
}
impl<'a> NotifyHook for TestNotify<'a> {
    fn notify(self: &mut Self, change: &ChangeDescrip) {
        self.notifs.push(change.clone());
    }
}

#[test]
fn scaling_add_remove_notify() {
    let scale = 0.2;
    let processor = ScaleTransformer{scale};
    let mut notif_history = vec![] as Vec<ChangeDescrip>;
    let mut transform = Transform::new(vec![] as Vec<f32>, processor, TestNotify{notifs: &mut notif_history});


    // pushing and removing signal lul
    let mut real_in_store: Vec<RawIn> = vec![];
    let mut raw = Store::new(&mut real_in_store);
    let sig_in = (0..100).map(|elem| elem as f32);
    let chunk_n = 5;
    let mut notif_expected = vec![] as Vec<ChangeDescrip>;
    for chonk in sig_in.chunks(chunk_n).into_iter() {
        let change = raw.add(chonk);
        transform.change(&raw, &change);
        notif_expected.push(change);
    }

    // check if added entries are expected
    // for (raw_val, out_val) in raw.get(Accessor::Range((0, raw.len()))).zip(transform.get_out().iter()) {
    //     println!("raw: {}, out: {}", raw_val, out_val);
    // }
    assert!(raw.get(&Accessor::Range((0, raw.len()))).zip(transform.get_out().iter()).all(|(raw_val, out_val)| {
        *raw_val * scale == *out_val
    }));

    let remov_indices = [50, 5, 0];
    let remov_change = raw.remove(remov_indices.iter().map(|index| *index as usize));
    transform.change(&raw, &remov_change);
    notif_expected.push(remov_change);
    assert!(raw.get(&Accessor::Range((0, raw.len()))).zip(transform.get_out().iter()).all(|(raw_val, out_val)| {
        *raw_val * scale == *out_val
    }));
    assert!(raw.len() == 100 - remov_indices.len());
    assert!(itertools::equal(notif_expected, notif_history));
}