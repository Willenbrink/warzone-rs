use ::libc;
extern "C" {
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
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Return the ID of the widget the mouse was over this frame */
    /* Return the user data for a widget */
    /* Set the user data for a widget */
    /* Return the user data for a widget */
    /* Set the user data for a widget */
    /* Return the user data for the returned widget */
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /*
 * Setup a tool tip.
 * The tip module will then wait until the correct points to
 * display and then remove the tool tip.
 * i.e. The tip will not be displayed immediately.
 * Calling this while another tip is being displayed will restart
 * the tip system.
 * psSource is the widget that started the tip.
 * x,y,width,height - specify the position of the button to place the
 * tip by.
 */
    #[no_mangle]
    fn tipStart(psSource: *mut WIDGET, pTip: *mut STRING,
                NewFontID: libc::c_int, pColours: *mut UDWORD, x: SDWORD,
                y: SDWORD, width: UDWORD, height: UDWORD);
    /* Stop a tool tip (e.g. if the hilite is lost on a button).
 * psSource should be the same as the widget that started the tip.
 */
    #[no_mangle]
    fn tipStop(psSource: *mut WIDGET);
    /* **************************************************************************/
/*
 * pieBlitFunc.h
 *
 * patch for exisitng ivis rectangle draw functions.
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
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
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
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_colourdef {
    pub red: UBYTE,
    pub green: UBYTE,
    pub blue: UBYTE,
    pub alpha: UBYTE,
}
pub type W_COLOURDEF = _w_colourdef;
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
pub type W_BARINIT = _w_barinit;
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/*
 * Bar.h
 *
 * Definitions for Bar Graph functions.
 */
/* The widget heap */
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
// Orientation of the bar on the widget
// Percentage of the main bar that is filled
// Percentage of the minor bar if there is one
// Maximum range
// Current value
// Colour for the major bar
// Colour for the minor bar
// The tool tip for the graph
/* Colour numbers */
pub type _w_colour = libc::c_uint;
// all colour numbers are less than this
// Text colour on a disabled button
pub const WCOL_MAX: _w_colour = 8;
// Background for the tool tip window
pub const WCOL_DISABLE: _w_colour = 7;
// Edit Box cursor colour
pub const WCOL_TIPBKGRND: _w_colour = 6;
// Hilite colour
pub const WCOL_CURSOR: _w_colour = 5;
// Dark colour for 3D effects
pub const WCOL_HILITE: _w_colour = 4;
// Light colour for 3D effects
pub const WCOL_DARK: _w_colour = 3;
// Text colour
pub const WCOL_LIGHT: _w_colour = 2;
// Background colours
pub const WCOL_TEXT: _w_colour = 1;
pub const WCOL_BKGRND: _w_colour = 0;
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
pub type W_CONTEXT = _w_context;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
/*
 * bar.c
 *
 * Functions for the bar graph widget
 */
// FIXME Direct iVis implementation include!
/* The widget heap */
#[no_mangle]
pub static mut psBarHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Create a barGraph widget data structure */
/* Create a barGraph widget data structure */
#[no_mangle]
pub unsafe extern "C" fn barGraphCreate(mut ppsWidget: *mut *mut W_BARGRAPH,
                                        mut psInit: *mut W_BARINIT) -> BOOL {
    if (*psInit).style &
           !(0 as libc::c_int | 1 as libc::c_int | 2 as libc::c_int |
                 0x8000 as libc::c_int) as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Unknown bar graph style\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  25 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"barGraphCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if ((*psInit).orientation as libc::c_int) < 0x1 as libc::c_int ||
           (*psInit).orientation as libc::c_int > 0x4 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"barGraphCreate: Unknown orientation\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  31 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"barGraphCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).size as libc::c_int > 100 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"barGraphCreate: Bar size out of range\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  37 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"barGraphCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).style & 2 as libc::c_int as libc::c_uint != 0 &&
           (*psInit).minorSize as libc::c_int > 100 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"barGraphCreate: Minor bar size out of range\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  42 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"barGraphCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate the required memory */
    if heapAlloc(psBarHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"barGraphCreate: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  54 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"barGraphCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate the memory for the tip and copy it if necessary */
    if !(*psInit).pTip.is_null() {
        (**ppsWidget).pTip = (*psInit).pTip
    } else { (**ppsWidget).pTip = 0 as *mut STRING }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_BARGRAPH;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).x = (*psInit).x;
    (**ppsWidget).y = (*psInit).y;
    (**ppsWidget).width = (*psInit).width;
    (**ppsWidget).height = (*psInit).height;
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    (**ppsWidget).barPos = (*psInit).orientation;
    (**ppsWidget).majorSize = (*psInit).size;
    (**ppsWidget).minorSize = (*psInit).minorSize;
    (**ppsWidget).iRange = (*psInit).iRange;
    /* Set the display function */
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else if (*psInit).style & 1 as libc::c_int as libc::c_uint != 0 {
        (**ppsWidget).display =
            Some(barGraphDisplayTrough as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    } else if (*psInit).style & 2 as libc::c_int as libc::c_uint != 0 {
        (**ppsWidget).display =
            Some(barGraphDisplayDouble as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    } else {
        (**ppsWidget).display =
            Some(barGraphDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    /* Set the major colour */
    (**ppsWidget).majorCol =
        pal_GetNearestColour((*psInit).sCol.red, (*psInit).sCol.green,
                             (*psInit).sCol.blue);
    /* Set the minor colour if necessary */
    if (*psInit).style & 2 as libc::c_int as libc::c_uint != 0 {
        (**ppsWidget).majorCol =
            pal_GetNearestColour((*psInit).sMinorCol.red,
                                 (*psInit).sMinorCol.green,
                                 (*psInit).sMinorCol.blue)
    }
    barGraphInitialise(*ppsWidget);
    return 1 as libc::c_int;
}
/* Free the memory used by a barGraph */
/* Free the memory used by a barGraph */
#[no_mangle]
pub unsafe extern "C" fn barGraphFree(mut psWidget: *mut W_BARGRAPH) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"barGraphFree: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bar.c\x00" as *const u8 as *const libc::c_char,
              132 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"barGraphFree\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_BARGRAPH))\x00" as *const u8 as
                  *const libc::c_char);
    };
    heapFree(psBarHeap, psWidget as *mut libc::c_void);
}
/* Initialise a barGraph widget before running it */
/* Initialise a barGraph widget before running it */
#[no_mangle]
pub unsafe extern "C" fn barGraphInitialise(mut psWidget: *mut W_BARGRAPH) { }
/* Set the current size of a bar graph */
#[no_mangle]
pub unsafe extern "C" fn widgSetBarSize(mut psScreen: *mut W_SCREEN,
                                        mut id: UDWORD, mut iValue: UDWORD) {
    let mut psBGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    let mut size: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetBarSize: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bar.c\x00" as *const u8 as *const libc::c_char,
              162 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"widgSetBarSize\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psBGraph = widgGetFromID(psScreen, id) as *mut W_BARGRAPH;
    if psBGraph.is_null() ||
           (*psBGraph).type_0 as libc::c_uint !=
               WIDG_BARGRAPH as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetBarSize: Couldn\'t find widget from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  167 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"widgSetBarSize\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    if iValue < (*psBGraph).iRange as libc::c_uint {
        (*psBGraph).iValue = iValue as UWORD
    } else { (*psBGraph).iValue = (*psBGraph).iRange }
    size =
        (100 as libc::c_int * (*psBGraph).iValue as libc::c_int /
             (*psBGraph).iRange as libc::c_int) as UDWORD;
    (*psBGraph).majorSize = size as UWORD;
}
/*
 * Widget.h
 *
 * Definitions for the Widget library
 */
