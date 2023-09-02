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

## LiteX SoC

```
$ cd litex-soc-icesugar-rust

# This command uses the upstream default config
# python3 -m litex_boards.targets.muselab_icesugar --build --doc
# python3 -m litex_boards.targets.muselab_icesugar --flash

# This command uses our custom config
$ python3 -m soc.targets.muselab_icesugar --build --doc
$ python3 -m soc.targets.muselab_icesugar --flash
```

## C Demo

```
# This command creates the folder demo that includes the source code
# litex_bare_metal_demo --build-path=./build/muselab_icesugar/

$ cd demo
$ make
$ icesprog -w demo.bin -o 0x40000
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

$ cargo new --lib icesugar-pac && cd icesugar-pac
$ cp ../icesugar.svd ./

$ svd2rust -i icesugar.svd
$ mv generic.rs src/

# Then manually added dependencies to Cargo.toml
```

## References

- https://github.com/icebreaker-fpga/icebreaker-litex-examples
- https://docs.rs/svd2rust/latest/svd2rust/
- http://pepijndevos.nl/2020/08/04/a-rust-hal-for-your-litex-fpga-soc.html
- https://github.com/pepijndevos/rust-litex-example
- https://github.com/pepijndevos/rust-litex-hal
