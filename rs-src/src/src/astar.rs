use ::libc;
extern "C" {
    /* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
    /*
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    /* Reset the heap, i.e. free all the objects in the heap */
    #[no_mangle]
    fn heapReset(psHeap: *mut OBJ_HEAP);
    /* Destroy a heap and release all the memory associated with it */
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    /* cast a ray from x,y (world coords) at angle ray (0-NUM_RAYS) */
    #[no_mangle]
    fn rayCast(x: UDWORD, y: UDWORD, ray: UDWORD, length: UDWORD,
               callback: RAY_CALLBACK);
    // Calculate the angle to cast a ray between two points
    #[no_mangle]
    fn rayPointsToAngle(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD)
     -> UDWORD;
    /*
 * Fpath.h
 *
 * Interface to the routing functions
 *
 */
    // limit the number of iterations for astar
    #[no_mangle]
    static mut astarMaxRoute: SDWORD;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
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
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type UINT = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
/* The raycast intersection callback.
 * Return FALSE if no more points are required, TRUE otherwise
 */
pub type RAY_CALLBACK
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD) -> BOOL>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _astar_route {
    pub asPos: [POINT; 50],
    pub finalX: SDWORD,
    pub finalY: SDWORD,
    pub numPoints: SDWORD,
}
pub type ASTAR_ROUTE = _astar_route;
// The structure to store a node of the route
pub type FP_NODE = _fp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fp_node {
    pub x: SWORD,
    pub y: SWORD,
    pub dist: SWORD,
    pub est: SWORD,
    pub type_0: SWORD,
    pub psOpen: *mut _fp_node,
    pub psRoute: *mut _fp_node,
    pub psNext: *mut _fp_node,
}
// map coords
// distance so far and estimate to end
// open or closed node
// Previous point in the route
// return codes for astar
pub type C2RustUnnamed = libc::c_uint;
// found a partial route to a nearby position
// routing cannot be finished this frame
// and should be continued the next frame
pub const ASR_NEAREST: C2RustUnnamed = 3;
// no route
pub const ASR_PARTIAL: C2RustUnnamed = 2;
// found a route
pub const ASR_FAILED: C2RustUnnamed = 1;
pub const ASR_OK: C2RustUnnamed = 0;
// route modes for astar
pub type C2RustUnnamed_0 = libc::c_uint;
// continue a route that was partially completed the last frame
// start a new route
pub const ASR_CONTINUE: C2RustUnnamed_0 = 1;
pub const ASR_NEWROUTE: C2RustUnnamed_0 = 0;
/*
 * AStar.c
 *
 * A* based findpath
 *
 */
// A* printf's
//#define DEBUG_GROUP0
// open list printf's
//#define DEBUG_GROUP1
// summary info printf's
//#define DEBUG_GROUP2
// open list storage methods :
// binary tree - 0
// ordered list - 1
// unordered list - 2
#[no_mangle]
pub static mut firstHit: SDWORD = 0;
#[no_mangle]
pub static mut secondHit: SDWORD = 0;
#[no_mangle]
pub static mut astarInner: SDWORD = 0;
#[no_mangle]
pub static mut astarOuter: SDWORD = 0;
#[no_mangle]
pub static mut astarRemove: SDWORD = 0;
// List of open nodes
#[no_mangle]
pub static mut psOpen: *mut FP_NODE = 0 as *const FP_NODE as *mut FP_NODE;
// Hash table for closed nodes
#[no_mangle]
pub static mut apsNodes: *mut *mut FP_NODE =
    0 as *const *mut FP_NODE as *mut *mut FP_NODE;
