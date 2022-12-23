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