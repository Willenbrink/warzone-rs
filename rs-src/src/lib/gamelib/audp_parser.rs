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
    /* Use misc.  */
    /* Use misc or X/Open.  */
    /* Allocate SIZE bytes of memory.  */
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: __builtin_va_list) -> libc::c_int;
    #[no_mangle]
    fn parserSetInputBuffer(pBuffer: *mut libc::c_char, size: UDWORD);
    #[no_mangle]
    fn parseGetErrorData(pLine: *mut libc::c_int,
                         ppText: *mut *mut libc::c_char);
    #[no_mangle]
    fn audio_SetTrackVals(szFileName: *mut libc::c_char, bLoop: BOOL,
                          piID: *mut libc::c_int, iVol: libc::c_int,
                          iPriority: libc::c_int, iAudibleRadius: libc::c_int,
                          VagID: libc::c_int) -> BOOL;
    /* **************************************************************************/
/*
 * Anim.h
 *
 * Animation types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    /* ensure ANIM2D/3D structs same size */
    /* width of container bitmap */
    /* ensure ANIM2D/3D structs same size */
    /* **************************************************************************/
    /* frame to play           */
    /* **************************************************************************/
    /* **************************************************************************/
    #[no_mangle]
    fn anim_Create3D(szPieFileName: *mut libc::c_char, uwFrames: UWORD,
                     uwFrameRate: UWORD, uwObj: UWORD, ubType: UBYTE,
                     uwID: UWORD) -> BOOL;
    #[no_mangle]
    fn anim_BeginScript();
    #[no_mangle]
    fn anim_EndScript() -> BOOL;
    #[no_mangle]
    fn anim_AddFrameToAnim(iFrame: libc::c_int, vecPos_0: VECTOR3D,
                           vecRot_0: VECTOR3D, vecScale_0: VECTOR3D) -> BOOL;
    #[no_mangle]
    fn anim_SetVals(szFileName: *mut libc::c_char, uwAnimID: UWORD);
    #[no_mangle]
    fn audp_lex() -> libc::c_int;
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
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
/* Stored state numbers (used for stacks). */
pub type yy_state_t = yytype_int8;
/* **************************************************************************/
pub type yytype_int8 = libc::c_schar;
/* Tokens.  */
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub fval: libc::c_float,
    pub ival: libc::c_long,
    pub bval: libc::c_schar,
    pub sval: [libc::c_char; 100],
}
/* State numbers in computations.  */
pub type yy_state_fast_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
pub const ANIM_3D_FRAMES: C2RustUnnamed = 1;
pub const ANIM_3D_TRANS: C2RustUnnamed = 2;
/* ! defined yyoverflow || YYERROR_VERBOSE */
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
pub type C2RustUnnamed = libc::c_uint;
pub const ANIM_2D: C2RustUnnamed = 0;
/* A Bison parser, made by GNU Bison 3.5.4.  */
/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
static mut g_iCurAnimID: libc::c_int = 0 as libc::c_int;
static mut g_iDummy: libc::c_int = 0;
static mut vecPos: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
static mut vecRot: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
static mut vecScale: VECTOR3D = VECTOR3D{x: 0, y: 0, z: 0,};
/* **************************************************************************/
/* A simple error reporting routine */
#[no_mangle]
pub unsafe extern "C" fn audp_error(mut pMessage: *mut libc::c_char,
                                    mut args: ...) {
    let mut line: libc::c_int = 0;
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut aTxtBuf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: va_list = 0 as *mut libc::c_char;
    args_0 = args.clone();
    vsprintf(aTxtBuf.as_mut_ptr(), pMessage, args_0);
    parseGetErrorData(&mut line, &mut pText);
    /* !YY_AUDP_Y_TAB_H_INCLUDED  */
    /* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */
    debug(LOG_ERROR,
          b"RES file parse error:\n%s at line %d\nToken: %d, Text: \'%s\'\n\x00"
              as *const u8 as *const libc::c_char, aTxtBuf.as_mut_ptr(), line,
          audp_char, pText);
    abort();
}
/* **************************************************************************/
/* Read a resource file */
#[no_mangle]
pub unsafe extern "C" fn ParseResourceFile(mut pData: *mut libc::c_char,
                                           mut fileSize: UDWORD) -> BOOL {
    // Tell lex about the input buffer
    parserSetInputBuffer(pData, fileSize);
    /* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */
    audp_parse();
    return 1 as libc::c_int;
}
/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static mut yytranslate: [yytype_int8; 271] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 12 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8];
/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static mut yypact: [yytype_int8; 81] =
    [20 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     -(4 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(5 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     18 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 9 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 30 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(2 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 32 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     33 as libc::c_int as yytype_int8, 34 as libc::c_int as yytype_int8,
     19 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 35 as libc::c_int as yytype_int8,
     36 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     39 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 29 as libc::c_int as yytype_int8,
     42 as libc::c_int as yytype_int8, 43 as libc::c_int as yytype_int8,
     44 as libc::c_int as yytype_int8, 38 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     45 as libc::c_int as yytype_int8, -(3 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 46 as libc::c_int as yytype_int8,
     41 as libc::c_int as yytype_int8, 38 as libc::c_int as yytype_int8,
     47 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 49 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     48 as libc::c_int as yytype_int8, 50 as libc::c_int as yytype_int8,
     51 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     52 as libc::c_int as yytype_int8, 44 as libc::c_int as yytype_int8,
     53 as libc::c_int as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     55 as libc::c_int as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     56 as libc::c_int as yytype_int8, 57 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8];
/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static mut yydefact: [yytype_int8; 81] =
    [18 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 22 as libc::c_int as yytype_int8,
     30 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     31 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 14 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     38 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 34 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 32 as libc::c_int as yytype_int8,
     37 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 33 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 35 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 36 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     39 as libc::c_int as yytype_int8];
/* YYPGOTO[NTERM-NUM].  */
static mut yypgoto: [yytype_int8; 24] =
    [-(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 58 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 54 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(8 as libc::c_int) as yytype_int8, 59 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, 2 as libc::c_int as yytype_int8,
     -(58 as libc::c_int) as yytype_int8, -(58 as libc::c_int) as yytype_int8,
     -(11 as libc::c_int) as yytype_int8,
     -(57 as libc::c_int) as yytype_int8];
/* YYDEFGOTO[NTERM-NUM].  */
static mut yydefgoto: [yytype_int8; 24] =
    [-(1 as libc::c_int) as yytype_int8, 5 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     12 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8,
     13 as libc::c_int as yytype_int8, 49 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 44 as libc::c_int as yytype_int8,
     52 as libc::c_int as yytype_int8, 60 as libc::c_int as yytype_int8,
     61 as libc::c_int as yytype_int8, 73 as libc::c_int as yytype_int8,
     57 as libc::c_int as yytype_int8, 58 as libc::c_int as yytype_int8];
/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static mut yytable: [yytype_int8; 86] =
    [64 as libc::c_int as yytype_int8, 56 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 56 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     15 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     63 as libc::c_int as yytype_int8, 33 as libc::c_int as yytype_int8,
     77 as libc::c_int as yytype_int8, 36 as libc::c_int as yytype_int8,
     64 as libc::c_int as yytype_int8, 39 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     18 as libc::c_int as yytype_int8, 42 as libc::c_int as yytype_int8,
     43 as libc::c_int as yytype_int8, 38 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     31 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 32 as libc::c_int as yytype_int8,
     35 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     41 as libc::c_int as yytype_int8, 45 as libc::c_int as yytype_int8,
     46 as libc::c_int as yytype_int8, 47 as libc::c_int as yytype_int8,
     48 as libc::c_int as yytype_int8, 50 as libc::c_int as yytype_int8,
     51 as libc::c_int as yytype_int8, 53 as libc::c_int as yytype_int8,
     54 as libc::c_int as yytype_int8, 55 as libc::c_int as yytype_int8,
     56 as libc::c_int as yytype_int8, 62 as libc::c_int as yytype_int8,
     65 as libc::c_int as yytype_int8, 68 as libc::c_int as yytype_int8,
     70 as libc::c_int as yytype_int8, 59 as libc::c_int as yytype_int8,
     69 as libc::c_int as yytype_int8, 72 as libc::c_int as yytype_int8,
     74 as libc::c_int as yytype_int8, 76 as libc::c_int as yytype_int8,
     66 as libc::c_int as yytype_int8, 78 as libc::c_int as yytype_int8,
     79 as libc::c_int as yytype_int8, 80 as libc::c_int as yytype_int8,
     75 as libc::c_int as yytype_int8, 67 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     71 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 34 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8];
static mut yycheck: [yytype_int8; 86] =
    [57 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     75 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     73 as libc::c_int as yytype_int8, 61 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     16 as libc::c_int as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, 23 as libc::c_int as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     -(1 as libc::c_int) as yytype_int8, 26 as libc::c_int as yytype_int8];
/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static mut yystos: [yytype_int8; 81] =
    [0 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 13 as libc::c_int as yytype_int8,
     14 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     26 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     30 as libc::c_int as yytype_int8, 32 as libc::c_int as yytype_int8,
     34 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8,
     30 as libc::c_int as yytype_int8, 31 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 31 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     35 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     16 as libc::c_int as yytype_int8, 33 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     36 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     41 as libc::c_int as yytype_int8, 15 as libc::c_int as yytype_int8,
     37 as libc::c_int as yytype_int8, 38 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     41 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     17 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 16 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 39 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 17 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8];
/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static mut yyr1: [yytype_int8; 40] =
    [0 as libc::c_int as yytype_int8, 18 as libc::c_int as yytype_int8,
     19 as libc::c_int as yytype_int8, 19 as libc::c_int as yytype_int8,
     20 as libc::c_int as yytype_int8, 20 as libc::c_int as yytype_int8,
     21 as libc::c_int as yytype_int8, 21 as libc::c_int as yytype_int8,
     22 as libc::c_int as yytype_int8, 23 as libc::c_int as yytype_int8,
     23 as libc::c_int as yytype_int8, 24 as libc::c_int as yytype_int8,
     24 as libc::c_int as yytype_int8, 25 as libc::c_int as yytype_int8,
     25 as libc::c_int as yytype_int8, 26 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 27 as libc::c_int as yytype_int8,
     27 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     28 as libc::c_int as yytype_int8, 28 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 29 as libc::c_int as yytype_int8,
     29 as libc::c_int as yytype_int8, 30 as libc::c_int as yytype_int8,
     30 as libc::c_int as yytype_int8, 31 as libc::c_int as yytype_int8,
     33 as libc::c_int as yytype_int8, 32 as libc::c_int as yytype_int8,
     35 as libc::c_int as yytype_int8, 36 as libc::c_int as yytype_int8,
     34 as libc::c_int as yytype_int8, 37 as libc::c_int as yytype_int8,
     37 as libc::c_int as yytype_int8, 39 as libc::c_int as yytype_int8,
     38 as libc::c_int as yytype_int8, 40 as libc::c_int as yytype_int8,
     40 as libc::c_int as yytype_int8, 41 as libc::c_int as yytype_int8];
/* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static mut yyr2: [yytype_int8; 40] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     7 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8];
/* YYERROR_VERBOSE */
/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/
unsafe extern "C" fn yydestruct(mut yymsg: *const libc::c_char,
                                mut yytype: libc::c_int,
                                mut yyvaluep: *mut YYSTYPE) {
    if yymsg.is_null() {
        yymsg = b"Deleting\x00" as *const u8 as *const libc::c_char
    };
}
/* The lookahead symbol.  */
#[no_mangle]
pub static mut audp_char: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
#[no_mangle]
pub static mut audp_lval: YYSTYPE = YYSTYPE{fval: 0.,};
/* Number of syntax errors so far.  */
#[no_mangle]
pub static mut audp_nerrs: libc::c_int = 0;
/*----------.
| yyparse.  |
`----------*/
#[no_mangle]
pub unsafe extern "C" fn audp_parse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: yy_state_fast_t = 0;
    /* Number of tokens to shift before error messages enabled.  */
    let mut yyerrstatus: libc::c_int = 0;
    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */
    /* The state stack.  */
    let mut yyssa: [yy_state_t; 200] = [0; 200];
    let mut yyss: *mut yy_state_t = 0 as *mut yy_state_t;
    let mut yyssp: *mut yy_state_t = 0 as *mut yy_state_t;
    /* The semantic value stack.  */
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE{fval: 0.,}; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
     action routines.  */
    let mut yyval: YYSTYPE = YYSTYPE{fval: 0.,};
    /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
    let mut yylen: libc::c_int =
        0 as libc::c_int; /* Cause a token to be read.  */
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    audp_nerrs = 0 as libc::c_int;
    audp_char = -(2 as libc::c_int);
    's_88:
        loop 
             /*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
             {
            (0 as libc::c_int != 0 &&
                 (0 as libc::c_int <= yystate && yystate < 81 as libc::c_int))
                as libc::c_int;
            *yyssp = yystate as yy_state_t;
            if yyss.offset(yystacksize as
                               isize).offset(-(1 as libc::c_int as isize)) <=
                   yyssp {
                /* Get the current used size of the three stacks, in elements.  */
                let mut yysize: libc::c_int =
                    yyssp.wrapping_offset_from(yyss) as libc::c_int +
                        1 as libc::c_int;
                /* defined YYSTACK_RELOCATE */
                /* Extend the stack our own way.  */
                if 10000 as libc::c_int <= yystacksize {
                    current_block = 4171266573807559451;
                    break ;
                }
                yystacksize *= 2 as libc::c_int;
                if (10000 as libc::c_int) < yystacksize {
                    yystacksize = 10000 as libc::c_int
                }
                let mut yyss1: *mut yy_state_t = yyss;
                let mut yyptr: *mut yyalloc =
                    malloc((yystacksize *
                                (::std::mem::size_of::<yy_state_t>() as
                                     libc::c_ulong as libc::c_int +
                                     ::std::mem::size_of::<YYSTYPE>() as
                                         libc::c_ulong as libc::c_int) +
                                (::std::mem::size_of::<yyalloc>() as
                                     libc::c_ulong as libc::c_int -
                                     1 as libc::c_int)) as libc::c_uint) as
                        *mut yyalloc;
                if yyptr.is_null() {
                    current_block = 4171266573807559451;
                    break ;
                }
                let mut yynewbytes: libc::c_int = 0;
                libc::memcpy(&mut (*yyptr).yyss_alloc as *mut yy_state_t as
                                 *mut libc::c_void,
                             yyss as *const libc::c_void,
                             (yysize as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<yy_state_t>()
                                                                 as
                                                                 libc::c_ulong)
                                 as libc::size_t);
                yyss = &mut (*yyptr).yyss_alloc;
                yynewbytes =
                    yystacksize *
                        ::std::mem::size_of::<yy_state_t>() as libc::c_ulong
                            as libc::c_int +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_int - 1 as libc::c_int);
                yyptr =
                    yyptr.offset((yynewbytes /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_int) as
                                     isize);
                let mut yynewbytes_0: libc::c_int = 0;
                libc::memcpy(&mut (*yyptr).yyvs_alloc as *mut YYSTYPE as
                                 *mut libc::c_void,
                             yyvs as *const libc::c_void,
                             (yysize as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<YYSTYPE>()
                                                                 as
                                                                 libc::c_ulong)
                                 as libc::size_t);
                yyvs = &mut (*yyptr).yyvs_alloc;
                yynewbytes_0 =
                    yystacksize *
                        ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong as
                            libc::c_int +
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong as
                             libc::c_int - 1 as libc::c_int);
                yyptr =
                    yyptr.offset((yynewbytes_0 /
                                      ::std::mem::size_of::<yyalloc>() as
                                          libc::c_ulong as libc::c_int) as
                                     isize);
                if yyss1 != yyssa.as_mut_ptr() {
                    free(yyss1 as *mut libc::c_void);
                }
                yyssp =
                    yyss.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                yyvsp =
                    yyvs.offset(yysize as
                                    isize).offset(-(1 as libc::c_int as
                                                        isize));
                if yyss.offset(yystacksize as
                                   isize).offset(-(1 as libc::c_int as isize))
                       <= yyssp {
                    current_block = 1286942664186997988;
                    break ;
                }
            }
            /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
            if yystate == 19 as libc::c_int {
                /*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
                yyresult = 0 as libc::c_int;
                current_block = 12485823068340601847;
                break ;
            } else {
                /*-----------.
| yybackup.  |
`-----------*/
                /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */
                /* First try to decide what to do without reference to lookahead token.  */
                yyn = yypact[yystate as usize] as libc::c_int;
                if yyn == -(58 as libc::c_int) {
                    current_block = 3021757436737313741;
                } else {
                    /* Not known => get a lookahead token if don't already have one.  */
                    /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                    if audp_char == -(2 as libc::c_int) {
                        audp_char = audp_lex()
                    }
                    if audp_char <= 0 as libc::c_int {
                        yytoken = 0 as libc::c_int;
                        audp_char = yytoken
                    } else {
                        yytoken =
                            if 0 as libc::c_int <= audp_char &&
                                   audp_char <= 270 as libc::c_int {
                                yytranslate[audp_char as usize] as libc::c_int
                            } else { 2 as libc::c_int }
                    }
                    /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
                    yyn += yytoken;
                    if yyn < 0 as libc::c_int || (85 as libc::c_int) < yyn ||
                           yycheck[yyn as usize] as libc::c_int != yytoken {
                        current_block = 3021757436737313741;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 3559315787181598580;
                        } else {
                            /* Count tokens shifted since error; after three, turn off error
     status.  */
                            if yyerrstatus != 0 { yyerrstatus -= 1 }
                            /* Shift the lookahead token.  */
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = audp_lval;
                            /* Discard the shifted token.  */
                            audp_char = -(2 as libc::c_int);
                            current_block = 6980994685763142299;
                        }
                    }
                }
                match current_block {
                    3021757436737313741 =>
                    /*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
                    {
                        yyn = yydefact[yystate as usize] as libc::c_int;
                        if yyn == 0 as libc::c_int {
                            /*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
                            /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
                            yytoken =
                                if audp_char == -(2 as libc::c_int) {
                                    -(2 as libc::c_int)
                                } else if 0 as libc::c_int <= audp_char &&
                                              audp_char <= 270 as libc::c_int
                                 {
                                    yytranslate[audp_char as usize] as
                                        libc::c_int
                                } else { 2 as libc::c_int };
                            /* If not already recovering from an error, report this error.  */
                            if yyerrstatus == 0 {
                                audp_nerrs += 1;
                                audp_error(b"syntax error\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char);
                            }
                            if yyerrstatus == 3 as libc::c_int {
                                /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */
                                if audp_char <= 0 as libc::c_int {
                                    /* Return failure if at end of input.  */
                                    if audp_char == 0 as libc::c_int {
                                        current_block = 1286942664186997988;
                                        break ;
                                    }
                                } else {
                                    yydestruct(b"Error: discarding\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               yytoken, &mut audp_lval);
                                    audp_char = -(2 as libc::c_int)
                                }
                            }
                            /* Else will try to reuse lookahead token after shifting the error
     token.  */
                            /*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
                            yyerrstatus =
                                3 as
                                    libc::c_int; /* Each real token shifted decrements this.  */
                            loop  {
                                yyn = yypact[yystate as usize] as libc::c_int;
                                if !(yyn == -(58 as libc::c_int)) {
                                    yyn += 1 as libc::c_int;
                                    if 0 as libc::c_int <= yyn &&
                                           yyn <= 85 as libc::c_int &&
                                           yycheck[yyn as usize] as
                                               libc::c_int == 1 as libc::c_int
                                       {
                                        yyn =
                                            yytable[yyn as usize] as
                                                libc::c_int;
                                        if (0 as libc::c_int) < yyn {
                                            break ;
                                        }
                                    }
                                }
                                /* Pop the current state because it cannot handle the error token.  */
                                if yyssp == yyss {
                                    current_block = 1286942664186997988;
                                    break 's_88 ;
                                }
                                yydestruct(b"Error: popping\x00" as *const u8
                                               as *const libc::c_char,
                                           yystos[yystate as usize] as
                                               libc::c_int, yyvsp);
                                yyvsp =
                                    yyvsp.offset(-(1 as libc::c_int as
                                                       isize));
                                yyssp =
                                    yyssp.offset(-(1 as libc::c_int as
                                                       isize));
                                yystate = *yyssp as yy_state_fast_t
                            }
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = audp_lval;
                            /* Shift the error token.  */
                            yystate = yyn;
                            current_block = 6980994685763142299;
                        } else { current_block = 3559315787181598580; }
                    }
                    _ => { }
                }
                match current_block {
                    3559315787181598580 =>
                    /*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
                    /* yyn is the number of a rule to reduce with.  */
                    {
                        yylen = yyr2[yyn as usize] as libc::c_int;
                        /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
                        yyval =
                            *yyvsp.offset((1 as libc::c_int - yylen) as
                                              isize);
                        match yyn {
                            13 => {
                                audio_SetTrackVals((*yyvsp.offset(-(4 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).sval.as_mut_ptr(),
                                                   1 as libc::c_int,
                                                   &mut g_iDummy,
                                                   (*yyvsp.offset(-(2 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   (*yyvsp.offset(-(1 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   (*yyvsp.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   0 as libc::c_int);
                            }
                            14 => {
                                audio_SetTrackVals((*yyvsp.offset(-(4 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).sval.as_mut_ptr(),
                                                   0 as libc::c_int,
                                                   &mut g_iDummy,
                                                   (*yyvsp.offset(-(2 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   (*yyvsp.offset(-(1 as
                                                                        libc::c_int)
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   (*yyvsp.offset(0 as
                                                                      libc::c_int
                                                                      as
                                                                      isize)).ival
                                                       as libc::c_int,
                                                   0 as libc::c_int);
                            }
                            27 => {
                                g_iCurAnimID =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).ival as
                                        libc::c_int;
                                anim_SetVals((*yyvsp.offset(-(1 as
                                                                  libc::c_int)
                                                                as
                                                                isize)).sval.as_mut_ptr(),
                                             (*yyvsp.offset(0 as libc::c_int
                                                                as
                                                                isize)).ival
                                                 as UWORD);
                            }
                            28 => {
                                anim_Create3D((*yyvsp.offset(-(3 as
                                                                   libc::c_int)
                                                                 as
                                                                 isize)).sval.as_mut_ptr(),
                                              (*yyvsp.offset(-(2 as
                                                                   libc::c_int)
                                                                 as
                                                                 isize)).ival
                                                  as UWORD,
                                              (*yyvsp.offset(-(1 as
                                                                   libc::c_int)
                                                                 as
                                                                 isize)).ival
                                                  as UWORD,
                                              (*yyvsp.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ival
                                                  as UWORD,
                                              ANIM_3D_TRANS as libc::c_int as
                                                  UBYTE,
                                              g_iCurAnimID as UWORD);
                            }
                            29 => { g_iCurAnimID += 1 }
                            30 => {
                                anim_Create3D((*yyvsp.offset(-(2 as
                                                                   libc::c_int)
                                                                 as
                                                                 isize)).sval.as_mut_ptr(),
                                              (*yyvsp.offset(-(1 as
                                                                   libc::c_int)
                                                                 as
                                                                 isize)).ival
                                                  as UWORD,
                                              (*yyvsp.offset(0 as libc::c_int
                                                                 as
                                                                 isize)).ival
                                                  as UWORD,
                                              1 as libc::c_int as UWORD,
                                              ANIM_3D_FRAMES as libc::c_int as
                                                  UBYTE,
                                              g_iCurAnimID as UWORD);
                            }
                            31 => { anim_BeginScript(); }
                            32 => { anim_EndScript(); g_iCurAnimID += 1 }
                            35 => { anim_BeginScript(); }
                            36 => { anim_EndScript(); }
                            39 => {
                                vecPos.x =
                                    (*yyvsp.offset(-(8 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecPos.y =
                                    (*yyvsp.offset(-(7 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecPos.z =
                                    (*yyvsp.offset(-(6 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecRot.x =
                                    (*yyvsp.offset(-(5 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecRot.y =
                                    (*yyvsp.offset(-(4 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecRot.z =
                                    (*yyvsp.offset(-(3 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecScale.x =
                                    (*yyvsp.offset(-(2 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecScale.y =
                                    (*yyvsp.offset(-(1 as libc::c_int) as
                                                       isize)).ival as SDWORD;
                                vecScale.z =
                                    (*yyvsp.offset(0 as libc::c_int as
                                                       isize)).ival as SDWORD;
                                anim_AddFrameToAnim((*yyvsp.offset(-(9 as
                                                                         libc::c_int)
                                                                       as
                                                                       isize)).ival
                                                        as libc::c_int,
                                                    vecPos, vecRot, vecScale);
                            }
                            15 | _ => { }
                        }
                        /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
                        yyvsp = yyvsp.offset(-(yylen as isize));
                        yyssp = yyssp.offset(-(yylen as isize));
                        yylen = 0 as libc::c_int;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yyval;
                        /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
                        let yylhs: libc::c_int =
                            yyr1[yyn as usize] as libc::c_int -
                                18 as libc::c_int;
                        let yyi: libc::c_int =
                            yypgoto[yylhs as usize] as libc::c_int +
                                *yyssp as libc::c_int;
                        yystate =
                            if 0 as libc::c_int <= yyi &&
                                   yyi <= 85 as libc::c_int &&
                                   yycheck[yyi as usize] as libc::c_int ==
                                       *yyssp as libc::c_int {
                                yytable[yyi as usize] as libc::c_int
                            } else {
                                yydefgoto[yylhs as usize] as libc::c_int
                            }
                    }
                    _ => { }
                }
                /*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
                /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
                yyssp = yyssp.offset(1)
            }
        }
    match current_block {
        4171266573807559451 =>
        /*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
        {
            audp_error(b"memory exhausted\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
            yyresult = 2 as libc::c_int
        }
        1286942664186997988 =>
        /*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
        {
            yyresult = 1 as libc::c_int
        }
        _ => { }
    }
    /* Fall through.  */
    /*-----------------------------------------------------.
| yyreturn -- parsing is finished, return the result.  |
`-----------------------------------------------------*/
    if audp_char != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
        yytoken =
            if 0 as libc::c_int <= audp_char &&
                   audp_char <= 270 as libc::c_int {
                yytranslate[audp_char as usize] as libc::c_int
            } else { 2 as libc::c_int };
        yydestruct(b"Cleanup: discarding lookahead\x00" as *const u8 as
                       *const libc::c_char, yytoken, &mut audp_lval);
    }
    /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(b"Cleanup: popping\x00" as *const u8 as
                       *const libc::c_char,
                   yystos[*yyssp as libc::c_int as usize] as libc::c_int,
                   yyvsp);
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize))
    }
    if yyss != yyssa.as_mut_ptr() { free(yyss as *mut libc::c_void); }
    return yyresult;
}
