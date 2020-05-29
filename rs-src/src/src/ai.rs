use ::libc;
extern "C" {
    pub type _formation;
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
    fn frameGetFrameNumber() -> UDWORD;
    /* Store for the objects near the droid currently being updated */
    #[no_mangle]
    static mut asDroidNaybors: [NAYBOR_INFO; 100];
    #[no_mangle]
    static mut numNaybors: UDWORD;
    // refresh the naybor list
// this only does anything if the naybor list is out of date
    #[no_mangle]
    fn droidGetNaybors(psDroid: *mut DROID);
    /*checks to see if an electronic warfare weapon is attached to the droid*/
    #[no_mangle]
    fn electronicDroid(psDroid: *mut DROID) -> BOOL;
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    /*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
    #[no_mangle]
    fn vtolEmpty(psDroid: *mut DROID) -> BOOL;
    /* EW works differently in multiplayer mode compared with single player.*/
    #[no_mangle]
    fn validStructResistance(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a Standard Turret sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structStandardSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a VTOL Intercept sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structVTOLSensor(psStruct: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asPropulsionTypes: *mut PROPULSION_TYPES;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /*
 * Player.h
 *
 * Update code for computer player AI
 *
 */
    /* Initialise the player AI system */
    #[no_mangle]
    fn playerInitialise() -> BOOL;
    /* Shutdown the player AI system */
    #[no_mangle]
    fn playerShutDown();
    /* Check the order state of a droid */
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn secondaryGetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         pState: *mut SECONDARY_STATE) -> BOOL;
    // initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
    #[no_mangle]
    fn gridStartIterate(x: SDWORD, y: SDWORD);
    // get the next object that could affect a location,
// should only be called after gridStartIterate
    #[no_mangle]
    fn gridIterate() -> *mut BASE_OBJECT;
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    static mut psDrivenDroid: *mut DROID;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    // return the current target designator for a player
    #[no_mangle]
    fn cmdDroidGetDesignator(player: UDWORD) -> *mut DROID;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
}
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
/* LOC used for holding locations for Sensors and ECM's*/
pub type _loc = libc::c_uint;
pub const LOC_TURRET: _loc = 1;
pub const LOC_DEFAULT: _loc = 0;
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
pub type WEAPON_STATS = _weapon_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
/* ***********************************************************************************
*	Additional stats tables
************************************************************************************/
pub type _travel_medium = libc::c_uint;
pub const AIR: _travel_medium = 1;
pub const GROUND: _travel_medium = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_types {
    pub powerRatioMult: UWORD,
    pub travel: UDWORD,
    pub startID: SWORD,
    pub idleID: SWORD,
    pub moveOffID: SWORD,
    pub moveID: SWORD,
    pub hissID: SWORD,
    pub shutDownID: SWORD,
}
pub type PROPULSION_TYPES = _propulsion_types;
//sound to play when this prop type shuts down
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
pub type STRUCTURE = _structure;
/* The range for neighbouring objects */
//used to stop structures being built too near the edge and droids being placed down - pickATile
/* Info stored for each droid neighbour */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _naybor_info {
    pub psObj: *mut BASE_OBJECT,
    pub distSqr: UDWORD,
}
pub type NAYBOR_INFO = _naybor_info;
// The square of the distance to the object
//UDWORD			dist;			// The distance to the object
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
pub const DACTION_ATTACK: _droid_action = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERGAME {
    pub type_0: UBYTE,
    pub map: [STRING; 128],
    pub version: [libc::c_char; 8],
    pub maxPlayers: UBYTE,
    pub name: [STRING; 128],
    pub bComputerPlayers: BOOL,
    pub fog: BOOL,
    pub power: UDWORD,
    pub base: UBYTE,
    pub alliance: UBYTE,
    pub limit: UBYTE,
    pub bytesPerSec: UWORD,
    pub packetsPerSec: UBYTE,
    pub encryptKey: UBYTE,
    pub skDiff: [UBYTE; 8],
}
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
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
// the state of secondary orders
pub type SECONDARY_STATE = _secondary_state;
pub type _secondary_state = libc::c_uint;
pub const DSS_VTOLPROD_END: _secondary_state = 268435456;
pub const DSS_VTOLPROD_START: _secondary_state = 16777216;
pub const DSS_FIREDES_SET: _secondary_state = 8388608;
pub const DSS_PATROL_SET: _secondary_state = 4194304;
pub const DSS_RTL_TRANSPORT: _secondary_state = 2097152;
pub const DSS_RTL_BASE: _secondary_state = 1048576;
pub const DSS_RTL_REPAIR: _secondary_state = 524288;
pub const DSS_ASSPROD_END: _secondary_state = 262144;
pub const DSS_ASSPROD_MID: _secondary_state = 8192;
pub const DSS_ASSPROD_START: _secondary_state = 512;
pub const DSS_RECYCLE_SET: _secondary_state = 256;
pub const DSS_HALT_PERSUE: _secondary_state = 192;
pub const DSS_HALT_GUARD: _secondary_state = 128;
pub const DSS_HALT_HOLD: _secondary_state = 64;
pub const DSS_ALEV_NEVER: _secondary_state = 48;
pub const DSS_ALEV_ATTACKED: _secondary_state = 32;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const DSS_ARANGE_LONG: _secondary_state = 2;
pub const DSS_ARANGE_SHORT: _secondary_state = 1;
// secondary orders for droids
pub type SECONDARY_ORDER = _secondary_order;
pub type _secondary_order = libc::c_uint;
pub const DSO_ASSIGN_VTOL_PRODUCTION: _secondary_order = 11;
pub const DSO_FIRE_DESIGNATOR: _secondary_order = 10;
pub const DSO_RETURN_TO_LOC: _secondary_order = 9;
pub const DSO_HALTTYPE: _secondary_order = 8;
pub const DSO_PATROL: _secondary_order = 7;
pub const DSO_RECYCLE: _secondary_order = 6;
pub const DSO_CLEAR_PRODUCTION: _secondary_order = 5;
pub const DSO_ASSIGN_CYBORG_PRODUCTION: _secondary_order = 4;
pub const DSO_ASSIGN_PRODUCTION: _secondary_order = 3;
pub const DSO_ATTACK_LEVEL: _secondary_order = 2;
pub const DSO_REPAIR_LEVEL: _secondary_order = 1;
pub const DSO_ATTACK_RANGE: _secondary_order = 0;
pub const DACTION_NONE: _droid_action = 0;
pub const GT_COMMAND: _group_type = 1;
pub const DACTION_SULK: _droid_action = 9;
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
pub const DACTION_FIRESUPPORT: _droid_action = 8;
pub const DACTION_OBSERVE: _droid_action = 7;
pub const DACTION_REPAIR: _droid_action = 5;
pub const DACTION_DEMOLISH: _droid_action = 4;
pub const DACTION_BUILD_FOUNDATION: _droid_action = 3;
pub const DACTION_BUILD: _droid_action = 2;
pub const DACTION_MOVE: _droid_action = 1;
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_NORMAL: _group_type = 0;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
//BOOL driveHasDriven(void);
//BOOL driveModeActive(void);
//BOOL driveIsDriven(DROID *psDroid);
//BOOL driveIsFollower(DROID *psDroid);
#[inline]
unsafe extern "C" fn driveGetDriven() -> *mut DROID { return psDrivenDroid; }
/*
 * AI.c
 *
 * AI update functions for the different object types.
 *
 */
/* Droid attack printf's */
// alliances
#[no_mangle]
pub static mut alliances: [[UBYTE; 8]; 8] = [[0; 8]; 8];
/* alliance code for ai. return true if an alliance has formed. */
#[no_mangle]
pub unsafe extern "C" fn aiCheckAlliances(mut s1: UDWORD, mut s2: UDWORD)
 -> BOOL {
    //features have their player number set to (MAX_PLAYERS + 1)
    if s1 == (8 as libc::c_int + 1 as libc::c_int) as libc::c_uint ||
           s2 == (8 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        return 0 as libc::c_int
    }
    if s1 == s2 ||
           alliances[s1 as usize][s2 as usize] as libc::c_int ==
               3 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* Initialise the AI system */
#[no_mangle]
pub unsafe extern "C" fn aiInitialise() -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            alliances[i as usize][j as usize] = 0 as libc::c_int as UBYTE;
            j += 1
        }
        i += 1
    }
    if playerInitialise() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* Shutdown the AI system */
