use ::libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
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
    /* Recurisvely find an object in a treap */
    #[no_mangle]
    fn treapFindRec(psRoot: *mut TREAP_NODE, key: UDWORD, cmp: TREAP_CMP)
     -> *mut libc::c_void;
    // Allocate some memory from a block heap
    #[no_mangle]
    fn blkAlloc(psHeap: *mut BLOCK_HEAP, size: SDWORD) -> *mut libc::c_void;
    // return a chunk of memory to the block
// this only does anything whith DEBUG_BLOCK defined
    #[no_mangle]
    fn blkFree(psHeap: *mut BLOCK_HEAP, pMemToFree: *mut libc::c_void);
    // Find which block a pointer is from if any
    #[no_mangle]
    fn blkFind(pPtr: *mut libc::c_void) -> *mut BLOCK_HEAP;
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
pub type BLOCK_HEAP = _block_heap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
// initial and extension block sizes
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
/* The character used to initialise malloc'ed memory, or to trash memory before
   freeing it */
// memory block header for the treap code
pub type MEM_NODE = _mem_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mem_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
    pub pFile: *mut STRING,
    pub line: SDWORD,
    pub size: UDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
// The treap data to store the node
// The debug info for the node (file, line).
// The memory block size
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
/* The root of the memory treap */
static mut psMemRoot: *mut MEM_NODE = 0 as *const MEM_NODE as *mut MEM_NODE;
/* The current block heap to use instead of MALLOC */
static mut psCurrBlockHeap: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
/*
 * mem.h
 *
 * Interface to the malloc/free replacements
 */
/* DEBUG_MALLOC == TRUE uses debugging malloc and free
   DEBUG_MALLOC == FALSE uses normal malloc and free */
/* Function Prototypes */
/* Initialise the memory system */
/* Initialise the memory system */
#[no_mangle]
pub unsafe extern "C" fn memInitialise() -> BOOL {
    if !psMemRoot.is_null() {
        debug(LOG_MEMORY,
              b"memInitialise: *** WARNING *** : memory still allocated??\x00"
                  as *const u8 as *const libc::c_char);
    }
    psMemRoot = 0 as *mut MEM_NODE;
    psCurrBlockHeap = 0 as *mut BLOCK_HEAP;
    return 1 as libc::c_int;
}
/* Release the memory treap */
unsafe extern "C" fn memTreapDestroy(mut psRoot: *mut TREAP_NODE) {
    if !psRoot.is_null() {
        // Destroy the sub trees
        memTreapDestroy((*psRoot).psLeft);
        memTreapDestroy((*psRoot).psRight);
        // Free the root
        free(psRoot as *mut libc::c_void);
    };
}
/* Shutdown the memory system */
/* Shutdown the memory system */
#[no_mangle]
pub unsafe extern "C" fn memShutDown() {
    // Report any memory still allocated
    memMemoryReport(0 as *mut STRING);
    // Free up the allocated memory
    memTreapDestroy(psMemRoot as *mut TREAP_NODE);
}
/* Set a block heap to use for all memory allocation rather than standard malloc/free */
/* Set a block heap to use for all memory allocation rather than standard malloc/free */
#[no_mangle]
pub unsafe extern "C" fn memSetBlockHeap(mut psHeap: *mut BLOCK_HEAP) {
    psCurrBlockHeap = psHeap;
}
/* Get the current block heap */
/* Get the current block heap */
#[no_mangle]
pub unsafe extern "C" fn memGetBlockHeap() -> *mut _block_heap {
    return psCurrBlockHeap;
}
/* compare two memory blocks */
/* compare two memory blocks
 * NOTE: key1 is always the block passed into the treap code
 *       and therefore not necessarily to be trusted
 */
