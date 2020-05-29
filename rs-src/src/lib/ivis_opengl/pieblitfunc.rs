use ::libc;
extern "C" {
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
    #[no_mangle]
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    #[no_mangle]
    fn glTexImage2D(target: GLenum, level: GLint, internalFormat: GLint,
                    width: GLsizei, height: GLsizei, border: GLint,
                    format: GLenum, type_0: GLenum,
                    pixels: *const libc::c_void);
    #[no_mangle]
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    #[no_mangle]
    fn glTexEnvf(target: GLenum, pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn glVertex2f(x: GLfloat, y: GLfloat);
    #[no_mangle]
    fn glEnd();
    #[no_mangle]
    fn glBegin(mode: GLenum);
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* Seed the random number generator with the given number.  */
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn pie_GetGamePal() -> *mut iColour;
    //extern void pie_Draw3DIntelShape(iIMDShape *shape, int frame, int team, UDWORD colour, UDWORD specular, int pieFlag, int pieData);
//extern void pie_Draw3DNowShape(iIMDShape *shape, int frame, int team, UDWORD col, UDWORD spec, int pieFlag, int pieFlagData);
    #[no_mangle]
    fn pie_DrawImage(image: *mut PIEIMAGE, dest: *mut PIERECT,
                     style: *mut PIESTYLE);
    #[no_mangle]
    fn pie_DrawRect(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD,
                    colour: UDWORD, bClip: BOOL);
    #[no_mangle]
    fn pie_GlobalRenderBegin();
    #[no_mangle]
    fn pie_GlobalRenderEnd(bForceClearToBlack: BOOL);
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
    #[no_mangle]
    fn pie_SetBilinear(bilinearOn: BOOL);
    #[no_mangle]
    fn pie_SetColourKeyedBlack(keyingOn: BOOL);
    #[no_mangle]
    fn pie_SetRendMode(rendMode: REND_MODE);
    #[no_mangle]
    fn pie_SetColour(val: UDWORD);
    // vid.c 0.1 10-01-96.22-11-96
    //*************************************************************************
//patch
    //#define	iV_DrawColourTransImage	pie_ImageFileIDColour
    //*************************************************************************
    // Direct3D 640x480x16bit RGB renderer (mmx)
    // Direct3D 640x480x16bit hardware
    // Direct3D 640x480x16bit hardware
    // 3dfx Glide API
    // 16bit software mode for video
    // off-screen surface
    // PlayStation - added by tjc
    // undefined mode
    //*************************************************************************
// polygon flags	b0..b7: col, b24..b31: anim index
    //#define PIE_FLAT			0x00000100
    //#define PIE_WIRE			0x00000400
    //#define PIE_GOURAUD			0x00001000
    //#define PIE_TEXANIM			0x00004000	// PIE_TEX must be set also
    // - use playstation texture allocation method
    // Freshly created by the BSP 
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    /* backDrop */
    #[no_mangle]
    fn screen_SetBackDrop(newBackDropBmp: *mut UWORD, width: UDWORD,
                          height: UDWORD);
    #[no_mangle]
    fn screen_SetBackDropFromFile(filename: *mut libc::c_char);
    #[no_mangle]
    fn screen_Upload(newBackDropBmp: *mut UWORD);
}
pub type __time_t = libc::c_long;
/* Returned by `time'.  */
pub type time_t = __time_t;
pub type GLenum = libc::c_uint;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
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
pub type CHAR = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint32 = libc::c_uint;
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
pub type _screenType = libc::c_uint;
pub const SCREEN_COVERMOUNT: _screenType = 8;
pub const SCREEN_SLIDE5: _screenType = 7;
pub const SCREEN_SLIDE4: _screenType = 6;
pub const SCREEN_SLIDE3: _screenType = 5;
pub const SCREEN_SLIDE2: _screenType = 4;
pub const SCREEN_SLIDE1: _screenType = 3;
pub const SCREEN_MISSIONEND: _screenType = 2;
pub const SCREEN_CREDITS: _screenType = 1;
pub const SCREEN_RANDOMBDROP: _screenType = 0;
pub type SCREENTYPE = _screenType;
/* **************************************************************************/
/*
 *	Local Definitions
 */
/* **************************************************************************/
#[no_mangle]
pub static mut backDropBmp: [UWORD; 1228800] = [0; 1228800];
#[no_mangle]
pub static mut gSurfaceOffsetX: SDWORD = 0;
#[no_mangle]
pub static mut gSurfaceOffsetY: SDWORD = 0;
#[no_mangle]
pub static mut pgSrcData: *mut UWORD = 0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut gSrcWidth: SDWORD = 0;
#[no_mangle]
pub static mut gSrcHeight: SDWORD = 0;
#[no_mangle]
pub static mut gSrcStride: SDWORD = 0;
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
#[no_mangle]
pub static mut rendStyle: PIESTYLE =
    PIESTYLE{pieFlag: 0,
             colour: PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
             specular:
                 PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
             light: 0,
             trans: 0,
             scale: 0,
             height: 0,};
#[no_mangle]
pub static mut rectVerts: [POINT; 4] = [POINT{x: 0, y: 0,}; 4];
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
pub unsafe extern "C" fn pie_Line(mut x0: libc::c_int, mut y0: libc::c_int,
                                  mut x1: libc::c_int, mut y1: libc::c_int,
                                  mut colour: uint32) {
    //	PIELIGHT light;
    pie_SetRendMode(REND_FLAT);
    pie_SetColour(colour);
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetColourKeyedBlack(0 as libc::c_int);
    glBegin(0x3 as libc::c_int as GLenum);
    glVertex2f(x0 as GLfloat, y0 as GLfloat);
    glVertex2f(x1 as GLfloat, y1 as GLfloat);
    glEnd();
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_Box(mut x0: libc::c_int, mut y0: libc::c_int,
                                 mut x1: libc::c_int, mut y1: libc::c_int,
                                 mut colour: uint32) {
    //	PIELIGHT light;
//	iColour* psPalette;
    pie_SetRendMode(REND_FLAT);
    pie_SetColour(colour);
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetColourKeyedBlack(0 as libc::c_int);
    if x0 > (*psRendSurface).clip.right || x1 < (*psRendSurface).clip.left ||
           y0 > (*psRendSurface).clip.bottom || y1 < (*psRendSurface).clip.top
       {
        return
    }
    if x0 < (*psRendSurface).clip.left { x0 = (*psRendSurface).clip.left }
    if x1 > (*psRendSurface).clip.right { x1 = (*psRendSurface).clip.right }
    if y0 < (*psRendSurface).clip.top { y0 = (*psRendSurface).clip.top }
    if y1 > (*psRendSurface).clip.bottom { y1 = (*psRendSurface).clip.bottom }
    glBegin(0x3 as libc::c_int as GLenum);
    glVertex2f(x0 as GLfloat, y0 as GLfloat);
    glVertex2f(x1 as GLfloat, y0 as GLfloat);
    glVertex2f(x1 as GLfloat, y1 as GLfloat);
    glVertex2f(x0 as GLfloat, y1 as GLfloat);
    glVertex2f(x0 as GLfloat, y0 as GLfloat);
    glEnd();
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_BoxFillIndex(mut x0: libc::c_int,
                                          mut y0: libc::c_int,
                                          mut x1: libc::c_int,
                                          mut y1: libc::c_int,
                                          mut colour: UBYTE) {
    let mut light: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut psPalette: *mut iColour = 0 as *mut iColour;
    pie_SetRendMode(REND_FLAT);
    pie_SetTexturePage(-(1 as libc::c_int));
    if x0 > (*psRendSurface).clip.right || x1 < (*psRendSurface).clip.left ||
           y0 > (*psRendSurface).clip.bottom || y1 < (*psRendSurface).clip.top
       {
        return
    }
    if x0 < (*psRendSurface).clip.left { x0 = (*psRendSurface).clip.left }
    if x1 > (*psRendSurface).clip.right { x1 = (*psRendSurface).clip.right }
    if y0 < (*psRendSurface).clip.top { y0 = (*psRendSurface).clip.top }
    if y1 > (*psRendSurface).clip.bottom { y1 = (*psRendSurface).clip.bottom }
    psPalette = pie_GetGamePal();
    light.byte.r = (*psPalette.offset(colour as isize)).r;
    light.byte.g = (*psPalette.offset(colour as isize)).g;
    light.byte.b = (*psPalette.offset(colour as isize)).b;
    light.byte.a = 255 as libc::c_int as UBYTE;
    pie_DrawRect(x0, y0, x1, y1, light.argb, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pie_BoxFill(mut x0: libc::c_int, mut y0: libc::c_int,
                                     mut x1: libc::c_int, mut y1: libc::c_int,
                                     mut colour: uint32) {
    pie_SetRendMode(REND_FLAT);
    pie_SetTexturePage(-(1 as libc::c_int));
    if x0 > (*psRendSurface).clip.right || x1 < (*psRendSurface).clip.left ||
           y0 > (*psRendSurface).clip.bottom || y1 < (*psRendSurface).clip.top
       {
        return
    }
    if x0 < (*psRendSurface).clip.left { x0 = (*psRendSurface).clip.left }
    if x1 > (*psRendSurface).clip.right { x1 = (*psRendSurface).clip.right }
    if y0 < (*psRendSurface).clip.top { y0 = (*psRendSurface).clip.top }
    if y1 > (*psRendSurface).clip.bottom { y1 = (*psRendSurface).clip.bottom }
    pie_DrawRect(x0, y0, x1, y1, colour, 0 as libc::c_int);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_TransBoxFill(mut x0: SDWORD, mut y0: SDWORD,
                                          mut x1: SDWORD, mut y1: SDWORD) {
    let mut rgb: UDWORD = 0; //blue
    let mut transparency: UDWORD = 0;
    rgb =
        ((16 as libc::c_int) << 16 as libc::c_int |
             (16 as libc::c_int) << 8 as libc::c_int | 128 as libc::c_int) as
            UDWORD;
    transparency = 128 as libc::c_int as UDWORD;
    pie_UniTransBoxFill(x0, y0, x1, y1, rgb, transparency);
    //	pie_doWeirdBoxFX(x0,y0,x1,y1);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_UniTransBoxFill(mut x0: SDWORD, mut y0: SDWORD,
                                             mut x1: SDWORD, mut y1: SDWORD,
                                             mut rgb: UDWORD,
                                             mut transparency: UDWORD) {
    let mut light: UDWORD = 0;
    if x0 > (*psRendSurface).clip.right || x1 < (*psRendSurface).clip.left ||
           y0 > (*psRendSurface).clip.bottom || y1 < (*psRendSurface).clip.top
       {
        return
    }
    if x0 < (*psRendSurface).clip.left { x0 = (*psRendSurface).clip.left }
    if x1 > (*psRendSurface).clip.right { x1 = (*psRendSurface).clip.right }
    if y0 < (*psRendSurface).clip.top { y0 = (*psRendSurface).clip.top }
    if y1 > (*psRendSurface).clip.bottom { y1 = (*psRendSurface).clip.bottom }
    if transparency == 0 as libc::c_int as libc::c_uint {
        transparency = 127 as libc::c_int as UDWORD
    }
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetRendMode(REND_ALPHA_FLAT);
    light =
        (rgb &
             0xffffff as libc::c_int as
                 libc::c_uint).wrapping_add(transparency <<
                                                24 as libc::c_int);
    pie_DrawRect(x0, y0, x1, y1, light, 0 as libc::c_int);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_DrawImageFileID(mut ImageFile: *mut IMAGEFILE,
                                             mut ID: UWORD,
                                             mut x: libc::c_int,
                                             mut y: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    //	iBitmap *bmp;
//	UDWORD modulus;
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    //	SDWORD width;
//	SDWORD height;
//	SDWORD delta;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    pieImage.texPage =
        (*ImageFile).TPageIDs[(*Image).TPageID as usize] as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
    dest.y = (y + (*Image).YOffset as libc::c_int) as SWORD;
    dest.w = (*Image).Width as SWORD;
    dest.h = (*Image).Height as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
}
#[no_mangle]
pub static mut bAddSprites: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut addSpriteLevel: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn pie_SetAdditiveSprites(mut val: BOOL) {
    bAddSprites = val;
}
#[no_mangle]
pub unsafe extern "C" fn pie_SetAdditiveSpriteLevel(mut val: UDWORD) {
    addSpriteLevel = val;
}
#[no_mangle]
pub unsafe extern "C" fn pie_GetAdditiveSprites() -> BOOL {
    return bAddSprites;
}
#[no_mangle]
pub unsafe extern "C" fn pie_ImageFileID(mut ImageFile: *mut IMAGEFILE,
                                         mut ID: UWORD, mut x: libc::c_int,
                                         mut y: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    //	iBitmap *bmp;
//	UDWORD modulus;
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    //	SDWORD width;
//	SDWORD height;
//	SDWORD delta;
//	SDWORD div,wave;
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    if pie_GetAdditiveSprites() != 0 {
        pie_SetBilinear(1 as libc::c_int);
        pie_SetRendMode(REND_ALPHA_TEX);
        pie_SetColour(addSpriteLevel);
    } else {
        pie_SetBilinear(0 as libc::c_int);
        pie_SetRendMode(REND_GOURAUD_TEX);
        pie_SetColour(0xffffffff as libc::c_uint);
    }
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pieImage.texPage =
        (*ImageFile).TPageIDs[(*Image).TPageID as usize] as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
    dest.y = (y + (*Image).YOffset as libc::c_int) as SWORD;
    dest.w = (*Image).Width as SWORD;
    dest.h = (*Image).Height as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_ImageFileIDTile(mut ImageFile: *mut IMAGEFILE,
                                             mut ID: UWORD,
                                             mut x: libc::c_int,
                                             mut y: libc::c_int,
                                             mut x0: libc::c_int,
                                             mut y0: libc::c_int,
                                             mut Width: libc::c_int,
                                             mut Height: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut hRep: SDWORD = 0;
    let mut hRemainder: SDWORD = 0;
    let mut vRep: SDWORD = 0;
    let mut vRemainder: SDWORD = 0;
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    pie_SetBilinear(0 as libc::c_int);
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetColour(0xffffffff as libc::c_uint);
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pieImage.texPage =
        (*ImageFile).TPageIDs[(*Image).TPageID as usize] as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
    dest.y = (y + (*Image).YOffset as libc::c_int) as SWORD;
    dest.w = (*Image).Width as SWORD;
    dest.h = (*Image).Height as SWORD;
    vRep = Height / (*Image).Height as libc::c_int;
    vRemainder = Height - vRep * (*Image).Height as libc::c_int;
    while vRep > 0 as libc::c_int {
        hRep = Width / (*Image).Width as libc::c_int;
        hRemainder = Width - hRep * (*Image).Width as libc::c_int;
        pieImage.tw = (*Image).Width as SWORD;
        dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
        dest.w = (*Image).Width as SWORD;
        while hRep > 0 as libc::c_int {
            pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
            hRep -= 1;
            dest.x =
                (dest.x as libc::c_int + (*Image).Width as libc::c_int) as
                    SWORD
        }
        //draw remainder
        if hRemainder > 0 as libc::c_int {
            pieImage.tw = hRemainder as SWORD;
            dest.w = hRemainder as SWORD;
            pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
        }
        vRep -= 1;
        dest.y =
            (dest.y as libc::c_int + (*Image).Height as libc::c_int) as SWORD
    }
    //draw remainder
    if vRemainder > 0 as libc::c_int {
        hRep = Width / (*Image).Width as libc::c_int;
        hRemainder = Width - hRep * (*Image).Width as libc::c_int;
        pieImage.th = vRemainder as SWORD;
        dest.h = vRemainder as SWORD;
        //as above
        pieImage.tw = (*Image).Width as SWORD;
        dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
        dest.w = (*Image).Width as SWORD;
        while hRep > 0 as libc::c_int {
            pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
            hRep -= 1;
            dest.x =
                (dest.x as libc::c_int + (*Image).Width as libc::c_int) as
                    SWORD
        }
        //draw remainder
        if hRemainder > 0 as libc::c_int {
            pieImage.tw = hRemainder as SWORD; //changed by alex 19 oct 98
            dest.w = hRemainder as SWORD;
            pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_ImageFileIDStretch(mut ImageFile: *mut IMAGEFILE,
                                                mut ID: UWORD,
                                                mut x: libc::c_int,
                                                mut y: libc::c_int,
                                                mut Width: libc::c_int,
                                                mut Height: libc::c_int) {
    let mut Image: *mut IMAGEDEF = 0 as *mut IMAGEDEF;
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    Image = &mut *(*ImageFile).ImageDefs.offset(ID as isize) as *mut IMAGEDEF;
    pie_SetBilinear(0 as libc::c_int);
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetColour(0xffffffff as libc::c_uint);
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pieImage.texPage =
        (*ImageFile).TPageIDs[(*Image).TPageID as usize] as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
    dest.y = (y + (*Image).YOffset as libc::c_int) as SWORD;
    dest.w = Width as SWORD;
    dest.h = Height as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
}
#[no_mangle]
pub unsafe extern "C" fn pie_ImageDef(mut Image: *mut IMAGEDEF,
                                      mut Bmp: *mut iBitmap,
                                      mut Modulus: UDWORD, mut x: libc::c_int,
                                      mut y: libc::c_int,
                                      mut bBilinear: BOOL) {
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    pie_SetBilinear(bBilinear);
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetColour(0xffffffff as libc::c_uint);
    pie_SetColourKeyedBlack(1 as libc::c_int);
    pieImage.texPage = (*Image).TPageID as SDWORD;
    pieImage.tu = (*Image).Tu as SWORD;
    pieImage.tv = (*Image).Tv as SWORD;
    pieImage.tw = (*Image).Width as SWORD;
    pieImage.th = (*Image).Height as SWORD;
    dest.x = (x + (*Image).XOffset as libc::c_int) as SWORD;
    dest.y = (y + (*Image).YOffset as libc::c_int) as SWORD;
    dest.w = (*Image).Width as SWORD;
    dest.h = (*Image).Height as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
    pie_SetBilinear(0 as libc::c_int);
    //changed by alex 19 oct 98
}
#[no_mangle]
pub unsafe extern "C" fn pie_ImageDefTrans(mut Image: *mut IMAGEDEF,
                                           mut Bmp: *mut iBitmap,
                                           mut Modulus: UDWORD,
                                           mut x: libc::c_int,
                                           mut y: libc::c_int,
                                           mut TransRate: libc::c_int) {
    pie_ImageDef(Image, Bmp, Modulus, x, y, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pie_UploadDisplayBuffer(mut DisplayBuffer:
                                                     *mut libc::c_char) {
    pie_GlobalRenderEnd(0 as libc::c_int);
    screen_Upload(0 as *mut UWORD);
    pie_GlobalRenderBegin();
}
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
pub unsafe extern "C" fn pie_DownloadDisplayBuffer(mut DisplayBuffer:
                                                       *mut libc::c_char) {
    /* Not implemented */
}
#[no_mangle]
pub static mut radarTexture: UDWORD = 0;
#[no_mangle]
pub static mut radarBitmap: [libc::c_uchar; 65536] = [0; 65536];
#[no_mangle]
pub unsafe extern "C" fn pie_InitRadar() -> BOOL {
    glGenTextures(1 as libc::c_int, &mut radarTexture);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_ShutdownRadar() -> BOOL {
    glDeleteTextures(1 as libc::c_int, &mut radarTexture);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_DownLoadRadar(mut buffer: *mut libc::c_uchar,
                                           mut texPageID: UDWORD) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut psPalette: *mut iColour = pie_GetGamePal();
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while i < (128 as libc::c_int * 128 as libc::c_int) as libc::c_uint {
        let fresh0 = j;
        j = j.wrapping_add(1);
        radarBitmap[fresh0 as usize] =
            (*psPalette.offset(*buffer.offset(i as isize) as isize)).r;
        let fresh1 = j;
        j = j.wrapping_add(1);
        radarBitmap[fresh1 as usize] =
            (*psPalette.offset(*buffer.offset(i as isize) as isize)).g;
        let fresh2 = j;
        j = j.wrapping_add(1);
        radarBitmap[fresh2 as usize] =
            (*psPalette.offset(*buffer.offset(i as isize) as isize)).b;
        if *buffer.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            let fresh3 = j;
            j = j.wrapping_add(1);
            radarBitmap[fresh3 as usize] = 0 as libc::c_int as libc::c_uchar
        } else {
            let fresh4 = j;
            j = j.wrapping_add(1);
            radarBitmap[fresh4 as usize] = 255 as libc::c_int as libc::c_uchar
        }
        i = i.wrapping_add(1)
    }
    pie_SetTexturePage(radarTexture as SDWORD);
    glTexImage2D(0xde1 as libc::c_int as GLenum, 0 as libc::c_int,
                 0x1908 as libc::c_int, 128 as libc::c_int,
                 128 as libc::c_int, 0 as libc::c_int,
                 0x1908 as libc::c_int as GLenum,
                 0x1401 as libc::c_int as GLenum,
                 radarBitmap.as_mut_ptr() as *const libc::c_void);
    glTexEnvf(0x2300 as libc::c_int as GLenum,
              0x2200 as libc::c_int as GLenum,
              0x2100 as libc::c_int as GLfloat);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2801 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2800 as libc::c_int as GLenum, 0x2601 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2802 as libc::c_int as GLenum, 0x2900 as libc::c_int);
    glTexParameteri(0xde1 as libc::c_int as GLenum,
                    0x2803 as libc::c_int as GLenum, 0x2900 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn pie_RenderRadar(mut Image: *mut IMAGEDEF,
                                         mut Bmp: *mut iBitmap,
                                         mut Modulus: UDWORD,
                                         mut x: libc::c_int,
                                         mut y: libc::c_int) {
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    pie_SetBilinear(1 as libc::c_int);
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetColour(0xffffffff as libc::c_uint);
    pie_SetColourKeyedBlack(1 as libc::c_int);
    //special case function because texture is held outside of texture list
    pieImage.texPage = radarTexture as SDWORD;
    pieImage.tu = 0 as libc::c_int as SWORD;
    pieImage.tv = 0 as libc::c_int as SWORD;
    pieImage.tw = 256 as libc::c_int as SWORD;
    pieImage.th = 256 as libc::c_int as SWORD;
    dest.x = x as SWORD;
    dest.y = y as SWORD;
    dest.w = 128 as libc::c_int as SWORD;
    dest.h = 128 as libc::c_int as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
}
#[no_mangle]
pub unsafe extern "C" fn pie_RenderRadarRotated(mut Image: *mut IMAGEDEF,
                                                mut Bmp: *mut iBitmap,
                                                mut Modulus: UDWORD,
                                                mut x: libc::c_int,
                                                mut y: libc::c_int,
                                                mut angle: libc::c_int) {
    let mut pieImage: PIEIMAGE =
        PIEIMAGE{texPage: 0, tu: 0, tv: 0, tw: 0, th: 0,};
    let mut dest: PIERECT = PIERECT{x: 0, y: 0, w: 0, h: 0,};
    pie_SetBilinear(1 as libc::c_int);
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetColour(0xffffffff as libc::c_uint);
    pie_SetColourKeyedBlack(1 as libc::c_int);
    //special case function because texture is held outside of texture list
    pieImage.texPage = radarTexture as SDWORD;
    pieImage.tu = 0 as libc::c_int as SWORD;
    pieImage.tv = 0 as libc::c_int as SWORD;
    pieImage.tw = 256 as libc::c_int as SWORD;
    pieImage.th = 256 as libc::c_int as SWORD;
    dest.x = x as SWORD;
    dest.y = y as SWORD;
    dest.w = 128 as libc::c_int as SWORD;
    dest.h = 128 as libc::c_int as SWORD;
    pie_DrawImage(&mut pieImage, &mut dest, &mut rendStyle);
}
#[no_mangle]
pub unsafe extern "C" fn pie_ResetBackDrop() {
    screen_SetBackDrop(backDropBmp.as_mut_ptr(), 640 as libc::c_int as UDWORD,
                       480 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn pie_LoadBackDrop(mut screenType: SCREENTYPE,
                                          mut b3DFX: BOOL) {
    let mut chooser0: UDWORD = 0;
    let mut chooser1: UDWORD = 0;
    let mut backd: [CHAR; 128] = [0; 128];
    //randomly load in a backdrop piccy.
    srand(time(0 as *mut time_t) as libc::c_uint);
    chooser0 = 0 as libc::c_int as UDWORD;
    chooser1 = (rand() % 7 as libc::c_int) as UDWORD;
    match screenType as libc::c_uint {
        0 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/bdrops/%d%d-bdrop.jpg\x00" as *const u8 as
                        *const libc::c_char, chooser0, chooser1);
        }
        8 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/bdrops/demo-bdrop.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        2 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/bdrops/missionend.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        3 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/slides/slide1.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        4 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/slides/slide2.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        5 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/slides/slide3.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        6 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/slides/slide4.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        7 => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/slides/slide5.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
        _ => {
            sprintf(backd.as_mut_ptr(),
                    b"texpages/bdrops/credits.jpg\x00" as *const u8 as
                        *const libc::c_char);
        }
    }
    screen_SetBackDropFromFile(backd.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn pie_D3DSetupRenderForFlip(mut surfaceOffsetX: SDWORD,
                                                   mut surfaceOffsetY: SDWORD,
                                                   mut pSrcData: *mut UWORD,
                                                   mut srcWidth: SDWORD,
                                                   mut srcHeight: SDWORD,
                                                   mut srcStride: SDWORD) {
    gSurfaceOffsetX = surfaceOffsetX;
    gSurfaceOffsetY = surfaceOffsetY;
    pgSrcData = pSrcData;
    gSrcWidth = srcWidth;
    gSrcHeight = srcHeight;
    gSrcStride = srcStride;
}
