use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn rand() -> libc::c_int;
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
    fn abs(_: libc::c_int) -> libc::c_int;
    /* **************************************************************************/
/*
 * pieMatrix.h
 *
 * matrix functions for pumpkin image library.
 *
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    //*************************************************************************
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_MATTRANS(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    #[no_mangle]
    fn pie_RotProj(v3d: *mut iVector, v2d: *mut iPoint) -> int32;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
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
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut camera: iView;
    #[no_mangle]
    static mut terrainMidX: UDWORD;
    #[no_mangle]
    static mut terrainMidY: UDWORD;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn DrawnInLastFrame(Frame: SDWORD) -> BOOL;
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
pub type FRACT = libc::c_float;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iPoint {
    pub x: int32,
    pub y: int32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iView {
    pub p: iVector,
    pub r: iVector,
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
pub type VERTEXID = libc::c_int;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
//COMP_ARMOUR,
//COMP_POWERPLANT,
//COMP_PROGRAM,		//this needs to be removed when save games changes
//ALL components and structures and research topics have a tech level to which they belong
pub type TECH_LEVEL = _tech_level;
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
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
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
pub type _droid_type = libc::c_uint;
pub const DROID_ANY: _droid_type = 13;
pub const DROID_CYBORG_SUPER: _droid_type = 12;
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
pub const DROID_DEFAULT: _droid_type = 9;
pub const DROID_REPAIR: _droid_type = 8;
pub const DROID_COMMAND: _droid_type = 7;
pub const DROID_TRANSPORTER: _droid_type = 6;
pub const DROID_CYBORG: _droid_type = 5;
pub const DROID_PERSON: _droid_type = 4;
pub const DROID_CONSTRUCT: _droid_type = 3;
pub const DROID_ECM: _droid_type = 2;
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
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
// Weapon droid
// Sensor droid
// ECM droid
// Constructor droid
// person
// cyborg-type thang
// guess what this is!
// Command droid
// Repair droid
// Default droid
//cyborg constructor droid - new for update 28/5/99
//cyborg repair droid - new for update 28/5/99
//cyborg repair droid - new for update 7/6/99
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//	UDWORD					nStat;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
//struct _base_object	*psObj;
//this needs to cope with objects and stats
//line build requires two sets of coords
// maximum number of characters in a droid name
// FOR DROID TEMPLATES
	// On the PC the pName entry in STATS_BASE is redundant and can be assumed to be NULL,
	// on the PSX the NameHash entry is used. If it is database generated template, the hashed version of the short name of the template is stored. If it is a user generated template NULL is stored.
/* basic stats */
// on the PC this contains the full editable ascii name of the template
	// on the PSX this is not used, the full name is NON-EDITABLE and is generated from the template components e.g. Viper Mk I
// Version number used in name (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied to droid structure
/* The droid components.  This array is indexed by COMPONENT_TYPE
	 * so the ECM would be accessed using asParts[COMP_ECM].
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/*total build points required to manufacture
												  the droid*/
/*total power points required to build/maintain
												  the droid */
/*used to load in weaps and progs*/
/* The weapon systems */
/* Number of weapons*/
/* weapon indices */
/* The programs */
	//UDWORD			numProgs;					/* Number of programs*/
	//UDWORD			asProgs[DROID_MAXPROGS];	/* program indices*/
// The type of droid
//#ifndef PSX
// multiplayer unique descriptor(cant use id's for templates)
// used for save games as well now - AB 29/10/98
//#endif
/* Pointer to next template*/
pub type DROID = _droid;
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
/* The common structure elements for all objects */
//Ascii name of the droid - This is generated from the droid template and can not be changed by the game player after creation.
//	UBYTE 		NameVersion;			// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
// The type of droid
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
//	DROID_TEMPLATE	*pTemplate;		//defines the droids components
/* Holds the specifics for the component parts - allows damage
	 * per part to be calculated. Indexed by COMPONENT_TYPE.
	 * Weapons and Programs need to be dealt with separately.
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/* The other droid data.  These are all derived from the components
	 * but stored here for easy access
	 */
//the base speed dependant on propulsion type
//the original body points
// the current body points
//UDWORD		power;
//tjc	UDWORD		imdNum;
//UWORD		turretRotRate; THIS IS A CONSTANT
//*
// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
//	UDWORD		numKills;
//used in Electronic Warfare
//	SDWORD		activeWeapon;		// The currently selected weapon
	//UDWORD		numWeaps;
