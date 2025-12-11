#!/bin/bash

openapi-generator-cli generate \
  -i ./api/agno-agentos-openapi.json \
  -g rust \
  -o ./api/agno-agentos-sdk \
  -t ./templates

find ./api/agno-agentos-sdk -type f -name "*.rs" -exec sed -i '' 's/use crate::models;/use crate::generated::models;/g' {} \;
find ./api/agno-agentos-sdk -type f -name "*.rs" -exec sed -i '' 's/crate::apis/crate::generated::apis/g' {} \;
find ./api/agno-agentos-sdk -type f -name "*.rs" -exec sed -i '' 's/use crate::{apis::ResponseContent, models};/use crate::generated::{apis::ResponseContent, models};/g' {} \;

rm -rf ./src/generated/apis
rm -rf ./src/generated/models
cp -r ./api/agno-agentos-sdk/src/apis ./src/generated/apis
cp -r ./api/agno-agentos-sdk/src/models ./src/generated/models
rm -rf ./api/agno-agentos-sdk

cargo fmt
