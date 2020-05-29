use ::libc;
extern "C" {
    pub type _droid_group;
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
    fn abs(_: libc::c_int) -> libc::c_int;
    /* Returns the current frame we're on - used to establish whats on screen */
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    // The next object that should get the router when a lot of units are
// in a MOVEROUTE state
    #[no_mangle]
    static mut psNextRouteDroid: *mut DROID;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
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
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    // returns a pointer to the current loaded map data
    #[no_mangle]
    fn GetHeightOfMap() -> UDWORD;
    #[no_mangle]
    fn GetWidthOfMap() -> UDWORD;
    //scroll min and max values
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    /* cast a ray from x,y (world coords) at angle ray (0-NUM_RAYS) */
    #[no_mangle]
    fn rayCast(x: UDWORD, y: UDWORD, ray: UDWORD, length: UDWORD,
               callback: RAY_CALLBACK);
    // Calculate the angle to cast a ray between two points
    #[no_mangle]
    fn rayPointsToAngle(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD)
     -> UDWORD;
    // Sizes for the node heap
    // counters for a-star
    #[no_mangle]
    static mut astarInner: SDWORD;
    // A* findpath
    #[no_mangle]
    fn fpathAStarRoute(routeMode: SDWORD, psRoutePoints: *mut ASTAR_ROUTE,
                       sx: SDWORD, sy: SDWORD, fx: SDWORD, fy: SDWORD)
     -> SDWORD;
    /*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
    // the list of gateways on the current map
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
    // A* findpath for the gateway system
    #[no_mangle]
    fn gwrAStarRoute(player: SDWORD, terrain: UDWORD, sx: SDWORD, sy: SDWORD,
                     fx: SDWORD, fy: SDWORD, ppsRoute: *mut *mut GATEWAY)
     -> SDWORD;
    // tell the action system of a potential location for walls blocking routing
    #[no_mangle]
    fn actionRouteBlockingPos(psDroid: *mut DROID, x: SDWORD, y: SDWORD)
     -> BOOL;
    // Try and find a formation near to a location
    #[no_mangle]
    fn formationFind(ppsFormation: *mut *mut FORMATION, x: SDWORD, y: SDWORD)
     -> BOOL;
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
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
// only needed when generating the tree
/* **************************************************************************/
// These 1st three entries can NOT NOW be cast into a iVectorf *   (iVectorf on PC are doubles)
// these values form the plane equation ax+by+cz=d
// a point on the plane - in normal non-fract format
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
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
pub type OBJECT_TYPE = _object_type;
pub type _object_type = libc::c_uint;
pub const OBJ_TARGET: _object_type = 4;
pub const OBJ_BULLET: _object_type = 3;
pub const OBJ_FEATURE: _object_type = 2;
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
/*
 * StatsDef.h
 *
 * Structure definitions for the stats system
 *
 */
/* Elements common to all stats structures */
/* Unique ID of the item */
/* pointer to the text id name (i.e. short language-independant name) */
/* Stats common to all stats structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
//COMP_PROGRAM,		//this needs to be removed when save games changes
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
//COMP_POWERPLANT,
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
//COMP_ARMOUR,
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
//ALL components and structures and research topics have a tech level to which they belong
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
/* SIZE used for specifying body size */
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
//only using KINETIC and HEAT for now
pub type _weapon_class = libc::c_uint;
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
pub type WEAPON_CLASS = _weapon_class;
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
pub type _weapon_subclass = libc::c_uint;
pub const NUM_WEAPON_SUBCLASS: _weapon_subclass = 17;
pub const WSC_EMP: _weapon_subclass = 16;
pub const WSC_COMMAND: _weapon_subclass = 15;
pub const WSC_BOMB: _weapon_subclass = 14;
pub const WSC_LAS_SAT: _weapon_subclass = 13;
pub const WSC_SLOWROCKET: _weapon_subclass = 12;
pub const WSC_SLOWMISSILE: _weapon_subclass = 11;
pub const WSC_AAGUN: _weapon_subclass = 10;
pub const WSC_ELECTRONIC: _weapon_subclass = 9;
//WSC_CLOSECOMBAT,
pub const WSC_HOWITZERS: _weapon_subclass = 8;
pub const WSC_FLAME: _weapon_subclass = 7;
pub const WSC_GAUSS: _weapon_subclass = 6;
pub const WSC_ENERGY: _weapon_subclass = 5;
pub const WSC_ROCKET: _weapon_subclass = 4;
pub const WSC_MISSILE: _weapon_subclass = 3;
//WSC_ARTILLARY,
pub const WSC_MORTARS: _weapon_subclass = 2;
pub const WSC_CANNON: _weapon_subclass = 1;
pub const WSC_MGUN: _weapon_subclass = 0;
pub type WEAPON_SUBCLASS = _weapon_subclass;
// used to define which projectile model to use for the weapon
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type WEAPON_EFFECT = _weapon_effect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _body_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub size: UBYTE,
    pub weaponSlots: UDWORD,
    pub armourValue: [UDWORD; 2],
    pub powerOutput: UDWORD,
    pub ppIMDList: *mut *mut iIMDShape,
    pub pFlameIMD: *mut iIMDShape,
}
pub type BODY_STATS = _body_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub shortRange: UDWORD,
    pub longRange: UDWORD,
    pub minRange: UDWORD,
    pub shortHit: UDWORD,
    pub longHit: UDWORD,
    pub firePause: UDWORD,
    pub numExplosions: UDWORD,
    pub numRounds: UBYTE,
    pub reloadTime: UDWORD,
    pub damage: UDWORD,
    pub radius: UDWORD,
    pub radiusHit: UDWORD,
    pub radiusDamage: UDWORD,
    pub incenTime: UDWORD,
    pub incenDamage: UDWORD,
    pub incenRadius: UDWORD,
    pub flightSpeed: UDWORD,
    pub indirectHeight: UDWORD,
    pub fireOnMove: FIREONMOVE,
    pub weaponClass: WEAPON_CLASS,
    pub weaponSubClass: WEAPON_SUBCLASS,
    pub movementModel: MOVEMENT_MODEL,
    pub weaponEffect: WEAPON_EFFECT,
    pub recoilValue: UDWORD,
    pub rotate: UBYTE,
    pub maxElevation: UBYTE,
    pub minElevation: SBYTE,
    pub facePlayer: UBYTE,
    pub faceInFlight: UBYTE,
    pub effectSize: UBYTE,
    pub lightWorld: BOOL,
    pub surfaceToAir: UBYTE,
    pub vtolAttackRuns: UBYTE,
    pub directLife: UDWORD,
    pub radiusLife: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
    pub pMuzzleGraphic: *mut iIMDShape,
    pub pInFlightGraphic: *mut iIMDShape,
    pub pTargetHitGraphic: *mut iIMDShape,
    pub pTargetMissGraphic: *mut iIMDShape,
    pub pWaterHitGraphic: *mut iIMDShape,
    pub pTrailGraphic: *mut iIMDShape,
    pub iAudioFireID: SDWORD,
    pub iAudioImpactID: SDWORD,
}
//pointer to which flame graphic to use - for VTOLs only at the moment
//weapon stats associated with this brain - for Command Droids
//defines the left and right sides for propulsion IMDs
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
//works as all of the above together! - new for updates - added 11/06/99 AB
/* Common stats */
// Sensor range
// Sensor power (put against ecm power)
// specifies whether the Sensor is default or for the Turret
// used for combat
//time delay before associated weapon droids 'know' where the
//attack is from
// The turret mount to use
/* Common stats */
// ECM range
// ECM power (put against sensor power)
// specifies whether the ECM is default or for the Turret
// The turret mount to use
/* Adjustment to armour damage for non-penetrating hit */
/* Adjustment to armour damage for penetrating hit */
/* Common stats */
// How much damage is restored to Body Points 
// and armour each Repair Cycle
// whether armour can be repaired or not
// specifies whether the Repair is default or for the Turret
// time delay for repair cycle
// The turret mount to use
//no longer used 7/8/98
/*typedef struct _program_stats
{
	// Common stats - This structure doesn't actually need all the stats
	COMPONENT_STATS;		
	UDWORD		slots;				// How many brain slots the program takes
	UDWORD		order;				// The order activated by the program if any
	UDWORD		special;			// The special ability that the droid can perform
									// with this program
} PROGRAM_STATS;*/
/*these are defined in Access database - if you change them in there,
  then change them here! (and the rest of the code)
  They are made up values for now - defined when Jim does it!*/
/*typedef enum _program_orders
{
	ORDER_STOP,
	ORDER_SCAVANGE,
	ORDER_ATTACK,
	ORDER_GUARD,
	ORDER_AID,
	ORDER_BUILD,
	ORDER_DEMOLISH,
	ORDER_REPAIR,
} PROGRAM_ORDERS;*/
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type _propulsion_type = libc::c_uint;
pub const NUM_PROP_TYPES: _propulsion_type = 9;
pub const JUMP: _propulsion_type = 8;
pub const HALF_TRACKED: _propulsion_type = 7;
pub const PROPELLOR: _propulsion_type = 6;
pub const LIFT: _propulsion_type = 5;
pub const SKI: _propulsion_type = 4;
pub const HOVER: _propulsion_type = 3;
pub const LEGGED: _propulsion_type = 2;
pub const TRACKED: _propulsion_type = 1;
pub const WHEELED: _propulsion_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub maxSpeed: UDWORD,
    pub propulsionType: UBYTE,
}
pub type PROPULSION_STATS = _propulsion_stats;
pub type _sensor_type = libc::c_uint;
pub const SUPER_SENSOR: _sensor_type = 4;
pub const VTOL_INTERCEPT_SENSOR: _sensor_type = 3;
pub const VTOL_CB_SENSOR: _sensor_type = 2;
pub const INDIRECT_CB_SENSOR: _sensor_type = 1;
pub const STANDARD_SENSOR: _sensor_type = 0;
pub type SENSOR_TYPE = _sensor_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sensor_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub type_0: SENSOR_TYPE,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ecm_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techLevel: TECH_LEVEL,
    pub buildPower: UDWORD,
    pub buildPoints: UDWORD,
    pub weight: UDWORD,
    pub hitPoints: UDWORD,
    pub systemPoints: UDWORD,
    pub body: UDWORD,
    pub design: BOOL,
    pub pIMD: *mut iIMDShape,
    pub range: UDWORD,
    pub power: UDWORD,
    pub location: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
/*Common stats for all Structure Functions*/
/* Unique ID of the item */
/* Text name of the component */
/* The type of Function */
/*Common struct for all functions*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UDWORD,
    pub techLevel: TECH_LEVEL,
    pub strength: STRUCT_STRENGTH,
    pub terrainType: UDWORD,
    pub baseWidth: UDWORD,
    pub baseBreadth: UDWORD,
    pub foundationType: UDWORD,
    pub buildPoints: UDWORD,
    pub height: UDWORD,
    pub armourValue: UDWORD,
    pub bodyPoints: UDWORD,
    pub repairSystem: UDWORD,
    pub powerToBuild: UDWORD,
    pub resistance: UDWORD,
    pub sizeModifier: UDWORD,
    pub pIMD: *mut iIMDShape,
    pub pBaseIMD: *mut iIMDShape,
    pub pECM: *mut _ecm_stats,
    pub pSensor: *mut _sensor_stats,
    pub psWeapStat: *mut _weapon_stats,
    pub numFuncs: UDWORD,
    pub defaultFunc: SDWORD,
    pub asFuncList: *mut *mut _function,
}
/*
 * StructureDef.h
 *
 * Structure definitions for structures
 *
 */
