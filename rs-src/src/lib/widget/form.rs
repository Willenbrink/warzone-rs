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
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Get widget structure */
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
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
    /* Release a list of widgets */
    #[no_mangle]
    fn widgReleaseWidgetList(psWidgets: *mut WIDGET);
    /* Call the correct function for mouse moving off */
    #[no_mangle]
    fn widgHiLiteLost(psWidget: *mut WIDGET, psContext: *mut W_CONTEXT);
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
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
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
    #[no_mangle]
    fn pie_Box(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
               y1: libc::c_int, colour: uint32);
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
// Extension memory for the heap
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
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
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
pub type uint8 = libc::c_uchar;
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
 * Form.h
 *
 * Definitions for the form functions.
 *
 */
/* The widget heaps */
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
// Button is flashing
/* The clickable form data structure */
pub type W_CLICKFORM = _w_clickform;
pub type uint32 = libc::c_uint;
/* The common form data */
// Button state of the form
// Tip for the form
// Audio ID for form clicked sound
// Audio ID for form hilighted sound
// Pointer to audio callback function
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
/* Store the position of a tab */
pub type TAB_POS = _tab_pos;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tab_pos {
    pub index: SDWORD,
    pub x: SDWORD,
    pub y: SDWORD,
    pub width: UDWORD,
    pub height: UDWORD,
}
/*
 * Form.c
 *
 * Functionality for the form widget.
 */
