import React, { useEffect, useRef, useState } from "react";
import { IconProps } from "../model/icon-props";

export function Icon(props: IconProps) {
    const sec_trans = 0.5
    const rad_total_arr = props.rad_total?.split("")
    const digs = rad_total_arr?.filter((c) => {
        return (c <= "9") && (c >= "0")
    })
    const unit_arr = rad_total_arr?.slice(digs?.length)
    const unit = unit_arr?.join("")
    const total_quant = Number(digs?.join(""))
    const rad = total_quant * (props.rad_f || 0)

    const x_val = (rad * Math.cos(props.theta || Math.PI / 2)).toString() + (unit || "")
    const y_val = (rad * Math.sin(props.theta || 0)).toString() + (unit || "")
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

    const activated = useRef(false)
    if (activated.current) {
        style.boxShadow = "0px 0px 20px 10px #0ff"
    }
    const triggerEvent = () => {
        activated.current = !activated.current
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