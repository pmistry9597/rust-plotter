import React, { KeyboardEvent, MouseEvent, useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import * as THREE from 'three';
import { Vec3Fixed, Vec2Fixed, PolarCoord3D } from "../model/three-helpers";
import { PerspectiveCamera } from "@react-three/drei";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { BlckInfo, CylProp, PtProp } from "../model/backend-comm";
import { rerender_updated } from "./rendering";
import { AxesElem, Scaler } from "./elems";
import { ControlHandlers } from "../model/control-handlers";
import { clamp } from "../tools/clamp";

export function Chart3d(props: {
    setControlHandler: (handlers: ControlHandlers) => void,
    setInfoRef: React.MutableRefObject<(header: string, contents: string[]) => void>,
}) {
    const end = 40
    const [ptprops, setptprops] = useState([] as PtProp[])
    const [cyls, setcyls] = useState([] as CylProp[])
    const [camRef, setCamRef] = useState<React.MutableRefObject<THREE.PerspectiveCamera | null> | null>(null)

    const setCamRefHandle = useMemo(() => {
        return (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => setCamRef(camRef)
    }, [])

    const cam_displace = useRef(new THREE.Vector3(20, 20, 20))
    const cam_focus = useRef(0)

    const [trig, settrig] = useState(false)

    return (
        <>
            <Canvas dpr={[2,1]}>
                <CamControl 
                    setControlHandler={props.setControlHandler} 
                    cam_displace={cam_displace}
                    cam_focus={cam_focus}
                    ptprops={ptprops} />
                <BackendEndpoint
                        settrig={settrig} 
                        setptprops={setptprops} 
                        setcyls={setcyls}
                    />
                <ThreejsFix />
                <CamTrack 
                    camRef={camRef} 
                    x_target={cam_focus} 
                    displace={cam_displace.current} />
                <CamSetup setCamRef={setCamRefHandle} />
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
                        end={end}
                        setInfoRef={props.setInfoRef} />
                </Scaler>
            </Canvas>
        </>
    )
}

function CamControl(props: 
    {cam_displace: React.MutableRefObject<THREE.Vector3>, 
    setControlHandler: (handlers: ControlHandlers) => void,
    cam_focus: React.MutableRefObject<number>,
    ptprops: PtProp[],}
) {
    const cam_displace = props.cam_displace
    const dragRef = useRef(false)
    const prevMousePos = useRef<Vec2Fixed | null>(null)
    const cam_polar = useRef<PolarCoord3D>({rad: 20, polar: 1, alpha: 1})
    const [tracking, setTracking] = useState(true)
    if (tracking) {
        props.cam_focus.current = props.ptprops.at(-1)?.pos[0] || 0
    }
    useEffect(() => {
        const down = () => {dragRef.current = true}
        const up = () => {dragRef.current = false}
        const move = (e: MouseEvent) => {
            if (!dragRef.current) {
                return
            }
            const mousePos: Vec2Fixed = [e.movementX, e.movementY]
            const prevPos = prevMousePos.current || mousePos
            const deltX = -(mousePos[0] - prevPos[0])
            const deltY = -(mousePos[1] - prevPos[1])
            const scale = 0.008
            cam_polar.current.polar += deltY * scale
            cam_polar.current.alpha += deltX * scale
            const rad_cos = cam_polar.current.rad * Math.sin(cam_polar.current.polar)
            cam_displace.current.y = cam_polar.current.rad * Math.cos(cam_polar.current.polar)
            cam_displace.current.z = rad_cos * Math.cos(cam_polar.current.alpha)
            cam_displace.current.x = rad_cos * Math.sin(cam_polar.current.alpha)
            prevMousePos.current = mousePos
        }
        const keydown = (e: KeyboardEvent) => {
            const old_rad = cam_polar.current.rad

            if (e.key === "ArrowDown") {
                cam_polar.current.rad += 1
            }
            else if (e.key === "ArrowUp") {
                cam_polar.current.rad += -1
            }
            else if (e.key === "ArrowLeft") {
                setTracking(false)
                props.cam_focus.current += -1
            }
            else if (e.key === "ArrowRight") {
                setTracking(false)
                props.cam_focus.current += 1
            }
            else if (e.key === "t") {
                setTracking(true)
            }
            cam_polar.current.rad = clamp(cam_polar.current.rad, 1, Infinity)
            cam_displace.current.multiplyScalar(cam_polar.current.rad / old_rad)
        }
        props.setControlHandler({up, down, move, keydown})
    }, [])
    return <></>
}

function CamTrack(props: {
    camRef: React.MutableRefObject<THREE.PerspectiveCamera | null> | null,
    x_target: React.MutableRefObject<number>,
    displace: THREE.Vector3,
})
{
    const { camRef, x_target } = props
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
    const cam_displace = props.displace

    useFrame((_state, delta) => {
        const idealCam = (new THREE.Vector3(x_target.current, 0, 0)).add(cam_displace)
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

function ThreejsFix() {
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
            console.log("ready!")
        })
      }, [])

    return <></>
}

function CamSetup(props: {setCamRef: (camRef: React.MutableRefObject<THREE.PerspectiveCamera | null>) => void}) {
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

function Point(props: {ptprop_w_index: [PtProp, number], 
    rad: number, 
    setInfoRef: React.MutableRefObject<(header: string, contents: string[]) => void>,
    addColorSetter: (set: (col: string) => void) => void,
    resetColor: () => void,
}) {
    const {ptprop_w_index, rad, setInfoRef, addColorSetter, resetColor} = props
    const [ptprop, index] = ptprop_w_index
    const [color, setColor] = useState("rgb(10, 255, 180)")
    const onclick = () => {
        setInfoRef.current("Point Info", [`x: ${ptprop.rl_data.pos[0]}`, `y: ${ptprop.rl_data.pos[1]}`])
        resetColor()
        setColor("rgb(255, 0, 180)")
    }
    useEffect(() => {
        addColorSetter(setColor)
    }, [])
    return (
        <group position={[0,0,0]} key={index}>
            <mesh position={ptprop.pos} onClick={onclick}>
                <sphereGeometry args={[rad, 32,32,32]} />
                <meshBasicMaterial color={color}></meshBasicMaterial>
            </mesh>
        </group>
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
    end: number,
    setInfoRef: React.MutableRefObject<(header: string, contents: string[]) => void>,
}) 
{
    const {setInfoRef} = props
    const ptRender = useRef([] as JSX.Element[])
    const pt_props_hash = useRef([] as (string | Int32Array)[])
    const ptColorSetters = useRef<((col: string) => void)[]>([])
    const addPtColorSetter = useMemo(() => {
        return (colSetter: (col: string) => void) => {
            ptColorSetters.current.push(colSetter)
        }
    }, [])
    const resetPtColor = useMemo(() => {
        return () => {
            ptColorSetters.current.forEach((colSet) => {
                colSet("rgb(10, 255, 180)")
            })
        }
    }, [])
    const pt_gener = useMemo(() => (ptprop_w_index: [PtProp, number]) => {
        const [_, index] = ptprop_w_index
        return <Point 
                    ptprop_w_index={ptprop_w_index} 
                    rad={props.ptRad} 
                    setInfoRef={setInfoRef}
                    addColorSetter={addPtColorSetter}
                    resetColor={resetPtColor}
                    key={index} />
        }, 
        [])
    const cylRender = useRef([] as JSX.Element[])
    const cyl_props_hash = useRef([] as (string | Int32Array)[])
    const cyl_gener = useMemo(() => (cylprop_w_index: [CylProp, number]) => {
        const [cylprop, index] = cylprop_w_index
        const [q_r, q_i] = cylprop.quat
        const quat = new THREE.Quaternion(q_r, ...q_i)
        return (
            <group 
                position={cylprop.pos} 
                key={index}>
                <mesh quaternion={quat} castShadow>
                    <cylinderBufferGeometry 
                        args={[props.lineRad, props.lineRad, cylprop.len, 16]} />
                    <meshBasicMaterial color="rgb(10,200,200)"></meshBasicMaterial>
                </mesh>
            </group>
        )
    }, [])
    useEffect(() => {
        const ptprops = props.ptprops
        rerender_updated(ptprops, pt_props_hash, pt_gener, ptRender.current)
        const cylprops = props.cyls
        rerender_updated(cylprops, cyl_props_hash, cyl_gener, cylRender.current)

        props.settrig(false)
    }, [props.trig])

    return (
        <group>
            <group>{ptRender.current}</group>
            <group>{cylRender.current}</group>
        </group>
    )
}