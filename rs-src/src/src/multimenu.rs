use ::libc;
extern "C" {
    pub type _formation;
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
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
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    // A few useful defined tabs.
    #[no_mangle]
    static mut StandardTab: TABDEF;
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intClosePlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
    /*
 * GTime.h
 *
 * Interface to the game clock.
 *
 */
    /* The number of ticks per second for the game clock */
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    // the current level descriptions
    #[no_mangle]
    static mut psLevels: *mut LEVEL_DATASET;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn displayComponentButtonObject(psDroid: *mut DROID,
                                    Rotation: *mut iVector,
                                    Position: *mut iVector, RotXYZ: BOOL,
                                    scale: SDWORD);
    //NOT ANYMORE! 10/08/98 AB
//#ifndef PSX
//#define INCLUDE_PRODSLIDER	// Include quantity slider in manufacture window.
//#endif
    //#ifndef PSX
    //#endif
    #[no_mangle]
    static mut intMode: INTMODE;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    /* Reset the widget screen to just the reticule */
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    //displays the Power Bar
    #[no_mangle]
    fn intShowPowerBar();
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn drawBlueBox(x: UDWORD, y: UDWORD, w: UDWORD, h: UDWORD);
    //alliance possibilities for games.
    //#define GROUP_WINS		2
    // alliances
    #[no_mangle]
    static mut alliances: [[UBYTE; 8]; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    #[no_mangle]
    fn AddCursorSnap(SnapBuffer: *mut CURSORSNAP, PosX: SWORD, PosY: SWORD,
                     FormID: UDWORD, ID: UDWORD, Bias: *mut SNAPBIAS);
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // note this is of type DPID, not DWORD
    #[no_mangle]
    static mut openchannels: [BOOL; 8];
    #[no_mangle]
    fn getPlayerName(player: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn responsibleFor(player: UDWORD, playerinquestion: UDWORD) -> BOOL;
    // form disk
    #[no_mangle]
    fn getMultiStats(player: UDWORD, bLocal: BOOL) -> PLAYERSTATS;
    /* 
 *multigifts. h
 *
 * multiplayer game, gifts and deathmatch relevant funcs.
 */
    #[no_mangle]
    fn requestAlliance(from: UBYTE, to: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn breakAlliance(p1: UBYTE, p2: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn formAlliance(p1: UBYTE, p2: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn sendGift(type_0: UDWORD, to: UDWORD) -> BOOL;
    #[no_mangle]
    fn getAsciiTime(psText: *mut STRING, time: UDWORD);
    // requester stuff.
    #[no_mangle]
    static mut MultiCustomMapsPath: [libc::c_char; 255];
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
    //extern W_SCREEN *psWScreen;
    #[no_mangle]
    static mut FrontImages: *mut IMAGEFILE;
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
// Routines to provide simple maths functions that work on both PSX & PC
// Use the type "FRACT" instead of FLOAT
//  - This is defined as a float on PC and a 20.12 fixed point number on PSX
//
//  Use:-
//		MAKEFRACT(int);  to convert from a SDWORD to a FRACT
//		MAKEINT(fract);	to convert the other way
//		FRACTmul(fract,fract); to multiply two fract numbers
//		FRACTdiv(fract,fract); to divide two numbers
//		SQRT(fract);		to get square root of a fract (returns a fract)
//      iSQRT(int);			to get a square root of an integer (returns an UDWORD)
//      FRACTCONST(constA,constB);	; Generates a constant of (constA/constB)
//                         e.g. to define 0.5 use FRACTCONST(1,2)
//                              to define 0.114 use FRACTCONT(114,1000)
//
// Also PERCENT(int,int);	// returns a int value 0->100 of the percentage of the first param over the second
//
// This file used to be in the deliverance src directory. But Jeremy quite correctly
// pointed out to me that it should be library based not deliverance based, and hence
// has now been moved to the lib\framework directory
//
// If you are reading this file from the deliverance source directory, please delete it now
// To multiply a FRACT by a integer just use the normal operator 
//   e.g.   FractValue2=FractValue*Interger;
//
// save is true of divide
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
pub type FRACT = libc::c_float;
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
// The next free ID
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
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
pub type W_SCREEN = _w_screen;
// ID of the IVIS font to use for tool tips.
/* The screen structure which stores all info for a widget screen */
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
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
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
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVertex {
    pub x: int32,
    pub y: int32,
    pub z: int32,
    pub u: int32,
    pub v: int32,
    pub g: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDPoly {
    pub flags: uint32,
    pub zcentre: int32,
    pub npnts: libc::c_int,
    pub normal: iVector,
    pub pindex: *mut VERTEXID,
    pub vrt: *mut iVertex,
    pub pTexAnim: *mut iTexAnim,
    pub BSP_NextPoly: BSPPOLYID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDShape {
    pub flags: uint32,
    pub texpage: int32,
    pub oradius: int32,
    pub sradius: int32,
    pub radius: int32,
    pub visRadius: int32,
    pub xmin: int32,
    pub xmax: int32,
    pub ymin: int32,
    pub ymax: int32,
    pub zmin: int32,
    pub zmax: int32,
    pub ocen: iVector,
    pub numFrames: UWORD,
    pub animInterval: UWORD,
    pub npoints: libc::c_int,
    pub npolys: libc::c_int,
    pub nconnectors: libc::c_int,
    pub points: *mut iVector,
    pub polys: *mut iIMDPoly,
    pub connectors: *mut iVector,
    pub ntexanims: libc::c_int,
    pub texanims: *mut *mut iTexAnim,
    pub next: *mut iIMDShape,
    pub BSPNode: PSBSPTREENODE,
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
/* **************************************************************************/
/* The type of object */
/* ID number of the object */
/* Object's location */
/* Object's direction +ve rotation about y axis*/
/* Object's pitch +ve nose up*/
/* Object's roll +ve left up, right down */
/* screen coordinate details */
/* Which player the object belongs to */
/* Which group selection is the droid currently in? */
/* Whether the object is selected */
/* might want this elsewhere */
/* Which cluster the object is a member of */
/* Whether object is visible to specific player */
/* When an object was destroyed, if 0 still alive */
/*BOOL			    (*damage)(pointerType	*psObject, */
/*UDWORD		damage, */
/* UDWORD		weaponClass); - Damage function */
/*UDWORD				emissionInterval;	how frequently does it puff out damage smoke?*/
/* when did it last puff out smoke? */
/* Data for fire damage */
/* TRUE if the object is in a fire */
/* When the object entered the fire */
/* How much damage has been done since the object */
/* entered the fire */
/* Pointer to next object in list */
/* Pointer to previois object in list */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_object {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _base_object,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
}
/*
 * DisplayDef.h
 *
 * Definitions of the display structures
 *
 */
// ffs am
//#define BOUNDARY_X		(DISP_WIDTH/20)	   // proportional to resolution - Alex M
//#define	BOUNDARY_Y		(DISP_WIDTH/16)
//#define	BOUNDARY_X		(24)
//#define	BOUNDARY_Y		(24)
pub type SCREEN_DISP_DATA = _screen_disp_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
}
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
/* **************************************************************************/
/*
 * base.h
 *
 * Definitions for the base object type.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/*
 * StatsDef.h
 *
 * Structure definitions for the stats system
 *
 */
/* Elements common to all stats structures */
/* Unique ID of the item */
/* pointer to the text id name (i.e. short language-independant name) */
/* Stats common to all stats structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
//ALL components and structures and research topics have a tech level to which they belong
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
//only using KINETIC and HEAT for now
pub type _weapon_class = libc::c_uint;
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
pub type WEAPON_CLASS = _weapon_class;
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
pub type _weapon_subclass = libc::c_uint;
pub const NUM_WEAPON_SUBCLASS: _weapon_subclass = 17;
pub const WSC_EMP: _weapon_subclass = 16;
pub const WSC_COMMAND: _weapon_subclass = 15;
pub const WSC_BOMB: _weapon_subclass = 14;
pub const WSC_LAS_SAT: _weapon_subclass = 13;
pub const WSC_SLOWROCKET: _weapon_subclass = 12;
pub const WSC_SLOWMISSILE: _weapon_subclass = 11;
pub const WSC_AAGUN: _weapon_subclass = 10;
pub const WSC_ELECTRONIC: _weapon_subclass = 9;
//WSC_CLOSECOMBAT,
pub const WSC_HOWITZERS: _weapon_subclass = 8;
pub const WSC_FLAME: _weapon_subclass = 7;
pub const WSC_GAUSS: _weapon_subclass = 6;
pub const WSC_ENERGY: _weapon_subclass = 5;
pub const WSC_ROCKET: _weapon_subclass = 4;
pub const WSC_MISSILE: _weapon_subclass = 3;
//WSC_ARTILLARY,
pub const WSC_MORTARS: _weapon_subclass = 2;
pub const WSC_CANNON: _weapon_subclass = 1;
pub const WSC_MGUN: _weapon_subclass = 0;
pub type WEAPON_SUBCLASS = _weapon_subclass;
// used to define which projectile model to use for the weapon
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type WEAPON_EFFECT = _weapon_effect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub shortRange: UDWORD,
    pub longRange: UDWORD,
    pub minRange: UDWORD,
    pub shortHit: UDWORD,
    pub longHit: UDWORD,
    pub firePause: UDWORD,
    pub numExplosions: UDWORD,
    pub numRounds: UBYTE,
    pub reloadTime: UDWORD,
    pub damage: UDWORD,
    pub radius: UDWORD,
    pub radiusHit: UDWORD,
    pub radiusDamage: UDWORD,
    pub incenTime: UDWORD,
    pub incenDamage: UDWORD,
    pub incenRadius: UDWORD,
    pub flightSpeed: UDWORD,
    pub indirectHeight: UDWORD,
    pub fireOnMove: FIREONMOVE,
    pub weaponClass: WEAPON_CLASS,
    pub weaponSubClass: WEAPON_SUBCLASS,
    pub movementModel: MOVEMENT_MODEL,
    pub weaponEffect: WEAPON_EFFECT,
    pub recoilValue: UDWORD,
    pub rotate: UBYTE,
    pub maxElevation: UBYTE,
    pub minElevation: SBYTE,
    pub facePlayer: UBYTE,
    pub faceInFlight: UBYTE,
    pub effectSize: UBYTE,
    pub lightWorld: BOOL,
    pub surfaceToAir: UBYTE,
    pub vtolAttackRuns: UBYTE,
    pub directLife: UDWORD,
    pub radiusLife: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
    pub pMuzzleGraphic: *mut iIMDShape,
    pub pInFlightGraphic: *mut iIMDShape,
    pub pTargetHitGraphic: *mut iIMDShape,
    pub pTargetMissGraphic: *mut iIMDShape,
    pub pWaterHitGraphic: *mut iIMDShape,
    pub pTrailGraphic: *mut iIMDShape,
    pub iAudioFireID: SDWORD,
    pub iAudioImpactID: SDWORD,
}
//weapon stats associated with this brain - for Command Droids
//defines the left and right sides for propulsion IMDs
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
//works as all of the above together! - new for updates - added 11/06/99 AB
/* Common stats */
// Sensor range
// Sensor power (put against ecm power)
// specifies whether the Sensor is default or for the Turret
// used for combat
//time delay before associated weapon droids 'know' where the
//attack is from
// The turret mount to use
/* Common stats */
// ECM range
// ECM power (put against sensor power)
// specifies whether the ECM is default or for the Turret
// The turret mount to use
/* Adjustment to armour damage for non-penetrating hit */
/* Adjustment to armour damage for penetrating hit */
/* Common stats */
// How much damage is restored to Body Points 
// and armour each Repair Cycle
// whether armour can be repaired or not
// specifies whether the Repair is default or for the Turret
// time delay for repair cycle
// The turret mount to use
//no longer used 7/8/98
/*typedef struct _program_stats
{
	// Common stats - This structure doesn't actually need all the stats
	COMPONENT_STATS;		
	UDWORD		slots;				// How many brain slots the program takes
	UDWORD		order;				// The order activated by the program if any
	UDWORD		special;			// The special ability that the droid can perform
									// with this program
} PROGRAM_STATS;*/
/*these are defined in Access database - if you change them in there,
  then change them here! (and the rest of the code)
  They are made up values for now - defined when Jim does it!*/
/*typedef enum _program_orders
{
	ORDER_STOP,
	ORDER_SCAVANGE,
	ORDER_ATTACK,
	ORDER_GUARD,
	ORDER_AID,
	ORDER_BUILD,
	ORDER_DEMOLISH,
	ORDER_REPAIR,
} PROGRAM_ORDERS;*/
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
pub type SENSOR_TYPE = _sensor_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sensor_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub type_0: SENSOR_TYPE,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ecm_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
/*Common stats for all Structure Functions*/
/* Unique ID of the item */
/* Text name of the component */
/* The type of Function */
/*Common struct for all functions*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UDWORD,
    pub techLevel: TECH_LEVEL,
    pub strength: STRUCT_STRENGTH,
    pub terrainType: UDWORD,
    pub baseWidth: UDWORD,
    pub baseBreadth: UDWORD,
    pub foundationType: UDWORD,
    pub buildPoints: UDWORD,
    pub height: UDWORD,
    pub armourValue: UDWORD,
    pub bodyPoints: UDWORD,
    pub repairSystem: UDWORD,
    pub powerToBuild: UDWORD,
    pub resistance: UDWORD,
    pub sizeModifier: UDWORD,
    pub pIMD: *mut iIMDShape,
    pub pBaseIMD: *mut iIMDShape,
    pub pECM: *mut _ecm_stats,
    pub pSensor: *mut _sensor_stats,
    pub psWeapStat: *mut _weapon_stats,
    pub numFuncs: UDWORD,
    pub defaultFunc: SDWORD,
    pub asFuncList: *mut *mut _function,
}
/*
 * StructureDef.h
 *
 * Structure definitions for structures
 *
 */
// Used to indicate any kind of building when calling intGotoNextStructureType()
/* Defines for indexing an appropriate IMD object given a buildings purpose. */
//draw as factory 2	
//corner wall - no gun
//control centre for command droids
//the demolish structure type - should only be one stat with this type
//added for updates - AB 8/6/99
//REF_WALLH,     //the following are needed for the demo
//REF_WALLV,		
//REF_CORNER1,
//REF_CORNER2,
//REF_CORNER3,
//REF_CORNER4,
//REF_GATE1,
//REF_GATE2,
//REF_GATE3,
//REF_GATE4,
//REF_TOWER1,
//REF_TOWER2,
//REF_TOWER3,
//REF_TOWER4,
//REF_VANH,
//REF_VANV,
//REF_JEEP,
//REF_TANKERH,
//REF_TANKERV,
//need to keep a count of how many types for IMD loading
//Delivery Points NOT wayPoints
//proximity messages that are data generated
//proximity messages that are in game generated
//SAVE ONLY delivery point for factory currently assigned to commander
/*the type of position obj - FlagPos or ProxDisp*/
/*when the Position was last drawn*/
/*screen coords and radius of Position imd */
/*which player the Position belongs to*/
/*flag to indicate whether the Position 
										is to be highlighted*/
//the world coords of the Position
//UDWORD		frameNumber;					//when the Position was last drawn
	//UDWORD		screenX, screenY, screenR;		//screen coords and radius of Position imd
	//UDWORD		player;							//which player the Position belongs to
	//BOOL		selected;						//flag to indicate whether the 
												//Position is to be highlighted
//indicates whether the first, second etc factory
//indicates whether standard, cyborg or vtol factory
//	UBYTE		factorySub;						//sub value. needed to order production points.
//	UBYTE		primary;
//only allowed one weapon per structure (more memory for Tim) 
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_point {
    pub x: UBYTE,
    pub y: UBYTE,
}
pub type PATH_POINT = _path_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _move_control {
    pub Status: UBYTE,
    pub Mask: UBYTE,
    pub Position: UBYTE,
    pub numPoints: UBYTE,
    pub asPath: [PATH_POINT; 100],
    pub DestinationX: SDWORD,
    pub DestinationY: SDWORD,
    pub srcX: SDWORD,
    pub srcY: SDWORD,
    pub targetX: SDWORD,
    pub targetY: SDWORD,
    pub fx: FRACT,
    pub fy: FRACT,
    pub speed: FRACT,
    pub boundX: SWORD,
    pub boundY: SWORD,
    pub dir: SWORD,
    pub bumpDir: SWORD,
    pub bumpTime: UDWORD,
    pub lastBump: UWORD,
    pub pauseTime: UWORD,
    pub bumpX: UWORD,
    pub bumpY: UWORD,
    pub shuffleStart: UDWORD,
    pub psFormation: *mut _formation,
    pub iVertSpeed: SWORD,
    pub iAttackRuns: UWORD,
    pub fz: FRACT,
}
pub type MOVE_CONTROL = _move_control;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/*
 * Weapons.h
 *
 * Definitions for the weapons
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
pub type WEAPON = _weapon;
pub type BASE_OBJECT = _base_object;
/* **************************************************************************/
/* ensure ANIM2D/3D structs same size */
/* width of container bitmap */
/* ensure ANIM2D/3D structs same size */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_STATE {
    pub uwFrame: UWORD,
    pub vecPos: VECTOR3D,
    pub vecAngle: VECTOR3D,
    pub vecScale: VECTOR3D,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASEANIM {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM3D {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
    pub psFrames: *mut iIMDShape,
    pub apFrame: *mut *mut iIMDShape,
}
/* **************************************************************************/
/*
 * Animobj.h
 *
 * Animation object types and function headers
 *
 * Gareth Jones 14/11/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* forward struct declarations */
/* **************************************************************************/
/* typedefs */
/* **************************************************************************/
/* struct member macros */
/* this must be the last entry in this structure */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_OBJECT {
    pub psNext: *mut ANIM_OBJECT,
    pub uwID: UWORD,
    pub psAnim: *mut ANIM3D,
    pub psParent: *mut libc::c_void,
    pub udwStartTime: UDWORD,
    pub udwStartDelay: UDWORD,
    pub uwCycles: UWORD,
    pub bVisible: BOOL,
    pub pDoneFunc: ANIMOBJDONEFUNC,
    pub apComponents: [COMPONENT_OBJECT; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMPONENT_OBJECT {
    pub position: VECTOR3D,
    pub orientation: VECTOR3D,
    pub psParent: *mut libc::c_void,
    pub psShape: *mut iIMDShape,
}
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
/*
 * DroidDef.h
 *
 * Droid structure definitions
 */
/* The number of components in the asParts / asBits arrays */
//(COMP_NUMCOMPONENTS - 2)
/* The maximum number of droid weapons and programs */
//3
//#define DROID_MAXPROGS		3
// This should really be logarithmic
//defines how many times to perform the iteration on looking for a blank location
//used to get a location next to a droid - withinh one tile
/* The different types of droid */
// NOTE, if you add to, or change this list then you'll need
// to update the DroidSelectionWeights lookup table in Display.c
pub type _droid_type = libc::c_uint;
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//cyborg repair droid - new for update 7/6/99
pub const DROID_ANY: _droid_type = 13;
//cyborg repair droid - new for update 28/5/99
pub const DROID_CYBORG_SUPER: _droid_type = 12;
//cyborg constructor droid - new for update 28/5/99
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
// Default droid
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
// Repair droid
pub const DROID_DEFAULT: _droid_type = 9;
// Command droid
pub const DROID_REPAIR: _droid_type = 8;
// guess what this is!
pub const DROID_COMMAND: _droid_type = 7;
// cyborg-type thang
pub const DROID_TRANSPORTER: _droid_type = 6;
// person
pub const DROID_CYBORG: _droid_type = 5;
// Constructor droid
pub const DROID_PERSON: _droid_type = 4;
// ECM droid
pub const DROID_CONSTRUCT: _droid_type = 3;
// Sensor droid
pub const DROID_ECM: _droid_type = 2;
// Weapon droid
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _order_list {
    pub order: SDWORD,
    pub psOrderTarget: *mut libc::c_void,
    pub x: UWORD,
    pub y: UWORD,
    pub x2: UWORD,
    pub y2: UWORD,
}
pub type ORDER_LIST = _order_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _droid,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub aName: [STRING; 60],
    pub droidType: DROID_TYPE,
    pub asBits: [COMPONENT; 8],
    pub weight: UDWORD,
    pub baseSpeed: UDWORD,
    pub sensorRange: UDWORD,
    pub sensorPower: UDWORD,
    pub ECMMod: UDWORD,
    pub originalBody: UDWORD,
    pub body: UDWORD,
    pub armour: [UDWORD; 2],
    pub numKills: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub NameVersion: UBYTE,
    pub currRayAng: UBYTE,
    pub resistance: SWORD,
    pub asWeaps: [WEAPON; 1],
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
    pub psBaseStruct: *mut _structure,
    pub listSize: SDWORD,
    pub asOrderList: [ORDER_LIST; 10],
    pub order: SDWORD,
    pub orderX: UWORD,
    pub orderY: UWORD,
    pub orderX2: UWORD,
    pub orderY2: UWORD,
    pub lastHitWeapon: UDWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
    pub psTarget: *mut _base_object,
    pub psTarStats: *mut _base_stats,
    pub secondaryOrder: UDWORD,
    pub lastSync: UDWORD,
    pub action: SDWORD,
    pub actionX: UDWORD,
    pub actionY: UDWORD,
    pub psActionTarget: *mut _base_object,
    pub actionStarted: UDWORD,
    pub actionPoints: UDWORD,
    pub powerAccrued: UWORD,
    pub illumination: UBYTE,
    pub updateFlags: UBYTE,
    pub sMove: MOVE_CONTROL,
    pub psCurAnim: *mut ANIM_OBJECT,
    pub iAudioID: SDWORD,
}
//line build requires two sets of coords
//this structure is used whenever an instance of a building is required in game
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _structure,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
    pub pStructureType: *mut STRUCTURE_STATS,
    pub status: UBYTE,
    pub currentBuildPts: SWORD,
    pub currentPowerAccrued: SWORD,
    pub body: UWORD,
    pub armour: UWORD,
    pub resistance: SWORD,
    pub lastResistance: UDWORD,
    pub sensorRange: UWORD,
    pub sensorPower: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub timeLastHit: UDWORD,
    pub lastHitWeapon: UDWORD,
    pub radarX: UWORD,
    pub radarY: UWORD,
    pub ecmPower: UWORD,
    pub pFunctionality: *mut FUNCTIONALITY,
    pub targetted: UBYTE,
    pub asWeaps: [WEAPON; 1],
    pub psTarget: *mut BASE_OBJECT,
    pub psCurAnim: *mut ANIM_OBJECT,
}
//this structure is used to hold the permenant stats for each type of building
/* basic stats */
/* the type of structure */
/* technology level of the structure */
/* strength against the weapon effects */
/*The type of terrain the structure has to be 
									  built next to - may be none*/
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
/*The type of foundation for the structure*/
/*The number of build points required to build
									  the structure*/
/*The height above/below the terrain - negative
									  values denote below the terrain*/
/*The armour value for the structure - can be 
									  upgraded */
/*The structure's body points - A structure goes
									  off-line when 50% of its body points are lost*/
/*The repair system points are added to the body
									  points until fully restored . The points are 
									  then added to the Armour Points*/
/*How much power the structure requires to build*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		minimumPower;		The minimum power requirement to start building
								      the structure*/
/*The number used to determine whether a 
									  structure can resist an enemy takeover - 
									  0 = cannot be attacked electrically*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		quantityLimit;		The maximum number that a player can have - 
									  0 = no limit 1 = only 1 allowed etc*/
/*The larger the target, the easier to hit*/
/*The IMD to draw for this structure */
/*The base IMD to draw for this structure */
/*Which ECM is standard for the structure - 
									  if any*/
/*Which Sensor is standard for the structure - 
									  if any*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		weaponSlots;		/Number of weapons that can be attached to the
									  building/
	UDWORD		numWeaps;			/Number of weapons for default /
	SDWORD		defaultWeap;		/The default weapon/
    
	struct _weapon_stats **asWeapList;		/List of pointers to default weapons/
    */
//can only have one weapon now
/*Number of functions for default*/
/*The default function*/
/*List of pointers to allowable functions - 
									  unalterable*/
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
pub type RUN_DATA = _run_data;
// basic chance to run
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
pub type W_MAJORTAB = _w_majortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_tabform {
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
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub majorT: UWORD,
    pub minorT: UWORD,
    pub state: UWORD,
    pub tabHiLite: UWORD,
    pub numMajor: UWORD,
    pub asMajor: [W_MAJORTAB; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
pub type W_TABFORM = _w_tabform;
/* Button states */
// Button is down
// Button is disabled
// Button is hilited
// Button is locked down
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDEF {
    pub MajorUp: SWORD,
    pub MajorDown: SWORD,
    pub MajorHilight: SWORD,
    pub MajorSelected: SWORD,
    pub MinorUp: SWORD,
    pub MinorDown: SWORD,
    pub MinorHilight: SWORD,
    pub MinorSelected: SWORD,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _level_dataset {
    pub type_0: SWORD,
    pub players: SWORD,
    pub game: SWORD,
    pub pName: *mut STRING,
    pub dataDir: libc::c_int,
    pub apDataFiles: [*mut STRING; 9],
    pub psBaseData: *mut _level_dataset,
    pub psChange: *mut _level_dataset,
    pub psNext: *mut _level_dataset,
}
// the WRF/WDG files needed for a particular level
// the WRF/WDG files needed for a particular level
pub type LEVEL_DATASET = _level_dataset;
pub type INTMODE = libc::c_uint;
pub const INT_MAXMODE: INTMODE = 15;
pub const INT_CDCHANGE: INTMODE = 14;
pub const INT_MULTIMENU: INTMODE = 13;
pub const INT_MISSIONRES: INTMODE = 12;
pub const INT_TRANSPORTER: INTMODE = 11;
pub const INT_INGAMEOP: INTMODE = 10;
pub const INT_ORDER: INTMODE = 9;
pub const INT_INTELMAP: INTMODE = 8;
pub const INT_DESIGN: INTMODE = 7;
pub const INT_CMDORDER: INTMODE = 6;
pub const INT_STAT: INTMODE = 5;
pub const INT_OBJECT: INTMODE = 4;
pub const INT_EDITSTAT: INTMODE = 3;
pub const INT_EDIT: INTMODE = 2;
pub const INT_OPTION: INTMODE = 1;
pub const INT_NORMAL: INTMODE = 0;
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
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
}
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
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
pub const IMAGE_PLAYERX: C2RustUnnamed_0 = 197;
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
pub const IMAGE_RETURN_HI: C2RustUnnamed_0 = 173;
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
pub const IMAGE_RETURN: C2RustUnnamed_0 = 149;
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
pub const IMAGE_PLAYER7: C2RustUnnamed_0 = 119;
pub const IMAGE_PLAYER6: C2RustUnnamed_0 = 118;
pub const IMAGE_PLAYER5: C2RustUnnamed_0 = 117;
pub const IMAGE_PLAYER4: C2RustUnnamed_0 = 116;
pub const IMAGE_PLAYER3: C2RustUnnamed_0 = 115;
pub const IMAGE_PLAYER2: C2RustUnnamed_0 = 114;
pub const IMAGE_PLAYER1: C2RustUnnamed_0 = 113;
pub const IMAGE_PLAYER0: C2RustUnnamed_0 = 112;
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
pub const IMAGE_FE_LOGO: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYERSTATS {
    pub played: DWORD,
    pub wins: DWORD,
    pub loses: DWORD,
    pub totalKills: DWORD,
    pub totalScore: SDWORD,
    pub recentKills: DWORD,
    pub recentScore: SDWORD,
    pub killsToAdd: DWORD,
    pub scoreToAdd: SDWORD,
}
/*
 *  MultiMenu.c
 *  Handles the In Game MultiPlayer Screen, alliances etc...
 *  Also the selection of disk files..
 */
//above line not needed for win32.  --Qamly
// FIXME Direct iVis implementation include!
//#include "intfac.h"		// for images.
// ////////////////////////////////////////////////////////////////////////////
// defines
#[no_mangle]
pub static mut psRScreen: *mut W_SCREEN =
    0 as *const W_SCREEN as *mut W_SCREEN;
#[no_mangle]
pub static mut MultiMenuUp: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingMultiMenu: BOOL = 0 as libc::c_int;
static mut context: UDWORD = 0 as libc::c_int as UDWORD;
static mut current_tech: UDWORD = 0 as libc::c_int as UDWORD;
static mut current_numplayers: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut multiRequestUp: BOOL = 0 as libc::c_int;
//multimenu is up.
static mut giftsUp: [BOOL; 8] = [1 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
//gift buttons for player are up.
// ////////////////////////////////////////////////////////////////////////////
// Map / force / name load save stuff.
// ////////////////////////////////////////////////////////////////////////////
// enumerates maps in the gamedesc file.
// returns only maps that are valid the right 'type'
#[no_mangle]
pub unsafe extern "C" fn enumerateMultiMaps(mut found: *mut STRING,
                                            mut players: *mut UDWORD,
                                            mut first: BOOL,
                                            mut camToUse: UBYTE,
                                            mut numPlayers: UBYTE) -> BOOL {
    static mut lev: *mut LEVEL_DATASET =
        0 as *const LEVEL_DATASET as *mut LEVEL_DATASET;
    let mut cam: UBYTE = 0;
    if first != 0 { lev = psLevels }
    while !lev.is_null() {
        //		if(game.type == DMATCH)
//		{
//			if(lev->type == DMATCH)
//			{
//				strcpy(found,lev->pName);
//				*players = lev->players;
//				lev = lev->psNext;
//				return TRUE;
//			}
//		}
//		else
        if game.type_0 as libc::c_int == 14 as libc::c_int {
            if (*lev).type_0 as libc::c_int == 18 as libc::c_int {
                cam = 2 as libc::c_int as UBYTE
            } else if (*lev).type_0 as libc::c_int == 19 as libc::c_int {
                cam = 3 as libc::c_int as UBYTE
            } else {
                //			else if(lev->type == MULTI_SKIRMISHA)
//			{
//				cam = 0;
//			}
                cam = 1 as libc::c_int as UBYTE
            }
            if ((*lev).type_0 as libc::c_int == 14 as libc::c_int ||
                    (*lev).type_0 as libc::c_int == 18 as libc::c_int ||
                    (*lev).type_0 as libc::c_int == 19 as libc::c_int) &&
                   (numPlayers as libc::c_int == 0 as libc::c_int ||
                        numPlayers as libc::c_int ==
                            (*lev).players as libc::c_int) &&
                   cam as libc::c_int == camToUse as libc::c_int {
                strcpy(found, (*lev).pName);
                *players = (*lev).players as UDWORD;
                lev = (*lev).psNext;
                return 1 as libc::c_int
            }
        } else {
            //  campaign, teamplay
            // 'service pack 1'
            if (*lev).type_0 as libc::c_int == 15 as libc::c_int {
                cam = 2 as libc::c_int as UBYTE
            } else if (*lev).type_0 as libc::c_int == 16 as libc::c_int {
                cam = 3 as libc::c_int as UBYTE
            } else {
                //			else if(lev->type == MULTI_CAMPAIGNA)
//			{
//				cam = 0;
//			}
                cam = 1 as libc::c_int as UBYTE
            }
            if ((*lev).type_0 as libc::c_int == 12 as libc::c_int ||
                    (*lev).type_0 as libc::c_int == 15 as libc::c_int ||
                    (*lev).type_0 as libc::c_int == 16 as libc::c_int) &&
                   (numPlayers as libc::c_int == 0 as libc::c_int ||
                        numPlayers as libc::c_int ==
                            (*lev).players as libc::c_int) &&
                   cam as libc::c_int == camToUse as libc::c_int {
                strcpy(found, (*lev).pName);
                *players = (*lev).players as UDWORD;
                lev = (*lev).psNext;
                return 1 as libc::c_int
            }
        }
        lev = (*lev).psNext
    }
    return 0 as libc::c_int;
}
//	end of service pack
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn displayRequestOption(mut psWidget: *mut _widget,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    //	UWORD	im = (UWORD)UNPACKDWORD_TRI_B((UDWORD)psWidget->pUserData);
//	UWORD	im2= (UWORD)(UNPACKDWORD_TRI_C((UDWORD)psWidget->pUserData));
    let mut count: UDWORD = 0; //draw box
    let mut butString: [STRING; 255] = [0; 255]; // font
    strcpy(butString.as_mut_ptr(),
           (*(psWidget as *mut W_BUTTON)).pTip); //colour
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD); //draw text
    iV_SetFont(WFont);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    while iV_GetTextWidth(butString.as_mut_ptr()) >
              (*psWidget).width as libc::c_int - 10 as libc::c_int {
        butString[strlen(butString.as_mut_ptr()).wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                      as usize] = '\u{0}' as i32 as STRING
    }
    pie_DrawText(butString.as_mut_ptr(),
                 x.wrapping_add(6 as libc::c_int as libc::c_uint),
                 y.wrapping_add(12 as libc::c_int as libc::c_uint));
    // if map, then draw no. of players.
    count = 0 as libc::c_int as UDWORD;
    while count < (*psWidget).pUserData as UDWORD {
        pie_ImageFileID(FrontImages, IMAGE_WEE_GUY as libc::c_int as UWORD,
                        x.wrapping_add((6 as libc::c_int as
                                            libc::c_uint).wrapping_mul(count)).wrapping_add(6
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                            as libc::c_int,
                        y.wrapping_add(16 as libc::c_int as libc::c_uint) as
                            libc::c_int);
        count = count.wrapping_add(1)
    }
    AddCursorSnap(&mut InterfaceSnap,
                  x.wrapping_add(5 as libc::c_int as libc::c_uint) as SWORD,
                  y.wrapping_add(5 as libc::c_int as libc::c_uint) as SWORD,
                  (*psWidget).formID, (*psWidget).id, 0 as *mut SNAPBIAS);
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn displayCamTypeBut(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut x: UDWORD =
        xOffset.wrapping_add((*psWidget).x as libc::c_uint); //draw box
    let mut y: UDWORD =
        yOffset.wrapping_add((*psWidget).y as libc::c_uint); //draw box
    let mut buffer: [libc::c_char; 8] = [0; 8];
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD);
    sprintf(buffer.as_mut_ptr(),
            b"T%i\x00" as *const u8 as *const libc::c_char,
            (*psWidget).UserData as libc::c_int);
    if (*psWidget).UserData as libc::c_int as libc::c_uint == current_tech {
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    } else { iV_SetTextColour(-(2 as libc::c_int) as SWORD); }
    pie_DrawText(buffer.as_mut_ptr(),
                 x.wrapping_add(2 as libc::c_int as libc::c_uint),
                 y.wrapping_add(12 as libc::c_int as libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn displayNumPlayersBut(mut psWidget: *mut _widget,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut buffer: [libc::c_char; 8] = [0; 8];
    drawBlueBox(x, y, (*psWidget).width as UDWORD,
                (*psWidget).height as UDWORD);
    if (*psWidget).UserData as libc::c_int as libc::c_uint ==
           current_numplayers {
        iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    } else { iV_SetTextColour(-(2 as libc::c_int) as SWORD); }
    if (*psWidget).UserData as libc::c_int == 0 as libc::c_int {
        sprintf(buffer.as_mut_ptr(),
                b" *\x00" as *const u8 as *const libc::c_char);
    } else {
        sprintf(buffer.as_mut_ptr(),
                b"%iP\x00" as *const u8 as *const libc::c_char,
                (*psWidget).UserData as libc::c_int);
    }
    pie_DrawText(buffer.as_mut_ptr(),
                 x.wrapping_add(2 as libc::c_int as libc::c_uint),
                 y.wrapping_add(12 as libc::c_int as libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn check_tip_index(mut i: libc::c_uint)
 -> libc::c_uint {
    if i < 512 as libc::c_int as libc::c_uint {
        return i
    } else {
        debug(LOG_MAIN,
              b"Tip window index too high (%ud)\x00" as *const u8 as
                  *const libc::c_char, i);
        return (512 as libc::c_int - 1 as libc::c_int) as libc::c_uint
    };
}
/*
 * MultiMenu.h
 *
 * Definition for in game,multiplayer, interface.
 */
// 
// requester
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// FIXME: what is this, and why is there some code that only works in win32
// here? - Per
#[no_mangle]
pub unsafe extern "C" fn addMultiRequest(mut ToFindb: *mut STRING,
                                         mut mode: UDWORD, mut mapCam: UBYTE,
                                         mut numPlayers: UBYTE) {
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
    let mut players: UDWORD = 0;
    let mut numButtons: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut sTemp: [STRING; 64] = [0; 64];
    static mut tips: [[STRING; 20]; 512] = [[0; 20]; 512];
    numButtons = 0 as libc::c_int as UDWORD;
    context = mode;
    current_tech = mapCam as UDWORD;
    current_numplayers = numPlayers as UDWORD;
    if mode == 10259 as libc::c_int as libc::c_uint {
        // if its a map, also look in the predone stuff.
        if enumerateMultiMaps(tips[0 as libc::c_int as usize].as_mut_ptr(),
                              &mut players, 1 as libc::c_int, mapCam,
                              numPlayers) != 0 {
            numButtons =
                numButtons.wrapping_add(1); // / move this to intinit or somewhere like that.. (close too.)
            while enumerateMultiMaps(tips[0 as libc::c_int as
                                              usize].as_mut_ptr(),
                                     &mut players, 0 as libc::c_int, mapCam,
                                     numPlayers) != 0 {
                numButtons = numButtons.wrapping_add(1)
            }
        }
    }
    widgCreateScreen(&mut psRScreen);
    widgSetTipFont(psRScreen, WFont);
    /* Calculate how many buttons will go on a single form */
    butPerForm =
        ((250 as libc::c_int - 0 as libc::c_int - 4 as libc::c_int) /
             (105 as libc::c_int + 4 as libc::c_int) *
             ((330 as libc::c_int - 0 as libc::c_int - 4 as libc::c_int) /
                  (30 as libc::c_int + 4 as libc::c_int))) as UDWORD;
    /* add a form to place the tabbed form on */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = (10600 as libc::c_int + 50 as libc::c_int) as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (373 as libc::c_int as
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
        (15 as libc::c_int as
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
    sFormInit.width = 250 as libc::c_int as UWORD;
    sFormInit.height = 330 as libc::c_int as UWORD;
    sFormInit.disableChildren = 1 as libc::c_int;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    widgAddForm(psRScreen, &mut sFormInit);
    /* Add the tabs */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = (10600 as libc::c_int + 50 as libc::c_int) as UDWORD;
    sFormInit.id = (10600 as libc::c_int + 51 as libc::c_int) as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 2 as libc::c_int as SWORD;
    sFormInit.width = 250 as libc::c_int as UWORD;
    sFormInit.height = (330 as libc::c_int - 4 as libc::c_int) as UWORD;
    sFormInit.numMajor = numForms(numButtons, butPerForm);
    if sFormInit.numMajor as libc::c_int > 8 as libc::c_int {
        sFormInit.numMajor = 8 as libc::c_int as UWORD
    }
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = (26 as libc::c_int + 2 as libc::c_int) as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        &mut StandardTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 2 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    widgAddForm(psRScreen, &mut sFormInit);
    // Add the close button.
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = (10600 as libc::c_int + 50 as libc::c_int) as UDWORD;
    sButInit.id = (10600 as libc::c_int + 49 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (250 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 0 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_CLOSEHILIGHT as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_CLOSE as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    widgAddButton(psRScreen, &mut sButInit);
    /* Put the buttons on it */
    /* Set up the button struct */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = (10600 as libc::c_int + 51 as libc::c_int) as UDWORD;
    sButInit.id = (10600 as libc::c_int + 100 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = 22 as libc::c_int as SWORD;
    sButInit.y = 4 as libc::c_int as SWORD;
    sButInit.width = 105 as libc::c_int as UWORD;
    sButInit.height = 30 as libc::c_int as UWORD;
    sButInit.pUserData = 0 as *mut libc::c_void;
    sButInit.pDisplay =
        Some(displayRequestOption as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.FontID = WFont;
    if mode == 10259 as libc::c_int as libc::c_uint {
        if enumerateMultiMaps(sTemp.as_mut_ptr(), &mut players,
                              1 as libc::c_int, mapCam, numPlayers) != 0 {
            loop  {
                let mut tip_index: libc::c_uint =
                    check_tip_index(sButInit.id.wrapping_sub((10600 as
                                                                  libc::c_int
                                                                  +
                                                                  100 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_uint));
                // add number of players to string.
                sprintf(tips[tip_index as usize].as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        sTemp.as_mut_ptr());
                sButInit.pTip = tips[tip_index as usize].as_mut_ptr();
                sButInit.pText = tips[tip_index as usize].as_mut_ptr();
                sButInit.pUserData = players as *mut libc::c_void;
                widgAddButton(psRScreen, &mut sButInit);
                sButInit.id =
                    (sButInit.id as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                sButInit.x =
                    (sButInit.x as libc::c_int +
                         (105 as libc::c_int + 4 as libc::c_int)) as SWORD;
                if sButInit.x as libc::c_int + 105 as libc::c_int +
                       2 as libc::c_int > 250 as libc::c_int {
                    sButInit.x = 22 as libc::c_int as SWORD;
                    sButInit.y =
                        (sButInit.y as libc::c_int + 30 as libc::c_int +
                             4 as libc::c_int) as SWORD
                }
                if sButInit.y as libc::c_int + 30 as libc::c_int +
                       4 as libc::c_int > 330 as libc::c_int {
                    sButInit.y = 4 as libc::c_int as SWORD;
                    sButInit.majorID =
                        (sButInit.majorID as libc::c_int + 1 as libc::c_int)
                            as UWORD
                }
                if !(enumerateMultiMaps(sTemp.as_mut_ptr(), &mut players,
                                        0 as libc::c_int, mapCam, numPlayers)
                         != 0) {
                    break ;
                }
            }
        }
    }
    multiRequestUp = 1 as libc::c_int;
    // if it's map select then add the cam style buttons.
    if mode == 10259 as libc::c_int as libc::c_uint {
        memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_BUTINIT>() as
                   libc::c_ulong); // move this to the frontend shutdown...
        sButInit.formID =
            (10600 as libc::c_int + 50 as libc::c_int) as UDWORD;
        sButInit.id = (10600 as libc::c_int + 61 as libc::c_int) as UDWORD;
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.x = 4 as libc::c_int as SWORD;
        sButInit.y = 258 as libc::c_int as SWORD;
        sButInit.width = 17 as libc::c_int as UWORD;
        sButInit.height = 17 as libc::c_int as UWORD;
        sButInit.UserData = 1 as libc::c_int as UDWORD;
        sButInit.FontID = WFont;
        sButInit.pTip =
            b"Tech:1\x00" as *const u8 as *const libc::c_char as *mut STRING;
        sButInit.pDisplay =
            Some(displayCamTypeBut as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 62 as libc::c_int) as UDWORD;
        sButInit.y = (sButInit.y as libc::c_int + 22 as libc::c_int) as SWORD;
        sButInit.UserData = 2 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"Tech:2\x00" as *const u8 as *const libc::c_char as *mut STRING;
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 63 as libc::c_int) as UDWORD;
        sButInit.y = (sButInit.y as libc::c_int + 22 as libc::c_int) as SWORD;
        sButInit.UserData = 3 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"Tech:3\x00" as *const u8 as *const libc::c_char as *mut STRING;
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 70 as libc::c_int) as UDWORD;
        sButInit.y = 17 as libc::c_int as SWORD;
        sButInit.UserData = 0 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"Any number of players\x00" as *const u8 as *const libc::c_char
                as *mut STRING;
        sButInit.pDisplay =
            Some(displayNumPlayersBut as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 71 as libc::c_int) as UDWORD;
        sButInit.y = (sButInit.y as libc::c_int + 22 as libc::c_int) as SWORD;
        sButInit.UserData = 2 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"2 players\x00" as *const u8 as *const libc::c_char as
                *mut STRING;
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 72 as libc::c_int) as UDWORD;
        sButInit.y = (sButInit.y as libc::c_int + 22 as libc::c_int) as SWORD;
        sButInit.UserData = 4 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"4 players\x00" as *const u8 as *const libc::c_char as
                *mut STRING;
        widgAddButton(psRScreen, &mut sButInit);
        sButInit.id = (10600 as libc::c_int + 73 as libc::c_int) as UDWORD;
        sButInit.y = (sButInit.y as libc::c_int + 22 as libc::c_int) as SWORD;
        sButInit.UserData = 8 as libc::c_int as UDWORD;
        sButInit.pTip =
            b"8 players\x00" as *const u8 as *const libc::c_char as
                *mut STRING;
        widgAddButton(psRScreen, &mut sButInit);
    };
}
#[no_mangle]
pub unsafe extern "C" fn closeMultiRequester() {
    widgDelete(psRScreen,
               (10600 as libc::c_int + 50 as libc::c_int) as UDWORD);
    multiRequestUp = 0 as libc::c_int;
    widgReleaseScreen(psRScreen);
}
// requester stuff.
#[no_mangle]
pub unsafe extern "C" fn runMultiRequester(mut id: UDWORD,
                                           mut mode: *mut UDWORD,
                                           mut chosen: *mut STRING,
                                           mut chosenValue: *mut UDWORD)
 -> BOOL {
    let mut tmp: [libc::c_char; 255] = [0; 255];
    if id == (10600 as libc::c_int + 49 as libc::c_int) as libc::c_uint {
        // close
        closeMultiRequester();
        return 1 as libc::c_int
    }
    if id >= (10600 as libc::c_int + 100 as libc::c_int) as libc::c_uint &&
           id <= (10600 as libc::c_int + 1100 as libc::c_int) as libc::c_uint
       {
        // chose a file.
        strcpy(chosen,
               (*(widgGetFromID(psRScreen, id) as *mut W_BUTTON)).pText);
        //		if(context == MULTIOP_MAP)						// chop off the number of players.
//		{
//			strcpy(chosen, strrchr(chosen,')')+1  );
//		}
        *chosenValue =
            (*(widgGetFromID(psRScreen, id) as *mut W_BUTTON)).pUserData as
                UDWORD;
        closeMultiRequester();
        *mode = context;
        return 1 as libc::c_int
    }
    strcpy(tmp.as_mut_ptr(), MultiCustomMapsPath.as_mut_ptr());
    strcat(tmp.as_mut_ptr(),
           b"*.wrf\x00" as *const u8 as *const libc::c_char);
    if id == (10600 as libc::c_int + 61 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        1 as libc::c_int as UBYTE,
                        current_numplayers as UBYTE);
    }
    if id == (10600 as libc::c_int + 62 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        2 as libc::c_int as UBYTE,
                        current_numplayers as UBYTE);
    }
    if id == (10600 as libc::c_int + 63 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        3 as libc::c_int as UBYTE,
                        current_numplayers as UBYTE);
    }
    if id == (10600 as libc::c_int + 70 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        current_tech as UBYTE, 0 as libc::c_int as UBYTE);
    }
    if id == (10600 as libc::c_int + 71 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        current_tech as UBYTE, 2 as libc::c_int as UBYTE);
    }
    if id == (10600 as libc::c_int + 72 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        current_tech as UBYTE, 4 as libc::c_int as UBYTE);
    }
    if id == (10600 as libc::c_int + 73 as libc::c_int) as libc::c_uint {
        closeMultiRequester();
        addMultiRequest(tmp.as_mut_ptr(), 10259 as libc::c_int as UDWORD,
                        current_tech as UBYTE, 8 as libc::c_int as UBYTE);
    }
    //	if( id == M_REQUEST_CA)
//	{
//		closeMultiRequester();
//		addMultiRequest("\\multiplay\\customMaps\\*.wrf",MULTIOP_MAP,0);
//	}
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Multiplayer in game menu stuff
// ////////////////////////////////////////////////////////////////////////////
// Display Functions
#[no_mangle]
pub unsafe extern "C" fn displayExtraGubbins(mut height: UDWORD) {
    let mut str: [libc::c_char; 128] = [0; 128];
    // draw timer
    getAsciiTime(str.as_mut_ptr(), gameTime);
    pie_DrawText(str.as_mut_ptr(),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint));
    //draw grid
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint))
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(height)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint))
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(height)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint))
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(height)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        50
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint))
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        50
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(height)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        50
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint))
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add((30
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        +
                                                                                                                                                        30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        36
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        32
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        95
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        50
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                        +
                                                                                                                                                        45
                                                                                                                                                            as
                                                                                                                                                            libc::c_int)
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint).wrapping_sub(6
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                      as
                                                                                                                                                                                      libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(height)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    pie_Line((10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint))
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(32
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
                 as libc::c_int,
             (10 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_add(620
                                                                                                                                                       as
                                                                                                                                                       libc::c_int
                                                                                                                                                       as
                                                                                                                                                       libc::c_uint)
                 as libc::c_int,
             (23 as libc::c_int as
                  libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)).wrapping_add(32
                                                                                                                                                        as
                                                                                                                                                        libc::c_int
                                                                                                                                                        as
                                                                                                                                                        libc::c_uint)
                 as libc::c_int,
             *colours.as_mut_ptr().offset(0 as libc::c_int as isize) as
                 uint32);
    //draw titles.
    iV_SetFont(WFont); // font
    iV_SetTextColour(-(1 as libc::c_int) as SWORD); //colour
    pie_DrawText(strresGetString(psStringRes,
                                 STR_MUL_ALLIANCES as libc::c_int as UDWORD),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint)); //,pl2;
    pie_DrawText(strresGetString(psStringRes,
                                 STR_MUL_SCORE as libc::c_int as UDWORD),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            45
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint)); //get the in game player number.
    pie_DrawText(strresGetString(psStringRes,
                                 STR_MUL_KILLS as libc::c_int as UDWORD),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            45
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint)); // font
    pie_DrawText(strresGetString(psStringRes,
                                 STR_MUL_PING as libc::c_int as UDWORD),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            45
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            50
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint)); //colour
    pie_DrawText(strresGetString(psStringRes,
                                 STR_MUL_PLAY as libc::c_int as UDWORD),
                 (10 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint).wrapping_div(2
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint)).wrapping_add((30
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            +
                                                                                                                                                            30
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            36
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            32
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            45
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            95
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            50
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                            +
                                                                                                                                                            45
                                                                                                                                                                as
                                                                                                                                                                libc::c_int)
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint),
                 (23 as libc::c_int as
                      libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)).wrapping_add(20
                                                                                                                                                            as
                                                                                                                                                            libc::c_int
                                                                                                                                                            as
                                                                                                                                                            libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn displayMultiPlayer(mut psWidget: *mut _widget,
                                            mut xOffset: UDWORD,
                                            mut yOffset: UDWORD,
                                            mut pColours: *mut UDWORD) {
    let mut str: [libc::c_char; 128] = [0; 128];
    let mut x: UDWORD = xOffset.wrapping_add((*psWidget).x as libc::c_uint);
    let mut y: UDWORD = yOffset.wrapping_add((*psWidget).y as libc::c_uint);
    let mut player: UDWORD = 0;
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    player = (*psWidget).pUserData as libc::c_int as UDWORD;
    if responsibleFor(player, 0 as libc::c_int as UDWORD) != 0 {
        displayExtraGubbins((*widgGetFromID(psWScreen,
                                            10600 as libc::c_int as
                                                UDWORD)).height as UDWORD);
    }
    iV_SetFont(WFont);
    iV_SetTextColour(-(1 as libc::c_int) as SWORD);
    if isHumanPlayer(player) != 0 ||
           game.type_0 as libc::c_int == 14 as libc::c_int &&
               player < game.maxPlayers as libc::c_uint {
        //c2:name,
        sprintf(str.as_mut_ptr(),
                b"%d:\x00" as *const u8 as *const libc::c_char, player);
        strcat(str.as_mut_ptr(), getPlayerName(player));
        while iV_GetTextWidth(str.as_mut_ptr()) >=
                  30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int -
                      (30 as libc::c_int + 30 as libc::c_int) -
                      10 as libc::c_int {
            str[strlen(str.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                          libc::c_uint) as
                    usize] = '\u{0}' as i32 as libc::c_char
        }
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int) as
                                        libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
        //c3-7 alliance
		//manage buttons by showing or hiding them. gifts only in campaign,
//		if(game.type != DMATCH)
        if game.alliance as libc::c_int != 0 as libc::c_int {
            if alliances[selectedPlayer as usize][player as usize] as
                   libc::c_int == 3 as libc::c_int {
                if player != selectedPlayer && giftsUp[player as usize] == 0 {
                    widgReveal(psWScreen,
                               ((10600 as libc::c_int + 2 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int) as
                                    libc::c_uint).wrapping_add(player));
                    widgReveal(psWScreen,
                               ((10600 as libc::c_int + 2 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int +
                                     8 as libc::c_int) as
                                    libc::c_uint).wrapping_add(player));
                    widgReveal(psWScreen,
                               ((10600 as libc::c_int + 2 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int) as
                                    libc::c_uint).wrapping_add(player));
                    widgReveal(psWScreen,
                               ((10600 as libc::c_int + 2 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int +
                                     8 as libc::c_int + 8 as libc::c_int +
                                     8 as libc::c_int) as
                                    libc::c_uint).wrapping_add(player));
                    giftsUp[player as usize] = 1 as libc::c_int
                }
            } else if player != selectedPlayer &&
                          giftsUp[player as usize] != 0 {
                widgHide(psWScreen,
                         ((10600 as libc::c_int + 2 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int) as
                              libc::c_uint).wrapping_add(player));
                widgHide(psWScreen,
                         ((10600 as libc::c_int + 2 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int +
                               8 as libc::c_int) as
                              libc::c_uint).wrapping_add(player));
                widgHide(psWScreen,
                         ((10600 as libc::c_int + 2 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int) as
                              libc::c_uint).wrapping_add(player));
                widgHide(psWScreen,
                         ((10600 as libc::c_int + 2 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int +
                               8 as libc::c_int + 8 as libc::c_int +
                               8 as libc::c_int) as
                              libc::c_uint).wrapping_add(player));
                giftsUp[player as usize] = 0 as libc::c_int
            }
        }
    }
    if isHumanPlayer(player) != 0 {
        //c8:score,
        sprintf(str.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                getMultiStats(player, 1 as libc::c_int).recentScore);
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int +
                                         95 as libc::c_int + 36 as libc::c_int
                                         + 36 as libc::c_int +
                                         32 as libc::c_int + 32 as libc::c_int
                                         + 32 as libc::c_int +
                                         45 as libc::c_int) as libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
        //c9:kills,
//		if(game.type == DMATCH)
//		{
//			sprintf(str,"%d",getMultiStats(player,TRUE).recentKills);
//		}
//		else
//		{
        sprintf(str.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                getMultiStats(player, 1 as libc::c_int).recentKills);
        //		}
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int +
                                         95 as libc::c_int + 36 as libc::c_int
                                         + 36 as libc::c_int +
                                         32 as libc::c_int + 32 as libc::c_int
                                         + 32 as libc::c_int +
                                         45 as libc::c_int +
                                         95 as libc::c_int) as libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
        //c10:ping
        if player != selectedPlayer {
            if ingame.PingTimes[player as usize] >
                   2000 as libc::c_int as libc::c_uint {
                sprintf(str.as_mut_ptr(),
                        b"***\x00" as *const u8 as *const libc::c_char);
            } else {
                sprintf(str.as_mut_ptr(),
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        ingame.PingTimes[player as usize]);
            }
            pie_DrawText(str.as_mut_ptr(),
                         x.wrapping_add((30 as libc::c_int + 30 as libc::c_int
                                             + 95 as libc::c_int +
                                             36 as libc::c_int +
                                             36 as libc::c_int +
                                             32 as libc::c_int +
                                             32 as libc::c_int +
                                             32 as libc::c_int +
                                             45 as libc::c_int +
                                             95 as libc::c_int +
                                             50 as libc::c_int) as
                                            libc::c_uint),
                         y.wrapping_add(20 as libc::c_int as libc::c_uint));
        }
        //c11:played
        sprintf(str.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                getMultiStats(player, 1 as libc::c_int).played);
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int +
                                         95 as libc::c_int + 36 as libc::c_int
                                         + 36 as libc::c_int +
                                         32 as libc::c_int + 32 as libc::c_int
                                         + 32 as libc::c_int +
                                         45 as libc::c_int + 95 as libc::c_int
                                         + 50 as libc::c_int +
                                         45 as libc::c_int) as libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
    } else {
        // estimate of score.
        sprintf(str.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                ingame.skScores[player as usize][0 as libc::c_int as usize]);
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int +
                                         95 as libc::c_int + 36 as libc::c_int
                                         + 36 as libc::c_int +
                                         32 as libc::c_int + 32 as libc::c_int
                                         + 32 as libc::c_int +
                                         45 as libc::c_int) as libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
        // estimated kills
        sprintf(str.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                ingame.skScores[player as usize][1 as libc::c_int as usize]);
        pie_DrawText(str.as_mut_ptr(),
                     x.wrapping_add((30 as libc::c_int + 30 as libc::c_int +
                                         95 as libc::c_int + 36 as libc::c_int
                                         + 36 as libc::c_int +
                                         32 as libc::c_int + 32 as libc::c_int
                                         + 32 as libc::c_int +
                                         45 as libc::c_int +
                                         95 as libc::c_int) as libc::c_uint),
                     y.wrapping_add(20 as libc::c_int as libc::c_uint));
    }
    // a droid of theirs.
    if !apsDroidLists[player as usize].is_null() {
        pie_SetGeometricOffset((10 as libc::c_int as
                                    libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint).wrapping_div(2
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         libc::c_uint)).wrapping_add(30
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_int
                                                                                                                                                                         as
                                                                                                                                                                         libc::c_uint)
                                   as libc::c_int,
                               y.wrapping_add(32 as libc::c_int as
                                                  libc::c_uint) as
                                   libc::c_int); //scale them!
        Rotation.x = -(15 as libc::c_int);
        Rotation.y = 45 as libc::c_int;
        Rotation.z = 0 as libc::c_int;
        Position.x = 0 as libc::c_int;
        Position.y = 0 as libc::c_int;
        Position.z = 2000 as libc::c_int;
        displayComponentButtonObject(apsDroidLists[player as usize],
                                     &mut Rotation, &mut Position,
                                     0 as libc::c_int, 100 as libc::c_int);
    }
    // clean up widgets if player leaves while menu is up.
    if isHumanPlayer(player) == 0 &&
           !(game.type_0 as libc::c_int == 14 as libc::c_int &&
                 player < game.maxPlayers as libc::c_uint) {
        if !widgGetFromID(psWScreen,
                          ((10600 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int + 8 as libc::c_int +
                                8 as libc::c_int + 8 as libc::c_int +
                                8 as libc::c_int + 8 as libc::c_int) as
                               libc::c_uint).wrapping_add(player)).is_null() {
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
        }
        if !widgGetFromID(psWScreen,
                          ((10600 as libc::c_int + 2 as libc::c_int +
                                8 as libc::c_int) as
                               libc::c_uint).wrapping_add(player)).is_null() {
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
            widgDelete(psWScreen,
                       ((10600 as libc::c_int + 2 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int + 8 as libc::c_int +
                             8 as libc::c_int) as
                            libc::c_uint).wrapping_add(player));
            giftsUp[player as usize] = 0 as libc::c_int
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// alliance display funcs
#[no_mangle]
pub unsafe extern "C" fn displayAllianceState(mut psWidget: *mut _widget,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    let mut a: UDWORD = 0; // replace with real gfx
    let mut b: UDWORD = 0; // replace with real gfx
    let mut c: UDWORD = 0; // replace with real gfx
    let mut player: UDWORD = 0;
    player = (*psWidget).pUserData as UDWORD;
    match alliances[selectedPlayer as usize][player as usize] as libc::c_int {
        0 => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_NOAL_HI as libc::c_int as UDWORD;
            c = IMAGE_MULTI_NOAL as libc::c_int as UDWORD
        }
        3 => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_AL_HI as libc::c_int as UDWORD;
            c = IMAGE_MULTI_AL as libc::c_int as UDWORD
        }
        1 | 2 => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_OFFAL_HI as libc::c_int as UDWORD;
            c = IMAGE_MULTI_OFFAL as libc::c_int as UDWORD
        }
        _ => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_NOAL_HI as libc::c_int as UDWORD;
            c = IMAGE_MULTI_NOAL as libc::c_int as UDWORD
        }
    }
    (*psWidget).pUserData =
        ((a & 0x3ff as libc::c_int as libc::c_uint) << 20 as libc::c_int |
             (b & 0x3ff as libc::c_int as libc::c_uint) << 10 as libc::c_int |
             c & 0x3ff as libc::c_int as libc::c_uint) as *mut libc::c_void;
    intDisplayImageHilight(psWidget, xOffset, yOffset, pColours);
    (*psWidget).pUserData = player as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn displayChannelState(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut a: UDWORD = 0;
    let mut b: UDWORD = 0;
    let mut c: UDWORD = 0;
    let mut player: UDWORD = 0;
    player = (*psWidget).pUserData as UDWORD;
    match openchannels[player as usize] {
        1 => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_CHAN as libc::c_int as UDWORD;
            c = IMAGE_MULTI_CHAN as libc::c_int as UDWORD
        }
        0 | _ => {
            a = 0 as libc::c_int as UDWORD;
            b = IMAGE_MULTI_NOCHAN as libc::c_int as UDWORD;
            c = IMAGE_MULTI_NOCHAN as libc::c_int as UDWORD
        }
    }
    (*psWidget).pUserData =
        ((a & 0x3ff as libc::c_int as libc::c_uint) << 20 as libc::c_int |
             (b & 0x3ff as libc::c_int as libc::c_uint) << 10 as libc::c_int |
             c & 0x3ff as libc::c_int as libc::c_uint) as *mut libc::c_void;
    intDisplayImageHilight(psWidget, xOffset, yOffset, pColours);
    (*psWidget).pUserData = player as *mut libc::c_void;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addMultiPlayer(mut player: UDWORD, mut pos: UDWORD) {
    let mut y: UDWORD = 0; // this is the top of the pos.
    let mut id: UDWORD = 0;
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
    y =
        (32 as libc::c_int as
             libc::c_uint).wrapping_mul(pos.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint));
    id =
        ((10600 as libc::c_int + 2 as libc::c_int) as
             libc::c_uint).wrapping_add(player);
    // add the whole thing.
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 10600 as libc::c_int as UDWORD;
    sFormInit.id = id;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = y as libc::c_short;
    sFormInit.width = (620 as libc::c_int - 4 as libc::c_int) as UWORD;
    sFormInit.height = 32 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(displayMultiPlayer as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData = player as *mut libc::c_void;
    widgAddForm(psWScreen, &mut sFormInit);
    //name,
	//score,
	//kills,
	//ping
	//ALL DONE IN THE DISPLAY FUNC.
    // add channel opener.
    if player != selectedPlayer {
        memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
        sButInit.formID = id;
        sButInit.style = 0 as libc::c_int as UDWORD;
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int) as
                SWORD;
        sButInit.y = 5 as libc::c_int as SWORD;
        sButInit.width = 35 as libc::c_int as UWORD;
        sButInit.height = 24 as libc::c_int as UWORD;
        sButInit.FontID = WFont;
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int + 8 as libc::c_int) as
                 libc::c_uint).wrapping_add(player);
        sButInit.pTip =
            b"channel\x00" as *const u8 as *const libc::c_char as *mut STRING;
        sButInit.pDisplay =
            Some(displayChannelState as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData = player as *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
    }
    if game.alliance as libc::c_int != 0 as libc::c_int &&
           player != selectedPlayer {
        //alliance
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int +
                 36 as libc::c_int) as SWORD;
        sButInit.y = 5 as libc::c_int as SWORD;
        sButInit.width = 35 as libc::c_int as UWORD;
        sButInit.height = 24 as libc::c_int as UWORD;
        sButInit.FontID = WFont;
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int) as
                 libc::c_uint).wrapping_add(player);
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_ALLI_STATE as libc::c_int as UDWORD);
        sButInit.pDisplay =
            Some(displayAllianceState as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButInit.pUserData = player as *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
        sButInit.pDisplay =
            Some(intDisplayImageHilight as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        // note buttons are up!
        //		}
        sButInit.y = (sButInit.y as libc::c_int + 1 as libc::c_int) as SWORD;
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int) as libc::c_uint).wrapping_add(player);
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int +
                 36 as libc::c_int + 36 as libc::c_int) as SWORD;
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_ALLI_VIS as libc::c_int as UDWORD);
        sButInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_MULTI_VIS_HI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_MULTI_VIS as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int + 8 as libc::c_int) as
                 libc::c_uint).wrapping_add(player);
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int +
                 36 as libc::c_int + 36 as libc::c_int + 32 as libc::c_int) as
                SWORD;
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_ALLI_TEC as libc::c_int as UDWORD);
        sButInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_MULTI_TEK_HI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_MULTI_TEK as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as
                 libc::c_uint).wrapping_add(player);
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int +
                 36 as libc::c_int + 36 as libc::c_int + 32 as libc::c_int +
                 32 as libc::c_int) as SWORD;
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_ALLI_DRO as libc::c_int as UDWORD);
        sButInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_MULTI_DRO_HI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_MULTI_DRO as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
        sButInit.id =
            ((10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                  8 as libc::c_int) as libc::c_uint).wrapping_add(player);
        sButInit.x =
            (30 as libc::c_int + 30 as libc::c_int + 95 as libc::c_int +
                 36 as libc::c_int + 36 as libc::c_int + 32 as libc::c_int +
                 32 as libc::c_int + 32 as libc::c_int) as SWORD;
        sButInit.pTip =
            strresGetString(psStringRes,
                            STR_ALLI_POW as libc::c_int as UDWORD);
        sButInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_MULTI_POW_HI as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_MULTI_POW as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        widgAddButton(psWScreen, &mut sButInit);
        giftsUp[player as usize] = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn intAddMultiMenu() -> BOOL {
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
    let mut i: UDWORD = 0;
    let mut pos: UDWORD = 0 as libc::c_int as UDWORD;
    let mut formHeight: UDWORD = 0 as libc::c_int as UDWORD;
    //		if(game.type != DMATCH)
//		{
			// add the gift buttons.
    // move down a wee bit.
    //check for already open.
    if !widgGetFromID(psWScreen, 10600 as libc::c_int as UDWORD).is_null() {
        intCloseMultiMenu();
        return 1 as libc::c_int
    }
    intResetScreen(0 as libc::c_int);
    // calculate required height.
    formHeight = (32 as libc::c_int + 7 as libc::c_int) as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //		if(isHumanPlayer(i) || (game.type == SKIRMISH && i<game.maxPlayers && game.skirmishPlayers[i] ))
        if isHumanPlayer(i) != 0 ||
               game.type_0 as libc::c_int == 14 as libc::c_int &&
                   i < game.maxPlayers as libc::c_uint &&
                   game.skDiff[i as usize] as libc::c_int != 0 {
            formHeight =
                (formHeight as
                     libc::c_uint).wrapping_add(32 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        i = i.wrapping_add(1)
    }
    // add form
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); //MULTIMENU_FORM_H;
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 10600 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (10 as libc::c_int as
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
        (23 as libc::c_int as
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
    sFormInit.width = 620 as libc::c_int as UWORD;
    sFormInit.height = formHeight as UWORD;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.disableChildren = 1 as libc::c_int;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    // add any players
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //		if(isHumanPlayer(i) || (game.type == SKIRMISH && i<game.maxPlayers && game.skirmishPlayers[i] ) )
        if isHumanPlayer(i) != 0 ||
               game.type_0 as libc::c_int == 14 as libc::c_int &&
                   i < game.maxPlayers as libc::c_uint &&
                   game.skDiff[i as usize] as libc::c_int != 0 {
            addMultiPlayer(i, pos);
            pos = pos.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    // Add the close button.
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as
               libc::c_ulong); // add power bar
    sButInit.formID =
        10600 as libc::c_int as UDWORD; // change interface mode.
    sButInit.id = (10600 as libc::c_int + 1 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (620 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 0 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_CLOSEHILIGHT as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_CLOSE as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    intShowPowerBar();
    intMode = INT_MULTIMENU;
    MultiMenuUp = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn intCloseMultiMenuNoAnim() {
    widgDelete(psWScreen,
               (10600 as libc::c_int + 1 as libc::c_int) as UDWORD);
    widgDelete(psWScreen, 10600 as libc::c_int as UDWORD);
    MultiMenuUp = 0 as libc::c_int;
    intMode = INT_NORMAL;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn intCloseMultiMenu() -> BOOL {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    widgDelete(psWScreen,
               (10600 as libc::c_int + 1 as libc::c_int) as UDWORD);
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 10600 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).pUserData = 0 as *mut libc::c_void;
        (*Form).disableChildren = 1 as libc::c_int;
        ClosingMultiMenu = 1 as libc::c_int;
        MultiMenuUp = 0 as libc::c_int
    }
    //intCloseMultiMenuNoAnim();
    intMode = INT_NORMAL;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// In Game Options house keeping stuff.
#[no_mangle]
pub unsafe extern "C" fn intRunMultiMenu() -> BOOL {
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn intCheckAllianceValid(mut player1: UBYTE,
                                               mut player2: UBYTE) -> BOOL {
    let mut i: UBYTE = 0;
    let mut iAlliances: UBYTE = 0;
    let mut iHumanPlayers: UBYTE = 0;
    /* only interested in teamplay */
    if bMultiPlayer != 0 && game.type_0 as libc::c_int != 13 as libc::c_int {
        return 1 as libc::c_int
    }
    /* throw out currently allied or human/computer alliances */
    if player1 as libc::c_int == player2 as libc::c_int ||
           aiCheckAlliances(player1 as UDWORD, player2 as UDWORD) != 0 ||
           !(isHumanPlayer(player1 as UDWORD) != 0 &&
                 isHumanPlayer(player2 as UDWORD) != 0) {
        return 0 as libc::c_int
    }
    /* get num human players */
    iHumanPlayers = 0 as libc::c_int as UBYTE;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        if isHumanPlayer(i as UDWORD) != 0 {
            iHumanPlayers = iHumanPlayers.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    /* count number of current alliances */
    iAlliances = 0 as libc::c_int as UBYTE;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        if isHumanPlayer(i as UDWORD) != 0 {
            if i as libc::c_int != player1 as libc::c_int &&
                   aiCheckAlliances(i as UDWORD, player1 as UDWORD) != 0 ||
                   i as libc::c_int != player2 as libc::c_int &&
                       aiCheckAlliances(i as UDWORD, player2 as UDWORD) != 0 {
                iAlliances = iAlliances.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    /* return FALSE if total alliances excedds max */
    if iAlliances as libc::c_int >=
           iHumanPlayers as libc::c_int - 2 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// multimenu
// ////////////////////////////////////////////////////////////////////////////
// process clicks made by user.
#[no_mangle]
pub unsafe extern "C" fn intProcessMultiMenu(mut id: UDWORD) {
    let mut i: UBYTE = 0;
    //close
    if id == (10600 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        intCloseMultiMenu();
    }
    //alliance button
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int) as libc::c_uint {
        i =
            id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                 8 as libc::c_int) as libc::c_uint) as UBYTE;
        match alliances[selectedPlayer as usize][i as usize] as libc::c_int {
            0 => {
                if intCheckAllianceValid(selectedPlayer as UBYTE, i) != 0 {
                    requestAlliance(selectedPlayer as UBYTE, i,
                                    1 as libc::c_int, 1 as libc::c_int);
                    // request an alliance
                }
            }
            2 => {
                if intCheckAllianceValid(selectedPlayer as UBYTE, i) != 0 {
                    formAlliance(selectedPlayer as UBYTE, i, 1 as libc::c_int,
                                 1 as libc::c_int);
                    // form an alliance
                }
            }
            1 => {
                breakAlliance(selectedPlayer as UBYTE, i, 1 as libc::c_int,
                              1 as libc::c_int); // break an alliance
            }
            3 => {
                if game.type_0 as libc::c_int != 13 as libc::c_int {
                    // cant break state in teamplay..
                    breakAlliance(selectedPlayer as UBYTE, i,
                                  1 as libc::c_int, 1 as libc::c_int);
                    // break an alliance
                }
            }
            _ => { }
        }
    }
    //channel opens.
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int + 8 as libc::c_int) as libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as
                   libc::c_uint {
        i =
            id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                 8 as libc::c_int + 8 as libc::c_int +
                                 8 as libc::c_int + 8 as libc::c_int +
                                 8 as libc::c_int + 8 as libc::c_int) as
                                libc::c_uint) as UBYTE;
        if openchannels[i as usize] != 0 {
            openchannels[i as usize] = 0 as libc::c_int
            // close channel
        } else {
            openchannels[i as usize] = 1 as libc::c_int
            // open channel
        }
    }
    //radar gifts
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int) as libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int) as libc::c_uint {
        sendGift(1 as libc::c_int as UDWORD,
                 id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int) as
                                     libc::c_uint));
    }
    // research gift
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int + 8 as libc::c_int) as libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as
                   libc::c_uint {
        sendGift(3 as libc::c_int as UDWORD,
                 id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int +
                                      8 as libc::c_int) as libc::c_uint));
    }
    //droid gift
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as
               libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int) as libc::c_uint {
        sendGift(2 as libc::c_int as UDWORD,
                 id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int) as
                                     libc::c_uint));
    }
    //power gift
    if id >=
           (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                8 as libc::c_int) as libc::c_uint &&
           id <
               (10600 as libc::c_int + 2 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int +
                    8 as libc::c_int + 8 as libc::c_int) as libc::c_uint {
        sendGift(4 as libc::c_int as UDWORD,
                 id.wrapping_sub((10600 as libc::c_int + 2 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int +
                                      8 as libc::c_int + 8 as libc::c_int +
                                      8 as libc::c_int) as libc::c_uint));
    };
}
// ////////////////////////////////////////////////////////////////////////////
/*
void intDisplayMiniMultiMenu(void)
{
	SDWORD sc[MAX_PLAYERS];
	SDWORD scp[MAX_PLAYERS];

	UDWORD j,i,temp;

	UDWORD x = RADTLX;
	UDWORD y = RADTLY - 60;
	UDWORD w = RADWIDTH;
	UDWORD h = 50;
	UDWORD players = 0;
	STRING	sTmp[64];

	if( ingame.localJoiningInProgress)
	{
		RenderWindowFrame(&FrameNormal,x, y ,w, h);			// draw a wee blu box.

		// display how far done..
		sprintf(sTmp,"%d%%", PERCENT(arenaPlayersReceived,MAX_PLAYERS) );
		iV_DrawText(sTmp, x+(w/2)-10, y+(h/2)+3);
	}
	else
	{
		for(i=0;i<MAX_PLAYERS;i++)							// clear out...
		{
			sc[i] = -1;
			scp[i] = -1;
		}

		for(j=0;j<MAX_PLAYERS;j++)							// find biggest score.
		{
			if(isHumanPlayer(j) )
			{
				players++;
				sc[j]  = getMultiStats(j,TRUE).recentScore;
				scp[j] = j;
			}
		}

		if(players <3)										// shrink box to fit.
		{
			y = RADTLY- (players*13) -18 ;
			h = players*13 + 8 ;
		}

		RenderWindowFrame(&FrameNormal,x, y ,w, h);			// draw a wee blu box.

		for(i=0;i<MAX_PLAYERS;++i)							// bubble sort.
		{
			for(j=MAX_PLAYERS-1;j>i;--j)
			{
				if (sc[j-1] > sc[j])
				{
					temp	= sc[j];
					sc[j]	= sc[j-1];
					sc[j-1] = temp;

					temp	= scp[j];
					scp[j]	= scp[j-1];
					scp[j-1] = temp;						// swap players too.
				}
			}
		}

		for(j=0;(scp[j] != (SDWORD)selectedPlayer) && (j<MAX_PLAYERS);j++);		// rate ourselves.

		iV_DrawText("1",x+5,y+13);							// display stuff
		strcpy(sTmp,getPlayerName(scp[7]));
		while(iV_GetTextWidth(sTmp) >= RADWIDTH-20 )
		{
			sTmp[strlen(sTmp)-1]='\0';
		}
		iV_DrawText(sTmp, x + 16, y + 13);

		if(players >1)
		{
			iV_DrawText("2",x+5,y+26);
			strcpy(sTmp,getPlayerName(scp[6]));
			while(iV_GetTextWidth(sTmp) >= RADWIDTH-20 )
			{
				sTmp[strlen(sTmp)-1]='\0';
			}
			iV_DrawText(sTmp, x + 16, y + 26);
		}

		if(players >2)
		{
			if(j!=7 && j!=6)
			{
				sprintf(sTmp,"%d",8-j);
				iV_DrawText(sTmp, x + 5, y + 39);

				strcpy(sTmp,getPlayerName(scp[selectedPlayer]));
				while(iV_GetTextWidth(sTmp) >= RADWIDTH-20 )
				{
					sTmp[strlen(sTmp)-1]='\0';
				}
				iV_DrawText(sTmp, x + 16, y + 39);
			}
			else
			{
				iV_DrawText("3",x+5,y+39);
				strcpy(sTmp,getPlayerName(scp[5]));
				while(iV_GetTextWidth(sTmp) >= RADWIDTH-20 )
				{
					sTmp[strlen(sTmp)-1]='\0';
				}
				iV_DrawText(sTmp, x + 16, y + 39);
			}
		}
	}
	return;
}
*/
