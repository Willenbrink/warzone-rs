use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* Standard input stream.  */
    #[no_mangle]
    static mut stdout: *mut FILE;
    /* Write formatted output to S from argument list ARG.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: __builtin_va_list)
     -> libc::c_int;
    /* Write formatted output to S from argument list ARG.  */
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: __builtin_va_list) -> libc::c_int;
    /* Call all functions registered with `atexit' and `on_exit',
   in the reverse of the order in which they were registered,
   perform stdio cleanup, and terminate program execution with STATUS.  */
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn iV_VideoMemoryUnlock();
    #[no_mangle]
    fn iV_ClearFonts();
    #[no_mangle]
    fn pie_ShutDown();
    //*************************************************************************
    #[no_mangle]
    static mut _TEX_INDEX: libc::c_int;
    //*************************************************************************
    #[no_mangle]
    fn pie_TexShutDown();
}
pub type __builtin_va_list = *mut libc::c_char;
pub type size_t = libc::c_uint;
/* Define ISO C stdio on top of C++ iostreams.
   Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
/*
 *	ISO C99 Standard: 7.19 Input/output	<stdio.h>
 */
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int64_t = libc::c_longlong;
pub type __off_t = libc::c_long;
pub type __off64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 40],
}
pub type _IO_lock_t = ();
/* The opaque type of streams.  This is the definition used elsewhere.  */
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iError {
    pub n: libc::c_long,
    pub msge: [libc::c_char; 240],
}
/* Write formatted output to stdout from argument list ARG.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
/* Optimizing macros and inline functions for stdio functions.
   Copyright (C) 1998-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
/* For -D_FORTIFY_SOURCE{,=2} bits/stdio2.h will define a different
   inline.  */
/* Write formatted output to stdout from argument list ARG.  */
#[inline]
unsafe extern "C" fn vprintf(mut __fmt: *const libc::c_char,
                             mut __arg: __gnuc_va_list) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg);
}
//*************************************************************************
#[no_mangle]
pub static mut _iVERROR: iError = iError{n: 0, msge: [0; 240],};
//*************************************************************************
//*************************************************************************
//*************************************************************************
// pass in true to reset the palette too.
#[no_mangle]
pub unsafe extern "C" fn iV_Reset(mut bPalReset: libc::c_int) {
    _TEX_INDEX = 0 as libc::c_int;
    iV_ClearFonts();
    // Initialise the IVIS font module.
}
#[no_mangle]
pub unsafe extern "C" fn iV_ShutDown() {
    pie_ShutDown();
    iV_VideoMemoryUnlock();
    pie_TexShutDown();
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_Stop(mut string: *mut libc::c_char,
                                 mut args: ...) {
    let mut argptr: va_list = 0 as *mut libc::c_char;
    iV_ShutDown();
    argptr = args.clone();
    vprintf(string, argptr);
    exit(0 as libc::c_int);
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_Abort(mut string: *mut libc::c_char,
                                  mut args: ...) {
    let mut argptr: va_list = 0 as *mut libc::c_char;
    argptr = args.clone();
    vprintf(string, argptr);
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_Error(mut errorn: libc::c_long,
                                  mut msge: *mut libc::c_char,
                                  mut args: ...) {
    let mut argptr: va_list = 0 as *mut libc::c_char;
    argptr = args.clone();
    vsprintf(&mut *_iVERROR.msge.as_mut_ptr().offset(0 as libc::c_int as
                                                         isize), msge,
             argptr);
    _iVERROR.n = errorn;
}
