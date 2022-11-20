import React, { useRef, useState } from "react";
import { IconProps } from "../model/icon-props";
import { get_quant_unit } from "../tools/get_quant_unit";

export function Icon(props: IconProps) {
    const sec_trans = props.sec_trans || 0.5
    const [rad, unit] = get_quant_unit(props.rad_total || "")

    const x_val = (rad * (props.rad_f || 0) * Math.cos(props.theta || Math.PI / 2)).toString() + (unit || "")
    const y_val = (rad * (props.rad_f || 0) * Math.sin(props.theta || 0)).toString() + (unit || "")
    const style: React.CSSProperties = {
        width: props.diam,
        height: props.diam,
        border: "solid #fff 2px",
        padding: "10px",
        borderRadius: "50%",
        zIndex: 4,
        cursor: "pointer",
        transition: `transform ${sec_trans}s, box-shadow ${sec_trans}s`,
    }
    const [doneContract, setDoneContract] = useState(true)
    const [contractTimer, setContractTimer]: [NodeJS.Timeout | null, any] = useState(null)
    handleExpansion(props.displayed, style, x_val, y_val, doneContract, setDoneContract, contractTimer, setContractTimer, sec_trans)

    const [activated, setActivated] = useState(false)
    const activatedRef = useRef(false)
    if (activated) {
        style.boxShadow = `0px 0px 20px 10px ${props.activColor || "#0ff"}`
    }
    const triggerEvent = () => {
        activatedRef.current = !activatedRef.current
        setActivated(activatedRef.current)
        props.triggerEvent?.()
    }
 
    return <>
        <div style={style} onClick={triggerEvent}>
            <img src={props.src}></img>
        </div>
    </>
}

function handleExpansion(
    displayed: boolean | undefined, 
    style: React.CSSProperties, 
    x_val: string, y_val: string,
    doneContract: boolean, setDoneContract: any, 
    contractTimer: NodeJS.Timeout | null, setContractTimer: any, 
    sec_trans: number,
) {
    if (displayed) {
        style.transform = `translate(${x_val}, ${y_val})`
        style.visibility = "visible"

        if (doneContract) {
            setDoneContract(false)
        }
        if (!!contractTimer) {
            clearTimeout(contractTimer)
            setContractTimer(null)
        }
    } else {
        if (doneContract) {
            style.visibility = "hidden"
        } else {
            if (!contractTimer) {
                const newContractTimer = setTimeout(() => {
                    setDoneContract(true)
                    setContractTimer(null)
                }, sec_trans * 1000)
                setContractTimer(newContractTimer)
            }
        }
    }
}