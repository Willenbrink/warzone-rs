use ::libc;
extern "C" {
    pub type _viewdata;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    /* Return the ID number for an ID string */
    #[no_mangle]
    fn strresGetIDNum(psRes: *mut STR_RES, pIDStr: *mut STRING,
                      pIDNum: *mut UDWORD) -> BOOL;
    /* Return the stored ID string that matches the string passed in */
    #[no_mangle]
    fn strresGetIDString(psRes: *mut STR_RES, pIDStr: *mut STRING,
                         ppStoredID: *mut *mut STRING) -> BOOL;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /*
 * Function.h
 *
 * Definitions for the Structure Functions.
 *
 */
    //holder for all functions
    #[no_mangle]
    static mut asFunctions: *mut *mut FUNCTION;
    #[no_mangle]
    static mut numFunctions: UDWORD;
    #[no_mangle]
    fn GetGameMode() -> UDWORD;
    /* **************************************************************************/
    #[no_mangle]
    fn audioID_GetIDFromStr(pWavStr: *mut STRING, piID: *mut SDWORD) -> BOOL;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
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
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
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
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
pub type TREAP_NODE = _treap_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
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
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
/* A String Resource */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
pub type STR_RES = _str_res;
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
// The next free ID
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
//ALL components and structures and research topics have a tech level to which they belong
pub type TECH_LEVEL = _tech_level;
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
pub type COMP_BASE_STATS = _comp_base_stats;
pub type _loc = libc::c_uint;
pub const LOC_TURRET: _loc = 1;
pub const LOC_DEFAULT: _loc = 0;
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
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
pub struct _brain_stats {
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
    pub progCap: UDWORD,
    pub psWeaponStat: *mut _weapon_stats,
}
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
/* common stats - N.B. system points for a body
	 * are the limit for all other components
	 */
//UDWORD		configuration;		// Body shape
	//UDWORD		armourMult;			// How configuration affects body shape
									// Actually could be a fractional value
									// store x 10
// How big the body is - affects how hit
//UDWORD		bodyPoints;			// How much damage the body can take before destruction
// The number of weapon slots on the body
// A measure of how much protection the armour provides
// cross-ref with the weapon types
// A measure of how much energy the power plant outputs
//list of IMDs to use for propulsion unit - up to numPropulsionStats
//pointer to which flame graphic to use - for VTOLs only at the moment
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
pub type BRAIN_STATS = _brain_stats;
pub type _prop_side = libc::c_uint;
pub const NUM_PROP_SIDES: _prop_side = 2;
pub const RIGHT_PROP: _prop_side = 1;
pub const LEFT_PROP: _prop_side = 0;
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
pub type PROPULSION_TYPE = _propulsion_type;
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
/* Common stats */
// Program capacity
//UDWORD		AICap;				// AI capacity
	//UDWORD		AISpeed;			// AI Learning Speed
//weapon stats associated with this brain - for Command Droids
// The turret mount to use
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _terrain_table {
    pub speedFactor: UDWORD,
}
pub type TERRAIN_TABLE = _terrain_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _special_ability {
    pub pName: *mut STRING,
}
pub type SPECIAL_ABILITY = _special_ability;
pub type WEAPON_MODIFIER = UWORD;
//sound to play when this prop type shuts down
// factor to multiply the speed by depending on the 
// method of propulsion and the terrain type - to be
								// divided by 100 before use
// Text name of the component
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
pub type RESEARCH = research_stats;
pub type STRUCTURE_STATS = _structure_stats;
pub const NO_SOUND: C2RustUnnamed = -1;
pub const ID_MAX_SOUND: C2RustUnnamed = 354;
pub const TER_MAX: _terrain_type = 12;
pub type _terrain_type = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_int;
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
/*The % to increase the chance to hit in blast radius*/
/*flag to specify the upgrade is valid 
										  for droids (non cyborgs!)*/
// % to increase range by
/* the 2nd IMD for base plates/turrets*/
/* The stores for the different stats */
#[no_mangle]
pub static mut asBodyStats: *mut BODY_STATS =
    0 as *const BODY_STATS as *mut BODY_STATS;
#[no_mangle]
pub static mut asBrainStats: *mut BRAIN_STATS =
    0 as *const BRAIN_STATS as *mut BRAIN_STATS;
//POWER_STATS			*asPowerStats;
#[no_mangle]
pub static mut asPropulsionStats: *mut PROPULSION_STATS =
    0 as *const PROPULSION_STATS as *mut PROPULSION_STATS;
#[no_mangle]
pub static mut asSensorStats: *mut SENSOR_STATS =
    0 as *const SENSOR_STATS as *mut SENSOR_STATS;
#[no_mangle]
pub static mut asECMStats: *mut ECM_STATS =
    0 as *const ECM_STATS as *mut ECM_STATS;
//ARMOUR_STATS		*asArmourStats;
#[no_mangle]
pub static mut asRepairStats: *mut REPAIR_STATS =
    0 as *const REPAIR_STATS as *mut REPAIR_STATS;
//PROGRAM_STATS		*asProgramStats;
#[no_mangle]
pub static mut asWeaponStats: *mut WEAPON_STATS =
    0 as *const WEAPON_STATS as *mut WEAPON_STATS;
#[no_mangle]
pub static mut asConstructStats: *mut CONSTRUCT_STATS =
    0 as *const CONSTRUCT_STATS as *mut CONSTRUCT_STATS;
#[no_mangle]
pub static mut asPropulsionTypes: *mut PROPULSION_TYPES =
    0 as *const PROPULSION_TYPES as *mut PROPULSION_TYPES;
#[no_mangle]
pub static mut asTerrainTable: *mut TERRAIN_TABLE =
    0 as *const TERRAIN_TABLE as *mut TERRAIN_TABLE;
#[no_mangle]
pub static mut asSpecialAbility: *mut SPECIAL_ABILITY =
    0 as *const SPECIAL_ABILITY as *mut SPECIAL_ABILITY;
//used to hold the modifiers cross refd by weapon effect and propulsion type
#[no_mangle]
pub static mut asWeaponModifier: [[WEAPON_MODIFIER; 9]; 6] = [[0; 9]; 6];
//used to hold the current upgrade level per player per weapon subclass
#[no_mangle]
pub static mut asWeaponUpgrade: [[WEAPON_UPGRADE; 17]; 8] =
    [[WEAPON_UPGRADE{firePause: 0,
                     shortHit: 0,
                     longHit: 0,
                     damage: 0,
                     radiusDamage: 0,
                     incenDamage: 0,
                     radiusHit: 0,}; 17]; 8];
#[no_mangle]
pub static mut asSensorUpgrade: [SENSOR_UPGRADE; 8] =
    [SENSOR_UPGRADE{power: 0, range: 0,}; 8];
#[no_mangle]
pub static mut asECMUpgrade: [ECM_UPGRADE; 8] = [ECM_UPGRADE{power: 0,}; 8];
#[no_mangle]
pub static mut asRepairUpgrade: [REPAIR_UPGRADE; 8] =
    [REPAIR_UPGRADE{repairPoints: 0,}; 8];
#[no_mangle]
pub static mut asConstUpgrade: [CONSTRUCTOR_UPGRADE; 8] =
    [CONSTRUCTOR_UPGRADE{constructPoints: 0,}; 8];
#[no_mangle]
pub static mut asBodyUpgrade: [[BODY_UPGRADE; 2]; 8] =
    [[BODY_UPGRADE{powerOutput: 0, body: 0, armourValue: [0; 2],}; 2]; 8];
/* The number of different stats stored */
#[no_mangle]
pub static mut numBodyStats: UDWORD = 0;
#[no_mangle]
pub static mut numBrainStats: UDWORD = 0;
//UDWORD		numPowerStats;
#[no_mangle]
pub static mut numPropulsionStats: UDWORD = 0;
#[no_mangle]
pub static mut numSensorStats: UDWORD = 0;
#[no_mangle]
pub static mut numECMStats: UDWORD = 0;
//UDWORD		numArmourStats;
#[no_mangle]
pub static mut numRepairStats: UDWORD = 0;
#[no_mangle]
pub static mut numProgramStats: UDWORD = 0;
#[no_mangle]
pub static mut numWeaponStats: UDWORD = 0;
#[no_mangle]
pub static mut numConstructStats: UDWORD = 0;
//UDWORD		numPropulsionTypes;
#[no_mangle]
pub static mut numSpecialAbility: UDWORD = 0;
//the max values of the stats used in the design screen
#[no_mangle]
pub static mut maxComponentWeight: UDWORD = 0;
#[no_mangle]
pub static mut maxBodyArmour: UDWORD = 0;
#[no_mangle]
pub static mut maxBodyPower: UDWORD = 0;
#[no_mangle]
pub static mut maxBodyPoints: UDWORD = 0;
#[no_mangle]
pub static mut maxSensorRange: UDWORD = 0;
#[no_mangle]
pub static mut maxSensorPower: UDWORD = 0;
#[no_mangle]
pub static mut maxECMPower: UDWORD = 0;
#[no_mangle]
pub static mut maxConstPoints: UDWORD = 0;
#[no_mangle]
pub static mut maxRepairPoints: UDWORD = 0;
#[no_mangle]
pub static mut maxWeaponRange: UDWORD = 0;
#[no_mangle]
pub static mut maxWeaponDamage: UDWORD = 0;
#[no_mangle]
pub static mut maxPropulsionSpeed: UDWORD = 0;
//stores for each players component states - can be either UNAVAILABLE, FOUND or AVAILABLE
#[no_mangle]
pub static mut apCompLists: [[*mut UBYTE; 9]; 8] =
    [[0 as *const UBYTE as *mut UBYTE; 9]; 8];
//store for each players Structure states
#[no_mangle]
pub static mut apStructTypeLists: [*mut UBYTE; 8] =
    [0 as *const UBYTE as *mut UBYTE; 8];
/* ******************************************************************************
*		Generic stats macros/functions
*******************************************************************************/
/* Macro to allocate memory for a set of stats */
/*Macro to Deallocate stats*/
#[no_mangle]
pub unsafe extern "C" fn statsInitVars() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    asBodyStats = 0 as *mut BODY_STATS;
    asBrainStats = 0 as *mut BRAIN_STATS;
    asPropulsionStats = 0 as *mut PROPULSION_STATS;
    asSensorStats = 0 as *mut SENSOR_STATS;
    asECMStats = 0 as *mut ECM_STATS;
    asRepairStats = 0 as *mut REPAIR_STATS;
    //asProgramStats = NULL;
    asWeaponStats = 0 as *mut WEAPON_STATS;
    asConstructStats = 0 as *mut CONSTRUCT_STATS;
    asPropulsionTypes = 0 as *mut PROPULSION_TYPES;
    asTerrainTable = 0 as *mut TERRAIN_TABLE;
    asSpecialAbility = 0 as *mut SPECIAL_ABILITY;
    /* The number of different stats stored */
    numBodyStats = 0 as libc::c_int as UDWORD;
    numBrainStats = 0 as libc::c_int as UDWORD;
    numPropulsionStats = 0 as libc::c_int as UDWORD;
    numSensorStats = 0 as libc::c_int as UDWORD;
    numECMStats = 0 as libc::c_int as UDWORD;
    numRepairStats = 0 as libc::c_int as UDWORD;
    //numProgramStats = 0;
    numWeaponStats = 0 as libc::c_int as UDWORD;
    numConstructStats = 0 as libc::c_int as UDWORD;
    //	numPropulsionTypes = 0;
    numSpecialAbility = 0 as libc::c_int as UDWORD;
    //stores for each players component states - can be either UNAVAILABLE, FOUND or AVAILABLE
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < COMP_NUMCOMPONENTS as libc::c_int {
            apCompLists[i as usize][j as usize] = 0 as *mut UBYTE;
            j += 1
        }
        i += 1
    }
    //store for each players Structure states
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        apStructTypeLists[i as usize] = 0 as *mut UBYTE;
        i += 1
    }
    //initialise the upgrade structures
    memset(asWeaponUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ((8 as libc::c_int * NUM_WEAPON_SUBCLASS as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<WEAPON_UPGRADE>()
                                               as libc::c_ulong));
    memset(asSensorUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<SENSOR_UPGRADE>()
                                               as libc::c_ulong));
    memset(asECMUpgrade.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<ECM_UPGRADE>()
                                               as libc::c_ulong));
    memset(asRepairUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<REPAIR_UPGRADE>()
                                               as libc::c_ulong));
    memset(asBodyUpgrade.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<BODY_UPGRADE>()
                                               as
                                               libc::c_ulong).wrapping_mul(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint));
    //init the max values
    maxPropulsionSpeed = 0 as libc::c_int as UDWORD;
    maxWeaponDamage = maxPropulsionSpeed;
    maxWeaponRange = maxWeaponDamage;
    maxRepairPoints = maxWeaponRange;
    maxConstPoints = maxRepairPoints;
    maxECMPower = maxConstPoints;
    maxSensorPower = maxECMPower;
    maxSensorRange = maxSensorPower;
    maxBodyPoints = maxSensorRange;
    maxBodyPower = maxBodyPoints;
    maxBodyArmour = maxBodyPower;
    maxComponentWeight = maxBodyArmour;
}
/*Deallocate all the stats assigned from input data*/
#[no_mangle]
pub unsafe extern "C" fn statsDealloc(mut pStats: *mut COMP_BASE_STATS,
                                      mut listSize: UDWORD,
                                      mut structureSize: UDWORD) {
    //#ifndef RESOURCE_NAMES
    memFreeRelease(pStats as *mut libc::c_void);
    pStats = 0 as *mut COMP_BASE_STATS;
}
unsafe extern "C" fn allocateStatName(mut pStat: *mut BASE_STATS,
                                      mut Name: *mut libc::c_char) -> BOOL {
    return allocateName(&mut (*pStat).pName, Name);
}
/* body stats need the extra list removing */
#[no_mangle]
pub unsafe extern "C" fn deallocBodyStats() {
    let mut psStat: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numBodyStats {
        psStat = &mut *asBodyStats.offset(inc as isize) as *mut BODY_STATS;
        //#ifndef RESOURCE_NAMES
        memFreeRelease((*psStat).ppIMDList as *mut libc::c_void);
        (*psStat).ppIMDList = 0 as *mut *mut iIMDShape;
        inc = inc.wrapping_add(1)
    }
    memFreeRelease(asBodyStats as *mut libc::c_void);
    asBodyStats = 0 as *mut BODY_STATS;
}
/*Deallocate all the stats assigned from input data*/
#[no_mangle]
pub unsafe extern "C" fn statsShutDown() -> BOOL {
    //statsDeallocWeapons();
    statsDealloc(asWeaponStats as *mut COMP_BASE_STATS, numWeaponStats,
                 ::std::mem::size_of::<WEAPON_STATS>() as libc::c_ulong);
    //STATS_DEALLOC(asArmourStats, numArmourStats, ARMOUR_STATS);
	//STATS_DEALLOC(asBodyStats, numBodyStats, BODY_STATS);
    deallocBodyStats();
    statsDealloc(asBrainStats as *mut COMP_BASE_STATS, numBrainStats,
                 ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong);
    //STATS_DEALLOC(asPowerStats, numPowerStats, POWER_STATS);
    statsDealloc(asPropulsionStats as *mut COMP_BASE_STATS,
                 numPropulsionStats,
                 ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong);
    statsDealloc(asSensorStats as *mut COMP_BASE_STATS, numSensorStats,
                 ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong);
    statsDealloc(asECMStats as *mut COMP_BASE_STATS, numECMStats,
                 ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong);
    statsDealloc(asRepairStats as *mut COMP_BASE_STATS, numRepairStats,
                 ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong);
    //	STATS_DEALLOC(asProgramStats, numProgramStats, PROGRAM_STATS);
    statsDealloc(asConstructStats as *mut COMP_BASE_STATS, numConstructStats,
                 ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong);
    deallocPropulsionTypes();
    deallocTerrainTable();
    deallocSpecialAbility();
    return 1 as libc::c_int;
}
/* Macro to set the stats for a particular ref
 * The macro uses the ref number in the stats structure to
 * index the correct array entry
 */
/* Return the number of newlines in a file buffer */
#[no_mangle]
pub unsafe extern "C" fn numCR(mut pFileBuffer: *mut libc::c_char,
                               mut fileSize: UDWORD) -> UDWORD {
    let mut lines: UDWORD = 0 as libc::c_int as UDWORD; //, filePos=0;
    loop  {
        let fresh0 = fileSize;
        fileSize = fileSize.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as libc::c_uint) { break ; }
        let fresh1 = pFileBuffer;
        pFileBuffer = pFileBuffer.offset(1);
        if *fresh1 as libc::c_int == '\n' as i32 {
            lines = lines.wrapping_add(1)
        }
    }
    return lines;
}
/*Load the stats from the Access database*/
//BOOL loadStats(void)
//{
	/*if (!loadWeaponStats())
	{
		DBERROR(("Unable to load weapon stats"));
		return FALSE;
	}*/
	/*if (!loadBodyStats())
	{
		DBERROR(("Unable to load body stats"));
		return FALSE;
	}*/
	/*if (!loadBrainStats())
	{
		DBERROR(("Unable to load brain stats"));
		return FALSE;
	}*/
	/*if (!loadPropulsionStats())
	{
		DBERROR(("Unable to load propulsion stats"));
		return FALSE;
	}*/
	/*if (!loadSensorStats())
	{
		DBERROR(("Unable to load sensor stats"));
		return FALSE;
	}*/
	/*if (!loadECMStats())
	{
		DBERROR(("Unable to load ecm stats"));
		return FALSE;
	}*/
	/*if (!loadRepairStats())
	{
		DBERROR(("Unable to load repair stats"));
		return FALSE;
	}*/
	/*if (!loadProgramStats())
	{
		DBERROR(("Unable to load program stats"));
		return FALSE;
	}*/
	/*if(!loadConstructStats())
	{
		DBERROR(("Unable to load construct stats"));
		return FALSE;
	}*/
	/*if (!loadPropulsionTypes())
	{
		DBERROR(("Unable to load propulsion types"));
		return FALSE;
	}*/
	/*if (!loadTerrainTable())
	{
		DBERROR(("Unable to load terrain table"));
		return FALSE;
	}*/
	/*if (!loadSpecialAbility())
	{
		DBERROR(("Unable to load special ability stats"));
		return FALSE;
	}*/
/*if (!loadFeatureStats())
	{
		DBERROR(("Unable to load feature stats"));
		return FALSE;
	}*/
//	return TRUE;
//}
/* ******************************************************************************
*		Allocate stats functions
*******************************************************************************/
/* Allocate Weapon stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocWeapons(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for WEAPON_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              401 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"statsAllocWeapons\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asWeaponStats =
        memMallocRelease((::std::mem::size_of::<WEAPON_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut WEAPON_STATS;
    if asWeaponStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numWeaponStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Armour Stats */
