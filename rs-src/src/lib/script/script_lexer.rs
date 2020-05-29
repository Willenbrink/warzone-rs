use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    /* The table of user types */
    #[no_mangle]
    static mut asScrTypeTab: *mut TYPE_SYMBOL;
    /* Look up a type symbol */
    #[no_mangle]
    fn scriptLookUpType(pIdent: *mut STRING, pType: *mut INTERP_TYPE) -> BOOL;
    /* Look up a variable symbol */
    #[no_mangle]
    fn scriptLookUpVariable(pIdent: *mut STRING, ppsSym: *mut *mut VAR_SYMBOL)
     -> BOOL;
    /* Look up a constant variable symbol */
    #[no_mangle]
    fn scriptLookUpConstant(pIdent: *mut STRING,
                            ppsSym: *mut *mut CONST_SYMBOL) -> BOOL;
    /* Lookup a trigger symbol */
    #[no_mangle]
    fn scriptLookUpTrigger(pIdent: *mut STRING,
                           ppsTrigger: *mut *mut TRIGGER_SYMBOL) -> BOOL;
    /* Lookup a callback trigger symbol */
    #[no_mangle]
    fn scriptLookUpCallback(pIdent: *mut STRING,
                            ppsCallback: *mut *mut CALLBACK_SYMBOL) -> BOOL;
    /* Lookup an event symbol */
    #[no_mangle]
    fn scriptLookUpEvent(pIdent: *mut STRING,
                         ppsEvent: *mut *mut EVENT_SYMBOL) -> BOOL;
    // The code block
    /* Look up a function symbol */
    #[no_mangle]
    fn scriptLookUpFunction(pIdent: *mut STRING,
                            ppsSym: *mut *mut FUNC_SYMBOL) -> BOOL;
    /* Look up an in-script custom function symbol */
    #[no_mangle]
    fn scriptLookUpCustomFunction(pIdent: *mut STRING,
                                  ppsSym: *mut *mut EVENT_SYMBOL) -> BOOL;
    #[no_mangle]
    static mut scr_lval: YYSTYPE;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(_: *const libc::c_void, _: libc::c_uint, _: libc::c_uint,
              _: *mut FILE) -> libc::c_uint;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
 *	ISO C99 Standard: 7.5 Errors	<errno.h>
 */
    /* The system-specific definitions of the E* constants, as macros.  */
    /* When included from assembly language, this header only provides the
   E* constants.  */
    /* The error code set by various library functions.  */
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    /* Convert a string to a long integer.  */
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* Call all functions registered with `atexit' and `on_exit',
   in the reverse of the order in which they were registered,
   perform stdio cleanup, and terminate program execution with STATUS.  */
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* Return 1 if FD is a valid descriptor associated
   with a terminal, zero if not.  */
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_uint;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_longlong;
/* Type of file link counts.  */
pub type __off_t = libc::c_long;
/* Type of file sizes and offsets.  */
pub type __off64_t = __int64_t;
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
/* Caution: The contents of this file are not part of the official
   stdio.h API.  However, much of it is part of the official *binary*
   interface, and therefore cannot be changed.  */
/* During the build of glibc itself, _IO_lock_t will already have been
   defined by internal headers.  */
/* The tag name of this struct is _IO_FILE to preserve historic
   C++ mangled names for functions taking FILE* arguments.
   That name should not be used in new code.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 40],
}
pub type _IO_lock_t = ();
/* The opaque type of streams.  This is the definition used elsewhere.  */
pub type FILE = _IO_FILE;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type flex_uint8_t = uint8_t;
pub type flex_int16_t = int16_t;
pub type flex_int32_t = int32_t;
/* Return all but the first "n" matched characters back to the input stream. */
/* Undo effects of setting up yytext. */
/* set up yytext again */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yy_buffer_state {
    pub yy_input_file: *mut FILE,
    pub yy_ch_buf: *mut libc::c_char,
    pub yy_buf_pos: *mut libc::c_char,
    pub yy_buf_size: libc::c_int,
    pub yy_n_chars: libc::c_int,
    pub yy_is_our_buffer: libc::c_int,
    pub yy_is_interactive: libc::c_int,
    pub yy_at_bol: libc::c_int,
    pub yy_bs_lineno: libc::c_int,
    pub yy_bs_column: libc::c_int,
    pub yy_fill_buffer: libc::c_int,
    pub yy_buffer_status: libc::c_int,
}
/* __ia64__ */
/* Skip white space */
/* rule 48 can match eol */
/* The state buf must be large enough to hold one state per character in the main buffer.
 */
/* Strip comments */
/* rule 51 can match eol */
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
/* rule 53 can match eol */
/* Strip single line comments */
pub type yy_size_t = size_t;
/* Begin user sect3 */
pub type YY_CHAR = flex_uint8_t;
pub type yy_state_type = libc::c_int;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type SWORD = libc::c_short;
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
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
pub type INTERP_TYPE = _interp_type;
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
pub type SCRIPT_FUNC = Option<unsafe extern "C" fn() -> BOOL>;
/* The type of function called to access an object or in-game variable */
pub type SCRIPT_VARFUNC = Option<unsafe extern "C" fn(_: UDWORD) -> BOOL>;
pub type _storage_type = libc::c_uint;
pub const ST_LOCAL: _storage_type = 4;
pub const ST_EXTERN: _storage_type = 3;
pub const ST_OBJECT: _storage_type = 2;
pub const ST_PRIVATE: _storage_type = 1;
pub const ST_PUBLIC: _storage_type = 0;
pub type STORAGE_TYPE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_debug {
    pub offset: UDWORD,
    pub line: UDWORD,
    pub pLabel: *mut STRING,
}
/* Line debugging information for a script */
pub type SCRIPT_DEBUG = _script_debug;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _code_block {
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
    pub type_0: INTERP_TYPE,
}
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
/* Definition for the chunks of code that are used within the compiler */
pub type CODE_BLOCK = _code_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _param_block {
    pub numParams: UDWORD,
    pub aParams: *mut INTERP_TYPE,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
}
// size of the code block
// pointer to the code data
// Debugging info for the script.
// The type of the code block
/* The chunk of code returned from parsing a parameter list. */
pub type PARAM_BLOCK = _param_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _param_decl {
    pub numParams: UDWORD,
    pub aParams: *mut INTERP_TYPE,
}
// List of parameter types
// The code that puts the parameters onto the stack
/* The types of a functions parameters, returned from parsing a parameter declaration */
pub type PARAM_DECL = _param_decl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cond_block {
    pub numOffsets: UDWORD,
    pub aOffsets: *mut UDWORD,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
}
/* The chunk of code used while parsing a conditional statement */
pub type COND_BLOCK = _cond_block;
pub type _access_type = libc::c_uint;
pub const AT_OBJECT: _access_type = 1;
pub const AT_SIMPLE: _access_type = 0;
// Positions in the code that have to be
// replaced with the offset to the end of the
							// conditional statment (for the jumps).
// Number of debugging entries in psDebug.
// Debugging info for the script.
// function pointer for script variable saving
// if pBuffer is NULL the script system is just asking how much space the saved variable will require
// otherwise pBuffer points to an array to store the value in
pub type SCR_VAL_SAVE
    =
    Option<unsafe extern "C" fn(_: INTERP_TYPE, _: UDWORD,
                                _: *mut libc::c_char, _: *mut UDWORD)
               -> BOOL>;
// function pointer for script variable loading
pub type SCR_VAL_LOAD
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: INTERP_TYPE,
                                _: *mut libc::c_char, _: UDWORD,
                                _: *mut UDWORD) -> BOOL>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _type_symbol {
    pub typeID: SWORD,
    pub accessType: SWORD,
    pub pIdent: *mut STRING,
    pub saveFunc: SCR_VAL_SAVE,
    pub loadFunc: SCR_VAL_LOAD,
}
/* Type for a user type symbol */
pub type TYPE_SYMBOL = _type_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_ident_decl {
    pub pIdent: *mut STRING,
    pub dimensions: SDWORD,
    pub elements: [SDWORD; 4],
}
// The type id to use in the type field of values
// Whether the type is an object or a simple value
// Type identifier
// load and save functions
// 
/* Type for a variable identifier declaration */
pub type VAR_IDENT_DECL = _var_ident_decl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_symbol {
    pub pIdent: *mut STRING,
    pub type_0: INTERP_TYPE,
    pub storage: STORAGE_TYPE,
    pub objType: INTERP_TYPE,
    pub index: UDWORD,
    pub get: SCRIPT_VARFUNC,
    pub set: SCRIPT_VARFUNC,
    pub dimensions: SDWORD,
    pub elements: [SDWORD; 4],
    pub psNext: *mut _var_symbol,
}
// variable identifier
// number of dimensions of an array - 0 for normal var
// number of elements in an array 
/* Type for a variable symbol */
pub type VAR_SYMBOL = _var_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_block {
    pub psArrayVar: *mut VAR_SYMBOL,
    pub dimensions: SDWORD,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
}
// variable's identifier
// variable type
// Where the variable is stored
// The object type if this is an object variable
// Index of the variable in its data space
// Access functions if the variable is stored in an object/in-game
// number of dimensions of an array - 0 for normal var
// number of elements in an array 
/* Type for an array access block */
pub type ARRAY_BLOCK = _array_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _const_symbol {
    pub pIdent: *mut STRING,
    pub type_0: INTERP_TYPE,
    pub bval: BOOL,
    pub ival: SDWORD,
    pub oval: *mut libc::c_void,
    pub sval: *mut STRING,
}
// Number of debugging entries in psDebug.
// Debugging info for the script.
/* Type for a constant symbol */
pub type CONST_SYMBOL = _const_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _objvar_block {
    pub psObjVar: *mut VAR_SYMBOL,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
}
// variable's identifier
// variable type
/* The actual value of the constant. 
	 * Only one of these will be valid depending on type.
	 * A union is not used as a union cannot be statically initialised
	 */
