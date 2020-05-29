use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type WMcursor;
    pub type SDL_SysWMmsg;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn SDL_GetError() -> *mut libc::c_char;
    #[no_mangle]
    fn SDL_Quit();
    #[no_mangle]
    fn SDL_Init(flags: Uint32) -> libc::c_int;
    #[no_mangle]
    fn SDL_GetTicks() -> Uint32;
    #[no_mangle]
    fn SDL_CreateCursor(data: *mut Uint8, mask: *mut Uint8, w: libc::c_int,
                        h: libc::c_int, hot_x: libc::c_int,
                        hot_y: libc::c_int) -> *mut SDL_Cursor;
    #[no_mangle]
    fn SDL_SetCursor(cursor: *mut SDL_Cursor);
    #[no_mangle]
    fn SDL_FreeCursor(cursor: *mut SDL_Cursor);
    #[no_mangle]
    fn SDL_PollEvent(event: *mut SDL_Event) -> libc::c_int;
    #[no_mangle]
    fn SDL_WM_SetCaption(title: *const libc::c_char,
                         icon: *const libc::c_char);
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_getDirSeparator() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_getRealDir(filename: *const libc::c_char)
     -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_fileLength(handle: *mut PHYSFS_File) -> PHYSFS_sint64;
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
    #[no_mangle]
    fn PHYSFSEXT_locateCorrectCase(buf: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn inputInitialise();
    #[no_mangle]
    fn inputProcessEvent(event: *mut SDL_Event);
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memShutDown();
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
 * Trig.h
 *
 * Interface to trig lookup tables
 *
 */
    /* The number of units around a full circle */
    /* conversion macros */
    /* Initialise the Trig tables */
    #[no_mangle]
    fn trigInitialise() -> BOOL;
    /* Shutdown the trig tables */
    #[no_mangle]
    fn trigShutDown();
    /* Initialise the resource module */
    #[no_mangle]
    fn resInitialise() -> BOOL;
    /* Shutdown the resource module */
    #[no_mangle]
    fn resShutDown();
    // shutdown the block system
    #[no_mangle]
    fn blkShutDown();
    /*
 * FrameInt.h
 *
 * Internal definitions for the framework library.
 *
 */
    /* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
    /* Define the style and extended style of the window.
 * Need these to calculate the size the window should be when returning to
 * window mode.
 *
 * create a title bar, minimise button on the title bar,
 * automatic ShowWindow, get standard system menu on title bar
 */
    // Go on task bar when iconified
    /* Program hInstance */
    /* Handle for the main window */
    /* Initialise the double buffered display */
    #[no_mangle]
    fn screenInitialise(width: UDWORD, height: UDWORD, bitDepth: UDWORD,
                        fullScreen: BOOL, bVidMem: BOOL, bDDraw: BOOL,
                        hWindow: HANDLE) -> BOOL;
    // The main windows handle
    /* Release the DD objects */
    #[no_mangle]
    fn screenShutDown();
    /* This is called once a frame so that the system can tell
 * whether a key was pressed this turn or held down from the last frame.
 */
    #[no_mangle]
    fn inputNewFrame();
    /* Functions return 0 or value for sucess and -1 for error */
    #[no_mangle]
    fn SDL_initFramerate(manager: *mut FPSmanager);
    #[no_mangle]
    fn SDL_setFramerate(manager: *mut FPSmanager, rate: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn SDL_framerateDelay(manager: *mut FPSmanager);
}
pub type size_t = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __off_t = libc::c_long;
pub type __off64_t = __int64_t;
pub type __clock_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type Uint8 = uint8_t;
pub type Sint16 = int16_t;
pub type Uint16 = uint16_t;
pub type Uint32 = uint32_t;
pub type SDLKey = libc::c_uint;
pub const SDLK_LAST: SDLKey = 323;
pub const SDLK_UNDO: SDLKey = 322;
pub const SDLK_EURO: SDLKey = 321;
pub const SDLK_POWER: SDLKey = 320;
pub const SDLK_MENU: SDLKey = 319;
pub const SDLK_BREAK: SDLKey = 318;
pub const SDLK_SYSREQ: SDLKey = 317;
pub const SDLK_PRINT: SDLKey = 316;
pub const SDLK_HELP: SDLKey = 315;
pub const SDLK_COMPOSE: SDLKey = 314;
pub const SDLK_MODE: SDLKey = 313;
pub const SDLK_RSUPER: SDLKey = 312;
pub const SDLK_LSUPER: SDLKey = 311;
pub const SDLK_LMETA: SDLKey = 310;
pub const SDLK_RMETA: SDLKey = 309;
pub const SDLK_LALT: SDLKey = 308;
pub const SDLK_RALT: SDLKey = 307;
pub const SDLK_LCTRL: SDLKey = 306;
pub const SDLK_RCTRL: SDLKey = 305;
pub const SDLK_LSHIFT: SDLKey = 304;
pub const SDLK_RSHIFT: SDLKey = 303;
pub const SDLK_SCROLLOCK: SDLKey = 302;
pub const SDLK_CAPSLOCK: SDLKey = 301;
pub const SDLK_NUMLOCK: SDLKey = 300;
pub const SDLK_F15: SDLKey = 296;
pub const SDLK_F14: SDLKey = 295;
pub const SDLK_F13: SDLKey = 294;
pub const SDLK_F12: SDLKey = 293;
pub const SDLK_F11: SDLKey = 292;
pub const SDLK_F10: SDLKey = 291;
pub const SDLK_F9: SDLKey = 290;
pub const SDLK_F8: SDLKey = 289;
pub const SDLK_F7: SDLKey = 288;
pub const SDLK_F6: SDLKey = 287;
pub const SDLK_F5: SDLKey = 286;
pub const SDLK_F4: SDLKey = 285;
pub const SDLK_F3: SDLKey = 284;
pub const SDLK_F2: SDLKey = 283;
pub const SDLK_F1: SDLKey = 282;
pub const SDLK_PAGEDOWN: SDLKey = 281;
pub const SDLK_PAGEUP: SDLKey = 280;
pub const SDLK_END: SDLKey = 279;
pub const SDLK_HOME: SDLKey = 278;
pub const SDLK_INSERT: SDLKey = 277;
pub const SDLK_LEFT: SDLKey = 276;
pub const SDLK_RIGHT: SDLKey = 275;
pub const SDLK_DOWN: SDLKey = 274;
pub const SDLK_UP: SDLKey = 273;
pub const SDLK_KP_EQUALS: SDLKey = 272;
pub const SDLK_KP_ENTER: SDLKey = 271;
pub const SDLK_KP_PLUS: SDLKey = 270;
pub const SDLK_KP_MINUS: SDLKey = 269;
pub const SDLK_KP_MULTIPLY: SDLKey = 268;
pub const SDLK_KP_DIVIDE: SDLKey = 267;
pub const SDLK_KP_PERIOD: SDLKey = 266;
pub const SDLK_KP9: SDLKey = 265;
pub const SDLK_KP8: SDLKey = 264;
pub const SDLK_KP7: SDLKey = 263;
pub const SDLK_KP6: SDLKey = 262;
pub const SDLK_KP5: SDLKey = 261;
pub const SDLK_KP4: SDLKey = 260;
pub const SDLK_KP3: SDLKey = 259;
pub const SDLK_KP2: SDLKey = 258;
pub const SDLK_KP1: SDLKey = 257;
pub const SDLK_KP0: SDLKey = 256;
pub const SDLK_WORLD_95: SDLKey = 255;
pub const SDLK_WORLD_94: SDLKey = 254;
pub const SDLK_WORLD_93: SDLKey = 253;
pub const SDLK_WORLD_92: SDLKey = 252;
pub const SDLK_WORLD_91: SDLKey = 251;
pub const SDLK_WORLD_90: SDLKey = 250;
pub const SDLK_WORLD_89: SDLKey = 249;
pub const SDLK_WORLD_88: SDLKey = 248;
pub const SDLK_WORLD_87: SDLKey = 247;
pub const SDLK_WORLD_86: SDLKey = 246;
pub const SDLK_WORLD_85: SDLKey = 245;
pub const SDLK_WORLD_84: SDLKey = 244;
pub const SDLK_WORLD_83: SDLKey = 243;
pub const SDLK_WORLD_82: SDLKey = 242;
pub const SDLK_WORLD_81: SDLKey = 241;
pub const SDLK_WORLD_80: SDLKey = 240;
pub const SDLK_WORLD_79: SDLKey = 239;
pub const SDLK_WORLD_78: SDLKey = 238;
pub const SDLK_WORLD_77: SDLKey = 237;
pub const SDLK_WORLD_76: SDLKey = 236;
pub const SDLK_WORLD_75: SDLKey = 235;
pub const SDLK_WORLD_74: SDLKey = 234;
pub const SDLK_WORLD_73: SDLKey = 233;
pub const SDLK_WORLD_72: SDLKey = 232;
pub const SDLK_WORLD_71: SDLKey = 231;
pub const SDLK_WORLD_70: SDLKey = 230;
pub const SDLK_WORLD_69: SDLKey = 229;
pub const SDLK_WORLD_68: SDLKey = 228;
pub const SDLK_WORLD_67: SDLKey = 227;
pub const SDLK_WORLD_66: SDLKey = 226;
pub const SDLK_WORLD_65: SDLKey = 225;
pub const SDLK_WORLD_64: SDLKey = 224;
pub const SDLK_WORLD_63: SDLKey = 223;
pub const SDLK_WORLD_62: SDLKey = 222;
pub const SDLK_WORLD_61: SDLKey = 221;
pub const SDLK_WORLD_60: SDLKey = 220;
pub const SDLK_WORLD_59: SDLKey = 219;
pub const SDLK_WORLD_58: SDLKey = 218;
pub const SDLK_WORLD_57: SDLKey = 217;
pub const SDLK_WORLD_56: SDLKey = 216;
pub const SDLK_WORLD_55: SDLKey = 215;
pub const SDLK_WORLD_54: SDLKey = 214;
pub const SDLK_WORLD_53: SDLKey = 213;
pub const SDLK_WORLD_52: SDLKey = 212;
pub const SDLK_WORLD_51: SDLKey = 211;
pub const SDLK_WORLD_50: SDLKey = 210;
pub const SDLK_WORLD_49: SDLKey = 209;
pub const SDLK_WORLD_48: SDLKey = 208;
pub const SDLK_WORLD_47: SDLKey = 207;
pub const SDLK_WORLD_46: SDLKey = 206;
pub const SDLK_WORLD_45: SDLKey = 205;
pub const SDLK_WORLD_44: SDLKey = 204;
pub const SDLK_WORLD_43: SDLKey = 203;
pub const SDLK_WORLD_42: SDLKey = 202;
pub const SDLK_WORLD_41: SDLKey = 201;
pub const SDLK_WORLD_40: SDLKey = 200;
pub const SDLK_WORLD_39: SDLKey = 199;
pub const SDLK_WORLD_38: SDLKey = 198;
pub const SDLK_WORLD_37: SDLKey = 197;
pub const SDLK_WORLD_36: SDLKey = 196;
pub const SDLK_WORLD_35: SDLKey = 195;
pub const SDLK_WORLD_34: SDLKey = 194;
pub const SDLK_WORLD_33: SDLKey = 193;
pub const SDLK_WORLD_32: SDLKey = 192;
pub const SDLK_WORLD_31: SDLKey = 191;
pub const SDLK_WORLD_30: SDLKey = 190;
pub const SDLK_WORLD_29: SDLKey = 189;
pub const SDLK_WORLD_28: SDLKey = 188;
pub const SDLK_WORLD_27: SDLKey = 187;
pub const SDLK_WORLD_26: SDLKey = 186;
pub const SDLK_WORLD_25: SDLKey = 185;
pub const SDLK_WORLD_24: SDLKey = 184;
pub const SDLK_WORLD_23: SDLKey = 183;
pub const SDLK_WORLD_22: SDLKey = 182;
pub const SDLK_WORLD_21: SDLKey = 181;
pub const SDLK_WORLD_20: SDLKey = 180;
pub const SDLK_WORLD_19: SDLKey = 179;
pub const SDLK_WORLD_18: SDLKey = 178;
pub const SDLK_WORLD_17: SDLKey = 177;
pub const SDLK_WORLD_16: SDLKey = 176;
pub const SDLK_WORLD_15: SDLKey = 175;
pub const SDLK_WORLD_14: SDLKey = 174;
pub const SDLK_WORLD_13: SDLKey = 173;
pub const SDLK_WORLD_12: SDLKey = 172;
pub const SDLK_WORLD_11: SDLKey = 171;
pub const SDLK_WORLD_10: SDLKey = 170;
pub const SDLK_WORLD_9: SDLKey = 169;
pub const SDLK_WORLD_8: SDLKey = 168;
pub const SDLK_WORLD_7: SDLKey = 167;
pub const SDLK_WORLD_6: SDLKey = 166;
pub const SDLK_WORLD_5: SDLKey = 165;
pub const SDLK_WORLD_4: SDLKey = 164;
pub const SDLK_WORLD_3: SDLKey = 163;
pub const SDLK_WORLD_2: SDLKey = 162;
pub const SDLK_WORLD_1: SDLKey = 161;
pub const SDLK_WORLD_0: SDLKey = 160;
pub const SDLK_DELETE: SDLKey = 127;
pub const SDLK_z: SDLKey = 122;
pub const SDLK_y: SDLKey = 121;
pub const SDLK_x: SDLKey = 120;
pub const SDLK_w: SDLKey = 119;
pub const SDLK_v: SDLKey = 118;
pub const SDLK_u: SDLKey = 117;
pub const SDLK_t: SDLKey = 116;
pub const SDLK_s: SDLKey = 115;
pub const SDLK_r: SDLKey = 114;
pub const SDLK_q: SDLKey = 113;
pub const SDLK_p: SDLKey = 112;
pub const SDLK_o: SDLKey = 111;
pub const SDLK_n: SDLKey = 110;
pub const SDLK_m: SDLKey = 109;
pub const SDLK_l: SDLKey = 108;
pub const SDLK_k: SDLKey = 107;
pub const SDLK_j: SDLKey = 106;
pub const SDLK_i: SDLKey = 105;
pub const SDLK_h: SDLKey = 104;
pub const SDLK_g: SDLKey = 103;
pub const SDLK_f: SDLKey = 102;
pub const SDLK_e: SDLKey = 101;
pub const SDLK_d: SDLKey = 100;
pub const SDLK_c: SDLKey = 99;
pub const SDLK_b: SDLKey = 98;
pub const SDLK_a: SDLKey = 97;
pub const SDLK_BACKQUOTE: SDLKey = 96;
pub const SDLK_UNDERSCORE: SDLKey = 95;
pub const SDLK_CARET: SDLKey = 94;
pub const SDLK_RIGHTBRACKET: SDLKey = 93;
pub const SDLK_BACKSLASH: SDLKey = 92;
pub const SDLK_LEFTBRACKET: SDLKey = 91;
pub const SDLK_AT: SDLKey = 64;
pub const SDLK_QUESTION: SDLKey = 63;
pub const SDLK_GREATER: SDLKey = 62;
pub const SDLK_EQUALS: SDLKey = 61;
pub const SDLK_LESS: SDLKey = 60;
pub const SDLK_SEMICOLON: SDLKey = 59;
pub const SDLK_COLON: SDLKey = 58;
pub const SDLK_9: SDLKey = 57;
pub const SDLK_8: SDLKey = 56;
pub const SDLK_7: SDLKey = 55;
pub const SDLK_6: SDLKey = 54;
pub const SDLK_5: SDLKey = 53;
pub const SDLK_4: SDLKey = 52;
pub const SDLK_3: SDLKey = 51;
pub const SDLK_2: SDLKey = 50;
pub const SDLK_1: SDLKey = 49;
pub const SDLK_0: SDLKey = 48;
pub const SDLK_SLASH: SDLKey = 47;
pub const SDLK_PERIOD: SDLKey = 46;
pub const SDLK_MINUS: SDLKey = 45;
pub const SDLK_COMMA: SDLKey = 44;
pub const SDLK_PLUS: SDLKey = 43;
pub const SDLK_ASTERISK: SDLKey = 42;
pub const SDLK_RIGHTPAREN: SDLKey = 41;
pub const SDLK_LEFTPAREN: SDLKey = 40;
pub const SDLK_QUOTE: SDLKey = 39;
pub const SDLK_AMPERSAND: SDLKey = 38;
pub const SDLK_DOLLAR: SDLKey = 36;
pub const SDLK_HASH: SDLKey = 35;
pub const SDLK_QUOTEDBL: SDLKey = 34;
pub const SDLK_EXCLAIM: SDLKey = 33;
pub const SDLK_SPACE: SDLKey = 32;
pub const SDLK_ESCAPE: SDLKey = 27;
pub const SDLK_PAUSE: SDLKey = 19;
pub const SDLK_RETURN: SDLKey = 13;
pub const SDLK_CLEAR: SDLKey = 12;
pub const SDLK_TAB: SDLKey = 9;
pub const SDLK_BACKSPACE: SDLKey = 8;
pub const SDLK_FIRST: SDLKey = 0;
pub const SDLK_UNKNOWN: SDLKey = 0;
pub type SDLMod = libc::c_uint;
pub const KMOD_RESERVED: SDLMod = 32768;
pub const KMOD_MODE: SDLMod = 16384;
pub const KMOD_CAPS: SDLMod = 8192;
pub const KMOD_NUM: SDLMod = 4096;
pub const KMOD_RMETA: SDLMod = 2048;
pub const KMOD_LMETA: SDLMod = 1024;
pub const KMOD_RALT: SDLMod = 512;
pub const KMOD_LALT: SDLMod = 256;
pub const KMOD_RCTRL: SDLMod = 128;
pub const KMOD_LCTRL: SDLMod = 64;
pub const KMOD_RSHIFT: SDLMod = 2;
pub const KMOD_LSHIFT: SDLMod = 1;
pub const KMOD_NONE: SDLMod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_keysym {
    pub scancode: Uint8,
    pub sym: SDLKey,
    pub mod_0: SDLMod,
    pub unicode: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Rect {
    pub x: Sint16,
    pub y: Sint16,
    pub w: Uint16,
    pub h: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_Cursor {
    pub area: SDL_Rect,
    pub hot_x: Sint16,
    pub hot_y: Sint16,
    pub data: *mut Uint8,
    pub mask: *mut Uint8,
    pub save: [*mut Uint8; 2],
    pub wm_cursor: *mut WMcursor,
}
pub type C2RustUnnamed = libc::c_uint;
pub const SDL_NUMEVENTS: C2RustUnnamed = 32;
pub const SDL_USEREVENT: C2RustUnnamed = 24;
pub const SDL_EVENT_RESERVED7: C2RustUnnamed = 23;
pub const SDL_EVENT_RESERVED6: C2RustUnnamed = 22;
pub const SDL_EVENT_RESERVED5: C2RustUnnamed = 21;
pub const SDL_EVENT_RESERVED4: C2RustUnnamed = 20;
pub const SDL_EVENT_RESERVED3: C2RustUnnamed = 19;
pub const SDL_EVENT_RESERVED2: C2RustUnnamed = 18;
pub const SDL_VIDEOEXPOSE: C2RustUnnamed = 17;
pub const SDL_VIDEORESIZE: C2RustUnnamed = 16;
pub const SDL_EVENT_RESERVEDB: C2RustUnnamed = 15;
pub const SDL_EVENT_RESERVEDA: C2RustUnnamed = 14;
pub const SDL_SYSWMEVENT: C2RustUnnamed = 13;
pub const SDL_QUIT: C2RustUnnamed = 12;
pub const SDL_JOYBUTTONUP: C2RustUnnamed = 11;
pub const SDL_JOYBUTTONDOWN: C2RustUnnamed = 10;
pub const SDL_JOYHATMOTION: C2RustUnnamed = 9;
pub const SDL_JOYBALLMOTION: C2RustUnnamed = 8;
pub const SDL_JOYAXISMOTION: C2RustUnnamed = 7;
pub const SDL_MOUSEBUTTONUP: C2RustUnnamed = 6;
pub const SDL_MOUSEBUTTONDOWN: C2RustUnnamed = 5;
pub const SDL_MOUSEMOTION: C2RustUnnamed = 4;
pub const SDL_KEYUP: C2RustUnnamed = 3;
pub const SDL_KEYDOWN: C2RustUnnamed = 2;
pub const SDL_ACTIVEEVENT: C2RustUnnamed = 1;
pub const SDL_NOEVENT: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ActiveEvent {
    pub type_0: Uint8,
    pub gain: Uint8,
    pub state: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub keysym: SDL_keysym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
    pub x: Uint16,
    pub y: Uint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub axis: Uint8,
    pub value: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub ball: Uint8,
    pub xrel: Sint16,
    pub yrel: Sint16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub hat: Uint8,
    pub value: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
    pub type_0: Uint8,
    pub which: Uint8,
    pub button: Uint8,
    pub state: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ResizeEvent {
    pub type_0: Uint8,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_ExposeEvent {
    pub type_0: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_QuitEvent {
    pub type_0: Uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_UserEvent {
    pub type_0: Uint8,
    pub code: libc::c_int,
    pub data1: *mut libc::c_void,
    pub data2: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDL_SysWMEvent {
    pub type_0: Uint8,
    pub msg: *mut SDL_SysWMmsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SDL_Event {
    pub type_0: Uint8,
    pub active: SDL_ActiveEvent,
    pub key: SDL_KeyboardEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub jaxis: SDL_JoyAxisEvent,
    pub jball: SDL_JoyBallEvent,
    pub jhat: SDL_JoyHatEvent,
    pub jbutton: SDL_JoyButtonEvent,
    pub resize: SDL_ResizeEvent,
    pub expose: SDL_ExposeEvent,
    pub quit: SDL_QuitEvent,
    pub user: SDL_UserEvent,
    pub syswm: SDL_SysWMEvent,
}
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type HANDLE = *mut libc::c_void;
pub type HINSTANCE = libc::c_int;
pub type HCURSOR = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type WORD = libc::c_short;
pub type UINT = libc::c_uint;
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
pub struct FPSmanager {
    pub framecount: Uint32,
    pub rateticks: libc::c_float,
    pub lastticks: Uint32,
    pub rate: Uint32,
}
pub type FOCUS_STATE = _focus_state;
pub type _focus_state = libc::c_uint;
pub const FOCUS_KILL: _focus_state = 3;
pub const FOCUS_IN: _focus_state = 2;
pub const FOCUS_SET: _focus_state = 1;
pub const FOCUS_OUT: _focus_state = 0;
pub type _frame_status = libc::c_uint;
pub const FRAME_QUIT: _frame_status = 3;
pub const FRAME_SETFOCUS: _frame_status = 2;
pub const FRAME_KILLFOCUS: _frame_status = 1;
pub const FRAME_OK: _frame_status = 0;
pub type FRAME_STATUS = _frame_status;
// Window does not have the focus
// Just received WM_SETFOCUS
// Window has got the focus
// Just received WM_KILLFOCUS
/* TODO: do bridge and attach need swapping? */
static mut cursor_arrow: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"X                               \x00" as *const u8 as
         *const libc::c_char,
     b"XX                              \x00" as *const u8 as
         *const libc::c_char,
     b"X.X                             \x00" as *const u8 as
         *const libc::c_char,
     b"X..X                            \x00" as *const u8 as
         *const libc::c_char,
     b"X...X                           \x00" as *const u8 as
         *const libc::c_char,
     b"X....X                          \x00" as *const u8 as
         *const libc::c_char,
     b"X.....X                         \x00" as *const u8 as
         *const libc::c_char,
     b"X......X                        \x00" as *const u8 as
         *const libc::c_char,
     b"X.......X                       \x00" as *const u8 as
         *const libc::c_char,
     b"X........X                      \x00" as *const u8 as
         *const libc::c_char,
     b"X.....XXXXX                     \x00" as *const u8 as
         *const libc::c_char,
     b"X..X..X                         \x00" as *const u8 as
         *const libc::c_char,
     b"X.X X..X                        \x00" as *const u8 as
         *const libc::c_char,
     b"XX  X..X                        \x00" as *const u8 as
         *const libc::c_char,
     b"X    X..X                       \x00" as *const u8 as
         *const libc::c_char,
     b"     X..X                       \x00" as *const u8 as
         *const libc::c_char,
     b"      X..X                      \x00" as *const u8 as
         *const libc::c_char,
     b"      X..X                      \x00" as *const u8 as
         *const libc::c_char,
     b"       XX                       \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char, b"0,0\x00" as *const u8 as *const libc::c_char];
static mut cursor_dest: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"           ..                   \x00" as *const u8 as
         *const libc::c_char,
     b"           ...X                 \x00" as *const u8 as
         *const libc::c_char,
     b"           ..X.X                \x00" as *const u8 as
         *const libc::c_char,
     b"           ..X .X               \x00" as *const u8 as
         *const libc::c_char,
     b"           .X.X .X              \x00" as *const u8 as
         *const libc::c_char,
     b"           .X.X  .X             \x00" as *const u8 as
         *const libc::c_char,
     b"           .X .X  .X            \x00" as *const u8 as
         *const libc::c_char,
     b"           .X .X   .......      \x00" as *const u8 as
         *const libc::c_char,
     b"           .X  .X  .XX....X     \x00" as *const u8 as
         *const libc::c_char,
     b"           .X  .X .XXX....X     \x00" as *const u8 as
         *const libc::c_char,
     b"           .X   ..XXXX....X     \x00" as *const u8 as
         *const libc::c_char,
     b"          ...X  ..XXXX....X     \x00" as *const u8 as
         *const libc::c_char,
     b"         .....X ..........X     \x00" as *const u8 as
         *const libc::c_char,
     b"         .....X ..........X     \x00" as *const u8 as
         *const libc::c_char,
     b"         .....X .XX.XX.XX..     \x00" as *const u8 as
         *const libc::c_char,
     b"          ...X .X..X..X..X..    \x00" as *const u8 as
         *const libc::c_char,
     b"           XX  .X..X..X..XX.X   \x00" as *const u8 as
         *const libc::c_char,
     b"               X...........XX   \x00" as *const u8 as
         *const libc::c_char,
     b"                XXXXXXXXXXXX    \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,18\x00" as *const u8 as *const libc::c_char];
static mut cursor_sight: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"            .......             \x00" as *const u8 as
         *const libc::c_char,
     b"   ..     ...........     ..    \x00" as *const u8 as
         *const libc::c_char,
     b"   ....  X..XXXXXXX..X  ....X   \x00" as *const u8 as
         *const libc::c_char,
     b"    .....XXXX      XXX.....XX   \x00" as *const u8 as
         *const libc::c_char,
     b"    .......X        .......X    \x00" as *const u8 as
         *const libc::c_char,
     b"     ........     ........XX    \x00" as *const u8 as
         *const libc::c_char,
     b"     .......XX     .......X     \x00" as *const u8 as
         *const libc::c_char,
     b"    XX.....XX       .....XXX    \x00" as *const u8 as
         *const libc::c_char,
     b"   ..X....XX         ....X..X   \x00" as *const u8 as
         *const libc::c_char,
     b"   ..XX..XX           ..XX..X   \x00" as *const u8 as
         *const libc::c_char,
     b"  ...X .XX             .X ...X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..XX  X               X  ..X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..X                      ..X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..X                      ..X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..X                      ..X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..X                      ..X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ...  .               .X ...X  \x00" as *const u8 as
         *const libc::c_char,
     b"   ..X ..             ..X ..XX  \x00" as *const u8 as
         *const libc::c_char,
     b"   ..X....           ....X..X   \x00" as *const u8 as
         *const libc::c_char,
     b"    XX.....         .....XXXX   \x00" as *const u8 as
         *const libc::c_char,
     b"     .......       .......XX    \x00" as *const u8 as
         *const libc::c_char,
     b"     ........     ........XX    \x00" as *const u8 as
         *const libc::c_char,
     b"    .......XXX    XX.......X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .....XXX        XX.....X    \x00" as *const u8 as
         *const libc::c_char,
     b"   ....XXX...     ...XXX....    \x00" as *const u8 as
         *const libc::c_char,
     b"   ..XXX X...........XXX X..X   \x00" as *const u8 as
         *const libc::c_char,
     b"    XX    XX.......XXX     XX   \x00" as *const u8 as
         *const libc::c_char,
     b"            XXXXXXXX            \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,16\x00" as *const u8 as *const libc::c_char];
static mut cursor_target: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"               .                \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"            ....X..             \x00" as *const u8 as
         *const libc::c_char,
     b"          ......X....           \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XXX.XXX...          \x00" as *const u8 as
         *const libc::c_char,
     b"        ..XX   .X   X..         \x00" as *const u8 as
         *const libc::c_char,
     b"       ..X     .X     ..        \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X      XX      ..       \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X              ..X      \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"   ........X   .    ........X   \x00" as *const u8 as
         *const libc::c_char,
     b"    XXXXXXXX   X     XXXXXXXX   \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ..                .X      \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X      .       ..X      \x00" as *const u8 as
         *const libc::c_char,
     b"       ..      .X     ..X       \x00" as *const u8 as
         *const libc::c_char,
     b"        ..     .X    ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"         ...   .X   ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"         X......X....X          \x00" as *const u8 as
         *const libc::c_char,
     b"          XX....X..XX           \x00" as *const u8 as
         *const libc::c_char,
     b"            XXX.XXX             \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"               XX               \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_larrow: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                ..              \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"            ....X               \x00" as *const u8 as
         *const libc::c_char,
     b"          ...............       \x00" as *const u8 as
         *const libc::c_char,
     b"        .................X      \x00" as *const u8 as
         *const libc::c_char,
     b"      ...................X      \x00" as *const u8 as
         *const libc::c_char,
     b"    .....................X      \x00" as *const u8 as
         *const libc::c_char,
     b"    XX...................X      \x00" as *const u8 as
         *const libc::c_char,
     b"      XX.................X      \x00" as *const u8 as
         *const libc::c_char,
     b"        XX...............X      \x00" as *const u8 as
         *const libc::c_char,
     b"          XX....XXXXXXXXXX      \x00" as *const u8 as
         *const libc::c_char,
     b"            XX...X              \x00" as *const u8 as
         *const libc::c_char,
     b"              XX..X             \x00" as *const u8 as
         *const libc::c_char,
     b"                XXX             \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"7,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_rarrow: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"              ..                \x00" as *const u8 as
         *const libc::c_char,
     b"               ...              \x00" as *const u8 as
         *const libc::c_char,
     b"                ....            \x00" as *const u8 as
         *const libc::c_char,
     b"       ...............          \x00" as *const u8 as
         *const libc::c_char,
     b"       .................        \x00" as *const u8 as
         *const libc::c_char,
     b"       ...................      \x00" as *const u8 as
         *const libc::c_char,
     b"       .....................X   \x00" as *const u8 as
         *const libc::c_char,
     b"       ...................XX    \x00" as *const u8 as
         *const libc::c_char,
     b"       .................XX      \x00" as *const u8 as
         *const libc::c_char,
     b"       ...............XX        \x00" as *const u8 as
         *const libc::c_char,
     b"        XXXXXXXX....XX          \x00" as *const u8 as
         *const libc::c_char,
     b"              X...XX            \x00" as *const u8 as
         *const libc::c_char,
     b"              ..XX              \x00" as *const u8 as
         *const libc::c_char,
     b"              XX                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"25,16\x00" as *const u8 as *const libc::c_char];
static mut cursor_darrow: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"            .......             \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X XX         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X .......XX.X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X.......X..X         \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,24\x00" as *const u8 as *const libc::c_char];
static mut cursor_uarrow: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"               .                \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X.......X..X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X .......XX.X         \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X XX         \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"             XXXXXXX            \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,8\x00" as *const u8 as *const libc::c_char];
static mut cursor_default: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"            ..XX                \x00" as *const u8 as
         *const libc::c_char,
     b"            ....XX              \x00" as *const u8 as
         *const libc::c_char,
     b"             .....XX            \x00" as *const u8 as
         *const libc::c_char,
     b"             .......XX          \x00" as *const u8 as
         *const libc::c_char,
     b"              ........XX        \x00" as *const u8 as
         *const libc::c_char,
     b"              ..........XX      \x00" as *const u8 as
         *const libc::c_char,
     b"               ...........X     \x00" as *const u8 as
         *const libc::c_char,
     b"               ....XXXXXXXX     \x00" as *const u8 as
         *const libc::c_char,
     b"                ...X            \x00" as *const u8 as
         *const libc::c_char,
     b"                ...X            \x00" as *const u8 as
         *const libc::c_char,
     b"                 ..X            \x00" as *const u8 as
         *const libc::c_char,
     b"                 ..X            \x00" as *const u8 as
         *const libc::c_char,
     b"                  .X            \x00" as *const u8 as
         *const libc::c_char,
     b"                  .X            \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"12,12\x00" as *const u8 as *const libc::c_char];
static mut cursor_attach: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"     ...X  ....X  ....X  ...    \x00" as *const u8 as
         *const libc::c_char,
     b"     XXX.X.XXXX.X.XXXX.X.XXX    \x00" as *const u8 as
         *const libc::c_char,
     b"       .....  .....  .....      \x00" as *const u8 as
         *const libc::c_char,
     b"        .X.X   .X.X   .X.X      \x00" as *const u8 as
         *const libc::c_char,
     b"     ...X  ....X  ....X  ...    \x00" as *const u8 as
         *const libc::c_char,
     b"     XXX   XXXX   XXXX   XXX    \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"17,14\x00" as *const u8 as *const libc::c_char];
static mut cursor_attack: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"               .                \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"            ....X..             \x00" as *const u8 as
         *const libc::c_char,
     b"          ......X....           \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XXX.XXX...          \x00" as *const u8 as
         *const libc::c_char,
     b"        ..XX   .X   X..         \x00" as *const u8 as
         *const libc::c_char,
     b"       ..X     .X     ..        \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X      XX      ..       \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X              ..X      \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"   ........X   .    ........X   \x00" as *const u8 as
         *const libc::c_char,
     b"    XXXXXXXX   X     XXXXXXXX   \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"     ..X                ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ..                .X      \x00" as *const u8 as
         *const libc::c_char,
     b"      ..X      .       ..X      \x00" as *const u8 as
         *const libc::c_char,
     b"       ..      .X     ..X       \x00" as *const u8 as
         *const libc::c_char,
     b"        ..     .X    ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"         ...   .X   ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"         X......X....X          \x00" as *const u8 as
         *const libc::c_char,
     b"          XX....X..XX           \x00" as *const u8 as
         *const libc::c_char,
     b"            XXX.XXX             \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"               XX               \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_bomb: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                   . .          \x00" as *const u8 as
         *const libc::c_char,
     b"                  . . .         \x00" as *const u8 as
         *const libc::c_char,
     b"                  XXX.          \x00" as *const u8 as
         *const libc::c_char,
     b"               XXXX . .         \x00" as *const u8 as
         *const libc::c_char,
     b"               XX  . .          \x00" as *const u8 as
         *const libc::c_char,
     b"               XX               \x00" as *const u8 as
         *const libc::c_char,
     b"              ....X             \x00" as *const u8 as
         *const libc::c_char,
     b"              ....X             \x00" as *const u8 as
         *const libc::c_char,
     b"            .XXXXXX.XX          \x00" as *const u8 as
         *const libc::c_char,
     b"          ............X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ..............X        \x00" as *const u8 as
         *const libc::c_char,
     b"        ................X       \x00" as *const u8 as
         *const libc::c_char,
     b"       ..................X      \x00" as *const u8 as
         *const libc::c_char,
     b"       ..................X      \x00" as *const u8 as
         *const libc::c_char,
     b"      ....................X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ....................X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ....................X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ....................X     \x00" as *const u8 as
         *const libc::c_char,
     b"      ....................X     \x00" as *const u8 as
         *const libc::c_char,
     b"       ..................X      \x00" as *const u8 as
         *const libc::c_char,
     b"       ..................X      \x00" as *const u8 as
         *const libc::c_char,
     b"        ................X       \x00" as *const u8 as
         *const libc::c_char,
     b"         ..............X        \x00" as *const u8 as
         *const libc::c_char,
     b"          ............X         \x00" as *const u8 as
         *const libc::c_char,
     b"            ........XX          \x00" as *const u8 as
         *const libc::c_char,
     b"            XXXXXXXX            \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,16\x00" as *const u8 as *const libc::c_char];
static mut cursor_bridge: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"           ..                   \x00" as *const u8 as
         *const libc::c_char,
     b"         ....                   \x00" as *const u8 as
         *const libc::c_char,
     b"       .......                  \x00" as *const u8 as
         *const libc::c_char,
     b"     .........                  \x00" as *const u8 as
         *const libc::c_char,
     b"   ............                 \x00" as *const u8 as
         *const libc::c_char,
     b"   ............             ..  \x00" as *const u8 as
         *const libc::c_char,
     b"   X............          .. .  \x00" as *const u8 as
         *const libc::c_char,
     b"   X............        ..XX.   \x00" as *const u8 as
         *const libc::c_char,
     b"    X............     ..XXX.    \x00" as *const u8 as
         *const libc::c_char,
     b"    X............   ..XXXX.     \x00" as *const u8 as
         *const libc::c_char,
     b"     X..............XXXXX.      \x00" as *const u8 as
         *const libc::c_char,
     b"   ..X............XXXXXX.       \x00" as *const u8 as
         *const libc::c_char,
     b"  ....X..........XXXXX..        \x00" as *const u8 as
         *const libc::c_char,
     b"  ....X.........XXXX......      \x00" as *const u8 as
         *const libc::c_char,
     b"  .....X.......XXX.........     \x00" as *const u8 as
         *const libc::c_char,
     b"  .....X......XX............    \x00" as *const u8 as
         *const libc::c_char,
     b"  ......X....XXXXX..........X   \x00" as *const u8 as
         *const libc::c_char,
     b"  ......X...XX..XXXX........X   \x00" as *const u8 as
         *const libc::c_char,
     b" ........X. ..XXX    .......X   \x00" as *const u8 as
         *const libc::c_char,
     b".........X..XXX      .......X.. \x00" as *const u8 as
         *const libc::c_char,
     b" XX......X XX        ..........X\x00" as *const u8 as
         *const libc::c_char,
     b"   XXX...X           .........XX\x00" as *const u8 as
         *const libc::c_char,
     b"      XXXX           .........X \x00" as *const u8 as
         *const libc::c_char,
     b"                     ........XX \x00" as *const u8 as
         *const libc::c_char,
     b"                   ..........X  \x00" as *const u8 as
         *const libc::c_char,
     b"                    XX......XX  \x00" as *const u8 as
         *const libc::c_char,
     b"                      XXX...X   \x00" as *const u8 as
         *const libc::c_char,
     b"                         XXXX   \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,16\x00" as *const u8 as *const libc::c_char];
static mut cursor_build: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                ......          \x00" as *const u8 as
         *const libc::c_char,
     b"              .....XX..X        \x00" as *const u8 as
         *const libc::c_char,
     b"           .X.....X  XXX        \x00" as *const u8 as
         *const libc::c_char,
     b"          .X......X             \x00" as *const u8 as
         *const libc::c_char,
     b"          X.......X             \x00" as *const u8 as
         *const libc::c_char,
     b"          .......XX             \x00" as *const u8 as
         *const libc::c_char,
     b"         .......X               \x00" as *const u8 as
         *const libc::c_char,
     b"         ......X.X              \x00" as *const u8 as
         *const libc::c_char,
     b"       .......X...X             \x00" as *const u8 as
         *const libc::c_char,
     b"      .......X ....X            \x00" as *const u8 as
         *const libc::c_char,
     b"       ....XX   ....X           \x00" as *const u8 as
         *const libc::c_char,
     b"        ...X     ....X          \x00" as *const u8 as
         *const libc::c_char,
     b"         .X       ....X         \x00" as *const u8 as
         *const libc::c_char,
     b"                   ....X        \x00" as *const u8 as
         *const libc::c_char,
     b"                    ....X       \x00" as *const u8 as
         *const libc::c_char,
     b"                     ...X       \x00" as *const u8 as
         *const libc::c_char,
     b"                      ..X       \x00" as *const u8 as
         *const libc::c_char,
     b"                       X        \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_embark: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"           ...........X         \x00" as *const u8 as
         *const libc::c_char,
     b"           ...........X         \x00" as *const u8 as
         *const libc::c_char,
     b"           ...........X         \x00" as *const u8 as
         *const libc::c_char,
     b"           ...........X         \x00" as *const u8 as
         *const libc::c_char,
     b"        .................X      \x00" as *const u8 as
         *const libc::c_char,
     b"         ...............XX      \x00" as *const u8 as
         *const libc::c_char,
     b"          .............X        \x00" as *const u8 as
         *const libc::c_char,
     b"           ...........X         \x00" as *const u8 as
         *const libc::c_char,
     b"           X.........X          \x00" as *const u8 as
         *const libc::c_char,
     b"        ...XX.......XX...X      \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XX.....XX...X       \x00" as *const u8 as
         *const libc::c_char,
     b"          ...XX...XX...X        \x00" as *const u8 as
         *const libc::c_char,
     b"           ...XX.XX...X         \x00" as *const u8 as
         *const libc::c_char,
     b"            ...XXX...X          \x00" as *const u8 as
         *const libc::c_char,
     b"             ...X...X           \x00" as *const u8 as
         *const libc::c_char,
     b"              .....X            \x00" as *const u8 as
         *const libc::c_char,
     b"               ...X             \x00" as *const u8 as
         *const libc::c_char,
     b"                .X              \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,22\x00" as *const u8 as *const libc::c_char];
static mut cursor_fix: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"           ...                  \x00" as *const u8 as
         *const libc::c_char,
     b"             ..X                \x00" as *const u8 as
         *const libc::c_char,
     b"              ..X               \x00" as *const u8 as
         *const libc::c_char,
     b"              ..X               \x00" as *const u8 as
         *const libc::c_char,
     b"       .      ..X               \x00" as *const u8 as
         *const libc::c_char,
     b"       .X     ..X               \x00" as *const u8 as
         *const libc::c_char,
     b"       .X    X..X               \x00" as *const u8 as
         *const libc::c_char,
     b"       ..XXXX...X               \x00" as *const u8 as
         *const libc::c_char,
     b"        .........X              \x00" as *const u8 as
         *const libc::c_char,
     b"         .........X             \x00" as *const u8 as
         *const libc::c_char,
     b"              .....X            \x00" as *const u8 as
         *const libc::c_char,
     b"               .....XXXXX       \x00" as *const u8 as
         *const libc::c_char,
     b"                .........X      \x00" as *const u8 as
         *const libc::c_char,
     b"                 .........X     \x00" as *const u8 as
         *const libc::c_char,
     b"                  ...X   ..X    \x00" as *const u8 as
         *const libc::c_char,
     b"                  ..X     .X    \x00" as *const u8 as
         *const libc::c_char,
     b"                  ..X     .X    \x00" as *const u8 as
         *const libc::c_char,
     b"                  ..X           \x00" as *const u8 as
         *const libc::c_char,
     b"                  ..X           \x00" as *const u8 as
         *const libc::c_char,
     b"                   ..XXX        \x00" as *const u8 as
         *const libc::c_char,
     b"                    ...X        \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,16\x00" as *const u8 as *const libc::c_char];
static mut cursor_guard: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"       .....X      .....        \x00" as *const u8 as
         *const libc::c_char,
     b"       .XXX.........XXX.X       \x00" as *const u8 as
         *const libc::c_char,
     b"       .X..XXXXXXXXX..X.X       \x00" as *const u8 as
         *const libc::c_char,
     b"       .X.............X.X       \x00" as *const u8 as
         *const libc::c_char,
     b"       .X.............X.X       \x00" as *const u8 as
         *const libc::c_char,
     b"       .X.............X.X       \x00" as *const u8 as
         *const libc::c_char,
     b"       .X............X.X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X...........X.X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X...........X.X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .XX.........XX.X        \x00" as *const u8 as
         *const libc::c_char,
     b"         .X.........X.X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .XX.......XX.X         \x00" as *const u8 as
         *const libc::c_char,
     b"          .XX.....XX.X          \x00" as *const u8 as
         *const libc::c_char,
     b"           .XX...XX.X           \x00" as *const u8 as
         *const libc::c_char,
     b"            ..XXX..X            \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"               .XX              \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,17\x00" as *const u8 as *const libc::c_char];
static mut cursor_jam: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"           ...XXX...X           \x00" as *const u8 as
         *const libc::c_char,
     b"          ..XX   XX..X          \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X       X..X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X   ...X  X.X         \x00" as *const u8 as
         *const libc::c_char,
     b"        ..X  .X  .X  ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .X    .X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .X .X .X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .XX.XX.X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        ..X  X...XX  ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"         .X   ...X   .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X .....X ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"          .XX.....XX.X          \x00" as *const u8 as
         *const libc::c_char,
     b"           X.......XX           \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"            XXXXXXXXX           \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_lockon: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"             .....X             \x00" as *const u8 as
         *const libc::c_char,
     b"           ...XXX...X           \x00" as *const u8 as
         *const libc::c_char,
     b"          ..XX   XX..X          \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X       X..X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X   ...X  X.X         \x00" as *const u8 as
         *const libc::c_char,
     b"        ..X  .X  .X  ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .X    .X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .X .X .X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        .X  .XX.XX.X  .X        \x00" as *const u8 as
         *const libc::c_char,
     b"        ..X  X...XX  ..X        \x00" as *const u8 as
         *const libc::c_char,
     b"         .X   ...X   .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X .....X ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"          .XX.....XX.X          \x00" as *const u8 as
         *const libc::c_char,
     b"           X.......XX           \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"            XXXXXXXXX           \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_menu: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"          ..XX                  \x00" as *const u8 as
         *const libc::c_char,
     b"          ....XX                \x00" as *const u8 as
         *const libc::c_char,
     b"           .....XX              \x00" as *const u8 as
         *const libc::c_char,
     b"           .......XX            \x00" as *const u8 as
         *const libc::c_char,
     b"            ........XX          \x00" as *const u8 as
         *const libc::c_char,
     b"            ..........XX        \x00" as *const u8 as
         *const libc::c_char,
     b"             ........XX         \x00" as *const u8 as
         *const libc::c_char,
     b"             .....XX            \x00" as *const u8 as
         *const libc::c_char,
     b"              ...XX             \x00" as *const u8 as
         *const libc::c_char,
     b"              ...XX             \x00" as *const u8 as
         *const libc::c_char,
     b"               ..XX             \x00" as *const u8 as
         *const libc::c_char,
     b"               . X              \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"11,11\x00" as *const u8 as *const libc::c_char];
static mut cursor_move: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"           .  .....X .          \x00" as *const u8 as
         *const libc::c_char,
     b"          ..X.XXXXX.X..         \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XXX    XX...        \x00" as *const u8 as
         *const libc::c_char,
     b"        ....X.X    .X....       \x00" as *const u8 as
         *const libc::c_char,
     b"       .....X.X    .X.....      \x00" as *const u8 as
         *const libc::c_char,
     b"        XXXXX.X    .XXXXX       \x00" as *const u8 as
         *const libc::c_char,
     b"        .X....X    ....X.X      \x00" as *const u8 as
         *const libc::c_char,
     b"       .XXXXXXX          .X     \x00" as *const u8 as
         *const libc::c_char,
     b"       .X                .X     \x00" as *const u8 as
         *const libc::c_char,
     b"       .X                .X     \x00" as *const u8 as
         *const libc::c_char,
     b"       .X                .X     \x00" as *const u8 as
         *const libc::c_char,
     b"       .X                .X     \x00" as *const u8 as
         *const libc::c_char,
     b"        .X....     ....X.X      \x00" as *const u8 as
         *const libc::c_char,
     b"        XXXXX.X    .XXXXX       \x00" as *const u8 as
         *const libc::c_char,
     b"       .....X.X    .X.....      \x00" as *const u8 as
         *const libc::c_char,
     b"        ....X.X    .X....X      \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XXX    XX...X       \x00" as *const u8 as
         *const libc::c_char,
     b"          ..X.X    .X..X        \x00" as *const u8 as
         *const libc::c_char,
     b"           .X .....X .X         \x00" as *const u8 as
         *const libc::c_char,
     b"              XXXXX             \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_notpossible: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"        .X            .X        \x00" as *const u8 as
         *const libc::c_char,
     b"       ...X          ...X       \x00" as *const u8 as
         *const libc::c_char,
     b"      .....X        .....X      \x00" as *const u8 as
         *const libc::c_char,
     b"     .......X      .......X     \x00" as *const u8 as
         *const libc::c_char,
     b"      .......X    .......X      \x00" as *const u8 as
         *const libc::c_char,
     b"       .......X  .......X       \x00" as *const u8 as
         *const libc::c_char,
     b"        .......X.......X        \x00" as *const u8 as
         *const libc::c_char,
     b"         .............X         \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"            .......X            \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X           \x00" as *const u8 as
         *const libc::c_char,
     b"          ...........X          \x00" as *const u8 as
         *const libc::c_char,
     b"         .............X         \x00" as *const u8 as
         *const libc::c_char,
     b"        .......X.......X        \x00" as *const u8 as
         *const libc::c_char,
     b"       .......X  .......X       \x00" as *const u8 as
         *const libc::c_char,
     b"      .......X    .......X      \x00" as *const u8 as
         *const libc::c_char,
     b"     .......X      .......X     \x00" as *const u8 as
         *const libc::c_char,
     b"      .....X        .....X      \x00" as *const u8 as
         *const libc::c_char,
     b"       ...X          ...X       \x00" as *const u8 as
         *const libc::c_char,
     b"        .X            .X        \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,17\x00" as *const u8 as *const libc::c_char];
static mut cursor_pickup: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"             .X  .X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .X  .X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .X.X.X             \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"             .X.X.X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .X  .X             \x00" as *const u8 as
         *const libc::c_char,
     b"             .X.X.X             \x00" as *const u8 as
         *const libc::c_char,
     b"              ...X              \x00" as *const u8 as
         *const libc::c_char,
     b"              X.XX              \x00" as *const u8 as
         *const libc::c_char,
     b"             .X.X.X             \x00" as *const u8 as
         *const libc::c_char,
     b"            ..X.X..X            \x00" as *const u8 as
         *const libc::c_char,
     b"           ...X.X...X           \x00" as *const u8 as
         *const libc::c_char,
     b"          ....X.X....X          \x00" as *const u8 as
         *const libc::c_char,
     b"         .....XXX.....X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ...XXXXXXX...X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .XXX      XX.X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X          .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X          .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X          .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         .X          .X         \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X        ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"          ..X      ..X          \x00" as *const u8 as
         *const libc::c_char,
     b"           ..X    ..X           \x00" as *const u8 as
         *const libc::c_char,
     b"            .X    .X            \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,20\x00" as *const u8 as *const libc::c_char];
