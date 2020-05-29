use ::libc;
extern "C" {
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
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
    /* **************************************************************************/
/* functions
 */
    #[no_mangle]
    fn hashTable_Create(ppsTable: *mut *mut HASHTABLE, udwTableSize: UDWORD,
                        udwInitElements: UDWORD, udwExtElements: UDWORD,
                        udwElementSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn hashTable_Destroy(psTable: *mut HASHTABLE);
    #[no_mangle]
    fn hashTable_GetElement(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_InsertElement(psTable: *mut HASHTABLE,
                               psElement: *mut libc::c_void,
                               iKey1: libc::c_int, iKey2: libc::c_int);
    #[no_mangle]
    fn hashTable_RemoveElement(psTable: *mut HASHTABLE,
                               psElement: *mut libc::c_void,
                               iKey1: libc::c_int, iKey2: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn hashTable_FindElement(psTable: *mut HASHTABLE, iKey1: libc::c_int,
                             iKey2: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_GetFirst(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_GetNext(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_SetHashFunction(psTable: *mut HASHTABLE,
                                 pHashFunc: HASHFUNC);
    #[no_mangle]
    fn hashTable_SetFreeElementFunction(psTable: *mut HASHTABLE,
                                        pFreeFunc: HASHFREEFUNC);
    /*
 * GTime.h
 *
 * Interface to the game clock.
 *
 */
    /* The number of ticks per second for the game clock */
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn anim_GetAnim(uwAnimID: UWORD) -> *mut BASEANIM;
    #[no_mangle]
    fn anim_GetFrame3D(psAnim: *mut ANIM3D, uwObj: UWORD, udwGameTime: UDWORD,
                       udwStartTime: UDWORD, udwStartDelay: UDWORD,
                       psVecPos: *mut VECTOR3D, psVecRot: *mut VECTOR3D,
                       psVecScale: *mut VECTOR3D) -> UWORD;
}
pub type UBYTE = libc::c_uchar;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
// Routines to provide simple maths functions that work on both PSX & PC
// Use the type "FRACT" instead of FLOAT
//  - This is defined as a float on PC and a 20.12 fixed point number on PSX
//
//  Use:-
//		MAKEFRACT(int);  to convert from a SDWORD to a FRACT
//		MAKEINT(fract);	to convert the other way
//		FRACTmul(fract,fract); to multiply two fract numbers
//		FRACTdiv(fract,fract); to divide two numbers
//		SQRT(fract);		to get square root of a fract (returns a fract)
//      iSQRT(int);			to get a square root of an integer (returns an UDWORD)
//      FRACTCONST(constA,constB);	; Generates a constant of (constA/constB)
//                         e.g. to define 0.5 use FRACTCONST(1,2)
//                              to define 0.114 use FRACTCONT(114,1000)
//
// Also PERCENT(int,int);	// returns a int value 0->100 of the percentage of the first param over the second
//
// This file used to be in the deliverance src directory. But Jeremy quite correctly
// pointed out to me that it should be library based not deliverance based, and hence
// has now been moved to the lib\framework directory
//
// If you are reading this file from the deliverance source directory, please delete it now
// To multiply a FRACT by a integer just use the normal operator 
//   e.g.   FractValue2=FractValue*Interger;
//
// save is true of divide
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
pub type FRACT = libc::c_float;
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* defines
 */
/* flags key not used in hash function */
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVector {
    pub x: int32,
    pub y: int32,
    pub z: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iVertex {
    pub x: int32,
    pub y: int32,
    pub z: int32,
    pub u: int32,
    pub v: int32,
    pub g: uint8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLANE {
    pub a: FRACT,
    pub b: FRACT,
    pub c: FRACT,
    pub d: FRACT,
    pub vP: iVector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDPoly {
    pub flags: uint32,
    pub zcentre: int32,
    pub npnts: libc::c_int,
    pub normal: iVector,
    pub pindex: *mut VERTEXID,
    pub vrt: *mut iVertex,
    pub pTexAnim: *mut iTexAnim,
    pub BSP_NextPoly: BSPPOLYID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iIMDShape {
    pub flags: uint32,
    pub texpage: int32,
    pub oradius: int32,
    pub sradius: int32,
    pub radius: int32,
    pub visRadius: int32,
    pub xmin: int32,
    pub xmax: int32,
    pub ymin: int32,
    pub ymax: int32,
    pub zmin: int32,
    pub zmax: int32,
    pub ocen: iVector,
    pub numFrames: UWORD,
    pub animInterval: UWORD,
    pub npoints: libc::c_int,
    pub npolys: libc::c_int,
    pub nconnectors: libc::c_int,
    pub points: *mut iVector,
    pub polys: *mut iIMDPoly,
    pub connectors: *mut iVector,
    pub ntexanims: libc::c_int,
    pub texanims: *mut *mut iTexAnim,
    pub next: *mut iIMDShape,
    pub BSPNode: PSBSPTREENODE,
}
/* **************************************************************************/
/*
 * Anim.h
 *
 * Animation types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const ANIM_3D_TRANS: C2RustUnnamed = 2;
pub const ANIM_3D_FRAMES: C2RustUnnamed = 1;
pub const ANIM_2D: C2RustUnnamed = 0;
/* **************************************************************************/
/* ensure ANIM2D/3D structs same size */
/* width of container bitmap */
/* ensure ANIM2D/3D structs same size */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_STATE {
    pub uwFrame: UWORD,
    pub vecPos: VECTOR3D,
    pub vecAngle: VECTOR3D,
    pub vecScale: VECTOR3D,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VECTOR3D {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BASEANIM {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM3D {
    pub szFileName: [libc::c_char; 256],
    pub animType: libc::c_char,
    pub uwID: UWORD,
    pub uwFrameRate: UWORD,
    pub uwStates: UWORD,
    pub uwObj: UWORD,
    pub uwAnimTime: UWORD,
    pub ubType: UBYTE,
    pub psStates: *mut ANIM_STATE,
    pub psNext: *mut BASEANIM,
    pub psFrames: *mut iIMDShape,
    pub apFrame: *mut *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIM_OBJECT {
    pub psNext: *mut ANIM_OBJECT,
    pub uwID: UWORD,
    pub psAnim: *mut ANIM3D,
    pub psParent: *mut libc::c_void,
    pub udwStartTime: UDWORD,
    pub udwStartDelay: UDWORD,
    pub uwCycles: UWORD,
    pub bVisible: BOOL,
    pub pDoneFunc: ANIMOBJDONEFUNC,
    pub apComponents: [COMPONENT_OBJECT; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COMPONENT_OBJECT {
    pub position: VECTOR3D,
    pub orientation: VECTOR3D,
    pub psParent: *mut libc::c_void,
    pub psShape: *mut iIMDShape,
}
pub type ANIMOBJDONEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut ANIM_OBJECT) -> ()>;
pub type ANIMOBJDIEDTESTFUNC
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> BOOL>;
/* Allocation sizes for the anim heap */
/* **************************************************************************/
/* global variables */
static mut g_pAnimObjTable: *mut HASHTABLE =
    0 as *const HASHTABLE as *mut HASHTABLE;
static mut g_pDiedFunc: ANIMOBJDIEDTESTFUNC = None;
/* **************************************************************************/
/*
 * Anim functions
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Init(mut pDiedFunc: ANIMOBJDIEDTESTFUNC)
 -> BOOL {
    let mut iSize: SDWORD =
        ::std::mem::size_of::<ANIM_OBJECT>() as libc::c_ulong as SDWORD;
    /* allocate hashtable */
    hashTable_Create(&mut g_pAnimObjTable, 499 as libc::c_int as UDWORD,
                     100 as libc::c_int as UDWORD,
                     20 as libc::c_int as UDWORD, iSize as UDWORD);
    /* set local hash table functions */
    hashTable_SetHashFunction(g_pAnimObjTable,
                              Some(animObj_HashFunction as
                                       unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int)
                                           -> UINT));
    hashTable_SetFreeElementFunction(g_pAnimObjTable,
                                     Some(animObj_HashFreeElementFunc as
                                              unsafe extern "C" fn(_:
                                                                       *mut libc::c_void)
                                                  -> ()));
    /* set global died test function */
    g_pDiedFunc = pDiedFunc;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Shutdown() -> BOOL {
    /* destroy hash table */
    hashTable_Destroy(g_pAnimObjTable);
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_SetDoneFunc(mut psObj: *mut ANIM_OBJECT,
                                             mut pDoneFunc: ANIMOBJDONEFUNC) {
    (*psObj).pDoneFunc = pDoneFunc;
}
/* **************************************************************************/
/* local functions */
/* **************************************************************************/
/*
 * animObj_HashFunction
 *
 * Takes object pointer and id and returns hashed index
 */
/* **************************************************************************/
unsafe extern "C" fn animObj_HashFunction(mut iKey1: libc::c_int,
                                          mut iKey2: libc::c_int) -> UINT {
    return ((iKey1 + iKey2) % 499 as libc::c_int) as UINT;
}
/* **************************************************************************/
unsafe extern "C" fn animObj_HashFreeElementFunc(mut psElement:
                                                     *mut libc::c_void) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"animObj_HashFreeElementFunc: object pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"animobj.c\x00" as *const u8 as *const libc::c_char,
              117 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"animObj_HashFreeElementFunc\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(ANIM_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Update() {
    let mut psObj: *mut ANIM_OBJECT = 0 as *mut ANIM_OBJECT;
    let mut dwTime: SDWORD = 0;
    let mut bRemove: BOOL = 0;
    psObj = hashTable_GetFirst(g_pAnimObjTable) as *mut ANIM_OBJECT;
    while !psObj.is_null() {
        bRemove = 0 as libc::c_int;
        /* test whether parent object has died */
        if g_pDiedFunc.is_some() {
            bRemove =
                g_pDiedFunc.expect("non-null function pointer")((*psObj).psParent)
        }
        /* remove any expired (non-looping) animations */
        if bRemove == 0 as libc::c_int &&
               (*psObj).uwCycles as libc::c_int != 0 as libc::c_int {
            dwTime =
                gameTime.wrapping_sub((*psObj).udwStartTime).wrapping_sub((*psObj).udwStartDelay)
                    as SDWORD;
            if dwTime >
                   (*(*psObj).psAnim).uwAnimTime as libc::c_int *
                       (*psObj).uwCycles as libc::c_int {
                /* fire callback if set */
                if (*psObj).pDoneFunc.is_some() {
                    (*psObj).pDoneFunc.expect("non-null function pointer")(psObj);
                }
                bRemove = 1 as libc::c_int
            }
        }
        /* remove object if flagged */
        if bRemove == 1 as libc::c_int {
            if hashTable_RemoveElement(g_pAnimObjTable,
                                       psObj as *mut libc::c_void,
                                       (*psObj).psParent as libc::c_int,
                                       (*(*psObj).psAnim).uwID as libc::c_int)
                   == 0 as libc::c_int {
                debug(LOG_ERROR,
                      b"animObj_Update: couldn\'t remove anim obj\n\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
        }
        psObj = hashTable_GetNext(g_pAnimObjTable) as *mut ANIM_OBJECT
    };
}
/* **************************************************************************/
/*
 * anim_Add
 *
 * uwCycles=0 for infinite looping
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Add(mut pParentObj: *mut libc::c_void,
                                     mut iAnimID: libc::c_int,
                                     mut udwStartDelay: UDWORD,
                                     mut uwCycles: UWORD)
 -> *mut ANIM_OBJECT {
    let mut psObj: *mut ANIM_OBJECT = 0 as *mut ANIM_OBJECT;
    let mut psAnim: *mut BASEANIM = anim_GetAnim(iAnimID as UWORD);
    let mut i: UWORD = 0;
    let mut uwObj: UWORD = 0;
    if !psAnim.is_null() {
    } else {
        debug(LOG_ERROR,
              b"anim_AddAnimObject: anim id %i not found\n\x00" as *const u8
                  as *const libc::c_char, iAnimID);
    };
    if !psAnim.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"animobj.c\x00" as *const u8 as *const libc::c_char,
              190 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"animObj_Add\x00")).as_ptr(),
              b"psAnim != NULL\x00" as *const u8 as *const libc::c_char);
    };
    /* get object from table */
    psObj = hashTable_GetElement(g_pAnimObjTable) as *mut ANIM_OBJECT;
    if psObj.is_null() {
        debug(LOG_ERROR,
              b"animObj_Add: No room in hash table\n\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /* init object */
    (*psObj).uwID = iAnimID as UWORD;
    (*psObj).psAnim = psAnim as *mut ANIM3D;
    (*psObj).udwStartTime = gameTime;
    (*psObj).udwStartDelay = udwStartDelay;
    (*psObj).uwCycles = uwCycles;
    (*psObj).bVisible = 1 as libc::c_int;
    (*psObj).psParent = pParentObj;
    (*psObj).pDoneFunc = None;
    /* allocate component objects */
    if (*psAnim).animType as libc::c_int == ANIM_3D_TRANS as libc::c_int {
        uwObj = (*psAnim).uwObj
    } else { uwObj = (*psAnim).uwStates }
    if uwObj as libc::c_int > 10 as libc::c_int {
        debug(LOG_ERROR,
              b"animObj_Add: number of components too small\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    /* set parent pointer and shape pointer */
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < uwObj as libc::c_int {
        (*psObj).apComponents[i as usize].psParent = pParentObj;
        (*psObj).apComponents[i as usize].psShape =
            *(*(*psObj).psAnim).apFrame.offset(i as isize);
        i = i.wrapping_add(1)
    }
    /* insert object in table by parent */
    hashTable_InsertElement(g_pAnimObjTable, psObj as *mut libc::c_void,
                            pParentObj as libc::c_int, iAnimID);
    return psObj;
}
/* **************************************************************************/
/*
 * animObj_GetFrame3D
 *
 * returns NULL if animation not started yet
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_GetFrame3D(mut psObj: *mut ANIM_OBJECT,
                                            mut uwObj: UWORD,
                                            mut psVecPos: *mut VECTOR3D,
                                            mut psVecRot: *mut VECTOR3D,
                                            mut psVecScale: *mut VECTOR3D)
 -> UWORD {
    let mut psAnim: *mut ANIM3D = 0 as *mut ANIM3D;
    /* get local anim pointer */
    psAnim = (*psObj).psAnim;
    return anim_GetFrame3D(psAnim, uwObj, gameTime, (*psObj).udwStartTime,
                           (*psObj).udwStartDelay, psVecPos, psVecRot,
                           psVecScale);
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_GetFirst() -> *mut ANIM_OBJECT {
    let mut psObj: *mut ANIM_OBJECT = 0 as *mut ANIM_OBJECT;
    psObj = hashTable_GetFirst(g_pAnimObjTable) as *mut ANIM_OBJECT;
    if psObj.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"animObj_GetFirst: object pointer not valid\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if psObj.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"animobj.c\x00" as *const u8 as *const libc::c_char,
              275 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"animObj_GetFirst\x00")).as_ptr(),
              b"psObj == NULL || PTRVALID(psObj, sizeof(ANIM_OBJECT))\x00" as
                  *const u8 as *const libc::c_char);
    };
    return psObj;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_GetNext() -> *mut ANIM_OBJECT {
    return hashTable_GetNext(g_pAnimObjTable) as *mut ANIM_OBJECT;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Find(mut pParentObj: *mut libc::c_void,
                                      mut iAnimID: libc::c_int)
 -> *mut ANIM_OBJECT {
    return hashTable_FindElement(g_pAnimObjTable, pParentObj as libc::c_int,
                                 iAnimID) as *mut ANIM_OBJECT;
}
/* **************************************************************************/
/*
 * Animobj.h
 *
 * Animation object types and function headers
 *
 * Gareth Jones 14/11/97
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/* forward struct declarations */
/* **************************************************************************/
/* typedefs */
/* **************************************************************************/
/* struct member macros */
/* this must be the last entry in this structure */
/* **************************************************************************/
/* this must be the last entry in this structure */
/* **************************************************************************/
/* uwCycles=0 for infinite looping */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn animObj_Remove(mut ppsObj: *mut *mut ANIM_OBJECT,
                                        mut iAnimID: libc::c_int) -> BOOL {
    let mut bRemOK: BOOL =
        hashTable_RemoveElement(g_pAnimObjTable, *ppsObj as *mut libc::c_void,
                                (**ppsObj).psParent as libc::c_int, iAnimID);
    //init the animation
    *ppsObj = 0 as *mut ANIM_OBJECT;
    return bRemOK;
}
/* **************************************************************************/
