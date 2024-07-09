#!/bin/bash

cargo build --release && espflash flash --monitor target/xtensa-esp32s3-espidf/release/watchy-rs
