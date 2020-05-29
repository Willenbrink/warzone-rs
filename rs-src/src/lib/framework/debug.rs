use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* Call all functions registered with `atexit' and `on_exit',
   in the reverse of the order in which they were registered,
   perform stdio cleanup, and terminate program execution with STATUS.  */
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn unix_fopen(filename: *const libc::c_char, mode: *const libc::c_char)
     -> *mut FILE;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_uint,
                 _: *const libc::c_char, _: __builtin_va_list) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_uint, _: *const libc::c_char,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type __int64_t = libc::c_longlong;
pub type __off_t = libc::c_long;
pub type __off64_t = __int64_t;
pub type size_t = libc::c_uint;
pub type va_list = __builtin_va_list;
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
pub type BOOL = libc::c_int;
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
pub type debug_callback_fn
    =
    Option<unsafe extern "C" fn(_: *mut *mut libc::c_void,
                                _: *const libc::c_char) -> ()>;
pub type debug_callback_init
    =
    Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;
pub type debug_callback_exit
    =
    Option<unsafe extern "C" fn(_: *mut *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _debug_callback {
    pub next: *mut _debug_callback,
    pub callback: debug_callback_fn,
    pub init: debug_callback_init,
    pub exit: debug_callback_exit,
    pub data: *mut libc::c_void,
}
pub type debug_callback = _debug_callback;
static mut callbackRegistry: *mut debug_callback =
    0 as *const debug_callback as *mut debug_callback;
static mut enabled_debug_parts: [BOOL; 12] = [0; 12];
/* This list _must_ match the enum in debug.h! */
static mut code_part_names: [*const libc::c_char; 13] =
    [b"all\x00" as *const u8 as *const libc::c_char,
     b"main\x00" as *const u8 as *const libc::c_char,
     b"sound\x00" as *const u8 as *const libc::c_char,
     b"video\x00" as *const u8 as *const libc::c_char,
     b"wz\x00" as *const u8 as *const libc::c_char,
     b"3d\x00" as *const u8 as *const libc::c_char,
     b"texture\x00" as *const u8 as *const libc::c_char,
     b"net\x00" as *const u8 as *const libc::c_char,
     b"memory\x00" as *const u8 as *const libc::c_char,
     b"error\x00" as *const u8 as *const libc::c_char,
     b"never\x00" as *const u8 as *const libc::c_char,
     b"script\x00" as *const u8 as *const libc::c_char,
     b"last\x00" as *const u8 as *const libc::c_char];
/* *********************************************************************
 cat_snprintf is like a combination of snprintf and strlcat;
 it does snprintf to the end of an existing string.

 Like mystrlcat, n is the total length available for str, including
 existing contents and trailing nul.  If there is no extra room
 available in str, does not change the string.

 Also like mystrlcat, returns the final length that str would have
 had without truncation.  I.e., if return is >= n, truncation occurred.
**********************************************************************/
unsafe extern "C" fn cat_snprintf(mut str: *mut libc::c_char, mut n: size_t,
                                  mut format: *const libc::c_char,
                                  mut args: ...) -> libc::c_int {
    let mut len: size_t = 0;
    let mut ret: libc::c_int = 0;
    let mut ap: va_list = 0 as *mut libc::c_char;
    len = strlen(str);
    ap = args.clone();
    ret =
        vsnprintf(str.offset(len as isize), n.wrapping_sub(len), format, ap);
    return (ret as libc::c_uint).wrapping_add(len) as libc::c_int;
}
/* *
 * Convert code_part names to enum. Case insensitive.
 *
 * \return	Codepart number or LOG_LAST if can't match.
 */
unsafe extern "C" fn code_part_from_str(mut str: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < LOG_LAST as libc::c_int {
        if strcasecmp(code_part_names[i as usize], str) == 0 as libc::c_int {
            return i
        }
        i += 1
    }
    return LOG_LAST as libc::c_int;
}
/* *
 * Callback for outputing to stderr
 *
 * \param	data			Ignored. Use NULL.
 * \param	outputBuffer	Buffer containing the preprocessed text to output.
 */
#[no_mangle]
pub unsafe extern "C" fn debug_callback_stderr(mut data:
                                                   *mut *mut libc::c_void,
                                               mut outputBuffer:
                                                   *const libc::c_char) {
    if strchr(outputBuffer, '\n' as i32).is_null() {
        fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char,
                outputBuffer);
    } else {
        fprintf(stderr, b"%s\x00" as *const u8 as *const libc::c_char,
                outputBuffer);
    };
}
/* *
 * Callback for outputting to a win32 debugger
 *
 * \param	data			Ignored. Use NULL.
 * \param	outputBuffer	Buffer containing the preprocessed text to output.
 */
#[no_mangle]
pub unsafe extern "C" fn debug_callback_win32debug(mut data:
                                                       *mut *mut libc::c_void,
                                                   mut outputBuffer:
                                                       *const libc::c_char) {
    // WIN32
}
/* *
 * Callback for outputing to a file
 *
 * \param	data			Filehandle to output to.
 * \param	outputBuffer	Buffer containing the preprocessed text to output.
 */