//String values
//	float			fval;
/* The chunk of code used to reference an object variable */
pub type OBJVAR_BLOCK = _objvar_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _func_symbol {
    pub pIdent: *mut STRING,
    pub pFunc: SCRIPT_FUNC,
    pub type_0: INTERP_TYPE,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
    pub script: BOOL,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub location: UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
    pub psNext: *mut _func_symbol,
}
// The object variables symbol
// The code to get the object value on the stack
/* The maximum number of parameters for an instinct function */
/* Type for a function symbol */
pub type FUNC_SYMBOL = _func_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_decl {
    pub type_0: INTERP_TYPE,
    pub storage: STORAGE_TYPE,
}
// function's identifier
// Pointer to the instinct function
// function type
// Number of parameters to the function
// List of parameter types
// Whether the function is defined in the script
// or a C instinct function
// The size of script code
// The code for a function if it is defined in the script
// The position of the function in the final code block
// Number of debugging entries in psDebug.
// Debugging info for the script.
/* The type for a variable declaration */
pub type VAR_DECL = _var_decl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_decl {
    pub type_0: TRIGGER_TYPE,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub time: UDWORD,
}
/* The type for a trigger sub declaration */
pub type TRIGGER_DECL = _trigger_decl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_symbol {
    pub pIdent: *mut STRING,
    pub index: UDWORD,
    pub type_0: TRIGGER_TYPE,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub time: UDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
    pub psNext: *mut _trigger_symbol,
}
/* Type for a trigger symbol */
pub type TRIGGER_SYMBOL = _trigger_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _callback_symbol {
    pub pIdent: *mut STRING,
    pub type_0: TRIGGER_TYPE,
    pub pFunc: SCRIPT_FUNC,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
}
// Trigger's identifier
// The triggers index number
// Trigger type
// Code size for the trigger
// The trigger code
// How often to check the trigger
/* The type for a callback trigger symbol */
pub type CALLBACK_SYMBOL = _callback_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _event_symbol {
    pub pIdent: *mut STRING,
    pub index: UDWORD,
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub trigger: SDWORD,
    pub debugEntries: UDWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
    pub numParams: UDWORD,
    pub numLocalVars: UDWORD,
    pub bFunction: BOOL,
    pub bDeclared: BOOL,
    pub retType: INTERP_TYPE,
    pub aParams: [INTERP_TYPE; 20],
    pub psNext: *mut _event_symbol,
}
// Callback identifier
// user defined callback id >= TR_CALLBACKSTART
// Pointer to the instinct function
// Number of parameters to the function
// List of parameter types
/* Type for an event symbol */
pub type EVENT_SYMBOL = _event_symbol;
// Event's identifier
// the events index number
// Code size for the event
// Event code
// Index of the event's trigger
//functions stuff
//Number of parameters to the function
//local variables
//if this event is defined as a function
//if function was declared before
//return type if a function
/* Value type.  */
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub tval: INTERP_TYPE,
    pub stype: STORAGE_TYPE,
    pub vSymbol: *mut VAR_SYMBOL,
    pub cSymbol: *mut CONST_SYMBOL,
    pub fSymbol: *mut FUNC_SYMBOL,
    pub tSymbol: *mut TRIGGER_SYMBOL,
    pub eSymbol: *mut EVENT_SYMBOL,
    pub cbSymbol: *mut CALLBACK_SYMBOL,
    pub cblock: *mut CODE_BLOCK,
    pub condBlock: *mut COND_BLOCK,
    pub objVarBlock: *mut OBJVAR_BLOCK,
    pub arrayBlock: *mut ARRAY_BLOCK,
    pub pblock: *mut PARAM_BLOCK,
    pub pdecl: *mut PARAM_DECL,
    pub tdecl: *mut TRIGGER_DECL,
    pub integer_val: UDWORD,
    pub vdecl: *mut VAR_DECL,
    pub videcl: *mut VAR_IDENT_DECL,
}
static mut aText: [[STRING; 255]; 10] = [[0; 255]; 10];
static mut currText: UDWORD = 0 as libc::c_int as UDWORD;
static mut inComment: BOOL = 0 as libc::c_int;
static mut pInputBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut pEndBuffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Get the token type for a variable symbol */
#[no_mangle]
pub unsafe extern "C" fn scriptGetVarToken(mut psVar: *mut VAR_SYMBOL)
 -> SDWORD {
    let mut object: BOOL = 0;
    // See if this is an object pointer
    if asScrTypeTab.is_null() ||
           ((*psVar).type_0 as libc::c_uint) <
               VAL_USERTYPESTART as libc::c_int as libc::c_uint {
        object = 0 as libc::c_int
    } else {
        object =
            ((*asScrTypeTab.offset(((*psVar).type_0 as
                                        libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                       as isize)).accessType as libc::c_int ==
                 AT_OBJECT as libc::c_int) as libc::c_int
    }
    if (*psVar).storage as libc::c_int == ST_OBJECT as libc::c_int {
        /* This is an object member variable */
        if object != 0 {
            return 301 as libc::c_int
        } else {
            match (*psVar).type_0 as libc::c_uint {
                0 => { return 298 as libc::c_int }
                1 => {
                    //			case VAL_FLOAT:
                    return 299 as libc::c_int
                }
                _ => { return 300 as libc::c_int }
            }
        }
    } else if (*psVar).dimensions > 0 as libc::c_int {
        /* This is an array variable */
        if object != 0 {
            return 297 as libc::c_int
        } else {
            match (*psVar).type_0 as libc::c_uint {
                0 => { return 295 as libc::c_int }
                1 => {
                    //			case VAL_FLOAT:
                    return 296 as libc::c_int
                }
                _ => { return 294 as libc::c_int }
            }
        }
    } else if object != 0 {
        return 292 as libc::c_int
    } else {
        match (*psVar).type_0 as libc::c_uint {
            0 => { return 290 as libc::c_int }
            1 => {
                /* This is a standard variable */
                //			case VAL_FLOAT:
                return 291 as libc::c_int
            }
            2 => { return 293 as libc::c_int }
            _ => { return 289 as libc::c_int }
        }
    };
}
/* Get the token type for a constant symbol */
#[no_mangle]
pub unsafe extern "C" fn scriptGetConstToken(mut psConst: *mut CONST_SYMBOL)
 -> SDWORD {
    let mut object: BOOL = 0;
    // See if this is an object constant
    if asScrTypeTab.is_null() ||
           ((*psConst).type_0 as libc::c_uint) <
               VAL_USERTYPESTART as libc::c_int as libc::c_uint {
        object = 0 as libc::c_int
    } else {
        object =
            ((*asScrTypeTab.offset(((*psConst).type_0 as
                                        libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                       as isize)).accessType as libc::c_int ==
                 AT_OBJECT as libc::c_int) as libc::c_int
    }
    match (*psConst).type_0 as libc::c_uint {
        0 => { return 302 as libc::c_int }
        1 => {
            //	case VAL_FLOAT:
            return 303 as libc::c_int
        }
        2 => { return 306 as libc::c_int }
        _ => {
            if object != 0 {
                //debug(LOG_SCRIPT, "scriptGetConstToken: OBJ_CONSTANT");
                return 305 as libc::c_int
            } else { return 304 as libc::c_int }
        }
    };
}
/* Get the token type for a function symbol */
#[no_mangle]
pub unsafe extern "C" fn scriptGetFuncToken(mut psFunc: *mut FUNC_SYMBOL)
 -> SDWORD {
    let mut object: BOOL = 0;
    // See if this is an object pointer
    if (*psFunc).type_0 as libc::c_uint >=
           VAL_USERTYPESTART as libc::c_int as libc::c_uint {
        object =
            ((*asScrTypeTab.offset(((*psFunc).type_0 as
                                        libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                       as isize)).accessType as libc::c_int ==
                 AT_OBJECT as libc::c_int) as libc::c_int;
        if object != 0 { return 311 as libc::c_int }
    }
    match (*psFunc).type_0 as libc::c_uint {
        0 => { return 308 as libc::c_int }
        1 => {
            //	case VAL_FLOAT:
            return 309 as libc::c_int
        }
        2 => { return 312 as libc::c_int }
        5 => { return 307 as libc::c_int }
        _ => { return 310 as libc::c_int }
    };
}
/* Get the token type for a custom function symbol */
#[no_mangle]
pub unsafe extern "C" fn scriptGetCustomFuncToken(mut psFunc:
                                                      *mut EVENT_SYMBOL)
 -> SDWORD {
    let mut object: BOOL = 0;
    // See if this is an object pointer
    object =
        ((*asScrTypeTab.offset(((*psFunc).retType as
                                    libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                   as isize)).accessType as libc::c_int ==
             AT_OBJECT as libc::c_int) as libc::c_int;
    if object != 0 {
        return 317 as libc::c_int
    } else {
        match (*psFunc).retType as libc::c_uint {
            0 => { return 314 as libc::c_int }
            1 => {
                //		case VAL_FLOAT:
                return 315 as libc::c_int
            }
            2 => { return 318 as libc::c_int }
            5 => { return 313 as libc::c_int }
            _ => { return 316 as libc::c_int }
        }
    };
}
/* Convert a string to a long integer.  */
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char,
                  10 as libc::c_int);
}
/* Set the current input buffer for the lexer */
#[no_mangle]
pub unsafe extern "C" fn scriptSetInputBuffer(mut pBuffer: *mut libc::c_char,
                                              mut size: UDWORD) {
    pInputBuffer = pBuffer;
    pEndBuffer = pBuffer.offset(size as isize);
    /* Reset the lexer in case it's been used before */
    scr__flush_buffer(if !yy_buffer_stack.is_null() {
                          *yy_buffer_stack.offset(yy_buffer_stack_top as
                                                      isize)
                      } else { 0 as YY_BUFFER_STATE });
}
#[no_mangle]
pub unsafe extern "C" fn scriptGetErrorData(mut pLine: *mut libc::c_int,
                                            mut ppText:
                                                *mut *mut libc::c_char) {
    *pLine = scr_lineno;
    *ppText = scr_text;
}
/* Macros after this point can all be overridden by user definitions in
 * section 1.
 */
#[no_mangle]
pub unsafe extern "C" fn scr_wrap() -> libc::c_int {
    if inComment != 0 {
        debug(LOG_ERROR,
              b"Warning: reached end of file in a comment\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    return 1 as libc::c_int;
}
/* !YY_STRUCT_YY_BUFFER_STATE */
/* Stack of input buffers. */
static mut yy_buffer_stack_top: size_t = 0 as libc::c_int as size_t;
/* *< index of top of stack. */
static mut yy_buffer_stack_max: size_t = 0 as libc::c_int as size_t;
/* *< capacity of stack. */
static mut yy_buffer_stack: *mut YY_BUFFER_STATE =
    0 as *const YY_BUFFER_STATE as *mut YY_BUFFER_STATE;
/* yy_hold_char holds the character lost when yytext is formed. */
static mut yy_hold_char: libc::c_char = 0;
static mut yy_n_chars: libc::c_int = 0;
/* number of characters read into yy_ch_buf */
#[no_mangle]
pub static mut scr_leng: libc::c_int = 0;
/* Points to current character in buffer. */
static mut yy_c_buf_p: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut yy_init: libc::c_int = 0 as libc::c_int;
/* whether we need to initialize */
static mut yy_start: libc::c_int = 0 as libc::c_int;
/* start state number */
/* Flag which is used to allow yywrap()'s to do buffer switches
 * instead of setting up a fresh yyin.  A bit of a hack ...
 */
static mut yy_did_buffer_switch_on_eof: libc::c_int = 0;
#[no_mangle]
pub static mut scr_in: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut scr_out: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut scr_lineno: libc::c_int = 1 as libc::c_int;
static mut yy_accept: [flex_int16_t; 184] =
    [0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 56 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 47 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 59 as libc::c_int as flex_int16_t,
     57 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 57 as libc::c_int as flex_int16_t,
     45 as libc::c_int as flex_int16_t, 57 as libc::c_int as flex_int16_t,
     57 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     36 as libc::c_int as flex_int16_t, 57 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 52 as libc::c_int as flex_int16_t,
     53 as libc::c_int as flex_int16_t, 52 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 55 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 58 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 32 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 49 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 34 as libc::c_int as flex_int16_t,
     31 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     40 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 15 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 56 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 38 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     26 as libc::c_int as flex_int16_t, 42 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 37 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     25 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 41 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 8 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     24 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 27 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 17 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 6 as libc::c_int as flex_int16_t,
     7 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     28 as libc::c_int as flex_int16_t, 19 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     4 as libc::c_int as flex_int16_t, 2 as libc::c_int as flex_int16_t,
     30 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 13 as libc::c_int as flex_int16_t,
     18 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     14 as libc::c_int as flex_int16_t, 22 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 11 as libc::c_int as flex_int16_t,
     9 as libc::c_int as flex_int16_t, 21 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 12 as libc::c_int as flex_int16_t,
     3 as libc::c_int as flex_int16_t, 10 as libc::c_int as flex_int16_t,
     5 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t];
static mut yy_ec: [YY_CHAR; 256] =
    [0 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 2 as libc::c_int as YY_CHAR,
     3 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 2 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     2 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     5 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     6 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 7 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 8 as libc::c_int as YY_CHAR,
     9 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     9 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     9 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     9 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     9 as libc::c_int as YY_CHAR, 9 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     10 as libc::c_int as YY_CHAR, 11 as libc::c_int as YY_CHAR,
     12 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 13 as libc::c_int as YY_CHAR,
     14 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     16 as libc::c_int as YY_CHAR, 17 as libc::c_int as YY_CHAR,
     18 as libc::c_int as YY_CHAR, 19 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 20 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     21 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     22 as libc::c_int as YY_CHAR, 23 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     24 as libc::c_int as YY_CHAR, 25 as libc::c_int as YY_CHAR,
     26 as libc::c_int as YY_CHAR, 27 as libc::c_int as YY_CHAR,
     28 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 29 as libc::c_int as YY_CHAR,
     30 as libc::c_int as YY_CHAR, 31 as libc::c_int as YY_CHAR,
     32 as libc::c_int as YY_CHAR, 33 as libc::c_int as YY_CHAR,
     34 as libc::c_int as YY_CHAR, 35 as libc::c_int as YY_CHAR,
     36 as libc::c_int as YY_CHAR, 37 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 38 as libc::c_int as YY_CHAR,
     39 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     40 as libc::c_int as YY_CHAR, 41 as libc::c_int as YY_CHAR,
     42 as libc::c_int as YY_CHAR, 15 as libc::c_int as YY_CHAR,
     43 as libc::c_int as YY_CHAR, 44 as libc::c_int as YY_CHAR,
     45 as libc::c_int as YY_CHAR, 46 as libc::c_int as YY_CHAR,
     47 as libc::c_int as YY_CHAR, 48 as libc::c_int as YY_CHAR,
     49 as libc::c_int as YY_CHAR, 50 as libc::c_int as YY_CHAR,
     15 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR];
static mut yy_meta: [YY_CHAR; 51] =
    [0 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 2 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 3 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 1 as libc::c_int as YY_CHAR,
     1 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR, 4 as libc::c_int as YY_CHAR,
     4 as libc::c_int as YY_CHAR];
static mut yy_base: [flex_int16_t; 190] =
    [0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     49 as libc::c_int as flex_int16_t, 205 as libc::c_int as flex_int16_t,
     204 as libc::c_int as flex_int16_t, 53 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 206 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 194 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 195 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 194 as libc::c_int as flex_int16_t,
     191 as libc::c_int as flex_int16_t, 190 as libc::c_int as flex_int16_t,
     189 as libc::c_int as flex_int16_t, 177 as libc::c_int as flex_int16_t,
     175 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     184 as libc::c_int as flex_int16_t, 174 as libc::c_int as flex_int16_t,
     172 as libc::c_int as flex_int16_t, 170 as libc::c_int as flex_int16_t,
     167 as libc::c_int as flex_int16_t, 168 as libc::c_int as flex_int16_t,
     168 as libc::c_int as flex_int16_t, 150 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 14 as libc::c_int as flex_int16_t,
     35 as libc::c_int as flex_int16_t, 31 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     144 as libc::c_int as flex_int16_t, 39 as libc::c_int as flex_int16_t,
     153 as libc::c_int as flex_int16_t, 140 as libc::c_int as flex_int16_t,
     141 as libc::c_int as flex_int16_t, 142 as libc::c_int as flex_int16_t,
     38 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 174 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     172 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     209 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     151 as libc::c_int as flex_int16_t, 150 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 151 as libc::c_int as flex_int16_t,
     147 as libc::c_int as flex_int16_t, 153 as libc::c_int as flex_int16_t,
     140 as libc::c_int as flex_int16_t, 130 as libc::c_int as flex_int16_t,
     126 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     131 as libc::c_int as flex_int16_t, 128 as libc::c_int as flex_int16_t,
     126 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 125 as libc::c_int as flex_int16_t,
     133 as libc::c_int as flex_int16_t, 118 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 116 as libc::c_int as flex_int16_t,
     124 as libc::c_int as flex_int16_t, 130 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 116 as libc::c_int as flex_int16_t,
     32 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     120 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     152 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     133 as libc::c_int as flex_int16_t, 128 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     132 as libc::c_int as flex_int16_t, 134 as libc::c_int as flex_int16_t,
     134 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     110 as libc::c_int as flex_int16_t, 115 as libc::c_int as flex_int16_t,
     36 as libc::c_int as flex_int16_t, 102 as libc::c_int as flex_int16_t,
     102 as libc::c_int as flex_int16_t, 114 as libc::c_int as flex_int16_t,
     113 as libc::c_int as flex_int16_t, 98 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 104 as libc::c_int as flex_int16_t,
     112 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     96 as libc::c_int as flex_int16_t, 92 as libc::c_int as flex_int16_t,
     99 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     91 as libc::c_int as flex_int16_t, 99 as libc::c_int as flex_int16_t,
     100 as libc::c_int as flex_int16_t, 101 as libc::c_int as flex_int16_t,
     101 as libc::c_int as flex_int16_t, 87 as libc::c_int as flex_int16_t,
     92 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 113 as libc::c_int as flex_int16_t,
     107 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 83 as libc::c_int as flex_int16_t,
     77 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     93 as libc::c_int as flex_int16_t, 80 as libc::c_int as flex_int16_t,
     79 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 84 as libc::c_int as flex_int16_t,
     89 as libc::c_int as flex_int16_t, 92 as libc::c_int as flex_int16_t,
     83 as libc::c_int as flex_int16_t, 76 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 82 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 83 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 96 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 77 as libc::c_int as flex_int16_t,
     76 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     80 as libc::c_int as flex_int16_t, 70 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     65 as libc::c_int as flex_int16_t, 55 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     43 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     0 as libc::c_int as flex_int16_t, 209 as libc::c_int as flex_int16_t,
     88 as libc::c_int as flex_int16_t, 92 as libc::c_int as flex_int16_t,
     96 as libc::c_int as flex_int16_t, 71 as libc::c_int as flex_int16_t,
     100 as libc::c_int as flex_int16_t, 104 as libc::c_int as flex_int16_t];
static mut yy_def: [flex_int16_t; 190] =
    [0 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 184 as libc::c_int as flex_int16_t,
     184 as libc::c_int as flex_int16_t, 185 as libc::c_int as flex_int16_t,
     185 as libc::c_int as flex_int16_t, 186 as libc::c_int as flex_int16_t,
     186 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     188 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     189 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 188 as libc::c_int as flex_int16_t,
     189 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     187 as libc::c_int as flex_int16_t, 0 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t];
static mut yy_nxt: [flex_int16_t; 260] =
    [0 as libc::c_int as flex_int16_t, 10 as libc::c_int as flex_int16_t,
     11 as libc::c_int as flex_int16_t, 12 as libc::c_int as flex_int16_t,
     13 as libc::c_int as flex_int16_t, 14 as libc::c_int as flex_int16_t,
     10 as libc::c_int as flex_int16_t, 15 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 17 as libc::c_int as flex_int16_t,
     18 as libc::c_int as flex_int16_t, 19 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 21 as libc::c_int as flex_int16_t,
     22 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     24 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     25 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     26 as libc::c_int as flex_int16_t, 27 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 28 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     30 as libc::c_int as flex_int16_t, 31 as libc::c_int as flex_int16_t,
     32 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     34 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 36 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 38 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 40 as libc::c_int as flex_int16_t,
     41 as libc::c_int as flex_int16_t, 42 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 23 as libc::c_int as flex_int16_t,
     23 as libc::c_int as flex_int16_t, 46 as libc::c_int as flex_int16_t,
     46 as libc::c_int as flex_int16_t, 72 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 47 as libc::c_int as flex_int16_t,
     51 as libc::c_int as flex_int16_t, 51 as libc::c_int as flex_int16_t,
     52 as libc::c_int as flex_int16_t, 52 as libc::c_int as flex_int16_t,
     55 as libc::c_int as flex_int16_t, 73 as libc::c_int as flex_int16_t,
     56 as libc::c_int as flex_int16_t, 74 as libc::c_int as flex_int16_t,
     75 as libc::c_int as flex_int16_t, 77 as libc::c_int as flex_int16_t,
     79 as libc::c_int as flex_int16_t, 90 as libc::c_int as flex_int16_t,
     83 as libc::c_int as flex_int16_t, 122 as libc::c_int as flex_int16_t,
     80 as libc::c_int as flex_int16_t, 78 as libc::c_int as flex_int16_t,
     110 as libc::c_int as flex_int16_t, 119 as libc::c_int as flex_int16_t,
     91 as libc::c_int as flex_int16_t, 60 as libc::c_int as flex_int16_t,
     135 as libc::c_int as flex_int16_t, 182 as libc::c_int as flex_int16_t,
     123 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     111 as libc::c_int as flex_int16_t, 76 as libc::c_int as flex_int16_t,
     84 as libc::c_int as flex_int16_t, 181 as libc::c_int as flex_int16_t,
     120 as libc::c_int as flex_int16_t, 85 as libc::c_int as flex_int16_t,
     180 as libc::c_int as flex_int16_t, 179 as libc::c_int as flex_int16_t,
     112 as libc::c_int as flex_int16_t, 45 as libc::c_int as flex_int16_t,
     45 as libc::c_int as flex_int16_t, 45 as libc::c_int as flex_int16_t,
     45 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 48 as libc::c_int as flex_int16_t,
     48 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 50 as libc::c_int as flex_int16_t,
     50 as libc::c_int as flex_int16_t, 93 as libc::c_int as flex_int16_t,
     178 as libc::c_int as flex_int16_t, 93 as libc::c_int as flex_int16_t,
     93 as libc::c_int as flex_int16_t, 94 as libc::c_int as flex_int16_t,
     177 as libc::c_int as flex_int16_t, 176 as libc::c_int as flex_int16_t,
     94 as libc::c_int as flex_int16_t, 175 as libc::c_int as flex_int16_t,
     174 as libc::c_int as flex_int16_t, 173 as libc::c_int as flex_int16_t,
     172 as libc::c_int as flex_int16_t, 171 as libc::c_int as flex_int16_t,
     170 as libc::c_int as flex_int16_t, 169 as libc::c_int as flex_int16_t,
     168 as libc::c_int as flex_int16_t, 167 as libc::c_int as flex_int16_t,
     166 as libc::c_int as flex_int16_t, 165 as libc::c_int as flex_int16_t,
     164 as libc::c_int as flex_int16_t, 163 as libc::c_int as flex_int16_t,
     162 as libc::c_int as flex_int16_t, 161 as libc::c_int as flex_int16_t,
     160 as libc::c_int as flex_int16_t, 159 as libc::c_int as flex_int16_t,
     158 as libc::c_int as flex_int16_t, 157 as libc::c_int as flex_int16_t,
     156 as libc::c_int as flex_int16_t, 155 as libc::c_int as flex_int16_t,
     154 as libc::c_int as flex_int16_t, 153 as libc::c_int as flex_int16_t,
     152 as libc::c_int as flex_int16_t, 151 as libc::c_int as flex_int16_t,
     150 as libc::c_int as flex_int16_t, 149 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     146 as libc::c_int as flex_int16_t, 145 as libc::c_int as flex_int16_t,
     144 as libc::c_int as flex_int16_t, 143 as libc::c_int as flex_int16_t,
     142 as libc::c_int as flex_int16_t, 141 as libc::c_int as flex_int16_t,
     140 as libc::c_int as flex_int16_t, 139 as libc::c_int as flex_int16_t,
     138 as libc::c_int as flex_int16_t, 137 as libc::c_int as flex_int16_t,
     134 as libc::c_int as flex_int16_t, 133 as libc::c_int as flex_int16_t,
     132 as libc::c_int as flex_int16_t, 131 as libc::c_int as flex_int16_t,
     130 as libc::c_int as flex_int16_t, 129 as libc::c_int as flex_int16_t,
     128 as libc::c_int as flex_int16_t, 127 as libc::c_int as flex_int16_t,
     126 as libc::c_int as flex_int16_t, 125 as libc::c_int as flex_int16_t,
     124 as libc::c_int as flex_int16_t, 121 as libc::c_int as flex_int16_t,
     118 as libc::c_int as flex_int16_t, 117 as libc::c_int as flex_int16_t,
     116 as libc::c_int as flex_int16_t, 115 as libc::c_int as flex_int16_t,
     114 as libc::c_int as flex_int16_t, 113 as libc::c_int as flex_int16_t,
     109 as libc::c_int as flex_int16_t, 108 as libc::c_int as flex_int16_t,
     107 as libc::c_int as flex_int16_t, 106 as libc::c_int as flex_int16_t,
     105 as libc::c_int as flex_int16_t, 104 as libc::c_int as flex_int16_t,
     103 as libc::c_int as flex_int16_t, 102 as libc::c_int as flex_int16_t,
     101 as libc::c_int as flex_int16_t, 100 as libc::c_int as flex_int16_t,
     99 as libc::c_int as flex_int16_t, 98 as libc::c_int as flex_int16_t,
     97 as libc::c_int as flex_int16_t, 96 as libc::c_int as flex_int16_t,
     95 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     92 as libc::c_int as flex_int16_t, 89 as libc::c_int as flex_int16_t,
     88 as libc::c_int as flex_int16_t, 87 as libc::c_int as flex_int16_t,
     86 as libc::c_int as flex_int16_t, 82 as libc::c_int as flex_int16_t,
     81 as libc::c_int as flex_int16_t, 71 as libc::c_int as flex_int16_t,
     70 as libc::c_int as flex_int16_t, 69 as libc::c_int as flex_int16_t,
     68 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     66 as libc::c_int as flex_int16_t, 65 as libc::c_int as flex_int16_t,
     64 as libc::c_int as flex_int16_t, 63 as libc::c_int as flex_int16_t,
     62 as libc::c_int as flex_int16_t, 61 as libc::c_int as flex_int16_t,
     59 as libc::c_int as flex_int16_t, 58 as libc::c_int as flex_int16_t,
     57 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     54 as libc::c_int as flex_int16_t, 53 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 49 as libc::c_int as flex_int16_t,
     49 as libc::c_int as flex_int16_t, 9 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t];
static mut yy_chk: [flex_int16_t; 260] =
    [0 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 1 as libc::c_int as flex_int16_t,
     1 as libc::c_int as flex_int16_t, 3 as libc::c_int as flex_int16_t,
     4 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     3 as libc::c_int as flex_int16_t, 4 as libc::c_int as flex_int16_t,
     7 as libc::c_int as flex_int16_t, 8 as libc::c_int as flex_int16_t,
     7 as libc::c_int as flex_int16_t, 8 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     16 as libc::c_int as flex_int16_t, 33 as libc::c_int as flex_int16_t,
     34 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     36 as libc::c_int as flex_int16_t, 44 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 88 as libc::c_int as flex_int16_t,
     36 as libc::c_int as flex_int16_t, 35 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 86 as libc::c_int as flex_int16_t,
     44 as libc::c_int as flex_int16_t, 187 as libc::c_int as flex_int16_t,
     106 as libc::c_int as flex_int16_t, 178 as libc::c_int as flex_int16_t,
     88 as libc::c_int as flex_int16_t, 106 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 34 as libc::c_int as flex_int16_t,
     39 as libc::c_int as flex_int16_t, 177 as libc::c_int as flex_int16_t,
     86 as libc::c_int as flex_int16_t, 39 as libc::c_int as flex_int16_t,
     176 as libc::c_int as flex_int16_t, 172 as libc::c_int as flex_int16_t,
     78 as libc::c_int as flex_int16_t, 184 as libc::c_int as flex_int16_t,
     184 as libc::c_int as flex_int16_t, 184 as libc::c_int as flex_int16_t,
     184 as libc::c_int as flex_int16_t, 185 as libc::c_int as flex_int16_t,
     185 as libc::c_int as flex_int16_t, 185 as libc::c_int as flex_int16_t,
     185 as libc::c_int as flex_int16_t, 186 as libc::c_int as flex_int16_t,
     186 as libc::c_int as flex_int16_t, 186 as libc::c_int as flex_int16_t,
     186 as libc::c_int as flex_int16_t, 188 as libc::c_int as flex_int16_t,
     171 as libc::c_int as flex_int16_t, 188 as libc::c_int as flex_int16_t,
     188 as libc::c_int as flex_int16_t, 189 as libc::c_int as flex_int16_t,
     170 as libc::c_int as flex_int16_t, 167 as libc::c_int as flex_int16_t,
     189 as libc::c_int as flex_int16_t, 166 as libc::c_int as flex_int16_t,
     165 as libc::c_int as flex_int16_t, 164 as libc::c_int as flex_int16_t,
     163 as libc::c_int as flex_int16_t, 160 as libc::c_int as flex_int16_t,
     159 as libc::c_int as flex_int16_t, 155 as libc::c_int as flex_int16_t,
     153 as libc::c_int as flex_int16_t, 149 as libc::c_int as flex_int16_t,
     148 as libc::c_int as flex_int16_t, 147 as libc::c_int as flex_int16_t,
     146 as libc::c_int as flex_int16_t, 145 as libc::c_int as flex_int16_t,
     144 as libc::c_int as flex_int16_t, 143 as libc::c_int as flex_int16_t,
     140 as libc::c_int as flex_int16_t, 139 as libc::c_int as flex_int16_t,
     138 as libc::c_int as flex_int16_t, 136 as libc::c_int as flex_int16_t,
     135 as libc::c_int as flex_int16_t, 130 as libc::c_int as flex_int16_t,
     129 as libc::c_int as flex_int16_t, 126 as libc::c_int as flex_int16_t,
     125 as libc::c_int as flex_int16_t, 124 as libc::c_int as flex_int16_t,
     123 as libc::c_int as flex_int16_t, 122 as libc::c_int as flex_int16_t,
     121 as libc::c_int as flex_int16_t, 120 as libc::c_int as flex_int16_t,
     118 as libc::c_int as flex_int16_t, 117 as libc::c_int as flex_int16_t,
     116 as libc::c_int as flex_int16_t, 114 as libc::c_int as flex_int16_t,
     113 as libc::c_int as flex_int16_t, 111 as libc::c_int as flex_int16_t,
     110 as libc::c_int as flex_int16_t, 109 as libc::c_int as flex_int16_t,
     108 as libc::c_int as flex_int16_t, 107 as libc::c_int as flex_int16_t,
     105 as libc::c_int as flex_int16_t, 104 as libc::c_int as flex_int16_t,
     102 as libc::c_int as flex_int16_t, 101 as libc::c_int as flex_int16_t,
     100 as libc::c_int as flex_int16_t, 97 as libc::c_int as flex_int16_t,
     96 as libc::c_int as flex_int16_t, 92 as libc::c_int as flex_int16_t,
     91 as libc::c_int as flex_int16_t, 90 as libc::c_int as flex_int16_t,
     89 as libc::c_int as flex_int16_t, 87 as libc::c_int as flex_int16_t,
     85 as libc::c_int as flex_int16_t, 84 as libc::c_int as flex_int16_t,
     83 as libc::c_int as flex_int16_t, 81 as libc::c_int as flex_int16_t,
     80 as libc::c_int as flex_int16_t, 79 as libc::c_int as flex_int16_t,
     76 as libc::c_int as flex_int16_t, 75 as libc::c_int as flex_int16_t,
     74 as libc::c_int as flex_int16_t, 73 as libc::c_int as flex_int16_t,
     72 as libc::c_int as flex_int16_t, 71 as libc::c_int as flex_int16_t,
     70 as libc::c_int as flex_int16_t, 69 as libc::c_int as flex_int16_t,
     68 as libc::c_int as flex_int16_t, 67 as libc::c_int as flex_int16_t,
     65 as libc::c_int as flex_int16_t, 64 as libc::c_int as flex_int16_t,
     63 as libc::c_int as flex_int16_t, 62 as libc::c_int as flex_int16_t,
     61 as libc::c_int as flex_int16_t, 54 as libc::c_int as flex_int16_t,
     47 as libc::c_int as flex_int16_t, 43 as libc::c_int as flex_int16_t,
     42 as libc::c_int as flex_int16_t, 41 as libc::c_int as flex_int16_t,
     40 as libc::c_int as flex_int16_t, 38 as libc::c_int as flex_int16_t,
     37 as libc::c_int as flex_int16_t, 32 as libc::c_int as flex_int16_t,
     31 as libc::c_int as flex_int16_t, 30 as libc::c_int as flex_int16_t,
     29 as libc::c_int as flex_int16_t, 28 as libc::c_int as flex_int16_t,
     27 as libc::c_int as flex_int16_t, 26 as libc::c_int as flex_int16_t,
     25 as libc::c_int as flex_int16_t, 24 as libc::c_int as flex_int16_t,
     22 as libc::c_int as flex_int16_t, 21 as libc::c_int as flex_int16_t,
     20 as libc::c_int as flex_int16_t, 19 as libc::c_int as flex_int16_t,
     18 as libc::c_int as flex_int16_t, 17 as libc::c_int as flex_int16_t,
     15 as libc::c_int as flex_int16_t, 13 as libc::c_int as flex_int16_t,
     9 as libc::c_int as flex_int16_t, 6 as libc::c_int as flex_int16_t,
     5 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t,
     183 as libc::c_int as flex_int16_t, 183 as libc::c_int as flex_int16_t];
/* Table of booleans, true if rule could match eol. */
static mut yy_rule_can_match_eol: [flex_int32_t; 59] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
     0 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
static mut yy_last_accepting_state: yy_state_type = 0;
static mut yy_last_accepting_cpos: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut scr__flex_debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut scr_text: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* __ia64__ */
/* Copy whatever the last rule matched to the standard output. */
/* This used to be an fputs(), but since the string might contain NUL's,
 * we now use fwrite().
 */
/* Gets input and stuffs it into "buf".  number of characters read, or YY_NULL,
 * is returned in "result".
 */
/* No semi-colon after return; correct usage is to write "yyterminate();" -
 * we don't want an extra ';' after the "return" because that will cause
 * some compilers to complain about unreachable statements.
 */
/* Number of entries by which start-condition stack grows. */
/* Report a fatal error. */
/* end tables serialization structures and prototypes */
/* Default declaration of generated scanner - a define so the user can
 * easily add parameters.
 */
/* !YY_DECL */
/* Code executed at the beginning of each rule, after yytext and yyleng
 * have been set up.
 */
/* Code executed at the end of each rule. */
/*LINTED*/
/* * The main scanner function which does all the work.
 */
#[no_mangle]
pub unsafe extern "C" fn scr_lex() -> libc::c_int {
    let mut yy_amount_of_matched_text: libc::c_int =
        0; /* first start state */
    let mut yy_next_state: yy_state_type = 0;
    let mut current_block: u64;
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_bp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut yy_act: libc::c_int = 0;
    if yy_init == 0 {
        yy_init = 1 as libc::c_int;
        if yy_start == 0 { yy_start = 1 as libc::c_int }
        if scr_in.is_null() { scr_in = stdin }
        if scr_out.is_null() { scr_out = stdout }
        if if !yy_buffer_stack.is_null() {
               *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
           } else { 0 as YY_BUFFER_STATE }.is_null() {
            scr_ensure_buffer_stack();
            let ref mut fresh0 =
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            *fresh0 = scr__create_buffer(scr_in, 16384 as libc::c_int)
        }
        scr__load_buffer_state();
    }
    loop 
         /* loops until end-of-file is reached */
         {
        yy_cp = yy_c_buf_p;
        /* end of action switch */
        /* Support of yytext. */
        *yy_cp = yy_hold_char;
        /* yy_bp points to the position in yy_ch_buf of the start of
		 * the current run.
		 */
        yy_bp = yy_cp;
        yy_current_state = yy_start;
        'c_16816:
            loop  {
                loop  {
                    let mut yy_c: YY_CHAR = yy_ec[*yy_cp as YY_CHAR as usize];
                    if yy_accept[yy_current_state as usize] != 0 {
                        yy_last_accepting_state = yy_current_state;
                        yy_last_accepting_cpos = yy_cp
                    }
                    while yy_chk[(yy_base[yy_current_state as usize] as
                                      libc::c_int + yy_c as libc::c_int) as
                                     usize] as libc::c_int != yy_current_state
                          {
                        yy_current_state =
                            yy_def[yy_current_state as usize] as libc::c_int;
                        if yy_current_state >= 184 as libc::c_int {
                            yy_c = yy_meta[yy_c as usize]
                        }
                    }
                    yy_current_state =
                        yy_nxt[(yy_base[yy_current_state as usize] as
                                    libc::c_int + yy_c as libc::c_int) as
                                   usize] as yy_state_type;
                    yy_cp = yy_cp.offset(1);
                    if !(yy_base[yy_current_state as usize] as libc::c_int !=
                             209 as libc::c_int) {
                        break ;
                    }
                }
                'c_16817:
                    loop  {
                        yy_act =
                            yy_accept[yy_current_state as usize] as
                                libc::c_int;
                        if yy_act == 0 as libc::c_int {
                            /* have to back up */
                            yy_cp = yy_last_accepting_cpos;
                            yy_current_state = yy_last_accepting_state;
                            yy_act =
                                yy_accept[yy_current_state as usize] as
                                    libc::c_int
                        }
                        scr_text = yy_bp;
                        scr_leng =
                            yy_cp.wrapping_offset_from(yy_bp) as libc::c_int;
                        yy_hold_char = *yy_cp;
                        *yy_cp = '\u{0}' as i32 as libc::c_char;
                        yy_c_buf_p = yy_cp;
                        if yy_act != 59 as libc::c_int &&
                               yy_rule_can_match_eol[yy_act as usize] != 0 {
                            let mut yyl: libc::c_int = 0;
                            yyl = 0 as libc::c_int;
                            while yyl < scr_leng {
                                if *scr_text.offset(yyl as isize) as
                                       libc::c_int == '\n' as i32 {
                                    scr_lineno += 1
                                }
                                yyl += 1
                            }
                        }
                        loop 
                             /* This label is used only to access EOF actions. */
                             {
                            match yy_act {
                                0 => {
                                    /* beginning of action switch */
                                    /* must back up */
                                    /* undo the effects of YY_DO_BEFORE_ACTION */
                                    *yy_cp = yy_hold_char;
                                    yy_cp = yy_last_accepting_cpos;
                                    yy_current_state =
                                        yy_last_accepting_state;
                                    continue 'c_16817 ;
                                }
                                1 => { return 261 as libc::c_int }
                                2 => { return 262 as libc::c_int }
                                3 => { return 259 as libc::c_int }
                                4 => { return 260 as libc::c_int }
                                5 => { return 263 as libc::c_int }
                                6 => { return 264 as libc::c_int }
                                7 => { return 265 as libc::c_int }
                                8 => { return 266 as libc::c_int }
                                9 => { return 267 as libc::c_int }
                                10 => { return 258 as libc::c_int }
                                11 => {
                                    scr_lval.stype =
                                        ST_PUBLIC as libc::c_int as
                                            STORAGE_TYPE;
                                    return 287 as libc::c_int
                                }
                                12 => {
                                    scr_lval.stype =
                                        ST_PRIVATE as libc::c_int as
                                            STORAGE_TYPE;
                                    return 287 as libc::c_int
                                }
                                13 => {
                                    scr_lval.stype =
                                        ST_LOCAL as libc::c_int as
                                            STORAGE_TYPE;
                                    return 287 as libc::c_int
                                }
                                14 => { return 268 as libc::c_int }
                                15 => { return 269 as libc::c_int }
                                16 => { return 270 as libc::c_int }
                                17 => { return 271 as libc::c_int }
                                18 => { return 272 as libc::c_int }
                                19 => {
                                    scr_lval.tval = VAL_VOID;
                                    return 286 as libc::c_int
                                }
                                20 => {
                                    scr_lval.tval = VAL_VOID;
                                    return 286 as libc::c_int
                                }
                                21 => {
                                    scr_lval.tval = VAL_STRING;
                                    return 286 as libc::c_int
                                }
                                22 => {
                                    scr_lval.tval = VAL_STRING;
                                    return 286 as libc::c_int
                                }
                                23 => {
                                    scr_lval.tval = VAL_BOOL;
                                    return 286 as libc::c_int
                                }
                                24 => {
                                    scr_lval.tval = VAL_BOOL;
                                    return 286 as libc::c_int
                                }
                                25 => {
                                    scr_lval.tval = VAL_INT;
                                    return 286 as libc::c_int
                                }
                                26 => {
                                    scr_lval.tval = VAL_INT;
                                    return 286 as libc::c_int
                                }
                                27 => {
                                    scr_lval.bval = 1 as libc::c_int;
                                    return 283 as libc::c_int
                                }
                                28 => {
                                    scr_lval.bval = 1 as libc::c_int;
                                    return 283 as libc::c_int
                                }
                                29 => {
                                    scr_lval.bval = 0 as libc::c_int;
                                    return 283 as libc::c_int
                                }
                                30 => {
                                    scr_lval.bval = 0 as libc::c_int;
                                    return 283 as libc::c_int
                                }
                                31 => { return 273 as libc::c_int }
                                32 => { return 274 as libc::c_int }
                                33 => { return 275 as libc::c_int }
                                34 => { return 276 as libc::c_int }
                                35 => { return 277 as libc::c_int }
                                36 => { return 278 as libc::c_int }
                                37 => { return 279 as libc::c_int }
                                38 => { return 279 as libc::c_int }
                                39 => { return 280 as libc::c_int }
                                40 => { return 280 as libc::c_int }
                                41 => { return 281 as libc::c_int }
                                42 => { return 281 as libc::c_int }
                                43 => {
                                    scr_lval.ival = atol(scr_text) as SDWORD;
                                    return 284 as libc::c_int
                                }
                                44 => {
                                    if scriptLookUpType(scr_text,
                                                        &mut scr_lval.tval) !=
                                           0 {
                                        return 286 as libc::c_int
                                    } else if scriptLookUpVariable(scr_text,
                                                                   &mut scr_lval.vSymbol)
                                                  != 0 {
                                        return scriptGetVarToken(scr_lval.vSymbol)
                                    } else if scriptLookUpConstant(scr_text,
                                                                   &mut scr_lval.cSymbol)
                                                  != 0 {
                                        return scriptGetConstToken(scr_lval.cSymbol)
                                    } else if scriptLookUpFunction(scr_text,
                                                                   &mut scr_lval.fSymbol)
                                                  != 0 {
                                        return scriptGetFuncToken(scr_lval.fSymbol)
                                    } else if scriptLookUpCustomFunction(scr_text,
                                                                         &mut scr_lval.eSymbol)
                                                  != 0 {
                                        return scriptGetCustomFuncToken(scr_lval.eSymbol)
                                    } else if scriptLookUpTrigger(scr_text,
                                                                  &mut scr_lval.tSymbol)
                                                  != 0 {
                                        return 319 as libc::c_int
                                    } else if scriptLookUpEvent(scr_text,
                                                                &mut scr_lval.eSymbol)
                                                  != 0 {
                                        return 320 as libc::c_int
                                    } else if scriptLookUpCallback(scr_text,
                                                                   &mut scr_lval.cbSymbol)
                                                  != 0 {
                                        return 321 as libc::c_int
                                    } else {
                                        strcpy(aText[currText as
                                                         usize].as_mut_ptr(),
                                               scr_text);
                                        scr_lval.sval =
                                            aText[currText as
                                                      usize].as_mut_ptr();
                                        currText =
                                            currText.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_rem(10
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint);
                                        return 288 as libc::c_int
                                    }
                                }
                                45 => {
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                3 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                46 => {
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                0 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                47 => {
                                    strcpy(aText[currText as
                                                     usize].as_mut_ptr(),
                                           scr_text);
                                    scr_lval.sval =
                                        aText[currText as usize].as_mut_ptr();
                                    currText =
                                        currText.wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_rem(10
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint);
                                    return 285 as libc::c_int
                                }
                                49 => {
                                    inComment = 1 as libc::c_int;
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                1 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                50 | 51 => {
                                    inComment = 0 as libc::c_int;
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                0 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                54 => {
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                2 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                55 => {
                                    yy_start =
                                        1 as libc::c_int +
                                            2 as libc::c_int *
                                                0 as libc::c_int;
                                    break 'c_16816 ;
                                }
                                48 | 52 | 53 | 56 => { break 'c_16816 ; }
                                57 => {
                                    return *scr_text.offset(0 as libc::c_int
                                                                as isize) as
                                               libc::c_int
                                }
                                58 => {
                                    (fwrite(scr_text as *const libc::c_void,
                                            scr_leng as size_t,
                                            1 as libc::c_int as libc::c_uint,
                                            scr_out)) != 0;
                                    break 'c_16816 ;
                                }
                                60 | 61 | 62 | 63 => {
                                    return 0 as libc::c_int
                                }
                                59 => {
                                    /* Amount of text matched not including the EOB char. */
                                    yy_amount_of_matched_text =
                                        yy_cp.wrapping_offset_from(scr_text)
                                            as libc::c_int - 1 as libc::c_int;
                                    /* Undo the effects of YY_DO_BEFORE_ACTION. */
                                    *yy_cp = yy_hold_char;
                                    if (**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                     as
                                                                     isize)).yy_buffer_status
                                           == 0 as libc::c_int {
                                        /* We're scanning a new file or input source.  It's
			 * possible that this happened because the user
			 * just pointed yyin at a new source and called
			 * yylex().  If so, then we have to assure
			 * consistency between YY_CURRENT_BUFFER and our
			 * globals.  Here is the right place to do so, because
			 * this is the first action (other than possibly a
			 * back-up) that will match for the new input source.
			 */
                                        yy_n_chars =
                                            (**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                          as
                                                                          isize)).yy_n_chars;
                                        let ref mut fresh1 =
                                            (**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                          as
                                                                          isize)).yy_input_file;
                                        *fresh1 = scr_in;
                                        (**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                      as
                                                                      isize)).yy_buffer_status
                                            = 1 as libc::c_int
                                    }
                                    /* Note that here we test for yy_c_buf_p "<=" to the position
		 * of the first EOB in the buffer, since yy_c_buf_p will
		 * already have been incremented past the NUL character
		 * (since all states make transitions on EOB to the
		 * end-of-buffer state).  Contrast this with the test
		 * in input().
		 */
                                    if yy_c_buf_p <=
                                           &mut *(**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                               as
                                                                               isize)).yy_ch_buf.offset(yy_n_chars
                                                                                                            as
                                                                                                            isize)
                                               as *mut libc::c_char {
                                        /* This was really a NUL. */
                                        yy_next_state = 0;
                                        yy_c_buf_p =
                                            scr_text.offset(yy_amount_of_matched_text
                                                                as isize);
                                        yy_current_state =
                                            yy_get_previous_state();
                                        /* Okay, we're now positioned to make the NUL
			 * transition.  We couldn't have
			 * yy_get_previous_state() go ahead and do it
			 * for us because it doesn't know how to deal
			 * with the possibility of jamming (and we don't
			 * want to build jamming into it because then it
			 * will run more slowly).
			 */
                                        yy_next_state =
                                            yy_try_NUL_trans(yy_current_state);
                                        yy_bp =
                                            scr_text.offset(0 as libc::c_int
                                                                as isize);
                                        if yy_next_state != 0 {
                                            current_block =
                                                5267916556966421873;
                                            break ;
                                        } else {
                                            current_block =
                                                17809788131590720680;
                                            break ;
                                        }
                                    } else {
                                        match yy_get_next_buffer() {
                                            1 => {
                                                yy_did_buffer_switch_on_eof =
                                                    0 as libc::c_int;
                                                if scr_wrap() != 0 {
                                                    /* Note: because we've taken care in
					 * yy_get_next_buffer() to have set up
					 * yytext, we can now set up
					 * yy_c_buf_p so that if some total
					 * hoser (like flex itself) wants to
					 * call the scanner after we return the
					 * YY_NULL, it'll still work - another
					 * YY_NULL will get returned.
					 */
                                                    yy_c_buf_p =
                                                        scr_text.offset(0 as
                                                                            libc::c_int
                                                                            as
                                                                            isize);
                                                    yy_act =
                                                        59 as libc::c_int +
                                                            (yy_start -
                                                                 1 as
                                                                     libc::c_int)
                                                                /
                                                                2 as
                                                                    libc::c_int
                                                            + 1 as libc::c_int
                                                } else {
                                                    if yy_did_buffer_switch_on_eof
                                                           == 0 {
                                                        scr_restart(scr_in);
                                                    }
                                                    break 'c_16816 ;
                                                }
                                            }
                                            0 => {
                                                yy_c_buf_p =
                                                    scr_text.offset(yy_amount_of_matched_text
                                                                        as
                                                                        isize);
                                                yy_current_state =
                                                    yy_get_previous_state();
                                                yy_cp = yy_c_buf_p;
                                                yy_bp =
                                                    scr_text.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize);
                                                break 'c_16817 ;
                                            }
                                            2 => {
                                                yy_c_buf_p =
                                                    &mut *(**yy_buffer_stack.offset(yy_buffer_stack_top
                                                                                        as
                                                                                        isize)).yy_ch_buf.offset(yy_n_chars
                                                                                                                     as
                                                                                                                     isize)
                                                        as *mut libc::c_char;
                                                yy_current_state =
                                                    yy_get_previous_state();
                                                yy_cp = yy_c_buf_p;
                                                yy_bp =
                                                    scr_text.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize);
                                                continue 'c_16817 ;
                                            }
                                            _ => { break 'c_16816 ; }
                                        }
                                    }
                                }
                                _ => {
                                    yy_fatal_error(b"fatal flex scanner internal error--no action found\x00"
                                                       as *const u8 as
                                                       *const libc::c_char);
                                }
                            }
                        }
                        match current_block {
                            17809788131590720680 => { yy_cp = yy_c_buf_p }
                            _ => {
                                /* Consume the NUL. */
                                yy_c_buf_p = yy_c_buf_p.offset(1);
                                yy_cp = yy_c_buf_p;
                                yy_current_state = yy_next_state;
                                break ;
                            }
                        }
                    }
            }
    };
    /* end of scanning one token */
    /* end of user's declarations */
}
/* end of yylex */
/* yy_get_next_buffer - try to read in a new buffer
 *
 * Returns a code representing an action:
 *	EOB_ACT_LAST_MATCH -
 *	EOB_ACT_CONTINUE_SCAN - continue scanning from current position
 *	EOB_ACT_END_OF_FILE - end of file
 */
unsafe extern "C" fn yy_get_next_buffer() -> libc::c_int {
    let mut dest: *mut libc::c_char =
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_ch_buf;
    let mut source: *mut libc::c_char = scr_text;
    let mut number_to_move: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0;
    if yy_c_buf_p >
           &mut *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                               isize)).yy_ch_buf.offset((yy_n_chars
                                                                             +
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                                                                            as
                                                                            isize)
               as *mut libc::c_char {
        yy_fatal_error(b"fatal flex scanner internal error--end of buffer missed\x00"
                           as *const u8 as *const libc::c_char);
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_fill_buffer
           == 0 as libc::c_int {
        /* Don't try to fill the buffer, so this is an EOF. */
        if yy_c_buf_p.wrapping_offset_from(scr_text) as libc::c_int -
               0 as libc::c_int == 1 as libc::c_int {
            /* We matched a single character, the EOB, so
			 * treat this as a final EOF.
			 */
            return 1 as libc::c_int
        } else {
            /* We matched some text prior to the EOB, first
			 * process it.
			 */
            return 2 as libc::c_int
        }
    }
    /* Try to read more data. */
    /* First move last chars to start of buffer. */
    number_to_move =
        yy_c_buf_p.wrapping_offset_from(scr_text) as libc::c_int -
            1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < number_to_move {
        let fresh2 = source;
        source = source.offset(1);
        let fresh3 = dest;
        dest = dest.offset(1);
        *fresh3 = *fresh2;
        i += 1
    }
    if (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                     isize)).yy_buffer_status ==
           2 as libc::c_int {
        /* don't do the read, it's not guaranteed to return an EOF,
		 * just force an EOF
		 */
        yy_n_chars = 0 as libc::c_int;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars =
            yy_n_chars
    } else {
        let mut num_to_read: libc::c_int =
            (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                          isize)).yy_buf_size - number_to_move
                - 1 as libc::c_int;
        while num_to_read <= 0 as libc::c_int {
            /* Not enough room in the buffer - grow it. */
            /* just a shorter name for the current buffer */
            let mut b: YY_BUFFER_STATE =
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
            let mut yy_c_buf_p_offset: libc::c_int =
                yy_c_buf_p.wrapping_offset_from((*b).yy_ch_buf) as
                    libc::c_int;
            if (*b).yy_is_our_buffer != 0 {
                let mut new_size: libc::c_int =
                    (*b).yy_buf_size * 2 as libc::c_int;
                if new_size <= 0 as libc::c_int {
                    (*b).yy_buf_size += (*b).yy_buf_size / 8 as libc::c_int
                } else { (*b).yy_buf_size *= 2 as libc::c_int }
                (*b).yy_ch_buf =
                    scr_realloc((*b).yy_ch_buf as *mut libc::c_void,
                                ((*b).yy_buf_size + 2 as libc::c_int) as
                                    yy_size_t) as *mut libc::c_char
            } else {
                /* Can't grow it, we don't own it. */
                (*b).yy_ch_buf = 0 as *mut libc::c_char
            }
            if (*b).yy_ch_buf.is_null() {
                yy_fatal_error(b"fatal error - scanner input buffer overflow\x00"
                                   as *const u8 as *const libc::c_char);
            }
            yy_c_buf_p =
                &mut *(*b).yy_ch_buf.offset(yy_c_buf_p_offset as isize) as
                    *mut libc::c_char;
            num_to_read =
                (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                              isize)).yy_buf_size -
                    number_to_move - 1 as libc::c_int
        }
        if num_to_read > 8192 as libc::c_int {
            num_to_read = 8192 as libc::c_int
        }
        /* Read in more data. */
        if pInputBuffer != pEndBuffer {
            let fresh4 = pInputBuffer;
            pInputBuffer = pInputBuffer.offset(1);
            *(&mut *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                                  isize)).yy_ch_buf.offset(number_to_move
                                                                               as
                                                                               isize)
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                *fresh4;
            yy_n_chars = 1 as libc::c_int
        } else {
            *(&mut *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                                  isize)).yy_ch_buf.offset(number_to_move
                                                                               as
                                                                               isize)
                  as *mut libc::c_char).offset(0 as libc::c_int as isize) =
                -(1 as libc::c_int) as libc::c_char;
            yy_n_chars = 0 as libc::c_int
        }
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars =
            yy_n_chars
    }
    if yy_n_chars == 0 as libc::c_int {
        if number_to_move == 0 as libc::c_int {
            ret_val = 1 as libc::c_int;
            scr_restart(scr_in);
        } else {
            ret_val = 2 as libc::c_int;
            (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                          isize)).yy_buffer_status =
                2 as libc::c_int
        }
    } else { ret_val = 0 as libc::c_int }
    if yy_n_chars + number_to_move >
           (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                         isize)).yy_buf_size {
        /* Extend the array by 50%, plus the number we really need. */
        let mut new_size_0: libc::c_int =
            yy_n_chars + number_to_move + (yy_n_chars >> 1 as libc::c_int);
        let ref mut fresh5 =
            (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                          isize)).yy_ch_buf;
        *fresh5 =
            scr_realloc((**yy_buffer_stack.offset(yy_buffer_stack_top as
                                                      isize)).yy_ch_buf as
                            *mut libc::c_void, new_size_0 as yy_size_t) as
                *mut libc::c_char;
        if (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                         isize)).yy_ch_buf.is_null() {
            yy_fatal_error(b"out of dynamic memory in yy_get_next_buffer()\x00"
                               as *const u8 as *const libc::c_char);
        }
        /* "- 2" to take care of EOB's */
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_size =
            new_size_0 - 2 as libc::c_int
    }
    yy_n_chars += number_to_move;
    *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                   isize)).yy_ch_buf.offset(yy_n_chars as
                                                                isize) =
        0 as libc::c_int as libc::c_char;
    *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                   isize)).yy_ch_buf.offset((yy_n_chars +
                                                                 1 as
                                                                     libc::c_int)
                                                                as isize) =
        0 as libc::c_int as libc::c_char;
    scr_text =
        &mut *(**yy_buffer_stack.offset(yy_buffer_stack_top as
                                            isize)).yy_ch_buf.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)
            as *mut libc::c_char;
    return ret_val;
}
/* yy_get_previous_state - get the state just before the EOB char was reached */
unsafe extern "C" fn yy_get_previous_state() -> yy_state_type {
    let mut yy_current_state: yy_state_type = 0;
    let mut yy_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    yy_current_state = yy_start;
    yy_cp = scr_text.offset(0 as libc::c_int as isize);
    while yy_cp < yy_c_buf_p {
        let mut yy_c: YY_CHAR =
            if *yy_cp as libc::c_int != 0 {
                yy_ec[*yy_cp as YY_CHAR as usize] as libc::c_int
            } else { 1 as libc::c_int } as YY_CHAR;
        if yy_accept[yy_current_state as usize] != 0 {
            yy_last_accepting_state = yy_current_state;
            yy_last_accepting_cpos = yy_cp
        }
        while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int +
                          yy_c as libc::c_int) as usize] as libc::c_int !=
                  yy_current_state {
            yy_current_state =
                yy_def[yy_current_state as usize] as libc::c_int;
            if yy_current_state >= 184 as libc::c_int {
                yy_c = yy_meta[yy_c as usize]
            }
        }
        yy_current_state =
            yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int +
                        yy_c as libc::c_int) as usize] as yy_state_type;
        yy_cp = yy_cp.offset(1)
    }
    return yy_current_state;
}
/* yy_try_NUL_trans - try to make a transition on the NUL character
 *
 * synopsis
 *	next_state = yy_try_NUL_trans( current_state );
 */
