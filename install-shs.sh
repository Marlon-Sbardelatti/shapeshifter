#!/bin/bash

cd shapeshifter
cargo build --release
sudo cp target/release/shapeshifter /usr/local/bin

 
