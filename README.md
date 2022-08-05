# Rustilloscope

An app intended for waveform and music analysis and visualization. Powered by [Tauri](https://github.com/tauri-apps) (in Rust + JS).

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

## Plans

- [ ] UI/Data visualizer
    - [ ] basic components/UX
    - [ ] charting
- [ ] signal sources
    - [ ] file formats
    - [ ] audio devices
- [ ] fourier analysis
    - [ ] frequency domain over an interval
    - [ ] freqencies present in sliding chunk

### Future things?

- pitch detection (seems kinda hard eh?)