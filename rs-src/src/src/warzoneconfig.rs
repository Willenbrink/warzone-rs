use ::libc;
extern "C" {
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
    /* special: sets all to on */
    /* special; on by default */
    /* if too verbose for anything but dedicated debugging... */
    /* _must_ be last! */
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
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* **************************************************************************/
/*
 * pieState.h
 *
 * render State controlr all pumpkin image library functions.
 *
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
    // FIXME Needed???
					// 	UBYTE				DDrawDriverName[256];
					// 	UBYTE				D3DDriverName[256];
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
    //Sets all states
    //renderer capability
    #[no_mangle]
    fn pie_SetAdditive(val: BOOL);
    #[no_mangle]
    fn pie_SetTranslucent(val: BOOL);
    //fog available
    //fog currently on
    #[no_mangle]
    fn pie_SetFogColour(colour: UDWORD);
    #[no_mangle]
    fn pie_SetFogCap(val: FOG_CAP);
    #[no_mangle]
    fn setRevealStatus(val: BOOL);
}
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
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
pub type SEQ_MODE = libc::c_uint;
pub const SEQ_SKIP: SEQ_MODE = 2;
pub const SEQ_SMALL: SEQ_MODE = 1;
pub const SEQ_FULL: SEQ_MODE = 0;
/* **************************************************************************/
/*
 * warzoneConfig.c
 *
 * warzone Global configuration functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Local Definitions
 */
/* **************************************************************************/
pub type WARZONE_GLOBALS = _warzoneGlobals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _warzoneGlobals {
    pub seqMode: SEQ_MODE,
    pub bFog: BOOL,
    pub bTranslucent: BOOL,
    pub bAdditive: BOOL,
    pub effectsLevel: SWORD,
    pub DDrawDriverName: [libc::c_char; 256],
    pub D3DDriverName: [libc::c_char; 256],
    pub allowSubtitles: BOOL,
    pub playAudioCDs: BOOL,
    pub Fullscreen: BOOL,
}
pub type FOG_CAP = libc::c_uint;
pub const FOG_CAP_UNDEFINED: FOG_CAP = 3;
pub const FOG_CAP_COLOURED: FOG_CAP = 2;
pub const FOG_CAP_GREY: FOG_CAP = 1;
pub const FOG_CAP_NO: FOG_CAP = 0;
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
static mut warGlobs: WARZONE_GLOBALS =
    WARZONE_GLOBALS{seqMode: SEQ_FULL,
                    bFog: 0,
                    bTranslucent: 0,
                    bAdditive: 0,
                    effectsLevel: 0,
                    DDrawDriverName: [0; 256],
                    D3DDriverName: [0; 256],
                    allowSubtitles: 0,
                    playAudioCDs: 0,
                    Fullscreen: 0,};
/* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
//STATIC use or write an access function if you need any of this
/* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetDefaultStates() 
 //Sets all states
 {
    //set those here and reset in clParse or loadConfig
    pie_SetFogCap(FOG_CAP_UNDEFINED);
    war_SetFog(0 as libc::c_int);
    war_SetTranslucent(0 as libc::c_int);
    war_SetAdditive(0 as libc::c_int);
    war_SetPlayAudioCDs(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn war_SetPlayAudioCDs(mut b: BOOL) {
    warGlobs.playAudioCDs = b;
}
#[no_mangle]
pub unsafe extern "C" fn war_GetPlayAudioCDs() -> BOOL {
    return warGlobs.playAudioCDs;
}
#[no_mangle]
pub unsafe extern "C" fn war_SetAllowSubtitles(mut b: BOOL) {
    warGlobs.allowSubtitles = b;
}
#[no_mangle]
pub unsafe extern "C" fn war_GetAllowSubtitles() -> BOOL {
    return warGlobs.allowSubtitles;
}
#[no_mangle]
pub unsafe extern "C" fn war_setFullscreen(mut b: BOOL) {
    warGlobs.Fullscreen = b;
}
#[no_mangle]
pub unsafe extern "C" fn war_getFullscreen() -> BOOL {
    return warGlobs.Fullscreen;
}
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetFog(mut val: BOOL) {
    if warGlobs.bFog != val { warGlobs.bFog = val }
    if warGlobs.bFog == 1 as libc::c_int {
        setRevealStatus(0 as libc::c_int);
    } else {
        setRevealStatus(1 as libc::c_int);
        pie_SetFogColour(0 as libc::c_int as UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn war_GetFog() -> BOOL { return warGlobs.bFog; }
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetTranslucent(mut val: BOOL) {
    pie_SetTranslucent(val);
    if warGlobs.bTranslucent != val { warGlobs.bTranslucent = val };
}
#[no_mangle]
pub unsafe extern "C" fn war_GetTranslucent() -> BOOL {
    return warGlobs.bTranslucent;
}
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetAdditive(mut val: BOOL) {
    pie_SetAdditive(val);
    if warGlobs.bAdditive != val { warGlobs.bAdditive = val };
}
#[no_mangle]
pub unsafe extern "C" fn war_GetAdditive() -> BOOL {
    return warGlobs.bAdditive;
}
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetSeqMode(mut mode: SEQ_MODE) {
    warGlobs.seqMode = mode;
}
#[no_mangle]
pub unsafe extern "C" fn war_GetSeqMode() -> SEQ_MODE {
    return warGlobs.seqMode;
}
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetDirectDrawDeviceName(mut pDDDeviceName:
                                                         *mut libc::c_char) {
    if strlen(pDDDeviceName) < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"DirectDraw device string exceeds max string length.\x00" as
                  *const u8 as *const libc::c_char);
    };
    if strlen(pDDDeviceName) < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"warzoneconfig.c\x00" as *const u8 as *const libc::c_char,
              168 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"war_SetDirectDrawDeviceName\x00")).as_ptr(),
              b"strlen(pDDDeviceName) < 255\x00" as *const u8 as
                  *const libc::c_char);
    };
    if strlen(pDDDeviceName) >= 255 as libc::c_int as libc::c_uint {
        *pDDDeviceName.offset(255 as libc::c_int as isize) =
            0 as libc::c_int as libc::c_char
    }
    strcpy(warGlobs.DDrawDriverName.as_mut_ptr(), pDDDeviceName);
}
#[no_mangle]
pub unsafe extern "C" fn war_GetDirectDrawDeviceName() -> *mut libc::c_char {
    return warGlobs.DDrawDriverName.as_mut_ptr();
}
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn war_SetDirect3DDeviceName(mut pD3DDeviceName:
                                                       *mut libc::c_char) {
    if strlen(pD3DDeviceName) < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Direct3D device string exceeds max string length.\x00" as
                  *const u8 as *const libc::c_char);
    };
    if strlen(pD3DDeviceName) < 255 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"warzoneconfig.c\x00" as *const u8 as *const libc::c_char,
              185 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"war_SetDirect3DDeviceName\x00")).as_ptr(),
              b"strlen(pD3DDeviceName) < 255\x00" as *const u8 as
                  *const libc::c_char);
    };
    if strlen(pD3DDeviceName) >= 255 as libc::c_int as libc::c_uint {
        *pD3DDeviceName.offset(255 as libc::c_int as isize) =
            0 as libc::c_int as libc::c_char
    }
    strcpy(warGlobs.D3DDriverName.as_mut_ptr(), pD3DDeviceName);
}
#[no_mangle]
pub unsafe extern "C" fn war_GetDirect3DDeviceName() -> *mut libc::c_char {
    return warGlobs.D3DDriverName.as_mut_ptr();
}
