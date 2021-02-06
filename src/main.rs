#![feature(asm)]
//#![windows_subsystem = "windows"]
use clap::{App, Arg};
mod runner;
use runner::run;
fn main() {
    let matches = App::new("rs_shellcode")
        .arg(
            Arg::new("file")
                .short('f')
                .about("shellcode path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("breakpoint")
                .short('b')
                .about("set breakpoint in debugger"),
        )
        .arg(
            Arg::new("offset")
                .short('o')
                .about("shellcode offset")
                .takes_value(true),
        )
        .get_matches();
    let set_breakpoint = matches.is_present("breakpoint");
    if set_breakpoint {
        println!("[*] Breakpoint flag set!");
    }
    let fp: String = matches.value_of_t("file").unwrap_or_else(|e| e.exit());
    let offset: u64 = match matches.value_of("offset") {
        Some(offset) => {
            if offset.find("0x") == Some(0) {
                let without_prefix = offset.trim_start_matches("0x");
                u64::from_str_radix(without_prefix, 16).unwrap_or(0)
            } else {
                u64::from_str_radix(offset, 10).unwrap_or(0)
            }
        }
        _ => 0,
    };
    println!("[*] Reading shellcode from path: {:?}", fp.clone());
    let contents = std::fs::read(fp).unwrap();
    run(&contents, offset, set_breakpoint);
}
