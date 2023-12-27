#!/usr/bin/env bash

# Example 1: run_pycparser_example.sh c-to-c.py c_files/funky.c
# Example 2: run_pycparser_example.sh cdecl.py "int *x;"

set -e
FULL_PATH="$(realpath ccompiler/external-code)"
export PYTHONPATH="$FULL_PATH/ply-3.4:$FULL_PATH/pycparser-2.08"
set -x
cd "$FULL_PATH/pycparser-2.08/examples"
python2 "$@"
set +x
cd ../..
set +e
