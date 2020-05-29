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
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn draw3dLine(src: *mut iVector, dest: *mut iVector, col: UBYTE);
}
pub type UBYTE = libc::c_uchar;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub type int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARROW {
    pub vecBase: iVector,
    pub vecHead: iVector,
    pub iColour: UBYTE,
    pub psNext: *mut ARROW,
}
// Extension memory for the heap
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub static mut g_psArrowHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut g_psArrowList: *mut ARROW = 0 as *const ARROW as *mut ARROW;
/* **************************************************************************/
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn arrowInit() -> BOOL {
    if heapCreate(&mut g_psArrowHeap,
                  ::std::mem::size_of::<ARROW>() as libc::c_ulong,
                  15 as libc::c_int as UDWORD, 15 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn arrowShutDown() { heapDestroy(g_psArrowHeap); }
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn arrowAdd(mut iBaseX: SDWORD, mut iBaseY: SDWORD,
                                  mut iBaseZ: SDWORD, mut iHeadX: SDWORD,
                                  mut iHeadY: SDWORD, mut iHeadZ: SDWORD,
                                  mut iColour: UBYTE) -> BOOL {
    let mut psArrow: *mut ARROW = 0 as *mut ARROW;
    if heapAlloc(g_psArrowHeap,
                 &mut psArrow as *mut *mut ARROW as *mut libc::c_void as
                     *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    /* ivis y,z swapped */
    (*psArrow).vecBase.x = iBaseX;
    (*psArrow).vecBase.y = iBaseZ;
    (*psArrow).vecBase.z = iBaseY;
    (*psArrow).vecHead.x = iHeadX;
    (*psArrow).vecHead.y = iHeadZ;
    (*psArrow).vecHead.z = iHeadY;
    (*psArrow).iColour = iColour;
    /* add to list */
    (*psArrow).psNext = g_psArrowList;
    g_psArrowList = psArrow;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn arrowDrawAll() {
    let mut psArrow: *mut ARROW = 0 as *mut ARROW;
    let mut psArrowTemp: *mut ARROW = 0 as *mut ARROW;
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_ON);
    pie_SetFogStatus(0 as libc::c_int);
    /* draw and clear list */
    psArrow = g_psArrowList;
    while !psArrow.is_null() {
        draw3dLine(&mut (*psArrow).vecHead, &mut (*psArrow).vecBase,
                   (*psArrow).iColour);
        psArrowTemp = (*psArrow).psNext;
        heapFree(g_psArrowHeap, psArrow as *mut libc::c_void);
        psArrow = psArrowTemp
    }
    /* reset list */
    g_psArrowList = 0 as *mut ARROW;
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
}
/* **************************************************************************/
