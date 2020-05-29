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
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn SetTransFilter(rgb: UDWORD, tablenumber: UDWORD);
}
pub type size_t = libc::c_uint;
pub type UDWORD = libc::c_uint;
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
pub type iBool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexture {
    pub xshift: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub bmp: *mut iBitmap,
    pub pPal: *mut iColour,
    pub bColourKeyed: iBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVertex {
    pub x: int32,
    pub y: int32,
    pub z: int32,
    pub u: int32,
    pub v: int32,
    pub g: uint8,
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
//*************************************************************************
#[no_mangle]
pub static mut iV_VSync: Option<unsafe extern "C" fn() -> ()> = None;
//*************************************************************************
#[no_mangle]
pub static mut rendSurface: iSurface =
    iSurface{usr: 0,
             flags: 0,
             xcentre: 0,
             ycentre: 0,
             xpshift: 0,
             ypshift: 0,
             clip: iClip{left: 0, top: 0, right: 0, bottom: 0,},
             buffer: 0 as *const uint8 as *mut uint8,
             scantable: [0; 1024],
             width: 0,
             height: 0,
             size: 0,};
#[no_mangle]
pub static mut psRendSurface: *mut iSurface =
    0 as *const iSurface as *mut iSurface;
//*************************************************************************
static mut _VIDEO_MEM: *mut uint8 = 0 as *const uint8 as *mut uint8;
static mut _VIDEO_SIZE: int32 = 0;
static mut _VIDEO_LOCK: iBool = 0;
//*************************************************************************
//temporary definition
//void (*iV_pPolygon)(int npoints, iVertex *vrt, iTexture *tex, uint32 type);
//void (*iV_pTriangle)(iVertex *vrt, iTexture *tex, uint32 type);
#[no_mangle]
pub static mut iV_ppBitmapColourTrans:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int) -> ()> =
    None;
//void (*iV_DrawStretchImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,int Width,int Height);
//*************************************************************************
//*************************************************************************
//*** return mode size in bytes
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_VideoMemorySize(mut mode: libc::c_int) -> int32 {
    let mut size: UDWORD = 0;
    size = pie_GetVideoBufferWidth().wrapping_mul(pie_GetVideoBufferHeight());
    return size as int32;
}
//*************************************************************************
//*** allocate and lock video memory (call only once!)
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_VideoMemoryLock(mut mode: libc::c_int) -> iBool {
    let mut size: int32 = 0;
    size = iV_VideoMemorySize(mode);
    if size == 0 as libc::c_int { return 0 as libc::c_int }
    _VIDEO_MEM = memMallocRelease(size as size_t) as *mut uint8;
    if _VIDEO_MEM.is_null() { return 0 as libc::c_int }
    _VIDEO_SIZE = size;
    _VIDEO_LOCK = 1 as libc::c_int;
    debug(LOG_3D,
          b"vid[VideoMemoryLock] = locked %dK of video memory\n\x00" as
              *const u8 as *const libc::c_char, size / 1024 as libc::c_int);
    return 1 as libc::c_int;
}
//*************************************************************************
//***
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_VideoMemoryFree() {
    if _VIDEO_LOCK != 0 { return }
    if !_VIDEO_MEM.is_null() {
        memFreeRelease(_VIDEO_MEM as *mut libc::c_void);
        _VIDEO_MEM = 0 as *mut uint8;
        _VIDEO_MEM = 0 as *mut uint8;
        _VIDEO_SIZE = 0 as libc::c_int
    };
}
//*************************************************************************
//***
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_VideoMemoryUnlock() {
    if _VIDEO_LOCK != 0 { _VIDEO_LOCK = 0 as libc::c_int }
    iV_VideoMemoryFree();
}
//*************************************************************************
//***
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_VideoMemoryAlloc(mut mode: libc::c_int)
 -> *mut uint8 {
    let mut size: int32 = 0;
    size = iV_VideoMemorySize(mode);
    if size == 0 as libc::c_int { return 0 as *mut uint8 }
    if _VIDEO_LOCK != 0 {
        if size <= _VIDEO_SIZE { return _VIDEO_MEM }
        return 0 as *mut uint8
    }
    _VIDEO_MEM = memMallocRelease(size as size_t) as *mut uint8;
    if _VIDEO_MEM.is_null() { return 0 as *mut uint8 }
    _VIDEO_SIZE = size;
    return _VIDEO_MEM;
}
//*************************************************************************
//***
//*
//*
//******
#[no_mangle]
pub unsafe extern "C" fn iV_SurfaceCreate(mut flags: uint32,
                                          mut width: libc::c_int,
                                          mut height: libc::c_int,
                                          mut xp: libc::c_int,
                                          mut yp: libc::c_int,
                                          mut buffer: *mut uint8)
 -> *mut iSurface {
    let mut s: *mut iSurface = 0 as *mut iSurface;
    let mut i: libc::c_int = 0;
    // on playstation this MUST be null
    s =
        memMallocRelease(::std::mem::size_of::<iSurface>() as libc::c_ulong)
            as *mut iSurface;
    if s.is_null() { return 0 as *mut iSurface }
    (*s).flags = flags;
    (*s).xcentre = width >> 1 as libc::c_int;
    (*s).ycentre = height >> 1 as libc::c_int;
    (*s).xpshift = xp;
    (*s).ypshift = yp;
    (*s).width = width;
    (*s).height = height;
    (*s).size = width * height;
    (*s).buffer = buffer;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        (*s).scantable[i as usize] = i * width;
        i += 1
    }
    (*s).clip.left = 0 as libc::c_int;
    (*s).clip.right = width - 1 as libc::c_int;
    (*s).clip.top = 0 as libc::c_int;
    (*s).clip.bottom = height - 1 as libc::c_int;
    return s;
}
// user must free s->buffer before calling
#[no_mangle]
pub unsafe extern "C" fn iV_SurfaceDestroy(mut s: *mut iSurface) {
    // if renderer assigned to surface
    if psRendSurface == s { psRendSurface = 0 as *mut iSurface }
    if !s.is_null() {
        memFreeRelease(s as *mut libc::c_void);
        s = 0 as *mut iSurface
    };
}
//*************************************************************************
//*** assign renderer
//*
//******
#[no_mangle]
pub unsafe extern "C" fn rend_Assign(mut s: *mut iSurface) {
    iV_RenderAssign(s);
    /* Need to look into this - won't the unwanted called still set render surface? */
    psRendSurface = s;
}
// pre VideoOpen
#[no_mangle]
pub unsafe extern "C" fn rend_AssignScreen() {
    iV_RenderAssign(&mut rendSurface);
}
#[no_mangle]
pub unsafe extern "C" fn iV_GetDisplayWidth() -> libc::c_int {
    return rendSurface.width;
}
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_GetDisplayHeight() -> libc::c_int {
    return rendSurface.height;
}
//*************************************************************************
//
// function pointers for render assign
//
//*************************************************************************
//void (*pie_VideoShutDown)(void);
//void (*iV_Clear)(uint32 colour);
//void (*iV_RenderEnd)(void);
//void (*iV_RenderBegin)(void);
//void (*iV_Palette)(int i, int r, int g, int b);
//void (*pie_Draw3DShape)(iIMDShape *shape, int frame, int team, UDWORD colour, UDWORD specular, int pieFlag, int pieData);
#[no_mangle]
pub static mut iV_pLine:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: uint32) -> ()> =
    None;
