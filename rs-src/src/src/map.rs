use ::libc;
extern "C" {
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn saveFile(pFileName: *const libc::c_char,
                pFileData: *const libc::c_char, fileSize: UDWORD) -> BOOL;
    /* Remove all droids */
    #[no_mangle]
    fn freeAllDroids();
    /* Remove all structures */
    #[no_mangle]
    fn freeAllStructs();
    /* Remove all features */
    #[no_mangle]
    fn freeAllFeatures();
    /* Set the map view point to the world coordinates x,y */
    #[no_mangle]
    fn intSetMapPos(x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn proj_FreeAllProjectiles();
    #[no_mangle]
    fn environGetValue(x: UDWORD, y: UDWORD) -> UDWORD;
    //this function is called whenever the map changes - load new level or return from an offWorld map
    #[no_mangle]
    fn environReset();
    // the RLE map zones for each tile
    #[no_mangle]
    static mut apRLEZones: *mut *mut UBYTE;
    // the number of map zones
    #[no_mangle]
    static mut gwNumZones: SDWORD;
    // The zone equivalence tables
    #[no_mangle]
    static mut aNumEquiv: *mut UBYTE;
    #[no_mangle]
    static mut apEquivZones: *mut *mut UBYTE;
    // Add a gateway to the system
    #[no_mangle]
    fn gwNewGateway(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> BOOL;
    // add the land/water link gateways
    #[no_mangle]
    fn gwGenerateLinkGates() -> BOOL;
    // link all the gateways together
    #[no_mangle]
    fn gwLinkGateways() -> BOOL;
    // Create a new empty zone map but don't allocate the actual zones yet.
    #[no_mangle]
    fn gwNewZoneMap() -> BOOL;
    // Create a new empty zone map line in the zone map.
    #[no_mangle]
    fn gwNewZoneLine(Line: UDWORD, Size: UDWORD) -> *mut UBYTE;
    // Create a NULL zone map for when there is no zone info loaded
    #[no_mangle]
    fn gwCreateNULLZoneMap() -> BOOL;
    // Get the gateway list.
    #[no_mangle]
    fn gwGetGateways() -> *mut GATEWAY;
    // Get number of zone lines.
    #[no_mangle]
    fn gwNumZoneLines() -> UDWORD;
    // Get size of a zone line in bytes.
    #[no_mangle]
    fn gwZoneLineSize(Line: UDWORD) -> UDWORD;
    // create an empty equivalence table
    #[no_mangle]
    fn gwNewEquivTable(numZones: SDWORD) -> BOOL;
    // set the zone equivalence for a zone
    #[no_mangle]
    fn gwSetZoneEquiv(zone: SDWORD, numEquiv: SDWORD, pEquiv: *mut UBYTE)
     -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
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
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
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
pub type MAPTILE = _maptile;
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
/* Structure definitions for loading and saving map data */
pub type MAP_SAVEHEADER = _map_save_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _map_save_header {
    pub aFileType: [STRING; 4],
    pub version: UDWORD,
    pub width: UDWORD,
    pub height: UDWORD,
}
pub type ZONEMAP_SAVEHEADER = _zonemap_save_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zonemap_save_header {
    pub version: UWORD,
    pub numZones: UWORD,
    pub numEquivZones: UWORD,
    pub pad: UWORD,
}
pub type ZONEMAP_SAVEHEADER_V1 = _zonemap_save_header_v1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _zonemap_save_header_v1 {
    pub version: UWORD,
    pub numZones: UWORD,
}
pub type GATEWAY_SAVE = _gateway_save;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_save {
    pub x0: UBYTE,
    pub y0: UBYTE,
    pub x1: UBYTE,
    pub y1: UBYTE,
}
pub type GATEWAY_SAVEHEADER = _gateway_save_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_save_header {
    pub version: UDWORD,
    pub numGateways: UDWORD,
}
pub type MAP_SAVETILEV2 = _map_save_tilev2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _map_save_tilev2 {
    pub texture: UWORD,
    pub height: UBYTE,
}
pub type GATEWAY = _gateway;
// Previous point in the route
/*
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
// flags for the gateway links
// the link is part of the current route - to the previous gateway
// the link is part of the current route - to the next gateway
// the route between the two zones is blocked
// the flags that get reset by the router
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway {
    pub x1: UBYTE,
    pub y1: UBYTE,
    pub x2: UBYTE,
    pub y2: UBYTE,
    pub zone1: UBYTE,
    pub zone2: UBYTE,
    pub psNext: *mut _gateway,
    pub psLinks: *mut GATEWAY_LINK,
    pub zone1Links: UBYTE,
    pub zone2Links: UBYTE,
    pub flags: UBYTE,
    pub dist: SWORD,
    pub est: SWORD,
    pub psOpen: *mut _gateway,
    pub psRoute: *mut _gateway,
}
pub type GATEWAY_LINK = _gateway_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
// the gateway is to be ignored by the router
pub const GWR_WATERLINK: _gw_node_flags = 128;
pub type MAP_SAVETILE = _map_save_tile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _map_save_tile {
    pub texture: UWORD,
    pub height: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vis_save_header {
    pub aFileType: [STRING; 4],
    pub version: UDWORD,
}
pub type VIS_SAVEHEADER = _vis_save_header;
// types of node for the gateway router
pub type _gw_node_flags = libc::c_uint;
// the gateway is a land/water link
// the gateway is totally blocked
pub const GWR_IGNORE: _gw_node_flags = 64;
// the gateway is part of the final route
pub const GWR_BLOCKED: _gw_node_flags = 32;
// the route goes from zone2 to zone1
pub const GWR_INROUTE: _gw_node_flags = 16;
// the route goes from zone1 to zone2
pub const GWR_ZONE2: _gw_node_flags = 8;
pub const GWR_ZONE1: _gw_node_flags = 4;
pub const GWR_CLOSED: _gw_node_flags = 2;
pub const GWR_OPEN: _gw_node_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _map_save_tilev1 {
    pub texture: UDWORD,
    pub type_0: UBYTE,
    pub height: UBYTE,
}
pub type MAP_SAVETILEV1 = _map_save_tilev1;
/* Floating point type for the aaLine */
// AAFLOAT's are interchangable with the FRACT type used in fractions.h
//
// - I couldn't bring myself to rewrite John's execellent fixed/floating point code
//
pub type AAFLOAT = libc::c_float;
#[inline]
unsafe extern "C" fn endian_udword(mut udword: *mut UDWORD) { }
#[inline]
unsafe extern "C" fn endian_uword(mut uword: *mut UWORD) { }
#[inline]
unsafe extern "C" fn map_TileHeight(mut x: UDWORD, mut y: UDWORD) -> SWORD {
    if x >= mapWidth || y >= mapHeight { return 0 as libc::c_int as SWORD }
    return ((*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                                    isize)).height as libc::c_int *
                2 as libc::c_int) as SWORD;
}
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
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
/*
 * Map.c
 *
 * Utility functions for the map data structure.
 *
 */
/* map line printf's */
//#define DEBUG_GROUP1
// defines the inline functions in this module
// chnaged line	- alex
// end of chnage - alex
#[no_mangle]
pub static mut bDoneWater: BOOL = 0 as libc::c_int;
//void	mapFreeTilesAndStrips( void );
//scroll min and max values
#[no_mangle]
pub static mut scrollMinX: SDWORD = 0;
#[no_mangle]
pub static mut scrollMaxX: SDWORD = 0;
#[no_mangle]
pub static mut scrollMinY: SDWORD = 0;
#[no_mangle]
pub static mut scrollMaxY: SDWORD = 0;
/* Floating point constants for aaLine */
/* Windows fpu version */
//\0.71			// Maximum perpendicular distance from line center
// Floating point divide
// Floating point multiply
/* Access the root table */
// Maximun expected return value from get height
/* Number of entries in the sqrt(1/(1+x*x)) table for aaLine */
/* aaLine direction bits and tables */
/* set when abs(dy) > abs(dx) */
/* set whey dy < 0 */
/* Defines to access the map for aaLine */
//#define PIXINC(dx,dy)		((dy << mapShift) + dx) //width no longer a power of 2
/* The size and contents of the map */
#[no_mangle]
pub static mut mapWidth: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut mapHeight: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut psMapTiles: *mut MAPTILE = 0 as *const MAPTILE as *mut MAPTILE;
/* The shift on the y coord when calculating into the map */
//UDWORD	mapShift = 0;
/* The map tiles generated by map calc line */
#[no_mangle]
pub static mut aMapLinePoints: *mut TILE_COORD =
    0 as *const TILE_COORD as *mut TILE_COORD;
//static UDWORD		maxLinePoints = 0;
/* The sqrt(1/(1+x*x)) table for aaLine */
#[no_mangle]
pub static mut aAARootTbl: *mut AAFLOAT = 0 as *const AAFLOAT as *mut AAFLOAT;
/* pixel increment values for aaLine                        */
/*   -- assume PIXINC(dx,dy) is a macro such that:          */
/*   PIXADDR(x0,y0) + PIXINC(dx,dy) = PIXADDR(x0+dx,y0+dy)  */
//static int adj_pixinc[4];
//static int diag_pixinc[4];
//static int orth_pixinc[4];
/* Initialise the sqrt(1/(1+x*x)) lookup table */
//static void mapRootTblInit(void);
/* Initialise the pixel offset tabels for aaLine */
//static void mapPixTblInit(void);
/* Look up table that returns the terrain type of a given tile texture */
#[no_mangle]
pub static mut terrainTypes: [UBYTE; 255] = [0; 255];
/* pointer to a load map function - depends on version */
#[no_mangle]
pub static mut pLoadMapFunc:
           Option<unsafe extern "C" fn(_: *mut libc::c_char, _: UDWORD)
                      -> BOOL> =
    None;
#[no_mangle]
pub unsafe extern "C" fn GetCurrentMap() -> *mut MAPTILE 
 // returns a pointer to the current loaded map data
 {
    return psMapTiles;
}
/* Create a new map of a specified size */
#[no_mangle]
pub unsafe extern "C" fn mapNew(mut width: UDWORD, mut height: UDWORD)
 -> BOOL {
    //	UDWORD	numPoints;
    let mut i: UDWORD = 0; //, tmp, bitCount, widthShift;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    /* See if a map has already been allocated */
    if !psMapTiles.is_null() {
        /* Clear all the objects off the map and free up the map memory */
        freeAllDroids();
        freeAllStructs();
        freeAllFeatures();
        proj_FreeAllProjectiles();
        //		FREE(psMapTiles);
//		mapFreeTilesAndStrips();
        memFreeRelease(aMapLinePoints as *mut libc::c_void);
        aMapLinePoints = 0 as *mut TILE_COORD;
        psMapTiles = 0 as *mut MAPTILE;
        aMapLinePoints = 0 as *mut TILE_COORD
    }
    //	if (width > MAP_MAXWIDTH || height > MAP_MAXHEIGHT)
    if width.wrapping_mul(height) >
           (256 as libc::c_int * 256 as libc::c_int) as libc::c_uint {
        debug(LOG_ERROR,
              b"mapNew: map too large : %d %d\n\x00" as *const u8 as
                  *const libc::c_char, width, height);
        abort();
    }
    //DON'T BOTHER ANYMORE
	/* Check the width is a power of 2 */
	/*bitCount = 0;
	tmp = width;
	widthShift = 0;
	for(i=0; i<32; i++)
	{
		if (tmp & 1)
		{
			bitCount ++;
			widthShift = i;
		}
		tmp = tmp >> 1;
	}
	if (bitCount != 1)
	{
		DBERROR(("mapNew: width must be a power of two"));
		return FALSE;
	}
	*/
    psMapTiles =
        memMallocRelease((::std::mem::size_of::<MAPTILE>() as
                              libc::c_ulong).wrapping_mul(width).wrapping_mul(height))
            as *mut MAPTILE;
    if psMapTiles.is_null() {
        debug(LOG_ERROR,
              b"mapNew: Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psMapTiles as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<MAPTILE>() as
                libc::c_ulong).wrapping_mul(width).wrapping_mul(height));
    mapWidth = width;
    mapHeight = height;
    i = 0 as libc::c_int as UDWORD;
    while i < 255 as libc::c_int as libc::c_uint {
        terrainTypes[i as usize] = TER_SANDYBRUSH as libc::c_int as UBYTE;
        i = i.wrapping_add(1)
    }
    /* Calculate the shift to use on y when indexing into the map array - USES mapWidth NOW*/
//	mapShift = widthShift;
    /* Allocate a buffer for the LOS routines points */
    /*	numPoints = iSQRT(mapWidth * mapWidth +  mapHeight * mapHeight) + 1;



	aMapLinePoints = (TILE_COORD *)MALLOC(sizeof(TILE_COORD) * numPoints);
	if (!aMapLinePoints)
	{
		DBERROR(("Out of memory"));
		return FALSE;
	}
	maxLinePoints = numPoints;
*/
	/* Initialise the root table for aaLine */
//	mapRootTblInit();
//	mapPixTblInit();
    intSetMapPos(mapWidth.wrapping_mul(128 as libc::c_int as
                                           libc::c_uint).wrapping_div(2 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint),
                 mapHeight.wrapping_mul(128 as libc::c_int as
                                            libc::c_uint).wrapping_div(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint));
    /* Initialise the map terrain type */
    psTile = psMapTiles;
    /*
	for(i=mapWidth * mapHeight; i>0; i--)
	{
		psTile->type = TER_GRASS;
		psTile++;
	}
	*/
    //environInit();
    environReset();
    /*set up the scroll mins and maxs - set values to valid ones for a new map*/
    scrollMinY = 0 as libc::c_int;
    scrollMinX = scrollMinY;
    scrollMaxX = mapWidth as SDWORD;
    scrollMaxY = mapHeight as SDWORD;
    return 1 as libc::c_int;
}
/* load the map data - for version 1 */
#[no_mangle]
pub unsafe extern "C" fn mapLoadV1(mut pFileData: *mut libc::c_char,
                                   mut fileSize: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTileData: *mut MAP_SAVETILEV1 = 0 as *mut MAP_SAVETILEV1;
    /* Load in the map data */
    psTileData =
        pFileData.offset(16 as libc::c_int as isize) as *mut MAP_SAVETILEV1;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        let mut DataTexture: UDWORD = 0;
        /* MAP_SAVETILEV1 */
        endian_udword(&mut (*psTileData).texture);
        DataTexture = (*psTileData).texture;
        // get the texture number
        (*psMapTiles.offset(i as isize)).texture =
            (DataTexture & 0xffff as libc::c_int as libc::c_uint) as UWORD;
        // get the flip bits
        let ref mut fresh0 = (*psMapTiles.offset(i as isize)).texture;
        *fresh0 =
            (*fresh0 as libc::c_int |
                 ((DataTexture & 0xff000000 as libc::c_uint) >>
                      16 as libc::c_int) as UWORD as libc::c_int) as UWORD;
        //		psMapTiles[i].type = psTileData->type;
        (*psMapTiles.offset(i as isize)).height = (*psTileData).height;
        //		psMapTiles[i].onFire = 0;
		// Changed line - alex
//		psMapTiles[i].rippleIndex = (UBYTE) (i%RIP_SIZE);
		//end of change - alex
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            //			psMapTiles[i].tileVisible[j]=FALSE;
            (*psMapTiles.offset(i as isize)).tileVisBits =
                ((*psMapTiles.offset(i as isize)).tileVisBits as libc::c_int &
                     !(((1 as libc::c_int) << j) as UBYTE as libc::c_int)) as
                    UBYTE;
            j = j.wrapping_add(1)
        }
        psTileData =
            (psTileData as *mut UBYTE).offset(6 as libc::c_int as isize) as
                *mut MAP_SAVETILEV1;
        i = i.wrapping_add(1)
    }
    if (psTileData as *mut libc::c_char).wrapping_offset_from(pFileData) as
           libc::c_int as libc::c_uint > fileSize {
        debug(LOG_ERROR,
              b"mapLoad: unexpected end of file\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    if gwCreateNULLZoneMap() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* load the map data - for version 1 */
#[no_mangle]
pub unsafe extern "C" fn mapLoadV2(mut pFileData: *mut libc::c_char,
                                   mut fileSize: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTileData: *mut MAP_SAVETILEV2 = 0 as *mut MAP_SAVETILEV2;
    /* Load in the map data */
    psTileData =
        pFileData.offset(16 as libc::c_int as isize) as *mut MAP_SAVETILEV2;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        /* MAP_SAVETILEV2 */
        endian_uword(&mut (*psTileData).texture);
        (*psMapTiles.offset(i as isize)).texture = (*psTileData).texture;
        //		psMapTiles[i].type = psTileData->type;
        (*psMapTiles.offset(i as isize)).height = (*psTileData).height;
        //		psMapTiles[i].onFire = 0;
		// Changed line - alex
//		psMapTiles[i].rippleIndex = (UBYTE) (i%RIP_SIZE);
		//end of change - alex
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            //			psMapTiles[i].tileVisible[j]=FALSE;
            (*psMapTiles.offset(i as isize)).tileVisBits =
                ((*psMapTiles.offset(i as isize)).tileVisBits as libc::c_int &
                     !(((1 as libc::c_int) << j) as UBYTE as libc::c_int)) as
                    UBYTE;
            j = j.wrapping_add(1)
        }
        psTileData =
            (psTileData as *mut UBYTE).offset(3 as libc::c_int as isize) as
                *mut MAP_SAVETILEV2;
        i = i.wrapping_add(1)
    }
    if (psTileData as *mut libc::c_char).wrapping_offset_from(pFileData) as
           libc::c_int as libc::c_uint > fileSize {
        debug(LOG_ERROR,
              b"mapLoad: unexpected end of file\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    if gwCreateNULLZoneMap() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* load the map data - for version 3 */
#[no_mangle]
pub unsafe extern "C" fn mapLoadV3(mut pFileData: *mut libc::c_char,
                                   mut fileSize: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTileData: *mut MAP_SAVETILEV2 = 0 as *mut MAP_SAVETILEV2;
    let mut psGateHeader: *mut GATEWAY_SAVEHEADER =
        0 as *mut GATEWAY_SAVEHEADER;
    let mut psGate: *mut GATEWAY_SAVE = 0 as *mut GATEWAY_SAVE;
    let mut psZoneHeader: *mut ZONEMAP_SAVEHEADER =
        0 as *mut ZONEMAP_SAVEHEADER;
    let mut ZoneSize: UWORD = 0;
    let mut pZone: *mut UBYTE = 0 as *mut UBYTE;
    let mut pDestZone: *mut UBYTE = 0 as *mut UBYTE;
    /* Load in the map data */
    psTileData =
        pFileData.offset(16 as libc::c_int as isize) as *mut MAP_SAVETILEV2;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        /* MAP_SAVETILEV2 */
        endian_uword(&mut (*psTileData).texture);
        (*psMapTiles.offset(i as isize)).texture = (*psTileData).texture;
        //		psMapTiles[i].type = psTileData->type;
        (*psMapTiles.offset(i as isize)).height = (*psTileData).height;
        //		psMapTiles[i].onFire = 0;
		// Changed line - alex
//		psMapTiles[i].rippleIndex = (UBYTE) (i%RIP_SIZE);
		//end of change - alex
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            //			psMapTiles[i].tileVisible[j]=FALSE;
            (*psMapTiles.offset(i as isize)).tileVisBits =
                ((*psMapTiles.offset(i as isize)).tileVisBits as libc::c_int &
                     !(((1 as libc::c_int) << j) as UBYTE as libc::c_int)) as
                    UBYTE;
            j = j.wrapping_add(1)
        }
        psTileData =
            (psTileData as *mut UBYTE).offset(3 as libc::c_int as isize) as
                *mut MAP_SAVETILEV2;
        i = i.wrapping_add(1)
    }
    psGateHeader = psTileData as *mut GATEWAY_SAVEHEADER;
    psGate =
        psGateHeader.offset(1 as libc::c_int as isize) as *mut GATEWAY_SAVE;
    /* GATEWAY_SAVEHEADER */
    endian_udword(&mut (*psGateHeader).version);
    endian_udword(&mut (*psGateHeader).numGateways);
    if (*psGateHeader).version == 1 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Invalid gateway version\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psGateHeader).version == 1 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              459 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"mapLoadV3\x00")).as_ptr(),
              b"psGateHeader->version == 1\x00" as *const u8 as
                  *const libc::c_char);
    };
    i = 0 as libc::c_int as UDWORD;
    while i < (*psGateHeader).numGateways {
        if gwNewGateway((*psGate).x0 as SDWORD, (*psGate).y0 as SDWORD,
                        (*psGate).x1 as SDWORD, (*psGate).y1 as SDWORD) == 0 {
            debug(LOG_ERROR,
                  b"mapLoadV3: Unable to add gateway\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        psGate = psGate.offset(1);
        i = i.wrapping_add(1)
    }
    //#ifndef PSX
//	if (!gwProcessMap())
//	{
//		return FALSE;
//	}
//
//	if ((psGateways != NULL) &&
//		!gwGenerateLinkGates())
//	{
//		return FALSE;
//	}
//#else
    psZoneHeader = psGate as *mut ZONEMAP_SAVEHEADER;
    /* ZONEMAP_SAVEHEADER */
    endian_uword(&mut (*psZoneHeader).version);
    endian_uword(&mut (*psZoneHeader).numZones);
    endian_uword(&mut (*psZoneHeader).numEquivZones);
    endian_uword(&mut (*psZoneHeader).pad);
    if (*psZoneHeader).version as libc::c_int == 1 as libc::c_int ||
           (*psZoneHeader).version as libc::c_int == 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Invalid zone map version\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psZoneHeader).version as libc::c_int == 1 as libc::c_int ||
           (*psZoneHeader).version as libc::c_int == 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              491 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"mapLoadV3\x00")).as_ptr(),
              b"(psZoneHeader->version == 1) || (psZoneHeader->version == 2)\x00"
                  as *const u8 as *const libc::c_char);
    };
    if gwNewZoneMap() == 0 { return 0 as libc::c_int }
    // This is a bit nasty but should work fine.
    if (*psZoneHeader).version as libc::c_int == 1 as libc::c_int {
        // version 1 so add the size of a version 1 header.
        pZone =
            (psZoneHeader as
                 *mut UBYTE).offset(::std::mem::size_of::<ZONEMAP_SAVEHEADER_V1>()
                                        as libc::c_ulong as isize)
    } else {
        // version 2 so add the size of a version 2 header.
        pZone =
            (psZoneHeader as
                 *mut UBYTE).offset(::std::mem::size_of::<ZONEMAP_SAVEHEADER>()
                                        as libc::c_ulong as isize)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < (*psZoneHeader).numZones as libc::c_uint {
        ZoneSize = *(pZone as *mut UWORD);
        endian_uword(&mut ZoneSize);
        pDestZone = gwNewZoneLine(i, ZoneSize as UDWORD);
        if pDestZone.is_null() { return 0 as libc::c_int }
        j = 0 as libc::c_int as UDWORD;
        while j < ZoneSize as libc::c_uint {
            *pDestZone.offset(j as isize) =
                *pZone.offset((2 as libc::c_int as
                                   libc::c_uint).wrapping_add(j) as isize);
            j = j.wrapping_add(1)
        }
        pZone =
            pZone.offset((ZoneSize as libc::c_int + 2 as libc::c_int) as
                             isize);
        i = i.wrapping_add(1)
    }
    // Version 2 has the zone equivelancy lists tacked on the end.
    if (*psZoneHeader).version as libc::c_int == 2 as libc::c_int {
        if (*psZoneHeader).numEquivZones as libc::c_int > 0 as libc::c_int {
            // Load in the zone equivelance lists.
            if gwNewEquivTable((*psZoneHeader).numEquivZones as SDWORD) == 0 {
                debug(LOG_ERROR,
                      b"gwNewEquivTable failed\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            i = 0 as libc::c_int as UDWORD;
            while i < (*psZoneHeader).numEquivZones as libc::c_uint {
                if *pZone as libc::c_int != 0 as libc::c_int {
                    if gwSetZoneEquiv(i as SDWORD, *pZone as SDWORD,
                                      pZone.offset(1 as libc::c_int as isize))
                           == 0 {
                        debug(LOG_ERROR,
                              b"gwSetZoneEquiv failed\x00" as *const u8 as
                                  *const libc::c_char);
                        abort();
                    }
                }
                pZone =
                    pZone.offset((*pZone as
                                      UDWORD).wrapping_add(1 as libc::c_int as
                                                               libc::c_uint)
                                     as isize);
                i = i.wrapping_add(1)
            }
        }
    }
    if (pZone as *mut libc::c_char).wrapping_offset_from(pFileData) as
           libc::c_int as libc::c_uint > fileSize {
        debug(LOG_ERROR,
              b"mapLoadV3: unexpected end of file\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //#endif
    //	loadingScreenCallback();
    if !apEquivZones.is_null() && gwGenerateLinkGates() == 0 {
        return 0 as libc::c_int
    }
    //	loadingScreenCallback();
    //add new map initialise
    if gwLinkGateways() == 0 { return 0 as libc::c_int }
    //	loadingScreenCallback();
    return 1 as libc::c_int;
}
/* Initialise the map structure */
#[no_mangle]
pub unsafe extern "C" fn mapLoad(mut pFileData: *mut libc::c_char,
                                 mut fileSize: UDWORD) -> BOOL {
    //	UDWORD				i;
//	UDWORD	tmp, bitCount, widthShift;
    let mut width: UDWORD = 0;
    let mut height: UDWORD = 0;
    let mut psHeader: *mut MAP_SAVEHEADER = 0 as *mut MAP_SAVEHEADER;
    let mut mapAlloc: BOOL = 0;
    //	UDWORD				i;
    /* Check the file type */
    psHeader = pFileData as *mut MAP_SAVEHEADER;
    if (*psHeader).aFileType[0 as libc::c_int as usize] as libc::c_int !=
           'm' as i32 ||
           (*psHeader).aFileType[1 as libc::c_int as usize] as libc::c_int !=
               'a' as i32 ||
           (*psHeader).aFileType[2 as libc::c_int as usize] as libc::c_int !=
               'p' as i32 ||
           (*psHeader).aFileType[3 as libc::c_int as usize] as libc::c_int !=
               ' ' as i32 {
        debug(LOG_ERROR,
              b"mapLoad: Incorrect file type\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /* MAP_SAVEHEADER */
    endian_udword(&mut (*psHeader).version);
    endian_udword(&mut (*psHeader).width);
    endian_udword(&mut (*psHeader).height);
    /* Check the file version - deal with version 1 files */
	/* Check the file version */
    if (*psHeader).version < 7 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"MapLoad: unsupported save format version %d\x00" as *const u8
                  as *const libc::c_char, (*psHeader).version);
        abort();
    } else {
        if (*psHeader).version <= 9 as libc::c_int as libc::c_uint {
            pLoadMapFunc =
                Some(mapLoadV2 as
                         unsafe extern "C" fn(_: *mut libc::c_char, _: UDWORD)
                             -> BOOL)
        } else if (*psHeader).version <= 33 as libc::c_int as libc::c_uint {
            pLoadMapFunc =
                Some(mapLoadV3 as
                         unsafe extern "C" fn(_: *mut libc::c_char, _: UDWORD)
                             -> BOOL)
            // Includes gateway data for routing.
        } else {
            debug(LOG_ERROR,
                  b"MapLoad: undefined save format version %d\x00" as
                      *const u8 as *const libc::c_char, (*psHeader).version);
            abort();
        }
    }
    /* Get the width and height */
    width = (*psHeader).width;
    height = (*psHeader).height;
    //	if (width > MAP_MAXWIDTH || height > MAP_MAXHEIGHT)
    if width.wrapping_mul(height) >
           (256 as libc::c_int * 256 as libc::c_int) as libc::c_uint {
        debug(LOG_ERROR,
              b"mapLoad: map too large : %d %d\n\x00" as *const u8 as
                  *const libc::c_char, width, height);
        abort();
    }
    //DON'T BOTHER ANYMORE
	/* Check the width is a power of 2 */
	/*bitCount = 0;
	tmp = width;
	widthShift = 0;
	for(i=0; i<32; i++)
	{
		if (tmp & 1)
		{
			bitCount ++;
			widthShift = i;
		}
		tmp = tmp >> 1;
	}
	if (bitCount != 1)
	{
		DBERROR(("mapLoad: width must be a power of two"));
		return FALSE;
	}
	*/
    /* See if this is the first time a map has been loaded */
    mapAlloc = 1 as libc::c_int;
    if !psMapTiles.is_null() {
        if mapWidth == width && mapHeight == height {
            mapAlloc = 0 as libc::c_int
        } else {
            /* Clear all the objects off the map and free up the map memory */
            freeAllDroids();
            freeAllStructs();
            freeAllFeatures();
            proj_FreeAllProjectiles();
            //			FREE(psMapTiles);
//			mapFreeTilesAndStrips();
            memFreeRelease(aMapLinePoints as *mut libc::c_void);
            aMapLinePoints = 0 as *mut TILE_COORD;
            psMapTiles = 0 as *mut MAPTILE;
            aMapLinePoints = 0 as *mut TILE_COORD
        }
    }
    /* Allocate the memory for the map */
    if mapAlloc != 0 {
        psMapTiles =
            memMallocRelease((::std::mem::size_of::<MAPTILE>() as
                                  libc::c_ulong).wrapping_mul(width).wrapping_mul(height))
                as *mut MAPTILE;
        if psMapTiles.is_null() {
            debug(LOG_ERROR,
                  b"mapLoad: Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        memset(psMapTiles as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<MAPTILE>() as
                    libc::c_ulong).wrapping_mul(width).wrapping_mul(height));
        mapWidth = width;
        mapHeight = height
        /*		a terrain type is loaded when necessary - so don't reset
		for (i=0; i<MAX_TILE_TEXTURES; i++)
		{
			terrainTypes[i] = TER_SANDYBRUSH;
		}*/
		/* Calculate the shift to use on y when indexing into the map array */
//		mapShift = widthShift;
        /* Allocate a buffer for the LOS routines points */
        /*		numPoints = iSQRT(mapWidth * mapWidth +  mapHeight * mapHeight) + 1;



		aMapLinePoints = (TILE_COORD *)MALLOC(sizeof(TILE_COORD) * numPoints);
		if (!aMapLinePoints)
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}
		maxLinePoints = numPoints;
*/
		/* Initialise the root table for aaLine */
//		mapRootTblInit();
//removed for NEW_SAVE //V11 Save
//		intSetMapPos(mapWidth * TILE_UNITS/2, mapHeight * TILE_UNITS/2);
    }
    //load in the map data itself
    pLoadMapFunc.expect("non-null function pointer")(pFileData, fileSize);
    //	mapPixTblInit();
    //environInit();
    environReset();
    /* set up the scroll mins and maxs - set values to valid ones for any new map */
    scrollMinY = 0 as libc::c_int;
    scrollMinX = scrollMinY;
    scrollMaxX = mapWidth as SDWORD;
    scrollMaxY = mapHeight as SDWORD;
    return 1 as libc::c_int;
}
/* Save the map data */
#[no_mangle]
pub unsafe extern "C" fn mapSave(mut ppFileData: *mut *mut libc::c_char,
                                 mut pFileSize: *mut UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut psHeader: *mut MAP_SAVEHEADER = 0 as *mut MAP_SAVEHEADER;
    let mut psTileData: *mut MAP_SAVETILE = 0 as *mut MAP_SAVETILE;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut psCurrGate: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psGateHeader: *mut GATEWAY_SAVEHEADER =
        0 as *mut GATEWAY_SAVEHEADER;
    let mut psGate: *mut GATEWAY_SAVE = 0 as *mut GATEWAY_SAVE;
    let mut psZoneHeader: *mut ZONEMAP_SAVEHEADER =
        0 as *mut ZONEMAP_SAVEHEADER;
    let mut psZone: *mut UBYTE = 0 as *mut UBYTE;
    let mut psLastZone: *mut UBYTE = 0 as *mut UBYTE;
    let mut numGateways: SDWORD = 0;
    // find the number of non water gateways
    numGateways = 0 as libc::c_int;
    psCurrGate = gwGetGateways();
    while !psCurrGate.is_null() {
        if (*psCurrGate).flags as libc::c_int & GWR_WATERLINK as libc::c_int
               == 0 {
            numGateways += 1 as libc::c_int
        }
        psCurrGate = (*psCurrGate).psNext
    }
    /* Allocate the data buffer */
    *pFileSize =
        (16 as libc::c_int as
             libc::c_uint).wrapping_add(mapWidth.wrapping_mul(mapHeight).wrapping_mul(3
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint));
    // Add on the size of the gateway data.
    *pFileSize =
        (*pFileSize as
             libc::c_uint).wrapping_add((::std::mem::size_of::<GATEWAY_SAVEHEADER>()
                                             as
                                             libc::c_ulong).wrapping_add((::std::mem::size_of::<GATEWAY_SAVE>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(numGateways
                                                                                                              as
                                                                                                              libc::c_uint)))
            as UDWORD as UDWORD;
    // Add on the size of the zone data header.
    *pFileSize =
        (*pFileSize as
             libc::c_uint).wrapping_add(::std::mem::size_of::<ZONEMAP_SAVEHEADER>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    // Add on the size of the zone data.
    i = 0 as libc::c_int as UDWORD;
    while i < gwNumZoneLines() {
        *pFileSize =
            (*pFileSize as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_add(gwZoneLineSize(i)))
                as UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    // Add on the size of the equivalency lists.
    i = 0 as libc::c_int as UDWORD;
    while i < gwNumZones as UDWORD {
        *pFileSize =
            (*pFileSize as
                 libc::c_uint).wrapping_add((1 as libc::c_int +
                                                 *aNumEquiv.offset(i as isize)
                                                     as libc::c_int) as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        i = i.wrapping_add(1)
    }
    *ppFileData = memMallocRelease(*pFileSize) as *mut libc::c_char;
    if (*ppFileData).is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    /* Put the file header on the file */
    psHeader = *ppFileData as *mut MAP_SAVEHEADER;
    (*psHeader).aFileType[0 as libc::c_int as usize] = 'm' as i32 as STRING;
    (*psHeader).aFileType[1 as libc::c_int as usize] = 'a' as i32 as STRING;
    (*psHeader).aFileType[2 as libc::c_int as usize] = 'p' as i32 as STRING;
    (*psHeader).aFileType[3 as libc::c_int as usize] = ' ' as i32 as STRING;
    (*psHeader).version = 33 as libc::c_int as UDWORD;
    (*psHeader).width = mapWidth;
    (*psHeader).height = mapHeight;
    /* MAP_SAVEHEADER */
    endian_udword(&mut (*psHeader).version);
    endian_udword(&mut (*psHeader).width);
    endian_udword(&mut (*psHeader).height);
    /* Put the map data into the buffer */
    psTileData =
        (*ppFileData).offset(16 as libc::c_int as isize) as *mut MAP_SAVETILE;
    psTile = psMapTiles;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        // don't save the noblock flag as it gets set again when the objects are loaded
        (*psTileData).texture =
            ((*psTile).texture as libc::c_int &
                 !(0x200 as libc::c_int) as UWORD as libc::c_int) as UWORD;
        (*psTileData).height = (*psTile).height;
        /* MAP_SAVETILEV2 */
        endian_uword(&mut (*psTileData).texture);
        psTileData =
            (psTileData as *mut UBYTE).offset(3 as libc::c_int as isize) as
                *mut MAP_SAVETILE;
        psTile = psTile.offset(1);
        i = i.wrapping_add(1)
    }
    // Put the gateway header.
    psGateHeader = psTileData as *mut GATEWAY_SAVEHEADER;
    (*psGateHeader).version = 1 as libc::c_int as UDWORD;
    (*psGateHeader).numGateways = numGateways as UDWORD;
    /* GATEWAY_SAVEHEADER */
    endian_udword(&mut (*psGateHeader).version);
    endian_udword(&mut (*psGateHeader).numGateways);
    psGate =
        psGateHeader.offset(1 as libc::c_int as isize) as *mut GATEWAY_SAVE;
    i = 0 as libc::c_int as UDWORD;
    // Put the gateway data.
    psCurrGate = gwGetGateways();
    while !psCurrGate.is_null() {
        if (*psCurrGate).flags as libc::c_int & GWR_WATERLINK as libc::c_int
               == 0 {
            (*psGate).x0 = (*psCurrGate).x1;
            (*psGate).y0 = (*psCurrGate).y1;
            (*psGate).x1 = (*psCurrGate).x2;
            (*psGate).y1 = (*psCurrGate).y2;
            psGate = psGate.offset(1);
            i = i.wrapping_add(1)
        }
        psCurrGate = (*psCurrGate).psNext
    }
    // Put the zone header.
    psZoneHeader = psGate as *mut ZONEMAP_SAVEHEADER;
    (*psZoneHeader).version = 2 as libc::c_int as UWORD;
    (*psZoneHeader).numZones = gwNumZoneLines() as UWORD;
    (*psZoneHeader).numEquivZones = gwNumZones as UWORD;
    /* ZONEMAP_SAVEHEADER */
    endian_uword(&mut (*psZoneHeader).version);
    endian_uword(&mut (*psZoneHeader).numZones);
    endian_uword(&mut (*psZoneHeader).numEquivZones);
    endian_uword(&mut (*psZoneHeader).pad);
    // Put the zone data.
    psZone = psZoneHeader.offset(1 as libc::c_int as isize) as *mut UBYTE;
    i = 0 as libc::c_int as UDWORD;
    while i < gwNumZoneLines() {
        psLastZone = psZone;
        *(psZone as *mut UWORD) = gwZoneLineSize(i) as UWORD;
        endian_uword(psZone as *mut UWORD);
        psZone =
            psZone.offset(::std::mem::size_of::<UWORD>() as libc::c_ulong as
                              isize);
        memcpy(psZone as *mut libc::c_void,
               *apRLEZones.offset(i as isize) as *const libc::c_void,
               gwZoneLineSize(i));
        psZone = psZone.offset(gwZoneLineSize(i) as isize);
        i = i.wrapping_add(1)
    }
    // Put the equivalency lists.
    if gwNumZones > 0 as libc::c_int {
        i = 0 as libc::c_int as UDWORD;
        while i < gwNumZones as UDWORD {
            psLastZone = psZone;
            *psZone = *aNumEquiv.offset(i as isize);
            psZone = psZone.offset(1);
            if *aNumEquiv.offset(i as isize) != 0 {
                memcpy(psZone as *mut libc::c_void,
                       *apEquivZones.offset(i as isize) as
                           *const libc::c_void,
                       *aNumEquiv.offset(i as isize) as libc::c_uint);
                psZone =
                    psZone.offset(*aNumEquiv.offset(i as isize) as libc::c_int
                                      as isize)
            }
            i = i.wrapping_add(1)
        }
    }
    if (psLastZone as UDWORD).wrapping_sub(*ppFileData as UDWORD) < *pFileSize
       {
    } else {
        debug(LOG_ERROR,
              b"Buffer overflow saving map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (psLastZone as UDWORD).wrapping_sub(*ppFileData as UDWORD) < *pFileSize
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              900 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 8],
                                        &[libc::c_char; 8]>(b"mapSave\x00")).as_ptr(),
              b"( ((UDWORD)psLastZone) - ((UDWORD)*ppFileData) ) < *pFileSize\x00"
                  as *const u8 as *const libc::c_char);
    };
    return 1 as libc::c_int;
}
/* Shutdown the map module */
#[no_mangle]
pub unsafe extern "C" fn mapShutdown() -> BOOL {
    if !psMapTiles.is_null() {
        memFreeRelease(psMapTiles as *mut libc::c_void);
        psMapTiles = 0 as *mut MAPTILE
        //		mapFreeTilesAndStrips();
    }
    psMapTiles = 0 as *mut MAPTILE;
    //	mapWidth = mapHeight = mapShift = 0;
    mapHeight = 0 as libc::c_int as UDWORD;
    mapWidth = mapHeight;
    /*	if(aMapLinePoints) {
		FREE(aMapLinePoints);
	}
	aMapLinePoints = NULL;

	if(aAARootTbl) {
		FREE(aAARootTbl);
	}
	aAARootTbl = NULL;
*/
    return 1 as libc::c_int;
}
/* work along a line on the map storing the points in aPoints.
 * The start and end points are in MAPTILE coordinates.
 */
#[no_mangle]
pub unsafe extern "C" fn mapCalcLine(mut startX: UDWORD, mut startY: UDWORD,
                                     mut endX: UDWORD, mut endY: UDWORD,
                                     mut pNumPoints: *mut UDWORD) {
}
/* Initialise the sqrt(1/(1+x*x)) lookup table */
#[no_mangle]
pub unsafe extern "C" fn mapRootTblInit() { }
/* Initialise the pixel offset tabels for aaLine */
#[no_mangle]
pub unsafe extern "C" fn mapPixTblInit() { }
/* Fill in the aa line */
#[no_mangle]
pub unsafe extern "C" fn mapCalcAALine(mut X1: SDWORD, mut Y1: SDWORD,
                                       mut X2: SDWORD, mut Y2: SDWORD,
                                       mut pNumPoints: *mut UDWORD) {
}
/* Return linear interpolated height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y)
#[no_mangle]
pub unsafe extern "C" fn map_Height(mut x: UDWORD, mut y: UDWORD) -> SWORD {
    let mut retVal: SDWORD = 0;
    let mut tileX: UDWORD = 0;
    let mut tileY: UDWORD = 0;
    let mut tileYOffset: UDWORD = 0;
    let mut h0: SDWORD = 0;
    let mut hx: SDWORD = 0;
    let mut hy: SDWORD = 0;
    let mut hxy: SDWORD = 0;
    let mut wTL: SDWORD = 0 as libc::c_int;
    let mut wTR: SDWORD = 0 as libc::c_int;
    let mut wBL: SDWORD = 0 as libc::c_int;
    let mut wBR: SDWORD = 0 as libc::c_int;
    //SDWORD	lowerHeightOffset,upperHeightOffset;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    let mut bWaterTile: BOOL = 0 as libc::c_int;
    /*	ASSERT( x < (mapWidth << TILE_SHIFT),
		"mapHeight: x coordinate bigger than map width" );
	ASSERT( y < (mapHeight<< TILE_SHIFT),
		"mapHeight: y coordinate bigger than map height" );
*/
    x =
        if x > 0x7fffffff as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else { x }; //negative SDWORD passed as UDWORD
    x =
        if x >= mapWidth << 7 as libc::c_int {
            (mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)) <<
                7 as libc::c_int
        } else { x }; //negative SDWORD passed as UDWORD
    y =
        if y > 0x7fffffff as libc::c_int as libc::c_uint {
            0 as libc::c_int as libc::c_uint
        } else { y };
    y =
        if y >= mapHeight << 7 as libc::c_int {
            (mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint)) <<
                7 as libc::c_int
        } else { y };
    /* Tile comp */
    tileX = x >> 7 as libc::c_int;
    tileY = y >> 7 as libc::c_int;
    /* Inter tile comp */
    ox =
        (x & (128 as libc::c_int - 1 as libc::c_int) as libc::c_uint) as
            SDWORD;
    oy =
        (y & (128 as libc::c_int - 1 as libc::c_int) as libc::c_uint) as
            SDWORD;
    if terrainTypes[((*mapTile(tileX, tileY)).texture as libc::c_int &
                         0x1ff as libc::c_int) as usize] as libc::c_int ==
           TER_WATER as libc::c_int {
        bWaterTile = 1 as libc::c_int;
        wTL =
            environGetValue(tileX,
                            tileY).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as SDWORD;
        wTR =
            environGetValue(tileX.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint),
                            tileY).wrapping_div(2 as libc::c_int as
                                                    libc::c_uint) as SDWORD;
        wBL =
            environGetValue(tileX,
                            tileY.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint)).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                as SDWORD;
        wBR =
            environGetValue(tileX.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint),
                            tileY.wrapping_add(1 as libc::c_int as
                                                   libc::c_uint)).wrapping_div(2
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint)
                as SDWORD
        /*
		lowerHeightOffset = waves[(y%(MAX_RIPPLES-1))];
		upperHeightOffset = waves[((y%(MAX_RIPPLES-1))+1)];
		oy = (SDWORD)y - (SDWORD)(tileY << TILE_SHIFT);
		oy = TILE_UNITS - oy;
		dy = ((lowerHeightOffset - upperHeightOffset) * oy )/ TILE_UNITS;

		return((SEA_LEVEL + (dy*ELEVATION_SCALE)));
		*/
    }
    tileYOffset = tileY.wrapping_mul(mapWidth);
    //	ox = (SDWORD)x - (SDWORD)(tileX << TILE_SHIFT);