// FIXME Direct iVis implementation include!
/* The widget heaps */
#[no_mangle]
pub static mut psFormHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut psCFormHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut psTFormHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* Set default colours for a form */
unsafe extern "C" fn formSetDefaultColours(mut psForm: *mut W_FORM) {
    static mut bDefaultsSet: BOOL = 0 as libc::c_int;
    static mut wcol_bkgrnd: UBYTE = 0;
    static mut wcol_text: SDWORD = 0;
    static mut wcol_light: UBYTE = 0;
    static mut wcol_dark: UBYTE = 0;
    static mut wcol_hilite: UBYTE = 0;
    static mut wcol_cursor: UBYTE = 0;
    static mut wcol_tipbkgrnd: UBYTE = 0;
    static mut wcol_disable: UBYTE = 0;
    if bDefaultsSet != 0 {
        (*psForm).aColours[WCOL_BKGRND as libc::c_int as usize] =
            wcol_bkgrnd as UDWORD;
        (*psForm).aColours[WCOL_TEXT as libc::c_int as usize] =
            wcol_text as UDWORD;
        (*psForm).aColours[WCOL_LIGHT as libc::c_int as usize] =
            wcol_light as UDWORD;
        (*psForm).aColours[WCOL_DARK as libc::c_int as usize] =
            wcol_dark as UDWORD;
        (*psForm).aColours[WCOL_HILITE as libc::c_int as usize] =
            wcol_hilite as UDWORD;
        (*psForm).aColours[WCOL_CURSOR as libc::c_int as usize] =
            wcol_cursor as UDWORD;
        (*psForm).aColours[WCOL_TIPBKGRND as libc::c_int as usize] =
            wcol_tipbkgrnd as UDWORD;
        (*psForm).aColours[WCOL_DISABLE as libc::c_int as usize] =
            wcol_disable as UDWORD
    } else {
        wcol_bkgrnd =
            pal_GetNearestColour(0x7f as libc::c_int as uint8,
                                 0x7f as libc::c_int as uint8,
                                 0x7f as libc::c_int as uint8);
        wcol_text = -(1 as libc::c_int);
        wcol_light =
            pal_GetNearestColour(0xff as libc::c_int as uint8,
                                 0xff as libc::c_int as uint8,
                                 0xff as libc::c_int as uint8);
        wcol_dark =
            pal_GetNearestColour(0 as libc::c_int as uint8,
                                 0 as libc::c_int as uint8,
                                 0 as libc::c_int as uint8);
        wcol_hilite =
            pal_GetNearestColour(0x40 as libc::c_int as uint8,
                                 0x40 as libc::c_int as uint8,
                                 0x40 as libc::c_int as uint8);
        wcol_cursor =
            pal_GetNearestColour(0xff as libc::c_int as uint8,
                                 0 as libc::c_int as uint8,
                                 0 as libc::c_int as uint8);
        wcol_tipbkgrnd =
            pal_GetNearestColour(0x30 as libc::c_int as uint8,
                                 0x30 as libc::c_int as uint8,
                                 0x60 as libc::c_int as uint8);
        wcol_disable =
            pal_GetNearestColour(0xbf as libc::c_int as uint8,
                                 0xbf as libc::c_int as uint8,
                                 0xbf as libc::c_int as uint8);
        bDefaultsSet = 1 as libc::c_int;
        (*psForm).aColours[WCOL_BKGRND as libc::c_int as usize] =
            wcol_bkgrnd as UDWORD;
        (*psForm).aColours[WCOL_TEXT as libc::c_int as usize] =
            wcol_text as UDWORD;
        (*psForm).aColours[WCOL_LIGHT as libc::c_int as usize] =
            wcol_light as UDWORD;
        (*psForm).aColours[WCOL_DARK as libc::c_int as usize] =
            wcol_dark as UDWORD;
        (*psForm).aColours[WCOL_HILITE as libc::c_int as usize] =
            wcol_hilite as UDWORD;
        (*psForm).aColours[WCOL_CURSOR as libc::c_int as usize] =
            wcol_cursor as UDWORD;
        (*psForm).aColours[WCOL_TIPBKGRND as libc::c_int as usize] =
            wcol_tipbkgrnd as UDWORD;
        (*psForm).aColours[WCOL_DISABLE as libc::c_int as usize] =
            wcol_disable as UDWORD
    };
}
/* Create a plain form widget */
unsafe extern "C" fn formCreatePlain(mut ppsWidget: *mut *mut W_FORM,
                                     mut psInit: *mut W_FORMINIT) -> BOOL {
    /* Allocate the required memory */
    if heapAlloc(psFormHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreatePlain: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  94 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"formCreatePlain\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Initialise the structure */
    memset(*ppsWidget as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<W_FORM>() as libc::c_ulong);
    (**ppsWidget).type_0 = WIDG_FORM;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).disableChildren = (*psInit).disableChildren;
    (**ppsWidget).animCount = 0 as libc::c_int as UDWORD;
    (**ppsWidget).x = (*psInit).x;
    (**ppsWidget).y = (*psInit).y;
    (**ppsWidget).width = (*psInit).width;
    (**ppsWidget).height = (*psInit).height;
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else {
        (**ppsWidget).display =
            Some(formDisplay as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    (**ppsWidget).psWidgets = 0 as *mut WIDGET;
    (**ppsWidget).psLastHiLite = 0 as *mut WIDGET;
    formSetDefaultColours(*ppsWidget);
    formInitialise(*ppsWidget);
    return 1 as libc::c_int;
}
/* Free a plain form widget */
unsafe extern "C" fn formFreePlain(mut psWidget: *mut W_FORM) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formFreePlain: Invalid form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              136 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"formFreePlain\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_FORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    widgReleaseWidgetList((*psWidget).psWidgets);
    heapFree(psFormHeap, psWidget as *mut libc::c_void);
}
/* Create a plain form widget */
unsafe extern "C" fn formCreateClickable(mut ppsWidget: *mut *mut W_CLICKFORM,
                                         mut psInit: *mut W_FORMINIT)
 -> BOOL {
    /* Allocate the required memory */
    if heapAlloc(psCFormHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreateClickable: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  158 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"formCreateClickable\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Initialise the structure */
    memset(*ppsWidget as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<W_CLICKFORM>() as libc::c_ulong);
    (**ppsWidget).type_0 = WIDG_FORM;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).disableChildren = (*psInit).disableChildren;
    (**ppsWidget).animCount = 0 as libc::c_int as UDWORD;
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
            Some(formDisplayClickable as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).psWidgets = 0 as *mut WIDGET;
    (**ppsWidget).psLastHiLite = 0 as *mut WIDGET;
    if !(*psInit).pTip.is_null() {
        (**ppsWidget).pTip = (*psInit).pTip
    } else { (**ppsWidget).pTip = 0 as *mut STRING }
    formSetDefaultColours(*ppsWidget as *mut W_FORM);
    formInitialise(*ppsWidget as *mut W_FORM);
    return 1 as libc::c_int;
}
/* Free a plain form widget */
unsafe extern "C" fn formFreeClickable(mut psWidget: *mut W_CLICKFORM) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formFreePlain: Invalid form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              224 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"formFreeClickable\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_FORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    widgReleaseWidgetList((*psWidget).psWidgets);
    heapFree(psCFormHeap, psWidget as *mut libc::c_void);
}
/* Create a tabbed form widget */
unsafe extern "C" fn formCreateTabbed(mut ppsWidget: *mut *mut W_TABFORM,
                                      mut psInit: *mut W_FORMINIT) -> BOOL {
    let mut major: UDWORD = 0;
    let mut minor: UDWORD = 0;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    if (*psInit).numMajor as libc::c_int == 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreateTabbed: Must have at least one major tab on a tabbed form\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  250 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).majorPos as libc::c_int != 0 as libc::c_int &&
           (*psInit).majorPos as libc::c_int ==
               (*psInit).minorPos as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreateTabbed: Cannot have major and minor tabs on same side\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  255 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).numMajor as libc::c_int >= 9 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreateTabbed: Too many Major tabs\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  260 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    major = 0 as libc::c_int as UDWORD;
    while major < (*psInit).numMajor as libc::c_uint {
        if (*psInit).aNumMinors[major as usize] as libc::c_int >=
               5 as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formCreateTabbed: Too many Minor tabs for Major %d\x00"
                          as *const u8 as *const libc::c_char, major);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      267 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        if (*psInit).aNumMinors[major as usize] as libc::c_int ==
               0 as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formCreateTabbed: Must have at least one Minor tab for each major\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      272 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        major = major.wrapping_add(1)
    }
    /* Allocate the required memory */
    if heapAlloc(psTFormHeap,
                 ppsWidget as *mut libc::c_void as *mut *mut libc::c_void) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreateTabbed: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  285 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"formCreateTabbed\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    memset(*ppsWidget as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<W_TABFORM>() as libc::c_ulong);
    /* Allocate the memory for tool tips and copy them in */
    psMajor = (**ppsWidget).asMajor.as_mut_ptr();
    major = 0 as libc::c_int as UDWORD;
    while major < (*psInit).numMajor as libc::c_uint {
        /* Check for a tip for the major tab */
        if !(*psInit).apMajorTips[major as usize].is_null() {
            (*psMajor).pTip = (*psInit).apMajorTips[major as usize]
        }
        /* Check for tips for the minor tab */
        minor = 0 as libc::c_int as UDWORD;
        while minor < (*psInit).aNumMinors[major as usize] as libc::c_uint {
            if !(*psInit).apMinorTips[major as
                                          usize][minor as usize].is_null() {
                (*psMajor).asMinor[minor as usize].pTip =
                    (*psInit).apMinorTips[major as usize][minor as usize]
            }
            minor = minor.wrapping_add(1)
        }
        psMajor = psMajor.offset(1);
        major = major.wrapping_add(1)
    }
    /* Initialise the structure */
    (**ppsWidget).type_0 = WIDG_FORM;
    (**ppsWidget).id = (*psInit).id;
    (**ppsWidget).formID = (*psInit).formID;
    (**ppsWidget).style = (*psInit).style;
    (**ppsWidget).disableChildren = (*psInit).disableChildren;
    (**ppsWidget).animCount = 0 as libc::c_int as UDWORD;
    (**ppsWidget).x = (*psInit).x;
    (**ppsWidget).y = (*psInit).y;
    (**ppsWidget).width = (*psInit).width;
    (**ppsWidget).height = (*psInit).height;
    if (*psInit).pDisplay.is_some() {
        (**ppsWidget).display = (*psInit).pDisplay
    } else {
        (**ppsWidget).display =
            Some(formDisplayTabbed as
                     unsafe extern "C" fn(_: *mut WIDGET, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    (**ppsWidget).callback = (*psInit).pCallback;
    (**ppsWidget).pUserData = (*psInit).pUserData;
    (**ppsWidget).UserData = (*psInit).UserData;
    (**ppsWidget).psLastHiLite = 0 as *mut WIDGET;
    (**ppsWidget).majorSize = (*psInit).majorSize;
    (**ppsWidget).minorSize = (*psInit).minorSize;
    (**ppsWidget).tabMajorThickness = (*psInit).tabMajorThickness;
    (**ppsWidget).tabMinorThickness = (*psInit).tabMinorThickness;
    (**ppsWidget).tabMajorGap = (*psInit).tabMajorGap;
    (**ppsWidget).tabMinorGap = (*psInit).tabMinorGap;
    (**ppsWidget).tabVertOffset = (*psInit).tabVertOffset;
    (**ppsWidget).tabHorzOffset = (*psInit).tabHorzOffset;
    (**ppsWidget).majorOffset = (*psInit).majorOffset;
    (**ppsWidget).minorOffset = (*psInit).minorOffset;
    (**ppsWidget).majorPos = (*psInit).majorPos;
    (**ppsWidget).minorPos = (*psInit).minorPos;
    (**ppsWidget).pTabDisplay = (*psInit).pTabDisplay;
    (**ppsWidget).pFormDisplay = (*psInit).pFormDisplay;
    formSetDefaultColours(*ppsWidget as *mut W_FORM);
    /* Set up the tab data.
	 * All widget pointers have been zeroed by the memset above.
	 */
    (**ppsWidget).numMajor = (*psInit).numMajor;
    major = 0 as libc::c_int as UDWORD;
    while major < (*psInit).numMajor as libc::c_uint {
        (**ppsWidget).asMajor[major as usize].numMinor =
            (*psInit).aNumMinors[major as usize];
        major = major.wrapping_add(1)
    }
    formInitialise(*ppsWidget as *mut W_FORM);
    return 1 as libc::c_int;
}
/* Control whether single tabs are displayed */
/* Free the tips strings for a tabbed form */
unsafe extern "C" fn formFreeTips(mut psForm: *mut W_TABFORM) {
    psForm = psForm;
}
/* Free a tabbed form widget */
unsafe extern "C" fn formFreeTabbed(mut psWidget: *mut W_TABFORM) {
    let mut psCurr: *mut WIDGET = 0 as *mut WIDGET;
    let mut sGetAll: W_FORMGETALL =
        W_FORMGETALL{psGAWList: 0 as *mut WIDGET,
                     psGAWForm: 0 as *mut W_TABFORM,
                     psGAWMajor: 0 as *mut W_MAJORTAB,
                     GAWMajor: 0,
                     GAWMinor: 0,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formFreeTabbed: Invalid form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              408 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"formFreeTabbed\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_TABFORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    formFreeTips(psWidget);
    formInitGetAllWidgets(psWidget as *mut W_FORM, &mut sGetAll);
    psCurr = formGetAllWidgets(&mut sGetAll);
    while !psCurr.is_null() {
        widgReleaseWidgetList(psCurr);
        psCurr = formGetAllWidgets(&mut sGetAll)
    }
    heapFree(psTFormHeap, psWidget as *mut libc::c_void);
}
/* Create a form widget data structure */
/* Create a form widget data structure */
#[no_mangle]
pub unsafe extern "C" fn formCreate(mut ppsWidget: *mut *mut W_FORM,
                                    mut psInit: *mut W_FORMINIT) -> BOOL {
    /* Check the style bits are OK */
    if (*psInit).style &
           !(1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int |
                 8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                 | 0x8000 as libc::c_int) as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreate: Unknown style bit\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  435 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"formCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).style & 1 as libc::c_int as libc::c_uint != 0 &&
           (*psInit).style &
               (2 as libc::c_int | 4 as libc::c_int) as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreate: Tabbed form cannot be invisible or clickable\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  441 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"formCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).style & 2 as libc::c_int as libc::c_uint != 0 &&
           (*psInit).style & 4 as libc::c_int as libc::c_uint != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreate: Cannot have an invisible clickable form\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  447 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"formCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psInit).style & 4 as libc::c_int as libc::c_uint == 0 &&
           ((*psInit).style & 0x10 as libc::c_int as libc::c_uint != 0 ||
                (*psInit).style & 0x20 as libc::c_int as libc::c_uint != 0) {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formCreate: Cannot set keys if the form isn\'t clickable\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  453 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"formCreate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Create the correct type of form */
    if (*psInit).style & 1 as libc::c_int as libc::c_uint != 0 {
        return formCreateTabbed(ppsWidget as *mut *mut W_TABFORM, psInit)
    } else if (*psInit).style & 4 as libc::c_int as libc::c_uint != 0 {
        return formCreateClickable(ppsWidget as *mut *mut W_CLICKFORM, psInit)
    } else { return formCreatePlain(ppsWidget, psInit) };
}
/* Free the memory used by a form */
/* Free the memory used by a form */
#[no_mangle]
pub unsafe extern "C" fn formFree(mut psWidget: *mut W_FORM) {
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        formFreeTabbed(psWidget as *mut W_TABFORM);
    } else if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        formFreeClickable(psWidget as *mut W_CLICKFORM);
    } else { formFreePlain(psWidget); };
}
/* Add a widget to a form */
/* Add a widget to a form */
#[no_mangle]
pub unsafe extern "C" fn formAddWidget(mut psForm: *mut W_FORM,
                                       mut psWidget: *mut WIDGET,
                                       mut psInit: *mut W_INIT) -> BOOL {
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut ppsList: *mut *mut WIDGET = 0 as *mut *mut WIDGET;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formAddWidget: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              501 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"formAddWidget\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(WIDGET))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psForm).style & 1 as libc::c_int as libc::c_uint != 0 {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formAddWidget: Invalid tab form pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  506 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"formAddWidget\x00")).as_ptr(),
                  b"PTRVALID(psForm, sizeof(W_TABFORM))\x00" as *const u8 as
                      *const libc::c_char);
        };
        psTabForm = psForm as *mut W_TABFORM;
        if (*psInit).majorID as libc::c_int >=
               (*psTabForm).numMajor as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formAddWidget: Major tab does not exist\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      510 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"formAddWidget\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        psMajor =
            (*psTabForm).asMajor.as_mut_ptr().offset((*psInit).majorID as
                                                         libc::c_int as
                                                         isize);
        if (*psInit).minorID as libc::c_int >=
               (*psMajor).numMinor as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formAddWidget: Minor tab does not exist\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      516 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"formAddWidget\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        ppsList =
            &mut (*(*psMajor).asMinor.as_mut_ptr().offset((*psInit).minorID as
                                                              isize)).psWidgets;
        (*psWidget).psNext = *ppsList;
        *ppsList = psWidget
    } else {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formAddWidget: Invalid form pointer\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  526 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"formAddWidget\x00")).as_ptr(),
                  b"PTRVALID(psForm, sizeof(W_FORM))\x00" as *const u8 as
                      *const libc::c_char);
        };
        (*psWidget).psNext = (*psForm).psWidgets;
        (*psForm).psWidgets = psWidget
    }
    return 1 as libc::c_int;
}
/* Get the button state of a click form */
/* Get the button state of a click form */
#[no_mangle]
pub unsafe extern "C" fn formGetClickState(mut psForm: *mut W_CLICKFORM)
 -> UDWORD {
    let mut State: UDWORD = 0 as libc::c_int as UDWORD;
    if (*psForm).state & 0x2 as libc::c_int as libc::c_uint != 0 {
        State |= 0x1 as libc::c_int as libc::c_uint
    }
    if (*psForm).state & 0x8 as libc::c_int as libc::c_uint != 0 {
        State |= 0x2 as libc::c_int as libc::c_uint
    }
    if (*psForm).state & 0x10 as libc::c_int as libc::c_uint != 0 {
        State |= 0x4 as libc::c_int as libc::c_uint
    }
    return State;
}
/* Set the button state of a click form */
/* Set the button state of a click form */
#[no_mangle]
pub unsafe extern "C" fn formSetClickState(mut psForm: *mut W_CLICKFORM,
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
              b"form.c\x00" as *const u8 as *const libc::c_char,
              563 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"formSetClickState\x00")).as_ptr(),
              b"!((state & WBUT_LOCK) && (state & WBUT_CLICKLOCK))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if state & 0x1 as libc::c_int as libc::c_uint != 0 {
        (*psForm).state |= 0x2 as libc::c_int as libc::c_uint
    } else { (*psForm).state &= !(0x2 as libc::c_int) as libc::c_uint }
    if state & 0x2 as libc::c_int as libc::c_uint != 0 {
        (*psForm).state |= 0x8 as libc::c_int as libc::c_uint
    } else { (*psForm).state &= !(0x8 as libc::c_int) as libc::c_uint }
    if state & 0x4 as libc::c_int as libc::c_uint != 0 {
        (*psForm).state |= 0x10 as libc::c_int as libc::c_uint
    } else { (*psForm).state &= !(0x10 as libc::c_int) as libc::c_uint };
}
/* Return the widgets currently displayed by a form */
/* Return the widgets currently displayed by a form */
#[no_mangle]
pub unsafe extern "C" fn formGetWidgets(mut psWidget: *mut W_FORM)
 -> *mut WIDGET {
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        psTabForm = psWidget as *mut W_TABFORM;
        psMajor =
            (*psTabForm).asMajor.as_mut_ptr().offset((*psTabForm).majorT as
                                                         libc::c_int as
                                                         isize);
        return (*psMajor).asMinor[(*psTabForm).minorT as usize].psWidgets
    } else { return (*psWidget).psWidgets };
}
/* Initialise the formGetAllWidgets function */
/* Initialise the formGetAllWidgets function */
#[no_mangle]
pub unsafe extern "C" fn formInitGetAllWidgets(mut psWidget: *mut W_FORM,
                                               mut psCtrl:
                                                   *mut W_FORMGETALL) {
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        (*psCtrl).psGAWList = 0 as *mut WIDGET;
        (*psCtrl).psGAWForm = psWidget as *mut W_TABFORM;
        (*psCtrl).psGAWMajor = (*(*psCtrl).psGAWForm).asMajor.as_mut_ptr();
        (*psCtrl).GAWMajor = 0 as libc::c_int as UDWORD;
        (*psCtrl).GAWMinor = 0 as libc::c_int as UDWORD
    } else {
        (*psCtrl).psGAWList = (*psWidget).psWidgets;
        (*psCtrl).psGAWForm = 0 as *mut W_TABFORM
    };
}
/* Repeated calls to this function will return widget lists
 * until all widgets in a form have been returned.
 * When a NULL list is returned, all widgets have been seen.
 */
