use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    /* Function to create a heap
 * Takes the size of the objects to be managed by the heap,
 * the initial number of objects to allocate and the number of
 * objects to allocate when the heap is extended.
 * Returns an initialised OBJ_HEAP structure.
 */
    #[no_mangle]
    fn heapCreate(ppsHeap: *mut *mut OBJ_HEAP, size: UDWORD, init: UDWORD,
                  ext: UDWORD) -> BOOL;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    /* Destroy a heap and release all the memory associated with it */
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
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
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
/* structure used to store the extra space allocated for the heap */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
pub type OBJ_HEAP = _obj_heap;
// Extension memory for the heap
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
pub type DROID = _droid;
pub type STRUCTURE = _structure;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _grid_array {
    pub apsObjects: [*mut BASE_OBJECT; 32],
    pub psNext: *mut _grid_array,
}
/* The type of feature */
// Graphic for the feature
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
//done in script files now
	/* component type activated if a FEAT_GEN_ARTE */
	//UDWORD			compType;			// type of component activated
	//UDWORD			compIndex;			// index of component
/* Flag to indicated whether the tile needs to be drawn
										   TRUE = draw tile */
/* Flag to indicate whether the feature allows the LOS
										   TRUE = can see through the feature */
/* Flag to indicate whether the feature is visible at
										   the start of the mission */
// Whether the feature can be blown up
// Number of body points
// Feature armour
// Objects are stored in an extensible array for each grid
pub type GRID_ARRAY = _grid_array;
// what to do when calculating the coverage of an object
pub type COVERAGE_MODE = _coverage_mode;
pub type _coverage_mode = libc::c_uint;
pub const GRID_REMOVEOBJECT: _coverage_mode = 1;
pub const GRID_ADDOBJECT: _coverage_mode = 0;
// Initial and extension sizes for the grid heap
//#define GRID_HEAPINIT	(GRID_WIDTH*GRID_HEIGHT)
#[no_mangle]
pub static mut gridWidth: UDWORD = 0;
#[no_mangle]
pub static mut gridHeight: UDWORD = 0;
// The heap for the grid arrays
#[no_mangle]
pub static mut psGridHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// The map grid
#[no_mangle]
pub static mut apsMapGrid: [*mut GRID_ARRAY; 1024] =
    [0 as *const GRID_ARRAY as *mut GRID_ARRAY; 1024];
//GRID_ARRAY	*apsMapGrid[GRID_WIDTH][GRID_HEIGHT];
// which grid to garbage collect on next
#[no_mangle]
pub static mut garbageX: SDWORD = 0;
#[no_mangle]
pub static mut garbageY: SDWORD = 0;
// the current state of the iterator
#[no_mangle]
pub static mut psIterateGrid: *mut GRID_ARRAY =
    0 as *const GRID_ARRAY as *mut GRID_ARRAY;
