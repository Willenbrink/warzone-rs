use ::libc;
extern "C" {
    pub type _formation;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    // the structure that was last hit
    #[no_mangle]
    static mut psLastDroidHit: *mut DROID;
    // the structure that was last hit
    #[no_mangle]
    static mut psLastStructHit: *mut STRUCTURE;
    #[no_mangle]
    fn stackPushResult(type_0: INTERP_TYPE, data: SDWORD) -> BOOL;
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
    /* **************************************************************************/
/*
 * Projectile types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    // the last unit that did damage - used by script functions
    #[no_mangle]
    static mut g_pProjLastAttacker: *mut BASE_OBJECT;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut intLastWidget: UDWORD;
    /*
 * Group.h
 *
 * Link droids together into a group for AI etc.
 *
 */
    // standard group
    // command droid group
    // transporter group
    // list of droids in the group
    // the command droid of a command group
    // where the group should retreat to
    // initialise the group system
    // shutdown the group system
    // create a new group
    // add a droid to a group
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // remove a droid from a group
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    /* get current transporter (for script callbacks) */
    #[no_mangle]
    fn transporterGetScriptCurrent() -> *mut DROID;
    //used for Callbacks to say which topic was last researched
    #[no_mangle]
    static mut psCBLastResearch: *mut RESEARCH;
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
/* **********************************************************************************
 *
 * Stats structures type definitions
 */
/* Stats common to all droid components */
/* Basic stats */
/* technology level of the component */
/* Power required to build the component */
/* Time required to build the component */
/* Component's weight */
/* Component's hit points */
/* Space the component takes in the droid */
/* Component's body points */
/* flag to indicate whether this component can*/
/* be used in the design screen*/
/* The IMD to draw for this component */
/* Stats common to all components */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _comp_base_stats {
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
}
pub type COMP_BASE_STATS = _comp_base_stats;
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
pub type MOVE_CONTROL = _move_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct research_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub techCode: UBYTE,
    pub techLevel: TECH_LEVEL,
    pub subGroup: UWORD,
    pub researchPoints: UWORD,
    pub researchPower: UDWORD,
    pub keyTopic: UBYTE,
    pub storeCount: UBYTE,
    pub numPRRequired: UBYTE,
    pub pPRList: *mut UWORD,
    pub numStructures: UBYTE,
    pub pStructList: *mut UWORD,
    pub numFunctions: UBYTE,
    pub pFunctionList: *mut *mut _function,
    pub numRedStructs: UBYTE,
    pub pRedStructs: *mut UWORD,
    pub numRedArtefacts: UBYTE,
    pub pRedArtefacts: *mut *mut COMP_BASE_STATS,
    pub numStructResults: UBYTE,
    pub pStructureResults: *mut UWORD,
    pub numArteResults: UBYTE,
    pub pArtefactResults: *mut *mut COMP_BASE_STATS,
    pub pReplacedArtefacts: *mut *mut COMP_BASE_STATS,
    pub pViewData: *mut _viewdata,
    pub iconID: UWORD,
    pub psStat: *mut BASE_STATS,
    pub pIMD: *mut iIMDShape,
    pub pIMD2: *mut iIMDShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _viewdata {
    pub pName: *mut STRING,
    pub type_0: VIEW_TYPE,
    pub numText: UBYTE,
    pub ppTextMsg: *mut *mut STRING,
    pub pData: *mut libc::c_void,
}
pub type VIEW_TYPE = _view_type;
pub type _view_type = libc::c_uint;
pub const VIEW_TYPES: _view_type = 4;
pub const VIEW_RPLX: _view_type = 3;
pub const VIEW_PROX: _view_type = 2;
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
pub type RESEARCH = research_stats;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/* the 2nd IMD for base plates/turrets*/
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
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
pub type STRUCTURE = _structure;
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
pub type INTERP_TYPE = _interp_type;
/* The data needed within an object to run a script */
// The actual script to run
// The objects copy of the global variables
// Number of currently active triggers
// Whether to release the context when there are no triggers
// The Event initialisation data
// value chunk init values
// trigger chunk init values
// context chunk init values
/*
 * A currently active trigger.
 * If the type of the triggger == TR_PAUSE, the trigger number stored is the
 * index of the trigger to replace this one when the event restarts
 */
// enum - TRIGGER_TYPE
// ID numbers for each user type
pub type _scr_user_types = libc::c_uint;
// maximum possible type - should always be last
// NULL stats
pub const ST_MAXTYPE: _scr_user_types = 34;
//used so we can check for NULL templates
pub const ST_POINTER_S: _scr_user_types = 33;
//used so we can check for NULL objects etc
pub const ST_POINTER_T: _scr_user_types = 32;
// A research topic
//private types for game code - not for use in script
pub const ST_POINTER_O: _scr_user_types = 31;
// A group of droids
pub const ST_RESEARCH: _scr_user_types = 30;
// The name of a game level
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
// text string for display messages in tutorial
pub const ST_SOUND: _scr_user_types = 27;
// ID of a droid
pub const ST_TEXTSTRING: _scr_user_types = 26;
// feature stat type
pub const ST_DROIDID: _scr_user_types = 25;
// structure stat type
pub const ST_FEATURESTAT: _scr_user_types = 24;
/* A structure ID number (don't really 
											   need this since just a number?)*/
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
// Template object
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
// Component types
pub const ST_PROPULSION: _scr_user_types = 14;
// General component
pub const ST_BODY: _scr_user_types = 13;
// General stats type
pub const ST_COMPONENT: _scr_user_types = 12;
// Feature object
pub const ST_BASESTATS: _scr_user_types = 11;
// Structure object
pub const ST_FEATURE: _scr_user_types = 10;
// Droid object
pub const ST_STRUCTURE: _scr_user_types = 9;
// Base object
pub const ST_DROID: _scr_user_types = 8;
// Intelligence message ?? (6)
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_ATTACKED: _scr_callback_types = 38;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_MANURUN: _scr_callback_types = 24;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
pub type DROID_GROUP = _droid_group;
/*
 * ScriptCB.c
 *
 * functions to deal with parameterised script callback triggers.
 *
 */
// unit taken over..
#[no_mangle]
pub static mut psScrCBDroidTaken: *mut DROID =
    0 as *const DROID as *mut DROID;
// The pointer to the droid that was just built for a CALL_NEWDROID
#[no_mangle]
pub static mut psScrCBNewDroid: *mut DROID = 0 as *const DROID as *mut DROID;
#[no_mangle]
pub static mut psScrCBNewDroidFact: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
// id of factory that built it.
// the attacker and target for a CALL_ATTACKED
#[no_mangle]
pub static mut psScrCBAttacker: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
#[no_mangle]
pub static mut psScrCBTarget: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// alliance details
#[no_mangle]
pub static mut CBallFrom: UDWORD = 0;
#[no_mangle]
pub static mut CBallTo: UDWORD = 0;
//console callback stuff
//---------------------------
#[no_mangle]
pub static mut ConsolePlayer: SDWORD = -(2 as libc::c_int);
#[no_mangle]
pub static mut MultiMsgPlayerTo: SDWORD = -(2 as libc::c_int);
#[no_mangle]
pub static mut beaconX: SDWORD = -(1 as libc::c_int);
#[no_mangle]
pub static mut beaconY: SDWORD = -(1 as libc::c_int);
#[no_mangle]
pub static mut MultiMsgPlayerFrom: SDWORD = -(2 as libc::c_int);
#[no_mangle]
pub static mut ConsoleMsg: [libc::c_char; 255] =
    [69, 82, 82, 79, 82, 33, 33, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//Last console message
#[no_mangle]
pub static mut MultiplayMsg: [libc::c_char; 255] = [0; 255];
/*
 * ScriptCB.h
 *
 * functions to deal with parameterised script callback triggers.
 *
 */
//console callback stuff
//---------------------------
//Last console message
//Last multiplayer message
//for scrCBStructBuilt callback
//for scrCBStructBuilt callback
// The pointer to the droid that was just built for a CALL_NEWDROID
// deal with unit takover(2)
//Last multiplayer message
#[no_mangle]
pub unsafe extern "C" fn scrCBDroidTaken() -> BOOL {
    let mut ppsDroid: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(1 as libc::c_int,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsDroid as *mut *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if psScrCBDroidTaken.is_null() {
        triggered = 0 as libc::c_int;
        *ppsDroid = 0 as *mut DROID
    } else { triggered = 1 as libc::c_int; *ppsDroid = psScrCBDroidTaken }
    psScrCBDroidTaken = 0 as *mut DROID;
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Deal with a CALL_NEWDROID
// Deal with a CALL_NEWDROID
#[no_mangle]
pub unsafe extern "C" fn scrCBNewDroid() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ppsDroid: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut ppsStructure: *mut *mut STRUCTURE = 0 as *mut *mut STRUCTURE;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsDroid as *mut *mut *mut DROID,
                      0x100000 as libc::c_int | ST_STRUCTURE as libc::c_int,
                      &mut ppsStructure as *mut *mut *mut STRUCTURE) == 0 {
        return 0 as libc::c_int
    }
    if psScrCBNewDroid.is_null() {
        // eh? got called without setting the new droid
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBNewUnit: no unit has been set\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  91 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrCBNewDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsDroid = 0 as *mut DROID;
        *ppsStructure = 0 as *mut STRUCTURE
    } else if (*psScrCBNewDroid).player as libc::c_uint == player as UDWORD {
        triggered = 1 as libc::c_int;
        *ppsDroid = psScrCBNewDroid;
        *ppsStructure = psScrCBNewDroidFact
    }
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// the attacker and target for a CALL_ATTACKED
// Deal with a CALL_STRUCT_ATTACKED
// Deal with a CALL_STRUCT_ATTACKED
#[no_mangle]
pub unsafe extern "C" fn scrCBStructAttacked() -> BOOL {
    let mut player: SDWORD = 0; //, **ppsTarget;
    let mut ppsTarget: *mut *mut STRUCTURE = 0 as *mut *mut STRUCTURE;
    let mut ppsAttacker: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_STRUCTURE as libc::c_int,
                      &mut ppsTarget as *mut *mut *mut STRUCTURE,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsAttacker as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psLastStructHit.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBStructAttacked: no target has been set\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  128 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrCBStructAttacked\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut STRUCTURE
    } else if (*psLastStructHit).player as libc::c_uint == player as UDWORD {
        triggered = 1 as libc::c_int;
        *ppsAttacker = g_pProjLastAttacker;
        *ppsTarget = psLastStructHit
    } else {
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut STRUCTURE
    }
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Deal with a CALL_DROID_ATTACKED
// Deal with a CALL_DROID_ATTACKED
#[no_mangle]
pub unsafe extern "C" fn scrCBDroidAttacked() -> BOOL {
    let mut player: SDWORD = 0; //, **ppsTarget;
    let mut ppsTarget: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut ppsAttacker: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsTarget as *mut *mut *mut DROID,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsAttacker as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psLastDroidHit.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBUnitAttacked: no target has been set\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  171 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrCBDroidAttacked\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut DROID
    } else if (*psLastDroidHit).player as libc::c_uint == player as UDWORD {
        triggered = 1 as libc::c_int;
        *ppsAttacker = g_pProjLastAttacker;
        *ppsTarget = psLastDroidHit
    } else {
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut DROID
    }
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Deal with a CALL_ATTACKED
// Deal with a CALL_ATTACKED
#[no_mangle]
pub unsafe extern "C" fn scrCBAttacked() -> BOOL {
    let mut player: SDWORD = 0; //, **ppsTarget;
    let mut ppsTarget: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut ppsAttacker: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsTarget as *mut *mut *mut BASE_OBJECT,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsAttacker as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psScrCBTarget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBAttacked: no target has been set\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  214 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrCBAttacked\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut BASE_OBJECT
    } else if (*psScrCBTarget).player as libc::c_uint == player as UDWORD {
        triggered = 1 as libc::c_int;
        *ppsAttacker = g_pProjLastAttacker;
        *ppsTarget = psScrCBTarget
    } else {
        triggered = 0 as libc::c_int;
        *ppsAttacker = 0 as *mut BASE_OBJECT;
        *ppsTarget = 0 as *mut BASE_OBJECT
    }
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// deal with CALL_BUTTON_PRESSED
// The button id
// deal with CALL_BUTTON_PRESSED
#[no_mangle]
pub unsafe extern "C" fn scrCBButtonPressed() -> BOOL {
    let mut button: UDWORD = 0;
    let mut triggered: BOOL = 0 as libc::c_int;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut button as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if button == intLastWidget { triggered = 1 as libc::c_int }
    if stackPushResult(VAL_BOOL, triggered) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// the Droid that was selected for a CALL_DROID_SELECTED
#[no_mangle]
pub static mut psCBSelectedDroid: *mut DROID =
    0 as *const DROID as *mut DROID;
// the Droid that was selected for a CALL_DROID_SELECTED
// deal with CALL_DROID_SELECTED
// deal with CALL_DROID_SELECTED
#[no_mangle]
pub unsafe extern "C" fn scrCBDroidSelected() -> BOOL {
    let mut ppsDroid: *mut *mut DROID = 0 as *mut *mut DROID;
    if stackPopParams(1 as libc::c_int,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsDroid as *mut *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrSCUnitSelected: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
              280 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"scrCBDroidSelected\x00")).as_ptr(),
              b"PTRVALID(psCBSelectedDroid, sizeof(DROID))\x00" as *const u8
                  as *const libc::c_char);
    };
    *ppsDroid = psCBSelectedDroid;
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// the object that was last killed for a CALL_OBJ_DESTROYED
#[no_mangle]
pub static mut psCBObjDestroyed: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// the object that was last killed for a CALL_OBJ_DESTROYED
// deal with a CALL_OBJ_DESTROYED
// deal with a CALL_OBJ_DESTROYED
#[no_mangle]
pub unsafe extern "C" fn scrCBObjDestroyed() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ppsObj: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsObj as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if !psCBObjDestroyed.is_null() &&
           (*psCBObjDestroyed).player as libc::c_uint == player as UDWORD &&
           (*psCBObjDestroyed).type_0 as libc::c_uint !=
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        retval = 1 as libc::c_int;
        *ppsObj = psCBObjDestroyed
    } else { retval = 0 as libc::c_int; *ppsObj = 0 as *mut BASE_OBJECT }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// deal with a CALL_STRUCT_DESTROYED
// deal with a CALL_STRUCT_DESTROYED
#[no_mangle]
pub unsafe extern "C" fn scrCBStructDestroyed() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ppsObj: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_STRUCTURE as libc::c_int,
                      &mut ppsObj as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if !psCBObjDestroyed.is_null() &&
           (*psCBObjDestroyed).player as libc::c_uint == player as UDWORD &&
           (*psCBObjDestroyed).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        retval = 1 as libc::c_int;
        *ppsObj = psCBObjDestroyed
    } else { retval = 0 as libc::c_int; *ppsObj = 0 as *mut BASE_OBJECT }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// deal with a CALL_DROID_DESTROYED