// Used to indicate any kind of building when calling intGotoNextStructureType()
/* Defines for indexing an appropriate IMD object given a buildings purpose. */
//draw as factory 2	
//corner wall - no gun
//control centre for command droids
//the demolish structure type - should only be one stat with this type
//added for updates - AB 8/6/99
//REF_WALLH,     //the following are needed for the demo
//REF_WALLV,		
//REF_CORNER1,
//REF_CORNER2,
//REF_CORNER3,
//REF_CORNER4,
//REF_GATE1,
//REF_GATE2,
//REF_GATE3,
//REF_GATE4,
//REF_TOWER1,
//REF_TOWER2,
//REF_TOWER3,
//REF_TOWER4,
//REF_VANH,
//REF_VANV,
//REF_JEEP,
//REF_TANKERH,
//REF_TANKERV,
//need to keep a count of how many types for IMD loading
//Delivery Points NOT wayPoints
//proximity messages that are data generated
//proximity messages that are in game generated
//SAVE ONLY delivery point for factory currently assigned to commander
/*the type of position obj - FlagPos or ProxDisp*/
/*when the Position was last drawn*/
/*screen coords and radius of Position imd */
/*which player the Position belongs to*/
/*flag to indicate whether the Position 
										is to be highlighted*/
//the world coords of the Position
//UDWORD		frameNumber;					//when the Position was last drawn
	//UDWORD		screenX, screenY, screenR;		//screen coords and radius of Position imd
	//UDWORD		player;							//which player the Position belongs to
	//BOOL		selected;						//flag to indicate whether the 
												//Position is to be highlighted
//indicates whether the first, second etc factory
//indicates whether standard, cyborg or vtol factory
//	UBYTE		factorySub;						//sub value. needed to order production points.
//	UBYTE		primary;
//only allowed one weapon per structure (more memory for Tim) 
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _path_point {
    pub x: UBYTE,
    pub y: UBYTE,
}
pub type PATH_POINT = _path_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _move_control {
    pub Status: UBYTE,
    pub Mask: UBYTE,
    pub Position: UBYTE,
    pub numPoints: UBYTE,
    pub asPath: [PATH_POINT; 100],
    pub DestinationX: SDWORD,
    pub DestinationY: SDWORD,
    pub srcX: SDWORD,
    pub srcY: SDWORD,
    pub targetX: SDWORD,
    pub targetY: SDWORD,
    pub fx: FRACT,
    pub fy: FRACT,
    pub speed: FRACT,
    pub boundX: SWORD,
    pub boundY: SWORD,
    pub dir: SWORD,
    pub bumpDir: SWORD,
    pub bumpTime: UDWORD,
    pub lastBump: UWORD,
    pub pauseTime: UWORD,
    pub bumpX: UWORD,
    pub bumpY: UWORD,
    pub shuffleStart: UDWORD,
    pub psFormation: *mut _formation,
    pub iVertSpeed: SWORD,
    pub iAttackRuns: UWORD,
    pub fz: FRACT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _formation {
    pub refCount: SWORD,
    pub size: SWORD,
    pub rankDist: SWORD,
    pub dir: SWORD,
    pub x: SDWORD,
    pub y: SDWORD,
    pub asLines: [F_LINE; 4],
    pub numLines: SWORD,
    pub maxRank: UBYTE,
    pub free: SBYTE,
    pub asMembers: [F_MEMBER; 20],
    pub iSpeed: UDWORD,
    pub psNext: *mut _formation,
}
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
}
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
pub type BASE_OBJECT = _base_object;
pub type F_LINE = _f_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_line {
    pub xoffset: SWORD,
    pub yoffset: SWORD,
    pub dir: SWORD,
    pub member: SBYTE,
}
pub type MOVE_CONTROL = _move_control;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/*
 * Weapons.h
 *
 * Definitions for the weapons
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
pub type WEAPON = _weapon;
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
/*
 * DroidDef.h
 *
 * Droid structure definitions
 */
/* The number of components in the asParts / asBits arrays */
//(COMP_NUMCOMPONENTS - 2)
/* The maximum number of droid weapons and programs */
//3
//#define DROID_MAXPROGS		3
// This should really be logarithmic
//defines how many times to perform the iteration on looking for a blank location
//used to get a location next to a droid - withinh one tile
/* The different types of droid */
// NOTE, if you add to, or change this list then you'll need
// to update the DroidSelectionWeights lookup table in Display.c
pub type _droid_type = libc::c_uint;
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//cyborg repair droid - new for update 7/6/99
pub const DROID_ANY: _droid_type = 13;
//cyborg repair droid - new for update 28/5/99
pub const DROID_CYBORG_SUPER: _droid_type = 12;
//cyborg constructor droid - new for update 28/5/99
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
// Default droid
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
// Repair droid
pub const DROID_DEFAULT: _droid_type = 9;
// Command droid
pub const DROID_REPAIR: _droid_type = 8;
// guess what this is!
pub const DROID_COMMAND: _droid_type = 7;
// cyborg-type thang
pub const DROID_TRANSPORTER: _droid_type = 6;
// person
pub const DROID_CYBORG: _droid_type = 5;
// Constructor droid
pub const DROID_PERSON: _droid_type = 4;
// ECM droid
pub const DROID_CONSTRUCT: _droid_type = 3;
// Sensor droid
pub const DROID_ECM: _droid_type = 2;
// Weapon droid
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _order_list {
    pub order: SDWORD,
    pub psOrderTarget: *mut libc::c_void,
    pub x: UWORD,
    pub y: UWORD,
    pub x2: UWORD,
    pub y2: UWORD,
}
pub type ORDER_LIST = _order_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _droid,
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
    pub aName: [STRING; 60],
    pub droidType: DROID_TYPE,
    pub asBits: [COMPONENT; 8],
    pub weight: UDWORD,
    pub baseSpeed: UDWORD,
    pub sensorRange: UDWORD,
    pub sensorPower: UDWORD,
    pub ECMMod: UDWORD,
    pub originalBody: UDWORD,
    pub body: UDWORD,
    pub armour: [UDWORD; 2],
    pub numKills: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub NameVersion: UBYTE,
    pub currRayAng: UBYTE,
    pub resistance: SWORD,
    pub asWeaps: [WEAPON; 1],
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
    pub psBaseStruct: *mut _structure,
    pub listSize: SDWORD,
    pub asOrderList: [ORDER_LIST; 10],
    pub order: SDWORD,
    pub orderX: UWORD,
    pub orderY: UWORD,
    pub orderX2: UWORD,
    pub orderY2: UWORD,
    pub lastHitWeapon: UDWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
    pub psTarget: *mut _base_object,
    pub psTarStats: *mut _base_stats,
    pub secondaryOrder: UDWORD,
    pub lastSync: UDWORD,
    pub action: SDWORD,
    pub actionX: UDWORD,
    pub actionY: UDWORD,
    pub psActionTarget: *mut _base_object,
    pub actionStarted: UDWORD,
    pub actionPoints: UDWORD,
    pub powerAccrued: UWORD,
    pub illumination: UBYTE,
    pub updateFlags: UBYTE,
    pub sMove: MOVE_CONTROL,
    pub psCurAnim: *mut ANIM_OBJECT,
    pub iAudioID: SDWORD,
}
//line build requires two sets of coords
//this structure is used whenever an instance of a building is required in game
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _structure,
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
    pub pStructureType: *mut STRUCTURE_STATS,
    pub status: UBYTE,
    pub currentBuildPts: SWORD,
    pub currentPowerAccrued: SWORD,
    pub body: UWORD,
    pub armour: UWORD,
    pub resistance: SWORD,
    pub lastResistance: UDWORD,
    pub sensorRange: UWORD,
    pub sensorPower: UWORD,
    pub turretRotation: UWORD,
    pub turretPitch: UWORD,
    pub timeLastHit: UDWORD,
    pub lastHitWeapon: UDWORD,
    pub radarX: UWORD,
    pub radarY: UWORD,
    pub ecmPower: UWORD,
    pub pFunctionality: *mut FUNCTIONALITY,
    pub targetted: UBYTE,
    pub asWeaps: [WEAPON; 1],
    pub psTarget: *mut BASE_OBJECT,
    pub psCurAnim: *mut ANIM_OBJECT,
}
//this structure is used to hold the permenant stats for each type of building
/* basic stats */
/* the type of structure */
/* technology level of the structure */
/* strength against the weapon effects */
/*The type of terrain the structure has to be 
									  built next to - may be none*/
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
/*The type of foundation for the structure*/
/*The number of build points required to build
									  the structure*/
/*The height above/below the terrain - negative
									  values denote below the terrain*/
/*The armour value for the structure - can be 
									  upgraded */
/*The structure's body points - A structure goes
									  off-line when 50% of its body points are lost*/
/*The repair system points are added to the body
									  points until fully restored . The points are 
									  then added to the Armour Points*/
/*How much power the structure requires to build*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		minimumPower;		The minimum power requirement to start building
								      the structure*/
/*The number used to determine whether a 
									  structure can resist an enemy takeover - 
									  0 = cannot be attacked electrically*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		quantityLimit;		The maximum number that a player can have - 
									  0 = no limit 1 = only 1 allowed etc*/
/*The larger the target, the easier to hit*/
/*The IMD to draw for this structure */
/*The base IMD to draw for this structure */
/*Which ECM is standard for the structure - 
									  if any*/
/*Which Sensor is standard for the structure - 
									  if any*/
//NOT USED ANYMORE - AB 24/01/99
	/*UDWORD		weaponSlots;		/Number of weapons that can be attached to the
									  building/
	UDWORD		numWeaps;			/Number of weapons for default /
	SDWORD		defaultWeap;		/The default weapon/
    
	struct _weapon_stats **asWeapList;		/List of pointers to default weapons/
    */
//can only have one weapon now
/*Number of functions for default*/
/*The default function*/
/*List of pointers to allowable functions - 
									  unalterable*/
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
pub type STRUCTURE_STATS = _structure_stats;
pub type DROID = _droid;
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
/*
 * AStar.h
 *
 */
// the buffer to store a route in
pub type ASTAR_ROUTE = _astar_route;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
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
pub type _gw_link_flags = libc::c_uint;
pub const GWRL_BLOCKED: _gw_link_flags = 4;
pub const GWRL_CHILD: _gw_link_flags = 2;
pub const GWRL_PARENT: _gw_link_flags = 1;
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
pub type GATEWAY = _gateway;
// zone to the left/above the gateway
// zone to the right/below the gateway
// array of links to other zones
// number of links
// Data for the gateway router
// open or closed node
// distance so far and estimate to end
// Previous point in the route
// types of node for the gateway router
pub type _gw_node_flags = libc::c_uint;
// the gateway is a land/water link
// the gateway is to be ignored by the router
pub const GWR_WATERLINK: _gw_node_flags = 128;
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
/*
 * GatewayRoute.h
 *
 * Interface to the gateway routing code
 *
 */
