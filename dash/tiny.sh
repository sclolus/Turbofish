#!/bin/bash

echo $PATH
cd dash-0.5.10
mkdir build
cd build
CFLAGS="-g -O0 -fno-omit-frame-pointer" ../configure --host=$TARGET
make
