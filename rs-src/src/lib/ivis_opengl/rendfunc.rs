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
    #[no_mangle]
    static mut palShades: [uint8; 4096];
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn pie_GetGamePal() -> *mut iColour;
    #[no_mangle]
    static mut iV_ppBitmap:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut iV_ppBitmapTrans:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int) -> ()>;
    #[no_mangle]
    static mut iV_pLine:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: uint32) -> ()>;
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
    fn pie_ImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD, x: libc::c_int,
                       y: libc::c_int);
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    #[no_mangle]
    static mut rendSurface: iSurface;
    // Valid values for paramaters for pie_SetFormattedTextFlags().
    // Skip leading spaces at the start of each line of text. Improves centre justification
// but may result in unwanted word breaks.
    // Skip trailing spaces at the end of each line of text, improves centre justification.
    // Inserts a space before the first word in the string, usefull when use FTEXT_LEFTJUSTIFYAPPEND
    #[no_mangle]
    fn pie_RenderBlueTintedBitmap(bmp: *mut iBitmap, x: libc::c_int,
                                  y: libc::c_int, w: libc::c_int,
                                  h: libc::c_int, ow: libc::c_int);
    #[no_mangle]
    fn pie_RenderDeepBlueTintedBitmap(bmp: *mut iBitmap, x: libc::c_int,
                                      y: libc::c_int, w: libc::c_int,
                                      h: libc::c_int, ow: libc::c_int);
    //*************************************************************************
    #[no_mangle]
    static mut iV_ppBitmapColourTrans:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int) -> ()>;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
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
//*************************************************************************
//
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
}
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
#[no_mangle]
pub static mut aTransTable: [UBYTE; 256] = [0; 256];
#[no_mangle]
pub static mut aTransTable2: [UBYTE; 256] = [0; 256];
// 2 trans tabels so we can have 2 transparancy colours without slowdown.
#[no_mangle]
pub static mut aTransTable3: [UBYTE; 256] = [0; 256];
// 3 trans tabels so we can have 3 transparancy colours without slowdown.
#[no_mangle]
pub static mut aTransTable4: [UBYTE; 256] = [0; 256];
// 4 trans tabels so we can have 4 transparancy colours without slowdown.
/* Set default transparency filter to green pass */
#[no_mangle]
pub static mut transFilter: UDWORD = 1 as libc::c_int as UDWORD;
//static int	g_mode = REND_UNDEFINED;
static mut MouseImageFile: *mut IMAGEFILE =
    0 as *const IMAGEFILE as *mut IMAGEFILE;
static mut MouseImageID: UWORD = 0;
// dummy prototypes for pointer build functions
#[no_mangle]
pub static mut iV_pBox:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: uint32) -> ()> =
    None;
#[no_mangle]
pub static mut iV_pBoxFill:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: uint32) -> ()> =
    None;
