import React from "react";
import { IconProps } from "../model/icon-props";
import { Icon } from "./icon";
import './pallete.css'

export function Pallete(props: {
    iconDescriptor: IconProps[],
    diam: string,
}) {
    const main_style: React.CSSProperties = {
        width: props.diam,
        height: props.diam
    }
    const icons = props.iconDescriptor.map((prop, i) => {
        // prop.marginAdjust = `-${(i) * 100}%`
        return Icon(prop)
    })
    const root = icons[0]

    return <>
        <div style={main_style} id='main'>
            {root}
            {icons.slice(1)}
        </div>
    </>
}