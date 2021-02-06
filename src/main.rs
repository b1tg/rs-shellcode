#![feature(asm)]
#![feature(pattern)]

// #![windows_subsystem = "windows"]
use bindings::windows::win32::system_services::VirtualAlloc;
use clap::{App, Arg};
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;

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
    let contents = match std::fs::read(fp) {
        Ok(res) => res,
        Err(e) => {
            println!("[-] Reading shellcode error: {}", e);
            return;
        }
    };
    let flen = contents.len();

    if flen as u64 <= offset {
        println!(
            "[-] Offset too big, offset: {}, file length: {}",
            offset, flen
        );
        return;
    }
    let new_buf = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            flen,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if new_buf == std::ptr::null_mut() {
        println!("[-] Failed to allocate memory");
        return;
    }
    let new_buf_ptr: *mut u8 = new_buf as *mut u8 as _;
    unsafe { std::ptr::copy_nonoverlapping(contents.as_ptr(), new_buf_ptr, flen) };
    println!("[*] Starting jmp to shellcode at offset 0x{:x}", offset);
    unsafe {
        let jmp_target = new_buf.offset(offset as isize);
        if set_breakpoint {
            asm!("int 3");
        }
        asm!("jmp {}",in(reg) jmp_target)
    };
}
