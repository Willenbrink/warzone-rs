use ::libc;
extern "C" {
    pub type _block_heap;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    /* Function type for object equality */
//typedef BOOL (*TREAP_EQUAL)(void *pObj1, void *pObj2);
    /* Function to create a treap
 * Pass in key comparison function
 * Initial number of nodes to allocate
 * Number of additional nodes to allocate when extending
 */
    #[no_mangle]
    fn treapCreate(ppsTreap: *mut *mut TREAP, cmp: TREAP_CMP, init: UDWORD,
                   ext: UDWORD) -> BOOL;
    /* Add an object to a treap
 */
    #[no_mangle]
    fn treapAdd(psTreap: *mut TREAP, key: UDWORD, pObj: *mut libc::c_void)
     -> BOOL;
    /* Remove an object from the treap */
    #[no_mangle]
    fn treapDel(psTreap: *mut TREAP, key: UDWORD) -> BOOL;
    /* Find an object in a treap */
    #[no_mangle]
    fn treapFind(psTreap: *mut TREAP, key: UDWORD) -> *mut libc::c_void;
    /* Destroy a treap and release all the memory associated with it */
    #[no_mangle]
    fn treapDestroy(psTreap: *mut TREAP);
    /* Return the object with the smallest key in the treap
 * This is useful if the objects in the treap need to be
 * deallocated.  i.e. getSmallest, delete from treap, free memory
 */
    #[no_mangle]
    fn treapGetSmallest(psTreap: *mut TREAP) -> *mut libc::c_void;
    /* ***************************************************************************************/
/*                            Comparison Functions                                      */
    /* A useful comparison function - keys are string pointers */
    #[no_mangle]
    fn treapStringCmp(key1: UDWORD, key2: UDWORD) -> SDWORD;
    /*
 * StrResLY.h
 *
 * Interface to the string resource lex and yacc functions.
 */
    /* Maximum number of characters in a directory entry */
    /* Maximum number of TEXT items in any one Yacc rule */
    /* The string resource currently being loaded */
    /* Set the current input buffer for the lexer - used by strresLoad */
    #[no_mangle]
    fn strresSetInputBuffer(pBuffer: *mut libc::c_char, size: UDWORD);
    /* Call the yacc parser */
    #[no_mangle]
    fn strres_parse() -> libc::c_int;
}
pub type size_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
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
pub type FREE_OBJECT = _free_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
/* structure used to store the extra space allocated for the heap */
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
// The size of the objects being stored on the heap
// The initial number of objects allocated
// The number of objects to allocate after the initial
// allocation is used up
// which block heap (if any) this object heap was allocated from
// The currently free objects
// The main memory heap
// Extension memory for the heap
/*
 * Treap.h
 *
 * A balanced binary tree implementation
 *
 * Overhead for using the treap is -
 *		Overhead for the heap used by the treap :
 *                  24 bytes + 4 bytes per extension
 *      12 bytes for the root
 *      20 bytes per node
 */
/* Turn on and off the treap debugging */
/* Function type for the object compare
 * return -1 for less
 *         1 for more
 *         0 for equal
 */
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
/* The basic elements in the treap node.
 * These are done as macros so that the memory system
 * can use parts of the treap system.
 */
/* The key to sort the node on */
/* Treap priority */
/* The object stored in the treap */
/* The sub trees */
/* The debug info */
/* file the node was created in */
/* line the node was created at */
pub type TREAP_NODE = _treap_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
/* Treap data structure */
pub type TREAP = _treap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
// comparison function
// node heap
// root of the tree
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
pub type STR_BLOCK = _str_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_id {
    pub id: UDWORD,
    pub pIDStr: *mut STRING,
}
/* An ID entry */
pub type STR_ID = _str_id;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
/* A String Resource */
pub type STR_RES = _str_res;
// The treap to store string identifiers
// The store for the strings themselves
// Sizes for the string blocks
// The next free ID
/*
 * StrRes.c
 *
 * String storage an manipulation functions
 *
 */
