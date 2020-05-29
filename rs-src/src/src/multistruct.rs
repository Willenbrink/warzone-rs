use ::libc;
extern "C" {
    pub type _formation;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    #[no_mangle]
    fn droidStartBuild(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn setUpBuildModule(psDroid: *mut DROID);
    #[no_mangle]
    fn getTileStructure(x: UDWORD, y: UDWORD) -> *mut STRUCTURE;
    /*this is called whenever a structure has finished building*/
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    // La!
    #[no_mangle]
    fn IsStatExpansionModule(psStats: *mut STRUCTURE_STATS) -> BOOL;
    #[no_mangle]
    fn destroyStruct(psDel: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn buildStructure(pStructureType: *mut STRUCTURE_STATS, x: UDWORD,
                      y: UDWORD, player: UDWORD, FromSave: BOOL)
     -> *mut STRUCTURE;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
    #[no_mangle]
    fn getDroidDestination(psPositionStats: *mut BASE_STATS, structX: UDWORD,
                           structY: UDWORD, pDroidX: *mut UDWORD,
                           pDroidY: *mut UDWORD) -> BOOL;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn proj_SendProjectile(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                           player: SDWORD, tarX: UDWORD, tarY: UDWORD,
                           tarZ: UDWORD, psTarget: *mut BASE_OBJECT,
                           bVisible: BOOL) -> BOOL;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
    //#define DMATCH					11			// to easily distinguish game types when joining.
    //#define MULTI_SKIRMISHA			20
    //#define MULTI_CAMPAIGNA			17
    // limit options for dmatch.
    // campaign subtypes
    // game templates are stored in player x.
    // this ping is kickin'.
    // this ping is crusin'.
    // this ping is crawlin'.
    // if ping is bigger than this, then worry and panic.
    // how many points to allocate for res levels???
    // ////////////////////////////////////////////////////////////////////////////
// macros For handling net messages, just copy things in & out of the msg buffer
/*
#define NetAdd(m,pos,thing) \
	memcpy(&(m.body[pos]),&(thing),sizeof(thing)) 

#define NetAdd2(m,pos,thing) \
	memcpy( &((*m).body[pos]), &(thing), sizeof(thing)) 

#define NetAddSt(m,pos,stri) \
	strcpy(&(m.body[pos]),stri)

#define NetGet(m,pos,thing) \
	memcpy(&(thing),&(m->body[pos]),sizeof(thing))

#define NetGetSt(m,pos,stri) \
	strcpy(stri,&(m->body[pos]))
*/
// ////////////////////////////////////////////////////////////////////////////
// functions
    #[no_mangle]
    fn IdToPointer(id: UDWORD, player: UDWORD) -> *mut BASE_OBJECT;
    #[no_mangle]
    fn IdToStruct(id: UDWORD, player: UDWORD) -> *mut STRUCTURE;
    #[no_mangle]
    fn IdToDroid(id: UDWORD, player: UDWORD, psDroid: *mut *mut DROID)
     -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn technologyGiveAway(pS: *mut STRUCTURE);
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
pub type CHAR = libc::c_char;
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
/* **************************************************************************/
/*
 * pieTypes.h
 *
 * type defines for simple pies.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
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
pub type BASE_STATS = _base_stats;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
pub type STRUCTURE = _structure;
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
pub type C2RustUnnamed = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed = 350;
pub const ID_SOUND_EMP: C2RustUnnamed = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed = 301;
pub const ID_SOUND_HELP: C2RustUnnamed = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed = 160;
pub const ID_GIFT: C2RustUnnamed = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed = 109;
pub const ID_SOUND_NO: C2RustUnnamed = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed = 0;
pub const NO_SOUND: C2RustUnnamed = -1;
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
// ///////////////////////////////////////////////////////
// definitions of functions in multiplay's other c files.
// Buildings . multistruct
// unique ID creation thing..
// ////////////////////////////////////////////////////////////////////////////
// Local Functions
//BOOL		levelGround				(UDWORD atx, UDWORD aty, UDWORD atz, UDWORD baseWidth,UDWORD baseBreadth);
// ////////////////////////////////////////////////////////////////////////////
// Level ground. A small function for updating the ground below a structure.
// if and when this gets broken the code was pinched from droidupdatefoundation();
/*BOOL levelGround(UDWORD atx, UDWORD aty, UDWORD atz, UDWORD baseWidth,UDWORD baseBreadth)
{
	UDWORD				x = atx >> TILE_SHIFT;
	UDWORD				y = aty >> TILE_SHIFT;
	UBYTE				width, breadth;

	for (breadth=0; breadth < (UBYTE)(baseBreadth + 1); breadth++)
	{
		for (width=0; width < (UBYTE)(baseWidth + 1); width++)
		{
			while((x+width-1)<0)width++;
			while((y+breadth-1)<0)breadth++;

			setTileHeight(x + width-1, y + breadth-1, atz);
		}
	}
	return TRUE;
}
*/
// ////////////////////////////////////////////////////////////////////////////
// structures
// ////////////////////////////////////////////////////////////////////////////
// INFORM others that a building has been started, and base plate should be put down.
#[no_mangle]
pub unsafe extern "C" fn sendBuildStarted(mut psStruct: *mut STRUCTURE,
                                          mut psDroid: *mut DROID) -> BOOL {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; //player
    let mut zero: UDWORD = 0 as libc::c_int as UDWORD; //id of thing to build
    let mut player: UWORD = 0; // x
    let mut order: UWORD = 0; // y
    player = (*psDroid).player as UWORD; // droid to order to build it
    order = (*psDroid).order as UBYTE as UWORD; // building id to create
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as
               libc::c_ulong); // building id to create
    memcpy(&mut *msg.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*(*psDroid).psTarStats).ref_0 as *mut UDWORD as
               *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(5 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).orderX as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(7 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).orderY as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(11 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(15 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(19 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut order as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    if !(*psDroid).psTarget.is_null() &&
           (*(*psDroid).psTarget).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        memcpy(&mut *msg.body.as_mut_ptr().offset(20 as libc::c_int as isize)
                   as *mut libc::c_char as *mut libc::c_void,
               &mut (*((*psDroid).psTarget as *mut STRUCTURE)).id as
                   *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    } else {
        memcpy(&mut *msg.body.as_mut_ptr().offset(20 as libc::c_int as isize)
                   as *mut libc::c_char as *mut libc::c_void,
               &mut zero as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    }
    memcpy(&mut *msg.body.as_mut_ptr().offset(24 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).z as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    msg.size = 28 as libc::c_int as libc::c_ushort;
    msg.type_0 = NET_BUILD as libc::c_int as libc::c_uchar;
    return NETbcast(&mut msg, 0 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
// put down a base plate and start droid building it!
#[no_mangle]
pub unsafe extern "C" fn recvBuildStarted(mut pMsg: *mut NETMSG) -> BOOL {
    //UDWORD			targetId,order,droidId,structId,x,z,y,structStat,player;
    let mut targetId: UDWORD = 0; // decode message.
    let mut order: UDWORD = 0; // find structure target
    let mut droidId: UDWORD = 0;
    let mut structId: UDWORD = 0;
    let mut structStat: UDWORD = 0;
    let mut x: UWORD = 0;
    let mut z: UWORD = 0;
    let mut y: UWORD = 0;
    let mut player: UWORD = 0;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut typeindex: UDWORD = 0;
    let mut actionX: UDWORD = 0;
    let mut actionY: UDWORD = 0;
    player = (*pMsg).body[0 as libc::c_int as usize] as UWORD;
    memcpy(&mut structStat as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut x as *mut UWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(5 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut y as *mut UWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(7 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    memcpy(&mut droidId as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(11 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut structId as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(15 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    order = (*pMsg).body[19 as libc::c_int as usize] as UDWORD;
    memcpy(&mut targetId as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(20 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut z as *mut UWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(24 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    typeindex = 0 as libc::c_int as UDWORD;
    while typeindex < numStructureStats &&
              (*asStructureStats.offset(typeindex as isize)).ref_0 !=
                  structStat {
        typeindex = typeindex.wrapping_add(1)
    }
    psStats =
        &mut *asStructureStats.offset(typeindex as isize) as
            *mut STRUCTURE_STATS;
    if IdToDroid(droidId, player as UDWORD, &mut psDroid) != 0 {
        if getDroidDestination(psStats as *mut BASE_STATS, x as UDWORD,
                               y as UDWORD, &mut actionX, &mut actionY) != 0 {
            (*psDroid).order = order as SDWORD;
            if (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
                (*psDroid).order = DORDER_BUILD as libc::c_int
            }
            (*psDroid).orderX = x;
            (*psDroid).orderY = y;
            (*psDroid).psTarStats = psStats as *mut BASE_STATS;
            if targetId != 0 {
                (*psDroid).psTarget =
                    IdToPointer(targetId, 99 as libc::c_int as UDWORD)
            } else { (*psDroid).psTarget = 0 as *mut _base_object }
            if IsStatExpansionModule(psStats) != 0 {
                setUpBuildModule(psDroid);
            } else {
                droidStartBuild(psDroid);
                (*psDroid).action = DACTION_BUILD as libc::c_int
            }
        }
        if !(*psDroid).psTarget.is_null() {
            //sync id's
            (*((*psDroid).psTarget as *mut STRUCTURE)).id = structId
        }
    }
    // order droid to start building it.
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// INFORM others that a building has been completed.
#[no_mangle]
pub unsafe extern "C" fn SendBuildFinished(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; //id of finished struct
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    // also enough info to build it if we don't already know about it.
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*(*psStruct).pStructureType).ref_0 as *mut UDWORD as
               *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); // kind of building.
    memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).x as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // x pos
    memcpy(&mut *m.body.as_mut_ptr().offset(10 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).y as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // y pos
    memcpy(&mut *m.body.as_mut_ptr().offset(12 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).z as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // y pos
    m.body[14 as libc::c_int as usize] =
        (*psStruct).player as libc::c_char; // player
    m.size = 15 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_BUILDFINISHED as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn recvBuildFinished(mut m: *mut NETMSG) -> BOOL {
    let mut strId: UDWORD = 0; //,i;
    let mut psStr: *mut STRUCTURE = 0 as *mut STRUCTURE; // get the struct id.
    let mut x: UWORD = 0;
    let mut y: UWORD = 0;
    let mut z: UWORD = 0;
    let mut type_0: UDWORD = 0;
    let mut typeindex: UDWORD = 0;
    let mut player: UBYTE = 0;
    memcpy(&mut strId as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    psStr = IdToStruct(strId, 99 as libc::c_int as UDWORD);
    if !psStr.is_null() {
        // make it complete.
        (*psStr).currentBuildPts =
            (*(*psStr).pStructureType).buildPoints.wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                as SWORD;
        if (*psStr).status as libc::c_int != SS_BUILT as libc::c_int {
            (*psStr).status = SS_BUILT as libc::c_int as UBYTE;
            buildingComplete(psStr);
        }
        NETlogEntry(b"building finished ok.\x00" as *const u8 as
                        *const libc::c_char as *mut CHAR,
                    0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD);
        return 1 as libc::c_int
    }
    // the building wasn't started, so we'll have to just plonk it down in the map.
    memcpy(&mut type_0 as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); // kind of building.
    memcpy(&mut x as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(8 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // x pos
    memcpy(&mut y as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(10 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // y pos
    memcpy(&mut z as *mut UWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(12 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong); // z pos
    player = (*m).body[14 as libc::c_int as usize] as UBYTE; // player
    typeindex = 0 as libc::c_int as UDWORD; // find structure target
    while typeindex < numStructureStats &&
              (*asStructureStats.offset(typeindex as isize)).ref_0 != type_0 {
        typeindex = typeindex.wrapping_add(1)
    }
    psStr = 0 as *mut STRUCTURE;
    // check for similar buildings, to avoid overlaps
    if (*mapTile((x as libc::c_int >> 7 as libc::c_int) as UDWORD,
                 (y as libc::c_int >> 7 as libc::c_int) as
                     UDWORD)).tileInfoBits as libc::c_int & 0x1 as libc::c_int
           != 0 {
        // get structure;
        psStr =
            getTileStructure((x as libc::c_int >> 7 as libc::c_int) as UDWORD,
                             (y as libc::c_int >> 7 as libc::c_int) as
                                 UDWORD);
        if (*asStructureStats.offset(typeindex as isize)).type_0 ==
               (*(*psStr).pStructureType).type_0 {
            // correct type, correct location, just rename the id's to sync it.. (urgh)
            (*psStr).id = strId;
            (*psStr).status = SS_BUILT as libc::c_int as UBYTE;
            buildingComplete(psStr);
            NETlogEntry(b"structure id modified\x00" as *const u8 as
                            *const libc::c_char as *mut CHAR,
                        0 as libc::c_int as UDWORD, player as UDWORD);
            return 1 as libc::c_int
        }
    }
    psStr =
        buildStructure(&mut *asStructureStats.offset(typeindex as isize),
                       x as UDWORD, y as UDWORD, player as UDWORD,
                       1 as libc::c_int);
    if !psStr.is_null() {
        (*psStr).id = strId;
        (*psStr).status = SS_BUILT as libc::c_int as UBYTE;
        buildingComplete(psStr);
        NETlogEntry(b"had to plonk down a building\x00" as *const u8 as
                        *const libc::c_char as *mut CHAR,
                    0 as libc::c_int as UDWORD, player as UDWORD);
    } else {
        NETlogEntry(b"had to plonk down a building, BUT FAILED OH S**T.\x00"
                        as *const u8 as *const libc::c_char as *mut CHAR,
                    0 as libc::c_int as UDWORD, player as UDWORD);
    }
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// demolish message.
#[no_mangle]
pub unsafe extern "C" fn SendDemolishFinished(mut psStruct: *mut STRUCTURE,
                                              mut psDroid: *mut DROID)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // demolish it.
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psDroid).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        (2 as libc::c_int as
             libc::c_uint).wrapping_mul(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as libc::c_ushort;
    m.type_0 = NET_DEMOLISH as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvDemolishFinished(mut m: *mut NETMSG) -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut s: UDWORD = 0;
    let mut d: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    memcpy(&mut s as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut d as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    psStruct = IdToStruct(s, 99 as libc::c_int as UDWORD);
    IdToDroid(d, 99 as libc::c_int as UDWORD, &mut psDroid);
    if !psStruct.is_null() {
        removeStruct(psStruct, 1 as libc::c_int);
        if !psDroid.is_null() && !(*psDroid).psTarStats.is_null() {
            (*psDroid).psTarStats = 0 as *mut _base_stats
            // update droid if reqd.
        }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Inform others that a structure has been destroyed
#[no_mangle]
pub unsafe extern "C" fn SendDestroyStructure(mut s: *mut STRUCTURE) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // struct to destroy
    technologyGiveAway(s);
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*s).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        ::std::mem::size_of::<UDWORD>() as libc::c_ulong as libc::c_ushort;
    m.type_0 = NET_STRUCTDEST as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
// acknowledge the destruction of a structure, from another player.
#[no_mangle]
pub unsafe extern "C" fn recvDestroyStructure(mut m: *mut NETMSG) -> BOOL {
    let mut s: UDWORD = 0; // struct to destory
    let mut psStr: *mut STRUCTURE =
        0 as *mut STRUCTURE; // remove the struct from remote players machine.
    memcpy(&mut s as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    psStr = IdToStruct(s, 99 as libc::c_int as UDWORD);
    if !psStr.is_null() {
        turnOffMultiMsg(1 as libc::c_int);
        destroyStruct(psStr);
        turnOffMultiMsg(0 as libc::c_int);
        technologyGiveAway(psStr);
        return 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
//lassat is firing
#[no_mangle]
pub unsafe extern "C" fn sendLasSat(mut player: UBYTE,
                                    mut psStruct: *mut STRUCTURE,
                                    mut psObj: *mut BASE_OBJECT) -> BOOL {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut p: UBYTE = 0;
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psStruct).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut *msg.body.as_mut_ptr().offset(5 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*psObj).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    p = (*psObj).player;
    memcpy(&mut *msg.body.as_mut_ptr().offset(9 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut p as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    msg.size = 10 as libc::c_int as libc::c_ushort;
    msg.type_0 = NET_LASSAT as libc::c_int as libc::c_uchar;
    return NETbcast(&mut msg, 0 as libc::c_int);
}
// recv lassat info on the receiving end.
#[no_mangle]
pub unsafe extern "C" fn recvLasSat(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut player: UBYTE = 0;
    let mut targetplayer: UBYTE = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut id: UDWORD = 0;
    let mut tid: UDWORD = 0;
    memcpy(&mut player as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut tid as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(5 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut targetplayer as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(9 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    psStruct = IdToStruct(id, player as UDWORD);
    psObj = IdToPointer(tid, targetplayer as UDWORD);
    if !psStruct.is_null() && !psObj.is_null() {
        proj_SendProjectile(&mut *(*psStruct).asWeaps.as_mut_ptr().offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize),
                            0 as *mut BASE_OBJECT, player as SDWORD,
                            (*psObj).x as UDWORD, (*psObj).y as UDWORD,
                            (*psObj).z as UDWORD, psObj, 1 as libc::c_int);
        //play 5 second countdown message
        audio_QueueTrackPos(ID_SOUND_LAS_SAT_COUNTDOWN as libc::c_int,
                            (*psObj).x as SDWORD, (*psObj).y as SDWORD,
                            (*psObj).z as SDWORD);
    }
    return 1 as libc::c_int;
}
