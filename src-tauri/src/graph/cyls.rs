use itertools::Itertools;
use tauri::Window;
use crate::data_transform::{NotifyHook, ChangeDescrip, change_desrip::Change, Processor, Retrieve};
use super::{notify::notify_new_data, types::{PtProp, Vec3, CylProp}, data_helpers::{get_xyz_delta, intrpol, pythag_tup3}};

pub struct CylProcess {
    scale: Vec3
}
impl Processor<Vec<CylProp>, PtProp, Vec<PtProp>> for CylProcess {
    fn change<StoreType: Retrieve<PtProp>>(self: &mut Self, raw: &StoreType, out: &mut Vec<CylProp>, change: &ChangeDescrip) -> ChangeDescrip {
        if let ChangeDescrip::Change(changes) = change {
            for change in changes {
                if let Change::Add(access) = change {
                    let out_pts = gen_cylprops_iter(raw.get(access), self.scale);
                    out.extend(out_pts);
                }
            }
            ChangeDescrip::Change(changes.clone())
        } else {
            ChangeDescrip::None
        }
    }
}

pub struct CylNotify {
    window: Window
}
impl NotifyHook for CylNotify {
    fn notify(self: &mut Self, change: &ChangeDescrip) {
        if let ChangeDescrip::Change(changes) = change {
            for change in changes {
                if let Change::Add(access) = change {
                    notify_new_data("cyl_update", access.to_indices(), &self.window);
                }
            }
        }
    }
}

fn gen_cylprops_iter<'p, PtPropIter>(ptprop_iter: PtPropIter, scale: Vec3) -> impl Iterator<Item = CylProp> + 'p
where
    PtPropIter: Iterator<Item = &'p PtProp> + 'p,
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