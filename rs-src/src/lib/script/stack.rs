use ::libc;
extern "C" {
    /* Set a block heap to use for all memory allocation rather than standard malloc/free */
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    /* Get the current block heap */
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
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
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Check if two types are equivalent */
    #[no_mangle]
    fn interpCheckEquiv(to: INTERP_TYPE, from: INTERP_TYPE) -> BOOL;
    /* Display a value  */
    #[no_mangle]
    fn cpPrintVal(psVal: *mut INTERP_VAL);
}
pub type __builtin_va_list = *mut libc::c_char;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
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
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
pub type BLOCK_HEAP = _block_heap;
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
// flag to specify a variable reference rather than simple value
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
//Points to the top of the string stack
/* store for a 'chunk' of the stack */
pub type STACK_CHUNK = _stack_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _stack_chunk {
    pub aVals: *mut INTERP_VAL,
    pub size: UDWORD,
    pub psNext: *mut _stack_chunk,
    pub psPrev: *mut _stack_chunk,
}
#[no_mangle]
pub static mut STRSTACK: [[libc::c_char; 255]; 6000] = [[0; 255]; 6000];
//simple string 'stack'
#[no_mangle]
pub static mut CURSTACKSTR: UDWORD = 0 as libc::c_int as UDWORD;
/* The first chunk of the stack */
static mut psStackBase: *mut STACK_CHUNK =
    0 as *const STACK_CHUNK as *mut STACK_CHUNK;
/* The current stack chunk */
static mut psCurrChunk: *mut STACK_CHUNK =
    0 as *const STACK_CHUNK as *mut STACK_CHUNK;
