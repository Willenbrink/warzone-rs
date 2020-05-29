use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn pie_GetFogEnabled() -> BOOL;
    #[no_mangle]
    fn pie_GetFogColour() -> UDWORD;
    #[no_mangle]
    fn pie_ByteScale(a: UBYTE, b: UBYTE) -> UBYTE;
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    //*************************************************************************
    #[no_mangle]
    fn pie_VectorNormalise(v: *mut iVector);
    #[no_mangle]
    fn pie_SurfaceNormal(p1: *mut iVector, p2: *mut iVector, p3: *mut iVector,
                         v: *mut iVector);
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    static mut playerXTile: int32;
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut tileScreenInfo: [[SVMESH; 33]; 33];
    #[no_mangle]
    static mut playerZTile: int32;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    #[no_mangle]
    fn environGetData(x: UDWORD, y: UDWORD) -> UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PIELIGHTBYTES {
    pub b: UBYTE,
    pub g: UBYTE,
    pub r: UBYTE,
    pub a: UBYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union PIELIGHT {
    pub byte: PIELIGHTBYTES,
    pub argb: UDWORD,
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
//works as all of the above together! - new for updates - added 11/06/99 AB
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
pub type MAPTILE = _maptile;
pub type _lightcols = libc::c_uint;
pub const LIGHT_WHITE: _lightcols = 4;
pub const LIGHT_YELLOW: _lightcols = 3;
pub const LIGHT_BLUE: _lightcols = 2;
pub const LIGHT_GREEN: _lightcols = 1;
pub const LIGHT_RED: _lightcols = 0;
pub type LIGHT_COLOUR = _lightcols;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light {
    pub position: iVector,
    pub type_0: UBYTE,
    pub range: UDWORD,
    pub colour: UDWORD,
}
pub type LIGHT = _light;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SVMESH {
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub tu: UWORD,
    pub tv: UWORD,
    pub light: PIELIGHT,
    pub specular: PIELIGHT,
    pub sx: SDWORD,
    pub sy: SDWORD,
    pub sz: SDWORD,
    pub wx: SDWORD,
    pub wy: SDWORD,
    pub wz: SDWORD,
    pub water_height: SDWORD,
    pub wlight: PIELIGHT,
    pub drawInfo: UBYTE,
    pub bWater: UBYTE,
}
#[inline]
unsafe extern "C" fn map_TileHeight(mut x: UDWORD, mut y: UDWORD) -> SWORD {
    if x >= mapWidth || y >= mapHeight { return 0 as libc::c_int as SWORD }
    return ((*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                                    isize)).height as libc::c_int *
                2 as libc::c_int) as SWORD;
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
/* Lighting.c - Alex McLean, Pumpkin Studios, EIDOS Interactive. */
/* Calculates the shading values for the terrain world. */
/* The terrain intensity values are calculated at map load/creation time. */
/*	The vector that holds the sun's lighting direction - planar */
#[no_mangle]
pub static mut theSun: iVector = iVector{x: 0, y: 0, z: 0,};
#[no_mangle]
pub static mut fogStatus: UDWORD = 0 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut numNormals: UDWORD = 0;
// How many normals have we got?
#[no_mangle]
pub static mut normals: [iVector; 8] = [iVector{x: 0, y: 0, z: 0,}; 8];
//extern void	initLighting( void );
/* ****************************************************************************/
/*
 * SOURCE
 */
/* ****************************************************************************/
/*Rewrote the function so it takes parameters and also doesn't loop thru'
the map 4 times!*/
/*void initLighting( void )
{
UDWORD	i,j;
MAPTILE	*psTile;

	for(i=0; i<mapWidth; i++)
	{
		for(j=0; j<mapHeight; j++)
		{
			mapTile(i,j)->illumination = 16;
		}
	}

	//for(i=2; i<mapHeight-2; i++)
	//{
	//	for(j=2; j<mapWidth-2; j++)
	//	{
	//		calcTileIllum(j,i);
	//	}
	//}

	for(i=1; i<mapHeight-1; i++)
	{
		for(j=1; j<mapWidth-1; j++)
		{
			calcTileIllum(j,i);
		}
	}

	for(i=0; i<mapWidth; i++)
	{
		for(j=0; j<mapHeight; j++)
		{
			if(i==0 OR j==0 OR i>=mapWidth-1 OR j>=mapHeight-1)
			{
//				mapTile(i,j)->height = 0;
				psTile = mapTile(i,j);
				if(TERRAIN_TYPE(psTile) == TER_WATER)
				{
					psTile->texture = 0;
				}
			}
		}
	}

	// Cheers to paul for this idea - works on PC too
	//	Basically darkens down the tiles that are outside the scroll
	//	limits - thereby emphasising the cannot-go-there-ness of them
	for(i=0; i<mapWidth; i++)
	{
		for(j=0; j<mapHeight; j++)
		{
			if(i<(scrollMinX+4) OR i>(scrollMaxX-4) OR j<(scrollMinY+4) OR j>(scrollMaxY-4))
			{
				mapTile(i,j)->illumination/=3;
			}
		}
	}
}*/
//should do the same as above except cuts down on the loop count!
//By passing in params - it means that if the scroll limits are changed mid-mission
//we can re-do over the area that hasn't been seen
#[no_mangle]
pub unsafe extern "C" fn initLighting(mut x1: UDWORD, mut y1: UDWORD,
                                      mut x2: UDWORD, mut y2: UDWORD) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //quick check not trying to go off the map - don't need to check for < 0 since UWORD's!!
    if x1 > mapWidth || x2 > mapWidth || y1 > mapHeight || y2 > mapHeight {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"initLighting: coords off edge of map\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"lighting.c\x00" as *const u8 as *const libc::c_char,
                  122 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"initLighting\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    i = x1;
    while i < x2 {
        j = y1;
        while j < y2 {
            psTile = mapTile(i, j);
            //always make the edge tiles dark
            if i == 0 as libc::c_int as libc::c_uint ||
                   j == 0 as libc::c_int as libc::c_uint ||
                   i >=
                       mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint)
                   ||
                   j >=
                       mapHeight.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) {
                (*psTile).illumination = 16 as libc::c_int as UBYTE;
                //give water tiles at edge of map a border
                if terrainTypes[((*psTile).texture as libc::c_int &
                                     0x1ff as libc::c_int) as usize] as
                       libc::c_int == TER_WATER as libc::c_int {
                    (*psTile).texture = 0 as libc::c_int as UWORD
                }
            } else { calcTileIllum(i, j); }
            // Cheers to paul for this idea - works on PC too
    	    //	Basically darkens down the tiles that are outside the scroll
	        //	limits - thereby emphasising the cannot-go-there-ness of them
            if (i as SDWORD) < scrollMinX + 4 as libc::c_int ||
                   i as SDWORD > scrollMaxX - 4 as libc::c_int ||
                   (j as SDWORD) < scrollMinY + 4 as libc::c_int ||
                   j as SDWORD > scrollMaxY - 4 as libc::c_int {
                (*psTile).illumination =
                    ((*psTile).illumination as libc::c_int / 3 as libc::c_int)
                        as UBYTE
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
//void	initLighting( void );
#[no_mangle]
pub unsafe extern "C" fn calcTileIllum(mut tileX: UDWORD, mut tileY: UDWORD) {
    let mut finalVector: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut dotProduct: SDWORD = 0;
    let mut i: UDWORD = 0;
    let mut val: UDWORD = 0;
    numNormals = 0 as libc::c_int as UDWORD;
    /* Quadrants look like:-

				  *
				  *
			0	  *    1
				  *
				  *
		**********V**********
				  *
				  *
			3	  *	   2
				  *
				  *
	*/
    /* Do quadrant 0 - tile that's above and left*/
    normalsOnTile(tileX.wrapping_sub(1 as libc::c_int as libc::c_uint),
                  tileY.wrapping_sub(1 as libc::c_int as libc::c_uint),
                  0 as libc::c_int as UDWORD);
    /* Do quadrant 1 - tile that's above and right*/
    normalsOnTile(tileX, tileY.wrapping_sub(1 as libc::c_int as libc::c_uint),
                  1 as libc::c_int as UDWORD);
    /* Do quadrant 2 - tile that's down and right*/
    normalsOnTile(tileX, tileY, 2 as libc::c_int as UDWORD);
    /* Do quadrant 3 - tile that's down and left*/
    normalsOnTile(tileX.wrapping_sub(1 as libc::c_int as libc::c_uint), tileY,
                  3 as libc::c_int as UDWORD);
    /* The number or normals that we got is in numNormals*/
    finalVector.z = 0 as libc::c_int;
    finalVector.y = finalVector.z;
    finalVector.x = finalVector.y;
    i = 0 as libc::c_int as UDWORD;
    while i < numNormals {
        finalVector.x += normals[i as usize].x;
        finalVector.y += normals[i as usize].y;
        finalVector.z += normals[i as usize].z;
        i = i.wrapping_add(1)
    }
    pie_VectorNormalise(&mut finalVector);
    pie_VectorNormalise(&mut theSun);
    //	iV_NumberOut(theSun.x,100,100,255);
//	iV_NumberOut(theSun.y,100,110,255);
//	iV_NumberOut(theSun.z,100,120,255);
    //	iV_NumberOut(numNormals,100,140,255);
    dotProduct =
        finalVector.x * theSun.x + finalVector.y * theSun.y +
            finalVector.z * theSun.z >> 12 as libc::c_int;
    /* iV_NumberOut(dotProduct,100,150,255);*/
    val = (abs(dotProduct) / 16 as libc::c_int) as UDWORD;
    if val == 0 as libc::c_int as libc::c_uint {
        val = 1 as libc::c_int as UDWORD
    }
    if val > 254 as libc::c_int as libc::c_uint {
        val = 254 as libc::c_int as UDWORD
    }
    (*mapTile(tileX, tileY)).illumination = val as UBYTE;
}
#[no_mangle]
pub unsafe extern "C" fn normalsOnTile(mut tileX: UDWORD, mut tileY: UDWORD,
                                       mut quadrant: UDWORD) {
    let mut corner1: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut corner2: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut corner3: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut tileRight: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut tileDownRight: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut tileDown: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut rMod: SDWORD = 0;
    let mut drMod: SDWORD = 0;
    let mut dMod: SDWORD = 0;
    let mut nMod: SDWORD = 0;
    /* Get a pointer to our tile */
    psTile = mapTile(tileX, tileY);
    /* And to the ones to the east, south and southeast of it */
    tileRight =
        mapTile(tileX.wrapping_add(1 as libc::c_int as libc::c_uint), tileY);
    tileDownRight =
        mapTile(tileX.wrapping_add(1 as libc::c_int as libc::c_uint),
                tileY.wrapping_add(1 as libc::c_int as libc::c_uint));
    tileDown =
        mapTile(tileX, tileY.wrapping_add(1 as libc::c_int as libc::c_uint));
    if terrainTypes[((*psTile).texture as libc::c_int & 0x1ff as libc::c_int)
                        as usize] as libc::c_int == TER_WATER as libc::c_int {
        nMod =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(environGetData(tileX,
                                                                                           tileY)))
                as SDWORD;
        rMod =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(environGetData(tileX.wrapping_add(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint),
                                                                                           tileY)))
                as SDWORD;
        drMod =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(environGetData(tileX.wrapping_add(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint),
                                                                                           tileY.wrapping_add(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint))))
                as SDWORD;
        dMod =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_add((2 as libc::c_int as
                                                 libc::c_uint).wrapping_mul(environGetData(tileX,
                                                                                           tileY.wrapping_add(1
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint))))
                as SDWORD
    } else {
        dMod = 0 as libc::c_int;
        drMod = dMod;
        rMod = drMod;
        nMod = rMod
    }
    match quadrant {
        0 | 2 => {
            /* Is it flipped? In this case one triangle  */
//		if(psTile->triangleFlip)
            if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
                if quadrant == 0 as libc::c_int as libc::c_uint {
                    corner1.x =
                        (tileX.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner1.y = (tileY << 7 as libc::c_int) as int32;
                    corner1.z = (*tileRight).height as libc::c_int - rMod;
                    corner2.x =
                        (tileX.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner2.y =
                        (tileY.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner2.z =
                        (*tileDownRight).height as libc::c_int - drMod;
                    corner3.x = (tileX << 7 as libc::c_int) as int32;
                    corner3.y =
                        (tileY.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner3.z = (*tileDown).height as libc::c_int - dMod;
                    let fresh0 = numNormals;
                    numNormals = numNormals.wrapping_add(1);
                    pie_SurfaceNormal(&mut corner1, &mut corner2,
                                      &mut corner3,
                                      &mut *normals.as_mut_ptr().offset(fresh0
                                                                            as
                                                                            isize));
                } else {
                    corner1.x = (tileX << 7 as libc::c_int) as int32;
                    corner1.y = (tileY << 7 as libc::c_int) as int32;
                    corner1.z = (*psTile).height as libc::c_int - nMod;
                    corner2.x =
                        (tileX.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner2.y = (tileY << 7 as libc::c_int) as int32;
                    corner2.z = (*tileRight).height as libc::c_int - rMod;
                    corner3.x = (tileX << 7 as libc::c_int) as int32;
                    corner3.y =
                        (tileY.wrapping_add(1 as libc::c_int as libc::c_uint)
                             << 7 as libc::c_int) as int32;
                    corner3.z = (*tileDown).height as libc::c_int - dMod;
                    let fresh1 = numNormals;
                    numNormals = numNormals.wrapping_add(1);
                    pie_SurfaceNormal(&mut corner1, &mut corner2,
                                      &mut corner3,
                                      &mut *normals.as_mut_ptr().offset(fresh1
                                                                            as
                                                                            isize));
                }
            } else {
                /* Otherwise, it's not flipped and so two triangles*/
                corner1.x = (tileX << 7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*psTile).height as libc::c_int - nMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y = (tileY << 7 as libc::c_int) as int32;
                corner2.z = (*tileRight).height as libc::c_int - rMod;
                corner3.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDownRight).height as libc::c_int - drMod;
                let fresh2 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh2 as
                                                                        isize));
                corner1.x = (tileX << 7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*psTile).height as libc::c_int - nMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.z = (*tileDownRight).height as libc::c_int - drMod;
                corner3.x = (tileX << 7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDown).height as libc::c_int - dMod;
                let fresh3 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh3 as
                                                                        isize));
            }
        }
        1 | 3 => {
            /* Is it flipped? In this case two triangles  */
//		if(psTile->triangleFlip)
            if (*psTile).texture as libc::c_int & 0x800 as libc::c_int != 0 {
                corner1.x = (tileX << 7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*psTile).height as libc::c_int - nMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y = (tileY << 7 as libc::c_int) as int32;
                corner2.z = (*tileRight).height as libc::c_int - rMod;
                corner3.x = (tileX << 7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDown).height as libc::c_int - dMod;
                let fresh4 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh4 as
                                                                        isize));
                corner1.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*tileRight).height as libc::c_int - rMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.z = (*tileDownRight).height as libc::c_int - drMod;
                corner3.x = (tileX << 7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDown).height as libc::c_int - dMod;
                let fresh5 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh5 as
                                                                        isize));
            } else if quadrant == 1 as libc::c_int as libc::c_uint {
                corner1.x = (tileX << 7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*psTile).height as libc::c_int - nMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.z = (*tileDownRight).height as libc::c_int - drMod;
                corner3.x = (tileX << 7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDown).height as libc::c_int - dMod;
                let fresh6 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh6 as
                                                                        isize));
            } else {
                corner1.x = (tileX << 7 as libc::c_int) as int32;
                corner1.y = (tileY << 7 as libc::c_int) as int32;
                corner1.z = (*psTile).height as libc::c_int - nMod;
                corner2.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner2.y = (tileY << 7 as libc::c_int) as int32;
                corner2.z = (*tileRight).height as libc::c_int - rMod;
                corner3.x =
                    (tileX.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.y =
                    (tileY.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as libc::c_int) as int32;
                corner3.z = (*tileDownRight).height as libc::c_int - drMod;
                let fresh7 = numNormals;
                numNormals = numNormals.wrapping_add(1);
                pie_SurfaceNormal(&mut corner1, &mut corner2, &mut corner3,
                                  &mut *normals.as_mut_ptr().offset(fresh7 as
                                                                        isize));
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Invalid quadrant in lighting code\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"lighting.c\x00" as *const u8 as *const libc::c_char,
                      387 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"normalsOnTile\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
    // end switch
}
#[no_mangle]
pub unsafe extern "C" fn processLight(mut psLight: *mut LIGHT) {
    let mut tileX: SDWORD = 0;
    let mut tileY: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut rangeSkip: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut distToCorner: SDWORD = 0;
    let mut xIndex: SDWORD = 0;
    let mut yIndex: SDWORD = 0;
    let mut percent: UDWORD = 0;
    /* Firstly - there's no point processing lights that are off the grid */
    if clipXY((*psLight).position.x, (*psLight).position.z) ==
           0 as libc::c_int {
        return
    }
    tileX = (*psLight).position.x / 128 as libc::c_int;
    tileY = (*psLight).position.z / 128 as libc::c_int;
    rangeSkip = (*psLight).range.wrapping_mul((*psLight).range) as SDWORD;
    rangeSkip *= 2 as libc::c_int;
    rangeSkip = sqrt(rangeSkip as libc::c_double) as UDWORD as SDWORD;
    rangeSkip /= 128 as libc::c_int;
    rangeSkip += 1 as libc::c_int;
    /* Rough guess? */
    startX = tileX - rangeSkip;
    endX = tileX + rangeSkip;
    startY = tileY - rangeSkip;
    endY = tileY + rangeSkip;
    /* Clip to grid limits */
    if startX < 0 as libc::c_int {
        startX = 0 as libc::c_int
    } else if startX >
                  mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      SDWORD {
        startX =
            mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    if endX < 0 as libc::c_int {
        endX = 0 as libc::c_int
    } else if endX >
                  mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      SDWORD {
        endX =
            mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    /* Clip to grid limits */
    if startY < 0 as libc::c_int {
        startY = 0 as libc::c_int
    } else if startY >
                  mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      SDWORD {
        startY =
            mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    if endY < 0 as libc::c_int {
        endY = 0 as libc::c_int
    } else if endY >
                  mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                      SDWORD {
        endY =
            mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as SDWORD
    }
    i = startX;
    while i <= endX {
        j = startY;
        while j <= endY {
            distToCorner =
                calcDistToTile(i as UDWORD, j as UDWORD,
                               &mut (*psLight).position) as SDWORD;
            /* If we're inside the range of the light */
            if distToCorner < (*psLight).range as SDWORD {
                /* Find how close we are to it */
                percent =
                    (100 as libc::c_int as
                         libc::c_uint).wrapping_sub(((distToCorner *
                                                          100 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_div((*psLight).range));
                xIndex = i - playerXTile;
                yIndex = j - playerZTile;
                // Might go off the grid for light ranges > one tile
//					if(i<visibleXTiles AND j<visibleYTiles AND i>=0 AND j>=0)
                if xIndex >= 0 as libc::c_int && yIndex >= 0 as libc::c_int &&
                       xIndex < visibleXTiles as SDWORD &&
                       yIndex < visibleYTiles as SDWORD {
                    colourTile(xIndex, yIndex,
                               (*psLight).colour as LIGHT_COLOUR,
                               (2 as libc::c_int as
                                    libc::c_uint).wrapping_mul(percent) as
                                   UBYTE);
                }
            }
            j += 1
        }
        i += 1
    };
}
/*
UDWORD	calcDistToTile(UDWORD tileX, UDWORD tileY, iVector *pos)
{
UDWORD	x1,y1;
UDWORD	x2,y2;
UDWORD	xDif,yDif,zDif;
UDWORD	total;

	x1 = tileX * TILE_UNITS;
	y1 = tileY * TILE_UNITS;

	x2 = pos->x;
	y2 = pos->z;

	xDif = abs(x1-x2);
	zDif = abs(y1-y2);

	total = (xDif*xDif) + (yDif*yDif);
	return((UDWORD)sqrt(total));
}
*/
#[no_mangle]
pub unsafe extern "C" fn calcDistToTile(mut tileX: UDWORD, mut tileY: UDWORD,
                                        mut pos: *mut iVector) -> UDWORD {
    let mut x1: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut z1: UDWORD = 0;
    let mut x2: UDWORD = 0;
    let mut y2: UDWORD = 0;
    let mut z2: UDWORD = 0;
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut zDif: UDWORD = 0;
    let mut total: UDWORD = 0;
    /* The coordinates of the tile corner */
    x1 = tileX.wrapping_mul(128 as libc::c_int as libc::c_uint);
    y1 = map_TileHeight(tileX, tileY) as UDWORD;
    z1 = tileY.wrapping_mul(128 as libc::c_int as libc::c_uint);
    /* The coordinates of the position */
    x2 = (*pos).x as UDWORD;
    y2 = (*pos).y as UDWORD;
    z2 = (*pos).z as UDWORD;
    xDif = abs(x1.wrapping_sub(x2) as libc::c_int) as UDWORD;
    yDif = abs(y1.wrapping_sub(y2) as libc::c_int) as UDWORD;
    zDif = abs(z1.wrapping_sub(z2) as libc::c_int) as UDWORD;
    total =
        xDif.wrapping_mul(xDif).wrapping_add(yDif.wrapping_mul(yDif)).wrapping_add(zDif.wrapping_mul(zDif));
    return sqrt(total as libc::c_double) as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn colourTile(mut xIndex: SDWORD, mut yIndex: SDWORD,
                                    mut colour: LIGHT_COLOUR,
                                    mut percent: UBYTE) {
    if xIndex < 32 as libc::c_int + 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"X Colour Value out of range (above) for lighting\x00" as
                  *const u8 as *const libc::c_char);
    };
    if xIndex < 32 as libc::c_int + 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"lighting.c\x00" as *const u8 as *const libc::c_char,
              613 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"colourTile\x00")).as_ptr(),
              b"xIndex<LAND_XGRD\x00" as *const u8 as *const libc::c_char);
    };
    if yIndex < 32 as libc::c_int + 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Y Colour Value out of range (above)for lighting\x00" as
                  *const u8 as *const libc::c_char);
    };
    if yIndex < 32 as libc::c_int + 1 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"lighting.c\x00" as *const u8 as *const libc::c_char,
              614 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"colourTile\x00")).as_ptr(),
              b"yIndex<LAND_YGRD\x00" as *const u8 as *const libc::c_char);
    };
    if xIndex >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"X Colour Value out of range (below) for lighting\x00" as
                  *const u8 as *const libc::c_char);
    };
    if xIndex >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"lighting.c\x00" as *const u8 as *const libc::c_char,
              615 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"colourTile\x00")).as_ptr(),
              b"xIndex>=0\x00" as *const u8 as *const libc::c_char);
    };
    if yIndex >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Y Colour Value out of range (below )for lighting\x00" as
                  *const u8 as *const libc::c_char);
    };
    if yIndex >= 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"lighting.c\x00" as *const u8 as *const libc::c_char,
              616 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 11],
                                        &[libc::c_char; 11]>(b"colourTile\x00")).as_ptr(),
              b"yIndex>=0\x00" as *const u8 as *const libc::c_char);
    };
    match colour as libc::c_uint {
        0 => {
            /* And add that to the lighting value */
            if tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                   as libc::c_int + percent as libc::c_int <=
                   255 as libc::c_int {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.r
                         as libc::c_int + percent as libc::c_int) as UBYTE
            } else {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    = 255 as libc::c_int as UBYTE
            }
        }
        1 => {
            /* And add that to the lighting value */
            if tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                   as libc::c_int + percent as libc::c_int <=
                   255 as libc::c_int {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.g
                         as libc::c_int + percent as libc::c_int) as UBYTE
            } else {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    = 255 as libc::c_int as UBYTE
            }
        }
        2 => {
            /* And add that to the lighting value */
            if tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.b
                   as libc::c_int + percent as libc::c_int <=
                   255 as libc::c_int {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.b
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.b
                         as libc::c_int + percent as libc::c_int) as UBYTE
            } else {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.b
                    = 255 as libc::c_int as UBYTE
            }
        }
        3 => {
            /* And add that to the lighting value */
            if tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                   as libc::c_int + percent as libc::c_int <=
                   255 as libc::c_int {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.r
                         as libc::c_int + percent as libc::c_int) as UBYTE;
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.g
                         as libc::c_int + percent as libc::c_int) as UBYTE
            } else {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    = 255 as libc::c_int as UBYTE;
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    = 255 as libc::c_int as UBYTE
            }
        }
        4 => {
            /* And add that to the lighting value */
            if tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                   as libc::c_int + percent as libc::c_int <=
                   255 as libc::c_int {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.r
                         as libc::c_int + percent as libc::c_int) as
                        UBYTE; // sum of light vals
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.g
                         as libc::c_int + percent as libc::c_int) as UBYTE;
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.b
                    =
                    (tileScreenInfo[yIndex as
                                        usize][xIndex as usize].light.byte.b
                         as libc::c_int + percent as libc::c_int) as UBYTE
            } else {
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.r
                    = 255 as libc::c_int as UBYTE;
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.g
                    = 255 as libc::c_int as UBYTE;
                tileScreenInfo[yIndex as usize][xIndex as usize].light.byte.b
                    = 255 as libc::c_int as UBYTE
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy colour of light attempted\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"lighting.c\x00" as *const u8 as *const libc::c_char,
                      683 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 11],
                                                &[libc::c_char; 11]>(b"colourTile\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn calcDroidIllumination(mut psDroid: *mut DROID) {
    let mut lightVal: UDWORD = 0;
    let mut presVal: UDWORD = 0;
    let mut tileX: UDWORD = 0;
    let mut tileY: UDWORD = 0;
    let mut retVal: UDWORD = 0;
    let mut fraction: libc::c_float = 0.;
    let mut adjust: libc::c_float = 0.;
    /* Establish how long the last game frame took */
    fraction = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
    /* See if the droid's at the edge of the map */
    tileX = ((*psDroid).x as libc::c_int / 128 as libc::c_int) as UDWORD;
    tileY = ((*psDroid).y as libc::c_int / 128 as libc::c_int) as UDWORD;
    /* Are we at the edge */
    if tileX <= 1 as libc::c_int as libc::c_uint ||
           tileX >= mapWidth.wrapping_sub(2 as libc::c_int as libc::c_uint) ||
           tileY <= 1 as libc::c_int as libc::c_uint ||
           tileY >= mapHeight.wrapping_sub(2 as libc::c_int as libc::c_uint) {
        lightVal = (*mapTile(tileX, tileY)).illumination as UDWORD; //
        lightVal =
            (lightVal as
                 libc::c_uint).wrapping_add(96 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    } else {
        lightVal =
            ((*mapTile(tileX, tileY)).illumination as libc::c_int +
                 (*mapTile(tileX.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint),
                           tileY)).illumination as libc::c_int +
                 (*mapTile(tileX,
                           tileY.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint))).illumination
                     as libc::c_int +
                 (*mapTile(tileX.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint),
                           tileY)).illumination as libc::c_int +
                 (*mapTile(tileX.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint),
                           tileY.wrapping_add(1 as libc::c_int as
                                                  libc::c_uint))).illumination
                     as libc::c_int) as UDWORD;
        lightVal =
            (lightVal as
                 libc::c_uint).wrapping_div(5 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        lightVal =
            (lightVal as
                 libc::c_uint).wrapping_add(96 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    /* Saturation */
    if lightVal > 255 as libc::c_int as libc::c_uint {
        lightVal = 255 as libc::c_int as UDWORD
    }
    presVal = (*psDroid).illumination as UDWORD;
    adjust = lightVal as FRACT - presVal as FRACT;
    adjust *= fraction * 2 as libc::c_int as libc::c_float;
    retVal = presVal.wrapping_add(adjust as SDWORD as libc::c_uint);
    if retVal > 255 as libc::c_int as libc::c_uint {
        retVal = 255 as libc::c_int as UDWORD
    }
    (*psDroid).illumination = retVal as UBYTE;
}
/*	Module function Prototypes */
//#define EDGE_FOG
#[no_mangle]
pub unsafe extern "C" fn lightDoFogAndIllumination(mut brightness: UBYTE,
                                                   mut dx: SDWORD,
                                                   mut dz: SDWORD,
                                                   mut pSpecular: *mut UDWORD)
 -> UDWORD {
    let mut umbraRadius: SDWORD = 0; // Distance to start of light falloff
    let mut penumbraRadius: SDWORD = 0; // radius of area of obscurity
    let mut umbra: SDWORD = 0;
    //SDWORD	edge;
    let mut distance: SDWORD = 0;
    let mut cosA: SDWORD = 0;
    let mut sinA: SDWORD = 0;
    let mut lighting: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut specular: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut fogColour: PIELIGHT =
        PIELIGHT{byte: PIELIGHTBYTES{b: 0, g: 0, r: 0, a: 0,},};
    let mut depth: SDWORD = 0 as libc::c_int;
    let mut colour: SDWORD = 0;
    let mut fog: SDWORD = 0 as libc::c_int;
    //SDWORD	mist = 0;
    distance = sqrt((dx * dx + dz * dz) as libc::c_double) as SDWORD;
    penumbraRadius =
        (visibleXTiles.wrapping_div(2 as libc::c_int as libc::c_uint) <<
             7 as libc::c_int) as SDWORD;
    umbraRadius = penumbraRadius - 384 as libc::c_int;
    if distance < umbraRadius {
        umbra = 255 as libc::c_int
    } else if distance > penumbraRadius {
        umbra = 0 as libc::c_int
    } else {
        umbra =
            255 as libc::c_int -
                (distance - umbraRadius) * 255 as libc::c_int /
                    384 as libc::c_int
    }
    if distance < 32 as libc::c_int { depth = 1 as libc::c_int }
    if fogStatus & 2 as libc::c_int as libc::c_uint != 0 ||
           fogStatus & 1 as libc::c_int as libc::c_uint != 0 {
        //add fog
        if pie_GetFogEnabled() != 0 {
            cosA =
                *aSinTable.as_mut_ptr().offset(((player.r.y as uint16 as
                                                     libc::c_int >>
                                                     4 as libc::c_int) +
                                                    1024 as libc::c_int) as
                                                   isize);
            sinA =
                *aSinTable.as_mut_ptr().offset((player.r.y as uint16 as
                                                    libc::c_int >>
                                                    4 as libc::c_int) as
                                                   isize);
            depth = sinA * dx + cosA * dz;
            depth >>= 12 as libc::c_int;
            depth += 512 as libc::c_int;
            depth /= 10 as libc::c_int
        }
    }
    if fogStatus & 2 as libc::c_int as libc::c_uint != 0 {
        //add fog
        if pie_GetFogEnabled() != 0 {
            if fogStatus & 1 as libc::c_int as libc::c_uint == 0 {
                //black penumbra so fade fog effect
                fog = depth - (255 as libc::c_int - umbra)
            } else { fog = depth }
            if fog < 0 as libc::c_int {
                fog = 0 as libc::c_int
            } else if fog > 255 as libc::c_int { fog = 255 as libc::c_int }
        }
    }
    /*
	if ((fogStatus & FOG_GROUND) && (pie_GetFogStatus()))
	{
		//add mist
		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
		mist = map_MistValue(centreX-dx,centreZ-dz);
		if (!(fogStatus & FOG_BACKGROUND))//black penumbra so fade fog effect
		{
			mist = mist - (255 - umbra);
		}
		if (mist < 0)
		{
			mist = 0;
		}
		else if (mist > 255)
		{
			mist = 255;
		}
	}

	fog = fog + mist;
	if (fog > 255)
	{
		fog = 255;
	}
	*/
    if fogStatus & 1 as libc::c_int as libc::c_uint != 0 &&
           pie_GetFogEnabled() != 0 {
        //fog the umbra but only for distant points
        if depth as libc::c_float > -0.1f64 as libc::c_float {
            fog = fog + (255 as libc::c_int - umbra);
            if fog > 255 as libc::c_int { fog = 255 as libc::c_int }
            if fog < 0 as libc::c_int { fog = 0 as libc::c_int }
        }
    } else { brightness = pie_ByteScale(brightness, umbra as UBYTE) }
    if fog == 0 as libc::c_int {
        // (d3d with no fog?)
        *pSpecular = 0 as libc::c_int as UDWORD;
        lighting.byte.a = 0xff as libc::c_int as UBYTE;
        lighting.byte.r = brightness;
        lighting.byte.g = brightness;
        lighting.byte.b = brightness
    } else {
        fogColour.argb = pie_GetFogColour();
        specular.byte.a = fog as UBYTE;
        specular.byte.r = pie_ByteScale(fog as UBYTE, fogColour.byte.r);
        specular.byte.g = pie_ByteScale(fog as UBYTE, fogColour.byte.g);
        specular.byte.b = pie_ByteScale(fog as UBYTE, fogColour.byte.b);
        *pSpecular = specular.argb;
        //calculate new brightness
        colour = 256 as libc::c_int - fog;
        brightness = pie_ByteScale(colour as UBYTE, brightness);
        lighting.byte.a = 0xff as libc::c_int as UBYTE;
        lighting.byte.r = brightness;
        lighting.byte.g = brightness;
        lighting.byte.b = brightness
    }
    return lighting.argb;
}
/*
void	doBuildingLights( void )
{
STRUCTURE	*psStructure;
UDWORD	i;
LIGHT	light;

	for(i=0; i<MAX_PLAYERS; i++)
	{
		for(psStructure = apsStructLists[i]; psStructure; psStructure = psStructure->psNext)
		{
			light.range = psStructure->pStructureType->baseWidth * TILE_UNITS;
			light.position.x = psStructure->x;
			light.position.z = psStructure->y;
			light.position.y = map_Height(light.position.x,light.position.z);
			light.range = psStructure->pStructureType->baseWidth * TILE_UNITS;
			light.colour = LIGHT_WHITE;
			processLight(&light);
		}
	}
}
*/
