import React, { useEffect, useState } from "react";
import './info-display.css'

export function InfoDisplay(props: {setInfoSetter: (setInfo: (header: string, contents: string[]) => void) => void}) {
    const [header, setHeader] = useState("Nothing yet!")
    const [contents, setContents] = useState(["same here"])
    const [update, setUpdate] = useState(false)
    useEffect(() => {
        const setInfo = (header: string, contents: string[]) => {
            setHeader(header)
            setContents(contents)
        }
        setUpdate(true)
        props.setInfoSetter(setInfo)
    }, [props.setInfoSetter])

    const contents_render = contents.map((content, index) => {
        return <li className="entry" key={index}>{content}</li>
    })
    if (update) {
        setUpdate(false)
    }
    return (
        <div id="main">
            <div className="header">{header}</div>
            <ul className="contents">
                {contents_render}
            </ul>
        </div>
    )
}