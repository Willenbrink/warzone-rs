use ::libc;
extern "C" {
    /* A Bison parser, made by GNU Bison 3.5.4.  */
    /*
 * resource.y
 *
 * Yacc file for parsing RES files
 */
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
    #[no_mangle]
    fn res_lex() -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
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
    /* special: sets all to on */
    /* special; on by default */
    /* if too verbose for anything but dedicated debugging... */
    /* _must_ be last! */
    // / Function which does the output
    // / Setup function
    // / Cleaning function
    // / Used to pass data to the above functions. Eg a filename or handle.
    /* *
 * Call once to initialize the debug logging system.
 *
 * Doesn't register any callbacks!
 */
    /* *
 * Shutdown the debug system and remove all output callbacks
 */
    /* *
 * Register a callback to be called on every call to debug()
 *
 * \param	callback	Function which does the output
 * \param	init		Initializer function which does all setup for the callback (optional, may be NULL)
 * \param	exit		Cleanup function called when unregistering the callback (optional, may be NULL)
 * \param	data		Data to be passed to all three functions (optional, may be NULL)
 */
    /* *
 * Toggle debug output for part associated with str
 *
 * \param	str	Codepart in textformat
 */
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
    /*
 * ResLY.h
 *
 * Interface to the RES file lex and yacc functions.
 */
    /* Maximum number of characters in a directory entry */
    /* Maximum number of TEXT items in any one Yacc rule */
    /* The initial resource directory and the current resource directory */
    #[no_mangle]
    static mut aResDir: [STRING; 255];
    #[no_mangle]
    static mut aCurrResDir: [STRING; 255];
    /* Give access to the line number and current text for error messages */
    #[no_mangle]
    fn resGetErrorData(pLine: *mut libc::c_int,
                       ppText: *mut *mut libc::c_char);
    /* Call the load function for a file */
    #[no_mangle]
    fn resLoadFile(pType: *mut STRING, pFile: *mut STRING) -> BOOL;
}
pub type STRING = libc::c_char;
pub type UDWORD = libc::c_uint;
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
/* Stored state numbers (used for stacks). */
pub type yy_state_t = yytype_int8;
/* !YY_RES_Y_TAB_H_INCLUDED  */
/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */
/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */
pub type yytype_int8 = libc::c_schar;
/* Tokens.  */
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub sval: *mut STRING,
}
/* State numbers in computations.  */
pub type yy_state_fast_t = libc::c_int;
/* ! defined yyoverflow || YYERROR_VERBOSE */
/* A type that is properly aligned for any stack member.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
/* Allow frame header files to be singly included */
// directory printfs
/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */
/*
 * A simple error reporting routine
 */
#[no_mangle]
pub unsafe extern "C" fn res_error(mut pMessage: *const libc::c_char,
                                   mut args: ...) {
    let mut line: libc::c_int = 0;
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    resGetErrorData(&mut line, &mut pText);
    /* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */
    debug(LOG_ERROR,
          b"RES file parse error:\n%s at line %d\nText: \'%s\'\n\x00" as
              *const u8 as *const libc::c_char, pMessage, line, pText);
    abort();
}
/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static mut yytranslate: [yytype_int8; 262] =
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
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8];
/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static mut yypact: [yytype_int8; 12] =
    [-(4 as libc::c_int) as yytype_int8, -(1 as libc::c_int) as yytype_int8,
     1 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     -(5 as libc::c_int) as yytype_int8, -(5 as libc::c_int) as yytype_int8,
     -(5 as libc::c_int) as yytype_int8, -(5 as libc::c_int) as yytype_int8,
     3 as libc::c_int as yytype_int8, -(5 as libc::c_int) as yytype_int8,
     -(5 as libc::c_int) as yytype_int8, -(5 as libc::c_int) as yytype_int8];
/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static mut yydefact: [yytype_int8; 12] =
    [0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     5 as libc::c_int as yytype_int8, 6 as libc::c_int as yytype_int8,
     0 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8];
/* YYPGOTO[NTERM-NUM].  */
static mut yypgoto: [yytype_int8; 5] =
    [-(5 as libc::c_int) as yytype_int8, -(5 as libc::c_int) as yytype_int8,
     5 as libc::c_int as yytype_int8, -(5 as libc::c_int) as yytype_int8,
     -(5 as libc::c_int) as yytype_int8];
/* YYDEFGOTO[NTERM-NUM].  */
static mut yydefgoto: [yytype_int8; 5] =
    [-(1 as libc::c_int) as yytype_int8, 3 as libc::c_int as yytype_int8,
     4 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8];