unsafe extern "C" fn yy_try_NUL_trans(mut yy_current_state: yy_state_type)
 -> yy_state_type {
    let mut yy_is_jam: libc::c_int = 0;
    let mut yy_cp: *mut libc::c_char = yy_c_buf_p;
    let mut yy_c: YY_CHAR = 1 as libc::c_int as YY_CHAR;
    if yy_accept[yy_current_state as usize] != 0 {
        yy_last_accepting_state = yy_current_state;
        yy_last_accepting_cpos = yy_cp
    }
    while yy_chk[(yy_base[yy_current_state as usize] as libc::c_int +
                      yy_c as libc::c_int) as usize] as libc::c_int !=
              yy_current_state {
        yy_current_state = yy_def[yy_current_state as usize] as libc::c_int;
        if yy_current_state >= 184 as libc::c_int {
            yy_c = yy_meta[yy_c as usize]
        }
    }
    yy_current_state =
        yy_nxt[(yy_base[yy_current_state as usize] as libc::c_int +
                    yy_c as libc::c_int) as usize] as yy_state_type;
    yy_is_jam = (yy_current_state == 183 as libc::c_int) as libc::c_int;
    return if yy_is_jam != 0 { 0 as libc::c_int } else { yy_current_state };
}
/* ifndef YY_NO_INPUT */
/* * Immediately switch to a different input stream.
 * @param input_file A readable stream.
 * 
 * @note This function does not reset the start condition to @c INITIAL .
 */
