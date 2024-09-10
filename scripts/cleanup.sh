#!/bin/sh

find . -type d -name 'target' -exec rm -v -r {} \;
find . -type f -name 'Cargo.lock' -exec rm -v {} \;
