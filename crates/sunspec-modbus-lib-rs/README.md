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

## Feature flags
This library has a feature flag for each of the DER models defined in the SunSpec Modbus standard. By default only the Common 
model (model 1) is enabled, and any further models may be enabled as required.

## Usage
To expose a SunSpec interface, a device must define a list of SunSpec models that is constant at runtime. In `sunspec-modbus-lib-rs`
this is done by defining an implementation of the [ModelList] trait - most simply done by using the included `derive` macro.

```rust
use sunspec_modbus_lib_rs::{
    ModelList, Sunspec, SunspecConfig,
    sunspec::models::{model_1, model_103, model_708},
};

const CURVE_COUNT: u16 = 2;
const POINT_COUNT: u16 = 3;

#[derive(ModelList)]
struct SunspecModels {
    model_1: model_1::Model1,
    model_103: model_103::Model103,
    model_708: model_708::Model708,
}

/// The device's register map: the common model, an inverter model, and a DER high-voltage-trip
/// curve model. The same list backs both reads and writes.
const SUNSPEC: Sunspec<SunspecModels> = Sunspec::new(
    SunspecModels {
        model_1: model_1::Model1,
        model_103: model_103::Model103,
        model_708: model_708::Model708 {
            stored_curve_count: CURVE_COUNT,
            number_of_points: POINT_COUNT,
        },
    },
    SunspecConfig::DEFAULT,
);
```

This [ModelList] can be used to construct a `Sunspec` service, which exposes the required read and write operations.
The second argument is a [SunspecConfig] - every setting is optional, so pass `SunspecConfig::DEFAULT` for standard behaviour,
or override individual settings, e.g. `SunspecConfig::DEFAULT.with_base_address(50000)` to serve the map from register 50000
rather than the default 40000. Strict mode is on by default: any request that reaches outside the register map is rejected with
`IllegalDataAddress` before any adapter is called, as is any write to the read-only `SunS` identifier or end model. Use
`.with_strict(false)` to instead fill reads past the end of the map with `0xffff` and ignore those writes.
Read operations require the relevant ReadAdapter implementation to be provided for each model (e.g. [model_1::ReadAdapter](sunspec_modbus_lib_rs::sunspec::models::model_1::ReadAdapter)) . Similarly, write operations will require a WriteAdapter
that has at least one writable point (e.g. [model_1::WriteAdapter](sunspec_modbus_lib_rs::sunspec::models::model_1::WriteAdapter)).

The choice of Modbus transport and server library can be made independently from this library - for examples of how to integrate
the `Sunspec` service with either `rmodbus` or `tokio-modbus`, see the [repository examples](https://github.com/cuprous-au/sunspec-modbus-codec/tree/main/crates/sunspec-modbus-lib-rs/examples).