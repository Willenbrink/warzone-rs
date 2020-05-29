use ::libc;
extern "C" {
    #[no_mangle]
    fn SDL_GL_SwapBuffers();
    #[no_mangle]
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf,
                    alpha: GLclampf);
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn glDepthMask(flag: GLboolean);
    //*************************************************************************
    #[no_mangle]
    fn pal_Init();
    #[no_mangle]
    fn pie_CleanUp();
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_SetDefaultStates();
    #[no_mangle]
    fn pie_GetFogColour() -> UDWORD;
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    fn pie_MatInit();
    #[no_mangle]
    fn pie_InitMaths();
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    // Was page provided by resource handler?
    //*************************************************************************
    #[no_mangle]
    static mut _TEX_INDEX: libc::c_int;
    //*************************************************************************
    #[no_mangle]
    fn pie_TexInit();
    #[no_mangle]
    static mut rendSurface: iSurface;
    #[no_mangle]
    fn iV_RenderAssign(s: *mut iSurface);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn screen_GetBackDrop() -> BOOL;
    #[no_mangle]
    fn screen_Upload(newBackDropBmp: *mut UWORD);
    /* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn screenDoDumpToDiskIfRequired();
}
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
pub type GLclampf = libc::c_float;
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
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
// Routines to provide simple maths functions that work on both PSX & PC
// Use the type "FRACT" instead of FLOAT
//  - This is defined as a float on PC and a 20.12 fixed point number on PSX
//
//  Use:-
//		MAKEFRACT(int);  to convert from a SDWORD to a FRACT
//		MAKEINT(fract);	to convert the other way
//		FRACTmul(fract,fract); to multiply two fract numbers
//		FRACTdiv(fract,fract); to divide two numbers
//		SQRT(fract);		to get square root of a fract (returns a fract)
//      iSQRT(int);			to get a square root of an integer (returns an UDWORD)
//      FRACTCONST(constA,constB);	; Generates a constant of (constA/constB)
//                         e.g. to define 0.5 use FRACTCONST(1,2)
//                              to define 0.114 use FRACTCONT(114,1000)
//
// Also PERCENT(int,int);	// returns a int value 0->100 of the percentage of the first param over the second
//
// This file used to be in the deliverance src directory. But Jeremy quite correctly
// pointed out to me that it should be library based not deliverance based, and hence
// has now been moved to the lib\framework directory
//
// If you are reading this file from the deliverance source directory, please delete it now
// To multiply a FRACT by a integer just use the normal operator 
//   e.g.   FractValue2=FractValue*Interger;
//
// save is true of divide
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
pub type FRACT = libc::c_float;
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
pub type CLEAR_MODE = libc::c_uint;
pub const CLEAR_FOG: CLEAR_MODE = 3;
pub const CLEAR_BLACK: CLEAR_MODE = 2;
pub const CLEAR_OFF_AND_NO_BUFFER_DOWNLOAD: CLEAR_MODE = 1;
pub const CLEAR_OFF: CLEAR_MODE = 0;
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
#[no_mangle]
pub static mut _iVPRIM_DIVTABLE: [int32; 1024] = [0; 1024];
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
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_Initialise() -> BOOL {
    let mut i: libc::c_int = 0;
    pie_InitMaths();
    pie_TexInit();
    rendSurface.usr = -(1 as libc::c_int);
    rendSurface.flags = 0 as libc::c_int as uint32;
    rendSurface.buffer = 0 as *mut uint8;
    rendSurface.size = 0 as libc::c_int;
    // divtable: first entry == unity to (ie n/0 == 1 !)
    _iVPRIM_DIVTABLE[0 as libc::c_int as usize] =
        (1 as libc::c_int) << 15 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 1024 as libc::c_int {
        _iVPRIM_DIVTABLE[(i - 0 as libc::c_int) as usize] =
            (1 as libc::c_int as FRACT / i as FRACT *
                 ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_float)
                as SDWORD;
        i += 1
    }
    pie_MatInit();
    _TEX_INDEX = 0 as libc::c_int;
    rendSurface.buffer = 0 as *mut uint8;
    rendSurface.flags = 1 as libc::c_int as uint32;
    rendSurface.width = pie_GetVideoBufferWidth() as libc::c_int;
    rendSurface.height = pie_GetVideoBufferHeight() as libc::c_int;
    rendSurface.xcentre =
        pie_GetVideoBufferWidth().wrapping_div(2 as libc::c_int as
                                                   libc::c_uint) as
            libc::c_int;
    rendSurface.ycentre =
        pie_GetVideoBufferHeight().wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as
            libc::c_int;
    rendSurface.clip.left = 0 as libc::c_int;
    rendSurface.clip.top = 0 as libc::c_int;
    rendSurface.clip.right = pie_GetVideoBufferWidth() as libc::c_int;
    rendSurface.clip.bottom = pie_GetVideoBufferHeight() as libc::c_int;
    rendSurface.xpshift = 10 as libc::c_int;
    rendSurface.ypshift = 10 as libc::c_int;
    pie_SetDefaultStates();
    iV_RenderAssign(&mut rendSurface);
    pal_Init();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_ShutDown() {
    rendSurface.buffer = 0 as *mut uint8;
    rendSurface.flags = 0 as libc::c_int as uint32;
    rendSurface.usr = -(1 as libc::c_int);
    rendSurface.size = 0 as libc::c_int;
    pie_CleanUp();
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_ScreenFlip(mut clearMode: CLEAR_MODE) {
    let mut fog_colour: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    screenDoDumpToDiskIfRequired();
    SDL_GL_SwapBuffers();
    match clearMode as libc::c_uint {
        1 => { }
        2 | _ => {
            glDepthMask(1 as libc::c_int as GLboolean);
            fog_colour.argb = pie_GetFogColour();
            glClearColor((fog_colour.byte.r as libc::c_int as libc::c_double /
                              255.0f64) as GLclampf,
                         (fog_colour.byte.g as libc::c_int as libc::c_double /
                              255.0f64) as GLclampf,
                         (fog_colour.byte.b as libc::c_int as libc::c_double /
                              255.0f64) as GLclampf,
                         (fog_colour.byte.a as libc::c_int as libc::c_double /
                              255.0f64) as GLclampf);
            glClear((0x4000 as libc::c_int | 0x100 as libc::c_int) as
                        GLbitfield);
        }
    }
    if screen_GetBackDrop() != 0 { screen_Upload(0 as *mut UWORD); };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_Clear(mut colour: UDWORD) { }
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_GlobalRenderBegin() { }
#[no_mangle]
pub unsafe extern "C" fn pie_GlobalRenderEnd(mut bForceClearToBlack: BOOL) { }
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_GetResScalingFactor() -> UDWORD {
    //	UDWORD	resWidth;	//n.b. resolution width implies resolution height...!
    if pie_GetVideoBufferWidth().wrapping_mul(4 as libc::c_int as
                                                  libc::c_uint) >
           pie_GetVideoBufferHeight().wrapping_mul(5 as libc::c_int as
                                                       libc::c_uint) {
        return pie_GetVideoBufferHeight().wrapping_mul(5 as libc::c_int as
                                                           libc::c_uint).wrapping_div(4
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint).wrapping_div(6
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint)
    } else {
        return pie_GetVideoBufferWidth().wrapping_div(6 as libc::c_int as
                                                          libc::c_uint)
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_LocalRenderBegin() { }
#[no_mangle]
pub unsafe extern "C" fn pie_LocalRenderEnd() { }
#[no_mangle]
pub unsafe extern "C" fn pie_RenderSetup() { }
