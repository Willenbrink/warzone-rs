use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn adpcm_decode(input: *mut libc::c_uchar, input_size: libc::c_uint,
                    output: *mut *mut libc::c_short);
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
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    /* *
 * \fn const char *PHYSFS_getLastError(void)
 * \brief Get human-readable error information.
 *
 * \deprecated Use PHYSFS_getLastErrorCode() and PHYSFS_getErrorByCode() instead.
 *
 * \warning As of PhysicsFS 2.1, this function has been nerfed.
 *          Before PhysicsFS 2.1, this function was the only way to get
 *          error details beyond a given function's basic return value.
 *          This was meant to be a human-readable string in one of several
 *          languages, and was not useful for application parsing. This was
 *          a problem, because the developer and not the user chose the
 *          language at compile time, and the PhysicsFS maintainers had
 *          to (poorly) maintain a significant amount of localization work.
 *          The app couldn't parse the strings, even if they counted on a
 *          specific language, since some were dynamically generated.
 *          In 2.1 and later, this always returns a static string in
 *          English; you may use it as a key string for your own
 *          localizations if you like, as we'll promise not to change
 *          existing error strings. Also, if your application wants to
 *          look at specific errors, we now offer a better option:
 *          use PHYSFS_getLastErrorCode() instead.
 *
 * Get the last PhysicsFS error message as a human-readable, null-terminated
 *  string. This will return NULL if there's been no error since the last call
 *  to this function. The pointer returned by this call points to an internal
 *  buffer. Each thread has a unique error state associated with it, but each
 *  time a new error message is set, it will overwrite the previous one
 *  associated with that thread. It is safe to call this function at anytime,
 *  even before PHYSFS_init().
 *
 * PHYSFS_getLastError() and PHYSFS_getLastErrorCode() both reset the same
 *  thread-specific error state. Calling one will wipe out the other's
 *  data. If you need both, call PHYSFS_getLastErrorCode(), then pass that
 *  value to PHYSFS_getErrorByCode().
 *
 * As of PhysicsFS 2.1, this function only presents text in the English
 *  language, but the strings are static, so you can use them as keys into
 *  your own localization dictionary. These strings are meant to be passed on
 *  directly to the user.
 *
 * Generally, applications should only concern themselves with whether a
 *  given function failed; however, if your code require more specifics, you
 *  should use PHYSFS_getLastErrorCode() instead of this function.
 *
 *   \return READ ONLY string of last error message.
 *
 * \sa PHYSFS_getLastErrorCode
 * \sa PHYSFS_getErrorByCode
 */
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    /* i/o stuff... */
    /* *
 * \fn PHYSFS_File *PHYSFS_openWrite(const char *filename)
 * \brief Open a file for writing.
 *
 * Open a file for writing, in platform-independent notation and in relation
 *  to the write dir as the root of the writable filesystem. The specified
 *  file is created if it doesn't exist. If it does exist, it is truncated to
 *  zero bytes, and the writing offset is set to the start.
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, and opening a
 *  symlink with this function will fail in such a case.
 *
 *   \param filename File to open.
 *  \return A valid PhysicsFS filehandle on success, NULL on error. Use
 *          PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_openRead
 * \sa PHYSFS_openAppend
 * \sa PHYSFS_write
 * \sa PHYSFS_close
 */
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    /* *
 * \fn PHYSFS_File *PHYSFS_openRead(const char *filename)
 * \brief Open a file for reading.
 *
 * Open a file for reading, in platform-independent notation. The search path
 *  is checked one at a time until a matching file is found, in which case an
 *  abstract filehandle is associated with it, and reading may be done.
 *  The reading offset is set to the first byte of the file.
 *
 * Note that entries that are symlinks are ignored if
 *  PHYSFS_permitSymbolicLinks(1) hasn't been called, and opening a
 *  symlink with this function will fail in such a case.
 *
 *   \param filename File to open.
 *  \return A valid PhysicsFS filehandle on success, NULL on error.
 *          Use PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_openWrite
 * \sa PHYSFS_openAppend
 * \sa PHYSFS_read
 * \sa PHYSFS_close
 */
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    /* *
 * \fn int PHYSFS_close(PHYSFS_File *handle)
 * \brief Close a PhysicsFS filehandle.
 *
 * This call is capable of failing if the operating system was buffering
 *  writes to the physical media, and, now forced to write those changes to
 *  physical media, can not store the data for some reason. In such a case,
 *  the filehandle stays open. A well-written program should ALWAYS check the
 *  return value from the close call in addition to every writing call!
 *
 *   \param handle handle returned from PHYSFS_open*().
 *  \return nonzero on success, zero on error. Use PHYSFS_getLastErrorCode()
 *          to obtain the specific error.
 *
 * \sa PHYSFS_openRead
 * \sa PHYSFS_openWrite
 * \sa PHYSFS_openAppend
 */
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    /* *
 * \fn PHYSFS_sint64 PHYSFS_read(PHYSFS_File *handle, void *buffer, PHYSFS_uint32 objSize, PHYSFS_uint32 objCount)
 * \brief Read data from a PhysicsFS filehandle
 *
 * The file must be opened for reading.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_readBytes() instead. This
 *             function just wraps it anyhow. This function never clarified
 *             what would happen if you managed to read a partial object, so
 *             working at the byte level makes this cleaner for everyone,
 *             especially now that PHYSFS_Io interfaces can be supplied by the
 *             application.
 *
 *   \param handle handle returned from PHYSFS_openRead().
 *   \param buffer buffer to store read data into.
 *   \param objSize size in bytes of objects being read from (handle).
 *   \param objCount number of (objSize) objects to read from (handle).
 *  \return number of objects read. PHYSFS_getLastErrorCode() can shed light
 *          on the reason this might be < (objCount), as can PHYSFS_eof().
 *          -1 if complete failure.
 *
 * \sa PHYSFS_readBytes
 * \sa PHYSFS_eof
 */
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    /* *
 * \fn PHYSFS_sint64 PHYSFS_write(PHYSFS_File *handle, const void *buffer, PHYSFS_uint32 objSize, PHYSFS_uint32 objCount)
 * \brief Write data to a PhysicsFS filehandle
 *
 * The file must be opened for writing.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_writeBytes() instead. This
 *             function just wraps it anyhow. This function never clarified
 *             what would happen if you managed to write a partial object, so
 *             working at the byte level makes this cleaner for everyone,
 *             especially now that PHYSFS_Io interfaces can be supplied by the
 *             application.
 *
 *   \param handle retval from PHYSFS_openWrite() or PHYSFS_openAppend().
 *   \param buffer buffer of bytes to write to (handle).
 *   \param objSize size in bytes of objects being written to (handle).
 *   \param objCount number of (objSize) objects to write to (handle).
 *  \return number of objects written. PHYSFS_getLastErrorCode() can shed
 *          light on the reason this might be < (objCount). -1 if complete
 *          failure.
 *
 * \sa PHYSFS_writeBytes
 */
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    /* *
 * \fn int PHYSFS_seek(PHYSFS_File *handle, PHYSFS_uint64 pos)
 * \brief Seek to a new position within a PhysicsFS filehandle.
 *
 * The next read or write will occur at that place. Seeking past the
 *  beginning or end of the file is not allowed, and causes an error.
 *
 *   \param handle handle returned from PHYSFS_open*().
 *   \param pos number of bytes from start of file to seek to.
 *  \return nonzero on success, zero on error. Use PHYSFS_getLastErrorCode()
 *          to obtain the specific error.
 *
 * \sa PHYSFS_tell
 */
    #[no_mangle]
    fn PHYSFS_seek(handle: *mut PHYSFS_File, pos: PHYSFS_uint64)
     -> libc::c_int;
    /* Buffering stuff... */
    /* *
 * \fn int PHYSFS_setBuffer(PHYSFS_File *handle, PHYSFS_uint64 bufsize)
 * \brief Set up buffering for a PhysicsFS file handle.
 *
 * Define an i/o buffer for a file handle. A memory block of (bufsize) bytes
 *  will be allocated and associated with (handle).
 *
 * For files opened for reading, up to (bufsize) bytes are read from (handle)
 *  and stored in the internal buffer. Calls to PHYSFS_read() will pull
 *  from this buffer until it is empty, and then refill it for more reading.
 *  Note that compressed files, like ZIP archives, will decompress while
 *  buffering, so this can be handy for offsetting CPU-intensive operations.
 *  The buffer isn't filled until you do your next read.
 *
 * For files opened for writing, data will be buffered to memory until the
 *  buffer is full or the buffer is flushed. Closing a handle implicitly
 *  causes a flush...check your return values!
 *
 * Seeking, etc transparently accounts for buffering.
 *
 * You can resize an existing buffer by calling this function more than once
 *  on the same file. Setting the buffer size to zero will free an existing
 *  buffer.
 *
 * PhysicsFS file handles are unbuffered by default.
 *
 * Please check the return value of this function! Failures can include
 *  not being able to seek backwards in a read-only file when removing the
 *  buffer, not being able to allocate the buffer, and not being able to
 *  flush the buffer to disk, among other unexpected problems.
 *
 *   \param handle handle returned from PHYSFS_open*().
 *   \param bufsize size, in bytes, of buffer to allocate.
 *  \return nonzero if successful, zero on error.
 *
 * \sa PHYSFS_flush
 * \sa PHYSFS_read
 * \sa PHYSFS_write
 * \sa PHYSFS_close
 */
    #[no_mangle]
    fn PHYSFS_setBuffer(handle: *mut PHYSFS_File, bufsize: PHYSFS_uint64)
     -> libc::c_int;
    #[no_mangle]
    fn dec130_decode(rpl: *mut RPL, in_0: *mut libc::c_char,
                     in_size: libc::c_uint, out: *mut libc::c_char)
     -> libc::c_uint;
}
pub type size_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
/* *
 * \typedef PHYSFS_uint32
 * \brief An unsigned, 32-bit integer type.
 */
