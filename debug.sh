#!/usr/bin/env bash
. config.sh
pushd "./ccompiler/input" || exit
python2 -m pdb ../src/vercomp.py "$VC_APP_NAME".c \
    --arith build/"$VC_APP_NAME"-p0-b32.arith \
    --bit-width 32 \
    --cpparg _Ibuild/ _DPARAM=0 _DBIT_WIDTH=32 \
    --loop-sanity-limit $VC_LOOP_SANITY_LIMIT \
    --nova-circuit-rs-dir "$VC_NOVA_CIRCUIT_RS_DIR"

popd || exit