#[no_mangle]
pub static mut iterateEntry: SDWORD = 0;
// The size of the grid
//#define GRID_WIDTH	(MAP_MAXWIDTH/GRID_SIZE)
//#define GRID_HEIGHT	(MAP_MAXHEIGHT/GRID_SIZE)
// The map grid 
//extern GRID_ARRAY	*apsMapGrid[GRID_WIDTH][GRID_HEIGHT];
// initialise the grid system
// initialise the grid system
#[no_mangle]
pub unsafe extern "C" fn gridInitialise() -> BOOL {
    if heapCreate(&mut psGridHeap,
                  ::std::mem::size_of::<GRID_ARRAY>() as libc::c_ulong,
                  (256 as libc::c_int * 256 as libc::c_int /
                       (8 as libc::c_int * 8 as libc::c_int)) as UDWORD,
                  4 as libc::c_int as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    //	memset(apsMapGrid, 0, sizeof(GRID_ARRAY *) * GRID_WIDTH * GRID_HEIGHT);
    memset(apsMapGrid.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<*mut GRID_ARRAY>() as
                libc::c_ulong).wrapping_mul((256 as libc::c_int *
                                                 256 as libc::c_int /
                                                 (8 as libc::c_int *
                                                      8 as libc::c_int)) as
                                                libc::c_uint));
    garbageX = 0 as libc::c_int;
    garbageY = 0 as libc::c_int;
    psIterateGrid = 0 as *mut GRID_ARRAY;
    iterateEntry = 0 as libc::c_int;
    return 1 as libc::c_int;
}
//clear the grid of everything on it
//clear the grid of everything on it
#[no_mangle]
pub unsafe extern "C" fn gridClear() {
    let mut psCurr: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psNext: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    debug(LOG_NEVER,
          b"gridClear %d %d\n\x00" as *const u8 as *const libc::c_char,
          gridWidth, gridHeight);
    //	for(x=0; x<GRID_WIDTH; x+=1)
    x = 0 as libc::c_int;
    while (x as libc::c_uint) < gridWidth {
        //		for(y=0; y<GRID_HEIGHT; y+=1)
        y = 0 as libc::c_int;
        while (y as libc::c_uint) < gridHeight {
            //			for(psCurr = apsMapGrid[x][y]; psCurr; psCurr = psNext)
            psCurr =
                apsMapGrid[(y as
                                libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                                       as
                                                                                       libc::c_uint)
                               as usize];
            while !psCurr.is_null() {
                psNext = (*psCurr).psNext;
                heapFree(psGridHeap, psCurr as *mut libc::c_void);
                psCurr = psNext
            }
            //			apsMapGrid[x][y] = NULL;
            apsMapGrid[(y as
                            libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                                   as
                                                                                   libc::c_uint)
                           as usize] = 0 as *mut GRID_ARRAY;
            y += 1 as libc::c_int
        }
        x += 1 as libc::c_int
    };
}
// reset the grid system
// reset the grid system
#[no_mangle]
pub unsafe extern "C" fn gridReset() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut inc: UBYTE = 0;
    // Setup the grid dimensions.
    gridWidth =
        mapWidth.wrapping_add(8 as libc::c_int as
                                  libc::c_uint).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_div(8
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint);
    gridHeight =
        mapHeight.wrapping_add(8 as libc::c_int as
                                   libc::c_uint).wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_div(8
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint);
    gridClear();
    //put all the existing objects into the grid
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_int) < 8 as libc::c_int {
        psDroid = apsDroidLists[inc as usize];
        while !psDroid.is_null() {
            gridAddObject(psDroid as *mut BASE_OBJECT);
            psDroid = (*psDroid).psNext
        }
        psStruct = apsStructLists[inc as usize];
        while !psStruct.is_null() {
            gridAddObject(psStruct as *mut BASE_OBJECT);
            psStruct = (*psStruct).psNext
        }
        psFeature = apsFeatureLists[inc as usize];
        while !psFeature.is_null() {
            gridAddObject(psFeature as *mut BASE_OBJECT);
            psFeature = (*psFeature).psNext
        }
        inc = inc.wrapping_add(1)
    };
}
// shutdown the grid system
// shutdown the grid system
#[no_mangle]
pub unsafe extern "C" fn gridShutDown() {
    //gridReset();
    gridClear();
    heapDestroy(psGridHeap);
}
// find the grid's that are covered by the object and either
// add or remove the object
#[no_mangle]
pub unsafe extern "C" fn gridCalcCoverage(mut psObj: *mut BASE_OBJECT,
                                          mut objx: SDWORD, mut objy: SDWORD,
                                          mut mode: COVERAGE_MODE) {
    let mut range: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut minx: SDWORD = 0;
    let mut maxx: SDWORD = 0;
    let mut miny: SDWORD = 0;
    let mut maxy: SDWORD = 0;
    range = gridObjRange(psObj);
    // calculate the range of grids that could be covered by the object
    objx =
        (objx & !(0x7f as libc::c_int)) +
            128 as libc::c_int / 2 as libc::c_int;
    objy =
        (objy & !(0x7f as libc::c_int)) +
            128 as libc::c_int / 2 as libc::c_int;
    minx = objx - range;
    maxx = objx + range;
    miny = objy - range;
    maxy = objy + range;
    minx = (minx >> 7 as libc::c_int) / 8 as libc::c_int;
    maxx = (maxx >> 7 as libc::c_int) / 8 as libc::c_int;
    miny = (miny >> 7 as libc::c_int) / 8 as libc::c_int;
    maxy = (maxy >> 7 as libc::c_int) / 8 as libc::c_int;
    // see which ones are covered by the object
    x = minx;
    while x <= maxx {
        //		if ( (x >= 0) && (x < GRID_WIDTH) )
        if x >= 0 as libc::c_int && (x as libc::c_uint) < gridWidth {
            y = miny;
            while y <= maxy {
                //				if ( (y >= 0) && (y < GRID_HEIGHT) &&
                if y >= 0 as libc::c_int && (y as libc::c_uint) < gridHeight
                       &&
                       gridIntersect(x *
                                         (8 as libc::c_int *
                                              128 as libc::c_int),
                                     y *
                                         (8 as libc::c_int *
                                              128 as libc::c_int),
                                     (x + 1 as libc::c_int) *
                                         (8 as libc::c_int *
                                              128 as libc::c_int),
                                     (y + 1 as libc::c_int) *
                                         (8 as libc::c_int *
                                              128 as libc::c_int), objx, objy,
                                     range) != 0 {
                    match mode as libc::c_uint {
                        0 => { gridAddArrayObject(x, y, psObj); }
                        1 => { gridRemoveArrayObject(x, y, psObj); }
                        _ => { }
                    }
                }
                y += 1
            }
        }
        x += 1
    };
}
// add an object to the grid system
// add an object to the grid system
#[no_mangle]
pub unsafe extern "C" fn gridAddObject(mut psObj: *mut BASE_OBJECT) {
    gridCalcCoverage(psObj, (*psObj).x as SDWORD, (*psObj).y as SDWORD,
                     GRID_ADDOBJECT);
}
// move an object within the grid
// oldX,oldY are the old position of the object in world coords
// move an object within the grid
// oldX,oldY are the old position of the object in world coords
#[no_mangle]
pub unsafe extern "C" fn gridMoveObject(mut psObj: *mut BASE_OBJECT,
                                        mut oldX: SDWORD, mut oldY: SDWORD) {
    if ((*psObj).x as libc::c_int >> 7 as libc::c_int) as libc::c_uint ==
           oldX as UDWORD >> 7 as libc::c_int &&
           ((*psObj).y as libc::c_int >> 7 as libc::c_int) as libc::c_uint ==
               oldY as UDWORD >> 7 as libc::c_int {
        // havn't changed the tile the object is on, don't bother updating
        return
    }
    gridCalcCoverage(psObj, oldX, oldY, GRID_REMOVEOBJECT);
    gridCalcCoverage(psObj, (*psObj).x as SDWORD, (*psObj).y as SDWORD,
                     GRID_ADDOBJECT);
}
// remove an object from the grid system
// remove an object from the grid system
#[no_mangle]
pub unsafe extern "C" fn gridRemoveObject(mut psObj: *mut BASE_OBJECT) {
    gridCalcCoverage(psObj, (*psObj).x as SDWORD, (*psObj).y as SDWORD,
                     GRID_REMOVEOBJECT);
}
// initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
// initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
#[no_mangle]
pub unsafe extern "C" fn gridStartIterate(mut x: SDWORD, mut y: SDWORD) {
    //	ASSERT( (x >= 0) && (x < GRID_WIDTH*GRID_UNITS) &&
//			 (y >= 0) && (y < GRID_WIDTH*GRID_UNITS),
//		"gridStartIterate: coords off grid" );
    if x >= 0 as libc::c_int &&
           (x as libc::c_uint) <
               gridWidth.wrapping_mul((8 as libc::c_int * 128 as libc::c_int)
                                          as libc::c_uint) &&
           y >= 0 as libc::c_int &&
           (y as libc::c_uint) <
               gridHeight.wrapping_mul((8 as libc::c_int * 128 as libc::c_int)
                                           as libc::c_uint) {
    } else {
        debug(LOG_ERROR,
              b"gridStartIterate: coords off grid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x >= 0 as libc::c_int &&
           (x as libc::c_uint) <
               gridWidth.wrapping_mul((8 as libc::c_int * 128 as libc::c_int)
                                          as libc::c_uint) &&
           y >= 0 as libc::c_int &&
           (y as libc::c_uint) <
               gridHeight.wrapping_mul((8 as libc::c_int * 128 as libc::c_int)
                                           as libc::c_uint) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mapgrid.c\x00" as *const u8 as *const libc::c_char,
              265 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"gridStartIterate\x00")).as_ptr(),
              b"(x >= 0) && (x < gridWidth*GRID_UNITS) && (y >= 0) && (y < gridHeight*GRID_UNITS)\x00"
                  as *const u8 as *const libc::c_char);
    };
    x = x / (8 as libc::c_int * 128 as libc::c_int);
    y = y / (8 as libc::c_int * 128 as libc::c_int);
    //	psIterateGrid = apsMapGrid[x][y];
    psIterateGrid =
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize];
    iterateEntry = 0 as libc::c_int;
}
// get the next object that could affect a location,
// should only be called after gridStartIterate
// get the next object that could affect a location,
// should only be called after gridStartIterate
#[no_mangle]
pub unsafe extern "C" fn gridIterate() -> *mut BASE_OBJECT {
    let mut psRet: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    while !psIterateGrid.is_null() {
        if !(*psIterateGrid).apsObjects[iterateEntry as usize].is_null() {
            break ;
        }
        iterateEntry += 1 as libc::c_int;
        if iterateEntry >= 32 as libc::c_int {
            psIterateGrid = (*psIterateGrid).psNext;
            iterateEntry = 0 as libc::c_int
        }
    }
    if !psIterateGrid.is_null() {
        psRet = (*psIterateGrid).apsObjects[iterateEntry as usize];
        iterateEntry += 1 as libc::c_int;
        if iterateEntry >= 32 as libc::c_int {
            psIterateGrid = (*psIterateGrid).psNext;
            iterateEntry = 0 as libc::c_int
        }
    } else { psRet = 0 as *mut BASE_OBJECT }
    return psRet;
}
// compact some of the grid arrays
// compact some of the grid arrays
#[no_mangle]
pub unsafe extern "C" fn gridGarbageCollect() {
    gridCompactArray(garbageX, garbageY);
    garbageX += 1 as libc::c_int;
    //	if (garbageX >= GRID_WIDTH)
    if garbageX as libc::c_uint >= gridWidth {
        garbageX = 0 as libc::c_int;
        garbageY += 1 as libc::c_int;
        //		if (garbageY >= GRID_HEIGHT)
        if garbageY as libc::c_uint >= gridHeight {
            garbageX = 0 as libc::c_int;
            garbageY = 0 as libc::c_int
        }
    };
    //#ifdef DEBUG
}
// add an object to a grid array
#[no_mangle]
pub unsafe extern "C" fn gridAddArrayObject(mut x: SDWORD, mut y: SDWORD,
                                            mut psObj: *mut BASE_OBJECT) {
    let mut psPrev: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psCurr: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psNew: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut i: SDWORD = 0;
    // see if there is an empty slot in the currently allocated array
    psPrev = 0 as *mut GRID_ARRAY;
    //	for (psCurr = apsMapGrid[x][y]; psCurr; psCurr=psCurr->psNext)
    psCurr =
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize];
    while !psCurr.is_null() {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*psCurr).apsObjects[i as usize].is_null() {
                // found an empty slot
                (*psCurr).apsObjects[i as usize] = psObj;
                return
            }
            i += 1
        }
        psPrev = psCurr;
        psCurr = (*psCurr).psNext
    }
    // allocate a new array chunk
    if heapAlloc(psGridHeap,
                 &mut psNew as *mut *mut GRID_ARRAY as *mut libc::c_void as
                     *mut *mut libc::c_void) == 0 {
        debug(LOG_NEVER,
              b"help - %d\n\x00" as *const u8 as *const libc::c_char,
              (*psObj).id);
        return
    }
    // store the object
    memset(psNew as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<GRID_ARRAY>() as libc::c_ulong);
    (*psNew).apsObjects[0 as libc::c_int as usize] = psObj;
    // add the chunk to the end of the list
    if psPrev.is_null() {
        //		apsMapGrid[x][y] = psNew;
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize] = psNew
    } else { (*psPrev).psNext = psNew };
}
// remove an object from a grid array
#[no_mangle]
pub unsafe extern "C" fn gridRemoveArrayObject(mut x: SDWORD, mut y: SDWORD,
                                               mut psObj: *mut BASE_OBJECT) {
    let mut psCurr: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut i: SDWORD = 0;
    //	for (psCurr = apsMapGrid[x][y]; psCurr; psCurr = psCurr->psNext)
    psCurr =
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize];
    while !psCurr.is_null() {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*psCurr).apsObjects[i as usize] == psObj {
                (*psCurr).apsObjects[i as usize] = 0 as *mut BASE_OBJECT;
                return
            }
            i += 1
        }
        psCurr = (*psCurr).psNext
    };
}
// Compact a grid array to remove any NULL objects
#[no_mangle]
pub unsafe extern "C" fn gridCompactArray(mut x: SDWORD, mut y: SDWORD) {
    let mut psDone: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psMove: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psPrev: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut psNext: *mut GRID_ARRAY = 0 as *mut GRID_ARRAY;
    let mut done: SDWORD = 0;
    let mut move_0: SDWORD = 0;
    //	psDone = psMove = apsMapGrid[x][y];
    psMove =
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize];
    psDone = psMove;
    move_0 = 0 as libc::c_int;
    done = move_0;
    // move every entry down in the array
    psPrev = 0 as *mut GRID_ARRAY;
    while !psMove.is_null() {
        if !(*psMove).apsObjects[move_0 as usize].is_null() {
            (*psDone).apsObjects[done as usize] =
                (*psMove).apsObjects[move_0 as usize];
            done += 1 as libc::c_int;
            if done >= 32 as libc::c_int {
                psPrev = psDone;
                psDone = (*psDone).psNext;
                done = 0 as libc::c_int
            }
        }
        move_0 += 1 as libc::c_int;
        if move_0 >= 32 as libc::c_int {
            psMove = (*psMove).psNext;
            move_0 = 0 as libc::c_int
        }
    }
    // if the compacted array finishes half way through, NULL the rest of it
    if !psDone.is_null() && done != 0 as libc::c_int {
        memset(&mut *(*psDone).apsObjects.as_mut_ptr().offset(done as isize)
                   as *mut *mut BASE_OBJECT as *mut libc::c_void,
               0 as libc::c_int,
               (::std::mem::size_of::<*mut BASE_OBJECT>() as
                    libc::c_ulong).wrapping_mul((32 as libc::c_int - done) as
                                                    libc::c_uint));
        psPrev = psDone;
        psDone = (*psDone).psNext
    }
    // now release any unused chunks
    if psPrev.is_null() {
        //		apsMapGrid[x][y] = NULL;
        apsMapGrid[(y as
                        libc::c_uint).wrapping_mul(gridWidth).wrapping_add(x
                                                                               as
                                                                               libc::c_uint)
                       as usize] = 0 as *mut GRID_ARRAY
    } else { (*psPrev).psNext = 0 as *mut _grid_array }
    while !psDone.is_null() {
        psNext = (*psDone).psNext;
        heapFree(psGridHeap, psDone as *mut libc::c_void);
        psDone = psNext
    };
}
// Display all the grid's an object is a member of
// Display all the grid's an object is a member of
#[no_mangle]
pub unsafe extern "C" fn gridDisplayCoverage(mut psObj: *mut BASE_OBJECT) { }
// Function prototypes
// Fast circle rectangle intersection, taken from Graphics Gems I, p51
// by Clifford A Shaffer
/* Return TRUE iff rectangle R intersects circle with centerpoint C and
   radius Rad. */
