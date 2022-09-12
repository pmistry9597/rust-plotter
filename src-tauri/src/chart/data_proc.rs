use itertools::Itertools;

use super::types::*;

type TupVec2 = (f32, f32);

pub fn gen_meshprops_iter<PtPropIter>(ptprop_iter: PtPropIter, scale: Vec2) -> impl Iterator<Item = CylProp> + Clone
where
    PtPropIter: Iterator<Item = PtProp> + Clone,
{
    ptprop_iter.tuple_windows().map(move |(xy, xyfut)| {
        let (xy, delta) = get_xy_delta(&xy.pos, &xyfut.pos, scale);
        let (delta_x, delta_y) = delta;

        let (intrp_x, intrp_y) = intrpol(xy, delta, 0.5);
        let len = pythag_tup2(delta);
        let z_angle = (-delta_x / delta_y).atan();

        CylProp {
            pos: [intrp_x, intrp_y, 0.0], 
            euler: [0.0, 0.0, z_angle], 
            len,
        }
    })
}

fn get_xy_delta(xy: &Vec2, xyfut: &Vec2, scale: Vec2) -> (TupVec2, TupVec2) {
    let (scale_x, scale_y) = vec2_to_tup(&scale);
    let xy = vec2_to_tup(xy);
    let xyfut = vec2_to_tup(xyfut);
    let (x, y) = xy;
    let (xfut, yfut) = xyfut;
    ((x * scale_x, y * scale_y), (xfut * scale_x - x, yfut * scale_y - y))
}

fn pythag_tup2(lens: (f32, f32)) -> f32 {
    let (x, y) = lens;
    pythag([x,y].into_iter())
}
fn pythag<LenIter>(len_iter: LenIter) -> f32 
where
    LenIter: Iterator<Item = f32>
{
    let mut sum = 0.0 as f32;
    len_iter.for_each(|len| {
        sum += len * len;
    });
    sum.powf(0.5)
}

fn intrpol(p: (f32, f32), delta: (f32, f32), frac: f32) -> (f32, f32) {
    let (x, y) = p;
    let (delta_x, delta_y) = delta;
    (intrpol_num(x, delta_x, frac), intrpol_num(y, delta_y, frac))
}
fn intrpol_num(start: f32, delta: f32, frac: f32) -> f32 {
    delta * frac + start
}

fn vec2_to_tup(vec: &Vec2) -> (f32, f32) {
    (vec[0], vec[1])
}

pub fn gen_ptprops_iter<RlDataIter>(data_chunk_iter: RlDataIter, scale: Vec2) -> impl Iterator<Item = PtProp> + Clone
where 
    RlDataIter: Iterator<Item = RlData> + Clone,
{
    data_chunk_iter.map(move |data| {
        let pos = scale_vec2(data.pos, scale);
        PtProp{pos, rl_data: data}
    })
}

pub fn scale_vec2(pt: Vec2, scale: Vec2) -> Vec2 {
    let scaled_vec: Vec<f32> = pt.iter().zip(scale.iter()).map(|(elem, scaler)| elem * scaler).collect();
    scaled_vec[0..2].try_into().expect("wow what a stupid failure")
}