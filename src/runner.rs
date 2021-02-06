use bindings::windows::win32::system_services::VirtualAlloc;
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;

pub fn run(shellcode: &[u8], offset: u64, set_bp: bool) {
    let sc_len = shellcode.len();
    if sc_len as u64 <= offset {
        println!(
            "[*] Offset too big, offset: {}, file length: {}",
            offset, sc_len
        );
        return;
    }
    let new_buf = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            sc_len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if new_buf == std::ptr::null_mut() {
        println!("[*] Failed to allocate memory");
        return;
    }
    let new_buf_ptr: *mut u8 = new_buf as *mut u8 as _;
    unsafe { std::ptr::copy_nonoverlapping(shellcode.as_ptr(), new_buf_ptr, sc_len) };
    println!("[*] Starting jmp to shellcode at offset 0x{:x}", offset);
    unsafe {
        let jmp_target = new_buf.offset(offset as isize);
        if set_bp {
            asm!("int 3");
        }
        asm!("jmp {}",in(reg) jmp_target)
    };
}
