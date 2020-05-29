use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
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
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Get a button or clickable form's state */
    #[no_mangle]
    fn widgGetButtonState(psScreen: *mut W_SCREEN, id: UDWORD) -> UDWORD;
    /* Set a button or clickable form's state */
    #[no_mangle]
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
    /* Clean up after a screen has been run.
 * Call this after the widgRunScreen / widgDisplayScreen cycle.
 */
    #[no_mangle]
    fn widgEndScreen(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    // Count number of factories assignable to a command droid.
    #[no_mangle]
    fn countAssignableFactories(player: UBYTE, FactoryType: UWORD) -> UWORD;
    // check whether a factory of a certain number and type exists
    #[no_mangle]
    fn checkFactoryExists(player: UDWORD, factoryType: UDWORD, inc: UDWORD)
     -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
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
    /* The button ID of the objects stat when the stat screen is displayed */
    #[no_mangle]
    static mut objStatID: UDWORD;
    #[no_mangle]
    fn intRemoveStatsNoAnim();
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    // see if a droid supports a secondary order
    #[no_mangle]
    fn secondarySupported(psDroid: *mut DROID, sec: SECONDARY_ORDER) -> BOOL;
    // get the state of a secondary order, return FALSE if unsupported
    #[no_mangle]
    fn secondaryGetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         pState: *mut SECONDARY_STATE) -> BOOL;
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    // set the state of a secondary order for a Factory, return FALSE if failed.
    #[no_mangle]
    fn setFactoryState(psStruct: *mut STRUCTURE, sec: SECONDARY_ORDER,
                       State: SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    fn getFactoryState(psStruct: *mut STRUCTURE, sec: SECONDARY_ORDER,
                       pState: *mut SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intClosePlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayAltButtonHilight(psWidget: *mut _widget, xOffset: UDWORD,
                                  yOffset: UDWORD, pColours: *mut UDWORD);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    #[no_mangle]
    static mut ClosingOrder: BOOL;
}
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
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
// only needed when generating the tree
/* **************************************************************************/
// These 1st three entries can NOT NOW be cast into a iVectorf *   (iVectorf on PC are doubles)
// these values form the plane equation ax+by+cz=d
// a point on the plane - in normal non-fract format
pub type PSBSPTREENODE = *mut BSPTREENODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type BSPPOLYID = uint16;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
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
pub type VERTEXID = libc::c_int;
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
pub type DROID = _droid;
pub type STRUCTURE = _structure;
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
pub type W_FORM = _w_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
/* The common form data */
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
pub type _secondary_order = libc::c_uint;
pub const DSO_ASSIGN_VTOL_PRODUCTION: _secondary_order = 11;
pub const DSO_FIRE_DESIGNATOR: _secondary_order = 10;
pub const DSO_RETURN_TO_LOC: _secondary_order = 9;
pub const DSO_HALTTYPE: _secondary_order = 8;
pub const DSO_PATROL: _secondary_order = 7;
pub const DSO_RECYCLE: _secondary_order = 6;
pub const DSO_CLEAR_PRODUCTION: _secondary_order = 5;
pub const DSO_ASSIGN_CYBORG_PRODUCTION: _secondary_order = 4;
pub const DSO_ASSIGN_PRODUCTION: _secondary_order = 3;
pub const DSO_ATTACK_LEVEL: _secondary_order = 2;
pub const DSO_REPAIR_LEVEL: _secondary_order = 1;
pub const DSO_ATTACK_RANGE: _secondary_order = 0;
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
// secondary orders for droids
pub type SECONDARY_ORDER = _secondary_order;
pub type _secondary_state = libc::c_uint;
pub const DSS_VTOLPROD_END: _secondary_state = 268435456;
pub const DSS_VTOLPROD_START: _secondary_state = 16777216;
pub const DSS_FIREDES_SET: _secondary_state = 8388608;
pub const DSS_PATROL_SET: _secondary_state = 4194304;
pub const DSS_RTL_TRANSPORT: _secondary_state = 2097152;
pub const DSS_RTL_BASE: _secondary_state = 1048576;
pub const DSS_RTL_REPAIR: _secondary_state = 524288;
pub const DSS_ASSPROD_END: _secondary_state = 262144;
pub const DSS_ASSPROD_MID: _secondary_state = 8192;
pub const DSS_ASSPROD_START: _secondary_state = 512;
pub const DSS_RECYCLE_SET: _secondary_state = 256;
pub const DSS_HALT_PERSUE: _secondary_state = 192;
pub const DSS_HALT_GUARD: _secondary_state = 128;
pub const DSS_HALT_HOLD: _secondary_state = 64;
pub const DSS_ALEV_NEVER: _secondary_state = 48;
pub const DSS_ALEV_ATTACKED: _secondary_state = 32;
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const DSS_ARANGE_LONG: _secondary_state = 2;
pub const DSS_ARANGE_SHORT: _secondary_state = 1;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
// the state of secondary orders
pub type SECONDARY_STATE = _secondary_state;
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
pub const ORD_JUSTIFY_COMBINE: ORDBUTTONJUSTIFY = 3;
pub type ORDBUTTONJUSTIFY = libc::c_uint;
pub const ORD_NUM_JUSTIFY_TYPES: ORDBUTTONJUSTIFY = 4;
pub const ORD_JUSTIFY_CENTER: ORDBUTTONJUSTIFY = 2;
pub const ORD_JUSTIFY_RIGHT: ORDBUTTONJUSTIFY = 1;
pub const ORD_JUSTIFY_LEFT: ORDBUTTONJUSTIFY = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ORDERBUTTONS {
    pub Class: ORDBUTTONCLASS,
    pub Order: SECONDARY_ORDER,
    pub StateMask: UDWORD,
    pub ButType: ORDBUTTONTYPE,
    pub ButJustify: ORDBUTTONJUSTIFY,
    pub ButBaseID: UDWORD,
    pub NumButs: UWORD,
    pub AcNumButs: UWORD,
    pub ButImageID: [UWORD; 5],
    pub ButGreyID: [UWORD; 5],
    pub ButHilightID: [UWORD; 5],
    pub ButTips: [UWORD; 5],
    pub States: [SECONDARY_STATE; 5],
}
pub type ORDBUTTONTYPE = libc::c_uint;
pub const ORD_BTYPE_BOOLEAN_COMBINE: ORDBUTTONTYPE = 3;
pub const ORD_BTYPE_BOOLEAN_DEPEND: ORDBUTTONTYPE = 2;
pub const ORD_BTYPE_BOOLEAN: ORDBUTTONTYPE = 1;
pub const ORD_BTYPE_RADIO: ORDBUTTONTYPE = 0;
pub type ORDBUTTONCLASS = libc::c_uint;
pub const ORDBUTCLASS_VTOLFACTORY: ORDBUTTONCLASS = 3;
pub const ORDBUTCLASS_CYBORGFACTORY: ORDBUTTONCLASS = 2;
pub const ORDBUTCLASS_FACTORY: ORDBUTTONCLASS = 1;
pub const ORDBUTCLASS_NORMAL: ORDBUTTONCLASS = 0;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
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
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVORDER {
    pub OrderIndex: UWORD,
    pub RefCount: UWORD,
}
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
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
// Define the order button groups.
#[no_mangle]
pub static mut OrderButtons: [ORDERBUTTONS; 11] =
    [{
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_ATTACK_RANGE,
                          StateMask: 0x3 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_RADIO,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8010 as libc::c_int as UDWORD,
                          NumButs: 3 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_RANGE3UP as libc::c_int as UWORD,
                               IMAGE_ORD_RANGE1UP as libc::c_int as UWORD,
                               IMAGE_ORD_RANGE2UP as libc::c_int as UWORD, 0,
                               0],
                          ButGreyID:
                              [IMAGE_ORD_RANGE3UP as libc::c_int as UWORD,
                               IMAGE_ORD_RANGE1UP as libc::c_int as UWORD,
                               IMAGE_ORD_RANGE2UP as libc::c_int as UWORD, 0,
                               0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD, 0,
                               0],
                          ButTips:
                              [STR_DORD_RANGE3 as libc::c_int as UWORD,
                               STR_DORD_RANGE1 as libc::c_int as UWORD,
                               STR_DORD_RANGE2 as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_ARANGE_DEFAULT, DSS_ARANGE_SHORT,
                               DSS_ARANGE_LONG, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_REPAIR_LEVEL,
                          StateMask: 0xc as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_RADIO,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8020 as libc::c_int as UDWORD,
                          NumButs: 3 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_REPAIR3UP as libc::c_int as UWORD,
                               IMAGE_ORD_REPAIR2UP as libc::c_int as UWORD,
                               IMAGE_ORD_REPAIR1UP as libc::c_int as UWORD, 0,
                               0],
                          ButGreyID:
                              [IMAGE_ORD_REPAIR3UP as libc::c_int as UWORD,
                               IMAGE_ORD_REPAIR2UP as libc::c_int as UWORD,
                               IMAGE_ORD_REPAIR1UP as libc::c_int as UWORD, 0,
                               0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD, 0,
                               0],
                          ButTips:
                              [STR_DORD_REPAIR3 as libc::c_int as UWORD,
                               STR_DORD_REPAIR2 as libc::c_int as UWORD,
                               STR_DORD_REPAIR1 as libc::c_int as UWORD, 0,
                               0],
                          States:
                              [DSS_REPLEV_NEVER, DSS_REPLEV_HIGH,
                               DSS_REPLEV_LOW, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_ATTACK_LEVEL,
                          StateMask: 0x30 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_RADIO,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8030 as libc::c_int as UDWORD,
                          NumButs: 3 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_FATWILLUP as libc::c_int as UWORD,
                               IMAGE_ORD_RETFIREUP as libc::c_int as UWORD,
                               IMAGE_ORD_HOLDFIREUP as libc::c_int as UWORD,
                               0, 0],
                          ButGreyID:
                              [IMAGE_ORD_FATWILLUP as libc::c_int as UWORD,
                               IMAGE_ORD_RETFIREUP as libc::c_int as UWORD,
                               IMAGE_ORD_HOLDFIREUP as libc::c_int as UWORD,
                               0, 0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD, 0,
                               0],
                          ButTips:
                              [STR_DORD_FIRE1 as libc::c_int as UWORD,
                               STR_DORD_FIRE2 as libc::c_int as UWORD,
                               STR_DORD_FIRE3 as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_ALEV_ALWAYS, DSS_ALEV_ATTACKED,
                               DSS_ALEV_NEVER, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_FIRE_DESIGNATOR,
                          StateMask: 0x800000 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN,
                          ButJustify: ORD_JUSTIFY_COMBINE,
                          ButBaseID: 8100 as libc::c_int as UDWORD,
                          NumButs: 1 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_FIREDES_UP as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButGreyID:
                              [IMAGE_ORD_FIREDES_UP as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButTips:
                              [STR_DORD_FIREDES as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_FIREDES_SET, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_PATROL,
                          StateMask: 0x400000 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN,
                          ButJustify: ORD_JUSTIFY_COMBINE,
                          ButBaseID: 8040 as libc::c_int as UDWORD,
                          NumButs: 1 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_PATROLUP as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButGreyID:
                              [IMAGE_ORD_PATROLUP as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButTips:
                              [STR_DORD_PATROL as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_PATROL_SET, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_HALTTYPE,
                          StateMask: 0xc0 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_RADIO,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8050 as libc::c_int as UDWORD,
                          NumButs: 3 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_PERSUEUP as libc::c_int as UWORD,
                               IMAGE_ORD_GUARDUP as libc::c_int as UWORD,
                               IMAGE_ORD_HALTUP as libc::c_int as UWORD, 0,
                               0],
                          ButGreyID:
                              [IMAGE_ORD_PERSUEUP as libc::c_int as UWORD,
                               IMAGE_ORD_GUARDUP as libc::c_int as UWORD,
                               IMAGE_ORD_HALTUP as libc::c_int as UWORD, 0,
                               0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD, 0,
                               0],
                          ButTips:
                              [STR_DORD_PERSUE as libc::c_int as UWORD,
                               STR_DORD_GUARD as libc::c_int as UWORD,
                               STR_DORD_HOLDPOS as libc::c_int as UWORD, 0,
                               0],
                          States:
                              [DSS_HALT_PERSUE, DSS_HALT_GUARD, DSS_HALT_HOLD,
                               0 as SECONDARY_STATE, 0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_RETURN_TO_LOC,
                          StateMask: 0x380000 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_RADIO,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8060 as libc::c_int as UDWORD,
                          NumButs: 3 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_RTRUP as libc::c_int as UWORD,
                               IMAGE_ORD_GOTOHQUP as libc::c_int as UWORD,
                               IMAGE_ORD_EMBARKUP as libc::c_int as UWORD, 0,
                               0],
                          ButGreyID:
                              [IMAGE_ORD_RTRUP as libc::c_int as UWORD,
                               IMAGE_ORD_GOTOHQUP as libc::c_int as UWORD,
                               IMAGE_ORD_EMBARKUP as libc::c_int as UWORD, 0,
                               0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD, 0,
                               0],
                          ButTips:
                              [STR_DORD_RETREPAIR as libc::c_int as UWORD,
                               STR_DORD_RETBASE as libc::c_int as UWORD,
                               STR_DORD_EMBARK as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_RTL_REPAIR, DSS_RTL_BASE,
                               DSS_RTL_TRANSPORT, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_NORMAL,
                          Order: DSO_RECYCLE,
                          StateMask: 0x100 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN_DEPEND,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8070 as libc::c_int as UDWORD,
                          NumButs: 2 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_DESTRUCT1UP as libc::c_int as UWORD,
                               IMAGE_ORD_DESTRUCT2UP as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButGreyID:
                              [IMAGE_ORD_DESTRUCT1UP as libc::c_int as UWORD,
                               IMAGE_ORD_DESTRUCT2GREY as libc::c_int as
                                   UWORD, 0 as libc::c_int as UWORD, 0, 0],
                          ButHilightID:
                              [IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               IMAGE_DES_HILIGHT as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          ButTips:
                              [STR_DORD_ARMRECYCLE as libc::c_int as UWORD,
                               STR_DORD_RECYCLE as libc::c_int as UWORD,
                               0 as libc::c_int as UWORD, 0, 0],
                          States:
                              [DSS_RECYCLE_SET, DSS_RECYCLE_SET,
                               0 as SECONDARY_STATE, 0 as SECONDARY_STATE,
                               0 as SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_FACTORY,
                          Order: DSO_ASSIGN_PRODUCTION,
                          StateMask: 0x1f07fe00 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN_COMBINE,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8080 as libc::c_int as UDWORD,
                          NumButs: 5 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButGreyID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButHilightID:
                              [IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD],
                          ButTips:
                              [STR_DORD_FACTORY as libc::c_int as UWORD,
                               STR_DORD_FACTORY as libc::c_int as UWORD,
                               STR_DORD_FACTORY as libc::c_int as UWORD,
                               STR_DORD_FACTORY as libc::c_int as UWORD,
                               STR_DORD_FACTORY as libc::c_int as UWORD],
                          States:
                              [((0x1 as libc::c_int) << 9 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x2 as libc::c_int) << 9 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x4 as libc::c_int) << 9 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x8 as libc::c_int) << 9 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x10 as libc::c_int) << 9 as libc::c_int) as
                                   SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_CYBORGFACTORY,
                          Order: DSO_ASSIGN_CYBORG_PRODUCTION,
                          StateMask: 0x1f07fe00 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN_COMBINE,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8090 as libc::c_int as UDWORD,
                          NumButs: 5 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButGreyID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButHilightID:
                              [IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD],
                          ButTips:
                              [STR_DORD_CYBORG_FACTORY as libc::c_int as
                                   UWORD,
                               STR_DORD_CYBORG_FACTORY as libc::c_int as
                                   UWORD,
                               STR_DORD_CYBORG_FACTORY as libc::c_int as
                                   UWORD,
                               STR_DORD_CYBORG_FACTORY as libc::c_int as
                                   UWORD,
                               STR_DORD_CYBORG_FACTORY as libc::c_int as
                                   UWORD],
                          States:
                              [((0x1 as libc::c_int) <<
                                    9 as libc::c_int + 5 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x2 as libc::c_int) <<
                                    9 as libc::c_int + 5 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x4 as libc::c_int) <<
                                    9 as libc::c_int + 5 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x8 as libc::c_int) <<
                                    9 as libc::c_int + 5 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x10 as libc::c_int) <<
                                    9 as libc::c_int + 5 as libc::c_int) as
                                   SECONDARY_STATE],};
         init
     },
     {
         let mut init =
             ORDERBUTTONS{Class: ORDBUTCLASS_VTOLFACTORY,
                          Order: DSO_ASSIGN_VTOL_PRODUCTION,
                          StateMask: 0x1f07fe00 as libc::c_int as UDWORD,
                          ButType: ORD_BTYPE_BOOLEAN_COMBINE,
                          ButJustify:
                              (ORD_JUSTIFY_CENTER as libc::c_int |
                                   0x10 as libc::c_int) as ORDBUTTONJUSTIFY,
                          ButBaseID: 8110 as libc::c_int as UDWORD,
                          NumButs: 5 as libc::c_int as UWORD,
                          AcNumButs: 0 as libc::c_int as UWORD,
                          ButImageID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButGreyID:
                              [IMAGE_ORD_FAC1UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC2UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC3UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC4UP as libc::c_int as UWORD,
                               IMAGE_ORD_FAC5UP as libc::c_int as UWORD],
                          ButHilightID:
                              [IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD,
                               IMAGE_ORD_FACHILITE as libc::c_int as UWORD],
                          ButTips:
                              [STR_DORD_VTOL_FACTORY as libc::c_int as UWORD,
                               STR_DORD_VTOL_FACTORY as libc::c_int as UWORD,
                               STR_DORD_VTOL_FACTORY as libc::c_int as UWORD,
                               STR_DORD_VTOL_FACTORY as libc::c_int as UWORD,
                               STR_DORD_VTOL_FACTORY as libc::c_int as UWORD],
                          States:
                              [((0x1 as libc::c_int) << 24 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x2 as libc::c_int) << 24 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x4 as libc::c_int) << 24 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x8 as libc::c_int) << 24 as libc::c_int) as
                                   SECONDARY_STATE,
                               ((0x10 as libc::c_int) << 24 as libc::c_int) as
                                   SECONDARY_STATE],};
         init
     }];
