# Pokemon Battle Simulator

Currently only supports generation 1

## How to Run

Install required stuff first :)

Run
```
wasm-pack build wasm --target web --out-dir ../pkg
```
to compile the WebAssembly

Run 
```
cargo run -- --mode stat --input input/staryu.json
``` 
to run the command line application