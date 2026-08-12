# sunspec-modbus-codec
A Rust-first SunSpec Modbus codec with C bindings for fast, reliable serialisation and deserialisation of wire-compatible models.

The objective is to provide a robust, open-source Sunspec MODBUS Server Library for resource-constrained microcontroller firmware,
accelerating vendor compliance by standardising the physical device interface.

The Sunspec MODBUS Server Library is known as a "bare-metal" Rust library with a C ABI for Sunspec MODBUS compliance in microcontrollers.
Bare-metal approaches are those that do not have dependencies on their hosts insofar as shared libraries and operating
systems. Device interactions are assumed to be direct with the hardware e.g. GPIO interactions, PWM control and so forth.
When the Sunspec MODBUS Server Library is used, it will be used either directly,
called from Rust, or via a C ABI provided here. The end result is that any language that is able to call upon a C
library is able to call on this codec library.

Note that we make a distinction between a server and a client library. In MODBUS, a client is a single entity that communicates
with many servers (devices, such as an inverter). Each server only responds when directly addressed by the client. This architecture evolved out of the
necessity to support half-duplex serial communications such as that imposed by RS485. There are a number of Sunspec MODBUS clients
available in a variety of languages, including reference implementations from Sunspec@python-sunspec. However, there are
very few hardened libraries for Sunspec MODBUS servers targeting microcontrollers at a minimum, hence the need for this project to deliver one.

As MODBUS itself is be delivered independently of its transport, this library retains this independence. For example,
a developer can choose to use the C-based libmodbus, or Rust-based MODBUS library, for use with an operating system such as Linux or Windows.
However, many devices have no operating system and quite often in support of serial comms over RS485 only.
In this instance, the library could be used along with implementations of Rust's embedded-hal IO library.

## Crates
# sunspec-gen
This crate is responsible for generating the content of the `src/sunspec` directory of the `sunspec-modbus-lib-rs` crate
described below. It sources the latest Sunspec MOBDBUS model definitions from https://github.com/sunspec/models, and generates
adapter definitions for each model.

> For development purposes of this crate, the codegen step can be triggered in isolation by running:
> ```sh
> cargo run -p sunspec-gen
> ```

# sunspec-modbus-lib-rs
This is the core rust crate that provides serialisation and deserialisation of Sunspec MODBUS models from a collection of
adapters.

# sunspec-modbus-lib-static
This wraps a subset of the `sunspec-modbus-lib-rs` crate in a stable FFI-safe interface to allow its usage from C.

First, declare a path to where libmodbus lives e.g.
```sh
LIBMODBUS_PREFIX=/opt/homebrew/opt/libmodbus
```

An example can be compiled and executed using the following steps:
```sh
cargo build --release -p sunspec-modbus-lib-static --no-default-features
cc crates/sunspec-modbus-lib-static/examples/libmodbus.c \
  -I"$LIBMODBUS_PREFIX/include" \
  -L"$LIBMODBUS_PREFIX/lib" \
  -o ./target/example-libmodbus.o \
  -lmodbus \
  -L./target/release \
  -lsunspec_modbus_lib_static
./target/example-libmodbus.o
```
