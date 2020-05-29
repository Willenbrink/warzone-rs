use ::libc;
extern "C" {
    pub type _w_context;
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
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* Warps the mouse to the given position */
    #[no_mangle]
    fn SetMousePos(nowt: UDWORD, x: UDWORD, y: UDWORD);
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
}
pub type size_t = libc::c_uint;
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
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
pub type WIDGET = _widget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
pub type SNAPDIRECTION = libc::c_uint;
pub const SNAP_NEAREST: SNAPDIRECTION = 4;
pub const SNAP_LEFT: SNAPDIRECTION = 3;
pub const SNAP_DOWN: SNAPDIRECTION = 2;
pub const SNAP_RIGHT: SNAPDIRECTION = 1;
pub const SNAP_UP: SNAPDIRECTION = 0;
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
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
/*
 * Cursor handling for keyboard/dpad control
 * Pumpkin Studios 98
 */
#[no_mangle]
pub static mut DefaultBias: SNAPBIAS =
    {
        let mut init =
            SNAPBIAS{NDxBias: 1 as libc::c_int as SWORD,
                     NDyBias: 1 as libc::c_int as SWORD,
                     UDxBias: 8 as libc::c_int as SWORD,
                     UDyBias: 1 as libc::c_int as SWORD,
                     RDxBias: 1 as libc::c_int as SWORD,
                     RDyBias: 8 as libc::c_int as SWORD,
                     DDxBias: 8 as libc::c_int as SWORD,
                     DDyBias: 1 as libc::c_int as SWORD,
                     LDxBias: 1 as libc::c_int as SWORD,
                     LDyBias: 8 as libc::c_int as SWORD,};
        init
    };
#[no_mangle]
pub static mut FrontendBias: SNAPBIAS =
    {
        let mut init =
            SNAPBIAS{NDxBias: 1 as libc::c_int as SWORD,
                     NDyBias: 1 as libc::c_int as SWORD,
                     UDxBias: 1 as libc::c_int as SWORD,
                     UDyBias: 1 as libc::c_int as SWORD,
                     RDxBias: 1 as libc::c_int as SWORD,
                     RDyBias: 1 as libc::c_int as SWORD,
                     DDxBias: 1 as libc::c_int as SWORD,
                     DDyBias: 1 as libc::c_int as SWORD,
                     LDxBias: 1 as libc::c_int as SWORD,
                     LDyBias: 1 as libc::c_int as SWORD,};
        init
    };
#[no_mangle]
pub static mut ReticuleBias: SNAPBIAS =
    {
        let mut init =
            SNAPBIAS{NDxBias: 1 as libc::c_int as SWORD,
                     NDyBias: 1 as libc::c_int as SWORD,
                     UDxBias: 1 as libc::c_int as SWORD,
                     UDyBias: 1 as libc::c_int as SWORD,
                     RDxBias: 1 as libc::c_int as SWORD,
                     RDyBias: 1 as libc::c_int as SWORD,
                     DDxBias: 1 as libc::c_int as SWORD,
                     DDyBias: 1 as libc::c_int as SWORD,
                     LDxBias: 1 as libc::c_int as SWORD,
                     LDyBias: 1 as libc::c_int as SWORD,};
        init
    };
#[no_mangle]
pub static mut TabBias: SNAPBIAS =
    {
        let mut init =
            SNAPBIAS{NDxBias: 1 as libc::c_int as SWORD,
                     NDyBias: 1 as libc::c_int as SWORD,
                     UDxBias: 1 as libc::c_int as SWORD,
                     UDyBias: 8 as libc::c_int as SWORD,
                     RDxBias: 1 as libc::c_int as SWORD,
                     RDyBias: 1 as libc::c_int as SWORD,
                     DDxBias: 1 as libc::c_int as SWORD,
                     DDyBias: 8 as libc::c_int as SWORD,
                     LDxBias: 1 as libc::c_int as SWORD,
                     LDyBias: 1 as libc::c_int as SWORD,};
        init
    };
