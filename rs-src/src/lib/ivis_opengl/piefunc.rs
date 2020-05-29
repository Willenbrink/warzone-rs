use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn glBegin(mode: GLenum);
    #[no_mangle]
    fn glEnd();
    #[no_mangle]
    fn glVertex2f(x: GLfloat, y: GLfloat);
    #[no_mangle]
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    #[no_mangle]
    fn glColor4ub(red: GLubyte, green: GLubyte, blue: GLubyte,
                  alpha: GLubyte);
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
    /* Useful for periodical stuff */
/* Will return a number that climbs over tickFrequency game ticks and ends up in the required range. */
/*	
	For instance getTimeValueRange(4096,256) will return a number that cycles through
	the values 0..256 every 4.096 seconds...
	Ensure that the first is an integer multiple of the second 
*/
    #[no_mangle]
    fn getTimeValueRange(tickFrequency: UDWORD, requiredRange: UDWORD)
     -> UDWORD;
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    static mut psRendSurface: *mut iSurface;
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    //render states
    #[no_mangle]
    fn pie_SetTexturePage(num: SDWORD);
    #[no_mangle]
    fn pie_SetRendMode(rendMode: REND_MODE);
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Set2DClip(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                     y1: libc::c_int);
    #[no_mangle]
    fn pie_ClipTextured(npoints: libc::c_int, points: *mut PIEVERTEX,
                        clip: *mut PIEVERTEX, bSpecular: BOOL) -> libc::c_int;
}
pub type GLenum = libc::c_uint;
pub type GLubyte = libc::c_uchar;
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
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
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
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
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
/* **************************************************************************/
/*
 * piefunc.c
 *
 * extended render routines for 3D rendering
 *
 */
/* **************************************************************************/
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
static mut clippedVrts: [PIEVERTEX; 10] =
    [PIEVERTEX{sx: 0,
               sy: 0,
               sz: 0,
               tu: 0,
               tv: 0,
               light: PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},
               specular:
                   PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},},};
        10];
