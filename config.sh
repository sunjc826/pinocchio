#!/usr/bin/env bash
default_app_name=aux-vector-clock
echo -n "Enter name of app (default=$default_app_name):"
export VC_APP_NAME
read -r VC_APP_NAME
: "${VC_APP_NAME:="$default_app_name"}"

export VC_LOOP_SANITY_LIMIT
VC_LOOP_SANITY_LIMIT=10000

export VC_NOVA_CIRCUIT_RS_DIR
VC_NOVA_CIRCUIT_RS_DIR="$(realpath src/circuits/)"

export PYTHONPATH
PYTHONPATH+=:"$(realpath ./dep)"