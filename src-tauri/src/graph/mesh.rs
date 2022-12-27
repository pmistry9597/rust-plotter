use tauri::Window;

use crate::data_transform::{mutator::Mutator, mutate_info::{MutateInfo, Mutation}, Retrieve, NotifyHook};

use super::{types::{PtProp, MeshProp, BufferGeom, BufferAttrib}, notify::notify_new_data};

pub struct MeshMutate;

impl MeshMutate {
    pub fn new() -> Self {
        Self
    }
}
impl Mutator<PtProp, Vec<MeshProp>> for MeshMutate {
    fn mutate<Source: Retrieve<PtProp>>(self: &mut Self, src: &Source, out: &mut Vec<MeshProp>, change: &MutateInfo) -> MutateInfo {
        out.push(MeshProp{
            buffer_geom: BufferGeom{
                position: BufferAttrib::<f32>{
                    array: vec![-10, 6, 4,
                                0, -5, 4,
                                -1, 5, -1,].iter().map(|poo| *poo as f32 + 1.0).collect(),
                    item_size: 3,
                },
                index: BufferAttrib::<usize>{ 
                    array: vec![0, 1, 2], 
                    item_size: 1, 
                }
            },
            colour: "green"
        });
        MutateInfo::new_add_single(out.len())
    }
}

pub struct MeshNotify {
    window: Window
}
impl MeshNotify {
    pub fn new(window: Window) -> Self {
        Self{window}
    }
}
impl NotifyHook for MeshNotify {
    fn notify(self: &mut Self, change: &MutateInfo) {
        if let MutateInfo::Change(changes) = change {
            for change in changes {
                if let Mutation::Add(access) = change {
                    notify_new_data("mesh_update", access.to_indices(), &self.window);
                }
            }
        }
    }
}