static mut aByteScale: [[UBYTE; 256]; 256] = [[0; 256]; 256];
/* **************************************************************************/
/*
 * piefunc.h
 *
 * type defines for extended image library functions.
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
pub unsafe extern "C" fn pie_DownLoadBufferToScreen(mut pSrcData:
                                                        *mut libc::c_void,
                                                    mut destX: UDWORD,
                                                    mut destY: UDWORD,
                                                    mut srcWidth: UDWORD,
                                                    mut srcHeight: UDWORD,
                                                    mut srcStride: UDWORD) {
}
/* **************************************************************************/
/*
 *	void pie_RectFilter(SDWORD x0, SDWORD y0, SDWORD x1, SDWORD y1, UDWORD colour)
 *
 * Draws rectangular filter to screen ivis mode defaults to
 *
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn pie_RectFilter(mut x0: SDWORD, mut y0: SDWORD,
                                        mut x1: SDWORD, mut y1: SDWORD,
                                        mut colour: UDWORD) {
    pie_TransBoxFill(x0, y0, x1, y1);
}
/* ---------------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn pie_CornerBox(mut x0: SDWORD, mut y0: SDWORD,
                                       mut x1: SDWORD, mut y1: SDWORD,
                                       mut colour: UDWORD, mut a: UBYTE,
                                       mut b: UBYTE, mut c: UBYTE,
                                       mut d: UBYTE) {
}
/* ---------------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn pie_DrawViewingWindow(mut v: *mut iVector,
                                               mut x1: UDWORD, mut y1: UDWORD,
                                               mut x2: UDWORD, mut y2: UDWORD,
                                               mut colour: UDWORD) {
    let mut clip: SDWORD = 0;
    let mut i: SDWORD = 0;
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetRendMode(REND_ALPHA_FLAT);
    //PIE verts
    pieVrts[0 as libc::c_int as usize].sx =
        (*v.offset(1 as libc::c_int as isize)).x;
    pieVrts[0 as libc::c_int as usize].sy =
        (*v.offset(1 as libc::c_int as isize)).y;
    //cull triangles with off screen points
    pieVrts[0 as libc::c_int as usize].sz =
        (32000.0f32 - 1.0f32) as SDWORD; //0x7fffffff;
    pieVrts[0 as libc::c_int as usize].tu = 0.0f64 as UWORD;
    pieVrts[0 as libc::c_int as usize].tv = 0.0f64 as UWORD;
    pieVrts[0 as libc::c_int as usize].light.argb = colour;
    pieVrts[0 as libc::c_int as usize].specular.argb =
        0 as libc::c_int as UDWORD;
    memcpy(&mut *pieVrts.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut PIEVERTEX as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(2 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut PIEVERTEX as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(3 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut PIEVERTEX as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    memcpy(&mut *pieVrts.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut PIEVERTEX as *mut libc::c_void,
           &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut PIEVERTEX as *const libc::c_void,
           ::std::mem::size_of::<PIEVERTEX>() as libc::c_ulong);
    pieVrts[1 as libc::c_int as usize].sx =
        (*v.offset(0 as libc::c_int as isize)).x;
    pieVrts[1 as libc::c_int as usize].sy =
        (*v.offset(0 as libc::c_int as isize)).y;
    pieVrts[2 as libc::c_int as usize].sx =
        (*v.offset(2 as libc::c_int as isize)).x;
    pieVrts[2 as libc::c_int as usize].sy =
        (*v.offset(2 as libc::c_int as isize)).y;
    pieVrts[3 as libc::c_int as usize].sx =
        (*v.offset(3 as libc::c_int as isize)).x;
    pieVrts[3 as libc::c_int as usize].sy =
        (*v.offset(3 as libc::c_int as isize)).y;
    pie_Set2DClip(x1 as libc::c_int, y1 as libc::c_int,
                  x2.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      libc::c_int,
                  y2.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      libc::c_int);
    clip =
        pie_ClipTextured(4 as libc::c_int,
                         &mut *pieVrts.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize),
                         &mut *clippedVrts.as_mut_ptr().offset(0 as
                                                                   libc::c_int
                                                                   as isize),
                         0 as libc::c_int);
    pie_Set2DClip(0 as libc::c_int, 0 as libc::c_int,
                  (*psRendSurface).width - 0 as libc::c_int,
                  (*psRendSurface).height - 0 as libc::c_int);
    if clip >= 3 as libc::c_int {
        let mut c: PIELIGHT =
            PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
        c.argb = colour;
        glBegin(0x6 as libc::c_int as GLenum);
        glColor4ub(c.byte.r, c.byte.g, c.byte.b,
                   (c.byte.a as libc::c_int >> 1 as libc::c_int) as GLubyte);
        i = 0 as libc::c_int;
        while i < clip {
            glVertex2f(clippedVrts[i as usize].sx as GLfloat,
                       clippedVrts[i as usize].sy as GLfloat);
            i += 1
        }
        glEnd();
        glBegin(0x3 as libc::c_int as GLenum);
        glColor4ub(c.byte.r, c.byte.g, c.byte.b, c.byte.a);
        i = 0 as libc::c_int;
        while i < clip {
            glVertex2f(clippedVrts[i as usize].sx as GLfloat,
                       clippedVrts[i as usize].sy as GLfloat);
            i += 1
        }
        glVertex2f(clippedVrts[0 as libc::c_int as usize].sx as GLfloat,
                   clippedVrts[0 as libc::c_int as usize].sy as GLfloat);
        glEnd();
    };
}
/* ---------------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn pie_TransColouredTriangle(mut vrt: *mut PIEVERTEX,
                                                   mut rgb: UDWORD,
                                                   mut trans: UDWORD) {
    let mut c: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut i: UDWORD = 0;
    c.argb = rgb;
    pie_SetTexturePage(-(1 as libc::c_int));
    pie_SetRendMode(REND_ALPHA_ITERATED);
    glBegin(0x6 as libc::c_int as GLenum);
    glColor4ub(c.byte.r, c.byte.g, c.byte.b, 128 as libc::c_int as GLubyte);
    i = 0 as libc::c_int as UDWORD;
    while i < 3 as libc::c_int as libc::c_uint {
        glVertex3f((*vrt.offset(i as isize)).sx as GLfloat,
                   (*vrt.offset(i as isize)).sy as GLfloat,
                   (*vrt.offset(i as isize)).sz as GLfloat);
        i = i.wrapping_add(1)
    }
    glEnd();
}
/* ---------------------------------------------------------------------------------- */
#[no_mangle]
pub unsafe extern "C" fn pie_InitMaths() {
    let mut c: UBYTE = 0;
    let mut a: UDWORD = 0;
    let mut b: UDWORD = 0;
    let mut bigC: UDWORD = 0;
    a = 0 as libc::c_int as UDWORD;
    while a <= 0xff as libc::c_int as libc::c_uint {
        b = 0 as libc::c_int as UDWORD;
        while b <= 0xff as libc::c_int as libc::c_uint {
            bigC = a.wrapping_mul(b);
            bigC =
                (bigC as
                     libc::c_uint).wrapping_div(0xff as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if bigC <= 0xff as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"light_InitMaths; rounding error\x00" as *const u8 as
                          *const libc::c_char);
            };
            if bigC <= 0xff as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"piefunc.c\x00" as *const u8 as *const libc::c_char,
                      174 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"pie_InitMaths\x00")).as_ptr(),
                      b"bigC <= UBYTE_MAX\x00" as *const u8 as
                          *const libc::c_char);
            };
            c = bigC as UBYTE;
            aByteScale[a as usize][b as usize] = c;
            b = b.wrapping_add(1)
        }
        a = a.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pie_ByteScale(mut a: UBYTE, mut b: UBYTE) -> UBYTE {
    return ((a as UDWORD).wrapping_mul(b as UDWORD) >> 8 as libc::c_int) as
               UBYTE;
}
//extern void	pie_doWeirdBoxFX(UDWORD x, UDWORD y, UDWORD x2, UDWORD y2);
#[no_mangle]
pub unsafe extern "C" fn pie_doWeirdBoxFX(mut x: UDWORD, mut y: UDWORD,
                                          mut x2: UDWORD, mut y2: UDWORD,
                                          mut trans: UDWORD) {
    let mut val: UDWORD = 0; // cos it's fixed point
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut radius: UDWORD = 0;
    val =
        getTimeValueRange(5760 as libc::c_int as UDWORD,
                          360 as libc::c_int as UDWORD);
    radius = 100 as libc::c_int as UDWORD;
    xDif =
        radius.wrapping_mul(*aSinTable.as_mut_ptr().offset((((65536 as
                                                                  libc::c_int
                                                                  /
                                                                  360 as
                                                                      libc::c_int)
                                                                 as
                                                                 libc::c_uint).wrapping_mul(val)
                                                                as uint16 as
                                                                libc::c_int >>
                                                                4 as
                                                                    libc::c_int)
                                                               as isize) as
                                libc::c_uint);
    yDif =
        radius.wrapping_mul(*aSinTable.as_mut_ptr().offset(((((65536 as
                                                                   libc::c_int
                                                                   /
                                                                   360 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint).wrapping_mul(val)
                                                                 as uint16 as
                                                                 libc::c_int
                                                                 >>
                                                                 4 as
                                                                     libc::c_int)
                                                                +
                                                                1024 as
                                                                    libc::c_int)
                                                               as isize) as
                                libc::c_uint);
    xDif = xDif.wrapping_div(4096 as libc::c_int as libc::c_uint);
    yDif = yDif.wrapping_div(4096 as libc::c_int as libc::c_uint);
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_CornerBox(x as SDWORD, y as SDWORD, x2 as SDWORD, y2 as SDWORD, trans,
                  (20 as libc::c_int as
                       libc::c_uint).wrapping_add(radius.wrapping_add(xDif))
                      as UBYTE,
                  (20 as libc::c_int as
                       libc::c_uint).wrapping_add(radius.wrapping_add(yDif))
                      as UBYTE,
                  (20 as libc::c_int as
                       libc::c_uint).wrapping_add(radius.wrapping_sub(xDif))
                      as UBYTE,
                  (20 as libc::c_int as
                       libc::c_uint).wrapping_add(radius.wrapping_sub(yDif))
                      as UBYTE);
    /*
	val = 360-getTimeValueRange(2880,360);
	xDif = radius * (SIN(DEG(val)));
	yDif = radius * (COS(DEG(val)));

	xDif = xDif/4096;	 // cos it's fixed point
	yDif = yDif/4096;
//	pie_BoxFill(100,100,200,200,234);
   	pie_CornerBox(x,y,x2,y2,trans,20+(radius+xDif),20+(radius+yDif),20+(radius-xDif),20+(radius-yDif));
   	pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
	*/
}
