use ::libc;
extern "C" {
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
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Return the length of S.  */
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
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* This returns true if the mouse key went from being up to being down this frame */
    #[no_mangle]
    fn mousePressed(code: MOUSE_KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being down to being up this frame */
    #[no_mangle]
    fn mouseReleased(code: MOUSE_KEY_CODE) -> BOOL;
    /* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
    #[no_mangle]
    fn heapCreate(ppsHeap: *mut *mut OBJ_HEAP, size: UDWORD, init: UDWORD,
                  ext: UDWORD) -> BOOL;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Destroy a heap and release all the memory associated with it */
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    /* The Current screen size and bit depth */
    #[no_mangle]
    static mut screenWidth: UDWORD;
    #[no_mangle]
    static mut screenHeight: UDWORD;
    /*
 * Form.h
 *
 * Definitions for the form functions.
 *
 */
    /* The widget heaps */
    #[no_mangle]
    static mut psFormHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut psCFormHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut psTFormHeap: *mut OBJ_HEAP;
    #[no_mangle]
    fn formClearFlash(psWidget: *mut W_FORM);
    /* Create a form widget data structure */
    #[no_mangle]
    fn formCreate(ppsWidget: *mut *mut W_FORM, psInit: *mut W_FORMINIT)
     -> BOOL;
    /* Free the memory used by a form */
    #[no_mangle]
    fn formFree(psWidget: *mut W_FORM);
    /* Add a widget to a form */
    #[no_mangle]
    fn formAddWidget(psForm: *mut W_FORM, psWidget: *mut WIDGET,
                     psInit: *mut W_INIT) -> BOOL;
    /* Initialise a form widget before running it */
    #[no_mangle]
    fn formInitialise(psWidget: *mut W_FORM);
    /* Return the widgets currently displayed by a form */
    #[no_mangle]
    fn formGetWidgets(psWidget: *mut W_FORM) -> *mut WIDGET;
    /* Return the origin on the form from which button locations are calculated */
    #[no_mangle]
    fn formGetOrigin(psWidget: *mut W_FORM, pXOrigin: *mut SDWORD,
                     pYOrigin: *mut SDWORD);
    /* Initialise the formGetAllWidgets function */
    #[no_mangle]
    fn formInitGetAllWidgets(psWidget: *mut W_FORM,
                             psCtrl: *mut W_FORMGETALL);
    /* Repeated calls to this function will return widget lists
 * until all widgets in a form have been returned.
 * When a NULL list is returned, all widgets have been seen.
 */
    #[no_mangle]
    fn formGetAllWidgets(psCtrl: *mut W_FORMGETALL) -> *mut WIDGET;
    /* Get the button state of a click form */
    #[no_mangle]
    fn formGetClickState(psForm: *mut W_CLICKFORM) -> UDWORD;
    #[no_mangle]
    fn formSetFlash(psWidget: *mut W_FORM);
    /* Set the button state of a click form */
    #[no_mangle]
    fn formSetClickState(psForm: *mut W_CLICKFORM, state: UDWORD);
    /* Run a form widget */
    #[no_mangle]
    fn formRun(psWidget: *mut W_FORM, psContext: *mut W_CONTEXT);
    /* Respond to a mouse click */
    #[no_mangle]
    fn formClicked(psWidget: *mut W_FORM, key: UDWORD);
    /* Respond to a mouse form up */
    #[no_mangle]
    fn formReleased(psWidget: *mut W_FORM, key: UDWORD,
                    psContext: *mut W_CONTEXT);
    /* Respond to a mouse moving over a form */
    #[no_mangle]
    fn formHiLite(psWidget: *mut W_FORM, psContext: *mut W_CONTEXT);
    /* Respond to the mouse moving off a form */
    #[no_mangle]
    fn formHiLiteLost(psWidget: *mut W_FORM, psContext: *mut W_CONTEXT);
    /*
 * Label.h
 *
 * Definitions for the label widget.
 */
    /* The widget heaps */
    #[no_mangle]
    static mut psLabHeap: *mut OBJ_HEAP;
    /* Create a button widget data structure */
    #[no_mangle]
    fn labelCreate(ppsWidget: *mut *mut W_LABEL, psInit: *mut W_LABINIT)
     -> BOOL;
    /* Free the memory used by a button */
    #[no_mangle]
    fn labelFree(psWidget: *mut W_LABEL);
    /* Respond to a mouse moving over a label */
    #[no_mangle]
    fn labelHiLite(psWidget: *mut W_LABEL, psContext: *mut W_CONTEXT);
    /* Respond to the mouse moving off a label */
    #[no_mangle]
    fn labelHiLiteLost(psWidget: *mut W_LABEL);
    /*
 * Button.h
 *
 * Definitions for edit box functions.
 */
    /* The widget heap */
    #[no_mangle]
    static mut psButHeap: *mut OBJ_HEAP;
    /* Create a button widget data structure */
    #[no_mangle]
    fn buttonCreate(ppsWidget: *mut *mut W_BUTTON, psInit: *mut W_BUTINIT)
     -> BOOL;
    /* Free the memory used by a button */
    #[no_mangle]
    fn buttonFree(psWidget: *mut W_BUTTON);
    /* Initialise a button widget before running it */
    #[no_mangle]
    fn buttonInitialise(psWidget: *mut W_BUTTON);
    /* Run a button widget */
    #[no_mangle]
    fn buttonRun(psWidget: *mut W_BUTTON);
    /* Respond to a mouse click */
    #[no_mangle]
    fn buttonClicked(psWidget: *mut W_BUTTON, key: UDWORD);
    /* Respond to a mouse button up */
    #[no_mangle]
    fn buttonReleased(psWidget: *mut W_BUTTON, key: UDWORD);
    /* Respond to a mouse moving over a button */
    #[no_mangle]
    fn buttonHiLite(psWidget: *mut W_BUTTON, psContext: *mut W_CONTEXT);
    /* Respond to the mouse moving off a button */
    #[no_mangle]
    fn buttonHiLiteLost(psWidget: *mut W_BUTTON);
    /* Get a button's state */
    #[no_mangle]
    fn buttonGetState(psButton: *mut W_BUTTON) -> UDWORD;
    /* Set a button's state */
    #[no_mangle]
    fn buttonSetState(psWidget: *mut W_BUTTON, state: UDWORD);
    #[no_mangle]
    fn buttonSetFlash(psButton: *mut W_BUTTON);
    #[no_mangle]
    fn buttonClearFlash(psButton: *mut W_BUTTON);
    /*
 * EditBox.h
 *
 * Definitions for the edit box functions.
 */
    /* The widget heap */
    #[no_mangle]
    static mut psEdbHeap: *mut OBJ_HEAP;
    /* Create an edit box widget data structure */
    #[no_mangle]
    fn editBoxCreate(ppsWidget: *mut *mut W_EDITBOX, psInit: *mut W_EDBINIT)
     -> BOOL;
    /* Free the memory used by an edit box */
    #[no_mangle]
    fn editBoxFree(psWidget: *mut W_EDITBOX);
    /* Initialise an edit box widget */
    #[no_mangle]
    fn editBoxInitialise(psWidget: *mut W_EDITBOX);
    /* Set the current string for the edit box */
    #[no_mangle]
    fn editBoxSetString(psWidget: *mut W_EDITBOX, pText: *mut STRING);
    /* Respond to loss of focus */
    #[no_mangle]
    fn editBoxFocusLost(psWidget: *mut W_EDITBOX);
    /* Run an edit box widget */
    #[no_mangle]
    fn editBoxRun(psWidget: *mut W_EDITBOX, psContext: *mut W_CONTEXT);
    /* Respond to a mouse click */
    #[no_mangle]
    fn editBoxClicked(psWidget: *mut W_EDITBOX, psContext: *mut W_CONTEXT);
    /* Respond to a mouse button up */
    #[no_mangle]
    fn editBoxReleased(psWidget: *mut W_EDITBOX);
    /* Respond to a mouse moving over an edit box */
    #[no_mangle]
    fn editBoxHiLite(psWidget: *mut W_EDITBOX);
    /* Respond to the mouse moving off an edit box */
    #[no_mangle]
    fn editBoxHiLiteLost(psWidget: *mut W_EDITBOX);
    /* set state of edit box */
    #[no_mangle]
    fn editBoxSetState(psEditBox: *mut W_EDITBOX, state: UDWORD);
    /*
 * Bar.h
 *
 * Definitions for Bar Graph functions.
 */
    /* The widget heap */
    #[no_mangle]
    static mut psBarHeap: *mut OBJ_HEAP;
    /* Create a barGraph widget data structure */
    #[no_mangle]
    fn barGraphCreate(ppsWidget: *mut *mut W_BARGRAPH, psInit: *mut W_BARINIT)
     -> BOOL;
    /* Free the memory used by a barGraph */
    #[no_mangle]
    fn barGraphFree(psWidget: *mut W_BARGRAPH);
    /* Respond to a mouse moving over a barGraph */
    #[no_mangle]
    fn barGraphHiLite(psWidget: *mut W_BARGRAPH, psContext: *mut W_CONTEXT);
    /* Respond to the mouse moving off a barGraph */
    #[no_mangle]
    fn barGraphHiLiteLost(psWidget: *mut W_BARGRAPH);
    /*
 * Slider.h
 *
 * Slider bar interface definitions.
 */
    /* The widget heaps */
    #[no_mangle]
    static mut psSldHeap: *mut OBJ_HEAP;
    /* Create a slider widget data structure */
    #[no_mangle]
    fn sliderCreate(ppsWidget: *mut *mut W_SLIDER, psInit: *mut W_SLDINIT)
     -> BOOL;
    /* Free the memory used by a slider */
    #[no_mangle]
    fn sliderFree(psWidget: *mut W_SLIDER);
    /* Initialise a slider widget before running it */
    #[no_mangle]
    fn sliderInitialise(psWidget: *mut W_SLIDER);
    /* Run a slider widget */
    #[no_mangle]
    fn sliderRun(psWidget: *mut W_SLIDER, psContext: *mut W_CONTEXT);
    /* Respond to a mouse click */
    #[no_mangle]
    fn sliderClicked(psWidget: *mut W_SLIDER, psContext: *mut W_CONTEXT);
    /* Respond to a mouse up */
    #[no_mangle]
    fn sliderReleased(psWidget: *mut W_SLIDER);
    /* Respond to a mouse moving over a slider */
    #[no_mangle]
    fn sliderHiLite(psWidget: *mut W_SLIDER);
    /* Respond to the mouse moving off a slider */
    #[no_mangle]
    fn sliderHiLiteLost(psWidget: *mut W_SLIDER);
    /*
 * Tip.h
 *
 * Interface to the tool tip display module
 *
 */
    /* Initialise the tool tip module */
    #[no_mangle]
    fn tipInitialise();
    /* Update and possibly display the tip */
    #[no_mangle]
    fn tipDisplay();
}
pub type size_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_colourdef {
    pub red: UBYTE,
    pub green: UBYTE,
    pub blue: UBYTE,
    pub alpha: UBYTE,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Control whether the primary or secondary buttons work on a clickable form */
// Primary works by default - this turns it off
/* *********** Label styles ***************/
// Plain text only label
// Align the text at the left of the box
// Center the text
// Align the text at the right of the box
/* *********** Button styles **************/
// Plain button (text with a box around it)
/* Disable movement on a button */
/* Control whether the primary or secondary buttons work on a button */
// Primary works by default - this turns it off
// text only buttons. centre the text?
/* ********** Edit Box styles *************/
// Plain edit box (text with a box around it)
// Disabled. Displayed but never gets focus.
/* ********** Bar Graph styles ************/
// Plain bar graph
// Bar graph with a trough showing empty percentage
// Double bar graph, one on top of other
/* ********** Slider styles ***************/
// Plain slider
/* **********************************************************************************/
/* Generic widget colour */
pub type W_COLOURDEF = _w_colourdef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_init {
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
}
/* Basic initialisation entries common to all widgets */
/* ID number of form to put widget on */
/* ID == 0 specifies the default form for the screen */
/* Which major and minor tab to put the widget */
/* on for a tabbed form */
/* Unique id number (chosen by user) */
/* widget style */
/* screen location */
/* widget size */
/* Optional display function */
/* Optional callback function */
/* Optional user data pointer */
/* User data (if any) */
/* The basic initialisation structure */
pub type W_INIT = _w_init;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_barinit {
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
    pub size: UWORD,
    pub minorSize: UWORD,
    pub iRange: UWORD,
    pub sCol: W_COLOURDEF,
    pub sMinorCol: W_COLOURDEF,
    pub pTip: *mut STRING,
}
/* The basic init entries */
// initial contents of the edit box
//	PROP_FONT	*psFont;		// edit box font
// ID of the IVIS font to use for this widget.
// Optional callback to display the form.
// Optional callback to display a string.
/* Orientation flags for the bar graph */
// Bar graph fills from left to right
// Bar graph fills from right to left
// Bar graph fills from top to bottom
// Bar graph fills from bottom to top
/* Bar Graph initialisation structure */
pub type W_BARINIT = _w_barinit;
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
// Orientation of the bar on the widget
// Initial percentage of the graph that is filled
// Percentage of second bar graph if there is one
// Maximum range
// Bar colour
// Minor bar colour
// Tool tip text
/* Orientation of the slider */
// Slider is horizontal and starts at left
// Slider is horizontal and starts at the right
// Slider is vertical and starts at the top
// Slider is vertical and starts at the bottom
/* Slider initialisation structure */
pub type W_SLDINIT = _w_sldinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_heapinit {
    pub barInit: UDWORD,
    pub barExt: UDWORD,
    pub butInit: UDWORD,
    pub butExt: UDWORD,
    pub edbInit: UDWORD,
    pub edbExt: UDWORD,
    pub formInit: UDWORD,
    pub formExt: UDWORD,
    pub cFormInit: UDWORD,
    pub cFormExt: UDWORD,
    pub tFormInit: UDWORD,
    pub tFormExt: UDWORD,
    pub labInit: UDWORD,
    pub labExt: UDWORD,
    pub sldInit: UDWORD,
    pub sldExt: UDWORD,
}
/* The basic init entries */
// Orientation of the slider
// Number of stops on the slider
// Size of the bar
// Initial position of the slider bar
// Tip string
/* The maximum value for bar graph size */
/* Structure to specify the heap sizes for the widget library */
pub type W_HEAPINIT = _w_heapinit;
// bar graph heap
// button heap
// edit box heap
// form heap
// clicable form heap
// tab form heap
// label heap
// slider heap
/* Slider state */
// Slider is being dragged
// Slider is hilited
pub type W_SLIDER = _w_slider;
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
/* The common widget data */
// The orientation of the slider
// Number of stop positions on the slider
// Thickness of slider bar
// Current stop position of the slider
// Slider state
// Tool tip
// label states.
// label is hilited
pub type W_LABEL = _w_label;
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
/* The common widget data */
// The current button state
// Text on the label
//	PROP_FONT	*psFont;				// Font for the label
// The tool tip for the button
/* The tabbed form data structure */
pub type W_TABFORM = _w_tabform;
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
/* Information for a major tab */
pub type W_MAJORTAB = _w_majortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Store which was the last selected minor tab
// Minor tab information
/* Information for a minor tab */
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
/* Graphics data for the tab will go here */
// Widgets on the tab
// Tool tip
// Button is locked but clickable
// Button flashing is enabled
// Button is flashing
/* The clickable form data structure */
pub type W_CLICKFORM = _w_clickform;
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
/* The common form data */
// Button state of the form
// Tip for the form
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
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
/* The common form data */
/* Edit Box states */
// No editing is going on
// Insertion editing
// Overwrite editing
// 
//
// disable button from selection
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
/* The common widget data */
// The current edit box state
// The text in the edit box
//	PROP_FONT	*psFont;					// The font for the edit box
// The insertion point in the buffer
// Where in the string appears at the far left of the box
// The number of characters appearing in the box
// The pixel width of the characters in the box
// Optional callback to display the edit box background.
// Optional callback to display a string.
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
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
pub type W_BARGRAPH = _w_bargraph;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_bargraph {
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
    pub barPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub iRange: UWORD,
    pub iValue: UWORD,
    pub majorCol: UBYTE,
    pub minorCol: UBYTE,
    pub pTip: *mut STRING,
}
/* The common widget data */
// The current button state
// The text for the button
// The tool tip for the button
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
//	PROP_FONT	*psFont;			// button font
/* The common widget data */
// Orientation of the bar on the widget
// Percentage of the main bar that is filled
// Percentage of the minor bar if there is one
// Maximum range
// Current value
// Colour for the major bar
// Colour for the minor bar
// The tool tip for the graph
/* Variables for the formGetAllWidgets functions */
pub type W_FORMGETALL = _w_formgetall;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_formgetall {
    pub psGAWList: *mut WIDGET,
    pub psGAWForm: *mut W_TABFORM,
    pub psGAWMajor: *mut W_MAJORTAB,
    pub GAWMajor: UDWORD,
    pub GAWMinor: UDWORD,
}
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
pub type W_CONTEXT = _w_context;
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
/*
 * Widget.c
 *
 * The main interface functions to the widget library
 */
/* The initial and extension number of strings to allocate in the string heap */
/* the widget to be returned by widgRunScreen */
static mut psRetWidget: *mut WIDGET = 0 as *const WIDGET as *mut WIDGET;
static mut bWidgetsActive: BOOL = 1 as libc::c_int;
/* The widget the mouse is over this update */
static mut psMouseOverWidget: *mut WIDGET = 0 as *const WIDGET as *mut WIDGET;
static mut pressed: UDWORD = 0;
static mut released: UDWORD = 0;
static mut AudioCallback: WIDGET_AUDIOCALLBACK = None;
static mut HilightAudioID: SWORD = -(1 as libc::c_int) as SWORD;
static mut ClickedAudioID: SWORD = -(1 as libc::c_int) as SWORD;
/* The heap for widget strings */
static mut psStrHeap: *mut OBJ_HEAP = 0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Buffer to return strings in */
static mut aStringRetBuffer: [STRING; 80] = [0; 80];
/* Initialise the widget module */
/* Initialise the widget module */
#[no_mangle]
pub unsafe extern "C" fn widgInitialise(mut psInit: *mut W_HEAPINIT) -> BOOL {
    tipInitialise();
    // Create the widget heaps
    if heapCreate(&mut psBarHeap,
                  ::std::mem::size_of::<W_BARGRAPH>() as libc::c_ulong,
                  (*psInit).barInit, (*psInit).barExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psButHeap,
                  ::std::mem::size_of::<W_BUTTON>() as libc::c_ulong,
                  (*psInit).butInit, (*psInit).butExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psEdbHeap,
                  ::std::mem::size_of::<W_EDITBOX>() as libc::c_ulong,
                  (*psInit).edbInit, (*psInit).edbExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psFormHeap,
                  ::std::mem::size_of::<W_FORM>() as libc::c_ulong,
                  (*psInit).formInit, (*psInit).formExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psCFormHeap,
                  ::std::mem::size_of::<W_CLICKFORM>() as libc::c_ulong,
                  (*psInit).cFormInit, (*psInit).cFormExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psTFormHeap,
                  ::std::mem::size_of::<W_TABFORM>() as libc::c_ulong,
                  (*psInit).tFormInit, (*psInit).tFormExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psLabHeap,
                  ::std::mem::size_of::<W_LABEL>() as libc::c_ulong,
                  (*psInit).labInit, (*psInit).labExt) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psSldHeap,
                  ::std::mem::size_of::<W_SLIDER>() as libc::c_ulong,
                  (*psInit).sldInit, (*psInit).sldExt) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Reset the widget module */
// Reset the widgets.
//
#[no_mangle]
pub unsafe extern "C" fn widgReset() { tipInitialise(); }
/* Shut down the widget module */
/* Shut down the widget module */
#[no_mangle]
pub unsafe extern "C" fn widgShutDown() {
    heapDestroy(psBarHeap);
    heapDestroy(psButHeap);
    heapDestroy(psEdbHeap);
    heapDestroy(psFormHeap);
    heapDestroy(psCFormHeap);
    heapDestroy(psTFormHeap);
    heapDestroy(psLabHeap);
    heapDestroy(psSldHeap);
}
/* Get a string from the string heap */
/* Get a string from the string heap */
#[no_mangle]
pub unsafe extern "C" fn widgAllocString(mut ppStr: *mut *mut STRING)
 -> BOOL {
    if heapAlloc(psStrHeap,
                 ppStr as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Copy one string to another
 * The string to copy will be truncated if it is longer than WIDG_MAXSTR.
 */
/* Copy one string to another
 * The string to copy will be truncated if it is longer than WIDG_MAXSTR.
 */
#[no_mangle]
pub unsafe extern "C" fn widgCopyString(mut pDest: *mut STRING,
                                        mut pSrc: *mut STRING) {
    /* See if we need to clip the string, then copy */
    if strlen(pSrc) >= 80 as libc::c_int as libc::c_uint {
        memcpy(pDest as *mut libc::c_void, pSrc as *const libc::c_void,
               (80 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
        *pDest.offset(80 as libc::c_int as
                          isize).offset(-(1 as libc::c_int as isize)) =
            0 as libc::c_int as STRING
    } else { strcpy(pDest, pSrc); };
}
/* Get a string from the heap and copy in some data.
 * The string to copy will be truncated if it is too long.
 */
/* Get a string from the heap and copy in some data.
 * The string to copy will be truncated if it is too long.
 */
#[no_mangle]
pub unsafe extern "C" fn widgAllocCopyString(mut ppDest: *mut *mut STRING,
                                             mut pSrc: *mut STRING) -> BOOL {
    if heapAlloc(psStrHeap,
                 ppDest as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        *ppDest = 0 as *mut STRING;
        return 0 as libc::c_int
    }
    widgCopyString(*ppDest, pSrc);
    return 1 as libc::c_int;
}
/* Return a string to the string heap */
/* Return a string to the string heap */
#[no_mangle]
pub unsafe extern "C" fn widgFreeString(mut pStr: *mut STRING) {
    heapFree(psStrHeap, pStr as *mut libc::c_void);
}
/* Create an empty widget screen */
/* Create an empty widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgCreateScreen(mut ppsScreen: *mut *mut W_SCREEN)
 -> BOOL {
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    let mut sInit: W_FORMINIT =
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
    *ppsScreen =
        memMallocRelease(::std::mem::size_of::<W_SCREEN>() as libc::c_ulong)
            as *mut W_SCREEN;
    if (*ppsScreen).is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Out of memory\x00" as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  204 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"widgCreateScreen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    memset(&mut sInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sInit.id = 0 as libc::c_int as UDWORD;
    sInit.style = (0 as libc::c_int | 2 as libc::c_int) as UDWORD;
    sInit.x = 0 as libc::c_int as SWORD;
    sInit.y = 0 as libc::c_int as SWORD;
    sInit.width =
        screenWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as UWORD;
    sInit.height =
        screenHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as UWORD;
    if formCreate(&mut psForm, &mut sInit) == 0 { return 0 as libc::c_int }
    (**ppsScreen).psForm = psForm as *mut WIDGET;
    (**ppsScreen).psFocus = 0 as *mut WIDGET;
    (**ppsScreen).TipFontID = 0 as libc::c_int;
    return 1 as libc::c_int;
}
/* Release a list of widgets */
/* Release a list of widgets */
#[no_mangle]
pub unsafe extern "C" fn widgReleaseWidgetList(mut psWidgets: *mut WIDGET) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut psNext: *mut WIDGET = 0 as *mut WIDGET;
    psCurr = psWidgets;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        match (*psCurr).type_0 as libc::c_uint {
            0 => { formFree(psCurr as *mut W_FORM); }
            1 => { labelFree(psCurr as *mut W_LABEL); }
            2 => { buttonFree(psCurr as *mut W_BUTTON); }
            3 => { editBoxFree(psCurr as *mut W_EDITBOX); }
            4 => { barGraphFree(psCurr as *mut W_BARGRAPH); }
            5 => { sliderFree(psCurr as *mut W_SLIDER); }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgReleaseWidgetList: Unknown widget type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          257 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"widgReleaseWidgetList\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        psCurr = psNext
    };
}
/* Release a screen and all its associated data */
/* Release a screen and all its associated data */
#[no_mangle]
pub unsafe extern "C" fn widgReleaseScreen(mut psScreen: *mut W_SCREEN) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgReleaseScreen: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              268 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"widgReleaseScreen\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    formFree((*psScreen).psForm as *mut W_FORM);
    memFreeRelease(psScreen as *mut libc::c_void);
    psScreen = 0 as *mut W_SCREEN;
}
/* Release a widget */
#[no_mangle]
pub unsafe extern "C" fn widgRelease(mut psWidget: *mut WIDGET) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formFree(psWidget as *mut W_FORM); }
        1 => { labelFree(psWidget as *mut W_LABEL); }
        2 => { buttonFree(psWidget as *mut W_BUTTON); }
        3 => { editBoxFree(psWidget as *mut W_EDITBOX); }
        4 => { barGraphFree(psWidget as *mut W_BARGRAPH); }
        5 => { sliderFree(psWidget as *mut W_SLIDER); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgRelease: Unknown widget type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      300 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"widgRelease\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Check whether an ID has been used on a form */
unsafe extern "C" fn widgCheckIDForm(mut psForm: *mut W_FORM, mut id: UDWORD)
 -> BOOL {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut sGetAll: W_FORMGETALL =
        W_FORMGETALL{psGAWList: 0 as *mut WIDGET,
                     psGAWForm: 0 as *mut W_TABFORM,
                     psGAWMajor: 0 as *mut W_MAJORTAB,
                     GAWMajor: 0,
                     GAWMinor: 0,};
    /* Check the widgets on the form */
    formInitGetAllWidgets(psForm, &mut sGetAll);
    psCurr = formGetAllWidgets(&mut sGetAll);
    while !psCurr.is_null() {
        if (*psCurr).id == id { return 1 as libc::c_int }
        if (*psCurr).type_0 as libc::c_uint ==
               WIDG_FORM as libc::c_int as libc::c_uint {
            /* Another form so recurse */
            if widgCheckIDForm(psCurr as *mut W_FORM, id) != 0 {
                return 1 as libc::c_int
            }
        }
        psCurr = (*psCurr).psNext;
        if psCurr.is_null() {
            /* Got to the end of this list see if there is another */
            psCurr = formGetAllWidgets(&mut sGetAll)
        }
    }
    return 0 as libc::c_int;
}
/* Set the tool tip font for a screen */
// /* Set the tool tip font for a screen */
//void widgSetTipFont(W_SCREEN *psScreen, PROP_FONT *psFont)
//{
//	ASSERT( PTRVALID(psScreen, sizeof(W_SCREEN)),
//		"widgSetTipFont: Invalid screen pointer" );
//	ASSERT( psFont == NULL || PTRVALID(psFont, sizeof(PROP_FONT)),
//		"widgSetTipFont: Invalid font pointer" );
//
//	psScreen->psTipFont = psFont;
//}
/* Set the tool tip font for a screen */
#[no_mangle]
pub unsafe extern "C" fn widgSetTipFont(mut psScreen: *mut W_SCREEN,
                                        mut FontID: libc::c_int) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetTipFont: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              366 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"widgSetTipFont\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //	ASSERT( psFont == NULL || PTRVALID(psFont, sizeof(PROP_FONT)),
//		"widgSetTipFont: Invalid font pointer" );
    (*psScreen).TipFontID = FontID;
}
/* Add a form to the widget screen */
/* Add a form to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgAddForm(mut psScreen: *mut W_SCREEN,
                                     mut psInit: *mut W_FORMINIT) -> BOOL {
    let mut psParent: *mut W_FORM = 0 as *mut W_FORM;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddForm: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              381 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"widgAddForm\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgAddForm: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  385 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"widgAddForm\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to add the widget to */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        /* Add to the base form */
        psParent = (*psScreen).psForm as *mut W_FORM
    } else {
        psParent = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psParent.is_null() ||
               (*psParent).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddForm: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      401 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"widgAddForm\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the form structure */
    if formCreate(&mut psForm, psInit) == 0 { return 0 as libc::c_int }
    /* Add it to the screen */
    if formAddWidget(psParent, psForm as *mut WIDGET, psInit as *mut W_INIT)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add a label to the widget screen */
/* Add a label to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgAddLabel(mut psScreen: *mut W_SCREEN,
                                      mut psInit: *mut W_LABINIT) -> BOOL {
    let mut psLabel: *mut W_LABEL = 0 as *mut W_LABEL;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddLabel: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              429 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"widgAddLabel\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgAddLabel: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  433 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"widgAddLabel\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to put the button on */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        psForm = (*psScreen).psForm as *mut W_FORM
    } else {
        psForm = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psForm.is_null() ||
               (*psForm).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddLabel: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      448 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"widgAddLabel\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the button structure */
    if labelCreate(&mut psLabel, psInit) == 0 { return 0 as libc::c_int }
    /* Add it to the form */
    if formAddWidget(psForm, psLabel as *mut WIDGET, psInit as *mut W_INIT) ==
           0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add a button to a form */
/* Add a button to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgAddButton(mut psScreen: *mut W_SCREEN,
                                       mut psInit: *mut W_BUTINIT) -> BOOL {
    let mut psButton: *mut W_BUTTON = 0 as *mut W_BUTTON;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddButton: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              476 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgAddButton\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgAddButton: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  480 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgAddButton\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to put the button on */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        psForm = (*psScreen).psForm as *mut W_FORM
    } else {
        psForm = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psForm.is_null() ||
               (*psForm).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddButton: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      495 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"widgAddButton\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the button structure */
    if buttonCreate(&mut psButton, psInit) == 0 { return 0 as libc::c_int }
    /* Add it to the form */
    if formAddWidget(psForm, psButton as *mut WIDGET, psInit as *mut W_INIT)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add an edit box to a form */
/* Add an edit box to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgAddEditBox(mut psScreen: *mut W_SCREEN,
                                        mut psInit: *mut W_EDBINIT) -> BOOL {
    let mut psEdBox: *mut W_EDITBOX = 0 as *mut W_EDITBOX;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddEditBox: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              523 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"widgAddEditBox\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgAddEditBox: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  527 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"widgAddEditBox\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to put the edit box on */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        psForm = (*psScreen).psForm as *mut W_FORM
    } else {
        psForm = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psForm.is_null() ||
               (*psForm).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddEditBox: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      542 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"widgAddEditBox\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the edit box structure */
    if editBoxCreate(&mut psEdBox, psInit) == 0 { return 0 as libc::c_int }
    /* Add it to the form */
    if formAddWidget(psForm, psEdBox as *mut WIDGET, psInit as *mut W_INIT) ==
           0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add a bar graph to a form */
/* Add a bar graph to the widget screen */
#[no_mangle]
pub unsafe extern "C" fn widgAddBarGraph(mut psScreen: *mut W_SCREEN,
                                         mut psInit: *mut W_BARINIT) -> BOOL {
    let mut psBarGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddEditBox: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              570 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"widgAddBarGraph\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgAddBarGraph: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  574 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"widgAddBarGraph\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to put the bar graph on */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        psForm = (*psScreen).psForm as *mut W_FORM
    } else {
        psForm = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psForm.is_null() ||
               (*psForm).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddBarGraph: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      589 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"widgAddBarGraph\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the bar graph structure */
    if barGraphCreate(&mut psBarGraph, psInit) == 0 {
        return 0 as libc::c_int
    }
    /* Add it to the form */
    if formAddWidget(psForm, psBarGraph as *mut WIDGET, psInit as *mut W_INIT)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Add a slider to a form */
/* Add a slider to a form */
#[no_mangle]
pub unsafe extern "C" fn widgAddSlider(mut psScreen: *mut W_SCREEN,
                                       mut psInit: *mut W_SLDINIT) -> BOOL {
    let mut psSlider: *mut W_SLIDER = 0 as *mut W_SLIDER;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgAddEditBox: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              617 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgAddSlider\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if widgCheckIDForm((*psScreen).psForm as *mut W_FORM, (*psInit).id) != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSlider: ID number has already been used\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  621 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgAddSlider\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Find the form to put the slider on */
    if (*psInit).formID == 0 as libc::c_int as libc::c_uint {
        psForm = (*psScreen).psForm as *mut W_FORM
    } else {
        psForm = widgGetFromID(psScreen, (*psInit).formID) as *mut W_FORM;
        if psForm.is_null() ||
               (*psForm).type_0 as libc::c_uint !=
                   WIDG_FORM as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgAddSlider: Could not find parent form from formID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      636 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"widgAddSlider\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Create the slider structure */
    if sliderCreate(&mut psSlider, psInit) == 0 { return 0 as libc::c_int }
    /* Add it to the form */
    if formAddWidget(psForm, psSlider as *mut WIDGET, psInit as *mut W_INIT)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Delete a widget from a form */
#[no_mangle]
pub unsafe extern "C" fn widgDeleteFromForm(mut psForm: *mut W_FORM,
                                            mut id: UDWORD,
                                            mut psContext: *mut W_CONTEXT)
 -> BOOL {
    let mut psPrev: *mut WIDGET = 0 as *mut WIDGET;
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut psNext: *mut WIDGET = 0 as *mut WIDGET;
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut minor: UDWORD = 0;
    let mut major: UDWORD = 0;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    let mut psMinor: *mut W_MINORTAB = 0 as *mut W_MINORTAB;
    let mut sNewContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    /* Clear the last hilite if necessary */
    if !(*psForm).psLastHiLite.is_null() && (*(*psForm).psLastHiLite).id == id
       {
        widgHiLiteLost((*psForm).psLastHiLite, psContext);
        (*psForm).psLastHiLite = 0 as *mut WIDGET
    }
    if (*psForm).style & 1 as libc::c_int as libc::c_uint != 0 {
        psTabForm = psForm as *mut W_TABFORM;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgDeleteFromForm: Invalid form pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  678 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgDeleteFromForm\x00")).as_ptr(),
                  b"PTRVALID(psTabForm, sizeof(W_TABFORM))\x00" as *const u8
                      as *const libc::c_char);
        };
        /* loop through all the tabs */
        psMajor = (*psTabForm).asMajor.as_mut_ptr();
        major = 0 as libc::c_int as UDWORD;
        while major < (*psTabForm).numMajor as libc::c_uint {
            psMinor = (*psMajor).asMinor.as_mut_ptr();
            minor = 0 as libc::c_int as UDWORD;
            while minor < (*psMajor).numMinor as libc::c_uint {
                if !(*psMinor).psWidgets.is_null() &&
                       (*(*psMinor).psWidgets).id == id {
                    /* The widget is the first on this tab */
                    psNext = (*(*psMinor).psWidgets).psNext;
                    widgRelease((*psMinor).psWidgets);
                    (*psMinor).psWidgets = psNext;
                    return 1 as libc::c_int
                } else {
                    psCurr = (*psMinor).psWidgets;
                    while !psCurr.is_null() {
                        if (*psCurr).id == id {
                            (*psPrev).psNext = (*psCurr).psNext;
                            widgRelease(psCurr);
                            return 1 as libc::c_int
                        }
                        if (*psCurr).type_0 as libc::c_uint ==
                               WIDG_FORM as libc::c_int as libc::c_uint {
                            /* Recurse down to other form */
                            sNewContext.psScreen = (*psContext).psScreen;
                            sNewContext.psForm = psCurr as *mut W_FORM;
                            sNewContext.xOffset =
                                (*psContext).xOffset -
                                    (*psCurr).x as libc::c_int;
                            sNewContext.yOffset =
                                (*psContext).yOffset -
                                    (*psCurr).y as libc::c_int;
                            sNewContext.mx =
                                (*psContext).mx - (*psCurr).x as libc::c_int;
                            sNewContext.my =
                                (*psContext).my - (*psCurr).y as libc::c_int;
                            if widgDeleteFromForm(psCurr as *mut W_FORM, id,
                                                  &mut sNewContext) != 0 {
                                return 1 as libc::c_int
                            }
                        }
                        psPrev = psCurr;
                        psCurr = (*psCurr).psNext
                    }
                }
                psMinor = psMinor.offset(1);
                minor = minor.wrapping_add(1)
            }
            psMajor = psMajor.offset(1);
            major = major.wrapping_add(1)
        }
    } else {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgDeleteFromForm: Invalid form pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  732 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgDeleteFromForm\x00")).as_ptr(),
                  b"PTRVALID(psForm, sizeof(W_FORM))\x00" as *const u8 as
                      *const libc::c_char);
        };
        /* Delete from a normal form */
        if !(*psForm).psWidgets.is_null() && (*(*psForm).psWidgets).id == id {
            /* The widget is the first in the list */
            psNext = (*(*psForm).psWidgets).psNext;
            widgRelease((*psForm).psWidgets);
            (*psForm).psWidgets = psNext;
            return 1 as libc::c_int
        } else {
            /* Search the rest of the list */
            psCurr = (*psForm).psWidgets;
            while !psCurr.is_null() {
                if (*psCurr).id == id {
                    (*psPrev).psNext = (*psCurr).psNext;
                    widgRelease(psCurr);
                    return 1 as libc::c_int
                }
                if (*psCurr).type_0 as libc::c_uint ==
                       WIDG_FORM as libc::c_int as libc::c_uint {
                    /* Recurse down to other form */
                    sNewContext.psScreen = (*psContext).psScreen;
                    sNewContext.psForm = psCurr as *mut W_FORM;
                    sNewContext.xOffset =
                        (*psContext).xOffset - (*psCurr).x as libc::c_int;
                    sNewContext.yOffset =
                        (*psContext).yOffset - (*psCurr).y as libc::c_int;
                    sNewContext.mx =
                        (*psContext).mx - (*psCurr).x as libc::c_int;
                    sNewContext.my =
                        (*psContext).my - (*psCurr).y as libc::c_int;
                    if widgDeleteFromForm(psCurr as *mut W_FORM, id,
                                          &mut sNewContext) != 0 {
                        return 1 as libc::c_int
                    }
                }
                psPrev = psCurr;
                psCurr = (*psCurr).psNext
            }
        }
    }
    return 0 as libc::c_int;
}
/* Delete a widget from the screen */
/* Delete a widget from the screen */
#[no_mangle]
pub unsafe extern "C" fn widgDelete(mut psScreen: *mut W_SCREEN,
                                    mut id: UDWORD) {
    let mut sContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgDelete: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              785 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"widgDelete\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Clear the keyboard focus if necessary */
    if !(*psScreen).psFocus.is_null() && (*(*psScreen).psFocus).id == id {
        screenClearFocus(psScreen);
    }
    /* Set up the initial context */
    sContext.psScreen = psScreen;
    sContext.psForm = (*psScreen).psForm as *mut W_FORM;
    sContext.xOffset = 0 as libc::c_int;
    sContext.yOffset = 0 as libc::c_int;
    sContext.mx = mouseX();
    sContext.my = mouseY();
    widgDeleteFromForm((*psScreen).psForm as *mut W_FORM, id, &mut sContext);
}
/* Initialise a form and all it's widgets */
unsafe extern "C" fn widgStartForm(mut psForm: *mut W_FORM) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut sGetAll: W_FORMGETALL =
        W_FORMGETALL{psGAWList: 0 as *mut WIDGET,
                     psGAWForm: 0 as *mut W_TABFORM,
                     psGAWMajor: 0 as *mut W_MAJORTAB,
                     GAWMajor: 0,
                     GAWMinor: 0,};
    /* Initialise this form */
    formInitialise(psForm);
    /*Initialise the widgets on the form */
    formInitGetAllWidgets(psForm, &mut sGetAll);
    psCurr = formGetAllWidgets(&mut sGetAll);
    while !psCurr.is_null() {
        match (*psCurr).type_0 as libc::c_uint {
            0 => { widgStartForm(psCurr as *mut W_FORM); }
            2 => { buttonInitialise(psCurr as *mut W_BUTTON); }
            3 => { editBoxInitialise(psCurr as *mut W_EDITBOX); }
            1 | 4 => { }
            5 => { sliderInitialise(psCurr as *mut W_SLIDER); }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgStartScreen: Unknown widget type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          837 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgStartForm\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        psCurr = (*psCurr).psNext;
        if psCurr.is_null() {
            /* Got to the end of this list see if there is another */
            psCurr = formGetAllWidgets(&mut sGetAll)
        }
    };
}
/* Initialise the set of widgets that make up a screen.
 * Call this once before calling widgRunScreen and widgDisplayScreen.
 * This should only be called once before calling Run and Display as many times
 * as is required.
 */
/* Initialise the set of widgets that make up a screen */
#[no_mangle]
pub unsafe extern "C" fn widgStartScreen(mut psScreen: *mut W_SCREEN) {
    (*psScreen).psFocus = 0 as *mut WIDGET;
    widgStartForm((*psScreen).psForm as *mut W_FORM);
}
/* Clean up after a screen has been run.
 * Call this after the widgRunScreen / widgDisplayScreen cycle.
 */
/* Clean up after a screen has been run */
#[no_mangle]
pub unsafe extern "C" fn widgEndScreen(mut psScreen: *mut W_SCREEN) { }
/* Find a widget on a form from it's id number */
unsafe extern "C" fn widgFormGetFromID(mut psForm: *mut W_FORM,
                                       mut id: UDWORD) -> *mut WIDGET {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut psFound: *mut WIDGET = 0 as *mut WIDGET;
    let mut sGetAll: W_FORMGETALL =
        W_FORMGETALL{psGAWList: 0 as *mut WIDGET,
                     psGAWForm: 0 as *mut W_TABFORM,
                     psGAWMajor: 0 as *mut W_MAJORTAB,
                     GAWMajor: 0,
                     GAWMinor: 0,};
    /* See if the form matches the ID */
    if (*psForm).id == id { return psForm as *mut WIDGET }
    /* Now search the widgets on the form */
    psFound = 0 as *mut WIDGET;
    formInitGetAllWidgets(psForm, &mut sGetAll);
    psCurr = formGetAllWidgets(&mut sGetAll);
    while !psCurr.is_null() && psFound.is_null() {
        if (*psCurr).id == id {
            psFound = psCurr
        } else if (*psCurr).type_0 as libc::c_uint ==
                      WIDG_FORM as libc::c_int as libc::c_uint {
            psFound = widgFormGetFromID(psCurr as *mut W_FORM, id)
        }
        psCurr = (*psCurr).psNext;
        if psCurr.is_null() {
            /* Got to the end of this list see if there is another */
            psCurr = formGetAllWidgets(&mut sGetAll)
        }
    }
    return psFound;
}
/* Get widget structure */
/* Find a widget in a screen from its ID number */
/* Find a widget in a screen from its ID number */
#[no_mangle]
pub unsafe extern "C" fn widgGetFromID(mut psScreen: *mut W_SCREEN,
                                       mut id: UDWORD) -> *mut WIDGET {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgGetFromID: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              905 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgGetFromID\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    return widgFormGetFromID((*psScreen).psForm as *mut W_FORM, id);
}
/* Hide a widget */
/* Hide a widget */
#[no_mangle]
pub unsafe extern "C" fn widgHide(mut psScreen: *mut W_SCREEN,
                                  mut id: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgHide: couldn\'t find widget from id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              918 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"widgHide\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(WIDGET))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psWidget.is_null() {
        (*psWidget).style |= 0x8000 as libc::c_int as libc::c_uint
    };
}
/* Reveal a widget */
/* Reveal a widget */
#[no_mangle]
pub unsafe extern "C" fn widgReveal(mut psScreen: *mut W_SCREEN,
                                    mut id: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgReveal: couldn\'t find widget from id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              933 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"widgReveal\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(WIDGET))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psWidget.is_null() {
        (*psWidget).style &= !(0x8000 as libc::c_int) as libc::c_uint
    };
}
/* Get the current position of a widget */
/* Get the current position of a widget */
#[no_mangle]
pub unsafe extern "C" fn widgGetPos(mut psScreen: *mut W_SCREEN,
                                    mut id: UDWORD, mut pX: *mut SWORD,
                                    mut pY: *mut SWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    /* Find the widget */
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() {
        *pX = (*psWidget).x;
        *pY = (*psWidget).y
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgGetPos: Couldn\'t find widget from ID\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  955 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"widgGetPos\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        *pX = 0 as libc::c_int as SWORD;
        *pY = 0 as libc::c_int as SWORD
    };
}
/* Return the ID of the widget the mouse was over this frame */
/* Return the ID of the widget the mouse was over this frame */
#[no_mangle]
pub unsafe extern "C" fn widgGetMouseOver(mut psScreen: *mut W_SCREEN)
 -> UDWORD {
    /* Don't actually need the screen parameter at the moment - but it might be
	   handy if psMouseOverWidget needs to stop being a static and moves into
	   the screen structure */
    if psMouseOverWidget.is_null() { return 0 as libc::c_int as UDWORD }
    return (*psMouseOverWidget).id;
}
/* Return the user data for a widget */
/* Return the user data for a widget */
#[no_mangle]
pub unsafe extern "C" fn widgGetUserData(mut psScreen: *mut W_SCREEN,
                                         mut id: UDWORD)
 -> *mut libc::c_void {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() { return (*psWidget).pUserData }
    return 0 as *mut libc::c_void;
}
/* Return the user data for a widget */
/* Return the user data for a widget */
#[no_mangle]
pub unsafe extern "C" fn widgGetUserData2(mut psScreen: *mut W_SCREEN,
                                          mut id: UDWORD) -> UDWORD {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() { return (*psWidget).UserData }
    return 0 as libc::c_int as UDWORD;
}
/* Set the user data for a widget */
/* Set user data for a widget */
#[no_mangle]
pub unsafe extern "C" fn widgSetUserData(mut psScreen: *mut W_SCREEN,
                                         mut id: UDWORD,
                                         mut UserData: *mut libc::c_void) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() { (*psWidget).pUserData = UserData };
}
/* Set the user data for a widget */
/* Set user data for a widget */
#[no_mangle]
pub unsafe extern "C" fn widgSetUserData2(mut psScreen: *mut W_SCREEN,
                                          mut id: UDWORD,
                                          mut UserData: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() { (*psWidget).UserData = UserData };
}
/* Return the user data for the returned widget */
/* Return the user data for the returned widget */
#[no_mangle]
pub unsafe extern "C" fn widgGetLastUserData(mut psScreen: *mut W_SCREEN)
 -> *mut libc::c_void {
    /* Don't actually need the screen parameter at the moment - but it might be
	   handy if psRetWidget needs to stop being a static and moves into
	   the screen structure */
    if !psRetWidget.is_null() { return (*psRetWidget).pUserData }
    return 0 as *mut libc::c_void;
}
/* Set tip string for a widget */
/* Set tip string for a widget */
#[no_mangle]
pub unsafe extern "C" fn widgSetTip(mut psScreen: *mut W_SCREEN,
                                    mut id: UDWORD, mut pTip: *mut STRING) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() {
        match (*psWidget).type_0 as libc::c_uint {
            0 => {
                if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
                    let ref mut fresh0 =
                        (*(psWidget as *mut W_CLICKFORM)).pTip;
                    *fresh0 = pTip
                } else if (*psWidget).style & 1 as libc::c_int as libc::c_uint
                              != 0 {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"widgSetTip: tabbed forms do not have a tip\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"widget.c\x00" as *const u8 as
                                  *const libc::c_char, 1067 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 11],
                                                        &[libc::c_char; 11]>(b"widgSetTip\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                } else {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"widgSetTip: plain forms do not have a tip\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"widget.c\x00" as *const u8 as
                                  *const libc::c_char, 1071 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 11],
                                                        &[libc::c_char; 11]>(b"widgSetTip\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            1 => {
                let ref mut fresh1 = (*(psWidget as *mut W_LABEL)).pTip;
                *fresh1 = pTip
            }
            2 => {
                let ref mut fresh2 = (*(psWidget as *mut W_BUTTON)).pTip;
                *fresh2 = pTip
            }
            4 => {
                let ref mut fresh3 = (*(psWidget as *mut W_BARGRAPH)).pTip;
                *fresh3 = pTip
            }
            5 => {
                let ref mut fresh4 = (*(psWidget as *mut W_SLIDER)).pTip;
                *fresh4 = pTip
            }
            3 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgSetTip: edit boxes do not have a tip\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1092 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 11],
                                                    &[libc::c_char; 11]>(b"widgSetTip\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgSetTip: Unknown widget type\x00" as *const u8
                              as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1096 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 11],
                                                    &[libc::c_char; 11]>(b"widgSetTip\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    };
}
/* Return which key was used to press the last returned widget */
/* Return which key was used to press the last returned widget */
#[no_mangle]
pub unsafe extern "C" fn widgGetButtonKey(mut psScreen: *mut W_SCREEN)
 -> UDWORD {
    /* Don't actually need the screen parameter at the moment - but it might be
	   handy if released needs to stop being a static and moves into
	   the screen structure */
    return released;
}
/* Get a button or clickable form's state */
/* Get a button or clickable form's state */
#[no_mangle]
pub unsafe extern "C" fn widgGetButtonState(mut psScreen: *mut W_SCREEN,
                                            mut id: UDWORD) -> UDWORD {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    /* Get the button */
    psWidget = widgGetFromID(psScreen, id);
    if psWidget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgGetButtonState: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1123 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgGetButtonState\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_BUTTON as libc::c_int as libc::c_uint {
        return buttonGetState(psWidget as *mut W_BUTTON)
    } else {
        if (*psWidget).type_0 as libc::c_uint ==
               WIDG_FORM as libc::c_int as libc::c_uint &&
               (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
            return formGetClickState(psWidget as *mut W_CLICKFORM)
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgGetButtonState: Couldn\'t find button/click form from ID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1135 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"widgGetButtonState\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 0 as libc::c_int as UDWORD;
}
/* Possible states for a button */
// Disable (grey out) a button
// Fix a button down
// Fix a button down but it is still clickable
// Make a button flash.
#[no_mangle]
pub unsafe extern "C" fn widgSetButtonFlash(mut psScreen: *mut W_SCREEN,
                                            mut id: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    /* Get the button */
    psWidget = widgGetFromID(psScreen, id);
    if psWidget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetButtonFlash: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1149 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgSetButtonFlash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_BUTTON as libc::c_int as libc::c_uint {
        buttonSetFlash(psWidget as *mut W_BUTTON);
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_FORM as libc::c_int as libc::c_uint &&
                  (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        formSetFlash(psWidget as *mut W_FORM);
    } else if !((*psWidget).type_0 as libc::c_uint ==
                    WIDG_EDITBOX as libc::c_int as libc::c_uint) {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetButtonFlash: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1165 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgSetButtonFlash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn widgClearButtonFlash(mut psScreen: *mut W_SCREEN,
                                              mut id: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    /* Get the button */
    psWidget = widgGetFromID(psScreen, id);
    if psWidget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetButtonFlash: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1178 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"widgClearButtonFlash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_BUTTON as libc::c_int as libc::c_uint {
        buttonClearFlash(psWidget as *mut W_BUTTON);
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_FORM as libc::c_int as libc::c_uint &&
                  (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        formClearFlash(psWidget as *mut W_FORM);
    } else if !((*psWidget).type_0 as libc::c_uint ==
                    WIDG_EDITBOX as libc::c_int as libc::c_uint) {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgClearButtonFlash: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1193 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"widgClearButtonFlash\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    };
}
/* Set a button or clickable form's state */
/* Set a button or clickable form's state */
#[no_mangle]
pub unsafe extern "C" fn widgSetButtonState(mut psScreen: *mut W_SCREEN,
                                            mut id: UDWORD,
                                            mut state: UDWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    /* Get the button */
    psWidget = widgGetFromID(psScreen, id);
    if psWidget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetButtonState: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1207 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgSetButtonState\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_BUTTON as libc::c_int as libc::c_uint {
        buttonSetState(psWidget as *mut W_BUTTON, state);
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_FORM as libc::c_int as libc::c_uint &&
                  (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        formSetClickState(psWidget as *mut W_CLICKFORM, state);
    } else if (*psWidget).type_0 as libc::c_uint ==
                  WIDG_EDITBOX as libc::c_int as libc::c_uint {
        editBoxSetState(psWidget as *mut W_EDITBOX, state);
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetButtonState: Couldn\'t find button/click form from ID\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1223 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"widgSetButtonState\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    };
}
/* Return a pointer to a buffer containing the current string of a widget if any.
 * This will always return a valid string pointer.
 * NOTE: The string must be copied out of the buffer
 */
/* Return a pointer to a buffer containing the current string of a widget.
 * NOTE: The string must be copied out of the buffer
 */
#[no_mangle]
pub unsafe extern "C" fn widgGetString(mut psScreen: *mut W_SCREEN,
                                       mut id: UDWORD) -> *mut STRING {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgGetString: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              1236 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Get the widget */
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() {
        match (*psWidget).type_0 as libc::c_uint {
            0 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Forms do not have a string\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1245 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                aStringRetBuffer[0 as libc::c_int as usize] =
                    0 as libc::c_int as STRING
            }
            1 => {
                strcpy(aStringRetBuffer.as_mut_ptr(),
                       (*(psWidget as *mut W_LABEL)).aText.as_mut_ptr());
            }
            2 => {
                if !(*(psWidget as *mut W_BUTTON)).pText.is_null() {
                    strcpy(aStringRetBuffer.as_mut_ptr(),
                           (*(psWidget as *mut W_BUTTON)).pText);
                } else {
                    aStringRetBuffer[0 as libc::c_int as usize] =
                        0 as libc::c_int as STRING
                }
            }
            3 => {
                strcpy(aStringRetBuffer.as_mut_ptr(),
                       (*(psWidget as *mut W_EDITBOX)).aText.as_mut_ptr());
            }
            4 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Bar Graphs do not have a string\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1265 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                aStringRetBuffer[0 as libc::c_int as usize] =
                    0 as libc::c_int as STRING
            }
            5 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Sliders do not have a string\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1269 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                aStringRetBuffer[0 as libc::c_int as usize] =
                    0 as libc::c_int as STRING
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Unknown widget type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1273 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                aStringRetBuffer[0 as libc::c_int as usize] =
                    0 as libc::c_int as STRING
            }
        }
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgGetString: couldn\'t get widget from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1280 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgGetString\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        aStringRetBuffer[0 as libc::c_int as usize] =
            0 as libc::c_int as STRING
    }
    return aStringRetBuffer.as_mut_ptr();
}
/* Set the text in a widget */
/* Set the text in a widget */
#[no_mangle]
pub unsafe extern "C" fn widgSetString(mut psScreen: *mut W_SCREEN,
                                       mut id: UDWORD,
                                       mut pText: *mut STRING) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetString: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"widget.c\x00" as *const u8 as *const libc::c_char,
              1294 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Get the widget */
    psWidget = widgGetFromID(psScreen, id);
    if !psWidget.is_null() {
        match (*psWidget).type_0 as libc::c_uint {
            0 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgSetString: forms do not have a string\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1303 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            1 => {
                widgCopyString((*(psWidget as
                                      *mut W_LABEL)).aText.as_mut_ptr(),
                               pText);
            }
            2 => {
                let ref mut fresh5 = (*(psWidget as *mut W_BUTTON)).pText;
                *fresh5 = pText
            }
            3 => {
                if (*psScreen).psFocus == psWidget {
                    screenClearFocus(psScreen);
                }
                editBoxSetString(psWidget as *mut W_EDITBOX, pText);
            }
            4 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Bar graphs do not have a string\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1330 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            5 => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgGetString: Sliders do not have a string\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1333 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"widgSetString: Unknown widget type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"widget.c\x00" as *const u8 as *const libc::c_char,
                          1336 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetString: couldn\'t get widget from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"widget.c\x00" as *const u8 as *const libc::c_char,
                  1342 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgSetString\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    };
}
/* Call any callbacks for the widgets on a form */
unsafe extern "C" fn widgProcessCallbacks(mut psContext: *mut W_CONTEXT) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut sFormContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut sWidgContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut xOrigin: SDWORD = 0;
    let mut yOrigin: SDWORD = 0;
    let mut sFormCtl: W_FORMGETALL =
        W_FORMGETALL{psGAWList: 0 as *mut WIDGET,
                     psGAWForm: 0 as *mut W_TABFORM,
                     psGAWMajor: 0 as *mut W_MAJORTAB,
                     GAWMajor: 0,
                     GAWMinor: 0,};
    /* Initialise the form context */
    sFormContext.psScreen = (*psContext).psScreen;
    /* Initialise widget context */
    formGetOrigin((*psContext).psForm, &mut xOrigin, &mut yOrigin);
    sWidgContext.psScreen = (*psContext).psScreen;
    sWidgContext.psForm = (*psContext).psForm;
    sWidgContext.mx = (*psContext).mx - xOrigin;
    sWidgContext.my = (*psContext).my - yOrigin;
    sWidgContext.xOffset = (*psContext).xOffset + xOrigin;
    sWidgContext.yOffset = (*psContext).yOffset + yOrigin;
    /* Go through all the widgets on the form */
    formInitGetAllWidgets((*psContext).psForm, &mut sFormCtl);
    psCurr = formGetAllWidgets(&mut sFormCtl);
    while !psCurr.is_null() {
        while !psCurr.is_null() {
            /* Skip any hidden widgets */
/*  Not sure if we want to skip hidden widgets or not ....
			if (psCurr->style & WIDG_HIDDEN)
			{
				continue;
			}*/
            /* Call the callback */
            if (*psCurr).callback.is_some() {
                (*psCurr).callback.expect("non-null function pointer")(psCurr,
                                                                       &mut sWidgContext);
            }
            /* and then recurse */
            if (*psCurr).type_0 as libc::c_uint ==
                   WIDG_FORM as libc::c_int as libc::c_uint {
                sFormContext.psForm = psCurr as *mut W_FORM;
                sFormContext.mx =
                    sWidgContext.mx - (*psCurr).x as libc::c_int;
                sFormContext.my =
                    sWidgContext.my - (*psCurr).y as libc::c_int;
                sFormContext.xOffset =
                    sWidgContext.xOffset + (*psCurr).x as libc::c_int;
                sFormContext.yOffset =
                    sWidgContext.yOffset + (*psCurr).y as libc::c_int;
                widgProcessCallbacks(&mut sFormContext);
            }
            psCurr = (*psCurr).psNext
        }
        /* See if the form has any more widgets on it */
        psCurr = formGetAllWidgets(&mut sFormCtl)
    };
}
/* Process all the widgets on a form.
 * mx and my are the coords of the mouse relative to the form origin.
 */
unsafe extern "C" fn widgProcessForm(mut psContext: *mut W_CONTEXT) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut psOver: *mut WIDGET = 0 as *mut WIDGET;
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    let mut omx: SDWORD = 0;
    let mut omy: SDWORD = 0;
    let mut xOffset: SDWORD = 0;
    let mut yOffset: SDWORD = 0;
    let mut xOrigin: SDWORD = 0;
    let mut yOrigin: SDWORD = 0;
    let mut psForm: *mut W_FORM = 0 as *mut W_FORM;
    let mut sFormContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut sWidgContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    /* Note current form */
    psForm = (*psContext).psForm;
    //	if(psForm->disableChildren == TRUE) {
//		return;
//	}
    /* Note the current mouse position */
    mx = (*psContext).mx;
    my = (*psContext).my;
    /* Note the current offset */
    xOffset = (*psContext).xOffset;
    yOffset = (*psContext).yOffset;
    /* Initialise the form context */
    sFormContext.psScreen = (*psContext).psScreen;
    /* Initialise widget context */
    formGetOrigin(psForm, &mut xOrigin, &mut yOrigin);
    sWidgContext.psScreen = (*psContext).psScreen;
    sWidgContext.psForm = psForm;
    sWidgContext.mx = mx - xOrigin;
    sWidgContext.my = my - yOrigin;
    sWidgContext.xOffset = xOffset + xOrigin;
    sWidgContext.yOffset = yOffset + yOrigin;
    /* Process the form's widgets */
    psOver = 0 as *mut WIDGET;
    psCurr = formGetWidgets(psForm);
    while !psCurr.is_null() {
        /* Skip any hidden widgets */
        if !((*psCurr).style & 0x8000 as libc::c_int as libc::c_uint != 0) {
            if (*psCurr).type_0 as libc::c_uint ==
                   WIDG_FORM as libc::c_int as libc::c_uint {
                /* Found a sub form, so set up the context */
                sFormContext.psForm = psCurr as *mut W_FORM;
                sFormContext.mx = mx - (*psCurr).x as libc::c_int - xOrigin;
                sFormContext.my = my - (*psCurr).y as libc::c_int - yOrigin;
                sFormContext.xOffset =
                    xOffset + (*psCurr).x as libc::c_int + xOrigin;
                sFormContext.yOffset =
                    yOffset + (*psCurr).y as libc::c_int + yOrigin;
                /* Process it */
                widgProcessForm(&mut sFormContext);
            } else {
                /* Run the widget */
                widgRun(psCurr, &mut sWidgContext);
            }
        }
        psCurr = (*psCurr).psNext
    }
    /* Now check for mouse clicks */
    omx = mx - xOrigin;
    omy = my - yOrigin;
    if mx >= 0 as libc::c_int && mx <= (*psForm).width as libc::c_int &&
           my >= 0 as libc::c_int && my <= (*psForm).height as libc::c_int {
        /* Update for the origin */
        /* Mouse is over the form - is it over any of the widgets */
        psCurr = formGetWidgets(psForm);
        while !psCurr.is_null() {
            /* Skip any hidden widgets */
            if !((*psCurr).style & 0x8000 as libc::c_int as libc::c_uint != 0)
               {
                if omx >= (*psCurr).x as libc::c_int &&
                       omy >= (*psCurr).y as libc::c_int &&
                       omx <=
                           (*psCurr).x as libc::c_int +
                               (*psCurr).width as libc::c_int &&
                       omy <=
                           (*psCurr).y as libc::c_int +
                               (*psCurr).height as libc::c_int {
                    /* Note the widget the mouse is over */
                    if psMouseOverWidget.is_null() {
                        psMouseOverWidget = psCurr
                    }
                    psOver = psCurr;
                    /* Don't check the widgets if it is a clickable form */
                    if (*psForm).style & 4 as libc::c_int as libc::c_uint == 0
                       {
                        if pressed != 0 as libc::c_int as libc::c_uint &&
                               (*psCurr).type_0 as libc::c_uint !=
                                   WIDG_FORM as libc::c_int as libc::c_uint {
                            /* Tell the widget it has been clicked */
                            widgClicked(psCurr, pressed, &mut sWidgContext);
                        }
                        if released != 0 as libc::c_int as libc::c_uint &&
                               (*psCurr).type_0 as libc::c_uint !=
                                   WIDG_FORM as libc::c_int as libc::c_uint {
                            /* Tell the widget the mouse button has gone up */
                            widgReleased(psCurr, released, &mut sWidgContext);
                        }
                    }
                }
            }
            psCurr = (*psCurr).psNext
        }
        /* Note that the mouse is over this form */
        if psMouseOverWidget.is_null() {
            psMouseOverWidget = psForm as *mut WIDGET
        }
        /* Only send the Clicked or Released messages if a widget didn't get the message */
        if pressed != 0 as libc::c_int as libc::c_uint &&
               (psOver.is_null() ||
                    (*psForm).style & 4 as libc::c_int as libc::c_uint != 0) {
            /* Tell the form it has been clicked */
            widgClicked(psForm as *mut WIDGET, pressed, psContext);
        }
        if released != 0 as libc::c_int as libc::c_uint &&
               (psOver.is_null() ||
                    (*psForm).style & 4 as libc::c_int as libc::c_uint != 0) {
            /* Tell the form the mouse button has gone up */
            widgReleased(psForm as *mut WIDGET, released, psContext);
        }
    }
    /* See if the mouse has moved onto or off a widget */
    if (*psForm).psLastHiLite != psOver {
        if !psOver.is_null() { widgHiLite(psOver, &mut sWidgContext); }
        if !(*psForm).psLastHiLite.is_null() {
            widgHiLiteLost((*psForm).psLastHiLite, &mut sWidgContext);
        }
        (*psForm).psLastHiLite = psOver
    }
    /* Run this form */
    widgRun(psForm as *mut WIDGET, psContext);
}
/* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
/* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
#[no_mangle]
pub unsafe extern "C" fn widgRunScreen(mut psScreen: *mut W_SCREEN)
 -> UDWORD {
    let mut sContext: W_CONTEXT =
        W_CONTEXT{psScreen: 0 as *mut W_SCREEN,
                  psForm: 0 as *mut _w_form,
                  xOffset: 0,
                  yOffset: 0,
                  mx: 0,
                  my: 0,};
    let mut returnID: UDWORD = 0;
    psRetWidget = 0 as *mut WIDGET;
    // Note which keys have been pressed
    pressed = 0 as libc::c_int as UDWORD;
    if getWidgetsStatus() != 0 {
        if mousePressed(MOUSE_LMB) != 0 {
            pressed = 1 as libc::c_int as UDWORD
        } else if mousePressed(MOUSE_RMB) != 0 {
            pressed = 2 as libc::c_int as UDWORD
        }
        released = 0 as libc::c_int as UDWORD;
        if mouseReleased(MOUSE_LMB) != 0 {
            released = 1 as libc::c_int as UDWORD
        } else if mouseReleased(MOUSE_RMB) != 0 {
            released = 2 as libc::c_int as UDWORD
        }
    }
    /* Initialise the context */
    sContext.psScreen = psScreen;
    sContext.psForm = (*psScreen).psForm as *mut W_FORM;
    sContext.xOffset = 0 as libc::c_int;
    sContext.yOffset = 0 as libc::c_int;
    sContext.mx = mouseX();
    sContext.my = mouseY();
    psMouseOverWidget = 0 as *mut WIDGET;
    /* Process the screen's widgets */
    widgProcessForm(&mut sContext);
    /* Process any user callback functions */
    widgProcessCallbacks(&mut sContext);
    /* Return the ID of a pressed button or finished edit box if any */
    if !psRetWidget.is_null() {
        returnID = (*psRetWidget).id
    } else { returnID = 0 as libc::c_int as UDWORD }
    return returnID;
}
/* Set the id number for widgRunScreen to return */
/* Set the id number for widgRunScreen to return */
#[no_mangle]
pub unsafe extern "C" fn widgSetReturn(mut psWidget: *mut WIDGET) {
    psRetWidget = psWidget;
}
/* Display the widgets on a form */
unsafe extern "C" fn widgDisplayForm(mut psForm: *mut W_FORM,
                                     mut xOffset: UDWORD,
                                     mut yOffset: UDWORD) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut xOrigin: SDWORD = 0 as libc::c_int;
    let mut yOrigin: SDWORD = 0 as libc::c_int;
    /* Display the form */
    (*psForm).display.expect("non-null function pointer")(psForm as
                                                              *mut WIDGET,
                                                          xOffset, yOffset,
                                                          (*psForm).aColours.as_mut_ptr());
    if (*psForm).disableChildren == 1 as libc::c_int { return }
    /* Update the offset from the current form's position */
    formGetOrigin(psForm, &mut xOrigin, &mut yOrigin);
    xOffset =
        (xOffset as
             libc::c_uint).wrapping_add(((*psForm).x as libc::c_int + xOrigin)
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    yOffset =
        (yOffset as
             libc::c_uint).wrapping_add(((*psForm).y as libc::c_int + yOrigin)
                                            as libc::c_uint) as UDWORD as
            UDWORD;
    /* If this is a clickable form, the widgets on it have to move when it's down */
    if (*psForm).style & 8 as libc::c_int as libc::c_uint == 0 {
        if (*psForm).style & 4 as libc::c_int as libc::c_uint != 0 &&
               (*(psForm as *mut W_CLICKFORM)).state &
                   (0x1 as libc::c_int | 0x8 as libc::c_int |
                        0x10 as libc::c_int) as libc::c_uint != 0 {
            xOffset =
                (xOffset as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            yOffset =
                (yOffset as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
    }
    /* Display the widgets on the form */
    psCurr = formGetWidgets(psForm);
    while !psCurr.is_null() {
        /* Skip any hidden widgets */
        if !((*psCurr).style & 0x8000 as libc::c_int as libc::c_uint != 0) {
            if (*psCurr).type_0 as libc::c_uint ==
                   WIDG_FORM as libc::c_int as libc::c_uint {
                widgDisplayForm(psCurr as *mut W_FORM, xOffset, yOffset);
            } else {
                (*psCurr).display.expect("non-null function pointer")(psCurr,
                                                                      xOffset,
                                                                      yOffset,
                                                                      (*psForm).aColours.as_mut_ptr());
            }
        }
        psCurr = (*psCurr).psNext
    };
}
/* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
/* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
#[no_mangle]
pub unsafe extern "C" fn widgDisplayScreen(mut psScreen: *mut W_SCREEN) {
    /* Display the widgets */
    widgDisplayForm((*psScreen).psForm as *mut W_FORM,
                    0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD);
    /* Display the tool tip if there is one */
    tipDisplay();
}
/* Set the keyboard focus for the screen */
/* Set the keyboard focus for the screen */
#[no_mangle]
pub unsafe extern "C" fn screenSetFocus(mut psScreen: *mut W_SCREEN,
                                        mut psWidget: *mut WIDGET) {
    if !(*psScreen).psFocus.is_null() { widgFocusLost((*psScreen).psFocus); }
    (*psScreen).psFocus = psWidget;
}
/* Clear the keyboard focus */
/* Clear the keyboard focus */
#[no_mangle]
pub unsafe extern "C" fn screenClearFocus(mut psScreen: *mut W_SCREEN) {
    if !(*psScreen).psFocus.is_null() {
        widgFocusLost((*psScreen).psFocus);
        (*psScreen).psFocus = 0 as *mut WIDGET
    };
}
/* Call the correct function for loss of focus */
/* Call the correct function for loss of focus */
#[no_mangle]
pub unsafe extern "C" fn widgFocusLost(mut psWidget: *mut WIDGET) {
    match (*psWidget).type_0 as libc::c_uint {
        3 => { editBoxFocusLost(psWidget as *mut W_EDITBOX); }
        0 | 1 | 2 | 4 | 5 => { }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgFocusLost: Unknown widget type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1731 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"widgFocusLost\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Call the correct function for mouse over */
/* Function prototypes */
/* Call the correct function for mouse over */
#[no_mangle]
pub unsafe extern "C" fn widgHiLite(mut psWidget: *mut WIDGET,
                                    mut psContext: *mut W_CONTEXT) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formHiLite(psWidget as *mut W_FORM, psContext); }
        1 => { labelHiLite(psWidget as *mut W_LABEL, psContext); }
        2 => { buttonHiLite(psWidget as *mut W_BUTTON, psContext); }
        3 => { editBoxHiLite(psWidget as *mut W_EDITBOX); }
        4 => { barGraphHiLite(psWidget as *mut W_BARGRAPH, psContext); }
        5 => { sliderHiLite(psWidget as *mut W_SLIDER); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgHiLite: Unknown widget type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1761 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"widgHiLite\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Call the correct function for mouse moving off */
/* Call the correct function for mouse moving off */
#[no_mangle]
pub unsafe extern "C" fn widgHiLiteLost(mut psWidget: *mut WIDGET,
                                        mut psContext: *mut W_CONTEXT) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formHiLiteLost(psWidget as *mut W_FORM, psContext); }
        1 => { labelHiLiteLost(psWidget as *mut W_LABEL); }
        2 => { buttonHiLiteLost(psWidget as *mut W_BUTTON); }
        3 => { editBoxHiLiteLost(psWidget as *mut W_EDITBOX); }
        4 => { barGraphHiLiteLost(psWidget as *mut W_BARGRAPH); }
        5 => { sliderHiLiteLost(psWidget as *mut W_SLIDER); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgHiLiteLost: Unknown widget type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1792 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"widgHiLiteLost\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Call the correct function for mouse pressed */
unsafe extern "C" fn widgClicked(mut psWidget: *mut WIDGET, mut key: UDWORD,
                                 mut psContext: *mut W_CONTEXT) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formClicked(psWidget as *mut W_FORM, key); }
        2 => { buttonClicked(psWidget as *mut W_BUTTON, key); }
        3 => { editBoxClicked(psWidget as *mut W_EDITBOX, psContext); }
        1 | 4 => { }
        5 => { sliderClicked(psWidget as *mut W_SLIDER, psContext); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgClicked: Unknown widget type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1819 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"widgClicked\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Call the correct function for mouse released */
unsafe extern "C" fn widgReleased(mut psWidget: *mut WIDGET, mut key: UDWORD,
                                  mut psContext: *mut W_CONTEXT) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formReleased(psWidget as *mut W_FORM, key, psContext); }
        2 => { buttonReleased(psWidget as *mut W_BUTTON, key); }
        3 => { editBoxReleased(psWidget as *mut W_EDITBOX); }
        1 | 4 => { }
        5 => { sliderReleased(psWidget as *mut W_SLIDER); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgReleased: Unknown widget type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1847 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"widgReleased\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Call the correct function to run a widget */
unsafe extern "C" fn widgRun(mut psWidget: *mut WIDGET,
                             mut psContext: *mut W_CONTEXT) {
    match (*psWidget).type_0 as libc::c_uint {
        0 => { formRun(psWidget as *mut W_FORM, psContext); }
        2 => { buttonRun(psWidget as *mut W_BUTTON); }
        3 => { editBoxRun(psWidget as *mut W_EDITBOX, psContext); }
        1 | 4 => { }
        5 => { sliderRun(psWidget as *mut W_SLIDER, psContext); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"widgRun: Unknown widget type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"widget.c\x00" as *const u8 as *const libc::c_char,
                      1875 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 8],
                                                &[libc::c_char; 8]>(b"widgRun\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
// Set the current audio callback function and audio id's.
#[no_mangle]
pub unsafe extern "C" fn WidgSetAudio(mut Callback: WIDGET_AUDIOCALLBACK,
                                      mut HilightID: SWORD,
                                      mut ClickedID: SWORD) {
    AudioCallback = Callback;
    HilightAudioID = HilightID;
    ClickedAudioID = ClickedID;
}
// Get pointer to current audio callback function.
#[no_mangle]
pub unsafe extern "C" fn WidgGetAudioCallback() -> WIDGET_AUDIOCALLBACK {
    return AudioCallback;
}
// Get current audio ID for hilight.
#[no_mangle]
pub unsafe extern "C" fn WidgGetHilightAudioID() -> SWORD {
    return HilightAudioID;
}
// Get current audio ID for clicked.
#[no_mangle]
pub unsafe extern "C" fn WidgGetClickedAudioID() -> SWORD {
    return ClickedAudioID;
}
#[no_mangle]
pub unsafe extern "C" fn setWidgetsStatus(mut var: BOOL) {
    bWidgetsActive = var;
}
#[no_mangle]
pub unsafe extern "C" fn getWidgetsStatus() -> BOOL { return bWidgetsActive; }
