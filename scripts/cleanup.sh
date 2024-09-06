#!/bin/sh

find . -type d -path '**/**/target' -exec rm -v -r {} \;
find . -type f -path '**/**/Cargo.lock' -exec rm -v {} \;