static mut MaxXDist: SDWORD = 64 as libc::c_int;
static mut MaxYDist: SDWORD = 64 as libc::c_int;
static mut EnabledFormID: UDWORD = 0 as libc::c_int as UDWORD;
//The widget screen
#[no_mangle]
pub unsafe extern "C" fn SetMaxDist(mut MaxX: SDWORD, mut MaxY: SDWORD) {
    MaxXDist = MaxX;
    MaxYDist = MaxY;
}
#[no_mangle]
pub unsafe extern "C" fn snapInitVars() {
    EnabledFormID = 0 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn AllocateSnapBuffer(mut SnapBuffer: *mut CURSORSNAP,
                                            mut MaxSnaps: UWORD) {
    (*SnapBuffer).SnapCoords =
        memMallocRelease((::std::mem::size_of::<CURSORSNAP>() as
                              libc::c_ulong).wrapping_mul(MaxSnaps as
                                                              libc::c_uint))
            as *mut SNAPCOORD;
    (*SnapBuffer).MaxSnaps = MaxSnaps;
    (*SnapBuffer).NumSnaps = 0 as libc::c_int as SWORD;
    (*SnapBuffer).CurrentSnap = 0 as libc::c_int as SWORD;
    (*SnapBuffer).NewCurrentFormID = 0 as libc::c_int as UDWORD;
    (*SnapBuffer).NewCurrentID = 0 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseSnapBuffer(mut SnapBuffer: *mut CURSORSNAP) {
    memFreeRelease((*SnapBuffer).SnapCoords as *mut libc::c_void);
    (*SnapBuffer).SnapCoords = 0 as *mut SNAPCOORD;
}
#[no_mangle]
pub unsafe extern "C" fn StartCursorSnap(mut SnapBuffer: *mut CURSORSNAP) {
    (*SnapBuffer).NumSnaps = 0 as libc::c_int as SWORD;
    //	SnapBuffer->CurrentSnap = 0;
}
#[no_mangle]
pub unsafe extern "C" fn InitCursorSnap(mut SnapBuffer: *mut CURSORSNAP,
                                        mut NumSnaps: UWORD) {
    (*SnapBuffer).NumSnaps = NumSnaps as SWORD;
    (*SnapBuffer).CurrentSnap = 0 as libc::c_int as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn AddCursorSnap(mut SnapBuffer: *mut CURSORSNAP,
                                       mut PosX: SWORD, mut PosY: SWORD,
                                       mut FormID: UDWORD, mut ID: UDWORD,
                                       mut Bias: *mut SNAPBIAS) {
    //	UWORD i;
    let mut Index: libc::c_int = -(1 as libc::c_int);
    if Index < 0 as libc::c_int {
        if ((*SnapBuffer).NumSnaps as libc::c_int) <
               (*SnapBuffer).MaxSnaps as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"AddCursorSnap: MAXCURSORSNAPS Exceeded\x00" as *const u8
                      as *const libc::c_char);
        };
        if ((*SnapBuffer).NumSnaps as libc::c_int) <
               (*SnapBuffer).MaxSnaps as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"csnap.c\x00" as *const u8 as *const libc::c_char,
                  129 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"AddCursorSnap\x00")).as_ptr(),
                  b"SnapBuffer->NumSnaps < SnapBuffer->MaxSnaps\x00" as
                      *const u8 as *const libc::c_char);
        };
        Index = (*SnapBuffer).NumSnaps as libc::c_int;
        (*SnapBuffer).NumSnaps += 1
    }
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).SnapX = PosX;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).SnapY = PosY;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).FormID = FormID;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).ID = ID;
    if Bias.is_null() {
        let ref mut fresh0 =
            (*(*SnapBuffer).SnapCoords.offset(Index as isize)).Bias;
        *fresh0 = &mut DefaultBias
    } else {
        let ref mut fresh1 =
            (*(*SnapBuffer).SnapCoords.offset(Index as isize)).Bias;
        *fresh1 = Bias
    }
    // If a new current snap was requested then see if this one meets its requirements.
    if (*SnapBuffer).NewCurrentFormID != 0 {
        if FormID == (*SnapBuffer).NewCurrentFormID {
            (*SnapBuffer).NewCurrentFormID = 0 as libc::c_int as UDWORD;
            (*SnapBuffer).CurrentSnap = Index as SWORD
        }
    } else if (*SnapBuffer).NewCurrentID != 0 {
        if ID == (*SnapBuffer).NewCurrentID {
            //			DBPRINTF(("Found New ID %d\n",SnapBuffer->NewCurrentID);
            (*SnapBuffer).NewCurrentID = 0 as libc::c_int as UDWORD;
            (*SnapBuffer).CurrentSnap = Index as SWORD;
            GotoSnap(SnapBuffer);
        }
    };
    //	DBPRINTF(("%d : %d,%d\n",Index,SnapBuffer->SnapCoords[Index].SnapX,SnapBuffer->SnapCoords[Index].SnapY);
}
#[no_mangle]
pub unsafe extern "C" fn RemoveCursorSnap(mut SnapBuffer: *mut CURSORSNAP,
                                          mut FormID: UDWORD) {
}
// Given a widget id get it's screen extents.
// NOTE that this function is slow and should be called infrequently,
// ideally only during widget initialisation.
//
#[no_mangle]
pub unsafe extern "C" fn widgGetScreenExtents(mut ID: UDWORD,
                                              mut sx: *mut libc::c_int,
                                              mut sy: *mut libc::c_int,
                                              mut sw: *mut libc::c_int,
                                              mut sh: *mut libc::c_int)
 -> BOOL {
    let mut psWidget: *mut _widget = widgGetFromID(psWScreen, ID);
    if !psWidget.is_null() {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        x = (*psWidget).x as libc::c_int;
        y = (*psWidget).y as libc::c_int;
        w = (*psWidget).width as libc::c_int;
        h = (*psWidget).height as libc::c_int;
        while (*psWidget).formID != 0 {
            let mut psParent: *mut _widget =
                widgGetFromID(psWScreen, (*psWidget).formID);
            if !psParent.is_null() {
                x += (*psParent).x as libc::c_int;
                y += (*psParent).y as libc::c_int
            }
            psWidget = psParent
        }
        *sx = x;
        *sy = y;
        *sw = w;
        *sh = h;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Given a widget form id, make the snap that matches it the current one next frame.
//
#[no_mangle]
pub unsafe extern "C" fn SetCurrentSnapFormID(mut SnapBuffer: *mut CURSORSNAP,
                                              mut FormID: UDWORD) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    (*SnapBuffer).NewCurrentFormID = FormID;
    if FormID != 0 {
        // Get the screen extents of the specified form and move the mouse there.
        if widgGetScreenExtents(FormID, &mut x, &mut y, &mut w, &mut h) ==
               1 as libc::c_int {
            //		DBPRINTF(("%d %d,%d %d\n",x,y,w,h);
            SetMousePos(0 as libc::c_int as UDWORD,
                        (x + w / 2 as libc::c_int) as UDWORD,
                        (y + h / 2 as libc::c_int) as UDWORD);
        }
    };
}
// Given a widget id, make the snap that matches it the current one next frame.
//
#[no_mangle]
pub unsafe extern "C" fn SetCurrentSnapID(mut SnapBuffer: *mut CURSORSNAP,
                                          mut ID: UDWORD) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    (*SnapBuffer).NewCurrentID = ID;
    if ID != 0 {
        // Get the screen extents of the specified widget and move the mouse there.
        if widgGetScreenExtents(ID, &mut x, &mut y, &mut w, &mut h) ==
               1 as libc::c_int {
            //		DBPRINTF(("%d %d,%d %d\n",x,y,w,h);
            SetMousePos(0 as libc::c_int as UDWORD,
                        (x + w / 2 as libc::c_int) as UDWORD,
                        (y + h / 2 as libc::c_int) as UDWORD);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn EnableAllCursorSnaps() {
    EnabledFormID = 0 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn DisableCursorSnapsExcept(mut FormID: UDWORD) {
    EnabledFormID = FormID;
}
#[no_mangle]
pub unsafe extern "C" fn SetCursorSnap(mut SnapBuffer: *mut CURSORSNAP,
                                       mut Index: UWORD, mut PosX: SWORD,
                                       mut PosY: SWORD, mut FormID: UDWORD,
                                       mut ID: UDWORD) {
    if (Index as libc::c_int) < (*SnapBuffer).NumSnaps as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"SetCursorSnap: Index out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (Index as libc::c_int) < (*SnapBuffer).NumSnaps as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"csnap.c\x00" as *const u8 as *const libc::c_char,
              266 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"SetCursorSnap\x00")).as_ptr(),
              b"Index < SnapBuffer->NumSnaps\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).SnapX = PosX;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).SnapY = PosY;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).FormID = FormID;
    (*(*SnapBuffer).SnapCoords.offset(Index as isize)).ID = ID;
}
#[no_mangle]
pub unsafe extern "C" fn GotoSnap(mut SnapBuffer: *mut CURSORSNAP) {
    if (*SnapBuffer).NumSnaps as libc::c_int > 0 as libc::c_int {
        SetMousePos(0 as libc::c_int as UDWORD,
                    (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap
                                                          as isize)).SnapX as
                        UDWORD,
                    (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap
                                                          as isize)).SnapY as
                        UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GotoNextSnap(mut SnapBuffer: *mut CURSORSNAP) {
    if (*SnapBuffer).NumSnaps as libc::c_int > 0 as libc::c_int {
        (*SnapBuffer).CurrentSnap += 1;
        if (*SnapBuffer).CurrentSnap as libc::c_int >=
               (*SnapBuffer).NumSnaps as libc::c_int {
            (*SnapBuffer).CurrentSnap = 0 as libc::c_int as SWORD
        }
    }
    SetMousePos(0 as libc::c_int as UDWORD,
                (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                      isize)).SnapX as UDWORD,
                (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                      isize)).SnapY as
                    UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn GotoPreviousSnap(mut SnapBuffer: *mut CURSORSNAP) {
    if (*SnapBuffer).NumSnaps as libc::c_int > 0 as libc::c_int {
        (*SnapBuffer).CurrentSnap -= 1;
        if ((*SnapBuffer).CurrentSnap as libc::c_int) < 0 as libc::c_int {
            (*SnapBuffer).CurrentSnap =
                ((*SnapBuffer).NumSnaps as libc::c_int - 1 as libc::c_int) as
                    SWORD
        }
    }
    SetMousePos(0 as libc::c_int as UDWORD,
                (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                      isize)).SnapX as UDWORD,
                (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                      isize)).SnapY as
                    UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn GotoDirectionalSnap(mut SnapBuffer: *mut CURSORSNAP,
                                             mut Direction: SNAPDIRECTION,
                                             mut CurrX: SWORD,
                                             mut CurrY: SWORD) {
    let mut i: UWORD = 0;
    let mut CurrentSnap: UWORD = (*SnapBuffer).CurrentSnap as UWORD;
    let mut ThisX: SWORD =
        (*(*SnapBuffer).SnapCoords.offset(CurrentSnap as isize)).SnapX;
    let mut ThisY: SWORD =
        (*(*SnapBuffer).SnapCoords.offset(CurrentSnap as isize)).SnapY;
    let mut NearestSnap: SWORD = -(1 as libc::c_int) as SWORD;
    let mut NearestDist: SWORD = 32767 as libc::c_int as SWORD;
    let mut dx: SWORD = 0;
    let mut dy: SWORD = 0;
    let mut Dist: SWORD = 0;
    if ((*SnapBuffer).CurrentSnap as libc::c_int) < 0 as libc::c_int {
        (*SnapBuffer).CurrentSnap =
            ((*SnapBuffer).NumSnaps as libc::c_int - 1 as libc::c_int) as
                SWORD
    }
    if (*SnapBuffer).CurrentSnap as libc::c_int >=
           (*SnapBuffer).NumSnaps as libc::c_int {
        (*SnapBuffer).CurrentSnap = 0 as libc::c_int as SWORD
    }
    //	DBPRINTF(("NumSnaps %d %d %d\n",SnapBuffer->NumSnaps,Direction,EnabledFormID);
    i = 0 as libc::c_int as UWORD; //i].Bias;
    while (i as libc::c_int) < (*SnapBuffer).NumSnaps as libc::c_int {
        let mut Bias: *mut SNAPBIAS =
            (*(*SnapBuffer).SnapCoords.offset(CurrentSnap as isize)).Bias;
        if (i as libc::c_int != CurrentSnap as libc::c_int ||
                Direction as libc::c_uint ==
                    SNAP_NEAREST as libc::c_int as libc::c_uint) &&
               ((*(*SnapBuffer).SnapCoords.offset(i as isize)).FormID ==
                    EnabledFormID ||
                    EnabledFormID == 0 as libc::c_int as libc::c_uint) {
            dx =
                ((*(*SnapBuffer).SnapCoords.offset(i as isize)).SnapX as
                     libc::c_int - ThisX as libc::c_int) as SWORD;
            dy =
                ((*(*SnapBuffer).SnapCoords.offset(i as isize)).SnapY as
                     libc::c_int - ThisY as libc::c_int) as SWORD;
            if Direction as libc::c_uint ==
                   SNAP_NEAREST as libc::c_int as libc::c_uint {
                dx =
                    ((*(*SnapBuffer).SnapCoords.offset(i as isize)).SnapX as
                         libc::c_int - CurrX as libc::c_int) as SWORD;
                dy =
                    ((*(*SnapBuffer).SnapCoords.offset(i as isize)).SnapY as
                         libc::c_int - CurrY as libc::c_int) as SWORD;
                Dist =
                    (abs(dx as libc::c_int * (*Bias).NDxBias as libc::c_int) +
                         abs(dy as libc::c_int *
                                 (*Bias).NDyBias as libc::c_int)) as SWORD;
                if (Dist as libc::c_int) < NearestDist as libc::c_int {
                    NearestSnap = i as SWORD;
                    NearestDist = Dist
                }
            } else if Direction as libc::c_uint ==
                          SNAP_UP as libc::c_int as libc::c_uint {
                if !(dy as libc::c_int >= 0 as libc::c_int) {
                    dx = abs(dx as libc::c_int) as SWORD;
                    dy = abs(dy as libc::c_int) as SWORD;
                    Dist =
                        (dx as libc::c_int * (*Bias).UDxBias as libc::c_int +
                             dy as libc::c_int *
                                 (*Bias).UDyBias as libc::c_int) as SWORD;
                    //DBPRINTF(("SNAP_UP %d %d : %d %d : %d\n",abs(dx),abs(dy),abs(dx*Bias->UDxBias),abs(dy*Bias->UDyBias),Dist);
                    if (Dist as libc::c_int) < NearestDist as libc::c_int &&
                           (dx as libc::c_int) < MaxXDist {
                        NearestSnap = i as SWORD;
                        NearestDist = Dist
                    }
                }
            } else if Direction as libc::c_uint ==
                          SNAP_RIGHT as libc::c_int as libc::c_uint {
                if !(dx as libc::c_int <= 0 as libc::c_int) {
                    dx = abs(dx as libc::c_int) as SWORD;
                    dy = abs(dy as libc::c_int) as SWORD;
                    Dist =
                        (dx as libc::c_int * (*Bias).RDxBias as libc::c_int +
                             dy as libc::c_int *
                                 (*Bias).RDyBias as libc::c_int) as SWORD;
                    //DBPRINTF(("SNAP_RIGHT %d %d : %d %d : %d\n",abs(dx),abs(dy),abs(dx*Bias->RDxBias),abs(dy*Bias->RDyBias),Dist);
                    if (Dist as libc::c_int) < NearestDist as libc::c_int &&
                           (dy as libc::c_int) < MaxYDist {
                        NearestSnap = i as SWORD;
                        NearestDist = Dist
                    }
                }
            } else if Direction as libc::c_uint ==
                          SNAP_DOWN as libc::c_int as libc::c_uint {
                if !(dy as libc::c_int <= 0 as libc::c_int) {
                    dx = abs(dx as libc::c_int) as SWORD;
                    dy = abs(dy as libc::c_int) as SWORD;
                    Dist =
                        (dx as libc::c_int * (*Bias).DDxBias as libc::c_int +
                             dy as libc::c_int *
                                 (*Bias).DDyBias as libc::c_int) as SWORD;
                    //DBPRINTF(("SNAP_DOWN %d %d : %d %d : %d\n",abs(dx),abs(dy),abs(dx*Bias->DDxBias),abs(dy*Bias->DDyBias),Dist));
                    if (Dist as libc::c_int) < NearestDist as libc::c_int &&
                           (dx as libc::c_int) < MaxXDist {
                        NearestSnap = i as SWORD;
                        NearestDist = Dist
                    }
                }
            } else if Direction as libc::c_uint ==
                          SNAP_LEFT as libc::c_int as libc::c_uint {
                if !(dx as libc::c_int >= 0 as libc::c_int) {
                    dx = abs(dx as libc::c_int) as SWORD;
                    dy = abs(dy as libc::c_int) as SWORD;
                    Dist =
                        (dx as libc::c_int * (*Bias).LDxBias as libc::c_int +
                             dy as libc::c_int *
                                 (*Bias).LDyBias as libc::c_int) as SWORD;
                    //DBPRINTF(("SNAP_LEFT %d %d : %d %d : %d\n",abs(dx),abs(dy),abs(dx*Bias->LDxBias),abs(dy*Bias->LDyBias),Dist);
                    if (Dist as libc::c_int) < NearestDist as libc::c_int &&
                           (dy as libc::c_int) < MaxYDist {
                        NearestSnap = i as SWORD;
                        NearestDist = Dist
                    }
                }
            }
        }
        //					DBPRINTF(("Skip SNAP_LEFT\n");
        i = i.wrapping_add(1)
    }
    if NearestSnap as libc::c_int >= 0 as libc::c_int {
        (*SnapBuffer).CurrentSnap = NearestSnap;
        SetMousePos(0 as libc::c_int as UDWORD,
                    (*(*SnapBuffer).SnapCoords.offset(NearestSnap as
                                                          isize)).SnapX as
                        UDWORD,
                    (*(*SnapBuffer).SnapCoords.offset(NearestSnap as
                                                          isize)).SnapY as
                        UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SnapToID(mut SnapBuffer: *mut CURSORSNAP,
                                  mut snp: UWORD) {
    (*SnapBuffer).CurrentSnap = snp as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn SnapCursorTo(mut x: UWORD, mut y: UWORD) {
    SetMousePos(0 as libc::c_int as UDWORD, x as UDWORD, y as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn SnapGetFormID(mut SnapBuffer: *mut CURSORSNAP)
 -> UDWORD {
    return (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                 isize)).FormID;
}
#[no_mangle]
pub unsafe extern "C" fn SnapGetID(mut SnapBuffer: *mut CURSORSNAP)
 -> UDWORD {
    return (*(*SnapBuffer).SnapCoords.offset((*SnapBuffer).CurrentSnap as
                                                 isize)).ID;
}