//SDWORD		activeProg;			// The currently running program
	//UDWORD		numProgs;
	//PROGRAM		asProgs[DROID_MAXPROGS];
// The group the droid belongs to
//a structure that this droid might be associated with
                                                    //for vtols its the rearming pad
// queued orders
/* Order data */
// 	struct _base_object	*psLastAttacker;
// secondary order data
// multiplayer synchronisation value.
/* Action data */
// Action target object
// Game time action started
// number of points done by action since start
//UWORD				actionHeight;		// height to level the ground to for foundation,
											// possibly use it for other data as well? Yup! - powerAccrued!
// renamed the above variable since this is what its used for now!
//UBYTE				tileNumber;			// tile number for foundation NOT USED ANYMORE
/* Movement control data */
//	void				*lastTile;
	/* AI data */
//	AI_DATA				sAI;
	/* anim data */
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
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _t_quad {
    pub coords: [POINT; 4],
}
pub type QUAD = _t_quad;
// sensor range display.
// This is also used to store the tile flip flags
//  What's been removed - 46 bytes per tile so far
//	BASE_OBJECT		*psObject;		// Any object sitting on the location (e.g. building)
//	UBYTE			onFire;			// Is tile on fire?
//	UBYTE			rippleIndex;	// Current value in ripple table?
//	BOOL			tileVisible[MAX_PLAYERS]; // Which players can see the tile?
//	BOOL			triangleFlip;	// Is the triangle flipped?
//	TYPE_OF_TERRAIN	type;			// The terrain type for the tile
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
/*sets the tile height */
#[inline]
unsafe extern "C" fn setTileHeight(mut x: UDWORD, mut y: UDWORD,
                                   mut height: UDWORD) {
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
              308 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"setTileHeight\x00")).as_ptr(),
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
              310 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"setTileHeight\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    //psMapTiles[x + (y << mapShift)].height = height;//width no longer a power of 2
    (*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                            isize)).height =
        height.wrapping_div(2 as libc::c_int as libc::c_uint) as UBYTE;
}
/* The arc over which bullets fly */
#[no_mangle]
pub static mut sineHeightTable: [UBYTE; 100] = [0; 100];
//BOOL	bScreenShakeActive = FALSE;
//UDWORD	screenShakeStarted = 0;
//UDWORD	screenShakeLength = 0;
#[no_mangle]
pub unsafe extern "C" fn initBulletTable() {
    let mut i: UDWORD = 0;
    let mut height: UBYTE = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 100 as libc::c_int as libc::c_uint {
        height =
            (100 as libc::c_int as libc::c_double *
                 sin(i as libc::c_double * 3.141592657f64 /
                         100 as libc::c_int as libc::c_double)) as UBYTE;
        sineHeightTable[i as usize] = height;
        i = i.wrapping_add(1)
    };
}
//void	attemptScreenShake(void)
//{
//	if(!bScreenShakeActive)
//	{
//		bScreenShakeActive = TRUE;
//		screenShakeStarted = gameTime;
//		screenShakeLength = 1500;
//	}
//}
/* Angle returned is reflected in line x=0 */
#[no_mangle]
pub unsafe extern "C" fn calcDirection(mut x0: UDWORD, mut y0: UDWORD,
                                       mut x1: UDWORD, mut y1: UDWORD)
 -> SDWORD {
    let mut xDif: SDWORD = 0;
    let mut yDif: SDWORD = 0;
    let mut angleInt: SDWORD = 0;
    let mut angle: libc::c_double = 0.;
    angleInt = 0 as libc::c_int;
    xDif = x1.wrapping_sub(x0) as SDWORD;
    /* Watch out here - should really be y1-y0, but coordinate system is reversed in Y */
    yDif = y0.wrapping_sub(y1) as SDWORD;
    angle = atan2(yDif as libc::c_double, xDif as libc::c_double);
    angle = 180 as libc::c_int as libc::c_double * (angle / 3.141592657f64);
    angleInt = angle as SDWORD;
    angleInt += 90 as libc::c_int;
    if angleInt < 0 as libc::c_int { angleInt += 360 as libc::c_int }
    if angleInt >= 0 as libc::c_int && angleInt < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"calcDirection: droid direction out of range\x00" as *const u8
                  as *const libc::c_char);
    };
    if angleInt >= 0 as libc::c_int && angleInt < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"geometry.c\x00" as *const u8 as *const libc::c_char,
              91 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"calcDirection\x00")).as_ptr(),
              b"angleInt >= 0 && angleInt < 360\x00" as *const u8 as
                  *const libc::c_char);
    };
    return angleInt;
}
// -------------------------------------------------------------------------------------------
/*	A useful function and one that should have been written long ago, assuming of course
	that is hasn't been!!!! Alex M, 24th Sept, 1998. Returns the nearest unit
	to a given world coordinate - we can choose whether we require that the unit be
	selected or not... Makes sending the most logical unit to do something very easy.

  NB*****THIS WON'T PICK A VTOL DROID*****
*/
#[no_mangle]
pub unsafe extern "C" fn getNearestDroid(mut x: UDWORD, mut y: UDWORD,
                                         mut bSelected: BOOL) -> *mut DROID {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psBestUnit: *mut DROID = 0 as *mut DROID;
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut bestSoFar: UDWORD = 0;
    /* Go thru' all the droids  - how often have we seen this - a MACRO maybe? */
    psDroid = apsDroidLists[selectedPlayer as usize];
    psBestUnit = 0 as *mut DROID;
    bestSoFar = 0xffffffff as libc::c_uint;
    while !psDroid.is_null() {
        if vtolDroid(psDroid) == 0 {
            /* Clever (?) bit that reads whether we're interested in droids being selected or not */
            if if bSelected != 0 {
                   (*psDroid).selected as libc::c_int
               } else { 1 as libc::c_int } != 0 {
                /* Get the differences */
                xDif =
                    abs(((*psDroid).x as libc::c_uint).wrapping_sub(x) as
                            libc::c_int) as UDWORD;
                yDif =
                    abs(((*psDroid).y as libc::c_uint).wrapping_sub(y) as
                            libc::c_int) as UDWORD;
                /* Approximates the distance away - using a sqrt approximation */
                dist =
                    (if xDif > yDif {
                         xDif
                     } else {
                         yDif
                     }).wrapping_add((if xDif < yDif {
                                          xDif
                                      } else {
                                          yDif
                                      }).wrapping_div(2 as libc::c_int as
                                                          libc::c_uint)); // approximates, but never more than 11% out...
                /* Is this the nearest one we got so far? */
                if dist < bestSoFar {
                    /* Yes, then keep a record of the distance for comparison... */
                    bestSoFar = dist;
                    /* ..and store away the droid responsible */
                    psBestUnit = psDroid
                }
            }
        }
        psDroid = (*psDroid).psNext
    }
    return psBestUnit;
}
// -------------------------------------------------------------------------------------------
/* Returns non-zero if a point is in a 4 sided polygon */
/* See header file for definition of QUAD */
#[no_mangle]
pub unsafe extern "C" fn inQuad(mut pt: *mut POINT, mut quad: *mut QUAD)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    j = 3 as libc::c_int;
    while i < 4 as libc::c_int {
        if ((*quad).coords[i as usize].y <= (*pt).y &&
                (*pt).y < (*quad).coords[j as usize].y ||
                (*quad).coords[j as usize].y <= (*pt).y &&
                    (*pt).y < (*quad).coords[i as usize].y) &&
               (*pt).x <
                   ((*quad).coords[j as usize].x -
                        (*quad).coords[i as usize].x) *
                       ((*pt).y - (*quad).coords[i as usize].y) /
                       ((*quad).coords[j as usize].y -
                            (*quad).coords[i as usize].y) +
                       (*quad).coords[i as usize].x {
            c = (c == 0) as libc::c_int
        }
        let fresh0 = i;
        i = i + 1;
        j = fresh0
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn adjustDirection(mut present: SDWORD,
                                         mut difference: SDWORD) -> UDWORD {
    let mut sum: SDWORD = 0;
    sum = present + difference;
    if sum >= 0 as libc::c_int && sum <= 360 as libc::c_int {
        return sum as UDWORD
    }
    if sum < 0 as libc::c_int { return (360 as libc::c_int + sum) as UDWORD }
    if sum > 360 as libc::c_int {
        return (sum - 360 as libc::c_int) as UDWORD
    }
    return 0 as libc::c_int as UDWORD;
}
/* Return a signed difference in direction : a - b
 * result is 180 .. -180
 */
/* Return a signed difference in direction : a - b
 * result is 180 .. -180
 */
#[no_mangle]
pub unsafe extern "C" fn directionDiff(mut a: SDWORD, mut b: SDWORD)
 -> SDWORD {
    let mut diff: SDWORD = a - b;
    if diff > 180 as libc::c_int {
        return diff - 360 as libc::c_int
    } else {
        if diff < -(180 as libc::c_int) { return 360 as libc::c_int + diff }
    }
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn WorldPointToScreen(mut worldPt: *mut iPoint,
                                            mut screenPt: *mut iPoint) {
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut null: iVector = iVector{x: 0, y: 0, z: 0,};
    //MAPTILE	*psTile;
    let mut worldX: UDWORD = 0;
    let mut worldY: UDWORD = 0;
    let mut xShift: SDWORD = 0;
    let mut zShift: SDWORD = 0;
    let mut rx: int32 = 0;
    let mut rz: int32 = 0;
    /* Get into game context */
	/* Get the x,z translation components */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Push identity matrix onto stack */
    pie_MatBegin();
    /* Set the camera position */
    pie_MATTRANS(camera.p.x, camera.p.y, camera.p.z);
    /* Rotate for the player */
    pie_MatRotZ(player.r.z);
    pie_MatRotX(player.r.x);
    pie_MatRotY(player.r.y);
    /* Translate */
    pie_TRANSLATE(-rx, -player.p.y, rz);
    /* No rotation is necessary*/
    null.x = 0 as libc::c_int;
    null.y = 0 as libc::c_int;
    null.z = 0 as libc::c_int;
    /* Pull out coords now, because we use them twice */
    worldX = (*worldPt).x as UDWORD;
    worldY = (*worldPt).y as UDWORD;
    /* Get the coordinates of the object into the grid */
    vec.x =
        worldX.wrapping_sub(player.p.x as
                                libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint))
            as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(worldY.wrapping_sub(player.p.z
                                                                                        as
                                                                                        libc::c_uint))
            as int32;
    /* Which tile is it on? - In order to establish height (y coordinate in 3 space) */
//	psTile = mapTile(worldX/TILE_UNITS,worldY/TILE_UNITS);
//	vec.y = psTile->height;
    vec.y =
        map_Height(worldX.wrapping_div(128 as libc::c_int as libc::c_uint),
                   worldY.wrapping_div(128 as libc::c_int as libc::c_uint)) as
            int32;
    /* Set matrix context to local - get an identity matrix */
    pie_MatBegin();
    /* Translate */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    xShift = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    zShift = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Translate */
    pie_TRANSLATE(xShift, 0 as libc::c_int, -zShift);
    /* Project - no rotation being done. So effectively mapping from 3 space to 2 space */
    pie_RotProj(&mut null, screenPt);
    /* Pop remaining matrices */
    pie_MatEnd();
    pie_MatEnd();
}
//extern	BOOL	bScreenShakeActive;
//extern	UDWORD	screenShakeStarted;
//extern	UDWORD	screenShakeLength;
//extern	void	attemptScreenShake(void);
//UDWORD	getTileOwner(UDWORD	x, UDWORD y);
//BASE_OBJECT	*getTileOccupier(UDWORD x, UDWORD y);
//STRUCTURE	*getTileStructure(UDWORD x, UDWORD y);
//FEATURE		*getTileFeature(UDWORD x, UDWORD y);
/*	Calculates the RELATIVE screen coords of a game object from its BASE_OBJECT pointer */
/*	Alex - Saturday 5th July, 1997  */
/*	Returns result in POINT pt. They're relative in the sense that even if you pass
	a pointer to an object that isn't on screen, it'll still return a result - just that
	the coords may be negative or larger than screen dimensions in either (or both) axis (axes).
	Remember also, that the Y coordinate axis is reversed for our display in that increasing Y
	implies a movement DOWN the screen, and NOT up. */
#[no_mangle]
pub unsafe extern "C" fn baseObjScreenCoords(mut baseObj: *mut BASE_OBJECT,
                                             mut pt: *mut iPoint) {
    let mut worldPt: iPoint = iPoint{x: 0, y: 0,};
    worldPt.x = (*baseObj).x as int32;
    worldPt.y = (*baseObj).y as int32;
    WorldPointToScreen(&mut worldPt, pt);
}
/* Get the structure pointer for a specified tile coord. NULL if no structure */
#[no_mangle]
pub unsafe extern "C" fn getTileStructure(mut x: UDWORD, mut y: UDWORD)
 -> *mut STRUCTURE {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psReturn: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut centreX: UDWORD = 0;
    let mut centreY: UDWORD = 0;
    let mut strX: UDWORD = 0;
    let mut strY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut i: UDWORD = 0;
    /* No point in checking if there's no structure here! */
    if (*mapTile(x, y)).tileInfoBits as libc::c_int & 0x1 as libc::c_int == 0
       {
        return 0 as *mut STRUCTURE
    }
    /* Otherwise - see which one it is! */
    psReturn = 0 as *mut STRUCTURE;
    /* Get the world coords for the tile centre */
    centreX =
        (x <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    centreY =
        (y <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    /* Go thru' all players - drop out if match though */
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint && psReturn.is_null() {
        /* Got thru' all structures for this player - again drop out if match */
        psStructure = apsStructLists[i as usize];
        while !psStructure.is_null() && psReturn.is_null() {
            /* Get structure coords */
            strX = (*psStructure).x as UDWORD;
            strY = (*psStructure).y as UDWORD;
            /*			if((centreX > (strX-width)) AND (centreX < (strX+width)) )
			{
				if((centreY > (strY-breadth)) AND (centreY < (strY+breadth)) )
				{
					psReturn = psStructure;
				}
			}
*/
            width =
                (*(*psStructure).pStructureType).baseWidth.wrapping_mul(128 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint);
            breadth =
                (*(*psStructure).pStructureType).baseBreadth.wrapping_mul(128
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint);
            if centreX >
                   strX.wrapping_sub(width.wrapping_div(2 as libc::c_int as
                                                            libc::c_uint)) &&
                   centreX <
                       strX.wrapping_add(width.wrapping_div(2 as libc::c_int
                                                                as
                                                                libc::c_uint))
               {
                if centreY >
                       strY.wrapping_sub(breadth.wrapping_div(2 as libc::c_int
                                                                  as
                                                                  libc::c_uint))
                       &&
                       centreY <
                           strY.wrapping_add(breadth.wrapping_div(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                   {
                    psReturn = psStructure
                }
            }
            psStructure = (*psStructure).psNext
        }
        i = i.wrapping_add(1)
    }
    /* And extents */
    /* Within x boundary? */
    /* Send back either NULL or structure */
    return psReturn;
}
/* Sends back the feature on the specified tile - NULL if no feature */
#[no_mangle]
pub unsafe extern "C" fn getTileFeature(mut x: UDWORD, mut y: UDWORD)
 -> *mut FEATURE {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psReturn: *mut FEATURE = 0 as *mut FEATURE;
    let mut centreX: UDWORD = 0;
    let mut centreY: UDWORD = 0;
    let mut strX: UDWORD = 0;
    let mut strY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    //UDWORD		i;
    /* No point in checking if there's no feature here! */
    if (*mapTile(x, y)).tileInfoBits as libc::c_int & 0x2 as libc::c_int == 0
       {
        return 0 as *mut FEATURE
    }
    /* Otherwise - see which one it is! */
    psReturn = 0 as *mut FEATURE;
    /* Get the world coords for the tile centre */
    centreX =
        (x <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    centreY =
        (y <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    /* Go through all features for this player - again drop out if we get one */
    psFeature = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeature.is_null() && psReturn.is_null() {
        /* Get the features coords */
        strX = (*psFeature).x as UDWORD;
        strY = (*psFeature).y as UDWORD;
        /* And it's base dimensions */
        width =
            ((*(*psFeature).psStats).baseWidth as libc::c_int *
                 128 as libc::c_int) as UDWORD;
        breadth =
            ((*(*psFeature).psStats).baseBreadth as libc::c_int *
                 128 as libc::c_int) as UDWORD;
        /* Does tile centre lie within the area covered by base of feature? */
			/* First check for x */
        if centreX >
               strX.wrapping_sub(width.wrapping_div(2 as libc::c_int as
                                                        libc::c_uint)) &&
               centreX <
                   strX.wrapping_add(width.wrapping_div(2 as libc::c_int as
                                                            libc::c_uint)) {
            /* Got a match on the x - now try y */
            if centreY >
                   strY.wrapping_sub(breadth.wrapping_div(2 as libc::c_int as
                                                              libc::c_uint))
                   &&
                   centreY <
                       strY.wrapping_add(breadth.wrapping_div(2 as libc::c_int
                                                                  as
                                                                  libc::c_uint))
               {
                /* Got it! */
                psReturn = psFeature
            }
        }
        psFeature = (*psFeature).psNext
    }
    /* Send back either NULL or feature pointer */
    return psReturn;
}
/*	Will return a base_object pointer to either a structure or feature - depending
	what's on tile. Returns NULL if nothing */
#[no_mangle]
pub unsafe extern "C" fn getTileOccupier(mut x: UDWORD, mut y: UDWORD)
 -> *mut BASE_OBJECT {
    //DBPRINTF(("gto x=%d y=%d (%d,%d)\n",x,y,x*TILE_UNITS,y*TILE_UNITS);
	/* Firsty - check there is something on it?! */
    if (*mapTile(x, y)).tileInfoBits as libc::c_int &
           (0x1 as libc::c_int | 0x2 as libc::c_int | 0x20 as libc::c_int) ==
           0 {
        //DBPRINTF(("gto nothing\n");
		/* Nothing here at all! */
        return 0 as *mut BASE_OBJECT
    }
    /* Now check we can see it... */
    if (*mapTile(x, y)).tileVisBits as libc::c_int &
           (1 as libc::c_int) << selectedPlayer == 0 as libc::c_int {
        return 0 as *mut BASE_OBJECT
    }
    /* Has it got a fetaure? */
    if (*mapTile(x, y)).tileInfoBits as libc::c_int & 0x2 as libc::c_int != 0
       {
        //DBPRINTF(("gto feature\n");
		/* Return the feature */
        return getTileFeature(x, y) as *mut BASE_OBJECT
    } else {
        /*	Otherwise check for a structure - we can do else here since a tile cannot
		have both a feature and structure simultaneously */
        if (*mapTile(x, y)).tileInfoBits as libc::c_int & 0x1 as libc::c_int
               != 0 {
            //DBPRINTF(("gto structure\n");
		/* Send back structure pointer */
            return getTileStructure(x, y) as *mut BASE_OBJECT
        }
    }
    return 0 as *mut BASE_OBJECT;
}
/* Will return the player who presently has a structure on the specified tile */
#[no_mangle]
pub unsafe extern "C" fn getTileOwner(mut x: UDWORD, mut y: UDWORD)
 -> UDWORD {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut retVal: UDWORD = 0;
    /* Arbitrary error code - player 8 (non existent) owns tile from invalid request */
    retVal = 8 as libc::c_int as UDWORD;
    /* Check it has a structure - cannot have owner otherwise */
    if (*mapTile(x, y)).tileInfoBits as libc::c_int & 0x1 as libc::c_int == 0
       {
        debug(LOG_ERROR,
              b"Asking for the owner of a tile with no structure on it!!!\x00"
                  as *const u8 as *const libc::c_char);
        abort();
    } else {
        /* Get a pointer to the structure */
        psStruct = getTileStructure(x, y);
        /* Did we get one - failsafe really as TILE_HAS_STRUCTURE should get it */
        if !psStruct.is_null() {
            /* Pull out the player number */
            retVal = (*psStruct).player as UDWORD
        }
    }
    /* returns eith the player number or MAX_PLAYERS to signify error */
    return retVal;
}
#[no_mangle]
pub unsafe extern "C" fn getObjectsOnTile(mut psTile: *mut MAPTILE) { }
// Approximates a square root - never more than 11% out...
#[no_mangle]
pub unsafe extern "C" fn dirtySqrt(mut x1: SDWORD, mut y1: SDWORD,
                                   mut x2: SDWORD, mut y2: SDWORD) -> UDWORD {
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut retVal: UDWORD = 0;
    xDif = abs(x1 - x2) as UDWORD;
    yDif = abs(y1 - y2) as UDWORD;
    retVal =
        (if xDif > yDif {
             xDif
         } else {
             yDif
         }).wrapping_add((if xDif < yDif {
                              xDif
                          } else {
                              yDif
                          }).wrapping_div(2 as libc::c_int as libc::c_uint));
    return retVal;
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn droidOnScreen(mut psDroid: *mut DROID,
                                       mut tolerance: SDWORD) -> BOOL {
    let mut dX: SDWORD = 0;
    let mut dY: SDWORD = 0;
    if DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD) ==
           1 as libc::c_int {
        dX = (*psDroid).sDisplay.screenX as SDWORD;
        dY = (*psDroid).sDisplay.screenY as SDWORD;
        /* Is it on screen */
        if dX > 0 as libc::c_int - tolerance &&
               dY > 0 as libc::c_int - tolerance &&
               dX <
                   pie_GetVideoBufferWidth().wrapping_add(tolerance as
                                                              libc::c_uint) as
                       SDWORD &&
               dY <
                   pie_GetVideoBufferHeight().wrapping_add(tolerance as
                                                               libc::c_uint)
                       as SDWORD {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn processImpact(mut worldX: UDWORD, mut worldY: UDWORD,
                                       mut severity: UBYTE,
                                       mut tilesAcross: UDWORD) {
    //MAPTILE	*psTile;
    let mut height: UDWORD = 0;
    let mut newHeight: SDWORD = 0;
    let mut distance: UDWORD = 0;
    let mut multiplier: libc::c_float = 0.;
    let mut damage: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut tileX: SDWORD = 0;
    let mut tileY: SDWORD = 0;
    let mut maxDisplacement: UDWORD = 0;
    let mut maxDistance: UDWORD = 0;
    if (severity as libc::c_int) < 255 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Damage is too severe\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (severity as libc::c_int) < 255 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"geometry.c\x00" as *const u8 as *const libc::c_char,
              532 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"processImpact\x00")).as_ptr(),
              b"severity<MAX_TILE_DAMAGE\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* Make sure it's odd */
    if tilesAcross & 0x1 as libc::c_int as libc::c_uint == 0 {
        tilesAcross =
            (tilesAcross as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    tileX =
        (worldX >>
             7 as
                 libc::c_int).wrapping_sub(tilesAcross.wrapping_div(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as SDWORD;
    tileY =
        (worldY >>
             7 as
                 libc::c_int).wrapping_sub(tilesAcross.wrapping_div(2 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_sub(1
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint))
            as SDWORD;
    maxDisplacement =
        tilesAcross.wrapping_div(2 as libc::c_int as
                                     libc::c_uint).wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint).wrapping_mul(128
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint);
    maxDisplacement =
        (maxDisplacement as libc::c_float * 1.42f64 as libc::c_float) as
            UDWORD;
    maxDistance =
        sqrt((maxDisplacement as libc::c_float *
                  maxDisplacement as libc::c_float) as libc::c_double) as
            UDWORD;
    if tileX < 0 as libc::c_int { tileX = 0 as libc::c_int }
    if tileY < 0 as libc::c_int { tileY = 0 as libc::c_int }
    i = tileX as UDWORD;
    while i <
              (tileX as
                   libc::c_uint).wrapping_add(tilesAcross).wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
          {
        j = tileY as UDWORD;
        while j <
                  (tileY as
                       libc::c_uint).wrapping_add(tilesAcross).wrapping_sub(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
              {
            /* Only process tiles that are on the map */
            if tileX < mapWidth as SDWORD && tileY < mapHeight as SDWORD {
                xDif =
                    abs(worldX.wrapping_sub(i << 7 as libc::c_int) as
                            libc::c_int) as UDWORD;
                yDif =
                    abs(worldY.wrapping_sub(j << 7 as libc::c_int) as
                            libc::c_int) as UDWORD;
                distance =
                    sqrt((xDif.wrapping_mul(xDif) as libc::c_float +
                              yDif.wrapping_mul(yDif) as libc::c_float) as
                             libc::c_double) as UDWORD;
                multiplier =
                    1 as libc::c_int as libc::c_float -
                        distance as libc::c_float /
                            maxDistance as libc::c_float;
                multiplier =
                    (1.0f64 -
                         (distance as libc::c_float /
                              maxDistance as libc::c_float) as libc::c_double)
                        as libc::c_float;
                /* Are we talking less than 15% damage? i.e - at the edge of carater? */
                if (multiplier as libc::c_double) < 0.15f64 {
                    /* In which case make the crater edge have jagged edges */
                    multiplier +=
                        ((20 as libc::c_int - rand() % 40 as libc::c_int) as
                             libc::c_float as libc::c_double * 0.01f64) as
                            libc::c_float
                }
                height = (*mapTile(i, j)).height as UDWORD;
                damage = (severity as libc::c_float * multiplier) as UDWORD;
                newHeight = height.wrapping_sub(damage) as SDWORD;
                if newHeight < 0 as libc::c_int {
                    newHeight = 0 as libc::c_int
                }
                setTileHeight(i, j, newHeight as UDWORD);
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