// what type of terrain the unit can move over
pub type _gwr_terrain_types = libc::c_uint;
pub const GWR_TER_ALL: _gwr_terrain_types = 255;
pub const GWR_TER_WATER: _gwr_terrain_types = 2;
pub const GWR_TER_LAND: _gwr_terrain_types = 1;
// return codes for the router
pub type _gwr_return_codes = libc::c_uint;
// the route did not start on a valid zone
// start and end points in the same zone
pub const GWR_NOZONE: _gwr_return_codes = 4;
// couldn't find a route
pub const GWR_SAMEZONE: _gwr_return_codes = 3;
// chose the nearest gateway to the target
pub const GWR_FAILED: _gwr_return_codes = 2;
// found a route
pub const GWR_NEAREST: _gwr_return_codes = 1;
pub const GWR_OK: _gwr_return_codes = 0;
pub type _droid_action = libc::c_uint;
pub const DACTION_FIRESUPPORT_RETREAT: _droid_action = 39;
pub const DACTION_RETURNTOPOS: _droid_action = 38;
pub const DACTION_CLEARREARMPAD: _droid_action = 37;
pub const DACTION_VTOLATTACK: _droid_action = 36;
pub const DACTION_WAITDURINGREARM: _droid_action = 35;
pub const DACTION_MOVETOREARMPOINT: _droid_action = 34;
pub const DACTION_WAITFORREARM: _droid_action = 33;
pub const DACTION_MOVETOREARM: _droid_action = 32;
pub const DACTION_MOVETOCLEAR: _droid_action = 31;
pub const DACTION_MOVETORESTORE: _droid_action = 30;
pub const DACTION_MOVETODROIDREPAIR: _droid_action = 29;
pub const DACTION_WAITDURINGREPAIR: _droid_action = 28;
pub const DACTION_MOVETOREPAIRPOINT: _droid_action = 27;
pub const DACTION_WAITFORREPAIR: _droid_action = 26;
pub const DACTION_MOVETOOBSERVE: _droid_action = 25;
pub const DACTION_ROTATETOATTACK: _droid_action = 24;
pub const DACTION_MOVETOATTACK: _droid_action = 23;
pub const DACTION_FOUNDATION_WANDER: _droid_action = 22;
pub const DACTION_BUILDWANDER: _droid_action = 21;
pub const DACTION_MOVETOREPAIR: _droid_action = 20;
pub const DACTION_MOVETODEMOLISH: _droid_action = 19;
pub const DACTION_MOVETOBUILD: _droid_action = 18;
pub const DACTION_MOVEFIRE: _droid_action = 17;
pub const DACTION_CLEARWRECK: _droid_action = 16;
pub const DACTION_RESTORE: _droid_action = 15;
pub const DACTION_DROIDREPAIR: _droid_action = 14;
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
pub const DACTION_TRANSPORTOUT: _droid_action = 11;
pub const DACTION_DESTRUCT: _droid_action = 10;
pub const DACTION_SULK: _droid_action = 9;
pub const DACTION_FIRESUPPORT: _droid_action = 8;
pub const DACTION_OBSERVE: _droid_action = 7;
pub const DACTION_ATTACK: _droid_action = 6;
pub const DACTION_REPAIR: _droid_action = 5;
pub const DACTION_DEMOLISH: _droid_action = 4;
pub const DACTION_BUILD_FOUNDATION: _droid_action = 3;
pub const DACTION_BUILD: _droid_action = 2;
pub const DACTION_MOVE: _droid_action = 1;
pub const DACTION_NONE: _droid_action = 0;
/*
 * FormationDef.h
 *
 */
// maximum number of lines in a formation
// maximum number of unit members of a formation (cannot be more that 128)
// information about a formation line
// a linked list of the formation members on this line is maintained
// using their index in the asMembers array.  (-1 == 'NULL')
// (cuts down the memory use over proper pointers)
// position relative to center
// orientation of line
// first member in the 'linked list' of members
// information about a formation member
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
// information about a formation
pub type FORMATION = _formation;
pub type _fpath_retval = libc::c_uint;
pub const FPR_RESCHEDULE: _fpath_retval = 3;
pub const FPR_WAIT: _fpath_retval = 2;
pub const FPR_FAILED: _fpath_retval = 1;
pub const FPR_OK: _fpath_retval = 0;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
// return values for routing
pub type FPATH_RETVAL = _fpath_retval;
// found a route
// failed to find a route
// route was too long to calculate this frame
// routing will continue on succeeding frames
// didn't try to route because too much time has been
// spent routing this frame
/*
 * fpath.c
 *
 * Interface to the routing functions
 *
 */
// route success printf's
//#define DEBUG_GROUP0
// way point printf's
//#define DEBUG_GROUP1
// gateway route printf's
//#define DEBUG_GROUP2
#[no_mangle]
pub static mut fpathDoMessage: BOOL = 0;
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
#[inline]
unsafe extern "C" fn tileOnMap(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    return (x >= 0 as libc::c_int && x < mapWidth as SDWORD &&
                y >= 0 as libc::c_int && y < mapHeight as SDWORD) as
               libc::c_int;
}
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
/* global pointer for object being routed - GJ hack -
 * currently only used in fpathLiftBlockingTile */
static mut g_psObjRoute: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// function pointer for the blocking tile check
#[no_mangle]
pub static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL> =
    None;
// if a route is spread over a number of frames this stores the object
// the route is being done for
static mut psPartialRouteObj: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// coords of the partial route
static mut partialSX: SDWORD = 0;
static mut partialSY: SDWORD = 0;
static mut partialTX: SDWORD = 0;
static mut partialTY: SDWORD = 0;
// the last frame on which the partial route was calculatated
static mut lastPartialFrame: SDWORD = 0;
// the maximum amount of routing to do per frame
#[no_mangle]
pub static mut astarMaxRoute: SDWORD = 400 as libc::c_int;
// initialise the findpath module
// initialise the findpath module
#[no_mangle]
pub unsafe extern "C" fn fpathInitialise() -> BOOL {
    fpathBlockingTile =
        Some(fpathGroundBlockingTile as
                 unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
    psPartialRouteObj = 0 as *mut BASE_OBJECT;
    return 1 as libc::c_int;
}
// update the findpath system each frame
// update routine for the findpath system
#[no_mangle]
pub unsafe extern "C" fn fpathUpdate() {
    if !psPartialRouteObj.is_null() &&
           ((*psPartialRouteObj).died != 0 ||
                (*(psPartialRouteObj as *mut DROID)).sMove.Status as
                    libc::c_int != 11 as libc::c_int ||
                (lastPartialFrame + 5 as libc::c_int) <
                    frameGetFrameNumber() as SDWORD) {
        psPartialRouteObj = 0 as *mut BASE_OBJECT
    };
}
// access functions for the loop limit
// access functions for the loop limit
#[no_mangle]
pub unsafe extern "C" fn fpathSetMaxRoute(mut max: SDWORD) {
    astarMaxRoute = max;
}
#[no_mangle]
pub unsafe extern "C" fn fpathGetMaxRoute() -> SDWORD {
    return astarMaxRoute;
}
// Check if the map tile at a location blocks a droid
// Check if the map tile at a location blocks a droid
#[no_mangle]
pub unsafe extern "C" fn fpathGroundBlockingTile(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //	FEATURE	*psFeat;
    //doesn't look like we need this - pickATile wasn't working with it! - AB 8/2/99
	/* check VTOL limits if not routing */
	/*if ( g_psObjRoute == NULL )
	{
		if ( x < VTOL_MAP_EDGE_TILES || y < VTOL_MAP_EDGE_TILES ||
			 x >= (SDWORD)mapWidth-VTOL_MAP_EDGE_TILES || y >= (SDWORD)mapHeight-VTOL_MAP_EDGE_TILES)
		{
			// coords off map - auto blocking tile
			return TRUE;
		}
	}
	else*/
    if x < scrollMinX + 1 as libc::c_int || y < scrollMinY + 1 as libc::c_int
           || x >= scrollMaxX - 1 as libc::c_int ||
           y >= scrollMaxY - 1 as libc::c_int {
        // coords off map - auto blocking tile
        return 1 as libc::c_int
    }
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"fpathBlockingTile: off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              146 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"fpathGroundBlockingTile\x00")).as_ptr(),
              b"!(x <1 || y < 1 || x >= (SDWORD)mapWidth-1 || y >= (SDWORD)mapHeight-1)\x00"
                  as *const u8 as *const libc::c_char);
    };
    psTile = mapTile(x as UDWORD, y as UDWORD);
    /*
	// THIS CAN'T BE HERE - TESTING ONLY FIXME
	if( (TILE_HAS_STRUCTURE(psTile)) AND
		(getTileStructure(x,y)->pStructureType->type == REF_BLASTDOOR) AND	  // slow bit
		(getTileStructure(x,y)->player==selectedPlayer) )
	{
		return(FALSE);
	}
*/
    //#ifndef PSX // Must have to do this on PSX as well?
/*  This god awful hack RIP - John 15.2.99
	if(TILE_HAS_FEATURE(psTile))
	{
		psFeat = getTileFeature(x,y);
		if ((psFeat != NULL) &&
			(psFeat->psStats->subType == FEAT_GEN_ARTE OR psFeat->psStats->subType == FEAT_OIL_DRUM))
		{
			return(FALSE);
		}
	}*/
//#endif
    if (*psTile).tileInfoBits as libc::c_int & 0x10 as libc::c_int != 0 ||
           (*psTile).tileInfoBits as libc::c_int &
               (0x1 as libc::c_int | 0x2 as libc::c_int | 0x20 as libc::c_int)
               != 0 &&
               (*psTile).texture as libc::c_int & 0x200 as libc::c_int == 0 ||
           terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_CLIFFFACE as libc::c_int ||
           terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_WATER as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Check if the map tile at a location blocks a droid
#[no_mangle]
pub unsafe extern "C" fn fpathHoverBlockingTile(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if x < scrollMinX + 1 as libc::c_int || y < scrollMinY + 1 as libc::c_int
           || x >= scrollMaxX - 1 as libc::c_int ||
           y >= scrollMaxY - 1 as libc::c_int {
        // coords off map - auto blocking tile
        return 1 as libc::c_int
    }
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"fpathBlockingTile: off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              200 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"fpathHoverBlockingTile\x00")).as_ptr(),
              b"!(x <1 || y < 1 || x >= (SDWORD)mapWidth-1 || y >= (SDWORD)mapHeight-1)\x00"
                  as *const u8 as *const libc::c_char);
    };
    psTile = mapTile(x as UDWORD, y as UDWORD);
    if (*psTile).tileInfoBits as libc::c_int & 0x10 as libc::c_int != 0 ||
           (*psTile).tileInfoBits as libc::c_int &
               (0x1 as libc::c_int | 0x2 as libc::c_int | 0x20 as libc::c_int)
               != 0 &&
               (*psTile).texture as libc::c_int & 0x200 as libc::c_int == 0 ||
           terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_CLIFFFACE as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Check if the map tile at a location blocks a vtol
