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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    #[no_mangle]
    fn abort() -> !;
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
pub type BLOCK_HEAP = _block_heap;
// The size of the objects being stored on the heap
// The initial number of objects allocated
// The number of objects to allocate after the initial
// allocation is used up
// which block heap (if any) this object heap was allocated from
// The currently free objects
// The main memory heap
// Extension memory for the heap
/*
 * Heap.c
 *
 * Memory routines for managing groups of the same sized objects.
 *
 */
/* Allow frame header files to be singly included */
// Control whether a heap usage report is printed out when a heap is destroyed
// Control whether a copy for the filename string is taken in DEBUG mode
/* Store the call position */
static mut pCFile: *mut STRING = 0 as *const STRING as *mut STRING;
static mut cPos: SDWORD = 0;
/* ***************************************************************************************/
/*                           Function Protoypes                                         */
/*                                                                                      */
/*      These should not be called directly - use the macros below                      */
/* Store the location in C code at which a call to the heap was made */
/* Store the location in C code at which a call to the heap was made */
#[no_mangle]
pub unsafe extern "C" fn heapSetCallPos(mut pFileName: *mut STRING,
                                        mut lineNumber: SDWORD) {
    cPos = lineNumber;
    pCFile = pFileName;
}
/* Create the free list for a heap */
unsafe extern "C" fn heapCreateFreeList(mut psHeap: *mut OBJ_HEAP) {
    let mut size: UDWORD = (*psHeap).objSize;
    let mut ext: UDWORD = (*psHeap).extAlloc;
    let mut i: UDWORD = 0;
    let mut extSize: UDWORD = 0;
    let mut psCurr: *mut FREE_OBJECT = 0 as *mut FREE_OBJECT;
    let mut pBase: *mut UBYTE = 0 as *mut UBYTE;
    let mut psExt: *mut HEAP_EXTENSION = 0 as *mut HEAP_EXTENSION;
    // Set up the main memory block
    (*psHeap).psFree = (*psHeap).pMemory as *mut FREE_OBJECT;
    pBase = (*psHeap).pMemory;
    i = (*psHeap).initAlloc;
    while i > 0 as libc::c_int as libc::c_uint {
        psCurr = pBase as *mut FREE_OBJECT;
        (*psCurr).psNext = pBase.offset(size as isize) as *mut FREE_OBJECT;
        pBase = pBase.offset(size as isize);
        i = i.wrapping_sub(1)
    }
    // Set up the extension blocks if any
    extSize = size.wrapping_mul(ext);
    psExt = (*psHeap).psExt;
    while !psExt.is_null() {
        // Link this list to the end of the free list
        (*psCurr).psNext = (*psExt).pMemory as *mut FREE_OBJECT;
        // Now create the free object list
        pBase = (*psExt).pMemory;
        i = (*psHeap).extAlloc;
        while i > 0 as libc::c_int as libc::c_uint {
            psCurr = pBase as *mut FREE_OBJECT;
            (*psCurr).psNext =
                pBase.offset(size as isize) as *mut FREE_OBJECT;
            pBase = pBase.offset(size as isize);
            i = i.wrapping_sub(1)
        }
        psExt = (*psExt).psNext
    }
    // Terminate the free list
    (*psCurr).psNext = 0 as *mut _free_object;
}
/* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
/* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
#[no_mangle]
pub unsafe extern "C" fn heapCreate(mut ppsHeap: *mut *mut OBJ_HEAP,
                                    mut size: UDWORD, mut init: UDWORD,
                                    mut ext: UDWORD) -> BOOL {
    /*	UDWORD		i;
	FREE_OBJECT	*psCurr;
	UBYTE		*pBase;*/
    if size >= ::std::mem::size_of::<FREE_OBJECT>() as libc::c_ulong {
    } else {
        debug(LOG_ERROR,
              b"heapCreate: object is too small to be stored in free list\x00"
                  as *const u8 as *const libc::c_char);
    };
    if size >= ::std::mem::size_of::<FREE_OBJECT>() as libc::c_ulong {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"heap.c\x00" as *const u8 as *const libc::c_char,
              127 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"heapCreate\x00")).as_ptr(),
              b"size >= sizeof(FREE_OBJECT)\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Allocate the heap object and its memory */
    *ppsHeap =
        memMallocRelease(::std::mem::size_of::<OBJ_HEAP>() as libc::c_ulong)
            as *mut OBJ_HEAP;
    if (*ppsHeap).is_null() {
        debug(LOG_ERROR,
              b"heapCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //	memset(*ppsHeap,0,sizeof(OBJ_HEAP));			//setting everything to 0 first (debug test)-Q
    (**ppsHeap).pMemory =
        memMallocRelease(size.wrapping_mul(init)) as *mut UBYTE;
    /*
	if (PTRVALID((*ppsHeap)->pMemory,size*init)==FALSE)
	{
		DBPRINTF(("Allocated heap memory is not valid!\n"));
	}
	else
	{
		DBPRINTF(("valid\n"));
	}
*/
    if (**ppsHeap).pMemory.is_null() {
        debug(LOG_ERROR,
              b"heapCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /* Initialise the object */
    (**ppsHeap).objSize = size;
    (**ppsHeap).initAlloc = init;
    (**ppsHeap).extAlloc = ext;
    (**ppsHeap).psExt = 0 as *mut HEAP_EXTENSION;
    (**ppsHeap).psBlkHeap = memGetBlockHeap();
    // Now create the free object list
    heapCreateFreeList(*ppsHeap);
    /*
	if (PTRVALID((*ppsHeap)->pMemory,10)==FALSE)
	{
		DBPRINTF(("Allocated heap memory is not valid!\n"));
	}
	else
	{
		DBPRINTF(("valid\n"));
	}
*/
	/*
#if DEBUG_MALLOC
	// Initialise the memory to a fixed value to check for memory overwrites
	memset((*ppsHeap)->pMemory, FREE_BYTE, size * init);
#endif

	// Now create the free object list
	(*ppsHeap)->psFree = (FREE_OBJECT *)((*ppsHeap)->pMemory);
	pBase = (*ppsHeap)->pMemory;
	for(i=0; i<init; i++)
	{
		psCurr = (FREE_OBJECT *)(pBase + i*size);
		psCurr->psNext = (FREE_OBJECT *)((UBYTE *)psCurr + size);
	}
	psCurr->psNext = NULL;
	*/
    return 1 as libc::c_int;
}
/* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
/* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
#[no_mangle]
pub unsafe extern "C" fn heapAlloc(mut psHeap: *mut OBJ_HEAP,
                                   mut ppObject: *mut *mut libc::c_void)
 -> BOOL {
    let mut psNew: *mut HEAP_EXTENSION = 0 as *mut HEAP_EXTENSION;
    let mut i: UDWORD = 0;
    let mut psCurr: *mut FREE_OBJECT = 0 as *mut FREE_OBJECT;
    let mut pBase: *mut UBYTE = 0 as *mut UBYTE;
    let mut psCurrBlk: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"heapAlloc: Invalid heap pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"heap.c\x00" as *const u8 as *const libc::c_char,
              241 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"heapAlloc\x00")).as_ptr(),
              b"PTRVALID(psHeap, sizeof(OBJ_HEAP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psHeap).psFree.is_null() {
        if (*psHeap).extAlloc == 0 as libc::c_int as libc::c_uint {
            // heap doesn't expand
            return 0 as libc::c_int
        }
        /* No objects left - need to add a heap extension */
        psCurrBlk = memGetBlockHeap();
        memSetBlockHeap((*psHeap).psBlkHeap);
        psNew =
            memMallocRelease(::std::mem::size_of::<HEAP_EXTENSION>() as
                                 libc::c_ulong) as *mut HEAP_EXTENSION;
        if psNew.is_null() {
            /* Out of memory */
            return 0 as libc::c_int
        }
        (*psNew).pMemory =
            memMallocRelease((*psHeap).objSize.wrapping_mul((*psHeap).extAlloc))
                as *mut UBYTE;
        if (*psNew).pMemory.is_null() {
            /* Out of memory */
            memFreeRelease(psNew as *mut libc::c_void);
            psNew = 0 as *mut HEAP_EXTENSION;
            return 0 as libc::c_int
        }
        memSetBlockHeap(psCurrBlk);
        /* Add the extension to the list */
        (*psNew).psNext = (*psHeap).psExt;
        (*psHeap).psExt = psNew;
        /* Now create the free object list */
        (*psHeap).psFree = (*psNew).pMemory as *mut FREE_OBJECT;
        pBase = (*psNew).pMemory;
        i = 0 as libc::c_int as UDWORD;
        while i < (*psHeap).extAlloc {
            psCurr =
                pBase.offset(i.wrapping_mul((*psHeap).objSize) as isize) as
                    *mut FREE_OBJECT;
            (*psCurr).psNext =
                (psCurr as *mut UBYTE).offset((*psHeap).objSize as isize) as
                    *mut FREE_OBJECT;
            i = i.wrapping_add(1)
        }
        (*psCurr).psNext = 0 as *mut _free_object
    }
    /* Return the object and update the free list */
    *ppObject = (*psHeap).psFree as *mut libc::c_void;
    (*psHeap).psFree = (*(*psHeap).psFree).psNext;
    return 1 as libc::c_int;
}
/* Return an object to the heap */
/* Return an object to the heap */
#[no_mangle]
pub unsafe extern "C" fn heapFree(mut psHeap: *mut OBJ_HEAP,
                                  mut pObject: *mut libc::c_void) -> BOOL {
    let mut psFree: *mut FREE_OBJECT = 0 as *mut FREE_OBJECT;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"heapFree: Invalid heap pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"heap.c\x00" as *const u8 as *const libc::c_char,
              351 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"heapFree\x00")).as_ptr(),
              b"PTRVALID(psHeap, sizeof(OBJ_HEAP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFree = pObject as *mut FREE_OBJECT;
    (*psFree).psNext = (*psHeap).psFree;
    (*psHeap).psFree = psFree;
    return 1 as libc::c_int;
}
/* Reset the heap, i.e. free all the objects in the heap */
/* Reset the heap, i.e. free all the objects in the heap */
#[no_mangle]
pub unsafe extern "C" fn heapReset(mut psHeap: *mut OBJ_HEAP) {
    /* Initialise the object */
    // Now create the free object list
    heapCreateFreeList(psHeap);
}
/* Destroy a heap and release all the memory associated with it */
/* Destroy a heap and release all the memory associated with it */
#[no_mangle]
pub unsafe extern "C" fn heapDestroy(mut psHeap: *mut OBJ_HEAP) {
    let mut psExt: *mut HEAP_EXTENSION = 0 as *mut HEAP_EXTENSION;
    let mut psNext: *mut HEAP_EXTENSION = 0 as *mut HEAP_EXTENSION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"heapDestroy: invalid heap pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"heap.c\x00" as *const u8 as *const libc::c_char,
              440 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"heapDestroy\x00")).as_ptr(),
              b"PTRVALID(psHeap, sizeof(OBJ_HEAP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Deallocate all the heap's memory */
    psExt = (*psHeap).psExt;
    while !psExt.is_null() {
        psNext = (*psExt).psNext;
        memFreeRelease((*psExt).pMemory as *mut libc::c_void);
        (*psExt).pMemory = 0 as *mut UBYTE;
        memFreeRelease(psExt as *mut libc::c_void);
        psExt = 0 as *mut HEAP_EXTENSION;
        psExt = psNext
    }
    memFreeRelease((*psHeap).pMemory as *mut libc::c_void);
    (*psHeap).pMemory = 0 as *mut UBYTE;
    memFreeRelease(psHeap as *mut libc::c_void);
    psHeap = 0 as *mut OBJ_HEAP;
}
#[no_mangle]
pub unsafe extern "C" fn heapIntegrityCheck(mut psHeap: *mut OBJ_HEAP)
 -> BOOL {
    psHeap = psHeap;
    return 1 as libc::c_int;
}
/* Produce a summary report on the heaps ... DEBUG_MALLOC only */
#[no_mangle]
pub unsafe extern "C" fn heapReport() { }
