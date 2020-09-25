#!/bin/sh


rm -rf build-linux
mkdir build-linux
cd build-linux
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --config Release -j 4
cmake --build . --config Release --target package
