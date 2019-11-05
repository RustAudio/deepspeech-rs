#!/usr/bin/env bash

dir=$(dirname "${BASH_SOURCE[0]}")
release="v0.6.0-alpha.12"

# Fail on error
set -e

# Verbose execution
set -v

# Download the native client
client_dir="$dir/target/native_client"
rm -rf $client_dir || true
mkdir -p $client_dir
pushd $client_dir
wget https://github.com/mozilla/DeepSpeech/releases/download/$release/native_client.amd64.cpu.linux.tar.xz
tar xf native_client.*
popd
#export LD_LIBRARY_PATH="${LD_LIBRARY_PATH:+${LD_LIBRARY_PATH}:}$client_dir"
export LIBRARY_PATH="${LIBRARY_PATH:+${LIBRARY_PATH}:}$client_dir"
cargo test
