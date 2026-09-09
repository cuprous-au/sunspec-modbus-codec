#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

cd "$(dirname "${BASH_SOURCE[0]}")"
cargo build --release
cc examples/libmodbus.c -o ../../target/example-libmodbus.o -lmodbus -L../../target/release -lsunspec_modbus_lib_static -lpthread
../../target/example-libmodbus.o