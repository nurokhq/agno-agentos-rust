#!/bin/bash

openapi-generator-cli generate \
  -i ./api/agno-agentos-openapi.json \
  -g rust \
  -o ./api/agno-agentos-sdk \
  -t ./templates

rm -rf ./src/generated/apis
rm -rf ./src/generated/models
cp -r ./api/agno-agentos-sdk/src/apis ./src/generated/apis
cp -r ./api/agno-agentos-sdk/src/models ./src/generated/models
rm -rf ./api/agno-agentos-sdk

cargo fmt
