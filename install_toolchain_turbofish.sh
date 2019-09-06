#!/bin/bash
export TARGET="i686-turbofish"
export ROOT_TOOLCHAIN="/toolchain_turbofish"
export TOOLCHAIN_SYSROOT="$ROOT_TOOLCHAIN/sysroot"
export CROSS="$ROOT_TOOLCHAIN/cross"
export LIBC_DIR="libc"

sudo mkdir -pv $ROOT_TOOLCHAIN
sudo chown $USER:$USER $ROOT_TOOLCHAIN
mkdir -pv $TOOLCHAIN_SYSROOT $CROSS
mkdir -pv $TOOLCHAIN_SYSROOT/usr
mkdir -pv $TOOLCHAIN_SYSROOT/usr/{lib,include}
cp -rv $LIBC_DIR/include/* $TOOLCHAIN_SYSROOT/usr/include

mkdir -pv build_toolchain
cp patch-binutils patch-gcc build_toolchain
cd build_toolchain

# CROSS COMPILE BINUTILS
wget -c 'https://ftp.gnu.org/gnu/binutils/binutils-2.32.tar.xz'
tar -xf 'binutils-2.32.tar.xz'
patch -p0 < patch-binutils
cd 'binutils-2.32'
# In LD subdirectory (Maybe install automake 1.15.1)
cd ld
automake-1.15
cd -
# Create a build directory in binutils
mkdir -p build
cd build
../configure --target=$TARGET --prefix=$CROSS --with-sysroot=$TOOLCHAIN_SYSROOT
make -j8
make install
cd ../..

# CROSS COMPILE GCC
echo 'WARNING: you must make install on libc to install the headers before compiling gcc'
sudo apt install g++ libmpc-dev libmpfr-dev libgmp-dev
wget -c 'https://ftp.gnu.org/gnu/gcc/gcc-9.1.0/gcc-9.1.0.tar.xz'
tar -xf 'gcc-9.1.0.tar.xz'
patch -p0 < patch-gcc
cd 'gcc-9.1.0'
mkdir -p build
cd build
../configure --target=$TARGET --prefix=$CROSS --with-sysroot=$TOOLCHAIN_SYSROOT --enable-languages=c,c++
make -j8 all-gcc all-target-libgcc
make install-gcc install-target-libgcc

rm /toolchain_turbofish/cross/lib/gcc/i686-turbofish/9.1.0/crti.o -f
rm /toolchain_turbofish/cross/lib/gcc/i686-turbofish/9.1.0/crtn.o -f
rm /toolchain_turbofish/cross/lib/gcc/i686-turbofish/9.1.0/crtbegin.o -f
rm /toolchain_turbofish/cross/lib/gcc/i686-turbofish/9.1.0/crtend.o -f

# DASH
# URL: http://gondor.apana.org.au/~herbert/dash/files/
# take version 0.5.10.2
