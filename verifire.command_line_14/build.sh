#!/bin/bash

git submodule update --init --recursive
git pull
cd command_line
cargo clean
cargo update
cargo build --release