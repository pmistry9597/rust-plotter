import React from "react";
import { Vec3Fixed } from "../model/three-helpers";

export function Sphere(props: {pos: Vec3Fixed, diam?: number, color?: string}) {
    const diam = props.diam || 1
    const color = props.color || 'rgb(120,120,170)'
    return (
        <mesh position={props.pos}>
            <sphereGeometry args={[diam, 32,32,32]} />
            <meshBasicMaterial color={color} />
        </mesh>
    )
}