/*BOOL statsAllocArmour(UDWORD	numStats)
{
	ALLOC_STATS(numStats, asArmourStats, numArmourStats, ARMOUR_STATS);
}*/
/* Allocate Body Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocBody(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for BODY_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              411 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsAllocBody\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asBodyStats =
        memMallocRelease((::std::mem::size_of::<BODY_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut BODY_STATS;
    if asBodyStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numBodyStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Brain Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocBrain(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for BRAIN_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              416 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"statsAllocBrain\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asBrainStats =
        memMallocRelease((::std::mem::size_of::<BRAIN_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut BRAIN_STATS;
    if asBrainStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numBrainStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Power Stats */
/*BOOL statsAllocPower(UDWORD	numStats)
{
	ALLOC_STATS(numStats, asPowerStats, numPowerStats, POWER_STATS);
}*/
/* Allocate Propulsion Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocPropulsion(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for PROPULSION_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              426 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"statsAllocPropulsion\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asPropulsionStats =
        memMallocRelease((::std::mem::size_of::<PROPULSION_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut PROPULSION_STATS;
    if asPropulsionStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numPropulsionStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Sensor Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocSensor(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for SENSOR_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              431 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"statsAllocSensor\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asSensorStats =
        memMallocRelease((::std::mem::size_of::<SENSOR_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut SENSOR_STATS;
    if asSensorStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numSensorStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Ecm Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocECM(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for ECM_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              436 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"statsAllocECM\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asECMStats =
        memMallocRelease((::std::mem::size_of::<ECM_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut ECM_STATS;
    if asECMStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numECMStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Repair Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocRepair(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for REPAIR_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              442 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"statsAllocRepair\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asRepairStats =
        memMallocRelease((::std::mem::size_of::<REPAIR_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut REPAIR_STATS;
    if asRepairStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numRepairStats = numStats;
    return 1 as libc::c_int;
}
/* Allocate Program Stats */
/*BOOL statsAllocProgram(UDWORD	numStats)
{
	ALLOC_STATS(numStats, asProgramStats, numProgramStats, PROGRAM_STATS);
}*/
/* Allocate Construct Stats */
#[no_mangle]
pub unsafe extern "C" fn statsAllocConstruct(mut numStats: UDWORD) -> BOOL {
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"allocStats: number of stats entries too large for CONSTRUCT_STATS\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numStats < 0x10000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              454 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"statsAllocConstruct\x00")).as_ptr(),
              b"(numStats) < REF_RANGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    asConstructStats =
        memMallocRelease((::std::mem::size_of::<CONSTRUCT_STATS>() as
                              libc::c_ulong).wrapping_mul(numStats)) as
            *mut CONSTRUCT_STATS;
    if asConstructStats.is_null() {
        debug(LOG_ERROR,
              b"Out of memory\x00" as *const u8 as *const libc::c_char);
        abort();
    }
    numConstructStats = numStats;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getStatName(mut Stat: *mut libc::c_void)
 -> *mut STRING {
    let mut psStats: *mut BASE_STATS = Stat as *mut BASE_STATS;
    return getName((*psStats).pName);
}
/* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/*Load the weapon stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadWeaponStats(mut pWeaponData: *mut libc::c_char,
                                         mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut sStats: WEAPON_STATS =
        WEAPON_STATS{ref_0: 0,
                     pName: 0 as *mut STRING,
                     techLevel: TECH_LEVEL_ONE,
                     buildPower: 0,
                     buildPoints: 0,
                     weight: 0,
                     hitPoints: 0,
                     systemPoints: 0,
                     body: 0,
                     design: 0,
                     pIMD: 0 as *mut iIMDShape,
                     shortRange: 0,
                     longRange: 0,
                     minRange: 0,
                     shortHit: 0,
                     longHit: 0,
                     firePause: 0,
                     numExplosions: 0,
                     numRounds: 0,
                     reloadTime: 0,
                     damage: 0,
                     radius: 0,
                     radiusHit: 0,
                     radiusDamage: 0,
                     incenTime: 0,
                     incenDamage: 0,
                     incenRadius: 0,
                     flightSpeed: 0,
                     indirectHeight: 0,
                     fireOnMove: FOM_NO,
                     weaponClass: WC_KINETIC,
                     weaponSubClass: WSC_MGUN,
                     movementModel: MM_DIRECT,
                     weaponEffect: WE_ANTI_PERSONNEL,
                     recoilValue: 0,
                     rotate: 0,
                     maxElevation: 0,
                     minElevation: 0,
                     facePlayer: 0,
                     faceInFlight: 0,
                     effectSize: 0,
                     lightWorld: 0,
                     surfaceToAir: 0,
                     vtolAttackRuns: 0,
                     directLife: 0,
                     radiusLife: 0,
                     pMountGraphic: 0 as *mut iIMDShape,
                     pMuzzleGraphic: 0 as *mut iIMDShape,
                     pInFlightGraphic: 0 as *mut iIMDShape,
                     pTargetHitGraphic: 0 as *mut iIMDShape,
                     pTargetMissGraphic: 0 as *mut iIMDShape,
                     pWaterHitGraphic: 0 as *mut iIMDShape,
                     pTrailGraphic: 0 as *mut iIMDShape,
                     iAudioFireID: 0,
                     iAudioImpactID: 0,};
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut psStartStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut NumWeapons: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut rotate: UDWORD = 0;
    let mut maxElevation: UDWORD = 0;
    let mut surfaceToAir: UDWORD = 0;
    let mut minElevation: SDWORD = 0;
    let mut WeaponName: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut mountGfx: [STRING; 60] = [0; 60];
    let mut flightGfx: [STRING; 60] = [0; 60];
    let mut hitGfx: [STRING; 60] = [0; 60];
    let mut missGfx: [STRING; 60] = [0; 60];
    let mut waterGfx: [STRING; 60] = [0; 60];
    let mut muzzleGfx: [STRING; 60] = [0; 60];
    let mut trailGfx: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut fireOnMove: [STRING; 10] = [0; 10];
    let mut weaponClass: [STRING; 15] = [0; 15];
    let mut weaponSubClass: [STRING; 15] = [0; 15];
    let mut weaponEffect: [STRING; 16] = [0; 16];
    let mut movement: [STRING; 15] = [0; 15];
    let mut facePlayer: [STRING; 5] = [0; 5];
    let mut faceInFlight: [STRING; 5] = [0; 5];
    let mut lightWorld: [STRING; 5] = [0; 5];
    let mut longRange: UDWORD = 0;
    let mut effectSize: UDWORD = 0;
    let mut numAttackRuns: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut numRounds: UDWORD = 0;
    let mut StatsName: *mut libc::c_char = 0 as *mut libc::c_char;
    //keep the start so we release it at the end
	//pData = pWeaponData;
    psStats = &mut sStats;
    /*	psStats = (WEAPON_STATS *)MALLOC(sizeof(WEAPON_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Weapon Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumWeapons = numCR(pWeaponData, bufferSize);
    if statsAllocWeapons(NumWeapons) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumWeapons {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<WEAPON_STATS>() as libc::c_ulong);
        WeaponName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        mountGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        muzzleGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        flightGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        hitGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        missGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        waterGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        trailGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        fireOnMove[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        weaponClass[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        weaponSubClass[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        movement[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        weaponEffect[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        facePlayer[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        faceInFlight[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pWeaponData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%d,\t\t\t%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%d,%d,%d,%[^\',\'],%[^\',\'],%d,%d,\t\t\t%[^\',\'],%d,%d,%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut WeaponName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut mountGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut muzzleGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut flightGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut hitGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut missGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut waterGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut trailGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).shortRange as *mut UDWORD,
               &mut (*psStats).longRange as *mut UDWORD,
               &mut (*psStats).shortHit as *mut UDWORD,
               &mut (*psStats).longHit as *mut UDWORD,
               &mut (*psStats).firePause as *mut UDWORD,
               &mut (*psStats).numExplosions as *mut UDWORD,
               &mut numRounds as *mut UDWORD,
               &mut (*psStats).reloadTime as *mut UDWORD,
               &mut (*psStats).damage as *mut UDWORD,
               &mut (*psStats).radius as *mut UDWORD,
               &mut (*psStats).radiusHit as *mut UDWORD,
               &mut (*psStats).radiusDamage as *mut UDWORD,
               &mut (*psStats).incenTime as *mut UDWORD,
               &mut (*psStats).incenDamage as *mut UDWORD,
               &mut (*psStats).incenRadius as *mut UDWORD,
               &mut (*psStats).directLife as *mut UDWORD,
               &mut (*psStats).radiusLife as *mut UDWORD,
               &mut (*psStats).flightSpeed as *mut UDWORD,
               &mut (*psStats).indirectHeight as *mut UDWORD,
               &mut fireOnMove as *mut [STRING; 10] as *mut libc::c_char,
               &mut weaponClass as *mut [STRING; 15] as *mut libc::c_char,
               &mut weaponSubClass as *mut [STRING; 15] as *mut libc::c_char,
               &mut movement as *mut [STRING; 15] as *mut libc::c_char,
               &mut weaponEffect as *mut [STRING; 16] as *mut libc::c_char,
               &mut rotate as *mut UDWORD, &mut maxElevation as *mut UDWORD,
               &mut minElevation as *mut SDWORD,
               &mut facePlayer as *mut [STRING; 5] as *mut libc::c_char,
               &mut faceInFlight as *mut [STRING; 5] as *mut libc::c_char,
               &mut (*psStats).recoilValue as *mut UDWORD,
               &mut (*psStats).minRange as *mut UDWORD,
               &mut lightWorld as *mut [STRING; 5] as *mut libc::c_char,
               &mut effectSize as *mut UDWORD,
               &mut surfaceToAir as *mut UDWORD,
               &mut numAttackRuns as *mut UDWORD,
               &mut designable as *mut UDWORD);
        (*psStats).numRounds = numRounds as UBYTE;
        //#ifdef DEBUG
// Hack to get the current stats working... a zero flight speed value will cause an assert in projectile.c line 957
//  I'm not sure if this should be on debug only...
//    ... the last thing we want is for a zero value to get through on release (with no asserts!)
//
// Anyway if anyone has a problem with this, take it up with Tim ... we have a frank and open discussion about it.
        if (*psStats).flightSpeed == 0 as libc::c_int as libc::c_uint {
            debug(LOG_NEVER,
                  b"STATS: Zero Flightspeed for %s - using default of %d\n\x00"
                      as *const u8 as *const libc::c_char,
                  WeaponName.as_mut_ptr(), 500 as libc::c_int);
            (*psStats).flightSpeed = 500 as libc::c_int as UDWORD
        }
        if allocateStatName(psStats as *mut BASE_STATS,
                            WeaponName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0xa0000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //multiply time stats
        (*psStats).firePause =
            ((*psStats).firePause as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        (*psStats).incenTime =
            ((*psStats).incenTime as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        (*psStats).directLife =
            ((*psStats).directLife as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        (*psStats).radiusLife =
            ((*psStats).radiusLife as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        (*psStats).reloadTime =
            ((*psStats).reloadTime as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        //get the IMD for the component
        if strcmp(GfxFile.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the weapon PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        //get the rest of the imd's
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pMountGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, mountGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMountGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the mount PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pMountGraphic = 0 as *mut iIMDShape }
        if GetGameMode() == 3 as libc::c_int as libc::c_uint {
            (*psStats).pMuzzleGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, muzzleGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMuzzleGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the muzzle PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
            (*psStats).pInFlightGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, flightGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pInFlightGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the flight PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
            (*psStats).pTargetHitGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, hitGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pTargetHitGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the target hit PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
            (*psStats).pTargetMissGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, missGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pTargetMissGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the target miss PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
            (*psStats).pWaterHitGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, waterGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pWaterHitGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the water hit PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
            //trail graphic can be null
            if strcmp(trailGfx.as_mut_ptr(),
                      b"0\x00" as *const u8 as *const libc::c_char) != 0 {
                (*psStats).pTrailGraphic =
                    resGetData(b"IMD\x00" as *const u8 as *const libc::c_char
                                   as *mut STRING, trailGfx.as_mut_ptr()) as
                        *mut iIMDShape;
                if (*psStats).pTrailGraphic.is_null() {
                    debug(LOG_ERROR,
                          b"Cannot find the trail PIE for record %s\x00" as
                              *const u8 as *const libc::c_char,
                          getStatName(psStats as *mut libc::c_void));
                    abort();
                }
            } else { (*psStats).pTrailGraphic = 0 as *mut iIMDShape }
        }
        //set the fireOnMove
        if strcmp(fireOnMove.as_mut_ptr(),
                  b"NO\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).fireOnMove = FOM_NO
        } else if strcmp(fireOnMove.as_mut_ptr(),
                         b"PARTIAL\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            (*psStats).fireOnMove = FOM_PARTIAL
        } else if strcmp(fireOnMove.as_mut_ptr(),
                         b"YES\x00" as *const u8 as *const libc::c_char) == 0
         {
            (*psStats).fireOnMove = FOM_YES
        } else {
            debug(LOG_ERROR,
                  b"Invalid fire on move flag for weapon %s\x00" as *const u8
                      as *const libc::c_char,
                  getStatName(psStats as *mut libc::c_void));
            abort();
        }
        //set the weapon class
        if strcmp(weaponClass.as_mut_ptr(),
                  b"KINETIC\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).weaponClass = WC_KINETIC
        } else if strcmp(weaponClass.as_mut_ptr(),
                         b"EXPLOSIVE\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            //psStats->weaponClass = WC_EXPLOSIVE;
            (*psStats).weaponClass = WC_KINETIC
        } else if strcmp(weaponClass.as_mut_ptr(),
                         b"HEAT\x00" as *const u8 as *const libc::c_char) == 0
         {
            (*psStats).weaponClass = WC_HEAT
        } else if strcmp(weaponClass.as_mut_ptr(),
                         b"MISC\x00" as *const u8 as *const libc::c_char) == 0
         {
            //psStats->weaponClass = WC_MISC;
            (*psStats).weaponClass = WC_HEAT
        } else {
            debug(LOG_ERROR,
                  b"Invalid weapon class for weapon %s\x00" as *const u8 as
                      *const libc::c_char,
                  getStatName(psStats as *mut libc::c_void));
            abort();
        }
        //set the subClass
        (*psStats).weaponSubClass =
            getWeaponSubClass(weaponSubClass.as_mut_ptr()) as WEAPON_SUBCLASS;
        if (*psStats).weaponSubClass as libc::c_uint ==
               (NUM_WEAPON_SUBCLASS as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            return 0 as libc::c_int
        }
        //set the movement model
        (*psStats).movementModel =
            getMovementModel(movement.as_mut_ptr()) as MOVEMENT_MODEL;
        if (*psStats).movementModel as libc::c_uint ==
               (NUM_MOVEMENT_MODEL as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            return 0 as libc::c_int
        }
        //set the weapon effect
        (*psStats).weaponEffect =
            getWeaponEffect(weaponEffect.as_mut_ptr()) as WEAPON_EFFECT;
        if (*psStats).weaponEffect as libc::c_uint ==
               (WE_NUMEFFECTS as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadWepaonStats: Invalid weapon effect for weapon %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(psStats as *mut libc::c_void));
            abort();
        }
        StatsName = (*psStats).pName;
        //covered by the movement model now - AB 15/06/98
		//set the homing round
		/*if (compareYes(homingRound, StatsName))
		{
			psStats->homingRound = TRUE;
		}
		else
		{
			psStats->homingRound = FALSE;
		}*/
        //set the face Player value
        if compareYes(facePlayer.as_mut_ptr(), StatsName) != 0 {
            (*psStats).facePlayer = 1 as libc::c_int as UBYTE
        } else { (*psStats).facePlayer = 0 as libc::c_int as UBYTE }
        //set the In flight face Player value
        if compareYes(faceInFlight.as_mut_ptr(), StatsName) != 0 {
            (*psStats).faceInFlight = 1 as libc::c_int as UBYTE
        } else { (*psStats).faceInFlight = 0 as libc::c_int as UBYTE }
        //set the light world value
        if compareYes(lightWorld.as_mut_ptr(), StatsName) != 0 {
            (*psStats).lightWorld = 1 as libc::c_int
        } else { (*psStats).lightWorld = 0 as libc::c_int }
        //set the effect size
        if effectSize > 0xff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: effectSize is greater than 255 for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      805 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        (*psStats).effectSize = effectSize as UBYTE;
        //set the rotate angle
        if rotate > 0xff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: rotate is greater than 255 for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      814 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        (*psStats).rotate = rotate as UBYTE;
        //set the minElevation
        if minElevation > 0x7f as libc::c_int ||
               minElevation < -(128 as libc::c_int) {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: minElevation is outside of limits for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      823 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        (*psStats).minElevation = minElevation as SBYTE;
        //set the maxElevation
        if maxElevation > 0xff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: maxElevation is outside of limits for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      832 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        (*psStats).maxElevation = maxElevation as UBYTE;
        //set the surfaceAir
        if surfaceToAir > 0xff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: Surface to Air is outside of limits for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      841 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        if surfaceToAir == 0 as libc::c_int as libc::c_uint {
            (*psStats).surfaceToAir = 0x1 as libc::c_int as UBYTE
        } else if surfaceToAir <= 50 as libc::c_int as libc::c_uint {
            (*psStats).surfaceToAir = 0x2 as libc::c_int as UBYTE
        } else {
            (*psStats).surfaceToAir =
                (0x1 as libc::c_int | 0x2 as libc::c_int) as UBYTE
        }
        //set the attackRuns for VTOLs
        if numAttackRuns > 0xff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadWeaponStats: num of attack runs is outside of limits for weapon %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      861 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadWeaponStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        (*psStats).vtolAttackRuns = numAttackRuns as UBYTE;
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        // error check the ranges
        if (*psStats).flightSpeed > 0 as libc::c_int as libc::c_uint &&
               proj_Direct(psStats) == 0 {
            longRange = proj_GetLongRange(psStats, 0 as libc::c_int) as UDWORD
        } else { longRange = 0xffffffff as libc::c_uint }
        if (*psStats).shortRange > longRange {
            debug(LOG_NEVER,
                  b"%s, flight speed is too low to reach short range (max range %d)\n\x00"
                      as *const u8 as *const libc::c_char,
                  WeaponName.as_mut_ptr(), longRange);
        } else if (*psStats).longRange > longRange {
            debug(LOG_NEVER,
                  b"%s, flight speed is too low to reach long range (max range %d)\n\x00"
                      as *const u8 as *const libc::c_char,
                  WeaponName.as_mut_ptr(), longRange);
        }
        //set the weapon sounds to default value
        (*psStats).iAudioFireID = NO_SOUND as libc::c_int;
        (*psStats).iAudioImpactID = NO_SOUND as libc::c_int;
        //save the stats
        statsSetWeapon(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxWeaponRange((*psStats).longRange);
            setMaxWeaponDamage((*psStats).damage);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pWeaponData =
            strchr(pWeaponData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Armour stats from the file exported from Access*/
/*BOOL loadArmourStats(void)
{
	char *pArmourData, *pStartArmourData;
	UDWORD fileSize;
	ARMOUR_STATS	*psStats, *psStartStats;
	UDWORD	NumArmour = 0,i;
	STRING  ArmourName[50];
	BOOL EndOfFile;


	if (!loadFile("Armour.txt", &pArmourData, &fileSize))
	{
		return FALSE;
	}
	pStartArmourData = pArmourData;

	psStats = (ARMOUR_STATS *)MALLOC(sizeof(ARMOUR_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Armour Stats - Out of memory"));
		return FALSE;
	}
	//reserve the start of the data
	psStartStats = psStats;

	EndOfFile = FALSE;
	//determine the number of records to add by counting the number of '\n'
	while (!EndOfFile)
	{
		pArmourData = strchr(pArmourData,'\n');
		if (pArmourData == NULL)
		{
			EndOfFile = TRUE;
		}
		else
		{
			pArmourData++;
			NumArmour++;
		}
	}
	//return to the start of the data
	pArmourData = pStartArmourData;

	if (!statsAllocArmour(NumArmour))
	{
		return FALSE;
	}

	for (i=0; i < NumArmour; i++)
	{
		memset(psStats, 0, sizeof(ARMOUR_STATS));

		//read the data into the storage - the data is delimeted using comma's
		sscanf(pArmourData,"%[^','],%d,%d,%d,%d,%d,%d",
			&ArmourName, &psStats->buildPower,&psStats->buildPoints,
			&psStats->weight, &psStats->hitPoints, &psStats->systemPoints,
			&psStats->strength);

		//allocate storage for the name
		psStats->pName = (STRING *)MALLOC((strlen(ArmourName))+1);
		if (psStats->pName == NULL)
		{
			DBERROR(("Armour Stats Name - Out of memory"));
			return FALSE;
		}
		strcpy(psStats->pName,ArmourName);

		psStats->ref = REF_ARMOUR_START + i;

		//save the stats
		statsSetArmour(psStats, i);

		psStats = psStartStats;
		//increment the pointer to the start of the next record
		pArmourData = strchr(pArmourData,'\n') + 1;
	}
	FREE(pStartArmourData);
	FREE(psStats);
	return TRUE;
}
*/
/*Load the Body stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadBodyStats(mut pBodyData: *mut libc::c_char,
                                       mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut sStats: BODY_STATS =
        BODY_STATS{ref_0: 0,
                   pName: 0 as *mut STRING,
                   techLevel: TECH_LEVEL_ONE,
                   buildPower: 0,
                   buildPoints: 0,
                   weight: 0,
                   hitPoints: 0,
                   systemPoints: 0,
                   body: 0,
                   design: 0,
                   pIMD: 0 as *mut iIMDShape,
                   size: 0,
                   weaponSlots: 0,
                   armourValue: [0; 2],
                   powerOutput: 0,
                   ppIMDList: 0 as *mut *mut iIMDShape,
                   pFlameIMD: 0 as *mut iIMDShape,};
    let mut psStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut psStartStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut NumBody: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut BodyName: [STRING; 60] = [0; 60];
    let mut size: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut flameIMD: [STRING; 60] = [0; 60];
    //keep the start so we can release it at the end
	//pData = pBodyData;
    psStats = &mut sStats;
    /*	psStats = (BODY_STATS *)MALLOC(sizeof(BODY_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Body Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumBody = numCR(pBodyData, bufferSize);
    if statsAllocBody(NumBody) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumBody {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<BODY_STATS>() as libc::c_ulong);
        BodyName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        size[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        flameIMD[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pBodyData,
               b"%[^\',\'],%[^\',\'],%[^\',\'],%d,%d,%d,%d,%[^\',\'],\t\t\t%d,%d,%d,%d,%d,%[^\',\'],%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut BodyName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut size as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).weaponSlots as *mut UDWORD,
               &mut (*psStats).powerOutput as *mut UDWORD,
               &mut *(*psStats).armourValue.as_mut_ptr().offset(WC_KINETIC as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut UDWORD,
               &mut *(*psStats).armourValue.as_mut_ptr().offset(WC_HEAT as
                                                                    libc::c_int
                                                                    as isize)
                   as *mut UDWORD,
               &mut flameIMD as *mut [STRING; 60] as *mut libc::c_char,
               &mut designable as
                   *mut UDWORD); //, &psStats->armourValue[WC_EXPLOSIVE],
        //&psStats->armourValue[WC_MISC]);
        //allocate storage for the name
		/*psStats->pName = (STRING *)MALLOC((strlen(BodyName))+1);
		if (psStats->pName == NULL)
		{
			DBERROR(("Body Stats Name - Out of memory"));
			return FALSE;
		}
		strcpy(psStats->pName,BodyName);*/
        if allocateStatName(psStats as *mut BASE_STATS, BodyName.as_mut_ptr())
               == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x10000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        if getBodySize(size.as_mut_ptr(), &mut (*psStats).size) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadBodyStats: unknown body size for %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1080 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"loadBodyStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //get the IMD for the component
        if strcmp(GfxFile.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the body PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        //get the flame graphic
        if strcmp(flameIMD.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pFlameIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, flameIMD.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pFlameIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the flame PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pFlameIMD = 0 as *mut iIMDShape }
        //save the stats
        statsSetBody(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxBodyArmour((*psStats).armourValue[WC_KINETIC as libc::c_int
                                                        as usize]);
            setMaxBodyArmour((*psStats).armourValue[WC_HEAT as libc::c_int as
                                                        usize]);
            setMaxBodyPower((*psStats).powerOutput);
            setMaxBodyPoints((*psStats).body);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pBodyData =
            strchr(pBodyData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Brain stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadBrainStats(mut pBrainData: *mut libc::c_char,
                                        mut bufferSize: UDWORD) -> BOOL {
    //SBYTE		*pData;
    let mut sStats: BRAIN_STATS =
        BRAIN_STATS{ref_0: 0,
                    pName: 0 as *mut STRING,
                    techLevel: TECH_LEVEL_ONE,
                    buildPower: 0,
                    buildPoints: 0,
                    weight: 0,
                    hitPoints: 0,
                    systemPoints: 0,
                    body: 0,
                    design: 0,
                    pIMD: 0 as *mut iIMDShape,
                    progCap: 0,
                    psWeaponStat: 0 as *mut _weapon_stats,};
    let mut psStats: *mut BRAIN_STATS = 0 as *mut BRAIN_STATS;
    let mut psStartStats: *mut BRAIN_STATS = 0 as *mut BRAIN_STATS;
    let mut NumBrain: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut incW: UDWORD = 0;
    let mut BrainName: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut weaponName: [STRING; 60] = [0; 60];
    //keep the start so we can release it at the end
	//pData = pBrainData;
    psStats = &mut sStats;
    /*	psStats = (BRAIN_STATS *)MALLOC(sizeof(BRAIN_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Brain Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumBrain = numCR(pBrainData, bufferSize);
    if statsAllocBrain(NumBrain) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumBrain {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong);
        BrainName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        weaponName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pBrainData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%[^\',\'],%d\x00" as
                   *const u8 as *const libc::c_char,
               &mut BrainName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut weaponName as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).progCap as
                   *mut UDWORD); //, &psStats->AICap, &psStats->AISpeed);
        if allocateStatName(psStats as *mut BASE_STATS,
                            BrainName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x20000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //check weapon attached
        if strcmp(weaponName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).psWeaponStat = 0 as *mut _weapon_stats
        } else {
            //get the weapon stat
            if getResourceName(weaponName.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            incW =
                getCompFromName(COMP_WEAPON as libc::c_int as UDWORD,
                                weaponName.as_mut_ptr()) as UDWORD;
            //if weapon not found - error
            if incW == -(1 as libc::c_int) as libc::c_uint {
                debug(LOG_ERROR,
                      b"Unable to find Weapon %s for brain %s\x00" as
                          *const u8 as *const libc::c_char,
                      weaponName.as_mut_ptr(), BrainName.as_mut_ptr());
                abort();
            } else {
                //Weapon found, alloc this to the brain
                (*psStats).psWeaponStat = asWeaponStats.offset(incW as isize)
            }
        }
        // All brains except ZNULLBRAIN available in design screen
        if strcmp(BrainName.as_mut_ptr(),
                  b"ZNULLBRAIN\x00" as *const u8 as *const libc::c_char) ==
               0 as libc::c_int {
            (*psStats).design = 0 as libc::c_int
        } else { (*psStats).design = 1 as libc::c_int }
        //save the stats
        statsSetBrain(psStats, i);
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pBrainData =
            strchr(pBrainData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Power stats from the file exported from Access*/
/*BOOL loadPowerStats(void)
{
	char *pPowerData, *pStartPowerData;
	UDWORD fileSize;
	POWER_STATS	*psStats, *psStartStats;
	UDWORD	NumPower = 0,i;
	STRING  PowerName[50];
	BOOL EndOfFile;


	if (!loadFile("PowerPlant.txt", &pPowerData, &fileSize))
	{
		return FALSE;
	}
	pStartPowerData = pPowerData;

	psStats = (POWER_STATS *)MALLOC(sizeof(POWER_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Power Stats - Out of memory"));
		return FALSE;
	}
	//reserve the start of the data
	psStartStats = psStats;

	EndOfFile = FALSE;
	//determine the number of records to add by counting the number of '\n'
	while (!EndOfFile)
	{
		pPowerData = strchr(pPowerData,'\n');
		if (pPowerData == NULL)
		{
			EndOfFile = TRUE;
		}
		else
		{
			pPowerData++;
			NumPower++;
		}
	}
	//return to the start of the data
	pPowerData = pStartPowerData;

	if (!statsAllocPower(NumPower))
	{
		return FALSE;
	}

	for (i=0; i < NumPower; i++)
	{
		memset(psStats, 0, sizeof(POWER_STATS));

		//read the data into the storage - the data is delimeted using comma's
		sscanf(pPowerData,"%[^','],%d,%d,%d,%d,%d,%d",
			&PowerName, &psStats->buildPower,&psStats->buildPoints,
			&psStats->weight, &psStats->hitPoints, &psStats->systemPoints,
			&psStats->output);

		//allocate storage for the name
		psStats->pName = (STRING *)MALLOC((strlen(PowerName))+1);
		if (psStats->pName == NULL)
		{
			DBERROR(("Power Stats Name - Out of memory"));
			return FALSE;
		}
		strcpy(psStats->pName,PowerName);

		psStats->ref = REF_POWER_START + i;

		//save the stats
		statsSetPower(psStats, i);

		psStats = psStartStats;
		//increment the pointer to the start of the next record
		pPowerData = strchr(pPowerData,'\n') + 1;
	}
	FREE(pStartPowerData);
	FREE(psStats);
	return TRUE;
}
*/
/*returns the propulsion type based on the string name passed in */
#[no_mangle]
pub unsafe extern "C" fn getPropulsionType(mut pType: *mut STRING) -> UBYTE {
    if strcmp(pType, b"Wheeled\x00" as *const u8 as *const libc::c_char) == 0
       {
        return WHEELED as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Tracked\x00" as *const u8 as *const libc::c_char) == 0
       {
        return TRACKED as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Legged\x00" as *const u8 as *const libc::c_char) == 0 {
        return LEGGED as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Hover\x00" as *const u8 as *const libc::c_char) == 0 {
        return HOVER as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Ski\x00" as *const u8 as *const libc::c_char) == 0 {
        return SKI as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Lift\x00" as *const u8 as *const libc::c_char) == 0 {
        return LIFT as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Propellor\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return PROPELLOR as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Half-Tracked\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return HALF_TRACKED as libc::c_int as UBYTE
    }
    if strcmp(pType, b"Jump\x00" as *const u8 as *const libc::c_char) == 0 {
        return JUMP as libc::c_int as UBYTE
    }
    return (NUM_PROP_TYPES as libc::c_int + 1 as libc::c_int) as UBYTE;
}
/*Load the Propulsion stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadPropulsionStats(mut pPropulsionData:
                                                 *mut libc::c_char,
                                             mut bufferSize: UDWORD) -> BOOL {
    //SBYTE				*pData;
    let mut sStats: PROPULSION_STATS =
        PROPULSION_STATS{ref_0: 0,
                         pName: 0 as *mut STRING,
                         techLevel: TECH_LEVEL_ONE,
                         buildPower: 0,
                         buildPoints: 0,
                         weight: 0,
                         hitPoints: 0,
                         systemPoints: 0,
                         body: 0,
                         design: 0,
                         pIMD: 0 as *mut iIMDShape,
                         maxSpeed: 0,
                         propulsionType: 0,};
    let mut psStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut psStartStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut NumPropulsion: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut PropulsionName: [STRING; 60] = [0; 60];
    let mut imdName: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut type_0: [STRING; 60] = [0; 60];
    //keep the start so we release it at the end
	//pData = pPropulsionData;
    psStats = &mut sStats;
    /*	psStats = (PROPULSION_STATS *)MALLOC(sizeof(PROPULSION_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Propulsion Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumPropulsion = numCR(pPropulsionData, bufferSize);
    if statsAllocPropulsion(NumPropulsion) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumPropulsion {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong);
        PropulsionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        imdName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pPropulsionData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut PropulsionName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut imdName as *mut [STRING; 60] as *mut libc::c_char,
               &mut type_0 as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).maxSpeed as *mut UDWORD,
               &mut designable as *mut UDWORD);
        if allocateStatName(psStats as *mut BASE_STATS,
                            PropulsionName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x40000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //deal with imd - stored so that got something to see in Design Screen!
        if strcmp(imdName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, imdName.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the propulsion PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        //set up the stats type
        (*psStats).propulsionType = getPropulsionType(type_0.as_mut_ptr());
        if (*psStats).propulsionType as libc::c_int ==
               NUM_PROP_TYPES as libc::c_int + 1 as libc::c_int {
            debug(LOG_ERROR,
                  b"loadPropulsionStats: Invalid Propulsion type for %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(psStats as *mut libc::c_void));
            abort();
        }
        //save the stats
        statsSetPropulsion(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxPropulsionSpeed((*psStats).maxSpeed);
            //setMaxComponentWeight(psStats->weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pPropulsionData =
            strchr(pPropulsionData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    /*since propulsion weight is a multiple of body weight we may need to
    adjust the max component weight value*/
    //check we've loaded them both in
    if !asBodyStats.is_null() && !asPropulsionStats.is_null() {
        //check against each body stat
        i = 0 as libc::c_int as UDWORD;
        while i < numBodyStats {
            //check stat is designable
            if (*asBodyStats.offset(i as isize)).design != 0 {
                //check against each propulsion stat
                NumPropulsion = 0 as libc::c_int as UDWORD;
                while NumPropulsion < numPropulsionStats {
                    //check stat is designable
                    if (*asPropulsionStats.offset(NumPropulsion as
                                                      isize)).design != 0 {
                        setMaxComponentWeight((*asPropulsionStats.offset(NumPropulsion
                                                                             as
                                                                             isize)).weight.wrapping_mul((*asBodyStats.offset(i
                                                                                                                                  as
                                                                                                                                  isize)).weight).wrapping_div(100
                                                                                                                                                                   as
                                                                                                                                                                   libc::c_int
                                                                                                                                                                   as
                                                                                                                                                                   libc::c_uint));
                    }
                    NumPropulsion = NumPropulsion.wrapping_add(1)
                }
            }
            i = i.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
}
/*Load the Sensor stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadSensorStats(mut pSensorData: *mut libc::c_char,
                                         mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut sStats: SENSOR_STATS =
        SENSOR_STATS{ref_0: 0,
                     pName: 0 as *mut STRING,
                     techLevel: TECH_LEVEL_ONE,
                     buildPower: 0,
                     buildPoints: 0,
                     weight: 0,
                     hitPoints: 0,
                     systemPoints: 0,
                     body: 0,
                     design: 0,
                     pIMD: 0 as *mut iIMDShape,
                     range: 0,
                     power: 0,
                     location: 0,
                     type_0: STANDARD_SENSOR,
                     time: 0,
                     pMountGraphic: 0 as *mut iIMDShape,};
    let mut psStats: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    let mut psStartStats: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    let mut NumSensor: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut SensorName: [STRING; 60] = [0; 60];
    let mut location: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut type_0: [STRING; 60] = [0; 60];
    let mut mountGfx: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    //keep the start so we release it at the end
	//pData = pSensorData;
    psStats = &mut sStats;
    /*	psStats = (SENSOR_STATS *)MALLOC(sizeof(SENSOR_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Sensor Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumSensor = numCR(pSensorData, bufferSize);
    if statsAllocSensor(NumSensor) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumSensor {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong);
        SensorName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        mountGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        location[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        type_0[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pSensorData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%d,%[^\',\'],%[^\',\'],%d,%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut SensorName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut mountGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).range as *mut UDWORD,
               &mut location as *mut [STRING; 60] as *mut libc::c_char,
               &mut type_0 as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).time as *mut UDWORD,
               &mut (*psStats).power as *mut UDWORD,
               &mut designable as *mut UDWORD);
        if allocateStatName(psStats as *mut BASE_STATS,
                            SensorName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x50000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        if strcmp(location.as_mut_ptr(),
                  b"DEFAULT\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).location = LOC_DEFAULT as libc::c_int as UDWORD
        } else if strcmp(location.as_mut_ptr(),
                         b"TURRET\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            (*psStats).location = LOC_TURRET as libc::c_int as UDWORD
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid Sensor location\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1593 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadSensorStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        if strcmp(type_0.as_mut_ptr(),
                  b"STANDARD\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).type_0 = STANDARD_SENSOR
        } else if strcmp(type_0.as_mut_ptr(),
                         b"INDIRECT CB\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            (*psStats).type_0 = INDIRECT_CB_SENSOR
        } else if strcmp(type_0.as_mut_ptr(),
                         b"VTOL CB\x00" as *const u8 as *const libc::c_char)
                      == 0 {
            (*psStats).type_0 = VTOL_CB_SENSOR
        } else if strcmp(type_0.as_mut_ptr(),
                         b"VTOL INTERCEPT\x00" as *const u8 as
                             *const libc::c_char) == 0 {
            (*psStats).type_0 = VTOL_INTERCEPT_SENSOR
        } else if strcmp(type_0.as_mut_ptr(),
                         b"SUPER\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            (*psStats).type_0 = SUPER_SENSOR
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid Sensor type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1618 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadSensorStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        //multiply time stats
        (*psStats).time =
            ((*psStats).time as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //get the IMD for the component
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the sensor PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pMountGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, mountGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMountGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the mount PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pMountGraphic = 0 as *mut iIMDShape }
        //save the stats
        statsSetSensor(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxSensorRange((*psStats).range);
            setMaxSensorPower((*psStats).power);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pSensorData =
            strchr(pSensorData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the ECM stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadECMStats(mut pECMData: *mut libc::c_char,
                                      mut bufferSize: UDWORD) -> BOOL {
    //SBYTE		*pData;
    let mut sStats: ECM_STATS =
        ECM_STATS{ref_0: 0,
                  pName: 0 as *mut STRING,
                  techLevel: TECH_LEVEL_ONE,
                  buildPower: 0,
                  buildPoints: 0,
                  weight: 0,
                  hitPoints: 0,
                  systemPoints: 0,
                  body: 0,
                  design: 0,
                  pIMD: 0 as *mut iIMDShape,
                  range: 0,
                  power: 0,
                  location: 0,
                  pMountGraphic: 0 as *mut iIMDShape,};
    let mut psStats: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut psStartStats: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut NumECM: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut ECMName: [STRING; 60] = [0; 60];
    let mut location: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut mountGfx: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    //keep the start so we release it at the end
	//pData = pECMData;
    psStats = &mut sStats;
    /*	psStats = (ECM_STATS *)MALLOC(sizeof(ECM_STATS));
	if (psStats == NULL)
	{
		DBERROR(("ECM Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumECM = numCR(pECMData, bufferSize);
    if statsAllocECM(NumECM) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumECM {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong);
        ECMName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        mountGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        location[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pECMData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],%[^\',\'],\t\t\t%[^\',\'],%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut ECMName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut mountGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut location as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).power as *mut UDWORD,
               &mut designable as *mut UDWORD);
        // set a default ECM range for now
        (*psStats).range = (128 as libc::c_int * 8 as libc::c_int) as UDWORD;
        if allocateStatName(psStats as *mut BASE_STATS, ECMName.as_mut_ptr())
               == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x60000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        if strcmp(location.as_mut_ptr(),
                  b"DEFAULT\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).location = LOC_DEFAULT as libc::c_int as UDWORD
        } else if strcmp(location.as_mut_ptr(),
                         b"TURRET\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            (*psStats).location = LOC_TURRET as libc::c_int as UDWORD
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid ECM location\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1763 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"loadECMStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //get the IMD for the component
        if strcmp(GfxFile.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the ECM PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pMountGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, mountGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMountGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the mount PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else {
            //set to NULL
            (*psStats).pMountGraphic = 0 as *mut iIMDShape
        }
        //save the stats
        statsSetECM(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxECMPower((*psStats).power);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pECMData =
            strchr(pECMData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Repair stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadRepairStats(mut pRepairData: *mut libc::c_char,
                                         mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut sStats: REPAIR_STATS =
        REPAIR_STATS{ref_0: 0,
                     pName: 0 as *mut STRING,
                     techLevel: TECH_LEVEL_ONE,
                     buildPower: 0,
                     buildPoints: 0,
                     weight: 0,
                     hitPoints: 0,
                     systemPoints: 0,
                     body: 0,
                     design: 0,
                     pIMD: 0 as *mut iIMDShape,
                     repairPoints: 0,
                     repairArmour: 0,
                     location: 0,
                     time: 0,
                     pMountGraphic: 0 as *mut iIMDShape,};
    let mut psStats: *mut REPAIR_STATS = 0 as *mut REPAIR_STATS;
    let mut psStartStats: *mut REPAIR_STATS = 0 as *mut REPAIR_STATS;
    let mut NumRepair: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut RepairName: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut mountGfx: [STRING; 60] = [0; 60];
    let mut location: [STRING; 60] = [0; 60];
    //keep the start so we can release it at the end
	//pData = pRepairData;
    psStats = &mut sStats;
    /*	psStats = (REPAIR_STATS *)MALLOC(sizeof(REPAIR_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Repair Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumRepair = numCR(pRepairData, bufferSize);
    if statsAllocRepair(NumRepair) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumRepair {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong);
        RepairName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        mountGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        location[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pRepairData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%[^\',\'],%d,%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut RepairName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).repairArmour as *mut BOOL,
               &mut location as *mut [STRING; 60] as *mut libc::c_char,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut mountGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).repairPoints as *mut UDWORD,
               &mut (*psStats).time as *mut UDWORD,
               &mut designable as *mut UDWORD);
        if allocateStatName(psStats as *mut BASE_STATS,
                            RepairName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0x80000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        if strcmp(location.as_mut_ptr(),
                  b"DEFAULT\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStats).location = LOC_DEFAULT as libc::c_int as UDWORD
        } else if strcmp(location.as_mut_ptr(),
                         b"TURRET\x00" as *const u8 as *const libc::c_char) ==
                      0 {
            (*psStats).location = LOC_TURRET as libc::c_int as UDWORD
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid Repair location\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1897 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadRepairStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        //multiply time stats
        (*psStats).time =
            ((*psStats).time as
                 libc::c_uint).wrapping_mul(100 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        //check its not 0 since we will be dividing by it at a later stage
        if (*psStats).time == 0 as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadRepairStats: the delay time cannot be zero for %s\x00"
                          as *const u8 as *const libc::c_char,
                      (*psStats).pName);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      1908 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"loadRepairStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            (*psStats).time = 1 as libc::c_int as UDWORD
        }
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //get the IMD for the component
        if strcmp(GfxFile.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the Repair PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pMountGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, mountGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMountGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the Repair mount PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else {
            //set to NULL
            (*psStats).pMountGraphic = 0 as *mut iIMDShape
        }
        //save the stats
        statsSetRepair(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxRepairPoints((*psStats).repairPoints);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pRepairData =
            strchr(pRepairData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Program stats from the file exported from Access*/
/*BOOL loadProgramStats(char *pProgramData, UDWORD bufferSize)
{
	//SBYTE			*pData;
	PROGRAM_STATS	sStats, *psStats, *psStartStats;
	UDWORD			NumProgram = 0,i;
	STRING			ProgramName[MAX_NAME_SIZE], techLevel[MAX_NAME_SIZE];

	//keep the start so we can release it at the end
	//pData = pProgramData;

	psStats = &sStats;
//	psStats = (PROGRAM_STATS *)MALLOC(sizeof(PROGRAM_STATS));
//	if (psStats == NULL)
//	{
//		DBERROR(("Program Stats - Out of memory"));
//		return FALSE;
//	}
	//reserve the start of the data
	psStartStats = psStats;

	NumProgram = numCR(pProgramData, bufferSize);
	if (!statsAllocProgram(NumProgram))
	{
		return FALSE;
	}

	for (i=0; i < NumProgram; i++)
	{
		memset(psStats, 0, sizeof(PROGRAM_STATS));

		ProgramName[0] = '\0';
		techLevel[0] = '\0';
		//read the data into the storage - the data is delimeted using comma's
		sscanf(pProgramData,"%[^','],%[^','],%d,%d,%d,%d,%d",
			&ProgramName, &techLevel, &psStats->buildPower,&psStats->buildPoints,
			&psStats->slots, &psStats->order, &psStats->special);

		if (!allocateStatName((BASE_STATS *)psStats, ProgramName))
		{
			return FALSE;
		}

		psStats->ref = REF_PROGRAM_START + i;

		//determine the tech level
		if (!setTechLevel((BASE_STATS *)psStats, techLevel))
		{
			return FALSE;
		}

		//save the stats
		statsSetProgram(psStats, i);

		psStats = psStartStats;
		//increment the pointer to the start of the next record
		pProgramData = strchr(pProgramData,'\n') + 1;
	}
//	FREE(pData);
//	FREE(psStats);
	return TRUE;
}*/
/*Load the Construct stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadConstructStats(mut pConstructData:
                                                *mut libc::c_char,
                                            mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut sStats: CONSTRUCT_STATS =
        CONSTRUCT_STATS{ref_0: 0,
                        pName: 0 as *mut STRING,
                        techLevel: TECH_LEVEL_ONE,
                        buildPower: 0,
                        buildPoints: 0,
                        weight: 0,
                        hitPoints: 0,
                        systemPoints: 0,
                        body: 0,
                        design: 0,
                        pIMD: 0 as *mut iIMDShape,
                        constructPoints: 0,
                        pMountGraphic: 0 as *mut iIMDShape,};
    let mut psStats: *mut CONSTRUCT_STATS = 0 as *mut CONSTRUCT_STATS;
    let mut psStartStats: *mut CONSTRUCT_STATS = 0 as *mut CONSTRUCT_STATS;
    let mut NumConstruct: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut designable: UDWORD = 0;
    let mut ConstructName: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut mountGfx: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    //keep the start so we release it at the end
	//pData = pConstructData;
    psStats = &mut sStats;
    /*	psStats = (CONSTRUCT_STATS *)MALLOC(sizeof(CONSTRUCT_STATS));
	if (psStats == NULL)
	{
		DBERROR(("Construct Stats - Out of memory"));
		return FALSE;
	}*/
	//reserve the start of the data
    psStartStats = psStats;
    NumConstruct = numCR(pConstructData, bufferSize);
    if statsAllocConstruct(NumConstruct) == 0 { return 0 as libc::c_int }
    i = 0 as libc::c_int as UDWORD;
    while i < NumConstruct {
        memset(psStats as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong);
        ConstructName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        techLevel[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        mountGfx[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pConstructData,
               b"%[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%[^\',\'],\t\t\t%[^\',\'],%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut ConstructName as *mut [STRING; 60] as *mut libc::c_char,
               &mut techLevel as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).buildPower as *mut UDWORD,
               &mut (*psStats).buildPoints as *mut UDWORD,
               &mut (*psStats).weight as *mut UDWORD,
               &mut (*psStats).hitPoints as *mut UDWORD,
               &mut (*psStats).systemPoints as *mut UDWORD,
               &mut (*psStats).body as *mut UDWORD,
               &mut GfxFile as *mut [STRING; 60] as *mut libc::c_char,
               &mut mountGfx as *mut [STRING; 60] as *mut libc::c_char,
               &mut (*psStats).constructPoints as *mut UDWORD,
               &mut designable as *mut UDWORD);
        if allocateStatName(psStats as *mut BASE_STATS,
                            ConstructName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        (*psStats).ref_0 =
            (0xf0000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        if setTechLevel(psStats as *mut BASE_STATS, techLevel.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //set design flag
        if designable != 0 {
            (*psStats).design = 1 as libc::c_int
        } else { (*psStats).design = 0 as libc::c_int }
        //get the IMD for the component
        if strcmp(GfxFile.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the constructor PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else { (*psStats).pIMD = 0 as *mut iIMDShape }
        if strcmp(mountGfx.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStats).pMountGraphic =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, mountGfx.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStats).pMountGraphic.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the mount PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getStatName(psStats as *mut libc::c_void));
                abort();
            }
        } else {
            //set to NULL
            (*psStats).pMountGraphic = 0 as *mut iIMDShape
        }
        //save the stats
        statsSetConstruct(psStats, i);
        //set the max stat values for the design screen
        if (*psStats).design != 0 {
            setMaxConstPoints((*psStats).constructPoints);
            setMaxComponentWeight((*psStats).weight);
        }
        psStats = psStartStats;
        //increment the pointer to the start of the next record
        pConstructData =
            strchr(pConstructData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
//	FREE(psStats);
    return 1 as libc::c_int;
}
/*Load the Propulsion Types from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadPropulsionTypes(mut pPropTypeData:
                                                 *mut libc::c_char,
                                             mut bufferSize: UDWORD) -> BOOL {
    //SBYTE				*pData;
    let mut pPropType: *mut PROPULSION_TYPES = 0 as *mut PROPULSION_TYPES;
    let mut NumTypes: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut multiplier: UDWORD = 0;
    let mut type_0: UDWORD = 0;
    let mut PropulsionName: [STRING; 60] = [0; 60];
    let mut flightName: [STRING; 60] = [0; 60];
    //keep the start so we can release it at the end
	//pData = pPropTypeData;
    //NumTypes = numCR(pPropTypeData, bufferSize);
    NumTypes = NUM_PROP_TYPES as libc::c_int as UDWORD;
    //allocate storage for the stats
    asPropulsionTypes =
        memMallocRelease((::std::mem::size_of::<PROPULSION_TYPES>() as
                              libc::c_ulong).wrapping_mul(NumTypes)) as
            *mut PROPULSION_TYPES;
    if asPropulsionTypes.is_null() {
        debug(LOG_ERROR,
              b"PropulsionTypes - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //	numPropulsionTypes = NumTypes;
    memset(asPropulsionTypes as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<PROPULSION_TYPES>() as
                libc::c_ulong).wrapping_mul(NumTypes));
    i = 0 as libc::c_int as UDWORD;
    while i < NumTypes {
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pPropTypeData,
               b"%[^\',\'],%[^\',\'],%d\x00" as *const u8 as
                   *const libc::c_char,
               &mut PropulsionName as *mut [STRING; 60] as *mut libc::c_char,
               &mut flightName as *mut [STRING; 60] as *mut libc::c_char,
               &mut multiplier as *mut UDWORD);
        //allocate storage for the name
/*#ifdef HASH_NAMES
		asPropulsionTypes->NameHash=HashString(PropulsionName);
#else
		asPropulsionTypes->pName = (STRING *)MALLOC((strlen(PropulsionName))+1);
		if (asPropulsionTypes->pName == NULL)
		{
			DBERROR(("Propulsion Type Name - Out of memory"));
			return FALSE;
		}
		strcpy(asPropulsionTypes->pName,PropulsionName);
#endif
*/
		//set the pointer for this record based on the name
        type_0 = getPropulsionType(PropulsionName.as_mut_ptr()) as UDWORD;
        if type_0 ==
               (NUM_PROP_TYPES as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadPropulsionTypes: Invalid Propulsion type - %s\x00" as
                      *const u8 as *const libc::c_char,
                  PropulsionName.as_mut_ptr());
            abort();
        }
        pPropType = asPropulsionTypes.offset(type_0 as isize);
        if strcmp(flightName.as_mut_ptr(),
                  b"GROUND\x00" as *const u8 as *const libc::c_char) == 0 {
            (*pPropType).travel = GROUND as libc::c_int as UDWORD
        } else if strcmp(flightName.as_mut_ptr(),
                         b"AIR\x00" as *const u8 as *const libc::c_char) == 0
         {
            (*pPropType).travel = AIR as libc::c_int as UDWORD
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid travel type for Propulsion\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      2227 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"loadPropulsionTypes\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        //don't care about this anymore! AB FRIDAY 13/11/98
        //want it back again! AB 27/11/98
        if multiplier > 0xffff as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"loadPropulsionTypes: power Ratio multiplier too high\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"stats.c\x00" as *const u8 as *const libc::c_char,
                      2234 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"loadPropulsionTypes\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            //set to a default value since not life threatening!
            multiplier = 100 as libc::c_int as UDWORD
        }
        (*pPropType).powerRatioMult = multiplier as UWORD;
        //initialise all the sound variables
        (*pPropType).startID = NO_SOUND as libc::c_int as SWORD;
        (*pPropType).idleID = NO_SOUND as libc::c_int as SWORD;
        (*pPropType).moveOffID = NO_SOUND as libc::c_int as SWORD;
        (*pPropType).moveID = NO_SOUND as libc::c_int as SWORD;
        (*pPropType).hissID = NO_SOUND as libc::c_int as SWORD;
        (*pPropType).shutDownID = NO_SOUND as libc::c_int as SWORD;
        //increment the pointer to the start of the next record
        pPropTypeData =
            strchr(pPropTypeData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    return 1 as libc::c_int;
}
/*Load the Terrain Table from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadTerrainTable(mut pTerrainTableData:
                                              *mut libc::c_char,
                                          mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut pTerrainTable: *mut TERRAIN_TABLE = 0 as *mut TERRAIN_TABLE;
    let mut NumEntries: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut terrainType: UDWORD = 0;
    let mut propulsionType: UDWORD = 0;
    let mut speedFactor: UDWORD = 0;
    //pData = pTerrainTableData;
    NumEntries = numCR(pTerrainTableData, bufferSize);
    //allocate storage for the stats
    asTerrainTable =
        memMallocRelease((::std::mem::size_of::<TERRAIN_TABLE>() as
                              libc::c_ulong).wrapping_mul(NUM_PROP_TYPES as
                                                              libc::c_int as
                                                              libc::c_uint).wrapping_mul(TER_MAX
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint))
            as *mut TERRAIN_TABLE;
    if asTerrainTable.is_null() {
        debug(LOG_ERROR,
              b"Terrain Types - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //initialise the storage to 100
	//memset(asTerrainTable, 0, (sizeof(TERRAIN_TABLE) * numPropulsionTypes
		//* TERRAIN_TYPES));
    i = 0 as libc::c_int as UDWORD;
    while i < TER_MAX as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
            pTerrainTable =
                asTerrainTable.offset(i.wrapping_mul(NUM_PROP_TYPES as
                                                         libc::c_int as
                                                         libc::c_uint).wrapping_add(j)
                                          as isize);
            (*pTerrainTable).speedFactor = 100 as libc::c_int as UDWORD;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    //copy the start location
	//pTerrainTable = asTerrainTable;
    i = 0 as libc::c_int as UDWORD;
    while i < NumEntries {
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pTerrainTableData,
               b"%d,%d,%d\x00" as *const u8 as *const libc::c_char,
               &mut terrainType as *mut UDWORD,
               &mut propulsionType as *mut UDWORD,
               &mut speedFactor as *mut UDWORD);
        //store the speed factor at the correct location from the start
        storeSpeedFactor(terrainType, propulsionType, speedFactor);
        //increment the pointer to the start of the next record
        pTerrainTableData =
            strchr(pTerrainTableData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    //check that none of the entries are 0 otherwise this will stop a droid dead in its tracks
	//and it will not be able to move again!
    i = 0 as libc::c_int as UDWORD;
    while i < TER_MAX as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
            pTerrainTable =
                asTerrainTable.offset(i.wrapping_mul(NUM_PROP_TYPES as
                                                         libc::c_int as
                                                         libc::c_uint).wrapping_add(j)
                                          as isize);
            if (*pTerrainTable).speedFactor ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"loadTerrainTable: Invalid propulsion/terrain table entry\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*Load the Special Ability stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadSpecialAbility(mut pSAbilityData:
                                                *mut libc::c_char,
                                            mut bufferSize: UDWORD) -> BOOL {
    //SBYTE			*pData;
    let mut pSAbility: *mut SPECIAL_ABILITY = 0 as *mut SPECIAL_ABILITY;
    let mut NumTypes: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut accessID: UDWORD = 0;
    let mut SAbilityName: [STRING; 60] = [0; 60];
    //keep the start so we can release it at the end
	//pData = pSAbilityData;
    NumTypes = numCR(pSAbilityData, bufferSize);
    //allocate storage for the stats
    asSpecialAbility =
        memMallocRelease((::std::mem::size_of::<SPECIAL_ABILITY>() as
                              libc::c_ulong).wrapping_mul(NumTypes)) as
            *mut SPECIAL_ABILITY;
    if asSpecialAbility.is_null() {
        debug(LOG_ERROR,
              b"SpecialAbility - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    numSpecialAbility = NumTypes;
    memset(asSpecialAbility as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<SPECIAL_ABILITY>() as
                libc::c_ulong).wrapping_mul(NumTypes));
    //copy the start location
    pSAbility = asSpecialAbility;
    i = 0 as libc::c_int as UDWORD;
    while i < NumTypes {
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pSAbilityData,
               b"%[^\',\'],%d\x00" as *const u8 as *const libc::c_char,
               &mut SAbilityName as *mut [STRING; 60] as *mut libc::c_char,
               &mut accessID as *mut UDWORD);
        //check that the data is ordered in the way it will be stored
        if accessID != i {
            debug(LOG_ERROR,
                  b"The Special Ability sequence is invalid\x00" as *const u8
                      as *const libc::c_char);
            abort();
        }
        //allocate storage for the name
        (*asSpecialAbility).pName =
            memMallocRelease(strlen(SAbilityName.as_mut_ptr()).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint))
                as *mut STRING;
        if (*asSpecialAbility).pName.is_null() {
            debug(LOG_ERROR,
                  b"Special Ability Name - Out of memory\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        strcpy((*asSpecialAbility).pName, SAbilityName.as_mut_ptr());
        //increment the pointer to the start of the next record
        pSAbilityData =
            strchr(pSAbilityData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        asSpecialAbility = asSpecialAbility.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    //reset the pointer to the start of the special ability stats
    asSpecialAbility = pSAbility;
    return 1 as libc::c_int;
}
/* load the IMDs to use for each body-propulsion combination */
#[no_mangle]
pub unsafe extern "C" fn loadBodyPropulsionIMDs(mut pData: *mut libc::c_char,
                                                mut bufferSize: UDWORD)
 -> BOOL {
    //	SBYTE				*pStartData;
    let mut psBodyStat: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut psPropulsionStat: *mut PROPULSION_STATS =
        0 as *mut PROPULSION_STATS;
    let mut NumTypes: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut numStats: UDWORD = 0;
    let mut bodyName: [STRING; 60] = [0; 60];
    let mut propulsionName: [STRING; 60] = [0; 60];
    let mut leftIMD: [STRING; 60] = [0; 60];
    let mut rightIMD: [STRING; 60] = [0; 60];
    let mut startIMDs: *mut *mut iIMDShape = 0 as *mut *mut iIMDShape;
    let mut found: BOOL = 0;
    //check that the body and propulsion stats have already been read in
    if !asBodyStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Body Stats have not been set up\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !asBodyStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2406 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"loadBodyPropulsionIMDs\x00")).as_ptr(),
              b"asBodyStats != NULL\x00" as *const u8 as *const libc::c_char);
    };
    if !asPropulsionStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Propulsion Stats have not been set up\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !asPropulsionStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2407 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"loadBodyPropulsionIMDs\x00")).as_ptr(),
              b"asPropulsionStats != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    psBodyStat = asBodyStats;
    psPropulsionStat = asPropulsionStats;
    //allocate space
    numStats = 0 as libc::c_int as UDWORD;
    while numStats < numBodyStats {
        psBodyStat =
            &mut *asBodyStats.offset(numStats as isize) as *mut BODY_STATS;
        (*psBodyStat).ppIMDList =
            memMallocRelease(numPropulsionStats.wrapping_mul(NUM_PROP_SIDES as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut iIMDShape>()
                                                                                                as
                                                                                                libc::c_ulong))
                as *mut *mut iIMDShape;
        if (*psBodyStat).ppIMDList.is_null() {
            debug(LOG_ERROR,
                  b"Out of memory\x00" as *const u8 as *const libc::c_char);
            abort();
        }
        //initialise the pointer space
        memset((*psBodyStat).ppIMDList as *mut libc::c_void, 0 as libc::c_int,
               numPropulsionStats.wrapping_mul(NUM_PROP_SIDES as libc::c_int
                                                   as
                                                   libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut iIMDShape>()
                                                                                  as
                                                                                  libc::c_ulong));
        numStats = numStats.wrapping_add(1)
    }
    //keep the start so we can release it at the end
	//pStartData = pData;
    NumTypes = numCR(pData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumTypes {
        bodyName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        propulsionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        leftIMD[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        rightIMD[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        /*read the data into the storage - the data is delimited using comma's
		not interested in the last number - needed for sscanf*/
        sscanf(pData,
               b"%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%*d\x00" as *const u8
                   as *const libc::c_char,
               &mut bodyName as *mut [STRING; 60] as *mut libc::c_char,
               &mut propulsionName as *mut [STRING; 60] as *mut libc::c_char,
               &mut leftIMD as *mut [STRING; 60] as *mut libc::c_char,
               &mut rightIMD as *mut [STRING; 60] as *mut libc::c_char);
        //get the body stats
        found = 0 as libc::c_int;
        if getResourceName(bodyName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        numStats = 0 as libc::c_int as UDWORD;
        while numStats < numBodyStats {
            psBodyStat =
                &mut *asBodyStats.offset(numStats as isize) as
                    *mut BODY_STATS;
            if strcmp((*psBodyStat).pName, bodyName.as_mut_ptr()) == 0 {
                found = 1 as libc::c_int;
                break ;
            } else { numStats = numStats.wrapping_add(1) }
        }
        if found == 0 {
            debug(LOG_ERROR,
                  b"loadBodyPropulsionPIEs: Invalid body name %s\x00" as
                      *const u8 as *const libc::c_char,
                  bodyName.as_mut_ptr());
            abort();
        }
        //get the propulsion stats
        found = 0 as libc::c_int;
        if getResourceName(propulsionName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        numStats = 0 as libc::c_int as UDWORD;
        while numStats < numPropulsionStats {
            psPropulsionStat =
                &mut *asPropulsionStats.offset(numStats as isize) as
                    *mut PROPULSION_STATS;
            if strcmp((*psPropulsionStat).pName, propulsionName.as_mut_ptr())
                   == 0 {
                found = 1 as libc::c_int;
                break ;
            } else { numStats = numStats.wrapping_add(1) }
        }
        if found == 0 {
            debug(LOG_ERROR,
                  b"Invalid propulsion name %s\x00" as *const u8 as
                      *const libc::c_char, propulsionName.as_mut_ptr());
            abort();
        }
        //allocate the left and right propulsion IMDs
        startIMDs = (*psBodyStat).ppIMDList;
        (*psBodyStat).ppIMDList =
            (*psBodyStat).ppIMDList.offset(numStats.wrapping_mul(NUM_PROP_SIDES
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                               as isize);
        if strcmp(leftIMD.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            *(*psBodyStat).ppIMDList =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, leftIMD.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*(*psBodyStat).ppIMDList).is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the left propulsion PIE for body %s\x00"
                          as *const u8 as *const libc::c_char,
                      bodyName.as_mut_ptr());
                abort();
            }
        } else { *(*psBodyStat).ppIMDList = 0 as *mut iIMDShape }
        (*psBodyStat).ppIMDList = (*psBodyStat).ppIMDList.offset(1);
        //right IMD might not be there
        if strcmp(rightIMD.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            *(*psBodyStat).ppIMDList =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, rightIMD.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*(*psBodyStat).ppIMDList).is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the right propulsion PIE for body %s\x00"
                          as *const u8 as *const libc::c_char,
                      bodyName.as_mut_ptr());
                abort();
            }
        } else { *(*psBodyStat).ppIMDList = 0 as *mut iIMDShape }
        //reset the IMDList pointer
        (*psBodyStat).ppIMDList = startIMDs;
        //increment the pointer to the start of the next record
        pData = strchr(pData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartData);
    return 1 as libc::c_int;
}
unsafe extern "C" fn statsGetAudioIDFromString(mut szStatName: *mut STRING,
                                               mut szWavName: *mut STRING,
                                               mut piWavID: *mut SDWORD)
 -> BOOL {
    if strcmp(szWavName, b"-1\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        *piWavID = NO_SOUND as libc::c_int
    } else if audioID_GetIDFromStr(szWavName, piWavID) == 0 as libc::c_int {
        debug(LOG_ERROR,
              b"statsGetAudioIDFromString: couldn\'t get ID %d for sound %s\x00"
                  as *const u8 as *const libc::c_char, *piWavID, szWavName);
        abort();
    }
    if (*piWavID < 0 as libc::c_int ||
            *piWavID >= ID_MAX_SOUND as libc::c_int) &&
           *piWavID != NO_SOUND as libc::c_int {
        debug(LOG_ERROR,
              b"statsGetAudioIDFromString: Invalid ID - %d for sound %s\x00"
                  as *const u8 as *const libc::c_char, *piWavID, szStatName);
        abort();
    }
    return 1 as libc::c_int;
}
/*Load the weapon sounds from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadWeaponSounds(mut pSoundData: *mut libc::c_char,
                                          mut bufferSize: UDWORD) -> BOOL {
    //#ifdef PSX
//#warning "loadWeaponSounds : NOT IMPLEMENTED ON PSX"
//	return TRUE;
//#else
	//SBYTE			*pData;
    let mut NumRecords: SDWORD = 0 as libc::c_int;
    let mut i: SDWORD = 0;
    let mut weaponSoundID: SDWORD = 0;
    let mut explosionSoundID: SDWORD = 0;
    let mut inc: SDWORD = 0;
    let mut iDum: SDWORD = 0;
    let mut WeaponName: [STRING; 60] = [0; 60];
    let mut szWeaponWav: [STRING; 60] = [0; 60];
    let mut szExplosionWav: [STRING; 60] = [0; 60];
    let mut Ok_0: BOOL = 1 as libc::c_int;
    NumRecords = numCR(pSoundData, bufferSize) as SDWORD;
    if !asWeaponStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"loadWeaponSounds: Weapon stats not loaded\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !asWeaponStats.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2607 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"loadWeaponSounds\x00")).as_ptr(),
              b"asWeaponStats != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    i = 0 as libc::c_int;
    while i < NumRecords {
        WeaponName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        szWeaponWav[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        szExplosionWav[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pSoundData,
               b"%[^\',\'],%[^\',\'],%[^\',\'],%d\x00" as *const u8 as
                   *const libc::c_char, WeaponName.as_mut_ptr(),
               szWeaponWav.as_mut_ptr(), szExplosionWav.as_mut_ptr(),
               &mut iDum as *mut SDWORD);
        if statsGetAudioIDFromString(WeaponName.as_mut_ptr(),
                                     szWeaponWav.as_mut_ptr(),
                                     &mut weaponSoundID) == 0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(WeaponName.as_mut_ptr(),
                                     szExplosionWav.as_mut_ptr(),
                                     &mut explosionSoundID) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        //find the weapon stat
        if getResourceName(WeaponName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        inc = 0 as libc::c_int;
        while inc < numWeaponStats as SDWORD {
            if strcmp((*asWeaponStats.offset(inc as isize)).pName,
                      WeaponName.as_mut_ptr()) == 0 {
                (*asWeaponStats.offset(inc as isize)).iAudioFireID =
                    weaponSoundID;
                (*asWeaponStats.offset(inc as isize)).iAudioImpactID =
                    explosionSoundID;
                break ;
            } else { inc += 1 }
        }
        if inc == numWeaponStats as SDWORD {
            debug(LOG_ERROR,
                  b"loadWeaponSounds: Weapon stat not found - %s\x00" as
                      *const u8 as *const libc::c_char,
                  WeaponName.as_mut_ptr());
            abort();
            //			return FALSE;
        }
        //increment the pointer to the start of the next record
        pSoundData =
            strchr(pSoundData, '\n' as i32).offset(1 as libc::c_int as isize);
        i += 1
    }
    //	return Ok;
    return 1 as libc::c_int;
    //#endif
}
/*Load the Weapon Effect Modifiers from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadWeaponModifiers(mut pWeapModData:
                                                 *mut libc::c_char,
                                             mut bufferSize: UDWORD) -> BOOL {
    let mut propInc: PROPULSION_TYPE = WHEELED;
    let mut effectInc: WEAPON_EFFECT = WE_ANTI_PERSONNEL;
    let mut NumRecords: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut modifier: UDWORD = 0;
    let mut weaponEffectName: [STRING; 60] = [0; 60];
    let mut propulsionName: [STRING; 60] = [0; 60];
    //memset(asWeaponModifier, 0, (sizeof(WEAPON_MODIFIER)*WE_NUMEFFECTS*NUM_PROP_TYPES));
	//initialise to 100%
    i = 0 as libc::c_int as UDWORD;
    while i < WE_NUMEFFECTS as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
            asWeaponModifier[i as usize][j as usize] =
                100 as libc::c_int as WEAPON_MODIFIER;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    NumRecords = numCR(pWeapModData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumRecords {
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pWeapModData,
               b"%[^\',\'],%[^\',\'],%d\x00" as *const u8 as
                   *const libc::c_char,
               &mut weaponEffectName as *mut [STRING; 60] as
                   *mut libc::c_char,
               &mut propulsionName as *mut [STRING; 60] as *mut libc::c_char,
               &mut modifier as *mut UDWORD);
        //get the weapon effect inc
        effectInc =
            getWeaponEffect(weaponEffectName.as_mut_ptr()) as WEAPON_EFFECT;
        if effectInc as libc::c_uint ==
               (WE_NUMEFFECTS as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadWeaponModifiers: Invalid Weapon Effect - %s\x00" as
                      *const u8 as *const libc::c_char,
                  weaponEffectName.as_mut_ptr());
            abort();
        }
        //get the propulsion inc
        propInc =
            getPropulsionType(propulsionName.as_mut_ptr()) as PROPULSION_TYPE;
        if propInc as libc::c_uint ==
               (NUM_PROP_TYPES as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadWeaponModifiers: Invalid Propulsion type - %s\x00" as
                      *const u8 as *const libc::c_char,
                  propulsionName.as_mut_ptr());
            abort();
        }
        if modifier > 0xffff as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"loadWeaponModifiers: modifier for effect %s, prop type %s is too large\x00"
                      as *const u8 as *const libc::c_char,
                  weaponEffectName.as_mut_ptr(), propulsionName.as_mut_ptr());
            abort();
        }
        //store in the appropriate index
        asWeaponModifier[effectInc as usize][propInc as usize] =
            modifier as UWORD;
        //increment the pointer to the start of the next record
        pWeapModData =
            strchr(pWeapModData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*Load the propulsion type sounds from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadPropulsionSounds(mut pPropSoundData:
                                                  *mut libc::c_char,
                                              mut bufferSize: UDWORD)
 -> BOOL {
    let mut NumRecords: SDWORD = 0 as libc::c_int;
    let mut i: SDWORD = 0;
    let mut startID: SDWORD = 0;
    let mut idleID: SDWORD = 0;
    let mut moveOffID: SDWORD = 0;
    let mut moveID: SDWORD = 0;
    let mut hissID: SDWORD = 0;
    let mut shutDownID: SDWORD = 0;
    let mut iDum: SDWORD = 0;
    let mut propulsionName: [STRING; 60] = [0; 60];
    let mut szStart: [STRING; 60] = [0; 60];
    let mut szIdle: [STRING; 60] = [0; 60];
    let mut szMoveOff: [STRING; 60] = [0; 60];
    let mut szMove: [STRING; 60] = [0; 60];
    let mut szHiss: [STRING; 60] = [0; 60];
    let mut szShutDown: [STRING; 60] = [0; 60];
    let mut type_0: UDWORD = 0;
    let mut pPropType: *mut PROPULSION_TYPES = 0 as *mut PROPULSION_TYPES;
    NumRecords = numCR(pPropSoundData, bufferSize) as SDWORD;
    if !asPropulsionTypes.is_null() {
    } else {
        debug(LOG_ERROR,
              b"loadPropulsionSounds: Propulsion type stats not loaded\x00" as
                  *const u8 as *const libc::c_char);
    };
    if !asPropulsionTypes.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2743 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"loadPropulsionSounds\x00")).as_ptr(),
              b"asPropulsionTypes != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    i = 0 as libc::c_int;
    while i < NumRecords {
        propulsionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pPropSoundData,
               b"%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%d\x00"
                   as *const u8 as *const libc::c_char,
               propulsionName.as_mut_ptr(), szStart.as_mut_ptr(),
               szIdle.as_mut_ptr(), szMoveOff.as_mut_ptr(),
               szMove.as_mut_ptr(), szHiss.as_mut_ptr(),
               szShutDown.as_mut_ptr(), &mut iDum as *mut SDWORD);
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szStart.as_mut_ptr(), &mut startID) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szIdle.as_mut_ptr(), &mut idleID) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szMoveOff.as_mut_ptr(), &mut moveOffID)
               == 0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szMove.as_mut_ptr(), &mut moveID) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szHiss.as_mut_ptr(), &mut hissID) ==
               0 as libc::c_int {
            return 0 as libc::c_int
        }
        if statsGetAudioIDFromString(propulsionName.as_mut_ptr(),
                                     szShutDown.as_mut_ptr(), &mut shutDownID)
               == 0 as libc::c_int {
            return 0 as libc::c_int
        }
        type_0 = getPropulsionType(propulsionName.as_mut_ptr()) as UDWORD;
        if type_0 ==
               (NUM_PROP_TYPES as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadPropulsionSounds: Invalid Propulsion type - %s\x00" as
                      *const u8 as *const libc::c_char,
                  propulsionName.as_mut_ptr());
            abort();
        }
        pPropType = asPropulsionTypes.offset(type_0 as isize);
        (*pPropType).startID = startID as SWORD;
        (*pPropType).idleID = idleID as SWORD;
        (*pPropType).moveOffID = moveOffID as SWORD;
        (*pPropType).moveID = moveID as SWORD;
        (*pPropType).hissID = hissID as SWORD;
        (*pPropType).shutDownID = shutDownID as SWORD;
        //increment the pointer to the start of the next record
        pPropSoundData =
            strchr(pPropSoundData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i += 1
    }
    return 1 as libc::c_int;
}
/* ******************************************************************************
*		Set stats functions
*******************************************************************************/
/* Set the stats for a particular weapon type */
#[no_mangle]
pub unsafe extern "C" fn statsSetWeapon(mut psStats: *mut WEAPON_STATS,
                                        mut index: UDWORD) {
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid WEAPON_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2811 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsSetWeapon\x00")).as_ptr(),
              b"((psStats)->ref >= (0x0a0000)) && ((psStats)->ref < (0x0a0000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asWeaponStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<WEAPON_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular armour type */
/*void statsSetArmour(ARMOUR_STATS	*psStats, UDWORD index)
{
	SET_STATS(psStats, asArmourStats, index, ARMOUR_STATS, REF_ARMOUR_START);
}*/
/* Set the stats for a particular body type */
#[no_mangle]
pub unsafe extern "C" fn statsSetBody(mut psStats: *mut BODY_STATS,
                                      mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid BODY_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2821 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"statsSetBody\x00")).as_ptr(),
              b"((psStats)->ref >= (0x010000)) && ((psStats)->ref < (0x010000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asBodyStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<BODY_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular brain type */
#[no_mangle]
pub unsafe extern "C" fn statsSetBrain(mut psStats: *mut BRAIN_STATS,
                                       mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid BRAIN_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2826 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"statsSetBrain\x00")).as_ptr(),
              b"((psStats)->ref >= (0x020000)) && ((psStats)->ref < (0x020000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asBrainStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular power type */
/*void statsSetPower(POWER_STATS	*psStats, UDWORD index)
{
	SET_STATS(psStats, asPowerStats, index, POWER_STATS, REF_POWER_START);
}*/
/* Set the stats for a particular power type */
#[no_mangle]
pub unsafe extern "C" fn statsSetPropulsion(mut psStats:
                                                *mut PROPULSION_STATS,
                                            mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid PROPULSION_STATS ref number\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2837 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"statsSetPropulsion\x00")).as_ptr(),
              b"((psStats)->ref >= (0x040000)) && ((psStats)->ref < (0x040000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asPropulsionStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular sensor type */
#[no_mangle]
pub unsafe extern "C" fn statsSetSensor(mut psStats: *mut SENSOR_STATS,
                                        mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid SENSOR_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2842 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsSetSensor\x00")).as_ptr(),
              b"((psStats)->ref >= (0x050000)) && ((psStats)->ref < (0x050000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asSensorStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular ecm type */
#[no_mangle]
pub unsafe extern "C" fn statsSetECM(mut psStats: *mut ECM_STATS,
                                     mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid ECM_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2847 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"statsSetECM\x00")).as_ptr(),
              b"((psStats)->ref >= (0x060000)) && ((psStats)->ref < (0x060000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asECMStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular repair type */
#[no_mangle]
pub unsafe extern "C" fn statsSetRepair(mut psStats: *mut REPAIR_STATS,
                                        mut index: UDWORD) {
    if (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid REPAIR_STATS ref number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2852 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsSetRepair\x00")).as_ptr(),
              b"((psStats)->ref >= (0x080000)) && ((psStats)->ref < (0x080000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asRepairStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong);
}
/* Set the stats for a particular program type */
/*void statsSetProgram(PROGRAM_STATS	*psStats, UDWORD index)
{
	SET_STATS(psStats, asProgramStats, index, PROGRAM_STATS, REF_PROGRAM_START);
}*/
/* Set the stats for a particular construct type */
#[no_mangle]
pub unsafe extern "C" fn statsSetConstruct(mut psStats: *mut CONSTRUCT_STATS,
                                           mut index: UDWORD) {
    if (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setStats: Invalid CONSTRUCT_STATS ref number\x00" as *const u8
                  as *const libc::c_char);
    };
    if (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2862 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"statsSetConstruct\x00")).as_ptr(),
              b"((psStats)->ref >= (0x0f0000)) && ((psStats)->ref < (0x0f0000) + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    memcpy(asConstructStats.offset(index as isize) as *mut libc::c_void,
           psStats as *const libc::c_void,
           ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong);
}
/* ******************************************************************************
*		Get stats functions
*******************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn statsGetWeapon(mut ref_0: UDWORD)
 -> *mut WEAPON_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetWeapon: Invalid reference number: %x\x00" as *const u8
                  as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2872 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetWeapon\x00")).as_ptr(),
              b"(ref >= REF_WEAPON_START) && (ref < REF_WEAPON_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numWeaponStats {
        if (*asWeaponStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asWeaponStats.offset(index as isize) as
                       *mut WEAPON_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetWeapon: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2881 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetWeapon\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut WEAPON_STATS;
    // should never get here, but this stops the compiler complaining.
}
/*ARMOUR_STATS *statsGetArmour(UDWORD ref)
{
	UDWORD index;
	ASSERT( (ref >= REF_ARMOUR_START) && (ref < REF_ARMOUR_START + REF_RANGE),
		"statsGetArmour: Invalid reference number: %x", ref );

	for (index = 0; index < numArmourStats; index++)
	{
		if (asArmourStats[index].ref == ref)
		{
			return &asArmourStats[index];
		}
	}
	ASSERT( FALSE, "statsGetArmour: Reference number not found in list: %x", ref );
}*/
#[no_mangle]
pub unsafe extern "C" fn statsGetBody(mut ref_0: UDWORD) -> *mut BODY_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetBody: Invalid reference number: %x\x00" as *const u8
                  as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2905 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"statsGetBody\x00")).as_ptr(),
              b"(ref >= REF_BODY_START) && (ref < REF_BODY_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numBodyStats {
        if (*asBodyStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asBodyStats.offset(index as isize) as *mut BODY_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetBody: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2914 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"statsGetBody\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut BODY_STATS;
    // should never get here, but this stops the compiler complaining.
}
#[no_mangle]
pub unsafe extern "C" fn statsGetBrain(mut ref_0: UDWORD)
 -> *mut BRAIN_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetBrain: Invalid reference number: %x\x00" as *const u8
                  as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2922 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"statsGetBrain\x00")).as_ptr(),
              b"(ref >= REF_BRAIN_START) && (ref < REF_BRAIN_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numBrainStats {
        if (*asBrainStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asBrainStats.offset(index as isize) as
                       *mut BRAIN_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetBrain: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2931 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"statsGetBrain\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut BRAIN_STATS;
    // should never get here, but this stops the compiler complaining.
}
/*POWER_STATS *statsGetPower(UDWORD ref)
{
	UDWORD index;
	ASSERT( (ref >= REF_POWER_START) && (ref < REF_POWER_START + REF_RANGE),
		"statsGetPower: Invalid reference number: %x", ref );

	for (index = 0; index < numPowerStats; index++)
	{
		if (asPowerStats[index].ref == ref)
		{
			return &asPowerStats[index];
		}
	}
	ASSERT( FALSE, "statsGetPower: Reference number not found in list: %x", ref );
}*/
#[no_mangle]
pub unsafe extern "C" fn statsGetPropulsion(mut ref_0: UDWORD)
 -> *mut PROPULSION_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetPropulsion: Invalid reference number: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2956 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"statsGetPropulsion\x00")).as_ptr(),
              b"(ref >= REF_PROPULSION_START) && (ref < REF_PROPULSION_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numPropulsionStats {
        if (*asPropulsionStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asPropulsionStats.offset(index as isize) as
                       *mut PROPULSION_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetPropulsion: Reference number not found in list: %x\x00"
                  as *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2965 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"statsGetPropulsion\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut PROPULSION_STATS;
    // should never get here, but this stops the compiler complaining.
}
#[no_mangle]
pub unsafe extern "C" fn statsGetSensor(mut ref_0: UDWORD)
 -> *mut SENSOR_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetSensor: Invalid reference number: %x\x00" as *const u8
                  as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2973 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetSensor\x00")).as_ptr(),
              b"(ref >= REF_SENSOR_START) && (ref < REF_SENSOR_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numSensorStats {
        if (*asSensorStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asSensorStats.offset(index as isize) as
                       *mut SENSOR_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetSensor: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2982 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetSensor\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut SENSOR_STATS;
    // should never get here, but this stops the compiler complaining.
}
#[no_mangle]
pub unsafe extern "C" fn statsGetECM(mut ref_0: UDWORD) -> *mut ECM_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetECM: Invalid reference number: %x\x00" as *const u8 as
                  *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2990 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"statsGetECM\x00")).as_ptr(),
              b"(ref >= REF_ECM_START) && (ref < REF_ECM_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numECMStats {
        if (*asECMStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asECMStats.offset(index as isize) as *mut ECM_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetECM: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              2999 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"statsGetECM\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut ECM_STATS;
    // should never get here, but this stops the compiler complaining.
}
#[no_mangle]
pub unsafe extern "C" fn statsGetRepair(mut ref_0: UDWORD)
 -> *mut REPAIR_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetRepair: Invalid reference number: %x\x00" as *const u8
                  as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3007 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetRepair\x00")).as_ptr(),
              b"(ref >= REF_REPAIR_START) && (ref < REF_REPAIR_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numRepairStats {
        if (*asRepairStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asRepairStats.offset(index as isize) as
                       *mut REPAIR_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetRepair: Reference number not found in list: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3016 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"statsGetRepair\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut REPAIR_STATS;
    // should never get here, but this stops the compiler complaining.
}
/*PROGRAM_STATS *statsGetProgram(UDWORD ref)
{
	UDWORD index;
	ASSERT( (ref >= REF_PROGRAM_START) && (ref < REF_PROGRAM_START + REF_RANGE),
		"statsGetProgram: Invalid reference number: %x", ref );

	for (index = 0; index < numProgramStats; index++)
	{
		if (asProgramStats[index].ref == ref)
		{
			return &asProgramStats[index];
		}
	}
	ASSERT( FALSE, "statsGetProgram: Reference number not found in list: %x", ref );
}*/
#[no_mangle]
pub unsafe extern "C" fn statsGetConstruct(mut ref_0: UDWORD)
 -> *mut CONSTRUCT_STATS {
    let mut index: UDWORD = 0;
    if ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"statsGetConstruct: Invalid reference number: %x\x00" as
                  *const u8 as *const libc::c_char, ref_0);
    };
    if ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3040 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"statsGetConstruct\x00")).as_ptr(),
              b"(ref >= REF_CONSTRUCT_START) && (ref < REF_CONSTRUCT_START + REF_RANGE)\x00"
                  as *const u8 as *const libc::c_char);
    };
    index = 0 as libc::c_int as UDWORD;
    while index < numConstructStats {
        if (*asConstructStats.offset(index as isize)).ref_0 == ref_0 {
            return &mut *asConstructStats.offset(index as isize) as
                       *mut CONSTRUCT_STATS
        }
        index = index.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"statsGetConstruct: Reference number not found in list: %x\x00"
                  as *const u8 as *const libc::c_char, ref_0);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3049 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"statsGetConstruct\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut CONSTRUCT_STATS;
    // should never get here, but this stops the compiler complaining.
}
/* **********************************************************************************
*	Dealloc the extra storage tables
***********************************************************************************/
//Deallocate the storage assigned for the Propulsion Types table
#[no_mangle]
pub unsafe extern "C" fn deallocPropulsionTypes() {
    //UBYTE inc;
//	PROPULSION_TYPES* pList = asPropulsionTypes;
    /*#ifndef HASH_NAMES
	for (inc=0; inc < numPropulsionTypes; inc++, pList++)
	{
		FREE(pList->pName);
	}
#endif*/
    memFreeRelease(asPropulsionTypes as *mut libc::c_void);
    asPropulsionTypes = 0 as *mut PROPULSION_TYPES;
}
//dealloc the storage assigned for the terrain table
#[no_mangle]
pub unsafe extern "C" fn deallocTerrainTable() {
    memFreeRelease(asTerrainTable as *mut libc::c_void);
    asTerrainTable = 0 as *mut TERRAIN_TABLE;
}
//dealloc the storage assigned for the Special Ability stats
#[no_mangle]
pub unsafe extern "C" fn deallocSpecialAbility() {
    let mut inc: UBYTE = 0;
    let mut pList: *mut SPECIAL_ABILITY = asSpecialAbility;
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_uint) < numSpecialAbility {
        memFreeRelease((*pList).pName as *mut libc::c_void);
        (*pList).pName = 0 as *mut STRING;
        inc = inc.wrapping_add(1);
        pList = pList.offset(1)
    }
    memFreeRelease(asSpecialAbility as *mut libc::c_void);
    asSpecialAbility = 0 as *mut SPECIAL_ABILITY;
}
//store the speed Factor in the terrain table
#[no_mangle]
pub unsafe extern "C" fn storeSpeedFactor(mut terrainType: UDWORD,
                                          mut propulsionType: UDWORD,
                                          mut speedFactor: UDWORD) {
    let mut pTerrainTable: *mut TERRAIN_TABLE = asTerrainTable;
    //ASSERT( propulsionType < numPropulsionTypes,
    if propulsionType < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"The propulsion type is too large\x00" as *const u8 as
                  *const libc::c_char);
    };
    if propulsionType < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3097 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"storeSpeedFactor\x00")).as_ptr(),
              b"propulsionType < NUM_PROP_TYPES\x00" as *const u8 as
                  *const libc::c_char);
    };
    //pTerrainTable += terrainType*numPropulsionTypes + propulsionType;
    pTerrainTable =
        pTerrainTable.offset(terrainType.wrapping_mul(NUM_PROP_TYPES as
                                                          libc::c_int as
                                                          libc::c_uint).wrapping_add(propulsionType)
                                 as isize);
    (*pTerrainTable).speedFactor = speedFactor;
}
//get the speed factor for a given terrain type and propulsion type
#[no_mangle]
pub unsafe extern "C" fn getSpeedFactor(mut type_0: UDWORD,
                                        mut propulsionType: UDWORD)
 -> UDWORD {
    //	UBYTE	type;
    let mut pTerrainTable: *mut TERRAIN_TABLE = asTerrainTable;
    //ASSERT( propulsionType < numPropulsionTypes,
    if propulsionType < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"The propulsion type is too large\x00" as *const u8 as
                  *const libc::c_char);
    };
    if propulsionType < NUM_PROP_TYPES as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3112 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"getSpeedFactor\x00")).as_ptr(),
              b"propulsionType < NUM_PROP_TYPES\x00" as *const u8 as
                  *const libc::c_char);
    };
    /*	switch (terrainType)
	{
		case TER_GRASS:
		{
			type = 0;
			break;
		}
		case TER_STONE:
		{
			type = 1;
			break;
		}
		case TER_SAND:
		{
			type = 2;
			break;
		}
		case TER_WATER:
		{
			type = 3;
			break;
		}
		default:
		{
			ASSERT( FALSE, "The terrain type is unknown" );
			return 0;
		}
	 }*/
	//pTerrainTable += type*numPropulsionTypes + propulsionType;
    pTerrainTable =
        pTerrainTable.offset(type_0.wrapping_mul(NUM_PROP_TYPES as libc::c_int
                                                     as
                                                     libc::c_uint).wrapping_add(propulsionType)
                                 as isize);
    return (*pTerrainTable).speedFactor;
}
//return the type of stat this stat is!
#[no_mangle]
pub unsafe extern "C" fn statType(mut ref_0: UDWORD) -> UDWORD {
    if ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x10000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_BODY as libc::c_int as UDWORD
    }
    if ref_0 >= 0x20000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x20000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_BRAIN as libc::c_int as UDWORD
    }
    /*if (ref >REF_POWER_START && ref < REF_POWER_START +
		REF_RANGE)
	{
		return COMP_POWERPLANT;
	}*/
    if ref_0 >= 0x40000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x40000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_PROPULSION as libc::c_int as UDWORD
    }
    if ref_0 >= 0x50000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x50000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_SENSOR as libc::c_int as UDWORD
    }
    if ref_0 >= 0x60000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x60000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_ECM as libc::c_int as UDWORD
    }
    /*if (ref >REF_ARMOUR_START && ref < REF_ARMOUR_START +
		REF_RANGE)
	{
		return COMP_ARMOUR;
	}*/
    if ref_0 >= 0x80000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0x80000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_REPAIRUNIT as libc::c_int as UDWORD
    }
    /*if (ref >=REF_PROGRAM_START && ref < REF_PROGRAM_START +
		REF_RANGE)
	{
		return COMP_PROGRAM;
	}*/
    if ref_0 >= 0xa0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_WEAPON as libc::c_int as UDWORD
    }
    if ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
           ref_0 <
               (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        return COMP_CONSTRUCT as libc::c_int as UDWORD
    }
    //else
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Invalid stat pointer - cannot determine Stat Type\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3210 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"statType\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return COMP_UNKNOWN as libc::c_int as UDWORD;
}
//return the REF_START value of this type of stat
#[no_mangle]
pub unsafe extern "C" fn statRefStart(mut stat: UDWORD) -> UDWORD {
    let mut start: UDWORD = 0;
    match stat {
        1 => { start = 0x10000 as libc::c_int as UDWORD }
        2 => { start = 0x20000 as libc::c_int as UDWORD }
        3 => {
            /*case COMP_POWERPLANT:
		{
			start = REF_POWER_START;
			break;
		}*/
            start = 0x40000 as libc::c_int as UDWORD
        }
        6 => { start = 0x50000 as libc::c_int as UDWORD }
        5 => { start = 0x60000 as libc::c_int as UDWORD }
        4 => {
            /*case COMP_ARMOUR:
		{
			start = REF_ARMOUR_START;
			break;
		}*/
            start = 0x80000 as libc::c_int as UDWORD
        }
        8 => { start = 0xa0000 as libc::c_int as UDWORD }
        7 => {
            /*case COMP_PROGRAM:
		{
			start = REF_PROGRAM_START;
			break;
		}*/
            start = 0xf0000 as libc::c_int as UDWORD
        }
        _ => {
            //COMP_UNKNOWN should be an error
            debug(LOG_ERROR,
                  b"Invalid stat type\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    return start;
}
/*Returns the component type based on the string - used for reading in data */
#[no_mangle]
pub unsafe extern "C" fn componentType(mut pType: *mut libc::c_char)
 -> UDWORD {
    if strcmp(pType, b"BODY\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_BODY as libc::c_int as UDWORD
    }
    if strcmp(pType, b"PROPULSION\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return COMP_PROPULSION as libc::c_int as UDWORD
    }
    if strcmp(pType, b"BRAIN\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_BRAIN as libc::c_int as UDWORD
    }
    if strcmp(pType, b"REPAIR\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_REPAIRUNIT as libc::c_int as UDWORD
    }
    if strcmp(pType, b"ECM\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_ECM as libc::c_int as UDWORD
    }
    if strcmp(pType, b"SENSOR\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_SENSOR as libc::c_int as UDWORD
    }
    /*if (!strcmp(pType,"PROGRAM"))
	{
		return COMP_PROGRAM;
	}*/
    if strcmp(pType, b"WEAPON\x00" as *const u8 as *const libc::c_char) == 0 {
        return COMP_WEAPON as libc::c_int as UDWORD
    }
    if strcmp(pType, b"CONSTRUCT\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return COMP_CONSTRUCT as libc::c_int as UDWORD
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Unknown Component Type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3327 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"componentType\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int as UDWORD;
    // Should never get here.
}
//function to compare a value with yes/no - if neither warns player!
unsafe extern "C" fn compareYes(mut strToCompare: *mut STRING,
                                mut strOwner: *mut STRING) -> BOOL {
    if strcmp(strToCompare, b"YES\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return 1 as libc::c_int
    } else if strcmp(strToCompare,
                     b"NO\x00" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int
    } else {
        //set default to FALSE but continue
		//DBERROR(("Invalid yes/no for record %s", strOwner));
        debug(LOG_ERROR,
              b"Invalid yes/no for record %s\x00" as *const u8 as
                  *const libc::c_char, getName(strOwner));
        abort();
    };
}
//get the component Inc for a stat based on the Resource name and type
//returns -1 if record not found
//used in Scripts
#[no_mangle]
pub unsafe extern "C" fn getCompFromResName(mut compType: UDWORD,
                                            mut pName: *mut STRING)
 -> SDWORD {
    return getCompFromName(compType, pName);
}
#[no_mangle]
pub unsafe extern "C" fn getStatsDetails(mut compType: UDWORD,
                                         mut ppsStats: *mut *mut BASE_STATS,
                                         mut pnumStats: *mut UDWORD,
                                         mut pstatSize: *mut UDWORD) {
    match compType {
        1 => {
            *ppsStats = asBodyStats as *mut BASE_STATS;
            *pnumStats = numBodyStats;
            *pstatSize = ::std::mem::size_of::<BODY_STATS>() as libc::c_ulong
        }
        2 => {
            *ppsStats = asBrainStats as *mut BASE_STATS;
            *pnumStats = numBrainStats;
            *pstatSize = ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong
        }
        3 => {
            *ppsStats = asPropulsionStats as *mut BASE_STATS;
            *pnumStats = numPropulsionStats;
            *pstatSize =
                ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong
        }
        4 => {
            *ppsStats = asRepairStats as *mut BASE_STATS;
            *pnumStats = numRepairStats;
            *pstatSize =
                ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong
        }
        5 => {
            *ppsStats = asECMStats as *mut BASE_STATS;
            *pnumStats = numECMStats;
            *pstatSize = ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong
        }
        6 => {
            *ppsStats = asSensorStats as *mut BASE_STATS;
            *pnumStats = numSensorStats;
            *pstatSize =
                ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong
        }
        7 => {
            *ppsStats = asConstructStats as *mut BASE_STATS;
            *pnumStats = numConstructStats;
            *pstatSize =
                ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong
        }
        8 => {
            /*case COMP_PROGRAM:
		psStats = (BASE_STATS*)asProgramStats;
		numStats = numProgramStats;
		statSize = sizeof(PROGRAM_STATS);
		break;*/
            *ppsStats = asWeaponStats as *mut BASE_STATS;
            *pnumStats = numWeaponStats;
            *pstatSize =
                ::std::mem::size_of::<WEAPON_STATS>() as libc::c_ulong
        }
        _ => {
            //COMP_UNKNOWN should be an error
            debug(LOG_ERROR,
                  b"Invalid component type - game.c\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    };
}
//get the component Inc for a stat based on the name and type
//returns -1 if record not found
#[no_mangle]
pub unsafe extern "C" fn getCompFromName(mut compType: UDWORD,
                                         mut pName: *mut STRING) -> SDWORD {
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut numStats: UDWORD = 0 as libc::c_int as UDWORD;
    let mut count: UDWORD = 0;
    let mut statSize: UDWORD = 0 as libc::c_int as UDWORD;
    //	DBPRINTF(("getcompfromname type=%d name=[%s]\n",compType,pName));
    getStatsDetails(compType, &mut psStats, &mut numStats, &mut statSize);
    //find the stat with the same name
    count = 0 as libc::c_int as UDWORD;
    while count < numStats {
        //DBPRINTF(("%x ",psStats->NameHash);
        if strcmp(pName, (*psStats).pName) == 0 {
            //			DBPRINTF(("found at %d\n",count));
            return count as SDWORD
        }
        psStats =
            (psStats as UDWORD).wrapping_add(statSize) as *mut BASE_STATS;
        count = count.wrapping_add(1)
    }
    //	DBPRINTF(("not found\n"));
	//return -1 if record not found or an invalid component type is passed in
    return -(1 as libc::c_int);
}
//converts the name read in from Access into the name which is used in the Stat lists
#[no_mangle]
pub unsafe extern "C" fn getResourceName(mut pName: *mut STRING) -> BOOL {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getNameFromStat(mut pStat: *mut BASE_STATS)
 -> *mut STRING {
    return getName((*pStat).pName);
}
/*return the name to display for the interface - valid for OBJECTS and STATS*/
#[no_mangle]
pub unsafe extern "C" fn getName(mut pNameID: *mut STRING) -> *mut STRING {
    let mut id: UDWORD = 0;
    let mut pName: *mut STRING = 0 as *mut STRING;
    static mut Unknown: [STRING; 13] =
        [78, 97, 109, 101, 32, 85, 110, 107, 110, 111, 119, 110, 0];
    /*see if the name has a resource associated with it by trying to get
	the ID for the string*/
    if strresGetIDNum(psStringRes, pNameID, &mut id) == 0 {
        debug(LOG_ERROR,
              b"Unable to find string resource for %s\x00" as *const u8 as
                  *const libc::c_char, pNameID);
        abort();
    }
    //get the string from the id
    pName = strresGetString(psStringRes, id);
    if !pName.is_null() { return pName } else { return Unknown.as_mut_ptr() };
}
/*sets the tech level for the stat passed in - returns TRUE if set OK*/
#[no_mangle]
pub unsafe extern "C" fn setTechLevel(mut psStats: *mut BASE_STATS,
                                      mut pLevel: *mut STRING) -> BOOL {
    let mut techLevel: TECH_LEVEL = MAX_TECH_LEVELS;
    if strcmp(pLevel, b"Level One\x00" as *const u8 as *const libc::c_char) ==
           0 {
        techLevel = TECH_LEVEL_ONE
    } else if strcmp(pLevel,
                     b"Level Two\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        techLevel = TECH_LEVEL_TWO
    } else if strcmp(pLevel,
                     b"Level Three\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        techLevel = TECH_LEVEL_THREE
    } else if strcmp(pLevel,
                     b"Level One-Two\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        techLevel = TECH_LEVEL_ONE_TWO
    } else if strcmp(pLevel,
                     b"Level Two-Three\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        techLevel = TECH_LEVEL_TWO_THREE
    } else if strcmp(pLevel,
                     b"Level All\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        techLevel = TECH_LEVEL_ALL
    } else if strcmp(pLevel,
                     b"Don\'t Display\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        techLevel = MAX_TECH_LEVELS
    } else {
        //invalid tech level passed in
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Unknown Technology Level - %s\x00" as *const u8 as
                      *const libc::c_char, pLevel);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stats.c\x00" as *const u8 as *const libc::c_char,
                  3590 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"setTechLevel\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //store tech level in the appropriate stat
    if (*psStats).ref_0 >= 0x10000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <=
               (0xa0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint ||
           (*psStats).ref_0 >= 0xf0000 as libc::c_int as libc::c_uint &&
               (*psStats).ref_0 <=
                   (0xf0000 as libc::c_int + 0x10000 as libc::c_int) as
                       libc::c_uint {
        (*(psStats as *mut COMP_BASE_STATS)).techLevel = techLevel
    } else if (*psStats).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <=
                      (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        (*(psStats as *mut STRUCTURE_STATS)).techLevel = techLevel
    } else if (*psStats).ref_0 >= 0xb0000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <=
                      (0xb0000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        (*(psStats as *mut RESEARCH)).techLevel = techLevel
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Invalid stat id for %s\x00" as *const u8 as
                      *const libc::c_char, (*psStats).pName);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"stats.c\x00" as *const u8 as *const libc::c_char,
                  3616 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"setTechLevel\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*sets the store to the body size based on the name passed in - returns FALSE
if doesn't compare with any*/
#[no_mangle]
pub unsafe extern "C" fn getBodySize(mut pSize: *mut STRING,
                                     mut pStore: *mut UBYTE) -> BOOL {
    if strcmp(pSize, b"LIGHT\x00" as *const u8 as *const libc::c_char) == 0 {
        *pStore = SIZE_LIGHT as libc::c_int as UBYTE
    } else if strcmp(pSize, b"MEDIUM\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        *pStore = SIZE_MEDIUM as libc::c_int as UBYTE
    } else if strcmp(pSize, b"HEAVY\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        *pStore = SIZE_HEAVY as libc::c_int as UBYTE
    } else if strcmp(pSize,
                     b"SUPER HEAVY\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        *pStore = SIZE_SUPER_HEAVY as libc::c_int as UBYTE
    } else { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*returns the weapon sub class based on the string name passed in */
#[no_mangle]
pub unsafe extern "C" fn getWeaponSubClass(mut pSubClass: *mut STRING)
 -> SDWORD {
    if strcmp(pSubClass, b"CANNON\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return WSC_CANNON as libc::c_int
    }
    /*if (!strcmp(pSubClass,"ARTILLARY"))
	{
		return WSC_ARTILLARY;
	}*/
    if strcmp(pSubClass, b"MORTARS\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_MORTARS as libc::c_int
    }
    if strcmp(pSubClass, b"MISSILE\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_MISSILE as libc::c_int
    }
    if strcmp(pSubClass, b"ROCKET\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return WSC_ROCKET as libc::c_int
    }
    if strcmp(pSubClass, b"ENERGY\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return WSC_ENERGY as libc::c_int
    }
    if strcmp(pSubClass, b"GAUSS\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return WSC_GAUSS as libc::c_int
    }
    if strcmp(pSubClass, b"FLAME\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return WSC_FLAME as libc::c_int
    }
    /*if (!strcmp(pSubClass,"CLOSECOMBAT"))
	{
		return WSC_CLOSECOMBAT;
	}*/
    if strcmp(pSubClass, b"HOWITZERS\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_HOWITZERS as libc::c_int
    }
    if strcmp(pSubClass,
              b"MACHINE GUN\x00" as *const u8 as *const libc::c_char) == 0 {
        return WSC_MGUN as libc::c_int
    }
    if strcmp(pSubClass,
              b"ELECTRONIC\x00" as *const u8 as *const libc::c_char) == 0 {
        return WSC_ELECTRONIC as libc::c_int
    }
    if strcmp(pSubClass, b"A-A GUN\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_AAGUN as libc::c_int
    }
    if strcmp(pSubClass,
              b"SLOW MISSILE\x00" as *const u8 as *const libc::c_char) == 0 {
        return WSC_SLOWMISSILE as libc::c_int
    }
    if strcmp(pSubClass,
              b"SLOW ROCKET\x00" as *const u8 as *const libc::c_char) == 0 {
        return WSC_SLOWROCKET as libc::c_int
    }
    if strcmp(pSubClass, b"LAS_SAT\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_LAS_SAT as libc::c_int
    }
    if strcmp(pSubClass, b"BOMB\x00" as *const u8 as *const libc::c_char) == 0
       {
        return WSC_BOMB as libc::c_int
    }
    if strcmp(pSubClass, b"COMMAND\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return WSC_COMMAND as libc::c_int
    }
    if strcmp(pSubClass, b"EMP\x00" as *const u8 as *const libc::c_char) == 0
       {
        return WSC_EMP as libc::c_int
    }
    //problem if we've got to here
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Invalid weapon sub class - %s\x00" as *const u8 as
                  *const libc::c_char, pSubClass);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3731 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"getWeaponSubClass\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return NUM_WEAPON_SUBCLASS as libc::c_int + 1 as libc::c_int;
}
/*returns the movement model based on the string name passed in */
unsafe extern "C" fn getMovementModel(mut pMovement: *mut STRING) -> SDWORD {
    if strcmp(pMovement, b"DIRECT\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return MM_DIRECT as libc::c_int
    }
    if strcmp(pMovement, b"INDIRECT\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return MM_INDIRECT as libc::c_int
    }
    if strcmp(pMovement,
              b"HOMING-DIRECT\x00" as *const u8 as *const libc::c_char) == 0 {
        return MM_HOMINGDIRECT as libc::c_int
    }
    if strcmp(pMovement,
              b"HOMING-INDIRECT\x00" as *const u8 as *const libc::c_char) == 0
       {
        return MM_HOMINGINDIRECT as libc::c_int
    }
    if strcmp(pMovement,
              b"ERRATIC-DIRECT\x00" as *const u8 as *const libc::c_char) == 0
       {
        return MM_ERRATICDIRECT as libc::c_int
    }
    if strcmp(pMovement, b"SWEEP\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return MM_SWEEP as libc::c_int
    }
    //problem if we've got to here
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Invalid movement model %s\x00" as *const u8 as
                  *const libc::c_char, pMovement);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3763 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"getMovementModel\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return NUM_MOVEMENT_MODEL as libc::c_int + 1 as libc::c_int;
}
/*returns the weapon effect based on the string name passed in */
#[no_mangle]
pub unsafe extern "C" fn getWeaponEffect(mut pWeaponEffect: *mut STRING)
 -> UBYTE {
    if strcmp(pWeaponEffect,
              b"ANTI PERSONNEL\x00" as *const u8 as *const libc::c_char) == 0
       {
        return WE_ANTI_PERSONNEL as libc::c_int as UBYTE
    } else if strcmp(pWeaponEffect,
                     b"ANTI TANK\x00" as *const u8 as *const libc::c_char) ==
                  0 {
        return WE_ANTI_TANK as libc::c_int as UBYTE
    } else if strcmp(pWeaponEffect,
                     b"BUNKER BUSTER\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        return WE_BUNKER_BUSTER as libc::c_int as UBYTE
    } else if strcmp(pWeaponEffect,
                     b"ARTILLERY ROUND\x00" as *const u8 as
                         *const libc::c_char) == 0 {
        return WE_ARTILLERY_ROUND as libc::c_int as UBYTE
    } else if strcmp(pWeaponEffect,
                     b"FLAMER\x00" as *const u8 as *const libc::c_char) == 0 {
        return WE_FLAMER as libc::c_int as UBYTE
    } else if strcmp(pWeaponEffect,
                     b"ANTI AIRCRAFT\x00" as *const u8 as *const libc::c_char)
                  == 0 {
        return WE_ANTI_AIRCRAFT as libc::c_int as UBYTE
    } else {
        return (WE_NUMEFFECTS as libc::c_int + 1 as libc::c_int) as UBYTE
    };
}
// don't allocate name
/*
looks up the name to get the resource associated with it - or allocates space
and stores the name. Eventually ALL names will be 'resourced' for translation
*/
#[no_mangle]
pub unsafe extern "C" fn allocateName(mut ppStore: *mut *mut STRING,
                                      mut pName: *mut STRING) -> BOOL {
    //checks the name has been loaded as a resource and gets the storage pointer
    if strresGetIDString(psStringRes, pName, ppStore) == 0 {
        debug(LOG_ERROR,
              b"Unable to find string resource for %s\x00" as *const u8 as
                  *const libc::c_char, pName);
        abort();
    }
    return 1 as libc::c_int;
}
/*Access functions for the upgradeable stats of a weapon*/
#[no_mangle]
pub unsafe extern "C" fn weaponFirePause(mut psStats: *mut WEAPON_STATS,
                                         mut player: UBYTE) -> UDWORD {
    return (*psStats).firePause.wrapping_sub((*psStats).firePause.wrapping_mul(asWeaponUpgrade[player
                                                                                                   as
                                                                                                   usize][(*psStats).weaponSubClass
                                                                                                              as
                                                                                                              usize].firePause
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(100
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn weaponShortHit(mut psStats: *mut WEAPON_STATS,
                                        mut player: UBYTE) -> UDWORD {
    return (*psStats).shortHit.wrapping_add((*psStats).shortHit.wrapping_mul(asWeaponUpgrade[player
                                                                                                 as
                                                                                                 usize][(*psStats).weaponSubClass
                                                                                                            as
                                                                                                            usize].shortHit
                                                                                 as
                                                                                 libc::c_uint).wrapping_div(100
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn weaponLongHit(mut psStats: *mut WEAPON_STATS,
                                       mut player: UBYTE) -> UDWORD {
    return (*psStats).longHit.wrapping_add((*psStats).longHit.wrapping_mul(asWeaponUpgrade[player
                                                                                               as
                                                                                               usize][(*psStats).weaponSubClass
                                                                                                          as
                                                                                                          usize].longHit
                                                                               as
                                                                               libc::c_uint).wrapping_div(100
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn weaponDamage(mut psStats: *mut WEAPON_STATS,
                                      mut player: UBYTE) -> UDWORD {
    return (*psStats).damage.wrapping_add((*psStats).damage.wrapping_mul(asWeaponUpgrade[player
                                                                                             as
                                                                                             usize][(*psStats).weaponSubClass
                                                                                                        as
                                                                                                        usize].damage
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint));
}
/*	Use this one if everything starts blowing up with suspect area damage
UDWORD	weaponRadDamage(WEAPON_STATS *psStats, UBYTE player)
{
	//---
UDWORD	val;
	//---
	val =  (psStats->radiusDamage + (psStats->radiusDamage * asWeaponUpgrade[player][
		psStats->weaponSubClass].radiusDamage)/100);
	ASSERT( val<512,"Big range!!!!" );
	return(val);
}
*/
#[no_mangle]
pub unsafe extern "C" fn weaponRadDamage(mut psStats: *mut WEAPON_STATS,
                                         mut player: UBYTE) -> UDWORD {
    return (*psStats).radiusDamage.wrapping_add((*psStats).radiusDamage.wrapping_mul(asWeaponUpgrade[player
                                                                                                         as
                                                                                                         usize][(*psStats).weaponSubClass
                                                                                                                    as
                                                                                                                    usize].radiusDamage
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(100
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn weaponIncenDamage(mut psStats: *mut WEAPON_STATS,
                                           mut player: UBYTE) -> UDWORD {
    return (*psStats).incenDamage.wrapping_add((*psStats).incenDamage.wrapping_mul(asWeaponUpgrade[player
                                                                                                       as
                                                                                                       usize][(*psStats).weaponSubClass
                                                                                                                  as
                                                                                                                  usize].incenDamage
                                                                                       as
                                                                                       libc::c_uint).wrapping_div(100
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn weaponRadiusHit(mut psStats: *mut WEAPON_STATS,
                                         mut player: UBYTE) -> UDWORD {
    return (*psStats).radiusHit.wrapping_add((*psStats).radiusHit.wrapping_mul(asWeaponUpgrade[player
                                                                                                   as
                                                                                                   usize][(*psStats).weaponSubClass
                                                                                                              as
                                                                                                              usize].radiusHit
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(100
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint));
}
/*Access functions for the upgradeable stats of a sensor*/
#[no_mangle]
pub unsafe extern "C" fn sensorPower(mut psStats: *mut SENSOR_STATS,
                                     mut player: UBYTE) -> UDWORD {
    return (*psStats).power.wrapping_add((*psStats).power.wrapping_mul(asSensorUpgrade[player
                                                                                           as
                                                                                           usize].power
                                                                           as
                                                                           libc::c_uint).wrapping_div(100
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint))
               as UWORD as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn sensorRange(mut psStats: *mut SENSOR_STATS,
                                     mut player: UBYTE) -> UDWORD {
    return (*psStats).range.wrapping_add((*psStats).range.wrapping_mul(asSensorUpgrade[player
                                                                                           as
                                                                                           usize].range
                                                                           as
                                                                           libc::c_uint).wrapping_div(100
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint))
               as UWORD as UDWORD;
}
/*Access functions for the upgradeable stats of a ECM*/
#[no_mangle]
pub unsafe extern "C" fn ecmPower(mut psStats: *mut ECM_STATS,
                                  mut player: UBYTE) -> UDWORD {
    return (*psStats).power.wrapping_add((*psStats).power.wrapping_mul(asECMUpgrade[player
                                                                                        as
                                                                                        usize].power
                                                                           as
                                                                           libc::c_uint).wrapping_div(100
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint))
               as UWORD as UDWORD;
}
/*Access functions for the upgradeable stats of a repair*/
#[no_mangle]
pub unsafe extern "C" fn repairPoints(mut psStats: *mut REPAIR_STATS,
                                      mut player: UBYTE) -> UDWORD {
    return (*psStats).repairPoints.wrapping_add((*psStats).repairPoints.wrapping_mul(asRepairUpgrade[player
                                                                                                         as
                                                                                                         usize].repairPoints
                                                                                         as
                                                                                         libc::c_uint).wrapping_div(100
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint));
}
/*Access functions for the upgradeable stats of a constructor*/
#[no_mangle]
pub unsafe extern "C" fn constructorPoints(mut psStats: *mut CONSTRUCT_STATS,
                                           mut player: UBYTE) -> UDWORD {
    return (*psStats).constructPoints.wrapping_add((*psStats).constructPoints.wrapping_mul(asConstUpgrade[player
                                                                                                              as
                                                                                                              usize].constructPoints
                                                                                               as
                                                                                               libc::c_uint).wrapping_div(100
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint));
}
/*Access functions for the upgradeable stats of a body*/
#[no_mangle]
pub unsafe extern "C" fn bodyPower(mut psStats: *mut BODY_STATS,
                                   mut player: UBYTE, mut bodyType: UBYTE)
 -> UDWORD {
    return (*psStats).powerOutput.wrapping_add((*psStats).powerOutput.wrapping_mul(asBodyUpgrade[player
                                                                                                     as
                                                                                                     usize][bodyType
                                                                                                                as
                                                                                                                usize].powerOutput
                                                                                       as
                                                                                       libc::c_uint).wrapping_div(100
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn bodyArmour(mut psStats: *mut BODY_STATS,
                                    mut player: UBYTE, mut bodyType: UBYTE,
                                    mut weaponClass: WEAPON_CLASS) -> UDWORD {
    match weaponClass as libc::c_uint {
        0 => {
            //case WC_EXPLOSIVE:
            return (*psStats).armourValue[WC_KINETIC as libc::c_int as
                                              usize].wrapping_add((*psStats).armourValue[WC_KINETIC
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize].wrapping_mul(asBodyUpgrade[player
                                                                                                                                   as
                                                                                                                                   usize][bodyType
                                                                                                                                              as
                                                                                                                                              usize].armourValue[WC_KINETIC
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     usize]
                                                                                                                     as
                                                                                                                     libc::c_uint).wrapping_div(100
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint))
        }
        1 => {
            //case WC_MISC:
            return (*psStats).armourValue[WC_HEAT as libc::c_int as
                                              usize].wrapping_add((*psStats).armourValue[WC_HEAT
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             usize].wrapping_mul(asBodyUpgrade[player
                                                                                                                                   as
                                                                                                                                   usize][bodyType
                                                                                                                                              as
                                                                                                                                              usize].armourValue[WC_HEAT
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     usize]
                                                                                                                     as
                                                                                                                     libc::c_uint).wrapping_div(100
                                                                                                                                                    as
                                                                                                                                                    libc::c_int
                                                                                                                                                    as
                                                                                                                                                    libc::c_uint))
        }
        _ => { }
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"bodyArmour() : Unknown weapon class\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"stats.c\x00" as *const u8 as *const libc::c_char,
              3970 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"bodyArmour\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int as UDWORD;
    // Should never get here.
}
//Access functions for the max values to be used in the Design Screen
//Access functions for the max values to be used in the Design Screen
unsafe extern "C" fn setMaxComponentWeight(mut weight: UDWORD) {
    if weight > maxComponentWeight { maxComponentWeight = weight };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxComponentWeight() -> UDWORD {
    return maxComponentWeight;
}
unsafe extern "C" fn setMaxBodyArmour(mut armour: UDWORD) {
    if armour > maxBodyArmour { maxBodyArmour = armour };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxBodyArmour() -> UDWORD {
    return maxBodyArmour;
}
unsafe extern "C" fn setMaxBodyPower(mut power: UDWORD) {
    if power > maxBodyPower { maxBodyPower = power };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxBodyPower() -> UDWORD { return maxBodyPower; }
unsafe extern "C" fn setMaxBodyPoints(mut points: UDWORD) {
    if points > maxBodyPoints { maxBodyPoints = points };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxBodyPoints() -> UDWORD {
    return maxBodyPoints;
}
unsafe extern "C" fn setMaxSensorRange(mut range: UDWORD) {
    if range > maxSensorRange { maxSensorRange = range };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxSensorRange() -> UDWORD {
    return maxSensorRange;
}
unsafe extern "C" fn setMaxSensorPower(mut power: UDWORD) {
    if power > maxSensorPower { maxSensorPower = power };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxSensorPower() -> UDWORD {
    return maxSensorPower;
}
unsafe extern "C" fn setMaxECMPower(mut power: UDWORD) {
    if power > maxECMPower { maxECMPower = power };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxECMPower() -> UDWORD { return maxECMPower; }
unsafe extern "C" fn setMaxConstPoints(mut points: UDWORD) {
    if points > maxConstPoints { maxConstPoints = points };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxConstPoints() -> UDWORD {
    return maxConstPoints;
}
unsafe extern "C" fn setMaxRepairPoints(mut repair: UDWORD) {
    if repair > maxRepairPoints { maxRepairPoints = repair };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxRepairPoints() -> UDWORD {
    return maxRepairPoints;
}
unsafe extern "C" fn setMaxWeaponRange(mut range: UDWORD) {
    if range > maxWeaponRange { maxWeaponRange = range };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxWeaponRange() -> UDWORD {
    return maxWeaponRange;
}
unsafe extern "C" fn setMaxWeaponDamage(mut damage: UDWORD) {
    if damage > maxWeaponDamage { maxWeaponDamage = damage };
}
#[no_mangle]
pub unsafe extern "C" fn getMaxWeaponDamage() -> UDWORD {
    return maxWeaponDamage;
}
unsafe extern "C" fn setMaxPropulsionSpeed(mut speed: UDWORD) {
    if speed > maxPropulsionSpeed { maxPropulsionSpeed = speed };
}
//Access functions for the max values to be used in the Design Screen
#[no_mangle]
pub unsafe extern "C" fn getMaxPropulsionSpeed() -> UDWORD {
    return maxPropulsionSpeed;
}
//determine the effect this upgrade would have on the max values
//determine the effect this upgrade would have on the max values
unsafe extern "C" fn updateMaxWeaponStats(mut maxValue: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxWeaponDamage();
    if currentMaxValue <
           currentMaxValue.wrapping_add((maxValue as libc::c_int /
                                             100 as libc::c_int) as
                                            libc::c_uint) {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxWeaponDamage(currentMaxValue);
    };
    //the fire pause is dealt with differently
}
unsafe extern "C" fn updateMaxSensorStats(mut maxRange: UWORD,
                                          mut maxPower: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxSensorRange();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxRange
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxRange
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxSensorRange(currentMaxValue);
    }
    currentMaxValue = getMaxSensorPower();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxPower
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxPower
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxSensorPower(currentMaxValue);
    };
}
unsafe extern "C" fn updateMaxRepairStats(mut maxValue: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxRepairPoints();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxRepairPoints(currentMaxValue);
    };
}
unsafe extern "C" fn updateMaxECMStats(mut maxValue: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxECMPower();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxECMPower(currentMaxValue);
    };
}
unsafe extern "C" fn updateMaxBodyStats(mut maxBody: UWORD,
                                        mut maxPower: UWORD,
                                        mut maxArmour: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxBodyPoints();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxBody
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxBody
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxBodyPoints(currentMaxValue);
    }
    currentMaxValue = getMaxBodyPower();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxPower
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxPower
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxBodyPower(currentMaxValue);
    }
    currentMaxValue = getMaxBodyArmour();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxArmour
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxArmour
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxBodyArmour(currentMaxValue);
    };
}
unsafe extern "C" fn updateMaxConstStats(mut maxValue: UWORD) {
    let mut currentMaxValue: UDWORD = getMaxConstPoints();
    if currentMaxValue <
           currentMaxValue.wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                         as
                                                                         libc::c_uint).wrapping_div(100
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint))
       {
        currentMaxValue =
            (currentMaxValue as
                 libc::c_uint).wrapping_add(currentMaxValue.wrapping_mul(maxValue
                                                                             as
                                                                             libc::c_uint).wrapping_div(100
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint))
                as UDWORD as UDWORD;
        setMaxConstPoints(currentMaxValue);
    };
}
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
//extern POWER_STATS			*asPowerStats;
//extern ARMOUR_STATS			*asArmourStats;
//extern PROGRAM_STATS		*asProgramStats;
//used to hold the modifiers cross refd by weapon effect and propulsion type
//used to hold the current upgrade level per player per weapon subclass
//body upgrades are possible for droids and/or cyborgs
/* The number of different stats stored */
//extern UDWORD		numPowerStats;
//extern UDWORD		numArmourStats;
//extern UDWORD		numPropulsionTypes;
/* What number the ref numbers start at for each type of stat */
//#define REF_POWER_START			0x030000
//#define REF_ARMOUR_START		0x070000
//#define REF_PROGRAM_START		0x090000
/* The maximum number of refs for a type of stat */
//stores for each players component states - see below
//store for each players Structure states
//flags to fill apCompLists and apStructTypeLists
//this item can be used to design droids
//the player does not know about this item
//this item has been found, but is unresearched
/* ******************************************************************************
*		Allocate stats functions
*******************************************************************************/
/* Allocate Weapon stats */
/*Allocate Armour stats*/
//extern BOOL statsAllocArmour(UDWORD numEntries);
/*Allocate Body stats*/
/*Allocate Brain stats*/
/*Allocate Power stats*/
//extern BOOL statsAllocPower(UDWORD numEntries);
/*Allocate Propulsion stats*/
/*Allocate Sensor stats*/
/*Allocate Ecm Stats*/
/*Allocate Repair Stats*/
/*Allocate Program Stats*/
/*Allocate Construct Stats*/
/* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
/*Load the weapon stats from the file exported from Access*/
/*Load the armour stats from the file exported from Access*/
//extern BOOL loadArmourStats(void);
/*Load the body stats from the file exported from Access*/
/*Load the brain stats from the file exported from Access*/
/*Load the power stats from the file exported from Access*/
//extern BOOL loadPowerStats(void);
/*Load the propulsion stats from the file exported from Access*/
/*Load the sensor stats from the file exported from Access*/
/*Load the ecm stats from the file exported from Access*/
/*Load the repair stats from the file exported from Access*/
/*Load the program stats from the file exported from Access*/
/*Load the construct stats from the file exported from Access*/
/*Load the Propulsion Types from the file exported from Access*/
/*Load the propulsion sounds from the file exported from Access*/
/*Load the Terrain Table from the file exported from Access*/
/*Load the Special Ability stats from the file exported from Access*/
/* load the IMDs to use for each body-propulsion combination */
/*Load the weapon sounds from the file exported from Access*/
/*Load the Weapon Effect Modifiers from the file exported from Access*/
/* ******************************************************************************
*		Set stats functions
*******************************************************************************/
/* Set the stats for a particular weapon type
 * The function uses the ref number in the stats structure to
 * index the correct array entry
 */
/*Set the stats for a particular armour type*/
//extern void statsSetArmour(ARMOUR_STATS	*psStats, UDWORD index);
/*Set the stats for a particular body type*/
/*Set the stats for a particular brain type*/
/*Set the stats for a particular power type*/
//extern void statsSetPower(POWER_STATS	*psStats, UDWORD index);
/*Set the stats for a particular propulsion type*/
/*Set the stats for a particular sensor type*/
/*Set the stats for a particular ecm type*/
/*Set the stats for a particular repair type*/
/*Set the stats for a particular program type*/
//extern void statsSetProgram(PROGRAM_STATS	*psStats, UDWORD index);
/*Set the stats for a particular construct type*/
/* ******************************************************************************
*		Get stats functions
*******************************************************************************/
//extern ARMOUR_STATS *statsGetArmour(UDWORD ref);
//extern POWER_STATS *statsGetPower(UDWORD ref);
//extern PROGRAM_STATS *statsGetProgram(UDWORD ref);
/* ******************************************************************************
*		Generic stats functions
*******************************************************************************/
/*Load the stats from the Access database*/
//extern BOOL loadStats(void);
/*calls the STATS_DEALLOC macro for each set of stats*/
/*Deallocate the stats passed in as parameter */
//return the type of stat this ref refers to!
//return the REF_START value of this type of stat
/*Returns the component type based on the string - used for reading in data */
//get the component Inc for a stat based on the name
//get the component Inc for a stat based on the Resource name held in Names.txt
/*sets the tech level for the stat passed in */
/*returns the weapon sub class based on the string name passed in */
/*either gets the name associated with the resource (if one) or allocates space and copies pName*/
//converts the name read in from Access into the name which is used in the Stat lists (or ignores it)
/*return the name to display for the interface - valid for OBJECTS and STATS*/
/*sets the store to the body size based on the name passed in - returns FALSE 
if doesn't compare with any*/
// Pass in a stat and get its name
/*returns the propulsion type based on the string name passed in */
/*returns the weapon effect based on the string name passed in */
/*Access functions for the upgradeable stats of a weapon*/
/*Access functions for the upgradeable stats of a sensor*/
/*Access functions for the upgradeable stats of a ECM*/
/*Access functions for the upgradeable stats of a repair*/
/*Access functions for the upgradeable stats of a constructor*/
/*Access functions for the upgradeable stats of a body*/
/*dummy function for John*/
//propulsion stats are not upgradeable
#[no_mangle]
pub unsafe extern "C" fn adjustMaxDesignStats() {
    let mut weaponDamage_0: UWORD = 0;
    let mut sensorRange_0: UWORD = 0;
    let mut sensorPower_0: UWORD = 0;
    let mut repairPoints_0: UWORD = 0;
    let mut ecmPower_0: UWORD = 0;
    let mut constPoints: UWORD = 0;
    let mut bodyPoints: UWORD = 0;
    let mut bodyPower_0: UWORD = 0;
    let mut bodyArmour_0: UWORD = 0;
    let mut inc: UWORD = 0;
    //init all the values
    bodyArmour_0 = 0 as libc::c_int as UWORD;
    bodyPower_0 = bodyArmour_0;
    bodyPoints = bodyPower_0;
    constPoints = bodyPoints;
    ecmPower_0 = constPoints;
    repairPoints_0 = ecmPower_0;
    sensorPower_0 = repairPoints_0;
    sensorRange_0 = sensorPower_0;
    weaponDamage_0 = sensorRange_0;
    //go thru' all the functions getting the max upgrade values for the stats
    inc = 0 as libc::c_int as UWORD;
    while (inc as libc::c_uint) < numFunctions {
        match (**asFunctions.offset(inc as isize)).type_0 as libc::c_int {
            13 => {
                if (repairPoints_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut UPGRADE_FUNCTION)).upgradePoints as
                           libc::c_int {
                    repairPoints_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut UPGRADE_FUNCTION)).upgradePoints
                }
            }
            14 => {
                if (ecmPower_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut UPGRADE_FUNCTION)).upgradePoints as
                           libc::c_int {
                    ecmPower_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut UPGRADE_FUNCTION)).upgradePoints
                }
            }
            15 => {
                if (bodyPoints as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDBODY_UPGRADE_FUNCTION)).body as
                           libc::c_int {
                    bodyPoints =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDBODY_UPGRADE_FUNCTION)).body
                }
                if (bodyPower_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDBODY_UPGRADE_FUNCTION)).upgradePoints
                           as libc::c_int {
                    bodyPower_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDBODY_UPGRADE_FUNCTION)).upgradePoints
                }
                if (bodyArmour_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDBODY_UPGRADE_FUNCTION)).armourValue[WC_KINETIC
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                           as libc::c_int {
                    bodyArmour_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDBODY_UPGRADE_FUNCTION)).armourValue[WC_KINETIC
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                }
                if (bodyArmour_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDBODY_UPGRADE_FUNCTION)).armourValue[WC_HEAT
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                usize]
                           as libc::c_int {
                    bodyArmour_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDBODY_UPGRADE_FUNCTION)).armourValue[WC_HEAT
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 usize]
                }
            }
            16 => {
                if (sensorRange_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDSENSOR_UPGRADE_FUNCTION)).range as
                           libc::c_int {
                    sensorRange_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDSENSOR_UPGRADE_FUNCTION)).range
                }
                if (sensorPower_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut DROIDSENSOR_UPGRADE_FUNCTION)).upgradePoints
                           as libc::c_int {
                    sensorPower_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut DROIDSENSOR_UPGRADE_FUNCTION)).upgradePoints
                }
            }
            17 => {
                if (constPoints as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut UPGRADE_FUNCTION)).upgradePoints as
                           libc::c_int {
                    constPoints =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut UPGRADE_FUNCTION)).upgradePoints
                }
            }
            7 => {
                if (weaponDamage_0 as libc::c_int) <
                       (*(*asFunctions.offset(inc as isize) as
                              *mut WEAPON_UPGRADE_FUNCTION)).damage as
                           libc::c_int {
                    weaponDamage_0 =
                        (*(*asFunctions.offset(inc as isize) as
                               *mut WEAPON_UPGRADE_FUNCTION)).damage
                }
            }
            _ => { }
        }
        inc = inc.wrapping_add(1)
    }
    //determine the effect on the max values for the stats
    updateMaxWeaponStats(weaponDamage_0);
    updateMaxSensorStats(sensorRange_0, sensorPower_0);
    updateMaxRepairStats(repairPoints_0);
    updateMaxECMStats(ecmPower_0);
    updateMaxBodyStats(bodyPoints, bodyPower_0, bodyArmour_0);
    updateMaxConstStats(constPoints);
}