/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static mut yytable: [yytype_int8; 9] =
    [9 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8];
static mut yycheck: [yytype_int8; 9] =
    [0 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8];
/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static mut yystos: [yytype_int8; 12] =
    [0 as libc::c_int as yytype_int8, 5 as libc::c_int as yytype_int8,
     6 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 10 as libc::c_int as yytype_int8,
     11 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8,
     3 as libc::c_int as yytype_int8, 0 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 4 as libc::c_int as yytype_int8];
/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static mut yyr1: [yytype_int8; 8] =
    [0 as libc::c_int as yytype_int8, 7 as libc::c_int as yytype_int8,
     8 as libc::c_int as yytype_int8, 8 as libc::c_int as yytype_int8,
     9 as libc::c_int as yytype_int8, 9 as libc::c_int as yytype_int8,
     10 as libc::c_int as yytype_int8, 11 as libc::c_int as yytype_int8];
/* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static mut yyr2: [yytype_int8; 8] =
    [0 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 2 as libc::c_int as yytype_int8,
     1 as libc::c_int as yytype_int8, 1 as libc::c_int as yytype_int8,
     2 as libc::c_int as yytype_int8, 3 as libc::c_int as yytype_int8];
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
pub static mut res_char: libc::c_int = 0;
/* The semantic value of the lookahead symbol.  */
#[no_mangle]
pub static mut res_lval: YYSTYPE =
    YYSTYPE{sval: 0 as *const STRING as *mut STRING,};
