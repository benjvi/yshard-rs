#!/bin/sh

# xcode C environment has been seen to have incompatibilities with jq code, need to take care with that combination
# required deps:
# brew install autoconf automake jq

export JQ_LIB_DIR=/usr/local/Cellar/jq/1.6/lib
cargo build --release
