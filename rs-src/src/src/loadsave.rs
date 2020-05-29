use ::libc;
extern "C" {
    /* ! _ISbit  */
    /* These are defined in ctype-info.c.
   The declarations here must match those in localeinfo.h.

   In the thread-specific locale model (see `uselocale' in <locale.h>)
   we cannot use global variables for these as was done in the past.
   Instead, the following accessor functions return the address of
   each variable, which is local to the current thread if multithreaded.

   These point into arrays of 384, so they can be indexed by any `unsigned
   char' value [0,255]; by EOF (-1); or by any `signed char' value
   [-128,-1).  ISO C requires that the ctype functions work for `unsigned
   char' values and for EOF; we also support negative `signed char' values
   for broken old programs.  The case conversion arrays are of `int's
   rather than `unsigned char's because tolower (EOF) must be EOF, which
   doesn't fit into an `unsigned char'.  But today more important is that
   the arrays are also used for multi-byte character sets.  */
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Append SRC onto DEST.  */
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Compare S1 and S2.  */
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* Find the first occurrence of NEEDLE in HAYSTACK.  */
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
 * \fn int PHYSFS_delete(const char *filename)
 * \brief Delete a file or directory.
 *
 * (filename) is specified in platform-independent notation in relation to the
 *  write dir.
 *
 * A directory must be empty before this call can delete it.
 *
 * Deleting a symlink will remove the link, not what it points to, regardless
 *  of whether you "permitSymLinks" or not.
 *
 * So if you've got the write dir set to "C:\mygame\writedir" and call
 *  PHYSFS_delete("downloads/maps/level1.map") then the file
 *  "C:\mygame\writedir\downloads\maps\level1.map" is removed from the
 *  physical filesystem, if it exists and the operating system permits the
 *  deletion.
 *
 * Note that on Unix systems, deleting a file may be successful, but the
 *  actual file won't be removed until all processes that have an open
 *  filehandle to it (including your program) close their handles.
 *
 * Chances are, the bits that make up the file still exist, they are just
 *  made available to be written over at a later point. Don't consider this
 *  a security method or anything.  :)
 *
 *   \param filename Filename to delete.
 *  \return nonzero on success, zero on error. Use PHYSFS_getLastErrorCode()
 *          to obtain the specific error.
 */
    #[no_mangle]
    fn PHYSFS_delete(filename: *const libc::c_char) -> libc::c_int;
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
    #[no_mangle]
    fn mouseY() -> SDWORD;
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Create an empty widget screen */
    #[no_mangle]
    fn widgCreateScreen(ppsScreen: *mut *mut W_SCREEN) -> BOOL;
    /* Release a screen and all its associated data */
    #[no_mangle]
    fn widgReleaseScreen(psScreen: *mut W_SCREEN);
    /* Set the tool tip font for a screen */
    #[no_mangle]
    fn widgSetTipFont(psScreen: *mut W_SCREEN, FontID: libc::c_int);
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Add an edit box to a form */
    #[no_mangle]
    fn widgAddEditBox(psScreen: *mut W_SCREEN, psInit: *mut W_EDBINIT)
     -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Hide a widget */
    #[no_mangle]
    fn widgHide(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Reveal a widget */
    #[no_mangle]
    fn widgReveal(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
    #[no_mangle]
    fn widgRunScreen(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
    #[no_mangle]
    fn widgDisplayScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn pie_UploadDisplayBuffer(DisplayBuffer_0: *mut libc::c_char);
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    /* Add the reticule widgets to the widget screen */
    #[no_mangle]
    fn intAddReticule() -> BOOL;
    #[no_mangle]
    fn intRemoveReticule();
    //displays the Power Bar
    #[no_mangle]
    fn intShowPowerBar();
    //hides the power bar from the display
//extern void intHidePowerBar(void);
    //hides the power bar from the display - regardless of what player requested!
    #[no_mangle]
    fn forceHidePowerBar();
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // Get the campaign number for loadGameInit game
    #[no_mangle]
    fn getCampaign(pGameToLoad: *mut STRING, bSkipCDCheck: *mut BOOL)
     -> UDWORD;
    #[no_mangle]
    fn loadOK();
    #[no_mangle]
    fn CancelPressed() -> BOOL;
    #[no_mangle]
    fn GetGameMode() -> UDWORD;
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    static mut bRender3DOnly: BOOL;
    /* Do the 3D display */
    #[no_mangle]
    fn displayWorld();
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    #[no_mangle]
    fn setGamePauseStatus(val: BOOL);
    #[no_mangle]
    fn setGameUpdatePause(state: BOOL);
    #[no_mangle]
    fn setScriptPause(state: BOOL);
    #[no_mangle]
    fn setScrollPause(state: BOOL);
    #[no_mangle]
    fn setConsolePause(state: BOOL);
    #[no_mangle]
    fn editBoxClicked(psWidget: *mut W_EDITBOX, psContext: *mut W_CONTEXT);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn resetMissionWidgets();
    #[no_mangle]
    fn setCampaignNumber(number: UDWORD);
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
}
/* Copyright (C) 1991-2019 Free Software Foundation, Inc.
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
 *	ISO C99 Standard 7.4: Character handling	<ctype.h>
 */
/* These are all the characteristics of characters.
   If there get to be more than 16 distinct characteristics,
   many things must be changed that use `unsigned short int's.

   The characteristics are stored always in network byte order (big
   endian).  We define the bit value interpretations here dependent on the
   machine's byte order.  */
/* __BYTE_ORDER == __LITTLE_ENDIAN */
pub type C2RustUnnamed = libc::c_uint;
/* Alphanumeric.  */
/* Punctuation.  */
pub const _ISalnum: C2RustUnnamed = 8;
/* Control character.  */
pub const _ISpunct: C2RustUnnamed = 4;
/* Blank (usually SPC and TAB).  */
pub const _IScntrl: C2RustUnnamed = 2;
/* Graphical.  */
pub const _ISblank: C2RustUnnamed = 1;
/* Printing.  */
pub const _ISgraph: C2RustUnnamed = 32768;
/* Whitespace.  */
pub const _ISprint: C2RustUnnamed = 16384;
/* Hexadecimal numeric.  */
pub const _ISspace: C2RustUnnamed = 8192;
/* Numeric.  */
pub const _ISxdigit: C2RustUnnamed = 4096;
/* Alphabetic.  */
pub const _ISdigit: C2RustUnnamed = 2048;
/* lowercase.  */
pub const _ISalpha: C2RustUnnamed = 1024;
/* UPPERCASE.  */
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
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
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
pub type _key_code = libc::c_uint;
pub const KEY_KPENTER: _key_code = 271;
pub const KEY_DELETE: _key_code = 127;
pub const KEY_INSERT: _key_code = 277;
pub const KEY_PAGEDOWN: _key_code = 281;
pub const KEY_DOWNARROW: _key_code = 274;
pub const KEY_END: _key_code = 279;
pub const KEY_RIGHTARROW: _key_code = 275;
pub const KEY_LEFTARROW: _key_code = 276;
pub const KEY_PAGEUP: _key_code = 280;
pub const KEY_UPARROW: _key_code = 273;
pub const KEY_HOME: _key_code = 278;
pub const KEY_RALT: _key_code = 307;
pub const KEY_KP_BACKSLASH: _key_code = 267;
pub const KEY_RCTRL: _key_code = 305;
pub const KEY_F12: _key_code = 293;
pub const KEY_F11: _key_code = 292;
pub const KEY_KP_FULLSTOP: _key_code = 266;
pub const KEY_KP_0: _key_code = 256;
pub const KEY_KP_3: _key_code = 259;
pub const KEY_KP_2: _key_code = 258;
pub const KEY_KP_1: _key_code = 257;
pub const KEY_KP_PLUS: _key_code = 270;
pub const KEY_KP_6: _key_code = 262;
pub const KEY_KP_5: _key_code = 261;
pub const KEY_KP_4: _key_code = 260;
pub const KEY_KP_MINUS: _key_code = 269;
pub const KEY_KP_9: _key_code = 265;
pub const KEY_KP_8: _key_code = 264;
pub const KEY_KP_7: _key_code = 263;
pub const KEY_SCROLLLOCK: _key_code = 302;
pub const KEY_NUMLOCK: _key_code = 300;
pub const KEY_F10: _key_code = 291;
pub const KEY_F9: _key_code = 290;
pub const KEY_F8: _key_code = 289;
pub const KEY_F7: _key_code = 288;
pub const KEY_F6: _key_code = 287;
pub const KEY_F5: _key_code = 286;
pub const KEY_F4: _key_code = 285;
pub const KEY_F3: _key_code = 284;
pub const KEY_F2: _key_code = 283;
pub const KEY_F1: _key_code = 282;
pub const KEY_CAPSLOCK: _key_code = 301;
pub const KEY_SPACE: _key_code = 32;
pub const KEY_LALT: _key_code = 308;
pub const KEY_KP_STAR: _key_code = 268;
pub const KEY_RSHIFT: _key_code = 303;
pub const KEY_FORWARDSLASH: _key_code = 47;
pub const KEY_FULLSTOP: _key_code = 46;
pub const KEY_COMMA: _key_code = 44;
pub const KEY_M: _key_code = 109;
pub const KEY_N: _key_code = 110;
pub const KEY_B: _key_code = 98;
pub const KEY_V: _key_code = 118;
pub const KEY_C: _key_code = 99;
pub const KEY_X: _key_code = 120;
pub const KEY_Z: _key_code = 122;
pub const KEY_BACKSLASH: _key_code = 92;
pub const KEY_LSHIFT: _key_code = 304;
pub const KEY_BACKQUOTE: _key_code = 96;
pub const KEY_QUOTE: _key_code = 39;
pub const KEY_SEMICOLON: _key_code = 59;
pub const KEY_L: _key_code = 108;
pub const KEY_K: _key_code = 107;
pub const KEY_J: _key_code = 106;
pub const KEY_H: _key_code = 104;
pub const KEY_G: _key_code = 103;
pub const KEY_F: _key_code = 102;
pub const KEY_D: _key_code = 100;
pub const KEY_S: _key_code = 115;
pub const KEY_A: _key_code = 97;
pub const KEY_LCTRL: _key_code = 306;
pub const KEY_RETURN: _key_code = 13;
pub const KEY_RBRACE: _key_code = 93;
pub const KEY_LBRACE: _key_code = 91;
pub const KEY_P: _key_code = 112;
pub const KEY_O: _key_code = 111;
pub const KEY_I: _key_code = 105;
pub const KEY_U: _key_code = 117;
pub const KEY_Y: _key_code = 121;
pub const KEY_T: _key_code = 116;
pub const KEY_R: _key_code = 114;
pub const KEY_E: _key_code = 101;
pub const KEY_W: _key_code = 119;
pub const KEY_Q: _key_code = 113;
pub const KEY_TAB: _key_code = 9;
pub const KEY_BACKSPACE: _key_code = 8;
pub const KEY_EQUALS: _key_code = 61;
pub const KEY_MINUS: _key_code = 45;
pub const KEY_0: _key_code = 48;
pub const KEY_9: _key_code = 57;
pub const KEY_8: _key_code = 56;
pub const KEY_7: _key_code = 55;
pub const KEY_6: _key_code = 54;
pub const KEY_5: _key_code = 53;
pub const KEY_4: _key_code = 52;
pub const KEY_3: _key_code = 51;
pub const KEY_2: _key_code = 50;
pub const KEY_1: _key_code = 49;
pub const KEY_ESC: _key_code = 27;
pub type KEY_CODE = _key_code;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
pub type OBJ_HEAP = _obj_heap;
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
pub type TREAP_NODE = _treap_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
/* A String Resource */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
pub type STR_RES = _str_res;
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
// The next free ID
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
pub type WIDGET_TYPE = _widget_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
}
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
/* The optional user callback function */
pub type WIDGET_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context) -> ()>;
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_context {
    pub psScreen: *mut W_SCREEN,
    pub psForm: *mut _w_form,
    pub xOffset: SDWORD,
    pub yOffset: SDWORD,
    pub mx: SDWORD,
    pub my: SDWORD,
}
/* The basic form data */
/* The common widget data */
/* Disable all child widgets if TRUE */
/* Working coords for animations. */
/* Animation counter. */
/* Animation start time */
/* Colours for the form and its widgets. signed since aColours -1 means use bitmap. */
/* The last widget to be hilited */
/* This is used to track when the mouse moves */
/* off something */
/* The widgets on the form */
/* The standard form */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_form {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
}
/* The common widget data */
/* ID of the widgets base form. */
/* The user set ID number for the widget */
/* This is returned when e.g. a button is pressed */
/* The widget type */
/* The style of the widget */
/* The location of the widget */
/* The size of the widget */
/* Display the widget */
/* User callback (if any) */
/* Pointer to a user data block (if any) */
/* User data (if any) */
/* Pointer to the next widget in the screen list */
/* The base widget data type */
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
/* The common widget data */
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type WIDGET_AUDIOCALLBACK
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Flags for controlling where the tabs appear on a form -
 * used in the majorPos and minorPos entries of the W_FORMINIT struct
 */
