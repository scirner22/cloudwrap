#!/bin/bash

rustc -Vv | grep host: | sed -e "s/host: //g"
