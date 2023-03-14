# ferrino

Ferrino is an Arduino-like framework for writing embedded applications that makes use of the Rust
type system and build system to build embedded applications.

Ferrino allows you to:

* Write a single application targeting multiple physical boards
* Depend on hardware capabilities instead of the hardware
* Use a single cli for building, flashing, running and debugging firmware
* Depend on a single Rust crate

Ferrino wires the board you have selected for your application, if it supports the required
capabilities of the application. If not, you get a compile time error.

Ferrino uses [Embassy](https://embassy.dev) as the underlying runtime and for the hardware support.

## Usage

To use ferrino, install the `ferrino` command line tool:

```shell
cargo install ferrino
```

NOTE: You can also use tools like `probe-run` with ferrino. The `ferrino` tool includes `probe-rs` just like `probe-run` but provides a simpler out-of-the-box experience.

## Listing supported boards

```bash
ferrino --list-boards
```

## Running an example

```bash
cd examples/blinky
ferrino run --board microbit
```
