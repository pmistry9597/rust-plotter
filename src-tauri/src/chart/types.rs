pub struct MetaData {
    cyl_count: i64,
    pt_count: i64,
}

pub type RlDataChunk<const N: usize> = [RlData; N];

pub type VecN<const N: usize> = [f32; N];
pub type Vec3 = VecN<3>;
pub type Vec2 = VecN<2>;

// precursor to general mesh?
pub struct CylProp {
    pos: Vec3,
    angle: f32,
    len: f32,
}

pub struct PtProp {
    pub pos: Vec2,
    pub rl_data: RlData,
}

// contains source data (not the rendering sort)
#[derive(Debug, Clone, Copy)]
pub struct RlData {
    pub pos: Vec2,
}