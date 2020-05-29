use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn PHYSFS_freeList(listVar: *mut libc::c_void);
    #[no_mangle]
    fn PHYSFS_enumerateFiles(dir: *const libc::c_char)
     -> *mut *mut libc::c_char;
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
}
pub type __int32_t = libc::c_int;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
               *(*__ctype_toupper_loc()).offset(__c as isize)
           } else { __c };
}
/* * \file ignorecase.c */
/* *
 * Please see ignorecase.h for details.
 *
 * License: this code is public domain. I make no warranty that it is useful,
 *  correct, harmless, or environmentally safe.
 *
 * This particular file may be used however you like, including copying it
 *  verbatim into a closed-source project, exploiting it commercially, and
 *  removing any trace of my name from the source (although I hope you won't
 *  do that). I welcome enhancements and corrections to this file, but I do
 *  not require you to send me patches if you make changes. This code has
 *  NO WARRANTY.
 *
 * Unless otherwise stated, the rest of PhysicsFS falls under the zlib license.
 *  Please see LICENSE in the root of the source tree.
 *
 *  \author Ryan C. Gordon.
 */
/* I'm not screwing around with stricmp vs. strcasecmp... */
unsafe extern "C" fn caseInsensitiveStringCompare(mut x: *const libc::c_char,
                                                  mut y: *const libc::c_char)
 -> libc::c_int {
    let mut ux: libc::c_int = 0;
    let mut uy: libc::c_int = 0;
    loop  {
        ux =
            ({
                 let mut __res: libc::c_int = 0;
                 if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >
                        1 as libc::c_int as libc::c_uint {
                     if 0 != 0 {
                         let mut __c: libc::c_int = *x as libc::c_int;
                         __res =
                             if __c < -(128 as libc::c_int) ||
                                    __c > 255 as libc::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else { __res = toupper(*x as libc::c_int) }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*x as libc::c_int as
                                                              isize)
                 }
                 __res
             });
        uy =
            ({
                 let mut __res: libc::c_int = 0;
                 if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong >
                        1 as libc::c_int as libc::c_uint {
                     if 0 != 0 {
                         let mut __c: libc::c_int = *y as libc::c_int;
                         __res =
                             if __c < -(128 as libc::c_int) ||
                                    __c > 255 as libc::c_int {
                                 __c
                             } else {
                                 *(*__ctype_toupper_loc()).offset(__c as
                                                                      isize)
                             }
                     } else { __res = toupper(*y as libc::c_int) }
                 } else {
                     __res =
                         *(*__ctype_toupper_loc()).offset(*y as libc::c_int as
                                                              isize)
                 }
                 __res
             });
        if ux != uy {
            return if ux > uy {
                       1 as libc::c_int
                   } else { -(1 as libc::c_int) }
        }
        x = x.offset(1);
        y = y.offset(1);
        if !(ux != 0 && uy != 0) { break ; }
    }
    return 0 as libc::c_int;
}
/* caseInsensitiveStringCompare */
unsafe extern "C" fn locateOneElement(mut buf: *mut libc::c_char)
 -> libc::c_int {
    let mut ptr: *mut libc::c_char =
        0 as *mut libc::c_char; /* quick rejection: exists in current case. */
    let mut rc: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; /* find entry at end of path. */
    let mut i: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; /* else */
    if PHYSFS_exists(buf) != 0 { return 1 as libc::c_int } /* if */
    ptr = strrchr(buf, '/' as i32);
    if ptr.is_null() {
        rc =
            PHYSFS_enumerateFiles(b"/\x00" as *const u8 as
                                      *const libc::c_char);
        ptr = buf
    } else {
        *ptr = '\u{0}' as i32 as libc::c_char;
        rc = PHYSFS_enumerateFiles(buf);
        *ptr = '/' as i32 as libc::c_char;
        ptr = ptr.offset(1)
        /* point past dirsep to entry itself. */
    } /* for */
    i = rc; /* found a match. Overwrite with this case. */
    while !(*i).is_null() {
        if caseInsensitiveStringCompare(*i, ptr) == 0 as libc::c_int {
            strcpy(ptr, *i);
            PHYSFS_freeList(rc as *mut libc::c_void);
            return 1 as libc::c_int
        }
        i = i.offset(1)
        /* if */
    }
    /* no match at all... */
    PHYSFS_freeList(rc as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* * \file ignorecase.h */
/* *
 * PhysicsFS ignorecase
 *
 * This is an extension to PhysicsFS to let you handle files in a
 *  case-insensitive manner, regardless of what sort of filesystem or
 *  archive they reside in. It does this by enumerating directories as
 *  needed and manually locating matching entries.
 *
 * Please note that this brings with it some caveats:
 *  - On filesystems that are case-insensitive to start with, such as those
 *    used on Windows or MacOS, you are adding extra overhead.
 *  - On filesystems that are case-sensitive, you might select the wrong dir
 *    or file (which brings security considerations and potential bugs). This
 *    code favours exact case matches, but you will lose access to otherwise
 *    duplicate filenames, or you might go down a wrong directory tree, etc.
 *    In practive, this is rarely a problem, but you need to be aware of it.
 *  - This doesn't do _anything_ with the write directory; you're on your
 *    own for opening the right files for writing. You can sort of get around
 *    this by adding your write directory to the search path, but then the
 *    interpolated directory tree can screw you up even more.
 *
 * This code should be considered an aid for legacy code. New development
 *  shouldn't do dumbass things that require this aid in the first place.  :)
 *
 * Usage: Set up PhysicsFS as you normally would, then use
 *  PHYSFSEXT_locateCorrectCase() to get a "correct" pathname to pass to
 *  functions like PHYSFS_openRead(), etc.
 *
 * License: this code is public domain. I make no warranty that it is useful,
 *  correct, harmless, or environmentally safe.
 *
 * This particular file may be used however you like, including copying it
 *  verbatim into a closed-source project, exploiting it commercially, and
 *  removing any trace of my name from the source (although I hope you won't
 *  do that). I welcome enhancements and corrections to this file, but I do
 *  not require you to send me patches if you make changes. This code has
 *  NO WARRANTY.
 *
 * Unless otherwise stated, the rest of PhysicsFS falls under the zlib license.
 *  Please see LICENSE in the root of the source tree.
 *
 *  \author Ryan C. Gordon.
 */
/* *
 * \fn int PHYSFSEXT_locateCorrectCase(char *buf)
 * \brief Find an existing filename with matching case.
 *
 * This function will look for a path/filename that matches the passed in
 *  buffer. Each element of the buffer's path is checked for a
 *  case-insensitive match. The buffer must specify a null-terminated string
 *  in platform-independent notation.
 *
 * Please note results may be skewed differently depending on whether symlinks
 *  are enabled or not.
 *
 * Each element of the buffer is overwritten with the actual case of an
 *  existing match. If there is no match, the search aborts and reports an
 *  error. Exact matches are favored over case-insensitive matches.
 *
 * THIS IS RISKY. Please do not use this function for anything but crappy
 *  legacy code.
 *
 *   \param buf Buffer with null-terminated string of path/file to locate.
 *               This buffer will be modified by this function.
 *  \return zero if match was found, -1 if the final element (the file itself)
 *               is missing, -2 if one of the parent directories is missing.
 */
/* locateOneElement */
#[no_mangle]
pub unsafe extern "C" fn PHYSFSEXT_locateCorrectCase(mut buf:
                                                         *mut libc::c_char)
 -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prevptr: *mut libc::c_char = 0 as *mut libc::c_char;
    while *buf as libc::c_int == '/' as i32 {
        /* skip any '/' at start of string... */
        buf = buf.offset(1)
    } /* Uh...I guess that's success. */
    prevptr = buf;
    ptr = prevptr;
    if *ptr as libc::c_int == '\u{0}' as i32 { return 0 as libc::c_int }
    loop 
         /* missing element in path. */
         {
        ptr =
            strchr(ptr.offset(1 as libc::c_int as isize),
                   '/' as i32); /* while */
        if ptr.is_null() {
            break ; /* block this path section off */
        } /* restore path separator */
        *ptr = '\u{0}' as i32 as libc::c_char;
        rc = locateOneElement(buf);
        *ptr = '/' as i32 as libc::c_char;
        if rc == 0 { return -(2 as libc::c_int) }
    }
    /* check final element... */
    return if locateOneElement(buf) != 0 {
               0 as libc::c_int
           } else { -(1 as libc::c_int) };
}
/* end of ignorecase.c ... */
/* PHYSFSEXT_locateCorrectCase */