/* **********************************************************************************
 *
 * Widget style definitions - these control how the basic widget appears on screen
 */
// The widget is initially hidden
/* *********** Form styles ****************/
/* Plain form */
/* Tabbed form */
/* Invisible (i.e. see through) form -
 * can be used in conjunction with WFORM_PLAIN or WFORM_TABBED.
 */
/* Clickable form - return form id when the form is clicked */
/* Disable movement on a clickable form. */
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
/* Form initialisation structure */
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
/* The basic init entries */
// label text
// Tool tip for the label.
//	PROP_FONT	*psFont;		// label font
// ID of the IVIS font to use for this widget.
/* Button initialisation structure */
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
/* Edit box initialisation structure */
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
/* The basic init entries */
// Orientation of the slider
// Number of stops on the slider
// Size of the bar
// Initial position of the slider bar
// Tip string
/* **********************************************************************************/
/* The maximum lenth of strings for the widget system */
/* The maximum value for bar graph size */
/* Structure to specify the heap sizes for the widget library */
// bar graph heap
// button heap
// edit box heap
// form heap
// clicable form heap
// tab form heap
// label heap
// slider heap
/* Initialise the widget module */
/* Reset the widget module */
/* Shut down the widget module */
/* Create an empty widget screen */
/* Release a screen and all its associated data */
/* Set the tool tip font for a screen */
/* Add a form to the widget screen */
/* Add a label to the widget screen */
/* Add a button to a form */
/* Add an edit box to a form */
/* Add a bar graph to a form */
/* Add a slider to a form */
/* Delete a widget from the screen */
/* Hide a widget */
/* Reveal a widget */
/* Return a pointer to a buffer containing the current string of a widget if any.
 * This will always return a valid string pointer.
 * NOTE: The string must be copied out of the buffer
 */
