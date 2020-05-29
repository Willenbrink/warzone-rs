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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* Return the absolute value of X.  */
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player: UDWORD, onMission: BOOL) -> *mut DROID;
    #[no_mangle]
    fn calcTemplatePower(psTemplate: *mut DROID_TEMPLATE) -> UDWORD;
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    #[no_mangle]
    fn droidRemove(psDroid: *mut DROID, pList: *mut *mut DROID) -> BOOL;
    #[no_mangle]
    fn initDroidMovement(psDroid: *mut DROID);
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
    #[no_mangle]
    fn moveDroidTo(psDroid: *mut DROID, x: UDWORD, y: UDWORD) -> BOOL;
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
// the droid will not join a formation when it gets to the location
    #[no_mangle]
    fn moveDroidToNoFormation(psDroid: *mut DROID, x: UDWORD, y: UDWORD)
     -> BOOL;
    /* update body and turret to local slope */
    #[no_mangle]
    fn updateDroidOrientation(psDroid: *mut DROID);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    /*subtract the power required */
    #[no_mangle]
    fn usePower(player: UDWORD, quantity: UDWORD) -> BOOL;
    //flag used to check for power calculations to be done or not
    #[no_mangle]
    static mut powerCalculated: BOOL;
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    // remove a droid from a group
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    //choose one. 
    #[no_mangle]
    fn NETsend(msg: *mut NETMSG, player: DPID, guarantee: BOOL) -> BOOL;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // true when more than 1 player.
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut player2dpid: [DWORD; 8];
    #[no_mangle]
    fn IdToStruct(id: UDWORD, player: UDWORD) -> *mut STRUCTURE;
    #[no_mangle]
    fn IdToDroid(id: UDWORD, player: UDWORD, psDroid: *mut *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn IdToFeature(id: UDWORD, player: UDWORD) -> *mut FEATURE;
    #[no_mangle]
    fn IdToTemplate(tempId: UDWORD, player: UDWORD) -> *mut DROID_TEMPLATE;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    /* Give a droid an action with an object target */
    #[no_mangle]
    fn actionDroidObj(psDroid: *mut DROID, action: DROID_ACTION,
                      psObj: *mut BASE_OBJECT);
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // ////////////////////////////////////////////////////////////////////////////
// External Stuff.
    #[no_mangle]
    fn chooseOrderLoc(psDroid: *mut DROID, x: UDWORD, y: UDWORD)
     -> DROID_ORDER;
    #[no_mangle]
    fn chooseOrderObj(psDroid: *mut DROID, psObj: *mut BASE_OBJECT)
     -> DROID_ORDER;
}
pub type size_t = libc::c_uint;
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
pub type CHAR = libc::c_char;
pub type USHORT = libc::c_ushort;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
/* !WIN32 */
pub type DPID = libc::c_int;
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
pub type WEAPON_STATS = _weapon_stats;
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
// information about a formation member
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
}
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
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
// position relative to center
// orientation of line
// first member in the 'linked list' of members
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
pub type _droid_type = libc::c_uint;
pub const DROID_ANY: _droid_type = 13;
pub const DROID_CYBORG_SUPER: _droid_type = 12;
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
pub const DROID_DEFAULT: _droid_type = 9;
pub const DROID_REPAIR: _droid_type = 8;
pub const DROID_COMMAND: _droid_type = 7;
pub const DROID_TRANSPORTER: _droid_type = 6;
pub const DROID_CYBORG: _droid_type = 5;
pub const DROID_PERSON: _droid_type = 4;
pub const DROID_CONSTRUCT: _droid_type = 3;
pub const DROID_ECM: _droid_type = 2;
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
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
pub struct _droid_template {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub aName: [STRING; 60],
    pub NameVersion: UBYTE,
    pub asParts: [SDWORD; 8],
    pub buildPoints: UDWORD,
    pub powerPoints: UDWORD,
    pub storeCount: UDWORD,
    pub numWeaps: UDWORD,
    pub asWeaps: [UDWORD; 1],
    pub droidType: DROID_TYPE,
    pub multiPlayerID: UDWORD,
    pub psNext: *mut _droid_template,
}
pub type DROID_TEMPLATE = _droid_template;
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
/* Pointer to next template*/
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
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
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
pub type STRUCTURE = _structure;
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/*
 * FeatureDef.h
 *
 * Structure definitions for features
 *
 */
pub type _feature_type = libc::c_uint;
pub const FEAT_SKYSCRAPER: _feature_type = 12;
pub const FEAT_TREE: _feature_type = 11;
pub const FEAT_OIL_DRUM: _feature_type = 10;
pub const FEAT_LOS_OBJ: _feature_type = 9;
pub const FEAT_DROID: _feature_type = 8;
pub const FEAT_BUILDING: _feature_type = 7;
pub const FEAT_VEHICLE: _feature_type = 6;
pub const FEAT_BOULDER: _feature_type = 5;
pub const FEAT_OIL_RESOURCE: _feature_type = 4;
pub const FEAT_GEN_ARTE: _feature_type = 3;
pub const FEAT_TANK: _feature_type = 2;
pub const FEAT_HOVER: _feature_type = 1;
pub const FEAT_BUILD_WRECK: _feature_type = 0;
pub type FEATURE_TYPE = _feature_type;
//FEAT_MESA,	no longer used
	//FEAT_MESA2,	
	//FEAT_CLIFF,
	//FEAT_STACK,
	//FEAT_BUILD_WRECK1,
	//FEAT_BUILD_WRECK2,
	//FEAT_BUILD_WRECK3,
	//FEAT_BUILD_WRECK4,
	//FEAT_BOULDER1,
	//FEAT_BOULDER2,
	//FEAT_BOULDER3,
	//FEAT_FUTCAR,
	//FEAT_FUTVAN,
