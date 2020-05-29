use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
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
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    // the structure that was last hit
    #[no_mangle]
    static mut psLastStructHit: *mut STRUCTURE;
    // the structure that was last hit
    #[no_mangle]
    static mut psLastDroidHit: *mut DROID;
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
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
    #[no_mangle]
    static mut psScrCBTarget: *mut BASE_OBJECT;
    // the last object to be seen for a CALL_OBJ_SEEN
    #[no_mangle]
    static mut psScrCBObjSeen: *mut BASE_OBJECT;
    // the object that saw psScrCBObjSeen for a CALL_OBJ_SEEN
    #[no_mangle]
    static mut psScrCBObjViewer: *mut BASE_OBJECT;
    // tell the scripts when a cluster is no longer valid
    #[no_mangle]
    static mut scrCBEmptyClusterID: SDWORD;
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
pub type STRUCTURE = _structure;
pub type TRIGGER_TYPE = _trigger_type;
/* Different types of triggers */
pub type _trigger_type = libc::c_uint;
// The user defined callback triggers should start with this id
// Event has paused for an interval and will restart in the middle of it's code
pub const TR_CALLBACKSTART: _trigger_type = 5;
// Trigger at repeated intervals
pub const TR_PAUSE: _trigger_type = 4;
// Trigger after a time pause
pub const TR_EVERY: _trigger_type = 3;
// Trigger uses script code
pub const TR_WAIT: _trigger_type = 2;
// Trigger fires when the script is first run
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_ATTACKED: _scr_callback_types = 38;
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
// Indirect the cluster ID to an actual cluster number
#[no_mangle]
pub static mut aClusterMap: [UBYTE; 255] = [0; 255];
// flag to note when a cluster needs the cluster empty callback
#[no_mangle]
pub static mut aClusterEmpty: [UBYTE; 255] = [0; 255];
// number of droids in a cluster
#[no_mangle]
pub static mut aClusterUsage: [UWORD; 255] = [0; 255];
// whether a cluster can be seen by a player
#[no_mangle]
pub static mut aClusterVisibility: [UBYTE; 255] = [0; 255];
// when a cluster was last attacked
#[no_mangle]
pub static mut aClusterAttacked: [UDWORD; 255] = [0; 255];
// information about the cluster
#[no_mangle]
pub static mut aClusterInfo: [UBYTE; 255] = [0; 255];
// distance between units for them to be in the same cluster
//#define CLUSTER_DIST	(TILE_UNITS*4)
// cluster information flags
// Indirect the cluster ID to an actual cluster number
// number of droids in a cluster
// whether a cluster can be seen by a player
// when a cluster was last attacked
// information about the cluster
// initialise the cluster system
// initialise the cluster system
#[no_mangle]
pub unsafe extern "C" fn clustInitialise() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut player: SDWORD = 0;
    if 0xff as libc::c_int <= 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"clustInitialse: invalid CLUSTER_MAX, this is a BUILD error\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 0xff as libc::c_int <= 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"cluster.c\x00" as *const u8 as *const libc::c_char,
              56 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"clustInitialise\x00")).as_ptr(),
              b"CLUSTER_MAX <= UBYTE_MAX\x00" as *const u8 as
                  *const libc::c_char);
    };
    memset(aClusterMap.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<UBYTE>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    memset(aClusterEmpty.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<UBYTE>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    memset(aClusterUsage.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<UWORD>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    memset(aClusterVisibility.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<UBYTE>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    memset(aClusterAttacked.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<UDWORD>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    memset(aClusterInfo.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<UBYTE>() as
                libc::c_ulong).wrapping_mul(0xff as libc::c_int as
                                                libc::c_uint));
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        psDroid = apsDroidLists[player as usize];
        while !psDroid.is_null() {
            (*psDroid).cluster = 0 as libc::c_int as UBYTE;
            psDroid = (*psDroid).psNext
        }
        psStruct = apsStructLists[player as usize];
        while !psStruct.is_null() {
            (*psStruct).cluster = 0 as libc::c_int as UBYTE;
            psStruct = (*psStruct).psNext
        }
        psStruct = apsStructLists[player as usize];
        while !psStruct.is_null() {
            if (*psStruct).cluster as libc::c_int == 0 as libc::c_int {
                clustUpdateObject(psStruct as *mut BASE_OBJECT);
            }
            psStruct = (*psStruct).psNext
        }
        player += 1
    };
}
// check the cluster usage
#[no_mangle]
pub unsafe extern "C" fn clustValidateUsage() {
    let mut cluster: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut droidUsage: SDWORD = 0;
    let mut structUsage: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut found: SDWORD = 0;
    cluster = 1 as libc::c_int;
    while cluster < 0xff as libc::c_int {
        found = 8 as libc::c_int;
        player = 0 as libc::c_int;
        while player < 8 as libc::c_int {
            droidUsage = 0 as libc::c_int;
            structUsage = 0 as libc::c_int;
            psDroid = apsDroidLists[player as usize];
            while !psDroid.is_null() {
                if (*psDroid).cluster as libc::c_int == cluster {
                    if found == 8 as libc::c_int ||
                           droidUsage != 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"clustValidateUsage: cluster has mixed players\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if found == 8 as libc::c_int ||
                           droidUsage != 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"cluster.c\x00" as *const u8 as
                                  *const libc::c_char, 108 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"clustValidateUsage\x00")).as_ptr(),
                              b"(found == MAX_PLAYERS) || (droidUsage != 0)\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    found = player;
                    droidUsage += 1 as libc::c_int
                }
                psDroid = (*psDroid).psNext
            }
            psStruct = apsStructLists[player as usize];
            while !psStruct.is_null() {
                if (*psStruct).cluster as libc::c_int == cluster {
                    if found == 8 as libc::c_int ||
                           structUsage != 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"clustValidateUsage: cluster has mixed players\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if found == 8 as libc::c_int ||
                           structUsage != 0 as libc::c_int {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"cluster.c\x00" as *const u8 as
                                  *const libc::c_char, 119 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"clustValidateUsage\x00")).as_ptr(),
                              b"(found == MAX_PLAYERS) || (structUsage != 0)\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    found = player;
                    structUsage += 1 as libc::c_int
                }
                psStruct = (*psStruct).psNext
            }
            if found == player {
                if droidUsage == 0 as libc::c_int ||
                       structUsage == 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"clustValidateUsage: cluster contains both droids and structs\x00"
                              as *const u8 as *const libc::c_char);
                };
                if droidUsage == 0 as libc::c_int ||
                       structUsage == 0 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"cluster.c\x00" as *const u8 as
                              *const libc::c_char, 129 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"clustValidateUsage\x00")).as_ptr(),
                          b"(droidUsage == 0) || (structUsage == 0)\x00" as
                              *const u8 as *const libc::c_char);
                };
                if aClusterUsage[cluster as usize] as libc::c_int ==
                       droidUsage + structUsage {
                } else {
                    debug(LOG_ERROR,
                          b"clustValidateUsage: invalid cluster usage\x00" as
                              *const u8 as *const libc::c_char);
                };
                if aClusterUsage[cluster as usize] as libc::c_int ==
                       droidUsage + structUsage {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"cluster.c\x00" as *const u8 as
                              *const libc::c_char, 132 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"clustValidateUsage\x00")).as_ptr(),
                          b"aClusterUsage[cluster] == droidUsage + structUsage\x00"
                              as *const u8 as *const libc::c_char);
                };
            }
            player += 1
        }
        cluster += 1
    };
}
// update routine for the cluster system
// update routine for the cluster system
#[no_mangle]
pub unsafe extern "C" fn clusterUpdate() {
    let mut i: SDWORD = 0;
    i = 1 as libc::c_int;
    while i < 0xff as libc::c_int {
        if aClusterEmpty[i as usize] != 0 {
            scrCBEmptyClusterID = i;
            eventFireCallbackTrigger(CALL_CLUSTER_EMPTY as libc::c_int as
                                         TRIGGER_TYPE);
            aClusterEmpty[i as usize] = 0 as libc::c_int as UBYTE
        }
        i += 1
    };
}
// update all objects from a list belonging to a specific cluster
// update all objects from a list belonging to a specific cluster
#[no_mangle]
pub unsafe extern "C" fn clustUpdateCluster(mut psList: *mut BASE_OBJECT,
                                            mut cluster: SDWORD) {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if cluster == 0 as libc::c_int { return }
    psCurr = psList;
    while !psCurr.is_null() {
        if (*psCurr).cluster as libc::c_int == cluster {
            clustUpdateObject(psCurr);
        }
        psCurr = (*psCurr).psNext
    };
}
// remove an object from the cluster system
// remove an object from the cluster system
#[no_mangle]
pub unsafe extern "C" fn clustRemoveObject(mut psObj: *mut BASE_OBJECT) {
    let mut i: SDWORD = 0;
    if ((*psObj).cluster as libc::c_int) < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"clustRemoveObject: invalid cluster number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psObj).cluster as libc::c_int) < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"cluster.c\x00" as *const u8 as *const libc::c_char,
              181 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"clustRemoveObject\x00")).as_ptr(),
              b"psObj->cluster < CLUSTER_MAX\x00" as *const u8 as
                  *const libc::c_char);
    };
    // update the usage counter
    if (*psObj).cluster as libc::c_int != 0 as libc::c_int {
        if aClusterUsage[(*psObj).cluster as usize] as libc::c_int >
               0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"clustRemoveObject: usage array out of sync\x00" as
                      *const u8 as *const libc::c_char);
        };
        if aClusterUsage[(*psObj).cluster as usize] as libc::c_int >
               0 as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"cluster.c\x00" as *const u8 as *const libc::c_char,
                  187 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"clustRemoveObject\x00")).as_ptr(),
                  b"aClusterUsage[psObj->cluster] > 0\x00" as *const u8 as
                      *const libc::c_char);
        };
        aClusterUsage[(*psObj).cluster as usize] =
            (aClusterUsage[(*psObj).cluster as usize] as libc::c_int -
                 1 as libc::c_int) as UWORD;
        if aClusterUsage[(*psObj).cluster as usize] as libc::c_int ==
               0 as libc::c_int {
            // cluster is empty - make sure the cluster map gets emptied
            i = 0 as libc::c_int;
            while i < 0xff as libc::c_int {
                if aClusterMap[i as usize] as libc::c_int ==
                       (*psObj).cluster as libc::c_int {
                    aClusterMap[i as usize] = 0 as libc::c_int as UBYTE;
                    if i != 0 as libc::c_int {
                        aClusterEmpty[i as usize] = 1 as libc::c_int as UBYTE
                    }
                }
                i += 1
            }
            // reset the cluster visibility and attacked
            aClusterVisibility[(*psObj).cluster as usize] =
                0 as libc::c_int as UBYTE;
            aClusterAttacked[(*psObj).cluster as usize] =
                0 as libc::c_int as UDWORD;
            aClusterInfo[(*psObj).cluster as usize] =
                0 as libc::c_int as UBYTE
        }
    }
    (*psObj).cluster = 0 as libc::c_int as UBYTE;
}
// tell a droid to join a cluster
#[no_mangle]
pub unsafe extern "C" fn _clustAddDroid(mut psDroid: *mut DROID,
                                        mut cluster: SDWORD) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut player: SDWORD = 0;
    clustRemoveObject(psDroid as *mut BASE_OBJECT);
    aClusterUsage[cluster as usize] =
        (aClusterUsage[cluster as usize] as libc::c_int + 1 as libc::c_int) as
            UWORD;
    (*psDroid).cluster = cluster as UBYTE;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        if (*psDroid).visible[player as usize] != 0 {
            aClusterVisibility[cluster as usize] =
                (aClusterVisibility[cluster as usize] as libc::c_int |
                     (1 as libc::c_int) << player) as UBYTE
        }
        player += 1
    }
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if !((*psCurr).cluster as libc::c_int ==
                 cluster as UBYTE as libc::c_int) {
            xdiff = (*psDroid).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psDroid).y as SDWORD - (*psCurr).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 8 as libc::c_int *
                       (128 as libc::c_int * 8 as libc::c_int) {
                _clustAddDroid(psCurr, cluster);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn clustAddDroid(mut psDroid: *mut DROID,
                                       mut cluster: SDWORD) {
    _clustAddDroid(psDroid, cluster);
}
// tell the cluster system about a new droid
// tell the cluster system about a new droid
#[no_mangle]
pub unsafe extern "C" fn clustNewDroid(mut psDroid: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    (*psDroid).cluster = 0 as libc::c_int as UBYTE;
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if (*psCurr).cluster as libc::c_int != 0 as libc::c_int {
            xdiff = (*psDroid).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psDroid).y as SDWORD - (*psCurr).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 8 as libc::c_int *
                       (128 as libc::c_int * 8 as libc::c_int) {
                clustAddDroid(psDroid, (*psCurr).cluster as SDWORD);
                return
            }
        }
        psCurr = (*psCurr).psNext
    };
}
// tell a structure to join a cluster
#[no_mangle]
pub unsafe extern "C" fn _clustAddStruct(mut psStruct: *mut STRUCTURE,
                                         mut cluster: SDWORD) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut player: SDWORD = 0;
    clustRemoveObject(psStruct as *mut BASE_OBJECT);
    aClusterUsage[cluster as usize] =
        (aClusterUsage[cluster as usize] as libc::c_int + 1 as libc::c_int) as
            UWORD;
    (*psStruct).cluster = cluster as UBYTE;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        if (*psStruct).visible[player as usize] != 0 {
            aClusterVisibility[cluster as usize] =
                (aClusterVisibility[cluster as usize] as libc::c_int |
                     (1 as libc::c_int) << player) as UBYTE
        }
        player += 1
    }
    psCurr = apsStructLists[(*psStruct).player as usize];
    while !psCurr.is_null() {
        if !((*psCurr).cluster as libc::c_int ==
                 cluster as UBYTE as libc::c_int) {
            xdiff = (*psStruct).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psStruct).y as SDWORD - (*psCurr).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 8 as libc::c_int *
                       (128 as libc::c_int * 8 as libc::c_int) {
                _clustAddStruct(psCurr, cluster);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
#[no_mangle]
pub unsafe extern "C" fn clustAddStruct(mut psStruct: *mut STRUCTURE,
                                        mut cluster: SDWORD) {
    _clustAddStruct(psStruct, cluster);
}
// tell the cluster system about a new structure
// tell the cluster system about a new structure
#[no_mangle]
pub unsafe extern "C" fn clustNewStruct(mut psStruct: *mut STRUCTURE) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    (*psStruct).cluster = 0 as libc::c_int as UBYTE;
    psCurr = apsStructLists[(*psStruct).player as usize];
    while !psCurr.is_null() {
        if (*psCurr).cluster as libc::c_int != 0 as libc::c_int {
            xdiff = (*psStruct).x as SDWORD - (*psCurr).x as SDWORD;
            ydiff = (*psStruct).y as SDWORD - (*psCurr).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 8 as libc::c_int *
                       (128 as libc::c_int * 8 as libc::c_int) {
                (*psStruct).cluster = (*psCurr).cluster;
                aClusterUsage[(*psCurr).cluster as usize] =
                    (aClusterUsage[(*psCurr).cluster as usize] as libc::c_int
                         + 1 as libc::c_int) as UWORD;
                break ;
            }
        }
        psCurr = (*psCurr).psNext
    }
    clustUpdateObject(psStruct as *mut BASE_OBJECT);
}
// find an unused cluster number for a droid
#[no_mangle]
pub unsafe extern "C" fn clustFindUnused() -> SDWORD {
    let mut cluster: SDWORD = 0;
    cluster = 1 as libc::c_int;
    while cluster < 0xff as libc::c_int {
        if aClusterUsage[cluster as usize] as libc::c_int == 0 as libc::c_int
           {
            return cluster
        }
        cluster += 1
    }
    // no unused cluster return the default
    return 0 as libc::c_int;
}
// display the current clusters
// display the current clusters
/*void clustDisplay(void)
{
	SDWORD	player, cluster;
	DROID	*psCurr;
	BOOL	shownCluster;

	DBPRINTF(("Current clusters:\n"));
	for (player=0; player<MAX_PLAYERS; player++)
	{
		DBPRINTF(("Player %d:\n", player));
		for (cluster=0; cluster < UBYTE_MAX; cluster++)
		{
			shownCluster = FALSE;
			for (psCurr=apsDroidLists[player]; psCurr; psCurr=psCurr->psNext)
			{
				if (psCurr->cluster == cluster)
				{
					if (!shownCluster)
					{
						DBPRINTF(("   Cluster %d:  ", cluster));
						shownCluster = TRUE;
					}
					DBPRINTF(("%d  ", psCurr->id));
				}
			}
			if (shownCluster)
			{
				DBPRINTF(("\n"));
			}
		}
	}
}*/
// display the current clusters
#[no_mangle]
pub unsafe extern "C" fn clustDisplay() {
    let mut cluster: SDWORD = 0;
    let mut map: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut aBuff: [STRING; 255] = [0; 255];
    let mut found: BOOL = 0;
    let mut numUsed: SDWORD = 0;
    numUsed = 0 as libc::c_int;
    map = 0 as libc::c_int;
    while map < 0xff as libc::c_int {
        if aClusterMap[map as usize] as libc::c_int != 0 as libc::c_int {
            numUsed += 1 as libc::c_int
        }
        map += 1
    }
    sprintf(ConsoleString.as_mut_ptr(),
            b"Current clusters (%d):\n\x00" as *const u8 as
                *const libc::c_char, numUsed);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    map = 0 as libc::c_int;
    while map < 0xff as libc::c_int {
        cluster = aClusterMap[map as usize] as SDWORD;
        if cluster != 0 as libc::c_int {
            found = 0 as libc::c_int;
            player = 0 as libc::c_int;
            while player < 8 as libc::c_int {
                //				if (player == (SDWORD)selectedPlayer)
//				{
//					continue;
//				}
                psDroid = apsDroidLists[player as usize];
                while !psDroid.is_null() {
                    if (*psDroid).cluster as libc::c_int == cluster &&
                           (*psDroid).player as libc::c_uint == selectedPlayer
                       {
                        if found == 0 {
                            // found a cluster print it out
                            sprintf(aBuff.as_mut_ptr(),
                                    b"Unit cluster %d (%d), \x00" as *const u8
                                        as *const libc::c_char, map, cluster);
                            sprintf(aBuff.as_mut_ptr().offset(strlen(aBuff.as_mut_ptr())
                                                                  as isize),
                                    b"player %d:\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*psDroid).player as libc::c_int);
                            found = 1 as libc::c_int
                        }
                        if strlen(aBuff.as_mut_ptr()) <
                               250 as libc::c_int as libc::c_uint {
                            sprintf(aBuff.as_mut_ptr().offset(strlen(aBuff.as_mut_ptr())
                                                                  as isize),
                                    b" %d\x00" as *const u8 as
                                        *const libc::c_char, (*psDroid).id);
                        }
                    }
                    psDroid = (*psDroid).psNext
                }
                psStruct = apsStructLists[player as usize];
                while !psStruct.is_null() {
                    if (*psStruct).cluster as libc::c_int == cluster &&
                           (*psStruct).player as libc::c_uint ==
                               selectedPlayer {
                        if found == 0 {
                            // found a cluster print it out
                            sprintf(aBuff.as_mut_ptr(),
                                    b"struct cluster %d (%d), \x00" as
                                        *const u8 as *const libc::c_char, map,
                                    cluster);
                            sprintf(aBuff.as_mut_ptr().offset(strlen(aBuff.as_mut_ptr())
                                                                  as isize),
                                    b"player %d:\x00" as *const u8 as
                                        *const libc::c_char,
                                    (*psStruct).player as libc::c_int);
                            found = 1 as libc::c_int
                        }
                        if strlen(aBuff.as_mut_ptr()) <
                               250 as libc::c_int as libc::c_uint {
                            sprintf(aBuff.as_mut_ptr().offset(strlen(aBuff.as_mut_ptr())
                                                                  as isize),
                                    b" %d\x00" as *const u8 as
                                        *const libc::c_char, (*psStruct).id);
                        }
                    }
                    psStruct = (*psStruct).psNext
                }
                player += 1
            }
            if found != 0 {
                sprintf(ConsoleString.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        aBuff.as_mut_ptr());
                addConsoleMessage(ConsoleString.as_mut_ptr(),
                                  DEFAULT_JUSTIFY);
            }
        }
        map += 1
    };
}
// update the cluster information for an object
// update the cluster information for an object
#[no_mangle]
pub unsafe extern "C" fn clustUpdateObject(mut psObj: *mut BASE_OBJECT) {
    let mut newCluster: SDWORD = 0;
    let mut oldCluster: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut found: BOOL = 0;
    let mut player: SDWORD = 0;
    newCluster = clustFindUnused();
    oldCluster = (*psObj).cluster as SDWORD;
    // update the cluster map
    found = 0 as libc::c_int;
    if oldCluster != 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 0xff as libc::c_int {
            if aClusterMap[i as usize] as libc::c_int == 0 as libc::c_int ||
                   aClusterUsage[aClusterMap[i as usize] as usize] as
                       libc::c_int != 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"clustUpdateObject: cluster map out of sync\x00" as
                          *const u8 as *const libc::c_char);
            };
            if aClusterMap[i as usize] as libc::c_int == 0 as libc::c_int ||
                   aClusterUsage[aClusterMap[i as usize] as usize] as
                       libc::c_int != 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"cluster.c\x00" as *const u8 as *const libc::c_char,
                      505 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"clustUpdateObject\x00")).as_ptr(),
                      b"(aClusterMap[i] == 0) || (aClusterUsage[ aClusterMap[i] ] != 0)\x00"
                          as *const u8 as *const libc::c_char);
            };
            if aClusterMap[i as usize] as libc::c_int == oldCluster {
                // found the old cluster - change it to the new one
                aClusterMap[i as usize] = newCluster as UBYTE;
                aClusterAttacked[newCluster as usize] =
                    aClusterAttacked[oldCluster as usize];
                //				aClusterVisibility[newCluster] = aClusterVisibility[oldCluster];
                aClusterVisibility[newCluster as usize] =
                    0 as libc::c_int as UBYTE;
                player = 0 as libc::c_int;
                while player < 8 as libc::c_int {
                    if (*psObj).visible[player as usize] != 0 {
                        aClusterVisibility[newCluster as usize] =
                            (aClusterVisibility[newCluster as usize] as
                                 libc::c_int | (1 as libc::c_int) << player)
                                as UBYTE
                    }
                    player += 1
                }
                found = 1 as libc::c_int;
                break ;
            } else { i += 1 }
        }
    }
    if found == 0 {
        // there is no current cluster map - create a new one
        i = 1 as libc::c_int;
        while i < 0xff as libc::c_int {
            if aClusterMap[i as usize] as libc::c_int == 0 as libc::c_int {
                // found a free cluster
                aClusterMap[i as usize] = newCluster as UBYTE;
                break ;
            } else { i += 1 }
        }
    }
    // store the information about this cluster
    aClusterInfo[newCluster as usize] =
        ((*psObj).player as libc::c_int & 0x7 as libc::c_int) as UBYTE;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            aClusterInfo[newCluster as usize] =
                (aClusterInfo[newCluster as usize] as libc::c_int |
                     0x8 as libc::c_int) as UBYTE
        }
        1 => {
            aClusterInfo[newCluster as usize] =
                (aClusterInfo[newCluster as usize] as libc::c_int |
                     0x10 as libc::c_int) as UBYTE
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"clustUpdateObject: invalid object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"cluster.c\x00" as *const u8 as *const libc::c_char,
                      552 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"clustUpdateObject\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    match (*psObj).type_0 as libc::c_uint {
        0 => { clustAddDroid(psObj as *mut DROID, newCluster); }
        1 => { clustAddStruct(psObj as *mut STRUCTURE, newCluster); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"clustUpdateObject: invalid object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"cluster.c\x00" as *const u8 as *const libc::c_char,
                      565 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"clustUpdateObject\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
// get the cluster ID for an object
// get the cluster ID for a droid
#[no_mangle]
pub unsafe extern "C" fn clustGetClusterID(mut psObj: *mut BASE_OBJECT)
 -> SDWORD {
    let mut cluster: SDWORD = 0;
    if (*psObj).cluster as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    cluster = 0 as libc::c_int;
    while cluster < 0xff as libc::c_int {
        if aClusterMap[cluster as usize] as libc::c_int ==
               (*psObj).cluster as libc::c_int {
            return cluster
        }
        cluster += 1
    }
    return 0 as libc::c_int;
}
// get the actual cluster number from a cluster ID
// get the actual cluster number from a cluster ID
#[no_mangle]
pub unsafe extern "C" fn clustGetClusterFromID(mut clusterID: SDWORD)
 -> SDWORD {
    if clusterID >= 0 as libc::c_int && clusterID < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"clustGetClusterFromID: invalid cluster ID\x00" as *const u8 as
                  *const libc::c_char);
    };
    if clusterID >= 0 as libc::c_int && clusterID < 0xff as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"cluster.c\x00" as *const u8 as *const libc::c_char,
              596 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"clustGetClusterFromID\x00")).as_ptr(),
              b"(clusterID >= 0) && (clusterID < CLUSTER_MAX)\x00" as
                  *const u8 as *const libc::c_char);
    };
    return aClusterMap[clusterID as usize] as SDWORD;
}
// variables for the cluster iteration
static mut iterateClusterID: SDWORD = 0;
static mut psIterateList: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
static mut psIterateObj: *mut BASE_OBJECT =
    0 as *const BASE_OBJECT as *mut BASE_OBJECT;
