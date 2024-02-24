#!/usr/bin/env bash
pushd ccompiler/external-code || exit
make all
popd || exit
if ! test -d "./dep/jinja"
then
    python2 -m pip install --target ./dep Jinja
fi