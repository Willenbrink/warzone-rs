use ::libc;
extern "C" {
    /* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
    /*
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Compare N characters of S1 and S2.  */
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    /* Return the length of the initial segment of S which
   consists entirely of characters not in REJECT.  */
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_uint;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn glClear(mask: GLbitfield);
    #[no_mangle]
    fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean,
                   alpha: GLboolean);
    #[no_mangle]
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    #[no_mangle]
    fn glCullFace(mode: GLenum);
    #[no_mangle]
    fn glEnable(cap: GLenum);
    #[no_mangle]
    fn glDisable(cap: GLenum);
    #[no_mangle]
    fn glGetFloatv(pname: GLenum, params: *mut GLfloat);
    #[no_mangle]
    fn glGetString(name: GLenum) -> *const GLubyte;
    #[no_mangle]
    fn glDepthFunc(func: GLenum);
    #[no_mangle]
    fn glDepthMask(flag: GLboolean);
    #[no_mangle]
    fn glPushMatrix();
    #[no_mangle]
    fn glPopMatrix();
    #[no_mangle]
    fn glLoadIdentity();
    #[no_mangle]
    fn glMultMatrixf(m: *const GLfloat);
    #[no_mangle]
    fn glGenLists(range: GLsizei) -> GLuint;
    #[no_mangle]
    fn glNewList(list: GLuint, mode: GLenum);
    #[no_mangle]
    fn glEndList();
    #[no_mangle]
    fn glCallList(list: GLuint);
    #[no_mangle]
    fn glBegin(mode: GLenum);
    #[no_mangle]
    fn glEnd();
    #[no_mangle]
    fn glVertex2f(x: GLfloat, y: GLfloat);
    #[no_mangle]
    fn glVertex2i(x: GLint, y: GLint);
    #[no_mangle]
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glNormal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat);
    #[no_mangle]
    fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte);
    #[no_mangle]
    fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    #[no_mangle]
    fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte,
                  alpha: GLubyte);
    #[no_mangle]
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    #[no_mangle]
    fn glLightfv(light: GLenum, pname: GLenum, params: *const GLfloat);
    #[no_mangle]
    fn glGetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat);
    #[no_mangle]
    fn glLightModeli(pname: GLenum, param: GLint);
    #[no_mangle]
    fn glLightModelfv(pname: GLenum, params: *const GLfloat);
    #[no_mangle]
    fn glMaterialf(face: GLenum, pname: GLenum, param: GLfloat);
    #[no_mangle]
    fn glMaterialfv(face: GLenum, pname: GLenum, params: *const GLfloat);
    #[no_mangle]
    fn glStencilFunc(func: GLenum, ref_0: GLint, mask: GLuint);
    #[no_mangle]
    fn glStencilMask(mask: GLuint);
    #[no_mangle]
    fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
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
    /* Use misc.  */
    /* Use misc or X/Open.  */
    /* Allocate SIZE bytes of memory.  */
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    /* Re-allocate the previously allocated block
   in PTR, making the new block SIZE bytes long.  */
/* __attribute_malloc__ is not used, because if realloc returns
   the same pointer that was passed to it, aliasing needs to be allowed
   between objects pointed by the old and new pointers.  */
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_uint) -> *mut libc::c_void;
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* *
 * Get the address of a GL function
 */
    #[no_mangle]
    fn SDL_GL_GetProcAddress(proc_0: *const libc::c_char)
     -> *mut libc::c_void;
    //*************************************************************************
    #[no_mangle]
    fn pie_PerspectiveBegin();
    #[no_mangle]
    fn pie_PerspectiveEnd();
    #[no_mangle]
    static mut _TEX_PAGE: [iTexPage; 64];
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut pieStateCount: SDWORD;
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
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
    #[no_mangle]
    fn pie_GetColour() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    /* **************************************************************************/
/*
 * piedraw.c
 *
 * updated render routines for 3D coloured shaded transparency rendering
 *
 */