/* Set the text in a widget */
/* Set the current tabs for a tab form */
/* Get the current tabs for a tab form */
/* Get the current position of a widget */
/* Get the current position of a slider bar */
/* Set the current position of a slider bar */
/* Set the current size of a bar graph */
/* Set the current size of a minor bar on a double graph */
/* Set the current size of a minor bar on a double graph */
#[no_mangle]
pub unsafe extern "C" fn widgSetMinorBarSize(mut psScreen: *mut W_SCREEN,
                                             mut id: UDWORD,
                                             mut iValue: UDWORD) {
    let mut psBGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    let mut size: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetBarSize: Invalid screen pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bar.c\x00" as *const u8 as *const libc::c_char,
              193 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"widgSetMinorBarSize\x00")).as_ptr(),
              b"PTRVALID(psScreen, sizeof(W_SCREEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psBGraph = widgGetFromID(psScreen, id) as *mut W_BARGRAPH;
    if psBGraph.is_null() ||
           (*psBGraph).type_0 as libc::c_uint !=
               WIDG_BARGRAPH as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetBarSize: Couldn\'t find widget from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"bar.c\x00" as *const u8 as *const libc::c_char,
                  198 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"widgSetMinorBarSize\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    size =
        (100 as libc::c_int as
             libc::c_uint).wrapping_mul(iValue).wrapping_div((*psBGraph).iRange
                                                                 as
                                                                 libc::c_uint);
    if size > 100 as libc::c_int as libc::c_uint {
        size = 100 as libc::c_int as UDWORD
    }
    (*psBGraph).minorSize = size as UWORD;
}
/* Respond to a mouse moving over a barGraph */
/* Respond to a mouse moving over a barGraph */
#[no_mangle]
pub unsafe extern "C" fn barGraphHiLite(mut psWidget: *mut W_BARGRAPH,
                                        mut psContext: *mut W_CONTEXT) {
    if !(*psWidget).pTip.is_null() {
        tipStart(psWidget as *mut WIDGET, (*psWidget).pTip,
                 (*(*psContext).psScreen).TipFontID,
                 (*(*psContext).psForm).aColours.as_mut_ptr(),
                 (*psWidget).x as libc::c_int + (*psContext).xOffset,
                 (*psWidget).y as libc::c_int + (*psContext).yOffset,
                 (*psWidget).width as UDWORD, (*psWidget).height as UDWORD);
    };
}
/* Respond to the mouse moving off a barGraph */
/* Respond to the mouse moving off a barGraph */
#[no_mangle]
pub unsafe extern "C" fn barGraphHiLiteLost(mut psWidget: *mut W_BARGRAPH) {
    tipStop(psWidget as *mut WIDGET);
}
/* The bar graph display function */
/* The simple bar graph display function */
#[no_mangle]
pub unsafe extern "C" fn barGraphDisplay(mut psWidget: *mut WIDGET,
                                         mut xOffset: UDWORD,
                                         mut yOffset: UDWORD,
                                         mut pColours: *mut UDWORD) {
    let mut x0: SDWORD = 0 as libc::c_int;
    let mut y0: SDWORD = 0 as libc::c_int;
    let mut x1: SDWORD = 0 as libc::c_int;
    let mut y1: SDWORD = 0 as libc::c_int;
    let mut psBGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    psBGraph = psWidget as *mut W_BARGRAPH;
    /* figure out which way the bar graph fills */
    match (*psBGraph).barPos as libc::c_int {
        1 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                x0 +
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 = y0 + (*psWidget).height as libc::c_int
        }
        2 => {
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                xOffset.wrapping_add((*psWidget).x as
                                         libc::c_uint).wrapping_add((*psWidget).width
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            x0 =
                x1 -
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 = y0 + (*psWidget).height as libc::c_int
        }
        3 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 = x0 + (*psWidget).width as libc::c_int;
            y1 =
                y0 +
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int
        }
        4 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            x1 = x0 + (*psWidget).width as libc::c_int;
            y1 =
                yOffset.wrapping_add((*psWidget).y as
                                         libc::c_uint).wrapping_add((*psWidget).height
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            y0 =
                y1 -
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int
        }
        _ => { }
    }
    /* Now draw the graph */
    pie_BoxFillIndex(x0, y0, x1, y1, (*psBGraph).majorCol);
    pie_Line(x0, y1, x0, y0,
             *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    pie_Line(x0, y0, x1, y0,
             *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    pie_Line(x1, y0, x1, y1,
             *pColours.offset(WCOL_DARK as libc::c_int as isize));
    pie_Line(x0, y1, x1, y1,
             *pColours.offset(WCOL_DARK as libc::c_int as isize));
}
/* The double bar graph display function */
/* The double bar graph display function */
#[no_mangle]
pub unsafe extern "C" fn barGraphDisplayDouble(mut psWidget: *mut WIDGET,
                                               mut xOffset: UDWORD,
                                               mut yOffset: UDWORD,
                                               mut pColours: *mut UDWORD) {
    let mut x0: SDWORD = 0 as libc::c_int;
    let mut y0: SDWORD = 0 as libc::c_int;
    let mut x1: SDWORD = 0 as libc::c_int;
    let mut y1: SDWORD = 0 as libc::c_int;
    let mut x2: SDWORD = 0 as libc::c_int;
    let mut y2: SDWORD = 0 as libc::c_int;
    let mut x3: SDWORD = 0 as libc::c_int;
    let mut y3: SDWORD = 0 as libc::c_int;
    let mut psBGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    psBGraph = psWidget as *mut W_BARGRAPH;
    /* figure out which way the bar graph fills */
    match (*psBGraph).barPos as libc::c_int {
        1 => {
            /* Calculate the major bar */
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                x0 +
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 =
                y0 +
                    2 as libc::c_int * (*psWidget).height as libc::c_int /
                        3 as libc::c_int;
            /* Calculate the minor bar */
            x2 = x0;
            y2 = y0 + (*psWidget).height as libc::c_int / 3 as libc::c_int;
            x3 =
                x2 +
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).minorSize as libc::c_int /
                        100 as libc::c_int;
            y3 = y0 + (*psWidget).height as libc::c_int
        }
        2 => {
            /* Calculate the major bar */
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                xOffset.wrapping_add((*psWidget).x as
                                         libc::c_uint).wrapping_add((*psWidget).width
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            x0 =
                x1 -
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 =
                y0 +
                    2 as libc::c_int * (*psWidget).height as libc::c_int /
                        3 as libc::c_int;
            /* Calculate the minor bar */
            x3 = x1;
            y2 = y0 + (*psWidget).height as libc::c_int / 3 as libc::c_int;
            x2 =
                x3 -
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).minorSize as libc::c_int /
                        100 as libc::c_int;
            y3 = y0 + (*psWidget).height as libc::c_int
        }
        3 => {
            /* Calculate the major bar */
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                x0 +
                    2 as libc::c_int * (*psWidget).width as libc::c_int /
                        3 as libc::c_int;
            y1 =
                y0 +
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            /* Calculate the minor bar */
            x2 = x0 + (*psWidget).width as libc::c_int / 3 as libc::c_int;
            y2 = y0;
            x3 = x0 + (*psWidget).width as libc::c_int;
            y3 =
                y2 +
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).minorSize as libc::c_int /
                        100 as libc::c_int
        }
        4 => {
            /* Calculate the major bar */
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            x1 =
                x0 +
                    2 as libc::c_int * (*psWidget).width as libc::c_int /
                        3 as libc::c_int;
            y1 =
                yOffset.wrapping_add((*psWidget).y as
                                         libc::c_uint).wrapping_add((*psWidget).height
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            y0 =
                y1 -
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            /* Calculate the minor bar */
            x2 = x0 + (*psWidget).width as libc::c_int / 3 as libc::c_int;
            x3 = x0 + (*psWidget).width as libc::c_int;
            y3 = y1;
            y2 =
                y3 -
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).minorSize as libc::c_int /
                        100 as libc::c_int
        }
        _ => { }
    }
    /* Draw the minor bar graph */
    if (*psBGraph).minorSize as libc::c_int > 0 as libc::c_int {
        pie_BoxFillIndex(x2, y2, x3, y3, (*psBGraph).minorCol);
        pie_Line(x2, y3, x2, y2,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x2, y2, x3, y2,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x3, y2, x3, y3,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x2, y3, x3, y3,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
    }
    /* Draw the major bar graph */
    pie_BoxFillIndex(x0, y0, x1, y1, (*psBGraph).majorCol);
    pie_Line(x0, y1, x0, y0,
             *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    pie_Line(x0, y0, x1, y0,
             *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    pie_Line(x1, y0, x1, y1,
             *pColours.offset(WCOL_DARK as libc::c_int as isize));
    pie_Line(x0, y1, x1, y1,
             *pColours.offset(WCOL_DARK as libc::c_int as isize));
}
/* The trough bar graph display function */
/* The trough bar graph display function */
#[no_mangle]
pub unsafe extern "C" fn barGraphDisplayTrough(mut psWidget: *mut WIDGET,
                                               mut xOffset: UDWORD,
                                               mut yOffset: UDWORD,
                                               mut pColours: *mut UDWORD) {
    let mut x0: SDWORD = 0 as libc::c_int; // Position of the bar
    let mut y0: SDWORD = 0 as libc::c_int; // Position of the trough
    let mut x1: SDWORD = 0 as libc::c_int;
    let mut y1: SDWORD = 0 as libc::c_int;
    let mut tx0: SDWORD = 0 as libc::c_int;
    let mut ty0: SDWORD = 0 as libc::c_int;
    let mut tx1: SDWORD = 0 as libc::c_int;
    let mut ty1: SDWORD = 0 as libc::c_int;
    let mut psBGraph: *mut W_BARGRAPH = 0 as *mut W_BARGRAPH;
    let mut showBar: BOOL = 1 as libc::c_int;
    let mut showTrough: BOOL = 1 as libc::c_int;
    psBGraph = psWidget as *mut W_BARGRAPH;
    /* figure out which way the bar graph fills */
    match (*psBGraph).barPos as libc::c_int {
        1 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                x0 +
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 = y0 + (*psWidget).height as libc::c_int;
            if x0 == x1 { showBar = 0 as libc::c_int }
            tx0 = x1 + 1 as libc::c_int;
            ty0 = y0;
            tx1 = x0 + (*psWidget).width as libc::c_int;
            ty1 = y1;
            if tx0 >= tx1 { showTrough = 0 as libc::c_int }
        }
        2 => {
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 =
                xOffset.wrapping_add((*psWidget).x as
                                         libc::c_uint).wrapping_add((*psWidget).width
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            x0 =
                x1 -
                    (*psWidget).width as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            y1 = y0 + (*psWidget).height as libc::c_int;
            if x0 == x1 { showBar = 0 as libc::c_int }
            tx0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            ty0 = y0;
            tx1 = x0 - 1 as libc::c_int;
            ty1 = y1;
            if tx0 >= tx1 { showTrough = 0 as libc::c_int }
        }
        3 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            y0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            x1 = x0 + (*psWidget).width as libc::c_int;
            y1 =
                y0 +
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            if y0 == y1 { showBar = 0 as libc::c_int }
            tx0 = x0;
            ty0 = y1 + 1 as libc::c_int;
            tx1 = x1;
            ty1 = y0 + (*psWidget).height as libc::c_int;
            if ty0 >= ty1 { showTrough = 0 as libc::c_int }
        }
        4 => {
            x0 =
                xOffset.wrapping_add((*psWidget).x as libc::c_uint) as SDWORD;
            x1 = x0 + (*psWidget).width as libc::c_int;
            y1 =
                yOffset.wrapping_add((*psWidget).y as
                                         libc::c_uint).wrapping_add((*psWidget).height
                                                                        as
                                                                        libc::c_uint)
                    as SDWORD;
            y0 =
                y1 -
                    (*psWidget).height as libc::c_int *
                        (*psBGraph).majorSize as libc::c_int /
                        100 as libc::c_int;
            if y0 == y1 { showBar = 0 as libc::c_int }
            tx0 = x0;
            ty0 =
                yOffset.wrapping_add((*psWidget).y as libc::c_uint) as SDWORD;
            tx1 = x1;
            ty1 = y0 - 1 as libc::c_int;
            if ty0 >= ty1 { showTrough = 0 as libc::c_int }
        }
        _ => { }
    }
    /* Now draw the graph */
    if showBar != 0 {
        pie_BoxFillIndex(x0, y0, x1, y1, (*psBGraph).majorCol);
    }
    if showTrough != 0 {
        pie_BoxFillIndex(tx0, ty0, tx1, ty1,
                         WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(tx0, ty1, tx0, ty0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(tx0, ty0, tx1, ty0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(tx1, ty0, tx1, ty1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(tx0, ty1, tx1, ty1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    };
}
