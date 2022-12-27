# Rust-plotter

An app that was *originally* intended for waveform analysis and visualization. Actually, this was meant to be a project to get me deeper into rust. Well, I think it succeeded at that one. Now I am just going to brand this as an attempt to do 3D plotting. Powered by [Tauri](https://github.com/tauri-apps) (in Rust + JS). JS UI uses React with [@react-three/fiber](https://github.com/pmndrs/react-three-fiber).

Gonna move on to projects I find more interesting :P.

## What I managed to do

1D plotting a sine wave with noise\
![Sine with noise](/images/sine_noise.png)
3D plot of decaying sinusoid\
![Decaying expo 3d](/images/decaying_expo.png)
3D surface plotting attempt (around the time I gave up, ofcourse)\
![Mesh attempt](/images/mesh_attempt.png)

## Why this thing failed
This was meant to be something smaller so I can move on to other things. Yet, I kept thinking of new features that may be useful to have. Got to love scope creep, eh?

In addition, a major design flaw I made was choosing to use the Tauri frontend to render the actual display. I should have switched to a different option once I realized Tauri could not stream a render from rust directly. This meant I had to render within the browser client provided by Tauri (*vomit*).

For instance, I could have used Tauri just for the UI, and then have used another library in parallel for actually rendering to a window.

## Build / Running

For a dev run, use the following commands in separate terminals, assuming you've installed Tauri with ```cargo```.

```
npm run start
```
```
cargo tauri dev
```

For an optimized binary, run the following one *after* the other.

```
npm run build
cargo tauri build
```