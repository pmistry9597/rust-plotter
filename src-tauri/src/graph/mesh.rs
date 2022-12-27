use tauri::Window;
use crate::data_transform::{mutator::Mutator, mutate_info::{MutateInfo, Mutation, Accessor}, Retrieve, NotifyHook};
use super::{types::{PtProp, MeshProp, BufferGeom, BufferAttrib}, notify::notify_new_data};
use nannou::geom;
use nannou::geom::tri::Tri;

pub struct MeshMutate {
    prev_pts_len: Option<usize>
}

impl MeshMutate {
    pub fn new() -> Self {
        Self{prev_pts_len: None}
    }
}
impl Mutator<PtProp, Vec<MeshProp>> for MeshMutate {
    fn mutate<Source: Retrieve<PtProp>>(self: &mut Self, src: &Source, out: &mut Vec<MeshProp>, change: &MutateInfo) -> MutateInfo {
        if let MutateInfo::Change(changes) = change {
            if let Mutation::Add(Accessor::Range((begin, end))) = &changes[0] {
                let pts_len = end - begin;

                if let Some(prev_pts_len) = self.prev_pts_len {
                    // may need to reverse one of these slices
                    let prev_slice = src.get(&Accessor::reverse_range(prev_pts_len, src.len() - pts_len));
                    let prev_slice_points_it = prev_slice
                        .iter().rev().map(|ptprop| {
                            let [x,y,z] = ptprop.pos;
                            geom::point::pt3(x,y,z)
                        });
                    let slice = src.get(&Accessor::reverse_range(pts_len, src.len()));
                    let slice_points_it = slice
                        .iter().rev().map(|ptprop| {
                            let [x,y,z] = ptprop.pos;
                            geom::point::pt3(x,y,z)
                        });
                    let tri_it = geom::polygon::triangles(prev_slice_points_it.chain(slice_points_it));
                    let triangles: Vec<Tri> = tri_it.expect("wtf bro").collect();
                    let pos_it = triangles.iter().fold(Box::new([].into_iter()) as Box<dyn Iterator<Item = f32>>, |it, tri| {
                        let Tri(pts) = tri;
                        let mut pts_flat = vec![];
                        for pt in pts {
                            pts_flat.push(pt.x);
                            pts_flat.push(pt.y);
                            pts_flat.push(pt.z);
                        }
                        Box::new(it.chain(pts_flat.into_iter()))
                    });
                    let positions = pos_it.collect::<Vec<_>>();
                    let no_triangles = positions.len();
                    let meshprop = MeshProp {
                        buffer_geom: BufferGeom{
                            position: BufferAttrib::<f32>{
                                array: positions,
                                item_size: 3,
                            },
                            index: BufferAttrib::<usize>{ 
                                array: (0..no_triangles / 3).collect(), 
                                item_size: 1, 
                            }
                        },
                        colour: [0.0, 1.0, 0.1]
                    };
                    out.push(meshprop);
                    
                    self.prev_pts_len = Some(pts_len);
                    MutateInfo::Change(vec![Mutation::Add(
                        Accessor::reverse_range(1, out.len())
                    )])
                } else {
                    self.prev_pts_len = Some(pts_len);
                    MutateInfo::None
                }
            } else {
                MutateInfo::None
            }
        } else {
            MutateInfo::None
        }
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