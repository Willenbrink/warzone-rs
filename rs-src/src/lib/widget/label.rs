use ::libc;
extern "C" {
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
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
    /* Copy one string to another
 * The string to copy will be truncated if it is longer than WIDG_MAXSTR.
 */
    #[no_mangle]
    fn widgCopyString(pDest: *mut STRING, pSrc: *mut STRING);
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
    fn iV_SetFont(FontID: libc::c_int);
    #[no_mangle]
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextAboveBase() -> libc::c_int;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/* Label initialisation structure */
pub type W_LABINIT = _w_labinit;
/* The basic init entries */
// label text
// Tool tip for the label.
//	PROP_FONT	*psFont;		// label font
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
// Parent screen of the widget
// Parent form of the widget
// Screen offset of the parent form
// mouse position on the form
// label is hilited
pub type W_LABEL = _w_label;
/* The common widget data */
// The current button state
// Text on the label
//	PROP_FONT	*psFont;				// Font for the label
// The tool tip for the button
/*
 * Label.c
 *
 * Functions for the label widget.
 */
// FIXME Direct iVis implementation include!
/* The widget heaps */
#[no_mangle]
pub static mut psLabHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Create a button widget data structure */
/* Create a button widget data structure */
#[no_mangle]
pub unsafe extern "C" fn labelCreate(mut ppsWidget: *mut *mut W_LABEL,
                                     mut psInit: *mut W_LABINIT) -> BOOL {
    /* Do some validation on the initialisation struct */
    if (*psInit).style &
           !(0 as libc::c_int | 1 as libc::c_int | 4 as libc::c_int |
                 2 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint !=
           0 {
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
                  b"label.c\x00" as *const u8 as *const libc::c_char,
                  27 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"labelCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //	ASSERT( PTRVALID(psInit->psFont, sizeof(PROP_FONT)),
//		"labelCreate: Invalid font pointer" );
    /* Allocate the required memory */
    if heapAlloc(psLabHeap,
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
                  b"label.c\x00" as *const u8 as *const libc::c_char,
                  42 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"labelCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Allocate the memory for the tip and copy it if necessary */
    if !(*psInit).pTip.is_null() {
        (**ppsWidget).pTip = (*psInit).pTip
    } else { (**ppsWidget).pTip = 0 as *mut STRING }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_LABEL;
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
            Some(labelDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    //	(*ppsWidget)->psFont = psInit->psFont;
    (**ppsWidget).FontID = (*psInit).FontID;
    if !(*psInit).pText.is_null() {
        widgCopyString((**ppsWidget).aText.as_mut_ptr(), (*psInit).pText);
    } else { *(**ppsWidget).aText.as_mut_ptr() = 0 as libc::c_int as STRING }
    return 1 as libc::c_int;
}
/* Free the memory used by a button */
/* Free the memory used by a button */
#[no_mangle]
pub unsafe extern "C" fn labelFree(mut psWidget: *mut W_LABEL) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"labelFree: Invalid label pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"label.c\x00" as *const u8 as *const libc::c_char,
              112 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"labelFree\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_LABEL))\x00" as *const u8 as
                  *const libc::c_char);
    };
    heapFree(psLabHeap, psWidget as *mut libc::c_void);
}
/* label display function */
/* label display function */
#[no_mangle]
pub unsafe extern "C" fn labelDisplay(mut psWidget: *mut WIDGET,
                                      mut xOffset: UDWORD,
                                      mut yOffset: UDWORD,
                                      mut pColours: *mut UDWORD) {
    let mut fx: SDWORD = 0;
    let mut fy: SDWORD = 0;
    let mut fw: SDWORD = 0;
    let mut psLabel: *mut W_LABEL = 0 as *mut W_LABEL;
    //	PROP_FONT	*psFont;
    let mut FontID: libc::c_int = 0;
    psLabel = psWidget as *mut W_LABEL;
    //	psFont = psLabel->psFont;
    FontID = (*psLabel).FontID;
    iV_SetFont(FontID);
    //	fontSetCacheColour(*(pColours + WCOL_TEXT));
    iV_SetTextColour(*pColours.offset(WCOL_TEXT as libc::c_int as isize) as
                         UWORD as SWORD);
    if (*psLabel).style & 2 as libc::c_int as libc::c_uint != 0 {
        fw = iV_GetTextWidth((*psLabel).aText.as_mut_ptr());
        fx =
            xOffset.wrapping_add((*psLabel).x as
                                     libc::c_uint).wrapping_add((((*psLabel).width
                                                                      as
                                                                      libc::c_int
                                                                      - fw) /
                                                                     2 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint)
                as SDWORD
    } else if (*psLabel).style & 4 as libc::c_int as libc::c_uint != 0 {
        fw = iV_GetTextWidth((*psLabel).aText.as_mut_ptr());
        fx =
            xOffset.wrapping_add((*psLabel).x as
                                     libc::c_uint).wrapping_add((*psLabel).width
                                                                    as
                                                                    libc::c_uint).wrapping_sub(fw
                                                                                                   as
                                                                                                   libc::c_uint)
                as SDWORD
    } else {
        fx = xOffset.wrapping_add((*psLabel).x as libc::c_uint) as SDWORD
    }
    fy =
        yOffset.wrapping_add((*psLabel).y as
                                 libc::c_uint).wrapping_add((((*psLabel).height
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
    //	fy = yOffset + psLabel->y + (psLabel->height -
//			psFont->height + psFont->baseLine) / 2;
    pie_DrawText((*psLabel).aText.as_mut_ptr(), fx as UDWORD, fy as UDWORD);
    //	fontPrint(fx,fy, psLabel->aText);
}
/* Respond to a mouse moving over a label */
/* Respond to a mouse moving over a label */
#[no_mangle]
pub unsafe extern "C" fn labelHiLite(mut psWidget: *mut W_LABEL,
                                     mut psContext: *mut W_CONTEXT) {
    (*psWidget).state |= 0x4 as libc::c_int as libc::c_uint;
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
/* Respond to the mouse moving off a label */
/* Respond to the mouse moving off a label */
#[no_mangle]
pub unsafe extern "C" fn labelHiLiteLost(mut psWidget: *mut W_LABEL) {
    (*psWidget).state &= !(0x4 as libc::c_int) as libc::c_uint;
    if !(*psWidget).pTip.is_null() { tipStop(psWidget as *mut WIDGET); };
}
