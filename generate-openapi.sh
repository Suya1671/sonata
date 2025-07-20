#!/bin/env sh

# Generate OpenAPI client
pnpx @openapitools/openapi-generator-cli generate -i https://opensubsonic.netlify.app/docs/openapi/openapi.json -g rust-axum -o sonata-openapi --global-property skipFormModel=false --enable-post-process-file --global-property generateAliasAsModel=true
# Replace name = "openapi" with name = "sonata-openapi" in sonata-openapi/Cargo.toml
sed -i 's/name = "openapi"/name = "sonata-openapi"/g' sonata-openapi/Cargo.toml
# remove all mentions to async_trait, including `use async_trait::async_trait;` and `#[async_trait]` for all src files
sed -i '/use async_trait::async_trait;/d' sonata-openapi/src/**/*.rs
sed -i 's/\#\[async_trait::async_trait\]/\#\[auto_async_send_sync\:\:auto_send_sync\]/g' sonata-openapi/src/**/*.rs
sed -i 's/\#\[async_trait\]/\#\[auto_async_send_sync\:\:auto_send_sync\]/g' sonata-openapi/src/**/*.rs
sed -i 's/async-trait = "0.1"/auto-async-send-sync = {path = "..\/auto-async-send-sync"}/g' sonata-openapi/Cargo.toml

# append [lints.clippy] section to Cargo.toml
echo '[lints.clippy]' >> sonata-openapi/Cargo.toml
echo 'pedantic = "allow"' >> sonata-openapi/Cargo.toml
echo 'nursery = "allow"' >> sonata-openapi/Cargo.toml
echo 'crates = "allow"' >> sonata-openapi/Cargo.toml
echo 'uninlined_format_args = "allow"' >> sonata-openapi/Cargo.toml
echo 'empty_line_after_doc_comments = "allow"' >> sonata-openapi/Cargo.toml
