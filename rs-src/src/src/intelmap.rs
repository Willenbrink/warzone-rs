use ::libc;
extern "C" {
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    /* Set the current cursor from a Resource ID
 * This is the same as calling:
 *       frameSetCursor(LoadCursor(MAKEINTRESOURCE(resID)));
 * but with a bit of extra error checking.
 */
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    /* Add a form to the widget screen */
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    /* Add a label to the widget screen */
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    /* Add a button to a form */
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    /* Delete a widget from the screen */
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Set the current tabs for a tab form */
    #[no_mangle]
    fn widgSetTabs(psScreen: *mut W_SCREEN, id: UDWORD, major: UWORD,
                   minor: UWORD);
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Set a button or clickable form's state */
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    //mouse states
    #[no_mangle]
    fn pie_SetMouse(ImageFile: *mut IMAGEFILE, ImageID: UWORD);
    #[no_mangle]
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    /* **************************************************************************/
/*
 * pieclip.h
 *
 * clipping for all pumpkin image library functions.
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
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayStream(szFileName: *mut libc::c_char, iVol: SDWORD,
                        pUserCallback: AUDIO_CALLBACK) -> BOOL;
    #[no_mangle]
    fn audio_StopAll();
    /*
 * Message.h
 *
 * Functions for the messages shown in the Intelligence Map View
 */
    /* The lists of messages allocated */
    #[no_mangle]
    static mut apsMessages: [*mut MESSAGE; 8];
    /*remove a message */
    #[no_mangle]
    fn removeMessage(psDel: *mut MESSAGE, player: UDWORD);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    // Pass in a stat and get its name
    #[no_mangle]
    fn getStatName(pStat: *mut libc::c_void) -> *mut STRING;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // All the 2d graphics for the user interface.
    // A few useful defined frames.
    #[no_mangle]
    static mut FrameNormal: IMAGEFRAME;
    // A few useful defined tabs.
    #[no_mangle]
    static mut StandardTab: TABDEF;
    // Draws a transparent window
    #[no_mangle]
    fn RenderWindowFrame(Frame: *mut IMAGEFRAME, x: UDWORD, y: UDWORD,
                         Width: UDWORD, Height: UDWORD);
    #[no_mangle]
    static mut ObjectBuffers: [RENDERED_BUTTON; 40];
    #[no_mangle]
    fn GetObjectBuffer() -> SDWORD;
    #[no_mangle]
    fn ClearObjectBuffers();
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    #[no_mangle]
    fn OpenButtonRender(XPos: UWORD, YPos: UWORD, Width: UWORD,
                        Height: UWORD);
    #[no_mangle]
    fn CloseButtonRender();
    #[no_mangle]
    fn RenderToButton(ImageFile: *mut IMAGEFILE, ImageID: UWORD,
                      Object: *mut libc::c_void, Player: UDWORD,
                      Buffer: *mut RENDERED_BUTTON, Down: BOOL,
                      IMDType: UDWORD, buttonType: UDWORD);
    #[no_mangle]
    fn RenderImageToButton(ImageFile: *mut IMAGEFILE, ImageID: UWORD,
                           Buffer: *mut RENDERED_BUTTON, Down: BOOL,
                           buttonType: UDWORD);
    #[no_mangle]
    fn StatIsStructure(Stat: *mut BASE_STATS) -> BOOL;
    #[no_mangle]
    fn StatIsComponent(Stat: *mut BASE_STATS) -> SDWORD;
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intClosePlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    //Buffer to hold the 3D view for the Intelligence Screen
    #[no_mangle]
    static mut pIntelMapSurface: *mut iSurface;
    #[no_mangle]
    static mut ClosingMessageView: BOOL;
    #[no_mangle]
    static mut ClosingIntelMap: BOOL;
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    //sets up the Intelligence Screen as far as the interface is concerned
//extern void addIntelScreen(BOOL playImmediate);
    #[no_mangle]
    fn addIntelScreen();
    #[no_mangle]
    fn renderResearchToBuffer(pSurface: *mut iSurface,
                              psResearch: *mut RESEARCH, OriginX: UDWORD,
                              OriginY: UDWORD);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /* For a given view data get the research this is related to */
    #[no_mangle]
    fn getResearchForMsg(pViewData: *mut _viewdata) -> *mut RESEARCH;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    #[no_mangle]
    fn setGameUpdatePause(state: BOOL);
    #[no_mangle]
    fn setScriptPause(state: BOOL);
    #[no_mangle]
    fn setScrollPause(state: BOOL);
    #[no_mangle]
    fn setConsolePause(state: BOOL);
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    //stop
    //play once and hold last frame
    //position text
    //justify if less than 3/4 length
    //justify if less than 520/600
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
//buffer render
    #[no_mangle]
    fn seq_RenderVideoToBuffer(pSurface: *mut iSurface,
                               sequenceName: *mut libc::c_char,
                               time: libc::c_int, seqCommand: libc::c_int)
     -> BOOL;
    //full screen render
//extern BOOL seq_PlayVideo(char* pSeq, char* pAudio);
//extern BOOL seq_StartFullScreenVideo(char* sequenceFile, char* audioFile);//start videos through seqList
    //control
    //text
    #[no_mangle]
    fn seq_AddTextForVideo(pText: *mut libc::c_char, xOffset: SDWORD,
                           yOffset: SDWORD, startTime: SDWORD,
                           endTime: SDWORD, bJustify: SDWORD,
                           PSXSeqNumber: UDWORD) -> BOOL;
    //clear the sequence list
    #[no_mangle]
    fn seq_ClearSeqList();
    //add a sequence to the list to be played
    #[no_mangle]
    fn seq_AddSeqToList(pSeqName: *mut STRING, pAudioName: *mut STRING,
                        pTextName: *mut STRING, bLoop: BOOL,
                        PSXSeqNumber: UDWORD);
    /*returns the next sequence in the list to play*/
    #[no_mangle]
    fn seq_StartNextFullScreenVideo();
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn cdAudio_Pause() -> BOOL;
    #[no_mangle]
    fn cdAudio_Resume() -> BOOL;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    #[no_mangle]
    fn intSetCurrentCursorPosition(Snap: *mut CURSORSNAP, id: UDWORD);
    /*
 * IntelMap.c		(Intelligence Map)
 *
 * Functions for the display of the Intelligence Map
 *
 */
    /* Includes direct access to render library */
    //#include "geo.h"
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type WORD = libc::c_short;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
pub type STR_BLOCK = _str_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
/* A String Resource */
pub type STR_RES = _str_res;
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
// The treap to store string identifiers
// The store for the strings themselves
// Sizes for the string blocks
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
/* The optional user callback function */
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
/* The common widget data */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
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
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type BASE_STATS = _base_stats;
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _comp_base_stats {
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
}
pub type COMP_BASE_STATS = _comp_base_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct research_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techCode: UBYTE,
    pub techLevel: TECH_LEVEL,
    pub subGroup: UWORD,
    pub researchPoints: UWORD,
    pub researchPower: UDWORD,
    pub keyTopic: UBYTE,
    pub storeCount: UBYTE,
    pub numPRRequired: UBYTE,
    pub pPRList: *mut UWORD,
    pub numStructures: UBYTE,
    pub pStructList: *mut UWORD,
    pub numFunctions: UBYTE,
    pub pFunctionList: *mut *mut _function,
    pub numRedStructs: UBYTE,
    pub pRedStructs: *mut UWORD,
    pub numRedArtefacts: UBYTE,
    pub pRedArtefacts: *mut *mut COMP_BASE_STATS,
    pub numStructResults: UBYTE,
    pub pStructureResults: *mut UWORD,
    pub numArteResults: UBYTE,
    pub pArtefactResults: *mut *mut COMP_BASE_STATS,
    pub pReplacedArtefacts: *mut *mut COMP_BASE_STATS,
    pub pViewData: *mut _viewdata,
    pub iconID: UWORD,
    pub psStat: *mut BASE_STATS,
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewdata {
    pub pName: *mut STRING,
    pub type_0: VIEW_TYPE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pData: *mut libc::c_void,
}
pub type VIEW_TYPE = _view_type;
/*
 * messageDef.h
 *
 * Message structure definitions
 */
// Research message
// Campaign message
// Mission Report messages
// Proximity message
pub type _view_type = libc::c_uint;
// full screen view sequence - flic.	extended format
pub const VIEW_TYPES: _view_type = 4;
// proximity view - no view really!
pub const VIEW_RPLX: _view_type = 3;
// full screen view sequence - flic
pub const VIEW_PROX: _view_type = 2;
// research view
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
pub type RESEARCH = research_stats;
/* * unsigned 32-bit integer */
pub type ALuint = libc::c_uint;
/* **************************************************************************/
/* **************************************************************************/
/* defines */
/* **************************************************************************/
/* **************************************************************************/
/* enums */
/* **************************************************************************/
/* forward definitions
 */
/* **************************************************************************/
/* typedefs
 */
/* **************************************************************************/
/* structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_SAMPLE {
    pub iTrack: SDWORD,
    pub iSample: ALuint,
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub iLoops: SDWORD,
    pub bRemove: BOOL,
    pub pCallback: AUDIO_CALLBACK,
    pub psObj: *mut libc::c_void,
    pub psPrev: *mut AUDIO_SAMPLE,
    pub psNext: *mut AUDIO_SAMPLE,
}
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub type MESSAGE_TYPE = _message_type;
// info required to view an object in Intelligence screen
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_research {
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
    pub sequenceName: [STRING; 256],
    pub pAudio: *mut STRING,
    pub numFrames: UWORD,
}
pub type VIEW_RESEARCH = _view_research;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _seq_display {
    pub sequenceName: [STRING; 256],
    pub flag: UBYTE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pAudio: *mut STRING,
    pub numFrames: UWORD,
}
pub type SEQ_DISPLAY = _seq_display;
/* On PSX if type is VIEW_RPL then
											this is used as a number_of_frames_in_the_stream
											count - NOT used on PC*/
/* On PSX if type is VIEW_RPL then
								this is used as a number_of_frames_in_the_stream
								count - NOT used on PC*/
//info required to view a flic in Intelligence Screen
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_replay {
    pub numSeq: UBYTE,
    pub pSeqList: *mut SEQ_DISPLAY,
}
pub type VIEW_REPLAY = _view_replay;
pub type VIEWDATA = _viewdata;
pub type MSG_VIEWDATA = *mut libc::c_void;
//STRING		**ppSeqName;
	//UBYTE		numText;	//the number of textmessages associated with this sequence
	//STRING		**ppTextMsg;	//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
//base structure for each message
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _message {
    pub type_0: MESSAGE_TYPE,
    pub id: UDWORD,
    pub pViewData: *mut MSG_VIEWDATA,
    pub read: BOOL,
    pub player: UDWORD,
    pub psNext: *mut _message,
}
pub type MESSAGE = _message;
//pointer to the next in the list
//used to display the text messages in 3D view of the Intel display
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _text_display {
    pub totalFrames: UDWORD,
    pub startTime: UDWORD,
    pub font: UDWORD,
    pub fontColour: UWORD,
    pub text: [STRING; 255],
}
pub type TEXT_DISPLAY = _text_display;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
//storage to hold the currently displayed text
/* Information for a minor tab */
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Widgets on the tab
// Tool tip
/* Information for a major tab */
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
/* Graphics data for the tab will go here */
// Store which was the last selected minor tab
// Minor tab information
/* The tabbed form data structure */
pub type W_TABFORM = _w_tabform;
/* The common form data */
// Position of the tabs on the form
// the size of tabs horizontally and vertically
// The thickness of the tabs
// The thickness of the tabs
// The gap between tabs
// The gap between tabs
// Tab form overlap offset.
// Tab form overlap offset.
// Tab start offset.
// Tab start offset.
// which tab is selected
// Current state of the widget
// which tab is hilited.
/* NOTE: If tabHiLite is (UWORD)(-1) then there is no hilite.  A bit of a hack I know */
	/*       but I don't really have the energy to change it.  (Don't design stuff after  */
	/*       beers at lunch-time :-)                                                      */
// The number of major tabs
// The major tab information
// Optional callback for display tabs.
// Optional callback to display the form.
/* Button states for a clickable form */
// Button is down
// Button is disabled
// Button is hilited
// Button is locked down
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
/* The clickable form data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_clickform {
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
    pub state: UDWORD,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
pub type W_CLICKFORM = _w_clickform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAMERECT {
    pub Type: UWORD,
    pub TLXOffset: SWORD,
    pub TLYOffset: SWORD,
    pub BRXOffset: SWORD,
    pub BRYOffset: SWORD,
    pub ColourIndex: UBYTE,
}
// Pointer to audio callback function
// Frame definition structure.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFRAME {
    pub OffsetX0: SWORD,
    pub OffsetY0: SWORD,
    pub OffsetX1: SWORD,
    pub OffsetY1: SWORD,
    pub TopLeft: SWORD,
    pub TopRight: SWORD,
    pub BottomLeft: SWORD,
    pub BottomRight: SWORD,
    pub TopEdge: SWORD,
    pub TopType: SWORD,
    pub RightEdge: SWORD,
    pub RightType: SWORD,
    pub BottomEdge: SWORD,
    pub BottomType: SWORD,
    pub LeftEdge: SWORD,
    pub LeftType: SWORD,
    pub FRect: [FRAMERECT; 5],
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IMDTYPE_STRUCTURESTAT: C2RustUnnamed_0 = 6;
pub const IMDTYPE_RESEARCH: C2RustUnnamed_0 = 5;
pub const IMDTYPE_STRUCTURE: C2RustUnnamed_0 = 4;
pub const IMDTYPE_COMPONENT: C2RustUnnamed_0 = 3;
pub const IMDTYPE_DROIDTEMPLATE: C2RustUnnamed_0 = 2;
pub const IMDTYPE_DROID: C2RustUnnamed_0 = 1;
pub const IMDTYPE_NONE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTTON_SURFACE {
    pub Buffer: *mut uint8,
    pub Surface: *mut iSurface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDERED_BUTTON {
    pub InUse: BOOL,
    pub Initialised: BOOL,
    pub ImdRotation: SDWORD,
    pub State: UDWORD,
    pub Data: *mut libc::c_void,
    pub Data2: *mut libc::c_void,
    pub ButSurf: *mut BUTTON_SURFACE,
}
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
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
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub type TRIGGER_TYPE = _trigger_type;
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
// flag to specify a variable reference rather than simple value
/* A value consists of its type and value */
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
/* Opcodes for the script interpreter */
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
/* The type of function called to access an object or in-game variable */
/* The possible storage types for a variable */
// Public variable
// Private variable
// A value stored in an objects data space.
// An external value accessed by function call
// A local variable
/* Variable debugging info for a script */
/* Array info for a script */
// the base index of the array values
// the array data type
/* Array debug info for a script */
/* Line debugging information for a script */
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type _trigger_type = libc::c_uint;
// The user defined callback triggers should start with this id
// Event has paused for an interval and will restart in the middle of it's code
pub const TR_CALLBACKSTART: _trigger_type = 5;
// Trigger at repeated intervals
pub const TR_PAUSE: _trigger_type = 4;
// Trigger after a time pause
pub const TR_EVERY: _trigger_type = 3;
// Trigger uses script code
pub const TR_WAIT: _trigger_type = 2;
// Trigger fires when the script is first run
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
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
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_ATTACKED: _scr_callback_types = 38;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_MANURUN: _scr_callback_types = 24;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
/* the widget screen */
/* Static variables ********************/
//static SDWORD			viewAngle;
//static SDWORD			viewHeight;
static mut messageID: UDWORD = 0;
static mut immediateMessage: BOOL = 0 as libc::c_int;
//How many proximity messages are currently being displayed
//static UDWORD			numProxMsg;
//flags whether to open the Intel Screen with a message
static mut playCurrent: BOOL = 0;
/* ********************** VARIABLES ****************************/
// The current message being displayed
#[no_mangle]
pub static mut psCurrentMsg: *mut MESSAGE =
    0 as *const MESSAGE as *mut MESSAGE;
// The display stats for the current messages' text
#[no_mangle]
pub static mut currentTextDisplay: TEXT_DISPLAY =
    TEXT_DISPLAY{totalFrames: 0,
                 startTime: 0,
                 font: 0,
                 fontColour: 0,
                 text: [0; 255],};
/* Add the Intelligence Map widgets to the widget screen */
//BOOL intAddIntelMap(BOOL playCurrent)
#[no_mangle]
pub unsafe extern "C" fn _intAddIntelMap() -> BOOL {
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
    let mut Animate: BOOL = 1 as libc::c_int;
    //check playCurrent with psCurrentMsg
    if psCurrentMsg.is_null() {
        playCurrent = 0 as libc::c_int
    } else { playCurrent = 1 as libc::c_int }
    // Is the form already up?
    if !widgGetFromID(psWScreen, 6000 as libc::c_int as UDWORD).is_null() {
        intRemoveIntelMapNoAnim();
        Animate = 0 as libc::c_int
    } else { audio_StopAll(); }
    cdAudio_Pause();
    //add message to indicate game is paused - single player mode
    if bMultiPlayer == 0 {
        if widgGetFromID(psWScreen, 6005 as libc::c_int as UDWORD).is_null() {
            memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
            sLabInit.id = 6005 as libc::c_int as UDWORD;
            sLabInit.formID = 0 as libc::c_int as UDWORD;
            sLabInit.style = 0 as libc::c_int as UDWORD;
            sLabInit.x = 23 as libc::c_int as SWORD;
            sLabInit.y = (10 as libc::c_int + 0 as libc::c_int) as SWORD;
            sLabInit.width = 60 as libc::c_int as UWORD;
            sLabInit.height = 20 as libc::c_int as UWORD;
            sLabInit.pText =
                strresGetString(psStringRes,
                                STR_MISC_PAUSED as libc::c_int as UDWORD);
            sLabInit.FontID = WFont;
            if widgAddLabel(psWScreen, &mut sLabInit) == 0 {
                return 0 as libc::c_int
            }
        }
    }
    //set pause states before putting the interface up
    setIntelligencePauseState();
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    // Add the main Intelligence Map form
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 6000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
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
        (324 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
            as SWORD;
    sFormInit.width = 320 as libc::c_int as UWORD;
    sFormInit.height = 115 as libc::c_int as UWORD;
    // If the window was closed then do open animation.
    if Animate != 0 {
        sFormInit.pDisplay =
            Some(intOpenPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.disableChildren = 1 as libc::c_int
    } else {
        // otherwise just recreate it.
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    //sFormInit.pDisplay = intDisplayPlainForm;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //#ifdef PSX
//	SetCurrentSnapFormID(&InterfaceSnap,sFormInit.id);
// //	SetMouseFormPosition(&sFormInit);
//#endif
    if intAddMessageForm(playCurrent) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* functions declarations ****************/
/* Add the Message sub form */
unsafe extern "C" fn intAddMessageForm(mut playCurrent_0: BOOL) -> BOOL {
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
    let mut sBFormInit: W_FORMINIT =
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
    let mut numButtons: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut BufferID: SDWORD = 0;
    /* Add the Message form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 6000 as libc::c_int as UDWORD;
    sFormInit.id = 6001 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.width = 316 as libc::c_int as UWORD;
    sFormInit.height = 112 as libc::c_int as UWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    numButtons = 0 as libc::c_int as UDWORD;
    //numProxMsg = 0;
	/*work out the number of buttons */
    psMessage = apsMessages[selectedPlayer as usize];
    while !psMessage.is_null() {
        //ignore proximity messages here
        if !((*psMessage).type_0 as libc::c_uint ==
                 MSG_PROXIMITY as libc::c_int as libc::c_uint) {
            numButtons = numButtons.wrapping_add(1)
        }
        //stop adding the buttons once max has been reached
        if numButtons >
               (6139 as libc::c_int - 6100 as libc::c_int) as libc::c_uint {
            break ;
        }
        psMessage = (*psMessage).psNext
    }
    //set the number of tabs required
    sFormInit.numMajor =
        numForms(((60 as libc::c_int + 2 as libc::c_int) as
                      libc::c_uint).wrapping_mul(numButtons),
                 ((316 as libc::c_int - 2 as libc::c_int) * 2 as libc::c_int)
                     as UDWORD);
    //set minor tabs to 1
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
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
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the message buttons */
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 6001 as libc::c_int as UDWORD;
    sBFormInit.id = 6100 as libc::c_int as UDWORD;
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.style = 4 as libc::c_int as UDWORD;
    sBFormInit.x = 2 as libc::c_int as SWORD;
    sBFormInit.y = 0 as libc::c_int as SWORD;
    sBFormInit.width = 60 as libc::c_int as UWORD;
    sBFormInit.height = 46 as libc::c_int as UWORD;
    ClearObjectBuffers();
    //add each button
    messageID = 0 as libc::c_int as UDWORD;
    psMessage = apsMessages[selectedPlayer as usize];
    while !psMessage.is_null() {
        /*if (psMessage->type == MSG_TUTORIAL)
		{
			//tutorial cases should never happen
			ASSERT( FALSE, "Tutorial message in Intelligence screen!" );
			continue;
		}*/
        if !((*psMessage).type_0 as libc::c_uint ==
                 MSG_PROXIMITY as libc::c_int as libc::c_uint) {
            /* Set the tip and add the button */
            match (*psMessage).type_0 as libc::c_uint {
                0 => {
                    psResearch =
                        getResearchForMsg((*psMessage).pViewData as
                                              *mut VIEWDATA);
                    if !psResearch.is_null() {
                        sBFormInit.pTip =
                            getStatName(psResearch as *mut libc::c_void)
                    } else {
                        sBFormInit.pTip =
                            strresGetString(psStringRes,
                                            STR_INT_RESMESSAGE as libc::c_int
                                                as UDWORD)
                    }
                }
                1 => {
                    sBFormInit.pTip =
                        strresGetString(psStringRes,
                                        STR_INT_GENMESSAGE as libc::c_int as
                                            UDWORD)
                }
                2 => {
                    sBFormInit.pTip =
                        strresGetString(psStringRes,
                                        STR_INT_MISMESSAGE as libc::c_int as
                                            UDWORD)
                }
                _ => { }
            }
            BufferID = GetObjectBuffer();
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Unable to acquire object buffer.\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intelmap.c\x00" as *const u8 as *const libc::c_char,
                      523 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"intAddMessageForm\x00")).as_ptr(),
                      b"BufferID >= 0\x00" as *const u8 as
                          *const libc::c_char);
            };
            ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            ObjectBuffers[BufferID as usize].Data =
                psMessage as *mut libc::c_void;
            sBFormInit.pUserData =
                &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit.pDisplay =
                Some(intDisplayMessageButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
                return 0 as libc::c_int
            }
            intSetCurrentCursorPosition(&mut InterfaceSnap, sBFormInit.id);
            /* if the current message matches psSelected lock the button */
            if psMessage == psCurrentMsg {
                messageID = sBFormInit.id;
                widgSetButtonState(psWScreen, messageID,
                                   0x2 as libc::c_int as UDWORD);
                widgSetTabs(psWScreen, 6001 as libc::c_int as UDWORD,
                            sBFormInit.majorID, 0 as libc::c_int as UWORD);
            }
            /* Update the init struct for the next button */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            //stop adding the buttons when at max
            if sBFormInit.id > 6139 as libc::c_int as libc::c_uint { break ; }
            if sBFormInit.id <
                   (6139 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many message buttons\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBFormInit.id <
                   (6139 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intelmap.c\x00" as *const u8 as *const libc::c_char,
                      553 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"intAddMessageForm\x00")).as_ptr(),
                      b"sBFormInit.id < (IDINTMAP_MSGEND+1)\x00" as *const u8
                          as *const libc::c_char);
            };
            sBFormInit.x =
                (sBFormInit.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 316 as libc::c_int {
                sBFormInit.x = 2 as libc::c_int as SWORD;
                sBFormInit.y =
                    (sBFormInit.y as libc::c_int +
                         (46 as libc::c_int + 2 as libc::c_int)) as SWORD
            }
            if sBFormInit.y as libc::c_int + 46 as libc::c_int +
                   2 as libc::c_int > 112 as libc::c_int {
                sBFormInit.y = 0 as libc::c_int as SWORD;
                sBFormInit.majorID =
                    (sBFormInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
        }
        //ignore proximity messages here
        psMessage = (*psMessage).psNext
    }
    //check to play current message instantly
    if playCurrent_0 != 0 {
        //is it a proximity message?
        if !((*psCurrentMsg).type_0 as libc::c_uint ==
                 MSG_PROXIMITY as libc::c_int as libc::c_uint) {
            intIntelButtonPressed(0 as libc::c_int, messageID);
        }
    }
    return 1 as libc::c_int;
}
/*Add the 3D world view for the current message */
/*Add the 3D world view for the particular message (only research nmessages now) */
#[no_mangle]
pub unsafe extern "C" fn intAddMessageView(mut psMessage: *mut MESSAGE)
 -> BOOL {
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
    let mut Animate: BOOL = 1 as libc::c_int;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    /*	ASSERT( psMessage->type == MSG_RESEARCH,
 *		"intAddMessageView: invalid message type" );
 *	had to comment out this check, since the 'Fast Play' tutorial triggered it
 *	with psMessage->type=MSG_MISSION and ((VIEWDATA)*psMessage->pViewData)->type=VIEW_RPL,
 * 	but which is probably using the wrong function. - Per
 */
    // Is the form already up?
    if !widgGetFromID(psWScreen, 6002 as libc::c_int as UDWORD).is_null() {
        intRemoveMessageView(0 as libc::c_int);
        Animate = 0 as libc::c_int
    }
    /* Add the base form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 6002 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    //size and position depends on the type of message - ONLY RESEARCH now
    sFormInit.width = 440 as libc::c_int as UWORD;
    sFormInit.height = 288 as libc::c_int as UWORD;
    sFormInit.x =
        (100 as libc::c_int as
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
    /*switch (type)
	{
	case MSG_RESEARCH:
		sFormInit.width = INTMAP_CAMPAIGNWIDTH;
		sFormInit.height = INTMAP_CAMPAIGNHEIGHT;
		sFormInit.x = INTMAP_CAMPAIGNX;
		sFormInit.y = INTMAP_CAMPAIGNY;
		break;
	//these are Full Screen FMV now
	case MSG_CAMPAIGN:
	case MSG_MISSION:
	//case MSG_TUTORIAL:
		sFormInit.width = INTMAP_MISSIONWIDTH;
		sFormInit.height = INTMAP_MISSIONHEIGHT;
		sFormInit.x = INTMAP_MISSIONX;
		sFormInit.y = INTMAP_MISSIONY;
		break;
	//these are no longer displayed in Intel Screen
	case MSG_PROXIMITY:
		sFormInit.width = INTMAP_PROXIMITYWIDTH;
		sFormInit.height = INTMAP_PROXIMITYHEIGHT;
		sFormInit.x = INTMAP_PROXIMITYX;
		sFormInit.y = INTMAP_PROXIMITYY;
		break;
	default:
		ASSERT( FALSE, "Unknown message type" );
		return FALSE;
	}*/
    // If the window was closed then do open animation.
    if Animate != 0 {
        sFormInit.pDisplay =
            Some(intOpenPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.disableChildren = 1 as libc::c_int
    } else {
        // otherwise just display it.
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the close box */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 6002 as libc::c_int as UDWORD;
    sButInit.id = 6004 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x =
        (sFormInit.width as libc::c_int - 5 as libc::c_int -
             15 as libc::c_int) as SWORD;
    sButInit.y = 5 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
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
    if (*psMessage).type_0 as libc::c_uint !=
           MSG_RESEARCH as libc::c_int as libc::c_uint &&
           (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as libc::c_uint
               == VIEW_RPL as libc::c_int as libc::c_uint {
        let mut sTabForm: W_FORMINIT =
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
        let mut psViewReplay: *mut VIEW_REPLAY = 0 as *mut VIEW_REPLAY;
        let mut i: size_t = 0;
        let mut cur_seq: size_t = 0;
        let mut cur_seqpage: size_t = 0;
        psViewReplay =
            (*((*psMessage).pViewData as *mut VIEWDATA)).pData as
                *mut VIEW_REPLAY;
        /* Add a big tabbed text box for the subtitle text */
        memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
        sFormInit.id = 6011 as libc::c_int as UDWORD;
        sFormInit.formID = 6002 as libc::c_int as UDWORD;
        sFormInit.style = 1 as libc::c_int as UDWORD;
        sFormInit.x = 0 as libc::c_int as SWORD;
        sFormInit.y = 0 as libc::c_int as SWORD;
        sFormInit.width = 440 as libc::c_int as UWORD;
        sFormInit.height = 288 as libc::c_int as UWORD;
        sFormInit.majorPos = 4 as libc::c_int as UWORD;
        sFormInit.minorPos = 0 as libc::c_int as UWORD;
        sFormInit.majorSize = 26 as libc::c_int as UWORD;
        sFormInit.majorOffset = 2 as libc::c_int as SWORD;
        sFormInit.tabVertOffset =
            (11 as libc::c_int / 2 as libc::c_int) as SWORD;
        sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
        sFormInit.numMajor = 0 as libc::c_int as UWORD;
        cur_seqpage = 0 as libc::c_int as size_t;
        cur_seq = cur_seqpage;
        loop  {
            sFormInit.aNumMinors[sFormInit.numMajor as usize] =
                1 as libc::c_int as UWORD;
            sFormInit.numMajor = sFormInit.numMajor.wrapping_add(1);
            if !(intDisplaySeqTextViewPage(psViewReplay,
                                           0 as libc::c_int as UDWORD,
                                           0 as libc::c_int as UDWORD,
                                           sFormInit.width as UDWORD,
                                           sFormInit.height as UDWORD,
                                           0 as libc::c_int, &mut cur_seq,
                                           &mut cur_seqpage) == 0) {
                break ;
            }
        }
        sFormInit.pFormDisplay =
            Some(intDisplayObjectForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.pUserData =
            &mut StandardTab as *mut TABDEF as *mut libc::c_void;
        sFormInit.pTabDisplay =
            Some(intDisplayTab as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: UDWORD, _: BOOL,
                                          _: BOOL, _: UDWORD, _: UDWORD,
                                          _: UDWORD, _: UDWORD) -> ());
        if widgAddForm(psWScreen, &mut sFormInit) == 0 {
            return 0 as libc::c_int
        }
        memset(&mut sTabForm as *mut W_FORMINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
        sTabForm.formID = 6011 as libc::c_int as UDWORD;
        sTabForm.id = 6200 as libc::c_int as UDWORD;
        sTabForm.majorID = 0 as libc::c_int as UWORD;
        sTabForm.minorID = 0 as libc::c_int as UWORD;
        sTabForm.style = 0 as libc::c_int as UDWORD;
        sTabForm.x = 0 as libc::c_int as SWORD;
        sTabForm.y = 0 as libc::c_int as SWORD;
        sTabForm.width = 440 as libc::c_int as UWORD;
        sTabForm.height = 288 as libc::c_int as UWORD;
        sTabForm.pDisplay =
            Some(intDisplaySeqTextView as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sTabForm.pUserData = psViewReplay as *mut libc::c_void;
        i = 0 as libc::c_int as size_t;
        while i < sFormInit.numMajor as libc::c_uint {
            sTabForm.id =
                (6200 as libc::c_int as libc::c_uint).wrapping_add(i);
            sTabForm.majorID = i as UWORD;
            if widgAddForm(psWScreen, &mut sTabForm) == 0 {
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
        return 1 as libc::c_int
    }
    /*Add the Title box*/
	/*memset(&sFormInit, 0, sizeof(W_FORMINIT));
	sFormInit.formID = IDINTMAP_MSGVIEW;
	sFormInit.id = IDINITMAP_TITLEVIEW;
	sFormInit.style = WFORM_PLAIN;
	sFormInit.x = INTMAP_TITLEX;
	sFormInit.y = INTMAP_TITLEY;
	sFormInit.width = INTMAP_TITLEWIDTH;
	sFormInit.height = INTMAP_TITLEHEIGHT;
	sFormInit.pDisplay = intDisplayPlainForm;
	if (!widgAddForm(psWScreen, &sFormInit))
	{
		return FALSE;
	}*/
    /*add the Label for the title box*/
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.id = 6010 as libc::c_int as UDWORD;
    sLabInit.formID = 6002 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = (0 as libc::c_int + 5 as libc::c_int) as SWORD;
    sLabInit.y = (0 as libc::c_int + 5 as libc::c_int) as SWORD;
    sLabInit.width = 440 as libc::c_int as UWORD;
    sLabInit.height = 18 as libc::c_int as UWORD;
    //print research name in title bar
    if (*psMessage).type_0 as libc::c_uint !=
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"intAddMessageView:Invalid message type for research\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psMessage).type_0 as libc::c_uint !=
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intelmap.c\x00" as *const u8 as *const libc::c_char,
              789 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intAddMessageView\x00")).as_ptr(),
              b"psMessage->type != MSG_PROXIMITY\x00" as *const u8 as
                  *const libc::c_char);
    };
    psResearch = getResearchForMsg((*psMessage).pViewData as *mut VIEWDATA);
    if !psResearch.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Research not found\x00" as *const u8 as *const libc::c_char);
    };
    if !psResearch.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intelmap.c\x00" as *const u8 as *const libc::c_char,
              793 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"intAddMessageView\x00")).as_ptr(),
              b"psResearch!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    //sLabInit.pText=psResearch->pName;
    sLabInit.pText = getStatName(psResearch as *mut libc::c_void);
    sLabInit.FontID = WFont;
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    /*Add the PIE box*/
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 6002 as libc::c_int as UDWORD;
    sFormInit.id = 6007 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 3 as libc::c_int as SWORD;
    sFormInit.y = 24 as libc::c_int as SWORD;
    sFormInit.width = 238 as libc::c_int as UWORD;
    sFormInit.height = 169 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayPIEView as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData = psMessage as *mut libc::c_void;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /*Add the text box*/
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 6002 as libc::c_int as UDWORD;
    sFormInit.id = 6009 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 0 as libc::c_int as SWORD;
    sFormInit.y = 200 as libc::c_int as SWORD;
    sFormInit.width = 440 as libc::c_int as UWORD;
    sFormInit.height = 88 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayTEXTView as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData = psMessage as *mut libc::c_void;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* Process return codes from the Intelligence Map */
/* Process return codes from the Intelligence Map */
#[no_mangle]
pub unsafe extern "C" fn intProcessIntelMap(mut id: UDWORD) {
    if id >= 6100 as libc::c_int as libc::c_uint &&
           id <= 6139 as libc::c_int as libc::c_uint {
        intIntelButtonPressed(0 as libc::c_int, id);
    } else if id == 6004 as libc::c_int as libc::c_uint {
        /*else if (id >= IDINTMAP_PROXSTART && id <= IDINTMAP_PROXEND)
	{
		intIntelButtonPressed(TRUE, id);
	}*/
        //if close button pressed on 3D View then close the view only
        psCurrentMsg = 0 as *mut MESSAGE;
        //initTextDisplay(psCurrentMsg, WFont, 255);
        intRemoveMessageView(1 as libc::c_int);
    };
}
unsafe extern "C" fn intDisplaySeqTextViewPage(mut psViewReplay:
                                                   *mut VIEW_REPLAY,
                                               mut x0: UDWORD, mut y0: UDWORD,
                                               mut width: UDWORD,
                                               mut height: UDWORD,
                                               mut render: BOOL,
                                               mut cur_seq: *mut size_t,
                                               mut cur_seqpage: *mut size_t)
 -> BOOL {
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut linePitch: UDWORD = 0;
    let mut cur_y: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut sequence: UDWORD = 0;
    if psViewReplay.is_null() {
        return 1 as libc::c_int
        /* nothing to do */
    }
    x1 = x0.wrapping_add(width);
    y1 = y0.wrapping_add(height);
    ty = y0;
    iV_SetFont(WFont);
    /* Get the travel to the next line */
    linePitch = iV_GetTextLineSize() as UDWORD;
    /* Fix for spacing.... */
    linePitch =
        (linePitch as
             libc::c_uint).wrapping_add(6 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    ty =
        (ty as libc::c_uint).wrapping_add(3 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    iV_SetTextColour(pal_GetNearestColour(255 as libc::c_int as uint8,
                                          255 as libc::c_int as uint8,
                                          255 as libc::c_int as uint8) as
                         SWORD);
    cur_y = 0 as libc::c_int as UDWORD;
    /* add each message */
    sequence = *cur_seq;
    i = *cur_seqpage;
    while sequence < (*psViewReplay).numSeq as libc::c_uint {
        let mut psSeqDisplay: *mut SEQ_DISPLAY =
            &mut *(*psViewReplay).pSeqList.offset(sequence as isize) as
                *mut SEQ_DISPLAY;
        while i < (*psSeqDisplay).numText as libc::c_uint {
            if render != 0 {
                pie_DrawText(*(*psSeqDisplay).ppTextMsg.offset(i as isize),
                             x0.wrapping_add(5 as libc::c_int as
                                                 libc::c_uint),
                             ty.wrapping_add((5 as libc::c_int *
                                                  3 as libc::c_int) as
                                                 libc::c_uint).wrapping_add(cur_y));
            }
            cur_y =
                (cur_y as libc::c_uint).wrapping_add(linePitch) as UDWORD as
                    UDWORD;
            if cur_y > height {
                /* run out of room - need to make new tab */
                *cur_seq = sequence;
                *cur_seqpage = i;
                return 0 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
        i = 0 as libc::c_int as UDWORD;
        sequence = sequence.wrapping_add(1)
    }
    return 1 as libc::c_int;
    /* done */
}
unsafe extern "C" fn intDisplaySeqTextView(mut psWidget: *mut _widget,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut psViewReplay: *mut VIEW_REPLAY =
        (*Form).pUserData as *mut VIEW_REPLAY;
    let mut cur_seq: size_t = 0;
    let mut cur_seqpage: size_t = 0;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut page: UDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    RenderWindowFrame(&mut FrameNormal, x0, y0, (*Form).width as UDWORD,
                      (*Form).height as UDWORD);
    /* work out where we're up to in the text */
    cur_seqpage = 0 as libc::c_int as size_t;
    cur_seq = cur_seqpage;
    page = 0 as libc::c_int as UDWORD;
    while page < (*Form).majorT as libc::c_uint {
        intDisplaySeqTextViewPage(psViewReplay, x0, y0,
                                  (*Form).width as UDWORD,
                                  (*Form).height as UDWORD, 0 as libc::c_int,
                                  &mut cur_seq, &mut cur_seqpage);
        page = page.wrapping_add(1)
    }
    intDisplaySeqTextViewPage(psViewReplay, x0, y0, (*Form).width as UDWORD,
                              (*Form).height as UDWORD, 1 as libc::c_int,
                              &mut cur_seq, &mut cur_seqpage);
}
// Add all the Video Sequences for a message ... works on PC &  PSX
#[no_mangle]
pub unsafe extern "C" fn StartMessageSequences(mut psMessage: *mut MESSAGE,
                                               mut Start: BOOL) {
    let mut bLoop: BOOL = 0 as libc::c_int;
    //		printf("start message sequence\n");		//[testing if we hit this] -Q
	//should never have a proximity message here
    if (*psMessage).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
        return
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"StartMessageSequences: invalid ViewData pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intelmap.c\x00" as *const u8 as *const libc::c_char,
              986 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"StartMessageSequences\x00")).as_ptr(),
              b"PTRVALID(psMessage->pViewData, sizeof(VIEWDATA))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as libc::c_uint ==
           VIEW_RPL as libc::c_int as libc::c_uint {
        let mut psViewReplay: *mut VIEW_REPLAY = 0 as *mut VIEW_REPLAY;
        let mut Sequence: UDWORD = 0;
        // Surely we don't need to set up psCurrentMsg when we pass the message into this routine ... tim
//		psViewReplay = (VIEW_REPLAY *)((VIEWDATA *)psCurrentMsg->pViewData)->pData;
        psViewReplay =
            (*((*psMessage).pViewData as *mut VIEWDATA)).pData as
                *mut VIEW_REPLAY;
        seq_ClearSeqList();
        //add any sequences to the list to be played when the first one is finished
        Sequence = 0 as libc::c_int as UDWORD;
        while Sequence < (*psViewReplay).numSeq as libc::c_uint {
            if (*(*psViewReplay).pSeqList.offset(Sequence as isize)).flag as
                   libc::c_int == 1 as libc::c_int {
                bLoop = 1 as libc::c_int
            } else { bLoop = 0 as libc::c_int }
            seq_AddSeqToList((*(*psViewReplay).pSeqList.offset(Sequence as
                                                                   isize)).sequenceName.as_mut_ptr(),
                             (*(*psViewReplay).pSeqList.offset(Sequence as
                                                                   isize)).pAudio,
                             0 as *mut STRING, bLoop, Sequence);
            //	{
//		STRING String[256];
//		sprintf(String,"seqadded %d of %d [%s]\n",Sequence,psViewReplay->numSeq,psViewReplay->pSeqList[Sequence].sequenceName);
//		prnt(1,String,0,0);
//	}
            debug(LOG_NEVER,
                  b"sequence=%d\n\x00" as *const u8 as *const libc::c_char,
                  Sequence);
            addVideoText(&mut *(*psViewReplay).pSeqList.offset(Sequence as
                                                                   isize),
                         Sequence);
            Sequence = Sequence.wrapping_add(1)
        }
        //play first full screen video
        if Start == 1 as libc::c_int { seq_StartNextFullScreenVideo(); }
    } else if (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as
                  libc::c_uint == VIEW_RES as libc::c_int as libc::c_uint {
        let mut psViewReplay_0: *mut VIEW_RESEARCH = 0 as *mut VIEW_RESEARCH;
        //UDWORD Sequence;
        psViewReplay_0 =
            (*((*psCurrentMsg).pViewData as *mut VIEWDATA)).pData as
                *mut VIEW_RESEARCH;
        seq_ClearSeqList();
        seq_AddSeqToList((*psViewReplay_0).sequenceName.as_mut_ptr(),
                         (*psViewReplay_0).pAudio, 0 as *mut STRING,
                         0 as libc::c_int, 0 as libc::c_int as UDWORD);
        //play first full screen video
        if Start == 1 as libc::c_int { seq_StartNextFullScreenVideo(); }
    };
}
#[no_mangle]
pub static mut ButtonPresses: UDWORD = 0 as libc::c_int as UDWORD;
/*
deal with the actual button press - proxMsg is set to true if a proximity
button has been pressed
*/
#[no_mangle]
pub unsafe extern "C" fn _intIntelButtonPressed(mut proxMsg: BOOL,
                                                mut id: UDWORD) {
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE; //, i;
    let mut currID: UDWORD = 0;
    //	char aAudioName[MAX_STR_LENGTH];	// made static to reduce stack usage.
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    if proxMsg != 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"intIntelButtonPressed: Shouldn\'t be able to get a proximity message!\x00"
                  as *const u8 as *const libc::c_char);
    };
    if proxMsg != 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intelmap.c\x00" as *const u8 as *const libc::c_char,
              1075 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"_intIntelButtonPressed\x00")).as_ptr(),
              b"proxMsg != TRUE\x00" as *const u8 as *const libc::c_char);
    };
    /* message button has been pressed - clear the old button and messageView*/
    if messageID != 0 as libc::c_int as libc::c_uint {
        widgSetButtonState(psWScreen, messageID, 0 as libc::c_int as UDWORD);
        intRemoveMessageView(0 as libc::c_int);
        psCurrentMsg = 0 as *mut MESSAGE
        //initTextDisplay(psCurrentMsg, WFont, 255);
    }
    /* Lock the new button */
	// This means we can't click on the same movie button twice.
//	widgSetButtonState(psWScreen, id, WBUT_LOCK);
    widgSetButtonState(psWScreen, id, 0x4 as libc::c_int as UDWORD);
    messageID = id;
    //Find the message for the new button */
    currID = 6100 as libc::c_int as UDWORD;
    psMessage = apsMessages[selectedPlayer as usize];
    while !psMessage.is_null() {
        if (*psMessage).type_0 as libc::c_uint !=
               MSG_PROXIMITY as libc::c_int as libc::c_uint {
            if currID == id { break ; }
            currID = currID.wrapping_add(1)
        }
        psMessage = (*psMessage).psNext
    }
    //deal with the message if one
    if !psMessage.is_null() {
        //set the current message
        psCurrentMsg = psMessage;
        //initTextDisplay(psCurrentMsg, WFont, 255);
        //set the read flag
        (*psCurrentMsg).read = 1 as libc::c_int;
        //this is for the deaf! - done in intDisplayMessageView()
		/*if (psMessage->pViewData->pTextMsg)
		{
			addGameMessage(psMessage->pViewData->pTextMsg, INTEL_TXT_LIFE,
				TRUE);
		}*/
        //DBPRINTF(("Dealing with a new message !!! type=%d\n",psMessage->pViewData->type);
		//should never have a proximity message
        if (*psMessage).type_0 as libc::c_uint ==
               MSG_PROXIMITY as libc::c_int as libc::c_uint {
            return
        }
        // If its a video sequence then play it anyway
        if (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as libc::c_uint
               == VIEW_RPL as libc::c_int as libc::c_uint {
            if !(*psMessage).pViewData.is_null() {
                intAddMessageView(psMessage);
            }
            StartMessageSequences(psMessage, 1 as libc::c_int);
        } else if (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as
                      libc::c_uint == VIEW_RES as libc::c_int as libc::c_uint
         {
            //this must be for the blind
			//with forsight this information was removed from the meassage text
/*
			if (((VIEW_RESEARCH *)((VIEWDATA *)psMessage->pViewData)->pData)->pAudio != NULL)
			{
				ASSERT( strlen(((VIEW_RESEARCH *)((VIEWDATA *)psMessage->pViewData)->
					pData)->pAudio)<244,"sequence path+name greater than max string" );
				strcpy(aAudioName,"sequenceAudio\\");
				strcat(aAudioName,((VIEW_RESEARCH *)((VIEWDATA *)psMessage->
					pViewData)->pData)->pAudio);

				audio_PlayStream(aAudioName, AUDIO_VOL_MAX, NULL);
			}
*/
			//This hack replaces it
            psResearch =
                getResearchForMsg((*psMessage).pViewData as *mut VIEWDATA);
            if !psResearch.is_null() {
                match (*psResearch).iconID as libc::c_int {
                    256 => {
                        audio_PlayStream(b"sequenceAudio\\Res_Droid.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                        //				default:
                    }
                    257 => {
                        audio_PlayStream(b"sequenceAudio\\Res_Weapons.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    258 => {
                        audio_PlayStream(b"sequenceAudio\\Res_com.wav\x00" as
                                             *const u8 as *const libc::c_char
                                             as *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    259 => {
                        audio_PlayStream(b"sequenceAudio\\Res_Power.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    260 => {
                        audio_PlayStream(b"sequenceAudio\\Res_SysTech.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    261 => {
                        audio_PlayStream(b"sequenceAudio\\Res_StruTech.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    349 => {
                        audio_PlayStream(b"sequenceAudio\\Res_Droid.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    395 => {
                        audio_PlayStream(b"sequenceAudio\\Res_StruTech.wav\x00"
                                             as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         100 as libc::c_long as SDWORD, None);
                    }
                    _ => { }
                }
            }
            //and finally for the dumb?
            if !(*psMessage).pViewData.is_null() {
                intAddMessageView(psMessage);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn intCleanUpIntelMap() {
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut psNext: *mut MESSAGE = 0 as *mut MESSAGE;
    //remove any research messages that have been read
    psMessage = apsMessages[selectedPlayer as usize];
    while !psMessage.is_null() {
        psNext = (*psMessage).psNext;
        if (*psMessage).type_0 as libc::c_uint ==
               MSG_RESEARCH as libc::c_int as libc::c_uint &&
               (*psMessage).read != 0 {
            removeMessage(psMessage, selectedPlayer);
        }
        psMessage = psNext
    }
    resetIntelligencePauseState();
    immediateMessage = 0 as libc::c_int;
    cdAudio_Resume();
    // FIXME: NOT SURE IT'S CORRECT. this makes the transports come.
    eventFireCallbackTrigger(CALL_VIDEO_QUIT as libc::c_int as TRIGGER_TYPE);
}
/* Process return code from the Message View for Tutorial Mode*/
//extern void intProcessMessageView(UDWORD id);
/* rotate the view so looking directly down if forward = TRUE or
 back to previous view if forward = FALSE */
//extern void intelMapView(BOOL forward);
/* Remove the Intelligence Map widgets from the screen */
/* Remove the Intelligence Map widgets from the screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveIntelMap() {
    //UDWORD buttonID;
    let mut Widg: *mut WIDGET = 0 as *mut WIDGET;
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    //MESSAGE		*psMessage, *psNext;
    //remove each proximity button
	/*for (buttonID = 0; buttonID < numProxMsg; buttonID++)
	{
		widgDelete(psWScreen, IDINTMAP_PROXSTART + buttonID);
	}*/
	//remove 3dView if still there
    Widg = widgGetFromID(psWScreen, 6002 as libc::c_int as UDWORD);
    if !Widg.is_null() { intRemoveMessageView(0 as libc::c_int); }
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 6000 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    (*Form).display =
        Some(intClosePlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    (*Form).disableChildren = 1 as libc::c_int;
    (*Form).pUserData = 0 as *mut libc::c_void;
    ClosingIntelMap = 1 as libc::c_int;
    //remove the text label
    widgDelete(psWScreen, 6005 as libc::c_int as UDWORD);
    intCleanUpIntelMap();
    //	//remove any research messages that have been read
//	for (psMessage = apsMessages[selectedPlayer]; psMessage != NULL; psMessage =
//		psNext)
//	{
//		psNext = psMessage->psNext;
//		if (psMessage->type == MSG_RESEARCH AND psMessage->read)
//		{
//			removeMessage(psMessage, selectedPlayer);
//		}
//	}
//	resetIntelligencePauseState();
//
//	immediateMessage = FALSE;
}
/* Remove the Intelligence Map widgets from the screen without animation*/
/* Remove the Intelligence Map widgets from the screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveIntelMapNoAnim() {
    //UDWORD buttonID;
    let mut Widg: *mut WIDGET = 0 as *mut WIDGET;
    //remove each proximity button
	/*for (buttonID = 0; buttonID < numProxMsg; buttonID++)
	{
		widgDelete(psWScreen, IDINTMAP_PROXSTART + buttonID);
	}*/
	//remove 3dView if still there
    Widg = widgGetFromID(psWScreen, 6002 as libc::c_int as UDWORD);
    if !Widg.is_null() { intRemoveMessageView(0 as libc::c_int); }
    //remove main Intelligence screen
    widgDelete(psWScreen, 6000 as libc::c_int as UDWORD);
    //remove the text label
    widgDelete(psWScreen, 6005 as libc::c_int as UDWORD);
    intCleanUpIntelMap();
    //	resetIntelligencePauseState();
//
//	immediateMessage = FALSE;
}
/* Remove the Message View from the Intelligence screen */
/* Remove the Message View from the Intelligence screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveMessageView(mut animated: BOOL) {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut psViewResearch: *mut VIEW_RESEARCH = 0 as *mut VIEW_RESEARCH;
    //remove 3dView if still there
    Form =
        widgGetFromID(psWScreen, 6002 as libc::c_int as UDWORD) as
            *mut W_TABFORM;
    if !Form.is_null() {
        //stop the video
        psViewResearch = (*Form).pUserData as *mut VIEW_RESEARCH;
        seq_RenderVideoToBuffer(0 as *mut iSurface,
                                (*psViewResearch).sequenceName.as_mut_ptr(),
                                gameTime2 as libc::c_int, 3 as libc::c_int);
        if animated != 0 {
            widgDelete(psWScreen, 6004 as libc::c_int as UDWORD);
            // Start the window close animation.
            (*Form).display =
                Some(intClosePlainForm as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             ->
                                 ()); // Used to signal when the close anim has finished.
            (*Form).disableChildren = 1 as libc::c_int;
            (*Form).pUserData = 0 as *mut libc::c_void;
            ClosingMessageView = 1 as libc::c_int
        } else {
            //remove without the animating close window
            widgDelete(psWScreen, 6002 as libc::c_int as UDWORD);
        }
    };
}
/*Displays the buttons used on the intelligence map */
/*Displays the buttons used on the intelligence map */
unsafe extern "C" fn intDisplayMessageButton(mut psWidget: *mut _widget,
                                             mut xOffset: UDWORD,
                                             mut yOffset: UDWORD,
                                             mut pColours: *mut UDWORD) {
    let mut psButton: *mut W_CLICKFORM = psWidget as *mut W_CLICKFORM;
    let mut psBuffer: *mut RENDERED_BUTTON =
        (*psButton).pUserData as *mut RENDERED_BUTTON;
    let mut psMsg: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut Hilight: BOOL = 0 as libc::c_int;
    let mut Down: UDWORD = 0 as libc::c_int as UDWORD;
    let mut IMDType: UDWORD = 0 as libc::c_int as UDWORD;
    let mut compID: UDWORD = 0;
    let mut image: SDWORD = -(1 as libc::c_int);
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut psResGraphic: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut MovieButton: BOOL = 0 as libc::c_int;
    OpenButtonRender(xOffset.wrapping_add((*psButton).x as libc::c_uint) as
                         UWORD,
                     yOffset.wrapping_add((*psButton).y as libc::c_uint) as
                         UWORD, (*psButton).width, (*psButton).height);
    Down =
        (*psButton).state &
            (0x1 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint;
    Hilight =
        ((*psButton).state & 0x4 as libc::c_int as libc::c_uint) as BOOL;
    // Get the object associated with this widget.
    psMsg = (*psBuffer).Data as *mut MESSAGE;
    if !psMsg.is_null() {
    } else {
        debug(LOG_ERROR,
              b"psBuffer->Data empty. Why?\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psMsg.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"intelmap.c\x00" as *const u8 as *const libc::c_char,
              1381 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"intDisplayMessageButton\x00")).as_ptr(),
              b"psMsg != NULL\x00" as *const u8 as *const libc::c_char);
    };
    //shouldn't have any proximity messages here...
    if (*psMsg).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
        return
    }
    //set the graphic for the button
    match (*psMsg).type_0 as libc::c_uint {
        0 => {
            pResearch =
                getResearchForMsg((*psMsg).pViewData as *mut VIEWDATA);
            //IMDType = IMDTYPE_RESEARCH;
        	//set the IMDType depending on what stat is associated with the research
            if !(*pResearch).psStat.is_null() {
                //we have a Stat associated with this research topic
                if StatIsStructure((*pResearch).psStat) != 0 {
                    //this defines how the button is drawn
                    IMDType = IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD;
                    psResGraphic = (*pResearch).psStat
                } else {
                    compID = StatIsComponent((*pResearch).psStat) as UDWORD;
                    if compID != COMP_UNKNOWN as libc::c_int as libc::c_uint {
                        //this defines how the button is drawn
                        IMDType = IMDTYPE_COMPONENT as libc::c_int as UDWORD;
                        psResGraphic = (*pResearch).psStat
                    } else {
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"intDisplayMessageButton: invalid stat\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"intelmap.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1414 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 24],
                                                            &[libc::c_char; 24]>(b"intDisplayMessageButton\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                        IMDType = IMDTYPE_RESEARCH as libc::c_int as UDWORD;
                        psResGraphic = pResearch as *mut BASE_STATS
                    }
                }
            } else {
                //no Stat for this research topic so use the research topic to define what is drawn
                psResGraphic = pResearch as *mut BASE_STATS;
                IMDType = IMDTYPE_RESEARCH as libc::c_int as UDWORD
            }
        }
        1 => {
            image = IMAGE_INTEL_CAMPAIGN as libc::c_int;
            MovieButton = 1 as libc::c_int
        }
        2 => {
            image = IMAGE_INTEL_MISSION as libc::c_int;
            MovieButton = 1 as libc::c_int
        }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown message type: %i\x00" as *const u8 as
                      *const libc::c_char, (*psMsg).type_0 as libc::c_uint);
            return
        }
    }
    //if research message
    if !pResearch.is_null() {
        if (*pResearch).iconID as libc::c_int != 0 as libc::c_int {
            image = (*pResearch).iconID as SDWORD
        }
        //do we have the same icon for the top right hand corner?
        if image > 0 as libc::c_int {
            //RenderToButton(IntImages,(UWORD)image,pResearch,selectedPlayer,psBuffer,Down,
			//				IMDType,TOPBUTTON);											// ajl, changed from 0 to selectedPLayer
            RenderToButton(IntImages, image as UWORD,
                           psResGraphic as *mut libc::c_void, selectedPlayer,
                           psBuffer, Down as BOOL, IMDType,
                           0 as libc::c_int as UDWORD);
        } else {
            RenderToButton(0 as *mut IMAGEFILE, 0 as libc::c_int as UWORD,
                           pResearch as *mut libc::c_void, selectedPlayer,
                           psBuffer, Down as BOOL, IMDType,
                           0 as libc::c_int as UDWORD);
            //ajl, changed from 0 to selectedPlayer
        }
    } else if image > 0 as libc::c_int {
        if MovieButton != 0 {
            //draw buttons for mission and general messages
            // draw the button with the relevant image, don't add Down to the image ID if it's
				// a movie button.
            RenderImageToButton(IntImages, image as UWORD, psBuffer,
                                Down as BOOL, 0 as libc::c_int as UDWORD);
        } else {
            //draw the button with the relevant image
            RenderImageToButton(IntImages,
                                (image as libc::c_uint).wrapping_add(Down) as
                                    UWORD, psBuffer, Down as BOOL,
                                0 as libc::c_int as UDWORD);
        }
    }
    CloseButtonRender();
    if Hilight != 0 {
        pie_ImageFileID(IntImages, IMAGE_BUT_HILITE as libc::c_int as UWORD,
                        xOffset.wrapping_add((*psButton).x as libc::c_uint) as
                            libc::c_int,
                        yOffset.wrapping_add((*psButton).y as libc::c_uint) as
                            libc::c_int);
    };
}
/*this sets the width and height for the Intel map surface so that it fill the
appropriate sized image for the view*/
//static void setIntelBufferSize(UDWORD type);
/*sets the intel map surface back to the size it was created with */
//static void resetIntelBufferSize(void);
//static BOOL checkMessageOverlap(MESSAGE *psMessage, SWORD x, SWORD y);
/* draws the text message in the message window - only allows for one at the moment!*/
//static void displayIntelligenceMessage(MESSAGE *psMessage);
/* Remove the Message View from the Intelligence screen without animation*/
//static void intRemoveMessageViewNoAnim(BOOL animated);
/* displays the PIE view for the current message */
unsafe extern "C" fn intDisplayPIEView(mut psWidget: *mut _widget,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut psMessage: *mut MESSAGE = (*Form).pUserData as *mut MESSAGE;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut psViewResearch: *mut VIEW_RESEARCH = 0 as *mut VIEW_RESEARCH;
    let mut image: SWORD = -(1 as libc::c_int) as SWORD;
    //#ifndef PSX
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    //#endif
    //shouldn't have any proximity messages here...
    if (*psMessage).type_0 as libc::c_uint ==
           MSG_PROXIMITY as libc::c_int as libc::c_uint {
        return
    }
    if !psMessage.is_null() && !(*psMessage).pViewData.is_null() {
        x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
        y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
        x1 = x0.wrapping_add((*Form).width as libc::c_uint);
        y1 = y0.wrapping_add((*Form).height as libc::c_uint);
        //moved from after close render
        RenderWindowFrame(&mut FrameNormal,
                          x0.wrapping_sub(1 as libc::c_int as libc::c_uint),
                          y0.wrapping_sub(1 as libc::c_int as libc::c_uint),
                          x1.wrapping_sub(x0).wrapping_add(2 as libc::c_int as
                                                               libc::c_uint),
                          y1.wrapping_sub(y0).wrapping_add(2 as libc::c_int as
                                                               libc::c_uint));
        OpenButtonRender(xOffset.wrapping_add((*Form).x as libc::c_uint) as
                             UWORD,
                         yOffset.wrapping_add((*Form).y as libc::c_uint) as
                             UWORD, (*Form).width, (*Form).height);
        //OpenButtonRender(Form->x, Form->y,Form->width, Form->height);
        if (*((*psMessage).pViewData as *mut VIEWDATA)).type_0 as libc::c_uint
               != VIEW_RES as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"intDisplayPIEView: Invalid message type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intelmap.c\x00" as *const u8 as *const libc::c_char,
                      1528 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"intDisplayPIEView\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return
        }
        //render an object
        psViewResearch =
            (*((*psCurrentMsg).pViewData as *mut VIEWDATA)).pData as
                *mut VIEW_RESEARCH;
        psResearch =
            getResearchForMsg((*psCurrentMsg).pViewData as *mut VIEWDATA);
        renderResearchToBuffer(pIntelMapSurface, psResearch,
                               x0.wrapping_add(x1.wrapping_sub(x0).wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)),
                               y0.wrapping_add(y1.wrapping_sub(y0).wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)));
        CloseButtonRender();
        //draw image icon in top left of window
        image =
            (*getResearchForMsg((*psMessage).pViewData as
                                    *mut VIEWDATA)).iconID as SWORD;
        if image as libc::c_int > 0 as libc::c_int {
            pie_ImageFileID(IntImages, image as UWORD, x0 as libc::c_int,
                            y0 as libc::c_int);
        }
    };
}
/* displays the TEXT view for the current message */
unsafe extern "C" fn intDisplayTEXTView(mut psWidget: *mut _widget,
                                        mut xOffset: UDWORD,
                                        mut yOffset: UDWORD,
                                        mut pColours: *mut UDWORD) {
    let mut Form: *mut W_TABFORM = psWidget as *mut W_TABFORM;
    let mut psMessage: *mut MESSAGE = (*Form).pUserData as *mut MESSAGE;
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut linePitch: UDWORD = 0;
    let mut ty: UDWORD = 0;
    x0 = xOffset.wrapping_add((*Form).x as libc::c_uint);
    y0 = yOffset.wrapping_add((*Form).y as libc::c_uint);
    x1 = x0.wrapping_add((*Form).width as libc::c_uint);
    y1 = y0.wrapping_add((*Form).height as libc::c_uint);
    ty = y0;
    RenderWindowFrame(&mut FrameNormal, x0, y0, x1.wrapping_sub(x0),
                      y1.wrapping_sub(y0));
    if !psMessage.is_null() {
        iV_SetFont(WFont);
        /* Get the travel to the next line */
        linePitch = iV_GetTextLineSize() as UDWORD;
        /* Fix for spacing.... */
        linePitch =
            (linePitch as
                 libc::c_uint).wrapping_add(6 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        ty =
            (ty as
                 libc::c_uint).wrapping_add(3 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        /* Fix for spacing.... */
        iV_SetTextColour(pal_GetNearestColour(255 as libc::c_int as uint8,
                                              255 as libc::c_int as uint8,
                                              255 as libc::c_int as uint8) as
                             SWORD);
        //add each message
        i = 0 as libc::c_int as UDWORD;
        while i <
                  (*((*psMessage).pViewData as *mut VIEWDATA)).numText as
                      libc::c_uint {
            //displayIntelligenceMessage(psMessage);
			//check haven't run out of room first!
            if i.wrapping_mul(linePitch) > (*Form).height as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intDisplayTEXTView: Run out of room!\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"intelmap.c\x00" as *const u8 as
                              *const libc::c_char, 1649 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"intDisplayTEXTView\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return
            }
            //need to check the string will fit!
            pie_DrawText(*(*((*psMessage).pViewData as
                                 *mut VIEWDATA)).ppTextMsg.offset(i as isize),
                         x0.wrapping_add(5 as libc::c_int as libc::c_uint),
                         ty.wrapping_add((5 as libc::c_int * 3 as libc::c_int)
                                             as
                                             libc::c_uint).wrapping_add(i.wrapping_mul(linePitch)));
            i = i.wrapping_add(1)
        }
    };
}
//adds text to full screen video
unsafe extern "C" fn addVideoText(mut psSeqDisplay: *mut SEQ_DISPLAY,
                                  mut sequence: UDWORD) {
    let mut i: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    if (*psSeqDisplay).numText as libc::c_int > 0 as libc::c_int {
        debug(LOG_NEVER,
              b"avt seq=%d [%s]\n\x00" as *const u8 as *const libc::c_char,
              sequence,
              *(*psSeqDisplay).ppTextMsg.offset(0 as libc::c_int as isize));
        //add each message, first at the top
        x = 20 as libc::c_int as UDWORD; //startframe endFrame
        y = 20 as libc::c_int as UDWORD;
        seq_AddTextForVideo(*(*psSeqDisplay).ppTextMsg.offset(0 as libc::c_int
                                                                  as isize),
                            x as SDWORD, y as SDWORD, 0 as libc::c_int,
                            9999 as libc::c_int, 0 as libc::c_int, sequence);
        //add each message, the rest at the bottom
        x = 20 as libc::c_int as UDWORD; //startframe endFrame
        y = 444 as libc::c_int as UDWORD;
        i = 1 as libc::c_int as UDWORD;
        while i < (*psSeqDisplay).numText as libc::c_uint {
            seq_AddTextForVideo(*(*psSeqDisplay).ppTextMsg.offset(i as isize),
                                x as SDWORD, y as SDWORD, 0 as libc::c_int,
                                9999 as libc::c_int, 0 as libc::c_int,
                                sequence);
            //initialise after the first setting
            y = 0 as libc::c_int as UDWORD;
            x = y;
            i = i.wrapping_add(1)
        }
    };
}
//initialise the text display stats for the current message
//extern void initTextDisplay(MESSAGE *psMessage, UDWORD fontID, UWORD fontColour);
/* scroll the text message from left to right - aka tickertape messages */
//extern void scrollMessage(STRING *pText, UDWORD startX, UDWORD endX, UDWORD y, UDWORD gap);
/*sets psCurrentMsg for the Intelligence screen*/
/*rotate the view so looking directly down if forward = TRUE or
 back to previous view if forward = FALSE */
/*void intelMapView(BOOL forward)
{
	if (forward)
	{
		//save the current viewing angle
		viewAngle = player.r.x;
		viewHeight = player.p.y;
		//rotate to top down view
		player.r.x = DEG(-90);
#ifndef PSX
		player.p.y = INTELMAP_VIEWHEIGHT;
#else
		camera.p.y = INTELMAP_VIEWHEIGHT;
#endif
	}
	else
	{
		//rotate back to previous view angle
		player.r.x = viewAngle;
		player.p.y = viewHeight;
	}
}*/
/*this sets the width and height for the Intel map surface so that it fill the
appropriate sized image for the view*/
/*void setIntelBufferSize(UDWORD type)
{
	switch (type)
	{
	case MSG_CAMPAIGN:
	case MSG_RESEARCH:
		pIntelMapSurface->width = INTMAP_CAMPAIGNWIDTH;
		pIntelMapSurface->height = INTMAP_CAMPAIGNHEIGHT;
		break;
	case MSG_MISSION:
	//case MSG_TUTORIAL:
		pIntelMapSurface->width = INTMAP_MISSIONWIDTH;
		pIntelMapSurface->height = INTMAP_MISSIONHEIGHT;
		break;
	case MSG_PROXIMITY:
		pIntelMapSurface->width = INTMAP_PROXIMITYWIDTH;
		pIntelMapSurface->height = INTMAP_PROXIMITYHEIGHT;
		break;
	default:
		ASSERT( FALSE, "Invalid message type" );
	}
}
*/
/*sets the intel map surface back to the size it was created with */
/*void resetIntelBufferSize(void)
{
	pIntelMapSurface->width = MSG_BUFFER_WIDTH;
	pIntelMapSurface->height = MSG_BUFFER_HEIGHT;
}*/
/* draws the text message in the message window - only allows for one at the moment!*/
/*void displayIntelligenceMessage(MESSAGE *psMessage)
{
	UDWORD	x1, x2, y, indent = 10;


	x1 = INTMAP_TEXTX;
	x2 = INTMAP_TEXTX + INTMAP_TEXTWIDTH;
	y = INTMAP_TEXTY + INTMAP_TEXTHEIGHT + indent;

	//size and position depends on the type of message
	//switch (psMessage->type)
	{
	case MSG_RESEARCH:
		x1 = INTMAP_RESEARCHX;
		x2 = INTMAP_RESEARCHX + INTMAP_RESEARCHWIDTH;
		y = INTMAP_RESEARCHY + INTMAP_RESEARCHHEIGHT + indent;
		break;
	case MSG_MISSION:
      case MSG_CAMPAIGN:
	//case MSG_TUTORIAL:
		x1 = INTMAP_MISSIONX;
		x2 = INTMAP_MISSIONX + INTMAP_MISSIONWIDTH;
		y = INTMAP_MISSIONY + INTMAP_MISSIONHEIGHT + indent;// + INTMAP_TEXTWINDOWHEIGHT;// - indent;
		break;
	case MSG_PROXIMITY:
		x1 = INTMAP_PROXIMITYX;
		x2 = INTMAP_PROXIMITYX + INTMAP_PROXIMITYWIDTH;
		y = INTMAP_PROXIMITYY + INTMAP_PROXIMITYHEIGHT + indent;// + INTMAP_TEXTWINDOWHEIGHT;// - indent;
		break;
	default:
		ASSERT( FALSE, "Unknown message type" );
	}

#ifdef PSX
	DisplayControlDiag();
#endif

//#ifdef PSX
//	iV_DrawText(psMessage->pViewData->pTextMsg,x1,y);
//#else
//	screenSetTextColour(255,255,255);
//	screenTextOut(x+1,y+1,psMessage->pViewData->pTextMsg);
	//scrollMessage(psMessage->pViewData->pTextMsg, x2, x1, y, 5);

#ifdef PSX
	setConsoleSizePos(x1+2,y-32-iV_GetTextLineSize()*2,(x2-x1)-4);
#else
	setConsoleSizePos(x1, y, (x2-x1));
#endif
	setConsolePermanence(TRUE);
	addConsoleMessage(psMessage->pViewData->pTextMsg, LEFT_JUSTIFY);
}
*/
/*sets psCurrentMsg for the Intelligence screen*/
#[no_mangle]
pub unsafe extern "C" fn setCurrentMsg() {
    let mut psMsg: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut psLastMsg: *mut MESSAGE = 0 as *mut MESSAGE;
    psLastMsg = 0 as *mut MESSAGE;
    psMsg = apsMessages[selectedPlayer as usize];
    while !psMsg.is_null() {
        if (*psMsg).type_0 as libc::c_uint !=
               MSG_PROXIMITY as libc::c_int as libc::c_uint {
            psLastMsg = psMsg
        }
        psMsg = (*psMsg).psNext
    }
    psCurrentMsg = psLastMsg;
}
/*sets which states need to be paused when the intelligence screen is up*/
//initialise the text display stats for the current message
/*void initTextDisplay(MESSAGE *psMessage, UDWORD fontID, UWORD fontColour)
{
	UDWORD		width, currentLength, strLen, frames, inc;

	currentTextDisplay.font = fontID;
	currentTextDisplay.fontColour = fontColour;
	currentTextDisplay.startTime = 0;
	currentTextDisplay.text[0] = '\0';

	if (psMessage == NULL OR psMessage->pViewData->pTextMsg == NULL)
	{
		currentTextDisplay.totalFrames = 0;
		return;
	}

	//size of the text window depends on the message type
	switch (psMessage->type)
	{
	//case MSG_CAMPAIGN:
	case MSG_RESEARCH:
		width = INTMAP_RESEARCHWIDTH;
		break;
	//case MSG_MISSION:
	//case MSG_TUTORIAL:
	//	width = INTMAP_MISSIONWIDTH;
	//	break;
	//case MSG_PROXIMITY:
	//	width = INTMAP_PROXIMITYWIDTH;
	//	break;
	default:
		ASSERT( FALSE, "Unknown message type" );
		return;
	}

	currentLength = 0;
	strLen = strlen(psMessage->pViewData->pTextMsg);
	frames = 0;
	currentTextDisplay.totalFrames = 0;

	//get the length of the text message
	for (inc=0; inc < strLen; inc++)
	{
		currentLength += iV_GetCharWidth(psMessage->pViewData->pTextMsg[inc]);
	}

	width += currentLength;

	currentLength = 0;
	//how long for string to completely pass along width of view
	for (inc=0; inc < MAX_STR_LENGTH; inc++)
	{
		if (inc < strLen)
		{
			currentLength += iV_GetCharWidth(psMessage->pViewData->pTextMsg[inc]);
		}
		else
		{
			currentLength += iV_GetCharWidth(' ');
		}

		if (currentLength > width)
		{
			break;
		}
		currentTextDisplay.totalFrames++;
	}
	currentTextDisplay.startTime = gameTime2;
}*/
/* scroll the text message from right to left - aka tickertape messages */
/*void scrollMessage(STRING *pText, UDWORD startX, UDWORD endX, UDWORD y, UDWORD gap)
{
	UDWORD			frames, inc, strLen;
	SDWORD			position, startChar, currentLength;
	UDWORD			endChar, text;

	//work out current frame
	frames = 20 * (gameTime2 - currentTextDisplay.startTime)/GAME_TICKS_PER_SEC;

	//get the number of chars in the string
	strLen = strlen(pText);

	currentLength = 0;

	//work out position of the string
	for (inc = 0; inc < frames; inc++)
	{
		//framesMinus1 = frames - 1;
		if ((frames - 1 - inc) > (strLen-1))
		{
			//add a blank for 'characters' at the end of the sentence
			currentLength += iV_GetCharWidth(' ');
			//nothing to draw so go to next inc
			continue;
		}

		//increment the current amount drawn
		currentLength += iV_GetCharWidth(pText[frames - 1 - inc]);

		if (((SDWORD)startX - currentLength) < (SDWORD)endX)
		{
			//ignore this character since off the scale
			inc--;
			break;
		}
		position = startX - currentLength;
	}
	startChar = frames - 1 - inc;
	if (startChar < 0)
	{
		startChar = 0;
	}
	endChar = frames;
	if (endChar > strLen)
	{
		endChar = strLen;
	}
	text = 0;
	for (inc = startChar; inc != endChar AND inc < strLen; inc++)
	{
		currentTextDisplay.text[text++] = pText[inc];
	}
	currentTextDisplay.text[text] = '\0';
	iV_SetFont(currentTextDisplay.font);
	iV_SetTextColour(currentTextDisplay.fontColour);
	iV_DrawText(currentTextDisplay.text, position, y);

	//time to redo message
	if (frames > currentTextDisplay.totalFrames + gap)
	{
		//start again
		currentTextDisplay.startTime = gameTime2;
	}
}*/
/* Process return code from the Message View for Tutorial Mode*/
/*void intProcessMessageView(UDWORD id)
{
	if (id == IDINTMAP_CLOSE)
	{
		//if close button pressed on 3D View then close the view only
		psCurrentMsg = NULL;
		initTextDisplay(psCurrentMsg, WFont, 255);
		//intRemoveMessageView();
		intRemoveMessageViewNoAnim();
		intResetScreen(TRUE);
	}
}*/
/* Add the Proximity message buttons */
/*BOOL intAddProximityButton(MESSAGE *pMessage, UDWORD inc)
{
	W_FORMINIT		sBFormInit;
	VIEW_LOCATION	*pViewLocation = (VIEW_LOCATION*)pMessage->pViewData->pData;

	memset(&sBFormInit, 0, sizeof(W_FORMINIT));
	sBFormInit.formID = 0;
	sBFormInit.id = IDINTMAP_PROXSTART + inc;
	ASSERT( sBFormInit.id < IDINTMAP_PROXEND,"Too many message buttons" );
	sBFormInit.majorID = 0;
	sBFormInit.minorID = 0;
	sBFormInit.style = WFORM_CLICKABLE;
	//width and height is dependant on the state of the message - see intDisplayProximityButton
	//the x and y need to be set up each time the button is drawn - see intDisplayProximityButton

	sBFormInit.pDisplay = intDisplayProximityButton;
	//set the data for this button
	sBFormInit.pUserData = pMessage;

	if (!widgAddForm(psWScreen, &sBFormInit))
	{
		return FALSE;
	}
	return TRUE;
}*/
/*Displays the proximity messages used on the intelligence map */
/*void intDisplayProximityButton(struct _widget *psWidget, UDWORD xOffset,
							  UDWORD yOffset, UDWORD *pColours)
{
	W_CLICKFORM			*psButton = (W_CLICKFORM*)psWidget;
	MESSAGE				*psMsg = (MESSAGE*)psButton->pUserData;
	PROXIMITY_DISPLAY	*psProximityDisplay;
	BOOL				Hilight = FALSE;
	VIEW_LOCATION		*psViewLocation;
	UBYTE				imageID;
	UDWORD				delay = 100;

	(void)pColours;
	(void)xOffset;
	(void)yOffset;

	// Get the object associated with this widget.
	psMsg = (MESSAGE *)psButton->pUserData;
	ASSERT( psMsg->type == MSG_PROXIMITY, "Invalid message type" );

	psViewLocation = (VIEW_LOCATION *)psMsg->pViewData->pData;

	//if not within view ignore message
	if (!clipXY(psViewLocation->location.x,	psViewLocation->location.z))
	{
		return;
	}

	psProximityDisplay = getProximityDisplay(psMsg);
	psButton->x = (SWORD)psProximityDisplay->screenX;
	psButton->y = (SWORD)psProximityDisplay->screenY;

	//get the screen coords for the message - check not 'off' the screen
	if (psButton->x < 0 OR psButton->x > DISP_WIDTH OR psButton->y < 0 OR
		psButton->y > DISP_HEIGHT)
	{
		return;
	}

	Hilight = psButton->state & WBUTS_HILITE;

	//if hilighted
	if (Hilight)
	{
		imageID = IMAGE_INTEL_PROXHILI;
		psButton->width = iV_GetImageWidth(IntImages,IMAGE_INTEL_PROXHILI);
		psButton->height = iV_GetImageHeight(IntImages,IMAGE_INTEL_PROXHILI);
	}
	else if (psMsg->read)
	{
		//if the message is read - don't animate
		imageID = IMAGE_INTEL_PROXREAD;
		psButton->width = iV_GetImageWidth(IntImages,IMAGE_INTEL_PROXREAD);
		psButton->height = iV_GetImageHeight(IntImages,IMAGE_INTEL_PROXREAD);
	}
	else
	{
		//draw animated
		if ((GetTickCount() - psProximityDisplay->timeLastDrawn) > delay)
		{
			psProximityDisplay->strobe++;
			if (psProximityDisplay->strobe > 2)
			{
				psProximityDisplay->strobe = 0;
			}
			psProximityDisplay->timeLastDrawn = GetTickCount();
		}
		imageID = (UBYTE)(IMAGE_INTEL_PROXIMITY + psProximityDisplay->strobe);
		psButton->width = iV_GetImageWidth(IntImages,IMAGE_INTEL_PROXIMITY);
		psButton->height = iV_GetImageHeight(IntImages,IMAGE_INTEL_PROXIMITY);
	}
	//adjust button x and y for width and height of button
	psButton->x = (SWORD)(psButton->x - psButton->width/(UWORD)2);
	psButton->y = (SWORD)(psButton->y - psButton->height/(UWORD)2);
	if (psButton->x < 0 OR psButton->x > DISP_WIDTH OR psButton->y < 0 OR
		psButton->y > DISP_HEIGHT)
	{
		return;
	}

	//if there is a message 3Dview up - don't draw the proximity messages underneath
	if (psCurrentMsg AND psCurrentMsg->pViewData)
	{
		if (!checkMessageOverlap(psCurrentMsg, psButton->x, psButton->y))
		{
			return;
		}
	}

	//draw the 'button'
	iV_DrawTransImage(IntImages,imageID, psButton->x, psButton->y);
}*/
/*check the x and y are within the messages 3D view if on screen */
/*BOOL checkMessageOverlap(MESSAGE *psMessage, SWORD x, SWORD y)
{
	SWORD		messageX, messageY, messageWidth, messageHeight;

	switch (psMessage->type)
	{
	case MSG_CAMPAIGN:
	case MSG_RESEARCH:
		messageX = INTMAP_CAMPAIGNX;
		messageY = INTMAP_CAMPAIGNY;
		messageWidth = INTMAP_CAMPAIGNWIDTH;
		messageHeight = INTMAP_CAMPAIGNHEIGHT;
		break;
	case MSG_MISSION:
		messageX = INTMAP_MISSIONX;
		messageY = INTMAP_MISSIONY;
		messageWidth = INTMAP_MISSIONWIDTH;
		messageHeight = INTMAP_MISSIONHEIGHT;
		break;
	case MSG_PROXIMITY:
		messageX = INTMAP_PROXIMITYX;
		messageY = INTMAP_PROXIMITYY;
		messageWidth = INTMAP_PROXIMITYWIDTH;
		messageHeight = INTMAP_PROXIMITYHEIGHT;
		break;
	default:
		ASSERT( FALSE, "Unknown message type" );
		return FALSE;
	}

	if ((x > messageX AND x < (messageX + messageWidth)) AND
		(y > messageY AND y < (messageY + messageHeight)))
	{
		return FALSE;
	}
	else
	{
		return TRUE;
	}
}*/
/*sets which states need to be paused when the intelligence screen is up*/
#[no_mangle]
pub unsafe extern "C" fn setIntelligencePauseState() {
    if bMultiPlayer == 0 {
        gameTimeStop();
        setGameUpdatePause(1 as libc::c_int);
        if bInTutorial == 0 {
            // Don't pause the scripts or the console if the tutorial is running.
            setScriptPause(1 as libc::c_int);
            setConsolePause(1 as libc::c_int);
        }
        setScrollPause(1 as libc::c_int);
    };
}
/*resets the pause states */
/*resets the pause states */
#[no_mangle]
pub unsafe extern "C" fn resetIntelligencePauseState() {
    if bMultiPlayer == 0 {
        setGameUpdatePause(0 as libc::c_int);
        if bInTutorial == 0 { setScriptPause(0 as libc::c_int); }
        setScrollPause(0 as libc::c_int);
        setConsolePause(0 as libc::c_int);
        gameTimeStart();
    };
}
// play this message immediately, but definitely donot tell the intelligence screen to start
#[no_mangle]
pub unsafe extern "C" fn _displayImmediateMessage(mut psMessage:
                                                      *mut MESSAGE) {
    /*
		This has to be changed to support a script calling a message in the intellegence screen

	*/
    psCurrentMsg = psMessage;
    /* so we lied about definately not starting the intelligence screen */
    addIntelScreen();
    /* reset mouse cursor, since addIntelScreen() doesn't do that */
    pie_SetMouse(IntImages, IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD);
    frameSetCursorFromRes(108 as libc::c_int as WORD);
}
// tell the intelligence screen to play this message immediately
#[no_mangle]
pub unsafe extern "C" fn displayImmediateMessage(mut psMessage:
                                                     *mut MESSAGE) {
    _displayImmediateMessage(psMessage);
}
// return whether a message is immediate
// return whether a message is immediate
#[no_mangle]
pub unsafe extern "C" fn messageIsImmediate() -> BOOL {
    return immediateMessage;
}
/*sets the flag*/
/*sets the flag*/
#[no_mangle]
pub unsafe extern "C" fn setMessageImmediate(mut state: BOOL) {
    immediateMessage = state;
}
// The current message being displayed
// The display stats for the current messages' text
/* Add the Intelligence Map widgets to the widget screen */
//extern BOOL intAddIntelMap(BOOL playCurrent);
#[no_mangle]
pub unsafe extern "C" fn intAddIntelMap() -> BOOL {
    return _intAddIntelMap();
}
/* displays the 3D view for the current message */
//static void intDisplayMessageView(struct _widget *psWidget, UDWORD xOffset, UDWORD yOffset,
//					  UDWORD *pColours);
/* Add the Proximity message buttons */
//static BOOL intAddProximityButton(MESSAGE *pMessage, UDWORD inc);
/*Displays the proximity messages used on the intelligence map */
//static void intDisplayProximityButton(struct _widget *psWidget, UDWORD xOffset,
//							  UDWORD yOffset, UDWORD *pColours);
/*deal with the actual button press - proxMsg is set to true if a proximity
  button has been pressed*/
unsafe extern "C" fn intIntelButtonPressed(mut proxMsg: BOOL,
                                           mut id: UDWORD) {
    _intIntelButtonPressed(proxMsg, id);
}