// deal with a CALL_DROID_DESTROYED
#[no_mangle]
pub unsafe extern "C" fn scrCBDroidDestroyed() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ppsObj: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsObj as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if !psCBObjDestroyed.is_null() &&
           (*psCBObjDestroyed).player as libc::c_uint == player as UDWORD &&
           (*psCBObjDestroyed).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        retval = 1 as libc::c_int;
        *ppsObj = psCBObjDestroyed
    } else { retval = 0 as libc::c_int; *ppsObj = 0 as *mut BASE_OBJECT }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// deal with a CALL_FEATURE_DESTROYED
// deal with a CALL_FEATURE_DESTROYED
#[no_mangle]
pub unsafe extern "C" fn scrCBFeatureDestroyed() -> BOOL {
    let mut ppsObj: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut retval: BOOL = 0;
    if stackPopParams(1 as libc::c_int,
                      0x100000 as libc::c_int | ST_FEATURE as libc::c_int,
                      &mut ppsObj as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if !psCBObjDestroyed.is_null() {
        retval = 1 as libc::c_int;
        *ppsObj = psCBObjDestroyed
    } else { retval = 0 as libc::c_int; *ppsObj = 0 as *mut BASE_OBJECT }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// the last object to be seen for a CALL_OBJ_SEEN
#[no_mangle]
pub static mut psScrCBObjSeen: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// the object that saw psScrCBObjSeen for a CALL_OBJ_SEEN
#[no_mangle]
pub static mut psScrCBObjViewer: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// deal with all the object seen functions
#[no_mangle]
pub unsafe extern "C" fn scrCBObjectSeen(mut callback: SDWORD) -> BOOL {
    let mut ppsObj: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut ppsViewer: *mut *mut BASE_OBJECT = 0 as *mut *mut BASE_OBJECT;
    let mut player: SDWORD = 0;
    let mut retval: BOOL = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsObj as *mut *mut *mut BASE_OBJECT,
                      0x100000 as libc::c_int | ST_BASEOBJECT as libc::c_int,
                      &mut ppsViewer as *mut *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psScrCBObjSeen.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBObjectSeen: no object set\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  449 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrCBObjectSeen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    *ppsObj = 0 as *mut BASE_OBJECT;
    if !psScrCBObjViewer.is_null() &&
           (*psScrCBObjViewer).player as libc::c_int != player ||
           (*psScrCBObjSeen).visible[player as usize] == 0 {
        retval = 0 as libc::c_int
    } else if callback == CALL_DROID_SEEN as libc::c_int &&
                  (*psScrCBObjSeen).type_0 as libc::c_uint !=
                      OBJ_DROID as libc::c_int as libc::c_uint {
        retval = 0 as libc::c_int
    } else if callback == CALL_STRUCT_SEEN as libc::c_int &&
                  (*psScrCBObjSeen).type_0 as libc::c_uint !=
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        retval = 0 as libc::c_int
    } else if callback == CALL_FEATURE_SEEN as libc::c_int &&
                  (*psScrCBObjSeen).type_0 as libc::c_uint !=
                      OBJ_FEATURE as libc::c_int as libc::c_uint {
        retval = 0 as libc::c_int
    } else {
        retval = 1 as libc::c_int;
        *ppsObj = psScrCBObjSeen;
        *ppsViewer = psScrCBObjViewer
    }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// the last object to be seen for a CALL_OBJ_SEEN
// the object that saw psScrCBObjSeen for a CALL_OBJ_SEEN
// deal with a CALL_OBJ_SEEN
// deal with a CALL_OBJ_SEEN
#[no_mangle]
pub unsafe extern "C" fn scrCBObjSeen() -> BOOL {
    return scrCBObjectSeen(CALL_OBJ_SEEN as libc::c_int);
}
// deal with a CALL_DROID_SEEN
// deal with a CALL_DROID_SEEN
#[no_mangle]
pub unsafe extern "C" fn scrCBDroidSeen() -> BOOL {
    return scrCBObjectSeen(CALL_DROID_SEEN as libc::c_int);
}
// deal with a CALL_STRUCT_SEEN
// deal with a CALL_STRUCT_SEEN
#[no_mangle]
pub unsafe extern "C" fn scrCBStructSeen() -> BOOL {
    return scrCBObjectSeen(CALL_STRUCT_SEEN as libc::c_int);
}
// deal with a CALL_FEATURE_SEEN
// deal with a CALL_FEATURE_SEEN
#[no_mangle]
pub unsafe extern "C" fn scrCBFeatureSeen() -> BOOL {
    return scrCBObjectSeen(CALL_FEATURE_SEEN as libc::c_int);
}
// deal with a CALL_TRANSPORTER_OFFMAP
#[no_mangle]
pub unsafe extern "C" fn scrCBTransporterOffMap() -> BOOL {
    let mut player: SDWORD = 0;
    let mut retval: BOOL = 0;
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psTransporter = transporterGetScriptCurrent();
    if !psTransporter.is_null() &&
           (*psTransporter).player as libc::c_uint == player as UDWORD {
        retval = 1 as libc::c_int
    } else { retval = 0 as libc::c_int }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// deal with a CALL_TRANSPORTER_LANDED
#[no_mangle]
pub unsafe extern "C" fn scrCBTransporterLanded() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    psTransporter = transporterGetScriptCurrent();
    if psTransporter.is_null() ||
           (*psTransporter).player as libc::c_uint != player as UDWORD {
        retval = 0 as libc::c_int
    } else {
        /* if not selectedPlayer unload droids */
        if player as UDWORD != selectedPlayer {
            /* transfer droids from transporter group to current group */
            psDroid = (*(*psTransporter).psGroup).psList;
            while !psDroid.is_null() {
                psNext = (*psDroid).psGrpNext;
                if psDroid != psTransporter {
                    grpLeave((*psTransporter).psGroup, psDroid);
                    grpJoin(psGroup, psDroid);
                }
                psDroid = psNext
            }
        }
        retval = 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//extern BOOL scrCallBeacon(void);
#[no_mangle]
pub unsafe extern "C" fn scrCBTransporterLandedB() -> BOOL {
    let mut player: SDWORD = 0; //return landed transporter
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut retval: BOOL = 0;
    let mut ppsTransp: *mut *mut DROID = 0 as *mut *mut DROID;
    if stackPopParams(3 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsTransp as *mut *mut *mut DROID) == 0 {
        debug(LOG_ERROR,
              b"scrCBTransporterLandedB(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    psTransporter = transporterGetScriptCurrent();
    if psTransporter.is_null() ||
           (*psTransporter).player as libc::c_uint != player as UDWORD {
        retval = 0 as libc::c_int
    } else {
        *ppsTransp = psTransporter;
        /* if not selectedPlayer unload droids */
		//if ( (UDWORD)player != selectedPlayer )
		//{
			/* transfer droids from transporter group to current group */
        psDroid = (*(*psTransporter).psGroup).psList;
        while !psDroid.is_null() {
            psNext = (*psDroid).psGrpNext;
            if psDroid != psTransporter {
                grpLeave((*psTransporter).psGroup, psDroid);
                grpJoin(psGroup, psDroid);
            }
            psDroid = psNext
        }
        //}
        retval = 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, retval) == 0 {
        debug(LOG_ERROR,
              b"scrCBTransporterLandedB: push landed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// tell the scripts when a cluster is no longer valid
#[no_mangle]
pub static mut scrCBEmptyClusterID: SDWORD = 0;
// tell the scripts when a cluster is no longer valid
#[no_mangle]
pub unsafe extern "C" fn scrCBClusterEmpty() -> BOOL {
    let mut pClusterID: *mut SDWORD = 0 as *mut SDWORD;
    if stackPopParams(1 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pClusterID as *mut *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    *pClusterID = scrCBEmptyClusterID;
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// note when a vtol has finished returning to base - used to vanish
// vtols when they are attacking from off map
#[no_mangle]
pub static mut psScrCBVtolOffMap: *mut DROID =
    0 as *const DROID as *mut DROID;
// note when a vtol has finished returning to base - used to vanish
// vtols when they are attacking from off map
#[no_mangle]
pub unsafe extern "C" fn scrCBVtolOffMap() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ppsVtol: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsVtol as *mut *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if psScrCBVtolOffMap.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBVtolAtBase: NULL vtol pointer\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  683 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrCBVtolOffMap\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    retval = 0 as libc::c_int;
    if (*psScrCBVtolOffMap).player as libc::c_int == player {
        retval = 1 as libc::c_int;
        *ppsVtol = psScrCBVtolOffMap
    }
    psScrCBVtolOffMap = 0 as *mut DROID;
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*called when selectedPlayer completes some research*/
/*called when selectedPlayer completes some research*/
#[no_mangle]
pub unsafe extern "C" fn scrCBResCompleted() -> BOOL {
    let mut ppsResearch: *mut *mut RESEARCH = 0 as *mut *mut RESEARCH;
    let mut retVal: BOOL = 0;
    if stackPopParams(1 as libc::c_int,
                      0x100000 as libc::c_int | ST_RESEARCH as libc::c_int,
                      &mut ppsResearch as *mut *mut *mut RESEARCH) == 0 {
        return 0 as libc::c_int
    }
    if psCBLastResearch.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBResCompleted: no research has been set\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  716 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrCBResCompleted\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        retVal = 0 as libc::c_int;
        *ppsResearch = 0 as *mut RESEARCH
    } else { retVal = 1 as libc::c_int; *ppsResearch = psCBLastResearch }
    if stackPushResult(VAL_BOOL, retVal) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* when a player leaves the game*/
/* when a humna player leaves a game*/
#[no_mangle]
pub unsafe extern "C" fn scrCBPlayerLeft() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// alliance offered.
// alliance has been offered.
#[no_mangle]
pub unsafe extern "C" fn scrCBAllianceOffer() -> BOOL {
    let mut from: *mut SDWORD = 0 as *mut SDWORD;
    let mut to: *mut SDWORD = 0 as *mut SDWORD;
    if stackPopParams(2 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut from as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut to as *mut *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    *from = CBallFrom as SDWORD;
    *to = CBallTo as SDWORD;
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Console callback
//------------------------------------------------------------------------------------------------
										/* New callbacks */
//console callback
//---------------------------
#[no_mangle]
pub unsafe extern "C" fn scrCallConsole() -> BOOL {
    let mut player: *mut SDWORD = 0 as *mut SDWORD;
    let mut ConsoleText: *mut *mut STRING = 0 as *mut *mut STRING;
    if stackPopParams(2 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut player as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_STRING as libc::c_int,
                      &mut ConsoleText as *mut *mut *mut STRING) == 0 {
        debug(LOG_ERROR,
              b"scrCallConsole(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if (*ConsoleText).is_null() {
        debug(LOG_ERROR,
              b"scrCallConsole(): passed string was not initialized\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    strcpy(*ConsoleText, ConsoleMsg.as_mut_ptr());
    *player = ConsolePlayer;
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        debug(LOG_ERROR,
              b"scrCallConsole(): stackPushResult failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//multiplayer beacon
//---------------------------
/*
BOOL scrCallBeacon(void)
{
	SDWORD	*playerFrom, playerTo;
	STRING	**BeaconText = NULL;
	SDWORD	*locX,*locY;

	if (!stackPopParams(5, VAL_INT, &playerTo, VAL_REF | VAL_INT, &playerFrom,
		VAL_REF | VAL_INT, &locX, VAL_REF | VAL_INT, &locY,
		VAL_REF | VAL_STRING, &BeaconText)) 
	{
		MessageBox(frameGetWinHandle(), "scrCallBeacon() failed to pop parameters.", "failed", MB_OK);
		return FALSE;
	}

	

	//DbgMsg("x=%d,y=%d",locX >> TILE_SHIFT,locY >> TILE_SHIFT);
	if(*BeaconText == NULL)
	{
		MessageBox(frameGetWinHandle(), "scrCallBeacon(): passed string was not initialized", "failed", MB_OK);
		return FALSE;
	}

	//DbgMsg("scrCallMultiMsg");

	//if(MultiMsgPlayerTo == playerTo)
	if(MultiMsgPlayerTo >= 0 && MultiMsgPlayerFrom >= 0 && MultiMsgPlayerTo < MAX_PLAYERS && MultiMsgPlayerFrom < MAX_PLAYERS)
	{
		//DbgMsg("(%d - %d),  %d", MultiMsgPlayerTo, playerTo, MultiMsgPlayerFrom);

		if(MultiMsgPlayerTo == playerTo)
		{

			//DbgMsg("triggered!!!!!!!!!!!");

			strcpy(*BeaconText,MultiplayMsg);
	 
			*playerFrom = MultiMsgPlayerFrom;
			*locX = beaconX;
			*locY = beaconY;

			if (!stackPushResult(VAL_BOOL, TRUE))	//triggered
			{
				DbgMsg("scrCallBeacon - faled to push");
				return FALSE;
			}

			return TRUE;
		}
	}
	else
	{
		DbgMsg("scrCallBeacon() - player indexes failed: %d - %d", MultiMsgPlayerFrom, MultiMsgPlayerTo);
		if (!stackPushResult(VAL_BOOL, FALSE))	//not triggered
		{
			return FALSE;
		}

		return TRUE;
	}

	//return "not triggered"
	if (!stackPushResult(VAL_BOOL, FALSE))
	{
		return FALSE;
	}
	
	return TRUE;
}
*/
//multiplayer message callback
//----------------------------
#[no_mangle]
pub unsafe extern "C" fn scrCallMultiMsg() -> BOOL {
    let mut player: *mut SDWORD = 0 as *mut SDWORD;
    let mut playerTo: SDWORD = 0;
    let mut ConsoleText: *mut *mut STRING = 0 as *mut *mut STRING;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut playerTo as *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut player as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_STRING as libc::c_int,
                      &mut ConsoleText as *mut *mut *mut STRING) == 0 {
        debug(LOG_ERROR,
              b"scrCallMultiMsg() failed to pop parameters.\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if (*ConsoleText).is_null() {
        debug(LOG_ERROR,
              b"scrCallMultiMsg(): passed string was not initialized\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if MultiMsgPlayerTo >= 0 as libc::c_int &&
           MultiMsgPlayerFrom >= 0 as libc::c_int &&
           MultiMsgPlayerTo < 8 as libc::c_int &&
           MultiMsgPlayerFrom < 8 as libc::c_int {
        if MultiMsgPlayerTo == playerTo {
            strcpy(*ConsoleText, MultiplayMsg.as_mut_ptr());
            *player = MultiMsgPlayerFrom;
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                //triggered
                debug(LOG_ERROR,
                      b"scrCallMultiMsg(): stackPushResult failed\x00" as
                          *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    } else {
        debug(LOG_ERROR,
              b"scrCallMultiMsg() - player indexes failed: %d - %d\x00" as
                  *const u8 as *const libc::c_char, MultiMsgPlayerFrom,
              MultiMsgPlayerTo);
        if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
            //not triggered
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    //return "not triggered"
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        debug(LOG_ERROR,
              b"scrCallMultiMsg: stackPushResult failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut psScrCBNewStruct: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
//for scrCBStructBuilt callback
#[no_mangle]
pub static mut psScrCBNewStructTruck: *mut DROID =
    0 as *const DROID as *mut DROID;
//structure built callback
//------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrCBStructBuilt() -> BOOL {
    let mut player: SDWORD = 0; //pass to script
    let mut ppsStructure: *mut *mut STRUCTURE = 0 as *mut *mut STRUCTURE;
    let mut triggered: BOOL = 0 as libc::c_int;
    let mut ppsDroid: *mut *mut DROID = 0 as *mut *mut DROID;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      0x100000 as libc::c_int | ST_DROID as libc::c_int,
                      &mut ppsDroid as *mut *mut *mut DROID,
                      0x100000 as libc::c_int | ST_STRUCTURE as libc::c_int,
                      &mut ppsStructure as *mut *mut *mut STRUCTURE) == 0 {
        debug(LOG_ERROR,
              b"scrCBStructBuilt() failed to pop parameters.\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if psScrCBNewStruct.is_null() {
        debug(LOG_ERROR,
              b"scrCBStructBuilt: no structure has been set\x00" as *const u8
                  as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBStructBuilt: no structure has been set\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  961 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrCBStructBuilt\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsStructure = 0 as *mut STRUCTURE;
        *ppsDroid = 0 as *mut DROID
    } else if psScrCBNewStructTruck.is_null() {
        debug(LOG_ERROR,
              b"scrCBStructBuilt: no builder has been set\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCBStructBuilt: no builder has been set\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptcb.c\x00" as *const u8 as *const libc::c_char,
                  969 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrCBStructBuilt\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        triggered = 0 as libc::c_int;
        *ppsStructure = 0 as *mut STRUCTURE;
        *ppsDroid = 0 as *mut DROID
    } else if (*psScrCBNewStruct).player as libc::c_uint == player as UDWORD {
        triggered = 1 as libc::c_int;
        *ppsStructure = psScrCBNewStruct;
        *ppsDroid = psScrCBNewStructTruck
    }
    if stackPushResult(VAL_BOOL, triggered) == 0 {
        debug(LOG_ERROR,
              b"scrCBStructBuilt: push failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
