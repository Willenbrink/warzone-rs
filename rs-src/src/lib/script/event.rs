use ::libc;
extern "C" {
    /* Write formatted output to S.  */
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
    #[no_mangle]
    fn heapCreate(ppsHeap: *mut *mut OBJ_HEAP, size: UDWORD, init: UDWORD,
                  ext: UDWORD) -> BOOL;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Destroy a heap and release all the memory associated with it */
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    // TRUE if the interpreter is currently running
    #[no_mangle]
    fn interpProcessorActive() -> BOOL;
    /* The table of callback triggers */
    #[no_mangle]
    static mut asScrCallbackTab: *mut CALLBACK_SYMBOL;
    #[no_mangle]
    fn stackPopType(psVal: *mut INTERP_VAL) -> BOOL;
    #[no_mangle]
    fn interpRunScript(psContext: *mut SCRIPT_CONTEXT,
                       runType: INTERP_RUNTYPE, index: UDWORD, offset: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn GetCallDepth() -> SDWORD;
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
}
pub type size_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
/* structure used to store the extra space allocated for the heap */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
pub type OBJ_HEAP = _obj_heap;
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
// Extension memory for the heap
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
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
pub type SCRIPT_FUNC = Option<unsafe extern "C" fn() -> BOOL>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _callback_symbol {
    pub pIdent: *mut STRING,
    pub type_0: TRIGGER_TYPE,
    pub pFunc: SCRIPT_FUNC,
    pub numParams: UDWORD,
    pub aParams: [INTERP_TYPE; 20],
}
// Run trigger code
// Run event code
/* The type for a callback trigger symbol */
pub type CALLBACK_SYMBOL = _callback_symbol;
// Callback identifier
// user defined callback id >= TR_CALLBACKSTART
// Pointer to the instinct function
// Number of parameters to the function
// List of parameter types
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
// The Event initialisation data
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _event_init {
    pub valInit: UWORD,
    pub valExt: UWORD,
    pub trigInit: UWORD,
    pub trigExt: UWORD,
    pub contInit: UWORD,
    pub contExt: UWORD,
}
pub type EVENT_INIT = _event_init;
// context chunk init values
/*
 * A currently active trigger.
 * If the type of the triggger == TR_PAUSE, the trigger number stored is the
 * index of the trigger to replace this one when the event restarts
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _active_trigger {
    pub testTime: UDWORD,
    pub psContext: *mut SCRIPT_CONTEXT,
    pub type_0: SWORD,
    pub trigger: SWORD,
    pub event: UWORD,
    pub offset: UWORD,
    pub psNext: *mut _active_trigger,
}
pub type ACTIVE_TRIGGER = _active_trigger;
// ID numbers for each user type
pub type _scr_user_types = libc::c_uint;
// maximum possible type - should always be last
// NULL stats
pub const ST_MAXTYPE: _scr_user_types = 34;
//used so we can check for NULL templates
pub const ST_POINTER_S: _scr_user_types = 33;
//used so we can check for NULL objects etc
pub const ST_POINTER_T: _scr_user_types = 32;
// A research topic
//private types for game code - not for use in script
pub const ST_POINTER_O: _scr_user_types = 31;
// A group of droids
pub const ST_RESEARCH: _scr_user_types = 30;
// The name of a game level
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
// text string for display messages in tutorial
pub const ST_SOUND: _scr_user_types = 27;
// ID of a droid
pub const ST_TEXTSTRING: _scr_user_types = 26;
// feature stat type
pub const ST_DROIDID: _scr_user_types = 25;
// structure stat type
pub const ST_FEATURESTAT: _scr_user_types = 24;
/* A structure ID number (don't really 
											   need this since just a number?)*/
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
// Template object
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
// Component types
pub const ST_PROPULSION: _scr_user_types = 14;
// General component
pub const ST_BODY: _scr_user_types = 13;
// General stats type
pub const ST_COMPONENT: _scr_user_types = 12;
// Feature object
pub const ST_BASESTATS: _scr_user_types = 11;
// Structure object
pub const ST_FEATURE: _scr_user_types = 10;
// Droid object
pub const ST_STRUCTURE: _scr_user_types = 9;
// Base object
pub const ST_DROID: _scr_user_types = 8;
// Intelligence message ?? (6)
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
pub type VAL_RELEASE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut INTERP_VAL) -> ()>;
/*
 * Script.h
 *
 * Interface to the script library
 */
/* Whether to include debug info when compiling */
// Generate debug info
// Do not generate debug info
// If this is defined we save out the compiled scripts
// Initialise the script library
// Shutdown the script library
/* **********************************************************************************
 *
 * Compiler setup functions
 */
/* Set the type table */
/* Set the function table */
/* Set the external variable table */
/* Set the object variable table */
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
pub type VAL_CREATE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut INTERP_VAL) -> BOOL>;
pub type _context_release = libc::c_uint;
pub const CR_NORELEASE: _context_release = 1;
pub const CR_RELEASE: _context_release = 0;
pub type CONTEXT_RELEASE = _context_release;
/*
 * Event.c
 *
 * The event management system.
 */
// event tracing printf's
// display tested triggers
//#define DEBUG_GROUP1
// array to store release functions
static mut asCreateFuncs: *mut VAL_CREATE_FUNC =
    0 as *const VAL_CREATE_FUNC as *mut VAL_CREATE_FUNC;
static mut asReleaseFuncs: *mut VAL_RELEASE_FUNC =
    0 as *const VAL_RELEASE_FUNC as *mut VAL_RELEASE_FUNC;
static mut numFuncs: SDWORD = 0;
// Heap for value chunks
static mut psValHeap: *mut OBJ_HEAP = 0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// Heap for active triggers
static mut psTrigHeap: *mut OBJ_HEAP = 0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// Heap for contexts
static mut psContHeap: *mut OBJ_HEAP = 0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// The list of currently active triggers
#[no_mangle]
pub static mut psTrigList: *mut ACTIVE_TRIGGER =
    0 as *const ACTIVE_TRIGGER as *mut ACTIVE_TRIGGER;
// The list of callback triggers
#[no_mangle]
pub static mut psCallbackList: *mut ACTIVE_TRIGGER =
    0 as *const ACTIVE_TRIGGER as *mut ACTIVE_TRIGGER;
// The new triggers added this loop
static mut psAddedTriggers: *mut ACTIVE_TRIGGER =
    0 as *const ACTIVE_TRIGGER as *mut ACTIVE_TRIGGER;
// The trigger which is currently firing
static mut psFiringTrigger: *mut ACTIVE_TRIGGER =
    0 as *const ACTIVE_TRIGGER as *mut ACTIVE_TRIGGER;
static mut triggerChanged: BOOL = 0;
static mut updateTime: UDWORD = 0;
// The currently allocated contexts
#[no_mangle]
pub static mut psContList: *mut SCRIPT_CONTEXT =
    0 as *const SCRIPT_CONTEXT as *mut SCRIPT_CONTEXT;
