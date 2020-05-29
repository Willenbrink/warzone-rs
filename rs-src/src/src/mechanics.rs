use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
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
    fn enableResearch(psResearch: *mut RESEARCH, player: UDWORD) -> BOOL;
    #[no_mangle]
    static mut numResearch: UDWORD;
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    #[no_mangle]
    fn droidRelease(psDroid: *mut DROID);
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    /* Release all resources associated with a structure */
    #[no_mangle]
    fn structureRelease(psBuilding: *mut STRUCTURE);
    /* Release the resources associated with a feature */
    #[no_mangle]
    fn featureRelease(psFeature: *mut FEATURE);
    /*
 * ObjMem.h
 *
 * Routines for managing object's memory
 *
 */
    //the died flag for a droid is set to this when it gets added to the non-current list
    /* The memory heaps for the different object types */
    #[no_mangle]
    static mut psDroidHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut psStructHeap: *mut OBJ_HEAP;
    #[no_mangle]
    static mut psFeatureHeap: *mut OBJ_HEAP;
    /* The list of destroyed objects */
    #[no_mangle]
    static mut psDestroyedObj: *mut BASE_OBJECT;
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    static mut numBrainStats: UDWORD;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
    #[no_mangle]
    static mut numConstructStats: UDWORD;
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
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
pub type COMPONENT_TYPE = _component_type;
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
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
pub type COMP_BASE_STATS = _comp_base_stats;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
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
pub type WEAPON_SUBCLASS = _weapon_subclass;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
pub type WEAPON_EFFECT = _weapon_effect;
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
//COMP_ARMOUR,
//COMP_POWERPLANT,
//COMP_PROGRAM,		//this needs to be removed when save games changes
//ALL components and structures and research topics have a tech level to which they belong
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
/* LOC used for holding locations for Sensors and ECM's*/
/* SIZE used for specifying body size */
//only using KINETIC and HEAT for now
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
// used to define which projectile model to use for the weapon
//used to modify the damage to a propuslion type (or structure) based on weapon
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
// Program capacity
//UDWORD		AICap;				// AI capacity
	//UDWORD		AISpeed;			// AI Learning Speed
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
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
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
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
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
// Feature armour
/* Initialise the mechanics system */
//BOOL mechInitialise(void)
//{
//	UBYTE	*pFileData;
//	UDWORD	fileSize;
/* Initialise the map */
	/*if (!loadFile("blank.map", &pFileData, &fileSize))
	{
		return FALSE;
	}

	if (!mapLoad(pFileData, fileSize))
	{
		return FALSE;
	}

	FREE(pFileData); */
//load all the component stats from the access database
	/*if (!loadStats())
	{
		return FALSE;
	}*/
//load the droid Templates
	/*if (!loadDroidTemplates())
	{
		return FALSE;
	}*/
/*if (!loadFunctionStats())
	{
		return FALSE;
	}*/
/*if (!loadStructureStats())
	{
		return FALSE;
	}*/
/*if (!loadStructureWeapons())
	{
		return FALSE;
	}

	if (!loadStructureFunctions())
	{
		return FALSE;
	}*/
//load the research stats - must have loaded the structure and functions stats first
/*	if (!loadResearch())
	{
		return FALSE;
	}
*/
	/*the template weapons and programs should have been read in through the wrf file by now
	 so calculate build points and power points*/
	//initTemplatePoints(); DONE IN SAVE GAME NOW SINCE ALWAYS STARTING IN ONE
//sets up the initial game stats - what the player's got etc
//	gameStatStart();	- moved to data.c when stats are loaded from WRF - John.
/*set up the Power levels for each player - again this is set up in load game now*/
	//initPlayerPower();
