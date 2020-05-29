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
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    /* Return the resource for a type and ID */
    #[no_mangle]
    fn resGetDataFromHash(pType: *mut STRING, HashedID: UDWORD)
     -> *mut libc::c_void;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    // return the HashedID string for a piece of data
    #[no_mangle]
    fn resGetHashfromData(pType: *mut STRING, pData: *mut libc::c_void,
                          pHash: *mut UDWORD) -> BOOL;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    /* The table of user types */
    #[no_mangle]
    static mut asScrTypeTab: *mut TYPE_SYMBOL;
    #[no_mangle]
    fn eventGetContextVal(psContext: *mut SCRIPT_CONTEXT, index: UDWORD,
                          ppsVal: *mut *mut INTERP_VAL) -> BOOL;
    #[no_mangle]
    fn eventSetContextVar(psContext: *mut SCRIPT_CONTEXT, index: UDWORD,
                          type_0: INTERP_TYPE, data: UDWORD) -> BOOL;
    #[no_mangle]
    fn eventNewContext(psCode: *mut SCRIPT_CODE, release: CONTEXT_RELEASE,
                       ppsContext: *mut *mut SCRIPT_CONTEXT) -> BOOL;
    #[no_mangle]
    static mut psTrigList: *mut ACTIVE_TRIGGER;
    #[no_mangle]
    static mut psCallbackList: *mut ACTIVE_TRIGGER;
    #[no_mangle]
    static mut psContList: *mut SCRIPT_CONTEXT;
    #[no_mangle]
    fn eventLoadTrigger(time: UDWORD, psContext: *mut SCRIPT_CONTEXT,
                        type_0: SDWORD, trigger: SDWORD, event: UDWORD,
                        offset: UDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
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
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
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
/* Description of a trigger for the SCRIPT_CODE */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trigger_data {
    pub type_0: UWORD,
    pub code: UWORD,
    pub time: UDWORD,
}
pub type TRIGGER_DATA = _trigger_data;
// How often to check the trigger
/* A compiled script and its associated data */
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
pub struct _val_chunk {
    pub asVals: [INTERP_VAL; 20],
    pub psNext: *mut _val_chunk,
}
pub type VAL_CHUNK = _val_chunk;
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
// The type id to use in the type field of values
// Whether the type is an object or a simple value
// Type identifier
// load and save functions
// 
/*
 * EvntSave.c
 *
 * Save the state of the event system.
 *
 */
// the event save file header
pub type EVENT_SAVE_HDR = _event_save_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _event_save_header {
    pub aFileType: [STRING; 4],
    pub version: UDWORD,
}
pub type CONTEXT_RELEASE = _context_release;
pub type _context_release = libc::c_uint;
pub const CR_NORELEASE: _context_release = 1;
pub const CR_RELEASE: _context_release = 0;
// save the context information for the script system
unsafe extern "C" fn eventSaveContext(mut pBuffer: *mut libc::c_char,
                                      mut pSize: *mut UDWORD) -> BOOL {
    let mut size: UDWORD = 0;
    let mut valSize: UDWORD = 0;
    let mut numVars: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numContext: SDWORD = 0;
    let mut psCCont: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut psCVals: *mut VAL_CHUNK = 0 as *mut VAL_CHUNK;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    let mut saveFunc: SCR_VAL_SAVE = None;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    //not hashed	STRING				*pScriptID;
    let mut hashedName: UDWORD = 0;
    let mut pValSize: *mut UWORD = 0 as *mut UWORD;
    size = 0 as libc::c_int as UDWORD;
    numContext = 0 as libc::c_int;
    pPos = pBuffer;
    // reserve space to store how many contexts are saved
    if !pBuffer.is_null() {
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize)
    }
    size =
        (size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    // go through the context list
    psCCont = psContList;
    while !psCCont.is_null() {
        numContext += 1 as libc::c_int;
        // save the context info
//nothashed if (!resGetIDfromData("SCRIPT", psCCont->psCode, &hashedName))
        if resGetHashfromData(b"SCRIPT\x00" as *const u8 as
                                  *const libc::c_char as *mut STRING,
                              (*psCCont).psCode as *mut libc::c_void,
                              &mut hashedName) == 0 {
            debug(LOG_ERROR,
                  b"eventSaveContext: couldn\'t find script resource id\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        numVars =
            ((*(*psCCont).psCode).numGlobals as
                 libc::c_uint).wrapping_add((*(*psCCont).psCode).arraySize) as
                SDWORD;
        if !pBuffer.is_null() {
            //not hashed			strcpy(pPos, pScriptID);
//not hashed			pPos += strlen(pScriptID) + 1;
            *(pPos as *mut UDWORD) = hashedName;
            pPos =
                pPos.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong
                                as isize);
            *(pPos as *mut SWORD) = numVars as SWORD;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            *pPos = (*psCCont).release as UBYTE as libc::c_char;
            pPos =
                pPos.offset(::std::mem::size_of::<UBYTE>() as libc::c_ulong as
                                isize)
        }
        //not hashed		size += strlen(pScriptID) + 1 + sizeof(SWORD) + sizeof(UBYTE);
        size =
            (size as
                 libc::c_uint).wrapping_add((::std::mem::size_of::<UDWORD>()
                                                 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<SWORD>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                as UDWORD as UDWORD;
        // save the context variables
        psCVals = (*psCCont).psGlobals;
        while !psCVals.is_null() {
            i = 0 as libc::c_int;
            while i < 20 as libc::c_int {
                psVal = (*psCVals).asVals.as_mut_ptr().offset(i as isize);
                // store the variable type
                if !pBuffer.is_null() {
                    if ((*psVal).type_0 as libc::c_uint) <
                           0x7fff as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"eventSaveContext: variable type number too big\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if ((*psVal).type_0 as libc::c_uint) <
                           0x7fff as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"evntsave.c\x00" as *const u8 as
                                  *const libc::c_char, 90 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 17],
                                                        &[libc::c_char; 17]>(b"eventSaveContext\x00")).as_ptr(),
                              b"psVal->type < SWORD_MAX\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    *(pPos as *mut SWORD) = (*psVal).type_0 as SWORD;
                    pPos =
                        pPos.offset(::std::mem::size_of::<SWORD>() as
                                        libc::c_ulong as isize)
                }
                size =
                    (size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>()
                                                        as libc::c_ulong) as
                        UDWORD as UDWORD;
                // store the variable value
                if ((*psVal).type_0 as libc::c_uint) <
                       VAL_USERTYPESTART as libc::c_int as libc::c_uint {
                    // internal type - just store the DWORD value
                    if !pBuffer.is_null() {
                        *(pPos as *mut UDWORD) =
                            (*psVal).v.ival as
                                UDWORD; //TODO: make it save strings properly
                        pPos =
                            pPos.offset(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong as isize)
                    }
                    size =
                        (size as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                            as libc::c_ulong)
                            as UDWORD as UDWORD
                } else {
                    // user defined type
                    saveFunc =
                        (*asScrTypeTab.offset(((*psVal).type_0 as
                                                   libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                                  as isize)).saveFunc;
                    if saveFunc.is_some() {
                    } else {
                        debug(LOG_ERROR,
                              b"eventSaveContext: no save function for type %d\n\x00"
                                  as *const u8 as *const libc::c_char,
                              (*psVal).type_0 as libc::c_uint);
                    };
                    if saveFunc.is_some() {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"evntsave.c\x00" as *const u8 as
                                  *const libc::c_char, 114 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 17],
                                                        &[libc::c_char; 17]>(b"eventSaveContext\x00")).as_ptr(),
                              b"saveFunc != NULL\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    // reserve some space to store how many bytes the value uses
                    if !pBuffer.is_null() {
                        pValSize = pPos as *mut UWORD;
                        pPos =
                            pPos.offset(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong as isize)
                    }
                    size =
                        (size as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>()
                                                            as libc::c_ulong)
                            as UDWORD as UDWORD;
                    if saveFunc.expect("non-null function pointer")((*psVal).type_0,
                                                                    (*psVal).v.ival
                                                                        as
                                                                        UDWORD,
                                                                    pPos,
                                                                    &mut valSize)
                           == 0 {
                        debug(LOG_ERROR,
                              b"eventSaveContext: couldn\'t get variable value size\x00"
                                  as *const u8 as *const libc::c_char);
                        abort();
                    }
                    if !pBuffer.is_null() {
                        *pValSize = valSize as UWORD;
                        pPos = pPos.offset(valSize as isize)
                    }
                    size =
                        (size as libc::c_uint).wrapping_add(valSize) as UDWORD
                            as UDWORD
                }
                numVars -= 1 as libc::c_int;
                if numVars <= 0 as libc::c_int {
                    // done all the variables
                    if (*psCVals).psNext.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"eventSaveContext: number of context variables does not match the script code\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if (*psCVals).psNext.is_null() {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"evntsave.c\x00" as *const u8 as
                                  *const libc::c_char, 144 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 17],
                                                        &[libc::c_char; 17]>(b"eventSaveContext\x00")).as_ptr(),
                              b"psCVals->psNext == NULL\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    break ;
                } else { i += 1 as libc::c_int }
            }
            psCVals = (*psCVals).psNext
        }
        if numVars == 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"eventSaveContext: number of context variables does not match the script code\x00"
                      as *const u8 as *const libc::c_char);
        };
        if numVars == 0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"evntsave.c\x00" as *const u8 as *const libc::c_char,
                  150 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"eventSaveContext\x00")).as_ptr(),
                  b"numVars == 0\x00" as *const u8 as *const libc::c_char);
        };
        psCCont = (*psCCont).psNext
    }
    // actually store how many contexts have been saved
    if !pBuffer.is_null() { *(pBuffer as *mut SWORD) = numContext as SWORD }
    *pSize = size;
    return 1 as libc::c_int;
}
// load the context information for the script system
unsafe extern "C" fn eventLoadContext(mut version: SDWORD,
                                      mut pBuffer: *mut libc::c_char,
                                      mut pSize: *mut UDWORD) -> BOOL {
    let mut size: UDWORD = 0;
    let mut valSize: UDWORD = 0;
    let mut data: UDWORD = 0;
    let mut numVars: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numContext: SDWORD = 0;
    let mut context: SDWORD = 0;
    let mut psCCont: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut loadFunc: SCR_VAL_LOAD = None;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pScriptID: *mut STRING = 0 as *mut STRING;
    let mut psCode: *mut SCRIPT_CODE = 0 as *mut SCRIPT_CODE;
    let mut release: SWORD = 0;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    size = 0 as libc::c_int as UDWORD;
    pPos = pBuffer;
    // get the number of contexts in the save file
    numContext = *(pPos as *mut SWORD) as SDWORD;
    pPos =
        pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as isize);
    size =
        (size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    // go through the contexts
    context = 0 as libc::c_int;
    while context < numContext {
        // get the script code
        pScriptID = pPos as *mut STRING;
        psCode =
            resGetData(b"SCRIPT\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, pScriptID) as *mut SCRIPT_CODE;
        pPos =
            pPos.offset(strlen(pScriptID).wrapping_add(1 as libc::c_int as
                                                           libc::c_uint) as
                            isize);
        // check the number of variables
        numVars =
            ((*psCode).numGlobals as
                 libc::c_uint).wrapping_add((*psCode).arraySize) as SDWORD;
        if numVars != *(pPos as *mut SWORD) as libc::c_int {
            debug(LOG_ERROR,
                  b"eventLoadContext: number of context variables does not match the script code\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize);
        release = *pPos as SWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<UBYTE>() as libc::c_ulong as
                            isize);
        // create the context
        if eventNewContext(psCode, release as CONTEXT_RELEASE, &mut psCCont)
               == 0 {
            return 0 as libc::c_int
        }
        // bit of a hack this - note the id of the context to link it to the triggers
        (*psContList).id = context as SWORD;
        size =
            (size as
                 libc::c_uint).wrapping_add(strlen(pScriptID).wrapping_add(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>()
                                                                                                              as
                                                                                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                                                                              as
                                                                                                                                              libc::c_ulong))
                as UDWORD as UDWORD;
        // set the context variables
        i = 0 as libc::c_int;
        while i < numVars {
            // get the variable type
            type_0 = *(pPos as *mut SWORD) as INTERP_TYPE;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            size =
                (size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>()
                                                    as libc::c_ulong) as
                    UDWORD as UDWORD;
            // get the variable value
            if (type_0 as libc::c_uint) <
                   VAL_USERTYPESTART as libc::c_int as libc::c_uint {
                // internal type - just get the DWORD value
                data = *(pPos as *mut UDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<UDWORD>() as
                                    libc::c_ulong as isize);
                size =
                    (size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                        as libc::c_ulong) as
                        UDWORD as UDWORD;
                // set the value in the context
                if eventSetContextVar(psCCont, i as UDWORD, type_0, data) == 0
                   {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t set variable value\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
            } else {
                // user defined type
                loadFunc =
                    (*asScrTypeTab.offset((type_0 as
                                               libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                              as isize)).loadFunc;
                if loadFunc.is_some() {
                } else {
                    debug(LOG_ERROR,
                          b"eventLoadContext: no load function for type %d\n\x00"
                              as *const u8 as *const libc::c_char,
                          type_0 as libc::c_uint);
                };
                if loadFunc.is_some() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"evntsave.c\x00" as *const u8 as
                              *const libc::c_char, 247 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"eventLoadContext\x00")).as_ptr(),
                          b"loadFunc != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                valSize = *(pPos as *mut UWORD) as UDWORD;
                pPos =
                    pPos.offset(::std::mem::size_of::<UWORD>() as
                                    libc::c_ulong as isize);
                size =
                    (size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>()
                                                        as libc::c_ulong) as
                        UDWORD as UDWORD;
                // get the value pointer so that the loadFunc can write directly
				// into the variables data space.
                if eventGetContextVal(psCCont, i as UDWORD, &mut psVal) == 0 {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t find variable in context\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
                if loadFunc.expect("non-null function pointer")(version,
                                                                type_0, pPos,
                                                                valSize,
                                                                &mut (*psVal).v.ival
                                                                    as
                                                                    *mut SDWORD
                                                                    as
                                                                    *mut UDWORD)
                       == 0 {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t get variable value\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
                pPos = pPos.offset(valSize as isize);
                size =
                    (size as libc::c_uint).wrapping_add(valSize) as UDWORD as
                        UDWORD
            }
            i += 1 as libc::c_int
        }
        context += 1 as libc::c_int
    }
    *pSize = size;
    return 1 as libc::c_int;
}
// load the context information for the script system
unsafe extern "C" fn eventLoadContextHashed(mut version: SDWORD,
                                            mut pBuffer: *mut libc::c_char,
                                            mut pSize: *mut UDWORD) -> BOOL {
    let mut size: UDWORD = 0;
    let mut valSize: UDWORD = 0;
    let mut data: UDWORD = 0;
    let mut numVars: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numContext: SDWORD = 0;
    let mut context: SDWORD = 0;
    let mut psCCont: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut loadFunc: SCR_VAL_LOAD = None;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    //not hashed	STRING				*pScriptID;
    let mut hashedName: UDWORD = 0;
    let mut psCode: *mut SCRIPT_CODE = 0 as *mut SCRIPT_CODE;
    let mut release: SWORD = 0;
    let mut psVal: *mut INTERP_VAL = 0 as *mut INTERP_VAL;
    size = 0 as libc::c_int as UDWORD;
    pPos = pBuffer;
    // get the number of contexts in the save file
    numContext = *(pPos as *mut SWORD) as SDWORD;
    pPos =
        pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as isize);
    size =
        (size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    // go through the contexts
    context = 0 as libc::c_int;
    while context < numContext {
        // get the script code
//notHashed		pScriptID = (STRING *)pPos;
//notHashed		psCode = resGetData("SCRIPT", pScriptID);
//notHashed		pPos += strlen(pScriptID) + 1;
        hashedName = *(pPos as *mut UDWORD);
        pPos =
            pPos.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong as
                            isize);
        psCode =
            resGetDataFromHash(b"SCRIPT\x00" as *const u8 as
                                   *const libc::c_char as *mut STRING,
                               hashedName) as *mut SCRIPT_CODE;
        // check the number of variables
        numVars =
            ((*psCode).numGlobals as
                 libc::c_uint).wrapping_add((*psCode).arraySize) as SDWORD;
        if numVars != *(pPos as *mut SWORD) as libc::c_int {
            debug(LOG_ERROR,
                  b"eventLoadContext: number of context variables does not match the script code\x00"
                      as *const u8 as *const libc::c_char);
            abort();
        }
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize);
        release = *pPos as SWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<UBYTE>() as libc::c_ulong as
                            isize);
        // create the context
        if eventNewContext(psCode, release as CONTEXT_RELEASE, &mut psCCont)
               == 0 {
            return 0 as libc::c_int
        }
        // bit of a hack this - note the id of the context to link it to the triggers
        (*psContList).id = context as SWORD;
        size =
            (size as
                 libc::c_uint).wrapping_add((::std::mem::size_of::<UDWORD>()
                                                 as
                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<SWORD>()
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                                                 as
                                                                                                                 libc::c_ulong))
                as UDWORD as UDWORD;
        // set the context variables
        i = 0 as libc::c_int;
        while i < numVars {
            // get the variable type
            type_0 = *(pPos as *mut SWORD) as INTERP_TYPE;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            size =
                (size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>()
                                                    as libc::c_ulong) as
                    UDWORD as UDWORD;
            // get the variable value
            if (type_0 as libc::c_uint) <
                   VAL_USERTYPESTART as libc::c_int as libc::c_uint {
                // internal type - just get the DWORD value
                data = *(pPos as *mut UDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<UDWORD>() as
                                    libc::c_ulong as isize);
                size =
                    (size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                        as libc::c_ulong) as
                        UDWORD as UDWORD;
                // set the value in the context
                if eventSetContextVar(psCCont, i as UDWORD, type_0, data) == 0
                   {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t set variable value\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
            } else {
                // user defined type
                loadFunc =
                    (*asScrTypeTab.offset((type_0 as
                                               libc::c_uint).wrapping_sub(VAL_USERTYPESTART
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                              as isize)).loadFunc;
                if loadFunc.is_some() {
                } else {
                    debug(LOG_ERROR,
                          b"eventLoadContext: no load function for type %d\n\x00"
                              as *const u8 as *const libc::c_char,
                          type_0 as libc::c_uint);
                };
                if loadFunc.is_some() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"evntsave.c\x00" as *const u8 as
                              *const libc::c_char, 369 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"eventLoadContextHashed\x00")).as_ptr(),
                          b"loadFunc != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                valSize = *(pPos as *mut UWORD) as UDWORD;
                pPos =
                    pPos.offset(::std::mem::size_of::<UWORD>() as
                                    libc::c_ulong as isize);
                size =
                    (size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>()
                                                        as libc::c_ulong) as
                        UDWORD as UDWORD;
                // get the value pointer so that the loadFunc can write directly
				// into the variables data space.
                if eventGetContextVal(psCCont, i as UDWORD, &mut psVal) == 0 {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t find variable in context\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
                if loadFunc.expect("non-null function pointer")(version,
                                                                type_0, pPos,
                                                                valSize,
                                                                &mut (*psVal).v.ival
                                                                    as
                                                                    *mut SDWORD
                                                                    as
                                                                    *mut UDWORD)
                       == 0 {
                    debug(LOG_ERROR,
                          b"eventLoadContext: couldn\'t get variable value\x00"
                              as *const u8 as *const libc::c_char);
                    abort();
                }
                pPos = pPos.offset(valSize as isize);
                size =
                    (size as libc::c_uint).wrapping_add(valSize) as UDWORD as
                        UDWORD
            }
            i += 1 as libc::c_int
        }
        context += 1 as libc::c_int
    }
    *pSize = size;
    return 1 as libc::c_int;
}
// return the index of a context
#[no_mangle]
pub unsafe extern "C" fn eventGetContextIndex(mut psContext:
                                                  *mut SCRIPT_CONTEXT,
                                              mut pIndex: *mut SDWORD)
 -> BOOL {
    let mut psCurr: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    let mut index: SDWORD = 0;
    index = 0 as libc::c_int;
    psCurr = psContList;
    while !psCurr.is_null() {
        if psCurr == psContext { *pIndex = index; return 1 as libc::c_int }
        index += 1 as libc::c_int;
        psCurr = (*psCurr).psNext
    }
    return 0 as libc::c_int;
}
// find a context from it's id number
#[no_mangle]
pub unsafe extern "C" fn eventFindContext(mut id: SDWORD,
                                          mut ppsContext:
                                              *mut *mut SCRIPT_CONTEXT)
 -> BOOL {
    let mut psCurr: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    psCurr = psContList;
    while !psCurr.is_null() {
        if (*psCurr).id as libc::c_int == id {
            *ppsContext = psCurr;
            return 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as libc::c_int;
}
// save a list of triggers
#[no_mangle]
pub unsafe extern "C" fn eventSaveTriggerList(mut psList: *mut ACTIVE_TRIGGER,
                                              mut pBuffer: *mut libc::c_char,
                                              mut pSize: *mut UDWORD)
 -> BOOL {
    let mut psCurr: *mut ACTIVE_TRIGGER = 0 as *mut ACTIVE_TRIGGER;
    let mut size: UDWORD = 0;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numTriggers: SDWORD = 0;
    let mut context: SDWORD = 0;
    size = 0 as libc::c_int as UDWORD;
    pPos = pBuffer;
    // reserve some space for the number of triggers
    if !pBuffer.is_null() {
        pPos =
            pPos.offset(::std::mem::size_of::<SDWORD>() as libc::c_ulong as
                            isize)
    }
    size =
        (size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    numTriggers = 0 as libc::c_int;
    psCurr = psList;
    while !psCurr.is_null() {
        numTriggers += 1 as libc::c_int;
        if !pBuffer.is_null() {
            *(pPos as *mut UDWORD) = (*psCurr).testTime;
            pPos =
                pPos.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong
                                as isize);
            if eventGetContextIndex((*psCurr).psContext, &mut context) == 0 {
                debug(LOG_ERROR,
                      b"eventSaveTriggerList: couldn\'t find context\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
            *(pPos as *mut SWORD) = context as SWORD;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            *(pPos as *mut SWORD) = (*psCurr).type_0;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            *(pPos as *mut SWORD) = (*psCurr).trigger;
            pPos =
                pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                                isize);
            *(pPos as *mut UWORD) = (*psCurr).event;
            pPos =
                pPos.offset(::std::mem::size_of::<UWORD>() as libc::c_ulong as
                                isize);
            *(pPos as *mut UWORD) = (*psCurr).offset;
            pPos =
                pPos.offset(::std::mem::size_of::<UWORD>() as libc::c_ulong as
                                isize)
        }
        size =
            (size as
                 libc::c_uint).wrapping_add((::std::mem::size_of::<UDWORD>()
                                                 as
                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<SWORD>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(3
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_add((::std::mem::size_of::<UWORD>()
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong).wrapping_mul(2
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_uint)))
                as UDWORD as UDWORD;
        psCurr = (*psCurr).psNext
    }
    if !pBuffer.is_null() { *(pBuffer as *mut SDWORD) = numTriggers }
    *pSize = size;
    return 1 as libc::c_int;
}
// load a list of triggers
unsafe extern "C" fn eventLoadTriggerList(mut version: SDWORD,
                                          mut pBuffer: *mut libc::c_char,
                                          mut pSize: *mut UDWORD) -> BOOL {
    let mut size: UDWORD = 0;
    let mut event: UDWORD = 0;
    let mut offset: UDWORD = 0;
    let mut time: UDWORD = 0;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numTriggers: SDWORD = 0;
    let mut context: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut trigger: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut psContext: *mut SCRIPT_CONTEXT = 0 as *mut SCRIPT_CONTEXT;
    version = version;
    size = 0 as libc::c_int as UDWORD;
    pPos = pBuffer;
    // get the number of triggers
    numTriggers = *(pPos as *mut SDWORD);
    pPos =
        pPos.offset(::std::mem::size_of::<SDWORD>() as libc::c_ulong as
                        isize);
    size =
        (size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    i = 0 as libc::c_int;
    while i < numTriggers {
        time = *(pPos as *mut UDWORD);
        pPos =
            pPos.offset(::std::mem::size_of::<UDWORD>() as libc::c_ulong as
                            isize);
        context = *(pPos as *mut SWORD) as SDWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize);
        if eventFindContext(context, &mut psContext) == 0 {
            debug(LOG_ERROR,
                  b"eventLoadTriggerList: couldn\'t find context\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
        type_0 = *(pPos as *mut SWORD) as SDWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize);
        trigger = *(pPos as *mut SWORD) as SDWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<SWORD>() as libc::c_ulong as
                            isize);
        event = *(pPos as *mut UWORD) as UDWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<UWORD>() as libc::c_ulong as
                            isize);
        offset = *(pPos as *mut UWORD) as UDWORD;
        pPos =
            pPos.offset(::std::mem::size_of::<UWORD>() as libc::c_ulong as
                            isize);
        size =
            (size as
                 libc::c_uint).wrapping_add((::std::mem::size_of::<UDWORD>()
                                                 as
                                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<SWORD>()
                                                                                  as
                                                                                  libc::c_ulong).wrapping_mul(3
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_add((::std::mem::size_of::<UWORD>()
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong).wrapping_mul(2
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                   as
                                                                                                                                                                                   libc::c_uint)))
                as UDWORD as UDWORD;
        if eventLoadTrigger(time, psContext, type_0, trigger, event, offset)
               == 0 {
            return 0 as libc::c_int
        }
        i += 1 as libc::c_int
    }
    *pSize = size;
    return 1 as libc::c_int;
}
// Save the state of the event system
#[no_mangle]
pub unsafe extern "C" fn eventSaveState(mut version: SDWORD,
                                        mut ppBuffer: *mut *mut libc::c_char,
                                        mut pFileSize: *mut UDWORD) -> BOOL {
    let mut size: UDWORD = 0;
    let mut totalSize: UDWORD = 0;
    let mut pBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psHdr: *mut EVENT_SAVE_HDR = 0 as *mut EVENT_SAVE_HDR;
    totalSize = ::std::mem::size_of::<EVENT_SAVE_HDR>() as libc::c_ulong;
    // find the size of the context save
    if eventSaveContext(0 as *mut libc::c_char, &mut size) == 0 {
        return 0 as libc::c_int
    }
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    // find the size of the trigger save
    if eventSaveTriggerList(psTrigList, 0 as *mut libc::c_char, &mut size) ==
           0 {
        return 0 as libc::c_int
    }
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    // find the size of the callback trigger save
    if eventSaveTriggerList(psCallbackList, 0 as *mut libc::c_char, &mut size)
           == 0 {
        return 0 as libc::c_int
    }
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    // Allocate the buffer to save to
    pBuffer = memMallocRelease(totalSize) as *mut libc::c_char;
    if pBuffer.is_null() {
        debug(LOG_ERROR,
              b"eventSaveState: out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    pPos = pBuffer;
    // set the header
    psHdr = pPos as *mut EVENT_SAVE_HDR;
    (*psHdr).aFileType[0 as libc::c_int as usize] = 'e' as i32 as STRING;
    (*psHdr).aFileType[1 as libc::c_int as usize] = 'v' as i32 as STRING;
    (*psHdr).aFileType[2 as libc::c_int as usize] = 'n' as i32 as STRING;
    (*psHdr).aFileType[3 as libc::c_int as usize] = 't' as i32 as STRING;
    (*psHdr).version = version as UDWORD;
    pPos =
        pPos.offset(::std::mem::size_of::<EVENT_SAVE_HDR>() as libc::c_ulong
                        as isize);
    // save the contexts
    if eventSaveContext(pPos, &mut size) == 0 { return 0 as libc::c_int }
    pPos = pPos.offset(size as isize);
    // save the triggers
    if eventSaveTriggerList(psTrigList, pPos, &mut size) == 0 {
        return 0 as libc::c_int
    }
    pPos = pPos.offset(size as isize);
    // save the callback triggers
    if eventSaveTriggerList(psCallbackList, pPos, &mut size) == 0 {
        return 0 as libc::c_int
    }
    pPos = pPos.offset(size as isize);
    *ppBuffer = pBuffer;
    *pFileSize = totalSize;
    return 1 as libc::c_int;
}
/*
 * EvntSave.h
 *
 * Save the state of the event system.
 *
 */
// Save the state of the event system
// Load the state of the event system
// Load the state of the event system
#[no_mangle]
pub unsafe extern "C" fn eventLoadState(mut pBuffer: *mut libc::c_char,
                                        mut fileSize: UDWORD,
                                        mut bHashed: BOOL) -> BOOL {
    let mut size: UDWORD = 0;
    let mut totalSize: UDWORD = 0;
    let mut version: UDWORD = 0;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psHdr: *mut EVENT_SAVE_HDR = 0 as *mut EVENT_SAVE_HDR;
    pPos = pBuffer;
    totalSize = 0 as libc::c_int as UDWORD;
    // Get the header
    psHdr = pPos as *mut EVENT_SAVE_HDR;
    if strncmp((*psHdr).aFileType.as_mut_ptr(),
               b"evnt\x00" as *const u8 as *const libc::c_char,
               4 as libc::c_int as libc::c_uint) != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"eventLoadState: invalid file header\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /*	if ((psHdr->version != 1) &&
		(psHdr->version != 2))
	{
		DBERROR(("eventLoadState: invalid file version"));
		return FALSE;
	}*/
    version = (*psHdr).version;
    pPos =
        pPos.offset(::std::mem::size_of::<EVENT_SAVE_HDR>() as libc::c_ulong
                        as isize);
    totalSize =
        (totalSize as
             libc::c_uint).wrapping_add(::std::mem::size_of::<EVENT_SAVE_HDR>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    // load the event contexts
    if bHashed != 0 {
        if eventLoadContextHashed(version as SDWORD, pPos, &mut size) == 0 {
            return 0 as libc::c_int
        }
    } else if eventLoadContext(version as SDWORD, pPos, &mut size) == 0 {
        return 0 as libc::c_int
    }
    pPos = pPos.offset(size as isize);
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    // load the normal triggers
    if eventLoadTriggerList(version as SDWORD, pPos, &mut size) == 0 {
        return 0 as libc::c_int
    }
    pPos = pPos.offset(size as isize);
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    // load the callback triggers
    if eventLoadTriggerList(version as SDWORD, pPos, &mut size) == 0 {
        return 0 as libc::c_int
    }
    pPos = pPos.offset(size as isize);
    totalSize =
        (totalSize as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
    if totalSize != fileSize {
        debug(LOG_ERROR,
              b"eventLoadState: corrupt save file\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    return 1 as libc::c_int;
}
