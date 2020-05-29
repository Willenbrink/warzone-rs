use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
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
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn ParseResourceFile(pData: *mut libc::c_char, fileSize: UDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type DWORD = libc::c_int;
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
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
}
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
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
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
//*************************************************************************
//
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
// currently uses 4k per structure (!)
//*************************************************************************
//
// texture animation structures
//
//*************************************************************************
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
pub type C2RustUnnamed = libc::c_uint;
pub const ANIM_3D_TRANS: C2RustUnnamed = 2;
pub const ANIM_3D_FRAMES: C2RustUnnamed = 1;
pub const ANIM_2D: C2RustUnnamed = 0;
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
pub struct ANIM2D {
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
    pub psFrames: *mut iSprite,
    pub uwBmapWidth: UWORD,
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
/* **************************************************************************/
pub type GETSHAPEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut STRING) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ANIMGLOBALS {
    pub psAnimList: *mut BASEANIM,
    pub uwCurObj: UWORD,
    pub uwCurState: UWORD,
    pub pGetShapeFunc: GETSHAPEFUNC,
}
/* **************************************************************************/
/*
 * Anim.c
 *
 * Anim functions
 *
 * Gareth Jones 11/7/97
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* structs */
/* **************************************************************************/
/* global variables */
#[no_mangle]
pub static mut g_animGlobals: ANIMGLOBALS =
    ANIMGLOBALS{psAnimList: 0 as *const BASEANIM as *mut BASEANIM,
                uwCurObj: 0,
                uwCurState: 0,
                pGetShapeFunc: None,};
