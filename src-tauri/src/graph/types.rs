use quaternion::Quaternion;
use serde::Serialize;

pub type RlDataOpChunk<const N: usize> = [Option<RlData>; N];
pub type VecN<const N: usize> = [f32; N];
pub type Vec3 = VecN<3>;
pub type Vec2 = VecN<2>;

// precursor to general mesh
#[derive(Serialize, Clone, Copy)]
pub struct CylProp {
    pub pos: Vec3,
    pub quat: Quaternion<f32>,
    pub len: f32,
}

#[derive(Serialize, Clone, Copy)]
pub struct PtProp {
    pub pos: Vec3,
    pub rl_data: RlData,
}

// contains source data (not the rendering sort)
#[derive(Debug, Serialize, Clone, Copy)]
pub struct RlData {
    pub pos: Vec3,
}