// No tab
/* Upper limits for major and minor tabs on a tab form.
 * Not the best way to do it I know, but it keeps the memory
 * management MUCH simpler.
 */
//15		// Maximum number of major tabs on a tab form
//15		// Maximum number of minor tabs off a major
// Tab types passed into tab display callbacks.
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
pub type FONT_DISPLAY
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut STRING) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_forminit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub disableChildren: BOOL,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub numMajor: UWORD,
    pub aNumMinors: [UWORD; 9],
    pub pTip: *mut STRING,
    pub apMajorTips: [*mut STRING; 9],
    pub apMinorTips: [[*mut STRING; 5]; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
/* Form initialisation structure */
pub type W_FORMINIT = _w_forminit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_labinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
/* The basic init entries */
/* Data for a tabbed form */
// Position of the tabs on the form
// Size of the tabs (in pixels)
// Tab start offset.
// Tab form overlap offset.
// Tab form overlap offset.
// The thickness of the tabs
// The thickness of the tabs
// The space between tabs
// The space between tabs
// Number of major tabs
// Number of minor tabs for each major
// Tool tip for the form itself
// Tool tips for the major tabs
// Tool tips for the minor tabs
// Optional callback for displaying a tab.
// Optional callback to display the form.
/* Label initialisation structure */
pub type W_LABINIT = _w_labinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_butinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
/* The basic init entries */
// label text
// Tool tip for the label.
//	PROP_FONT	*psFont;		// label font
// ID of the IVIS font to use for this widget.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_edbinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub FontID: libc::c_int,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
}
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* Edit box initialisation structure */
pub type W_EDBINIT = _w_edbinit;
pub type uint8 = libc::c_uchar;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
}
pub type _loadsave_mode = libc::c_uint;
pub const SAVE_FORCE: _loadsave_mode = 6;
pub const LOAD_FORCE: _loadsave_mode = 5;
pub const SAVE_INGAME: _loadsave_mode = 4;
pub const LOAD_INGAME: _loadsave_mode = 3;
pub const SAVE_MISSIONEND: _loadsave_mode = 2;
pub const LOAD_MISSIONEND: _loadsave_mode = 1;
pub const LOAD_FRONTEND: _loadsave_mode = 0;
/* The basic init entries */
// initial contents of the edit box
//	PROP_FONT	*psFont;		// edit box font
// ID of the IVIS font to use for this widget.
// Optional callback to display the form.
// Optional callback to display a string.
/* **************************************************************************/
/* 
 * loadsave.h
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
pub type LOADSAVE_MODE = _loadsave_mode;
/*
 * Button.h
 *
 * Definitions for edit box functions.
 */
