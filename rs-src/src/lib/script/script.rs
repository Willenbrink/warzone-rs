use ::libc;
extern "C" {
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn stackShutDown();
    #[no_mangle]
    fn stackInitialise() -> BOOL;
    #[no_mangle]
    fn interpInitialise() -> BOOL;
    /* Initialise the event system */
    #[no_mangle]
    fn eventInitialise(psInit: *mut EVENT_INIT) -> BOOL;
    // Shutdown the event system
    #[no_mangle]
    fn eventShutDown();
}
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
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
// user defined types should start with this id
/* Type used by the compiler for functions that do not return a value */
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
// events and triggers
pub const VAL_TRIGGER: _interp_type = 3;
//	VAL_FLOAT,
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
// Basic types
pub const VAL_BOOL: _interp_type = 0;
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
pub type INTERP_VAL = _interp_val;
pub type STORAGE_TYPE = UBYTE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _var_debug {
    pub pIdent: *mut STRING,
    pub storage: STORAGE_TYPE,
}
pub type VAR_DEBUG = _var_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_data {
    pub base: UDWORD,
    pub type_0: UBYTE,
    pub dimensions: UBYTE,
    pub elements: [UBYTE; 4],
}
pub type ARRAY_DATA = _array_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _array_debug {
    pub pIdent: *mut STRING,
    pub storage: UBYTE,
}
pub type ARRAY_DEBUG = _array_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _script_debug {
    pub offset: UDWORD,
    pub line: UDWORD,
    pub pLabel: *mut STRING,
}
pub type SCRIPT_DEBUG = _script_debug;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
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
pub type SCRIPT_CODE = _script_code;
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
/*
 * Script.c
 *
 * A few general functions for the script library
 */
