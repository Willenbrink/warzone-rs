use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    static mut aSinTable: [SDWORD; 0];
    /*

	BSP Draw routines for iVis02

	- 5 Sept 1997  - Tim Cannell - Pumpkin Studios - Eidos



	- Main routines were written by Gareth Jones .... so if you find any bugs I think you better speak to him ...

*/
    // this can have the #define for BSPIMD in it
    // covers the whole file
    // if defined we perform the backface cull in the BSP (we get this for free (!))
    //#define BSP_MAXDEBUG		// define this if you want max debug options (runs very slow)
    // from imddraw.c
    #[no_mangle]
    fn DrawTriangleList(PolygonNumber: BSPPOLYID);
}
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
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
pub type FRACT = libc::c_float;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
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
pub struct iVectorf {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
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
pub type BSPPOLYID = uint16;
pub type WORLDCOORD = UDWORD;
// only needed when generating the tree
pub type ANGLE = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBJPOS {
    pub x: WORLDCOORD,
    pub y: WORLDCOORD,
    pub z: WORLDCOORD,
    pub pitch: ANGLE,
    pub yaw: ANGLE,
    pub roll: ANGLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BNODE {
    pub link: [*mut BNODE; 2],
}
pub type PSBNODE = *mut BNODE;
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
pub type PSPLANE = *mut PLANE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
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
/*
	These routines are used by the IMD Load_BSP routine
*/
pub type PSTRIANGLE = *mut iIMDPoly;
#[no_mangle]
pub static mut BSPimd: *mut iIMDShape =
    0 as *const iIMDShape as *mut iIMDShape;
// This is a global ... it is used in imddraw.c (for speed)
// Local static variables
static mut BSPScrPos: *mut iVector = 0 as *const iVector as *mut iVector;
static mut CurrentVertexList: *mut iVector =
    0 as *const iVector as *mut iVector;
// Local prototypes
// Oh yes... a global externaly referenced variable....
// General routines that work on both PC & PSX
/* **************************************************************************/
/*
 * Calculates whether point is on same side, opposite side or in plane;
 *
 * returns OPPOSITE_SIDE if opposite,
 *         IN_PLANE if contained in plane,
 *         SAME_SIDE if same side
 *       Also returns pvDot - the dot product
 * - inputs vP vector to the point
 * - psPlane structure containing the plane equation
 */
/* **************************************************************************/
#[inline]
unsafe extern "C" fn IsPointOnPlane(mut psPlane: PSPLANE,
                                    mut vP: *mut iVector) -> libc::c_int {
    let mut vecP: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut Dot: FRACT = 0.;
    /* validate input */
    /* subtract point on plane from input point to get position vector */
    vecP.x = ((*vP).x - (*psPlane).vP.x) as FRACT as libc::c_double;
    vecP.y = ((*vP).y - (*psPlane).vP.y) as FRACT as libc::c_double;
    vecP.z = ((*vP).z - (*psPlane).vP.z) as FRACT as libc::c_double;
    /* get dot product of result with plane normal (a,b,c of plane) */
    Dot =
        (vecP.x * (*psPlane).a as libc::c_double +
             vecP.y * (*psPlane).b as libc::c_double +
             vecP.z * (*psPlane).c as libc::c_double) as FRACT;
    /* if result is -ve, return -1 */
    if (abs(Dot as libc::c_int) as libc::c_float) <
           1 as libc::c_int as libc::c_float /
               100 as libc::c_int as libc::c_float {
        return 1 as libc::c_int
    } else {
        if Dot < 0 as libc::c_int as libc::c_float { return 0 as libc::c_int }
    }
    return 2 as libc::c_int;
}
/*
	This is the main BSP Traversal routine. It Zaps through the tree (recursively) - and draws all the polygons
	for the IMD in the correct order ... pretty clever eh ..
*/
unsafe extern "C" fn TraverseTreeAndRender(mut psNode: PSBSPTREENODE) {
    /* is viewer on same side? */
// On the playstation we need the list in reverse order (front most polygon first)
// so we just do the list the opposite way around - this affects the BACKFACE culling as well
    if IsPointOnPlane(&mut (*psNode).Plane, BSPScrPos) == 2 as libc::c_int {
        /* recurse on opposite side, render this node on same side,
		 * recurse on same side.
		 */
        if !(*psNode).link[1 as libc::c_int as usize].is_null() {
            TraverseTreeAndRender((*psNode).link[1 as libc::c_int as usize]);
        }
        if (*psNode).TriSameDir as libc::c_int !=
               65534 as libc::c_int + 1 as libc::c_int {
            DrawTriangleList((*psNode).TriSameDir);
        }
        if !(*psNode).link[0 as libc::c_int as usize].is_null() {
            TraverseTreeAndRender((*psNode).link[0 as libc::c_int as usize]);
        }
    } else {
        /* viewer in plane or on opposite side */
        /* recurse on same side, render this node on opposite side
		 * recurse on opposite side.
		 */
        if !(*psNode).link[0 as libc::c_int as usize].is_null() {
            TraverseTreeAndRender((*psNode).link[0 as libc::c_int as usize]);
        }
        if (*psNode).TriOppoDir as libc::c_int !=
               65534 as libc::c_int + 1 as libc::c_int {
            DrawTriangleList((*psNode).TriOppoDir);
        }
        if !(*psNode).link[1 as libc::c_int as usize].is_null() {
            TraverseTreeAndRender((*psNode).link[1 as libc::c_int as usize]);
        }
    };
}
// little routine for getting an imd vector structure in the IMD from the vertex ID
#[inline]
unsafe extern "C" fn IMDvec(mut Vertex: libc::c_int) -> *mut iVector {
    return CurrentVertexList.offset(Vertex as isize);
}
/*
 Its easy enough to calculate the plane equation if there is only 3 points
 ... but if there is four things get a little tricky ...

In theory you should be able the pick any 3 of the points to calculate the equation, however
in practise mathematically inacurraceys mean that  you need the three points that are the furthest apart

*/
#[no_mangle]
pub unsafe extern "C" fn GetPlane(mut s: *mut iIMDShape,
                                  mut PolygonID: UDWORD,
                                  mut psPlane: PSPLANE) {
    let mut Result: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut psTri: *mut iIMDPoly = 0 as *mut iIMDPoly;
    /* validate input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"GetPlane: invalid plane\n\x00" as *const u8 as
                  *const libc::c_char); // Distance between two points in the quad
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bspimd.c\x00" as *const u8 as *const libc::c_char,
              191 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"GetPlane\x00")).as_ptr(),
              b"PTRVALID(psPlane,sizeof(PLANE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psTri = &mut *(*s).polys.offset(PolygonID as isize) as *mut iIMDPoly;
    CurrentVertexList = (*s).points;
    if (*psTri).npnts == 4 as libc::c_int {
        let mut pA: libc::c_int = 0;
        let mut pB: libc::c_int = 0;
        let mut pC: libc::c_int = 0;
        let mut ShortDist: FRACT = 0.;
        let mut Dist: FRACT = 0.;
        ShortDist = 999 as libc::c_int as FRACT;
        pA = 0 as libc::c_int;
        pB = 1 as libc::c_int;
        pC = 2 as libc::c_int;
        Dist = GetDist(psTri, 0 as libc::c_int, 1 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 2 as libc::c_int;
            pC = 3 as libc::c_int
        }
        Dist = GetDist(psTri, 0 as libc::c_int, 2 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 1 as libc::c_int;
            pC = 3 as libc::c_int
        }
        Dist = GetDist(psTri, 0 as libc::c_int, 3 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 1 as libc::c_int;
            pC = 2 as libc::c_int
        }
        Dist = GetDist(psTri, 1 as libc::c_int, 2 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 1 as libc::c_int;
            pC = 3 as libc::c_int
        }
        Dist = GetDist(psTri, 1 as libc::c_int, 3 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 1 as libc::c_int;
            pC = 2 as libc::c_int
        }
        Dist = GetDist(psTri, 2 as libc::c_int, 3 as libc::c_int);
        if Dist < ShortDist {
            ShortDist = Dist;
            pA = 0 as libc::c_int;
            pB = 1 as libc::c_int;
            pC = 2 as libc::c_int
        }
        GetTriangleNormal(psTri, &mut Result, pA, pB, pC);
    } else {
        GetTriangleNormal(psTri, &mut Result, 0 as libc::c_int,
                          1 as libc::c_int, 2 as libc::c_int);
        // for a triangle there is no choice ...
    }
    /* normalise normal */
    iNormalise(&mut Result);
    // This result MUST be cast to a fract and not called using MAKEFRACT
//
//  This is because on a playstation we are casting from FRACT->FRACT (so no conversion is needed)
//    ... and on a PC we are converting from DOUBLE->FLOAT  (so a cast is needed)
    (*psPlane).a = Result.x as FRACT;
    (*psPlane).b = Result.y as FRACT;
    (*psPlane).c = Result.z as FRACT;
    /* since plane eqn is ax + by + cz + d = 0,
	 * d = -(ax + by + cz).
	 */
	// Because we are multiplying a FRACT by a const we can use normal '*'
    (*psPlane).d =
        -((*psPlane).a *
              (*IMDvec(*(*psTri).pindex.offset(0 as libc::c_int as isize))).x
                  as libc::c_float +
              (*psPlane).b *
                  (*IMDvec(*(*psTri).pindex.offset(0 as libc::c_int as
                                                       isize))).y as
                      libc::c_float +
              (*psPlane).c *
                  (*IMDvec(*(*psTri).pindex.offset(0 as libc::c_int as
                                                       isize))).z as
                      libc::c_float);
    /* store first triangle vertex as point on plane for later point /
	 * plane classification in IsPointOnPlane
	 */
    memcpy(&mut (*psPlane).vP as *mut iVector as *mut libc::c_void,
           IMDvec(*(*psTri).pindex.offset(0 as libc::c_int as isize)) as
               *const libc::c_void,
           ::std::mem::size_of::<iVector>() as libc::c_ulong);
}
/* **************************************************************************/
/*
 * iCrossProduct
 *
 * Calculates cross product of a and b.
 */
/* **************************************************************************/
unsafe extern "C" fn iCrossProduct(mut psD: *mut iVectorf,
                                   mut psA: *mut iVectorf,
                                   mut psB: *mut iVectorf) -> *mut iVectorf {
    (*psD).x = (*psA).y * (*psB).z - (*psA).z * (*psB).y;
    (*psD).y = (*psA).z * (*psB).x - (*psA).x * (*psB).z;
    (*psD).z = (*psA).x * (*psB).y - (*psA).y * (*psB).x;
    return psD;
}
unsafe extern "C" fn GetDist(mut psTri: PSTRIANGLE, mut pA: libc::c_int,
                             mut pB: libc::c_int) -> FRACT {
    let mut vx: FRACT = 0.;
    let mut vy: FRACT = 0.;
    let mut vz: FRACT = 0.;
    let mut sum_square: FRACT = 0.;
    let mut dist: FRACT = 0.;
    vx =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).x -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).x) as FRACT;
    vy =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).y -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).y) as FRACT;
    vz =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).z -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).z) as FRACT;
    sum_square = vx * vx + vy * vy + vz * vz;
    dist = sqrt(sum_square as libc::c_double) as FRACT;
    return dist;
}
unsafe extern "C" fn GetTriangleNormal(mut psTri: PSTRIANGLE,
                                       mut psN: *mut iVectorf,
                                       mut pA: libc::c_int,
                                       mut pB: libc::c_int,
                                       mut pC: libc::c_int) {
    let mut vecA: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    let mut vecB: iVectorf = iVectorf{x: 0., y: 0., z: 0.,};
    /* validate input */
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"GetTriangleNormal: invalid triangle\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bspimd.c\x00" as *const u8 as *const libc::c_char,
              297 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"GetTriangleNormal\x00")).as_ptr(),
              b"PTRVALID(psTri,sizeof(iIMDPoly))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* get triangle edge vectors */
    vecA.x =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).x -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).x) as FRACT as
            libc::c_double;
    vecA.y =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).y -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).y) as FRACT as
            libc::c_double;
    vecA.z =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).z -
             (*IMDvec(*(*psTri).pindex.offset(pB as isize))).z) as FRACT as
            libc::c_double;
    vecB.x =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).x -
             (*IMDvec(*(*psTri).pindex.offset(pC as isize))).x) as FRACT as
            libc::c_double;
    vecB.y =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).y -
             (*IMDvec(*(*psTri).pindex.offset(pC as isize))).y) as FRACT as
            libc::c_double;
    vecB.z =
        ((*IMDvec(*(*psTri).pindex.offset(pA as isize))).z -
             (*IMDvec(*(*psTri).pindex.offset(pC as isize))).z) as FRACT as
            libc::c_double;
    /* normal is cross product */
    iCrossProduct(psN, &mut vecA, &mut vecB);
}
/* **************************************************************************/
/*
 * Normalises the vector v
 */
