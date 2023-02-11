# Rust Kernel (x86_64)

Based in [this blog series](https://os.phil-opp.com/).

## Install Rust Nightly

This step is only required if you are not using Rust Nightly:
> rustup default nightly

## Steps to compile the kernel

> rustup component llvm-tools-preview

> rustup component add rust-src

> cargo install bootimage

## Compile

> cargo bootimage

## Running the kernel

First of all, [Install Qemu](https://www.qemu.org/download/) and put the executable dir into PATH.

(After installing qemu on Windows you could need to  restart your shell/editor)

> cargo run


You can run the kernel without rust using the following command:

> qemu-system-x86_64 -drive format=raw,file=target\x86_64-rust_os\debug\bootimage-rust_os.bin