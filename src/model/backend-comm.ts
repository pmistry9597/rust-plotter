import { Vec2Fixed, Vec3Fixed } from "./three-helpers";

export interface CylProp {
    len: number,
    euler: Vec3Fixed,
    pos: Vec3Fixed,
}
export interface BlckInfo {
    index: number,
    name: string,
}
export interface RlData {
    pos: Vec2Fixed,
}
export interface PtProp {
    pos: Vec2Fixed,
    rl_data: RlData,
}