#[no_mangle]
pub unsafe extern "C" fn aiShutdown() -> BOOL {
    playerShutDown();
    return 1 as libc::c_int;
}
// Find the nearest target to a droid
#[no_mangle]
pub unsafe extern "C" fn aiNearestTarget(mut psDroid: *mut DROID,
                                         mut ppsObj: *mut *mut BASE_OBJECT)
 -> BOOL {
    let mut i: UDWORD = 0;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut electronic: BOOL = 0 as libc::c_int;
    if (*psDroid).id.wrapping_rem(8 as libc::c_int as libc::c_uint) ==
           frameGetFrameNumber().wrapping_rem(8 as libc::c_int as
                                                  libc::c_uint) {
        droidGetNaybors(psDroid);
    } else { return 0 as libc::c_int }
    //don't bother looking if empty vtol droid
    if vtolEmpty(psDroid) != 0 { return 0 as libc::c_int }
    //electronic warfare can only be used against structures at present - not any more! AB 6/11/98
    electronic = electronicDroid(psDroid);
    psTarget = 0 as *mut BASE_OBJECT;
    i = 0 as libc::c_int as UDWORD;
    while i < numNaybors {
        if (*asDroidNaybors[i as usize].psObj).player as libc::c_int !=
               (*psDroid).player as libc::c_int &&
               ((*asDroidNaybors[i as usize].psObj).type_0 as libc::c_uint ==
                    OBJ_DROID as libc::c_int as libc::c_uint ||
                    (*asDroidNaybors[i as usize].psObj).type_0 as libc::c_uint
                        == OBJ_STRUCTURE as libc::c_int as libc::c_uint) &&
               (*asDroidNaybors[i as
                                    usize].psObj).visible[(*psDroid).player as
                                                              usize] as
                   libc::c_int != 0 &&
               aiCheckAlliances((*asDroidNaybors[i as usize].psObj).player as
                                    UDWORD, (*psDroid).player as UDWORD) == 0
           {
            psObj = asDroidNaybors[i as usize].psObj;
            if !(validTarget(psDroid as *mut BASE_OBJECT, psObj) == 0) {
                if (*psObj).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                    //in multiPlayer - don't attack Transporters with EW
                    if bMultiPlayer != 0 {
                        //if not electronic then valid target
                        if electronic == 0 ||
                               electronic != 0 &&
                                   (*(psObj as *mut DROID)).droidType as
                                       libc::c_uint !=
                                       DROID_TRANSPORTER as libc::c_int as
                                           libc::c_uint {
                            //only a valid target if NOT a transporter
                            psTarget = psObj;
                            break ;
                        }
                    } else { psTarget = psObj; break ; }
                } else if (*psObj).type_0 as libc::c_uint ==
                              OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                    if electronic != 0 {
                        /*don't want to target structures with resistance of zero if
                    using electronic warfare*/
//					if (((STRUCTURE *)psObj)->pStructureType->resistance != 0)// AND
						//((STRUCTURE *)psObj)->resistance >= (SDWORD)(((STRUCTURE *)
						//psObj)->pStructureType->resistance))
						//((STRUCTURE *)psObj)->resistance >= (SDWORD)
						//structureResistance(((STRUCTURE *)psObj)->pStructureType,
						//psObj->player))
                        if validStructResistance(psObj as *mut STRUCTURE) != 0
                           {
                            psTarget = psObj;
                            break ;
                        }
                    } else if (*(psObj as
                                     *mut STRUCTURE)).asWeaps[0 as libc::c_int
                                                                  as
                                                                  usize].nStat
                                  > 0 as libc::c_int as libc::c_uint {
                        //else if (((STRUCTURE *)psObj)->numWeaps > 0)
                        // structure with weapons - go for this
                        psTarget = psObj;
                        break ;
                    } else if (*(*(psObj as
                                       *mut STRUCTURE)).pStructureType).type_0
                                  != REF_WALL as libc::c_int as libc::c_uint
                                  &&
                                  (*(*(psObj as
                                           *mut STRUCTURE)).pStructureType).type_0
                                      !=
                                      REF_WALLCORNER as libc::c_int as
                                          libc::c_uint ||
                                  driveModeActive() != 0 ||
                                  bMultiPlayer != 0 &&
                                      game.type_0 as libc::c_int ==
                                          14 as libc::c_int &&
                                      isHumanPlayer((*psDroid).player as
                                                        UDWORD) == 0 {
                        psTarget = psObj
                    }
                }
            }
        }
        i = i.wrapping_add(1)
    }
    if !psTarget.is_null() { *ppsObj = psTarget; return 1 as libc::c_int }
    return 0 as libc::c_int;
}
// see if a structure has the range to fire on a target
#[no_mangle]
pub unsafe extern "C" fn aiStructHasRange(mut psStruct: *mut STRUCTURE,
                                          mut psTarget: *mut BASE_OBJECT)
 -> BOOL {
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut longRange: SDWORD = 0;
    if (*psStruct).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        // Can't attack without a weapon
        return 0 as libc::c_int
    }
    psWStats =
        asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int as
                                                     usize].nStat as isize);
    xdiff = (*psStruct).x as SDWORD - (*psTarget).x as SDWORD;
    ydiff = (*psStruct).y as SDWORD - (*psTarget).y as SDWORD;
    longRange = proj_GetLongRange(psWStats, 0 as libc::c_int);
    if xdiff * xdiff + ydiff * ydiff < longRange * longRange {
        // in range
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// see if an object is a wall
#[no_mangle]
pub unsafe extern "C" fn aiObjIsWall(mut psObj: *mut BASE_OBJECT) -> BOOL {
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if (*(*(psObj as *mut STRUCTURE)).pStructureType).type_0 !=
           REF_WALL as libc::c_int as libc::c_uint &&
           (*(*(psObj as *mut STRUCTURE)).pStructureType).type_0 !=
               REF_WALLCORNER as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* See if there is a target in range */
#[no_mangle]
pub unsafe extern "C" fn aiChooseTarget(mut psObj: *mut BASE_OBJECT,
                                        mut ppsTarget: *mut *mut BASE_OBJECT)
 -> BOOL {
    let mut radSquared: UDWORD = 0;
    //	UDWORD	player;
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT; //, longRange;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut tarDist: SDWORD = 0;
    let mut minDist: SDWORD = 0;
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut bCBTower: BOOL = 0;
    let mut psCStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psCommander: *mut DROID = 0 as *mut DROID;
    let mut bCommanderBlock: BOOL = 0;
    let mut sensorRange: UDWORD = 0;
    /* Get the sensor range */
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            //if (((DROID *)psObj)->numWeaps == 0)
            if (*(psObj as
                      *mut DROID)).asWeaps[0 as libc::c_int as usize].nStat ==
                   0 as libc::c_int as libc::c_uint &&
                   (*(psObj as *mut DROID)).droidType as libc::c_uint !=
                       DROID_SENSOR as libc::c_int as libc::c_uint {
                // Can't attack without a weapon
                return 0 as libc::c_int
            }
            radSquared =
                (*(psObj as
                       *mut DROID)).sensorRange.wrapping_mul((*(psObj as
                                                                    *mut DROID)).sensorRange)
        }
        1 => {
            //if (((STRUCTURE *)psObj)->numWeaps == 0)
            if (*(psObj as
                      *mut STRUCTURE)).asWeaps[0 as libc::c_int as
                                                   usize].nStat ==
                   0 as libc::c_int as libc::c_uint {
                // Can't attack without a weapon
                return 0 as libc::c_int
            }
            sensorRange = (*(psObj as *mut STRUCTURE)).sensorRange as UDWORD;
            // increase the sensor range for AA sites
		// AA sites are defensive structures that can only shoot in the air
            if (*(*(psObj as *mut STRUCTURE)).pStructureType).type_0 ==
                   REF_DEFENSE as libc::c_int as libc::c_uint &&
                   (*asWeaponStats.offset((*(psObj as
                                                 *mut STRUCTURE)).asWeaps[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize].nStat
                                              as isize)).surfaceToAir as
                       libc::c_int == 0x2 as libc::c_int {
                sensorRange =
                    (3 as libc::c_int as
                         libc::c_uint).wrapping_mul(sensorRange).wrapping_div(2
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
            }
            radSquared = sensorRange.wrapping_mul(sensorRange)
        }
        _ => { radSquared = 0 as libc::c_int as UDWORD }
    }
    /* See if there is a something in range */
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        if aiNearestTarget(psObj as *mut DROID, &mut psTarget) != 0 {
            /*check its a valid target*/
            if validTarget(psObj, psTarget) != 0 {
                /* See if in sensor range */
                xdiff =
                    (*psTarget).x as libc::c_int - (*psObj).x as libc::c_int;
                ydiff =
                    (*psTarget).y as libc::c_int - (*psObj).y as libc::c_int;
                if xdiff * xdiff + ydiff * ydiff < radSquared as SDWORD {
                    *ppsTarget = psTarget;
                    return 1 as libc::c_int
                }
            }
        }
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        //ASSERT( ((STRUCTURE *)psObj)->numWeaps > 0,
        if (*(psObj as
                  *mut STRUCTURE)).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"aiChooseTarget: no weapons on structure\x00" as *const u8
                      as *const libc::c_char);
        };
        if (*(psObj as
                  *mut STRUCTURE)).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"ai.c\x00" as *const u8 as *const libc::c_char,
                  307 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"aiChooseTarget\x00")).as_ptr(),
                  b"((STRUCTURE *)psObj)->asWeaps[0].nStat > 0\x00" as
                      *const u8 as *const libc::c_char);
        };
        psWStats =
            asWeaponStats.offset((*(psObj as
                                        *mut STRUCTURE)).asWeaps[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].nStat
                                     as isize);
        // see if there is a target from the command droids
        psTarget = 0 as *mut BASE_OBJECT;
        psCommander = cmdDroidGetDesignator((*psObj).player as UDWORD);
        bCommanderBlock = 0 as libc::c_int;
        if proj_Direct(psWStats) == 0 && !psCommander.is_null() &&
               aiStructHasRange(psObj as *mut STRUCTURE,
                                psCommander as *mut BASE_OBJECT) != 0 {
            // there is a commander that can fire designate for this structure
			// set bCommanderBlock so that the structure does not fire until the commander
			// has a target - (slow firing weapons will not be ready to fire otherwise).
            bCommanderBlock = 1 as libc::c_int;
            if (*psCommander).action == DACTION_ATTACK as libc::c_int &&
                   !(*psCommander).psActionTarget.is_null() {
                // the commander has a target to fire on
                if aiStructHasRange(psObj as *mut STRUCTURE,
                                    (*psCommander).psActionTarget) != 0 {
                    // target in range - fire on it
                    psTarget = (*psCommander).psActionTarget
                } else {
                    // target out of range - release the commander block
                    bCommanderBlock = 0 as libc::c_int
                }
            }
        }
        // indirect fire structures use sensor towers first
        tarDist = 0x7fffffff as libc::c_int;
        minDist =
            (*psWStats).minRange.wrapping_mul((*psWStats).minRange) as SDWORD;
        bCBTower = 0 as libc::c_int;
        if psTarget.is_null() && bCommanderBlock == 0 &&
               proj_Direct(psWStats) == 0 {
            psCStruct = apsStructLists[(*psObj).player as usize];
            while !psCStruct.is_null() {
                // skip incomplete structures
                if !((*psCStruct).status as libc::c_int !=
                         SS_BUILT as libc::c_int) {
                    if bCBTower == 0 && structStandardSensor(psCStruct) != 0
                           && !(*psCStruct).psTarget.is_null() {
                        /*check its a valid target*/
                        if validTarget(psObj, (*psCStruct).psTarget) != 0 &&
                               aiStructHasRange(psObj as *mut STRUCTURE,
                                                (*psCStruct).psTarget) != 0 {
                            xdiff =
                                (*(*psCStruct).psTarget).x as SDWORD -
                                    (*psObj).x as SDWORD;
                            ydiff =
                                (*(*psCStruct).psTarget).y as SDWORD -
                                    (*psObj).y as SDWORD;
                            distSq = xdiff * xdiff + ydiff * ydiff;
                            if distSq < tarDist && distSq > minDist {
                                tarDist = distSq;
                                psTarget = (*psCStruct).psTarget
                            }
                        }
                    } else if structCBSensor(psCStruct) != 0 &&
                                  !(*psCStruct).psTarget.is_null() {
                        /*check its a valid target*/
                        if validTarget(psObj, (*psCStruct).psTarget) != 0 &&
                               aiStructHasRange(psObj as *mut STRUCTURE,
                                                (*psCStruct).psTarget) != 0 {
                            xdiff =
                                (*(*psCStruct).psTarget).x as SDWORD -
                                    (*psObj).x as SDWORD;
                            ydiff =
                                (*(*psCStruct).psTarget).y as SDWORD -
                                    (*psObj).y as SDWORD;
                            distSq = xdiff * xdiff + ydiff * ydiff;
                            if (bCBTower == 0 || distSq < tarDist) &&
                                   distSq > minDist {
                                tarDist = distSq;
                                psTarget = (*psCStruct).psTarget;
                                bCBTower = 1 as libc::c_int
                            }
                        }
                    }
                }
                psCStruct = (*psCStruct).psNext
            }
        }
        if psTarget.is_null() && bCommanderBlock == 0 {
            gridStartIterate((*psObj).x as SDWORD, (*psObj).y as SDWORD);
            psCurr = gridIterate();
            while !psCurr.is_null() {
                //don't target features
                if (*psCurr).type_0 as libc::c_uint !=
                       OBJ_FEATURE as libc::c_int as libc::c_uint {
                    if (*psObj).player as libc::c_int !=
                           (*psCurr).player as libc::c_int &&
                           aiCheckAlliances((*psCurr).player as UDWORD,
                                            (*psObj).player as UDWORD) == 0 {
                        /*check its a valid target*/
                        if validTarget(psObj, psCurr) != 0 &&
                               aiObjIsWall(psCurr) == 0 {
                            // See if in sensor range and visible
                            xdiff =
                                (*psCurr).x as libc::c_int -
                                    (*psObj).x as libc::c_int;
                            ydiff =
                                (*psCurr).y as libc::c_int -
                                    (*psObj).y as libc::c_int;
                            distSq = xdiff * xdiff + ydiff * ydiff;
                            if distSq < radSquared as SDWORD &&
                                   (*psCurr).visible[(*psObj).player as usize]
                                       as libc::c_int != 0 && distSq < tarDist
                               {
                                psTarget = psCurr;
                                tarDist = distSq
                            }
                        }
                    }
                }
                psCurr = gridIterate()
            }
        }
        /*		for(player = 0; player < MAX_PLAYERS; player++)
		{
			if (player != psObj->player && !aiCheckAlliances(player,psObj->player) )
			{
				for(psCurr = (BASE_OBJECT *)apsDroidLists[player]; psCurr;
					psCurr = psCurr->psNext)
				{
					// See if in sensor range and visible
					xdiff = psCurr->x - psObj->x;
					ydiff = psCurr->y - psObj->y;
					distSq = xdiff*xdiff + ydiff*ydiff;
					if (distSq < (SDWORD)radSquared &&
						psCurr->visible[psObj->player] &&
						distSq < tarDist)
					{
						psTarget = psCurr;
						tarDist = distSq;
					}
				}
			}
		}

		if (psTarget)
		{
			*ppsTarget = psTarget;
			return TRUE;
		}

		for(player = 0; player < MAX_PLAYERS; player++)
		{
			if (player != psObj->player  && !aiCheckAlliances(player,psObj->player) )
			{
				for(psCurr = (BASE_OBJECT *)apsStructLists[player]; psCurr;
					psCurr = psCurr->psNext)
				{
					// See if in sensor range and visible
					xdiff = psCurr->x - psObj->x;
					ydiff = psCurr->y - psObj->y;
					distSq = xdiff*xdiff + ydiff*ydiff;
					if (distSq < (SDWORD)radSquared &&
						psCurr->visible[psObj->player] &&
						distSq < tarDist)
					{
						psTarget = psCurr;
						tarDist = distSq;
					}
				}
			}
		}*/
        if !psTarget.is_null() {
            *ppsTarget = psTarget;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* See if there is a target in range for Sensor objects*/
#[no_mangle]
pub unsafe extern "C" fn aiChooseSensorTarget(mut psObj: *mut BASE_OBJECT,
                                              mut ppsTarget:
                                                  *mut *mut BASE_OBJECT)
 -> BOOL {
    let mut radSquared: UDWORD = 0;
    //	UDWORD	player;
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut tarDist: SDWORD = 0;
    //    BOOL    bSuperSensor = FALSE;
    /* Get the sensor range */
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            if (*asSensorStats.offset((*(psObj as
                                             *mut DROID)).asBits[COMP_SENSOR
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     usize].nStat
                                          as isize)).location !=
                   LOC_TURRET as libc::c_int as libc::c_uint {
                // to be used for Turret Sensors only
                return 0 as libc::c_int
            }
            radSquared =
                (*(psObj as
                       *mut DROID)).sensorRange.wrapping_mul((*(psObj as
                                                                    *mut DROID)).sensorRange)
        }
        1 => {
            if !(structStandardSensor(psObj as *mut STRUCTURE) != 0 ||
                     structVTOLSensor(psObj as *mut STRUCTURE) != 0) {
                // to be used for Standard and VTOL intercept Turret Sensors only
                return 0 as libc::c_int
            }
            radSquared =
                ((*(psObj as *mut STRUCTURE)).sensorRange as libc::c_int *
                     (*(psObj as *mut STRUCTURE)).sensorRange as libc::c_int)
                    as UDWORD
        }
        _ => { radSquared = 0 as libc::c_int as UDWORD }
    }
    /* See if there is a something in range */
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        if aiNearestTarget(psObj as *mut DROID, &mut psTarget) != 0 {
            /* See if in sensor range */
            xdiff = (*psTarget).x as libc::c_int - (*psObj).x as libc::c_int;
            ydiff = (*psTarget).y as libc::c_int - (*psObj).y as libc::c_int;
            if xdiff * xdiff + ydiff * ydiff < radSquared as SDWORD {
                *ppsTarget = psTarget;
                return 1 as libc::c_int
            }
        }
    } else {
        //AB 3/9/99 - the SUPER_SENSOR now uses the stat defined range - same as all other sensors
        //new for the SUPER sensor which can see EVERYWHERE!
        /*if (bSuperSensor)
        {
            DROID       *psDroid;
            STRUCTURE   *psStruct;
            UBYTE       player;

            tarDist = SDWORD_MAX;
            psTarget = NULL;
            // just go through the list of droids/structures for the oppositions
            // and get the nearest target. This might be REAL slow...
            for (player = 0; player < MAX_PLAYERS; player++)
            {
                //ignore the Sensor Structure's objects
                if ((player == psObj->player) OR (aiCheckAlliances(player,psObj->player)))
                {
                    continue;
                }
                for (psDroid = apsDroidLists[player]; psDroid; psDroid = psDroid->psNext)
                {
                    //everything is visible!
					xdiff = psDroid->x - psObj->x;
					ydiff = psDroid->y - psObj->y;
					distSq = xdiff*xdiff + ydiff*ydiff;
                    //store if nearer
                    if (distSq < tarDist)
                    {
    					psTarget = (BASE_OBJECT *)psDroid;
	    				tarDist = distSq;
                    }
                }
                for (psStruct = apsStructLists[player]; psStruct; psStruct = psStruct->psNext)
                {
                    //everything is visible!
					xdiff = psStruct->x - psObj->x;
					ydiff = psStruct->y - psObj->y;
					distSq = xdiff*xdiff + ydiff*ydiff;
                    //store if nearer
                    if (distSq < tarDist)
                    {
    					psTarget = (BASE_OBJECT *)psStruct;
	    				tarDist = distSq;
                    }
                }
            }
        }
        else*/
        tarDist = 0x7fffffff as libc::c_int;
        psTarget = 0 as *mut BASE_OBJECT;
        gridStartIterate((*psObj).x as SDWORD, (*psObj).y as SDWORD);
        psCurr = gridIterate();
        while !psCurr.is_null() {
            //don't target features
            if (*psCurr).type_0 as libc::c_uint !=
                   OBJ_FEATURE as libc::c_int as libc::c_uint {
                if (*psObj).player as libc::c_int !=
                       (*psCurr).player as libc::c_int &&
                       aiCheckAlliances((*psCurr).player as UDWORD,
                                        (*psObj).player as UDWORD) == 0 &&
                       aiObjIsWall(psCurr) == 0 {
                    // See if in sensor range and visible
                    xdiff =
                        (*psCurr).x as libc::c_int -
                            (*psObj).x as libc::c_int;
                    ydiff =
                        (*psCurr).y as libc::c_int -
                            (*psObj).y as libc::c_int;
                    distSq = xdiff * xdiff + ydiff * ydiff;
                    if distSq < radSquared as SDWORD &&
                           (*psCurr).visible[(*psObj).player as usize] as
                               libc::c_int != 0 && distSq < tarDist {
                        psTarget = psCurr;
                        tarDist = distSq
                    }
                }
            }
            psCurr = gridIterate()
        }
        if !psTarget.is_null() {
            *ppsTarget = psTarget;
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/* Do the AI for a droid */
#[no_mangle]
pub unsafe extern "C" fn aiUpdateDroid(mut psDroid: *mut DROID) {
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut lookForTarget: BOOL = 0;
    //	BOOL		bTemp;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"updateUnitAI: invalid Unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"ai.c\x00" as *const u8 as *const libc::c_char,
              650 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"aiUpdateDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    lookForTarget = 1 as libc::c_int;
    // don't look for a target if sulking
    if (*psDroid).action == DACTION_SULK as libc::c_int {
        lookForTarget = 0 as libc::c_int
    }
    // don't look for a target if doing something else
    if orderState(psDroid, DORDER_NONE) == 0 &&
           orderState(psDroid, DORDER_GUARD) == 0 {
        lookForTarget = 0 as libc::c_int
    }
    // don't look for a target if there are any queued orders
    if (*psDroid).listSize > 0 as libc::c_int {
        lookForTarget = 0 as libc::c_int
    }
    // horrible check to stop droids looking for a target if
	// they would switch to the guard order in the order update loop
    if (*psDroid).order == DORDER_NONE as libc::c_int &&
           (*psDroid).player as libc::c_uint == selectedPlayer &&
           vtolDroid(psDroid) == 0 &&
           secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) != 0 &&
           state as libc::c_uint ==
               DSS_HALT_GUARD as libc::c_int as libc::c_uint {
        lookForTarget = 0 as libc::c_int
    }
    // don't allow units to start attacking if they will switch to guarding the commander
    if (*psDroid).droidType as libc::c_uint !=
           DROID_COMMAND as libc::c_int as libc::c_uint &&
           !(*psDroid).psGroup.is_null() &&
           (*(*psDroid).psGroup).type_0 as libc::c_int ==
               GT_COMMAND as libc::c_int {
        lookForTarget = 0 as libc::c_int
    }
    if bMultiPlayer != 0 && vtolDroid(psDroid) != 0 &&
           isHumanPlayer((*psDroid).player as UDWORD) != 0 {
        lookForTarget = 0 as libc::c_int
    }
    // do not choose another target if doing anything while guarding
    if orderState(psDroid, DORDER_GUARD) != 0 &&
           (*psDroid).action != DACTION_NONE as libc::c_int {
        lookForTarget = 0 as libc::c_int
    }
    //do not look for a target if droid is an empty vtol
/*    if (vtolEmpty(psDroid))
    {
        lookForTarget = FALSE;
    }*/
    // do not look for a target if droid is currently under direct control.
    if driveModeActive() != 0 && psDroid == driveGetDriven() {
        lookForTarget = 0 as libc::c_int
    }
    // only computer senosr droids in the single player game aquire targets
    if (*psDroid).droidType as libc::c_uint ==
           DROID_SENSOR as libc::c_int as libc::c_uint &&
           (*psDroid).player as libc::c_uint == selectedPlayer &&
           bMultiPlayer == 0 {
        lookForTarget = 0 as libc::c_int
    }
    // do not attack if the attack level is wrong
    if secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &mut state) != 0 {
        if state as libc::c_uint !=
               DSS_ALEV_ALWAYS as libc::c_int as libc::c_uint {
            lookForTarget = 0 as libc::c_int
        }
    }
    /* Null target - see if there is an enemy to attack */
    if lookForTarget != 0 &&
           aiChooseTarget(psDroid as *mut BASE_OBJECT, &mut psTarget) != 0 {
        //			my_error("",0,"","Droid(%s) attacking : %d\n",
//					psDroid->pName, psTarget->id );
        turnOffMultiMsg(1 as libc::c_int);
        if (*psDroid).droidType as libc::c_uint ==
               DROID_SENSOR as libc::c_int as libc::c_uint {
            orderDroidObj(psDroid, DORDER_OBSERVE, psTarget);
        } else { orderDroidObj(psDroid, DORDER_ATTACKTARGET, psTarget); }
        turnOffMultiMsg(0 as libc::c_int);
    };
}
/*
 * AI.h
 *
 * Definitions for the AI system structures
 *
 */
