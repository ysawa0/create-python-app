#!/bin/bash

cargo install --path .
cpa create --name example/python --preset python3.10
cpa create --name example/rust --preset rust
cpa create --name example/base --preset base
