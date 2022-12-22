import { KeyboardEvent, KeyboardEventHandler, MouseEvent, MouseEventHandler } from "react";

export interface ControlHandlers {
    down: MouseEventHandler,
    up: MouseEventHandler,
    move: MouseEventHandler,
    keydown: KeyboardEventHandler,
}

export function emptyControlHandlers(): ControlHandlers {
    const mhandler = (e: MouseEvent) => {}
    const khandler = (e: KeyboardEvent) => {}
    return {up: mhandler, down: mhandler, move: mhandler, keydown: khandler}
}