# Rust on Litex SoC (RISC-V)

> Using Rust on a customized LiteX SoC (RISC-V).

## Soc

```
$ python3 -m litex_boards.targets.muselab_icesugar --build --doc
$ python3 -m litex_boards.targets.muselab_icesugar --flash
```

## C Demo

```
$ litex_bare_metal_demo --build-path=/home/wuhanstudio/muselab_ice_sugar/build/muselab_icesugar/
$ icesprog -w demo.bin -o 0x40000
```

## Rust Demo

```
$ cargo objcopy --target riscv32i-unknown-none-elf --release -- -O binary app.bin
$ icesprog -o 0x40000 app.bin
```
