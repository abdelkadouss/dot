#!/bin/bash

docker run -it --rm \
-v $PWD/src:/app/src \
-v $PWD/Cargo.toml:/app/Cargo.toml \
-v $PWD/Cargo.lock:/app/Cargo.lock \
-v $PWD/example:/app/example \
dot:latest;
