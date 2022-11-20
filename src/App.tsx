import React from 'react';
import './App.css';
import { Chart3d } from './chart-3d/chart-3d'
import { IconProps } from './model/icon-props';
import { Pallete } from './pallete/pallete';

function App() {
  const insights_icons: IconProps[] = [
    {src: "/icon/bulb.svg", diam: '35px'},
    {src: "/icon/wand.svg", diam: '35px', theta: -3.14 * 0.4, rad_f: 0.8},
    {src: "/icon/stats.svg", diam: '35px', theta: -3.14 * 0.1, rad_f: 0.8},
    {src: "/icon/fourier.svg", diam: '35px', theta: 3.14 * 0.2, rad_f: 0.8},
  ]

  return (
    <div className='main'>
      <div id="chart" className='full-fill'>
        <Chart3d></Chart3d>
      </div>
      <div id="insights">
        <Pallete 
          iconDescriptor={insights_icons}
          diam='150px' />
      </div>
    </div>
  );
}

export default App;
