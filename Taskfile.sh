#!/bin/env bash

set -e
shopt -s expand_aliases

function test {
    cargo test
}

function watch-run {
    cargo watch -x run
}

function lint {
    cargo fmt
    cargo clippy
}

function autofix {
    cargo clippy --fix --allow-dirty
    cargo fmt
}

function db-cli {
    sqlite3 "${DATABASE_URL/sqlite:\/\//}"
}

function docker:build {
    docker build . -t "$(_image-tag)"
}

function docker:publish {
    docker:build
    docker push "$(_image-tag)"
}

function _image-tag {
    echo "discrimy/sdgenbox:v$(version)"
}

function version {
    cargo metadata --no-deps --format-version=1 | jq '.packages[0].version' -r
}

function help {
    echo "$0 <command> <args>"
    echo "Commands:"
    grep -Po 'function \K([a-z\d:-]+)' Taskfile.sh | cat -n
}

TIMEFORMAT="Task completed in %3lR"
time "${@:-help}"
