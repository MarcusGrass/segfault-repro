#!/bin/sh
set -e
RUSTFLAGS='-C panic=abort -C link-arg=-nostartfiles -g -C target-feature=+crt-static' cargo r --target x86_64-unknown-linux-gnu "$@"