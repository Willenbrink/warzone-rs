use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    /*
 * droid.h
 *
 * Definitions for the droid object.
 *
 */
    // world->screen check - alex
    // percentage of body points remaining at which to repair droid automatically.
    // ditto, but this will repair much sooner..
    /*defines the % to decrease the illumination of a tile when building - gets set 
back when building is destroyed*/
//#define FOUNDATION_ILLUMIN		50
    //storage
    #[no_mangle]
    static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8];
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    /* get a stat inc based on the name */
    #[no_mangle]
    fn getStructStatFromName(pName: *mut STRING) -> SDWORD;
    /*checks to see if a specific structure type exists -as opposed to a structure 
stat type*/
    #[no_mangle]
    fn checkSpecificStructExists(structInc: UDWORD, player: UDWORD) -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
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
    //extern BOOL loadFunction(char *pData, UDWORD functionType);
//extern BOOL loadDefensiveStructFunction(char *pData);
//extern BOOL loadArmourUpgradeFunction(char *pData);
//extern BOOL loadPowerRegFunction(char *pData);
//extern BOOL loadPowerRelayFunction(char *pData);
//extern BOOL loadRadarMapFunction(char *pData);
//extern BOOL loadRepairUpgradeFunction(char *pData);
//extern BOOL loadResistanceUpgradeFunction(char *pData);
//extern BOOL loadBodyUpgradeFunction(char *pData);
    #[no_mangle]
    fn productionUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn researchUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn powerUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn reArmUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn repairFacUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn weaponUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn structureUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn wallDefenceUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn structureBodyUpgrade(pFunction: *mut FUNCTION,
                            psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureArmourUpgrade(pFunction: *mut FUNCTION,
                              psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureResistanceUpgrade(pFunction: *mut FUNCTION,
                                  psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureProductionUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureResearchUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structurePowerUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureRepairUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureSensorUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureReArmUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureECMUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn sensorUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn repairUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn ecmUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn constructorUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn bodyUpgrade(pFunction: *mut FUNCTION, player: UBYTE);
    #[no_mangle]
    fn droidSensorUpgrade(psDroid: *mut DROID);
    #[no_mangle]
    fn droidECMUpgrade(psDroid: *mut DROID);
    #[no_mangle]
    fn droidBodyUpgrade(pFunction: *mut FUNCTION, psDroid: *mut DROID);
    #[no_mangle]
    fn upgradeTransporterDroids(psTransporter: *mut DROID,
                                pUpgradeFunction:
                                    Option<unsafe extern "C" fn(_: *mut DROID)
                                               -> ()>);
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut asBrainStats: *mut BRAIN_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
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
    /* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    //return the type of stat this ref refers to!
    #[no_mangle]
    fn statType(ref_0: UDWORD) -> UDWORD;
    //return the REF_START value of this type of stat
    #[no_mangle]
    fn statRefStart(stat: UDWORD) -> UDWORD;
    /*Returns the component type based on the string - used for reading in data */
    #[no_mangle]
    fn componentType(pType: *mut libc::c_char) -> UDWORD;
    /*sets the tech level for the stat passed in */
    #[no_mangle]
    fn setTechLevel(psStats: *mut BASE_STATS, pLevel: *mut STRING) -> BOOL;
    /*either gets the name associated with the resource (if one) or allocates space and copies pName*/
    #[no_mangle]
    fn allocateName(ppStore: *mut *mut STRING, pName: *mut STRING) -> BOOL;
    //converts the name read in from Access into the name which is used in the Stat lists (or ignores it)
    #[no_mangle]
    fn getResourceName(pName: *mut STRING) -> BOOL;
    /*return the name to display for the interface - valid for OBJECTS and STATS*/
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    /*Access functions for the upgradeable stats of a sensor*/
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /*Add a messgae to the list */
    #[no_mangle]
    fn addMessage(msgType: UDWORD, proxPos: BOOL, player: UDWORD)
     -> *mut MESSAGE;
    /* Remove all Messages*/
    /* removes all the proximity displays */
    /*load the view data for the messages from the file exported from the world editor*/
    /*get the view data that contains the text message pointer passed in */
    #[no_mangle]
    fn getViewData(pTextMsg: *mut STRING) -> *mut VIEWDATA;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn flashReticuleButton(buttonID: UDWORD);
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    // Make new command droids available
    #[no_mangle]
    fn cmdDroidAvailable(psBrainStats: *mut BRAIN_STATS, player: SDWORD);
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    static mut apsLimboDroids: [*mut DROID; 8];
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn sendReseachStatus(psBuilding: *mut STRUCTURE, index: UDWORD,
                         player: UBYTE, bStart: BOOL) -> BOOL;
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
// The next free ID
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
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
// The turret mount to use
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
 * ResearchDef.h
 *
 * Structure definitions for research
 *
 */
/* Research struct type definitions */
pub type C2RustUnnamed = libc::c_uint;
pub const TC_MINOR: C2RustUnnamed = 1;
pub const TC_MAJOR: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
/* the 2nd IMD for base plates/turrets*/
// Bit flags   ...  see below
//	UBYTE		possible;				/* Flag to specify whether the research is possible - so
//										   can enable topics vis scripts */
//	UBYTE		researched;				/* Flag to specify whether the research is 
//										   complete	*/
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed_0 = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed_0 = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed_0 = 20;
pub const REF_REARM_PAD: C2RustUnnamed_0 = 19;
pub const REF_LAB: C2RustUnnamed_0 = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed_0 = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed_0 = 16;
pub const REF_DEMOLISH: C2RustUnnamed_0 = 15;
pub const REF_BRIDGE: C2RustUnnamed_0 = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed_0 = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed_0 = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed_0 = 11;
pub const REF_RESEARCH: C2RustUnnamed_0 = 10;
pub const REF_BLASTDOOR: C2RustUnnamed_0 = 9;
pub const REF_WALLCORNER: C2RustUnnamed_0 = 8;
pub const REF_WALL: C2RustUnnamed_0 = 7;
pub const REF_DEFENSE: C2RustUnnamed_0 = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed_0 = 5;
pub const REF_POWER_MODULE: C2RustUnnamed_0 = 4;
pub const REF_POWER_GEN: C2RustUnnamed_0 = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed_0 = 2;
pub const REF_FACTORY: C2RustUnnamed_0 = 1;
pub const REF_HQ: C2RustUnnamed_0 = 0;
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
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
pub type STRUCTURE = _structure;
/* The time the research facility was put on hold*/
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
/*
 * Research.h
 *
 * structures required for research stats
 *
 */
//used for loading in the research stats into the appropriate list
pub type C2RustUnnamed_1 = libc::c_uint;
pub const RES_LIST: C2RustUnnamed_1 = 2;
pub const RED_LIST: C2RustUnnamed_1 = 1;
pub const REQ_LIST: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const RID_MAXRID: C2RustUnnamed_2 = 20;
pub const RID_GRPDAM: C2RustUnnamed_2 = 19;
pub const RID_GRPROF: C2RustUnnamed_2 = 18;
pub const RID_GRPREP: C2RustUnnamed_2 = 17;
pub const RID_GRPUPG: C2RustUnnamed_2 = 16;
pub const RID_GRPACC: C2RustUnnamed_2 = 15;
pub const RID_QUESTIONMARK: C2RustUnnamed_2 = 14;
pub const RID_DEFENCE: C2RustUnnamed_2 = 13;
pub const RID_CYBORGTECH: C2RustUnnamed_2 = 12;
pub const RID_STRUCTURETECH: C2RustUnnamed_2 = 11;
pub const RID_SYSTEMTECH: C2RustUnnamed_2 = 10;
pub const RID_POWERTECH: C2RustUnnamed_2 = 9;
pub const RID_COMPUTERTECH: C2RustUnnamed_2 = 8;
pub const RID_WEAPONTECH: C2RustUnnamed_2 = 7;
pub const RID_DROIDTECH: C2RustUnnamed_2 = 6;
pub const RID_TRACKS: C2RustUnnamed_2 = 5;
pub const RID_PLASCRETE: C2RustUnnamed_2 = 4;
pub const RID_ECM: C2RustUnnamed_2 = 3;
pub const RID_HOVERCRAFT: C2RustUnnamed_2 = 2;
pub const RID_CANNON: C2RustUnnamed_2 = 1;
pub const RID_ROCKET: C2RustUnnamed_2 = 0;
pub type VIEWDATA = _viewdata;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_4 = 461;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_4 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_4 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_4 = 457;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_4 = 460;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_4 = 262;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_4 = 395;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_4 = 349;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_4 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_4 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_4 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_4 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_4 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_4 = 256;
pub const IMAGE_TRACKS: C2RustUnnamed_4 = 252;
pub const IMAGE_PLASCRETE: C2RustUnnamed_4 = 108;
pub const IMAGE_ECM: C2RustUnnamed_4 = 107;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_4 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_4 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_4 = 103;
//name ID of the message - used for loading in and identifying
//the type of view
//the number of textmessages associated with this data
//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
/*
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
// flag to specify a variable reference rather than simple value
/* A value consists of its type and value */
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
/* Opcodes for the script interpreter */
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
/* The type of function called to access an object or in-game variable */
/* The possible storage types for a variable */
// Public variable
// Private variable
// A value stored in an objects data space.
// An external value accessed by function call
// A local variable
/* Variable debugging info for a script */
/* Array info for a script */
// the base index of the array values
// the array data type
/* Array debug info for a script */
/* Line debugging information for a script */
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_3 = 34;
pub type MSG_VIEWDATA = *mut libc::c_void;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
//base structure for each message
pub type MESSAGE = _message;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _message {
    pub type_0: MESSAGE_TYPE,
    pub id: UDWORD,
    pub pViewData: *mut MSG_VIEWDATA,
    pub read: BOOL,
    pub player: UDWORD,
    pub psNext: *mut _message,
}
//The type of message
//ID number of the message
//VIEWDATA		*pViewData;				//Pointer to view data - if any - should be some!
//Pointer to view data - if any - should be some!
//flag to indicate whether message has been read
//which player this message belongs to
//pointer to the next in the list
/*
 * messageDef.h
 *
 * Message structure definitions
 */
pub type MESSAGE_TYPE = _message_type;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_3 = 35;
// Research message
// Campaign message
// Mission Report messages
// Proximity message
/*
 * MissionDef.h
 *
 * Structure definitions for Mission
 *
 */
//mission types
//used to set the reinforcement time on hold whilst the Transporter is unable to land
//hopefully they'll never need to set it this high for other reasons!
//this is used to compare the value passed in from the scripts with which is multiplied by 100
//storage structure for values that need to be kept between missions
pub type MISSION = _mission;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mission {
    pub type_0: UDWORD,
    pub psMapTiles: *mut MAPTILE,
    pub aMapLinePoints: *mut TILE_COORD,
    pub mapWidth: UDWORD,
    pub mapHeight: UDWORD,
    pub psGateways: *mut _gateway,
    pub apRLEZones: *mut *mut UBYTE,
    pub gwNumZones: SDWORD,
    pub aNumEquiv: *mut UBYTE,
    pub apEquivZones: *mut *mut UBYTE,
    pub aZoneReachable: *mut UBYTE,
    pub scrollMinX: UDWORD,
    pub scrollMinY: UDWORD,
    pub scrollMaxX: UDWORD,
    pub scrollMaxY: UDWORD,
    pub apsStructLists: [*mut STRUCTURE; 8],
    pub apsDroidLists: [*mut DROID; 8],
    pub apsFeatureLists: [*mut FEATURE; 8],
    pub apsFlagPosLists: [*mut FLAG_POSITION; 8],
    pub asPower: [PLAYER_POWER; 8],
    pub startTime: UDWORD,
    pub time: SDWORD,
    pub ETA: SDWORD,
    pub cheatTime: UDWORD,
    pub homeLZ_X: UWORD,
    pub homeLZ_Y: UWORD,
    pub playerX: SDWORD,
    pub playerY: SDWORD,
    pub iTranspEntryTileX: [UWORD; 8],
    pub iTranspEntryTileY: [UWORD; 8],
    pub iTranspExitTileX: [UWORD; 8],
    pub iTranspExitTileY: [UWORD; 8],
}
//MISSION_TYPE		type;							//defines which start and end functions to use
//defines which start and end functions to use - see levels_type in levels.h
//the original mapTiles
//the original mapLinePoints
//the original mapWidth
//the original mapHeight
//the gateway list
//the RLE map zones
//the number of map zones
//zone equivalence data
//scroll coords for original map
//original object lists
//struct _proximity_display	*apsProxDisp[MAX_PLAYERS];
//time the mission started
//how long the mission can last
// < 0 = no limit
//time taken for reinforcements to arrive
// < 0 = none allowed
//time the cheating started (mission time-wise!)
//LANDING_ZONE		homeLZ;
//selectedPlayer's LZ x and y
//original view position
/* transporter entry/exit tiles */
/*
 * Power.h
 *
 * Definitions for the Power Functionality.
 *
 */
// free power on collection of oildrum.
//% used to determine the power cost of repairing a droid
                                         //definately DON'T WANT the brackets round 1/2 - it will equate to zero!
//used to multiply all repair calculations by to avaoid rounding errors
//OLD PLAYER_POWER
//typedef struct _player_power
//{
//	UDWORD		availablePower;		/* quantity that can be used from the Generators */
//	UDWORD		extractedPower;		/* quantity being extracted but not converted 
//									   by a Generator */
//	SDWORD		capacity;			/* the spare capacity of the generators */
//	SDWORD		usedPower;			/* quantity currently being used */
//} PLAYER_POWER;
//NEW PLAYER_POWER
pub type PLAYER_POWER = _player_power;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
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
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_3 = 0;
/*#ifndef PSX
	UBYTE			tileExtraBits;	// We've got more than you... We've got more than you..;-)
#endif*/
// COMPRESSED - bit per player
/*#ifndef PSX
	UBYTE			tileDoorBits;   // same thing - bit per player
#endif*/
// The height at the top left of the tile
// How bright is this tile?
// Which graphics texture is on this tile
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
/*
 * Text.h (was Strings.h)
 *
 * String management functions
 *
 */
//the two defines below are MUTUALLY EXCLUSIVE! don't have both defined...
//#define RESOURCE_NAMES
/* ID numbers for all the fixed strings */
pub type _fixed_str_id = libc::c_uint;
pub const STR_MAX_ID: _fixed_str_id = 442;
pub const STR_SEQ_MINIMAL: _fixed_str_id = 441;
pub const STR_SEQ_WINDOW: _fixed_str_id = 440;
pub const STR_SEQ_FULL: _fixed_str_id = 439;
pub const STR_SEQ_PLAYBACK: _fixed_str_id = 438;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_BIND_NORMAL_SPEED: _fixed_str_id = 424;
pub const STR_BIND_SLOW_DOWN: _fixed_str_id = 423;
pub const STR_BIND_SPEED_UP: _fixed_str_id = 422;
pub const STR_BIND_LEFT: _fixed_str_id = 421;
pub const STR_BIND_RIGHT: _fixed_str_id = 420;
pub const STR_BIND_DOWN: _fixed_str_id = 419;
pub const STR_BIND_UP: _fixed_str_id = 418;
pub const STR_BIND_CONSOLE: _fixed_str_id = 417;
pub const STR_BIND_SELCYBORG: _fixed_str_id = 416;
pub const STR_BIND_SELPOWER: _fixed_str_id = 415;
pub const STR_BIND_SELRESEARCH: _fixed_str_id = 414;
pub const STR_BIND_SELFACTORY: _fixed_str_id = 413;
pub const STR_BIND_CMD9: _fixed_str_id = 412;
pub const STR_BIND_CMD8: _fixed_str_id = 411;
pub const STR_BIND_CMD7: _fixed_str_id = 410;
pub const STR_BIND_CMD6: _fixed_str_id = 409;
pub const STR_BIND_CMD5: _fixed_str_id = 408;
pub const STR_BIND_CMD4: _fixed_str_id = 407;
pub const STR_BIND_CMD3: _fixed_str_id = 406;
pub const STR_BIND_CMD2: _fixed_str_id = 405;
pub const STR_BIND_CMD1: _fixed_str_id = 404;
pub const STR_GAM_COMNOTFOUND: _fixed_str_id = 403;
pub const STR_GAM_SENNOTFOUND: _fixed_str_id = 402;
pub const STR_GAM_CONNOTFOUND: _fixed_str_id = 401;
pub const STR_BIND_COMJ: _fixed_str_id = 400;
pub const STR_BIND_SENJ: _fixed_str_id = 399;
// number of string ID's
//added by alex 26-01-99
pub const STR_BIND_CONJ: _fixed_str_id = 398;
pub const STR_FE_CYAN: _fixed_str_id = 397;
pub const STR_FE_PINK: _fixed_str_id = 396;
pub const STR_FE_BLUE: _fixed_str_id = 395;
pub const STR_FE_RED: _fixed_str_id = 394;
pub const STR_FE_BLACK: _fixed_str_id = 393;
pub const STR_FE_GREY: _fixed_str_id = 392;
pub const STR_FE_ORANGE: _fixed_str_id = 391;
pub const STR_FE_GREEN: _fixed_str_id = 390;
pub const STR_FE_OFF: _fixed_str_id = 389;
pub const STR_FE_ON: _fixed_str_id = 388;
pub const STR_FE_MFLIP: _fixed_str_id = 387;
pub const STR_FE_SSHAKE: _fixed_str_id = 386;
pub const STR_SEL_NO_STRUCTURE: _fixed_str_id = 385;
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_BIND_ASIMIL: _fixed_str_id = 383;
pub const STR_BIND_AWHE: _fixed_str_id = 382;
pub const STR_BIND_AVTOL: _fixed_str_id = 381;
pub const STR_BIND_ALL: _fixed_str_id = 380;
pub const STR_BIND_ATR: _fixed_str_id = 379;
pub const STR_BIND_ASCR: _fixed_str_id = 378;
pub const STR_BIND_RECY: _fixed_str_id = 377;
pub const STR_BIND_AHOV: _fixed_str_id = 376;
pub const STR_BIND_AHTR: _fixed_str_id = 375;
pub const STR_BIND_ABDU: _fixed_str_id = 374;
pub const STR_BIND_ACU: _fixed_str_id = 373;
pub const STR_BIND_NDAM: _fixed_str_id = 372;
pub const STR_BIND_HDAM: _fixed_str_id = 371;
pub const STR_BIND_LDAM: _fixed_str_id = 370;
pub const STR_BIND_SCAT: _fixed_str_id = 369;
pub const STR_BIND_LONGR: _fixed_str_id = 368;
pub const STR_BIND_SENDT: _fixed_str_id = 367;
pub const STR_BIND_DSTOP: _fixed_str_id = 366;
pub const STR_BIND_REPA: _fixed_str_id = 365;
pub const STR_BIND_PATR: _fixed_str_id = 364;
pub const STR_BIND_PURS: _fixed_str_id = 363;
pub const STR_BIND_SHOR: _fixed_str_id = 362;
pub const STR_BIND_SPLIM: _fixed_str_id = 361;
pub const STR_BIND_DEFR: _fixed_str_id = 360;
pub const STR_BIND_RTB: _fixed_str_id = 359;
pub const STR_BIND_FAW: _fixed_str_id = 358;
pub const STR_BIND_ENGAG: _fixed_str_id = 357;
pub const STR_BIND_UNITJ: _fixed_str_id = 356;
pub const STR_BIND_CEASE: _fixed_str_id = 355;
pub const STR_BIND_CENTV: _fixed_str_id = 354;
pub const STR_BIND_OVERL: _fixed_str_id = 353;
pub const STR_BIND_REPJ: _fixed_str_id = 352;
pub const STR_BIND_RESJ: _fixed_str_id = 351;
pub const STR_BIND_ORD: _fixed_str_id = 350;
pub const STR_BIND_PB: _fixed_str_id = 349;
pub const STR_BIND_RR: _fixed_str_id = 348;
pub const STR_BIND_RP: _fixed_str_id = 347;
pub const STR_BIND_RL: _fixed_str_id = 346;
pub const STR_BIND_PF: _fixed_str_id = 345;
pub const STR_BIND_ZOUT: _fixed_str_id = 344;
pub const STR_BIND_ZIN: _fixed_str_id = 343;
pub const STR_BIND_ROUT: _fixed_str_id = 342;
pub const STR_BIND_RIN: _fixed_str_id = 341;
pub const STR_BIND_OPT: _fixed_str_id = 340;
pub const STR_BIND_TRACK: _fixed_str_id = 339;
pub const STR_BIND_NORTH: _fixed_str_id = 338;
pub const STR_BIND_AUDOFF: _fixed_str_id = 337;
pub const STR_BIND_AUDON: _fixed_str_id = 336;
pub const STR_BIND_MULOP: _fixed_str_id = 335;
pub const STR_BIND_GR9: _fixed_str_id = 334;
pub const STR_BIND_GR8: _fixed_str_id = 333;
pub const STR_BIND_GR7: _fixed_str_id = 332;
pub const STR_BIND_GR6: _fixed_str_id = 331;
pub const STR_BIND_GR5: _fixed_str_id = 330;
pub const STR_BIND_GR4: _fixed_str_id = 329;
pub const STR_BIND_GR3: _fixed_str_id = 328;
pub const STR_BIND_GR2: _fixed_str_id = 327;
pub const STR_BIND_GR1: _fixed_str_id = 326;
pub const STR_BIND_AS9: _fixed_str_id = 325;
pub const STR_BIND_AS8: _fixed_str_id = 324;
pub const STR_BIND_AS7: _fixed_str_id = 323;
pub const STR_BIND_AS6: _fixed_str_id = 322;
pub const STR_BIND_AS5: _fixed_str_id = 321;
pub const STR_BIND_AS4: _fixed_str_id = 320;
pub const STR_BIND_AS3: _fixed_str_id = 319;
pub const STR_BIND_AS2: _fixed_str_id = 318;
pub const STR_BIND_AS1: _fixed_str_id = 317;
pub const STR_BIND_PREV: _fixed_str_id = 316;
pub const STR_BIND_PAUSE: _fixed_str_id = 315;
pub const STR_BIND_SHOT: _fixed_str_id = 314;
pub const STR_BIND_BARS: _fixed_str_id = 313;
pub const STR_BIND_TOGCON: _fixed_str_id = 312;
pub const STR_BIND_TOGRAD: _fixed_str_id = 311;
pub const STR_BIND_CHOCOM: _fixed_str_id = 310;
pub const STR_BIND_CHOINT: _fixed_str_id = 309;
pub const STR_BIND_CHODES: _fixed_str_id = 308;
pub const STR_BIND_CHOBUI: _fixed_str_id = 307;
pub const STR_BIND_CHORES: _fixed_str_id = 306;
pub const STR_BIND_CHOMAN: _fixed_str_id = 305;
pub const STR_KM_KEYMAP_SIDE: _fixed_str_id = 304;
// keymap editor.
pub const STR_KM_KEYMAP: _fixed_str_id = 303;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
pub const STR_GAME_RESTART: _fixed_str_id = 301;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
pub const STR_GAME_LOADED: _fixed_str_id = 299;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
pub const STR_GAME_REPLAY: _fixed_str_id = 297;
pub const STR_CD_CHANGE_1OR2: _fixed_str_id = 296;
pub const STR_CD_CHANGE: _fixed_str_id = 295;
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const STR_MR_LEVEL_ACE: _fixed_str_id = 284;
pub const STR_MR_LEVEL_SPECIAL: _fixed_str_id = 283;
pub const STR_MR_LEVEL_ELITE: _fixed_str_id = 282;
pub const STR_MR_LEVEL_CRACK: _fixed_str_id = 281;
pub const STR_MR_LEVEL_VETERAN: _fixed_str_id = 280;
pub const STR_MR_LEVEL_PROF: _fixed_str_id = 279;
pub const STR_MR_LEVEL_REGULAR: _fixed_str_id = 278;
pub const STR_MR_LEVEL_TRAINED: _fixed_str_id = 277;
pub const STR_MR_LEVEL_GREEN: _fixed_str_id = 276;
pub const STR_MR_LEVEL_ROOKIE: _fixed_str_id = 275;
pub const STR_MR_BABAS_RUN_OVER: _fixed_str_id = 274;
pub const STR_MR_SHOTS_OFF_TARGET: _fixed_str_id = 273;
pub const STR_MR_SHOTS_ON_TARGET: _fixed_str_id = 272;
pub const STR_MR_GAME_TIME: _fixed_str_id = 271;
pub const STR_MR_MISSION_TIME: _fixed_str_id = 270;
pub const STR_MR_ARTEFACTS_FOUND: _fixed_str_id = 269;
pub const STR_MR_STR_NOW: _fixed_str_id = 268;
pub const STR_MR_STR_LOST: _fixed_str_id = 267;
pub const STR_MR_STR_BLOWN_UP: _fixed_str_id = 266;
pub const STR_MR_STR_BUILT: _fixed_str_id = 265;
pub const STR_MR_AV_UNIT_EL: _fixed_str_id = 264;
pub const STR_MR_UNITS_NOW: _fixed_str_id = 263;
pub const STR_MR_UNITS_LOST: _fixed_str_id = 262;
pub const STR_MR_UNITS_KILLED: _fixed_str_id = 261;
pub const STR_MR_UNITS_BUILT: _fixed_str_id = 260;
pub const STR_MR_RANKINGS: _fixed_str_id = 259;
pub const STR_MR_FORCE_INFO: _fixed_str_id = 258;
pub const STR_MR_STRUCTURE_LOSSES: _fixed_str_id = 257;
pub const STR_MR_UNIT_LOSSES: _fixed_str_id = 256;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GAM_REPNOTFOUND: _fixed_str_id = 245;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const STR_DORD_FACTORY: _fixed_str_id = 219;
pub const STR_DORD_RECYCLE: _fixed_str_id = 218;
pub const STR_DORD_ARMRECYCLE: _fixed_str_id = 217;
pub const STR_DORD_EMBARK: _fixed_str_id = 216;
pub const STR_DORD_RETBASE: _fixed_str_id = 215;
pub const STR_DORD_RETREPAIR: _fixed_str_id = 214;
pub const STR_DORD_HOLDPOS: _fixed_str_id = 213;
pub const STR_DORD_GUARD: _fixed_str_id = 212;
pub const STR_DORD_PERSUE: _fixed_str_id = 211;
pub const STR_DORD_PATROL: _fixed_str_id = 210;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_MUL_RESPOND: _fixed_str_id = 200;
pub const STR_MUL_JOINING: _fixed_str_id = 199;
pub const STR_MUL_LEAVE: _fixed_str_id = 198;
pub const STR_HARD: _fixed_str_id = 197;
pub const STR_NORMAL: _fixed_str_id = 196;
pub const STR_EASY: _fixed_str_id = 195;
pub const STR_FE_DIFFICULTY: _fixed_str_id = 194;
pub const STR_FE_TRANSPARENCY: _fixed_str_id = 193;
pub const STR_FE_FOG: _fixed_str_id = 192;
pub const STR_FE_EFFECTS: _fixed_str_id = 191;
pub const STR_FE_TEXTURE: _fixed_str_id = 190;
pub const STR_FE_AUDIO: _fixed_str_id = 189;
pub const STR_FE_GRAPHICS: _fixed_str_id = 188;
pub const STR_FE_GAME: _fixed_str_id = 187;
pub const STR_FE_GLIDE: _fixed_str_id = 186;
pub const STR_FE_OPENGL: _fixed_str_id = 185;
pub const STR_FE_DIRECTX: _fixed_str_id = 184;
pub const STR_FE_SOFTWARE: _fixed_str_id = 183;
pub const STR_FE_VIDEO: _fixed_str_id = 182;
pub const STR_FE_GOODFOG: _fixed_str_id = 181;
pub const STR_FE_CRAPFOG: _fixed_str_id = 180;
pub const STR_FE_CLAN: _fixed_str_id = 179;
pub const STR_FE_MUSIC: _fixed_str_id = 178;
pub const STR_FE_3D_FX: _fixed_str_id = 177;
pub const STR_FE_FX: _fixed_str_id = 176;
pub const STR_FE_GAMMA: _fixed_str_id = 175;
pub const STR_FE_SCROLL: _fixed_str_id = 174;
pub const STR_FE_MOUSE: _fixed_str_id = 173;
pub const STR_FE_SIDEOPTIONS: _fixed_str_id = 172;
pub const STR_FE_SKIRMISH: _fixed_str_id = 171;
pub const STR_FE_FORCEEDIT: _fixed_str_id = 170;
pub const STR_FE_JOIN: _fixed_str_id = 169;
pub const STR_FE_HOST: _fixed_str_id = 168;
pub const STR_FE_SIDEMULTI: _fixed_str_id = 167;
pub const STR_FE_RETURN: _fixed_str_id = 166;
pub const STR_FE_FASTPLAY: _fixed_str_id = 165;
pub const STR_FE_TUT: _fixed_str_id = 164;
pub const STR_FE_LOAD: _fixed_str_id = 163;
pub const STR_FE_NEW: _fixed_str_id = 162;
pub const STR_FE_SIDETUT: _fixed_str_id = 161;
pub const STR_FE_SIDESINGLE2: _fixed_str_id = 160;
pub const STR_FE_SIDESINGLE1: _fixed_str_id = 159;
pub const STR_FE_QUIT: _fixed_str_id = 158;
pub const STR_FE_INTRO: _fixed_str_id = 157;
pub const STR_FE_OPTIONS: _fixed_str_id = 156;
pub const STR_FE_MULTI: _fixed_str_id = 155;
pub const STR_FE_SINGLE: _fixed_str_id = 154;
// FrontEnd STrings
pub const STR_FE_SIDEMAIN: _fixed_str_id = 153;
pub const STR_GAME_RESUME: _fixed_str_id = 152;
//ingame ops.
pub const STR_GAME_QUIT: _fixed_str_id = 151;
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
pub const STR_COMPATIBLE: _fixed_str_id = 148;
pub const STR_MUL_ARTIF: _fixed_str_id = 147;
pub const STR_GIFT_POW: _fixed_str_id = 146;
pub const STR_GIFT_TEC: _fixed_str_id = 145;
pub const STR_GIFT_DRO: _fixed_str_id = 144;
pub const STR_GIFT_VIS: _fixed_str_id = 143;
pub const STR_ALLI_FRM: _fixed_str_id = 142;
pub const STR_ALLI_BRK: _fixed_str_id = 141;
pub const STR_ALLI_OFF: _fixed_str_id = 140;
pub const STR_ALLI_REQ: _fixed_str_id = 139;
pub const STR_ALLI_POW: _fixed_str_id = 138;
pub const STR_ALLI_DRO: _fixed_str_id = 137;
pub const STR_ALLI_TEC: _fixed_str_id = 136;
pub const STR_ALLI_VIS: _fixed_str_id = 135;
pub const STR_ALLI_STATE: _fixed_str_id = 134;
pub const STR_MUL_FOG_OFF: _fixed_str_id = 133;
pub const STR_MUL_FOG_ON: _fixed_str_id = 132;
pub const STR_MUL_TEAMPLAY: _fixed_str_id = 131;
pub const STR_MUL_SKIRMISH: _fixed_str_id = 130;
pub const STR_MUL_STRLIM: _fixed_str_id = 129;
pub const STR_MUL_COMP_N: _fixed_str_id = 128;
pub const STR_MUL_COMP_Y: _fixed_str_id = 127;
pub const STR_MUL_COMP: _fixed_str_id = 126;
pub const STR_MUL_PLAY: _fixed_str_id = 125;
pub const STR_MUL_PLAYERS: _fixed_str_id = 124;
pub const STR_LABEL_FOG: _fixed_str_id = 123;
pub const STR_LABEL_LIMIT: _fixed_str_id = 122;
pub const STR_LABEL_BASE: _fixed_str_id = 121;
pub const STR_LABEL_TEC: _fixed_str_id = 120;
pub const STR_LABEL_ALLI: _fixed_str_id = 119;
pub const STR_LABEL_TYPE: _fixed_str_id = 118;
pub const STR_GAMES_GAMES: _fixed_str_id = 117;
pub const STR_CON_MORE: _fixed_str_id = 116;
pub const STR_CON_CABLE: _fixed_str_id = 115;
pub const STR_CON_LAN: _fixed_str_id = 114;
pub const STR_CON_INTERNET: _fixed_str_id = 113;
pub const STR_CON_MODEM: _fixed_str_id = 112;
pub const STR_MUL_FRAGLIM: _fixed_str_id = 111;
pub const STR_MUL_TIMELIM: _fixed_str_id = 110;
pub const STR_MUL_NOLIM: _fixed_str_id = 109;
pub const STR_MUL_ALLIANCEY: _fixed_str_id = 108;
pub const STR_MUL_ALLIANCEN: _fixed_str_id = 107;
pub const STR_MUL_GAMEIC: _fixed_str_id = 106;
pub const STR_MUL_MAPIC: _fixed_str_id = 105;
pub const STR_MUL_FORCEIC: _fixed_str_id = 104;
pub const STR_MUL_PLAYERIC: _fixed_str_id = 103;
pub const STR_MUL_CAMPBASE: _fixed_str_id = 102;
pub const STR_MUL_CAMPDEFENCE: _fixed_str_id = 101;
pub const STR_MUL_CAMPCLEAN: _fixed_str_id = 100;
pub const STR_MUL_TECH3: _fixed_str_id = 99;
pub const STR_MUL_TECH2: _fixed_str_id = 98;
pub const STR_MUL_TECH1: _fixed_str_id = 97;
pub const STR_MUL_POWHI: _fixed_str_id = 96;
pub const STR_MUL_POWMED: _fixed_str_id = 95;
pub const STR_MUL_POWLO: _fixed_str_id = 94;
pub const STR_MUL_PING: _fixed_str_id = 93;
pub const STR_MUL_KILLS: _fixed_str_id = 92;
pub const STR_MUL_SCORE: _fixed_str_id = 91;
pub const STR_MUL_ALLIANCES: _fixed_str_id = 90;
pub const STR_MUL_STARTING: _fixed_str_id = 89;
pub const STR_MUL_CHAT: _fixed_str_id = 88;
pub const STR_MUL_SIDEINFO: _fixed_str_id = 87;
pub const STR_MUL_SIDETEMPLATES: _fixed_str_id = 86;
pub const STR_MUL_SIDEFORCE: _fixed_str_id = 85;
pub const STR_MUL_SIDEPLAYERS: _fixed_str_id = 84;
pub const STR_MUL_SIDEGAMES: _fixed_str_id = 83;
pub const STR_MUL_SIDEOPTIONS: _fixed_str_id = 82;
pub const STR_MUL_SIDECONNECTION: _fixed_str_id = 81;
pub const STR_MUL_115200: _fixed_str_id = 80;
pub const STR_MUL_56000: _fixed_str_id = 79;
pub const STR_MUL_19200: _fixed_str_id = 78;
pub const STR_MUL_14400: _fixed_str_id = 77;
pub const STR_MUL_SEARCHING: _fixed_str_id = 76;
pub const STR_MUL_DESIGN: _fixed_str_id = 75;
pub const STR_MUL_SAVE: _fixed_str_id = 74;
pub const STR_MUL_LOAD: _fixed_str_id = 73;
pub const STR_MUL_DEFAULT: _fixed_str_id = 72;
pub const STR_MUL_CLEAR: _fixed_str_id = 71;
pub const STR_MUL_HOST: _fixed_str_id = 70;
//	STR_MUL_RESLO,	
//	STR_MUL_RESMED,		
//	STR_MUL_RESHI,		
//	STR_MUL_HELPON,	
//	STR_MUL_HELPOFF,		
pub const STR_MUL_REFRESH: _fixed_str_id = 69;
pub const STR_MUL_CAMPAIGN: _fixed_str_id = 68;
pub const STR_MUL_ARENA: _fixed_str_id = 67;
pub const STR_MUL_MAXPLAY: _fixed_str_id = 66;
pub const STR_MUL_GAME: _fixed_str_id = 65;
pub const STR_MUL_PLAYER: _fixed_str_id = 64;
pub const STR_MUL_OK: _fixed_str_id = 63;
pub const STR_MUL_CANCEL: _fixed_str_id = 62;
pub const STR_MUL_COM4: _fixed_str_id = 61;
pub const STR_MUL_COM3: _fixed_str_id = 60;
pub const STR_MUL_COM2: _fixed_str_id = 59;
pub const STR_MUL_COM1: _fixed_str_id = 58;
pub const STR_MUL_IPADDR: _fixed_str_id = 57;
// multiplayer strings
pub const STR_MUL_PHONENO: _fixed_str_id = 56;
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
// Reticule strings
pub const STR_RET_OPTIONS: _fixed_str_id = 1;
// The default (id == 0) string
pub const STR_DEFAULT: _fixed_str_id = 0;
pub type C2RustUnnamed_3 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_3 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_3 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_3 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_3 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_3 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_3 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_3 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_3 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_3 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_3 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_3 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_3 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_3 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_3 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_3 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_3 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_3 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_3 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_3 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_3 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_3 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_3 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_3 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_3 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_3 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_3 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_3 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_3 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_3 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_3 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_3 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_3 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_3 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_3 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_3 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_3 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_3 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_3 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_3 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_3 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_3 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_3 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_3 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_3 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_3 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_3 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_3 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_3 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_3 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_3 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_3 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_3 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_3 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_3 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_3 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_3 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_3 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_3 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_3 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_3 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_3 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_3 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_3 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_3 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_3 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_3 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_3 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_3 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_3 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_3 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_3 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_3 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_3 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_3 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_3 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_3 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_3 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_3 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_3 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_3 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_3 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_3 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_3 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_3 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_3 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_3 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_3 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_3 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_3 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_3 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_3 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_3 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_3 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_3 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_3 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_3 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_3 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_3 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_3 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_3 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_3 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_3 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_3 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_3 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_3 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_3 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_3 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_3 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_3 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_3 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_3 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_3 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_3 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_3 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_3 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_3 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_3 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_3 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_3 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_3 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_3 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_3 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_3 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_3 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_3 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_3 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_3 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_3 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_3 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_3 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_3 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_3 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_3 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_3 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_3 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_3 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_3 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_3 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_3 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_3 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_3 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_3 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_3 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_3 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_3 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_3 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_3 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_3 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_3 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_3 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_3 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_3 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_3 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_3 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_3 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_3 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_3 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_3 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_3 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_3 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_3 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_3 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_3 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_3 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_3 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_3 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_3 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_3 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_3 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_3 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_3 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_3 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_3 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_3 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_3 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_3 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_3 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_3 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_3 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_3 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_3 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_3 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_3 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_3 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_3 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_3 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_3 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_3 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_3 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_3 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_3 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_3 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_3 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_3 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_3 = 160;
pub const ID_GIFT: C2RustUnnamed_3 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_3 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_3 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_3 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_3 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_3 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_3 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_3 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_3 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_3 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_3 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_3 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_3 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_3 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_3 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_3 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_3 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_3 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_3 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_3 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_3 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_3 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_3 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_3 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_3 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_3 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_3 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_3 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_3 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_3 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_3 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_3 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_3 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_3 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_3 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_3 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_3 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_3 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_3 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_3 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_3 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_3 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_3 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_3 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_3 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_3 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_3 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_3 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_3 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_3 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_3 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_3 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_3 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_3 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_3 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_3 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_3 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_3 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_3 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_3 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_3 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_3 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_3 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_3 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_3 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_3 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_3 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_3 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_3 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_3 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_3 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_3 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_3 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_3 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_3 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_3 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_3 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_3 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_3 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_3 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_3 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_3 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_3 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_3 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_3 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_3 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_3 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_3 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_3 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_3 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_3 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_3 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_3 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_3 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_3 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_3 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_3 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_3 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_3 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_3 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_3 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_3 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_3 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_3 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_3 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_3 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_3 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_3 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_3 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_3 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_3 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_3 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_3 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_3 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_3 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_3 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_3 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_3 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_3 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_3 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_3 = 36;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_3 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_3 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_3 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_3 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_3 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_3 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_3 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_3 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_3 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_3 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_3 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_3 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_3 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_3 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_3 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_3 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_3 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_3 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_3 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_3 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_3 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_3 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_3 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_3 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_3 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_3 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_3 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_3 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_3 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_3 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_3 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_3 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_3 = 1;
pub const NO_SOUND: C2RustUnnamed_3 = -1;
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
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_4 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_4 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_4 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_4 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_4 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_4 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_4 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_4 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_4 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_4 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_4 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_4 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_4 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_4 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_4 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_4 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_4 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_4 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_4 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_4 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_4 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_4 = 462;
pub const IMAGE_ASCII131: C2RustUnnamed_4 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_4 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_4 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_4 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_4 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_4 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_4 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_4 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_4 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_4 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_4 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_4 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_4 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_4 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_4 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_4 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_4 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_4 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_4 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_4 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_4 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_4 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_4 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_4 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_4 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_4 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_4 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_4 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_4 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_4 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_4 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_4 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_4 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_4 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_4 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_4 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_4 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_4 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_4 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_4 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_4 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_4 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_4 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_4 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_4 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_4 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_4 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_4 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_4 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_4 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_4 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_4 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_4 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_4 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_4 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_4 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_4 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_4 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_4 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_4 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_4 = 396;
pub const IMAGE_LEV_7: C2RustUnnamed_4 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_4 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_4 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_4 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_4 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_4 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_4 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_4 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_4 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_4 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_4 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_4 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_4 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_4 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_4 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_4 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_4 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_4 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_4 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_4 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_4 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_4 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_4 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_4 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_4 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_4 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_4 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_4 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_4 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_4 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_4 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_4 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_4 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_4 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_4 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_4 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_4 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_4 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_4 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_4 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_4 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_4 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_4 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_4 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_4 = 350;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_4 = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_4 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_4 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_4 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_4 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_4 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_4 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_4 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_4 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_4 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_4 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_4 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_4 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_4 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_4 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_4 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_4 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_4 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_4 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_4 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_4 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_4 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_4 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_4 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_4 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_4 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_4 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_4 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_4 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_4 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_4 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_4 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_4 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_4 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_4 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_4 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_4 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_4 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_4 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_4 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_4 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_4 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_4 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_4 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_4 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_4 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_4 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_4 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_4 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_4 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_4 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_4 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_4 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_4 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_4 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_4 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_4 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_4 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_4 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_4 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_4 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_4 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_4 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_4 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_4 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_4 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_4 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_4 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_4 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_4 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_4 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_4 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_4 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_4 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_4 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_4 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_4 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_4 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_4 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_4 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_4 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_4 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_4 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_4 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_4 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_4 = 263;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_4 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_4 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_4 = 253;
pub const IMAGE_STAR: C2RustUnnamed_4 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_4 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_4 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_4 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_4 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_4 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_4 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_4 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_4 = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_4 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_4 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_4 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_4 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_4 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_4 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_4 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_4 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_4 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_4 = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_4 = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_4 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_4 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_4 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_4 = 228;
pub const IMAGE_ASCII126: C2RustUnnamed_4 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_4 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_4 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_4 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_4 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_4 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_4 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_4 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_4 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_4 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_4 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_4 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_4 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_4 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_4 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_4 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_4 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_4 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_4 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_4 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_4 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_4 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_4 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_4 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_4 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_4 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_4 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_4 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_4 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_4 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_4 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_4 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_4 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_4 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_4 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_4 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_4 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_4 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_4 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_4 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_4 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_4 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_4 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_4 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_4 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_4 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_4 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_4 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_4 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_4 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_4 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_4 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_4 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_4 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_4 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_4 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_4 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_4 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_4 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_4 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_4 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_4 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_4 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_4 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_4 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_4 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_4 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_4 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_4 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_4 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_4 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_4 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_4 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_4 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_4 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_4 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_4 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_4 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_4 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_4 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_4 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_4 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_4 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_4 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_4 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_4 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_4 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_4 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_4 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_4 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_4 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_4 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_4 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_4 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_4 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_4 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_4 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_4 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_4 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_4 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_4 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_4 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_4 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_4 = 124;
pub const IMAGE_9: C2RustUnnamed_4 = 123;
pub const IMAGE_8: C2RustUnnamed_4 = 122;
pub const IMAGE_7: C2RustUnnamed_4 = 121;
pub const IMAGE_6: C2RustUnnamed_4 = 120;
pub const IMAGE_5: C2RustUnnamed_4 = 119;
pub const IMAGE_4: C2RustUnnamed_4 = 118;
pub const IMAGE_3: C2RustUnnamed_4 = 117;
pub const IMAGE_2: C2RustUnnamed_4 = 116;
pub const IMAGE_1: C2RustUnnamed_4 = 115;
pub const IMAGE_0: C2RustUnnamed_4 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_4 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_4 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_4 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_4 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_4 = 109;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_4 = 106;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_4 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_4 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_4 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_4 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_4 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_4 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_4 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_4 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_4 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_4 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_4 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_4 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_4 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_4 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_4 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_4 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_4 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_4 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_4 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_4 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_4 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_4 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_4 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_4 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_4 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_4 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_4 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_4 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_4 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_4 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_4 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_4 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_4 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_4 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_4 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_4 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_4 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_4 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_4 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_4 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_4 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_4 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_4 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_4 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_4 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_4 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_4 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_4 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_4 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_4 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_4 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_4 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_4 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_4 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_4 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_4 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_4 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_4 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_4 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_4 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_4 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_4 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_4 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_4 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_4 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_4 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_4 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_4 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_4 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_4 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_4 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_4 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_4 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_4 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_4 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_4 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_4 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_4 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_4 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_4 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_4 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_4 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_4 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_4 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_4 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_4 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_4 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_4 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_4 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_4 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_4 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_4 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_4 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_4 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_4 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_4 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_4 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_4 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_4 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_4 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_4 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_4 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_4 = 0;
// The stores for the research stats
#[no_mangle]
pub static mut asResearch: *mut RESEARCH =
    0 as *const RESEARCH as *mut RESEARCH;
#[no_mangle]
pub static mut numResearch: UDWORD = 0;
//used for Callbacks to say which topic was last researched
#[no_mangle]
pub static mut psCBLastResearch: *mut RESEARCH =
    0 as *const RESEARCH as *mut RESEARCH;
//need corresponding arrays for the above
//needs to be a UWORD* for the Patches
#[no_mangle]
pub static mut pResearchPR: *mut UWORD = 0 as *const UWORD as *mut UWORD;
//UBYTE               *pResearchPR;
#[no_mangle]
pub static mut pResearchStructPR: *mut UWORD =
    0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut pResearchFunc: *mut *mut FUNCTION =
    0 as *const *mut FUNCTION as *mut *mut FUNCTION;
#[no_mangle]
pub static mut pResearchStructRed: *mut UWORD =
    0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut pResearchArteRed: *mut *mut COMP_BASE_STATS =
    0 as *const *mut COMP_BASE_STATS as *mut *mut COMP_BASE_STATS;
#[no_mangle]
pub static mut pResearchStructRes: *mut UWORD =
    0 as *const UWORD as *mut UWORD;
#[no_mangle]
pub static mut pResearchArteRes: *mut *mut COMP_BASE_STATS =
    0 as *const *mut COMP_BASE_STATS as *mut *mut COMP_BASE_STATS;
#[no_mangle]
pub static mut pResearchArteRep: *mut *mut COMP_BASE_STATS =
    0 as *const *mut COMP_BASE_STATS as *mut *mut COMP_BASE_STATS;
#[no_mangle]
pub static mut numResearchPR: UWORD = 0;
#[no_mangle]
pub static mut numResearchStructPR: UWORD = 0;
#[no_mangle]
pub static mut numResearchFunc: UWORD = 0;
#[no_mangle]
pub static mut numResearchStructRed: UWORD = 0;
#[no_mangle]
pub static mut numResearchArteRed: UBYTE = 0;
#[no_mangle]
pub static mut numResearchStructRes: UWORD = 0;
#[no_mangle]
pub static mut numResearchArteRes: UBYTE = 0;
#[no_mangle]
pub static mut numResearchArteRep: UBYTE = 0;
//List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
#[no_mangle]
pub static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8] =
    [0 as *const PLAYER_RESEARCH as *mut PLAYER_RESEARCH; 8];
/* Default level of sensor, Repair and ECM */
#[no_mangle]
pub static mut aDefaultSensor: [UDWORD; 8] = [0; 8];
#[no_mangle]
pub static mut aDefaultECM: [UDWORD; 8] = [0; 8];
#[no_mangle]
pub static mut aDefaultRepair: [UDWORD; 8] = [0; 8];
//static void enableSelfRepair(UBYTE player);
#[no_mangle]
pub unsafe extern "C" fn getResearchName(mut pResearch: *mut RESEARCH)
 -> *mut libc::c_char {
    return getName((*pResearch).pName);
}
//flag that indicates whether the player can self repair
static mut bSelfRepair: [UBYTE; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn researchInitVars() -> BOOL {
    let mut i: libc::c_int = 0;
    psCBLastResearch = 0 as *mut RESEARCH;
    asResearch = 0 as *mut RESEARCH;
    //research is a pre-defined size now
    asResearch =
        memMallocRelease((::std::mem::size_of::<RESEARCH>() as
                              libc::c_ulong).wrapping_mul(450 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut RESEARCH;
    if asResearch.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(asResearch as *mut libc::c_void, 0 as libc::c_int,
           (450 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<RESEARCH>()
                                               as libc::c_ulong));
    //create the PLAYER_RESEARCH arrays
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        asPlayerResList[i as usize] =
            memMallocRelease((450 as libc::c_int as
                                  libc::c_uint).wrapping_mul(::std::mem::size_of::<PLAYER_RESEARCH>()
                                                                 as
                                                                 libc::c_ulong))
                as *mut PLAYER_RESEARCH;
        if asPlayerResList[i as usize].is_null() {
            debug(LOG_ERROR,
                  b"Out of memory assigning Player_Research\x00" as *const u8
                      as *const libc::c_char);
            abort();
        }
        memset(asPlayerResList[i as usize] as *mut libc::c_void,
               0 as libc::c_int,
               (450 as libc::c_int as
                    libc::c_uint).wrapping_mul(::std::mem::size_of::<PLAYER_RESEARCH>()
                                                   as libc::c_ulong));
        i += 1
    }
    numResearch = 0 as libc::c_int as UDWORD;
    //and deal with all the other arrays for research
    //needs to be UWORD sized for the Patches
    pResearchPR =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul(650 as libc::c_int
                                                              as
                                                              libc::c_uint))
            as *mut UWORD;
    //pResearchPR = (UBYTE *) MALLOC(sizeof(UBYTE) * MAX_RESEARCH_PR);
    if pResearchPR.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //needs to be UWORD sized for the Patches
    memset(pResearchPR as *mut libc::c_void, 0 as libc::c_int,
           (650 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    //memset(pResearchPR, 0, (MAX_RESEARCH_PR * sizeof(UBYTE)));
    pResearchStructPR =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul((44 as libc::c_int +
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut UWORD;
    if pResearchStructPR.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchStructPR as *mut libc::c_void, 0 as libc::c_int,
           ((44 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    pResearchFunc =
        memMallocRelease((::std::mem::size_of::<*mut FUNCTION>() as
                              libc::c_ulong).wrapping_mul((250 as libc::c_int
                                                               +
                                                               25 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut *mut FUNCTION;
    if pResearchFunc.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchFunc as *mut libc::c_void, 0 as libc::c_int,
           ((250 as libc::c_int + 25 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut FUNCTION>()
                                               as libc::c_ulong));
    pResearchStructRed =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul((30 as libc::c_int +
                                                               2 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut UWORD;
    if pResearchStructRed.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchStructRed as *mut libc::c_void, 0 as libc::c_int,
           ((30 as libc::c_int + 2 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    pResearchArteRed =
        memMallocRelease((::std::mem::size_of::<*mut COMP_BASE_STATS>() as
                              libc::c_ulong).wrapping_mul((40 as libc::c_int +
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut *mut COMP_BASE_STATS;
    if pResearchArteRed.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchArteRed as *mut libc::c_void, 0 as libc::c_int,
           ((40 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    pResearchStructRes =
        memMallocRelease((::std::mem::size_of::<UWORD>() as
                              libc::c_ulong).wrapping_mul((84 as libc::c_int +
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut UWORD;
    if pResearchStructRes.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchStructRes as *mut libc::c_void, 0 as libc::c_int,
           ((84 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    pResearchArteRes =
        memMallocRelease((::std::mem::size_of::<*mut COMP_BASE_STATS>() as
                              libc::c_ulong).wrapping_mul((125 as libc::c_int
                                                               +
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut *mut COMP_BASE_STATS;
    if pResearchArteRes.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchArteRes as *mut libc::c_void, 0 as libc::c_int,
           ((125 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    pResearchArteRep =
        memMallocRelease((::std::mem::size_of::<*mut COMP_BASE_STATS>() as
                              libc::c_ulong).wrapping_mul((125 as libc::c_int
                                                               +
                                                               5 as
                                                                   libc::c_int)
                                                              as
                                                              libc::c_uint))
            as *mut *mut COMP_BASE_STATS;
    if pResearchArteRep.is_null() {
        debug(LOG_ERROR,
              b"Research Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    memset(pResearchArteRep as *mut libc::c_void, 0 as libc::c_int,
           ((125 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        //asPlayerResList[i] = NULL;
        bSelfRepair[i as usize] = 0 as libc::c_int as UBYTE;
        i += 1
    }
    return 1 as libc::c_int;
}
/* The store for the research stats */
//List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
//used for Callbacks to say which topic was last researched
/* Default level of sensor, repair and ECM */
//extern BOOL loadResearch(void);
/*Load the research stats from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadResearch(mut pResearchData: *mut libc::c_char,
                                      mut bufferSize: UDWORD) -> BOOL {
    let mut pStartResearchData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut psComp: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut structID: SDWORD = 0;
    let mut researchCount: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut keyTopic: UDWORD = 0;
    let mut techCode: UDWORD = 0;
    let mut resPoints: UDWORD = 0;
    let mut ResearchName: [STRING; 60] = [0; 60];
    let mut msgName: [STRING; 20] = [0; 20];
    let mut iconID: [STRING; 60] = [0; 60];
    let mut imdName: [STRING; 60] = [0; 60];
    let mut imdName2: [STRING; 60] = [0; 60];
    let mut structName: [STRING; 60] = [0; 60];
    let mut compName: [STRING; 60] = [0; 60];
    let mut compType: [STRING; 20] = [0; 20];
    //reserve the start of the data
    pStartResearchData = pResearchData;
    researchCount = numCR(pResearchData, bufferSize);
    /*asResearch = (RESEARCH *)MALLOC(sizeof(RESEARCH)*researchCount);
	if (asResearch == NULL)
	{
		DBERROR(("Research Stats - Out of memory"));
		return FALSE;
	}*/
    numResearch = researchCount;
    //ASSERT( (numResearch) < REF_RANGE, "Too many ResearchStats!!" );
    if numResearch <= 450 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Too many ResearchStats!! - max allowed %d\x00" as *const u8 as
                  *const libc::c_char, 450 as libc::c_int);
    };
    if numResearch <= 450 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              292 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"loadResearch\x00")).as_ptr(),
              b"(numResearch) <= MAX_RESEARCH\x00" as *const u8 as
                  *const libc::c_char);
    };
    //init all the counts
    numResearchArteRep = 0 as libc::c_int as UBYTE;
    numResearchArteRes = numResearchArteRep;
    numResearchArteRed = numResearchArteRes;
    numResearchFunc = numResearchArteRed as UWORD;
    numResearchPR = numResearchFunc;
    numResearchStructRes = 0 as libc::c_int as UWORD;
    numResearchStructRed = numResearchStructRes;
    numResearchStructPR = numResearchStructRed;
    //get the start of the research storage
    pResearch = asResearch;
    i = 0 as libc::c_int as UDWORD;
    while i < researchCount {
        memset(pResearch as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<RESEARCH>() as libc::c_ulong);
        //read the data into the storage - the data is delimeted using comma's
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pResearchData,
               b"%[^\',\'],\x00" as *const u8 as *const libc::c_char,
               ResearchName.as_mut_ptr());
        //allocate storage for the name
        if allocateName(&mut (*pResearch).pName, ResearchName.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //check the name hasn't been used already
        if checkResearchName(pResearch, i) == 0 { return 0 as libc::c_int }
        pResearchData =
            pResearchData.offset(strlen(ResearchName.as_mut_ptr()).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                     as isize);
        (*pResearch).ref_0 =
            (0xb0000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the tech level
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pResearchData,
               b"%[^\',\'],\x00" as *const u8 as *const libc::c_char,
               ResearchName.as_mut_ptr());
        if setTechLevel(pResearch as *mut BASE_STATS,
                        ResearchName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        pResearchData =
            pResearchData.offset(strlen(ResearchName.as_mut_ptr()).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                     as isize);
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pResearchData,
               b"%[^\',\'],\x00" as *const u8 as *const libc::c_char,
               ResearchName.as_mut_ptr());
        //subGroup value now holds which category the research comes under for yet another icon!
		// store subgroup. may differ from tech level at some point?
		/*if(strcmp(ResearchName,"Sub Group One") == 0)
		{
			pResearch->subGroup	= 1;
		}
		else if(strcmp(ResearchName,"Sub Group Two") == 0)
		{
			pResearch->subGroup	= 2;
		}
		else if(strcmp(ResearchName,"Sub Group Three") == 0)
		{
			pResearch->subGroup	= 3;
		}
		// now support for future 'add on packs' (hey I can be hopeful).
		else if(strcmp(ResearchName,"Sub Group Four") == 0)
		{
			pResearch->subGroup	= 4;
		}
		else if(strcmp(ResearchName,"Sub Group Five") == 0)
		{
			pResearch->subGroup	= 5;
		}
		else if(strcmp(ResearchName,"Sub Group Six") == 0)
		{
			pResearch->subGroup	= 6;
		}
		else if(strcmp(ResearchName,"Sub Group Seven") == 0)
		{
			pResearch->subGroup	= 7;
		}
		else if(strcmp(ResearchName,"Sub Group Eight") == 0)
		{
			pResearch->subGroup	= 8;
		}
		else if(strcmp(ResearchName,"Sub Group Nine") == 0)
		{
			pResearch->subGroup	= 9;
		}
		else
		{
			DBERROR(("Unknown Research Subgroup."));
		}*/
        if strcmp(ResearchName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*pResearch).subGroup =
                setIconID(ResearchName.as_mut_ptr(), (*pResearch).pName)
        } else { (*pResearch).subGroup = 0 as libc::c_int as UWORD }
        pResearchData =
            pResearchData.offset(strlen(ResearchName.as_mut_ptr()).wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                     as isize);
        iconID[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        imdName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        imdName2[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        msgName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        structName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        compName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        compType[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        let mut numPRRequired: UDWORD = 0;
        let mut numFunctions_0: UDWORD = 0;
        let mut numStructures: UDWORD = 0;
        let mut numRedStructs: UDWORD = 0;
        let mut numStructResults: UDWORD = 0;
        let mut numRedArtefacts: UDWORD = 0;
        let mut numArteResults: UDWORD = 0;
        sscanf(pResearchData,
               b"%d,%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],                 %[^\',\'],%[^\',\'],%d,%d,%d,%d,%d,%d,%d,%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               &mut techCode as *mut UDWORD, iconID.as_mut_ptr(),
               imdName.as_mut_ptr(), imdName2.as_mut_ptr(),
               msgName.as_mut_ptr(), structName.as_mut_ptr(),
               compName.as_mut_ptr(), compType.as_mut_ptr(),
               &mut resPoints as *mut UDWORD, &mut keyTopic as *mut UDWORD,
               &mut numPRRequired as *mut UDWORD,
               &mut numFunctions_0 as *mut UDWORD,
               &mut numStructures as *mut UDWORD,
               &mut numRedStructs as *mut UDWORD,
               &mut numStructResults as *mut UDWORD,
               &mut numRedArtefacts as *mut UDWORD,
               &mut numArteResults as *mut UDWORD);
        (*pResearch).numPRRequired = numPRRequired as UBYTE;
        (*pResearch).numFunctions = numFunctions_0 as UBYTE;
        (*pResearch).numStructures = numStructures as UBYTE;
        (*pResearch).numRedStructs = numRedStructs as UBYTE;
        (*pResearch).numStructResults = numStructResults as UBYTE;
        (*pResearch).numRedArtefacts = numRedArtefacts as UBYTE;
        (*pResearch).numArteResults = numArteResults as UBYTE;
        //set keytopic flag
        if keyTopic != 0 {
            (*pResearch).keyTopic = 1 as libc::c_int as UBYTE
        } else { (*pResearch).keyTopic = 0 as libc::c_int as UBYTE }
        //check the tech code is valid
        if techCode > 1 as libc::c_int as libc::c_uint {
            //if (pResearch->techCode != TC_MAJOR AND pResearch->techCode != TC_MINOR)
            debug(LOG_ERROR,
                  b"Invalid tech code for research topic - %s \x00" as
                      *const u8 as *const libc::c_char,
                  getResearchName(pResearch));
            abort();
        }
        if techCode == 0 as libc::c_int as libc::c_uint {
            (*pResearch).techCode = TC_MAJOR as libc::c_int as UBYTE
        } else { (*pResearch).techCode = TC_MINOR as libc::c_int as UBYTE }
        //set the iconID
        if strcmp(iconID.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*pResearch).iconID =
                setIconID(iconID.as_mut_ptr(), (*pResearch).pName)
        } else { (*pResearch).iconID = 0 as libc::c_int as UWORD }
        //get the IMDs used in the interface
        if strcmp(structName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            //find the structure stat
            structID = getStructStatFromName(structName.as_mut_ptr());
            if structID >= 0 as libc::c_int {
                (*pResearch).psStat =
                    asStructureStats.offset(structID as isize) as
                        *mut BASE_STATS
            } else {
                debug(LOG_ERROR,
                      b"Cannot find the structure Stat for Research %s\x00" as
                          *const u8 as *const libc::c_char,
                      getResearchName(pResearch));
                abort();
            }
        } else if strcmp(compName.as_mut_ptr(),
                         b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            //find the component stat
            psComp =
                getComponentDetails(compType.as_mut_ptr(),
                                    compName.as_mut_ptr());
            if !psComp.is_null() {
                (*pResearch).psStat = psComp as *mut BASE_STATS
            } else {
                debug(LOG_ERROR,
                      b"Cannot find the component Stat for Research %s\x00" as
                          *const u8 as *const libc::c_char,
                      getResearchName(pResearch));
                abort();
            }
        } else { (*pResearch).psStat = 0 as *mut BASE_STATS }
        if strcmp(imdName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*pResearch).pIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, imdName.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*pResearch).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the research PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getResearchName(pResearch));
                abort();
            }
        } else { (*pResearch).pIMD = 0 as *mut iIMDShape }
        if strcmp(imdName2.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*pResearch).pIMD2 =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, imdName2.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*pResearch).pIMD2.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the 2nd research PIE for record %s\x00" as
                          *const u8 as *const libc::c_char,
                      getResearchName(pResearch));
                abort();
            }
        } else { (*pResearch).pIMD2 = 0 as *mut iIMDShape }
        //get the message viewdata - if any
        if strcmp(msgName.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            //check its a major tech code
            if (*pResearch).techCode as libc::c_int != TC_MAJOR as libc::c_int
               {
                debug(LOG_ERROR,
                      b"This research should not have a message associated with it, %s the message will be ignored!\x00"
                          as *const u8 as *const libc::c_char,
                      getResearchName(pResearch));
                abort();
            } else {
                (*pResearch).pViewData = getViewData(msgName.as_mut_ptr())
            }
        }
        //redundancies - artefacts
        if (*pResearch).numRedArtefacts as libc::c_int > 0 as libc::c_int {
            /*pResearch->pRedArtefacts = (COMP_BASE_STATS **) MALLOC(pResearch->
				numRedArtefacts*sizeof(COMP_BASE_STATS *));
	    	if (pResearch->pRedArtefacts == NULL)
		    {
			   	DBERROR(("Out of memory assigning research artefacts - redundancies"));
			    return FALSE;
			}*/
            if numResearchArteRed as libc::c_int >=
                   40 as libc::c_int + 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research artefacts - redundancies\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pRedArtefacts =
                pResearchArteRed.offset(numResearchArteRed as libc::c_int as
                                            isize);
            //pResearchArteRed += pResearch->numRedArtefacts;
            //keep track on how many are being allocated
            numResearchArteRed =
                (numResearchArteRed as libc::c_int +
                     (*pResearch).numRedArtefacts as libc::c_int) as UBYTE
        }
        //results
        if (*pResearch).numArteResults as libc::c_int > 0 as libc::c_int {
            /*pResearch->pArtefactResults = (COMP_BASE_STATS **) MALLOC(pResearch->
				numArteResults*sizeof(COMP_BASE_STATS *));
			if (pResearch->pArtefactResults == NULL)
			{
				DBERROR(("Out of memory assigning research artefacts - results"));
				return FALSE;
			}*/
            if numResearchArteRed as libc::c_int >=
                   125 as libc::c_int + 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research artefacts - results\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pArtefactResults =
                pResearchArteRes.offset(numResearchArteRes as libc::c_int as
                                            isize);
            //pResearchArteRes += pResearch->numArteResults;
            //keep track on how many are being allocated
            numResearchArteRes =
                (numResearchArteRes as libc::c_int +
                     (*pResearch).numArteResults as libc::c_int) as UBYTE
        }
        //replacements
        if (*pResearch).numArteResults as libc::c_int > 0 as libc::c_int {
            /*pResearch->pReplacedArtefacts = (COMP_BASE_STATS **) MALLOC(pResearch->
				numArteResults*sizeof(COMP_BASE_STATS *));
			if (pResearch->pReplacedArtefacts == NULL)
			{
				DBERROR(("Out of memory assigning research artefacts - replacements"));
				return FALSE;
			}*/
            if numResearchArteRep as libc::c_int >=
                   125 as libc::c_int + 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research artefacts - replacements\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pReplacedArtefacts =
                pResearchArteRep.offset(numResearchArteRep as libc::c_int as
                                            isize);
            //pResearchArteRep += pResearch->numArteResults;
            //keep track on how many are being allocated
            numResearchArteRep =
                (numResearchArteRep as libc::c_int +
                     (*pResearch).numArteResults as libc::c_int) as UBYTE
        }
        //allocate storage for the functions
        if (*pResearch).numFunctions as libc::c_int > 0 as libc::c_int {
            /*pResearch->pFunctionList = (FUNCTION**)MALLOC(pResearch->
				numFunctions*sizeof(FUNCTION*));
			if (pResearch->pFunctionList == NULL)
			{
				DBERROR(("Out of memory assigning research functions"));
				return FALSE;
			}*/
            if numResearchFunc as libc::c_int >=
                   250 as libc::c_int + 25 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research functions\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pFunctionList =
                pResearchFunc.offset(numResearchFunc as libc::c_int as isize);
            //pResearchFunc += pResearch->numFunctions;
            //keep track on how many are being allocated
            numResearchFunc =
                (numResearchFunc as libc::c_int +
                     (*pResearch).numFunctions as libc::c_int) as UWORD
        }
        //allocate storage for the pre-requisities
        if (*pResearch).numPRRequired as libc::c_int > 0 as libc::c_int {
            /*pResearch->pPRList = (UDWORD*)MALLOC(pResearch->
				numPRRequired*sizeof(UDWORD));
			if (pResearch->pPRList == NULL)
			{
				DBERROR(("Out of memory assigning research pre-requisities"));
				return FALSE;
			}*/
            if numResearchPR as libc::c_int >= 650 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research pre-requisities\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pPRList =
                pResearchPR.offset(numResearchPR as libc::c_int as isize);
            //pResearchPR += pResearch->numPRRequired;
            //keep track on how many are being allocated
            numResearchPR =
                (numResearchPR as libc::c_int +
                     (*pResearch).numPRRequired as libc::c_int) as UWORD
        }
        //allocate storage for the structures
		//requirements
        if (*pResearch).numStructures as libc::c_int > 0 as libc::c_int {
            /*pResearch->pStructList = (UDWORD *) MALLOC(pResearch->
				numStructures*sizeof(UDWORD));
			if (pResearch->pStructList == NULL)
			{
				DBERROR(("Out of memory assigning research structures - requirements"));
				return FALSE;
			}*/
            if numResearchStructPR as libc::c_int >=
                   44 as libc::c_int + 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research structures - requirements\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pStructList =
                pResearchStructPR.offset(numResearchStructPR as libc::c_int as
                                             isize);
            //pResearchStructPR += pResearch->numStructures;
            //keep track on how many are being allocated
            numResearchStructPR =
                (numResearchStructPR as libc::c_int +
                     (*pResearch).numStructures as libc::c_int) as UBYTE as
                    UWORD
        }
        //redundancies
        if (*pResearch).numRedStructs as libc::c_int > 0 as libc::c_int {
            /*pResearch->pRedStructs = (UDWORD *) MALLOC(pResearch->
				numRedStructs*sizeof(UDWORD));
			if (pResearch->pRedStructs == NULL)
			{
				DBERROR(("Out of memory assigning research structures - redundancies"));
				return FALSE;
			}*/
            if numResearchStructRed as libc::c_int >=
                   30 as libc::c_int + 2 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research structures - redundancies\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pRedStructs =
                pResearchStructRed.offset(numResearchStructRed as libc::c_int
                                              as isize);
            //pResearchStructRed += pResearch->numRedStructs;
            //keep track on how many are being allocated
            numResearchStructRed =
                (numResearchStructRed as libc::c_int +
                     (*pResearch).numRedStructs as libc::c_int) as UBYTE as
                    UWORD
        }
        //results
        if (*pResearch).numStructResults as libc::c_int > 0 as libc::c_int {
            /*pResearch->pStructureResults = (UDWORD *) MALLOC(pResearch->
				numStructResults*sizeof(UDWORD));
			if (pResearch->pStructureResults == NULL)
			{
				DBERROR(("Out of memory assigning research structures - results"));
				return FALSE;
			}*/
            if numResearchStructRes as libc::c_int >=
                   84 as libc::c_int + 5 as libc::c_int {
                debug(LOG_ERROR,
                      b"Out of memory assigning research structures - results\x00"
                          as *const u8 as *const libc::c_char);
                abort();
            }
            //don't MALLOC - get them from the pre-defined arrays
            (*pResearch).pStructureResults =
                pResearchStructRes.offset(numResearchStructRes as libc::c_int
                                              as isize);
            //pResearchStructRes += pResearch->numStructResults;
            //keep track on how many are being allocated
            numResearchStructRes =
                (numResearchStructRes as libc::c_int +
                     (*pResearch).numStructResults as libc::c_int) as UBYTE as
                    UWORD
        }
        //set the researchPoints
        if resPoints > 0xffff as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"Research Points too high for research topic - %s \x00" as
                      *const u8 as *const libc::c_char,
                  getResearchName(pResearch));
            abort();
        }
        (*pResearch).researchPoints = resPoints as UWORD;
        //set the research power
        (*pResearch).researchPower =
            ((*pResearch).researchPoints as libc::c_int / 32 as libc::c_int)
                as UDWORD;
        if (*pResearch).researchPower > 450 as libc::c_int as libc::c_uint {
            (*pResearch).researchPower = 450 as libc::c_int as UDWORD
        }
        //increment the pointer to the start of the next record
        pResearchData =
            strchr(pResearchData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        //increment the list to the start of the next storage block
        pResearch = pResearch.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartResearchData);
    //Do this in initResearch now since there is a Max Research
	//now we know how many research topics there are we can create the
	//PLAYER_RESEARCH arrays
	/*for (i=0; i < MAX_PLAYERS; i++)
	{
		asPlayerResList[i] = (PLAYER_RESEARCH*)MALLOC(numResearch *
			sizeof(PLAYER_RESEARCH));
		if (asPlayerResList[i] == NULL)
		{
			DBERROR(("Out of memory assigning Player_Research"));
			return FALSE;
		}
		memset(asPlayerResList[i], 0, (numResearch * sizeof(PLAYER_RESEARCH)));
	}*/
    return 1 as libc::c_int;
}
//Load the pre-requisites for a research list
//Load the pre-requisites for a research list
#[no_mangle]
pub unsafe extern "C" fn loadResearchPR(mut pPRData: *mut libc::c_char,
                                        mut bufferSize: UDWORD) -> BOOL {
    let mut pStartPRData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut ResearchName: [STRING; 60] = [0; 60];
    let mut PRName: [STRING; 60] = [0; 60];
    //UBYTE				incR, incPR;
    let mut incR: UWORD = 0;
    let mut incPR: UWORD = 0;
    let mut pResearch: *mut RESEARCH = asResearch;
    let mut pPRResearch: *mut RESEARCH = asResearch;
    let mut recFound: BOOL = 0;
    pStartPRData = pPRData;
    NumToAlloc = numCR(pPRData, bufferSize);
    //check not going to go over max
    if NumToAlloc <= 650 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"loadResearchPR: too many!\x00" as *const u8 as
                  *const libc::c_char);
    };
    if NumToAlloc <= 650 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              811 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"loadResearchPR\x00")).as_ptr(),
              b"NumToAlloc <= MAX_RESEARCH_PR\x00" as *const u8 as
                  *const libc::c_char);
    };
    numResearchPR = 0 as libc::c_int as UWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        recFound = 0 as libc::c_int;
        //read the data into the storage - the data is delimited using commas
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        PRName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pPRData,
               b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as
                   *const libc::c_char, ResearchName.as_mut_ptr(),
               PRName.as_mut_ptr());
        if getResourceName(ResearchName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if getResourceName(PRName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        //loop through each Research to compare the name
        incR = 0 as libc::c_int as UWORD;
        while (incR as libc::c_uint) < numResearch {
            if strcmp(ResearchName.as_mut_ptr(),
                      (*pResearch.offset(incR as isize)).pName) == 0 {
                //Research found
                incPR = 0 as libc::c_int as UWORD;
                while (incPR as libc::c_uint) < numResearch {
                    if strcmp(PRName.as_mut_ptr(),
                              (*pPRResearch.offset(incPR as isize)).pName) ==
                           0 {
                        //check not allocating more than allowed
                        if (*pResearch.offset(incR as isize)).storeCount as
                               libc::c_int + 1 as libc::c_int >
                               (*pResearch.offset(incR as
                                                      isize)).numPRRequired as
                                   SDWORD {
                            debug(LOG_ERROR,
                                  b"Trying to allocate more pre-requisites than allowed for research %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  getResearchName(pResearch));
                            abort();
                        }
                        //PRresearch found alloc this to the current Research
                        *(*pResearch.offset(incR as
                                                isize)).pPRList.offset((*pResearch.offset(incR
                                                                                              as
                                                                                              isize)).storeCount
                                                                           as
                                                                           isize)
                            = incPR;
                        //keep tab on how many we have loaded in
                        numResearchPR = numResearchPR.wrapping_add(1);
                        let ref mut fresh0 =
                            (*pResearch.offset(incR as isize)).storeCount;
                        *fresh0 = (*fresh0).wrapping_add(1);
                        recFound = 1 as libc::c_int;
                        break ;
                    } else { incPR = incPR.wrapping_add(1) }
                }
                //if pre-requisite not found - error
                if !(recFound == 0) { break ; }
                debug(LOG_ERROR,
                      b"Unable to find Pre-requisite %s for research %s\x00"
                          as *const u8 as *const libc::c_char,
                      PRName.as_mut_ptr(), ResearchName.as_mut_ptr());
                abort();
            } else { incR = incR.wrapping_add(1) }
        }
        //if Research not found - error
        if recFound == 0 {
            debug(LOG_ERROR,
                  b"Unable to find Research %s\x00" as *const u8 as
                      *const libc::c_char, ResearchName.as_mut_ptr());
            abort();
        }
        //quick check that haven't reached maxPR
        if numResearchPR as libc::c_int >= 650 as libc::c_int { break ; }
        //increment the pointer to the start of the next record
        pPRData =
            strchr(pPRData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartPRData);
    return 1 as libc::c_int;
}
//Load the artefacts for a research list
//Load the artefacts for a research list
#[no_mangle]
pub unsafe extern "C" fn loadResearchArtefacts(mut pArteData:
                                                   *mut libc::c_char,
                                               mut bufferSize: UDWORD,
                                               mut listNumber: UDWORD)
 -> BOOL {
    let mut pStartArteData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut ResearchName: [STRING; 60] = [0; 60];
    let mut ArteName: [STRING; 60] = [0; 60];
    let mut TypeName: [STRING; 60] = [0; 60];
    let mut incR: UDWORD = 0;
    let mut pResearch: *mut RESEARCH = asResearch;
    let mut pArtefact: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    let mut newType: UDWORD = 0;
    let mut maxArtefacts: UBYTE = 0;
    //initialise the storage flags
    incR = 0 as libc::c_int as UDWORD;
    while incR < numResearch {
        (*pResearch.offset(incR as isize)).storeCount =
            0 as libc::c_int as UBYTE;
        incR = incR.wrapping_add(1)
    }
    pResearch = asResearch;
    pStartArteData = pArteData;
    NumToAlloc = numCR(pArteData, bufferSize);
    //check not going to go over max
    match listNumber {
        1 => {
            if NumToAlloc <=
                   (40 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"loadResearchArtefacts: too many Redundant Components\x00"
                          as *const u8 as *const libc::c_char);
            };
            if NumToAlloc <=
                   (40 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      934 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"loadResearchArtefacts\x00")).as_ptr(),
                      b"NumToAlloc <= MAX_RESEARCH_ARTE_RED\x00" as *const u8
                          as *const libc::c_char);
            };
            numResearchArteRed = 0 as libc::c_int as UBYTE
        }
        2 => {
            if NumToAlloc <=
                   (125 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"loadResearchArtefacts: too many Component Results\x00"
                          as *const u8 as *const libc::c_char);
            };
            if NumToAlloc <=
                   (125 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      939 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"loadResearchArtefacts\x00")).as_ptr(),
                      b"NumToAlloc <= MAX_RESEARCH_ARTE_RES\x00" as *const u8
                          as *const libc::c_char);
            };
            numResearchArteRes = 0 as libc::c_int as UBYTE;
            numResearchArteRep = 0 as libc::c_int as UBYTE
        }
        _ => { }
    }
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        //read the data into the storage - the data is delimited using commas
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        ArteName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        TypeName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pArteData,
               b"%[^\',\'],%[^\',\'],%[^\',\']\x00" as *const u8 as
                   *const libc::c_char, ResearchName.as_mut_ptr(),
               ArteName.as_mut_ptr(), TypeName.as_mut_ptr());
        //increment the data pointer
        pArteData =
            pArteData.offset(strlen(ResearchName.as_mut_ptr()).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint).wrapping_add(strlen(ArteName.as_mut_ptr())).wrapping_add(1
                                                                                                                                                           as
                                                                                                                                                           libc::c_int
                                                                                                                                                           as
                                                                                                                                                           libc::c_uint).wrapping_add(strlen(TypeName.as_mut_ptr())).wrapping_add(1
                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                      libc::c_uint)
                                 as isize);
        if getResourceName(ResearchName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if getResourceName(ArteName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        pArtefact =
            getComponentDetails(TypeName.as_mut_ptr(), ArteName.as_mut_ptr());
        if pArtefact.is_null() { return 0 as libc::c_int }
        //get the type for comparison later
        newType = statType((*pArtefact).ref_0);
        pResearch = getResearch(ResearchName.as_mut_ptr(), 0 as libc::c_int);
        if pResearch.is_null() { return 0 as libc::c_int }
        //ArtefactResearch found - alloc the artefact to the current Research topic
        match listNumber {
            1 => {
                let ref mut fresh1 =
                    *(*pResearch).pRedArtefacts.offset((*pResearch).storeCount
                                                           as libc::c_int as
                                                           isize);
                *fresh1 = pArtefact;
                //keep tab on how many we have loaded in
                numResearchArteRed = numResearchArteRed.wrapping_add(1);
                maxArtefacts = (*pResearch).numRedArtefacts
            }
            2 => {
                let ref mut fresh2 =
                    *(*pResearch).pArtefactResults.offset((*pResearch).storeCount
                                                              as libc::c_int
                                                              as isize);
                *fresh2 = pArtefact;
                //keep tab on how many we have loaded in
                numResearchArteRes = numResearchArteRes.wrapping_add(1);
                maxArtefacts = (*pResearch).numArteResults
            }
            _ => {
                debug(LOG_ERROR,
                      b"Unknown research list\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
        }
        //deal with extra data
        match listNumber {
            1 => {
                //ignore the last character
                sscanf(pArteData,
                       b",%*d\x00" as *const u8 as *const libc::c_char);
            }
            2 => {
                ArteName[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as STRING;
                TypeName[0 as libc::c_int as usize] =
                    '\u{0}' as i32 as STRING;
                sscanf(pArteData,
                       b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as
                           *const libc::c_char, ArteName.as_mut_ptr(),
                       TypeName.as_mut_ptr());
                if strcmp(ArteName.as_mut_ptr(),
                          b"0\x00" as *const u8 as *const libc::c_char) == 0 {
                    let ref mut fresh3 =
                        *(*pResearch).pReplacedArtefacts.offset((*pResearch).storeCount
                                                                    as
                                                                    libc::c_int
                                                                    as isize);
                    *fresh3 = 0 as *mut COMP_BASE_STATS
                } else {
                    if getResourceName(ArteName.as_mut_ptr()) == 0 {
                        return 0 as libc::c_int
                    }
                    pArtefact =
                        getComponentDetails(TypeName.as_mut_ptr(),
                                            ArteName.as_mut_ptr());
                    if pArtefact.is_null() { return 0 as libc::c_int }
                    //check the old and new types are the same
                    if statType((*pArtefact).ref_0) != newType {
                        debug(LOG_ERROR,
                              b"You are trying to replace one type of component with a different type for research %s in ResultComponents.txt\x00"
                                  as *const u8 as *const libc::c_char,
                              ResearchName.as_mut_ptr());
                        abort();
                    }
                    //ArtefactResearch found - alloc the artefact to the current Research topic
                    let ref mut fresh4 =
                        *(*pResearch).pReplacedArtefacts.offset((*pResearch).storeCount
                                                                    as
                                                                    libc::c_int
                                                                    as isize);
                    *fresh4 = pArtefact;
                    numResearchArteRep = numResearchArteRep.wrapping_add(1)
                }
            }
            _ => {
                debug(LOG_ERROR,
                      b"Unknown research list\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
        }
        //check not allocating more than allowed
        if (*pResearch).storeCount as libc::c_int >
               maxArtefacts as libc::c_int {
            debug(LOG_ERROR,
                  b"Trying to allocate more artefacts than allowed for research %s\x00"
                      as *const u8 as *const libc::c_char,
                  getResearchName(pResearch));
            abort();
        }
        (*pResearch).storeCount = (*pResearch).storeCount.wrapping_add(1);
        //quick check that haven't reached maxArtes
        if numResearchArteRed as libc::c_int >=
               40 as libc::c_int + 5 as libc::c_int ||
               numResearchArteRes as libc::c_int >=
                   125 as libc::c_int + 5 as libc::c_int ||
               numResearchArteRep as libc::c_int >
                   125 as libc::c_int + 5 as libc::c_int {
            break ;
        }
        //increment the pointer to the start of the next record
        pArteData =
            strchr(pArteData, '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartArteData);
    return 1 as libc::c_int;
}
//Load the Structures for a research list
//Load the Structures for a research list
#[no_mangle]
pub unsafe extern "C" fn loadResearchStructures(mut pStructData:
                                                    *mut libc::c_char,
                                                mut bufferSize: UDWORD,
                                                mut listNumber: UDWORD)
 -> BOOL {
    let mut pStartStructData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut ResearchName: [STRING; 60] = [0; 60];
    let mut StructureName: [STRING; 60] = [0; 60];
    //UBYTE				incR;
    let mut incR: UWORD = 0;
    let mut incS: UWORD = 0;
    let mut pResearch: *mut RESEARCH = asResearch;
    let mut pStructure: *mut STRUCTURE_STATS = asStructureStats;
    let mut recFound: BOOL = 0;
    let mut numToFind: UDWORD = 0;
    //initialise the storage flags
    incR = 0 as libc::c_int as UWORD;
    while (incR as libc::c_uint) < numResearch {
        (*pResearch.offset(incR as isize)).storeCount =
            0 as libc::c_int as UBYTE;
        incR = incR.wrapping_add(1)
    }
    pResearch = asResearch;
    pStartStructData = pStructData;
    NumToAlloc = numCR(pStructData, bufferSize);
    match listNumber {
        0 => {
            //check not going to go over max
            if NumToAlloc <=
                   (44 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"loadResearchStructures: too many Struct PRs\x00" as
                          *const u8 as *const libc::c_char);
            };
            if NumToAlloc <=
                   (44 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      1099 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"loadResearchStructures\x00")).as_ptr(),
                      b"NumToAlloc <= MAX_RESEARCH_STRUCT_PR\x00" as *const u8
                          as *const libc::c_char);
            };
            numResearchStructPR = 0 as libc::c_int as UWORD
        }
        1 => {
            //check not going to go over max
            if NumToAlloc <=
                   (30 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"loadResearchStructures: too many redundant structure\x00"
                          as *const u8 as *const libc::c_char);
            };
            if NumToAlloc <=
                   (30 as libc::c_int + 2 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      1104 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"loadResearchStructures\x00")).as_ptr(),
                      b"NumToAlloc <= MAX_RESEARCH_STRUCT_RED\x00" as
                          *const u8 as *const libc::c_char);
            };
            numResearchStructRed = 0 as libc::c_int as UWORD
        }
        2 => {
            //check not going to go over max
            if NumToAlloc <=
                   (84 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"loadResearchStructures: too many structure results\x00"
                          as *const u8 as *const libc::c_char);
            };
            if NumToAlloc <=
                   (84 as libc::c_int + 5 as libc::c_int) as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      1109 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"loadResearchStructures\x00")).as_ptr(),
                      b"NumToAlloc <= MAX_RESEARCH_STRUCT_RES\x00" as
                          *const u8 as *const libc::c_char);
            };
            numResearchStructRes = 0 as libc::c_int as UWORD
        }
        _ => { }
    }
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        recFound = 0 as libc::c_int;
        numToFind = 0 as libc::c_int as UDWORD;
        //read the data into the storage - the data is delimited using comma's
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        StructureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pStructData,
               b"%[^\',\'],%[^\',\'],%*d,%*d\x00" as *const u8 as
                   *const libc::c_char, ResearchName.as_mut_ptr(),
               StructureName.as_mut_ptr());
        if getResourceName(ResearchName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if getResourceName(StructureName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        //loop through each Research to compare the name
        incR = 0 as libc::c_int as UWORD;
        while (incR as libc::c_uint) < numResearch {
            if strcmp(ResearchName.as_mut_ptr(),
                      (*pResearch.offset(incR as isize)).pName) == 0 {
                //Research found
                incS = 0 as libc::c_int as UWORD;
                while (incS as libc::c_uint) < numStructureStats {
                    if strcmp(StructureName.as_mut_ptr(),
                              (*pStructure.offset(incS as isize)).pName) == 0
                       {
                        //Structure found - alloc this to the current Research
                        match listNumber {
                            0 => {
                                *(*pResearch.offset(incR as
                                                        isize)).pStructList.offset((*pResearch.offset(incR
                                                                                                          as
                                                                                                          isize)).storeCount
                                                                                       as
                                                                                       isize)
                                    = incS;
                                //keep tab on how many we have loaded in
                                numResearchStructPR =
                                    numResearchStructPR.wrapping_add(1);
                                numToFind =
                                    (*pResearch.offset(incR as
                                                           isize)).numStructures
                                        as UDWORD
                            }
                            1 => {
                                *(*pResearch.offset(incR as
                                                        isize)).pRedStructs.offset((*pResearch.offset(incR
                                                                                                          as
                                                                                                          isize)).storeCount
                                                                                       as
                                                                                       isize)
                                    = incS;
                                //keep tab on how many we have loaded in
                                numResearchStructRed =
                                    numResearchStructRed.wrapping_add(1);
                                numToFind =
                                    (*pResearch.offset(incR as
                                                           isize)).numRedStructs
                                        as UDWORD
                            }
                            2 => {
                                *(*pResearch.offset(incR as
                                                        isize)).pStructureResults.offset((*pResearch.offset(incR
                                                                                                                as
                                                                                                                isize)).storeCount
                                                                                             as
                                                                                             isize)
                                    = incS;
                                //keep tab on how many we have loaded in
                                numResearchStructRes =
                                    numResearchStructRes.wrapping_add(1);
                                numToFind =
                                    (*pResearch.offset(incR as
                                                           isize)).numStructResults
                                        as UDWORD
                            }
                            _ => {
                                /* NO DEFAULT CASE? Alex.... Here ya go - just for you...*/
                                debug(LOG_ERROR,
                                      b"Unknown research list\x00" as
                                          *const u8 as *const libc::c_char);
                                abort();
                            }
                        }
                        recFound = 1 as libc::c_int;
                        //check not allocating more than allowed
                        if (*pResearch.offset(incR as isize)).storeCount as
                               libc::c_int > numToFind as SDWORD {
                            debug(LOG_ERROR,
                                  b"Trying to allocate more Structures than allowed for research %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  getResearchName(pResearch));
                            abort();
                        }
                        let ref mut fresh5 =
                            (*pResearch.offset(incR as isize)).storeCount;
                        *fresh5 = (*fresh5).wrapping_add(1);
                        break ;
                    } else { incS = incS.wrapping_add(1) }
                }
                //if Structure not found - error
                if !(recFound == 0) { break ; }
                debug(LOG_ERROR,
                      b"Unable to find Structure %s for research %s\x00" as
                          *const u8 as *const libc::c_char,
                      StructureName.as_mut_ptr(), ResearchName.as_mut_ptr());
                abort();
            } else { incR = incR.wrapping_add(1) }
        }
        //if Research not found - error
        if recFound == 0 {
            debug(LOG_ERROR,
                  b"Unable to allocate all Research Structures for %s\x00" as
                      *const u8 as *const libc::c_char,
                  ResearchName.as_mut_ptr());
            abort();
        }
        //quick check that haven't reached max structs
        if numResearchStructPR as libc::c_int >=
               44 as libc::c_int + 5 as libc::c_int ||
               numResearchStructRes as libc::c_int >=
                   84 as libc::c_int + 5 as libc::c_int ||
               numResearchStructRed as libc::c_int >=
                   30 as libc::c_int + 2 as libc::c_int {
            break ;
        }
        //increment the pointer to the start of the next record
        pStructData =
            strchr(pStructData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartStructData);
    return 1 as libc::c_int;
}
//Load the pre-requisites for a research list
//Load the pre-requisites for a research list
#[no_mangle]
pub unsafe extern "C" fn loadResearchFunctions(mut pFunctionData:
                                                   *mut libc::c_char,
                                               mut bufferSize: UDWORD)
 -> BOOL {
    let mut pStartFunctionData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut ResearchName: [STRING; 60] = [0; 60];
    let mut FunctionName: [STRING; 60] = [0; 60];
    let mut incR: UDWORD = 0;
    let mut incF: UDWORD = 0;
    let mut pResearch: *mut RESEARCH = asResearch;
    let mut pFunction: *mut *mut FUNCTION = asFunctions;
    let mut recFound: BOOL = 0;
    //initialise the storage flags
    incR = 0 as libc::c_int as UDWORD;
    while incR < numResearch {
        (*pResearch.offset(incR as isize)).storeCount =
            0 as libc::c_int as UBYTE;
        incR = incR.wrapping_add(1)
    }
    pResearch = asResearch;
    pStartFunctionData = pFunctionData;
    NumToAlloc = numCR(pFunctionData, bufferSize);
    //check not going to go over max
    if NumToAlloc <= (250 as libc::c_int + 25 as libc::c_int) as libc::c_uint
       {
    } else {
        debug(LOG_ERROR,
              b"loadResearchFunctions: too many\x00" as *const u8 as
                  *const libc::c_char);
    };
    if NumToAlloc <= (250 as libc::c_int + 25 as libc::c_int) as libc::c_uint
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              1261 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"loadResearchFunctions\x00")).as_ptr(),
              b"NumToAlloc <= MAX_RESEARCH_FUNC\x00" as *const u8 as
                  *const libc::c_char);
    };
    numResearchFunc = 0 as libc::c_int as UWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        recFound = 0 as libc::c_int;
        //read the data into the storage - the data is delimited using comma's
        ResearchName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        FunctionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pFunctionData,
               b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as
                   *const libc::c_char, ResearchName.as_mut_ptr(),
               FunctionName.as_mut_ptr());
        if getResourceName(ResearchName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        //loop through each Research to compare the name
        incR = 0 as libc::c_int as UDWORD;
        while incR < numResearch {
            if strcmp(ResearchName.as_mut_ptr(),
                      (*pResearch.offset(incR as isize)).pName) == 0 {
                //Research found
                incF = 0 as libc::c_int as UDWORD;
                while incF < numFunctions {
                    if strcmp(FunctionName.as_mut_ptr(),
                              (**pFunction.offset(incF as isize)).pName) == 0
                       {
                        //Function found alloc this to the current Research
                        let ref mut fresh6 =
                            *(*pResearch.offset(incR as
                                                    isize)).pFunctionList.offset((*pResearch.offset(incR
                                                                                                        as
                                                                                                        isize)).storeCount
                                                                                     as
                                                                                     isize);
                        *fresh6 = *pFunction.offset(incF as isize);
                        //keep tab on how many we have loaded in
                        numResearchFunc = numResearchFunc.wrapping_add(1);
                        recFound = 1 as libc::c_int;
                        //check not allocating more than allowed
                        if (*pResearch.offset(incR as isize)).storeCount as
                               libc::c_int >
                               (*pResearch.offset(incR as isize)).numFunctions
                                   as SDWORD {
                            debug(LOG_ERROR,
                                  b"Trying to allocate more Functions than allowed for research %s\x00"
                                      as *const u8 as *const libc::c_char,
                                  ResearchName.as_mut_ptr());
                            abort();
                        }
                        let ref mut fresh7 =
                            (*pResearch.offset(incR as isize)).storeCount;
                        *fresh7 = (*fresh7).wrapping_add(1);
                        break ;
                    } else { incF = incF.wrapping_add(1) }
                }
                //if Function not found - error
                if !(recFound == 0) { break ; }
                debug(LOG_ERROR,
                      b"Unable to find Function %s for research %s\x00" as
                          *const u8 as *const libc::c_char,
                      FunctionName.as_mut_ptr(), ResearchName.as_mut_ptr());
                abort();
            } else { incR = incR.wrapping_add(1) }
        }
        //if Research not found - error
        if recFound == 0 {
            debug(LOG_ERROR,
                  b"Unable to allocate all research Functions for %s\x00" as
                      *const u8 as *const libc::c_char,
                  ResearchName.as_mut_ptr());
            abort();
        }
        //quick check that haven't reached maxPR
        if numResearchFunc as libc::c_int >=
               250 as libc::c_int + 25 as libc::c_int {
            break ;
        }
        //increment the pointer to the start of the next record
        pFunctionData =
            strchr(pFunctionData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartFunctionData);
    return 1 as libc::c_int;
}
/*function to check what can be researched for a particular player at any one 
  instant. Returns the number to research*/
