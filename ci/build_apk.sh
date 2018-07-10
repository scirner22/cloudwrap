#!/bin/bash

VERSION=$(./version.sh)

fpm -s dir -t apk -n cloudwrap -v ${VERSION} -a x86_64 \
	--chdir target/x86_64-unknown-linux-musl/release \
	--prefix /usr/bin cloudwrap
