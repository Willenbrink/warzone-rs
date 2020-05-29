use ::libc;
extern "C" {
    pub type _w_form;
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* This returns true if the mouse key went from being up to being down this frame */
    #[no_mangle]
    fn mousePressed(code: MOUSE_KEY_CODE) -> BOOL;
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
    fn GetTickCount() -> DWORD;
    /* The Current screen size and bit depth */
    #[no_mangle]
    static mut screenWidth: UDWORD;
    #[no_mangle]
    static mut screenHeight: UDWORD;
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
    fn iV_SetTextColour(Index: SWORD);
    #[no_mangle]
    fn iV_SetFont(FontID_0: libc::c_int);
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
    fn pie_Box(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
               y1: libc::c_int, colour: uint32);
    #[no_mangle]
    fn pie_BoxFillIndex(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                        y1: libc::c_int, colour: UBYTE);
    #[no_mangle]
    fn iV_GetTextAboveBase() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextLineSize() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextBelowBase() -> libc::c_int;
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
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
/* The tool tip state */
pub type _tip_state = libc::c_uint;
// A tip is being displayed
// A button is hilited, but not yet ready to show the tip
pub const TIP_ACTIVE: _tip_state = 2;
// No tip, and no button hilited
pub const TIP_WAIT: _tip_state = 1;
pub const TIP_NONE: _tip_state = 0;
pub type uint32 = libc::c_uint;
static mut tipState: _tip_state = TIP_NONE;
static mut startTime: SDWORD = 0;
// When the tip was created
static mut mx: SDWORD = 0;
static mut my: SDWORD = 0;
// Last mouse coords
static mut wx: SDWORD = 0;
static mut wy: SDWORD = 0;
static mut ww: SDWORD = 0;
static mut wh: SDWORD = 0;
// Position and size of button to place tip by
static mut tx: SDWORD = 0;
static mut ty: SDWORD = 0;
static mut tw: SDWORD = 0;
static mut th: SDWORD = 0;
// Position and size of the tip box
static mut fx: SDWORD = 0;
static mut fy: SDWORD = 0;
// Position of the text
static mut pTip: *mut STRING = 0 as *const STRING as *mut STRING;
// Tip text
static mut pColours: *mut UDWORD = 0 as *const UDWORD as *mut UDWORD;
// The colours for the tool tip
static mut psWidget: *mut WIDGET = 0 as *const WIDGET as *mut WIDGET;
// The button the tip is for
//static PROP_FONT	*psFont;			// The font to display the tip with
static mut FontID: libc::c_int = 0 as libc::c_int;
// ID for the Ivis Font.
static mut TipColour: libc::c_int = 0;
/*
 * Tip.h
 *
 * Interface to the tool tip display module
 *
 */
/* Initialise the tool tip module */
/* Initialise the tool tip module */
#[no_mangle]
pub unsafe extern "C" fn tipInitialise() { tipState = TIP_NONE; }
// Set the global toop tip text colour.
// Set the global toop tip text colour.
#[no_mangle]
pub unsafe extern "C" fn widgSetTipColour(mut psScreen: *mut W_SCREEN,
                                          mut red: UBYTE, mut green: UBYTE,
                                          mut blue: UBYTE) {
    TipColour = -(1 as libc::c_int);
    // use bitmap colourings.
}
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
//void tipStart(WIDGET *psSource, STRING *pNewTip, PROP_FONT *psNewFont,
#[no_mangle]
pub unsafe extern "C" fn tipStart(mut psSource: *mut WIDGET,
                                  mut pNewTip: *mut STRING,
                                  mut NewFontID: libc::c_int,
                                  mut pNewColours: *mut UDWORD, mut x: SDWORD,
                                  mut y: SDWORD, mut width: UDWORD,
                                  mut height: UDWORD) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"tipStart: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"tip.c\x00" as *const u8 as *const libc::c_char,
              79 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"tipStart\x00")).as_ptr(),
              b"PTRVALID(psSource, sizeof(WIDGET))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //	ASSERT( PTRVALID(pNewTip, WIDG_MAXSTR),
//		"tipStart: Invalid tip pointer" );
//	ASSERT( PTRVALID(psNewFont, sizeof(PROP_FONT)),
//		"tipStart: Invalid font pointer" );
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"tipStart: Invalid colours pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"tip.c\x00" as *const u8 as *const libc::c_char,
              85 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"tipStart\x00")).as_ptr(),
              b"PTRVALID(pNewColours, sizeof(UDWORD) * WCOL_MAX)\x00" as
                  *const u8 as *const libc::c_char);
    };
    tipState = TIP_WAIT;
    startTime = GetTickCount();
    mx = mouseX();
    my = mouseY();
    wx = x;
    wy = y;
    ww = width as SDWORD;
    wh = height as SDWORD;
    pTip = pNewTip;
    psWidget = psSource;
    FontID = NewFontID;
    pColours = pNewColours;
}
/* Stop a tool tip (e.g. if the hilite is lost on a button).
 * psSource should be the same as the widget that started the tip.
 */
