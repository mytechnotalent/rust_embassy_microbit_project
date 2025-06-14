#!/bin/bash

probe-rs gdb \
    target/thumbv7em-none-eabihf/debug/microbit-async-display-example \
    --chip nRF52833_xxAA --speed 1000
