use ::libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
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
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type UINT = libc::c_uint;
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
// Extension memory for the heap
/* **************************************************************************/
/* macros
 */
pub type HASHFUNC
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> UDWORD>;
pub type HASHFREEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHNODE {
    pub iKey1: libc::c_int,
    pub iKey2: libc::c_int,
    pub psElement: *mut libc::c_void,
    pub psNext: *mut HASHNODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHTABLE {
    pub psNodeHeap: *mut OBJ_HEAP,
    pub psElementHeap: *mut OBJ_HEAP,
    pub ppsNode: *mut *mut HASHNODE,
    pub psNextNode: *mut HASHNODE,
    pub pHashFunc: HASHFUNC,
    pub pFreeFunc: HASHFREEFUNC,
    pub udwTableSize: UDWORD,
    pub udwElements: UDWORD,
    pub udwExtElements: UDWORD,
    pub udwElementSize: UDWORD,
    pub sdwCurIndex: UDWORD,
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn HashTest(mut iKey1: libc::c_int,
                                  mut iKey2: libc::c_int) -> UINT {
    return (iKey1 as UINT).wrapping_add(iKey2 as libc::c_uint);
}
/* **************************************************************************/
/*
 * HashPJW
 *
 * Adaptation of Peter Weinberger's (PJW) generic hashing algorithm listed
 * in Binstock+Rex, "Practical Algorithms" p 69.
 *
 * Accepts element pointer and returns hashed integer.
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn HashPJW(mut iKey1: libc::c_int,
                                 mut iKey2: libc::c_int) -> UINT {
    let mut iHashValue: UINT = 0;
    let mut i: UINT = 0;
    let mut c: *mut CHAR = iKey1 as *mut CHAR;
    /* don't use second key in this one */
    iKey2 = -(747 as libc::c_int);
    iHashValue = 0 as libc::c_int as UINT;
    while *c != 0 {
        iHashValue =
            (iHashValue <<
                 (32 as libc::c_int / 8 as libc::c_int) as
                     UINT).wrapping_add(*c as libc::c_uint);
        i =
            iHashValue &
                !(0xffffffff as libc::c_uint >>
                      (32 as libc::c_int / 8 as libc::c_int) as UINT);
        if i != 0 as libc::c_int as libc::c_uint {
            iHashValue =
                (iHashValue ^
                     i >>
                         (32 as libc::c_int * 3 as libc::c_int /
                              4 as libc::c_int) as UINT) &
                    !!(0xffffffff as libc::c_uint >>
                           (32 as libc::c_int / 8 as libc::c_int) as UINT)
        }
        c = c.offset(1)
    }
    return iHashValue;
}
/* **************************************************************************/
/* functions
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_Create(mut ppsTable: *mut *mut HASHTABLE,
                                          mut udwTableSize: UDWORD,
                                          mut udwInitElements: UDWORD,
                                          mut udwExtElements: UDWORD,
                                          mut udwElementSize: UDWORD)
 -> BOOL {
    let mut udwSize: UDWORD = 0;
    /* allocate and init table */
    *ppsTable =
        memMallocRelease(::std::mem::size_of::<HASHTABLE>() as libc::c_ulong)
            as *mut HASHTABLE;
    udwSize =
        udwTableSize.wrapping_mul(::std::mem::size_of::<*mut HASHNODE>() as
                                      libc::c_ulong);
    (**ppsTable).ppsNode = memMallocRelease(udwSize) as *mut *mut HASHNODE;
    memset((**ppsTable).ppsNode as *mut libc::c_void, 0 as libc::c_int,
           udwSize);
    /* allocate heaps */
    if heapCreate(&mut (**ppsTable).psNodeHeap,
                  ::std::mem::size_of::<HASHNODE>() as libc::c_ulong,
                  udwInitElements, udwExtElements) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut (**ppsTable).psElementHeap, udwElementSize,
                  udwInitElements, udwExtElements) == 0 {
        //		my_error("",0,"","htc FAIL!\n");
        return 0 as libc::c_int
    }
    /* init members */
    (**ppsTable).udwTableSize = udwTableSize;
    (**ppsTable).udwElements = udwInitElements;
    (**ppsTable).udwExtElements = udwExtElements;
    (**ppsTable).udwElementSize = udwElementSize;
    (**ppsTable).sdwCurIndex = 0 as libc::c_int as UDWORD;
    (**ppsTable).psNextNode =
        *(**ppsTable).ppsNode.offset(0 as libc::c_int as isize);
    (**ppsTable).pFreeFunc = None;
    /* set hash function to internal */
    hashTable_SetHashFunction(*ppsTable,
                              Some(HashTest as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int)
                                           -> UINT));
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_Destroy(mut psTable: *mut HASHTABLE) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_Destroy: table pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              121 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"hashTable_Destroy\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    hashTable_Clear(psTable);
    /* destroy heaps */
    heapDestroy((*psTable).psNodeHeap);
    heapDestroy((*psTable).psElementHeap);
    /* free table */
    memFreeRelease((*psTable).ppsNode as *mut libc::c_void);
    (*psTable).ppsNode = 0 as *mut *mut HASHNODE;
    memFreeRelease(psTable as *mut libc::c_void);
    psTable = 0 as *mut HASHTABLE;
}
/* **************************************************************************/
/*
 * hashTable_Clear
 *
 * Returns all nodes from hash table to free node list
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_Clear(mut psTable: *mut HASHTABLE) {
    let mut psNode: *mut HASHNODE = 0 as *mut HASHNODE;
    let mut psNodeTmp: *mut HASHNODE = 0 as *mut HASHNODE;
    let mut i: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_Destroy: table pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              149 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"hashTable_Clear\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* free nodes */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psTable).udwTableSize {
        /* free table entry nodelist */
        psNode = *(*psTable).ppsNode.offset(i as isize);
        while !psNode.is_null() {
            /* return node element to heap */
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"hashTable_Destroy: element pointer invalid\n\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
                      160 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"hashTable_Clear\x00")).as_ptr(),
                      b"PTRVALID(psNode->psElement, psTable->udwElementSize)\x00"
                          as *const u8 as *const libc::c_char);
            };
            /* do free-element callback if set */
            if (*psTable).pFreeFunc.is_some() {
                (*psTable).pFreeFunc.expect("non-null function pointer")((*psNode).psElement);
            }
            /* free element */
            heapFree((*psTable).psElementHeap, (*psNode).psElement);
            /* return node to heap */
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"hashTable_Destroy: node pointer invalid\n\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
                      173 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"hashTable_Clear\x00")).as_ptr(),
                      b"PTRVALID(psNode, sizeof(HASHNODE))\x00" as *const u8
                          as *const libc::c_char);
            };
            psNodeTmp = (*psNode).psNext;
            heapFree((*psTable).psNodeHeap, psNode as *mut libc::c_void);
            psNode = psNodeTmp
        }
        /* set table entry to NULL */
        let ref mut fresh0 = *(*psTable).ppsNode.offset(i as isize);
        *fresh0 = 0 as *mut HASHNODE;
        i = i.wrapping_add(1)
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_SetHashFunction(mut psTable:
                                                       *mut HASHTABLE,
                                                   mut pHashFunc: HASHFUNC) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_SetHashFunction: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              190 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"hashTable_SetHashFunction\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psTable).pHashFunc = pHashFunc;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_SetFreeElementFunction(mut psTable:
                                                              *mut HASHTABLE,
                                                          mut pFreeFunc:
                                                              HASHFREEFUNC) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_SetFreeElementFunction: table pointer invalid\n\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              202 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 33],
                                        &[libc::c_char; 33]>(b"hashTable_SetFreeElementFunction\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psTable).pFreeFunc = pFreeFunc;
}
/* **************************************************************************/
/*
 * hashTable_GetElement
 *
 * Gets free node from heap and returns element pointer
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_GetElement(mut psTable: *mut HASHTABLE)
 -> *mut libc::c_void {
    let mut psElement: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut result: BOOL = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_GetElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              222 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"hashTable_GetElement\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    result = heapAlloc((*psTable).psElementHeap, &mut psElement);
    // if the alloc fails then return NULL
    if result == 0 as libc::c_int { return 0 as *mut libc::c_void }
    return psElement;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_GetHashKey(mut psTable: *mut HASHTABLE,
                                              mut iKey1: libc::c_int,
                                              mut iKey2: libc::c_int)
 -> UDWORD {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_GetFirst: hash table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              239 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"hashTable_GetHashKey\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* get hashed index */
    return (*psTable).pHashFunc.expect("non-null function pointer")(iKey1,
                                                                    iKey2).wrapping_rem((*psTable).udwTableSize);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_InsertElement(mut psTable: *mut HASHTABLE,
                                                 mut psElement:
                                                     *mut libc::c_void,
                                                 mut iKey1: libc::c_int,
                                                 mut iKey2: libc::c_int) {
    let mut udwHashIndex: UDWORD = 0;
    let mut psNode: *mut HASHNODE = 0 as *mut HASHNODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_InsertElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              255 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"hashTable_InsertElement\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_InsertElement: element pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              257 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"hashTable_InsertElement\x00")).as_ptr(),
              b"PTRVALID(psElement, psTable->udwElementSize)\x00" as *const u8
                  as *const libc::c_char);
    };
    /* get hashed index */
    udwHashIndex = hashTable_GetHashKey(psTable, iKey1, iKey2);
    /* get node from heap */
    heapAlloc((*psTable).psNodeHeap,
              &mut psNode as *mut *mut HASHNODE as *mut libc::c_void as
                  *mut *mut libc::c_void);
    /* set node elements */
    (*psNode).iKey1 = iKey1;
    (*psNode).iKey2 = iKey2;
    (*psNode).psElement = psElement;
    /* add new node to head of list */
    (*psNode).psNext = *(*psTable).ppsNode.offset(udwHashIndex as isize);
    let ref mut fresh1 = *(*psTable).ppsNode.offset(udwHashIndex as isize);
    *fresh1 = psNode;
}
/* **************************************************************************/
/*
 * hashTable_FindElement
 *
 * Calculates hash index from keys and returns element in hash table
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_FindElement(mut psTable: *mut HASHTABLE,
                                               mut iKey1: libc::c_int,
                                               mut iKey2: libc::c_int)
 -> *mut libc::c_void {
    let mut udwHashIndex: UDWORD = 0;
    let mut psNode: *mut HASHNODE = 0 as *mut HASHNODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_FindElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              290 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"hashTable_FindElement\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* get hashed index */
    udwHashIndex = hashTable_GetHashKey(psTable, iKey1, iKey2);
    /* check hash index within bounds */
    if udwHashIndex >= 0 as libc::c_int as libc::c_uint &&
           udwHashIndex < (*psTable).udwTableSize {
    } else {
        debug(LOG_ERROR,
              b"hashTable_GetElement: hash value %i too large for table size %i\n\x00"
                  as *const u8 as *const libc::c_char, udwHashIndex,
              (*psTable).udwTableSize);
    };
    if udwHashIndex >= 0 as libc::c_int as libc::c_uint &&
           udwHashIndex < (*psTable).udwTableSize {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              298 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"hashTable_FindElement\x00")).as_ptr(),
              b"udwHashIndex>=0 && udwHashIndex<psTable->udwTableSize\x00" as
                  *const u8 as *const libc::c_char);
    };
    psNode = *(*psTable).ppsNode.offset(udwHashIndex as isize);
    /* loop through node list to find element match */
    while !psNode.is_null() &&
              !((*psNode).iKey1 == iKey1 && (*psNode).iKey2 == iKey2) {
        psNode = (*psNode).psNext
    }
    /* remove node from hash table and return to heap */
    if psNode.is_null() {
        return 0 as *mut libc::c_void
    } else { return (*psNode).psElement };
}
/* **************************************************************************/
unsafe extern "C" fn hashTable_SetNextNode(mut psTable: *mut HASHTABLE,
                                           mut bMoveToNextNode: BOOL) {
    if bMoveToNextNode == 1 as libc::c_int && !(*psTable).psNextNode.is_null()
       {
        /* get next node */
        (*psTable).psNextNode = (*(*psTable).psNextNode).psNext;
        /* if next node NULL increment index */
        if (*psTable).psNextNode.is_null() {
            (*psTable).sdwCurIndex = (*psTable).sdwCurIndex.wrapping_add(1)
        }
    }
    /* search through table for next allocated node */
    while (*psTable).sdwCurIndex < (*psTable).udwTableSize &&
              (*psTable).psNextNode.is_null() {
        (*psTable).psNextNode =
            *(*psTable).ppsNode.offset((*psTable).sdwCurIndex as isize);
        if (*psTable).psNextNode.is_null() {
            (*psTable).sdwCurIndex = (*psTable).sdwCurIndex.wrapping_add(1)
        }
    }
    /* reset pointer if table overrun */
    if (*psTable).sdwCurIndex >= (*psTable).udwTableSize {
        (*psTable).psNextNode = 0 as *mut HASHNODE
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_RemoveElement(mut psTable: *mut HASHTABLE,
                                                 mut psElement:
                                                     *mut libc::c_void,
                                                 mut iKey1: libc::c_int,
                                                 mut iKey2: libc::c_int)
 -> BOOL {
    let mut udwHashIndex: UDWORD = 0;
    let mut psNode: *mut HASHNODE = 0 as *mut HASHNODE;
    let mut psPrev: *mut HASHNODE = 0 as *mut HASHNODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_RemoveElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              365 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"hashTable_RemoveElement\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* get hashed index */
    udwHashIndex = hashTable_GetHashKey(psTable, iKey1, iKey2);
    /* init previous node pointer */
    psPrev = 0 as *mut HASHNODE;
    /* get first node in table slot */
    psNode = *(*psTable).ppsNode.offset(udwHashIndex as isize);
    /* loop through node list to find element match */
    while !psNode.is_null() &&
              !(psElement == (*psNode).psElement && (*psNode).iKey1 == iKey1
                    && (*psNode).iKey2 == iKey2) {
        psPrev = psNode;
        psNode = (*psNode).psNext
    }
    /* remove node from hash table and return to heap */
    if psNode.is_null() {
        return 0 as libc::c_int
    } else {
        /* remove from hash table */
        if psPrev.is_null() {
            let ref mut fresh2 =
                *(*psTable).ppsNode.offset(udwHashIndex as isize);
            *fresh2 = (*psNode).psNext
        } else { (*psPrev).psNext = (*psNode).psNext }
        /* set next node pointer to this one if necessary */
        if (*psTable).psNextNode == psNode { (*psTable).psNextNode = psPrev }
        /* setup next node pointer */
        hashTable_SetNextNode(psTable, 1 as libc::c_int);
        /* return element to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"hashTable_RemoveElement: element pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
                  412 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"hashTable_RemoveElement\x00")).as_ptr(),
                  b"PTRVALID(psNode->psElement, psTable->udwElementSize)\x00"
                      as *const u8 as *const libc::c_char);
        };
        heapFree((*psTable).psElementHeap, (*psNode).psElement);
        /* return node to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"hashTable_RemoveElement: node pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
                  417 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"hashTable_RemoveElement\x00")).as_ptr(),
                  b"PTRVALID(psNode, sizeof(HASHNODE))\x00" as *const u8 as
                      *const libc::c_char);
        };
        heapFree((*psTable).psNodeHeap, psNode as *mut libc::c_void);
        return 1 as libc::c_int
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_GetNext(mut psTable: *mut HASHTABLE)
 -> *mut libc::c_void {
    let mut psElement: *mut libc::c_void = 0 as *mut libc::c_void;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_GetNext: hash table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              432 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"hashTable_GetNext\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTable).psNextNode.is_null() {
        return 0 as *mut libc::c_void
    } else {
        psElement = (*(*psTable).psNextNode).psElement;
        /* setup next node pointer */
        hashTable_SetNextNode(psTable, 1 as libc::c_int);
        return psElement
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn hashTable_GetFirst(mut psTable: *mut HASHTABLE)
 -> *mut libc::c_void {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"hashTable_GetFirst: hash table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"hashtabl.c\x00" as *const u8 as *const libc::c_char,
              455 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"hashTable_GetFirst\x00")).as_ptr(),
              b"PTRVALID(psTable, sizeof(HASHTABLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* init current index and node to start of table */
    (*psTable).sdwCurIndex = 0 as libc::c_int as UDWORD;
    (*psTable).psNextNode =
        *(*psTable).ppsNode.offset(0 as libc::c_int as isize);
    /* search through table for first allocated node */
    hashTable_SetNextNode(psTable, 0 as libc::c_int);
    /* return it */
    return hashTable_GetNext(psTable);
}
/* **************************************************************************/
