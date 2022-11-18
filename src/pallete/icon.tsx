import React from "react";
import { IconProps } from "../model/icon-props";

export function Icon(props: IconProps) {
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
        gridColumn: 1,
        gridRow: 1,
        transform: `translate( ${x_val}, ${y_val} )`,
        border: "solid #fff 2px",
        padding: "10px",
        borderRadius: "50%",
        position: "absolute",
        marginLeft: props.marginAdjust,
    }
 
    return <>
        <div id="root" style={style}>
            <img src={props.src}></img>
        </div>
    </>
}