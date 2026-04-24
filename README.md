# rload

A minimal, blazing-fast load testing CLI written in Rust.

## Features

- Simple CLI
- High concurrency (WIP)
- Minimal dependencies

## Usage

```bash
cargo run -- \
  --url https://reqres.in/api/login \
  --method POST \
  --requests 100 \
  --concurrency 5 \
  --header "Content-Type: application/json" \
  --body '{"email":"eve.holt@reqres.in","password":"cityslicka"}'