use ::libc;
extern "C" {
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
    /* The size of each opcode */
    #[no_mangle]
    static mut aOpSize: [SDWORD; 0];
    /* The table of user types */
    #[no_mangle]
    static mut asScrTypeTab: *mut TYPE_SYMBOL;
    /* The table of external variables and their access functions */
    #[no_mangle]
    static mut asScrExternalTab: *mut VAR_SYMBOL;
    /* The table of object variable access routines */
    #[no_mangle]
    static mut asScrObjectVarTab: *mut VAR_SYMBOL;
    /* The table of instinct function type definitions */
    #[no_mangle]
    static mut asScrInstinctTab: *mut FUNC_SYMBOL;
    /* The table of callback triggers */
    #[no_mangle]
    static mut asScrCallbackTab: *mut CALLBACK_SYMBOL;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_val {
    pub type_0: INTERP_TYPE,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub oval: *mut libc::c_void,
    pub pVoid: *mut libc::c_void,
}
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
/* A value consists of its type and value */
pub type INTERP_VAL = _interp_val;
pub type _op_code = libc::c_uint;
pub const OP_PUSHLOCALREF: _op_code = 34;
pub const OP_PUSHLOCAL: _op_code = 33;
pub const OP_POPLOCAL: _op_code = 32;
pub const OP_FUNC: _op_code = 31;
pub const OP_LESS: _op_code = 30;
pub const OP_GREATER: _op_code = 29;
pub const OP_LESSEQUAL: _op_code = 28;
pub const OP_GREATEREQUAL: _op_code = 27;
pub const OP_NOTEQUAL: _op_code = 26;
pub const OP_EQUAL: _op_code = 25;
pub const OP_CANC: _op_code = 24;
pub const OP_NOT: _op_code = 23;
pub const OP_OR: _op_code = 22;
pub const OP_AND: _op_code = 21;
pub const OP_NEG: _op_code = 20;
pub const OP_DIV: _op_code = 19;
pub const OP_MUL: _op_code = 18;
pub const OP_SUB: _op_code = 17;
pub const OP_ADD: _op_code = 16;
pub const OP_PAUSE: _op_code = 15;
pub const OP_EXIT: _op_code = 14;
pub const OP_UNARYOP: _op_code = 13;
pub const OP_BINARYOP: _op_code = 12;
pub const OP_JUMPFALSE: _op_code = 11;
pub const OP_JUMPTRUE: _op_code = 10;
pub const OP_JUMP: _op_code = 9;
pub const OP_VARCALL: _op_code = 8;
pub const OP_CALL: _op_code = 7;
pub const OP_POPARRAYGLOBAL: _op_code = 6;
pub const OP_PUSHARRAYGLOBAL: _op_code = 5;
pub const OP_POPGLOBAL: _op_code = 4;
pub const OP_PUSHGLOBAL: _op_code = 3;
pub const OP_POP: _op_code = 2;
pub const OP_PUSHREF: _op_code = 1;
pub const OP_PUSH: _op_code = 0;
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
/* Opcodes for the script interpreter */
pub type OPCODE = _op_code;
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
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
pub struct _var_debug {
    pub pIdent: *mut STRING,
    pub storage: STORAGE_TYPE,
}
/* Variable debugging info for a script */
pub type VAR_DEBUG = _var_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_data {
    pub base: UDWORD,
    pub type_0: UBYTE,
    pub dimensions: UBYTE,
    pub elements: [UBYTE; 4],
}
/* Array info for a script */
pub type ARRAY_DATA = _array_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_debug {
    pub pIdent: *mut STRING,
    pub storage: UBYTE,
}
// the base index of the array values
// the array data type
/* Array debug info for a script */
pub type ARRAY_DEBUG = _array_debug;
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
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
/* Description of a trigger for the SCRIPT_CODE */
pub type TRIGGER_DATA = _trigger_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_code {
    pub size: UDWORD,
    pub pCode: *mut UDWORD,
    pub numTriggers: UWORD,
    pub numEvents: UWORD,
    pub pTriggerTab: *mut UWORD,
    pub psTriggerData: *mut TRIGGER_DATA,
    pub pEventTab: *mut UWORD,
    pub pEventLinks: *mut SWORD,
    pub numGlobals: UWORD,
    pub numArrays: UWORD,
    pub arraySize: UDWORD,
    pub pGlobals: *mut INTERP_TYPE,
    pub ppsLocalVars: *mut *mut INTERP_TYPE,
    pub numLocalVars: *mut UDWORD,
    pub ppsLocalVarVal: *mut *mut INTERP_VAL,
    pub numParams: *mut UDWORD,
    pub psVarDebug: *mut VAR_DEBUG,
    pub psArrayInfo: *mut ARRAY_DATA,
    pub psArrayDebug: *mut ARRAY_DEBUG,
    pub debugEntries: UWORD,
    pub psDebug: *mut SCRIPT_DEBUG,
}
// Type of trigger
// BOOL - is there code with this trigger
// How often to check the trigger
/* A compiled script and its associated data */
pub type SCRIPT_CODE = _script_code;
// The size (in bytes) of the compiled code
// Pointer to the compiled code
// The number of triggers
// The number of events
// The table of trigger offsets
// The extra info for each trigger
// The table of event offsets
// The original trigger/event linkage
// -1 for no link
// The number of global variables
// the number of arrays in the program
// the number of elements in all the defined arrays
// Types of the global variables
//storage for local vars (type)
//number of local vars each event has
//Values of the local vars used during interpreting process
//number of arguments this event has
// The names and storage types of variables
// The sizes of the program arrays
// Debug info for the arrays
// Number of entries in psDebug
// Debugging info for the script
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
// The type id to use in the type field of values
// Whether the type is an object or a simple value
// Type identifier
// load and save functions
// 
/* Type for a variable symbol */
pub type VAR_SYMBOL = _var_symbol;
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
// variable's identifier
// variable type
// Where the variable is stored
// The object type if this is an object variable
// Index of the variable in its data space
// Access functions if the variable is stored in an object/in-game
// number of dimensions of an array - 0 for normal var
// number of elements in an array 
/* The maximum number of parameters for an instinct function */
/* Type for a function symbol */
pub type FUNC_SYMBOL = _func_symbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _callback_symbol {
    pub pIdent: *mut STRING,
    pub type_0: TRIGGER_TYPE,
    pub pFunc: SCRIPT_FUNC,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
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
/* The type for a callback trigger symbol */
pub type CALLBACK_SYMBOL = _callback_symbol;
// Callback identifier
// user defined callback id >= TR_CALLBACKSTART
// Pointer to the instinct function
// Number of parameters to the function
// List of parameter types
/*
 * CodePrint.h
 *
 * Routines for displaying compiled scripts
 *
 */
/* Display a value type */
/*
 * CodePrint.c
 *
 * Routines for displaying compiled scripts
 */
/* Display a value type */
#[no_mangle]
pub unsafe extern "C" fn cpPrintType(mut type_0: INTERP_TYPE) {
    let mut i: UDWORD = 0;
    let mut ref_0: BOOL = 0 as libc::c_int;
    if type_0 as libc::c_uint & 0x100000 as libc::c_int as libc::c_uint != 0 {
        ref_0 = 1 as libc::c_int;
        type_0 =
            (type_0 as libc::c_uint &
                 !(0x100000 as libc::c_int) as libc::c_uint) as INTERP_TYPE
    }
    match type_0 as libc::c_uint {
        0 => {
            debug(LOG_NEVER, b"BOOL\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            debug(LOG_NEVER, b"INT\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            /*	case VAL_FLOAT:
		debug( LOG_NEVER, "FLOAT" );
		break;*/
            debug(LOG_NEVER,
                  b"STRING\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            debug(LOG_NEVER,
                  b"TRIGGER\x00" as *const u8 as *const libc::c_char);
        }
        4 => {
            debug(LOG_NEVER,
                  b"EVENT\x00" as *const u8 as *const libc::c_char);
        }
        5 => {
            debug(LOG_NEVER, b"VOID\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            // See if it is a user defined type
            if !asScrTypeTab.is_null() {
                i = 0 as libc::c_int as UDWORD;
                while (*asScrTypeTab.offset(i as isize)).typeID as libc::c_int
                          != 0 as libc::c_int {
                    if (*asScrTypeTab.offset(i as isize)).typeID as
                           libc::c_uint == type_0 as libc::c_uint {
                        debug(LOG_NEVER,
                              b"%s\x00" as *const u8 as *const libc::c_char,
                              (*asScrTypeTab.offset(i as isize)).pIdent);
                        return
                    }
                    i = i.wrapping_add(1)
                }
            }
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"cpPrintType: Unknown type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"codeprint.c\x00" as *const u8 as *const libc::c_char,
                      61 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"cpPrintType\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    if ref_0 != 0 {
        debug(LOG_NEVER, b" REF\x00" as *const u8 as *const libc::c_char);
    };
}
/* Display a value  */
/* Display a value  */
#[no_mangle]
pub unsafe extern "C" fn cpPrintVal(mut psVal: *mut INTERP_VAL) {
    let mut i: UDWORD = 0;
    if (*psVal).type_0 as libc::c_uint &
           0x100000 as libc::c_int as libc::c_uint != 0 {
        debug(LOG_NEVER, b"type: \x00" as *const u8 as *const libc::c_char);
        cpPrintType((*psVal).type_0);
        debug(LOG_NEVER,
              b" value: %x\x00" as *const u8 as *const libc::c_char,
              (*psVal).v.ival);
        return
    }
    match (*psVal).type_0 as libc::c_uint {
        0 => {
            debug(LOG_NEVER,
                  b"type: BOOL    value: %s\x00" as *const u8 as
                      *const libc::c_char,
                  if (*psVal).v.bval != 0 {
                      b"true\x00" as *const u8 as *const libc::c_char
                  } else {
                      b"false\x00" as *const u8 as *const libc::c_char
                  });
        }
        1 => {
            debug(LOG_NEVER,
                  b"type: INT     value: %d\x00" as *const u8 as
                      *const libc::c_char, (*psVal).v.ival);
        }
        2 => {
            /*	case VAL_FLOAT:
		debug( LOG_NEVER, "type: FLOAT   value: %f", psVal->v.fval );
		break;*/
            debug(LOG_NEVER,
                  b"type: STRING  value: %s\x00" as *const u8 as
                      *const libc::c_char, (*psVal).v.sval);
        }
        3 => {
            debug(LOG_NEVER,
                  b"type: TRIGGER value: %d\x00" as *const u8 as
                      *const libc::c_char, (*psVal).v.ival);
        }
        4 => {
            debug(LOG_NEVER,
                  b"type: EVENT   value: %d\x00" as *const u8 as
                      *const libc::c_char, (*psVal).v.ival);
        }
        _ => {
            // See if it is a user defined type
            if !asScrTypeTab.is_null() {
                i = 0 as libc::c_int as UDWORD;
                while (*asScrTypeTab.offset(i as isize)).typeID as libc::c_int
                          != 0 as libc::c_int {
                    if (*asScrTypeTab.offset(i as isize)).typeID as
                           libc::c_uint == (*psVal).type_0 as libc::c_uint {
                        debug(LOG_NEVER,
                              b"type: %s value: %x\x00" as *const u8 as
                                  *const libc::c_char,
                              (*asScrTypeTab.offset(i as isize)).pIdent,
                              (*psVal).v.ival);
                        return
                    }
                    i = i.wrapping_add(1)
                }
            }
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"cpPrintVal: Unknown value type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"codeprint.c\x00" as *const u8 as *const libc::c_char,
                      118 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"cpPrintVal\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Display a value from a program that has been packed with an opcode */
/* Display a value from a program that has been packed with an opcode */
#[no_mangle]
pub unsafe extern "C" fn cpPrintPackedVal(mut ip: *mut UDWORD) {
    let mut type_0: INTERP_TYPE =
        (*ip & 0xffffff as libc::c_int as libc::c_uint) as INTERP_TYPE;
    let mut i: UDWORD = 0;
    let mut data: UDWORD = *ip.offset(1 as libc::c_int as isize);
    if type_0 as libc::c_uint & 0x100000 as libc::c_int as libc::c_uint != 0 {
        debug(LOG_NEVER, b"type: \x00" as *const u8 as *const libc::c_char);
        cpPrintType(type_0);
        debug(LOG_NEVER,
              b" value: %x\x00" as *const u8 as *const libc::c_char, data);
        return
    }
    match type_0 as libc::c_uint {
        0 => {
            debug(LOG_NEVER,
                  b"BOOL    : %s\x00" as *const u8 as *const libc::c_char,
                  if data as BOOL != 0 {
                      b"true\x00" as *const u8 as *const libc::c_char
                  } else {
                      b"false\x00" as *const u8 as *const libc::c_char
                  });
        }
        1 => {
            debug(LOG_NEVER,
                  b"INT     : %d\x00" as *const u8 as *const libc::c_char,
                  data as SDWORD);
        }
        2 => {
            /*	case VAL_FLOAT:
		debug( LOG_NEVER, "FLOAT   : %f", (float)data );
		break;*/
            debug(LOG_NEVER,
                  b"STRING  : %s\x00" as *const u8 as *const libc::c_char,
                  data as *mut STRING);
        }
        3 => {
            debug(LOG_NEVER,
                  b"TRIGGER : %d\x00" as *const u8 as *const libc::c_char,
                  data as SDWORD);
        }
        4 => {
            debug(LOG_NEVER,
                  b"EVENT   : %d\x00" as *const u8 as *const libc::c_char,
                  data as SDWORD);
        }
        _ => {
            // See if it is a user defined type
            if !asScrTypeTab.is_null() {
                i = 0 as libc::c_int as UDWORD;
                while (*asScrTypeTab.offset(i as isize)).typeID as libc::c_int
                          != 0 as libc::c_int {
                    if (*asScrTypeTab.offset(i as isize)).typeID as
                           libc::c_uint == type_0 as libc::c_uint {
                        debug(LOG_NEVER,
                              b"type: %s value: %x\x00" as *const u8 as
                                  *const libc::c_char,
                              (*asScrTypeTab.offset(i as isize)).pIdent,
                              data);
                        return
                    }
                    i = i.wrapping_add(1)
                }
            }
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"cpPrintVal: Unknown value type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"codeprint.c\x00" as *const u8 as *const libc::c_char,
                      172 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"cpPrintPackedVal\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Display a maths operator */
/* Display a maths operator */
#[no_mangle]
pub unsafe extern "C" fn cpPrintMathsOp(mut opcode: UDWORD) {
    match opcode {
        16 => {
            debug(LOG_NEVER, b"ADD\x00" as *const u8 as *const libc::c_char);
        }
        17 => {
            debug(LOG_NEVER, b"SUB\x00" as *const u8 as *const libc::c_char);
        }
        18 => {
            debug(LOG_NEVER, b"MUL\x00" as *const u8 as *const libc::c_char);
        }
        19 => {
            debug(LOG_NEVER, b"DIV\x00" as *const u8 as *const libc::c_char);
        }
        20 => {
            debug(LOG_NEVER, b"NEG\x00" as *const u8 as *const libc::c_char);
        }
        21 => {
            debug(LOG_NEVER, b"AND\x00" as *const u8 as *const libc::c_char);
        }
        22 => {
            debug(LOG_NEVER, b"OR\x00" as *const u8 as *const libc::c_char);
        }
        23 => {
            debug(LOG_NEVER, b"NOT\x00" as *const u8 as *const libc::c_char);
        }
        25 => {
            debug(LOG_NEVER,
                  b"EQUAL\x00" as *const u8 as *const libc::c_char);
        }
        26 => {
            debug(LOG_NEVER,
                  b"NOT_EQUAL\x00" as *const u8 as *const libc::c_char);
        }
        27 => {
            debug(LOG_NEVER,
                  b"GRT_EQUAL\x00" as *const u8 as *const libc::c_char);
        }
        28 => {
            debug(LOG_NEVER,
                  b"LESS_EQUAL\x00" as *const u8 as *const libc::c_char);
        }
        29 => {
            debug(LOG_NEVER,
                  b"GREATER\x00" as *const u8 as *const libc::c_char);
        }
        30 => {
            debug(LOG_NEVER, b"LESS\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"cpPrintMathsOp: unknown operator\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"codeprint.c\x00" as *const u8 as *const libc::c_char,
                      226 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"cpPrintMathsOp\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Print a function name */
/* Print a function name */
#[no_mangle]
pub unsafe extern "C" fn cpPrintFunc(mut pFunc: SCRIPT_FUNC) {
    let mut i: SDWORD = 0;
    // Search the instinct functions
    if !asScrInstinctTab.is_null() {
        i = 0 as libc::c_int;
        while (*asScrInstinctTab.offset(i as isize)).pFunc.is_some() {
            if (*asScrInstinctTab.offset(i as isize)).pFunc == pFunc {
                debug(LOG_NEVER,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      (*asScrInstinctTab.offset(i as isize)).pIdent);
                return
            }
            i += 1
        }
    }
    // Search the callback functions
    if !asScrCallbackTab.is_null() {
        i = 0 as libc::c_int;
        while (*asScrCallbackTab.offset(i as isize)).type_0 as libc::c_uint !=
                  0 as libc::c_int as libc::c_uint {
            if (*asScrCallbackTab.offset(i as isize)).pFunc == pFunc {
                debug(LOG_NEVER,
                      b"%s\x00" as *const u8 as *const libc::c_char,
                      (*asScrCallbackTab.offset(i as isize)).pIdent);
                return
            }
            i += 1
        }
    };
}
/* Print a variable access function name */
/* Print a variable access function name */
#[no_mangle]
pub unsafe extern "C" fn cpPrintVarFunc(mut pFunc: SCRIPT_VARFUNC,
                                        mut index: UDWORD) {
    let mut i: SDWORD = 0;
    // Search the external variable functions
    if !asScrExternalTab.is_null() {
        i = 0 as libc::c_int;
        while !(*asScrExternalTab.offset(i as isize)).pIdent.is_null() {
            if (*asScrExternalTab.offset(i as isize)).set == pFunc &&
                   (*asScrExternalTab.offset(i as isize)).index == index {
                debug(LOG_NEVER,
                      b"%s (set)\x00" as *const u8 as *const libc::c_char,
                      (*asScrExternalTab.offset(i as isize)).pIdent);
                return
            } else {
                if (*asScrExternalTab.offset(i as isize)).get == pFunc &&
                       (*asScrExternalTab.offset(i as isize)).index == index {
                    debug(LOG_NEVER,
                          b"%s (get)\x00" as *const u8 as *const libc::c_char,
                          (*asScrExternalTab.offset(i as isize)).pIdent);
                    return
                }
            }
            i += 1
        }
    }
    // Search the object functions
    if !asScrObjectVarTab.is_null() {
        i = 0 as libc::c_int;
        while !(*asScrObjectVarTab.offset(i as isize)).pIdent.is_null() {
            if (*asScrObjectVarTab.offset(i as isize)).set == pFunc &&
                   (*asScrObjectVarTab.offset(i as isize)).index == index {
                debug(LOG_NEVER,
                      b"%s (set)\x00" as *const u8 as *const libc::c_char,
                      (*asScrObjectVarTab.offset(i as isize)).pIdent);
                return
            } else {
                if (*asScrObjectVarTab.offset(i as isize)).get == pFunc &&
                       (*asScrObjectVarTab.offset(i as isize)).index == index
                   {
                    debug(LOG_NEVER,
                          b"%s (get)\x00" as *const u8 as *const libc::c_char,
                          (*asScrObjectVarTab.offset(i as isize)).pIdent);
                    return
                }
            }
            i += 1
        }
    };
}
/* Print the array information */
#[no_mangle]
pub unsafe extern "C" fn cpPrintArrayInfo(mut pip: *mut *mut UDWORD,
                                          mut psProg: *mut SCRIPT_CODE) {
    let mut i: SDWORD = 0; //, elements[VAR_MAX_DIMENSIONS];
    let mut dimensions: SDWORD = 0;
    //	SDWORD		elementDWords;
//	UBYTE		*pElem;
    let mut ip: *mut UDWORD = *pip;
    let mut base: UDWORD = 0;
    // get the base index of the array
    base = *ip & 0xfffff as libc::c_int as libc::c_uint;
    // get the number of dimensions
    dimensions =
        ((*ip & 0xf00000 as libc::c_int as libc::c_uint) >> 20 as libc::c_int)
            as SDWORD;
    // get the number of elements for each dimension
/*	pElem = (UBYTE *) (ip + 1);
	for(i=0; i<dimensions; i+=1)
	{
		elements[i] = *pElem;

		pElem += 1;
	}*/
    debug(LOG_NEVER, b"%d->\x00" as *const u8 as *const libc::c_char, base);
    i = 0 as libc::c_int;
    while i <
              (*(*psProg).psArrayInfo.offset(base as isize)).dimensions as
                  libc::c_int {
        debug(LOG_NEVER, b"[%d]\x00" as *const u8 as *const libc::c_char,
              (*(*psProg).psArrayInfo.offset(base as
                                                 isize)).elements[i as usize]
                  as libc::c_int);
        i += 1 as libc::c_int
    }
    // calculate the number of DWORDs needed to store the number of elements for each dimension of the array
//	elementDWords = (dimensions - 1)/4 + 1;
    // update the insrtuction pointer
    *pip = (*pip).offset(1 as libc::c_int as isize);
    // + elementDWords;
}
/* Display the contents of a program in readable form */
/* Display the contents of a program in readable form */
#[no_mangle]
pub unsafe extern "C" fn cpPrintProgram(mut psProg: *mut SCRIPT_CODE) {
    let mut ip: *mut UDWORD = 0 as *mut UDWORD;
    let mut end: *mut UDWORD = 0 as *mut UDWORD;
    let mut opcode: OPCODE = OP_PUSH;
    let mut data: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut dim: UDWORD = 0;
    let mut psCurrDebug: *mut SCRIPT_DEBUG = 0 as *mut SCRIPT_DEBUG;
    let mut debugInfo: BOOL = 0;
    let mut triggerCode: BOOL = 0;
    let mut jumpOffset: UDWORD = 0;
    let mut psCurrVar: *mut VAR_DEBUG = 0 as *mut VAR_DEBUG;
    let mut psCurrArray: *mut ARRAY_DATA = 0 as *mut ARRAY_DATA;
    let mut psCurrArrayDebug: *mut ARRAY_DEBUG = 0 as *mut ARRAY_DEBUG;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"cpPrintProgram: Invalid program pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"codeprint.c\x00" as *const u8 as *const libc::c_char,
              362 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"cpPrintProgram\x00")).as_ptr(),
              b"PTRVALID(psProg, sizeof(SCRIPT_CODE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    debugInfo =
        ((*psProg).psDebug != 0 as *mut libc::c_void as *mut SCRIPT_DEBUG) as
            libc::c_int;
    if debugInfo != 0 {
        // Print out the global variables
        if (*psProg).numGlobals as libc::c_int > 0 as libc::c_int {
            debug(LOG_NEVER,
                  b"index  storage  type variable name\n\x00" as *const u8 as
                      *const libc::c_char);
            psCurrVar = (*psProg).psVarDebug;
            i = 0 as libc::c_int as UDWORD;
            while i < (*psProg).numGlobals as libc::c_uint {
                debug(LOG_NEVER,
                      b"%-6d %s  %-4d %s\n\x00" as *const u8 as
                          *const libc::c_char,
                      psCurrVar.wrapping_offset_from((*psProg).psVarDebug) as
                          libc::c_int,
                      if (*psCurrVar).storage as libc::c_int ==
                             ST_PUBLIC as libc::c_int {
                          b"Public \x00" as *const u8 as *const libc::c_char
                      } else {
                          b"Private\x00" as *const u8 as *const libc::c_char
                      },
                      *(*psProg).pGlobals.offset(i as isize) as libc::c_uint,
                      (*psCurrVar).pIdent);
                psCurrVar = psCurrVar.offset(1);
                i = i.wrapping_add(1)
            }
        }
        if (*psProg).numArrays as libc::c_int > 0 as libc::c_int {
            debug(LOG_NEVER,
                  b"\narrays\n\x00" as *const u8 as *const libc::c_char);
            psCurrArray = (*psProg).psArrayInfo;
            psCurrArrayDebug = (*psProg).psArrayDebug;
            i = 0 as libc::c_int as UDWORD;
            while i < (*psProg).numArrays as libc::c_uint {
                debug(LOG_NEVER,
                      b"%-6d %s  %-4d %s\x00" as *const u8 as
                          *const libc::c_char, i,
                      if (*psCurrArrayDebug).storage as libc::c_int ==
                             ST_PUBLIC as libc::c_int {
                          b"Public \x00" as *const u8 as *const libc::c_char
                      } else {
                          b"Private\x00" as *const u8 as *const libc::c_char
                      }, (*psCurrArray).type_0 as libc::c_int,
                      (*psCurrArrayDebug).pIdent);
                dim = 0 as libc::c_int as UDWORD;
                while dim < (*psCurrArray).dimensions as libc::c_uint {
                    debug(LOG_NEVER,
                          b"[%d]\x00" as *const u8 as *const libc::c_char,
                          (*psCurrArray).elements[dim as usize] as
                              libc::c_int);
                    dim =
                        (dim as
                             libc::c_uint).wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                            UDWORD as UDWORD
                }
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
                psCurrArray = psCurrArray.offset(1);
                psCurrArrayDebug = psCurrArrayDebug.offset(1);
                i = i.wrapping_add(1)
            }
        }
        debug(LOG_NEVER,
              b"\nindex line offset\n\x00" as *const u8 as
                  *const libc::c_char);
        psCurrDebug = (*psProg).psDebug
    } else {
        debug(LOG_NEVER,
              b"index offset\n\x00" as *const u8 as *const libc::c_char);
    }
    // Find the first trigger with code
    jumpOffset = 0 as libc::c_int as UDWORD;
    while jumpOffset < (*psProg).numTriggers as libc::c_uint {
        if (*(*psProg).psTriggerData.offset(jumpOffset as isize)).type_0 as
               libc::c_int == TR_CODE as libc::c_int {
            break ;
        }
        jumpOffset = jumpOffset.wrapping_add(1)
    }
    ip = (*psProg).pCode;
    triggerCode =
        if (*psProg).numTriggers as libc::c_int > 0 as libc::c_int {
            1 as libc::c_int
        } else { 0 as libc::c_int };
    end = (ip as *mut UBYTE).offset((*psProg).size as isize) as *mut UDWORD;
    opcode = (*ip >> 24 as libc::c_int) as OPCODE;
    data = *ip & 0xffffff as libc::c_int as libc::c_uint;
    while ip < end {
        // display a label if there is one
        if debugInfo != 0 {
            if ip.wrapping_offset_from((*psProg).pCode) as libc::c_int as
                   UDWORD == (*psCurrDebug).offset &&
                   !(*psCurrDebug).pLabel.is_null() {
                debug(LOG_NEVER,
                      b"%s\n\x00" as *const u8 as *const libc::c_char,
                      (*psCurrDebug).pLabel);
            }
        }
        // Display the trigger/event index
        if triggerCode != 0 {
            if ip.wrapping_offset_from((*psProg).pCode) as libc::c_int ==
                   *(*psProg).pTriggerTab.offset(jumpOffset as isize) as
                       libc::c_int {
                debug(LOG_NEVER,
                      b"%-6d\x00" as *const u8 as *const libc::c_char,
                      jumpOffset);
                jumpOffset =
                    (jumpOffset as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD;
                // Find the next trigger with code
                while jumpOffset < (*psProg).numTriggers as libc::c_uint {
                    if (*(*psProg).psTriggerData.offset(jumpOffset as
                                                            isize)).type_0 as
                           libc::c_int == TR_CODE as libc::c_int {
                        break ;
                    }
                    jumpOffset = jumpOffset.wrapping_add(1)
                }
                if jumpOffset >= (*psProg).numTriggers as libc::c_uint {
                    // Got to the end of the triggers
                    triggerCode = 0 as libc::c_int;
                    jumpOffset = 0 as libc::c_int as UDWORD
                }
            } else {
                debug(LOG_NEVER, b"\x00" as *const u8 as *const libc::c_char);
            }
        } else if ip.wrapping_offset_from((*psProg).pCode) as libc::c_int ==
                      *(*psProg).pEventTab.offset(jumpOffset as isize) as
                          libc::c_int {
            debug(LOG_NEVER, b"%-6d\x00" as *const u8 as *const libc::c_char,
                  jumpOffset);
            jumpOffset =
                (jumpOffset as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        } else {
            debug(LOG_NEVER, b"\x00" as *const u8 as *const libc::c_char);
        }
        // Display the line number
        if debugInfo != 0 {
            if ip.wrapping_offset_from((*psProg).pCode) as libc::c_int as
                   UDWORD == (*psCurrDebug).offset {
                debug(LOG_NEVER,
                      b"%-6d\x00" as *const u8 as *const libc::c_char,
                      (*psCurrDebug).line);
                psCurrDebug = psCurrDebug.offset(1)
            } else {
                debug(LOG_NEVER, b"\x00" as *const u8 as *const libc::c_char);
            }
        }
        // Display the code offset
        debug(LOG_NEVER, b"%-6d\x00" as *const u8 as *const libc::c_char,
              ip.wrapping_offset_from((*psProg).pCode) as libc::c_int);
        match opcode as libc::c_uint {
            0 => {
                debug(LOG_NEVER,
                      b"PUSH\x00" as *const u8 as *const libc::c_char);
                cpPrintPackedVal(ip);
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            1 => {
                debug(LOG_NEVER,
                      b"PUSHREF\x00" as *const u8 as *const libc::c_char);
                cpPrintPackedVal(ip);
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            2 => {
                debug(LOG_NEVER,
                      b"POP\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            3 => {
                debug(LOG_NEVER,
                      b"PUSHGLOBAL %d\n\x00" as *const u8 as
                          *const libc::c_char, data);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            4 => {
                debug(LOG_NEVER,
                      b"POPGLOBAL  %d\n\x00" as *const u8 as
                          *const libc::c_char, data);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            5 => {
                debug(LOG_NEVER,
                      b"PUSHARRAYGLOBAL\x00" as *const u8 as
                          *const libc::c_char);
                cpPrintArrayInfo(&mut ip, psProg);
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
            }
            6 => {
                debug(LOG_NEVER,
                      b"POPARRAYGLOBAL\x00" as *const u8 as
                          *const libc::c_char);
                cpPrintArrayInfo(&mut ip, psProg);
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
            }
            7 => {
                debug(LOG_NEVER,
                      b"CALL\x00" as *const u8 as *const libc::c_char);
                cpPrintFunc(::std::mem::transmute::<libc::intptr_t,
                                                    SCRIPT_FUNC>(*ip.offset(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)
                                                                     as
                                                                     libc::intptr_t));
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            8 => {
                debug(LOG_NEVER,
                      b"VARCALL\x00" as *const u8 as *const libc::c_char);
                cpPrintVarFunc(::std::mem::transmute::<libc::intptr_t,
                                                       SCRIPT_VARFUNC>(*ip.offset(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)
                                                                           as
                                                                           libc::intptr_t),
                               data);
                debug(LOG_NEVER,
                      b"(%d)\n\x00" as *const u8 as *const libc::c_char,
                      data);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            9 => {
                debug(LOG_NEVER,
                      b"JUMP       %d (%d)\n\x00" as *const u8 as
                          *const libc::c_char, data as SWORD as libc::c_int,
                      ip.wrapping_offset_from((*psProg).pCode) as libc::c_int
                          + data as SWORD as libc::c_int);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            10 => {
                debug(LOG_NEVER,
                      b"JUMPTRUE   %d (%d)\n\x00" as *const u8 as
                          *const libc::c_char, data as SWORD as libc::c_int,
                      ip.wrapping_offset_from((*psProg).pCode) as libc::c_int
                          + data as SWORD as libc::c_int);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            11 => {
                debug(LOG_NEVER,
                      b"JUMPFALSE  %d (%d)\n\x00" as *const u8 as
                          *const libc::c_char, data as SWORD as libc::c_int,
                      ip.wrapping_offset_from((*psProg).pCode) as libc::c_int
                          + data as SWORD as libc::c_int);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            12 | 13 => {
                cpPrintMathsOp(data);
                debug(LOG_NEVER,
                      b"\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            14 => {
                debug(LOG_NEVER,
                      b"EXIT\n\x00" as *const u8 as *const libc::c_char);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            15 => {
                debug(LOG_NEVER,
                      b"PAUSE %d\n\x00" as *const u8 as *const libc::c_char,
                      data);
                ip =
                    ip.offset(*aOpSize.as_mut_ptr().offset(opcode as isize) as
                                  isize)
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"cpPrintProgram: Unknown opcode: %x\x00" as
                              *const u8 as *const libc::c_char, *ip);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"codeprint.c\x00" as *const u8 as
                              *const libc::c_char, 564 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"cpPrintProgram\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        if ip <= end || 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"cpPrintProgram: instruction pointer no longer valid\x00"
                      as *const u8 as *const libc::c_char);
        };
        if ip <= end || 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"codeprint.c\x00" as *const u8 as *const libc::c_char,
                  569 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"cpPrintProgram\x00")).as_ptr(),
                  b"(ip <= end) || PTRVALID(ip, sizeof(UDWORD))\x00" as
                      *const u8 as *const libc::c_char);
        };
        opcode = (*ip >> 24 as libc::c_int) as OPCODE;
        data = *ip & 0xffffff as libc::c_int as libc::c_uint
    };
}