//void (*iV_Line)(int x0, int y0, int x1, int y1, uint32 colour);
//void (*iV_Polygon)(int npoints, iVertex *vrt, iTexture *tex, uint32 type);
//void (*iV_Triangle)(iVertex *vrt, iTexture *tex, uint32 type);
//void (*iV_TransPolygon)(int num, iVertex *vrt);
#[no_mangle]
pub static mut iV_TransTriangle:
           Option<unsafe extern "C" fn(_: *mut iVertex) -> ()> =
    None;
//void (*iV_Box)(int x0,int y0, int x1, int y1, uint32 colour);
//void (*iV_BoxFill)(int x0, int y0, int x1, int y1, uint32 colour);
//void (*iV_DownloadDisplayBuffer)(char *DisplayBuffer);
//void (*pie_DownLoadRadar)(char *buffer);
//void (*iV_TransBoxFill)(SDWORD x0, SDWORD y0, SDWORD x1, SDWORD y1);
//void (*iV_UniTransBoxFill)(SDWORD x0,SDWORD y0, SDWORD x1, SDWORD y1, UDWORD rgb, UDWORD transparency);
//void (*iV_DrawImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//void (*iV_DrawImageRect)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,int x0,int y0,int Width,int Height);
//void (*iV_DrawTransImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//void (*iV_DrawTransImageRect)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,int x0,int y0,int Width,int Height);
//void (*iV_DrawSemiTransImageDef)(IMAGEDEF *Image,iBitmap *Bmp,UDWORD Modulus,int x,int y,int TransRate);
//void (*iV_DrawStretchImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,int Width,int Height);
#[no_mangle]
pub static mut iV_ppBitmap:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int) -> ()>
           =
    None;
