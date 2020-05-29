use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* Close STREAM.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    /* Write formatted output to S from argument list ARG.

   This function is a possible cancellation point and therefore not
   marked with __THROW.  */
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: __builtin_va_list)
     -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    /* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
    /* ***************************************************************************************
 *
 * Basic debugging macro's
 *
 */
    /*
 *
 * ASSERT
 *
 * Rewritten version of assert that allows a printf format text string to be passed
 * to ASSERT along with the condition.
 *
 * Arguments:	ASSERT( condition, "Format string with variables: %d, %d", var1, var2 );
 */
    /* ***************************************************************************************
 *
 * Conditional debugging macro's that can be selectively turned on or off on a file
 * by file basis.
 *
 * Modified to not output nothing under no conditions
 *
 */
    /* **
 ***
 ***  New debug logging output interface below. Heavily inspired
 ***  by similar code in Freeciv. Parts ripped directly.
 ***
 ***/
    /* Want to use GCC's __attribute__ keyword to check variadic
 * parameters to printf-like functions, without upsetting other
 * compilers: put any required defines magic here.
 * If other compilers have something equivalent, could also
 * work that out here.   Should this use configure stuff somehow?
 * --dwp
 */
    /* Must match code_part_names in debug.c */
    /* special: sets all to on */
    /* special; on by default */
    /* if too verbose for anything but dedicated debugging... */
    /* _must_ be last! */
    // / Function which does the output
    // / Setup function
    // / Cleaning function
    // / Used to pass data to the above functions. Eg a filename or handle.
    /* *
 * Call once to initialize the debug logging system.
 *
 * Doesn't register any callbacks!
 */
    /* *
 * Shutdown the debug system and remove all output callbacks
 */
    /* *
 * Register a callback to be called on every call to debug()
 *
 * \param	callback	Function which does the output
 * \param	init		Initializer function which does all setup for the callback (optional, may be NULL)
 * \param	exit		Cleanup function called when unregistering the callback (optional, may be NULL)
 * \param	data		Data to be passed to all three functions (optional, may be NULL)
 */
    /* *
 * Toggle debug output for part associated with str
 *
 * \param	str	Codepart in textformat
 */
    /* *
 * Output printf style format str with additional arguments.
 *
 * Only outputs if debugging of part was formerly enabled with debug_enable_switch.
 *
 * \param	part	Code part to associate with this message
 * \param	str		printf style formatstring
 */
    #[no_mangle]
    fn debug(part: code_part, str: *const libc::c_char, _: ...);
    /*
 * Frame.h
 *
 * The framework library initialisation and shutdown routines.
 *
 */
    /* Linux specific stuff */
    #[no_mangle]
    fn unix_fopen(filename: *const libc::c_char, mode: *const libc::c_char)
     -> *mut FILE;
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
pub type code_part = libc::c_uint;
pub const LOG_LAST: code_part = 12;
pub const LOG_SCRIPT: code_part = 11;
pub const LOG_NEVER: code_part = 10;
pub const LOG_ERROR: code_part = 9;
pub const LOG_MEMORY: code_part = 8;
pub const LOG_NET: code_part = 7;
pub const LOG_TEXTURE: code_part = 6;
pub const LOG_3D: code_part = 5;
pub const LOG_WZ: code_part = 4;
pub const LOG_VIDEO: code_part = 3;
pub const LOG_SOUND: code_part = 2;
pub const LOG_MAIN: code_part = 1;
pub const LOG_ALL: code_part = 0;
//*************************************************************************
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn _debug_create_log() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp =
        unix_fopen(b"IVI.LOG\x00" as *const u8 as *const libc::c_char,
                   b"w\x00" as *const u8 as *const libc::c_char);
    if !fp.is_null() { fclose(fp); };
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_Debug(mut string: *mut libc::c_char,
                                  mut args: ...) {
    let mut argptr: va_list = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp =
        unix_fopen(b"IVI.LOG\x00" as *const u8 as *const libc::c_char,
                   b"a\x00" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        argptr = args.clone();
        vfprintf(fp, string, argptr);
        fclose(fp);
    };
}
//*************************************************************************
//***	ivi.h iVi engine definitions. [Sam Kerbeck]
//*	24-04-96.30-07-96 PC
//*	18-11-96.04-12-96	WIN'95
//*
// Simple derived types
// If its a final build we need to undefine iv_error so that it doesn't generate any code !
//*************************************************************************
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
//* Macros:
//*
// added by TJC
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
// Basic type (replace with framework definitions)
//*************************************************************************
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_DisplayLogFile() {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut c: libc::c_int = 0;
    fp =
        unix_fopen(b"IVI.LOG\x00" as *const u8 as *const libc::c_char,
                   b"r\x00" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        loop  {
            c = getc(fp);
            if !(c != -(1 as libc::c_int)) { break ; }
            debug(LOG_NEVER, b"%c\x00" as *const u8 as *const libc::c_char,
                  c);
        }
        fclose(fp);
    };
}
