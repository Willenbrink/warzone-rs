#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, main, ptr_wrapping_offset_from,
           register_tool)]
use ::rs-src::*;
extern "C" {
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
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
    fn debug_register_callback(callback: debug_callback_fn,
                               init: debug_callback_init,
                               exit_0: debug_callback_exit,
                               data: *mut libc::c_void);
    #[no_mangle]
    fn debug_exit();
    #[no_mangle]
    fn debug_init();
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* *********************************************************************************/
/*                    function prototypes                                         */
    // initialise the block system
    #[no_mangle]
    fn blkInitialise() -> BOOL;
    /* Initialise the frame work library */
    #[no_mangle]
    fn frameInitialise(hInstance: HANDLE, pWindowName: *mut STRING,
                       width: UDWORD, height: UDWORD, bitDepth: UDWORD,
                       fullScreen: BOOL, bVidMem: BOOL) -> BOOL;
    // Whether to put surfaces in video memory
    /* Shut down the framework library.
 * This clears up all the Direct Draw stuff and ensures
 * that Windows gets restored properly after Full screen mode.
 */
    #[no_mangle]
    fn frameShutDown();
    /* Call this each cycle to allow the framework to deal with
 * windows messages, and do general house keeping.
 *
 * Returns FRAME_STATUS.
 */
    #[no_mangle]
    fn frameUpdate() -> FRAME_STATUS;
    // load a file from disk into a fixed memory buffer
    #[no_mangle]
    fn loadFileToBuffer(pFileName: *mut libc::c_char,
                        pFileBuffer: *mut libc::c_char, bufferSize: UDWORD,
                        pSize: *mut UDWORD) -> BOOL;
    /* DOXYGEN_SHOULD_IGNORE_THIS */
    /* PhysicsFS state stuff ... */
    /* *
 * \def PHYSFS_VERSION(x)
 * \brief Macro to determine PhysicsFS version program was compiled against.
 *
 * This macro fills in a PHYSFS_Version structure with the version of the
 *  library you compiled against. This is determined by what header the
 *  compiler uses. Note that if you dynamically linked the library, you might
 *  have a slightly newer or older version at runtime. That version can be
 *  determined with PHYSFS_getLinkedVersion(), which, unlike PHYSFS_VERSION,
 *  is not a macro.
 *
 * \param x A pointer to a PHYSFS_Version struct to initialize.
 *
 * \sa PHYSFS_Version
 * \sa PHYSFS_getLinkedVersion
 */
    /* *
 * \fn void PHYSFS_getLinkedVersion(PHYSFS_Version *ver)
 * \brief Get the version of PhysicsFS that is linked against your program.
 *
 * If you are using a shared library (DLL) version of PhysFS, then it is
 *  possible that it will be different than the version you compiled against.
 *
 * This is a real function; the macro PHYSFS_VERSION tells you what version
 *  of PhysFS you compiled against:
 *
 * \code
 * PHYSFS_Version compiled;
 * PHYSFS_Version linked;
 *
 * PHYSFS_VERSION(&compiled);
 * PHYSFS_getLinkedVersion(&linked);
 * printf("We compiled against PhysFS version %d.%d.%d ...\n",
 *           compiled.major, compiled.minor, compiled.patch);
 * printf("But we linked against PhysFS version %d.%d.%d.\n",
 *           linked.major, linked.minor, linked.patch);
 * \endcode
 *
 * This function may be called safely at any time, even before PHYSFS_init().
 *
 * \sa PHYSFS_VERSION
 */
    #[no_mangle]
    fn PHYSFS_getLinkedVersion(ver: *mut PHYSFS_Version);
    /* *
 * \fn int PHYSFS_init(const char *argv0)
 * \brief Initialize the PhysicsFS library.
 *
 * This must be called before any other PhysicsFS function.
 *
 * This should be called prior to any attempts to change your process's
 *  current working directory.
 *
 *   \param argv0 the argv[0] string passed to your program's mainline.
 *          This may be NULL on most platforms (such as ones without a
 *          standard main() function), but you should always try to pass
 *          something in here. Unix-like systems such as Linux _need_ to
 *          pass argv[0] from main() in here.
 *  \return nonzero on success, zero on error. Specifics of the error can be
 *          gleaned from PHYSFS_getLastError().
 *
 * \sa PHYSFS_deinit
 * \sa PHYSFS_isInit
 */
    #[no_mangle]
    fn PHYSFS_init(argv0: *const libc::c_char) -> libc::c_int;
    /* *
 * \fn void PHYSFS_freeList(void *listVar)
 * \brief Deallocate resources of lists returned by PhysicsFS.
 *
 * Certain PhysicsFS functions return lists of information that are
 *  dynamically allocated. Use this function to free those resources.
 *
 * It is safe to pass a NULL here, but doing so will cause a crash in versions
 *  before PhysicsFS 2.1.0.
 *
 *   \param listVar List of information specified as freeable by this function.
 *                  Passing NULL is safe; it is a valid no-op.
 *
 * \sa PHYSFS_getCdRomDirs
 * \sa PHYSFS_enumerateFiles
 * \sa PHYSFS_getSearchPath
 */
    #[no_mangle]
    fn PHYSFS_freeList(listVar: *mut libc::c_void);
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
    /* *
 * \fn const char *PHYSFS_getDirSeparator(void)
 * \brief Get platform-dependent dir separator string.
 *
 * This returns "\\" on win32, "/" on Unix, and ":" on MacOS. It may be more
 *  than one character, depending on the platform, and your code should take
 *  that into account. Note that this is only useful for setting up the
 *  search/write paths, since access into those dirs always use '/'
 *  (platform-independent notation) to separate directories. This is also
 *  handy for getting platform-independent access when using stdio calls.
 *
 *   \return READ ONLY null-terminated string of platform's dir separator.
 */
    #[no_mangle]
    fn PHYSFS_getDirSeparator() -> *const libc::c_char;
    /* *
 * \fn void PHYSFS_permitSymbolicLinks(int allow)
 * \brief Enable or disable following of symbolic links.
 *
 * Some physical filesystems and archives contain files that are just pointers
 *  to other files. On the physical filesystem, opening such a link will
 *  (transparently) open the file that is pointed to.
 *
 * By default, PhysicsFS will check if a file is really a symlink during open
 *  calls and fail if it is. Otherwise, the link could take you outside the
 *  write and search paths, and compromise security.
 *
 * If you want to take that risk, call this function with a non-zero parameter.
 *  Note that this is more for sandboxing a program's scripting language, in
 *  case untrusted scripts try to compromise the system. Generally speaking,
 *  a user could very well have a legitimate reason to set up a symlink, so
 *  unless you feel there's a specific danger in allowing them, you should
 *  permit them.
 *
 * Symlinks are only explicitly checked when dealing with filenames
 *  in platform-independent notation. That is, when setting up your
 *  search and write paths, etc, symlinks are never checked for.
 *
 * Please note that PHYSFS_stat() will always check the path specified; if
 *  that path is a symlink, it will not be followed in any case. If symlinks
 *  aren't permitted through this function, PHYSFS_stat() ignores them, and
 *  would treat the query as if the path didn't exist at all.
 *
 * Symbolic link permission can be enabled or disabled at any time after
 *  you've called PHYSFS_init(), and is disabled by default.
 *
 *   \param allow nonzero to permit symlinks, zero to deny linking.
 *
 * \sa PHYSFS_symbolicLinksPermitted
 */
    #[no_mangle]
    fn PHYSFS_permitSymbolicLinks(allow: libc::c_int);
    /* *
 * \fn const char *PHYSFS_getBaseDir(void)
 * \brief Get the path where the application resides.
 *
 * Helper function.
 *
 * Get the "base dir". This is the directory where the application was run
 *  from, which is probably the installation directory, and may or may not
 *  be the process's current working directory.
 *
 * You should probably use the base dir in your search path.
 *
 *  \return READ ONLY string of base dir in platform-dependent notation.
 *
 * \sa PHYSFS_getPrefDir
 */
    #[no_mangle]
    fn PHYSFS_getBaseDir() -> *const libc::c_char;
    /* *
 * \fn const char *PHYSFS_getUserDir(void)
 * \brief Get the path where user's home directory resides.
 *
 * \deprecated As of PhysicsFS 2.1, you probably want PHYSFS_getPrefDir().
 *
 * Helper function.
 *
 * Get the "user dir". This is meant to be a suggestion of where a specific
 *  user of the system can store files. On Unix, this is her home directory.
 *  On systems with no concept of multiple home directories (MacOS, win95),
 *  this will default to something like "C:\mybasedir\users\username"
 *  where "username" will either be the login name, or "default" if the
 *  platform doesn't support multiple users, either.
 *
 *  \return READ ONLY string of user dir in platform-dependent notation.
 *
 * \sa PHYSFS_getBaseDir
 * \sa PHYSFS_getPrefDir
 */
    #[no_mangle]
    fn PHYSFS_getUserDir() -> *const libc::c_char;
    /* *
 * \fn const char *PHYSFS_getWriteDir(void)
 * \brief Get path where PhysicsFS will allow file writing.
 *
 * Get the current write dir. The default write dir is NULL.
 *
 *  \return READ ONLY string of write dir in platform-dependent notation,
 *           OR NULL IF NO WRITE PATH IS CURRENTLY SET.
 *
 * \sa PHYSFS_setWriteDir
 */
    #[no_mangle]
    fn PHYSFS_getWriteDir() -> *const libc::c_char;
    /* *
 * \fn int PHYSFS_setWriteDir(const char *newDir)
 * \brief Tell PhysicsFS where it may write files.
 *
 * Set a new write dir. This will override the previous setting.
 *
 * This call will fail (and fail to change the write dir) if the current
 *  write dir still has files open in it.
 *
 *   \param newDir The new directory to be the root of the write dir,
 *                   specified in platform-dependent notation. Setting to NULL
 *                   disables the write dir, so no files can be opened for
 *                   writing via PhysicsFS.
 *  \return non-zero on success, zero on failure. All attempts to open a file
 *           for writing via PhysicsFS will fail until this call succeeds.
 *           Use PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_getWriteDir
 */
    #[no_mangle]
    fn PHYSFS_setWriteDir(newDir: *const libc::c_char) -> libc::c_int;
    /* *
 * \fn int PHYSFS_addToSearchPath(const char *newDir, int appendToPath)
 * \brief Add an archive or directory to the search path.
 *
 * \deprecated As of PhysicsFS 2.0, use PHYSFS_mount() instead. This
 *             function just wraps it anyhow.
 *
 * This function is equivalent to:
 *
 * \code
 *  PHYSFS_mount(newDir, NULL, appendToPath);
 * \endcode
 *
 * You must use this and not PHYSFS_mount if binary compatibility with
 *  PhysicsFS 1.0 is important (which it may not be for many people).
 *
 * \sa PHYSFS_mount
 * \sa PHYSFS_removeFromSearchPath
 * \sa PHYSFS_getSearchPath
 */
    #[no_mangle]
    fn PHYSFS_addToSearchPath(newDir: *const libc::c_char,
                              appendToPath: libc::c_int) -> libc::c_int;
    /* *
 * \fn int PHYSFS_removeFromSearchPath(const char *oldDir)
 * \brief Remove a directory or archive from the search path.
 *
 * \deprecated As of PhysicsFS 2.1, use PHYSFS_unmount() instead. This
 *             function just wraps it anyhow. There's no functional difference
 *             except the vocabulary changed from "adding to the search path"
 *             to "mounting" when that functionality was extended, and thus
 *             the preferred way to accomplish this function's work is now
 *             called "unmounting."
 *
 * This function is equivalent to:
 *
 * \code
 *  PHYSFS_unmount(oldDir);
 * \endcode
 *
 * You must use this and not PHYSFS_unmount if binary compatibility with
 *  PhysicsFS 1.0 is important (which it may not be for many people).
 *
 * \sa PHYSFS_addToSearchPath
 * \sa PHYSFS_getSearchPath
 * \sa PHYSFS_unmount
 */
    #[no_mangle]
    fn PHYSFS_removeFromSearchPath(oldDir: *const libc::c_char)
     -> libc::c_int;
    /* *
 * \fn char **PHYSFS_getSearchPath(void)
 * \brief Get the current search path.
 *
 * The default search path is an empty list.
 *
 * The returned value is an array of strings, with a NULL entry to signify the
 *  end of the list:
 *
 * \code
 * char **i;
 *
 * for (i = PHYSFS_getSearchPath(); *i != NULL; i++)
 *     printf("[%s] is in the search path.\n", *i);
 * \endcode
 *
 * When you are done with the returned information, you may dispose of the
 *  resources by calling PHYSFS_freeList() with the returned pointer.
 *
 *   \return Null-terminated array of null-terminated strings. NULL if there
 *            was a problem (read: OUT OF MEMORY).
 *
 * \sa PHYSFS_getSearchPathCallback
 * \sa PHYSFS_addToSearchPath
 * \sa PHYSFS_removeFromSearchPath
 */
    #[no_mangle]
    fn PHYSFS_getSearchPath() -> *mut *mut libc::c_char;
    /* Directory management stuff ... */
    /* *
 * \fn int PHYSFS_mkdir(const char *dirName)
 * \brief Create a directory.
 *
 * This is specified in platform-independent notation in relation to the
 *  write dir. All missing parent directories are also created if they
 *  don't exist.
 *
 * So if you've got the write dir set to "C:\mygame\writedir" and call
 *  PHYSFS_mkdir("downloads/maps") then the directories
 *  "C:\mygame\writedir\downloads" and "C:\mygame\writedir\downloads\maps"
 *  will be created if possible. If the creation of "maps" fails after we
 *  have successfully created "downloads", then the function leaves the
 *  created directory behind and reports failure.
 *
 *   \param dirName New dir to create.
 *  \return nonzero on success, zero on error. Use
 *          PHYSFS_getLastErrorCode() to obtain the specific error.
 *
 * \sa PHYSFS_delete
 */
    #[no_mangle]
    fn PHYSFS_mkdir(dirName: *const libc::c_char) -> libc::c_int;
    /* *
 * \fn const char *PHYSFS_getRealDir(const char *filename)
 * \brief Figure out where in the search path a file resides.
 *
 * The file is specified in platform-independent notation. The returned
 *  filename will be the element of the search path where the file was found,
 *  which may be a directory, or an archive. Even if there are multiple
 *  matches in different parts of the search path, only the first one found
 *  is used, just like when opening a file.
 *
 * So, if you look for "maps/level1.map", and C:\\mygame is in your search
 *  path and C:\\mygame\\maps\\level1.map exists, then "C:\mygame" is returned.
 *
 * If a any part of a match is a symbolic link, and you've not explicitly
 *  permitted symlinks, then it will be ignored, and the search for a match
 *  will continue.
 *
 * If you specify a fake directory that only exists as a mount point, it'll
 *  be associated with the first archive mounted there, even though that
 *  directory isn't necessarily contained in a real archive.
 *
 * \warning This will return NULL if there is no real directory associated
 *          with (filename). Specifically, PHYSFS_mountIo(),
 *          PHYSFS_mountMemory(), and PHYSFS_mountHandle() will return NULL
 *          even if the filename is found in the search path. Plan accordingly.
 *
 *     \param filename file to look for.
 *    \return READ ONLY string of element of search path containing the
 *             the file in question. NULL if not found.
 */
    #[no_mangle]
    fn PHYSFS_getRealDir(filename: *const libc::c_char)
     -> *const libc::c_char;
    /* *
 * \fn char **PHYSFS_enumerateFiles(const char *dir)
 * \brief Get a file listing of a search path's directory.
 *
 * \warning In PhysicsFS versions prior to 2.1, this function would return
 *          as many items as it could in the face of a failure condition
 *          (out of memory, disk i/o error, etc). Since this meant apps
 *          couldn't distinguish between complete success and partial failure,
 *          and since the function could always return NULL to report
 *          catastrophic failures anyway, in PhysicsFS 2.1 this function's
 *          policy changed: it will either return a list of complete results
 *          or it will return NULL for any failure of any kind, so we can
 *          guarantee that the enumeration ran to completion and has no gaps
 *          in its results.
 *
 * Matching directories are interpolated. That is, if "C:\mydir" is in the
 *  search path and contains a directory "savegames" that contains "x.sav",
 *  "y.sav", and "z.sav", and there is also a "C:\userdir" in the search path
 *  that has a "savegames" subdirectory with "w.sav", then the following code:
 *
 * \code
 * char **rc = PHYSFS_enumerateFiles("savegames");
 * char **i;
 *
 * for (i = rc; *i != NULL; i++)
 *     printf(" * We've got [%s].\n", *i);
 *
 * PHYSFS_freeList(rc);
 * \endcode
 *
 *  \...will print:
 *
 * \verbatim
 * We've got [x.sav].
 * We've got [y.sav].
 * We've got [z.sav].
 * We've got [w.sav].\endverbatim
 *
 * Feel free to sort the list however you like. However, the returned data
 *  will always contain no duplicates, and will be always sorted in alphabetic
 *  (rather: case-sensitive Unicode) order for you.
 *
 * Don't forget to call PHYSFS_freeList() with the return value from this
 *  function when you are done with it.
 *
 *    \param dir directory in platform-independent notation to enumerate.
 *   \return Null-terminated array of null-terminated strings, or NULL for
 *           failure cases.
 *
 * \sa PHYSFS_enumerate
 */
    #[no_mangle]
    fn PHYSFS_enumerateFiles(dir: *const libc::c_char)
     -> *mut *mut libc::c_char;
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
    #[no_mangle]
    fn systemInitialise() -> BOOL;
    #[no_mangle]
    fn systemShutdown() -> BOOL;
    #[no_mangle]
    fn frontendInitialise(ResourceFile: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    fn frontendShutdown() -> BOOL;
    #[no_mangle]
    fn cleanSearchPath();
    #[no_mangle]
    fn registerSearchPath(path: *const libc::c_char, priority: libc::c_uint);
    #[no_mangle]
    fn rebuildSearchPath(mode: searchPathMode, force: BOOL) -> BOOL;
    #[no_mangle]
    fn gameLoop() -> GAMECODE;
    #[no_mangle]
    fn videoLoop() -> GAMECODE;
    #[no_mangle]
    fn loop_GetVideoStatus() -> BOOL;
    //set all the pause states to the state value
    #[no_mangle]
    fn setAllPauseStates(state: BOOL);
    #[no_mangle]
    fn pal_ShutDown();
    #[no_mangle]
    fn pal_AddNewPalette(pal: *mut iColour) -> libc::c_int;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_StopAll();
    /*checks that the structure stats have loaded up as expected - must be done after 
all StructureStats parts have been loaded*/
    #[no_mangle]
    fn checkStructureStats() -> BOOL;
    /* Tidy up after a mode change */
    #[no_mangle]
    fn dispModeChange() -> BOOL;
    //fog available
    #[no_mangle]
    fn pie_EnableFog(val: BOOL);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    /* Call this each loop to update the game timer */
    #[no_mangle]
    fn gameTimeUpdate();
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    //used to set the scriptWinLoseVideo variable
    #[no_mangle]
    fn frontendInitVars() -> BOOL;
    #[no_mangle]
    fn titleLoop() -> TITLECODE;
    #[no_mangle]
    fn initLoadingScreen(drawbdrop: BOOL, bRenderActive: BOOL);
    // the global case
    //#define DEFAULT_LEVEL	"CAM_2A"
    #[no_mangle]
    static mut pLevelName: [libc::c_char; 257];
    //clear the sequence list
    #[no_mangle]
    fn seq_ClearSeqList();
    //add a sequence to the list to be played
    #[no_mangle]
    fn seq_AddSeqToList(pSeqName: *mut STRING, pAudioName: *mut STRING,
                        pTextName: *mut STRING, bLoop: BOOL,
                        PSXSeqNumber: UDWORD);
    #[no_mangle]
    fn pie_ScreenFlip(ClearMode: CLEAR_MODE);
    /*returns the next sequence in the list to play*/
    #[no_mangle]
    fn seq_StartNextFullScreenVideo();
    #[no_mangle]
    fn pie_LoadBackDrop(screenType: SCREENTYPE, b3DFX: BOOL);
    // load up the data for a level
    #[no_mangle]
    fn levLoadData(pName: *mut STRING, pSaveName: *mut STRING,
                   saveType: SDWORD) -> BOOL;
    // free the currently loaded dataset
    #[no_mangle]
    fn levReleaseAll() -> BOOL;
    #[no_mangle]
    fn checkResearchStats() -> BOOL;
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn war_SetDefaultStates();
    #[no_mangle]
    fn war_getFullscreen() -> BOOL;
    /*
 * clParse.h
 *
 * All the command line values
 *
 */
    // whether to play the intro video
    #[no_mangle]
    static mut clIntroVideo: BOOL;
    // parse the commandline
    #[no_mangle]
    fn ParseCommandLine(argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> BOOL;
    #[no_mangle]
    fn ParseCommandLineEarly(argc: libc::c_int, argv: *mut *mut libc::c_char)
     -> BOOL;
    #[no_mangle]
    fn loadRenderMode() -> BOOL;
    #[no_mangle]
    fn closeConfig();
    #[no_mangle]
    fn lobbyInitialise() -> BOOL;
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    // true when interface is up and should be run.
    //the name of the save game to load from the front end
    #[no_mangle]
    static mut saveGameName: [STRING; 256];
    // UserSaveGame is TRUE when the save game is not a new level (User Save Game)
    /*This just loads up the .gam file to determine which level data to set up - split up
so can be called in levLoadData when starting a game from a load save game*/
    #[no_mangle]
    fn loadGameInit(pGameToLoad: *mut STRING) -> BOOL;
    /* Lighting.h - Alex */
    #[no_mangle]
    static mut fogStatus: UDWORD;
    #[no_mangle]
    fn screen_StopBackDrop();
    #[no_mangle]
    fn screen_RestartBackDrop();
    #[no_mangle]
    fn version() -> *const libc::c_char;
    #[no_mangle]
    fn debug_callback_stderr(_: *mut *mut libc::c_void,
                             _: *const libc::c_char);
}
pub type size_t = libc::c_uint;
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type HANDLE = *mut libc::c_void;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
/* !WIN32 */
pub type DPID = libc::c_int;
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
/* The current status of the framework */
pub type _frame_status = libc::c_uint;
// The main app window has been told to quit
// The main app window has focus back
pub const FRAME_QUIT: _frame_status = 3;
// The main app window has lost focus (might well want to pause)
pub const FRAME_SETFOCUS: _frame_status = 2;
// Everything normal
pub const FRAME_KILLFOCUS: _frame_status = 1;
pub const FRAME_OK: _frame_status = 0;
pub type FRAME_STATUS = _frame_status;
/* *
 * \file physfs.h
 *
 * Main header file for PhysicsFS.
 */
/* *
 * \mainpage PhysicsFS
 *
 * The latest version of PhysicsFS can be found at:
 *     https://icculus.org/physfs/
 *
 * PhysicsFS; a portable, flexible file i/o abstraction.
 *
 * This API gives you access to a system file system in ways superior to the
 *  stdio or system i/o calls. The brief benefits:
 *
 *   - It's portable.
 *   - It's safe. No file access is permitted outside the specified dirs.
 *   - It's flexible. Archives (.ZIP files) can be used transparently as
 *      directory structures.
 *
 * With PhysicsFS, you have a single writing directory and multiple
 *  directories (the "search path") for reading. You can think of this as a
 *  filesystem within a filesystem. If (on Windows) you were to set the
 *  writing directory to "C:\MyGame\MyWritingDirectory", then no PHYSFS calls
 *  could touch anything above this directory, including the "C:\MyGame" and
 *  "C:\" directories. This prevents an application's internal scripting
 *  language from piddling over c:\\config.sys, for example. If you'd rather
 *  give PHYSFS full access to the system's REAL file system, set the writing
 *  dir to "C:\", but that's generally A Bad Thing for several reasons.
 *
 * Drive letters are hidden in PhysicsFS once you set up your initial paths.
 *  The search path creates a single, hierarchical directory structure.
 *  Not only does this lend itself well to general abstraction with archives,
 *  it also gives better support to operating systems like MacOS and Unix.
 *  Generally speaking, you shouldn't ever hardcode a drive letter; not only
 *  does this hurt portability to non-Microsoft OSes, but it limits your win32
 *  users to a single drive, too. Use the PhysicsFS abstraction functions and
 *  allow user-defined configuration options, too. When opening a file, you
 *  specify it like it was on a Unix filesystem: if you want to write to
 *  "C:\MyGame\MyConfigFiles\game.cfg", then you might set the write dir to
 *  "C:\MyGame" and then open "MyConfigFiles/game.cfg". This gives an
 *  abstraction across all platforms. Specifying a file in this way is termed
 *  "platform-independent notation" in this documentation. Specifying a
 *  a filename in a form such as "C:\mydir\myfile" or
 *  "MacOS hard drive:My Directory:My File" is termed "platform-dependent
 *  notation". The only time you use platform-dependent notation is when
 *  setting up your write directory and search path; after that, all file
 *  access into those directories are done with platform-independent notation.
 *
 * All files opened for writing are opened in relation to the write directory,
 *  which is the root of the writable filesystem. When opening a file for
 *  reading, PhysicsFS goes through the search path. This is NOT the
 *  same thing as the PATH environment variable. An application using
 *  PhysicsFS specifies directories to be searched which may be actual
 *  directories, or archive files that contain files and subdirectories of
 *  their own. See the end of these docs for currently supported archive
 *  formats.
 *
 * Once the search path is defined, you may open files for reading. If you've
 *  got the following search path defined (to use a win32 example again):
 *
 *  - C:\\mygame
 *  - C:\\mygame\\myuserfiles
 *  - D:\\mygamescdromdatafiles
 *  - C:\\mygame\\installeddatafiles.zip
 *
 * Then a call to PHYSFS_openRead("textfiles/myfile.txt") (note the directory
 *  separator, lack of drive letter, and lack of dir separator at the start of
 *  the string; this is platform-independent notation) will check for
 *  C:\\mygame\\textfiles\\myfile.txt, then
 *  C:\\mygame\\myuserfiles\\textfiles\\myfile.txt, then
 *  D:\\mygamescdromdatafiles\\textfiles\\myfile.txt, then, finally, for
 *  textfiles\\myfile.txt inside of C:\\mygame\\installeddatafiles.zip.
 *  Remember that most archive types and platform filesystems store their
 *  filenames in a case-sensitive manner, so you should be careful to specify
 *  it correctly.
 *
 * Files opened through PhysicsFS may NOT contain "." or ".." or ":" as dir
 *  elements. Not only are these meaningless on MacOS Classic and/or Unix,
 *  they are a security hole. Also, symbolic links (which can be found in
 *  some archive types and directly in the filesystem on Unix platforms) are
 *  NOT followed until you call PHYSFS_permitSymbolicLinks(). That's left to
 *  your own discretion, as following a symlink can allow for access outside
 *  the write dir and search paths. For portability, there is no mechanism for
 *  creating new symlinks in PhysicsFS.
 *
 * The write dir is not included in the search path unless you specifically
 *  add it. While you CAN change the write dir as many times as you like,
 *  you should probably set it once and stick to it. Remember that your
 *  program will not have permission to write in every directory on Unix and
 *  NT systems.
 *
 * All files are opened in binary mode; there is no endline conversion for
 *  textfiles. Other than that, PhysicsFS has some convenience functions for
 *  platform-independence. There is a function to tell you the current
 *  platform's dir separator ("\\" on windows, "/" on Unix, ":" on MacOS),
 *  which is needed only to set up your search/write paths. There is a
 *  function to tell you what CD-ROM drives contain accessible discs, and a
 *  function to recommend a good search path, etc.
 *
 * A recommended order for the search path is the write dir, then the base dir,
 *  then the cdrom dir, then any archives discovered. Quake 3 does something
 *  like this, but moves the archives to the start of the search path. Build
 *  Engine games, like Duke Nukem 3D and Blood, place the archives last, and
 *  use the base dir for both searching and writing. There is a helper
 *  function (PHYSFS_setSaneConfig()) that puts together a basic configuration
 *  for you, based on a few parameters. Also see the comments on
 *  PHYSFS_getBaseDir(), and PHYSFS_getPrefDir() for info on what those
 *  are and how they can help you determine an optimal search path.
 *
 * PhysicsFS 2.0 adds the concept of "mounting" archives to arbitrary points
 *  in the search path. If a zipfile contains "maps/level.map" and you mount
 *  that archive at "mods/mymod", then you would have to open
 *  "mods/mymod/maps/level.map" to access the file, even though "mods/mymod"
 *  isn't actually specified in the .zip file. Unlike the Unix mentality of
 *  mounting a filesystem, "mods/mymod" doesn't actually have to exist when
 *  mounting the zipfile. It's a "virtual" directory. The mounting mechanism
 *  allows the developer to seperate archives in the tree and avoid trampling
 *  over files when added new archives, such as including mod support in a
 *  game...keeping external content on a tight leash in this manner can be of
 *  utmost importance to some applications.
 *
 * PhysicsFS is mostly thread safe. The errors returned by
 *  PHYSFS_getLastErrorCode() are unique by thread, and library-state-setting
 *  functions are mutex'd. For efficiency, individual file accesses are 
 *  not locked, so you can not safely read/write/seek/close/etc the same 
 *  file from two threads at the same time. Other race conditions are bugs 
 *  that should be reported/patched.
 *
 * While you CAN use stdio/syscall file access in a program that has PHYSFS_*
 *  calls, doing so is not recommended, and you can not directly use system
 *  filehandles with PhysicsFS and vice versa (but as of PhysicsFS 2.1, you
 *  can wrap them in a PHYSFS_Io interface yourself if you wanted to).
 *
 * Note that archives need not be named as such: if you have a ZIP file and
 *  rename it with a .PKG extension, the file will still be recognized as a
 *  ZIP archive by PhysicsFS; the file's contents are used to determine its
 *  type where possible.
 *
 * Currently supported archive types:
 *   - .ZIP (pkZip/WinZip/Info-ZIP compatible)
 *   - .7Z  (7zip archives)
 *   - .ISO (ISO9660 files, CD-ROM images)
 *   - .GRP (Build Engine groupfile archives)
 *   - .PAK (Quake I/II archive format)
 *   - .HOG (Descent I/II HOG file archives)
 *   - .MVL (Descent II movielib archives)
 *   - .WAD (DOOM engine archives)
 *   - .VDF (Gothic I/II engine archives)
 *   - .SLB (Independence War archives)
 *
 * String policy for PhysicsFS 2.0 and later:
 *
 * PhysicsFS 1.0 could only deal with null-terminated ASCII strings. All high
 *  ASCII chars resulted in undefined behaviour, and there was no Unicode
 *  support at all. PhysicsFS 2.0 supports Unicode without breaking binary
 *  compatibility with the 1.0 API by using UTF-8 encoding of all strings
 *  passed in and out of the library.
 *
 * All strings passed through PhysicsFS are in null-terminated UTF-8 format.
 *  This means that if all you care about is English (ASCII characters <= 127)
 *  then you just use regular C strings. If you care about Unicode (and you
 *  should!) then you need to figure out what your platform wants, needs, and
 *  offers. If you are on Windows before Win2000 and build with Unicode
 *  support, your TCHAR strings are two bytes per character (this is called
 *  "UCS-2 encoding"). Any modern Windows uses UTF-16, which is two bytes
 *  per character for most characters, but some characters are four. You
 *  should convert them to UTF-8 before handing them to PhysicsFS with
 *  PHYSFS_utf8FromUtf16(), which handles both UTF-16 and UCS-2. If you're
 *  using Unix or Mac OS X, your wchar_t strings are four bytes per character
 *  ("UCS-4 encoding", sometimes called "UTF-32"). Use PHYSFS_utf8FromUcs4().
 *  Mac OS X can give you UTF-8 directly from a CFString or NSString, and many
 *  Unixes generally give you C strings in UTF-8 format everywhere. If you
 *  have a single-byte high ASCII charset, like so-many European "codepages"
 *  you may be out of luck. We'll convert from "Latin1" to UTF-8 only, and
 *  never back to Latin1. If you're above ASCII 127, all bets are off: move
 *  to Unicode or use your platform's facilities. Passing a C string with
 *  high-ASCII data that isn't UTF-8 encoded will NOT do what you expect!
 *
 * Naturally, there's also PHYSFS_utf8ToUcs2(), PHYSFS_utf8ToUtf16(), and
 *  PHYSFS_utf8ToUcs4() to get data back into a format you like. Behind the
 *  scenes, PhysicsFS will use Unicode where possible: the UTF-8 strings on
 *  Windows will be converted and used with the multibyte Windows APIs, for
 *  example.
 *
 * PhysicsFS offers basic encoding conversion support, but not a whole string
 *  library. Get your stuff into whatever format you can work with.
 *
 * Most platforms supported by PhysicsFS 2.1 and later fully support Unicode.
 *  Some older platforms have been dropped (Windows 95, Mac OS 9). Some, like
 *  OS/2, might be able to convert to a local codepage or will just fail to
 *  open/create the file. Modern OSes (macOS, Linux, Windows, etc) should all
 *  be fine.
 *
 * Many game-specific archivers are seriously unprepared for Unicode (the
 *  Descent HOG/MVL and Build Engine GRP archivers, for example, only offer a
 *  DOS 8.3 filename, for example). Nothing can be done for these, but they
 *  tend to be legacy formats for existing content that was all ASCII (and
 *  thus, valid UTF-8) anyhow. Other formats, like .ZIP, don't explicitly
 *  offer Unicode support, but unofficially expect filenames to be UTF-8
 *  encoded, and thus Just Work. Most everything does the right thing without
 *  bothering you, but it's good to be aware of these nuances in case they
 *  don't.
 *
 *
 * Other stuff:
 *
 * Please see the file LICENSE.txt in the source's root directory for
 *  licensing and redistribution rights.
 *
 * Please see the file CREDITS.txt in the source's "docs" directory for
 *  a more or less complete list of who's responsible for this.
 *
 *  \author Ryan C. Gordon.
 */
/* technically, this arrived in gcc 3.1, but oh well. */
/* !!! FIXME: look into this later. */
/* *
 * \typedef PHYSFS_uint8
 * \brief An unsigned, 8-bit integer type.
 */
pub type PHYSFS_uint8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_Version {
    pub major: PHYSFS_uint8,
    pub minor: PHYSFS_uint8,
    pub patch: PHYSFS_uint8,
}
pub type searchPathMode = libc::c_uint;
pub const mod_multiplay: searchPathMode = 2;
pub const mod_campaign: searchPathMode = 1;
pub const mod_clean: searchPathMode = 0;
pub type GAMECODE = libc::c_uint;
pub const GAMECODE_LOADGAME: GAMECODE = 6;
pub const GAMECODE_FASTEXIT: GAMECODE = 5;
pub const GAMECODE_NEWLEVEL: GAMECODE = 4;
pub const GAMECODE_PLAYVIDEO: GAMECODE = 3;
pub const GAMECODE_QUITGAME: GAMECODE = 2;
pub const GAMECODE_RESTARTGAME: GAMECODE = 1;
pub const GAMECODE_CONTINUE: GAMECODE = 0;
pub type uint8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
pub type TITLECODE = libc::c_uint;
pub const TITLECODE_SAVEGAMELOAD: TITLECODE = 4;
pub const TITLECODE_SHOWINTRO: TITLECODE = 3;
pub const TITLECODE_QUITGAME: TITLECODE = 2;
pub const TITLECODE_STARTGAME: TITLECODE = 1;
pub const TITLECODE_CONTINUE: TITLECODE = 0;
pub type CLEAR_MODE = libc::c_uint;
pub const CLEAR_FOG: CLEAR_MODE = 3;
pub const CLEAR_BLACK: CLEAR_MODE = 2;
pub const CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD: CLEAR_MODE = 1;
pub const CLEAR_OFF: CLEAR_MODE = 0;
pub type _screenType = libc::c_uint;
pub const SCREEN_COVERMOUNT: _screenType = 8;
pub const SCREEN_SLIDE5: _screenType = 7;
pub const SCREEN_SLIDE4: _screenType = 6;
pub const SCREEN_SLIDE3: _screenType = 5;
pub const SCREEN_SLIDE2: _screenType = 4;
pub const SCREEN_SLIDE1: _screenType = 3;
pub const SCREEN_MISSIONEND: _screenType = 2;
pub const SCREEN_CREDITS: _screenType = 1;
pub const SCREEN_RANDOMBDROP: _screenType = 0;
pub type SCREENTYPE = _screenType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SESSIONDESC {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub host: [libc::c_char; 16],
    pub dwMaxPlayers: DWORD,
    pub dwCurrentPlayers: DWORD,
    pub dwUser1: DWORD,
    pub dwUser2: DWORD,
    pub dwUser3: DWORD,
    pub dwUser4: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYER {
    pub dpid: DPID,
    pub name: [libc::c_char; 64],
    pub bHost: BOOL,
    pub bSpectator: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETPLAY {
    pub games: [GAMESTRUCT; 12],
    pub players: [PLAYER; 8],
    pub playercount: UDWORD,
    pub dpidPlayer: DPID,
    pub bComms: BOOL,
    pub bHost: BOOL,
    pub bLobbyLaunched: BOOL,
    pub bSpectator: BOOL,
    pub bEncryptAllPackets: BOOL,
    pub cryptKey: [UDWORD; 4],
    pub bCaptureInUse: BOOL,
    pub bAllowCaptureRecord: BOOL,
    pub bAllowCapturePlay: BOOL,
}
#[no_mangle]
pub static mut datadir: [libc::c_char; 260] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Global that src/clparse.c:ParseCommandLine can write to, so it can override the default datadir on runtime. Needs to be \0 on startup for ParseCommandLine to work!
#[no_mangle]
pub static mut global_mods: [*mut libc::c_char; 100] =
    [0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut campaign_mods: [*mut libc::c_char; 100] =
    [0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub static mut multiplay_mods: [*mut libc::c_char; 100] =
    [0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char,
     0 as *const libc::c_char as *mut libc::c_char];
// Warzone 2100 . Pumpkin Studios
#[no_mangle]
pub static mut gameStatus: UDWORD = 1 as libc::c_int as UDWORD;
// Start game in title mode.
#[no_mangle]
pub static mut lastStatus: UDWORD = 1 as libc::c_int as UDWORD;
//flag to indicate when initialisation is complete
#[no_mangle]
pub static mut videoInitialised: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut gameInitialised: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut frontendInitialised: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut reInit: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bDisableLobby: BOOL = 0;
#[no_mangle]
pub static mut pQUEUE: BOOL = 1 as libc::c_int;
//This is used to control our pQueue list. Always ON except for SP games! -Q
#[no_mangle]
pub static mut SaveGamePath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut ScreenDumpPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut MultiForcesPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut MultiCustomMapsPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut MultiPlayersPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut KeyMapPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut UserMusicPath: [libc::c_char; 260] = [0; 260];
#[no_mangle]
pub static mut RegFilePath: [libc::c_char; 260] = [0; 260];
/*
BOOL checkDisableLobby(void)
{
	BOOL	disable;

	disable = FALSE;
	if(!openWarzoneKey())
	{
		return FALSE;
	}

	if (!getWarzoneKeyNumeric("DisableLobby",(DWORD*)&disable))
	{
		return FALSE;
	}

	if (!closeWarzoneKey())
	{
		return FALSE;
	}

	return disable;
}
*/
unsafe extern "C" fn inList(mut list: *mut *mut libc::c_char,
                            mut item: *const libc::c_char) -> BOOL {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        if strcmp(*list.offset(i as isize), item) == 0 as libc::c_int {
            return 1 as libc::c_int
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addSubdirs(mut basedir: *const libc::c_char,
                                    mut subdir: *const libc::c_char,
                                    appendToPath: BOOL,
                                    mut checkList: *mut *mut libc::c_char) {
    let mut tmpstr: [libc::c_char; 260] = [0; 260];
    let mut subdirlist: *mut *mut libc::c_char =
        PHYSFS_enumerateFiles(subdir);
    let mut i: *mut *mut libc::c_char = subdirlist;
    while !(*i).is_null() {
        // DEBUG
        if checkList.is_null() || inList(checkList, *i) != 0 {
            strcpy(tmpstr.as_mut_ptr(), basedir);
            strcat(tmpstr.as_mut_ptr(), subdir);
            strcat(tmpstr.as_mut_ptr(), PHYSFS_getDirSeparator());
            strcat(tmpstr.as_mut_ptr(), *i);
            // DEBUG
            PHYSFS_addToSearchPath(tmpstr.as_mut_ptr(), appendToPath);
        }
        i = i.offset(1)
    }
    PHYSFS_freeList(subdirlist as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn removeSubdirs(mut basedir: *const libc::c_char,
                                       mut subdir: *const libc::c_char,
                                       mut checkList:
                                           *mut *mut libc::c_char) {
    let mut tmpstr: [libc::c_char; 260] = [0; 260];
    let mut subdirlist: *mut *mut libc::c_char =
        PHYSFS_enumerateFiles(subdir);
    let mut i: *mut *mut libc::c_char = subdirlist;
    while !(*i).is_null() {
        // DEBUG
        if checkList.is_null() || inList(checkList, *i) != 0 {
            strcpy(tmpstr.as_mut_ptr(), basedir);
            strcat(tmpstr.as_mut_ptr(), subdir);
            strcat(tmpstr.as_mut_ptr(), PHYSFS_getDirSeparator());
            strcat(tmpstr.as_mut_ptr(), *i);
            // DEBUG
            PHYSFS_removeFromSearchPath(tmpstr.as_mut_ptr());
        }
        i = i.offset(1)
    }
    PHYSFS_freeList(subdirlist as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn printSearchPath() {
    let mut i: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut searchPath: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    debug(LOG_WZ, b"Search paths:\x00" as *const u8 as *const libc::c_char);
    searchPath = PHYSFS_getSearchPath();
    i = searchPath;
    while !(*i).is_null() {
        debug(LOG_WZ, b"    [%s]\x00" as *const u8 as *const libc::c_char,
              *i);
        i = i.offset(1)
    }
    PHYSFS_freeList(searchPath as *mut libc::c_void);
}
/* **************************************************************************
	Initialize the PhysicsFS library.
***************************************************************************/
unsafe extern "C" fn initialize_PhysicsFS() {
    let mut compiled: PHYSFS_Version =
        PHYSFS_Version{major: 0,
                       minor: 0,
                       patch:
                           0,}; // Use PhysFS supplied UserDir (As fallback on Windows / Mac, default on Linux)
    let mut linked: PHYSFS_Version =
        PHYSFS_Version{major: 0, minor: 0, patch: 0,};
    let mut tmpstr: [libc::c_char; 260] =
        ['\u{0}' as i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    compiled.major = 3 as libc::c_int as PHYSFS_uint8;
    compiled.minor = 0 as libc::c_int as PHYSFS_uint8;
    compiled.patch = 2 as libc::c_int as PHYSFS_uint8;
    PHYSFS_getLinkedVersion(&mut linked);
    debug(LOG_WZ,
          b"Compiled against PhysFS version: %d.%d.%d\x00" as *const u8 as
              *const libc::c_char, compiled.major as libc::c_int,
          compiled.minor as libc::c_int, compiled.patch as libc::c_int);
    debug(LOG_WZ,
          b"Linked against PhysFS version: %d.%d.%d\x00" as *const u8 as
              *const libc::c_char, linked.major as libc::c_int,
          linked.minor as libc::c_int, linked.patch as libc::c_int);
    strcpy(tmpstr.as_mut_ptr(), PHYSFS_getUserDir());
    if PHYSFS_setWriteDir(tmpstr.as_mut_ptr()) == 0 {
        // Workaround for PhysFS not creating the writedir as expected.
        debug(LOG_ERROR,
              b"Error setting write directory to \"%s\": %s\x00" as *const u8
                  as *const libc::c_char, PHYSFS_getUserDir(),
              PHYSFS_getLastError());
        exit(1 as libc::c_int);
    }
    if PHYSFS_mkdir(b".warzone2100\x00" as *const u8 as *const libc::c_char)
           == 0 {
        // s.a.
        debug(LOG_ERROR,
              b"Error creating directory \"%s\": %s\x00" as *const u8 as
                  *const libc::c_char,
              b".warzone2100\x00" as *const u8 as *const libc::c_char,
              PHYSFS_getLastError());
        exit(1 as libc::c_int);
    }
    // Append the Warzone subdir
    strcat(tmpstr.as_mut_ptr(),
           b".warzone2100\x00" as *const u8 as *const libc::c_char);
    strcat(tmpstr.as_mut_ptr(), PHYSFS_getDirSeparator());
    if PHYSFS_setWriteDir(tmpstr.as_mut_ptr()) == 0 {
        debug(LOG_ERROR,
              b"Error setting write directory to \"%s\": %s\x00" as *const u8
                  as *const libc::c_char, tmpstr.as_mut_ptr(),
              PHYSFS_getLastError());
        exit(1 as libc::c_int);
    }
    // User's home dir first so we allways see what we write
    PHYSFS_addToSearchPath(PHYSFS_getWriteDir(), 0 as libc::c_int);
    PHYSFS_permitSymbolicLinks(1 as libc::c_int);
    debug(LOG_WZ, b"Write dir: %s\x00" as *const u8 as *const libc::c_char,
          PHYSFS_getWriteDir());
    debug(LOG_WZ, b"Base dir: %s\x00" as *const u8 as *const libc::c_char,
          PHYSFS_getBaseDir());
}
/*
 * \fn void scanDataDirs( void )
 * \brief Adds default data dirs
 *
 * Priority:
 * Lower loads first. Current:
 * -datadir > User's home dir > SVN data > AutoPackage > BaseDir > DEFAULT_DATADIR
 *
 * Only -datadir and home dir are allways examined. Others only if data still not found.
 *
 * We need ParseCommandLine, before we can add any mods...
 *
 * \sa loadMods
 */
#[no_mangle]
pub unsafe extern "C" fn scanDataDirs() {
    let mut tmpstr: [libc::c_char; 260] = [0; 260];
    let mut prefix: [libc::c_char; 260] =
        ['\u{0}' as i32 as libc::c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // Find out which PREFIX we are in...
    strcpy(tmpstr.as_mut_ptr(),
           PHYSFS_getBaseDir()); // Trim ending '/', which getBaseDir always provides
    *strrchr(tmpstr.as_mut_ptr(), *PHYSFS_getDirSeparator() as libc::c_int) =
        '\u{0}' as i32 as libc::c_char;
    strncpy(prefix.as_mut_ptr(), PHYSFS_getBaseDir(),
            strrchr(tmpstr.as_mut_ptr(),
                    *PHYSFS_getDirSeparator() as
                        libc::c_int).wrapping_offset_from(tmpstr.as_mut_ptr())
                as libc::c_int as libc::c_uint);
    atexit(Some(cleanSearchPath as unsafe extern "C" fn() -> ()));
    // Commandline supplied datadir
    if strlen(datadir.as_mut_ptr()) != 0 as libc::c_int as libc::c_uint {
        registerSearchPath(datadir.as_mut_ptr() as *const libc::c_char,
                           1 as libc::c_int as libc::c_uint);
    }
    // User's home dir
    registerSearchPath(PHYSFS_getWriteDir(),
                       2 as libc::c_int as libc::c_uint);
    rebuildSearchPath(mod_multiplay, 1 as libc::c_int);
    if PHYSFS_exists(b"gamedesc.lev\x00" as *const u8 as *const libc::c_char)
           == 0 {
        // Data in SVN dir
        strcpy(tmpstr.as_mut_ptr(), prefix.as_mut_ptr());
        strcat(tmpstr.as_mut_ptr(),
               b"/data/\x00" as *const u8 as *const libc::c_char);
        registerSearchPath(tmpstr.as_mut_ptr() as *const libc::c_char,
                           3 as libc::c_int as libc::c_uint);
        rebuildSearchPath(mod_multiplay, 1 as libc::c_int);
        if PHYSFS_exists(b"gamedesc.lev\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            // Relocation for AutoPackage
            strcpy(tmpstr.as_mut_ptr(), prefix.as_mut_ptr());
            strcat(tmpstr.as_mut_ptr(),
                   b"/share/warzone2100/\x00" as *const u8 as
                       *const libc::c_char);
            registerSearchPath(tmpstr.as_mut_ptr() as *const libc::c_char,
                               4 as libc::c_int as libc::c_uint);
            rebuildSearchPath(mod_multiplay, 1 as libc::c_int);
            if PHYSFS_exists(b"gamedesc.lev\x00" as *const u8 as
                                 *const libc::c_char) == 0 {
                // Program dir
                registerSearchPath(PHYSFS_getBaseDir(),
                                   5 as libc::c_int as libc::c_uint);
                rebuildSearchPath(mod_multiplay, 1 as libc::c_int);
                if PHYSFS_exists(b"gamedesc.lev\x00" as *const u8 as
                                     *const libc::c_char) == 0 {
                    // Guessed fallback default datadir on Unix
                    registerSearchPath(b"/usr/local/share/warzone2100\x00" as
                                           *const u8 as *const libc::c_char,
                                       6 as libc::c_int as libc::c_uint);
                    rebuildSearchPath(mod_multiplay, 1 as libc::c_int);
                }
            }
        }
    }
    /* * Debugging and sanity checks **/
    printSearchPath();
    if PHYSFS_exists(b"gamedesc.lev\x00" as *const u8 as *const libc::c_char)
           != 0 {
        debug(LOG_WZ,
              b"gamedesc.lev found at %s\x00" as *const u8 as
                  *const libc::c_char,
              PHYSFS_getRealDir(b"gamedesc.lev\x00" as *const u8 as
                                    *const libc::c_char));
    } else {
        debug(LOG_ERROR,
              b"Could not find game data. Aborting.\x00" as *const u8 as
                  *const libc::c_char);
        exit(1 as libc::c_int);
    };
}
/* **************************************************************************
	Make a directory in write path and set a variable to point to it.
***************************************************************************/
unsafe extern "C" fn make_dir(mut dest: *mut libc::c_char,
                              mut dirname: *mut libc::c_char,
                              mut subdir: *mut libc::c_char) {
    strcpy(dest, dirname); //, firstTime = TRUE;
    if !subdir.is_null() {
        strcat(dest, b"/\x00" as *const u8 as *const libc::c_char);
        strcat(dest, subdir);
    }
    let mut l: size_t = strlen(dest);
    if *dest.offset(l.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
           as libc::c_int != '/' as i32 {
        *dest.offset(l as isize) = '/' as i32 as libc::c_char;
        *dest.offset(l.wrapping_add(1 as libc::c_int as libc::c_uint) as
                         isize) = '\u{0}' as i32 as libc::c_char
    }
    PHYSFS_mkdir(dest);
    if PHYSFS_mkdir(dest) == 0 {
        debug(LOG_ERROR,
              b"Unable to create directory \"%s\" in write dir \"%s\"!\x00" as
                  *const u8 as *const libc::c_char, dest,
              PHYSFS_getWriteDir());
        exit(1 as libc::c_int);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut frameRet: FRAME_STATUS = FRAME_OK;
    let mut quit: BOOL = 0 as libc::c_int;
    let mut Restart: BOOL = 0 as libc::c_int;
    let mut paused: BOOL = 0 as libc::c_int;
    let mut bVidMem: BOOL = 0 as libc::c_int;
    let mut dispBitDepth: SDWORD = 8 as libc::c_int;
    let mut introVideoControl: SDWORD = 3 as libc::c_int;
    let mut loopStatus: libc::c_int = 0 as libc::c_int;
    let mut psPaletteBuffer: *mut iColour = 0 as *mut iColour;
    let mut pSize: UDWORD = 0;
    /* ** Initialize the debug subsystem ***/
    // _MSC_VER
    debug_init();
    atexit(Some(debug_exit as unsafe extern "C" fn() -> ()));
    debug_register_callback(Some(debug_callback_stderr as
                                     unsafe extern "C" fn(_:
                                                              *mut *mut libc::c_void,
                                                          _:
                                                              *const libc::c_char)
                                         -> ()), None, None,
                            0 as *mut libc::c_void);
    // WIN32
    // find early boot info
    if ParseCommandLineEarly(argc, argv) == 0 { return -(1 as libc::c_int) }
    debug(LOG_WZ,
          b"Warzone 2100 - Version %s - Built %s\x00" as *const u8 as
              *const libc::c_char, version(),
          b"May 29 2020\x00" as *const u8 as *const libc::c_char);
    /* ** Initialize PhysicsFS ***/
    PHYSFS_init(*argv.offset(0 as libc::c_int as isize));
    initialize_PhysicsFS();
    make_dir(ScreenDumpPath.as_mut_ptr(),
             b"screendumps\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 0 as *mut libc::c_char);
    make_dir(SaveGamePath.as_mut_ptr(),
             b"savegame\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 0 as *mut libc::c_char);
    make_dir(MultiPlayersPath.as_mut_ptr(),
             b"multiplay\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char, 0 as *mut libc::c_char);
    make_dir(MultiPlayersPath.as_mut_ptr(),
             b"multiplay\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char,
             b"players\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    make_dir(MultiForcesPath.as_mut_ptr(),
             b"multiplay\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char,
             b"forces\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    make_dir(MultiCustomMapsPath.as_mut_ptr(),
             b"multiplay\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char,
             b"custommaps\x00" as *const u8 as *const libc::c_char as
                 *mut libc::c_char);
    /* Put these files in the writedir root */
    strcpy(RegFilePath.as_mut_ptr(),
           b"config\x00" as *const u8 as *const libc::c_char);
    strcpy(KeyMapPath.as_mut_ptr(),
           b"keymap.map\x00" as *const u8 as *const libc::c_char);
    strcpy(UserMusicPath.as_mut_ptr(),
           b"music\x00" as *const u8 as *const libc::c_char);
    // initialise all the command line states
    clIntroVideo = 0 as libc::c_int;
    war_SetDefaultStates();
    'c_19141:
        loop  {
            //jump here from the end if re_initialising
            if blkInitialise() == 0 {
                return 0 as libc::c_int
            } //get the registry entry for clRendMode
            loadRenderMode();
            bDisableLobby = 0 as libc::c_int;
            // parse the command line
            if reInit == 0 {
                if ParseCommandLine(argc, argv) == 0 {
                    return -(1 as libc::c_int)
                }
                atexit(Some(closeConfig as unsafe extern "C" fn() -> ()));
            }
            scanDataDirs();
            debug(LOG_MAIN,
                  b"reinitializing\x00" as *const u8 as *const libc::c_char);
            // find out if the lobby stuff has been disabled
//	bDisableLobby = checkDisableLobby();
            if bDisableLobby == 0 && lobbyInitialise() == 0 {
                // ajl. Init net stuff. Lobby can modify startup conditions like commandline.
                return -(1 as libc::c_int)
            } //just so we dont restart again
            reInit = 0 as libc::c_int;
            bVidMem = 0 as libc::c_int;
            dispBitDepth = 8 as libc::c_int;
            //	frameDDEnumerate();
            if frameInitialise(0 as *mut libc::c_void,
                               b"Warzone 2100\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                               pie_GetVideoBufferWidth(),
                               pie_GetVideoBufferHeight(),
                               dispBitDepth as UDWORD, war_getFullscreen(),
                               bVidMem) == 0 {
                return -(1 as libc::c_int)
            } //play video
            pie_SetFogStatus(0 as libc::c_int);
            pie_ScreenFlip(CLEAR_BLACK);
            pie_ScreenFlip(CLEAR_BLACK);
            if gameStatus == 4 as libc::c_int as libc::c_uint {
                introVideoControl = 0 as libc::c_int;
                gameStatus = 1 as libc::c_int as UDWORD
            }
            //load palette
            psPaletteBuffer =
                memMallocRelease((256 as libc::c_int as
                                      libc::c_uint).wrapping_mul(::std::mem::size_of::<iColour>()
                                                                     as
                                                                     libc::c_ulong).wrapping_add(1
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint))
                    as *mut iColour;
            if psPaletteBuffer.is_null() {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            if loadFileToBuffer(b"palette.bin\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                psPaletteBuffer as *mut libc::c_char,
                                (256 as libc::c_int as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<iColour>()
                                                                    as
                                                                    libc::c_ulong).wrapping_add(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint),
                                &mut pSize) == 0 {
                debug(LOG_ERROR,
                      b"Couldn\'t load palette data\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            pal_AddNewPalette(psPaletteBuffer);
            memFreeRelease(psPaletteBuffer as *mut libc::c_void);
            psPaletteBuffer = 0 as *mut iColour;
            pie_LoadBackDrop(SCREEN_RANDOMBDROP, 0 as libc::c_int);
            pie_SetFogStatus(0 as libc::c_int);
            pie_ScreenFlip(CLEAR_BLACK);
            quit = 0 as libc::c_int;
            if systemInitialise() == 0 { return -(1 as libc::c_int) }
            //set all the pause states to false
            setAllPauseStates(0 as libc::c_int); // End of !quit loop.
            while quit == 0 {
                // Do the game mode specific initialisation.
                match gameStatus {
                    1 => {
                        screen_RestartBackDrop();
                        //loadLevels(DIR_MULTIPLAYER);
                        if frontendInitialise(b"wrf/frontend.wrf\x00" as
                                                  *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) == 0 {
                            break 'c_19141 ;
                        }
                        frontendInitialised = 1 as libc::c_int;
                        frontendInitVars();
                        //if intro required set up the video
                        if introVideoControl <= 1 as libc::c_int {
                            seq_ClearSeqList();
                            seq_AddSeqToList(b"eidos-logo.rpl\x00" as
                                                 *const u8 as
                                                 *const libc::c_char as
                                                 *mut STRING,
                                             0 as *mut STRING,
                                             0 as *mut STRING,
                                             0 as libc::c_int,
                                             0 as libc::c_int as UDWORD);
                            seq_AddSeqToList(b"pumpkin.rpl\x00" as *const u8
                                                 as *const libc::c_char as
                                                 *mut STRING,
                                             0 as *mut STRING,
                                             0 as *mut STRING,
                                             0 as libc::c_int,
                                             0 as libc::c_int as UDWORD);
                            seq_AddSeqToList(b"titles.rpl\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut STRING,
                                             0 as *mut STRING,
                                             0 as *mut STRING,
                                             0 as libc::c_int,
                                             0 as libc::c_int as UDWORD);
                            seq_AddSeqToList(b"devastation.rpl\x00" as
                                                 *const u8 as
                                                 *const libc::c_char as
                                                 *mut STRING,
                                             0 as *mut STRING,
                                             b"devastation.txa\x00" as
                                                 *const u8 as
                                                 *const libc::c_char as
                                                 *mut STRING,
                                             0 as libc::c_int,
                                             0 as libc::c_int as UDWORD);
                            seq_StartNextFullScreenVideo();
                            introVideoControl = 2 as libc::c_int
                        }
                    }
                    5 => {
                        screen_RestartBackDrop();
                        gameStatus = 3 as libc::c_int as UDWORD;
                        // load up a save game
                        if loadGameInit(saveGameName.as_mut_ptr()) == 0 {
                            break 'c_19141 ;
                        }
                        /*if (!levLoadData(pLevelName, saveGameName)) {
					return -1;
				}*/
                        screen_StopBackDrop();
                    }
                    3 => {
                        if levLoadData(pLevelName.as_mut_ptr(),
                                       0 as *mut STRING, 0 as libc::c_int) ==
                               0 {
                            break 'c_19141 ;
                        }
                        //after data is loaded check the research stats are valid
                        if checkResearchStats() == 0 {
                            debug(LOG_ERROR,
                                  b"Invalid Research Stats\x00" as *const u8
                                      as *const libc::c_char);
                            break 'c_19141 ;
                        } else if checkStructureStats() == 0 {
                            debug(LOG_ERROR,
                                  b"Invalid Structure Stats\x00" as *const u8
                                      as *const libc::c_char);
                            break 'c_19141 ;
                        } else {
                            //and check the structure stats are valid
                            //set a flag for the trigger/event system to indicate initialisation is complete
                            gameInitialised = 1 as libc::c_int;
                            screen_StopBackDrop();
                        }
                    }
                    4 => {
                        debug(LOG_ERROR,
                              b"Video_mode no longer valid\x00" as *const u8
                                  as *const libc::c_char);
                        abort();
                    }
                    _ => {
                        debug(LOG_ERROR,
                              b"Unknown game status on shutdown!\x00" as
                                  *const u8 as *const libc::c_char);
                    }
                }
                debug(LOG_MAIN,
                      b"Entering main loop\x00" as *const u8 as
                          *const libc::c_char);
                Restart = 0 as libc::c_int;
                //firstTime = TRUE;
                while Restart == 0 {
                    frameRet = frameUpdate(); // End of !Restart loop.
                    /* Unused...
			if (pie_GetRenderEngine() == ENGINE_OPENGL)	//Was ENGINE_D3D -Q
			{
				if ( frameRet == FRAME_SETFOCUS )
				{
//					D3DTestCooperativeLevel( TRUE );
				}
				else
				{
//					D3DTestCooperativeLevel( FALSE );
				}
			}
*/
                    match frameRet as libc::c_uint {
                        1 => {
                            paused = 1 as libc::c_int;
                            gameTimeStop();
                            audio_StopAll();
                        }
                        2 => {
                            paused = 0 as libc::c_int;
                            gameTimeStart();
                            if dispModeChange() == 0 {
                                quit = 1 as libc::c_int;
                                Restart = 1 as libc::c_int
                            }
                        }
                        3 => {
                            debug(LOG_MAIN,
                                  b"frame quit\x00" as *const u8 as
                                      *const libc::c_char);
                            quit = 1 as libc::c_int;
                            Restart = 1 as libc::c_int
                        }
                        _ => { }
                    }
                    lastStatus = gameStatus;
                    if paused == 0 && quit == 0 {
                        match gameStatus {
                            1 => {
                                if loop_GetVideoStatus() != 0 {
                                    videoLoop();
                                } else {
                                    match titleLoop() as libc::c_uint {
                                        2 => {
                                            debug(LOG_MAIN,
                                                  b"TITLECODE_QUITGAME\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            Restart = 1 as libc::c_int;
                                            quit = 1 as libc::c_int
                                        }
                                        4 => {
                                            //						case TITLECODE_ATTRACT:
	//							DBPRINTF(("TITLECODE_ATTRACT\n"));
	//							break;
                                            debug(LOG_MAIN,
                                                  b"TITLECODE_SAVEGAMELOAD\x00"
                                                      as *const u8 as
                                                      *const libc::c_char); //play the video but dont init the sound system
                                            gameStatus =
                                                5 as libc::c_int as UDWORD;
                                            Restart = 1 as libc::c_int
                                        }
                                        1 => {
                                            debug(LOG_MAIN,
                                                  b"TITLECODE_STARTGAME\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                            gameStatus =
                                                3 as libc::c_int as UDWORD;
                                            Restart = 1 as libc::c_int
                                        }
                                        3 => {
                                            debug(LOG_MAIN,
                                                  b"TITLECODE_SHOWINTRO\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                            seq_ClearSeqList();
                                            seq_AddSeqToList(b"eidos-logo.rpl\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int
                                                                 as UDWORD);
                                            seq_AddSeqToList(b"pumpkin.rpl\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int
                                                                 as UDWORD);
                                            seq_AddSeqToList(b"titles.rpl\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as *mut STRING,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int
                                                                 as UDWORD);
                                            seq_AddSeqToList(b"devastation.rpl\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut STRING,
                                                             0 as *mut STRING,
                                                             b"devastation.txa\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut STRING,
                                                             0 as libc::c_int,
                                                             0 as libc::c_int
                                                                 as UDWORD);
                                            seq_StartNextFullScreenVideo();
                                            introVideoControl =
                                                2 as libc::c_int
                                        }
                                        0 => { }
                                        _ => {
                                            debug(LOG_ERROR,
                                                  b"Unknown code returned by titleLoop\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        }
                                    }
                                }
                            }
                            3 => {
                                /*				case GS_SAVEGAMELOAD:
					if (loopNewLevel)
					{
						//the start of a campaign/expand mission
						DBPRINTF(("GAMECODE_NEWLEVEL\n"));
						loopNewLevel = FALSE;
						// gameStatus is unchanged, just loading additional data
						Restart = TRUE;
					}
					break;
*/
                                if loop_GetVideoStatus() != 0 {
                                    videoLoop();
                                } else {
                                    loopStatus = gameLoop() as libc::c_int;
                                    match loopStatus {
                                        2 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_QUITGAME\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            gameStatus =
                                                1 as libc::c_int as UDWORD;
                                            Restart = 1 as libc::c_int;
                                            if NetPlay.bLobbyLaunched != 0 {
                                                //									changeTitleMode(QUIT);
                                                quit = 1 as libc::c_int
                                            }
                                        }
                                        5 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_FASTEXIT\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            Restart = 1 as libc::c_int;
                                            quit = 1 as libc::c_int
                                        }
                                        6 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_LOADGAME\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            Restart = 1 as libc::c_int;
                                            gameStatus =
                                                5 as libc::c_int as UDWORD
                                        }
                                        3 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_PLAYVIDEO\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            //dont schange mode any more								gameStatus = GS_VIDEO_MODE;
                                            Restart = 0 as libc::c_int
                                        }
                                        4 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_NEWLEVEL\x00" as
                                                      *const u8 as
                                                      *const libc::c_char);
                                            // gameStatus is unchanged, just loading additional data
                                            Restart = 1 as libc::c_int
                                        }
                                        1 => {
                                            debug(LOG_MAIN,
                                                  b"GAMECODE_RESTARTGAME\x00"
                                                      as *const u8 as
                                                      *const libc::c_char); //"sequences\\factory.rpl","sequences\\factory.wav");
                                            Restart = 1 as libc::c_int
                                        }
                                        0 => { }
                                        _ => {
                                            debug(LOG_ERROR,
                                                  b"Unknown code returned by gameLoop\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                        }
                                    }
                                }
                            }
                            4 => {
                                debug(LOG_ERROR,
                                      b"Video_mode no longer valid\x00" as
                                          *const u8 as *const libc::c_char);
                                if loop_GetVideoStatus() != 0 {
                                    videoLoop();
                                } else if introVideoControl <=
                                              1 as libc::c_int {
                                    seq_ClearSeqList();
                                    seq_AddSeqToList(b"factory.rpl\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as *mut STRING,
                                                     0 as *mut STRING,
                                                     0 as *mut STRING,
                                                     0 as libc::c_int,
                                                     0 as libc::c_int as
                                                         UDWORD);
                                    seq_StartNextFullScreenVideo();
                                    introVideoControl = 2 as libc::c_int
                                } else {
                                    debug(LOG_MAIN,
                                          b"VIDEO_QUIT\x00" as *const u8 as
                                              *const libc::c_char);
                                    if introVideoControl == 2 as libc::c_int {
                                        //finished playing intro video
                                        gameStatus =
                                            1 as libc::c_int as UDWORD;
                                        if videoInitialised != 0 {
                                            Restart = 1 as libc::c_int
                                        }
                                        introVideoControl = 3 as libc::c_int
                                    } else {
                                        gameStatus =
                                            3 as libc::c_int as UDWORD
                                    }
                                }
                            }
                            _ => {
                                debug(LOG_ERROR,
                                      b"Weirdy game status, I\'m afraid!!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            }
                        }
                        gameTimeUpdate();
                    }
                }
                // Do game mode specific shutdown.
                match lastStatus {
                    1 => {
                        if frontendShutdown() == 0 { break 'c_19141 ; }
                        frontendInitialised = 0 as libc::c_int
                    }
                    3 => {
                        /*			case GS_SAVEGAMELOAD:
				//get the next level to load up
				gameStatus = GS_NORMAL;
				break;*/
                        if loopStatus != GAMECODE_NEWLEVEL as libc::c_int {
                            initLoadingScreen(1 as libc::c_int,
                                              0 as
                                                  libc::c_int); // returning to f.e. do a loader.render not active
                            pie_EnableFog(0 as
                                              libc::c_int); //dont let the normal loop code set status on
                            fogStatus = 0 as libc::c_int as UDWORD;
                            if loopStatus != GAMECODE_LOADGAME as libc::c_int
                               {
                                levReleaseAll();
                            }
                        }
                        gameInitialised = 0 as libc::c_int
                    }
                    4 => {
                        debug(LOG_ERROR,
                              b"Video_mode no longer valid\x00" as *const u8
                                  as *const libc::c_char);
                        if videoInitialised != 0 {
                            videoInitialised = 0 as libc::c_int
                        }
                    }
                    _ => {
                        debug(LOG_ERROR,
                              b"Unknown game status on shutdown!\x00" as
                                  *const u8 as *const libc::c_char);
                    }
                }
            }
            debug(LOG_MAIN,
                  b"Shuting down application\x00" as *const u8 as
                      *const libc::c_char);
            systemShutdown();
            pal_ShutDown();
            frameShutDown();
            if reInit != 0 { continue ; }
            return 0 as libc::c_int
        }
    debug(LOG_ERROR,
          b"Shutting down after failure\x00" as *const u8 as
              *const libc::c_char);
    systemShutdown();
    pal_ShutDown();
    frameShutDown();
    return 1 as libc::c_int;
}
//flag to indicate when initialisation is complete
#[no_mangle]
pub unsafe extern "C" fn GetGameMode() -> UDWORD { return gameStatus; }
#[no_mangle]
pub unsafe extern "C" fn SetGameMode(mut status: UDWORD) {
    if status == 1 as libc::c_int as libc::c_uint ||
           status == 2 as libc::c_int as libc::c_uint ||
           status == 3 as libc::c_int as libc::c_uint ||
           status == 4 as libc::c_int as libc::c_uint ||
           status == 5 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"SetGameMode: invalid game mode\x00" as *const u8 as
                  *const libc::c_char);
    };
    if status == 1 as libc::c_int as libc::c_uint ||
           status == 2 as libc::c_int as libc::c_uint ||
           status == 3 as libc::c_int as libc::c_uint ||
           status == 4 as libc::c_int as libc::c_uint ||
           status == 5 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"main.c\x00" as *const u8 as *const libc::c_char,
              914 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"SetGameMode\x00")).as_ptr(),
              b"status == GS_TITLE_SCREEN || status == GS_MISSION_SCREEN || status == GS_NORMAL || status == GS_VIDEO_MODE || status == GS_SAVEGAMELOAD\x00"
                  as *const u8 as *const libc::c_char);
    };
    gameStatus = status;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
