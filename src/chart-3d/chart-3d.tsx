import React, { useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import * as THREE from 'three';
import { Vec3Fixed } from "../model/three-helpers";
import { Bloom, EffectComposer } from "@react-three/postprocessing";
import { PerspectiveCamera } from "@react-three/drei";
import { seg_prop_gen } from "./math-manip";

export function Chart3d() {
    return (
        <>
            <Canvas dpr={[2,1]}>
                <CamControl />
                <AxesElem 
                    len={5} 
                    diam={0.1} 
                    arrowDim={{wd: 0.2, hght: 0.4}} 
                />
                <Scaler scaling={[1,1,1]}>
                    <BasicFunc ptDiam={0.1} lineDiam={0.04} scale={[0.7, 1.0]} />
                </Scaler>
                <EffectComposer>
                    <Bloom
                        intensity={7}
                        luminanceThreshold={0}
                        luminanceSmoothing={0.6}
                    />
                </EffectComposer>
            </Canvas>
        </>
    )
}

function CamControl() {
    const [doneSet, setDone] = useState(false)
    const [quat, setQuat] = useState(new THREE.Quaternion())

    useEffect(() => {
        quat.setFromUnitVectors(new THREE.Vector3(0,0,1), 
            (new THREE.Vector3(1,0,1))
            .normalize())
        const upRot = (new THREE.Quaternion())
            .setFromUnitVectors(
                (new THREE.Vector3(1,0,1)).normalize(), 
                (new THREE.Vector3(1,1,1)).normalize()
            )
        setQuat(upRot.multiply(quat))
        setDone(true)
    }, [])
    useFrame((state, delta) => {
        if (!doneSet) {
            return
        }
        const axis = new THREE.Vector3(0.2,1,0.2).normalize()
        const mainRot = new THREE.Quaternion().setFromAxisAngle(axis, Math.PI * 0.1 * delta)
        setQuat(mainRot.multiply(quat))
    })

    return (
        <group quaternion={quat}>
            <PerspectiveCamera
                up={[0,1,0]}
                position={[0,0,13]}
                makeDefault
            />
        </group>
    )
}

function genPoints() {
    const range = [0, 10]
    const size = 30
    const interv = (range[1] - range[0]) / size
    const xPre: number[] = [...Array<number>(size)].map((_, i) => {
        return i * interv
    })
    const x = xPre
    return x.map((xval, i) => {
        const yval = Math.sin(xval)
        return [xval, yval]
    }) as [number, number][]
}

function BasicFunc(props: {ptDiam: number, lineDiam: number, scale: [number, number]}) {
    const plotData = useMemo(() => {
        const xypts = genPoints()

        return xypts
                    .map((xy, i) => {
            const xyfut = xypts[i + 1]
            const segProps = seg_prop_gen(xy, xyfut, props.scale)
            const cylProps = segProps.cyl
            
            const bodySeg = !!cylProps ?
            (
                <group position={cylProps.pos} key={i}>
                    <mesh rotation={new THREE.Euler(0, 0, cylProps.slopeAngle)} castShadow>
                        <cylinderBufferGeometry 
                            args={[props.lineDiam, props.lineDiam, cylProps.len ,16]} />
                        <meshBasicMaterial color="rgb(100,200,100)"></meshBasicMaterial>
                    </mesh>
                </group>
            ) : null

            return (
                <group position={[0,0,0]} key={i}>
                    {bodySeg}
                    <mesh position={segProps.ptPos}>
                        <sphereGeometry args={[props.ptDiam, 32,32,32]} />
                        <meshBasicMaterial color="rgb(210,100,120)"></meshBasicMaterial>
                    </mesh>
                </group>
            )
        })
    }, [])

    return (
        <>
            <group castShadow>
                {plotData}
            </group>
        </>
    )
}

function Scaler(props: {children: JSX.Element | JSX.Element[], scaling: [number, number, number]}) {
    const scaleVec = new THREE.Vector3(...props.scaling)
    return (
        <group scale={scaleVec}>
            {props.children}
        </group>
    )
}

function Tip(props: {wd: number, hght: number, rotation: Vec3Fixed}) {
    return (
        <mesh position={[0,0,0]} rotation={props.rotation}>
            <coneGeometry args={[props.wd, props.hght, 32]} />
            <meshBasicMaterial color="rgb(210, 80, 100)" />
        </mesh>
    )
}

function AxisLine(props: {len: number, rot: Vec3Fixed, diam: number, arrowDim: {wd: number, hght: number}}) {
    const body =( 
        <mesh position={[0,0,0]}>
            <cylinderGeometry args={[props.diam, props.diam, props.len, 32]} />
            <meshBasicMaterial color="rgb(150,0,150)" />
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
        <group rotation={props.rot}>
            {body}
            {tips}
        </group>
    )
}

function AxesElem(props: {len: number, diam: number, arrowDim: {wd: number, hght: number}}) {
    const axesParams: {key: string, rot: Vec3Fixed}[] =
        [ 
            {key: 'x', rot: [0,0,0.5 * Math.PI]},
            {key: 'y', rot: [0,0,0]},
            {key: 'z', rot: [0.5 * Math.PI,0,0]},
        ]
    const axes = axesParams.map((param, i) => {
        return <AxisLine 
                    key={i}
                    len={props.len} 
                    rot={param.rot} 
                    diam={props.diam} 
                    arrowDim={props.arrowDim} 
                    />
    })
    return (
        <group>
            {axes}
        </group>
    )
}