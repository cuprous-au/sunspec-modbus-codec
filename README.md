# sunspec-modbus-codec
A SunSpec Modbus codec written in Rust with C bindings for fast, reliable serialisation and deserialisation of wire-compatible models.

The objective is to provide a robust, open-source SunSpec MODBUS Server Library for resource-constrained microcontroller firmware,
accelerating vendor compliance by standardising the physical device interface.

The SunSpec MODBUS Server Library is a "bare-metal" Rust library with a C ABI for SunSpec MODBUS compliance in microcontrollers.
Bare-metal approaches are those that do not depend on host shared libraries or operating systems.
Device interactions are assumed to be direct with the hardware, e.g. GPIO interactions, PWM control and so forth.
When the SunSpec MODBUS Server Library is used, it will be used either directly,
called from Rust, or via a C ABI provided here. The end result is that any language that is able to call upon a C
library can call this codec library.

Note that we make a distinction between a server and a client library. In MODBUS, a client is a single entity that communicates
with many servers (devices, such as an inverter or a gateway controlling other devices). Each server only responds when directly addressed by the client. This architecture evolved out of the
necessity to support half-duplex serial communications such as that imposed by RS485. There are a number of SunSpec MODBUS clients
available in a variety of languages, including reference implementations such as SunSpec@python-sunspec. However, there are
very few hardened libraries for SunSpec MODBUS servers targeting microcontrollers, hence the need for this project to deliver one.

As MODBUS itself is specified independently of its transport, this library retains this independence. For example,
a developer can choose to use the C-based libmodbus or a Rust-based MODBUS library with an operating system such as Linux or Windows.
However, many devices have no operating system and often support only serial communications over RS485.
In this instance, the library could be used along with implementations of Rust's embedded-hal I/O library.

## Data model

### Model specification
A static description of a single specific SunSpec model, including the counts for any repeating groups. This must be static at runtime
and is used identically for both reads and writes. Critically, this information is sufficient to fully calculate the register layout
of the model and its points.

In Rust-facing interfaces, this is represented by an implementation of the ModelSpec trait.

### Model list
A model list is an ordered collection of [Model specification](#model-specification) objects, representing the full set
of models exposed by a Modbus server.

In Rust-facing interfaces, a ModelList implementation can be defined for any arbitrary data structure, and it must expose 
an iterator over each of its included models bound to an appropriate adapter for each of the read and write cases.

### Read/write adapter
An implementation that can provide a value or handle an update for each point in a specific model. To handle
an operation for a given model list, one such adapter must be provided for each specification as defined above.

These are provided as traits, and a Rust consumer of the library can implement them in any way they choose. For convenience and compatibility
with the interface for C or other programming languages, two standard implementations of the adapters are provided:
 - The Stateful adapter holds a representation of each point in a struct, and directly gets and sets these values according
   to requests
 - The Callback adapter holds function pointers for each method that are directly invoked, allowing more complex custom
   behaviour in non-Rust languages

### Adapter binding
An adapter binding represents the pairing of a model specification with an adapter implementation. This is represented by the
ReadBinding and WriteBinding enumerations. These are explicitly paired to ensure that the register layout is static, and 
consistent between read and write.

For Rust-facing interfaces, a binding is expected to be represented by the variant specific to the model being represented.
For other languages, there is a single Extern variant which allows the adapter to be referenced by void pointer. This variant
includes the other parameters required to safely cast to the specific implementation and map through to the correct point
traversal methods.

## Crates
### sunspec-gen
This crate is responsible for generating the content of the `src/sunspec` directory of the `sunspec-modbus-lib-rs` crate
described below. It sources the latest SunSpec MODBUS model definitions from https://github.com/sunspec/models, and generates
adapter definitions for each model.

> For development purposes of this crate, the codegen step can be triggered in isolation by running:
> ```sh
> cargo run -p sunspec-gen
> ```

### sunspec-modbus-lib-rs
This is the core Rust crate that provides serialisation and deserialisation of SunSpec MODBUS models from a collection of
adapters.

An example can be compiled and executed using the following steps (you will need a Sunspec MODBUS client to drive it):
```sh
cargo run --example tokio-modbus
```

### sunspec-modbus-lib-static
This wraps a subset of the `sunspec-modbus-lib-rs` crate in a stable FFI-safe interface to allow its use from C.

First, declare a path to where libmodbus lives, e.g.:
```sh
LIBMODBUS_PREFIX=/opt/homebrew/opt/libmodbus
```

An example can be compiled and executed using the following steps (you will need a Sunspec MODBUS client to drive it):
```sh
cargo build --release -p sunspec-modbus-lib-static --no-default-features
cc crates/sunspec-modbus-lib-static/examples/libmodbus.c \
  -I"$LIBMODBUS_PREFIX/include" \
  -L"$LIBMODBUS_PREFIX/lib" \
  -o ./target/example-libmodbus.o \
  -lmodbus \
  -L./target/release \
  -lsunspec_modbus_lib_static \
  -pthread
./target/example-libmodbus.o
```

## Contribution policy

Contributions via GitHub pull requests are gladly accepted from their original author. Along with any pull requests, please state that the contribution is your original work and that you license the work to the project under the project's open source license. Whether or not you state this explicitly, by submitting any copyrighted material via pull request, email, or other means you agree to license the material under the project's open source license and warrant that you have the legal authority to do so.

## License

This code is open source software licensed under the [Apache-2.0 license](./LICENSE).

An AR-PST deliverable
##

This repository is a deliverable of a research grant with the Australian Research in Power Systems Transition project
([AR-PST](https://www.csiro.au/en/research/technology-space/energy/electricity-transition/ar-pst));
specifically Stage 6 which is focused on "behind-the-meter" solutions at residential and commercial sites.
The AR-PST is a multi-year program to identify strategies for transitioning Australia to a stable, secure, and affordable electricity grid.

&copy; Copyright 2026 Commonwealth Scientific and Industrial Research Organisation (CSIRO)
