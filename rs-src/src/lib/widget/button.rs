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
    // Get pointer to current audio callback function.
    #[no_mangle]
    fn WidgGetAudioCallback() -> WIDGET_AUDIOCALLBACK;
    // Get current audio ID for hilight.
    #[no_mangle]
    fn WidgGetHilightAudioID() -> SWORD;
    // Get current audio ID for clicked.
    #[no_mangle]
    fn WidgGetClickedAudioID() -> SWORD;
    /* Set the id number for widgRunScreen to return */
    #[no_mangle]
    fn widgSetReturn(psWidget: *mut WIDGET);
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
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
    // Valid values for "Justify" argument of pie_DrawFormattedText().
    // Left justify.
    // Centre justify.
    // Right justify.
    // Start from end of last print and then left justify.
    // Valid values for paramaters for pie_SetFormattedTextFlags().
    // Skip leading spaces at the start of each line of text. Improves centre justification
// but may result in unwanted word breaks.
    // Skip trailing spaces at the end of each line of text, improves centre justification.
    // Inserts a space before the first word in the string, usefull when use FTEXT_LEFTJUSTIFYAPPEND
    #[no_mangle]
    fn pie_DrawText(string: *mut STRING, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn iV_GetTextAboveBase() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    static mut gameTime2: UDWORD;
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
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
// Extension memory for the heap
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Button initialisation structure */
pub type W_BUTINIT = _w_butinit;
/* The basic init entries */
// button text
// Tool tip text
//	PROP_FONT	*psFont;		// button font
// ID of the IVIS font to use for this widget.
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
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
// Button is flashing
pub type W_BUTTON = _w_button;
pub type uint32 = libc::c_uint;
/* The common widget data */
// The current button state
// The text for the button
// The tool tip for the button
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
//	PROP_FONT	*psFont;			// button font
/*
 * Button.c
 *
 * Functions for the button widget
 */
// FIXME Direct iVis implementation include!
/* The widget heap */
#[no_mangle]
pub static mut psButHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Initialise the button module */
/* Initialise the button module */
#[no_mangle]
pub unsafe extern "C" fn buttonStartUp() -> BOOL { return 1 as libc::c_int; }
/* Create a button widget data structure */
/* Create a button widget data structure */
#[no_mangle]
pub unsafe extern "C" fn buttonCreate(mut ppsWidget: *mut *mut W_BUTTON,
                                      mut psInit: *mut W_BUTINIT) -> BOOL {
    if (*psInit).style &
           !(0 as libc::c_int | 0x8000 as libc::c_int | 8 as libc::c_int |
                 0x10 as libc::c_int | 0x20 as libc::c_int |
                 64 as libc::c_int) as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Unknown button style\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"button.c\x00" as *const u8 as *const libc::c_char,
                  33 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"buttonCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //#ifdef DEBUG
//	if (psInit->pText)
//	{
//		ASSERT( PTRVALID(psInit->psFont, sizeof(PROP_FONT)),
//			"buttonCreate: Invalid font pointer" );
//	}
//#endif
    /* Allocate the required memory */
    if heapAlloc(psButHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"buttonCreate: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"button.c\x00" as *const u8 as *const libc::c_char,
                  53 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"buttonCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate memory for the text and copy it if necessary */
    if !(*psInit).pText.is_null() {
        (**ppsWidget).pText = (*psInit).pText
    } else { (**ppsWidget).pText = 0 as *mut STRING }
    /* Allocate the memory for the tip and copy it if necessary */
    if !(*psInit).pTip.is_null() {
        (**ppsWidget).pTip = (*psInit).pTip
    } else { (**ppsWidget).pTip = 0 as *mut STRING }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_BUTTON;
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
    (**ppsWidget).AudioCallback = WidgGetAudioCallback();
    (**ppsWidget).HilightAudioID = WidgGetHilightAudioID();
    (**ppsWidget).ClickedAudioID = WidgGetClickedAudioID();
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else {
        (**ppsWidget).display =
            Some(buttonDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    //	(*ppsWidget)->psFont = psInit->psFont;
    (**ppsWidget).FontID = (*psInit).FontID;
    buttonInitialise(*ppsWidget);
    return 1 as libc::c_int;
}
/* Free the memory used by a button */
/* Free the memory used by a button */
#[no_mangle]
pub unsafe extern "C" fn buttonFree(mut psWidget: *mut W_BUTTON) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"buttonFree: invalid button pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"button.c\x00" as *const u8 as *const libc::c_char,
              135 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"buttonFree\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_BUTTON))\x00" as *const u8 as
                  *const libc::c_char);
    };
    heapFree(psButHeap, psWidget as *mut libc::c_void);
}
/* Initialise a button widget before running it */
/* Initialise a button widget before it is run */
#[no_mangle]
pub unsafe extern "C" fn buttonInitialise(mut psWidget: *mut W_BUTTON) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"buttonDisplay: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"button.c\x00" as *const u8 as *const libc::c_char,
              160 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"buttonInitialise\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_BUTTON))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psWidget).state = 0 as libc::c_int as UDWORD;
}
/* Get a button's state */
/* Get a button's state */
#[no_mangle]
pub unsafe extern "C" fn buttonGetState(mut psButton: *mut W_BUTTON)
 -> UDWORD {
    let mut State: UDWORD = 0 as libc::c_int as UDWORD;
    if (*psButton).state & 0x2 as libc::c_int as libc::c_uint != 0 {
        State |= 0x1 as libc::c_int as libc::c_uint
    }
    if (*psButton).state & 0x8 as libc::c_int as libc::c_uint != 0 {
        State |= 0x2 as libc::c_int as libc::c_uint
    }
    if (*psButton).state & 0x10 as libc::c_int as libc::c_uint != 0 {
        State |= 0x4 as libc::c_int as libc::c_uint
    }
    if (*psButton).state & 0x20 as libc::c_int as libc::c_uint != 0 {
        State |= 0x8 as libc::c_int as libc::c_uint
    }
    return State;
}
#[no_mangle]
pub unsafe extern "C" fn buttonSetFlash(mut psButton: *mut W_BUTTON) {
    (*psButton).state |= 0x20 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn buttonClearFlash(mut psButton: *mut W_BUTTON) {
    (*psButton).state &= !(0x20 as libc::c_int) as libc::c_uint;
    (*psButton).state &= !(0x40 as libc::c_int) as libc::c_uint;
}
/* Set a button's state */
/* Set a button's state */
#[no_mangle]
pub unsafe extern "C" fn buttonSetState(mut psButton: *mut W_BUTTON,
                                        mut state: UDWORD) {
    if !(state & 0x2 as libc::c_int as libc::c_uint != 0 &&
             state & 0x4 as libc::c_int as libc::c_uint != 0) {
    } else {
        debug(LOG_ERROR,
              b"widgSetButtonState: Cannot have WBUT_LOCK and WBUT_CLICKLOCK\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !(state & 0x2 as libc::c_int as libc::c_uint != 0 &&
             state & 0x4 as libc::c_int as libc::c_uint != 0) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"button.c\x00" as *const u8 as *const libc::c_char,
              212 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"buttonSetState\x00")).as_ptr(),
              b"!((state & WBUT_LOCK) && (state & WBUT_CLICKLOCK))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if state & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*psButton).state |= 0x2 as libc::c_int as libc::c_uint
    } else { (*psButton).state &= !(0x2 as libc::c_int) as libc::c_uint }
    if state & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*psButton).state |= 0x8 as libc::c_int as libc::c_uint
    } else { (*psButton).state &= !(0x8 as libc::c_int) as libc::c_uint }
    if state & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*psButton).state |= 0x10 as libc::c_int as libc::c_uint
    } else { (*psButton).state &= !(0x10 as libc::c_int) as libc::c_uint };
}
/* Run a button widget */
/* Run a button widget */
#[no_mangle]
pub unsafe extern "C" fn buttonRun(mut psButton: *mut W_BUTTON) {
    //	(void)psButton;
    if (*psButton).state & 0x20 as libc::c_int as libc::c_uint != 0 {
        if gameTime2.wrapping_div(250 as libc::c_int as
                                      libc::c_uint).wrapping_rem(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
               == 0 as libc::c_int as libc::c_uint {
            (*psButton).state &= !(0x40 as libc::c_int) as libc::c_uint
        } else { (*psButton).state |= 0x40 as libc::c_int as libc::c_uint }
    };
}
/* Respond to a mouse click */
/* Respond to a mouse click */
#[no_mangle]
pub unsafe extern "C" fn buttonClicked(mut psWidget: *mut W_BUTTON,
                                       mut key: UDWORD) {
    /* Can't click a button if it is disabled or locked down */
    if (*psWidget).state &
           (0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint == 0 {
        // Check this is the correct key
        if (*psWidget).style & 0x10 as libc::c_int as libc::c_uint == 0 &&
               key == 1 as libc::c_int as libc::c_uint ||
               (*psWidget).style & 0x20 as libc::c_int as libc::c_uint != 0 &&
                   key == 2 as libc::c_int as libc::c_uint {
            if (*psWidget).AudioCallback.is_some() {
                (*psWidget).AudioCallback.expect("non-null function pointer")((*psWidget).ClickedAudioID
                                                                                  as
                                                                                  libc::c_int); // Stop it flashing
            }
            (*psWidget).state &= !(0x20 as libc::c_int) as libc::c_uint;
            (*psWidget).state &= !(0x40 as libc::c_int) as libc::c_uint;
            (*psWidget).state |= 0x1 as libc::c_int as libc::c_uint
        }
    }
    /* Kill the tip if there is one */
    if !(*psWidget).pTip.is_null() { tipStop(psWidget as *mut WIDGET); };
}
/* Respond to a mouse button up */
/* Respond to a mouse button up */
#[no_mangle]
pub unsafe extern "C" fn buttonReleased(mut psWidget: *mut W_BUTTON,
                                        mut key: UDWORD) {
    if (*psWidget).state & 0x1 as libc::c_int as libc::c_uint != 0 {
        // Check this is the correct key
        if (*psWidget).style & 0x10 as libc::c_int as libc::c_uint == 0 &&
               key == 1 as libc::c_int as libc::c_uint ||
               (*psWidget).style & 0x20 as libc::c_int as libc::c_uint != 0 &&
                   key == 2 as libc::c_int as libc::c_uint {
            widgSetReturn(psWidget as *mut WIDGET);
            (*psWidget).state &= !(0x1 as libc::c_int) as libc::c_uint
        }
    };
}
/* Respond to a mouse moving over a button */
/* Respond to a mouse moving over a button */
#[no_mangle]
pub unsafe extern "C" fn buttonHiLite(mut psWidget: *mut W_BUTTON,
                                      mut psContext: *mut W_CONTEXT) {
    (*psWidget).state |= 0x4 as libc::c_int as libc::c_uint;
    if (*psWidget).AudioCallback.is_some() {
        (*psWidget).AudioCallback.expect("non-null function pointer")((*psWidget).HilightAudioID
                                                                          as
                                                                          libc::c_int);
    }
    /* If there is a tip string start the tool tip */
    if !(*psWidget).pTip.is_null() {
        tipStart(psWidget as *mut WIDGET, (*psWidget).pTip,
                 (*(*psContext).psScreen).TipFontID,
                 (*(*psContext).psForm).aColours.as_mut_ptr(),
                 (*psWidget).x as libc::c_int + (*psContext).xOffset,
                 (*psWidget).y as libc::c_int + (*psContext).yOffset,
                 (*psWidget).width as UDWORD, (*psWidget).height as UDWORD);
    };
}
/* Respond to the mouse moving off a button */
/* Respond to the mouse moving off a button */
#[no_mangle]
pub unsafe extern "C" fn buttonHiLiteLost(mut psWidget: *mut W_BUTTON) {
    (*psWidget).state &=
        !(0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint;
    if !(*psWidget).pTip.is_null() { tipStop(psWidget as *mut WIDGET); };
}
/* The button display function */
/* Display a button */
#[no_mangle]
pub unsafe extern "C" fn buttonDisplay(mut psWidget: *mut WIDGET,
                                       mut xOffset: UDWORD,
                                       mut yOffset: UDWORD,
                                       mut pColours: *mut UDWORD) {
    let mut psButton: *mut W_BUTTON = 0 as *mut W_BUTTON;
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut fx: SDWORD = 0;
    let mut fy: SDWORD = 0;
    let mut fw: SDWORD = 0;
    //	PROP_FONT	*psCurrFont;
    let mut CurrFontID: libc::c_int = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"buttonDisplay: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"button.c\x00" as *const u8 as *const libc::c_char,
              339 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"buttonDisplay\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_BUTTON))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psButton = psWidget as *mut W_BUTTON;
    //	psCurrFont = psButton->psFont;
    CurrFontID = (*psButton).FontID;
    x0 = ((*psButton).x as libc::c_uint).wrapping_add(xOffset) as SDWORD;
    y0 = ((*psButton).y as libc::c_uint).wrapping_add(yOffset) as SDWORD;
    x1 = x0 + (*psButton).width as libc::c_int;
    y1 = y0 + (*psButton).height as libc::c_int;
    if (*psButton).state &
           (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        /* Display the button down */
        pie_BoxFillIndex(x0, y0, x1, y1, WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0, y0, x1, y0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x0, y0, x0, y1,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x0, y1, x1, y1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1, y1, x1, y0,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        if !(*psButton).pText.is_null() {
            iV_SetFont((*psButton).FontID);
            iV_SetTextColour(*pColours.offset(WCOL_TEXT as libc::c_int as
                                                  isize) as UWORD as SWORD);
            fw = iV_GetTextWidth((*psButton).pText);
            if (*psButton).style & 8 as libc::c_int as libc::c_uint != 0 {
                fx =
                    x0 +
                        ((*psButton).width as libc::c_int - fw) /
                            2 as libc::c_int + 1 as libc::c_int;
                fy =
                    y0 + 1 as libc::c_int +
                        ((*psButton).height as libc::c_int -
                             iV_GetTextLineSize()) / 2 as libc::c_int -
                        iV_GetTextAboveBase()
            } else {
                fx =
                    x0 +
                        ((*psButton).width as libc::c_int - fw) /
                            2 as libc::c_int;
                fy =
                    y0 +
                        ((*psButton).height as libc::c_int -
                             iV_GetTextLineSize()) / 2 as libc::c_int -
                        iV_GetTextAboveBase()
            }
            pie_DrawText((*psButton).pText, fx as UDWORD, fy as UDWORD);
        }
        if (*psButton).state & 0x4 as libc::c_int as libc::c_uint != 0 {
            /* Display the button hilite */
            pie_Line(x0 + 3 as libc::c_int, y0 + 3 as libc::c_int,
                     x1 - 2 as libc::c_int, y0 + 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 3 as libc::c_int, y0 + 3 as libc::c_int,
                     x0 + 3 as libc::c_int, y1 - 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 3 as libc::c_int, y1 - 2 as libc::c_int,
                     x1 - 2 as libc::c_int, y1 - 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x1 - 2 as libc::c_int, y1 - 2 as libc::c_int,
                     x1 - 2 as libc::c_int, y0 + 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        }
    } else if (*psButton).state & 0x2 as libc::c_int as libc::c_uint != 0 {
        /* Display the disabled button */
        pie_BoxFillIndex(x0, y0, x1, y1, WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0, y0, x1, y0,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0, y0, x0, y1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0, y1, x1, y1,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1, y1, x1, y0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        if !(*psButton).pText.is_null() {
            iV_SetFont((*psButton).FontID);
            fw = iV_GetTextWidth((*psButton).pText);
            fx =
                x0 +
                    ((*psButton).width as libc::c_int - fw) /
                        2 as libc::c_int;
            fy =
                y0 +
                    ((*psButton).height as libc::c_int - iV_GetTextLineSize())
                        / 2 as libc::c_int - iV_GetTextAboveBase();
            iV_SetTextColour(*pColours.offset(WCOL_LIGHT as libc::c_int as
                                                  isize) as UWORD as SWORD);
            pie_DrawText((*psButton).pText, (fx + 1 as libc::c_int) as UDWORD,
                         (fy + 1 as libc::c_int) as UDWORD);
            iV_SetTextColour(*pColours.offset(WCOL_DISABLE as libc::c_int as
                                                  isize) as UWORD as SWORD);
            pie_DrawText((*psButton).pText, fx as UDWORD, fy as UDWORD);
        }
        if (*psButton).state & 0x4 as libc::c_int as libc::c_uint != 0 {
            /* Display the button hilite */
            pie_Line(x0 + 2 as libc::c_int, y0 + 2 as libc::c_int,
                     x1 - 3 as libc::c_int, y0 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 2 as libc::c_int, y0 + 2 as libc::c_int,
                     x0 + 2 as libc::c_int, y1 - 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 2 as libc::c_int, y1 - 3 as libc::c_int,
                     x1 - 3 as libc::c_int, y1 - 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x1 - 3 as libc::c_int, y1 - 3 as libc::c_int,
                     x1 - 3 as libc::c_int, y0 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        }
    } else {
        /* Display the button up */
        pie_BoxFillIndex(x0, y0, x1, y1, WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0, y0, x1, y0,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0, y0, x0, y1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0, y1, x1, y1,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1, y1, x1, y0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        //if (0)
        if !(*psButton).pText.is_null() {
            iV_SetFont((*psButton).FontID);
            iV_SetTextColour(*pColours.offset(WCOL_TEXT as libc::c_int as
                                                  isize) as UWORD as SWORD);
            fw = iV_GetTextWidth((*psButton).pText);
            fx =
                x0 +
                    ((*psButton).width as libc::c_int - fw) /
                        2 as libc::c_int;
            fy =
                y0 +
                    ((*psButton).height as libc::c_int - iV_GetTextLineSize())
                        / 2 as libc::c_int - iV_GetTextAboveBase();
            pie_DrawText((*psButton).pText, fx as UDWORD, fy as UDWORD);
        }
        if (*psButton).state & 0x4 as libc::c_int as libc::c_uint != 0 {
            /* Display the button hilite */
            pie_Line(x0 + 2 as libc::c_int, y0 + 2 as libc::c_int,
                     x1 - 3 as libc::c_int, y0 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 2 as libc::c_int, y0 + 2 as libc::c_int,
                     x0 + 2 as libc::c_int, y1 - 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 + 2 as libc::c_int, y1 - 3 as libc::c_int,
                     x1 - 3 as libc::c_int, y1 - 3 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x1 - 3 as libc::c_int, y1 - 3 as libc::c_int,
                     x1 - 3 as libc::c_int, y0 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        }
    };
}
