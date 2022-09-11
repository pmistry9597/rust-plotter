import React, { useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame } from "@react-three/fiber";
import * as THREE from 'three';
import { Vec2Fixed, Vec3Fixed } from "../model/three-helpers";
import { Bloom, EffectComposer } from "@react-three/postprocessing";
import { PerspectiveCamera } from "@react-three/drei";
import { seg_prop_gen } from "./math-manip";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export function Chart3d() {
    const end = 40
    const [xyPtsCurr, setxyPts] = useState<Vec2Fixed[]>([])
    const [camRef, setCamRef] = useState<React.MutableRefObject<THREE.PerspectiveCamera | null> | null>(null)

    const setCamRefHandle = useMemo(() => {
        return (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => setCamRef(camRef)
    }, [])

    const scale: [number, number] = [1.0, 1.0]

    return (
        <>
            <Canvas dpr={[2,1]}>
                <TimedStream scale={scale} camRef={camRef} xyPtsCurr={xyPtsCurr} setxyPts={setxyPts} end={end} />
                <CamControl setCamRef={setCamRefHandle} />
                <AxesElem 
                    ranges={[[-4,end],[-4,4],[-4,4]]} 
                    rad={0.1} 
                    arrowDim={{wd: 0.2, hght: 0.4}} 
                />
                <Scaler scaling={[1,1,1]}>
                    <BasicFunc xyPtsCurr={xyPtsCurr} ptRad={0.1} lineRad={0.04} scale={scale}
                        end={end} />
                </Scaler>
                <EffectComposer>
                    <Bloom
                        intensity={1}
                        luminanceThreshold={0}
                        luminanceSmoothing={0.6}
                    />
                </EffectComposer>
            </Canvas>
        </>
    )
}

interface BlckInfo {
    index: number,
    name: string,
}
interface PtProp {
    pos: Vec2Fixed,
}

function TimedStream(props: {scale: [number, number], camRef: React.MutableRefObject<THREE.PerspectiveCamera | null> | null, xyPtsCurr: Vec2Fixed[], setxyPts: any, end: number}) {
    const { xyPtsCurr, setxyPts, camRef } = props
    const [xscale, yscale] = props.scale

    const xyref = useRef([] as Vec2Fixed[])
    setxyPts(xyref.current)

    // const [trigger, setTrigger] = useState(false)
    const [timer, setTimer] = useState(0)
    const [camTimer, setCamTimer] = useState(0)
    const [camDate, setCamDate] = useState(false)

    // below are timed shits
    useFrame((_state, delta) => {
        setTimer(timer + delta)
        setCamTimer(camTimer + delta)
        if (camTimer > 0.02) {
            setCamDate(!camDate)
            setCamTimer(0)
        }
    })
    
    // pt injection
    useEffect(() => {
        listen("pt_update", (event: any) => {
            const payload: BlckInfo = event.payload;
            invoke("get_ptprop", {i: payload.index}).then((ptprop_val) => {
                const ptprop = ptprop_val as PtProp
                const newxyPtsCurr = [...xyref.current]
                newxyPtsCurr.push(ptprop.pos)
                console.log(newxyPtsCurr)
                xyref.current = newxyPtsCurr
            }).catch((reason) => {
                console.log("huh retrieving didn't work when sent?: ",  reason)
            })
        })
      }, [])

    // precursor to camera tracking routine
    const [currCam, setCamCurr] = useState([0,0,0])
    const displaceAlpha = 0.4
    const [displaceAvg, setDisplaceAvg] = useState(new THREE.Vector3(0,0,0))
    const [cam_displace, setCamDisp] = useState(new THREE.Vector3(10,10,10))
    useFrame((_state, delta) => {
        const dimMap = [xscale, yscale]
        const recentFuncPtScaled = xyPtsCurr.at(-1)?.map((val, i) => val * dimMap[i]) as Vec2Fixed || [0,0] as Vec2Fixed
        const idealCam = (new THREE.Vector3(recentFuncPtScaled[0], 0, 0)).add(cam_displace)
        const camVec = new THREE.Vector3(...currCam)
        const diff2Ideal = idealCam.sub(camVec)

        const decayTimeBase = Math.pow(0.7, 100)
        const displace = diff2Ideal.multiplyScalar(Math.pow(decayTimeBase, delta))
        setDisplaceAvg(displace.multiplyScalar(displaceAlpha).add(displaceAvg.multiplyScalar(1 - displaceAlpha)))
        const [camX, camY, camZ] = currCam
        const newCamPt: Vec3Fixed = [camX + displaceAvg.x, camY + displaceAvg.y, camZ + displaceAvg.z]
        setCamCurr(newCamPt)
    })
    useEffect(() => {
        camRef?.current?.position.setX(currCam[0])
        camRef?.current?.position.setY(currCam[1])
        camRef?.current?.position.setZ(currCam[2])
        camRef?.current?.lookAt(camRef?.current?.position.clone().sub(cam_displace))
        setCamDate(false)
    }, [camDate])

    return <></>
}

function genPoints(end: number) {
    const range = [0, end]
    const rate = 15
    const size = 5 * rate
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

function CamControl(props: {setCamRef: (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => void}) {
    const camR: React.MutableRefObject<THREE.PerspectiveCamera | null> = useRef(null)

    useEffect(() => {
        if (!camR.current) {
            return
        }
        props.setCamRef(camR)
    }, [camR])

    return (
        <PerspectiveCamera
            up={[0,1,0]}
            ref={camR}
            makeDefault
        />
    )
}

function BasicFunc(props: {xyPtsCurr: Vec2Fixed[], ptRad: number, lineRad: number, scale: Vec2Fixed, end:number}) {
    const [oldData, setOldData] = useState([] as Vec2Fixed[])
    const [entities, setEntities] = useState([] as JSX.Element[])
    useEffect(() => {
        const xypts = props.xyPtsCurr

        const tobeUpdated = xypts
        .map((xy, i) => {
            const xyfut = xypts[i + 1]
            return [xy, xyfut, i] as [Vec2Fixed, Vec2Fixed | undefined, number]
        })
        .filter(([xy, _dataIndex], i) => {
            const xyfut = xypts[i + 1]
            const oldDat = oldData[i]
            const xyold = oldDat
            const oldDatFut = oldData[i + 1]
            const xyfutold = oldDatFut

            return (xy !== xyold) || (xyfut !== xyfutold)
        })

        const newEntities = tobeUpdated.map(([xy, xyfut, dataIndex], _i) => {
            const segProp = seg_prop_gen(xy, xyfut, props.scale)
            const cylProp = segProp.cyl

            const bodySeg = !!cylProp ?
            (
                <group position={cylProp.pos} key={dataIndex}>
                    <mesh rotation={new THREE.Euler(0, 0, cylProp.slopeAngle)} castShadow>
                        <cylinderBufferGeometry 
                            args={[props.lineRad, props.lineRad, cylProp.len ,16]} />
                        <meshBasicMaterial color="rgb(100,200,100)"></meshBasicMaterial>
                    </mesh>
                </group>
            ) : null

            const newEntry = (
                <group position={[0,0,0]} key={dataIndex}>
                    {bodySeg}
                    <mesh position={segProp.ptPos}>
                        <sphereGeometry args={[props.ptRad, 32,32,32]} />
                        <meshBasicMaterial color="rgb(210,100,120)"></meshBasicMaterial>
                    </mesh>
                </group>
            )

            return [newEntry, dataIndex] as [JSX.Element, number]
        })

        newEntities.forEach(([newEntry, dataIndex]) => {
            if (dataIndex < entities.length) {
                entities[dataIndex] = newEntry
            } else {
                entities.push(newEntry)
            }
        })
        setEntities(entities)
        setOldData(xypts)
    }, [props.xyPtsCurr])

    return (
        <group>
            {entities}
        </group>
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

function AxisLine(props: {len: number, rad: number, arrowDim: {wd: number, hght: number}}) {
    const body =( 
        <mesh position={[0,0,0]}>
            <cylinderGeometry args={[props.rad, props.rad, props.len, 32]} />
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
        <group>
            {body}
            {tips}
        </group>
    )
}

function AxesElem(props: {ranges: [Vec2Fixed, Vec2Fixed, Vec2Fixed], 
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