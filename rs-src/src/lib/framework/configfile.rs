use ::libc;
extern "C" {
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn saveFile(pFileName: *const libc::c_char,
                pFileData: *const libc::c_char, fileSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_uint, _: *const libc::c_char,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
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
    /* *
 * \fn int PHYSFS_exists(const char *fname)
 * \brief Determine if a file exists in the search path.
 *
 * Reports true if there is an entry anywhere in the search path by the
 *  name of (fname).
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, so you
 *  might end up further down in the search path than expected.
 *
 *    \param fname filename in platform-independent notation.
 *   \return non-zero if filename exists. zero otherwise.
 */
    #[no_mangle]
    fn PHYSFS_exists(fname: *const libc::c_char) -> libc::c_int;
}
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
pub type UCHAR = libc::c_uchar;
pub type DWORD = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regkey_t {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut regkey_t,
}
#[no_mangle]
pub static mut registry: [*mut regkey_t; 32] =
    [0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t,
     0 as *const regkey_t as *mut regkey_t];
#[no_mangle]
pub static mut RegFilePath: [libc::c_char; 260] = [0; 260];
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_clear() {
    //~~~~~~~~~~~~~~
    let mut i: libc::c_uint = 0;
    //~~~~~~~~~~~~~~
    i = 0 as libc::c_int as libc::c_uint;
    while i < 32 as libc::c_int as libc::c_uint {
        //~~~~~~~~~~~~~
        let mut j: *mut regkey_t = 0 as *mut regkey_t;
        let mut tmp: *mut regkey_t = 0 as *mut regkey_t;
        //~~~~~~~~~~~~~
        j = registry[i as usize];
        while !j.is_null() {
            tmp = (*j).next;
            free((*j).key as *mut libc::c_void);
            free((*j).value as *mut libc::c_void);
            free(j as *mut libc::c_void);
            j = tmp
        }
        registry[i as usize] = 0 as *mut regkey_t;
        i = i.wrapping_add(1)
    };
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_hash(mut s: *const libc::c_char)
 -> libc::c_uint {
    //~~~~~~~~~~~~~~~~~~
    let mut i: libc::c_uint = 0;
    let mut h: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    //~~~~~~~~~~~~~~~~~~
    if !s.is_null() {
        i = 0 as libc::c_int as libc::c_uint;
        while *s.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
            h = h.wrapping_add(*s.offset(i as isize) as libc::c_uint);
            i = i.wrapping_add(1)
        }
    }
    return h.wrapping_rem(32 as libc::c_int as libc::c_uint);
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_find_key(mut k: *const libc::c_char)
 -> *mut regkey_t {
    //~~~~~~~~~~~
    let mut i: *mut regkey_t = 0 as *mut regkey_t;
    //~~~~~~~~~~~
    i = registry[registry_hash(k) as usize];
    while !i.is_null() {
        if strcmp(k, (*i).key) == 0 { return i }
        i = (*i).next
    }
    return 0 as *mut regkey_t;
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_get_key(mut k: *const libc::c_char)
 -> *mut libc::c_char {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut key: *mut regkey_t = registry_find_key(k);
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if key.is_null() {
        //
		// printf("registry_get_key(%s) -> key not found\n", k);
		//
        return 0 as *mut libc::c_char
    } else {
        //
		// printf("registry_get_key(%s) -> %s\n", k, key->value);
		//
        return (*key).value
    };
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_set_key(mut k: *const libc::c_char,
                                          mut v: *const libc::c_char) {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut key: *mut regkey_t = registry_find_key(k);
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
	// printf("registry_set_key(%s, %s)\n", k, v);
	//
    if key.is_null() {
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        let mut h: libc::c_uint = registry_hash(k);
        //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        key =
            malloc(::std::mem::size_of::<regkey_t>() as libc::c_ulong) as
                *mut regkey_t;
        (*key).key = strdup(k);
        (*key).next = registry[h as usize];
        registry[h as usize] = key
    } else { free((*key).value as *mut libc::c_void); }
    (*key).value = strdup(v);
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_load(mut filename: *mut libc::c_char)
 -> BOOL {
    let mut buffer: [libc::c_char; 255] = [0; 255];
    let mut bptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufstart: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: [libc::c_char; 32] = [0; 32];
    let mut l: libc::c_uint = 0;
    let mut filesize: UDWORD = 0;
    debug(LOG_WZ,
          b"Loading the registry from %s\x00" as *const u8 as
              *const libc::c_char, filename);
    if PHYSFS_exists(filename) != 0 {
        if loadFile(filename, &mut bptr, &mut filesize) == 0 {
            return 0 as libc::c_int
            // happens only in NDEBUG case
        }
    } else {
        // Registry does not exist. Caller will write a new one.
        return 0 as libc::c_int
    } // make sure it is terminated
    debug(LOG_WZ,
          b"Parsing the registry from %s\x00" as *const u8 as
              *const libc::c_char, filename);
    if filesize == 0 as libc::c_int as libc::c_uint ||
           strlen(bptr) == 0 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"Registry file %s is empty!\x00" as *const u8 as
                  *const libc::c_char, filename);
        return 0 as libc::c_int
    }
    bufstart = bptr;
    *bptr.offset(filesize.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                     isize) = '\u{0}' as i32 as libc::c_char;
    while *bptr as libc::c_int != '\u{0}' as i32 {
        let mut count: libc::c_int = 0 as libc::c_int;
        /* Put a line into buffer */
        while *bptr as libc::c_int != '\u{0}' as i32 &&
                  *bptr as libc::c_int != '\n' as i32 &&
                  count < 255 as libc::c_int {
            buffer[count as usize] = *bptr;
            bptr = bptr.offset(1);
            count += 1
        }
        if *bptr as libc::c_int != '\u{0}' as i32 {
            bptr = bptr.offset(1)
            // skip EOL
        }
        buffer[count as usize] = '\u{0}' as i32 as libc::c_char;
        if sscanf(buffer.as_mut_ptr(),
                  b" %[^=] = %n\x00" as *const u8 as *const libc::c_char,
                  key.as_mut_ptr(), &mut l as *mut libc::c_uint) ==
               1 as libc::c_int {
            let mut i: libc::c_uint = 0;
            i = l;
            while !(buffer[i as usize] as libc::c_int == '\u{0}' as i32) {
                if (buffer[i as usize] as libc::c_int) < ' ' as i32 {
                    buffer[i as usize] = '\u{0}' as i32 as libc::c_char;
                    break ;
                } else { i = i.wrapping_add(1) }
            }
            registry_set_key(key.as_mut_ptr(),
                             buffer.as_mut_ptr().offset(l as isize));
        }
    }
    memFreeRelease(bufstart as *mut libc::c_void);
    bufstart = 0 as *mut libc::c_char;
    return 1 as libc::c_int;
}
//
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn registry_save(mut filename: *mut libc::c_char)
 -> BOOL {
    let mut buffer: [libc::c_char; 8160] = [0; 8160];
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    debug(LOG_WZ,
          b"Saving the registry to %s\x00" as *const u8 as
              *const libc::c_char, filename);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 32 as libc::c_int as libc::c_uint {
        let mut j: *mut regkey_t = 0 as *mut regkey_t;
        j = registry[i as usize];
        while !j.is_null() {
            let mut linebuf: [libc::c_char; 255] = [0; 255];
            snprintf(linebuf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 255]>() as
                         libc::c_ulong,
                     b"%s=%s\n\x00" as *const u8 as *const libc::c_char,
                     (*j).key, (*j).value);
            memcpy(buffer.as_mut_ptr().offset(count as isize) as
                       *mut libc::c_void,
                   linebuf.as_mut_ptr() as *const libc::c_void,
                   strlen(linebuf.as_mut_ptr()));
            count =
                (count as
                     libc::c_uint).wrapping_add(strlen(linebuf.as_mut_ptr()))
                    as libc::c_int as libc::c_int;
            j = (*j).next
        }
        i = i.wrapping_add(1)
    }
    if saveFile(filename, buffer.as_mut_ptr(), count as UDWORD) == 0 {
        return 0 as libc::c_int
        // only in NDEBUG case
    }
    return 1 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn openWarzoneKey() -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~
    static mut done: BOOL = 0 as libc::c_int;
    //~~~~~~~~~~~~~~~~~~~~~
    if done == 0 as libc::c_int {
        registry_load(RegFilePath.as_mut_ptr());
        done = 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn closeWarzoneKey() -> BOOL {
    registry_save(RegFilePath.as_mut_ptr());
    return 1 as libc::c_int;
}
/*
 * config.h
 * load and save favourites to the registry.
 */
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn getWarzoneKeyNumeric(mut pName: *mut STRING,
                                              mut val: *mut DWORD) -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut value: *mut libc::c_char = registry_get_key(pName);
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if value.is_null() ||
           sscanf(value, b"%i\x00" as *const u8 as *const libc::c_char, val)
               != 1 as libc::c_int {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn getWarzoneKeyString(mut pName: *mut STRING,
                                             mut pString: *mut STRING)
 -> BOOL {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let mut value: *mut libc::c_char = registry_get_key(pName);
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    if value.is_null() {
        return 0 as libc::c_int
    } else { strcpy(pString, value); }
    return 1 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn getWarzoneKeyBinary(mut pName: *mut STRING,
                                             mut pData: *mut UCHAR,
                                             mut pSize: *mut UDWORD) -> BOOL {
    return 0 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn setWarzoneKeyNumeric(mut pName: *mut STRING,
                                              mut val: DWORD) -> BOOL {
    //~~~~~~~~~~~~
    let mut buf: [libc::c_char; 32] = [0; 32];
    //~~~~~~~~~~~~
    sprintf(buf.as_mut_ptr(), b"%i\x00" as *const u8 as *const libc::c_char,
            val);
    registry_set_key(pName, buf.as_mut_ptr());
    return 1 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn setWarzoneKeyString(mut pName: *mut STRING,
                                             mut pString: *mut STRING)
 -> BOOL {
    registry_set_key(pName, pString);
    return 1 as libc::c_int;
}
// /
// =======================================================================================================================
// =======================================================================================================================
//
#[no_mangle]
pub unsafe extern "C" fn setWarzoneKeyBinary(mut pName: *mut STRING,
                                             mut pData: *mut libc::c_void,
                                             mut size: UDWORD) -> BOOL {
    return 0 as libc::c_int;
}
