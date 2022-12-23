import React, { useMemo } from "react";
import { Vec2Fixed, Vec3Fixed } from "../model/three-helpers";
import * as THREE from 'three';

function Tip(props: {wd: number, hght: number, rotation: Vec3Fixed}) {
    return (
        <mesh position={[0,0,0]} rotation={props.rotation}>
            <coneGeometry args={[props.wd, props.hght, 32]} />
            <meshBasicMaterial color="rgb(210, 80, 100)" />
        </mesh>
    )
}

function AxisLine(props: {len: number, rad: number, arrowDim: {wd: number, hght: number}}) {
    const body =( 
        <mesh position={[0,0,0]}>
            <cylinderGeometry args={[props.rad, props.rad, props.len, 32]} />
            <meshBasicMaterial color="rgb(150,0,40)" />
        </mesh>
    )
    const displace = 0.5 * props.len
    const tipParams: {rot: Vec3Fixed, displace: number}[] =
        [ {rot: [0, 0, 0], displace}, {rot: [Math.PI, 0, 0], displace: -displace} ]

    const arrowDim = props.arrowDim
    const tips = tipParams.map((tipParam, i) => {
        const pos: Vec3Fixed = [0, tipParam.displace, 0]
        const rot = tipParam.rot
        return (
            <group key={i} position={pos}>
                <Tip 
                    wd={arrowDim.wd} 
                    hght={arrowDim.hght} 
                    rotation={rot} 
                />
            </group>
        )
    })

    return (
        <group>
            {body}
            {tips}
        </group>
    )
}

export function AxesElem(props: {ranges: [Vec2Fixed, Vec2Fixed, Vec2Fixed], 
                    rad: number, arrowDim: {wd: number, hght: number}}) {
    const axes = useMemo(() => {
        const axesParams: {key: string, rot: Vec3Fixed}[] =
            [ 
                // assuming body starts from parallel to y axis
                {key: 'x', rot: [0,0,-0.5 * Math.PI]},
                {key: 'y', rot: [0,0,0]},
                {key: 'z', rot: [0.5 * Math.PI,0,0]},
            ]
        return axesParams.map((param, i) => {
            const [beg, end] = props.ranges[i]
            const len = end - beg
            const axis_displace = beg + len * 0.5
            const pos: Vec3Fixed = [0,0,0]
            pos[i] = axis_displace
            return (
                <group key={i} position={pos} rotation={param.rot}>
                    <AxisLine 
                        len={len} 
                        rad={props.rad} 
                        arrowDim={props.arrowDim} 
                        />
                </group>
            )
        })
    }, [props])
    return (
        <group>
            {axes}
        </group>
    )
}