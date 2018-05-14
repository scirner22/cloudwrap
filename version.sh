#!/bin/bash

cat Cargo.toml | grep version | head -n 1 | sed -e "s/version = //g" | sed -e "s/\"//g"
