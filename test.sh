#!/bin/bash
make -C programs
rm ./image_disk.img
cd rust_kernel
./integration_test.sh $@