static mut cursor_seekrepair: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"      ...                       \x00" as *const u8 as
         *const libc::c_char,
     b"        ..X             X       \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X            .X      \x00" as *const u8 as
         *const libc::c_char,
     b"         ..X            ..X     \x00" as *const u8 as
         *const libc::c_char,
     b"  .      ..X        XXXX...X    \x00" as *const u8 as
         *const libc::c_char,
     b"  .X     ..X        ........X   \x00" as *const u8 as
         *const libc::c_char,
     b"  .X    X..X        .........X  \x00" as *const u8 as
         *const libc::c_char,
     b"  ..XXXX...X        ..........  \x00" as *const u8 as
         *const libc::c_char,
     b"   .........X       .........   \x00" as *const u8 as
         *const libc::c_char,
     b"    .........X      ........    \x00" as *const u8 as
         *const libc::c_char,
     b"         .....X         ...     \x00" as *const u8 as
         *const libc::c_char,
     b"          .....XXXXX    ..      \x00" as *const u8 as
         *const libc::c_char,
     b"           .........X   .       \x00" as *const u8 as
         *const libc::c_char,
     b"            .........X          \x00" as *const u8 as
         *const libc::c_char,
     b"             ...X   ..X         \x00" as *const u8 as
         *const libc::c_char,
     b"             ..X     .X         \x00" as *const u8 as
         *const libc::c_char,
     b"             ..X     .X         \x00" as *const u8 as
         *const libc::c_char,
     b"             ..X                \x00" as *const u8 as
         *const libc::c_char,
     b"             ..X                \x00" as *const u8 as
         *const libc::c_char,
     b"              ..XXX             \x00" as *const u8 as
         *const libc::c_char,
     b"               ...X             \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"16,15\x00" as *const u8 as *const libc::c_char];