/* **************************************************************************/
/* **************************************************************************/
/* local functions */
//static UINT	anim_HashFunction( int iKey1, int iKey2 );
/* **************************************************************************/
/*
 * Anim functions
 */
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_Init(mut pGetShapeFunc: GETSHAPEFUNC) -> BOOL {
    let mut iSizeAnim2D: libc::c_int =
        ::std::mem::size_of::<ANIM2D>() as libc::c_ulong as libc::c_int;
    let mut iSizeAnim3D: libc::c_int =
        ::std::mem::size_of::<ANIM3D>() as libc::c_ulong as libc::c_int;
    /* ensure ANIM2D and ANIM3D structs same size */
    if iSizeAnim2D != iSizeAnim3D {
        debug(LOG_ERROR,
              b"anim_Init: ANIM2D and ANIM3D structs not same size in anim.h!\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    /* init globals */
    g_animGlobals.psAnimList = 0 as *mut BASEANIM;
    g_animGlobals.uwCurObj = 0 as libc::c_int as UWORD;
    g_animGlobals.uwCurState = 0 as libc::c_int as UWORD;
    g_animGlobals.pGetShapeFunc = pGetShapeFunc;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_ReleaseAnim(mut psAnim: *mut BASEANIM) {
    let mut psAnim3D: *mut ANIM3D = 0 as *mut ANIM3D;
    // remove the anim from the list
    let mut psPrev: *mut BASEANIM = 0 as *mut BASEANIM;
    let mut psCurr: *mut BASEANIM = 0 as *mut BASEANIM;
    psPrev = 0 as *mut BASEANIM;
    psCurr = g_animGlobals.psAnimList;
    while !psCurr.is_null() {
        if psCurr == psAnim { break ; }
        psPrev = psCurr;
        psCurr = (*psCurr).psNext
    }
    if !psCurr.is_null() {
    } else {
        debug(LOG_ERROR,
              b"LIST_REMOVE: anim.c(%d): entry not found\x00" as *const u8 as
                  *const libc::c_char, 70 as libc::c_int);
    };
    if !psCurr.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"anim.c\x00" as *const u8 as *const libc::c_char,
              70 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"anim_ReleaseAnim\x00")).as_ptr(),
              b"psCurr!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    if psPrev.is_null() {
        g_animGlobals.psAnimList = (*g_animGlobals.psAnimList).psNext
    } else if !psCurr.is_null() { (*psPrev).psNext = (*psCurr).psNext }
    /* free anim scripts */
    memFreeRelease((*psAnim).psStates as *mut libc::c_void);
    (*psAnim).psStates = 0 as *mut ANIM_STATE;
    /* free anim shape */
    if (*psAnim).animType as libc::c_int == ANIM_3D_FRAMES as libc::c_int ||
           (*psAnim).animType as libc::c_int == ANIM_3D_TRANS as libc::c_int {
        psAnim3D = psAnim as *mut ANIM3D;
        memFreeRelease((*psAnim3D).apFrame as *mut libc::c_void);
        (*psAnim3D).apFrame = 0 as *mut *mut iIMDShape
    }
    memFreeRelease(psAnim as *mut libc::c_void);
    psAnim = 0 as *mut BASEANIM;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_Shutdown() -> BOOL {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    let mut psAnimTmp: *mut BASEANIM = 0 as *mut BASEANIM;
    if !g_animGlobals.psAnimList.is_null() {
        debug(LOG_NEVER,
              b"anim_Shutdown: warning: anims still allocated\x00" as
                  *const u8 as *const libc::c_char);
    }
    /* empty anim list */
    psAnim = g_animGlobals.psAnimList;
    while !psAnim.is_null() {
        psAnimTmp = (*psAnim).psNext;
        anim_ReleaseAnim(psAnim);
        psAnim = psAnimTmp
    }
    return 1 as libc::c_int;
}
/* **************************************************************************/
unsafe extern "C" fn anim_InitBaseMembers(mut psAnim: *mut BASEANIM,
                                          mut uwStates: UWORD,
                                          mut uwFrameRate: UWORD,
                                          mut uwObj: UWORD, mut ubType: UBYTE,
                                          mut uwID: UWORD) {
    (*psAnim).uwStates = uwStates;
    (*psAnim).uwFrameRate = uwFrameRate;
    (*psAnim).uwObj = uwObj;
    (*psAnim).ubType = ubType;
    (*psAnim).uwID = uwID;
    (*psAnim).uwAnimTime =
        (uwStates as libc::c_int * 1000 as libc::c_int /
             (*psAnim).uwFrameRate as libc::c_int) as UWORD;
    /* allocate frames */
    (*psAnim).psStates =
        memMallocRelease(((uwObj as libc::c_int *
                               (*psAnim).uwStates as libc::c_int) as
                              libc::c_uint).wrapping_mul(::std::mem::size_of::<ANIM_STATE>()
                                                             as
                                                             libc::c_ulong))
            as *mut ANIM_STATE;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_Create3D(mut szPieFileName: *mut libc::c_char,
                                       mut uwStates: UWORD,
                                       mut uwFrameRate: UWORD,
                                       mut uwObj: UWORD, mut ubType: UBYTE,
                                       mut uwID: UWORD) -> BOOL {
    let mut psAnim3D: *mut ANIM3D = 0 as *mut ANIM3D;
    let mut psFrames: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut uwFrames: UWORD = 0;
    let mut i: UWORD = 0;
    /* allocate anim */
    psAnim3D =
        memMallocRelease(::std::mem::size_of::<ANIM3D>() as libc::c_ulong) as
            *mut ANIM3D;
    if psAnim3D.is_null() { return 0 as libc::c_int }
    /* get local pointer to shape */
    (*psAnim3D).psFrames =
        g_animGlobals.pGetShapeFunc.expect("non-null function pointer")(szPieFileName)
            as *mut iIMDShape;
    /* count frames in imd */
    psFrames = (*psAnim3D).psFrames;
    uwFrames = 0 as libc::c_int as UWORD;
    while !psFrames.is_null() {
        uwFrames = uwFrames.wrapping_add(1);
        psFrames = (*psFrames).next
    }
    /* check frame count matches script */
    if ubType as libc::c_int == ANIM_3D_TRANS as libc::c_int &&
           uwObj as libc::c_int != uwFrames as libc::c_int {
        debug(LOG_ERROR,
              b"anim_Create3D: frames in pie %s != script objects %i\n\x00" as
                  *const u8 as *const libc::c_char, szPieFileName,
              uwObj as libc::c_int);
        abort();
    }
    /* get pointers to individual frames */
    (*psAnim3D).apFrame =
        memMallocRelease((uwFrames as
                              libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut iIMDShape>()
                                                             as
                                                             libc::c_ulong))
            as *mut *mut iIMDShape;
    psFrames = (*psAnim3D).psFrames;
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_int) < uwFrames as libc::c_int {
        let ref mut fresh0 = *(*psAnim3D).apFrame.offset(i as isize);
        *fresh0 = psFrames;
        psFrames = (*psFrames).next;
        i = i.wrapping_add(1)
    }
    /* init members */
    (*psAnim3D).animType = ubType as libc::c_char;
    anim_InitBaseMembers(psAnim3D as *mut BASEANIM, uwStates, uwFrameRate,
                         uwObj, ubType, uwID);
    /* add to head of list */
    (*psAnim3D).psNext = g_animGlobals.psAnimList;
    g_animGlobals.psAnimList = psAnim3D as *mut BASEANIM;
    /* update globals */
    g_animGlobals.uwCurObj = 0 as libc::c_int as UWORD;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_BeginScript() {
    /* update globals */
    g_animGlobals.uwCurState = 0 as libc::c_int as UWORD;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_EndScript() -> BOOL {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    /* get pointer to current anim */
    psAnim = g_animGlobals.psAnimList;
    if g_animGlobals.uwCurState as libc::c_int !=
           (*psAnim).uwStates as libc::c_int {
        debug(LOG_ERROR,
              b"anim_End3D: states in current anim not consistent with header\n\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    /* update globals */
    g_animGlobals.uwCurObj = g_animGlobals.uwCurObj.wrapping_add(1);
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_AddFrameToAnim(mut iFrame: libc::c_int,
                                             mut vecPos: VECTOR3D,
                                             mut vecRot: VECTOR3D,
                                             mut vecScale: VECTOR3D) -> BOOL {
    let mut psState: *mut ANIM_STATE = 0 as *mut ANIM_STATE;
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    let mut uwState: UWORD = 0;
    /* get pointer to current anim */
    psAnim = g_animGlobals.psAnimList;
    /* check current anim valid */
    if !psAnim.is_null() {
    } else {
        debug(LOG_ERROR,
              b"anim_AddFrameToAnim: NULL current anim\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psAnim.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"anim.c\x00" as *const u8 as *const libc::c_char,
              241 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"anim_AddFrameToAnim\x00")).as_ptr(),
              b"psAnim != NULL\x00" as *const u8 as *const libc::c_char);
    };
    /* check frame number in range */
    if iFrame < (*psAnim).uwStates as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"anim_AddFrameToAnim: frame number %i > %i frames in imd\n\x00"
                  as *const u8 as *const libc::c_char, iFrame,
              (*psAnim).uwObj as libc::c_int);
    };
    if iFrame < (*psAnim).uwStates as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"anim.c\x00" as *const u8 as *const libc::c_char,
              246 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"anim_AddFrameToAnim\x00")).as_ptr(),
              b"iFrame<psAnim->uwStates\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* get state */
    uwState =
        (g_animGlobals.uwCurObj as libc::c_int *
             (*psAnim).uwStates as libc::c_int +
             g_animGlobals.uwCurState as libc::c_int) as UWORD;
    psState =
        &mut *(*psAnim).psStates.offset(uwState as isize) as *mut ANIM_STATE;
    /* set state pointer */
    (*psState).uwFrame = iFrame as UWORD;
    (*psState).vecPos.x = vecPos.x;
    (*psState).vecPos.y = vecPos.y;
    (*psState).vecPos.z = vecPos.z;
    /* max anims right-handed; Necromancer anims left */
    (*psState).vecAngle.x = vecRot.x;
    (*psState).vecAngle.y = vecRot.y;
    (*psState).vecAngle.z = vecRot.z;
    (*psState).vecScale.x = vecScale.x;
    (*psState).vecScale.y = vecScale.y;
    (*psState).vecScale.z = vecScale.z;
    /* update globals */
    g_animGlobals.uwCurState = g_animGlobals.uwCurState.wrapping_add(1);
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_GetAnim(mut uwAnimID: UWORD) -> *mut BASEANIM {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    /* find matching anim id in list */
    psAnim = g_animGlobals.psAnimList;
    while !psAnim.is_null() &&
              (*psAnim).uwID as libc::c_int != uwAnimID as libc::c_int {
        psAnim = (*psAnim).psNext
    }
    return psAnim;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_SetVals(mut szFileName: *mut libc::c_char,
                                      mut uwAnimID: UWORD) {
    /* get track pointer from resource */
    let mut psAnim: *mut BASEANIM =
        resGetData(b"ANI\x00" as *const u8 as *const libc::c_char as
                       *mut STRING, szFileName) as *mut BASEANIM;
    if psAnim.is_null() {
        debug(LOG_ERROR,
              b"anim_SetVals: can\'t find anim %s\n\x00" as *const u8 as
                  *const libc::c_char, szFileName);
        abort();
    }
    /* set anim vals */
    (*psAnim).uwID = uwAnimID;
    strcpy((*psAnim).szFileName.as_mut_ptr(),
           szFileName as *const libc::c_char);
}
/* **************************************************************************/
// the playstation version uses sscanf's ... see animload.c
#[no_mangle]
pub unsafe extern "C" fn anim_LoadFromBuffer(mut pBuffer: *mut libc::c_char,
                                             mut size: UDWORD)
 -> *mut BASEANIM {
    if ParseResourceFile(pBuffer, size) == 0 as libc::c_int {
        debug(LOG_ERROR,
              b"anim_LoadFromBuffer: couldn\'t parse file\n\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    /* loaded anim is at head of list */
    return g_animGlobals.psAnimList;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_GetAnimID(mut szName: *mut libc::c_char)
 -> UWORD {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    let mut cPos: *mut libc::c_char =
        strstr(szName, b".ani\x00" as *const u8 as *const libc::c_char);
    if cPos.is_null() {
        debug(LOG_ERROR,
              b"anim_GetAnimID: %s isn\'t .ani file\n\x00" as *const u8 as
                  *const libc::c_char, szName);
        abort();
    }
    /* find matching anim string in list */
    psAnim = g_animGlobals.psAnimList;
    while !psAnim.is_null() &&
              strcasecmp((*psAnim).szFileName.as_mut_ptr(), szName) !=
                  0 as libc::c_int {
        psAnim = (*psAnim).psNext
    }
    if !psAnim.is_null() {
        return (*psAnim).uwID
    } else { return 0xfffd as libc::c_int as UWORD };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_GetShapeFromID(mut uwID: UWORD)
 -> *mut iIMDShape {
    let mut psAnim: *mut BASEANIM = 0 as *mut BASEANIM;
    let mut psAnim3D: *mut ANIM3D = 0 as *mut ANIM3D;
    /* find matching anim id in list */
    psAnim = g_animGlobals.psAnimList;
    while !psAnim.is_null() &&
              (*psAnim).uwID as libc::c_int != uwID as libc::c_int {
        psAnim = (*psAnim).psNext
    }
    if psAnim.is_null() {
        return 0 as *mut iIMDShape
    } else {
        psAnim3D = psAnim as *mut ANIM3D;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"anim_GetShapeFromID: invalid anim pointer\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"anim.c\x00" as *const u8 as *const libc::c_char,
                  386 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"anim_GetShapeFromID\x00")).as_ptr(),
                  b"PTRVALID( psAnim3D, sizeof(ANIM3D))\x00" as *const u8 as
                      *const libc::c_char);
        };
        return (*psAnim3D).psFrames
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn anim_GetFrame3D(mut psAnim: *mut ANIM3D,
                                         mut uwObj: UWORD,
                                         mut udwGameTime: UDWORD,
                                         mut udwStartTime: UDWORD,
                                         mut udwStartDelay: UDWORD,
                                         mut psVecPos: *mut VECTOR3D,
                                         mut psVecRot: *mut VECTOR3D,
                                         mut psVecScale: *mut VECTOR3D)
 -> UWORD {
    let mut dwTime: DWORD = 0;
    let mut uwState: UWORD = 0;
    let mut uwFrame: UWORD = 0;
    let mut psState: *mut ANIM_STATE = 0 as *mut ANIM_STATE;
    /* calculate current anim frame */
    dwTime =
        udwGameTime.wrapping_sub(udwStartTime).wrapping_sub(udwStartDelay) as
            DWORD;
    /* return NULL if animation still delayed */
    if dwTime < 0 as libc::c_int { return 0xfffe as libc::c_int as UWORD }
    uwFrame =
        (dwTime % (*psAnim).uwAnimTime as libc::c_int *
             (*psAnim).uwFrameRate as libc::c_int / 1000 as libc::c_int) as
            UWORD;
    /* check in range */
    if (uwFrame as libc::c_int) < (*psAnim).uwStates as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"anim_GetObjectFrame3D: error in animation calculation\n\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (uwFrame as libc::c_int) < (*psAnim).uwStates as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"anim.c\x00" as *const u8 as *const libc::c_char,
              417 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"anim_GetFrame3D\x00")).as_ptr(),
              b"uwFrame<psAnim->uwStates\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* find current state */
    uwState =
        (uwObj as libc::c_int * (*psAnim).uwStates as libc::c_int +
             uwFrame as libc::c_int) as UWORD;
    psState =
        &mut *(*psAnim).psStates.offset(uwState as isize) as *mut ANIM_STATE;
    (*psVecPos).x = (*psState).vecPos.x / 1000 as libc::c_int;
    (*psVecPos).y = (*psState).vecPos.y / 1000 as libc::c_int;
    (*psVecPos).z = (*psState).vecPos.z / 1000 as libc::c_int;
    (*psVecRot).x =
        (*psState).vecAngle.x * (65536 as libc::c_int / 360 as libc::c_int) /
            1000 as libc::c_int;
    (*psVecRot).y =
        (*psState).vecAngle.y * (65536 as libc::c_int / 360 as libc::c_int) /
            1000 as libc::c_int;
    (*psVecRot).z =
        (*psState).vecAngle.z * (65536 as libc::c_int / 360 as libc::c_int) /
            1000 as libc::c_int;
    (*psVecScale).x = (*psState).vecScale.x;
    (*psVecScale).y = (*psState).vecScale.y;
    (*psVecScale).z = (*psState).vecScale.z;
    if (*psAnim).ubType as libc::c_int == ANIM_3D_TRANS as libc::c_int {
        return uwFrame
    } else { return (*(*psAnim).psStates.offset(uwState as isize)).uwFrame };
}
/* **************************************************************************/
