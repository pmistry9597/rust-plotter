use itertools::Itertools;
use tauri::Window;
use crate::data_transform::{NotifyHook, Retrieve, mutate_info::{MutateInfo, Mutation, Accessor}, mutator::Mutator};
use super::{notify::notify_new_data, types::{PtProp, Vec3, CylProp}, data_helpers::{get_xyz_delta, intrpol, pythag_tup3}};

pub struct CylMutate {
    scale: Vec3
}
impl CylMutate {
    pub fn new(scale: Vec3) -> Self {
        CylMutate{scale}
    }
}
impl Mutator<PtProp, Vec<CylProp>> for CylMutate {
    fn mutate<Source: Retrieve<PtProp>>(self: &mut Self, src: &Source, out: &mut Vec<CylProp>, change: &MutateInfo) -> MutateInfo {
        if let MutateInfo::Change(changes) = change {
            let changes = changes.iter().map(|change| {
                if let Mutation::Add(access) = change {
                    let mut access = access.clone();
                    access.extend_left(1);
                    Mutation::Add(access)
                } else {
                    change.clone()
                }
            });
            let mut changes = changes.collect::<Vec<_>>();
            changes.iter_mut().for_each(|change| {
                if let Mutation::Add(access) = change {
                    let out_pts_it = gen_cylprops_iter(src.get(&access).into_iter(), self.scale);
                    let out_pts = out_pts_it.collect::<Vec<_>>();
                    let change_len = out_pts.len();
                    out.extend(out_pts);
                    *change = Mutation::Add(Accessor::Range((out.len() - change_len, out.len())));
                }
            });
            MutateInfo::Change(changes)
        } else {
            MutateInfo::None
        }
    }
}

pub struct CylNotify {
    window: Window
}
impl CylNotify {
    pub fn new(window: Window) -> Self {
        CylNotify{window}
    }
}
impl NotifyHook for CylNotify {
    fn notify(self: &mut Self, change: &MutateInfo) {
        if let MutateInfo::Change(changes) = change {
            for change in changes {
                if let Mutation::Add(access) = change {
                    notify_new_data("cyl_update", access.to_indices(), &self.window);
                }
            }
        }
    }
}

fn gen_cylprops_iter<'p, PtPropIter>(ptprop_iter: PtPropIter, scale: Vec3) -> impl Iterator<Item = CylProp> + 'p
where
    PtPropIter: Iterator<Item = PtProp> + 'p,
{
    ptprop_iter.tuple_windows().map(move |(xyz, xyzfut)| {
        let (xyz, delta) = get_xyz_delta(&xyz.pos, &xyzfut.pos, scale);
        let (delta_x, delta_y, delta_z) = delta;

        let (intrp_x, intrp_y, intrp_z) = intrpol(xyz, delta, 0.5);
        let len = pythag_tup3(delta);
        let target = [delta_z, delta_y, -delta_x];
        let quat = quaternion::rotation_from_to([0.0, -1.0, 0.0], target);

        CylProp {
            pos: [intrp_x, intrp_y, intrp_z], 
            quat,
            len,
        }
    })
}