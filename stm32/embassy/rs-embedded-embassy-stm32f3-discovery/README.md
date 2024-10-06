# rs-embedded-embassy-stm32f3-discovery

This project serves as an introduction to embedded programming with Rust on the STM32F3-Discovery board using the asynchronous `Embassy` framework. It includes examples for configuring peripherals, GPIO, and communication, all while leveraging `no_std` for a resource-efficient environment.

## Features

- Asynchronous embedded programming with `Embassy`.
- Support for the STM32F3-Discovery board.
- Utilization of `no_std` for optimized resource usage.
- Examples demonstrating various peripherals and functionalities.

## Requirements

- Rust `nightly` toolchain
- `probe-rs` for flashing the board
- STM32F3-Discovery board

## Setup & Usage

```bash
git clone https://github.com/vagotec/rs-embedded
cd stm32/rs-embedded-embassy-stm32f3-discovery
# Build the project for a specific binary (e.g., blinky):
cargo build --bin blinky --release
# Run the project for a specific binary (e.g., blinky):
cargo run --bin blinky --release
# Clean the build artifacts:
cargo clean
