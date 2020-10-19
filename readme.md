# Example for fk-lis3dsh accelerometer-driver crate
Made for [STM32F4-discovery](https://www.st.com/en/evaluation-tools/stm32f4discovery.html) board

Based on [emb-rust](https://github.com/arthurggordon/emb-rust) by [Arthur Gordon](https://github.com/arthurggordon) modified to run with [probe-run](https://github.com/knurling-rs/probe-run) 

Simpler demo for only probe-run: https://github.com/straend/emb-rust

Driver crate: https://crates.io/crates/fk-lis3dsh
### Usage
It's `clone 'n run` (If You have [probe-run](https://github.com/knurling-rs/probe-run) installed)
```
git clone https://github.com/straend/fk-lis3dsh-example
cd fk-lis3dsh-example
cargo run
```
Read output in terminal and notice that the onboard leds light up differently depending on how the board is rotated.