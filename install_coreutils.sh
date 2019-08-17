#!/bin/bash
export TARGET="i686-turbofish"
export PATH="/toolchain_turbofish/cross/bin:$PATH"
export TARGET_DIR="../../../rust_kernel/src/userland"

mkdir -pv build_coreutils
cp patch-coreutils build_coreutils
cd build_coreutils
wget -c 'https://ftp.gnu.org/gnu/coreutils/coreutils-5.0.tar.bz2'
tar -xf 'coreutils-5.0.tar.bz2'
patch -p0 < patch-coreutils
cd coreutils-5.0
cp ../../patch-coreutils-configure .
patch configure < patch-coreutils-configure
mkdir build
cd build
CFLAGS="-g -O0 -fno-omit-frame-pointer" ../configure --host=$TARGET
cp ../../../patch-coreutils-config-h .
patch config.h < patch-coreutils-config-h
make
make -C src cat
make -C src echo
make -C src kill
make -C src yes
cp -v src/echo $TARGET_DIR
cp -v src/cat $TARGET_DIR
cp -v src/kill $TARGET_DIR
cp -v src/yes $TARGET_DIR
