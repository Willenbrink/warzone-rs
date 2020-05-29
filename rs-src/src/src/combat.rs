use ::libc;
extern "C" {
    pub type _formation;
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
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn labs(_: libc::c_long) -> libc::c_long;
    #[no_mangle]
    fn getDroidLevel(psDroid: *mut DROID) -> UDWORD;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn cbSensorDroid(psDroid: *mut DROID) -> BOOL;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    /*checks if the structure has a Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a VTOL Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structVTOLCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*set of rules which determine whether the weapon associated with the object
can fire on the propulsion type of the target*/
    #[no_mangle]
    fn validTarget(psObject: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    fn weaponFirePause(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponShortHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponLongHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    /* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
    #[no_mangle]
    fn visibleObject(psViewer: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    // Do visibility check, but with walls completely blocking LOS.
    #[no_mangle]
    fn visibleObjWallBlock(psViewer: *mut BASE_OBJECT,
                           psTarget: *mut BASE_OBJECT) -> BOOL;
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
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn proj_SendProjectile(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                           player: SDWORD, tarX: UDWORD, tarY: UDWORD,
                           tarZ: UDWORD, psTarget: *mut BASE_OBJECT,
                           bVisible: BOOL) -> BOOL;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    // get the level of a droids commander, if any
    #[no_mangle]
    fn cmdGetCommanderLevel(psDroid: *mut DROID) -> SDWORD;
    // initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
    #[no_mangle]
    fn gridStartIterate(x: SDWORD, y: SDWORD);
    // get the next object that could affect a location,
// should only be called after gridStartIterate
    #[no_mangle]
    fn gridIterate() -> *mut BASE_OBJECT;
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    // check if a target is inside minimum weapon range
    #[no_mangle]
    fn actionInsideMinRange(psDroid: *mut DROID, psObj: *mut BASE_OBJECT)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
pub type _component_type = libc::c_uint;
pub const COMP_NUMCOMPONENTS: _component_type = 9;
pub const COMP_WEAPON: _component_type = 8;
pub const COMP_CONSTRUCT: _component_type = 7;
pub const COMP_SENSOR: _component_type = 6;
pub const COMP_ECM: _component_type = 5;
pub const COMP_REPAIRUNIT: _component_type = 4;
pub const COMP_PROPULSION: _component_type = 3;
pub const COMP_BRAIN: _component_type = 2;
pub const COMP_BODY: _component_type = 1;
pub const COMP_UNKNOWN: _component_type = 0;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
//ALL components and structures and research topics have a tech level to which they belong
pub type TECH_LEVEL = _tech_level;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
//only using KINETIC and HEAT for now
pub type WEAPON_CLASS = _weapon_class;
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
pub const WSC_HOWITZERS: _weapon_subclass = 8;
pub const WSC_FLAME: _weapon_subclass = 7;
pub const WSC_GAUSS: _weapon_subclass = 6;
pub const WSC_ENERGY: _weapon_subclass = 5;
pub const WSC_ROCKET: _weapon_subclass = 4;
pub const WSC_MISSILE: _weapon_subclass = 3;
pub const WSC_MORTARS: _weapon_subclass = 2;
pub const WSC_CANNON: _weapon_subclass = 1;
pub const WSC_MGUN: _weapon_subclass = 0;
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
pub type WEAPON_SUBCLASS = _weapon_subclass;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
// used to define which projectile model to use for the weapon
pub type MOVEMENT_MODEL = _movement_model;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
//used to modify the damage to a propuslion type (or structure) based on weapon
pub type WEAPON_EFFECT = _weapon_effect;
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
pub type SENSOR_STATS = _sensor_stats;
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
pub type WEAPON_STATS = _weapon_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
//works as all of the above together! - new for updates - added 11/06/99 AB
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
pub type BASE_OBJECT = _base_object;
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
pub type STRUCTURE = _structure;
/* direction array for missed bullets */
pub type BUL_DIR = _bul_dir;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bul_dir {
    pub x: SDWORD,
    pub y: SDWORD,
}
/*
 * Order.h
 *
 * Function prototypes for giving droids orders
 *
 */
//turn off the build queue availability until desired release date!
//#define DISABLE_BUILD_QUEUE
// The droid orders
pub type DROID_ORDER = _droid_order;
pub type _droid_order = libc::c_uint;
pub const DORDER_RTR_SPECIFIED: _droid_order = 37;
pub const DORDER_LEAVEMAP: _droid_order = 36;
pub const DORDER_RECOVER: _droid_order = 35;
pub const DORDER_SCOUT_ATTACKWALL: _droid_order = 34;
pub const DORDER_MOVE_ATTACKWALL: _droid_order = 33;
pub const DORDER_REARM: _droid_order = 32;
pub const DORDER_PATROL: _droid_order = 31;
pub const DORDER_CLEARWRECK: _droid_order = 30;
pub const DORDER_RUNBURN: _droid_order = 29;
pub const DORDER_SCOUT: _droid_order = 28;
pub const DORDER_RESTORE: _droid_order = 27;
pub const DORDER_DROIDREPAIR: _droid_order = 26;
pub const DORDER_GUARD: _droid_order = 25;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
pub const DORDER_RECYCLE: _droid_order = 21;
pub const DORDER_BUILDMODULE: _droid_order = 20;
pub const DORDER_COMMAND: _droid_order = 19;
pub const DORDER_ATTACKTARGET: _droid_order = 18;
pub const DORDER_DISEMBARK: _droid_order = 17;
pub const DORDER_EMBARK: _droid_order = 16;
pub const DORDER_RUN: _droid_order = 15;
pub const DORDER_RTR: _droid_order = 14;
pub const DORDER_RTB: _droid_order = 13;
pub const DORDER_DESTRUCT: _droid_order = 12;
pub const DORDER_RETREAT: _droid_order = 11;
pub const DORDER_FIRESUPPORT: _droid_order = 10;
pub const DORDER_OBSERVE: _droid_order = 9;
pub const DORDER_REPAIR: _droid_order = 8;
pub const DORDER_DEMOLISH: _droid_order = 7;
pub const DORDER_LINEBUILD: _droid_order = 6;
pub const DORDER_HELPBUILD: _droid_order = 5;
pub const DORDER_BUILD: _droid_order = 4;
pub const DORDER_ATTACK: _droid_order = 3;
pub const DORDER_MOVE: _droid_order = 2;
pub const DORDER_STOP: _droid_order = 1;
pub const DORDER_NONE: _droid_order = 0;
static mut aScatterDir: [BUL_DIR; 8] =
    [{
         let mut init =
             _bul_dir{x: 0 as libc::c_int, y: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             _bul_dir{x: 1 as libc::c_int, y: -(1 as libc::c_int),};
         init
     },
     {
         let mut init = _bul_dir{x: 1 as libc::c_int, y: 0 as libc::c_int,};
         init
     },
     {
         let mut init = _bul_dir{x: 1 as libc::c_int, y: 1 as libc::c_int,};
         init
     },
     {
         let mut init = _bul_dir{x: 0 as libc::c_int, y: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             _bul_dir{x: -(1 as libc::c_int), y: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             _bul_dir{x: -(1 as libc::c_int), y: 0 as libc::c_int,};
         init
     },
     {
         let mut init =
             _bul_dir{x: -(1 as libc::c_int), y: -(1 as libc::c_int),};
         init
     }];
// no order set
// stop the current order
// 2 - move to a location
// attack an enemy
// 4 - build a structure
// help to build a structure
// 6 - build a number of structures in a row (walls + bridges)
// demolish a structure
// 8 - repair a structure
// keep a target in sensor view
// 10 - attack whatever the linked sensor droid attacks
// return to the players retreat position
// 12 - self destruct
// return to base
// 14 - return to repair at any repair facility
// run away after moral failure
// 16 - board a transporter
// get off a transporter
// 18 - a suggestion to attack something
// i.e. the target was chosen because the droid could see it
// a command droid issuing orders to it's group
// 20 - build a module (power, research or factory)
// return to factory to be recycled
// 22 - offworld transporter order
// onworld transporter order
// 24 - transporter return after unloading
// guard a structure
// 26 - repair a droid
// restore resistance points for a structure
// 28 - same as move, but stop if an enemy is seen
// run away on fire
// 30 - constructor droid to clear up building wreckage
// move between two way points
// 32 - order a vtol to rearming pad
// move to a location taking out a blocking wall on the way
// 34 - scout to a location taking out a blocking wall on the way
// pick up an artifact
// 36 - vtol flying off the map
// return to repair at a specified repair center
/* Initialise the combat system */
/* Initialise the combat system */
#[no_mangle]
pub unsafe extern "C" fn combInitialise() -> BOOL { return 1 as libc::c_int; }
/* Shutdown the combat system */
/* Shutdown the combat system */
#[no_mangle]
pub unsafe extern "C" fn combShutdown() -> BOOL { return 1 as libc::c_int; }
/* Fire a weapon at something */
/* Fire a weapon at something */
#[no_mangle]
pub unsafe extern "C" fn combFire(mut psWeap: *mut WEAPON,
                                  mut psAttacker: *mut BASE_OBJECT,
                                  mut psTarget: *mut BASE_OBJECT) {
    let mut current_block: u64;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut xDiff: UDWORD = 0;
    let mut yDiff: UDWORD = 0;
    let mut distSquared: UDWORD = 0;
    let mut dice: UDWORD = 0;
    let mut damLevel: UDWORD = 0;
    let mut missDir: SDWORD = 0;
    let mut missDist: SDWORD = 0;
    let mut missX: SDWORD = 0;
    let mut missY: SDWORD = 0;
    let mut hitMod: SDWORD = 0;
    let mut hitInc: SDWORD = 0;
    let mut fireChance: SDWORD = 0;
    let mut firePause: UDWORD = 0;
    let mut targetDir: SDWORD = 0;
    let mut dirDiff: SDWORD = 0;
    let mut longRange: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut level: SDWORD = 0;
    let mut cmdLevel: SDWORD = 0;
    let mut bMissVisible: BOOL = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"combFire: Invalid weapon pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"combat.c\x00" as *const u8 as *const libc::c_char,
              104 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"combFire\x00")).as_ptr(),
              b"PTRVALID(psWeap, sizeof(WEAPON))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"combFire: Invalid attacker pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"combat.c\x00" as *const u8 as *const libc::c_char,
              106 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"combFire\x00")).as_ptr(),
              b"PTRVALID(psAttacker, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"combFire: Invalid target pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"combat.c\x00" as *const u8 as *const libc::c_char,
              108 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"combFire\x00")).as_ptr(),
              b"PTRVALID(psTarget, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Get the stats for the weapon */
    psStats = asWeaponStats.offset((*psWeap).nStat as isize);
    //check valid weapon/prop combination
    if validTarget(psAttacker, psTarget) == 0 { return }
    /*see if reload-able weapon and out of ammo*/
    if (*psStats).reloadTime != 0 && (*psWeap).ammo == 0 {
        if gameTime.wrapping_sub((*psWeap).lastFired) < (*psStats).reloadTime
           {
            return
        }
        //reset the ammo level
        (*psWeap).ammo = (*psStats).numRounds as UDWORD
    }
    /* See when the weapon last fired to control it's rate of fire */
    firePause = weaponFirePause(psStats, (*psAttacker).player);
    // increase the pause if heavily damaged
    match (*psAttacker).type_0 as libc::c_uint {
        0 => {
            psDroid = psAttacker as *mut DROID;
            damLevel =
                (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                 libc::c_uint).wrapping_div((*psDroid).originalBody)
        }
        1 => {
            damLevel =
                (((*(psAttacker as *mut STRUCTURE)).body as libc::c_int *
                      100 as libc::c_int) as
                     libc::c_uint).wrapping_div(structureBody(psAttacker as
                                                                  *mut STRUCTURE))
        }
        _ => { damLevel = 100 as libc::c_int as UDWORD }
    }
    if damLevel < 25 as libc::c_int as libc::c_uint {
        firePause =
            (firePause as libc::c_uint).wrapping_add(firePause) as UDWORD as
                UDWORD
    }
    if gameTime.wrapping_sub((*psWeap).lastFired) <= firePause {
        /* Too soon to fire again */
        return
    }
    // add a random delay to the fire
    fireChance =
        gameTime.wrapping_sub((*psWeap).lastFired.wrapping_add(firePause)) as
            SDWORD;
    if rand() % 500 as libc::c_int > fireChance { return }
    /* Check we can see the target */
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint &&
           vtolDroid(psAttacker as *mut DROID) == 0 &&
           (proj_Direct(psStats) != 0 ||
                actionInsideMinRange(psDroid, (*psDroid).psActionTarget) != 0)
       {
        if visibleObjWallBlock(psAttacker, psTarget) == 0 {
            // Can't see the target - can't hit it with direct fire
            return
        }
    } else if (*psAttacker).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                  (*(*(psAttacker as *mut STRUCTURE)).pStructureType).height
                      == 1 as libc::c_int as libc::c_uint &&
                  proj_Direct(psStats) != 0 {
        // a bunker can't shoot through walls
        if visibleObjWallBlock(psAttacker, psTarget) == 0 {
            // Can't see the target - can't hit it with direct fire
            return
        }
    } else if proj_Direct(psStats) != 0 {
        if visibleObject(psAttacker, psTarget) == 0 {
            // Can't see the target - can't hit it with direct fire
            return
        }
    } else if (*psTarget).visible[(*psAttacker).player as usize] == 0 {
        // Can't get an indirect LOS - can't hit it with the weapon
        return
    }
    /*	if ( proj_Direct(psStats) ||
		 ((psAttacker->type == OBJ_DROID) &&
		  !proj_Direct(psStats) &&
		   actionInsideMinRange(psDroid, psDroid->psActionTarget)) )
	{
		switch (psAttacker->type)
		{
		case OBJ_DROID:
			if (!visibleObjWallBlock(psAttacker, psTarget))
			{
				// Can't see the target - can't hit it with direct fire
				DBP3(("directLOS failed\n"));
				return;
			}
			break;
		default:
			if (!visibleObject(psAttacker, psTarget))
			{
				// Can't see the target - can't hit it with direct fire
				DBP3(("directLOS failed\n"));
				return;
			}
			break;
		}
	}
	else
	{
		if (!psTarget->visible[psAttacker->player])
		{
			// Can't get an indirect LOS - can't hit it with the weapon
			DBP3(("indirectLOS failed\n"));
			return;
		}
	}*/
    // if the turret doesn't turn, check the attacker is in alignment with the
	// target
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint && (*psStats).rotate == 0
       {
        targetDir =
            calcDirection((*psAttacker).x as UDWORD,
                          (*psAttacker).y as UDWORD, (*psTarget).x as UDWORD,
                          (*psTarget).y as UDWORD);
        dirDiff =
            labs((targetDir - (*psAttacker).direction as SDWORD) as
                     libc::c_long) as SDWORD;
        if dirDiff > 1 as libc::c_int { return }
    }
    // base modifier 100% of original
    hitMod = 100 as libc::c_int;
    // base hit increment of zero
    hitInc = 0 as libc::c_int;
    // apply upgrades - do these when know if its longHit or shortHit
	//hitMod = hitMod * (asWeaponUpgrade[psAttacker->player]
	//							[psStats->weaponSubClass].shortHit + 100) / 100;
    // add the attackers experience modifier
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        //		hitMod = hitMod + (hitMod * 5) * getDroidLevel((DROID *)psAttacker) / 100;
//		hitMod = hitMod + hitMod * cmdDroidHitMod((DROID *)psAttacker) / 100;
        level = getDroidLevel(psAttacker as *mut DROID) as SDWORD;
        cmdLevel = cmdGetCommanderLevel(psAttacker as *mut DROID);
        if level > cmdLevel {
            hitInc += 5 as libc::c_int * level
        } else { hitInc += 5 as libc::c_int * cmdLevel }
    }
    // subtract the defenders experience modifier
