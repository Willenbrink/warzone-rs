use ::libc;
extern "C" {
    pub type _w_form;
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
    /* This returns true if the mouse key is currently depressed */
    #[no_mangle]
    fn mouseDown(code: MOUSE_KEY_CODE) -> BOOL;
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Set the id number for widgRunScreen to return */
    #[no_mangle]
    fn widgSetReturn(psWidget: *mut WIDGET);
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
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
}
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
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
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
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
pub type W_SLDINIT = _w_sldinit;
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
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
pub type _w_colour = libc::c_uint;
pub const WCOL_MAX: _w_colour = 8;
pub const WCOL_DISABLE: _w_colour = 7;
pub const WCOL_TIPBKGRND: _w_colour = 6;
pub const WCOL_CURSOR: _w_colour = 5;
pub const WCOL_HILITE: _w_colour = 4;
pub const WCOL_DARK: _w_colour = 3;
pub const WCOL_LIGHT: _w_colour = 2;
pub const WCOL_TEXT: _w_colour = 1;
pub const WCOL_BKGRND: _w_colour = 0;
/* The common widget data */
// The orientation of the slider
// Number of stop positions on the slider
// Thickness of slider bar
// Current stop position of the slider
// Slider state
// Tool tip
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
pub type uint32 = libc::c_uint;
/*
 * Slider.c
 *
 * Slide bar widget definitions.
 */
// FIXME Direct iVis implementation include!
/* The widget heaps */
#[no_mangle]
pub static mut psSldHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut DragEnabled: BOOL = 1 as libc::c_int;
/* Set the current size of a bar graph */
/* Set the current size of a minor bar on a double graph */
/* Return the ID of the widget the mouse was over this frame */
/* Return the user data for a widget */
/* Set the user data for a widget */
/* Return the user data for a widget */
/* Set the user data for a widget */
/* Return the user data for the returned widget */
/* Get widget structure */
/* Set tip string for a widget */
/* Colour numbers */
// Background colours
// Text colour
// Light colour for 3D effects
// Dark colour for 3D effects
// Hilite colour
// Edit Box cursor colour
// Background for the tool tip window
// Text colour on a disabled button
// all colour numbers are less than this
/* Set a colour on a form */
// Set the global toop tip text colour.
/* Possible states for a button */
// Disable (grey out) a button
// Fix a button down
// Fix a button down but it is still clickable
// Make a button flash.
/* Get a button or clickable form's state */
/* Set a button or clickable form's state */
/* The keys that can be used to press a button */
/* Return which key was used to press the last returned widget */
/* Initialise the set of widgets that make up a screen.
 * Call this once before calling widgRunScreen and widgDisplayScreen.
 * This should only be called once before calling Run and Display as many times
 * as is required.
 */
/* Clean up after a screen has been run.
 * Call this after the widgRunScreen / widgDisplayScreen cycle.
 */
/* Execute a set of widgets for one cycle.
 * Return the id of the widget that was activated, or 0 for none.
 */
/* Display the screen's widgets in their current state
 * (Call after calling widgRunScreen, this allows the input
 *  processing to be seperated from the display of the widgets).
 */
