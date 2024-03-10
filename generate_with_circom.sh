#!/usr/bin/env bash
read -r CIRCOM_FILE_BASENAME
: "${CIRCOM_FILE_BASENAME:="aux_vector_clock"}"
set -e
pushd src/circom_circuits
if ! command -v circom > /dev/null
then
    echo "circom isn't installed"
    echo "Please install circom manually and add to PATH" 
    exit 1
fi
circom --r1cs --sym --c --O2 -- "$CIRCOM_FILE_BASENAME".circom
ln -s ../../../json/include/nlohmann "$CIRCOM_FILE_BASENAME"_cpp
pushd "$CIRCOM_FILE_BASENAME"_cpp
make
popd
popd
set +e

