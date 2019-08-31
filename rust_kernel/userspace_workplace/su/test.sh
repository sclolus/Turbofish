#!/bin/bash
TARGET=su
TEST_DIR=tests

mkdir -pv $TEST_DIR
mkdir -pv $TEST_DIR/bin
mkdir -pv $TEST_DIR/lib
mkdir -pv $TEST_DIR/lib64
mkdir -pv $TEST_DIR/usr
cp $TARGET $TEST_DIR/
# sudo mount -r --bind /bin ./tests/bin
# sudo mount -r --bind /lib ./tests/lib
# sudo mount -r --bind /lib64 ./tests/lib64
# sudo mount -r --bind /usr ./tests/usr
PATH=/:$PATH chroot ./tests /bin/bash
