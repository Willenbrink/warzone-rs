use ::libc;
extern "C" {
    pub type _formation;
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
    /*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
    /* A string block */
    /* An ID entry */
    /* A String Resource */
    // The treap to store string identifiers
    // The store for the strings themselves
    // Sizes for the string blocks
    // The next free ID
    /* Create a string resource object */
    /* Release a string resource object */
    /* Release the id strings, but not the strings themselves,
 * (they can be accessed only by id number).
 */
    /* Load a list of string ID's from a memory buffer
 * id == 0 should be a default string which is returned if the
 * requested string is not found.
 */
    /* Return the ID number for an ID string */
    /* Return the stored ID string that matches the string passed in */
    /* Get the string from an ID number */
    /* Load a string resource file */
    /* Return the the length of a STRING */
    /* Copy a STRING */
    /* Get the ID number for a string*/
    #[no_mangle]
    fn strresGetIDfromString(psRes: *mut STR_RES, pString: *mut STRING)
     -> UDWORD;
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    /* get a stat inc based on the name */
    #[no_mangle]
    fn getStructStatFromName(pName: *mut STRING) -> SDWORD;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    #[no_mangle]
    fn getCompFromResName(compType: UDWORD, pName: *mut STRING) -> SDWORD;
    #[no_mangle]
    static mut numConstructStats: UDWORD;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut numBrainStats: UDWORD;
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asBrainStats: *mut BRAIN_STATS;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    fn sound_GetTrackHashName(iTrack: SDWORD) -> UDWORD;
    #[no_mangle]
    fn audio_SetTrackValsHashName(hash: UDWORD, bLoop: BOOL,
                                  iTrack: libc::c_int, iVol: libc::c_int,
                                  iPriority: libc::c_int,
                                  iAudibleRadius: libc::c_int,
                                  VagID: libc::c_int) -> BOOL;
    #[no_mangle]
    fn audio_GetTrackIDFromHash(hash: UDWORD) -> SDWORD;
    #[no_mangle]
    fn audio_GetAvailableID() -> SDWORD;
    /*
 * Feature.h
 *
 * Definitions for the feature structures.
 *
 */
    //they're just not there anymore!!!!! Ye ha!
    /* The statistics for the features */
    #[no_mangle]
    static mut asFeatureStats: *mut FEATURE_STATS;
    #[no_mangle]
    static mut numFeatureStats: UDWORD;
    /* get a feature stat id from its name */
    #[no_mangle]
    fn getFeatureStatFromName(pName: *mut STRING) -> SDWORD;
    #[no_mangle]
    fn checkValidId(id: UDWORD) -> BOOL;
    #[no_mangle]
    fn stackPushResult(type_0: INTERP_TYPE, data: SDWORD) -> BOOL;
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
    // create a new group
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    // add a droid to a group
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // count the members of a group
    #[no_mangle]
    fn grpNumMembers(psGroup: *mut DROID_GROUP) -> SDWORD;
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
    // get the cluster ID for an object
    #[no_mangle]
    fn clustGetClusterID(psObj: *mut BASE_OBJECT) -> SDWORD;
    /*get the view data that contains the text message pointer passed in */
    #[no_mangle]
    fn getViewData(pTextMsg: *mut STRING) -> *mut VIEWDATA;
    #[no_mangle]
    fn IdToTemplate(tempId: UDWORD, player: UDWORD) -> *mut DROID_TEMPLATE;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    // find the level dataset
    #[no_mangle]
    fn levFindDataSet(pName: *mut STRING, ppsDataSet: *mut *mut LEVEL_DATASET)
     -> BOOL;
    // Link any object types to the actual pointer values
//extern BOOL scrvLinkValues(void);
    // Find a base object from it's id
    #[no_mangle]
    fn scrvGetBaseObj(id: UDWORD, ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    /* For a given view data get the research this is related to */
    #[no_mangle]
    fn getResearch(pName: *mut STRING, resName: BOOL) -> *mut RESEARCH;
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
/*
 * Treap.h
 *
 * A balanced binary tree implementation
 *
 * Overhead for using the treap is -
 *		Overhead for the heap used by the treap :
 *                  24 bytes + 4 bytes per extension
 *      12 bytes for the root
 *      20 bytes per node
 */
/* Turn on and off the treap debugging */
/* Function type for the object compare
 * return -1 for less
 *         1 for more
 *         0 for equal
 */
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
/* The basic elements in the treap node.
 * These are done as macros so that the memory system
 * can use parts of the treap system.
 */
/* The key to sort the node on */
/* Treap priority */
/* The object stored in the treap */
/* The sub trees */
/* The debug info */
/* file the node was created in */
/* line the node was created at */
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
/* Treap data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
// root of the tree
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
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
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
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
/*
 * base.h
 *
 * Definitions for the base object type.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _screen_disp_data {
    pub imd: *mut iIMDShape,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
// warning.... this is not used on the playstation version !
// the polygon number for the next in the BSP list ... or BSPPOLYID_TERMINATE for no more
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
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type BRAIN_STATS = _brain_stats;
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
//line build requires two sets of coords
// maximum number of characters in a droid name
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
pub type RUN_DATA = _run_data;
// basic chance to run
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
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
// Feature armour
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
pub type INTERP_TYPE = _interp_type;
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
/*
 * Event.h
 *
 * Interface to the event management system.
 *
 */
/* The number of values in a context value chunk */
/* One chunk of variables for a script context */
/* The number of links in a context event link chunk */
/* One chunk of event links for a script context */
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
/*
 * ScriptObj.h
 *
 * Object access functions for the script library
 *
 */
// id's for object variables
pub type _objids = libc::c_uint;
//added object->psTarget
//if droid is selected (humans only)
pub const OBJID_TARGET: _objids = 19;
pub const OBJID_SELECTED: _objids = 18;
pub const OBJID_ACTION: _objids = 17;
// order coords.106
pub const OBJID_ORDERY: _objids = 16;
//new
pub const OBJID_ORDERX: _objids = 15;
// the stat of a structure
pub const OBJID_STRUCTSTATTYPE: _objids = 14;
// the weapon component
pub const OBJID_STRUCTSTAT: _objids = 13;
// the propulsion component
pub const OBJID_WEAPON: _objids = 12;
// the body component
pub const OBJID_PROPULSION: _objids = 11;
// %age damage level
pub const OBJID_BODY: _objids = 10;
// which cluster the object is a member of
pub const OBJID_HEALTH: _objids = 9;
// what type of droid
pub const OBJID_CLUSTERID: _objids = 8;
// current droid order
pub const OBJID_DROIDTYPE: _objids = 7;
// type of a base object
pub const OBJID_ORDER: _objids = 6;
// player of a base object
pub const OBJID_TYPE: _objids = 5;
// id of a base object
pub const OBJID_PLAYER: _objids = 4;
pub const OBJID_ID: _objids = 3;
pub const OBJID_POSZ: _objids = 2;
// Position of a base object
pub const OBJID_POSY: _objids = 1;
pub const OBJID_POSX: _objids = 0;
// id's for group variables
pub type _groupids = libc::c_uint;
// average health of a group
// number of units in a group
pub const GROUPID_HEALTH: _groupids = 3;
// average y of a group
pub const GROUPID_MEMBERS: _groupids = 2;
// average x of a group
pub const GROUPID_POSY: _groupids = 1;
pub const GROUPID_POSX: _groupids = 0;
pub const DORDER_NONE: _droid_order = 0;
pub const DORDER_GUARD: _droid_order = 25;
pub type DROID_GROUP = _droid_group;
pub type VIEWDATA = _viewdata;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
//name ID of the message - used for loading in and identifying
//the type of view
//the number of textmessages associated with this data
//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
// the WRF/WDG files needed for a particular level
// the WRF/WDG files needed for a particular level
pub type LEVEL_DATASET = _level_dataset;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _level_dataset {
    pub type_0: SWORD,
    pub players: SWORD,
    pub game: SWORD,
    pub pName: *mut STRING,
    pub dataDir: libc::c_int,
    pub apDataFiles: [*mut STRING; 9],
    pub psBaseData: *mut _level_dataset,
    pub psChange: *mut _level_dataset,
    pub psNext: *mut _level_dataset,
}
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
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
// Get values from a base object
/*
 * ScriptObj.c
 *
 * Object access functions for the script library
 *
 */
// Get values from a base object
#[no_mangle]
pub unsafe extern "C" fn scrBaseObjGet(mut index: UDWORD) -> BOOL {
    //	INTERP_VAL		sVal;
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut val: SDWORD = 0 as libc::c_int;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        debug(LOG_ERROR,
              b"scrBaseObjGet: stackPopParams failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    // Check this is a valid pointer
    if psObj.is_null() {
        debug(LOG_ERROR,
              b"scrBaseObjGet: was passed an invalid pointer\x00" as *const u8
                  as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBaseObjGet: was passed an invalid pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptobj.c\x00" as *const u8 as *const libc::c_char,
                  50 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // Check this is a valid pointer
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_DROID as libc::c_int as libc::c_uint &&
           (*psObj).type_0 as libc::c_uint !=
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*psObj).type_0 as libc::c_uint !=
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"scrBaseObjGet: invalid object\x00" as *const u8 as
                  *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBaseObjGet: invalid object\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptobj.c\x00" as *const u8 as *const libc::c_char,
                  57 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // set the type and return value
    match index {
        0 => { type_0 = VAL_INT; val = (*psObj).x as SDWORD }
        1 => { type_0 = VAL_INT; val = (*psObj).y as SDWORD }
        2 => { type_0 = VAL_INT; val = (*psObj).z as SDWORD }
        3 => { type_0 = VAL_INT; val = (*psObj).id as SDWORD }
        4 => { type_0 = VAL_INT; val = (*psObj).player as SDWORD }
        5 => { type_0 = VAL_INT; val = (*psObj).type_0 as SDWORD }
        6 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: order only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: order only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 92 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = (*(psObj as *mut DROID)).order;
            if val == DORDER_GUARD as libc::c_int &&
                   (*(psObj as *mut DROID)).psTarget.is_null() {
                val = DORDER_NONE as libc::c_int
            }
        }
        17 => {
            //new member variable
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: action only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: action only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 107 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = (*(psObj as *mut DROID)).action
        }
        18 => {
            //new member variable - if droid is selected (humans only)
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: selected only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: selected only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 118 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_BOOL;
            val = (*(psObj as *mut DROID)).selected as SDWORD
        }
        14 => {
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                type_0 = VAL_INT;
                val =
                    (*(*(psObj as *mut STRUCTURE)).pStructureType).type_0 as
                        SDWORD
            } else {
                debug(LOG_ERROR,
                      b".stattype is only supported by Structures\x00" as
                          *const u8 as *const libc::c_char);
                return 0 as libc::c_int
            }
        }
        15 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: order only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: order only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 142 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = (*(psObj as *mut DROID)).orderX as SDWORD
        }
        16 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: order only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: order only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 152 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = (*(psObj as *mut DROID)).orderY as SDWORD
        }
        7 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: droidType only valid for a droid\x00"
                          as *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: droidType only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 162 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = (*(psObj as *mut DROID)).droidType as SDWORD
        }
        8 => {
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_FEATURE as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: clusterID not valid for features\x00"
                          as *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: clusterID not valid for features\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 172 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = VAL_INT;
            val = clustGetClusterID(psObj)
        }
        9 => {
            match (*psObj).type_0 as libc::c_uint {
                0 => {
                    psDroid = psObj as *mut DROID;
                    type_0 = VAL_INT;
                    val =
                        (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                         libc::c_uint).wrapping_div((*psDroid).originalBody)
                            as SDWORD
                }
                2 => {
                    psFeature = psObj as *mut FEATURE;
                    type_0 = VAL_INT;
                    if (*(*psFeature).psStats).damageable != 0 {
                        val =
                            (*psFeature).body.wrapping_mul(100 as libc::c_int
                                                               as
                                                               libc::c_uint).wrapping_div((*(*psFeature).psStats).body)
                                as SDWORD
                    } else { val = 100 as libc::c_int }
                }
                1 => {
                    psStruct = psObj as *mut STRUCTURE;
                    type_0 = VAL_INT;
                    //val = psStruct->body * 100 / psStruct->baseBodyPoints;
                    val =
                        (((*psStruct).body as libc::c_int *
                              100 as libc::c_int) as
                             libc::c_uint).wrapping_div(structureBody(psStruct))
                            as SDWORD
                }
                _ => { }
            }
        }
        10 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: body only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: body only valid for a droid\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 212 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = ST_BODY as libc::c_int as INTERP_TYPE;
            val =
                (*(psObj as
                       *mut DROID)).asBits[COMP_BODY as libc::c_int as
                                               usize].nStat as SDWORD
        }
        11 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: propulsion only valid for a droid\x00"
                          as *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: propulsion only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 222 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = ST_PROPULSION as libc::c_int as INTERP_TYPE;
            val =
                (*(psObj as
                       *mut DROID)).asBits[COMP_PROPULSION as libc::c_int as
                                               usize].nStat as SDWORD
        }
        12 => {
            if (*psObj).type_0 as libc::c_uint !=
                   OBJ_DROID as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: weapon only valid for a droid\x00" as
                          *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet: weapon only valid for a droid\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 232 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            type_0 = ST_WEAPON as libc::c_int as INTERP_TYPE;
            //if (((DROID *)psObj)->numWeaps == 0)
            if (*(psObj as
                      *mut DROID)).asWeaps[0 as libc::c_int as usize].nStat ==
                   0 as libc::c_int as libc::c_uint {
                val = 0 as libc::c_int
            } else {
                val =
                    (*(psObj as
                           *mut DROID)).asWeaps[0 as libc::c_int as
                                                    usize].nStat as SDWORD
            }
        }
        13 => {
            //droid.stat - now returns the type of structure a truck is building for droids
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                type_0 = ST_STRUCTURESTAT as libc::c_int as INTERP_TYPE;
                val =
                    (*(psObj as
                           *mut STRUCTURE)).pStructureType.wrapping_offset_from(asStructureStats)
                        as libc::c_int
            } else if (*psObj).type_0 as libc::c_uint ==
                          OBJ_DROID as libc::c_int as libc::c_uint {
                //psStructStats = (STRUCTURE_STATS*)psDroid->psTarStats;
                type_0 = ST_STRUCTURESTAT as libc::c_int as INTERP_TYPE;
                val =
                    ((*(psObj as *mut DROID)).psTarStats as
                         *mut STRUCTURE_STATS).wrapping_offset_from(asStructureStats)
                        as libc::c_int
            } else {
                //Nothing else supported
                debug(LOG_ERROR,
                      b"scrBaseObjGet(): .stat only valid for structures and droids\x00"
                          as *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet(): .stat only valid for structures and droids\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 265 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        19 => {
            //added object->psTarget
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                type_0 = ST_BASEOBJECT as libc::c_int as INTERP_TYPE;
                val = (*(psObj as *mut STRUCTURE)).psTarget as SDWORD
            } else if (*psObj).type_0 as libc::c_uint ==
                          OBJ_DROID as libc::c_int as libc::c_uint {
                //psStructStats = (STRUCTURE_STATS*)psDroid->psTarStats;
                type_0 = ST_BASEOBJECT as libc::c_int as INTERP_TYPE;
                val = (*(psObj as *mut DROID)).psTarget as SDWORD
            } else {
                //Nothing else supported
                debug(LOG_ERROR,
                      b"scrBaseObjGet(): .target only valid for structures and droids\x00"
                          as *const u8 as *const libc::c_char);
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrBaseObjGet(): .target only valid for structures and droids\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptobj.c\x00" as *const u8 as
                              *const libc::c_char, 287 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        _ => {
            debug(LOG_ERROR,
                  b"scrBaseObjGet: unknown variable index\x00" as *const u8 as
                      *const libc::c_char);
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrBaseObjGet: unknown variable index\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptobj.c\x00" as *const u8 as *const libc::c_char,
                      295 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"scrBaseObjGet\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    // Return the value
    if stackPushResult(type_0, val) == 0 {
        debug(LOG_ERROR,
              b"scrBaseObjGet: stackPushResult() failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Set values from a base object
// Set values from a base object
#[no_mangle]
pub unsafe extern "C" fn scrBaseObjSet(mut index: UDWORD) -> BOOL {
    index = index;
    return 1 as libc::c_int;
}
// convert a base object to a droid if it is the right type
// convert a base object to a droid if it is the right type
#[no_mangle]
pub unsafe extern "C" fn scrObjToDroid() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    // return NULL if not a droid
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_DROID as libc::c_int as libc::c_uint {
        psObj = 0 as *mut BASE_OBJECT
    }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psObj as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// convert a base object to a structure if it is the right type
// convert a base object to a structure if it is the right type
#[no_mangle]
pub unsafe extern "C" fn scrObjToStructure() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    // return NULL if not a droid
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        psObj = 0 as *mut BASE_OBJECT
    }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psObj as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// convert a base object to a feature if it is the right type
// convert a base object to a feature if it is the right type
#[no_mangle]
pub unsafe extern "C" fn scrObjToFeature() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    // return NULL if not a droid
    if (*psObj).type_0 as libc::c_uint !=
           OBJ_FEATURE as libc::c_int as libc::c_uint {
        psObj = 0 as *mut BASE_OBJECT
    }
    if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                       psObj as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// cache all the possible values for the last group to try
// to speed up access
static mut psScrLastGroup: *mut DROID_GROUP =
    0 as *const DROID_GROUP as *mut DROID_GROUP;
static mut lgX: SDWORD = 0;
static mut lgY: SDWORD = 0;
static mut lgMembers: SDWORD = 0;
static mut lgHealth: SDWORD = 0;
static mut lgGameTime: UDWORD = 0;
// Get values from a group
// Get values from a group
#[no_mangle]
pub unsafe extern "C" fn scrGroupObjGet(mut index: UDWORD) -> BOOL {
    let mut type_0: INTERP_TYPE = VAL_BOOL;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut val: SDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP) == 0 {
        return 0 as libc::c_int
    }
    // recalculate the values if necessary
    if lgGameTime != gameTime || psScrLastGroup != psGroup {
        lgGameTime = gameTime;
        psScrLastGroup = psGroup;
        lgMembers = 0 as libc::c_int;
        lgHealth = 0 as libc::c_int;
        lgY = 0 as libc::c_int;
        lgX = lgY;
        psCurr = (*psGroup).psList;
        while !psCurr.is_null() {
            lgMembers += 1 as libc::c_int;
            lgX += (*psCurr).x as SDWORD;
            lgY += (*psCurr).y as SDWORD;
            lgHealth +=
                (100 as libc::c_int as
                     libc::c_uint).wrapping_mul((*psCurr).body).wrapping_div((*psCurr).originalBody)
                    as SDWORD;
            psCurr = (*psCurr).psGrpNext
        }
        if lgMembers > 0 as libc::c_int {
            lgX = lgX / lgMembers;
            lgY = lgY / lgMembers;
            lgHealth = lgHealth / lgMembers
        }
    }
    // set the type and return value
    match index {
        0 => { type_0 = VAL_INT; val = lgX }
        1 => { type_0 = VAL_INT; val = lgY }
        2 => { type_0 = VAL_INT; val = lgMembers }
        3 => { type_0 = VAL_INT; val = lgHealth }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrGroupObjGet: unknown variable index\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptobj.c\x00" as *const u8 as *const libc::c_char,
                      457 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"scrGroupObjGet\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    // Return the value
    if stackPushResult(type_0, val) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// get the name from a stat pointer
#[no_mangle]
pub unsafe extern "C" fn scrGetStatName(mut type_0: INTERP_TYPE,
                                        mut data: UDWORD) -> *mut STRING {
    let mut pName: *mut STRING = 0 as *mut STRING;
    match type_0 as libc::c_uint {
        23 => {
            if data < numStructureStats {
                pName = (*asStructureStats.offset(data as isize)).pName
            }
        }
        24 => {
            if data < numFeatureStats {
                pName = (*asFeatureStats.offset(data as isize)).pName
            }
        }
        13 => {
            if data < numBodyStats {
                pName = (*asBodyStats.offset(data as isize)).pName
            }
        }
        14 => {
            if data < numPropulsionStats {
                pName = (*asPropulsionStats.offset(data as isize)).pName
            }
        }
        15 => {
            if data < numECMStats {
                pName = (*asECMStats.offset(data as isize)).pName
            }
        }
        16 => {
            if data < numSensorStats {
                pName = (*asSensorStats.offset(data as isize)).pName
            }
        }
        17 => {
            if data < numConstructStats {
                pName = (*asConstructStats.offset(data as isize)).pName
            }
        }
        18 => {
            if data < numWeaponStats {
                pName = (*asWeaponStats.offset(data as isize)).pName
            }
        }
        19 => {
            if data < numRepairStats {
                pName = (*asRepairStats.offset(data as isize)).pName
            }
        }
        20 => {
            if data < numBrainStats {
                pName = (*asBrainStats.offset(data as isize)).pName
            }
        }
        11 | 12 | _ => { }
    }
    if pName.is_null() {
        debug(LOG_ERROR,
              b"scrGetStatName: cannot get name for a base stat\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    return pName;
}
// default value save routine
// default value save routine
#[no_mangle]
pub unsafe extern "C" fn scrValDefSave(mut type_0: INTERP_TYPE,
                                       mut data: UDWORD,
                                       mut pBuffer: *mut libc::c_char,
                                       mut pSize: *mut UDWORD) -> BOOL {
    let mut psIntMessage: *mut VIEWDATA = 0 as *mut VIEWDATA;
    let mut pName: *mut STRING = 0 as *mut STRING;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psCDroid: *mut DROID = 0 as *mut DROID;
    let mut members: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    match type_0 as libc::c_uint {
        6 => {
            // save the name
            psIntMessage = data as *mut VIEWDATA;
            if !psIntMessage.is_null() {
                if !pBuffer.is_null() {
                    strcpy(pBuffer, (*psIntMessage).pName);
                }
                *pSize =
                    strlen((*psIntMessage).pName).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
            } else {
                if !pBuffer.is_null() {
                    *(pBuffer as *mut UBYTE) = 0 as libc::c_int as UBYTE
                }
                *pSize = 1 as libc::c_int as UDWORD
            }
        }
        7 | 8 | 9 | 10 => {
            // just save the id
            if !pBuffer.is_null() {
                if data == 0 as libc::c_int as libc::c_uint {
                    *(pBuffer as *mut UDWORD) = 0xffffffff as libc::c_uint
                } else {
                    *(pBuffer as *mut UDWORD) =
                        (*(data as *mut BASE_OBJECT)).id
                }
            }
            *pSize = ::std::mem::size_of::<UDWORD>() as libc::c_ulong
        }
        11 | 12 | 24 | 23 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
            pName = scrGetStatName(type_0, data);
            if !pName.is_null() {
                if !pBuffer.is_null() { strcpy(pBuffer, pName); }
                *pSize =
                    strlen(pName).wrapping_add(1 as libc::c_int as
                                                   libc::c_uint)
            } else { return 0 as libc::c_int }
        }
        21 => {
            if !pBuffer.is_null() {
                if data == 0 as libc::c_int as libc::c_uint {
                    *(pBuffer as *mut UDWORD) = 0xffffffff as libc::c_uint
                } else {
                    *(pBuffer as *mut UDWORD) =
                        (*(data as *mut DROID_TEMPLATE)).multiPlayerID
                }
            }
            *pSize = ::std::mem::size_of::<UDWORD>() as libc::c_ulong
        }
        26 => {
            if !pBuffer.is_null() {
                if data == 0 as libc::c_int as libc::c_uint {
                    *(pBuffer as *mut UDWORD) = 0xffffffff as libc::c_uint
                } else {
                    *(pBuffer as *mut UDWORD) =
                        strresGetIDfromString(psStringRes,
                                              data as *mut STRING)
                }
            }
            *pSize = ::std::mem::size_of::<UDWORD>() as libc::c_ulong
        }
        28 => {
            if data != 0 as libc::c_int as libc::c_uint {
                if !pBuffer.is_null() {
                    strcpy(pBuffer, data as *mut STRING);
                }
                *pSize =
                    strlen(data as
                               *mut STRING).wrapping_add(1 as libc::c_int as
                                                             libc::c_uint)
            } else {
                if !pBuffer.is_null() {
                    *(pBuffer as *mut UBYTE) = 0 as libc::c_int as UBYTE
                }
                *pSize = 1 as libc::c_int as UDWORD
            }
        }
        30 => {
            psResearch = data as *mut RESEARCH;
            if !psResearch.is_null() {
                if !pBuffer.is_null() {
                    strcpy(pBuffer, (*psResearch).pName);
                }
                *pSize =
                    strlen((*psResearch).pName).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
            } else {
                if !pBuffer.is_null() {
                    *(pBuffer as *mut UBYTE) = 0 as libc::c_int as UBYTE
                }
                *pSize = 1 as libc::c_int as UDWORD
            }
        }
        29 => {
            if data != 0 as libc::c_int as libc::c_uint {
                members = grpNumMembers(data as *mut DROID_GROUP)
            } else { members = 0 as libc::c_int }
            if !pBuffer.is_null() {
                pPos = pBuffer;
                psGroup = data as *mut DROID_GROUP;
                // store the run data
                *(pPos as *mut SDWORD) = (*psGroup).sRunData.sPos.x;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                *(pPos as *mut SDWORD) = (*psGroup).sRunData.sPos.y;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                *(pPos as *mut SDWORD) =
                    (*psGroup).sRunData.forceLevel as SDWORD;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                *(pPos as *mut SDWORD) =
                    (*psGroup).sRunData.leadership as SDWORD;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                *(pPos as *mut SDWORD) =
                    (*psGroup).sRunData.healthLevel as SDWORD;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                // now store the droids
                psCDroid = (*(data as *mut DROID_GROUP)).psList;
                while !psCDroid.is_null() {
                    checkValidId((*psCDroid).id);
                    *(pPos as *mut UDWORD) = (*psCDroid).id;
                    pPos =
                        pPos.offset(::std::mem::size_of::<UDWORD>() as
                                        libc::c_ulong as isize);
                    psCDroid = (*psCDroid).psGrpNext
                }
            }
            *pSize =
                (::std::mem::size_of::<UDWORD>() as
                     libc::c_ulong).wrapping_mul(members as
                                                     libc::c_uint).wrapping_add((::std::mem::size_of::<SDWORD>()
                                                                                     as
                                                                                     libc::c_ulong).wrapping_mul(5
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint))
        }
        27 => {
            /*		pName = sound_GetTrackName((SDWORD)data);
		if (pName == NULL)
		{
			DBERROR(("scrValDefSave: couldn't get sound track name"));
			return FALSE;
		}
		if (pBuffer)
		{
			strcpy((char *)pBuffer, pName);
		}
		*pSize = strlen((char *)pName) + 1;*/
            if !pBuffer.is_null() {
                *(pBuffer as *mut UDWORD) =
                    sound_GetTrackHashName(data as SDWORD)
            }
            *pSize = ::std::mem::size_of::<UDWORD>() as libc::c_ulong
        }
        22 | 25 | _ => {
            if type_0 as libc::c_uint ==
                   ST_STRUCTUREID as libc::c_int as libc::c_uint ||
                   type_0 as libc::c_uint ==
                       ST_DROIDID as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"scrValDefSave: unknown script variable type for save\x00"
                          as *const u8 as *const libc::c_char);
            };
            if type_0 as libc::c_uint ==
                   ST_STRUCTUREID as libc::c_int as libc::c_uint ||
                   type_0 as libc::c_uint ==
                       ST_DROIDID as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptobj.c\x00" as *const u8 as *const libc::c_char,
                      763 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"scrValDefSave\x00")).as_ptr(),
                      b"(type == ST_STRUCTUREID) || (type == ST_DROIDID)\x00"
                          as *const u8 as *const libc::c_char);
            };
            // just save the variable contents directly
            if !pBuffer.is_null() { *(pBuffer as *mut UDWORD) = data }
            *pSize = ::std::mem::size_of::<UDWORD>() as libc::c_ulong
        }
    }
    return 1 as libc::c_int;
}
// default value load routine
// default value load routine
#[no_mangle]
pub unsafe extern "C" fn scrValDefLoad(mut version: SDWORD,
                                       mut type_0: INTERP_TYPE,
                                       mut pBuffer: *mut libc::c_char,
                                       mut size: UDWORD,
                                       mut pData: *mut UDWORD) -> BOOL {
    let mut pPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psCDroid: *mut DROID = 0 as *mut DROID;
    let mut index: SDWORD = 0;
    let mut members: SDWORD = 0;
    let mut id: UDWORD = 0;
    let mut psLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    match type_0 as libc::c_uint {
        6 => {
            if size == 1 as libc::c_int as libc::c_uint &&
                   *pBuffer as libc::c_int == 0 as libc::c_int {
                *pData = 0 as libc::c_int as UDWORD
            } else {
                *pData = getViewData(pBuffer as *mut STRING) as UDWORD;
                if *pData == 0 as libc::c_int as libc::c_uint {
                    return 0 as libc::c_int
                }
            }
        }
        7 | 8 | 9 | 10 => {
            id = *(pBuffer as *mut UDWORD);
            if id == 0xffffffff as libc::c_uint {
                *pData = 0 as libc::c_int as UDWORD
            } else if scrvGetBaseObj(*(pBuffer as *mut UDWORD),
                                     pData as *mut *mut BASE_OBJECT) == 0 {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find object id %d\x00" as
                          *const u8 as *const libc::c_char,
                      *(pBuffer as *mut UDWORD));
                abort();
            }
        }
        11 | 12 => { }
        23 => {
            index = getStructStatFromName(pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find structure stat %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        24 => {
            index = getFeatureStatFromName(pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find feature stat %s\x00" as
                          *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        13 => {
            index =
                getCompFromResName(COMP_BODY as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find body component %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        14 => {
            index =
                getCompFromResName(COMP_PROPULSION as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find propulsion component %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        15 => {
            index =
                getCompFromResName(COMP_ECM as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find ECM component %s\x00" as
                          *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        16 => {
            index =
                getCompFromResName(COMP_SENSOR as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find sensor component %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        17 => {
            index =
                getCompFromResName(COMP_CONSTRUCT as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find constructor component %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        18 => {
            index =
                getCompFromResName(COMP_WEAPON as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find weapon %s\x00" as
                          *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        19 => {
            index =
                getCompFromResName(COMP_REPAIRUNIT as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find repair component %s\x00"
                          as *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        20 => {
            index =
                getCompFromResName(COMP_BRAIN as libc::c_int as UDWORD,
                                   pBuffer);
            if index == -(1 as libc::c_int) {
                debug(LOG_ERROR,
                      b"scrValDefLoad: couldn\'t find repair brain %s\x00" as
                          *const u8 as *const libc::c_char, pBuffer);
                abort();
            }
            *pData = index as UDWORD
        }
        21 => {
            id = *(pBuffer as *mut UDWORD);
            if id == 0xffffffff as libc::c_uint {
                let ref mut fresh0 = *(pData as *mut *mut DROID_TEMPLATE);
                *fresh0 = 0 as *mut DROID_TEMPLATE
            } else {
                let ref mut fresh1 = *(pData as *mut *mut DROID_TEMPLATE);
                *fresh1 = IdToTemplate(id, 99 as libc::c_int as UDWORD);
                if *pData == 0 as libc::c_int as libc::c_uint {
                    debug(LOG_ERROR,
                          b"scrValDefLoad: couldn\'t find template id %d\x00"
                              as *const u8 as *const libc::c_char,
                          *(pBuffer as *mut UDWORD));
                    abort();
                }
            }
        }
        26 => {
            if *(pBuffer as *mut UDWORD) == 0xffffffff as libc::c_uint {
                *pData = 0 as libc::c_int as UDWORD
            } else {
                *pData =
                    strresGetString(psStringRes, *(pBuffer as *mut UDWORD)) as
                        UDWORD
            }
        }
        28 => {
            if size == 1 as libc::c_int as libc::c_uint &&
                   *pBuffer as libc::c_int == 0 as libc::c_int {
                *pData = 0 as libc::c_int as UDWORD
            } else {
                if levFindDataSet(pBuffer, &mut psLevel) == 0 {
                    debug(LOG_ERROR,
                          b"scrValDefLoad: couldn\'t find level dataset %s\x00"
                              as *const u8 as *const libc::c_char, pBuffer);
                    abort();
                }
                let ref mut fresh2 = *(pData as *mut *mut STRING);
                *fresh2 = (*psLevel).pName
            }
        }
        30 => {
            if size == 1 as libc::c_int as libc::c_uint &&
                   *pBuffer as libc::c_int == 0 as libc::c_int {
                *pData = 0 as libc::c_int as UDWORD
            } else {
                *pData = getResearch(pBuffer, 1 as libc::c_int) as UDWORD;
                if *pData == 0 as libc::c_int as libc::c_uint {
                    debug(LOG_ERROR,
                          b"scrValDefLoad: couldn\'t find research %s\x00" as
                              *const u8 as *const libc::c_char, pBuffer);
                    abort();
                }
            }
        }
        29 => {
            if *pData == 0 as libc::c_int as libc::c_uint {
                if grpCreate(pData as *mut *mut DROID_GROUP) == 0 {
                    debug(LOG_ERROR,
                          b"scrValDefLoad: out of memory\x00" as *const u8 as
                              *const libc::c_char);
                    abort();
                } else {
                    grpJoin(*(pData as *mut *mut DROID_GROUP),
                            0 as *mut DROID);
                }
            }
            if version == 1 as libc::c_int {
                members =
                    size.wrapping_div(::std::mem::size_of::<UDWORD>() as
                                          libc::c_ulong) as SDWORD;
                pPos = pBuffer
            } else if version == 2 as libc::c_int {
                members =
                    size.wrapping_sub((::std::mem::size_of::<SDWORD>() as
                                           libc::c_ulong).wrapping_mul(4 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)).wrapping_div(::std::mem::size_of::<UDWORD>()
                                                                                                           as
                                                                                                           libc::c_ulong)
                        as SDWORD;
                pPos = pBuffer;
                // load the retreat data
                psGroup = *(pData as *mut *mut DROID_GROUP);
                (*psGroup).sRunData.sPos.x = *(pPos as *mut SDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.sPos.y = *(pPos as *mut SDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.forceLevel =
                    *(pPos as *mut SDWORD) as UBYTE;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.leadership =
                    *(pPos as *mut SDWORD) as UBYTE;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize)
            } else {
                members =
                    size.wrapping_sub((::std::mem::size_of::<SDWORD>() as
                                           libc::c_ulong).wrapping_mul(5 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)).wrapping_div(::std::mem::size_of::<UDWORD>()
                                                                                                           as
                                                                                                           libc::c_ulong)
                        as SDWORD;
                pPos = pBuffer;
                // load the retreat data
                psGroup = *(pData as *mut *mut DROID_GROUP);
                (*psGroup).sRunData.sPos.x = *(pPos as *mut SDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.sPos.y = *(pPos as *mut SDWORD);
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.forceLevel =
                    *(pPos as *mut SDWORD) as UBYTE;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.leadership =
                    *(pPos as *mut SDWORD) as UBYTE;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize);
                (*psGroup).sRunData.healthLevel =
                    *(pPos as *mut SDWORD) as UBYTE;
                pPos =
                    pPos.offset(::std::mem::size_of::<SDWORD>() as
                                    libc::c_ulong as isize)
            }
            // load the droids
            while members > 0 as libc::c_int {
                if scrvGetBaseObj(*(pPos as *mut UDWORD),
                                  &mut psCDroid as *mut *mut DROID as
                                      *mut *mut BASE_OBJECT) == 0 {
                    debug(LOG_ERROR,
                          b"scrValDefLoad: couldn\'t find object id %d\x00" as
                              *const u8 as *const libc::c_char,
                          *(pBuffer as *mut UDWORD));
                    abort();
                } else {
                    grpJoin(*(pData as *mut *mut DROID_GROUP), psCDroid);
                }
                pPos =
                    pPos.offset(::std::mem::size_of::<UDWORD>() as
                                    libc::c_ulong as isize);
                members -= 1 as libc::c_int
            }
        }
        27 => {
            // find audio id
            index = audio_GetTrackIDFromHash(*(pBuffer as *mut UDWORD));
            if index == -(3 as libc::c_int) {
                // find empty id
                index = audio_GetAvailableID();
                if index == -(1 as libc::c_int) {
                    debug(LOG_ERROR,
                          b"Sound ID not available %s not found\x00" as
                              *const u8 as *const libc::c_char, pBuffer);
                    abort();
                } else {
                    // set track vals
                    audio_SetTrackValsHashName(*(pBuffer as *mut UDWORD),
                                               0 as libc::c_int, index,
                                               100 as libc::c_int,
                                               1 as libc::c_int,
                                               1800 as libc::c_int,
                                               0 as libc::c_int);
                }
            }
            *pData = index as UDWORD
        }
        22 | 25 | _ => {
            // just set the contents directly
            *pData = *(pBuffer as *mut UDWORD)
        }
    }
    return 1 as libc::c_int;
}
