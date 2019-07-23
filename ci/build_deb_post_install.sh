#!/usr/bin/env bash

###############################################################################
# Note:
#
# This is an ERB template that is converted into a post-installation
# script run by `fpm`. It is to be used with the `fpm` arguments
# `--template-script` and `--template-value`.
#
# IT IS NOT MEANT TO BE RUN DIRECTLY!
#
# Expected variables:
#
#   - bf_path : string =>
#
#     The path to the Blackfynn installation, e.g. "/usr/local/opt/blackfynn",
#     "C:\Program Files\blackfynn", etc.
#
#   - release_name : string =>
#
#     The name of the binary itself ("cloudwrap")
#
#   - executable : string =>
#
#     The absolute path to the Blackfynn binary, e.g
#     /usr/local/opt/blackfynn/bin/${bf_release_name}
#
###############################################################################

BF_PATH="<%= bf_path %>"
EXECUTABLE="<%= executable %>"
RELEASE_NAME="<%= release_name %>"

# Set the appropriate permissions:
chmod 755 "$BF_PATH"

# Symlink $EXECUTABLE to /usr/local/bin:
if [ -d "/usr/local/bin" ]; then
  ln -s -f "$EXECUTABLE" "/usr/local/bin/$RELEASE_NAME"
fi
