#!/bin/bash
TARGET=su
TEST_DIR=tests

mkdir -pv $TEST_DIR
mkdir -pv $TEST_DIR/bin
mkdir -pv $TEST_DIR/lib
mkdir -pv $TEST_DIR/lib64
mkdir -pv $TEST_DIR/usr
sudo cp $TARGET $TEST_DIR/
sudo mount -r --bind /bin ./tests/bin
sudo mount -r --bind /lib ./tests/lib
sudo mount -r --bind /lib64 ./tests/lib64
sudo mount -r --bind /usr ./tests/usr
sudo mount --bind /tmp ./tests/tmp
PATH=/:$PATH sudo chroot ./tests /bin/bash

sudo umount -lf ./tests/bin
sudo umount -lf ./tests/lib
sudo umount -lf ./tests/lib64
sudo umount -lf ./tests/usr