// initialise iterating a cluster
// initialise iterating a cluster
#[no_mangle]
pub unsafe extern "C" fn clustInitIterate(mut clusterID: SDWORD) {
    let mut player: SDWORD = 0;
    let mut cluster: SDWORD = 0;
    iterateClusterID = clusterID;
    cluster = aClusterMap[clusterID as usize] as SDWORD;
    player =
        aClusterInfo[cluster as usize] as libc::c_int & 0x7 as libc::c_int;
    if aClusterInfo[cluster as usize] as libc::c_int & 0x8 as libc::c_int != 0
       {
        psIterateList = apsDroidLists[player as usize] as *mut BASE_OBJECT
    } else {
        // if (aClusterInfo[cluster] & CLUSTER_STRUCTURE)
        psIterateList = apsStructLists[player as usize] as *mut BASE_OBJECT
    }
    psIterateObj = 0 as *mut BASE_OBJECT;
}
// iterate a cluster
// iterate a cluster
#[no_mangle]
pub unsafe extern "C" fn clustIterate() -> *mut BASE_OBJECT {
    let mut psStart: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut cluster: SDWORD = 0;
    cluster = aClusterMap[iterateClusterID as usize] as SDWORD;
    if psIterateObj.is_null() {
        psStart = psIterateList
    } else { psStart = (*psIterateObj).psNext }
    psIterateObj = psStart;
    while !psIterateObj.is_null() &&
              (*psIterateObj).cluster as libc::c_int != cluster {
        psIterateObj = (*psIterateObj).psNext
    }
    if psIterateObj.is_null() { psIterateList = 0 as *mut BASE_OBJECT }
    return psIterateObj;
}
// find the center of a cluster
// find the center of a cluster
#[no_mangle]
pub unsafe extern "C" fn clustGetCenter(mut psObj: *mut BASE_OBJECT,
                                        mut px: *mut SDWORD,
                                        mut py: *mut SDWORD) {
    let mut psList: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut averagex: SDWORD = 0;
    let mut averagey: SDWORD = 0;
    let mut num: SDWORD = 0;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            psList =
                apsDroidLists[(*psObj).player as usize] as *mut BASE_OBJECT
        }
        1 => {
            psList =
                apsStructLists[(*psObj).player as usize] as *mut BASE_OBJECT
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"clustGetCenter: invalid object type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"cluster.c\x00" as *const u8 as *const libc::c_char,
                      672 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"clustGetCenter\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            psList = 0 as *mut BASE_OBJECT
        }
    }
    averagex = 0 as libc::c_int;
    averagey = 0 as libc::c_int;
    num = 0 as libc::c_int;
    psCurr = psList;
    while !psCurr.is_null() {
        if (*psCurr).cluster as libc::c_int == (*psObj).cluster as libc::c_int
           {
            averagex += (*psCurr).x as SDWORD;
            averagey += (*psCurr).y as SDWORD;
            num += 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    if num > 0 as libc::c_int {
        *px = averagex / num;
        *py = averagey / num
    } else { *px = (*psObj).x as SDWORD; *py = (*psObj).y as SDWORD };
}
// tell the cluster system that an objects visibility has changed
// tell the cluster system that an objects visibility has changed
#[no_mangle]
pub unsafe extern "C" fn clustObjectSeen(mut psObj: *mut BASE_OBJECT,
                                         mut psViewer: *mut BASE_OBJECT) {
    let mut player: SDWORD = 0;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        if player != (*psObj).player as SDWORD &&
               (*psObj).visible[player as usize] as libc::c_int != 0 &&
               aClusterVisibility[(*psObj).cluster as usize] as libc::c_int &
                   (1 as libc::c_int) << player == 0 {
            //			DBPRINTF(("cluster %d (player %d) seen by player %d\n",
//				clustGetClusterID(psObj), psObj->player, player));
            aClusterVisibility[(*psObj).cluster as usize] =
                (aClusterVisibility[(*psObj).cluster as usize] as libc::c_int
                     | (1 as libc::c_int) << player) as UBYTE;
            psScrCBObjSeen = psObj;
            psScrCBObjViewer = psViewer;
            eventFireCallbackTrigger(CALL_OBJ_SEEN as libc::c_int as
                                         TRIGGER_TYPE);
            match (*psObj).type_0 as libc::c_uint {
                0 => {
                    eventFireCallbackTrigger(CALL_DROID_SEEN as libc::c_int as
                                                 TRIGGER_TYPE);
                }
                1 => {
                    eventFireCallbackTrigger(CALL_STRUCT_SEEN as libc::c_int
                                                 as TRIGGER_TYPE);
                }
                2 => {
                    eventFireCallbackTrigger(CALL_FEATURE_SEEN as libc::c_int
                                                 as TRIGGER_TYPE);
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"clustObjectSeen: invalid object type\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"cluster.c\x00" as *const u8 as
                                  *const libc::c_char, 734 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 16],
                                                        &[libc::c_char; 16]>(b"clustObjectSeen\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            psScrCBObjSeen = 0 as *mut BASE_OBJECT;
            psScrCBObjViewer = 0 as *mut BASE_OBJECT
        }
        player += 1
    };
}
// tell the cluster system that an object has been attacked
// tell the cluster system that an object has been attacked
#[no_mangle]
pub unsafe extern "C" fn clustObjectAttacked(mut psObj: *mut BASE_OBJECT) {
    if aClusterAttacked[(*psObj).cluster as
                            usize].wrapping_add(5000 as libc::c_int as
                                                    libc::c_uint) < gameTime {
        //		DBPRINTF(("CALL_ATTACKED player %d, cluster %d\n",
//			psObj->player, psObj->cluster));
        psScrCBTarget = psObj;
        eventFireCallbackTrigger(CALL_ATTACKED as libc::c_int as
                                     TRIGGER_TYPE);
        match (*psObj).type_0 as libc::c_uint {
            0 => {
                psLastDroidHit = psObj as *mut DROID;
                eventFireCallbackTrigger(CALL_DROID_ATTACKED as libc::c_int as
                                             TRIGGER_TYPE);
                psLastDroidHit = 0 as *mut DROID
            }
            1 => {
                psLastStructHit = psObj as *mut STRUCTURE;
                eventFireCallbackTrigger(CALL_STRUCT_ATTACKED as libc::c_int
                                             as TRIGGER_TYPE);
                psLastStructHit = 0 as *mut STRUCTURE
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"clustObjectAttacked: invalid object type\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"cluster.c\x00" as *const u8 as
                              *const libc::c_char, 768 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"clustObjectAttacked\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        aClusterAttacked[(*psObj).cluster as usize] = gameTime
    };
}
// reset the visibility for all clusters for a particular player
// reset the visibility for all clusters for a particular player
#[no_mangle]
pub unsafe extern "C" fn clustResetVisibility(mut player: SDWORD) {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 0xff as libc::c_int {
        aClusterVisibility[i as usize] =
            (aClusterVisibility[i as usize] as libc::c_int &
                 !((1 as libc::c_int) << player)) as UBYTE;
        i += 1
    };
}