/* **************************************************************************/
unsafe extern "C" fn iNormalise(mut v: *mut iVectorf) -> *mut iVectorf {
    let mut vx: FRACT = 0.;
    let mut vy: FRACT = 0.;
    let mut vz: FRACT = 0.;
    let mut mod_0: FRACT = 0.;
    let mut sum_square: FRACT = 0.;
    vx = (*v).x as FRACT;
    vy = (*v).y as FRACT;
    vz = (*v).z as FRACT;
    if vx == 0 as libc::c_int as libc::c_float &&
           vy == 0 as libc::c_int as libc::c_float &&
           vz == 0 as libc::c_int as libc::c_float {
        return v
    }
    sum_square = vx * vx + vy * vy + vz * vz;
    mod_0 = sqrt(sum_square as libc::c_double) as FRACT;
    (*v).x = (vx / mod_0) as libc::c_double;
    (*v).y = (vy / mod_0) as libc::c_double;
    (*v).z = (vz / mod_0) as libc::c_double;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn InitNode(mut psBSPNode: PSBSPTREENODE)
 -> PSBSPTREENODE {
    let mut psBNode: PSBNODE = 0 as *mut BNODE;
    /* create node triangle lists */
    (*psBSPNode).TriSameDir =
        (65534 as libc::c_int + 1 as libc::c_int) as BSPPOLYID;
    (*psBSPNode).TriOppoDir =
        (65534 as libc::c_int + 1 as libc::c_int) as BSPPOLYID;
    psBNode = psBSPNode as PSBNODE;
    (*psBNode).link[1 as libc::c_int as usize] = 0 as *mut BNODE;
    (*psBNode).link[0 as libc::c_int as usize] = 0 as *mut BNODE;
    return psBSPNode;
}
/* **************************************************************************/
// PC Specific drawing routines
// Calculate the real camera position based on the coordinates of the camera and the camera
// distance - the result is stores in CameraLoc ,,  up is +ve Y
#[no_mangle]
pub unsafe extern "C" fn GetRealCameraPos(mut Camera: *mut OBJPOS,
                                          mut Distance: SDWORD,
                                          mut CameraLoc: *mut iVector) {
    let mut Yinc: libc::c_int = 0;
    //  as pitch is negative ... we need to subtract the value from y to go up the axis
    (*CameraLoc).y =
        (*Camera).y.wrapping_sub((*aSinTable.as_mut_ptr().offset(((*Camera).pitch
                                                                      as
                                                                      uint16
                                                                      as
                                                                      libc::c_int
                                                                      >>
                                                                      4 as
                                                                          libc::c_int)
                                                                     as isize)
                                      * Distance >> 12 as libc::c_int) as
                                     libc::c_uint) as int32;
    //	CameraLoc->y = Camera->y + 	((iV_SIN(Camera->pitch) * Distance)>>FP12_SHIFT);
    Yinc =
        *aSinTable.as_mut_ptr().offset((((*Camera).pitch as uint16 as
                                             libc::c_int >> 4 as libc::c_int)
                                            + 1024 as libc::c_int) as isize) *
            Distance >> 12 as libc::c_int;
    //	DBPRINTF(("camera x=%d y=%d z=%d\n",Camera->x,Camera->y,Camera->z));
//	DBPRINTF(("pitch=%ld yaw=%ld Yinc=%ld Dist=%ld\n",Camera->pitch,Camera->yaw,Yinc,Distance));
    (*CameraLoc).x =
        (*Camera).x.wrapping_add((*aSinTable.as_mut_ptr().offset((-((*Camera).yaw
                                                                        as
                                                                        libc::c_int)
                                                                      as
                                                                      uint16
                                                                      as
                                                                      libc::c_int
                                                                      >>
                                                                      4 as
                                                                          libc::c_int)
                                                                     as isize)
                                      * -Yinc >> 12 as libc::c_int) as
                                     libc::c_uint) as int32;
    (*CameraLoc).z =
        (*Camera).z.wrapping_add((*aSinTable.as_mut_ptr().offset(((-((*Camera).yaw
                                                                         as
                                                                         libc::c_int)
                                                                       as
                                                                       uint16
                                                                       as
                                                                       libc::c_int
                                                                       >>
                                                                       4 as
                                                                           libc::c_int)
                                                                      +
                                                                      1024 as
                                                                          libc::c_int)
                                                                     as isize)
                                      * Yinc >> 12 as libc::c_int) as
                                     libc::c_uint) as int32;
    //	DBPRINTF(("out) camera x=%d y=%d z=%d\n",CameraLoc->x,CameraLoc->y,CameraLoc->z));
//	DBPRINTF(("t=%d t1=%d t2=%d t3=%d z=%d\n",t,t1,t2,t3,CameraLoc->z));
}
#[no_mangle]
pub unsafe extern "C" fn DrawBSPIMD(mut IMDdef: *mut iIMDShape,
                                    mut pPos: *mut iVector) {
    BSPScrPos = pPos;
    BSPimd = IMDdef;
    TraverseTreeAndRender((*IMDdef).BSPNode);
}
// #ifdef BSPIMD