#[no_mangle]
pub unsafe extern "C" fn debug_callback_file(mut data: *mut *mut libc::c_void,
                                             mut outputBuffer:
                                                 *const libc::c_char) {
    let mut logfile: *mut FILE = *data as *mut FILE;
    if strchr(outputBuffer, '\n' as i32).is_null() {
        fprintf(logfile, b"%s\n\x00" as *const u8 as *const libc::c_char,
                outputBuffer);
    } else {
        fprintf(logfile, b"%s\x00" as *const u8 as *const libc::c_char,
                outputBuffer);
    };
}
/* *
 * Setup the file callback
 *
 * Sets data to the filehandle opened for the filename found in data.
 *
 * \param[in,out]	data	In: 	The filename to output to.
 * 							Out:	The filehandle.
 */
#[no_mangle]
pub unsafe extern "C" fn debug_callback_file_init(mut data:
                                                      *mut *mut libc::c_void) {
    let mut filename: *const libc::c_char = *data as *const libc::c_char;
    let mut logfile: *mut FILE = 0 as *mut FILE;
    logfile =
        unix_fopen(filename, b"a\x00" as *const u8 as *const libc::c_char);
    if logfile.is_null() {
        fprintf(stderr,
                b"Could not open %s for appending!\n\x00" as *const u8 as
                    *const libc::c_char, filename);
    } else {
        setbuf(logfile, 0 as *mut libc::c_char);
        fprintf(logfile,
                b"\n--- Starting log ---\n\x00" as *const u8 as
                    *const libc::c_char);
        *data = logfile as *mut libc::c_void
    };
}
/* *
 * Shutdown the file callback
 *
 * Closes the logfile.
 *
 * \param	data	The filehandle to close.
 */
