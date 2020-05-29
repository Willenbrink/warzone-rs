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
    /* This returns true if the mouse key went from being up to being down this frame */
    #[no_mangle]
    fn mousePressed(code: MOUSE_KEY_CODE) -> BOOL;
    /* The input buffer can contain normal character codes and these control codes */
    /* Some defines for keys that map into the normal character space */
    /* Return the next key press or 0 if no key in the buffer.
 * The key returned will have been remaped to the correct ascii code for the
 * windows key map.
 * All key presses are buffered up (including windows auto repeat).
 */
    #[no_mangle]
    fn inputGetKey() -> UDWORD;
    #[no_mangle]
    fn inputGetCharKey() -> libc::c_char;
    /* Clear the input buffer */
    #[no_mangle]
    fn inputClearBuffer();
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
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
    /* Copy one string to another
 * The string to copy will be truncated if it is longer than WIDG_MAXSTR.
 */
    #[no_mangle]
    fn widgCopyString(pDest: *mut STRING, pSrc: *mut STRING);
    /* Set the keyboard focus for the screen */
    #[no_mangle]
    fn screenSetFocus(psScreen: *mut W_SCREEN, psWidget: *mut WIDGET);
    /* Clear the keyboard focus */
    #[no_mangle]
    fn screenClearFocus(psScreen: *mut W_SCREEN);
    #[no_mangle]
    fn iV_GetCharWidth(Char: STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_SetFont(FontID: libc::c_int);
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
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
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
    fn iV_GetTextBelowBase() -> libc::c_int;
    #[no_mangle]
    fn init_scrap() -> libc::c_int;
    #[no_mangle]
    fn get_scrap(type_0: libc::c_int, dstlen: *mut libc::c_int,
                 dst: *mut *mut libc::c_char);
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
// Extension memory for the heap
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
pub type FONT_DISPLAY
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut STRING) -> ()>;
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Edit box initialisation structure */
pub type W_EDBINIT = _w_edbinit;
/* The basic init entries */
// initial contents of the edit box
//	PROP_FONT	*psFont;		// edit box font
// ID of the IVIS font to use for this widget.
// Optional callback to display the form.
// Optional callback to display a string.
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
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
// disable button from selection
pub type W_EDITBOX = _w_editbox;
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
/* The widget heap */
#[no_mangle]
pub static mut psEdbHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Create an edit box widget data structure */
/* Create an edit box widget data structure */
#[no_mangle]
pub unsafe extern "C" fn editBoxCreate(mut ppsWidget: *mut *mut W_EDITBOX,
                                       mut psInit: *mut W_EDBINIT) -> BOOL {
    if (*psInit).style &
           !(0 as libc::c_int | 0x8000 as libc::c_int | 1 as libc::c_int) as
               libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Unknown edit box style\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"editbox.c\x00" as *const u8 as *const libc::c_char,
                  45 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"editBoxCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //	ASSERT( PTRVALID(psInit->psFont, sizeof(PROP_FONT)),
//		"editBoxCreate: Invalid font pointer" );
    /* Allocate the required memory */
    if heapAlloc(psEdbHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
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
                  b"editbox.c\x00" as *const u8 as *const libc::c_char,
                  60 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"editBoxCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_EDITBOX;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).x = (*psInit).x;
    (**ppsWidget).y = (*psInit).y;
    (**ppsWidget).width = (*psInit).width;
    (**ppsWidget).height = (*psInit).height;
    //	(*ppsWidget)->psFont = psInit->psFont;
    (**ppsWidget).FontID = (*psInit).FontID;
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else {
        (**ppsWidget).display =
            Some(editBoxDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    (**ppsWidget).pBoxDisplay = (*psInit).pBoxDisplay;
    (**ppsWidget).pFontDisplay = (*psInit).pFontDisplay;
    (**ppsWidget).AudioCallback = WidgGetAudioCallback();
    (**ppsWidget).HilightAudioID = WidgGetHilightAudioID();
    (**ppsWidget).ClickedAudioID = WidgGetClickedAudioID();
    if !(*psInit).pText.is_null() {
        widgCopyString((**ppsWidget).aText.as_mut_ptr(), (*psInit).pText);
    } else {
        (**ppsWidget).aText[0 as libc::c_int as usize] =
            0 as libc::c_int as STRING
    }
    editBoxInitialise(*ppsWidget);
    init_scrap();
    return 1 as libc::c_int;
}
/* Free the memory used by an edit box */
/* Free the memory used by an edit box */
#[no_mangle]
pub unsafe extern "C" fn editBoxFree(mut psWidget: *mut W_EDITBOX) {
    heapFree(psEdbHeap, psWidget as *mut libc::c_void);
}
/* Initialise an edit box widget */
/* Initialise an edit box widget */
#[no_mangle]
pub unsafe extern "C" fn editBoxInitialise(mut psWidget: *mut W_EDITBOX) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"editBoxInitialise: Invalid edit box pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              125 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"editBoxInitialise\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_EDITBOX))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psWidget).state = 0x1 as libc::c_int as UDWORD;
    (*psWidget).printStart = 0 as libc::c_int as UWORD;
    iV_SetFont((*psWidget).FontID);
    fitStringStart((*psWidget).aText.as_mut_ptr(),
                   (*psWidget).width as UDWORD, &mut (*psWidget).printChars,
                   &mut (*psWidget).printWidth);
}
/* Insert a character into a text buffer */
unsafe extern "C" fn insertChar(mut pBuffer: *mut STRING,
                                mut pPos: *mut UDWORD, mut ch: STRING) {
    let mut pSrc: *mut STRING = 0 as *mut STRING;
    let mut pDest: *mut STRING = 0 as *mut STRING;
    let mut len: UDWORD = 0;
    let mut count: UDWORD = 0;
    if ch as libc::c_int == '\u{0}' as i32 { return }
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"insertChar: Invalid insertion point\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              144 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"insertChar\x00")).as_ptr(),
              b"*pPos <= strlen(pBuffer)\x00" as *const u8 as
                  *const libc::c_char);
    };
    len = strlen(pBuffer);
    if len == (80 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        /* Buffer is full */
        return
    }
    /* Move the end of the string up by one (including terminating \0) */
    count =
        len.wrapping_sub(*pPos).wrapping_add(1 as libc::c_int as
                                                 libc::c_uint);
    pSrc = pBuffer.offset(len as isize);
    pDest = pSrc.offset(1 as libc::c_int as isize);
    loop  {
        let fresh0 = count;
        count = count.wrapping_sub(1);
        if !(fresh0 != 0) { break ; }
        let fresh1 = pSrc;
        pSrc = pSrc.offset(-1);
        let fresh2 = pDest;
        pDest = pDest.offset(-1);
        *fresh2 = *fresh1
    }
    /* Insert the character */
    *pDest = ch;
    /* Update the insertion point */
    *pPos =
        (*pPos as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
            as UDWORD as UDWORD;
}
/* Put a character into a text buffer overwriting any text under the cursor */
unsafe extern "C" fn overwriteChar(mut pBuffer: *mut STRING,
                                   mut pPos: *mut UDWORD, mut ch: STRING) {
    let mut pDest: *mut STRING = 0 as *mut STRING;
    let mut len: UDWORD = 0;
    if ch as libc::c_int == '\u{0}' as i32 { return }
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"insertChar: Invalid insertion point\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              180 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"overwriteChar\x00")).as_ptr(),
              b"*pPos <= strlen(pBuffer)\x00" as *const u8 as
                  *const libc::c_char);
    };
    len = strlen(pBuffer);
    if len == (80 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        /* Buffer is full */
        return
    }
    /* Store the character */
    pDest = pBuffer.offset(*pPos as isize);
    *pDest = ch;
    if *pPos == len {
        /* At the end of the string, move the \0 up one */
        *pDest.offset(1 as libc::c_int as isize) = '\u{0}' as i32 as STRING
    }
    /* Update the insertion point */
    *pPos =
        (*pPos as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
            as UDWORD as UDWORD;
}
/* Put a character into a text buffer overwriting any text under the cursor */
unsafe extern "C" fn putSelection(mut pBuffer: *mut STRING,
                                  mut pPos: *mut UDWORD) {
    static mut scrap: *mut libc::c_char =
        0 as *const libc::c_char as *mut libc::c_char;
    let mut scraplen: libc::c_int = 0;
    get_scrap(('T' as i32) << 24 as libc::c_int |
                  ('E' as i32) << 16 as libc::c_int |
                  ('X' as i32) << 8 as libc::c_int |
                  ('T' as i32) << 0 as libc::c_int, &mut scraplen,
              &mut scrap);
    if scraplen > 0 as libc::c_int &&
           scraplen < 80 as libc::c_int - 2 as libc::c_int {
        strncpy(pBuffer, scrap, scraplen as libc::c_uint);
        *pBuffer.offset(scraplen as isize) = '\u{0}' as i32 as STRING;
        *pPos = scraplen as UDWORD
    };
}
/* Delete a character to the left of the position */
unsafe extern "C" fn delCharLeft(mut pBuffer: *mut STRING,
                                 mut pPos: *mut UDWORD) {
    let mut pSrc: *mut STRING = 0 as *mut STRING;
    let mut pDest: *mut STRING = 0 as *mut STRING;
    let mut len: UDWORD = 0;
    let mut count: UDWORD = 0;
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"delCharLeft: Invalid insertion point\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              227 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"delCharLeft\x00")).as_ptr(),
              b"*pPos <= strlen(pBuffer)\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Can't delete if we are at the start of the string */
    if *pPos == 0 as libc::c_int as libc::c_uint { return }
    len = strlen(pBuffer);
    /* Move the end of the string down by one */
    count =
        len.wrapping_sub(*pPos).wrapping_add(1 as libc::c_int as
                                                 libc::c_uint);
    pSrc = pBuffer.offset(*pPos as isize);
    pDest = pSrc.offset(-(1 as libc::c_int as isize));
    loop  {
        let fresh3 = count;
        count = count.wrapping_sub(1);
        if !(fresh3 != 0) { break ; }
        let fresh4 = pSrc;
        pSrc = pSrc.offset(1);
        let fresh5 = pDest;
        pDest = pDest.offset(1);
        *fresh5 = *fresh4
    }
    /* Update the insertion point */
    *pPos =
        (*pPos as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as UDWORD as UDWORD;
}
/* Delete a character to the right of the position */
unsafe extern "C" fn delCharRight(mut pBuffer: *mut STRING,
                                  mut pPos: *mut UDWORD) {
    let mut pSrc: *mut STRING = 0 as *mut STRING;
    let mut pDest: *mut STRING = 0 as *mut STRING;
    let mut len: UDWORD = 0;
    let mut count: UDWORD = 0;
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"delCharLeft: Invalid insertion point\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *pPos <= strlen(pBuffer) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              258 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"delCharRight\x00")).as_ptr(),
              b"*pPos <= strlen(pBuffer)\x00" as *const u8 as
                  *const libc::c_char);
    };
    len = strlen(pBuffer);
    /* Can't delete if we are at the end of the string */
    if *pPos == len { return }
    /* Move the end of the string down by one */
    count = len.wrapping_sub(*pPos);
    pDest = pBuffer.offset(*pPos as isize);
    pSrc = pDest.offset(1 as libc::c_int as isize);
    loop  {
        let fresh6 = count;
        count = count.wrapping_sub(1);
        if !(fresh6 != 0) { break ; }
        let fresh7 = pSrc;
        pSrc = pSrc.offset(1);
        let fresh8 = pDest;
        pDest = pDest.offset(1);
        *fresh8 = *fresh7
    };
}
// the other states
/* Calculate how much of the start of a string can fit into the edit box */
/* Calculate how much of the start of a string can fit into the edit box */
unsafe extern "C" fn fitStringStart(mut pBuffer: *mut STRING,
                                    mut boxWidth: UDWORD,
                                    mut pCount: *mut UWORD,
                                    mut pCharWidth: *mut UWORD) {
    let mut len: UDWORD = 0;
    let mut printWidth: UWORD = 0;
    let mut printChars: UWORD = 0;
    let mut width: UWORD = 0;
    let mut pCurr: *mut STRING = 0 as *mut STRING;
    //	PROP_FONT	*psCurrFont;
    len = strlen(pBuffer);
    printWidth = 0 as libc::c_int as UWORD;
    printChars = 0 as libc::c_int as UWORD;
    pCurr = pBuffer;
    //	psCurrFont = fontGet();
    /* Find the number of characters that will fit in boxWidth */
    while (printChars as libc::c_uint) < len {
        width =
            (printWidth as libc::c_int + iV_GetCharWidth(*pCurr)) as UWORD;
        if width as libc::c_uint >
               boxWidth.wrapping_sub((4 as libc::c_int * 2 as libc::c_int) as
                                         libc::c_uint) {
            break ;
        }
        printWidth = width;
        printChars = (printChars as libc::c_int + 1 as libc::c_int) as UWORD;
        pCurr = pCurr.offset(1 as libc::c_int as isize)
    }
    /* Return the number of characters and their width */
    *pCount = printChars;
    *pCharWidth = printWidth;
}
/* Calculate how much of the end of a string can fit into the edit box */
unsafe extern "C" fn fitStringEnd(mut pBuffer: *mut STRING,
                                  mut boxWidth: UDWORD,
                                  mut pStart: *mut UWORD,
                                  mut pCount: *mut UWORD,
                                  mut pCharWidth: *mut UWORD) {
    let mut len: UDWORD = 0;
    let mut printWidth: UWORD = 0;
    let mut printChars: UWORD = 0;
    let mut width: UWORD = 0;
    let mut pCurr: *mut STRING = 0 as *mut STRING;
    //	PROP_FONT	*psCurrFont;
    len = strlen(pBuffer);
    //	psCurrFont = fontGet();
    pCurr = pBuffer.offset(len as isize).offset(-(1 as libc::c_int as isize));
    printChars = 0 as libc::c_int as UWORD;
    printWidth = 0 as libc::c_int as UWORD;
    /* Find the number of characters that will fit in boxWidth */
    while (printChars as libc::c_uint) < len {
        width =
            (printWidth as libc::c_int + iV_GetCharWidth(*pCurr)) as UWORD;
        if width as libc::c_uint >
               boxWidth.wrapping_sub((4 as libc::c_int * 2 as libc::c_int +
                                          8 as libc::c_int) as libc::c_uint) {
            break ;
        }
        printWidth = width;
        printChars = (printChars as libc::c_int + 1 as libc::c_int) as UWORD;
        pCurr = pCurr.offset(-(1 as libc::c_int as isize))
    }
    /* Return the number of characters and their width */
    *pStart = len.wrapping_sub(printChars as libc::c_uint) as UWORD;
    *pCount = printChars;
    *pCharWidth = printWidth;
}
/* Run an edit box widget */
/* Run an edit box widget */
#[no_mangle]
pub unsafe extern "C" fn editBoxRun(mut psWidget: *mut W_EDITBOX,
                                    mut psContext: *mut W_CONTEXT) {
    let mut key: UDWORD = 0;
    let mut len: UDWORD = 0 as libc::c_int as UDWORD;
    let mut editState: UDWORD = 0;
    let mut pos: UDWORD = 0;
    let mut pBuffer: *mut STRING = 0 as *mut STRING;
    let mut done: BOOL = 0;
    let mut printStart: UWORD = 0;
    let mut printWidth: UWORD = 0;
    let mut printChars: UWORD = 0;
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    /* Note the edit state */
    editState = (*psWidget).state & 0xf as libc::c_int as libc::c_uint;
    /* Only have anything to do if the widget is being edited */
    if editState & 0xf as libc::c_int as libc::c_uint ==
           0x1 as libc::c_int as libc::c_uint {
        return
    }
    /* If there is a mouse click outside of the edit box - stop editing */
    mx = (*psContext).mx;
    my = (*psContext).my;
    if mousePressed(MOUSE_LMB) != 0 &&
           (mx < (*psWidget).x as libc::c_int ||
                mx >
                    (*psWidget).x as libc::c_int +
                        (*psWidget).width as libc::c_int ||
                my < (*psWidget).y as libc::c_int ||
                my >
                    (*psWidget).y as libc::c_int +
                        (*psWidget).height as libc::c_int) {
        screenClearFocus((*psContext).psScreen);
        return
    }
    /* note the widget state */
    pos = (*psWidget).insPos as UDWORD;
    pBuffer = (*psWidget).aText.as_mut_ptr();
    printStart = (*psWidget).printStart;
    printWidth = (*psWidget).printWidth;
    printChars = (*psWidget).printChars;
    iV_SetFont((*psWidget).FontID);
    /* Loop through the characters in the input buffer */
    done = 0 as libc::c_int;
    key = inputGetKey();
    while key != 0 as libc::c_int as libc::c_uint && done == 0 {
        /* Deal with all the control keys, assume anything else is a printable character */
        match key {
            65536 => {
                /* Move the cursor left */
                if pos > 0 as libc::c_int as libc::c_uint {
                    pos =
                        (pos as
                             libc::c_uint).wrapping_sub(1 as libc::c_int as
                                                            libc::c_uint) as
                            UDWORD as UDWORD
                }
                /* If the cursor has gone off the left of the edit box,
			 * need to update the printable text.
			 */
                if pos < printStart as libc::c_uint {
                    if printStart as libc::c_int <= 6 as libc::c_int {
                        /* Got to the start of the string */
                        printStart = 0 as libc::c_int as UWORD;
                        fitStringStart(pBuffer, (*psWidget).width as UDWORD,
                                       &mut printChars, &mut printWidth);
                    } else {
                        printStart =
                            (printStart as libc::c_int - 6 as libc::c_int) as
                                UWORD;
                        fitStringStart(pBuffer.offset(printStart as
                                                          libc::c_int as
                                                          isize),
                                       (*psWidget).width as UDWORD,
                                       &mut printChars, &mut printWidth);
                    }
                }
            }
            131072 => {
                /* Move the cursor right */
                len = strlen(pBuffer);
                if pos < len {
                    pos =
                        (pos as
                             libc::c_uint).wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                            UDWORD as UDWORD
                }
                /* If the cursor has gone off the right of the edit box,
			 * need to update the printable text.
			 */
                if pos >
                       (printStart as libc::c_int + printChars as libc::c_int)
                           as UDWORD {
                    printStart =
                        (printStart as libc::c_int + 6 as libc::c_int) as
                            UWORD;
                    if printStart as libc::c_uint >= len {
                        printStart =
                            len.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as UWORD
                    }
                    fitStringStart(pBuffer.offset(printStart as libc::c_int as
                                                      isize),
                                   (*psWidget).width as UDWORD,
                                   &mut printChars, &mut printWidth);
                }
            }
            327680 => {
                /* Move the cursor to the start of the buffer */
                pos = 0 as libc::c_int as UDWORD;
                printStart = 0 as libc::c_int as UWORD;
                fitStringStart(pBuffer, (*psWidget).width as UDWORD,
                               &mut printChars, &mut printWidth);
            }
            393216 => {
                /* Move the cursor to the end of the buffer */
                pos = strlen(pBuffer);
                if pos !=
                       (printStart as libc::c_int + printChars as libc::c_int)
                           as UWORD as libc::c_uint {
                    fitStringEnd(pBuffer, (*psWidget).width as UDWORD,
                                 &mut printStart, &mut printChars,
                                 &mut printWidth);
                }
            }
            458752 => {
                if editState == 0x2 as libc::c_int as libc::c_uint {
                    editState = 0x3 as libc::c_int as UDWORD
                } else { editState = 0x2 as libc::c_int as UDWORD }
            }
            524288 => {
                delCharRight(pBuffer, &mut pos);
                /* Update the printable text */
                fitStringStart(pBuffer.offset(printStart as libc::c_int as
                                                  isize),
                               (*psWidget).width as UDWORD, &mut printChars,
                               &mut printWidth);
            }
            8 => {
                /* Delete the character to the left of the cursor */
                delCharLeft(pBuffer, &mut pos);
                /* Update the printable text */
                if pos <= printStart as libc::c_uint {
                    if printStart as libc::c_int <= 6 as libc::c_int {
                        /* Got to the start of the string */
                        printStart = 0 as libc::c_int as UWORD;
                        fitStringStart(pBuffer, (*psWidget).width as UDWORD,
                                       &mut printChars, &mut printWidth);
                    } else {
                        printStart =
                            (printStart as libc::c_int - 6 as libc::c_int) as
                                UWORD;
                        fitStringStart(pBuffer.offset(printStart as
                                                          libc::c_int as
                                                          isize),
                                       (*psWidget).width as UDWORD,
                                       &mut printChars, &mut printWidth);
                    }
                } else {
                    fitStringStart(pBuffer.offset(printStart as libc::c_int as
                                                      isize),
                                   (*psWidget).width as UDWORD,
                                   &mut printChars, &mut printWidth);
                }
            }
            9 => {
                putSelection(pBuffer, &mut pos);
                /* Update the printable text */
                fitStringEnd(pBuffer, (*psWidget).width as UDWORD,
                             &mut printStart, &mut printChars,
                             &mut printWidth);
            }
            13 => {
                /* Finish editing */
                editBoxFocusLost(psWidget);
                screenClearFocus((*psContext).psScreen);
                return
            }
            196608 | 262144 | 589824 | 655360 | 27 => { }
            _ => {
                /* Dealt with everything else this must be a printable character */
                if editState == 0x2 as libc::c_int as libc::c_uint {
                    insertChar(pBuffer, &mut pos, inputGetCharKey());
                } else {
                    overwriteChar(pBuffer, &mut pos, inputGetCharKey());
                }
                /* Update the printable chars */
                if pos == strlen(pBuffer) {
                    fitStringEnd(pBuffer, (*psWidget).width as UDWORD,
                                 &mut printStart, &mut printChars,
                                 &mut printWidth);
                } else {
                    fitStringStart(pBuffer.offset(printStart as libc::c_int as
                                                      isize),
                                   (*psWidget).width as UDWORD,
                                   &mut printChars, &mut printWidth);
                    if pos >
                           (printStart as libc::c_int +
                                printChars as libc::c_int) as UDWORD {
                        printStart =
                            (printStart as libc::c_int + 6 as libc::c_int) as
                                UWORD;
                        if printStart as libc::c_uint >= len {
                            printStart =
                                len.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) as UWORD;
                            fitStringStart(pBuffer.offset(printStart as
                                                              libc::c_int as
                                                              isize),
                                           (*psWidget).width as UDWORD,
                                           &mut printChars, &mut printWidth);
                        }
                    }
                }
            }
        }
        key = inputGetKey()
    }
    /* Store the current widget state */
    (*psWidget).insPos = pos as UWORD;
    (*psWidget).state =
        (*psWidget).state & !(0xf as libc::c_int) as libc::c_uint | editState;
    (*psWidget).printStart = printStart;
    (*psWidget).printWidth = printWidth;
    (*psWidget).printChars = printChars;
}
/* Set the current string for the edit box */
/* Set the current string for the edit box */
#[no_mangle]
pub unsafe extern "C" fn editBoxSetString(mut psWidget: *mut W_EDITBOX,
                                          mut pText: *mut STRING) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"editBoxSetString: Invalid edit box pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              574 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"editBoxSetString\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_EDITBOX))\x00" as *const u8 as
                  *const libc::c_char);
    };
    widgCopyString((*psWidget).aText.as_mut_ptr(), pText);
    (*psWidget).state = 0x1 as libc::c_int as UDWORD;
    (*psWidget).printStart = 0 as libc::c_int as UWORD;
    iV_SetFont((*psWidget).FontID);
    fitStringStart((*psWidget).aText.as_mut_ptr(),
                   (*psWidget).width as UDWORD, &mut (*psWidget).printChars,
                   &mut (*psWidget).printWidth);
}
/* Respond to a mouse click */
/* Respond to a mouse click */
#[no_mangle]
pub unsafe extern "C" fn editBoxClicked(mut psWidget: *mut W_EDITBOX,
                                        mut psContext: *mut W_CONTEXT) {
    let mut len: UDWORD = 0;
    if (*psWidget).state & 0x20 as libc::c_int as libc::c_uint != 0 {
        // disabled button.
        return
    }
    if (*psWidget).state & 0xf as libc::c_int as libc::c_uint ==
           0x1 as libc::c_int as libc::c_uint {
        if (*psWidget).style & 1 as libc::c_int as libc::c_uint == 0 {
            if (*psWidget).AudioCallback.is_some() {
                (*psWidget).AudioCallback.expect("non-null function pointer")((*psWidget).ClickedAudioID
                                                                                  as
                                                                                  libc::c_int);
            }
            /* Set up the widget state */
            (*psWidget).state =
                (*psWidget).state & !(0xf as libc::c_int) as libc::c_uint |
                    0x2 as libc::c_int as libc::c_uint;
            len = strlen((*psWidget).aText.as_mut_ptr());
            (*psWidget).insPos = len as UWORD;
            /* Calculate how much of the string can appear in the box */
            iV_SetFont((*psWidget).FontID);
            fitStringEnd((*psWidget).aText.as_mut_ptr(),
                         (*psWidget).width as UDWORD,
                         &mut (*psWidget).printStart,
                         &mut (*psWidget).printChars,
                         &mut (*psWidget).printWidth);
            /* Clear the input buffer */
            inputClearBuffer();
            /* Tell the form that the edit box has focus */
            screenSetFocus((*psContext).psScreen, psWidget as *mut WIDGET);
        }
    };
}
/* Respond to loss of focus */
/* Respond to loss of focus */
#[no_mangle]
pub unsafe extern "C" fn editBoxFocusLost(mut psWidget: *mut W_EDITBOX) {
    if (*psWidget).state & 0x20 as libc::c_int as libc::c_uint == 0 {
    } else {
        debug(LOG_ERROR,
              b"editBoxFocusLost: disabled edit box\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psWidget).state & 0x20 as libc::c_int as libc::c_uint == 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"editbox.c\x00" as *const u8 as *const libc::c_char,
              629 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"editBoxFocusLost\x00")).as_ptr(),
              b"!(psWidget->state & WEDBS_DISABLE)\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Stop editing the widget */
    (*psWidget).state = 0x1 as libc::c_int as UDWORD;
    (*psWidget).printStart = 0 as libc::c_int as UWORD;
    fitStringStart((*psWidget).aText.as_mut_ptr(),
                   (*psWidget).width as UDWORD, &mut (*psWidget).printChars,
                   &mut (*psWidget).printWidth);
    widgSetReturn(psWidget as *mut WIDGET);
}
/* Respond to a mouse button up */
/* Respond to a mouse button up */
#[no_mangle]
pub unsafe extern "C" fn editBoxReleased(mut psWidget: *mut W_EDITBOX) { }
/* Respond to a mouse moving over an edit box */
/* Respond to a mouse moving over an edit box */
#[no_mangle]
pub unsafe extern "C" fn editBoxHiLite(mut psWidget: *mut W_EDITBOX) {
    if (*psWidget).state & 0x20 as libc::c_int as libc::c_uint != 0 { return }
    if (*psWidget).AudioCallback.is_some() {
        (*psWidget).AudioCallback.expect("non-null function pointer")((*psWidget).HilightAudioID
                                                                          as
                                                                          libc::c_int);
    }
    (*psWidget).state |= 0x10 as libc::c_int as libc::c_uint;
}
/* Respond to the mouse moving off an edit box */
/* Respond to the mouse moving off an edit box */
#[no_mangle]
pub unsafe extern "C" fn editBoxHiLiteLost(mut psWidget: *mut W_EDITBOX) {
    if (*psWidget).state & 0x20 as libc::c_int as libc::c_uint != 0 { return }
    (*psWidget).state =
        (*psWidget).state & 0xf as libc::c_int as libc::c_uint;
}
/* The edit box display function */
/* The edit box display function */
#[no_mangle]
pub unsafe extern "C" fn editBoxDisplay(mut psWidget: *mut WIDGET,
                                        mut xOffset: UDWORD,
                                        mut yOffset: UDWORD,
                                        mut pColours: *mut UDWORD) {
    let mut psEdBox: *mut W_EDITBOX = 0 as *mut W_EDITBOX;
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut fx: SDWORD = 0;
    let mut fy: SDWORD = 0;
    let mut cx: SDWORD = 0;
    let mut cy: SDWORD = 0;
    //	PROP_FONT	*psCurrFont;
    let mut CurrFontID: libc::c_int = 0;
    let mut ch: STRING = 0;
    let mut pInsPoint: *mut STRING = 0 as *mut STRING;
    let mut pPrint: *mut STRING = 0 as *mut STRING;
    psEdBox = psWidget as *mut W_EDITBOX;
    //	psCurrFont = psEdBox->psFont;
    CurrFontID = (*psEdBox).FontID; // + (psEdBox->width - fw) / 2;
    x0 = ((*psEdBox).x as libc::c_uint).wrapping_add(xOffset) as SDWORD;
    y0 = ((*psEdBox).y as libc::c_uint).wrapping_add(yOffset) as SDWORD;
    x1 = x0 + (*psEdBox).width as libc::c_int;
    y1 = y0 + (*psEdBox).height as libc::c_int;
    if (*psEdBox).pBoxDisplay.is_some() {
        (*psEdBox).pBoxDisplay.expect("non-null function pointer")(psEdBox as
                                                                       *mut WIDGET,
                                                                   xOffset,
                                                                   yOffset,
                                                                   pColours);
    } else {
        pie_BoxFillIndex(x0, y0, x1, y1, WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0, y0, x1, y0,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x0, y0, x0, y1,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x0, y1, x1, y1,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1, y1, x1, y0,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    }
    fx = x0 + 4 as libc::c_int;
    //	fy = y0 + (psEdBox->height - psCurrFont->height + psCurrFont->baseLine) / 2;
    iV_SetFont(CurrFontID);
    //	fontSet(psCurrFont);
    iV_SetTextColour(*pColours.offset(WCOL_TEXT as libc::c_int as isize) as
                         UBYTE as SWORD);
    fy =
        y0 +
            ((*psEdBox).height as libc::c_int - iV_GetTextLineSize()) /
                2 as libc::c_int - iV_GetTextAboveBase();
    /* If there is more text than will fit into the box,
	   display the bit with the cursor in it */
    pPrint =
        (*psEdBox).aText.as_mut_ptr().offset((*psEdBox).printStart as
                                                 libc::c_int as isize);
    pInsPoint = pPrint.offset((*psEdBox).printChars as libc::c_int as isize);
    ch = *pInsPoint;
    *pInsPoint = '\u{0}' as i32 as STRING;
    //	if(psEdBox->pFontDisplay) {
//		psEdBox->pFontDisplay(fx,fy, pPrint);
//	} else {
    pie_DrawText(pPrint, fx as UDWORD, fy as UDWORD);
    //	}
    *pInsPoint = ch;
    /* Display the cursor if editing */
    if (*psEdBox).state & 0xf as libc::c_int as libc::c_uint ==
           0x2 as libc::c_int as libc::c_uint {
        pInsPoint =
            (*psEdBox).aText.as_mut_ptr().offset((*psEdBox).insPos as
                                                     libc::c_int as isize);
        ch = *pInsPoint;
        *pInsPoint = '\u{0}' as i32 as STRING;
        cx =
            x0 + 4 as libc::c_int +
                iV_GetTextWidth((*psEdBox).aText.as_mut_ptr().offset((*psEdBox).printStart
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         isize));
        *pInsPoint = ch;
        cy = fy;
        pie_Line(cx, cy + iV_GetTextAboveBase(), cx,
                 cy + iV_GetTextBelowBase(),
                 *pColours.offset(WCOL_CURSOR as libc::c_int as isize));
    } else if (*psEdBox).state & 0xf as libc::c_int as libc::c_uint ==
                  0x3 as libc::c_int as libc::c_uint {
        pInsPoint =
            (*psEdBox).aText.as_mut_ptr().offset((*psEdBox).insPos as
                                                     libc::c_int as isize);
        ch = *pInsPoint;
        *pInsPoint = '\u{0}' as i32 as STRING;
        cx =
            x0 + 4 as libc::c_int +
                iV_GetTextWidth((*psEdBox).aText.as_mut_ptr().offset((*psEdBox).printStart
                                                                         as
                                                                         libc::c_int
                                                                         as
                                                                         isize));
        *pInsPoint = ch;
        cy = fy;
        //		cy = fy + psCurrFont->height - (psCurrFont->baseLine >> 1);
        pie_Line(cx, cy, cx + 8 as libc::c_int, cy,
                 *pColours.offset(WCOL_CURSOR as libc::c_int as isize));
    }
    if (*psEdBox).pBoxDisplay.is_none() {
        if (*psEdBox).state & 0x10 as libc::c_int as libc::c_uint != 0 {
            /* Display the button hilite */
            pie_Line(x0 - 2 as libc::c_int, y0 - 2 as libc::c_int,
                     x1 + 2 as libc::c_int, y0 - 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 - 2 as libc::c_int, y0 - 2 as libc::c_int,
                     x0 - 2 as libc::c_int, y1 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x0 - 2 as libc::c_int, y1 + 2 as libc::c_int,
                     x1 + 2 as libc::c_int, y1 + 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
            pie_Line(x1 + 2 as libc::c_int, y1 + 2 as libc::c_int,
                     x1 + 2 as libc::c_int, y0 - 2 as libc::c_int,
                     *pColours.offset(WCOL_HILITE as libc::c_int as isize));
        }
    };
}
/* set state of edit box */
/* Set an edit box'sstate */
#[no_mangle]
pub unsafe extern "C" fn editBoxSetState(mut psEditBox: *mut W_EDITBOX,
                                         mut state: UDWORD) {
    if state & 0x20 as libc::c_int as libc::c_uint != 0 {
        (*psEditBox).state |= 0x20 as libc::c_int as libc::c_uint
    } else { (*psEditBox).state &= !(0x20 as libc::c_int) as libc::c_uint };
}
