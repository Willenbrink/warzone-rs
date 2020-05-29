use ::libc;
extern "C" {
    pub type _formation;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn rand() -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    /*return the name to display for the interface - valid for OBJECTS and STATS*/
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    #[no_mangle]
    fn hqReward(losingPlayer: UBYTE, rewardPlayer: UBYTE);
    #[no_mangle]
    fn getMaxDroids(PlayerNumber: UDWORD) -> UDWORD;
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
    /* Create a feature on the map */
    #[no_mangle]
    fn buildFeature(psStats: *mut FEATURE_STATS, x: UDWORD, y: UDWORD,
                    FromSave: BOOL) -> *mut FEATURE;
    // free up a feature with no visual effects
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
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
    #[no_mangle]
    static mut alliances: [[UBYTE; 8]; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    #[no_mangle]
    fn giftSingleDroid(psD: *mut DROID, to: UDWORD) -> *mut DROID;
    #[no_mangle]
    fn pickATileGen(x: *mut UDWORD, y: *mut UDWORD, numIterations: UBYTE,
                    function:
                        Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                   -> BOOL>) -> BOOL;
    #[no_mangle]
    fn zonedPAT(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    #[no_mangle]
    static mut numResearch: UDWORD;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /* process the results of a completed research topic */
    #[no_mangle]
    fn researchResult(researchIndex: UDWORD, player: UBYTE, bDisplay: BOOL);
    /*subtract the power required */
    #[no_mangle]
    fn usePower(player: UDWORD, quantity: UDWORD) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /*
 * ScriptFuncs.h
 *
 * All the C functions callable from the script code
 *
 */
    // not used in scripts, but used in code.
    #[no_mangle]
    fn objectInRange(psList: *mut BASE_OBJECT, x: SDWORD, y: SDWORD,
                     range: SDWORD) -> BOOL;
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
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    #[no_mangle]
    static mut CBallFrom: UDWORD;
    #[no_mangle]
    static mut CBallTo: UDWORD;
    // Number of units in the current list.
    #[no_mangle]
    fn getNumDroids(player: UDWORD) -> UDWORD;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn IdToDroid(id: UDWORD, player: UDWORD, psDroid: *mut *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn getPlayerName(player: UDWORD) -> *mut STRING;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
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
pub type CHAR = libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
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
pub type STRUCTURE = _structure;
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
//line build requires two sets of coords
/*
 * FeatureDef.h
 *
 * Structure definitions for features
 *
 */
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
pub type FEATURE = _feature;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
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
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
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
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
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
pub type EFFECT_GROUP = libc::c_uint;
pub const EFFECT_FIREWORK: EFFECT_GROUP = 11;
pub const EFFECT_FIRE: EFFECT_GROUP = 10;
pub const EFFECT_DUST_BALL: EFFECT_GROUP = 9;
pub const EFFECT_SAT_LASER: EFFECT_GROUP = 8;
pub const EFFECT_DESTRUCTION: EFFECT_GROUP = 7;
pub const EFFECT_BLOOD: EFFECT_GROUP = 6;
pub const EFFECT_WAYPOINT: EFFECT_GROUP = 5;
pub const EFFECT_GRAVITON: EFFECT_GROUP = 4;
pub const EFFECT_STRUCTURE: EFFECT_GROUP = 3;
pub const EFFECT_SMOKE: EFFECT_GROUP = 2;
pub const EFFECT_CONSTRUCTION: EFFECT_GROUP = 1;
pub const EFFECT_EXPLOSION: EFFECT_GROUP = 0;
pub type EFFECT_TYPE = libc::c_uint;
pub const FIREWORK_TYPE_LAUNCHER: EFFECT_TYPE = 42;
pub const FIREWORK_TYPE_STARBURST: EFFECT_TYPE = 41;
pub const WAYPOINT_TYPE: EFFECT_TYPE = 40;
pub const SAT_LASER_STANDARD: EFFECT_TYPE = 39;
pub const DESTRUCTION_TYPE_SKYSCRAPER: EFFECT_TYPE = 38;
pub const DESTRUCTION_TYPE_FEATURE: EFFECT_TYPE = 37;
pub const DESTRUCTION_TYPE_WALL_SECTION: EFFECT_TYPE = 36;
pub const DESTRUCTION_TYPE_POWER_STATION: EFFECT_TYPE = 35;
pub const DESTRUCTION_TYPE_STRUCTURE: EFFECT_TYPE = 34;
pub const DESTRUCTION_TYPE_DROID: EFFECT_TYPE = 33;
pub const DUST_TYPE_NORMAL: EFFECT_TYPE = 32;
pub const BLOOD_TYPE_NORMAL: EFFECT_TYPE = 31;
pub const CONSTRUCTION_TYPE_DRIFTING: EFFECT_TYPE = 30;
pub const FIRE_TYPE_SMOKY_BLUE: EFFECT_TYPE = 29;
pub const FIRE_TYPE_SMOKY: EFFECT_TYPE = 28;
pub const FIRE_TYPE_LOCALISED: EFFECT_TYPE = 27;
pub const SMOKE_TYPE_TRAIL: EFFECT_TYPE = 26;
pub const SMOKE_TYPE_STEAM: EFFECT_TYPE = 25;
pub const SMOKE_TYPE_BILLOW: EFFECT_TYPE = 24;
pub const SMOKE_TYPE_DRIFTING_SMALL: EFFECT_TYPE = 23;
pub const SMOKE_TYPE_DRIFTING_HIGH: EFFECT_TYPE = 22;
pub const SMOKE_TYPE_DRIFTING: EFFECT_TYPE = 21;
pub const GRAVITON_TYPE_GIBLET: EFFECT_TYPE = 20;
pub const GRAVITON_TYPE_EMITTING_ST: EFFECT_TYPE = 19;
pub const GRAVITON_TYPE_EMITTING_DR: EFFECT_TYPE = 18;
pub const GRAVITON_TYPE_STANDARD: EFFECT_TYPE = 17;
pub const EXPLOSION_TYPE_SHOCKWAVE: EFFECT_TYPE = 16;
pub const EXPLOSION_TYPE_LAND_LIGHT: EFFECT_TYPE = 15;
pub const EXPLOSION_TYPE_KICKUP: EFFECT_TYPE = 14;
pub const EXPLOSION_TYPE_PLASMA: EFFECT_TYPE = 13;
pub const EXPLOSION_TYPE_FLARE: EFFECT_TYPE = 12;
pub const EXPLOSION_TYPE_DISCOVERY: EFFECT_TYPE = 11;
pub const EXPLOSION_TYPE_TESLA: EFFECT_TYPE = 10;
pub const EXPLOSION_TYPE_LASER: EFFECT_TYPE = 9;
pub const EXPLOSION_TYPE_FLAMETHROWER: EFFECT_TYPE = 8;
pub const EXPLOSION_TYPE_SPECIFIED_FIXME: EFFECT_TYPE = 7;
pub const EXPLOSION_TYPE_SPECIFIED_SOLID: EFFECT_TYPE = 6;
pub const EXPLOSION_TYPE_NOT_FACING: EFFECT_TYPE = 5;
pub const EXPLOSION_TYPE_SPECIFIED: EFFECT_TYPE = 4;
pub const EXPLOSION_TYPE_LARGE: EFFECT_TYPE = 3;
pub const EXPLOSION_TYPE_MEDIUM: EFFECT_TYPE = 2;
pub const EXPLOSION_TYPE_VERY_SMALL: EFFECT_TYPE = 1;
pub const EXPLOSION_TYPE_SMALL: EFFECT_TYPE = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_0 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_0 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_0 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_0 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_0 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_0 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_0 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_0 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_0 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_0 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_0 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_0 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_0 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_0 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_0 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_0 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_0 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_0 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_0 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_0 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_0 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_0 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_0 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_0 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_0 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_0 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_0 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_0 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_0 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_0 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_0 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_0 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_0 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_0 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_0 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_0 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_0 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_0 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_0 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_0 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_0 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_0 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_0 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_0 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_0 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_0 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_0 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_0 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_0 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_0 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_0 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_0 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_0 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_0 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_0 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_0 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_0 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_0 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_0 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_0 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_0 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_0 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_0 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_0 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_0 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_0 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_0 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_0 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_0 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_0 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_0 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_0 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_0 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_0 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_0 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_0 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_0 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_0 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_0 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_0 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_0 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_0 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_0 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_0 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_0 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_0 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_0 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_0 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_0 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_0 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_0 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_0 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_0 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_0 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_0 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_0 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_0 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_0 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_0 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_0 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_0 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_0 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_0 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_0 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_0 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_0 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_0 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_0 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_0 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_0 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_0 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_0 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_0 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_0 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_0 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_0 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_0 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_0 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_0 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_0 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_0 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_0 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_0 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_0 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_0 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_0 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_0 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_0 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_0 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_0 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_0 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_0 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_0 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_0 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_0 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_0 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_0 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_0 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_0 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_0 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_0 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_0 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_0 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_0 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_0 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_0 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_0 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_0 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_0 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_0 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_0 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_0 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_0 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_0 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_0 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_0 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_0 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_0 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_0 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_0 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_0 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_0 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_0 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_0 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_0 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_0 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_0 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_0 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_0 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_0 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_0 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_0 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_0 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_0 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_0 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_0 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_0 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_0 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_0 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_0 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_0 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_0 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_0 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_0 = 160;
pub const ID_GIFT: C2RustUnnamed_0 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_0 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_0 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_0 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_0 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_0 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_0 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_0 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_0 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_0 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_0 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_0 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_0 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_0 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_0 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_0 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_0 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_0 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_0 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_0 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_0 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_0 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_0 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_0 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_0 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_0 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_0 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_0 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_0 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_0 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_0 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_0 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_0 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_0 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_0 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_0 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_0 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_0 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_0 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_0 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_0 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_0 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_0 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_0 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_0 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_0 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_0 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_0 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_0 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_0 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_0 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_0 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_0 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_0 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_0 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_0 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_0 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_0 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_0 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_0 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_0 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_0 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_0 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_0 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_0 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_0 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_0 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_0 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_0 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_0 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_0 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_0 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_0 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_0 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_0 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_0 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_0 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_0 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_0 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_0 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_0 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_0 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_0 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_0 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_0 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_0 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_0 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_0 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_0 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_0 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_0 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_0 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_0 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_0 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_0 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_0 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_0 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_0 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_0 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_0 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_0 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_0 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_0 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_0 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_0 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_0 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_0 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_0 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_0 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_0 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_0 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_0 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_0 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_0 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_0 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_0 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_0 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_0 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_0 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_0 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_0 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_0 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_0 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_0 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_0 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_0 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_0 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_0 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_0 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_0 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_0 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_0 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_0 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_0 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_0 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_0 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_0 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_0 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_0 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_0 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_0 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_0 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_0 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_0 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_0 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_0 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_0 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_0 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_0 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_0 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_0 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_0 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_0 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_0 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_0 = 0;
pub const NO_SOUND: C2RustUnnamed_0 = -1;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
/*
 * MultiGift.c
 * gifts one player can give to another..
 * Also home to Deathmatch hardcoded RULES.
 */
// ////////////////////////////////////////////////////////////////////////////
//#define DMATCH_DROID_LIMIT	25			// max number of droids in a dmatch game (per player).
// how often to check end game conditions
// max score in a frag match.
// max time in a time limit dmatch.
// /////////////////////////////////////////////////////////////////////////////
// prototypes
// /////////////////////////////////////////////////////////////////////////////
// gifts..
// recieve gift
#[no_mangle]
pub unsafe extern "C" fn recvGift(mut pMsg: *mut NETMSG) -> BOOL {
    let mut t: UDWORD = 0; //decode msg
    let mut from: UDWORD = 0;
    let mut to: UDWORD = 0;
    t = (*pMsg).body[0 as libc::c_int as usize] as UDWORD;
    from = (*pMsg).body[1 as libc::c_int as usize] as UDWORD;
    to = (*pMsg).body[2 as libc::c_int as usize] as UDWORD;
    match t {
        1 => { giftRadar(from, to, 0 as libc::c_int); }
        2 => { recvGiftDroids(from, to, pMsg); }
        3 => { giftResearch(from, to, 0 as libc::c_int); }
        4 => { giftPower(from, to, 0 as libc::c_int); }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown Gift recvd\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    // play some audio.
    if to == selectedPlayer {
        audio_QueueTrack(ID_GIFT as libc::c_int);
        match t {
            1 => { audio_QueueTrack(ID_SENSOR_DOWNLOAD as libc::c_int); }
            2 => { audio_QueueTrack(ID_UNITS_TRANSFER as libc::c_int); }
            3 => { audio_QueueTrack(ID_TECHNOLOGY_TRANSFER as libc::c_int); }
            4 => { audio_QueueTrack(ID_POWER_TRANSMIT as libc::c_int); }
            _ => { }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sendGift(mut type_0: UDWORD, mut to: UDWORD)
 -> BOOL {
    match type_0 {
        1 => {
            giftRadar(selectedPlayer, to, 1 as libc::c_int);
            audio_QueueTrack(ID_SENSOR_DOWNLOAD as libc::c_int);
        }
        2 => {
            sendGiftDroids(selectedPlayer, to);
            audio_QueueTrack(ID_UNITS_TRANSFER as libc::c_int);
        }
        3 => {
            giftResearch(selectedPlayer, to, 1 as libc::c_int);
            audio_QueueTrack(ID_TECHNOLOGY_TRANSFER as libc::c_int);
        }
        4 => {
            giftPower(selectedPlayer, to, 1 as libc::c_int);
            audio_QueueTrack(ID_POWER_TRANSMIT as libc::c_int);
        }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown Gift sent\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// give radar information
#[no_mangle]
pub unsafe extern "C" fn giftRadar(mut from: UDWORD, mut to: UDWORD,
                                   mut send: BOOL) {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    hqReward(from as UBYTE, to as UBYTE);
    if send != 0 {
        m.body[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char;
        m.body[1 as libc::c_int as usize] = from as UBYTE as libc::c_char;
        m.body[2 as libc::c_int as usize] = to as UBYTE as libc::c_char;
        m.type_0 = NET_GIFT as libc::c_int as libc::c_uchar;
        m.size = 3 as libc::c_int as libc::c_ushort;
        NETbcast(&mut m, 1 as libc::c_int);
        //send it
    } else if to == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GIFT_VIS as libc::c_int as UDWORD),
                getPlayerName(from));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
//static VOID		giftSingleDroid					(DROID *psD,UDWORD from,UDWORD to);
// ////////////////////////////////////////////////////////////////////////////
// give a droid - MOVED INTO DROID.C - AB 5/11/98
/*static void giftSingleDroid(DROID *psD,UDWORD from,UDWORD to)
{
}*/
unsafe extern "C" fn recvGiftDroids(mut from: UDWORD, mut to: UDWORD,
                                    mut pMsg: *mut NETMSG) {
    let mut id: UDWORD = 0;
    let mut pos: UDWORD = 3 as libc::c_int as UDWORD;
    let mut pD: *mut DROID = 0 as *mut DROID;
    while pos < (*pMsg).size as libc::c_uint {
        memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        pos =
            (pos as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        if IdToDroid(id, from, &mut pD) != 0 {
            // find the droid.
            //giftSingleDroid(pD,from,to);	// give it away.
            giftSingleDroid(pD, to);
        }
    }
    if to == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GIFT_DRO as libc::c_int as UDWORD),
                getPlayerName(from));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
// give selected droid
unsafe extern "C" fn sendGiftDroids(mut from: UDWORD, mut to: UDWORD) {
    let mut next: *mut DROID =
        0 as *mut DROID; // store copy, since droid list may change.
    let mut psD: *mut DROID = apsDroidLists[from as usize];
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut count: UDWORD = 0;
    if apsDroidLists[from as usize].is_null() { return }
    m.body[0 as libc::c_int as usize] = 2 as libc::c_int as libc::c_char;
    m.body[1 as libc::c_int as usize] = from as UBYTE as libc::c_char;
    m.body[2 as libc::c_int as usize] = to as UBYTE as libc::c_char;
    m.type_0 = NET_GIFT as libc::c_int as libc::c_uchar;
    m.size = 3 as libc::c_int as libc::c_ushort;
    count = 0 as libc::c_int as UDWORD;
    loop  {
        next = (*psD).psNext;
        if (*psD).selected != 0 {
            // check if recv has too many droids..
	//		if(! IsPlayerDroidLimitReached(to) )
            if getNumDroids(to).wrapping_add(count) < getMaxDroids(to) {
                giftSingleDroid(psD, to);
                memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                           *mut libc::c_char as *mut libc::c_void,
                       &mut (*psD).id as *mut UDWORD as *const libc::c_void,
                       ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
                m.size =
                    (m.size as
                         libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                        as libc::c_ulong) as
                        libc::c_ushort as libc::c_ushort;
                count = count.wrapping_add(1)
            }
        }
        psD = next;
        if psD.is_null() { break ; }
    }
    if m.size as libc::c_int > 3 as libc::c_int {
        NETbcast(&mut m, 1 as libc::c_int);
        //send it
    };
}
// ////////////////////////////////////////////////////////////////////////////
// give technologies.
unsafe extern "C" fn giftResearch(mut from: UDWORD, mut to: UDWORD,
                                  mut send: BOOL) {
    let mut pR: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut pRto: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut i: UDWORD = 0;
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    pR = asPlayerResList[from as usize];
    pRto = asPlayerResList[to as usize];
    i = 0 as libc::c_int as UDWORD;
    while i < numResearch {
        // do for each topic.
        if (*pR.offset(i as isize)).ResearchStatus as libc::c_int &
               0x4 as libc::c_int != 0 {
            if (*pRto.offset(i as isize)).ResearchStatus as libc::c_int &
                   0x4 as libc::c_int == 0 as libc::c_int {
                (*pRto.offset(i as isize)).ResearchStatus =
                    ((*pRto.offset(i as isize)).ResearchStatus as libc::c_int
                         &
                         !(0x1 as libc::c_int | 0x2 as libc::c_int |
                               0x4 as libc::c_int) | 0x4 as libc::c_int) as
                        UBYTE;
                researchResult(i, to as UBYTE, 0 as libc::c_int);
            }
        }
        i = i.wrapping_add(1)
    }
    /*	pPlayerRes = asPlayerResList[player];
	pPlayerRes += index;
	if(IsResearchCompleted(pPlayerRes)==FALSE)
	{
		MakeResearchCompleted(pPlayerRes);
		rese
*/
    if send != 0 {
        m.body[0 as libc::c_int as usize] = 3 as libc::c_int as libc::c_char;
        m.body[1 as libc::c_int as usize] = from as UBYTE as libc::c_char;
        m.body[2 as libc::c_int as usize] = to as UBYTE as libc::c_char;
        m.type_0 = NET_GIFT as libc::c_int as libc::c_uchar;
        m.size = 3 as libc::c_int as libc::c_ushort;
        NETbcast(&mut m, 1 as libc::c_int);
        //send it
    } else if to == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GIFT_TEC as libc::c_int as UDWORD),
                getPlayerName(from));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
// ////////////////////////////////////////////////////////////////////////////
// give Power
#[no_mangle]
pub unsafe extern "C" fn giftPower(mut from: UDWORD, mut to: UDWORD,
                                   mut send: BOOL) {
    let mut gifval: UDWORD = 0;
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if from == 99 as libc::c_int as libc::c_uint {
        gifval = 100 as libc::c_int as UDWORD
    } else {
        gifval =
            (*asPower[from as
                          usize]).currentPower.wrapping_div(3 as libc::c_int
                                                                as
                                                                libc::c_uint);
        //		asPower[from]->currentPower -= gifval;
        usePower(from, gifval);
    }
    addPower(to, gifval);
    if send != 0 {
        m.body[0 as libc::c_int as usize] = 4 as libc::c_int as libc::c_char;
        m.body[1 as libc::c_int as usize] = from as UBYTE as libc::c_char;
        m.body[2 as libc::c_int as usize] = to as UBYTE as libc::c_char;
        m.type_0 = NET_GIFT as libc::c_int as libc::c_uchar;
        m.size = 3 as libc::c_int as libc::c_ushort;
        NETbcast(&mut m, 1 as libc::c_int);
        //send it
    } else if to == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GIFT_POW as libc::c_int as UDWORD),
                getPlayerName(from));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
/* 
 *multigifts. h
 *
 * multiplayer game, gifts and deathmatch relevant funcs.
 */
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// alliance code......
#[no_mangle]
pub unsafe extern "C" fn requestAlliance(mut from: UBYTE, mut to: UBYTE,
                                         mut prop: BOOL,
                                         mut allowAudio: BOOL) {
    alliances[from as usize][to as usize] =
        1 as libc::c_int as UBYTE; // we've asked.
    alliances[to as usize][from as usize] =
        2 as libc::c_int as UBYTE; // they've been invited.
    CBallFrom = from as UDWORD;
    CBallTo = to as UDWORD;
    eventFireCallbackTrigger(CALL_ALLIANCEOFFER as libc::c_int as
                                 TRIGGER_TYPE);
    if to as libc::c_uint == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_ALLI_REQ as libc::c_int as UDWORD),
                getPlayerName(from as UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        if allowAudio != 0 {
            audio_QueueTrack(ID_ALLIANCE_OFF as libc::c_int);
        }
    }
    if from as libc::c_uint == selectedPlayer {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_ALLI_OFF as libc::c_int as UDWORD),
                getPlayerName(to as UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        if allowAudio != 0 {
            audio_QueueTrack(ID_ALLIANCE_OFF as libc::c_int);
        }
    }
    if prop != 0 {
        sendAlliance(from, to, 1 as libc::c_int as UBYTE, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn breakAlliance(mut p1: UBYTE, mut p2: UBYTE,
                                       mut prop: BOOL, mut allowAudio: BOOL) {
    let mut tm1: [CHAR; 128] = [0; 128];
    if alliances[p1 as usize][p2 as usize] as libc::c_int == 3 as libc::c_int
       {
        strcpy(tm1.as_mut_ptr(), getPlayerName(p1 as UDWORD));
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_ALLI_BRK as libc::c_int as UDWORD),
                tm1.as_mut_ptr(), getPlayerName(p2 as UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        if allowAudio != 0 &&
               (p1 as libc::c_uint == selectedPlayer ||
                    p2 as libc::c_uint == selectedPlayer) {
            audio_QueueTrack(ID_ALLIANCE_BRO as libc::c_int);
        }
    }
    alliances[p1 as usize][p2 as usize] = 0 as libc::c_int as UBYTE;
    alliances[p2 as usize][p1 as usize] = 0 as libc::c_int as UBYTE;
    if prop != 0 {
        sendAlliance(p1, p2, 0 as libc::c_int as UBYTE, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn formAlliance(mut p1: UBYTE, mut p2: UBYTE,
                                      mut prop: BOOL, mut allowAudio: BOOL) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut tm1: [CHAR; 128] = [0; 128];
    let mut i: UBYTE = 0;
    // dont add message if already allied,
    if bMultiPlayer != 0 &&
           !(alliances[p1 as usize][p2 as usize] as libc::c_int ==
                 3 as libc::c_int) {
        strcpy(tm1.as_mut_ptr(), getPlayerName(p1 as UDWORD));
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_ALLI_FRM as libc::c_int as UDWORD),
                tm1.as_mut_ptr(), getPlayerName(p2 as UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    }
    alliances[p1 as usize][p2 as usize] = 3 as libc::c_int as UBYTE;
    alliances[p2 as usize][p1 as usize] = 3 as libc::c_int as UBYTE;
    if allowAudio != 0 &&
           (p1 as libc::c_uint == selectedPlayer ||
                p2 as libc::c_uint == selectedPlayer) {
        audio_QueueTrack(ID_ALLIANCE_ACC as libc::c_int);
    }
    if bMultiPlayer != 0 {
        //jps 15apr99
        if prop != 0 {
            sendAlliance(p1, p2, 3 as libc::c_int as UBYTE, 0 as libc::c_int);
        }
    }
    // teamplay init others vis.
    if bMultiPlayer != 0 && game.type_0 as libc::c_int == 13 as libc::c_int &&
           (p1 as libc::c_uint == selectedPlayer ||
                p2 as libc::c_uint == selectedPlayer) {
        giftRadar(p1 as UDWORD, p2 as UDWORD, 0 as libc::c_int);
        giftRadar(p2 as UDWORD, p1 as UDWORD, 0 as libc::c_int);
        // THIS BIT ADDED BY AJL 28th april
        i = 0 as libc::c_int as UBYTE;
        while (i as libc::c_int) < 8 as libc::c_int {
            //for each alliance with the selectedPlayer make a new one with the newly allied player
            if p2 as libc::c_uint == selectedPlayer {
                if p1 as libc::c_int != i as libc::c_int &&
                       p2 as libc::c_int != i as libc::c_int &&
                       aiCheckAlliances(p1 as UDWORD, i as UDWORD) != 0 {
                    if aiCheckAlliances(p2 as UDWORD, i as UDWORD) == 0 {
                        formAlliance(p2, i, 1 as libc::c_int,
                                     0 as libc::c_int);
                    }
                }
            } else if p1 as libc::c_int != i as libc::c_int &&
                          p2 as libc::c_int != i as libc::c_int &&
                          aiCheckAlliances(p2 as UDWORD, i as UDWORD) != 0 {
                if aiCheckAlliances(p1 as UDWORD, i as UDWORD) == 0 {
                    formAlliance(p1, i, 1 as libc::c_int, 0 as libc::c_int);
                }
            }
            i = i.wrapping_add(1)
        }
    }
    // clear out any attacking orders.
    turnOffMultiMsg(1 as libc::c_int);
    psDroid = apsDroidLists[p1 as usize];
    while !psDroid.is_null() {
        // from -> to
        if (*psDroid).order == DORDER_ATTACK as libc::c_int &&
               !(*psDroid).psTarget.is_null() &&
               (*(*psDroid).psTarget).player as libc::c_int ==
                   p2 as libc::c_int {
            orderDroid(psDroid, DORDER_STOP);
        }
        psDroid = (*psDroid).psNext
    }
    psDroid = apsDroidLists[p2 as usize];
    while !psDroid.is_null() {
        // to -> from
        if (*psDroid).order == DORDER_ATTACK as libc::c_int &&
               !(*psDroid).psTarget.is_null() &&
               (*(*psDroid).psTarget).player as libc::c_int ==
                   p1 as libc::c_int {
            orderDroid(psDroid, DORDER_STOP);
        }
        psDroid = (*psDroid).psNext
    }
    turnOffMultiMsg(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sendAlliance(mut from: UBYTE, mut to: UBYTE,
                                      mut state: UBYTE, mut value: SDWORD) {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    m.size = 0 as libc::c_int as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut from as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut to as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut state as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut value as *mut SDWORD as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    m.type_0 = NET_ALLIANCE as libc::c_int as libc::c_uchar;
    NETbcast(&mut m, 1 as libc::c_int);
    /*
	// teamplay init others vis.
	if(bMultiPlayer && state == ALLIANCE_FORMED && game.type == TEAMPLAY && (from == selectedPlayer || to== selectedPlayer))
	{
		giftRadar(from,to,FALSE);
		giftRadar(to,from,FALSE);
	}
*/
}
#[no_mangle]
pub unsafe extern "C" fn recvAlliance(mut pMsg: *mut NETMSG,
                                      mut allowAudio: BOOL) -> BOOL {
    let mut to: UBYTE = 0;
    let mut from: UBYTE = 0;
    let mut state: UBYTE = 0;
    let mut pos: UBYTE = 0 as libc::c_int as UBYTE;
    let mut value: SDWORD = 0;
    memcpy(&mut from as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UBYTE as UBYTE;
    memcpy(&mut to as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UBYTE as UBYTE;
    memcpy(&mut state as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UBYTE as UBYTE;
    memcpy(&mut value as *mut SDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    match state as libc::c_int {
        4 => { }
        1 => { requestAlliance(from, to, 0 as libc::c_int, allowAudio); }
        3 => { formAlliance(from, to, 0 as libc::c_int, allowAudio); }
        0 => { breakAlliance(from, to, 0 as libc::c_int, allowAudio); }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown alliance state recvd.\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// add an artifact on destruction if required.
#[no_mangle]
pub unsafe extern "C" fn technologyGiveAway(mut pS: *mut STRUCTURE) {
    let mut i: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut nx: UWORD = 0;
    let mut ny: UWORD = 0;
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    let mut type_0: SDWORD = FEAT_GEN_ARTE as libc::c_int;
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if (*(*pS).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint &&
           myResponsibility((*pS).player as UDWORD) != 0 {
        x = ((*pS).x as libc::c_int >> 7 as libc::c_int) as UDWORD;
        y = ((*pS).y as libc::c_int >> 7 as libc::c_int) as UDWORD;
        if pickATileGen(&mut x, &mut y, 20 as libc::c_int as UBYTE,
                        Some(zonedPAT as
                                 unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                     -> BOOL)) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"technologyGiveAway: Unable to find a free location\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"multigifts.c\x00" as *const u8 as *const libc::c_char,
                      598 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"technologyGiveAway\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        i = 0 as libc::c_int as UDWORD;
        while i < numFeatureStats &&
                  (*asFeatureStats.offset(i as isize)).subType as libc::c_uint
                      != FEAT_GEN_ARTE as libc::c_int as libc::c_uint {
            i = i.wrapping_add(1)
        }
        pF =
            buildFeature(asFeatureStats.offset(i as isize),
                         x << 7 as libc::c_int, y << 7 as libc::c_int,
                         0 as libc::c_int);
        if !pF.is_null() { (*pF).player = (*pS).player }
        // tell everyone.
        m.body[0 as libc::c_int as usize] =
            1 as libc::c_int as UBYTE as libc::c_char; // note how many
        m.size = 1 as libc::c_int as libc::c_ushort;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut type_0 as *mut SDWORD as *const libc::c_void,
               ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        nx = x as UWORD;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut nx as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        ny = y as UWORD;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut ny as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pF).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        m.body[m.size as usize] = (*pS).player as libc::c_char;
        m.size = (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
        m.type_0 = NET_ARTIFACTS as libc::c_int as libc::c_uchar;
        NETbcast(&mut m, 0 as libc::c_int);
    };
}
// type.
// send it.
//BOOL			addDMatchDroid					(UDWORD count);
//BOOL			foundDMatchDroid				(UDWORD player,UDWORD x,UDWORD y);
//BOOL			deathmatchCheck					(VOID);
//static BOOL		dMatchWinner					(UDWORD winplayer,BOOL bcast);
//BOOL			recvdMatchWinner				(NETMSG *pMsg);
// every 5 mins tops..
#[no_mangle]
pub unsafe extern "C" fn addLoserGifts() {
    //	DROID			*psD;
//	DROID_TEMPLATE	*psTempl;
//	iVector			position;
    static mut lastgift: UDWORD =
        0 as libc::c_int as UDWORD; // might be a restart
    let mut i: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut quantity: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut nx: UWORD = 0;
    let mut ny: UWORD = 0;
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    let mut type_0: SDWORD = FEAT_OIL_DRUM as libc::c_int;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if lastgift > gameTime { lastgift = 0 as libc::c_int as UDWORD }
    // player has no power
    if !apsStructLists[selectedPlayer as usize].is_null() &&
           (*asPower[selectedPlayer as usize]).currentPower <
               10 as libc::c_int as libc::c_uint {
        // give some oil drums.
        // only proceed if it's been a while
        if gameTime.wrapping_sub(lastgift) <
               (1000 as libc::c_int * (60 as libc::c_int * 5 as libc::c_int))
                   as libc::c_uint {
            return
        }
        psStruct = apsStructLists[selectedPlayer as usize];
        while !psStruct.is_null() &&
                  (*(*psStruct).pStructureType).type_0 !=
                      REF_POWER_GEN as libc::c_int as libc::c_uint {
            psStruct = (*psStruct).psNext
        }
        if !psStruct.is_null() { return }
        lastgift = gameTime;
        i = 0 as libc::c_int as UDWORD;
        while i < numFeatureStats &&
                  (*asFeatureStats.offset(i as isize)).subType as libc::c_uint
                      != FEAT_OIL_DRUM as libc::c_int as libc::c_uint {
            i = i.wrapping_add(1)
        }
        quantity = (rand() % 5 as libc::c_int + 1 as libc::c_int) as UDWORD;
        m.size = 1 as libc::c_int as libc::c_ushort;
        m.body[0 as libc::c_int as usize] = quantity as UBYTE as libc::c_char;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut type_0 as *mut SDWORD as *const libc::c_void,
               ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        count = 0 as libc::c_int as UDWORD;
        while count < quantity {
            x =
                ((*apsStructLists[selectedPlayer as usize]).x as libc::c_int
                     >> 7 as libc::c_int) as UDWORD;
            y =
                ((*apsStructLists[selectedPlayer as usize]).y as libc::c_int
                     >> 7 as libc::c_int) as UDWORD;
            if pickATileGen(&mut x, &mut y, 20 as libc::c_int as UBYTE,
                            Some(zonedPAT as
                                     unsafe extern "C" fn(_: UDWORD,
                                                          _: UDWORD) -> BOOL))
                   == 0 {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"addlosergifts: Unable to find a free location\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"multigifts.c\x00" as *const u8 as
                              *const libc::c_char, 694 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"addLoserGifts\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            NETlogEntry(b"gift\x00" as *const u8 as *const libc::c_char as
                            *mut CHAR, 0 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD);
            pF =
                buildFeature(asFeatureStats.offset(i as isize),
                             x << 7 as libc::c_int, y << 7 as libc::c_int,
                             0 as libc::c_int);
            nx = x as UWORD;
            ny = y as UWORD;
            memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut x as *mut UDWORD as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            m.size =
                (m.size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>()
                                                    as libc::c_ulong) as
                    libc::c_ushort as libc::c_ushort;
            memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut x as *mut UDWORD as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            m.size =
                (m.size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>()
                                                    as libc::c_ulong) as
                    libc::c_ushort as libc::c_ushort;
            memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut (*pF).id as *mut UDWORD as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            m.size =
                (m.size as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                    as libc::c_ulong) as
                    libc::c_ushort as libc::c_ushort;
            m.body[m.size as usize] = 98 as libc::c_int as libc::c_char;
            m.size =
                (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
            if !pF.is_null() {
                (*pF).player = 98 as libc::c_int as UBYTE
                // only proceed if no powergen.
                // note how many
                // flag for multiplayer artifacts
            }
            count = count.wrapping_add(1)
        }
        audio_QueueTrack(ID_GIFT as libc::c_int);
        m.type_0 = NET_ARTIFACTS as libc::c_int as libc::c_uchar;
        NETbcast(&mut m, 0 as libc::c_int);
    };
    /* removed. too confusing.. con droids all over!
	// player has no construction droids
	for(psD=apsDroidLists[selectedPlayer];(psD != NULL)&&(psD->droidType !=DROID_CONSTRUCT);psD = psD->psNext);
	if(!psD)
	{
		for(psTempl=apsDroidTemplates[selectedPlayer];
		psTempl && (psTempl->asParts[COMP_CONSTRUCT] == 0);
		psTempl = psTempl->psNext);

		if(psTempl)
		{
			// give player a construction Droid.right now!
			if(apsStructLists[selectedPlayer])
			{
				x = apsStructLists[selectedPlayer]->x >>TILE_SHIFT;
				y = apsStructLists[selectedPlayer]->y >>TILE_SHIFT;
				z = apsStructLists[selectedPlayer]->z >>TILE_SHIFT;

				pickATileGen(&x,&y,LOOK_FOR_EMPTY_TILE,normalPAT);

				position.x = x<<TILE_SHIFT;				// Add an effect
				position.z = y<<TILE_SHIFT;
				position.y = z<<TILE_SHIFT;

				if(gameTime - lastgift< GIFTFREQ)
				{
					return;
				}
				lastgift = gameTime;
				powerCalc(FALSE);
				psD=buildDroid(	psTempl, x<<TILE_SHIFT,  y<<TILE_SHIFT, selectedPlayer, FALSE);
				if(psD)
				{
					audio_QueueTrack(ID_GIFT);
					addDroid(psD,apsDroidLists);							// add droid. telling everyone
					addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_DISCOVERY,FALSE,NULL,FALSE);
				}
				powerCalc(TRUE);											// power back on.
			}
		}
	}
	*/
}
// /////////////////////////////////////////////////////////////////////////////
// splatter artifact gifts randomly about.
#[no_mangle]
pub unsafe extern "C" fn addMultiPlayerRandomArtifacts(mut quantity: UDWORD,
                                                       mut type_0: SDWORD) {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // note how many
    let mut pF: *mut FEATURE =
        0 as *mut FEATURE; //( between 10 and mapwidth-10)
    let mut i: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut nx: UWORD = 0;
    let mut ny: UWORD = 0;
    m.type_0 = NET_ARTIFACTS as libc::c_int as libc::c_uchar;
    m.size = 1 as libc::c_int as libc::c_ushort;
    m.body[0 as libc::c_int as usize] = quantity as UBYTE as libc::c_char;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut type_0 as *mut SDWORD as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    i = 0 as libc::c_int as UDWORD;
    while i < numFeatureStats &&
              (*asFeatureStats.offset(i as isize)).subType as libc::c_uint !=
                  type_0 as libc::c_uint {
        i = i.wrapping_add(1)
    }
    if mapWidth > 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"map not big enough\x00" as *const u8 as *const libc::c_char);
    };
    if mapWidth > 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"multigifts.c\x00" as *const u8 as *const libc::c_char,
              784 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 30],
                                        &[libc::c_char; 30]>(b"addMultiPlayerRandomArtifacts\x00")).as_ptr(),
              b"mapWidth>20\x00" as *const u8 as *const libc::c_char);
    };
    if mapHeight > 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"map not big enough\x00" as *const u8 as *const libc::c_char);
    };
    if mapHeight > 20 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"multigifts.c\x00" as *const u8 as *const libc::c_char,
              785 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 30],
                                        &[libc::c_char; 30]>(b"addMultiPlayerRandomArtifacts\x00")).as_ptr(),
              b"mapHeight>20\x00" as *const u8 as *const libc::c_char);
    };
    count = 0 as libc::c_int as UDWORD;
    while count < quantity {
        x =
            (rand() as
                 libc::c_uint).wrapping_rem(mapWidth.wrapping_sub(20 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)).wrapping_add(10
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint);
        y =
            (rand() as
                 libc::c_uint).wrapping_rem(mapHeight.wrapping_sub(20 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)).wrapping_add(10
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint);
        if pickATileGen(&mut x, &mut y, 20 as libc::c_int as UBYTE,
                        Some(zonedPAT as
                                 unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                     -> BOOL)) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"addMultiPlayerRandomArtifacts: Unable to find a free location\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"multigifts.c\x00" as *const u8 as *const libc::c_char,
                      793 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 30],
                                                &[libc::c_char; 30]>(b"addMultiPlayerRandomArtifacts\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        pF =
            buildFeature(asFeatureStats.offset(i as isize),
                         x << 7 as libc::c_int, y << 7 as libc::c_int,
                         0 as libc::c_int);
        nx = x as UWORD;
        ny = y as UWORD;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut nx as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut ny as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pF).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as
                libc::c_ushort as libc::c_ushort;
        m.body[m.size as usize] = 99 as libc::c_int as libc::c_char;
        m.size = (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
        if !pF.is_null() {
            (*pF).player = 99 as libc::c_int as UBYTE
            // flag for multiplayer artifacts
        }
        count = count.wrapping_add(1)
    }
    NETbcast(&mut m, 0 as libc::c_int);
    // tell everyone.
}
// ///////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addDMatchDroid(mut count: UDWORD) -> BOOL {
    addMultiPlayerRandomArtifacts(count, FEAT_GEN_ARTE as libc::c_int);
    return 1 as libc::c_int;
}
// deathmatch stuff
//DROID_TEMPLATE *pickDistribTempl				(UDWORD player);
// ///////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn addOilDrum(mut count: UDWORD) -> BOOL {
    addMultiPlayerRandomArtifacts(count, FEAT_OIL_DRUM as libc::c_int);
    return 1 as libc::c_int;
}
// ///////////////////////////////////////////////////////////////
// receive splattered artifacts
#[no_mangle]
pub unsafe extern "C" fn recvMultiPlayerRandomArtifacts(mut pMsg:
                                                            *mut NETMSG) {
    let mut index: UDWORD = 0; // flag for multiplayer artifacts
    let mut count: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut quantity: UDWORD = 0;
    let mut ref_0: UDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    let mut tx: UWORD = 0;
    let mut ty: UWORD = 0;
    quantity = (*pMsg).body[0 as libc::c_int as usize] as UDWORD;
    index = 1 as libc::c_int as UDWORD;
    memcpy(&mut type_0 as *mut SDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    index =
        (index as
             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < numFeatureStats &&
              (*asFeatureStats.offset(i as isize)).subType as libc::c_uint !=
                  type_0 as libc::c_uint {
        i = i.wrapping_add(1)
    }
    count = 0 as libc::c_int as UDWORD;
    while count < quantity {
        memcpy(&mut tx as *mut UWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        index =
            (index as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut ty as *mut UWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        index =
            (index as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        memcpy(&mut ref_0 as *mut UDWORD as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        index =
            (index as
                 libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                                libc::c_ulong) as UDWORD as
                UDWORD;
        pF =
            buildFeature(asFeatureStats.offset(i as isize),
                         ((tx as libc::c_int) << 7 as libc::c_int) as UDWORD,
                         ((ty as libc::c_int) << 7 as libc::c_int) as UDWORD,
                         0 as libc::c_int);
        if !pF.is_null() {
            (*pF).id = ref_0;
            (*pF).player = (*pMsg).body[index as usize] as UBYTE;
            index =
                (index as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        count = count.wrapping_add(1)
    };
}
// ///////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn giftArtifact(mut owner: UDWORD, mut x: UDWORD,
                                      mut y: UDWORD) {
    let mut pO: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut pR: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut topic: UDWORD = 0 as libc::c_int as UDWORD;
    pR = asPlayerResList[selectedPlayer as usize];
    if !(owner == 99 as libc::c_int as libc::c_uint) {
        if owner >= 8 as libc::c_int as libc::c_uint {
            //1.04 bodge to stop savegame crash
            return
        } else {
            // steal a research topic from owner player.
            pO = asPlayerResList[owner as usize];
            topic = numResearch;
            while topic > 0 as libc::c_int as libc::c_uint {
                if (*pO.offset(topic as isize)).ResearchStatus as libc::c_int
                       & 0x4 as libc::c_int != 0 &&
                       (*pR.offset(topic as isize)).ResearchStatus as
                           libc::c_int & 0x80 as libc::c_int ==
                           0 as libc::c_int {
                    let ref mut fresh0 =
                        (*pR.offset(topic as isize)).ResearchStatus;
                    *fresh0 =
                        (*fresh0 as libc::c_int | 0x80 as libc::c_int) as
                            UBYTE;
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_MUL_ARTIF as libc::c_int as
                                                UDWORD),
                            getName((*asResearch.offset(topic as
                                                            isize)).pName));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    return
                }
                topic = topic.wrapping_sub(1)
            }
        }
    };
}
// ///////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn processMultiPlayerArtifacts() {
    static mut lastCall: UDWORD = 0;
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    let mut pFN: *mut FEATURE = 0 as *mut FEATURE;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut pl: UDWORD = 0;
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut found: BOOL = 0 as libc::c_int;
    // only do this every now and again.
    if lastCall > gameTime { lastCall = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastCall) < 2000 as libc::c_int as libc::c_uint {
        return
    }
    lastCall = gameTime;
    addLoserGifts();
    pF = apsFeatureLists[0 as libc::c_int as usize];
    while !pF.is_null() {
        pFN = (*pF).psNext;
        // oil drums
//		if(pF->psStats->subType == FEAT_OIL_DRUM)
//		{
//			found = objectInRange((BASE_OBJECT *)apsDroidLists[selectedPlayer], pF->x,pF->y,(TILE_UNITS+(TILE_UNITS/3))  );
//			if(found)
//			{
//				giftPower(ANYPLAYER,selectedPlayer,TRUE);		// give power and tell everyone.
//				removeFeature(pF);								// remove artifact+ send info.
//				addOilDrum(1);
//			}
//		}
        if (*(*pF).psStats).subType as libc::c_uint ==
               FEAT_GEN_ARTE as libc::c_int as libc::c_uint {
            found =
                objectInRange(apsDroidLists[selectedPlayer as usize] as
                                  *mut BASE_OBJECT, (*pF).x as SDWORD,
                              (*pF).y as SDWORD,
                              128 as libc::c_int +
                                  128 as libc::c_int / 3 as libc::c_int);
            if found != 0 {
                // artifacts
                position.x = (*pF).x as int32;
                position.z = (*pF).y as int32;
                position.y = (*pF).z as int32;
                addEffect(&mut position, EFFECT_EXPLOSION,
                          EXPLOSION_TYPE_DISCOVERY, 0 as libc::c_int,
                          0 as *mut iIMDShape,
                          0 as libc::c_int); // Add an effect
                //				}
                x = (*pF).x as UDWORD;
                y = (*pF).y as UDWORD;
                pl = (*pF).player as UDWORD;
                removeFeature(pF);
                giftArtifact(pl, x, y);
                (*pF).player = 0 as libc::c_int as UBYTE;
                audio_QueueTrack(ID_SOUND_ARTIFACT_RECOVERED as libc::c_int);
            }
        }
        pF = pFN
    };
}
//				if(game.type == DMATCH)
//				{
//					x = pF->x;
//					y = pF->y;
//					removeFeature(pF);			// remove artifact+ send info.
//					foundDMatchDroid(selectedPlayer, x,y);
//					addDMatchDroid(1);
//				}
//				else
//				{
// remove artifact+ send info.
// reward player.
/*
// ///////////////////////////////////////////////////////////////
// check for respawn or win situations.
BOOL deathmatchCheck(VOID)
{
	UDWORD			pl,maxScP;
	SDWORD			maxSc;
	static BOOL		gameComplete=FALSE;
	static UDWORD	lastCheck=0;
	CHAR			sTemp[256];

	// respawn check
	if(apsDroidLists[selectedPlayer] == NULL)
	{
		setPower(selectedPlayer,LEV_HI);		// reset power.
		sendPowerCheck(TRUE);					// tell everyone.

		strcpy(sTemp, "multiplay/forces/");
		strcat(sTemp, sForceName);
		strcat(sTemp,".for");
		loadForce( sTemp);

		useTheForce(FALSE);

		cameraToHome(selectedPlayer,FALSE);			// move camera.
	}


	// go no further if recently called.
	if(lastCheck > gameTime)lastCheck = 0;
	if(gameTime-lastCheck < ENDFREQUENCY)
	{
		return TRUE;
	}
	lastCheck = gameTime;

	// end game conditions.
	// fraglimit / timelimit.
	if(NetPlay.bHost && !gameComplete)
	{
		switch(game.limit)
		{
		case FRAGLIMIT:		// check for fraglimit => FRAGLIMIT
			for(pl=0; pl<MAX_PLAYERS;pl++)
			{
				if(isHumanPlayer(pl) && getMultiStats(pl,TRUE).recentScore >= MAXFRAGS)
				{
					gameComplete = TRUE;
					dMatchWinner(pl,TRUE);
					break;
				}
			}
			break;

		case TIMELIMIT:		// check for timelimit exceeded.
			if(gameTime-ingame.startTime > MAXTIME)
			{
				// pick a winner.
				maxScP = 0;
				maxSc  = 0;
				for(pl=0; pl<MAX_PLAYERS;pl++)
				{
					if(isHumanPlayer(pl) && getMultiStats(pl,FALSE).recentScore  > maxSc)
					{
						maxSc  = getMultiStats(pl,FALSE).recentScore;
						maxScP = pl;
					}
				}
				gameComplete == TRUE;
				dMatchWinner(pl,TRUE);
			}
			break;

		default:
			break;
		}


	}

	return TRUE;
}

// ///////////////////////////////////////////////////////////////
// Winner recieved.

BOOL recvdMatchWinner(NETMSG *pMsg)
{
	return dMatchWinner( (UDWORD)pMsg->body[0] ,FALSE);
}

// ///////////////////////////////////////////////////////////////
// Winner.
static BOOL dMatchWinner(UDWORD winplayer,BOOL bcast)
{
	NETMSG m;

	if(bcast)
	{
		m.type = NET_DMATCHWIN;		// send a netmsg.
		m.size = 1;
		m.body[0] = (UBYTE)winplayer;
		NETbcast(m,TRUE);
	}

	// declare player winner/looser.
	if(winplayer == selectedPlayer)
	{
		displayGameOver(TRUE);		// winner.
	}
	else
	{
		displayGameOver(FALSE);		// looooseeer.
	}

	// throw up the multimenu
	if(widgGetFromID(psWScreen,MULTIMENU_FORM) == NULL)
	{
		intAddMultiMenu();
	}

	// add some effects??

	// fire up a new level????????

	return TRUE;
}

*/
/*
// ///////////////////////////////////////////////////////////////
// deathmatch Stuff find/add droids using artifacts.

// pick a template approximating the normal distribution around powerpoints.
DROID_TEMPLATE * pickDistribTempl(UDWORD player)
{
	DROID_TEMPLATE *psTempl;
	UDWORD	i,rn,rt,dist,min=UDWORD_MAX,max=0,av=0,num=0;

	// gather data
	for(psTempl=apsDroidTemplates[player];psTempl;psTempl=psTempl->psNext)
	{
		num++;
		av += psTempl->powerPoints;
		if(psTempl->powerPoints < min) min = psTempl->powerPoints;
		if(psTempl->powerPoints > max) max = psTempl->powerPoints;
	}

	av = av / num; 		// get average build pts.
	if(av-min > max-av)	// get furthest from average.
	{
		max = min;
	}

	rn = (rand()% abs(max-av) )+ av;	// pick a test level. av - max.
	rn = abs(rn - av);					// rn is now a random distance from av.

	dist = 0;					//init distn

	while(dist <= rn)
	{
		//pick a template
		rt = (rand()%num);	// template to pick
		i =0;
		for(psTempl=apsDroidTemplates[player];i<rt;psTempl=psTempl->psNext)
		{
			i++;
		}

		// distance of this template
		dist = abs(psTempl->powerPoints - av);
	}
	return psTempl;

}
// ///////////////////////////////////////////////////////////////
// player runs into a dmatch artifact
BOOL foundDMatchDroid(UDWORD player,UDWORD x,UDWORD y)
{
	DROID_TEMPLATE *psTempl;
	DROID	*psD;
	UDWORD	count=0;

	// check to see if player has too many droids.
	for(psD = apsDroidLists[player];psD; psD=psD->psNext)
	{
		count++;
	}
	if(count >= DMATCH_DROID_LIMIT)
	{
		return TRUE;
	}

	psTempl = pickDistribTempl(player);	//pick a droid

	// no longer need to turn off power, but just in case.
	powerCalc(FALSE);					//turn off power temporarily.

	psD=buildDroid(	psTempl,  x,  y,  player, FALSE);
	if(psD)
	{
		addDroid(psD,apsDroidLists);	// add droid. telling everyone
	}
	powerCalc(TRUE);					// power back on.

	return TRUE;
}
*/
