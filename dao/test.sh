#!/bin/bash

export RUST_BACKTRACE=1
cargo test -- --nocapture --test-threads 1
