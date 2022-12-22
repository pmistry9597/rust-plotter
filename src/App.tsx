import React, { useMemo, useRef, useState } from 'react';
import './App.css';
import { Chart3d } from './chart-3d/chart-3d'
import { InfoDisplay } from './info-display/info-display';
import { ControlHandlers, emptyControlHandlers } from './model/control-handlers';
import { IconProps } from './model/icon-props';
import { Pallete } from './pallete/pallete';

function App() {
  const icon_rad = 0.8
  const icon_diam = '35px'
  const insights_icons: IconProps[] = [
    {src: "/icon/bulb.svg", diam: icon_diam},
    {src: "/icon/wand.svg", diam: icon_diam, theta: Math.PI * -0.4, rad_f: icon_rad},
    {src: "/icon/stats.svg", diam: icon_diam, theta: Math.PI * 0.01, rad_f: icon_rad},
    {src: "/icon/fourier.svg", diam: icon_diam, theta: Math.PI * 0.4, rad_f: icon_rad},
  ]
  const view_icons: IconProps[] = [
    {src: "/icon/mag.svg", diam: icon_diam},
    {src: "/icon/arrow-right.svg", diam: icon_diam, theta: Math.PI * -0.9, rad_f: icon_rad},
    {src: "/icon/chevron-down.svg", diam: icon_diam, theta: Math.PI * -0.5, rad_f: icon_rad},
    {src: "/icon/cube.svg", diam: icon_diam, theta: Math.PI * -0.1, rad_f: icon_rad},
  ]
  const signal_icons: IconProps[] = [
    {src: "/icon/pulse.svg", diam: icon_diam},
    {src: "/icon/files.svg", diam: icon_diam, theta: Math.PI * -0.8, rad_f: icon_rad},
    {src: "/icon/cycle.svg", diam: icon_diam, theta: Math.PI * -1.2, rad_f: icon_rad},
  ]
  const [chartControlHandler, setControlHandler] = useState<ControlHandlers>(emptyControlHandlers)
  const setInfoRef = useRef((header: string, contents: string[]) => {})
  const setInfoSetter = useMemo<(setInfo: (header: string, contents: string[]) => void) => void>(() => {
    return (setInfo: (header: string, contents: string[]) => void) => {
      setInfoRef.current = setInfo
    }
  }, [])

  return (
    <div className='main'>
      <div id="chart" className='full-fill' 
        onMouseDown={chartControlHandler.down} 
        onMouseUp={chartControlHandler.up} 
        onMouseMove={chartControlHandler.move}
        onKeyDown={chartControlHandler.keydown} 
        tabIndex={0}
        >
        <Chart3d 
          setControlHandler={setControlHandler}
          setInfo={setInfoRef.current} />
      </div>
      <div id="display">
        <InfoDisplay setInfoSetter={setInfoSetter} />
      </div>
      <div id="insights">
        <Pallete 
          iconDescriptor={insights_icons}
          diam='150px' />
      </div>
      <div id="view">
        <Pallete 
          iconDescriptor={view_icons}
          diam='150px' />
      </div>
      <div id="signal">
        <Pallete 
          iconDescriptor={signal_icons}
          diam='150px' />
      </div>
    </div>
  );
}

export default App;
