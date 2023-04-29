#!/bin/bash

RUSTCC=$(dirname "$0")"/target/debug/rustcc"

CODE_FILENAME=$(basename "$1" .c)
CODE_PATH=$(dirname "$1")

RUST_BACKTRACE=1 $RUSTCC "$1" && gcc "${CODE_PATH}/${CODE_FILENAME}.s" -o "${CODE_PATH}/${CODE_FILENAME}"
