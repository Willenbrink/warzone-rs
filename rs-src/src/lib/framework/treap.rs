use ::libc;
extern "C" {
    pub type _block_heap;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Compare S1 and S2.  */
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
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
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
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
// comparison function
// node heap
// root of the tree
/*
 * Treap.c
 *
 * Balanced tree implementation
 *
 */
/* Allow frame header files to be singly included */
/* Position of the last call */
static mut cLine: SDWORD = 0;
static mut pCFile: *mut STRING = 0 as *const STRING as *mut STRING;
static mut pCFileNone: [STRING; 5] = [78, 111, 110, 101, 0];
/* ***************************************************************************************/
/*                           Function Protoypes                                         */
/*                                                                                      */
/*      These should not be called directly - use the macros below                      */
/* Store the location in C code at which a call to the heap was made */
/* Store the location in C code at which a call to the heap was made */
#[no_mangle]
pub unsafe extern "C" fn treapSetCallPos(mut pFileName: *mut STRING,
                                         mut lineNumber: SDWORD) {
    cLine = lineNumber;
    pCFile =
        memMallocRelease(strlen(pFileName).wrapping_add(1 as libc::c_int as
                                                            libc::c_uint)) as
            *mut STRING;
    if !pCFile.is_null() {
        strcpy(pCFile, pFileName);
    } else { pCFile = pCFileNone.as_mut_ptr() };
}
/* Default comparison function - assumes keys are ints */
unsafe extern "C" fn defaultCmp(mut key1: UDWORD, mut key2: UDWORD)
 -> SDWORD {
    if key1 < key2 {
        return -(1 as libc::c_int)
    } else { if key1 > key2 { return 1 as libc::c_int } }
    return 0 as libc::c_int;
}
/* ***************************************************************************************/
/*                            Comparison Functions                                      */
/* A useful comparison function - keys are string pointers */
/* A useful comparison function - keys are string pointers */
#[no_mangle]
pub unsafe extern "C" fn treapStringCmp(mut key1: UDWORD, mut key2: UDWORD)
 -> SDWORD {
    let mut result: SDWORD = 0;
    let mut pStr1: *mut STRING = key1 as *mut STRING;
    let mut pStr2: *mut STRING = key2 as *mut STRING;
    result = strcmp(pStr1, pStr2);
    if result < 0 as libc::c_int { return -(1 as libc::c_int) }
    if result > 0 as libc::c_int { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/* Function type for object equality */
//typedef BOOL (*TREAP_EQUAL)(void *pObj1, void *pObj2);
/* Function to create a treap
 * Pass in key comparison function
 * Initial number of nodes to allocate
 * Number of additional nodes to allocate when extending
 */
/* Function to create a treap
 * Pass in key comparison function,
 * initial number of nodes to allocate,
 * number of additional nodes to allocate when extending.
 */
#[no_mangle]
pub unsafe extern "C" fn treapCreate(mut ppsTreap: *mut *mut TREAP,
                                     mut cmp: TREAP_CMP, mut init: UDWORD,
                                     mut ext: UDWORD) -> BOOL {
    *ppsTreap =
        memMallocRelease(::std::mem::size_of::<TREAP>() as libc::c_ulong) as
            *mut TREAP;
    if (*ppsTreap).is_null() {
        debug(LOG_ERROR,
              b"treapCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    if heapCreate(&mut (**ppsTreap).psNodes,
                  ::std::mem::size_of::<TREAP_NODE>() as libc::c_ulong, init,
                  ext) == 0 {
        debug(LOG_ERROR,
              b"treapCreate: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // Store the comparison function if there is one, use the default otherwise
    if cmp.is_some() {
        (**ppsTreap).cmp = cmp
    } else {
        (**ppsTreap).cmp =
            Some(defaultCmp as
                     unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD)
    }
    // Initialise the tree to nothing
    (**ppsTreap).psRoot = 0 as *mut TREAP_NODE;
    return 1 as libc::c_int;
}
/* Rotate a tree to the right
 * (Make left sub tree the root and the root the right sub tree)
 */
unsafe extern "C" fn treapRotRight(mut ppsRoot: *mut *mut TREAP_NODE) {
    let mut psNewRoot: *mut TREAP_NODE = 0 as *mut TREAP_NODE;
    psNewRoot = (**ppsRoot).psLeft;
    (**ppsRoot).psLeft = (*psNewRoot).psRight;
    (*psNewRoot).psRight = *ppsRoot;
    *ppsRoot = psNewRoot;
}
/* Rotate a tree to the left
 * (Make right sub tree the root and the root the left sub tree)
 */
unsafe extern "C" fn treapRotLeft(mut ppsRoot: *mut *mut TREAP_NODE) {
    let mut psNewRoot: *mut TREAP_NODE = 0 as *mut TREAP_NODE;
    psNewRoot = (**ppsRoot).psRight;
    (**ppsRoot).psRight = (*psNewRoot).psLeft;
    (*psNewRoot).psLeft = *ppsRoot;
    *ppsRoot = psNewRoot;
}
/* Recursive function to add an object to a tree */
#[no_mangle]
pub unsafe extern "C" fn treapAddNode(mut ppsRoot: *mut *mut TREAP_NODE,
                                      mut psNew: *mut TREAP_NODE,
                                      mut cmp: TREAP_CMP) {
    if (*ppsRoot).is_null() {
        // Make the node the root of the tree
        *ppsRoot = psNew
    } else if cmp.expect("non-null function pointer")((*psNew).key,
                                                      (**ppsRoot).key) <
                  0 as libc::c_int {
        // Node less than root, insert to the left of the tree
        treapAddNode(&mut (**ppsRoot).psLeft, psNew, cmp);
        // Sort the priority
        if (**ppsRoot).priority > (*(**ppsRoot).psLeft).priority {
            // rotate tree right
            treapRotRight(ppsRoot);
        }
    } else {
        // Node greater than root, insert to the right of the tree
        treapAddNode(&mut (**ppsRoot).psRight, psNew, cmp);
        // Sort the priority
        if (**ppsRoot).priority > (*(**ppsRoot).psRight).priority {
            // rotate tree left
            treapRotLeft(ppsRoot);
        }
    };
}
/* Add an object to a treap
 */
/* Add an object to a treap
 */
#[no_mangle]
pub unsafe extern "C" fn treapAdd(mut psTreap: *mut TREAP, mut key: UDWORD,
                                  mut pObj: *mut libc::c_void) -> BOOL {
    let mut psNew: *mut TREAP_NODE = 0 as *mut TREAP_NODE;
    if heapAlloc((*psTreap).psNodes,
                 &mut psNew as *mut *mut TREAP_NODE as *mut *mut libc::c_void)
           == 0 {
        return 0 as libc::c_int
    }
    (*psNew).priority = rand() as UDWORD;
    (*psNew).key = key;
    (*psNew).pObj = pObj;
    (*psNew).psLeft = 0 as *mut _treap_node;
    (*psNew).psRight = 0 as *mut _treap_node;
    treapAddNode(&mut (*psTreap).psRoot, psNew, (*psTreap).cmp);
    return 1 as libc::c_int;
}
/* Recursively find and remove a node from the tree */
#[no_mangle]
pub unsafe extern "C" fn treapDelRec(mut ppsRoot: *mut *mut TREAP_NODE,
                                     mut key: UDWORD, mut cmp: TREAP_CMP)
 -> *mut TREAP_NODE {
    let mut psFound: *mut TREAP_NODE = 0 as *mut TREAP_NODE;
    if (*ppsRoot).is_null() {
        // not found
        return 0 as *mut TREAP_NODE
    }
    match cmp.expect("non-null function pointer")(key, (**ppsRoot).key) {
        -1 => {
            // less than
            return treapDelRec(&mut (**ppsRoot).psLeft, key, cmp)
        }
        1 => {
            // greater than
            return treapDelRec(&mut (**ppsRoot).psRight, key, cmp)
        }
        0 => {
            // equal - either remove or push down the tree to balance it
            if (**ppsRoot).psLeft.is_null() && (**ppsRoot).psRight.is_null() {
                // no sub trees, remove
                psFound = *ppsRoot;
                *ppsRoot = 0 as *mut TREAP_NODE;
                return psFound
            } else if (**ppsRoot).psLeft.is_null() {
                // one sub tree, replace
                psFound = *ppsRoot;
                *ppsRoot = (*psFound).psRight;
                return psFound
            } else if (**ppsRoot).psRight.is_null() {
                // one sub tree, replace
                psFound = *ppsRoot;
                *ppsRoot = (*psFound).psLeft;
                return psFound
            } else if (*(**ppsRoot).psLeft).priority >
                          (*(**ppsRoot).psRight).priority {
                // two sub trees, push the node down and recurse
                // rotate right and recurse
                treapRotLeft(ppsRoot);
                return treapDelRec(&mut (**ppsRoot).psLeft, key, cmp)
            } else {
                // rotate left and recurse
                treapRotRight(ppsRoot);
                return treapDelRec(&mut (**ppsRoot).psRight, key, cmp)
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"treapDelRec: invalid return from comparison\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"treap.c\x00" as *const u8 as *const libc::c_char,
                      265 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"treapDelRec\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 0 as *mut TREAP_NODE;
}
/* Remove an object from the treap */
/* Remove an object from the treap */
#[no_mangle]
pub unsafe extern "C" fn treapDel(mut psTreap: *mut TREAP, mut key: UDWORD)
 -> BOOL {
    let mut psDel: *mut TREAP_NODE = 0 as *mut TREAP_NODE;
    // Find the node to remove
    psDel = treapDelRec(&mut (*psTreap).psRoot, key, (*psTreap).cmp);
    if psDel.is_null() { return 0 as libc::c_int }
    // Release the node
    heapFree((*psTreap).psNodes, psDel as *mut libc::c_void);
    return 1 as libc::c_int;
}
/* Recurisvely find an object in a treap */
#[no_mangle]
pub unsafe extern "C" fn treapFindRec(mut psRoot: *mut TREAP_NODE,
                                      mut key: UDWORD, mut cmp: TREAP_CMP)
 -> *mut libc::c_void {
    if psRoot.is_null() { return 0 as *mut libc::c_void }
    match cmp.expect("non-null function pointer")(key, (*psRoot).key) {
        0 => {
            // equal
            return (*psRoot).pObj
        }
        -1 => { return treapFindRec((*psRoot).psLeft, key, cmp) }
        1 => { return treapFindRec((*psRoot).psRight, key, cmp) }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"treapFindRec: invalid return from comparison\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"treap.c\x00" as *const u8 as *const libc::c_char,
                      315 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"treapFindRec\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 0 as *mut libc::c_void;
}
/* Find an object in a treap */
/* Find an object in a treap */
#[no_mangle]
pub unsafe extern "C" fn treapFind(mut psTreap: *mut TREAP, mut key: UDWORD)
 -> *mut libc::c_void {
    return treapFindRec((*psTreap).psRoot, key, (*psTreap).cmp);
}
/* Recursively free a treap */
unsafe extern "C" fn treapDestroyRec(mut psRoot: *mut TREAP_NODE,
                                     mut psHeap: *mut OBJ_HEAP) {
    if psRoot.is_null() { return }
    // free the sub branches
    treapDestroyRec((*psRoot).psLeft, psHeap);
    treapDestroyRec((*psRoot).psRight, psHeap);
    // free the root
    heapFree(psHeap, psRoot as *mut libc::c_void);
}
/* Release all the nodes in the treap */
/* Release all the nodes in the treap */
#[no_mangle]
pub unsafe extern "C" fn treapReset(mut psTreap: *mut TREAP) {
    treapDestroyRec((*psTreap).psRoot, (*psTreap).psNodes);
    (*psTreap).psRoot = 0 as *mut TREAP_NODE;
}
/* Destroy a treap and release all the memory associated with it */
/* Destroy a treap and release all the memory associated with it */
#[no_mangle]
pub unsafe extern "C" fn treapDestroy(mut psTreap: *mut TREAP) {
    treapDestroyRec((*psTreap).psRoot, (*psTreap).psNodes);
    heapDestroy((*psTreap).psNodes);
    memFreeRelease(psTreap as *mut libc::c_void);
    psTreap = 0 as *mut TREAP;
}
/* Recursively display the treap structure */
#[no_mangle]
pub unsafe extern "C" fn treapDisplayRec(mut psRoot: *mut TREAP_NODE,
                                         mut indent: UDWORD) {
    let mut i: UDWORD = 0;
    // Display the root
    debug(LOG_NEVER, b"%d,%d\n\x00" as *const u8 as *const libc::c_char,
          (*psRoot).key, (*psRoot).priority);
    // Display the left of the tree
    if !(*psRoot).psLeft.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < indent {
            debug(LOG_NEVER, b"\x00" as *const u8 as *const libc::c_char);
            i = i.wrapping_add(1)
        }
        debug(LOG_NEVER, b"L\x00" as *const u8 as *const libc::c_char);
        treapDisplayRec((*psRoot).psLeft,
                        indent.wrapping_add(1 as libc::c_int as
                                                libc::c_uint));
    }
    // Display the right of the tree
    if !(*psRoot).psRight.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < indent {
            debug(LOG_NEVER, b"\x00" as *const u8 as *const libc::c_char);
            i = i.wrapping_add(1)
        }
        debug(LOG_NEVER, b"R\x00" as *const u8 as *const libc::c_char);
        treapDisplayRec((*psRoot).psRight,
                        indent.wrapping_add(1 as libc::c_int as
                                                libc::c_uint));
    };
}
/* Display the treap structure using DBPRINTF */
/* Display the treap structure using DBPRINTF */
#[no_mangle]
pub unsafe extern "C" fn treapDisplay(mut psTreap: *mut TREAP) {
    if !(*psTreap).psRoot.is_null() {
        treapDisplayRec((*psTreap).psRoot, 0 as libc::c_int as UDWORD);
    };
}
#[no_mangle]
pub unsafe extern "C" fn treapGetSmallestRec(mut psRoot: *mut TREAP_NODE)
 -> *mut libc::c_void {
    if (*psRoot).psLeft.is_null() { return (*psRoot).pObj }
    return treapGetSmallestRec((*psRoot).psLeft);
}
/* Return the object with the smallest key in the treap
 * This is useful if the objects in the treap need to be
 * deallocated.  i.e. getSmallest, delete from treap, free memory
 */
/* Return the object with the smallest key in the treap
 * This is useful if the objects in the treap need to be
 * deallocated.  i.e. getSmallest, delete from treap, free memory
 */
#[no_mangle]
pub unsafe extern "C" fn treapGetSmallest(mut psTreap: *mut TREAP)
 -> *mut libc::c_void {
    if (*psTreap).psRoot.is_null() { return 0 as *mut libc::c_void }
    return treapGetSmallestRec((*psTreap).psRoot);
}
