#!/bin/bash
wget https://api.beatsaver.com/docs/swagger.json
openapi-generator generate -g rust -i swagger.json --skip-validate-spec
rm swagger.json
cargo fmt