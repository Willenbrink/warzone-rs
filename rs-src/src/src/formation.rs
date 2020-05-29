use ::libc;
extern "C" {
    pub type _droid_group;
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
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Lookup trig functions */
    #[no_mangle]
    fn trigSin(angle: SDWORD) -> FRACT;
    #[no_mangle]
    fn trigCos(angle: SDWORD) -> FRACT;
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
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    // Check los between two tiles
    #[no_mangle]
    fn fpathTileLOS(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> BOOL;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
    #[no_mangle]
    fn adjustDirection(present: SDWORD, difference: SDWORD) -> UDWORD;
}
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
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
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
//works as all of the above together! - new for updates - added 11/06/99 AB
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
pub type BASE_OBJECT = _base_object;
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
// information about a formation
pub type FORMATION = _formation;
pub type _formation_type = libc::c_uint;
pub const FT_COLUMN: _formation_type = 1;
pub const FT_LINE: _formation_type = 0;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
/*
 * Formation.h
 *
 * Control units moving in formation.
 *
 */
pub type FORMATION_TYPE = _formation_type;
/*
 * Formation.c
 *
 * Control units moving in formation.
 *
 */
//#define DEBUG_GROUP0
// radius for the different body sizes
static mut fmLtRad: SDWORD = 80 as libc::c_int;
static mut fmMedRad: SDWORD = 100 as libc::c_int;
static mut fmHvyRad: SDWORD = 110 as libc::c_int;
// The heap of formations
#[no_mangle]
pub static mut psFHeap: *mut OBJ_HEAP = 0 as *const OBJ_HEAP as *mut OBJ_HEAP;
// The list of allocated formations
#[no_mangle]
pub static mut psFormationList: *mut FORMATION =
    0 as *const FORMATION as *mut FORMATION;
// Initialise the formation system
// Initialise the formation system
#[no_mangle]
pub unsafe extern "C" fn formationInitialise() -> BOOL {
    if heapCreate(&mut psFHeap,
                  ::std::mem::size_of::<FORMATION>() as libc::c_ulong,
                  10 as libc::c_int as UDWORD, 5 as libc::c_int as UDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    psFormationList = 0 as *mut FORMATION;
    return 1 as libc::c_int;
}
// Shutdown the formation system
// Shutdown the formation system
#[no_mangle]
pub unsafe extern "C" fn formationShutDown() {
    let mut psNext: *mut FORMATION = 0 as *mut FORMATION;
    while !psFormationList.is_null() {
        debug(LOG_NEVER,
              b"formation with %d units still attached\n\x00" as *const u8 as
                  *const libc::c_char,
              (*psFormationList).refCount as libc::c_int);
        psNext = (*psFormationList).psNext;
        heapFree(psFHeap, psFormationList as *mut libc::c_void);
        psFormationList = psNext
    }
    heapDestroy(psFHeap);
}
// Create a new formation
// Create a new formation
#[no_mangle]
pub unsafe extern "C" fn formationNew(mut ppsFormation: *mut *mut FORMATION,
                                      mut type_0: FORMATION_TYPE,
                                      mut x: SDWORD, mut y: SDWORD,
                                      mut dir: SDWORD) -> BOOL {
    let mut psNew: *mut FORMATION = 0 as *mut FORMATION;
    let mut i: SDWORD = 0;
    // get a heap structure
    if heapAlloc(psFHeap,
                 &mut psNew as *mut *mut FORMATION as *mut libc::c_void as
                     *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    // initialise it
    (*psNew).refCount = 0 as libc::c_int as SWORD;
    (*psNew).size = (4 as libc::c_int * fmLtRad) as SWORD;
    (*psNew).rankDist = (2 as libc::c_int * fmLtRad) as SWORD;
    (*psNew).dir = dir as SWORD;
    (*psNew).x = x;
    (*psNew).y = y;
    (*psNew).free = 0 as libc::c_int as SBYTE;
    (*psNew).iSpeed = 100000 as libc::c_long as UDWORD;
    memset((*psNew).asMembers.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[F_MEMBER; 20]>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        (*psNew).asMembers[i as usize].next = (i + 1 as libc::c_int) as SBYTE;
        i += 1
    }
    (*psNew).asMembers[(20 as libc::c_int - 1 as libc::c_int) as usize].next =
        -(1 as libc::c_int) as SBYTE;
    // add the formation lines based on the formation type
    match type_0 as libc::c_uint {
        0 => {
            (*psNew).numLines = 2 as libc::c_int as SWORD;
            // line to the left
            (*psNew).asLines[0 as libc::c_int as usize].xoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].yoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].dir =
                adjustDirection(dir, -(110 as libc::c_int)) as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].member =
                -(1 as libc::c_int) as SBYTE;
            // line to the right
            (*psNew).asLines[1 as libc::c_int as usize].xoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[1 as libc::c_int as usize].yoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[1 as libc::c_int as usize].dir =
                adjustDirection(dir, 110 as libc::c_int) as SWORD;
            (*psNew).asLines[1 as libc::c_int as usize].member =
                -(1 as libc::c_int) as SBYTE
        }
        1 => {
            (*psNew).numLines = 1 as libc::c_int as SWORD;
            // line to the left
            (*psNew).asLines[0 as libc::c_int as usize].xoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].yoffset =
                0 as libc::c_int as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].dir =
                adjustDirection(dir, 180 as libc::c_int) as SWORD;
            (*psNew).asLines[0 as libc::c_int as usize].member =
                -(1 as libc::c_int) as SBYTE
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"fmNewFormation: unknown formation type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"formation.c\x00" as *const u8 as *const libc::c_char,
                      167 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"formationNew\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    (*psNew).psNext = psFormationList;
    psFormationList = psNew;
    *ppsFormation = psNew;
    return 1 as libc::c_int;
}
// Try and find a formation near to a location
// Try and find a formation near to a location
#[no_mangle]
pub unsafe extern "C" fn formationFind(mut ppsFormation: *mut *mut FORMATION,
                                       mut x: SDWORD, mut y: SDWORD) -> BOOL {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut psCurr: *mut FORMATION = 0 as *mut FORMATION;
    psCurr = psFormationList;
    while !psCurr.is_null() {
        // see if the positioin is close to the formation
        xdiff = (*psCurr).x - x;
        ydiff = (*psCurr).y - y;
        distSq = xdiff * xdiff + ydiff * ydiff;
        if distSq <
               128 as libc::c_int / 2 as libc::c_int *
                   (128 as libc::c_int / 2 as libc::c_int) {
            break ;
        }
        psCurr = (*psCurr).psNext
    }
    *ppsFormation = psCurr;
    return (psCurr != 0 as *mut libc::c_void as *mut FORMATION) as
               libc::c_int;
}
// find formation speed (currently speed of slowest unit)
#[no_mangle]
pub unsafe extern "C" fn formationUpdateSpeed(mut psFormation: *mut FORMATION,
                                              mut psNew: *mut BASE_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut iUnit: SDWORD = 0;
    let mut asMembers: *mut F_MEMBER = (*psFormation).asMembers.as_mut_ptr();
    if !psNew.is_null() &&
           (*psNew).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psNew as *mut DROID;
        if (*psFormation).iSpeed > (*psDroid).baseSpeed {
            (*psFormation).iSpeed = (*psDroid).baseSpeed
        }
    } else { (*psFormation).iSpeed = 100000 as libc::c_long as UDWORD }
    iUnit = 0 as libc::c_int;
    while iUnit < 20 as libc::c_int {
        if !(*asMembers.offset(iUnit as isize)).psObj.is_null() &&
               (*(*asMembers.offset(iUnit as isize)).psObj).type_0 as
                   libc::c_uint == OBJ_DROID as libc::c_int as libc::c_uint {
            psDroid = (*asMembers.offset(iUnit as isize)).psObj as *mut DROID;
            if (*psFormation).iSpeed > (*psDroid).baseSpeed {
                (*psFormation).iSpeed = (*psDroid).baseSpeed
            }
        }
        iUnit += 1
    };
}
// Associate a unit with a formation
// Associate a unit with a formation
#[no_mangle]
pub unsafe extern "C" fn formationJoin(mut psFormation: *mut FORMATION,
                                       mut psObj: *mut BASE_OBJECT) {
    let mut rankDist: SDWORD = 0;
    let mut size: SDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formationJoin: invalid formation\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"formation.c\x00" as *const u8 as *const libc::c_char,
              243 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"formationJoin\x00")).as_ptr(),
              b"PTRVALID(psFormation, sizeof(FORMATION))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psFormation).refCount =
        ((*psFormation).refCount as libc::c_int + 1 as libc::c_int) as SWORD;
    rankDist = formationObjRadius(psObj) * 2 as libc::c_int;
    if ((*psFormation).rankDist as libc::c_int) < rankDist {
        (*psFormation).rankDist = rankDist as SWORD
    }
    size = formationObjRadius(psObj) * 4 as libc::c_int;
    if ((*psFormation).size as libc::c_int) < size {
        (*psFormation).size = size as SWORD
    }
    /* update formation speed */
    formationUpdateSpeed(psFormation, psObj);
}
// Remove a unit from a formation
// Remove a unit from a formation
#[no_mangle]
pub unsafe extern "C" fn formationLeave(mut psFormation: *mut FORMATION,
                                        mut psObj: *mut BASE_OBJECT) {
    let mut prev: SDWORD = 0;
    let mut curr: SDWORD = 0;
    let mut unit: SDWORD = 0;
    let mut line: SDWORD = 0;
    let mut asLines: *mut F_LINE = 0 as *mut F_LINE;
    let mut asMembers: *mut F_MEMBER = 0 as *mut F_MEMBER;
    let mut psCurr: *mut FORMATION = 0 as *mut FORMATION;
    let mut psPrev: *mut FORMATION = 0 as *mut FORMATION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formationLeave: invalid formation\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"formation.c\x00" as *const u8 as *const libc::c_char,
              274 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"formationLeave\x00")).as_ptr(),
              b"PTRVALID(psFormation, sizeof(FORMATION))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psFormation).refCount as libc::c_int > 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"formationLeave: refcount is zero\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psFormation).refCount as libc::c_int > 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"formation.c\x00" as *const u8 as *const libc::c_char,
              276 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"formationLeave\x00")).as_ptr(),
              b"psFormation->refCount > 0\x00" as *const u8 as
                  *const libc::c_char);
    };
    asMembers = (*psFormation).asMembers.as_mut_ptr();
    // see if the unit is a member
    unit = 0 as libc::c_int;
    while unit < 20 as libc::c_int {
        if (*asMembers.offset(unit as isize)).psObj == psObj { break ; }
        unit += 1
    }
    if unit < 20 as libc::c_int {
        // remove the member from the members list
        asLines = (*psFormation).asLines.as_mut_ptr();
        prev = -(1 as libc::c_int);
        line = (*asMembers.offset(unit as isize)).line as SDWORD;
        curr = (*asLines.offset(line as isize)).member as SDWORD;
        while curr != unit {
            prev = curr;
            curr = (*asMembers.offset(curr as isize)).next as SDWORD
        }
        if prev == -(1 as libc::c_int) {
            (*asLines.offset(line as isize)).member =
                (*asMembers.offset(unit as isize)).next
        } else {
            (*asMembers.offset(prev as isize)).next =
                (*asMembers.offset(unit as isize)).next
        }
        (*asMembers.offset(unit as isize)).next = (*psFormation).free;
        let ref mut fresh0 = (*asMembers.offset(unit as isize)).psObj;
        *fresh0 = 0 as *mut BASE_OBJECT;
        (*psFormation).free = unit as SBYTE;
        /* update formation speed */
        formationUpdateSpeed(psFormation, 0 as *mut BASE_OBJECT);
    }
    (*psFormation).refCount =
        ((*psFormation).refCount as libc::c_int - 1 as libc::c_int) as SWORD;
    if (*psFormation).refCount as libc::c_int == 0 as libc::c_int {
        if psFormation == psFormationList {
            psFormationList = (*psFormationList).psNext
        } else {
            psPrev = 0 as *mut FORMATION;
            psCurr = psFormationList;
            while !psCurr.is_null() && psCurr != psFormation {
                psPrev = psCurr;
                psCurr = (*psCurr).psNext
            }
            (*psPrev).psNext = (*psFormation).psNext
        }
        heapFree(psFHeap, psFormation as *mut libc::c_void);
    };
}
// remove all the members from a formation and release it
// remove all the members from a formation and release it
#[no_mangle]
pub unsafe extern "C" fn formationReset(mut psFormation: *mut FORMATION) {
    let mut i: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        psObj = (*psFormation).asMembers[i as usize].psObj;
        if !psObj.is_null() {
            formationLeave(psFormation,
                           (*psFormation).asMembers[i as usize].psObj);
            match (*psObj).type_0 as libc::c_uint {
                0 => {
                    let ref mut fresh1 =
                        (*(psObj as *mut DROID)).sMove.psFormation;
                    *fresh1 = 0 as *mut _formation
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"formationReset: unknown unit type\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"formation.c\x00" as *const u8 as
                                  *const libc::c_char, 360 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"formationReset\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
        }
        i += 1
    };
}
// calculate the coordinates of a position on a line
#[no_mangle]
pub unsafe extern "C" fn formationCalcPos(mut psFormation: *mut FORMATION,
                                          mut line: SDWORD, mut dist: SDWORD,
                                          mut pX: *mut SDWORD,
                                          mut pY: *mut SDWORD) {
    let mut dir: SDWORD = 0;
    let mut xoffset: SDWORD = 0;
    let mut yoffset: SDWORD = 0;
    let mut rank: SDWORD = 0;
    rank = dist / (*psFormation).size as libc::c_int;
    // calculate the offset of the line based on the rank
    dir =
        adjustDirection((*psFormation).dir as SDWORD, 180 as libc::c_int) as
            SDWORD;
    xoffset =
        (trigSin(dir) *
             ((*psFormation).rankDist as libc::c_int * rank) as FRACT) as
            SDWORD +
            (*psFormation).asLines[line as usize].xoffset as libc::c_int;
    yoffset =
        (trigCos(dir) *
             ((*psFormation).rankDist as libc::c_int * rank) as FRACT) as
            SDWORD +
            (*psFormation).asLines[line as usize].yoffset as libc::c_int;
    // calculate the position of the gap
    dir = (*psFormation).asLines[line as usize].dir as SDWORD;
    dist -= (*psFormation).size as libc::c_int * rank;
    *pX =
        (trigSin(dir) * dist as FRACT) as SDWORD + xoffset + (*psFormation).x;
    *pY =
        (trigCos(dir) * dist as FRACT) as SDWORD + yoffset + (*psFormation).y;
}
// assign a unit to a free spot in the formation
#[no_mangle]
pub unsafe extern "C" fn formationFindFree(mut psFormation: *mut FORMATION,
                                           mut psObj: *mut BASE_OBJECT,
                                           mut pX: *mut SDWORD,
                                           mut pY: *mut SDWORD) {
    let mut line: SDWORD = 0;
    let mut unit: SDWORD = 0;
    let mut objRadius: SDWORD = 0;
    let mut radius: SDWORD = 0;
    let mut currDist: SDWORD = 0;
    let mut prev: SDWORD = 0;
    let mut rank: SDWORD = 0;
    let mut asLines: *mut F_LINE = (*psFormation).asLines.as_mut_ptr();
    let mut asMembers: *mut F_MEMBER = (*psFormation).asMembers.as_mut_ptr();
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut objDist: SDWORD = 0;
    let mut chosenLine: SDWORD = 0;
    let mut chosenDist: SDWORD = 0;
    let mut chosenPrev: SDWORD = 0;
    let mut chosenRank: SDWORD = 0;
    let mut found: BOOL = 0;
    if (*psFormation).free as libc::c_int == -(1 as libc::c_int) {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"formationFindFree: no members left to allocate\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"formation.c\x00" as *const u8 as *const libc::c_char,
                  407 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"formationFindFree\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        *pX = (*psFormation).x;
        *pY = (*psFormation).y;
        return
    }
    objRadius = formationObjRadius(psObj);
    *pX = (*psFormation).x;
    *pY = (*psFormation).y;
    chosenLine = 0 as libc::c_int;
    chosenDist = 0 as libc::c_int;
    chosenPrev = -(1 as libc::c_int);
    objDist = 0x7fffffff as libc::c_int;
    chosenRank = 0x7fffffff as libc::c_int;
    line = 0 as libc::c_int;
    while line < (*psFormation).numLines as libc::c_int {
        // find the first free position on this line
        currDist = 0 as libc::c_int;
        rank = 1 as libc::c_int;
        prev = -(1 as libc::c_int);
        unit = (*asLines.offset(line as isize)).member as SDWORD;
        found = 0 as libc::c_int;
        while found == 0 && rank <= 15 as libc::c_int {
            if unit < 20 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"formationFindFree: unit out of range\x00" as *const u8
                          as *const libc::c_char);
            };
            if unit < 20 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"formation.c\x00" as *const u8 as *const libc::c_char,
                      432 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"formationFindFree\x00")).as_ptr(),
                      b"unit < F_MAXMEMBERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            if unit != -(1 as libc::c_int) {
                // See if the object can fit in the gap between this and the last unit
                radius =
                    formationObjRadius((*asMembers.offset(unit as
                                                              isize)).psObj);
                if objRadius * 2 as libc::c_int <=
                       (*asMembers.offset(unit as isize)).dist as libc::c_int
                           - radius - currDist {
                    found = 1 as libc::c_int
                } else {
                    prev = unit;
                    currDist =
                        (*asMembers.offset(unit as isize)).dist as libc::c_int
                            + radius;
                    unit = (*asMembers.offset(unit as isize)).next as SDWORD;
                    // if this line is full move onto a rank behind it
					// reset the distance to the start of a rank
                    if currDist + objRadius >=
                           (*psFormation).size as libc::c_int * rank {
                        currDist = (*psFormation).size as libc::c_int * rank;
                        rank += 1 as libc::c_int
                    }
                }
            } else { found = 1 as libc::c_int }
            if found != 0 {
                // calculate the position on the line
                formationCalcPos(psFormation, line, currDist + objRadius,
                                 &mut x, &mut y);
                if fpathBlockingTile.expect("non-null function pointer")(x >>
                                                                             7
                                                                                 as
                                                                                 libc::c_int,
                                                                         y >>
                                                                             7
                                                                                 as
                                                                                 libc::c_int)
                       != 0 {
                    // blocked, try the next rank
                    found = 0 as libc::c_int;
                    currDist = (*psFormation).size as libc::c_int * rank;
                    rank += 1 as libc::c_int
                }
            }
        }
        // see if this gap is closer to the unit than the previous one
        xdiff = x - (*psObj).x as SDWORD;
        ydiff = y - (*psObj).y as SDWORD;
        dist = xdiff * xdiff + ydiff * ydiff;
        //		dist += psFormation->rankDist*psFormation->rankDist * rank*rank;
        if dist < objDist && rank == chosenRank || rank < chosenRank {
            //		if (dist < objDist)
            // this gap is nearer
            objDist = dist;
            chosenLine = line;
            chosenDist = currDist + objRadius;
            chosenPrev = prev;
            chosenRank = rank;
            *pX = x;
            *pY = y
        }
        line += 1
    }
    //			objDist += psFormation->rankDist*psFormation->rankDist * rank*rank;
    // initialise the member
    unit = (*psFormation).free as SDWORD;
    (*psFormation).free = (*asMembers.offset(unit as isize)).next;
    (*asMembers.offset(unit as isize)).line = chosenLine as SBYTE;
    (*asMembers.offset(unit as isize)).dist = chosenDist as SWORD;
    let ref mut fresh2 = (*asMembers.offset(unit as isize)).psObj;
    *fresh2 = psObj;
    // insert the unit into the list
    if chosenPrev == -(1 as libc::c_int) {
        (*asMembers.offset(unit as isize)).next =
            (*asLines.offset(chosenLine as isize)).member;
        (*asLines.offset(chosenLine as isize)).member = unit as SBYTE
    } else {
        (*asMembers.offset(unit as isize)).next =
            (*asMembers.offset(chosenPrev as isize)).next;
        (*asMembers.offset(chosenPrev as isize)).next = unit as SBYTE
    };
}
// re-insert all the units in the formation
// re-insert all the units in the formation
#[no_mangle]
pub unsafe extern "C" fn formationReorder(mut psFormation: *mut FORMATION) {
    let mut numObj: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut curr: SDWORD = 0;
    let mut prev: SDWORD = 0;
    let mut asMembers: *mut F_MEMBER = 0 as *mut F_MEMBER;
    let mut asObjects: [F_MEMBER; 20] =
        [F_MEMBER{line: 0, next: 0, dist: 0, psObj: 0 as *mut BASE_OBJECT,};
            20];
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut insert: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut aDist: [SDWORD; 20] = [0; 20];
    // first find all the units to insert
    asMembers = (*psFormation).asMembers.as_mut_ptr();
    numObj = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        psObj = (*asMembers.offset(i as isize)).psObj;
        if !psObj.is_null() {
            asObjects[numObj as usize].psObj = psObj;
            xdiff = (*psObj).x as SDWORD - (*psFormation).x;
            ydiff = (*psObj).y as SDWORD - (*psFormation).y;
            aDist[numObj as usize] = xdiff * xdiff + ydiff * ydiff;
            numObj += 1 as libc::c_int
        }
        i += 1
    }
    // create a list in distance order
    insert = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < numObj {
        if insert == -(1 as libc::c_int) {
            // insert at the start of the list
            asObjects[i as usize].next = -(1 as libc::c_int) as SBYTE;
            insert = i
        } else {
            prev = -(1 as libc::c_int);
            curr = insert;
            while curr != -(1 as libc::c_int) {
                if aDist[i as usize] < aDist[curr as usize] { break ; }
                prev = curr;
                curr = asObjects[curr as usize].next as SDWORD
            }
            if prev == -(1 as libc::c_int) {
                // insert at the start of the list
                asObjects[i as usize].next = insert as SBYTE;
                insert = i
            } else {
                asObjects[i as usize].next = asObjects[prev as usize].next;
                asObjects[prev as usize].next = i as SBYTE
            }
        }
        i += 1
    }
    // reset the free list
    (*psFormation).free = 0 as libc::c_int as SBYTE;
    memset((*psFormation).asMembers.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<[F_MEMBER; 20]>() as libc::c_ulong);
    i = 0 as libc::c_int;
    while i < 20 as libc::c_int {
        (*psFormation).asMembers[i as usize].next =
            (i + 1 as libc::c_int) as SBYTE;
        i += 1
    }
    (*psFormation).asMembers[(20 as libc::c_int - 1 as libc::c_int) as
                                 usize].next = -(1 as libc::c_int) as SBYTE;
    i = 0 as libc::c_int;
    while i < (*psFormation).numLines as libc::c_int {
        (*psFormation).asLines[i as usize].member =
            -(1 as libc::c_int) as SBYTE;
        i += 1
    }
    // insert each member again
    while insert != -(1 as libc::c_int) {
        formationFindFree(psFormation, asObjects[insert as usize].psObj,
                          &mut xdiff, &mut ydiff);
        insert = asObjects[insert as usize].next as SDWORD
    };
}
// get a target position to move into a formation
// re-insert all the units in the formation
/*void formationReorder(FORMATION *psFormation)
{
	SDWORD		numObj, i,j, rank;
	F_MEMBER	*asMembers;
	F_LINE		*asLines;
	BASE_OBJECT *apsObjects[F_MAXMEMBERS];
	SDWORD		aDist[F_MAXLINES][F_MAXMEMBERS];
	struct
	{
		SDWORD	rank, dist, prev, x,y;
	}			aFreePos[F_MAXLINES];
	SDWORD		xdiff,ydiff, line,dist,obj, unit;

	// first find all the units to insert
	asMembers = psFormation->asMembers;
	asLines = psFormation->asLines;
	numObj = 0;
	memset(apsObjects, 0, sizeof(apsObjects));
	for(i=0; i<F_MAXMEMBERS; i++)
	{
		if (asMembers[i].psObj != NULL)
		{
			apsObjects[numObj] = asMembers[i].psObj;
			numObj += 1;
		}
	}

	// reset the free list
	psFormation->free = 0;
	memset(psFormation->asMembers, 0, sizeof(psFormation->asMembers));
	for(i=0; i<F_MAXMEMBERS; i++)
	{
		psFormation->asMembers[i].next = (SBYTE)(i+1);
	}
	psFormation->asMembers[F_MAXMEMBERS - 1].next = -1;
	for(i=0; i<psFormation->numLines; i++)
	{
		psFormation->asLines[i].member = -1;
	}
	psFormation->maxRank = 0;

	// initialise the free positions in the formation
	for(i=0; i< psFormation->numLines; i++)
	{
		aFreePos[i].rank = 1;
		aFreePos[i].dist = 0;
		aFreePos[i].prev = -1;
		formationCalcPos(psFormation, i, 0,
			&aFreePos[i].x, &aFreePos[i].y);
	}

	// now insert all the objects into the formation
	while (numObj > 0)
	{
		// decide which rank to use
		rank = SDWORD_MAX;
		for(i=0; i< psFormation->numLines; i++)
		{
			if (aFreePos[i].rank < rank)
			{
				rank = aFreePos[i].rank;
			}
		}
		if (psFormation->maxRank < rank)
		{
			psFormation->maxRank = (UBYTE)rank;
		}

		// now calculate the distance between each object and the free positions
		for(i=0; i<psFormation->numLines; i++)
		{
			for(j=0; j<F_MAXMEMBERS; j++)
			{
				if ((apsObjects[j] == NULL) || (aFreePos[i].rank != rank) )
				{
					aDist[i][j] = SDWORD_MAX;
				}
				else
				{
					xdiff = aFreePos[i].x - (SDWORD)apsObjects[j]->x;
					ydiff = aFreePos[i].y - (SDWORD)apsObjects[j]->y;
					aDist[i][j] = xdiff*xdiff + ydiff*ydiff;
				}
			}
		}

		// find the object nearest to a free position
		dist = SDWORD_MAX;
		for(i=0; i<psFormation->numLines; i++)
		{
			for(j=0; j<F_MAXMEMBERS; j++)
			{
				if (aDist[i][j] < dist)
				{
					dist = aDist[i][j];
					line = i;
					obj = j;
				}
			}
		}

		// put the object into the formation
		unit = psFormation->free;
		psFormation->free = asMembers[unit].next;
		asMembers[unit].line = (SBYTE)line;
		asMembers[unit].dist = (SWORD)(aFreePos[line].dist + formationObjRadius(apsObjects[obj]));
		if (asMembers[unit].dist >= psFormation->size * aFreePos[line].rank)
		{
			asMembers[unit].dist = (SWORD)(psFormation->size * aFreePos[line].rank +
						formationObjRadius(apsObjects[obj]));
		}
		asMembers[unit].psObj = apsObjects[obj];

		// insert the unit into the list
		if (aFreePos[line].prev == -1)
		{
			asMembers[unit].next = asLines[line].member;
			asLines[line].member = (SBYTE)unit;
		}
		else
		{
			asMembers[unit].next = asMembers[aFreePos[line].prev].next;
			asMembers[aFreePos[line].prev].next = (SBYTE)unit;
		}

		// update the free position
		aFreePos[line].dist = asMembers[unit].dist + formationObjRadius(apsObjects[obj]);
		aFreePos[line].prev = unit;
		if (aFreePos[line].dist >= psFormation->size * aFreePos[line].rank)
		{
			aFreePos[line].dist = psFormation->size * aFreePos[line].rank;
//				+ formationObjRadius(apsObjects[obj]);
			aFreePos[line].rank += 1;
		}
		formationCalcPos(psFormation, line, aFreePos[line].dist,
			&aFreePos[line].x, &aFreePos[line].y);

		apsObjects[obj] = NULL;
		numObj -= 1;
	}
}*/
// get a target position to move into a formation
#[no_mangle]
pub unsafe extern "C" fn formationGetPos(mut psFormation: *mut FORMATION,
                                         mut psObj: *mut BASE_OBJECT,
                                         mut pX: *mut SDWORD,
                                         mut pY: *mut SDWORD,
                                         mut bCheckLOS: BOOL) -> BOOL {
    let mut xdiff: SDWORD = 0; //,rangeSq;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut member: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut asMembers: *mut F_MEMBER = 0 as *mut F_MEMBER;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"formationGetPos: invalid formation pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"formation.c\x00" as *const u8 as *const libc::c_char,
              753 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"formationGetPos\x00")).as_ptr(),
              b"PTRVALID(psFormation, sizeof(FORMATION))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /*	if (psFormation->refCount == 1)
	{
		// nothing else in the formation so don't do anything
		return FALSE;
	}*/
    // see if the unit is close enough to join the formation
    xdiff = (*psFormation).x - (*psObj).x as SDWORD;
    ydiff = (*psFormation).y - (*psObj).y as SDWORD;
    distSq = xdiff * xdiff + ydiff * ydiff;
    //	rangeSq = 3*psFormation->size/2;