// The current event trace level
static mut eventTraceLevel: SDWORD = 3 as libc::c_int;
//resets the event timer - updateTime
#[no_mangle]
pub unsafe extern "C" fn eventTimeReset(mut initTime: UDWORD) {
    updateTime = initTime;
}
/* Initialise the event system */
#[no_mangle]
pub unsafe extern "C" fn eventInitialise(mut psInit: *mut EVENT_INIT)
 -> BOOL {
    // Create the value heap
    if heapCreate(&mut psValHeap,
                  ::std::mem::size_of::<VAL_CHUNK>() as libc::c_ulong,
                  (*psInit).valInit as UDWORD, (*psInit).valExt as UDWORD) ==
           0 {
        debug(LOG_ERROR,
              b"eventInitialise: HEAP_CREATE failed for values\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    // Create the trigger heap
    if heapCreate(&mut psTrigHeap,
                  ::std::mem::size_of::<ACTIVE_TRIGGER>() as libc::c_ulong,
                  (*psInit).trigInit as UDWORD, (*psInit).trigExt as UDWORD)
           == 0 {
        debug(LOG_ERROR,
              b"eventInitialise: HEAP_CREATE failed for triggers\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    // Create the context heap
    if heapCreate(&mut psContHeap,
                  ::std::mem::size_of::<SCRIPT_CONTEXT>() as libc::c_ulong,
                  (*psInit).contInit as UDWORD, (*psInit).contExt as UDWORD)
           == 0 {
        debug(LOG_ERROR,
              b"eventInitialise: HEAP_CREATE failed for context\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    psTrigList = 0 as *mut ACTIVE_TRIGGER;
    psCallbackList = 0 as *mut ACTIVE_TRIGGER;
    psContList = 0 as *mut SCRIPT_CONTEXT;
    eventTraceLevel = 0 as libc::c_int;
    asCreateFuncs = 0 as *mut VAL_CREATE_FUNC;
    asReleaseFuncs = 0 as *mut VAL_RELEASE_FUNC;
    numFuncs = 0 as libc::c_int;
    return 1 as libc::c_int;
}
// reset the event system
#[no_mangle]
pub unsafe extern "C" fn eventReset() {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    // Free any active triggers and their context's
    while !psTrigList.is_null() {
        psCurr = psTrigList;
        psTrigList = (*psTrigList).psNext;
        eventRemoveContext((*psCurr).psContext);
        heapFree(psTrigHeap, psCurr as *mut libc::c_void);
    }
    // Free any active callback triggers and their context's
    while !psCallbackList.is_null() {
        psCurr = psCallbackList;
        psCallbackList = (*psCallbackList).psNext;
        eventRemoveContext((*psCurr).psContext);
        heapFree(psTrigHeap, psCurr as *mut libc::c_void);
    }
    // Now free any context's that are left
    while !psContList.is_null() { eventRemoveContext(psContList); };
}
// Shutdown the event system
#[no_mangle]
pub unsafe extern "C" fn eventShutDown() {
    eventReset();
    heapDestroy(psValHeap);
    heapDestroy(psTrigHeap);
    heapDestroy(psContHeap);
    if !asCreateFuncs.is_null() {
        memFreeRelease(asCreateFuncs as *mut libc::c_void);
        asCreateFuncs = 0 as *mut VAL_CREATE_FUNC
    }
    if !asReleaseFuncs.is_null() {
        memFreeRelease(asReleaseFuncs as *mut libc::c_void);
        asReleaseFuncs = 0 as *mut VAL_RELEASE_FUNC
    };
}
// get the trigger id string
#[no_mangle]
pub unsafe extern "C" fn eventGetTriggerID(mut psCode: *mut SCRIPT_CODE,
                                           mut trigger: SDWORD)
 -> *mut STRING {
    let mut pID: *mut STRING = 0 as *mut STRING;
    let mut pTrigType: *mut STRING = 0 as *mut STRING;
    static mut aIDNum: [STRING; 255] = [0; 255];
    let mut i: SDWORD = 0;
    let mut type_0: TRIGGER_TYPE = TR_INIT;
    if trigger >= 0 as libc::c_int &&
           trigger < (*psCode).numTriggers as libc::c_int {
        type_0 =
            (*(*psCode).psTriggerData.offset(trigger as isize)).type_0 as
                TRIGGER_TYPE
    } else {
        return b"INACTIVE\x00" as *const u8 as *const libc::c_char as
                   *mut STRING
    }
    pTrigType =
        b"UNKNOWN\x00" as *const u8 as *const libc::c_char as *mut STRING;
    match type_0 as libc::c_uint {
        0 => {
            pTrigType =
                b"INIT\x00" as *const u8 as *const libc::c_char as *mut STRING
        }
        1 => {
            pTrigType =
                b"CODE\x00" as *const u8 as *const libc::c_char as *mut STRING
        }
        2 => {
            pTrigType =
                b"WAIT\x00" as *const u8 as *const libc::c_char as *mut STRING
        }
        3 => {
            pTrigType =
                b"EVERY\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        4 => {
            pTrigType =
                b"PAUSE\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        _ => {
            if !asScrCallbackTab.is_null() {
                pTrigType =
                    (*asScrCallbackTab.offset((type_0 as
                                                   libc::c_uint).wrapping_sub(TR_CALLBACKSTART
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize)).pIdent
            }
        }
    }
    if (*psCode).psDebug.is_null() ||
           type_0 as libc::c_uint != TR_CODE as libc::c_int as libc::c_uint {
        sprintf(aIDNum.as_mut_ptr(),
                b"%d (%s)\x00" as *const u8 as *const libc::c_char, trigger,
                pTrigType);
    } else {
        pID =
            b"NOT FOUND\x00" as *const u8 as *const libc::c_char as
                *mut STRING;
        i = 0 as libc::c_int;
        while i < (*psCode).debugEntries as libc::c_int {
            if (*(*psCode).psDebug.offset(i as isize)).offset ==
                   *(*psCode).pTriggerTab.offset(trigger as isize) as
                       libc::c_uint {
                pID = (*(*psCode).psDebug.offset(i as isize)).pLabel;
                break ;
            } else { i += 1 }
        }
        sprintf(aIDNum.as_mut_ptr(),
                b"%s (%s)\x00" as *const u8 as *const libc::c_char, pID,
                pTrigType);
    }
    return aIDNum.as_mut_ptr();
}
// get the event id string
#[no_mangle]
pub unsafe extern "C" fn eventGetEventID(mut psCode: *mut SCRIPT_CODE,
                                         mut event: SDWORD) -> *mut STRING {
    let mut pID: *mut STRING = 0 as *mut STRING;
    static mut aIDNum: [STRING; 255] = [0; 255];
    let mut i: SDWORD = 0;
    if (*psCode).psDebug.is_null() || event < 0 as libc::c_int ||
           event > (*psCode).numEvents as libc::c_int {
        sprintf(aIDNum.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char, event);
        return aIDNum.as_mut_ptr()
    }
    pID = b"NOT FOUND\x00" as *const u8 as *const libc::c_char as *mut STRING;
    i = 0 as libc::c_int;
    while i < (*psCode).debugEntries as libc::c_int {
        if (*(*psCode).psDebug.offset(i as isize)).offset ==
               *(*psCode).pEventTab.offset(event as isize) as libc::c_uint {
            pID = (*(*psCode).psDebug.offset(i as isize)).pLabel;
            break ;
        } else { i += 1 }
    }
    return pID;
}
// Print out all the info available about a trigger
#[no_mangle]
pub unsafe extern "C" fn eventPrintTriggerInfo(mut psTrigger:
                                                   *mut ACTIVE_TRIGGER) {
    let mut psCode: *mut SCRIPT_CODE = (*(*psTrigger).psContext).psCode;
    //	BOOL		debugInfo = psCode->psDebug != NULL;
    let mut pTrigLab: *mut STRING = 0 as *mut STRING;
    let mut pEventLab: *mut STRING = 0 as *mut STRING;
    // find the debug info for the trigger
    pTrigLab = eventGetTriggerID(psCode, (*psTrigger).trigger as SDWORD);
    // find the debug info for the event
    pEventLab = eventGetEventID(psCode, (*psTrigger).event as SDWORD);
    debug(LOG_NEVER,
          b"trigger %s at %d -> %s\x00" as *const u8 as *const libc::c_char,
          pTrigLab, (*psTrigger).testTime, pEventLab);
    if (*psTrigger).offset as libc::c_int != 0 as libc::c_int {
        debug(LOG_NEVER, b" %d\x00" as *const u8 as *const libc::c_char,
              (*psTrigger).offset as libc::c_int);
    };
}
// Initialise the create/release function array - specify the maximum value type
#[no_mangle]
pub unsafe extern "C" fn eventInitValueFuncs(mut maxType: SDWORD) -> BOOL {
    if !asReleaseFuncs.is_null() {
        //<NEW> 13.05.05
        debug(LOG_ERROR,
              b"eventInitValueFuncs: array already initialised\x00" as
                  *const u8 as *const libc::c_char);
    }
    if asReleaseFuncs.is_null() {
    } else {
        debug(LOG_ERROR,
              b"eventInitValueFuncs: array already initialised\x00" as
                  *const u8 as *const libc::c_char);
    };
    if asReleaseFuncs.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              312 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"eventInitValueFuncs\x00")).as_ptr(),
              b"asReleaseFuncs == NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    asCreateFuncs =
        memMallocRelease((::std::mem::size_of::<VAL_CREATE_FUNC>() as
                              libc::c_ulong).wrapping_mul(maxType as
                                                              libc::c_uint))
            as *mut VAL_CREATE_FUNC;
    if asCreateFuncs.is_null() {
        debug(LOG_ERROR,
              b"eventInitValueFuncs: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    asReleaseFuncs =
        memMallocRelease((::std::mem::size_of::<VAL_RELEASE_FUNC>() as
                              libc::c_ulong).wrapping_mul(maxType as
                                                              libc::c_uint))
            as *mut VAL_RELEASE_FUNC;
    if asReleaseFuncs.is_null() {
        debug(LOG_ERROR,
              b"eventInitValueFuncs: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    memset(asCreateFuncs as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<VAL_CREATE_FUNC>() as
                libc::c_ulong).wrapping_mul(maxType as libc::c_uint));
    memset(asReleaseFuncs as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<VAL_RELEASE_FUNC>() as
                libc::c_ulong).wrapping_mul(maxType as libc::c_uint));
    numFuncs = maxType;
    return 1 as libc::c_int;
}
// Add a new value create function
#[no_mangle]
pub unsafe extern "C" fn eventAddValueCreate(mut type_0: INTERP_TYPE,
                                             mut create: VAL_CREATE_FUNC)
 -> BOOL {
    if type_0 as libc::c_uint >= numFuncs as libc::c_uint {
        debug(LOG_ERROR,
              b"eventAddValueCreate: type out of range\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    let ref mut fresh0 = *asCreateFuncs.offset(type_0 as isize);
    *fresh0 = create;
    return 1 as libc::c_int;
}
// Add a new value release function
#[no_mangle]
pub unsafe extern "C" fn eventAddValueRelease(mut type_0: INTERP_TYPE,
                                              mut release: VAL_RELEASE_FUNC)
 -> BOOL {
    if type_0 as libc::c_uint >= numFuncs as libc::c_uint {
        debug(LOG_ERROR,
              b"eventAddValueRelease: type out of range\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    let ref mut fresh1 = *asReleaseFuncs.offset(type_0 as isize);
    *fresh1 = release;
    return 1 as libc::c_int;
}
// Create a new context for a script
#[no_mangle]
pub unsafe extern "C" fn eventNewContext(mut psCode: *mut SCRIPT_CODE,
                                         mut release: CONTEXT_RELEASE,
                                         mut ppsContext:
                                             *mut *mut SCRIPT_CONTEXT)
 -> BOOL {
    let mut psContext: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut val: SDWORD = 0;
    let mut storeIndex: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut arrayNum: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut arraySize: SDWORD = 0;
    let mut psNewChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut psNextChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"eventNewContext: Invalid code pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              373 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"eventNewContext\x00")).as_ptr(),
              b"PTRVALID(psCode, sizeof(SCRIPT_CODE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Get a new context
    if heapAlloc(psContHeap,
                 &mut psContext as *mut *mut SCRIPT_CONTEXT as
                     *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        debug(LOG_ERROR,
              b"eventNewContext: HEAP_ALLOC failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    // Initialise the context
    (*psContext).psCode = psCode; // only used by the save game
    (*psContext).triggerCount = 0 as libc::c_int;
    (*psContext).release =
        if release as libc::c_uint ==
               CR_RELEASE as libc::c_int as libc::c_uint {
            1 as libc::c_int
        } else { 0 as libc::c_int } as SWORD;
    (*psContext).psGlobals = 0 as *mut VAL_CHUNK;
    (*psContext).id = -(1 as libc::c_int) as SWORD;
    val =
        ((*psCode).numGlobals as
             libc::c_uint).wrapping_add((*psCode).arraySize).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
            as SDWORD;
    arrayNum = (*psCode).numArrays as libc::c_int - 1 as libc::c_int;
    arraySize = 1 as libc::c_int;
    if (*psCode).numArrays as libc::c_int > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i <
                  (*(*psCode).psArrayInfo.offset(arrayNum as
                                                     isize)).dimensions as
                      libc::c_int {
            arraySize *=
                (*(*psCode).psArrayInfo.offset(arrayNum as
                                                   isize)).elements[i as
                                                                        usize]
                    as libc::c_int;
            i += 1
        }
    }
    //prepare local variables (initialize, store type)
	//-------------------------------
    (*psCode).ppsLocalVarVal =
        memMallocRelease((::std::mem::size_of::<*mut INTERP_VAL>() as
                              libc::c_ulong).wrapping_mul((*psCode).numEvents
                                                              as
                                                              libc::c_uint))
            as
            *mut *mut INTERP_VAL; //allocate space for array of local var arrays for each event
    debug(LOG_SCRIPT,
          b"allocated space for %d events\x00" as *const u8 as
              *const libc::c_char, (*psCode).numEvents as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*psCode).numEvents as libc::c_int {
        if *(*psCode).numLocalVars.offset(i as isize) >
               0 as libc::c_int as libc::c_uint {
            //this event has any local vars declared
            let ref mut fresh2 =
                *(*psCode).ppsLocalVarVal.offset(i as
                                                     isize); //allocate space for local vars array (for the current event)
            *fresh2 =
                memMallocRelease((::std::mem::size_of::<INTERP_VAL>() as
                                      libc::c_ulong).wrapping_mul(*(*psCode).numLocalVars.offset(i
                                                                                                     as
                                                                                                     isize)))
                    as *mut INTERP_VAL;
            debug(LOG_SCRIPT,
                  b"Event %d has %d local variables\x00" as *const u8 as
                      *const libc::c_char, i,
                  *(*psCode).numLocalVars.offset(i as isize));
            j = 0 as libc::c_int;
            while (j as libc::c_uint) <
                      *(*psCode).numLocalVars.offset(i as isize) {
                type_0 =
                    *(*(*psCode).ppsLocalVars.offset(i as
                                                         isize)).offset(j as
                                                                            isize)
                        as SDWORD;
                //debug(LOG_SCRIPT,"------");
                //debug(LOG_SCRIPT, "i=%d, j=%d, value=%d",i,j,psCode->ppsLocalVarVal[i][j].v.ival);
                (*(*(*psCode).ppsLocalVarVal.offset(i as
                                                        isize)).offset(j as
                                                                           isize)).type_0
                    =
                    type_0 as
                        INTERP_TYPE; //store (copy) var type (data used during parsing -> data used during interpreting)
                if type_0 == VAL_STRING as libc::c_int {
                    //debug(LOG_SCRIPT,"var %d's type: %d", i, type);
                    //initialize Strings and integers
                    //debug(LOG_ERROR,"eventNewContext: STRING type variables are not implemented");
                    let ref mut fresh3 =
                        (*(*(*psCode).ppsLocalVarVal.offset(i as
                                                                isize)).offset(j
                                                                                   as
                                                                                   isize)).v.sval;
                    *fresh3 =
                        memMallocRelease(255 as libc::c_int as size_t) as
                            *mut libc::c_char;
                    strcpy((*(*(*psCode).ppsLocalVarVal.offset(i as
                                                                   isize)).offset(j
                                                                                      as
                                                                                      isize)).v.sval,
                           b"\x00\x00" as *const u8 as *const libc::c_char);
                } else {
                    (*(*(*psCode).ppsLocalVarVal.offset(i as
                                                            isize)).offset(j
                                                                               as
                                                                               isize)).v.ival
                        = 0 as libc::c_int
                }
                if !asCreateFuncs.is_null() && type_0 < numFuncs &&
                       (*asCreateFuncs.offset(type_0 as isize)).is_some() {
                    if (*asCreateFuncs.offset(type_0 as
                                                  isize)).expect("non-null function pointer")(&mut *(*(*psCode).ppsLocalVarVal.offset(i
                                                                                                                                          as
                                                                                                                                          isize)).offset(j
                                                                                                                                                             as
                                                                                                                                                             isize))
                           == 0 {
                        debug(LOG_ERROR,
                              b"eventNewContext: asCreateFuncs failed for local var\x00"
                                  as *const u8 as *const libc::c_char);
                        return 0 as libc::c_int
                    }
                }
                j += 1
            }
        } else {
            //Initialize objects
            //this event has no local vars
            let ref mut fresh4 = *(*psCode).ppsLocalVarVal.offset(i as isize);
            *fresh4 = 0 as *mut INTERP_VAL
        }
        i += 1
    }
    while val >= 0 as libc::c_int {
        if heapAlloc(psValHeap,
                     &mut psNewChunk as *mut *mut VAL_CHUNK as
                         *mut libc::c_void as *mut *mut libc::c_void) == 0 {
            psNewChunk = (*psContext).psGlobals;
            while !psNewChunk.is_null() {
                psNextChunk = (*psNewChunk).psNext;
                heapFree(psValHeap, psNewChunk as *mut libc::c_void);
                psNewChunk = psNextChunk
            }
            heapFree(psContHeap, psContext as *mut libc::c_void);
            return 0 as libc::c_int
        }
        // Set the value types
        storeIndex = val % 20 as libc::c_int;
        while storeIndex >= 0 as libc::c_int {
            if val >= (*psCode).numGlobals as libc::c_int {
                type_0 =
                    (*(*psCode).psArrayInfo.offset(arrayNum as isize)).type_0
                        as SDWORD
            } else {
                type_0 = *(*psCode).pGlobals.offset(val as isize) as SDWORD
            }
            (*psNewChunk).asVals[storeIndex as usize].type_0 =
                type_0 as INTERP_TYPE;
            //initialize Strings
            if type_0 == VAL_STRING as libc::c_int {
                (*psNewChunk).asVals[storeIndex as usize].v.sval =
                    memMallocRelease(255 as libc::c_int as size_t) as
                        *mut libc::c_char;
                strcpy((*psNewChunk).asVals[storeIndex as usize].v.sval,
                       b"\x00\x00" as *const u8 as *const libc::c_char);
            } else {
                (*psNewChunk).asVals[storeIndex as usize].v.ival =
                    0 as libc::c_int
            }
            //initialize objects
            if !asCreateFuncs.is_null() && type_0 < numFuncs &&
                   (*asCreateFuncs.offset(type_0 as isize)).is_some() {
                if (*asCreateFuncs.offset(type_0 as
                                              isize)).expect("non-null function pointer")((*psNewChunk).asVals.as_mut_ptr().offset(storeIndex
                                                                                                                                       as
                                                                                                                                       isize))
                       == 0 {
                    heapFree(psValHeap, psNewChunk as *mut libc::c_void);
                    psNewChunk = (*psContext).psGlobals;
                    while !psNewChunk.is_null() {
                        psNextChunk = (*psNewChunk).psNext;
                        heapFree(psValHeap, psNewChunk as *mut libc::c_void);
                        psNewChunk = psNextChunk
                    }
                    heapFree(psContHeap, psContext as *mut libc::c_void);
                    return 0 as libc::c_int
                }
            }
            storeIndex -= 1 as libc::c_int;
            val -= 1 as libc::c_int;
            arraySize -= 1 as libc::c_int;
            if arraySize <= 0 as libc::c_int {
                // finished this array
                arrayNum -= 1 as libc::c_int;
                if arrayNum >= 0 as libc::c_int {
                    // calculate the next array size
                    arraySize = 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i <
                              (*(*psCode).psArrayInfo.offset(arrayNum as
                                                                 isize)).dimensions
                                  as libc::c_int {
                        arraySize *=
                            (*(*psCode).psArrayInfo.offset(arrayNum as
                                                               isize)).elements[i
                                                                                    as
                                                                                    usize]
                                as libc::c_int;
                        i += 1
                    }
                }
            }
        }
        (*psNewChunk).psNext = (*psContext).psGlobals;
        (*psContext).psGlobals = psNewChunk
    }
    (*psContext).psNext = psContList;
    psContList = psContext;
    *ppsContext = psContext;
    return 1 as libc::c_int;
}
// Copy a context, including variable values
#[no_mangle]
pub unsafe extern "C" fn eventCopyContext(mut psContext: *mut SCRIPT_CONTEXT,
                                          mut ppsNew:
                                              *mut *mut SCRIPT_CONTEXT)
 -> BOOL {
    let mut psNew: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut val: SDWORD = 0;
    let mut psChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut psOChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"eventCopyContext: Invalid context pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              550 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"eventCopyContext\x00")).as_ptr(),
              b"PTRVALID(psContext, sizeof(SCRIPT_CONTEXT))\x00" as *const u8
                  as *const libc::c_char);
    };
    // Get a new context
    if eventNewContext((*psContext).psCode,
                       (*psContext).release as CONTEXT_RELEASE, &mut psNew) ==
           0 {
        return 0 as libc::c_int
    }
    // Now copy the values over
    psChunk = (*psNew).psGlobals;
    psOChunk = (*psContext).psGlobals;
    while !psChunk.is_null() {
        val = 0 as libc::c_int;
        while val < 20 as libc::c_int {
            (*psChunk).asVals[val as usize].v.ival =
                (*psOChunk).asVals[val as usize].v.ival;
            val += 1
        }
        psChunk = (*psChunk).psNext;
        psOChunk = (*psOChunk).psNext
    }
    *ppsNew = psNew;
    return 1 as libc::c_int;
}
// Add a new object to the trigger system
// Time is the application time at which all the triggers are to be started
#[no_mangle]
pub unsafe extern "C" fn eventRunContext(mut psContext: *mut SCRIPT_CONTEXT,
                                         mut time: UDWORD) -> BOOL {
    let mut event: SDWORD = 0;
    let mut psTrigger: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psData: *mut TRIGGER_DATA = 0 as *mut TRIGGER_DATA;
    let mut psCode: *mut SCRIPT_CODE = 0 as *mut SCRIPT_CODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"eventNewObject: Invalid context pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              587 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"eventRunContext\x00")).as_ptr(),
              b"PTRVALID(psContext, sizeof(SCRIPT_CONTEXT))\x00" as *const u8
                  as *const libc::c_char);
    };
    // Now setup all the triggers
    (*psContext).triggerCount = 0 as libc::c_int;
    psCode = (*psContext).psCode;
    event = 0 as libc::c_int;
    while event < (*psCode).numEvents as libc::c_int {
        if *(*psCode).pEventLinks.offset(event as isize) as libc::c_int >=
               0 as libc::c_int {
            // See if this is an init event
            psData =
                (*psCode).psTriggerData.offset(*(*psCode).pEventLinks.offset(event
                                                                                 as
                                                                                 isize)
                                                   as libc::c_int as isize);
            if (*psData).type_0 as libc::c_int == TR_INIT as libc::c_int {
                if interpRunScript(psContext, IRT_EVENT, event as UDWORD,
                                   0 as libc::c_int as UDWORD) == 0 {
                    return 0 as libc::c_int
                }
            } else {
                if eventInitTrigger(&mut psTrigger, psContext,
                                    event as UDWORD,
                                    *(*psCode).pEventLinks.offset(event as
                                                                      isize)
                                        as SDWORD, time) == 0 {
                    return 0 as libc::c_int
                }
                eventAddTrigger(psTrigger);
            }
        }
        event += 1
    }
    return 1 as libc::c_int;
}
// Remove an object from the event system
#[no_mangle]
pub unsafe extern "C" fn eventRemoveContext(mut psContext:
                                                *mut SCRIPT_CONTEXT) {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psPrev: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psNext: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psCChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut psNChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut psCCont: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut psPCont: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut i: SDWORD = 0;
    let mut chunkStart: SDWORD = 0;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    // Get rid of all it's triggers
    while !psTrigList.is_null() && (*psTrigList).psContext == psContext {
        psNext = (*psTrigList).psNext;
        eventFreeTrigger(psTrigList);
        psTrigList = psNext
    }
    psCurr = psTrigList;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        if (*psCurr).psContext == psContext {
            eventFreeTrigger(psCurr);
            (*psPrev).psNext = psNext
        } else { psPrev = psCurr }
        psCurr = psNext
    }
    // Get rid of all it's callback triggers
    while !psCallbackList.is_null() &&
              (*psCallbackList).psContext == psContext {
        psNext = (*psCallbackList).psNext;
        eventFreeTrigger(psCallbackList);
        psCallbackList = psNext
    }
    psCurr = psCallbackList;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        if (*psCurr).psContext == psContext {
            eventFreeTrigger(psCurr);
            (*psPrev).psNext = psNext
        } else { psPrev = psCurr }
        psCurr = psNext
    }
    // Call the release function for all the values
    if !asReleaseFuncs.is_null() {
        psCChunk = (*psContext).psGlobals;
        chunkStart = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*(*psContext).psCode).numGlobals as libc::c_int {
            if i - chunkStart >= 20 as libc::c_int {
                chunkStart += 20 as libc::c_int;
                psCChunk = (*psCChunk).psNext;
                if !psCChunk.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"eventRemoveContext: not enough value chunks\x00"
                              as *const u8 as *const libc::c_char);
                };
                if !psCChunk.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"event.c\x00" as *const u8 as *const libc::c_char,
                          685 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"eventRemoveContext\x00")).as_ptr(),
                          b"psCChunk != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
            }
            psVal =
                (*psCChunk).asVals.as_mut_ptr().offset((i - chunkStart) as
                                                           isize);
            if ((*psVal).type_0 as libc::c_uint) < numFuncs as libc::c_uint &&
                   (*asReleaseFuncs.offset((*psVal).type_0 as
                                               isize)).is_some() {
                (*asReleaseFuncs.offset((*psVal).type_0 as
                                            isize)).expect("non-null function pointer")(psVal);
            }
            i += 1
        }
    }
    // Free it's variables
    psCChunk = (*psContext).psGlobals;
    while !psCChunk.is_null() {
        psNChunk = (*psCChunk).psNext;
        heapFree(psValHeap, psCChunk as *mut libc::c_void);
        psCChunk = psNChunk
    }
    // Remove it from the context list
    if psContext == psContList {
        psCCont = psContList;
        psContList = (*psContList).psNext;
        heapFree(psContHeap, psCCont as *mut libc::c_void);
    } else {
        psCCont = psContList;
        while !psCCont.is_null() && psCCont != psContext {
            psPCont = psCCont;
            psCCont = (*psCCont).psNext
        }
        if !psCCont.is_null() {
            (*psPCont).psNext = (*psCCont).psNext;
            heapFree(psContHeap, psContext as *mut libc::c_void);
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"eventRemoveContext: context not found\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"event.c\x00" as *const u8 as *const libc::c_char,
                      723 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"eventRemoveContext\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
// Get the value pointer for a variable index
#[no_mangle]
pub unsafe extern "C" fn eventGetContextVal(mut psContext:
                                                *mut SCRIPT_CONTEXT,
                                            mut index: UDWORD,
                                            mut ppsVal: *mut *mut INTERP_VAL)
 -> BOOL {
    let mut psChunk: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    // Find the chunk for the variable
    psChunk = (*psContext).psGlobals;
    while !psChunk.is_null() && index >= 20 as libc::c_int as libc::c_uint {
        index =
            (index as
                 libc::c_uint).wrapping_sub(20 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        psChunk = (*psChunk).psNext
    }
    if psChunk.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"eventGetContextVal: Variable not found\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"event.c\x00" as *const u8 as *const libc::c_char,
                  742 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"eventGetContextVal\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    *ppsVal = (*psChunk).asVals.as_mut_ptr().offset(index as isize);
    return 1 as libc::c_int;
}
// Set a global variable value for a context
#[no_mangle]
pub unsafe extern "C" fn eventSetContextVar(mut psContext:
                                                *mut SCRIPT_CONTEXT,
                                            mut index: UDWORD,
                                            mut type_0: INTERP_TYPE,
                                            mut data: UDWORD) -> BOOL {
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    if eventGetContextVal(psContext, index, &mut psVal) == 0 {
        return 0 as libc::c_int
    }
    if (*psVal).type_0 as libc::c_uint != type_0 as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"eventSetContextVar: Variable type mismatch\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"event.c\x00" as *const u8 as *const libc::c_char,
                  764 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"eventSetContextVar\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // Store the data
    (*psVal).v.ival = data as SDWORD;
    return 1 as libc::c_int;
}
// Add a trigger to the list in order
// Add a trigger to the list in order
unsafe extern "C" fn eventAddTrigger(mut psTrigger: *mut ACTIVE_TRIGGER) {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psPrev: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut testTime: UDWORD = (*psTrigger).testTime;
    if (*psTrigger).type_0 as libc::c_int >= TR_CALLBACKSTART as libc::c_int {
        // Add this to the callback trigger list
        if psCallbackList.is_null() {
            (*psTrigger).psNext = 0 as *mut _active_trigger;
            psCallbackList = psTrigger
        } else if (*psTrigger).type_0 as libc::c_int <=
                      (*psCallbackList).type_0 as libc::c_int {
            (*psTrigger).psNext = psCallbackList;
            psCallbackList = psTrigger
        } else {
            psCurr = psCallbackList;
            while !psCurr.is_null() &&
                      ((*psCurr).type_0 as libc::c_int) <
                          (*psTrigger).type_0 as libc::c_int {
                psPrev = psCurr;
                psCurr = (*psCurr).psNext
            }
            (*psTrigger).psNext = (*psPrev).psNext;
            (*psPrev).psNext = psTrigger
        }
    } else if psTrigList.is_null() {
        (*psTrigger).psNext = 0 as *mut _active_trigger;
        psTrigList = psTrigger
    } else if testTime <= (*psTrigList).testTime {
        (*psTrigger).psNext = psTrigList;
        psTrigList = psTrigger
    } else {
        psCurr = psTrigList;
        while !psCurr.is_null() && (*psCurr).testTime < testTime {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        (*psTrigger).psNext = (*psPrev).psNext;
        (*psPrev).psNext = psTrigger
    };
}
// print info on trigger
// event tracing printf
// Initialise a trigger
// Initialise a trigger
unsafe extern "C" fn eventInitTrigger(mut ppsTrigger:
                                          *mut *mut ACTIVE_TRIGGER,
                                      mut psContext: *mut SCRIPT_CONTEXT,
                                      mut event: UDWORD, mut trigger: SDWORD,
                                      mut currTime: UDWORD) -> BOOL {
    let mut psNewTrig: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psTrigData: *mut TRIGGER_DATA = 0 as *mut TRIGGER_DATA;
    let mut testTime: UDWORD = 0;
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"eventAddTrigger: Event out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              836 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"eventInitTrigger\x00")).as_ptr(),
              b"event < psContext->psCode->numEvents\x00" as *const u8 as
                  *const libc::c_char);
    };
    if trigger < (*(*psContext).psCode).numTriggers as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"eventAddTrigger: Trigger out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if trigger < (*(*psContext).psCode).numTriggers as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              838 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"eventInitTrigger\x00")).as_ptr(),
              b"trigger < psContext->psCode->numTriggers\x00" as *const u8 as
                  *const libc::c_char);
    };
    if trigger == -(1 as libc::c_int) { return 0 as libc::c_int }
    // Get a trigger object
    if heapAlloc(psTrigHeap,
                 &mut psNewTrig as *mut *mut ACTIVE_TRIGGER as
                     *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    // Initialise the trigger
    (*psNewTrig).psContext = psContext;
    (*psContext).triggerCount += 1 as libc::c_int;
    psTrigData =
        (*(*psContext).psCode).psTriggerData.offset(trigger as isize);
    testTime = currTime.wrapping_add((*psTrigData).time);
    (*psNewTrig).testTime = testTime;
    (*psNewTrig).trigger = trigger as SWORD;
    (*psNewTrig).type_0 = (*psTrigData).type_0 as SWORD;
    (*psNewTrig).event = event as UWORD;
    (*psNewTrig).offset = 0 as libc::c_int as UWORD;
    *ppsTrigger = psNewTrig;
    return 1 as libc::c_int;
}
// Load a trigger into the system from a save game
#[no_mangle]
pub unsafe extern "C" fn eventLoadTrigger(mut time: UDWORD,
                                          mut psContext: *mut SCRIPT_CONTEXT,
                                          mut type_0: SDWORD,
                                          mut trigger: SDWORD,
                                          mut event: UDWORD,
                                          mut offset: UDWORD) -> BOOL {
    let mut psNewTrig: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psTrigData: *mut TRIGGER_DATA = 0 as *mut TRIGGER_DATA;
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"eventLoadTrigger: Event out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              874 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"eventLoadTrigger\x00")).as_ptr(),
              b"event < psContext->psCode->numEvents\x00" as *const u8 as
                  *const libc::c_char);
    };
    if trigger < (*(*psContext).psCode).numTriggers as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"eventLoadTrigger: Trigger out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if trigger < (*(*psContext).psCode).numTriggers as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              876 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"eventLoadTrigger\x00")).as_ptr(),
              b"trigger < psContext->psCode->numTriggers\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Get a trigger object
    if heapAlloc(psTrigHeap,
                 &mut psNewTrig as *mut *mut ACTIVE_TRIGGER as
                     *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        debug(LOG_ERROR,
              b"eventLoadTrigger: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // Initialise the trigger
    (*psNewTrig).psContext = psContext;
    (*psContext).triggerCount += 1 as libc::c_int;
    psTrigData =
        (*(*psContext).psCode).psTriggerData.offset(trigger as isize);
    (*psNewTrig).testTime = time;
    (*psNewTrig).trigger = trigger as SWORD;
    (*psNewTrig).type_0 = type_0 as SWORD;
    (*psNewTrig).event = event as UWORD;
    (*psNewTrig).offset = offset as UWORD;
    eventAddTrigger(psNewTrig);
    return 1 as libc::c_int;
}
// add a TR_PAUSE trigger to the event system.
#[no_mangle]
pub unsafe extern "C" fn eventAddPauseTrigger(mut psContext:
                                                  *mut SCRIPT_CONTEXT,
                                              mut event: UDWORD,
                                              mut offset: UDWORD,
                                              mut time: UDWORD) -> BOOL {
    let mut psNewTrig: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut trigger: SDWORD = 0;
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"eventAddTrigger: Event out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if event < (*(*psContext).psCode).numEvents as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"event.c\x00" as *const u8 as *const libc::c_char,
              909 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"eventAddPauseTrigger\x00")).as_ptr(),
              b"event < psContext->psCode->numEvents\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Get a trigger object
    if heapAlloc(psTrigHeap,
                 &mut psNewTrig as *mut *mut ACTIVE_TRIGGER as
                     *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    // figure out what type of trigger will go into the system when the pause
	// finishes
    match (*psFiringTrigger).type_0 as libc::c_int {
        2 => {
            // fired by a wait trigger, no trigger to replace it
            trigger = -(1 as libc::c_int)
        }
        4 => {
            // fired by a pause trigger, use the trigger stored with it
            trigger = (*psFiringTrigger).trigger as SDWORD
        }
        _ => {
            // just store the trigger that fired
            trigger = (*psFiringTrigger).trigger as SDWORD
        }
    }
    // Initialise the trigger
    (*psNewTrig).psContext = psContext;
    (*psContext).triggerCount += 1 as libc::c_int;
    (*psNewTrig).testTime = updateTime.wrapping_add(time);
    (*psNewTrig).trigger = trigger as SWORD;
    (*psNewTrig).type_0 = TR_PAUSE as libc::c_int as SWORD;
    (*psNewTrig).event = event as UWORD;
    (*psNewTrig).offset = offset as UWORD;
    // store the new trigger
    (*psNewTrig).psNext = psAddedTriggers;
    psAddedTriggers = psNewTrig;
    // tell the event system the trigger has been changed
    triggerChanged = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// Free up a trigger
// Free up a trigger
unsafe extern "C" fn eventFreeTrigger(mut psTrigger: *mut ACTIVE_TRIGGER) {
    if (*(*psTrigger).psContext).release as libc::c_int != 0 &&
           (*(*psTrigger).psContext).triggerCount <= 1 as libc::c_int {
        // Free the context as well
        eventRemoveContext((*psTrigger).psContext);
    }
    heapFree(psTrigHeap, psTrigger as *mut libc::c_void);
}
// Activate a callback trigger
#[no_mangle]
pub unsafe extern "C" fn eventFireCallbackTrigger(mut callback:
                                                      TRIGGER_TYPE) {
    let mut psPrev: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psNext: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psTrigDat: *mut TRIGGER_DATA = 0 as *mut TRIGGER_DATA;
    let mut fired: BOOL = 0;
    if interpProcessorActive() != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"eventFireCallbackTrigger: script interpreter is already running\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"event.c\x00" as *const u8 as *const libc::c_char,
                  977 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"eventFireCallbackTrigger\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    //this can be called from eventProcessTriggers and so will wipe out all the current added ones
	//psAddedTriggers = NULL;
    psPrev = 0 as *mut ACTIVE_TRIGGER;
    let mut current_block_58: u64;
    psCurr = psCallbackList;
    while !psCurr.is_null() &&
              (*psCurr).type_0 as libc::c_uint <= callback as libc::c_uint {
        psNext = (*psCurr).psNext;
        if (*psCurr).type_0 as libc::c_uint == callback as libc::c_uint {
            // see if the callback should be fired
            fired = 0 as libc::c_int;
            if (*psCurr).type_0 as libc::c_int != TR_PAUSE as libc::c_int {
                if (*psCurr).trigger as libc::c_int >= 0 as libc::c_int &&
                       ((*psCurr).trigger as libc::c_int) <
                           (*(*(*psCurr).psContext).psCode).numTriggers as
                               libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"eventFireCallbackTrigger: invalid trigger number\x00"
                              as *const u8 as *const libc::c_char);
                };
                if (*psCurr).trigger as libc::c_int >= 0 as libc::c_int &&
                       ((*psCurr).trigger as libc::c_int) <
                           (*(*(*psCurr).psContext).psCode).numTriggers as
                               libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"event.c\x00" as *const u8 as *const libc::c_char,
                          996 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 25],
                                                    &[libc::c_char; 25]>(b"eventFireCallbackTrigger\x00")).as_ptr(),
                          b"psCurr->trigger >= 0 && psCurr->trigger < psCurr->psContext->psCode->numTriggers\x00"
                              as *const u8 as *const libc::c_char);
                };
                psTrigDat =
                    (*(*(*psCurr).psContext).psCode).psTriggerData.offset((*psCurr).trigger
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
            } else { psTrigDat = 0 as *mut TRIGGER_DATA }
            if !psTrigDat.is_null() && (*psTrigDat).code as libc::c_int != 0 {
                if interpRunScript((*psCurr).psContext, IRT_TRIGGER,
                                   (*psCurr).trigger as UDWORD,
                                   0 as libc::c_int as UDWORD) == 0 {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"eventFireCallbackTrigger: trigger %s: code failed\x00"
                                  as *const u8 as *const libc::c_char,
                              eventGetTriggerID((*(*psCurr).psContext).psCode,
                                                (*psCurr).trigger as SDWORD));
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"event.c\x00" as *const u8 as
                                  *const libc::c_char, 1009 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 25],
                                                        &[libc::c_char; 25]>(b"eventFireCallbackTrigger\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    psPrev = psCurr;
                    current_block_58 = 5720623009719927633;
                } else if stackPopParams(1 as libc::c_int,
                                         VAL_BOOL as libc::c_int,
                                         &mut fired as *mut BOOL) == 0 {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"eventFireCallbackTrigger: trigger %s: code failed\x00"
                                  as *const u8 as *const libc::c_char,
                              eventGetTriggerID((*(*psCurr).psContext).psCode,
                                                (*psCurr).trigger as SDWORD));
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"event.c\x00" as *const u8 as
                                  *const libc::c_char, 1016 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 25],
                                                        &[libc::c_char; 25]>(b"eventFireCallbackTrigger\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    psPrev = psCurr;
                    current_block_58 = 5720623009719927633;
                } else { current_block_58 = 7245201122033322888; }
            } else {
                fired = 1 as libc::c_int;
                current_block_58 = 7245201122033322888;
            }
            match current_block_58 {
                5720623009719927633 => { }
                _ => {
                    // run the event
                    if fired != 0 {
                        // remove the trigger from the list
                        if psPrev.is_null() {
                            psCallbackList = (*psCallbackList).psNext
                        } else { (*psPrev).psNext = psNext }
                        triggerChanged = 0 as libc::c_int;
                        psFiringTrigger = psCurr;
                        if interpRunScript((*psCurr).psContext, IRT_EVENT,
                                           (*psCurr).event as UDWORD,
                                           (*psCurr).offset as UDWORD) == 0 {
                            // this could set triggerChanged
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"eventFireCallbackTrigger: event %s: code failed\x00"
                                          as *const u8 as *const libc::c_char,
                                      eventGetEventID((*(*psCurr).psContext).psCode,
                                                      (*psCurr).event as
                                                          SDWORD));
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"event.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1048 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 25],
                                                                &[libc::c_char; 25]>(b"eventFireCallbackTrigger\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                        }
                        if triggerChanged != 0 {
                            // don't need to add the trigger again - just free it
                            eventFreeTrigger(psCurr);
                        } else {
                            // make sure the trigger goes back into the system
                            (*psFiringTrigger).psNext = psAddedTriggers;
                            psAddedTriggers = psFiringTrigger
                        }
                    } else { psPrev = psCurr }
                }
            }
        } else { psPrev = psCurr }
        psCurr = psNext
    }
    // Now add all the new triggers
    psCurr = psAddedTriggers;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        eventAddTrigger(psCurr);
        psCurr = psNext
    }
    //clear out after added them all
    psAddedTriggers = 0 as *mut ACTIVE_TRIGGER;
}
// Run a trigger
unsafe extern "C" fn eventFireTrigger(mut psTrigger: *mut ACTIVE_TRIGGER)
 -> BOOL {
    //	TRIGGER_DATA	*psTrigData;
    let mut fired: BOOL = 0;
    let mut sResult: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    fired = 0 as libc::c_int;
    // If this is a code trigger see if it fires
    if (*psTrigger).type_0 as libc::c_int == TR_CODE as libc::c_int {
        // Run the trigger
        if interpRunScript((*psTrigger).psContext, IRT_TRIGGER,
                           (*psTrigger).trigger as UDWORD,
                           0 as libc::c_int as UDWORD) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"eventFireTrigger: trigger %s: code failed\x00" as
                          *const u8 as *const libc::c_char,
                      eventGetTriggerID((*(*psTrigger).psContext).psCode,
                                        (*psTrigger).trigger as SDWORD));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"event.c\x00" as *const u8 as *const libc::c_char,
                      1102 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"eventFireTrigger\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        // Get the result
        sResult.type_0 = VAL_BOOL;
        if stackPopType(&mut sResult) == 0 { return 0 as libc::c_int }
        fired = sResult.v.bval
    } else { fired = 1 as libc::c_int }
    // If it fired run the event
    if fired != 0 {
        if interpRunScript((*psTrigger).psContext, IRT_EVENT,
                           (*psTrigger).event as UDWORD,
                           (*psTrigger).offset as UDWORD) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"eventFireTrigger: event %s: code failed\x00" as
                          *const u8 as *const libc::c_char,
                      eventGetEventID((*(*psTrigger).psContext).psCode,
                                      (*psTrigger).event as SDWORD));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"event.c\x00" as *const u8 as *const libc::c_char,
                      1130 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"eventFireTrigger\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// Process all the currently active triggers
