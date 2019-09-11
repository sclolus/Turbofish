#!/bin/bash
export TARGET="i686-turbofish"
export PATH="/toolchain_turbofish/cross/bin:$PATH"
export TARGET_DIR="../../../system/bin"

mkdir -pv build_coreutils
cp patch-coreutils build_coreutils
cd build_coreutils
wget -c 'https://ftp.gnu.org/gnu/coreutils/coreutils-5.0.tar.bz2'
tar -xf 'coreutils-5.0.tar.bz2'
patch -p0 < patch-coreutils
cd coreutils-5.0
cp ../../patch-coreutils-configure .
mkdir build
cd build
CFLAGS="-g -O0 -fno-omit-frame-pointer" ../configure --build="`gcc -dumpmachine`" --host=$TARGET
cp ../../../patch-coreutils-config-h .
make -C lib
make -C src yes
make -C src cat
make -C src echo
make -C src kill
make -C src sleep
make -C src hostname
make -C src pwd
make -C src ls
make -C src chmod
make -C src cp
make -C src date
make -C src dd
make -C src df
make -C src ln
make -C src mkdir
make -C src mv
make -C src rm
make -C src touch
make -C src rmdir
make -C src env
make -C src head
make -C src tail
make -C src wc

cp -v src/cat $TARGET_DIR
cp -v src/echo $TARGET_DIR
cp -v src/kill $TARGET_DIR
cp -v src/sleep $TARGET_DIR
cp -v src/hostname $TARGET_DIR
cp -v src/pwd $TARGET_DIR
cp -v src/yes $TARGET_DIR
cp -v src/ls $TARGET_DIR
cp -v src/chmod $TARGET_DIR
cp -v src/cp    $TARGET_DIR
cp -v src/date  $TARGET_DIR
cp -v src/dd    $TARGET_DIR
cp -v src/df    $TARGET_DIR
cp -v src/ln    $TARGET_DIR
cp -v src/mkdir $TARGET_DIR
cp -v src/mv    $TARGET_DIR
cp -v src/rm    $TARGET_DIR
cp -v src/touch $TARGET_DIR
cp -v src/rmdir $TARGET_DIR
cp -v src/env $TARGET_DIR
cp -v src/head $TARGET_DIR
cp -v src/tail $TARGET_DIR
cp -v src/wc $TARGET_DIR