#[no_mangle]
pub unsafe extern "C" fn scr_restart(mut input_file: *mut FILE) {
    if if !yy_buffer_stack.is_null() {
           *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        scr_ensure_buffer_stack();
        let ref mut fresh6 =
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh6 = scr__create_buffer(scr_in, 16384 as libc::c_int)
    }
    scr__init_buffer(if !yy_buffer_stack.is_null() {
                         *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
                     } else { 0 as YY_BUFFER_STATE }, input_file);
    scr__load_buffer_state();
}
/* * Switch to a different input buffer.
 * @param new_buffer The new input buffer.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr__switch_to_buffer(mut new_buffer:
                                                   YY_BUFFER_STATE) {
    /* TODO. We should be able to replace this entire function body
	 * with
	 *		yypop_buffer_state();
	 *		yypush_buffer_state(new_buffer);
     */
    scr_ensure_buffer_stack();
    if (if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else { 0 as YY_BUFFER_STATE }) == new_buffer {
        return
    }
    if !if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        /* Flush out information for old buffer. */
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh7 =
            (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                          isize)).yy_buf_pos;
        *fresh7 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars =
            yy_n_chars
    }
    let ref mut fresh8 =
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh8 = new_buffer;
    scr__load_buffer_state();
    /* We don't actually know whether we did this switch during
	 * EOF (yywrap()) processing, but the only time this flag
	 * is looked at is after yywrap() is called, so it's safe
	 * to go ahead and always set it.
	 */
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
unsafe extern "C" fn scr__load_buffer_state() {
    yy_n_chars =
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars;
    yy_c_buf_p =
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_buf_pos;
    scr_text = yy_c_buf_p;
    scr_in =
        (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                      isize)).yy_input_file;
    yy_hold_char = *yy_c_buf_p;
}
/* * Allocate and initialize an input buffer state.
 * @param file A readable stream.
 * @param size The character buffer size in bytes. When in doubt, use @c YY_BUF_SIZE.
 * 
 * @return the allocated buffer state.
 */