//	rangeSq = rangeSq*rangeSq;
//	if (distSq > F_JOINRANGE*F_JOINRANGE)
//	{
//		return FALSE;
//	}
    // see if the unit is already in the formation
    asMembers = (*psFormation).asMembers.as_mut_ptr();
    member = 0 as libc::c_int;
    while member < 20 as libc::c_int {
        if (*asMembers.offset(member as isize)).psObj == psObj { break ; }
        member += 1
    }
    if member < 20 as libc::c_int {
        // the unit is already in the formation - return it's current position
        formationCalcPos(psFormation,
                         (*asMembers.offset(member as isize)).line as SDWORD,
                         (*asMembers.offset(member as isize)).dist as SDWORD,
                         &mut x, &mut y);
    } else if (*psFormation).free as libc::c_int != -(1 as libc::c_int) {
        // add the new object to the members
        (*psFormation).asMembers[(*psFormation).free as libc::c_int as
                                     usize].psObj = psObj;
        (*psFormation).free =
            (*psFormation).asMembers[(*psFormation).free as libc::c_int as
                                         usize].next;
        formationReorder(psFormation);
        /*		// a unit has just joined the formation - find a location for it
		formationFindFree(psFormation, psObj, &x,&y);
		DBP0(("formation new member : (%d, %d)\n",
					x,y));*/
        member = 0 as libc::c_int;
        while member < 20 as libc::c_int {
            if (*asMembers.offset(member as isize)).psObj == psObj { break ; }
            member += 1
        }
        formationCalcPos(psFormation,
                         (*asMembers.offset(member as isize)).line as SDWORD,
                         (*asMembers.offset(member as isize)).dist as SDWORD,
                         &mut x, &mut y);
    } else { return 0 as libc::c_int }
    // find the object
    // calculate its position
    // check the unit can get to the formation position
    if bCheckLOS != 0 &&
           fpathTileLOS((*psObj).x as libc::c_int >> 7 as libc::c_int,
                        (*psObj).y as libc::c_int >> 7 as libc::c_int,
                        x >> 7 as libc::c_int, y >> 7 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    *pX = x;
    *pY = y;
    return 1 as libc::c_int;
}
// See if a unit is a member of a formation (i.e. it has a position assigned)
// See if a unit is a member of a formation (i.e. it has a position assigned)
#[no_mangle]
pub unsafe extern "C" fn formationMember(mut psFormation: *mut FORMATION,
                                         mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut member: SDWORD = 0;
    let mut asMembers: *mut F_MEMBER = 0 as *mut F_MEMBER;
    // see if the unit is already in the formation
    asMembers = (*psFormation).asMembers.as_mut_ptr();
    member = 0 as libc::c_int;
    while member < 20 as libc::c_int {
        if (*asMembers.offset(member as isize)).psObj == psObj {
            return 1 as libc::c_int
        }
        member += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn formationObjRadius(mut psObj: *mut BASE_OBJECT)
 -> SDWORD {
    let mut radius: SDWORD = 0;
    let mut psBdyStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            radius =
                3 as libc::c_int * (*(*psObj).sDisplay.imd).radius /
                    2 as libc::c_int;
            psBdyStats =
                asBodyStats.offset((*(psObj as
                                          *mut DROID)).asBits[COMP_BODY as
                                                                  libc::c_int
                                                                  as
                                                                  usize].nStat
                                       as libc::c_int as isize);
            match (*psBdyStats).size as libc::c_int {
                1 => { radius = fmMedRad }
                2 => { radius = fmHvyRad }
                3 => { radius = 500 as libc::c_int }
                0 | _ => { radius = fmLtRad }
            }
        }
        1 => {
            //		radius = psObj->sDisplay.imd->visRadius;
            radius = (*(*psObj).sDisplay.imd).radius / 2 as libc::c_int
        }
        2 => {
            //		radius = psObj->sDisplay.imd->visRadius;
            radius = (*(*psObj).sDisplay.imd).radius / 2 as libc::c_int
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"formationObjRadius: unknown object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"formation.c\x00" as *const u8 as *const libc::c_char,
                      891 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"formationObjRadius\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            radius = 0 as libc::c_int
        }
    }
    return radius;
}
