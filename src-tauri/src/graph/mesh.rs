use tauri::Window;

use crate::data_transform::{mutator::Mutator, mutate_info::{MutateInfo, Mutation}, Retrieve, NotifyHook};

use super::{types::{PtProp, MeshProp}, notify::notify_new_data};

pub struct MeshMutate;

impl MeshMutate {
    pub fn new() -> Self {
        Self
    }
}
impl Mutator<PtProp, Vec<MeshProp>> for MeshMutate {
    fn mutate<Source: Retrieve<PtProp>>(self: &mut Self, src: &Source, out: &mut Vec<MeshProp>, change: &MutateInfo) -> MutateInfo {
        todo!();
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