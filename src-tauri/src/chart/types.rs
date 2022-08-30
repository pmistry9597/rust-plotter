pub struct MetaData {
    cylCount: i64,
    ptCount: i64,
}

pub type Vec3 = [f32; 3];

// precursor to general mesh?
pub struct CylProp {
    pos: Vec3,
    angle: f32,
    len: f32,
}

pub struct PtProp {
    pos: Vec3,
    rlData: RlData,
}

// contains source data (not the rendering sort)
pub struct RlData {
    pos: Vec3,
}