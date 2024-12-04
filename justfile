#!/usr/bin/env just --justfile

set dotenv-load := true

@run: create migrate
    bacon run

@lint:
    pre-commit run --all-files

[group('sqlx')]
@create:
    sqlx database create

[group('sqlx')]
@migrate:
    sqlx migrate run

[group('sqlx')]
@revert:
    sqlx migrate revert

[group('sqlx')]
@prepare:
    cargo sqlx prepare

[group('sqlx')]
@add name:
    sqlx migrate add -r {{ name }}
