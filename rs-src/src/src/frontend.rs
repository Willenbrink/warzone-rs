use ::libc;
extern "C" {
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_uint, _: *const libc::c_char,
                _: ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn abort() -> !;
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being down to being up this frame */
    #[no_mangle]
    fn keyReleased(code: KEY_CODE) -> BOOL;
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* Warps the mouse to the given position */
    #[no_mangle]
    fn SetMousePos(nowt: UDWORD, x: UDWORD, y: UDWORD);
    /* Sets the state of the mouse key to down */
    #[no_mangle]
    fn setMouseDown(code: MOUSE_KEY_CODE);
    /* Sets the state of the mouse key to up */
    #[no_mangle]
    fn setMouseUp(code: MOUSE_KEY_CODE);
    #[no_mangle]
    fn resLoad(pResFile: *mut STRING, blockID: SDWORD,
               pLoadBuffer: *mut libc::c_char, bufferSize: SDWORD,
               psMemHeap: *mut _block_heap) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    static mut bDisableLobby: BOOL;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn setInvertMouseStatus(val: BOOL);
    #[no_mangle]
    fn getInvertMouseStatus() -> BOOL;
    // Illumination value for standard light level "as the artist drew it" ... not darker, not lighter
    //#define MAX_SCROLL_SPEED	1600
//#define SCROLL_SPEED_ACCEL	800
    // make max speed dependant on accel chosen.
    #[no_mangle]
    static mut scroll_speed_accel: UDWORD;
    #[no_mangle]
    fn setShakeStatus(val: BOOL);
    #[no_mangle]
    fn getShakeStatus() -> BOOL;
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Add a slider to a form */
    #[no_mangle]
    fn widgAddSlider(psScreen: *mut W_SCREEN, psInit: *mut W_SLDINIT) -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Set the text in a widget */
    #[no_mangle]
    fn widgSetString(psScreen: *mut W_SCREEN, id: UDWORD, pText: *mut STRING);
    /* Get the current position of a slider bar */
    #[no_mangle]
    fn widgGetSliderPos(psScreen: *mut W_SCREEN, id: UDWORD) -> UDWORD;
    /* Return the ID of the widget the mouse was over this frame */
    #[no_mangle]
    fn widgGetMouseOver(psScreen: *mut W_SCREEN) -> UDWORD;
    /* Set a button or clickable form's state */
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
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
    fn startKeyMapEditor(first: BOOL) -> BOOL;
    #[no_mangle]
    fn war_SetFog(val: BOOL);
    #[no_mangle]
    fn war_GetFog() -> BOOL;
    #[no_mangle]
    fn war_SetSeqMode(mode: SEQ_MODE);
    #[no_mangle]
    fn war_GetSeqMode() -> SEQ_MODE;
    #[no_mangle]
    fn war_GetAllowSubtitles() -> BOOL;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextAboveBase() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn pie_DrawText270(String: *mut STRING, XPos: libc::c_int,
                       YPos: libc::c_int);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // Begin a rendering lock.
    #[no_mangle]
    fn DrawBegin();
    // End a rendering lock.
    #[no_mangle]
    fn DrawEnd();
    #[no_mangle]
    fn intUpdateQuantitySlider(psWidget: *mut _widget,
                               psContext: *mut _w_context);
    #[no_mangle]
    fn mixer_GetCDVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_SetCDVolume(iVol: SDWORD);
    #[no_mangle]
    fn mixer_GetWavVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_SetWavVolume(iVol: SDWORD);
    #[no_mangle]
    fn mixer_Get3dWavVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_Set3dWavVolume(iVol: SDWORD);
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    fn intRemoveReticule();
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn initLoadingScreen(drawbdrop: BOOL, bRenderActive: BOOL);
    #[no_mangle]
    fn closeLoadingScreen();
    #[no_mangle]
    fn startCreditsScreen(bRenderActive: BOOL);
    #[no_mangle]
    fn setPlayerColour(player: UDWORD, col: UDWORD) -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut bLoadSaveUp: BOOL;
    // true when interface is up and should be run.
    //the name of the save game to load from the front end
    #[no_mangle]
    static mut saveGameName: [STRING; 256];
    #[no_mangle]
    static mut sRequestResult: [STRING; 255];
    #[no_mangle]
    fn addLoadSave(mode: LOADSAVE_MODE, defaultdir: *mut CHAR,
                   extension: *mut CHAR, title: *mut CHAR) -> BOOL;
    #[no_mangle]
    fn runLoadSave(bResetMissionWidgets: BOOL) -> BOOL;
    #[no_mangle]
    fn displayLoadSave() -> BOOL;
    #[no_mangle]
    static mut FrontendBias: SNAPBIAS;
    #[no_mangle]
    fn StartCursorSnap(SnapBuffer: *mut CURSORSNAP);
    #[no_mangle]
    fn AddCursorSnap(SnapBuffer: *mut CURSORSNAP, PosX: SWORD, PosY: SWORD,
                     FormID: UDWORD, ID: UDWORD, Bias: *mut SNAPBIAS);
    #[no_mangle]
    fn GotoDirectionalSnap(SnapBuffer: *mut CURSORSNAP,
                           Direction: SNAPDIRECTION, CurrX: SWORD,
                           CurrY: SWORD);
    #[no_mangle]
    fn SetCurrentSnapID(SnapBuffer: *mut CURSORSNAP, ID: UDWORD);
    #[no_mangle]
    fn SnapToID(SnapBuffer: *mut CURSORSNAP, snp: UWORD);
    // the block heap for the game data
    #[no_mangle]
    static mut psGameHeap: *mut BLOCK_HEAP;
    #[no_mangle]
    fn setDifficultyLevel(lev: DIFFICULTY_LEVEL);
    #[no_mangle]
    fn getDifficultyLevel() -> DIFFICULTY_LEVEL;
    #[no_mangle]
    fn avSetStatus(var: BOOL);
    #[no_mangle]
    fn seq_AddSeqToList(pSeqName: *mut STRING, pAudioName: *mut STRING,
                        pTextName: *mut STRING, bLoop: BOOL,
                        PSXSeqNumber: UDWORD);
    #[no_mangle]
    fn seq_ClearSeqList();
    //set and check subtitle mode, TRUE subtitles on
    #[no_mangle]
    fn seq_SetSubtitles(bNewState: BOOL);
    #[no_mangle]
    fn seq_GetSubtitles() -> BOOL;
    /*returns the next sequence in the list to play*/
    #[no_mangle]
    fn seq_StartNextFullScreenVideo();
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    #[no_mangle]
    fn startConnectionScreen() -> BOOL;
    #[no_mangle]
    fn startGameFind();
    #[no_mangle]
    fn startMultiOptions(bReenter: BOOL) -> BOOL;
    #[no_mangle]
    fn startForceSelect() -> BOOL;
    #[no_mangle]
    fn addMultiBut(screen: *mut W_SCREEN, formid: UDWORD, id: UDWORD,
                   x: UDWORD, y: UDWORD, width: UDWORD, height: UDWORD,
                   tipres: UDWORD, norm: UDWORD, hi: UDWORD,
                   showmouseover: BOOL) -> BOOL;
    /*
 * multilimit.h
 */
    #[no_mangle]
    fn startLimitScreen() -> BOOL;
    //from netusers.c
    #[no_mangle]
    fn NETuseNetwork(val: BOOL) -> BOOL;
    #[no_mangle]
    fn version() -> *const libc::c_char;
    /*
 * FrontEnd.c
 *
 * front end title and options screens.
 * Alex Lee. Pumpkin Studios. Eidos PLC 98,
 */
    /*	Playstation button symbol -> font mappings.
|	=	X
{	=	Circle
}	=	Square
~	=	Triangle
*/
    /* Includes direct access to render library */
    // FIXME Direct iVis implementation include!
    // for sound.
    //#include "wrappers.h"				// for bUsingKeyboard.
    #[no_mangle]
    fn intUpdateOptionText(psWidget: *mut _widget,
                           psContext: *mut _w_context);
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
    // ////////////////////////////////////////////////////////////////////////////
// extern Definitions
    //extern W_SCREEN		*psWScreen;					//The widget screen
    #[no_mangle]
    static mut SaveGamePath: [libc::c_char; 0];
    #[no_mangle]
    static mut FrontImages: *mut IMAGEFILE;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
/*
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
/* structure used to store the extra space allocated for the heap */
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
// Extension memory for the heap
/*
 * Treap.h
 *
 * A balanced binary tree implementation
 *
 * Overhead for using the treap is -
 *		Overhead for the heap used by the treap :
 *                  24 bytes + 4 bytes per extension
 *      12 bytes for the root
 *      20 bytes per node
 */
/* Turn on and off the treap debugging */
/* Function type for the object compare
 * return -1 for less
 *         1 for more
 *         0 for equal
 */
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
/* The basic elements in the treap node.
 * These are done as macros so that the memory system
 * can use parts of the treap system.
 */
/* The key to sort the node on */
/* Treap priority */
/* The object stored in the treap */
/* The sub trees */
/* The debug info */
/* file the node was created in */
/* line the node was created at */
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
/* Treap data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
// root of the tree
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
pub type BLOCK_HEAP = _block_heap;
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
// The next free ID
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
/* The optional user callback function */
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
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
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
pub struct _w_sldinit {
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
    pub orientation: UWORD,
    pub numStops: UWORD,
    pub barSize: UWORD,
    pub pos: UWORD,
    pub pTip: *mut STRING,
}
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
// Slider is horizontal and starts at left
// Slider is horizontal and starts at the right
// Slider is vertical and starts at the top
// Slider is vertical and starts at the bottom
/* Slider initialisation structure */
pub type W_SLDINIT = _w_sldinit;
pub type SEQ_MODE = libc::c_uint;
pub const SEQ_SKIP: SEQ_MODE = 2;
pub const SEQ_SMALL: SEQ_MODE = 1;
pub const SEQ_FULL: SEQ_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_label {
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
    pub pTip: *mut STRING,
}
pub type W_LABEL = _w_label;
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
pub type W_BUTTON = _w_button;
/* The basic init entries */
// Orientation of the slider
// Number of stops on the slider
// Size of the bar
// Initial position of the slider bar
// Tip string
/* Slider state */
// Slider is being dragged
// Slider is hilited
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_slider {
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
    pub orientation: UWORD,
    pub numStops: UWORD,
    pub barSize: UWORD,
    pub pos: UWORD,
    pub state: UWORD,
    pub pTip: *mut STRING,
}
pub type W_SLIDER = _w_slider;
// Tool tip
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed = 479;
pub const IMAGE_BLUE6: C2RustUnnamed = 478;
pub const IMAGE_BLUE5: C2RustUnnamed = 477;
pub const IMAGE_BLUE4: C2RustUnnamed = 476;
pub const IMAGE_BLUE3: C2RustUnnamed = 475;
pub const IMAGE_BLUE2: C2RustUnnamed = 474;
pub const IMAGE_BLUE1: C2RustUnnamed = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed = 468;
pub const IMAGE_TARGET5: C2RustUnnamed = 467;
pub const IMAGE_TARGET4: C2RustUnnamed = 466;
pub const IMAGE_TARGET3: C2RustUnnamed = 465;
pub const IMAGE_TARGET2: C2RustUnnamed = 464;
pub const IMAGE_TARGET1: C2RustUnnamed = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed = 457;
pub const IMAGE_ASCII131: C2RustUnnamed = 456;
pub const IMAGE_ASCII161: C2RustUnnamed = 455;
pub const IMAGE_ASCII191: C2RustUnnamed = 454;
pub const IMAGE_ASCII208: C2RustUnnamed = 453;
pub const IMAGE_ASCII207: C2RustUnnamed = 452;
pub const IMAGE_ASCII206: C2RustUnnamed = 451;
pub const IMAGE_ASCII205: C2RustUnnamed = 450;
pub const IMAGE_ASCII204: C2RustUnnamed = 449;
pub const IMAGE_ASCII203: C2RustUnnamed = 448;
pub const IMAGE_ASCII202: C2RustUnnamed = 447;
pub const IMAGE_ASCII201: C2RustUnnamed = 446;
pub const IMAGE_ASCII200: C2RustUnnamed = 445;
pub const IMAGE_ASCII198: C2RustUnnamed = 444;
pub const IMAGE_ASCII197: C2RustUnnamed = 443;
pub const IMAGE_ASCII196: C2RustUnnamed = 442;
pub const IMAGE_ASCII195: C2RustUnnamed = 441;
pub const IMAGE_ASCII194: C2RustUnnamed = 440;
pub const IMAGE_ASCII193: C2RustUnnamed = 439;
pub const IMAGE_ASCII192: C2RustUnnamed = 438;
pub const IMAGE_ASCII210: C2RustUnnamed = 437;
pub const IMAGE_ASCII211: C2RustUnnamed = 436;
pub const IMAGE_ASCII214: C2RustUnnamed = 435;
pub const IMAGE_ASCII213: C2RustUnnamed = 434;
pub const IMAGE_ASCII212: C2RustUnnamed = 433;
pub const IMAGE_ASCII216: C2RustUnnamed = 432;
pub const IMAGE_ASCII220: C2RustUnnamed = 431;
pub const IMAGE_ASCII217: C2RustUnnamed = 430;
pub const IMAGE_ASCII219: C2RustUnnamed = 429;
pub const IMAGE_ASCII218: C2RustUnnamed = 428;
pub const IMAGE_ASCII221: C2RustUnnamed = 427;
pub const IMAGE_ASCII223: C2RustUnnamed = 426;
pub const IMAGE_ASCII248: C2RustUnnamed = 425;
pub const IMAGE_ASCII241: C2RustUnnamed = 424;
pub const IMAGE_ASCII253: C2RustUnnamed = 423;
pub const IMAGE_ASCII252: C2RustUnnamed = 422;
pub const IMAGE_ASCII251: C2RustUnnamed = 421;
pub const IMAGE_ASCII250: C2RustUnnamed = 420;
pub const IMAGE_ASCII249: C2RustUnnamed = 419;
pub const IMAGE_ASCII246: C2RustUnnamed = 418;
pub const IMAGE_ASCII245: C2RustUnnamed = 417;
pub const IMAGE_ASCII244: C2RustUnnamed = 416;
pub const IMAGE_ASCII243: C2RustUnnamed = 415;
pub const IMAGE_ASCII242: C2RustUnnamed = 414;
pub const IMAGE_ASCII239: C2RustUnnamed = 413;
pub const IMAGE_ASCII238: C2RustUnnamed = 412;
pub const IMAGE_ASCII237: C2RustUnnamed = 411;
pub const IMAGE_ASCII236: C2RustUnnamed = 410;
pub const IMAGE_ASCII235: C2RustUnnamed = 409;
pub const IMAGE_ASCII234: C2RustUnnamed = 408;
pub const IMAGE_ASCII233: C2RustUnnamed = 407;
pub const IMAGE_ASCII232: C2RustUnnamed = 406;
pub const IMAGE_ASCII231: C2RustUnnamed = 405;
pub const IMAGE_ASCII230: C2RustUnnamed = 404;
pub const IMAGE_ASCII229: C2RustUnnamed = 403;
pub const IMAGE_ASCII228: C2RustUnnamed = 402;
pub const IMAGE_ASCII227: C2RustUnnamed = 401;
pub const IMAGE_ASCII226: C2RustUnnamed = 400;
pub const IMAGE_ASCII225: C2RustUnnamed = 399;
pub const IMAGE_ASCII224: C2RustUnnamed = 398;
pub const IMAGE_ASCII189: C2RustUnnamed = 397;
pub const IMAGE_ASCII188: C2RustUnnamed = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed = 395;
pub const IMAGE_LEV_7: C2RustUnnamed = 394;
pub const IMAGE_LEV_6: C2RustUnnamed = 393;
pub const IMAGE_LEV_5: C2RustUnnamed = 392;
pub const IMAGE_LEV_4: C2RustUnnamed = 391;
pub const IMAGE_LEV_3: C2RustUnnamed = 390;
pub const IMAGE_LEV_2: C2RustUnnamed = 389;
pub const IMAGE_LEV_1: C2RustUnnamed = 388;
pub const IMAGE_LEV_0: C2RustUnnamed = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed = 373;
pub const IMAGE_GN_0: C2RustUnnamed = 372;
pub const IMAGE_GN_1: C2RustUnnamed = 371;
pub const IMAGE_GN_2: C2RustUnnamed = 370;
pub const IMAGE_GN_3: C2RustUnnamed = 369;
pub const IMAGE_GN_4: C2RustUnnamed = 368;
pub const IMAGE_GN_5: C2RustUnnamed = 367;
pub const IMAGE_GN_6: C2RustUnnamed = 366;
pub const IMAGE_GN_7: C2RustUnnamed = 365;
pub const IMAGE_GN_8: C2RustUnnamed = 364;
pub const IMAGE_GN_9: C2RustUnnamed = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed = 320;
pub const IMAGE_NRUTER: C2RustUnnamed = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed = 253;
pub const IMAGE_TRACKS: C2RustUnnamed = 252;
pub const IMAGE_STAR: C2RustUnnamed = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed = 228;
pub const IMAGE_ASCII126: C2RustUnnamed = 227;
pub const IMAGE_ASCII125: C2RustUnnamed = 226;
pub const IMAGE_ASCII124: C2RustUnnamed = 225;
pub const IMAGE_ASCII123: C2RustUnnamed = 224;
pub const IMAGE_ASCII122: C2RustUnnamed = 223;
pub const IMAGE_ASCII121: C2RustUnnamed = 222;
pub const IMAGE_ASCII120: C2RustUnnamed = 221;
pub const IMAGE_ASCII119: C2RustUnnamed = 220;
pub const IMAGE_ASCII118: C2RustUnnamed = 219;
pub const IMAGE_ASCII117: C2RustUnnamed = 218;
pub const IMAGE_ASCII116: C2RustUnnamed = 217;
pub const IMAGE_ASCII115: C2RustUnnamed = 216;
pub const IMAGE_ASCII114: C2RustUnnamed = 215;
pub const IMAGE_ASCII113: C2RustUnnamed = 214;
pub const IMAGE_ASCII112: C2RustUnnamed = 213;
pub const IMAGE_ASCII111: C2RustUnnamed = 212;
pub const IMAGE_ASCII110: C2RustUnnamed = 211;
pub const IMAGE_ASCII109: C2RustUnnamed = 210;
pub const IMAGE_ASCII108: C2RustUnnamed = 209;
pub const IMAGE_ASCII107: C2RustUnnamed = 208;
pub const IMAGE_ASCII106: C2RustUnnamed = 207;
pub const IMAGE_ASCII105: C2RustUnnamed = 206;
pub const IMAGE_ASCII104: C2RustUnnamed = 205;
pub const IMAGE_ASCII103: C2RustUnnamed = 204;
pub const IMAGE_ASCII102: C2RustUnnamed = 203;
pub const IMAGE_ASCII101: C2RustUnnamed = 202;
pub const IMAGE_ASCII100: C2RustUnnamed = 201;
pub const IMAGE_ASCII99: C2RustUnnamed = 200;
pub const IMAGE_ASCII98: C2RustUnnamed = 199;
pub const IMAGE_ASCII97: C2RustUnnamed = 198;
pub const IMAGE_ASCII96: C2RustUnnamed = 197;
pub const IMAGE_ASCII95: C2RustUnnamed = 196;
pub const IMAGE_ASCII94: C2RustUnnamed = 195;
pub const IMAGE_ASCII93: C2RustUnnamed = 194;
pub const IMAGE_ASCII92: C2RustUnnamed = 193;
pub const IMAGE_ASCII91: C2RustUnnamed = 192;
pub const IMAGE_ASCII90: C2RustUnnamed = 191;
pub const IMAGE_ASCII89: C2RustUnnamed = 190;
pub const IMAGE_ASCII88: C2RustUnnamed = 189;
pub const IMAGE_ASCII87: C2RustUnnamed = 188;
pub const IMAGE_ASCII86: C2RustUnnamed = 187;
pub const IMAGE_ASCII85: C2RustUnnamed = 186;
pub const IMAGE_ASCII84: C2RustUnnamed = 185;
pub const IMAGE_ASCII83: C2RustUnnamed = 184;
pub const IMAGE_ASCII82: C2RustUnnamed = 183;
pub const IMAGE_ASCII81: C2RustUnnamed = 182;
pub const IMAGE_ASCII80: C2RustUnnamed = 181;
pub const IMAGE_ASCII79: C2RustUnnamed = 180;
pub const IMAGE_ASCII78: C2RustUnnamed = 179;
pub const IMAGE_ASCII77: C2RustUnnamed = 178;
pub const IMAGE_ASCII76: C2RustUnnamed = 177;
pub const IMAGE_ASCII75: C2RustUnnamed = 176;
pub const IMAGE_ASCII74: C2RustUnnamed = 175;
pub const IMAGE_ASCII73: C2RustUnnamed = 174;
pub const IMAGE_ASCII72: C2RustUnnamed = 173;
pub const IMAGE_ASCII71: C2RustUnnamed = 172;
pub const IMAGE_ASCII70: C2RustUnnamed = 171;
pub const IMAGE_ASCII69: C2RustUnnamed = 170;
pub const IMAGE_ASCII68: C2RustUnnamed = 169;
pub const IMAGE_ASCII67: C2RustUnnamed = 168;
pub const IMAGE_ASCII66: C2RustUnnamed = 167;
pub const IMAGE_ASCII65: C2RustUnnamed = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed = 165;
pub const IMAGE_ASCII63: C2RustUnnamed = 164;
pub const IMAGE_ASCII62: C2RustUnnamed = 163;
pub const IMAGE_ASCII61: C2RustUnnamed = 162;
pub const IMAGE_ASCII60: C2RustUnnamed = 161;
pub const IMAGE_ASCII59: C2RustUnnamed = 160;
pub const IMAGE_ASCII58: C2RustUnnamed = 159;
pub const IMAGE_ASCII57: C2RustUnnamed = 158;
pub const IMAGE_ASCII56: C2RustUnnamed = 157;
pub const IMAGE_ASCII55: C2RustUnnamed = 156;
pub const IMAGE_ASCII54: C2RustUnnamed = 155;
pub const IMAGE_ASCII53: C2RustUnnamed = 154;
pub const IMAGE_ASCII52: C2RustUnnamed = 153;
pub const IMAGE_ASCII51: C2RustUnnamed = 152;
pub const IMAGE_ASCII50: C2RustUnnamed = 151;
pub const IMAGE_ASCII49: C2RustUnnamed = 150;
pub const IMAGE_ASCII48: C2RustUnnamed = 149;
pub const IMAGE_ASCII47: C2RustUnnamed = 148;
pub const IMAGE_ASCII46: C2RustUnnamed = 147;
pub const IMAGE_ASCII45: C2RustUnnamed = 146;
pub const IMAGE_ASCII44: C2RustUnnamed = 145;
pub const IMAGE_ASCII43: C2RustUnnamed = 144;
pub const IMAGE_ASCII42: C2RustUnnamed = 143;
pub const IMAGE_ASCII41: C2RustUnnamed = 142;
pub const IMAGE_ASCII40: C2RustUnnamed = 141;
pub const IMAGE_ASCII39: C2RustUnnamed = 140;
pub const IMAGE_ASCII38: C2RustUnnamed = 139;
pub const IMAGE_ASCII37: C2RustUnnamed = 138;
pub const IMAGE_ASCII36: C2RustUnnamed = 137;
pub const IMAGE_ASCII35: C2RustUnnamed = 136;
pub const IMAGE_ASCII34: C2RustUnnamed = 135;
pub const IMAGE_ASCII33: C2RustUnnamed = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed = 124;
pub const IMAGE_9: C2RustUnnamed = 123;
pub const IMAGE_8: C2RustUnnamed = 122;
pub const IMAGE_7: C2RustUnnamed = 121;
pub const IMAGE_6: C2RustUnnamed = 120;
pub const IMAGE_5: C2RustUnnamed = 119;
pub const IMAGE_4: C2RustUnnamed = 118;
pub const IMAGE_3: C2RustUnnamed = 117;
pub const IMAGE_2: C2RustUnnamed = 116;
pub const IMAGE_1: C2RustUnnamed = 115;
pub const IMAGE_0: C2RustUnnamed = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed = 108;
pub const IMAGE_ECM: C2RustUnnamed = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed = 105;
pub const IMAGE_CANNON: C2RustUnnamed = 104;
pub const IMAGE_ROCKET: C2RustUnnamed = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed = 44;
pub const IMAGE_CLOSE: C2RustUnnamed = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed = 14;
pub const IMAGE_TAB4: C2RustUnnamed = 13;
pub const IMAGE_TAB3: C2RustUnnamed = 12;
pub const IMAGE_TAB2: C2RustUnnamed = 11;
pub const IMAGE_TAB1: C2RustUnnamed = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed = 0;
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
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
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
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
pub const GAMEFIND: _title_mode = 10;
pub const FORCESELECT: _title_mode = 9;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPCOORD {
    pub FormID: UDWORD,
    pub ID: UDWORD,
    pub SnapX: SWORD,
    pub SnapY: SWORD,
    pub Bias: *mut SNAPBIAS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPBIAS {
    pub NDxBias: SWORD,
    pub NDyBias: SWORD,
    pub UDxBias: SWORD,
    pub UDyBias: SWORD,
    pub RDxBias: SWORD,
    pub RDyBias: SWORD,
    pub DDxBias: SWORD,
    pub DDyBias: SWORD,
    pub LDxBias: SWORD,
    pub LDyBias: SWORD,
}
pub const IMAGE_RETURN_HI: C2RustUnnamed_0 = 173;
pub const IMAGE_RETURN: C2RustUnnamed_0 = 149;
pub const IMAGE_FE_LOGO: C2RustUnnamed_0 = 0;
pub const IMAGE_PLAYERX: C2RustUnnamed_0 = 197;
pub const IMAGE_PLAYER7: C2RustUnnamed_0 = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed_0 = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed_0 = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed_0 = 116;
pub const IMAGE_PLAYER0: C2RustUnnamed_0 = 112;
pub const DL_HARD: _difficulty_level = 2;
pub const DL_NORMAL: _difficulty_level = 1;
pub const DL_EASY: _difficulty_level = 0;
pub type DIFFICULTY_LEVEL = _difficulty_level;
pub type _difficulty_level = libc::c_uint;
pub const DL_KILLER: _difficulty_level = 4;
pub const DL_TOUGH: _difficulty_level = 3;
pub type SNAPDIRECTION = libc::c_uint;
pub const SNAP_NEAREST: SNAPDIRECTION = 4;
pub const SNAP_LEFT: SNAPDIRECTION = 3;
pub const SNAP_DOWN: SNAPDIRECTION = 2;
pub const SNAP_RIGHT: SNAPDIRECTION = 1;
pub const SNAP_UP: SNAPDIRECTION = 0;
// 0 intro mode
// 1 single player menu
// 2 multiplayer menu
// 3 options menu	
// 4
// 5  tutorial/fastplay	
// 6  credits
// 7  MULTIPLAYER, select proto
// 8 MULTIPLAYER, select game options
// 9 MULTIPLAYER, Force design screen
// 10 MULTIPLAYER, gamefinder.
// 11 MULTIPLAYER, Limit the multistuff.
// 12 Fire up the game
// 13 reshow the intro
// 14 leaving game
// 15 loading a save game
// 16 keymap editor
// 17 second options menu.
// 18 third options menu.
//		GRAPHICS,					// 5
//		VIDEO,
//	DEMOMODE,					// demo mode. remove for release?
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
pub type _loadsave_mode = libc::c_uint;
pub const SAVE_FORCE: _loadsave_mode = 6;
pub const LOAD_FORCE: _loadsave_mode = 5;
pub const SAVE_INGAME: _loadsave_mode = 4;
pub const LOAD_INGAME: _loadsave_mode = 3;
pub const SAVE_MISSIONEND: _loadsave_mode = 2;
pub const LOAD_MISSIONEND: _loadsave_mode = 1;
pub const LOAD_FRONTEND: _loadsave_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERINGAME {
    pub PingTimes: [UDWORD; 8],
    pub localOptionsReceived: BOOL,
    pub localJoiningInProgress: BOOL,
    pub JoiningInProgress: [BOOL; 8],
    pub bHostSetup: BOOL,
    pub startTime: UDWORD,
    pub modem: UDWORD,
    pub numStructureLimits: UDWORD,
    pub pStructureLimits: *mut UBYTE,
    pub skScores: [[UDWORD; 2]; 8],
    pub phrases: [[CHAR; 255]; 5],
}
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMAGE_NOJOIN: C2RustUnnamed_0 = 281;
pub const IMAGE_TFONT_45: C2RustUnnamed_0 = 280;
pub const IMAGE_TFONT_63: C2RustUnnamed_0 = 279;
pub const IMAGE_TFONT_205: C2RustUnnamed_0 = 278;
pub const IMAGE_TFONT_204: C2RustUnnamed_0 = 277;
pub const IMAGE_TFONT_207: C2RustUnnamed_0 = 276;
pub const IMAGE_TFONT_206: C2RustUnnamed_0 = 275;
pub const IMAGE_TFONT_211: C2RustUnnamed_0 = 274;
pub const IMAGE_TFONT_202: C2RustUnnamed_0 = 273;
pub const IMAGE_TFONT_170: C2RustUnnamed_0 = 272;
pub const IMAGE_TFONT_163: C2RustUnnamed_0 = 271;
pub const IMAGE_TFONT_223: C2RustUnnamed_0 = 270;
pub const IMAGE_MULTIRANK3: C2RustUnnamed_0 = 269;
pub const IMAGE_KEYMAP_DEFAULT: C2RustUnnamed_0 = 268;
pub const IMAGE_NOPENCIL: C2RustUnnamed_0 = 267;
pub const IMAGE_PENCIL: C2RustUnnamed_0 = 266;
pub const IMAGE_TFONT_245: C2RustUnnamed_0 = 265;
pub const IMAGE_TFONT_221: C2RustUnnamed_0 = 264;
pub const IMAGE_TFONT_217: C2RustUnnamed_0 = 263;
pub const IMAGE_TFONT_219: C2RustUnnamed_0 = 262;
pub const IMAGE_TFONT_218: C2RustUnnamed_0 = 261;
pub const IMAGE_TFONT_213: C2RustUnnamed_0 = 260;
pub const IMAGE_TFONT_210: C2RustUnnamed_0 = 259;
pub const IMAGE_TFONT_212: C2RustUnnamed_0 = 258;
pub const IMAGE_TFONT_200: C2RustUnnamed_0 = 257;
pub const IMAGE_TFONT_203: C2RustUnnamed_0 = 256;
pub const IMAGE_TFONT_208: C2RustUnnamed_0 = 255;
pub const IMAGE_TFONT_240: C2RustUnnamed_0 = 254;
pub const IMAGE_TFONT_195: C2RustUnnamed_0 = 253;
pub const IMAGE_TFONT_192: C2RustUnnamed_0 = 252;
pub const IMAGE_TFONT_194: C2RustUnnamed_0 = 251;
pub const IMAGE_TFONT_193: C2RustUnnamed_0 = 250;
pub const IMAGE_TFONT_187: C2RustUnnamed_0 = 249;
pub const IMAGE_TFONT_171: C2RustUnnamed_0 = 248;
pub const IMAGE_TFONT_172: C2RustUnnamed_0 = 247;
pub const IMAGE_TFONT_174: C2RustUnnamed_0 = 246;
pub const IMAGE_TFONT_161: C2RustUnnamed_0 = 245;
pub const IMAGE_TFONT_191: C2RustUnnamed_0 = 244;
pub const IMAGE_TFONT_176: C2RustUnnamed_0 = 243;
pub const IMAGE_TFONT_131: C2RustUnnamed_0 = 242;
pub const IMAGE_TFONT_215: C2RustUnnamed_0 = 241;
pub const IMAGE_TFONT_216: C2RustUnnamed_0 = 240;
pub const IMAGE_TFONT_220: C2RustUnnamed_0 = 239;
pub const IMAGE_TFONT_214: C2RustUnnamed_0 = 238;
pub const IMAGE_TFONT_201: C2RustUnnamed_0 = 237;
pub const IMAGE_TFONT_197: C2RustUnnamed_0 = 236;
pub const IMAGE_TFONT_196: C2RustUnnamed_0 = 235;
pub const IMAGE_TFONT_198: C2RustUnnamed_0 = 234;
pub const IMAGE_TFONT_128: C2RustUnnamed_0 = 233;
pub const IMAGE_TFONT_253: C2RustUnnamed_0 = 232;
pub const IMAGE_TFONT_252: C2RustUnnamed_0 = 231;
pub const IMAGE_TFONT_251: C2RustUnnamed_0 = 230;
pub const IMAGE_TFONT_250: C2RustUnnamed_0 = 229;
pub const IMAGE_TFONT_249: C2RustUnnamed_0 = 228;
pub const IMAGE_TFONT_248: C2RustUnnamed_0 = 227;
pub const IMAGE_TFONT_246: C2RustUnnamed_0 = 226;
pub const IMAGE_TFONT_244: C2RustUnnamed_0 = 225;
pub const IMAGE_TFONT_243: C2RustUnnamed_0 = 224;
pub const IMAGE_TFONT_242: C2RustUnnamed_0 = 223;
pub const IMAGE_TFONT_209: C2RustUnnamed_0 = 222;
pub const IMAGE_TFONT_241: C2RustUnnamed_0 = 221;
pub const IMAGE_TFONT_239: C2RustUnnamed_0 = 220;
pub const IMAGE_TFONT_238: C2RustUnnamed_0 = 219;
pub const IMAGE_TFONT_237: C2RustUnnamed_0 = 218;
pub const IMAGE_TFONT_236: C2RustUnnamed_0 = 217;
pub const IMAGE_TFONT_235: C2RustUnnamed_0 = 216;
pub const IMAGE_TFONT_234: C2RustUnnamed_0 = 215;
pub const IMAGE_TFONT_233: C2RustUnnamed_0 = 214;
pub const IMAGE_TFONT_232: C2RustUnnamed_0 = 213;
pub const IMAGE_TFONT_199: C2RustUnnamed_0 = 212;
pub const IMAGE_TFONT_230: C2RustUnnamed_0 = 211;
pub const IMAGE_TFONT_229: C2RustUnnamed_0 = 210;
pub const IMAGE_TFONT_228: C2RustUnnamed_0 = 209;
pub const IMAGE_TFONT_227: C2RustUnnamed_0 = 208;
pub const IMAGE_TFONT_226: C2RustUnnamed_0 = 207;
pub const IMAGE_TFONT_188: C2RustUnnamed_0 = 206;
pub const IMAGE_TFONT_224: C2RustUnnamed_0 = 205;
pub const IMAGE_TFONT_225: C2RustUnnamed_0 = 204;
pub const IMAGE_TFONT_189: C2RustUnnamed_0 = 203;
pub const IMAGE_WEE_GUY: C2RustUnnamed_0 = 202;
pub const IMAGE_FOG_ON_HI: C2RustUnnamed_0 = 201;
pub const IMAGE_FOG_OFF_HI: C2RustUnnamed_0 = 200;
pub const IMAGE_FOG_ON: C2RustUnnamed_0 = 199;
pub const IMAGE_FOG_OFF: C2RustUnnamed_0 = 198;
pub const IMAGE_MEDAL_DUMMY: C2RustUnnamed_0 = 196;
pub const IMAGE_MULTIRANK2: C2RustUnnamed_0 = 195;
pub const IMAGE_PLAYER_PC: C2RustUnnamed_0 = 194;
pub const IMAGE_TEAM_HI: C2RustUnnamed_0 = 193;
pub const IMAGE_SKIRMISH_HI: C2RustUnnamed_0 = 192;
pub const IMAGE_TEAM: C2RustUnnamed_0 = 191;
pub const IMAGE_SKIRMISH: C2RustUnnamed_0 = 190;
pub const IMAGE_TEAM_OVER: C2RustUnnamed_0 = 189;
pub const IMAGE_SKIRMISH_OVER: C2RustUnnamed_0 = 188;
pub const IMAGE_LAMP_GREEN: C2RustUnnamed_0 = 187;
pub const IMAGE_LAMP_AMBER: C2RustUnnamed_0 = 186;
pub const IMAGE_LAMP_RED: C2RustUnnamed_0 = 185;
pub const IMAGE_COMPUTER_Y_HI: C2RustUnnamed_0 = 184;
pub const IMAGE_COMPUTER_Y: C2RustUnnamed_0 = 183;
pub const IMAGE_COMPUTER_N: C2RustUnnamed_0 = 182;
pub const IMAGE_COMPUTER_N_HI: C2RustUnnamed_0 = 181;
pub const IMAGE_HI56: C2RustUnnamed_0 = 180;
pub const IMAGE_DEFAULTFORCE: C2RustUnnamed_0 = 179;
pub const IMAGE_CLEARFORCE: C2RustUnnamed_0 = 178;
pub const IMAGE_SAVEFORCE: C2RustUnnamed_0 = 177;
pub const IMAGE_LOADFORCE: C2RustUnnamed_0 = 176;
pub const IMAGE_SLIM_HI: C2RustUnnamed_0 = 175;
pub const IMAGE_SLIM: C2RustUnnamed_0 = 174;
pub const IMAGE_FRAGLIMIT_HI: C2RustUnnamed_0 = 172;
pub const IMAGE_TIMELIMIT_HI: C2RustUnnamed_0 = 171;
pub const IMAGE_NOLIMIT_HI: C2RustUnnamed_0 = 170;
pub const IMAGE_FRAGLIMIT: C2RustUnnamed_0 = 169;
pub const IMAGE_TIMELIMIT: C2RustUnnamed_0 = 168;
pub const IMAGE_NOLIMIT: C2RustUnnamed_0 = 167;
pub const IMAGE_LBASE_HI: C2RustUnnamed_0 = 166;
pub const IMAGE_SBASE_HI: C2RustUnnamed_0 = 165;
pub const IMAGE_NOBASE_HI: C2RustUnnamed_0 = 164;
pub const IMAGE_LBASE: C2RustUnnamed_0 = 163;
pub const IMAGE_SBASE: C2RustUnnamed_0 = 162;
pub const IMAGE_NOBASE: C2RustUnnamed_0 = 161;
pub const IMAGE_TECHHI_HI: C2RustUnnamed_0 = 160;
pub const IMAGE_TECHMED_HI: C2RustUnnamed_0 = 159;
pub const IMAGE_TECHLO_HI: C2RustUnnamed_0 = 158;
pub const IMAGE_TECHHI: C2RustUnnamed_0 = 157;
pub const IMAGE_TECHMED: C2RustUnnamed_0 = 156;
pub const IMAGE_TECHLO: C2RustUnnamed_0 = 155;
pub const IMAGE_BIGOK: C2RustUnnamed_0 = 154;
pub const IMAGE_EDIT_GAME: C2RustUnnamed_0 = 153;
pub const IMAGE_EDIT_MAP: C2RustUnnamed_0 = 152;
pub const IMAGE_EDIT_FORCE: C2RustUnnamed_0 = 151;
pub const IMAGE_EDIT_PLAYER: C2RustUnnamed_0 = 150;
pub const IMAGE_MULTIRANK1: C2RustUnnamed_0 = 148;
pub const IMAGE_POWLO: C2RustUnnamed_0 = 147;
pub const IMAGE_MEDAL_BRONZE: C2RustUnnamed_0 = 146;
pub const IMAGE_MEDAL_SILVER: C2RustUnnamed_0 = 145;
pub const IMAGE_MEDAL_GOLD: C2RustUnnamed_0 = 144;
pub const IMAGE_CAMPAIGN_OVER: C2RustUnnamed_0 = 143;
pub const IMAGE_ARENA_OVER: C2RustUnnamed_0 = 142;
pub const IMAGE_HI64: C2RustUnnamed_0 = 141;
pub const IMAGE_HI41: C2RustUnnamed_0 = 140;
pub const IMAGE_HI39: C2RustUnnamed_0 = 139;
pub const IMAGE_HI23: C2RustUnnamed_0 = 138;
pub const IMAGE_HI31: C2RustUnnamed_0 = 137;
pub const IMAGE_HI34: C2RustUnnamed_0 = 136;
pub const IMAGE_COM4_HI: C2RustUnnamed_0 = 135;
pub const IMAGE_COM3_HI: C2RustUnnamed_0 = 134;
pub const IMAGE_ALLI_HI: C2RustUnnamed_0 = 133;
pub const IMAGE_OFFALLI_HI: C2RustUnnamed_0 = 132;
pub const IMAGE_NOALLI_HI: C2RustUnnamed_0 = 131;
pub const IMAGE_ALLI: C2RustUnnamed_0 = 130;
pub const IMAGE_OFFALLI: C2RustUnnamed_0 = 129;
pub const IMAGE_NOALLI: C2RustUnnamed_0 = 128;
pub const IMAGE_POWHI_HI: C2RustUnnamed_0 = 127;
pub const IMAGE_POWMED_HI: C2RustUnnamed_0 = 126;
pub const IMAGE_POWLO_HI: C2RustUnnamed_0 = 125;
pub const IMAGE_POWHI: C2RustUnnamed_0 = 124;
pub const IMAGE_POWMED: C2RustUnnamed_0 = 123;
pub const IMAGE_OK: C2RustUnnamed_0 = 122;
pub const IMAGE_NO: C2RustUnnamed_0 = 121;
pub const IMAGE_HOST: C2RustUnnamed_0 = 120;
pub const IMAGE_PLAYER3: C2RustUnnamed_0 = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed_0 = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed_0 = 113;
pub const IMAGE_REFRESH: C2RustUnnamed_0 = 111;
pub const IMAGE_CAMPAIGN: C2RustUnnamed_0 = 110;
pub const IMAGE_ARENA: C2RustUnnamed_0 = 109;
pub const IMAGE_115200: C2RustUnnamed_0 = 108;
pub const IMAGE_56000: C2RustUnnamed_0 = 107;
pub const IMAGE_19200: C2RustUnnamed_0 = 106;
pub const IMAGE_14400: C2RustUnnamed_0 = 105;
pub const IMAGE_CAMPAIGN_HI: C2RustUnnamed_0 = 104;
pub const IMAGE_ARENA_HI: C2RustUnnamed_0 = 103;
pub const IMAGE_115200_HI: C2RustUnnamed_0 = 102;
pub const IMAGE_56000_HI: C2RustUnnamed_0 = 101;
pub const IMAGE_19200_HI: C2RustUnnamed_0 = 100;
pub const IMAGE_14400_HI: C2RustUnnamed_0 = 99;
pub const IMAGE_COM2_HI: C2RustUnnamed_0 = 98;
pub const IMAGE_COM1_HI: C2RustUnnamed_0 = 97;
pub const IMAGE_COM4: C2RustUnnamed_0 = 96;
pub const IMAGE_COM3: C2RustUnnamed_0 = 95;
pub const IMAGE_COM2: C2RustUnnamed_0 = 94;
pub const IMAGE_COM1: C2RustUnnamed_0 = 93;
pub const IMAGE_TFONT_126: C2RustUnnamed_0 = 92;
pub const IMAGE_TFONT_125: C2RustUnnamed_0 = 91;
pub const IMAGE_TFONT_124: C2RustUnnamed_0 = 90;
pub const IMAGE_TFONT_123: C2RustUnnamed_0 = 89;
pub const IMAGE_TFONT_122: C2RustUnnamed_0 = 88;
pub const IMAGE_TFONT_121: C2RustUnnamed_0 = 87;
pub const IMAGE_TFONT_120: C2RustUnnamed_0 = 86;
pub const IMAGE_TFONT_119: C2RustUnnamed_0 = 85;
pub const IMAGE_TFONT_118: C2RustUnnamed_0 = 84;
pub const IMAGE_TFONT_117: C2RustUnnamed_0 = 83;
pub const IMAGE_TFONT_116: C2RustUnnamed_0 = 82;
pub const IMAGE_TFONT_115: C2RustUnnamed_0 = 81;
pub const IMAGE_TFONT_114: C2RustUnnamed_0 = 80;
pub const IMAGE_TFONT_113: C2RustUnnamed_0 = 79;
pub const IMAGE_TFONT_112: C2RustUnnamed_0 = 78;
pub const IMAGE_TFONT_111: C2RustUnnamed_0 = 77;
pub const IMAGE_TFONT_110: C2RustUnnamed_0 = 76;
pub const IMAGE_TFONT_109: C2RustUnnamed_0 = 75;
pub const IMAGE_TFONT_108: C2RustUnnamed_0 = 74;
pub const IMAGE_TFONT_107: C2RustUnnamed_0 = 73;
pub const IMAGE_TFONT_106: C2RustUnnamed_0 = 72;
pub const IMAGE_TFONT_105: C2RustUnnamed_0 = 71;
pub const IMAGE_TFONT_104: C2RustUnnamed_0 = 70;
pub const IMAGE_TFONT_103: C2RustUnnamed_0 = 69;
pub const IMAGE_TFONT_102: C2RustUnnamed_0 = 68;
pub const IMAGE_TFONT_101: C2RustUnnamed_0 = 67;
pub const IMAGE_TFONT_100: C2RustUnnamed_0 = 66;
pub const IMAGE_TFONT_99: C2RustUnnamed_0 = 65;
pub const IMAGE_TFONT_98: C2RustUnnamed_0 = 64;
pub const IMAGE_TFONT_97: C2RustUnnamed_0 = 63;
pub const IMAGE_TFONT_96: C2RustUnnamed_0 = 62;
pub const IMAGE_TFONT_95: C2RustUnnamed_0 = 61;
pub const IMAGE_TFONT_94: C2RustUnnamed_0 = 60;
pub const IMAGE_TFONT_93: C2RustUnnamed_0 = 59;
pub const IMAGE_TFONT_92: C2RustUnnamed_0 = 58;
pub const IMAGE_TFONT_91: C2RustUnnamed_0 = 57;
pub const IMAGE_TFONT_90: C2RustUnnamed_0 = 56;
pub const IMAGE_TFONT_89: C2RustUnnamed_0 = 55;
pub const IMAGE_TFONT_88: C2RustUnnamed_0 = 54;
pub const IMAGE_TFONT_87: C2RustUnnamed_0 = 53;
pub const IMAGE_TFONT_86: C2RustUnnamed_0 = 52;
pub const IMAGE_TFONT_85: C2RustUnnamed_0 = 51;
pub const IMAGE_TFONT_84: C2RustUnnamed_0 = 50;
pub const IMAGE_TFONT_83: C2RustUnnamed_0 = 49;
pub const IMAGE_TFONT_82: C2RustUnnamed_0 = 48;
pub const IMAGE_TFONT_81: C2RustUnnamed_0 = 47;
pub const IMAGE_TFONT_80: C2RustUnnamed_0 = 46;
pub const IMAGE_TFONT_79: C2RustUnnamed_0 = 45;
pub const IMAGE_TFONT_78: C2RustUnnamed_0 = 44;
pub const IMAGE_TFONT_77: C2RustUnnamed_0 = 43;
pub const IMAGE_TFONT_76: C2RustUnnamed_0 = 42;
pub const IMAGE_TFONT_75: C2RustUnnamed_0 = 41;
pub const IMAGE_TFONT_74: C2RustUnnamed_0 = 40;
pub const IMAGE_TFONT_73: C2RustUnnamed_0 = 39;
pub const IMAGE_TFONT_72: C2RustUnnamed_0 = 38;
pub const IMAGE_TFONT_71: C2RustUnnamed_0 = 37;
pub const IMAGE_TFONT_70: C2RustUnnamed_0 = 36;
pub const IMAGE_TFONT_69: C2RustUnnamed_0 = 35;
pub const IMAGE_TFONT_68: C2RustUnnamed_0 = 34;
pub const IMAGE_TFONT_67: C2RustUnnamed_0 = 33;
pub const IMAGE_TFONT_66: C2RustUnnamed_0 = 32;
pub const IMAGE_TFONT_65: C2RustUnnamed_0 = 31;
pub const IMAGE_TFONT_64: C2RustUnnamed_0 = 30;
pub const IMAGE_TFONT_62: C2RustUnnamed_0 = 29;
pub const IMAGE_TFONT_61: C2RustUnnamed_0 = 28;
pub const IMAGE_TFONT_60: C2RustUnnamed_0 = 27;
pub const IMAGE_TFONT_59: C2RustUnnamed_0 = 26;
pub const IMAGE_TFONT_58: C2RustUnnamed_0 = 25;
pub const IMAGE_TFONT_57: C2RustUnnamed_0 = 24;
pub const IMAGE_TFONT_56: C2RustUnnamed_0 = 23;
pub const IMAGE_TFONT_55: C2RustUnnamed_0 = 22;
pub const IMAGE_TFONT_54: C2RustUnnamed_0 = 21;
pub const IMAGE_TFONT_53: C2RustUnnamed_0 = 20;
pub const IMAGE_TFONT_52: C2RustUnnamed_0 = 19;
pub const IMAGE_TFONT_51: C2RustUnnamed_0 = 18;
pub const IMAGE_TFONT_50: C2RustUnnamed_0 = 17;
pub const IMAGE_TFONT_49: C2RustUnnamed_0 = 16;
pub const IMAGE_TFONT_48: C2RustUnnamed_0 = 15;
pub const IMAGE_TFONT_47: C2RustUnnamed_0 = 14;
pub const IMAGE_TFONT_46: C2RustUnnamed_0 = 13;
pub const IMAGE_TFONT_44: C2RustUnnamed_0 = 12;
pub const IMAGE_TFONT_43: C2RustUnnamed_0 = 11;
pub const IMAGE_TFONT_42: C2RustUnnamed_0 = 10;
pub const IMAGE_TFONT_41: C2RustUnnamed_0 = 9;
pub const IMAGE_TFONT_40: C2RustUnnamed_0 = 8;
pub const IMAGE_TFONT_39: C2RustUnnamed_0 = 7;
pub const IMAGE_TFONT_38: C2RustUnnamed_0 = 6;
pub const IMAGE_TFONT_37: C2RustUnnamed_0 = 5;
pub const IMAGE_TFONT_36: C2RustUnnamed_0 = 4;
pub const IMAGE_TFONT_35: C2RustUnnamed_0 = 3;
pub const IMAGE_TFONT_34: C2RustUnnamed_0 = 2;
pub const IMAGE_TFONT_33: C2RustUnnamed_0 = 1;
#[no_mangle]
pub static mut StartWithGame: libc::c_int = 1 as libc::c_int;
// New game starts in Cam 1.
#[no_mangle]
pub static mut OnString: [libc::c_char; 4] = [79, 110, 32, 0];
#[no_mangle]
pub static mut OffString: [libc::c_char; 4] = [79, 102, 102, 0];
#[no_mangle]
pub static mut strFog: [STRING; 256] = [0; 256];
#[no_mangle]
pub static mut strTrans: [STRING; 256] = [0; 256];
#[no_mangle]
pub static mut titleMode: tMode = TITLE;
// the global case
// ////////////////////////////////////////////////////////////////////////////
// Local Definitions
// iPalette			titlePalette;
#[no_mangle]
pub static mut FEFont: libc::c_int = 0;
//int				FEBigFont;
#[no_mangle]
pub static mut pLevelName: [libc::c_char; 257] = [0; 257];
//256];			// vital! the wrf file to use.
#[no_mangle]
pub static mut bForceEditorLoaded: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut bUsingKeyboard: BOOL = 0 as libc::c_int;
// to disable mouse pointer when using keys.
#[no_mangle]
pub static mut bUsingSlider: BOOL = 0 as libc::c_int;
// Returns TRUE if escape key pressed on PC or close button pressed on Playstation.
//
#[no_mangle]
pub unsafe extern "C" fn CancelPressed() -> BOOL {
    if keyPressed(KEY_ESC) != 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Function Definitions
// ////////////////////////////////////////////////////////////////////////////
// for cursorsnap stuff on pc
#[no_mangle]
pub unsafe extern "C" fn processFrontendSnap(mut bHideCursor: BOOL) {
    static mut point: POINT = POINT{x: 0, y: 0,};
    static mut opoint: POINT = POINT{x: 0, y: 0,};
    point.x = mouseX();
    point.y = mouseY();
    if point.x != opoint.x || point.y != opoint.y {
        bUsingKeyboard = 0 as libc::c_int
    }
    if bUsingSlider == 0 {
        if keyPressed(KEY_RIGHTARROW) != 0 {
            bUsingKeyboard = 1 as libc::c_int;
            GotoDirectionalSnap(&mut InterfaceSnap, SNAP_RIGHT,
                                0 as libc::c_int as SWORD,
                                0 as libc::c_int as SWORD);
        } else if keyPressed(KEY_LEFTARROW) != 0 {
            bUsingKeyboard = 1 as libc::c_int;
            GotoDirectionalSnap(&mut InterfaceSnap, SNAP_LEFT,
                                0 as libc::c_int as SWORD,
                                0 as libc::c_int as SWORD);
        }
    }
    if keyPressed(KEY_UPARROW) != 0 {
        bUsingKeyboard = 1 as libc::c_int;
        bUsingSlider = 0 as libc::c_int;
        GotoDirectionalSnap(&mut InterfaceSnap, SNAP_UP,
                            0 as libc::c_int as SWORD,
                            0 as libc::c_int as SWORD);
    } else if keyPressed(KEY_DOWNARROW) != 0 {
        bUsingKeyboard = 1 as libc::c_int;
        bUsingSlider = 0 as libc::c_int;
        GotoDirectionalSnap(&mut InterfaceSnap, SNAP_DOWN,
                            0 as libc::c_int as SWORD,
                            0 as libc::c_int as SWORD);
    }
    if keyDown(KEY_LALT) == 0 && keyDown(KEY_RALT) == 0 &&
           (*psWScreen).psFocus.is_null() {
        if keyPressed(KEY_RETURN) != 0 {
            bUsingKeyboard = 1 as libc::c_int;
            setMouseDown(MOUSE_LMB);
        }
        if keyReleased(KEY_RETURN) != 0 {
            bUsingKeyboard = 1 as libc::c_int;
            setMouseUp(MOUSE_LMB);
        }
    }
    if bHideCursor == 0 { bUsingKeyboard = 0 as libc::c_int }
    opoint.x = mouseX();
    opoint.y = mouseY();
}
//256];			// vital! the wrf file to use.
// to disable mouse pointer when using keys.
// ////////////////////////////////////////////////////////////////////////////
// Change Mode
#[no_mangle]
pub unsafe extern "C" fn changeTitleMode(mut mode: tMode) {
    let mut oldMode: tMode = TITLE; // delete backdrop.
    widgDelete(psWScreen, 20000 as libc::c_int as UDWORD); // store old mode
    oldMode = titleMode; // set new mode
    titleMode = mode;
    match mode as libc::c_uint {
        1 => {
            /*	case DEMOMODE:// demo case. remove for release
		startDemoMenu();
		break;
	case VIDEO:
		startVideoOptionsMenu();
		break;
*/
            startSinglePlayerMenu();
        }
        4 => { startGameOptionsMenu(); }
        17 => { startGameOptions2Menu(); }
        18 => { startGameOptions3Menu(); }
        5 => { startTutorialMenu(); }
        3 => { startOptionsMenu(); }
        0 => { startTitleMenu(); }
        6 => {
            //	case GRAPHICS:
//		startGraphicsOptionsMenu();
//		break;
            startCreditsScreen(0 as libc::c_int); // goto multiplayer menu
        }
        2 => { startMultiPlayerMenu(); }
        7 => { startConnectionScreen(); }
        9 => { bUsingKeyboard = 0 as libc::c_int; startForceSelect(); }
        8 => {
            bUsingKeyboard = 0 as libc::c_int;
            if oldMode as libc::c_uint ==
                   MULTILIMIT as libc::c_int as libc::c_uint {
                startMultiOptions(1 as libc::c_int);
            } else { startMultiOptions(0 as libc::c_int); }
        }
        10 => { bUsingKeyboard = 0 as libc::c_int; startGameFind(); }
        11 => { bUsingKeyboard = 0 as libc::c_int; startLimitScreen(); }
        16 => {
            bUsingKeyboard = 0 as libc::c_int;
            startKeyMapEditor(1 as libc::c_int);
        }
        12 | 14 | 15 => {
            bUsingKeyboard = 0 as libc::c_int;
            bForceEditorLoaded = 0 as libc::c_int
        }
        13 => { }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown title mode requested\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// Title Screen
#[no_mangle]
pub unsafe extern "C" fn startTitleMenu() -> BOOL {
    //	widgDelete(psWScreen,1);	// close reticule if it's open. MAGIC NUMBERS?
    intRemoveReticule(); // Run the current set of widgets
    addBackdrop(); // show the widgets currently running
    addTopForm();
    addBottomForm();
    addTextButton(20003 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_SINGLE as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    if bDisableLobby == 0 {
        addTextButton(20004 as libc::c_int as UDWORD,
                      20 as libc::c_int as UDWORD,
                      90 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_MULTI as libc::c_int as UDWORD),
                      0 as libc::c_int, 0 as libc::c_int);
    } else {
        addTextButton(20004 as libc::c_int as UDWORD,
                      20 as libc::c_int as UDWORD,
                      90 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_MULTI as libc::c_int as UDWORD),
                      0 as libc::c_int, 1 as libc::c_int);
    }
    addTextButton(20019 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_TUT as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20006 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  170 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_OPTIONS as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20005 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  210 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_QUIT as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDEMAIN as libc::c_int as UDWORD));
    SetMousePos(0 as libc::c_int as UDWORD, 320 as libc::c_int as UDWORD,
                (170 as libc::c_int + 50 as libc::c_int) as UDWORD);
    SnapToID(&mut InterfaceSnap, 4 as libc::c_int as UWORD);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runTitleMenu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(1 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20005 => { changeTitleMode(CREDITS); }
        20004 => { changeTitleMode(MULTI); }
        20003 => { changeTitleMode(SINGLE); }
        20006 => { changeTitleMode(OPTIONS); }
        20019 => { changeTitleMode(TUTORIAL); }
        _ => { }
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
//BOOL		runDemoMenu				(VOID);
//BOOL		startDemoMenu			(VOID);
// ////////////////////////////////////////////////////////////////////////////
// Tutorial Menu
#[no_mangle]
pub unsafe extern "C" fn startTutorialMenu() -> BOOL {
    addBackdrop(); // Run the current set of widgets
    addTopForm();
    addBottomForm();
    addTextButton(20019 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_TUT as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20020 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_FASTPLAY as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDETUT as libc::c_int as UDWORD));
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    SetCurrentSnapID(&mut InterfaceSnap, 20020 as libc::c_int as UDWORD);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runTutorialMenu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(1 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20019 => {
            strcpy(pLevelName.as_mut_ptr(),
                   b"TUTORIAL3\x00" as *const u8 as *const libc::c_char);
            changeTitleMode(STARTGAME);
        }
        20020 => {
            strcpy(pLevelName.as_mut_ptr(),
                   b"FASTPLAY\x00" as *const u8 as *const libc::c_char);
            changeTitleMode(STARTGAME);
        }
        20005 => { changeTitleMode(TITLE); }
        _ => { }
    }
    // If close button pressed then return from this menu.
    if CancelPressed() != 0 {
        changeTitleMode(TITLE); // show the widgets currently running
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Single Player Menu
#[no_mangle]
pub unsafe extern "C" fn startSinglePlayerMenu() {
    addBackdrop();
    addTopForm();
    addBottomForm();
    addTextButton(20012 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_LOAD as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20011 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_NEW as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDESINGLE1 as libc::c_int as UDWORD));
    SetCurrentSnapID(&mut InterfaceSnap, 20012 as libc::c_int as UDWORD);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn endSinglePlayerMenu() {
    removeTopForm();
    removeBottomForm();
    removeBackdrop();
}
#[no_mangle]
pub unsafe extern "C" fn frontEndNewGame() {
    match StartWithGame {
        1 => {
            strcpy(pLevelName.as_mut_ptr(),
                   b"CAM_1A\x00" as *const u8 as *const libc::c_char);
            seq_ClearSeqList();
            seq_AddSeqToList(b"CAM1\\c001.rpl\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as *mut STRING,
                             b"CAM1\\c001.txa\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as libc::c_int, 0 as libc::c_int as UDWORD);
            seq_StartNextFullScreenVideo();
        }
        2 => {
            strcpy(pLevelName.as_mut_ptr(),
                   b"CAM_2A\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            strcpy(pLevelName.as_mut_ptr(),
                   b"CAM_3A\x00" as *const u8 as *const libc::c_char);
        }
        _ => { }
    }
    changeTitleMode(STARTGAME);
}
#[no_mangle]
pub unsafe extern "C" fn loadOK() {
    if strlen(sRequestResult.as_mut_ptr()) != 0 {
        strcpy(saveGameName.as_mut_ptr(), sRequestResult.as_mut_ptr());
        changeTitleMode(LOADSAVEGAME);
    }
    SetCurrentSnapID(&mut InterfaceSnap, 20012 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn runSinglePlayerMenu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(1 as libc::c_int);
    if bLoadSaveUp != 0 {
        if runLoadSave(0 as libc::c_int) != 0 {
            // check for file name.
            loadOK(); // Run the current set of widgets
            SetCurrentSnapID(&mut InterfaceSnap,
                             20012 as libc::c_int as
                                 UDWORD); // change mode when loadsave returns
        }
    } else {
        id = widgRunScreen(psWScreen);
        match id {
            20011 => { frontEndNewGame(); }
            20080 => {
                strcpy(pLevelName.as_mut_ptr(),
                       b"CAM_2A\x00" as *const u8 as *const libc::c_char);
                changeTitleMode(STARTGAME);
                initLoadingScreen(1 as libc::c_int, 1 as libc::c_int);
            }
            20081 => {
                strcpy(pLevelName.as_mut_ptr(),
                       b"CAM_3A\x00" as *const u8 as *const libc::c_char);
                changeTitleMode(STARTGAME);
                initLoadingScreen(1 as libc::c_int, 1 as libc::c_int);
            }
            20012 => {
                addLoadSave(LOAD_FRONTEND, SaveGamePath.as_mut_ptr(),
                            b"gam\x00" as *const u8 as *const libc::c_char as
                                *mut CHAR,
                            strresGetString(psStringRes,
                                            STR_MR_LOAD_GAME as libc::c_int as
                                                UDWORD));
            }
            20005 => { changeTitleMode(TITLE); }
            _ => { }
        }
        if CancelPressed() != 0 { changeTitleMode(TITLE); }
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    if bLoadSaveUp == 0 {
        // if save/load screen is up
        widgDisplayScreen(psWScreen);
        // show the widgets currently running
    }
    if bLoadSaveUp != 0 {
        // if save/load screen is up
        displayLoadSave();
    }
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Multi Player Menu
#[no_mangle]
pub unsafe extern "C" fn startMultiPlayerMenu() -> BOOL {
    addBackdrop(); // Run the current set of widgets
    addTopForm();
    addBottomForm();
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDEMULTI as libc::c_int as UDWORD));
    addTextButton(20007 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_HOST as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20008 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_JOIN as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20009 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_FORCEEDIT as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20010 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  170 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_SKIRMISH as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    SetMousePos(0 as libc::c_int as UDWORD, 320 as libc::c_int as UDWORD,
                (170 as libc::c_int + 90 as libc::c_int) as UDWORD);
    SnapToID(&mut InterfaceSnap, 3 as libc::c_int as UWORD);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runMultiPlayerMenu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(1 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20007 => {
            ingame.bHostSetup = 1 as libc::c_int;
            changeTitleMode(PROTOCOL);
        }
        20008 => {
            ingame.bHostSetup = 0 as libc::c_int;
            changeTitleMode(PROTOCOL);
        }
        20009 => {
            if bForceEditorLoaded == 0 {
                initLoadingScreen(1 as libc::c_int, 1 as libc::c_int);
                /*			if (!resLoad("wrf\\forcedit.wrf", 500,
						 DisplayBuffer, displayBufferSize,
						 psGameHeap))				//need the object heaps to have been set up before loading
			{
				return FALSE;
			}
*/
                if resLoad(b"wrf\\piestats.wrf\x00" as *const u8 as
                               *const libc::c_char as *mut STRING,
                           501 as libc::c_int, DisplayBuffer,
                           displayBufferSize as SDWORD, psGameHeap) == 0 {
                    //need the object heaps to have been set up before loading
                    return 0 as libc::c_int
                }
                if resLoad(b"wrf\\forcedit2.wrf\x00" as *const u8 as
                               *const libc::c_char as *mut STRING,
                           502 as libc::c_int, DisplayBuffer,
                           displayBufferSize as SDWORD, psGameHeap) == 0 {
                    //need the object heaps to have been set up before loading
                    return 0 as libc::c_int
                } //skip draw.
                bForceEditorLoaded =
                    1 as libc::c_int; // pretend its a multiplayer.
                closeLoadingScreen();
            }
            changeTitleMode(FORCESELECT);
            return 1 as libc::c_int
        }
        20010 => {
            ingame.bHostSetup = 1 as libc::c_int;
            NETuseNetwork(0 as libc::c_int);
            //		strcpy(sPlayer,"LastUsed");					// initialize name string.
//		loadMultiStats(sPlayer,&nullStats);
//		NETchangePlayerName(1,sPlayer);
            changeTitleMode(MULTIOPTION); // show the widgets currently running
        }
        20005 => { changeTitleMode(TITLE); }
        _ => { }
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Options Menu
#[no_mangle]
pub unsafe extern "C" fn startOptionsMenu() -> BOOL {
    addBackdrop(); // Run the current set of widgets
    addTopForm();
    addBottomForm();
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDEOPTIONS as libc::c_int as UDWORD));
    addTextButton(20063 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_GAME as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20092 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_GRAPHICS as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20099 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_AUDIO as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addTextButton(20089 as libc::c_int as UDWORD, 20 as libc::c_int as UDWORD,
                  170 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_KM_KEYMAP as libc::c_int as UDWORD),
                  0 as libc::c_int, 0 as libc::c_int);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    SetMousePos(0 as libc::c_int as UDWORD, 320 as libc::c_int as UDWORD,
                (170 as libc::c_int + 90 as libc::c_int) as UDWORD);
    SnapToID(&mut InterfaceSnap, 3 as libc::c_int as UWORD);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runOptionsMenu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(1 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20063 => { changeTitleMode(GAME); }
        20092 => { changeTitleMode(GAME2); }
        20099 => { changeTitleMode(GAME3); }
        20089 => {
            //	case FRONTEND_VIDEO:
//		changeTitleMode(VIDEO);
//		break;
//	case FRONTEND_GRAPHICS:
//		changeTitleMode(GRAPHICS);
//		break;
            changeTitleMode(KEYMAP);
        }
        20005 => { changeTitleMode(TITLE); }
        _ => { }
    }
    // If close button pressed then return from this menu.
    if CancelPressed() != 0 {
        changeTitleMode(TITLE); // show the widgets currently running
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Game Options Menu 2!
#[no_mangle]
pub unsafe extern "C" fn startGameOptions2Menu() -> BOOL {
    addBackdrop();
    addTopForm();
    addBottomForm();
    // //////////
	// mouseflip
    addTextButton(20095 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_MFLIP as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    if getInvertMouseStatus() != 0 {
        // flipped
        addTextButton(20096 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      50 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_ON as libc::c_int as UDWORD),
                      1 as libc::c_int, 0 as libc::c_int);
    } else {
        // not flipped
        addTextButton(20096 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      50 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_OFF as libc::c_int as UDWORD),
                      1 as libc::c_int, 0 as libc::c_int);
    }
    // //////////
	// screenshake
    addTextButton(20093 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_SSHAKE as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    if getShakeStatus() != 0 {
        // shaking on
        addTextButton(20094 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      90 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_ON as libc::c_int as UDWORD),
                      1 as libc::c_int, 0 as libc::c_int);
    } else {
        //shaking off.
        addTextButton(20094 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      90 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_OFF as libc::c_int as UDWORD),
                      1 as libc::c_int, 0 as libc::c_int);
    }
    // //////////
	// fog
    addTextButton(20078 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_FOG as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    if war_GetFog() != 0 {
        addTextButton(20079 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      130 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_CRAPFOG as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      0 as libc::c_int);
    } else {
        addTextButton(20079 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      130 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_GOODFOG as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      0 as libc::c_int);
    }
    /*	////////////
	// fog
	addTextButton(FRONTEND_FOG,		FRONTEND_POS5X-15,FRONTEND_POS5Y, strresGetString(psStringRes, STR_FE_FOG),TRUE,FALSE);
	if (war_GetFog())
	{
		addTextButton(FRONTEND_FOG_R,	FRONTEND_POS5M-55,	FRONTEND_POS5Y, strresGetString(psStringRes,STR_FE_ON),TRUE,FALSE);
	}
	else
	{
		addTextButton(FRONTEND_FOG_R,	FRONTEND_POS5M-55,	FRONTEND_POS5Y, strresGetString(psStringRes,STR_FE_OFF),TRUE,FALSE);
	}
*/
    //	////////////
//	//sequence mode.
    addTextButton(20097 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                  210 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_SEQ_PLAYBACK as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    if war_GetSeqMode() as libc::c_uint ==
           SEQ_FULL as libc::c_int as libc::c_uint {
        addTextButton(20098 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      210 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_SEQ_FULL as libc::c_int as UDWORD),
                      1 as libc::c_int, 0 as libc::c_int);
    } else if war_GetSeqMode() as libc::c_uint ==
                  SEQ_SMALL as libc::c_int as libc::c_uint {
        addTextButton(20098 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      210 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_SEQ_WINDOW as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      0 as libc::c_int);
    } else {
        addTextButton(20098 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      210 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_SEQ_MINIMAL as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      0 as libc::c_int);
    }
    //	////////////
//	//translucency mode.
//	addTextButton(FRONTEND_TRANSPARENCY,	FRONTEND_POS5X-15,FRONTEND_POS5Y, strresGetString(psStringRes, STR_FE_TRANSPARENCY),TRUE,FALSE);
//	if (!war_GetTranslucent())
//	{
//		addTextButton(FRONTEND_TRANSPARENCY_R,	FRONTEND_POS5M-55,FRONTEND_POS5Y, strresGetString(psStringRes,STR_FE_OFF),TRUE,FALSE);
//	}
//	else if (!war_GetAdditive())
//	{
//	addTextButton(FRONTEND_TRANSPARENCY_R,	FRONTEND_POS5M-55,FRONTEND_POS5Y, strresGetString(psStringRes,STR_COMPATIBLE),TRUE,FALSE);
//	}
//	else
//	{
//	addTextButton(FRONTEND_TRANSPARENCY_R,	FRONTEND_POS5M-55,FRONTEND_POS5Y, strresGetString(psStringRes,STR_FE_ON),TRUE,FALSE);
//	}
    // //////////
	//subtitle mode.
    if war_GetAllowSubtitles() != 0 {
        addTextButton(20069 as libc::c_int as UDWORD,
                      (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                      170 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_SUBTITLES as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      0 as libc::c_int);
    } else {
        addTextButton(20069 as libc::c_int as UDWORD,
                      (20 as libc::c_int - 35 as libc::c_int) as UDWORD,
                      170 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_SUBTITLES as libc::c_int as
                                          UDWORD), 1 as libc::c_int,
                      1 as libc::c_int);
    }
    if war_GetAllowSubtitles() != 0 {
        if seq_GetSubtitles() == 0 {
            addTextButton(20070 as libc::c_int as UDWORD,
                          (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                          170 as libc::c_int as UDWORD,
                          strresGetString(psStringRes,
                                          STR_FE_OFF as libc::c_int as
                                              UDWORD), 1 as libc::c_int,
                          0 as libc::c_int);
        } else {
            addTextButton(20070 as libc::c_int as UDWORD,
                          (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                          170 as libc::c_int as UDWORD,
                          strresGetString(psStringRes,
                                          STR_FE_ON as libc::c_int as UDWORD),
                          1 as libc::c_int, 0 as libc::c_int);
        }
    } else {
        addTextButton(20070 as libc::c_int as UDWORD,
                      (290 as libc::c_int - 55 as libc::c_int) as UDWORD,
                      170 as libc::c_int as UDWORD,
                      strresGetString(psStringRes,
                                      STR_FE_OFF as libc::c_int as UDWORD),
                      1 as libc::c_int, 1 as libc::c_int);
    }
    // //////////
	// quit.
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD,
                1 as libc::c_int); // Run the current set of widgets
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn runGameOptions2Menu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(0 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20093 | 20094 => {
            if getShakeStatus() != 0 {
                setShakeStatus(0 as libc::c_int);
                widgSetString(psWScreen, 20094 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_OFF as libc::c_int as
                                                  UDWORD));
            } else {
                setShakeStatus(1 as libc::c_int);
                widgSetString(psWScreen, 20094 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_ON as libc::c_int as
                                                  UDWORD));
            }
        }
        20095 | 20096 => {
            if getInvertMouseStatus() != 0 {
                //	 flipped
                setInvertMouseStatus(0 as libc::c_int);
                widgSetString(psWScreen, 20096 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_OFF as libc::c_int as
                                                  UDWORD));
            } else {
                // not flipped
                setInvertMouseStatus(1 as libc::c_int);
                widgSetString(psWScreen, 20096 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_ON as libc::c_int as
                                                  UDWORD));
            }
        }
        20078 | 20079 => {
            if war_GetFog() != 0 {
                // turn off crap fog, turn on vis fog.
                war_SetFog(0 as libc::c_int);
                avSetStatus(1 as libc::c_int);
                widgSetString(psWScreen, 20079 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_GOODFOG as libc::c_int as
                                                  UDWORD));
            } else {
                // turn off vis fog, turn on normal crap fog.
                avSetStatus(0 as libc::c_int);
                war_SetFog(1 as libc::c_int);
                widgSetString(psWScreen, 20079 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_CRAPFOG as libc::c_int as
                                                  UDWORD));
            }
        }
        20005 => { changeTitleMode(OPTIONS); }
        20069 | 20070 => {
            if seq_GetSubtitles() != 0 {
                // turn off
                seq_SetSubtitles(0 as libc::c_int);
                widgSetString(psWScreen, 20070 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_OFF as libc::c_int as
                                                  UDWORD));
            } else {
                // turn on
                seq_SetSubtitles(1 as libc::c_int);
                widgSetString(psWScreen, 20070 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_FE_ON as libc::c_int as
                                                  UDWORD));
            }
        }
        20097 | 20098 => {
            /*	case FRONTEND_FOG:
	case FRONTEND_FOG_R:
		if (war_GetFog())
		{
			war_SetFog(FALSE);
			widgSetString(psWScreen,FRONTEND_FOG_R,strresGetString(psStringRes,STR_FE_OFF));
		}
		else
		{
			war_SetFog(TRUE);
			widgSetString(psWScreen,FRONTEND_FOG_R,strresGetString(psStringRes,STR_FE_ON));
		}
		break;
*/
            //	case FRONTEND_TRANSPARENCY:
//	case FRONTEND_TRANSPARENCY_R:
//		if (!war_GetTranslucent())
//		{
//			war_SetTranslucent(TRUE);
//			war_SetAdditive(FALSE);
//			widgSetString(psWScreen,FRONTEND_TRANSPARENCY_R, strresGetString(psStringRes,STR_COMPATIBLE));
//		}
//		else if (!war_GetAdditive())
//		{
//			war_SetTranslucent(TRUE);
//			war_SetAdditive(TRUE);
//			widgSetString(psWScreen,FRONTEND_TRANSPARENCY_R,strresGetString(psStringRes,STR_FE_ON));
//		}
//		else
//		{
//			war_SetTranslucent(FALSE);
//			war_SetAdditive(FALSE);
//			widgSetString(psWScreen,FRONTEND_TRANSPARENCY_R,strresGetString(psStringRes,STR_FE_OFF));
//		}
//		break;
            if war_GetSeqMode() as libc::c_uint ==
                   SEQ_FULL as libc::c_int as libc::c_uint {
                war_SetSeqMode(SEQ_SMALL);
                widgSetString(psWScreen, 20098 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_SEQ_WINDOW as libc::c_int as
                                                  UDWORD));
            } else if war_GetSeqMode() as libc::c_uint ==
                          SEQ_SMALL as libc::c_int as libc::c_uint {
                war_SetSeqMode(SEQ_SKIP);
                widgSetString(psWScreen, 20098 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_SEQ_MINIMAL as libc::c_int
                                                  as UDWORD));
            } else {
                war_SetSeqMode(SEQ_FULL);
                widgSetString(psWScreen, 20098 as libc::c_int as UDWORD,
                              strresGetString(psStringRes,
                                              STR_SEQ_FULL as libc::c_int as
                                                  UDWORD));
            }
        }
        _ => { }
    }
    // If close button pressed then return from this menu.
    if CancelPressed() != 0 {
        changeTitleMode(OPTIONS); // show the widgets currently running
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Game Options Menu
#[no_mangle]
pub unsafe extern "C" fn startGameOptions3Menu() -> BOOL {
    addBackdrop();
    addTopForm();
    addBottomForm();
    // 2d audio
    addTextButton(20017 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_FX as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    addFESlider(20024 as libc::c_int as UDWORD,
                20002 as libc::c_int as UDWORD, 290 as libc::c_int as UDWORD,
                (50 as libc::c_int + 5 as libc::c_int) as UDWORD,
                100 as libc::c_long as UDWORD, mixer_GetWavVolume() as UDWORD,
                20017 as libc::c_int as UDWORD);
    // 3d audio
    addTextButton(20016 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_3D_FX as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    addFESlider(20023 as libc::c_int as UDWORD,
                20002 as libc::c_int as UDWORD, 290 as libc::c_int as UDWORD,
                (90 as libc::c_int + 5 as libc::c_int) as UDWORD,
                100 as libc::c_long as UDWORD,
                mixer_Get3dWavVolume() as UDWORD,
                20016 as libc::c_int as UDWORD);
    // cd audio
    addTextButton(20018 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_MUSIC as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    addFESlider(20025 as libc::c_int as UDWORD,
                20002 as libc::c_int as UDWORD, 290 as libc::c_int as UDWORD,
                (130 as libc::c_int + 5 as libc::c_int) as UDWORD,
                100 as libc::c_long as UDWORD, mixer_GetCDVolume() as UDWORD,
                20018 as libc::c_int as UDWORD);
    // quit.
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    //add some text down the side of the form
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDEOPTIONS as libc::c_int as
                                    UDWORD)); // Run the current set of widgets
    return 1 as libc::c_int; // move mouse
}
#[no_mangle]
pub unsafe extern "C" fn runGameOptions3Menu() -> BOOL {
    let mut id: UDWORD = 0;
    processFrontendSnap(0 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20017 | 20016 | 20018 => {
            SetMousePos(0 as libc::c_int as UDWORD,
                        (80 as libc::c_int + 290 as libc::c_int +
                             5 as libc::c_int) as UDWORD,
                        (mouseY() - 3 as libc::c_int) as UDWORD);
        }
        20024 => {
            mixer_SetWavVolume(widgGetSliderPos(psWScreen,
                                                20024 as libc::c_int as
                                                    UDWORD) as SDWORD);
        }
        20023 => {
            mixer_Set3dWavVolume(widgGetSliderPos(psWScreen,
                                                  20023 as libc::c_int as
                                                      UDWORD) as SDWORD);
        }
        20025 => {
            mixer_SetCDVolume(widgGetSliderPos(psWScreen,
                                               20025 as libc::c_int as UDWORD)
                                  as SDWORD);
        }
        20005 => { changeTitleMode(OPTIONS); }
        _ => { }
    }
    // If close button pressed then return from this menu.
    if CancelPressed() != 0 {
        changeTitleMode(TITLE); // show the widgets currently running
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Game Options Menu
#[no_mangle]
pub unsafe extern "C" fn startGameOptionsMenu() -> BOOL {
    let mut w: UDWORD = 0;
    let mut h: UDWORD = 0;
    addBackdrop();
    addTopForm();
    addBottomForm();
    // difficulty
    addTextButton(20074 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  50 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_DIFFICULTY as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    match getDifficultyLevel() as libc::c_uint {
        0 => {
            addTextButton(20075 as libc::c_int as UDWORD,
                          (290 as libc::c_int - 25 as libc::c_int) as UDWORD,
                          50 as libc::c_int as UDWORD,
                          strresGetString(psStringRes,
                                          STR_EASY as libc::c_int as UDWORD),
                          1 as libc::c_int, 0 as libc::c_int);
        }
        1 => {
            addTextButton(20075 as libc::c_int as UDWORD,
                          (290 as libc::c_int - 25 as libc::c_int) as UDWORD,
                          50 as libc::c_int as UDWORD,
                          strresGetString(psStringRes,
                                          STR_NORMAL as libc::c_int as
                                              UDWORD), 1 as libc::c_int,
                          0 as libc::c_int);
        }
        2 | _ => {
            addTextButton(20075 as libc::c_int as UDWORD,
                          (290 as libc::c_int - 25 as libc::c_int) as UDWORD,
                          50 as libc::c_int as UDWORD,
                          strresGetString(psStringRes,
                                          STR_HARD as libc::c_int as UDWORD),
                          1 as libc::c_int, 0 as libc::c_int);
        }
    }
    // scroll speed.
    addTextButton(20015 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  90 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_SCROLL as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    addFESlider(20022 as libc::c_int as UDWORD,
                20002 as libc::c_int as UDWORD, 290 as libc::c_int as UDWORD,
                (90 as libc::c_int + 5 as libc::c_int) as UDWORD,
                16 as libc::c_int as UDWORD,
                scroll_speed_accel.wrapping_div(100 as libc::c_int as
                                                    libc::c_uint),
                20015 as libc::c_int as UDWORD);
    // colour stuff
    w =
        iV_GetImageWidth(FrontImages, IMAGE_PLAYER0 as libc::c_int as UWORD)
            as UDWORD;
    h =
        iV_GetImageHeight(FrontImages, IMAGE_PLAYER0 as libc::c_int as UWORD)
            as UDWORD;
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20031 as libc::c_int as UDWORD,
                (290 as libc::c_int as
                     libc::c_uint).wrapping_add((0 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(w.wrapping_add(6
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))),
                130 as libc::c_int as UDWORD, w, h,
                0 as libc::c_int as UDWORD,
                IMAGE_PLAYER0 as libc::c_int as UDWORD,
                IMAGE_PLAYERX as libc::c_int as UDWORD, 1 as libc::c_int);
    //	addMultiBut(psWScreen,FRONTEND_BOTFORM,FE_P1, FRONTEND_POS6M-(3*(w+4)),FRONTEND_POS6Y,w,h,0,IMAGE_PLAYER1	,IMAGE_HI34,TRUE);
//	addMultiBut(psWScreen,FRONTEND_BOTFORM,FE_P2, FRONTEND_POS6M-(2*(w+4)),FRONTEND_POS6Y,w,h,0,IMAGE_PLAYER2	,IMAGE_HI34,TRUE);
//	addMultiBut(psWScreen,FRONTEND_BOTFORM,FE_P3, FRONTEND_POS6M-(1*(w+4)),FRONTEND_POS6Y,w,h,0,IMAGE_PLAYER3	,IMAGE_HI34,TRUE);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20035 as libc::c_int as UDWORD,
                (290 as libc::c_int as
                     libc::c_uint).wrapping_add((1 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(w.wrapping_add(6
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))),
                130 as libc::c_int as UDWORD, w, h,
                0 as libc::c_int as UDWORD,
                IMAGE_PLAYER4 as libc::c_int as UDWORD,
                IMAGE_PLAYERX as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20036 as libc::c_int as UDWORD,
                (290 as libc::c_int as
                     libc::c_uint).wrapping_add((2 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(w.wrapping_add(6
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))),
                130 as libc::c_int as UDWORD, w, h,
                0 as libc::c_int as UDWORD,
                IMAGE_PLAYER5 as libc::c_int as UDWORD,
                IMAGE_PLAYERX as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20037 as libc::c_int as UDWORD,
                (290 as libc::c_int as
                     libc::c_uint).wrapping_add((3 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(w.wrapping_add(6
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))),
                130 as libc::c_int as UDWORD, w, h,
                0 as libc::c_int as UDWORD,
                IMAGE_PLAYER6 as libc::c_int as UDWORD,
                IMAGE_PLAYERX as libc::c_int as UDWORD, 1 as libc::c_int);
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20038 as libc::c_int as UDWORD,
                (290 as libc::c_int as
                     libc::c_uint).wrapping_add((4 as libc::c_int as
                                                     libc::c_uint).wrapping_mul(w.wrapping_add(6
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint))),
                130 as libc::c_int as UDWORD, w, h,
                0 as libc::c_int as UDWORD,
                IMAGE_PLAYER7 as libc::c_int as UDWORD,
                IMAGE_PLAYERX as libc::c_int as UDWORD, 1 as libc::c_int);
    widgSetButtonState(psWScreen,
                       (20031 as libc::c_int +
                            getPlayerColour(0 as libc::c_int as UDWORD) as
                                libc::c_int) as UDWORD,
                       0x2 as libc::c_int as UDWORD);
    addTextButton(20073 as libc::c_int as UDWORD,
                  (20 as libc::c_int - 25 as libc::c_int) as UDWORD,
                  130 as libc::c_int as UDWORD,
                  strresGetString(psStringRes,
                                  STR_FE_CLAN as libc::c_int as UDWORD),
                  1 as libc::c_int, 0 as libc::c_int);
    // quit.
    addMultiBut(psWScreen, 20002 as libc::c_int as UDWORD,
                20005 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD,
                10 as libc::c_int as UDWORD, 30 as libc::c_int as UDWORD,
                29 as libc::c_int as UDWORD,
                STR_FE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN as libc::c_int as UDWORD,
                IMAGE_RETURN_HI as libc::c_int as UDWORD, 1 as libc::c_int);
    //add some text down the side of the form
    addSideText(20026 as libc::c_int as UDWORD, 44 as libc::c_int as UDWORD,
                170 as libc::c_int as UDWORD,
                strresGetString(psStringRes,
                                STR_FE_SIDEOPTIONS as libc::c_int as UDWORD));
    return 1 as libc::c_int;
}
//extern BOOL runVideoOptionsMenu		(VOID);
//extern BOOL runGraphicsOptionsMenu	(VOID);
#[no_mangle]
pub unsafe extern "C" fn runGameOptionsMenu() -> BOOL {
    let mut id: UDWORD = 0; // Run the current set of widgets
    processFrontendSnap(0 as libc::c_int);
    id = widgRunScreen(psWScreen);
    match id {
        20015 => {
            //	case FRONTEND_GAMMA:
            SetMousePos(0 as libc::c_int as UDWORD,
                        (80 as libc::c_int + 290 as libc::c_int +
                             5 as libc::c_int) as UDWORD,
                        (mouseY() - 3 as libc::c_int) as
                            UDWORD); // move mouse
        }
        20074 | 20075 => {
            /*	case FRONTEND_FOGTYPE:
	case FRONTEND_FOGTYPE_R:
	if( war_GetFog()	)
	{	// turn off crap fog, turn on vis fog.
		war_SetFog(FALSE);
		avSetStatus(TRUE);
		widgSetString(psWScreen,FRONTEND_FOGTYPE_R, strresGetString(psStringRes,STR_FE_GOODFOG));
	}
	else
	{	// turn off vis fog, turn on normal crap fog.
		avSetStatus(FALSE);
		war_SetFog(TRUE);
		widgSetString(psWScreen,FRONTEND_FOGTYPE_R, strresGetString(psStringRes,STR_FE_CRAPFOG));
	}
	break;
*/
            match getDifficultyLevel() as libc::c_uint {
                0 => {
                    setDifficultyLevel(DL_NORMAL); //0-1600
                    widgSetString(psWScreen, 20075 as libc::c_int as UDWORD,
                                  strresGetString(psStringRes,
                                                  STR_NORMAL as libc::c_int as
                                                      UDWORD));
                }
                1 => {
                    setDifficultyLevel(DL_HARD);
                    widgSetString(psWScreen, 20075 as libc::c_int as UDWORD,
                                  strresGetString(psStringRes,
                                                  STR_HARD as libc::c_int as
                                                      UDWORD));
                }
                2 => {
                    setDifficultyLevel(DL_EASY);
                    widgSetString(psWScreen, 20075 as libc::c_int as UDWORD,
                                  strresGetString(psStringRes,
                                                  STR_EASY as libc::c_int as
                                                      UDWORD));
                }
                _ => { }
            }
        }
        20022 => {
            scroll_speed_accel =
                widgGetSliderPos(psWScreen,
                                 20022 as libc::c_int as
                                     UDWORD).wrapping_mul(100 as libc::c_int
                                                              as
                                                              libc::c_uint);
            if scroll_speed_accel == 0 as libc::c_int as libc::c_uint {
                // make sure you CAN scroll.
                scroll_speed_accel = 100 as libc::c_int as UDWORD
            }
        }
        20005 => { changeTitleMode(OPTIONS); }
        20031 => {
            widgSetButtonState(psWScreen, 20031 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            //		widgSetButtonState(psWScreen, FE_P1, 0);
//		widgSetButtonState(psWScreen, FE_P2, 0);
//		widgSetButtonState(psWScreen, FE_P3, 0);
            widgSetButtonState(psWScreen, 20035 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20036 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20037 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20038 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            setPlayerColour(0 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
        }
        20035 => {
            widgSetButtonState(psWScreen, 20031 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            //	widgSetButtonState(psWScreen, FE_P1, 0);
	//	widgSetButtonState(psWScreen, FE_P2, 0);
	//	widgSetButtonState(psWScreen, FE_P3, 0);
            widgSetButtonState(psWScreen, 20035 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20036 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20037 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20038 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            setPlayerColour(0 as libc::c_int as UDWORD,
                            4 as libc::c_int as UDWORD);
        }
        20036 => {
            widgSetButtonState(psWScreen, 20031 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            //	widgSetButtonState(psWScreen, FE_P1, 0);
	//	widgSetButtonState(psWScreen, FE_P2, 0);
	//	widgSetButtonState(psWScreen, FE_P3, 0);
            widgSetButtonState(psWScreen, 20035 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20036 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20037 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20038 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            setPlayerColour(0 as libc::c_int as UDWORD,
                            5 as libc::c_int as UDWORD);
        }
        20037 => {
            widgSetButtonState(psWScreen, 20031 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            //	widgSetButtonState(psWScreen, FE_P1, 0);
	//	widgSetButtonState(psWScreen, FE_P2, 0);
	//	widgSetButtonState(psWScreen, FE_P3, 0);
            widgSetButtonState(psWScreen, 20035 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20036 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20037 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20038 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            setPlayerColour(0 as libc::c_int as UDWORD,
                            6 as libc::c_int as UDWORD);
        }
        20038 => {
            widgSetButtonState(psWScreen, 20031 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            //	widgSetButtonState(psWScreen, FE_P1, 0);
	//	widgSetButtonState(psWScreen, FE_P2, 0);
	//	widgSetButtonState(psWScreen, FE_P3, 0);
            widgSetButtonState(psWScreen, 20035 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20036 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20037 as libc::c_int as UDWORD,
                               0 as libc::c_int as UDWORD);
            widgSetButtonState(psWScreen, 20038 as libc::c_int as UDWORD,
                               0x2 as libc::c_int as UDWORD);
            setPlayerColour(0 as libc::c_int as UDWORD,
                            7 as libc::c_int as UDWORD);
        }
        _ => { }
    }
    // If close button pressed then return from this menu.
    if CancelPressed() != 0 {
        changeTitleMode(TITLE); // show the widgets currently running
    }
    DrawBegin();
    StartCursorSnap(&mut InterfaceSnap);
    widgDisplayScreen(psWScreen);
    DrawEnd();
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// common widgets.
#[no_mangle]
pub unsafe extern "C" fn addBackdrop() {
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
                   pFormDisplay: None,}; // Backdrop
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 20000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        pie_GetVideoBufferWidth().wrapping_sub(640 as libc::c_int as
                                                   libc::c_uint).wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
            as SWORD;
    sFormInit.y =
        pie_GetVideoBufferHeight().wrapping_sub(480 as libc::c_int as
                                                    libc::c_uint).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
            as SWORD;
    sFormInit.width = (640 as libc::c_int - 1 as libc::c_int) as UWORD;
    sFormInit.height = (480 as libc::c_int - 1 as libc::c_int) as UWORD;
    sFormInit.pDisplay =
        Some(displayTitleBitmap as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn removeBackdrop() {
    widgDelete(psWScreen, 20000 as libc::c_int as UDWORD);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addBottomForm() {
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
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 20002 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 80 as libc::c_int as SWORD;
    sFormInit.y = 170 as libc::c_int as SWORD;
    sFormInit.width = 480 as libc::c_int as UWORD;
    sFormInit.height = 300 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.disableChildren = 1 as libc::c_int;
    widgAddForm(psWScreen, &mut sFormInit);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn removeBottomForm() {
    widgDelete(psWScreen, 20002 as libc::c_int as UDWORD);
}
//BOOL		startVideoOptionsMenu	(VOID);
//BOOL		runVideoOptionsMenu		(VOID);
//BOOL		startGraphicsOptionsMenu(VOID);
//BOOL		runGraphicsptionsMenu	(VOID);
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addTopForm() {
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
                   pFormDisplay: None,}; //115;
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong); //18;
    sFormInit.formID = 20000 as libc::c_int as UDWORD;
    sFormInit.id = 20001 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    if titleMode as libc::c_uint == MULTIOPTION as libc::c_int as libc::c_uint
       {
        sFormInit.x = 28 as libc::c_int as SWORD;
        sFormInit.y = 10 as libc::c_int as SWORD;
        sFormInit.width = 588 as libc::c_int as UWORD;
        sFormInit.height = 150 as libc::c_int as UWORD
    } else {
        sFormInit.x = 80 as libc::c_int as SWORD;
        sFormInit.y = 10 as libc::c_int as SWORD;
        sFormInit.width = 480 as libc::c_int as UWORD;
        sFormInit.height = 150 as libc::c_int as UWORD
    }
    sFormInit.pDisplay =
        Some(intDisplayPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
    sFormInit.formID = 20001 as libc::c_int as UDWORD;
    sFormInit.id = 20062 as libc::c_int as UDWORD;
    sFormInit.x =
        (sFormInit.width as libc::c_int / 2 as libc::c_int -
             248 as libc::c_int / 2 as libc::c_int) as libc::c_short;
    sFormInit.y =
        (sFormInit.height as libc::c_int / 2 as libc::c_int -
             118 as libc::c_int / 2 as libc::c_int) as libc::c_short;
    sFormInit.width = 248 as libc::c_int as UWORD;
    sFormInit.height = 118 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(displayLogo as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psWScreen, &mut sFormInit);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn removeTopForm() {
    widgDelete(psWScreen, 20001 as libc::c_int as UDWORD);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addTextButton(mut id: UDWORD, mut PosX: UDWORD,
                                       mut PosY: UDWORD, mut txt: *mut STRING,
                                       mut bAlign: BOOL, mut bGrey: BOOL) {
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
                  FontID: 0,}; //FRONTEND_BUTWIDTH;
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as
               libc::c_ulong); // store disable state
    sButInit.formID = 20002 as libc::c_int as UDWORD;
    sButInit.id = id;
    sButInit.x = PosX as libc::c_short;
    sButInit.y = PosY as libc::c_short;
    if bAlign != 0 {
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.width =
            (iV_GetTextWidth(txt) + 10 as libc::c_int) as libc::c_short as
                UWORD;
        sButInit.x = (sButInit.x as libc::c_int + 35 as libc::c_int) as SWORD
    } else {
        sButInit.style = (0 as libc::c_int | 64 as libc::c_int) as UDWORD;
        sButInit.width = (480 as libc::c_int - 40 as libc::c_int) as UWORD
    }
    sButInit.pUserData = bGrey as *mut libc::c_void;
    sButInit.height = 30 as libc::c_int as UWORD;
    sButInit.pDisplay =
        Some(displayTextOption as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.FontID = FEFont;
    sButInit.pText = txt;
    widgAddButton(psWScreen, &mut sButInit);
    if bGrey != 0 {
        // dont allow clicks to this button...
        widgSetButtonState(psWScreen, id, 0x1 as libc::c_int as UDWORD);
    };
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addFESlider(mut id: UDWORD, mut parent: UDWORD,
                                     mut x: UDWORD, mut y: UDWORD,
                                     mut stops: UDWORD, mut pos: UDWORD,
                                     mut attachID: UDWORD) {
    let mut sSldInit: W_SLDINIT =
        W_SLDINIT{formID: 0,
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
                  orientation: 0,
                  numStops: 0,
                  barSize: 0,
                  pos: 0,
                  pTip: 0 as *mut STRING,};
    memset(&mut sSldInit as *mut W_SLDINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_SLDINIT>() as libc::c_ulong);
    sSldInit.formID = parent;
    sSldInit.id = id;
    sSldInit.style = 0 as libc::c_int as UDWORD;
    sSldInit.x = x as libc::c_short;
    sSldInit.y = y as libc::c_short;
    sSldInit.width =
        iV_GetImageWidth(IntImages, IMAGE_SLIDER_BIG as libc::c_int as UWORD);
    sSldInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_SLIDER_BIG as libc::c_int as UWORD);
    sSldInit.orientation = 0x1 as libc::c_int as UWORD;
    sSldInit.numStops = stops as UBYTE as UWORD;
    sSldInit.barSize =
        iV_GetImageHeight(IntImages,
                          IMAGE_SLIDER_BIG as libc::c_int as UWORD);
    sSldInit.pos = pos as UBYTE as UWORD;
    sSldInit.pDisplay =
        Some(displayBigSlider as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sSldInit.pCallback =
        Some(intUpdateQuantitySlider as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    widgAddSlider(psWScreen, &mut sSldInit);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addSideText(mut id: UDWORD, mut PosX: UDWORD,
                                     mut PosY: UDWORD, mut txt: *mut STRING) {
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
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = 20000 as libc::c_int as UDWORD;
    sLabInit.id = id;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = PosX as libc::c_short;
    sLabInit.y = PosY as libc::c_short;
    sLabInit.width = 30 as libc::c_int as UWORD;
    sLabInit.height = 300 as libc::c_int as UWORD;
    sLabInit.FontID = FEFont;
    sLabInit.pDisplay =
        Some(displayTextAt270 as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sLabInit.pText = txt;
    widgAddLabel(psWScreen, &mut sLabInit);
}
#[no_mangle]
pub unsafe extern "C" fn addText(mut FontID: libc::c_int, mut FormID: UDWORD,
                                 mut id: UDWORD, mut PosX: UDWORD,
                                 mut PosY: UDWORD, mut txt: *mut STRING,
                                 mut attachID: UDWORD, mut State: *mut BOOL) {
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
    debug(LOG_NEVER,
          b"addText : %s\n\x00" as *const u8 as *const libc::c_char, txt);
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = FormID;
    sLabInit.id = id;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = PosX as libc::c_short;
    sLabInit.y =
        PosY.wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_short;
    sLabInit.width = 16 as libc::c_int as UWORD;
    sLabInit.height = 30 as libc::c_int as UWORD;
    sLabInit.FontID = FontID;
    sLabInit.pText = txt;
    sLabInit.UserData = attachID;
    sLabInit.pUserData = State as *mut libc::c_void;
    sLabInit.pCallback =
        Some(intUpdateOptionText as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    widgAddLabel(psWScreen, &mut sLabInit);
}
// ////////////////////////////////////////////////////////////////////////////
// drawing functions
// show a background piccy
#[no_mangle]
pub unsafe extern "C" fn displayTitleBitmap(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let sTmpSize: size_t =
        200 as libc::c_int as
            size_t; //Couldn't have sTmp[sTmpSize], .net did NOT like that, so for now...
    let mut sTmp: [STRING; 200] = [0; 200];
    iV_SetFont(WFont);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    snprintf(sTmp.as_mut_ptr(), sTmpSize,
             b"Version %s - Built %s\x00" as *const u8 as *const libc::c_char,
             version(),
             b"May 29 2020\x00" as *const u8 as *const libc::c_char);
    pie_DrawText270(sTmp.as_mut_ptr(),
                    pie_GetVideoBufferWidth().wrapping_sub(10 as libc::c_int
                                                               as
                                                               libc::c_uint)
                        as libc::c_int,
                    pie_GetVideoBufferHeight().wrapping_sub(15 as libc::c_int
                                                                as
                                                                libc::c_uint)
                        as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
// show warzone logo
#[no_mangle]
pub unsafe extern "C" fn displayLogo(mut psWidget: *mut _widget,
                                     mut xOffset: UDWORD, mut yOffset: UDWORD,
                                     mut pColours: *mut UDWORD) {
    pie_ImageFileID(FrontImages, IMAGE_FE_LOGO as libc::c_int as UWORD,
                    xOffset.wrapping_add((*psWidget).x as libc::c_uint) as
                        libc::c_int,
                    yOffset.wrapping_add((*psWidget).y as libc::c_uint) as
                        libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
// show a text option.
#[no_mangle]
pub unsafe extern "C" fn displayTextOption(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut fx: SDWORD = 0; // if option is unavailable.
    let mut fy: SDWORD = 0;
    let mut fw: SDWORD = 0;
    let mut psBut: *mut W_BUTTON = 0 as *mut W_BUTTON;
    let mut hilight: BOOL = 0 as libc::c_int;
    let mut greyOut: BOOL = (*psWidget).pUserData as BOOL;
    psBut = psWidget as *mut W_BUTTON;
    iV_SetFont((*psBut).FontID);
    if widgGetMouseOver(psWScreen) == (*psBut).id {
        // if mouse is over text then hilight.
        hilight = 1 as libc::c_int
    }
    fw = iV_GetTextWidth((*psBut).pText);
    fy =
        yOffset.wrapping_add((*psWidget).y as
                                 libc::c_uint).wrapping_add((((*psWidget).height
                                                                  as
                                                                  libc::c_int
                                                                  -
                                                                  iV_GetTextLineSize())
                                                                 /
                                                                 2 as
                                                                     libc::c_int)
                                                                as
                                                                libc::c_uint).wrapping_sub(iV_GetTextAboveBase()
                                                                                               as
                                                                                               libc::c_uint)
            as SDWORD;
    if (*psWidget).style & 64 as libc::c_int as libc::c_uint != 0 {
        //check for centering, calculate offset.
        fx =
            xOffset.wrapping_add((*psWidget).x as
                                     libc::c_uint).wrapping_add((((*psWidget).width
                                                                      as
                                                                      libc::c_int
                                                                      - fw) /
                                                                     2 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                as SDWORD
    } else {
        fx = xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD
    }
    if greyOut != 0 {
        // unavailable
        iV_SetTextColour(-(3 as libc::c_int) as SWORD);
    } else if hilight != 0 {
        // available
        // hilight
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
        //#ifdef PSX
//			displayHilightPulseBox( fx-4,fy+iV_GetTextAboveBase()-iV_GetTextBelowBase(),
//									fx+fw,fy+iV_GetTextBelowBase());
//#endif
    } else {
        // dont highlight
        iV_SetTextColour(-(2 as libc::c_int) as SWORD);
        //(unsigned short)iV_PaletteNearestColour(129,142,184)
    }
    pie_DrawText((*psBut).pText, fx as UDWORD, fy as UDWORD);
    if greyOut == 0 {
        // dont snap to unavailable buttons.
        //		AddCursorSnap(&InterfaceSnap, (SWORD)(fx+10) ,(short) fy,psWidget->formID,psWidget->id,NULL);
        if (*psWidget).style & 64 as libc::c_int as libc::c_uint != 0 {
            //check for centering, calculate offset.
            //			DBPRINTF(("%d : %s\n",fx+fw/2,psBut->pText);
            AddCursorSnap(&mut InterfaceSnap,
                          (fx + fw / 2 as libc::c_int) as SWORD,
                          fy as libc::c_short, (*psWidget).formID,
                          (*psWidget).id, &mut FrontendBias);
        } else {
            //			DBPRINTF(("%d : %s\n",fx+10,psBut->pText);
            AddCursorSnap(&mut InterfaceSnap,
                          (fx + 10 as libc::c_int) as SWORD,
                          fy as libc::c_short, (*psWidget).formID,
                          (*psWidget).id, &mut FrontendBias);
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// show text written on its side.
#[no_mangle]
pub unsafe extern "C" fn displayTextAt270(mut psWidget: *mut _widget,
                                          mut xOffset: UDWORD,
                                          mut yOffset: UDWORD,
                                          mut pColours: *mut UDWORD) {
    let mut fx: SDWORD = 0;
    let mut fy: SDWORD = 0;
    let mut psLab: *mut W_LABEL = 0 as *mut W_LABEL;
    psLab = psWidget as *mut W_LABEL;
    iV_SetFont(FEFont);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    fx = xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
    fy =
        yOffset.wrapping_add((*psWidget).y as
                                 libc::c_uint).wrapping_add(iV_GetTextWidth((*psLab).aText.as_mut_ptr())
                                                                as
                                                                libc::c_uint)
            as SDWORD;
    pie_DrawText270((*psLab).aText.as_mut_ptr(), fx, fy);
}
// ////////////////////////////////////////////////////////////////////////////
// show, well have a guess..
unsafe extern "C" fn displayBigSlider(mut psWidget: *mut _widget,
                                      mut xOffset: UDWORD,
                                      mut yOffset: UDWORD,
                                      mut pColours: *mut UDWORD) {
    let mut Slider: *mut W_SLIDER = psWidget as *mut W_SLIDER; // draw bdrop
    let mut x: UDWORD =
        xOffset.wrapping_add((*psWidget).x as libc::c_uint); // determine pos.
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut sx: SWORD = 0;
    pie_ImageFileID(IntImages, IMAGE_SLIDER_BIG as libc::c_int as UWORD,
                    x.wrapping_add(0 as libc::c_int as libc::c_uint) as
                        libc::c_int,
                    y.wrapping_add(0 as libc::c_int as libc::c_uint) as
                        libc::c_int);
    sx =
        (((*Slider).width as libc::c_int - 3 as libc::c_int -
              (*Slider).barSize as libc::c_int) * (*Slider).pos as libc::c_int
             / (*Slider).numStops as libc::c_int) as SWORD;
    pie_ImageFileID(IntImages, IMAGE_SLIDER_BIGBUT as libc::c_int as UWORD,
                    x.wrapping_add(3 as libc::c_int as
                                       libc::c_uint).wrapping_add(sx as
                                                                      libc::c_uint)
                        as libc::c_int,
                    y.wrapping_add(3 as libc::c_int as libc::c_uint) as
                        libc::c_int);
    //draw amount
}
// // Given a string id, set a text buttons dimensions.
// //
//void SetTextButtonExt(W_BUTINIT *psButInit,UWORD x,UWORD y,UDWORD StringID)
//{
//	psButInit->x = x;
//	psButInit->y = y;
//	psButInit->width = iV_GetTextWidth(strresGetString(psStringRes,StringID));
//	psButInit->height = iV_GetTextLineSize();
//}
// Placed here to avoid automatic inlining in InGameOp.c by the Playstation compiler.
//
#[no_mangle]
pub unsafe extern "C" fn addIGTextButton(mut id: UDWORD, mut y: UWORD,
                                         mut StringID: UDWORD,
                                         mut Style: UDWORD) -> BOOL {
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
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    //resume
    sButInit.formID = 10500 as libc::c_int as UDWORD;
    sButInit.id = id;
    sButInit.style = Style;
    //	SetTextButtonExt(&sButInit,INTINGAMEOP_1_X,y,StringID);
    sButInit.x = 5 as libc::c_int as SWORD;
    sButInit.y = y as SWORD;
    sButInit.width = (120 as libc::c_int - 10 as libc::c_int) as UWORD;
    sButInit.height = 10 as libc::c_int as UWORD;
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(displayTextOption as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pText = strresGetString(psStringRes, StringID);
    widgAddButton(psWScreen, &mut sButInit);
    return 1 as libc::c_int;
}
