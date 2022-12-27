use quaternion::Quaternion;
use serde::Serialize;

pub type OpChunk<Type, const N: usize> = [Option<Type>; N];
pub type RlDataOpChunk<const N: usize> = OpChunk<RlData, N>;
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

// kinda like below but for mesh case
#[derive(Serialize, Clone, Copy)]
pub struct PtProp {
    pub pos: Vec3,
    pub rl_data: RlData,
}

#[derive(Serialize, Clone)]
pub struct MeshProp {
    pub buffer_geom: BufferGeom,
    pub colour: &'static str,
}

#[derive(Serialize, Clone)]
pub struct BufferGeom {
    pub position: BufferAttrib<f32>,
    pub index: BufferAttrib<f32>,
}

#[derive(Serialize, Clone)]
pub struct BufferAttrib<T> {
    // pub attach: &'static str,
    pub array: Vec<T>,
    pub item_size: usize,
}

#[derive(Debug, Serialize, Clone)]
pub struct RlPointSlice {
    pub pts: Vec<RlData>
}

// contains source data (not the rendering sort)
#[derive(Debug, Serialize, Clone, Copy)]
pub struct RlData {
    pub pos: Vec3,
}