#[no_mangle]
pub unsafe extern "C" fn eventProcessTriggers(mut currTime: UDWORD) {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psNext: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psNew: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psData: *mut TRIGGER_DATA = 0 as *mut TRIGGER_DATA;
    // Process all the current triggers
    psAddedTriggers = 0 as *mut ACTIVE_TRIGGER;
    updateTime = currTime;
    while !psTrigList.is_null() && (*psTrigList).testTime <= currTime {
        psCurr = psTrigList;
        psTrigList = (*psTrigList).psNext;
        // Store the trigger so that I can tell if the event changes
		// the trigger assigned to it
        psFiringTrigger = psCurr;
        triggerChanged = 0 as libc::c_int;
        // Run the trigger
        if eventFireTrigger(psCurr) != 0 {
            // This might set triggerChanged
            if triggerChanged != 0 ||
                   (*psCurr).type_0 as libc::c_int == TR_WAIT as libc::c_int {
                // remove the trigger
                eventFreeTrigger(psCurr);
            } else if (*psCurr).type_0 as libc::c_int ==
                          TR_PAUSE as libc::c_int {
                // restarted a paused event - replace the old trigger
                if (*psCurr).trigger as libc::c_int != -(1 as libc::c_int) {
                    if eventInitTrigger(&mut psNew, (*psCurr).psContext,
                                        (*psCurr).event as UDWORD,
                                        (*psCurr).trigger as SDWORD,
                                        updateTime) != 0 {
                        (*psNew).psNext = psAddedTriggers;
                        psAddedTriggers = psNew
                    }
                }
                // remove the TR_PAUSE trigger
                eventFreeTrigger(psCurr);
            } else {
                // Add the trigger again
                psData =
                    (*(*(*psCurr).psContext).psCode).psTriggerData.offset((*psCurr).trigger
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              isize);
                (*psCurr).testTime = currTime.wrapping_add((*psData).time);
                (*psCurr).psNext = psAddedTriggers;
                psAddedTriggers = psCurr
            }
        }
    }
    // Now add all the new triggers
    psCurr = psAddedTriggers;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        eventAddTrigger(psCurr);
        psCurr = psNext
    }
    //clear out after added them all
    psAddedTriggers = 0 as *mut ACTIVE_TRIGGER;
}
// remove a trigger from a list
#[no_mangle]
pub unsafe extern "C" fn eventRemoveTriggerFromList(mut ppsList:
                                                        *mut *mut ACTIVE_TRIGGER,
                                                    mut psContext:
                                                        *mut SCRIPT_CONTEXT,
                                                    mut event: SDWORD,
                                                    mut pTrigger:
                                                        *mut SDWORD) {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut psPrev: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    if !(*ppsList).is_null() && (**ppsList).event as libc::c_int == event &&
           (**ppsList).psContext == psContext {
        if (**ppsList).type_0 as libc::c_int == TR_PAUSE as libc::c_int {
            // pause trigger, don't remove it,
			// just note the type for when the pause finishes
            (**ppsList).trigger = *pTrigger as SWORD;
            *pTrigger = -(1 as libc::c_int)
        } else {
            psCurr = *ppsList;
            *ppsList = (**ppsList).psNext;
            heapFree(psTrigHeap, psCurr as *mut libc::c_void);
        }
    } else {
        psCurr = *ppsList;
        while !psCurr.is_null() {
            if (*psCurr).event as libc::c_int == event &&
                   (*psCurr).psContext == psContext {
                break ;
            }
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() &&
               (*psCurr).type_0 as libc::c_int == TR_PAUSE as libc::c_int {
            // pause trigger, don't remove it,
			// just note the type for when the pause finishes
            (*psCurr).trigger = *pTrigger as SWORD;
            *pTrigger = -(1 as libc::c_int)
        } else if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            heapFree(psTrigHeap, psCurr as *mut libc::c_void);
        }
    };
}
// Change the trigger assigned to an event
// This is an instinct function that takes a VAL_EVENT and VAL_TRIGGER as parameters
// Change the trigger assigned to an event - to be called from script functions
#[no_mangle]
pub unsafe extern "C" fn eventSetTrigger() -> BOOL {
    let mut psTrigger: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut event: UDWORD = 0;
    let mut trigger: SDWORD = 0;
    let mut psContext: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    if stackPopParams(2 as libc::c_int, VAL_EVENT as libc::c_int,
                      &mut event as *mut UDWORD, VAL_TRIGGER as libc::c_int,
                      &mut trigger as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // See if this is the event that is running
    psContext = (*psFiringTrigger).psContext;
    if (*psFiringTrigger).event as libc::c_uint == event {
        triggerChanged = 1 as libc::c_int
    } else {
        // Remove any old trigger from the list
        eventRemoveTriggerFromList(&mut psTrigList, psContext,
                                   event as SDWORD, &mut trigger);
        eventRemoveTriggerFromList(&mut psCallbackList, psContext,
                                   event as SDWORD, &mut trigger);
        eventRemoveTriggerFromList(&mut psAddedTriggers, psContext,
                                   event as SDWORD, &mut trigger);
        /*		if (psTrigList &&
			psTrigList->event == event &&
			psTrigList->psContext == psContext)
		{
			if (psTrigList->type == TR_PAUSE)
			{
				// pause trigger, don't remove it,
				// just note the type for when the pause finishes
				psTrigList->trigger = (SWORD)trigger;
				trigger = -1;
			}
			else
			{
				psCurr = psTrigList;
				psTrigList = psTrigList->psNext;
				HEAP_FREE(psTrigHeap, psCurr);
			}
		}
		else
		{
			for(psCurr=psTrigList; psCurr; psCurr=psCurr->psNext)
			{
				if (psCurr->event == event &&
					psTrigList->psContext == psContext)
				{
					break;
				}
				psPrev = psCurr;
			}
			if (psCurr && psCurr->type == TR_PAUSE)
			{
				// pause trigger, don't remove it,
				// just note the type for when the pause finishes
				psCurr->trigger = (SWORD)trigger;
				trigger = -1;
			}
			else if (psCurr)
			{
				psPrev->psNext = psCurr->psNext;
				HEAP_FREE(psTrigHeap, psCurr);
			}
		}
		// Remove any old callback trigger from the list
		if (psCallbackList && psCallbackList->event == event &&
			psCallbackList->psContext == psContext)
		{
			psCurr = psCallbackList;
			psCallbackList = psCallbackList->psNext;
			HEAP_FREE(psTrigHeap, psCurr);
		}
		else
		{
			for(psCurr=psCallbackList; psCurr; psCurr=psCurr->psNext)
			{
				if (psCurr->psContext == psContext &&
					psCurr->event == event)
				{
					break;
				}
				psPrev = psCurr;
			}
			if (psCurr)
			{
				psPrev->psNext = psCurr->psNext;
				HEAP_FREE(psTrigHeap, psCurr);
			}
		}*/
    }
    // Create a new trigger if necessary
    if trigger >= 0 as libc::c_int {
        if eventInitTrigger(&mut psTrigger, (*psFiringTrigger).psContext,
                            event, trigger, updateTime) == 0 {
            return 0 as libc::c_int
        }
        (*psTrigger).psNext = psAddedTriggers;
        psAddedTriggers = psTrigger
    }
    return 1 as libc::c_int;
}
// set the event tracing level
//   0 - no tracing
//   1 - only fired triggers
//   2 - added and fired triggers
//   3 - as 2 but show tested but not fired triggers as well
// set the event tracing level - to be called from script functions
#[no_mangle]
pub unsafe extern "C" fn eventSetTraceLevel() -> BOOL {
    let mut level: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut level as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if level < 0 as libc::c_int { level = 0 as libc::c_int }
    if level > 3 as libc::c_int { level = 3 as libc::c_int }
    eventTraceLevel = level;
    return 1 as libc::c_int;
}
// The list of currently active triggers
// The list of callback triggers
// The currently allocated contexts
/* Initialise the event system */
// Shutdown the event system
// add a TR_PAUSE trigger to the event system.
// Load a trigger into the system from a save game
//resets the event timer - updateTime
//reset local vars
#[no_mangle]
pub unsafe extern "C" fn resetLocalVars(mut psCode: *mut SCRIPT_CODE,
                                        mut EventIndex: UDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    if EventIndex >= (*psCode).numEvents as libc::c_uint {
        if !(*psCode).psDebug.is_null() {
            debug(LOG_ERROR,
                  b"resetLocalVars: wrong event index: %d (Event name: %s, total events count = %d, stack depth = %d)\x00"
                      as *const u8 as *const libc::c_char, EventIndex,
                  eventGetEventID(psCode, EventIndex as SDWORD),
                  (*psCode).numEvents as libc::c_int, GetCallDepth());
        } else {
            debug(LOG_ERROR,
                  b"resetLocalVars: wrong event index: %d (total events count = %d, stack depth = %d)\x00"
                      as *const u8 as *const libc::c_char, EventIndex,
                  (*psCode).numEvents as libc::c_int, GetCallDepth());
        }
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while (i as libc::c_uint) <
              *(*psCode).numLocalVars.offset(EventIndex as isize) {
        //Initialize main value
        if (*(*(*psCode).ppsLocalVarVal.offset(EventIndex as
                                                   isize)).offset(i as
                                                                      isize)).type_0
               as libc::c_uint == VAL_STRING as libc::c_int as libc::c_uint {
            //debug(LOG_ERROR , "resetLocalVars: String type is not implemented");
            let ref mut fresh5 =
                (*(*(*psCode).ppsLocalVarVal.offset(EventIndex as
                                                        isize)).offset(i as
                                                                           isize)).v.sval;
            *fresh5 =
                memMallocRelease(255 as libc::c_int as size_t) as
                    *mut libc::c_char;
            strcpy((*(*(*psCode).ppsLocalVarVal.offset(EventIndex as
                                                           isize)).offset(i as
                                                                              isize)).v.sval,
                   b"\x00\x00" as *const u8 as *const libc::c_char);
        } else {
            (*(*(*psCode).ppsLocalVarVal.offset(EventIndex as
                                                    isize)).offset(i as
                                                                       isize)).v.ival
                = 0 as libc::c_int
        }
        /* only group (!) must be re-created each time */
        if (*(*(*psCode).ppsLocalVarVal.offset(EventIndex as
                                                   isize)).offset(i as
                                                                      isize)).type_0
               as libc::c_uint == ST_GROUP as libc::c_int as libc::c_uint {
            debug(LOG_SCRIPT,
                  b"resetLocalVars -  created\x00" as *const u8 as
                      *const libc::c_char);
            if (*asCreateFuncs.offset((*(*(*psCode).ppsLocalVarVal.offset(EventIndex
                                                                              as
                                                                              isize)).offset(i
                                                                                                 as
                                                                                                 isize)).type_0
                                          as
                                          isize)).expect("non-null function pointer")(&mut *(*(*psCode).ppsLocalVarVal.offset(EventIndex
                                                                                                                                  as
                                                                                                                                  isize)).offset(i
                                                                                                                                                     as
                                                                                                                                                     isize))
                   == 0 {
                debug(LOG_ERROR,
                      b"asCreateFuncs failed for local var (re-init)\x00" as
                          *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
        }
        i += 1
    }
    //debug(LOG_SCRIPT, "Reset local vars for event %d", EventIndex);
    return 1 as libc::c_int;
}