//	return TRUE;
//}
/* Shutdown the mechanics system */
#[no_mangle]
pub unsafe extern "C" fn mechShutdown() -> BOOL {
    //	UDWORD	i;
//	DROID	*psCurr;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psNext: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    /*	for(i=0; i<MAX_PLAYERS; i++)
	{
		for(psCurr=apsDroidLists[i]; psCurr != NULL; psCurr = psCurr->psNext)
		{
			FREE(psCurr->pName);
			if (psCurr->numWeaps > 0)
			{
				FREE(psCurr->asWeaps);
			}
			if (psCurr->numProgs > 0)
			{
				FREE(psCurr->asProgs);
			}
		}
	}*/
    psObj = psDestroyedObj;
    while !psObj.is_null() {
        psNext = (*psObj).psNext;
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            /*	FREE(((DROID *)psObj)->pName);
			if (((DROID *)psObj)->numWeaps > 0)
			{
				FREE(((DROID *)psObj)->asWeaps);
			}
			if (((DROID *)psObj)->numProgs > 0)
			{
				FREE(((DROID *)psObj)->asProgs);
			}*/
            droidRelease(psObj as *mut DROID);
            heapFree(psDroidHeap, psObj as *mut DROID as *mut libc::c_void);
        }
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            structureRelease(psObj as *mut STRUCTURE);
            heapFree(psStructHeap,
                     psObj as *mut STRUCTURE as *mut libc::c_void);
        }
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
            featureRelease(psObj as *mut FEATURE);
            heapFree(psFeatureHeap,
                     psObj as *mut FEATURE as *mut libc::c_void);
        }
        psObj = psNext
    }
    psDestroyedObj = 0 as *mut BASE_OBJECT;
    //Free the space allocated for the players lists
//	gameStatEnd();
    return 1 as libc::c_int;
}
// Allocate the list for a component
#[no_mangle]
pub unsafe extern "C" fn allocComponentList(mut type_0: COMPONENT_TYPE,
                                            mut number: SDWORD) -> BOOL {
    let mut inc: SDWORD = 0;
    let mut comp: SDWORD = 0;
    //allocate the space for the Players' component lists
    inc = 0 as libc::c_int;
    while inc < 8 as libc::c_int {
        apCompLists[inc as usize][type_0 as usize] =
            memMallocRelease((::std::mem::size_of::<UBYTE>() as
                                  libc::c_ulong).wrapping_mul(number as
                                                                  libc::c_uint))
                as *mut UBYTE;
        if apCompLists[inc as usize][type_0 as usize].is_null() {
            debug(LOG_ERROR,
                  b"Out of memory assigning Player Component Lists\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
        //initialise the players' lists
        comp = 0 as libc::c_int;
        while comp < number {
            *apCompLists[inc as usize][type_0 as usize].offset(comp as isize)
                = 0x2 as libc::c_int as UBYTE;
            comp += 1
        }
        inc += 1
    }
    return 1 as libc::c_int;
}
// release all the component lists
#[no_mangle]
pub unsafe extern "C" fn freeComponentLists() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //free the component lists
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_BODY as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_BODY as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_BRAIN as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_BRAIN as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_PROPULSION as libc::c_int
                                                  as usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_PROPULSION as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_SENSOR as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_SENSOR as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_ECM as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_ECM as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_REPAIRUNIT as libc::c_int
                                                  as usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_REPAIRUNIT as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_CONSTRUCT as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_CONSTRUCT as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_WEAPON as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_WEAPON as libc::c_int as usize] =
            0 as *mut UBYTE;
        inc = inc.wrapping_add(1)
        //FREE(apCompLists[inc][COMP_PROGRAM]);
    };
}
//allocate the space for the Players' structure lists
#[no_mangle]
pub unsafe extern "C" fn allocStructLists() -> BOOL {
    let mut inc: SDWORD = 0;
    let mut stat: SDWORD = 0;
    inc = 0 as libc::c_int;
    while inc < 8 as libc::c_int {
        if numStructureStats != 0 {
            apStructTypeLists[inc as usize] =
                memMallocRelease((::std::mem::size_of::<UBYTE>() as
                                      libc::c_ulong).wrapping_mul(numStructureStats))
                    as *mut UBYTE;
            if apStructTypeLists[inc as usize].is_null() {
                debug(LOG_ERROR,
                      b"Out of memory assigning Player Structure Lists\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
            stat = 0 as libc::c_int;
            while stat < numStructureStats as SDWORD {
                *apStructTypeLists[inc as usize].offset(stat as isize) =
                    0x2 as libc::c_int as UBYTE;
                stat += 1
            }
        } else { apStructTypeLists[inc as usize] = 0 as *mut UBYTE }
        inc += 1
    }
    return 1 as libc::c_int;
}
// release the structure lists
#[no_mangle]
pub unsafe extern "C" fn freeStructureLists() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //free the structure lists
        if !apStructTypeLists[inc as usize].is_null() {
            memFreeRelease(apStructTypeLists[inc as usize] as
                               *mut libc::c_void);
            apStructTypeLists[inc as usize] = 0 as *mut UBYTE
        }
        inc = inc.wrapping_add(1)
    };
}
/*
 * Mechanics.c
 *
 * Game world mechanics.
 *
 */
