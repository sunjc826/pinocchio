#!/usr/bin/env bash
cd "./ccompiler/input" || exit
PROGRAM="my-test"
python2 -m pdb ../src/vercomp.py "$PROGRAM".c --arith build/"$PROGRAM"-p0-b32.arith --bit-width 32 --cpparg _Ibuild/ _DPARAM=0 _DBIT_WIDTH=32
cd ../../