use ::libc;
extern "C" {
    pub type _formation;
    /* Read formatted input from S.  */
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    //used to hold the current upgrade level per player per weapon subclass
    #[no_mangle]
    static mut asWeaponUpgrade: [[WEAPON_UPGRADE; 17]; 8];
    #[no_mangle]
    static mut asSensorUpgrade: [SENSOR_UPGRADE; 8];
    #[no_mangle]
    static mut asECMUpgrade: [ECM_UPGRADE; 8];
    #[no_mangle]
    static mut asRepairUpgrade: [REPAIR_UPGRADE; 8];
    #[no_mangle]
    static mut asConstUpgrade: [CONSTRUCTOR_UPGRADE; 8];
    #[no_mangle]
    static mut asBodyUpgrade: [[BODY_UPGRADE; 2]; 8];
    /* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    /*returns the weapon sub class based on the string name passed in */
    #[no_mangle]
    fn getWeaponSubClass(pSubClass: *mut STRING) -> SDWORD;
    /*either gets the name associated with the resource (if one) or allocates space and copies pName*/
    #[no_mangle]
    fn allocateName(ppStore: *mut *mut STRING, pName: *mut STRING) -> BOOL;
    /*sets the store to the body size based on the name passed in - returns FALSE 
if doesn't compare with any*/
    #[no_mangle]
    fn getBodySize(pSize: *mut STRING, pStore: *mut UBYTE) -> BOOL;
    /*Access functions for the upgradeable stats of a sensor*/
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    /*Access functions for the upgradeable stats of a ECM*/
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
    //holds the upgrades attained through research for structure stats
    #[no_mangle]
    static mut asStructureUpgrade: [STRUCTURE_UPGRADE; 8];
    #[no_mangle]
    static mut asWallDefenceUpgrade: [WALLDEFENCE_UPGRADE; 8];
    //holds the upgrades for the functionality of structures through research
    #[no_mangle]
    static mut asResearchUpgrade: [RESEARCH_UPGRADE; 8];
    #[no_mangle]
    static mut asPowerUpgrade: [POWER_UPGRADE; 8];
    #[no_mangle]
    static mut asRepairFacUpgrade: [REPAIR_FACILITY_UPGRADE; 8];
    #[no_mangle]
    static mut asProductionUpgrade: [[PRODUCTION_UPGRADE; 3]; 8];
    #[no_mangle]
    static mut asReArmUpgrade: [REARM_UPGRADE; 8];
    /*for a given structure, return a pointer to its module stat */
    #[no_mangle]
    fn getModuleStat(psStruct: *mut STRUCTURE) -> *mut STRUCTURE_STATS;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn structureArmour(psStats: *mut STRUCTURE_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    fn structureResistance(psStats: *mut STRUCTURE_STATS, player: UBYTE)
     -> UDWORD;
    /*this returns the Base Body points of a structure - regardless of upgrade*/
    #[no_mangle]
    fn structureBaseBody(psStructure: *mut STRUCTURE) -> UDWORD;
    /* Default level of sensor, repair and ECM */
    #[no_mangle]
    static mut aDefaultSensor: [UDWORD; 8];
    /* Calculate the base body points of a droid without upgrades*/
    #[no_mangle]
    fn calcDroidBaseBody(psDroid: *mut DROID) -> UDWORD;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // multijoin
    #[no_mangle]
    fn modifyResources(psFunction: *mut POWER_GEN_FUNCTION);
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
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
pub type BASE_STATS = _base_stats;
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
pub type ECM_STATS = _ecm_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
/* weapon stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_upgrade {
    pub firePause: UBYTE,
    pub shortHit: UWORD,
    pub longHit: UWORD,
    pub damage: UWORD,
    pub radiusDamage: UWORD,
    pub incenDamage: UWORD,
    pub radiusHit: UWORD,
}
pub type WEAPON_UPGRADE = _weapon_upgrade;
/*sensor stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _sensor_upgrade {
    pub power: UWORD,
    pub range: UWORD,
}
pub type SENSOR_UPGRADE = _sensor_upgrade;
/*ECM stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ecm_upgrade {
    pub power: UWORD,
}
pub type ECM_UPGRADE = _ecm_upgrade;
/*repair stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_upgrade {
    pub repairPoints: UWORD,
}
pub type REPAIR_UPGRADE = _repair_upgrade;
/*constructor stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _constructor_upgrade {
    pub constructPoints: UWORD,
}
pub type CONSTRUCTOR_UPGRADE = _constructor_upgrade;
/*body stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _body_upgrade {
    pub powerOutput: UWORD,
    pub body: UWORD,
    pub armourValue: [UWORD; 2],
}
pub type BODY_UPGRADE = _body_upgrade;
/*
 * FunctionDef.h
 *
 * Structure defs for functions.
 *
 */
pub type FUNCTION_TYPES = libc::c_uint;
//DEFENSIVE_STRUCTURE_TYPE,
	//RADAR_MAP_TYPE,
	//POWER_REG_TYPE,
	//POWER_RELAY_TYPE,
	//ARMOUR_UPGRADE_TYPE,
	//REPAIR_UPGRADE_TYPE,
	//RESISTANCE_UPGRADE_TYPE,
	//DROID_DESIGN_TYPE,
	//MAP_MARKER_TYPE,
	//SKY_DOME_MAP_TYPE,
	//BODY_UPGRADE_TYPE,
	//HQ_TYPE,
/* The number of function types */
pub const NUMFUNCTIONS: FUNCTION_TYPES = 20;
pub const REARM_UPGRADE_TYPE: FUNCTION_TYPES = 19;
pub const REARM_TYPE: FUNCTION_TYPES = 18;
pub const DROIDCONST_UPGRADE_TYPE: FUNCTION_TYPES = 17;
pub const DROIDSENSOR_UPGRADE_TYPE: FUNCTION_TYPES = 16;
pub const DROIDBODY_UPGRADE_TYPE: FUNCTION_TYPES = 15;
pub const DROIDECM_UPGRADE_TYPE: FUNCTION_TYPES = 14;
pub const DROIDREPAIR_UPGRADE_TYPE: FUNCTION_TYPES = 13;
pub const REPAIR_UPGRADE_TYPE: FUNCTION_TYPES = 12;
pub const POWER_UPGRADE_TYPE: FUNCTION_TYPES = 11;
pub const WALLDEFENCE_UPGRADE_TYPE: FUNCTION_TYPES = 10;
pub const STRUCTURE_UPGRADE_TYPE: FUNCTION_TYPES = 9;
pub const WALL_TYPE: FUNCTION_TYPES = 8;
pub const WEAPON_UPGRADE_TYPE: FUNCTION_TYPES = 7;
pub const REPAIR_DROID_TYPE: FUNCTION_TYPES = 6;
pub const RESOURCE_TYPE: FUNCTION_TYPES = 5;
pub const POWER_GEN_TYPE: FUNCTION_TYPES = 4;
pub const RESEARCH_UPGRADE_TYPE: FUNCTION_TYPES = 3;
pub const RESEARCH_TYPE: FUNCTION_TYPES = 2;
pub const PRODUCTION_UPGRADE_TYPE: FUNCTION_TYPES = 1;
pub const PRODUCTION_TYPE: FUNCTION_TYPES = 0;
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
pub type FUNCTION = _function;
/*To repair droids that enter the repair facility*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_droid_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub repairPoints: UDWORD,
}
pub type REPAIR_DROID_FUNCTION = _repair_droid_function;
/*The number of repair points used to reduce 
									  damage to the droid. These repair points can 
									  restore even destroyed droid components*/
/*To generate and supply power to other structures*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub powerOutput: UDWORD,
    pub powerMultiplier: UDWORD,
    pub criticalMassChance: UDWORD,
    pub criticalMassRadius: UDWORD,
    pub criticalMassDamage: UDWORD,
    pub radiationDecayTime: UDWORD,
}
pub type POWER_GEN_FUNCTION = _power_gen_function;
/*How long the radiation lasts n time cycles*/
/*function used by walls to define which corner to use*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wall_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub pStructName: *mut STRING,
    pub pCornerStat: *mut _structure_stats,
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
pub type WALL_FUNCTION = _wall_function;
//pointer to which stat to use as a corner wall
/*function used by Resource Extractor to indicate how much resource is available*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _resource_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub maxPower: UDWORD,
}
pub type RESOURCE_FUNCTION = _resource_function;
/*The max amount output from the resource*/
/*To increase a production facilities output*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _production_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub outputModifier: UBYTE,
    pub factory: UBYTE,
    pub cyborgFactory: UBYTE,
    pub vtolFactory: UBYTE,
}
pub type PRODUCTION_UPGRADE_FUNCTION = _production_upgrade_function;
/*flag to indicate upgrades vtol factories*/
/*To manufacture droids designed previously*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _production_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub capacity: UWORD,
    pub productionOutput: UWORD,
}
pub type PRODUCTION_FUNCTION = _production_function;
/*Droid Build Points Produced Per 
												  Build Cycle*/
//struct _propulsion_types*		propulsionType;		
	//UBYTE					propulsionType;		/*The type of propulsion the facility 
	//											  can produce*/
/*To research topics available*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub researchPoints: UDWORD,
}
pub type RESEARCH_FUNCTION = _research_function;
/*The number of research points added per 
									  research cycle*/
/*To rearm VTOLs*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rearm_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub reArmPoints: UDWORD,
}
pub type REARM_FUNCTION = _rearm_function;
/*The number of reArm points added per cycle*/
/*Generic upgrade function*/
/*common stats*/
/*The % to add to the action points*/
//UBYTE			upgradePoints	/*The % to add to the action points*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub upgradePoints: UWORD,
}
pub type UPGRADE_FUNCTION = _upgrade_function;
pub type RESEARCH_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
pub type REPAIR_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
pub type POWER_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
pub type REARM_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
/*Upgrade the weapon ROF and accuracy for the weapons of a particular class*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub subClass: WEAPON_CLASS,
    pub firePause: UBYTE,
    pub shortHit: UWORD,
    pub longHit: UWORD,
    pub damage: UWORD,
    pub radiusDamage: UWORD,
    pub incenDamage: UWORD,
    pub radiusHit: UWORD,
}
pub type WEAPON_UPGRADE_FUNCTION = _weapon_upgrade_function;
/*The % to increase the chance to hit in blast radius*/
/*Upgrade the structure stats for all non wall and defence structures*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub armour: UWORD,
    pub body: UWORD,
    pub resistance: UWORD,
}
pub type STRUCTURE_UPGRADE_FUNCTION = _structure_upgrade_function;
/*The % to increase the resistance*/
/*Upgrade the structure stats for all wall and defence structures*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wallDefence_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub armour: UWORD,
    pub body: UWORD,
}
pub type WALLDEFENCE_UPGRADE_FUNCTION = _wallDefence_upgrade_function;
pub type DROIDREPAIR_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
pub type DROIDECM_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
pub type DROIDCONSTR_UPGRADE_FUNCTION = UPGRADE_FUNCTION;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droidBody_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub upgradePoints: UWORD,
    pub body: UWORD,
    pub armourValue: [UWORD; 2],
    pub cyborg: UBYTE,
    pub droid: UBYTE,
}
pub type DROIDBODY_UPGRADE_FUNCTION = _droidBody_upgrade_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droidsensor_upgrade_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub upgradePoints: UWORD,
    pub range: UWORD,
}
pub type DROIDSENSOR_UPGRADE_FUNCTION = _droidsensor_upgrade_function;
/*The % to increase the body points*/
/*flag to specify the upgrade is valid 
										  for droids (non cyborgs!)*/
// % to increase range by
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
pub type RUN_DATA = _run_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
pub type C2RustUnnamed = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed = 20;
pub const REF_REARM_PAD: C2RustUnnamed = 19;
pub const REF_LAB: C2RustUnnamed = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed = 16;
pub const REF_DEMOLISH: C2RustUnnamed = 15;
pub const REF_BRIDGE: C2RustUnnamed = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed = 11;
pub const REF_RESEARCH: C2RustUnnamed = 10;
pub const REF_BLASTDOOR: C2RustUnnamed = 9;
pub const REF_WALLCORNER: C2RustUnnamed = 8;
pub const REF_WALL: C2RustUnnamed = 7;
pub const REF_DEFENSE: C2RustUnnamed = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed = 5;
pub const REF_POWER_MODULE: C2RustUnnamed = 4;
pub const REF_POWER_GEN: C2RustUnnamed = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed = 2;
pub const REF_FACTORY: C2RustUnnamed = 1;
pub const REF_HQ: C2RustUnnamed = 0;
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type POSITION_TYPE = _position_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _flag_position {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub coords: iVector,
    pub factoryInc: UBYTE,
    pub factoryType: UBYTE,
    pub psNext: *mut _flag_position,
}
pub type FLAG_POSITION = _flag_position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_facility {
    pub psSubject: *mut _base_stats,
    pub capacity: UDWORD,
    pub timeStarted: UDWORD,
    pub researchPoints: UDWORD,
    pub timeToResearch: UDWORD,
    pub psBestTopic: *mut _base_stats,
    pub powerAccrued: UDWORD,
    pub timeStartHold: UDWORD,
}
pub type RESEARCH_FACILITY = _research_facility;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _factory {
    pub capacity: UBYTE,
    pub quantity: UBYTE,
    pub loopsPerformed: UBYTE,
    pub productionOutput: UBYTE,
    pub powerAccrued: UDWORD,
    pub psSubject: *mut BASE_STATS,
    pub timeStarted: UDWORD,
    pub timeToBuild: UDWORD,
    pub timeStartHold: UDWORD,
    pub psAssemblyPoint: *mut FLAG_POSITION,
    pub psFormation: *mut _formation,
    pub psCommander: *mut _droid,
    pub secondaryOrder: UDWORD,
}
pub type FACTORY = _factory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen {
    pub power: UDWORD,
    pub multiplier: UDWORD,
    pub capacity: UDWORD,
    pub apResExtractors: [*mut _structure; 4],
}
pub type POWER_GEN = _power_gen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REPAIR_FACILITY {
    pub power: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub powerAccrued: UDWORD,
    pub psDeliveryPoint: *mut FLAG_POSITION,
    pub currentPtsAdded: UDWORD,
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rearm_pad {
    pub reArmPoints: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub currentPtsAdded: UDWORD,
}
pub type REARM_PAD = _rearm_pad;
pub type STRUCTURE = _structure;
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*pointers to the res ext
																associated with this gen*/
/* stores the amount of body points added to the unit
                                               that is being worked on */
/* structure stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_upgrade {
    pub armour: UWORD,
    pub body: UWORD,
    pub resistance: UWORD,
}
pub type STRUCTURE_UPGRADE = _structure_upgrade;
/* wall/Defence structure stats which can be upgraded by research*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wallDefence_upgrade {
    pub armour: UWORD,
    pub body: UWORD,
}
pub type WALLDEFENCE_UPGRADE = _wallDefence_upgrade;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _upgrade {
    pub modifier: UWORD,
}
pub type UPGRADE = _upgrade;
pub type RESEARCH_UPGRADE = UPGRADE;
pub type PRODUCTION_UPGRADE = UPGRADE;
pub type REPAIR_FACILITY_UPGRADE = UPGRADE;
pub type POWER_UPGRADE = UPGRADE;
pub type REARM_UPGRADE = UPGRADE;
//% to increase the stat by
/*
 * Function.c
 *
 * Store function stats for the Structures etc.
 *
 */
//holder for all functions
#[no_mangle]
pub static mut asFunctions: *mut *mut FUNCTION =
    0 as *const *mut FUNCTION as *mut *mut FUNCTION;
#[no_mangle]
pub static mut numFunctions: UDWORD = 0;
//array of functions pointers for each load function
#[no_mangle]
pub static mut pLoadFunction:
           [Option<unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL>; 20] =
    unsafe {
        [Some(loadProduction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadProductionUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadResearchFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadResearchUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadPowerGenFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadResourceFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadRepairDroidFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadWeaponUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadWallFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadStructureUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadWallDefenceUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadPowerUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadRepairUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadDroidRepairUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadDroidECMUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadDroidBodyUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadDroidSensorUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadDroidConstUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadReArmFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL),
         Some(loadReArmUpgradeFunction as
                  unsafe extern "C" fn(_: *mut libc::c_char) -> BOOL)]
    };
/*
 * Function.h
 *
 * Definitions for the Structure Functions.
 *
 */
//holder for all functions
//lists the current Upgrade level that can be applied to a structure through research
//extern FUNCTION_UPGRADE		*apProductionUpgrades[MAX_PLAYERS];
//extern UDWORD		numProductionUpgrades;
//extern FUNCTION_UPGRADE		*apResearchUpgrades[MAX_PLAYERS];
//extern UDWORD		numResearchUpgrades;
//extern FUNCTION_UPGRADE		*apArmourUpgrades[MAX_PLAYERS];
//extern UDWORD		numArmourUpgrades;
//extern FUNCTION_UPGRADE		*apBodyUpgrades[MAX_PLAYERS];
//extern UDWORD		numBodyUpgrades;
//extern FUNCTION_UPGRADE		*apRepairUpgrades[MAX_PLAYERS];
//extern UDWORD		numRepairUpgrades;
//extern FUNCTION_UPGRADE		*apResistanceUpgrades[MAX_PLAYERS];
//extern UDWORD		numResistanceUpgrades;
//extern FUNCTION_UPGRADE		*apWeaponUpgrades[MAX_PLAYERS];
//extern UDWORD		numWeaponUpgrades;
#[no_mangle]
pub unsafe extern "C" fn loadFunctionStats(mut pFunctionData:
                                               *mut libc::c_char,
                                           mut bufferSize: UDWORD) -> BOOL {
    let mut pStartFunctionData: *mut libc::c_char =
        0 as *mut libc::c_char; //, player;
    let mut totalFunctions: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut type_0: UDWORD = 0;
    let mut FunctionType: [STRING; 60] = [0; 60];
    let mut pStartList: *mut *mut FUNCTION = 0 as *mut *mut FUNCTION;
    //keep the start so we release it at the end
    pStartFunctionData = pFunctionData;
    totalFunctions = numCR(pFunctionData, bufferSize);
    //allocate storage for the Function pointer array
    asFunctions =
        memMallocRelease(totalFunctions.wrapping_mul(::std::mem::size_of::<*mut FUNCTION>()
                                                         as libc::c_ulong)) as
            *mut *mut FUNCTION;
    if asFunctions.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    pStartList = asFunctions;
    //initialise the storage
    memset(asFunctions as *mut libc::c_void, 0 as libc::c_int,
           totalFunctions.wrapping_mul(::std::mem::size_of::<*mut FUNCTION>()
                                           as libc::c_ulong));
    numFunctions = 0 as libc::c_int as UDWORD;
    //numProductionUpgrades =	numResearchUpgrades = 0;//numArmourUpgrades =
		//numRepairUpgrades = numResistanceUpgrades = numBodyUpgrades =
		//numWeaponUpgrades = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < totalFunctions {
        //read the data into the storage - the data is delimeted using comma's
        FunctionType[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pFunctionData,
               b"%[^\',\']\x00" as *const u8 as *const libc::c_char,
               FunctionType.as_mut_ptr());
        type_0 = functionType(FunctionType.as_mut_ptr());
        pFunctionData =
            pFunctionData.offset(strlen(FunctionType.as_mut_ptr()).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                     as isize);
        if pLoadFunction[type_0 as
                             usize].expect("non-null function pointer")(pFunctionData)
               == 0 {
            return 0 as libc::c_int
        }
        //increment the pointer to the start of the next record
        pFunctionData =
            strchr(pFunctionData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //set the function list pointer to the start
    asFunctions = pStartList;
    //	FREE (pStartFunctionData);
    //create Upgrade arrays
	//for (player = 0; player < MAX_PLAYERS; player++)
	//{
		/*apProductionUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numProductionUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apProductionUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}

		apResearchUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numResearchUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apResearchUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
    /*apBodyUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numBodyUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apBodyUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
    /*apArmourUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numArmourUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apArmourUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
    /*apRepairUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numRepairUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apRepairUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
    /*apResistanceUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numResistanceUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apResistanceUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
		/*apWeaponUpgrades[player] = (FUNCTION_UPGRADE *) MALLOC(numWeaponUpgrades *
			sizeof(FUNCTION_UPGRADE));
		if (!apWeaponUpgrades[player])
		{
			DBERROR(("Out of memory"));
			return FALSE;
		}*/
	//}
    //pStartList = asFunctions;
	//numProductionUpgrades =	numResearchUpgrades = 0;//numArmourUpgrades =
		//numRepairUpgrades = numResistanceUpgrades = numBodyUpgrades =
		//numWeaponUpgrades = 0;
	//now fill the Upgrade arrays
	//for (i = 0; i < numFunctions; i++)
	//{
	//	switch ((*pStartList)->type)
	//	{
			/*case (PRODUCTION_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apProductionUpgrades[player][numProductionUpgrades].functionInc = i;
					apProductionUpgrades[player][numProductionUpgrades].available = FALSE;
				}
				numProductionUpgrades++;
				break;
			}*/
			/*case (RESEARCH_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apResearchUpgrades[player][numResearchUpgrades].functionInc = i;
					apResearchUpgrades[player][numResearchUpgrades].available = FALSE;
				}
				numResearchUpgrades++;
				break;
			}*/
			/*case (ARMOUR_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apArmourUpgrades[player][numArmourUpgrades].functionInc = i;
					apArmourUpgrades[player][numArmourUpgrades].available = FALSE;
				}
				numArmourUpgrades++;
				break;
			}*/
			/*case (BODY_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apBodyUpgrades[player][numBodyUpgrades].functionInc = i;
					apBodyUpgrades[player][numBodyUpgrades].available = FALSE;
				}
				numBodyUpgrades++;
				break;
			}*/
			/*case (REPAIR_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apRepairUpgrades[player][numRepairUpgrades].functionInc = i;
					apRepairUpgrades[player][numRepairUpgrades].available = FALSE;
				}
				numRepairUpgrades++;
				break;
			}*/
			/*case (RESISTANCE_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apResistanceUpgrades[player][numResistanceUpgrades].functionInc = i;
					apResistanceUpgrades[player][numResistanceUpgrades].available = FALSE;
				}
				numResistanceUpgrades++;
				break;
			}*/
			/*case (WEAPON_UPGRADE_TYPE):
			{
				for (player = 0; player < MAX_PLAYERS; player++)
				{
					apWeaponUpgrades[player][numWeaponUpgrades].functionInc = i;
					apWeaponUpgrades[player][numWeaponUpgrades].available = FALSE;
				}
				numWeaponUpgrades++;
				break;
			}*/
			//default:
				//do nothing
	//	}//end of switch
	//	pStartList++;
	//}
    return 1 as libc::c_int;
}
// Allocate storage for the name
unsafe extern "C" fn storeName(mut pFunction: *mut FUNCTION,
                               mut pNameToStore: *mut STRING) -> BOOL {
    (*pFunction).pName =
        memMallocRelease(strlen(pNameToStore).wrapping_add(1 as libc::c_int as
                                                               libc::c_uint))
            as *mut STRING;
    if (*pFunction).pName.is_null() {
        debug(LOG_ERROR,
              b"Function Name - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    strcpy((*pFunction).pName, pNameToStore);
    return 1 as libc::c_int;
}
//load the specific stats for each function
/*BOOL loadFunction(char *pData, UDWORD functionType)
{
	FUNCTION*				psFunction;
	STRING					functionName[50];

	//allocate storage
	psFunction = (FUNCTION *)MALLOC(sizeof(FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION*)psFunction;
	numFunctions++;
	asFunctions++;

	//set the type of function
	switch (functionType)
	{
	case DROID_DESIGN_TYPE:
		psFunction->type = DROID_DESIGN_TYPE;
		break;
	case MAP_MARKER_TYPE:
		psFunction->type = MAP_MARKER_TYPE;
		break;
	case SKY_DOME_MAP_TYPE:
		psFunction->type = SKY_DOME_MAP_TYPE;
		break;
	default:
		DBERROR(("Unknown Function Type"));
		return FALSE;
	}

	//read the data in
	sscanf(pData, "%[^',']", &functionName);

	//allocate storage for the name
	storeName(psFunction, functionName);

	return TRUE;
}*/
#[no_mangle]
pub unsafe extern "C" fn loadProduction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut PRODUCTION_FUNCTION =
        0 as *mut PRODUCTION_FUNCTION;
    //UBYTE					propType;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut bodySize: [STRING; 60] = [0; 60];
    let mut productionOutput: UDWORD = 0;
    //STRING					propulsionType[MAX_NAME_SIZE];
	//PROPULSION_TYPES*		pPropulsionType;
/*#ifdef HASH_NAMES
	UDWORD	HashedName;
#endif*/
	//allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<PRODUCTION_FUNCTION>() as
                             libc::c_ulong) as *mut PRODUCTION_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Production Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<PRODUCTION_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = PRODUCTION_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    //propulsionType[0] = '\0';
    bodySize[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), bodySize.as_mut_ptr(),
           &mut productionOutput as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    //get the propulsion stats pointer
/*	pPropulsionType = asPropulsionTypes;
	psFunction->propulsionType = 0;
#ifdef HASH_NAMES
	HashedName=HashString(propulsionType);
#endif
	for (i=0; i < numPropulsionTypes; i++)
	{
		//compare the names
#ifdef HASH_NAMES
		if (HashedName== pPropulsionType->NameHash)
#else
		if (!strcmp(propulsionType, pPropulsionType->pName))
#endif
		{
			psFunction->propulsionType = pPropulsionType;
			break;
		}
		pPropulsionType++;
	}
	//if haven't found the propulsion type, then problem
	if (!psFunction->propulsionType)
	{
		DBERROR(("Unknown Propulsion Type"));
		return FALSE;
	}
*/
	/*propType = getPropulsionType(propulsionType);
	if (propType == INVALID_PROP_TYPE)
	{
		DBERROR(("Unknown Propulsion Type - %s", propulsionType));
		return FALSE;
	}
	psFunction->propulsionType = propType;*/
    if getBodySize(bodySize.as_mut_ptr(),
                   &mut (*psFunction).capacity as *mut UWORD as *mut UBYTE) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"loadProduction: unknown body size for %s\x00" as *const u8
                      as *const libc::c_char, (*psFunction).pName);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"function.c\x00" as *const u8 as *const libc::c_char,
                  419 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"loadProduction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check prod output < UWORD_MAX
    if productionOutput < 0xffff as libc::c_int as libc::c_uint {
        (*psFunction).productionOutput = productionOutput as UWORD
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"loadProduction: production Output too big for %s\x00" as
                      *const u8 as *const libc::c_char, (*psFunction).pName);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"function.c\x00" as *const u8 as *const libc::c_char,
                  432 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"loadProduction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        (*psFunction).productionOutput = 0 as libc::c_int as UWORD
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadProductionUpgradeFunction(mut pData:
                                                           *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut PRODUCTION_UPGRADE_FUNCTION =
        0 as *mut PRODUCTION_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut factory: UDWORD = 0;
    let mut cyborg: UDWORD = 0;
    let mut vtol: UDWORD = 0;
    let mut outputModifier: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<PRODUCTION_UPGRADE_FUNCTION>()
                             as libc::c_ulong) as
            *mut PRODUCTION_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Production Upgrade Function - Out of memory\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<PRODUCTION_UPGRADE_FUNCTION>() as
               libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = PRODUCTION_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%d,%d,%d,%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), &mut factory as *mut UDWORD,
           &mut cyborg as *mut UDWORD, &mut vtol as *mut UDWORD,
           &mut outputModifier as *mut UDWORD);
    (*psFunction).outputModifier = outputModifier as UBYTE;
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    //set the factory flags
    if factory != 0 {
        (*psFunction).factory = 1 as libc::c_int as UBYTE
    } else { (*psFunction).factory = 0 as libc::c_int as UBYTE }
    if cyborg != 0 {
        (*psFunction).cyborgFactory = 1 as libc::c_int as UBYTE
    } else { (*psFunction).cyborgFactory = 0 as libc::c_int as UBYTE }
    if vtol != 0 {
        (*psFunction).vtolFactory = 1 as libc::c_int as UBYTE
    } else { (*psFunction).vtolFactory = 0 as libc::c_int as UBYTE }
    //increment the number of upgrades
	//numProductionUpgrades++;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadResearchFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut RESEARCH_FUNCTION = 0 as *mut RESEARCH_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<RESEARCH_FUNCTION>() as
                             libc::c_ulong) as *mut RESEARCH_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Research Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<RESEARCH_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = RESEARCH_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(),
           &mut (*psFunction).researchPoints as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadReArmFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut REARM_FUNCTION = 0 as *mut REARM_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<REARM_FUNCTION>() as
                             libc::c_ulong) as *mut REARM_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"ReArm Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<REARM_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = REARM_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(),
           &mut (*psFunction).reArmPoints as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadResearchUpgradeFunction(mut pData:
                                                         *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData,
                           RESEARCH_UPGRADE_TYPE as libc::c_int as UBYTE) == 0
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadPowerUpgradeFunction(mut pData:
                                                      *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData, POWER_UPGRADE_TYPE as libc::c_int as UBYTE)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadRepairUpgradeFunction(mut pData:
                                                       *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData, REPAIR_UPGRADE_TYPE as libc::c_int as UBYTE)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDroidRepairUpgradeFunction(mut pData:
                                                            *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData,
                           DROIDREPAIR_UPGRADE_TYPE as libc::c_int as UBYTE)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDroidECMUpgradeFunction(mut pData:
                                                         *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData,
                           DROIDECM_UPGRADE_TYPE as libc::c_int as UBYTE) == 0
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDroidConstUpgradeFunction(mut pData:
                                                           *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData,
                           DROIDCONST_UPGRADE_TYPE as libc::c_int as UBYTE) ==
           0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadReArmUpgradeFunction(mut pData:
                                                      *mut libc::c_char)
 -> BOOL {
    if loadUpgradeFunction(pData, REARM_UPGRADE_TYPE as libc::c_int as UBYTE)
           == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//generic load function for upgrade type
unsafe extern "C" fn loadUpgradeFunction(mut pData: *mut libc::c_char,
                                         mut type_0: UBYTE) -> BOOL {
    let mut functionName: [STRING; 60] = [0; 60];
    let mut modifier: UDWORD = 0;
    let mut psFunction: *mut UPGRADE_FUNCTION = 0 as *mut UPGRADE_FUNCTION;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<UPGRADE_FUNCTION>() as
                             libc::c_ulong) as *mut UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Upgrade Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<UPGRADE_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = type_0;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), &mut modifier as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    if modifier > 0xffff as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"loadUpgradeFunction: modifier too great for %s\x00" as
                      *const u8 as *const libc::c_char,
                  functionName.as_mut_ptr());
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"function.c\x00" as *const u8 as *const libc::c_char,
                  682 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"loadUpgradeFunction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //store the % upgrade
    (*psFunction).upgradePoints = modifier as UWORD;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDroidBodyUpgradeFunction(mut pData:
                                                          *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut DROIDBODY_UPGRADE_FUNCTION =
        0 as *mut DROIDBODY_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut modifier: UDWORD = 0;
    let mut armourKinetic: UDWORD = 0;
    let mut armourHeat: UDWORD = 0;
    let mut body: UDWORD = 0;
    let mut droid: UDWORD = 0;
    let mut cyborg: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<DROIDBODY_UPGRADE_FUNCTION>()
                             as libc::c_ulong) as
            *mut DROIDBODY_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"UnitBody Upgrade Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<DROIDBODY_UPGRADE_FUNCTION>() as
               libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = DROIDBODY_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%d,%d,%d,%d,%d,%d\x00" as *const u8 as
               *const libc::c_char, functionName.as_mut_ptr(),
           &mut modifier as *mut UDWORD, &mut body as *mut UDWORD,
           &mut armourKinetic as *mut UDWORD, &mut armourHeat as *mut UDWORD,
           &mut droid as *mut UDWORD, &mut cyborg as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    if modifier > 0xffff as libc::c_int as libc::c_uint ||
           armourKinetic > 0xffff as libc::c_int as libc::c_uint ||
           armourHeat > 0xffff as libc::c_int as libc::c_uint ||
           body > 0xffff as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"loadUnitBodyUpgradeFunction: one or more modifiers too great\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"function.c\x00" as *const u8 as *const libc::c_char,
                  731 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 29],
                                            &[libc::c_char; 29]>(b"loadDroidBodyUpgradeFunction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //store the % upgrades
    (*psFunction).upgradePoints = modifier as UWORD;
    (*psFunction).body = body as UWORD;
    (*psFunction).armourValue[WC_KINETIC as libc::c_int as usize] =
        armourKinetic as UWORD;
    (*psFunction).armourValue[WC_HEAT as libc::c_int as usize] =
        armourHeat as UWORD;
    if droid != 0 {
        (*psFunction).droid = 1 as libc::c_int as UBYTE
    } else { (*psFunction).droid = 0 as libc::c_int as UBYTE }
    if cyborg != 0 {
        (*psFunction).cyborg = 1 as libc::c_int as UBYTE
    } else { (*psFunction).cyborg = 0 as libc::c_int as UBYTE }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDroidSensorUpgradeFunction(mut pData:
                                                            *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut DROIDSENSOR_UPGRADE_FUNCTION =
        0 as *mut DROIDSENSOR_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut modifier: UDWORD = 0;
    let mut range: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<DROIDSENSOR_UPGRADE_FUNCTION>()
                             as libc::c_ulong) as
            *mut DROIDSENSOR_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"UnitSensor Upgrade Function - Out of memory\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<DROIDSENSOR_UPGRADE_FUNCTION>() as
               libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = DROIDSENSOR_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d,%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), &mut modifier as *mut UDWORD,
           &mut range as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    if modifier > 0xffff as libc::c_int as libc::c_uint ||
           range > 0xffff as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"loadUnitSensorUpgradeFunction: one or more modifiers too great\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"function.c\x00" as *const u8 as *const libc::c_char,
                  796 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 31],
                                            &[libc::c_char; 31]>(b"loadDroidSensorUpgradeFunction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //store the % upgrades
    (*psFunction).upgradePoints = modifier as UWORD;
    (*psFunction).range = range as UWORD;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadWeaponUpgradeFunction(mut pData:
                                                       *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut WEAPON_UPGRADE_FUNCTION =
        0 as *mut WEAPON_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut weaponSubClass: [STRING; 60] = [0; 60];
    let mut firePause: UDWORD = 0;
    let mut shortHit: UDWORD = 0;
    let mut longHit: UDWORD = 0;
    let mut damage: UDWORD = 0;
    let mut radiusDamage: UDWORD = 0;
    let mut incenDamage: UDWORD = 0;
    let mut radiusHit: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<WEAPON_UPGRADE_FUNCTION>() as
                             libc::c_ulong) as *mut WEAPON_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Weapon Upgrade Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<WEAPON_UPGRADE_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = WEAPON_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    weaponSubClass[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%d\x00" as *const u8 as
               *const libc::c_char, functionName.as_mut_ptr(),
           weaponSubClass.as_mut_ptr(), &mut firePause as *mut UDWORD,
           &mut shortHit as *mut UDWORD, &mut longHit as *mut UDWORD,
           &mut damage as *mut UDWORD, &mut radiusDamage as *mut UDWORD,
           &mut incenDamage as *mut UDWORD, &mut radiusHit as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    (*psFunction).subClass =
        getWeaponSubClass(weaponSubClass.as_mut_ptr()) as WEAPON_CLASS;
    if (*psFunction).subClass as libc::c_uint ==
           (NUM_WEAPON_SUBCLASS as libc::c_int + 1 as libc::c_int) as
               libc::c_uint {
        return 0 as libc::c_int
    }
    //check none of the %increases are over UBYTE max
    if firePause > 0xff as libc::c_int as libc::c_uint ||
           shortHit > 0xffff as libc::c_int as libc::c_uint ||
           longHit > 0xffff as libc::c_int as libc::c_uint ||
           damage > 0xffff as libc::c_int as libc::c_uint ||
           radiusDamage > 0xffff as libc::c_int as libc::c_uint ||
           incenDamage > 0xffff as libc::c_int as libc::c_uint ||
           radiusHit > 0xffff as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"A percentage increase for Weapon Upgrade function is too large\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    //copy the data across
    (*psFunction).firePause = firePause as UBYTE;
    (*psFunction).shortHit = shortHit as UWORD;
    (*psFunction).longHit = longHit as UWORD;
    (*psFunction).damage = damage as UWORD;
    (*psFunction).radiusDamage = radiusDamage as UWORD;
    (*psFunction).incenDamage = incenDamage as UWORD;
    (*psFunction).radiusHit = radiusHit as UWORD;
    //increment the number of upgrades
	//numWeaponUpgrades++;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadStructureUpgradeFunction(mut pData:
                                                          *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut STRUCTURE_UPGRADE_FUNCTION =
        0 as *mut STRUCTURE_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut armour: UDWORD = 0;
    let mut body: UDWORD = 0;
    let mut resistance: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<STRUCTURE_UPGRADE_FUNCTION>()
                             as libc::c_ulong) as
            *mut STRUCTURE_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Structure Upgrade Function - Out of memory\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<STRUCTURE_UPGRADE_FUNCTION>() as
               libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = STRUCTURE_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%d,%d,%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), &mut armour as *mut UDWORD,
           &mut body as *mut UDWORD, &mut resistance as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    //check none of the %increases are over UWORD max
    if armour > 0xffff as libc::c_int as libc::c_uint ||
           body > 0xffff as libc::c_int as libc::c_uint ||
           resistance > 0xffff as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"A percentage increase for Structure Upgrade function is too large\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    //copy the data across
    (*psFunction).armour = armour as UWORD;
    (*psFunction).body = body as UWORD;
    (*psFunction).resistance = resistance as UWORD;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadWallDefenceUpgradeFunction(mut pData:
                                                            *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut WALLDEFENCE_UPGRADE_FUNCTION =
        0 as *mut WALLDEFENCE_UPGRADE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut armour: UDWORD = 0;
    let mut body: UDWORD = 0;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<WALLDEFENCE_UPGRADE_FUNCTION>()
                             as libc::c_ulong) as
            *mut WALLDEFENCE_UPGRADE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"WallDefence Upgrade Function - Out of memory\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<WALLDEFENCE_UPGRADE_FUNCTION>() as
               libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = WALLDEFENCE_UPGRADE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d,%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), &mut armour as *mut UDWORD,
           &mut body as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    //check none of the %increases are over UWORD max
    if armour > 0xffff as libc::c_int as libc::c_uint ||
           body > 0xffff as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"A percentage increase for WallDefence Upgrade function is too large\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    }
    //copy the data across
    (*psFunction).armour = armour as UWORD;
    (*psFunction).body = body as UWORD;
    return 1 as libc::c_int;
}
/*BOOL loadBodyUpgradeFunction(char *pData)
{
	BODY_UPGRADE_FUNCTION*		psFunction;
	STRING						functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (BODY_UPGRADE_FUNCTION *)MALLOC(sizeof
		(BODY_UPGRADE_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Body Upgrade Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = BODY_UPGRADE_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d", &functionName, &psFunction->bodyPoints);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	//increment the number of upgrades
	numBodyUpgrades++;

	return TRUE;
}*/
/*BOOL loadRadarMapFunction(char *pData)
{
	RADAR_MAP_FUNCTION*			psFunction;
	STRING						functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (RADAR_MAP_FUNCTION *)MALLOC(sizeof
		(RADAR_MAP_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Radar Map Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = RADAR_MAP_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d,%d", &functionName,
		&psFunction->radarDecayRate, &psFunction->radarRadius);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	return TRUE;
}*/
#[no_mangle]
pub unsafe extern "C" fn loadPowerGenFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut POWER_GEN_FUNCTION =
        0 as *mut POWER_GEN_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<POWER_GEN_FUNCTION>() as
                             libc::c_ulong) as *mut POWER_GEN_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Power Gen Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<POWER_GEN_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = POWER_GEN_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%d,%d,%d,%d,%d,%d\x00" as *const u8 as
               *const libc::c_char, functionName.as_mut_ptr(),
           &mut (*psFunction).powerOutput as *mut UDWORD,
           &mut (*psFunction).powerMultiplier as *mut UDWORD,
           &mut (*psFunction).criticalMassChance as *mut UDWORD,
           &mut (*psFunction).criticalMassRadius as *mut UDWORD,
           &mut (*psFunction).criticalMassDamage as *mut UDWORD,
           &mut (*psFunction).radiationDecayTime as *mut UDWORD);
    if bMultiPlayer != 0 { modifyResources(psFunction); }
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadResourceFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut RESOURCE_FUNCTION = 0 as *mut RESOURCE_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<RESOURCE_FUNCTION>() as
                             libc::c_ulong) as *mut RESOURCE_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Resource Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<RESOURCE_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = RESOURCE_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(),
           &mut (*psFunction).maxPower as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    return 1 as libc::c_int;
}
/*BOOL loadPowerRegFunction(char *pData)
{
	POWER_REG_FUNCTION*			psFunction;
	STRING						functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (POWER_REG_FUNCTION *)MALLOC(sizeof
		(POWER_REG_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Power Reg Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
 	psFunction->type = POWER_REG_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d", &functionName, &psFunction->maxPower);


	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	return TRUE;
}*/
/*BOOL loadPowerRelayFunction(char *pData)
{
	POWER_RELAY_FUNCTION*			psFunction;
	STRING						functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (POWER_RELAY_FUNCTION *)MALLOC(sizeof
		(POWER_RELAY_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Power Relay Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = POWER_RELAY_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d,%d", &functionName,
		&psFunction->powerRelayType, &psFunction->powerRelayRange);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	return TRUE;
}*/
#[no_mangle]
pub unsafe extern "C" fn loadRepairDroidFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut REPAIR_DROID_FUNCTION =
        0 as *mut REPAIR_DROID_FUNCTION;
    let mut functionName: [STRING; 60] = [0; 60];
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<REPAIR_DROID_FUNCTION>() as
                             libc::c_ulong) as *mut REPAIR_DROID_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Repair Droid Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<REPAIR_DROID_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = REPAIR_DROID_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData, b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(),
           &mut (*psFunction).repairPoints as *mut UDWORD);
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    return 1 as libc::c_int;
}
/*BOOL loadDefensiveStructFunction(char *pData)
{
	//pData;
	//DBERROR(("Defensive Structure Function is not longer used - \
	//	do not allocate it to a Structure!"));
	//return FALSE;
	DEFENSIVE_STRUCTURE_FUNCTION*	psFunction;
	UDWORD							i;
	STRING							functionName[MAX_NAME_SIZE];
	STRING							sensorType[MAX_NAME_SIZE];
	STRING							ecmType[MAX_NAME_SIZE];
	SENSOR_STATS*					pSensorType;
	ECM_STATS*						pECMType;

	//allocate storage
	psFunction = (DEFENSIVE_STRUCTURE_FUNCTION *)MALLOC(
		sizeof(DEFENSIVE_STRUCTURE_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Defensive Structure Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = DEFENSIVE_STRUCTURE_TYPE;

	//read the data in
	sscanf(pData, "%[^','],%[^','],%[^','],%d", &functionName, &ecmType,
		&sensorType, &psFunction->weaponCapacity);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	//get the sensor stats pointer
	if (!strcmp(sensorType,"0"))
	{
		psFunction->pSensor = NULL;
	}
	else
	{
		pSensorType = asSensorStats;
		for (i=0; i < numSensorStats; i++)
		{
			//compare the names
			if (!strcmp(sensorType, pSensorType->pName))
			{
				psFunction->pSensor = pSensorType;
				break;
			}
			pSensorType++;
		}
	}
	//get the ecm stats pointer
	if (!strcmp(ecmType,"0"))
	{
		psFunction->pECM = NULL;
	}
	else
	{
		pECMType = asECMStats;
		for (i=0; i < numECMStats; i++)
		{
			//compare the names
			if (!strcmp(ecmType, pECMType->pName))
			{
				psFunction->pECM = pECMType;
				break;
			}
			pECMType++;
		}
	}
	return TRUE;
}*/
/*BOOL loadHQFunction(SBYTE *pData)
{
	HQ_FUNCTION*		psFunction;
	STRING				functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (HQ_FUNCTION *)MALLOC(sizeof(HQ_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("HQ Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = HQ_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d", &functionName,
		&psFunction->power);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	return TRUE;
}*/
/*BOOL loadArmourUpgradeFunction(SBYTE *pData)
{
	ARMOUR_UPGRADE_FUNCTION*	psFunction;
//	UDWORD						i;
	STRING						functionName[MAX_NAME_SIZE];
	//STRING						armourType[50];
//	ARMOUR_STATS*				pArmourType;

	//allocate storage
	psFunction = (ARMOUR_UPGRADE_FUNCTION *)MALLOC(sizeof(ARMOUR_UPGRADE_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Armour Upgrade Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = ARMOUR_UPGRADE_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d,%d,%d", &functionName,
		&psFunction->buildPoints, &psFunction->powerRequired, &psFunction->armourPoints);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	//get the armour stats pointer
	//pArmourType = asArmourStats;
	//psFunction->pArmour = NULL;
	//for (i=0; i < numArmourStats; i++)
	//{
		//compare the names
	//	if (!strcmp(armourType, pArmourType->pName))
	//	{
	//		psFunction->pArmour = pArmourType;
	//		break;
	//	}
	//	pArmourType++;
	//}
	//if not found the armour stat then problem
	//if (!psFunction->pArmour)
	//{
	//	DBERROR(("Armour Type invalid"));
	//	return FALSE;
	//}

	//increment the number of upgrades
	numArmourUpgrades++;

	return TRUE;
}*/
/*BOOL loadRepairUpgradeFunction(SBYTE *pData)
{
	REPAIR_UPGRADE_FUNCTION*	psFunction;
	UDWORD						i;
	STRING						functionName[MAX_NAME_SIZE];
	STRING						repairType[MAX_NAME_SIZE];
	REPAIR_STATS*				pRepairType;

#ifdef HASH_NAMES
	UDWORD	HashedName;
#endif
	//allocate storage
	psFunction = (REPAIR_UPGRADE_FUNCTION *)MALLOC(sizeof(REPAIR_UPGRADE_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Repair Upgrade Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = REPAIR_UPGRADE_TYPE;

	//read the data in
	functionName[0] = '\0';
	repairType[0] = '\0';
	sscanf(pData, "%[^','],%[^','],%d,%d,%d", &functionName,
		&repairType, &psFunction->repairPoints, &psFunction->buildPoints,
		&psFunction->powerRequired);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	if (!getResourceName(repairType))
	{
		return FALSE;
	}


	//get the repair stats pointer
	pRepairType = asRepairStats;
	psFunction->pRepair = NULL;
#ifdef HASH_NAMES
	HashedName=HashString(repairType);
#endif


	for (i=0; i < numRepairStats; i++)
	{
		//compare the names
#ifdef HASH_NAMES
		if (HashedName== pRepairType->NameHash)
#else
		if (!strcmp(repairType, pRepairType->pName))
#endif
		{
			psFunction->pRepair = pRepairType;
			break;
		}
		pRepairType++;
	}
	//if not found the repair stats then problem
	if (!psFunction->pRepair)
	{
		DBERROR(("Repair Stats Invalid for function - %s", functionName));
		return FALSE;
	}
	//increment the number of upgrades
	numRepairUpgrades++;

	return TRUE;
}*/
/*BOOL loadResistanceUpgradeFunction(SBYTE *pData)
{
	RESISTANCE_UPGRADE_FUNCTION*		psFunction;
	STRING								functionName[MAX_NAME_SIZE];

	//allocate storage
	psFunction = (RESISTANCE_UPGRADE_FUNCTION *)MALLOC(sizeof
		(RESISTANCE_UPGRADE_FUNCTION));
	if (psFunction == NULL)
	{
		DBERROR(("Resistance Upgrade Function - Out of memory"));
		return FALSE;
	}

	//store the pointer in the Function Array
	*asFunctions = (FUNCTION *)psFunction;
	psFunction->ref = REF_FUNCTION_START + numFunctions;
	numFunctions++;
	asFunctions++;

	//set the type of function
	psFunction->type = RESISTANCE_UPGRADE_TYPE;

	//read the data in
	functionName[0] = '\0';
	sscanf(pData, "%[^','],%d,%d,%d,%d", &functionName,
		&psFunction->resistanceUpgrade, &psFunction->buildPoints,
		&psFunction->powerRequired, &psFunction->resistancePoints);

	//allocate storage for the name
	storeName((FUNCTION *)psFunction, functionName);

	//increment the number of upgrades
	numResistanceUpgrades++;

	return TRUE;
}*/
/*loads the corner stat to use for a particular wall stat */
#[no_mangle]
pub unsafe extern "C" fn loadWallFunction(mut pData: *mut libc::c_char)
 -> BOOL {
    let mut psFunction: *mut WALL_FUNCTION = 0 as *mut WALL_FUNCTION;
    //	UDWORD					i;
    let mut functionName: [STRING; 60] = [0; 60];
    let mut structureName: [STRING; 60] = [0; 60];
    //	STRUCTURE_STATS			*pStructStat;
    //allocate storage
    psFunction =
        memMallocRelease(::std::mem::size_of::<WALL_FUNCTION>() as
                             libc::c_ulong) as *mut WALL_FUNCTION;
    if psFunction.is_null() {
        debug(LOG_ERROR,
              b"Wall Function - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(psFunction as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<WALL_FUNCTION>() as libc::c_ulong);
    //store the pointer in the Function Array
    *asFunctions = psFunction as *mut FUNCTION;
    (*psFunction).ref_0 =
        (0xe0000 as libc::c_int as libc::c_uint).wrapping_add(numFunctions);
    numFunctions = numFunctions.wrapping_add(1);
    asFunctions = asFunctions.offset(1);
    //set the type of function
    (*psFunction).type_0 = WALL_TYPE as libc::c_int as UBYTE;
    //read the data in
    functionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    structureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
    sscanf(pData,
           b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as *const libc::c_char,
           functionName.as_mut_ptr(), structureName.as_mut_ptr());
    //allocate storage for the name
    storeName(psFunction as *mut FUNCTION, functionName.as_mut_ptr());
    //store the structure name - cannot set the stat pointer here because structures
	//haven't been loaded in yet!
	/*psFunction->pStructName = (STRING *)MALLOC(strlen(structureName)+1);
	if (psFunction->pStructName == NULL)
	{
		DBERROR(("Function Name - Out of memory"));
		return FALSE;
	}
	strcpy(psFunction->pStructName,structureName);*/
    if allocateName(&mut (*psFunction).pStructName,
                    structureName.as_mut_ptr()) == 0 {
        debug(LOG_ERROR,
              b"Structure Stats Invalid for function - %s\x00" as *const u8 as
                  *const libc::c_char, functionName.as_mut_ptr());
        abort();
    }
    (*psFunction).pCornerStat = 0 as *mut _structure_stats;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn productionUpgrade(mut pFunction: *mut FUNCTION,
                                           mut player: UBYTE) {
    let mut pUpgrade: *mut PRODUCTION_UPGRADE_FUNCTION =
        0 as *mut PRODUCTION_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut PRODUCTION_UPGRADE_FUNCTION;
    //check upgrades increase all values
    if (*pUpgrade).factory != 0 {
        if (asProductionUpgrade[player as
                                    usize][0 as libc::c_int as usize].modifier
                as libc::c_int) < (*pUpgrade).outputModifier as libc::c_int {
            asProductionUpgrade[player as
                                    usize][0 as libc::c_int as usize].modifier
                = (*pUpgrade).outputModifier as UWORD
        }
    }
    if (*pUpgrade).cyborgFactory != 0 {
        if (asProductionUpgrade[player as
                                    usize][1 as libc::c_int as usize].modifier
                as libc::c_int) < (*pUpgrade).outputModifier as libc::c_int {
            asProductionUpgrade[player as
                                    usize][1 as libc::c_int as usize].modifier
                = (*pUpgrade).outputModifier as UWORD
        }
    }
    if (*pUpgrade).vtolFactory != 0 {
        if (asProductionUpgrade[player as
                                    usize][2 as libc::c_int as usize].modifier
                as libc::c_int) < (*pUpgrade).outputModifier as libc::c_int {
            asProductionUpgrade[player as
                                    usize][2 as libc::c_int as usize].modifier
                = (*pUpgrade).outputModifier as UWORD
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn researchUpgrade(mut pFunction: *mut FUNCTION,
                                         mut player: UBYTE) {
    let mut pUpgrade: *mut RESEARCH_UPGRADE_FUNCTION =
        0 as *mut RESEARCH_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut RESEARCH_UPGRADE_FUNCTION;
    //check upgrades increase all values
    if (asResearchUpgrade[player as usize].modifier as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asResearchUpgrade[player as usize].modifier =
            (*pUpgrade).upgradePoints
    };
}
#[no_mangle]
pub unsafe extern "C" fn repairFacUpgrade(mut pFunction: *mut FUNCTION,
                                          mut player: UBYTE) {
    let mut pUpgrade: *mut REPAIR_UPGRADE_FUNCTION =
        0 as *mut REPAIR_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut REPAIR_UPGRADE_FUNCTION;
    //check upgrades increase all values
    if (asRepairFacUpgrade[player as usize].modifier as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asRepairFacUpgrade[player as usize].modifier =
            (*pUpgrade).upgradePoints
    };
}
#[no_mangle]
pub unsafe extern "C" fn powerUpgrade(mut pFunction: *mut FUNCTION,
                                      mut player: UBYTE) {
    let mut pUpgrade: *mut POWER_UPGRADE_FUNCTION =
        0 as *mut POWER_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut POWER_UPGRADE_FUNCTION;
    //check upgrades increase all values
    if (asPowerUpgrade[player as usize].modifier as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asPowerUpgrade[player as usize].modifier = (*pUpgrade).upgradePoints
    };
}
#[no_mangle]
pub unsafe extern "C" fn reArmUpgrade(mut pFunction: *mut FUNCTION,
                                      mut player: UBYTE) {
    let mut pUpgrade: *mut REARM_UPGRADE_FUNCTION =
        0 as *mut REARM_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut REARM_UPGRADE_FUNCTION;
    //check upgrades increase all values
    if (asReArmUpgrade[player as usize].modifier as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asReArmUpgrade[player as usize].modifier = (*pUpgrade).upgradePoints
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureBodyUpgrade(mut pFunction: *mut FUNCTION,
                                              mut psBuilding:
                                                  *mut STRUCTURE) {
    let mut increase: UWORD = 0;
    let mut prevBaseBody: UWORD = 0;
    let mut newBaseBody: UWORD = 0;
    match (*(*psBuilding).pStructureType).type_0 {
        7 | 8 | 6 | 9 => {
            increase =
                (*(pFunction as *mut WALLDEFENCE_UPGRADE_FUNCTION)).body
        }
        _ => {
            increase = (*(pFunction as *mut STRUCTURE_UPGRADE_FUNCTION)).body
        }
    }
    prevBaseBody = structureBody(psBuilding) as UWORD;
    //newBaseBody = (UWORD)(psBuilding->pStructureType->bodyPoints + (psBuilding->
	//	pStructureType->bodyPoints * increase) / 100);
    newBaseBody =
        structureBaseBody(psBuilding).wrapping_add(structureBaseBody(psBuilding).wrapping_mul(increase
                                                                                                  as
                                                                                                  libc::c_uint).wrapping_div(100
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint))
            as UWORD;
    if newBaseBody as libc::c_int > prevBaseBody as libc::c_int {
        (*psBuilding).body =
            ((*psBuilding).body as libc::c_int * newBaseBody as libc::c_int /
                 prevBaseBody as libc::c_int) as UWORD
        //psBuilding->baseBodyPoints = newBaseBody;
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureArmourUpgrade(mut pFunction: *mut FUNCTION,
                                                mut psBuilding:
                                                    *mut STRUCTURE) {
    let mut increase: UWORD = 0;
    let mut prevBaseArmour: UWORD = 0;
    let mut newBaseArmour: UWORD = 0;
    match (*(*psBuilding).pStructureType).type_0 {
        7 | 8 | 6 | 9 => {
            increase =
                (*(pFunction as *mut WALLDEFENCE_UPGRADE_FUNCTION)).armour
        }
        _ => {
            increase =
                (*(pFunction as *mut STRUCTURE_UPGRADE_FUNCTION)).armour
        }
    }
    prevBaseArmour =
        structureArmour((*psBuilding).pStructureType, (*psBuilding).player) as
            UWORD;
    newBaseArmour =
        (*(*psBuilding).pStructureType).armourValue.wrapping_add((*(*psBuilding).pStructureType).armourValue.wrapping_mul(increase
                                                                                                                              as
                                                                                                                              libc::c_uint).wrapping_div(100
                                                                                                                                                             as
                                                                                                                                                             libc::c_int
                                                                                                                                                             as
                                                                                                                                                             libc::c_uint))
            as UWORD;
    if newBaseArmour as libc::c_int > prevBaseArmour as libc::c_int {
        (*psBuilding).armour =
            ((*psBuilding).armour as libc::c_int *
                 newBaseArmour as libc::c_int / prevBaseArmour as libc::c_int)
                as UWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureResistanceUpgrade(mut pFunction:
                                                        *mut FUNCTION,
                                                    mut psBuilding:
                                                        *mut STRUCTURE) {
    let mut increase: UWORD = 0;
    let mut prevBaseResistance: UWORD = 0;
    let mut newBaseResistance: UWORD = 0;
    increase = (*(pFunction as *mut STRUCTURE_UPGRADE_FUNCTION)).resistance;
    prevBaseResistance =
        structureResistance((*psBuilding).pStructureType,
                            (*psBuilding).player) as UWORD;
    newBaseResistance =
        (*(*psBuilding).pStructureType).resistance.wrapping_add((*(*psBuilding).pStructureType).resistance.wrapping_mul(increase
                                                                                                                            as
                                                                                                                            libc::c_uint).wrapping_div(100
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint))
            as UWORD;
    if newBaseResistance as libc::c_int > prevBaseResistance as libc::c_int {
        (*psBuilding).resistance =
            ((*psBuilding).resistance as libc::c_int *
                 newBaseResistance as libc::c_int /
                 prevBaseResistance as libc::c_int) as UWORD as SWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureProductionUpgrade(mut psBuilding:
                                                        *mut STRUCTURE) {
    let mut pFact: *mut FACTORY = 0 as *mut FACTORY;
    let mut pFactFunc: *mut PRODUCTION_FUNCTION =
        0 as *mut PRODUCTION_FUNCTION;
    let mut type_0: UDWORD = 0;
    let mut baseOutput: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    match (*(*psBuilding).pStructureType).type_0 {
        1 => { type_0 = 0 as libc::c_int as UDWORD }
        16 => { type_0 = 1 as libc::c_int as UDWORD }
        17 => { type_0 = 2 as libc::c_int as UDWORD }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"structureProductionUpgrade: Invalid factory type\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"function.c\x00" as *const u8 as *const libc::c_char,
                      1764 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 27],
                                                &[libc::c_char; 27]>(b"structureProductionUpgrade\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return
        }
    }
    //upgrade the Output
    pFact = (*psBuilding).pFunctionality as *mut FACTORY;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureProductionUpgrade: invalid Factory pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1771 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"structureProductionUpgrade\x00")).as_ptr(),
              b"PTRVALID(pFact, sizeof(FACTORY))\x00" as *const u8 as
                  *const libc::c_char);
    };
    pFactFunc =
        *(*(*psBuilding).pStructureType).asFuncList.offset(0 as libc::c_int as
                                                               isize) as
            *mut PRODUCTION_FUNCTION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureProductionUpgrade: invalid Function pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1775 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"structureProductionUpgrade\x00")).as_ptr(),
              b"PTRVALID(pFactFunc, sizeof(PRODUCTION_FUNCTION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    //current base value depends on whether there are modules attached to the structure
    baseOutput = (*pFactFunc).productionOutput as UDWORD;
    psStat = getModuleStat(psBuilding);
    if !psStat.is_null() {
        i = 0 as libc::c_int as UDWORD;
        while i < (*pFact).capacity as libc::c_uint {
            baseOutput =
                (baseOutput as
                     libc::c_uint).wrapping_add((*(*(*psStat).asFuncList.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                                       as
                                                       *mut PRODUCTION_FUNCTION)).productionOutput
                                                    as libc::c_uint) as UDWORD
                    as UDWORD;
            i = i.wrapping_add(1)
        }
    }
    (*pFact).productionOutput =
        baseOutput.wrapping_add(((*pFactFunc).productionOutput as libc::c_int
                                     *
                                     asProductionUpgrade[(*psBuilding).player
                                                             as
                                                             usize][type_0 as
                                                                        usize].modifier
                                         as libc::c_int / 100 as libc::c_int)
                                    as libc::c_uint) as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn structureResearchUpgrade(mut psBuilding:
                                                      *mut STRUCTURE) {
    let mut pRes: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    let mut pResFunc: *mut RESEARCH_FUNCTION = 0 as *mut RESEARCH_FUNCTION;
    let mut baseOutput: UDWORD = 0;
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //upgrade the research points
    pRes = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureResearchUpgrade: invalid Research pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1802 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"structureResearchUpgrade\x00")).as_ptr(),
              b"PTRVALID(pRes, sizeof(RESEARCH_FACILITY))\x00" as *const u8 as
                  *const libc::c_char);
    };
    pResFunc =
        *(*(*psBuilding).pStructureType).asFuncList.offset(0 as libc::c_int as
                                                               isize) as
            *mut RESEARCH_FUNCTION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureResearchUpgrade: invalid Function pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1806 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"structureResearchUpgrade\x00")).as_ptr(),
              b"PTRVALID(pResFunc, sizeof(RESEARCH_FUNCTION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    //current base value depends on whether there are modules attached to the structure
    baseOutput = (*pResFunc).researchPoints;
    psStat = getModuleStat(psBuilding);
    if !psStat.is_null() {
        if (*pRes).capacity != 0 {
            baseOutput =
                (baseOutput as
                     libc::c_uint).wrapping_add((*(*(*psStat).asFuncList.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                                       as
                                                       *mut RESEARCH_FUNCTION)).researchPoints)
                    as UDWORD as UDWORD
        }
    }
    (*pRes).researchPoints =
        baseOutput.wrapping_add((*pResFunc).researchPoints.wrapping_mul(asResearchUpgrade[(*psBuilding).player
                                                                                              as
                                                                                              usize].modifier
                                                                            as
                                                                            libc::c_uint).wrapping_div(100
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn structureReArmUpgrade(mut psBuilding:
                                                   *mut STRUCTURE) {
    let mut pPad: *mut REARM_PAD = 0 as *mut REARM_PAD;
    let mut pPadFunc: *mut REARM_FUNCTION = 0 as *mut REARM_FUNCTION;
    //upgrade the reArm points
    pPad = (*psBuilding).pFunctionality as *mut REARM_PAD;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureReArmUpgrade: invalid ReArm pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1830 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"structureReArmUpgrade\x00")).as_ptr(),
              b"PTRVALID(pPad, sizeof(REARM_PAD))\x00" as *const u8 as
                  *const libc::c_char);
    };
    pPadFunc =
        *(*(*psBuilding).pStructureType).asFuncList.offset(0 as libc::c_int as
                                                               isize) as
            *mut REARM_FUNCTION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureReArmUpgrade: invalid Function pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1834 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"structureReArmUpgrade\x00")).as_ptr(),
              b"PTRVALID(pPadFunc, sizeof(REARM_FUNCTION))\x00" as *const u8
                  as *const libc::c_char);
    };
    (*pPad).reArmPoints =
        (*pPadFunc).reArmPoints.wrapping_add((*pPadFunc).reArmPoints.wrapping_mul(asReArmUpgrade[(*psBuilding).player
                                                                                                     as
                                                                                                     usize].modifier
                                                                                      as
                                                                                      libc::c_uint).wrapping_div(100
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn structurePowerUpgrade(mut psBuilding:
                                                   *mut STRUCTURE) {
    let mut pPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut pPGFunc: *mut POWER_GEN_FUNCTION = 0 as *mut POWER_GEN_FUNCTION;
    let mut baseOutput: UDWORD = 0;
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //upgrade the research points
    pPowerGen = (*psBuilding).pFunctionality as *mut POWER_GEN;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structurePowerUpgrade: invalid Power Gen pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1850 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"structurePowerUpgrade\x00")).as_ptr(),
              b"PTRVALID(pPowerGen, sizeof(POWER_GEN))\x00" as *const u8 as
                  *const libc::c_char);
    };
    pPGFunc =
        *(*(*psBuilding).pStructureType).asFuncList.offset(0 as libc::c_int as
                                                               isize) as
            *mut POWER_GEN_FUNCTION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structurePowerUpgrade: invalid Function pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1854 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"structurePowerUpgrade\x00")).as_ptr(),
              b"PTRVALID(pPGFunc, sizeof(POWER_GEN_FUNCTION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    //current base value depends on whether there are modules attached to the structure
    baseOutput = (*pPGFunc).powerMultiplier;
    psStat = getModuleStat(psBuilding);
    if !psStat.is_null() {
        if (*pPowerGen).capacity != 0 {
            baseOutput =
                (baseOutput as
                     libc::c_uint).wrapping_add((*(*(*psStat).asFuncList.offset(0
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize)
                                                       as
                                                       *mut POWER_GEN_FUNCTION)).powerOutput)
                    as UDWORD as UDWORD
        }
    }
    (*pPowerGen).multiplier =
        baseOutput.wrapping_add((*pPGFunc).powerMultiplier.wrapping_mul(asPowerUpgrade[(*psBuilding).player
                                                                                           as
                                                                                           usize].modifier
                                                                            as
                                                                            libc::c_uint).wrapping_div(100
                                                                                                           as
                                                                                                           libc::c_int
                                                                                                           as
                                                                                                           libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn structureRepairUpgrade(mut psBuilding:
                                                    *mut STRUCTURE) {
    let mut pRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut pRepairFunc: *mut REPAIR_DROID_FUNCTION =
        0 as *mut REPAIR_DROID_FUNCTION;
    //upgrade the research points
    pRepair = (*psBuilding).pFunctionality as *mut REPAIR_FACILITY;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureRepairUpgrade: invalid Repair pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1878 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"structureRepairUpgrade\x00")).as_ptr(),
              b"PTRVALID(pRepair, sizeof(REPAIR_FACILITY))\x00" as *const u8
                  as *const libc::c_char);
    };
    pRepairFunc =
        *(*(*psBuilding).pStructureType).asFuncList.offset(0 as libc::c_int as
                                                               isize) as
            *mut REPAIR_DROID_FUNCTION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureRepairUpgrade: invalid Function pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              1882 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"structureRepairUpgrade\x00")).as_ptr(),
              b"PTRVALID(pRepairFunc, sizeof(REPAIR_DROID_FUNCTION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    (*pRepair).power =
        (*pRepairFunc).repairPoints.wrapping_add((*pRepairFunc).repairPoints.wrapping_mul(asRepairFacUpgrade[(*psBuilding).player
                                                                                                                 as
                                                                                                                 usize].modifier
                                                                                              as
                                                                                              libc::c_uint).wrapping_div(100
                                                                                                                             as
                                                                                                                             libc::c_int
                                                                                                                             as
                                                                                                                             libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn structureSensorUpgrade(mut psBuilding:
                                                    *mut STRUCTURE) {
    //reallocate the sensor range and power since the upgrade
    if !(*(*psBuilding).pStructureType).pSensor.is_null() {
        (*psBuilding).sensorRange =
            sensorRange((*(*psBuilding).pStructureType).pSensor,
                        (*psBuilding).player) as UWORD;
        (*psBuilding).sensorPower =
            sensorPower((*(*psBuilding).pStructureType).pSensor,
                        (*psBuilding).player) as UWORD
    } else {
        //give them the default sensor for droids if not
        (*psBuilding).sensorRange =
            sensorRange(asSensorStats.offset(aDefaultSensor[(*psBuilding).player
                                                                as usize] as
                                                 isize), (*psBuilding).player)
                as UWORD;
        (*psBuilding).sensorPower =
            sensorPower(asSensorStats.offset(aDefaultSensor[(*psBuilding).player
                                                                as usize] as
                                                 isize), (*psBuilding).player)
                as UWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureECMUpgrade(mut psBuilding: *mut STRUCTURE) {
    //reallocate the sensor range and power since the upgrade
    if !(*(*psBuilding).pStructureType).pECM.is_null() {
        (*psBuilding).ecmPower =
            ecmPower((*(*psBuilding).pStructureType).pECM,
                     (*psBuilding).player) as UWORD
    } else { (*psBuilding).ecmPower = 0 as libc::c_int as UWORD };
}
#[no_mangle]
pub unsafe extern "C" fn droidSensorUpgrade(mut psDroid: *mut DROID) {
    //reallocate the sensor range and power since the upgrade
    (*psDroid).sensorRange =
        sensorRange(asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                               libc::c_int as
                                                               usize].nStat as
                                             libc::c_int as isize),
                    (*psDroid).player);
    (*psDroid).sensorPower =
        sensorPower(asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                               libc::c_int as
                                                               usize].nStat as
                                             libc::c_int as isize),
                    (*psDroid).player);
}
#[no_mangle]
pub unsafe extern "C" fn droidECMUpgrade(mut psDroid: *mut DROID) {
    //reallocate the ecm power since the upgrade
    (*psDroid).ECMMod =
        ecmPower(asECMStats.offset((*psDroid).asBits[COMP_ECM as libc::c_int
                                                         as usize].nStat as
                                       libc::c_int as isize),
                 (*psDroid).player);
}
#[no_mangle]
pub unsafe extern "C" fn droidBodyUpgrade(mut pFunction: *mut FUNCTION,
                                          mut psDroid: *mut DROID) {
    let mut increase: UDWORD = 0;
    let mut prevBaseBody: UDWORD = 0;
    let mut newBaseBody: UDWORD = 0;
    let mut base: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    increase =
        (*(pFunction as *mut DROIDBODY_UPGRADE_FUNCTION)).body as UDWORD;
    prevBaseBody = (*psDroid).originalBody;
    base = calcDroidBaseBody(psDroid);
    newBaseBody =
        base.wrapping_add(base.wrapping_mul(increase).wrapping_div(100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint));
    if newBaseBody > prevBaseBody {
        (*psDroid).body =
            (*psDroid).body.wrapping_mul(newBaseBody).wrapping_div(prevBaseBody);
        (*psDroid).originalBody = newBaseBody
    }
    //if a transporter droid then need to upgrade the contents
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        psCurr = (*(*psDroid).psGroup).psList;
        while !psCurr.is_null() {
            if psCurr != psDroid { droidBodyUpgrade(pFunction, psCurr); }
            psCurr = (*psCurr).psGrpNext
        }
    };
}
/*void armourUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding)
{
	psBuilding->armour = psBuilding->pStructureType->armourValue + (psBuilding->
		pStructureType->armourValue *((	ARMOUR_UPGRADE_FUNCTION *)pFunction)->
		armourPoints) / 100;
}*/
/*void repairUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding)
{
	psBuilding->repair = psBuilding->pStructureType->repairSystem + (psBuilding->
		pStructureType->repairSystem *((REPAIR_UPGRADE_FUNCTION *)pFunction)->
		repairPoints) / 100;
}*/
/*void bodyUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding)
{
	UDWORD	increment;

	increment = (psBuilding->pStructureType->bodyPoints *((
		BODY_UPGRADE_FUNCTION *)pFunction)->bodyPoints) / 100;

	psBuilding->body += increment;
	//upgrade the base body points
	psBuilding->baseBodyPoints += increment;
}*/
/*void resistanceUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding)
{
	psBuilding->resistance = psBuilding->pStructureType->resistance + (psBuilding->
		pStructureType->resistance *((RESISTANCE_UPGRADE_FUNCTION *)pFunction)->
		resistancePoints) / 100;

	//I'm not convinced this function is ever going to get used but if it does then
	we will have to keep a copy of the baseResistancePoints per Structure like we do
	with the body points, but I don't want to increase the size of STRUCTURE if we
	don't need to! AB 01/07/98
	ASSERT( TRUE,
		"resistanceUpgrade - for this function to work, there has to be a code change" );
}*/
//upgrade the weapon stats for the correct subclass
#[no_mangle]
pub unsafe extern "C" fn weaponUpgrade(mut pFunction: *mut FUNCTION,
                                       mut player: UBYTE) {
    let mut pUpgrade: *mut WEAPON_UPGRADE_FUNCTION =
        0 as *mut WEAPON_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut WEAPON_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].firePause as
            libc::c_int) < (*pUpgrade).firePause as libc::c_int {
        //make sure don't go less than 100%
        if (*pUpgrade).firePause as libc::c_int > 100 as libc::c_int {
            (*pUpgrade).firePause = 100 as libc::c_int as UBYTE
        }
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].firePause =
            (*pUpgrade).firePause
    }
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].shortHit as
            libc::c_int) < (*pUpgrade).shortHit as libc::c_int {
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].shortHit =
            (*pUpgrade).shortHit
    }
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].longHit as
            libc::c_int) < (*pUpgrade).longHit as libc::c_int {
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].longHit =
            (*pUpgrade).longHit
    }
    if (asWeaponUpgrade[player as usize][(*pUpgrade).subClass as usize].damage
            as libc::c_int) < (*pUpgrade).damage as libc::c_int {
        asWeaponUpgrade[player as usize][(*pUpgrade).subClass as usize].damage
            = (*pUpgrade).damage
    }
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].radiusDamage
            as libc::c_int) < (*pUpgrade).radiusDamage as libc::c_int {
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].radiusDamage
            = (*pUpgrade).radiusDamage
    }
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].incenDamage
            as libc::c_int) < (*pUpgrade).incenDamage as libc::c_int {
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].incenDamage
            = (*pUpgrade).incenDamage
    }
    if (asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].radiusHit as
            libc::c_int) < (*pUpgrade).radiusHit as libc::c_int {
        asWeaponUpgrade[player as
                            usize][(*pUpgrade).subClass as usize].radiusHit =
            (*pUpgrade).radiusHit
    };
}
//upgrade the sensor stats
#[no_mangle]
pub unsafe extern "C" fn sensorUpgrade(mut pFunction: *mut FUNCTION,
                                       mut player: UBYTE) {
    let mut pUpgrade: *mut DROIDSENSOR_UPGRADE_FUNCTION =
        0 as *mut DROIDSENSOR_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut DROIDSENSOR_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asSensorUpgrade[player as usize].range as libc::c_int) <
           (*pUpgrade).range as libc::c_int {
        asSensorUpgrade[player as usize].range = (*pUpgrade).range
    }
    if (asSensorUpgrade[player as usize].power as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asSensorUpgrade[player as usize].power = (*pUpgrade).upgradePoints
    };
}
//upgrade the repair stats
#[no_mangle]
pub unsafe extern "C" fn repairUpgrade(mut pFunction: *mut FUNCTION,
                                       mut player: UBYTE) {
    let mut pUpgrade: *mut DROIDREPAIR_UPGRADE_FUNCTION =
        0 as *mut DROIDREPAIR_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut DROIDREPAIR_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asRepairUpgrade[player as usize].repairPoints as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asRepairUpgrade[player as usize].repairPoints =
            (*pUpgrade).upgradePoints
    };
}
//upgrade the repair stats
#[no_mangle]
pub unsafe extern "C" fn ecmUpgrade(mut pFunction: *mut FUNCTION,
                                    mut player: UBYTE) {
    let mut pUpgrade: *mut DROIDECM_UPGRADE_FUNCTION =
        0 as *mut DROIDECM_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut DROIDECM_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asECMUpgrade[player as usize].power as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asECMUpgrade[player as usize].power = (*pUpgrade).upgradePoints
    };
}
//upgrade the repair stats
#[no_mangle]
pub unsafe extern "C" fn constructorUpgrade(mut pFunction: *mut FUNCTION,
                                            mut player: UBYTE) {
    let mut pUpgrade: *mut DROIDCONSTR_UPGRADE_FUNCTION =
        0 as *mut DROIDCONSTR_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut DROIDCONSTR_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asConstUpgrade[player as usize].constructPoints as libc::c_int) <
           (*pUpgrade).upgradePoints as libc::c_int {
        asConstUpgrade[player as usize].constructPoints =
            (*pUpgrade).upgradePoints
    };
}
//upgrade the body stats
#[no_mangle]
pub unsafe extern "C" fn bodyUpgrade(mut pFunction: *mut FUNCTION,
                                     mut player: UBYTE) {
    let mut pUpgrade: *mut DROIDBODY_UPGRADE_FUNCTION =
        0 as *mut DROIDBODY_UPGRADE_FUNCTION;
    let mut inc: UBYTE = 0;
    pUpgrade = pFunction as *mut DROIDBODY_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (*pUpgrade).droid != 0 {
        if (asBodyUpgrade[player as
                              usize][0 as libc::c_int as usize].powerOutput as
                libc::c_int) < (*pUpgrade).upgradePoints as libc::c_int {
            asBodyUpgrade[player as
                              usize][0 as libc::c_int as usize].powerOutput =
                (*pUpgrade).upgradePoints
        }
        if (asBodyUpgrade[player as usize][0 as libc::c_int as usize].body as
                libc::c_int) < (*pUpgrade).body as libc::c_int {
            asBodyUpgrade[player as usize][0 as libc::c_int as usize].body =
                (*pUpgrade).body
        }
        inc = 0 as libc::c_int as UBYTE;
        while (inc as libc::c_int) < NUM_WEAPON_CLASS as libc::c_int {
            if (asBodyUpgrade[player as
                                  usize][0 as libc::c_int as
                                             usize].armourValue[inc as usize]
                    as libc::c_int) <
                   (*pUpgrade).armourValue[inc as usize] as libc::c_int {
                asBodyUpgrade[player as
                                  usize][0 as libc::c_int as
                                             usize].armourValue[inc as usize]
                    = (*pUpgrade).armourValue[inc as usize]
            }
            inc = inc.wrapping_add(1)
        }
    }
    if (*pUpgrade).cyborg != 0 {
        if (asBodyUpgrade[player as
                              usize][1 as libc::c_int as usize].powerOutput as
                libc::c_int) < (*pUpgrade).upgradePoints as libc::c_int {
            asBodyUpgrade[player as
                              usize][1 as libc::c_int as usize].powerOutput =
                (*pUpgrade).upgradePoints
        }
        if (asBodyUpgrade[player as usize][1 as libc::c_int as usize].body as
                libc::c_int) < (*pUpgrade).body as libc::c_int {
            asBodyUpgrade[player as usize][1 as libc::c_int as usize].body =
                (*pUpgrade).body
        }
        inc = 0 as libc::c_int as UBYTE;
        while (inc as libc::c_int) < NUM_WEAPON_CLASS as libc::c_int {
            if (asBodyUpgrade[player as
                                  usize][1 as libc::c_int as
                                             usize].armourValue[inc as usize]
                    as libc::c_int) <
                   (*pUpgrade).armourValue[inc as usize] as libc::c_int {
                asBodyUpgrade[player as
                                  usize][1 as libc::c_int as
                                             usize].armourValue[inc as usize]
                    = (*pUpgrade).armourValue[inc as usize]
            }
            inc = inc.wrapping_add(1)
        }
    };
}
//upgrade the structure stats for the correct player
#[no_mangle]
pub unsafe extern "C" fn structureUpgrade(mut pFunction: *mut FUNCTION,
                                          mut player: UBYTE) {
    let mut pUpgrade: *mut STRUCTURE_UPGRADE_FUNCTION =
        0 as *mut STRUCTURE_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut STRUCTURE_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asStructureUpgrade[player as usize].armour as libc::c_int) <
           (*pUpgrade).armour as libc::c_int {
        asStructureUpgrade[player as usize].armour = (*pUpgrade).armour
    }
    if (asStructureUpgrade[player as usize].body as libc::c_int) <
           (*pUpgrade).body as libc::c_int {
        asStructureUpgrade[player as usize].body = (*pUpgrade).body
    }
    if (asStructureUpgrade[player as usize].resistance as libc::c_int) <
           (*pUpgrade).resistance as libc::c_int {
        asStructureUpgrade[player as usize].resistance =
            (*pUpgrade).resistance
    };
}
//upgrade the wall/Defence structure stats for the correct player
#[no_mangle]
pub unsafe extern "C" fn wallDefenceUpgrade(mut pFunction: *mut FUNCTION,
                                            mut player: UBYTE) {
    let mut pUpgrade: *mut WALLDEFENCE_UPGRADE_FUNCTION =
        0 as *mut WALLDEFENCE_UPGRADE_FUNCTION;
    pUpgrade = pFunction as *mut WALLDEFENCE_UPGRADE_FUNCTION;
    //check upgrades increase all values!
    if (asWallDefenceUpgrade[player as usize].armour as libc::c_int) <
           (*pUpgrade).armour as libc::c_int {
        asWallDefenceUpgrade[player as usize].armour = (*pUpgrade).armour
    }
    if (asWallDefenceUpgrade[player as usize].body as libc::c_int) <
           (*pUpgrade).body as libc::c_int {
        asWallDefenceUpgrade[player as usize].body = (*pUpgrade).body
    };
}
/*upgrades the droids inside a Transporter uwith the appropriate upgrade function*/
#[no_mangle]
pub unsafe extern "C" fn upgradeTransporterDroids(mut psTransporter:
                                                      *mut DROID,
                                                  mut pUpgradeFunction:
                                                      Option<unsafe extern "C" fn(_:
                                                                                      *mut DROID)
                                                                 -> ()>) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"upgradeTransporterUnits: invalid unit type\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              2217 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"upgradeTransporterDroids\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    //loop thru' each unit in the Transporter
    psCurr = (*(*psTransporter).psGroup).psList;
    while !psCurr.is_null() {
        if psCurr != psTransporter {
            //apply upgrade if not the transporter itself
            pUpgradeFunction.expect("non-null function pointer")(psCurr);
        }
        psCurr = (*psCurr).psGrpNext
    };
}
//extern BOOL loadFunction(char *pData, UDWORD functionType);
//extern BOOL loadDefensiveStructFunction(char *pData);
//extern BOOL loadArmourUpgradeFunction(char *pData);
//extern BOOL loadPowerRegFunction(char *pData);
//extern BOOL loadPowerRelayFunction(char *pData);
//extern BOOL loadRadarMapFunction(char *pData);
//extern BOOL loadRepairUpgradeFunction(char *pData);
//extern BOOL loadResistanceUpgradeFunction(char *pData);
//extern BOOL loadBodyUpgradeFunction(char *pData);
//extern void armourUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void repairUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void bodyUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
//extern void resistanceUpgrade(FUNCTION *pFunction, STRUCTURE *psBuilding);
#[no_mangle]
pub unsafe extern "C" fn FunctionShutDown() -> BOOL {
    let mut inc: UDWORD = 0; //, player;
    let mut pFunction: *mut FUNCTION = 0 as *mut FUNCTION;
    let mut pStartList: *mut *mut FUNCTION = asFunctions;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numFunctions {
        pFunction = *asFunctions;
        memFreeRelease((*pFunction).pName as *mut libc::c_void);
        (*pFunction).pName = 0 as *mut STRING;
        //#ifndef RESOURCE_NAMES
        memFreeRelease(pFunction as *mut libc::c_void);
        pFunction = 0 as *mut FUNCTION;
        asFunctions = asFunctions.offset(1);
        inc = inc.wrapping_add(1)
    }
    memFreeRelease(pStartList as *mut libc::c_void);
    pStartList = 0 as *mut *mut FUNCTION;
    //free the Upgrade lists
	/*for (player=0; player < MAX_PLAYERS; player++)
	{
		FREE(apProductionUpgrades[player]);
		//FREE(apBodyUpgrades[player]);
		//FREE(apRepairUpgrades[player]);
		//FREE(apResistanceUpgrades[player]);
		FREE(apResearchUpgrades[player]);
		//FREE(apArmourUpgrades[player]);
		//FREE(apWeaponUpgrades[player]);
	}*/
    return 1 as libc::c_int;
}
//lists the current Upgrade level that can be applied to a structure through research
//FUNCTION_UPGRADE		*apProductionUpgrades[MAX_PLAYERS];
//UDWORD					numProductionUpgrades;
//FUNCTION_UPGRADE		*apResearchUpgrades[MAX_PLAYERS];
//UDWORD					numResearchUpgrades;
//FUNCTION_UPGRADE		*apArmourUpgrades[MAX_PLAYERS];
//UDWORD					numArmourUpgrades;
//FUNCTION_UPGRADE		*apBodyUpgrades[MAX_PLAYERS];
//UDWORD					numBodyUpgrades;
//FUNCTION_UPGRADE		*apRepairUpgrades[MAX_PLAYERS];
//UDWORD					numRepairUpgrades;
//FUNCTION_UPGRADE		*apResistanceUpgrades[MAX_PLAYERS];
//UDWORD					numResistanceUpgrades;
//FUNCTION_UPGRADE		*apWeaponUpgrades[MAX_PLAYERS];
//UDWORD					numWeaponUpgrades;
/*Returns the Function type based on the string - used for reading in data */
/*Returns the Function type based on the string - used for reading in data */
unsafe extern "C" fn functionType(mut pType: *mut libc::c_char) -> UDWORD {
    if strcmp(pType, b"Production\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return PRODUCTION_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Production Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return PRODUCTION_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"Research\x00" as *const u8 as *const libc::c_char) == 0
       {
        return RESEARCH_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Research Upgrade\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return RESEARCH_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Power Generator\x00" as *const u8 as *const libc::c_char) == 0
       {
        return POWER_GEN_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"Resource\x00" as *const u8 as *const libc::c_char) == 0
       {
        return RESOURCE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"Repair Droid\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return REPAIR_DROID_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Weapon Upgrade\x00" as *const u8 as *const libc::c_char) == 0
       {
        return WEAPON_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"Wall Function\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WALL_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Structure Upgrade\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return STRUCTURE_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"WallDefence Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WALLDEFENCE_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"Power Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return POWER_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"Repair Upgrade\x00" as *const u8 as *const libc::c_char) == 0
       {
        return REPAIR_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"VehicleRepair Upgrade\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return DROIDREPAIR_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"VehicleECM Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return DROIDECM_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"VehicleConst Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return DROIDCONST_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"VehicleBody Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return DROIDBODY_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType,
              b"VehicleSensor Upgrade\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return DROIDSENSOR_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"ReArm\x00" as *const u8 as *const libc::c_char) == 0 {
        return REARM_TYPE as libc::c_int as UDWORD
    }
    if strcmp(pType, b"ReArm Upgrade\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return REARM_UPGRADE_TYPE as libc::c_int as UDWORD
    }
    /*if (!strcmp(pType,"Radar Map"))
	{
		return RADAR_MAP_TYPE;
	}*/
	/*if (!strcmp(pType,"Power Regulator"))
	{
		return POWER_REG_TYPE;
	}*/
	/*if (!strcmp(pType,"Power Relay"))
	{
		return POWER_RELAY_TYPE;
	}*/
	/*if (!strcmp(pType,"Defensive Structure"))
	{
		return DEFENSIVE_STRUCTURE_TYPE;
	}*/
	/*if (!strcmp(pType,"Armour Upgrade"))
	{
		return ARMOUR_UPGRADE_TYPE;
	}*/
	/*if (!strcmp(pType,"Repair Upgrade"))
	{
		return REPAIR_UPGRADE_TYPE;
	}*/
	/*if (!strcmp(pType,"Resistance Upgrade"))
	{
		return RESISTANCE_UPGRADE_TYPE;
	}*/
/*	if (!strcmp(pType,"Droid Design"))
	{
		return DROID_DESIGN_TYPE;
	}
	if (!strcmp(pType,"Map Marker"))
	{
		return MAP_MARKER_TYPE;
	}
	if (!strcmp(pType,"Sky Dome Map"))
	{
		return SKY_DOME_MAP_TYPE;
	}*/
	/*if (!strcmp(pType,"Body Upgrade"))
	{
		return BODY_UPGRADE_TYPE;
	}*/
	/*if (!strcmp(pType,"HQ"))
	{
		return HQ_TYPE;
	}*/
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Unknown Function Type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"function.c\x00" as *const u8 as *const libc::c_char,
              2402 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"functionType\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int as UDWORD;
}
