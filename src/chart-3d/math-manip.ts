import { Vec3Fixed } from "../model/three-helpers"

export interface CylProp {
    len: number,
    slopeAngle: number,
    pos: Vec3Fixed,
}

export interface SegProp {
    cyl: CylProp | null,
    ptPos: Vec3Fixed,
}

export function seg_prop_gen(xy0: [number, number], xy1: [number, number], scale: [number, number]) {
    const [x_raw, y_raw] = xy0
    const [xscale, yscale] = scale

    
    const y = y_raw * yscale, x = x_raw * xscale
    const ptPos = [x, y, 0] as Vec3Fixed

    const segProp: SegProp = {
        ptPos,
        cyl: null,
    }
    if (!xy1) {
        return segProp
    }
    
    const [xfut_raw, yfut_raw] = xy1

    const xfut = xfut_raw * xscale, yfut = yfut_raw * yscale
    const deltaX = xfut - x, deltaY = yfut - y

    const [interp_x, interp_y] = intrpol([x, y], [deltaX, deltaY], 0.5)
    
    const len = pythag(deltaX, deltaY)
    const slopeAngle = Math.atan(-deltaX / deltaY)
    const cylpos = [interp_x, interp_y, 0] as Vec3Fixed

    const cylProp: CylProp = {
        len,
        slopeAngle,
        pos: cylpos,
    }
    segProp.cyl = cylProp

    return segProp
}

function pythag(...lens: number[]) {
    let sum = 0
    lens.forEach((len) => {
        sum += len * len
    })
    return Math.pow(sum, 0.5)
}

function intrpol(p0 : number[], delta : number[], frac : number): number[] {
    const x = delta[0] * frac + p0[0]
    const y = delta[1] * frac + p0[1]
    return [x, y]
}