static mut cursor_select: [*const libc::c_char; 37] =
    [b"    32    32        3            1\x00" as *const u8 as
         *const libc::c_char,
     b"X c #000000\x00" as *const u8 as *const libc::c_char,
     b". c #ffffff\x00" as *const u8 as *const libc::c_char,
     b"  c None\x00" as *const u8 as *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"    .....X            .....X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .XXXXX             XXX.X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    XX                    XX    \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"               .X               \x00" as *const u8 as
         *const libc::c_char,
     b"               XX               \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"    .                     .     \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .X                    .X    \x00" as *const u8 as
         *const libc::c_char,
     b"    .....X            .....X    \x00" as *const u8 as
         *const libc::c_char,
     b"    XXXXXX            XXXXXX    \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"                                \x00" as *const u8 as
         *const libc::c_char,
     b"15,15\x00" as *const u8 as *const libc::c_char];
unsafe extern "C" fn init_system_cursor(mut image: *mut *const libc::c_char)
 -> *mut SDL_Cursor {
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut data: [Uint8; 128] = [0; 128];
    let mut mask: [Uint8; 128] = [0; 128];
    let mut hot_x: libc::c_int = 0;
    let mut hot_y: libc::c_int = 0;
    i = -(1 as libc::c_int);
    row = 0 as libc::c_int;
    while row < 32 as libc::c_int {
        col = 0 as libc::c_int;
        while col < 32 as libc::c_int {
            if col % 8 as libc::c_int != 0 {
                data[i as usize] =
                    ((data[i as usize] as libc::c_int) << 1 as libc::c_int) as
                        Uint8;
                mask[i as usize] =
                    ((mask[i as usize] as libc::c_int) << 1 as libc::c_int) as
                        Uint8
            } else {
                i += 1;
                mask[i as usize] = 0 as libc::c_int as Uint8;
                data[i as usize] = mask[i as usize]
            }
            match *(*image.offset((4 as libc::c_int + row) as
                                      isize)).offset(col as isize) as
                      libc::c_int {
                88 => {
                    data[i as usize] =
                        (data[i as usize] as libc::c_int | 0x1 as libc::c_int)
                            as Uint8;
                    mask[i as usize] =
                        (mask[i as usize] as libc::c_int | 0x1 as libc::c_int)
                            as Uint8
                }
                46 => {
                    mask[i as usize] =
                        (mask[i as usize] as libc::c_int | 0x1 as libc::c_int)
                            as Uint8
                }
                32 | _ => { }
            }
            col += 1
        }
        row += 1
    }
    sscanf(*image.offset((4 as libc::c_int + row) as isize),
           b"%d,%d\x00" as *const u8 as *const libc::c_char,
           &mut hot_x as *mut libc::c_int, &mut hot_y as *mut libc::c_int);
    return SDL_CreateCursor(data.as_mut_ptr(), mask.as_mut_ptr(),
                            32 as libc::c_int, 32 as libc::c_int, hot_x,
                            hot_y);
}
/*
 * Frame.c
 *
 * Initialisation and shutdown for the framework library.
 *
 * Includes a basic windows message loop.
 *
 */
