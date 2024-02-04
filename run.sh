#!/usr/bin/env bash
echo -n "Enter name of app (default=bloom-filter):"
export VC_APP_NAME
read -r VC_APP_NAME
: "${VC_APP_NAME:="bloom-filter"}"

export VC_LOOP_SANITY_LIMIT
VC_LOOP_SANITY_LIMIT=10000

export VC_NOVA_CIRCUIT_RS_DIR
VC_NOVA_CIRCUIT_RS_DIR="$(realpath src/circuits/)"

export PYTHONPATH
if ! test -d "./dep/jinja"
then
    python2 -m pip install --target ./dep Jinja
fi
PYTHONPATH+=":$(realpath ./dep)"
pushd "./ccompiler/input" || exit
python2 "$(realpath ../src/build-test-matrix.py)"
make --ignore-errors --file make.matrix
popd || exit