#[no_mangle]
pub unsafe extern "C" fn scr__create_buffer(mut file: *mut FILE,
                                            mut size: libc::c_int)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    b =
        scr_alloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
            as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_create_buffer()\x00" as
                           *const u8 as *const libc::c_char);
    }
    (*b).yy_buf_size = size;
    /* yy_ch_buf has to be 2 characters longer than the size given because
	 * we need to put in 2 end-of-buffer characters.
	 */
    (*b).yy_ch_buf =
        scr_alloc(((*b).yy_buf_size + 2 as libc::c_int) as yy_size_t) as
            *mut libc::c_char;
    if (*b).yy_ch_buf.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_create_buffer()\x00" as
                           *const u8 as *const libc::c_char);
    }
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    scr__init_buffer(b, file);
    return b;
}
/* * Destroy the buffer.
 * @param b a buffer created with yy_create_buffer()
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr__delete_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() { return }
    if b ==
           (if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else { 0 as YY_BUFFER_STATE }) {
        /* Not sure if we should pop here. */
        let ref mut fresh9 =
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh9 = 0 as YY_BUFFER_STATE
    }
    if (*b).yy_is_our_buffer != 0 {
        scr_free((*b).yy_ch_buf as *mut libc::c_void);
    }
    scr_free(b as *mut libc::c_void);
}
/* Initializes or reinitializes a buffer.
 * This function is sometimes called more than once on the same buffer,
 * such as during a yyrestart() or at EOF.
 */
