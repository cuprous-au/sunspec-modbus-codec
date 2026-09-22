#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

cd "$(dirname "${BASH_SOURCE[0]}")"
cargo build --release --features model_103,model_708
cc examples/libmodbus.c \
  -DSUNSPEC_MODEL_103_ENABLED \
  -DSUNSPEC_MODEL_708_ENABLED \
  -o ../../target/example-libmodbus.o \
  -lmodbus \
  -L../../target/release \
  -lsunspecmodbus \
  -pthread
../../target/example-libmodbus.o