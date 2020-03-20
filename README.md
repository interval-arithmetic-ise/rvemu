# rvemu: RISC-V Online Emulataor
[![Build Status](https://travis-ci.com/d0iasm/rvemu.svg?branch=master)](https://travis-ci.com/d0iasm/rvemu)
[![Actions Status](https://github.com/d0iasm/rvemu/workflows/CI/badge.svg)](https://github.com/d0iasm/rvemu/actions)
[![docs.rs](https://docs.rs/rvemu/badge.svg)](https://docs.rs/rvemu)
[![crate.io](https://img.shields.io/crates/v/rvemu.svg)](https://crates.io/crates/rvemu)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/d0iasm/rvemu/master/LICENSE)

RISC-V online emulator with WebAssembly generated by Rust. The emulator implements the standard extensions RV64G (RV64IMAFD, Zicsr, Zifencei) and complies with [the RISC-V specifications](https://riscv.org/specifications/).

The online emulator is available here: [**rvemu.app**](https://rvemu.app/)

Supports the following features:  
"The RISC-V Instruction Set ManualVolume I: Unprivileged ISADocument Version 20191213"
- [x] RV64G ISAs
  - [x] RV64I (v2.1): supports 52/52 instructions (FENCE, ECALL and EBREAK do nothing for now)
  - [x] RV64M (v2.0): supports 13/13 instructions
  - [x] RV64A (v2.1): supports 22/22 instructions (No atomicity for now)
  - [x] RV64F (v2.2): supports 30/30 instructions
  - [x] RV64D (v2.2): supports 32/32 instructions
  - [x] Zifencei (v2.0): supports 1/1 instructions (FENCE.i does nothing for now)
  - [x] Zicsr (v2.0): supports 6/6 instructions (No atomicity for now)
- [ ] RV64C ISAs

"The RISC-V Instruction Set ManualVolume II: Privileged ArchitectureDocument Version 20190608-Priv-MSU-Ratified"
- [x] Privileged ISAs: supports 7/7 instructions (WFI, SFENCE.VMA, HFENCE.BVMA
  and HFENCE.GVMA do nothing for now)
- [x] Control and status registers (CSRs)
  - [x] Machine-level CSRs
  - [x] Supervisor-level CSRs
  - [ ] User-level CSRs
- [x] Virtual memory system (Sv39)
- [x] Devices
  - [x] UART
  - [x] CLINT
  - [x] PLIC
  - [x] Virtio

# Usage
The online emulator supports the following commands:
- __upload__: Upload local RISC-V binaries for the execution on the emulator.
- __ls__: List the files you uploaded.
- __run [file]__: Execute a file which you uploaded or some files are already
  embedded.
- __help__: Print all commands you can use.

It can run [`xv6`](https://github.com/mit-pdos/xv6-riscv), a simple Unix-like operating system, by the
`run xv6` command. A normal xv6 ELF file can't be used in the emulator because of [this
issue](https://github.com/mit-pdos/xv6-riscv/pull/9), so you need to use the
embedded xv6 binary or
[examples/xv6-kernel.text](https://github.com/d0iasm/rvemu/blob/master/examples/xv6-kernel.text).

![Demo](https://raw.githubusercontent.com/d0iasm/rvemu/master/demo.gif)

## Build and run on the local browser
The `wasm-pack build` command generates a pkg directory and makes Rust source code into `.wasm` binary. It also generates the JavaScript API for using our Rust-generated WebAssembly. The toolchain's supported target is `wasm32-unknown-unknown`.
You need to execute this command whenever you change your Rust code.
```
$ make rvemu-wasm
// This is the alias of `wasm-pack build lib/rvemu-wasm --out-dir <path-to-rvemu>/public/pkg --target web`.
```

This command installs dependencies in the `node_modules` directory. Need `npm install --save` in the `public` directory at the first time and whenever you change dependencies in package.json.
```
$ npm install --save // at the public directory
```

You can see the website via http://localhost:8000. `npm start` is the alias of `python3 -m http.server` so you need Python3 in your environment.
```
$ npm start // at the public directory
```

## Build and run as a CLI tool
The emulator can be executed as a CLI tool too. You can build it by `make rvemu-cli` command which is the alias of `cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml`.

To execute the RISC-V ELF binary, XV6 in the folloing example, you can use `--kernel` or `-k` options to specify the kernel image. Note that `xv6-kernel.text` is an ELF file without headers by the command `riscv64-unknown-elf-objcopy -O binary kernel xv6-kernel.text`
```
$ ./target/release/rvemu-cli -k examples/xv6-kernel.text -f examples/xv6-fs.img
```

You can see the details of how to use by the `help` option.
```
$ ./target/release/rvemu-cli --help
rvemu: RISC-V emulator 0.0.1
Asami Doi <@d0iasm>

USAGE:
    rvemu-cli [FLAGS] [OPTIONS] --kernel <kernel>

FLAGS:
    -d, --debug      Enables to output debug messages
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>        A raw disk image
    -k, --kernel <kernel>    A kernel ELF image without headers
```

## Build RISC-V binary
This emulator starts to execute at the address 0x8000_0000, the start address of DRAM, so you need to extract .text section to execute your binary file on the emulator.
```
// Make an assembly file from a C file.
$ riscv64-unknown-elf-gcc -S -nostdlib hoge.c
// Make a binary file from an assembly file with start position 0.
$ riscv64-unknown-elf-gcc -Wl,-Ttext=0x80000000 -nostdlib -o hoge hoge.s
// Extract a text section from a binary file.
$ riscv64-unknown-elf-objcopy -O binary hoge hoge.text
```

## Testing
You can see the binaries for unit testings in [riscv/riscv-tests](https://github.com/riscv/riscv-tests).
The following command executes all `rv64ua/d/f/i/m-p-*` binaries.
TODO: not all tests are passed for now.
```
$ make test
```

## Analyzing with perf
```
$ perf record -F99 --call-graph dwarf ./target/release/rvemu-cli -k examples/xv6-kernel.text -f examples/xv6-fs.img
$ perf report
```

## Publish
[The site](https://rvemu.app/) is hosted by a firebase.
```
$ firebase deploy
```

## Dependencies
- wasm-pack
- npm
  - [xterm](https://xtermjs.org/)
  - xterm-addon-fit

## Resources
### Documents
- [RISC-V Specifications](https://riscv.org/specifications/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/introduction.html)
- [riscv/riscv-sbi-doc](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc)
- [riscv/riscv-elf-psabi-doc](https://github.com/riscv/riscv-elf-psabi-doc/blob/master/riscv-elf.md)
- [riscv/riscv-asm-manual](https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md)

### Implementation of other emulators
- [qemu/qemu](https://github.com/qemu/qemu)
- [riscv/riscv-isa-sim](https://github.com/riscv/riscv-isa-sim)
- [riscv/riscv-angel](https://github.com/riscv/riscv-angel)
- [riscv/riscv-ovpsim](https://github.com/riscv/riscv-ovpsim)
- [rv8-io/rv8](https://github.com/rv8-io/rv8)
- [TinyEmu](https://bellard.org/tinyemu/)
- [stephank/rvsim](https://github.com/stephank/rvsim)

### Helpful tools
- [riscv/riscv-tests](https://github.com/riscv/riscv-tests)
- [wat2wasm demo](https://webassembly.github.io/wabt/demo/wat2wasm/)
- [RISC-V Online Simulator](https://www.kvakil.me/venus/)

## Articles about this project
- [Emulate 32-Bit And 64-Bit RISC-V In Your Browser With Asami’s Open Source rvemu | Gareth Halfacree, Hackster.io](https://riscv.org/2020/01/emulate-32-bit-and-64-bit-risc-v-in-your-browser-with-asamis-open-source-rvemu-gareth-halfacree-hackster-io/)
- [Emulate 32-Bit and 64-Bit RISC-V in Your Browser with Asami's Open Source rvemu](https://www.hackster.io/news/emulate-32-bit-and-64-bit-risc-v-in-your-browser-with-asami-s-open-source-rvemu-b783f672e463)
