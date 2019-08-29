#!/bin/bash
TARGET=su
TEST_DIR=tests

mkdir -pv $TEST_DIR
mkdir -pv $TEST_DIR/bin
cp $TARGET $TEST_DIR/bin
su
