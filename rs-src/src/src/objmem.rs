use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
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
    fn heapCreate(ppsHeap: *mut *mut OBJ_HEAP, size: UDWORD, init: UDWORD,
                  ext: UDWORD) -> BOOL;
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn heapDestroy(psHeap: *mut OBJ_HEAP);
    /* Release all resources associated with a structure */
    #[no_mangle]
    fn structureRelease(psBuilding: *mut STRUCTURE);
    /* Release the resources associated with a feature */
    #[no_mangle]
    fn featureRelease(psFeature: *mut FEATURE);
    #[no_mangle]
    fn droidRelease(psDroid: *mut DROID);
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
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    // Check all the base pointers to see if they have died
    #[no_mangle]
    fn scrvUpdateBasePointers();
    // the object that was last killed for a CALL_OBJ_DESTROYED
    #[no_mangle]
    static mut psCBObjDestroyed: *mut BASE_OBJECT;
    #[no_mangle]
    static mut apsLimboDroids: [*mut DROID; 8];
    #[no_mangle]
    static mut mission: MISSION;
    // initialise the group system
    // shutdown the group system
    // create a new group
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    // add a droid to a group
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
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
pub type _tech_level = libc::c_uint;
pub const MAX_TECH_LEVELS: _tech_level = 6;
pub const TECH_LEVEL_ALL: _tech_level = 5;
pub const TECH_LEVEL_TWO_THREE: _tech_level = 4;
pub const TECH_LEVEL_ONE_TWO: _tech_level = 3;
pub const TECH_LEVEL_THREE: _tech_level = 2;
pub const TECH_LEVEL_TWO: _tech_level = 1;
pub const TECH_LEVEL_ONE: _tech_level = 0;
pub type TECH_LEVEL = _tech_level;
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
pub type STRUCT_STRENGTH = _struct_strength;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REPAIR_FACILITY {
    pub power: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub powerAccrued: UDWORD,
    pub psDeliveryPoint: *mut FLAG_POSITION,
    pub currentPtsAdded: UDWORD,
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
}
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
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
// Feature armour
/*
 * Group.h
 *
 * Link droids together into a group for AI etc.
 *
 */
// standard group
// command droid group
// transporter group
pub type DROID_GROUP = _droid_group;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
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
 * ObjMem.c
 *
 * Object memory management functions.
 *
 */
//#define DEBUG_GROUP1
/* Allocation sizes for the droid, structure and feature heaps */
// Surely this can be reduced.
// was 40 but this there is 116 templates in template.txt alone ... Arse ... 84 bytes each as well ... arse ...
/* The memory heaps for the different object types */
#[no_mangle]
pub static mut psDroidHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut psStructHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
#[no_mangle]
pub static mut psFeatureHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/*Heap for structure functionality*/
#[no_mangle]
pub static mut psStructFuncHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
/* The memory heap for the Flag Postions */
#[no_mangle]
pub static mut psFlagPosHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// the memory heap for templates
#[no_mangle]
pub static mut psTemplateHeap: *mut OBJ_HEAP =
    0 as *const OBJ_HEAP as *mut OBJ_HEAP;
//SDWORD	factoryDeliveryPointCheck[MAX_PLAYERS][NUM_FACTORY_TYPES][MAX_FACTORY];
#[no_mangle]
pub static mut factoryDeliveryPointCheck: [[[SDWORD; 5]; 4]; 8] =
    [[[0; 5]; 4]; 8];
/* The id number for the next object allocated
 * Each object will have a unique id number irrespective of type
 */
#[no_mangle]
pub static mut objID: UDWORD = 0;
/* The lists of objects allocated */
#[no_mangle]
pub static mut apsDroidLists: [*mut DROID; 8] =
    [0 as *const DROID as *mut DROID; 8];
#[no_mangle]
pub static mut apsStructLists: [*mut STRUCTURE; 8] =
    [0 as *const STRUCTURE as *mut STRUCTURE; 8];
#[no_mangle]
pub static mut apsFeatureLists: [*mut FEATURE; 8] =
    [0 as *const FEATURE as *mut FEATURE; 8];
// Only player zero is valid for
// features
/* The list of structure functionality's required*/
#[no_mangle]
pub static mut apsStructFuncLists: [*mut FUNCTIONALITY; 8] =
    [0 as *const FUNCTIONALITY as *mut FUNCTIONALITY; 8];
/*The list of Flag Positions allocated */
#[no_mangle]
pub static mut apsFlagPosLists: [*mut FLAG_POSITION; 8] =
    [0 as *const FLAG_POSITION as *mut FLAG_POSITION; 8];
