#!/bin/sh

rustfmt ./**/**/src/*.rs --files-with-diff
rustfmt ./**/**/tests/*.rs --files-with-diff
