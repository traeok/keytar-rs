#!/bin/bash

# Set environment variables needed for cross-compilation in current shell
set_env() {
    export PKG_CONFIG_SYSROOT_DIR="/"
    export RUSTFLAGS="-L $1"
    export PKG_CONFIG_PATH="$1/pkgconfig/"
}

case "$1" in
    "aarch64-unknown-linux-gnu")
        set_env "/usr/lib/aarch64-linux-gnu"
        ;;
    "aarch64-unknown-linux-musl")
        set_env "/usr/lib"
        ;;
    "armv7-unknown-linux-gnueabihf")
        set_env "/usr/lib/arm-linux-gnueabihf"
        ;;
    "i686-unknown-linux-gnu")
        set_env "/usr/lib/i386-linux-gnu"
        ;;
    "x86_64-unknown-linux-gnu")
        set_env "/usr/lib/x86_64-linux-gnu"
        ;;
    "x86_64-unknown-linux-musl")
        set_env "/usr/lib"
        ;;
    *)
        ;;
esac
