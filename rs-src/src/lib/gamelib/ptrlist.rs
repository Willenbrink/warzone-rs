use ::libc;
extern "C" {
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
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LISTNODE {
    pub psElement: *mut libc::c_void,
    pub sdwKey: SDWORD,
    pub psNext: *mut LISTNODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sPTRLIST {
    pub psNodeHeap: *mut OBJ_HEAP,
    pub psElementHeap: *mut OBJ_HEAP,
    pub psNode: *mut LISTNODE,
    pub psCurNode: *mut LISTNODE,
    pub udwTableSize: UDWORD,
    pub udwElements: UDWORD,
    pub udwExtElements: UDWORD,
    pub udwElementSize: UDWORD,
    pub sdwCurIndex: UDWORD,
    pub bDontGetNext: BOOL,
}
pub type PTRLIST = sPTRLIST;
// Extension memory for the heap
/* **************************************************************************/
/* functions
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_Create(mut ppsList: *mut *mut PTRLIST,
                                        mut udwInitElements: UDWORD,
                                        mut udwExtElements: UDWORD,
                                        mut udwElementSize: UDWORD) -> BOOL {
    /* create ptr list struct */
    *ppsList =
        memMallocRelease(::std::mem::size_of::<PTRLIST>() as libc::c_ulong) as
            *mut PTRLIST;
    /* allocate heaps */
    if heapCreate(&mut (**ppsList).psNodeHeap,
                  ::std::mem::size_of::<LISTNODE>() as libc::c_ulong,
                  udwInitElements, udwExtElements) == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut (**ppsList).psElementHeap, udwElementSize,
                  udwInitElements, udwExtElements) == 0 {
        return 0 as libc::c_int
    }
    /* init members */
    (**ppsList).udwElements = udwInitElements;
    (**ppsList).udwExtElements = udwExtElements;
    (**ppsList).udwElementSize = udwElementSize;
    ptrList_Init(*ppsList);
    //ffs		//Not really needed I guess?  -Qamly
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_Destroy(mut ptrList: *mut PTRLIST) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_Destroy: list pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              63 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"ptrList_Destroy\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    ptrList_Clear(ptrList);
    /* destroy heaps */
    heapDestroy((*ptrList).psNodeHeap);
    heapDestroy((*ptrList).psElementHeap);
    /* free struct */
    memFreeRelease(ptrList as *mut libc::c_void);
    ptrList = 0 as *mut PTRLIST;
    // Not really needed I guess?  -Qamly
}
/* **************************************************************************/
// ffs			//Not really needed I guess?  [Don't delete yet, possible future use?] -Qamly
/* **************************************************************************/
/* **************************************************************************/
unsafe extern "C" fn ptrList_Init(mut ptrList: *mut PTRLIST) {
    (*ptrList).sdwCurIndex = 0 as libc::c_int as UDWORD;
    (*ptrList).psCurNode = 0 as *mut LISTNODE;
    (*ptrList).psNode = 0 as *mut LISTNODE;
    (*ptrList).bDontGetNext = 0 as libc::c_int;
}
/* **************************************************************************/
/*
 * ptrList_Clear
 *
 * Returns all nodes from hash table to free node list
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_Clear(mut ptrList: *mut PTRLIST) {
    let mut psNode: *mut LISTNODE = 0 as *mut LISTNODE;
    let mut psNodeTmp: *mut LISTNODE = 0 as *mut LISTNODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_Destroy: table pointer invalid\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              103 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"ptrList_Clear\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* free nodes */
    psNode = (*ptrList).psNode;
    while !psNode.is_null() {
        /* return node element to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_Destroy: element pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  112 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"ptrList_Clear\x00")).as_ptr(),
                  b"PTRVALID(psNode->psElement, ptrList->udwElementSize)\x00"
                      as *const u8 as *const libc::c_char);
        };
        heapFree((*ptrList).psElementHeap, (*psNode).psElement);
        /* return node to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_Destroy: node pointer invalid\n\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  117 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"ptrList_Clear\x00")).as_ptr(),
                  b"PTRVALID(psNode, sizeof(LISTNODE))\x00" as *const u8 as
                      *const libc::c_char);
        };
        psNodeTmp = (*psNode).psNext;
        heapFree((*ptrList).psNodeHeap, psNode as *mut libc::c_void);
        psNode = psNodeTmp
    }
    ptrList_Init(ptrList);
}
/* **************************************************************************/
/*
 * ptrList_GetElement
 *
 * Gets free node from heap and returns element pointer
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_GetElement(mut ptrList: *mut PTRLIST)
 -> *mut libc::c_void {
    let mut psElement: *mut libc::c_void = 0 as *mut libc::c_void;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_GetElement: table pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              140 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"ptrList_GetElement\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    heapAlloc((*ptrList).psElementHeap, &mut psElement);
    return psElement;
}
/* **************************************************************************/
/*
 * ptrList_FreeElement
 *
 * Free element that was allocated using ptrList_GetElement without
 * inserting in list: will fail if element not allocated from ptrList
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_FreeElement(mut ptrList: *mut PTRLIST,
                                             mut psElement:
                                                 *mut libc::c_void) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_FreeElement: table pointer invalid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              160 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"ptrList_FreeElement\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if heapFree((*ptrList).psElementHeap, psElement) == 0 as libc::c_int {
        debug(LOG_NEVER,
              b"ptrList_FreeElement: couldn\'t free element\n\x00" as
                  *const u8 as *const libc::c_char);
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_InsertElement(mut ptrList: *mut PTRLIST,
                                               mut psElement:
                                                   *mut libc::c_void,
                                               mut sdwKey: SDWORD) {
    let mut psNode: *mut LISTNODE = 0 as *mut LISTNODE;
    let mut psCurNode: *mut LISTNODE = 0 as *mut LISTNODE;
    let mut psPrevNode: *mut LISTNODE = 0 as *mut LISTNODE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_InsertElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              176 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"ptrList_InsertElement\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_InsertElement: element pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              178 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"ptrList_InsertElement\x00")).as_ptr(),
              b"PTRVALID(psElement, ptrList->udwElementSize)\x00" as *const u8
                  as *const libc::c_char);
    };
    /* get node from heap */
    heapAlloc((*ptrList).psNodeHeap,
              &mut psNode as *mut *mut LISTNODE as *mut libc::c_void as
                  *mut *mut libc::c_void);
    /* set node elements */
    (*psNode).sdwKey = sdwKey;
    (*psNode).psElement = psElement;
    (*psNode).psNext = 0 as *mut LISTNODE;
    psPrevNode = 0 as *mut LISTNODE;
    psCurNode = (*ptrList).psNode;
    //ffs	//Not really needed I guess?  -Qamly
    /* find correct position to insert node */
    while !psCurNode.is_null() {
        if (*psCurNode).sdwKey < sdwKey { break ; }
        psPrevNode = psCurNode;
        psCurNode = (*psCurNode).psNext
    }
    /* insert node */
    if psPrevNode.is_null() {
        (*ptrList).psNode = psNode;
        if !psCurNode.is_null() { (*psNode).psNext = psCurNode }
    } else { (*psPrevNode).psNext = psNode; (*psNode).psNext = psCurNode };
    // ffs	//Not really needed I guess?  -Qamly
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_RemoveElement(mut ptrList: *mut PTRLIST,
                                               mut psElement:
                                                   *mut libc::c_void,
                                               mut sdwKey: SDWORD) -> BOOL {
    let mut psCurNode: *mut LISTNODE = 0 as *mut LISTNODE;
    let mut psPrevNode: *mut LISTNODE = 0 as *mut LISTNODE;
    let mut bOK: BOOL = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"ptrList_RemoveElement: table pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
              236 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"ptrList_RemoveElement\x00")).as_ptr(),
              b"PTRVALID(ptrList, sizeof(PTRLIST))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psPrevNode = 0 as *mut LISTNODE;
    psCurNode = (*ptrList).psNode;
    // ffs	//Not really needed I guess?  -Qamly
    /* find correct position to insert node */
    while !psCurNode.is_null() &&
              !((*psCurNode).sdwKey == sdwKey &&
                    (*psCurNode).psElement == psElement) {
        psPrevNode = psCurNode;
        psCurNode = (*psCurNode).psNext
    }
    /* remove node from hash table and return to heap */
    if psCurNode.is_null() {
        bOK = 0 as libc::c_int
    } else {
        if (*psCurNode).psElement == psElement {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_RemoveElement: removing wrong element!\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*psCurNode).psElement == psElement {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  262 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"ptrList_RemoveElement\x00")).as_ptr(),
                  b"psCurNode->psElement == psElement\x00" as *const u8 as
                      *const libc::c_char);
        };
        /* remove from list */
        if psPrevNode.is_null() {
            (*ptrList).psNode = (*psCurNode).psNext
        } else { (*psPrevNode).psNext = (*psCurNode).psNext }
        /* check whether table current node pointer is this node */
        if (*ptrList).psCurNode == psCurNode {
            /* point it to the previous node if valid */
            if psPrevNode.is_null() {
                /* set next node and set flag */
                (*ptrList).psCurNode = (*psCurNode).psNext;
                (*ptrList).bDontGetNext = 1 as libc::c_int
            } else { (*ptrList).psCurNode = psPrevNode }
        }
        /* return element to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_RemoveElement: element pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  292 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"ptrList_RemoveElement\x00")).as_ptr(),
                  b"PTRVALID(psCurNode->psElement, ptrList->udwElementSize)\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psCurNode).psElement == psElement {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_RemoveElement: removing wrong element!\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*psCurNode).psElement == psElement {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  294 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"ptrList_RemoveElement\x00")).as_ptr(),
                  b"psCurNode->psElement == psElement\x00" as *const u8 as
                      *const libc::c_char);
        };
        heapFree((*ptrList).psElementHeap, (*psCurNode).psElement);
        /* return node to heap */
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"ptrList_RemoveElement: node pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ptrlist.c\x00" as *const u8 as *const libc::c_char,
                  299 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"ptrList_RemoveElement\x00")).as_ptr(),
                  b"PTRVALID(psCurNode, sizeof(LISTNODE))\x00" as *const u8 as
                      *const libc::c_char);
        };
        heapFree((*ptrList).psNodeHeap, psCurNode as *mut libc::c_void);
        bOK = 1 as libc::c_int
    }
    // ffs	//Not really needed I guess?  -Qamly
    return bOK;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_GetNext(mut ptrList: *mut PTRLIST)
 -> *mut libc::c_void {
    let mut pElement: *mut libc::c_void = 0 as *mut libc::c_void;
    // ffs	//Not really needed I guess?  -Qamly
    if ptrList.is_null() { pElement = 0 as *mut libc::c_void }
    if (*ptrList).psCurNode.is_null() {
        pElement = 0 as *mut libc::c_void
    } else {
        if (*ptrList).bDontGetNext == 1 as libc::c_int {
            (*ptrList).bDontGetNext = 0 as libc::c_int
        } else { (*ptrList).psCurNode = (*(*ptrList).psCurNode).psNext }
        if (*ptrList).psCurNode.is_null() {
            pElement = 0 as *mut libc::c_void
        } else { pElement = (*(*ptrList).psCurNode).psElement }
    }
    // ffs	//Not really needed I guess?  -Qamly
    return pElement;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn ptrList_GetFirst(mut ptrList: *mut PTRLIST)
 -> *mut libc::c_void {
    let mut pElement: *mut libc::c_void = 0 as *mut libc::c_void;
    // ffs	//Not really needed I guess?  -Qamly
    (*ptrList).bDontGetNext = 0 as libc::c_int;
    (*ptrList).psCurNode = (*ptrList).psNode;
    if (*ptrList).psCurNode.is_null() {
        pElement = 0 as *mut libc::c_void
    } else { pElement = (*(*ptrList).psCurNode).psElement }
    // ffs	//Not really needed I guess?  -Qamly
    return pElement;
}
/* **************************************************************************/
