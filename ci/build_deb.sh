#!/bin/bash

VERSION=$(./version.sh)
RELEASE_NAME="cloudwrap"

BF_PATH="/opt/blackfynn"
EXECUTABLE="$BF_PATH/bin/$RELEASE_NAME"

BUILD_DIR="target/x86_64-unknown-linux-gnu/release"

# Create the bin/ directory:
mkdir "$BUILD_DIR/bin"
mv "$BUILD_DIR/$RELEASE_NAME" "$BUILD_DIR/bin/$RELEASE_NAME"

fpm \
  -f \
  --verbose \
  -s dir \
  -t deb \
  -a x86_64 \
  --prefix="$BF_PATH" \
  --deb-no-default-config-files \
  -n "$RELEASE_NAME" \
  --after-install ci/build_deb_post_install.sh \
  -v $VERSION \
  --template-scripts \
  --template-value bf_path="$BF_PATH" \
  --template-value release_name="$RELEASE_NAME" \
  --template-value executable="$EXECUTABLE" \
  "$BUILD_DIR/=."