#[no_mangle]
pub unsafe extern "C" fn memBlockCmp(mut key1: UDWORD, mut key2: UDWORD)
 -> SDWORD {
    let mut start1: UDWORD = 0;
    let mut start2: UDWORD = 0;
    let mut end1: UDWORD = 0;
    let mut end2: UDWORD = 0;
    // Calculate the edges of the memory blocks
    start1 =
        ((*(key1 as *mut MEM_NODE)).pObj as
             *mut UBYTE).offset(::std::mem::size_of::<MEM_NODE>() as
                                    libc::c_ulong as
                                    isize).offset(32 as libc::c_int as isize)
            as UDWORD;
    start2 =
        ((*(key2 as *mut MEM_NODE)).pObj as
             *mut UBYTE).offset(::std::mem::size_of::<MEM_NODE>() as
                                    libc::c_ulong as
                                    isize).offset(32 as libc::c_int as isize)
            as UDWORD;
    end1 = start1.wrapping_add((*(key1 as *mut MEM_NODE)).size);
    end2 = start2.wrapping_add((*(key2 as *mut MEM_NODE)).size);
    // see if one block is inside another
    if start1 >= start2 && end1 <= end2 {
        // ||		// block 1 inside block 2
        //		(start2 >= start1 && end2 <= end1))			// block 2 inside block 1
        return 0 as libc::c_int
    } else {
        if start1 < start2 {
            // less than
            return -(1 as libc::c_int)
        }
    }
    // greater than
    return 1 as libc::c_int;
}
/* Replacement for malloc for release builds. */
#[no_mangle]
pub unsafe extern "C" fn memMallocRelease(mut Size: size_t)
 -> *mut libc::c_void {
    if !psCurrBlockHeap.is_null() {
        // use a block heap rather than normal malloc
        return blkAlloc(psCurrBlockHeap, Size as SDWORD)
    }
    return malloc(Size);
}
/* Replacement for Free for release builds */
#[no_mangle]
pub unsafe extern "C" fn memFreeRelease(mut pMemToFree: *mut libc::c_void) {
    let mut psBlock: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    // see if the pointer was allocated in a block
    psBlock = blkFind(pMemToFree);
    if !psBlock.is_null() {
        // use a block heap rather than normal free
        blkFree(psBlock, pMemToFree);
        return
    }
    free(pMemToFree);
}
/* Check a pointer refers to a valid block of memory */
/* Checks whether the memory buffer pointed to by pPtr of size Size
 * is contained in any of the memory blocks allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn memPointerValid(mut pPtr: *mut libc::c_void,
                                         mut size: size_t) -> BOOL {
    let mut sNode: MEM_NODE =
        MEM_NODE{key: 0,
                 priority: 0,
                 pObj: 0 as *mut libc::c_void,
                 psLeft: 0 as *mut _treap_node,
                 psRight: 0 as *mut _treap_node,
                 pFile: 0 as *mut STRING,
                 line: 0,
                 size: 0,};
    if size != 0 {
    } else {
        debug(LOG_ERROR,
              b"memPointerValid: cannot check a pointer with zero size\x00" as
                  *const u8 as *const libc::c_char);
    };
    if size != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mem.c\x00" as *const u8 as *const libc::c_char,
              344 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"memPointerValid\x00")).as_ptr(),
              b"size\x00" as *const u8 as *const libc::c_char);
    };
    if pPtr.is_null() { return 0 as libc::c_int }
    // Create a dummy node for the search
	// This is only looked at by memBlockCmp so only need to set the object and size
    sNode.pObj =
        (pPtr as
             *mut UBYTE).offset(-(::std::mem::size_of::<MEM_NODE>() as
                                      libc::c_ulong as
                                      isize)).offset(-(32 as libc::c_int as
                                                           isize)) as
            *mut libc::c_void;
    sNode.size = size;
    // See if the block is in the treap
    if !treapFindRec(psMemRoot as *mut TREAP_NODE,
                     &mut sNode as *mut MEM_NODE as UDWORD,
                     Some(memBlockCmp as
                              unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                  -> SDWORD)).is_null() {
        return 1 as libc::c_int
    }
    // check the block heaps as well (if the code is there)
    return 0 as libc::c_int;
}
/* Recursive function to print out the list of memory blocks */
/* Recursive function to print out the list of memory blocks */
#[no_mangle]
pub unsafe extern "C" fn memRecReport(mut psRoot: *mut MEM_NODE) -> SDWORD {
    return 0 as libc::c_int;
}
/* Recursive function to total up the amount of mem allocated */
#[no_mangle]
pub unsafe extern "C" fn memSummary(mut psRoot: *mut MEM_NODE) { }
#[no_mangle]
pub unsafe extern "C" fn memMemorySummary() { memMemoryDump(psMemRoot); }
#[no_mangle]
pub unsafe extern "C" fn memMemoryDump(mut Node: *mut MEM_NODE) { }
/* Report on currently allocated memory */
/* Report on currently allocated memory.
 * If pFileName is not NULL send the report to the specified file.
 * If pFileName is NULL the report goes to DBPRINTF
 */
#[no_mangle]
pub unsafe extern "C" fn memMemoryReport(mut pFileName: *mut STRING) { }
/* Display the memory treap */
/* Display the memory treap */
#[no_mangle]
pub unsafe extern "C" fn memDisplayTreap(mut pFileName: *mut STRING) { }
