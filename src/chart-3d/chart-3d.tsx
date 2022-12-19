import React, { useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import * as THREE from 'three';
import { Vec3Fixed } from "../model/three-helpers";
import { Bloom, EffectComposer } from "@react-three/postprocessing";
import { PerspectiveCamera } from "@react-three/drei";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { BlckInfo, CylProp, PtProp } from "../model/backend-comm";
import { rerender_updated } from "./rendering";
import { AxesElem, Scaler } from "./elems";

export function Chart3d() {
    const end = 40
    const [ptprops, setptprops] = useState([] as PtProp[])
    const [cyls, setcyls] = useState([] as CylProp[])
    const [camRef, setCamRef] = useState<React.MutableRefObject<THREE.PerspectiveCamera | null> | null>(null)

    const setCamRefHandle = useMemo(() => {
        return (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => setCamRef(camRef)
    }, [])

    const [trig, settrig] = useState(false)

    return (
        <>
            <Canvas dpr={[2,1]}>
                <BackendEndpoint
                        settrig={settrig} 
                        setptprops={setptprops} 
                        setcyls={setcyls}
                    />
                <Threejs_fix />
                <CamTrack camRef={camRef} ptprops={ptprops} />
                <CamInstall setCamRef={setCamRefHandle} />
                <AxesElem 
                    ranges={[[-4,end],[-4,4],[-4,4]]} 
                    rad={0.1} 
                    arrowDim={{wd: 0.2, hght: 0.4}} 
                />
                <Scaler scaling={[1,1,1]}>
                    <Graph1D
                        trig={trig} 
                        settrig={settrig} 
                        ptprops={ptprops} 
                        setptprops={setptprops} 
                        cyls={cyls}
                        setcyls={setcyls}
                        ptRad={0.1} 
                        lineRad={0.04}
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

function CamTrack(props: {
    camRef: React.MutableRefObject<THREE.PerspectiveCamera | null> | null,
    ptprops: PtProp[],
})
{
    const { camRef, ptprops } = props
    const [timer, setTimer] = useState(0)
    const [camDate, setCamDate] = useState(false)
    const [camTimer, setCamTimer] = useState(0)

    useFrame((_state, delta) => {
        setTimer(timer + delta)
        setCamTimer(camTimer + delta)
        if (camTimer > 0.02) {
            setCamDate(!camDate)
            setCamTimer(0)
        }
    })
    const [currCam, setCamCurr] = useState([0,0,0])
    const displaceAlpha = 0.4
    const [displaceAvg, setDisplaceAvg] = useState(new THREE.Vector3(0,0,0))
    const cam_displace = new THREE.Vector3(10,10,10)
    const recentx_pt = ptprops.at(-1)?.pos[0]

    useFrame((_state, delta) => {
        const idealCam = (new THREE.Vector3(recentx_pt, 0, 0)).add(cam_displace)
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

function Threejs_fix() {
    // three js wont work with tauri on my setup without following for whatever reason
    const useless_three_js = useThree()
    useEffect(() => {
        setInterval(() => {
            useless_three_js.advance(0)
        }, 2)
    }, [])

    return <></>
}

function BackendEndpoint(props: {
    settrig: any,
    setptprops: any,
    setcyls: any,
}) 
{
    const { setptprops, setcyls } = props
    const ptprops_ref = useRef([] as PtProp[])
    setptprops(ptprops_ref.current)
    const cyles_ref = useRef([] as CylProp[])
    setcyls(cyles_ref.current)

    useEffect(() => {
        listen("pt_update", (event: any) => {
            const payload: BlckInfo = event.payload;
            const i = payload.index
            invoke("get_ptprop", {i}).then((ptprop_val) => {
                const ptprop = ptprop_val as PtProp
                ptprops_ref.current.push(ptprop)
                props.settrig(true)
            }).catch((reason) => {
                console.log("huh retrieving didn't work when sent?: ",  reason)
            })
        })
        listen("cyl_update", (event: any) => {
            const payload: BlckInfo = event.payload
            const i = payload.index
            invoke("get_cylprop", {i}).then((cylprop_val) => {
                const cylprop = cylprop_val as CylProp
                cyles_ref.current.push(cylprop)
                props.settrig(true)
            })
        })
        invoke("ready").then(() => {
            console.log("readied up eh?")
        })
      }, [])

    return <></>
}

function CamInstall(props: {setCamRef: (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => void}) {
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

function Graph1D(props: {
    trig: boolean, 
    settrig: any, 
    ptprops: PtProp[], 
    setptprops: any, 
    cyls: CylProp[],
    setcyls: any,
    ptRad: number, 
    lineRad: number,
    end: number } ) 
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
    const cyl_props_hash = useRef([] as (string | Int32Array)[])
    const mesh_gener = useMemo(() => (cylprop_w_index: [CylProp, number]) => {
        const [cylprop, index] = cylprop_w_index
        return (
            <group 
                position={cylprop.pos} 
                key={index}>
                <mesh rotation={new THREE.Euler(...cylprop.euler)} castShadow>
                    <cylinderBufferGeometry 
                        args={[props.lineRad, props.lineRad, cylprop.len, 16]} />
                    <meshBasicMaterial color="rgb(100,200,100)"></meshBasicMaterial>
                </mesh>
            </group>
        )
    }, [])
    useEffect(() => {
        const ptprops = props.ptprops
        rerender_updated(ptprops, pt_props_hash, pt_gener, ptRender.current)
        const cylprops = props.cyls
        rerender_updated(cylprops, cyl_props_hash, mesh_gener, meshRender.current)

        props.settrig(false)
    }, [props.trig])

    return (
        <group>
            <group>{ptRender.current}</group>
            <group>{meshRender.current}</group>
        </group>
    )
}