#!/bin/bash

VERSION=$(./version.sh)
ARCH=$(./arch_triple.sh)
BUILD=cloudwrap_${VERSION}_${ARCH}.tar.gz

tar -cvzf ${BUILD} target/release/cloudwrap