#[no_mangle]
pub unsafe extern "C" fn fpathLiftBlockingTile(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut iLiftHeight: SDWORD = 0;
    let mut iBlockingHeight: SDWORD = 0;
    let mut psDroid: *mut DROID = g_psObjRoute as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"fpathLiftBlockingTile: invalid object pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              221 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"fpathLiftBlockingTile\x00")).as_ptr(),
              b"PTRVALID(g_psObjRoute, sizeof(BASE_OBJECT))\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"fpathLiftBlockingTile: invalid droid pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              223 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"fpathLiftBlockingTile\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if x < 1 as libc::c_int || y < 1 as libc::c_int ||
               x >= mapWidth as SDWORD - 1 as libc::c_int ||
               y >= mapHeight as SDWORD - 1 as libc::c_int {
            return 1 as libc::c_int
        }
        psTile = mapTile(x as UDWORD, y as UDWORD);
        if (*psTile).tileInfoBits as libc::c_int & 0x80 as libc::c_int != 0 {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
    }
    if x < 1 as libc::c_int || y < 1 as libc::c_int ||
           x >= mapWidth as SDWORD - 1 as libc::c_int ||
           y >= mapHeight as SDWORD - 1 as libc::c_int {
        // coords off map - auto blocking tile
        return 1 as libc::c_int
    }
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"fpathLiftBlockingTile: off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(x < 1 as libc::c_int || y < 1 as libc::c_int ||
             x >= mapWidth as SDWORD - 1 as libc::c_int ||
             y >= mapHeight as SDWORD - 1 as libc::c_int) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              254 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"fpathLiftBlockingTile\x00")).as_ptr(),
              b"!(x <1 || y < 1 || x >= (SDWORD)mapWidth-1 || y >= (SDWORD)mapHeight-1)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* no tiles are blocking if returning to rearm */
    if (*psDroid).action == DACTION_MOVETOREARM as libc::c_int {
        return 0 as libc::c_int
    }
    psTile = mapTile(x as UDWORD, y as UDWORD);
    /* consider cliff faces */
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int ==
           TER_CLIFFFACE as libc::c_int {
        match (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int
                                                         as usize].nStat as
                                       libc::c_int as isize)).size as
                  libc::c_int {
            0 => { iBlockingHeight = 30 as libc::c_int }
            1 => { iBlockingHeight = 350 as libc::c_int }
            2 => { iBlockingHeight = 350 as libc::c_int }
            _ => { iBlockingHeight = 30 as libc::c_int }
        }
        /* approaching cliff face; block if below it */
        iLiftHeight =
            map_Height((x << 7 as libc::c_int) as UDWORD,
                       (y << 7 as libc::c_int) as UDWORD) as SDWORD -
                map_Height((*g_psObjRoute).x as UDWORD,
                           (*g_psObjRoute).y as UDWORD) as SDWORD;
        if iLiftHeight > iBlockingHeight {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
    } else if (*psTile).tileInfoBits as libc::c_int & 0x80 as libc::c_int != 0
     {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// Check if an edge map tile blocks a vtol (for sliding at map edge)
#[no_mangle]
pub unsafe extern "C" fn fpathLiftSlideBlockingTile(mut x: SDWORD,
                                                    mut y: SDWORD) -> BOOL {
    if x < 1 as libc::c_int || y < 1 as libc::c_int ||
           x >= GetWidthOfMap() as SDWORD - 1 as libc::c_int ||
           y >= GetHeightOfMap() as SDWORD - 1 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// Calculate the distance to a tile from a point
#[no_mangle]
pub unsafe extern "C" fn fpathDistToTile(mut tileX: SDWORD, mut tileY: SDWORD,
                                         mut pointX: SDWORD,
                                         mut pointY: SDWORD) -> SDWORD {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    // get the difference in tile coords
    xdiff = tileX - (pointX >> 7 as libc::c_int);
    ydiff = tileY - (pointY >> 7 as libc::c_int);
    if xdiff >= -(1 as libc::c_int) && xdiff <= 1 as libc::c_int &&
           ydiff >= -(1 as libc::c_int) && ydiff <= 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"fpathDistToTile: points are more than one tile apart\x00" as
                  *const u8 as *const libc::c_char);
    };
    if xdiff >= -(1 as libc::c_int) && xdiff <= 1 as libc::c_int &&
           ydiff >= -(1 as libc::c_int) && ydiff <= 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              331 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"fpathDistToTile\x00")).as_ptr(),
              b"(xdiff >= -1 && xdiff <= 1 && ydiff >= -1 && ydiff <= 1)\x00"
                  as *const u8 as *const libc::c_char);
    };
    if xdiff != 0 as libc::c_int || ydiff != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"fpathDistToTile: points are on same tile\x00" as *const u8 as
                  *const libc::c_char);
    };
    if xdiff != 0 as libc::c_int || ydiff != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              333 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"fpathDistToTile\x00")).as_ptr(),
              b"xdiff != 0 || ydiff != 0\x00" as *const u8 as
                  *const libc::c_char);
    };
    // not the most elegant solution but it works
    match xdiff + ydiff * 10 as libc::c_int {
        10 => {
            // xdiff == 0, ydiff == 1
            dist = 128 as libc::c_int - (pointY & 0x7f as libc::c_int)
        }
        9 => {
            // xdiff == -1, ydiff == 1
            tx = pointX & 0x7f as libc::c_int;
            ty = 128 as libc::c_int - (pointY & 0x7f as libc::c_int);
            dist =
                if tx > ty {
                    (tx) + ty / 2 as libc::c_int
                } else { (tx / 2 as libc::c_int) + ty }
        }
        -1 => {
            // xdiff == -1, ydiff == 0
            dist = pointX & 0x7f as libc::c_int
        }
        -11 => {
            // xdiff == -1, ydiff == -1
            tx = pointX & 0x7f as libc::c_int;
            ty = pointY & 0x7f as libc::c_int;
            dist =
                if tx > ty {
                    (tx) + ty / 2 as libc::c_int
                } else { (tx / 2 as libc::c_int) + ty }
        }
        -10 => {
            // xdiff == 0, ydiff == -1
            dist = pointY & 0x7f as libc::c_int
        }
        -9 => {
            // xdiff == 1, ydiff == -1
            tx = 128 as libc::c_int - (pointX & 0x7f as libc::c_int);
            ty = pointY & 0x7f as libc::c_int;
            dist =
                if tx > ty {
                    (tx) + ty / 2 as libc::c_int
                } else { (tx / 2 as libc::c_int) + ty }
        }
        1 => {
            // xdiff == 1, ydiff == 0
            dist = 128 as libc::c_int - (pointX & 0x7f as libc::c_int)
        }
        11 => {
            // xdiff == 1, ydiff == 1
            tx = 128 as libc::c_int - (pointX & 0x7f as libc::c_int);
            ty = 128 as libc::c_int - (pointY & 0x7f as libc::c_int);
            dist =
                if tx > ty {
                    (tx) + ty / 2 as libc::c_int
                } else { (tx / 2 as libc::c_int) + ty }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"fpathDistToTile: unexpected point relationship\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"fpath.c\x00" as *const u8 as *const libc::c_char,
                      371 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"fpathDistToTile\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            dist = 128 as libc::c_int
        }
    }
    return dist;
}
// Variables for the callback
static mut finalX: SDWORD = 0;
static mut finalY: SDWORD = 0;
static mut vectorX: SDWORD = 0;
static mut vectorY: SDWORD = 0;
static mut clearX: SDWORD = 0;
static mut clearY: SDWORD = 0;
static mut obstruction: BOOL = 0;
// callback to find the first clear tile before an obstructed target
#[no_mangle]
pub unsafe extern "C" fn fpathEndPointCallback(mut x: SDWORD, mut y: SDWORD,
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
    // note the last clear tile
    if fpathBlockingTile.expect("non-null function pointer")(x >>
                                                                 7 as
                                                                     libc::c_int,
                                                             y >>
                                                                 7 as
                                                                     libc::c_int)
           == 0 {
        clearX =
            (x & !(0x7f as libc::c_int)) +
                128 as libc::c_int / 2 as libc::c_int;
        clearY =
            (y & !(0x7f as libc::c_int)) +
                128 as libc::c_int / 2 as libc::c_int
    } else { obstruction = 1 as libc::c_int }
    return 1 as libc::c_int;
}
/* set direct path to position */
#[no_mangle]
pub unsafe extern "C" fn fpathSetDirectRoute(mut psObj: *mut BASE_OBJECT,
                                             mut targetX: SDWORD,
                                             mut targetY: SDWORD) {
    let mut psMoveCntl: *mut MOVE_CONTROL = 0 as *mut MOVE_CONTROL;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"fpathSetDirectRoute: invalid object pointer\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"fpath.c\x00" as *const u8 as *const libc::c_char,
              418 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"fpathSetDirectRoute\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psMoveCntl = &mut (*(psObj as *mut DROID)).sMove;
        /* set global pointer for object being routed - GJ hack */
        fpathSetCurrentObject(psObj);
        (*psMoveCntl).DestinationX = targetX;
        (*psMoveCntl).DestinationY = targetY;
        //		psMoveCntl->MovementList[0].XCoordinate = targetX;
//		psMoveCntl->MovementList[0].YCoordinate = targetY;
//		psMoveCntl->MovementList[1].XCoordinate = -1;
//		psMoveCntl->MovementList[1].YCoordinate = -1;
        (*psMoveCntl).numPoints = 1 as libc::c_int as UBYTE;
        (*psMoveCntl).asPath[0 as libc::c_int as usize].x =
            (targetX >> 7 as libc::c_int) as UBYTE;
        (*psMoveCntl).asPath[0 as libc::c_int as usize].y =
            (targetY >> 7 as libc::c_int) as UBYTE
    };
}
// append an astar route onto a move-control route
#[no_mangle]
pub unsafe extern "C" fn fpathAppendRoute(mut psMoveCntl: *mut MOVE_CONTROL,
                                          mut psAStarRoute:
                                              *mut ASTAR_ROUTE) {
    let mut mi: SDWORD = 0;
    let mut ai: SDWORD = 0;
    // find the end of the last route
//	for(mi=0; psMoveCntl->MovementList[mi].XCoordinate != -1; mi += 1)
//		;
    mi = (*psMoveCntl).numPoints as SDWORD;
    ai = 0 as libc::c_int;
    while mi < 100 as libc::c_int && ai < (*psAStarRoute).numPoints {
        //		psMoveCntl->MovementList[mi].XCoordinate =
        (*psMoveCntl).asPath[mi as usize].x =
            (*psAStarRoute).asPos[ai as usize].x as UBYTE;
        //			(UBYTE)(psAStarRoute->asPos[ai].x >> TILE_SHIFT);
//		psMoveCntl->MovementList[mi].YCoordinate =
        (*psMoveCntl).asPath[mi as usize].y =
            (*psAStarRoute).asPos[ai as usize].y as UBYTE;
        //			(UBYTE)(psAStarRoute->asPos[ai].y >> TILE_SHIFT);
        ai += 1 as libc::c_int;
        mi += 1 as libc::c_int
    }
    //	psMoveCntl->MovementList[mi].XCoordinate = -1;
//	psMoveCntl->MovementList[mi].YCoordinate = -1;
    (*psMoveCntl).numPoints =
        ((*psMoveCntl).numPoints as libc::c_int + ai) as UBYTE;
    (*psMoveCntl).DestinationX =
        ((*psAStarRoute).finalX << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int;
    (*psMoveCntl).DestinationY =
        ((*psAStarRoute).finalY << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int;
}
// set the routing block flags for the gateways
/*void fpathSetGatewayBlock(void)
{
	GATEWAY		*psCurr;
	SDWORD		pos;
	MAPTILE		*psTile;

	for(psCurr=psGateways; psCurr; psCurr=psCurr->psNext)
	{
		if (!(psCurr->flags & GWR_INROUTE))
		{
			if (psCurr->x1 == psCurr->x2)
			{
				for(pos = psCurr->y1; pos <= psCurr->y2; pos += 1)
				{
					psTile = mapTile(psCurr->x1, pos);
					psTile->tileInfoBits |= BITS_FPATHBLOCK;
				}
			}
			else
			{
				for(pos = psCurr->x1; pos <= psCurr->x2; pos += 1)
				{
					psTile = mapTile(pos, psCurr->y1);
					psTile->tileInfoBits |= BITS_FPATHBLOCK;
				}
			}
		}
	}
}*/
// clear the routing block flags for the gateways
/*void fpathClearGatewayBlock(void)
{
	GATEWAY		*psCurr;
	SDWORD		pos;
	MAPTILE		*psTile;

	for(psCurr=psGateways; psCurr; psCurr=psCurr->psNext)
	{
//		if (!(psCurr->flags & GWR_INROUTE))
		{
			if (psCurr->x1 == psCurr->x2)
			{
				for(pos = psCurr->y1; pos <= psCurr->y2; pos += 1)
				{
					psTile = mapTile(psCurr->x1, pos);
					psTile->tileInfoBits &= ~BITS_FPATHBLOCK;
				}
			}
			else
			{
				for(pos = psCurr->x1; pos <= psCurr->x2; pos += 1)
				{
					psTile = mapTile(pos, psCurr->y1);
					psTile->tileInfoBits &= ~BITS_FPATHBLOCK;
				}
			}
		}
	}
}*/
// check whether a WORLD coordinate point is within a gateway's tiles
#[no_mangle]
pub unsafe extern "C" fn fpathPointInGateway(mut x: SDWORD, mut y: SDWORD,
                                             mut psGate: *mut GATEWAY)
 -> BOOL {
    x = x >> 7 as libc::c_int;
    y = y >> 7 as libc::c_int;
    if x >= (*psGate).x1 as libc::c_int && x <= (*psGate).x2 as libc::c_int &&
           y >= (*psGate).y1 as libc::c_int &&
           y <= (*psGate).y2 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// set blocking flags for all gateways around a zone
#[no_mangle]
pub unsafe extern "C" fn fpathSetGatewayBlock(mut zone: SDWORD,
                                              mut psLast: *mut GATEWAY,
                                              mut psNext: *mut GATEWAY) {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut pos: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut blockZone: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psCurr = psGateways;
    while !psCurr.is_null() {
        if psCurr != psLast && psCurr != psNext &&
               (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int
                   == 0 &&
               ((*psCurr).zone1 as libc::c_int == zone ||
                    (*psCurr).zone2 as libc::c_int == zone) {
            if (*psCurr).x1 as libc::c_int == (*psCurr).x2 as libc::c_int {
                pos = (*psCurr).y1 as SDWORD;
                while pos <= (*psCurr).y2 as libc::c_int {
                    psTile = mapTile((*psCurr).x1 as UDWORD, pos as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x10 as libc::c_int) as UBYTE;
                    pos += 1 as libc::c_int
                }
            } else {
                pos = (*psCurr).x1 as SDWORD;
                while pos <= (*psCurr).x2 as libc::c_int {
                    psTile = mapTile(pos as UDWORD, (*psCurr).y1 as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x10 as libc::c_int) as UBYTE;
                    pos += 1 as libc::c_int
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    // now set the blocking flags next to the two gateways that the route
	// is going through
    if fpathDoMessage != 0 {
        debug(LOG_NEVER,
              b"Blocking gateways for zones :\x00" as *const u8 as
                  *const libc::c_char);
    }
    if !psLast.is_null() {
        blockZone =
            if (*psLast).flags as libc::c_int & GWR_ZONE1 as libc::c_int != 0
               {
                (*psLast).zone1 as libc::c_int
            } else { (*psLast).zone2 as libc::c_int };
        if fpathDoMessage != 0 {
            debug(LOG_NEVER, blockZone as *const libc::c_char);
        }
        tx = (*psLast).x1 as libc::c_int - 1 as libc::c_int;
        while tx <= (*psLast).x2 as libc::c_int + 1 as libc::c_int {
            ty = (*psLast).y1 as libc::c_int - 1 as libc::c_int;
            while ty <= (*psLast).y2 as libc::c_int + 1 as libc::c_int {
                if fpathPointInGateway(tx << 7 as libc::c_int,
                                       ty << 7 as libc::c_int, psLast) == 0 &&
                       tileOnMap(tx, ty) != 0 &&
                       gwGetZone(tx, ty) == blockZone {
                    psTile = mapTile(tx as UDWORD, ty as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x10 as libc::c_int) as UBYTE
                }
                ty += 1
            }
            tx += 1
        }
    }
    if !psNext.is_null() {
        blockZone =
            if (*psNext).flags as libc::c_int & GWR_ZONE1 as libc::c_int != 0
               {
                (*psNext).zone2 as libc::c_int
            } else { (*psNext).zone1 as libc::c_int };
        if fpathDoMessage != 0 {
            debug(LOG_NEVER, blockZone as *const libc::c_char);
        }
        tx = (*psNext).x1 as libc::c_int - 1 as libc::c_int;
        while tx <= (*psNext).x2 as libc::c_int + 1 as libc::c_int {
            ty = (*psNext).y1 as libc::c_int - 1 as libc::c_int;
            while ty <= (*psNext).y2 as libc::c_int + 1 as libc::c_int {
                if fpathPointInGateway(tx << 7 as libc::c_int,
                                       ty << 7 as libc::c_int, psNext) == 0 &&
                       tileOnMap(tx, ty) != 0 &&
                       gwGetZone(tx, ty) == blockZone {
                    psTile = mapTile(tx as UDWORD, ty as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x10 as libc::c_int) as UBYTE
                }
                ty += 1
            }
            tx += 1
        }
    }
    if fpathDoMessage != 0 {
        debug(LOG_NEVER, b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
// clear blocking flags for all gateways around a zone
#[no_mangle]
pub unsafe extern "C" fn fpathClearGatewayBlock(mut zone: SDWORD,
                                                mut psLast: *mut GATEWAY,
                                                mut psNext: *mut GATEWAY) {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut pos: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut blockZone: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psCurr = psGateways;
    while !psCurr.is_null() {
        if (*psCurr).flags as libc::c_int & GWR_WATERLINK as libc::c_int == 0
               &&
               ((*psCurr).zone1 as libc::c_int == zone ||
                    (*psCurr).zone2 as libc::c_int == zone) {
            if (*psCurr).x1 as libc::c_int == (*psCurr).x2 as libc::c_int {
                pos = (*psCurr).y1 as SDWORD;
                while pos <= (*psCurr).y2 as libc::c_int {
                    psTile = mapTile((*psCurr).x1 as UDWORD, pos as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int &
                             !(0x10 as libc::c_int)) as UBYTE;
                    pos += 1 as libc::c_int
                }
            } else {
                pos = (*psCurr).x1 as SDWORD;
                while pos <= (*psCurr).x2 as libc::c_int {
                    psTile = mapTile(pos as UDWORD, (*psCurr).y1 as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int &
                             !(0x10 as libc::c_int)) as UBYTE;
                    pos += 1 as libc::c_int
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    // clear the flags around the route gateways
    if !psLast.is_null() {
        blockZone =
            if (*psLast).flags as libc::c_int & GWR_ZONE1 as libc::c_int != 0
               {
                (*psLast).zone1 as libc::c_int
            } else { (*psLast).zone2 as libc::c_int };
        tx = (*psLast).x1 as libc::c_int - 1 as libc::c_int;
        while tx <= (*psLast).x2 as libc::c_int + 1 as libc::c_int {
            ty = (*psLast).y1 as libc::c_int - 1 as libc::c_int;
            while ty <= (*psLast).y2 as libc::c_int + 1 as libc::c_int {
                if fpathPointInGateway(tx << 7 as libc::c_int,
                                       ty << 7 as libc::c_int, psLast) == 0 &&
                       tileOnMap(tx, ty) != 0 &&
                       gwGetZone(tx, ty) == blockZone {
                    psTile = mapTile(tx as UDWORD, ty as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int &
                             !(0x10 as libc::c_int)) as UBYTE
                }
                ty += 1
            }
            tx += 1
        }
    }
    if !psNext.is_null() {
        blockZone =
            if (*psNext).flags as libc::c_int & GWR_ZONE1 as libc::c_int != 0
               {
                (*psNext).zone2 as libc::c_int
            } else { (*psNext).zone1 as libc::c_int };
        tx = (*psNext).x1 as libc::c_int - 1 as libc::c_int;
        while tx <= (*psNext).x2 as libc::c_int + 1 as libc::c_int {
            ty = (*psNext).y1 as libc::c_int - 1 as libc::c_int;
            while ty <= (*psNext).y2 as libc::c_int + 1 as libc::c_int {
                if fpathPointInGateway(tx << 7 as libc::c_int,
                                       ty << 7 as libc::c_int, psNext) == 0 &&
                       tileOnMap(tx, ty) != 0 &&
                       gwGetZone(tx, ty) == blockZone {
                    psTile = mapTile(tx as UDWORD, ty as UDWORD);
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int &
                             !(0x10 as libc::c_int)) as UBYTE
                }
                ty += 1
            }
            tx += 1
        }
    };
}
// clear the routing ignore flags for the gateways
#[no_mangle]
pub unsafe extern "C" fn fpathClearIgnore() {
    let mut psCurr: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut link: SDWORD = 0;
    let mut numLinks: SDWORD = 0;
    psCurr = psGateways;
    while !psCurr.is_null() {
        (*psCurr).flags =
            ((*psCurr).flags as libc::c_int & !(GWR_IGNORE as libc::c_int)) as
                UBYTE;
        numLinks =
            (*psCurr).zone1Links as libc::c_int +
                (*psCurr).zone2Links as libc::c_int;
        link = 0 as libc::c_int;
        while link < numLinks {
            let ref mut fresh0 =
                (*(*psCurr).psLinks.offset(link as isize)).flags;
            *fresh0 =
                (*fresh0 as libc::c_int & !(GWRL_BLOCKED as libc::c_int)) as
                    SWORD;
            link += 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    };
}
// find a clear tile on a gateway to route to
#[no_mangle]
pub unsafe extern "C" fn fpathGatewayCoords(mut psGate: *mut GATEWAY,
                                            mut px: *mut SDWORD,
                                            mut py: *mut SDWORD) {
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    let mut dist: SDWORD = 0;
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    let mut pos: SDWORD = 0;
    // find the clear tile nearest to the middle
    mx =
        ((*psGate).x1 as libc::c_int + (*psGate).x2 as libc::c_int) /
            2 as libc::c_int;
    my =
        ((*psGate).y1 as libc::c_int + (*psGate).y2 as libc::c_int) /
            2 as libc::c_int;
    dist = 0x7fffffff as libc::c_int;
    if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
        pos = (*psGate).y1 as SDWORD;
        while pos <= (*psGate).y2 as libc::c_int {
            if fpathBlockingTile.expect("non-null function pointer")((*psGate).x1
                                                                         as
                                                                         SDWORD,
                                                                     pos) == 0
                   && abs(pos - my) < dist {
                x = (*psGate).x1 as SDWORD;
                y = pos;
                dist = abs(pos - my)
            }
            pos += 1 as libc::c_int
        }
    } else {
        pos = (*psGate).x1 as SDWORD;
        while pos <= (*psGate).x2 as libc::c_int {
            if fpathBlockingTile.expect("non-null function pointer")(pos,
                                                                     (*psGate).y1
                                                                         as
                                                                         SDWORD)
                   == 0 && abs(pos - mx) < dist {
                x = pos;
                y = (*psGate).y1 as SDWORD;
                dist = abs(pos - mx)
            }
            pos += 1 as libc::c_int
        }
    }
    // if no clear tile is found just return the middle
    if dist == 0x7fffffff as libc::c_int { x = mx; y = my }
    *px = x * 128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
    *py = y * 128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
}
// create a final route from a gateway route
// check if the route to a gateway has already been generated
#[no_mangle]
pub unsafe extern "C" fn fpathCheckRouteMatch(mut psMoveCntl:
                                                  *mut MOVE_CONTROL,
                                              mut pPos: *mut SDWORD,
                                              mut gwx: SDWORD,
                                              mut gwy: SDWORD) -> BOOL {
    let mut pos: SDWORD = *pPos;
    while pos < (*psMoveCntl).numPoints as libc::c_int {
        if (*psMoveCntl).asPath[pos as usize].x as libc::c_int ==
               gwx >> 7 as libc::c_int &&
               (*psMoveCntl).asPath[pos as usize].y as libc::c_int ==
                   gwy >> 7 as libc::c_int {
            *pPos = pos + 1 as libc::c_int;
            return 1 as libc::c_int
        }
        pos += 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fpathBlockGatewayLink(mut psLast: *mut GATEWAY,
                                               mut psCurr: *mut GATEWAY) {
    let mut link: SDWORD = 0;
    let mut numLinks: SDWORD = 0;
    if psLast.is_null() && !psCurr.is_null() {
        if fpathDoMessage != 0 {
            debug(LOG_NEVER,
                  b"   Blocking first gateway\n\x00" as *const u8 as
                      *const libc::c_char);
        }
        (*psCurr).flags =
            ((*psCurr).flags as libc::c_int | GWR_IGNORE as libc::c_int) as
                UBYTE
    } else if psCurr.is_null() && !psLast.is_null() {
        if fpathDoMessage != 0 {
            debug(LOG_NEVER,
                  b"   Blocking last gateway\n\x00" as *const u8 as
                      *const libc::c_char);
        }
        (*psLast).flags =
            ((*psLast).flags as libc::c_int | GWR_IGNORE as libc::c_int) as
                UBYTE
    } else if !psLast.is_null() && !psCurr.is_null() {
        if fpathDoMessage != 0 {
            debug(LOG_NEVER,
                  b"   Blocking link between gateways\x00" as *const u8 as
                      *const libc::c_char);
        }
        numLinks =
            (*psLast).zone1Links as libc::c_int +
                (*psLast).zone2Links as libc::c_int;
        link = 0 as libc::c_int;
        while link < numLinks {
            //			if (psLast->psLinks[link].psGateway == psCurr)
            if (*(*psLast).psLinks.offset(link as isize)).flags as libc::c_int
                   & GWRL_CHILD as libc::c_int != 0 {
                if fpathDoMessage != 0 {
                    debug(LOG_NEVER, link as *const libc::c_char);
                }
                let ref mut fresh1 =
                    (*(*psLast).psLinks.offset(link as isize)).flags;
                *fresh1 =
                    (*fresh1 as libc::c_int | GWRL_BLOCKED as libc::c_int) as
                        SWORD
            }
            link += 1 as libc::c_int
        }
        numLinks =
            (*psCurr).zone1Links as libc::c_int +
                (*psCurr).zone2Links as libc::c_int;
        link = 0 as libc::c_int;
        while link < numLinks {
            //			if (psCurr->psLinks[link].psGateway == psLast)
            if (*(*psCurr).psLinks.offset(link as isize)).flags as libc::c_int
                   & GWRL_PARENT as libc::c_int != 0 {
                if fpathDoMessage != 0 {
                    debug(LOG_NEVER, link as *const libc::c_char);
                }
                let ref mut fresh2 =
                    (*(*psCurr).psLinks.offset(link as isize)).flags;
                *fresh2 =
                    (*fresh2 as libc::c_int | GWRL_BLOCKED as libc::c_int) as
                        SWORD
            }
            link += 1 as libc::c_int
        }
        if fpathDoMessage != 0 {
            debug(LOG_NEVER, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
// check if a new route is closer to the target than the one stored in
// the droid
#[no_mangle]
pub unsafe extern "C" fn fpathRouteCloser(mut psMoveCntl: *mut MOVE_CONTROL,
                                          mut psAStarRoute: *mut ASTAR_ROUTE,
                                          mut tx: SDWORD, mut ty: SDWORD)
 -> BOOL {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut prevDist: SDWORD = 0;
    let mut nextDist: SDWORD = 0;
    if (*psAStarRoute).numPoints == 0 as libc::c_int {
        // no route to copy do nothing
        return 0 as libc::c_int
    }
    if (*psMoveCntl).numPoints as libc::c_int == 0 as libc::c_int {
        // no previous route - this has to be better
        return 1 as libc::c_int
    }
    // see which route is closest to the final destination
    xdiff =
        (((*psMoveCntl).asPath[((*psMoveCntl).numPoints as libc::c_int -
                                    1 as libc::c_int) as usize].x as
              libc::c_int) << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int - tx;
    ydiff =
        (((*psMoveCntl).asPath[((*psMoveCntl).numPoints as libc::c_int -
                                    1 as libc::c_int) as usize].y as
              libc::c_int) << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int - ty;
    prevDist = xdiff * xdiff + ydiff * ydiff;
    xdiff =
        ((*psAStarRoute).finalX << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int - tx;
    ydiff =
        ((*psAStarRoute).finalY << 7 as libc::c_int) +
            128 as libc::c_int / 2 as libc::c_int - ty;
    nextDist = xdiff * xdiff + ydiff * ydiff;
    if nextDist < prevDist { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
// create a final route from a gateway route
#[no_mangle]
pub unsafe extern "C" fn fpathGatewayRoute(mut psObj: *mut BASE_OBJECT,
                                           mut routeMode: SDWORD,
                                           mut GWTerrain: SDWORD,
                                           mut sx: SDWORD, mut sy: SDWORD,
                                           mut fx: SDWORD, mut fy: SDWORD,
                                           mut psMoveCntl: *mut MOVE_CONTROL)
 -> SDWORD {
    let mut current_block: u64;
    static mut linkx: SDWORD = 0;
    static mut linky: SDWORD = 0;
    static mut gwx: SDWORD = 0;
    static mut gwy: SDWORD = 0;
    static mut asret: SDWORD = 0;
    static mut matchPoints: SDWORD = 0;
    static mut sAStarRoute: ASTAR_ROUTE =
        ASTAR_ROUTE{asPos: [POINT{x: 0, y: 0,}; 50],
                    finalX: 0,
                    finalY: 0,
                    numPoints: 0,};
    let mut retval: SDWORD = FPR_OK as libc::c_int;
    let mut gwRet: SDWORD = 0;
    let mut zone: SDWORD = 0;
    static mut psCurrRoute: *mut GATEWAY =
        0 as *const GATEWAY as *mut GATEWAY;
    static mut psGWRoute: *mut GATEWAY = 0 as *const GATEWAY as *mut GATEWAY;
    static mut psLastGW: *mut GATEWAY = 0 as *const GATEWAY as *mut GATEWAY;
    let mut bRouting: BOOL = 0;
    let mut bFinished: BOOL = 0;
    static mut bFirstRoute: BOOL = 0;
    if routeMode == ASR_NEWROUTE as libc::c_int {
        fpathClearIgnore();
        // initialise the move control structures
        (*psMoveCntl).numPoints = 0 as libc::c_int as UBYTE;
        sAStarRoute.numPoints = 0 as libc::c_int;
        bFirstRoute = 1 as libc::c_int
    }
    // keep trying gateway routes until out of options
    bRouting = 1 as libc::c_int;
    's_50:
        loop  {
            if !(bRouting != 0) {
                current_block = 7297078374430259003;
                break ;
            }
            if routeMode == ASR_NEWROUTE as libc::c_int {
                if fpathDoMessage != 0 {
                    debug(LOG_NEVER, (*psObj).id as *const libc::c_char);
                }
                gwRet =
                    gwrAStarRoute((*psObj).player as SDWORD,
                                  GWTerrain as UDWORD, sx, sy, fx, fy,
                                  &mut psGWRoute);
                match gwRet {
                    1 => {
                        // need to deal with this case for retried routing - only accept this if no previous route?
                        if bFirstRoute == 0 {
                            if (*psMoveCntl).numPoints as libc::c_int >
                                   0 as libc::c_int {
                                if fpathDoMessage != 0 {
                                    debug(LOG_NEVER,
                                          b"   Gateway route nearest - Use previous route\n\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                }
                                retval = FPR_OK as libc::c_int;
                                current_block = 17175546144130745016;
                                break ;
                            } else {
                                if fpathDoMessage != 0 {
                                    debug(LOG_NEVER,
                                          b"   Gateway route nearest - No points - failed\n\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                }
                                retval = FPR_FAILED as libc::c_int;
                                current_block = 17175546144130745016;
                                break ;
                            }
                        }
                    }
                    4 | 3 => {
                        // no zone information - try a normal route
                        psGWRoute = 0 as *mut GATEWAY
                    }
                    2 => {
                        if fpathDoMessage != 0 {
                            debug(LOG_NEVER,
                                  b"   Gateway route failed\n\x00" as
                                      *const u8 as *const libc::c_char);
                        }
                        if (*psObj).type_0 as libc::c_uint ==
                               OBJ_DROID as libc::c_int as libc::c_uint &&
                               vtolDroid(psObj as *mut DROID) != 0 {
                            // just fail for VTOLs - they can set a direct route
                            retval = FPR_FAILED as libc::c_int;
                            current_block = 17175546144130745016;
                            break ;
                        } else { psGWRoute = 0 as *mut GATEWAY }
                    }
                    0 | _ => { }
                }
                // reset matchPoints so that routing between gateways generated
			// by the previous gateway route can be reused
                matchPoints = 0 as libc::c_int;
                sAStarRoute.numPoints = 0 as libc::c_int
            }
            bFirstRoute = 0 as libc::c_int;
            if routeMode == ASR_NEWROUTE as libc::c_int {
                // if the start of the route is on the first gateway, skip it
                if !psGWRoute.is_null() &&
                       fpathPointInGateway(sx, sy, psGWRoute) != 0 {
                    psGWRoute = (*psGWRoute).psRoute
                }
                linkx = sx;
                linky = sy;
                psCurrRoute = psGWRoute;
                psLastGW = 0 as *mut GATEWAY
            }
            // now generate the route
            bRouting = 0 as libc::c_int;
            bFinished = 0 as libc::c_int;
            while bFinished == 0 {
                if psCurrRoute.is_null() ||
                       (*psCurrRoute).psRoute.is_null() &&
                           fpathPointInGateway(fx, fy, psCurrRoute) != 0 {
                    // last stretch on the route is not to a gatway but to
				// the final route coordinates
                    gwx = fx;
                    gwy = fy;
                    zone =
                        gwGetZone(fx >> 7 as libc::c_int,
                                  fy >> 7 as libc::c_int)
                } else {
                    fpathGatewayCoords(psCurrRoute, &mut gwx, &mut gwy);
                    zone =
                        if (*psCurrRoute).flags as libc::c_int &
                               GWR_ZONE1 as libc::c_int != 0 {
                            (*psCurrRoute).zone1 as libc::c_int
                        } else { (*psCurrRoute).zone2 as libc::c_int }
                }
                // only route between the gateways if it wasn't done on a previous route
//			if (!fpathCheckRouteMatch(psMoveCntl, &matchPoints, gwx,gwy))
//			if (1)
                //				psMoveCntl->numPoints = (UBYTE)matchPoints;
                if fpathDoMessage != 0 {
                    debug(LOG_NEVER, zone as *const libc::c_char);
                }
                fpathSetGatewayBlock(zone, psLastGW, psCurrRoute);
                asret =
                    fpathAStarRoute(routeMode, &mut sAStarRoute, linkx, linky,
                                    gwx, gwy);
                fpathClearGatewayBlock(zone, psLastGW, psCurrRoute);
                if asret == ASR_PARTIAL as libc::c_int {
                    // routing hasn't finished yet
                    if fpathDoMessage != 0 {
                        debug(LOG_NEVER,
                              b"   Reschedule\n\x00" as *const u8 as
                                  *const libc::c_char);
                    }
                    retval = FPR_WAIT as libc::c_int;
                    current_block = 17175546144130745016;
                    break 's_50 ;
                } else {
                    /*				else if (asret != ASR_FAILED)
				{
					fpathAppendRoute(psMoveCntl, &sAStarRoute);
					matchPoints = psMoveCntl->numPoints;
				}*/
                    routeMode = ASR_NEWROUTE as libc::c_int;
                    if asret == ASR_NEAREST as libc::c_int &&
                           actionRouteBlockingPos(psObj as *mut DROID,
                                                  sAStarRoute.finalX,
                                                  sAStarRoute.finalY) != 0 {
                        // found a blocking wall - route to that
                        if fpathDoMessage != 0 {
                            debug(LOG_NEVER,
                                  b"   Got blocking wall\n\x00" as *const u8
                                      as *const libc::c_char);
                        }
                        retval = FPR_OK as libc::c_int;
                        current_block = 17175546144130745016;
                        break 's_50 ;
                    } else if asret == ASR_NEAREST as libc::c_int &&
                                  psGWRoute.is_null() {
                        // all routing was in one zone - this is as good as it's going to be
                        if fpathDoMessage != 0 {
                            debug(LOG_NEVER,
                                  b"   Nearest route in same zone\n\x00" as
                                      *const u8 as *const libc::c_char);
                        }
                        if fpathRouteCloser(psMoveCntl, &mut sAStarRoute, fx,
                                            fy) != 0 {
                            (*psMoveCntl).numPoints =
                                0 as libc::c_int as UBYTE;
                            fpathAppendRoute(psMoveCntl, &mut sAStarRoute);
                        }
                        retval = FPR_OK as libc::c_int;
                        current_block = 17175546144130745016;
                        break 's_50 ;
                    } else if asret == ASR_FAILED as libc::c_int &&
                                  psGWRoute.is_null() {
                        // all routing was in one zone - can't retry
                        if fpathDoMessage != 0 {
                            debug(LOG_NEVER,
                                  b"   Failed route in same zone\n\x00" as
                                      *const u8 as *const libc::c_char);
                        }
                        retval = FPR_FAILED as libc::c_int;
                        current_block = 17175546144130745016;
                        break 's_50 ;
                    } else if asret == ASR_FAILED as libc::c_int ||
                                  asret == ASR_NEAREST as libc::c_int {
                        // no route found - try ditching this gateway
					// and trying a new gateway route
                        if fpathDoMessage != 0 {
                            debug(LOG_NEVER,
                                  b"   Route failed - ignore gateway/link and reroute\n\x00"
                                      as *const u8 as *const libc::c_char);
                        }
                        if fpathRouteCloser(psMoveCntl, &mut sAStarRoute, fx,
                                            fy) != 0 {
                            (*psMoveCntl).numPoints =
                                0 as libc::c_int as UBYTE;
                            fpathAppendRoute(psMoveCntl, &mut sAStarRoute);
                        }
                        fpathBlockGatewayLink(psLastGW, psCurrRoute);
                        bRouting = 1 as libc::c_int;
                        break ;
                    } else {
                        linkx = gwx;
                        linky = gwy;
                        psLastGW = psCurrRoute;
                        if !psCurrRoute.is_null() {
                            psCurrRoute = (*psCurrRoute).psRoute
                        } else { bFinished = 1 as libc::c_int }
                    }
                }
            }
        }
    match current_block {
        7297078374430259003 => {
            if fpathRouteCloser(psMoveCntl, &mut sAStarRoute, fx, fy) != 0 {
                (*psMoveCntl).numPoints = 0 as libc::c_int as UBYTE;
                fpathAppendRoute(psMoveCntl, &mut sAStarRoute);
            }
        }
        _ => { }
    }
    // reset the routing block flags
    if retval != FPR_WAIT as libc::c_int { fpathClearIgnore(); }
    return retval;
}
/* set pointer for current fpath object - GJ hack */
/* set pointer for current fpath object - GJ hack */
#[no_mangle]
pub unsafe extern "C" fn fpathSetCurrentObject(mut psObj: *mut BASE_OBJECT) {
    g_psObjRoute = psObj;
}
/* set the correct blocking tile function */
// set the correct blocking tile function
#[no_mangle]
pub unsafe extern "C" fn fpathSetBlockingTile(mut ubPropulsionType: UBYTE) {
    match ubPropulsionType as libc::c_int {
        3 => {
            fpathBlockingTile =
                Some(fpathHoverBlockingTile as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL)
        }
        5 => {
            fpathBlockingTile =
                Some(fpathLiftBlockingTile as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL)
        }
        _ => {
            fpathBlockingTile =
                Some(fpathGroundBlockingTile as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL)
        }
    };
}
// Find a route for an object to a location
// Find a route for an object to a location
#[no_mangle]
pub unsafe extern "C" fn fpathRoute(mut psObj: *mut BASE_OBJECT,
                                    mut psMoveCntl: *mut MOVE_CONTROL,
                                    mut tX: SDWORD, mut tY: SDWORD)
 -> FPATH_RETVAL {
    let mut current_block: u64;
    let mut startX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut targetX: SDWORD = 0;
    let mut targetY: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut dir: SDWORD = 0;
    let mut nearestDir: SDWORD = 0;
    let mut minDist: SDWORD = 0;
    let mut tileDist: SDWORD = 0;
    let mut retVal: FPATH_RETVAL = FPR_OK;
    //	DROID				*psCurr;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut GWTerrain: UDWORD = 0;
    /* set global pointer for object being routed - GJ hack */
    fpathSetCurrentObject(psObj);
    if psPartialRouteObj.is_null() || psPartialRouteObj != psObj {
        targetX = tX;
        targetY = tY;
        startX = (*psObj).x as SDWORD;
        startY = (*psObj).y as SDWORD
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_DROID as libc::c_int as libc::c_uint &&
                  (*(psObj as *mut DROID)).sMove.Status as libc::c_int ==
                      11 as libc::c_int &&
                  ((*(psObj as *mut DROID)).sMove.DestinationX != tX ||
                       (*(psObj as *mut DROID)).sMove.DestinationX != tX) {
        psPartialRouteObj = 0 as *mut BASE_OBJECT;
        targetX = tX;
        targetY = tY;
        startX = (*psObj).x as SDWORD;
        startY = (*psObj).y as SDWORD
    } else {
        // continuing routing for the object
        startX = partialSX;
        startY = partialSY;
        targetX = partialTX;
        targetY = partialTY
    }
    // don't have to do anything if already there
    if startX == targetX && startY == targetY {
        // return failed to stop them moving anywhere
        return FPR_FAILED
    }
    // set the correct blocking tile function and gateway terrain flag
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psObj as *mut DROID;
        psPropStats =
            asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                         libc::c_int as isize);
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"fpathRoute: invalid propulsion stats pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"fpath.c\x00" as *const u8 as *const libc::c_char,
                  1358 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"fpathRoute\x00")).as_ptr(),
                  b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                      *const u8 as *const libc::c_char);
        };
        fpathSetBlockingTile((*psPropStats).propulsionType);
        /* set gateway terrain flag */
        match (*psPropStats).propulsionType as libc::c_int {
            3 => { GWTerrain = GWR_TER_ALL as libc::c_int as UDWORD }
            5 => { GWTerrain = GWR_TER_ALL as libc::c_int as UDWORD }
            _ => { GWTerrain = GWR_TER_LAND as libc::c_int as UDWORD }
        }
    } else { GWTerrain = GWR_TER_LAND as libc::c_int as UDWORD }
    // set all the flags for stationary droids
/*	for(psCurr = apsDroidLists[psObj->player]; psCurr; psCurr = psCurr->psNext)
	{
		if (psCurr != (DROID *)psObj &&
			!psCurr->selected &&
			psCurr->sMove.Status == MOVEINACTIVE)
		{
			psTile = mapTile(psCurr->x >> TILE_SHIFT, psCurr->y >> TILE_SHIFT);
			psTile->tileInfoBits |= BITS_FPATHBLOCK;
		}
	}*/
    if psPartialRouteObj.is_null() || psPartialRouteObj != psObj {
        // check whether the start point of the route
		// is a blocking tile and find an alternative if it is
        if fpathBlockingTile.expect("non-null function pointer")(startX >>
                                                                     7 as
                                                                         libc::c_int,
                                                                 startY >>
                                                                     7 as
                                                                         libc::c_int)
               != 0 {
            // find the nearest non blocking tile to the object
            minDist = 0x7fffffff as libc::c_int;
            nearestDir = 8 as libc::c_int;
            dir = 0 as libc::c_int;
            while dir < 8 as libc::c_int {
                x = (startX >> 7 as libc::c_int) + aDirOffset[dir as usize].x;
                y = (startY >> 7 as libc::c_int) + aDirOffset[dir as usize].y;
                if fpathBlockingTile.expect("non-null function pointer")(x, y)
                       == 0 {
                    tileDist = fpathDistToTile(x, y, startX, startY);
                    if tileDist < minDist {
                        minDist = tileDist;
                        nearestDir = dir
                    }
                }
                dir += 1
            }
            if nearestDir == 8 as libc::c_int {
                // surrounded by blocking tiles, give up
                retVal = FPR_FAILED;
                current_block = 9912211644625474985;
            } else {
                startX =
                    ((startX >> 7 as libc::c_int) +
                         aDirOffset[nearestDir as usize].x <<
                         7 as libc::c_int) +
                        7 as libc::c_int / 2 as libc::c_int;
                startY =
                    ((startY >> 7 as libc::c_int) +
                         aDirOffset[nearestDir as usize].y <<
                         7 as libc::c_int) +
                        7 as libc::c_int / 2 as libc::c_int;
                current_block = 6545907279487748450;
            }
        } else { current_block = 6545907279487748450; }
        match current_block {
            9912211644625474985 => { }
            _ => {
                // initialise the raycast - if there is los to the target, no routing necessary
                finalX = targetX & !(0x7f as libc::c_int);
                finalX += 128 as libc::c_int / 2 as libc::c_int;
                finalY = targetY & !(0x7f as libc::c_int);
                finalY += 128 as libc::c_int / 2 as libc::c_int;
                clearX = finalX;
                clearY = finalY;
                vectorX = startX - finalX;
                vectorY = startY - finalY;
                obstruction = 0 as libc::c_int;
                // cast the ray to find the last clear tile before the obstruction
                rayCast(startX as UDWORD, startY as UDWORD,
                        rayPointsToAngle(startX, startY, finalX, finalY),
                        0x7ffff as libc::c_int as UDWORD,
                        Some(fpathEndPointCallback as
                                 unsafe extern "C" fn(_: SDWORD, _: SDWORD,
                                                      _: SDWORD) -> BOOL));
                if obstruction == 0 {
                    // no obstructions - trivial route
                    fpathSetDirectRoute(psObj, targetX, targetY);
                    retVal = FPR_OK;
                    !psPartialRouteObj.is_null();
                    current_block = 9912211644625474985;
                } else {
                    // check whether the end point of the route
		// is a blocking tile and find an alternative if it is
                    if fpathBlockingTile.expect("non-null function pointer")(targetX
                                                                                 >>
                                                                                 7
                                                                                     as
                                                                                     libc::c_int,
                                                                             targetY
                                                                                 >>
                                                                                 7
                                                                                     as
                                                                                     libc::c_int)
                           != 0 {
                        // route to the last clear tile found by the raycast
                        targetX = clearX;
                        targetY = clearY
                    }
                    // see if there is another unit with a usable route
                    if fpathFindRoute(psDroid, startX, startY, targetX,
                                      targetY) != 0 {
                        !psPartialRouteObj.is_null();
                        current_block = 9912211644625474985;
                    } else { current_block = 5207889489643863322; }
                }
            }
        }
    } else { current_block = 5207889489643863322; }
    match current_block {
        5207889489643863322 => {
            if startX >= 0 as libc::c_int &&
                   startX < mapWidth as SDWORD * 128 as libc::c_int &&
                   startY >= 0 as libc::c_int &&
                   startY < mapHeight as SDWORD * 128 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"fpathRoute: start coords off map\x00" as *const u8 as
                          *const libc::c_char);
            };
            if startX >= 0 as libc::c_int &&
                   startX < mapWidth as SDWORD * 128 as libc::c_int &&
                   startY >= 0 as libc::c_int &&
                   startY < mapHeight as SDWORD * 128 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"fpath.c\x00" as *const u8 as *const libc::c_char,
                      1489 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"fpathRoute\x00")).as_ptr(),
                      b"startX >= 0 && startX < (SDWORD)mapWidth*TILE_UNITS && startY >= 0 && startY < (SDWORD)mapHeight*TILE_UNITS\x00"
                          as *const u8 as *const libc::c_char);
            };
            if targetX >= 0 as libc::c_int &&
                   targetX < mapWidth as SDWORD * 128 as libc::c_int &&
                   targetY >= 0 as libc::c_int &&
                   targetY < mapHeight as SDWORD * 128 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"fpathRoute: target coords off map\x00" as *const u8 as
                          *const libc::c_char);
            };
            if targetX >= 0 as libc::c_int &&
                   targetX < mapWidth as SDWORD * 128 as libc::c_int &&
                   targetY >= 0 as libc::c_int &&
                   targetY < mapHeight as SDWORD * 128 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"fpath.c\x00" as *const u8 as *const libc::c_char,
                      1492 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"fpathRoute\x00")).as_ptr(),
                      b"targetX >= 0 && targetX < (SDWORD)mapWidth*TILE_UNITS && targetY >= 0 && targetY < (SDWORD)mapHeight*TILE_UNITS\x00"
                          as *const u8 as *const libc::c_char);
            };
            if fpathBlockingTile ==
                   Some(fpathGroundBlockingTile as
                            unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                -> BOOL) ||
                   fpathBlockingTile ==
                       Some(fpathHoverBlockingTile as
                                unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                    -> BOOL) ||
                   fpathBlockingTile ==
                       Some(fpathLiftBlockingTile as
                                unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                    -> BOOL) {
            } else {
                debug(LOG_ERROR,
                      b"fpathRoute: invalid blocking function\x00" as
                          *const u8 as *const libc::c_char);
            };
            if fpathBlockingTile ==
                   Some(fpathGroundBlockingTile as
                            unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                -> BOOL) ||
                   fpathBlockingTile ==
                       Some(fpathHoverBlockingTile as
                                unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                    -> BOOL) ||
                   fpathBlockingTile ==
                       Some(fpathLiftBlockingTile as
                                unsafe extern "C" fn(_: SDWORD, _: SDWORD)
                                    -> BOOL) {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"fpath.c\x00" as *const u8 as *const libc::c_char,
                      1496 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"fpathRoute\x00")).as_ptr(),
                      b"fpathBlockingTile == fpathGroundBlockingTile || fpathBlockingTile == fpathHoverBlockingTile || fpathBlockingTile == fpathLiftBlockingTile\x00"
                          as *const u8 as *const libc::c_char);
            };
            if astarInner > astarMaxRoute {
                if psPartialRouteObj == psObj {
                    retVal = FPR_WAIT
                } else { retVal = FPR_RESCHEDULE }
            } else if !psPartialRouteObj.is_null() &&
                          psPartialRouteObj != psObj ||
                          psPartialRouteObj != psObj &&
                              !psNextRouteDroid.is_null() &&
                              psNextRouteDroid != psObj as *mut DROID {
                retVal = FPR_RESCHEDULE
            } else {
                if psPartialRouteObj.is_null() {
                    retVal =
                        fpathGatewayRoute(psObj, ASR_NEWROUTE as libc::c_int,
                                          GWTerrain as SDWORD, startX, startY,
                                          targetX, targetY, psMoveCntl) as
                            FPATH_RETVAL
                } else {
                    //		DBPRINTF(("Partial Route: %d\n", psDroid->id));
                    psPartialRouteObj = 0 as *mut BASE_OBJECT;
                    retVal =
                        fpathGatewayRoute(psObj, ASR_CONTINUE as libc::c_int,
                                          GWTerrain as SDWORD, startX, startY,
                                          targetX, targetY, psMoveCntl) as
                            FPATH_RETVAL
                }
                if retVal as libc::c_uint ==
                       FPR_WAIT as libc::c_int as libc::c_uint {
                    psPartialRouteObj = psObj;
                    lastPartialFrame = frameGetFrameNumber() as SDWORD;
                    partialSX = startX;
                    partialSY = startY;
                    partialTX = targetX;
                    partialTY = targetY
                } else if retVal as libc::c_uint ==
                              FPR_FAILED as libc::c_int as libc::c_uint &&
                              (*psObj).type_0 as libc::c_uint ==
                                  OBJ_DROID as libc::c_int as libc::c_uint &&
                              vtolDroid(psObj as *mut DROID) != 0 {
                    fpathSetDirectRoute(psObj, targetX, targetY);
                    retVal = FPR_OK
                }
            }
        }
        _ => { }
    }
    // reset all the droid flags
/*	for(psCurr = apsDroidLists[psObj->player]; psCurr; psCurr = psCurr->psNext)
	{
		if (psCurr->sMove.Status == MOVEINACTIVE)
		{
			psTile = mapTile(psCurr->x >> TILE_SHIFT, psCurr->y >> TILE_SHIFT);
			psTile->tileInfoBits &= ~BITS_FPATHBLOCK;
		}
	}*/
    // reset the blocking tile function
    fpathBlockingTile =
        Some(fpathGroundBlockingTile as
                 unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
    /* reset global pointer for object being routed */
    fpathSetCurrentObject(0 as *mut BASE_OBJECT);
    return retVal;
}
// find the first point on the route which has both droids on the same side of it
#[no_mangle]
pub unsafe extern "C" fn fpathFindFirstRoutePoint(mut psMove:
                                                      *mut MOVE_CONTROL,
                                                  mut pIndex: *mut SDWORD,
                                                  mut x1: SDWORD,
                                                  mut y1: SDWORD,
                                                  mut x2: SDWORD,
                                                  mut y2: SDWORD) -> BOOL {
    let mut vx1: SDWORD = 0;
    let mut vy1: SDWORD = 0;
    let mut vx2: SDWORD = 0;
    let mut vy2: SDWORD = 0;
    *pIndex = 0 as libc::c_int;
    while *pIndex < (*psMove).numPoints as libc::c_int {
        vx1 = x1 - (*psMove).asPath[*pIndex as usize].x as libc::c_int;
        vy1 = y1 - (*psMove).asPath[*pIndex as usize].y as libc::c_int;
        vx2 = x2 - (*psMove).asPath[*pIndex as usize].x as libc::c_int;
        vy2 = y2 - (*psMove).asPath[*pIndex as usize].y as libc::c_int;
        // found it if the dot products have the same sign
        if vx1 * vx2 + vy1 * vy2 < 0 as libc::c_int {
            return 1 as libc::c_int
        }
        *pIndex += 1
    }
    return 0 as libc::c_int;
}
// See if there is another unit on your side that has a route this unit can use
#[no_mangle]
pub unsafe extern "C" fn fpathFindRoute(mut psDroid: *mut DROID,
                                        mut sX: SDWORD, mut sY: SDWORD,
                                        mut tX: SDWORD, mut tY: SDWORD)
 -> BOOL {
    let mut psFormation: *mut FORMATION = 0 as *mut FORMATION;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut i: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut index: SDWORD = 0;
    if formationFind(&mut psFormation, tX, tY) == 0 {
        return 0 as libc::c_int
    }
    // now look for a unit in this formation with a route that can be used
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if psCurr != psDroid && psCurr != psPartialRouteObj as *mut DROID &&
               (*psCurr).sMove.psFormation == psFormation &&
               (*psCurr).sMove.numPoints as libc::c_int > 0 as libc::c_int {
            // find the first route point
            if !(fpathFindFirstRoutePoint(&mut (*psCurr).sMove, &mut index,
                                          sX, sY, (*psCurr).x as SDWORD,
                                          (*psCurr).y as SDWORD) == 0) {
                // initialise the raycast - if there is los to the start of the route
                startX =
                    (sX & !(0x7f as libc::c_int)) +
                        128 as libc::c_int / 2 as libc::c_int;
                startY =
                    (sY & !(0x7f as libc::c_int)) +
                        128 as libc::c_int / 2 as libc::c_int;
                finalX =
                    (*psCurr).sMove.asPath[index as usize].x as libc::c_int *
                        128 as libc::c_int +
                        128 as libc::c_int / 2 as libc::c_int;
                finalY =
                    (*psCurr).sMove.asPath[index as usize].y as libc::c_int *
                        128 as libc::c_int +
                        128 as libc::c_int / 2 as libc::c_int;
                clearX = finalX;
                clearY = finalY;
                vectorX = startX - finalX;
                vectorY = startY - finalY;
                obstruction = 0 as libc::c_int;
                // cast the ray to find the last clear tile before the obstruction
                rayCast(startX as UDWORD, startY as UDWORD,
                        rayPointsToAngle(startX, startY, finalX, finalY),
                        0x7ffff as libc::c_int as UDWORD,
                        Some(fpathEndPointCallback as
                                 unsafe extern "C" fn(_: SDWORD, _: SDWORD,
                                                      _: SDWORD) -> BOOL));
                if obstruction == 0 {
                    // This route is OK, copy it over
                    i = index;
                    while i < (*psCurr).sMove.numPoints as libc::c_int {
                        (*psDroid).sMove.asPath[i as usize] =
                            (*psCurr).sMove.asPath[i as usize];
                        i += 1
                    }
                    (*psDroid).sMove.numPoints = (*psCurr).sMove.numPoints;
                    // now see if the route
                    return 1 as libc::c_int
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as libc::c_int;
}