/* Stats for a feature */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub subType: FEATURE_TYPE,
    pub psImd: *mut iIMDShape,
    pub baseWidth: UWORD,
    pub baseBreadth: UWORD,
    pub tileDraw: BOOL,
    pub allowLOS: BOOL,
    pub visibleAtStart: BOOL,
    pub damageable: BOOL,
    pub body: UDWORD,
    pub armour: UDWORD,
}
pub type FEATURE_STATS = _feature_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _feature {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut _feature,
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
    pub psStats: *mut FEATURE_STATS,
    pub startTime: UDWORD,
    pub body: UDWORD,
    pub gfxScaling: UWORD,
    pub timeLastHit: UDWORD,
    pub bTargetted: BOOL,
}
pub type FEATURE = _feature;
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
pub type DROID_ORDER = _droid_order;
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
pub type SECONDARY_ORDER = _secondary_order;
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
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const DSS_ARANGE_LONG: _secondary_state = 2;
pub const DSS_ARANGE_SHORT: _secondary_state = 1;
pub type SECONDARY_STATE = _secondary_state;
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_COMMAND: _group_type = 1;
pub const GT_NORMAL: _group_type = 0;
pub type DROID_GROUP = _droid_group;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
}
pub type _msgtype = libc::c_uint;
pub const NET_SET_TEAMS: _msgtype = 50;
pub const NET_BEACONMSG: _msgtype = 49;
pub const NET_TEAMS_ON: _msgtype = 48;
pub const NET_AITEXTMSG: _msgtype = 47;
pub const NET_REQUESTMAP: _msgtype = 46;
pub const NET_LASSAT: _msgtype = 45;
pub const NET_RESEARCHSTATUS: _msgtype = 44;
pub const NET_DROIDDISEMBARK: _msgtype = 43;
pub const NET_DROIDEMBARK: _msgtype = 42;
pub const NET_SECONDARY_ALL: _msgtype = 41;
pub const NET_WHITEBOARD: _msgtype = 40;
pub const NET_VTOLREARM: _msgtype = 39;
pub const NET_VTOL: _msgtype = 38;
pub const NET_DESTROYXTRA: _msgtype = 37;
pub const NET_SCORESUBMIT: _msgtype = 36;
pub const NET_DMATCHWIN: _msgtype = 35;
pub const NET_ARTIFACTS: _msgtype = 34;
pub const NET_COLOURREQUEST: _msgtype = 33;
pub const NET_DEMOLISH: _msgtype = 32;
pub const NET_GIFT: _msgtype = 31;
pub const NET_ALLIANCE: _msgtype = 30;
pub const NET_FIREUP: _msgtype = 29;
pub const NET_SECONDARY: _msgtype = 28;
pub const NET_KICK: _msgtype = 27;
pub const NET_OPTIONS: _msgtype = 26;
pub const NET_PLAYERRESPONDING: _msgtype = 25;
pub const NET_FEATURES: _msgtype = 24;
pub const NET_WHOLEDROID: _msgtype = 23;
pub const NET_STRUCT: _msgtype = 22;
pub const NET_REQUESTPLAYER: _msgtype = 21;
pub const NET_PLAYERCOMPLETE: _msgtype = 20;
pub const NET_REQUESTDROID: _msgtype = 19;
pub const NET_LEAVING: _msgtype = 18;
pub const NET_TEXTMSG: _msgtype = 17;
pub const NET_RESEARCH: _msgtype = 16;
pub const NET_BUILDFINISHED: _msgtype = 15;
pub const NET_STRUCTDEST: _msgtype = 14;
pub const NET_BUILD: _msgtype = 13;
pub const NET_VERSION: _msgtype = 12;
pub const NET_CHECK_POWER: _msgtype = 11;
pub const NET_CHECK_STRUCT: _msgtype = 10;
pub const NET_CHECK_DROID: _msgtype = 9;
pub const NET_PING: _msgtype = 8;
pub const NET_FEATUREDEST: _msgtype = 7;
pub const NET_TEMPLATEDEST: _msgtype = 6;
pub const NET_TEMPLATE: _msgtype = 5;
pub const NET_GROUPORDER: _msgtype = 4;
pub const NET_DROIDMOVE: _msgtype = 3;
pub const NET_DROIDDEST: _msgtype = 2;
pub const NET_DROIDINFO: _msgtype = 1;
pub const NET_DROID: _msgtype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERINGAME {
    pub PingTimes: [UDWORD; 8],
    pub localOptionsReceived: BOOL,
    pub localJoiningInProgress: BOOL,
    pub JoiningInProgress: [BOOL; 8],
    pub bHostSetup: BOOL,
    pub startTime: UDWORD,
    pub modem: UDWORD,
    pub numStructureLimits: UDWORD,
    pub pStructureLimits: *mut UBYTE,
    pub skScores: [[UDWORD; 2]; 8],
    pub phrases: [[CHAR; 255]; 5],
}
pub const DACTION_NONE: _droid_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DROIDSTORE {
    pub psDroid: *mut DROID,
    pub psNext: *mut libc::c_void,
}
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
// Feature armour
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
/*
 * Action.h
 *
 * Function prototypes for setting the action of a droid
 *
 */
// What a droid is currently doing
// Not necessarily the same as it's order as the AI may get a droid to do
// something else whilst carrying out an order
pub type DROID_ACTION = _droid_action;
#[no_mangle]
pub static mut tempDroidList: *mut DROIDSTORE =
    0 as *const DROIDSTORE as *mut DROIDSTORE;
// not doing anything
// 1 moving to a location
// building a structure
// 3 building a foundation for a structure
// demolishing a structure
// 5 repairing a structure
// attacking something
// 7 observing something
// attacking something visible by a sensor droid
// 9 refuse to do anything aggresive for a fixed time
// self destruct
// 11 move transporter offworld
// wait for timer to move reinforcements in
// 13 move transporter onworld
// repairing a droid
// 15 restore resistance points of a structure
// clearing building wreckage
// The states below are used by the action system
	// but should not be given as an action