/* Repeated calls to this function will return widget lists
 * until all widgets in a form have been returned.
 * When a NULL list is returned, all widgets have been seen.
 */
#[no_mangle]
pub unsafe extern "C" fn formGetAllWidgets(mut psCtrl: *mut W_FORMGETALL)
 -> *mut WIDGET {
    let mut psRetList: *mut WIDGET = 0 as *mut WIDGET;
    if (*psCtrl).psGAWForm.is_null() {
        /* Not a tabbed form, return the list */
        psRetList = (*psCtrl).psGAWList;
        (*psCtrl).psGAWList = 0 as *mut WIDGET
    } else {
        /* Working with a tabbed form - search for the first widget list */
        psRetList = 0 as *mut WIDGET;
        while psRetList.is_null() &&
                  (*psCtrl).GAWMajor <
                      (*(*psCtrl).psGAWForm).numMajor as libc::c_uint {
            let fresh0 = (*psCtrl).GAWMinor;
            (*psCtrl).GAWMinor = (*psCtrl).GAWMinor.wrapping_add(1);
            psRetList =
                (*(*psCtrl).psGAWMajor).asMinor[fresh0 as usize].psWidgets;
            if (*psCtrl).GAWMinor >=
                   (*(*psCtrl).psGAWMajor).numMinor as libc::c_uint {
                (*psCtrl).GAWMinor = 0 as libc::c_int as UDWORD;
                (*psCtrl).GAWMajor = (*psCtrl).GAWMajor.wrapping_add(1);
                (*psCtrl).psGAWMajor = (*psCtrl).psGAWMajor.offset(1)
            }
        }
    }
    return psRetList;
}
/* Set the current tabs for a tab form */
/* Set the current tabs for a tab form */
#[no_mangle]
pub unsafe extern "C" fn widgSetTabs(mut psScreen: *mut W_SCREEN,
                                     mut id: UDWORD, mut major: UWORD,
                                     mut minor: UWORD) {
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    psForm = widgGetFromID(psScreen, id) as *mut W_TABFORM;
    if psForm.is_null() ||
           (*psForm).style & 1 as libc::c_int as libc::c_uint == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetTabs: couldn\'t find tabbed form from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  671 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"widgSetTabs\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetTabs: Invalid tab form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              675 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"widgSetTabs\x00")).as_ptr(),
              b"PTRVALID(psForm, sizeof(W_TABFORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if major as libc::c_int >= (*psForm).numMajor as libc::c_int ||
           minor as libc::c_int >=
               (*psForm).asMajor[major as usize].numMinor as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetTabs: invalid major or minor id\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  680 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"widgSetTabs\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    (*psForm).majorT = major;
    (*psForm).minorT = minor;
    (*psForm).asMajor[major as usize].lastMinor = minor;
}
/* Get the current tabs for a tab form */
/* Get the current tabs for a tab form */
#[no_mangle]
pub unsafe extern "C" fn widgGetTabs(mut psScreen: *mut W_SCREEN,
                                     mut id: UDWORD, mut pMajor: *mut UWORD,
                                     mut pMinor: *mut UWORD) {
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    psForm = widgGetFromID(psScreen, id) as *mut W_TABFORM;
    if psForm.is_null() ||
           (*psForm).type_0 as libc::c_uint !=
               WIDG_FORM as libc::c_int as libc::c_uint ||
           (*psForm).style & 1 as libc::c_int as libc::c_uint == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgGetTabs: couldn\'t find tabbed form from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  699 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"widgGetTabs\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgGetTabs: Invalid tab form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              703 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"widgGetTabs\x00")).as_ptr(),
              b"PTRVALID(psForm, sizeof(W_TABFORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    *pMajor = (*psForm).majorT;
    *pMinor = (*psForm).minorT;
}
/* Set a colour on a form */
/* Set a colour on a form */
#[no_mangle]
pub unsafe extern "C" fn widgSetColour(mut psScreen: *mut W_SCREEN,
                                       mut id: UDWORD, mut colour: UDWORD,
                                       mut red: UBYTE, mut green: UBYTE,
                                       mut blue: UBYTE) {
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    psForm = widgGetFromID(psScreen, id) as *mut W_TABFORM;
    if psForm.is_null() ||
           (*psForm).type_0 as libc::c_uint !=
               WIDG_FORM as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetColour: couldn\'t find form from id\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  719 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgSetColour\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"widgSetColour: Invalid tab form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              723 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"widgSetColour\x00")).as_ptr(),
              b"PTRVALID(psForm, sizeof(W_FORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if colour >= WCOL_MAX as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"widgSetColour: Colour id out of range\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  727 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"widgSetColour\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    (*psForm).aColours[colour as usize] =
        pal_GetNearestColour(red, green, blue) as UDWORD;
}
/* Return the origin on the form from which button locations are calculated */
/* Return the origin on the form from which button locations are calculated */
#[no_mangle]
pub unsafe extern "C" fn formGetOrigin(mut psWidget: *mut W_FORM,
                                       mut pXOrigin: *mut SDWORD,
                                       mut pYOrigin: *mut SDWORD) {
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formGetOrigin: Invalid form pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"form.c\x00" as *const u8 as *const libc::c_char,
              740 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"formGetOrigin\x00")).as_ptr(),
              b"PTRVALID(psWidget, sizeof(W_FORM))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        psTabForm = psWidget as *mut W_TABFORM;
        if (*psTabForm).majorPos as libc::c_int == 1 as libc::c_int {
            *pYOrigin = (*psTabForm).tabMajorThickness as SDWORD
        } else if (*psTabForm).minorPos as libc::c_int == 1 as libc::c_int {
            *pYOrigin = (*psTabForm).tabMinorThickness as SDWORD
        } else { *pYOrigin = 0 as libc::c_int }
        if (*psTabForm).majorPos as libc::c_int == 2 as libc::c_int {
            *pXOrigin = (*psTabForm).tabMajorThickness as SDWORD
        } else if (*psTabForm).minorPos as libc::c_int == 2 as libc::c_int {
            *pXOrigin = (*psTabForm).tabMinorThickness as SDWORD
        } else { *pXOrigin = 0 as libc::c_int }
        //		if ((psTabForm->majorPos == WFORM_TABTOP) ||
