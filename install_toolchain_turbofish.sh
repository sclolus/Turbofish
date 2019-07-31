#!/bin/sh
export TARGET="i686-turbofish"
export ROOT_TOOLCHAIN="/toolchain_turbofish"
export SYSROOT="$ROOT_TOOLCHAIN/sysroot"
export CROSS="$ROOT_TOOLCHAIN/cross"

sudo mkdir -pv $ROOT_TOOLCHAIN
sudo chown $USER:$USER $ROOT_TOOLCHAIN
ln -s --force --no-dereference --verbose $ROOT_TOOLCHAIN toolchain_turbofish
mkdir -pv $SYSROOT $CROSS

mkdir -pv build_toolchain
cp patch-binutils patch-gcc build_toolchain
cd build_toolchain

# CROSS COMPILE BINUTILS
wget 'https://ftp.gnu.org/gnu/binutils/binutils-2.32.tar.xz'
tar -xf 'binutils-2.32.tar.xz'
patch -p0 < PatchBinutils
cd 'binutils-2.32'
# In LD subdirectory (Maybe install automake 1.15.1)
cd ld
automake
cd -
# Create a build directory in binutils
mkdir -p build
cd build
../configure --target=$TARGET --prefix=$CROSS --with-sysroot=$SYSROOT
make
make install
cd ../..

# CROSS COMPILE GCC
echo 'WARNING: you must make install on libc to install the headers before compiling gcc'
sudo apt install g++ libmpc-dev libmpfr-dev libgmp-dev
wget 'https://ftp.gnu.org/gnu/gcc/gcc-9.1.0/gcc-9.1.0.tar.xz'
tar -xf 'gcc-9.1.0.tar.xz'
patch -p0 < PatchGcc
cd 'gcc-9.1.0'
mkdir build
cd build
../configure --target=$TARGET --prefix=$CROSS --with-sysroot=$SYSROOT
make
make install
