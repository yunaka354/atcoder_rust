#!/bin/bash
cargo generate --path ./atcoder_template --name $1
code $1
code $1/src/main.rs