//			(psTabForm->minorPos == WFORM_TABTOP))
//		{
//			*pYOrigin = psTabForm->tabThickness;
//		}
//		else
//		{
//			*pYOrigin = 0;
//		}
//		if ((psTabForm->majorPos == WFORM_TABLEFT) ||
//			(psTabForm->minorPos == WFORM_TABLEFT))
//		{
//			*pXOrigin = psTabForm->tabThickness;
//		}
//		else
//		{
//			*pXOrigin = 0;
//		}
    } else { *pXOrigin = 0 as libc::c_int; *pYOrigin = 0 as libc::c_int };
}
/* Initialise a form widget before running it */
/* Initialise a form widget before running it */
#[no_mangle]
pub unsafe extern "C" fn formInitialise(mut psWidget: *mut W_FORM) {
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut psClickForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    let mut i: UDWORD = 0;
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formInitialise: invalid tab form pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  796 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"formInitialise\x00")).as_ptr(),
                  b"PTRVALID(psWidget, sizeof(W_TABFORM))\x00" as *const u8 as
                      *const libc::c_char);
        };
        psTabForm = psWidget as *mut W_TABFORM;
        (*psTabForm).majorT = 0 as libc::c_int as UWORD;
        (*psTabForm).minorT = 0 as libc::c_int as UWORD;
        (*psTabForm).tabHiLite = -(1 as libc::c_int) as UWORD;
        i = 0 as libc::c_int as UDWORD;
        while i < (*psTabForm).numMajor as libc::c_uint {
            (*psTabForm).asMajor[i as usize].lastMinor =
                0 as libc::c_int as UWORD;
            i = i.wrapping_add(1)
        }
    } else if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formInitialise: invalid clickable form pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  809 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"formInitialise\x00")).as_ptr(),
                  b"PTRVALID(psWidget, sizeof(W_CLICKFORM))\x00" as *const u8
                      as *const libc::c_char);
        };
        psClickForm = psWidget as *mut W_CLICKFORM;
        (*psClickForm).state = 0 as libc::c_int as UDWORD
    } else {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formInitialise: invalid form pointer\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"form.c\x00" as *const u8 as *const libc::c_char,
                  816 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"formInitialise\x00")).as_ptr(),
                  b"PTRVALID(psWidget, sizeof(W_FORM))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    (*psWidget).psLastHiLite = 0 as *mut WIDGET;
}
/* Choose a horizontal tab from a coordinate */
unsafe extern "C" fn formPickHTab(mut psTabPos: *mut TAB_POS, mut x0: SDWORD,
                                  mut y0: SDWORD, mut width: UDWORD,
                                  mut height: UDWORD, mut gap: UDWORD,
                                  mut number: UDWORD, mut fx: SDWORD,
                                  mut fy: SDWORD) -> BOOL {
    let mut x: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't have single tabs */
        return 0 as libc::c_int
    }
    x = x0;
    y1 = (y0 as libc::c_uint).wrapping_add(height) as SDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < number {
        //		if (fx >= x && fx <= x + (SDWORD)(width - gap) &&
        if fx >= x && fx <= x + width as SDWORD && fy >= y0 && fy <= y1 {
            /* found a tab under the coordinate */
            (*psTabPos).index = i as SDWORD;
            (*psTabPos).x = x;
            (*psTabPos).y = y0;
            (*psTabPos).width = width;
            (*psTabPos).height = height;
            return 1 as libc::c_int
        }
        x =
            (x as libc::c_uint).wrapping_add(width.wrapping_add(gap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    }
    /* Didn't find any  */
    return 0 as libc::c_int;
}
/* Choose a vertical tab from a coordinate */
unsafe extern "C" fn formPickVTab(mut psTabPos: *mut TAB_POS, mut x0: SDWORD,
                                  mut y0: SDWORD, mut width: UDWORD,
                                  mut height: UDWORD, mut gap: UDWORD,
                                  mut number: UDWORD, mut fx: SDWORD,
                                  mut fy: SDWORD) -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't have single tabs */
        return 0 as libc::c_int
    }
    x1 = (x0 as libc::c_uint).wrapping_add(width) as SDWORD;
    y = y0;
    i = 0 as libc::c_int as UDWORD;
    while i < number as SDWORD as libc::c_uint {
        if fx >= x0 && fx <= x1 && fy >= y && fy <= y + height as SDWORD {
            //			fy >= y && fy <= y + (SDWORD)(height - gap))
            /* found a tab under the coordinate */
            (*psTabPos).index = i as SDWORD;
            (*psTabPos).x = x0;
            (*psTabPos).y = y;
            (*psTabPos).width = width;
            (*psTabPos).height = height;
            return 1 as libc::c_int
        }
        y =
            (y as libc::c_uint).wrapping_add(height.wrapping_add(gap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    }
    /* Didn't find any */
    return 0 as libc::c_int;
}
/* Find which tab is under a form coordinate */
unsafe extern "C" fn formPickTab(mut psForm: *mut W_TABFORM, mut fx: UDWORD,
                                 mut fy: UDWORD, mut psTabPos: *mut TAB_POS)
 -> BOOL {
    let mut x0: SDWORD = 0;
    let mut y0: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    let mut xOffset: SDWORD = 0;
    let mut yOffset: SDWORD = 0;
    let mut xOffset2: SDWORD = 0;
    let mut yOffset2: SDWORD = 0;
    /* Get the basic position of the form */
    x0 = 0 as libc::c_int;
    y0 = 0 as libc::c_int;
    x1 = (*psForm).width as SDWORD;
    y1 = (*psForm).height as SDWORD;
    /* Adjust for where the tabs are */
    if (*psForm).majorPos as libc::c_int == 2 as libc::c_int {
        x0 += (*psForm).tabMajorThickness as libc::c_int
    } else if (*psForm).minorPos as libc::c_int == 2 as libc::c_int {
        x0 += (*psForm).tabMinorThickness as libc::c_int
    }
    if (*psForm).majorPos as libc::c_int == 3 as libc::c_int {
        x1 -= (*psForm).tabMajorThickness as libc::c_int
    } else if (*psForm).minorPos as libc::c_int == 3 as libc::c_int {
        x1 -= (*psForm).tabMinorThickness as libc::c_int
    }
    if (*psForm).majorPos as libc::c_int == 1 as libc::c_int {
        y0 += (*psForm).tabMajorThickness as libc::c_int
    } else if (*psForm).minorPos as libc::c_int == 1 as libc::c_int {
        y0 += (*psForm).tabMinorThickness as libc::c_int
    }
    if (*psForm).majorPos as libc::c_int == 4 as libc::c_int {
        y1 -= (*psForm).tabMajorThickness as libc::c_int
    } else if (*psForm).minorPos as libc::c_int == 4 as libc::c_int {
        y1 -= (*psForm).tabMinorThickness as libc::c_int
    }
    //		/* Adjust for where the tabs are */
//	if (psForm->majorPos == WFORM_TABLEFT || psForm->minorPos == WFORM_TABLEFT)
//	{
//		x0 += psForm->tabThickness;
//	}
//	if (psForm->majorPos == WFORM_TABRIGHT || psForm->minorPos == WFORM_TABRIGHT)
//	{
//		x1 -= psForm->tabThickness;
//	}
//	if (psForm->majorPos == WFORM_TABTOP || psForm->minorPos == WFORM_TABTOP)
//	{
//		y0 += psForm->tabThickness;
//	}
//	if (psForm->majorPos == WFORM_TABBOTTOM || psForm->minorPos == WFORM_TABBOTTOM)
//	{
//		y1 -= psForm->tabThickness;
//	}
    yOffset = 0 as libc::c_int;
    xOffset = yOffset;
    match (*psForm).minorPos as libc::c_int {
        1 => { yOffset = (*psForm).tabVertOffset as SDWORD }
        2 => { xOffset = (*psForm).tabHorzOffset as SDWORD }
        4 => { yOffset = (*psForm).tabVertOffset as SDWORD }
        3 => { xOffset = (*psForm).tabHorzOffset as SDWORD }
        _ => { }
    }
    yOffset2 = 0 as libc::c_int;
    xOffset2 = yOffset2;
    /* Check the major tabs */
    match (*psForm).majorPos as libc::c_int {
        1 => {
            if formPickHTab(psTabPos,
                            x0 + (*psForm).majorOffset as libc::c_int -
                                xOffset,
                            y0 - (*psForm).tabMajorThickness as libc::c_int,
                            (*psForm).majorSize as UDWORD,
                            (*psForm).tabMajorThickness as UDWORD,
                            (*psForm).tabMajorGap as UDWORD,
                            (*psForm).numMajor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                return 1 as libc::c_int
            }
            yOffset2 = -((*psForm).tabVertOffset as libc::c_int)
        }
        4 => {
            if formPickHTab(psTabPos,
                            x0 + (*psForm).majorOffset as libc::c_int -
                                xOffset, y1, (*psForm).majorSize as UDWORD,
                            (*psForm).tabMajorThickness as UDWORD,
                            (*psForm).tabMajorGap as UDWORD,
                            (*psForm).numMajor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                return 1 as libc::c_int
            }
        }
        2 => {
            if formPickVTab(psTabPos,
                            x0 - (*psForm).tabMajorThickness as libc::c_int,
                            y0 + (*psForm).majorOffset as libc::c_int -
                                yOffset,
                            (*psForm).tabMajorThickness as UDWORD,
                            (*psForm).majorSize as UDWORD,
                            (*psForm).tabMajorGap as UDWORD,
                            (*psForm).numMajor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                return 1 as libc::c_int
            }
            xOffset2 = (*psForm).tabHorzOffset as SDWORD
        }
        3 => {
            if formPickVTab(psTabPos, x1,
                            y0 + (*psForm).majorOffset as libc::c_int -
                                yOffset,
                            (*psForm).tabMajorThickness as UDWORD,
                            (*psForm).majorSize as UDWORD,
                            (*psForm).tabMajorGap as UDWORD,
                            (*psForm).numMajor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                return 1 as libc::c_int
            }
        }
        0 => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formDisplayTabbed: Cannot have a tabbed form with no major tabs\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      1022 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"formPickTab\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        _ => { }
    }
    /* Draw the minor tabs */
    psMajor =
        (*psForm).asMajor.as_mut_ptr().offset((*psForm).majorT as libc::c_int
                                                  as isize);
    match (*psForm).minorPos as libc::c_int {
        1 => {
            if formPickHTab(psTabPos,
                            x0 + (*psForm).minorOffset as libc::c_int -
                                xOffset2,
                            y0 - (*psForm).tabMinorThickness as libc::c_int,
                            (*psForm).minorSize as UDWORD,
                            (*psForm).tabMinorThickness as UDWORD,
                            (*psForm).tabMinorGap as UDWORD,
                            (*psMajor).numMinor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                (*psTabPos).index += (*psForm).numMajor as libc::c_int;
                return 1 as libc::c_int
            }
            /* case WFORM_TABNONE - no minor tabs so nothing to display */
        }
        4 => {
            if formPickHTab(psTabPos,
                            x0 + (*psForm).minorOffset as libc::c_int -
                                xOffset2, y1, (*psForm).minorSize as UDWORD,
                            (*psForm).tabMinorThickness as UDWORD,
                            (*psForm).tabMinorGap as UDWORD,
                            (*psMajor).numMinor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                (*psTabPos).index += (*psForm).numMajor as libc::c_int;
                return 1 as libc::c_int
            }
        }
        2 => {
            if formPickVTab(psTabPos,
                            x0 + xOffset -
                                (*psForm).tabMinorThickness as libc::c_int,
                            y0 + (*psForm).minorOffset as libc::c_int -
                                yOffset2,
                            (*psForm).tabMinorThickness as UDWORD,
                            (*psForm).minorSize as UDWORD,
                            (*psForm).tabMinorGap as UDWORD,
                            (*psMajor).numMinor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                (*psTabPos).index += (*psForm).numMajor as libc::c_int;
                return 1 as libc::c_int
            }
        }
        3 => {
            if formPickVTab(psTabPos, x1 + xOffset,
                            y0 + (*psForm).minorOffset as libc::c_int -
                                yOffset2,
                            (*psForm).tabMinorThickness as UDWORD,
                            (*psForm).minorSize as UDWORD,
                            (*psForm).tabMinorGap as UDWORD,
                            (*psMajor).numMinor as UDWORD, fx as SDWORD,
                            fy as SDWORD) != 0 {
                (*psTabPos).index += (*psForm).numMajor as libc::c_int;
                return 1 as libc::c_int
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
/* Run a form widget */
/* Run a form widget */
#[no_mangle]
pub unsafe extern "C" fn formRun(mut psWidget: *mut W_FORM,
                                 mut psContext: *mut W_CONTEXT) {
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    let mut sTabPos: TAB_POS =
        TAB_POS{index: 0, x: 0, y: 0, width: 0, height: 0,};
    let mut pTip: *mut STRING = 0 as *mut STRING;
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        if (*(psWidget as *mut W_CLICKFORM)).state &
               0x20 as libc::c_int as libc::c_uint != 0 {
            if gameTime2.wrapping_div(250 as libc::c_int as
                                          libc::c_uint).wrapping_rem(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                   == 0 as libc::c_int as libc::c_uint {
                let ref mut fresh1 = (*(psWidget as *mut W_CLICKFORM)).state;
                *fresh1 &= !(0x40 as libc::c_int) as libc::c_uint
            } else {
                let ref mut fresh2 = (*(psWidget as *mut W_CLICKFORM)).state;
                *fresh2 |= 0x40 as libc::c_int as libc::c_uint
            }
        }
    }
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        mx = (*psContext).mx;
        my = (*psContext).my;
        psTabForm = psWidget as *mut W_TABFORM;
        /* If the mouse is over the form, see if any tabs need to be hilited */
        if mx >= 0 as libc::c_int && mx <= (*psTabForm).width as libc::c_int
               && my >= 0 as libc::c_int &&
               my <= (*psTabForm).height as libc::c_int {
            if formPickTab(psTabForm, mx as UDWORD, my as UDWORD,
                           &mut sTabPos) != 0 {
                if (*psTabForm).tabHiLite as libc::c_int !=
                       sTabPos.index as UWORD as libc::c_int {
                    /* Got a new tab - start the tool tip if there is one */
                    (*psTabForm).tabHiLite = sTabPos.index as UWORD;
                    if sTabPos.index >= (*psTabForm).numMajor as libc::c_int {
                        pTip =
                            (*psTabForm).asMajor[(*psTabForm).majorT as
                                                     usize].asMinor[(sTabPos.index
                                                                         -
                                                                         (*psTabForm).numMajor
                                                                             as
                                                                             libc::c_int)
                                                                        as
                                                                        usize].pTip
                    } else {
                        pTip =
                            (*psTabForm).asMajor[sTabPos.index as usize].pTip
                    }
                    if !pTip.is_null() {
                        /* Got a tip - start it off */
                        tipStart(psTabForm as *mut WIDGET, pTip,
                                 (*(*psContext).psScreen).TipFontID,
                                 (*psTabForm).aColours.as_mut_ptr(),
                                 sTabPos.x + (*psContext).xOffset,
                                 sTabPos.y + (*psContext).yOffset,
                                 sTabPos.width, sTabPos.height);
                    } else {
                        /* No tip - clear any old tip */
                        tipStop(psWidget as *mut WIDGET);
                    }
                }
            } else {
                /* No tab - clear the tool tip */
                tipStop(psWidget as *mut WIDGET);
                /* And clear the hilite */
                (*psTabForm).tabHiLite = -(1 as libc::c_int) as UWORD
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn formSetFlash(mut psWidget: *mut W_FORM) {
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh3 = (*(psWidget as *mut W_CLICKFORM)).state;
        *fresh3 |= 0x20 as libc::c_int as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn formClearFlash(mut psWidget: *mut W_FORM) {
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh4 = (*(psWidget as *mut W_CLICKFORM)).state;
        *fresh4 &= !(0x20 as libc::c_int) as libc::c_uint;
        let ref mut fresh5 = (*(psWidget as *mut W_CLICKFORM)).state;
        *fresh5 &= !(0x40 as libc::c_int) as libc::c_uint
    };
}
/* Respond to a mouse click */
/* Respond to a mouse click */
#[no_mangle]
pub unsafe extern "C" fn formClicked(mut psWidget: *mut W_FORM,
                                     mut key: UDWORD) {
    let mut psClickForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    /* Stop the tip if there is one */
    tipStop(psWidget as *mut WIDGET);
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        /* Can't click a button if it is disabled or locked down */
        if (*(psWidget as *mut W_CLICKFORM)).state &
               (0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint == 0
           {
            // Check this is the correct key
            if (*psWidget).style & 0x10 as libc::c_int as libc::c_uint == 0 &&
                   key == 1 as libc::c_int as libc::c_uint ||
                   (*psWidget).style & 0x20 as libc::c_int as libc::c_uint !=
                       0 && key == 2 as libc::c_int as libc::c_uint {
                let ref mut fresh6 =
                    (*(psWidget as
                           *mut W_CLICKFORM)).state; // Stop it flashing
                *fresh6 &= !(0x20 as libc::c_int) as libc::c_uint;
                let ref mut fresh7 = (*(psWidget as *mut W_CLICKFORM)).state;
                *fresh7 &= !(0x40 as libc::c_int) as libc::c_uint;
                let ref mut fresh8 = (*(psWidget as *mut W_CLICKFORM)).state;
                *fresh8 |= 0x1 as libc::c_int as libc::c_uint;
                psClickForm = psWidget as *mut W_CLICKFORM;
                if (*psClickForm).AudioCallback.is_some() {
                    (*psClickForm).AudioCallback.expect("non-null function pointer")((*psClickForm).ClickedAudioID
                                                                                         as
                                                                                         libc::c_int);
                }
            }
        }
    };
}
/* Respond to a mouse form up */
/* Respond to a mouse form up */
#[no_mangle]
pub unsafe extern "C" fn formReleased(mut psWidget: *mut W_FORM,
                                      mut key: UDWORD,
                                      mut psContext: *mut W_CONTEXT) {
    let mut psTabForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut psClickForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    let mut sTabPos: TAB_POS =
        TAB_POS{index: 0, x: 0, y: 0, width: 0, height: 0,};
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        psTabForm = psWidget as *mut W_TABFORM;
        /* See if a tab has been clicked on */
        if formPickTab(psTabForm, (*psContext).mx as UDWORD,
                       (*psContext).my as UDWORD, &mut sTabPos) != 0 {
            if sTabPos.index >= (*psTabForm).numMajor as libc::c_int {
                /* Clicked on a minor tab */
                (*psTabForm).minorT =
                    (sTabPos.index - (*psTabForm).numMajor as libc::c_int) as
                        UWORD;
                (*psTabForm).asMajor[(*psTabForm).majorT as usize].lastMinor =
                    (*psTabForm).minorT;
                widgSetReturn(psWidget as *mut WIDGET);
            } else {
                /* Clicked on a major tab */
                (*psTabForm).majorT = sTabPos.index as UWORD;
                (*psTabForm).minorT =
                    (*psTabForm).asMajor[sTabPos.index as usize].lastMinor;
                widgSetReturn(psWidget as *mut WIDGET);
            }
        }
    } else if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        psClickForm = psWidget as *mut W_CLICKFORM;
        if (*psClickForm).state & 0x1 as libc::c_int as libc::c_uint != 0 {
            // Check this is the correct key
            if (*psWidget).style & 0x10 as libc::c_int as libc::c_uint == 0 &&
                   key == 1 as libc::c_int as libc::c_uint ||
                   (*psWidget).style & 0x20 as libc::c_int as libc::c_uint !=
                       0 && key == 2 as libc::c_int as libc::c_uint {
                widgSetReturn(psClickForm as *mut WIDGET);
                (*psClickForm).state &= !(0x1 as libc::c_int) as libc::c_uint
            }
        }
    };
}
/* Respond to a mouse moving over a form */
/* Respond to a mouse moving over a form */
#[no_mangle]
pub unsafe extern "C" fn formHiLite(mut psWidget: *mut W_FORM,
                                    mut psContext: *mut W_CONTEXT) {
    let mut psClickForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        psClickForm = psWidget as *mut W_CLICKFORM;
        (*psClickForm).state |= 0x4 as libc::c_int as libc::c_uint;
        /* If there is a tip string start the tool tip */
        if !(*psClickForm).pTip.is_null() {
            tipStart(psClickForm as *mut WIDGET, (*psClickForm).pTip,
                     (*(*psContext).psScreen).TipFontID,
                     (*(*psContext).psForm).aColours.as_mut_ptr(),
                     (*psWidget).x as libc::c_int + (*psContext).xOffset,
                     (*psWidget).y as libc::c_int + (*psContext).yOffset,
                     (*psWidget).width as UDWORD,
                     (*psWidget).height as UDWORD);
        }
        if (*psClickForm).AudioCallback.is_some() {
            (*psClickForm).AudioCallback.expect("non-null function pointer")((*psClickForm).HilightAudioID
                                                                                 as
                                                                                 libc::c_int);
        }
    };
}
/* Respond to the mouse moving off a form */
/* Respond to the mouse moving off a form */
#[no_mangle]
pub unsafe extern "C" fn formHiLiteLost(mut psWidget: *mut W_FORM,
                                        mut psContext: *mut W_CONTEXT) {
    /* If one of the widgets were hilited that has to loose it as well */
    if !(*psWidget).psLastHiLite.is_null() {
        widgHiLiteLost((*psWidget).psLastHiLite, psContext);
    }
    if (*psWidget).style & 1 as libc::c_int as libc::c_uint != 0 {
        (*(psWidget as *mut W_TABFORM)).tabHiLite =
            -(1 as libc::c_int) as UWORD
    }
    if (*psWidget).style & 4 as libc::c_int as libc::c_uint != 0 {
        let ref mut fresh9 = (*(psWidget as *mut W_CLICKFORM)).state;
        *fresh9 &= !(0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint
    }
    /* Clear the tool tip if there is one */
    tipStop(psWidget as *mut WIDGET);
}
/* Display function prototypes */
/* Display a form */
#[no_mangle]
pub unsafe extern "C" fn formDisplay(mut psWidget: *mut WIDGET,
                                     mut xOffset: UDWORD, mut yOffset: UDWORD,
                                     mut pColours: *mut UDWORD) {
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    if (*psWidget).style & 2 as libc::c_int as libc::c_uint == 0 {
        x0 = ((*psWidget).x as libc::c_uint).wrapping_add(xOffset);
        y0 = ((*psWidget).y as libc::c_uint).wrapping_add(yOffset);
        x1 = x0.wrapping_add((*psWidget).width as libc::c_uint);
        y1 = y0.wrapping_add((*psWidget).height as libc::c_uint);
        pie_BoxFillIndex(x0.wrapping_add(1 as libc::c_int as libc::c_uint) as
                             libc::c_int,
                         y0.wrapping_add(1 as libc::c_int as libc::c_uint) as
                             libc::c_int,
                         x1.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                             libc::c_int,
                         y1.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                             libc::c_int,
                         WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
    };
}
/* Display a clickable form */
#[no_mangle]
pub unsafe extern "C" fn formDisplayClickable(mut psWidget: *mut WIDGET,
                                              mut xOffset: UDWORD,
                                              mut yOffset: UDWORD,
                                              mut pColours: *mut UDWORD) {
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut psForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    psForm = psWidget as *mut W_CLICKFORM;
    x0 = ((*psWidget).x as libc::c_uint).wrapping_add(xOffset);
    y0 = ((*psWidget).y as libc::c_uint).wrapping_add(yOffset);
    x1 = x0.wrapping_add((*psWidget).width as libc::c_uint);
    y1 = y0.wrapping_add((*psWidget).height as libc::c_uint);
    /* Fill the background */
    pie_BoxFillIndex(x0.wrapping_add(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     y0.wrapping_add(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     x1.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                         libc::c_int,
                     y1.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                         libc::c_int, WCOL_BKGRND as libc::c_int as UBYTE);
    /* Display the border */
    if (*psForm).state &
           (0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int) as
               libc::c_uint != 0 {
        /* Form down */
        pie_Line(x0 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
    } else {
        /* Form up */
        pie_Line(x0 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
    };
}
/* Draw top tabs */
unsafe extern "C" fn formDisplayTTabs(mut psForm: *mut W_TABFORM,
                                      mut x0: SDWORD, mut y0: SDWORD,
                                      mut width: UDWORD, mut height: UDWORD,
                                      mut number: UDWORD,
                                      mut selected: UDWORD,
                                      mut hilite: UDWORD,
                                      mut pColours: *mut UDWORD,
                                      mut TabType: UDWORD,
                                      mut TabGap: UDWORD) {
    let mut x: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't display single tabs */
        return
    }
    x = x0 + 2 as libc::c_int;
    x1 =
        (x as
             libc::c_uint).wrapping_add(width).wrapping_sub(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
            as SDWORD;
    y1 = (y0 as libc::c_uint).wrapping_add(height) as SDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < number {
        if (*psForm).pTabDisplay.is_some() {
            (*psForm).pTabDisplay.expect("non-null function pointer")(psForm
                                                                          as
                                                                          *mut WIDGET,
                                                                      TabType,
                                                                      1 as
                                                                          libc::c_int
                                                                          as
                                                                          UDWORD,
                                                                      i,
                                                                      (i ==
                                                                           selected)
                                                                          as
                                                                          libc::c_int,
                                                                      (i ==
                                                                           hilite)
                                                                          as
                                                                          libc::c_int,
                                                                      x as
                                                                          UDWORD,
                                                                      y0 as
                                                                          UDWORD,
                                                                      width,
                                                                      height);
        } else {
            if i == selected {
                /* Fill in the tab */
                pie_BoxFillIndex(x + 1 as libc::c_int, y0 + 1 as libc::c_int,
                                 x1 - 1 as libc::c_int, y1,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x, y0 + 2 as libc::c_int, x, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x, y0 + 2 as libc::c_int, x + 2 as libc::c_int, y0,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x + 2 as libc::c_int, y0, x1 - 1 as libc::c_int, y0,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x1, y0 + 1 as libc::c_int, x1, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            } else {
                /* Fill in the tab */
                pie_BoxFillIndex(x + 1 as libc::c_int, y0 + 2 as libc::c_int,
                                 x1 - 1 as libc::c_int, y1 - 1 as libc::c_int,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x, y0 + 3 as libc::c_int, x, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x, y0 + 3 as libc::c_int, x + 2 as libc::c_int,
                         y0 + 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x + 2 as libc::c_int, y0 + 1 as libc::c_int,
                         x1 - 1 as libc::c_int, y0 + 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x1, y0 + 2 as libc::c_int, x1, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            }
            if i == hilite {
                /* Draw the hilite box */
                pie_Box(x + 2 as libc::c_int, y0 + 4 as libc::c_int,
                        x1 - 3 as libc::c_int, y1 - 3 as libc::c_int,
                        *pColours.offset(WCOL_HILITE as libc::c_int as
                                             isize));
            }
        }
        x =
            (x as libc::c_uint).wrapping_add(width.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        x1 =
            (x1 as libc::c_uint).wrapping_add(width.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    };
}
/* Draw bottom tabs */
unsafe extern "C" fn formDisplayBTabs(mut psForm: *mut W_TABFORM,
                                      mut x0: SDWORD, mut y0: SDWORD,
                                      mut width: UDWORD, mut height: UDWORD,
                                      mut number: UDWORD,
                                      mut selected: UDWORD,
                                      mut hilite: UDWORD,
                                      mut pColours: *mut UDWORD,
                                      mut TabType: UDWORD,
                                      mut TabGap: UDWORD) {
    let mut x: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't display single tabs */
        return
    }
    x = x0 + 2 as libc::c_int;
    x1 =
        (x as
             libc::c_uint).wrapping_add(width).wrapping_sub(2 as libc::c_int
                                                                as
                                                                libc::c_uint)
            as SDWORD;
    y1 = (y0 as libc::c_uint).wrapping_add(height) as SDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < number {
        if (*psForm).pTabDisplay.is_some() {
            (*psForm).pTabDisplay.expect("non-null function pointer")(psForm
                                                                          as
                                                                          *mut WIDGET,
                                                                      TabType,
                                                                      4 as
                                                                          libc::c_int
                                                                          as
                                                                          UDWORD,
                                                                      i,
                                                                      (i ==
                                                                           selected)
                                                                          as
                                                                          libc::c_int,
                                                                      (i ==
                                                                           hilite)
                                                                          as
                                                                          libc::c_int,
                                                                      x as
                                                                          UDWORD,
                                                                      y0 as
                                                                          UDWORD,
                                                                      width,
                                                                      height);
        } else {
            if i == selected {
                /* Fill in the tab */
                pie_BoxFillIndex(x + 1 as libc::c_int, y0,
                                 x1 - 1 as libc::c_int, y1 - 1 as libc::c_int,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x, y0, x, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x, y1, x1 - 3 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 2 as libc::c_int, y1, x1, y1 - 2 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1, y1 - 3 as libc::c_int, x1, y0 + 1 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            } else {
                /* Fill in the tab */
                pie_BoxFillIndex(x + 1 as libc::c_int, y0 + 1 as libc::c_int,
                                 x1 - 1 as libc::c_int, y1 - 2 as libc::c_int,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x, y0 + 1 as libc::c_int, x, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x + 1 as libc::c_int, y1 - 1 as libc::c_int,
                         x1 - 3 as libc::c_int, y1 - 1 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 2 as libc::c_int, y1 - 1 as libc::c_int, x1,
                         y1 - 3 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1, y1 - 4 as libc::c_int, x1, y0 + 1 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            }
            if i == hilite {
                /* Draw the hilite box */
                pie_Box(x + 2 as libc::c_int, y0 + 3 as libc::c_int,
                        x1 - 3 as libc::c_int, y1 - 4 as libc::c_int,
                        *pColours.offset(WCOL_HILITE as libc::c_int as
                                             isize));
            }
        }
        x =
            (x as libc::c_uint).wrapping_add(width.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        x1 =
            (x1 as libc::c_uint).wrapping_add(width.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    };
}
/* Draw left tabs */
unsafe extern "C" fn formDisplayLTabs(mut psForm: *mut W_TABFORM,
                                      mut x0: SDWORD, mut y0: SDWORD,
                                      mut width: UDWORD, mut height: UDWORD,
                                      mut number: UDWORD,
                                      mut selected: UDWORD,
                                      mut hilite: UDWORD,
                                      mut pColours: *mut UDWORD,
                                      mut TabType: UDWORD,
                                      mut TabGap: UDWORD) {
    let mut x1: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't display single tabs */
        return
    }
    x1 = (x0 as libc::c_uint).wrapping_add(width) as SDWORD;
    y = y0 + 2 as libc::c_int;
    y1 =
        (y as
             libc::c_uint).wrapping_add(height).wrapping_sub(2 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
            as SDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < number as SDWORD as libc::c_uint {
        if (*psForm).pTabDisplay.is_some() {
            (*psForm).pTabDisplay.expect("non-null function pointer")(psForm
                                                                          as
                                                                          *mut WIDGET,
                                                                      TabType,
                                                                      2 as
                                                                          libc::c_int
                                                                          as
                                                                          UDWORD,
                                                                      i,
                                                                      (i ==
                                                                           selected)
                                                                          as
                                                                          libc::c_int,
                                                                      (i ==
                                                                           hilite)
                                                                          as
                                                                          libc::c_int,
                                                                      x0 as
                                                                          UDWORD,
                                                                      y as
                                                                          UDWORD,
                                                                      width,
                                                                      height);
        } else {
            if i == selected {
                /* Fill in the tab */
                pie_BoxFillIndex(x0 + 1 as libc::c_int, y + 1 as libc::c_int,
                                 x1, y1 - 1 as libc::c_int,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x0, y, x1 - 1 as libc::c_int, y,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x0, y + 1 as libc::c_int, x0, y1 - 2 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x0 + 1 as libc::c_int, y1 - 1 as libc::c_int,
                         x0 + 2 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x0 + 3 as libc::c_int, y1, x1, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            } else {
                /* Fill in the tab */
                pie_BoxFillIndex(x0 + 2 as libc::c_int, y + 1 as libc::c_int,
                                 x1 - 1 as libc::c_int, y1 - 1 as libc::c_int,
                                 WCOL_BKGRND as libc::c_int as UBYTE);
                /* Draw the outline */
                pie_Line(x0 + 1 as libc::c_int, y, x1 - 1 as libc::c_int, y,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x0 + 1 as libc::c_int, y + 1 as libc::c_int,
                         x0 + 1 as libc::c_int, y1 - 2 as libc::c_int,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x0 + 2 as libc::c_int, y1 - 1 as libc::c_int,
                         x0 + 3 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x0 + 4 as libc::c_int, y1, x1 - 1 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            }
            if i == hilite {
                pie_Box(x0 + 4 as libc::c_int, y + 2 as libc::c_int,
                        x1 - 2 as libc::c_int, y1 - 3 as libc::c_int,
                        *pColours.offset(WCOL_HILITE as libc::c_int as
                                             isize));
            }
        }
        y =
            (y as libc::c_uint).wrapping_add(height.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        y1 =
            (y1 as libc::c_uint).wrapping_add(height.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    };
}
/* Draw right tabs */
unsafe extern "C" fn formDisplayRTabs(mut psForm: *mut W_TABFORM,
                                      mut x0: SDWORD, mut y0: SDWORD,
                                      mut width: UDWORD, mut height: UDWORD,
                                      mut number: UDWORD,
                                      mut selected: UDWORD,
                                      mut hilite: UDWORD,
                                      mut pColours: *mut UDWORD,
                                      mut TabType: UDWORD,
                                      mut TabGap: UDWORD) {
    let mut x1: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut i: UDWORD = 0;
    if number == 1 as libc::c_int as libc::c_uint {
        /* Don't display single tabs */
        return
    }
    x1 = (x0 as libc::c_uint).wrapping_add(width) as SDWORD;
    y = y0 + 2 as libc::c_int;
    y1 =
        (y as
             libc::c_uint).wrapping_add(height).wrapping_sub(2 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
            as SDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < number as SDWORD as libc::c_uint {
        if (*psForm).pTabDisplay.is_some() {
            (*psForm).pTabDisplay.expect("non-null function pointer")(psForm
                                                                          as
                                                                          *mut WIDGET,
                                                                      TabType,
                                                                      3 as
                                                                          libc::c_int
                                                                          as
                                                                          UDWORD,
                                                                      i,
                                                                      (i ==
                                                                           selected)
                                                                          as
                                                                          libc::c_int,
                                                                      (i ==
                                                                           hilite)
                                                                          as
                                                                          libc::c_int,
                                                                      x0 as
                                                                          UDWORD,
                                                                      y as
                                                                          UDWORD,
                                                                      width,
                                                                      height);
        } else {
            if i == selected {
                /* Fill in the tab */
                pie_BoxFillIndex(x0, y + 1 as libc::c_int,
                                 x1 - 1 as libc::c_int, y1 - 1 as libc::c_int,
                                 *pColours.offset(WCOL_BKGRND as libc::c_int
                                                      as isize) as UBYTE);
                /* Draw the outline */
                pie_Line(x0, y, x1 - 1 as libc::c_int, y,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x1, y, x1, y1 - 2 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 1 as libc::c_int, y1 - 1 as libc::c_int,
                         x1 - 2 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 3 as libc::c_int, y1, x0, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            } else {
                /* Fill in the tab */
                pie_BoxFillIndex(x0 + 1 as libc::c_int, y + 1 as libc::c_int,
                                 x1 - 2 as libc::c_int, y1 - 1 as libc::c_int,
                                 *pColours.offset(WCOL_BKGRND as libc::c_int
                                                      as isize) as UBYTE);
                /* Draw the outline */
                pie_Line(x0 + 1 as libc::c_int, y, x1 - 1 as libc::c_int, y,
                         *pColours.offset(WCOL_LIGHT as libc::c_int as
                                              isize));
                pie_Line(x1 - 1 as libc::c_int, y, x1 - 1 as libc::c_int,
                         y1 - 2 as libc::c_int,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 2 as libc::c_int, y1 - 1 as libc::c_int,
                         x1 - 3 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
                pie_Line(x1 - 4 as libc::c_int, y1, x0 + 1 as libc::c_int, y1,
                         *pColours.offset(WCOL_DARK as libc::c_int as isize));
            }
            if i == hilite {
                pie_Box(x0 + 2 as libc::c_int, y + 2 as libc::c_int,
                        x1 - 4 as libc::c_int, y1 - 3 as libc::c_int,
                        *pColours.offset(WCOL_HILITE as libc::c_int as
                                             isize));
            }
        }
        y =
            (y as libc::c_uint).wrapping_add(height.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        y1 =
            (y1 as libc::c_uint).wrapping_add(height.wrapping_add(TabGap)) as
                SDWORD as SDWORD;
        i = i.wrapping_add(1)
    };
}
/* Display a tabbed form */
#[no_mangle]
pub unsafe extern "C" fn formDisplayTabbed(mut psWidget: *mut WIDGET,
                                           mut xOffset: UDWORD,
                                           mut yOffset: UDWORD,
                                           mut pColours: *mut UDWORD) {
    let mut x0: UDWORD = 0;
    let mut y0: UDWORD = 0;
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    let mut psMajor: *mut W_MAJORTAB = 0 as *mut W_MAJORTAB;
    psForm = psWidget as *mut W_TABFORM;
    /* Get the basic position of the form */
    x0 = ((*psForm).x as libc::c_uint).wrapping_add(xOffset);
    y0 = ((*psForm).y as libc::c_uint).wrapping_add(yOffset);
    x1 = x0.wrapping_add((*psForm).width as libc::c_uint);
    y1 = y0.wrapping_add((*psForm).height as libc::c_uint);
    /* Adjust for where the tabs are */
    if (*psForm).majorPos as libc::c_int == 2 as libc::c_int {
        x0 =
            (x0 as
                 libc::c_uint).wrapping_add(((*psForm).tabMajorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*psForm).minorPos as libc::c_int == 2 as libc::c_int {
        x0 =
            (x0 as
                 libc::c_uint).wrapping_add(((*psForm).tabMinorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*psForm).majorPos as libc::c_int == 3 as libc::c_int {
        x1 =
            (x1 as
                 libc::c_uint).wrapping_sub(((*psForm).tabMajorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*psForm).minorPos as libc::c_int == 3 as libc::c_int {
        x1 =
            (x1 as
                 libc::c_uint).wrapping_sub(((*psForm).tabMinorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabHorzOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*psForm).majorPos as libc::c_int == 1 as libc::c_int {
        y0 =
            (y0 as
                 libc::c_uint).wrapping_add(((*psForm).tabMajorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*psForm).minorPos as libc::c_int == 1 as libc::c_int {
        y0 =
            (y0 as
                 libc::c_uint).wrapping_add(((*psForm).tabMinorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    if (*psForm).majorPos as libc::c_int == 4 as libc::c_int {
        y1 =
            (y1 as
                 libc::c_uint).wrapping_sub(((*psForm).tabMajorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    } else if (*psForm).minorPos as libc::c_int == 4 as libc::c_int {
        y1 =
            (y1 as
                 libc::c_uint).wrapping_sub(((*psForm).tabMinorThickness as
                                                 libc::c_int -
                                                 (*psForm).tabVertOffset as
                                                     libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD
    }
    /* Adjust for where the tabs are */
//	if (psForm->majorPos == WFORM_TABLEFT || psForm->minorPos == WFORM_TABLEFT)
//	{
//		x0 += psForm->tabThickness - psForm->tabHorzOffset;
//	}
//	if (psForm->majorPos == WFORM_TABRIGHT || psForm->minorPos == WFORM_TABRIGHT)
//	{
//		x1 -= psForm->tabThickness - psForm->tabHorzOffset;
//	}
//	if (psForm->majorPos == WFORM_TABTOP || psForm->minorPos == WFORM_TABTOP)
//	{
//		y0 += psForm->tabThickness - psForm->tabVertOffset;
//	}
//	if (psForm->majorPos == WFORM_TABBOTTOM || psForm->minorPos == WFORM_TABBOTTOM)
//	{
//		y1 -= psForm->tabThickness - psForm->tabVertOffset;
//	}
    if (*psForm).pFormDisplay.is_some() {
        (*psForm).pFormDisplay.expect("non-null function pointer")(psForm as
                                                                       *mut WIDGET,
                                                                   xOffset,
                                                                   yOffset,
                                                                   (*psForm).aColours.as_mut_ptr());
    } else if (*psForm).style & 2 as libc::c_int as libc::c_uint == 0 {
        pie_BoxFillIndex(x0 as libc::c_int, y0 as libc::c_int,
                         x1 as libc::c_int, y1 as libc::c_int,
                         WCOL_BKGRND as libc::c_int as UBYTE);
        pie_Line(x0 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x0 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y0 as libc::c_int,
                 *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y0 as libc::c_int, x1 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
        pie_Line(x1 as libc::c_int, y1 as libc::c_int, x0 as libc::c_int,
                 y1 as libc::c_int,
                 *pColours.offset(WCOL_DARK as libc::c_int as isize));
    }
    /* Draw the form outline */
    /* Draw the major tabs */
    match (*psForm).majorPos as libc::c_int {
        1 => {
            formDisplayTTabs(psForm,
                             x0.wrapping_add((*psForm).majorOffset as
                                                 libc::c_uint) as SDWORD,
                             y0.wrapping_sub((*psForm).tabMajorThickness as
                                                 libc::c_uint).wrapping_add((*psForm).tabVertOffset
                                                                                as
                                                                                libc::c_uint)
                                 as SDWORD, (*psForm).majorSize as UDWORD,
                             (*psForm).tabMajorThickness as UDWORD,
                             (*psForm).numMajor as UDWORD,
                             (*psForm).majorT as UDWORD,
                             (*psForm).tabHiLite as UDWORD, pColours,
                             1 as libc::c_int as UDWORD,
                             (*psForm).tabMajorGap as UDWORD);
        }
        4 => {
            formDisplayBTabs(psForm,
                             x0.wrapping_add((*psForm).majorOffset as
                                                 libc::c_uint) as SDWORD,
                             y1.wrapping_add((*psForm).tabVertOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).majorSize as UDWORD,
                             (*psForm).tabMajorThickness as UDWORD,
                             (*psForm).numMajor as UDWORD,
                             (*psForm).majorT as UDWORD,
                             (*psForm).tabHiLite as UDWORD, pColours,
                             1 as libc::c_int as UDWORD,
                             (*psForm).tabMajorGap as UDWORD);
        }
        2 => {
            formDisplayLTabs(psForm,
                             x0.wrapping_sub((*psForm).tabMajorThickness as
                                                 libc::c_uint).wrapping_add((*psForm).tabHorzOffset
                                                                                as
                                                                                libc::c_uint)
                                 as SDWORD,
                             y0.wrapping_add((*psForm).majorOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).tabMajorThickness as UDWORD,
                             (*psForm).majorSize as UDWORD,
                             (*psForm).numMajor as UDWORD,
                             (*psForm).majorT as UDWORD,
                             (*psForm).tabHiLite as UDWORD, pColours,
                             1 as libc::c_int as UDWORD,
                             (*psForm).tabMajorGap as UDWORD);
        }
        3 => {
            formDisplayRTabs(psForm,
                             x1.wrapping_sub((*psForm).tabHorzOffset as
                                                 libc::c_uint) as SDWORD,
                             y0.wrapping_add((*psForm).majorOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).tabMajorThickness as UDWORD,
                             (*psForm).majorSize as UDWORD,
                             (*psForm).numMajor as UDWORD,
                             (*psForm).majorT as UDWORD,
                             (*psForm).tabHiLite as UDWORD, pColours,
                             1 as libc::c_int as UDWORD,
                             (*psForm).tabMajorGap as UDWORD);
        }
        0 => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formDisplayTabbed: Cannot have a tabbed form with no major tabs\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"form.c\x00" as *const u8 as *const libc::c_char,
                      1666 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"formDisplayTabbed\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        _ => { }
    }
    /* Draw the minor tabs */
    psMajor =
        (*psForm).asMajor.as_mut_ptr().offset((*psForm).majorT as libc::c_int
                                                  as isize);
    match (*psForm).minorPos as libc::c_int {
        1 => {
            formDisplayTTabs(psForm,
                             x0.wrapping_add((*psForm).minorOffset as
                                                 libc::c_uint) as SDWORD,
                             y0.wrapping_sub((*psForm).tabMinorThickness as
                                                 libc::c_uint).wrapping_add((*psForm).tabVertOffset
                                                                                as
                                                                                libc::c_uint)
                                 as SDWORD, (*psForm).minorSize as UDWORD,
                             (*psForm).tabMinorThickness as UDWORD,
                             (*psMajor).numMinor as UDWORD,
                             (*psForm).minorT as UDWORD,
                             ((*psForm).tabHiLite as libc::c_int -
                                  (*psForm).numMajor as libc::c_int) as
                                 UDWORD, pColours, 0 as libc::c_int as UDWORD,
                             (*psForm).tabMinorGap as UDWORD);
            /* case WFORM_TABNONE - no minor tabs so nothing to display */
        }
        4 => {
            formDisplayBTabs(psForm,
                             x0.wrapping_add((*psForm).minorOffset as
                                                 libc::c_uint) as SDWORD,
                             y1.wrapping_add((*psForm).tabVertOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).minorSize as UDWORD,
                             (*psForm).tabMinorThickness as UDWORD,
                             (*psMajor).numMinor as UDWORD,
                             (*psForm).minorT as UDWORD,
                             ((*psForm).tabHiLite as libc::c_int -
                                  (*psForm).numMajor as libc::c_int) as
                                 UDWORD, pColours, 0 as libc::c_int as UDWORD,
                             (*psForm).tabMinorGap as UDWORD);
        }
        2 => {
            formDisplayLTabs(psForm,
                             x0.wrapping_sub((*psForm).tabMinorThickness as
                                                 libc::c_uint).wrapping_add((*psForm).tabHorzOffset
                                                                                as
                                                                                libc::c_uint).wrapping_add((*psForm).minorOffset
                                                                                                               as
                                                                                                               libc::c_uint)
                                 as SDWORD,
                             y0.wrapping_add((*psForm).minorOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).tabMinorThickness as UDWORD,
                             (*psForm).minorSize as UDWORD,
                             (*psMajor).numMinor as UDWORD,
                             (*psForm).minorT as UDWORD,
                             ((*psForm).tabHiLite as libc::c_int -
                                  (*psForm).numMajor as libc::c_int) as
                                 UDWORD, pColours, 0 as libc::c_int as UDWORD,
                             (*psForm).tabMinorGap as UDWORD);
        }
        3 => {
            formDisplayRTabs(psForm,
                             x1.wrapping_add((*psForm).tabHorzOffset as
                                                 libc::c_uint) as SDWORD,
                             y0.wrapping_add((*psForm).minorOffset as
                                                 libc::c_uint) as SDWORD,
                             (*psForm).tabMinorThickness as UDWORD,
                             (*psForm).minorSize as UDWORD,
                             (*psMajor).numMinor as UDWORD,
                             (*psForm).minorT as UDWORD,
                             ((*psForm).tabHiLite as libc::c_int -
                                  (*psForm).numMajor as libc::c_int) as
                                 UDWORD, pColours, 0 as libc::c_int as UDWORD,
                             (*psForm).tabMinorGap as UDWORD);
        }
        _ => { }
    };
}