//	oy = (SDWORD)y - (SDWORD)(tileY << TILE_SHIFT);
    if ox < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: x offset too big\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ox < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              1429 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
              b"ox < TILE_UNITS\x00" as *const u8 as *const libc::c_char);
    };
    if oy < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: y offset too big\x00" as *const u8 as
                  *const libc::c_char);
    };
    if oy < 128 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              1430 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
              b"oy < TILE_UNITS\x00" as *const u8 as *const libc::c_char);
    };
    if ox >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: x offset too small\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ox >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              1431 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
              b"ox >= 0\x00" as *const u8 as *const libc::c_char);
    };
    if oy >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: y offset too small\x00" as *const u8 as
                  *const libc::c_char);
    };
    if oy >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"map.c\x00" as *const u8 as *const libc::c_char,
              1432 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
              b"oy >= 0\x00" as *const u8 as *const libc::c_char);
    };
    //different code for 4 different triangle cases
    if (*psMapTiles.offset(tileX.wrapping_add(tileYOffset) as isize)).texture
           as libc::c_int & 0x800 as libc::c_int != 0 {
        if ox + oy > 128 as libc::c_int {
            //tile split top right to bottom left object if in bottom right half
            ox = 128 as libc::c_int - ox;
            oy = 128 as libc::c_int - oy;
            hy =
                (*psMapTiles.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                        as isize)).height as SDWORD;
            hx =
                (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint).wrapping_add(tileYOffset)
                                        as isize)).height as SDWORD;
            hxy =
                (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                        as isize)).height as SDWORD;
            if bWaterTile != 0 { hy += wBL; hx += wTR; hxy += wBR }
            dx = (hy - hxy) * ox / 128 as libc::c_int;
            dy = (hx - hxy) * oy / 128 as libc::c_int;
            retVal = (hxy + dx + dy) * 2 as libc::c_int;
            if retVal < 256 as libc::c_int * 2 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Map height\'s gone weird!!!\x00" as *const u8 as
                          *const libc::c_char);
            };
            if retVal < 256 as libc::c_int * 2 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"map.c\x00" as *const u8 as *const libc::c_char,
                      1455 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
                      b"retVal<MAX_HEIGHT\x00" as *const u8 as
                          *const libc::c_char);
            };
            return retVal as SWORD
        } else {
            //tile split top right to bottom left object if in top left half
            h0 =
                (*psMapTiles.offset(tileX.wrapping_add(tileYOffset) as
                                        isize)).height as SDWORD;
            hy =
                (*psMapTiles.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                        as isize)).height as SDWORD;
            hx =
                (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                           libc::c_uint).wrapping_add(tileYOffset)
                                        as isize)).height as SDWORD;
            if bWaterTile != 0 { h0 += wTL; hy += wBL; hx += wTR }
            dx = (hx - h0) * ox / 128 as libc::c_int;
            dy = (hy - h0) * oy / 128 as libc::c_int;
            retVal = (h0 + dx + dy) * 2 as libc::c_int;
            if retVal < 256 as libc::c_int * 2 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Map height\'s gone weird!!!\x00" as *const u8 as
                          *const libc::c_char);
            };
            if retVal < 256 as libc::c_int * 2 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"map.c\x00" as *const u8 as *const libc::c_char,
                      1474 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
                      b"retVal<MAX_HEIGHT\x00" as *const u8 as
                          *const libc::c_char);
            };
            return retVal as SWORD
        }
    } else if ox > oy {
        //tile split topleft to bottom right object if in top right half
        h0 =
            (*psMapTiles.offset(tileX.wrapping_add(tileYOffset) as
                                    isize)).height as SDWORD;
        hx =
            (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                       libc::c_uint).wrapping_add(tileYOffset)
                                    as isize)).height as SDWORD;
        hxy =
            (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                       libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                    as isize)).height as SDWORD;
        if bWaterTile != 0 { h0 += wTL; hx += wTR; hxy += wBR }
        dx = (hx - h0) * ox / 128 as libc::c_int;
        dy = (hxy - hx) * oy / 128 as libc::c_int;
        retVal = (h0 + dx + dy) * 2 as libc::c_int;
        if retVal < 256 as libc::c_int * 2 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Map height\'s gone weird!!!\x00" as *const u8 as
                      *const libc::c_char);
        };
        if retVal < 256 as libc::c_int * 2 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"map.c\x00" as *const u8 as *const libc::c_char,
                  1495 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
                  b"retVal<MAX_HEIGHT\x00" as *const u8 as
                      *const libc::c_char);
        };
        return retVal as SWORD
    } else {
        //tile split topleft to bottom right object if in bottom left half
        h0 =
            (*psMapTiles.offset(tileX.wrapping_add(tileYOffset) as
                                    isize)).height as SDWORD;
        hy =
            (*psMapTiles.offset(tileX.wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                    as isize)).height as SDWORD;
        hxy =
            (*psMapTiles.offset(tileX.wrapping_add(1 as libc::c_int as
                                                       libc::c_uint).wrapping_add(tileYOffset).wrapping_add(mapWidth)
                                    as isize)).height as SDWORD;
        if bWaterTile != 0 { h0 += wTL; hy += wBL; hxy += wBR }
        dx = (hxy - hy) * ox / 128 as libc::c_int;
        dy = (hy - h0) * oy / 128 as libc::c_int;
        retVal = (h0 + dx + dy) * 2 as libc::c_int;
        if retVal < 256 as libc::c_int * 2 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Map height\'s gone weird!!!\x00" as *const u8 as
                      *const libc::c_char);
        };
        if retVal < 256 as libc::c_int * 2 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"map.c\x00" as *const u8 as *const libc::c_char,
                  1514 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"map_Height\x00")).as_ptr(),
                  b"retVal<MAX_HEIGHT\x00" as *const u8 as
                      *const libc::c_char);
        };
        return retVal as SWORD
    };
}
/* returns TRUE if object is above ground */
#[no_mangle]
pub unsafe extern "C" fn mapObjIsAboveGround(mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut iZ: SDWORD = 0;
    let mut tileX: SDWORD = (*psObj).x as libc::c_int >> 7 as libc::c_int;
    let mut tileY: SDWORD = (*psObj).y as libc::c_int >> 7 as libc::c_int;
    let mut tileYOffset1: SDWORD =
        (tileY as libc::c_uint).wrapping_mul(mapWidth) as SDWORD;
    let mut tileYOffset2: SDWORD =
        ((tileY + 1 as libc::c_int) as libc::c_uint).wrapping_mul(mapWidth) as
            SDWORD;
    let mut h1: SDWORD =
        (*psMapTiles.offset((tileYOffset1 + tileX) as isize)).height as
            SDWORD;
    let mut h2: SDWORD =
        (*psMapTiles.offset((tileYOffset1 + tileX + 1 as libc::c_int) as
                                isize)).height as SDWORD;
    let mut h3: SDWORD =
        (*psMapTiles.offset((tileYOffset2 + tileX) as isize)).height as
            SDWORD;
    let mut h4: SDWORD =
        (*psMapTiles.offset((tileYOffset2 + tileX + 1 as libc::c_int) as
                                isize)).height as SDWORD;
    /* trivial test above */
    if (*psObj).z as libc::c_int > h1 && (*psObj).z as libc::c_int > h2 &&
           (*psObj).z as libc::c_int > h3 && (*psObj).z as libc::c_int > h4 {
        return 1 as libc::c_int
    }
    /* trivial test below */
    if (*psObj).z as libc::c_int <= h1 && (*psObj).z as libc::c_int <= h2 &&
           (*psObj).z as libc::c_int <= h3 && (*psObj).z as libc::c_int <= h4
       {
        return 0 as libc::c_int
    }
    /* exhaustive test */
    iZ = map_Height((*psObj).x as UDWORD, (*psObj).y as UDWORD) as SDWORD;
    if (*psObj).z as libc::c_int > iZ {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* returns the max and min height of a tile by looking at the four corners
   in tile coords */
#[no_mangle]
pub unsafe extern "C" fn getTileMaxMin(mut x: UDWORD, mut y: UDWORD,
                                       mut pMax: *mut UDWORD,
                                       mut pMin: *mut UDWORD) {
    let mut height: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    *pMin = (255 as libc::c_int * 2 as libc::c_int) as UDWORD;
    *pMax = 0 as libc::c_int as UDWORD;
    j = 0 as libc::c_int as UDWORD;
    while j < 2 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as UDWORD;
        while i < 2 as libc::c_int as libc::c_uint {
            height =
                map_TileHeight(x.wrapping_add(i), y.wrapping_add(j)) as
                    UDWORD;
            if *pMin > height { *pMin = height }
            if *pMax < height { *pMax = height }
            i = i.wrapping_add(1)
        }
        j = j.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetWidthOfMap() -> UDWORD { return mapWidth; }
#[no_mangle]
pub unsafe extern "C" fn GetHeightOfMap() -> UDWORD { return mapHeight; }
// -----------------------------------------------------------------------------------
/* This will save out the visibility data */
#[no_mangle]
pub unsafe extern "C" fn writeVisibilityData(mut pFileName: *mut STRING)
 -> BOOL {
    let mut pFileData: *mut libc::c_char =
        0 as *mut libc::c_char; // Pointer to the necessary allocated memory
    let mut pVisData: *mut libc::c_char =
        0 as *mut libc::c_char; // Pointer to the start of the map data
    let mut fileSize: UDWORD =
        0; // How many bytes we need - depends on compression
    let mut psHeader: *mut VIS_SAVEHEADER =
        0 as *mut VIS_SAVEHEADER; // Pointer to the header part of the file
    let mut mapEntries: UDWORD = 0; // Effectively, how many tiles are there?
    let mut i: UDWORD = 0;
    let mut status: BOOL = 1 as libc::c_int;
    /* How many tiles do we write out data from? */
    mapEntries = mapWidth.wrapping_mul(mapHeight);
    /* Calculate memory required */
    fileSize =
        (::std::mem::size_of::<_vis_save_header>() as
             libc::c_ulong).wrapping_add(mapEntries.wrapping_mul(::std::mem::size_of::<UBYTE>()
                                                                     as
                                                                     libc::c_ulong));
    /* Try and allocate it - freed up in same function */
    pFileData = memMallocRelease(fileSize) as *mut libc::c_char;
    /* Did we get it? */
    if pFileData.is_null() {
        /* Nope, so do one */
        debug(LOG_ERROR,
              b"Saving visibility data : Cannot get the memory! (%d)\x00" as
                  *const u8 as *const libc::c_char, fileSize);
        abort();
    }
    /* We got the memory, so put the file header on the file */
    psHeader = pFileData as *mut VIS_SAVEHEADER;
    (*psHeader).aFileType[0 as libc::c_int as usize] = 'v' as i32 as STRING;
    (*psHeader).aFileType[1 as libc::c_int as usize] = 'i' as i32 as STRING;
    (*psHeader).aFileType[2 as libc::c_int as usize] = 's' as i32 as STRING;
    (*psHeader).aFileType[3 as libc::c_int as usize] = 'd' as i32 as STRING;
    /* Wirte out the version number - unlikely to change for visibility data */
    (*psHeader).version = 33 as libc::c_int as UDWORD;
    /* VIS_SAVEHEADER */
    endian_udword(&mut (*psHeader).version);
    /* Skip past the header to the raw data area */
    pVisData =
        pFileData.offset(::std::mem::size_of::<_vis_save_header>() as
                             libc::c_ulong as isize);
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        *pVisData.offset(i as isize) =
            (*psMapTiles.offset(i as isize)).tileVisBits as libc::c_char;
        i = i.wrapping_add(1)
    }
    /* Have a bash at opening the file to write */
    status = saveFile(pFileName, pFileData, fileSize);
    /* And free up the memory we used */
    if !pFileData.is_null() {
        memFreeRelease(pFileData as *mut libc::c_void);
        pFileData = 0 as *mut libc::c_char
    }
    /* Everything is just fine! */
    return status;
}
/*
 * Map.h
 *
 * Definitions for the map structure
 *
 */
// Visibility bits - can also be accessed as a byte (as a whole).
/* The different types of terrain as far as the game is concerned */
/* change these if you change above - maybe wrap up in enumerate? */
/* Flags for whether texture tiles are flipped in X and Y or rotated */
// This bit describes the direction the tile is split into 2 triangles (same as triangleFlip)
// set when the tile has the structure cursor over it
// NASTY - this should be in tileInfoBits but there isn't any room left
// units can drive on this even if there is a structure or feature on it
//#define	BITS_TILE_HIGHLIGHT 0x8
// show small structures - tank traps / bunkers
// bit set temporarily by find path to mark a blocking tile
// bit set to show a gateway on the tile
// bit set to show a tall structure which camera needs to avoid.
/*#ifndef PSX	// Extra tile info bits.... WIN32 only
#define	EXTRA_BITS_SENSOR	0x1
#define	EXTRA_BITS_2		0x2
#define	EXTRA_BITS_3		0x4
#define	EXTRA_BITS_4		0x8
#define	EXTRA_BITS_5		0x10
#define	EXTRA_BITS_6		0x20
#define	EXTRA_BITS_7		0x40
#define	EXTRA_BITS_8		0x80
#endif*/
//#define TILE_HIGHLIGHT(x)		(x->tileInfoBits & BITS_TILE_HIGHLIGHT)
/*
#ifndef PSX		// I've even set them up for you...:-)
#define TILE_IN_SENSORRANGE(x)	(x->tileExtraBits & EXTRA_BITS_SENSOR)
#define TILE_EXTRA_BIT2_SET(x)	(x->tileExtraBits & EXTRA_BITS_2)
#define TILE_EXTRA_BIT3_SET(x)	(x->tileExtraBits & EXTRA_BITS_3)
#define TILE_EXTRA_BIT4_SET(x)	(x->tileExtraBits & EXTRA_BITS_4)
#define TILE_EXTRA_BIT5_SET(x)	(x->tileExtraBits & EXTRA_BITS_5)
#define TILE_EXTRA_BIT6_SET(x)	(x->tileExtraBits & EXTRA_BITS_6)
#define TILE_EXTRA_BIT7_SET(x)	(x->tileExtraBits & EXTRA_BITS_7)
#define TILE_EXTRA_BIT8_SET(x)	(x->tileExtraBits & EXTRA_BITS_8)
#endif
*/
/*
#ifndef PSX	// again, done for you again!
#define SET_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_SENSOR))
#define CLEAR_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_SENSOR)))
#define SET_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_2))
#define CLEAR_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_2)))
#define SET_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_3))
#define CLEAR_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_3)))
#define SET_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_4))
#define CLEAR_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_4)))
#define SET_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_5))
#define CLEAR_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_5)))
#define SET_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_6))
#define CLEAR_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_6)))
#define SET_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_7))
#define CLEAR_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_7)))
#define SET_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_8))
#define CLEAR_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_8)))
#endif
*/
// Multiplier for the tile height
/* Allows us to do if(TRI_FLIPPED(psTile)) */
/* Flips the triangle partition on a tile pointer */
/* Can player number p see tile t? */
/* Set a tile to be visible for a player */
/*#ifndef PSX
#define SET_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits | (1<<p))
#define CLEAR_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits & (~(1<<p))) // check logic
// Is there a door here for the player?
#define TEST_TILE_DOOR(p,t) ( (t->tileDoorBits) & (1<<p) )
#endif*/
/* Arbitrary maximum number of terrain textures - used in look up table for terrain type */
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
/* The maximum map size */
/* The size and contents of the map */
/* The shift on the y coord when calculating into the map */
/* The number of units accross a tile */
/* The shift on a coordinate to get the tile coordinate */
/* The mask to get internal tile coords from a full coordinate */
/* Shutdown the map module */
/* Create a new map of a specified size */
/* Load the map data */
/* Save the map data */
/* Load map texture info */
/* Save the current map texture info */
/* A post process for the water tiles in map to ensure height integrity */
/* Return a pointer to the tile structure at x,y */
//return psMapTiles + x + (y << mapShift); //width no longer a power of 2
/* Return height of tile at x,y */
//static inline SDWORD map_TileHeight(UDWORD x, UDWORD y)
/*sets the tile height */
//psMapTiles[x + (y << mapShift)].height = height;//width no longer a power of 2
/*increases the tile height by one */
/*static inline void incTileHeight(UDWORD x, UDWORD y)
{
	psMapTiles[x + (y << mapShift)].height++;
}*/
/*decreases the tile height by one */
/*static inline void decTileHeight(UDWORD x, UDWORD y)
{
	psMapTiles[x + (y << mapShift)].height--;
}*/
/* Return whether a tile coordinate is on the map */
/* Return whether a world coordinate is on the map */
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
/* The map tiles generated by map calc line */
/* work along a line on the map storing the points in aMapLinePoints.
 * pNumPoints is set to the number of points generated.
 * The start and end points are in TILE coordinates.
 */
/* Same as mapCalcLine, but does a wider line in the map */
/* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
/* returns TRUE if object is above ground */
/* returns the max and min height of a tile by looking at the four corners
   in tile coords */
// returns a pointer to the current loaded map data
// -----------------------------------------------------------------------------------
/* This will read in the visibility data */
#[no_mangle]
pub unsafe extern "C" fn readVisibilityData(mut pFileData: *mut libc::c_char,
                                            mut fileSize: UDWORD) -> BOOL {
    let mut expectedFileSize: UDWORD = 0;
    let mut mapEntries: UDWORD = 0;
    let mut psHeader: *mut VIS_SAVEHEADER = 0 as *mut VIS_SAVEHEADER;
    let mut i: UDWORD = 0;
    let mut pVisData: *mut UBYTE = 0 as *mut UBYTE;
    /* See if we've been given the right file type? */
    psHeader = pFileData as *mut VIS_SAVEHEADER;
    if (*psHeader).aFileType[0 as libc::c_int as usize] as libc::c_int !=
           'v' as i32 ||
           (*psHeader).aFileType[1 as libc::c_int as usize] as libc::c_int !=
               'i' as i32 ||
           (*psHeader).aFileType[2 as libc::c_int as usize] as libc::c_int !=
               's' as i32 ||
           (*psHeader).aFileType[3 as libc::c_int as usize] as libc::c_int !=
               'd' as i32 {
        debug(LOG_ERROR,
              b"Read visibility data: Weird file type found? Has header letters - %c %c %c %c\x00"
                  as *const u8 as *const libc::c_char,
              (*psHeader).aFileType[0 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[1 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[2 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[3 as libc::c_int as usize] as
                  libc::c_int);
        return 0 as libc::c_int
    }
    /* VIS_SAVEHEADER */
    endian_udword(&mut (*psHeader).version);
    /* How much data are we expecting? */
    mapEntries = mapWidth.wrapping_mul(mapHeight);
    expectedFileSize =
        (::std::mem::size_of::<_vis_save_header>() as
             libc::c_ulong).wrapping_add(mapEntries.wrapping_mul(::std::mem::size_of::<UBYTE>()
                                                                     as
                                                                     libc::c_ulong));
    /* Is that what we've been given? */
    if fileSize != expectedFileSize {
        /* No, so bomb out */
        debug(LOG_ERROR,
              b"Read visibility data : Weird file size for %d by %d sized map?\x00"
                  as *const u8 as *const libc::c_char, mapWidth, mapHeight);
        abort();
    }
    /* Skip past the header gubbins - can check version number here too */
    pVisData =
        (pFileData as
             *mut UBYTE).offset(::std::mem::size_of::<_vis_save_header>() as
                                    libc::c_ulong as isize);
    /* For every tile... */
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth.wrapping_mul(mapHeight) {
        /* Get the visibility data */
        (*psMapTiles.offset(i as isize)).tileVisBits =
            *pVisData.offset(i as isize);
        i = i.wrapping_add(1)
    }
    /* Hopefully everything's just fine by now */
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------
