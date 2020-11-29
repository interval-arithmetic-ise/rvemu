# rvemu: RISC-V Emulataor

[![Build Status](https://travis-ci.com/d0iasm/rvemu.svg?branch=master)](https://travis-ci.com/d0iasm/rvemu)
[![Actions Status](https://github.com/d0iasm/rvemu/workflows/CI/badge.svg)](https://github.com/d0iasm/rvemu/actions)
[![docs.rs](https://docs.rs/rvemu/badge.svg)](https://docs.rs/rvemu)
[![crate.io](https://img.shields.io/crates/v/rvemu.svg)](https://crates.io/crates/rvemu)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/d0iasm/rvemu/master/LICENSE)

RISC-V online/CLI emulator with WebAssembly generated by Rust.

The online emulator is available here:
- [**rvemu.app**](https://rvemu.app/): Run a RISC-V binary you uploaded.
- [**rvemu.app/xv6**](https://rvemu.app/xv6.html): Run
  [`xv6`](https://github.com/mit-pdos/xv6-riscv).

The emulator supports RV64GC ISA (RV64IMAFD, Zicsr, Zifencei, RV64C),
privileged ISA, CSRs, virtual memory system (Sv39), peripheral devices (UART,
CLINT, PLIC, Virtio), and device tree. See [the "Features List"
section](https://github.com/d0iasm/rvemu#features-list) for the details of
features.

These features are compliant with "The RISC-V Instruction Set Manual Volume I:
Unprivileged ISA Document Version 20191213" and "The RISC-V Instruction Set
Manual Volume II: Privileged Architecture Document Version
20190608-Priv-MSU-Ratified" in [the RISC-V
specifications](https://riscv.org/specifications/).

## Usage

The emulator can run both in your browser and in your terminal. Also, the
emulator can be embedded in your project by [the crate
registry](https://crates.io/crates/rvemu).

### On browser

You can run [`xv6`](https://github.com/mit-pdos/xv6-riscv), a simple Unix-like
operating system, in [**rvemu.app/xv6**](https://rvemu.app/xv6.html).

![Demo](https://raw.githubusercontent.com/d0iasm/rvemu/master/demo.gif)

You also be able to run an arbitrary RISC-V binary in
[**rvemu.app**](https://rvemu.app/). The online emulator supports the following
commands:
- __upload__: Upload local RISC-V binaries for the execution on the emulator.
- __ls__: List the files you uploaded.
- __run [file]__: Execute a file which you uploaded or some files are already
  embedded.
- __help__: Print all commands you can use.

See [the "Build RISC-V binary"
section](https://github.com/d0iasm/rvemu#build-risc-v-binary) for more
information to build RISC-V binary.

### On terminal

The option `--kernel` or `-k` specifies a kernel image, and `--file` or `-f`
specifies a root filesystem image.

**Linux**

```
$ ./target/release/rvemu-cli -k bin/linux/bbl.bin -f bin/linux/busybear.bin
```

**xv6**

```
$ ./target/release/rvemu-cli -k bin/xv6/kernel.bin -f bin/xv6/fs.img
```

**Bare-metal binary**

You can use an arbitrary RISC-V binary and you can skip the `-f` option. An ELF
binary should have no headers.
```
$ ./target/release/rvemu-cli -k <your-binary>
```

## Build

### For WEB application

The `wasm-pack build` command generates a `pkg` directory and makes Rust
source code into `.wasm` binary. It also generates the JavaScript API for
using our Rust-generated WebAssembly. The toolchain's supported target is
`wasm32-unknown-unknown`. You need to execute this command whenever you change
your Rust code. 

```
// This is the alias of `wasm-pack build lib/rvemu-wasm --out-dir
<path-to-rvemu>/public/pkg --target web --no-typescript`.
$ make rvemu-wasm
```

This command installs dependencies in the `node_modules` directory. Need `npm
install --save` in the `public` directory at the first time and whenever you
change dependencies in package.json.

```
$ npm install --save // at the public directory
```

You can see the website via http://localhost:8000. `npm start` is the alias of
`python3 -m http.server` so you need Python3 in your environment.

```
$ npm start // at the public directory
```

### For CLI tool

Build the emulator as a CLI tool.

```
$ make rvemu-cli
```

## Build RISC-V binary

You might need to build riscv toolchain.

```
// In the riscv-gnu-toolchain directory.
// https://github.com/riscv/riscv-gnu-toolchain
$ ./configure --prefix=/opt/riscv --with-arch=rv64gc
$ make
$ make linux
```

### Bare-metal C program

You need to make an ELF file without headers, which starts at the address `0x8000_0000` by the following instructions:

```
// Make an assembly file from a C file.
$ riscv64-unknown-elf-gcc -S -nostdlib foo.c

// Make a binary file from an assembly file with start position 0x8000_0000.
$ riscv64-unknown-elf-gcc -Wl,-Ttext=0x80000000 -nostdlib -o foo foo.s

// Remove headers from a binary file.
$ riscv64-unknown-elf-objcopy -O binary foo foo.text
```

### Linu

The page [Running 64- and 32-bit RISC-V Linux on
QEMU](https://risc-v-getting-started-guide.readthedocs.io/en/latest/linux-qemu.html)
helps to build a Linux image. When you compile this project in a x86 computer,
you may need to:

- update `CC := gcc` to `CC := riscv64-unknown-elf-gcc` in
  `riscv-pk/build/Makefile`
- comment out the "build bbl" part in `busybear-linux/scripts/build.sh`

because the build script for cross compiling in riscv-pk is broken.  See
https://github.com/riscv/riscv-pk/blob/master/configure#L1146-L1148

## Testing

You can see the binaries for unit testings in
[riscv/riscv-tests](https://github.com/riscv/riscv-tests).  The following
command executes all `rv64ua/d/f/i/m-p-*` binaries.

## Analyzing with perf

```
$ perf record -F99 --call-graph dwarf ./target/release/rvemu-cli -k bin/xv6/kernel.bin -f bin/xv6-fs.img
$ perf report
```

## Publish

[The site](https://rvemu.app/) is hosted by a firebase.

```
$ firebase deploy
```

## Features List

The emulator supports the following features:
- [x] RV64G ISA
  - [x] RV64I (v2.1): supports 52/52 instructions (`fence` does nothing for
    now)
  - [x] RV64M (v2.0): supports 13/13 instructions
  - [x] RV64A (v2.1): supports 22/22 instructions (No atomicity for now)
  - [x] RV64F (v2.2): supports 30/30 instructions
  - [x] RV64D (v2.2): supports 32/32 instructions
  - [x] Zifencei (v2.0): supports 1/1 instructions (`fence.i` does nothing for
    now)
  - [x] Zicsr (v2.0): supports 6/6 instructions (No atomicity for now)
- [x] RV64C ISA (v2.0): support 36/36 instructions
- [x] Privileged ISA: supports 7/7 instructions (`sfence.vma`, `hfence.bvma`,
  and `hfence.gvma` do nothing for now)
- [x] Control and status registers (CSRs)
  - [x] Machine-level CSRs
  - [x] Supervisor-level CSRs
  - [ ] User-level CSRs
- [x] Virtual memory system (Sv39)
- [x] Devices
  - [x] UART: universal asynchronous receiver-transmitter
  - [x] CLINT: core local interruptor
  - [x] PLIC: platform level interrupt controller
  - [x] Virtio: virtual I/O
- [x] Device tree

## Install To set up environment:

1. rustc
2. rustup nightly
3. wasm-pack
4. dtc (apt install device-tree-compiler)

## Dependencies

- [Nightly Rust](https://doc.rust-lang.org/1.2.0/book/nightly-rust.html)
- [Python3](https://www.python.org/downloads/)
- wasm-pack
- npm
  - [xterm](https://xtermjs.org/)
  - xterm-addon-fit
- dtc: device tree compiler

## Resources ### Documents
- [RISC-V Specifications](https://riscv.org/specifications/)
- [Rust and
  WebAssembly](https://rustwasm.github.io/docs/book/introduction.html)
- [riscv/riscv-sbi-doc](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc)
- [riscv/riscv-elf-psabi-doc](https://github.com/riscv/riscv-elf-psabi-doc/blob/master/riscv-elf.md)
- [riscv/riscv-asm-manual](https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md)

### Implementation of other emulators

- [qemu/qemu](https://github.com/qemu/qemu)
- [riscv/riscv-isa-sim](https://github.com/riscv/riscv-isa-sim)

### Helpful tools

- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)
- [RISC-V Online Simulator](https://www.kvakil.me/venus/)

## Articles about this project

- [Emulate 32-Bit And 64-Bit RISC-V In Your Browser With Asami’s Open Source
  rvemu | Gareth Halfacree,
  Hackster.io](https://riscv.org/2020/01/emulate-32-bit-and-64-bit-risc-v-in-your-browser-with-asamis-open-source-rvemu-gareth-halfacree-hackster-io/)
- [Emulate 32-Bit and 64-Bit RISC-V in Your Browser with Asami's Open Source
  rvemu](https://www.hackster.io/news/emulate-32-bit-and-64-bit-risc-v-in-your-browser-with-asami-s-open-source-rvemu-b783f672e463)
