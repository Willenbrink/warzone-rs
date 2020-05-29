use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn asctime(__tp: *const tm) -> *mut libc::c_char;
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_flush(handle: *mut PHYSFS_File) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
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
pub type code_part = libc::c_uint;
pub const LOG_LAST: code_part = 12;
/* _must_ be last! */
/* if too verbose for anything but dedicated debugging... */
pub const LOG_SCRIPT: code_part = 11;
/* special; on by default */
pub const LOG_NEVER: code_part = 10;
pub const LOG_ERROR: code_part = 9;
pub const LOG_MEMORY: code_part = 8;
pub const LOG_NET: code_part = 7;
pub const LOG_TEXTURE: code_part = 6;
pub const LOG_3D: code_part = 5;
pub const LOG_WZ: code_part = 4;
pub const LOG_VIDEO: code_part = 3;
pub const LOG_SOUND: code_part = 2;
/* special: sets all to on */
pub const LOG_MAIN: code_part = 1;
pub const LOG_ALL: code_part = 0;
pub type PHYSFS_sint64 = libc::c_longlong;
pub type PHYSFS_uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
// ////////////////////////////////////////////////////////////////////////
// Includes
// ////////////////////////////////////////////////////////////////////////
// Logging for degug only
// ////////////////////////////////////////////////////////////////////////
static mut pFileHandle: *mut PHYSFS_File =
    0 as *const PHYSFS_File as *mut PHYSFS_File;
#[no_mangle]
pub unsafe extern "C" fn NETstartLogging() -> BOOL {
    let mut aclock: time_t = 0; /* Get time in seconds */
    let mut newtime: *mut tm = 0 as *mut tm; /* Convert time to struct */
    let mut buf: [libc::c_char; 256] = [0; 256]; // open the file
    time(&mut aclock); /* Get time in seconds */
    newtime = localtime(&mut aclock); /* Convert time to struct */
    pFileHandle =
        PHYSFS_openWrite(b"netplay.log\x00" as *const u8 as
                             *const libc::c_char);
    if pFileHandle.is_null() { return 0 as libc::c_int }
    sprintf(buf.as_mut_ptr(),
            b"NETPLAY log: %s\n\x00" as *const u8 as *const libc::c_char,
            asctime(newtime));
    PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                 strlen(buf.as_mut_ptr()), 1 as libc::c_int as PHYSFS_uint32);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NETstopLogging() -> BOOL {
    if PHYSFS_close(pFileHandle) == 0 {
        debug(LOG_ERROR,
              b"Could not close net log: %s\x00" as *const u8 as
                  *const libc::c_char, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NETlogEntry(mut str: *mut CHAR, mut a: UDWORD,
                                     mut b: UDWORD) -> BOOL {
    static mut lastframe: UDWORD = 0 as libc::c_int as UDWORD;
    let mut frame: UDWORD = frameGetFrameNumber();
    let mut aclock: time_t = 0;
    let mut newtime: *mut tm = 0 as *mut tm;
    let mut buf: [libc::c_char; 256] = [0; 256];
    if a == 9 as libc::c_int as libc::c_uint ||
           a == 10 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    time(&mut aclock);
    newtime = localtime(&mut aclock);
    // check to see if a new frame.
    if frame != lastframe {
        lastframe = frame;
        sprintf(buf.as_mut_ptr(),
                b"-----------------------------------------------------------\n\x00"
                    as *const u8 as *const libc::c_char);
        PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                     strlen(buf.as_mut_ptr()),
                     1 as libc::c_int as PHYSFS_uint32);
    }
    match a {
        1 => {
            // replace common msgs with txt descriptions
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_DROIDINFO  \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        2 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_DROIDDEST  \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        3 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_DROIDMOVE  \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        4 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_GROUPORDER  \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        8 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_PING \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        9 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_CHECK_DROID \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        10 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_CHECK_STRUCT \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        11 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_CHECK_POWER \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        13 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_BUILD \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        15 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_BUILDFINISHED \t:%d\t\t%s\x00" as *const u8
                        as *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        17 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_TXTMSG \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        18 => {
            sprintf(buf.as_mut_ptr(),
                    b"************************************************************\n\x00"
                        as *const u8 as *const libc::c_char);
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_LEAVING \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
            sprintf(buf.as_mut_ptr(),
                    b"************************************************************\n\x00"
                        as *const u8 as *const libc::c_char);
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        19 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_REQUESTDROID \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        23 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_WHOLEDROID \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        22 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_STRUCT (Whole) \t:%d\t\t%s\x00" as *const u8
                        as *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        25 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_PLAYERRESPONDING \t:%d\t\t%s\x00" as
                        *const u8 as *const libc::c_char, str, b,
                    asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        26 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_OPTIONS \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        27 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_WAYPOINT \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        28 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_SECONDARY \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        29 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_FIREUP \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        34 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_ARTIFACTS \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        36 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_SCORESUBMIT \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        37 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_DESTROYXTRA \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        38 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_VTOL \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        39 => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t: NET_VTOLREARM \t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
        _ => {
            sprintf(buf.as_mut_ptr(),
                    b"%s \t:%d \t\t\t:%d\t\t%s\x00" as *const u8 as
                        *const libc::c_char, str, a, b, asctime(newtime));
            PHYSFS_write(pFileHandle, buf.as_mut_ptr() as *const libc::c_void,
                         strlen(buf.as_mut_ptr()),
                         1 as libc::c_int as PHYSFS_uint32);
        }
    }
    PHYSFS_flush(pFileHandle);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
