import React from 'react';
import './App.css';
import { Chart3d } from './chart-3d/chart-3d'
import { IconProps } from './model/icon-props';
import { Icon } from './pallete/icon';
import { Pallete } from './pallete/pallete';

function App() {
  const icons: IconProps[] = [
    {src: "/icon/bulb.svg", diam: '30px'},
    {src: "/icon/wand.svg", diam: '35px', rad_total: '15em', theta: 0.9, rad_f: 0.5},
    {src: "/icon/wand.svg", diam: '35px', rad_total: '15em', theta: 0.45, rad_f: 0.5},
    {src: "/icon/bulb.svg", diam: '35px', rad_total: '15em', theta: -0.9, rad_f: 0.5},
    {src: "/icon/wand.svg", diam: '35px', rad_total: '15em', theta: 2.2, rad_f: 0.8},
  ]

  return (
    <div className='main'>
      <div id="chart" className='full-fill'>
        <Chart3d></Chart3d>
      </div>
      <div id="testpallete">
        <Pallete 
          iconDescriptor={icons}
          diam='300px' />
      </div>
    </div>
  );
}

export default App;