/* **************************************************************************/
    #[no_mangle]
    static mut drawing_interface: BOOL;
    #[no_mangle]
    static mut BSPimd: *mut iIMDShape;
}
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLbitfield = libc::c_uint;
pub type GLint = libc::c_int;
pub type GLubyte = libc::c_uchar;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type PFNGLACTIVESTENCILFACEEXTPROC
    =
    Option<unsafe extern "C" fn(_: GLenum) -> ()>;
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
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
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
pub struct iPoint {
    pub x: int32,
    pub y: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
pub type BSPPOLYID = uint16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
// currently uses 4k per structure (!)
//*************************************************************************
//
// texture animation structures
//
//*************************************************************************
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDPoly {
    pub flags: uint32,
    pub zcentre: int32,
    pub npnts: libc::c_int,
    pub normal: iVector,
    pub pindex: *mut VERTEXID,
    pub vrt: *mut iVertex,
    pub pTexAnim: *mut iTexAnim,
    pub BSP_NextPoly: BSPPOLYID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDShape {
    pub flags: uint32,
    pub texpage: int32,
    pub oradius: int32,
    pub sradius: int32,
    pub radius: int32,
    pub visRadius: int32,
    pub xmin: int32,
    pub xmax: int32,
    pub ymin: int32,
    pub ymax: int32,
    pub zmin: int32,
    pub zmax: int32,
    pub ocen: iVector,
    pub numFrames: UWORD,
    pub animInterval: UWORD,
    pub npoints: libc::c_int,
    pub npolys: libc::c_int,
    pub nconnectors: libc::c_int,
    pub points: *mut iVector,
    pub polys: *mut iIMDPoly,
    pub connectors: *mut iVector,
    pub ntexanims: libc::c_int,
    pub texanims: *mut *mut iTexAnim,
    pub next: *mut iIMDShape,
    pub BSPNode: PSBSPTREENODE,
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
pub struct PIEVERTEX {
    pub sx: SDWORD,
    pub sy: SDWORD,
    pub sz: SDWORD,
    pub tu: UWORD,
    pub tv: UWORD,
    pub light: PIELIGHT,
    pub specular: PIELIGHT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEPIXEL {
    pub d3dx: libc::c_float,
    pub d3dy: libc::c_float,
    pub d3dz: libc::c_float,
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
//for byte fields in a DWORD
//screen rectangle
//an area of texture
//render style for pie draw functions
// This is the new resource loaded structure (TEXPAGE)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIEPOLY {
    pub flags: UDWORD,
    pub nVrts: SDWORD,
    pub pVrts: *mut PIEVERTEX,
    pub pTexAnim: *mut iTexAnim,
}
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fVector {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
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
pub struct shadowcasting_shape_t {
    pub matrix: [libc::c_float; 16],
    pub shape: *mut iIMDShape,
    pub flag: libc::c_int,
    pub flag_data: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transluscent_shape_t {
    pub matrix: [libc::c_float; 16],
    pub shape: *mut iIMDShape,
    pub frame: libc::c_int,
    pub colour: PIELIGHT,
    pub specular: PIELIGHT,
    pub flag: libc::c_int,
    pub flag_data: libc::c_int,
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
 *	OpenGL extensions for shadows
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn check_extension(mut extension_name:
                                             *const libc::c_char) -> BOOL {
    let mut extension_list: *mut libc::c_char =
        glGetString(0x1f03 as libc::c_int as GLenum) as *mut libc::c_char;
    let mut extension_name_length: libc::c_uint = strlen(extension_name);
    let mut tmp: *mut libc::c_char = extension_list;
    let mut first_extension_length: libc::c_uint = 0;
    if extension_name.is_null() || extension_list.is_null() {
        return 0 as libc::c_int
    }
    while *tmp.offset(0 as libc::c_int as isize) != 0 {
        first_extension_length =
            strcspn(tmp, b" \x00" as *const u8 as *const libc::c_char);
        if extension_name_length == first_extension_length &&
               strncmp(extension_name, tmp, first_extension_length) ==
                   0 as libc::c_int {
            debug(LOG_3D,
                  b"%s is supported.\n\x00" as *const u8 as
                      *const libc::c_char, extension_name);
            return 1 as libc::c_int
        }
        tmp =
            tmp.offset(first_extension_length.wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                           as isize)
    }
    debug(LOG_3D,
          b"%s is not supported.\n\x00" as *const u8 as *const libc::c_char,
          extension_name);
    return 0 as libc::c_int;
}
// EXT_stencil_two_side
#[no_mangle]
pub static mut glActiveStencilFaceEXT: PFNGLACTIVESTENCILFACEEXTPROC = None;
#[no_mangle]
pub unsafe extern "C" fn stencil_one_pass() -> BOOL {
    static mut initialised: BOOL = 0 as libc::c_int;
    static mut return_value: BOOL = 0;
    if initialised == 0 {
        return_value =
            (check_extension(b"GL_EXT_stencil_two_side\x00" as *const u8 as
                                 *const libc::c_char) != 0 &&
                 check_extension(b"GL_EXT_stencil_wrap\x00" as *const u8 as
                                     *const libc::c_char) != 0) as
                libc::c_int;
        glActiveStencilFaceEXT =
            ::std::mem::transmute::<*mut libc::c_void,
                                    PFNGLACTIVESTENCILFACEEXTPROC>(SDL_GL_GetProcAddress(b"glActiveStencilFaceEXT\x00"
                                                                                             as
                                                                                             *const u8
                                                                                             as
                                                                                             *const libc::c_char));
        initialised = 1 as libc::c_int
    }
    return return_value;
}
/* **************************************************************************/
/*
 *	Local Definitions
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Local Variables
 */
/* **************************************************************************/
static mut scrPoints: [PIEPIXEL; 512] =
    [PIEPIXEL{d3dx: 0., d3dy: 0., d3dz: 0.,}; 512];
static mut pieVrts: [PIEVERTEX; 10] =
    [PIEVERTEX{sx: 0,
               sy: 0,
               sz: 0,
               tu: 0,
               tv: 0,
               light: PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
               specular:
                   PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},};
        10];
static mut imdVrts: [iVertex; 10] =
    [iVertex{x: 0, y: 0, z: 0, u: 0, v: 0, g: 0,}; 10];
static mut pieCount: SDWORD = 0 as libc::c_int;
static mut tileCount: SDWORD = 0 as libc::c_int;
static mut polyCount: SDWORD = 0 as libc::c_int;
/* **************************************************************************/
/*
 *	Source
 */
/* **************************************************************************/
static mut lighting: BOOL = 0 as libc::c_int;
static mut shadows: BOOL = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn pie_BeginLighting(mut x: libc::c_float,
                                           mut y: libc::c_float,
                                           mut z: libc::c_float) {
    let mut pos: [libc::c_float; 4] = [0.; 4];
    let mut zero: [libc::c_float; 4] =
        [0 as libc::c_int as libc::c_float, 0 as libc::c_int as libc::c_float,
         0 as libc::c_int as libc::c_float,
         0 as libc::c_int as libc::c_float];
    let mut ambient: [libc::c_float; 4] =
        [0.2f64 as libc::c_float, 0.2f64 as libc::c_float,
         0.2f64 as libc::c_float, 0 as libc::c_int as libc::c_float];
    let mut diffuse: [libc::c_float; 4] =
        [0.5f64 as libc::c_float, 0.5f64 as libc::c_float,
         0.5f64 as libc::c_float, 0 as libc::c_int as libc::c_float];
    let mut specular: [libc::c_float; 4] =
        [1 as libc::c_int as libc::c_float, 1 as libc::c_int as libc::c_float,
         1 as libc::c_int as libc::c_float,
         0 as libc::c_int as libc::c_float];
    pos[0 as libc::c_int as usize] = x;
    pos[1 as libc::c_int as usize] = y;
    pos[2 as libc::c_int as usize] = z;
    pos[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    glLightModelfv(0xb53 as libc::c_int as GLenum, zero.as_mut_ptr());
    glLightModeli(0xb51 as libc::c_int as GLenum, 0 as libc::c_int);
    glLightfv(0x4000 as libc::c_int as GLenum,
              0x1203 as libc::c_int as GLenum, pos.as_mut_ptr());
    glLightfv(0x4000 as libc::c_int as GLenum,
              0x1200 as libc::c_int as GLenum, ambient.as_mut_ptr());
    glLightfv(0x4000 as libc::c_int as GLenum,
              0x1201 as libc::c_int as GLenum, diffuse.as_mut_ptr());
    glLightfv(0x4000 as libc::c_int as GLenum,
              0x1202 as libc::c_int as GLenum, specular.as_mut_ptr());
    glEnable(0x4000 as libc::c_int as GLenum);
    //lighting = TRUE;
    shadows = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pie_EndLighting() {
    shadows = 0 as libc::c_int;
    lighting = 0 as libc::c_int;
}
#[no_mangle]
pub static mut ShapeTexPage: UDWORD = 0;
#[no_mangle]
pub static mut ShapeFrame: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn DrawTriangleList(mut PolygonNumber: BSPPOLYID) {
    let mut pPolys: *mut iIMDPoly = 0 as *mut iIMDPoly;
    let mut n: UDWORD = 0;
    let mut index: *mut VERTEXID = 0 as *mut VERTEXID;
    let mut imdPoly: iIMDPoly =
        iIMDPoly{flags: 0,
                 zcentre: 0,
                 npnts: 0,
                 normal: iVector{x: 0, y: 0, z: 0,},
                 pindex: 0 as *mut VERTEXID,
                 vrt: 0 as *mut iVertex,
                 pTexAnim: 0 as *mut iTexAnim,
                 BSP_NextPoly: 0,};
}
// BSP object position
static mut BSPObject: iVector = iVector{x: 0, y: 0, z: 0,};
static mut BSPCamera: iVector = iVector{x: 0, y: 0, z: 0,};
static mut BSPObject_Yaw: SDWORD = 0 as libc::c_int;
static mut BSPObject_Pitch: SDWORD = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn SetBSPObjectPos(mut x: SDWORD, mut y: SDWORD,
                                         mut z: SDWORD) {
    BSPObject.x = x;
    BSPObject.y = y;
    BSPObject.z = z;
    // Reset the yaw & pitch
    // these values must be set every time they are used ...
    BSPObject_Yaw = 0 as libc::c_int;
    BSPObject_Pitch = 0 as libc::c_int;
}
// This MUST be called after SetBSPObjectPos ...
#[no_mangle]
pub unsafe extern "C" fn SetBSPObjectRot(mut Yaw: SDWORD, mut Pitch: SDWORD) {
    BSPObject_Yaw = Yaw;
    BSPObject_Pitch = Pitch;
}
// This must be called once per frame after the terrainMidX & player.p values have been updated
#[no_mangle]
pub unsafe extern "C" fn SetBSPCameraPos(mut x: SDWORD, mut y: SDWORD,
                                         mut z: SDWORD) {
    BSPCamera.x = x;
    BSPCamera.y = y;
    BSPCamera.z = z;
}
#[no_mangle]
pub unsafe extern "C" fn fVector_Set(mut v: *mut fVector,
                                     mut x: libc::c_float,
                                     mut y: libc::c_float,
                                     mut z: libc::c_float) {
    (*v).x = x;
    (*v).y = y;
    (*v).z = z;
}
#[no_mangle]
pub unsafe extern "C" fn fVector_Sub(mut dest: *mut fVector,
                                     mut op1: *mut fVector,
                                     mut op2: *mut fVector) {
    (*dest).x = (*op1).x - (*op2).x;
    (*dest).y = (*op1).y - (*op2).y;
    (*dest).z = (*op1).z - (*op2).z;
}
#[no_mangle]
pub unsafe extern "C" fn fVector_SP(mut op1: *mut fVector,
                                    mut op2: *mut fVector) -> libc::c_float {
    return (*op1).x * (*op2).x + (*op1).y * (*op2).y + (*op1).z * (*op2).z;
}
#[no_mangle]
pub unsafe extern "C" fn fVector_CP(mut dest: *mut fVector,
                                    mut op1: *mut fVector,
                                    mut op2: *mut fVector) {
    (*dest).x = (*op1).y * (*op2).z - (*op1).z * (*op2).y;
    (*dest).y = (*op1).z * (*op2).x - (*op1).x * (*op2).z;
    (*dest).z = (*op1).x * (*op2).y - (*op1).y * (*op2).x;
}
#[no_mangle]
pub unsafe extern "C" fn fVector_Length2(mut v: *mut fVector)
 -> libc::c_float {
    return (*v).x * (*v).x + (*v).y * (*v).y + (*v).z * (*v).z;
}
#[no_mangle]
pub unsafe extern "C" fn pie_Polygon(mut numVerts: SDWORD,
                                     mut pVrts: *mut PIEVERTEX,
                                     mut texture_offset: FRACT,
                                     mut light: BOOL) {
    let mut i: SDWORD = 0;
    if numVerts < 1 as libc::c_int {
        return
    } else {
        if numVerts == 1 as libc::c_int {
            glBegin(0 as libc::c_int as GLenum);
        } else if numVerts == 2 as libc::c_int {
            glBegin(0x3 as libc::c_int as GLenum);
        } else {
            if light != 0 {
                let mut ambient: [libc::c_float; 4] =
                    [1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float];
                let mut diffuse: [libc::c_float; 4] =
                    [1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float];
                let mut specular: [libc::c_float; 4] =
                    [1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float,
                     1 as libc::c_int as libc::c_float];
                let mut shininess: libc::c_float =
                    10 as libc::c_int as libc::c_float;
                glEnable(0xb50 as libc::c_int as GLenum);
                glEnable(0xba1 as libc::c_int as GLenum);
                glMaterialfv(0x408 as libc::c_int as GLenum,
                             0x1200 as libc::c_int as GLenum,
                             ambient.as_mut_ptr());
                glMaterialfv(0x408 as libc::c_int as GLenum,
                             0x1201 as libc::c_int as GLenum,
                             diffuse.as_mut_ptr());
                glMaterialfv(0x408 as libc::c_int as GLenum,
                             0x1202 as libc::c_int as GLenum,
                             specular.as_mut_ptr());
                glMaterialf(0x408 as libc::c_int as GLenum,
                            0x1601 as libc::c_int as GLenum, shininess);
            }
            glBegin(0x6 as libc::c_int as GLenum);
            if light != 0 {
                let mut p1: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut p2: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut p3: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut v1: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut v2: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut n: fVector = fVector{x: 0., y: 0., z: 0.,};
                let mut l: libc::c_float = 0.;
                fVector_Set(&mut p1,
                            (*pVrts.offset(0 as libc::c_int as isize)).sx as
                                libc::c_float,
                            (*pVrts.offset(0 as libc::c_int as isize)).sy as
                                libc::c_float,
                            (*pVrts.offset(0 as libc::c_int as isize)).sz as
                                libc::c_float);
                fVector_Set(&mut p2,
                            (*pVrts.offset(1 as libc::c_int as isize)).sx as
                                libc::c_float,
                            (*pVrts.offset(1 as libc::c_int as isize)).sy as
                                libc::c_float,
                            (*pVrts.offset(1 as libc::c_int as isize)).sz as
                                libc::c_float);
                fVector_Set(&mut p3,
                            (*pVrts.offset(2 as libc::c_int as isize)).sx as
                                libc::c_float,
                            (*pVrts.offset(2 as libc::c_int as isize)).sy as
                                libc::c_float,
                            (*pVrts.offset(2 as libc::c_int as isize)).sz as
                                libc::c_float);
                fVector_Sub(&mut v1, &mut p3, &mut p1);
                fVector_Sub(&mut v2, &mut p2, &mut p1);
                fVector_CP(&mut n, &mut v1, &mut v2);
                l = 1.0f64 as libc::c_float;
                glNormal3f(n.x * l, n.y * l, n.z * l);
                //printf("%f %f %f\n", n.x*l, n.y*l, n.z*l);
            }
        }
    }
    i = 0 as libc::c_int;
    while i < numVerts {
        //glColor4ub(255, 255, 255, 255);
        glColor4ub((*pVrts.offset(i as isize)).light.byte.r,
                   (*pVrts.offset(i as isize)).light.byte.g,
                   (*pVrts.offset(i as isize)).light.byte.b,
                   (*pVrts.offset(i as isize)).light.byte.a);
        glTexCoord2f((*pVrts.offset(i as isize)).tu as GLfloat,
                     (*pVrts.offset(i as isize)).tv as libc::c_int as
                         libc::c_float + texture_offset);
        //d3dVrts[i].specular = pVrts[i].specular.argb;
        glVertex3f((*pVrts.offset(i as isize)).sx as GLfloat,
                   (*pVrts.offset(i as isize)).sy as GLfloat,
                   (*pVrts.offset(i as isize)).sz as GLfloat);
        i += 1
    }
    glEnd();
    glDisable(0xb50 as libc::c_int as GLenum);
}
static mut scshapes: *mut shadowcasting_shape_t =
    0 as *const shadowcasting_shape_t as *mut shadowcasting_shape_t;
static mut scshapes_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut nb_scshapes: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut tshapes: *mut transluscent_shape_t =
    0 as *const transluscent_shape_t as *mut transluscent_shape_t;
static mut tshapes_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut nb_tshapes: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn pie_Draw3DShape2(mut shape: *mut iIMDShape,
                                          mut frame: libc::c_int,
                                          mut colour: PIELIGHT,
                                          mut specular: PIELIGHT,
                                          mut pieFlag: libc::c_int,
                                          mut pieFlagData: libc::c_int) {
    let mut tempY: int32 = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pVertices: *mut iVector = 0 as *mut iVector;
    let mut pPixels: *mut PIEPIXEL = 0 as *mut PIEPIXEL;
    let mut pPolys: *mut iIMDPoly = 0 as *mut iIMDPoly;
    let mut piePoly: PIEPOLY =
        PIEPOLY{flags: 0,
                nVrts: 0,
                pVrts: 0 as *mut PIEVERTEX,
                pTexAnim: 0 as *mut iTexAnim,};
    let mut index: *mut VERTEXID = 0 as *mut VERTEXID;
    let mut light: BOOL = lighting;
    /* Set tranlucency */
    if pieFlag & 0x4 as libc::c_int != 0 {
        //Assume also translucent
        pie_SetFogStatus(0 as
                             libc::c_int); //never bilinear with constant alpha, gives black edges
        pie_SetRendMode(REND_ADDITIVE_TEX);
        colour.byte.a = pieFlagData as UBYTE;
        pie_SetBilinear(1 as libc::c_int);
        light = 0 as libc::c_int
    } else if pieFlag & 0x2 as libc::c_int != 0 {
        pie_SetFogStatus(0 as libc::c_int);
        pie_SetRendMode(REND_ALPHA_TEX);
        colour.byte.a = pieFlagData as UBYTE;
        pie_SetBilinear(0 as libc::c_int);
        light = 0 as libc::c_int
    } else {
        if pieFlag & 0x40 as libc::c_int != 0 {
            pie_SetFogStatus(0 as libc::c_int);
            pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
        } else { pie_SetFogStatus(1 as libc::c_int); }
        pie_SetRendMode(REND_GOURAUD_TEX);
        //if hardware fog then alpha is set else unused in decal mode
		//colour.byte.a = MAX_UB_LIGHT;
        if pieFlag & 0x8 as libc::c_int != 0 {
            pie_SetBilinear(0 as libc::c_int);
        } else { pie_SetBilinear(1 as libc::c_int); }
    }
    if pieFlag & 0x20 as libc::c_int != 0 {
        pieFlagData =
            (*shape).ymax * (256 as libc::c_int - pieFlagData) /
                256 as libc::c_int
    }
    pie_SetTexturePage((*shape).texpage);
    //now draw the shape
	//rotate and project points from shape->points to scrPoints
    pVertices = (*shape).points;
    pPixels =
        &mut *scrPoints.as_mut_ptr().offset(0 as libc::c_int as isize) as
            *mut PIEPIXEL;
    //--
    i = 0 as libc::c_int;
    while i < (*shape).npoints {
        tempY = (*pVertices).y;
        if pieFlag & 0x20 as libc::c_int != 0 {
            tempY = (*pVertices).y - pieFlagData;
            if tempY < 0 as libc::c_int { tempY = 0 as libc::c_int }
        } else if pieFlag & 0x10 as libc::c_int != 0 {
            if (*pVertices).y > 0 as libc::c_int {
                tempY = (*pVertices).y * pieFlagData / 256 as libc::c_int
            }
            //if (tempY < 0) tempY = 0;
        }
        (*pPixels).d3dx = (*pVertices).x as libc::c_float;
        (*pPixels).d3dy = tempY as libc::c_float;
        (*pPixels).d3dz = (*pVertices).z as libc::c_float;
        i += 1;
        pVertices = pVertices.offset(1);
        pPixels = pPixels.offset(1)
    }
    pPolys = (*shape).polys;
    i = 0 as libc::c_int;
    while i < (*shape).npolys {
        index = (*pPolys).pindex;
        piePoly.flags = (*pPolys).flags;
        if pieFlag & 0x2 as libc::c_int != 0 {
            piePoly.flags |= 0x40000 as libc::c_int as libc::c_uint
        } else if pieFlag & 0x4 as libc::c_int != 0 {
            piePoly.flags &=
                (0xffffffff as
                     libc::c_uint).wrapping_sub(0x800 as libc::c_int as
                                                    libc::c_uint)
            //dont treat additive images as colour keyed
        }
        n = 0 as libc::c_int;
        while n < (*pPolys).npnts {
            pieVrts[n as usize].sx =
                scrPoints[*index as usize].d3dx as SDWORD;
            pieVrts[n as usize].sy =
                scrPoints[*index as usize].d3dy as SDWORD;
            pieVrts[n as usize].sz =
                scrPoints[*index as usize].d3dz as SDWORD;
            pieVrts[n as usize].tu =
                (*(*pPolys).vrt.offset(n as isize)).u as UWORD;
            pieVrts[n as usize].tv =
                (*(*pPolys).vrt.offset(n as isize)).v as UWORD;
            pieVrts[n as usize].light.argb = colour.argb;
            pieVrts[n as usize].specular.argb = specular.argb;
            n += 1;
            index = index.offset(1)
        }
        piePoly.nVrts = (*pPolys).npnts;
        piePoly.pVrts =
            &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
                *mut PIEVERTEX;
        piePoly.pTexAnim = (*pPolys).pTexAnim;
        if piePoly.flags > 0 as libc::c_int as libc::c_uint {
            pie_PiePolyFrame(&mut piePoly, frame, light);
            // draw the polygon ... this is an inline function
        }
        i += 1;
        pPolys = pPolys.offset(1)
    }
    if pieFlag & 0x40 as libc::c_int != 0 {
        pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawShadow(mut shape: *mut iIMDShape,
                                        mut flag: libc::c_int,
                                        mut flag_data: libc::c_int,
                                        mut light: *mut fVector) {
    let mut tempY: int32 = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut pVertices: *mut iVector = 0 as *mut iVector;
    let mut pPixels: *mut PIEPIXEL = 0 as *mut PIEPIXEL;
    let mut pPolys: *mut iIMDPoly = 0 as *mut iIMDPoly;
    let mut index: *mut VERTEXID = 0 as *mut VERTEXID;
    pVertices = (*shape).points;
    pPixels = scrPoints.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < (*shape).npoints {
        tempY = (*pVertices).y;
        if flag & 0x20 as libc::c_int != 0 {
            tempY = (*pVertices).y - flag_data;
            if tempY < 0 as libc::c_int { tempY = 0 as libc::c_int }
        } else if flag & 0x10 as libc::c_int != 0 {
            if (*pVertices).y > 0 as libc::c_int {
                tempY = (*pVertices).y * flag_data / 256 as libc::c_int
            }
        }
        (*pPixels).d3dx = (*pVertices).x as libc::c_float;
        (*pPixels).d3dy = tempY as libc::c_float;
        (*pPixels).d3dz = (*pVertices).z as libc::c_float;
        i += 1;
        pVertices = pVertices.offset(1);
        pPixels = pPixels.offset(1)
    }
    pPolys = (*shape).polys;
    i = 0 as libc::c_int;
    while i < (*shape).npolys {
        let mut p1: fVector = fVector{x: 0., y: 0., z: 0.,};
        let mut p2: fVector = fVector{x: 0., y: 0., z: 0.,};
        let mut p3: fVector = fVector{x: 0., y: 0., z: 0.,};
        let mut v1: fVector = fVector{x: 0., y: 0., z: 0.,};
        let mut v2: fVector = fVector{x: 0., y: 0., z: 0.,};
        let mut normal: fVector = fVector{x: 0., y: 0., z: 0.,};
        index = (*pPolys).pindex;
        fVector_Set(&mut p1, scrPoints[*index as usize].d3dx,
                    scrPoints[*index as usize].d3dy,
                    scrPoints[*index as usize].d3dz);
        index = index.offset(1);
        fVector_Set(&mut p2, scrPoints[*index as usize].d3dx,
                    scrPoints[*index as usize].d3dy,
                    scrPoints[*index as usize].d3dz);
        index = index.offset(1);
        fVector_Set(&mut p3, scrPoints[*index as usize].d3dx,
                    scrPoints[*index as usize].d3dy,
                    scrPoints[*index as usize].d3dz);
        fVector_Sub(&mut v1, &mut p3, &mut p1);
        fVector_Sub(&mut v2, &mut p2, &mut p1);
        fVector_CP(&mut normal, &mut v1, &mut v2);
        if fVector_SP(&mut normal, light) > 0 as libc::c_int as libc::c_float
           {
            if (*pPolys).flags & 0x800 as libc::c_int as libc::c_uint != 0 &&
                   (*pPolys).flags & 0x2000 as libc::c_int as libc::c_uint !=
                       0 {
                let mut i_0: VERTEXID = 0;
                glBegin(0x6 as libc::c_int as GLenum);
                index = (*pPolys).pindex;
                glVertex3f(scrPoints[*index as usize].d3dx,
                           scrPoints[*index as usize].d3dy,
                           scrPoints[*index as usize].d3dz);
                n = (*pPolys).npnts - 1 as libc::c_int;
                while n > 0 as libc::c_int {
                    i_0 = *(*pPolys).pindex.offset(n as isize);
                    glVertex3f(scrPoints[i_0 as usize].d3dx,
                               scrPoints[i_0 as usize].d3dy,
                               scrPoints[i_0 as usize].d3dz);
                    n -= 1
                }
                glEnd();
            }
            index = (*pPolys).pindex;
            glBegin(0x5 as libc::c_int as GLenum);
            n = 0 as libc::c_int;
            while n < (*pPolys).npnts {
                glVertex3f(scrPoints[*index as usize].d3dx + (*light).x,
                           scrPoints[*index as usize].d3dy + (*light).y,
                           scrPoints[*index as usize].d3dz + (*light).z);
                glVertex3f(scrPoints[*index as usize].d3dx,
                           scrPoints[*index as usize].d3dy,
                           scrPoints[*index as usize].d3dz);
                n += 1;
                index = index.offset(1)
            }
            index = (*pPolys).pindex;
            glVertex3f(scrPoints[*index as usize].d3dx + (*light).x,
                       scrPoints[*index as usize].d3dy + (*light).y,
                       scrPoints[*index as usize].d3dz + (*light).z);
            glVertex3f(scrPoints[*index as usize].d3dx,
                       scrPoints[*index as usize].d3dy,
                       scrPoints[*index as usize].d3dz);
        } else {
            if (*pPolys).flags & 0x800 as libc::c_int as libc::c_uint != 0 &&
                   (*pPolys).flags & 0x2000 as libc::c_int as libc::c_uint !=
                       0 {
                glBegin(0x6 as libc::c_int as GLenum);
                index = (*pPolys).pindex;
                n = 0 as libc::c_int;
                while n < (*pPolys).npnts {
                    glVertex3f(scrPoints[*index as usize].d3dx,
                               scrPoints[*index as usize].d3dy,
                               scrPoints[*index as usize].d3dz);
                    n += 1;
                    index = index.offset(1)
                }
                glEnd();
            }
            index = (*pPolys).pindex;
            glBegin(0x5 as libc::c_int as GLenum);
            n = 0 as libc::c_int;
            while n < (*pPolys).npnts {
                glVertex3f(scrPoints[*index as usize].d3dx,
                           scrPoints[*index as usize].d3dy,
                           scrPoints[*index as usize].d3dz);
                glVertex3f(scrPoints[*index as usize].d3dx + (*light).x,
                           scrPoints[*index as usize].d3dy + (*light).y,
                           scrPoints[*index as usize].d3dz + (*light).z);
                n += 1;
                index = index.offset(1)
            }
            index = (*pPolys).pindex;
            glVertex3f(scrPoints[*index as usize].d3dx,
                       scrPoints[*index as usize].d3dy,
                       scrPoints[*index as usize].d3dz);
            glVertex3f(scrPoints[*index as usize].d3dx + (*light).x,
                       scrPoints[*index as usize].d3dy + (*light).y,
                       scrPoints[*index as usize].d3dz + (*light).z);
        }
        glEnd();
        i += 1;
        pPolys = pPolys.offset(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_CleanUp() {
    free(tshapes as *mut libc::c_void);
    free(scshapes as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pie_Draw3DShape(mut shape: *mut iIMDShape,
                                         mut frame: libc::c_int,
                                         mut team: libc::c_int,
                                         mut col: UDWORD, mut spec: UDWORD,
                                         mut pieFlag: libc::c_int,
                                         mut pieFlagData: libc::c_int) {
    let mut colour: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut specular: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut distance: libc::c_float = 0.;
    pieCount += 1;
    // Fix for transparent buildings and features!! */
    if pieFlag & 0x2 as libc::c_int != 0 && pieFlagData > 220 as libc::c_int {
        pieFlagData = 0 as libc::c_int;
        pieFlag = pieFlagData
        // force to bilinear and non-transparent
    }
    // Fix for transparent buildings and features!! */
    // WARZONE light as byte passed in colour so expand
    if col <= 255 as libc::c_int as UBYTE as libc::c_uint {
        colour.byte.a = 255 as libc::c_int as UBYTE; //no fog
        colour.byte.r = col as UBYTE;
        colour.byte.g = col as UBYTE;
        colour.byte.b = col as UBYTE
    } else { colour.argb = col }
    specular.argb = spec;
    if frame == 0 as libc::c_int { frame = team }
    if drawing_interface != 0 || shadows == 0 {
        pie_Draw3DShape2(shape, frame, colour, specular, pieFlag,
                         pieFlagData);
    } else if pieFlag & (0x4 as libc::c_int | 0x2 as libc::c_int) != 0 {
        if tshapes_size <= nb_tshapes {
            if tshapes_size == 0 as libc::c_int as libc::c_uint {
                tshapes_size = 64 as libc::c_int as libc::c_uint;
                tshapes =
                    malloc(tshapes_size.wrapping_mul(::std::mem::size_of::<transluscent_shape_t>()
                                                         as libc::c_ulong)) as
                        *mut transluscent_shape_t;
                memset(tshapes as *mut libc::c_void, 0 as libc::c_int,
                       tshapes_size.wrapping_mul(::std::mem::size_of::<transluscent_shape_t>()
                                                     as libc::c_ulong));
            } else {
                let mut old_size: libc::c_uint = tshapes_size;
                tshapes_size <<= 1 as libc::c_int;
                tshapes =
                    realloc(tshapes as *mut libc::c_void,
                            tshapes_size.wrapping_mul(::std::mem::size_of::<transluscent_shape_t>()
                                                          as libc::c_ulong))
                        as *mut transluscent_shape_t;
                memset(&mut *tshapes.offset(old_size as isize) as
                           *mut transluscent_shape_t as *mut libc::c_void,
                       0 as libc::c_int,
                       tshapes_size.wrapping_sub(old_size).wrapping_mul(::std::mem::size_of::<transluscent_shape_t>()
                                                                            as
                                                                            libc::c_ulong));
            }
        }
        glGetFloatv(0xba6 as libc::c_int as GLenum,
                    (*tshapes.offset(nb_tshapes as
                                         isize)).matrix.as_mut_ptr());
        let ref mut fresh0 = (*tshapes.offset(nb_tshapes as isize)).shape;
        *fresh0 = shape;
        (*tshapes.offset(nb_tshapes as isize)).frame = frame;
        (*tshapes.offset(nb_tshapes as isize)).colour = colour;
        (*tshapes.offset(nb_tshapes as isize)).specular = specular;
        (*tshapes.offset(nb_tshapes as isize)).flag = pieFlag;
        (*tshapes.offset(nb_tshapes as isize)).flag_data = pieFlagData;
        nb_tshapes = nb_tshapes.wrapping_add(1)
    } else {
        if scshapes_size <= nb_scshapes {
            if scshapes_size == 0 as libc::c_int as libc::c_uint {
                scshapes_size = 64 as libc::c_int as libc::c_uint;
                scshapes =
                    malloc(scshapes_size.wrapping_mul(::std::mem::size_of::<shadowcasting_shape_t>()
                                                          as libc::c_ulong))
                        as *mut shadowcasting_shape_t;
                memset(scshapes as *mut libc::c_void, 0 as libc::c_int,
                       scshapes_size.wrapping_mul(::std::mem::size_of::<shadowcasting_shape_t>()
                                                      as libc::c_ulong));
            } else {
                let mut old_size_0: libc::c_uint = scshapes_size;
                scshapes_size <<= 1 as libc::c_int;
                scshapes =
                    realloc(scshapes as *mut libc::c_void,
                            scshapes_size.wrapping_mul(::std::mem::size_of::<shadowcasting_shape_t>()
                                                           as libc::c_ulong))
                        as *mut shadowcasting_shape_t;
                memset(&mut *scshapes.offset(old_size_0 as isize) as
                           *mut shadowcasting_shape_t as *mut libc::c_void,
                       0 as libc::c_int,
                       scshapes_size.wrapping_sub(old_size_0).wrapping_mul(::std::mem::size_of::<shadowcasting_shape_t>()
                                                                               as
                                                                               libc::c_ulong));
            }
        }
        glGetFloatv(0xba6 as libc::c_int as GLenum,
                    (*scshapes.offset(nb_scshapes as
                                          isize)).matrix.as_mut_ptr());
        distance =
            (*scshapes.offset(nb_scshapes as
                                  isize)).matrix[12 as libc::c_int as usize] *
                (*scshapes.offset(nb_scshapes as
                                      isize)).matrix[12 as libc::c_int as
                                                         usize];
        distance +=
            (*scshapes.offset(nb_scshapes as
                                  isize)).matrix[13 as libc::c_int as usize] *
                (*scshapes.offset(nb_scshapes as
                                      isize)).matrix[13 as libc::c_int as
                                                         usize];
        distance +=
            (*scshapes.offset(nb_scshapes as
                                  isize)).matrix[14 as libc::c_int as usize] *
                (*scshapes.offset(nb_scshapes as
                                      isize)).matrix[14 as libc::c_int as
                                                         usize];
        // if object is too far in the fog don't generate a shadow.
        if distance <
               (6000 as libc::c_int * 6000 as libc::c_int) as libc::c_float {
            let ref mut fresh1 =
                (*scshapes.offset(nb_scshapes as isize)).shape;
            *fresh1 = shape;
            (*scshapes.offset(nb_scshapes as isize)).flag = pieFlag;
            (*scshapes.offset(nb_scshapes as isize)).flag_data = pieFlagData;
            nb_scshapes = nb_scshapes.wrapping_add(1)
        }
        pie_Draw3DShape2(shape, frame, colour, specular, pieFlag,
                         pieFlagData);
    };
}
#[no_mangle]
pub unsafe extern "C" fn inverse_matrix(mut src: *mut libc::c_float,
                                        mut dst: *mut libc::c_float) {
    let mut det: libc::c_float =
        *src.offset(0 as libc::c_int as isize) *
            *src.offset(5 as libc::c_int as isize) *
            *src.offset(10 as libc::c_int as isize) +
            *src.offset(4 as libc::c_int as isize) *
                *src.offset(9 as libc::c_int as isize) *
                *src.offset(2 as libc::c_int as isize) +
            *src.offset(8 as libc::c_int as isize) *
                *src.offset(1 as libc::c_int as isize) *
                *src.offset(6 as libc::c_int as isize) -
            *src.offset(2 as libc::c_int as isize) *
                *src.offset(5 as libc::c_int as isize) *
                *src.offset(8 as libc::c_int as isize) -
            *src.offset(6 as libc::c_int as isize) *
                *src.offset(9 as libc::c_int as isize) *
                *src.offset(0 as libc::c_int as isize) -
            *src.offset(10 as libc::c_int as isize) *
                *src.offset(1 as libc::c_int as isize) *
                *src.offset(4 as libc::c_int as isize);
    let mut invdet: libc::c_float =
        (1.0f64 / det as libc::c_double) as libc::c_float;
    *dst.offset(0 as libc::c_int as isize) =
        invdet *
            (*src.offset(5 as libc::c_int as isize) *
                 *src.offset(10 as libc::c_int as isize) -
                 *src.offset(9 as libc::c_int as isize) *
                     *src.offset(6 as libc::c_int as isize));
    *dst.offset(1 as libc::c_int as isize) =
        invdet *
            (*src.offset(9 as libc::c_int as isize) *
                 *src.offset(2 as libc::c_int as isize) -
                 *src.offset(1 as libc::c_int as isize) *
                     *src.offset(10 as libc::c_int as isize));
    *dst.offset(2 as libc::c_int as isize) =
        invdet *
            (*src.offset(1 as libc::c_int as isize) *
                 *src.offset(6 as libc::c_int as isize) -
                 *src.offset(5 as libc::c_int as isize) *
                     *src.offset(2 as libc::c_int as isize));
    *dst.offset(3 as libc::c_int as isize) =
        invdet *
            (*src.offset(8 as libc::c_int as isize) *
                 *src.offset(6 as libc::c_int as isize) -
                 *src.offset(4 as libc::c_int as isize) *
                     *src.offset(10 as libc::c_int as isize));
    *dst.offset(4 as libc::c_int as isize) =
        invdet *
            (*src.offset(0 as libc::c_int as isize) *
                 *src.offset(10 as libc::c_int as isize) -
                 *src.offset(8 as libc::c_int as isize) *
                     *src.offset(2 as libc::c_int as isize));
    *dst.offset(5 as libc::c_int as isize) =
        invdet *
            (*src.offset(4 as libc::c_int as isize) *
                 *src.offset(2 as libc::c_int as isize) -
                 *src.offset(0 as libc::c_int as isize) *
                     *src.offset(6 as libc::c_int as isize));
    *dst.offset(6 as libc::c_int as isize) =
        invdet *
            (*src.offset(4 as libc::c_int as isize) *
                 *src.offset(9 as libc::c_int as isize) -
                 *src.offset(8 as libc::c_int as isize) *
                     *src.offset(5 as libc::c_int as isize));
    *dst.offset(7 as libc::c_int as isize) =
        invdet *
            (*src.offset(8 as libc::c_int as isize) *
                 *src.offset(1 as libc::c_int as isize) -
                 *src.offset(0 as libc::c_int as isize) *
                     *src.offset(9 as libc::c_int as isize));
    *dst.offset(8 as libc::c_int as isize) =
        invdet *
            (*src.offset(0 as libc::c_int as isize) *
                 *src.offset(5 as libc::c_int as isize) -
                 *src.offset(4 as libc::c_int as isize) *
                     *src.offset(1 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawShadows() {
    static mut dlist_defined: BOOL = 0 as libc::c_int;
    static mut dlist: GLuint = 0;
    let mut i: libc::c_uint = 0;
    let mut l: [libc::c_float; 4] = [0.; 4];
    let mut light: fVector = fVector{x: 0., y: 0., z: 0.,};
    let mut invmat: [libc::c_float; 9] = [0.; 9];
    let mut width: libc::c_float = pie_GetVideoBufferWidth() as libc::c_float;
    let mut height: libc::c_float =
        pie_GetVideoBufferHeight() as libc::c_float;
    glGetLightfv(0x4000 as libc::c_int as GLenum,
                 0x1203 as libc::c_int as GLenum, l.as_mut_ptr());
    pie_SetTexturePage(-(1 as libc::c_int));
    glPushMatrix();
    pie_SetColourKeyedBlack(0 as libc::c_int);
    glColorMask(0 as libc::c_int as GLboolean, 0 as libc::c_int as GLboolean,
                0 as libc::c_int as GLboolean, 0 as libc::c_int as GLboolean);
    glDepthFunc(0x201 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    glEnable(0xb90 as libc::c_int as GLenum);
    glClear(0x400 as libc::c_int as GLbitfield);
    if stencil_one_pass() != 0 {
        glEnable(0x8910 as libc::c_int as GLenum);
        glDisable(0xb44 as libc::c_int as GLenum);
        glStencilMask(!(0 as libc::c_int) as GLuint);
        glActiveStencilFaceEXT.expect("non-null function pointer")(0x405 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum);
        glStencilOp(0x1e00 as libc::c_int as GLenum,
                    0x1e00 as libc::c_int as GLenum,
                    0x8508 as libc::c_int as GLenum);
        glStencilFunc(0x207 as libc::c_int as GLenum, 0 as libc::c_int,
                      !(0 as libc::c_int) as GLuint);
        glActiveStencilFaceEXT.expect("non-null function pointer")(0x404 as
                                                                       libc::c_int
                                                                       as
                                                                       GLenum);
        glStencilOp(0x1e00 as libc::c_int as GLenum,
                    0x1e00 as libc::c_int as GLenum,
                    0x8507 as libc::c_int as GLenum);
        glStencilFunc(0x207 as libc::c_int as GLenum, 0 as libc::c_int,
                      !(0 as libc::c_int) as GLuint);
    } else {
        if dlist_defined == 0 {
            dlist = glGenLists(1 as libc::c_int);
            dlist_defined = 1 as libc::c_int
        }
        // Setup stencil for back faces.
        glStencilMask(!(0 as libc::c_int) as GLuint);
        glStencilFunc(0x207 as libc::c_int as GLenum, 0 as libc::c_int,
                      !(0 as libc::c_int) as GLuint);
        glEnable(0xb44 as libc::c_int as GLenum);
        glCullFace(0x405 as libc::c_int as GLenum);
        glStencilOp(0x1e00 as libc::c_int as GLenum,
                    0x1e00 as libc::c_int as GLenum,
                    0x1e02 as libc::c_int as GLenum);
        // Start display list.
        glNewList(dlist, 0x1301 as libc::c_int as GLenum);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < nb_scshapes {
        glLoadIdentity();
        glMultMatrixf((*scshapes.offset(i as isize)).matrix.as_mut_ptr());
        inverse_matrix((*scshapes.offset(i as isize)).matrix.as_mut_ptr(),
                       invmat.as_mut_ptr());
        light.x =
            invmat[0 as libc::c_int as usize] * l[0 as libc::c_int as usize] +
                invmat[3 as libc::c_int as usize] *
                    l[1 as libc::c_int as usize] +
                invmat[6 as libc::c_int as usize] *
                    l[2 as libc::c_int as usize];
        light.y =
            invmat[1 as libc::c_int as usize] * l[0 as libc::c_int as usize] +
                invmat[4 as libc::c_int as usize] *
                    l[1 as libc::c_int as usize] +
                invmat[7 as libc::c_int as usize] *
                    l[2 as libc::c_int as usize];
        light.z =
            invmat[2 as libc::c_int as usize] * l[0 as libc::c_int as usize] +
                invmat[5 as libc::c_int as usize] *
                    l[1 as libc::c_int as usize] +
                invmat[8 as libc::c_int as usize] *
                    l[2 as libc::c_int as usize];
        pie_DrawShadow((*scshapes.offset(i as isize)).shape,
                       (*scshapes.offset(i as isize)).flag,
                       (*scshapes.offset(i as isize)).flag_data, &mut light);
        i = i.wrapping_add(1)
    }
    glEndList();
    if stencil_one_pass() != 0 {
        glDisable(0x8910 as libc::c_int as GLenum);
    } else {
        // End display list.
        glEndList();
        // Setup stencil for front faces.
        glCullFace(0x404 as libc::c_int as GLenum);
        glStencilOp(0x1e00 as libc::c_int as GLenum,
                    0x1e00 as libc::c_int as GLenum,
                    0x1e03 as libc::c_int as GLenum);
        // Draw display list
        glCallList(dlist);
    }
    glEnable(0xb44 as libc::c_int as GLenum);
    glColorMask(1 as libc::c_int as GLboolean, 1 as libc::c_int as GLboolean,
                1 as libc::c_int as GLboolean, 1 as libc::c_int as GLboolean);
    glStencilOp(0x1e00 as libc::c_int as GLenum,
                0x1e00 as libc::c_int as GLenum,
                0x1e00 as libc::c_int as GLenum);
    glStencilMask(!(0 as libc::c_int) as GLuint);
    glStencilFunc(0x201 as libc::c_int as GLenum, 0 as libc::c_int,
                  !(0 as libc::c_int) as GLuint);
    glEnable(0xbe2 as libc::c_int as GLenum);
    glBlendFunc(0x302 as libc::c_int as GLenum,
                0x303 as libc::c_int as GLenum);
    glColor4f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat,
              0 as libc::c_int as GLfloat, 0.5f64 as GLfloat);
    pie_PerspectiveEnd();
    glLoadIdentity();
    glDisable(0xb71 as libc::c_int as GLenum);
    glBegin(0x5 as libc::c_int as GLenum);
    glVertex2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex2f(width, 0 as libc::c_int as GLfloat);
    glVertex2f(0 as libc::c_int as GLfloat, height);
    glVertex2f(width, height);
    glEnd();
    pie_PerspectiveBegin();
    glDisable(0xbe2 as libc::c_int as GLenum);
    glDisable(0xb90 as libc::c_int as GLenum);
    glEnable(0xb71 as libc::c_int as GLenum);
    glDepthMask(1 as libc::c_int as GLboolean);
    glPopMatrix();
    nb_scshapes = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawRemainingTransShapes() {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < nb_tshapes {
        glPushMatrix();
        glLoadIdentity();
        glMultMatrixf((*tshapes.offset(i as isize)).matrix.as_mut_ptr());
        pie_Draw3DShape2((*tshapes.offset(i as isize)).shape,
                         (*tshapes.offset(i as isize)).frame,
                         (*tshapes.offset(i as isize)).colour,
                         (*tshapes.offset(i as isize)).specular,
                         (*tshapes.offset(i as isize)).flag,
                         (*tshapes.offset(i as isize)).flag_data);
        glPopMatrix();
        i = i.wrapping_add(1)
    }
    nb_tshapes = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn pie_RemainingPasses() {
    pie_DrawShadows();
    pie_DrawRemainingTransShapes();
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawImage(mut image: *mut PIEIMAGE,
                                       mut dest: *mut PIERECT,
                                       mut style: *mut PIESTYLE) {
    /* Set transparent color to be 0 red, 0 green, 0 blue, 0 alpha */
    polyCount += 1;
    pie_SetTexturePage((*image).texPage);
    (*style).colour.argb = pie_GetColour();
    (*style).specular.argb = 0 as libc::c_int as UDWORD;
    glBegin(0x5 as libc::c_int as GLenum);
    glColor4ub((*style).colour.byte.r, (*style).colour.byte.g,
               (*style).colour.byte.b, (*style).colour.byte.a);
    //set up 4 pie verts
    glTexCoord2f((*image).tu as GLfloat, (*image).tv as GLfloat);
    glVertex2f((*dest).x as GLfloat, (*dest).y as GLfloat);
    //pieVrts[0].sz  = (SDWORD)INTERFACE_DEPTH;
	//pieVrts[0].specular.argb = style->specular.argb;
    glTexCoord2f(((*image).tu as libc::c_int + (*image).tw as libc::c_int +
                      0 as libc::c_int) as GLfloat, (*image).tv as GLfloat);
    glVertex2f(((*dest).x as libc::c_int + (*dest).w as libc::c_int +
                    0 as libc::c_int) as GLfloat, (*dest).y as GLfloat);
    //pieVrts[1].sz  = (SDWORD)INTERFACE_DEPTH;
	//pieVrts[1].specular.argb = style->specular.argb;
    glTexCoord2f((*image).tu as GLfloat,
                 ((*image).tv as libc::c_int + (*image).th as libc::c_int +
                      0 as libc::c_int) as GLfloat);
    glVertex2f((*dest).x as GLfloat,
               ((*dest).y as libc::c_int + (*dest).h as libc::c_int +
                    0 as libc::c_int) as GLfloat);
    //pieVrts[3].sz  = (SDWORD)INTERFACE_DEPTH;
	//pieVrts[3].specular.argb = style->specular.argb;
    glTexCoord2f(((*image).tu as libc::c_int + (*image).tw as libc::c_int +
                      0 as libc::c_int) as GLfloat,
                 ((*image).tv as libc::c_int + (*image).th as libc::c_int +
                      0 as libc::c_int) as GLfloat);
    glVertex2f(((*dest).x as libc::c_int + (*dest).w as libc::c_int +
                    0 as libc::c_int) as GLfloat,
               ((*dest).y as libc::c_int + (*dest).h as libc::c_int +
                    0 as libc::c_int) as GLfloat);
    //pieVrts[2].sz  = (SDWORD)INTERFACE_DEPTH;
	//pieVrts[2].specular.argb = style->specular.argb;
    glEnd();
}
/* **************************************************************************
 * pie_Drawimage270
 *
 * General purpose blit function
 * Will support zbuffering, non_textured, coloured lighting and alpha effects
 *
 * replaces all ivis blit functions
 *
 ***************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_DrawImage270(mut image: *mut PIEIMAGE,
                                          mut dest: *mut PIERECT,
                                          mut style: *mut PIESTYLE) {
    let mut colour: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    polyCount += 1;
    pie_SetTexturePage((*image).texPage);
    colour.argb = pie_GetColour();
    glBegin(0x6 as libc::c_int as GLenum);
    glColor4ub(colour.byte.r, colour.byte.g, colour.byte.b, colour.byte.a);
    glTexCoord2f(((*image).tu as libc::c_int + (*image).tw as libc::c_int +
                      0 as libc::c_int) as GLfloat, (*image).tv as GLfloat);
    glVertex2f((*dest).x as GLfloat, (*dest).y as GLfloat);
    glTexCoord2f(((*image).tu as libc::c_int + (*image).tw as libc::c_int +
                      0 as libc::c_int) as GLfloat,
                 ((*image).tv as libc::c_int + (*image).th as libc::c_int +
                      0 as libc::c_int) as GLfloat);
    glVertex2f(((*dest).x as libc::c_int + (*dest).h as libc::c_int +
                    0 as libc::c_int) as GLfloat, (*dest).y as GLfloat);
    glTexCoord2f((*image).tu as GLfloat,
                 ((*image).tv as libc::c_int + (*image).th as libc::c_int +
                      0 as libc::c_int) as GLfloat);
    glVertex2f(((*dest).x as libc::c_int + (*dest).h as libc::c_int +
                    0 as libc::c_int) as GLfloat,
               ((*dest).y as libc::c_int + (*dest).w as libc::c_int +
                    0 as libc::c_int) as GLfloat);
    glTexCoord2f((*image).tu as GLfloat, (*image).tv as GLfloat);
    glVertex2f((*dest).x as GLfloat,
               ((*dest).y as libc::c_int + (*dest).w as libc::c_int +
                    0 as libc::c_int) as GLfloat);
    glEnd();
}
/* **************************************************************************
 * pie_DrawLine
 *
 * universal line function for hardware
 *
 * Assumes render mode set up externally
 *
 ***************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_DrawLine(mut x0: SDWORD, mut y0: SDWORD,
                                      mut x1: SDWORD, mut y1: SDWORD,
                                      mut colour: UDWORD, mut bClip: BOOL) {
    polyCount += 1;
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetColourKeyedBlack(0 as libc::c_int);
    pie_SetColour(colour);
    glBegin(0x3 as libc::c_int as GLenum);
    glVertex3f(x0 as GLfloat, y0 as GLfloat, 32000.0f32 - 1.0f32);
    glVertex3f(x1 as GLfloat, y1 as GLfloat, 32000.0f32 - 1.0f32);
    glEnd();
}
/* **************************************************************************
 * pie_DrawRect
 *
 * universal rectangle function for hardware
 *
 * Assumes render mode set up externally, draws filled rectangle
 *
 ***************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_DrawRect(mut x0: SDWORD, mut y0: SDWORD,
                                      mut x1: SDWORD, mut y1: SDWORD,
                                      mut colour: UDWORD, mut bClip: BOOL) {
    //	SDWORD swap;
    let mut c: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    polyCount += 1;
    c.argb = colour;
    pie_SetColourKeyedBlack(0 as libc::c_int);
    glColor4ub(c.byte.r, c.byte.g, c.byte.b, c.byte.a);
    glBegin(0x5 as libc::c_int as GLenum);
    glVertex2i(x0, y0);
    glVertex2i(x1, y0);
    glVertex2i(x0, y1);
    glVertex2i(x1, y1);
    glEnd();
}
//static void pie_D3DPolyFrame(PIED3DPOLY *poly, SDWORD frame);
//pievertex draw poly (low level) //all modes from PIEVERTEX data
/* **************************************************************************
 * pie_PiePoly
 *
 * universal poly draw function for hardware
 *
 * Assumes render mode set up externally
 *
 ***************************************************************************/
unsafe extern "C" fn pie_PiePoly(mut poly: *mut PIEPOLY, mut light: BOOL) {
    polyCount += 1;
    if (*poly).nVrts >= 3 as libc::c_int {
        if (*poly).flags & 0x800 as libc::c_int as libc::c_uint != 0 {
            pie_SetColourKeyedBlack(1 as libc::c_int);
        } else { pie_SetColourKeyedBlack(0 as libc::c_int); }
        pie_SetColourKeyedBlack(1 as libc::c_int);
        if (*poly).flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
            glDisable(0xb44 as libc::c_int as GLenum);
        }
        pie_Polygon((*poly).nVrts, (*poly).pVrts, 0.0f64 as FRACT, light);
        if (*poly).flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
            glEnable(0xb44 as libc::c_int as GLenum);
        }
    };
}
unsafe extern "C" fn pie_PiePolyFrame(mut poly: *mut PIEPOLY,
                                      mut frame: SDWORD, mut light: BOOL) {
    let mut uFrame: libc::c_int = 0;
    let mut vFrame: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut framesPerLine: libc::c_int = 0;
    if (*poly).flags & 0x4000 as libc::c_int as libc::c_uint != 0 &&
           frame != 0 as libc::c_int {
        if !(*poly).pTexAnim.is_null() {
            if (*(*poly).pTexAnim).nFrames >= 0 as libc::c_int {
                frame %= (*(*poly).pTexAnim).nFrames
            } else { frame %= -(*(*poly).pTexAnim).nFrames }
            if frame > 0 as libc::c_int {
                // HACK - fix this!!!!
                framesPerLine =
                    256 as libc::c_int / (*(*poly).pTexAnim).textureWidth;
                //should be		framesPerLine = iV_TEXTEX(texPage)->width / poly->pTexAnim->textureWidth;
                vFrame = 0 as libc::c_int;
                while frame >= framesPerLine {
                    frame -= framesPerLine;
                    vFrame += (*(*poly).pTexAnim).textureHeight
                }
                uFrame = frame * (*(*poly).pTexAnim).textureWidth;
                j = 0 as libc::c_int;
                while j < (*poly).nVrts {
                    let ref mut fresh2 =
                        (*(*poly).pVrts.offset(j as isize)).tu;
                    *fresh2 = (*fresh2 as libc::c_int + uFrame) as UWORD;
                    let ref mut fresh3 =
                        (*(*poly).pVrts.offset(j as isize)).tv;
                    *fresh3 = (*fresh3 as libc::c_int + vFrame) as UWORD;
                    j += 1
                }
            }
        }
    }
    //draw with new texture data
    pie_PiePoly(poly, light);
}
/* **************************************************************************/
/*
 *	Local ProtoTypes
 */
/* **************************************************************************/
//old ivis style draw poly (low level) software mode only
//old ivis style draw poly
/* **************************************************************************
 * pie_IvisPoly
 *
 * optimised poly draw function for software
 *
 * Assumes render mode NOT set up externally
 *                     ---
 ***************************************************************************/
unsafe extern "C" fn pie_IvisPoly(mut texPage: SDWORD,
                                  mut poly: *mut iIMDPoly, mut bClip: BOOL) {
    polyCount += 1;
    if (*poly).npnts >= 3 as libc::c_int {
        let mut i: SDWORD = 0;
        if (*poly).flags & 0x800 as libc::c_int as libc::c_uint != 0 {
            pie_SetColourKeyedBlack(1 as libc::c_int);
        } else { pie_SetColourKeyedBlack(0 as libc::c_int); }
        glBegin(0x6 as libc::c_int as GLenum);
        i = 0 as libc::c_int;
        while i < (*poly).npnts {
            glColor3ub(((*(*poly).vrt.offset(i as isize)).g as libc::c_int *
                            16 as libc::c_int) as GLubyte,
                       ((*(*poly).vrt.offset(i as isize)).g as libc::c_int *
                            16 as libc::c_int) as GLubyte,
                       ((*(*poly).vrt.offset(i as isize)).g as libc::c_int *
                            16 as libc::c_int) as GLubyte);
            glTexCoord2f((*(*poly).vrt.offset(i as isize)).u as GLfloat,
                         (*(*poly).vrt.offset(i as isize)).v as GLfloat);
            glVertex3f((*(*poly).vrt.offset(i as isize)).x as GLfloat,
                       (*(*poly).vrt.offset(i as isize)).y as GLfloat,
                       (*(*poly).vrt.offset(i as isize)).z as GLfloat);
            i += 1
        }
        glEnd();
    };
}
unsafe extern "C" fn pie_IvisPolyFrame(mut texPage: SDWORD,
                                       mut poly: *mut iIMDPoly,
                                       mut frame: SDWORD, mut bClip: BOOL) {
    let mut uFrame: libc::c_int = 0;
    let mut vFrame: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut framesPerLine: libc::c_int = 0;
    polyCount += 1;
    if (*poly).flags & 0x4000 as libc::c_int as libc::c_uint != 0 &&
           frame != 0 as libc::c_int {
        if !(*poly).pTexAnim.is_null() {
            if (*(*poly).pTexAnim).nFrames >= 0 as libc::c_int {
                frame %= (*(*poly).pTexAnim).nFrames
            } else {
                //frame is colour key
                frame %= -(*(*poly).pTexAnim).nFrames
            }
            if frame > 0 as libc::c_int {
                framesPerLine =
                    (*(&mut (*_TEX_PAGE.as_mut_ptr().offset(texPage as
                                                                isize)).tex as
                           *mut iTexture)).width /
                        (*(*poly).pTexAnim).textureWidth;
                vFrame = 0 as libc::c_int;
                while frame >= framesPerLine {
                    frame -= framesPerLine;
                    vFrame += (*(*poly).pTexAnim).textureHeight
                }
                uFrame = frame * (*(*poly).pTexAnim).textureWidth;
                // shift the textures for animation
                if (*poly).flags & 0x4000 as libc::c_int as libc::c_uint != 0
                   {
                    j = 0 as libc::c_int;
                    while j < (*poly).npnts {
                        let ref mut fresh4 =
                            (*(*poly).vrt.offset(j as isize)).u;
                        *fresh4 += uFrame;
                        let ref mut fresh5 =
                            (*(*poly).vrt.offset(j as isize)).v;
                        *fresh5 += vFrame;
                        j += 1
                    }
                }
            }
        }
    }
    pie_IvisPoly(texPage, poly, bClip);
}
/* **************************************************************************
 *
 *
 *
 ***************************************************************************/
//ivis style draw function
#[no_mangle]
pub unsafe extern "C" fn pie_DrawTriangle(mut pv: *mut iVertex,
                                          mut texPage: *mut iTexture,
                                          mut renderFlags: UDWORD,
                                          mut offset: *mut iPoint) {
    let mut i: UDWORD = 0;
    glBegin(0x6 as libc::c_int as GLenum);
    i = 0 as libc::c_int as UDWORD;
    while i < 3 as libc::c_int as libc::c_uint {
        glColor4ub(((*pv.offset(i as isize)).g as libc::c_int *
                        16 as libc::c_int) as GLubyte,
                   ((*pv.offset(i as isize)).g as libc::c_int *
                        16 as libc::c_int) as GLubyte,
                   ((*pv.offset(i as isize)).g as libc::c_int *
                        16 as libc::c_int) as GLubyte,
                   255 as libc::c_int as GLubyte);
        glTexCoord2f((*pv.offset(i as isize)).u as GLfloat,
                     (*pv.offset(i as isize)).v as GLfloat);
        glVertex3f((*pv.offset(i as isize)).x as GLfloat,
                   (*pv.offset(i as isize)).y as GLfloat,
                   (*pv.offset(i as isize)).z as GLfloat);
        i = i.wrapping_add(1)
    }
    glEnd();
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawFastTriangle(mut v1: *mut PIEVERTEX,
                                              mut v2: *mut PIEVERTEX,
                                              mut v3: *mut PIEVERTEX,
                                              mut texPage: *mut iTexture,
                                              mut pieFlag: libc::c_int,
                                              mut pieFlagData: libc::c_int) {
}
#[no_mangle]
pub unsafe extern "C" fn pie_DrawPoly(mut numVrts: SDWORD,
                                      mut aVrts: *mut PIEVERTEX,
                                      mut texPage: SDWORD,
                                      mut psEffects: *mut libc::c_void) {
    let mut offset: FRACT = 0 as libc::c_int as FRACT;
    /*	Since this is only used from within source for the terrain draw - we can backface cull the
		polygons.


	*/
    tileCount += 1;
    pie_SetTexturePage(texPage);
    pie_SetFogStatus(1 as libc::c_int);
    if psEffects.is_null() {
        //jps 15apr99 translucent water code
        pie_SetRendMode(REND_GOURAUD_TEX); //jps 15apr99 old solid water code
        pie_SetColourKeyedBlack(1 as libc::c_int);
    } else {
        //jps 15apr99 translucent water code
        pie_SetRendMode(REND_ALPHA_TEX); //jps 15apr99 old solid water code
        pie_SetColourKeyedBlack(0 as libc::c_int);
        offset = *(psEffects as *mut libc::c_float)
    }
    pie_SetBilinear(1 as libc::c_int);
    if numVrts >= 3 as libc::c_int {
        pie_Polygon(numVrts, aVrts, offset, 0 as libc::c_int);
    };
}
//piedraw functions used in piefunc.c
// PNG
//necromancer
//#ifdef NECROMANCER
#[no_mangle]
pub unsafe extern "C" fn pie_DrawTile(mut pv0: *mut PIEVERTEX,
                                      mut pv1: *mut PIEVERTEX,
                                      mut pv2: *mut PIEVERTEX,
                                      mut pv3: *mut PIEVERTEX,
                                      mut texPage: SDWORD) {
    tileCount += 1;
    pie_SetRendMode(REND_GOURAUD_TEX);
    pie_SetTexturePage(texPage);
    pie_SetBilinear(1 as libc::c_int);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           pv0 as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           pv1 as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           pv2 as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(3 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           pv3 as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    pie_Polygon(4 as libc::c_int, pieVrts.as_mut_ptr(), 0.0f64 as FRACT,
                1 as libc::c_int);
}
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
//extern void pie_Draw3DIntelShape(iIMDShape *shape, int frame, int team, UDWORD colour, UDWORD specular, int pieFlag, int pieData);
//extern void pie_Draw3DNowShape(iIMDShape *shape, int frame, int team, UDWORD col, UDWORD spec, int pieFlag, int pieFlagData);
//PIEVERTEX line draw for all hardware modes
//iVetrex triangle draw for software modes
//PIEVERTEX poly draw for all hardware modes
//PIEVERTEX triangle draw (glide specific)
//#endif
#[no_mangle]
pub unsafe extern "C" fn pie_GetResetCounts(mut pPieCount: *mut SDWORD,
                                            mut pTileCount: *mut SDWORD,
                                            mut pPolyCount: *mut SDWORD,
                                            mut pStateCount: *mut SDWORD) {
    *pPieCount = pieCount;
    *pTileCount = tileCount;
    *pPolyCount = polyCount;
    *pStateCount = pieStateCount;
    pieCount = 0 as libc::c_int;
    tileCount = 0 as libc::c_int;
    polyCount = 0 as libc::c_int;
    pieStateCount = 0 as libc::c_int;
}