pub type PHYSFS_uint32 = libc::c_uint;
/* *
 * \typedef PHYSFS_uint64
 * \brief An unsigned, 64-bit integer type.
 * \warning on platforms without any sort of 64-bit datatype, this is
 *           equivalent to PHYSFS_uint32!
 */
/* *
 * \typedef PHYSFS_sint64
 * \brief A signed, 64-bit integer type.
 * \warning on platforms without any sort of 64-bit datatype, this is
 *           equivalent to PHYSFS_sint32!
 */
/* oh well. */
pub type PHYSFS_uint64 = libc::c_ulonglong;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPL_chunk_info_t {
    pub offset: libc::c_int,
    pub video_size: libc::c_int,
    pub audio_size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RPL {
    pub f: *mut PHYSFS_File,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bpp: libc::c_int,
    pub fps: libc::c_float,
    pub video_decoder: Option<unsafe extern "C" fn(_: *mut RPL,
                                                   _: *mut libc::c_char,
                                                   _: libc::c_uint,
                                                   _: *mut libc::c_char)
                                  -> libc::c_uint>,
    pub current_video_frame: libc::c_uint,
    pub samples: libc::c_int,
    pub channels: libc::c_int,
    pub bps: libc::c_int,
    pub fpc: libc::c_int,
    pub sound_decoder: Option<unsafe extern "C" fn(_: *mut RPL,
                                                   _: *mut libc::c_short,
                                                   _: libc::c_uint)
                                  -> libc::c_uint>,
    pub current_sound_frame: libc::c_uint,
    pub nb_chunks: libc::c_int,
    pub chunks: *mut RPL_chunk_info_t,
    pub ocs: libc::c_int,
    pub ecs: libc::c_int,
    pub otcc: libc::c_int,
    pub ots: libc::c_int,
    pub sprite_size: libc::c_int,
    pub otkf: libc::c_int,
}
//*************************************************************************************
#[no_mangle]
pub static mut data_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut data_buffer_size: libc::c_uint =
    0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn resize_data_buffer(mut size: libc::c_uint) {
    if size > data_buffer_size {
        if !data_buffer.is_null() { free(data_buffer as *mut libc::c_void); }
        data_buffer = malloc(size) as *mut libc::c_char;
        data_buffer_size = size
    };
}
//*************************************************************************************
unsafe extern "C" fn readline(mut f: *mut PHYSFS_File,
                              mut linebuf: *mut libc::c_char, mut len: size_t)
 -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut i: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < len.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        if PHYSFS_read(f, &mut c as *mut libc::c_char as *mut libc::c_void,
                       1 as libc::c_int as PHYSFS_uint32,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"Error reading from sequence file: %s\n\x00" as *const u8
                      as *const libc::c_char, PHYSFS_getLastError());
            break ;
        } else {
            if c as libc::c_int == '\n' as i32 ||
                   *(*__ctype_b_loc()).offset(c as libc::c_int as isize) as
                       libc::c_int &
                       _ISprint as libc::c_int as libc::c_ushort as
                           libc::c_int == 0 {
                break ;
            }
            *linebuf.offset(i as isize) = c;
            i = i.wrapping_add(1)
        }
    }
    *linebuf.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return linebuf;
}
unsafe extern "C" fn readint(mut f: *mut PHYSFS_File,
                             mut linebuf: *mut libc::c_char, mut len: size_t)
 -> libc::c_int {
    let mut num: libc::c_int = 0;
    readline(f, linebuf, len);
    if sscanf(linebuf, b"%u\x00" as *const u8 as *const libc::c_char,
              &mut num as *mut libc::c_int) < 1 as libc::c_int {
        num = 0 as libc::c_int
    }
    return num;
}
unsafe extern "C" fn readfloat(mut f: *mut PHYSFS_File,
                               mut linebuf: *mut libc::c_char,
                               mut len: size_t) -> libc::c_float {
    let mut num: libc::c_float = 0.;
    readline(f, linebuf, len);
    if sscanf(linebuf, b"%f\x00" as *const u8 as *const libc::c_char,
              &mut num as *mut libc::c_float) < 1 as libc::c_int {
        num = 0.0f64 as libc::c_float
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_open(mut filename: *mut libc::c_char)
 -> *mut RPL {
    let mut current_block: u64;
    let mut f: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut rpl: *mut RPL = 0 as *mut RPL;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut tmp: libc::c_int = 0;
    let mut len: size_t =
        ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong;
    /* FIXME: we should just clean up our data */
    tmp = 0 as libc::c_int;
    while (tmp as libc::c_uint) < strlen(filename) {
        if *filename.offset(tmp as isize) as libc::c_int == '\\' as i32 {
            *filename.offset(tmp as isize) = '/' as i32 as libc::c_char
        }
        tmp += 1
    }
    f = PHYSFS_openRead(filename);
    if f.is_null() {
        debug(LOG_ERROR,
              b"Error reading %s: %s\x00" as *const u8 as *const libc::c_char,
              filename, PHYSFS_getLastError());
        return 0 as *mut RPL
    }
    /* going to do lots of small reads, so use a buffer */
    PHYSFS_setBuffer(f,
                     1024 as libc::c_int as
                         PHYSFS_uint64); /* discard filename */
    rpl =
        malloc(::std::mem::size_of::<RPL>() as libc::c_ulong) as
            *mut RPL; /* discard copyright */
    (*rpl).chunks = 0 as *mut RPL_chunk_info_t;
    (*rpl).f = f;
    if strcmp(readline(f, buf.as_mut_ptr(), len),
              b"ARMovie\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        debug(LOG_NEVER,
              b"%s missing RPL magic number\n\x00" as *const u8 as
                  *const libc::c_char, filename);
    }
    readline(f, buf.as_mut_ptr(), len);
    readline(f, buf.as_mut_ptr(), len);
    if strcmp(readline(f, buf.as_mut_ptr(), len),
              b"ESCAPE 2.0\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        /* This field is really "author", but.. */
        debug(LOG_NEVER,
              b"%s not in \"ESCAPE 2.0\" format?\n\x00" as *const u8 as
                  *const libc::c_char, filename);
    }
    tmp = readint(f, buf.as_mut_ptr(), len);
    match tmp {
        130 => {
            (*rpl).video_decoder =
                Some(dec130_decode as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_char,
                                              _: libc::c_uint,
                                              _: *mut libc::c_char)
                             -> libc::c_uint)
        }
        _ => {
            (*rpl).video_decoder =
                Some(rpl_decode_video_unknown as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_char,
                                              _: libc::c_uint,
                                              _: *mut libc::c_char)
                             -> libc::c_uint);
            debug(LOG_NEVER,
                  b"Unknown video format %i\n\x00" as *const u8 as
                      *const libc::c_char, tmp);
        }
    }
    (*rpl).width = readint(f, buf.as_mut_ptr(), len);
    //printf("width : %i\n", rpl->width);
    (*rpl).height = readint(f, buf.as_mut_ptr(), len);
    //printf("height : %i\n", rpl->height);
    (*rpl).bpp = readint(f, buf.as_mut_ptr(), len);
    //printf("bpp : %i\n", rpl->bpp);
    (*rpl).fps = readfloat(f, buf.as_mut_ptr(), len);
    //printf("fps : %f\n\n", rpl->fps);
    (*rpl).current_video_frame = 0 as libc::c_int as libc::c_uint;
    tmp = readint(f, buf.as_mut_ptr(), len);
    match tmp {
        0 => {
            (*rpl).sound_decoder =
                Some(rpl_decode_sound_none as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_short,
                                              _: libc::c_uint)
                             -> libc::c_uint)
        }
        1 => {
            (*rpl).sound_decoder =
                Some(rpl_decode_sound_raw as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_short,
                                              _: libc::c_uint)
                             -> libc::c_uint)
        }
        101 => {
            (*rpl).sound_decoder =
                Some(rpl_decode_sound_adpcm as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_short,
                                              _: libc::c_uint)
                             -> libc::c_uint)
        }
        _ => {
            (*rpl).sound_decoder =
                Some(rpl_decode_sound_unknown as
                         unsafe extern "C" fn(_: *mut RPL,
                                              _: *mut libc::c_short,
                                              _: libc::c_uint)
                             -> libc::c_uint);
            debug(LOG_VIDEO,
                  b"Unknown sound format %i\n\x00" as *const u8 as
                      *const libc::c_char, tmp);
        }
    }
    (*rpl).current_sound_frame = 0 as libc::c_int as libc::c_uint;
    (*rpl).samples = readint(f, buf.as_mut_ptr(), len);
    //printf("samples : %i\n", rpl->samples);
    (*rpl).channels = readint(f, buf.as_mut_ptr(), len);
    //printf("channels : %i\n", rpl->channels);
    (*rpl).bps = readint(f, buf.as_mut_ptr(), len);
    //printf("bits per sample : %i\n\n", rpl->bps);
    (*rpl).fpc = readint(f, buf.as_mut_ptr(), len);
    //printf("frames per chunk : %i\n", rpl->fpc);
    (*rpl).nb_chunks = readint(f, buf.as_mut_ptr(), len) + 1 as libc::c_int;
    //printf("chunks : %i\n", rpl->nb_chunks);
    (*rpl).ocs = readint(f, buf.as_mut_ptr(), len);
    //printf("odd chunk size : %i\n", rpl->ocs);
    (*rpl).ecs = readint(f, buf.as_mut_ptr(), len);
    //printf("even chunk size : %i\n", rpl->ecs);
    (*rpl).otcc = readint(f, buf.as_mut_ptr(), len);
    //printf("offset to chunk cat : %i\n\n", rpl->otcc);
    (*rpl).ots = readint(f, buf.as_mut_ptr(), len);
    //printf("offset to sprite : %i\n", rpl->ots);
    (*rpl).sprite_size = readint(f, buf.as_mut_ptr(), len);
    //printf("size of sprite : %i\n\n", rpl->sprite_size);
    (*rpl).otkf = readint(f, buf.as_mut_ptr(), len);
    //printf("offset to key frames : %i\n", rpl->otkf);
    let mut i: libc::c_uint = 0;
    let mut max_video_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    (*rpl).chunks =
        malloc((::std::mem::size_of::<RPL_chunk_info_t>() as
                    libc::c_ulong).wrapping_mul((*rpl).nb_chunks as
                                                    libc::c_uint)) as
            *mut RPL_chunk_info_t;
    PHYSFS_seek(f, (*rpl).otcc as PHYSFS_uint64);
    i = 0 as libc::c_int as libc::c_uint;
    loop  {
        if !(i < (*rpl).nb_chunks as libc::c_uint) {
            current_block = 1623252117315916725;
            break ;
        }
        readline(f, buf.as_mut_ptr(), len);
        if sscanf(buf.as_mut_ptr(),
                  b"%i,%i;%i\x00" as *const u8 as *const libc::c_char,
                  &mut (*(*rpl).chunks.offset(i as isize)).offset as
                      *mut libc::c_int,
                  &mut (*(*rpl).chunks.offset(i as isize)).video_size as
                      *mut libc::c_int,
                  &mut (*(*rpl).chunks.offset(i as isize)).audio_size as
                      *mut libc::c_int) != 3 as libc::c_int {
            debug(LOG_VIDEO,
                  b"Error in chunk catalog\n\x00" as *const u8 as
                      *const libc::c_char);
            current_block = 16389613605840669344;
            break ;
        } else {
            if (*(*rpl).chunks.offset(i as isize)).video_size as libc::c_uint
                   > max_video_size {
                max_video_size =
                    (*(*rpl).chunks.offset(i as isize)).video_size as
                        libc::c_uint
            }
            i = i.wrapping_add(1)
        }
    }
    match current_block {
        16389613605840669344 => {
            free((*rpl).chunks as *mut libc::c_void);
            free(rpl as *mut libc::c_void);
            return 0 as *mut RPL
        }
        _ => { resize_data_buffer(max_video_size); return rpl }
    };
}
//looks like we need this. --Qamly
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_sound_none(mut rpl: *mut RPL,
                                               mut buffer: *mut libc::c_short,
                                               mut buffer_size: libc::c_uint)
 -> libc::c_uint {
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_sound_unknown(mut rpl: *mut RPL,
                                                  mut buffer:
                                                      *mut libc::c_short,
                                                  mut buffer_size:
                                                      libc::c_uint)
 -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut total_audio_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut audio_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    debug(LOG_VIDEO,
          b"Saving unknown sound stream to file\n\x00" as *const u8 as
              *const libc::c_char);
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*rpl).nb_chunks as libc::c_uint {
        total_audio_size =
            total_audio_size.wrapping_add((*(*rpl).chunks.offset(i as
                                                                     isize)).audio_size
                                              as libc::c_uint);
        i = i.wrapping_add(1)
    }
    audio_buffer = malloc(total_audio_size) as *mut libc::c_char;
    tmp = audio_buffer;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*rpl).nb_chunks as libc::c_uint {
        PHYSFS_seek((*rpl).f,
                    ((*(*rpl).chunks.offset(i as isize)).offset +
                         (*(*rpl).chunks.offset(i as isize)).video_size) as
                        PHYSFS_uint64);
        PHYSFS_read((*rpl).f, tmp as *mut libc::c_void,
                    (*(*rpl).chunks.offset(i as isize)).audio_size as
                        PHYSFS_uint32, 1 as libc::c_int as PHYSFS_uint32);
        tmp =
            tmp.offset((*(*rpl).chunks.offset(i as isize)).audio_size as
                           isize);
        i = i.wrapping_add(1)
    }
    out =
        PHYSFS_openWrite(b"unknown_sound\x00" as *const u8 as
                             *const libc::c_char);
    PHYSFS_write(out, audio_buffer as *const libc::c_void, total_audio_size,
                 1 as libc::c_int as PHYSFS_uint32);
    PHYSFS_close(out);
    free(audio_buffer as *mut libc::c_void);
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_sound_raw(mut rpl: *mut RPL,
                                              mut buffer: *mut libc::c_short,
                                              mut buffer_size: libc::c_uint)
 -> libc::c_uint {
    let mut size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tmp: *mut libc::c_short = buffer;
    loop  {
        let mut cf: libc::c_uint = (*rpl).current_sound_frame;
        let mut audio_frame_size: libc::c_uint =
            (*(*rpl).chunks.offset(cf as isize)).audio_size as libc::c_uint;
        if (*rpl).current_sound_frame >= (*rpl).nb_chunks as libc::c_uint {
            break ;
        }
        if size.wrapping_add(audio_frame_size >> 1 as libc::c_int) >
               buffer_size {
            break ;
        }
        PHYSFS_seek((*rpl).f,
                    ((*(*rpl).chunks.offset(cf as isize)).offset +
                         (*(*rpl).chunks.offset(cf as isize)).video_size) as
                        PHYSFS_uint64);
        PHYSFS_read((*rpl).f, tmp as *mut libc::c_void, audio_frame_size,
                    1 as libc::c_int as PHYSFS_uint32);
        tmp = tmp.offset((audio_frame_size >> 1 as libc::c_int) as isize);
        size = size.wrapping_add(audio_frame_size >> 1 as libc::c_int);
        (*rpl).current_sound_frame =
            (*rpl).current_sound_frame.wrapping_add(1)
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_sound_adpcm(mut rpl: *mut RPL,
                                                mut buffer:
                                                    *mut libc::c_short,
                                                mut buffer_size: libc::c_uint)
 -> libc::c_uint {
    static mut tmp_buffer: *mut libc::c_uchar =
        0 as *const libc::c_uchar as *mut libc::c_uchar;
    let mut tmp_buffer_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut tmp: *mut libc::c_short = buffer;
    loop  {
        let mut cf: libc::c_uint = (*rpl).current_sound_frame;
        let mut audio_frame_size: libc::c_uint =
            (*(*rpl).chunks.offset(cf as isize)).audio_size as libc::c_uint;
        if (*rpl).current_sound_frame >= (*rpl).nb_chunks as libc::c_uint {
            break ;
        }
        if size.wrapping_add(audio_frame_size << 1 as libc::c_int) >
               buffer_size {
            break ;
        }
        if audio_frame_size > tmp_buffer_size {
            tmp_buffer_size = audio_frame_size << 1 as libc::c_int;
            free(tmp_buffer as *mut libc::c_void);
            tmp_buffer = malloc(tmp_buffer_size) as *mut libc::c_uchar
        }
        PHYSFS_seek((*rpl).f,
                    ((*(*rpl).chunks.offset(cf as isize)).offset +
                         (*(*rpl).chunks.offset(cf as isize)).video_size) as
                        PHYSFS_uint64);
        PHYSFS_read((*rpl).f, tmp_buffer as *mut libc::c_void,
                    audio_frame_size, 1 as libc::c_int as PHYSFS_uint32);
        adpcm_decode(tmp_buffer, audio_frame_size, &mut tmp);
        size = size.wrapping_add(audio_frame_size << 1 as libc::c_int);
        (*rpl).current_sound_frame =
            (*rpl).current_sound_frame.wrapping_add(1)
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_sound(mut rpl: *mut RPL,
                                          mut buffer: *mut libc::c_short,
                                          mut buffer_size: libc::c_uint)
 -> libc::c_uint {
    return (*rpl).sound_decoder.expect("non-null function pointer")(rpl,
                                                                    buffer,
                                                                    buffer_size);
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_video_unknown(mut rpl: *mut RPL,
                                                  mut in_0: *mut libc::c_char,
                                                  mut in_size: libc::c_uint,
                                                  mut out: *mut libc::c_char)
 -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while i < in_size {
        *out.offset(j as isize) = *in_0.offset(i as isize);
        *out.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
            =
            *in_0.offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                             isize);
        i = i.wrapping_add(2 as libc::c_int as libc::c_uint);
        j = j.wrapping_add(3 as libc::c_int as libc::c_uint)
    }
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_decode_next_image(mut rpl: *mut RPL,
                                               mut buffer: *mut libc::c_char)
 -> libc::c_int {
    let mut data_size: libc::c_uint = 0;
    if (*rpl).current_video_frame >= (*rpl).nb_chunks as libc::c_uint {
        return -(1 as libc::c_int)
    }
    data_size =
        (*(*rpl).chunks.offset((*rpl).current_video_frame as
                                   isize)).video_size as libc::c_uint;
    PHYSFS_seek((*rpl).f,
                (*(*rpl).chunks.offset((*rpl).current_video_frame as
                                           isize)).offset as PHYSFS_uint64);
    PHYSFS_read((*rpl).f, data_buffer as *mut libc::c_void, data_size,
                1 as libc::c_int as PHYSFS_uint32);
    (*rpl).video_decoder.expect("non-null function pointer")(rpl, data_buffer,
                                                             data_size,
                                                             buffer);
    let fresh0 = (*rpl).current_video_frame;
    (*rpl).current_video_frame = (*rpl).current_video_frame.wrapping_add(1);
    return fresh0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_close(mut rpl: *mut RPL) {
    if !rpl.is_null() {
        PHYSFS_close((*rpl).f);
        free((*rpl).chunks as *mut libc::c_void);
        free(rpl as *mut libc::c_void);
    };
}
