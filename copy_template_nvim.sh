#!/bin/bash
cargo generate --path ./atcoder_template --name $1
cd $1
nvim
