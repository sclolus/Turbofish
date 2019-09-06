#!/bin/bash

mkdir -pv build_criterion
cd build_criterion
git clone --recursive https://github.com/Snaipe/Criterion.git
cd Criterion
mkdir -pv build
cd build
cmake ..
cmake --build .
make install
