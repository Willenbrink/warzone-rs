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
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    /* Return the absolute value of X.  */
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    /* Return the square root of X.  */
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    /* Lookup trig functions */
    #[no_mangle]
    fn trigSin(angle: SDWORD) -> FRACT;
    #[no_mangle]
    fn trigCos(angle: SDWORD) -> FRACT;
    /* Supposedly fast lookup sqrt - unfortunately it's probably slower than the FPU sqrt :-( */
    #[no_mangle]
    fn trigIntSqrt(val: UDWORD) -> FRACT;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn audio_PlayStaticTrack(iX: SDWORD, iY: SDWORD, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjStaticTrack(psObj: *mut libc::c_void, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjDynamicTrack(psObj: *mut libc::c_void,
                                 iTrack: libc::c_int,
                                 pUserCallback: AUDIO_CALLBACK) -> BOOL;
    /* Do damage to a droid */
    #[no_mangle]
    fn droidDamage(psDroid: *mut DROID, damage: UDWORD, weaponClass: UDWORD,
                   weaponSubClass: UDWORD) -> BOOL;
    /* calculate muzzle tip location in 3d world */
    #[no_mangle]
    fn calcDroidMuzzleLocation(psDroid: *mut DROID, muzzle: *mut iVector)
     -> BOOL;
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    /*checks if the droid is a VTOL droid and updates the attack runs as required*/
    #[no_mangle]
    fn updateVtolAttackRun(psDroid: *mut DROID);
    #[no_mangle]
    static mut asStructStrengthModifier: [[STRUCTSTRENGTH_MODIFIER; 4]; 6];
    #[no_mangle]
    fn structureDamage(psStructure: *mut STRUCTURE, damage: UDWORD,
                       weaponClass: UDWORD, weaponSubClass: UDWORD) -> BOOL;
    #[no_mangle]
    fn calcStructureMuzzleLocation(psStructure: *mut STRUCTURE,
                                   muzzle: *mut iVector) -> BOOL;
    #[no_mangle]
    fn electronicDamage(psTarget: *mut BASE_OBJECT, damage: UDWORD,
                        attackPlayer: UBYTE) -> BOOL;
    #[no_mangle]
    fn featureDamage(psFeature: *mut FEATURE, damage: UDWORD,
                     weaponClass: UDWORD, weaponSubClass: UDWORD) -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asWeaponModifier: [[WEAPON_MODIFIER; 9]; 6];
    #[no_mangle]
    fn weaponDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponRadDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponIncenDamage(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponRadiusHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn ptInStructure(psStruct: *mut STRUCTURE, x: UDWORD, y: UDWORD) -> BOOL;
    /* Give a droid an action */
    #[no_mangle]
    fn actionDroid(psDroid: *mut DROID, action: DROID_ACTION);
    /*checks through the target players list of structures and droids to see
if any support a counter battery sensor*/
    #[no_mangle]
    fn counterBatteryFire(psAttacker: *mut BASE_OBJECT,
                          psTarget: *mut BASE_OBJECT);
    /* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
    #[no_mangle]
    fn effectGiveAuxVar(var: UDWORD);
    // naughty
    #[no_mangle]
    fn effectGiveAuxVarSec(var: UDWORD);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn addMultiEffect(basePos: *mut iVector, scatter: *mut iVector,
                      group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                      specified: BOOL, imd: *mut iIMDShape, number: UDWORD,
                      lit: BOOL, size: UDWORD);
    /* Allows us to do if(TRI_FLIPPED(psTile)) */
    /* Flips the triangle partition on a tile pointer */
    /* Can player number p see tile t? */
    /* Set a tile to be visible for a player */
    /*#ifndef PSX
#define SET_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits | (1<<p))
#define CLEAR_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits & (~(1<<p))) // check logic
// Is there a door here for the player?
#define TEST_TILE_DOOR(p,t) ( (t->tileDoorBits) & (1<<p) )
#endif*/
/* Arbitrary maximum number of terrain textures - used in look up table for terrain type */
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    /* returns TRUE if object is above ground */
    #[no_mangle]
    fn mapObjIsAboveGround(psObj: *mut BASE_OBJECT) -> BOOL;
    /* **************************************************************************/
/* functions
 */
    #[no_mangle]
    fn hashTable_Create(ppsTable: *mut *mut HASHTABLE, udwTableSize: UDWORD,
                        udwInitElements: UDWORD, udwExtElements: UDWORD,
                        udwElementSize: UDWORD) -> BOOL;
    #[no_mangle]
    fn hashTable_Destroy(psTable: *mut HASHTABLE);
    #[no_mangle]
    fn hashTable_Clear(psTable: *mut HASHTABLE);
    #[no_mangle]
    fn hashTable_GetElement(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_InsertElement(psTable: *mut HASHTABLE,
                               psElement: *mut libc::c_void,
                               iKey1: libc::c_int, iKey2: libc::c_int);
    #[no_mangle]
    fn hashTable_RemoveElement(psTable: *mut HASHTABLE,
                               psElement: *mut libc::c_void,
                               iKey1: libc::c_int, iKey2: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn hashTable_GetFirst(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    #[no_mangle]
    fn hashTable_GetNext(psTable: *mut HASHTABLE) -> *mut libc::c_void;
    /* **************************************************************************/
/*
 * Projectile types and function headers
 *
 * Gareth Jones 11/7/97
 */
/* **************************************************************************/
    /* **************************************************************************/
    /* **************************************************************************/
    // the last unit that did damage - used by script functions
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    // return the current target designator for a player
    #[no_mangle]
    fn cmdDroidGetDesignator(player: UDWORD) -> *mut DROID;
    // update the kills of a command droid if psKiller is in a command group
    #[no_mangle]
    fn cmdDroidUpdateKills(psKiller: *mut DROID);
    #[no_mangle]
    fn gamePaused() -> BOOL;
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    #[no_mangle]
    fn scoreUpdateVar(var: DATA_INDEX);
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    fn shakeStart();
    #[no_mangle]
    static mut godMode: BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn sendDestroyExtra(psKilled: *mut BASE_OBJECT,
                        psKiller: *mut BASE_OBJECT) -> BOOL;
    // send to net.
    #[no_mangle]
    fn updateMultiStatsDamage(attacker: UDWORD, defender: UDWORD,
                              inflicted: UDWORD);
    #[no_mangle]
    fn updateMultiStatsKills(psKilled: *mut BASE_OBJECT, player: UDWORD);
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
pub type FRACT = libc::c_float;
pub type FRACT_D = libc::c_float;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
// Extension memory for the heap
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
pub type WEAPON_MODIFIER = UWORD;
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
/* * unsigned 32-bit integer */
pub type ALuint = libc::c_uint;
/* **************************************************************************/
/* **************************************************************************/
/* defines */
/* **************************************************************************/
/* **************************************************************************/
/* enums */
/* **************************************************************************/
/* forward definitions
 */
/* **************************************************************************/
/* typedefs
 */
/* **************************************************************************/
/* structs */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_SAMPLE {
    pub iTrack: SDWORD,
    pub iSample: ALuint,
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub iLoops: SDWORD,
    pub bRemove: BOOL,
    pub pCallback: AUDIO_CALLBACK,
    pub psObj: *mut libc::c_void,
    pub psPrev: *mut AUDIO_SAMPLE,
    pub psNext: *mut AUDIO_SAMPLE,
}
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
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
pub type STRUCTSTRENGTH_MODIFIER = UWORD;
pub type STRUCTURE = _structure;
/* **************************************************************************/
/*
 * BulletDef.h
 *
 * Structure Definitions for the bullet object.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
pub type PROJ_STATE = libc::c_uint;
pub const PROJ_POSTIMPACT: PROJ_STATE = 2;
pub const PROJ_IMPACT: PROJ_STATE = 1;
pub const PROJ_INFLIGHT: PROJ_STATE = 0;
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PROJ_OBJECT {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut PROJ_OBJECT,
    pub state: UBYTE,
    pub airTarget: UBYTE,
    pub player: UBYTE,
    pub bVisible: UBYTE,
    pub psWStats: *mut WEAPON_STATS,
    pub psSource: *mut BASE_OBJECT,
    pub psDest: *mut BASE_OBJECT,
    pub startX: UDWORD,
    pub startY: UDWORD,
    pub tarX: UDWORD,
    pub tarY: UDWORD,
    pub vXY: SDWORD,
    pub vZ: SDWORD,
    pub srcHeight: UDWORD,
    pub altChange: SDWORD,
    pub born: UDWORD,
    pub targetRadius: UDWORD,
    pub pInFlightFunc: PROJECTILE_FUNC,
}
pub type PROJECTILE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut PROJ_OBJECT) -> ()>;
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
//used for passing data to the checkBurnDamage function
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fire_box {
    pub x1: SWORD,
    pub y1: SWORD,
    pub x2: SWORD,
    pub y2: SWORD,
    pub rad: SWORD,
}
pub type FIRE_BOX = _fire_box;
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
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
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
/* **************************************************************************/
/* macros
 */
pub type HASHFUNC
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_int) -> UDWORD>;
pub type HASHFREEFUNC
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHNODE {
    pub iKey1: libc::c_int,
    pub iKey2: libc::c_int,
    pub psElement: *mut libc::c_void,
    pub psNext: *mut HASHNODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHTABLE {
    pub psNodeHeap: *mut OBJ_HEAP,
    pub psElementHeap: *mut OBJ_HEAP,
    pub ppsNode: *mut *mut HASHNODE,
    pub psNextNode: *mut HASHNODE,
    pub pHashFunc: HASHFUNC,
    pub pFreeFunc: HASHFREEFUNC,
    pub udwTableSize: UDWORD,
    pub udwElements: UDWORD,
    pub udwExtElements: UDWORD,
    pub udwElementSize: UDWORD,
    pub sdwCurIndex: UDWORD,
}
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
// --------------------------------------------------------------------
pub type DATA_INDEX = data_index;
pub type data_index = libc::c_uint;
pub const WD_BARBARIANS_MOWED_DOWN: data_index = 10;
pub const WD_SHOTS_OFF_TARGET: data_index = 9;
pub const WD_SHOTS_ON_TARGET: data_index = 8;
pub const WD_MISSION_STARTED: data_index = 7;
pub const WD_ARTEFACTS_FOUND: data_index = 6;
pub const WD_STR_LOST: data_index = 5;
pub const WD_STR_KILLED: data_index = 4;
pub const WD_STR_BUILT: data_index = 3;
pub const WD_UNITS_LOST: data_index = 2;
pub const WD_UNITS_KILLED: data_index = 1;
pub const WD_UNITS_BUILT: data_index = 0;
/* Return a pointer to the tile structure at x,y */
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
    //return psMapTiles + x + (y << mapShift); //width no longer a power of 2
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
/* Return whether a world coordinate is on the map */
#[inline]
unsafe extern "C" fn worldOnMap(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    return (x >= 0 as libc::c_int &&
                x < (mapWidth as SDWORD) << 7 as libc::c_int &&
                y >= 0 as libc::c_int &&
                y < (mapHeight as SDWORD) << 7 as libc::c_int) as libc::c_int;
}
/*#define GFX_VISIBLE(psObj)		 ((psObj->psSource != NULL) AND psObj->psSource->visible[selectedPlayer]) OR \
								 ((psObj->psDest != NULL) AND psObj->psDest->visible[selectedPlayer] )*/
/* **************************************************************************/
static mut g_pProjObjTable: *mut HASHTABLE =
    0 as *const HASHTABLE as *mut HASHTABLE;
// the last unit that did damage - used by script functions
#[no_mangle]
pub static mut g_pProjLastAttacker: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn gfxVisible(mut psObj: *mut PROJ_OBJECT) -> BOOL {
    let mut bVisible: BOOL = 0;
    bVisible = 0 as libc::c_int;
    // already know it is visible
    if (*psObj).bVisible != 0 { return 1 as libc::c_int }
    // you fired it
    if (*psObj).player as libc::c_uint == selectedPlayer {
        return 1 as libc::c_int
    }
    // always see in this mode
    if godMode != 0 { return 1 as libc::c_int }
    // you can see the source
    if !(*psObj).psSource.is_null() &&
           (*(*psObj).psSource).visible[selectedPlayer as usize] as
               libc::c_int != 0 {
        bVisible = 1 as libc::c_int
    }
    // you can see the destination
    if !(*psObj).psDest.is_null() &&
           (*(*psObj).psDest).visible[selectedPlayer as usize] as libc::c_int
               != 0 {
        bVisible = 1 as libc::c_int
    }
    // someone elses structure firing at something you can't see
    if !(*psObj).psSource.is_null() &&
           (*(*psObj).psSource).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*(*psObj).psSource).player as libc::c_uint != selectedPlayer &&
           ((*psObj).psDest.is_null() ||
                (*(*psObj).psDest).visible[selectedPlayer as usize] == 0) {
        bVisible = 0 as libc::c_int
    }
    // something you cannot see firing at a structure that isn't yours
    if !(*psObj).psDest.is_null() &&
           (*(*psObj).psDest).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           (*(*psObj).psDest).player as libc::c_uint != selectedPlayer &&
           ((*psObj).psSource.is_null() ||
                (*(*psObj).psSource).visible[selectedPlayer as usize] == 0) {
        bVisible = 0 as libc::c_int
    }
    return bVisible;
}
// how long an object burns for after leaving a fire
// how much damaga a second an object takes when it is burning
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_InitSystem() -> BOOL {
    /* allocate object hashtable */
    hashTable_Create(&mut g_pProjObjTable, 97 as libc::c_int as UDWORD,
                     200 as libc::c_int as UDWORD,
                     10 as libc::c_int as UDWORD,
                     ::std::mem::size_of::<PROJ_OBJECT>() as libc::c_ulong);
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_FreeAllProjectiles() {
    if !g_pProjObjTable.is_null() { hashTable_Clear(g_pProjObjTable); };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_Shutdown() -> BOOL {
    /* destroy hash table */
    hashTable_Destroy(g_pProjObjTable);
    g_pProjObjTable = 0 as *mut HASHTABLE;
    return 1 as libc::c_int;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_GetFirst() -> *mut PROJ_OBJECT {
    return hashTable_GetFirst(g_pProjObjTable) as *mut PROJ_OBJECT;
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_GetNext() -> *mut PROJ_OBJECT {
    return hashTable_GetNext(g_pProjObjTable) as *mut PROJ_OBJECT;
}
/* **************************************************************************/
// update the kills after a target is destroyed
#[no_mangle]
pub unsafe extern "C" fn proj_UpdateKills(mut psObj: *mut PROJ_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID; //, *psSensor;
    let mut psSensor: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT; //, *psTarget;
    if (*psObj).psSource.is_null() ||
           !(*psObj).psDest.is_null() &&
               (*(*psObj).psDest).type_0 as libc::c_uint ==
                   OBJ_FEATURE as libc::c_int as libc::c_uint {
        return
    }
    if bMultiPlayer != 0 {
        sendDestroyExtra((*psObj).psDest, (*psObj).psSource);
        updateMultiStatsKills((*psObj).psDest,
                              (*(*psObj).psSource).player as UDWORD);
    }
    if (*(*psObj).psSource).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        /* update droid kills */
        psDroid = (*psObj).psSource as *mut DROID;
        (*psDroid).numKills = (*psDroid).numKills.wrapping_add(1);
        cmdDroidUpdateKills(psDroid);
        //can't assume the sensor object is a droid - it might be a structure
		/*if (orderStateObj(psDroid, DORDER_FIRESUPPORT, (BASE_OBJECT **)&psSensor))
		{
			psSensor->numKills ++;
		}*/
        if orderStateObj(psDroid, DORDER_FIRESUPPORT, &mut psSensor) != 0 {
            if (*psSensor).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
                let ref mut fresh0 = (*(psSensor as *mut DROID)).numKills;
                *fresh0 = (*fresh0).wrapping_add(1)
            }
        }
    } else if (*(*psObj).psSource).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        // see if there was a command droid designating this target
        psDroid =
            cmdDroidGetDesignator((*(*psObj).psSource).player as UDWORD);
        if !psDroid.is_null() {
            if (*psDroid).action == DACTION_ATTACK as libc::c_int &&
                   (*psDroid).psActionTarget == (*psObj).psDest {
                (*psDroid).numKills = (*psDroid).numKills.wrapping_add(1)
            }
        }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_SendProjectile(mut psWeap: *mut WEAPON,
                                             mut psAttacker: *mut BASE_OBJECT,
                                             mut player: SDWORD,
                                             mut tarX: UDWORD,
                                             mut tarY: UDWORD,
                                             mut tarZ: UDWORD,
                                             mut psTarget: *mut BASE_OBJECT,
                                             mut bVisible: BOOL) -> BOOL {
    let mut psObj: *mut PROJ_OBJECT =
        0 as *mut PROJ_OBJECT; // 52.12 fixed point on PSX, float on PC.
    let mut tarHeight: SDWORD = 0;
    let mut srcHeight: SDWORD = 0;
    let mut iMinSq: SDWORD = 0;
    let mut altChange: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut iVelSq: SDWORD = 0;
    let mut iVel: SDWORD = 0;
    let mut fR: FRACT_D = 0.;
    let mut fA: FRACT_D = 0.;
    let mut fS: FRACT_D = 0.;
    let mut fT: FRACT_D = 0.;
    let mut fC: FRACT_D = 0.;
    let mut muzzle: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut iRadSq: SDWORD = 0;
    let mut iPitchLow: SDWORD = 0;
    let mut iPitchHigh: SDWORD = 0;
    let mut iTemp: SDWORD = 0;
    let mut heightVariance: UDWORD = 0;
    let mut psWeapStats: *mut WEAPON_STATS =
        &mut *asWeaponStats.offset((*psWeap).nStat as isize) as
            *mut WEAPON_STATS;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_SendProjectile: invalid weapon stats\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              280 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"proj_SendProjectile\x00")).as_ptr(),
              b"PTRVALID(psWeapStats,sizeof(WEAPON_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    /* get unused projectile object from hashtable*/
    psObj = hashTable_GetElement(g_pProjObjTable) as *mut PROJ_OBJECT;
    /* get muzzle offset */
    if psAttacker.is_null() {
        // if there isn't an attacker just start at the target position
		// NB this is for the script function to fire the las sats
        muzzle.x = tarX as SDWORD;
        muzzle.y = tarY as SDWORD;
        muzzle.z = tarZ as SDWORD
    } else if (*psAttacker).type_0 as libc::c_uint ==
                  OBJ_DROID as libc::c_int as libc::c_uint {
        calcDroidMuzzleLocation(psAttacker as *mut DROID, &mut muzzle);
        /*update attack runs for VTOL droid's each time a shot is fired*/
        updateVtolAttackRun(psAttacker as *mut DROID);
    } else if (*psAttacker).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        calcStructureMuzzleLocation(psAttacker as *mut STRUCTURE,
                                    &mut muzzle);
    } else {
        // incase anything wants a projectile
        muzzle.x = (*psAttacker).x as int32;
        muzzle.y = (*psAttacker).y as int32;
        muzzle.z = (*psAttacker).z as int32
        /* GJ - horrible hack to get droid tower projectiles at correct height */
		//muzzle.z = psAttacker->z + psAttacker->sDisplay.imd->ymax;
    }
    /* Initialise the structure */
    (*psObj).type_0 = OBJ_BULLET; // New - needed to backtrack FX
    (*psObj).state = PROJ_INFLIGHT as libc::c_int as UBYTE;
    (*psObj).psWStats = asWeaponStats.offset((*psWeap).nStat as isize);
    (*psObj).x = muzzle.x as UWORD;
    (*psObj).y = muzzle.y as UWORD;
    (*psObj).z = muzzle.z as UWORD;
    (*psObj).startX = muzzle.x as UDWORD;
    (*psObj).startY = muzzle.y as UDWORD;
    (*psObj).tarX = tarX;
    (*psObj).tarY = tarY;
    (*psObj).targetRadius = establishTargetRadius(psTarget);
    (*psObj).psSource = psAttacker;
    (*psObj).psDest = psTarget;
    (*psObj).born = gameTime;
    (*psObj).player = player as UBYTE;
    (*psObj).bVisible = 0 as libc::c_int as UBYTE;
    (*psObj).airTarget =
        (!psTarget.is_null() &&
             (*psTarget).type_0 as libc::c_uint ==
                 OBJ_DROID as libc::c_int as libc::c_uint &&
             vtolDroid(psTarget as *mut DROID) != 0 ||
             psTarget.is_null() &&
                 tarZ as SDWORD > map_Height(tarX, tarY) as libc::c_int) as
            libc::c_int as UBYTE;
    if !psTarget.is_null() {
        scoreUpdateVar(WD_SHOTS_ON_TARGET);
        heightVariance = 0 as libc::c_int as UDWORD;
        match (*psTarget).type_0 as libc::c_uint {
            0 | 2 => {
                if (*(psTarget as *mut DROID)).droidType as libc::c_uint ==
                       DROID_PERSON as libc::c_int as libc::c_uint {
                    heightVariance = (rand() % 8 as libc::c_int) as UDWORD
                } else {
                    heightVariance = (rand() % 24 as libc::c_int) as UDWORD
                }
            }
            1 => {
                heightVariance =
                    (rand() % (*(*psTarget).sDisplay.imd).ymax) as UDWORD
            }
            _ => { }
        }
        tarHeight =
            ((*psTarget).z as libc::c_uint).wrapping_add(heightVariance) as
                SDWORD
    } else {
        //		tarHeight = map_Height(tarX,tarY);
        tarHeight = tarZ as SDWORD;
        scoreUpdateVar(WD_SHOTS_OFF_TARGET);
    }
    srcHeight = muzzle.z;
    altChange = tarHeight - srcHeight;
    (*psObj).srcHeight = srcHeight as UDWORD;
    (*psObj).altChange = altChange;
    dx = (*psObj).tarX as SDWORD - muzzle.x;
    dy = (*psObj).tarY as SDWORD - muzzle.y;
    dz = tarHeight - muzzle.z;
    /* roll never set */
    (*psObj).roll = 0 as libc::c_int as SWORD;
    fR = atan2(dx as libc::c_double, dy as libc::c_double) as FRACT_D;
    if (fR as libc::c_double) < 0.0f64 {
        fR += (2 as libc::c_int as libc::c_double * 3.141592654f64) as FRACT_D
    }
    (*psObj).direction =
        (fR as libc::c_double * 180.0f64 / 3.141592654f64) as UWORD;
    /* get target distance */
    iRadSq = dx * dx + dy * dy + dz * dz;
    fR = trigIntSqrt(iRadSq as UDWORD);
    iMinSq =
        (*psWeapStats).minRange.wrapping_mul((*psWeapStats).minRange) as
            SDWORD;
    if proj_Direct((*psObj).psWStats) != 0 ||
           proj_Direct(psWeapStats) == 0 && iRadSq <= iMinSq {
        fR = atan2(dz as libc::c_double, fR as libc::c_double) as FRACT_D;
        if (fR as libc::c_double) < 0.0f64 {
            fR +=
                (2 as libc::c_int as libc::c_double * 3.141592654f64) as
                    FRACT_D
        }
        (*psObj).pitch =
            (fR as libc::c_double * 180.0f64 / 3.141592654f64) as SWORD;
        //	DBPRINTF(("dx=%d dy=%d dir=%d\n",dx,dy,psObj->direction);
//DBPRINTF(("direct- pitch=%d direction=%d\n",psObj->pitch,psObj->direction);
		/* set function pointer */
        (*psObj).pInFlightFunc =
            Some(proj_InFlightDirectFunc as
                     unsafe extern "C" fn(_: *mut PROJ_OBJECT) -> ())
    } else {
        /* indirect */
        iVelSq =
            (*(*psObj).psWStats).flightSpeed.wrapping_mul((*(*psObj).psWStats).flightSpeed)
                as SDWORD;
        fA =
            1000 as libc::c_int as libc::c_float * iRadSq as FRACT_D /
                (2 as libc::c_int * iVelSq) as libc::c_float;
        fC = 4 as libc::c_int as libc::c_float * fA * (dz as FRACT_D + fA);
        fS = iRadSq as FRACT_D - fC;
        /* target out of range - increase velocity to hit target */
        if fS < 0 as libc::c_int as FRACT_D {
            /* set optimal pitch */
            (*psObj).pitch = 30 as libc::c_int as SWORD;
            /* increase velocity: tan angle could be hard-coded but is variable here */
//			fS = trigSin(PROJ_MAX_PITCH);
//			fC = trigCos(PROJ_MAX_PITCH);
//			fT = FRACTdiv( fS, fC );
//			fS = ACC_GRAVITY*(1+(fT*fT)) / (MAKEFRACT(2) * (fR*fT - MAKEFRACT(dz)));
//			iVel = MAKEINT_D( trigIntSqrt(MAKEINT(fS*fR*fR)) );
            fS = trigSin(30 as libc::c_int);
            fC = trigCos(30 as libc::c_int);
            fT = fS / fC;
            //			fS = ACC_GRAVITY*(1+(fT*fT)) / (MAKEFRACT(2) * (fR*fT - MAKEFRACT(dz)));
//			iVel = MAKEINT( trigIntSqrt(MAKEINT(fS*fR*fR)) );
            fS =
                1000 as libc::c_int as libc::c_float *
                    (1 as libc::c_int as FRACT_D + fT * fT);
            fS =
                fS /
                    (2 as libc::c_int as libc::c_float *
                         (fR * fT - dz as FRACT_D));
            let mut Tmp: FRACT_D = 0.;
            Tmp = fR * fR;
            iVel = trigIntSqrt((fS * Tmp) as SDWORD as UDWORD) as SDWORD
        } else {
            /* set velocity to stats value */
            iVel = (*(*psObj).psWStats).flightSpeed as SDWORD;
            /* get floating point square root */
            fS = trigIntSqrt(fS as SDWORD as UDWORD);
            fT =
                atan2((fR + fS) as libc::c_double,
                      (2 as libc::c_int as libc::c_float * fA) as
                          libc::c_double) as FRACT_D;
            /* make sure angle positive */
            if fT < 0 as libc::c_int as libc::c_float {
                fT +=
                    (2 as libc::c_int as libc::c_double * 3.141592654f64) as
                        FRACT_D
            }
            iPitchLow =
                (fT as libc::c_double * 180.0f64 / 3.141592654f64) as SDWORD;
            fT =
                atan2((fR - fS) as libc::c_double,
                      (2 as libc::c_int as libc::c_float * fA) as
                          libc::c_double) as FRACT_D;
            /* make sure angle positive */
            if fT < 0 as libc::c_int as libc::c_float {
                fT +=
                    (2 as libc::c_int as libc::c_double * 3.141592654f64) as
                        FRACT_D
            }
            iPitchHigh =
                (fT as libc::c_double * 180.0f64 / 3.141592654f64) as SDWORD;
            /* swap pitches if wrong way round */
            if iPitchLow > iPitchHigh {
                iTemp = iPitchHigh;
                iPitchLow = iPitchHigh;
                iPitchHigh = iTemp
            }
            /* chooselow pitch unless -ve */
            if iPitchLow > 0 as libc::c_int {
                (*psObj).pitch = iPitchLow as SWORD
            } else { (*psObj).pitch = iPitchHigh as SWORD }
        }
        /* if droid set muzzle pitch */
        if !psAttacker.is_null() {
            if (*psAttacker).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
                (*(psAttacker as *mut DROID)).turretPitch =
                    (*psObj).pitch as UWORD
            } else if (*psAttacker).type_0 as libc::c_uint ==
                          OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                (*(psAttacker as *mut STRUCTURE)).turretPitch =
                    (*psObj).pitch as UWORD
            }
        }
        (*psObj).vXY =
            (iVel as libc::c_float * trigCos((*psObj).pitch as SDWORD)) as
                SDWORD;
        (*psObj).vZ =
            (iVel as libc::c_float * trigSin((*psObj).pitch as SDWORD)) as
                SDWORD;
        /* set function pointer */
        (*psObj).pInFlightFunc =
            Some(proj_InFlightIndirectFunc as
                     unsafe extern "C" fn(_: *mut PROJ_OBJECT) -> ())
    }
    /* put object in hashtable */
    hashTable_InsertElement(g_pProjObjTable, psObj as *mut libc::c_void,
                            psObj as libc::c_int, -(747 as libc::c_int));
    /* play firing audio */
	// only play if either object is visible, i know it's a bit of a hack, but it avoids the problem
	// of having to calculate real visibility values for each projectile.
//	if(  ((psObj->psSource != NULL) && psObj->psSource->visible[selectedPlayer] ) ||
//		 ((psObj->psDest != NULL) && psObj->psDest->visible[selectedPlayer]     )    )
//	if(GFX_VISIBLE(psObj))
    if bVisible != 0 || gfxVisible(psObj) != 0 {
        // note that the projectile is visible
        (*psObj).bVisible = 1 as libc::c_int as UBYTE;
        if (*(*psObj).psWStats).iAudioFireID != NO_SOUND as libc::c_int {
            if !(*psObj).psSource.is_null() {
                /* firing sound emitted from source */
                audio_PlayObjDynamicTrack((*psObj).psSource as
                                              *mut libc::c_void,
                                          (*(*psObj).psWStats).iAudioFireID,
                                          None);
                /* GJ HACK: move howitzer sound with shell */
                if (*(*psObj).psWStats).weaponSubClass as libc::c_uint ==
                       WSC_HOWITZERS as libc::c_int as libc::c_uint {
                    audio_PlayObjDynamicTrack(psObj as *mut BASE_OBJECT as
                                                  *mut libc::c_void,
                                              ID_SOUND_HOWITZ_FLIGHT as
                                                  libc::c_int, None);
                }
            } else if !(bMultiPlayer != 0 &&
                            (*psWeapStats).weaponSubClass as libc::c_uint ==
                                WSC_LAS_SAT as libc::c_int as libc::c_uint) {
                audio_PlayObjStaticTrack(psObj as *mut libc::c_void,
                                         (*(*psObj).psWStats).iAudioFireID);
            }
        }
    }
    if !psAttacker.is_null() && proj_Direct(psWeapStats) == 0 {
        //don't play the sound for a LasSat in multiPlayer
        //check for Counter Battery Sensor in range of target
        counterBatteryFire(psAttacker, psTarget);
    }
    return 1 as libc::c_int;
}
/* **************************************************************************/
unsafe extern "C" fn proj_InFlightDirectFunc(mut psObj: *mut PROJ_OBJECT) {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut timeSoFar: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut rad: SDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_InFlightDirectFunc: invalid projectile pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              572 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"proj_InFlightDirectFunc\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(PROJ_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats = (*psObj).psWStats;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_InFlightDirectFunc: Invalid weapon stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              576 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"proj_InFlightDirectFunc\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    timeSoFar = gameTime.wrapping_sub((*psObj).born) as SDWORD;
    //we want a delay between Las-Sats firing and actually hitting in multiPlayer
    if bMultiPlayer != 0 {
        if (*psStats).weaponSubClass as libc::c_uint ==
               WSC_LAS_SAT as libc::c_int as libc::c_uint {
            //magic number but that's how long the audio countdown message lasts!
            if timeSoFar < 8 as libc::c_int * 1000 as libc::c_int { return }
        }
    }
    /*
	if(psObj->psDest)
	{
		erraticX = 128-rand()%256;
		erraticY = 128-rand()%256;

		dx = ( (SDWORD)psObj->psDest->x+erraticX) -(SDWORD)psObj->x;
		dy = ( (SDWORD)psObj->psDest->y+erraticY) -(SDWORD)psObj->y;

	}
	else
	{
*/
    //	}
    /* If it's homing and it has a target (not a miss)... */
    if (*psStats).movementModel as libc::c_uint ==
           MM_HOMINGDIRECT as libc::c_int as libc::c_uint &&
           !(*psObj).psDest.is_null() {
        dx = (*(*psObj).psDest).x as SDWORD - (*psObj).startX as SDWORD;
        dy = (*(*psObj).psDest).y as SDWORD - (*psObj).startY as SDWORD;
        dz = (*(*psObj).psDest).z as SDWORD - (*psObj).srcHeight as SDWORD
    } else {
        dx = (*psObj).tarX as SDWORD - (*psObj).startX as SDWORD;
        dy = (*psObj).tarY as SDWORD - (*psObj).startY as SDWORD;
        dz =
            (*psObj).srcHeight.wrapping_add((*psObj).altChange as
                                                libc::c_uint) as SDWORD -
                (*psObj).srcHeight as SDWORD
    }
    /*
	dx = (SDWORD)psObj->tarX-(SDWORD)psObj->startX;
	dy = (SDWORD)psObj->tarY-(SDWORD)psObj->startY;
	*/
    // ffs
//	rad = fastRoot(dx,dy);
    rad = sqrt((dx * dx + dy * dy) as libc::c_double) as FRACT as SDWORD;
    if rad == 0 as libc::c_int { rad = 1 as libc::c_int }
    dist =
        (timeSoFar as
             libc::c_uint).wrapping_mul((*psStats).flightSpeed).wrapping_div(1000
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
            as SDWORD;
    iX =
        (*psObj).startX.wrapping_add((dist * dx / rad) as libc::c_uint) as
            SDWORD;
    iY =
        (*psObj).startY.wrapping_add((dist * dy / rad) as libc::c_uint) as
            SDWORD;
    //	iX = psObj->x + MAKEINT(FRACTmul(baseSpeed, MAKEFRACT(dx))) / rad;
//	iY = psObj->y + MAKEINT(FRACTmul(baseSpeed, MAKEFRACT(dy))) / rad;
    /* impact if about to go off map else update coordinates */
    if worldOnMap(iX, iY) == 0 as libc::c_int {
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE;
        debug(LOG_NEVER,
              b"**** projectile off map - removed ****\n\x00" as *const u8 as
                  *const libc::c_char);
        return
    } else { (*psObj).x = iX as UWORD; (*psObj).y = iY as UWORD }
    (*psObj).z =
        (*psObj).srcHeight.wrapping_add((dist * dz / rad) as libc::c_uint) as
            UWORD;
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_FLAME as libc::c_int as libc::c_uint {
        //		if(GFX_VISIBLE(psObj))
        if gfxVisible(psObj) != 0 {
            effectGiveAuxVar((dist * 100 as libc::c_int / rad) as UDWORD);
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int - 8 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_FLAMETHROWER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    } else if (*psStats).weaponSubClass as libc::c_uint ==
                  WSC_COMMAND as libc::c_int as libc::c_uint ||
                  (*psStats).weaponSubClass as libc::c_uint ==
                      WSC_ELECTRONIC as libc::c_int as libc::c_uint ||
                  (*psStats).weaponSubClass as libc::c_uint ==
                      WSC_EMP as libc::c_int as libc::c_uint {
        if gfxVisible(psObj) != 0 {
            //		if(GFX_VISIBLE(psObj))
            effectGiveAuxVar((dist * 100 as libc::c_int / rad /
                                  2 as libc::c_int) as UDWORD);
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int - 8 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LASER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            //		addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_FLARE,FALSE,NULL,0);
        }
    }
    /*
	else
	if(psStats->weaponSubClass == WSC_ROCKET OR psStats->weaponSubClass == WSC_MISSILE OR
        psStats->weaponSubClass == WSC_SLOWROCKET OR psStats->weaponSubClass == WSC_SLOWMISSILE)
	{
		if(GFX_VISIBLE(psObj))
		{
			pos.x = psObj->x;
			pos.y = psObj->z-8;
			pos.z = psObj->y;
			addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_FLARE,FALSE,NULL,0);
		}
	}
	*/
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_ROCKET as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_MISSILE as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_SLOWROCKET as libc::c_int as libc::c_uint ||
           (*psStats).weaponSubClass as libc::c_uint ==
               WSC_SLOWMISSILE as libc::c_int as libc::c_uint {
        if gfxVisible(psObj) != 0 {
            //		if(GFX_VISIBLE(psObj))
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int + 8 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_TRAIL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    }
    /* See if effect has finished */
    if (*psStats).movementModel as libc::c_uint ==
           MM_HOMINGDIRECT as libc::c_int as libc::c_uint &&
           !(*psObj).psDest.is_null() {
        xdiff = (*psObj).x as SDWORD - (*(*psObj).psDest).x as SDWORD;
        ydiff = (*psObj).y as SDWORD - (*(*psObj).psDest).y as SDWORD;
        if xdiff * xdiff + ydiff * ydiff <
               (*psObj).targetRadius as SDWORD *
                   (*psObj).targetRadius as SDWORD {
            (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE
        }
    } else if dist > rad - (*psObj).targetRadius as SDWORD {
        /* It's damage time */
//		psObj->x = psObj->tarX;		// leave it there, but use tarX and tarY for damage
//		psObj->y = psObj->tarY;
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE
    }
    /* check not trying to travel through terrain - if so count as a miss */
    if mapObjIsAboveGround(psObj as *mut BASE_OBJECT) == 0 as libc::c_int {
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE;
        /* miss registered if NULL target */
        (*psObj).psDest = 0 as *mut BASE_OBJECT;
        return
    }
    /* add smoke trail to indirect weapons firing directly */
    if proj_Direct(psStats) == 0 && gfxVisible(psObj) != 0 {
        //GFX_VISIBLE(psObj) )
        pos.x = (*psObj).x as int32;
        pos.y = (*psObj).z as libc::c_int + 8 as libc::c_int;
        pos.z = (*psObj).y as int32;
        addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_TRAIL, 0 as libc::c_int,
                  0 as *mut iIMDShape, 0 as libc::c_int);
    };
}
/* **************************************************************************/
unsafe extern "C" fn proj_InFlightIndirectFunc(mut psObj: *mut PROJ_OBJECT) {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut iTime: SDWORD = 0;
    let mut iRad: SDWORD = 0;
    let mut iDist: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut fVVert: FRACT = 0.;
    let mut bOver: BOOL = 0 as libc::c_int;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_InFlightIndirectFunc: invalid projectile pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              769 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"proj_InFlightIndirectFunc\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(PROJ_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats = (*psObj).psWStats;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_InFlightIndirectFunc: Invalid weapon stats pointer\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              773 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"proj_InFlightIndirectFunc\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    iTime = gameTime.wrapping_sub((*psObj).born) as SDWORD;
    dx = (*psObj).tarX as SDWORD - (*psObj).startX as SDWORD;
    dy = (*psObj).tarY as SDWORD - (*psObj).startY as SDWORD;
    // ffs
    iRad =
        if abs(dx) > abs(dy) {
            (abs(dx)) + abs(dy) / 2 as libc::c_int
        } else { (abs(dy)) + abs(dx) / 2 as libc::c_int };
    //DBPRINTF(("dx=%d dy=%d irad=%d\n",dx,dy,iRad);
    iDist = iTime * (*psObj).vXY / 1000 as libc::c_int;
    iX =
        (*psObj).startX.wrapping_add((iDist * dx / iRad) as libc::c_uint) as
            SDWORD;
    iY =
        (*psObj).startY.wrapping_add((iDist * dy / iRad) as libc::c_uint) as
            SDWORD;
    /* impact if about to go off map else update coordinates */
    if worldOnMap(iX, iY) == 0 as libc::c_int {
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE;
        debug(LOG_NEVER,
              b"**** projectile off map - removed ****\n\x00" as *const u8 as
                  *const libc::c_char);
        return
    } else { (*psObj).x = iX as UWORD; (*psObj).y = iY as UWORD }
    dz =
        ((*psObj).vZ -
             iTime * 1000 as libc::c_int / 1000 as libc::c_int /
                 2 as libc::c_int) * iTime / 1000 as libc::c_int;
    (*psObj).z = (*psObj).srcHeight.wrapping_add(dz as libc::c_uint) as UWORD;
    //DBPRINTF(("missile: dist=%d time=%d x=%d y=%d z=%d vxy=%d\n",iDist,iTime,psObj->x,psObj->y,psObj->z,psObj->vXY);
    fVVert =
        ((*psObj).vZ - iTime * 1000 as libc::c_int / 1000 as libc::c_int) as
            FRACT;
    (*psObj).pitch =
        (atan2(fVVert as libc::c_double, (*psObj).vXY as libc::c_double) *
             180.0f64 / 3.141592654f64) as SWORD;
    if (*psStats).weaponSubClass as libc::c_uint ==
           WSC_FLAME as libc::c_int as libc::c_uint {
        if gfxVisible(psObj) != 0 {
            //		if(GFX_VISIBLE(psObj))
            effectGiveAuxVar((iDist * 100 as libc::c_int / iRad) as UDWORD);
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int - 8 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_FLAMETHROWER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    } else if (*psStats).weaponSubClass as libc::c_uint ==
                  WSC_COMMAND as libc::c_int as libc::c_uint ||
                  (*psStats).weaponSubClass as libc::c_uint ==
                      WSC_ELECTRONIC as libc::c_int as libc::c_uint ||
                  (*psStats).weaponSubClass as libc::c_uint ==
                      WSC_EMP as libc::c_int as libc::c_uint {
        if gfxVisible(psObj) != 0 {
            //		if(GFX_VISIBLE(psObj))
            effectGiveAuxVar((iDist * 100 as libc::c_int / iRad /
                                  2 as libc::c_int) as UDWORD);
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int - 8 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LASER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            //		addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_FLARE,FALSE,NULL,0);
        }
    }
    /*
	else
	if(psStats->weaponSubClass == WSC_ROCKET)
	{
		if(GFX_VISIBLE(psObj))
		{
			pos.x = psObj->x;
			pos.y = psObj->z-8;
			pos.z = psObj->y;
			addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_FLARE,FALSE,NULL,0);
		}
	}
	else
	{
		if(GFX_VISIBLE(psObj))
		{
			pos.x = psObj->x;
			pos.y = psObj->z;
			pos.z = psObj->y;
			addEffect(&pos,EFFECT_SMOKE,SMOKE_TYPE_TRAIL,FALSE,NULL,0);
		}
	}
	*/
    /* See if effect has finished */
    if iDist > iRad - (*psObj).targetRadius as SDWORD {
        pos.x = (*psObj).x as int32;
        pos.z = (*psObj).y as int32;
        pos.y =
            map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
                8 as libc::c_int;
        /* It's damage time */
//		psObj->x = psObj->tarX;		 // leave it where it is, but use tarX, tarY for damage
//		psObj->y = psObj->tarY;
        (*psObj).z =
            (pos.y + 8 as libc::c_int) as
                UWORD; //map_Height(psObj->x,psObj->y) + 8;	// bring up the impact explosion
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE;
        bOver = 1 as libc::c_int
    }
    /* check not trying to travel through terrain - if so count as a miss */
    if mapObjIsAboveGround(psObj as *mut BASE_OBJECT) == 0 as libc::c_int {
        (*psObj).state = PROJ_IMPACT as libc::c_int as UBYTE;
        /* miss registered if NULL target */
        (*psObj).psDest = 0 as *mut BASE_OBJECT;
        bOver = 1 as libc::c_int
    }
    /* Add smoke particle at projectile location (in world coords) */
	/* Add a trail graphic */
	/* If it's indirect and not a flamethrower - add a smoke trail! */
	/* MAKE IT ADD A 'TRAIL GRAPHIC'? */
    if (*psStats).weaponSubClass as libc::c_uint !=
           WSC_FLAME as libc::c_int as libc::c_uint &&
           (*psStats).weaponSubClass as libc::c_uint !=
               WSC_ENERGY as libc::c_int as libc::c_uint &&
           (*psStats).weaponSubClass as libc::c_uint !=
               WSC_COMMAND as libc::c_int as libc::c_uint &&
           (*psStats).weaponSubClass as libc::c_uint !=
               WSC_ELECTRONIC as libc::c_int as libc::c_uint &&
           (*psStats).weaponSubClass as libc::c_uint !=
               WSC_EMP as libc::c_int as libc::c_uint && bOver == 0 {
        if gfxVisible(psObj) != 0 {
            //		if(GFX_VISIBLE(psObj))// AND psStats->pTrailGraphic )
            pos.x = (*psObj).x as int32;
            pos.y = (*psObj).z as libc::c_int + 4 as libc::c_int;
            pos.z = (*psObj).y as int32;
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_TRAIL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    };
}
/* **************************************************************************/
unsafe extern "C" fn proj_ImpactFunc(mut psObj: *mut PROJ_OBJECT) {
    let mut psStats: *mut WEAPON_STATS =
        0 as *mut WEAPON_STATS; //,bMultiTemp;
    let mut i: SDWORD =
        0; //optimisation - were all being calculated twice on PC
    let mut iAudioImpactID: SDWORD = 0;
    let mut psCurrD: *mut DROID = 0 as *mut DROID;
    let mut psNextD: *mut DROID = 0 as *mut DROID;
    let mut psCurrS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNextS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psCurrF: *mut FEATURE = 0 as *mut FEATURE;
    let mut psNextF: *mut FEATURE = 0 as *mut FEATURE;
    let mut dice: UDWORD = 0;
    let mut tarX0: SDWORD = 0;
    let mut tarY0: SDWORD = 0;
    let mut tarX1: SDWORD = 0;
    let mut tarY1: SDWORD = 0;
    let mut radSquared: SDWORD = 0;
    let mut xDiff: SDWORD = 0;
    let mut yDiff: SDWORD = 0;
    let mut bKilled: BOOL = 0;
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut scatter: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut damage: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_ImpactFunc: invalid projectile pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              933 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"proj_ImpactFunc\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(PROJ_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats = (*psObj).psWStats;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_ImpactFunc: Invalid weapon stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              937 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"proj_ImpactFunc\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* play impact audio */
    if gfxVisible(psObj) != 0 {
        if (*psStats).iAudioImpactID == NO_SOUND as libc::c_int {
            // ID_SOUND_RICOCHET_1 is -1 on PSX so code below must be PC only.
            /* play richochet if MG */
            if !(*psObj).psDest.is_null() &&
                   (*(*psObj).psWStats).weaponSubClass as libc::c_uint ==
                       WSC_MGUN as libc::c_int as libc::c_uint &&
                   rand() % 3 as libc::c_int == 0 as libc::c_int {
                iAudioImpactID =
                    ID_SOUND_RICOCHET_1 as libc::c_int +
                        rand() % 3 as libc::c_int;
                audio_PlayStaticTrack((*(*psObj).psDest).x as SDWORD,
                                      (*(*psObj).psDest).y as SDWORD,
                                      iAudioImpactID);
            }
        } else if (*psObj).psDest.is_null() {
            audio_PlayStaticTrack((*psObj).tarX as SDWORD,
                                  (*psObj).tarY as SDWORD,
                                  (*psStats).iAudioImpactID);
        } else {
            audio_PlayStaticTrack((*(*psObj).psDest).x as SDWORD,
                                  (*(*psObj).psDest).y as SDWORD,
                                  (*psStats).iAudioImpactID);
        }
    }
    // note the attacker if any
    g_pProjLastAttacker = (*psObj).psSource;
    if gfxVisible(psObj) != 0 {
        //	if(GFX_VISIBLE(psObj))
        /* Set stuff burning if it's a flame droid... */
//		if(psStats->weaponSubClass == WSC_FLAME)
//		{
//			position.x = psObj->tarX;
//			position.z = psObj->tarY;
//			position.y = map_Height(position.x, position.z);
			/* Shouldn't need to do this check but the stats aren't all at a value yet... */
        // FIXME
        if (*psStats).incenRadius != 0 && (*psStats).incenTime != 0 {
            position.x = (*psObj).tarX as int32;
            position.z = (*psObj).tarY as int32;
            position.y =
                map_Height(position.x as UDWORD, position.z as UDWORD) as
                    int32;
            effectGiveAuxVar((*psStats).incenRadius);
            effectGiveAuxVarSec((*psStats).incenTime);
            addEffect(&mut position, EFFECT_FIRE, FIRE_TYPE_LOCALISED,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        if (*psStats).weaponSubClass as libc::c_uint ==
               WSC_LAS_SAT as libc::c_int as libc::c_uint {
            //		else if (psStats->weaponSubClass == 100)
            position.x = (*psObj).tarX as int32;
            position.z = (*psObj).tarY as int32;
            position.y =
                map_Height(position.x as UDWORD, position.z as UDWORD) as
                    int32;
            addEffect(&mut position, EFFECT_SAT_LASER, SAT_LASER_STANDARD,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            if clipXY((*psObj).tarX as SDWORD, (*psObj).tarY as SDWORD) != 0 {
                shakeStart();
            }
        }
    }
    //		}
		// may want to add both a fire effect and the las sat effect
    /* Nothings been killed */
    bKilled = 0 as libc::c_int;
    if !(*psObj).psDest.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"proj_ImpactFunc: Invalid destination object pointer\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"projectile.c\x00" as *const u8 as *const libc::c_char,
                  1013 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"proj_ImpactFunc\x00")).as_ptr(),
                  b"PTRVALID(psObj->psDest, sizeof(BASE_OBJECT))\x00" as
                      *const u8 as *const libc::c_char);
        };
    }
    if (*psObj).psDest.is_null() {
        //		 (psObj->x != psObj->psDest->x) || (psObj->y != psObj->psDest->y) )
        /* The bullet missed or the target was destroyed in flight */
		/* So show the MISS effect */
        position.x =
            (*psObj).x as int32; //map_Height(psObj->x, psObj->y) + 24;//jps
        position.z = (*psObj).y as int32;
        position.y = (*psObj).z as int32;
        scatter.x = (*psStats).radius as int32;
        scatter.y = 0 as libc::c_int;
        scatter.z = (*psStats).radius as int32;
        if (*psObj).airTarget != 0 {
            if (*psStats).surfaceToAir as libc::c_int & 0x2 as libc::c_int !=
                   0 &&
                   (*psStats).surfaceToAir as libc::c_int & 0x1 as libc::c_int
                       == 0 {
                if (*psStats).facePlayer != 0 {
                    if gfxVisible(psObj) != 0 {
                        //					if(GFX_VISIBLE(psObj))
                        addMultiEffect(&mut position, &mut scatter,
                                       EFFECT_EXPLOSION,
                                       EXPLOSION_TYPE_SPECIFIED,
                                       1 as libc::c_int,
                                       (*psStats).pTargetMissGraphic,
                                       (*psStats).numExplosions,
                                       (*psStats).lightWorld,
                                       (*psStats).effectSize as UDWORD);
                    }
                } else if gfxVisible(psObj) != 0 {
                    //					if(GFX_VISIBLE(psObj))
                    addMultiEffect(&mut position, &mut scatter,
                                   EFFECT_EXPLOSION,
                                   EXPLOSION_TYPE_NOT_FACING,
                                   1 as libc::c_int,
                                   (*psStats).pTargetMissGraphic,
                                   (*psStats).numExplosions,
                                   (*psStats).lightWorld,
                                   (*psStats).effectSize as UDWORD);
                }
                /* Add some smoke to flak */
                addMultiEffect(&mut position, &mut scatter, EFFECT_SMOKE,
                               SMOKE_TYPE_DRIFTING, 0 as libc::c_int,
                               0 as *mut iIMDShape,
                               3 as libc::c_int as UDWORD, 0 as libc::c_int,
                               0 as libc::c_int as UDWORD);
            }
        } else if terrainTypes[((*mapTile(((*psObj).x as libc::c_int /
                                               128 as libc::c_int) as UDWORD,
                                          ((*psObj).y as libc::c_int /
                                               128 as libc::c_int) as
                                              UDWORD)).texture as libc::c_int
                                    & 0x1ff as libc::c_int) as usize] as
                      libc::c_int == TER_WATER as libc::c_int {
            if (*psStats).facePlayer != 0 {
                if gfxVisible(psObj) != 0 {
                    //				if(GFX_VISIBLE(psObj))
                    addMultiEffect(&mut position, &mut scatter,
                                   EFFECT_EXPLOSION, EXPLOSION_TYPE_SPECIFIED,
                                   1 as libc::c_int,
                                   (*psStats).pWaterHitGraphic,
                                   (*psStats).numExplosions,
                                   (*psStats).lightWorld,
                                   (*psStats).effectSize as UDWORD);
                }
                //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_SPECIFIED,TRUE,psStats->pWaterHitGraphic);
            } else if gfxVisible(psObj) != 0 {
                //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_NOT_FACING,TRUE,psStats->pWaterHitGraphic);
                //				if(GFX_VISIBLE(psObj))
                addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                               EXPLOSION_TYPE_NOT_FACING, 1 as libc::c_int,
                               (*psStats).pWaterHitGraphic,
                               (*psStats).numExplosions,
                               (*psStats).lightWorld,
                               (*psStats).effectSize as UDWORD);
            }
        } else if (*psStats).facePlayer != 0 {
            //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_SPECIFIED,TRUE,psStats->pTargetMissGraphic);
            if gfxVisible(psObj) != 0 {
                //				if(GFX_VISIBLE(psObj))
                addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                               EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                               (*psStats).pTargetMissGraphic,
                               (*psStats).numExplosions,
                               (*psStats).lightWorld,
                               (*psStats).effectSize as UDWORD);
            }
        } else if gfxVisible(psObj) != 0 {
            //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_NOT_FACING,TRUE,psStats->pTargetMissGraphic);
            //				if(GFX_VISIBLE(psObj))
            addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                           EXPLOSION_TYPE_NOT_FACING, 1 as libc::c_int,
                           (*psStats).pTargetMissGraphic,
                           (*psStats).numExplosions, (*psStats).lightWorld,
                           (*psStats).effectSize as UDWORD);
        }
    } else {
        position.x = (*psObj).x as int32;
        position.z = (*psObj).y as int32;
        // you got to have at least 1 in the numExplosions field or you'll see nowt..:-)
        position.y =
            (*psObj).z as int32; //map_Height(psObj->x, psObj->y) + 24;
        scatter.x = (*psStats).radius as int32;
        scatter.y = 0 as libc::c_int;
        scatter.z = (*psStats).radius as int32;
        if (*psStats).facePlayer != 0 {
            //			addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_SPECIFIED,TRUE,psStats->pTargetHitGraphic);
            if gfxVisible(psObj) != 0 {
                //			if(GFX_VISIBLE(psObj))
                addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                               EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                               (*psStats).pTargetHitGraphic,
                               (*psStats).numExplosions,
                               (*psStats).lightWorld,
                               (*psStats).effectSize as UDWORD);
            }
        } else if gfxVisible(psObj) != 0 {
            //			addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_NOT_FACING,TRUE,psStats->pTargetHitGraphic);
            //			if(GFX_VISIBLE(psObj))
            addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                           EXPLOSION_TYPE_NOT_FACING, 1 as libc::c_int,
                           (*psStats).pTargetHitGraphic,
                           (*psStats).numExplosions, (*psStats).lightWorld,
                           (*psStats).effectSize as UDWORD);
        }
        /* Do damage to the target */
        if proj_Direct(psStats) != 0 {
            /*Check for Electronic Warfare damage where we know the subclass
            of the weapon*/
            if (*psStats).weaponSubClass as libc::c_uint ==
                   WSC_ELECTRONIC as libc::c_int as libc::c_uint {
                // AND psObj->psDest->
                //type == OBJ_STRUCTURE)
                if !(*psObj).psSource.is_null() {
                    //if (electronicDamage((STRUCTURE *)psObj->psDest, calcDamage(
                    if electronicDamage((*psObj).psDest,
                                        calcDamage(weaponDamage(psStats,
                                                                (*psObj).player),
                                                   (*psStats).weaponEffect,
                                                   (*psObj).psDest),
                                        (*psObj).player) != 0 {
                        if (*(*psObj).psSource).type_0 as libc::c_uint ==
                               OBJ_DROID as libc::c_int as libc::c_uint {
                            //the structure has lost all resistance so quit
                            (*((*psObj).psSource as *mut DROID)).order =
                                DORDER_NONE as libc::c_int;
                            actionDroid((*psObj).psSource as *mut DROID,
                                        DACTION_NONE);
                        } else if (*(*psObj).psSource).type_0 as libc::c_uint
                                      ==
                                      OBJ_STRUCTURE as libc::c_int as
                                          libc::c_uint {
                            /*else
						{
							ASSERT( FALSE, "proj_ImpactFunc: EW Weapon not attached to a droid" );
						}*/
                            //the droid has lost all resistance - init the structures target
                            let ref mut fresh1 =
                                (*((*psObj).psSource as
                                       *mut STRUCTURE)).psTarget;
                            *fresh1 = 0 as *mut BASE_OBJECT
                        }
                    }
                }
            } else {
                /* Just assume a direct fire weapon hits the target */
                damage =
                    calcDamage(weaponDamage(psStats, (*psObj).player),
                               (*psStats).weaponEffect, (*psObj).psDest);
                if bMultiPlayer != 0 {
                    if !(*psObj).psSource.is_null() &&
                           myResponsibility((*(*psObj).psSource).player as
                                                UDWORD) != 0 {
                        updateMultiStatsDamage((*(*psObj).psSource).player as
                                                   UDWORD,
                                               (*(*psObj).psDest).player as
                                                   UDWORD, damage);
                    }
                    // do the attacked callback
	/*			psScrCBAttacker = psObj->psSource;
				psScrCBTarget = psObj->psObj->psDest;
				eventFireCallbackTrigger(CALL_ATTACKED);
				psScrCBAttacker = NULL;
				psScrCBTarget = NULL;*/
                    //				bMultiTemp = TRUE;
	//				bMultiPlayer = FALSE;
                }
                //			else
	//			{
	//				bMultiTemp = FALSE;
	//			}
                /*the damage depends on the weapon effect and the target propulsion type or structure strength*/
                bKilled =
                    objectDamage((*psObj).psDest, damage,
                                 (*psStats).weaponClass as UDWORD,
                                 (*psStats).weaponSubClass as UDWORD);
                if bKilled != 0 { proj_UpdateKills(psObj); }
            }
        } else if (*(*psObj).psDest).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint ||
                      (*(*psObj).psDest).type_0 as libc::c_uint ==
                          OBJ_FEATURE as libc::c_int as libc::c_uint ||
                      (*(*psObj).psDest).x as SDWORD >=
                          (*psObj).x as SDWORD - 128 as libc::c_int &&
                          (*(*psObj).psDest).x as SDWORD <=
                              (*psObj).x as SDWORD + 128 as libc::c_int &&
                          (*(*psObj).psDest).y as SDWORD >=
                              (*psObj).y as SDWORD - 128 as libc::c_int &&
                          (*(*psObj).psDest).y as SDWORD <=
                              (*psObj).y as SDWORD + 128 as libc::c_int {
            damage =
                calcDamage(weaponDamage(psStats, (*psObj).player),
                           (*psStats).weaponEffect, (*psObj).psDest);
            if bMultiPlayer != 0 {
                if !(*psObj).psSource.is_null() &&
                       myResponsibility((*(*psObj).psSource).player as UDWORD)
                           != 0 {
                    //#ifndef PSX
	//			if(bMultiTemp)
	//			{
	//				bMultiPlayer = TRUE;
	//			}
	//#endif
                    /* See if the target is still close to the impact point */
			/* Should do a better test than this but as we don't have */
			/* a world size for objects we'll just see if it is within */
			/* a tile of the bullet */
			/* **********************************************************/
			/*  MIGHT WANT TO CHANGE THIS                              */
			/* **********************************************************/
                    // do the attacked callback
/*				psScrCBAttacker = psObj->psSource;
				psScrCBTarget = psObj->psObj->psDest;
				eventFireCallbackTrigger(CALL_ATTACKED);
				psScrCBAttacker = NULL;
				psScrCBTarget = NULL;*/
                    //updateMultiStatsDamage(psObj->psSource->player,psObj->psDest->player,psStats->damage);
                    updateMultiStatsDamage((*(*psObj).psSource).player as
                                               UDWORD,
                                           (*(*psObj).psDest).player as
                                               UDWORD, damage);
                }
                //					bMultiPlayer = FALSE;
//					bMultiTemp = TRUE;
            }
            //				else
//				{
//					bMultiTemp = FALSE;
//				}
            //my_error("",0,"","Damage to object %d, player %d\n",psObj->psDest->id, psObj->psDest->player);
            //bKilled = psObj->psDest->damage(psObj->psDest, psStats->damage,psStats->weaponClass);
				/*bKilled = psObj->psDest->damage(psObj->psDest, calcDamage(
					//psStats->damage, psStats->weaponEffect, psObj->psDest),
					weaponDamage(psStats,psObj->player), psStats->
					weaponEffect, psObj->psDest), psStats->weaponClass);*/
            bKilled =
                objectDamage((*psObj).psDest, damage,
                             (*psStats).weaponClass as UDWORD,
                             (*psStats).weaponSubClass as UDWORD);
            if bKilled != 0 { proj_UpdateKills(psObj); }
        }
    }
    if (*psStats).radius == 0 as libc::c_int as libc::c_uint &&
           (*psStats).incenTime == 0 as libc::c_int as libc::c_uint {
        position.x = (*psObj).x as int32;
        position.z = (*psObj).y as int32;
        //#ifndef PSX
//				if(bMultiTemp)
//				{
//					bMultiPlayer = TRUE;
//				}
//#endif
        position.y =
            (*psObj).z as int32; //map_Height(psObj->x, psObj->y) + 24;//jps
        scatter.x = (*psStats).radius as int32;
        scatter.y = 0 as libc::c_int;
        scatter.z = (*psStats).radius as int32;
        if terrainTypes[((*mapTile(((*psObj).x as libc::c_int /
                                        128 as libc::c_int) as UDWORD,
                                   ((*psObj).y as libc::c_int /
                                        128 as libc::c_int) as
                                       UDWORD)).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_WATER as libc::c_int {
            if (*psStats).facePlayer != 0 {
                //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_SPECIFIED,TRUE,psStats->pWaterHitGraphic);
                if gfxVisible(psObj) != 0 {
                    //				if(GFX_VISIBLE(psObj))
                    addMultiEffect(&mut position, &mut scatter,
                                   EFFECT_EXPLOSION, EXPLOSION_TYPE_SPECIFIED,
                                   1 as libc::c_int,
                                   (*psStats).pWaterHitGraphic,
                                   (*psStats).numExplosions,
                                   (*psStats).lightWorld,
                                   (*psStats).effectSize as UDWORD);
                }
            } else if gfxVisible(psObj) != 0 {
                //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_NOT_FACING,TRUE,psStats->pWaterHitGraphic);
                //				if(GFX_VISIBLE(psObj))
                addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                               EXPLOSION_TYPE_NOT_FACING, 1 as libc::c_int,
                               (*psStats).pWaterHitGraphic,
                               (*psStats).numExplosions,
                               (*psStats).lightWorld,
                               (*psStats).effectSize as UDWORD);
            }
        } else if (*psStats).facePlayer != 0 {
            //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_SPECIFIED,TRUE,psStats->pTargetHitGraphic);
            if gfxVisible(psObj) != 0 {
                //				if(GFX_VISIBLE(psObj))
                addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                               EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                               (*psStats).pTargetHitGraphic,
                               (*psStats).numExplosions,
                               (*psStats).lightWorld,
                               (*psStats).effectSize as UDWORD);
            }
        } else if gfxVisible(psObj) != 0 {
            //				addEffect(&position,EFFECT_EXPLOSION,EXPLOSION_TYPE_NOT_FACING,TRUE,psStats->pTargetHitGraphic);
            //				if(GFX_VISIBLE(psObj))
            addMultiEffect(&mut position, &mut scatter, EFFECT_EXPLOSION,
                           EXPLOSION_TYPE_NOT_FACING, 1 as libc::c_int,
                           (*psStats).pTargetHitGraphic,
                           (*psStats).numExplosions, (*psStats).lightWorld,
                           (*psStats).effectSize as UDWORD);
        }
        /* This was just a simple bullet - release it and return */
        if hashTable_RemoveElement(g_pProjObjTable,
                                   psObj as *mut libc::c_void,
                                   psObj as libc::c_int,
                                   -(747 as libc::c_int)) == 0 as libc::c_int
           {
            debug(LOG_NEVER,
                  b"proj_ImpactFunc: couldn\'t remove projectile from table\n\x00"
                      as *const u8 as *const libc::c_char);
        }
        return
    }
    if (*psStats).radius != 0 as libc::c_int as libc::c_uint {
        /* An area effect bullet */
        (*psObj).state = PROJ_POSTIMPACT as libc::c_int as UBYTE;
        /* Note when it exploded for the explosion effect */
        (*psObj).born = gameTime;
        /* Work out the bounding box for the blast radius */
        tarX0 = (*psObj).x as SDWORD - (*psStats).radius as SDWORD;
        tarY0 = (*psObj).y as SDWORD - (*psStats).radius as SDWORD;
        tarX1 = (*psObj).x as SDWORD + (*psStats).radius as SDWORD;
        tarY1 = (*psObj).y as SDWORD + (*psStats).radius as SDWORD;
        /* Store the radius squared */
        radSquared =
            (*psStats).radius.wrapping_mul((*psStats).radius) as SDWORD;
        /* Do damage to everything in range */
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            psCurrD = apsDroidLists[i as usize];
            while !psCurrD.is_null() {
                /* have to store the next pointer as psCurrD could be destroyed */
                psNextD = (*psCurrD).psNext;
                if !(vtolDroid(psCurrD) != 0 &&
                         (*psCurrD).sMove.Status as libc::c_int !=
                             0 as libc::c_int) {
                    /* see if psCurrD is hit (don't hit main target twice) */
                    if psCurrD as *mut BASE_OBJECT != (*psObj).psDest &&
                           (*psCurrD).x as SDWORD >= tarX0 &&
                           (*psCurrD).x as SDWORD <= tarX1 &&
                           (*psCurrD).y as SDWORD >= tarY0 &&
                           (*psCurrD).y as SDWORD <= tarY1 {
                        /* Within the bounding box, now check the radius */
                        xDiff =
                            (*psCurrD).x as libc::c_int -
                                (*psObj).x as libc::c_int;
                        yDiff =
                            (*psCurrD).y as libc::c_int -
                                (*psObj).y as libc::c_int;
                        if xDiff * xDiff + yDiff * yDiff <= radSquared {
                            dice = (rand() % 100 as libc::c_int) as UDWORD;
                            //if (dice < psStats->radiusHit)
                            if dice <
                                   weaponRadiusHit(psStats, (*psObj).player) {
                                damage =
                                    calcDamage(weaponRadDamage(psStats,
                                                               (*psObj).player),
                                               (*psStats).weaponEffect,
                                               psCurrD as *mut BASE_OBJECT);
                                if bMultiPlayer != 0 {
                                    if !(*psObj).psSource.is_null() &&
                                           myResponsibility((*(*psObj).psSource).player
                                                                as UDWORD) !=
                                               0 {
                                        updateMultiStatsDamage((*(*psObj).psSource).player
                                                                   as UDWORD,
                                                               (*psCurrD).player
                                                                   as UDWORD,
                                                               damage);
                                    }
                                    turnOffMultiMsg(1 as libc::c_int);
                                }
                                //bKilled = psCurrD->damage(psCurrD, psStats->radiusDamage, psStats->weaponClass);
							/*bKilled = psCurrD->damage(psCurrD, calcDamage(
								//psStats->radiusDamage, psStats->weaponEffect,
								weaponRadDamage(psStats,psObj->player),
								psStats->weaponEffect,
								(BASE_OBJECT *)psCurrD), psStats->weaponClass);*/
                                bKilled =
                                    droidDamage(psCurrD, damage,
                                                (*psStats).weaponClass as
                                                    UDWORD,
                                                (*psStats).weaponSubClass as
                                                    UDWORD); // multiplay msgs back on.
                                turnOffMultiMsg(0 as libc::c_int);
                                if bKilled != 0 { proj_UpdateKills(psObj); }
                            }
                        }
                    }
                }
                // skip VTOLs in the air
                psCurrD = psNextD
            }
            psCurrS = apsStructLists[i as usize];
            while !psCurrS.is_null() {
                /* have to store the next pointer as psCurrD could be destroyed */
                psNextS = (*psCurrS).psNext;
                /* see if psCurrS is hit (don't hit main target twice) */
                if psCurrS as *mut BASE_OBJECT != (*psObj).psDest &&
                       (*psCurrS).x as SDWORD >= tarX0 &&
                       (*psCurrS).x as SDWORD <= tarX1 &&
                       (*psCurrS).y as SDWORD >= tarY0 &&
                       (*psCurrS).y as SDWORD <= tarY1 {
                    /* Within the bounding box, now check the radius */
                    xDiff =
                        (*psCurrS).x as libc::c_int -
                            (*psObj).x as libc::c_int;
                    yDiff =
                        (*psCurrS).y as libc::c_int -
                            (*psObj).y as libc::c_int;
                    if xDiff * xDiff + yDiff * yDiff <= radSquared {
                        dice = (rand() % 100 as libc::c_int) as UDWORD;
                        //if (dice < psStats->radiusHit)
                        if dice < weaponRadiusHit(psStats, (*psObj).player) {
                            damage =
                                calcDamage(weaponRadDamage(psStats,
                                                           (*psObj).player),
                                           (*psStats).weaponEffect,
                                           psCurrS as *mut BASE_OBJECT);
                            if bMultiPlayer != 0 {
                                if !(*psObj).psSource.is_null() &&
                                       myResponsibility((*(*psObj).psSource).player
                                                            as UDWORD) != 0 {
                                    updateMultiStatsDamage((*(*psObj).psSource).player
                                                               as UDWORD,
                                                           (*psCurrS).player
                                                               as UDWORD,
                                                           damage);
                                }
                            }
                            bKilled =
                                structureDamage(psCurrS, damage,
                                                (*psStats).weaponClass as
                                                    UDWORD,
                                                (*psStats).weaponSubClass as
                                                    UDWORD);
                            if bKilled != 0 { proj_UpdateKills(psObj); }
                        }
                    }
                } else if ptInStructure(psCurrS, (*psObj).x as UDWORD,
                                        (*psObj).y as UDWORD) != 0 &&
                              psCurrS as *mut BASE_OBJECT != (*psObj).psDest {
                    damage = 5 as libc::c_int as UDWORD;
                    if bMultiPlayer != 0 {
                        if !(*psObj).psSource.is_null() &&
                               myResponsibility((*(*psObj).psSource).player as
                                                    UDWORD) != 0 {
                            updateMultiStatsDamage((*(*psObj).psSource).player
                                                       as UDWORD,
                                                   (*psCurrS).player as
                                                       UDWORD, damage);
                        }
                    }
                    bKilled =
                        structureDamage(psCurrS, damage,
                                        (*psStats).weaponClass as UDWORD,
                                        (*psStats).weaponSubClass as UDWORD);
                    if bKilled != 0 { proj_UpdateKills(psObj); }
                }
                psCurrS = psNextS
            }
            i += 1
        }
        psCurrF = apsFeatureLists[0 as libc::c_int as usize];
        while !psCurrF.is_null() {
            // Missed by old method, but maybe in landed within the building's footprint(baseplate)
            /* have to store the next pointer as psCurrD could be destroyed */
            psNextF = (*psCurrF).psNext;
            //ignore features that are not damageable
            if !((*(*psCurrF).psStats).damageable == 0) {
                /* see if psCurrS is hit (don't hit main target twice) */
                if psCurrF as *mut BASE_OBJECT != (*psObj).psDest &&
                       (*psCurrF).x as SDWORD >= tarX0 &&
                       (*psCurrF).x as SDWORD <= tarX1 &&
                       (*psCurrF).y as SDWORD >= tarY0 &&
                       (*psCurrF).y as SDWORD <= tarY1 {
                    /* Within the bounding box, now check the radius */
                    xDiff =
                        (*psCurrF).x as libc::c_int -
                            (*psObj).x as libc::c_int;
                    yDiff =
                        (*psCurrF).y as libc::c_int -
                            (*psObj).y as libc::c_int;
                    if xDiff * xDiff + yDiff * yDiff <= radSquared {
                        dice = (rand() % 100 as libc::c_int) as UDWORD;
                        //if (dice < psStats->radiusHit)
                        if dice < weaponRadiusHit(psStats, (*psObj).player) {
                            //(void)psCurrF->damage(psCurrF, psStats->radiusDamage, psStats->weaponClass);
						//(void)psCurrF->damage(psCurrF, calcDamage(psStats->radiusDamage,
						/*(void)psCurrF->damage(psCurrF, calcDamage(weaponRadDamage(
							psStats, psObj->player), psStats->weaponEffect,
							(BASE_OBJECT *)psCurrF), psStats->weaponClass);*/
                            bKilled =
                                featureDamage(psCurrF,
                                              calcDamage(weaponRadDamage(psStats,
                                                                         (*psObj).player),
                                                         (*psStats).weaponEffect,
                                                         psCurrF as
                                                             *mut BASE_OBJECT),
                                              (*psStats).weaponClass as
                                                  UDWORD,
                                              (*psStats).weaponSubClass as
                                                  UDWORD);
                            if bKilled != 0 { proj_UpdateKills(psObj); }
                        }
                    }
                }
            }
            psCurrF = psNextF
        }
    }
    if (*psStats).incenTime != 0 as libc::c_int as libc::c_uint {
        /* Incendiary round */
		/* Incendiary damage gets done in the bullet update routine */
		/* Just note when the incendiary started burning            */
        (*psObj).state = PROJ_POSTIMPACT as libc::c_int as UBYTE;
        (*psObj).born = gameTime
    };
    /* Something was blown up */
}
/* **************************************************************************/
unsafe extern "C" fn proj_PostImpactFunc(mut psObj: *mut PROJ_OBJECT) {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut i: SDWORD = 0;
    let mut age: SDWORD = 0;
    let mut flame: FIRE_BOX = FIRE_BOX{x1: 0, y1: 0, x2: 0, y2: 0, rad: 0,};
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_PostImpactFunc: invalid projectile pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              1532 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"proj_PostImpactFunc\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(PROJ_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats = (*psObj).psWStats;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_PostImpactFunc: Invalid weapon stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              1536 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"proj_PostImpactFunc\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(WEAPON_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    age = gameTime as SDWORD - (*psObj).born as SDWORD;
    /* Time to finish postimpact effect? */
    if age > (*psStats).radiusLife as SDWORD &&
           age > (*psStats).incenTime as SDWORD {
        if hashTable_RemoveElement(g_pProjObjTable,
                                   psObj as *mut libc::c_void,
                                   psObj as libc::c_int,
                                   -(747 as libc::c_int)) == 0 as libc::c_int
           {
            debug(LOG_NEVER,
                  b"proj_PostImpactFunc: couldn\'t remove projectile from table\n\x00"
                      as *const u8 as *const libc::c_char);
        }
        return
    }
    /* Burning effect */
    if (*psStats).incenTime > 0 as libc::c_int as libc::c_uint {
        /* See if anything is in the fire and burn it */
        /* Calculate the fire's bounding box */
        flame.x1 =
            ((*psObj).x as libc::c_uint).wrapping_sub((*psStats).incenRadius)
                as SWORD;
        flame.y1 =
            ((*psObj).y as libc::c_uint).wrapping_sub((*psStats).incenRadius)
                as SWORD;
        flame.x2 =
            ((*psObj).x as libc::c_uint).wrapping_add((*psStats).incenRadius)
                as SWORD;
        flame.y2 =
            ((*psObj).y as libc::c_uint).wrapping_add((*psStats).incenRadius)
                as SWORD;
        flame.rad =
            (*psStats).incenRadius.wrapping_mul((*psStats).incenRadius) as
                SWORD;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            /* Don't damage your own droids - unrealistic, but better */
            if i != (*psObj).player as libc::c_int {
                proj_checkBurnDamage(apsDroidLists[i as usize] as
                                         *mut BASE_OBJECT, psObj, &mut flame);
                proj_checkBurnDamage(apsStructLists[i as usize] as
                                         *mut BASE_OBJECT, psObj, &mut flame);
            }
            i += 1
        }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_Update(mut psObj: *mut PROJ_OBJECT) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"proj_Update: Invalid bullet pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"projectile.c\x00" as *const u8 as *const libc::c_char,
              1581 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"proj_Update\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(PROJ_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* See if any of the stored objects have died
	 * since the projectile was created
	 */
    if !(*psObj).psSource.is_null() && (*(*psObj).psSource).died != 0 {
        (*psObj).psSource = 0 as *mut BASE_OBJECT
    }
    if !(*psObj).psDest.is_null() && (*(*psObj).psDest).died != 0 {
        (*psObj).psDest = 0 as *mut BASE_OBJECT
    }
    match (*psObj).state as libc::c_int {
        0 => {
            (*psObj).pInFlightFunc.expect("non-null function pointer")(psObj);
        }
        1 => { proj_ImpactFunc(psObj); }
        2 => { proj_PostImpactFunc(psObj); }
        _ => { }
    };
}
/* **************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn proj_UpdateAll() {
    let mut psObj: *mut PROJ_OBJECT = 0 as *mut PROJ_OBJECT;
    psObj = hashTable_GetFirst(g_pProjObjTable) as *mut PROJ_OBJECT;
    while !psObj.is_null() {
        proj_Update(psObj);
        psObj = hashTable_GetNext(g_pProjObjTable) as *mut PROJ_OBJECT
    };
}
//static void	proj_MachineGunInFlightFunc( PROJ_OBJECT *psObj );
/* **************************************************************************/
unsafe extern "C" fn proj_checkBurnDamage(mut apsList: *mut BASE_OBJECT,
                                          mut psProj: *mut PROJ_OBJECT,
                                          mut pFireBox: *mut FIRE_BOX) {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psNext: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut xDiff: SDWORD = 0;
    let mut yDiff: SDWORD = 0;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut damageSoFar: UDWORD = 0;
    let mut damageToDo: SDWORD = 0;
    let mut bKilled: BOOL = 0;
    //	BOOL			bMultiTemp;
    // note the attacker if any
    g_pProjLastAttacker = (*psProj).psSource;
    psStats = (*psProj).psWStats;
    psCurr = apsList;
    while !psCurr.is_null() {
        /* have to store the next pointer as psCurr could be destroyed */
        psNext = (*psCurr).psNext;
        if !((*psCurr).type_0 as libc::c_uint ==
                 OBJ_DROID as libc::c_int as libc::c_uint &&
                 vtolDroid(psCurr as *mut DROID) != 0 &&
                 (*(psCurr as *mut DROID)).sMove.Status as libc::c_int !=
                     0 as libc::c_int) {
            /* see if psCurr is hit (don't hit main target twice) */
            if (*psCurr).x as SDWORD >= (*pFireBox).x1 as libc::c_int &&
                   (*psCurr).x as SDWORD <= (*pFireBox).x2 as libc::c_int &&
                   (*psCurr).y as SDWORD >= (*pFireBox).y1 as libc::c_int &&
                   (*psCurr).y as SDWORD <= (*pFireBox).y2 as libc::c_int {
                /* Within the bounding box, now check the radius */
                xDiff =
                    (*psCurr).x as libc::c_int - (*psProj).x as libc::c_int;
                yDiff =
                    (*psCurr).y as libc::c_int - (*psProj).y as libc::c_int;
                if xDiff * xDiff + yDiff * yDiff <=
                       (*pFireBox).rad as libc::c_int {
                    /* The object is in the fire */
                    (*psCurr).inFire |= 0x1 as libc::c_int;
                    if (*psCurr).burnStart == 0 as libc::c_int as libc::c_uint
                           || (*psCurr).inFire & 0x2 as libc::c_int != 0 {
                        /* This is the first turn the object is in the fire */
                        (*psCurr).burnStart = gameTime;
                        (*psCurr).burnDamage = 0 as libc::c_int as UDWORD
                    } else {
                        /* Calculate how much damage should have
					   been done up till now */
                        damageSoFar =
                            gameTime.wrapping_sub((*psCurr).burnStart).wrapping_mul(weaponIncenDamage(psStats,
                                                                                                      (*psProj).player)).wrapping_div(1000
                                                                                                                                          as
                                                                                                                                          libc::c_int
                                                                                                                                          as
                                                                                                                                          libc::c_uint);
                        damageToDo =
                            damageSoFar as SDWORD -
                                (*psCurr).burnDamage as SDWORD;
                        if damageToDo > 0 as libc::c_int {
                            /* The damage could be negative if the object
					   is being burn't by another fire
					   with a higher burn damage */
                            //#ifndef PSX
//						if(bMultiPlayer)
//						{
//							bMultiPlayer = FALSE;
//							bMultiTemp = TRUE;
//						}
//						else
//						{
//							bMultiTemp = FALSE;
//						}
//#endif
                            //bKilled = psCurr->damage(psCurr, damageToDo, psStats->weaponClass);
                            bKilled =
                                objectDamage(psCurr, damageToDo as UDWORD,
                                             (*psStats).weaponClass as UDWORD,
                                             (*psStats).weaponSubClass as
                                                 UDWORD);
                            (*psCurr).burnDamage =
                                ((*psCurr).burnDamage as
                                     libc::c_uint).wrapping_add(damageToDo as
                                                                    libc::c_uint)
                                    as UDWORD as UDWORD;
                            //#ifndef PSX
//						if(bMultiTemp)
//						{
//							bMultiPlayer = TRUE;
//						}
//#endif
                            if bKilled != 0 { proj_UpdateKills(psProj); }
                        }
                    }
                }
            }
        }
        // can't set flying vtols on fire
        psCurr = psNext
    };
}
// return whether a weapon is direct or indirect
/* **************************************************************************/
// return whether a weapon is direct or indirect
#[no_mangle]
pub unsafe extern "C" fn proj_Direct(mut psStats: *mut WEAPON_STATS) -> BOOL {
    match (*psStats).movementModel as libc::c_uint {
        0 | 2 | 4 | 5 => { return 1 as libc::c_int }
        1 | 3 => { return 0 as libc::c_int }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"proj_Direct: unknown movement model\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"projectile.c\x00" as *const u8 as *const libc::c_char,
                      1752 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"proj_Direct\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 1 as libc::c_int;
}
// return the maximum range for a weapon
/* **************************************************************************/
// return the maximum range for a weapon
#[no_mangle]
pub unsafe extern "C" fn proj_GetLongRange(mut psStats: *mut WEAPON_STATS,
                                           mut dz: SDWORD) -> SDWORD {
    //	dz;
    return (*psStats).longRange as SDWORD;
}
/* **************************************************************************/
/* **************************************************************************/
unsafe extern "C" fn establishTargetRadius(mut psTarget: *mut BASE_OBJECT)
 -> UDWORD {
    let mut radius: UDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    radius = 0 as libc::c_int as UDWORD;
    if psTarget.is_null() {
        // Can this happen?
        return 0 as libc::c_int as UDWORD
    } else {
        match (*psTarget).type_0 as libc::c_uint {
            0 => {
                radius = (128 as libc::c_int / 4 as libc::c_int) as UDWORD
            }
            1 => {
                psStructure =
                    psTarget as *mut STRUCTURE; // how will we arrive at this?
                radius =
                    (if (*(*psStructure).pStructureType).baseBreadth >
                            (*(*psStructure).pStructureType).baseWidth {
                         (*(*psStructure).pStructureType).baseBreadth
                     } else {
                         (*(*psStructure).pStructureType).baseWidth
                     }).wrapping_mul(128 as libc::c_int as
                                         libc::c_uint).wrapping_div(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
            }
            2 => {
                //			radius = TILE_UNITS/4;	// how will we arrive at this?
                psFeat = psTarget as *mut FEATURE;
                radius =
                    ((if (*(*psFeat).psStats).baseBreadth as libc::c_int >
                             (*(*psFeat).psStats).baseWidth as libc::c_int {
                          (*(*psFeat).psStats).baseBreadth as libc::c_int
                      } else {
                          (*(*psFeat).psStats).baseWidth as libc::c_int
                      }) * 128 as libc::c_int / 2 as libc::c_int) as UDWORD
            }
            _ => { }
        }
    }
    return radius;
}
/*
// The fattest macro around - change this little bastard at your peril
#define GFX_VISIBLE(psObj)	(							\
	(													\
		(psObj->player == selectedPlayer)				\
	)													\
	OR													\
	(													\
		(psObj->psSource != NULL)						\
		AND												\
		(												\
			(psObj->psSource->type == OBJ_STRUCTURE && psObj->psSource->player == selectedPlayer) OR\
			(psObj->psSource->type == OBJ_STRUCTURE && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psSource->type == OBJ_DROID     && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psSource->type == OBJ_DROID     && psObj->psSource->player == selectedPlayer) \
		)												\
	)													\
	OR													\
	(													\
		(psObj->psDest != NULL)							\
		AND												\
		(												\
			(psObj->psDest->type == OBJ_STRUCTURE && psObj->psDest->player == selectedPlayer) OR\
			(psObj->psDest->type == OBJ_STRUCTURE && psObj->psSource->visible[selectedPlayer]) OR\
			(psObj->psDest->type == OBJ_DROID     && psObj->psDest->visible[selectedPlayer]) OR\
			(psObj->psDest->type == OBJ_DROID     && psObj->psSource->player == selectedPlayer) \
		)												\
	)													\
	OR													\
	(													\
		godMode											\
	)													\
)
*/
/* **************************************************************************/
/*the damage depends on the weapon effect and the target propulsion type or
structure strength*/
#[no_mangle]
pub unsafe extern "C" fn calcDamage(mut baseDamage: UDWORD,
                                    mut weaponEffect: WEAPON_EFFECT,
                                    mut psTarget: *mut BASE_OBJECT)
 -> UDWORD {
    let mut damage: UDWORD = 0;
    //default value
    damage = baseDamage;
    if (*psTarget).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        damage =
            baseDamage.wrapping_mul(asStructStrengthModifier[weaponEffect as
                                                                 usize][(*(*(psTarget
                                                                                 as
                                                                                 *mut STRUCTURE)).pStructureType).strength
                                                                            as
                                                                            usize]
                                        as
                                        libc::c_uint).wrapping_div(100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
        //a little fail safe!
        if damage == 0 as libc::c_int as libc::c_uint &&
               baseDamage != 0 as libc::c_int as libc::c_uint {
            damage = 1 as libc::c_int as UDWORD
        }
    } else if (*psTarget).type_0 as libc::c_uint ==
                  OBJ_DROID as libc::c_int as libc::c_uint {
        damage =
            baseDamage.wrapping_mul(asWeaponModifier[weaponEffect as
                                                         usize][(*asPropulsionStats.offset((*(psTarget
                                                                                                  as
                                                                                                  *mut DROID)).asBits[COMP_PROPULSION
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          usize].nStat
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize)).propulsionType
                                                                    as usize]
                                        as
                                        libc::c_uint).wrapping_div(100 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint);
        //a little fail safe!
        if damage == 0 as libc::c_int as libc::c_uint &&
               baseDamage != 0 as libc::c_int as libc::c_uint {
            damage = 1 as libc::c_int as UDWORD
        }
    }
    return damage;
}
unsafe extern "C" fn objectDamage(mut psObj: *mut BASE_OBJECT,
                                  mut damage: UDWORD, mut weaponClass: UDWORD,
                                  mut weaponSubClass: UDWORD) -> BOOL {
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            return droidDamage(psObj as *mut DROID, damage, weaponClass,
                               weaponSubClass)
        }
        1 => {
            return structureDamage(psObj as *mut STRUCTURE, damage,
                                   weaponClass, weaponSubClass)
        }
        2 => {
            return featureDamage(psObj as *mut FEATURE, damage, weaponClass,
                                 weaponSubClass)
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"objectDamage - unknown object type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"projectile.c\x00" as *const u8 as *const libc::c_char,
                      1899 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"objectDamage\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return 0 as libc::c_int;
}
/* **************************************************************************/
// got to be over 5 frames per sec.
/* Returns true if an object has just been hit by an electronic warfare weapon*/
#[no_mangle]
pub unsafe extern "C" fn justBeenHitByEW(mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if gamePaused() != 0 {
        return 0 as libc::c_int
        // Don't shake when paused...!
    }
    let mut current_block_13: u64;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            psDroid = psObj as *mut DROID;
            if gameTime.wrapping_sub((*psDroid).timeLastHit) <
                   (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint &&
                   (*psDroid).lastHitWeapon ==
                       WSC_ELECTRONIC as libc::c_int as libc::c_uint {
                return 1 as libc::c_int
            }
            current_block_13 = 3856339402597749846;
        }
        2 => { current_block_13 = 3856339402597749846; }
        1 => {
            psStructure = psObj as *mut STRUCTURE;
            if gameTime.wrapping_sub((*psStructure).timeLastHit) <
                   (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint &&
                   (*psStructure).lastHitWeapon ==
                       WSC_ELECTRONIC as libc::c_int as libc::c_uint {
                return 1 as libc::c_int
            }
            current_block_13 = 7149356873433890176;
        }
        _ => {
            debug(LOG_ERROR,
                  b"Weird object type in justBeenHitByEW\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    match current_block_13 {
        3856339402597749846 => {
            psFeature = psObj as *mut FEATURE;
            if gameTime.wrapping_sub((*psFeature).timeLastHit) <
                   (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint {
                return 1 as libc::c_int
            }
        }
        _ => { }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn objectShimmy(mut psObj: *mut BASE_OBJECT) {
    if justBeenHitByEW(psObj) != 0 {
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
        if (*psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            pie_TRANSLATE(1 as libc::c_int - rand() % 3 as libc::c_int,
                          0 as libc::c_int,
                          1 as libc::c_int - rand() % 3 as libc::c_int);
        }
    };
}
