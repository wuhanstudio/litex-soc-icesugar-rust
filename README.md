# Rust on Litex SoC (RISC-V)

> Using Rust on a customized LiteX SoC (RISC-V).

## Soc

```
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