#[no_mangle]
pub static mut iV_ppBitmapTrans:
           Option<unsafe extern "C" fn(_: *mut iBitmap, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int) -> ()>
           =
    None;
#[no_mangle]
pub static mut iV_SetTransFilter:
           Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> ()> =
    None;
//void (*iV_DrawColourTransImage)(IMAGEFILE *ImageFile,UWORD ID,int x,int y,UWORD ColourIndex);
#[no_mangle]
pub static mut iV_UniBitmapDepth:
           Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_int,
                                       _: libc::c_int, _: libc::c_uchar,
                                       _: libc::c_int) -> ()> =
    None;
//void (*iV_SetGammaValue)(float val);
//void (*iV_SetFogStatus)(BOOL val);
//void (*iV_SetFogTable)(UDWORD color, UDWORD zMin, UDWORD zMax);
#[no_mangle]
pub static mut iV_SetTransImds: Option<unsafe extern "C" fn(_: BOOL) -> ()> =
    None;
//mapdisplay
#[no_mangle]
pub static mut iV_tgTriangle:
           Option<unsafe extern "C" fn(_: *mut iVertex, _: *mut iTexture)
                      -> ()> =
    None;
#[no_mangle]
pub static mut iV_tgPolygon:
           Option<unsafe extern "C" fn(_: libc::c_int, _: *mut iVertex,
                                       _: *mut iTexture) -> ()> =
    None;
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
//*************************************************************************
//void (*iV_DrawImageDef)(IMAGEDEF *Image,iBitmap *Bmp,UDWORD Modulus,int x,int y);
//design
//void (*iV_UploadDisplayBuffer)(UBYTE *DisplayBuffer);
//void (*iV_ScaleBitmapRGB)(UBYTE *DisplayBuffer,int Width,int Height,int ScaleR,int ScaleG,int ScaleB);
//text
//void (*iV_BeginTextRender)(SWORD ColourIndex);
//void (*iV_TextRender)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//void (*iV_TextRender270)(IMAGEFILE *ImageFile,UWORD ID,int x,int y);
//*************************************************************************
//
// function pointers for render assign
//
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_RenderAssign(mut s: *mut iSurface) {
    /* Need to look into this - won't the unwanted called still set render surface? */
    psRendSurface = s;
    iV_SetTransFilter =
        Some(SetTransFilter as
                 unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> ());
}
// don't want this function at all if we have PIETOOL defined