/*	if (psTarget->type == OBJ_DROID)
	{
//		hitMod = hitMod - hitMod * 2 * getDroidLevel((DROID *)psTarget) / 100;
//		hitMod = hitMod - hitMod * cmdDroidEvasionMod((DROID *)psTarget) / 100;
		level = getDroidLevel((DROID *)psTarget);
		cmdLevel = cmdGetCommanderLevel((DROID *)psTarget);
		if (level > cmdLevel)
		{
			hitInc -= 5 * level;
		}
		else
		{
			hitInc -= 5 * cmdLevel;
		}
	}*/
    // fire while moving modifiers
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint &&
           (*(psAttacker as *mut DROID)).sMove.Status as libc::c_int !=
               0 as libc::c_int {
        match (*psStats).fireOnMove as libc::c_uint {
            0 => {
                // Can't fire while moving
                return
            }
            1 => { hitMod = 50 as libc::c_int * hitMod / 100 as libc::c_int }
            2 | _ => { }
        }
    }
    // visibility modifiers
    if ((*psTarget).visible[(*psAttacker).player as usize] as libc::c_int) <
           150 as libc::c_int {
        hitMod = 50 as libc::c_int * hitMod / 100 as libc::c_int
    }
    /* Now see if the target is in range  - also check not too near*/
    xDiff =
        abs((*psAttacker).x as libc::c_int - (*psTarget).x as libc::c_int) as
            UDWORD;
    yDiff =
        abs((*psAttacker).y as libc::c_int - (*psTarget).y as libc::c_int) as
            UDWORD;
    distSquared =
        xDiff.wrapping_mul(xDiff).wrapping_add(yDiff.wrapping_mul(yDiff));
    longRange =
        proj_GetLongRange(psStats,
                          (*psAttacker).z as SDWORD -
                              (*psTarget).z as SDWORD);
    if distSquared <=
           (*psStats).shortRange.wrapping_mul((*psStats).shortRange) &&
           distSquared >=
               (*psStats).minRange.wrapping_mul((*psStats).minRange) {
        /* note when the weapon fired */
        (*psWeap).lastFired = gameTime;
        /*reduce ammo if salvo*/
        if (*psStats).reloadTime != 0 {
            (*psWeap).ammo = (*psWeap).ammo.wrapping_sub(1)
        }
        /* Can do a short range shot - see if it hits */
		/* ***********************************************/
		/* NEED TO TAKE ACCOUNT OF ECM, BODY SHAPE ETC. */
		/* ***********************************************/
        dice = (rand() % 100 as libc::c_int) as UDWORD;
        //if (dice <= psStats->shortHit * hitMod /100)
        if dice <=
               weaponShortHit(psStats,
                              (*psAttacker).player).wrapping_mul(hitMod as
                                                                     libc::c_uint).wrapping_div(100
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_add(hitInc
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
           {
            /* Kerrrbaaang !!!!! a hit */
            if proj_SendProjectile(psWeap, psAttacker,
                                   (*psAttacker).player as SDWORD,
                                   (*psTarget).x as UDWORD,
                                   (*psTarget).y as UDWORD,
                                   (*psTarget).z as UDWORD, psTarget,
                                   0 as libc::c_int) == 0 {
                /* Out of memory - we can safely ignore this */
                return
            }
            current_block = 3951782611973701285;
        } else { current_block = 14652688882591975137; }
    } else if distSquared as SDWORD <= longRange * longRange &&
                  (distSquared >=
                       (*psStats).minRange.wrapping_mul((*psStats).minRange)
                       ||
                       (*psAttacker).type_0 as libc::c_uint ==
                           OBJ_DROID as libc::c_int as libc::c_uint &&
                           proj_Direct(psStats) == 0 &&
                           actionInsideMinRange(psDroid,
                                                (*psDroid).psActionTarget) !=
                               0) {
        /* note when the weapon fired */
        (*psWeap).lastFired = gameTime;
        /*reduce ammo if salvo*/
        if (*psStats).reloadTime != 0 {
            (*psWeap).ammo = (*psWeap).ammo.wrapping_sub(1)
        }
        /* Can do a long range shot - see if it hits */
		/* ***********************************************/
		/* NEED TO TAKE ACCOUNT OF ECM, BODY SHAPE ETC. */
		/* ***********************************************/
        dice = (rand() % 100 as libc::c_int) as UDWORD;
        //if (dice <= psStats->longHit * hitMod /100)
        if dice <=
               weaponLongHit(psStats,
                             (*psAttacker).player).wrapping_mul(hitMod as
                                                                    libc::c_uint).wrapping_div(100
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_add(hitInc
                                                                                                                                  as
                                                                                                                                  libc::c_uint)
           {
            /* Kerrrbaaang !!!!! a hit */
            if proj_SendProjectile(psWeap, psAttacker,
                                   (*psAttacker).player as SDWORD,
                                   (*psTarget).x as UDWORD,
                                   (*psTarget).y as UDWORD,
                                   (*psTarget).z as UDWORD, psTarget,
                                   0 as libc::c_int) == 0 {
                /* Out of memory - we can safely ignore this */
                return
            }
            current_block = 3951782611973701285;
        } else { current_block = 14652688882591975137; }
    } else {
        /* Out of range */
        return
    }
    match current_block {
        3951782611973701285 => { return }
        _ =>
        /* Deal with a missed shot */
        // Approximate the distance between the attacker and target
        {
            xDiff =
                if (*psAttacker).x as libc::c_int >
                       (*psTarget).x as libc::c_int {
                    ((*psAttacker).x as libc::c_int) -
                        (*psTarget).x as libc::c_int
                } else {
                    ((*psTarget).x as libc::c_int) -
                        (*psAttacker).x as libc::c_int
                } as UDWORD;
            yDiff =
                if (*psAttacker).y as libc::c_int >
                       (*psTarget).y as libc::c_int {
                    ((*psAttacker).y as libc::c_int) -
                        (*psTarget).y as libc::c_int
                } else {
                    ((*psTarget).y as libc::c_int) -
                        (*psAttacker).y as libc::c_int
                } as UDWORD;
            missDist =
                if xDiff > yDiff {
                    xDiff.wrapping_add(yDiff >> 1 as libc::c_int)
                } else { yDiff.wrapping_add(xDiff >> 1 as libc::c_int) } as
                    SDWORD;
            // Calculate where the shot will end up
            missDist = missDist >> 2 as libc::c_int;
            if missDist < 128 as libc::c_int / 6 as libc::c_int {
                missDist = 128 as libc::c_int / 6 as libc::c_int
            }
            missDir = rand() % 8 as libc::c_int;
            missX =
                aScatterDir[missDir as usize].x * (rand() % missDist) +
                    (*psTarget).x as libc::c_int;
            missY =
                aScatterDir[missDir as usize].y * (rand() % missDist) +
                    (*psTarget).y as libc::c_int;
            // decide if a miss is visible
            bMissVisible = 0 as libc::c_int;
            if (*psTarget).player as libc::c_uint == selectedPlayer {
                bMissVisible = 1 as libc::c_int
            }
            /* Fire off the bullet to the miss location */
            if proj_SendProjectile(psWeap, psAttacker,
                                   (*psAttacker).player as SDWORD,
                                   missX as UDWORD, missY as UDWORD,
                                   (*psTarget).z as UDWORD,
                                   0 as *mut BASE_OBJECT, bMissVisible) == 0 {
                /* Out of memory */
                return
            }
            return
        }
    };
}
/*checks through the target players list of structures and droids to see
if any support a counter battery sensor*/
/*checks through the target players list of structures and droids to see
if any support a counter battery sensor*/
#[no_mangle]
pub unsafe extern "C" fn counterBatteryFire(mut psAttacker: *mut BASE_OBJECT,
                                            mut psTarget: *mut BASE_OBJECT) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psViewer: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut sensorRange: SDWORD = 0;
    let mut xDiff: SDWORD = 0;
    let mut yDiff: SDWORD = 0;
    /*if a null target is passed in ignore - this will be the case when a 'miss'
	projectile is sent - we may have to cater for these at some point*/
	// also ignore cases where you attack your own player
    if psTarget.is_null() ||
           !psAttacker.is_null() &&
               (*psAttacker).player as libc::c_int ==
                   (*psTarget).player as libc::c_int {
        return
    }
    gridStartIterate((*psTarget).x as SDWORD, (*psTarget).y as SDWORD);
    psViewer = gridIterate();
    while !psViewer.is_null() {
        if !((*psViewer).player as libc::c_int !=
                 (*psTarget).player as libc::c_int) {
            sensorRange = 0 as libc::c_int;
            if (*psViewer).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                psStruct = psViewer as *mut STRUCTURE;
                //check if have a sensor of correct type
                if structCBSensor(psStruct) != 0 ||
                       structVTOLCBSensor(psStruct) != 0 {
                    sensorRange =
                        (*(*(*psStruct).pStructureType).pSensor).range as
                            SDWORD
                }
            } else if (*psViewer).type_0 as libc::c_uint ==
                          OBJ_DROID as libc::c_int as libc::c_uint {
                psDroid = psViewer as *mut DROID;
                //must be a CB sensor
			/*if (asSensorStats[psDroid->asBits[COMP_SENSOR].nStat].type ==
				INDIRECT_CB_SENSOR OR asSensorStats[psDroid->asBits[COMP_SENSOR].
				nStat].type == VTOL_CB_SENSOR)*/
                if cbSensorDroid(psDroid) != 0 {
                    sensorRange =
                        (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     usize].nStat
                                                   as isize)).range as SDWORD
                }
            }
            //check sensor distance from target
            if sensorRange != 0 {
                xDiff = (*psViewer).x as SDWORD - (*psTarget).x as SDWORD;
                yDiff = (*psViewer).y as SDWORD - (*psTarget).y as SDWORD;
                if xDiff * xDiff + yDiff * yDiff < sensorRange * sensorRange {
                    //inform viewer of target
                    if (*psViewer).type_0 as libc::c_uint ==
                           OBJ_DROID as libc::c_int as libc::c_uint {
                        orderDroidObj(psViewer as *mut DROID, DORDER_OBSERVE,
                                      psAttacker);
                    } else if (*psViewer).type_0 as libc::c_uint ==
                                  OBJ_STRUCTURE as libc::c_int as libc::c_uint
                     {
                        let ref mut fresh0 =
                            (*(psViewer as *mut STRUCTURE)).psTarget;
                        *fresh0 = psAttacker
                    }
                }
            }
        }
        //ignore non target players' objects
        psViewer = gridIterate()
    };
}
