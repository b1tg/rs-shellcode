# rs-shellcode

A shellcode runner write in Rust use [windows-rs](https://github.com/microsoft/windows-rs).


# how to use it

Install [rustup](https://rustup.rs/), use nightly toochain.

```sh
rustup default nightly
```

Use msfvenom generate shellcode for test.

```sh
msfvenom -p windows/x64/exec CMD=calc.exe  --platform win -f raw -o calc64.raw
```

Build:

```sh
cargo build
```

Usage:
```
rs_shellcode 

USAGE:
    rs_shellcode.exe [FLAGS] [OPTIONS] -f <file>

FLAGS:
    -b               set breakpoint in debugger
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f <file>          shellcode path
    -o <offset>        shellcode offset
```

Run:

```sh
./target/debug/rs_shellcode.exe -f <SHELLCODE_PATH>
```

When your shellcode not start at offset 0, you can specify the offset use `-o`:

```sh
./target/debug/rs_shellcode.exe -f <SHELLCODE_PATH> -o 0x30
```


Run with breakpoint flag (`-b`):

```sh
./target/debug/rs_shellcode.exe -f <SHELLCODE_PATH> -b
```

use this flag, you can break just before your shellcode in the debugger, which will make your life easier.

![breakpoint in windbg](./breakpoint.png)