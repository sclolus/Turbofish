#!/bin/bash
export TARGET="i686-turbofish"

mkdir -pv build_coreutils
cp patch-coreutils build_coreutils
cd build_coreutils
wget -c 'https://ftp.gnu.org/gnu/coreutils/coreutils-5.0.tar.bz2'
tar -xf 'coreutils-5.0.tar.bz2'
patch -p0 < patch-coreutils
cd coreutils-5.0
mkdir build
cd build
../configure --host=$TARGET
make