static mut NumSelectedDroids: UWORD = 0;
static mut SelectedDroids: [*mut DROID; 100] =
    [0 as *const DROID as *mut DROID; 100];
static mut psSelectedFactory: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
static mut NumAvailableOrders: UWORD = 0;
static mut AvailableOrders: [AVORDER; 16] =
    [AVORDER{OrderIndex: 0, RefCount: 0,}; 16];
#[no_mangle]
pub static mut OrderUp: BOOL = 0 as libc::c_int;
// update the order interface only if it is already open.
#[no_mangle]
pub unsafe extern "C" fn intUpdateOrder(mut psDroid: *mut DROID) -> BOOL {
    if !widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD).is_null() &&
           OrderUp != 0 {
        widgDelete(psWScreen, 8001 as libc::c_int as UDWORD);
        //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
		//intAddOrder(psDroid);
        intAddOrder(psDroid as *mut BASE_OBJECT);
    }
    return 1 as libc::c_int;
}
// Add the droid order screen.
// Returns TRUE if the form was displayed ok.
//
//changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
//BOOL _intAddOrder(DROID *Droid)
#[no_mangle]
pub unsafe extern "C" fn _intAddOrder(mut psObj: *mut BASE_OBJECT) -> BOOL {
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
                   pFormDisplay: None,}; //,k;
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
    let mut Animate: BOOL = 1 as libc::c_int;
    let mut State: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut i: UWORD = 0;
    let mut j: UWORD = 0;
    let mut OrdIndex: UWORD = 0;
    let mut Form: *mut W_FORM = 0 as *mut W_FORM;
    let mut Height: UWORD = 0;
    let mut NumDisplayedOrders: UWORD = 0;
    let mut NumButs: UWORD = 0;
    let mut NumJustifyButs: UWORD = 0;
    let mut NumCombineButs: UWORD = 0;
    let mut NumCombineBefore: UWORD = 0;
    let mut bLastCombine: BOOL = 0;
    let mut bHidden: BOOL = 0;
    //added to accomodate the factories - AB 21/04/99
    let mut Droid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if bInTutorial != 0 {
        // No RMB orders in tutorial!!
        return 0 as libc::c_int
    }
    // Is the form already up?
    if !widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD).is_null() {
        intRemoveOrderNoAnim();
        Animate = 0 as libc::c_int
    }
    // Is the stats window up?
    if !widgGetFromID(psWScreen, 4000 as libc::c_int as UDWORD).is_null() {
        intRemoveStatsNoAnim();
        Animate = 0 as libc::c_int
    }
    if !psObj.is_null() {
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            Droid = psObj as *mut DROID;
            psStructure = 0 as *mut STRUCTURE
        } else if (*psObj).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            Droid = 0 as *mut DROID;
            psStructure = psObj as *mut STRUCTURE;
            psSelectedFactory = psStructure
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"_intAddOrder: Invalid object type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intorder.c\x00" as *const u8 as *const libc::c_char,
                      492 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"_intAddOrder\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            Droid = 0 as *mut DROID;
            psStructure = 0 as *mut STRUCTURE
        }
    } else { Droid = 0 as *mut DROID; psStructure = 0 as *mut STRUCTURE }
    //	intResetScreen(TRUE);
    setWidgetsStatus(1 as libc::c_int);
    NumAvailableOrders = 0 as libc::c_int as UWORD;
    NumSelectedDroids = 0 as libc::c_int as UWORD;
    // Selected droid is a command droid?
    if !Droid.is_null() &&
           (*Droid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
        // displaying for a command droid - ignore any other droids
        SelectedDroids[0 as libc::c_int as usize] = Droid;
        NumSelectedDroids = 1 as libc::c_int as UWORD
    } else if !psStructure.is_null() {
        if BuildStructureOrderList(psStructure) == 0 {
            return 0 as libc::c_int
        }
    } else if BuildSelectedDroidList() == 0 {
        //added to accomodate the factories - AB 21/04/99
        // Otherwise build a list of selected droids.
        // If no droids selected then see if we were given a specific droid.
        if !Droid.is_null() {
            // and put it in the list.
            SelectedDroids[0 as libc::c_int as usize] = Droid;
            NumSelectedDroids = 1 as libc::c_int as UWORD
        }
    }
    // Build a list of orders available for the list of selected droids. - if a factory has not been selected
    if psStructure.is_null() {
        if BuildDroidOrderList() == 0 {
            // If no orders then return;
            return 0 as libc::c_int
        }
    }
    widgEndScreen(psWScreen);
    /* Create the basic form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 8000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 23 as libc::c_int as SWORD;
    sFormInit.y = 45 as libc::c_int as SWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
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
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    // Add the close button.
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 8000 as libc::c_int as UDWORD;
    sButInit.id = 8001 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (132 as libc::c_int - 15 as libc::c_int) as SWORD;
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
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 8000 as libc::c_int as UDWORD;
    sButInit.id = (8001 as libc::c_int + 1 as libc::c_int) as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.pDisplay =
        Some(intDisplayAltButtonHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.FontID = WFont;
    sButInit.y = 16 as libc::c_int as SWORD;
    Height = 0 as libc::c_int as UWORD;
    NumDisplayedOrders = 0 as libc::c_int as UWORD;
    j = 0 as libc::c_int as UWORD;
    while (j as libc::c_int) < NumAvailableOrders as libc::c_int &&
              (NumDisplayedOrders as libc::c_int) < 11 as libc::c_int {
        OrdIndex = AvailableOrders[j as usize].OrderIndex;
        // Get current order state.
   		//secondaryGetState(SelectedDroids[0], OrderButtons[OrdIndex].Order, &State);
        State =
            GetSecondaryStates(OrderButtons[OrdIndex as usize].Order) as
                SECONDARY_STATE;
        // Get number of buttons.
        NumButs = OrderButtons[OrdIndex as usize].NumButs;
        // Set actual number of buttons.
        OrderButtons[OrdIndex as usize].AcNumButs = NumButs;
        // Handle special case for factory -> command droid assignment buttons.
        match OrderButtons[OrdIndex as usize].Class as libc::c_uint {
            1 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             0 as libc::c_int as UWORD)
            }
            2 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             1 as libc::c_int as UWORD)
            }
            3 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             2 as libc::c_int as UWORD)
            }
            _ => { }
        }
        sButInit.id = OrderButtons[OrdIndex as usize].ButBaseID;
        NumJustifyButs = NumButs;
        //		for(k=j; k<NumAvailableOrders; k++) {
//	   		UWORD Index = AvailableOrders[j].OrderIndex;
//			if(OrderButtons[OrdIndex].ButJustify & ORD_JUSTIFY_NEWLINE) {
//				DBPRINTF(("NewLine %d \n",k);
//				break;
//			} else {
//				DBPRINTF(("SameLine %d \n",k);
//			}
//		}
        bLastCombine = 0 as libc::c_int;
        match OrderButtons[OrdIndex as usize].ButJustify as libc::c_uint &
                  0xf as libc::c_int as libc::c_uint {
            0 => { sButInit.x = 8 as libc::c_int as SWORD }
            1 => {
                sButInit.x =
                    ((sFormInit.width as libc::c_int - 8 as libc::c_int) as
                         libc::c_uint).wrapping_sub((NumJustifyButs as
                                                         libc::c_uint).wrapping_mul(GetImageWidth(IntImages,
                                                                                                  OrderButtons[OrdIndex
                                                                                                                   as
                                                                                                                   usize].ButImageID[0
                                                                                                                                         as
                                                                                                                                         libc::c_int
                                                                                                                                         as
                                                                                                                                         usize]
                                                                                                      as
                                                                                                      UDWORD)).wrapping_add(((NumJustifyButs
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  -
                                                                                                                                  1
                                                                                                                                      as
                                                                                                                                      libc::c_int)
                                                                                                                                 *
                                                                                                                                 4
                                                                                                                                     as
                                                                                                                                     libc::c_int)
                                                                                                                                as
                                                                                                                                libc::c_uint))
                        as SWORD
            }
            2 => {
                //				sButInit.x = (SWORD)((sFormInit.width / 2) -
//						( ((NumJustifyButs * GetImageWidth(IntImages,OrderButtons[OrdIndex].ButImageID[0])) +
//						((NumJustifyButs-1) * ORDER_BUTGAP ) ) / 2 ));
                sButInit.x =
                    ((sFormInit.width as
                          libc::c_uint).wrapping_sub((NumJustifyButs as
                                                          libc::c_uint).wrapping_mul(GetImageWidth(IntImages,
                                                                                                   OrderButtons[OrdIndex
                                                                                                                    as
                                                                                                                    usize].ButImageID[0
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          usize]
                                                                                                       as
                                                                                                       UDWORD)).wrapping_add(((NumJustifyButs
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   -
                                                                                                                                   1
                                                                                                                                       as
                                                                                                                                       libc::c_int)
                                                                                                                                  *
                                                                                                                                  4
                                                                                                                                      as
                                                                                                                                      libc::c_int)
                                                                                                                                 as
                                                                                                                                 libc::c_uint))
                         as SWORD as libc::c_int / 2 as libc::c_int) as SWORD
            }
            3 => {
                // see how many are on this line before the button
                NumCombineBefore = 0 as libc::c_int as UWORD;
                i = 0 as libc::c_int as UWORD;
                while (i as libc::c_int) < j as libc::c_int {
                    if OrderButtons[AvailableOrders[i as usize].OrderIndex as
                                        usize].ButJustify as libc::c_uint &
                           0xf as libc::c_int as libc::c_uint ==
                           ORD_JUSTIFY_COMBINE as libc::c_int as libc::c_uint
                       {
                        NumCombineBefore =
                            (NumCombineBefore as libc::c_int +
                                 1 as libc::c_int) as UWORD
                    }
                    i = i.wrapping_add(1)
                }
                NumCombineButs =
                    (NumCombineBefore as libc::c_int + 1 as libc::c_int) as
                        UWORD;
                // now see how many in total
                i = (j as libc::c_int + 1 as libc::c_int) as UWORD;
                while (i as libc::c_int) < NumAvailableOrders as libc::c_int {
                    if OrderButtons[AvailableOrders[i as usize].OrderIndex as
                                        usize].ButJustify as libc::c_uint &
                           0xf as libc::c_int as libc::c_uint ==
                           ORD_JUSTIFY_COMBINE as libc::c_int as libc::c_uint
                       {
                        NumCombineButs =
                            (NumCombineButs as libc::c_int + 1 as libc::c_int)
                                as UWORD
                    }
                    i = i.wrapping_add(1)
                }
                // get position on line
                NumCombineButs =
                    (NumCombineButs as libc::c_int -
                         (NumCombineBefore as libc::c_int -
                              NumCombineBefore as libc::c_int %
                                  3 as libc::c_int)) as UWORD;
                if NumCombineButs as libc::c_int >= 3 as libc::c_int {
                    // the buttons will fill the line
                    sButInit.x =
                        (8 as libc::c_int as
                             libc::c_uint).wrapping_add(GetImageWidth(IntImages,
                                                                      OrderButtons[OrdIndex
                                                                                       as
                                                                                       usize].ButImageID[0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                                          as
                                                                          UDWORD).wrapping_add(4
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_mul(NumCombineBefore
                                                                                                                                  as
                                                                                                                                  libc::c_uint))
                            as SWORD
                } else {
                    // center the buttons
                    sButInit.x =
                        ((sFormInit.width as libc::c_int / 2 as libc::c_int)
                             as
                             libc::c_uint).wrapping_sub((NumCombineButs as
                                                             libc::c_uint).wrapping_mul(GetImageWidth(IntImages,
                                                                                                      OrderButtons[OrdIndex
                                                                                                                       as
                                                                                                                       usize].ButImageID[0
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             usize]
                                                                                                          as
                                                                                                          UDWORD)).wrapping_add(((NumCombineButs
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      -
                                                                                                                                      1
                                                                                                                                          as
                                                                                                                                          libc::c_int)
                                                                                                                                     *
                                                                                                                                     4
                                                                                                                                         as
                                                                                                                                         libc::c_int)
                                                                                                                                    as
                                                                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                                                                   as
                                                                                                                                                                   libc::c_int
                                                                                                                                                                   as
                                                                                                                                                                   libc::c_uint))
                            as SWORD;
                    sButInit.x =
                        (sButInit.x as
                             libc::c_uint).wrapping_add(GetImageWidth(IntImages,
                                                                      OrderButtons[OrdIndex
                                                                                       as
                                                                                       usize].ButImageID[0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             usize]
                                                                          as
                                                                          UDWORD).wrapping_add(4
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_mul(NumCombineBefore
                                                                                                                                  as
                                                                                                                                  libc::c_uint))
                            as SWORD
                }
                // see if need to start a new line of buttons
                if NumCombineBefore as libc::c_int + 1 as libc::c_int ==
                       NumCombineButs as libc::c_int % 3 as libc::c_int {
                    bLastCombine = 1 as libc::c_int
                }
            }
            _ => { }
        }
        i = 0 as libc::c_int as UWORD;
        while (i as libc::c_int) <
                  OrderButtons[OrdIndex as usize].AcNumButs as libc::c_int {
            sButInit.pTip =
                strresGetString(psStringRes,
                                OrderButtons[OrdIndex as
                                                 usize].ButTips[i as usize] as
                                    UDWORD);
            sButInit.width =
                GetImageWidth(IntImages,
                              OrderButtons[OrdIndex as
                                               usize].ButImageID[i as usize]
                                  as UDWORD) as UWORD;
            sButInit.height =
                GetImageHeight(IntImages,
                               OrderButtons[OrdIndex as
                                                usize].ButImageID[i as usize]
                                   as UDWORD) as UWORD;
            sButInit.pUserData =
                ((OrderButtons[OrdIndex as usize].ButGreyID[i as usize] as
                      libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int
                     |
                     (OrderButtons[OrdIndex as usize].ButHilightID[i as usize]
                          as libc::c_int & 0x3ff as libc::c_int) <<
                         10 as libc::c_int |
                     OrderButtons[OrdIndex as usize].ButImageID[i as usize] as
                         libc::c_int & 0x3ff as libc::c_int) as
                    *mut libc::c_void;
            if widgAddButton(psWScreen, &mut sButInit) == 0 {
                return 0 as libc::c_int
            }
            // Set the default state for the button.
            match OrderButtons[OrdIndex as usize].ButType as libc::c_uint {
                0 | 1 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].StateMask ==
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0x4 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0 as libc::c_int as UDWORD);
                    }
                }
                2 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].StateMask ==
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0x4 as libc::c_int as UDWORD);
                    } else if i as libc::c_int == 0 as libc::c_int {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0x1 as libc::c_int as UDWORD);
                    }
                }
                3 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD != 0 {
                        widgSetButtonState(psWScreen, sButInit.id,
                                           0x4 as libc::c_int as UDWORD);
                    }
                }
                _ => { }
            }
            // may not add a button if the factory doesn't exist
            bHidden = 0 as libc::c_int;
            match OrderButtons[OrdIndex as usize].Class as libc::c_uint {
                1 => {
                    if checkFactoryExists(selectedPlayer,
                                          0 as libc::c_int as UDWORD,
                                          i as UDWORD) == 0 {
                        widgHide(psWScreen, sButInit.id);
                        bHidden = 1 as libc::c_int
                    }
                }
                2 => {
                    if checkFactoryExists(selectedPlayer,
                                          1 as libc::c_int as UDWORD,
                                          i as UDWORD) == 0 {
                        widgHide(psWScreen, sButInit.id);
                        bHidden = 1 as libc::c_int
                    }
                }
                3 => {
                    if checkFactoryExists(selectedPlayer,
                                          2 as libc::c_int as UDWORD,
                                          i as UDWORD) == 0 {
                        widgHide(psWScreen, sButInit.id);
                        bHidden = 1 as libc::c_int
                    }
                }
                _ => { }
            }
            if bHidden == 0 {
                sButInit.x =
                    (sButInit.x as libc::c_int + sButInit.width as libc::c_int
                         + 4 as libc::c_int) as SWORD
            }
            sButInit.id = sButInit.id.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        if OrderButtons[OrdIndex as usize].ButJustify as libc::c_uint &
               0xf as libc::c_int as libc::c_uint !=
               ORD_JUSTIFY_COMBINE as libc::c_int as libc::c_uint ||
               bLastCombine != 0 {
            sButInit.y =
                (sButInit.y as libc::c_int + sButInit.height as libc::c_int +
                     4 as libc::c_int) as SWORD;
            Height =
                (Height as libc::c_int + sButInit.height as libc::c_int +
                     4 as libc::c_int) as UWORD
        }
        NumDisplayedOrders = NumDisplayedOrders.wrapping_add(1);
        j = j.wrapping_add(1)
    }
    // Now we know how many orders there are we can resize the form accordingly.
    Form =
        widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD) as
            *mut W_FORM;
    (*Form).height =
        (Height as libc::c_int + 15 as libc::c_int + 4 as libc::c_int) as
            UWORD;
    (*Form).y =
        (318 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)).wrapping_sub((*Form).height
                                                                                                                    as
                                                                                                                    libc::c_uint)
            as SWORD;
    OrderUp = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// create and open order form
// Do any housekeeping for the droid order screen.
// Any droids that die now get set to NULL - John.
// No droids being selected no longer removes the screen,
// this lets the screen work with command droids - John.
#[no_mangle]
pub unsafe extern "C" fn intRunOrder() {
    let mut i: UWORD = 0;
    let mut NumDead: UDWORD = 0 as libc::c_int as UDWORD;
    let mut NumSelected: UDWORD = 0 as libc::c_int as UDWORD;
    // Check to see if there all dead or unselected.
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < NumSelectedDroids as libc::c_int {
        if !SelectedDroids[i as usize].is_null() {
            //			if(SelectedDroids[i]->selected) {
            NumSelected = NumSelected.wrapping_add(1);
            //			}
            if (*SelectedDroids[i as usize]).died != 0 {
                NumDead = NumDead.wrapping_add(1);
                SelectedDroids[i as usize] = 0 as *mut DROID
            }
        }
        i = i.wrapping_add(1)
    }
    // If all dead then remove the screen.
    if NumDead == NumSelectedDroids as libc::c_uint {
        //might have a factory selected
        if psSelectedFactory.is_null() { intRemoveOrder(); return }
    }
    // If droids no longer selected then remove screen.
    if NumSelected == 0 as libc::c_int as libc::c_uint {
        //might have a factory selected
        if psSelectedFactory.is_null() { intRemoveOrder(); return }
    };
}
// Process the droid order screen.
//
#[no_mangle]
pub unsafe extern "C" fn _intProcessOrder(mut id: UDWORD) {
    let mut i: UWORD = 0;
    let mut OrdIndex: UWORD = 0;
    let mut BaseID: UDWORD = 0;
    let mut StateIndex: UDWORD = 0;
    let mut CombineState: UDWORD = 0;
    if id == 8001 as libc::c_int as libc::c_uint {
        intRemoveOrder();
        if intMode as libc::c_uint == INT_ORDER as libc::c_int as libc::c_uint
           {
            intMode = INT_NORMAL
        } else {
            /* Unlock the stats button */
            widgSetButtonState(psWScreen, objStatID,
                               0 as libc::c_int as UDWORD);
            intMode = INT_OBJECT
        }
        return
    }
    OrdIndex = 0 as libc::c_int as UWORD;
    while (OrdIndex as libc::c_int) < 11 as libc::c_int {
        BaseID = OrderButtons[OrdIndex as usize].ButBaseID;
        match OrderButtons[OrdIndex as usize].ButType as libc::c_uint {
            0 => {
                if id >= BaseID &&
                       id <
                           BaseID.wrapping_add(OrderButtons[OrdIndex as
                                                                usize].AcNumButs
                                                   as libc::c_uint) {
                    StateIndex = id.wrapping_sub(BaseID);
                    i = 0 as libc::c_int as UWORD;
                    while (i as libc::c_int) <
                              OrderButtons[OrdIndex as usize].AcNumButs as
                                  libc::c_int {
                        widgSetButtonState(psWScreen,
                                           BaseID.wrapping_add(i as
                                                                   libc::c_uint),
                                           0 as libc::c_int as UDWORD);
                        i = i.wrapping_add(1)
                    }
                    if SetSecondaryState(OrderButtons[OrdIndex as
                                                          usize].Order,
                                         (OrderButtons[OrdIndex as
                                                           usize].States[StateIndex
                                                                             as
                                                                             usize]
                                              as libc::c_uint &
                                              OrderButtons[OrdIndex as
                                                               usize].StateMask)
                                             as SECONDARY_STATE) != 0 {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                    }
                }
            }
            1 => {
                if id >= BaseID &&
                       id <
                           BaseID.wrapping_add(OrderButtons[OrdIndex as
                                                                usize].AcNumButs
                                                   as libc::c_uint) {
                    StateIndex = id.wrapping_sub(BaseID);
                    if widgGetButtonState(psWScreen, id) &
                           0x4 as libc::c_int as libc::c_uint != 0 {
                        widgSetButtonState(psWScreen, id,
                                           0 as libc::c_int as UDWORD);
                        SetSecondaryState(OrderButtons[OrdIndex as
                                                           usize].Order,
                                          0 as SECONDARY_STATE);
                    } else {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                        SetSecondaryState(OrderButtons[OrdIndex as
                                                           usize].Order,
                                          (OrderButtons[OrdIndex as
                                                            usize].States[StateIndex
                                                                              as
                                                                              usize]
                                               as libc::c_uint &
                                               OrderButtons[OrdIndex as
                                                                usize].StateMask)
                                              as SECONDARY_STATE);
                    }
                }
            }
            2 => {
                // Toggle the state of this button.
                if id == BaseID {
                    if widgGetButtonState(psWScreen, id) &
                           0x4 as libc::c_int as libc::c_uint != 0 {
                        widgSetButtonState(psWScreen, id,
                                           0 as libc::c_int as UDWORD);
                        // Disable the dependant button.
                        widgSetButtonState(psWScreen,
                                           id.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint),
                                           0x1 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                        // Enable the dependant button.
                        widgSetButtonState(psWScreen,
                                           id.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint),
                                           0 as libc::c_int as UDWORD);
                    }
                }
                if id > BaseID &&
                       id <
                           BaseID.wrapping_add(OrderButtons[OrdIndex as
                                                                usize].AcNumButs
                                                   as libc::c_uint) {
                    // If the previous button is down ( armed )..
                    if widgGetButtonState(psWScreen,
                                          id.wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint)) &
                           0x4 as libc::c_int as libc::c_uint != 0 {
                        // Toggle the state of this button.
                        if widgGetButtonState(psWScreen, id) &
                               0x4 as libc::c_int as libc::c_uint != 0 {
                            widgSetButtonState(psWScreen, id,
                                               0 as libc::c_int as UDWORD);
                            SetSecondaryState(OrderButtons[OrdIndex as
                                                               usize].Order,
                                              0 as SECONDARY_STATE);
                        } else {
                            StateIndex = id.wrapping_sub(BaseID);
                            widgSetButtonState(psWScreen, id,
                                               0x4 as libc::c_int as UDWORD);
                            SetSecondaryState(OrderButtons[OrdIndex as
                                                               usize].Order,
                                              (OrderButtons[OrdIndex as
                                                                usize].States[StateIndex
                                                                                  as
                                                                                  usize]
                                                   as libc::c_uint &
                                                   OrderButtons[OrdIndex as
                                                                    usize].StateMask)
                                                  as SECONDARY_STATE);
                        }
                    }
                }
            }
            3 => {
                if id >= BaseID &&
                       id <
                           BaseID.wrapping_add(OrderButtons[OrdIndex as
                                                                usize].AcNumButs
                                                   as libc::c_uint) {
                    // Toggle the state of this button.
                    if widgGetButtonState(psWScreen, id) &
                           0x4 as libc::c_int as libc::c_uint != 0 {
                        widgSetButtonState(psWScreen, id,
                                           0 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                    }
                    // read the state of all the buttons to get the final state
                    CombineState = 0 as libc::c_int as UDWORD;
                    StateIndex = 0 as libc::c_int as UDWORD;
                    while StateIndex <
                              OrderButtons[OrdIndex as usize].AcNumButs as
                                  libc::c_uint {
                        if widgGetButtonState(psWScreen,
                                              BaseID.wrapping_add(StateIndex))
                               & 0x4 as libc::c_int as libc::c_uint != 0 {
                            CombineState |=
                                OrderButtons[OrdIndex as
                                                 usize].States[StateIndex as
                                                                   usize] as
                                    libc::c_uint
                        }
                        StateIndex = StateIndex.wrapping_add(1)
                    }
                    // set the final state
                    SetSecondaryState(OrderButtons[OrdIndex as usize].Order,
                                      (CombineState &
                                           OrderButtons[OrdIndex as
                                                            usize].StateMask)
                                          as SECONDARY_STATE);
                }
            }
            _ => { }
        }
        OrdIndex = OrdIndex.wrapping_add(1)
    };
}
unsafe extern "C" fn _intRefreshOrder() -> BOOL {
    // Is the Order screen up?
    if intMode as libc::c_uint == INT_ORDER as libc::c_int as libc::c_uint &&
           !widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD).is_null()
       {
        let mut Ret: BOOL = 0;
        NumSelectedDroids = 0 as libc::c_int as UWORD;
        //check for factory selected first
        if psSelectedFactory.is_null() {
            if BuildSelectedDroidList() == 0 {
                // no units selected - quit
                intRemoveOrder();
                return 1 as libc::c_int
            }
        }
        // if the orders havn't changed, just reset the state
		//if (CheckDroidOrderList())
        if CheckObjectOrderList() != 0 {
            Ret = intRefreshOrderButtons()
        } else {
            // Refresh it by re-adding it.
            Ret = intAddOrder(0 as *mut BASE_OBJECT);
            if Ret == 0 as libc::c_int { intMode = INT_NORMAL }
        }
        return Ret
    }
    return 1 as libc::c_int;
}
// Call to refresh the Order screen, ie when a droids boards it.
//
#[no_mangle]
pub unsafe extern "C" fn intRefreshOrder() -> BOOL {
    //	DBPRINTF(("intRefreshOrder\n"));
    return _intRefreshOrder();
}
// update already open order form
//changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
//BOOL intAddOrder(DROID *Droid);			// create and open order form
//changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
//BOOL intAddOrder(DROID *Droid)
#[no_mangle]
pub unsafe extern "C" fn intAddOrder(mut psObj: *mut BASE_OBJECT) -> BOOL {
    //changed to a BASE_OBJECT to accomodate the factories - AB 21/04/99
	//return _intAddOrder(Droid);
    return _intAddOrder(psObj);
}
#[no_mangle]
pub unsafe extern "C" fn intProcessOrder(mut id: UDWORD) {
    _intProcessOrder(id);
}
// Remove the droids order screen with animation.
//
#[no_mangle]
pub unsafe extern "C" fn intRemoveOrder() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    widgDelete(psWScreen, 8001 as libc::c_int as UDWORD);
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).pUserData = 0 as *mut libc::c_void;
        (*Form).disableChildren = 1 as libc::c_int;
        ClosingOrder = 1 as libc::c_int;
        OrderUp = 0 as libc::c_int;
        NumSelectedDroids = 0 as libc::c_int as UWORD;
        psSelectedFactory = 0 as *mut STRUCTURE
    };
}
// Remove the droids order screen without animation.
//
#[no_mangle]
pub unsafe extern "C" fn intRemoveOrderNoAnim() {
    widgDelete(psWScreen, 8001 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 8000 as libc::c_int as UDWORD);
    OrderUp = 0 as libc::c_int;
    NumSelectedDroids = 0 as libc::c_int as UWORD;
    psSelectedFactory = 0 as *mut STRUCTURE;
}
// Build a list of currently selected droids.
// Returns TRUE if droids were selected.
//
unsafe extern "C" fn BuildSelectedDroidList() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        //		if(psDroid->selected AND psDroid->sDisplay.frameNumber == currentGameFrame) {
        if (*psDroid).selected != 0 {
            if (NumSelectedDroids as libc::c_int) < 100 as libc::c_int {
                SelectedDroids[NumSelectedDroids as usize] = psDroid;
                NumSelectedDroids = NumSelectedDroids.wrapping_add(1)
            }
        }
        psDroid = (*psDroid).psNext
    }
    //	DBPRINTF(("%d droids selected\n",NumSelectedDroids));
    if NumSelectedDroids != 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
