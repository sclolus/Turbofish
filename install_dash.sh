#!/bin/bash
export TARGET="i686-turbofish"
export PATH="/toolchain_turbofish/cross/bin:$PATH"
export TARGET_DIR="../../../system_disk/bin/"

mkdir -pv build_dash
cd build_dash
wget -c 'http://gondor.apana.org.au/~herbert/dash/files/dash-0.5.10.tar.gz'
tar -xf 'dash-0.5.10.tar.gz'
cd dash-0.5.10
mkdir build
cd build
CFLAGS="-O3 -fno-omit-frame-pointer -Wl,--gc-sections" ../configure --build=`gcc -dumpmachine` --host=$TARGET
make
sudo cp -v src/dash $TARGET_DIR
sudo ln -s -v --force /bin/dash $TARGET_DIR/sh
