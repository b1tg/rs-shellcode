#![feature(asm)]
// #![windows_subsystem = "windows"]
use bindings::windows::win32::system_services::VirtualAlloc;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./rs_shellcode \"C:\\Users\\Admin\\Desktop\\calc64.raw\"");
        return;
    }
    let fp = &args[1];
    // let fp = "C:\\Users\\Admin\\Desktop\\calc64.raw".to_owned();
    println!("[*] Reading shellcode from path: {:?}", fp.clone());
    let contents = std::fs::read(fp).unwrap();
    let flen = contents.len();

    let test_buf = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            flen,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if test_buf == std::ptr::null_mut() {
        println!("[*] Failed to allocate memory");
        return;
    }
    let test_buf_ptr: *mut u8 = test_buf as *mut u8 as _;
    unsafe { std::ptr::copy_nonoverlapping(contents.as_ptr(), test_buf_ptr, flen) };
    println!("[*] Before jmp to shellcode");

    unsafe { asm!("jmp {}",in(reg) test_buf) };
}