unsafe extern "C" fn scr__init_buffer(mut b: YY_BUFFER_STATE,
                                      mut file: *mut FILE) {
    let mut oerrno: libc::c_int = *__errno_location();
    scr__flush_buffer(b);
    (*b).yy_input_file = file;
    (*b).yy_fill_buffer = 1 as libc::c_int;
    /* If b is the current buffer, then yy_init_buffer was _probably_
     * called from yyrestart() or through yy_get_next_buffer.
     * In that case, we don't want to reset the lineno or column.
     */
    if b !=
           (if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else { 0 as YY_BUFFER_STATE }) {
        (*b).yy_bs_lineno = 1 as libc::c_int;
        (*b).yy_bs_column = 0 as libc::c_int
    }
    (*b).yy_is_interactive =
        if !file.is_null() {
            (isatty(fileno(file)) > 0 as libc::c_int) as libc::c_int
        } else { 0 as libc::c_int };
    *__errno_location() = oerrno;
}
/* * Discard all buffered characters. On the next scan, YY_INPUT will be called.
 * @param b the buffer state to be flushed, usually @c YY_CURRENT_BUFFER.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr__flush_buffer(mut b: YY_BUFFER_STATE) {
    if b.is_null() { return }
    (*b).yy_n_chars = 0 as libc::c_int;
    /* We always need two end-of-buffer characters.  The first causes
	 * a transition to the end-of-buffer state.  The second causes
	 * a jam in that state.
	 */
    *(*b).yy_ch_buf.offset(0 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    *(*b).yy_ch_buf.offset(1 as libc::c_int as isize) =
        0 as libc::c_int as libc::c_char;
    (*b).yy_buf_pos =
        &mut *(*b).yy_ch_buf.offset(0 as libc::c_int as isize) as
            *mut libc::c_char;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    if b ==
           (if !yy_buffer_stack.is_null() {
                *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
            } else { 0 as YY_BUFFER_STATE }) {
        scr__load_buffer_state();
    };
}
/* * Pushes the new state onto the stack. The new state becomes
 *  the current state. This function will allocate the stack
 *  if necessary.
 *  @param new_buffer The new state.
 *  
 */
