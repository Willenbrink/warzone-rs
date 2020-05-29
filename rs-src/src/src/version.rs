use ::libc;
#[no_mangle]
pub unsafe extern "C" fn version() -> *const libc::c_char {
    return b"2.0.4\x00" as *const u8 as *const libc::c_char;
}
