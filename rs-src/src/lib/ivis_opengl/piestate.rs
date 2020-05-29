use ::libc;
extern "C" {
    #[no_mangle]
    fn glFogfv(pname: GLenum, params: *const GLfloat);
    #[no_mangle]
    fn glFogi(pname: GLenum, param: GLint);
    #[no_mangle]
    fn glFogf(pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn glBindTexture(target: GLenum, texture: GLuint);
    #[no_mangle]
    fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte,
                  alpha: GLubyte);
    #[no_mangle]
    fn glDepthMask(flag: GLboolean);
    #[no_mangle]
    fn glDepthFunc(func: GLenum);
    #[no_mangle]
    fn glHint(target: GLenum, mode: GLenum);
    #[no_mangle]
    fn glDisable(cap: GLenum);
    #[no_mangle]
    fn glEnable(cap: GLenum);
    #[no_mangle]
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    #[no_mangle]
    fn glAlphaFunc(func: GLenum, ref_0: GLclampf);
    #[no_mangle]
    static mut pieStateCount: SDWORD;
    #[no_mangle]
    fn pie_GetFogColour() -> UDWORD;
    #[no_mangle]
    static mut _TEX_PAGE: [iTexPage; 64];
    /* **************************************************************************/
/*
 * pieState.c
 *
 * renderer setup and state control routines for 3D rendering
 *
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut rendStates: RENDER_STATE;
}
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLfloat = libc::c_float;
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
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type uint8 = libc::c_uchar;
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
/* **************************************************************************/
/*
 * piedef.h
 *
 * type defines for all pumpkin image library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions (CONSTANTS)
 */
/* **************************************************************************/
//stretchs z range for (1000 to 4000) to (8000 to 32000)
//raised to 32000 from 6000 when stretched
//1/32000
//will be stretched to 16000
//Effects
//Render style flags for all pie draw functions
/* **************************************************************************/
/*
 *	Global Definitions (MACROS)
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions (STRUCTURES)
 */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIELIGHTBYTES {
    pub b: UBYTE,
    pub g: UBYTE,
    pub r: UBYTE,
    pub a: UBYTE,
}
//for byte fields in a DWORD
#[derive(Copy, Clone)]
#[repr(C)]
pub union PIELIGHT {
    pub byte: PIELIGHTBYTES,
    pub argb: UDWORD,
}
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
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
pub type TRANSLUCENCY_MODE = libc::c_uint;
pub const TRANS_ADDITIVE: TRANSLUCENCY_MODE = 4;
pub const TRANS_ALPHA: TRANSLUCENCY_MODE = 3;
pub const TRANS_FILTER: TRANSLUCENCY_MODE = 2;
pub const TRANS_DECAL_FOG: TRANSLUCENCY_MODE = 1;
pub const TRANS_DECAL: TRANSLUCENCY_MODE = 0;
pub type FOG_CAP = libc::c_uint;
pub const FOG_CAP_UNDEFINED: FOG_CAP = 3;
pub const FOG_CAP_COLOURED: FOG_CAP = 2;
pub const FOG_CAP_GREY: FOG_CAP = 1;
pub const FOG_CAP_NO: FOG_CAP = 0;
pub type COLOUR_MODE = libc::c_uint;
pub const COLOUR_TEX_CONSTANT: COLOUR_MODE = 3;
pub const COLOUR_TEX_ITERATED: COLOUR_MODE = 2;
pub const COLOUR_FLAT_ITERATED: COLOUR_MODE = 1;
pub const COLOUR_FLAT_CONSTANT: COLOUR_MODE = 0;
pub type TEX_MODE = libc::c_uint;
pub const TEX_NONE: TEX_MODE = 1;
pub const TEX_LOCAL: TEX_MODE = 0;
pub type ALPHA_MODE = libc::c_uint;
pub const ALPHA_CONSTANT: ALPHA_MODE = 1;
pub const ALPHA_ITERATED: ALPHA_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDER_STATE {
    pub depthBuffer: DEPTH_MODE,
    pub translucent: BOOL,
    pub additive: BOOL,
    pub fogCap: FOG_CAP,
    pub fogEnabled: BOOL,
    pub fog: BOOL,
    pub fogColour: UDWORD,
    pub texPage: SDWORD,
    pub rendMode: REND_MODE,
    pub bilinearOn: BOOL,
    pub keyingOn: BOOL,
    pub colourCombine: COLOUR_MODE,
    pub texCombine: TEX_MODE,
    pub alphaCombine: ALPHA_MODE,
    pub transMode: TRANSLUCENCY_MODE,
    pub colour: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexPage {
    pub tex: iTexture,
    pub type_0: uint8,
    pub name: [libc::c_char; 80],
    pub textPage3dfx: libc::c_uint,
    pub bResource: libc::c_int,
}
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetDepthBufferStatus(mut depthMode: DEPTH_MODE) {
    match depthMode as libc::c_uint {
        0 => {
            glEnable(0xb71 as libc::c_int as GLenum);
            glDepthFunc(0x203 as libc::c_int as GLenum);
            glDepthMask(1 as libc::c_int as GLboolean);
        }
        1 => {
            glDisable(0xb71 as libc::c_int as GLenum);
            glDepthMask(1 as libc::c_int as GLboolean);
        }
        2 => {
            glEnable(0xb71 as libc::c_int as GLenum);
            glDepthFunc(0x203 as libc::c_int as GLenum);
            glDepthMask(0 as libc::c_int as GLboolean);
        }
        3 => {
            glDisable(0xb71 as libc::c_int as GLenum);
            glDepthMask(0 as libc::c_int as GLboolean);
        }
        _ => { }
    };
}
//***************************************************************************
//
// pie_SetFogStatus(BOOL val)
//
// Toggle fog on and off for rendering objects inside or outside the 3D world
//
//***************************************************************************
#[no_mangle]
pub unsafe extern "C" fn pie_SetFogStatus(mut val: BOOL) {
    let mut fog: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut fog_colour: [libc::c_float; 4] = [0.; 4];
    if rendStates.fogEnabled != 0 {
        //fog enabled so toggle if required
        if rendStates.fog != val {
            rendStates.fog = val;
            if rendStates.fog != 0 {
                fog.argb = pie_GetFogColour();
                fog_colour[0 as libc::c_int as usize] =
                    (fog.byte.r as libc::c_int as libc::c_double / 255.0f64)
                        as libc::c_float;
                fog_colour[1 as libc::c_int as usize] =
                    (fog.byte.g as libc::c_int as libc::c_double / 255.0f64)
                        as libc::c_float;
                fog_colour[2 as libc::c_int as usize] =
                    (fog.byte.b as libc::c_int as libc::c_double / 255.0f64)
                        as libc::c_float;
                fog_colour[3 as libc::c_int as usize] =
                    (fog.byte.a as libc::c_int as libc::c_double / 255.0f64)
                        as libc::c_float;
                glFogi(0xb65 as libc::c_int as GLenum, 0x2601 as libc::c_int);
                glFogfv(0xb66 as libc::c_int as GLenum,
                        fog_colour.as_mut_ptr());
                glFogf(0xb62 as libc::c_int as GLenum, 0.35f32);
                glHint(0xc54 as libc::c_int as GLenum,
                       0x1100 as libc::c_int as GLenum);
                glFogf(0xb63 as libc::c_int as GLenum, 5000.0f32);
                glFogf(0xb64 as libc::c_int as GLenum, 7000.0f32);
                glEnable(0xb60 as libc::c_int as GLenum);
            } else { glDisable(0xb60 as libc::c_int as GLenum); }
        }
    } else if rendStates.fog != 0 as libc::c_int {
        rendStates.fog = 0 as libc::c_int
    };
}
//fog disabled so turn it off if not off already
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetTexturePage(mut num: SDWORD) {
    if num != rendStates.texPage {
        rendStates.texPage = num;
        if num < 0 as libc::c_int {
            glDisable(0xde1 as libc::c_int as GLenum);
        } else {
            glEnable(0xde1 as libc::c_int as GLenum);
            glBindTexture(0xde1 as libc::c_int as GLenum,
                          _TEX_PAGE[num as usize].textPage3dfx);
        }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetColourKeyedBlack(mut keyingOn: BOOL) {
    if keyingOn != rendStates.keyingOn {
        rendStates.keyingOn = keyingOn;
        pieStateCount += 1;
        if keyingOn == 1 as libc::c_int {
            glEnable(0xbc0 as libc::c_int as GLenum);
            glAlphaFunc(0x204 as libc::c_int as GLenum, 0.1f64 as GLclampf);
        } else { glDisable(0xbc0 as libc::c_int as GLenum); }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetColourCombine(mut colCombMode: COLOUR_MODE) {
    //ffs
    if colCombMode as libc::c_uint != rendStates.colourCombine as libc::c_uint
       {
        rendStates.colourCombine = colCombMode;
        pieStateCount += 1;
        match colCombMode as libc::c_uint {
            0 | 1 => { pie_SetTexturePage(-(1 as libc::c_int)); }
            3 | 2 | _ => { }
        }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetTranslucencyMode(mut transMode:
                                                     TRANSLUCENCY_MODE) {
    if transMode as libc::c_uint != rendStates.transMode as libc::c_uint {
        rendStates.transMode = transMode;
        match transMode as libc::c_uint {
            3 => {
                glEnable(0xbe2 as libc::c_int as GLenum);
                glBlendFunc(0x302 as libc::c_int as GLenum,
                            0x303 as libc::c_int as GLenum);
            }
            4 => {
                glEnable(0xbe2 as libc::c_int as GLenum);
                glBlendFunc(0x302 as libc::c_int as GLenum,
                            1 as libc::c_int as GLenum);
            }
            2 => {
                glEnable(0xbe2 as libc::c_int as GLenum);
                glBlendFunc(0x302 as libc::c_int as GLenum,
                            0x300 as libc::c_int as GLenum);
            }
            _ => {
                rendStates.transMode = TRANS_DECAL;
                glDisable(0xbe2 as libc::c_int as GLenum);
            }
        }
    };
}
/* **************************************************************************/
// set the constant colour used in text and flat render modes
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetColour(mut colour: UDWORD) {
    if colour != rendStates.colour {
        let mut c: PIELIGHT =
            PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
        rendStates.colour = colour;
        pieStateCount += 1;
        c.argb = colour;
        glColor4ub(c.byte.r, c.byte.g, c.byte.b, c.byte.a);
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_SetGammaValue(mut val: libc::c_float) 
 // FIXME Remove if unused
 {
}