//*************************************************************************
//*************************************************************************
//*************************************************************************
//*************************************************************************
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
//*************************************************************************
//*** line plot 2D line - clipped
//*
//* params	x0,y0  = line point 1
//*	 		x1,y1  = line point 2
//*	 		colour = line colour value
//*****
#[no_mangle]
pub unsafe extern "C" fn line(mut x0: libc::c_int, mut y0: libc::c_int,
                              mut x1: libc::c_int, mut y1: libc::c_int,
                              mut colour: uint32) {
    let mut code1: libc::c_int = 0;
    let mut code2: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    code2 = 0 as libc::c_int;
    code1 = code2;
    if y0 > (*psRendSurface).clip.bottom {
        code1 = 1 as libc::c_int
    } else if y0 < (*psRendSurface).clip.top { code1 = 2 as libc::c_int }
    if x0 > (*psRendSurface).clip.right {
        code1 |= 4 as libc::c_int
    } else if x0 < (*psRendSurface).clip.left { code1 |= 8 as libc::c_int }
    if y1 > (*psRendSurface).clip.bottom {
        code2 = 1 as libc::c_int
    } else if y1 < (*psRendSurface).clip.top { code2 = 2 as libc::c_int }
    if x1 > (*psRendSurface).clip.right {
        code2 |= 4 as libc::c_int
    } else if x1 < (*psRendSurface).clip.left { code2 |= 8 as libc::c_int }
    // line totally outside screen: reject
    if code1 & code2 != 0 as libc::c_int { return }
    // line totally inside screen: accept
    if code1 | code2 == 0 as libc::c_int {
        iV_pLine.expect("non-null function pointer")(x0, y0, x1, y1, colour);
        return
    }
    // at least one point needs clipping
    code = if code1 != 0 as libc::c_int { code1 } else { code2 };
    if code & 1 as libc::c_int != 0 {
        x = x0 + (x1 - x0) * ((*psRendSurface).clip.bottom - y0) / (y1 - y0);
        y = (*psRendSurface).clip.bottom
    } else if code & 2 as libc::c_int != 0 {
        x = x0 + (x1 - x0) * ((*psRendSurface).clip.top - y0) / (y1 - y0);
        y = (*psRendSurface).clip.top
    } else if code & 4 as libc::c_int != 0 {
        y = y0 + (y1 - y0) * ((*psRendSurface).clip.right - x0) / (x1 - x0);
        x = (*psRendSurface).clip.right
    } else if code & 8 as libc::c_int != 0 {
        y = y0 + (y1 - y0) * ((*psRendSurface).clip.left - x0) / (x1 - x0);
        x = (*psRendSurface).clip.left
    }
    if code == code1 { x0 = x; y0 = y } else { x1 = x; y1 = y }
    code2 = 0 as libc::c_int;
    code1 = code2;
    if y0 > (*psRendSurface).clip.bottom {
        code1 = 1 as libc::c_int
    } else if y0 < (*psRendSurface).clip.top { code1 = 2 as libc::c_int }
    if x0 > (*psRendSurface).clip.right {
        code1 |= 4 as libc::c_int
    } else if x0 < (*psRendSurface).clip.left { code1 |= 8 as libc::c_int }
    if y1 > (*psRendSurface).clip.bottom {
        code2 = 1 as libc::c_int
    } else if y1 < (*psRendSurface).clip.top { code2 = 2 as libc::c_int }
    if x1 > (*psRendSurface).clip.right {
        code2 |= 4 as libc::c_int
    } else if x1 < (*psRendSurface).clip.left { code2 |= 8 as libc::c_int }
    if code1 & code2 != 0 as libc::c_int { return }
    if code1 | code2 == 0 as libc::c_int {
        iV_pLine.expect("non-null function pointer")(x0, y0, x1, y1, colour);
        return
    }
    code = if code1 != 0 as libc::c_int { code1 } else { code2 };
    if code & 1 as libc::c_int != 0 {
        x = x0 + (x1 - x0) * ((*psRendSurface).clip.bottom - y0) / (y1 - y0);
        y = (*psRendSurface).clip.bottom
    } else if code & 2 as libc::c_int != 0 {
        x = x0 + (x1 - x0) * ((*psRendSurface).clip.top - y0) / (y1 - y0);
        y = (*psRendSurface).clip.top
    } else if code & 4 as libc::c_int != 0 {
        y = y0 + (y1 - y0) * ((*psRendSurface).clip.right - x0) / (x1 - x0);
        x = (*psRendSurface).clip.right
    } else if code & 8 as libc::c_int != 0 {
        y = y0 + (y1 - y0) * ((*psRendSurface).clip.left - x0) / (x1 - x0);
        x = (*psRendSurface).clip.left
    }
    if code == code1 {
        iV_pLine.expect("non-null function pointer")(x, y, x1, y1, colour);
    } else {
        iV_pLine.expect("non-null function pointer")(x0, y0, x, y, colour);
    };
}
//*************************************************************************
#[export_name = "box"]
pub unsafe extern "C" fn box_0(mut x0: libc::c_int, mut y0: libc::c_int,
                               mut x1: libc::c_int, mut y1: libc::c_int,
                               mut colour: uint32) {
    if x0 > (*psRendSurface).clip.right || x1 < (*psRendSurface).clip.left ||
           y0 > (*psRendSurface).clip.bottom || y1 < (*psRendSurface).clip.top
       {
        return
    }
    if x0 < (*psRendSurface).clip.left { x0 = (*psRendSurface).clip.left }
    if x1 > (*psRendSurface).clip.right { x1 = (*psRendSurface).clip.right }
    if y0 < (*psRendSurface).clip.top { y0 = (*psRendSurface).clip.top }
    if y1 > (*psRendSurface).clip.bottom { y1 = (*psRendSurface).clip.bottom }
    iV_pBox.expect("non-null function pointer")(x0, y0, x1, y1, colour);
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn SetTransFilter(mut rgb: UDWORD,
                                        mut tablenumber: UDWORD) {
    transFilter = rgb;
    pie_BuildTransTable(tablenumber);
    /* Need to recalculate the transparency table */
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn TransBoxFill(mut x0: UDWORD, mut y0: UDWORD,
                                      mut x1: UDWORD, mut y1: UDWORD) {
    let mut screen: *mut UDWORD = 0 as *mut UDWORD;
    let mut fourPixels: UDWORD = 0;
    let mut output: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    /* Note x1 must be greater than x0 */
    width = x1.wrapping_sub(x0);
    /* Not worth it, so use pixel version */
    if width < 8 as libc::c_int as libc::c_uint {
        iVBlitPixelTransRect(x0, y0, x1, y1);
    } else {
        /* Get a handle on the current back buffer */
        i = y0;
        while i < y1 {
            screen =
                (*psRendSurface).buffer.offset((*psRendSurface).scantable[i as
                                                                              usize]
                                                   as
                                                   isize).offset(x0 as isize)
                    as *mut UDWORD;
            /* We need to go through width/4 times */
            j = x0;
            while j < x1 {
                fourPixels = *screen;
                output =
                    aTransTable[((fourPixels & 0xff000000 as libc::c_uint) >>
                                     24 as libc::c_int) as UBYTE as usize] as
                        UDWORD;
                output = output << 8 as libc::c_int;
                output =
                    output ^
                        aTransTable[((fourPixels &
                                          0xff0000 as libc::c_int as
                                              libc::c_uint) >>
                                         16 as libc::c_int) as UBYTE as usize]
                            as UDWORD;
                output = output << 8 as libc::c_int;
                output =
                    output ^
                        aTransTable[((fourPixels &
                                          0xff00 as libc::c_int as
                                              libc::c_uint) >>
                                         8 as libc::c_int) as UBYTE as usize]
                            as UDWORD;
                output = output << 8 as libc::c_int;
                output =
                    output ^
                        aTransTable[(fourPixels &
                                         0xff as libc::c_int as libc::c_uint)
                                        as UBYTE as usize] as UDWORD;
                let fresh0 = screen;
                screen = screen.offset(1);
                *fresh0 = output;
                j =
                    (j as
                         libc::c_uint).wrapping_add(4 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD
            }
            i = i.wrapping_add(1)
        }
    };
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DrawImageDef(mut Image: *mut IMAGEDEF,
                                      mut Bmp: *mut iBitmap,
                                      mut Modulus: UDWORD, mut x: libc::c_int,
                                      mut y: libc::c_int) {
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                    x +
                                                        (*Image).XOffset as
                                                            libc::c_int,
                                                    y +
                                                        (*Image).YOffset as
                                                            libc::c_int,
                                                    (*Image).Width as
                                                        libc::c_int,
                                                    (*Image).Height as
                                                        libc::c_int,
                                                    Modulus as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DrawSemiTransImageDef(mut Image: *mut IMAGEDEF,
                                               mut Bmp: *mut iBitmap,
                                               mut Modulus: UDWORD,
                                               mut x: libc::c_int,
                                               mut y: libc::c_int,
                                               mut TransRate: libc::c_int) {
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                         x +
                                                             (*Image).XOffset
                                                                 as
                                                                 libc::c_int,
                                                         y +
                                                             (*Image).YOffset
                                                                 as
                                                                 libc::c_int,
                                                         (*Image).Width as
                                                             libc::c_int,
                                                         (*Image).Height as
                                                             libc::c_int,
                                                         Modulus as
                                                             libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn DrawImage(mut ImageFile: *mut IMAGEFILE,
                                   mut ID: UWORD, mut x: libc::c_int,
                                   mut y: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut Bmp: *mut iBitmap = 0 as *mut iBitmap;
    let mut Modulus: UDWORD = 0;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    Modulus =
        (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).width;
    Bmp = (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).bmp;
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                    x +
                                                        (*Image).XOffset as
                                                            libc::c_int,
                                                    y +
                                                        (*Image).YOffset as
                                                            libc::c_int,
                                                    (*Image).Width as
                                                        libc::c_int,
                                                    (*Image).Height as
                                                        libc::c_int,
                                                    Modulus as libc::c_int);
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DrawImageRect(mut ImageFile: *mut IMAGEFILE,
                                       mut ID: UWORD, mut x: libc::c_int,
                                       mut y: libc::c_int,
                                       mut x0: libc::c_int,
                                       mut y0: libc::c_int,
                                       mut Width: libc::c_int,
                                       mut Height: libc::c_int) {
    let mut RepWidth: UDWORD = 1 as libc::c_int as UDWORD;
    let mut RepWidthRemain: UDWORD = 0 as libc::c_int as UDWORD;
    let mut RepHeight: UDWORD = 1 as libc::c_int as UDWORD;
    let mut RepHeightRemain: UDWORD = 0 as libc::c_int as UDWORD;
    let mut Tiled: BOOL = 0 as libc::c_int;
    let mut w: UDWORD = 0;
    let mut h: UDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut Bmp: *mut iBitmap = 0 as *mut iBitmap;
    let mut Modulus: UDWORD = 0;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    Modulus =
        (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).width;
    Bmp = (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).bmp;
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    if Width + x0 > (*Image).Width as libc::c_int {
        RepWidth = (Width / (*Image).Width as libc::c_int) as UDWORD;
        RepWidthRemain =
            (Width as
                 libc::c_uint).wrapping_sub(RepWidth.wrapping_mul((*Image).Width
                                                                      as
                                                                      libc::c_uint));
        Width = (*Image).Width as libc::c_int;
        Tiled = 1 as libc::c_int
    }
    if Height + y0 > (*Image).Height as libc::c_int {
        RepHeight = (Height / (*Image).Height as libc::c_int) as UDWORD;
        RepHeightRemain =
            (Height as
                 libc::c_uint).wrapping_sub(RepHeight.wrapping_mul((*Image).Height
                                                                       as
                                                                       libc::c_uint));
        Height = (*Image).Height as libc::c_int;
        Tiled = 1 as libc::c_int
    }
    if Tiled != 0 {
        ty = y as UDWORD;
        h = 0 as libc::c_int as UDWORD;
        while h < RepHeight {
            tx = x as UDWORD;
            w = 0 as libc::c_int as UDWORD;
            while w < RepWidth {
                iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                                tx.wrapping_add((*Image).XOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                ty.wrapping_add((*Image).YOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                Width, Height,
                                                                Modulus as
                                                                    libc::c_int);
                tx =
                    (tx as libc::c_uint).wrapping_add(Width as libc::c_uint)
                        as UDWORD as UDWORD;
                w = w.wrapping_add(1)
            }
            if RepWidthRemain != 0 {
                iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                                tx.wrapping_add((*Image).XOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                ty.wrapping_add((*Image).YOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                RepWidthRemain
                                                                    as
                                                                    libc::c_int,
                                                                Height,
                                                                Modulus as
                                                                    libc::c_int);
            }
            ty =
                (ty as libc::c_uint).wrapping_add(Height as libc::c_uint) as
                    UDWORD as UDWORD;
            h = h.wrapping_add(1)
        }
        if RepHeightRemain != 0 {
            tx = x as UDWORD;
            w = 0 as libc::c_int as UDWORD;
            while w < RepWidth {
                iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                                tx.wrapping_add((*Image).XOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                ty.wrapping_add((*Image).YOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                Width,
                                                                RepHeightRemain
                                                                    as
                                                                    libc::c_int,
                                                                Modulus as
                                                                    libc::c_int);
                tx =
                    (tx as libc::c_uint).wrapping_add(Width as libc::c_uint)
                        as UDWORD as UDWORD;
                w = w.wrapping_add(1)
            }
            if RepWidthRemain != 0 {
                iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                                tx.wrapping_add((*Image).XOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                ty.wrapping_add((*Image).YOffset
                                                                                    as
                                                                                    libc::c_uint)
                                                                    as
                                                                    libc::c_int,
                                                                RepWidthRemain
                                                                    as
                                                                    libc::c_int,
                                                                RepHeightRemain
                                                                    as
                                                                    libc::c_int,
                                                                Modulus as
                                                                    libc::c_int);
            }
        }
    } else {
        iV_ppBitmap.expect("non-null function pointer")(Bmp,
                                                        x +
                                                            (*Image).XOffset
                                                                as
                                                                libc::c_int,
                                                        y +
                                                            (*Image).YOffset
                                                                as
                                                                libc::c_int,
                                                        Width, Height,
                                                        Modulus as
                                                            libc::c_int);
    };
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DrawTransImage(mut ImageFile: *mut IMAGEFILE,
                                        mut ID: UWORD, mut x: libc::c_int,
                                        mut y: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut Bmp: *mut iBitmap = 0 as *mut iBitmap;
    let mut Modulus: UDWORD = 0;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    Modulus =
        (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).width;
    Bmp = (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).bmp;
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                         x +
                                                             (*Image).XOffset
                                                                 as
                                                                 libc::c_int,
                                                         y +
                                                             (*Image).YOffset
                                                                 as
                                                                 libc::c_int,
                                                         (*Image).Width as
                                                             libc::c_int,
                                                         (*Image).Height as
                                                             libc::c_int,
                                                         Modulus as
                                                             libc::c_int);
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DrawTransImageRect(mut ImageFile: *mut IMAGEFILE,
                                            mut ID: UWORD, mut x: libc::c_int,
                                            mut y: libc::c_int,
                                            mut x0: libc::c_int,
                                            mut y0: libc::c_int,
                                            mut Width: libc::c_int,
                                            mut Height: libc::c_int) {
    let mut RepWidth: UDWORD = 1 as libc::c_int as UDWORD;
    let mut RepWidthRemain: UDWORD = 0 as libc::c_int as UDWORD;
    let mut RepHeight: UDWORD = 1 as libc::c_int as UDWORD;
    let mut RepHeightRemain: UDWORD = 0 as libc::c_int as UDWORD;
    let mut Tiled: BOOL = 0 as libc::c_int;
    let mut w: UDWORD = 0;
    let mut h: UDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut Bmp: *mut iBitmap = 0 as *mut iBitmap;
    let mut Modulus: UDWORD = 0;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    Modulus =
        (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).width;
    Bmp = (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).bmp;
    Bmp =
        Bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(Modulus))
                       as isize);
    if Width + x0 > (*Image).Width as libc::c_int {
        RepWidth = (Width / (*Image).Width as libc::c_int) as UDWORD;
        RepWidthRemain =
            (Width as
                 libc::c_uint).wrapping_sub(RepWidth.wrapping_mul((*Image).Width
                                                                      as
                                                                      libc::c_uint));
        Width = (*Image).Width as libc::c_int;
        Tiled = 1 as libc::c_int
    }
    if Height + y0 > (*Image).Height as libc::c_int {
        RepHeight = (Height / (*Image).Height as libc::c_int) as UDWORD;
        RepHeightRemain =
            (Height as
                 libc::c_uint).wrapping_sub(RepHeight.wrapping_mul((*Image).Height
                                                                       as
                                                                       libc::c_uint));
        Height = (*Image).Height as libc::c_int;
        Tiled = 1 as libc::c_int
    }
    if Tiled != 0 {
        ty = y as UDWORD;
        h = 0 as libc::c_int as UDWORD;
        while h < RepHeight {
            tx = x as UDWORD;
            w = 0 as libc::c_int as UDWORD;
            while w < RepWidth {
                iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                                     tx.wrapping_add((*Image).XOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     ty.wrapping_add((*Image).YOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     Width,
                                                                     Height,
                                                                     Modulus
                                                                         as
                                                                         libc::c_int);
                tx =
                    (tx as libc::c_uint).wrapping_add(Width as libc::c_uint)
                        as UDWORD as UDWORD;
                w = w.wrapping_add(1)
            }
            if RepWidthRemain != 0 {
                iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                                     tx.wrapping_add((*Image).XOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     ty.wrapping_add((*Image).YOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     RepWidthRemain
                                                                         as
                                                                         libc::c_int,
                                                                     Height,
                                                                     Modulus
                                                                         as
                                                                         libc::c_int);
            }
            ty =
                (ty as libc::c_uint).wrapping_add(Height as libc::c_uint) as
                    UDWORD as UDWORD;
            h = h.wrapping_add(1)
        }
        if RepHeightRemain != 0 {
            tx = x as UDWORD;
            w = 0 as libc::c_int as UDWORD;
            while w < RepWidth {
                iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                                     tx.wrapping_add((*Image).XOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     ty.wrapping_add((*Image).YOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     Width,
                                                                     RepHeightRemain
                                                                         as
                                                                         libc::c_int,
                                                                     Modulus
                                                                         as
                                                                         libc::c_int);
                tx =
                    (tx as libc::c_uint).wrapping_add(Width as libc::c_uint)
                        as UDWORD as UDWORD;
                w = w.wrapping_add(1)
            }
            if RepWidthRemain != 0 {
                iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                                     tx.wrapping_add((*Image).XOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     ty.wrapping_add((*Image).YOffset
                                                                                         as
                                                                                         libc::c_uint)
                                                                         as
                                                                         libc::c_int,
                                                                     RepWidthRemain
                                                                         as
                                                                         libc::c_int,
                                                                     RepHeightRemain
                                                                         as
                                                                         libc::c_int,
                                                                     Modulus
                                                                         as
                                                                         libc::c_int);
            }
        }
    } else {
        iV_ppBitmapTrans.expect("non-null function pointer")(Bmp,
                                                             x +
                                                                 (*Image).XOffset
                                                                     as
                                                                     libc::c_int,
                                                             y +
                                                                 (*Image).YOffset
                                                                     as
                                                                     libc::c_int,
                                                             Width, Height,
                                                             Modulus as
                                                                 libc::c_int);
    };
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DrawTransColourImage(mut ImageFile: *mut IMAGEFILE,
                                              mut ID: UWORD,
                                              mut x: libc::c_int,
                                              mut y: libc::c_int,
                                              mut ColourIndex: SWORD) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut bmp: *mut iBitmap = 0 as *mut iBitmap;
    let mut modulus: UDWORD = 0;
    let mut width: SDWORD = 0;
    let mut height: SDWORD = 0;
    let mut delta: SDWORD = 0;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    width = (*Image).Width as SDWORD;
    height = (*Image).Height as SDWORD;
    modulus =
        (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).width;
    bmp = (*(*ImageFile).TexturePages.offset((*Image).TPageID as isize)).bmp;
    bmp =
        bmp.offset(((*Image).Tu as
                        UDWORD).wrapping_add(((*Image).Tv as
                                                  UDWORD).wrapping_mul(modulus))
                       as isize);
    x += (*Image).XOffset as libc::c_int;
    y += (*Image).YOffset as libc::c_int;
    //clip
		//clip
    if x < (*psRendSurface).clip.left {
        //off left hand edge
        delta = (*psRendSurface).clip.left - x;
        if delta < width {
            bmp = bmp.offset(delta as isize);
            x += delta;
            width -= delta
        } else { return }
    }
    if x + width > (*psRendSurface).clip.right + 1 as libc::c_int {
        delta = x + width - ((*psRendSurface).clip.right + 1 as libc::c_int);
        if delta < width { width -= delta } else { return }
    }
    if y < (*psRendSurface).clip.top {
        delta = (*psRendSurface).clip.top - y;
        if delta < height {
            bmp =
                bmp.offset((delta as libc::c_uint).wrapping_mul(modulus) as
                               isize);
            y += delta;
            height -= delta
        } else { return }
    }
    if y + height > (*psRendSurface).clip.bottom + 1 as libc::c_int {
        delta =
            y + height - ((*psRendSurface).clip.bottom + 1 as libc::c_int);
        if delta < height { height -= delta } else { return }
    }
    if ColourIndex as libc::c_int == -(1 as libc::c_int) {
        iV_ppBitmapColourTrans.expect("non-null function pointer")(bmp, x, y,
                                                                   width,
                                                                   height,
                                                                   modulus as
                                                                       libc::c_int,
                                                                   255 as
                                                                       libc::c_int);
    } else if ColourIndex as libc::c_int == -(2 as libc::c_int) {
        pie_RenderBlueTintedBitmap(bmp, x, y, width, height,
                                   modulus as libc::c_int);
    } else if ColourIndex as libc::c_int == -(3 as libc::c_int) {
        pie_RenderDeepBlueTintedBitmap(bmp, x, y, width, height,
                                       modulus as libc::c_int);
    } else {
        iV_ppBitmapColourTrans.expect("non-null function pointer")(bmp, x, y,
                                                                   width,
                                                                   height,
                                                                   modulus as
                                                                       libc::c_int,
                                                                   ColourIndex
                                                                       as
                                                                       libc::c_int);
    };
}
//mapdisplay
//extern void (*iV_DrawImageDef)(IMAGEDEF *Image,iBitmap *Bmp,UDWORD Modulus,int x,int y);
//design
//extern void (*iV_UploadDisplayBuffer)(UBYTE *DisplayBuffer);
//extern void (*iV_ScaleBitmapRGB)(UBYTE *DisplayBuffer,int Width,int Height,int ScaleR,int ScaleG,int ScaleB);
/* Blit a transparent rectangle to the back buffer */
/* Optimised DWORD read/write to memory */
/* Possible filter colours for the transparency rectangle blit */
//extern void (*iV_BeginTextRender)(SWORD ColourIndex);
//extern void (*iV_TextRender)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//extern void (*iV_TextRender270)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//extern void (*iV_EndTextRender)(void);
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_SetMousePointer(mut ImageFile: *mut IMAGEFILE,
                                            mut ImageID: UWORD) {
    if (ImageID as libc::c_int) < (*ImageFile).Header.NumImages as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"iV_SetMousePointer : Invalid image id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (ImageID as libc::c_int) < (*ImageFile).Header.NumImages as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"rendfunc.c\x00" as *const u8 as *const libc::c_char,
              548 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"iV_SetMousePointer\x00")).as_ptr(),
              b"ImageID < ImageFile->Header.NumImages\x00" as *const u8 as
                  *const libc::c_char);
    };
    MouseImageFile = ImageFile;
    MouseImageID = ImageID;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetMouseFrame() -> UDWORD {
    return MouseImageID as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn iV_DrawMousePointer(mut x: libc::c_int,
                                             mut y: libc::c_int) {
    pie_ImageFileID(MouseImageFile, MouseImageID, x, y);
}
//*************************************************************************
// Software version does nothing.
#[no_mangle]
pub unsafe extern "C" fn DownLoadRadar(mut buffer: *mut libc::c_uchar) { }
//*************************************************************************
// Upload the current display back buffer into system memory.
//
#[no_mangle]
pub unsafe extern "C" fn UploadDisplayBuffer(mut DisplayBuffer:
                                                 *mut libc::c_char) {
    let mut Source: *mut UDWORD = rendSurface.buffer as *mut UDWORD;
    let mut Dest: *mut UDWORD = DisplayBuffer as *mut UDWORD;
    let mut Size: UDWORD = (rendSurface.size / 4 as libc::c_int) as UDWORD;
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < Size {
        *Dest = *Source;
        Source = Source.offset(1);
        Dest = Dest.offset(1);
        i = i.wrapping_add(1)
    };
}
// Download buffer in system memory to the display back buffer.
//
/*
void DownloadDisplayBuffer(char *DisplayBuffer)
{
#ifndef PIEPSX		// was #ifndef PSX
	UDWORD *Source = (UDWORD*)DisplayBuffer;
	UDWORD *Dest = (UDWORD*) rendSurface.buffer;
	UDWORD Size = rendSurface.size / 4;
	UDWORD i;

	for(i=0; i<Size; i++) {
		*Dest = *Source;
		Source++;
		Dest++;
	}
#endif
}
 */
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn DownloadDisplayBuffer(mut DisplayBuffer:
                                                   *mut libc::c_char) {
    let mut Source: *mut UDWORD = DisplayBuffer as *mut UDWORD;
    let mut Dest: *mut UDWORD = rendSurface.buffer as *mut UDWORD;
    let mut depth: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut modulo: UDWORD = 0;
    let mut drop_0: UDWORD = 0;
    let mut srcWidth: UDWORD = 0;
    let mut srcDepth: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    depth = pie_GetVideoBufferHeight();
    width = pie_GetVideoBufferWidth();
    //always full screen
    modulo = 0 as libc::c_int as UDWORD; //width - BACKDROP_WIDTH;
    drop_0 = 0 as libc::c_int as UDWORD; //(depth - BACKDROP_HEIGHT)/2;
    Dest =
        Dest.offset(drop_0.wrapping_mul(width).wrapping_add(modulo).wrapping_div(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                        as isize); // dealing with dwords
    //always full screen
    srcDepth = depth; //BACKDROP_HEIGHT;
    srcWidth = width; //BACKDROP_WIDTH;
    i = 0 as libc::c_int as UDWORD;
    while i < srcDepth {
        j = 0 as libc::c_int as UDWORD;
        while j < srcWidth.wrapping_div(4 as libc::c_int as libc::c_uint) {
            *Dest = *Source;
            Source = Source.offset(1);
            Dest = Dest.offset(1);
            j = j.wrapping_add(1)
        }
        Dest =
            Dest.offset(modulo.wrapping_div(4 as libc::c_int as libc::c_uint)
                            as isize);
        i = i.wrapping_add(1)
        // dest is in dwords ... !
    };
}
/* **************************************************************************/
/*
 * rendfunc.h
 *
 * render functions for base render library.
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
//*************************************************************************
// functions accessed dirtectly from rendmode
//*************************************************************************
//*************************************************************************
// Scale a bitmaps colours.
//
#[no_mangle]
pub unsafe extern "C" fn ScaleBitmapRGB(mut DisplayBuffer: *mut libc::c_char,
                                        mut Width: libc::c_int,
                                        mut Height: libc::c_int,
                                        mut ScaleR: libc::c_int,
                                        mut ScaleG: libc::c_int,
                                        mut ScaleB: libc::c_int) {
    let mut Ptr: *mut libc::c_char =
        DisplayBuffer; //iV_PALETTE_SHADE_LEVEL-4];
    let mut Size: UDWORD = (Width * Height) as UDWORD;
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < Size {
        *Ptr =
            palShades[(*Ptr as libc::c_int * 16 as libc::c_int +
                           4 as libc::c_int) as usize] as libc::c_char;
        Ptr = Ptr.offset(1);
        i = i.wrapping_add(1)
    };
}
/* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
//*************************************************************************
//
// local functions
//
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iVBlitPixelTransRect(mut x0: UDWORD, mut y0: UDWORD,
                                              mut x1: UDWORD,
                                              mut y1: UDWORD) {
    let mut screen: *mut UBYTE = 0 as *mut UBYTE;
    let mut present: UBYTE = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    /* Do a horizontal strip at a time. We're going x inside y to take advantage of the scan table */
    i = y0;
    while i < y1 {
        /* Get this outside loop cos it's invariant to each scan line */
        screen =
            ((*psRendSurface).buffer as
                 *mut UBYTE).offset((*psRendSurface).scantable[i as usize] as
                                        isize).offset(x0 as isize);
        j = x0;
        while j < x1 {
            /* What colour is there at the moment? */
            present = *screen;
            /* Write in the new improved greener version */
            let fresh1 = screen;
            screen = screen.offset(1);
            *fresh1 = aTransTable[present as usize];
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
/* Build a transparency look up table for the interface */
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_BuildTransTable(mut tableNo: UDWORD) {
    let mut i: UDWORD = 0;
    let mut red: UBYTE = 0 as libc::c_int as UBYTE;
    let mut green: UBYTE = 0 as libc::c_int as UBYTE;
    let mut blue: UBYTE = 0 as libc::c_int as UBYTE;
    let mut psPalette: *mut iColour = pie_GetGamePal();
    // Step through all the palette entries for the currently selected iVPALETTE
    i = 0 as libc::c_int as UDWORD;
    while i < 256 as libc::c_int as libc::c_uint {
        match transFilter {
            0 => {
                red =
                    ((*psPalette.offset(i as isize)).r as libc::c_int *
                         5 as libc::c_int / 8 as libc::c_int) as UBYTE;
                blue =
                    ((*psPalette.offset(i as isize)).b as libc::c_int *
                         7 as libc::c_int / 8 as libc::c_int) as UBYTE;
                green =
                    ((*psPalette.offset(i as isize)).g as libc::c_int *
                         5 as libc::c_int / 8 as libc::c_int) as UBYTE
            }
            4 => {
                red =
                    ((*psPalette.offset(i as isize)).r as libc::c_int *
                         3 as libc::c_int / 8 as libc::c_int) as UBYTE;
                blue =
                    ((*psPalette.offset(i as isize)).b as libc::c_int *
                         5 as libc::c_int / 8 as libc::c_int) as UBYTE;
                green =
                    ((*psPalette.offset(i as isize)).g as libc::c_int *
                         3 as libc::c_int / 8 as libc::c_int) as UBYTE
            }
            1 => {
                red =
                    ((*psPalette.offset(i as isize)).r as libc::c_int /
                         2 as libc::c_int) as UBYTE;
                blue =
                    ((*psPalette.offset(i as isize)).b as libc::c_int /
                         2 as libc::c_int) as UBYTE;
                green =
                    ((*psPalette.offset(i as isize)).g as libc::c_int /
                         2 as libc::c_int) as UBYTE
            }
            2 => {
                red =
                    ((*psPalette.offset(i as isize)).r as libc::c_int /
                         2 as libc::c_int) as UBYTE;
                blue = (*psPalette.offset(i as isize)).b;
                green =
                    ((*psPalette.offset(i as isize)).g as libc::c_int /
                         2 as libc::c_int) as UBYTE
            }
            3 => {
                if ((*psPalette.offset(i as isize)).r as
                        UDWORD).wrapping_add(50 as libc::c_int as
                                                 libc::c_uint) >
                       255 as libc::c_int as libc::c_uint {
                    red = 255 as libc::c_int as UBYTE
                } else {
                    red =
                        ((*psPalette.offset(i as isize)).r as libc::c_int +
                             50 as libc::c_int) as UBYTE
                }
                if ((*psPalette.offset(i as isize)).b as
                        UDWORD).wrapping_add(50 as libc::c_int as
                                                 libc::c_uint) >
                       255 as libc::c_int as libc::c_uint {
                    blue = 255 as libc::c_int as UBYTE
                } else {
                    blue =
                        ((*psPalette.offset(i as isize)).b as libc::c_int +
                             50 as libc::c_int) as UBYTE
                }
                if ((*psPalette.offset(i as isize)).g as
                        UDWORD).wrapping_add(50 as libc::c_int as
                                                 libc::c_uint) >
                       255 as libc::c_int as libc::c_uint {
                    green = 255 as libc::c_int as UBYTE
                } else {
                    green =
                        ((*psPalette.offset(i as isize)).g as libc::c_int +
                             50 as libc::c_int) as UBYTE
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Invalid transparency filter selection\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"rendfunc.c\x00" as *const u8 as
                              *const libc::c_char, 767 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"pie_BuildTransTable\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        if tableNo == 0 as libc::c_int as libc::c_uint {
            aTransTable[i as usize] = pal_GetNearestColour(red, green, blue)
        } else if tableNo == 1 as libc::c_int as libc::c_uint {
            aTransTable2[i as usize] = pal_GetNearestColour(red, green, blue)
        } else if tableNo == 2 as libc::c_int as libc::c_uint {
            aTransTable3[i as usize] = pal_GetNearestColour(red, green, blue)
        } else {
            aTransTable4[i as usize] = pal_GetNearestColour(red, green, blue)
        }
        i = i.wrapping_add(1)
    };
}
