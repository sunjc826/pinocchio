#!/usr/bin/env bash
pushd "./ccompiler/input" || exit
python2 "$(realpath ../src/build-test-matrix.py)"
make --ignore-errors --file make.matrix
popd || exit