/* The widget heap */
/* Button states */
// Button is down
// Button is disabled
// Button is hilited
// Button is locked down
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
pub type W_BUTTON = _w_button;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_button {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub state: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
    pub FontID: libc::c_int,
}
/* The common widget data */
// The current button state
// The text for the button
// The tool tip for the button
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
//	PROP_FONT	*psFont;			// button font
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const IMAGE_NRUTER: C2RustUnnamed_1 = 319;
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
pub type W_EDITBOX = _w_editbox;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_editbox {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
    pub state: UDWORD,
    pub aText: [STRING; 80],
    pub FontID: libc::c_int,
    pub insPos: UWORD,
    pub printStart: UWORD,
    pub printChars: UWORD,
    pub printWidth: UWORD,
    pub pBoxDisplay: WIDGET_DISPLAY,
    pub pFontDisplay: FONT_DISPLAY,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_0 = 6;
pub type W_CONTEXT = _w_context;
pub type W_FORM = _w_form;
pub type C2RustUnnamed_0 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_0 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_0 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_0 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_0 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_0 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_0 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_0 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_0 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_0 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_0 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_0 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_0 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_0 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_0 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_0 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_0 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_0 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_0 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_0 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_0 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_0 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_0 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_0 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_0 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_0 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_0 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_0 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_0 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_0 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_0 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_0 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_0 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_0 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_0 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_0 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_0 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_0 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_0 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_0 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_0 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_0 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_0 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_0 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_0 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_0 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_0 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_0 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_0 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_0 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_0 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_0 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_0 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_0 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_0 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_0 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_0 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_0 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_0 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_0 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_0 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_0 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_0 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_0 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_0 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_0 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_0 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_0 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_0 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_0 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_0 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_0 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_0 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_0 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_0 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_0 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_0 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_0 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_0 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_0 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_0 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_0 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_0 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_0 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_0 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_0 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_0 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_0 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_0 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_0 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_0 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_0 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_0 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_0 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_0 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_0 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_0 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_0 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_0 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_0 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_0 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_0 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_0 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_0 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_0 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_0 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_0 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_0 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_0 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_0 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_0 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_0 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_0 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_0 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_0 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_0 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_0 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_0 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_0 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_0 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_0 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_0 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_0 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_0 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_0 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_0 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_0 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_0 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_0 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_0 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_0 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_0 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_0 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_0 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_0 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_0 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_0 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_0 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_0 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_0 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_0 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_0 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_0 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_0 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_0 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_0 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_0 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_0 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_0 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_0 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_0 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_0 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_0 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_0 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_0 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_0 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_0 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_0 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_0 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_0 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_0 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_0 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_0 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_0 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_0 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_0 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_0 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_0 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_0 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_0 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_0 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_0 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_0 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_0 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_0 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_0 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_0 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_0 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_0 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_0 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_0 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_0 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_0 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_0 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_0 = 160;
pub const ID_GIFT: C2RustUnnamed_0 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_0 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_0 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_0 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_0 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_0 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_0 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_0 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_0 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_0 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_0 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_0 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_0 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_0 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_0 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_0 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_0 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_0 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_0 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_0 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_0 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_0 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_0 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_0 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_0 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_0 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_0 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_0 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_0 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_0 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_0 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_0 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_0 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_0 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_0 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_0 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_0 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_0 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_0 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_0 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_0 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_0 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_0 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_0 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_0 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_0 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_0 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_0 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_0 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_0 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_0 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_0 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_0 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_0 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_0 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_0 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_0 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_0 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_0 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_0 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_0 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_0 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_0 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_0 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_0 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_0 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_0 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_0 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_0 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_0 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_0 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_0 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_0 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_0 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_0 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_0 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_0 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_0 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_0 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_0 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_0 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_0 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_0 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_0 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_0 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_0 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_0 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_0 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_0 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_0 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_0 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_0 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_0 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_0 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_0 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_0 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_0 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_0 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_0 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_0 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_0 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_0 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_0 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_0 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_0 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_0 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_0 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_0 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_0 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_0 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_0 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_0 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_0 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_0 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_0 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_0 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_0 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_0 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_0 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_0 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_0 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_0 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_0 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_0 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_0 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_0 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_0 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_0 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_0 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_0 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_0 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_0 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_0 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_0 = 7;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_0 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_0 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_0 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_0 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_0 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_0 = 0;
pub const NO_SOUND: C2RustUnnamed_0 = -1;
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_1 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_1 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_1 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_1 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_1 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_1 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_1 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_1 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_1 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_1 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_1 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_1 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_1 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_1 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_1 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_1 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_1 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_1 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_1 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_1 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_1 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_1 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_1 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_1 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_1 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_1 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_1 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_1 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_1 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_1 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_1 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_1 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_1 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_1 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_1 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_1 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_1 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_1 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_1 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_1 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_1 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_1 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_1 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_1 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_1 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_1 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_1 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_1 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_1 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_1 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_1 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_1 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_1 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_1 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_1 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_1 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_1 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_1 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_1 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_1 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_1 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_1 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_1 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_1 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_1 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_1 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_1 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_1 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_1 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_1 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_1 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_1 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_1 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_1 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_1 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_1 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_1 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_1 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_1 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_1 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_1 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_1 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_1 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_1 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_1 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_1 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_1 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_1 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_1 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_1 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_1 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_1 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_1 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_1 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_1 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_1 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_1 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_1 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_1 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_1 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_1 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_1 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_1 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_1 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_1 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_1 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_1 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_1 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_1 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_1 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_1 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_1 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_1 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_1 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_1 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_1 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_1 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_1 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_1 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_1 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_1 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_1 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_1 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_1 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_1 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_1 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_1 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_1 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_1 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_1 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_1 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_1 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_1 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_1 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_1 = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_1 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_1 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_1 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_1 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_1 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_1 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_1 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_1 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_1 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_1 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_1 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_1 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_1 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_1 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_1 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_1 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_1 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_1 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_1 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_1 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_1 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_1 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_1 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_1 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_1 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_1 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_1 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_1 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_1 = 320;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_1 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_1 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_1 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_1 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_1 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_1 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_1 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_1 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_1 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_1 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_1 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_1 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_1 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_1 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_1 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_1 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_1 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_1 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_1 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_1 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_1 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_1 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_1 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_1 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_1 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_1 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_1 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_1 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_1 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_1 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_1 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_1 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_1 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_1 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_1 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_1 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_1 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_1 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_1 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_1 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_1 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_1 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_1 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_1 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_1 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_1 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_1 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_1 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_1 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_1 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_1 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_1 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_1 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_1 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_1 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_1 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_1 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_1 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_1 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_1 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_1 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_1 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_1 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_1 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_1 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_1 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_1 = 252;
pub const IMAGE_STAR: C2RustUnnamed_1 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_1 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_1 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_1 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_1 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_1 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_1 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_1 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_1 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_1 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_1 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_1 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_1 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_1 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_1 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_1 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_1 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_1 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_1 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_1 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_1 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_1 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_1 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_1 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_1 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_1 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_1 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_1 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_1 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_1 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_1 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_1 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_1 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_1 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_1 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_1 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_1 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_1 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_1 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_1 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_1 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_1 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_1 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_1 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_1 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_1 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_1 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_1 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_1 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_1 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_1 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_1 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_1 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_1 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_1 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_1 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_1 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_1 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_1 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_1 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_1 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_1 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_1 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_1 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_1 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_1 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_1 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_1 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_1 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_1 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_1 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_1 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_1 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_1 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_1 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_1 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_1 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_1 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_1 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_1 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_1 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_1 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_1 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_1 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_1 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_1 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_1 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_1 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_1 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_1 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_1 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_1 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_1 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_1 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_1 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_1 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_1 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_1 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_1 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_1 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_1 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_1 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_1 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_1 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_1 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_1 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_1 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_1 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_1 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_1 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_1 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_1 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_1 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_1 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_1 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_1 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_1 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_1 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_1 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_1 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_1 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_1 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_1 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_1 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_1 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_1 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_1 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_1 = 124;
pub const IMAGE_9: C2RustUnnamed_1 = 123;
pub const IMAGE_8: C2RustUnnamed_1 = 122;
pub const IMAGE_7: C2RustUnnamed_1 = 121;
pub const IMAGE_6: C2RustUnnamed_1 = 120;
pub const IMAGE_5: C2RustUnnamed_1 = 119;
pub const IMAGE_4: C2RustUnnamed_1 = 118;
pub const IMAGE_3: C2RustUnnamed_1 = 117;
pub const IMAGE_2: C2RustUnnamed_1 = 116;
pub const IMAGE_1: C2RustUnnamed_1 = 115;
pub const IMAGE_0: C2RustUnnamed_1 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_1 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_1 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_1 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_1 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_1 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_1 = 108;
pub const IMAGE_ECM: C2RustUnnamed_1 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_1 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_1 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_1 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_1 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_1 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_1 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_1 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_1 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_1 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_1 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_1 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_1 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_1 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_1 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_1 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_1 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_1 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_1 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_1 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_1 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_1 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_1 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_1 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_1 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_1 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_1 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_1 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_1 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_1 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_1 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_1 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_1 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_1 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_1 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_1 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_1 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_1 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_1 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_1 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_1 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_1 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_1 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_1 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_1 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_1 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_1 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_1 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_1 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_1 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_1 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_1 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_1 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_1 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_1 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_1 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_1 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_1 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_1 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_1 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_1 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_1 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_1 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_1 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_1 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_1 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_1 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_1 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_1 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_1 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_1 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_1 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_1 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_1 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_1 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_1 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_1 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_1 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_1 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_1 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_1 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_1 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_1 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_1 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_1 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_1 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_1 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_1 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_1 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_1 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_1 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_1 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_1 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_1 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_1 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_1 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_1 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_1 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_1 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_1 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_1 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_1 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_1 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_1 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_1 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_1 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_1 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_1 = 0;
/*
 * Text.h (was Strings.h)
 *
 * String management functions
 *
 */
//the two defines below are MUTUALLY EXCLUSIVE! don't have both defined...
//#define RESOURCE_NAMES
/* ID numbers for all the fixed strings */
pub type _fixed_str_id = libc::c_uint;
pub const STR_MAX_ID: _fixed_str_id = 442;
pub const STR_SEQ_MINIMAL: _fixed_str_id = 441;
pub const STR_SEQ_WINDOW: _fixed_str_id = 440;
pub const STR_SEQ_FULL: _fixed_str_id = 439;
pub const STR_SEQ_PLAYBACK: _fixed_str_id = 438;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_BIND_NORMAL_SPEED: _fixed_str_id = 424;
pub const STR_BIND_SLOW_DOWN: _fixed_str_id = 423;
pub const STR_BIND_SPEED_UP: _fixed_str_id = 422;
pub const STR_BIND_LEFT: _fixed_str_id = 421;
pub const STR_BIND_RIGHT: _fixed_str_id = 420;
pub const STR_BIND_DOWN: _fixed_str_id = 419;
pub const STR_BIND_UP: _fixed_str_id = 418;
pub const STR_BIND_CONSOLE: _fixed_str_id = 417;
pub const STR_BIND_SELCYBORG: _fixed_str_id = 416;
pub const STR_BIND_SELPOWER: _fixed_str_id = 415;
pub const STR_BIND_SELRESEARCH: _fixed_str_id = 414;
pub const STR_BIND_SELFACTORY: _fixed_str_id = 413;
pub const STR_BIND_CMD9: _fixed_str_id = 412;
pub const STR_BIND_CMD8: _fixed_str_id = 411;
pub const STR_BIND_CMD7: _fixed_str_id = 410;
pub const STR_BIND_CMD6: _fixed_str_id = 409;
pub const STR_BIND_CMD5: _fixed_str_id = 408;
pub const STR_BIND_CMD4: _fixed_str_id = 407;
pub const STR_BIND_CMD3: _fixed_str_id = 406;
pub const STR_BIND_CMD2: _fixed_str_id = 405;
pub const STR_BIND_CMD1: _fixed_str_id = 404;
pub const STR_GAM_COMNOTFOUND: _fixed_str_id = 403;
pub const STR_GAM_SENNOTFOUND: _fixed_str_id = 402;
pub const STR_GAM_CONNOTFOUND: _fixed_str_id = 401;
pub const STR_BIND_COMJ: _fixed_str_id = 400;
pub const STR_BIND_SENJ: _fixed_str_id = 399;
// number of string ID's
//added by alex 26-01-99
pub const STR_BIND_CONJ: _fixed_str_id = 398;
pub const STR_FE_CYAN: _fixed_str_id = 397;
pub const STR_FE_PINK: _fixed_str_id = 396;
pub const STR_FE_BLUE: _fixed_str_id = 395;
pub const STR_FE_RED: _fixed_str_id = 394;
pub const STR_FE_BLACK: _fixed_str_id = 393;
pub const STR_FE_GREY: _fixed_str_id = 392;
pub const STR_FE_ORANGE: _fixed_str_id = 391;
pub const STR_FE_GREEN: _fixed_str_id = 390;
pub const STR_FE_OFF: _fixed_str_id = 389;
pub const STR_FE_ON: _fixed_str_id = 388;
pub const STR_FE_MFLIP: _fixed_str_id = 387;
pub const STR_FE_SSHAKE: _fixed_str_id = 386;
pub const STR_SEL_NO_STRUCTURE: _fixed_str_id = 385;
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_BIND_ASIMIL: _fixed_str_id = 383;
pub const STR_BIND_AWHE: _fixed_str_id = 382;
pub const STR_BIND_AVTOL: _fixed_str_id = 381;
pub const STR_BIND_ALL: _fixed_str_id = 380;
pub const STR_BIND_ATR: _fixed_str_id = 379;
pub const STR_BIND_ASCR: _fixed_str_id = 378;
pub const STR_BIND_RECY: _fixed_str_id = 377;
pub const STR_BIND_AHOV: _fixed_str_id = 376;
pub const STR_BIND_AHTR: _fixed_str_id = 375;
pub const STR_BIND_ABDU: _fixed_str_id = 374;
pub const STR_BIND_ACU: _fixed_str_id = 373;
pub const STR_BIND_NDAM: _fixed_str_id = 372;
pub const STR_BIND_HDAM: _fixed_str_id = 371;
pub const STR_BIND_LDAM: _fixed_str_id = 370;
pub const STR_BIND_SCAT: _fixed_str_id = 369;
pub const STR_BIND_LONGR: _fixed_str_id = 368;
pub const STR_BIND_SENDT: _fixed_str_id = 367;
pub const STR_BIND_DSTOP: _fixed_str_id = 366;
pub const STR_BIND_REPA: _fixed_str_id = 365;
pub const STR_BIND_PATR: _fixed_str_id = 364;
pub const STR_BIND_PURS: _fixed_str_id = 363;
pub const STR_BIND_SHOR: _fixed_str_id = 362;
pub const STR_BIND_SPLIM: _fixed_str_id = 361;
pub const STR_BIND_DEFR: _fixed_str_id = 360;
pub const STR_BIND_RTB: _fixed_str_id = 359;
pub const STR_BIND_FAW: _fixed_str_id = 358;
pub const STR_BIND_ENGAG: _fixed_str_id = 357;
pub const STR_BIND_UNITJ: _fixed_str_id = 356;
pub const STR_BIND_CEASE: _fixed_str_id = 355;
pub const STR_BIND_CENTV: _fixed_str_id = 354;
pub const STR_BIND_OVERL: _fixed_str_id = 353;
pub const STR_BIND_REPJ: _fixed_str_id = 352;
pub const STR_BIND_RESJ: _fixed_str_id = 351;
pub const STR_BIND_ORD: _fixed_str_id = 350;
pub const STR_BIND_PB: _fixed_str_id = 349;
pub const STR_BIND_RR: _fixed_str_id = 348;
pub const STR_BIND_RP: _fixed_str_id = 347;
pub const STR_BIND_RL: _fixed_str_id = 346;
pub const STR_BIND_PF: _fixed_str_id = 345;
pub const STR_BIND_ZOUT: _fixed_str_id = 344;
pub const STR_BIND_ZIN: _fixed_str_id = 343;
pub const STR_BIND_ROUT: _fixed_str_id = 342;
pub const STR_BIND_RIN: _fixed_str_id = 341;
pub const STR_BIND_OPT: _fixed_str_id = 340;
pub const STR_BIND_TRACK: _fixed_str_id = 339;
pub const STR_BIND_NORTH: _fixed_str_id = 338;
pub const STR_BIND_AUDOFF: _fixed_str_id = 337;
pub const STR_BIND_AUDON: _fixed_str_id = 336;
pub const STR_BIND_MULOP: _fixed_str_id = 335;
pub const STR_BIND_GR9: _fixed_str_id = 334;
pub const STR_BIND_GR8: _fixed_str_id = 333;
pub const STR_BIND_GR7: _fixed_str_id = 332;
pub const STR_BIND_GR6: _fixed_str_id = 331;
pub const STR_BIND_GR5: _fixed_str_id = 330;
pub const STR_BIND_GR4: _fixed_str_id = 329;
pub const STR_BIND_GR3: _fixed_str_id = 328;
pub const STR_BIND_GR2: _fixed_str_id = 327;
pub const STR_BIND_GR1: _fixed_str_id = 326;
pub const STR_BIND_AS9: _fixed_str_id = 325;
pub const STR_BIND_AS8: _fixed_str_id = 324;
pub const STR_BIND_AS7: _fixed_str_id = 323;
pub const STR_BIND_AS6: _fixed_str_id = 322;
pub const STR_BIND_AS5: _fixed_str_id = 321;
pub const STR_BIND_AS4: _fixed_str_id = 320;
pub const STR_BIND_AS3: _fixed_str_id = 319;
pub const STR_BIND_AS2: _fixed_str_id = 318;
pub const STR_BIND_AS1: _fixed_str_id = 317;
pub const STR_BIND_PREV: _fixed_str_id = 316;
pub const STR_BIND_PAUSE: _fixed_str_id = 315;
pub const STR_BIND_SHOT: _fixed_str_id = 314;
pub const STR_BIND_BARS: _fixed_str_id = 313;
pub const STR_BIND_TOGCON: _fixed_str_id = 312;
pub const STR_BIND_TOGRAD: _fixed_str_id = 311;
pub const STR_BIND_CHOCOM: _fixed_str_id = 310;
pub const STR_BIND_CHOINT: _fixed_str_id = 309;
pub const STR_BIND_CHODES: _fixed_str_id = 308;
pub const STR_BIND_CHOBUI: _fixed_str_id = 307;
pub const STR_BIND_CHORES: _fixed_str_id = 306;
pub const STR_BIND_CHOMAN: _fixed_str_id = 305;
pub const STR_KM_KEYMAP_SIDE: _fixed_str_id = 304;
// keymap editor.
pub const STR_KM_KEYMAP: _fixed_str_id = 303;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
pub const STR_GAME_RESTART: _fixed_str_id = 301;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
pub const STR_GAME_LOADED: _fixed_str_id = 299;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
pub const STR_GAME_REPLAY: _fixed_str_id = 297;
pub const STR_CD_CHANGE_1OR2: _fixed_str_id = 296;
pub const STR_CD_CHANGE: _fixed_str_id = 295;
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const STR_MR_LEVEL_ACE: _fixed_str_id = 284;
pub const STR_MR_LEVEL_SPECIAL: _fixed_str_id = 283;
pub const STR_MR_LEVEL_ELITE: _fixed_str_id = 282;
pub const STR_MR_LEVEL_CRACK: _fixed_str_id = 281;
pub const STR_MR_LEVEL_VETERAN: _fixed_str_id = 280;
pub const STR_MR_LEVEL_PROF: _fixed_str_id = 279;
pub const STR_MR_LEVEL_REGULAR: _fixed_str_id = 278;
pub const STR_MR_LEVEL_TRAINED: _fixed_str_id = 277;
pub const STR_MR_LEVEL_GREEN: _fixed_str_id = 276;
pub const STR_MR_LEVEL_ROOKIE: _fixed_str_id = 275;
pub const STR_MR_BABAS_RUN_OVER: _fixed_str_id = 274;
pub const STR_MR_SHOTS_OFF_TARGET: _fixed_str_id = 273;
pub const STR_MR_SHOTS_ON_TARGET: _fixed_str_id = 272;
pub const STR_MR_GAME_TIME: _fixed_str_id = 271;
pub const STR_MR_MISSION_TIME: _fixed_str_id = 270;
pub const STR_MR_ARTEFACTS_FOUND: _fixed_str_id = 269;
pub const STR_MR_STR_NOW: _fixed_str_id = 268;
pub const STR_MR_STR_LOST: _fixed_str_id = 267;
pub const STR_MR_STR_BLOWN_UP: _fixed_str_id = 266;
pub const STR_MR_STR_BUILT: _fixed_str_id = 265;
pub const STR_MR_AV_UNIT_EL: _fixed_str_id = 264;
pub const STR_MR_UNITS_NOW: _fixed_str_id = 263;
pub const STR_MR_UNITS_LOST: _fixed_str_id = 262;
pub const STR_MR_UNITS_KILLED: _fixed_str_id = 261;
pub const STR_MR_UNITS_BUILT: _fixed_str_id = 260;
pub const STR_MR_RANKINGS: _fixed_str_id = 259;
pub const STR_MR_FORCE_INFO: _fixed_str_id = 258;
pub const STR_MR_STRUCTURE_LOSSES: _fixed_str_id = 257;
pub const STR_MR_UNIT_LOSSES: _fixed_str_id = 256;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GAM_REPNOTFOUND: _fixed_str_id = 245;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const STR_DORD_FACTORY: _fixed_str_id = 219;
pub const STR_DORD_RECYCLE: _fixed_str_id = 218;
pub const STR_DORD_ARMRECYCLE: _fixed_str_id = 217;
pub const STR_DORD_EMBARK: _fixed_str_id = 216;
pub const STR_DORD_RETBASE: _fixed_str_id = 215;
pub const STR_DORD_RETREPAIR: _fixed_str_id = 214;
pub const STR_DORD_HOLDPOS: _fixed_str_id = 213;
pub const STR_DORD_GUARD: _fixed_str_id = 212;
pub const STR_DORD_PERSUE: _fixed_str_id = 211;
pub const STR_DORD_PATROL: _fixed_str_id = 210;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_MUL_RESPOND: _fixed_str_id = 200;
pub const STR_MUL_JOINING: _fixed_str_id = 199;
pub const STR_MUL_LEAVE: _fixed_str_id = 198;
pub const STR_HARD: _fixed_str_id = 197;
pub const STR_NORMAL: _fixed_str_id = 196;
pub const STR_EASY: _fixed_str_id = 195;
pub const STR_FE_DIFFICULTY: _fixed_str_id = 194;
pub const STR_FE_TRANSPARENCY: _fixed_str_id = 193;
pub const STR_FE_FOG: _fixed_str_id = 192;
pub const STR_FE_EFFECTS: _fixed_str_id = 191;
pub const STR_FE_TEXTURE: _fixed_str_id = 190;
pub const STR_FE_AUDIO: _fixed_str_id = 189;
pub const STR_FE_GRAPHICS: _fixed_str_id = 188;
pub const STR_FE_GAME: _fixed_str_id = 187;
pub const STR_FE_GLIDE: _fixed_str_id = 186;
pub const STR_FE_OPENGL: _fixed_str_id = 185;
pub const STR_FE_DIRECTX: _fixed_str_id = 184;
pub const STR_FE_SOFTWARE: _fixed_str_id = 183;
pub const STR_FE_VIDEO: _fixed_str_id = 182;
pub const STR_FE_GOODFOG: _fixed_str_id = 181;
pub const STR_FE_CRAPFOG: _fixed_str_id = 180;
pub const STR_FE_CLAN: _fixed_str_id = 179;
pub const STR_FE_MUSIC: _fixed_str_id = 178;
pub const STR_FE_3D_FX: _fixed_str_id = 177;
pub const STR_FE_FX: _fixed_str_id = 176;
pub const STR_FE_GAMMA: _fixed_str_id = 175;
pub const STR_FE_SCROLL: _fixed_str_id = 174;
pub const STR_FE_MOUSE: _fixed_str_id = 173;
pub const STR_FE_SIDEOPTIONS: _fixed_str_id = 172;
pub const STR_FE_SKIRMISH: _fixed_str_id = 171;
pub const STR_FE_FORCEEDIT: _fixed_str_id = 170;
pub const STR_FE_JOIN: _fixed_str_id = 169;
pub const STR_FE_HOST: _fixed_str_id = 168;
pub const STR_FE_SIDEMULTI: _fixed_str_id = 167;
pub const STR_FE_RETURN: _fixed_str_id = 166;
pub const STR_FE_FASTPLAY: _fixed_str_id = 165;
pub const STR_FE_TUT: _fixed_str_id = 164;
pub const STR_FE_LOAD: _fixed_str_id = 163;
pub const STR_FE_NEW: _fixed_str_id = 162;
pub const STR_FE_SIDETUT: _fixed_str_id = 161;
pub const STR_FE_SIDESINGLE2: _fixed_str_id = 160;
pub const STR_FE_SIDESINGLE1: _fixed_str_id = 159;
pub const STR_FE_QUIT: _fixed_str_id = 158;
pub const STR_FE_INTRO: _fixed_str_id = 157;
pub const STR_FE_OPTIONS: _fixed_str_id = 156;
pub const STR_FE_MULTI: _fixed_str_id = 155;
pub const STR_FE_SINGLE: _fixed_str_id = 154;
// FrontEnd STrings
pub const STR_FE_SIDEMAIN: _fixed_str_id = 153;
pub const STR_GAME_RESUME: _fixed_str_id = 152;
//ingame ops.
pub const STR_GAME_QUIT: _fixed_str_id = 151;
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
pub const STR_COMPATIBLE: _fixed_str_id = 148;
pub const STR_MUL_ARTIF: _fixed_str_id = 147;
pub const STR_GIFT_POW: _fixed_str_id = 146;
pub const STR_GIFT_TEC: _fixed_str_id = 145;
pub const STR_GIFT_DRO: _fixed_str_id = 144;
pub const STR_GIFT_VIS: _fixed_str_id = 143;
pub const STR_ALLI_FRM: _fixed_str_id = 142;
pub const STR_ALLI_BRK: _fixed_str_id = 141;
pub const STR_ALLI_OFF: _fixed_str_id = 140;
pub const STR_ALLI_REQ: _fixed_str_id = 139;
pub const STR_ALLI_POW: _fixed_str_id = 138;
pub const STR_ALLI_DRO: _fixed_str_id = 137;
pub const STR_ALLI_TEC: _fixed_str_id = 136;
pub const STR_ALLI_VIS: _fixed_str_id = 135;
pub const STR_ALLI_STATE: _fixed_str_id = 134;
pub const STR_MUL_FOG_OFF: _fixed_str_id = 133;
pub const STR_MUL_FOG_ON: _fixed_str_id = 132;
pub const STR_MUL_TEAMPLAY: _fixed_str_id = 131;
pub const STR_MUL_SKIRMISH: _fixed_str_id = 130;
pub const STR_MUL_STRLIM: _fixed_str_id = 129;
pub const STR_MUL_COMP_N: _fixed_str_id = 128;
pub const STR_MUL_COMP_Y: _fixed_str_id = 127;
pub const STR_MUL_COMP: _fixed_str_id = 126;
pub const STR_MUL_PLAY: _fixed_str_id = 125;
pub const STR_MUL_PLAYERS: _fixed_str_id = 124;
pub const STR_LABEL_FOG: _fixed_str_id = 123;
pub const STR_LABEL_LIMIT: _fixed_str_id = 122;
pub const STR_LABEL_BASE: _fixed_str_id = 121;
pub const STR_LABEL_TEC: _fixed_str_id = 120;
pub const STR_LABEL_ALLI: _fixed_str_id = 119;
pub const STR_LABEL_TYPE: _fixed_str_id = 118;
pub const STR_GAMES_GAMES: _fixed_str_id = 117;
pub const STR_CON_MORE: _fixed_str_id = 116;
pub const STR_CON_CABLE: _fixed_str_id = 115;
pub const STR_CON_LAN: _fixed_str_id = 114;
pub const STR_CON_INTERNET: _fixed_str_id = 113;
pub const STR_CON_MODEM: _fixed_str_id = 112;
pub const STR_MUL_FRAGLIM: _fixed_str_id = 111;
pub const STR_MUL_TIMELIM: _fixed_str_id = 110;
pub const STR_MUL_NOLIM: _fixed_str_id = 109;
pub const STR_MUL_ALLIANCEY: _fixed_str_id = 108;
pub const STR_MUL_ALLIANCEN: _fixed_str_id = 107;
pub const STR_MUL_GAMEIC: _fixed_str_id = 106;
pub const STR_MUL_MAPIC: _fixed_str_id = 105;
pub const STR_MUL_FORCEIC: _fixed_str_id = 104;
pub const STR_MUL_PLAYERIC: _fixed_str_id = 103;
pub const STR_MUL_CAMPBASE: _fixed_str_id = 102;
pub const STR_MUL_CAMPDEFENCE: _fixed_str_id = 101;
pub const STR_MUL_CAMPCLEAN: _fixed_str_id = 100;
pub const STR_MUL_TECH3: _fixed_str_id = 99;
pub const STR_MUL_TECH2: _fixed_str_id = 98;
pub const STR_MUL_TECH1: _fixed_str_id = 97;
pub const STR_MUL_POWHI: _fixed_str_id = 96;
pub const STR_MUL_POWMED: _fixed_str_id = 95;
pub const STR_MUL_POWLO: _fixed_str_id = 94;
pub const STR_MUL_PING: _fixed_str_id = 93;
pub const STR_MUL_KILLS: _fixed_str_id = 92;
pub const STR_MUL_SCORE: _fixed_str_id = 91;
pub const STR_MUL_ALLIANCES: _fixed_str_id = 90;
pub const STR_MUL_STARTING: _fixed_str_id = 89;
pub const STR_MUL_CHAT: _fixed_str_id = 88;
pub const STR_MUL_SIDEINFO: _fixed_str_id = 87;
pub const STR_MUL_SIDETEMPLATES: _fixed_str_id = 86;
pub const STR_MUL_SIDEFORCE: _fixed_str_id = 85;
pub const STR_MUL_SIDEPLAYERS: _fixed_str_id = 84;
pub const STR_MUL_SIDEGAMES: _fixed_str_id = 83;
pub const STR_MUL_SIDEOPTIONS: _fixed_str_id = 82;
pub const STR_MUL_SIDECONNECTION: _fixed_str_id = 81;
pub const STR_MUL_115200: _fixed_str_id = 80;
pub const STR_MUL_56000: _fixed_str_id = 79;
pub const STR_MUL_19200: _fixed_str_id = 78;
pub const STR_MUL_14400: _fixed_str_id = 77;
pub const STR_MUL_SEARCHING: _fixed_str_id = 76;
pub const STR_MUL_DESIGN: _fixed_str_id = 75;
pub const STR_MUL_SAVE: _fixed_str_id = 74;
pub const STR_MUL_LOAD: _fixed_str_id = 73;
pub const STR_MUL_DEFAULT: _fixed_str_id = 72;
pub const STR_MUL_CLEAR: _fixed_str_id = 71;
pub const STR_MUL_HOST: _fixed_str_id = 70;
//	STR_MUL_RESLO,	
//	STR_MUL_RESMED,		
//	STR_MUL_RESHI,		
//	STR_MUL_HELPON,	
//	STR_MUL_HELPOFF,		
pub const STR_MUL_REFRESH: _fixed_str_id = 69;
pub const STR_MUL_CAMPAIGN: _fixed_str_id = 68;
pub const STR_MUL_ARENA: _fixed_str_id = 67;
pub const STR_MUL_MAXPLAY: _fixed_str_id = 66;
pub const STR_MUL_GAME: _fixed_str_id = 65;
pub const STR_MUL_PLAYER: _fixed_str_id = 64;
pub const STR_MUL_OK: _fixed_str_id = 63;
pub const STR_MUL_CANCEL: _fixed_str_id = 62;
pub const STR_MUL_COM4: _fixed_str_id = 61;
pub const STR_MUL_COM3: _fixed_str_id = 60;
pub const STR_MUL_COM2: _fixed_str_id = 59;
pub const STR_MUL_COM1: _fixed_str_id = 58;
pub const STR_MUL_IPADDR: _fixed_str_id = 57;
// multiplayer strings
pub const STR_MUL_PHONENO: _fixed_str_id = 56;
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
// Reticule strings
pub const STR_RET_OPTIONS: _fixed_str_id = 1;
// The default (id == 0) string
pub const STR_DEFAULT: _fixed_str_id = 0;
static mut psRequestScreen: *mut W_SCREEN =
    0 as *const W_SCREEN as *mut W_SCREEN;
// Widget screen for requester
static mut mode: BOOL = 0;
static mut chosenSlotId: UDWORD = 0;
#[no_mangle]
pub static mut bLoadSaveUp: BOOL = 0 as libc::c_int;
// true when interface is up and should be run.
#[no_mangle]
pub static mut saveGameName: [STRING; 256] = [0; 256];
//the name of the save game to load from the front end
#[no_mangle]
pub static mut sRequestResult: [STRING; 255] = [0; 255];
// filename returned;
#[no_mangle]
pub static mut sDelete: [STRING; 256] = [0; 256];
#[no_mangle]
pub static mut bRequestLoad: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bLoadSaveMode: LOADSAVE_MODE = LOAD_FRONTEND;
static mut sPath: [CHAR; 255] = [0; 255];
static mut sExt: [CHAR; 4] = [0; 4];
// return whether the save screen was displayed in the mission results screen
// ////////////////////////////////////////////////////////////////////////////
// return whether the save screen was displayed in the mission results screen
#[no_mangle]
pub unsafe extern "C" fn saveInMissionRes() -> BOOL {
    return (bLoadSaveMode as libc::c_uint ==
                SAVE_MISSIONEND as libc::c_int as libc::c_uint) as
               libc::c_int;
}
// return whether the save screen was displayed in the middle of a mission
// ////////////////////////////////////////////////////////////////////////////
// return whether the save screen was displayed in the middle of a mission
#[no_mangle]
pub unsafe extern "C" fn saveMidMission() -> BOOL {
    return (bLoadSaveMode as libc::c_uint ==
                SAVE_INGAME as libc::c_int as libc::c_uint) as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addLoadSave(mut mode_0: LOADSAVE_MODE,
                                     mut sSearchPath: *mut CHAR,
                                     mut sExtension: *mut CHAR,
                                     mut title: *mut CHAR) -> BOOL {
    let mut bLoad: BOOL = 0;
    bLoadSaveMode = mode_0;
    match mode_0 as libc::c_uint {
        0 | 1 | 3 | 5 => { bLoad = 1 as libc::c_int }
        2 | 4 | 6 | _ => { bLoad = 0 as libc::c_int }
    }
    return _addLoadSave(bLoad, sSearchPath, sExtension, title);
}
//****************************************************************************************
// Load menu/save menu?
//*****************************************************************************************
unsafe extern "C" fn _addLoadSave(mut bLoad: BOOL, mut sSearchPath: *mut CHAR,
                                  mut sExtension: *mut CHAR,
                                  mut title: *mut CHAR) -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut sLabInit: W_LABINIT =
        W_LABINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut slotCount: UDWORD = 0;
    // removed hardcoded values!  change with the defines above! -Q
    static mut sSlots: [[STRING; 64]; 20] =
        [[0; 64]; 20]; // Only do this in main game.
    let mut i: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; // Just display the 3d, no interface
    let mut files: *mut *mut libc::c_char =
        0 as
            *mut *mut libc::c_char; // Upload the current display back buffer into system memory.
    mode = bLoad; // just in case
    debug(LOG_WZ,
          b"_addLoadSave(%d, %s, %s, %s)\x00" as *const u8 as
              *const libc::c_char, bLoad, sSearchPath, sExtension,
          title); // init the screen.
    if bLoadSaveMode as libc::c_uint ==
           LOAD_INGAME as libc::c_int as libc::c_uint ||
           bLoadSaveMode as libc::c_uint ==
               SAVE_INGAME as libc::c_int as libc::c_uint {
        if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
            gameTimeStop();
            if GetGameMode() == 3 as libc::c_int as libc::c_uint {
                let mut radOnScreen: BOOL = radarOnScreen;
                bRender3DOnly = 1 as libc::c_int;
                radarOnScreen = 0 as libc::c_int;
                displayWorld();
                pie_UploadDisplayBuffer(DisplayBuffer);
                radarOnScreen = radOnScreen;
                bRender3DOnly = 0 as libc::c_int
            }
            setGamePauseStatus(1 as libc::c_int);
            setGameUpdatePause(1 as libc::c_int);
            setScriptPause(1 as libc::c_int);
            setScrollPause(1 as libc::c_int);
            setConsolePause(1 as libc::c_int);
        }
        forceHidePowerBar();
        intRemoveReticule();
    }
    PHYSFS_mkdir(sSearchPath);
    widgCreateScreen(&mut psRequestScreen);
    widgSetTipFont(psRequestScreen, WFont);
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); //this adds the blue background, and the "box" behind the buttons -Q
    sFormInit.formID =
        0 as libc::c_int as UDWORD; // hmm..the bottom of the box.... -Q
    sFormInit.id = (21000 as libc::c_int + 1 as libc::c_int) as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (130 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint))
            as SWORD;
    sFormInit.y =
        (30 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SWORD;
    sFormInit.width = 380 as libc::c_int as UWORD;
    sFormInit.height =
        (240 as libc::c_int * 2 as libc::c_int - 46 as libc::c_int) as UWORD;
    sFormInit.disableChildren = 1 as libc::c_int;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psRequestScreen, &mut sFormInit);
    // Add Banner
    sFormInit.formID = (21000 as libc::c_int + 1 as libc::c_int) as UDWORD;
    sFormInit.id = (21000 as libc::c_int + 4 as libc::c_int) as UDWORD;
    sFormInit.x = 9 as libc::c_int as SWORD;
    sFormInit.y = 9 as libc::c_int as SWORD;
    sFormInit.width =
        (380 as libc::c_int - 2 as libc::c_int * 9 as libc::c_int) as UWORD;
    sFormInit.height = 40 as libc::c_int as UWORD;
    sFormInit.disableChildren = 0 as libc::c_int;
    sFormInit.pDisplay =
        Some(displayLoadBanner as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData = bLoad as *mut libc::c_void;
    widgAddForm(psRequestScreen, &mut sFormInit);
    // Add Banner Label
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong); //LOADSAVE_W;
    sLabInit.formID =
        (21000 as libc::c_int + 4 as libc::c_int) as
            UDWORD; //This looks right -Q
    sLabInit.id = (21000 as libc::c_int + 3 as libc::c_int) as UDWORD;
    sLabInit.style = 2 as libc::c_int as UDWORD;
    sLabInit.x = 0 as libc::c_int as SWORD;
    sLabInit.y = 3 as libc::c_int as SWORD;
    sLabInit.width =
        (380 as libc::c_int - 2 as libc::c_int * 9 as libc::c_int) as UWORD;
    sLabInit.height = 40 as libc::c_int as UWORD;
    sLabInit.pText = title;
    sLabInit.FontID = WFont;
    widgAddLabel(psRequestScreen, &mut sLabInit);
    // add cancel.
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = (21000 as libc::c_int + 4 as libc::c_int) as UDWORD;
    sButInit.x = 8 as libc::c_int as SWORD;
    sButInit.y = 8 as libc::c_int as SWORD;
    sButInit.width =
        iV_GetImageWidth(IntImages, IMAGE_NRUTER as libc::c_int as UWORD);
    sButInit.height =
        iV_GetImageHeight(IntImages, IMAGE_NRUTER as libc::c_int as UWORD);
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_NRUTER as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_NRUTER as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    sButInit.id = (21000 as libc::c_int + 2 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddButton(psRequestScreen, &mut sButInit);
    // add slots
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = (21000 as libc::c_int + 1 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.width =
        ((380 as libc::c_int - 3 as libc::c_int * 9 as libc::c_int) /
             2 as libc::c_int) as UWORD;
    sButInit.height =
        ((240 as libc::c_int - 6 as libc::c_int * 9 as libc::c_int -
              (40 as libc::c_int + 9 as libc::c_int)) / 5 as libc::c_int) as
            UWORD;
    sButInit.pDisplay =
        Some(displayLoadSlot as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.FontID = WFont;
    slotCount = 0 as libc::c_int as UDWORD;
    while slotCount < 20 as libc::c_int as libc::c_uint {
        sButInit.id =
            slotCount.wrapping_add(21000 as libc::c_int as
                                       libc::c_uint).wrapping_add(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint);
        if slotCount < (20 as libc::c_int / 2 as libc::c_int) as libc::c_uint
           {
            sButInit.x = 9 as libc::c_int as SWORD;
            sButInit.y =
                ((40 as libc::c_int + 2 as libc::c_int * 9 as libc::c_int) as
                     libc::c_uint).wrapping_add(slotCount.wrapping_mul((9 as
                                                                            libc::c_int
                                                                            +
                                                                            (240
                                                                                 as
                                                                                 libc::c_int
                                                                                 -
                                                                                 6
                                                                                     as
                                                                                     libc::c_int
                                                                                     *
                                                                                     9
                                                                                         as
                                                                                         libc::c_int
                                                                                 -
                                                                                 (40
                                                                                      as
                                                                                      libc::c_int
                                                                                      +
                                                                                      9
                                                                                          as
                                                                                          libc::c_int))
                                                                                /
                                                                                5
                                                                                    as
                                                                                    libc::c_int)
                                                                           as
                                                                           libc::c_uint))
                    as SWORD
        } else {
            sButInit.x =
                (2 as libc::c_int * 9 as libc::c_int +
                     (380 as libc::c_int -
                          3 as libc::c_int * 9 as libc::c_int) /
                         2 as libc::c_int) as SWORD;
            sButInit.y =
                ((40 as libc::c_int + 2 as libc::c_int * 9 as libc::c_int) as
                     libc::c_uint).wrapping_add(slotCount.wrapping_sub((20 as
                                                                            libc::c_int
                                                                            /
                                                                            2
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           libc::c_uint).wrapping_mul((9
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           +
                                                                                                           (240
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                -
                                                                                                                6
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    *
                                                                                                                    9
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                -
                                                                                                                (40
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     +
                                                                                                                     9
                                                                                                                         as
                                                                                                                         libc::c_int))
                                                                                                               /
                                                                                                               5
                                                                                                                   as
                                                                                                                   libc::c_int)
                                                                                                          as
                                                                                                          libc::c_uint))
                    as SWORD
        }
        widgAddButton(psRequestScreen, &mut sButInit);
        slotCount = slotCount.wrapping_add(1)
    }
    // fill slots.
    slotCount = 0 as libc::c_int as UDWORD; // setup locals.
    strcpy(sPath.as_mut_ptr(), sSearchPath);
    strcpy(sExt.as_mut_ptr(), sExtension);
    debug(LOG_WZ,
          b"_addLoadSave: Searching \"%s\" for savegames\x00" as *const u8 as
              *const libc::c_char, sSearchPath);
    // add savegame filenames minus extensions to buttons (up to max 10)
    files = PHYSFS_enumerateFiles(sSearchPath);
    i = files;
    while !(*i).is_null() {
        let mut button: *mut W_BUTTON = 0 as *mut W_BUTTON;
        if !strstr(*i, sExtension).is_null() {
            button =
                widgGetFromID(psRequestScreen,
                              ((21000 as libc::c_int + 10 as libc::c_int) as
                                   libc::c_uint).wrapping_add(slotCount)) as
                    *mut W_BUTTON;
            debug(LOG_WZ,
                  b"_addLoadSave: We found [%s]\x00" as *const u8 as
                      *const libc::c_char, *i);
            /* Set the tip and add the button */
            *(*i).offset(strlen(*i).wrapping_sub(4 as libc::c_int as
                                                     libc::c_uint) as isize) =
                '\u{0}' as i32 as libc::c_char; // remove .gam extension
            strcpy(sSlots[slotCount as usize].as_mut_ptr(), *i); //store it!
            (*button).pTip =
                sSlots[slotCount as usize].as_mut_ptr(); // goto next but...
            (*button).pText = sSlots[slotCount as usize].as_mut_ptr();
            slotCount = slotCount.wrapping_add(1);
            if slotCount == 20 as libc::c_int as libc::c_uint { break ; }
        }
        i = i.offset(1)
    }
    PHYSFS_freeList(files as *mut libc::c_void);
    bLoadSaveUp = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn closeLoadSave() -> BOOL {
    widgDelete(psRequestScreen,
               (21000 as libc::c_int + 1 as libc::c_int) as UDWORD);
    bLoadSaveUp = 0 as libc::c_int;
    if bLoadSaveMode as libc::c_uint ==
           LOAD_INGAME as libc::c_int as libc::c_uint ||
           bLoadSaveMode as libc::c_uint ==
               SAVE_INGAME as libc::c_int as libc::c_uint {
        if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
            gameTimeStart();
            setGamePauseStatus(0 as libc::c_int);
            setGameUpdatePause(0 as libc::c_int);
            setScriptPause(0 as libc::c_int);
            setScrollPause(0 as libc::c_int);
            setConsolePause(0 as libc::c_int);
        }
        intAddReticule();
        intShowPowerBar();
    }
    widgReleaseScreen(psRequestScreen);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn loadSaveCDOK() {
    bRequestLoad = 1 as libc::c_int;
    closeLoadSave();
    loadOK();
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn loadSaveCDCancel() {
    bRequestLoad = 0 as libc::c_int;
    widgReveal(psRequestScreen,
               (21000 as libc::c_int + 1 as libc::c_int) as UDWORD);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn runLoadSave(mut bResetMissionWidgets: BOOL) -> BOOL {
    return _runLoadSave(bResetMissionWidgets);
}
/* **************************************************************************
	Delete a savegame.  saveGameName should be a .gam extension save game
	filename reference.  We delete this file, any .es file with the same
	name, and any files in the directory with the same name.
***************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn deleteSaveGame(mut saveGameName_0:
                                            *mut libc::c_char) {
    let mut files: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; // strip extension
    let mut i: *mut *mut libc::c_char =
        0 as *mut *mut libc::c_char; // remove script data if it exists.
    if strlen(saveGameName_0) < 256 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"deleteSaveGame; save game name too long\x00" as *const u8 as
                  *const libc::c_char); // strip extension
    };
    if strlen(saveGameName_0) < 256 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"loadsave.c\x00" as *const u8 as *const libc::c_char,
              369 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"deleteSaveGame\x00")).as_ptr(),
              b"strlen(saveGameName) < MAX_STR_LENGTH\x00" as *const u8 as
                  *const libc::c_char);
    };
    PHYSFS_delete(saveGameName_0);
    *saveGameName_0.offset(strlen(saveGameName_0).wrapping_sub(4 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                               as isize) = '\u{0}' as i32 as libc::c_char;
    strcat(saveGameName_0, b".es\x00" as *const u8 as *const libc::c_char);
    PHYSFS_delete(saveGameName_0);
    *saveGameName_0.offset(strlen(saveGameName_0).wrapping_sub(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                               as isize) = '\u{0}' as i32 as libc::c_char;
    // check for a directory and remove that too.
    files = PHYSFS_enumerateFiles(saveGameName_0); // now empty directory
    i = files;
    while !(*i).is_null() {
        debug(LOG_WZ,
              b"Deleting [%s].\x00" as *const u8 as *const libc::c_char, *i);
        PHYSFS_delete(*i);
        i = i.offset(1)
    }
    PHYSFS_freeList(files as *mut libc::c_void);
    PHYSFS_delete(saveGameName_0);
}
// ////////////////////////////////////////////////////////////////////////////
// Returns TRUE if cancel pressed or a valid game slot was selected.
// if when returning TRUE strlen(sRequestResult) != 0 then a valid game
// slot was selected otherwise cancel was selected..
unsafe extern "C" fn _runLoadSave(mut bResetMissionWidgets: BOOL) -> BOOL {
    let mut current_block: u64; // set returned filename to null;
    let mut id: UDWORD = 0 as libc::c_int as UDWORD;
    let mut sEdInit: W_EDBINIT =
        W_EDBINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  FontID: 0,
                  pBoxDisplay: None,
                  pFontDisplay: None,};
    let mut sTemp: [CHAR; 256] = [0; 256];
    let mut i: UDWORD = 0;
    let mut context: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut bSkipCD: BOOL = 0 as libc::c_int;
    id = widgRunScreen(psRequestScreen);
    strcpy(sRequestResult.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    // cancel this operation...
    if !(id == (21000 as libc::c_int + 2 as libc::c_int) as libc::c_uint ||
             CancelPressed() != 0) {
        // clicked a load entry
        if id >= (21000 as libc::c_int + 10 as libc::c_int) as libc::c_uint &&
               id <=
                   (21000 as libc::c_int + 10 as libc::c_int +
                        20 as libc::c_int) as libc::c_uint {
            if mode != 0 {
                // Loading, return that entry.
                if !(*(widgGetFromID(psRequestScreen, id) as
                           *mut W_BUTTON)).pText.is_null() {
                    sprintf(sRequestResult.as_mut_ptr(),
                            b"%s%s.%s\x00" as *const u8 as
                                *const libc::c_char, sPath.as_mut_ptr(),
                            (*(widgGetFromID(psRequestScreen, id) as
                                   *mut W_BUTTON)).pText, sExt.as_mut_ptr());
                    if !(bLoadSaveMode as libc::c_uint ==
                             LOAD_FORCE as libc::c_int as libc::c_uint ||
                             bLoadSaveMode as libc::c_uint ==
                                 SAVE_FORCE as libc::c_int as libc::c_uint) {
                        // success on load.
                        setCampaignNumber(getCampaign(sRequestResult.as_mut_ptr(),
                                                      &mut bSkipCD));
                    }
                    closeLoadSave();
                    bRequestLoad = 1 as libc::c_int;
                    return 1 as libc::c_int
                    // it's a force, dont check the cd.
                }
                current_block = 4352044479084202818;
            } else {
                //  SAVING!add edit box at that position.
                if widgGetFromID(psRequestScreen,
                                 (21000 as libc::c_int + 50 as libc::c_int) as
                                     UDWORD).is_null() {
                    // add blank box.
                    memset(&mut sEdInit as *mut W_EDBINIT as
                               *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<W_EDBINIT>() as
                               libc::c_ulong); // hide the old button
                    sEdInit.formID =
                        (21000 as libc::c_int + 1 as libc::c_int) as
                            UDWORD; // prepare the savegame name.
                    sEdInit.id =
                        (21000 as libc::c_int + 50 as libc::c_int) as
                            UDWORD; // strip extension
                    sEdInit.style = 0 as libc::c_int as UDWORD;
                    sEdInit.x = (*widgGetFromID(psRequestScreen, id)).x;
                    sEdInit.y = (*widgGetFromID(psRequestScreen, id)).y;
                    sEdInit.width =
                        (*widgGetFromID(psRequestScreen, id)).width;
                    sEdInit.height =
                        (*widgGetFromID(psRequestScreen, id)).height;
                    sEdInit.pText =
                        (*(widgGetFromID(psRequestScreen, id) as
                               *mut W_BUTTON)).pText;
                    sEdInit.FontID = WFont;
                    sEdInit.pBoxDisplay =
                        Some(displayLoadSaveEdit as
                                 unsafe extern "C" fn(_: *mut _widget,
                                                      _: UDWORD, _: UDWORD,
                                                      _: *mut UDWORD) -> ());
                    widgAddEditBox(psRequestScreen, &mut sEdInit);
                    sprintf(sTemp.as_mut_ptr(),
                            b"%s%s.%s\x00" as *const u8 as
                                *const libc::c_char, sPath.as_mut_ptr(),
                            (*(widgGetFromID(psRequestScreen, id) as
                                   *mut W_BUTTON)).pText, sExt.as_mut_ptr());
                    widgHide(psRequestScreen, id);
                    chosenSlotId = id;
                    strcpy(sDelete.as_mut_ptr(), sTemp.as_mut_ptr());
                    sTemp[strlen(sTemp.as_mut_ptr()).wrapping_sub(4 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                              as usize] = '\u{0}' as i32 as CHAR;
                    // auto click in the edit box we just made.
                    context.psScreen = psRequestScreen;
                    context.psForm = (*psRequestScreen).psForm as *mut W_FORM;
                    context.xOffset = 0 as libc::c_int;
                    context.yOffset = 0 as libc::c_int;
                    context.mx = mouseX();
                    context.my = mouseY();
                    editBoxClicked(widgGetFromID(psRequestScreen,
                                                 (21000 as libc::c_int +
                                                      50 as libc::c_int) as
                                                     UDWORD) as
                                       *mut W_EDITBOX, &mut context);
                }
                current_block = 7746103178988627676;
            }
        } else { current_block = 7746103178988627676; }
        match current_block {
            4352044479084202818 => { }
            _ =>
            // finished entering a name.
            {
                if id ==
                       (21000 as libc::c_int + 50 as libc::c_int) as
                           libc::c_uint {
                    if keyPressed(KEY_RETURN) == 0 {
                        // enter was not pushed, so not a vaild entry.
                        widgDelete(psRequestScreen,
                                   (21000 as libc::c_int + 50 as libc::c_int)
                                       as
                                       UDWORD); //unselect this box, and go back ..
                        widgReveal(psRequestScreen, chosenSlotId);
                        return 1 as libc::c_int
                    }
                    // scan to see if that game exists in another slot, if
		// so then fail.
                    strcpy(sTemp.as_mut_ptr(),
                           (*(widgGetFromID(psRequestScreen, id) as
                                  *mut W_EDITBOX)).aText.as_mut_ptr()); //unselect this box, and go back ..
                    i = (21000 as libc::c_int + 10 as libc::c_int) as UDWORD;
                    while i <
                              (21000 as libc::c_int + 10 as libc::c_int +
                                   20 as libc::c_int) as libc::c_uint {
                        if i != chosenSlotId {
                            if !(*(widgGetFromID(psRequestScreen, i) as
                                       *mut W_BUTTON)).pText.is_null() &&
                                   strcmp(sTemp.as_mut_ptr(),
                                          (*(widgGetFromID(psRequestScreen, i)
                                                 as *mut W_BUTTON)).pText) ==
                                       0 as libc::c_int {
                                widgDelete(psRequestScreen,
                                           (21000 as libc::c_int +
                                                50 as libc::c_int) as UDWORD);
                                widgReveal(psRequestScreen, chosenSlotId);
                                // move mouse to same box..
				//	SetMousePos(widgGetFromID(psRequestScreen,i)->x ,widgGetFromID(psRequestScreen,i)->y);
                                audio_PlayTrack(ID_SOUND_BUILD_FAIL as
                                                    libc::c_int);
                                return 1 as libc::c_int
                            }
                        }
                        i = i.wrapping_add(1)
                    }
                    // return with this name, as we've edited it.
                    if strlen((*(widgGetFromID(psRequestScreen, id) as
                                     *mut W_EDITBOX)).aText.as_mut_ptr()) != 0
                       {
                        strcpy(sTemp.as_mut_ptr(),
                               (*(widgGetFromID(psRequestScreen, id) as
                                      *mut W_EDITBOX)).aText.as_mut_ptr());
                        removeWildcards(sTemp.as_mut_ptr());
                        sprintf(sRequestResult.as_mut_ptr(),
                                b"%s%s.%s\x00" as *const u8 as
                                    *const libc::c_char, sPath.as_mut_ptr(),
                                sTemp.as_mut_ptr(), sExt.as_mut_ptr());
                        deleteSaveGame(sDelete.as_mut_ptr());
                        //only delete game if a new game fills the slot
                        // we're done. saving.
                        closeLoadSave();
                        bRequestLoad = 0 as libc::c_int;
                        if bResetMissionWidgets != 0 &&
                               widgGetFromID(psWScreen,
                                             11002 as libc::c_int as
                                                 UDWORD).is_null() {
                            resetMissionWidgets();
                            //reset the mission widgets here if necessary
                        }
                        return 1 as libc::c_int
                    }
                } else { return 0 as libc::c_int }
            }
        }
    }
    // failed and/or cancelled..
    closeLoadSave();
    bRequestLoad = 0 as libc::c_int;
    if bResetMissionWidgets != 0 &&
           widgGetFromID(psWScreen, 11002 as libc::c_int as UDWORD).is_null()
       {
        resetMissionWidgets();
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// should be done when drawing the other widgets.
#[no_mangle]
pub unsafe extern "C" fn displayLoadSave() -> BOOL {
    widgDisplayScreen(psRequestScreen); // display widgets.
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// STRING HANDLER, replaces dos wildcards in a string with harmless chars.
#[no_mangle]
pub unsafe extern "C" fn removeWildcards(mut pStr: *mut libc::c_char) {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < strlen(pStr) {
        /*	if(   pStr[i] == '?'
		   || pStr[i] == '*'
		   || pStr[i] == '"'
		   || pStr[i] == '.'
		   || pStr[i] == '/'
		   || pStr[i] == '\\'
		   || pStr[i] == '|' )
		{
			pStr[i] = '_';
		}
*/
        if *(*__ctype_b_loc()).offset(*pStr.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
               && *pStr.offset(i as isize) as libc::c_int != ' ' as i32 &&
               *pStr.offset(i as isize) as libc::c_int != '-' as i32 &&
               *pStr.offset(i as isize) as libc::c_int != '+' as i32 &&
               *pStr.offset(i as isize) as libc::c_int != '!' as i32 {
            *pStr.offset(i as isize) = '_' as i32 as libc::c_char
        }
        i = i.wrapping_add(1)
    }
    if strlen(pStr) >= 60 as libc::c_int as libc::c_uint {
        *pStr.offset((60 as libc::c_int - 1 as libc::c_int) as isize) =
            0 as libc::c_int as libc::c_char
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// DISPLAY FUNCTIONS
unsafe extern "C" fn displayLoadBanner(mut psWidget: *mut _widget,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    //UDWORD col;
    let mut col: UBYTE = 0;
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    if !(*psWidget).pUserData.is_null() {
        col = *colours.as_mut_ptr().offset(2 as libc::c_int as isize)
    } else { col = *colours.as_mut_ptr().offset(4 as libc::c_int as isize) }
    pie_BoxFillIndex(x as libc::c_int, y as libc::c_int,
                     x.wrapping_add((*psWidget).width as libc::c_uint) as
                         libc::c_int,
                     y.wrapping_add((*psWidget).height as libc::c_uint) as
                         libc::c_int, col);
    pie_BoxFillIndex(x.wrapping_add(2 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     y.wrapping_add(2 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     x.wrapping_add((*psWidget).width as
                                        libc::c_uint).wrapping_sub(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                         as libc::c_int,
                     y.wrapping_add((*psWidget).height as
                                        libc::c_uint).wrapping_sub(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                         as libc::c_int,
                     *colours.as_mut_ptr().offset(1 as libc::c_int as isize));
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn displayLoadSlot(mut psWidget: *mut _widget,
                                     mut xOffset: UDWORD, mut yOffset: UDWORD,
                                     mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    //	UWORD	im = (UWORD)UNPACKDWORD_TRI_B((UDWORD)psWidget->pUserData);
//	UWORD	im2= (UWORD)(UNPACKDWORD_TRI_C((UDWORD)psWidget->pUserData));
    let mut butString: [STRING; 64] = [0; 64]; //draw box
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD); // font
    if !(*(psWidget as *mut W_BUTTON)).pTip.is_null() {
        strcpy(butString.as_mut_ptr(),
               (*(psWidget as *mut W_BUTTON)).pTip); //colour
        iV_SetFont(WFont);
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
        while iV_GetTextWidth(butString.as_mut_ptr()) >
                  (*psWidget).width as libc::c_int {
            butString[strlen(butString.as_mut_ptr()).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                          as usize] = '\u{0}' as i32 as STRING
        }
        //draw text
        pie_DrawText(butString.as_mut_ptr(),
                     x.wrapping_add(4 as libc::c_int as libc::c_uint),
                     y.wrapping_add(17 as libc::c_int as libc::c_uint));
    };
}
// ////////////////////////////////////////////////////////////////////////////
unsafe extern "C" fn displayLoadSaveEdit(mut psWidget: *mut _widget,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut w: UDWORD = (*psWidget).width as UDWORD;
    let mut h: UDWORD = (*psWidget).height as UDWORD;
    pie_BoxFillIndex(x as libc::c_int, y as libc::c_int,
                     x.wrapping_add(w) as libc::c_int,
                     y.wrapping_add(h) as libc::c_int,
                     *colours.as_mut_ptr().offset(4 as libc::c_int as isize));
    pie_BoxFillIndex(x.wrapping_add(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     y.wrapping_add(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     x.wrapping_add(w).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) as
                         libc::c_int,
                     y.wrapping_add(h).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) as
                         libc::c_int,
                     *colours.as_mut_ptr().offset(1 as libc::c_int as isize));
}
/* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
// true when interface is up and should be run.
//the name of the save game to load from the front end
/* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
// back form.
// cancel but.
// load/save
// banner.
// each of the buttons.
// must have unique ID hmm -Q
// save edit box. must be highest value possible I guess. -Q
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn drawBlueBox(mut x: UDWORD, mut y: UDWORD,
                                     mut w: UDWORD, mut h: UDWORD) {
    let mut dark: UBYTE =
        *colours.as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut light: UBYTE =
        *colours.as_mut_ptr().offset(9 as libc::c_int as isize);
    // box
    pie_BoxFillIndex(x.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     y.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     x.wrapping_add(w).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                         libc::c_int,
                     y.wrapping_add(h).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                         libc::c_int, light);
    pie_BoxFillIndex(x as libc::c_int, y as libc::c_int,
                     x.wrapping_add(w) as libc::c_int,
                     y.wrapping_add(h) as libc::c_int, dark);
}
