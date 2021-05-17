#!/bin/sh

# required deps: 
# sudo apt install libjq1 libjq-dev libonig-dev gcc rustc

export JQ_LIB_DIR=/usr/lib/x86_64-linux-gnu/
cargo build --release
