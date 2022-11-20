import React, { useState } from "react";
import { IconProps } from "../model/icon-props";
import { get_quant_unit } from "../tools/get_quant_unit";
import { Icon } from "./icon";
import './pallete.css'

export function Pallete(props: {
    iconDescriptor: IconProps[],
    diam: string,
}) {
    const sec_trans = 0.4
    const main_style: React.CSSProperties = {
        transition: `width ${sec_trans}s, height ${sec_trans}s`
    }
    const [expanded, setExpanded] = useState(false)
    const iconPads = props.iconDescriptor.map((prop, i) => {
        const style: React.CSSProperties = {
            width: "100%",
            height: "100%",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            zIndex: 1,
            gridRowStart: 1,
            gridColumnEnd: 1,
        }
        prop.displayed = expanded
        prop.sec_trans = sec_trans
        prop.activColor = "orange"
        const [rad, unit] = get_quant_unit(props.diam)
        prop.rad_total = (rad * 0.5).toString() + unit
        handleRootCase(style, prop, i, expanded, setExpanded, sec_trans)

        return <div key={i} style={style}>{Icon(prop)}</div>
    })

    return <>
        <div style={main_style} className="pallete">
            {iconPads}
        </div>
    </>
}

function handleRootCase(style: React.CSSProperties, prop: IconProps, i: number, expanded: boolean, setExpanded: any, sec_trans: number) {
    if (i == 0) {
        style.transition = `transform ${sec_trans}s`
        if (expanded) {
            style.transform = "rotate(90deg)"
        }
        style.zIndex = 4
        prop.displayed = true
        prop.activColor = "#0ff"
        prop.triggerEvent = () => setExpanded(!expanded)
    }
}