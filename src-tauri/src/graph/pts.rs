use tauri::Window;
use crate::{data_transform::{NotifyHook, ChangeDescrip, change_desrip::Change, Processor, Retrieve}};
use super::{notify::notify_new_data, types::{PtProp, Vec3, RlData}, data_helpers::scale_vecn};

pub struct PtProcess {
    scale: Vec3
}
impl PtProcess {
    pub fn new(scale: Vec3) -> Self {
        PtProcess{scale}
    }
}
impl Processor<Vec<PtProp>, RlData, &mut Vec<RlData>> for PtProcess {
    fn change<StoreType: Retrieve<RlData>>(self: &mut Self, raw: &StoreType, out: &mut Vec<PtProp>, change: &ChangeDescrip) -> ChangeDescrip {
        if let ChangeDescrip::Change(changes) = change {
            for change in changes {
                if let Change::Add(access) = change {
                    let out_pts = gen_ptprops_iter(raw.get(access).into_iter(), self.scale);
                    out.extend(out_pts);
                }
            }
            ChangeDescrip::Change(changes.clone())
        } else {
            ChangeDescrip::None
        }
    }
}

pub struct PtNotify {
    window: Window
}
impl PtNotify {
    pub fn new(window: Window) -> Self {
        PtNotify{window}
    }
}
impl NotifyHook for PtNotify {
    fn notify(self: &mut Self, change: &ChangeDescrip) {
        if let ChangeDescrip::Change(changes) = change {
            for change in changes {
                if let Change::Add(access) = change {
                    notify_new_data("pt_update", access.to_indices(), &self.window);
                }
            }
        }
    }
}

fn gen_ptprops_iter<'d, RlDataIter>(data_chunk_iter: RlDataIter, scale: Vec3) -> impl Iterator<Item = PtProp> + 'd
where 
    RlDataIter: Iterator<Item = RlData> + 'd,
{
    data_chunk_iter.map(move |data| {
        let pos = scale_vecn(data.pos, scale);
        PtProp{pos, rl_data: data}
    })
}