#!/bin/bash
export TARGET="i686-turbofish"
export ROOT_TOOLCHAIN="/toolchain_turbofish"
export TOOLCHAIN_SYSROOT="$ROOT_TOOLCHAIN/sysroot"
export CROSS="$ROOT_TOOLCHAIN/cross"
export LIBC_DIR="libc"
export HOST_TRIPLET="`gcc -dumpmachine`"
export PATH="$CROSS/bin:$PATH"

mkdir -pv build_gcc
cp patch-binutils patch-gcc build_gcc
pushd build_gcc
wget -c 'https://ftp.gnu.org/gnu/binutils/binutils-2.32.tar.xz'
tar -xf 'binutils-2.32.tar.xz'
patch -p0 < patch-binutils
pushd 'binutils-2.32'
# In LD subdirectory (Maybe install automake 1.15.1)
pushd ld
automake-1.15
popd
# Create a build directory in binutils
mkdir -p build
pushd build
../configure --build=$HOST_TRIPLET --host=$TARGET --target=$TARGET --prefix=$TOOLCHAIN_SYSROOT --with-sysroot=$TOOLCHAIN_SYSROOT
make -j8
make install
popd
popd
echo 'WARNING: you must make install on libc to install the headers before compiling gcc'
wget -c 'https://ftp.gnu.org/gnu/gcc/gcc-9.1.0/gcc-9.1.0.tar.xz'
tar -xf 'gcc-9.1.0.tar.xz'
patch -p0 < patch-gcc
pushd "gcc-9.1.0"
mkdir -pv build
pushd build
../configure --build=$HOST_TRIPLET --host=$TARGET --target=$TARGET --prefix=$TOOLCHAIN_SYSROOT --with-sysroot=$TOOLCHAIN_SYSROOT --enable-languages=c,c++
make -j8 all-gcc all-target-libgcc
make install-gcc install-target-libgcc

popd
popd
popd