/* extra structures required for demo */
//#define DEMO
//initialises the players list for the start of the DEMO
#[no_mangle]
pub unsafe extern "C" fn gameStatStart() -> BOOL {
    //	UDWORD			inc, comp, stat;
//	BOOL			builtRes = FALSE, builtGen = FALSE;
    /* *********************************************************************
 *              All this gets done by calls from data.c to
 *              allocComponentList and allocStructLists (above)
 *              John.

	//allocate the space for the Players' component lists
	for (inc=0; inc < MAX_PLAYERS; inc++)
	{
		apCompLists[inc][COMP_BODY] = (UDWORD *) MALLOC(sizeof(UDWORD) * numBodyStats);
		if (apCompLists[inc][COMP_BODY] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_BRAIN] = (UDWORD *) MALLOC(sizeof(UDWORD) * numBrainStats);
		if (apCompLists[inc][COMP_BRAIN] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_ECM] = (UDWORD *) MALLOC(sizeof(UDWORD) * numECMStats);
		if (apCompLists[inc][COMP_ECM] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_PROPULSION] = (UDWORD *) MALLOC(sizeof(UDWORD) * numPropulsionStats);
		if (apCompLists[inc][COMP_PROPULSION] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_SENSOR] = (UDWORD *) MALLOC(sizeof(UDWORD) * numSensorStats);
		if (apCompLists[inc][COMP_SENSOR] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_REPAIRUNIT] = (UDWORD *) MALLOC(sizeof(UDWORD) * numRepairStats);
		if (apCompLists[inc][COMP_REPAIRUNIT] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_CONSTRUCT] = (UDWORD *) MALLOC(sizeof(UDWORD) * numConstructStats);
		if (apCompLists[inc][COMP_REPAIRUNIT] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_WEAPON] = (UDWORD *) MALLOC(sizeof(UDWORD) * numWeaponStats);
		if (apCompLists[inc][COMP_WEAPON] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
		apCompLists[inc][COMP_PROGRAM] = (UDWORD *) MALLOC(sizeof(UDWORD) * numProgramStats);
		if (apCompLists[inc][COMP_PROGRAM] == NULL)
		{
			DBERROR(("Out of memory assigning Player Component Lists"));
			return FALSE;
		}
	}

	//initialise the players' lists
	for (inc = 0; inc < MAX_PLAYERS; inc++)
	{
		for (comp=0; comp <numBodyStats; comp++)
		{
			apCompLists[inc][COMP_BODY][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numBrainStats; comp++)
		{
			apCompLists[inc][COMP_BRAIN][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numPropulsionStats; comp++)
		{
			apCompLists[inc][COMP_PROPULSION][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numSensorStats; comp++)
		{
			apCompLists[inc][COMP_SENSOR][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numECMStats; comp++)
		{
			apCompLists[inc][COMP_ECM][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numRepairStats; comp++)
		{
			apCompLists[inc][COMP_REPAIRUNIT][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numConstructStats; comp++)
		{
			apCompLists[inc][COMP_CONSTRUCT][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numWeaponStats; comp++)
		{
			apCompLists[inc][COMP_WEAPON][comp] = UNAVAILABLE;
		}
		for (comp=0; comp < numProgramStats; comp++)
		{
			apCompLists[inc][COMP_PROGRAM][comp] = UNAVAILABLE;
		}
	}

	//allocate the space for the Players' structure lists
	for (inc=0; inc < MAX_PLAYERS; inc++)
	{
		if(numStructureStats)
		{
#ifdef DEMO
			apStructTypeLists[inc] = (UDWORD *) MALLOC(sizeof(UDWORD) *
								(numStructureStats + NUM_DEMO_STRUCTS));
#else
			apStructTypeLists[inc] = (UDWORD *) MALLOC(sizeof(UDWORD) *
								numStructureStats);
#endif
			if (apStructTypeLists[inc] == NULL)
			{
				DBERROR(("Out of memory assigning Player Structure Lists"));
				return FALSE;
			}
#ifdef DEMO
			for (stat = 0; stat < numStructureStats + NUM_DEMO_STRUCTS; stat++)
			{
				apStructTypeLists[inc][stat] = UNAVAILABLE;
			}
#else
			for (stat = 0; stat < numStructureStats; stat++)
			{
				apStructTypeLists[inc][stat] = UNAVAILABLE;
			}
#endif
		}
		else
		{
			apStructTypeLists[inc] = NULL;
		}
	}
*/
	//don't want any of the next bit if using scripts
    //#ifdef DEMO
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gameStatEnd() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //free the component lists
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_BODY as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_BODY as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_BRAIN as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_BRAIN as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_PROPULSION as libc::c_int
                                                  as usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_PROPULSION as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_SENSOR as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_SENSOR as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_ECM as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_ECM as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_REPAIRUNIT as libc::c_int
                                                  as usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_REPAIRUNIT as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_CONSTRUCT as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_CONSTRUCT as libc::c_int as usize] =
            0 as *mut UBYTE;
        memFreeRelease(apCompLists[inc as
                                       usize][COMP_WEAPON as libc::c_int as
                                                  usize] as
                           *mut libc::c_void);
        apCompLists[inc as usize][COMP_WEAPON as libc::c_int as usize] =
            0 as *mut UBYTE;
        //FREE(apCompLists[inc][COMP_PROGRAM]);
        //free the structure lists
        if !apStructTypeLists[inc as usize].is_null() {
            memFreeRelease(apStructTypeLists[inc as usize] as
                               *mut libc::c_void);
            apStructTypeLists[inc as usize] = 0 as *mut UBYTE
        }
        inc = inc.wrapping_add(1)
    };
}
/*
 * Mechanics.h
 *
 * Game world mechanics.
 *
 */