// Flags for storing function indexes
// set for a set function, clear otherwise
// instinct function
// callback function
// object variable
// external variable
// Initialise the script library
#[no_mangle]
pub unsafe extern "C" fn scriptInitialise(mut psInit: *mut EVENT_INIT)
 -> BOOL {
    if stackInitialise() == 0 { return 0 as libc::c_int }
    if interpInitialise() == 0 { return 0 as libc::c_int }
    if eventInitialise(psInit) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Shutdown the script library
#[no_mangle]
pub unsafe extern "C" fn scriptShutDown() {
    eventShutDown();
    stackShutDown();
}
/* Free a SCRIPT_CODE structure */
#[no_mangle]
pub unsafe extern "C" fn scriptFreeCode(mut psCode: *mut SCRIPT_CODE) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    memFreeRelease((*psCode).pCode as *mut libc::c_void);
    (*psCode).pCode = 0 as *mut UDWORD;
    if !(*psCode).pTriggerTab.is_null() {
        memFreeRelease((*psCode).pTriggerTab as *mut libc::c_void);
        (*psCode).pTriggerTab = 0 as *mut UWORD
    }
    if !(*psCode).psTriggerData.is_null() {
        memFreeRelease((*psCode).psTriggerData as *mut libc::c_void);
        (*psCode).psTriggerData = 0 as *mut TRIGGER_DATA
    }
    memFreeRelease((*psCode).pEventTab as *mut libc::c_void);
    (*psCode).pEventTab = 0 as *mut UWORD;
    memFreeRelease((*psCode).pEventLinks as *mut libc::c_void);
    (*psCode).pEventLinks = 0 as *mut SWORD;
    if !(*psCode).pGlobals.is_null() {
        memFreeRelease((*psCode).pGlobals as *mut libc::c_void);
        (*psCode).pGlobals = 0 as *mut INTERP_TYPE
    }
    if !(*psCode).psArrayInfo.is_null() {
        memFreeRelease((*psCode).psArrayInfo as *mut libc::c_void);
        (*psCode).psArrayInfo = 0 as *mut ARRAY_DATA
    }
    if !(*psCode).psDebug.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < (*psCode).debugEntries as libc::c_uint {
            if !(*(*psCode).psDebug.offset(i as isize)).pLabel.is_null() {
                memFreeRelease((*(*psCode).psDebug.offset(i as isize)).pLabel
                                   as *mut libc::c_void);
                let ref mut fresh0 =
                    (*(*psCode).psDebug.offset(i as isize)).pLabel;
                *fresh0 = 0 as *mut STRING
            }
            i = i.wrapping_add(1)
        }
        memFreeRelease((*psCode).psDebug as *mut libc::c_void);
        (*psCode).psDebug = 0 as *mut SCRIPT_DEBUG
    }
    if !(*psCode).psVarDebug.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < (*psCode).numGlobals as libc::c_uint {
            if !(*(*psCode).psVarDebug.offset(i as isize)).pIdent.is_null() {
                memFreeRelease((*(*psCode).psVarDebug.offset(i as
                                                                 isize)).pIdent
                                   as *mut libc::c_void);
                let ref mut fresh1 =
                    (*(*psCode).psVarDebug.offset(i as isize)).pIdent;
                *fresh1 = 0 as *mut STRING
            }
            i = i.wrapping_add(1)
        }
        memFreeRelease((*psCode).psVarDebug as *mut libc::c_void);
        (*psCode).psVarDebug = 0 as *mut VAR_DEBUG
    }
    if !(*psCode).psArrayDebug.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < (*psCode).numArrays as libc::c_uint {
            if !(*(*psCode).psArrayDebug.offset(i as isize)).pIdent.is_null()
               {
                memFreeRelease((*(*psCode).psArrayDebug.offset(i as
                                                                   isize)).pIdent
                                   as *mut libc::c_void);
                let ref mut fresh2 =
                    (*(*psCode).psArrayDebug.offset(i as isize)).pIdent;
                *fresh2 = 0 as *mut STRING
            }
            i = i.wrapping_add(1)
        }
        memFreeRelease((*psCode).psArrayDebug as *mut libc::c_void);
        (*psCode).psArrayDebug = 0 as *mut ARRAY_DEBUG
    }
    /* Free local vars */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psCode).numEvents as libc::c_uint {
        if *(*psCode).numLocalVars.offset(i as isize) >
               0 as libc::c_int as libc::c_uint {
            //only free if any defined
            //free strings for event i
            j = 0 as libc::c_int as UDWORD; //if a string
            while j < *(*psCode).numLocalVars.offset(i as isize) {
                ((*(*(*psCode).ppsLocalVarVal.offset(i as
                                                         isize)).offset(j as
                                                                            isize)).type_0
                     as libc::c_uint) ==
                    VAL_STRING as libc::c_int as libc::c_uint;
                if !(*(*(*psCode).ppsLocalVarVal.offset(i as
                                                            isize)).offset(j
                                                                               as
                                                                               isize)).v.sval.is_null()
                   {
                    //doublecheck..
                    memFreeRelease((*(*(*psCode).ppsLocalVarVal.offset(i as
                                                                           isize)).offset(j
                                                                                              as
                                                                                              isize)).v.sval
                                       as *mut libc::c_void);
                    let ref mut fresh3 =
                        (*(*(*psCode).ppsLocalVarVal.offset(i as
                                                                isize)).offset(j
                                                                                   as
                                                                                   isize)).v.sval;
                    *fresh3 = 0 as *mut STRING
                }
                j = j.wrapping_add(1)
                //free string
            }
            memFreeRelease(*(*psCode).ppsLocalVars.offset(i as isize) as
                               *mut libc::c_void);
            let ref mut fresh4 = *(*psCode).ppsLocalVars.offset(i as isize);
            *fresh4 = 0 as *mut INTERP_TYPE;
            memFreeRelease(*(*psCode).ppsLocalVarVal.offset(i as isize) as
                               *mut libc::c_void);
            let ref mut fresh5 = *(*psCode).ppsLocalVarVal.offset(i as isize);
            *fresh5 = 0 as *mut INTERP_VAL
        }
        i = i.wrapping_add(1)
    }
    memFreeRelease((*psCode).numParams as *mut libc::c_void);
    (*psCode).numParams = 0 as *mut UDWORD;
    memFreeRelease((*psCode).numLocalVars as *mut libc::c_void);
    (*psCode).numLocalVars = 0 as *mut UDWORD;
    memFreeRelease((*psCode).ppsLocalVars as *mut libc::c_void);
    (*psCode).ppsLocalVars = 0 as *mut *mut INTERP_TYPE;
    memFreeRelease((*psCode).ppsLocalVarVal as *mut libc::c_void);
    (*psCode).ppsLocalVarVal = 0 as *mut *mut INTERP_VAL;
    (*psCode).numEvents = 0 as libc::c_int as UWORD;
    memFreeRelease(psCode as *mut libc::c_void);
    psCode = 0 as *mut SCRIPT_CODE;
}
//free pointer to event i local vars
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
/* Lookup a script variable */
#[no_mangle]
pub unsafe extern "C" fn scriptGetVarIndex(mut psCode: *mut SCRIPT_CODE,
                                           mut pID: *mut STRING,
                                           mut pIndex: *mut UDWORD) -> BOOL {
    let mut index: UDWORD = 0;
    if (*psCode).psVarDebug.is_null() { return 0 as libc::c_int }
    index = 0 as libc::c_int as UDWORD;
    while index < (*psCode).numGlobals as libc::c_uint {
        if strcmp((*(*psCode).psVarDebug.offset(index as isize)).pIdent, pID)
               == 0 as libc::c_int {
            *pIndex = index;
            return 1 as libc::c_int
        }
        index = index.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
