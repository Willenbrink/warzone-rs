use ::libc;
extern "C" {
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
    fn validLocation(psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD,
                     player: UDWORD, bCheckBuildQueue: BOOL) -> BOOL;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    static mut mouseTileX: SDWORD;
    #[no_mangle]
    static mut mouseTileY: SDWORD;
    #[no_mangle]
    static mut dragBox3D: _dragBox;
    #[no_mangle]
    static mut wallDrag: _dragBox;
    #[no_mangle]
    fn CancelDeliveryRepos();
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
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
pub type STRUCTURE_STATS = _structure_stats;
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
pub type BUILDCALLBACK
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut libc::c_void)
               -> ()>;
pub type BUILDDETAILS = _build_details;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _build_details {
    pub CallBack: BUILDCALLBACK,
    pub UserData: *mut libc::c_void,
    pub x: UDWORD,
    pub y: UDWORD,
    pub width: UDWORD,
    pub height: UDWORD,
    pub psStats: *mut BASE_STATS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dragBox {
    pub x1: UDWORD,
    pub y1: UDWORD,
    pub x2: UDWORD,
    pub y2: UDWORD,
    pub status: UDWORD,
    pub lastTime: UDWORD,
    pub boxColourIndex: UDWORD,
}
pub type HIGHLIGHT = _highlight;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _highlight {
    pub xTL: UWORD,
    pub yTL: UWORD,
    pub xBR: UWORD,
    pub yBR: UWORD,
}
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
// Feature armour
//this holds the OBJECT_POSITION pointer for a Deliv Point
// Top left of box to highlight
// Bottom right of box to highlight
/*
	Edit3D.c - to ultimately contain the map editing functions -
	they are presently scattered in various files .
	Alex McLean, Pumpkin Studios, EIDOS Interactive, 1997
	*/
/*
Definition of a tile to highlight - presently more than is required
but means that we can highlight any individual tile in future. An
x coordinate that is greater than mapWidth implies that the highlight
is invalid (not currently being used)
*/
#[no_mangle]
pub static mut buildState: UDWORD = 99 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut sBuildDetails: BUILDDETAILS =
    BUILDDETAILS{CallBack: None,
                 UserData: 0 as *const libc::c_void as *mut libc::c_void,
                 x: 0,
                 y: 0,
                 width: 0,
                 height: 0,
                 psStats: 0 as *const BASE_STATS as *mut BASE_STATS,};
#[no_mangle]
pub static mut buildSite: HIGHLIGHT =
    HIGHLIGHT{xTL: 0, yTL: 0, xBR: 0, yBR: 0,};
// Initialisation function for statis & globals in this module.
//
#[no_mangle]
pub unsafe extern "C" fn Edit3DInitVars() {
    buildState = 99 as libc::c_int as UDWORD;
}
/* Raises a tile by a #defined height */
#[no_mangle]
pub unsafe extern "C" fn raiseTile(mut tile3dX: UDWORD, mut tile3dY: UDWORD) {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psTile = mapTile(tile3dX, tile3dY);
    adjustTileHeight(psTile, 1 as libc::c_int);
    psTile =
        mapTile(tile3dX.wrapping_add(1 as libc::c_int as libc::c_uint),
                tile3dY);
    adjustTileHeight(psTile, 1 as libc::c_int);
    psTile =
        mapTile(tile3dX.wrapping_add(1 as libc::c_int as libc::c_uint),
                tile3dY.wrapping_add(1 as libc::c_int as libc::c_uint));
    adjustTileHeight(psTile, 1 as libc::c_int);
    psTile =
        mapTile(tile3dX,
                tile3dY.wrapping_add(1 as libc::c_int as libc::c_uint));
    adjustTileHeight(psTile, 1 as libc::c_int);
}
/* Lowers a tile by a #defined height */
#[no_mangle]
pub unsafe extern "C" fn lowerTile(mut tile3dX: UDWORD, mut tile3dY: UDWORD) {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psTile = mapTile(tile3dX, tile3dY);
    adjustTileHeight(psTile, -(1 as libc::c_int));
    psTile =
        mapTile(tile3dX.wrapping_add(1 as libc::c_int as libc::c_uint),
                tile3dY);
    adjustTileHeight(psTile, -(1 as libc::c_int));
    psTile =
        mapTile(tile3dX.wrapping_add(1 as libc::c_int as libc::c_uint),
                tile3dY.wrapping_add(1 as libc::c_int as libc::c_uint));
    adjustTileHeight(psTile, -(1 as libc::c_int));
    psTile =
        mapTile(tile3dX,
                tile3dY.wrapping_add(1 as libc::c_int as libc::c_uint));
    adjustTileHeight(psTile, -(1 as libc::c_int));
}
/* Ensures any adjustment to tile elevation is within allowed ranges */
#[no_mangle]
pub unsafe extern "C" fn adjustTileHeight(mut psTile: *mut MAPTILE,
                                          mut adjust: SDWORD) {
    let mut newHeight: SDWORD = 0;
    newHeight = (*psTile).height as libc::c_int + adjust;
    if newHeight >= 0 as libc::c_int && newHeight <= 255 as libc::c_int {
        (*psTile).height = newHeight as libc::c_uchar
    };
}
#[no_mangle]
pub unsafe extern "C" fn inHighlight(mut realX: UDWORD, mut realY: UDWORD)
 -> BOOL {
    let mut retVal: BOOL = 0 as libc::c_int;
    if realX >= buildSite.xTL as libc::c_uint &&
           realX <= buildSite.xBR as libc::c_uint {
        if realY >= buildSite.yTL as libc::c_uint &&
               realY <= buildSite.yBR as libc::c_uint {
            retVal = 1 as libc::c_int
        }
    }
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn init3DBuilding(mut psStats: *mut BASE_STATS,
                                        mut CallBack: BUILDCALLBACK,
                                        mut UserData: *mut libc::c_void) {
    buildState = 100 as libc::c_int as UDWORD;
    sBuildDetails.CallBack = CallBack;
    sBuildDetails.UserData = UserData;
    sBuildDetails.x = mouseTileX as UDWORD;
    sBuildDetails.y = mouseTileY as UDWORD;
    if (*psStats).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        sBuildDetails.width = (*(psStats as *mut STRUCTURE_STATS)).baseWidth;
        sBuildDetails.height =
            (*(psStats as *mut STRUCTURE_STATS)).baseBreadth;
        sBuildDetails.psStats = psStats;
        // hack to increase the size of repair facilities
        if (*(psStats as *mut STRUCTURE_STATS)).type_0 ==
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            sBuildDetails.width =
                (sBuildDetails.width as
                     libc::c_uint).wrapping_add(2 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            sBuildDetails.height =
                (sBuildDetails.height as
                     libc::c_uint).wrapping_add(2 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
    } else if (*psStats).ref_0 >= 0x100000 as libc::c_int as libc::c_uint &&
                  (*psStats).ref_0 <
                      (0x100000 as libc::c_int + 0x10000 as libc::c_int) as
                          libc::c_uint {
        sBuildDetails.width =
            (*(psStats as *mut FEATURE_STATS)).baseWidth as UDWORD;
        sBuildDetails.height =
            (*(psStats as *mut FEATURE_STATS)).baseBreadth as UDWORD;
        sBuildDetails.psStats = psStats
    } else {
        /*if (psStats->ref >= REF_TEMPLATE_START &&
			 psStats->ref < (REF_TEMPLATE_START + REF_RANGE))*/
        sBuildDetails.width = 1 as libc::c_int as UDWORD;
        sBuildDetails.height = 1 as libc::c_int as UDWORD;
        sBuildDetails.psStats = psStats
    };
}
#[no_mangle]
pub unsafe extern "C" fn kill3DBuilding() {
    CancelDeliveryRepos();
    //cancel the drag boxes
    dragBox3D.status = 0 as libc::c_int as UDWORD;
    wallDrag.status = 0 as libc::c_int as UDWORD;
    buildState = 99 as libc::c_int as UDWORD;
}
// Call once per frame to handle structure positioning and callbacks.
//
#[no_mangle]
pub unsafe extern "C" fn process3DBuilding() -> BOOL {
    let mut bX: UDWORD = 0;
    let mut bY: UDWORD = 0;
    //if not trying to build ignore
    if buildState == 99 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if buildState != 101 as libc::c_int as libc::c_uint {
        // AND buildState != BUILD3D_NONE)
        bX = mouseTileX as UDWORD;
        bY = mouseTileY as UDWORD;
        // lovely hack to make the repair facility 3x3 - need to offset the position by 1
        if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 ==
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            bX =
                (bX as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            bY =
                (bY as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        if validLocation(sBuildDetails.psStats, bX, bY, selectedPlayer,
                         1 as libc::c_int) != 0 {
            buildState = 102 as libc::c_int as UDWORD
        } else { buildState = 100 as libc::c_int as UDWORD }
    }
    /* Need to update the building locations if we're building */
    bX = mouseTileX as UDWORD;
    bY = mouseTileY as UDWORD;
    if mouseTileX < 2 as libc::c_int {
        bX = 2 as libc::c_int as UDWORD
    } else { bX = mouseTileX as UDWORD }
    if mouseTileX >
           mapWidth.wrapping_sub(3 as libc::c_int as libc::c_uint) as SDWORD {
        bX = mapWidth.wrapping_sub(3 as libc::c_int as libc::c_uint)
    } else { bX = mouseTileX as UDWORD }
    if mouseTileY < 2 as libc::c_int {
        bY = 2 as libc::c_int as UDWORD
    } else { bY = mouseTileY as UDWORD }
    if mouseTileY >
           mapHeight.wrapping_sub(3 as libc::c_int as libc::c_uint) as SDWORD
       {
        bY = mapHeight.wrapping_sub(3 as libc::c_int as libc::c_uint)
    } else { bY = mouseTileY as UDWORD }
    buildSite.xTL = bX as UWORD;
    sBuildDetails.x = buildSite.xTL as UDWORD;
    buildSite.yTL = bY as UWORD;
    sBuildDetails.y = buildSite.yTL as UDWORD;
    buildSite.xBR =
        (buildSite.xTL as
             libc::c_uint).wrapping_add(sBuildDetails.width).wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
            as UWORD;
    buildSite.yBR =
        (buildSite.yTL as
             libc::c_uint).wrapping_add(sBuildDetails.height).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
            as UWORD;
    if buildState == 101 as libc::c_int as libc::c_uint &&
           sBuildDetails.CallBack.is_some() {
        sBuildDetails.CallBack.expect("non-null function pointer")(sBuildDetails.x,
                                                                   sBuildDetails.y,
                                                                   sBuildDetails.UserData);
        buildState = 99 as libc::c_int as UDWORD;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* See if a structure location has been found */
#[no_mangle]
pub unsafe extern "C" fn found3DBuilding(mut x: *mut UDWORD,
                                         mut y: *mut UDWORD) -> BOOL {
    if buildState != 101 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    *x = sBuildDetails.x;
    *y = sBuildDetails.y;
    // lovely hack to make the repair facility 3x3 - need to offset the position by 1
    if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 ==
           REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
        *x =
            (*x as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        *y =
            (*y as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    buildState = 99 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* See if a second position for a build has been found */
#[no_mangle]
pub unsafe extern "C" fn found3DBuildLocTwo(mut px1: *mut UDWORD,
                                            mut py1: *mut UDWORD,
                                            mut px2: *mut UDWORD,
                                            mut py2: *mut UDWORD) -> BOOL {
    if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 !=
           REF_WALL as libc::c_int as libc::c_uint &&
           (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 !=
               REF_DEFENSE as libc::c_int as libc::c_uint ||
           wallDrag.status != 2 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    //whilst we're still looking for a valid location - return FALSE
    if buildState == 100 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    wallDrag.status = 0 as libc::c_int as UDWORD;
    *px1 = wallDrag.x1;
    *py1 = wallDrag.y1;
    *px2 = wallDrag.x2;
    *py2 = wallDrag.y2;
    return 1 as libc::c_int;
}
/*returns true if the build state is not equal to BUILD3D_NONE*/
/*returns true if the build state is not equal to BUILD3D_NONE*/
#[no_mangle]
pub unsafe extern "C" fn tryingToGetLocation() -> BOOL {
    if buildState == 99 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
