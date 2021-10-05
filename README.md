# gravity-wasm

This is a simulation of gravity, using WebAssembly and rust as a the engine for driving particle position, velocity, and acceleration.

## Building

Get wasm-pack from https://rustwasm.github.io/wasm-pack/

After installing wasm-pack, run the following command inside this project's directory.

```sh
wasm-pack build
```

This will generate the WASM binary and the JavaScript modules, ready for import into your project.

## Exported Methods and Properties

```
Universe {
    positions: Array2<f64>
    velocities: Array2<f64>
    acceleration: Array2<f64>
    masses: Array1<f64>
    new: method that creates new universe
    tick: method that iterates the universe
    get_positions_ptr: returns pointer to memory containing current positions of particles
    get_masses_ptr: returns pointer to memory containing current masses of particles
}
```
