#!/usr/bin/env just --justfile

release:
  cargo build --release

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
    dx --verbose serve --hot-patch --package dioxus-storybook --platform web