/* Stop a tool tip (e.g. if the hilite is lost on a button).
 * psSource should be the same as the widget that started the tip.
 */
#[no_mangle]
pub unsafe extern "C" fn tipStop(mut psSource: *mut WIDGET) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"tipStop: Invalid widget pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"tip.c\x00" as *const u8 as *const libc::c_char,
              106 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"tipStop\x00")).as_ptr(),
              b"PTRVALID(psSource, sizeof(WIDGET))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if tipState as libc::c_uint != TIP_NONE as libc::c_int as libc::c_uint &&
           psSource == psWidget {
        tipState = TIP_NONE
    };
}
/* Update and possibly display the tip */
/* Update and possibly display the tip */
#[no_mangle]
pub unsafe extern "C" fn tipDisplay() {
    let mut newMX: SDWORD = 0;
    let mut newMY: SDWORD = 0;
    let mut currTime: SDWORD = 0;
    let mut fw: SDWORD = 0;
    let mut topGap: SDWORD = 0;
    //	UDWORD		time;
    match tipState as libc::c_uint {
        1 => {
            /* See if the tip has to be shown */
            newMX = mouseX();
            newMY = mouseY();
            currTime = GetTickCount();
            if newMX == mx && newMY == my &&
                   currTime - startTime > 200 as libc::c_int {
                /* Activate the tip */
                tipState = TIP_ACTIVE;
                /* Calculate the size of the tip box */
                topGap = 3 as libc::c_int;
                iV_SetFont(FontID);
                fw = iV_GetTextWidth(pTip);
                tw = fw + 6 as libc::c_int * 2 as libc::c_int;
                th =
                    topGap * 2 as libc::c_int + iV_GetTextLineSize() +
                        iV_GetTextBelowBase();
                /* Position the tip box */
                tx = wx + (ww >> 1 as libc::c_int);
                ty = wy + wh + 3 as libc::c_int;
                /* Check the box is on screen */
                if tx < 0 as libc::c_int { tx = 0 as libc::c_int }
                if tx + tw >= screenWidth as SDWORD - 0 as libc::c_int {
                    tx =
                        screenWidth.wrapping_sub(0 as libc::c_int as
                                                     libc::c_uint).wrapping_sub(tw
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub(1
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint)
                            as SDWORD
                }
                if ty < 0 as libc::c_int { ty = 0 as libc::c_int }
                if ty + th >= screenHeight as SDWORD - 0 as libc::c_int {
                    /* Position the tip above the button */
                    ty = wy - th - 3 as libc::c_int
                }
                /* Position the text */
                fx = tx + 6 as libc::c_int;
                fy =
                    ty + (th - iV_GetTextLineSize()) / 2 as libc::c_int -
                        iV_GetTextAboveBase();
                /* Note the time */
                startTime = GetTickCount()
            } else if newMX != mx || newMY != my ||
                          mousePressed(MOUSE_LMB) != 0 {
                mx = newMX;
                my = newMY;
                startTime = currTime
            }
        }
        2 => {
            /* See if the tip still needs to be displayed */
//		time = GetTickCount();
//		if (mousePressed(MOUSE_LMB) ||
//			((time - startTime) > TIP_TIME))
//		{
//			tipState = TIP_NONE;
//			return;
//		}
            /* Draw the tool tip */
            pie_BoxFillIndex(tx, ty, tx + tw, ty + th,
                             *pColours.offset(WCOL_TIPBKGRND as libc::c_int as
                                                  isize) as UBYTE);
            pie_Box(tx, ty, tx + tw - 1 as libc::c_int,
                    ty + th - 1 as libc::c_int,
                    *pColours.offset(WCOL_LIGHT as libc::c_int as isize));
            pie_Line(tx + 1 as libc::c_int, ty + th - 2 as libc::c_int,
                     tx + 1 as libc::c_int, ty + 1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(tx + 2 as libc::c_int, ty + 1 as libc::c_int,
                     tx + tw - 2 as libc::c_int, ty + 1 as libc::c_int,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(tx, ty + th, tx + tw, ty + th,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            pie_Line(tx + tw, ty + th - 1 as libc::c_int, tx + tw, ty,
                     *pColours.offset(WCOL_DARK as libc::c_int as isize));
            iV_SetFont(FontID);
            //		iV_SetTextColour((UWORD)*(pColours + WCOL_TEXT));
            iV_SetTextColour(TipColour as UWORD as SWORD);
            pie_DrawText(pTip, fx as UDWORD, fy as UDWORD);
        }
        _ => { }
    };
}
