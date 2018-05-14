#!/bin/bash

VERSION=0.0.1

fpm -s dir -t apk -n cloudwrap -v ${VERSION} -a x86_64 -p target/debug/ target/debug/cloudwrap=/usr/bin
#aws s3 cp target/debug/cloudwrap_${VERSION}_x86_64.apk s3://data.blackfynn.io/public-downloads/cloudwrap/
