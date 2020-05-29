use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /*
 * Stats.h
 *
 * Interface to the common stats module
 *
 */
    /* *************************************************************************************
 *
 * Function prototypes and data storage for the stats
 */
    /* The stores for the different stats */
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    //extern POWER_STATS			*asPowerStats;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    //extern ARMOUR_STATS			*asArmourStats;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    // Pass in a stat and get its name
    #[no_mangle]
    fn getStatName(pStat: *mut libc::c_void) -> *mut STRING;
    /*Access functions for the upgradeable stats of a weapon*/
    #[no_mangle]
    fn weaponFirePause(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponShortHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponLongHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    /*Access functions for the upgradeable stats of a sensor*/
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    /*Access functions for the upgradeable stats of a ECM*/
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
    /*Access functions for the upgradeable stats of a repair*/
    #[no_mangle]
    fn repairPoints(psStats: *mut REPAIR_STATS, player: UBYTE) -> UDWORD;
    /*Access functions for the upgradeable stats of a constructor*/
    #[no_mangle]
    fn constructorPoints(psStats: *mut CONSTRUCT_STATS, player: UBYTE)
     -> UDWORD;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_stats {
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
    pub repairPoints: UDWORD,
    pub repairArmour: BOOL,
    pub location: UDWORD,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type REPAIR_STATS = _repair_stats;
pub type WEAPON_STATS = _weapon_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _construct_stats {
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
    pub constructPoints: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type CONSTRUCT_STATS = _construct_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
// The turret mount to use
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
//line build requires two sets of coords
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
//this structure is used to hold the permenant stats for each type of building
pub type STRUCTURE_STATS = _structure_stats;
pub type DROID = _droid;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
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
/*
 * oPrint.h
 *
 * Object information printing routines
 *
 */
// print out information about a base object
/*
 * oPrint.c
 *
 * Object information printing routines
 *
 */
//#define OPRINTF DBPRINTF
// print out information about a base object
#[no_mangle]
pub unsafe extern "C" fn printBaseObjInfo(mut psObj: *mut BASE_OBJECT) {
    let mut pType: *mut STRING = 0 as *mut STRING;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            pType =
                b"UNIT\x00" as *const u8 as *const libc::c_char as *mut STRING
        }
        1 => {
            pType =
                b"STRUCT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        2 => {
            pType =
                b"FEAT\x00" as *const u8 as *const libc::c_char as *mut STRING
        }
        _ => {
            pType =
                b"UNKNOWN TYPE\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
    }
    sprintf(ConsoleString.as_mut_ptr(),
            b"%s id %d at (%d,%d,%d) dpr (%d,%d,%d)\n\x00" as *const u8 as
                *const libc::c_char, pType, (*psObj).id,
            (*psObj).x as libc::c_int, (*psObj).y as libc::c_int,
            (*psObj).z as libc::c_int, (*psObj).direction as libc::c_int,
            (*psObj).pitch as libc::c_int, (*psObj).roll as libc::c_int);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// print out information about a general component
// print out information about a general component
#[no_mangle]
pub unsafe extern "C" fn printComponentInfo(mut psStats:
                                                *mut COMP_BASE_STATS) {
    psStats = psStats;
    sprintf(ConsoleString.as_mut_ptr(),
            b"%s ref %d tl %d\n   bPwr %d bPnts %d wt %d hp %d sp %d bdy %d imd %p\n\x00"
                as *const u8 as *const libc::c_char,
            getStatName(psStats as *mut libc::c_void), (*psStats).ref_0,
            (*psStats).techLevel as libc::c_uint, (*psStats).buildPower,
            (*psStats).buildPoints, (*psStats).weight, (*psStats).hitPoints,
            (*psStats).systemPoints, (*psStats).body, (*psStats).pIMD);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// print out weapon information
// print out weapon information
#[no_mangle]
pub unsafe extern "C" fn printWeaponInfo(mut psStats: *mut WEAPON_STATS) {
    let mut pWC: *mut STRING = 0 as *mut STRING;
    let mut pWSC: *mut STRING = 0 as *mut STRING;
    let mut pMM: *mut STRING = 0 as *mut STRING;
    match (*psStats).weaponClass as libc::c_uint {
        0 => {
            //bullets etc
            pWC =
                b"WC_KINETIC\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        1 => {
            //case WC_EXPLOSIVE:	//rockets etc
	//	pWC = "WC_EXPLOSIVE";
	//	break;
            //laser etc
            pWC =
                b"WC_HEAT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        _ => {
            //case WC_MISC:		//others we haven't thought of!
	//	pWC = "WC_MISC";
	//	break;
            pWC =
                b"UNKNOWN CLASS\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
    }
    match (*psStats).weaponSubClass as libc::c_uint {
        0 => {
            pWSC =
                b"WSC_MGUN\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        1 => {
            pWSC =
                b"WSC_CANNON\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        2 => {
            /*case WSC_ARTILLARY:
		pWSC = "WSC_ARTILLARY";
		break;*/
            pWSC =
                b"WSC_MORTARS\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        3 => {
            pWSC =
                b"WSC_MISSILE\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        4 => {
            pWSC =
                b"WSC_ROCKET\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        5 => {
            pWSC =
                b"WSC_ENERGY\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        6 => {
            pWSC =
                b"WSC_GAUSS\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        7 => {
            pWSC =
                b"WSC_FLAME\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        8 => {
            /*case WSC_CLOSECOMBAT:
		pWSC = "WSC_CLOSECOMBAT";
		break;*/
            pWSC =
                b"WSC_HOWITZERS\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        9 => {
            pWSC =
                b"WSC_ELECTRONIC\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        10 => {
            pWSC =
                b"WSC_AAGUN\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        11 => {
            pWSC =
                b"WSC_SLOWMISSILE\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        12 => {
            pWSC =
                b"WSC_SLOWROCKET\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        13 => {
            pWSC =
                b"WSC_LAS_SAT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        14 => {
            pWSC =
                b"WSC_BOMB\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        15 => {
            pWSC =
                b"WSC_COMMAND\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        16 => {
            pWSC =
                b"WSC_EMP\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        _ => {
            pWSC =
                b"UNKNOWN SUB CLASS\x00" as *const u8 as *const libc::c_char
                    as *mut STRING
        }
    }
    match (*psStats).movementModel as libc::c_uint {
        0 => {
            pMM =
                b"MM_DIRECT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        1 => {
            pMM =
                b"MM_INDIRECT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        2 => {
            pMM =
                b"MM_HOMINGDIRECT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        3 => {
            pMM =
                b"MM_HOMINGINDIRECT\x00" as *const u8 as *const libc::c_char
                    as *mut STRING
        }
        4 => {
            pMM =
                b"MM_ERRATICDIRECT\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        5 => {
            pMM =
                b"MM_SWEEP\x00" as *const u8 as *const libc::c_char as
                    *mut STRING
        }
        _ => {
            pMM =
                b"UNKNOWN MOVE MODEL\x00" as *const u8 as *const libc::c_char
                    as *mut STRING
        }
    }
    sprintf(ConsoleString.as_mut_ptr(),
            b"Weapon: \x00" as *const u8 as *const libc::c_char);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    printComponentInfo(psStats as *mut COMP_BASE_STATS);
    sprintf(ConsoleString.as_mut_ptr(),
            b"   sRng %d lRng %d mRng %d %s\n   sHt %d lHt %d pause %d dam %d\n\x00"
                as *const u8 as *const libc::c_char, (*psStats).shortRange,
            proj_GetLongRange(psStats, 0 as libc::c_int), (*psStats).minRange,
            if proj_Direct(psStats) != 0 {
                b"direct\x00" as *const u8 as *const libc::c_char
            } else { b"indirect\x00" as *const u8 as *const libc::c_char },
            weaponShortHit(psStats, selectedPlayer as UBYTE),
            weaponLongHit(psStats, selectedPlayer as UBYTE),
            weaponFirePause(psStats, selectedPlayer as UBYTE),
            weaponDamage(psStats, selectedPlayer as UBYTE));
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    sprintf(ConsoleString.as_mut_ptr(),
            b"   rad %d radHt %d radDam %d\n   inTime %d inDam %d inRad %d\n\x00"
                as *const u8 as *const libc::c_char, (*psStats).radius,
            (*psStats).radiusHit, (*psStats).radiusDamage,
            (*psStats).incenTime, (*psStats).incenDamage,
            (*psStats).incenRadius);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    sprintf(ConsoleString.as_mut_ptr(),
            b"   flSpd %d inHt %d %s\n\x00" as *const u8 as
                *const libc::c_char, (*psStats).flightSpeed,
            (*psStats).indirectHeight,
            if (*psStats).fireOnMove as libc::c_uint != 0 {
                b"fireOnMove\x00" as *const u8 as *const libc::c_char
            } else {
                b"not fireOnMove\x00" as *const u8 as *const libc::c_char
            });
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    sprintf(ConsoleString.as_mut_ptr(),
            b"   %s %s %s\n\x00" as *const u8 as *const libc::c_char, pWC,
            pWSC, pMM);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    /*OPRINTF(ConsoleString,(ConsoleString,"   %shoming %srotate recoil %d\n"
			  "   dLife %d radLife %d\n",
			  psStats->homingRound ? "" : "not ", psStats->rotate ? "" : "not ",
			  psStats->recoilValue, psStats->directLife, psStats->radiusLife));*/
    sprintf(ConsoleString.as_mut_ptr(),
            b"   %srotate recoil %d\n   dLife %d radLife %d\n\x00" as
                *const u8 as *const libc::c_char,
            if (*psStats).rotate as libc::c_int != 0 {
                b"\x00" as *const u8 as *const libc::c_char
            } else { b"not \x00" as *const u8 as *const libc::c_char },
            (*psStats).recoilValue, (*psStats).directLife,
            (*psStats).radiusLife);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// print out information about a droid and it's components
// print out information about a droid and it's components
#[no_mangle]
pub unsafe extern "C" fn printDroidInfo(mut psDroid: *mut DROID) {
    let mut i: SDWORD = 0;
    let mut psBdyStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut psECMStats: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut psSensStats: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    let mut psConstStats: *mut CONSTRUCT_STATS = 0 as *mut CONSTRUCT_STATS;
    let mut psRepairStats: *mut REPAIR_STATS = 0 as *mut REPAIR_STATS;
    printBaseObjInfo(psDroid as *mut BASE_OBJECT);
    sprintf(ConsoleString.as_mut_ptr(),
            b"   wt %d bSpeed %d sRng %d sPwr %d ECM %d bdy %d\n\x00" as
                *const u8 as *const libc::c_char, (*psDroid).weight,
            (*psDroid).baseSpeed, (*psDroid).sensorRange,
            (*psDroid).sensorPower, (*psDroid).ECMMod, (*psDroid).body);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    /*for(i=0; i<(SDWORD)psDroid->numWeaps; i++)
	{
		printWeaponInfo(asWeaponStats + psDroid->asWeaps[i].nStat);
	}*/
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        printWeaponInfo(asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                                 as isize));
    }
    i = 0 as libc::c_int;
    while i < COMP_NUMCOMPONENTS as libc::c_int - 1 as libc::c_int {
        match i {
            1 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"Body: \x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psBdyStats =
                        asBodyStats.offset((*psDroid).asBits[i as usize].nStat
                                               as libc::c_int as isize);
                    printComponentInfo(psBdyStats as *mut COMP_BASE_STATS);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"ZNULL BODY\n\x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            3 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"Prop: \x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psPropStats =
                        asPropulsionStats.offset((*psDroid).asBits[i as
                                                                       usize].nStat
                                                     as libc::c_int as isize);
                    printComponentInfo(psPropStats as *mut COMP_BASE_STATS);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"ZNULL PROPULSION\n\x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            5 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"ECM: \x00" as *const u8 as *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psECMStats =
                        asECMStats.offset((*psDroid).asBits[i as usize].nStat
                                              as libc::c_int as isize);
                    printComponentInfo(psECMStats as *mut COMP_BASE_STATS);
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"   pwr %d loc %d imd %p\n\x00" as *const u8 as
                                *const libc::c_char,
                            ecmPower(psECMStats, (*psDroid).player),
                            (*psECMStats).location,
                            (*psECMStats).pMountGraphic);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"ZNULL ECM\n\x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            6 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"Sensor: \x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psSensStats =
                        asSensorStats.offset((*psDroid).asBits[i as
                                                                   usize].nStat
                                                 as libc::c_int as isize);
                    printComponentInfo(psSensStats as *mut COMP_BASE_STATS);
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"   rng %d pwr %d loc %d imd %p\n\x00" as
                                *const u8 as *const libc::c_char,
                            sensorRange(psSensStats, (*psDroid).player),
                            sensorPower(psSensStats, (*psDroid).player),
                            (*psSensStats).location,
                            (*psSensStats).pMountGraphic);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"ZNULL SENSOR\n\x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            7 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"Construct: \x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psConstStats =
                        asConstructStats.offset((*psDroid).asBits[i as
                                                                      usize].nStat
                                                    as libc::c_int as isize);
                    printComponentInfo(psConstStats as *mut COMP_BASE_STATS);
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"   cPnts %d imd %p\n\x00" as *const u8 as
                                *const libc::c_char,
                            constructorPoints(psConstStats,
                                              (*psDroid).player),
                            (*psConstStats).pMountGraphic);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            4 => {
                if (*psDroid).asBits[i as usize].nStat as libc::c_int >
                       0 as libc::c_int {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"Repair: \x00" as *const u8 as
                                *const libc::c_char);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    psRepairStats =
                        asRepairStats.offset((*psDroid).asBits[i as
                                                                   usize].nStat
                                                 as libc::c_int as isize);
                    printComponentInfo(psRepairStats as *mut COMP_BASE_STATS);
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"   repPnts %d loc %d imd %p\n\x00" as *const u8
                                as *const libc::c_char,
                            repairPoints(psRepairStats, (*psDroid).player),
                            (*psRepairStats).location,
                            (*psRepairStats).pMountGraphic);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
            }
            0 | 2 | _ => { }
        }
        i += 1
    };
}
