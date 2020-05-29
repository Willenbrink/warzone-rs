use ::libc;
extern "C" {
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    /* Append SRC onto DEST.  */
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn pie_DrawImage270(image: *mut PIEIMAGE, dest: *mut PIERECT,
                        style: *mut PIESTYLE);
    #[no_mangle]
    fn pie_SetColour(val: UDWORD);
    #[no_mangle]
    fn pie_SetRendMode(rendMode: REND_MODE);
    #[no_mangle]
    fn pie_SetColourKeyedBlack(keyingOn: BOOL);
    #[no_mangle]
    fn pie_SetBilinear(bilinearOn: BOOL);
    #[no_mangle]
    fn pie_GetGamePal() -> *mut iColour;
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    #[no_mangle]
    fn pie_DrawImageFileID(ImageFile: *mut IMAGEFILE, ID: UWORD,
                           x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageYOffset(ImageFile: *mut IMAGEFILE, ID: UWORD) -> SWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    //from rendfunc
    #[no_mangle]
    static mut aTransTable3: [UBYTE; 256];
    //from rendfunc
    #[no_mangle]
    static mut aTransTable4: [UBYTE; 256];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIELIGHTBYTES {
    pub b: UBYTE,
    pub g: UBYTE,
    pub r: UBYTE,
    pub a: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PIELIGHT {
    pub byte: PIELIGHTBYTES,
    pub argb: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIERECT {
    pub x: SWORD,
    pub y: SWORD,
    pub w: SWORD,
    pub h: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEIMAGE {
    pub texPage: SDWORD,
    pub tu: SWORD,
    pub tv: SWORD,
    pub tw: SWORD,
    pub th: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIESTYLE {
    pub pieFlag: UDWORD,
    pub colour: PIELIGHT,
    pub specular: PIELIGHT,
    pub light: UBYTE,
    pub trans: UBYTE,
    pub scale: UBYTE,
    pub height: UBYTE,
}
pub type REND_MODE = libc::c_uint;
pub const REND_FILTER_ITERATED: REND_MODE = 9;
pub const REND_FILTER_FLAT: REND_MODE = 8;
pub const REND_ALPHA_ITERATED: REND_MODE = 7;
pub const REND_ALPHA_FLAT: REND_MODE = 6;
pub const REND_FLAT: REND_MODE = 5;
pub const REND_ALPHA_TEXT: REND_MODE = 4;
pub const REND_TEXT: REND_MODE = 3;
pub const REND_ADDITIVE_TEX: REND_MODE = 2;
pub const REND_ALPHA_TEX: REND_MODE = 1;
pub const REND_GOURAUD_TEX: REND_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IVIS_FONT {
    pub FontFile: *mut IMAGEFILE,
    pub FontAbove: libc::c_int,
    pub FontBelow: libc::c_int,
    pub FontLineSize: libc::c_int,
    pub FontSpaceSize: libc::c_int,
    pub FontColourIndex: SWORD,
    pub AsciiTable: *mut UWORD,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FTEXT_LEFTJUSTIFYAPPEND: C2RustUnnamed = 3;
pub const FTEXT_RIGHTJUSTIFY: C2RustUnnamed = 2;
pub const FTEXT_CENTRE: C2RustUnnamed = 1;
pub const FTEXT_LEFTJUSTIFY: C2RustUnnamed = 0;
pub const EXTENTS_END: C2RustUnnamed_0 = 2;
pub const EXTENTS_NONE: C2RustUnnamed_0 = 0;
pub const EXTENTS_START: C2RustUnnamed_0 = 1;
pub type RENDERTEXT_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut STRING, _: UDWORD, _: UDWORD) -> ()>;
// --------------------------------------------------------------------------
pub type C2RustUnnamed_0 = libc::c_uint;
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
static mut TextColourIndex: SWORD = 0;
static mut NumFonts: libc::c_int = 0;
static mut ActiveFontID: libc::c_int = 0;
static mut iVFonts: [IVIS_FONT; 8] =
    [IVIS_FONT{FontFile: 0 as *const IMAGEFILE as *mut IMAGEFILE,
               FontAbove: 0,
               FontBelow: 0,
               FontLineSize: 0,
               FontSpaceSize: 0,
               FontColourIndex: 0,
               AsciiTable: 0 as *const UWORD as *mut UWORD,}; 8];
//prototype
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn iV_ClearFonts() {
    NumFonts = 0 as libc::c_int;
    ActiveFontID = -(1 as libc::c_int);
}
// Create a font using an ascii lookup table.
//
// IMAGEFILE *ImageFile		Image file containing the font graphics.
// UWORD *AsciiTable		Array of 256 Ascii to ImageID lookups.
// int SpaceSize			Pixel size of a space.
// BOOL bInGame				Specifies that the font is used in game (WHY?)
//
#[no_mangle]
pub unsafe extern "C" fn iV_CreateFontIndirect(mut ImageFile: *mut IMAGEFILE,
                                               mut AsciiTable: *mut UWORD,
                                               mut SpaceSize: libc::c_int)
 -> libc::c_int {
    let mut Above: libc::c_int = 0;
    let mut Below: libc::c_int = 0;
    let mut Height: libc::c_int = 0;
    let mut Index: UWORD = 0;
    let mut c: UWORD = 0;
    let mut Font: *mut IVIS_FONT = 0 as *mut IVIS_FONT;
    Font =
        &mut *iVFonts.as_mut_ptr().offset(NumFonts as isize) as
            *mut IVIS_FONT;
    (*Font).FontFile = ImageFile;
    (*Font).AsciiTable = AsciiTable;
    (*Font).FontSpaceSize = SpaceSize;
    (*Font).FontLineSize = 0 as libc::c_int;
    (*Font).FontAbove = 0 as libc::c_int;
    (*Font).FontBelow = 0 as libc::c_int;
    // Initialise the font metrics.
    c = 0 as libc::c_int as UWORD;
    while (c as libc::c_int) < 256 as libc::c_int {
        Index = *AsciiTable.offset(c as isize);
        Above = iV_GetImageYOffset((*Font).FontFile, Index) as libc::c_int;
        Below =
            Above + iV_GetImageHeight((*Font).FontFile, Index) as libc::c_int;
        Height = abs(Above) + abs(Below);
        if Above < (*Font).FontAbove { (*Font).FontAbove = Above }
        if Below > (*Font).FontBelow { (*Font).FontBelow = Below }
        if Height > (*Font).FontLineSize { (*Font).FontLineSize = Height }
        c = c.wrapping_add(1)
    }
    ActiveFontID = NumFonts;
    NumFonts += 1;
    return NumFonts - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iV_SetFont(mut FontID: libc::c_int) {
    ActiveFontID = FontID;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetTextLineSize() -> libc::c_int {
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    return abs((*Font).FontAbove) + abs((*Font).FontBelow);
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetTextAboveBase() -> libc::c_int {
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    return (*Font).FontAbove;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetTextBelowBase() -> libc::c_int {
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    return (*Font).FontBelow;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetTextWidth(mut String: *mut STRING)
 -> libc::c_int {
    let mut Index: libc::c_int = 0;
    let mut MaxX: libc::c_int = 0 as libc::c_int;
    let mut ImageID: UWORD = 0;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    while *String as libc::c_int != 0 as libc::c_int {
        Index = *String as libc::c_int;
        if Index != '#' as i32 {
            if Index != 32 as libc::c_int {
                ImageID = *(*Font).AsciiTable.offset(Index as isize);
                MaxX +=
                    iV_GetImageWidth((*Font).FontFile, ImageID) as libc::c_int
                        + 1 as libc::c_int
            } else { MaxX += (*Font).FontSpaceSize }
        }
        //		DBPRINTF(("letter[%c] currentwidth=%d\n",Index,MaxX));
        String = String.offset(1)
    }
    return MaxX;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetTextDetails(mut Char: libc::c_uchar,
                                           mut Width: *mut UWORD,
                                           mut Height: *mut UWORD,
                                           mut YOffset: *mut SWORD,
                                           mut U: *mut UBYTE,
                                           mut V: *mut UBYTE,
                                           mut TpageID: *mut UWORD) -> BOOL {
    let mut Index: libc::c_int = 0;
    let mut ImageID: UWORD = 0;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    Index = Char as libc::c_int;
    if Index != '#' as i32 {
        if Index != 32 as libc::c_int {
            ImageID = *(*Font).AsciiTable.offset(Index as isize);
            *Width = iV_GetImageWidth((*Font).FontFile, ImageID);
            *Height = iV_GetImageHeight((*Font).FontFile, ImageID);
            *YOffset = iV_GetImageYOffset((*Font).FontFile, ImageID);
            return 1 as libc::c_int
        } else {
            *Width = (*Font).FontSpaceSize as UWORD;
            *Height = 0 as libc::c_int as UWORD;
            *YOffset = 0 as libc::c_int as SWORD;
            return 1 as libc::c_int
        }
    }
    *Width = 0 as libc::c_int as UWORD;
    *Height = 0 as libc::c_int as UWORD;
    *YOffset = 0 as libc::c_int as SWORD;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetCharWidth(mut Char: STRING) -> libc::c_int {
    let mut Index: libc::c_int = 0;
    let mut ImageID: UWORD = 0;
    let mut Width: libc::c_int = 0 as libc::c_int;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    Index = Char as libc::c_int;
    if Index != '#' as i32 {
        if Index != 32 as libc::c_int {
            ImageID = *(*Font).AsciiTable.offset(Index as isize);
            Width =
                iV_GetImageWidth((*Font).FontFile, ImageID) as libc::c_int +
                    1 as libc::c_int
        } else { Width = (*Font).FontSpaceSize }
    }
    return Width;
}
#[no_mangle]
pub unsafe extern "C" fn iV_SetTextColour(mut Index: SWORD) {
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    (*Font).FontColourIndex = Index;
}
static mut FString: [STRING; 256] = [0; 256];
// Must do something about these wastefull static arrays.
static mut FWord: [STRING; 256] = [0; 256];
static mut LastX: libc::c_int = 0;
// Cursor position after last draw.
static mut LastY: libc::c_int = 0;
static mut LastTWidth: libc::c_int = 0;
// Pixel width of the last string draw.
static mut FFlags: UDWORD = (2 as libc::c_int | 4 as libc::c_int) as UDWORD;
static mut RecordExtents: libc::c_int = EXTENTS_NONE as libc::c_int;
static mut ExtentsStartX: libc::c_int = 0;
static mut ExtentsStartY: libc::c_int = 0;
static mut ExtentsEndX: libc::c_int = 0;
static mut ExtentsEndY: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn pie_SetFormattedTextFlags(mut Flags: UDWORD) {
    FFlags = Flags;
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetFormattedTextFlags() -> UDWORD {
    return FFlags;
}
static mut Indirect_pie_DrawText: RENDERTEXT_CALLBACK =
    unsafe {
        Some(pie_DrawText as
                 unsafe extern "C" fn(_: *mut STRING, _: UDWORD, _: UDWORD)
                     -> ())
    };
#[no_mangle]
pub unsafe extern "C" fn SetIndirectDrawTextCallback(mut routine:
                                                         RENDERTEXT_CALLBACK) {
    Indirect_pie_DrawText = routine;
}
// routines used for textdraw
#[no_mangle]
pub unsafe extern "C" fn GetIndirectDrawTextCallback()
 -> RENDERTEXT_CALLBACK {
    return Indirect_pie_DrawText;
}
//UBYTE ExtentsMode=EXTENTS_USEMAXWIDTH;
#[no_mangle]
pub static mut ExtentsMode: UBYTE = 0 as libc::c_int as UBYTE;
#[no_mangle]
pub unsafe extern "C" fn SetExtentsMode_USELAST() {
    ExtentsMode = 1 as libc::c_int as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn SetExtentsMode_USEMAX() {
    ExtentsMode = 0 as libc::c_int as UBYTE;
}
// Draws formatted text with word wrap, long word splitting, embedded
// newlines ( uses @ rather than \n ) and colour mode toggle ( # ) which enables
// or disables font colouring.
//
//	UBYTE *String		The string to display.
//	UDWORD x			x coord of top left of formatted text window.
//	UDWORD y			y coord of top left of formatted text window.
//	UDWORD Width		Width of formatted text window.
//	UDWORD Justify		Justify mode, one of the following:
//							FTEXT_LEFTJUSTIFY
//							FTEXT_CENTRE
//							FTEXT_RIGHTJUSTIFY
//							FTEXT_LEFTJUSTIFYAPPEND
//	BOOL DrawBack		If TRUE then draws transparent box behind text.
//
// NOTE,
//	FTEXT_LEFTJUSTIFYAPPEND should only be used immediatly after calling with FTEXT_LEFTJUSTIFY
//  or FTEXT_LEFTJUSTIFYAPPEND
//
// Returns y coord of next text line.
//
#[no_mangle]
pub unsafe extern "C" fn pie_DrawFormattedText(mut String: *mut libc::c_char,
                                               mut x: UDWORD, mut y: UDWORD,
                                               mut Width: UDWORD,
                                               mut Justify: UDWORD,
                                               mut DrawBack: BOOL) -> UDWORD {
    let mut i: libc::c_int = 0; // Default to left justify.
    let mut si: libc::c_int = 0;
    let mut osi: libc::c_int = 0;
    let mut Len: libc::c_int = strlen(String) as libc::c_int;
    let mut jx: libc::c_int = x as libc::c_int;
    let mut jy: libc::c_int = y as libc::c_int;
    let mut WWidth: libc::c_int = 0;
    let mut GotSpace: BOOL = 0;
    let mut NewLine: BOOL = 0;
    let mut AddLeadingSpace: BOOL = 0 as libc::c_int;
    let mut t: libc::c_int = 0;
    let mut TWidth: libc::c_int = 0;
    si = 0 as libc::c_int;
    //	DBPRINTF(("[%s] @(%d,%d) extentsmode=%d just=%d\n",String,x,y,ExtentsMode,Justify));
    while si < Len {
        // Remove leading spaces, usefull when doing centre justify.
        if FFlags & 1 as libc::c_int as libc::c_uint != 0 {
            while (si as libc::c_uint) < strlen(String) &&
                      *String.offset(si as isize) as libc::c_int == ' ' as i32
                  {
                si += 1
            }
        }
        FString[0 as libc::c_int as usize] = 0 as libc::c_int as STRING;
        if Justify == FTEXT_LEFTJUSTIFYAPPEND as libc::c_int as libc::c_uint {
            WWidth = LastTWidth;
            if FFlags & 4 as libc::c_int as libc::c_uint != 0 {
                AddLeadingSpace = 1 as libc::c_int
            }
        } else {
            WWidth = 0 as libc::c_int;
            AddLeadingSpace = 0 as libc::c_int
        }
        GotSpace = 0 as libc::c_int;
        NewLine = 0 as libc::c_int;
        // Parse through the string, adding words until width is achieved.
        while (si as libc::c_uint) < strlen(String) &&
                  WWidth as libc::c_uint <= Width && NewLine == 0 {
            osi = si;
            //DBPRINTF(("[%s] si=%d wwidth=%d width=%d factor=%d\n",FWord,si,WWidth,Width,DisplayXFactor));
            // Get the next word.
            i = 0 as libc::c_int;
            if AddLeadingSpace != 0 {
                WWidth += iV_GetCharWidth(' ' as i32 as STRING);
                if WWidth as libc::c_uint <= Width {
                    FWord[i as usize] = ' ' as i32 as STRING;
                    i += 1;
                    GotSpace = 1 as libc::c_int;
                    AddLeadingSpace = 0 as libc::c_int
                }
            }
            while *String.offset(si as isize) as libc::c_int !=
                      0 as libc::c_int &&
                      *String.offset(si as isize) as libc::c_int != ' ' as i32
                      && WWidth as libc::c_uint <= Width {
                // Check for new line character.
                if *String.offset(si as isize) as libc::c_int == '#' as i32 {
                    // If it's a colour mode toggle char then just add it to the word.
                    FWord[i as usize] = *String.offset(si as isize);
                    i += 1;
                    si += 1
                } else {
                    // Update this lines pixel width.
                    WWidth += iV_GetCharWidth(*String.offset(si as isize));
                    // If width ok then add this character to the current word.
                    if WWidth as libc::c_uint <= Width {
                        FWord[i as usize] = *String.offset(si as isize);
                        i += 1;
                        si += 1
                    }
                }
            }
            // Don't forget the space.
            if *String.offset(si as isize) as libc::c_int == ' ' as i32 {
                WWidth += iV_GetCharWidth(' ' as i32 as STRING);
                if WWidth as libc::c_uint <= Width {
                    FWord[i as usize] = ' ' as i32 as STRING;
                    i += 1;
                    si += 1;
                    GotSpace = 1 as libc::c_int
                }
            }
            // If we've passed a space and the word goes past the width then rewind
			// to that space and finish this line.
            if GotSpace != 0 {
                if WWidth as libc::c_uint >= Width {
                    if FWord[(i - 1 as libc::c_int) as usize] as libc::c_int
                           == ' ' as i32 {
                        FWord[i as usize] = 0 as libc::c_int as STRING
                    } else { si = osi; break ; }
                }
            }
            // Terminate the word.
            FWord[i as usize] = 0 as libc::c_int as STRING;
            // And add it to the output string.
            strcat(FString.as_mut_ptr(), FWord.as_mut_ptr());
        }
        // Remove trailing spaces, usefull when doing centre justify.
        if FFlags & 2 as libc::c_int as libc::c_uint != 0 {
            t =
                strlen(FString.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                              libc::c_uint) as
                    libc::c_int;
            while t >= 0 as libc::c_int {
                if FString[t as usize] as libc::c_int != ' ' as i32 {
                    break ;
                }
                FString[t as usize] = 0 as libc::c_int as STRING;
                t -= 1
            }
        }
        //DBPRINTF(("end [%s] si=%d wwidth=%d width=%d factor=%d\n",FWord,si,WWidth,Width,DisplayXFactor));
        TWidth = iV_GetTextWidth(FString.as_mut_ptr());
        //		DBPRINTF(("string[%s] is %d of %d pixels wide (according to DrawFormattedText)\n",FString,TWidth,Width));
        // Do justify.
        match Justify {
            1 => {
                jx =
                    x.wrapping_add(Width.wrapping_sub(TWidth as
                                                          libc::c_uint).wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint))
                        as libc::c_int
            }
            2 => {
                jx =
                    x.wrapping_add(Width).wrapping_sub(TWidth as libc::c_uint)
                        as libc::c_int
            }
            0 => { jx = x as libc::c_int }
            3 => {
                jx = LastX;
                jy = LastY;
                Justify = FTEXT_LEFTJUSTIFY as libc::c_int as UDWORD
            }
            _ => { }
        }
        // draw the text.
		// This is an indirect routine
        if Indirect_pie_DrawText.is_some() {
            Indirect_pie_DrawText.expect("non-null function pointer")(FString.as_mut_ptr(),
                                                                      jx as
                                                                          UDWORD,
                                                                      jy as
                                                                          UDWORD);
        }
        //DBPRINTF(("[%s] @ %d,%d\n",FString,jx,jy));
        /* callback type for resload display callback*/
		// remember where we were..
        LastX = jx + TWidth;
        LastY = jy;
        LastTWidth = TWidth;
        if ExtentsMode as libc::c_int == 1 as libc::c_int {
            if RecordExtents == EXTENTS_START as libc::c_int {
                //
                ExtentsStartY =
                    y.wrapping_add(iV_GetTextAboveBase() as libc::c_uint) as
                        libc::c_int;
                ExtentsEndY =
                    jy - iV_GetTextLineSize() + iV_GetTextBelowBase();
                RecordExtents = EXTENTS_END as libc::c_int;
                ExtentsStartX = jx;
                ExtentsEndX = LastX
            } else {
                if jx < ExtentsStartX { ExtentsStartX = jx }
                if LastX > ExtentsEndX { ExtentsEndX = LastX }
            }
            //			DBPRINTF(("extentsstartx = %d extentsendx=%d\n",ExtentsStartX,ExtentsEndX));
        }
        // and move down a line.
        jy += iV_GetTextLineSize()
    } // Was jx, but this broke the console centre justified text background.
    if RecordExtents == EXTENTS_START as libc::c_int {
        RecordExtents = EXTENTS_END as libc::c_int;
        ExtentsStartY =
            y.wrapping_add(iV_GetTextAboveBase() as libc::c_uint) as
                libc::c_int;
        ExtentsEndY = jy - iV_GetTextLineSize() + iV_GetTextBelowBase();
        if ExtentsMode as libc::c_int == 0 as libc::c_int {
            ExtentsStartX = x as libc::c_int;
            //			ExtentsEndX = jx + TWidth;
            ExtentsEndX = x.wrapping_add(Width) as libc::c_int
        } else {
            if jx < ExtentsStartX { ExtentsStartX = jx }
            if LastX > ExtentsEndX { ExtentsEndX = LastX }
        }
    } else if RecordExtents == EXTENTS_END as libc::c_int {
        ExtentsEndY = jy - iV_GetTextLineSize() + iV_GetTextBelowBase();
        if ExtentsMode as libc::c_int == 0 as libc::c_int {
            ExtentsEndX = x.wrapping_add(Width) as libc::c_int
        }
    }
    //#ifdef PSX
//#ifndef TESTBED
//	// Need to take	FTEXT_LEFTJUSTIFYAPPEND into account here.
//	if(DrawBack) {
//		// Move y up to the top of the text.
//		y += iV_GetTextAboveBase();
//
//		// and draw a transparent box behind the text.
//		TransBoxFillRGB_psx(x,y, x+Width,
//							jy-iV_GetTextLineSize()+iV_GetTextBelowBase(),
//							16,16,128);
//	}
//#endif
//#endif
    return jy as UDWORD;
}
static mut OldTextColourIndex: SWORD = -(1 as libc::c_int) as SWORD;
#[no_mangle]
pub unsafe extern "C" fn pie_DrawText(mut string: *mut STRING, mut x: UDWORD,
                                      mut y: UDWORD) {
    let mut Index: libc::c_int = 0;
    let mut ImageID: UWORD = 0;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    /* Colour selection */
    pie_BeginTextRender((*Font).FontColourIndex);
    while *string as libc::c_int != 0 as libc::c_int {
        Index = *string as libc::c_int;
        // Toggle colour mode?
        if Index == '#' as i32 {
            if TextColourIndex as libc::c_int >= 0 as libc::c_int {
                OldTextColourIndex = TextColourIndex;
                TextColourIndex = -(1 as libc::c_int) as SWORD
            } else if OldTextColourIndex as libc::c_int >= 0 as libc::c_int {
                TextColourIndex = OldTextColourIndex
            }
        } else if Index == 32 as libc::c_int {
            x =
                (x as
                     libc::c_uint).wrapping_add((*Font).FontSpaceSize as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        } else {
            ImageID = *(*Font).AsciiTable.offset(Index as isize);
            pie_TextRender((*Font).FontFile, ImageID, x as libc::c_int,
                           y as libc::c_int);
            x =
                (x as
                     libc::c_uint).wrapping_add((iV_GetImageWidth((*Font).FontFile,
                                                                  ImageID) as
                                                     libc::c_int +
                                                     1 as libc::c_int) as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        // Don't use this any more, If the text needs to wrap then use
// pie_DrawFormattedText() defined above.
		/* New bit to make text wrap */
        if x >
               pie_GetVideoBufferWidth().wrapping_sub((*Font).FontSpaceSize as
                                                          libc::c_uint) {
            /* Drop it to the next line if we hit screen edge */
            x = 0 as libc::c_int as UDWORD;
            y =
                (y as
                     libc::c_uint).wrapping_add(iV_GetTextLineSize() as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        string = string.offset(1)
    };
}
//draw Blue tinted bitmap
#[no_mangle]
pub unsafe extern "C" fn pie_RenderBlueTintedBitmap(mut bmp: *mut iBitmap,
                                                    mut x: libc::c_int,
                                                    mut y: libc::c_int,
                                                    mut w: libc::c_int,
                                                    mut h: libc::c_int,
                                                    mut ow: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lineSkip: libc::c_int = 0;
    let mut bp: *mut uint8 = 0 as *mut uint8;
    let mut present: uint8 = 0;
    bp =
        (*psRendSurface).buffer.offset(x as
                                           isize).offset((*psRendSurface).scantable[y
                                                                                        as
                                                                                        usize]
                                                             as isize);
    lineSkip = (*psRendSurface).width - w;
    i = 0 as libc::c_int;
    while i < h {
        j = 0 as libc::c_int;
        while j < w {
            let fresh0 = bmp;
            bmp = bmp.offset(1);
            present = *fresh0;
            if present != 0 {
                *bp = aTransTable3[present as usize]
                // Write in the new version (blue tinted)
            }
            bp = bp.offset(1);
            j += 1
        }
        bmp = bmp.offset((ow - w) as isize);
        bp = bp.offset(lineSkip as isize);
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_RenderDeepBlueTintedBitmap(mut bmp: *mut iBitmap,
                                                        mut x: libc::c_int,
                                                        mut y: libc::c_int,
                                                        mut w: libc::c_int,
                                                        mut h: libc::c_int,
                                                        mut ow: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lineSkip: libc::c_int = 0;
    let mut bp: *mut uint8 = 0 as *mut uint8;
    let mut present: uint8 = 0;
    bp =
        (*psRendSurface).buffer.offset(x as
                                           isize).offset((*psRendSurface).scantable[y
                                                                                        as
                                                                                        usize]
                                                             as isize);
    lineSkip = (*psRendSurface).width - w;
    i = 0 as libc::c_int;
    while i < h {
        j = 0 as libc::c_int;
        while j < w {
            let fresh1 = bmp;
            bmp = bmp.offset(1);
            present = *fresh1;
            if present != 0 {
                *bp = aTransTable4[present as usize]
                // Write in the new version (blue tinted)
            }
            bp = bp.offset(1);
            j += 1
        }
        bmp = bmp.offset((ow - w) as isize);
        bp = bp.offset(lineSkip as isize);
        i += 1
    };
}
//===========================================================
// Partial fix for rendering text on 'video', you can read it now. -Q
// --still to do, add 'boxes' under text so you can see it better.
//===========================================================
#[no_mangle]
pub unsafe extern "C" fn pie_DrawTextToSurface(mut string: *mut STRING,
                                               mut x: libc::c_int,
                                               mut y: libc::c_int) {
    let mut Index: libc::c_int = 0;
    let mut ImageID: UWORD = 0;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    /* Colour selection */
    pie_BeginTextRender((*Font).FontColourIndex);
    while *string as libc::c_int != 0 as libc::c_int {
        Index = *string as libc::c_int;
        // Toggle colour mode?
        if Index == '#' as i32 {
            if TextColourIndex as libc::c_int >= 0 as libc::c_int {
                OldTextColourIndex = TextColourIndex;
                TextColourIndex = -(1 as libc::c_int) as SWORD
            } else if OldTextColourIndex as libc::c_int >= 0 as libc::c_int {
                TextColourIndex = OldTextColourIndex
            }
        } else if Index == 32 as libc::c_int {
            x += (*Font).FontSpaceSize
        } else {
            ImageID = *(*Font).AsciiTable.offset(Index as isize);
            pie_TextRender((*Font).FontFile, ImageID, x, y);
            x +=
                iV_GetImageWidth((*Font).FontFile, ImageID) as libc::c_int +
                    1 as libc::c_int
        }
        // Don't use this any more, If the text needs to wrap then use
// pie_DrawFormattedText() defined above.
		/* New bit to make text wrap */
        if x as libc::c_uint >
               pie_GetVideoBufferWidth().wrapping_sub((*Font).FontSpaceSize as
                                                          libc::c_uint) {
            /* Drop it to the next line if we hit screen edge */
            x = 0 as libc::c_int;
            y += iV_GetTextLineSize()
        }
        string = string.offset(1)
    };
}
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
pub unsafe extern "C" fn pie_DrawText270(mut String: *mut STRING,
                                         mut XPos: libc::c_int,
                                         mut YPos: libc::c_int) {
    let mut Index: libc::c_int = 0;
    let mut ImageID: UWORD = 0;
    let mut Font: *mut IVIS_FONT =
        &mut *iVFonts.as_mut_ptr().offset(ActiveFontID as isize) as
            *mut IVIS_FONT;
    YPos +=
        iV_GetImageWidth((*Font).FontFile,
                         *(*Font).AsciiTable.offset(33 as libc::c_int as
                                                        isize)) as libc::c_int
            + 1 as libc::c_int;
    pie_BeginTextRender((*Font).FontColourIndex);
    while *String as libc::c_int != 0 as libc::c_int {
        Index = *String as libc::c_int;
        if Index != 32 as libc::c_int {
            ImageID = *(*Font).AsciiTable.offset(Index as isize);
            pie_TextRender270((*Font).FontFile, ImageID, XPos, YPos);
            YPos -=
                iV_GetImageWidth((*Font).FontFile, ImageID) as libc::c_int +
                    1 as libc::c_int
        } else { YPos -= (*Font).FontSpaceSize }
        String = String.offset(1)
    };
}
/* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_BeginTextRender(mut ColourIndex: SWORD) {
    TextColourIndex = ColourIndex;
    pie_SetRendMode(REND_TEXT);
    pie_SetBilinear(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pie_TextRender(mut ImageFile: *mut IMAGEFILE,
                                        mut ID: UWORD, mut x: libc::c_int,
                                        mut y: libc::c_int) {
    let mut Red: UDWORD = 0;
    let mut Green: UDWORD = 0;
    let mut Blue: UDWORD = 0;
    let mut Alpha: UDWORD = 255 as libc::c_int as UBYTE as UDWORD;
    let mut psPalette: *mut iColour = 0 as *mut iColour;
    if TextColourIndex as libc::c_int == -(1 as libc::c_int) ||
           TextColourIndex as libc::c_int == 255 as libc::c_int {
        pie_SetColour(0xffffffff as libc::c_uint);
    } else if TextColourIndex as libc::c_int == -(1 as libc::c_int) {
        pie_SetColour(0xffffffff as libc::c_uint);
    } else if TextColourIndex as libc::c_int == -(2 as libc::c_int) {
        pie_SetColour(0xffa0a0ff as libc::c_uint);
    } else if TextColourIndex as libc::c_int == -(3 as libc::c_int) {
        pie_SetColour(0xff6060c0 as libc::c_uint);
    } else {
        psPalette = pie_GetGamePal();
        Red = (*psPalette.offset(TextColourIndex as isize)).r as UDWORD;
        Green = (*psPalette.offset(TextColourIndex as isize)).g as UDWORD;
        Blue = (*psPalette.offset(TextColourIndex as isize)).b as UDWORD;
        pie_SetColour(Alpha << 24 as libc::c_int | Red << 16 as libc::c_int |
                          Green << 8 as libc::c_int | Blue);
    }
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pie_DrawImageFileID(ImageFile, ID, x, y);
    pie_SetColourKeyedBlack(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn TextRender270(mut ImageFile: *mut IMAGEFILE,
                                       mut ImageID: UWORD, mut x: libc::c_int,
                                       mut y: libc::c_int) {
    /*
	int w,h,i,j,Modulus;
	uint8 *srcbp, *bp;
	IMAGEDEF *Image;
	iBitmap *Bmp;
	UBYTE	present;
	UDWORD	width;

	assert(ImageID < ImageFile->Header.NumImages);				// setup the image
	Image = &ImageFile->ImageDefs[ImageID];
	Modulus = ImageFile->TexturePages[Image->TPageID].width;

	Bmp = ImageFile->TexturePages[Image->TPageID].bmp;
	Bmp += ((UDWORD)Image->Tu) + ((UDWORD)Image->Tv) * Modulus;

	// ideally i would just call
	//	iV_ppBitmapRot270(Bmp,
	//			   	x+Image->YOffset,
	//				y+Image->XOffset,
	//			   	Image->Width,
	//				Image->Height,Modulus);
	// but we want a transparant image. Instead of creating a load
	// of blank drawing functions I've just provided the one for 4101.
	// if we need other modes I'll arrange it nicely in the library.
	// note 3dfx uses a different method (gl_textrender270).

	x = x+Image->YOffset;										// sort the position
	y = y+Image->XOffset;
	w = Image->Width;
	h = Image->Height;

//	y -= iV_GetImageWidth(ImageFile,ImageID);					// since drawing 'upside down'

	srcbp = bp = (uint8 *) psRendSurface->buffer + x + psRendSurface->scantable[y+h-1];
	width = pie_GetVideoBufferWidth();
	for (j=0; j<h; j++)
	{
		bp = srcbp;
		for (i=0; i<w; i++)
		{
			if(*Bmp++)
			{
				present =  *bp;							// What colour is there at the moment?
				*bp = aTransTable2[present];			// Write in the new version (brite?)
				//	*bp = (UBYTE)TextColourIndex;		// old
			}
			bp -= width;		// goto previous line.
		}
		srcbp++;
		Bmp += (Modulus - w);
	}
	*/
}
#[no_mangle]
pub unsafe extern "C" fn pie_TextRender270(mut ImageFile: *mut IMAGEFILE,
                                           mut ImageID: UWORD,
                                           mut x: libc::c_int,
                                           mut y: libc::c_int) {
    let mut Red: UDWORD = 0;
    let mut Green: UDWORD = 0;
    let mut Blue: UDWORD = 0;
    let mut Alpha: UDWORD = 255 as libc::c_int as UBYTE as UDWORD;
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    let mut rendStyle: PIESTYLE =
        PIESTYLE{pieFlag: 0,
                 colour:
                     PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                 specular:
                     PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
                 light: 0,
                 trans: 0,
                 scale: 0,
                 height: 0,};
    let mut psPalette: *mut iColour = 0 as *mut iColour;
    Image =
        &mut *(*ImageFile).ImageDefs.offset(ImageID as isize) as
            *mut IMAGEDEF;
    //not coloured yet
    if TextColourIndex as libc::c_int == -(1 as libc::c_int) {
        pie_SetColour(0xffffffff as libc::c_uint &
                          0x80ffffff as libc::c_uint);
        //special case semi transparent for rotated text
    } else if TextColourIndex as libc::c_int == -(2 as libc::c_int) {
        pie_SetColour(0xffa0a0ff as libc::c_uint);
    } else if TextColourIndex as libc::c_int == -(3 as libc::c_int) {
        pie_SetColour(0xff6060c0 as libc::c_uint);
    } else {
        psPalette = pie_GetGamePal();
        Red = (*psPalette.offset(TextColourIndex as isize)).r as UDWORD;
        Green = (*psPalette.offset(TextColourIndex as isize)).g as UDWORD;
        Blue = (*psPalette.offset(TextColourIndex as isize)).b as UDWORD;
        pie_SetColour(Alpha << 24 as libc::c_int | Red << 16 as libc::c_int |
                          Green << 8 as libc::c_int | Blue);
    }
    pie_SetRendMode(REND_ALPHA_TEXT);
    pieImage.texPage =
        (*ImageFile).TPageIDs[(*Image).TPageID as usize] as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).YOffset as libc::c_int) as SWORD;
    dest.y =
        (y + (*Image).XOffset as libc::c_int - (*Image).Width as libc::c_int)
            as SWORD;
    dest.w = (*Image).Width as SWORD;
    dest.h = (*Image).Height as SWORD;
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pie_DrawImage270(&mut pieImage, &mut dest, &mut rendStyle);
    pie_SetColourKeyedBlack(0 as libc::c_int);
}
