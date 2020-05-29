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
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
}
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
pub struct iView {
    pub p: iVector,
    pub r: iVector,
}
/* **************************************************************************/
/*
 * base.h
 *
 * Definitions for the base object type.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/* **************************************************************************/
/* The type of object */
/* ID number of the object */
/* Object's location */
/* Object's direction +ve rotation about y axis*/
/* Object's pitch +ve nose up*/
/* Object's roll +ve left up, right down */
/* screen coordinate details */
/* Which player the object belongs to */
/* Which group selection is the droid currently in? */
/* Whether the object is selected */
/* might want this elsewhere */
/* Which cluster the object is a member of */
/* Whether object is visible to specific player */
/* When an object was destroyed, if 0 still alive */
/*BOOL			    (*damage)(pointerType	*psObject, */
/*UDWORD		damage, */
/* UDWORD		weaponClass); - Damage function */
/*UDWORD				emissionInterval;	how frequently does it puff out damage smoke?*/
/* when did it last puff out smoke? */
/* Data for fire damage */
/* TRUE if the object is in a fire */
/* When the object entered the fire */
/* How much damage has been done since the object */
/* entered the fire */
/* Pointer to next object in list */
/* Pointer to previois object in list */
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_object {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _base_object,
    pub sDisplay: SCREEN_DISP_DATA,
    pub player: UBYTE,
    pub group: UBYTE,
    pub selected: UBYTE,
    pub cluster: UBYTE,
    pub visible: [UBYTE; 8],
    pub died: UDWORD,
    pub lastEmission: UDWORD,
    pub inFire: BOOL,
    pub burnStart: UDWORD,
    pub burnDamage: UDWORD,
}
pub type SCREEN_DISP_DATA = _screen_disp_data;
/*
 * DisplayDef.h
 *
 * Definitions of the display structures
 *
 */