/* Allow frame header files to be singly included */
// Report unused strings
//#define DEBUG_GROUP0
/* The string resource currently being loaded */
#[no_mangle]
pub static mut psCurrRes: *mut STR_RES = 0 as *const STR_RES as *mut STR_RES;
/* Allocate a string block */
unsafe extern "C" fn strresAllocBlock(mut ppsBlock: *mut *mut STR_BLOCK,
                                      mut size: UDWORD) -> BOOL {
    *ppsBlock =
        memMallocRelease(::std::mem::size_of::<STR_BLOCK>() as libc::c_ulong)
            as *mut STR_BLOCK;
    if (*ppsBlock).is_null() {
        debug(LOG_ERROR,
              b"strresAllocBlock: Out of memory - 1\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    (**ppsBlock).apStrings =
        memMallocRelease((::std::mem::size_of::<*mut STRING>() as
                              libc::c_ulong).wrapping_mul(size)) as
            *mut *mut STRING;
    if (**ppsBlock).apStrings.is_null() {
        debug(LOG_ERROR,
              b"strresAllocBlock: Out of memory - 2\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset((**ppsBlock).apStrings as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut STRING>() as
                libc::c_ulong).wrapping_mul(size));
    return 1 as libc::c_int;
}
/* Create a string resource object */
/* Initialise the string system */
#[no_mangle]
pub unsafe extern "C" fn strresCreate(mut ppsRes: *mut *mut STR_RES,
                                      mut init: UDWORD, mut ext: UDWORD)
 -> BOOL {
    let mut psRes: *mut STR_RES = 0 as *mut STR_RES;
    psRes =
        memMallocRelease(::std::mem::size_of::<STR_RES>() as libc::c_ulong) as
            *mut STR_RES;
    if psRes.is_null() {
        debug(LOG_ERROR,
              b"strresCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    (*psRes).init = init;
    (*psRes).ext = ext;
    (*psRes).nextID = 0 as libc::c_int as UDWORD;
    if treapCreate(&mut (*psRes).psIDTreap,
                   Some(treapStringCmp as
                            unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                -> SDWORD), init, ext) == 0 {
        debug(LOG_ERROR,
              b"strresCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    if strresAllocBlock(&mut (*psRes).psStrings, init) == 0 {
        treapDestroy((*psRes).psIDTreap);
        memFreeRelease(psRes as *mut libc::c_void);
        psRes = 0 as *mut STR_RES;
        return 0 as libc::c_int
    }
    (*(*psRes).psStrings).psNext = 0 as *mut _str_block;
    (*(*psRes).psStrings).idStart = 0 as libc::c_int as UDWORD;
    (*(*psRes).psStrings).idEnd =
        init.wrapping_sub(1 as libc::c_int as libc::c_uint);
    *ppsRes = psRes;
    return 1 as libc::c_int;
}
/* Release the id strings, but not the strings themselves,
 * (they can be accessed only by id number).
 */
/* Release the id strings, but not the strings themselves,
 * (they can be accessed only by id number).
 */
#[no_mangle]
pub unsafe extern "C" fn strresReleaseIDStrings(mut psRes: *mut STR_RES) {
    let mut psID: *mut STR_ID = 0 as *mut STR_ID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              107 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"strresReleaseIDStrings\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psID = treapGetSmallest((*psRes).psIDTreap) as *mut STR_ID;
    while !psID.is_null() {
        treapDel((*psRes).psIDTreap, (*psID).pIDStr as UDWORD);
        if (*psID).id & 0x80000000 as libc::c_uint != 0 {
            memFreeRelease((*psID).pIDStr as *mut libc::c_void);
            (*psID).pIDStr = 0 as *mut STRING;
            memFreeRelease(psID as *mut libc::c_void);
            psID = 0 as *mut STR_ID
        }
        psID = treapGetSmallest((*psRes).psIDTreap) as *mut STR_ID
    };
}
/* Release a string resource object */
/* Shutdown the string system */
#[no_mangle]
pub unsafe extern "C" fn strresDestroy(mut psRes: *mut STR_RES) {
    let mut psBlock: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    let mut psNext: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    let mut i: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              129 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"strresDestroy\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Free the string id's
    strresReleaseIDStrings(psRes);
    // Free the strings themselves
    psBlock = (*psRes).psStrings;
    while !psBlock.is_null() {
        i = (*psBlock).idStart;
        while i <= (*psBlock).idEnd {
            if !(*(*psBlock).apStrings.offset(i.wrapping_sub((*psBlock).idStart)
                                                  as isize)).is_null() {
                memFreeRelease(*(*psBlock).apStrings.offset(i.wrapping_sub((*psBlock).idStart)
                                                                as isize) as
                                   *mut libc::c_void);
                let ref mut fresh0 =
                    *(*psBlock).apStrings.offset(i.wrapping_sub((*psBlock).idStart)
                                                     as isize);
                *fresh0 = 0 as *mut STRING
            }
            i = i.wrapping_add(1)
        }
        psNext = (*psBlock).psNext;
        memFreeRelease((*psBlock).apStrings as *mut libc::c_void);
        (*psBlock).apStrings = 0 as *mut *mut STRING;
        memFreeRelease(psBlock as *mut libc::c_void);
        psBlock = 0 as *mut STR_BLOCK;
        psBlock = psNext
    }
    // Release the treap and free the final memory
    treapDestroy((*psRes).psIDTreap);
    memFreeRelease(psRes as *mut libc::c_void);
    psRes = 0 as *mut STR_RES;
}
/* Load a list of string ID's from a memory buffer
 * id == 0 should be a default string which is returned if the
 * requested string is not found.
 */
/* Load a list of string ID's from a memory buffer */
#[no_mangle]
pub unsafe extern "C" fn strresLoadFixedID(mut psRes: *mut STR_RES,
                                           mut psID: *mut STR_ID,
                                           mut numID: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              179 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"strresLoadFixedID\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    i = 0 as libc::c_int as UDWORD;
    while i < numID {
        if (*psID).id == (*psRes).nextID {
        } else {
            debug(LOG_ERROR,
                  b"strresLoadFixedID: id out of sequence\x00" as *const u8 as
                      *const libc::c_char);
        };
        if (*psID).id == (*psRes).nextID {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"strres.c\x00" as *const u8 as *const libc::c_char,
                  184 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"strresLoadFixedID\x00")).as_ptr(),
                  b"psID->id == psRes->nextID\x00" as *const u8 as
                      *const libc::c_char);
        };
        // Store the ID string
        if treapAdd((*psRes).psIDTreap, (*psID).pIDStr as UDWORD,
                    psID as *mut libc::c_void) == 0 {
            debug(LOG_ERROR,
                  b"strresLoadFixedID: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        psID = psID.offset(1 as libc::c_int as isize);
        (*psRes).nextID =
            ((*psRes).nextID as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Return the ID number for an ID string */
/* Return the ID number for an ID string */
#[no_mangle]
pub unsafe extern "C" fn strresGetIDNum(mut psRes: *mut STR_RES,
                                        mut pIDStr: *mut STRING,
                                        mut pIDNum: *mut UDWORD) -> BOOL {
    let mut psID: *mut STR_ID = 0 as *mut STR_ID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              208 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"strresGetIDNum\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psID = treapFind((*psRes).psIDTreap, pIDStr as UDWORD) as *mut STR_ID;
    if psID.is_null() {
        *pIDNum = 0 as libc::c_int as UDWORD;
        return 0 as libc::c_int
    }
    if (*psID).id & 0x80000000 as libc::c_uint != 0 {
        *pIDNum = (*psID).id & !(0x80000000 as libc::c_uint)
    } else { *pIDNum = (*psID).id }
    return 1 as libc::c_int;
}
/* Return the stored ID string that matches the string passed in */
/* Return the ID stored ID string that matches the string passed in */
#[no_mangle]
pub unsafe extern "C" fn strresGetIDString(mut psRes: *mut STR_RES,
                                           mut pIDStr: *mut STRING,
                                           mut ppStoredID: *mut *mut STRING)
 -> BOOL {
    let mut psID: *mut STR_ID = 0 as *mut STR_ID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              235 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"strresGetIDString\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psID = treapFind((*psRes).psIDTreap, pIDStr as UDWORD) as *mut STR_ID;
    if psID.is_null() {
        *ppStoredID = 0 as *mut STRING;
        return 0 as libc::c_int
    }
    *ppStoredID = (*psID).pIDStr;
    return 1 as libc::c_int;
}
/* Store a string */
/* Store a string */
#[no_mangle]
pub unsafe extern "C" fn strresStoreString(mut psRes: *mut STR_RES,
                                           mut pID: *mut STRING,
                                           mut pString: *mut STRING) -> BOOL {
    let mut psID: *mut STR_ID = 0 as *mut STR_ID;
    let mut pNew: *mut STRING = 0 as *mut STRING;
    let mut psBlock: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    let mut id: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              259 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"strresStoreString\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Find the id for the string
    psID = treapFind((*psRes).psIDTreap, pID as UDWORD) as *mut STR_ID;
    if psID.is_null() {
        // No ID yet so generate a new one
        psID =
            memMallocRelease(::std::mem::size_of::<STR_ID>() as libc::c_ulong)
                as *mut STR_ID;
        if psID.is_null() {
            debug(LOG_ERROR,
                  b"strresStoreString: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        (*psID).pIDStr =
            memMallocRelease((::std::mem::size_of::<STRING>() as
                                  libc::c_ulong).wrapping_mul(stringLen(pID).wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)))
                as *mut STRING;
        if (*psID).pIDStr.is_null() {
            debug(LOG_ERROR,
                  b"strresStoreString: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        stringCpy((*psID).pIDStr, pID);
        (*psID).id = (*psRes).nextID | 0x80000000 as libc::c_uint;
        (*psRes).nextID =
            ((*psRes).nextID as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        treapAdd((*psRes).psIDTreap, (*psID).pIDStr as UDWORD,
                 psID as *mut libc::c_void);
    }
    id = (*psID).id & !(0x80000000 as libc::c_uint);
    // Find the block to store the string in
    psBlock = (*psRes).psStrings;
    while (*psBlock).idEnd < id {
        if (*psBlock).psNext.is_null() {
            // Need to allocate a new string block
            if strresAllocBlock(&mut (*psBlock).psNext, (*psRes).ext) == 0 {
                return 0 as libc::c_int
            }
            (*(*psBlock).psNext).idStart =
                (*psBlock).idEnd.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint);
            (*(*psBlock).psNext).idEnd =
                (*psBlock).idEnd.wrapping_add((*psRes).ext);
            (*(*psBlock).psNext).psNext = 0 as *mut _str_block
        }
        psBlock = (*psBlock).psNext
    }
    // Put the new string in the string block
    if !(*(*psBlock).apStrings.offset((*psID).id.wrapping_sub((*psBlock).idStart)
                                          as isize)).is_null() {
        debug(LOG_ERROR,
              b"strresStoreString: Duplicate string for id: %s\x00" as
                  *const u8 as *const libc::c_char, (*psID).pIDStr);
        abort();
    }
    // Allocate a copy of the string
    pNew =
        memMallocRelease((::std::mem::size_of::<STRING>() as
                              libc::c_ulong).wrapping_mul(stringLen(pString).wrapping_add(1
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)))
            as *mut STRING;
    if pNew.is_null() {
        debug(LOG_ERROR,
              b"strresStoreString: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    stringCpy(pNew, pString);
    let ref mut fresh1 =
        *(*psBlock).apStrings.offset((*psID).id.wrapping_sub((*psBlock).idStart)
                                         as isize);
    *fresh1 = pNew;
    return 1 as libc::c_int;
}
/* Get the string from an ID number */
/* Get the string from an ID number */
#[no_mangle]
pub unsafe extern "C" fn strresGetString(mut psRes: *mut STR_RES,
                                         mut id: UDWORD) -> *mut STRING {
    let mut psBlock: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresLoadFixedID: Invalid string res pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              334 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"strresGetString\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // find the block the string is in
    psBlock = (*psRes).psStrings;
    while !psBlock.is_null() && (*psBlock).idEnd < id {
        psBlock = (*psBlock).psNext
    }
    if psBlock.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"strresGetString: String not found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"strres.c\x00" as *const u8 as *const libc::c_char,
                  343 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"strresGetString\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        // Return the default string
        return *(*(*psRes).psStrings).apStrings.offset(0 as libc::c_int as
                                                           isize)
    }
    if !(*(*psBlock).apStrings.offset(id.wrapping_sub((*psBlock).idStart) as
                                          isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"strresGetString: String not found\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*(*psBlock).apStrings.offset(id.wrapping_sub((*psBlock).idStart) as
                                          isize)).is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              349 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"strresGetString\x00")).as_ptr(),
              b"psBlock->apStrings[id - psBlock->idStart] != NULL\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psBlock).apStrings.offset(id.wrapping_sub((*psBlock).idStart) as
                                         isize)).is_null() {
        // Return the default string
        return *(*(*psRes).psStrings).apStrings.offset(0 as libc::c_int as
                                                           isize)
    }
    return *(*psBlock).apStrings.offset(id.wrapping_sub((*psBlock).idStart) as
                                            isize);
}
/* Load a string resource file */
/* Load a string resource file */
#[no_mangle]
pub unsafe extern "C" fn strresLoad(mut psRes: *mut STR_RES,
                                    mut pData: *mut libc::c_char,
                                    mut size: UDWORD) -> BOOL {
    psCurrRes = psRes;
    strresSetInputBuffer(pData, size);
    if strres_parse() != 0 as libc::c_int { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* Return the the length of a STRING */
/* Return the the length of a STRING */
#[no_mangle]
pub unsafe extern "C" fn stringLen(mut pStr: *mut STRING) -> UDWORD {
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    loop  {
        let fresh2 = pStr;
        pStr = pStr.offset(1);
        if !(*fresh2 != 0) { break ; }
        count =
            (count as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    return count;
}
/* Copy a STRING */
/* Copy a STRING */
#[no_mangle]
pub unsafe extern "C" fn stringCpy(mut pDest: *mut STRING,
                                   mut pSrc: *mut STRING) {
    loop  {
        let fresh3 = pDest;
        pDest = pDest.offset(1);
        *fresh3 = *pSrc;
        let fresh4 = pSrc;
        pSrc = pSrc.offset(1);
        if !(*fresh4 != 0) { break ; }
    };
}
/* Get the ID number for a string*/
/* Get the ID number for a string*/
#[no_mangle]
pub unsafe extern "C" fn strresGetIDfromString(mut psRes: *mut STR_RES,
                                               mut pString: *mut STRING)
 -> UDWORD {
    let mut psBlock: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    let mut psNext: *mut STR_BLOCK = 0 as *mut STR_BLOCK;
    let mut i: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"strresGetID: Invalid string res pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"strres.c\x00" as *const u8 as *const libc::c_char,
              409 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"strresGetIDfromString\x00")).as_ptr(),
              b"PTRVALID(psRes, sizeof(STR_RES))\x00" as *const u8 as
                  *const libc::c_char);
    };
    // Search through all the blocks to find the string
    psBlock = (*psRes).psStrings;
    while !psBlock.is_null() {
        i = (*psBlock).idStart;
        while i <= (*psBlock).idEnd {
            if !(*(*psBlock).apStrings.offset(i.wrapping_sub((*psBlock).idStart)
                                                  as isize)).is_null() {
                if strcmp(*(*psBlock).apStrings.offset(i.wrapping_sub((*psBlock).idStart)
                                                           as isize), pString)
                       == 0 {
                    return i
                }
            }
            i = i.wrapping_add(1)
        }
        psNext = (*psBlock).psNext;
        psBlock = psNext
    }
    return 0 as libc::c_int as UDWORD;
}
