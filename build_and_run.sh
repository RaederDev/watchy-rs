#!/bin/bash

cargo build --release && espflash flash --monitor target/xtensa-esp32s3-none-elf/release/watchy-rs