#[no_mangle]
pub unsafe extern "C" fn debug_callback_file_exit(mut data:
                                                      *mut *mut libc::c_void) {
    let mut logfile: *mut FILE = *data as *mut FILE;
    fclose(logfile);
    *data = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn debug_init() {
    let mut count: libc::c_int = 0 as libc::c_int;
    while strcmp(code_part_names[count as usize],
                 b"last\x00" as *const u8 as *const libc::c_char) !=
              0 as libc::c_int {
        count += 1
    }
    if count != LOG_LAST as libc::c_int {
        fprintf(stderr,
                b"LOG_LAST != last; whoever edited the debug code last did a mistake.\n\x00"
                    as *const u8 as *const libc::c_char);
        fprintf(stderr,
                b"Always edit both the enum in debug.h and the string list in debug.c!\n\x00"
                    as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(enabled_debug_parts.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[BOOL; 12]>() as libc::c_ulong);
    enabled_debug_parts[LOG_ERROR as libc::c_int as usize] = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn debug_exit() {
    let mut curCallback: *mut debug_callback = callbackRegistry;
    let mut tmpCallback: *mut debug_callback = 0 as *mut debug_callback;
    while !curCallback.is_null() {
        if (*curCallback).exit.is_some() {
            (*curCallback).exit.expect("non-null function pointer")(&mut (*curCallback).data);
        }
        tmpCallback = (*curCallback).next;
        free(curCallback as *mut libc::c_void);
        curCallback = tmpCallback
    }
    callbackRegistry = 0 as *mut debug_callback;
}
#[no_mangle]
pub unsafe extern "C" fn debug_register_callback(mut callback:
                                                     debug_callback_fn,
                                                 mut init:
                                                     debug_callback_init,
                                                 mut exit_0:
                                                     debug_callback_exit,
                                                 mut data:
                                                     *mut libc::c_void) {
    let mut curCallback: *mut debug_callback = callbackRegistry;
    let mut tmpCallback: *mut debug_callback = 0 as *mut debug_callback;
    tmpCallback =
        malloc(::std::mem::size_of::<debug_callback>() as libc::c_ulong) as
            *mut debug_callback;
    (*tmpCallback).next = 0 as *mut _debug_callback;
    (*tmpCallback).callback = callback;
    (*tmpCallback).init = init;
    (*tmpCallback).exit = exit_0;
    (*tmpCallback).data = data;
    if (*tmpCallback).init.is_some() {
        (*tmpCallback).init.expect("non-null function pointer")(&mut (*tmpCallback).data);
    }
    if curCallback.is_null() { callbackRegistry = tmpCallback; return }
    while !(*curCallback).next.is_null() { curCallback = (*curCallback).next }
    (*curCallback).next = tmpCallback;
}
#[no_mangle]
pub unsafe extern "C" fn debug_enable_switch(mut str: *const libc::c_char)
 -> BOOL {
    let mut part: libc::c_int = code_part_from_str(str);
    if part != LOG_LAST as libc::c_int {
        enabled_debug_parts[part as usize] =
            (enabled_debug_parts[part as usize] == 0) as libc::c_int
    }
    if part == LOG_ALL as libc::c_int {
        memset(enabled_debug_parts.as_mut_ptr() as *mut libc::c_void,
               1 as libc::c_int,
               ::std::mem::size_of::<[BOOL; 12]>() as libc::c_ulong);
    }
    return (part != LOG_LAST as libc::c_int) as libc::c_int;
}
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
pub unsafe extern "C" fn debug(mut part: code_part,
                               mut str: *const libc::c_char, mut args: ...) {
    let mut ap: va_list =
        0 as *mut libc::c_char; /* times current message repeated */
    static mut inputBuffer: [[libc::c_char; 512]; 2] =
        [[0; 512]; 2]; /* next total to print update */
    static mut outputBuffer: [libc::c_char; 524] =
        [0; 524]; /* total on last update */
    static mut useInputBuffer1: BOOL = 0 as libc::c_int;
    let mut curCallback: *mut debug_callback = callbackRegistry;
    static mut repeated: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    static mut next: libc::c_uint = 2 as libc::c_int as libc::c_uint;
    static mut prev: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    /* Not enabled debugging for this part? Punt! */
    if enabled_debug_parts[part as usize] == 0 { return }
    ap = args.clone();
    vsnprintf(if useInputBuffer1 != 0 {
                  inputBuffer[1 as libc::c_int as usize].as_mut_ptr()
              } else { inputBuffer[0 as libc::c_int as usize].as_mut_ptr() },
              512 as libc::c_int as libc::c_uint, str, ap);
    if strncmp(inputBuffer[0 as libc::c_int as usize].as_mut_ptr(),
               inputBuffer[1 as libc::c_int as usize].as_mut_ptr(),
               (512 as libc::c_int - 1 as libc::c_int) as libc::c_uint) ==
           0 as libc::c_int {
        // Recieved again the same line
        repeated = repeated.wrapping_add(1);
        if repeated == next {
            snprintf(outputBuffer.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 524]>() as
                         libc::c_ulong,
                     b"last message repeated %d times\x00" as *const u8 as
                         *const libc::c_char, repeated.wrapping_sub(prev));
            if repeated > 2 as libc::c_int as libc::c_uint {
                cat_snprintf(outputBuffer.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 524]>() as
                                 libc::c_ulong,
                             b" (total %d repeats)\x00" as *const u8 as
                                 *const libc::c_char, repeated);
            }
            while !curCallback.is_null() {
                (*curCallback).callback.expect("non-null function pointer")(&mut (*curCallback).data,
                                                                            outputBuffer.as_mut_ptr());
                curCallback = (*curCallback).next
            }
            curCallback = callbackRegistry;
            prev = repeated;
            next = next.wrapping_mul(2 as libc::c_int as libc::c_uint)
        }
    } else {
        // Recieved another line, cleanup the old
        if repeated > 0 as libc::c_int as libc::c_uint && repeated != prev &&
               repeated != 1 as libc::c_int as libc::c_uint {
            /* just repeat the previous message when only one repeat occured */
            snprintf(outputBuffer.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 524]>() as
                         libc::c_ulong,
                     b"last message repeated %d times\x00" as *const u8 as
                         *const libc::c_char, repeated.wrapping_sub(prev));
            if repeated > 2 as libc::c_int as libc::c_uint {
                cat_snprintf(outputBuffer.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 524]>() as
                                 libc::c_ulong,
                             b" (total %d repeats)\x00" as *const u8 as
                                 *const libc::c_char, repeated);
            }
            while !curCallback.is_null() {
                (*curCallback).callback.expect("non-null function pointer")(&mut (*curCallback).data,
                                                                            outputBuffer.as_mut_ptr());
                curCallback = (*curCallback).next
            }
            curCallback = callbackRegistry
        }
        repeated = 0 as libc::c_int as libc::c_uint;
        next = 2 as libc::c_int as libc::c_uint;
        prev = 0 as libc::c_int as libc::c_uint
    }
    if repeated == 0 {
        // Assemble the outputBuffer:
        sprintf(outputBuffer.as_mut_ptr(),
                b"%s:\x00" as *const u8 as *const libc::c_char,
                code_part_names[part as usize]); // Fill with whitespaces
        memset(outputBuffer.as_mut_ptr().offset(strlen(code_part_names[part as
                                                                           usize])
                                                    as
                                                    isize).offset(1 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                   as *mut libc::c_void, ' ' as i32,
               (12 as libc::c_int as
                    libc::c_uint).wrapping_sub(strlen(code_part_names[part as
                                                                          usize])).wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)); // Append the message
        snprintf(outputBuffer.as_mut_ptr().offset(12 as libc::c_int as isize),
                 512 as libc::c_int as libc::c_uint,
                 if useInputBuffer1 != 0 {
                     inputBuffer[1 as libc::c_int as usize].as_mut_ptr()
                 } else {
                     inputBuffer[0 as libc::c_int as usize].as_mut_ptr()
                 });
        while !curCallback.is_null() {
            (*curCallback).callback.expect("non-null function pointer")(&mut (*curCallback).data,
                                                                        outputBuffer.as_mut_ptr());
            curCallback = (*curCallback).next
        }
    }
    useInputBuffer1 = (useInputBuffer1 == 0) as libc::c_int;
    // Swap buffers
}