// ffs am
//#define BOUNDARY_X		(DISP_WIDTH/20)	   // proportional to resolution - Alex M
//#define	BOUNDARY_Y		(DISP_WIDTH/16)
//#define	BOUNDARY_X		(24)
//#define	BOUNDARY_Y		(24)
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
// warning.... this is not used on the playstation version !
// the polygon number for the next in the BSP list ... or BSPPOLYID_TERMINATE for no more
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
pub type PSBSPTREENODE = *mut BSPTREENODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type BSPPOLYID = uint16;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
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
pub type VERTEXID = libc::c_int;
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
pub type BASE_OBJECT = _base_object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _maptile {
    pub tileInfoBits: UBYTE,
    pub tileVisBits: UBYTE,
    pub height: UBYTE,
    pub illumination: UBYTE,
    pub texture: UWORD,
    pub bMaxed: UBYTE,
    pub level: UBYTE,
    pub inRange: UBYTE,
}
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
/*#ifndef PSX
	UBYTE			tileExtraBits;	// We've got more than you... We've got more than you..;-)
#endif*/
// COMPRESSED - bit per player
/*#ifndef PSX
	UBYTE			tileDoorBits;   // same thing - bit per player
#endif*/
// The height at the top left of the tile
// How bright is this tile?
// Which graphics texture is on this tile
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/* Return a pointer to the tile structure at x,y */
#[inline]
unsafe extern "C" fn mapTile(mut x: UDWORD, mut y: UDWORD) -> *mut MAPTILE {
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"mapTile: x coordinate bigger than map width\x00" as *const u8
                  as *const libc::c_char);
    };
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              285 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"x < mapWidth\x00" as *const u8 as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"mapTile: y coordinate bigger than map height\x00" as *const u8
                  as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"./map.h\x00" as *const u8 as *const libc::c_char,
              287 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapTile\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    //return psMapTiles + x + (y << mapShift); //width no longer a power of 2
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
#[no_mangle]
pub static mut avConsidered: UDWORD = 0;
#[no_mangle]
pub static mut avCalculated: UDWORD = 0;
#[no_mangle]
pub static mut avIgnored: UDWORD = 0;
// ------------------------------------------------------------------------------------
#[no_mangle]
pub static mut bRevealActive: BOOL = 0 as libc::c_int;
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn avSetStatus(mut var: BOOL) { bRevealActive = var; }
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn avInformOfChange(mut x: SDWORD, mut y: SDWORD) {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut lowerX: SDWORD = 0;
    let mut upperX: SDWORD = 0;
    let mut lowerY: SDWORD = 0;
    let mut upperY: SDWORD = 0;
    psTile = mapTile(x as UDWORD, y as UDWORD);
    if (*psTile).level as libc::c_int == 0xff as libc::c_int {
        lowerX = player.p.x / 128 as libc::c_int;
        upperX =
            (lowerX as libc::c_uint).wrapping_add(visibleXTiles) as SDWORD;
        lowerY = player.p.z / 128 as libc::c_int;
        upperY =
            (lowerY as libc::c_uint).wrapping_add(visibleYTiles) as SDWORD;
        if lowerX < 0 as libc::c_int { lowerX = 0 as libc::c_int }
        if lowerY < 0 as libc::c_int { lowerY = 0 as libc::c_int }
        if x > lowerX && x < upperX && y > lowerY && y < upperY {
            /* tile is on grid - so initiate fade up */
            (*psTile).level = 0 as libc::c_int as UBYTE
        } else {
            /* tile is off the gird, so force to maximum and finish */
            (*psTile).level = (*psTile).illumination;
            (*psTile).bMaxed = 1 as libc::c_int as UBYTE
        }
    } else {
        /* Already know about this one - so exit */
        return
    };
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn avUpdateTiles() {
    let mut startX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    /* Clear stats */
    avConsidered = 0 as libc::c_int as UDWORD;
    avCalculated = 0 as libc::c_int as UDWORD;
    avIgnored = 0 as libc::c_int as UDWORD;
    /* Only process the ones on the grid. Find top left */
    if player.p.x >= 0 as libc::c_int {
        startX = player.p.x / 128 as libc::c_int
    } else { startX = 0 as libc::c_int }
    if player.p.z >= 0 as libc::c_int {
        startY = player.p.z / 128 as libc::c_int
    } else { startY = 0 as libc::c_int }
    /* Find bottom right */
    endX = (startX as libc::c_uint).wrapping_add(visibleXTiles) as SDWORD;
    endY = (startY as libc::c_uint).wrapping_add(visibleYTiles) as SDWORD;
    /* Clip, as we may be off map */
    if startX < 0 as libc::c_int { startX = 0 as libc::c_int }
    if startY < 0 as libc::c_int { startY = 0 as libc::c_int }
    if endX >
           mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD {
        endX =
            mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    if endY >
           mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
       {
        endY =
            mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    /* Go through the grid */
    i = startY as UDWORD;
    while i < endY as UDWORD {
        j = startX as UDWORD;
        while j < endX as UDWORD {
            processAVTile(j, i);
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    avConsidered = visibleXTiles.wrapping_mul(visibleYTiles);
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn processAVTile(mut x: UDWORD, mut y: UDWORD) {
    let mut time: FRACT = 0.;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut newLevel: UDWORD = 0;
    psTile = mapTile(x, y);
    if (*psTile).level as libc::c_int == 0xff as libc::c_int ||
           (*psTile).bMaxed as libc::c_int != 0 {
        return
    }
    time = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
    newLevel =
        ((*psTile).level as libc::c_int as libc::c_float +
             time *
                 (1000 as libc::c_int / 10 as libc::c_int) as libc::c_float)
            as SDWORD as UDWORD;
    if newLevel >= (*psTile).illumination as libc::c_uint {
        (*psTile).level = (*psTile).illumination;
        (*psTile).bMaxed = 1 as libc::c_int as UBYTE
    } else { (*psTile).level = newLevel as UBYTE };
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn avGetObjLightLevel(mut psObj: *mut BASE_OBJECT,
                                            mut origLevel: UDWORD) -> UDWORD {
    let mut div: FRACT = 0.;
    let mut lowest: UDWORD = 0;
    let mut newLevel: UDWORD = 0;
    div =
        (*psObj).visible[selectedPlayer as usize] as FRACT /
            255 as libc::c_int as libc::c_float;
    lowest = origLevel.wrapping_div(8 as libc::c_int as libc::c_uint);
    newLevel = (div * origLevel as libc::c_float) as UDWORD;
    if newLevel < lowest { newLevel = lowest }
    return newLevel;
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn avGetStats(mut considered: *mut UDWORD,
                                    mut ignored: *mut UDWORD,
                                    mut calculated: *mut UDWORD) {
    *considered = avConsidered;
    *ignored = avIgnored;
    *calculated = avCalculated;
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getRevealStatus() -> BOOL { return bRevealActive; }
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn setRevealStatus(mut val: BOOL) {
    bRevealActive = val;
}
// ------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn preProcessVisibility() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //STRUCTURE	*psStruct;
//FEATURE		*psFeature;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth {
        j = 0 as libc::c_int as UDWORD;
        while j < mapHeight {
            psTile = mapTile(i, j);
            if (*psTile).tileVisBits as libc::c_int &
                   (1 as libc::c_int) << selectedPlayer != 0 {
                (*psTile).bMaxed = 1 as libc::c_int as UBYTE;
                (*psTile).level = (*psTile).illumination
                //can't have this cos when load up a save game where a structure has been built by the
                //enemy in an area that has been seen before it flags the structure as visible!
				/*if(TILE_HAS_STRUCTURE(psTile))
				{
				 	psStruct = getTileStructure(i,j);
                    if (psStruct)
                    {
					    psStruct->visible[selectedPlayer] = UBYTE_MAX;
                    }
                    else
                    {
                        ASSERT( FALSE, "preProcessVisibility: should be a structure at %d, %d", i, j );
                    }
				}*/
				/*
				if(TILE_HAS_FEATURE(psTile))
				{
					psFeature = getTileFeature(i,j);
					psFeature->visible[selectedPlayer] = UBYTE_MAX;
				}
				*/
            } else {
                (*psTile).level = 0xff as libc::c_int as UBYTE;
                (*psTile).bMaxed = 0 as libc::c_int as UBYTE
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// ------------------------------------------------------------------------------------
