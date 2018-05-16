#!/bin/bash

VERSION=$(./version.sh)

fpm -s dir -t apk -n cloudwrap -v ${VERSION} -a x86_64 target/x86_64-unknown-linux-musl/release/cloudwrap=/usr/bin
aws s3 cp cloudwrap_${VERSION}_x86_64.apk s3://data.blackfynn.io/public-downloads/cloudwrap/
