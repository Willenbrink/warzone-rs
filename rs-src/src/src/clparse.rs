use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    /* *
 * Register a callback to be called on every call to debug()
 *
 * \param	callback	Function which does the output
 * \param	init		Initializer function which does all setup for the callback (optional, may be NULL)
 * \param	exit		Cleanup function called when unregistering the callback (optional, may be NULL)
 * \param	data		Data to be passed to all three functions (optional, may be NULL)
 */
    #[no_mangle]
    fn debug_register_callback(callback: debug_callback_fn,
                               init: debug_callback_init,
                               exit: debug_callback_exit,
                               data: *mut libc::c_void);
    /* *
 * Toggle debug output for part associated with str
 *
 * \param	str	Codepart in textformat
 */
    #[no_mangle]
    fn debug_enable_switch(str: *const libc::c_char) -> BOOL;
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
    fn abort() -> !;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut bDisableLobby: BOOL;
    #[no_mangle]
    fn SetGameMode(status: UDWORD);
    // the global case
    //#define DEFAULT_LEVEL	"CAM_2A"
    #[no_mangle]
    static mut pLevelName: [libc::c_char; 257];
    #[no_mangle]
    fn pie_SetVideoBuffer(width: UDWORD, height: UDWORD) -> BOOL;
    #[no_mangle]
    fn war_SetTranslucent(val: BOOL);
    #[no_mangle]
    fn war_SetAdditive(val: BOOL);
    #[no_mangle]
    fn war_SetSeqMode(mode: SEQ_MODE);
    #[no_mangle]
    fn war_SetPlayAudioCDs(b: BOOL);
    #[no_mangle]
    fn war_setFullscreen(_: BOOL);
    #[no_mangle]
    fn pie_SetFogCap(val: FOG_CAP);
    // true when interface is up and should be run.
    //the name of the save game to load from the front end
    #[no_mangle]
    static mut saveGameName: [STRING; 256];
    #[no_mangle]
    static mut gameSpy: GAMESPY;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // for Init.c
    #[no_mangle]
    fn lobbyInitialise() -> BOOL;
    #[no_mangle]
    static mut sPlayer: [libc::c_char; 128];
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    //put connections in Protocols[] (Lobbies optional)
    #[no_mangle]
    fn NETselectProtocol(lpConnection: LPVOID) -> BOOL;
    #[no_mangle]
    fn version() -> *const libc::c_char;
    /*
 * clParse.c
 *
 * Parse command line arguments
 *
 */
    // not for .net I should say..  --Qamly
    #[no_mangle]
    fn NETsetupTCPIP(addr: *mut LPVOID, machine: *mut libc::c_char) -> BOOL;
    #[no_mangle]
    static mut SaveGamePath: [libc::c_char; 0];
    #[no_mangle]
    static mut datadir: [libc::c_char; 260];
    #[no_mangle]
    static mut global_mods: [*mut libc::c_char; 100];
    #[no_mangle]
    static mut campaign_mods: [*mut libc::c_char; 100];
    #[no_mangle]
    static mut multiplay_mods: [*mut libc::c_char; 100];
    #[no_mangle]
    fn debug_callback_file(_: *mut *mut libc::c_void, _: *const libc::c_char);
    #[no_mangle]
    fn debug_callback_file_init(_: *mut *mut libc::c_void);
    #[no_mangle]
    fn debug_callback_file_exit(_: *mut *mut libc::c_void);
}
pub type __int64_t = libc::c_longlong;
pub type __off_t = libc::c_long;
pub type __off64_t = __int64_t;
pub type size_t = libc::c_uint;
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
/*
 * types.h
 *
 * Simple type definitions.
 *
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
// WIN32
/* Compilers that have support for C99 have all of the above defined in stdint.h */
// _MSC_VER
/* Basic numeric types */
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
pub type LPSTR = *mut libc::c_char;
pub type LPVOID = *mut libc::c_void;
/* !WIN32 */
pub type DPID = libc::c_int;
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
pub type SEQ_MODE = libc::c_uint;
pub const SEQ_SKIP: SEQ_MODE = 2;
pub const SEQ_SMALL: SEQ_MODE = 1;
pub const SEQ_FULL: SEQ_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERGAME {
    pub type_0: UBYTE,
    pub map: [STRING; 128],
    pub version: [libc::c_char; 8],
    pub maxPlayers: UBYTE,
    pub name: [STRING; 128],
    pub bComputerPlayers: BOOL,
    pub fog: BOOL,
    pub power: UDWORD,
    pub base: UBYTE,
    pub alliance: UBYTE,
    pub limit: UBYTE,
    pub bytesPerSec: UWORD,
    pub packetsPerSec: UBYTE,
    pub encryptKey: UBYTE,
    pub skDiff: [UBYTE; 8],
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
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
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
pub struct GAMESPY {
    pub bGameSpy: BOOL,
}
pub type FOG_CAP = libc::c_uint;
pub const FOG_CAP_UNDEFINED: FOG_CAP = 3;
pub const FOG_CAP_COLOURED: FOG_CAP = 2;
pub const FOG_CAP_GREY: FOG_CAP = 1;
pub const FOG_CAP_NO: FOG_CAP = 0;
// whether to play the intro video
#[no_mangle]
pub static mut clIntroVideo: BOOL = 0;
// let the end user into debug mode....
#[no_mangle]
pub static mut bAllowDebugMode: BOOL = 0 as libc::c_int;
/* *************************************************************************
	First half of the command line parsing. Also see ParseCommandLine()
	below. The parameters here are needed early in the boot process,
	while the ones in ParseCommandLine can benefit from debugging being
	set up first.
**************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ParseCommandLineEarly(mut argc: libc::c_int,
                                               mut argv:
                                                   *mut *mut libc::c_char)
 -> BOOL {
    let mut tokenType: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    /* loop through command line */
    i = 1 as libc::c_int;
    while i < argc {
        tokenType = *argv.offset(i as isize);
        if strcasecmp(tokenType,
                      b"--version\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            fprintf(stdout,
                    b"Warzone 2100 - Version %s - Built %s\n\x00" as *const u8
                        as *const libc::c_char, version(),
                    b"May 29 2020\x00" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        } else {
            if strcasecmp(tokenType,
                          b"--help\x00" as *const u8 as *const libc::c_char)
                   == 0 as libc::c_int {
                // Show help
                fprintf(stdout,
                        b"Warzone 2100:\n   An OpenGL based 3D real time strategy game, scened in post-nuclear warfare\nUsage:\n   warzone2100 [OPTIONS]\nOptions:\n   --cheat                    Run in cheat mode\n   --datadir DIR              Set default datadir to DIR\n   --debug FLAGS              Show debug for FLAGS\n   --debugfile FILE           Log debug output in FILE\n   --fullscreen               Play in fullscreen mode\n   --help                     Show this help and exit\n   --mod MOD                  Enable global mod MOD\n   --mod_ca MOD               Enable campaign only mod MOD\n   --mod_mp MOD               Enable multiplay only mod MOD\n   --savegame NAME            Load a saved game NAME\n   --window                   Play in windowed mode\n   --version                  Output version info and exit\n   --viewport WIDTHxHEIGHT    Set the dimensions of the viewport (screen or window)\n\x00"
                            as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            } else {
                if strcasecmp(tokenType,
                              b"--datadir\x00" as *const u8 as
                                  *const libc::c_char) == 0 as libc::c_int {
                    // find the quoted path name
                    i += 1;
                    token = *argv.offset(i as isize);
                    if token.is_null() {
                        debug(LOG_ERROR,
                              b"Unrecognised datadir\n\x00" as *const u8 as
                                  *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    strncpy(datadir.as_mut_ptr(), token,
                            ::std::mem::size_of::<[libc::c_char; 260]>() as
                                libc::c_ulong);
                } else if strcasecmp(tokenType,
                                     b"--debug\x00" as *const u8 as
                                         *const libc::c_char) ==
                              0 as libc::c_int {
                    // find the part name
                    i += 1;
                    token = *argv.offset(i as isize);
                    if token.is_null() {
                        debug(LOG_ERROR,
                              b"Usage: --debug <flag>\x00" as *const u8 as
                                  *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    if debug_enable_switch(token) == 0 {
                        debug(LOG_ERROR,
                              b"Debug flag \"%s\" not found!\x00" as *const u8
                                  as *const libc::c_char, token);
                        return 0 as libc::c_int
                    }
                } else if strcasecmp(tokenType,
                                     b"--debugfile\x00" as *const u8 as
                                         *const libc::c_char) ==
                              0 as libc::c_int {
                    // find the file name
                    i += 1;
                    token = *argv.offset(i as isize);
                    if token.is_null() {
                        debug(LOG_ERROR,
                              b"Missing filename?\n\x00" as *const u8 as
                                  *const libc::c_char);
                        abort();
                    }
                    debug_register_callback(Some(debug_callback_file as
                                                     unsafe extern "C" fn(_:
                                                                              *mut *mut libc::c_void,
                                                                          _:
                                                                              *const libc::c_char)
                                                         -> ()),
                                            Some(debug_callback_file_init as
                                                     unsafe extern "C" fn(_:
                                                                              *mut *mut libc::c_void)
                                                         -> ()),
                                            Some(debug_callback_file_exit as
                                                     unsafe extern "C" fn(_:
                                                                              *mut *mut libc::c_void)
                                                         -> ()),
                                            token as *mut libc::c_void);
                }
            }
        }
        i += 1
    }
    return 1 as libc::c_int;
}
/*
 * clParse.h
 *
 * All the command line values
 *
 */
// whether to play the intro video
// parse the commandline
/* *************************************************************************
	Second half of command line parsing. See ParseCommandLineEarly() for
	the first half. Note that render mode must come before resolution flag.
**************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ParseCommandLine(mut argc: libc::c_int,
                                          mut argv: *mut *mut libc::c_char)
 -> BOOL {
    let mut tokenType: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut height: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    /* loop through command line */
    i = 1 as libc::c_int;
    while i < argc {
        tokenType = *argv.offset(i as isize);
        if strcasecmp(tokenType,
                      b"--cheat\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            fprintf(stdout,
                    b"  ** CHEAT MODE ACTIVATED! **\n\x00" as *const u8 as
                        *const libc::c_char);
            bAllowDebugMode = 1 as libc::c_int
        } else if strcasecmp(tokenType,
                             b"--fullscreen\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_setFullscreen(1 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--game\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            // find the game name
            i += 1;
            token = *argv.offset(i as isize);
            if token.is_null() {
                debug(LOG_ERROR,
                      b"Unrecognised game name\n\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            strncpy(pLevelName.as_mut_ptr(), token,
                    254 as libc::c_int as libc::c_uint);
            SetGameMode(3 as libc::c_int as UDWORD);
        } else if strcasecmp(tokenType,
                             b"--mod\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            // find the file name
            i += 1;
            token = *argv.offset(i as isize);
            if token.is_null() {
                debug(LOG_ERROR,
                      b"Missing mod name?\n\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            j = 0 as libc::c_int;
            while !global_mods[j as usize].is_null() && j < 100 as libc::c_int
                  {
                j += 1
            }
            if !global_mods[j as usize].is_null() {
                debug(LOG_ERROR,
                      b"Too many mods registered! Aborting!\x00" as *const u8
                          as *const libc::c_char);
            }
            global_mods[j as usize] = token
        } else if strcasecmp(tokenType,
                             b"--mod_ca\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            // find the file name
            i += 1;
            token = *argv.offset(i as isize);
            if token.is_null() {
                debug(LOG_ERROR,
                      b"Missing mod name?\n\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            j = 0 as libc::c_int;
            while !campaign_mods[j as usize].is_null() &&
                      j < 100 as libc::c_int {
                j += 1
            }
            if !campaign_mods[j as usize].is_null() {
                debug(LOG_ERROR,
                      b"Too many mods registered! Aborting!\x00" as *const u8
                          as *const libc::c_char);
            }
            campaign_mods[j as usize] = token
        } else if strcasecmp(tokenType,
                             b"--mod_mp\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            // find the file name
            i += 1;
            token = *argv.offset(i as isize);
            if token.is_null() {
                debug(LOG_ERROR,
                      b"Missing mod name?\n\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            j = 0 as libc::c_int;
            while !multiplay_mods[j as usize].is_null() &&
                      j < 100 as libc::c_int {
                j += 1
            }
            if !multiplay_mods[j as usize].is_null() {
                debug(LOG_ERROR,
                      b"Too many mods registered! Aborting!\x00" as *const u8
                          as *const libc::c_char);
            }
            multiplay_mods[j as usize] = token
        } else if strcasecmp(tokenType,
                             b"--savegame\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            // find the game name
            i += 1;
            token = *argv.offset(i as isize);
            if token.is_null() {
                debug(LOG_ERROR,
                      b"Unrecognised savegame name\n\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            strcpy(saveGameName.as_mut_ptr(), SaveGamePath.as_mut_ptr());
            strcat(saveGameName.as_mut_ptr(),
                   b"/\x00" as *const u8 as *const libc::c_char);
            strncat(saveGameName.as_mut_ptr(), token,
                    240 as libc::c_int as libc::c_uint);
            SetGameMode(5 as libc::c_int as UDWORD);
        } else if strcasecmp(tokenType,
                             b"--viewport\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            i += 1;
            token = *argv.offset(i as isize);
            if (sscanf(token,
                       b"%ix%i\x00" as *const u8 as *const libc::c_char,
                       &mut width as *mut libc::c_uint,
                       &mut height as *mut libc::c_uint) == 0) as libc::c_int
                   == 2 as libc::c_int {
                debug(LOG_ERROR,
                      b"Invalid viewport\n\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            pie_SetVideoBuffer(width, height);
        } else if strcasecmp(tokenType,
                             b"--window\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_setFullscreen(0 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--intro\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            SetGameMode(4 as libc::c_int as UDWORD);
        } else if strcasecmp(tokenType,
                             b"--title\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            SetGameMode(1 as libc::c_int as UDWORD);
        } else if strcasecmp(tokenType,
                             b"--noTranslucent\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_SetTranslucent(0 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--noAdditive\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_SetAdditive(0 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--noFog\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            pie_SetFogCap(FOG_CAP_NO);
        } else if strcasecmp(tokenType,
                             b"--greyFog\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            pie_SetFogCap(FOG_CAP_GREY);
        } else if strcasecmp(tokenType,
                             b"--CDA\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int {
            war_SetPlayAudioCDs(1 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--noCDA\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_SetPlayAudioCDs(0 as libc::c_int);
        } else if strcasecmp(tokenType,
                             b"--seqSmall\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_SetSeqMode(SEQ_SMALL);
        } else if strcasecmp(tokenType,
                             b"--seqSkip\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            war_SetSeqMode(SEQ_SKIP);
        } else if strcasecmp(tokenType,
                             b"--disableLobby\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
            bDisableLobby = 1 as libc::c_int
        } else if strcasecmp(tokenType,
                             b"+host\x00" as *const u8 as *const libc::c_char)
                      == 0 as libc::c_int ||
                      strcasecmp(tokenType,
                                 b"+connect\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                      ||
                      strcasecmp(tokenType,
                                 b"+name\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                      ||
                      strcasecmp(tokenType,
                                 b"+ip\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                      ||
                      strcasecmp(tokenType,
                                 b"+maxplayers\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                      ||
                      strcasecmp(tokenType,
                                 b"+hostname\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
         {
            i += 1;
            token = *argv.offset(i as isize);
            scanGameSpyFlags(tokenType, token);
        }
        i += 1
    }
    return 1 as libc::c_int;
}
// gamespy flags
// ////////////////////////////////////////////////////////
// gamespy flags
#[no_mangle]
pub unsafe extern "C" fn scanGameSpyFlags(mut gflag: LPSTR, mut value: LPSTR)
 -> BOOL {
    static mut count: UBYTE = 0 as libc::c_int as UBYTE;
    //	UDWORD val;
    let mut finalconnection: LPVOID =
        0 as *mut libc::c_void; // dont reset everything on boot!
    count = count.wrapping_add(1);
    if count as libc::c_int == 1 as libc::c_int {
        lobbyInitialise();
        bDisableLobby = 1 as libc::c_int;
        gameSpy.bGameSpy = 1 as libc::c_int
    }
    if strcasecmp(gflag as *const libc::c_char,
                  b"+host\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        // host a multiplayer.
        NetPlay.bHost = 1 as libc::c_int;
        game.bytesPerSec = 1201 as libc::c_int as UWORD;
        game.packetsPerSec = 5 as libc::c_int as UBYTE;
        NETsetupTCPIP(&mut finalconnection,
                      b"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        NETselectProtocol(finalconnection);
    } else if strcasecmp(gflag as *const libc::c_char,
                         b"+connect\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        // join a multiplayer.
        NetPlay.bHost = 0 as libc::c_int;
        game.bytesPerSec = 1201 as libc::c_int as UWORD;
        game.packetsPerSec = 5 as libc::c_int as UBYTE;
        NETsetupTCPIP(&mut finalconnection, value);
        NETselectProtocol(finalconnection);
        // gflag is add to con to.
    } else if strcasecmp(gflag as *const libc::c_char,
                         b"+name\x00" as *const u8 as *const libc::c_char) ==
                  0 as libc::c_int {
        // player name.
        strcpy(sPlayer.as_mut_ptr(), value as *const libc::c_char);
    } else if strcasecmp(gflag as *const libc::c_char,
                         b"+hostname\x00" as *const u8 as *const libc::c_char)
                  == 0 as libc::c_int {
        // game name.
        strcpy(game.name.as_mut_ptr(), value as *const libc::c_char);
    }
    /*not used from here on..
+ip
+maxplayers
+game
+team
+skin
+playerid
+password
tokenType = strtok( NULL, seps );
*/
    return 1 as libc::c_int;
}
