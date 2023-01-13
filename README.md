# Rust on Litex SoC (RISC-V)

> Using Rust on a customized LiteX SoC (RISC-V).

## Prerequisites

Clone the repo:
```
$ git clone https://github.com/wuhanstudio/litex-soc-icesugar-rust
$ cd litex-soc-icesugar-rust
$ git submodule init
$ git submodule update
```

Install LiteX:

```
$ mkdir python-litex && cd python-litex
$ wget https://raw.githubusercontent.com/enjoy-digital/litex/master/litex_setup.py
$ chmod +x litex_setup.py
$ ./litex_setup.py --init --install --user `whoami` --config=full
```

For C development:

```
$ ./litex_setup.py --gcc=riscv
```

For Rust development:

```
$ rustup target add riscv32imac-unknown-none-elf
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

You can upload bitstream and firmware to Icesugar board using the tool `icesprog` in side the folder `icesugar/tools/`.

## LiteX Soc

```
$ cd litex-soc-icesugar-rust
$ python3 -m litex_boards.targets.muselab_icesugar --build --doc
$ python3 -m litex_boards.targets.muselab_icesugar --flash
```

## C Demo

```
$ cd demo
$ make
$ icesprog -w demo.bin -o 0x40000
```

This C example can be generated using:

```
$ litex_bare_metal_demo --build-path=/home/YOUR_NAME/litex-soc-icesugar-rust/build/muselab_icesugar/
```

## Rust Demo

```
$ cd app
$ cargo objcopy --target riscv32i-unknown-none-elf --release -- -O binary app.bin
$ icesprog -o 0x40000 app.bin
```

This Rust library `icesugar-pac` was generated using [svd2rust](https://github.com/rust-embedded/svd2rust):

```
$ cargo install svd2rust
$ python3 -m litex_boards.targets.muselab_icesugar --csr-json csr.json --timer-uptime --build --csr-svd icesugar.svd
$ cargo new icesugar-pac && cd icesugar-pac
$ cp ../icesugar.svd ./
$ svd2rust -i icesugar.svd
$ mv generic.rs src/
```