// Set the current audio callback function and audio id's.
// Get pointer to current audio callback function.
// Get current audio ID for hilight.
// Get current audio ID for clicked.
#[no_mangle]
pub unsafe extern "C" fn sliderEnableDrag(mut Enable: BOOL) {
    DragEnabled = Enable;
}
/* Create a slider widget data structure */
/* Create a slider widget data structure */
#[no_mangle]
pub unsafe extern "C" fn sliderCreate(mut ppsWidget: *mut *mut W_SLIDER,
                                      mut psInit: *mut W_SLDINIT) -> BOOL {
    if (*psInit).style &
           !(0 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: Unknown style\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  28 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if ((*psInit).orientation as libc::c_int) < 0x1 as libc::c_int ||
           (*psInit).orientation as libc::c_int > 0x4 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: Unknown orientation\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  34 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if ((*psInit).orientation as libc::c_int == 0x1 as libc::c_int ||
            (*psInit).orientation as libc::c_int == 0x2 as libc::c_int) &&
           (*psInit).numStops as libc::c_int >
               (*psInit).width as libc::c_int -
                   (*psInit).barSize as libc::c_int ||
           ((*psInit).orientation as libc::c_int == 0x3 as libc::c_int ||
                (*psInit).orientation as libc::c_int == 0x4 as libc::c_int) &&
               (*psInit).numStops as libc::c_int >
                   (*psInit).height as libc::c_int -
                       (*psInit).barSize as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: Too many stops for slider length\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  43 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).pos as libc::c_int > (*psInit).numStops as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: slider position greater than stops\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  49 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if ((*psInit).orientation as libc::c_int == 0x1 as libc::c_int ||
            (*psInit).orientation as libc::c_int == 0x2 as libc::c_int) &&
           (*psInit).barSize as libc::c_int > (*psInit).width as libc::c_int
           ||
           ((*psInit).orientation as libc::c_int == 0x3 as libc::c_int ||
                (*psInit).orientation as libc::c_int == 0x4 as libc::c_int) &&
               (*psInit).barSize as libc::c_int >
                   (*psInit).height as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: slider bar is larger than slider width\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  58 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate the required memory */
    if heapAlloc(psSldHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"sliderCreate: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"slider.c\x00" as *const u8 as *const libc::c_char,
                  70 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"sliderCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate the memory for the tip and copy it if necessary */
    if !(*psInit).pTip.is_null() {
        (**ppsWidget).pTip = (*psInit).pTip
    } else { (**ppsWidget).pTip = 0 as *mut STRING }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_SLIDER;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).x = (*psInit).x;
    (**ppsWidget).y = (*psInit).y;
    (**ppsWidget).width = (*psInit).width;
    (**ppsWidget).height = (*psInit).height;
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else {
        (**ppsWidget).display =
            Some(sliderDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    (**ppsWidget).orientation = (*psInit).orientation;
    (**ppsWidget).numStops = (*psInit).numStops;
    (**ppsWidget).barSize = (*psInit).barSize;
    sliderInitialise(*ppsWidget);
    (**ppsWidget).pos = (*psInit).pos;
    return 1 as libc::c_int;
}
/* Free the memory used by a slider */
/* Free the memory used by a slider */
#[no_mangle]
pub unsafe extern "C" fn sliderFree(mut psWidget: *mut W_SLIDER) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"sliderFree: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"slider.c\x00" as *const u8 as *const libc::c_char,
              129 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"sliderFree\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_SLIDER))\x00" as *const u8 as
                  *const libc::c_char);
    };
    heapFree(psSldHeap, psWidget as *mut libc::c_void);
}
/* Initialise a slider widget before running it */
/* Initialise a slider widget before running it */
#[no_mangle]
pub unsafe extern "C" fn sliderInitialise(mut psWidget: *mut W_SLIDER) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"sliderInitialise: Invalid slider pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"slider.c\x00" as *const u8 as *const libc::c_char,
              151 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"sliderInitialise\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_SLIDER))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psWidget).state = 0 as libc::c_int as UWORD;
    (*psWidget).pos = 0 as libc::c_int as UWORD;
}
/* Get the current position of a slider bar */
#[no_mangle]
pub unsafe extern "C" fn widgGetSliderPos(mut psScreen: *mut W_SCREEN,
                                          mut id: UDWORD) -> UDWORD {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgGetSliderPos: couldn\'t find widget from id\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"slider.c\x00" as *const u8 as *const libc::c_char,
              165 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"widgGetSliderPos\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_SLIDER))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psWidget.is_null() {
        return (*(psWidget as *mut W_SLIDER)).pos as UDWORD
    }
    return 0 as libc::c_int as UDWORD;
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
/* Set the current position of a slider bar */
#[no_mangle]
pub unsafe extern "C" fn widgSetSliderPos(mut psScreen: *mut W_SCREEN,
                                          mut id: UDWORD, mut pos: UWORD) {
    let mut psWidget: *mut WIDGET = 0 as *mut WIDGET;
    psWidget = widgGetFromID(psScreen, id);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgGetSliderPos: couldn\'t find widget from id\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"slider.c\x00" as *const u8 as *const libc::c_char,
              181 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"widgSetSliderPos\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_SLIDER))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psWidget.is_null() {
        if pos as libc::c_int >
               (*(psWidget as *mut W_SLIDER)).numStops as libc::c_int {
            (*(psWidget as *mut W_SLIDER)).pos =
                (*(psWidget as *mut W_SLIDER)).numStops
        } else { (*(psWidget as *mut W_SLIDER)).pos = pos }
    };
}
/* Return the current position of the slider bar on the widget */
unsafe extern "C" fn sliderGetBarBox(mut psSlider: *mut W_SLIDER,
                                     mut pX: *mut SWORD, mut pY: *mut SWORD,
                                     mut pWidth: *mut UWORD,
                                     mut pHeight: *mut UWORD) {
    match (*psSlider).orientation as libc::c_int {
        1 => {
            *pX =
                (((*psSlider).width as libc::c_int -
                      (*psSlider).barSize as libc::c_int) *
                     (*psSlider).pos as libc::c_int /
                     (*psSlider).numStops as libc::c_int) as SWORD;
            *pY = 0 as libc::c_int as SWORD;
            *pWidth = (*psSlider).barSize;
            *pHeight = (*psSlider).height
        }
        2 => {
            *pX =
                ((*psSlider).width as libc::c_int -
                     (*psSlider).barSize as libc::c_int -
                     ((*psSlider).width as libc::c_int -
                          (*psSlider).barSize as libc::c_int) *
                         (*psSlider).pos as libc::c_int /
                         (*psSlider).numStops as libc::c_int) as SWORD;
            *pY = 0 as libc::c_int as SWORD;
            *pWidth = (*psSlider).barSize;
            *pHeight = (*psSlider).height
        }
        3 => {
            *pX = 0 as libc::c_int as SWORD;
            *pY =
                (((*psSlider).height as libc::c_int -
                      (*psSlider).barSize as libc::c_int) *
                     (*psSlider).pos as libc::c_int /
                     (*psSlider).numStops as libc::c_int) as SWORD;
            *pWidth = (*psSlider).width;
            *pHeight = (*psSlider).barSize
        }
        4 => {
            *pX = 0 as libc::c_int as SWORD;
            *pY =
                ((*psSlider).height as libc::c_int -
                     (*psSlider).barSize as libc::c_int -
                     ((*psSlider).height as libc::c_int -
                          (*psSlider).barSize as libc::c_int) *
                         (*psSlider).pos as libc::c_int /
                         (*psSlider).numStops as libc::c_int) as SWORD;
            *pWidth = (*psSlider).width;
            *pHeight = (*psSlider).barSize
        }
        _ => { }
    };
}
/* Run a slider widget */
/* Run a slider widget */
#[no_mangle]
pub unsafe extern "C" fn sliderRun(mut psWidget: *mut W_SLIDER,
                                   mut psContext: *mut W_CONTEXT) {
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    let mut stopSize: UDWORD = 0;
    if (*psWidget).state as libc::c_int & 0x1 as libc::c_int != 0 &&
           mouseDown(MOUSE_LMB) == 0 {
        (*psWidget).state =
            ((*psWidget).state as libc::c_int & !(0x1 as libc::c_int)) as
                UWORD;
        widgSetReturn(psWidget as *mut WIDGET);
    } else if (*psWidget).state as libc::c_int & 0x1 as libc::c_int != 0 {
        /* Figure out where the drag box should be */
        mx = (*psContext).mx - (*psWidget).x as libc::c_int;
        my = (*psContext).my - (*psWidget).y as libc::c_int;
        match (*psWidget).orientation as libc::c_int {
            1 => {
                if mx <= (*psWidget).barSize as libc::c_int / 2 as libc::c_int
                   {
                    (*psWidget).pos = 0 as libc::c_int as UWORD
                } else if mx >=
                              (*psWidget).width as libc::c_int -
                                  (*psWidget).barSize as libc::c_int /
                                      2 as libc::c_int {
                    (*psWidget).pos = (*psWidget).numStops
                } else {
                    /* Mouse is in the middle of the slider, calculate which stop */
                    stopSize =
                        (((*psWidget).width as libc::c_int -
                              (*psWidget).barSize as libc::c_int) /
                             (*psWidget).numStops as libc::c_int) as UDWORD;
                    (*psWidget).pos =
                        (mx as
                             libc::c_uint).wrapping_add(stopSize.wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)).wrapping_sub(((*psWidget).barSize
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   /
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                                                  as
                                                                                                                  libc::c_uint).wrapping_mul((*psWidget).numStops
                                                                                                                                                 as
                                                                                                                                                 libc::c_uint).wrapping_div(((*psWidget).width
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 -
                                                                                                                                                                                 (*psWidget).barSize
                                                                                                                                                                                     as
                                                                                                                                                                                     libc::c_int)
                                                                                                                                                                                as
                                                                                                                                                                                libc::c_uint)
                            as UWORD
                }
            }
            2 => {
                if mx <= (*psWidget).barSize as libc::c_int / 2 as libc::c_int
                   {
                    (*psWidget).pos = (*psWidget).numStops
                } else if mx >=
                              (*psWidget).width as libc::c_int -
                                  (*psWidget).barSize as libc::c_int /
                                      2 as libc::c_int {
                    (*psWidget).pos = 0 as libc::c_int as UWORD
                } else {
                    /* Mouse is in the middle of the slider, calculate which stop */
                    stopSize =
                        (((*psWidget).width as libc::c_int -
                              (*psWidget).barSize as libc::c_int) /
                             (*psWidget).numStops as libc::c_int) as UDWORD;
                    (*psWidget).pos =
                        ((*psWidget).numStops as
                             libc::c_uint).wrapping_sub((mx as
                                                             libc::c_uint).wrapping_add(stopSize.wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_sub(((*psWidget).barSize
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   /
                                                                                                                                                   2
                                                                                                                                                       as
                                                                                                                                                       libc::c_int)
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint).wrapping_mul((*psWidget).numStops
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint).wrapping_div(((*psWidget).width
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                 -
                                                                                                                                                                                                                 (*psWidget).barSize
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_int)
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_uint))
                            as UWORD
                }
            }
            3 => {
                if my <= (*psWidget).barSize as libc::c_int / 2 as libc::c_int
                   {
                    (*psWidget).pos = 0 as libc::c_int as UWORD
                } else if my >=
                              (*psWidget).height as libc::c_int -
                                  (*psWidget).barSize as libc::c_int /
                                      2 as libc::c_int {
                    (*psWidget).pos = (*psWidget).numStops
                } else {
                    /* Mouse is in the middle of the slider, calculate which stop */
                    stopSize =
                        (((*psWidget).height as libc::c_int -
                              (*psWidget).barSize as libc::c_int) /
                             (*psWidget).numStops as libc::c_int) as UDWORD;
                    (*psWidget).pos =
                        (my as
                             libc::c_uint).wrapping_add(stopSize.wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)).wrapping_sub(((*psWidget).barSize
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   /
                                                                                                                   2
                                                                                                                       as
                                                                                                                       libc::c_int)
                                                                                                                  as
                                                                                                                  libc::c_uint).wrapping_mul((*psWidget).numStops
                                                                                                                                                 as
                                                                                                                                                 libc::c_uint).wrapping_div(((*psWidget).height
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 -
                                                                                                                                                                                 (*psWidget).barSize
                                                                                                                                                                                     as
                                                                                                                                                                                     libc::c_int)
                                                                                                                                                                                as
                                                                                                                                                                                libc::c_uint)
                            as UWORD
                }
            }
            4 => {
                if my <= (*psWidget).barSize as libc::c_int / 2 as libc::c_int
                   {
                    (*psWidget).pos = (*psWidget).numStops
                } else if my >=
                              (*psWidget).height as libc::c_int -
                                  (*psWidget).barSize as libc::c_int /
                                      2 as libc::c_int {
                    (*psWidget).pos = 0 as libc::c_int as UWORD
                } else {
                    /* Mouse is in the middle of the slider, calculate which stop */
                    stopSize =
                        (((*psWidget).height as libc::c_int -
                              (*psWidget).barSize as libc::c_int) /
                             (*psWidget).numStops as libc::c_int) as UDWORD;
                    (*psWidget).pos =
                        ((*psWidget).numStops as
                             libc::c_uint).wrapping_sub((my as
                                                             libc::c_uint).wrapping_add(stopSize.wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_sub(((*psWidget).barSize
                                                                                                                                                   as
                                                                                                                                                   libc::c_int
                                                                                                                                                   /
                                                                                                                                                   2
                                                                                                                                                       as
                                                                                                                                                       libc::c_int)
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint).wrapping_mul((*psWidget).numStops
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint).wrapping_div(((*psWidget).height
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                 -
                                                                                                                                                                                                                 (*psWidget).barSize
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_int)
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_uint))
                            as UWORD
                }
            }
            _ => { }
        }
    };
}
/* Respond to a mouse click */
/* Respond to a mouse click */
#[no_mangle]
pub unsafe extern "C" fn sliderClicked(mut psWidget: *mut W_SLIDER,
                                       mut psContext: *mut W_CONTEXT) {
    if DragEnabled != 0 {
        if (*psContext).mx >= (*psWidget).x as libc::c_int &&
               (*psContext).mx <=
                   (*psWidget).x as libc::c_int +
                       (*psWidget).width as libc::c_int &&
               (*psContext).my >= (*psWidget).y as libc::c_int &&
               (*psContext).my <=
                   (*psWidget).y as libc::c_int +
                       (*psWidget).height as libc::c_int {
            (*psWidget).state =
                ((*psWidget).state as libc::c_int | 0x1 as libc::c_int) as
                    UWORD
        }
    };
}
/* Respond to a mouse up */
/* Respond to a mouse up */
#[no_mangle]
pub unsafe extern "C" fn sliderReleased(mut psWidget: *mut W_SLIDER) { }
/* Respond to a mouse moving over a slider */
/* Respond to a mouse moving over a slider */
#[no_mangle]
pub unsafe extern "C" fn sliderHiLite(mut psWidget: *mut W_SLIDER) {
    (*psWidget).state =
        ((*psWidget).state as libc::c_int | 0x2 as libc::c_int) as UWORD;
}
/* Respond to the mouse moving off a slider */
/* Respond to the mouse moving off a slider */
#[no_mangle]
pub unsafe extern "C" fn sliderHiLiteLost(mut psWidget: *mut W_SLIDER) {
    (*psWidget).state =
        ((*psWidget).state as libc::c_int & !(0x2 as libc::c_int)) as UWORD;
}
/* The slider display function */
/* The slider display function */
#[no_mangle]
pub unsafe extern "C" fn sliderDisplay(mut psWidget: *mut WIDGET,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    let mut psSlider: *mut W_SLIDER = 0 as *mut W_SLIDER;
    let mut x0: SWORD = 0;
    let mut y0: SWORD = 0;
    let mut x1: SWORD = 0;
    let mut y1: SWORD = 0;
    let mut width: UWORD = 0;
    let mut height: UWORD = 0;
    psSlider = psWidget as *mut W_SLIDER;
    match (*psSlider).orientation as libc::c_int {
        1 | 2 => {
            /* Draw the line */
            x0 =
                ((*psSlider).x as
                     libc::c_uint).wrapping_add(xOffset).wrapping_add(((*psSlider).barSize
                                                                           as
                                                                           libc::c_int
                                                                           /
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               SWORD
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_uint)
                    as SWORD;
            y0 =
                ((*psSlider).y as
                     libc::c_uint).wrapping_add(yOffset).wrapping_add(((*psSlider).height
                                                                           as
                                                                           libc::c_int
                                                                           /
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               SWORD
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_uint)
                    as SWORD;
            x1 =
                (x0 as libc::c_int + (*psSlider).width as libc::c_int -
                     (*psSlider).barSize as libc::c_int) as SWORD;
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                     y0 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(x0 as libc::c_int, y0 as libc::c_int + 1 as libc::c_int,
                     x1 as libc::c_int, y0 as libc::c_int + 1 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            /* Now Draw the bar */
            sliderGetBarBox(psSlider, &mut x0, &mut y0, &mut width,
                            &mut height);
            x0 =
                ((x0 as libc::c_int + (*psSlider).x as libc::c_int) as
                     libc::c_uint).wrapping_add(xOffset) as SWORD;
            y0 =
                ((y0 as libc::c_int + (*psSlider).y as libc::c_int) as
                     libc::c_uint).wrapping_add(yOffset) as SWORD;
            x1 = (x0 as libc::c_int + width as libc::c_int) as SWORD;
            y1 = (y0 as libc::c_int + height as libc::c_int) as SWORD;
            pie_BoxFillIndex(x0 as libc::c_int, y0 as libc::c_int,
                             x1 as libc::c_int, y1 as libc::c_int,
                             WCOL_BKGRND as libc::c_int as UBYTE);
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                     y0 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x0 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(x0 as libc::c_int, y1 as libc::c_int, x1 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
        }
        3 | 4 => {
            /* Draw the line */
            x0 =
                ((*psSlider).x as
                     libc::c_uint).wrapping_add(xOffset).wrapping_add(((*psSlider).width
                                                                           as
                                                                           libc::c_int
                                                                           /
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               SWORD
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_uint)
                    as SWORD;
            y0 =
                ((*psSlider).y as
                     libc::c_uint).wrapping_add(yOffset).wrapping_add(((*psSlider).barSize
                                                                           as
                                                                           libc::c_int
                                                                           /
                                                                           2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               SWORD
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_uint)
                    as SWORD;
            y1 =
                (y0 as libc::c_int + (*psSlider).height as libc::c_int -
                     (*psSlider).barSize as libc::c_int) as SWORD;
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x0 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(x0 as libc::c_int + 1 as libc::c_int, y0 as libc::c_int,
                     x0 as libc::c_int + 1 as libc::c_int, y1 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            /* Now Draw the bar */
            sliderGetBarBox(psSlider, &mut x0, &mut y0, &mut width,
                            &mut height);
            x0 =
                ((x0 as libc::c_int + (*psSlider).x as libc::c_int) as
                     libc::c_uint).wrapping_add(xOffset) as SWORD;
            y0 =
                ((y0 as libc::c_int + (*psSlider).y as libc::c_int) as
                     libc::c_uint).wrapping_add(yOffset) as SWORD;
            x1 = (x0 as libc::c_int + width as libc::c_int) as SWORD;
            y1 = (y0 as libc::c_int + height as libc::c_int) as SWORD;
            pie_BoxFillIndex(x0 as libc::c_int, y0 as libc::c_int,
                             x1 as libc::c_int, y1 as libc::c_int,
                             WCOL_BKGRND as libc::c_int as UBYTE);
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                     y0 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            pie_Line(x0 as libc::c_int, y0 as libc::c_int, x0 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(x0 as libc::c_int, y1 as libc::c_int, x1 as libc::c_int,
                     y1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
        }
        _ => { }
    }
    if (*psSlider).state as libc::c_int & 0x2 as libc::c_int != 0 {
        x0 =
            ((*psWidget).x as
                 libc::c_uint).wrapping_add(xOffset).wrapping_sub(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                as SWORD;
        y0 =
            ((*psWidget).y as
                 libc::c_uint).wrapping_add(yOffset).wrapping_sub(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                as SWORD;
        x1 =
            (x0 as libc::c_int + (*psWidget).width as libc::c_int +
                 4 as libc::c_int) as SWORD;
        y1 =
            (y0 as libc::c_int + (*psWidget).height as libc::c_int +
                 4 as libc::c_int) as SWORD;
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y1 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x0 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_HILITE as libc::c_int as isize));
    };
}
