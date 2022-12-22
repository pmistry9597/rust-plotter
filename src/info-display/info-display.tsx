import React, { useEffect, useRef } from "react";
import './info-display.css'

export function InfoDisplay(props: {setInfoSetter: (setInfo: (header: string, contents: string[]) => void) => void}) {
    const headerRef = useRef("Nothing yet!")
    const contentsRef = useRef(["same here"])
    useEffect(() => {
        const setInfo = (header: string, contents: string[]) => {
            headerRef.current = header
            contentsRef.current = contents
        }
        props.setInfoSetter(setInfo)
    }, [props.setInfoSetter])

    const contents = contentsRef.current.map((content, index) => {
        return <li className="entry" key={index}>{content}</li>
    })
    return (
        <div id="main">
            <div className="header">{headerRef.current}</div>
            <ul className="contents">
                {contents}
            </ul>
        </div>
    )
}