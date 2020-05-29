use ::libc;
extern "C" {
    /* Write formatted output to S.  */
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer_0: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    /* File position stuff... */
    /* *
 * \fn int PHYSFS_eof(PHYSFS_File *handle)
 * \brief Check for end-of-file state on a PhysicsFS filehandle.
 *
 * Determine if the end of file has been reached in a PhysicsFS filehandle.
 *
 *   \param handle handle returned from PHYSFS_openRead().
 *  \return nonzero if EOF, zero if not.
 *
 * \sa PHYSFS_read
 * \sa PHYSFS_tell
 */
    #[no_mangle]
    fn PHYSFS_eof(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn rand() -> libc::c_int;
}
/* *
 * \typedef PHYSFS_uint32
 * \brief An unsigned, 32-bit integer type.
 */
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WZ_TRACK {
    pub songs: *mut *mut libc::c_char,
    pub nb_songs: libc::c_uint,
    pub list_size: libc::c_uint,
    pub shuffle: BOOL,
}
static mut buffer: [libc::c_char; 2048] = [0; 2048];
static mut ByteBuf: libc::c_char = '\u{0}' as i32 as libc::c_char;
static mut ByteBufPos: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut current_track: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut current_song: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut playlist: [WZ_TRACK; 3] =
    [WZ_TRACK{songs: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
              nb_songs: 0,
              list_size: 0,
              shuffle: 0,}; 3];
#[no_mangle]
pub unsafe extern "C" fn PlayList_Init() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        playlist[i as usize].songs =
            malloc((2 as libc::c_int as
                        libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                       as libc::c_ulong)) as
                *mut *mut libc::c_char;
        playlist[i as usize].list_size = 2 as libc::c_int as libc::c_uint;
        playlist[i as usize].nb_songs = 0 as libc::c_int as libc::c_uint;
        memset(playlist[i as usize].songs as *mut libc::c_void,
               0 as libc::c_int,
               playlist[i as
                            usize].list_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                              as
                                                              libc::c_ulong));
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_Quit() {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as libc::c_uint;
        while j < playlist[i as usize].list_size {
            free(*playlist[i as usize].songs.offset(j as isize) as
                     *mut libc::c_void);
            j = j.wrapping_add(1)
        }
        free(playlist[i as usize].songs as *mut libc::c_void);
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_Read(mut path: *const libc::c_char)
 -> libc::c_char {
    let mut f: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut path_to_music: *mut libc::c_char = 0 as *mut libc::c_char;
    sprintf(buffer.as_mut_ptr(),
            b"%s/music.wpl\x00" as *const u8 as *const libc::c_char, path);
    f = PHYSFS_openRead(buffer.as_mut_ptr());
    if f.is_null() { return 1 as libc::c_int as libc::c_char }
    while PHYSFS_eof(f) == 0 {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        while ByteBufPos <
                  (2048 as libc::c_int - 1 as libc::c_int) as libc::c_uint &&
                  PHYSFS_read(f,
                              &mut ByteBuf as *mut libc::c_char as
                                  *mut libc::c_void,
                              1 as libc::c_int as PHYSFS_uint32,
                              1 as libc::c_int as PHYSFS_uint32) != 0 &&
                  ByteBuf as libc::c_int != '\n' as i32 {
            buffer[ByteBufPos as usize] = ByteBuf;
            ByteBufPos = ByteBufPos.wrapping_add(1)
        }
        buffer[ByteBufPos as usize] = '\u{0}' as i32 as libc::c_char;
        ByteBufPos = 0 as libc::c_int as libc::c_uint;
        if strncmp(buffer.as_mut_ptr(),
                   b"[game]\x00" as *const u8 as *const libc::c_char,
                   6 as libc::c_int as libc::c_uint) == 0 as libc::c_int {
            current_track = 1 as libc::c_int as libc::c_uint;
            free(path_to_music as *mut libc::c_void);
            path_to_music = 0 as *mut libc::c_char;
            playlist[current_track as usize].shuffle = 0 as libc::c_int
        } else if strncmp(buffer.as_mut_ptr(),
                          b"[menu]\x00" as *const u8 as *const libc::c_char,
                          6 as libc::c_int as libc::c_uint) ==
                      0 as libc::c_int {
            current_track = 2 as libc::c_int as libc::c_uint;
            free(path_to_music as *mut libc::c_void);
            path_to_music = 0 as *mut libc::c_char;
            playlist[current_track as usize].shuffle = 0 as libc::c_int
        } else if strncmp(buffer.as_mut_ptr(),
                          b"path=\x00" as *const u8 as *const libc::c_char,
                          5 as libc::c_int as libc::c_uint) ==
                      0 as libc::c_int {
            free(path_to_music as *mut libc::c_void);
            path_to_music =
                strtok(buffer.as_mut_ptr().offset(5 as libc::c_int as isize),
                       b"\n\x00" as *const u8 as *const libc::c_char);
            if strcmp(path_to_music,
                      b".\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                path_to_music = strdup(path)
            } else { path_to_music = strdup(path_to_music) }
            debug(LOG_WZ,
                  b"  path = %s\n\x00" as *const u8 as *const libc::c_char,
                  path_to_music);
        } else if strncmp(buffer.as_mut_ptr(),
                          b"shuffle=\x00" as *const u8 as *const libc::c_char,
                          8 as libc::c_int as libc::c_uint) ==
                      0 as libc::c_int {
            if strcmp(strtok(buffer.as_mut_ptr().offset(8 as libc::c_int as
                                                            isize),
                             b"\n\x00" as *const u8 as *const libc::c_char),
                      b"yes\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                playlist[current_track as usize].shuffle = 1 as libc::c_int
            }
            debug(LOG_WZ,
                  b"  shuffle = yes\n\x00" as *const u8 as
                      *const libc::c_char);
        } else if buffer[0 as libc::c_int as usize] as libc::c_int !=
                      '\u{0}' as i32 &&
                      {
                          filename =
                              strtok(buffer.as_mut_ptr(),
                                     b"\n\x00" as *const u8 as
                                         *const libc::c_char);
                          !filename.is_null()
                      } &&
                      strlen(filename) != 0 as libc::c_int as libc::c_uint {
            let mut filepath: *mut libc::c_char = 0 as *mut libc::c_char;
            if path_to_music.is_null() {
                filepath =
                    malloc(strlen(filename).wrapping_add(1 as libc::c_int as
                                                             libc::c_uint)) as
                        *mut libc::c_char;
                sprintf(filepath,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        filename);
            } else {
                filepath =
                    malloc(strlen(filename).wrapping_add(strlen(path_to_music)).wrapping_add(2
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint))
                        as *mut libc::c_char;
                sprintf(filepath,
                        b"%s/%s\x00" as *const u8 as *const libc::c_char,
                        path_to_music, filename);
            }
            debug(LOG_WZ,
                  b"  adding song %s\n\x00" as *const u8 as
                      *const libc::c_char, filepath);
            if playlist[current_track as usize].nb_songs ==
                   playlist[current_track as usize].list_size {
                playlist[current_track as usize].list_size <<=
                    1 as libc::c_int;
                playlist[current_track as usize].songs =
                    realloc(playlist[current_track as usize].songs as
                                *mut libc::c_void,
                            playlist[current_track as
                                         usize].list_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                           as
                                                                           libc::c_ulong))
                        as *mut *mut libc::c_char
            }
            let fresh0 = playlist[current_track as usize].nb_songs;
            playlist[current_track as usize].nb_songs =
                playlist[current_track as usize].nb_songs.wrapping_add(1);
            let ref mut fresh1 =
                *playlist[current_track as
                              usize].songs.offset(fresh0 as isize);
            *fresh1 = filepath
        }
    }
    free(path_to_music as *mut libc::c_void);
    PHYSFS_close(f);
    return 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_Shuffle() {
    if playlist[current_track as usize].shuffle != 0 {
        let mut i: libc::c_uint = 0;
        i =
            playlist[current_track as
                         usize].nb_songs.wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint);
        while i > 0 as libc::c_int as libc::c_uint {
            let mut j: libc::c_uint =
                (rand() as
                     libc::c_uint).wrapping_rem(i.wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint));
            let mut swap: *mut libc::c_char =
                *playlist[current_track as usize].songs.offset(j as isize);
            let ref mut fresh2 =
                *playlist[current_track as usize].songs.offset(j as isize);
            *fresh2 =
                *playlist[current_track as usize].songs.offset(i as isize);
            let ref mut fresh3 =
                *playlist[current_track as usize].songs.offset(i as isize);
            *fresh3 = swap;
            i = i.wrapping_sub(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_SetTrack(mut t: libc::c_uint) {
    if t >= 0 as libc::c_int as libc::c_uint &&
           t < 3 as libc::c_int as libc::c_uint {
        current_track = t
    } else { current_track = 0 as libc::c_int as libc::c_uint }
    PlayList_Shuffle();
    current_song = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_CurrentSong() -> *mut libc::c_char {
    if current_song >= playlist[current_track as usize].nb_songs {
        return 0 as *mut libc::c_char
    } else {
        return *playlist[current_track as
                             usize].songs.offset(current_song as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_NextSong() -> *mut libc::c_char {
    current_song = current_song.wrapping_add(1);
    if current_song >= playlist[current_track as usize].nb_songs {
        PlayList_Shuffle();
        current_song = 0 as libc::c_int as libc::c_uint
    }
    if playlist[current_track as usize].nb_songs ==
           0 as libc::c_int as libc::c_uint {
        return 0 as *mut libc::c_char
    } else {
        return *playlist[current_track as
                             usize].songs.offset(current_song as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn PlayList_DeleteCurrentSong() { }