// states of alliance between players
// for setting values only.
//alliance possibilities for games.
//#define GROUP_WINS		2
// alliances
/* Check no alliance has formed*/
/* Initialise the AI system */
/* Shutdown the AI system */
/* Initialise a droid structure for AI */
/* Do the AI for a droid */
// Find the nearest target to a droid
/* See if there is a target in range */
/*set the droid to attack if wihin range otherwise move to target*/
/* See if there is a target in range for Sensor objects*/
/*set of rules which determine whether the weapon associated with the object
can fire on the propulsion type of the target*/
/*set of rules which determine whether the weapon associated with the object
can fire on the propulsion type of the target*/
#[no_mangle]
pub unsafe extern "C" fn validTarget(mut psObject: *mut BASE_OBJECT,
                                     mut psTarget: *mut BASE_OBJECT) -> BOOL {
    let mut bTargetInAir: BOOL = 0;
    let mut bValidTarget: BOOL = 0;
    let mut surfaceToAir: UBYTE = 0;
    bValidTarget = 0 as libc::c_int;
    //need to check propulsion type of target
    match (*psTarget).type_0 as libc::c_uint {
        0 => {
            if (*asPropulsionTypes.offset((*asPropulsionStats.offset((*(psTarget
                                                                            as
                                                                            *mut DROID)).asBits[COMP_PROPULSION
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    usize].nStat
                                                                         as
                                                                         isize)).propulsionType
                                              as isize)).travel ==
                   AIR as libc::c_int as libc::c_uint {
                if (*(psTarget as *mut DROID)).sMove.Status as libc::c_int !=
                       0 as libc::c_int {
                    bTargetInAir = 1 as libc::c_int
                } else { bTargetInAir = 0 as libc::c_int }
            } else { bTargetInAir = 0 as libc::c_int }
        }
        1 | _ => {
            //lets hope so!
            bTargetInAir = 0 as libc::c_int
        }
    }
    //need what can shoot at
    match (*psObject).type_0 as libc::c_uint {
        0 => {
            // Can't attack without a weapon
		//if (((DROID *)psObject)->numWeaps != 0)
            if (*(psObject as
                      *mut DROID)).asWeaps[0 as libc::c_int as usize].nStat !=
                   0 as libc::c_int as libc::c_uint {
                surfaceToAir =
                    (*asWeaponStats.offset((*(psObject as
                                                  *mut DROID)).asWeaps[0 as
                                                                           libc::c_int
                                                                           as
                                                                           usize].nStat
                                               as isize)).surfaceToAir
            } else { surfaceToAir = 0 as libc::c_int as UBYTE }
        }
        1 => {
            // Can't attack without a weapon
		//if (((STRUCTURE *)psObject)->numWeaps != 0)
            if (*(psObject as
                      *mut STRUCTURE)).asWeaps[0 as libc::c_int as
                                                   usize].nStat !=
                   0 as libc::c_int as libc::c_uint {
                surfaceToAir =
                    (*asWeaponStats.offset((*(psObject as
                                                  *mut STRUCTURE)).asWeaps[0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize].nStat
                                               as isize)).surfaceToAir
            } else { surfaceToAir = 0 as libc::c_int as UBYTE }
        }
        _ => { surfaceToAir = 0 as libc::c_int as UBYTE }
    }
    //if target is in the air and you can shoot in the air - OK
    if bTargetInAir != 0 &&
           surfaceToAir as libc::c_int & 0x2 as libc::c_int != 0 {
        bValidTarget = 1 as libc::c_int
    }
    //if target is on the ground and can shoot at it - OK
    if bTargetInAir == 0 &&
           surfaceToAir as libc::c_int & 0x1 as libc::c_int != 0 {
        bValidTarget = 1 as libc::c_int
    }
    return bValidTarget;
}
