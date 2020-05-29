use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Append SRC onto DEST.  */
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Compare S1 and S2.  */
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    /* Compare no more than N chars of S1 and S2, ignoring case.  */
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char,
                   _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn loadFile(pFileName: *const libc::c_char,
                ppFileData: *mut *mut libc::c_char, pFileSize: *mut UDWORD)
     -> BOOL;
    #[no_mangle]
    static mut rendSurface: iSurface;
    #[no_mangle]
    fn iV_Error(n: libc::c_long, msge: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn iV_GetTexture(filename: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn InitNode(psBSPNode: PSBSPTREENODE) -> PSBSPTREENODE;
    #[no_mangle]
    fn GetPlane(s: *mut iIMDShape, PolygonID: UDWORD, psPlane: PSPLANE);
    // kludge
    #[no_mangle]
    fn pie_SurfaceNormal(p1: *mut iVector, p2: *mut iVector, p3: *mut iVector,
                         v: *mut iVector);
}
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
 *	ISO C99 Standard 7.4: Character handling	<ctype.h>
 */
/* These are all the characteristics of characters.
   If there get to be more than 16 distinct characteristics,
   many things must be changed that use `unsigned short int's.

   The characteristics are stored always in network byte order (big
   endian).  We define the bit value interpretations here dependent on the
   machine's byte order.  */
/* __BYTE_ORDER == __LITTLE_ENDIAN */
pub type C2RustUnnamed = libc::c_uint;
/* Alphanumeric.  */
/* Punctuation.  */
pub const _ISalnum: C2RustUnnamed = 8;
/* Control character.  */
pub const _ISpunct: C2RustUnnamed = 4;
/* Blank (usually SPC and TAB).  */
pub const _IScntrl: C2RustUnnamed = 2;
/* Graphical.  */
pub const _ISblank: C2RustUnnamed = 1;
/* Printing.  */
pub const _ISgraph: C2RustUnnamed = 32768;
/* Whitespace.  */
pub const _ISprint: C2RustUnnamed = 16384;
/* Hexadecimal numeric.  */
pub const _ISspace: C2RustUnnamed = 8192;
/* Numeric.  */
pub const _ISxdigit: C2RustUnnamed = 4096;
/* Alphabetic.  */
pub const _ISdigit: C2RustUnnamed = 2048;
/* lowercase.  */
pub const _ISalpha: C2RustUnnamed = 1024;
/* UPPERCASE.  */
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_uint;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
 * pieTypes.h
 *
 * type defines for simple pies.
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
// Simple derived types
//
//*************************************************************************
pub type iBool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVectorf {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
pub type BSPPOLYID = uint16;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
pub type PSPLANE = *mut PLANE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
pub type VERTEXID = libc::c_int;
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
/* **************************************************************************/
/*
 * imdload.c
 *
 * updated to load version 4 files
 *
 * changes at version 4;
 *		pcx name as string
 *		pcx filepath
 *		cut down vertex list
 *
 */
/* **************************************************************************/
//#include "piematrix.h" //for surface normals
// Static variables
static mut _IMD_FLAGS: uint32 = 0;
static mut _IMD_NAME: [libc::c_char; 256] = [0; 256];
static mut _IMD_VER: int32 = 0;
static mut vertexTable: [VERTEXID; 512] = [0; 512];
static mut imagePath: [libc::c_char; 256] =
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// convert a string to lower case... by Tim ... dedicated to JS ... with love
#[no_mangle]
pub unsafe extern "C" fn strlwr(mut String: *mut STRING) {
    while *String as libc::c_int != 0 as libc::c_int {
        // loop around till we reach the end of the zero terminated string
        if *String as libc::c_int >= 'A' as i32 &&
               *String as libc::c_int <= 'Z' as i32 {
            // if the current letter is in upper case...
            *String =
                (*String as libc::c_int + ('a' as i32 - 'A' as i32)) as STRING
            // ... convert it to lower case
        }
        String = String.offset(1)
        // move to the next letter
    };
}
#[no_mangle]
pub unsafe extern "C" fn AtEndOfFile(mut CurPos: *mut STRING,
                                     mut EndOfFile: *mut STRING) -> BOOL {
    while *CurPos as libc::c_int == 0x9 as libc::c_int ||
              *CurPos as libc::c_int == 0xa as libc::c_int ||
              *CurPos as libc::c_int == 0xd as libc::c_int ||
              *CurPos as libc::c_int == 0x20 as libc::c_int ||
              *CurPos as libc::c_int == 0 as libc::c_int {
        CurPos = CurPos.offset(1);
        if CurPos >= EndOfFile { return 1 as libc::c_int }
    }
    if CurPos >= EndOfFile {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
//*************************************************************************
//*** load IMD shape
//*
//* params	filename = IMD file to load (including extention)
//*
//* returns	pointer to imd shape def
//*
//******
#[no_mangle]
pub static mut TESTDEBUG: BOOL = 0 as libc::c_int;
// load the polygon level ... then load the texture     .... Gareths code
#[no_mangle]
pub unsafe extern "C" fn iV_IMDLoad(mut filename: *mut STRING,
                                    mut palkeep: iBool) -> *mut iIMDShape {
    let mut pIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut pFileData: *mut STRING = 0 as *mut STRING;
    let mut pFileDataStart: *mut STRING = 0 as *mut STRING;
    let mut FileSize: UDWORD = 0;
    let mut res: BOOL = 0;
    let mut path: [libc::c_char; 256] = [0; 256];
    let mut tp: *mut UDWORD = 0 as *mut UDWORD;
    let mut tt: UDWORD = 0;
    strcpy(_IMD_NAME.as_mut_ptr(), filename);
    strlwr(_IMD_NAME.as_mut_ptr());
    _imd_get_path(filename, path.as_mut_ptr());
    if strlen(path.as_mut_ptr()) != 0 as libc::c_int as libc::c_uint {
        if strlen(imagePath.as_mut_ptr()) != 0 as libc::c_int as libc::c_uint
           {
            if strlen(path.as_mut_ptr()).wrapping_add(strlen(imagePath.as_mut_ptr()))
                   > 256 as libc::c_int as libc::c_uint {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(iv_IMDLoad) image path too long for load file\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
                return 0 as *mut iIMDShape
            }
            strcat(imagePath.as_mut_ptr(), path.as_mut_ptr());
        }
    }
    res = loadFile(_IMD_NAME.as_mut_ptr(), &mut pFileData, &mut FileSize);
    if res == 0 as libc::c_int {
        iV_Error(0xff as libc::c_int as libc::c_long,
                 b"(iv_IMDLoad) unable to load file\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
        return 0 as *mut iIMDShape
    }
    tp = pFileData as *mut UDWORD;
    tt = *tp;
    //         'BPIE'
    if tt == 0x45495042 as libc::c_int as libc::c_uint {
        memFreeRelease(pFileData as *mut libc::c_void); // free the file up
        pFileData = 0 as *mut STRING; // free the file up
        return 0 as *mut iIMDShape
    }
    pFileDataStart = pFileData;
    pIMD =
        iV_ProcessIMD(&mut pFileData, pFileData.offset(FileSize as isize),
                      path.as_mut_ptr(), imagePath.as_mut_ptr(), palkeep);
    memFreeRelease(pFileDataStart as *mut libc::c_void);
    pFileDataStart = 0 as *mut STRING;
    return pIMD;
}
static mut IMDcount: UDWORD = 0 as libc::c_int as UDWORD;
static mut IMDPolycount: UDWORD = 0 as libc::c_int as UDWORD;
static mut IMDVertexcount: UDWORD = 0 as libc::c_int as UDWORD;
static mut IMDPoints: UDWORD = 0 as libc::c_int as UDWORD;
static mut IMDTexAnims: UDWORD = 0 as libc::c_int as UDWORD;
static mut IMDConnectors: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub unsafe extern "C" fn DumpIMDInfo() {
    debug(LOG_NEVER,
          b"imds loaded    =%d - using %d bytes\n\x00" as *const u8 as
              *const libc::c_char, IMDcount,
          IMDcount.wrapping_mul(::std::mem::size_of::<iIMDShape>() as
                                    libc::c_ulong));
    debug(LOG_NEVER,
          b"polys loaded   =%d - using %d bytes\n\x00" as *const u8 as
              *const libc::c_char, IMDPolycount,
          IMDPolycount.wrapping_mul(::std::mem::size_of::<iIMDPoly>() as
                                        libc::c_ulong));
    debug(LOG_NEVER,
          b"vertices loaded=%d - using %d bytes\n\x00" as *const u8 as
              *const libc::c_char, IMDVertexcount,
          IMDVertexcount.wrapping_mul((::std::mem::size_of::<VERTEXID>() as
                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<iVertex>()
                                                                           as
                                                                           libc::c_ulong)));
    debug(LOG_NEVER,
          b"points loaded  =%d - using %d bytes\n\x00" as *const u8 as
              *const libc::c_char, IMDPoints,
          IMDPoints.wrapping_mul(::std::mem::size_of::<iVector>() as
                                     libc::c_ulong));
    debug(LOG_NEVER,
          b"connectors     =%d - using %d bytes\n\x00" as *const u8 as
              *const libc::c_char, IMDConnectors,
          IMDConnectors.wrapping_mul(::std::mem::size_of::<iVector>() as
                                         libc::c_ulong));
}
static mut texfile: [STRING; 64] = [0; 64];
//Last loaded texture page filename
#[no_mangle]
pub unsafe extern "C" fn GetLastLoadedTexturePage() -> *mut libc::c_char {
    return texfile.as_mut_ptr();
}
// ppFileData is incremented to the end of the file on exit!
#[no_mangle]
pub unsafe extern "C" fn iV_ProcessIMD(mut ppFileData: *mut *mut STRING,
                                       mut FileDataEnd: *mut STRING,
                                       mut IMDpath: *mut STRING,
                                       mut PCXpath: *mut STRING,
                                       mut palkeep: iBool) -> *mut iIMDShape {
    let mut pFileData: *mut STRING = *ppFileData; //, *str;
    let mut cnt: libc::c_int = 0;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    let mut texType: [libc::c_char; 256] = [0; 256];
    let mut ch: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut nlevels: libc::c_int = 0;
    let mut ptype: libc::c_int = 0;
    let mut pwidth: libc::c_int = 0;
    let mut pheight: libc::c_int = 0;
    let mut texpage: libc::c_int = 0;
    let mut s: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psShape: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut bColourKey: BOOL = 1 as libc::c_int;
    let mut bTextured: BOOL = 0 as libc::c_int;
    let mut level: UDWORD = 0;
    IMDcount = IMDcount.wrapping_add(1);
    if sscanf(pFileData, b"%s %d%n\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr(), &mut _IMD_VER as *mut int32,
              &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD file corrupt -A (%s)\x00" as *const u8 as
                  *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    pFileData = pFileData.offset(cnt as isize);
    if strcmp(b"IMD\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr()) != 0 as libc::c_int &&
           strcmp(b"PIE\x00" as *const u8 as *const libc::c_char,
                  buffer.as_mut_ptr()) != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: not an IMD file (%s %d)\x00" as *const u8 as
                  *const libc::c_char, buffer.as_mut_ptr(), _IMD_VER);
        return 0 as *mut iIMDShape
    }
    //Now supporting version 4 files
    if _IMD_VER < 1 as libc::c_int || _IMD_VER > 4 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: file version not supported (%s)\x00" as
                  *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    if sscanf(pFileData, b"%s %x%n\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr(), &mut _IMD_FLAGS as *mut uint32,
              &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: file corrupt -B (%s)\x00" as *const u8 as
                  *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    pFileData = pFileData.offset(cnt as isize);
    texpage = -(1 as libc::c_int);
    // get texture page if specified
    if _IMD_FLAGS & 0x200 as libc::c_int as libc::c_uint != 0 {
        if _IMD_VER == 1 as libc::c_int {
            if sscanf(pFileData,
                      b"%s %d %s %d %d%n\x00" as *const u8 as
                          *const libc::c_char, buffer.as_mut_ptr(),
                      &mut ptype as *mut libc::c_int, texfile.as_mut_ptr(),
                      &mut pwidth as *mut libc::c_int,
                      &mut pheight as *mut libc::c_int,
                      &mut cnt as *mut libc::c_int) != 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"iV_ProcessIMD: file corrupt -C (%s)\x00" as *const u8
                          as *const libc::c_char, buffer.as_mut_ptr());
                return 0 as *mut iIMDShape
            }
            pFileData = pFileData.offset(cnt as isize);
            if strcmp(buffer.as_mut_ptr(),
                      b"TEXTURE\x00" as *const u8 as *const libc::c_char) !=
                   0 as libc::c_int {
                debug(LOG_ERROR,
                      b"iV_ProcessIMD: expecting \'TEXTURE\' directive (%s)\x00"
                          as *const u8 as *const libc::c_char,
                      buffer.as_mut_ptr());
                return 0 as *mut iIMDShape
            }
            bTextured = 1 as libc::c_int
        } else {
            //version 2 copes with long file names
            if sscanf(pFileData,
                      b"%s %d%n\x00" as *const u8 as *const libc::c_char,
                      buffer.as_mut_ptr(), &mut ptype as *mut libc::c_int,
                      &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
                debug(LOG_ERROR,
                      b"iV_ProcessIMD: file corrupt -D (%s)\x00" as *const u8
                          as *const libc::c_char, buffer.as_mut_ptr());
                return 0 as *mut iIMDShape
            }
            pFileData = pFileData.offset(cnt as isize);
            if strcmp(buffer.as_mut_ptr(),
                      b"TEXTURE\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                let fresh0 = pFileData;
                pFileData = pFileData.offset(1);
                ch = *fresh0;
                i = 0 as libc::c_int;
                while i < 80 as libc::c_int &&
                          {
                              let fresh1 = pFileData;
                              pFileData = pFileData.offset(1);
                              ch = *fresh1;
                              (ch as libc::c_int) != -(1 as libc::c_int)
                          } && ch as libc::c_int != '.' as i32 {
                    // yummy
                    texfile[i as usize] = ch;
                    i += 1
                }
                if sscanf(pFileData,
                          b"%s%n\x00" as *const u8 as *const libc::c_char,
                          texType.as_mut_ptr(), &mut cnt as *mut libc::c_int)
                       != 1 as libc::c_int {
                    debug(LOG_ERROR,
                          b"iV_ProcessIMD: file corrupt -E (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          buffer.as_mut_ptr());
                    return 0 as *mut iIMDShape
                }
                pFileData = pFileData.offset(cnt as isize);
                if strcmp(texType.as_mut_ptr(),
                          b"png\x00" as *const u8 as *const libc::c_char) !=
                       0 as libc::c_int {
                    debug(LOG_ERROR,
                          b"iV_ProcessIMD: file corrupt -F (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          buffer.as_mut_ptr());
                    return 0 as *mut iIMDShape
                }
                texfile[i as usize] = 0 as libc::c_int as STRING;
                strcat(texfile.as_mut_ptr(),
                       b".png\x00" as *const u8 as *const libc::c_char);
                if sscanf(pFileData,
                          b"%d %d%n\x00" as *const u8 as *const libc::c_char,
                          &mut pwidth as *mut libc::c_int,
                          &mut pheight as *mut libc::c_int,
                          &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
                    debug(LOG_ERROR,
                          b"iV_ProcessIMD: file corrupt -G (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          buffer.as_mut_ptr());
                    return 0 as *mut iIMDShape
                }
                pFileData = pFileData.offset(cnt as isize);
                bTextured = 1 as libc::c_int
            } else if strcmp(buffer.as_mut_ptr(),
                             b"NOTEXTURE\x00" as *const u8 as
                                 *const libc::c_char) == 0 as libc::c_int {
                if sscanf(pFileData,
                          b"%s %d %d%n\x00" as *const u8 as
                              *const libc::c_char, texfile.as_mut_ptr(),
                          &mut pwidth as *mut libc::c_int,
                          &mut pheight as *mut libc::c_int,
                          &mut cnt as *mut libc::c_int) != 3 as libc::c_int {
                    debug(LOG_ERROR,
                          b"iV_ProcessIMD: file corrupt -H (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          buffer.as_mut_ptr());
                    return 0 as *mut iIMDShape
                }
                pFileData = pFileData.offset(cnt as isize)
            } else {
                debug(LOG_ERROR,
                      b"iV_ProcessIMD(2): expecting \'TEXTURE\' directive (%s)\x00"
                          as *const u8 as *const libc::c_char,
                      buffer.as_mut_ptr());
                return 0 as *mut iIMDShape
            }
        }
        // The BSP tool should not reduce the texture page name down (please)
        // Super scrummy hack to reduce texture page names down to the page id
        if bTextured != 0 {
            //resToLower(texfile);
//          printf("texfile cmp in imdload.c :%s\n", texfile);
            if strncasecmp(texfile.as_mut_ptr(),
                           b"page-\x00" as *const u8 as *const libc::c_char,
                           5 as libc::c_int as libc::c_uint) ==
                   0 as libc::c_int {
                i = 5 as libc::c_int;
                while i < strlen(texfile.as_mut_ptr()) as SDWORD {
                    if *(*__ctype_b_loc()).offset(texfile[i as usize] as
                                                      libc::c_int as isize) as
                           libc::c_int &
                           _ISdigit as libc::c_int as libc::c_ushort as
                               libc::c_int == 0 {
                        break ;
                    }
                    i += 1
                }
                texfile[i as usize] = 0 as libc::c_int as STRING
            }
        }
    }
    if sscanf(pFileData, b"%s %d%n\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr(), &mut nlevels as *mut libc::c_int,
              &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: file corrupt -I (%s)\x00" as *const u8 as
                  *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    pFileData = pFileData.offset(cnt as isize);
    if strcmp(buffer.as_mut_ptr(),
              b"LEVELS\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: expecting \'LEVELS\' directive (%s)\x00" as
                  *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    // if we might have BSP then we need to preread the LEVEL directive
    if sscanf(pFileData, b"%s %d%n\x00" as *const u8 as *const libc::c_char,
              buffer.as_mut_ptr(), &mut level as *mut UDWORD,
              &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
        iV_Error(0xff as libc::c_int as libc::c_long,
                 b"(_load_level) file corrupt -J\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
        return 0 as *mut iIMDShape
    }
    pFileData = pFileData.offset(cnt as isize);
    if strcmp(buffer.as_mut_ptr(),
              b"LEVEL\x00" as *const u8 as *const libc::c_char) !=
           0 as libc::c_int {
        debug(LOG_ERROR,
              b"iV_ProcessIMD(2): expecting \'LEVELS\' directive (%s)\x00" as
                  *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        return 0 as *mut iIMDShape
    }
    s = _imd_load_level(&mut pFileData, FileDataEnd, nlevels, texpage);
    // load texture page if specified
    if !s.is_null() && _IMD_FLAGS & 0x200 as libc::c_int as libc::c_uint != 0
       {
        bColourKey =
            0 as
                libc::c_int; // CheckColourKey( s );//TRUE not the only imd using this texture
        if bTextured != 0 {
            texpage = iV_GetTexture(texfile.as_mut_ptr());
            if texpage < 0 as libc::c_int {
                debug(LOG_ERROR,
                      b"iV_ProcessIMD: could not load tex page %s\x00" as
                          *const u8 as *const libc::c_char,
                      texfile.as_mut_ptr());
                return 0 as *mut iIMDShape
            }
        } else { texpage = -(1 as libc::c_int) }
        /* assign tex page to levels */
        psShape = s;
        while !psShape.is_null() {
            (*psShape).texpage = texpage;
            psShape = (*psShape).next
        }
    }
    if s.is_null() {
        debug(LOG_ERROR,
              b"iV_ProcessIMD: unsuccessful (%s)\x00" as *const u8 as
                  *const libc::c_char, buffer.as_mut_ptr());
    }
    *ppFileData = pFileData;
    return s;
}
//*************************************************************************
//*** load shape level polygons
//*
//* pre		fp open
//*			s allocated, s->npolys set
//*
//* params	fp = currently open shape file pointer
//*			s	= pointer to shape level
//*
//* on exit	s->polys allocated (iFSDPoly * s->npolys
//*			s->pindex allocated for each poly
//* returns	FALSE on error (memory allocation failure/bad file format)
//*
//******
unsafe extern "C" fn _imd_load_polys(mut ppFileData: *mut *mut STRING,
                                     mut FileDataEnd: *mut STRING,
                                     mut s: *mut iIMDShape) -> iBool {
    let mut pFileData: *mut STRING = *ppFileData; //, anim;
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p0: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut p1: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut p2: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut points: *mut iVector = 0 as *mut iVector;
    let mut poly: *mut iIMDPoly = 0 as *mut iIMDPoly;
    let mut nFrames: libc::c_int = 0;
    let mut pbRate: libc::c_int = 0;
    let mut tWidth: libc::c_int = 0;
    let mut tHeight: libc::c_int = 0;
    //assumes points already set
    points = (*s).points;
    IMDPolycount =
        (IMDPolycount as
             libc::c_uint).wrapping_add((*s).npolys as libc::c_uint) as UDWORD
            as UDWORD;
    (*s).numFrames = 0 as libc::c_int as UWORD;
    (*s).animInterval = 0 as libc::c_int as UWORD;
    (*s).polys =
        memMallocRelease((::std::mem::size_of::<iIMDPoly>() as
                              libc::c_ulong).wrapping_mul((*s).npolys as
                                                              libc::c_uint))
            as *mut iIMDPoly;
    if !(*s).polys.is_null() {
        poly = (*s).polys;
        i = 0 as libc::c_int;
        while i < (*s).npolys {
            let mut flags: UDWORD = 0;
            let mut npnts: UDWORD = 0;
            if sscanf(pFileData,
                      b"%x %d%n\x00" as *const u8 as *const libc::c_char,
                      &mut flags as *mut UDWORD, &mut npnts as *mut UDWORD,
                      &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_polys) [poly %d] error loading flags and npoints\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char, i);
            }
            pFileData = pFileData.offset(cnt as isize);
            (*poly).flags = flags;
            if flags & 0x2000 as libc::c_int as libc::c_uint != 0 {
                (*s).flags |= 0x20 as libc::c_int as libc::c_uint
            }
            (*poly).npnts = npnts as libc::c_int;
            IMDVertexcount =
                (IMDVertexcount as
                     libc::c_uint).wrapping_add((*poly).npnts as libc::c_uint)
                    as UDWORD as UDWORD;
            (*poly).pindex =
                memMallocRelease((::std::mem::size_of::<VERTEXID>() as
                                      libc::c_ulong).wrapping_mul((*poly).npnts
                                                                      as
                                                                      libc::c_uint))
                    as *mut VERTEXID;
            (*poly).vrt =
                memMallocRelease((::std::mem::size_of::<iVertex>() as
                                      libc::c_ulong).wrapping_mul((*poly).npnts
                                                                      as
                                                                      libc::c_uint))
                    as *mut iVertex;
            if (*poly).vrt.is_null() {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_polys) [poly %d] memory alloc fail (vertex struct)\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char, i);
                return 0 as libc::c_int
            }
            if !(*poly).pindex.is_null() {
                j = 0 as libc::c_int;
                while j < (*poly).npnts {
                    let mut NewID: libc::c_int = 0;
                    if sscanf(pFileData,
                              b"%d%n\x00" as *const u8 as *const libc::c_char,
                              &mut NewID as *mut libc::c_int,
                              &mut cnt as *mut libc::c_int) !=
                           1 as libc::c_int {
                        debug(LOG_NEVER,
                              b"failed poly %d. point %d [%s]\n\x00" as
                                  *const u8 as *const libc::c_char, i, j,
                              _IMD_NAME.as_mut_ptr());
                        iV_Error(0xff as libc::c_int as libc::c_long,
                                 b"(_load_polys) [poly %d] error reading poly indices\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char, i);
                        return 0 as libc::c_int
                    }
                    pFileData = pFileData.offset(cnt as isize);
                    *(*poly).pindex.offset(j as isize) =
                        vertexTable[NewID as usize];
                    j += 1
                }
            } else {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_polys) [poly %d] memory alloc fail (poly indices)\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char, i);
                return 0 as libc::c_int
            }
            // make it end end of the BSP chain by default
            if (*poly).npnts > 2 as libc::c_int {
                p0.x =
                    (*points.offset(*(*poly).pindex.offset(0 as libc::c_int as
                                                               isize) as
                                        isize)).x;
                p0.y =
                    (*points.offset(*(*poly).pindex.offset(0 as libc::c_int as
                                                               isize) as
                                        isize)).y;
                p0.z =
                    (*points.offset(*(*poly).pindex.offset(0 as libc::c_int as
                                                               isize) as
                                        isize)).z;
                p1.x =
                    (*points.offset(*(*poly).pindex.offset(1 as libc::c_int as
                                                               isize) as
                                        isize)).x;
                p1.y =
                    (*points.offset(*(*poly).pindex.offset(1 as libc::c_int as
                                                               isize) as
                                        isize)).y;
                p1.z =
                    (*points.offset(*(*poly).pindex.offset(1 as libc::c_int as
                                                               isize) as
                                        isize)).z;
                p2.x =
                    (*points.offset(*(*poly).pindex.offset(((*poly).npnts -
                                                                1 as
                                                                    libc::c_int)
                                                               as isize) as
                                        isize)).x;
                p2.y =
                    (*points.offset(*(*poly).pindex.offset(((*poly).npnts -
                                                                1 as
                                                                    libc::c_int)
                                                               as isize) as
                                        isize)).y;
                p2.z =
                    (*points.offset(*(*poly).pindex.offset(((*poly).npnts -
                                                                1 as
                                                                    libc::c_int)
                                                               as isize) as
                                        isize)).z;
                pie_SurfaceNormal(&mut p0, &mut p1, &mut p2,
                                  &mut (*poly).normal);
                // calc poly normal
                //iV_DEBUG3("normal %d %d %d\n",poly->normal.x,poly->normal.y,poly->normal.z);
            } else {
                (*poly).normal.z = 0 as libc::c_int;
                (*poly).normal.y = (*poly).normal.z;
                (*poly).normal.x = (*poly).normal.y
            }
            if (*poly).flags & 0x4000 as libc::c_int as libc::c_uint != 0 {
                IMDTexAnims = IMDTexAnims.wrapping_add(1);
                (*poly).pTexAnim =
                    memMallocRelease(::std::mem::size_of::<iTexAnim>() as
                                         libc::c_ulong) as *mut iTexAnim;
                if (*poly).pTexAnim.is_null() {
                    iV_Error(0xff as libc::c_int as libc::c_long,
                             b"(_load_polys) [poly %d] memory alloc fail (iTexAnim struct)\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, i);
                    return 0 as libc::c_int
                }
                // even the psx needs to skip the data
                if sscanf(pFileData,
                          b"%d %d %d %d%n\x00" as *const u8 as
                              *const libc::c_char,
                          &mut nFrames as *mut libc::c_int,
                          &mut pbRate as *mut libc::c_int,
                          &mut tWidth as *mut libc::c_int,
                          &mut tHeight as *mut libc::c_int,
                          &mut cnt as *mut libc::c_int) != 4 as libc::c_int {
                    iV_Error(0xff as libc::c_int as libc::c_long,
                             b"(_load_polys) [poly %d] error reading texanim data\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, i);
                    return 0 as libc::c_int
                }
                pFileData = pFileData.offset(cnt as isize);
                if tWidth > 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"_imd_load_polys: texture width = %i\x00" as
                              *const u8 as *const libc::c_char, tWidth);
                };
                if tWidth > 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"imdload.c\x00" as *const u8 as
                              *const libc::c_char, 466 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"_imd_load_polys\x00")).as_ptr(),
                          b"tWidth>0\x00" as *const u8 as
                              *const libc::c_char);
                };
                if tHeight > 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"_imd_load_polys: texture height = %i\x00" as
                              *const u8 as *const libc::c_char, tHeight);
                };
                if tHeight > 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"imdload.c\x00" as *const u8 as
                              *const libc::c_char, 467 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"_imd_load_polys\x00")).as_ptr(),
                          b"tHeight>0\x00" as *const u8 as
                              *const libc::c_char);
                };
                (*(*poly).pTexAnim).nFrames = nFrames;
                /* Assumes same number of frames per poly */
                (*s).numFrames = nFrames as UWORD;
                (*(*poly).pTexAnim).playbackRate = pbRate;
                /* Uses Max metric playback rate */
                (*s).animInterval = pbRate as UWORD;
                (*(*poly).pTexAnim).textureWidth = tWidth;
                (*(*poly).pTexAnim).textureHeight = tHeight
            } else { (*poly).pTexAnim = 0 as *mut iTexAnim }
            if !(*poly).vrt.is_null() &&
                   (*poly).flags &
                       (0x200 as libc::c_int | 0x8000 as libc::c_int) as
                           libc::c_uint != 0 {
                j = 0 as libc::c_int;
                while j < (*poly).npnts {
                    let mut VertexU: int32 = 0;
                    let mut VertexV: int32 = 0;
                    if sscanf(pFileData,
                              b"%d %d%n\x00" as *const u8 as
                                  *const libc::c_char,
                              &mut VertexU as *mut int32,
                              &mut VertexV as *mut int32,
                              &mut cnt as *mut libc::c_int) !=
                           2 as libc::c_int {
                        iV_Error(0xff as libc::c_int as libc::c_long,
                                 b"(_load_polys) [poly %d] error reading tex outline\x00"
                                     as *const u8 as *const libc::c_char as
                                     *mut libc::c_char, i);
                        return 0 as libc::c_int
                    }
                    pFileData = pFileData.offset(cnt as isize);
                    (*(*poly).vrt.offset(j as isize)).u = VertexU;
                    (*(*poly).vrt.offset(j as isize)).v = VertexV;
                    (*(*poly).vrt.offset(j as isize)).g =
                        255 as libc::c_int as uint8;
                    j += 1
                }
            }
            (*poly).BSP_NextPoly =
                (65534 as libc::c_int + 1 as libc::c_int) as BSPPOLYID;
            i += 1;
            poly = poly.offset(1)
        }
    } else { return 0 as libc::c_int }
    *ppFileData = pFileData;
    return 1 as libc::c_int;
}
unsafe extern "C" fn _imd_load_bsp(mut ppFileData: *mut *mut STRING,
                                   mut FileDataEnd: *mut STRING,
                                   mut s: *mut iIMDShape,
                                   mut BSPNodeCount: UWORD) -> iBool {
    let mut pFileData: *mut STRING = *ppFileData;
    let mut cnt: libc::c_int = 0;
    let mut Node: UWORD = 0;
    // PC texture coord routine
    let mut NodeList: PSBSPTREENODE =
        0 as *mut BSPTREENODE; // An pointer to an array of  nodes
    let mut IMDTri: *mut iIMDPoly =
        0 as
            *mut iIMDPoly; // pointer to a polygon ... for handling the link list in the bsp
    if (*s).npolys > 65534 as libc::c_int {
        iV_Error(0xff as libc::c_int as libc::c_long,
                 b"(_imd_load_bsp) Too many polygons in IMD for BSP to handle\x00"
                     as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    }
    // Build table of nodes - we sort out the links later
    NodeList =
        memMallocRelease((::std::mem::size_of::<BSPTREENODE>() as
                              libc::c_ulong).wrapping_mul(BSPNodeCount as
                                                              libc::c_uint))
            as PSBSPTREENODE; // Allocate the entire node tree
    memset(NodeList as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<BSPTREENODE>() as
                libc::c_ulong).wrapping_mul(BSPNodeCount as
                                                libc::c_uint)); // Zero it out ... we need to make all pointers NULL
    Node = 0 as libc::c_int as UWORD;
    while (Node as libc::c_int) < BSPNodeCount as libc::c_int {
        let mut psNode: *mut BSPTREENODE = 0 as *mut BSPTREENODE;
        // This could be -1 indicating an empty node
        let mut NodeID: SDWORD = 0; // Temp storage area for a node ID
        let mut PolygonID: SDWORD = 0; // Temp storage area for a polygon ID
        let mut FirstPolygonID: SDWORD =
            0; // This indicates the first polygon in the forward facing BSP list
        psNode = &mut *NodeList.offset(Node as isize) as *mut BSPTREENODE;
        FirstPolygonID = -(1 as libc::c_int);
        InitNode(psNode);
        if sscanf(pFileData, b"%d%n\x00" as *const u8 as *const libc::c_char,
                  &mut NodeID as *mut SDWORD, &mut cnt as *mut libc::c_int) !=
               1 as libc::c_int {
            // Check that we read 1 parameter ok
            iV_Error(0xff as libc::c_int as libc::c_long,
                     b"(_load_bsp) - needed a left node!\x00" as *const u8 as
                         *const libc::c_char as
                         *mut libc::c_char); // This could be -1 indicating an empty node
            return 0 as libc::c_int
        }
        pFileData = pFileData.offset(cnt as isize);
        (*psNode).link[1 as libc::c_int as usize] = NodeID as PSBSPTREENODE;
        loop 
             // Get forward facing polygon list - never empty apart from root node
             {
            if sscanf(pFileData,
                      b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut PolygonID as *mut SDWORD,
                      &mut cnt as *mut libc::c_int) != 1 as libc::c_int {
                // Get a valid polygon number
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - needed a polygon number\x00" as
                             *const u8 as *const libc::c_char as
                             *mut libc::c_char);
                return 0 as libc::c_int
            }
            pFileData = pFileData.offset(cnt as isize);
            if PolygonID == -(1 as libc::c_int) { break ; }
            if PolygonID < 0 as libc::c_int || PolygonID >= (*s).npolys {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - bad polygon number\x00" as *const u8
                             as *const libc::c_char as *mut libc::c_char);
                return 0 as libc::c_int
            }
            if FirstPolygonID == -(1 as libc::c_int) {
                FirstPolygonID = PolygonID
            }
            IMDTri =
                &mut *(*s).polys.offset(PolygonID as isize) as *mut iIMDPoly;
            if (*IMDTri).BSP_NextPoly as libc::c_int !=
                   65534 as libc::c_int + 1 as libc::c_int {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - Polygon is mentioned more than once in the BSP\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
            }
            (*IMDTri).BSP_NextPoly = (*psNode).TriSameDir;
            (*psNode).TriSameDir = PolygonID as BSPPOLYID
        }
        if FirstPolygonID != -(1 as libc::c_int) {
            GetPlane(s, FirstPolygonID as UDWORD, &mut (*psNode).Plane);
        } else {
            memset(&mut (*psNode).Plane as *mut PLANE as *mut libc::c_char as
                       *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<PLANE>() as libc::c_ulong);
            // Generate the plane equation - if weve got any polygons
            // Clear the plane equation
        }
        loop 
             // Get reverse facing polygon list - frequently empty
             {
            if sscanf(pFileData,
                      b"%d%n\x00" as *const u8 as *const libc::c_char,
                      &mut PolygonID as *mut SDWORD,
                      &mut cnt as *mut libc::c_int) != 1 as libc::c_int {
                // Get a valid polygon number
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - needed a polygon number\x00" as
                             *const u8 as *const libc::c_char as
                             *mut libc::c_char);
                return 0 as libc::c_int
            }
            pFileData = pFileData.offset(cnt as isize);
            if PolygonID == -(1 as libc::c_int) { break ; }
            if PolygonID < 0 as libc::c_int || PolygonID >= (*s).npolys {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - bad polygon number\x00" as *const u8
                             as *const libc::c_char as *mut libc::c_char);
                return 0 as libc::c_int
            }
            // Insert into the list
            IMDTri =
                &mut *(*s).polys.offset(PolygonID as isize) as *mut iIMDPoly;
            if (*IMDTri).BSP_NextPoly as libc::c_int !=
                   65534 as libc::c_int + 1 as libc::c_int {
                iV_Error(0xff as libc::c_int as libc::c_long,
                         b"(_load_bsp) - Polygon is mentioned more than once in the BSP\x00"
                             as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
            }
            (*IMDTri).BSP_NextPoly = (*psNode).TriOppoDir;
            (*psNode).TriOppoDir = PolygonID as BSPPOLYID
        }
        if sscanf(pFileData, b"%d%n\x00" as *const u8 as *const libc::c_char,
                  &mut NodeID as *mut SDWORD, &mut cnt as *mut libc::c_int) !=
               1 as libc::c_int {
            // Check that we read 1 parameter ok
            iV_Error(0xff as libc::c_int as libc::c_long,
                     b"(_load_bsp) - needed a right node!\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char);
            return 0 as libc::c_int
        }
        pFileData = pFileData.offset(cnt as isize);
        (*psNode).link[0 as libc::c_int as usize] = NodeID as PSBSPTREENODE;
        Node = Node.wrapping_add(1)
    }
    // Now fix all the links
    Node = 0 as libc::c_int as UWORD;
    while (Node as libc::c_int) < BSPNodeCount as libc::c_int {
        let mut psNode_0: *mut BSPTREENODE = 0 as *mut BSPTREENODE;
        let mut NodeID_0: libc::c_int = 0;
        psNode_0 = &mut *NodeList.offset(Node as isize) as *mut BSPTREENODE;
        if (*psNode_0).link[1 as libc::c_int as usize] as SDWORD ==
               -(1 as libc::c_int) {
            (*psNode_0).link[1 as libc::c_int as usize] =
                0 as *mut BSPTREENODE
            // if its zero then its an empty link
        } else {
            NodeID_0 =
                (*psNode_0).link[1 as libc::c_int as usize] as libc::c_int;
            (*psNode_0).link[1 as libc::c_int as usize] =
                &mut *NodeList.offset(NodeID_0 as isize) as *mut BSPTREENODE
        }
        if (*psNode_0).link[0 as libc::c_int as usize] as SDWORD ==
               -(1 as libc::c_int) {
            (*psNode_0).link[0 as libc::c_int as usize] =
                0 as *mut BSPTREENODE
            // if its zero then its an empty link
        } else {
            NodeID_0 =
                (*psNode_0).link[0 as libc::c_int as usize] as libc::c_int;
            (*psNode_0).link[0 as libc::c_int as usize] =
                &mut *NodeList.offset(NodeID_0 as isize) as *mut BSPTREENODE
        }
        Node = Node.wrapping_add(1)
    }
    // Set the shape node list to the root node ... this can be used to
  // FREE up the BSP memory if we needed to
    (*s).BSPNode =
        &mut *NodeList.offset(0 as libc::c_int as isize) as *mut BSPTREENODE;
    *ppFileData = pFileData;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ReadPoints(mut ppFileData: *mut *mut STRING,
                                    mut FileDataEnd: *mut STRING,
                                    mut s: *mut iIMDShape) -> BOOL {
    let mut pFileData: *mut STRING = *ppFileData;
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut iVector = 0 as *mut iVector;
    let mut lastPoint: libc::c_int = 0;
    let mut match_0: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut newX: SDWORD = 0;
    let mut newY: SDWORD = 0;
    let mut newZ: SDWORD = 0;
    p = (*s).points;
    lastPoint = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*s).npoints {
        if sscanf(pFileData,
                  b"%d %d %d%n\x00" as *const u8 as *const libc::c_char,
                  &mut newX as *mut SDWORD, &mut newY as *mut SDWORD,
                  &mut newZ as *mut SDWORD, &mut cnt as *mut libc::c_int) !=
               3 as libc::c_int {
            iV_Error(0xff as libc::c_int as libc::c_long,
                     b"(_load_points) file corrupt -K\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char);
            return 0 as libc::c_int
        }
        pFileData = pFileData.offset(cnt as isize);
        //		DBPRINTF(("%d) x=%d y=%x z=%d\n",i,newX,newY,newZ));
		//check for duplicate points
        match_0 = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        // scan through list upto the number of points added (lastPoint) ... not up to the number of points scanned in (i)  (which will include duplicates)
        while j < lastPoint && match_0 == -(1 as libc::c_int) {
            //		while((j < i) && (match == -1))
            if newX == (*p.offset(j as isize)).x {
                if newY == (*p.offset(j as isize)).y {
                    if newZ == (*p.offset(j as isize)).z { match_0 = j }
                }
            }
            j += 1
        }
        if match_0 == -(1 as libc::c_int) {
            // new point
            (*p.offset(lastPoint as isize)).x = newX;
            (*p.offset(lastPoint as isize)).y = newY;
            (*p.offset(lastPoint as isize)).z = newZ;
            vertexTable[i as usize] = lastPoint;
            lastPoint += 1
        } else { vertexTable[i as usize] = match_0 }
        i += 1
    }
    //clear remaining table
    i = (*s).npoints;
    while i < 512 as libc::c_int {
        vertexTable[i as usize] = -(1 as libc::c_int);
        i += 1
    }
    (*s).npoints = lastPoint;
    *ppFileData = pFileData;
    return 1 as libc::c_int;
}
//*************************************************************************
//*** load shape level vertices
//*
//* pre		fp open
//*			s allocated, s->npoints set
//*
//* params	fp 		= currently open shape file pointer
//*			s			= pointer to shape level
//*
//* on exit	s->points allocated (iVector * s->npoints
//* returns	FALSE on error (memory allocation failure/bad file format)
//*
//******
unsafe extern "C" fn _imd_load_points(mut ppFileData: *mut *mut STRING,
                                      mut FileDataEnd: *mut STRING,
                                      mut s: *mut iIMDShape) -> iBool {
    let mut i: libc::c_int = 0;
    let mut p: *mut iVector = 0 as *mut iVector;
    let mut tempXMax: int32 = 0;
    let mut tempXMin: int32 = 0;
    let mut tempZMax: int32 = 0;
    let mut tempZMin: int32 = 0;
    let mut extremeX: int32 = 0;
    let mut extremeZ: int32 = 0;
    let mut xmax: int32 = 0;
    let mut ymax: int32 = 0;
    let mut zmax: int32 = 0;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    let mut rad_sq: libc::c_double = 0.;
    let mut rad: libc::c_double = 0.;
    let mut old_to_p_sq: libc::c_double = 0.;
    let mut old_to_p: libc::c_double = 0.;
    let mut old_to_new: libc::c_double = 0.;
    let mut xspan: libc::c_double = 0.;
    let mut yspan: libc::c_double = 0.;
    let mut zspan: libc::c_double = 0.;
    let mut maxspan: libc::c_double = 0.;
    let mut dia1: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut dia2: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut cen: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut vxmin: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    let mut vymin: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    let mut vzmin: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    let mut vxmax: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    let mut vymax: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    let mut vzmax: iVectorf =
        {
            let mut init =
                iVectorf{x: 0 as libc::c_int as libc::c_double,
                         y: 0 as libc::c_int as libc::c_double,
                         z: 0 as libc::c_int as libc::c_double,};
            init
        };
    //load the points then pass through a second time to setup bounding datavalues
    IMDPoints =
        (IMDPoints as libc::c_uint).wrapping_add((*s).npoints as libc::c_uint)
            as UDWORD as UDWORD;
    p =
        memMallocRelease((::std::mem::size_of::<iVector>() as
                              libc::c_ulong).wrapping_mul((*s).npoints as
                                                              libc::c_uint))
            as *mut iVector;
    (*s).points = p;
    if p.is_null() { return 0 as libc::c_int }
    // Read in points and remove duplicates (!)
    if ReadPoints(ppFileData, FileDataEnd, s) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    tempZMax = -((1 as libc::c_int) << 12 as libc::c_int);
    tempXMax = tempZMax;
    (*s).zmax = tempXMax;
    (*s).ymax = (*s).zmax;
    (*s).xmax = (*s).ymax;
    tempZMin = (1 as libc::c_int) << 12 as libc::c_int;
    tempXMin = tempZMin;
    (*s).zmin = tempXMin;
    (*s).ymin = (*s).zmin;
    (*s).xmin = (*s).ymin;
    vzmax.z = -((1 as libc::c_int) << 12 as libc::c_int) as libc::c_double;
    vymax.y = vzmax.z;
    vxmax.x = vymax.y;
    vzmin.z = ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_double;
    vymin.y = vzmin.z;
    vxmin.x = vymin.y;
    // set up bounding data for minimum number of vertices
    i = 0 as libc::c_int;
    while i < (*s).npoints {
        if (*p).x > (*s).xmax { (*s).xmax = (*p).x }
        if (*p).x < (*s).xmin { (*s).xmin = (*p).x }
        /* Biggest x coord so far within our height window? */
        if (*p).x > tempXMax && (*p).y > 10 as libc::c_int &&
               (*p).y < 100 as libc::c_int {
            tempXMax = (*p).x
        }
        /* Smallest x coord so far within our height window? */
        if (*p).x < tempXMin && (*p).y > 10 as libc::c_int &&
               (*p).y < 100 as libc::c_int {
            tempXMin = (*p).x
        }
        if (*p).y > (*s).ymax { (*s).ymax = (*p).y }
        if (*p).y < (*s).ymin { (*s).ymin = (*p).y }
        if (*p).z > (*s).zmax { (*s).zmax = (*p).z }
        if (*p).z < (*s).zmin { (*s).zmin = (*p).z }
        /* Biggest z coord so far within our height window? */
        if (*p).z > tempZMax && (*p).y > 10 as libc::c_int &&
               (*p).y < 100 as libc::c_int {
            tempZMax = (*p).z
        }
        /* Smallest z coord so far within our height window? */
        if (*p).z < tempZMax && (*p).y > 10 as libc::c_int &&
               (*p).y < 100 as libc::c_int {
            tempZMin = (*p).z
        }
        // for tight sphere calculations
        if ((*p).x as libc::c_double) < vxmin.x {
            vxmin.x = (*p).x as libc::c_double;
            vxmin.y = (*p).y as libc::c_double;
            vxmin.z = (*p).z as libc::c_double
        }
        if (*p).x as libc::c_double > vxmax.x {
            vxmax.x = (*p).x as libc::c_double;
            vxmax.y = (*p).y as libc::c_double;
            vxmax.z = (*p).z as libc::c_double
        }
        if ((*p).y as libc::c_double) < vymin.y {
            vymin.x = (*p).x as libc::c_double;
            vymin.y = (*p).y as libc::c_double;
            vymin.z = (*p).z as libc::c_double
        }
        if (*p).y as libc::c_double > vymax.y {
            vymax.x = (*p).x as libc::c_double;
            vymax.y = (*p).y as libc::c_double;
            vymax.z = (*p).z as libc::c_double
        }
        if ((*p).z as libc::c_double) < vzmin.z {
            vzmin.x = (*p).x as libc::c_double;
            vzmin.y = (*p).y as libc::c_double;
            vzmin.z = (*p).z as libc::c_double
        }
        if (*p).z as libc::c_double > vzmax.z {
            vzmax.x = (*p).x as libc::c_double;
            vzmax.y = (*p).y as libc::c_double;
            vzmax.z = (*p).z as libc::c_double
        }
        i += 1;
        p = p.offset(1)
    }
    /* Centered about origin I can do the '-' thing below!! */
    extremeX = if tempXMax > -tempXMin { tempXMax } else { -tempXMin };
    extremeZ = if tempZMax > -tempZMin { tempZMax } else { -tempZMin };
    (*s).visRadius = if extremeX > extremeZ { extremeX } else { extremeZ };
    // no need to scale an IMD shape (only FSD)
    xmax = if (*s).xmax > -(*s).xmin { (*s).xmax } else { -(*s).xmin };
    ymax = if (*s).ymax > -(*s).ymin { (*s).ymax } else { -(*s).ymin };
    zmax = if (*s).zmax > -(*s).zmin { (*s).zmax } else { -(*s).zmin };
    (*s).radius =
        if xmax > (if ymax > zmax { ymax } else { zmax }) {
            xmax
        } else if ymax > zmax { ymax } else { zmax };
    (*s).sradius =
        sqrt((xmax * xmax + ymax * ymax + zmax * zmax) as libc::c_double) as
            libc::c_float as SDWORD;
    // START: tight bounding sphere
    // set xspan = distance between 2 points xmin & xmax (squared)
    dx = vxmax.x - vxmin.x;
    dy = vxmax.y - vxmin.y;
    dz = vxmax.z - vxmin.z;
    xspan = dx * dx + dy * dy + dz * dz;
    // same for yspan
    dx = vymax.x - vymin.x;
    dy = vymax.y - vymin.y;
    dz = vymax.z - vymin.z;
    yspan = dx * dx + dy * dy + dz * dz;
    // and ofcourse zspan
    dx = vzmax.x - vzmin.x;
    dy = vzmax.y - vzmin.y;
    dz = vzmax.z - vzmin.z;
    zspan = dx * dx + dy * dy + dz * dz;
    // set points dia1 & dia2 to maximally seperated pair
    // assume xspan biggest
    dia1 = vxmin;
    dia2 = vxmax;
    maxspan = xspan;
    if yspan > maxspan { maxspan = yspan; dia1 = vymin; dia2 = vymax }
    if zspan > maxspan { maxspan = zspan; dia1 = vzmin; dia2 = vzmax }
    // dia1, dia2 diameter of initial sphere
    cen.x = (dia1.x + dia2.x) / 2.0f64;
    cen.y = (dia1.y + dia2.y) / 2.0f64;
    cen.z = (dia1.z + dia2.z) / 2.0f64;
    // calc initial radius
    dx = dia2.x - cen.x;
    dy = dia2.y - cen.y;
    dz = dia2.z - cen.z;
    rad_sq = dx * dx + dy * dy + dz * dz;
    rad = sqrt(rad_sq);
    // second pass (find tight sphere)
    p = (*s).points;
    i = 0 as libc::c_int;
    while i < (*s).npoints {
        dx = (*p).x as libc::c_double - cen.x;
        dy = (*p).y as libc::c_double - cen.y;
        dz = (*p).z as libc::c_double - cen.z;
        old_to_p_sq = dx * dx + dy * dy + dz * dz;
        // do r**2 first
        if old_to_p_sq > rad_sq {
            // this point outside current sphere
            old_to_p = sqrt(old_to_p_sq);
            // radius of new sphere
            rad = (rad + old_to_p) / 2.0f64;
            // rad**2 for next compare
            rad_sq = rad * rad;
            old_to_new = old_to_p - rad;
            // centre of new sphere
            cen.x =
                (rad * cen.x + old_to_new * (*p).x as libc::c_double) /
                    old_to_p;
            cen.y =
                (rad * cen.y + old_to_new * (*p).y as libc::c_double) /
                    old_to_p;
            cen.z =
                (rad * cen.z + old_to_new * (*p).z as libc::c_double) /
                    old_to_p
        }
        i += 1;
        p = p.offset(1)
    }
    (*s).ocen.x = cen.x as int32;
    (*s).ocen.y = cen.y as int32;
    (*s).ocen.z = cen.z as int32;
    (*s).oradius = rad as int32;
    // END: tight bounding sphere
    return 1 as libc::c_int;
}
unsafe extern "C" fn _imd_load_connectors(mut ppFileData: *mut *mut STRING,
                                          mut FileDataEnd: *mut STRING,
                                          mut s: *mut iIMDShape) -> iBool {
    let mut pFileData: *mut STRING = *ppFileData;
    let mut cnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut p: *mut iVector = 0 as *mut iVector;
    let mut newX: SDWORD = 0;
    let mut newY: SDWORD = 0;
    let mut newZ: SDWORD = 0;
    IMDConnectors =
        (IMDConnectors as
             libc::c_uint).wrapping_add((*s).nconnectors as libc::c_uint) as
            UDWORD as UDWORD;
    (*s).connectors =
        memMallocRelease((::std::mem::size_of::<iVector>() as
                              libc::c_ulong).wrapping_mul((*s).nconnectors as
                                                              libc::c_uint))
            as *mut iVector;
    if (*s).connectors.is_null() {
        iV_Error(0xff as libc::c_int as libc::c_long,
                 b"(_load_connectors) iV_HeapAlloc fail\x00" as *const u8 as
                     *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    }
    p = (*s).connectors;
    i = 0 as libc::c_int;
    while i < (*s).nconnectors {
        if sscanf(pFileData,
                  b"%d %d %d%n\x00" as *const u8 as *const libc::c_char,
                  &mut newX as *mut SDWORD, &mut newY as *mut SDWORD,
                  &mut newZ as *mut SDWORD, &mut cnt as *mut libc::c_int) !=
               3 as libc::c_int {
            iV_Error(0xff as libc::c_int as libc::c_long,
                     b"(_load_connectors) file corrupt -M\x00" as *const u8 as
                         *const libc::c_char as *mut libc::c_char);
            return 0 as libc::c_int
        }
        pFileData = pFileData.offset(cnt as isize);
        (*p).x = newX;
        (*p).y = newY;
        (*p).z = newZ;
        i += 1;
        p = p.offset(1)
    }
    *ppFileData = pFileData;
    return 1 as libc::c_int;
}
// local prototypes
//*************************************************************************
//*** load shape levels recurrsively
//*
//* pre		fp open
//*
//* params	fp 		= currently open shape file pointer
//*			s			= pointer to shape level
//*			texpage	= texture page number if iV_IMD_TEX
//*
//* on exit	s allocated
//* returns	pointer to iFSDShape structure (or NULL on error)
//*
//******
unsafe extern "C" fn _imd_load_level(mut ppFileData: *mut *mut STRING,
                                     mut FileDataEnd: *mut STRING,
                                     mut nlevels: libc::c_int,
                                     mut texpage: libc::c_int)
 -> *mut iIMDShape {
    let mut pFileData: *mut STRING = *ppFileData;
    let mut cnt: libc::c_int = 0;
    let mut s: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut buffer: [libc::c_char; 256] = [0; 256];
    //	int level;
    let mut n: libc::c_int = 0;
    let mut npolys: libc::c_int = 0;
    //		UWORD NumberOfParameters;
//		UDWORD count;
    if nlevels == 0 as libc::c_int { return 0 as *mut iIMDShape }
    s =
        memMallocRelease(::std::mem::size_of::<iIMDShape>() as libc::c_ulong)
            as *mut iIMDShape;
    if !s.is_null() {
        (*s).points = 0 as *mut iVector;
        (*s).polys = 0 as *mut iIMDPoly;
        (*s).connectors = 0 as *mut iVector;
        (*s).texanims = 0 as *mut *mut iTexAnim;
        (*s).next = 0 as *mut iIMDShape;
        // if we can be sure that there is no bsp ... the we check for level number at this point
        (*s).flags = _IMD_FLAGS;
        (*s).texpage = texpage;
        if sscanf(pFileData,
                  b"%s %d%n\x00" as *const u8 as *const libc::c_char,
                  buffer.as_mut_ptr(), &mut n as *mut libc::c_int,
                  &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
            debug(LOG_ERROR,
                  b"_imd_load_level(2): file corrupt\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as *mut iIMDShape
        }
        pFileData = pFileData.offset(cnt as isize);
        // load texture anims if specified
        // load points
        if strcmp(buffer.as_mut_ptr(),
                  b"POINTS\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
            debug(LOG_ERROR,
                  b"_imd_load_level: expecting \'POINTS\' directive\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as *mut iIMDShape
        }
        if n > 512 as libc::c_int {
            debug(LOG_ERROR,
                  b"_imd_load_level: too many points in IMD\x00" as *const u8
                      as *const libc::c_char);
        }
        (*s).npoints = n;
        //	DBPRINTF(("%s %d , %d \n",buffer,n,s->npoints));
        // Some imd/pie's were greater than the max number of points causing all sorts of memory overflows (blfact2)
		//
		// There was no check / error handling!
		//
        _imd_load_points(&mut pFileData, FileDataEnd, s);
        if sscanf(pFileData,
                  b"%s %d%n\x00" as *const u8 as *const libc::c_char,
                  buffer.as_mut_ptr(), &mut npolys as *mut libc::c_int,
                  &mut cnt as *mut libc::c_int) != 2 as libc::c_int {
            debug(LOG_ERROR,
                  b"_imd_load_level(3): file corrupt\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as *mut iIMDShape
        }
        pFileData = pFileData.offset(cnt as isize);
        (*s).npolys = npolys;
        if strcmp(buffer.as_mut_ptr(),
                  b"POLYGONS\x00" as *const u8 as *const libc::c_char) !=
               0 as libc::c_int {
            debug(LOG_ERROR,
                  b"_imd_load_level: expecting \'POLYGONS\' directive\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as *mut iIMDShape
        }
        //			DBPRINTF(("loading polygons - %d\n",s->npolys));
        _imd_load_polys(&mut pFileData, FileDataEnd, s);
        //NOW load optional stuff
        let mut OptionalsCompleted: BOOL =
            0; // Zero the bsp node pointer to zero as a default
        (*s).BSPNode =
            0 as
                PSBSPTREENODE; // Default number of connectors must be 0 ( this was'nt being done PBD. )
        (*s).nconnectors = 0 as libc::c_int;
        OptionalsCompleted = 0 as libc::c_int;
        while OptionalsCompleted == 0 as libc::c_int {
            //				DBPRINTF(("current file pos = %p (%x)(%x)(%x)  - endoffile = %p\n",*ppFileData,**ppFileData,*((*ppFileData)+1),*((*ppFileData)+2),FileDataEnd));
            // check for end of file (give or take white space)
            if AtEndOfFile(pFileData, FileDataEnd) == 1 as libc::c_int {
                OptionalsCompleted = 1 as libc::c_int;
                break ;
            } else if sscanf(pFileData,
                             b"%s %d%n\x00" as *const u8 as
                                 *const libc::c_char, buffer.as_mut_ptr(),
                             &mut n as *mut libc::c_int,
                             &mut cnt as *mut libc::c_int) != 2 as libc::c_int
             {
                OptionalsCompleted = 1 as libc::c_int;
                break ;
            } else {
                pFileData = pFileData.offset(cnt as isize);
                // Scans in the line ... if we don't get 2 parameters then quit
                // check for next level ... or might be a BSP      - This should handle an imd if it has a BSP tree attached to it
				// might be "BSP" or "LEVEL"
                if strcmp(buffer.as_mut_ptr(),
                          b"LEVEL\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    (*s).next =
                        _imd_load_level(&mut pFileData, FileDataEnd,
                                        nlevels - 1 as libc::c_int, texpage)
                } else if strcmp(buffer.as_mut_ptr(),
                                 b"BSP\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    _imd_load_bsp(&mut pFileData, FileDataEnd, s, n as UWORD);
                } else if strcmp(buffer.as_mut_ptr(),
                                 b"CONNECTORS\x00" as *const u8 as
                                     *const libc::c_char) == 0 as libc::c_int
                 {
                    //load connector stuff
                    (*s).nconnectors = n;
                    _imd_load_connectors(&mut pFileData, FileDataEnd, s);
                } else {
                    //				DBPRINTF(("1) current file pos = %p (%x)  - endoffile = %p\n",*ppFileData,**ppFileData,FileDataEnd));
                    iV_Error(0xff as libc::c_int as libc::c_long,
                             b"(_load_level) unexpected directive %s %d\x00"
                                 as *const u8 as *const libc::c_char as
                                 *mut libc::c_char, buffer.as_mut_ptr(),
                             &mut n as *mut libc::c_int);
                    OptionalsCompleted = 1 as libc::c_int;
                    break ;
                }
            }
        }
    } else {
        /* Failed to allocate memory for s */
        debug(LOG_ERROR,
              b"_imd_load_level: Memory allocation error\x00" as *const u8 as
                  *const libc::c_char);
    }
    *ppFileData = pFileData;
    return s;
}
// extended draw routines flags
//*************************************************************************
#[no_mangle]
pub unsafe extern "C" fn iV_setImagePath(mut path: *mut STRING) -> BOOL {
    let mut i: libc::c_int = 0;
    strcpy(imagePath.as_mut_ptr(), path);
    i = strlen(imagePath.as_mut_ptr()) as libc::c_int;
    if imagePath[i as usize] as libc::c_int != '\\' as i32 {
        imagePath[i as usize] = '\\' as i32 as libc::c_char;
        imagePath[(i + 1 as libc::c_int) as usize] =
            0 as libc::c_int as libc::c_char
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _imd_get_path(mut filename: *mut STRING,
                                   mut path: *mut STRING)
 -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    n = strlen(filename) as libc::c_int;
    i = n - 1 as libc::c_int;
    while i >= 0 as libc::c_int &&
              *filename.offset(i as isize) as libc::c_int != '\\' as i32 {
        i -= 1
    }
    if i < 0 as libc::c_int {
        *path.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as STRING
    } else {
        memcpy(path as *mut libc::c_void, filename as *const libc::c_void,
               (i + 1 as libc::c_int) as libc::c_uint);
        *path.offset((i + 1 as libc::c_int) as isize) =
            '\u{0}' as i32 as STRING
    }
    return path;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn CheckColourKey(mut psShape: *mut iIMDShape) -> BOOL {
    let mut psShapeLevel: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut i: libc::c_int = 0;
    if rendSurface.usr >= 0x133 as libc::c_int &&
           rendSurface.usr <= 0x153 as libc::c_int {
        /* check model override flags else check all polys for colorkey flag */
        if _IMD_FLAGS & 0x800 as libc::c_int as libc::c_uint != 0 {
            return 1 as libc::c_int
        } else {
            psShapeLevel = psShape;
            /* loop over levels in model */
            while !psShape.is_null() {
                /* loop over polys in level */
                i = 0 as libc::c_int;
                while i < (*psShape).npolys {
                    /* break if transparent poly found */
                    if (*(*psShape).polys.offset(i as isize)).flags &
                           0x800 as libc::c_int as libc::c_uint != 0 {
                        return 1 as libc::c_int
                    }
                    i += 1
                }
                /* next level */
                psShape = (*psShape).next
            }
        }
    }
    return 0 as libc::c_int;
}
