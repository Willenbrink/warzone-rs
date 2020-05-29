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
    /* Push a value onto the stack */
    #[no_mangle]
    fn stackPush(psVal: *mut INTERP_VAL) -> BOOL;
    /* Pop a value off the stack */
    #[no_mangle]
    fn stackPop(psVal: *mut INTERP_VAL) -> BOOL;
    /* Pop a value off the stack, checking that the type matches what is passed in */
    #[no_mangle]
    fn stackPopType(psVal: *mut INTERP_VAL) -> BOOL;
    /* Print the top value on the stack */
    #[no_mangle]
    fn stackPrintTop();
    /* Check if the stack is empty */
    #[no_mangle]
    fn stackEmpty() -> BOOL;
    /* Do binary operations on the top of the stack
 * This effectively pops two values and pushes the result
 */
    #[no_mangle]
    fn stackBinaryOp(opcode: OPCODE) -> BOOL;
    /* Perform a unary operation on the top of the stack
 * This effectively pops a value and pushes the result
 */
    #[no_mangle]
    fn stackUnaryOp(opcode: OPCODE) -> BOOL;
    /* Reset the stack to an empty state */
    #[no_mangle]
    fn stackReset();
    /* Display a value  */
    #[no_mangle]
    fn cpPrintVal(psVal: *mut INTERP_VAL);
    /* Print a function name */
    #[no_mangle]
    fn cpPrintFunc(pFunc: SCRIPT_FUNC);
    /* Print a variable access function name */
    #[no_mangle]
    fn cpPrintVarFunc(pFunc: SCRIPT_VARFUNC, index: UDWORD);
    /* Display a maths operator */
    #[no_mangle]
    fn cpPrintMathsOp(opcode: UDWORD);
    // add a TR_PAUSE trigger to the event system.
    #[no_mangle]
    fn eventAddPauseTrigger(psContext: *mut SCRIPT_CONTEXT, event: UDWORD,
                            offset: UDWORD, time: UDWORD) -> BOOL;
    #[no_mangle]
    fn eventGetEventID(psCode: *mut SCRIPT_CODE, event: SDWORD)
     -> *mut STRING;
    #[no_mangle]
    fn eventGetTriggerID(psCode: *mut SCRIPT_CODE, trigger: SDWORD)
     -> *mut STRING;
    #[no_mangle]
    fn resetLocalVars(psCode: *mut SCRIPT_CODE, EventIndex: UDWORD) -> BOOL;
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_typeequiv {
    pub base: INTERP_TYPE,
    pub numEquiv: SDWORD,
    pub aEquivTypes: [INTERP_TYPE; 10],
}
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
pub type TYPE_EQUIV = _interp_typeequiv;
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
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
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
pub type _interp_runtype = libc::c_uint;
pub const IRT_EVENT: _interp_runtype = 1;
pub const IRT_TRIGGER: _interp_runtype = 0;
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
/* What type of code should be run by the interpreter */
pub type INTERP_RUNTYPE = _interp_runtype;
// Run trigger code
// Run event code
/*
 * Event.h
 *
 * Interface to the event management system.
 *
 */
/* The number of values in a context value chunk */
/* One chunk of variables for a script context */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _val_chunk {
    pub asVals: [INTERP_VAL; 20],
    pub psNext: *mut _val_chunk,
}
pub type VAL_CHUNK = _val_chunk;
/* The data needed within an object to run a script */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_context {
    pub psCode: *mut SCRIPT_CODE,
    pub psGlobals: *mut VAL_CHUNK,
    pub triggerCount: SDWORD,
    pub release: SWORD,
    pub id: SWORD,
    pub psNext: *mut _script_context,
}
pub type SCRIPT_CONTEXT = _script_context;
/* The size of each opcode */
#[no_mangle]
pub static mut aOpSize: [SDWORD; 16] =
    [2 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 2 as libc::c_int,
     2 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int];
/* The type equivalence table */
static mut asInterpTypeEquiv: *mut TYPE_EQUIV =
    0 as *const TYPE_EQUIV as *mut TYPE_EQUIV;