//static BOOL factorySelected(void);
//looks thru' the players' list of Structures to see if one is a factory and it is selected
/*static BOOL factorySelected(void)
{
    STRUCTURE *psStruct;

    for (psStruct = apsStructLists[selectedPlayer]; psStruct; psStruct = psStruct->psNext)
    {
        if (psStruct->selected AND StructIsFactory(psStruct))
        {
            //found one - set as one to use for the interface
            psSelectedFactory = psStruct;
            return TRUE;
        }
    }
    //obviously never found a factory
    return FALSE;
}*/
// Set the secondary order state for all currently selected droids. And Factory (if one selected)
// Returns TRUE if succesfull.
//
unsafe extern "C" fn SetSecondaryState(mut sec: SECONDARY_ORDER,
                                       mut State: SECONDARY_STATE) -> BOOL {
    let mut i: UWORD = 0;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < NumSelectedDroids as libc::c_int {
        if !SelectedDroids[i as usize].is_null() {
            //Only set the state if it's not a transporter.
            if (*SelectedDroids[i as usize]).droidType as libc::c_uint !=
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                if secondarySetState(SelectedDroids[i as usize], sec, State)
                       == 0 {
                    return 0 as libc::c_int
                }
            }
        }
        i = i.wrapping_add(1)
    }
    //set the Factory settings
    if !psSelectedFactory.is_null() {
        if setFactoryState(psSelectedFactory, sec, State) == 0 {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// Build a list of orders available for the selected group of droids.
//
unsafe extern "C" fn BuildDroidOrderList() -> BOOL {
    let mut OrdIndex: UWORD = 0;
    let mut i: UWORD = 0;
    let mut j: UWORD = 0;
    let mut Found: BOOL = 0;
    let mut Sorted: BOOL = 0;
    let mut Tmp: AVORDER = AVORDER{OrderIndex: 0, RefCount: 0,};
    NumAvailableOrders = 0 as libc::c_int as UWORD;
    j = 0 as libc::c_int as UWORD;
    while (j as libc::c_int) < NumSelectedDroids as libc::c_int {
        OrdIndex = 0 as libc::c_int as UWORD;
        while (OrdIndex as libc::c_int) < 11 as libc::c_int {
            // Is the order available?
            if secondarySupported(SelectedDroids[j as usize],
                                  OrderButtons[OrdIndex as usize].Order) != 0
               {
                if (NumAvailableOrders as libc::c_int) < 16 as libc::c_int {
                    // Have we already got this order?
                    Found = 0 as libc::c_int;
                    i = 0 as libc::c_int as UWORD;
                    while (i as libc::c_int) <
                              NumAvailableOrders as libc::c_int {
                        if AvailableOrders[i as usize].OrderIndex as
                               libc::c_int == OrdIndex as libc::c_int {
                            // Yes! Then increment it's reference count.
                            AvailableOrders[i as usize].RefCount =
                                AvailableOrders[i as
                                                    usize].RefCount.wrapping_add(1);
                            Found = 1 as libc::c_int
                        }
                        i = i.wrapping_add(1)
                    }
                    if Found == 0 {
                        // Not already got it so add it to the list of available orders.
                        AvailableOrders[NumAvailableOrders as
                                            usize].OrderIndex = OrdIndex;
                        AvailableOrders[NumAvailableOrders as usize].RefCount
                            = 1 as libc::c_int as UWORD;
                        NumAvailableOrders =
                            NumAvailableOrders.wrapping_add(1)
                    }
                }
            }
            OrdIndex = OrdIndex.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
    if NumAvailableOrders as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if NumAvailableOrders as libc::c_int > 1 as libc::c_int {
        loop 
             // Sort by Order index, A bubble sort? I know but it's only
		// a small list so what the hell.
             {
            Sorted = 1 as libc::c_int;
            i = 0 as libc::c_int as UWORD;
            while (i as libc::c_int) <
                      NumAvailableOrders as libc::c_int - 1 as libc::c_int {
                if AvailableOrders[i as usize].OrderIndex as libc::c_int >
                       AvailableOrders[(i as libc::c_int + 1 as libc::c_int)
                                           as usize].OrderIndex as libc::c_int
                   {
                    Tmp = AvailableOrders[i as usize];
                    AvailableOrders[i as usize] =
                        AvailableOrders[(i as libc::c_int + 1 as libc::c_int)
                                            as usize];
                    AvailableOrders[(i as libc::c_int + 1 as libc::c_int) as
                                        usize] = Tmp;
                    Sorted = 0 as libc::c_int
                }
                i = i.wrapping_add(1)
            }
            if !(Sorted == 0) { break ; }
        }
    }
    return 1 as libc::c_int;
}
// Build a list of orders available for the selected structure.
unsafe extern "C" fn BuildStructureOrderList(mut psStructure: *mut STRUCTURE)
 -> BOOL {
    //only valid for Factories (at the moment)
    if StructIsFactory(psStructure) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"BuildStructureOrderList: structure is not a factory\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"intorder.c\x00" as *const u8 as *const libc::c_char,
                  1232 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"BuildStructureOrderList\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //this can be hard-coded!
    AvailableOrders[0 as libc::c_int as usize].OrderIndex =
        0 as libc::c_int as UWORD; //DSO_ATTACK_RANGE;
    AvailableOrders[0 as libc::c_int as usize].RefCount =
        1 as libc::c_int as UWORD; //DSO_REPAIR_LEVEL;
    AvailableOrders[1 as libc::c_int as usize].OrderIndex =
        1 as libc::c_int as UWORD; //DSO_ATTACK_LEVEL;
    AvailableOrders[1 as libc::c_int as usize].RefCount =
        1 as libc::c_int as UWORD; //DSO_HALTTYPE;
    AvailableOrders[2 as libc::c_int as usize].OrderIndex =
        2 as libc::c_int as UWORD;
    AvailableOrders[2 as libc::c_int as usize].RefCount =
        1 as libc::c_int as UWORD;
    AvailableOrders[3 as libc::c_int as usize].OrderIndex =
        5 as libc::c_int as UWORD;
    AvailableOrders[3 as libc::c_int as usize].RefCount =
        1 as libc::c_int as UWORD;
    NumAvailableOrders = 4 as libc::c_int as UWORD;
    return 1 as libc::c_int;
}
//works on factories now as well - AB 21/04/99
//static BOOL CheckDroidOrderList(void);
// check whether the order list has changed
//works on factories now as well - AB 21/04/99
//static BOOL CheckDroidOrderList(void)
unsafe extern "C" fn CheckObjectOrderList() -> BOOL {
    let mut OrdIndex: UWORD = 0;
    let mut i: UWORD = 0;
    let mut j: UWORD = 0;
    let mut Found: BOOL = 0;
    let mut Sorted: BOOL = 0;
    let mut Tmp: AVORDER = AVORDER{OrderIndex: 0, RefCount: 0,};
    let mut NumNewOrders: UWORD = 0;
    let mut NewAvailableOrders: [AVORDER; 16] =
        [AVORDER{OrderIndex: 0, RefCount: 0,}; 16];
    NumNewOrders = 0 as libc::c_int as UWORD;
    //added for factories - AB 21/04/99
    if !psSelectedFactory.is_null() {
        //only valid for Factories (at the moment)
        if StructIsFactory(psSelectedFactory) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"CheckObjectOrderList: structure is not a factory\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"intorder.c\x00" as *const u8 as *const libc::c_char,
                      1273 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"CheckObjectOrderList\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        //this can be hard-coded!
        NewAvailableOrders[0 as libc::c_int as usize].OrderIndex =
            0 as libc::c_int as UWORD; //DSO_ATTACK_RANGE;
        NewAvailableOrders[0 as libc::c_int as usize].RefCount =
            1 as libc::c_int as UWORD; //DSO_REPAIR_LEVEL;
        NewAvailableOrders[1 as libc::c_int as usize].OrderIndex =
            1 as libc::c_int as UWORD; //DSO_ATTACK_LEVEL;
        NewAvailableOrders[1 as libc::c_int as usize].RefCount =
            1 as libc::c_int as UWORD; //DSO_HALTTYPE;
        NewAvailableOrders[2 as libc::c_int as usize].OrderIndex =
            2 as libc::c_int as UWORD;
        NewAvailableOrders[2 as libc::c_int as usize].RefCount =
            1 as libc::c_int as UWORD;
        NewAvailableOrders[3 as libc::c_int as usize].OrderIndex =
            5 as libc::c_int as UWORD;
        NewAvailableOrders[3 as libc::c_int as usize].RefCount =
            1 as libc::c_int as UWORD;
        NumNewOrders = 4 as libc::c_int as UWORD;
        if NumNewOrders as libc::c_int != NumAvailableOrders as libc::c_int {
            return 0 as libc::c_int
        }
    } else {
        j = 0 as libc::c_int as UWORD;
        while (j as libc::c_int) < NumSelectedDroids as libc::c_int {
            OrdIndex = 0 as libc::c_int as UWORD;
            while (OrdIndex as libc::c_int) < 11 as libc::c_int {
                // Is the order available?
                if secondarySupported(SelectedDroids[j as usize],
                                      OrderButtons[OrdIndex as usize].Order)
                       != 0 {
                    if (NumNewOrders as libc::c_int) < 16 as libc::c_int {
                        // Have we already got this order?
                        Found = 0 as libc::c_int;
                        i = 0 as libc::c_int as UWORD;
                        while (i as libc::c_int) < NumNewOrders as libc::c_int
                              {
                            if NewAvailableOrders[i as usize].OrderIndex as
                                   libc::c_int == OrdIndex as libc::c_int {
                                // Yes! Then increment it's reference count.
                                NewAvailableOrders[i as usize].RefCount =
                                    NewAvailableOrders[i as
                                                           usize].RefCount.wrapping_add(1);
                                Found = 1 as libc::c_int
                            }
                            i = i.wrapping_add(1)
                        }
                        if Found == 0 {
                            // Not already got it so add it to the list of available orders.
                            NewAvailableOrders[NumNewOrders as
                                                   usize].OrderIndex =
                                OrdIndex;
                            NewAvailableOrders[NumNewOrders as usize].RefCount
                                = 1 as libc::c_int as UWORD;
                            NumNewOrders = NumNewOrders.wrapping_add(1)
                        }
                    }
                }
                OrdIndex = OrdIndex.wrapping_add(1)
            }
            j = j.wrapping_add(1)
        }
        if NumNewOrders as libc::c_int != NumAvailableOrders as libc::c_int {
            return 0 as libc::c_int
        }
        if NumNewOrders as libc::c_int > 1 as libc::c_int {
            loop 
                 // Sort by Order index, A bubble sort? I know but it's only
		    // a small list so what the hell.
                 {
                Sorted = 1 as libc::c_int;
                i = 0 as libc::c_int as UWORD;
                while (i as libc::c_int) <
                          NumNewOrders as libc::c_int - 1 as libc::c_int {
                    if NewAvailableOrders[i as usize].OrderIndex as
                           libc::c_int >
                           NewAvailableOrders[(i as libc::c_int +
                                                   1 as libc::c_int) as
                                                  usize].OrderIndex as
                               libc::c_int {
                        Tmp = NewAvailableOrders[i as usize];
                        NewAvailableOrders[i as usize] =
                            NewAvailableOrders[(i as libc::c_int +
                                                    1 as libc::c_int) as
                                                   usize];
                        NewAvailableOrders[(i as libc::c_int +
                                                1 as libc::c_int) as usize] =
                            Tmp;
                        Sorted = 0 as libc::c_int
                    }
                    i = i.wrapping_add(1)
                }
                if !(Sorted == 0) { break ; }
            }
        }
    }
    // now check that all the orders are the same
    i = 0 as libc::c_int as UWORD; //,k;
    while (i as libc::c_int) < NumNewOrders as libc::c_int {
        if NewAvailableOrders[i as usize].OrderIndex as libc::c_int !=
               AvailableOrders[i as usize].OrderIndex as libc::c_int {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn intRefreshOrderButtons() -> BOOL {
    let mut State: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut i: UWORD = 0;
    let mut j: UWORD = 0;
    let mut OrdIndex: UWORD = 0;
    let mut NumButs: UWORD = 0;
    let mut id: UDWORD = 0;
    j = 0 as libc::c_int as UWORD;
    while (j as libc::c_int) < NumAvailableOrders as libc::c_int &&
              (j as libc::c_int) < 11 as libc::c_int {
        OrdIndex = AvailableOrders[j as usize].OrderIndex;
        // Get current order state.
   		//secondaryGetState(SelectedDroids[0], OrderButtons[OrdIndex].Order, &State);
        State =
            GetSecondaryStates(OrderButtons[OrdIndex as usize].Order) as
                SECONDARY_STATE;
        // Get number of buttons.
        NumButs = OrderButtons[OrdIndex as usize].NumButs;
        // Set actual number of buttons.
        OrderButtons[OrdIndex as usize].AcNumButs = NumButs;
        // Handle special case for factory -> command droid assignment buttons.
        match OrderButtons[OrdIndex as usize].Class as libc::c_uint {
            1 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             0 as libc::c_int as UWORD)
            }
            2 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             1 as libc::c_int as UWORD)
            }
            3 => {
                NumButs =
                    countAssignableFactories(selectedPlayer as UBYTE,
                                             2 as libc::c_int as UWORD)
            }
            _ => { }
        }
        id = OrderButtons[OrdIndex as usize].ButBaseID;
        i = 0 as libc::c_int as UWORD;
        while (i as libc::c_int) <
                  OrderButtons[OrdIndex as usize].AcNumButs as libc::c_int {
            // Set the state for the button.
            match OrderButtons[OrdIndex as usize].ButType as libc::c_uint {
                0 | 1 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].StateMask ==
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, id,
                                           0 as libc::c_int as UDWORD);
                    }
                }
                2 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].StateMask ==
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                    } else if i as libc::c_int == 0 as libc::c_int {
                        widgSetButtonState(psWScreen, id,
                                           0 as libc::c_int as UDWORD);
                    } else {
                        widgSetButtonState(psWScreen, id,
                                           0x1 as libc::c_int as UDWORD);
                    }
                }
                3 => {
                    if State as libc::c_uint &
                           OrderButtons[OrdIndex as usize].States[i as usize]
                               as UDWORD != 0 {
                        widgSetButtonState(psWScreen, id,
                                           0x4 as libc::c_int as UDWORD);
                    }
                }
                _ => { }
            }
            id = id.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// return the state for an order for all the units selected
