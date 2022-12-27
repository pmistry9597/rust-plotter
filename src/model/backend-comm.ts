import { Vec3Fixed } from "./three-helpers";

export interface CylProp {
    len: number,
    quat: [number, Vec3Fixed],
    pos: Vec3Fixed,
}
export interface BlckInfo {
    index: number,
    name: string,
}
export interface RlData {
    pos: Vec3Fixed,
}
export interface PtProp {
    pos: Vec3Fixed,
    rl_data: RlData,
}
export interface MeshProp {
    buffer_geom: BufferGeom,
    colour: string,
}
export interface BufferGeom {
    position: BufferAttribf,
    index: BufferAttribi,
}
export interface BufferAttribf {
    array: Float32Array,
    item_size: number,
}
export interface BufferAttribi {
    array: Uint32Array,
    item_size: number,
}