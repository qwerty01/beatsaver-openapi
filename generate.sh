!/bin/bash
wget https://api.beatsaver.com/docs/swagger.json
openapi-generator generate -g rust -i swagger.json --skip-validate-spec
rm swagger.json
sed -i 's/version = "\([0-9]\+\)\.\([0-9]\+\)"/version = "\1.\2.0"/' Cargo.toml
cargo fmt