/* The list of destroyed objects */
#[no_mangle]
pub static mut psDestroyedObj: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
/* Initialise the object heaps */
#[no_mangle]
pub unsafe extern "C" fn objmemInitialise() -> BOOL {
    if heapCreate(&mut psDroidHeap,
                  ::std::mem::size_of::<DROID>() as libc::c_ulong,
                  400 as libc::c_int as UDWORD, 15 as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psStructHeap,
                  ::std::mem::size_of::<STRUCTURE>() as libc::c_ulong,
                  200 as libc::c_int as UDWORD, 15 as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psStructFuncHeap,
                  ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong,
                  50 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psFeatureHeap,
                  ::std::mem::size_of::<FEATURE>() as libc::c_ulong,
                  145 as libc::c_int as UDWORD, 15 as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psFlagPosHeap,
                  ::std::mem::size_of::<FLAG_POSITION>() as libc::c_ulong,
                  20 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    if heapCreate(&mut psTemplateHeap,
                  ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong,
                  120 as libc::c_int as UDWORD, 10 as libc::c_int as UDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    // reset the object ID number
    objID = 20000 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* Release the object heaps */
#[no_mangle]
pub unsafe extern "C" fn objmemShutdown() {
    heapDestroy(psDroidHeap);
    heapDestroy(psStructHeap);
    heapDestroy(psStructFuncHeap);
    heapDestroy(psFeatureHeap);
    heapDestroy(psFlagPosHeap);
    heapDestroy(psTemplateHeap);
}
/* General housekeeping for the object system */
#[no_mangle]
pub unsafe extern "C" fn objmemUpdate() {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psNext: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psPrev: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    // tell the script system about any destroyed objects
    if !psDestroyedObj.is_null() { scrvUpdateBasePointers(); }
    /* Go through the destroyed objects list looking for objects that
	   were destroyed before this turn */
    /* First remove the objects from the start of the list */
    while !psDestroyedObj.is_null() && (*psDestroyedObj).died != gameTime {
        psNext = (*psDestroyedObj).psNext;
        match (*psDestroyedObj).type_0 as libc::c_uint {
            0 => {
                droidRelease(psDestroyedObj as *mut DROID);
                heapFree(psDroidHeap, psDestroyedObj as *mut libc::c_void);
            }
            1 => {
                structureRelease(psDestroyedObj as *mut STRUCTURE);
                heapFree(psStructHeap, psDestroyedObj as *mut libc::c_void);
            }
            2 => {
                featureRelease(psDestroyedObj as *mut FEATURE);
                heapFree(psFeatureHeap, psDestroyedObj as *mut libc::c_void);
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"objmemUpdate: unknown object type in destroyed list\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"objmem.c\x00" as *const u8 as *const libc::c_char,
                          257 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 13],
                                                    &[libc::c_char; 13]>(b"objmemUpdate\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        psDestroyedObj = psNext
    }
    /* Now see if there are any further down the list
	Keep track of the previous object to set its Next pointer*/
    psPrev = psDestroyedObj;
    psCurr = psPrev;
    while !psCurr.is_null() {
        psNext = (*psCurr).psNext;
        if (*psCurr).died != gameTime {
            match (*psCurr).type_0 as libc::c_uint {
                0 => {
                    droidRelease(psCurr as *mut DROID);
                    heapFree(psDroidHeap, psCurr as *mut libc::c_void);
                }
                1 => {
                    structureRelease(psCurr as *mut STRUCTURE);
                    heapFree(psStructHeap, psCurr as *mut libc::c_void);
                }
                2 => {
                    featureRelease(psDestroyedObj as *mut FEATURE);
                    heapFree(psFeatureHeap, psCurr as *mut libc::c_void);
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"objmemUpdate: unknown object type in destroyed list\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"objmem.c\x00" as *const u8 as
                                  *const libc::c_char, 286 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 13],
                                                        &[libc::c_char; 13]>(b"objmemUpdate\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            /*set the linked list up - you will never be deleting the top
			of the list, so don't have to check*/
            (*psPrev).psNext = psNext
        } else {
            // do the object died callback
            psCBObjDestroyed = psCurr;
            eventFireCallbackTrigger(CALL_OBJ_DESTROYED as libc::c_int as
                                         TRIGGER_TYPE);
            match (*psCurr).type_0 as libc::c_uint {
                0 => {
                    eventFireCallbackTrigger(CALL_DROID_DESTROYED as
                                                 libc::c_int as TRIGGER_TYPE);
                }
                1 => {
                    eventFireCallbackTrigger(CALL_STRUCT_DESTROYED as
                                                 libc::c_int as TRIGGER_TYPE);
                }
                2 => {
                    eventFireCallbackTrigger(CALL_FEATURE_DESTROYED as
                                                 libc::c_int as TRIGGER_TYPE);
                }
                _ => { }
            }
            psCBObjDestroyed = 0 as *mut BASE_OBJECT;
            psPrev = psCurr
        }
        psCurr = psNext
    };
}
/* *************************************************************************************
 *
 * Macros for the object memory functions
 * The code is the same for the different object types only the pointer types
 * change.
 */
/* Creating a new object
 * new is a pointer to a pointer to the new object
 * type is the type of the object
 */
// ajl modified for netplaying..
/* Add the object to its list
 * list is a pointer to the object list
 */
/* Move an object from the active list to the destroyed list.
 * list is a pointer to the object list
 * del is a pointer to the object to remove
 * type is the type of the object
 */
// turn off the list integrity check for all builds
//#ifdef DEBUG
/* Remove an object from the active list
 * list is a pointer to the object list
 * remove is a pointer to the object to remove
 * type is the type of the object
 */
/* **************************************************************************************
 *
 * The actual object memory management functions for the different object types
 */
/* **************************  DROID  *********************************/
/* Create a new droid */
#[no_mangle]
pub unsafe extern "C" fn createDroid(mut player: UDWORD,
                                     mut ppsNew: *mut *mut DROID) -> BOOL {
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"addObject: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              470 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"createDroid\x00")).as_ptr(),
              b"player<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if heapAlloc(psDroidHeap,
                 ppsNew as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    (**ppsNew).type_0 = OBJ_DROID;
    (**ppsNew).id = objID << 3 as libc::c_int | player;
    objID = objID.wrapping_add(1);
    (**ppsNew).player = player as UBYTE;
    (**ppsNew).died = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* add the droid to the Droid Lists */
#[no_mangle]
pub unsafe extern "C" fn addDroid(mut psDroidToAdd: *mut DROID,
                                  mut pList: *mut *mut DROID) {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"addObject: Invalid DROID pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              478 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 9],
                                        &[libc::c_char; 9]>(b"addDroid\x00")).as_ptr(),
              b"PTRVALID((psDroidToAdd), sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroidToAdd).psNext = *pList.offset((*psDroidToAdd).player as isize);
    let ref mut fresh0 = *pList.offset((*psDroidToAdd).player as isize);
    *fresh0 = psDroidToAdd;
    /*whenever a droid gets added to a list other than the current list
     its died flag is set to NOT_CURRENT_LIST so that anything targetting
     it will cancel itself - HACK?!*/
    if *pList.offset((*psDroidToAdd).player as isize) ==
           apsDroidLists[(*psDroidToAdd).player as usize] {
        (*psDroidToAdd).died = 0 as libc::c_int as UDWORD;
        // commanders have to get their group back
        if (*psDroidToAdd).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
            grpCreate(&mut psGroup);
            if !psGroup.is_null() { grpJoin(psGroup, psDroidToAdd); }
        }
    };
}
/*destroy a droid */
#[no_mangle]
pub unsafe extern "C" fn killDroid(mut psDel: *mut DROID) {
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"killUnit: pointer is not a unit\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              502 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"killDroid\x00")).as_ptr(),
              b"psDel->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDel).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"killUnit: invalid player for unit\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDel).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              504 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"killDroid\x00")).as_ptr(),
              b"psDel->player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyObject: Invalid DROID pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              505 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 10],
                                        &[libc::c_char; 10]>(b"killDroid\x00")).as_ptr(),
              b"PTRVALID((psDel), sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if apsDroidLists[(*psDel).player as usize] == psDel {
        apsDroidLists[(*psDel).player as usize] =
            (*apsDroidLists[(*psDel).player as usize]).psNext;
        let ref mut fresh1 = (*(psDel as *mut BASE_OBJECT)).psNext;
        *fresh1 = psDestroyedObj;
        psDestroyedObj = psDel as *mut BASE_OBJECT;
        (*psDel).died = gameTime
    } else {
        let mut psPrev: *mut DROID = 0 as *mut DROID;
        let mut psCurr: *mut DROID = 0 as *mut DROID;
        psCurr = apsDroidLists[(*psDel).player as usize];
        while psCurr != psDel && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"destroyObject:DROID object not found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  505 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"killDroid\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            let ref mut fresh2 = (*(psDel as *mut BASE_OBJECT)).psNext;
            *fresh2 = psDestroyedObj;
            psDestroyedObj = psDel as *mut BASE_OBJECT;
            (*psDel).died = gameTime
        }
    };
}
/* Remove all droids */
#[no_mangle]
pub unsafe extern "C" fn freeAllDroids() {
    let mut i: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsDroidLists[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            droidRelease(psCurr);
            heapFree(psDroidHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        apsDroidLists[i as usize] = 0 as *mut DROID;
        i = i.wrapping_add(1)
    };
}
/*Remove a single Droid from a list*/
#[no_mangle]
pub unsafe extern "C" fn removeDroid(mut psDroidToRemove: *mut DROID,
                                     mut pList: *mut *mut DROID) {
    if (*psDroidToRemove).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"removeUnit: pointer is not a unit\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroidToRemove).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              518 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"removeDroid\x00")).as_ptr(),
              b"psDroidToRemove->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroidToRemove).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"removeUnit: invalid player for unit\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroidToRemove).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              520 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"removeDroid\x00")).as_ptr(),
              b"psDroidToRemove->player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"removeObject: Invalid DROID pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              521 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"removeDroid\x00")).as_ptr(),
              b"PTRVALID((psDroidToRemove), sizeof(DROID))\x00" as *const u8
                  as *const libc::c_char);
    };
    if *pList.offset((*psDroidToRemove).player as isize) == psDroidToRemove {
        let ref mut fresh3 =
            *pList.offset((*psDroidToRemove).player as isize);
        *fresh3 = (**pList.offset((*psDroidToRemove).player as isize)).psNext
    } else {
        let mut psPrev: *mut DROID = 0 as *mut DROID;
        let mut psCurr: *mut DROID = 0 as *mut DROID;
        psCurr = *pList.offset((*psDroidToRemove).player as isize);
        while psCurr != psDroidToRemove && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"removeObject:DROID object not found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  521 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"removeDroid\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() { (*psPrev).psNext = (*psCurr).psNext }
    }
    /*whenever a droid is removed from the current list its died
    flag is set to NOT_CURRENT_LIST so that anything targetting
    it will cancel itself - HACK?!*/
    if *pList.offset((*psDroidToRemove).player as isize) ==
           apsDroidLists[(*psDroidToRemove).player as usize] {
        (*psDroidToRemove).died = 1 as libc::c_int as UDWORD
    };
}
/*Removes all droids that may be stored in the mission lists*/
#[no_mangle]
pub unsafe extern "C" fn freeAllMissionDroids() {
    let mut i: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = mission.apsDroidLists[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            droidRelease(psCurr);
            heapFree(psDroidHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        mission.apsDroidLists[i as usize] = 0 as *mut DROID;
        i = i.wrapping_add(1)
    };
}
/*Removes all droids that may be stored in the limbo lists*/
#[no_mangle]
pub unsafe extern "C" fn freeAllLimboDroids() {
    let mut i: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsLimboDroids[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            droidRelease(psCurr);
            heapFree(psDroidHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        apsLimboDroids[i as usize] = 0 as *mut DROID;
        i = i.wrapping_add(1)
    };
}
/* *************************  STRUCTURE  *******************************/
/* Create a new structure */
#[no_mangle]
pub unsafe extern "C" fn createStruct(mut player: UDWORD,
                                      mut ppsNew: *mut *mut STRUCTURE)
 -> BOOL {
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"addObject: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              549 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"createStruct\x00")).as_ptr(),
              b"player<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if heapAlloc(psStructHeap,
                 ppsNew as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    (**ppsNew).type_0 = OBJ_STRUCTURE;
    (**ppsNew).id = objID << 3 as libc::c_int | player;
    objID = objID.wrapping_add(1);
    (**ppsNew).player = player as UBYTE;
    (**ppsNew).died = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* add the structure to the Structure Lists */
#[no_mangle]
pub unsafe extern "C" fn addStructure(mut psStructToAdd: *mut STRUCTURE) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"addObject: Invalid STRUCTURE pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              555 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"addStructure\x00")).as_ptr(),
              b"PTRVALID((psStructToAdd), sizeof(STRUCTURE))\x00" as *const u8
                  as *const libc::c_char);
    };
    (*psStructToAdd).psNext =
        apsStructLists[(*psStructToAdd).player as usize];
    apsStructLists[(*psStructToAdd).player as usize] = psStructToAdd;
}
/* Destroy a structure */
#[no_mangle]
pub unsafe extern "C" fn killStruct(mut psDel: *mut STRUCTURE) {
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"killStruct: pointer is not a droid\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              562 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"killStruct\x00")).as_ptr(),
              b"psDel->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDel).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"killStruct: invalid player for stucture\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDel).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              564 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"killStruct\x00")).as_ptr(),
              b"psDel->player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyObject: Invalid STRUCTURE pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              565 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"killStruct\x00")).as_ptr(),
              b"PTRVALID((psDel), sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if apsStructLists[(*psDel).player as usize] == psDel {
        apsStructLists[(*psDel).player as usize] =
            (*apsStructLists[(*psDel).player as usize]).psNext;
        let ref mut fresh4 = (*(psDel as *mut BASE_OBJECT)).psNext;
        *fresh4 = psDestroyedObj;
        psDestroyedObj = psDel as *mut BASE_OBJECT;
        (*psDel).died = gameTime
    } else {
        let mut psPrev: *mut STRUCTURE = 0 as *mut STRUCTURE;
        let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
        psCurr = apsStructLists[(*psDel).player as usize];
        while psCurr != psDel && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"destroyObject:STRUCTURE object not found\x00" as *const u8
                      as *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  565 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"killStruct\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            let ref mut fresh5 = (*(psDel as *mut BASE_OBJECT)).psNext;
            *fresh5 = psDestroyedObj;
            psDestroyedObj = psDel as *mut BASE_OBJECT;
            (*psDel).died = gameTime
        }
    };
}
/* Remove heapall structures */
#[no_mangle]
pub unsafe extern "C" fn freeAllStructs() {
    let mut i: UDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNext: *mut STRUCTURE = 0 as *mut STRUCTURE;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsStructLists[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            structureRelease(psCurr);
            heapFree(psStructHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        apsStructLists[i as usize] = 0 as *mut STRUCTURE;
        i = i.wrapping_add(1)
    };
}
/*Remove a single Structure from a list*/
#[no_mangle]
pub unsafe extern "C" fn removeStructureFromList(mut psStructToRemove:
                                                     *mut STRUCTURE,
                                                 mut pList:
                                                     *mut *mut STRUCTURE) {
    if (*psStructToRemove).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"removeStructureFromList: pointer is not a structure\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStructToRemove).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              578 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"removeStructureFromList\x00")).as_ptr(),
              b"psStructToRemove->type == OBJ_STRUCTURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psStructToRemove).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"removeStructureFromList: invalid player for structure\x00" as
                  *const u8 as *const libc::c_char);
    };
    if ((*psStructToRemove).player as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              580 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"removeStructureFromList\x00")).as_ptr(),
              b"psStructToRemove->player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"removeObject: Invalid STRUCTURE pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              581 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"removeStructureFromList\x00")).as_ptr(),
              b"PTRVALID((psStructToRemove), sizeof(STRUCTURE))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if *pList.offset((*psStructToRemove).player as isize) == psStructToRemove
       {
        let ref mut fresh6 =
            *pList.offset((*psStructToRemove).player as isize);
        *fresh6 = (**pList.offset((*psStructToRemove).player as isize)).psNext
    } else {
        let mut psPrev: *mut STRUCTURE = 0 as *mut STRUCTURE;
        let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
        psCurr = *pList.offset((*psStructToRemove).player as isize);
        while psCurr != psStructToRemove && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"removeObject:STRUCTURE object not found\x00" as *const u8
                      as *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  581 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"removeStructureFromList\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() { (*psPrev).psNext = (*psCurr).psNext }
    };
}
/* *************************  FEATURE  *********************************/
/* Create a new Feature */
#[no_mangle]
pub unsafe extern "C" fn createFeature(mut ppsNew: *mut *mut FEATURE)
 -> BOOL {
    if (0 as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"addObject: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (0 as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              589 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"createFeature\x00")).as_ptr(),
              b"0<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if heapAlloc(psFeatureHeap,
                 ppsNew as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    (**ppsNew).type_0 = OBJ_FEATURE;
    (**ppsNew).id =
        objID << 3 as libc::c_int | 0 as libc::c_int as libc::c_uint;
    objID = objID.wrapping_add(1);
    (**ppsNew).player = 0 as libc::c_int as UBYTE;
    (**ppsNew).died = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* add the feature to the Feature Lists */
#[no_mangle]
pub unsafe extern "C" fn addFeature(mut psFeatureToAdd: *mut FEATURE) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"addObject: Invalid FEATURE pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              595 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"addFeature\x00")).as_ptr(),
              b"PTRVALID((psFeatureToAdd), sizeof(FEATURE))\x00" as *const u8
                  as *const libc::c_char);
    };
    (*psFeatureToAdd).psNext =
        apsFeatureLists[(*psFeatureToAdd).player as usize];
    apsFeatureLists[(*psFeatureToAdd).player as usize] = psFeatureToAdd;
}
/* Destroy a feature */
// set the player to 0 since features have player = maxplayers+1. This screws up DESTROY
// it's a bit of a hack, but hey, it works
#[no_mangle]
pub unsafe extern "C" fn killFeature(mut psDel: *mut FEATURE) {
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"killFeature: pointer is not a feature\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDel).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              604 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"killFeature\x00")).as_ptr(),
              b"psDel->type == OBJ_FEATURE\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDel).player = 0 as libc::c_int as UBYTE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyObject: Invalid FEATURE pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              606 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"killFeature\x00")).as_ptr(),
              b"PTRVALID((psDel), sizeof(FEATURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if apsFeatureLists[(*psDel).player as usize] == psDel {
        apsFeatureLists[(*psDel).player as usize] =
            (*apsFeatureLists[(*psDel).player as usize]).psNext;
        let ref mut fresh7 = (*(psDel as *mut BASE_OBJECT)).psNext;
        *fresh7 = psDestroyedObj;
        psDestroyedObj = psDel as *mut BASE_OBJECT;
        (*psDel).died = gameTime
    } else {
        let mut psPrev: *mut FEATURE = 0 as *mut FEATURE;
        let mut psCurr: *mut FEATURE = 0 as *mut FEATURE;
        psCurr = apsFeatureLists[(*psDel).player as usize];
        while psCurr != psDel && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"destroyObject:FEATURE object not found\x00" as *const u8
                      as *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  606 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"killFeature\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            let ref mut fresh8 = (*(psDel as *mut BASE_OBJECT)).psNext;
            *fresh8 = psDestroyedObj;
            psDestroyedObj = psDel as *mut BASE_OBJECT;
            (*psDel).died = gameTime
        }
    };
}
/* Remove all features */
#[no_mangle]
pub unsafe extern "C" fn freeAllFeatures() {
    let mut i: UDWORD = 0;
    let mut psCurr: *mut FEATURE = 0 as *mut FEATURE;
    let mut psNext: *mut FEATURE = 0 as *mut FEATURE;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psCurr = apsFeatureLists[i as usize];
        while !psCurr.is_null() {
            psNext = (*psCurr).psNext;
            featureRelease(psCurr);
            heapFree(psFeatureHeap, psCurr as *mut libc::c_void);
            psCurr = psNext
        }
        apsFeatureLists[i as usize] = 0 as *mut FEATURE;
        i = i.wrapping_add(1)
    };
}
/* *************************  FLAG_POSITION ********************************/
/* Create a new Flag Position */
#[no_mangle]
pub unsafe extern "C" fn createFlagPosition(mut ppsNew:
                                                *mut *mut FLAG_POSITION,
                                            mut player: UDWORD) -> BOOL {
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"createFlagPosition: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              620 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"createFlagPosition\x00")).as_ptr(),
              b"player<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if heapAlloc(psFlagPosHeap,
                 ppsNew as *mut libc::c_void as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    (**ppsNew).type_0 = POS_DELIVERY;
    (**ppsNew).player = player;
    (**ppsNew).frameNumber = 0 as libc::c_int as UDWORD;
    (**ppsNew).selected = 0 as libc::c_int;
    return 1 as libc::c_int;
}
/* add the Flag Position to the Flag Position Lists */
#[no_mangle]
pub unsafe extern "C" fn addFlagPosition(mut psFlagPosToAdd:
                                             *mut FLAG_POSITION) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"addFlagPosition: Invalid FlagPosition pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              637 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"addFlagPosition\x00")).as_ptr(),
              b"PTRVALID((psFlagPosToAdd), sizeof(FLAG_POSITION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    (*psFlagPosToAdd).psNext =
        apsFlagPosLists[(*psFlagPosToAdd).player as usize];
    apsFlagPosLists[(*psFlagPosToAdd).player as usize] = psFlagPosToAdd;
}
/* Remove a Flag Position from the Lists */
#[no_mangle]
pub unsafe extern "C" fn removeFlagPosition(mut psDel: *mut FLAG_POSITION) {
    let mut psPrev: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut psCurr: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"removeFlagPosition: Invalid Flag Positionpointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              649 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"removeFlagPosition\x00")).as_ptr(),
              b"PTRVALID((psDel), sizeof(FLAG_POSITION))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if apsFlagPosLists[(*psDel).player as usize] == psDel {
        apsFlagPosLists[(*psDel).player as usize] =
            (*apsFlagPosLists[(*psDel).player as usize]).psNext;
        heapFree(psFlagPosHeap, psDel as *mut libc::c_void);
    } else {
        psCurr = apsFlagPosLists[(*psDel).player as usize];
        while psCurr != psDel && !psCurr.is_null() {
            psPrev = psCurr;
            psCurr = (*psCurr).psNext
        }
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"removeFlagPosition:object not found\x00" as *const u8 as
                      *const libc::c_char);
        };
        if !psCurr.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  664 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"removeFlagPosition\x00")).as_ptr(),
                  b"psCurr != NULL\x00" as *const u8 as *const libc::c_char);
        };
        if !psCurr.is_null() {
            (*psPrev).psNext = (*psCurr).psNext;
            heapFree(psFlagPosHeap, psCurr as *mut libc::c_void);
        }
    };
}
// free all flag positions
#[no_mangle]
pub unsafe extern "C" fn freeAllFlagPositions() {
    let mut psNext: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut player: SDWORD = 0;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        while !apsFlagPosLists[player as usize].is_null() {
            psNext = (*apsFlagPosLists[player as usize]).psNext;
            heapFree(psFlagPosHeap,
                     apsFlagPosLists[player as usize] as *mut libc::c_void);
            apsFlagPosLists[player as usize] = psNext
        }
        player += 1
    };
}
/* *************************  STRUC FUNCTIONALITY ********************************/
/* Create a new Structure Functionality*/
#[no_mangle]
pub unsafe extern "C" fn createStructFunc(mut ppsNew: *mut *mut FUNCTIONALITY)
 -> BOOL {
    if heapAlloc(psStructFuncHeap, ppsNew as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*remove a structure Functionality from the heap*/
#[no_mangle]
pub unsafe extern "C" fn removeStructFunc(mut psDel: *mut FUNCTIONALITY) {
    heapFree(psStructFuncHeap, psDel as *mut libc::c_void);
}
/* *************************  OBJECT ACCESS FUNCTIONALITY ********************************/
// Find a base object from it's id
//this function is similar to BOOL scrvGetBaseObj(UDWORD id, BASE_OBJECT **ppsObj)
#[no_mangle]
pub unsafe extern "C" fn getBaseObjFromId(mut id: UDWORD)
 -> *mut BASE_OBJECT {
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psTrans: *mut DROID = 0 as *mut DROID;
    i = 0 as libc::c_int as UDWORD;
    while i < 7 as libc::c_int as libc::c_uint {
        player = 0 as libc::c_int as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            match i {
                0 => {
                    psObj = apsDroidLists[player as usize] as *mut BASE_OBJECT
                }
                1 => {
                    psObj =
                        apsStructLists[player as usize] as *mut BASE_OBJECT
                }
                2 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            apsFeatureLists[0 as libc::c_int as usize] as
                                *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                3 => {
                    psObj =
                        mission.apsDroidLists[player as usize] as
                            *mut BASE_OBJECT
                }
                4 => {
                    psObj =
                        mission.apsStructLists[player as usize] as
                            *mut BASE_OBJECT
                }
                5 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            mission.apsFeatureLists[0 as libc::c_int as usize]
                                as *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                6 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            apsLimboDroids[0 as libc::c_int as usize] as
                                *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                _ => { psObj = 0 as *mut BASE_OBJECT }
            }
            while !psObj.is_null() {
                if (*psObj).id == id { return psObj }
                // if transporter check any droids in the grp
                if (*psObj).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint &&
                       (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    psTrans = (*(*(psObj as *mut DROID)).psGroup).psList;
                    while !psTrans.is_null() {
                        if (*psTrans).id == id {
                            return psTrans as *mut BASE_OBJECT
                        }
                        psTrans = (*psTrans).psGrpNext
                    }
                }
                psObj = (*psObj).psNext
            }
            player = player.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"getBaseObjFromId() failed for id %d\x00" as *const u8 as
                  *const libc::c_char, id);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              835 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"getBaseObjFromId\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as *mut BASE_OBJECT;
}
#[no_mangle]
pub unsafe extern "C" fn getRepairIdFromFlag(mut psFlag: *mut FLAG_POSITION)
 -> UDWORD {
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psObj: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    player = (*psFlag).player;
    //probably dont need to check mission list
    i = 0 as libc::c_int as UDWORD;
    while i < 2 as libc::c_int as libc::c_uint {
        match i {
            0 => { psObj = apsStructLists[player as usize] }
            1 => { psObj = mission.apsStructLists[player as usize] }
            _ => { psObj = 0 as *mut STRUCTURE }
        }
        while !psObj.is_null() {
            if !(*psObj).pFunctionality.is_null() {
                if (*(*psObj).pStructureType).type_0 ==
                       REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
                    //check for matching delivery point
                    psRepair =
                        (*psObj).pFunctionality as *mut REPAIR_FACILITY;
                    if (*psRepair).psDeliveryPoint == psFlag {
                        return (*psObj).id
                    }
                }
            }
            psObj = (*psObj).psNext
        }
        i = i.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"getRepairIdFromFlag() failed\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              882 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"getRepairIdFromFlag\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0xffffffff as libc::c_uint;
}
/*
 * ObjMem.h
 *
 * Routines for managing object's memory
 *
 */
//the died flag for a droid is set to this when it gets added to the non-current list
/* The memory heaps for the different object types */
// the memory heap for templates
/* The lists of objects allocated */
/* The list of destroyed objects */
/* Initialise the object heaps */
/* Release the object heaps */
/* General housekeeping for the object system */
/* Create a new droid */
/* add the droid to the Droid Lists */
/*destroy a droid */
/* Remove all droids */
/*Remove a single Droid from its list*/
/*Removes all droids that may be stored in the mission lists*/
/*Removes all droids that may be stored in the limbo lists*/
/* Create a new structure */
/* add the structure to the Structure Lists */
/* Destroy a structure */
/* Remove all structures */
/*Remove a single Structure from a list*/
/* Create a new Feature */
/* add the feature to the Feature Lists */
/* Destroy a feature */
/* Remove all features */
/* Create a new Flag Position */
/* add the Flag Position to the Flag Position Lists */
/* Remove a Flag Position from the Lists */
// free all flag positions
/* Create a new Structure Functionality*/
/*remove a structure Functionality from the heap*/
// Find a base object from it's id
// check a base object exists for an ID
#[no_mangle]
pub unsafe extern "C" fn checkValidId(mut id: UDWORD) -> BOOL {
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psTrans: *mut DROID = 0 as *mut DROID;
    i = 0 as libc::c_int as UDWORD;
    while i < 7 as libc::c_int as libc::c_uint {
        player = 0 as libc::c_int as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            match i {
                0 => {
                    psObj = apsDroidLists[player as usize] as *mut BASE_OBJECT
                }
                1 => {
                    psObj =
                        apsStructLists[player as usize] as *mut BASE_OBJECT
                }
                2 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            apsFeatureLists[0 as libc::c_int as usize] as
                                *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                3 => {
                    psObj =
                        mission.apsDroidLists[player as usize] as
                            *mut BASE_OBJECT
                }
                4 => {
                    psObj =
                        mission.apsStructLists[player as usize] as
                            *mut BASE_OBJECT
                }
                5 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            mission.apsFeatureLists[0 as libc::c_int as usize]
                                as *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                6 => {
                    if player == 0 as libc::c_int as libc::c_uint {
                        psObj =
                            apsLimboDroids[0 as libc::c_int as usize] as
                                *mut BASE_OBJECT
                    } else { psObj = 0 as *mut BASE_OBJECT }
                }
                _ => { psObj = 0 as *mut BASE_OBJECT }
            }
            while !psObj.is_null() {
                if (*psObj).id == id { return 1 as libc::c_int }
                // if transporter check any droids in the grp
                if (*psObj).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint &&
                       (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    psTrans = (*(*(psObj as *mut DROID)).psGroup).psList;
                    while !psTrans.is_null() {
                        if (*psTrans).id == id { return 1 as libc::c_int }
                        psTrans = (*psTrans).psGrpNext
                    }
                }
                psObj = (*psObj).psNext
            }
            player = player.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"checkValidId() failed for id %d\x00" as *const u8 as
                  *const libc::c_char, id);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"objmem.c\x00" as *const u8 as *const libc::c_char,
              969 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"checkValidId\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
    return 0 as libc::c_int;
}
// integrity check the lists
#[no_mangle]
pub unsafe extern "C" fn objListIntegCheck() {
    let mut player: SDWORD = 0;
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        psCurr = apsDroidLists[player as usize] as *mut BASE_OBJECT;
        while !psCurr.is_null() {
            if (*psCurr).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*psCurr).player as SDWORD == player {
            } else {
                debug(LOG_ERROR,
                      b"objListIntegCheck: misplaced object in the droid list for player %d\x00"
                          as *const u8 as *const libc::c_char, player);
            };
            if (*psCurr).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*psCurr).player as SDWORD == player {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"objmem.c\x00" as *const u8 as *const libc::c_char,
                      988 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"objListIntegCheck\x00")).as_ptr(),
                      b"psCurr->type == OBJ_DROID && (SDWORD)psCurr->player == player\x00"
                          as *const u8 as *const libc::c_char);
            };
            psCurr = (*psCurr).psNext
        }
        player += 1 as libc::c_int
    }
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        psCurr = apsStructLists[player as usize] as *mut BASE_OBJECT;
        while !psCurr.is_null() {
            if (*psCurr).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                   (*psCurr).player as SDWORD == player {
            } else {
                debug(LOG_ERROR,
                      b"objListIntegCheck: misplaced object in the structure list for player %d\x00"
                          as *const u8 as *const libc::c_char, player);
            };
            if (*psCurr).type_0 as libc::c_uint ==
                   OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                   (*psCurr).player as SDWORD == player {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"objmem.c\x00" as *const u8 as *const libc::c_char,
                      998 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"objListIntegCheck\x00")).as_ptr(),
                      b"psCurr->type == OBJ_STRUCTURE && (SDWORD)psCurr->player == player\x00"
                          as *const u8 as *const libc::c_char);
            };
            psCurr = (*psCurr).psNext
        }
        player += 1 as libc::c_int
    }
    psCurr = apsFeatureLists[0 as libc::c_int as usize] as *mut BASE_OBJECT;
    while !psCurr.is_null() {
        if (*psCurr).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"objListIntegCheck: misplaced object in the feature list\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psCurr).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"objmem.c\x00" as *const u8 as *const libc::c_char,
                  1004 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"objListIntegCheck\x00")).as_ptr(),
                  b"psCurr->type == OBJ_FEATURE\x00" as *const u8 as
                      *const libc::c_char);
        };
        psCurr = (*psCurr).psNext
    };
}
