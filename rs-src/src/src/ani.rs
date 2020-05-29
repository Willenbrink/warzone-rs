use ::libc;
extern "C" {
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
}
pub type STRING = libc::c_char;
// FIXME Stubfile!
/* **************************************************************************/
/*
 * Ani.h
 *
 * Warzone animation function wrappers
 *
 * Gareth Jones 16/12/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 * Ani.c
 *
 * Warzone animation function wrappers
 *
 * Gareth Jones 16/12/97
 */
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_GetShapeFunc(mut pStr: *mut STRING)
 -> *mut libc::c_void {
    return resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                          *mut STRING, pStr);
}
/* **************************************************************************/
