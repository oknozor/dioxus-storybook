#!/usr/bin/env just --justfile

release:
  cargo build --release

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
    dx --verbose serve --hot-patch --package storybook-example --platform web

storybook:
    dx --verbose serve --hot-patch --package storybook --platform web
