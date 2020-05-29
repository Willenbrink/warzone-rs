use ::libc;
extern "C" {
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
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
    /* Set palette entries for the display buffer
 * first specifies the first palette entry. count the number of entries
 * The psPalette should have at least first + count entries in it.
 */
    #[no_mangle]
    fn screenSetPalette(first: UDWORD, count: UDWORD,
                        psPalette: *mut PALETTEENTRY);
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
pub type UDWORD = libc::c_uint;
pub type BOOL = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTEENTRY {
    pub peRed: UBYTE,
    pub peGreen: UBYTE,
    pub peBlue: UBYTE,
    pub peFlags: UBYTE,
}
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
// 3 from the end. (two brighter shades!)
#[no_mangle]
pub static mut psGamePal: *mut iColour = 0 as *const iColour as *mut iColour;
#[no_mangle]
pub static mut psWinPal: *mut PALETTEENTRY =
    0 as *const PALETTEENTRY as *mut PALETTEENTRY;
#[no_mangle]
pub static mut palShades: [uint8; 4096] = [0; 4096];
#[no_mangle]
pub static mut bPaletteInitialised: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut colours: [uint8; 16] = [0; 16];
//*************************************************************************
//*** add a new palette
//*
//* params	pal = pointer to palette to add
//*
//* returns slot number of added palette or -1 if error
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pal_AddNewPalette(mut pal: *mut iColour)
 -> libc::c_int {
    // PSX version dos'nt use palettes as such, SetRGBLookup sets up a global palette instead which is generally
// just used for colour index to RGB conversions.
    let mut i: libc::c_int = 0;
    //	int rg;
//	long	entry;
//	long	cardPal[256];
    let mut p: *mut iColour = 0 as *mut iColour;
    let mut w: *mut PALETTEENTRY = 0 as *mut PALETTEENTRY;
    bPaletteInitialised = 1 as libc::c_int;
    if psGamePal.is_null() {
        psGamePal =
            memMallocRelease((256 as libc::c_int as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<iColour>()
                                                                 as
                                                                 libc::c_ulong))
                as *mut iColour;
        if psGamePal.is_null() {
            debug(LOG_ERROR,
                  b"pal_AddNewPalette - Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    if psWinPal.is_null() {
        psWinPal =
            memMallocRelease((256 as libc::c_int as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<PALETTEENTRY>()
                                                                 as
                                                                 libc::c_ulong))
                as *mut PALETTEENTRY;
        if psGamePal.is_null() {
            debug(LOG_ERROR,
                  b"pal_AddNewPalette - Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    p = psGamePal;
    w = psWinPal;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        //set pie palette
        (*p.offset(i as isize)).r = (*pal.offset(i as isize)).r;
        (*p.offset(i as isize)).g = (*pal.offset(i as isize)).g;
        (*p.offset(i as isize)).b = (*pal.offset(i as isize)).b;
        //set copy of windows palette
        (*w.offset(i as isize)).peRed = (*pal.offset(i as isize)).r;
        (*w.offset(i as isize)).peGreen = (*pal.offset(i as isize)).g;
        (*w.offset(i as isize)).peBlue = (*pal.offset(i as isize)).b;
        (*w.offset(i as isize)).peFlags = 0 as libc::c_int as UBYTE;
        i += 1
    }
    //set windows palette
    screenSetPalette(0 as libc::c_int as UDWORD, 256 as libc::c_int as UDWORD,
                     psWinPal);
    pie_SetColourDefines();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pal_SelectPalette(mut n: libc::c_int) { }
//*************************************************************************
//***
//*
//******
#[no_mangle]
pub unsafe extern "C" fn pal_SetPalette() { }
//*************************************************************************
//*** calculate primary colours for current palette (store in COL_ ..
//*
//* on exit	_iVCOLS[0..15] contain colour values matched
//*			COL_.. below access _iVCOLS[0..15]
//******
#[no_mangle]
pub unsafe extern "C" fn pie_SetColourDefines() {
    colours[0 as libc::c_int as usize] =
        pal_GetNearestColour(1 as libc::c_int as uint8,
                             1 as libc::c_int as uint8,
                             1 as libc::c_int as uint8);
    colours[4 as libc::c_int as usize] =
        pal_GetNearestColour(128 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[2 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             128 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[1 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             128 as libc::c_int as uint8);
    colours[3 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             128 as libc::c_int as uint8,
                             128 as libc::c_int as uint8);
    colours[5 as libc::c_int as usize] =
        pal_GetNearestColour(128 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             128 as libc::c_int as uint8);
    colours[6 as libc::c_int as usize] =
        pal_GetNearestColour(128 as libc::c_int as uint8,
                             64 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[8 as libc::c_int as usize] =
        pal_GetNearestColour(32 as libc::c_int as uint8,
                             32 as libc::c_int as uint8,
                             32 as libc::c_int as uint8);
    colours[7 as libc::c_int as usize] =
        pal_GetNearestColour(128 as libc::c_int as uint8,
                             128 as libc::c_int as uint8,
                             128 as libc::c_int as uint8);
    colours[12 as libc::c_int as usize] =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[10 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[9 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    colours[11 as libc::c_int as usize] =
        pal_GetNearestColour(0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    colours[13 as libc::c_int as usize] =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
    colours[14 as libc::c_int as usize] =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             0 as libc::c_int as uint8);
    colours[15 as libc::c_int as usize] =
        pal_GetNearestColour(255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8,
                             255 as libc::c_int as uint8);
}
//*************************************************************************
//*** init palette (sets default palette and calc primary colours)
//*
//* on exit	psCurrentPalette = pointer to default palette (palette 0)
//******
#[no_mangle]
pub unsafe extern "C" fn pal_Init() { }
#[no_mangle]
pub unsafe extern "C" fn pal_ShutDown() {
    if bPaletteInitialised != 0 {
        bPaletteInitialised = 0 as libc::c_int;
        memFreeRelease(psGamePal as *mut libc::c_void);
        psGamePal = 0 as *mut iColour;
        memFreeRelease(psWinPal as *mut libc::c_void);
        psWinPal = 0 as *mut PALETTEENTRY
    };
}
#[no_mangle]
pub unsafe extern "C" fn pal_GetNearestColour(mut r: uint8, mut g: uint8,
                                              mut b: uint8) -> uint8 {
    let mut c: libc::c_int = 0;
    let mut distance_r: int32 = 0;
    let mut distance_g: int32 = 0;
    let mut distance_b: int32 = 0;
    let mut squared_distance: int32 = 0;
    let mut best_colour: int32 = 0 as libc::c_int;
    let mut best_squared_distance: int32 = 0;
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"pal_GetNearestColour, palette not initialised.\x00" as
                  *const u8 as *const libc::c_char);
    };
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"piepalette.c\x00" as *const u8 as *const libc::c_char,
              193 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"pal_GetNearestColour\x00")).as_ptr(),
              b"bPaletteInitialised\x00" as *const u8 as *const libc::c_char);
    };
    best_squared_distance = 0x10000 as libc::c_int;
    c = 0 as libc::c_int;
    while c < 256 as libc::c_int {
        distance_r =
            r as libc::c_int -
                (*psGamePal.offset(c as isize)).r as libc::c_int;
        distance_g =
            g as libc::c_int -
                (*psGamePal.offset(c as isize)).g as libc::c_int;
        distance_b =
            b as libc::c_int -
                (*psGamePal.offset(c as isize)).b as libc::c_int;
        squared_distance =
            distance_r * distance_r + distance_g * distance_g +
                distance_b * distance_b;
        if squared_distance < best_squared_distance {
            best_squared_distance = squared_distance;
            best_colour = c
        }
        c += 1
    }
    if best_colour == 0 as libc::c_int { best_colour = 1 as libc::c_int }
    return best_colour as uint8;
}
#[no_mangle]
pub unsafe extern "C" fn pal_BuildAdjustedShadeTable() {
    let mut redFraction: libc::c_float = 0.;
    let mut greenFraction: libc::c_float = 0.;
    let mut blueFraction: libc::c_float = 0.;
    let mut seekRed: libc::c_int = 0;
    let mut seekGreen: libc::c_int = 0;
    let mut seekBlue: libc::c_int = 0;
    let mut numColours: libc::c_int = 0;
    let mut numShades: libc::c_int = 0;
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"pal_BuildAdjustedShadeTable, palette not initialised.\x00" as
                  *const u8 as *const libc::c_char);
    };
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"piepalette.c\x00" as *const u8 as *const libc::c_char,
              231 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"pal_BuildAdjustedShadeTable\x00")).as_ptr(),
              b"bPaletteInitialised\x00" as *const u8 as *const libc::c_char);
    };
    numColours = 0 as libc::c_int;
    while numColours < 255 as libc::c_int {
        redFraction =
            (*psGamePal.offset(numColours as isize)).r as libc::c_float /
                16 as libc::c_int as libc::c_float;
        greenFraction =
            (*psGamePal.offset(numColours as isize)).g as libc::c_float /
                16 as libc::c_int as libc::c_float;
        blueFraction =
            (*psGamePal.offset(numColours as isize)).b as libc::c_float /
                16 as libc::c_int as libc::c_float;
        numShades = 6 as libc::c_int;
        while numShades < 16 as libc::c_int + 6 as libc::c_int {
            seekRed =
                (numShades as libc::c_float * redFraction) as libc::c_int;
            seekGreen =
                (numShades as libc::c_float * greenFraction) as libc::c_int;
            seekBlue =
                (numShades as libc::c_float * blueFraction) as libc::c_int;
            if seekRed > 255 as libc::c_int { seekRed = 255 as libc::c_int }
            if seekGreen > 255 as libc::c_int {
                seekGreen = 255 as libc::c_int
            }
            if seekBlue > 255 as libc::c_int { seekBlue = 255 as libc::c_int }
            palShades[(numColours * 16 as libc::c_int +
                           (numShades - 6 as libc::c_int)) as usize] =
                pal_GetNearestColour(seekRed as uint8, seekGreen as uint8,
                                     seekBlue as uint8);
            numShades += 1
        }
        numColours += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetGamePal() -> *mut iColour {
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"pie_GetGamePal, palette not initialised\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"piepalette.c\x00" as *const u8 as *const libc::c_char,
              257 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"pie_GetGamePal\x00")).as_ptr(),
              b"bPaletteInitialised\x00" as *const u8 as *const libc::c_char);
    };
    return psGamePal;
}
//*************************************************************************
//*************************************************************************
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_GetWinPal() -> *mut PALETTEENTRY {
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"pie_GetWinPal, palette not initialised\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bPaletteInitialised != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"piepalette.c\x00" as *const u8 as *const libc::c_char,
              263 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"pie_GetWinPal\x00")).as_ptr(),
              b"bPaletteInitialised\x00" as *const u8 as *const libc::c_char);
    };
    return psWinPal;
}