#[no_mangle]
pub unsafe extern "C" fn scr_push_buffer_state(mut new_buffer:
                                                   YY_BUFFER_STATE) {
    if new_buffer.is_null() { return }
    scr_ensure_buffer_stack();
    /* This block is copied from yy_switch_to_buffer. */
    if !if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        /* Flush out information for old buffer. */
        *yy_c_buf_p = yy_hold_char;
        let ref mut fresh10 =
            (**yy_buffer_stack.offset(yy_buffer_stack_top as
                                          isize)).yy_buf_pos;
        *fresh10 = yy_c_buf_p;
        (**yy_buffer_stack.offset(yy_buffer_stack_top as isize)).yy_n_chars =
            yy_n_chars
    }
    /* Only push if top exists. Otherwise, replace top. */
    if !if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_add(1)
    }
    let ref mut fresh11 =
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh11 = new_buffer;
    /* copied from yy_switch_to_buffer. */
    scr__load_buffer_state();
    yy_did_buffer_switch_on_eof = 1 as libc::c_int;
}
/* * Removes and deletes the top of the stack, if present.
 *  The next element becomes the new top.
 *  
 */
#[no_mangle]
pub unsafe extern "C" fn scr_pop_buffer_state() {
    if if !yy_buffer_stack.is_null() {
           *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
       } else { 0 as YY_BUFFER_STATE }.is_null() {
        return
    }
    scr__delete_buffer(if !yy_buffer_stack.is_null() {
                           *yy_buffer_stack.offset(yy_buffer_stack_top as
                                                       isize)
                       } else { 0 as YY_BUFFER_STATE });
    let ref mut fresh12 =
        *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
    *fresh12 = 0 as YY_BUFFER_STATE;
    if yy_buffer_stack_top > 0 as libc::c_int as libc::c_uint {
        yy_buffer_stack_top = yy_buffer_stack_top.wrapping_sub(1)
    }
    if !if !yy_buffer_stack.is_null() {
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
        } else { 0 as YY_BUFFER_STATE }.is_null() {
        scr__load_buffer_state();
        yy_did_buffer_switch_on_eof = 1 as libc::c_int
    };
}
/* Allocates the stack if it does not exist.
 *  Guarantees space for at least one push.
 */