/* The current free entry on the current stack chunk */
static mut currEntry: UDWORD = 0 as libc::c_int as UDWORD;
/* The block heap the stack was created in */
static mut psStackBlock: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
/* Check if the stack is empty */
/* Check if the stack is empty */
#[no_mangle]
pub unsafe extern "C" fn stackEmpty() -> BOOL {
    return (psCurrChunk == psStackBase &&
                currEntry == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
/* Allocate a new chunk for the stack */
unsafe extern "C" fn stackNewChunk(mut size: UDWORD) -> BOOL {
    let mut psHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    /* see if a chunk has already been alocated */
    if !(*psCurrChunk).psNext.is_null() {
        psCurrChunk = (*psCurrChunk).psNext;
        currEntry = 0 as libc::c_int as UDWORD
    } else {
        psHeap = memGetBlockHeap();
        memSetBlockHeap(psStackBlock);
        /* Allocate a new chunk */
        (*psCurrChunk).psNext =
            memMallocRelease(::std::mem::size_of::<STACK_CHUNK>() as
                                 libc::c_ulong) as *mut STACK_CHUNK;
        if (*psCurrChunk).psNext.is_null() { return 0 as libc::c_int }
        (*(*psCurrChunk).psNext).aVals =
            memMallocRelease((::std::mem::size_of::<INTERP_VAL>() as
                                  libc::c_ulong).wrapping_mul(size)) as
                *mut INTERP_VAL;
        if (*(*psCurrChunk).psNext).aVals.is_null() {
            memFreeRelease((*psCurrChunk).psNext as *mut libc::c_void);
            (*psCurrChunk).psNext = 0 as *mut _stack_chunk;
            return 0 as libc::c_int
        }
        (*(*psCurrChunk).psNext).size = size;
        (*(*psCurrChunk).psNext).psPrev = psCurrChunk;
        (*(*psCurrChunk).psNext).psNext = 0 as *mut _stack_chunk;
        psCurrChunk = (*psCurrChunk).psNext;
        currEntry = 0 as libc::c_int as UDWORD;
        memSetBlockHeap(psHeap);
    }
    return 1 as libc::c_int;
}
/* Push a value onto the stack */
/* Push a value onto the stack */
#[no_mangle]
pub unsafe extern "C" fn stackPush(mut psVal: *mut INTERP_VAL) -> BOOL {
    /* Store the value in the stack - psCurrChunk/currEntry always point to
	   valid space */
    memcpy(&mut *(*psCurrChunk).aVals.offset(currEntry as isize) as
               *mut INTERP_VAL as *mut libc::c_void,
           psVal as *const libc::c_void,
           ::std::mem::size_of::<INTERP_VAL>() as libc::c_ulong);
    /* String support: Copy the string, otherwise the stack will operate directly on the
	original string (like & opcode will actually store the result in the first
	variable which would point to the original string) */
    if (*psVal).type_0 as libc::c_uint ==
           VAL_STRING as libc::c_int as libc::c_uint {
        let ref mut fresh0 =
            (*(*psCurrChunk).aVals.offset(currEntry as isize)).v.sval;
        *fresh0 =
            memMallocRelease(255 as libc::c_int as size_t) as
                *mut libc::c_char;
        strcpy((*(*psCurrChunk).aVals.offset(currEntry as isize)).v.sval,
               (*psVal).v.sval);
    }
    /* Now update psCurrChunk and currEntry */
    currEntry = currEntry.wrapping_add(1);
    if currEntry == (*psCurrChunk).size {
        /* At the end of this chunk, need a new one */
        if stackNewChunk(2 as libc::c_int as UDWORD) == 0 {
            /* Out of memory */
            debug(LOG_ERROR,
                  b"stackPush: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/* Pop a value off the stack */
/* Pop a value off the stack */
#[no_mangle]
pub unsafe extern "C" fn stackPop(mut psVal: *mut INTERP_VAL) -> BOOL {
    if (*psCurrChunk).psPrev.is_null() &&
           currEntry == 0 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"stackPop: stack empty\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackPop: stack empty\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  134 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 9],
                                            &[libc::c_char; 9]>(b"stackPop\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* move the stack pointer down one */
    if currEntry == 0 as libc::c_int as libc::c_uint {
        /* have to move onto the previous chunk. */
        psCurrChunk = (*psCurrChunk).psPrev;
        currEntry =
            (*psCurrChunk).size.wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else { currEntry = currEntry.wrapping_sub(1) }
    /* copy the value off the stack */
    memcpy(psVal as *mut libc::c_void,
           &mut *(*psCurrChunk).aVals.offset(currEntry as isize) as
               *mut INTERP_VAL as *const libc::c_void,
           ::std::mem::size_of::<INTERP_VAL>() as libc::c_ulong);
    return 1 as libc::c_int;
}
/* Pop a value off the stack, checking that the type matches what is passed in */
/* Pop a value off the stack, checking that the type matches what is passed in */
#[no_mangle]
pub unsafe extern "C" fn stackPopType(mut psVal: *mut INTERP_VAL) -> BOOL {
    let mut psTop: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    //debug(LOG_SCRIPT, "stackPopType 1");
    if (*psCurrChunk).psPrev.is_null() &&
           currEntry == 0 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"stackPopType: stack empty\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackPopType: stack empty\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  167 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"stackPopType\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //debug(LOG_SCRIPT, "stackPopType 2");
    /* move the stack pointer down one */
    if currEntry == 0 as libc::c_int as libc::c_uint {
        /* have to move onto the previous chunk. */
        psCurrChunk = (*psCurrChunk).psPrev;
        currEntry =
            (*psCurrChunk).size.wrapping_sub(1 as libc::c_int as libc::c_uint)
    } else { currEntry = currEntry.wrapping_sub(1) }
    //debug(LOG_SCRIPT, "stackPopType 3");
    psTop = (*psCurrChunk).aVals.offset(currEntry as isize);
    //debug(LOG_SCRIPT, "stackPopType 4");
    //String support
    if (*psVal).type_0 as libc::c_uint ==
           VAL_STRING as libc::c_int as libc::c_uint {
        //If we are about to assign something to a string variable (psVal)
        if (*psTop).type_0 as libc::c_uint !=
               VAL_STRING as libc::c_int as libc::c_uint {
            //if assigning a non-string to a string, then convert it
            /* Check for compatible types */
            if (*psTop).type_0 as libc::c_uint ==
                   VAL_INT as libc::c_int as libc::c_uint ||
                   (*psTop).type_0 as libc::c_uint ==
                       VAL_BOOL as libc::c_int as libc::c_uint {
                let mut tempstr: *mut STRING = 0 as *mut STRING;
                tempstr =
                    memMallocRelease(255 as libc::c_int as size_t) as
                        *mut libc::c_char;
                sprintf(tempstr,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*psTop).v.ival);
                (*psVal).v.sval = tempstr
            } else {
                debug(LOG_ERROR,
                      b"stackPopType: trying to assign an incompatible data type to a string variable (type = %d)\x00"
                          as *const u8 as *const libc::c_char,
                      (*psTop).type_0 as libc::c_uint);
                return 0 as libc::c_int
            }
        } else {
            //Assigning a string to a string, just do the default action
            (*psVal).v.ival = (*psTop).v.ival
        }
    } else if interpCheckEquiv((*psVal).type_0, (*psTop).type_0) == 0 {
        debug(LOG_ERROR,
              b"stackPopType: type mismatch\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackPopType: type mismatch\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  221 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"stackPopType\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // we are about to assign something to a non-string variable (psVal)
    //debug(LOG_SCRIPT, "stackPopType 5");
    /* copy the value off the stack */
    (*psVal).v.ival = (*psTop).v.ival;
    //debug(LOG_SCRIPT, "stackPopType 6");
    return 1 as libc::c_int;
}
/* Pop a number of values off the stack checking their types
 * This is used by instinct functions to get their parameters
 */
#[no_mangle]
pub unsafe extern "C" fn stackPopParams(mut numParams: SDWORD, mut args: ...)
 -> BOOL {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    let mut i: SDWORD = 0;
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut pData: *mut UDWORD = 0 as *mut UDWORD;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    let mut index: UDWORD = 0;
    let mut params: UDWORD = 0;
    let mut psCurr: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    //debug(LOG_SCRIPT,"stackPopParams");
    args_0 = args.clone();
    // Find the position of the first parameter, and set
	// the stack top to it
    if numParams as UDWORD <= currEntry {
        // parameters are all on current chunk
        currEntry = currEntry.wrapping_sub(numParams as libc::c_uint);
        psCurr = psCurrChunk
    } else {
        // Have to work down the previous chunks to find the first param
        params = (numParams as libc::c_uint).wrapping_sub(currEntry);
        psCurr = (*psCurrChunk).psPrev;
        while !psCurr.is_null() {
            if params <= (*psCurr).size {
                // found the first param
                currEntry = (*psCurr).size.wrapping_sub(params);
                psCurrChunk = psCurr;
                break ;
            } else {
                params =
                    (params as libc::c_uint).wrapping_sub((*psCurr).size) as
                        UDWORD as UDWORD;
                psCurr = (*psCurr).psPrev
            }
        }
    }
    //debug(LOG_SCRIPT,"stackPopParams 1");
    if psCurr.is_null() {
        debug(LOG_ERROR,
              b"stackPopParams: not enough parameters on stack\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackPopParams: not enough parameters on stack\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  288 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"stackPopParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //debug(LOG_SCRIPT,"stackPopParams 2");
    //string support
	// Get the values, checking their types
    index = currEntry;
    i = 0 as libc::c_int;
    while i < numParams {
        type_0 = args_0.arg::<libc::c_int>() as INTERP_TYPE;
        pData = args_0.arg::<*mut UDWORD>();
        //debug(LOG_SCRIPT,"stackPopParams 3 (%d parameter)", i);
        psVal = (*psCurr).aVals.offset(index as isize);
        if type_0 as libc::c_uint != VAL_STRING as libc::c_int as libc::c_uint
           {
            //Allow param to be any type, if function expects a string (auto-convert later)
            //debug(LOG_SCRIPT,"stackPopParams 4 - non string");
            if interpCheckEquiv(type_0, (*psVal).type_0) == 0 {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"stackPopParams: type mismatch\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"stack.c\x00" as *const u8 as *const libc::c_char,
                          312 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"stackPopParams\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            *pData = (*psVal).v.ival as UDWORD
        } else if (*psVal).type_0 as libc::c_uint ==
                      VAL_STRING as libc::c_int as libc::c_uint {
            //TODO: allow only compatible types
            //debug(LOG_SCRIPT,"stackPopParams 5 - string");
            //Passing a String
            //debug(LOG_SCRIPT, "stackPopParams - string");
            *pData = (*psVal).v.ival as UDWORD;
            debug(LOG_SCRIPT, b"%s\x00" as *const u8 as *const libc::c_char,
                  *pData);
        } else {
            //Integer
            let mut tempstr: *mut STRING = 0 as *mut STRING;
            //itoa(psVal->v.ival,tmpstr,10);
            tempstr =
                memMallocRelease(255 as libc::c_int as size_t) as
                    *mut libc::c_char;
            sprintf(tempstr, b"%d\x00" as *const u8 as *const libc::c_char,
                    (*psVal).v.ival);
            *pData = tempstr as UDWORD
        }
        index =
            (index as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        if index >= (*psCurr).size {
            psCurr = (*psCurr).psNext;
            index = 0 as libc::c_int as UDWORD
        }
        i += 1
    }
    //debug(LOG_SCRIPT, "stackPopParams - non string");
    //debug(LOG_SCRIPT,"END stackPopParams");
    return 1 as libc::c_int;
}
/* Set the constant table */
/* Set the callback table */
/* Set the type equivalence table */
/* **********************************************************************************
 *
 * Return stack stuff
 */
/* **********************************************************************************
 *
 * Compiler functions
 */
/* Compile a script program */
/* Free a SCRIPT_CODE structure */
/* Display the contents of a program in readable form */
/* Lookup a script variable */
/* Run a compiled script */
/* **********************************************************************************
 *
 * Event system functions
 */
// Whether a context is released when there are no active triggers for it
// release the context
// do not release the context
// reset the event system
// Initialise the create/release function array - specify the maximum value type
// a create function for data stored in an INTERP_VAL
// a release function for data stored in an INTERP_VAL
// Add a new value create function
// Add a new value release function
// Create a new context for a script
// Copy a context, including variable values
// Add a new object to the trigger system
// Time is the application time at which all the triggers are to be started
// Remove a context from the event system
// Set a global variable value for a context
// Get the value pointer for a variable index
// Process all the currently active triggers
// Time is the application time at which all the triggers are to be processed
// Activate a callback trigger
/* **********************************************************************************
 *
 * Support functions for writing instinct functions
 */
/* Pop a number of values off the stack checking their types
 * This is used by instinct functions to get their parameters
 * The varargs part is a set of INTERP_TYPE, UDWORD * pairs.
 * The value of the parameter is stored in the DWORD pointed to by the UDWORD *
 */
/* Push a value onto the stack without using a value structure */
/* Push a value onto the stack without using a value structure */
#[no_mangle]
pub unsafe extern "C" fn stackPushResult(mut type_0: INTERP_TYPE,
                                         mut data: SDWORD) -> BOOL {
    // Store the value
    (*(*psCurrChunk).aVals.offset(currEntry as isize)).type_0 = type_0;
    (*(*psCurrChunk).aVals.offset(currEntry as isize)).v.ival = data;
    // Now update psCurrChunk and currEntry
    currEntry = currEntry.wrapping_add(1);
    if currEntry == (*psCurrChunk).size {
        // At the end of this chunk, need a new one
        if stackNewChunk(2 as libc::c_int as UDWORD) == 0 {
            // Out of memory
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/* Look at a value on the stack without removing it.
 * index is how far down the stack to look.
 * Index 0 is the top entry on the stack.
 */
/* Look at a value on the stack without removing it.
 * index is how far down the stack to look.
 * Index 0 is the top entry on the stack.
 */
#[no_mangle]
pub unsafe extern "C" fn stackPeek(mut psVal: *mut INTERP_VAL,
                                   mut index: UDWORD) -> BOOL {
    let mut psCurr: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    if index < currEntry {
        /* Looking at entry on current chunk */
        memcpy(psVal as *mut libc::c_void,
               &mut *(*psCurrChunk).aVals.offset(currEntry.wrapping_sub(index).wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                     as isize) as
                   *mut INTERP_VAL as *const libc::c_void,
               ::std::mem::size_of::<INTERP_VAL>() as libc::c_ulong);
        return 1 as libc::c_int
    } else {
        /* Have to work down the previous chunks to find the entry */
        index =
            (index as libc::c_uint).wrapping_sub(currEntry) as UDWORD as
                UDWORD;
        psCurr = (*psCurrChunk).psPrev;
        while !psCurr.is_null() {
            if index < (*psCurr).size {
                /* found the entry */
                memcpy(psVal as *mut libc::c_void,
                       &mut *(*psCurr).aVals.offset((*psCurr).size.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_sub(index)
                                                        as isize) as
                           *mut INTERP_VAL as *const libc::c_void,
                       ::std::mem::size_of::<INTERP_VAL>() as libc::c_ulong);
                return 1 as libc::c_int
            } else {
                index =
                    (index as libc::c_uint).wrapping_sub((*psCurr).size) as
                        UDWORD as UDWORD
            }
            psCurr = (*psCurr).psPrev
        }
    }
    /* If we got here the index is off the bottom of the stack */
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"stackPeek: index too large\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stack.c\x00" as *const u8 as *const libc::c_char,
              415 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"stackPeek\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int;
}
/* Print the top value on the stack */
/* Print the top value on the stack */
#[no_mangle]
pub unsafe extern "C" fn stackPrintTop() {
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    if stackPeek(&mut sVal, 0 as libc::c_int as UDWORD) != 0 {
        cpPrintVal(&mut sVal);
    } else {
        debug(LOG_NEVER,
              b"STACK EMPTY\x00" as *const u8 as *const libc::c_char);
    };
}
/* Do binary operations on the top of the stack
 * This effectively pops two values and pushes the result
 */
/* Do binary operations on the top of the stack
 * This effectively pops two values and pushes the result
 */
#[no_mangle]
pub unsafe extern "C" fn stackBinaryOp(mut opcode: OPCODE) -> BOOL {
    let mut psChunk: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    let mut psV1: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    let mut psV2: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    // Get the parameters
    if (*psCurrChunk).psPrev.is_null() &&
           currEntry < 2 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"stackBinaryOp: not enough entries on stack\x00" as *const u8
                  as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackBinaryOp: not enough entries on stack\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  447 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"stackBinaryOp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if currEntry > 1 as libc::c_int as libc::c_uint {
        psV1 =
            (*psCurrChunk).aVals.offset(currEntry as
                                            isize).offset(-(2 as libc::c_int
                                                                as isize));
        psV2 =
            (*psCurrChunk).aVals.offset(currEntry as
                                            isize).offset(-(1 as libc::c_int
                                                                as isize));
        currEntry =
            (currEntry as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    } else if currEntry == 1 as libc::c_int as libc::c_uint {
        // One value is on the previous chunk
        psChunk = (*psCurrChunk).psPrev;
        psV1 =
            (*psChunk).aVals.offset((*psChunk).size as
                                        isize).offset(-(1 as libc::c_int as
                                                            isize));
        psV2 =
            (*psCurrChunk).aVals.offset(currEntry as
                                            isize).offset(-(1 as libc::c_int
                                                                as isize));
        currEntry =
            (currEntry as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    } else {
        // both on the previous chunk, pop to the previous chunk
        psCurrChunk = (*psCurrChunk).psPrev;
        currEntry =
            (*psCurrChunk).size.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint);
        psV1 =
            (*psCurrChunk).aVals.offset((*psCurrChunk).size as
                                            isize).offset(-(2 as libc::c_int
                                                                as isize));
        psV2 =
            (*psCurrChunk).aVals.offset((*psCurrChunk).size as
                                            isize).offset(-(1 as libc::c_int
                                                                as isize))
    }
    if opcode as libc::c_uint != OP_CANC as libc::c_int as libc::c_uint {
        //string - Don't check if OP_CANC, since types can be mixed here
        if interpCheckEquiv((*psV1).type_0, (*psV2).type_0) == 0 {
            debug(LOG_ERROR,
                  b"stackBinaryOp: type mismatch\x00" as *const u8 as
                      *const libc::c_char);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"stackBinaryOp: type mismatch\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stack.c\x00" as *const u8 as *const libc::c_char,
                      479 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"stackBinaryOp\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    // do the operation
    match opcode as libc::c_uint {
        16 => { (*psV1).v.ival = (*psV1).v.ival + (*psV2).v.ival }
        17 => { (*psV1).v.ival = (*psV1).v.ival - (*psV2).v.ival }
        18 => { (*psV1).v.ival = (*psV1).v.ival * (*psV2).v.ival }
        19 => { (*psV1).v.ival = (*psV1).v.ival / (*psV2).v.ival }
        21 => {
            (*psV1).v.bval =
                ((*psV1).v.bval != 0 && (*psV2).v.bval != 0) as libc::c_int
        }
        22 => {
            (*psV1).v.bval =
                ((*psV1).v.bval != 0 || (*psV2).v.bval != 0) as libc::c_int
        }
        25 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.ival = ((*psV1).v.ival == (*psV2).v.ival) as libc::c_int
        }
        26 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.ival = ((*psV1).v.ival != (*psV2).v.ival) as libc::c_int
        }
        27 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.bval = ((*psV1).v.ival >= (*psV2).v.ival) as libc::c_int
        }
        28 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.bval = ((*psV1).v.ival <= (*psV2).v.ival) as libc::c_int
        }
        29 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.bval = ((*psV1).v.ival > (*psV2).v.ival) as libc::c_int
        }
        30 => {
            (*psV1).type_0 = VAL_BOOL;
            (*psV1).v.bval = ((*psV1).v.ival < (*psV2).v.ival) as libc::c_int
        }
        24 => {
            //String cancatenation
            let mut tempstr1: [*mut libc::c_char; 255] =
                [0 as *mut libc::c_char; 255];
            let mut tempstr2: [*mut libc::c_char; 255] =
                [0 as *mut libc::c_char; 255];
            /* Check first value if it's compatible with Strings */
            if (*psV1).type_0 as libc::c_uint ==
                   VAL_INT as libc::c_int as libc::c_uint ||
                   (*psV1).type_0 as libc::c_uint ==
                       VAL_BOOL as libc::c_int as libc::c_uint {
                //First value isn't string, but can be converted to string
                sprintf(tempstr1.as_mut_ptr() as *mut libc::c_char,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*psV1).v.ival); //Convert
                (*psV1).type_0 = VAL_STRING
            } else if (*psV1).type_0 as libc::c_uint ==
                          VAL_STRING as libc::c_int as libc::c_uint {
                //Is a string
                strcpy(tempstr1.as_mut_ptr() as *mut libc::c_char,
                       (*psV1).v.sval);
            } else {
                debug(LOG_ERROR,
                      b"stackBinaryOp: OP_CANC: first parameter is not compatible with Strings\x00"
                          as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
            /* Check first value if it's compatible with Strings */
            if (*psV2).type_0 as libc::c_uint ==
                   VAL_INT as libc::c_int as libc::c_uint ||
                   (*psV2).type_0 as libc::c_uint ==
                       VAL_BOOL as libc::c_int as libc::c_uint {
                sprintf(tempstr2.as_mut_ptr() as *mut libc::c_char,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        (*psV2).v.ival);
                //Convert
            } else if (*psV2).type_0 as libc::c_uint ==
                          VAL_STRING as libc::c_int as libc::c_uint {
                //Is a string
                strcpy(tempstr2.as_mut_ptr() as *mut libc::c_char,
                       (*psV2).v.sval); //Assign
            } else {
                debug(LOG_ERROR,
                      b"stackBinaryOp: OP_CANC: first parameter is not compatible with Strings\x00"
                          as *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
            strcat(tempstr1.as_mut_ptr() as *mut libc::c_char,
                   tempstr2.as_mut_ptr() as *const libc::c_char);
            strcpy((*psV1).v.sval,
                   tempstr1.as_mut_ptr() as *const libc::c_char);
            (*psV1).type_0 = VAL_STRING
        }
        _ => {
            debug(LOG_ERROR,
                  b"stackBinaryOp: unknown opcode\x00" as *const u8 as
                      *const libc::c_char);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"stackBinaryOp: unknown opcode\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stack.c\x00" as *const u8 as *const libc::c_char,
                      576 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"stackBinaryOp\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
/* Perform a unary operation on the top of the stack
 * This effectively pops a value and pushes the result
 */
/* Perform a unary operation on the top of the stack
 * This effectively pops a value and pushes the result
 */
#[no_mangle]
pub unsafe extern "C" fn stackUnaryOp(mut opcode: OPCODE) -> BOOL {
    let mut psChunk: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    // Get the value
    if (*psCurrChunk).psPrev.is_null() &&
           currEntry == 0 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"stackUnaryOp: not enough entries on stack\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stack.c\x00" as *const u8 as *const libc::c_char,
                  596 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"stackUnaryOp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if currEntry > 0 as libc::c_int as libc::c_uint {
        psVal =
            (*psCurrChunk).aVals.offset(currEntry as
                                            isize).offset(-(1 as libc::c_int
                                                                as isize))
    } else {
        // Value is on the previous chunk
        psChunk = (*psCurrChunk).psPrev;
        psVal =
            (*psChunk).aVals.offset((*psChunk).size as
                                        isize).offset(-(1 as libc::c_int as
                                                            isize))
    }
    // Do the operation
    match opcode as libc::c_uint {
        20 => {
            match (*psVal).type_0 as libc::c_uint {
                1 => { (*psVal).v.ival = -(*psVal).v.ival }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"stackUnaryOp: invalid type for negation\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"stack.c\x00" as *const u8 as
                                  *const libc::c_char, 621 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"stackUnaryOp\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
        }
        23 => {
            match (*psVal).type_0 as libc::c_uint {
                0 => {
                    (*psVal).v.bval = ((*psVal).v.bval == 0) as libc::c_int
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"stackUnaryOp: invalid type for NOT\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"stack.c\x00" as *const u8 as
                                  *const libc::c_char, 632 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"stackUnaryOp\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"stackUnaryOp: unknown opcode\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stack.c\x00" as *const u8 as *const libc::c_char,
                      637 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"stackUnaryOp\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 1 as libc::c_int;
}
/*
 * Stack.h
 *
 * Interface to the stack system
 */
//String support
//-----------------------------
//Max len of a single string
/* Initialise the stack */
/* Initialise the stack */
#[no_mangle]
pub unsafe extern "C" fn stackInitialise() -> BOOL {
    psStackBlock = memGetBlockHeap();
    psStackBase =
        memMallocRelease(::std::mem::size_of::<STACK_CHUNK>() as
                             libc::c_ulong) as *mut STACK_CHUNK;
    if psStackBase.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    (*psStackBase).aVals =
        memMallocRelease((::std::mem::size_of::<INTERP_VAL>() as
                              libc::c_ulong).wrapping_mul(15 as libc::c_int as
                                                              libc::c_uint))
            as *mut INTERP_VAL;
    if (*psStackBase).aVals.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    (*psStackBase).size = 15 as libc::c_int as UDWORD;
    (*psStackBase).psPrev = 0 as *mut _stack_chunk;
    (*psStackBase).psNext = 0 as *mut _stack_chunk;
    psCurrChunk = psStackBase;
    //string support
    CURSTACKSTR = 0 as libc::c_int as UDWORD; //initialize string 'stack'
    return 1 as libc::c_int;
}
/* Shutdown the stack */
/* Shutdown the stack */
#[no_mangle]
pub unsafe extern "C" fn stackShutDown() {
    let mut psCurr: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    let mut psNext: *mut STACK_CHUNK = 0 as *mut STACK_CHUNK;
    if psCurrChunk != psStackBase &&
           currEntry != 0 as libc::c_int as libc::c_uint {
        debug(LOG_NEVER,
              b"stackShutDown: stack is not empty on shutdown\x00" as
                  *const u8 as *const libc::c_char);
    }
    psCurr = psStackBase;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        memFreeRelease((*psCurr).aVals as *mut libc::c_void);
        (*psCurr).aVals = 0 as *mut INTERP_VAL;
        memFreeRelease(psCurr as *mut libc::c_void);
        psCurr = 0 as *mut STACK_CHUNK;
        psCurr = psNext
    };
}
/* Reset the stack to an empty state */
/* Reset the stack to an empty state */
#[no_mangle]
pub unsafe extern "C" fn stackReset() {
    if psCurrChunk == psStackBase &&
           currEntry == 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"stackReset: stack is not empty\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psCurrChunk == psStackBase &&
           currEntry == 0 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stack.c\x00" as *const u8 as *const libc::c_char,
              701 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"stackReset\x00")).as_ptr(),
              b"((psCurrChunk == psStackBase) && (currEntry == 0))\x00" as
                  *const u8 as *const libc::c_char);
    };
    psCurrChunk = psStackBase;
    currEntry = 0 as libc::c_int as UDWORD;
}
/* Pop the top value from the stack and replace the new top value
 * This is used to return the result of a binary maths operator
 */
/*BOOL stackPopAndSet(UDWORD type, UDWORD data)
{
	STACK_CHUNK		*psChunk;
	INTERP_VAL		*psVal;

	if (psCurrChunk->psPrev == NULL && currEntry < 2)
	{
		ASSERT( FALSE, "stackGetSecond: not enough entries on stack" );
		return FALSE;
	}

	if (currEntry > 1)
	{
		psVal = psCurrChunk->aVals + currEntry - 2;
		currEntry -= 1;
	}
	else if (currEntry == 1)
	{
		// Value is on the previous chunk, but pop doesn't change the chunk
		psChunk = psCurrChunk->psPrev;
		psVal = psChunk->aVals + psChunk->size - 1;
		currEntry -= 1;
	}
	else
	{
		// pop to the previous chunk
		psCurrChunk = psCurrChunk->psPrev;
		psVal = psCurrChunk->aVals + psCurrChunk->size - 2;
		currEntry = psCurrChunk->size - 1;
	}

	// The data could be any type but this will copy it over
	psVal->type = type;
	psVal->v.ival = (SDWORD)data;

	return TRUE;
}*/
/* Get pointers to the two top values */
/*BOOL stackTopTwo(INTERP_VAL **ppsV1, INTERP_VAL **ppsV2)
{
	STACK_CHUNK		*psChunk;

	if (psCurrChunk->psPrev == NULL && currEntry < 2)
	{
		ASSERT( FALSE, "stackGetSecond: not enough entries on stack" );
		return FALSE;
	}

	if (currEntry > 1)
	{
		*ppsV1 = psCurrChunk->aVals + currEntry - 2;
		*ppsV2 = psCurrChunk->aVals + currEntry - 1;
	}
	else if (currEntry == 1)
	{
		// Value is on the previous chunk, but pop doesn't change the chunk
		psChunk = psCurrChunk->psPrev;
		*ppsV1 = psChunk->aVals + psChunk->size - 1;
		*ppsV2 = psCurrChunk->aVals + currEntry - 1;
	}
	else
	{
		// both on the previous chunk
		psChunk = psCurrChunk->psPrev;
		*ppsV1 = psChunk->aVals + psChunk->size - 2;
		*ppsV2 = psChunk->aVals + psChunk->size - 1;
	}

	return TRUE;
}*/
/* Get the second entry on the stack
 * (i.e. the first value for a binary operator).
 */
/*BOOL stackGetSecond(UDWORD *pType, UDWORD *pData)
{
	STACK_CHUNK		*psChunk;
	INTERP_VAL		*psVal;

	if (psCurrChunk->psPrev == NULL && currEntry < 2)
	{
		ASSERT( FALSE, "stackGetSecond: not enough entries on stack" );
		return FALSE;
	}

	if (currEntry < 2)
	{
		// Value is on the previous chunk
		psChunk = psCurrChunk->psPrev;
		psVal = psChunk->aVals + psChunk->size + currEntry - 2;
	}
	else
	{
		psVal = psCurrChunk->aVals + currEntry - 2;
	}

	// The data could be any type but this will copy it over
	*pType = (UDWORD)psVal->type;
	*pData = (UDWORD)psVal->v.ival;

	return TRUE;
}*/
/* Replace the value at the top of the stack */
/*BOOL stackSetTop(UDWORD type, UDWORD data)
{
	STACK_CHUNK		*psChunk;
	INTERP_VAL		*psVal;

	if (psCurrChunk->psPrev == NULL && currEntry == 0)
	{
		ASSERT( FALSE, "stackSetTop: not enough entries on stack" );
		return FALSE;
	}

	if (currEntry == 0)
	{
		// Value is on the previous chunk
		psChunk = psCurrChunk->psPrev;
		psVal = psChunk->aVals + psChunk->size - 1;
	}
	else
	{
		psVal = psCurrChunk->aVals + currEntry - 1;
	}

	// The data could be any type but this will copy it over
	psVal->type = type;
	psVal->v.ival = (SDWORD)data;

	return TRUE;
}*/
/* Get the first entry on the stack
 * (i.e. the second value for a binary operator).
 */
/*BOOL stackGetTop(UDWORD *pType, UDWORD *pData)
{
	STACK_CHUNK		*psChunk;
	INTERP_VAL		*psVal;

	if (psCurrChunk->psPrev == NULL && currEntry == 0)
	{
		ASSERT( FALSE, "stackGetTop: not enough entries on stack" );
		return FALSE;
	}

	if (currEntry == 0)
	{
		// Value is on the previous chunk
		psChunk = psCurrChunk->psPrev;
		psVal = psChunk->aVals + psChunk->size - 1;
	}
	else
	{
		psVal = psCurrChunk->aVals + currEntry - 1;
	}

	// The data could be any type but this will copy it over
	*pType = (UDWORD)psVal->type;
	*pData = (UDWORD)psVal->v.ival;

	return TRUE;
}*/
