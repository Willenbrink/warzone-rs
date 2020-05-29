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
    fn malloc(_: libc::c_uint) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    #[no_mangle]
    fn abort() -> !;
}
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
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
pub type BLOCK_HEAP = _block_heap;
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
// initial and extension block sizes
/* Whether allocated memory is initialised to a value and whether the memory
 * is trashed before it is freed.
 * This is done automatically by Visual C's memory routines.
 */
// the filename and line number of the last call to the block functions
#[no_mangle]
pub static mut pCallFileName: *mut STRING = 0 as *const STRING as *mut STRING;
#[no_mangle]
pub static mut callLine: SDWORD = 0;
// the list of allocated blocks
static mut psBlockList: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
/* *********************************************************************************/
/*                    function prototypes                                         */
// initialise the block system
// initialise the block system
#[no_mangle]
pub unsafe extern "C" fn blkInitialise() -> BOOL {
    if psBlockList.is_null() {
    } else {
        debug(LOG_ERROR,
              b"blkInitialise: Blocks already initialised\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psBlockList.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"block.c\x00" as *const u8 as *const libc::c_char,
              51 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"blkInitialise\x00")).as_ptr(),
              b"psBlockList==NULL\x00" as *const u8 as *const libc::c_char);
    };
    // blkShutDown() needs to be called
    psBlockList = 0 as *mut BLOCK_HEAP;
    return 1 as libc::c_int;
}
// shutdown the block system
// shutdown the block system
#[no_mangle]
pub unsafe extern "C" fn blkShutDown() {
    let mut psNext: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    if !psBlockList.is_null() {
        debug(LOG_NEVER,
              b"blkShutDown: blocks still allocated:\n\x00" as *const u8 as
                  *const libc::c_char);
        while !psBlockList.is_null() {
            psNext = (*psBlockList).psNext;
            blkDestroy(psBlockList);
            psBlockList = psNext
        }
    };
}
// Note the call position for a blkAlloc or blkFree
// Note the call position for a blkAlloc or blkFree
#[no_mangle]
pub unsafe extern "C" fn blkCallPos(mut pFileName: *mut STRING,
                                    mut line: SDWORD) {
    pCallFileName = pFileName;
    callLine = line;
}
// Create a new block heap
// Create a new block heap
#[no_mangle]
pub unsafe extern "C" fn blkCreate(mut ppsHeap: *mut *mut BLOCK_HEAP,
                                   mut init: SDWORD, mut ext: SDWORD)
 -> BOOL {
    debug(LOG_NEVER,
          b"BLKCREATE CALLED !!!!!!!!!!!!!!!!!!!!!!\n\x00" as *const u8 as
              *const libc::c_char);
    *ppsHeap =
        malloc(::std::mem::size_of::<BLOCK_HEAP>() as libc::c_ulong) as
            *mut BLOCK_HEAP;
    if (*ppsHeap).is_null() {
        debug(LOG_ERROR,
              b"blkCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    (**ppsHeap).psBlocks =
        malloc(::std::mem::size_of::<BLOCK_HEAP_MEM>() as libc::c_ulong) as
            *mut BLOCK_HEAP_MEM;
    if (**ppsHeap).psBlocks.is_null() {
        debug(LOG_ERROR,
              b"blkCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    (*(**ppsHeap).psBlocks).pMem = malloc(init as libc::c_uint) as *mut UBYTE;
    if (*(**ppsHeap).psBlocks).pMem.is_null() {
        debug(LOG_ERROR,
              b"blkCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    (**ppsHeap).init = init;
    (**ppsHeap).ext = ext;
    (*(**ppsHeap).psBlocks).size = init;
    (*(**ppsHeap).psBlocks).pFree = (*(**ppsHeap).psBlocks).pMem;
    (*(**ppsHeap).psBlocks).psNext = 0 as *mut _block_heap_mem;
    (**ppsHeap).psNext = psBlockList;
    psBlockList = *ppsHeap;
    return 1 as libc::c_int;
}
// Release a block heap
// Release a block heap
#[no_mangle]
pub unsafe extern "C" fn blkDestroy(mut psHeap: *mut BLOCK_HEAP) {
    let mut psCurr: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    let mut psNext: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    let mut psPrev: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    let mut psCurr_0: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    psPrev = 0 as *mut BLOCK_HEAP;
    psCurr_0 = psBlockList;
    while !psCurr_0.is_null() {
        if psCurr_0 == psHeap { break ; }
        psPrev = psCurr_0;
        psCurr_0 = (*psCurr_0).psNext
    }
    if !psCurr_0.is_null() {
    } else {
        debug(LOG_ERROR,
              b"LIST_REMOVE: block.c(%d): entry not found\x00" as *const u8 as
                  *const libc::c_char, 142 as libc::c_int);
    };
    if !psCurr_0.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"block.c\x00" as *const u8 as *const libc::c_char,
              142 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"blkDestroy\x00")).as_ptr(),
              b"psCurr!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    if psPrev.is_null() {
        psBlockList = (*psBlockList).psNext
    } else if !psCurr_0.is_null() { (*psPrev).psNext = (*psCurr_0).psNext }
    psCurr = (*psHeap).psBlocks;
    while !psCurr.is_null() {
        free((*psCurr).pMem as *mut libc::c_void);
        psNext = (*psCurr).psNext;
        free(psCurr as *mut libc::c_void);
        psCurr = psNext
    }
    free(psHeap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn blkPrintDetails(mut psHeap: *mut BLOCK_HEAP) {
    if !psHeap.is_null() {
        debug(LOG_NEVER,
              b"ptr=%p init=%d ext=%d\n\x00" as *const u8 as
                  *const libc::c_char, psHeap, (*psHeap).init, (*psHeap).ext);
    } else {
        debug(LOG_NEVER,
              b"NULL POINTER IN BLOCK LIST\n\x00" as *const u8 as
                  *const libc::c_char);
    };
}
// report on the blocks
#[no_mangle]
pub unsafe extern "C" fn blkReport() { }
// Allocate some memory from a block heap
// no longer used - uploaded in small chunks
// Allocate some memory from a block heap
#[no_mangle]
pub unsafe extern "C" fn blkAlloc(mut psHeap: *mut BLOCK_HEAP,
                                  mut size: SDWORD) -> *mut libc::c_void {
    let mut pAlloc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut psCurr: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    let mut psNew: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    // Round up to nearest 4 bytes ( 32 bit align ).. Neaded for Playstation.. PD.
    size =
        ((size + 3 as libc::c_int) as libc::c_uint &
             0xfffffffc as libc::c_uint) as SDWORD;
    // can't allocate 0 bytes
    if size <= 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"blkAlloc: cannot allocate 0 bytes\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"block.c\x00" as *const u8 as *const libc::c_char,
                  264 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 9],
                                            &[libc::c_char; 9]>(b"blkAlloc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut libc::c_void
    }
    // find a block with a large enough segment free
    pAlloc = 0 as *mut libc::c_void;
    psCurr = (*psHeap).psBlocks;
    while !psCurr.is_null() {
        if (*psCurr).pFree.offset(size as isize) <=
               (*psCurr).pMem.offset((*psCurr).size as isize) {
            pAlloc = (*psCurr).pFree as *mut libc::c_void;
            (*psCurr).pFree = (*psCurr).pFree.offset(size as isize);
            break ;
        } else { psCurr = (*psCurr).psNext }
    }
    // if there wasn't a block try to allocate a new one
    if psCurr.is_null() && (*psHeap).ext != 0 as libc::c_int {
        psNew =
            malloc(::std::mem::size_of::<BLOCK_HEAP_MEM>() as libc::c_ulong)
                as *mut BLOCK_HEAP_MEM;
        if psNew.is_null() {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"blkAlloc: warning out of memory\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"block.c\x00" as *const u8 as *const libc::c_char,
                      301 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 9],
                                                &[libc::c_char; 9]>(b"blkAlloc\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            // Out of memory
            return 0 as *mut libc::c_void
        }
        if size < (*psHeap).ext {
            (*psNew).pMem =
                malloc((*psHeap).ext as libc::c_uint) as *mut UBYTE;
            (*psNew).size = (*psHeap).ext
        } else {
            (*psNew).pMem = malloc(size as libc::c_uint) as *mut UBYTE;
            (*psNew).size = size
        }
        if (*psNew).pMem.is_null() {
            // Out of memory
            free(psNew as *mut libc::c_void);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"blkAlloc: warning out of memory\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"block.c\x00" as *const u8 as *const libc::c_char,
                      321 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 9],
                                                &[libc::c_char; 9]>(b"blkAlloc\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut libc::c_void
        }
        (*psNew).psNext = 0 as *mut _block_heap_mem;
        (*psNew).pFree = (*psNew).pMem.offset(size as isize);
        pAlloc = (*psNew).pMem as *mut libc::c_void;
        // Add the block to the end of the list
        psCurr = (*psHeap).psBlocks;
        while !(*psCurr).psNext.is_null() { psCurr = (*psCurr).psNext }
        (*psCurr).psNext = psNew
    }
    (*psCurr).pLastAllocated = pAlloc as *mut UBYTE;
    // /* - error trapping an out-of-mem allocation !!!
    //NoMemChk:
    return pAlloc;
}
// return a chunk of memory to the block
// this only does anything whith DEBUG_BLOCK defined
//*/
// return a chunk of memory to the block
// this only does anything whith DEBUG_BLOCK defined
#[no_mangle]
pub unsafe extern "C" fn blkFree(mut psHeap: *mut BLOCK_HEAP,
                                 mut pMemToFree: *mut libc::c_void) {
}
// Reset a block heap
// Reset a block heap
#[no_mangle]
pub unsafe extern "C" fn blkReset(mut psHeap: *mut BLOCK_HEAP) {
    let mut psCurr: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    let mut block: SDWORD = 0 as libc::c_int;
    let mut alloc: SDWORD = 0 as libc::c_int;
    debug(LOG_NEVER,
          b"blkReset: memory usage:\n\x00" as *const u8 as
              *const libc::c_char);
    psCurr = (*psHeap).psBlocks;
    while !psCurr.is_null() {
        alloc +=
            (*psCurr).pFree.wrapping_offset_from((*psCurr).pMem) as
                libc::c_int;
        block += (*psCurr).size;
        (*psCurr).pFree = (*psCurr).pMem;
        psCurr = (*psCurr).psNext
    }
    debug(LOG_NEVER,
          b"    Blocks allocated %dk, Memory allocated %dk\n\x00" as *const u8
              as *const libc::c_char, block / 1024 as libc::c_int,
          alloc / 1024 as libc::c_int);
}
// Find which block a pointer is from if any
// Find which block a pointer is from if any
#[no_mangle]
pub unsafe extern "C" fn blkFind(mut pPtr: *mut libc::c_void)
 -> *mut BLOCK_HEAP {
    let mut psHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    let mut psMem: *mut BLOCK_HEAP_MEM = 0 as *mut BLOCK_HEAP_MEM;
    psHeap = psBlockList;
    while !psHeap.is_null() {
        psMem = (*psHeap).psBlocks;
        while !psMem.is_null() {
            if pPtr as *mut UBYTE >= (*psMem).pMem &&
                   (pPtr as *mut UBYTE) <
                       (*psMem).pMem.offset((*psMem).size as isize) {
                return psHeap
            }
            psMem = (*psMem).psNext
        }
        psHeap = (*psHeap).psNext
    }
    return 0 as *mut BLOCK_HEAP;
}
// check if a pointer is valid in a block
// check if a pointer is valid in a block
#[no_mangle]
pub unsafe extern "C" fn blkPointerValid(mut psHeap: *mut BLOCK_HEAP,
                                         mut pData: *mut libc::c_void,
                                         mut size: SDWORD) -> BOOL {
    psHeap = psHeap;
    pData = pData;
    size = size;
    return 1 as libc::c_int;
}
// check if a pointer is valid in any currently allocated block
// check if a pointer is valid in any currently allocated block
#[no_mangle]
pub unsafe extern "C" fn blkPointerValidAll(mut pData: *mut libc::c_void,
                                            mut size: SDWORD) -> BOOL {
    pData = pData;
    size = size;
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut psSuspendedHeap: *mut BLOCK_HEAP =
    0 as *const BLOCK_HEAP as *mut BLOCK_HEAP;
// suspend the current block ... all allocations pass though to system memory allocation
// if a block is already suspended then an assertion will occur.
#[no_mangle]
pub unsafe extern "C" fn blockSuspendUsage() {
    if psSuspendedHeap.is_null() {
    } else {
        debug(LOG_ERROR,
              b"a memory block is already suspended\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psSuspendedHeap.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"block.c\x00" as *const u8 as *const libc::c_char,
              615 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"blockSuspendUsage\x00")).as_ptr(),
              b"psSuspendedHeap==NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    psSuspendedHeap = memGetBlockHeap();
    memSetBlockHeap(0 as *mut _block_heap);
}
// restore the current block  - if there is one
#[no_mangle]
pub unsafe extern "C" fn blockUnsuspendUsage() {
    memSetBlockHeap(psSuspendedHeap);
    psSuspendedHeap = 0 as *mut BLOCK_HEAP;
}
#[no_mangle]
pub unsafe extern "C" fn blockCurrentBlockInfo() { }
