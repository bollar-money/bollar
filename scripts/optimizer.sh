#!/bin/bash

# U="cosmwasm"
# V="0.16.1"

# M=$(uname -m)
#M="x86_64" # Force Intel arch

# A="linux/${M/x86_64/amd64}"
# S=${M#x86_64}
# S=${S:+-$S}

# docker run --platform $A --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   $U/optimizer$S:$V

# docker run --rm -v "$(pwd)":/code \
#   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   cosmwasm/optimizer:0.16.1

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  babylonlabs/optimizer


# DOCKER=$(which docker)
# CUR_DIR=$(pwd)
# CUR_BASENAME=$(basename $CUR_DIR)

# OPTIMIZER_IMAGE_NAME="babylonlabs/optimizer"

# $DOCKER run --rm -v "$CUR_DIR":/code \
#   --mount type=volume,source="${CUR_BASENAME}_cache",target=/target \
#   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
#   $OPTIMIZER_IMAGE_NAME
