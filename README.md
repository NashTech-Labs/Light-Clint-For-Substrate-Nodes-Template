# Light-Clint-For-Substrate-Nodes-Template

This templeate enables user to connect to Blockchain node.   

## Building

### Install Rust


```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
### Install Substrate
```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

## Compiling Template

1. Clone the Template
```bash
$ https://github.com/knoldus/Light-Clint-For-Substrate-Nodes-Template.git
```
3. Clone the Substrate based PoE Application
```bash
$ git clone https://github.com/knoldus/Substrate-Node-Template.git
``` 
2. Initialize your WebAssembly build environment
```bash
source ~/.cargo/env

rustup update nightly
rustup update stable

rustup target add wasm32-unknown-unknown --toolchain nightly
```
3. Compile and run the PoE Application
```bash
cd Substrate-Node-Template/
cargo build --release
./target/release/node-template --dev --tmp
```
4. Compile andrun the Sibstrate Client
```bash
cd Substrate-Light-Clint-For-Substrate-Nodes-Template/
cargo build --release
cargo run
```