#[no_mangle]
pub unsafe extern "C" fn gridIntersect(mut x1: SDWORD, mut y1: SDWORD,
                                       mut x2: SDWORD, mut y2: SDWORD,
                                       mut cx: SDWORD, mut cy: SDWORD,
                                       mut Rad: SDWORD) -> BOOL {
    let mut Rad2: SDWORD = 0;
    Rad2 = Rad * Rad;
    /* Translate coordinates, placing C at the origin. */
    x1 -= cx;
    y1 -= cy;
    x2 -= cx;
    y2 -= cy;
    if x2 < 0 as libc::c_int {
        /* R to left of circle center */
        if y2 < 0 as libc::c_int {
            /* R in lower left corner */
            return (x2 * x2 + y2 * y2 < Rad2) as libc::c_int
        } else if y1 > 0 as libc::c_int {
            /* R in upper left corner */
            return (x2 * x2 + y1 * y1 < Rad2) as libc::c_int
        } else {
            /* R due West of circle */
            return (abs(x2) < Rad) as libc::c_int
        }
    } else if x1 > 0 as libc::c_int {
        /* R to right of circle center */
        if y2 < 0 as libc::c_int {
            /* R in lower right corner */
            return (x1 * x1 + y2 * y2 < Rad2) as libc::c_int
        } else if y1 > 0 as libc::c_int {
            /* R in upper right corner */
            return (x1 * x1 + y1 * y1 < Rad2) as libc::c_int
        } else {
            /* R due East of circle */
            return (x1 < Rad) as libc::c_int
        }
    } else if y2 < 0 as libc::c_int {
        /* R on circle vertical centerline */
        /* R due South of circle */
        return (abs(y2) < Rad) as libc::c_int
    } else if y1 > 0 as libc::c_int {
        /* R due North of circle */
        return (y1 < Rad) as libc::c_int
    } else {
        /* R contains circle centerpoint */
        return 1 as libc::c_int
    };
}
// Get the range of effect of an object
#[no_mangle]
pub unsafe extern "C" fn gridObjRange(mut psObj: *mut BASE_OBJECT) -> SDWORD {
    /*	SDWORD	range;

	switch (psObj->type)
	{
	case OBJ_DROID:
		range = ((DROID *)psObj)->sensorRange;
		break;
	case OBJ_STRUCTURE:
		range = ((STRUCTURE *)psObj)->sensorRange;
		if (structCBSensor((STRUCTURE *)psObj) ||
			structVTOLCBSensor((STRUCTURE *)psObj))
		{
			range = MAP_MAXWIDTH > MAP_MAXHEIGHT ?
				MAP_MAXWIDTH*TILE_UNITS : MAP_MAXHEIGHT*TILE_UNITS;
		}
		break;
	case OBJ_FEATURE:
		range = 0;
		break;
	default:
		range = 0;
		break;
	}

	if (range < NAYBOR_RANGE)
	{
		range = NAYBOR_RANGE;
	}

	return range;*/
    return 128 as libc::c_int * 20 as libc::c_int;
}