/* Number of syntax errors so far.  */
#[no_mangle]
pub static mut res_nerrs: libc::c_int = 0;
/* Call the yacc parser */
/*----------.
| yyparse.  |
`----------*/
#[no_mangle]
pub unsafe extern "C" fn res_parse() -> libc::c_int {
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
    let mut yyvsa: [YYSTYPE; 200] =
        [YYSTYPE{sval: 0 as *const STRING as *mut STRING,}; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_int = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    /* Lookahead token as an internal (translated) token number.  */
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    /* The variables used to return semantic value and location from the
     action routines.  */
    let mut yyval: YYSTYPE =
        YYSTYPE{sval: 0 as *const STRING as *mut STRING,};
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
    res_nerrs = 0 as libc::c_int;
    res_char = -(2 as libc::c_int);
    's_88:
        loop 
             /*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
             {
            (0 as libc::c_int != 0 &&
                 (0 as libc::c_int <= yystate && yystate < 12 as libc::c_int))
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
                    current_block = 13654579226489013423;
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
                    current_block = 13654579226489013423;
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
                    current_block = 17645337076986514487;
                    break ;
                }
            }
            /* !defined yyoverflow && !defined YYSTACK_RELOCATE */
            if yystate == 9 as libc::c_int {
                /*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
                yyresult = 0 as libc::c_int;
                current_block = 12646643519710607562;
                break ;
            } else {
                /*-----------.
| yybackup.  |
`-----------*/
                /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */
                /* First try to decide what to do without reference to lookahead token.  */
                yyn = yypact[yystate as usize] as libc::c_int;
                if yyn == -(5 as libc::c_int) {
                    current_block = 16521473262250461308;
                } else {
                    /* Not known => get a lookahead token if don't already have one.  */
                    /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
                    if res_char == -(2 as libc::c_int) {
                        res_char = res_lex()
                    }
                    if res_char <= 0 as libc::c_int {
                        yytoken = 0 as libc::c_int;
                        res_char = yytoken
                    } else {
                        yytoken =
                            if 0 as libc::c_int <= res_char &&
                                   res_char <= 261 as libc::c_int {
                                yytranslate[res_char as usize] as libc::c_int
                            } else { 2 as libc::c_int }
                    }
                    /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
                    yyn += yytoken;
                    if yyn < 0 as libc::c_int || (8 as libc::c_int) < yyn ||
                           yycheck[yyn as usize] as libc::c_int != yytoken {
                        current_block = 16521473262250461308;
                    } else {
                        yyn = yytable[yyn as usize] as libc::c_int;
                        if yyn <= 0 as libc::c_int {
                            yyn = -yyn;
                            current_block = 7826724679303511971;
                        } else {
                            /* Count tokens shifted since error; after three, turn off error
     status.  */
                            if yyerrstatus != 0 { yyerrstatus -= 1 }
                            /* Shift the lookahead token.  */
                            yystate = yyn;
                            yyvsp = yyvsp.offset(1);
                            *yyvsp = res_lval;
                            /* Discard the shifted token.  */
                            res_char = -(2 as libc::c_int);
                            current_block = 5162397934938121410;
                        }
                    }
                }
                match current_block {
                    16521473262250461308 =>
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
                                if res_char == -(2 as libc::c_int) {
                                    -(2 as libc::c_int)
                                } else if 0 as libc::c_int <= res_char &&
                                              res_char <= 261 as libc::c_int {
                                    yytranslate[res_char as usize] as
                                        libc::c_int
                                } else { 2 as libc::c_int };
                            /* If not already recovering from an error, report this error.  */
                            if yyerrstatus == 0 {
                                res_nerrs += 1;
                                res_error(b"syntax error\x00" as *const u8 as
                                              *const libc::c_char);
                            }
                            if yyerrstatus == 3 as libc::c_int {
                                /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */
                                if res_char <= 0 as libc::c_int {
                                    /* Return failure if at end of input.  */
                                    if res_char == 0 as libc::c_int {
                                        current_block = 17645337076986514487;
                                        break ;
                                    }
                                } else {
                                    yydestruct(b"Error: discarding\x00" as
                                                   *const u8 as
                                                   *const libc::c_char,
                                               yytoken, &mut res_lval);
                                    res_char = -(2 as libc::c_int)
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
                                if !(yyn == -(5 as libc::c_int)) {
                                    yyn += 1 as libc::c_int;
                                    if 0 as libc::c_int <= yyn &&
                                           yyn <= 8 as libc::c_int &&
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
                                    current_block = 17645337076986514487;
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
                            *yyvsp = res_lval;
                            /* Shift the error token.  */
                            yystate = yyn;
                            current_block = 5162397934938121410;
                        } else { current_block = 7826724679303511971; }
                    }
                    _ => { }
                }
                match current_block {
                    7826724679303511971 =>
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
                            6 => {
                                let mut len: UDWORD = 0;
                                debug(LOG_NEVER,
                                      b"directory: %s\n\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*yyvsp.offset(0 as libc::c_int as
                                                         isize)).sval);
                                if *(*yyvsp.offset(0 as libc::c_int as
                                                       isize)).sval.offset(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize)
                                       as libc::c_int == ':' as i32 ||
                                       *(*yyvsp.offset(0 as libc::c_int as
                                                           isize)).sval.offset(0
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   isize)
                                           as libc::c_int == '\\' as i32 {
                                    strcpy(aCurrResDir.as_mut_ptr(),
                                           (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sval);
                                } else {
                                    strcpy(aCurrResDir.as_mut_ptr(),
                                           aResDir.as_mut_ptr());
                                    strcpy(aCurrResDir.as_mut_ptr().offset(strlen(aResDir.as_mut_ptr())
                                                                               as
                                                                               isize),
                                           (*yyvsp.offset(0 as libc::c_int as
                                                              isize)).sval);
                                }
                                if strlen((*yyvsp.offset(0 as libc::c_int as
                                                             isize)).sval) >
                                       0 as libc::c_int as libc::c_uint {
                                    len = strlen(aCurrResDir.as_mut_ptr());
                                    aCurrResDir[len as usize] =
                                        '\\' as i32 as STRING;
                                    aCurrResDir[len.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                    as usize] =
                                        0 as libc::c_int as STRING
                                }
                            }
                            7 => {
                                if resLoadFile((*yyvsp.offset(-(1 as
                                                                    libc::c_int)
                                                                  as
                                                                  isize)).sval,
                                               (*yyvsp.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).sval)
                                       == 0 {
                                    current_block = 17645337076986514487;
                                    break ;
                                }
                            }
                            _ => { }
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
                                7 as libc::c_int;
                        let yyi: libc::c_int =
                            yypgoto[yylhs as usize] as libc::c_int +
                                *yyssp as libc::c_int;
                        yystate =
                            if 0 as libc::c_int <= yyi &&
                                   yyi <= 8 as libc::c_int &&
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
        13654579226489013423 =>
        /*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
        {
            res_error(b"memory exhausted\x00" as *const u8 as
                          *const libc::c_char);
            yyresult = 2 as libc::c_int
        }
        17645337076986514487 =>
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
    if res_char != -(2 as libc::c_int) {
        /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
        yytoken =
            if 0 as libc::c_int <= res_char && res_char <= 261 as libc::c_int
               {
                yytranslate[res_char as usize] as libc::c_int
            } else { 2 as libc::c_int };
        yydestruct(b"Cleanup: discarding lookahead\x00" as *const u8 as
                       *const libc::c_char, yytoken, &mut res_lval);
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
