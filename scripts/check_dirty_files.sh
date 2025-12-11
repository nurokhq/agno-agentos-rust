#!/bin/bash

./scripts/generate_api_sdk.sh

if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Generated files are not up to date. Please run ./scripts/generate_api_sdk.sh and commit the changes."
    git status --porcelain
    exit 1
fi