/* Initialise the mechanics system */
//extern BOOL mechInitialise(void);
/* Shutdown the mechanics system */
// Allocate the list for a component
// release all the component lists
//allocate the space for the Players' structure lists
// release the structure lists
//TEST FUNCTION - MAKE EVERYTHING AVAILABLE
//TEST FUNCTION - MAKE EVERYTHING AVAILABLE
#[no_mangle]
pub unsafe extern "C" fn makeAllAvailable() {
    let mut comp: UDWORD = 0;
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        comp = 0 as libc::c_int as UDWORD;
        while comp < numWeaponStats {
            *apCompLists[i as
                             usize][COMP_WEAPON as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numBodyStats {
            *apCompLists[i as
                             usize][COMP_BODY as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numPropulsionStats {
            *apCompLists[i as
                             usize][COMP_PROPULSION as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numSensorStats {
            *apCompLists[i as
                             usize][COMP_SENSOR as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numECMStats {
            *apCompLists[i as
                             usize][COMP_ECM as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numConstructStats {
            *apCompLists[i as
                             usize][COMP_CONSTRUCT as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numBrainStats {
            *apCompLists[i as
                             usize][COMP_BRAIN as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        comp = 0 as libc::c_int as UDWORD;
        while comp < numRepairStats {
            *apCompLists[i as
                             usize][COMP_REPAIRUNIT as libc::c_int as
                                        usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        /*for (comp=i; comp <numProgramStats; comp++)
		{
			apCompLists[i][COMP_PROGRAM][comp] = AVAILABLE;
		}*/
        //make all the structures available
        comp = 0 as libc::c_int as UDWORD;
        while comp < numStructureStats {
            *apStructTypeLists[i as usize].offset(comp as isize) =
                0x1 as libc::c_int as UBYTE;
            comp = comp.wrapping_add(1)
        }
        //make all research availble to be performed
        comp = 0 as libc::c_int as UDWORD;
        while comp < numResearch {
            enableResearch(&mut *asResearch.offset(comp as isize), i);
            comp = comp.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
