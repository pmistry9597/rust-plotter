import React, { useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import { Chart3d } from './chart-3d/chart-3d'

import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

function App() {
  useEffect(() => {
    listen("fake_blck", (event: any) => {
      const payload = event.payload
      invoke("get_blck_fake", { i: payload.index, }).then((val) => {
        console.log(`in cumming:`)
        console.log(val)
      })
    })
  }, [])

  return (
    <div className='main'>
      <Chart3d></Chart3d>
    </div>
  );
}

export default App;