// 17
// moving to a new building location
// 19 moving to a new demolition location
// moving to a new repair location
// 21 moving around while building
// moving around while building the foundation
// 23 moving to a target to attack
// rotating to a target to attack
// 25 moving to be able to see a target
// waiting to be repaired by a facility
// 27 move to repair facility repair point
// waiting to be repaired by a facility
// 29 moving to a new location next to droid to be repaired
// moving to a low resistance structure
// 31 moving to a building wreck location
// (32)moving to a rearming pad - VTOLS
// (33)waiting for rearm - VTOLS
// (34)move to rearm point - VTOLS - this actually moves them onto the pad
// (35)waiting during rearm process- VTOLS
// (36) a VTOL droid doing attack runs
// (37) a VTOL droid being told to get off a rearm pad
// (38) used by scout/patrol order when returning to route
// (39) used by firesupport order when sensor retreats
// ////////////////////////////////////////////////////////////////////////////
// Command Droids.
// sod em.
// ////////////////////////////////////////////////////////////////////////////
// vtol bits.
// happy vtol = vtol ready to go back to attack.
#[no_mangle]
pub unsafe extern "C" fn sendHappyVtol(mut psDroid: *mut DROID) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if myResponsibility((*psDroid).player as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).player as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_VTOL as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvHappyVtol(mut pMsg: *mut NETMSG) -> BOOL {
    let mut pD: *mut DROID = 0 as *mut DROID;
    let mut player: UBYTE = 0;
    let mut id: UDWORD = 0;
    memcpy(&mut player as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    if IdToDroid(id, player as UDWORD, &mut pD) == 0 {
        //find droid.
        return 0 as libc::c_int
    } // finish it for next time round.
    (*pD).sMove.iAttackRuns = 0 as libc::c_int as UWORD;
    (*pD).body = (*pD).originalBody;
    (*pD).asWeaps[0 as libc::c_int as usize].ammo =
        (*asWeaponStats.offset((*pD).asWeaps[0 as libc::c_int as usize].nStat
                                   as isize)).numRounds as UDWORD;
    (*pD).asWeaps[0 as libc::c_int as usize].lastFired =
        0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Local Prototypes
// vtol now needs to return to rearm.
#[no_mangle]
pub unsafe extern "C" fn sendVtolRearm(mut psDroid: *mut DROID,
                                       mut psStruct: *mut STRUCTURE,
                                       mut chosen: UBYTE) -> BOOL {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut blank: UDWORD = 0 as libc::c_int as UDWORD;
    let mut attackRuns: UBYTE = 0;
    let mut ammo: UBYTE = 0;
    let mut player: UBYTE = 0;
    if myResponsibility((*psDroid).player as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    player = (*psDroid).player;
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(5 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut chosen as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    if !psStruct.is_null() {
        memcpy(&mut *msg.body.as_mut_ptr().offset(6 as libc::c_int as isize)
                   as *mut libc::c_char as *mut libc::c_void,
               &mut (*psStruct).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    } else {
        memcpy(&mut *msg.body.as_mut_ptr().offset(6 as libc::c_int as isize)
                   as *mut libc::c_char as *mut libc::c_void,
               &mut blank as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    }
    attackRuns = (*psDroid).sMove.iAttackRuns as UBYTE;
    ammo = (*psDroid).asWeaps[0 as libc::c_int as usize].ammo as UBYTE;
    memcpy(&mut *msg.body.as_mut_ptr().offset(10 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut attackRuns as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(11 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut ammo as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    msg.size = 12 as libc::c_int as libc::c_ushort;
    msg.type_0 = NET_VTOLREARM as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int;
}
// this is a complete rip from recvvtolrearm
#[no_mangle]
pub unsafe extern "C" fn recvVtolRearm(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut player: UBYTE = 0;
    let mut chosen: UBYTE = 0;
    let mut aruns: UBYTE = 0;
    let mut amm: UBYTE = 0;
    let mut id: UDWORD = 0;
    let mut ids: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    memcpy(&mut player as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut chosen as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(5 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut ids as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(6 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut aruns as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(10 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut amm as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(11 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    if IdToDroid(id, player as UDWORD, &mut psDroid) == 0 {
        //find droid.
        return 0 as libc::c_int
    }
    if ids != 0 {
        // find rearm pad.
        psStruct = IdToStruct(id, (*psDroid).player as UDWORD);
        if psStruct.is_null() {
            return 0 as libc::c_int
        } else { (*psDroid).psBaseStruct = psStruct }
    }
    (*psDroid).sMove.iAttackRuns = aruns as UWORD;
    (*psDroid).asWeaps[0 as libc::c_int as usize].ammo = amm as UDWORD;
    turnOffMultiMsg(1 as libc::c_int);
    match chosen as libc::c_int {
        1 => {
            (*psDroid).order = DORDER_NONE as libc::c_int;
            orderDroidObj(psDroid, DORDER_REARM,
                          psStruct as *mut BASE_OBJECT);
        }
        2 => {
            actionDroidObj(psDroid, DACTION_MOVETOREARM,
                           psStruct as *mut BASE_OBJECT);
        }
        3 => { orderDroid(psDroid, DORDER_RTB); }
        _ => { }
    }
    turnOffMultiMsg(0 as libc::c_int);
    return 1 as libc::c_int;
}
//extern BOOL SendDroidWaypoint	(UBYTE player, UDWORD	x, UDWORD y);
//extern BOOL SendSingleDroidWaypoint(DROID *psDroid, UDWORD x,UDWORD y);
// ////////////////////////////////////////////////////////////////////////////
// Secondary Orders.
// send
#[no_mangle]
pub unsafe extern "C" fn sendDroidSecondary(mut psDroid: *mut DROID,
                                            mut sec: SECONDARY_ORDER,
                                            mut state: SECONDARY_STATE)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut sec as *mut SECONDARY_ORDER as *const libc::c_void,
           ::std::mem::size_of::<SECONDARY_ORDER>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut state as *mut SECONDARY_STATE as *const libc::c_void,
           ::std::mem::size_of::<SECONDARY_STATE>() as libc::c_ulong);
    m.body[12 as libc::c_int as usize] = (*psDroid).player as libc::c_char;
    m.size = 13 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_SECONDARY as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
// recv
#[no_mangle]
pub unsafe extern "C" fn recvDroidSecondary(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut sec: SECONDARY_ORDER = DSO_ATTACK_RANGE;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut id: UDWORD = 0;
    let mut player: UDWORD = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut sec as *mut SECONDARY_ORDER as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SECONDARY_ORDER>() as libc::c_ulong);
    memcpy(&mut state as *mut SECONDARY_STATE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(8 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SECONDARY_STATE>() as libc::c_ulong);
    player = (*pMsg).body[12 as libc::c_int as usize] as UDWORD;
    if IdToDroid(id, player, &mut psDroid) == 0 {
        //find droid.
        return 0 as libc::c_int
    } // set order
    turnOffMultiMsg(1 as libc::c_int);
    secondarySetState(psDroid, sec, state);
    turnOffMultiMsg(0 as libc::c_int);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sendDroidSecondaryAll(mut psDroid: *mut DROID)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).secondaryOrder as *mut UDWORD as
               *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.body[8 as libc::c_int as usize] = (*psDroid).player as libc::c_char;
    m.size = 9 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_SECONDARY_ALL as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvDroidSecondaryAll(mut pMsg: *mut NETMSG)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut id: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut sorder: UDWORD = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut sorder as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    player = (*pMsg).body[8 as libc::c_int as usize] as UDWORD;
    if IdToDroid(id, player, &mut psDroid) == 0 {
        //find droid.
        return 0 as libc::c_int
    }
    if !psDroid.is_null() { (*psDroid).secondaryOrder = sorder }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sendDroidEmbark(mut psDroid: *mut DROID) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.body[4 as libc::c_int as usize] = (*psDroid).player as libc::c_char;
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_DROIDEMBARK as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvDroidEmbark(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut id: UDWORD = 0;
    let mut player: UDWORD = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    player = (*pMsg).body[4 as libc::c_int as usize] as UDWORD;
    if IdToDroid(id, player, &mut psDroid) == 0 {
        //find droid.
        return 0 as libc::c_int
    }
    if !psDroid.is_null() {
        //take it out of the world without destroying it
        droidRemove(psDroid, apsDroidLists.as_mut_ptr());
        //init the order for when disembark
        (*psDroid).order = DORDER_NONE as libc::c_int;
        (*psDroid).psTarget = 0 as *mut _base_object;
        (*psDroid).psTarStats = 0 as *mut _base_stats
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sendDroidDisEmbark(mut psDroid: *mut DROID) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).x as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(6 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).y as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    m.body[8 as libc::c_int as usize] = (*psDroid).player as libc::c_char;
    m.size = 9 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_DROIDDISEMBARK as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvDroidDisEmbark(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut id: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut x: UWORD = 0;
    let mut y: UWORD = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut x as *mut UWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut y as *mut UWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(6 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    player = (*pMsg).body[8 as libc::c_int as usize] as UDWORD;
    if IdToDroid(id, player, &mut psDroid) == 0 {
        //find droid.
        return 0 as libc::c_int
    }
    if !psDroid.is_null() {
        //add it back into the world at the x/y
        (*psDroid).x = x;
        (*psDroid).y = y;
        updateDroidOrientation(psDroid);
        //initialise the movement data
        initDroidMovement(psDroid);
        //reset droid orders
        orderDroid(psDroid, DORDER_STOP);
        gridAddObject(psDroid as *mut BASE_OBJECT);
        (*psDroid).cluster = 0 as libc::c_int as UBYTE;
        addDroid(psDroid, apsDroidLists.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
//BOOL		SendSingleDroidWaypoint(DROID *psDroid, UDWORD x,UDWORD y);
//BOOL		SendDroidWaypoint	(UBYTE player,UDWORD	x, UDWORD y);
//BOOL		recvDroidWaypoint	(NETMSG *pMsg);
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Waypoints
// send
/*
// occasionally need to send non seleceted waypoints.
BOOL SendSingleDroidWaypoint(DROID *psDroid, UDWORD x,UDWORD y)
{
	NETMSG m;
	UDWORD	count =1;
	NetAdd(m,0,(psDroid->player));			// player
	NetAdd(m,1,x);							// waypoint x
	NetAdd(m,5,y);							// waypoint y
	NetAdd(m,9,count);
	NetAdd(m,13,psDroid->id);
	m.type = NET_WAYPOINT;
	m.size =17;
	return NETbcast(m,FALSE);
}

BOOL SendDroidWaypoint	(UBYTE player,UDWORD	x, UDWORD y)
{
	NETMSG	m;
	UDWORD	count=0;
	DROID	*psCurr;

	NetAdd(m,0,player);				// player
	NetAdd(m,1,x);					// waypoint x
	NetAdd(m,5,y);					// waypoint y
	m.size = 13;
	m.type = NET_WAYPOINT;

	// find each droid under instruction
	for(psCurr = apsDroidLists[player]; psCurr; psCurr=psCurr->psNext)
	{
		if (psCurr->selected)
		{
			NetAdd(m,m.size,psCurr->id);
			count++;
			m.size += 4;
		}
	}
	NetAdd(m,9,count);

	if(count>0)
	{
		return NETbcast(m,FALSE);
	}
	else
	{
		return TRUE;
	}
}

// recv
BOOL recvDroidWaypoint(NETMSG *pMsg)
{
	UBYTE	play;
	UDWORD	x,y,count,i,id;
	DROID	*psDroid;
	FORMATION	*psFormation=NULL;
	DROID		*psPrev=NULL;

	NetGet(pMsg,0,play);
	NetGet(pMsg,1,x);
	NetGet(pMsg,5,y);
	NetGet(pMsg,9,count);

	for(i=0;i<count;i++)
	{
		NetGet(pMsg,13+(i*4),id);			// get droid id.
		if(!IdToDroid(id,play,&psDroid))	//find droid.
		{
			sendRequestDroid(id);			// request it.
			return FALSE;
		}

		turnOffMultiMsg(TRUE);
		orderAddWayPoint(psDroid,x,y);		// add waypoint to that droid.
		turnOffMultiMsg(FALSE);
	}
	return TRUE;
}

*/
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Droids
// posibly Send an updated droid movement order.
#[no_mangle]
pub unsafe extern "C" fn SendDroidMove(mut pDroid: *mut DROID, mut x: UDWORD,
                                       mut y: UDWORD, mut bFormation: BOOL)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // wether or not to send
    let mut sendit: BOOL =
        1 as libc::c_int; // wether or not to allow the move to proceed.
    let mut allow: BOOL = 1 as libc::c_int;
    match (*pDroid).action {
        0 => {
            // look at the action. Some actions shouldn't be sent.
            sendit = 0 as libc::c_int
        }
        _ => { sendit = 1 as libc::c_int }
    }
    match (*pDroid).order {
        2 => {
            // next look at the order.
            sendit = 0 as libc::c_int
        }
        _ => { }
    }
    if myResponsibility((*pDroid).player as UDWORD) == 0 {
        // we may not be responsible, find out.
        allow = 0 as libc::c_int;
        sendit = 0 as libc::c_int
    }
    if sendit != 0 {
        // send the mesg if required.
        let mut player: libc::c_char = 0; //droid to move
        let mut formation: libc::c_char = 0; //x pos
        player = (*pDroid).player as libc::c_char; //y pos
        formation = bFormation as libc::c_char; //owner of droid(for speed!)
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pDroid).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as
                   libc::c_ulong); //use a formation?
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut x as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut y as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(12 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut player as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(13 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut formation as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
        m.size = 14 as libc::c_int as libc::c_ushort;
        m.type_0 = NET_DROIDMOVE as libc::c_int as libc::c_uchar;
        NETbcast(&mut m, 0 as libc::c_int);
    }
    return allow;
}
// recv and updated droid position
#[no_mangle]
pub unsafe extern "C" fn recvDroidMove(mut m: *mut NETMSG) -> BOOL {
    let mut player: UDWORD = 0;
    let mut id: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bFormation: UBYTE = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut x as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut y as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    player = (*m).body[12 as libc::c_int as usize] as UDWORD;
    memcpy(&mut bFormation as *mut UBYTE as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(13 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    if IdToDroid(id, player, &mut psDroid) == 0 {
        // find the droid
        sendRequestDroid(id); // if we can't find it, ask for it!
        return 1 as libc::c_int
    }
    turnOffMultiMsg(1 as libc::c_int);
    if bFormation != 0 {
        moveDroidTo(psDroid, x, y);
        // do the move
    } else {
        moveDroidToNoFormation(psDroid, x, y);
        // move, no form...
    }
    turnOffMultiMsg(0 as libc::c_int);
    return 1 as libc::c_int;
}
// droids . multibot
// ////////////////////////////////////////////////////////////////////////////
// Send a new Droid to the other players
#[no_mangle]
pub unsafe extern "C" fn SendDroid(mut pTemplate: *mut DROID_TEMPLATE,
                                   mut x: UDWORD, mut y: UDWORD,
                                   mut player: UBYTE, mut id: UDWORD)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    //	if( ingame.localJoiningInProgress &&  (game.type != DMATCH) )
    if ingame.localJoiningInProgress != 0 {
        return 1 as libc::c_int
        // dont send other droids during campaign setup.
    }
    if myResponsibility(player as UDWORD) == 0 {
        // only send the droid if we are responsible
        return 0 as libc::c_int
        // dont' build if we are not responsible.
    } //ok since <255  players!
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as
               libc::c_ulong); //x pos of new droid
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut x as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); //y pos of new droid
    memcpy(&mut *m.body.as_mut_ptr().offset(5 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut y as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); //id of droid to create
    memcpy(&mut *m.body.as_mut_ptr().offset(9 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    if powerCalculated != 0 {
        m.body[13 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char
    } else {
        m.body[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
    }
    // new version
    memcpy(&mut *m.body.as_mut_ptr().offset(14 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pTemplate).multiPlayerID as *mut UDWORD as
               *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // send it.
    m.size =
        (14 as libc::c_int as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as libc::c_ushort;
    m.type_0 = NET_DROID as libc::c_int as libc::c_uchar;
    NETbcast(&mut m, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// receive droid creation information from other players
#[no_mangle]
pub unsafe extern "C" fn recvDroid(mut m: *mut NETMSG) -> BOOL {
    let mut pT: *mut DROID_TEMPLATE =
        0 as *mut DROID_TEMPLATE; // new droids x position
    let mut x: UDWORD = 0; // new droids y position
    let mut y: UDWORD = 0; // droid to build's id.
    let mut id: UDWORD = 0; // request the droid instead.
    let mut player: UDWORD = 0;
    let mut targetRef: UDWORD = 0;
    let mut d: *mut DROID = 0 as *mut DROID;
    player = (*m).body[0 as libc::c_int as usize] as UDWORD;
    memcpy(&mut x as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut y as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(5 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(9 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut targetRef as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(14 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pT = IdToTemplate(targetRef, player);
    if pT.is_null() {
        NETlogEntry(b"Couldn\'t find template to build recvd droid. val = player\x00"
                        as *const u8 as *const libc::c_char as *mut CHAR,
                    0 as libc::c_int as UDWORD, player);
        debug(LOG_NEVER,
              b"Couldn\'t find template to build recvd droid\x00" as *const u8
                  as *const libc::c_char);
        sendRequestDroid(id);
        return 0 as libc::c_int
    }
    if (*m).body[13 as libc::c_int as usize] as libc::c_int !=
           0 as libc::c_int {
        if usePower(player, (*pT).powerPoints) == 0 {
            // take the power.
            //			DBCONPRINTF(ConsoleString,(ConsoleString,"MULTIPLAYER: not enough power to build remote droid."));
            NETlogEntry(b"not enough power to build recvd droid, val=player\x00"
                            as *const u8 as *const libc::c_char as *mut CHAR,
                        0 as libc::c_int as UDWORD,
                        player); // create that droid on this machine.
        }
    }
    turnOffMultiMsg(1 as libc::c_int);
    d = buildDroid(pT, x, y, player, 0 as libc::c_int);
    turnOffMultiMsg(0 as libc::c_int);
    if !d.is_null() {
        (*d).id = id;
        addDroid(d, apsDroidLists.as_mut_ptr());
        // put droid in world
    } else { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Droid Group/selection orders.
// minimises comms by sending orders for whole groups, rather than each droid
#[no_mangle]
pub unsafe extern "C" fn SendCmdGroup(mut psGroup: *mut DROID_GROUP,
                                      mut x: UWORD, mut y: UWORD,
                                      mut psObj: *mut BASE_OBJECT) -> BOOL {
    //NETMSG	m;
	//DROID	*pDroid;
	//USHORT	droidcount=0;
    return 0 as libc::c_int;
    //doesnt fukin work. return FALSE and use other msgs to cope with this (about 2 packet overhead)
    /*
	if (psObj == NULL)							//it's a position order
	{
		NetAdd(m,0,x);
		NetAdd(m,4,y);
		m.body[8] = 0;							// subtype flag
	}
	else										// it's a object order
	{
		NetAdd(m,0, psObj->id);
		NetAdd(m,4, psObj->type);
		m.body[8]=1;							// subtype flag
	}
	m.body[10]=1;		//  a cmd order.
	m.size=12;

	for (pDroid = psGroup->psList; pDroid; pDroid=pDroid->psGrpNext)
	{

		if (!orderState(pDroid, DORDER_RTR))	// CHANGE THIS BIT WHEN REQD!!!
		{
			NetAdd(m,m.size,pDroid->id);
			m.size += sizeof(UDWORD);
			droidcount ++;
		}
	}

	if( droidcount > 0	) 							// return TRUE if it's worth using.
	{
		NetAdd(m,9,droidcount);						// note how many in this message.
		m.type = NET_GROUPORDER;					// send it
		NETbcast(&m,FALSE);
		return TRUE;
	}
	else
	{
		return FALSE;								// didn't bother using it, so return false, to allow individiual orders.
	}
*/
}
#[no_mangle]
pub unsafe extern "C" fn SendGroupOrderSelected(mut player: UBYTE,
                                                mut x: UDWORD, mut y: UDWORD,
                                                mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pDroid: *mut DROID = 0 as *mut DROID;
    let mut droidcount: USHORT = 0 as libc::c_int as USHORT;
    if psObj.is_null() {
        //it's a position order
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut x as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut y as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.body[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        // subtype flag
    } else {
        // it's a object order
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).type_0 as *mut OBJECT_TYPE as
                   *const libc::c_void,
               ::std::mem::size_of::<OBJECT_TYPE>() as libc::c_ulong);
        m.body[8 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char
        // subtype flag
    } // not a cmd order.
    m.body[10 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char; // set the order.
    m.body[12 as libc::c_int as usize] = 99 as libc::c_int as libc::c_char;
    m.size = 13 as libc::c_int as libc::c_ushort;
    //now add the list of droid id's to order
    pDroid = apsDroidLists[player as usize];
    while !pDroid.is_null() {
        if (*pDroid).selected != 0 {
            memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut (*pDroid).id as *mut UDWORD as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            m.size =
                (m.size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                    as libc::c_ulong) as
                    libc::c_ushort as libc::c_ushort;
            droidcount = droidcount.wrapping_add(1)
        }
        pDroid = (*pDroid).psNext
    }
    if droidcount as libc::c_int > 1 as libc::c_int {
        // return TRUE if it's worth using.
        memcpy(&mut *m.body.as_mut_ptr().offset(9 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut droidcount as *mut USHORT as *const libc::c_void,
               ::std::mem::size_of::<USHORT>() as
                   libc::c_ulong); // note how many in this message.
        m.type_0 = NET_GROUPORDER as libc::c_int as libc::c_uchar; // send it
        NETbcast(&mut m, 0 as libc::c_int);
        return 1 as libc::c_int
    } else {
        // didn't bother using it, so return false, to allow individiual orders.
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn SendGroupOrderGroup(mut psGroup: *mut DROID_GROUP,
                                             mut order: DROID_ORDER,
                                             mut x: UDWORD, mut y: UDWORD,
                                             mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pDroid: *mut DROID = 0 as *mut DROID;
    let mut droidcount: USHORT = 0 as libc::c_int as USHORT;
    if psObj.is_null() {
        //it's a position order
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut x as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut y as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.body[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        // subtype flag
    } else {
        // it's a object order
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).type_0 as *mut OBJECT_TYPE as
                   *const libc::c_void,
               ::std::mem::size_of::<OBJECT_TYPE>() as libc::c_ulong);
        m.body[8 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char
        // subtype flag
    }
    // check this!!!!!
    m.body[10 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char; // not a cmd order.
    m.body[12 as libc::c_int as usize] =
        order as libc::c_char; // set the order.
    m.size = 13 as libc::c_int as libc::c_ushort;
    //now add the list of droid id's to order
    pDroid = (*psGroup).psList; // note how many in this message.
    while !pDroid.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pDroid).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // send it
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        droidcount = droidcount.wrapping_add(1);
        pDroid = (*pDroid).psGrpNext
    }
    memcpy(&mut *m.body.as_mut_ptr().offset(9 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut droidcount as *mut USHORT as *const libc::c_void,
           ::std::mem::size_of::<USHORT>() as libc::c_ulong);
    m.type_0 = NET_GROUPORDER as libc::c_int as libc::c_uchar;
    NETbcast(&mut m, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// receive a group order.
#[no_mangle]
pub unsafe extern "C" fn recvGroupOrder(mut pMsg: *mut NETMSG) -> BOOL {
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut id: UDWORD = 0;
    let mut destid: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut desttype: OBJECT_TYPE = OBJ_DROID;
    let mut droidcount: USHORT = 0;
    //	DROID		*psPrev = NULL;	// fomation vars.
//	FORMATION	*psFormation = NULL;
    let mut bCmdOr: BOOL = 0 as libc::c_int;
    let mut order: DROID_ORDER = DORDER_NONE;
    bCmdOr = (*pMsg).body[10 as libc::c_int as usize] as BOOL;
    order = (*pMsg).body[12 as libc::c_int as usize] as DROID_ORDER;
    memcpy(&mut droidcount as *mut USHORT as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(9 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<USHORT>() as libc::c_ulong);
    if (*pMsg).body[8 as libc::c_int as usize] as libc::c_int ==
           1 as libc::c_int {
        // it's target is an object
        memcpy(&mut destid as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut desttype as *mut OBJECT_TYPE as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<OBJECT_TYPE>() as libc::c_ulong);
        x = 0 as libc::c_int as UDWORD;
        y = 0 as libc::c_int as UDWORD
    } else {
        // it's target is a position
        memcpy(&mut x as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // x coord
        memcpy(&mut y as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // y coord
        destid = 0 as libc::c_int as UDWORD;
        desttype = OBJ_DROID
    }
    // for each droid
    while droidcount as libc::c_int > 0 as libc::c_int {
        memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset((13 as libc::c_int as
                                                           libc::c_uint).wrapping_add(((droidcount
                                                                                            as
                                                                                            libc::c_int
                                                                                            -
                                                                                            1
                                                                                                as
                                                                                                libc::c_int)
                                                                                           as
                                                                                           libc::c_uint).wrapping_mul(::std::mem::size_of::<UDWORD>()
                                                                                                                          as
                                                                                                                          libc::c_ulong))
                                                          as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as
                   libc::c_ulong); // find the droid
        IdToDroid(id, 99 as libc::c_int as UDWORD,
                  &mut psDroid); //droid not found, request it.
        if psDroid.is_null() {
            sendRequestDroid(id); // process the order.
            return 0 as libc::c_int
        }
        if bCmdOr == 0 {
            if (*psDroid).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint &&
                   !(*psDroid).psGroup.is_null() &&
                   (*(*psDroid).psGroup).type_0 as libc::c_int ==
                       GT_COMMAND as libc::c_int {
                grpLeave((*psDroid).psGroup, psDroid);
            }
        }
        ProcessDroidOrder(psDroid, order, x, y, desttype, destid);
        // no need to do formation stuff anymore.
        droidcount = droidcount.wrapping_sub(1)
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Droid update information
#[no_mangle]
pub unsafe extern "C" fn SendDroidInfo(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER, mut x: UDWORD,
                                       mut y: UDWORD,
                                       mut psObj: *mut BASE_OBJECT) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; //note: x,y not needed
    if myResponsibility((*psDroid).player as UDWORD) == 0 {
        return 1 as libc::c_int
    }
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut order as *mut DROID_ORDER as *const libc::c_void,
           ::std::mem::size_of::<DROID_ORDER>() as libc::c_ulong);
    if x == 0 as libc::c_int as libc::c_uint &&
           y == 0 as libc::c_int as libc::c_uint && psObj.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut x as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(12 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut y as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.body[16 as libc::c_int as usize] = 2 as libc::c_int as libc::c_char
    } else if psObj.is_null() {
        //it's a position order
        memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut x as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(12 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut y as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.body[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char
        // subtype flag
    } else {
        // it's a object order
        memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(12 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psObj).type_0 as *mut OBJECT_TYPE as
                   *const libc::c_void,
               ::std::mem::size_of::<OBJECT_TYPE>() as libc::c_ulong);
        m.body[16 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char
        // subtype flag
    } // set the type
    m.size =
        (4 as libc::c_int as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
            as libc::c_ushort;
    m.type_0 = NET_DROIDINFO as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
    // send it.
}
// ////////////////////////////////////////////////////////////////////////////
// receive droid information form other players.
#[no_mangle]
pub unsafe extern "C" fn recvDroidInfo(mut pMsg: *mut NETMSG) -> BOOL {
    let mut x: UDWORD = 0; //droid's id
    let mut y: UDWORD = 0; //droid's order
    let mut id: UDWORD = 0; //droid not found, request it.
    let mut destid: UDWORD = 0;
    let mut order: DROID_ORDER = DORDER_NONE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut desttype: OBJECT_TYPE = OBJ_DROID;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut order as *mut DROID_ORDER as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<DROID_ORDER>() as libc::c_ulong);
    if IdToDroid(id, 99 as libc::c_int as UDWORD, &mut psDroid) == 0 {
        sendRequestDroid(id);
        return 0 as libc::c_int
    }
    // now process the actual order..
    if (*pMsg).body[16 as libc::c_int as usize] as libc::c_int ==
           2 as libc::c_int {
        turnOffMultiMsg(1 as libc::c_int);
        orderDroid(psDroid, order);
        turnOffMultiMsg(0 as libc::c_int);
    } else if (*pMsg).body[16 as libc::c_int as usize] as libc::c_int ==
                  1 as libc::c_int {
        // it's an object order
        memcpy(&mut destid as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(8 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut desttype as *mut OBJECT_TYPE as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(12 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<OBJECT_TYPE>() as libc::c_ulong);
        ProcessDroidOrder(psDroid, order, 0 as libc::c_int as UDWORD,
                          0 as libc::c_int as UDWORD, desttype, destid);
    } else {
        // it's a position order
        memcpy(&mut x as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(8 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // x coord
        memcpy(&mut y as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(12 as libc::c_int as
                                                          isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // y coord
        ProcessDroidOrder(psDroid, order, x, y, OBJ_DROID,
                          0 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// process droid order
unsafe extern "C" fn ProcessDroidOrder(mut psDroid: *mut DROID,
                                       mut order: DROID_ORDER, mut x: UDWORD,
                                       mut y: UDWORD,
                                       mut desttype: OBJECT_TYPE,
                                       mut destid: UDWORD) {
    let mut i: UDWORD = 0;
    let mut pD: *mut DROID = 0 as *mut DROID;
    let mut pS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if destid == 0 as libc::c_int as libc::c_uint &&
           desttype as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        // target is a location
        if abs(((*psDroid).x as libc::c_uint).wrapping_sub(x) as libc::c_int)
               < 128 as libc::c_int / 2 as libc::c_int &&
               abs(((*psDroid).y as libc::c_uint).wrapping_sub(y) as
                       libc::c_int) < 128 as libc::c_int / 2 as libc::c_int {
            // don't bother if close.
            return
        }
        if order as libc::c_uint == 99 as libc::c_int as libc::c_uint {
            // get the order to do if we didn't specify it.
            order = chooseOrderLoc(psDroid, x, y)
        }
        turnOffMultiMsg(1 as libc::c_int);
        orderDroidLoc(psDroid, order, x, y);
        turnOffMultiMsg(0 as libc::c_int);
    } else {
        //  target is object
        match desttype as libc::c_uint {
            0 => {
                psObj = 0 as *mut BASE_OBJECT;
                i = 0 as libc::c_int as UDWORD;
                while i < 8 as libc::c_int as libc::c_uint && psObj.is_null()
                      {
                    pD = apsDroidLists[i as usize];
                    while !pD.is_null() && (*pD).id != destid {
                        pD = (*pD).psNext
                    }
                    if !pD.is_null() { psObj = pD as *mut BASE_OBJECT }
                    i = i.wrapping_add(1)
                }
            }
            1 => {
                psObj = 0 as *mut BASE_OBJECT;
                pS = IdToStruct(destid, 99 as libc::c_int as UDWORD);
                if !pS.is_null() { psObj = pS as *mut BASE_OBJECT }
            }
            2 => {
                psObj = 0 as *mut BASE_OBJECT;
                pF = IdToFeature(destid, 99 as libc::c_int as UDWORD);
                if !pF.is_null() { psObj = pF as *mut BASE_OBJECT }
            }
            3 => {
                // shouldn't be getting this!
                debug(LOG_ERROR,
                      b"multibot: order specified destination as a bullet. what am i to do??\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            _ => {
                debug(LOG_ERROR,
                      b"unknown object type\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
        }
        if psObj.is_null() {
            // failed to find it;
            return
        }
        // if we didn't sepcify an order, then make one..
        if order as libc::c_uint == 99 as libc::c_int as libc::c_uint {
            // get the order to do if we didn't specify it.
            order = chooseOrderObj(psDroid, psObj)
        }
        turnOffMultiMsg(1 as libc::c_int);
        orderDroidObj(psDroid, order, psObj);
        turnOffMultiMsg(0 as libc::c_int);
    };
}
// ////////////////////////////////////////////////////////////////////////////
// Inform other players that a droid has been destroyed
#[no_mangle]
pub unsafe extern "C" fn SendDestroyDroid(mut pD: *mut DROID) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // id of the droid to be destroyed
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        ::std::mem::size_of::<UDWORD>() as libc::c_ulong as libc::c_ushort;
    m.type_0 = NET_DROIDDEST as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 1 as libc::c_int);
    //guaranteed msg?????
}
// ////////////////////////////////////////////////////////////////////////////
// Accept a droid which was destroyed on another machine
#[no_mangle]
pub unsafe extern "C" fn recvDestroyDroid(mut pMsg: *mut NETMSG) -> BOOL {
    let mut pD: *mut DROID = 0 as *mut DROID; // get the id of the droid.
    let mut r: UDWORD =
        0; // remove the droid recvd from the remote players machine.
    memcpy(&mut r as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    if IdToDroid(r, 99 as libc::c_int as UDWORD, &mut pD) != 0 {
        if (*pD).died == 0 {
            turnOffMultiMsg(1 as libc::c_int);
            destroyDroid(pD);
            turnOffMultiMsg(0 as libc::c_int);
        }
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// stuff for sending the WHOLE of a droid!
#[no_mangle]
pub unsafe extern "C" fn sendWholeDroid(mut pD: *mut DROID, mut dest: DPID)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut sizecount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut noTarget: UDWORD = 0 as libc::c_int as UDWORD;
    let mut asParts: [SDWORD; 8] = [0; 8];
    let mut asWeaps: [UDWORD; 1] = [0; 1];
    if (*pD).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        // build some bits for the template.
        asWeaps[0 as libc::c_int as usize] =
            (*pD).asWeaps[0 as libc::c_int as usize].nStat
    } else {
        asWeaps[0 as libc::c_int as usize] = 0 as libc::c_int as UDWORD
    } //allocate the components
    asParts[COMP_BODY as libc::c_int as usize] =
        (*pD).asBits[COMP_BODY as libc::c_int as usize].nStat as
            SDWORD; // to build a template.
    asParts[COMP_BRAIN as libc::c_int as usize] =
        (*pD).asBits[COMP_BRAIN as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_PROPULSION as libc::c_int as usize] =
        (*pD).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_SENSOR as libc::c_int as usize] =
        (*pD).asBits[COMP_SENSOR as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_ECM as libc::c_int as usize] =
        (*pD).asBits[COMP_ECM as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_REPAIRUNIT as libc::c_int as usize] =
        (*pD).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_CONSTRUCT as libc::c_int as usize] =
        (*pD).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat as SDWORD;
    asParts[COMP_WEAPON as libc::c_int as usize] =
        (*pD).asBits[COMP_WEAPON as libc::c_int as usize].nStat as SDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut asParts as *mut [SDWORD; 8] as *const libc::c_void,
           ::std::mem::size_of::<[SDWORD; 8]>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[SDWORD; 8]>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut asWeaps as *mut [UDWORD; 1] as *const libc::c_void,
           ::std::mem::size_of::<[UDWORD; 1]>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UDWORD; 1]>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).x as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).y as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).z as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).player as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    strcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize),
           (*pD).aName.as_mut_ptr());
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(strlen((*pD).aName.as_mut_ptr()).wrapping_add(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint))
            as UDWORD as UDWORD;
    // that's enough to build a template, now the specific stuff!
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).NameVersion as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).droidType as *mut DROID_TYPE as *const libc::c_void,
           ::std::mem::size_of::<DROID_TYPE>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<DROID_TYPE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).direction as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).pitch as *mut SWORD as *const libc::c_void,
           ::std::mem::size_of::<SWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).roll as *mut SWORD as *const libc::c_void,
           ::std::mem::size_of::<SWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).visible as *mut [UBYTE; 8] as *const libc::c_void,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UBYTE; 8]>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).inFire as *mut BOOL as *const libc::c_void,
           ::std::mem::size_of::<BOOL>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<BOOL>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).burnDamage as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).body as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).secondaryOrder as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).order as *mut SDWORD as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).orderX as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).orderY as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).orderX2 as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).orderY2 as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    if !(*pD).psTarget.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*(*pD).psTarget).id as *mut UDWORD as
                   *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD
    } else {
        memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut noTarget as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD
    }
    if !(*pD).psTarStats.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*(*pD).psTarStats).ref_0 as *mut UDWORD as
                   *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD
    } else {
        memcpy(&mut *m.body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut noTarget as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD
    }
    m.type_0 = NET_WHOLEDROID as libc::c_int as libc::c_uchar;
    m.size = sizecount as UWORD;
    return NETsend(&mut m, dest, 0 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
// receive a whole droid!!!!
#[no_mangle]
pub unsafe extern "C" fn receiveWholeDroid(mut m: *mut NETMSG) -> BOOL {
    let mut sizecount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut dt: DROID_TEMPLATE =
        DROID_TEMPLATE{ref_0: 0,
                       pName: 0 as *mut STRING,
                       aName: [0; 60],
                       NameVersion: 0,
                       asParts: [0; 8],
                       buildPoints: 0,
                       powerPoints: 0,
                       storeCount: 0,
                       numWeaps: 0,
                       asWeaps: [0; 1],
                       droidType: DROID_WEAPON,
                       multiPlayerID: 0,
                       psNext: 0 as *mut _droid_template,};
    let mut pD: *mut DROID = 0 as *mut DROID;
    let mut existingdroid: *mut DROID = 0 as *mut DROID;
    //	BOOL			temp= FALSE;
    let mut tempDroid: *mut DROIDSTORE = 0 as *mut DROIDSTORE;
    //UDWORD x,y,z,id;
    let mut x: UWORD = 0;
    let mut y: UWORD = 0;
    let mut z: UWORD = 0;
    let mut id: UDWORD = 0;
    let mut player: UBYTE = 0;
    // get the stuff
    memcpy(&mut dt.asParts as *mut [SDWORD; 8] as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[SDWORD; 8]>() as
               libc::c_ulong); // build a template
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[SDWORD; 8]>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    //	NetGet(m,sizecount,dt.powerPoints);			sizecount+=sizeof(dt.powerPoints);
    memcpy(&mut dt.asWeaps as *mut [UDWORD; 1] as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[UDWORD; 1]>() as libc::c_ulong); // edit it.
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UDWORD; 1]>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD; // just in case.
    memcpy(&mut x as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as
               libc::c_ulong); // name is pointed at directly into the buffer.
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut y as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut z as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut player as *mut UBYTE as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    dt.pName = &mut dt.aName as *mut [STRING; 60] as *mut CHAR;
    strncpy(dt.aName.as_mut_ptr(),
            &mut *(*m).body.as_mut_ptr().offset(sizecount as isize),
            (60 as libc::c_int - 1 as libc::c_int) as libc::c_uint);
    dt.aName[(60 as libc::c_int - 1 as libc::c_int) as usize] =
        0 as libc::c_int as STRING;
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(strlen(dt.pName).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
            as UDWORD as UDWORD;
    if dt.asWeaps[0 as libc::c_int as usize] ==
           0 as libc::c_int as libc::c_uint {
        dt.numWeaps = 0 as libc::c_int as UDWORD
    } else { dt.numWeaps = 1 as libc::c_int as UDWORD }
    dt.powerPoints = calcTemplatePower(&mut dt);
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    if IdToDroid(id, 99 as libc::c_int as UDWORD, &mut existingdroid) != 0 {
        // if a droid of id already exists then go no further.
        return 0 as libc::c_int
    }
    // could do usepower , but we usually do this in an emergency, so leave it!
    turnOffMultiMsg(1 as libc::c_int); // make a droid
    pD =
        buildDroid(&mut dt, x as UDWORD, y as UDWORD, player as UDWORD,
                   0 as libc::c_int);
    turnOffMultiMsg(0 as libc::c_int);
    if pD.is_null() {
        // failed to build it, give up.
        return 0 as libc::c_int
    }
    // now the instance specific stuff.
    (*pD).id = id; //correct builddroid to use exact pos, not tile center
    (*pD).x = x; // processed laater
    (*pD).y = y; //later!
    (*pD).z = z; //later!
    memcpy(&mut (*pD).NameVersion as *mut UBYTE as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong); //later!
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD; //later!
    memcpy(&mut (*pD).droidType as *mut DROID_TYPE as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<DROID_TYPE>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<DROID_TYPE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).direction as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).pitch as *mut SWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).roll as *mut SWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).visible as *mut [UBYTE; 8] as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UBYTE; 8]>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).inFire as *mut BOOL as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<BOOL>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<BOOL>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).burnDamage as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).body as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut (*pD).secondaryOrder as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    sizecount =
        (sizecount as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    if ingame.localJoiningInProgress != 0 {
        memcpy(&mut (*pD).order as *mut SDWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).orderX as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).orderY as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).orderX2 as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).orderY2 as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).psTarget as *mut *mut _base_object as
                   *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<*mut _base_object>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<*mut _base_object>()
                                                as libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut (*pD).psTarStats as *mut *mut _base_stats as
                   *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(sizecount as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<*mut _base_stats>() as libc::c_ulong);
        sizecount =
            (sizecount as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<*mut _base_stats>()
                                                as libc::c_ulong) as UDWORD as
                UDWORD;
        //store the droid for later.
        tempDroid =
            memMallocRelease(::std::mem::size_of::<DROIDSTORE>() as
                                 libc::c_ulong) as *mut DROIDSTORE;
        (*tempDroid).psDroid = pD;
        (*tempDroid).psNext = tempDroidList as *mut libc::c_void;
        tempDroidList = tempDroid
    } else {
        //don't bother setting the orders. they'll update sooner or later anywho.
        (*pD).order = 0 as libc::c_int;
        (*pD).orderX = 0 as libc::c_int as UWORD;
        (*pD).orderY = 0 as libc::c_int as UWORD;
        (*pD).psTarget = 0 as *mut _base_object;
        (*pD).psTarStats = 0 as *mut _base_stats;
        addDroid(pD, apsDroidLists.as_mut_ptr());
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Functions for cases where a machine receives a netmessage about a certain
// droid. The droid is unknown, so the machine uses tese functions in order to
// find out about it.
#[no_mangle]
pub unsafe extern "C" fn sendRequestDroid(mut droidId: UDWORD) -> BOOL {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if ingame.localJoiningInProgress != 0 {
        // dont worry if still joining.
        return 0 as libc::c_int
    }
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut droidId as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut *player2dpid.as_mut_ptr().offset(selectedPlayer as isize) as
               *mut DWORD as *const libc::c_void,
           ::std::mem::size_of::<DWORD>() as libc::c_ulong);
    debug(LOG_NEVER,
          b"multibot: unknown droid %d, requesting info\n\x00" as *const u8 as
              *const libc::c_char, droidId);
    msg.type_0 = NET_REQUESTDROID as libc::c_int as libc::c_uchar;
    msg.size =
        (::std::mem::size_of::<DPID>() as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                             libc::c_ulong) as libc::c_ushort;
    NETbcast(&mut msg, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn recvRequestDroid(mut pMsg: *mut NETMSG) -> BOOL {
    let mut pDroid: *mut DROID = 0 as *mut DROID; // get the droid's id
    let mut droidid: UDWORD = 0; // get the player who needs it.
    let mut dpid: UDWORD = 0;
    memcpy(&mut droidid as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut dpid as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    if IdToDroid(droidid, 99 as libc::c_int as UDWORD, &mut pDroid) == 0 {
        // find the droid
        return 1 as libc::c_int
        // can't find it, so ignore.
    }
    if myResponsibility((*pDroid).player as UDWORD) != 0 {
        // if resposible
        sendWholeDroid(pDroid, dpid as DPID);
        // send the whole of the droid
    }
    return 1 as libc::c_int;
}
