use ::libc;
extern "C" {
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_SetGeometricOffset(x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    fn iV_SurfaceCreate(flags: uint32, width: libc::c_int,
                        height: libc::c_int, xp: libc::c_int, yp: libc::c_int,
                        buffer: *mut uint8) -> *mut iSurface;
    #[no_mangle]
    fn getResearchRadius(Stat: *mut BASE_STATS) -> UDWORD;
    #[no_mangle]
    fn getStructureStatSize(Stats: *mut STRUCTURE_STATS) -> UDWORD;
    #[no_mangle]
    fn getStructureStatHeight(psStat: *mut STRUCTURE_STATS) -> UDWORD;
    #[no_mangle]
    fn displayStructureStatButton(Stats: *mut STRUCTURE_STATS, Player: UDWORD,
                                  Rotation: *mut iVector,
                                  Position: *mut iVector, RotXYZ: BOOL,
                                  scale: SDWORD);
    #[no_mangle]
    fn displayComponentButton(Stat: *mut BASE_STATS, Rotation: *mut iVector,
                              Position: *mut iVector, RotXYZ: BOOL,
                              scale: SDWORD);
    #[no_mangle]
    fn displayResearchButton(Stat: *mut BASE_STATS, Rotation: *mut iVector,
                             Position: *mut iVector, RotXYZ: BOOL,
                             scale: SDWORD);
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    #[no_mangle]
    fn StatIsComponent(Stat: *mut BASE_STATS) -> SDWORD;
    #[no_mangle]
    fn StatIsStructure(Stat: *mut BASE_STATS) -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
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
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
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
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
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
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
/* **************************************************************************/
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
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
pub type PSBSPTREENODE = *mut BSPTREENODE;
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
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
pub type STRUCTURE_STATS = _structure_stats;
pub const IMDTYPE_STRUCTURESTAT: C2RustUnnamed = 6;
pub const IMDTYPE_RESEARCH: C2RustUnnamed = 5;
pub const IMDTYPE_COMPONENT: C2RustUnnamed = 3;
pub type C2RustUnnamed = libc::c_uint;
pub const IMDTYPE_STRUCTURE: C2RustUnnamed = 4;
pub const IMDTYPE_DROIDTEMPLATE: C2RustUnnamed = 2;
pub const IMDTYPE_DROID: C2RustUnnamed = 1;
pub const IMDTYPE_NONE: C2RustUnnamed = 0;
/* the 2nd IMD for base plates/turrets*/
//fill the intelColours array with the colours used for the background
//static void setUpIntelColours(void);
/* ----------------------------------------------------------------------------------------- */
#[no_mangle]
pub static mut elevation: SDWORD = 0;
#[no_mangle]
pub static mut mapPos: iVector = iVector{x: 0, y: 0, z: 0,};
#[no_mangle]
pub static mut mapView: iVector = iVector{x: 0, y: 0, z: 0,};
//static	iVector	oldPos, oldView;
#[no_mangle]
pub static mut sP1: POINT = POINT{x: 0, y: 0,};
#[no_mangle]
pub static mut sP2: POINT = POINT{x: 0, y: 0,};
#[no_mangle]
pub static mut sP3: POINT = POINT{x: 0, y: 0,};
#[no_mangle]
pub static mut sP4: POINT = POINT{x: 0, y: 0,};
#[no_mangle]
pub static mut psP1: *mut POINT = 0 as *const POINT as *mut POINT;
#[no_mangle]
pub static mut psP2: *mut POINT = 0 as *const POINT as *mut POINT;
#[no_mangle]
pub static mut psP3: *mut POINT = 0 as *const POINT as *mut POINT;
#[no_mangle]
pub static mut psP4: *mut POINT = 0 as *const POINT as *mut POINT;
#[no_mangle]
pub static mut psPTemp: *mut POINT = 0 as *const POINT as *mut POINT;
/*Flag to switch code for bucket sorting in renderFeatures etc
  for the renderMapToBuffer code */
  /*This is no longer used but may be useful for testing so I've left it in - maybe
  get rid of it eventually? - AB 1/4/98*/
#[no_mangle]
pub static mut doBucket: BOOL = 1 as libc::c_int;
//colours used to 'paint' the background of 3D view
#[no_mangle]
pub static mut intelColours: [UDWORD; 20] = [0; 20];
/*	Sets up a map surface by allocating the necessary memory and assigning world
	variables for the renderer to work with */
/* ----------------------------------------------------------------------------------------- */
/* Functions */
#[no_mangle]
pub unsafe extern "C" fn setUpMapSurface(mut width: UDWORD,
                                         mut height: UDWORD)
 -> *mut iSurface {
    let mut bufSpace: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pMapSurface: *mut iSurface = 0 as *mut iSurface;
    /*	Release the old buffer if necessary - we may use many different intel maps
		before resetting the game back to init/close */
	//releaseIntelMap();
    /* Get the required memory for the render surface */
    bufSpace = memMallocRelease(width.wrapping_mul(height));
    //initialise the buffer
    memset(bufSpace, 0 as libc::c_int, width.wrapping_mul(height));
    /* Exit if we can't get it! */
    if !bufSpace.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Can\'t get the memory for the map buffer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !bufSpace.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mapdisplay.c\x00" as *const u8 as *const libc::c_char,
              115 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"setUpMapSurface\x00")).as_ptr(),
              b"bufSpace!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    /* Build our new surface */
    pMapSurface =
        iV_SurfaceCreate(2 as libc::c_int as uint32, width as libc::c_int,
                         height as libc::c_int, 10 as libc::c_int,
                         10 as libc::c_int, bufSpace as *mut uint8);
    /* Exit if we can't get it! */
    if !pMapSurface.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Whoa - can\'t make surface for map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !pMapSurface.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mapdisplay.c\x00" as *const u8 as *const libc::c_char,
              121 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"setUpMapSurface\x00")).as_ptr(),
              b"pMapSurface!=NULL\x00" as *const u8 as *const libc::c_char);
    };
    //set up the intel colours
	//setUpIntelColours();
    /*	Return a pointer to our surface - from this they can get the rendered buffer
		as well as info about width and height etc. */
    return pMapSurface;
}
#[no_mangle]
pub unsafe extern "C" fn releaseMapSurface(mut pSurface: *mut iSurface) {
    /* Free up old alloaction if necessary */
    if !pSurface.is_null() {
        /* Free up old buffer if necessary */
        if !(*pSurface).buffer.is_null() {
            memFreeRelease((*pSurface).buffer as *mut libc::c_void);
            (*pSurface).buffer = 0 as *mut uint8
        }
        memFreeRelease(pSurface as *mut libc::c_void);
        pSurface = 0 as *mut iSurface
    };
}
/* ----------------------------------------------------------------------------------------- */
/* Function prototypes */
/*	Sets up the intelligence map by allocating the necessary memory and assigning world
	variables for the renderer to work with */
