use itertools::Itertools;

use super::types::*;

type TupVec3 = (f32, f32, f32);

pub fn gen_cylprops_iter<PtPropIter>(ptprop_iter: PtPropIter, scale: Vec3) -> impl Iterator<Item = CylProp> + Clone
where
    PtPropIter: Iterator<Item = PtProp> + Clone,
{
    ptprop_iter.tuple_windows().map(move |(xy, xyfut)| {
        let (xyz, delta) = get_xyz_delta(&xy.pos, &xyfut.pos, scale);
        let (delta_x, delta_y, delta_z) = delta;

        let (intrp_x, intrp_y, intrp_z) = intrpol(xyz, delta, 0.5);
        let len = pythag_tup3(delta);
        let target = [-delta_z, delta_y, delta_x];
        let quat = quaternion::rotation_from_to([0.0, 1.0, 0.0], target);

        CylProp {
            pos: [intrp_x, intrp_y, intrp_z], 
            quat,
            len,
        }
    })
}

fn get_xyz_delta(xy: &Vec3, xyfut: &Vec3, scale: Vec3) -> (TupVec3, TupVec3) {
    let (scale_x, scale_y, scale_z) = vec3_to_tup(&scale);
    let xyz = vec3_to_tup(xy);
    let xyzfut = vec3_to_tup(xyfut);
    let (x, y, z) = xyz;
    let (xfut, yfut, zfut) = xyzfut;
    let orig = (x * scale_x, y * scale_y, z * scale_z);
    (
        orig, 
        (xfut * scale_x - orig.0, yfut * scale_y - orig.1, zfut * scale_z - orig.2),
    )
}

// fn pythag_tup2(lens: (f32, f32)) -> f32 {
//     let (x, y) = lens;
//     pythag([x,y].into_iter())
// }
fn pythag_tup3(lens: TupVec3) -> f32 {
    let (x, y, z) = lens;
    pythag([x,y,z].into_iter())
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

fn intrpol(p: TupVec3, delta: TupVec3, frac: f32) -> TupVec3 {
    let (x, y, z) = p;
    let (delta_x, delta_y, delta_z) = delta;
    (
        intrpol_num(x, delta_x, frac), 
        intrpol_num(y, delta_y, frac),
        intrpol_num(z, delta_z, frac),
    )
}
fn intrpol_num(start: f32, delta: f32, frac: f32) -> f32 {
    delta * frac + start
}

// fn vec2_to_tup(vec: &Vec2) -> (f32, f32) {
//     (vec[0], vec[1])
// }
fn vec3_to_tup(vec: &Vec3) -> (f32, f32, f32) {
    (vec[0], vec[1], vec[2])
}

pub fn gen_ptprops_iter<RlDataIter>(data_chunk_iter: RlDataIter, scale: Vec3) -> impl Iterator<Item = PtProp> + Clone
where 
    RlDataIter: Iterator<Item = RlData> + Clone,
{
    data_chunk_iter.map(move |data| {
        let pos = scale_vecn(data.pos, scale);
        PtProp{pos, rl_data: data}
    })
}

pub fn scale_vecn<const N: usize>(pt: VecN<N>, scale: VecN<N>) -> VecN<N> {
    let scaled_vec: Vec<f32> = pt.iter().zip(scale.iter()).map(|(elem, scaler)| elem * scaler).collect();
    scaled_vec[0..3].try_into().expect("wow what a stupid failure")
}