// if there are multiple states then don't return a state
unsafe extern "C" fn GetSecondaryStates(mut sec: SECONDARY_ORDER) -> SDWORD {
    let mut i: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut currState: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut bFirst: BOOL = 0;
    state = 0 as SECONDARY_STATE;
    bFirst = 1 as libc::c_int;
    //handle a factory being selected - AB 22/04/99
    if !psSelectedFactory.is_null() {
        if getFactoryState(psSelectedFactory, sec, &mut currState) != 0 {
            state = currState
        }
    } else {
        //droids
        i = 0 as libc::c_int;
        while i < NumSelectedDroids as libc::c_int {
            if secondaryGetState(SelectedDroids[i as usize], sec,
                                 &mut currState) != 0 {
                if bFirst != 0 {
                    state = currState;
                    bFirst = 0 as libc::c_int
                } else if state as libc::c_uint != currState as libc::c_uint {
                    state = 0 as SECONDARY_STATE
                }
            }
            i += 1
        }
    }
    return state as SDWORD;
}
unsafe extern "C" fn GetImageWidth(mut ImageFile: *mut IMAGEFILE,
                                   mut ImageID: UDWORD) -> UDWORD {
    return iV_GetImageWidth(ImageFile, ImageID as UWORD) as UDWORD;
}
unsafe extern "C" fn GetImageHeight(mut ImageFile: *mut IMAGEFILE,
                                    mut ImageID: UDWORD) -> UDWORD {
    return iV_GetImageHeight(ImageFile, ImageID as UWORD) as UDWORD;
}
//new function added to bring up the RMB order form for Factories as well as droids
//new function added to bring up the RMB order form for Factories as well as droids
#[no_mangle]
pub unsafe extern "C" fn intAddFactoryOrder(mut psStructure: *mut STRUCTURE) {
    if OrderUp == 0 {
        intResetScreen(0 as libc::c_int);
        intAddOrder(psStructure as *mut BASE_OBJECT);
        intMode = INT_ORDER
    } else { intAddOrder(psStructure as *mut BASE_OBJECT); };
}