// defines the inline functions in this module
// window focus messages
//#define DEBUG_GROUP1
/* Linux specific stuff */
static mut wzFPSmanager: FPSmanager =
    FPSmanager{framecount: 0, rateticks: 0., lastticks: 0, rate: 0,};
#[no_mangle]
pub unsafe extern "C" fn unix_path(mut path: *const libc::c_char)
 -> *mut libc::c_char {
    static mut returnval: [libc::c_char; 512] = [0; 512];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while *path.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        if *path.offset(i as isize) as libc::c_int >= 'A' as i32 &&
               *path.offset(i as isize) as libc::c_int <= 'Z' as i32 {
            returnval[i as usize] =
                (*path.offset(i as isize) as libc::c_int - 'A' as i32 +
                     'a' as i32) as libc::c_char
        } else if *path.offset(i as isize) as libc::c_int == '\\' as i32 {
            returnval[i as usize] = '/' as i32 as libc::c_char
        } else { returnval[i as usize] = *path.offset(i as isize) }
        i = i.wrapping_add(1)
    }
    while returnval[i.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
              as libc::c_int == '/' as i32 {
        i = i.wrapping_sub(1)
    }
    returnval[i as usize] = '\u{0}' as i32 as libc::c_char;
    return returnval.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn unix_fopen(mut filename: *const libc::c_char,
                                    mut mode: *const libc::c_char)
 -> *mut FILE {
    // ridiculous kludge because we redefine fopen to unix_fopen
    return fopen(unix_path(filename), mode);
}
/* Handle for the main window */
#[no_mangle]
pub static mut hWndMain: HANDLE =
    0 as *const libc::c_void as *mut libc::c_void;
/* Program hInstance */
#[no_mangle]
pub static mut hInstance: HINSTANCE = 0;
/* Flag if directdraw is active*/
static mut bActiveDDraw: BOOL = 0;
static mut currentCursorResID: WORD = 0xffff as libc::c_int as WORD;
#[no_mangle]
pub static mut aCursors: [*mut SDL_Cursor; 26] =
    [0 as *const SDL_Cursor as *mut SDL_Cursor; 26];
/* Stores whether a windows quit message has been received */
static mut winQuit: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut focusState: FOCUS_STATE = FOCUS_OUT;
#[no_mangle]
pub static mut focusLast: FOCUS_STATE = FOCUS_OUT;
/* Whether the mouse is currently being displayed or not */
static mut mouseOn: BOOL = 1 as libc::c_int;
/* Whether the mouse should be displayed in the app workspace */
static mut displayMouse: BOOL = 1 as libc::c_int;
/* Global variables for the frame rate stuff */
static mut FrameCounts: [SDWORD; 5] = [0; 5];
static mut Frames: SDWORD = 0;
// Number of frames elapsed since start
static mut LastFrames: SDWORD = 0;
static mut RecentFrames: SDWORD = 0;
// Number of frames in last second
static mut PresSeconds: SDWORD = 0;
// Number of seconds since execution started
static mut LastSeconds: SDWORD = 0;
static mut FrameRate: SDWORD = 0;
// Average frame rate since start
static mut FrameIndex: SDWORD = 0;
static mut Total: SDWORD = 0;
static mut RecentAverage: SDWORD = 0;
// Average frame rate over last TIMSPAN seconds
/* InitFrameStuff - needs to be called once before frame loop commences */
unsafe extern "C" fn InitFrameStuff() {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        FrameCounts[i as usize] = 70 as libc::c_int;
        i += 1
    }
    Frames = 0 as libc::c_int;
    LastFrames = 0 as libc::c_int;
    RecentFrames = 0 as libc::c_int;
    RecentAverage = 0 as libc::c_int;
    PresSeconds = 0 as libc::c_int;
    LastSeconds = 0 as libc::c_int;
    LastSeconds = PresSeconds;
    FrameIndex = 0 as libc::c_int;
}
/* MaintainFrameStuff - call this during completion of each frame loop */
unsafe extern "C" fn MaintainFrameStuff() {
    let mut i: SDWORD = 0;
    PresSeconds = (clock() / 1000000 as libc::c_int as __clock_t) as SDWORD;
    if PresSeconds != LastSeconds {
        LastSeconds = PresSeconds;
        RecentFrames = Frames - LastFrames;
        LastFrames = Frames;
        let fresh0 = FrameIndex;
        FrameIndex = FrameIndex + 1;
        FrameCounts[fresh0 as usize] = RecentFrames;
        if FrameIndex >= 5 as libc::c_int { FrameIndex = 0 as libc::c_int }
        Total = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 5 as libc::c_int {
            Total += FrameCounts[i as usize];
            i += 1
        }
        RecentAverage = Total / 5 as libc::c_int;
        if PresSeconds > 0 as libc::c_int { FrameRate = Frames / PresSeconds }
    }
    Frames += 1;
}
/* replacement for win32 function */
//Check this.  [test] --Qamly
//was WIN32, but gcc is OK with this?			//Note, I vote for name change, since we are using SDL now right? --Qamly
#[no_mangle]
pub unsafe extern "C" fn GetTickCount() -> DWORD {
    return SDL_GetTicks() as DWORD;
}
/* Return the current frame rate */
#[no_mangle]
pub unsafe extern "C" fn frameGetFrameRate() -> UDWORD {
    return RecentAverage as UDWORD;
}
/* Return the overall frame rate */
#[no_mangle]
pub unsafe extern "C" fn frameGetOverallRate() -> UDWORD {
    return FrameRate as UDWORD;
}
/* Return the frame rate for the last second */
#[no_mangle]
pub unsafe extern "C" fn frameGetRecentRate() -> UDWORD {
    return RecentFrames as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn frameGetFrameNumber() -> UDWORD {
    return Frames as UDWORD;
}
/* Return the handle for the application window */
#[no_mangle]
pub unsafe extern "C" fn frameGetWinHandle() -> HANDLE { return hWndMain; }
/* If cursor on is TRUE the windows cursor will be displayed over the game window
 * (and in full screen mode).  If it is FALSE the cursor will not be displayed.
 */
#[no_mangle]
pub unsafe extern "C" fn frameShowCursor(mut cursorOn: BOOL) {
    displayMouse = cursorOn;
}
/* Set the current cursor from a cursor handle */
#[no_mangle]
pub unsafe extern "C" fn frameSetCursor(mut hNewCursor: HCURSOR) { }
/* Set the current cursor from a Resource ID */
#[no_mangle]
pub unsafe extern "C" fn frameSetCursorFromRes(mut resID: WORD) {
    if resID as libc::c_int >= 100 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"frameSetCursorFromRes: bad resource ID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if resID as libc::c_int >= 100 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"frame.c\x00" as *const u8 as *const libc::c_char,
              237 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"frameSetCursorFromRes\x00")).as_ptr(),
              b"resID >= CURSOR_OFFSET\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (resID as libc::c_int) < 100 as libc::c_int + 26 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"frameSetCursorFromRes: bad resource ID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (resID as libc::c_int) < 100 as libc::c_int + 26 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"frame.c\x00" as *const u8 as *const libc::c_char,
              238 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"frameSetCursorFromRes\x00")).as_ptr(),
              b"resID < CURSOR_OFFSET + MAX_CURSORS\x00" as *const u8 as
                  *const libc::c_char);
    };
    //If we are already using this cursor then  return
    if resID as libc::c_int != currentCursorResID as libc::c_int {
        SDL_SetCursor(aCursors[(resID as libc::c_int - 100 as libc::c_int) as
                                   usize]);
        currentCursorResID = resID
    };
}
/*
 * Wndproc
 *
 * The windows message processing function.
 */
