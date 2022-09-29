import React, { useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import * as THREE from 'three';
import { Vec2Fixed, Vec3Fixed } from "../model/three-helpers";
import { Bloom, EffectComposer } from "@react-three/postprocessing";
import { PerspectiveCamera } from "@react-three/drei";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { BlckInfo, CylProp, PtProp } from "../model/backend-comm";
import { Md5 } from "ts-md5"

export function Chart3d() {
    const end = 40
    const [ptprops, setptprops] = useState([] as PtProp[])
    const [meshes, setmeshes] = useState([] as CylProp[])
    const [camRef, setCamRef] = useState<React.MutableRefObject<THREE.PerspectiveCamera | null> | null>(null)

    const setCamRefHandle = useMemo(() => {
        return (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => setCamRef(camRef)
    }, [])

    const [trig, settrig] = useState(false)
    const scale: [number, number] = [1.0, 1.0]

    return (
        <>
            <Canvas dpr={[2,1]}>
                <TimedStream
                        settrig={settrig} 
                        setptprops={setptprops} 
                        setmeshes={setmeshes}
                        scale={scale} 
                        camRef={camRef} 
                        end={end} 
                    />
                <CamControl setCamRef={setCamRefHandle} />
                <AxesElem 
                    ranges={[[-4,end],[-4,4],[-4,4]]} 
                    rad={0.1} 
                    arrowDim={{wd: 0.2, hght: 0.4}} 
                />
                <Scaler scaling={[1,1,1]}>
                    <BasicFunc
                        trig={trig} 
                        settrig={settrig} 
                        ptprops={ptprops} 
                        setptprops={setptprops} 
                        meshes={meshes}
                        setmeshes={setmeshes}
                        ptRad={0.1} 
                        lineRad={0.04} 
                        scale={scale}
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

function TimedStream(props: {
    settrig: any, 
    scale: [number, number], 
    camRef: React.MutableRefObject<THREE.PerspectiveCamera | null> | null,
    setptprops: any, 
    end: number,
    setmeshes: any,
}) 
{
    const { setptprops, camRef, setmeshes } = props

    const ptprops_ref = useRef([] as PtProp[])
    setptprops(ptprops_ref.current)
    const meshes_ref = useRef([] as CylProp[])
    setmeshes(meshes_ref.current)

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
    // three js wont work with tauri on my setup without following for whatever reason
    const useless_three_js = useThree()
    useEffect(() => {
        setInterval(() => {
            useless_three_js.advance(0)
        }, 5)
    }, [])
    
    const [recentPt, setRecentPt] = useState([0,0] as Vec2Fixed)
    // data injection
    useEffect(() => {
        listen("pt_update", (event: any) => {
            const payload: BlckInfo = event.payload;
            const i = payload.index
            invoke("get_ptprop", {i}).then((ptprop_val) => {
                const ptprop = ptprop_val as PtProp
                ptprops_ref.current.push(ptprop)
                setRecentPt(ptprop.pos)

                props.settrig(true)
            }).catch((reason) => {
                console.log("huh retrieving didn't work when sent?: ",  reason)
            })
        })
        listen("mesh_update", (event: any) => {
            const payload: BlckInfo = event.payload
            const i = payload.index
            invoke("get_meshprop", {i}).then((meshprop_val) => {
                const meshprop = meshprop_val as CylProp
                meshes_ref.current.push(meshprop)

                props.settrig(true)
            })
        })
      }, [])

    // precursor to camera tracking routine
    const [currCam, setCamCurr] = useState([0,0,0])
    const displaceAlpha = 0.4
    const [displaceAvg, setDisplaceAvg] = useState(new THREE.Vector3(0,0,0))
    const cam_displace = new THREE.Vector3(10,10,10)
    // cam_displace.multiplyScalar(4)
    useFrame((_state, delta) => {
        const idealCam = (new THREE.Vector3(recentPt[0], 0, 0)).add(cam_displace)
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

function hash_equals(a: (string | Int32Array), b: (string | Int32Array)): boolean {
    if (a.length != b.length) {
        return false;
    }
    for (let i = 0; i < a.length; i += 1) {
        const a_val = a[i] as string
        const b_val = b[i] as string
        if (a_val != b_val) {
            return false;
        }
    }
    return true;
}
function exhaust_entity_info<Prop>(
    queue: [Prop, number][], 
    entity_gener: (info: [Prop, number]) => JSX.Element, 
    render_dump: JSX.Element[])
{
    while (queue.length > 0) {
        const info_index = queue.pop() as [Prop, number]
        const entity = entity_gener(info_index)
        const [_, index] = info_index

        if (index < render_dump.length) {
            render_dump[index] = entity
        } else {
            render_dump.fill(<group></group>, render_dump.length, index)
            render_dump.push(entity)
        }
    }
}
function gen_new_hashes<Prop>(props: Prop[]) {
    return props.map((prop) => {
        const hash = new Md5()
        hash.start()
        hash.appendStr(JSON.stringify(prop))
        return hash.end() || ""
    }) || []
}
function updated_props<Prop>(
    new_hashes: (string | Int32Array)[], 
    old_hashes: React.MutableRefObject<(string | Int32Array)[]>, 
    props: Prop[],
    )
{
    const updated_indices = new_hashes.map((hash, i) => [hash, i] as [(string | Int32Array), number])
        .filter(([new_hash, i]) => {
            const old_hash = old_hashes.current[i] || ""
            return !hash_equals(new_hash, old_hash)
        })
    console.log('szie of penis?', updated_indices.length)
    return updated_indices.map(([_, i]) => [props[i], i] as [Prop, number])
}
function rerender_updated<Prop>(
    props: Prop[], 
    old_hashes: React.MutableRefObject<(string | Int32Array)[]>, 
    entity_gener: (prop_w_index: [Prop, number]) => JSX.Element,
    entities: JSX.Element[] )
{
    const new_hashes = gen_new_hashes(props)
    const updated = updated_props(new_hashes, old_hashes, props)
    console.log('new vs updated poo -', new_hashes.length, updated.length)
    exhaust_entity_info(updated, entity_gener, entities)
    old_hashes.current = new_hashes
}

function BasicFunc(props: {
    trig: boolean, 
    settrig: any, 
    ptprops: PtProp[], 
    setptprops: any, 
    meshes: CylProp[],
    setmeshes: any,
    ptRad: number, 
    lineRad: number, 
    scale: Vec2Fixed, 
    end:number } ) 
{
    const ptRender = useRef([] as JSX.Element[])
    const pt_props_hash = useRef([] as (string | Int32Array)[])
    const pt_gener = useMemo(() => (ptprop_w_index: [PtProp, number]) => {
        const [ptprop, index] = ptprop_w_index
        return (
            <group position={[0,0,0]} key={index}>
                <mesh position={[...ptprop.pos, 0]}>
                    <sphereGeometry args={[props.ptRad, 32,32,32]} />
                    <meshBasicMaterial color="rgb(210,100,120)"></meshBasicMaterial>
                </mesh>
            </group>
        )
    }, [])
    const meshRender = useRef([] as JSX.Element[])
    const mesh_props_hash = useRef([] as (string | Int32Array)[])
    const mesh_gener = useMemo(() => (meshprop_w_index: [CylProp, number]) => {
        const [meshprop, index] = meshprop_w_index
        return (
            <group 
                position={meshprop.pos} 
                key={index}>
                <mesh rotation={new THREE.Euler(...meshprop.euler)} castShadow>
                    <cylinderBufferGeometry 
                        args={[props.lineRad, props.lineRad, meshprop.len, 16]} />
                    <meshBasicMaterial color="rgb(100,200,100)"></meshBasicMaterial>
                </mesh>
            </group>
        )
    }, [])
    useEffect(() => {
        const ptprops = props.ptprops
        rerender_updated(ptprops, pt_props_hash, pt_gener, ptRender.current)
        const meshprops = props.meshes
        rerender_updated(meshprops, mesh_props_hash, mesh_gener, meshRender.current)

        props.settrig(false)
    }, [props.trig])

    return (
        <group>
            <group>{ptRender.current}</group>
            <group>{meshRender.current}</group>
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