//extern UBYTE fillResearchList(UBYTE *plist, UDWORD playerID, UWORD topic, 
//							   UWORD limit);
//needs to be UWORD sized for Patches
/*
Function to check what can be researched for a particular player at any one
instant.

A topic can be researched if the playerRes 'possible' flag has been set (by script)
or if the research pre-req topics have been researched. A check is made for any
structures that are required to have been built for topics that do not have
the 'possible' flag set.

 **NB** A topic with zero PR's can ONLY be researched once the 'possible' flag
 has been set.

There can only be 'limit' number of entries
'topic' is the currently researched topic
*/
//needs to be UWORD sized for Patches
//UBYTE fillResearchList(UBYTE *plist, UDWORD playerID, UWORD topic, UWORD limit)
// NOTE by AJL may 99 - skirmish now has it's own version of this, skTopicAvail.
#[no_mangle]
pub unsafe extern "C" fn fillResearchList(mut plist: *mut UWORD,
                                          mut playerID: UDWORD,
                                          mut topic: UWORD, mut limit: UWORD)
 -> UWORD {
    let mut current_block: u64;
    //UBYTE				inc, count=0;
    let mut inc: UWORD = 0;
    let mut count: UWORD = 0 as libc::c_int as UWORD;
    let mut incPR: UDWORD = 0;
    let mut incS: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[playerID as usize];
    let mut bPRFound: BOOL = 0;
    let mut bStructFound: BOOL = 0;
    //needs to be UWORD sized for Patches
    if numResearch < 0xffff as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"fillResearchList: only using a UWORD for storage - need more!\x00"
                  as *const u8 as *const libc::c_char);
    };
    if numResearch < 0xffff as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              1383 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"fillResearchList\x00")).as_ptr(),
              b"numResearch < UWORD_MAX\x00" as *const u8 as
                  *const libc::c_char);
    };
    //ASSERT( numResearch < UBYTE_MAX,
	//	"fillResearchList: only using a UBYTE for storage - need more!" );
    inc = 0 as libc::c_int as UWORD;
    while (inc as libc::c_uint) < numResearch {
        //if the inc matches the 'topic' - automatically add to the list
        if inc as libc::c_int == topic as libc::c_int {
            current_block = 2731969147856261738;
        } else if (*pPlayerRes.offset(inc as isize)).ResearchStatus as
                      libc::c_int & 0x2 as libc::c_int != 0 {
            current_block = 2731969147856261738;
        } else {
            //if its a cancelled topic - add to list
            //if the topic is possible and has not already been researched - add to list
            if (*pPlayerRes.offset(inc as isize)).ResearchStatus as
                   libc::c_int & 0x80 as libc::c_int != 0 {
                if (*pPlayerRes.offset(inc as isize)).ResearchStatus as
                       libc::c_int & 0x4 as libc::c_int == 0 as libc::c_int &&
                       (*pPlayerRes.offset(inc as isize)).ResearchStatus as
                           libc::c_int & 0x1 as libc::c_int ==
                           0 as libc::c_int {
                    current_block = 2731969147856261738;
                } else { current_block = 12349973810996921269; }
            } else { current_block = 12349973810996921269; }
            match current_block {
                2731969147856261738 => { }
                _ =>
                //if single player mode and key topic, then ignore cos can't do it!
                {
                    if bMultiPlayer == 0 {
                        if (*asResearch.offset(inc as isize)).keyTopic != 0 {
                            current_block = 6483416627284290920;
                        } else { current_block = 8457315219000651999; }
                    } else { current_block = 8457315219000651999; }
                    match current_block {
                        6483416627284290920 => { }
                        _ =>
                        // make sure that the research is not completed  or started by another researchfac
                        {
                            if (*pPlayerRes.offset(inc as
                                                       isize)).ResearchStatus
                                   as libc::c_int & 0x4 as libc::c_int ==
                                   0 as libc::c_int &&
                                   (*pPlayerRes.offset(inc as
                                                           isize)).ResearchStatus
                                       as libc::c_int & 0x1 as libc::c_int ==
                                       0 as libc::c_int {
                                // Research is not completed  ... also  it has not been started by another researchfac
                                //if there aren't any PR's - go to next topic
                                if (*asResearch.offset(inc as
                                                           isize)).numPRRequired
                                       == 0 {
                                    current_block = 6483416627284290920;
                                } else {
                                    //check for pre-requisites
                                    bPRFound = 1 as libc::c_int;
                                    incPR = 0 as libc::c_int as UDWORD;
                                    while incPR <
                                              (*asResearch.offset(inc as
                                                                      isize)).numPRRequired
                                                  as libc::c_uint {
                                        if (*pPlayerRes.offset(*(*asResearch.offset(inc
                                                                                        as
                                                                                        isize)).pPRList.offset(incPR
                                                                                                                   as
                                                                                                                   isize)
                                                                   as
                                                                   isize)).ResearchStatus
                                               as libc::c_int &
                                               0x4 as libc::c_int ==
                                               0 as libc::c_int {
                                            //if haven't pre-requisite - quit checking rest
                                            bPRFound = 0 as libc::c_int;
                                            break ;
                                        } else {
                                            incPR = incPR.wrapping_add(1)
                                        }
                                    }
                                    if bPRFound == 0 {
                                        //if haven't pre-requisites, skip the rest of the checks
                                        current_block = 6483416627284290920;
                                    } else {
                                        //check for structure effects
                                        bStructFound = 1 as libc::c_int;
                                        incS = 0 as libc::c_int as UDWORD;
                                        while incS <
                                                  (*asResearch.offset(inc as
                                                                          isize)).numStructures
                                                      as libc::c_uint {
                                            //if (!checkStructureStatus(asStructureStats + asResearch[inc].
				//	pStructList[incS], playerID, SS_BUILT))
                                            if checkSpecificStructExists(*(*asResearch.offset(inc
                                                                                                  as
                                                                                                  isize)).pStructList.offset(incS
                                                                                                                                 as
                                                                                                                                 isize)
                                                                             as
                                                                             UDWORD,
                                                                         playerID)
                                                   == 0 {
                                                //if not built, quit checking
                                                bStructFound =
                                                    0 as libc::c_int;
                                                break ;
                                            } else {
                                                incS = incS.wrapping_add(1)
                                            }
                                        }
                                        if bStructFound == 0 {
                                            current_block =
                                                6483416627284290920;
                                        } else {
                                            current_block =
                                                2731969147856261738;
                                        }
                                    }
                                }
                            } else { current_block = 6483416627284290920; }
                        }
                    }
                }
            }
        }
        match current_block {
            2731969147856261738 => {
                //if passed all the tests - add it to the list
                let fresh8 = plist;
                plist = plist.offset(1);
                *fresh8 = inc;
                count = count.wrapping_add(1);
                if count as libc::c_int == limit as libc::c_int {
                    return count
                }
            }
            _ => { }
        }
        //if haven't all structs built, skip to next topic
        inc = inc.wrapping_add(1)
    }
    return count;
}
/* process the results of a completed research topic */
/* process the results of a completed research topic */
#[no_mangle]
pub unsafe extern "C" fn researchResult(mut researchIndex: UDWORD,
                                        mut player: UBYTE,
                                        mut bDisplay: BOOL) {
    let mut pResearch: *mut RESEARCH =
        asResearch.offset(researchIndex as isize); //, upgrade;
    let mut type_0: UDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut pFunction: *mut FUNCTION = 0 as *mut FUNCTION;
    let mut compInc: UDWORD = 0;
    let mut pMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[player as usize];
    if researchIndex < numResearch {
    } else {
        debug(LOG_ERROR,
              b"researchResult: invalid research index\x00" as *const u8 as
                  *const libc::c_char);
    };
    if researchIndex < numResearch {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              1491 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"researchResult\x00")).as_ptr(),
              b"researchIndex < numResearch\x00" as *const u8 as
                  *const libc::c_char);
    };
    sendReseachStatus(0 as *mut STRUCTURE, researchIndex, player,
                      0 as libc::c_int);
    (*pPlayerRes.offset(researchIndex as isize)).ResearchStatus =
        ((*pPlayerRes.offset(researchIndex as isize)).ResearchStatus as
             libc::c_int &
             !(0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) |
             0x4 as libc::c_int) as UBYTE;
    //check for structures to be made available
    inc = 0 as libc::c_int as UDWORD;
    while inc < (*pResearch).numStructResults as libc::c_uint {
        *apStructTypeLists[player as
                               usize].offset(*(*pResearch).pStructureResults.offset(inc
                                                                                        as
                                                                                        isize)
                                                 as isize) =
            0x1 as libc::c_int as UBYTE;
        inc = inc.wrapping_add(1)
    }
    //check for structures to be made redundant
    inc = 0 as libc::c_int as UDWORD;
    while inc < (*pResearch).numRedStructs as libc::c_uint {
        *apStructTypeLists[player as
                               usize].offset(*(*pResearch).pRedStructs.offset(inc
                                                                                  as
                                                                                  isize)
                                                 as isize) =
            0x2 as libc::c_int as UBYTE;
        inc = inc.wrapping_add(1)
    }
    //check for artefacts to be made available
    inc = 0 as libc::c_int as UDWORD;
    while inc < (*pResearch).numArteResults as libc::c_uint {
        //determine the type of artefact
        type_0 =
            statType((**(*pResearch).pArtefactResults.offset(inc as
                                                                 isize)).ref_0);
        //set the component state to AVAILABLE
        compInc =
            (**(*pResearch).pArtefactResults.offset(inc as
                                                        isize)).ref_0.wrapping_sub(statRefStart(type_0));
        *apCompLists[player as
                         usize][type_0 as usize].offset(compInc as isize) =
            0x1 as libc::c_int as UBYTE;
        //check for default sensor
        if type_0 == COMP_SENSOR as libc::c_int as libc::c_uint {
            if (*asSensorStats.offset(compInc as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                aDefaultSensor[player as usize] = compInc
            }
        }
        //check for default ECM
        if type_0 == COMP_ECM as libc::c_int as libc::c_uint {
            if (*asECMStats.offset(compInc as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                aDefaultECM[player as usize] = compInc
            }
        }
        //check for default Repair
        if type_0 == COMP_REPAIRUNIT as libc::c_int as libc::c_uint {
            if (*asRepairStats.offset(compInc as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                aDefaultRepair[player as usize] = compInc
            }
        }
        //check if this component replaces an 'older' component
        if !(*(*pResearch).pReplacedArtefacts.offset(inc as isize)).is_null()
           {
            replaceComponent(*(*pResearch).pArtefactResults.offset(inc as
                                                                       isize),
                             *(*pResearch).pReplacedArtefacts.offset(inc as
                                                                         isize),
                             player);
            //set the 'old' component to unavailable
            type_0 =
                statType((**(*pResearch).pReplacedArtefacts.offset(inc as
                                                                       isize)).ref_0);
            //set the component state to AVAILABLE
            compInc =
                (**(*pResearch).pReplacedArtefacts.offset(inc as
                                                              isize)).ref_0.wrapping_sub(statRefStart(type_0));
            *apCompLists[player as
                             usize][type_0 as usize].offset(compInc as isize)
                = 0x2 as libc::c_int as UBYTE
        }
        //check if the component is a brain
        if type_0 == COMP_BRAIN as libc::c_int as libc::c_uint {
            cmdDroidAvailable(asBrainStats.offset(compInc as isize),
                              player as SDWORD);
        }
        //check if self repair has come on line
        if type_0 == COMP_REPAIRUNIT as libc::c_int as libc::c_uint {
            if (*asRepairStats.offset(compInc as isize)).location ==
                   LOC_DEFAULT as libc::c_int as libc::c_uint {
                enableSelfRepair(player);
            }
        }
        inc = inc.wrapping_add(1)
    }
    //check for artefacts to be made redundant
    inc = 0 as libc::c_int as UDWORD;
    while inc < (*pResearch).numRedArtefacts as libc::c_uint {
        //determine the type of artefact
        type_0 =
            statType((**(*pResearch).pRedArtefacts.offset(inc as
                                                              isize)).ref_0);
        //set the component state to UNAVAILABLE
        *apCompLists[player as
                         usize][type_0 as
                                    usize].offset((**(*pResearch).pRedArtefacts.offset(inc
                                                                                           as
                                                                                           isize)).ref_0.wrapping_sub(statRefStart(type_0))
                                                      as isize) =
            0x2 as libc::c_int as UBYTE;
        inc = inc.wrapping_add(1)
    }
    //check for technology effects
    inc = 0 as libc::c_int as UDWORD; //end of function loop
    while inc < (*pResearch).numFunctions as libc::c_uint {
        pFunction = *(*pResearch).pFunctionList.offset(inc as isize);
        match (*pFunction).type_0 as libc::c_int {
            1 => {
                productionUpgrade(pFunction, player);
                //end of switch
                //search the list of players structures for a Factory
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    /*if (psCurr->pStructureType->type == REF_FACTORY OR
						psCurr->pStructureType->type == REF_CYBORG_FACTORY OR
						psCurr->pStructureType->type == REF_VTOL_FACTORY)
					{
						//upgrade the Output
						productionUpgrade(pFunction, psCurr);
					}*/
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_FACTORY as libc::c_int as libc::c_uint &&
                           (*(pFunction as
                                  *mut PRODUCTION_UPGRADE_FUNCTION)).factory
                               as libc::c_int != 0 ||
                           (*(*psCurr).pStructureType).type_0 ==
                               REF_CYBORG_FACTORY as libc::c_int as
                                   libc::c_uint &&
                               (*(pFunction as
                                      *mut PRODUCTION_UPGRADE_FUNCTION)).cyborgFactory
                                   as libc::c_int != 0 ||
                           (*(*psCurr).pStructureType).type_0 ==
                               REF_VTOL_FACTORY as libc::c_int as libc::c_uint
                               &&
                               (*(pFunction as
                                      *mut PRODUCTION_UPGRADE_FUNCTION)).vtolFactory
                                   as libc::c_int != 0 {
                        //upgrade the Output for the structure
                        structureProductionUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                    //set the function upgrade flag for future factories being built
					/*for (upgrade = 0; upgrade < numProductionUpgrades; upgrade++)
					{
						if (apProductionUpgrades[player][upgrade].functionInc == pFunction->
							ref - REF_FUNCTION_START)
						{
							apProductionUpgrades[player][upgrade].available = TRUE;
							break;
						}
					}*/
                }
                //and the mission structures
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_FACTORY as libc::c_int as libc::c_uint &&
                           (*(pFunction as
                                  *mut PRODUCTION_UPGRADE_FUNCTION)).factory
                               as libc::c_int != 0 ||
                           (*(*psCurr).pStructureType).type_0 ==
                               REF_CYBORG_FACTORY as libc::c_int as
                                   libc::c_uint &&
                               (*(pFunction as
                                      *mut PRODUCTION_UPGRADE_FUNCTION)).cyborgFactory
                                   as libc::c_int != 0 ||
                           (*(*psCurr).pStructureType).type_0 ==
                               REF_VTOL_FACTORY as libc::c_int as libc::c_uint
                               &&
                               (*(pFunction as
                                      *mut PRODUCTION_UPGRADE_FUNCTION)).vtolFactory
                                   as libc::c_int != 0 {
                        //upgrade the Output for the structure
                        structureProductionUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
            }
            3 => {
                researchUpgrade(pFunction, player);
                //search the list of players structures for a Research Facility
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_RESEARCH as libc::c_int as libc::c_uint {
                        //upgrade the research points
						//researchUpgrade(pFunction, psCurr);
                        structureResearchUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                    //set the function upgrade flag for future factories being built
					/*for (upgrade = 0; upgrade < numResearchUpgrades; upgrade++)
					{
						if (apResearchUpgrades[player][upgrade].functionInc == pFunction->
							ref - REF_FUNCTION_START)
						{
							apResearchUpgrades[player][upgrade].available = TRUE;
							break;
						}
					}*/
                }
                //and the mission structures
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_RESEARCH as libc::c_int as libc::c_uint {
                        //upgrade the research points
                        structureResearchUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
            }
            11 => {
                powerUpgrade(pFunction, player);
                //search the list of players structures for a Power Gens
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_POWER_GEN as libc::c_int as libc::c_uint {
                        //upgrade the power points
                        structurePowerUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                //and the mission structure
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_POWER_GEN as libc::c_int as libc::c_uint {
                        //upgrade the power points
                        structurePowerUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
            }
            19 => {
                reArmUpgrade(pFunction, player);
                //search the list of players structures for a ReArm pad
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_REARM_PAD as libc::c_int as libc::c_uint {
                        //upgrade the rearm points
                        structureReArmUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                //and the mission structure
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_REARM_PAD as libc::c_int as libc::c_uint {
                        //upgrade the rearm points
                        structureReArmUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
            }
            12 => {
                repairFacUpgrade(pFunction, player);
                //search the list of players structures for a Power Gens
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_REPAIR_FACILITY as libc::c_int as libc::c_uint
                       {
                        //upgrade the repair points
                        structureRepairUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                //and the mission structure
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_REPAIR_FACILITY as libc::c_int as libc::c_uint
                       {
                        //upgrade the repair points
                        structureRepairUpgrade(psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
            }
            7 => {
                //for the current player, upgrade the weapon stats
                weaponUpgrade(pFunction, player);
            }
            16 => {
                //for the current player, upgrade the sensor stats
                sensorUpgrade(pFunction, player);
                //for each structure in the player's list, upgrade the sensor stat
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    structureSensorUpgrade(psCurr);
                    psCurr = (*psCurr).psNext
                }
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    structureSensorUpgrade(psCurr);
                    psCurr = (*psCurr).psNext
                }
                //for each droid in the player's list, upgrade the sensor stat
                psDroid = apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidSensorUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidSensorUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = mission.apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidSensorUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidSensorUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = apsLimboDroids[player as usize];
                while !psDroid.is_null() {
                    droidSensorUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidSensorUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
            }
            14 => {
                //for the current player, upgrade the ecm stats
                ecmUpgrade(pFunction, player);
                //for each structure in the player's list, upgrade the ecm stat
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    structureECMUpgrade(psCurr);
                    psCurr = (*psCurr).psNext
                }
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    structureECMUpgrade(psCurr);
                    psCurr = (*psCurr).psNext
                }
                //for each droid in the player's list, upgrade the ecm stat
                psDroid = apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidECMUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidECMUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = mission.apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidECMUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidECMUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = apsLimboDroids[player as usize];
                while !psDroid.is_null() {
                    droidECMUpgrade(psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidECMUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
            }
            13 => {
                //for the current player, upgrade the repair stats
                repairUpgrade(pFunction, player);
            }
            17 => {
                //for the current player, upgrade the constructor stats
                constructorUpgrade(pFunction, player);
            }
            15 => {
                //for each droid in the player's list, upgrade the body points
                psDroid = apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidBodyUpgrade(pFunction, psDroid);
                    psDroid = (*psDroid).psNext
                }
                psDroid = mission.apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    droidBodyUpgrade(pFunction, psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidSensorUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = apsLimboDroids[player as usize];
                while !psDroid.is_null() {
                    droidBodyUpgrade(pFunction, psDroid);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        upgradeTransporterDroids(psDroid,
                                                 Some(droidSensorUpgrade as
                                                          unsafe extern "C" fn(_:
                                                                                   *mut DROID)
                                                              -> ()));
                    }
                    psDroid = (*psDroid).psNext
                }
                //DO THIS AFTER so above calculations can use the previous upgrade values
				//for the current player, upgrade the body stats
                bodyUpgrade(pFunction, player);
            }
            9 => {
                //for each structure in the player's list, upgrade the stats
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    //do this for none wallDefense structs
                    if wallDefenceStruct((*psCurr).pStructureType) == 0 {
                        structureBodyUpgrade(pFunction, psCurr);
                        structureArmourUpgrade(pFunction, psCurr);
                        structureResistanceUpgrade(pFunction, psCurr);
                    }
                    //defense type can have resistance upgrade now - AB 19/02/99
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_DEFENSE as libc::c_int as libc::c_uint {
                        structureResistanceUpgrade(pFunction, psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    //do this for none wallDefense structs
                    if wallDefenceStruct((*psCurr).pStructureType) == 0 {
                        structureBodyUpgrade(pFunction, psCurr);
                        structureArmourUpgrade(pFunction, psCurr);
                        structureResistanceUpgrade(pFunction, psCurr);
                    }
                    //defense type can have resistance upgrade now - AB 19/02/99
                    if (*(*psCurr).pStructureType).type_0 ==
                           REF_DEFENSE as libc::c_int as libc::c_uint {
                        structureResistanceUpgrade(pFunction, psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                //DO THIS AFTER so above calculations can use the previous upgrade values
				//for the current player, upgrade the structure stats
                structureUpgrade(pFunction, player);
            }
            10 => {
                //for each structure in the player's list, upgrade the stats
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    //do this for wallDefense structs
                    if wallDefenceStruct((*psCurr).pStructureType) != 0 {
                        structureBodyUpgrade(pFunction, psCurr);
                        structureArmourUpgrade(pFunction, psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                psCurr = mission.apsStructLists[player as usize];
                while !psCurr.is_null() {
                    //do this for wallDefense structs
                    if wallDefenceStruct((*psCurr).pStructureType) != 0 {
                        structureBodyUpgrade(pFunction, psCurr);
                        structureArmourUpgrade(pFunction, psCurr);
                    }
                    psCurr = (*psCurr).psNext
                }
                //DO THIS AFTER so above calculations can use the previous upgrade values
				//for the current player, upgrade the wall/defence structure stats
                wallDefenceUpgrade(pFunction, player);
            }
            _ => {
                /*case(ARMOUR_UPGRADE_TYPE):
			{
				//for each structure in the player's list, upgrade the armour type
				for (psCurr = apsStructLists[player]; psCurr != NULL; psCurr =
					psCurr->psNext)
				{
					armourUpgrade(pFunction, psCurr);
				}
				//set the function upgrade flag for future factories being built
				for (upgrade = 0; upgrade < numArmourUpgrades; upgrade++)
				{
					if (apArmourUpgrades[player][upgrade].functionInc == pFunction->
						ref - REF_FUNCTION_START)
					{
						apArmourUpgrades[player][upgrade].available = TRUE;
						break;
					}
				}
				// message/sound in here for armour upgrade
				break;
			}*/
			/*case(REPAIR_UPGRADE_TYPE):
			{
				//for each structure in the player's list, upgrade the armour type
				for (psCurr = apsStructLists[player]; psCurr != NULL; psCurr =
					psCurr->psNext)
				{
					repairUpgrade(pFunction, psCurr);
				}
				//set the function upgrade flag for future factories being built
				for (upgrade = 0; upgrade < numRepairUpgrades; upgrade++)
				{
					if (apRepairUpgrades[player][upgrade].functionInc == pFunction->
						ref - REF_FUNCTION_START)
					{
						apRepairUpgrades[player][upgrade].available = TRUE;
						break;
					}
				}
			   	//message/sound in here for repair points upgraded
				break;
			}*/
			/*case(BODY_UPGRADE_TYPE):
			{
				//for each structure in the player's list, upgrade the armour type
				for (psCurr = apsStructLists[player]; psCurr != NULL; psCurr =
					psCurr->psNext)
				{
					bodyUpgrade(pFunction, psCurr);
				}
				//set the function upgrade flag for future factories being built
				for (upgrade = 0; upgrade < numBodyUpgrades; upgrade++)
				{
					if (apBodyUpgrades[player][upgrade].functionInc == pFunction->
						ref - REF_FUNCTION_START)
					{
						apBodyUpgrades[player][upgrade].available = TRUE;
						break;
					}
				}
			   	// message/sound in here for body points upgrade
				break;
			}*/
			/*case(RESISTANCE_UPGRADE_TYPE):
			{
				//for each structure in the player's list, upgrade the armour type
				for (psCurr = apsStructLists[player]; psCurr != NULL; psCurr =
					psCurr->psNext)
				{
					resistanceUpgrade(pFunction, psCurr);
				}
				//set the function upgrade flag for future factories being built
				for (upgrade = 0; upgrade < numResistanceUpgrades; upgrade++)
				{
					if (apResistanceUpgrades[player][upgrade].functionInc == pFunction->
						ref - REF_FUNCTION_START)
					{
						apResistanceUpgrades[player][upgrade].available = TRUE;
						break;
					}
				}
				// message/sound for resistance upgrade
				break;
			}*/
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Invalid function type\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 2057 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"researchResult\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        inc = inc.wrapping_add(1)
    }
    //Add message to player's list if Major Topic
    if (*pResearch).techCode as libc::c_int == TC_MAJOR as libc::c_int &&
           bDisplay != 0 {
        //only play sound if major topic
        if player as libc::c_uint == selectedPlayer {
            audio_QueueTrack(ID_SOUND_MAJOR_RESEARCH as libc::c_int);
            //add console text message
            addConsoleMessage(strresGetString(psStringRes,
                                              STR_INT_RESCOMPLETED as
                                                  libc::c_int as UDWORD),
                              LEFT_JUSTIFY);
        }
        //check there is viewdata for the research topic - just don't add message if not!
        if !(*pResearch).pViewData.is_null() {
            pMessage =
                addMessage(MSG_RESEARCH as libc::c_int as UDWORD,
                           0 as libc::c_int, player as UDWORD);
            if !pMessage.is_null() {
                (*pMessage).pViewData =
                    (*pResearch).pViewData as *mut MSG_VIEWDATA
            }
        }
    } else if player as libc::c_uint == selectedPlayer && bDisplay != 0 {
        audio_QueueTrack(ID_SOUND_RESEARCH_COMPLETED as libc::c_int);
        //add console text message
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_INT_RESCOMPLETED as libc::c_int
                                              as UDWORD), LEFT_JUSTIFY);
    }
    if player as libc::c_uint == selectedPlayer && bDisplay != 0 {
        psCBLastResearch = pResearch;
        eventFireCallbackTrigger(CALL_RESEARCHCOMPLETED as libc::c_int as
                                     TRIGGER_TYPE);
        psCBLastResearch = 0 as *mut RESEARCH
    };
    //just a debug check...
}
//this just inits all the research arrays
/*This function is called when the research files are reloaded*/
#[no_mangle]
pub unsafe extern "C" fn ResearchShutDown() -> BOOL {
    //we don't malloc these anymore this way - 10/12/98
	/*UDWORD inc;
	RESEARCH* pList = asResearch;

	for (inc=0; inc < numResearch; inc++)
	{
#if !defined (RESOURCE_NAMES) && !defined (STORE_RESOURCE_ID)
		FREE(pList->pName);
#endif
		//FREE(pList->pTechnologyName);
//		FREE(pList->pSubGroupName);
		if (pList->numRedArtefacts > 0)
		{
			FREE(pList->pRedArtefacts);
		}
		if (pList->numArteResults > 0)
		{
			FREE(pList->pArtefactResults);
			FREE(pList->pReplacedArtefacts);
		}
		if (pList->numPRRequired > 0)
		{
			FREE(pList->pPRList);
		}
		if (pList->numStructures > 0)
		{
			FREE(pList->pStructList);
		}
		if (pList->numRedStructs > 0)
		{
			FREE(pList->pRedStructs);
		}
		if (pList->numStructResults > 0)
		{
			FREE(pList->pStructureResults);
		}
		if (pList->numFunctions > 0)
		{
			FREE(pList->pFunctionList);
		}
		pList++;
	}

	if(numResearch) {
		FREE (asResearch);
	}

	//free up the lists for each player
	for (inc = 0; inc < MAX_PLAYERS; inc++)
	{
		if(asPlayerResList[inc]) {
			FREE (asPlayerResList[inc]);
		}
	}*/
    let mut i: UBYTE = 0;
    //don't allocate the memory now - done in initResearch, so just initialise it
    memset(asResearch as *mut libc::c_void, 0 as libc::c_int,
           (450 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<RESEARCH>()
                                               as libc::c_ulong));
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        memset(asPlayerResList[i as usize] as *mut libc::c_void,
               0 as libc::c_int,
               (450 as libc::c_int as
                    libc::c_uint).wrapping_mul(::std::mem::size_of::<PLAYER_RESEARCH>()
                                                   as libc::c_ulong));
        i = i.wrapping_add(1)
    }
    //and init all the other arrays used
    //needs to be UWORD sized for the Patches
    memset(pResearchPR as *mut libc::c_void, 0 as libc::c_int,
           (650 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    //memset(pResearchPR, 0, (MAX_RESEARCH_PR * sizeof(UBYTE)));
    memset(pResearchStructPR as *mut libc::c_void, 0 as libc::c_int,
           ((44 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    memset(pResearchFunc as *mut libc::c_void, 0 as libc::c_int,
           ((250 as libc::c_int + 25 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut FUNCTION>()
                                               as libc::c_ulong));
    memset(pResearchStructRed as *mut libc::c_void, 0 as libc::c_int,
           ((30 as libc::c_int + 2 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    memset(pResearchArteRed as *mut libc::c_void, 0 as libc::c_int,
           ((40 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    memset(pResearchStructRes as *mut libc::c_void, 0 as libc::c_int,
           ((84 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<UWORD>() as
                                               libc::c_ulong));
    memset(pResearchArteRes as *mut libc::c_void, 0 as libc::c_int,
           ((125 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    memset(pResearchArteRep as *mut libc::c_void, 0 as libc::c_int,
           ((125 as libc::c_int + 5 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<*mut COMP_BASE_STATS>()
                                               as libc::c_ulong));
    return 1 as libc::c_int;
}
//this free the memory used for the research
/*This function is called when a game finishes*/
#[no_mangle]
pub unsafe extern "C" fn ResearchRelease() -> BOOL {
    let mut i: UBYTE = 0;
    //free all the pre-defined arrays for research
    memFreeRelease(asResearch as *mut libc::c_void);
    asResearch = 0 as *mut RESEARCH;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        memFreeRelease(asPlayerResList[i as usize] as *mut libc::c_void);
        asPlayerResList[i as usize] = 0 as *mut PLAYER_RESEARCH;
        i = i.wrapping_add(1)
    }
    memFreeRelease(pResearchPR as *mut libc::c_void);
    pResearchPR = 0 as *mut UWORD;
    memFreeRelease(pResearchStructPR as *mut libc::c_void);
    pResearchStructPR = 0 as *mut UWORD;
    memFreeRelease(pResearchFunc as *mut libc::c_void);
    pResearchFunc = 0 as *mut *mut FUNCTION;
    memFreeRelease(pResearchStructRed as *mut libc::c_void);
    pResearchStructRed = 0 as *mut UWORD;
    memFreeRelease(pResearchArteRed as *mut libc::c_void);
    pResearchArteRed = 0 as *mut *mut COMP_BASE_STATS;
    memFreeRelease(pResearchStructRes as *mut libc::c_void);
    pResearchStructRes = 0 as *mut UWORD;
    memFreeRelease(pResearchArteRes as *mut libc::c_void);
    pResearchArteRes = 0 as *mut *mut COMP_BASE_STATS;
    memFreeRelease(pResearchArteRep as *mut libc::c_void);
    pResearchArteRep = 0 as *mut *mut COMP_BASE_STATS;
    return 1 as libc::c_int;
}
/*puts research facility on hold*/
/*puts research facility on hold*/
#[no_mangle]
pub unsafe extern "C" fn holdResearch(mut psBuilding: *mut STRUCTURE) {
    let mut psResFac: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"holdResearch: structure not a research facility\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              2256 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"holdResearch\x00")).as_ptr(),
              b"psBuilding->pStructureType->type == REF_RESEARCH\x00" as
                  *const u8 as *const libc::c_char);
    };
    psResFac = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    if !(*psResFac).psSubject.is_null() {
        //set the time the research facilty was put on hold
        (*psResFac).timeStartHold = gameTime;
        //play audio to indicate on hold
        if (*psBuilding).player as libc::c_uint == selectedPlayer {
            audio_PlayTrack(ID_SOUND_WINDOWCLOSE as libc::c_int);
        }
    };
}
/*release a research facility from hold*/
/*release a research facility from hold*/
#[no_mangle]
pub unsafe extern "C" fn releaseResearch(mut psBuilding: *mut STRUCTURE) {
    let mut psResFac: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"releaseResearch: structure not a research facility\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              2279 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"releaseResearch\x00")).as_ptr(),
              b"psBuilding->pStructureType->type == REF_RESEARCH\x00" as
                  *const u8 as *const libc::c_char);
    };
    psResFac = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    if !(*psResFac).psSubject.is_null() && (*psResFac).timeStartHold != 0 {
        //adjust the start time for the current subject
        if (*psResFac).timeStarted != 0 as libc::c_int as libc::c_uint {
            (*psResFac).timeStarted =
                ((*psResFac).timeStarted as
                     libc::c_uint).wrapping_add(gameTime.wrapping_sub((*psResFac).timeStartHold))
                    as UDWORD as UDWORD
        }
        (*psResFac).timeStartHold = 0 as libc::c_int as UDWORD
    };
}
/*

	Cancel All Research for player 0

*/
#[no_mangle]
pub unsafe extern "C" fn CancelAllResearch(mut pl: UDWORD) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psCurr = apsStructLists[pl as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
            if !((*psCurr).pFunctionality as *mut RESEARCH_FACILITY).is_null()
                   &&
                   !(*((*psCurr).pFunctionality as
                           *mut RESEARCH_FACILITY)).psSubject.is_null() {
                debug(LOG_NEVER,
                      b"canceling research for %p\n\x00" as *const u8 as
                          *const libc::c_char, psCurr);
                cancelResearch(psCurr);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
/* sets the status of the topic to cancelled and stores the current research
   points accquired */
/* sets the status of the topic to cancelled and stores the current research
   points accquired */
#[no_mangle]
pub unsafe extern "C" fn cancelResearch(mut psBuilding: *mut STRUCTURE) {
    let mut topicInc: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut psResFac: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"cancelResearch: structure not a research facility\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              2330 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"cancelResearch\x00")).as_ptr(),
              b"psBuilding->pStructureType->type == REF_RESEARCH\x00" as
                  *const u8 as *const libc::c_char);
    };
    psResFac = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    topicInc =
        ((*psResFac).psSubject as
             *mut RESEARCH).wrapping_offset_from(asResearch) as libc::c_int as
            UDWORD;
    if topicInc > numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"cancelResearch: invalid research topic\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"research.c\x00" as *const u8 as *const libc::c_char,
                  2336 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"cancelResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    pPlayerRes =
        asPlayerResList[(*psBuilding).player as
                            usize].offset(topicInc as isize);
    if (*(*psBuilding).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
        //check if waiting to accrue power
        if (*psResFac).timeStarted == 0 as libc::c_int as libc::c_uint {
            //return the power
            addPower((*psBuilding).player as UDWORD,
                     (*psResFac).powerAccrued);
            (*psResFac).powerAccrued = 0 as libc::c_int as UDWORD;
            //reset this topic as not having been researched
            (*pPlayerRes).ResearchStatus =
                ((*pPlayerRes).ResearchStatus as libc::c_int &
                     !(0x1 as libc::c_int | 0x2 as libc::c_int |
                           0x4 as libc::c_int)) as UBYTE
        } else {
            // only PC version saves these
			/*store the points - need to keep this so can add points after the topic
			has been cancelled and restarted*/
            (*pPlayerRes).currentPoints =
                ((*pPlayerRes).currentPoints as
                     libc::c_uint).wrapping_add((*psResFac).researchPoints.wrapping_mul(gameTime.wrapping_sub((*psResFac).timeStarted)).wrapping_div(1000
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint))
                    as UDWORD as UDWORD;
            //set the researched flag
            (*pPlayerRes).ResearchStatus =
                ((*pPlayerRes).ResearchStatus as libc::c_int &
                     !(0x1 as libc::c_int | 0x2 as libc::c_int |
                           0x4 as libc::c_int) | 0x2 as libc::c_int) as UBYTE
        }
        sendReseachStatus(psBuilding, topicInc, (*psBuilding).player,
                          0 as libc::c_int);
        //set the researched flag - only set the flag to cancelled if it got past the accruePower stage
		//pPlayerRes->researched = CANCELLED_RESEARCH;
        (*psResFac).psSubject = 0 as *mut _base_stats
    };
}
//initialise the research facility's subject
/* For a given view data get the research this is related to */
/* For a given view data get the research this is related to */
#[no_mangle]
pub unsafe extern "C" fn getResearchForMsg(mut pViewData: *mut VIEWDATA)
 -> *mut RESEARCH {
    let mut inc: UDWORD = 0;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numResearch {
        psResearch = asResearch.offset(inc as isize);
        //compare the pointer
        if (*psResearch).pViewData == pViewData { return psResearch }
        inc = inc.wrapping_add(1)
    }
    return 0 as *mut RESEARCH;
}
//set the iconID based on the name read in in the stats
//set the iconID based on the name read in in the stats
unsafe extern "C" fn setIconID(mut pIconName: *mut STRING,
                               mut pName: *mut STRING) -> UWORD {
    //compare the names with those created in 'Framer'
    if strcmp(pIconName,
              b"IMAGE_ROCKET\x00" as *const u8 as *const libc::c_char) == 0 {
        return IMAGE_ROCKET as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_CANNON\x00" as *const u8 as *const libc::c_char) == 0 {
        return IMAGE_CANNON as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_HOVERCRAFT\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_HOVERCRAFT as libc::c_int as UWORD
    }
    if strcmp(pIconName, b"IMAGE_ECM\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_ECM as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_PLASCRETE\x00" as *const u8 as *const libc::c_char) == 0
       {
        return IMAGE_PLASCRETE as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_TRACKS\x00" as *const u8 as *const libc::c_char) == 0 {
        return IMAGE_TRACKS as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_DROIDTECH\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_RES_DROIDTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_WEAPONTECH\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_RES_WEAPONTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_COMPUTERTECH\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return IMAGE_RES_COMPUTERTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_POWERTECH\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_RES_POWERTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_SYSTEMTECH\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_RES_SYSTEMTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_STRUCTURETECH\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return IMAGE_RES_STRUCTURETECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_CYBORGTECH\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return IMAGE_RES_CYBORGTECH as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_DEFENCE\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_DEFENCE as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_QUESTIONMARK\x00" as *const u8 as
                  *const libc::c_char) == 0 {
        return IMAGE_RES_QUESTIONMARK as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_GRPACC\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_GRPACC as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_GRPUPG\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_GRPUPG as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_GRPREP\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_GRPREP as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_GRPROF\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_GRPROF as libc::c_int as UWORD
    }
    if strcmp(pIconName,
              b"IMAGE_RES_GRPDAM\x00" as *const u8 as *const libc::c_char) ==
           0 {
        return IMAGE_RES_GRPDAM as libc::c_int as UWORD
    }
    //add more names as images are created
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Invalid icon graphic %s for topic %s\x00" as *const u8 as
                  *const libc::c_char, pIconName, pName);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              2521 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"setIconID\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int as UWORD;
    // Should never get here.
}
#[no_mangle]
pub unsafe extern "C" fn mapRIDToIcon(mut rid: UDWORD) -> SDWORD {
    match rid {
        0 => { return IMAGE_ROCKET as libc::c_int }
        1 => { return IMAGE_CANNON as libc::c_int }
        2 => { return IMAGE_HOVERCRAFT as libc::c_int }
        3 => { return IMAGE_ECM as libc::c_int }
        4 => { return IMAGE_PLASCRETE as libc::c_int }
        5 => { return IMAGE_TRACKS as libc::c_int }
        6 => { return IMAGE_RES_DROIDTECH as libc::c_int }
        7 => { return IMAGE_RES_WEAPONTECH as libc::c_int }
        8 => { return IMAGE_RES_COMPUTERTECH as libc::c_int }
        9 => { return IMAGE_RES_POWERTECH as libc::c_int }
        10 => { return IMAGE_RES_SYSTEMTECH as libc::c_int }
        11 => { return IMAGE_RES_STRUCTURETECH as libc::c_int }
        12 => { return IMAGE_RES_CYBORGTECH as libc::c_int }
        13 => { return IMAGE_RES_DEFENCE as libc::c_int }
        14 => { return IMAGE_RES_QUESTIONMARK as libc::c_int }
        15 => { return IMAGE_RES_GRPACC as libc::c_int }
        16 => { return IMAGE_RES_GRPUPG as libc::c_int }
        17 => { return IMAGE_RES_GRPREP as libc::c_int }
        18 => { return IMAGE_RES_GRPROF as libc::c_int }
        19 => { return IMAGE_RES_GRPDAM as libc::c_int }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy mapping request for RID to icon\x00" as
                          *const u8 as
                          *const libc::c_char); //pass back a value that can never have been set up
            }; //pass back a value that can never have been set up
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      2595 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"mapRIDToIcon\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return -(1 as libc::c_int)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapIconToRID(mut iconID: UDWORD) -> SDWORD {
    match iconID {
        103 => { return RID_ROCKET as libc::c_int }
        104 => { return RID_CANNON as libc::c_int }
        105 => { return RID_HOVERCRAFT as libc::c_int }
        107 => { return RID_ECM as libc::c_int }
        108 => { return RID_PLASCRETE as libc::c_int }
        252 => { return RID_TRACKS as libc::c_int }
        256 => { return RID_DROIDTECH as libc::c_int }
        257 => { return RID_WEAPONTECH as libc::c_int }
        258 => { return RID_COMPUTERTECH as libc::c_int }
        259 => { return RID_POWERTECH as libc::c_int }
        260 => { return RID_SYSTEMTECH as libc::c_int }
        261 => { return RID_STRUCTURETECH as libc::c_int }
        349 => { return RID_CYBORGTECH as libc::c_int }
        395 => { return RID_DEFENCE as libc::c_int }
        262 => { return RID_QUESTIONMARK as libc::c_int }
        460 => { return RID_GRPACC as libc::c_int }
        457 => { return RID_GRPUPG as libc::c_int }
        458 => { return RID_GRPREP as libc::c_int }
        459 => { return RID_GRPROF as libc::c_int }
        461 => { return RID_GRPDAM as libc::c_int }
        _ => { return -(1 as libc::c_int) }
    };
}
/* returns a pointer to a component based on the name - used to load in the research */
unsafe extern "C" fn getComponentDetails(mut pName: *mut STRING,
                                         mut pCompName: *mut STRING)
 -> *mut COMP_BASE_STATS {
    let mut stat: UDWORD = 0;
    let mut size: UDWORD = 0;
    let mut quantity: UDWORD = 0;
    let mut address: UDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut pArtefact: *mut COMP_BASE_STATS = 0 as *mut COMP_BASE_STATS;
    stat = componentType(pName);
    //get the stat list
    match stat {
        1 => {
            pArtefact = asBodyStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<BODY_STATS>() as libc::c_ulong;
            quantity = numBodyStats
        }
        2 => {
            pArtefact = asBrainStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<BRAIN_STATS>() as libc::c_ulong;
            quantity = numBrainStats
        }
        3 => {
            pArtefact = asPropulsionStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<PROPULSION_STATS>() as libc::c_ulong;
            quantity = numPropulsionStats
        }
        4 => {
            pArtefact = asRepairStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<REPAIR_STATS>() as libc::c_ulong;
            quantity = numRepairStats
        }
        5 => {
            pArtefact = asECMStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<ECM_STATS>() as libc::c_ulong;
            quantity = numECMStats
        }
        6 => {
            pArtefact = asSensorStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<SENSOR_STATS>() as libc::c_ulong;
            quantity = numSensorStats
        }
        8 => {
            /*case COMP_PROGRAM:
		{
			pArtefact = (COMP_BASE_STATS*)asProgramStats;
			size = sizeof(PROGRAM_STATS);
			quantity = numProgramStats;
			break;
		}*/
            pArtefact = asWeaponStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<WEAPON_STATS>() as libc::c_ulong;
            quantity = numWeaponStats
        }
        7 => {
            pArtefact = asConstructStats as *mut COMP_BASE_STATS;
            size = ::std::mem::size_of::<CONSTRUCT_STATS>() as libc::c_ulong;
            quantity = numConstructStats
        }
        _ => {
            //COMP_UNKNOWN should be an error
            debug(LOG_ERROR,
                  b"Unknown artefact type  - %s\x00" as *const u8 as
                      *const libc::c_char, pName);
            abort();
        }
    }
    address = pArtefact as UDWORD;
    inc = 0 as libc::c_int as UDWORD;
    while inc < quantity {
        if strcmp((*pArtefact).pName, pCompName) == 0 { return pArtefact }
        address =
            (address as libc::c_uint).wrapping_add(size) as UDWORD as UDWORD;
        pArtefact = address as *mut COMP_BASE_STATS;
        inc = inc.wrapping_add(1)
    }
    debug(LOG_ERROR,
          b"Cannot find component %s\x00" as *const u8 as *const libc::c_char,
          pCompName);
    abort();
}
/* For a given view data get the research this is related to */
//return a pointer to a research topic based on the name
#[no_mangle]
pub unsafe extern "C" fn getResearch(mut pName: *mut STRING,
                                     mut resName: BOOL) -> *mut RESEARCH {
    let mut inc: UDWORD = 0;
    //need to get the in game name if a resource name has been passed in
    if resName != 0 {
        if getResourceName(pName) == 0 {
            debug(LOG_ERROR,
                  b"getResearch: resource not found\x00" as *const u8 as
                      *const libc::c_char);
            return 0 as *mut RESEARCH
        }
    }
    inc = 0 as libc::c_int as UDWORD;
    while inc < numResearch {
        if strcasecmp((*asResearch.offset(inc as isize)).pName, pName) == 0 {
            return &mut *asResearch.offset(inc as isize) as *mut RESEARCH
        }
        inc = inc.wrapping_add(1)
    }
    inc = 0 as libc::c_int as UDWORD;
    while inc < numResearch {
        debug(LOG_ERROR,
              b"  Research %d: %s\x00" as *const u8 as *const libc::c_char,
              inc, (*asResearch.offset(inc as isize)).pName);
        inc = inc.wrapping_add(1)
    }
    debug(LOG_ERROR,
          b"Unknown research - %s\x00" as *const u8 as *const libc::c_char,
          pName);
    return 0 as *mut RESEARCH;
}
/* looks through the players lists of structures and droids to see if any are using
 the old component - if any then replaces them with the new component */
unsafe extern "C" fn replaceComponent(mut pNewComponent: *mut COMP_BASE_STATS,
                                      mut pOldComponent: *mut COMP_BASE_STATS,
                                      mut player: UBYTE) {
    //DROID			*psDroid;
    let mut psTemplates: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    //STRUCTURE		*psStructure;
    let mut inc: UDWORD = 0;
    let mut oldType: UDWORD = 0;
    let mut newType: UDWORD = 0;
    let mut oldCompInc: UDWORD = 0;
    let mut newCompInc: UDWORD = 0;
    //get the type and index of the old component
    oldType = statType((*pOldComponent).ref_0);
    oldCompInc = (*pOldComponent).ref_0.wrapping_sub(statRefStart(oldType));
    //get the type and index of the new component
    newType = statType((*pNewComponent).ref_0);
    newCompInc = (*pNewComponent).ref_0.wrapping_sub(statRefStart(newType));
    //check old and new type are the same
    if oldType != newType { return }
    replaceDroidComponent(apsDroidLists[player as usize], oldType, oldCompInc,
                          newCompInc);
    replaceDroidComponent(mission.apsDroidLists[player as usize], oldType,
                          oldCompInc, newCompInc);
    replaceDroidComponent(apsLimboDroids[player as usize], oldType,
                          oldCompInc, newCompInc);
    //check thru the droids
	/*for (psDroid = apsDroidLists[player]; psDroid != NULL; psDroid = psDroid->psNext)
	{
		switch(oldType)
		{
			case COMP_BODY:
			case COMP_BRAIN:
			case COMP_PROPULSION:
			case COMP_REPAIRUNIT:
			case COMP_ECM:
			case COMP_SENSOR:
			case COMP_CONSTRUCT:
				if (psDroid->asBits[oldType].nStat == oldCompInc)
				{
					psDroid->asBits[oldType].nStat = (UBYTE)newCompInc;
				}
				break;
			//case COMP_PROGRAM:
			//	for (inc=0; inc < psDroid->numProgs; inc++)
			//	{
			//		if ((psDroid->asProgs[inc].psStats->ref - REF_PROGRAM_START) ==
			//			oldCompInc)
			//		{
			//			psDroid->asProgs[inc].psStats = (asProgramStats + newCompInc);
			//		}
			//	}
			//	break;
			case COMP_WEAPON:
                //can only be one weapon now
				//for (inc=0; inc < psDroid->numWeaps; inc++)
                if (psDroid->asWeaps[0].nStat > 0)
				{
					//if (psDroid->asWeaps[inc].nStat == oldCompInc)
                    if (psDroid->asWeaps[0].nStat == oldCompInc)
					{
						//psDroid->asWeaps[inc].nStat = newCompInc;
                        psDroid->asWeaps[0].nStat = newCompInc;
					}
				}
				break;
			default:
				//unknown comp type
				DBERROR(("Unknown component type - invalid droid"));
				return;
		}
	}*/
    //check thru the templates
    psTemplates = apsDroidTemplates[player as usize];
    while !psTemplates.is_null() {
        match oldType {
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                if (*psTemplates).asParts[oldType as usize] ==
                       oldCompInc as SDWORD {
                    (*psTemplates).asParts[oldType as usize] =
                        newCompInc as SDWORD
                }
            }
            8 => {
                //case COMP_PROGRAM:
			//	for (inc=0; inc < psTemplates->numProgs; inc++)
			//	{
			//		if (psTemplates->asProgs[inc] == oldCompInc)
			//		{
			//			psTemplates->asProgs[inc] = newCompInc;
			//		}
			//	}
			//	break;
                inc = 0 as libc::c_int as UDWORD;
                while inc < (*psTemplates).numWeaps {
                    if (*psTemplates).asWeaps[inc as usize] == oldCompInc {
                        (*psTemplates).asWeaps[inc as usize] = newCompInc
                    }
                    inc = inc.wrapping_add(1)
                }
            }
            _ => {
                //unknown comp type
                debug(LOG_ERROR,
                      b"Unknown component type - invalid Template\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
        }
        psTemplates = (*psTemplates).psNext
    }
    replaceStructureComponent(apsStructLists[player as usize], oldType,
                              oldCompInc, newCompInc, player);
    replaceStructureComponent(mission.apsStructLists[player as usize],
                              oldType, oldCompInc, newCompInc, player);
    //check thru the structures
	/*for (psStructure = apsStructLists[player]; psStructure != NULL; psStructure =
		psStructure->psNext)
	{
		switch (oldType)
		{
			case COMP_ECM:
				if (psStructure->pStructureType->pECM == (asECMStats + oldCompInc))
				{
					psStructure->ecmPower = (UWORD)(asECMStats + newCompInc)->power;
				}
				break;
			case COMP_SENSOR:
				if (psStructure->pStructureType->pSensor == (asSensorStats + oldCompInc))
				{
					//psStructure->sensorPower = (asSensorStats + newCompInc)->power;
					psStructure->sensorPower = (UWORD)sensorPower(asSensorStats +
						newCompInc,player);
					//psStructure->sensorRange = (asSensorStats + newCompInc)->range;
					psStructure->sensorRange = (UWORD)sensorRange(asSensorStats +
						newCompInc,player);
				}
				break;
			case COMP_WEAPON:
				//for (inc=0; inc < psStructure->numWeaps; inc++)
                //can only be one weapon now
                if (psStructure->asWeaps[0].nStat > 0)
				{
					//if (psStructure->asWeaps[inc].nStat == oldCompInc)
					//{
					//	psStructure->asWeaps[inc].nStat = newCompInc;
					//}
					if (psStructure->asWeaps[0].nStat == oldCompInc)
					{
						psStructure->asWeaps[0].nStat = newCompInc;
					}
				}
				break;
			default:
				//ignore all other component types
				break;
		}
	}*/
}
/*Looks through all the currently allocated stats to check the name is not
a duplicate*/
unsafe extern "C" fn checkResearchName(mut psResearch: *mut RESEARCH,
                                       mut numStats: UDWORD) -> BOOL {
    let mut inc: UDWORD = 0;
    let mut pName: *mut libc::c_char = (*psResearch).pName;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numStats {
        if strcmp((*asResearch.offset(inc as isize)).pName, pName) == 0 {
            //oops! found the name
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Research name has already been used - %s\x00" as
                          *const u8 as *const libc::c_char, pName);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"research.c\x00" as *const u8 as *const libc::c_char,
                      3003 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"checkResearchName\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        inc = inc.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Sets the 'possible' flag for a player's research so the topic will appear in 
the research list next time the Research Facilty is selected */
/* Sets the 'possible' flag for a player's research so the topic will appear in
the research list next time the Research Facilty is selected */
#[no_mangle]
pub unsafe extern "C" fn enableResearch(mut psResearch: *mut RESEARCH,
                                        mut player: UDWORD) -> BOOL {
    let mut inc: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[player as usize];
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut resFree: BOOL = 0 as libc::c_int;
    inc =
        psResearch.wrapping_offset_from(asResearch) as libc::c_int as UDWORD;
    if inc > numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"enableResearch: Invalid research topic - %s\x00" as
                      *const u8 as *const libc::c_char,
                  getResearchName(psResearch));
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"research.c\x00" as *const u8 as *const libc::c_char,
                  3043 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"enableResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //found, so set the flag
    let ref mut fresh9 = (*pPlayerRes.offset(inc as isize)).ResearchStatus;
    *fresh9 = (*fresh9 as libc::c_int | 0x80 as libc::c_int) as UBYTE;
    if player == selectedPlayer {
        //set the research reticule button to flash if research facility is free
        psStruct = apsStructLists[selectedPlayer as usize];
        while !psStruct.is_null() {
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_RESEARCH as libc::c_int as libc::c_uint &&
                   (*psStruct).status as libc::c_int ==
                       SS_BUILT as libc::c_int &&
                   (*((*psStruct).pFunctionality as
                          *mut RESEARCH_FACILITY)).psSubject.is_null() {
                resFree = 1 as libc::c_int;
                break ;
            } else { psStruct = (*psStruct).psNext }
        }
        if resFree != 0 { flashReticuleButton(5 as libc::c_int as UDWORD); }
    }
    return 1 as libc::c_int;
}
/*find the last research topic of importance that the losing player did and 
'give' the results to the reward player*/
/*find the last research topic of importance that the losing player did and
'give' the results to the reward player*/
#[no_mangle]
pub unsafe extern "C" fn researchReward(mut losingPlayer: UBYTE,
                                        mut rewardPlayer: UBYTE) {
    let mut topicIndex: UDWORD = 0;
    let mut researchPoints: UDWORD = 0;
    let mut rewardID: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFacility: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    rewardID = 0 as libc::c_int as UDWORD;
    researchPoints = rewardID;
    topicIndex = researchPoints;
    //look through the losing players structures to find a research facility
    psStruct = apsStructLists[losingPlayer as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
            psFacility = (*psStruct).pFunctionality as *mut RESEARCH_FACILITY;
            if !(*psFacility).psBestTopic.is_null() {
                topicIndex =
                    (*((*psFacility).psBestTopic as
                           *mut RESEARCH)).ref_0.wrapping_sub(0xb0000 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint);
                if topicIndex != 0 {
                    //if it cost more - it is better (or should be)
                    if researchPoints <
                           (*asResearch.offset(topicIndex as
                                                   isize)).researchPoints as
                               libc::c_uint {
                        //store the 'best' topic
                        researchPoints =
                            (*asResearch.offset(topicIndex as
                                                    isize)).researchPoints as
                                UDWORD;
                        rewardID = topicIndex
                    }
                }
            }
        }
        psStruct = (*psStruct).psNext
    }
    //if a topic was found give the reward player the results of that research
    if rewardID != 0 {
        researchResult(rewardID, rewardPlayer, 1 as libc::c_int);
        if rewardPlayer as libc::c_uint == selectedPlayer {
            //addConsoleMessage(strresGetString(psStringRes,STR_GAM_RESREWARD), DEFAULT_JUSTIFY);
            //name the actual reward
            //addConsoleMessage(asResearch[rewardID].pName, DEFAULT_JUSTIFY);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"%s :- %s\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_GAM_RESREWARD as libc::c_int as
                                        UDWORD),
                    getName((*asResearch.offset(rewardID as isize)).pName));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
    };
    /* Not worth mentioning if nothing useful to gain?
    if (rewardPlayer == selectedPlayer)
	{
		addConsoleMessage(strresGetString(psStringRes,STR_GAM_RESREWARD), DEFAULT_JUSTIFY);
	}*/
}
/*checks that the research has loaded up as expected - must be done after
all research parts have been loaded*/
#[no_mangle]
pub unsafe extern "C" fn checkResearchStats() -> BOOL {
    let mut resInc: UDWORD = 0;
    let mut inc: UDWORD = 0;
    resInc = 0 as libc::c_int as UDWORD;
    while resInc < numResearch {
        if (*asResearch.offset(resInc as isize)).numPRRequired as libc::c_int
               == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as isize)).pPRList.is_null() {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: PreReq for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3150 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numPRRequired as
                          libc::c_uint {
                if *(*asResearch.offset(resInc as
                                            isize)).pPRList.offset(inc as
                                                                       isize)
                       as libc::c_uint > numResearch {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"checkResearchStats: Invalid PreReq for topic %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*asResearch.offset(resInc as isize)).pName);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"research.c\x00" as *const u8 as
                                  *const libc::c_char, 3162 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numStructures as libc::c_int
               == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as isize)).pStructList.is_null() {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: StructureList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3174 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numStructures as
                          libc::c_uint {
                if *(*asResearch.offset(resInc as
                                            isize)).pStructList.offset(inc as
                                                                           isize)
                       as libc::c_uint > numStructureStats {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"checkResearchStats: Invalid Structure for topic %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*asResearch.offset(resInc as isize)).pName);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"research.c\x00" as *const u8 as
                                  *const libc::c_char, 3186 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numFunctions as libc::c_int
               == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as isize)).pFunctionList.is_null()
               {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: FunctionList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3197 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numFunctions as
                          libc::c_uint {
                if (**(*asResearch.offset(resInc as
                                              isize)).pFunctionList.offset(inc
                                                                               as
                                                                               isize)).ref_0.wrapping_sub(0xe0000
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                       > numFunctions {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"checkResearchStats: Invalid function for %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*asResearch.offset(resInc as isize)).pName);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"research.c\x00" as *const u8 as
                                  *const libc::c_char, 3209 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numRedStructs as libc::c_int
               == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as isize)).pRedStructs.is_null() {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: Redundant StructList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3219 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numRedStructs as
                          libc::c_uint {
                if *(*asResearch.offset(resInc as
                                            isize)).pRedStructs.offset(inc as
                                                                           isize)
                       as libc::c_uint > numStructureStats {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"checkResearchStats: Invalid Redundant Structure for topic %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*asResearch.offset(resInc as isize)).pName);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"research.c\x00" as *const u8 as
                                  *const libc::c_char, 3231 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numStructResults as
               libc::c_int == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as
                                        isize)).pStructureResults.is_null() {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: Result StructList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3242 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numStructResults
                          as libc::c_uint {
                if *(*asResearch.offset(resInc as
                                            isize)).pStructureResults.offset(inc
                                                                                 as
                                                                                 isize)
                       as libc::c_uint > numStructureStats {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"checkResearchStats: Invalid Result Structure for topic %s\x00"
                                  as *const u8 as *const libc::c_char,
                              (*asResearch.offset(resInc as isize)).pName);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"research.c\x00" as *const u8 as
                                  *const libc::c_char, 3254 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numArteResults as libc::c_int
               == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as
                                        isize)).pArtefactResults.is_null() {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: CompResultList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3265 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numArteResults as
                          libc::c_uint {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: Invalid Comp Result for topic %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3276 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"PTRVALID(asResearch[resInc].pArtefactResults[inc], sizeof(COMP_BASE_STATS*))\x00"
                              as *const u8 as *const libc::c_char);
                };
                inc = inc.wrapping_add(1)
            }
        }
        if (*asResearch.offset(resInc as isize)).numRedArtefacts as
               libc::c_int == 0 as libc::c_int {
            if !(*asResearch.offset(resInc as isize)).pRedArtefacts.is_null()
               {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: RedundantCompList for topic %s should be NULL\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3285 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        } else {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asResearch.offset(resInc as isize)).numRedArtefacts as
                          libc::c_uint {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkResearchStats: Invalid Redundant Comp for topic %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*asResearch.offset(resInc as isize)).pName);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"research.c\x00" as *const u8 as
                              *const libc::c_char, 3296 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"checkResearchStats\x00")).as_ptr(),
                          b"PTRVALID(asResearch[resInc].pRedArtefacts[inc], sizeof(COMP_BASE_STATS*))\x00"
                              as *const u8 as *const libc::c_char);
                };
                inc = inc.wrapping_add(1)
            }
        }
        resInc = resInc.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*flag self repair so droids can start when idle*/
#[no_mangle]
pub unsafe extern "C" fn enableSelfRepair(mut player: UBYTE) {
    bSelfRepair[player as usize] = 1 as libc::c_int as UBYTE;
}
/*check to see if any research has been completed that enables self repair*/
/*check to see if any research has been completed that enables self repair*/
#[no_mangle]
pub unsafe extern "C" fn selfRepairEnabled(mut player: UBYTE) -> BOOL {
    if bSelfRepair[player as usize] != 0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*checks the stat to see if its of type wall or defence*/
/*checks the stat to see if its of type wall or defence*/
#[no_mangle]
pub unsafe extern "C" fn wallDefenceStruct(mut psStats: *mut STRUCTURE_STATS)
 -> BOOL {
    if (*psStats).type_0 == REF_DEFENSE as libc::c_int as libc::c_uint ||
           (*psStats).type_0 == REF_WALL as libc::c_int as libc::c_uint ||
           (*psStats).type_0 == REF_WALLCORNER as libc::c_int as libc::c_uint
           ||
           (*psStats).type_0 == REF_BLASTDOOR as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*for a given list of droids, replace the old component if exists*/
unsafe extern "C" fn replaceDroidComponent(mut pList: *mut DROID,
                                           mut oldType: UDWORD,
                                           mut oldCompInc: UDWORD,
                                           mut newCompInc: UDWORD) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //check thru the droids
    psDroid = pList;
    while !psDroid.is_null() {
        switchComponent(psDroid, oldType, oldCompInc, newCompInc);
        //need to replace the units inside the transporter
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            replaceTransDroidComponents(psDroid, oldType, oldCompInc,
                                        newCompInc);
        }
        psDroid = (*psDroid).psNext
    };
}
/*replaces any components necessary for units that are inside a transporter*/
unsafe extern "C" fn replaceTransDroidComponents(mut psTransporter:
                                                     *mut DROID,
                                                 mut oldType: UDWORD,
                                                 mut oldCompInc: UDWORD,
                                                 mut newCompInc: UDWORD) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"replaceTransUnitComponents: invalid unit type\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              3363 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"replaceTransDroidComponents\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    psCurr = (*(*psTransporter).psGroup).psList;
    while !psCurr.is_null() {
        if psCurr != psTransporter {
            switchComponent(psCurr, oldType, oldCompInc, newCompInc);
        }
        psCurr = (*psCurr).psGrpNext
    };
}
unsafe extern "C" fn replaceStructureComponent(mut pList: *mut STRUCTURE,
                                               mut oldType: UDWORD,
                                               mut oldCompInc: UDWORD,
                                               mut newCompInc: UDWORD,
                                               mut player: UBYTE) {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //if the type is not one we are interested in, then don't bother checking
    if !(oldType == COMP_ECM as libc::c_int as libc::c_uint ||
             oldType == COMP_SENSOR as libc::c_int as libc::c_uint ||
             oldType == COMP_WEAPON as libc::c_int as libc::c_uint) {
        return
    }
    //check thru the structures
    psStructure = pList;
    while !psStructure.is_null() {
        match oldType {
            5 => {
                if (*(*psStructure).pStructureType).pECM ==
                       asECMStats.offset(oldCompInc as isize) {
                    (*psStructure).ecmPower =
                        (*asECMStats.offset(newCompInc as isize)).power as
                            UWORD
                }
            }
            6 => {
                if (*(*psStructure).pStructureType).pSensor ==
                       asSensorStats.offset(oldCompInc as isize) {
                    //psStructure->sensorPower = (asSensorStats + newCompInc)->power;
                    (*psStructure).sensorPower =
                        sensorPower(asSensorStats.offset(newCompInc as isize),
                                    player) as UWORD;
                    //psStructure->sensorRange = (asSensorStats + newCompInc)->range;
                    (*psStructure).sensorRange =
                        sensorRange(asSensorStats.offset(newCompInc as isize),
                                    player) as UWORD
                }
            }
            8 => {
                //for (inc=0; inc < psStructure->numWeaps; inc++)
                //can only be one weapon now
                if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
                       0 as libc::c_int as libc::c_uint {
                    /*if (psStructure->asWeaps[inc].nStat == oldCompInc)
					{
						psStructure->asWeaps[inc].nStat = newCompInc;
					}*/
                    if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat
                           == oldCompInc {
                        (*psStructure).asWeaps[0 as libc::c_int as
                                                   usize].nStat = newCompInc
                    }
                }
            }
            _ => { }
        }
        psStructure = (*psStructure).psNext
    };
}
/*swaps the old component for the new one for a specific droid*/
unsafe extern "C" fn switchComponent(mut psDroid: *mut DROID,
                                     mut oldType: UDWORD,
                                     mut oldCompInc: UDWORD,
                                     mut newCompInc: UDWORD) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"switchComponent:invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"research.c\x00" as *const u8 as *const libc::c_char,
              3436 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"switchComponent\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    match oldType {
        1 | 2 | 3 | 4 | 5 | 6 | 7 => {
            if (*psDroid).asBits[oldType as usize].nStat as libc::c_uint ==
                   oldCompInc {
                (*psDroid).asBits[oldType as usize].nStat =
                    newCompInc as UBYTE
            }
        }
        8 => {
            /*case COMP_PROGRAM:
			for (inc=0; inc < psDroid->numProgs; inc++)
			{
				if ((psDroid->asProgs[inc].psStats->ref - REF_PROGRAM_START) ==
					oldCompInc)
				{
					psDroid->asProgs[inc].psStats = (asProgramStats + newCompInc);
				}
			}
			break;*/
            //can only be one weapon now
			//for (inc=0; inc < psDroid->numWeaps; inc++)
            if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                   0 as libc::c_int as libc::c_uint {
                //if (psDroid->asWeaps[inc].nStat == oldCompInc)
                if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
                       oldCompInc {
                    //psDroid->asWeaps[inc].nStat = newCompInc;
                    (*psDroid).asWeaps[0 as libc::c_int as usize].nStat =
                        newCompInc
                }
            }
        }
        _ => {
            //unknown comp type
            debug(LOG_ERROR,
                  b"Unknown component type - invalid droid\x00" as *const u8
                      as *const libc::c_char);
            abort();
        }
    };
}