// whether the interpreter is running
static mut bInterpRunning: BOOL = 0 as libc::c_int;
/* Whether to output trace information */
#[no_mangle]
pub static mut interpTrace: BOOL = 0;
// TRUE if the interpreter is currently running
/* Print out trace info if tracing is turned on */
// TRUE if the interpreter is currently running
#[no_mangle]
pub unsafe extern "C" fn interpProcessorActive() -> BOOL {
    return bInterpRunning;
}
/* Find the value store for a global variable */
#[inline]
unsafe extern "C" fn interpGetVarData(mut psGlobals: *mut VAL_CHUNK,
                                      mut index: UDWORD) -> *mut INTERP_VAL {
    let mut psChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    psChunk = psGlobals;
    while index >= 20 as libc::c_int as libc::c_uint {
        index =
            (index as
                 libc::c_uint).wrapping_sub(20 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        psChunk = (*psChunk).psNext
    }
    return (*psChunk).asVals.as_mut_ptr().offset(index as isize);
}
// get the array data for an array operation
#[no_mangle]
pub unsafe extern "C" fn interpGetArrayVarData(mut pip: *mut *mut UDWORD,
                                               mut psGlobals: *mut VAL_CHUNK,
                                               mut psProg: *mut SCRIPT_CODE,
                                               mut ppsVal:
                                                   *mut *mut INTERP_VAL)
 -> BOOL {
    let mut i: SDWORD = 0; //[VAR_MAX_DIMENSIONS]
    let mut dimensions: SDWORD = 0; //, elementDWords;
    let mut vals: [SDWORD; 4] = [0; 4];
    let mut elements: *mut UBYTE = 0 as *mut UBYTE;
    let mut size: SDWORD = 0;
    let mut val: SDWORD = 0;
    //	UBYTE		*pElem;
    let mut ip: *mut UDWORD = *pip;
    let mut base: UDWORD = 0;
    let mut index: UDWORD = 0;
    // get the base index of the array
    base = *ip & 0xfffff as libc::c_int as libc::c_uint;
    // get the number of dimensions
    dimensions =
        ((*ip & 0xf00000 as libc::c_int as libc::c_uint) >> 20 as libc::c_int)
            as SDWORD;
    if base >= (*psProg).numArrays as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"interpGetArrayVarData: array base index out of range\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"interp.c\x00" as *const u8 as *const libc::c_char,
                  128 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"interpGetArrayVarData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if dimensions !=
           (*(*psProg).psArrayInfo.offset(base as isize)).dimensions as
               libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"interpGetArrayVarData: dimensions do not match\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"interp.c\x00" as *const u8 as *const libc::c_char,
                  134 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"interpGetArrayVarData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // get the number of elements for each dimension
    elements =
        (*(*psProg).psArrayInfo.offset(base as isize)).elements.as_mut_ptr();
    /*	pElem = (UBYTE *) (ip + 1);
	for(i=0; i<dimensions; i+=1)
	{
		elements[i] = *pElem;

		pElem += 1;
	}*/
    // calculate the index of the array element
    size = 1 as libc::c_int;
    index = 0 as libc::c_int as UDWORD;
    i = dimensions - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                          &mut val as *mut SDWORD) == 0 {
            return 0 as libc::c_int
        }
        if val < 0 as libc::c_int ||
               val >= *elements.offset(i as isize) as libc::c_int {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"interpGetArrayVarData: Array index for dimension %d out of range\x00"
                          as *const u8 as *const libc::c_char, i);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"interp.c\x00" as *const u8 as *const libc::c_char,
                      160 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"interpGetArrayVarData\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        index =
            (index as libc::c_uint).wrapping_add((val * size) as libc::c_uint)
                as UDWORD as UDWORD;
        size *=
            (*(*psProg).psArrayInfo.offset(base as
                                               isize)).elements[i as usize] as
                libc::c_int;
        vals[i as usize] = val;
        i -= 1 as libc::c_int
    }
    // print out the debug trace
    if interpTrace != 0 {
        debug(LOG_NEVER, b"%d->\x00" as *const u8 as *const libc::c_char,
              base);
        i = 0 as libc::c_int;
        while i < dimensions {
            debug(LOG_NEVER,
                  b"[%d/%d]\x00" as *const u8 as *const libc::c_char,
                  vals[i as usize],
                  *elements.offset(i as isize) as libc::c_int);
            i += 1 as libc::c_int
        }
        debug(LOG_NEVER, b"(%d) \x00" as *const u8 as *const libc::c_char,
              index);
    }
    // check the index is valid
    if index > (*psProg).arraySize {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"interpGetArrayVarData: Array indexes out of variable space\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"interp.c\x00" as *const u8 as *const libc::c_char,
                  183 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"interpGetArrayVarData\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // get the variable data
    *ppsVal =
        interpGetVarData(psGlobals,
                         (*(*psProg).psArrayInfo.offset(base as
                                                            isize)).base.wrapping_add(index));
    // calculate the number of DWORDs needed to store the number of elements for each dimension of the array
//	elementDWords = (dimensions - 1)/4 + 1;
    // update the insrtuction pointer
    *pip = (*pip).offset(1 as libc::c_int as isize); // + elementDWords;
    return 1 as libc::c_int;
}
// Initialise the interpreter
// Initialise the interpreter
#[no_mangle]
pub unsafe extern "C" fn interpInitialise() -> BOOL {
    asInterpTypeEquiv = 0 as *mut TYPE_EQUIV;
    return 1 as libc::c_int;
}
/* Run a compiled script */
#[no_mangle]
pub unsafe extern "C" fn interpRunScript(mut psContext: *mut SCRIPT_CONTEXT,
                                         mut runType: INTERP_RUNTYPE,
                                         mut index: UDWORD,
                                         mut offset: UDWORD) -> BOOL {
    let mut current_block: u64;
    let mut ip: *mut UDWORD = 0 as *mut UDWORD;
    let mut opcode: UDWORD = 0;
    let mut data: UDWORD = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    let mut psVar: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    let mut psGlobals: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut numGlobals: UDWORD = 0;
    let mut pCodeStart: *mut UDWORD = 0 as *mut UDWORD;
    let mut pCodeEnd: *mut UDWORD = 0 as *mut UDWORD;
    let mut pCodeBase: *mut UDWORD = 0 as *mut UDWORD;
    let mut scriptFunc: SCRIPT_FUNC = None;
    let mut scriptVarFunc: SCRIPT_VARFUNC = None;
    let mut psProg: *mut SCRIPT_CODE = 0 as *mut SCRIPT_CODE;
    //	SDWORD			arrayIndex, dimensions, arrayElements[VAR_MAX_DIMENSIONS];
    let mut instructionCount: SDWORD = 0 as libc::c_int;
    let mut CurEvent: UDWORD = 0;
    let mut bStop: BOOL = 0;
    let mut bEvent: BOOL = 0;
    let mut callDepth: SDWORD = 0;
    let mut pTrigLab: *mut STRING = 0 as *mut STRING;
    let mut pEventLab: *mut STRING = 0 as *mut STRING;
    //debug(LOG_SCRIPT, "interpRunScript 1");
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"interpRunScript: invalid context pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"interp.c\x00" as *const u8 as *const libc::c_char,
              230 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
              b"PTRVALID(psContext, sizeof(SCRIPT_CONTEXT))\x00" as *const u8
                  as *const libc::c_char);
    };
    psProg = (*psContext).psCode;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"interpRunScript: invalid script code pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"interp.c\x00" as *const u8 as *const libc::c_char,
              233 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
              b"PTRVALID(psProg, sizeof(SCRIPT_CODE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bInterpRunning != 0 {
        debug(LOG_ERROR,
              b"interpRunScript: interpreter already running                 - callback being called from within a script function?\x00"
                  as *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"interpRunScript: interpreter already running                 - callback being called from within a script function?\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"interp.c\x00" as *const u8 as *const libc::c_char,
                  242 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    } else {
        // note that the interpreter is running to stop recursive script calls
        bInterpRunning = 1 as libc::c_int;
        // Reset the stack in case another script messed up
        stackReset();
        //reset return stack
        retStackReset();
        // Turn off tracing initially
        interpTrace = 0 as libc::c_int;
        /* Get the global variables */
        numGlobals = (*psProg).numGlobals as UDWORD;
        psGlobals = (*psContext).psGlobals;
        bEvent = 0 as libc::c_int;
        // Find the code range
        match runType as libc::c_uint {
            0 => {
                if index > (*psProg).numTriggers as libc::c_uint {
                    debug(LOG_ERROR,
                          b"interpRunScript: trigger index out of range\x00"
                              as *const u8 as
                              *const libc::c_char); //offset only used for pause() script function
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"interpRunScript: trigger index out of range\x00"
                                  as *const u8 as
                                  *const libc::c_char); //remember it's an event
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"interp.c\x00" as *const u8 as
                                  *const libc::c_char, 271 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                pCodeBase =
                    (*psProg).pCode.offset(*(*psProg).pTriggerTab.offset(index
                                                                             as
                                                                             isize)
                                               as libc::c_int as isize);
                pCodeStart = pCodeBase;
                pCodeEnd =
                    (*psProg).pCode.offset(*(*psProg).pTriggerTab.offset(index.wrapping_add(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                                             as
                                                                             isize)
                                               as libc::c_int as isize)
            }
            1 => {
                if index > (*psProg).numEvents as libc::c_uint {
                    debug(LOG_ERROR,
                          b"interpRunScript: trigger index out of range\x00"
                              as *const u8 as *const libc::c_char);
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"interpRunScript: trigger index out of range\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"interp.c\x00" as *const u8 as
                                  *const libc::c_char, 282 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                pCodeBase =
                    (*psProg).pCode.offset(*(*psProg).pEventTab.offset(index
                                                                           as
                                                                           isize)
                                               as libc::c_int as isize);
                pCodeStart = pCodeBase.offset(offset as isize);
                pCodeEnd =
                    (*psProg).pCode.offset(*(*psProg).pEventTab.offset(index.wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)
                                                                           as
                                                                           isize)
                                               as libc::c_int as isize);
                bEvent = 1 as libc::c_int
            }
            _ => {
                debug(LOG_ERROR,
                      b"interpRunScript: unknown run type\x00" as *const u8 as
                          *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"interpRunScript: unknown run type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"interp.c\x00" as *const u8 as *const libc::c_char,
                          293 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        // Get the first opcode
        ip = pCodeStart;
        opcode = *ip >> 24 as libc::c_int;
        instructionCount = 0 as libc::c_int;
        CurEvent = index;
        bStop = 0 as libc::c_int;
        //debug(LOG_SCRIPT, "interpRunScript 2");
        loop 
             //debug(LOG_SCRIPT, "OP_CALL 3");
             {
            if !(bStop == 0) { current_block = 16992166533277814053; break ; }
            // Run the code
            if ip < pCodeEnd {
                // && opcode != OP_EXIT)
                if instructionCount > 100000 as libc::c_int {
                    debug(LOG_ERROR,
                          b"interpRunScript: max instruction count exceeded - infinite loop ?\x00"
                              as *const u8 as *const libc::c_char);
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"interpRunScript: max instruction count exceeded - infinite loop ?\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"interp.c\x00" as *const u8 as
                                  *const libc::c_char, 318 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    current_block = 17057041599611039122;
                    break ;
                } else {
                    instructionCount += 1 as libc::c_int;
                    if interpTrace != 0 {
                        debug(LOG_NEVER,
                              ip.wrapping_offset_from((*psProg).pCode) as
                                  libc::c_int as *const libc::c_char);
                    }
                    opcode = *ip >> 24 as libc::c_int;
                    data = *ip & 0xffffff as libc::c_int as libc::c_uint;
                    match opcode {
                        31 => {
                            /* Custom function call */
                            //debug( LOG_SCRIPT, "-OP_FUNC" );
					//debug( LOG_SCRIPT, "OP_FUNC: remember event %d, ip=%d", CurEvent, (ip + 2) );
                            if RetStackRemember(CurEvent,
                                                ip.offset(2 as libc::c_int as
                                                              isize) as
                                                    UDWORD) == 0 {
                                //Remember where to jump back later
                                debug(LOG_ERROR,
                                      b"interpRunScript() - RetStackRemember() failed.\x00"
                                          as *const u8 as
                                          *const libc::c_char); //Current event = event to jump to
                                return 0 as libc::c_int
                            }
                            CurEvent = *ip.offset(1 as libc::c_int as isize);
                            if CurEvent > (*psProg).numEvents as libc::c_uint
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: trigger index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                return 0 as libc::c_int
                            }
                            //Set new code execution boundries
					//----------------------------------
                            pCodeBase =
                                (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent
                                                                                       as
                                                                                       isize)
                                                           as libc::c_int as
                                                           isize); //Start at the beginning of the new event
                            pCodeStart = pCodeBase;
                            pCodeEnd =
                                (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                       as
                                                                                       isize)
                                                           as libc::c_int as
                                                           isize);
                            ip = pCodeStart
                        }
                        33 => {
                            //debug( LOG_SCRIPT, "OP_PUSHLOCAL");
				//debug( LOG_SCRIPT, "OP_PUSHLOCAL, (CurEvent=%d, data =%d) num loc vars: %d; pushing: %d", CurEvent, data, psContext->psCode->numLocalVars[CurEvent], psContext->psCode->ppsLocalVarVal[CurEvent][data].v.ival);
                            if data >=
                                   *(*(*psContext).psCode).numLocalVars.offset(CurEvent
                                                                                   as
                                                                                   isize)
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_PUSHLOCAL: variable index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: OP_PUSHLOCAL: variable index out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          371 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else if stackPush(&mut *(*(*(*psContext).psCode).ppsLocalVarVal.offset(CurEvent
                                                                                                         as
                                                                                                         isize)).offset(data
                                                                                                                            as
                                                                                                                            isize))
                                          == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_PUSHLOCAL: push failed\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                //debug( LOG_SCRIPT, "OP_PUSHLOCAL 2");
				//debug(LOG_SCRIPT, "OP_PUSHLOCAL type: %d", psContext->psCode->ppsLocalVarVal[CurEvent][data].type);
                                //debug( LOG_SCRIPT, "OP_PUSHLOCAL 3");
                                ip = ip.offset(1 as libc::c_int as isize)
                            }
                        }
                        32 => { //aOpSize[opcode];
                            //debug( LOG_SCRIPT, "OP_POPLOCAL, event index: '%d', data: '%d'", CurEvent, data);
				//debug( LOG_SCRIPT, "OP_POPLOCAL, numLocalVars: '%d'", psContext->psCode->numLocalVars[CurEvent]);
                            if data >=
                                   *(*(*psContext).psCode).numLocalVars.offset(CurEvent
                                                                                   as
                                                                                   isize)
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_POPLOCAL: variable index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: variable index out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          396 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else if stackPopType(&mut *(*(*(*psContext).psCode).ppsLocalVarVal.offset(CurEvent
                                                                                                            as
                                                                                                            isize)).offset(data
                                                                                                                               as
                                                                                                                               isize))
                                          == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_POPLOCAL: pop failed\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                //debug( LOG_SCRIPT, "OP_POPLOCAL 2");
				//DbgMsg("OP_POPLOCAL type: %d, CurEvent=%d, data=%d", psContext->psCode->ppsLocalVarVal[CurEvent][data].type, CurEvent, data);
                                //debug(LOG_SCRIPT, "OP_POPLOCAL: type=%d, val=%d", psContext->psCode->ppsLocalVarVal[CurEvent][data].type, psContext->psCode->ppsLocalVarVal[CurEvent][data].v.ival);
                                //debug( LOG_SCRIPT, "OP_POPLOCAL 3");
                                ip = ip.offset(1 as libc::c_int as isize)
                            }
                        }
                        34 => { //aOpSize[opcode];
                            // The type of the variable is stored in with the opcode
                            sVal.type_0 =
                                (*ip &
                                     0xffffff as libc::c_int as libc::c_uint)
                                    as INTERP_TYPE;
                            /* get local var index */
                            data = *ip.offset(1 as libc::c_int as isize);
                            if data >=
                                   *(*(*psContext).psCode).numLocalVars.offset(CurEvent
                                                                                   as
                                                                                   isize)
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_PUSHLOCALREF: variable index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: OP_PUSHLOCALREF: variable index out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          426 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                /* get local variable */
                                psVar =
                                    &mut *(*(*(*psContext).psCode).ppsLocalVarVal.offset(CurEvent
                                                                                             as
                                                                                             isize)).offset(data
                                                                                                                as
                                                                                                                isize)
                                        as *mut INTERP_VAL;
                                sVal.v.oval =
                                    &mut (*psVar).v.ival as *mut SDWORD as
                                        *mut libc::c_void;
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"PUSHREF     \x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                if interpTrace != 0 { cpPrintVal(&mut sVal); }
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                if stackPush(&mut sVal) == 0 {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: OP_PUSHLOCALREF: push failed\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                    current_block = 17057041599611039122;
                                    break ;
                                } else {
                                    ip = ip.offset(2 as libc::c_int as isize)
                                }
                            }
                        }
                        0 => {
                            // The type of the value is stored in with the opcode
                            sVal.type_0 =
                                (*ip &
                                     0xffffff as libc::c_int as libc::c_uint)
                                    as INTERP_TYPE;
                            // Copy the data as a DWORD
                            sVal.v.ival =
                                *ip.offset(1 as libc::c_int as isize) as
                                    SDWORD;
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"PUSH        \x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if interpTrace != 0 { cpPrintVal(&mut sVal); }
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"\n\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if stackPush(&mut sVal) == 0 {
                                // Eeerk, out of memory
                                debug(LOG_ERROR,
                                      b"interpRunScript: out of memory!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: out of memory!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          458 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        1 => {
                            // The type of the variable is stored in with the opcode
                            sVal.type_0 =
                                (*ip &
                                     0xffffff as libc::c_int as libc::c_uint)
                                    as INTERP_TYPE;
                            // store the pointer
                            psVar =
                                interpGetVarData(psGlobals,
                                                 *ip.offset(1 as libc::c_int
                                                                as isize));
                            sVal.v.oval =
                                &mut (*psVar).v.ival as *mut SDWORD as
                                    *mut libc::c_void;
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"PUSHREF     \x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if interpTrace != 0 { cpPrintVal(&mut sVal); }
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"\n\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if stackPush(&mut sVal) == 0 {
                                // Eeerk, out of memory
                                debug(LOG_ERROR,
                                      b"interpRunScript: out of memory!\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: out of memory!\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          476 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        2 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"POP\n\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if stackPop(&mut sVal) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do stack pop\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do stack pop\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          486 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        12 => {
                            if interpTrace != 0 { cpPrintMathsOp(data); }
                            if stackBinaryOp(data as OPCODE) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do binary op\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do binary op\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          496 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                if interpTrace != 0 { stackPrintTop(); }
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        13 => {
                            if interpTrace != 0 { cpPrintMathsOp(data); }
                            if stackUnaryOp(data as OPCODE) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do unary op\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do unary op\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          508 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                if interpTrace != 0 { stackPrintTop(); }
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        3 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER, data as *const libc::c_char);
                            }
                            if data >= numGlobals {
                                debug(LOG_ERROR,
                                      b"interpRunScript: variable index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: variable index out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          520 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else if stackPush(interpGetVarData(psGlobals,
                                                                 data)) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do stack push\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do stack push\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          526 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        4 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER, data as *const libc::c_char);
                            }
                            if interpTrace != 0 { stackPrintTop(); }
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"\n\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if data >= numGlobals {
                                debug(LOG_ERROR,
                                      b"interpRunScript: variable index out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: variable index out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          538 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else if stackPopType(interpGetVarData(psGlobals,
                                                                    data)) ==
                                          0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do stack pop\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do stack pop\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          544 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        5 => {
                            // get the number of array elements
	//			arrayElements = (data & ARRAY_ELEMENT_MASK) >> ARRAY_ELEMENT_SHIFT;
	//			data = data & ARRAY_INDEX_MASK;
				// get the array index
	//			if (!stackPopParams(1, VAL_INT, &arrayIndex))
	//			{
	//				goto exit_with_error;
	//			}
	//			TRCPRINTF(("PUSHARRAYGLOBAL  [%d] %d(+%d)\n", arrayIndex, data, arrayElements));
	//			if (data + arrayElements > numGlobals)
	//			{
	//				ASSERT( FALSE, "interpRunScript: variable index out of range" );
	//				goto exit_with_error;
	//			}
	//			if (arrayIndex < 0 || arrayIndex >= arrayElements)
	//			{
	//				ASSERT( FALSE, "interpRunScript: array index out of range" );
	//				goto exit_with_error;
	//			}
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"PUSHARRAYGLOBAL  \x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if interpGetArrayVarData(&mut ip, psGlobals,
                                                     psProg, &mut psVar) == 0
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not get array var data, CurEvent=%d\x00"
                                          as *const u8 as *const libc::c_char,
                                      CurEvent);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not get array var data\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          573 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                if !(stackPush(psVar) == 0) { continue ; }
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do stack push\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do stack push\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          580 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            }
                        }
                        6 => {
                            // get the number of array elements
	//			arrayElements = (data & ARRAY_ELEMENT_MASK) >> ARRAY_ELEMENT_SHIFT;
	//			data = data & ARRAY_INDEX_MASK;
				// get the array index
	/*			if (!stackPopParams(1, VAL_INT, &arrayIndex))
				{
					ASSERT( FALSE, "interpRunScript: could not do pop of params" );
					goto exit_with_error;
				}
				TRCPRINTF(("POPARRAYGLOBAL   [%d] %d(+%d) ", arrayIndex, data, arrayElements));
				TRCPRINTSTACKTOP();
				TRCPRINTF(("\n"));
				if (data + arrayElements > numGlobals)
				{
					ASSERT( FALSE, "interpRunScript: variable index out of range" );
					goto exit_with_error;
				}
				if (arrayIndex < 0 || arrayIndex >= arrayElements)
				{
					ASSERT( FALSE, "interpRunScript: array index out of range" );
					goto exit_with_error;
				}
				if (!stackPopType(interpGetVarData(psGlobals, data + arrayIndex)))
				{
					ASSERT( FALSE, "interpRunScript: could not do pop stack of type" );
					goto exit_with_error;
				}
				ip += aOpSize[opcode];*/
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"POPARRAYGLOBAL   \x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if interpGetArrayVarData(&mut ip, psGlobals,
                                                     psProg, &mut psVar) == 0
                               {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not get array var data\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not get array var data\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          617 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                if interpTrace != 0 { stackPrintTop(); }
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                if !(stackPopType(psVar) == 0) { continue ; }
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do pop stack of type\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do pop stack of type\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          625 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            }
                        }
                        11 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      (ip.wrapping_offset_from((*psProg).pCode)
                                           as libc::c_int +
                                           data as SWORD as libc::c_int) as
                                          *const libc::c_char);
                            }
                            if stackPop(&mut sVal) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do pop of stack\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do pop of stack\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          635 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else if sVal.v.bval == 0 {
                                // Do the jump
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b" - done -\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                ip =
                                    ip.offset(data as SWORD as libc::c_int as
                                                  isize);
                                if !(ip < pCodeStart || ip > pCodeEnd) {
                                    continue ;
                                }
                                debug(LOG_ERROR,
                                      b"interpRunScript: jump out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: jump out of range\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          646 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                if interpTrace != 0 {
                                    debug(LOG_NEVER,
                                          b"\n\x00" as *const u8 as
                                              *const libc::c_char);
                                }
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        9 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      (ip.wrapping_offset_from((*psProg).pCode)
                                           as libc::c_int +
                                           data as SWORD as libc::c_int) as
                                          *const libc::c_char);
                            }
                            // Do the jump
                            ip =
                                ip.offset(data as SWORD as libc::c_int as
                                              isize);
                            if !(ip < pCodeStart || ip > pCodeEnd) {
                                continue ;
                            }
                            debug(LOG_ERROR,
                                  b"interpRunScript: jump out of range\x00" as
                                      *const u8 as *const libc::c_char);
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"interpRunScript: jump out of range\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"interp.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      664 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 16],
                                                                &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            current_block = 17057041599611039122;
                            break ;
                        }
                        7 => {
                            //debug(LOG_SCRIPT, "OP_CALL");
                            if interpTrace != 0 {
                                cpPrintFunc(::std::mem::transmute::<libc::intptr_t,
                                                                    SCRIPT_FUNC>(*ip.offset(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                                                     as
                                                                                     libc::intptr_t));
                            }
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"\n\x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            scriptFunc =
                                ::std::mem::transmute::<libc::intptr_t,
                                                        SCRIPT_FUNC>(*ip.offset(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                                                         as
                                                                         libc::intptr_t);
                            //debug(LOG_SCRIPT, "OP_CALL 1");
                            if scriptFunc.expect("non-null function pointer")()
                                   == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do func\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do func\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          677 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                //debug(LOG_SCRIPT, "OP_CALL 2");
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        8 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER,
                                      b"VARCALL     \x00" as *const u8 as
                                          *const libc::c_char);
                            }
                            if interpTrace != 0 {
                                cpPrintVarFunc(::std::mem::transmute::<libc::intptr_t,
                                                                       SCRIPT_VARFUNC>(*ip.offset(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize)
                                                                                           as
                                                                                           libc::intptr_t),
                                               data);
                            }
                            if interpTrace != 0 {
                                debug(LOG_NEVER, data as *const libc::c_char);
                            }
                            scriptVarFunc =
                                ::std::mem::transmute::<libc::intptr_t,
                                                        SCRIPT_VARFUNC>(*ip.offset(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       isize)
                                                                            as
                                                                            libc::intptr_t);
                            if scriptVarFunc.expect("non-null function pointer")(data)
                                   == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not do var func\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not do var func\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          692 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                ip =
                                    ip.offset(aOpSize[opcode as usize] as
                                                  isize)
                            }
                        }
                        14 => {
                            // jump out of the code
                            ip = pCodeEnd
                        }
                        15 => {
                            if interpTrace != 0 {
                                debug(LOG_NEVER, data as *const libc::c_char);
                            }
                            if stackEmpty() != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"interpRunScript: OP_PAUSE without empty stack\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if stackEmpty() != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"interp.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      704 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 16],
                                                                &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                      b"stackEmpty()\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            ip = ip.offset(aOpSize[opcode as usize] as isize);
                            // tell the event system to reschedule this event
                            if eventAddPauseTrigger(psContext, index,
                                                    ip.wrapping_offset_from(pCodeBase)
                                                        as libc::c_int as
                                                        UDWORD, data) == 0 {
                                debug(LOG_ERROR,
                                      b"interpRunScript: could not add pause trigger\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"interpRunScript: could not add pause trigger\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"interp.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          710 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 16],
                                                                    &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                current_block = 17057041599611039122;
                                break ;
                            } else {
                                // now jump out of the event
                                ip = pCodeEnd
                            }
                        }
                        _ => {
                            debug(LOG_ERROR,
                                  b"interpRunScript: unknown opcode\x00" as
                                      *const u8 as *const libc::c_char);
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"interpRunScript: unknown opcode\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"interp.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      718 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 16],
                                                                &[libc::c_char; 16]>(b"interpRunScript\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            current_block = 17057041599611039122;
                            break ;
                        }
                    }
                }
            } else if IsRetStackEmpty() == 0 {
                //End of the event reached, see if we have to jump back to the caller function or just exit
                //debug(LOG_SCRIPT, "End of event reached");
                //There was a caller function before this one
                //debug(LOG_SCRIPT, "GetCallDepth = %d", GetCallDepth());
                //reset local vars (since trigger can't be called, only local vars of an event can be reset here)
                if resetLocalVars(psProg, CurEvent) == 0 {
                    debug(LOG_ERROR,
                          b"interpRunScript: could not reset local vars for event %d\x00"
                              as *const u8 as *const libc::c_char, CurEvent);
                    current_block = 17057041599611039122;
                    break ;
                } else {
                    if PopRetStack(&mut ip as *mut *mut UDWORD as *mut UDWORD)
                           == 0 {
                        //Pop return address
                        debug(LOG_ERROR,
                              b"interpRunScript() - PopRetStack(): failed to pop return adress.\x00"
                                  as *const u8 as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    //debug(LOG_SCRIPT, "Return adress = %d", ip);
                    if PopRetStack(&mut CurEvent) == 0 {
                        //Pop event index
                        debug(LOG_ERROR,
                              b"interpRunScript() - PopRetStack(): failed to pop return event index.\x00"
                                  as *const u8 as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                    //debug( LOG_SCRIPT, "RETURNED TO CALLER EVENT %d", CurEvent );
                    //Set new boundries
				//--------------------------
                    if IsRetStackEmpty() != 0 {
                        //if we jumped back to the original caller
                        if bEvent == 0 {
                            //original caller was a trigger (is it possible at all?)
                            pCodeBase =
                                (*psProg).pCode.offset(*(*psProg).pTriggerTab.offset(CurEvent
                                                                                         as
                                                                                         isize)
                                                           as libc::c_int as
                                                           isize);
                            pCodeStart = pCodeBase;
                            pCodeEnd =
                                (*psProg).pCode.offset(*(*psProg).pTriggerTab.offset(CurEvent.wrapping_add(1
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint)
                                                                                         as
                                                                                         isize)
                                                           as libc::c_int as
                                                           isize)
                        } else {
                            //original caller was an event
                            pCodeBase =
                                (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent
                                                                                       as
                                                                                       isize)
                                                           as libc::c_int as
                                                           isize); //also use the offset passed, since it's an original caller event (offset is used for pause() )
                            pCodeStart = pCodeBase.offset(offset as isize);
                            pCodeEnd =
                                (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent.wrapping_add(1
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             libc::c_uint)
                                                                                       as
                                                                                       isize)
                                                           as libc::c_int as
                                                           isize)
                        }
                    } else {
                        //we are still jumping thru functions (this can't be a callback, since it can't/should not be called)
                        pCodeBase =
                            (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent
                                                                                   as
                                                                                   isize)
                                                       as libc::c_int as
                                                       isize);
                        pCodeStart = pCodeBase;
                        pCodeEnd =
                            (*psProg).pCode.offset(*(*psProg).pEventTab.offset(CurEvent.wrapping_add(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                                   as
                                                                                   isize)
                                                       as libc::c_int as
                                                       isize)
                    }
                }
            } else {
                //debug( LOG_SCRIPT, " *** CALL STACK EMPTY ***" );
                //reset local vars only if original caller was an event, not a trigger
                if bEvent != 0 {
                    if resetLocalVars(psProg, index) == 0 {
                        debug(LOG_ERROR,
                              b"interpRunScript: could not reset local vars\x00"
                                  as *const u8 as *const libc::c_char);
                        current_block = 17057041599611039122;
                        break ;
                    }
                }
                bStop = 1 as libc::c_int
                //Stop execution of this event here, no more calling functions stored
            }
        }
        match current_block {
            17057041599611039122 => { }
            _ => {
                //debug(LOG_SCRIPT, "interpRunScript 3");
                if interpTrace != 0 {
                    debug(LOG_NEVER,
                          ip.wrapping_offset_from((*psProg).pCode) as
                              libc::c_int as *const libc::c_char);
                }
                bInterpRunning = 0 as libc::c_int;
                return 1 as libc::c_int
            }
        }
    }
    // Deal with the script crashing or running out of memory
    debug(LOG_ERROR,
          b"interpRunScript: *** ERROR EXIT *** (CurEvent=%d)\x00" as
              *const u8 as *const libc::c_char, CurEvent);
    if bEvent != 0 {
        debug(LOG_ERROR,
              b"Original event ID: %d (of %d)\x00" as *const u8 as
                  *const libc::c_char, index,
              (*psProg).numEvents as libc::c_int);
    } else {
        debug(LOG_ERROR,
              b"Original trigger ID: %d (of %d)\x00" as *const u8 as
                  *const libc::c_char, index,
              (*psProg).numTriggers as libc::c_int);
    }
    debug(LOG_ERROR,
          b"Current event ID: %d (of %d)\x00" as *const u8 as
              *const libc::c_char, CurEvent,
          (*psProg).numEvents as libc::c_int);
    callDepth = GetCallDepth();
    debug(LOG_ERROR,
          b"Call depth : %d\x00" as *const u8 as *const libc::c_char,
          callDepth);
    if !(*psProg).psDebug.is_null() {
        debug(LOG_ERROR,
              b"Displaying debug info:\x00" as *const u8 as
                  *const libc::c_char);
        if bEvent != 0 {
            // find the debug info for the original (caller)  event
            pEventLab = eventGetEventID(psProg, index as SDWORD);
            debug(LOG_ERROR,
                  b"Original event name: %s\x00" as *const u8 as
                      *const libc::c_char, pEventLab);
            pEventLab = eventGetEventID(psProg, CurEvent as SDWORD);
            debug(LOG_ERROR,
                  b"Current event name: %s\x00" as *const u8 as
                      *const libc::c_char, pEventLab);
        } else {
            // find the debug info for the trigger
            pTrigLab = eventGetTriggerID(psProg, index as SDWORD);
            debug(LOG_ERROR,
                  b"Trigger: %s\x00" as *const u8 as *const libc::c_char,
                  pTrigLab);
        }
    }
    if interpTrace != 0 {
        debug(LOG_NEVER,
              b"*** ERROR EXIT ***\n\x00" as *const u8 as
                  *const libc::c_char);
    }
    bInterpRunning = 0 as libc::c_int;
    return 0 as libc::c_int;
}
/* Set the type equivalence table */
#[no_mangle]
pub unsafe extern "C" fn scriptSetTypeEquiv(mut psTypeTab: *mut TYPE_EQUIV) {
    asInterpTypeEquiv = psTypeTab;
}
/* The size of each opcode */
/* Check if two types are equivalent */
/* Check if two types are equivalent */
#[no_mangle]
pub unsafe extern "C" fn interpCheckEquiv(mut to: INTERP_TYPE,
                                          mut from: INTERP_TYPE) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut toRef: BOOL = 0 as libc::c_int;
    let mut fromRef: BOOL = 0 as libc::c_int;
    // check for the VAL_REF flag
    if to as libc::c_uint & 0x100000 as libc::c_int as libc::c_uint != 0 {
        toRef = 1 as libc::c_int;
        to =
            (to as libc::c_uint & !(0x100000 as libc::c_int) as libc::c_uint)
                as INTERP_TYPE
    }
    if from as libc::c_uint & 0x100000 as libc::c_int as libc::c_uint != 0 {
        fromRef = 1 as libc::c_int;
        from =
            (from as libc::c_uint &
                 !(0x100000 as libc::c_int) as libc::c_uint) as INTERP_TYPE
    }
    if toRef != fromRef { return 0 as libc::c_int }
    if to as libc::c_uint == from as libc::c_uint {
        return 1 as libc::c_int
    } else {
        if !asInterpTypeEquiv.is_null() {
            i = 0 as libc::c_int;
            while (*asInterpTypeEquiv.offset(i as isize)).base as libc::c_uint
                      != 0 as libc::c_int as libc::c_uint {
                if (*asInterpTypeEquiv.offset(i as isize)).base as
                       libc::c_uint == to as libc::c_uint {
                    j = 0 as libc::c_int;
                    while j < (*asInterpTypeEquiv.offset(i as isize)).numEquiv
                          {
                        if (*asInterpTypeEquiv.offset(i as
                                                          isize)).aEquivTypes[j
                                                                                  as
                                                                                  usize]
                               as libc::c_uint == from as libc::c_uint {
                            return 1 as libc::c_int
                        }
                        j += 1
                    }
                }
                i += 1
            }
        }
    }
    return 0 as libc::c_int;
}
/* Instinct function to turn on tracing */
#[no_mangle]
pub unsafe extern "C" fn interpTraceOn() -> BOOL {
    interpTrace = 1 as libc::c_int;
    return 1 as libc::c_int;
}
/* Instinct function to turn off tracing */
#[no_mangle]
pub unsafe extern "C" fn interpTraceOff() -> BOOL {
    interpTrace = 0 as libc::c_int;
    return 1 as libc::c_int;
}
/* Call stack stuff */
static mut retStack: [UDWORD; 200] = [0; 200];
//Primitive stack
static mut retStackPos: SDWORD = 0;
//Current Position, always points to the last valid element
#[no_mangle]
pub unsafe extern "C" fn retStackReset() {
    retStackPos = -(1 as libc::c_int);
    //Beginning of the stack
}
//Remember current script execution adress
#[no_mangle]
pub unsafe extern "C" fn RetStackRemember(mut EvTrigIndex: UDWORD,
                                          mut address: UDWORD) -> UDWORD {
    //DbgMsg("To remember: %d, %d, %d", EvTrigIndex, address, retStackPos);
    if retStackPos >= 200 as libc::c_int {
        debug(LOG_ERROR,
              b"RetStackRemember() - return address stack is full\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int as UDWORD
        //Stack full
    } //First store event index
    retStackPos = retStackPos + 2 as libc::c_int; //current ip
    retStack[(retStackPos - 1 as libc::c_int) as usize] = EvTrigIndex;
    retStack[retStackPos as usize] = address;
    //debug( LOG_SCRIPT, "RetStackRemember: ip=%d, event=%d", address, EvTrigIndex);
    return 1 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn IsRetStackEmpty() -> BOOL {
    if retStackPos == -(1 as libc::c_int) { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PopRetStack(mut psVal: *mut UDWORD) -> BOOL {
    if retStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"PopRetStack: retStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    *psVal = retStack[retStackPos as usize];
    retStackPos = retStackPos - 1 as libc::c_int;
    //debug( LOG_SCRIPT, "PopRetStack: val=%d", *psVal);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetCallDepth() -> SDWORD {
    return (retStackPos + 1 as libc::c_int) / 2 as libc::c_int;
}