// object heap to store nodes
#[no_mangle]
pub static mut psFPNodeHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// Convert a direction into an offset
// dir 0 => x = 0, y = -1
static mut aDirOffset: [POINT; 8] =
    [{
         let mut init = POINT{x: 0 as libc::c_int, y: 1 as libc::c_int,};
         init
     },
     {
         let mut init = POINT{x: -(1 as libc::c_int), y: 1 as libc::c_int,};
         init
     },
     {
         let mut init = POINT{x: -(1 as libc::c_int), y: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             POINT{x: -(1 as libc::c_int), y: -(1 as libc::c_int),};
         init
     },
     {
         let mut init = POINT{x: 0 as libc::c_int, y: -(1 as libc::c_int),};
         init
     },
     {
         let mut init = POINT{x: 1 as libc::c_int, y: -(1 as libc::c_int),};
         init
     },
     {
         let mut init = POINT{x: 1 as libc::c_int, y: 0 as libc::c_int,};
         init
     },
     {
         let mut init = POINT{x: 1 as libc::c_int, y: 1 as libc::c_int,};
         init
     }];
// Sizes for the node heap
// counters for a-star
// reset the astar counters
// reset the astar counters
#[no_mangle]
pub unsafe extern "C" fn astarResetCounters() {
    astarInner = 0 as libc::c_int;
    astarOuter = 0 as libc::c_int;
    astarRemove = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ClearAstarNodes() {
    memset(apsNodes as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut FP_NODE>() as
                libc::c_ulong).wrapping_mul(4091 as libc::c_int as
                                                libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn GetApsNodesSize() -> UDWORD {
    return (::std::mem::size_of::<*mut FP_NODE>() as
                libc::c_ulong).wrapping_mul(4091 as libc::c_int as
                                                libc::c_uint);
}
// Initialise the findpath routine
// Initialise the findpath routine
#[no_mangle]
pub unsafe extern "C" fn astarInitialise() -> BOOL {
    // Create the node heap
    if heapCreate(&mut psFPNodeHeap,
                  ::std::mem::size_of::<FP_NODE>() as libc::c_ulong,
                  600 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    apsNodes =
        memMallocRelease((::std::mem::size_of::<*mut FP_NODE>() as
                              libc::c_ulong).wrapping_mul(4091 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut *mut FP_NODE;
    if apsNodes.is_null() { return 0 as libc::c_int }
    ClearAstarNodes();
    return 1 as libc::c_int;
}
// Shutdown the findpath routine
// Shutdown the findpath routine
#[no_mangle]
pub unsafe extern "C" fn fpathShutDown() {
    heapDestroy(psFPNodeHeap);
    memFreeRelease(apsNodes as *mut libc::c_void);
    apsNodes = 0 as *mut *mut FP_NODE;
}
/* **************************************************************************/
/*
 * HashString
 *
 * Adaptation of Peter Weinberger's (PJW) generic hashing algorithm listed
 * in Binstock+Rex, "Practical Algorithms" p 69.
 *
 * Accepts string and returns hashed integer.
 *
 * Hack to use coordinates instead of a string by John.
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn fpathHashFunc(mut x: SDWORD, mut y: SDWORD)
 -> SDWORD {
    let mut iHashValue: UINT = 0;
    let mut i: UINT = 0;
    let mut c: *mut CHAR = 0 as *mut CHAR;
    let mut aBuff: [CHAR; 8] = [0; 8];
    memcpy(&mut *aBuff.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut CHAR as *mut libc::c_void,
           &mut x as *mut SDWORD as *const libc::c_void,
           4 as libc::c_int as libc::c_uint);
    memcpy(&mut *aBuff.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut CHAR as *mut libc::c_void,
           &mut y as *mut SDWORD as *const libc::c_void,
           4 as libc::c_int as libc::c_uint);
    c = aBuff.as_mut_ptr();
    iHashValue = 0 as libc::c_int as UINT;
    while c <
              &mut *aBuff.as_mut_ptr().offset(8 as libc::c_int as isize) as
                  *mut CHAR {
        if *c as libc::c_int != 0 as libc::c_int {
            iHashValue =
                (iHashValue <<
                     (32 as libc::c_int / 8 as libc::c_int) as
                         UINT).wrapping_add(*c as libc::c_uint);
            i =
                iHashValue &
                    !(!(0 as libc::c_int) as UINT >>
                          (32 as libc::c_int / 8 as libc::c_int) as UINT);
            if i != 0 as libc::c_int as libc::c_uint {
                iHashValue =
                    (iHashValue ^
                         i >>
                             (32 as libc::c_int * 3 as libc::c_int /
                                  4 as libc::c_int) as UINT) &
                        !!(!(0 as libc::c_int) as UINT >>
                               (32 as libc::c_int / 8 as libc::c_int) as UINT)
            }
        }
        c = c.offset(1)
    }
    iHashValue =
        (iHashValue as
             libc::c_uint).wrapping_rem(4091 as libc::c_int as libc::c_uint)
            as UINT as UINT;
    return iHashValue as SDWORD;
}
// Add a node to the hash table
#[no_mangle]
pub unsafe extern "C" fn fpathHashAdd(mut apsTable: *mut *mut FP_NODE,
                                      mut psNode: *mut FP_NODE) {
    let mut index: SDWORD = 0;
    index = fpathHashFunc((*psNode).x as SDWORD, (*psNode).y as SDWORD);
    (*psNode).psNext = *apsTable.offset(index as isize);
    let ref mut fresh0 = *apsTable.offset(index as isize);
    *fresh0 = psNode;
}
// See if a node is in the hash table
#[no_mangle]
pub unsafe extern "C" fn fpathHashPresent(mut apsTable: *mut *mut FP_NODE,
                                          mut x: SDWORD, mut y: SDWORD)
 -> *mut FP_NODE {
    let mut index: SDWORD = 0;
    let mut psFound: *mut FP_NODE = 0 as *mut FP_NODE;
    index = fpathHashFunc(x, y);
    psFound = *apsTable.offset(index as isize);
    while !psFound.is_null() &&
              !((*psFound).x as libc::c_int == x &&
                    (*psFound).y as libc::c_int == y) {
        psFound = (*psFound).psNext
    }
    return psFound;
}
/*	FP_NODE		*psPrev, *psFound;

	index = fpathHashFunc(x,y);

	return psFound;
}*/
// Remove a node from the hash table
#[no_mangle]
pub unsafe extern "C" fn fpathHashRemove(mut apsTable: *mut *mut FP_NODE,
                                         mut x: SDWORD, mut y: SDWORD)
 -> *mut FP_NODE {
    let mut index: SDWORD = 0;
    let mut psPrev: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psFound: *mut FP_NODE = 0 as *mut FP_NODE;
    index = fpathHashFunc(x, y);
    if !(*apsTable.offset(index as isize)).is_null() &&
           (**apsTable.offset(index as isize)).x as libc::c_int == x &&
           (**apsTable.offset(index as isize)).y as libc::c_int == y {
        psFound = *apsTable.offset(index as isize);
        let ref mut fresh1 = *apsTable.offset(index as isize);
        *fresh1 = (**apsTable.offset(index as isize)).psNext
    } else {
        psPrev = 0 as *mut FP_NODE;
        psFound = *apsTable.offset(index as isize);
        while !psFound.is_null() {
            if (*psFound).x as libc::c_int == x &&
                   (*psFound).y as libc::c_int == y {
                break ;
            }
            psPrev = psFound;
            psFound = (*psFound).psNext
        }
        if !psFound.is_null() { (*psPrev).psNext = (*psFound).psNext }
    }
    return psFound;
}
// Remove a node from the hash table
#[no_mangle]
pub unsafe extern "C" fn fpathHashCondRemove(mut apsTable: *mut *mut FP_NODE,
                                             mut x: SDWORD, mut y: SDWORD,
                                             mut dist: SDWORD)
 -> *mut FP_NODE {
    let mut index: SDWORD = 0;
    let mut psPrev: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psFound: *mut FP_NODE = 0 as *mut FP_NODE;
    index = fpathHashFunc(x, y);
    if !(*apsTable.offset(index as isize)).is_null() &&
           (**apsTable.offset(index as isize)).x as libc::c_int == x &&
           (**apsTable.offset(index as isize)).y as libc::c_int == y {
        psFound = *apsTable.offset(index as isize);
        if (*psFound).dist as libc::c_int > dist {
            let ref mut fresh2 = *apsTable.offset(index as isize);
            *fresh2 = (**apsTable.offset(index as isize)).psNext
        }
        firstHit += 1
    } else {
        psPrev = 0 as *mut FP_NODE;
        psFound = *apsTable.offset(index as isize);
        while !psFound.is_null() {
            if (*psFound).x as libc::c_int == x &&
                   (*psFound).y as libc::c_int == y {
                break ;
            }
            psPrev = psFound;
            psFound = (*psFound).psNext
        }
        if !psFound.is_null() && (*psFound).dist as libc::c_int > dist {
            (*psPrev).psNext = (*psFound).psNext
        }
        secondHit += 1
    }
    return psFound;
}
// Reset the hash tables
#[no_mangle]
pub unsafe extern "C" fn fpathHashReset() {
    let mut i: SDWORD = 0;
    let mut psNext: *mut FP_NODE = 0 as *mut FP_NODE;
    i = 0 as libc::c_int;
    while i < 4091 as libc::c_int {
        while !(*apsNodes.offset(i as isize)).is_null() {
            psNext = (**apsNodes.offset(i as isize)).psNext;
            heapFree(psFPNodeHeap,
                     *apsNodes.offset(i as isize) as *mut libc::c_void);
            let ref mut fresh3 = *apsNodes.offset(i as isize);
            *fresh3 = psNext
        }
        i += 1
    }
    heapReset(psFPNodeHeap);
}
// Compare two nodes
#[inline]
unsafe extern "C" fn fpathCompare(mut psFirst: *mut FP_NODE,
                                  mut psSecond: *mut FP_NODE) -> SDWORD {
    let mut first: SDWORD = 0;
    let mut second: SDWORD = 0;
    first = (*psFirst).dist as libc::c_int + (*psFirst).est as libc::c_int;
    second = (*psSecond).dist as libc::c_int + (*psSecond).est as libc::c_int;
    if first < second {
        return -(1 as libc::c_int)
    } else { if first > second { return 1 as libc::c_int } }
    // equal totals, give preference to node closer to target
    if ((*psFirst).est as libc::c_int) < (*psSecond).est as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if (*psFirst).est as libc::c_int > (*psSecond).est as libc::c_int {
            return 1 as libc::c_int
        }
    }
    // exactly equal
    return 0 as libc::c_int;
}
// make a 50/50 random choice
static mut seed: UWORD = 1234 as libc::c_int as UWORD;
#[no_mangle]
pub unsafe extern "C" fn fpathRandChoice() -> BOOL {
    let mut val: UDWORD = 0;
    val =
        (seed as libc::c_int * 25173 as libc::c_int + 13849 as libc::c_int &
             0xffff as libc::c_int) as UDWORD;
    seed = val as UWORD;
    return (val & 1 as libc::c_int as libc::c_uint) as BOOL;
}
// Add a node to the open list
#[no_mangle]
pub unsafe extern "C" fn fpathOpenAdd(mut psNode: *mut FP_NODE) {
    (*psNode).psOpen = psOpen;
    psOpen = psNode;
}
// Get the nearest entry in the open list
#[no_mangle]
pub unsafe extern "C" fn fpathOpenGet() -> *mut FP_NODE {
    let mut psNode: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psCurr: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psPrev: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psParent: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut comp: SDWORD = 0;
    if psOpen.is_null() { return 0 as *mut FP_NODE }
    // find the node with the lowest distance
    psPrev = 0 as *mut FP_NODE;
    psNode = psOpen;
    psCurr = psOpen;
    while !psCurr.is_null() {
        comp = fpathCompare(psCurr, psNode);
        if comp < 0 as libc::c_int ||
               comp == 0 as libc::c_int && fpathRandChoice() != 0 {
            psParent = psPrev;
            psNode = psCurr
        }
        psPrev = psCurr;
        psCurr = (*psCurr).psOpen
    }
    // remove the node from the list
    if psNode == psOpen {
        // node is at head of list
        psOpen = (*psOpen).psOpen
    } else { (*psParent).psOpen = (*psNode).psOpen }
    return psNode;
}
// Remove a node from the open list
#[no_mangle]
pub unsafe extern "C" fn fpathOpenRemove(mut psNode: *mut FP_NODE) {
    let mut psCurr: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psPrev: *mut FP_NODE = 0 as *mut FP_NODE;
    if psOpen.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"fpathOpenRemove: NULL list\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"astar.c\x00" as *const u8 as *const libc::c_char,
                  710 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"fpathOpenRemove\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    } else {
        if psNode == psOpen {
            // Remove from start
            psOpen = (*psOpen).psOpen
        } else {
            // remove from the middle
            psPrev = 0 as *mut FP_NODE;
            psCurr = psOpen;
            while !psCurr.is_null() && psCurr != psNode {
                psPrev = psCurr;
                psCurr = (*psCurr).psOpen
            }
            if !psCurr.is_null() {
                (*psPrev).psOpen = (*psCurr).psOpen
            } else {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"fpathOpenRemove: failed to find node\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"astar.c\x00" as *const u8 as *const libc::c_char,
                          733 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 16],
                                                    &[libc::c_char; 16]>(b"fpathOpenRemove\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return
            }
        }
    };
}
// estimate the distance to the target point
#[no_mangle]
pub unsafe extern "C" fn fpathEstimate(mut x: SDWORD, mut y: SDWORD,
                                       mut fx: SDWORD, mut fy: SDWORD)
 -> SDWORD {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    xdiff = if x > fx { (x) - fx } else { (fx) - x };
    ydiff = if y > fy { (y) - fy } else { (fy) - y };
    xdiff = xdiff * 10 as libc::c_int;
    ydiff = ydiff * 10 as libc::c_int;
    return if xdiff > ydiff {
               (xdiff) + ydiff / 2 as libc::c_int
           } else { (xdiff / 2 as libc::c_int) + ydiff };
}
// Generate a new node
#[no_mangle]
pub unsafe extern "C" fn fpathNewNode(mut x: SDWORD, mut y: SDWORD,
                                      mut dist: SDWORD,
                                      mut psRoute: *mut FP_NODE)
 -> *mut FP_NODE {
    let mut psNode: *mut FP_NODE = 0 as *mut FP_NODE;
    if heapAlloc(psFPNodeHeap,
                 &mut psNode as *mut *mut FP_NODE as *mut libc::c_void as
                     *mut *mut libc::c_void) == 0 {
        return 0 as *mut FP_NODE
    }
    (*psNode).x = x as SWORD;
    (*psNode).y = y as SWORD;
    (*psNode).dist = dist as SWORD;
    (*psNode).psRoute = psRoute;
    (*psNode).type_0 = 1 as libc::c_int as SWORD;
    return psNode;
}
// Compare a location with those in the hash table and return a new one
// if necessary
/*FP_NODE *fpathHashCompare(SDWORD x, SDWORD y, SDWORD dist)
{
	FP_NODE	*psNew, *psPrev, *psFound;
	SDWORD	index;

	index = fpathHashFunc(x,y);
	if (apsClosed[index] &&
		apsClosed[index]->x == x && apsClosed[index]->y == y)
	{
		psFound = apsClosed[index];
		apsClosed[index] = apsClosed[index]->psNext;
	}
	else
	{
		psPrev = NULL;
		for(psFound = apsClosed[index]; psFound; psFound = psFound->psNext)
		{
			if (psFound->x == x && psFound->y == y)
			{
				break;
			}
			psPrev = psFound;
		}
		if (psFound)
		{
			psPrev->psNext = psFound->psNext;
		}
	}

	return psNew;
}*/
// Variables for the callback
static mut finalX: SDWORD = 0;
static mut finalY: SDWORD = 0;
static mut vectorX: SDWORD = 0;
static mut vectorY: SDWORD = 0;
static mut obstruction: BOOL = 0;
// The visibility ray callback
#[no_mangle]
pub unsafe extern "C" fn fpathVisCallback(mut x: SDWORD, mut y: SDWORD,
                                          mut dist: SDWORD) -> BOOL {
    let mut vx: SDWORD = 0;
    let mut vy: SDWORD = 0;
    dist = dist;
    // See if this point is past the final point (dot product)
    vx = x - finalX;
    vy = y - finalY;
    if vx * vectorX + vy * vectorY <= 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if fpathBlockingTile.expect("non-null function pointer")(x >>
                                                                 7 as
                                                                     libc::c_int,
                                                             y >>
                                                                 7 as
                                                                     libc::c_int)
           != 0 {
        // found an obstruction
        obstruction = 1 as libc::c_int;
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Check los between two tiles
// Check los between two tiles
#[no_mangle]
pub unsafe extern "C" fn fpathTileLOS(mut x1: SDWORD, mut y1: SDWORD,
                                      mut x2: SDWORD, mut y2: SDWORD)
 -> BOOL {
    // convert to world coords
    x1 = (x1 << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    y1 = (y1 << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    x2 = (x2 << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    y2 = (y2 << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    // Initialise the callback variables
    finalX = x2;
    finalY = y2;
    vectorX = x1 - x2;
    vectorY = y1 - y2;
    obstruction = 0 as libc::c_int;
    rayCast(x1 as UDWORD, y1 as UDWORD, rayPointsToAngle(x1, y1, x2, y2),
            0x7ffff as libc::c_int as UDWORD,
            Some(fpathVisCallback as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                         -> BOOL));
    return (obstruction == 0) as libc::c_int;
}
// Optimise the route
#[no_mangle]
pub unsafe extern "C" fn fpathOptimise(mut psRoute: *mut FP_NODE) {
    let mut psCurr: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psSearch: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psTest: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut los: BOOL = 0;
    if !psRoute.is_null() {
    } else {
        debug(LOG_ERROR,
              b"fpathOptimise: NULL route pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psRoute.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"astar.c\x00" as *const u8 as *const libc::c_char,
              871 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"fpathOptimise\x00")).as_ptr(),
              b"psRoute != NULL\x00" as *const u8 as *const libc::c_char);
    };
    psCurr = psRoute;
    loop  {
        // work down the route looking for a failed LOS
        los = 1 as libc::c_int;
        psSearch = (*psCurr).psRoute;
        while !psSearch.is_null() {
            psTest = (*psSearch).psRoute;
            if !psTest.is_null() {
                los =
                    fpathTileLOS((*psCurr).x as SDWORD, (*psCurr).y as SDWORD,
                                 (*psTest).x as SDWORD, (*psTest).y as SDWORD)
            }
            if los == 0 { break ; }
            psSearch = psTest
        }
        // store the previous successful point
        (*psCurr).psRoute = psSearch;
        psCurr = psSearch;
        if psCurr.is_null() { break ; }
    };
}
// A* findpath
// A* findpath
#[no_mangle]
pub unsafe extern "C" fn fpathAStarRoute(mut routeMode: SDWORD,
                                         mut psRoutePoints: *mut ASTAR_ROUTE,
                                         mut sx: SDWORD, mut sy: SDWORD,
                                         mut fx: SDWORD, mut fy: SDWORD)
 -> SDWORD {
    let mut psFound: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psCurr: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psNew: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psParent: *mut FP_NODE = 0 as *mut FP_NODE;
    let mut psNext: *mut FP_NODE = 0 as *mut FP_NODE;
    static mut psNearest: *mut FP_NODE = 0 as *const FP_NODE as *mut FP_NODE;
    static mut psRoute: *mut FP_NODE = 0 as *const FP_NODE as *mut FP_NODE;
    let mut tileSX: SDWORD = 0;
    let mut tileSY: SDWORD = 0;
    let mut tileFX: SDWORD = 0;
    let mut tileFY: SDWORD = 0;
    let mut dir: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut currDist: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut retval: SDWORD = 0;
    /*	firstHit=0;
	secondHit=0;
	astarInner=0;
	astarOuter=0;
	astarRemove=0;*/
    //	DBPRINTF(("Astar start\n");
    tileSX = sx >> 7 as libc::c_int;
    tileSY = sy >> 7 as libc::c_int;
    tileFX = fx >> 7 as libc::c_int;
    tileFY = fy >> 7 as libc::c_int;
    if routeMode == ASR_NEWROUTE as libc::c_int {
        fpathHashReset();
        // Add the start point to the open list
        psCurr =
            fpathNewNode(tileSX, tileSY, 0 as libc::c_int, 0 as *mut FP_NODE);
        if psCurr.is_null() {
            fpathHashReset();
            //	fpathOpenReset();
            return ASR_FAILED as libc::c_int
        } else {
            (*psCurr).est =
                fpathEstimate((*psCurr).x as SDWORD, (*psCurr).y as SDWORD,
                              tileFX, tileFY) as SWORD;
            psOpen = 0 as *mut FP_NODE;
            fpathOpenAdd(psCurr);
            fpathHashAdd(apsNodes, psCurr);
            psRoute = 0 as *mut FP_NODE;
            psNearest = 0 as *mut FP_NODE
        }
    }
    // search for a route
    while !psOpen.is_null() {
        if astarInner > astarMaxRoute { return ASR_PARTIAL as libc::c_int }
        psCurr = fpathOpenGet();
        if (*psCurr).x as libc::c_int == tileFX &&
               (*psCurr).y as libc::c_int == tileFY {
            // reached the target
            psRoute = psCurr;
            break ;
        } else {
            // note the nearest node to the target so far
            if psNearest.is_null() ||
                   ((*psCurr).est as libc::c_int) <
                       (*psNearest).est as libc::c_int {
                psNearest = psCurr
            }
            astarOuter += 1 as libc::c_int;
            dir = 0 as libc::c_int;
            while dir < 8 as libc::c_int {
                if dir % 2 as libc::c_int == 0 as libc::c_int {
                    currDist =
                        (*psCurr).dist as libc::c_int + 10 as libc::c_int
                } else {
                    currDist =
                        (*psCurr).dist as libc::c_int + 14 as libc::c_int
                }
                // Try a new location
                x = (*psCurr).x as libc::c_int + aDirOffset[dir as usize].x;
                y = (*psCurr).y as libc::c_int + aDirOffset[dir as usize].y;
                // See if the node has already been visited
                psFound = fpathHashPresent(apsNodes, x, y);
                if !(!psFound.is_null() &&
                         (*psFound).dist as libc::c_int <= currDist) {
                    // If the tile hasn't been visited see if it is a blocking tile
                    if !(psFound.is_null() &&
                             fpathBlockingTile.expect("non-null function pointer")(x,
                                                                                   y)
                                 != 0) {
                        astarInner += 1 as libc::c_int;
                        // Now insert the point into the appropriate list
                        if psFound.is_null() {
                            // Not in open or closed lists - add to the open list
                            psNew = fpathNewNode(x, y, currDist, psCurr);
                            if !psNew.is_null() {
                                (*psNew).est =
                                    fpathEstimate(x, y, tileFX, tileFY) as
                                        SWORD;
                                fpathOpenAdd(psNew);
                                fpathHashAdd(apsNodes, psNew);
                            }
                        } else if (*psFound).type_0 as libc::c_int ==
                                      1 as libc::c_int {
                            astarRemove += 1 as libc::c_int;
                            // already in the open list but this is shorter
                            (*psFound).dist = currDist as SWORD;
                            (*psFound).psRoute = psCurr
                        } else if (*psFound).type_0 as libc::c_int ==
                                      2 as libc::c_int {
                            // already in the closed list but this is shorter
                            (*psFound).type_0 = 1 as libc::c_int as SWORD;
                            (*psFound).dist = currDist as SWORD;
                            (*psFound).psRoute = psCurr;
                            fpathOpenAdd(psFound);
                        } else {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"fpathAStarRoute: the open and closed lists are f***ed\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"astar.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      1232 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 16],
                                                                &[libc::c_char; 16]>(b"fpathAStarRoute\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                        }
                    }
                }
                // tile is blocked, skip it
                dir += 1 as libc::c_int
            }
            //		ASSERT( fpathValidateTree(psOpen),
//			"fpathAStarRoute: Invalid open tree" );
            // add the current point to the closed nodes
//		fpathHashRemove(apsOpen, psCurr->x, psCurr->y);
//		fpathHashAdd(apsClosed, psCurr);
            (*psCurr).type_0 = 2 as libc::c_int as SWORD
        }
    }
    //	DBPRINTF(("Astar fin astarOuter=%d astarInner=%d\n",astarOuter,astarInner);
    retval = ASR_OK as libc::c_int;
    // return the nearest route if no actual route was found
    if psRoute.is_null() && !psNearest.is_null() {
        //		!((psNearest->x == tileSX) && (psNearest->y == tileSY)))
        psRoute = psNearest;
        fx =
            (((*psNearest).x as libc::c_int) << 7 as libc::c_int) +
                128 as libc::c_int / 2 as libc::c_int;
        fy =
            (((*psNearest).y as libc::c_int) << 7 as libc::c_int) +
                128 as libc::c_int / 2 as libc::c_int;
        retval = ASR_NEAREST as libc::c_int
    }
    // optimise the route if one was found
    if !psRoute.is_null() {
        fpathOptimise(psRoute);
        // get the route in the correct order
		//	If as I suspect this is to reverse the list, then it's my suspicion that
		//	we could route from destination to source as opposed to source to
		//	destination. We could then save the reversal. to risky to try now...Alex M
        psParent = 0 as *mut FP_NODE;
        psCurr = psRoute;
        while !psCurr.is_null() {
            psNext = (*psCurr).psRoute;
            (*psCurr).psRoute = psParent;
            psParent = psCurr;
            psCurr = psNext
        }
        psRoute = psParent;
        psCurr = psRoute;
        index = (*psRoutePoints).numPoints;
        while !psCurr.is_null() && index < 50 as libc::c_int {
            (*psRoutePoints).asPos[index as usize].x =
                (*psCurr).x as libc::c_int;
            (*psRoutePoints).asPos[index as usize].y =
                (*psCurr).y as libc::c_int;
            index += 1 as libc::c_int;
            psCurr = (*psCurr).psRoute
        }
        (*psRoutePoints).numPoints = index;
        (*psRoutePoints).finalX =
            (*psRoutePoints).asPos[(index - 1 as libc::c_int) as usize].x;
        (*psRoutePoints).finalY =
            (*psRoutePoints).asPos[(index - 1 as libc::c_int) as usize].y
    } else { retval = ASR_FAILED as libc::c_int }
    fpathHashReset();
    //	fpathOpenReset();
    return retval;
}