unsafe extern "C" fn processEvent(mut event: *mut SDL_Event) {
    match (*event).type_0 as libc::c_int {
        3 | 2 | 6 | 5 | 4 => { inputProcessEvent(event); }
        _ => { }
    };
}
unsafe extern "C" fn initCursors() {
    aCursors[(100 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_arrow.as_mut_ptr());
    aCursors[(101 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_dest.as_mut_ptr());
    aCursors[(102 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_sight.as_mut_ptr());
    aCursors[(103 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_target.as_mut_ptr());
    aCursors[(104 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_larrow.as_mut_ptr());
    aCursors[(105 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_rarrow.as_mut_ptr());
    aCursors[(106 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_darrow.as_mut_ptr());
    aCursors[(107 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_uarrow.as_mut_ptr());
    aCursors[(108 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_default.as_mut_ptr());
    aCursors[(109 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_default.as_mut_ptr());
    aCursors[(110 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_attach.as_mut_ptr());
    aCursors[(111 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_attack.as_mut_ptr());
    aCursors[(112 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_bomb.as_mut_ptr());
    aCursors[(113 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_bridge.as_mut_ptr());
    aCursors[(114 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_build.as_mut_ptr());
    aCursors[(115 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_embark.as_mut_ptr());
    aCursors[(116 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_fix.as_mut_ptr());
    aCursors[(117 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_guard.as_mut_ptr());
    aCursors[(118 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_jam.as_mut_ptr());
    aCursors[(119 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_lockon.as_mut_ptr());
    aCursors[(120 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_menu.as_mut_ptr());
    aCursors[(121 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_move.as_mut_ptr());
    aCursors[(122 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_notpossible.as_mut_ptr());
    aCursors[(123 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_pickup.as_mut_ptr());
    aCursors[(124 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_seekrepair.as_mut_ptr());
    aCursors[(125 as libc::c_int - 100 as libc::c_int) as usize] =
        init_system_cursor(cursor_select.as_mut_ptr());
}
unsafe extern "C" fn freeCursors() {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 26 as libc::c_int as libc::c_uint {
        SDL_FreeCursor(aCursors[i as usize]);
        i = i.wrapping_add(1)
    };
}
/*
 * frameInitialise
 *
 * Initialise the framework library. - PC version
 */
#[no_mangle]
pub unsafe extern "C" fn frameInitialise(mut hInst: HANDLE,
                                         mut pWindowName: *mut STRING,
                                         mut width: UDWORD,
                                         mut height: UDWORD,
                                         mut bitDepth: UDWORD,
                                         mut fullScreen: BOOL,
                                         mut bVidMem: BOOL) -> BOOL 
 // Whether to put surfaces in video memory
 {
    if SDL_Init((0x20 as libc::c_int | 0x100 as libc::c_int) as Uint32) !=
           0 as libc::c_int {
        debug(LOG_ERROR,
              b"Error: Could not initialise SDL (%s).\n\x00" as *const u8 as
                  *const libc::c_char, SDL_GetError());
        return 0 as libc::c_int
    }
    SDL_WM_SetCaption(pWindowName, 0 as *const libc::c_char);
    winQuit = 0 as libc::c_int;
    focusState = FOCUS_IN;
    focusLast = FOCUS_IN;
    mouseOn = 1 as libc::c_int;
    displayMouse = 1 as libc::c_int;
    //	hInstance = hInst;
    bActiveDDraw = 1 as libc::c_int;
    //	/* Initialise the memory system */
//	if (!memInitialise())
//	{
//		return FALSE;
//	}
    //	if (!blkInitialise())
//	{
//		return FALSE;
//	}
    /* Initialise the trig stuff */
    if trigInitialise() == 0 { return 0 as libc::c_int }
    /* initialise all cursors */
    initCursors();
    /* Initialise the Direct Draw Buffers */
    if screenInitialise(width, height, bitDepth, fullScreen, bVidMem,
                        bActiveDDraw, hWndMain) == 0 {
        return 0 as libc::c_int
    }
    /* Initialise the input system */
    inputInitialise();
    /* Initialise the frame rate stuff */
    InitFrameStuff();
    SDL_initFramerate(&mut wzFPSmanager);
    SDL_setFramerate(&mut wzFPSmanager, 60 as libc::c_int);
    // Initialise the resource stuff
    if resInitialise() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*
 * frameUpdate
 *
 * Call this each cycle to allow the framework to deal with
 * windows messages, and do general house keeping.
 *
 * Returns FRAME_STATUS.
 */
#[no_mangle]
pub unsafe extern "C" fn frameUpdate() -> FRAME_STATUS {
    let mut event: SDL_Event = SDL_Event{type_0: 0,};
    let mut retVal: FRAME_STATUS = FRAME_OK;
    /* Tell the input system about the start of another frame */
    inputNewFrame();
    /* Deal with any windows messages */
    while SDL_PollEvent(&mut event) != 0 as libc::c_int {
        if event.type_0 as libc::c_int == SDL_QUIT as libc::c_int { break ; }
        processEvent(&mut event);
    }
    /* Now figure out what to return */
    retVal = FRAME_OK;
    if winQuit != 0 {
        retVal = FRAME_QUIT
    } else if focusState as libc::c_uint ==
                  FOCUS_SET as libc::c_int as libc::c_uint &&
                  focusLast as libc::c_uint ==
                      FOCUS_OUT as libc::c_int as libc::c_uint {
        focusState = FOCUS_IN;
        retVal = FRAME_SETFOCUS
    } else if focusState as libc::c_uint ==
                  FOCUS_KILL as libc::c_int as libc::c_uint &&
                  focusLast as libc::c_uint ==
                      FOCUS_IN as libc::c_int as libc::c_uint {
        focusState = FOCUS_OUT;
        retVal = FRAME_KILLFOCUS
    }
    if focusState as libc::c_uint == FOCUS_SET as libc::c_int as libc::c_uint
           ||
           focusState as libc::c_uint ==
               FOCUS_KILL as libc::c_int as libc::c_uint {
        /* Got a SET or KILL when we were already in or out of
		   focus respectively */
        focusState = focusLast
    } else if focusLast as libc::c_uint != focusState as libc::c_uint {
        focusLast = focusState
    }
    /* If things are running normally update the framerate */
    if winQuit == 0 &&
           focusState as libc::c_uint ==
               FOCUS_IN as libc::c_int as libc::c_uint {
        /* Update the frame rate stuff */
        MaintainFrameStuff();
        SDL_framerateDelay(&mut wzFPSmanager);
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn frameShutDown() {
    screenShutDown();
    /* Free the default cursor */
	// DestroyCursor(hCursor);
    /* Free all cursors */
    freeCursors();
    /* Destroy the Application window */
    SDL_Quit();
    /* shutdown the trig stuff */
    trigShutDown();
    // Shutdown the resource stuff
    resShutDown();
    // shutdown the block memory heap
    blkShutDown();
    /* Shutdown the memory system */
    memShutDown();
}
/* **************************************************************************
  Load the file with name pointed to by pFileName into a memory buffer.
  If AllocateMem is true then the memory is allocated ... else it is
  already in allocated in ppFileData, and the max size is in pFileSize
  ... this is adjusted to the actual loaded file size.

  If hard_fail is true, we will assert and report on failures.
***************************************************************************/
unsafe extern "C" fn loadFile2(mut pFileName: *mut libc::c_char,
                               mut ppFileData: *mut *mut libc::c_char,
                               mut pFileSize: *mut UDWORD,
                               mut AllocateMem: BOOL, mut hard_fail: BOOL)
 -> BOOL {
    let mut pfile: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut filesize: PHYSFS_sint64 = 0;
    let mut length_read: PHYSFS_sint64 = 0;
    if PHYSFSEXT_locateCorrectCase(pFileName) != 0 {
        if hard_fail != 0 {
            debug(LOG_ERROR,
                  b"loadFile2: %s not found\x00" as *const u8 as
                      *const libc::c_char, pFileName);
        }
        return 0 as libc::c_int
    }
    pfile = PHYSFS_openRead(pFileName);
    if pfile.is_null() {
        debug(LOG_ERROR,
              b"loadFile2: %s could not be opened: %s\x00" as *const u8 as
                  *const libc::c_char, pFileName, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    filesize = PHYSFS_fileLength(pfile);
    //debug(LOG_WZ, "loadFile2: %s opened, size %i", pFileName, filesize);
    if AllocateMem == 1 as libc::c_int {
        // Allocate a buffer to store the data and a terminating zero
        *ppFileData =
            memMallocRelease((filesize + 1 as libc::c_int as libc::c_longlong)
                                 as size_t) as *mut libc::c_char;
        if (*ppFileData).is_null() {
            debug(LOG_ERROR,
                  b"loadFile2: Out of memory loading %s\x00" as *const u8 as
                      *const libc::c_char, pFileName);
            return 0 as libc::c_int
        }
    } else if filesize > *pFileSize as libc::c_longlong {
        debug(LOG_ERROR,
              b"loadFile2: No room for file %s\x00" as *const u8 as
                  *const libc::c_char, pFileName);
        return 0 as libc::c_int
    }
    /* Load the file data */
    length_read =
        PHYSFS_read(pfile, *ppFileData as *mut libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32,
                    filesize as PHYSFS_uint32);
    if length_read != filesize {
        memFreeRelease(*ppFileData as *mut libc::c_void);
        *ppFileData = 0 as *mut libc::c_char;
        debug(LOG_ERROR,
              b"loadFile2: Reading %s short: %s\x00" as *const u8 as
                  *const libc::c_char, pFileName, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    if PHYSFS_close(pfile) == 0 {
        memFreeRelease(*ppFileData as *mut libc::c_void);
        *ppFileData = 0 as *mut libc::c_char;
        debug(LOG_ERROR,
              b"loadFile2: Error closing %s: %s\x00" as *const u8 as
                  *const libc::c_char, pFileName, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    // Add the terminating zero
    *(*ppFileData).offset(filesize as isize) =
        0 as libc::c_int as libc::c_char;
    // always set to correct size
    *pFileSize = filesize as UDWORD;
    return 1 as libc::c_int;
}
/* **************************************************************************
	Save the data in the buffer into the given file.
***************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn saveFile(mut pFileName: *const libc::c_char,
                                  mut pFileData: *const libc::c_char,
                                  mut fileSize: UDWORD) -> BOOL {
    let mut filename: [libc::c_char; 200] =
        [0; 200]; // pFileName in lowercase
    let mut pfile: *mut PHYSFS_File = 0 as *mut PHYSFS_File;
    let mut size: PHYSFS_uint32 = fileSize;
    strncpy(filename.as_mut_ptr(), unix_path(pFileName),
            ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong);
    debug(LOG_WZ,
          b"We are to write (%s) as (%s) of size %d\x00" as *const u8 as
              *const libc::c_char, pFileName, filename.as_mut_ptr(),
          fileSize);
    pfile = PHYSFS_openWrite(filename.as_mut_ptr());
    if pfile.is_null() {
        let mut found: *const libc::c_char =
            PHYSFS_getRealDir(filename.as_mut_ptr());
        debug(LOG_ERROR,
              b"saveFile: %s could not be opened: %s\x00" as *const u8 as
                  *const libc::c_char, filename.as_mut_ptr(),
              PHYSFS_getLastError());
        if !found.is_null() {
            debug(LOG_ERROR,
                  b"saveFile: %s found as %s\x00" as *const u8 as
                      *const libc::c_char, filename.as_mut_ptr(), found);
        }
        return 0 as libc::c_int
    }
    if PHYSFS_write(pfile, pFileData as *const libc::c_void,
                    1 as libc::c_int as PHYSFS_uint32, size) !=
           size as libc::c_longlong {
        debug(LOG_ERROR,
              b"saveFile: %s could not write: %s\x00" as *const u8 as
                  *const libc::c_char, filename.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    if PHYSFS_close(pfile) == 0 {
        debug(LOG_ERROR,
              b"saveFile: Error closing %s: %s\x00" as *const u8 as
                  *const libc::c_char, filename.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    if PHYSFS_getRealDir(filename.as_mut_ptr()).is_null() {
        // weird
        debug(LOG_ERROR,
              b"saveFile: PHYSFS_getRealDir(%s) returns NULL?!\x00" as
                  *const u8 as *const libc::c_char, filename.as_mut_ptr());
    } else {
        debug(LOG_WZ,
              b"Successfully wrote to %s%s%s with %d bytes\x00" as *const u8
                  as *const libc::c_char,
              PHYSFS_getRealDir(filename.as_mut_ptr()),
              PHYSFS_getDirSeparator(), filename.as_mut_ptr(), size);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadFile(mut pFileName: *const libc::c_char,
                                  mut ppFileData: *mut *mut libc::c_char,
                                  mut pFileSize: *mut UDWORD) -> BOOL {
    return loadFile2(unix_path(pFileName), ppFileData, pFileSize,
                     1 as libc::c_int, 1 as libc::c_int);
}
// load a file from disk into a fixed memory buffer
#[no_mangle]
pub unsafe extern "C" fn loadFileToBuffer(mut pFileName: *mut libc::c_char,
                                          mut pFileBuffer: *mut libc::c_char,
                                          mut bufferSize: UDWORD,
                                          mut pSize: *mut UDWORD) -> BOOL {
    *pSize = bufferSize;
    return loadFile2(unix_path(pFileName), &mut pFileBuffer, pSize,
                     0 as libc::c_int, 1 as libc::c_int);
}
// as above but returns quietly if no file found
#[no_mangle]
pub unsafe extern "C" fn loadFileToBufferNoError(mut pFileName:
                                                     *mut libc::c_char,
                                                 mut pFileBuffer:
                                                     *mut libc::c_char,
                                                 mut bufferSize: UDWORD,
                                                 mut pSize: *mut UDWORD)
 -> BOOL {
    *pSize = bufferSize;
    return loadFile2(unix_path(pFileName), &mut pFileBuffer, pSize,
                     0 as libc::c_int, 0 as libc::c_int);
}
//#define	HIGH_BITS		((UINT)(0xf0000000))
//#define	LOW_BITS		((UINT)(0x0fffffff))
// /////////////////////////////////////////////////////////////////
/* **************************************************************************/
/*
 * HashString
 *
 * Adaptation of Peter Weinberger's (PJW) generic hashing algorithm listed
 * in Binstock+Rex, "Practical Algorithms" p 69.
 *
 * Accepts string and returns hashed integer.
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn HashString(mut String: *mut libc::c_char) -> UINT {
    let mut iHashValue: UINT = 0;
    let mut i: UINT = 0;
    let mut c: *mut CHAR = String as *mut CHAR;
    iHashValue = 0 as libc::c_int as UINT;
    while *c != 0 {
        iHashValue =
            (iHashValue <<
                 (32 as libc::c_int / 8 as libc::c_int) as
                     UINT).wrapping_add(*c as libc::c_uint);
        i =
            iHashValue &
                !(!(0 as libc::c_int) as UINT >>
                      (32 as libc::c_int / 8 as libc::c_int) as UINT);
        if i != 0 as libc::c_int as libc::c_uint {
            iHashValue =
                (iHashValue ^
                     i >>
                         (32 as libc::c_int * 3 as libc::c_int /
                              4 as libc::c_int) as UINT) &
                    !!(!(0 as libc::c_int) as UINT >>
                           (32 as libc::c_int / 8 as libc::c_int) as UINT)
        }
        c = c.offset(1)
    }
    //	printf("%%%%%%%% String:%s Hash:%0x\n",String,iHashValue);
    return iHashValue;
}
/*
 * Frame.h
 *
 * The framework library initialisation and shutdown routines.
 *
 */
/* Linux specific stuff */
/* Initialise the frame work library */
// The windows application instance
// The text to appear in the window title bar
// The display width
// The display height
// The display bit depth
// Whether to start full screen or windowed
// Whether to put surfaces in video memory
/* Shut down the framework library.
 * This clears up all the Direct Draw stuff and ensures
 * that Windows gets restored properly after Full screen mode.
 */
/* The current status of the framework */
// Everything normal
// The main app window has lost focus (might well want to pause)
// The main app window has focus back
// The main app window has been told to quit
/* Call this each cycle to allow the framework to deal with
 * windows messages, and do general house keeping.
 *
 * Returns FRAME_STATUS.
 */
/* If cursor on is TRUE the windows cursor will be displayed over the game window
 * (and in full screen mode).  If it is FALSE the cursor will not be displayed.
 */
/* Set the current cursor from a cursor handle */
/* Set the current cursor from a Resource ID
 * This is the same as calling:
 *       frameSetCursor(LoadCursor(MAKEINTRESOURCE(resID)));
 * but with a bit of extra error checking.
 */
/* Returns the current frame we're on - used to establish whats on screen */
/* Return the current frame rate */
/* Return the overall frame rate */
/* Return the frame rate for the last second */
/* The handle for the application window */
//enumerate all available direct draw devices
// Return a string for a windows error code
/* The default window procedure for the library.
 * This is initially set to the standard DefWindowProc, but can be changed
 * by this function.
 * Call this function with NULL to reset to DefWindowProc.
 */
/* Load the file with name pointed to by pFileName into a memory buffer. */
// The filename
// A buffer containing the file contents
// The size of this buffer
/* Save the data in the buffer into the given file */
// load a file from disk into a fixed memory buffer
// as above but returns quietly if no file found
#[no_mangle]
pub unsafe extern "C" fn HashStringIgnoreCase(mut String: *mut libc::c_char)
 -> UINT {
    let mut iHashValue: UINT = 0;
    let mut i: UINT = 0;
    let mut c: *mut CHAR = String as *mut CHAR;
    iHashValue = 0 as libc::c_int as UINT;
    while *c != 0 {
        iHashValue =
            (iHashValue <<
                 (32 as libc::c_int / 8 as libc::c_int) as
                     UINT).wrapping_add((*c as libc::c_int &
                                             0xdf as libc::c_int) as
                                            libc::c_uint);
        i =
            iHashValue &
                !(!(0 as libc::c_int) as UINT >>
                      (32 as libc::c_int / 8 as libc::c_int) as UINT);
        if i != 0 as libc::c_int as libc::c_uint {
            iHashValue =
                (iHashValue ^
                     i >>
                         (32 as libc::c_int * 3 as libc::c_int /
                              4 as libc::c_int) as UINT) &
                    !!(!(0 as libc::c_int) as UINT >>
                           (32 as libc::c_int / 8 as libc::c_int) as UINT)
        }
        c = c.offset(1)
    }
    //	printf("%%%%%%%% (Ignorcase) String:%s Hash:%0x\n",String,iHashValue);
    return iHashValue;
}
// Examine a filename for the last dot and slash
// and so giving the extension of the file and the directory
//
// PosOfDot and/of PosOfSlash can be NULL and then nothing will be stored
//
#[no_mangle]
pub unsafe extern "C" fn ScanFilename(mut Fullname: *mut libc::c_char,
                                      mut PosOfDot: *mut libc::c_int,
                                      mut PosOfSlash: *mut libc::c_int) {
    let mut Namelength: libc::c_int = 0;
    let mut DotPos: libc::c_int = -(1 as libc::c_int);
    let mut SlashPos: libc::c_int = -(1 as libc::c_int);
    let mut Pos: libc::c_int = 0;
    Namelength = strlen(Fullname) as libc::c_int;
    Pos = Namelength;
    while Pos >= 0 as libc::c_int {
        if *Fullname.offset(Pos as isize) as libc::c_int == '.' as i32 {
            DotPos = Pos;
            break ;
        } else { Pos -= 1 }
    }
    Pos = Namelength;
    while Pos >= 0 as libc::c_int {
        if *Fullname.offset(Pos as isize) as libc::c_int == '\\' as i32 {
            SlashPos = Pos;
            break ;
        } else { Pos -= 1 }
    }
    if !PosOfDot.is_null() { *PosOfDot = DotPos }
    if !PosOfSlash.is_null() { *PosOfSlash = SlashPos };
}