//void		setUpIntelMap		(UDWORD width, UDWORD height);
/* Draws the intelligence map to the already setup buffer */
//void		renderIntelMap		(iVector *location, iVector *viewVector, UDWORD elevation);
/* Frees up the memory we've used */
//void		releaseIntelMap		( void );
/* Draw a tile on the grid */
//void		drawMapTile				(SDWORD i, SDWORD j);//line draw nolonger used
/* Clears the map buffer prior to drawing in it */
//static	void	clearMapBuffer(iSurface *surface);
//clear text message background with gray fill
//static void clearIntelText(iSurface *pSurface);
/*fills the map buffer with intelColours prior to drawing in it*/
//static	void	fillMapBuffer(iSurface *surface);
#[no_mangle]
pub unsafe extern "C" fn tileLayouts(mut texture: libc::c_int) {
    /* Store the source rect as four points */
    sP1.x = 0 as libc::c_int;
    sP1.y = 0 as libc::c_int;
    sP2.x = 63 as libc::c_int;
    sP2.y = 0 as libc::c_int;
    sP3.x = 63 as libc::c_int;
    sP3.y = 63 as libc::c_int;
    sP4.x = 0 as libc::c_int;
    sP4.y = 63 as libc::c_int;
    /* Store pointers to the points */
    psP1 = &mut sP1;
    psP2 = &mut sP2;
    psP3 = &mut sP3;
    psP4 = &mut sP4;
    if texture & 0x8000 as libc::c_int != 0 {
        psPTemp = psP1;
        psP1 = psP2;
        psP2 = psPTemp;
        psPTemp = psP3;
        psP3 = psP4;
        psP4 = psPTemp
    }
    if texture & 0x4000 as libc::c_int != 0 {
        psPTemp = psP1;
        psP1 = psP4;
        psP4 = psPTemp;
        psPTemp = psP2;
        psP2 = psP3;
        psP3 = psPTemp
    }
    match (texture & 0x3000 as libc::c_int) >> 12 as libc::c_int {
        1 => {
            psPTemp = psP1;
            psP1 = psP4;
            psP4 = psP3;
            psP3 = psP2;
            psP2 = psPTemp
        }
        2 => {
            psPTemp = psP1;
            psP1 = psP3;
            psP3 = psPTemp;
            psPTemp = psP4;
            psP4 = psP2;
            psP2 = psPTemp
        }
        3 => {
            psPTemp = psP1;
            psP1 = psP2;
            psP2 = psP3;
            psP3 = psP4;
            psP4 = psPTemp
        }
        _ => { }
    };
}
/* renders the Research IMDs into the surface - used by message display in
Intelligence Map */
#[no_mangle]
pub unsafe extern "C" fn renderResearchToBuffer(mut pSurface: *mut iSurface,
                                                mut psResearch: *mut RESEARCH,
                                                mut OriginX: UDWORD,
                                                mut OriginY: UDWORD) {
    let mut angle: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psResGraphic: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut compID: UDWORD = 0;
    let mut IMDType: UDWORD = 0;
    let mut Rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut basePlateSize: UDWORD = 0;
    let mut Radius: UDWORD = 0;
    let mut scale: SDWORD = 0 as libc::c_int;
    // Set identity (present) context
    pie_MatBegin();
    pie_SetGeometricOffset(OriginX.wrapping_add(10 as libc::c_int as
                                                    libc::c_uint) as
                               libc::c_int,
                           OriginY.wrapping_add(10 as libc::c_int as
                                                    libc::c_uint) as
                               libc::c_int);
    // Pitch down a bit
	//pie_MatRotX(DEG(-30));
    // Rotate round
	// full rotation once every 2 seconds..
    angle =
        gameTime2.wrapping_rem((2 as libc::c_int * 1000 as libc::c_int) as
                                   libc::c_uint).wrapping_mul(360 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint).wrapping_div((2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  *
                                                                                                  1000
                                                                                                      as
                                                                                                      libc::c_int)
                                                                                                 as
                                                                                                 libc::c_uint);
    Position.x = 0 as libc::c_int;
    Position.y = 0 as libc::c_int;
    Position.z = 2000 as libc::c_int;
    // Rotate round
    Rotation.x = -(30 as libc::c_int);
    Rotation.y = angle as int32;
    Rotation.z = 0 as libc::c_int;
    //draw the IMD for the research
    if !(*psResearch).psStat.is_null() {
        //we have a Stat associated with this research topic
        if StatIsStructure((*psResearch).psStat) != 0 {
            //this defines how the button is drawn
            IMDType = IMDTYPE_STRUCTURESTAT as libc::c_int as UDWORD;
            psResGraphic = (*psResearch).psStat;
            //set up the scale
            basePlateSize =
                getStructureStatSize((*psResearch).psStat as
                                         *mut STRUCTURE_STATS);
            if basePlateSize == 1 as libc::c_int as libc::c_uint {
                scale = 300 as libc::c_int / 2 as libc::c_int;
                /*HACK HACK HACK!
                if its a 'tall thin (ie tower)' structure stat with something on
                the top - offset the position to show the object on top*/
                if (*(*((*psResearch).psStat as
                            *mut STRUCTURE_STATS)).pIMD).nconnectors != 0 &&
                       getStructureStatHeight((*psResearch).psStat as
                                                  *mut STRUCTURE_STATS) >
                           100 as libc::c_int as libc::c_uint {
                    Position.y -= 30 as libc::c_int
                }
            } else if basePlateSize == 2 as libc::c_int as libc::c_uint {
                scale = 300 as libc::c_int / 4 as libc::c_int
            } else { scale = 300 as libc::c_int / 5 as libc::c_int }
        } else {
            compID = StatIsComponent((*psResearch).psStat) as UDWORD;
            if compID != COMP_UNKNOWN as libc::c_int as libc::c_uint {
                //this defines how the button is drawn
                IMDType = IMDTYPE_COMPONENT as libc::c_int as UDWORD;
                psResGraphic = (*psResearch).psStat;
                scale = 300 as libc::c_int
            } else {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"intDisplayMessageButton: invalid stat\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"mapdisplay.c\x00" as *const u8 as
                              *const libc::c_char, 260 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"renderResearchToBuffer\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                IMDType = IMDTYPE_RESEARCH as libc::c_int as UDWORD;
                psResGraphic = psResearch as *mut BASE_STATS
            }
        }
    } else {
        //no Stat for this research topic so use the research topic to define what is drawn
        psResGraphic = psResearch as *mut BASE_STATS;
        IMDType = IMDTYPE_RESEARCH as libc::c_int as UDWORD
    }
    //scale the research according to size of IMD
    if IMDType == IMDTYPE_RESEARCH as libc::c_int as libc::c_uint {
        Radius = getResearchRadius(psResGraphic);
        if Radius <= 100 as libc::c_int as libc::c_uint {
            scale = 300 as libc::c_int / 2 as libc::c_int
        } else if Radius <= 128 as libc::c_int as libc::c_uint {
            scale = 300 as libc::c_int / 3 as libc::c_int
        } else if Radius <= 256 as libc::c_int as libc::c_uint {
            scale = 300 as libc::c_int / 4 as libc::c_int
        } else { scale = 300 as libc::c_int / 5 as libc::c_int }
    }
    /* display the IMDs */
    if IMDType == IMDTYPE_COMPONENT as libc::c_int as libc::c_uint {
        displayComponentButton(psResGraphic, &mut Rotation, &mut Position,
                               1 as libc::c_int, scale);
    } else if IMDType == IMDTYPE_RESEARCH as libc::c_int as libc::c_uint {
        displayResearchButton(psResGraphic, &mut Rotation, &mut Position,
                              1 as libc::c_int, scale);
    } else if IMDType == IMDTYPE_STRUCTURESTAT as libc::c_int as libc::c_uint
     {
        displayStructureStatButton(psResGraphic as *mut STRUCTURE_STATS,
                                   selectedPlayer, &mut Rotation,
                                   &mut Position, 1 as libc::c_int, scale);
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"renderResearchToBuffer: Unknown PIEType\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"mapdisplay.c\x00" as *const u8 as *const libc::c_char,
                  305 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"renderResearchToBuffer\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    // close matrix context
    pie_MatEnd();
}