unsafe extern "C" fn scr_ensure_buffer_stack() {
    let mut num_to_alloc: yy_size_t = 0;
    if yy_buffer_stack.is_null() {
        /* First allocation is just for 2 elements, since we don't know if this
		 * scanner will even need a stack. We use 2 instead of 1 to avoid an
		 * immediate realloc on the next call.
         */
        num_to_alloc =
            1 as libc::c_int as
                yy_size_t; /* After all that talk, this was set to 1 anyways... */
        yy_buffer_stack =
            scr_alloc(num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                                    as libc::c_ulong)) as
                *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(b"out of dynamic memory in yyensure_buffer_stack()\x00"
                               as *const u8 as *const libc::c_char);
        }
        memset(yy_buffer_stack as *mut libc::c_void, 0 as libc::c_int,
               num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                             as libc::c_ulong));
        yy_buffer_stack_max = num_to_alloc;
        yy_buffer_stack_top = 0 as libc::c_int as size_t;
        return
    }
    if yy_buffer_stack_top >=
           yy_buffer_stack_max.wrapping_sub(1 as libc::c_int as libc::c_uint)
       {
        /* Increase the buffer to prepare for a possible push. */
        let mut grow_size: yy_size_t = 8 as libc::c_int as yy_size_t;
        num_to_alloc = yy_buffer_stack_max.wrapping_add(grow_size);
        yy_buffer_stack =
            scr_realloc(yy_buffer_stack as *mut libc::c_void,
                        num_to_alloc.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                                      as libc::c_ulong)) as
                *mut *mut yy_buffer_state;
        if yy_buffer_stack.is_null() {
            yy_fatal_error(b"out of dynamic memory in yyensure_buffer_stack()\x00"
                               as *const u8 as *const libc::c_char);
        }
        /* arbitrary grow size */
        memset(yy_buffer_stack.offset(yy_buffer_stack_max as isize) as
                   *mut libc::c_void, 0 as libc::c_int,
               grow_size.wrapping_mul(::std::mem::size_of::<*mut yy_buffer_state>()
                                          as libc::c_ulong));
        yy_buffer_stack_max = num_to_alloc
    };
}
/* zero only the new slots.*/
/* * Setup the input buffer state to scan directly from a user-specified character buffer.
 * @param base the character buffer
 * @param size the size in bytes of the character buffer
 * 
 * @return the newly allocated buffer state object.
 */
#[no_mangle]
pub unsafe extern "C" fn scr__scan_buffer(mut base: *mut libc::c_char,
                                          mut size: yy_size_t)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    if size < 2 as libc::c_int as libc::c_uint ||
           *base.offset(size.wrapping_sub(2 as libc::c_int as libc::c_uint) as
                            isize) as libc::c_int != 0 as libc::c_int ||
           *base.offset(size.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                            isize) as libc::c_int != 0 as libc::c_int {
        /* They forgot to leave room for the EOB's. */
        return 0 as YY_BUFFER_STATE
    } /* "- 2" to take care of EOB's */
    b =
        scr_alloc(::std::mem::size_of::<yy_buffer_state>() as libc::c_ulong)
            as YY_BUFFER_STATE;
    if b.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_scan_buffer()\x00" as
                           *const u8 as *const libc::c_char);
    }
    (*b).yy_buf_size =
        size.wrapping_sub(2 as libc::c_int as libc::c_uint) as libc::c_int;
    (*b).yy_ch_buf = base;
    (*b).yy_buf_pos = (*b).yy_ch_buf;
    (*b).yy_is_our_buffer = 0 as libc::c_int;
    (*b).yy_input_file = 0 as *mut FILE;
    (*b).yy_n_chars = (*b).yy_buf_size;
    (*b).yy_is_interactive = 0 as libc::c_int;
    (*b).yy_at_bol = 1 as libc::c_int;
    (*b).yy_fill_buffer = 0 as libc::c_int;
    (*b).yy_buffer_status = 0 as libc::c_int;
    scr__switch_to_buffer(b);
    return b;
}
/* * Setup the input buffer state to scan a string. The next call to yylex() will
 * scan from a @e copy of @a str.
 * @param yystr a NUL-terminated string to scan
 * 
 * @return the newly allocated buffer state object.
 * @note If you want to scan bytes that may contain NUL values, then use
 *       yy_scan_bytes() instead.
 */
#[no_mangle]
pub unsafe extern "C" fn scr__scan_string(mut yystr: *const libc::c_char)
 -> YY_BUFFER_STATE {
    return scr__scan_bytes(yystr, strlen(yystr) as libc::c_int);
}
/* * Setup the input buffer state to scan the given bytes. The next call to yylex() will
 * scan from a @e copy of @a bytes.
 * @param yybytes the byte buffer to scan
 * @param _yybytes_len the number of bytes in the buffer pointed to by @a bytes.
 * 
 * @return the newly allocated buffer state object.
 */
#[no_mangle]
pub unsafe extern "C" fn scr__scan_bytes(mut yybytes: *const libc::c_char,
                                         mut _yybytes_len: libc::c_int)
 -> YY_BUFFER_STATE {
    let mut b: YY_BUFFER_STATE = 0 as *mut yy_buffer_state;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: yy_size_t = 0;
    let mut i: libc::c_int = 0;
    /* Get memory for full buffer, including space for trailing EOB's. */
    n = (_yybytes_len + 2 as libc::c_int) as yy_size_t;
    buf = scr_alloc(n) as *mut libc::c_char;
    if buf.is_null() {
        yy_fatal_error(b"out of dynamic memory in yy_scan_bytes()\x00" as
                           *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < _yybytes_len {
        *buf.offset(i as isize) = *yybytes.offset(i as isize);
        i += 1
    }
    let ref mut fresh13 =
        *buf.offset((_yybytes_len + 1 as libc::c_int) as isize);
    *fresh13 = 0 as libc::c_int as libc::c_char;
    *buf.offset(_yybytes_len as isize) = *fresh13;
    b = scr__scan_buffer(buf, n);
    if b.is_null() {
        yy_fatal_error(b"bad buffer in yy_scan_bytes()\x00" as *const u8 as
                           *const libc::c_char);
    }
    /* It's okay to grow etc. this buffer, and we should throw it
	 * away when we're done.
	 */
    (*b).yy_is_our_buffer = 1 as libc::c_int;
    return b;
}
unsafe extern "C" fn yy_fatal_error(mut msg: *const libc::c_char) -> ! {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
    exit(2 as libc::c_int);
}
/* Redefine yyless() so it works in section 3 code. */
/* Undo effects of setting up yytext. */
/* Accessor  methods (get/set functions) to struct members. */
/* * Get the current line number.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_get_lineno() -> libc::c_int {
    return scr_lineno;
}
/* * Get the input stream.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_get_in() -> *mut FILE { return scr_in; }
/* * Get the output stream.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_get_out() -> *mut FILE { return scr_out; }
/* * Get the length of the current token.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_get_leng() -> libc::c_int { return scr_leng; }
/* * Get the current token.
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_get_text() -> *mut libc::c_char {
    return scr_text;
}
/* * Set the current line number.
 * @param _line_number line number
 * 
 */
#[no_mangle]
pub unsafe extern "C" fn scr_set_lineno(mut _line_number: libc::c_int) {
    scr_lineno = _line_number;
}
/* * Set the input stream. This does not discard the current
 * input buffer.
 * @param _in_str A readable stream.
 * 
 * @see yy_switch_to_buffer
 */
#[no_mangle]
pub unsafe extern "C" fn scr_set_in(mut _in_str: *mut FILE) {
    scr_in = _in_str;
}
#[no_mangle]
pub unsafe extern "C" fn scr_set_out(mut _out_str: *mut FILE) {
    scr_out = _out_str;
}
#[no_mangle]
pub unsafe extern "C" fn scr_get_debug() -> libc::c_int {
    return scr__flex_debug;
}
#[no_mangle]
pub unsafe extern "C" fn scr_set_debug(mut _bdebug: libc::c_int) {
    scr__flex_debug = _bdebug;
}
/* Special case for "unistd.h", since it is non-ANSI. We include it way
 * down here because we want the user's section 1 to have been scanned first.
 * The user has a chance to override it with an option.
 */
unsafe extern "C" fn yy_init_globals() -> libc::c_int {
    /* Initialization is the same as for the non-reentrant scanner.
     * This function is called from yylex_destroy(), so don't allocate here.
     */
    /* We do not touch yylineno unless the option is enabled. */
    scr_lineno = 1 as libc::c_int;
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    yy_buffer_stack_top = 0 as libc::c_int as size_t;
    yy_buffer_stack_max = 0 as libc::c_int as size_t;
    yy_c_buf_p = 0 as *mut libc::c_char;
    yy_init = 0 as libc::c_int;
    yy_start = 0 as libc::c_int;
    /* Defined in main.c */
    scr_in = 0 as *mut FILE;
    scr_out = 0 as *mut FILE;
    /* For future reference: Set errno on error, since we are called by
     * yylex_init()
     */
    return 0 as libc::c_int;
}
/* Accessor methods to globals.
   These are made visible to non-reentrant scanners for convenience. */
/* yylex_destroy is for both reentrant and non-reentrant scanners. */
#[no_mangle]
pub unsafe extern "C" fn scr_lex_destroy() -> libc::c_int {
    /* Pop the buffer stack, destroying each element. */
    while !if !yy_buffer_stack.is_null() {
               *yy_buffer_stack.offset(yy_buffer_stack_top as isize)
           } else { 0 as YY_BUFFER_STATE }.is_null() {
        scr__delete_buffer(if !yy_buffer_stack.is_null() {
                               *yy_buffer_stack.offset(yy_buffer_stack_top as
                                                           isize)
                           } else { 0 as YY_BUFFER_STATE });
        let ref mut fresh14 =
            *yy_buffer_stack.offset(yy_buffer_stack_top as isize);
        *fresh14 = 0 as YY_BUFFER_STATE;
        scr_pop_buffer_state();
    }
    /* Destroy the stack itself. */
    scr_free(yy_buffer_stack as *mut libc::c_void);
    yy_buffer_stack = 0 as *mut YY_BUFFER_STATE;
    /* Reset the globals. This is important in a non-reentrant scanner so the next time
     * yylex() is called, initialization will occur. */
    yy_init_globals();
    return 0 as libc::c_int;
}
/*
 * Internal utility routines.
 */
#[no_mangle]
pub unsafe extern "C" fn scr_alloc(mut size: yy_size_t) -> *mut libc::c_void {
    return malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn scr_realloc(mut ptr: *mut libc::c_void,
                                     mut size: yy_size_t)
 -> *mut libc::c_void {
    /* The cast to (char *) in the following accommodates both
	 * implementations that use char* generic pointers, and those
	 * that use void* generic pointers.  It works with the latter
	 * because both ANSI C and C++ allow castless assignment from
	 * any pointer type to void*, and deal with argument conversions
	 * as though doing an assignment.
	 */
    return realloc(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn scr_free(mut ptr: *mut libc::c_void) {
    free(ptr as *mut libc::c_char as *mut libc::c_void);
    /* see yyrealloc() for (char *) cast */
}
