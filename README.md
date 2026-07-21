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

## C bindings generation

This crate auto-generates a C header during build using `cbindgen`.

- Config file: `cbindgen.toml`
- Generated header: `sunspec_modbus_codec.h`
- Trigger: any change under `src/` or `cbindgen.toml`

To regenerate manually without building:

```bash
cbindgen --config cbindgen.toml --crate sunspec-modbus-codec --output sunspec_modbus_codec.h
```

The current work configures deterministic C header generation with a no_std-friendly dependency surface.
It does not by itself make all exported Rust types fully C ABI-safe.
