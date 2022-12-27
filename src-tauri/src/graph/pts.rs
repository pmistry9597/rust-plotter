use tauri::Window;
use crate::{data_transform::{NotifyHook, Retrieve, mutate_info::{MutateInfo, Mutation}, mutator::Mutator}};
use super::{notify::notify_new_data, types::{PtProp, Vec3, RlData, RlPointSlice}, data_helpers::scale_vecn};

pub struct PtMutate {
    scale: Vec3
}
impl PtMutate {
    pub fn new(scale: Vec3) -> Self {
        PtMutate{scale}
    }
}
impl Mutator<RlData, Vec<PtProp>> for PtMutate {
    fn mutate<Source: Retrieve<RlData>>(self: &mut Self, src: &Source, out: &mut Vec<PtProp>, change: &MutateInfo) -> MutateInfo {
        if let MutateInfo::Change(changes) = change {
            for change in changes {
                if let Mutation::Add(access) = change {
                    let out_pts = gen_ptprops_iter(src.get(access).into_iter(), self.scale);
                    out.extend(out_pts);
                }
            }
            MutateInfo::Change(changes.clone())
        } else {
            MutateInfo::None
        }
    }
}

pub struct PtMeshMutate {
    scale: Vec3
}
impl PtMeshMutate {
    pub fn new(scale: Vec3) -> Self {
        Self{scale}
    }
}
impl Mutator<RlPointSlice, Vec<PtProp>> for PtMeshMutate {
    fn mutate<Source: Retrieve<RlPointSlice>>(self: &mut Self, src: &Source, out: &mut Vec<PtProp>, change: &MutateInfo) -> MutateInfo {
        if let MutateInfo::Change(changes) = change {
            for change in changes {
                if let Mutation::Add(access) = change {
                    let out_pts = gen_ptprops_iter_slice(src.get(access).into_iter(), self.scale);
                    out.extend(out_pts);
                }
            }
            MutateInfo::Change(changes.clone())
        } else {
            MutateInfo::None
        }
    }
}

pub struct PtNotify {
    window: Window,
    notify_name: &'static str,
}
impl PtNotify {
    pub fn new(window: Window) -> Self {
        PtNotify{window, notify_name: "pt_update"}
    }
    pub fn new_name(window: Window, notify_name: &'static str) -> Self {
        Self{window, notify_name}
    }
}
impl NotifyHook for PtNotify {
    fn notify(self: &mut Self, change: &MutateInfo) {
        if let MutateInfo::Change(changes) = change {
            for change in changes {
                if let Mutation::Add(access) = change {
                    notify_new_data(self.notify_name, access.to_indices(), &self.window);
                }
            }
        }
    }
}

fn gen_ptprops_iter_slice<SliceIter>(data_chunk_iter: SliceIter, scale: Vec3) -> Box<dyn Iterator<Item = PtProp>>
where 
    SliceIter: Iterator<Item = RlPointSlice>,
{
    let slice_iters = data_chunk_iter.map(move |data| gen_ptprops_iter(data.pts.clone().into_iter(), scale));
    slice_iters.fold(Box::new([].into_iter()) as Box<dyn Iterator<Item = PtProp>>, |it, slice_iter| {
        Box::new(it.chain(slice_iter))
    })
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