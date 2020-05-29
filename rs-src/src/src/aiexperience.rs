use ::libc;
extern "C" {
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
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn printf_console(pFormat: *const libc::c_char, _: ...);
    // / Print to the ingame console in debug mode only
    #[no_mangle]
    fn console(pFormat: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dirtySqrt(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> UDWORD;
    #[no_mangle]
    fn PHYSFS_getLastError() -> *const libc::c_char;
    #[no_mangle]
    fn PHYSFS_mkdir(dirName: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_openWrite(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_openRead(filename: *const libc::c_char) -> *mut PHYSFS_File;
    #[no_mangle]
    fn PHYSFS_close(handle: *mut PHYSFS_File) -> libc::c_int;
    #[no_mangle]
    fn PHYSFS_read(handle: *mut PHYSFS_File, buffer: *mut libc::c_void,
                   objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
    #[no_mangle]
    fn PHYSFS_write(handle: *mut PHYSFS_File, buffer: *const libc::c_void,
                    objSize: PHYSFS_uint32, objCount: PHYSFS_uint32)
     -> PHYSFS_sint64;
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
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
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
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub type PHYSFS_uint32 = libc::c_uint;
pub type PHYSFS_sint64 = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PHYSFS_File {
    pub opaque: *mut libc::c_void,
}
static mut aiSaveFile: [*mut PHYSFS_File; 8] =
    [0 as *const PHYSFS_File as *mut PHYSFS_File; 8];
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
//aiexperience.c
#[no_mangle]
pub static mut baseLocation: [[[SDWORD; 2]; 8]; 8] = [[[0; 2]; 8]; 8];
//each player's visible enemy base x,y coords for each player (hence 3d)
#[no_mangle]
pub static mut oilLocation: [[[SDWORD; 2]; 300]; 8] = [[[0; 2]; 300]; 8];
//remembered oil locations
#[no_mangle]
pub static mut baseDefendLocation: [[[SDWORD; 2]; 30]; 8] = [[[0; 2]; 30]; 8];
#[no_mangle]
pub static mut baseDefendLocPrior: [[SDWORD; 30]; 8] = [[0; 30]; 8];
//Priority
#[no_mangle]
pub static mut oilDefendLocation: [[[SDWORD; 2]; 100]; 8] =
    [[[0; 2]; 100]; 8];
#[no_mangle]
pub static mut oilDefendLocPrior: [[SDWORD; 100]; 8] = [[0; 100]; 8];
#[no_mangle]
pub unsafe extern "C" fn InitializeAIExperience() {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            baseLocation[i as usize][j as usize][0 as libc::c_int as usize] =
                -(1 as libc::c_int);
            baseLocation[i as usize][j as usize][1 as libc::c_int as usize] =
                -(1 as libc::c_int);
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 30 as libc::c_int {
            baseDefendLocation[i as
                                   usize][j as
                                              usize][0 as libc::c_int as
                                                         usize] =
                -(1 as libc::c_int);
            baseDefendLocation[i as
                                   usize][j as
                                              usize][1 as libc::c_int as
                                                         usize] =
                -(1 as libc::c_int);
            baseDefendLocPrior[i as usize][j as usize] = -(1 as libc::c_int);
            j += 1
        }
        j = 0 as libc::c_int;
        while j < 100 as libc::c_int {
            oilDefendLocation[i as
                                  usize][j as
                                             usize][0 as libc::c_int as usize]
                = -(1 as libc::c_int);
            oilDefendLocation[i as
                                  usize][j as
                                             usize][1 as libc::c_int as usize]
                = -(1 as libc::c_int);
            oilDefendLocPrior[i as usize][j as usize] = -(1 as libc::c_int);
            j += 1
        }
        //oil locations
        j = 0 as libc::c_int;
        while j < 300 as libc::c_int {
            oilLocation[i as usize][j as usize][0 as libc::c_int as usize] =
                -(1 as libc::c_int);
            oilLocation[i as usize][j as usize][1 as libc::c_int as usize] =
                -(1 as libc::c_int);
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn LoadAIExperience(mut bNotify: BOOL) -> BOOL {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < game.maxPlayers as libc::c_int {
        LoadPlayerAIExperience(i, bNotify);
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SaveAIExperience(mut bNotify: BOOL) -> BOOL {
    let mut i: SDWORD = 0;
    i = 0 as libc::c_int;
    while i < game.maxPlayers as libc::c_int {
        SavePlayerAIExperience(i, bNotify);
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LoadPlayerAIExperience(mut nPlayer: SDWORD,
                                                mut bNotify: BOOL) -> BOOL {
    if nPlayer > -(1 as libc::c_int) && nPlayer < 8 as libc::c_int {
        if ReadAISaveData(nPlayer) != 0 {
            //addConsoleMessage("Experience loaded successfully.",RIGHT_JUSTIFY);
            if bNotify != 0 {
                console(b"Experience for player %d loaded successfully.\x00"
                            as *const u8 as *const libc::c_char, nPlayer);
            }
            return 1 as libc::c_int
        }
    }
    //addConsoleMessage("Failed to load experience (no experience saved?).",RIGHT_JUSTIFY);
    console(b"Failed to load experience for player %d (no experience saved?).\x00"
                as *const u8 as *const libc::c_char, nPlayer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SavePlayerAIExperience(mut nPlayer: SDWORD,
                                                mut bNotify: BOOL) -> BOOL {
    if nPlayer > -(1 as libc::c_int) && nPlayer < 8 as libc::c_int {
        if WriteAISaveData(nPlayer) == 0 {
            debug(LOG_ERROR,
                  b"SavePlayerAIExperience - failed to save exper\x00" as
                      *const u8 as *const libc::c_char);
            //addConsoleMessage("Failed to save experience.",RIGHT_JUSTIFY);
            console(b"Failed to save experience for player %d.\x00" as
                        *const u8 as *const libc::c_char, nPlayer);
            /*
			res = MessageBox(frameGetWinHandle(), "WriteAISaveData failed, delete this experience file?", "Confirmation",
							 MB_ICONQUESTION | MB_YESNO);
			if (res == IDYES)
			{
				//ToDo: delete
				//winQuit = TRUE;
			}
*/
            return 0 as libc::c_int
        }
        //else
		//{
		//	addConsoleMessage("Experience saved successfully.",RIGHT_JUSTIFY);
		//}
    }
    //addConsoleMessage("Experience saved successfully.",RIGHT_JUSTIFY);
    if bNotify != 0 {
        console(b"Experience for player %d saved successfully.\x00" as
                    *const u8 as *const libc::c_char,
                nPlayer); //"multiplay\\LearnData\\";
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetUpOutputFile(mut pMapName: *mut STRING,
                                         mut nPlayer: SDWORD) -> BOOL {
    let mut sPlayer: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut SaveDir: [STRING; 260] =
        *::std::mem::transmute::<&[u8; 260],
                                 &mut [STRING; 260]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut FileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    //debug(LOG_ERROR,"SetUpOutputFile");
    //strcpy( SaveDir, PHYSFS_getUserDir() );
	//strcat( SaveDir, WZ_WRITEDIR );				//TODO: fix it
	//strcat( SaveDir, PHYSFS_getDirSeparator() );
    strcpy(SaveDir.as_mut_ptr(),
           b"multiplay/learndata/\x00" as *const u8 as *const libc::c_char);
    //strcat( SaveDir, "multiplay" );
	//strcat( SaveDir, PHYSFS_getDirSeparator() );
	//strcat( SaveDir, "learndata" );
	//strcat( SaveDir, PHYSFS_getDirSeparator() );
    /* Assemble dir string */
    sprintf(sPlayer.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char, nPlayer);
    strcat(SaveDir.as_mut_ptr(),
           b"player\x00" as *const u8 as *const libc::c_char);
    strcat(SaveDir.as_mut_ptr(), sPlayer.as_mut_ptr());
    //strcat( SaveDir, "/" );	//like "multiplay\learndata\player0\"
    /* Create dir on disk */
    if PHYSFS_mkdir(SaveDir.as_mut_ptr()) == 0 {
        debug(LOG_ERROR,
              b"SetUpOutputFile: Error creating directory \"%s\": %s\x00" as
                  *const u8 as *const libc::c_char, SaveDir.as_mut_ptr(),
              PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    strcat(SaveDir.as_mut_ptr(),
           b"/\x00" as *const u8 as *const libc::c_char);
    /* Create filename */
    strcpy(FileName.as_mut_ptr(), SaveDir.as_mut_ptr()); //Map name
    strcat(FileName.as_mut_ptr(),
           game.map.as_mut_ptr()); //Like "multiplay\learndata\player0\Rush.lrn"
    strcat(FileName.as_mut_ptr(),
           b".lrn\x00" as *const u8 as *const libc::c_char);
    /* Open file */
    aiSaveFile[nPlayer as usize] =
        0 as *mut PHYSFS_File; //fopen(FileName, "wb");	//new write
    aiSaveFile[nPlayer as usize] =
        PHYSFS_openWrite(FileName.as_mut_ptr()); // "multiplay\\learndata\\";
    if aiSaveFile[nPlayer as usize].is_null() {
        debug(LOG_ERROR,
              b"SetUpOutputFile(): Couldn\'t open debugging output file: \'%s\' for player %d\x00"
                  as *const u8 as *const libc::c_char, FileName.as_mut_ptr(),
              nPlayer);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetUpInputFile(mut nPlayer: SDWORD) -> BOOL {
    let mut FileName: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut sPlayer: [STRING; 255] =
        *::std::mem::transmute::<&[u8; 255],
                                 &mut [STRING; 255]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let mut SaveDir: [STRING; 260] =
        *::std::mem::transmute::<&[u8; 260],
                                 &mut [STRING; 260]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    /* assemble "multiplay\learndata\" */
    strcat(SaveDir.as_mut_ptr(),
           b"multiplay/learndata/\x00" as *const u8 as *const libc::c_char);
    /* Assemble dir */
    sprintf(sPlayer.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            nPlayer); //like "multiplay\learndata\player0\"
    strcat(SaveDir.as_mut_ptr(),
           b"player\x00" as *const u8 as *const libc::c_char);
    strcat(SaveDir.as_mut_ptr(), sPlayer.as_mut_ptr());
    strcat(SaveDir.as_mut_ptr(),
           b"/\x00" as *const u8 as *const libc::c_char);
    /* Create filename */
    strcpy(FileName.as_mut_ptr(), SaveDir.as_mut_ptr()); //map name
    strcat(FileName.as_mut_ptr(),
           game.map.as_mut_ptr()); //Like "multiplay\learndata\player0\Rush.lrn"
    strcat(FileName.as_mut_ptr(),
           b".lrn\x00" as *const u8 as
               *const libc::c_char); //How many derricks/oil resources will be saved
    aiSaveFile[nPlayer as usize] =
        0 as *mut PHYSFS_File; //Locations, 0=x,1=y,2=x etc
    aiSaveFile[nPlayer as usize] = PHYSFS_openRead(FileName.as_mut_ptr());
    if aiSaveFile[nPlayer as usize].is_null() {
        debug(LOG_ERROR,
              b"SetUpInputFile(): Couldn\'t open input file: \'%s\' for player %d:\n%s\x00"
                  as *const u8 as *const libc::c_char, FileName.as_mut_ptr(),
              nPlayer, PHYSFS_getLastError());
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ExperienceRecallOil(mut nPlayer: SDWORD) -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn WriteAISaveData(mut nPlayer: SDWORD) -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    let mut NumEntries: SDWORD = 0 as libc::c_int;
    let mut PosXY: [UDWORD; 600] = [0; 600];
    let mut i: SDWORD = 0;
    /* Prepair experience file for the current map */
    if SetUpOutputFile(game.map.as_mut_ptr(), nPlayer) == 0 {
        debug(LOG_ERROR,
              b"Failed to prepare experience file for player %d\x00" as
                  *const u8 as *const libc::c_char, nPlayer);
        return 0 as libc::c_int
    }
    if !aiSaveFile[nPlayer as usize].is_null() {
        //debug(LOG_ERROR,"WriteAISaveData - aiSaveFile ok");
        /* Version */
        NumEntries = 1 as libc::c_int; //Version
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write version for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //fwrite(&NumEntries,sizeof(NumEntries),1,aiSaveFile[nPlayer]);	//Version
        //debug(LOG_ERROR,"WriteAISaveData - Version ok");
        /* ***********************/
		/*		Enemy bases		*/
		/* ***********************/
        NumEntries = 8 as libc::c_int;
        /* max num of players to store */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            //num of players to store
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write MAX_PLAYERS for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"WriteAISaveData - MAX_PLAYERS ok");
        /* base locations of all players */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        baseLocation[nPlayer as usize].as_mut_ptr() as
                            *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        (8 as libc::c_int * 2 as libc::c_int) as
                            PHYSFS_uint32) !=
               (8 as libc::c_int * 2 as libc::c_int) as libc::c_longlong {
            //num of players to store
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write base locations for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"WriteAISaveData - Enemy bases ok");
        /* ***********************************/
		/*		Base attack locations		*/
		/* ***********************************/
        NumEntries = 30 as libc::c_int;
        /* write MAX_BASE_DEFEND_LOCATIONS */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) <
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write defence locations count for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* base defence locations */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        baseDefendLocation[nPlayer as usize].as_mut_ptr() as
                            *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        (30 as libc::c_int * 2 as libc::c_int) as
                            PHYSFS_uint32) <
               (30 as libc::c_int * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write defence locations for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* base defend priorities */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        baseDefendLocPrior[nPlayer as usize].as_mut_ptr() as
                            *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        (30 as libc::c_int * 2 as libc::c_int) as
                            PHYSFS_uint32) <
               (30 as libc::c_int * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write defence locations priority for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"WriteAISaveData - Base attack locations ok");
        /* ***********************************/
		/*		Oil attack locations		*/
		/* ***********************************/
        NumEntries = 100 as libc::c_int;
        /* MAX_OIL_DEFEND_LOCATIONS */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) <
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write oil defence locations count for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* oil locations */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        oilDefendLocation[nPlayer as usize].as_mut_ptr() as
                            *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        (100 as libc::c_int * 2 as libc::c_int) as
                            PHYSFS_uint32) <
               (100 as libc::c_int * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write oil defence locations for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* oil location priority */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        oilDefendLocPrior[nPlayer as usize].as_mut_ptr() as
                            *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        (100 as libc::c_int * 2 as libc::c_int) as
                            PHYSFS_uint32) <
               (100 as libc::c_int * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write oil defence locations priority for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"WriteAISaveData - Oil attack locations ok");
        /* ***************************/
		/*		Oil Resources		*/
		/* ***************************/
        NumEntries = 300 as libc::c_int;
        /* MAX_OIL_LOCATIONS */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) <
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write oil locations count for player %d\x00"
                      as *const u8 as *const libc::c_char,
                  nPlayer); //Num of oil resources
            return 0 as libc::c_int
        }
        NumEntries = 0 as libc::c_int;
        /* first save everything what we recalled from last load */
        i = 0 as libc::c_int; //skip to next one, since nothing stored
        while i < 300 as libc::c_int {
            if !(oilLocation[nPlayer as
                                 usize][i as usize][0 as libc::c_int as usize]
                     <= 0 as libc::c_int ||
                     oilLocation[nPlayer as
                                     usize][i as
                                                usize][1 as libc::c_int as
                                                           usize] <=
                         0 as libc::c_int) {
                PosXY[(NumEntries * 2 as libc::c_int) as usize] =
                    oilLocation[nPlayer as
                                    usize][i as
                                               usize][0 as libc::c_int as
                                                          usize] as
                        UDWORD; //x
                PosXY[(NumEntries * 2 as libc::c_int + 1 as libc::c_int) as
                          usize] =
                    oilLocation[nPlayer as
                                    usize][i as
                                               usize][1 as libc::c_int as
                                                          usize] as
                        UDWORD; //y
                NumEntries += 1
            }
            i += 1
        }
        /* now remember new ones that are not in memory yet (discovered this time) */
        psFeature = apsFeatureLists[0 as libc::c_int as usize];
        while !psFeature.is_null() {
            if (*(*psFeature).psStats).subType as libc::c_uint ==
                   FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                if (*psFeature).visible[nPlayer as usize] != 0 {
                    //OR godMode)
                    if canRecallOilAt(nPlayer, (*psFeature).x as SDWORD,
                                      (*psFeature).y as SDWORD) == 0 {
                        //already stored?
                        /* Save X */
                        PosXY[(NumEntries * 2 as libc::c_int) as usize] =
                            (*psFeature).x as UDWORD;
                        PosXY[(NumEntries * 2 as libc::c_int +
                                   1 as libc::c_int) as usize] =
                            (*psFeature).y as UDWORD;
                        NumEntries += 1
                    }
                }
            }
            psFeature = (*psFeature).psNext
        }
        /* Save Y */
        //printf_console("New oil visible x: %d y: %d. Storing.", PosXY[NumEntries * 2]/128,PosXY[NumEntries * 2 + 1]/128);
        //Save Derricks as oil resources, since most of them will be unoccupied when experiance will be loaded
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            psCurr = apsStructLists[i as usize];
            while !psCurr.is_null() {
                if (*(*psCurr).pStructureType).type_0 ==
                       REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
                    if (*psCurr).visible[nPlayer as usize] != 0 {
                        //if can see it
                        if canRecallOilAt(nPlayer, (*psCurr).x as SDWORD,
                                          (*psCurr).y as SDWORD) == 0 {
                            //already stored?
                            //psResExtractor = (RES_EXTRACTOR *)psCurr->pFunctionality;
                            x = (*psCurr).x as SDWORD;
                            y = (*psCurr).y as SDWORD;
                            PosXY[(NumEntries * 2 as libc::c_int) as usize] =
                                (*psCurr).x as UDWORD;
                            PosXY[(NumEntries * 2 as libc::c_int +
                                       1 as libc::c_int) as usize] =
                                (*psCurr).y as UDWORD;
                            NumEntries += 1
                        }
                    }
                }
                psCurr = (*psCurr).psNext
            }
            i = i + 1 as libc::c_int
        }
        //printf_console("Found derrick at x: %d, y: %d,, width: %d",psCurr->x/128,psCurr->y/128,mapWidth);
        // Save X //
        // Save Y //
        //printf_console("New derrick visible x: %d y: %d. Storing.", PosXY[NumEntries * 2]/128,PosXY[NumEntries * 2 + 1]/128);
        /* Write number of Oil Resources */
        if PHYSFS_write(aiSaveFile[nPlayer as usize],
                        &mut NumEntries as *mut SDWORD as *const libc::c_void,
                        ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                        1 as libc::c_int as PHYSFS_uint32) <
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"WriteAISaveData: failed to write stored oil locations count for player %d\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //printf_console("Num Oil Resources: %d ****",NumEntries);
        /* Write Oil Resources coords */
        if NumEntries > 0 as libc::c_int {
            //Anything to save
            if PHYSFS_write(aiSaveFile[nPlayer as usize],
                            PosXY.as_mut_ptr() as *const libc::c_void,
                            ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                            (NumEntries * 2 as libc::c_int) as PHYSFS_uint32)
                   < (NumEntries * 2 as libc::c_int) as libc::c_longlong {
                debug(LOG_ERROR,
                      b"WriteAISaveData: failed to write oil locations fir player %d\x00"
                          as *const u8 as *const libc::c_char, nPlayer);
                return 0 as libc::c_int
            }
        }
    } else {
        debug(LOG_ERROR,
              b"WriteAISaveData(): no output file for player %d\x00" as
                  *const u8 as *const libc::c_char, nPlayer);
        return 0 as libc::c_int
    }
    //printf_console("AI settings file written for player %d",nPlayer);
    return PHYSFS_close(aiSaveFile[nPlayer as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn canRecallOilAt(mut nPlayer: SDWORD, mut x: SDWORD,
                                        mut y: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    /* go through all remembered oil and check */
    i = 0 as libc::c_int;
    while i < 300 as libc::c_int {
        if !(oilLocation[nPlayer as
                             usize][i as usize][0 as libc::c_int as usize] !=
                 x) {
            if !(oilLocation[nPlayer as
                                 usize][i as usize][1 as libc::c_int as usize]
                     != y) {
                return 1 as libc::c_int
            }
        }
        i += 1
        //yep, both matched
    }
    return 0 as libc::c_int;
    //no
}
#[no_mangle]
pub unsafe extern "C" fn ReadAISaveData(mut nPlayer: SDWORD) -> BOOL {
    let mut psFeature: *mut FEATURE =
        0 as *mut FEATURE; //How many derricks/oil resources will be saved
    let mut x: SDWORD = 0 as libc::c_int; //Locations, 0=x,1=y,2=x etc
    let mut y: SDWORD = 0 as libc::c_int;
    let mut NumEntries: SDWORD = 0 as libc::c_int;
    let mut PosXY: [UDWORD; 600] = [0; 600];
    let mut i: SDWORD = 0;
    let mut Found: BOOL = 0;
    if SetUpInputFile(nPlayer) == 0 {
        //printf_console
        //debug(LOG_ERROR,"No experience data loaded for %d",nPlayer);
        return 0 as libc::c_int
    } else {
        /* Read data version */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to read version number for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"version: %d", NumEntries);
        /* ***********************/
		/*		Enemy bases		*/
		/* ***********************/
        /* read max number of players (usually 8) */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to read number of players for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"num enemy base: %d", NumEntries);
        /* read base locations of all players */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       baseLocation[nPlayer as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       (NumEntries * 2 as libc::c_int) as PHYSFS_uint32) !=
               (NumEntries * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load baseLocation for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* ***********************************/
		/*		Base attack locations		*/
		/* ***********************************/
        /* read MAX_BASE_DEFEND_LOCATIONS */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to read MAX_BASE_DEFEND_LOCATIONS for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* check it's the same as current MAX_BASE_DEFEND_LOCATIONS */
        if NumEntries > 30 as libc::c_int {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): saved MAX_BASE_DEFEND_LOCATIONS and current one don\'t match (%d / %d)\x00"
                      as *const u8 as *const libc::c_char, NumEntries,
                  30 as libc::c_int);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"num base attack loc: %d", NumEntries);
        /* read base defence locations */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       baseDefendLocation[nPlayer as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       (NumEntries * 2 as libc::c_int) as PHYSFS_uint32) !=
               (NumEntries * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load baseDefendLocation for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* read base defend priorities */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       baseDefendLocPrior[nPlayer as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       (NumEntries * 2 as libc::c_int) as PHYSFS_uint32) !=
               (NumEntries * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load baseDefendLocPrior for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* ***********************************/
		/*		Oil attack locations		*/
		/* ***********************************/
        /* read MAX_OIL_DEFEND_LOCATIONS */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to read max number of oil attack locations for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* check it's the same as current MAX_OIL_DEFEND_LOCATIONS */
        if NumEntries > 100 as libc::c_int {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): saved MAX_OIL_DEFEND_LOCATIONS and current one don\'t match (%d / %d)\x00"
                      as *const u8 as *const libc::c_char, NumEntries,
                  100 as libc::c_int);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"num oil attack loc: %d", NumEntries);
        /* read oil locations */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       oilDefendLocation[nPlayer as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       (NumEntries * 2 as libc::c_int) as PHYSFS_uint32) !=
               (NumEntries * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load oilDefendLocation for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* read oil location priority */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       oilDefendLocPrior[nPlayer as usize].as_mut_ptr() as
                           *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       (NumEntries * 2 as libc::c_int) as PHYSFS_uint32) !=
               (NumEntries * 2 as libc::c_int) as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load oilDefendLocPrior for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* ***************************/
		/*		Oil Resources		*/
		/* ***************************/
        /* read MAX_OIL_LOCATIONS */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to load MAX_OIL_LOCATIONS for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        /* check it's the same as current MAX_OIL_LOCATIONS */
        if NumEntries > 300 as libc::c_int {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): saved MAX_OIL_LOCATIONS and current one don\'t match (%d / %d)\x00"
                      as *const u8 as *const libc::c_char, NumEntries,
                  300 as libc::c_int);
            return 0 as libc::c_int
        }
        /* Read number of Oil Resources */
        if PHYSFS_read(aiSaveFile[nPlayer as usize],
                       &mut NumEntries as *mut SDWORD as *mut libc::c_void,
                       ::std::mem::size_of::<SDWORD>() as libc::c_ulong,
                       1 as libc::c_int as PHYSFS_uint32) !=
               1 as libc::c_int as libc::c_longlong {
            debug(LOG_ERROR,
                  b"ReadAISaveData(): Failed to read Oil Resources count for player \'%d\'\x00"
                      as *const u8 as *const libc::c_char, nPlayer);
            return 0 as libc::c_int
        }
        //debug(LOG_ERROR,"Num oil: %d", NumEntries);
        if NumEntries > 0 as libc::c_int {
            //any oil resources were saved?
            /* Read Oil Resources coordinates */
            if PHYSFS_read(aiSaveFile[nPlayer as usize],
                           PosXY.as_mut_ptr() as *mut libc::c_void,
                           ::std::mem::size_of::<UDWORD>() as libc::c_ulong,
                           (NumEntries * 2 as libc::c_int) as PHYSFS_uint32)
                   != (NumEntries * 2 as libc::c_int) as libc::c_longlong {
                debug(LOG_ERROR,
                      b"ReadAISaveData(): Failed to read Oil Resources coordinates for player \'%d\'\x00"
                          as *const u8 as *const libc::c_char, nPlayer);
                return 0 as libc::c_int
            }
            i = 0 as libc::c_int;
            while i < NumEntries {
                Found = 0 as libc::c_int;
                //if(!Found)		//Couldn't find oil resource with this coords on the map
				//	printf_console("!!Failed to match oil resource #%d at x: %d y: %d", i,PosXY[i * 2]/128,PosXY[i * 2 + 1]/128);
                if i < 300 as libc::c_int {
                    //re-read into remory
                    //didn't max out?
                    oilLocation[nPlayer as
                                    usize][i as
                                               usize][0 as libc::c_int as
                                                          usize] =
                        PosXY[(i * 2 as libc::c_int) as usize] as SDWORD; //x
                    oilLocation[nPlayer as
                                    usize][i as
                                               usize][1 as libc::c_int as
                                                          usize] =
                        PosXY[(i * 2 as libc::c_int + 1 as libc::c_int) as
                                  usize] as SDWORD
                }
                psFeature = apsFeatureLists[0 as libc::c_int as usize];
                while !psFeature.is_null() {
                    if (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                        /* Iterate through all Oil Resources and try to match coordinates */
                        //Oil resource
                        if (*psFeature).visible[nPlayer as usize] == 0 {
                            //Not visible yet
                            if PosXY[(i * 2 as libc::c_int) as usize] ==
                                   (*psFeature).x as libc::c_uint &&
                                   PosXY[(i * 2 as libc::c_int +
                                              1 as libc::c_int) as usize] ==
                                       (*psFeature).y as libc::c_uint {
                                /* Found it */
                                //printf_console("Matched oil resource at x: %d y: %d", PosXY[i * 2]/128,PosXY[i * 2 + 1]/128);
                                (*psFeature).visible[nPlayer as usize] =
                                    1 as libc::c_int as
                                        UBYTE; //Make visible for AI
                                Found =
                                    1 as
                                        libc::c_int; //How many derricks/oil resources will be saved
                                break ;
                            }
                        }
                    }
                    psFeature = (*psFeature).psNext
                }
                i += 1
            }
        }
    }
    return PHYSFS_close(aiSaveFile[nPlayer as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn OilResourceAt(mut OilX: UDWORD, mut OilY: UDWORD,
                                       mut VisibleToPlayer: SDWORD) -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    let mut NumEntries: SDWORD = 0 as libc::c_int;
    let mut Found: BOOL = 0;
    /* Iterate through all Oil Resources and try to match coordinates */
    psFeature = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeature.is_null() {
        if (*(*psFeature).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            //Oil resource
            if VisibleToPlayer < 0 as libc::c_int ||
                   (*psFeature).visible[VisibleToPlayer as usize] == 0 {
                //Not visible yet
                if OilX == (*psFeature).x as libc::c_uint &&
                       OilY == (*psFeature).y as libc::c_uint {
                    /* Found it */
                    printf_console(b"Matched oil resource at x: %d y: %d\x00"
                                       as *const u8 as *const libc::c_char,
                                   OilX.wrapping_div(128 as libc::c_int as
                                                         libc::c_uint),
                                   OilY.wrapping_div(128 as libc::c_int as
                                                         libc::c_uint)); //Make visible for AI
                    (*psFeature).visible[VisibleToPlayer as usize] =
                        1 as libc::c_int as UBYTE;
                    Found = 1 as libc::c_int;
                    break ;
                }
            }
        }
        psFeature = (*psFeature).psNext
    }
    return 1 as libc::c_int;
}
//x and y are passed by script, find out if this loc is close to
//an already stored loc, if yes then increase its priority
#[no_mangle]
pub unsafe extern "C" fn StoreBaseDefendLoc(mut x: SDWORD, mut y: SDWORD,
                                            mut nPlayer: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut found: BOOL = 0;
    index = GetBaseDefendLocIndex(x, y, nPlayer);
    if index < 0 as libc::c_int {
        //this one is new
        //find an empty element
        found = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 30 as libc::c_int {
            if baseDefendLocation[nPlayer as
                                      usize][i as
                                                 usize][0 as libc::c_int as
                                                            usize] <
                   0 as libc::c_int {
                //not initialized yet
                //addConsoleMessage("Base defense location - NEW LOCATION.", RIGHT_JUSTIFY);
                baseDefendLocation[nPlayer as
                                       usize][i as
                                                  usize][0 as libc::c_int as
                                                             usize] = x;
                baseDefendLocation[nPlayer as
                                       usize][i as
                                                  usize][1 as libc::c_int as
                                                             usize] = y;
                baseDefendLocPrior[nPlayer as usize][i as usize] =
                    1 as libc::c_int;
                found = 1 as libc::c_int;
                return 1 as libc::c_int
            }
            i += 1
        }
        addConsoleMessage(b"Base defense location - NO SPACE LEFT.\x00" as
                              *const u8 as *const libc::c_char as *mut STRING,
                          RIGHT_JUSTIFY);
        return 0 as libc::c_int
    } else {
        //not enough space to store
        //this one already stored
        //addConsoleMessage("Base defense location - INCREASED PRIORITY.",RIGHT_JUSTIFY);
        baseDefendLocPrior[nPlayer as usize][index as usize] +=
            1; //higher the priority
        if baseDefendLocPrior[nPlayer as usize][index as usize] ==
               0x7fffffff as libc::c_int {
            baseDefendLocPrior[nPlayer as usize][index as usize] =
                1 as libc::c_int
        } //start all over
        SortBaseDefendLoc(nPlayer); //in world units
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetBaseDefendLocIndex(mut x: SDWORD, mut y: SDWORD,
                                               mut nPlayer: SDWORD)
 -> SDWORD {
    let mut i: SDWORD = 0;
    let mut range: SDWORD = 0;
    range = (8 as libc::c_int) << 7 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        if baseDefendLocation[nPlayer as
                                  usize][i as
                                             usize][0 as libc::c_int as usize]
               > 0 as libc::c_int &&
               baseDefendLocation[nPlayer as
                                      usize][i as
                                                 usize][1 as libc::c_int as
                                                            usize] >
                   0 as libc::c_int {
            //if this one initialized
            //check if very close to an already stored location
            if dirtySqrt(x, y,
                         baseDefendLocation[nPlayer as
                                                usize][i as
                                                           usize][0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize],
                         baseDefendLocation[nPlayer as
                                                usize][i as
                                                           usize][1 as
                                                                      libc::c_int
                                                                      as
                                                                      usize])
                   < range as libc::c_uint {
                return i
                //end here
            }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
//aiexperience.h
//(Max number of derricks or oil resources) / 2
//max number of attack locations to store
//max number of oil locations to store
//max number of base locations to store
//if within this range, consider it the same loc
//sort the priorities, placing the higher ones at the top
#[no_mangle]
pub unsafe extern "C" fn SortBaseDefendLoc(mut nPlayer: SDWORD) -> BOOL {
    let mut i: SDWORD = 0; //nothing sorted yet, point at last elem
    let mut prior: SDWORD = 0;
    let mut temp: SDWORD = 0;
    let mut LowestPrior: SDWORD = 0;
    let mut LowestIndex: SDWORD = 0;
    let mut SortBound: SDWORD = 0;
    SortBound = 30 as libc::c_int - 1 as libc::c_int;
    while SortBound >= 0 as libc::c_int {
        //while didn't reach the top
        LowestPrior = 0x7fffffff as libc::c_int - 1 as libc::c_int;
        LowestIndex = -(1 as libc::c_int);
        //in any case lower the boundry, even if didn't swap
        i = 0 as libc::c_int;
        while i <= SortBound {
            prior = baseDefendLocPrior[nPlayer as usize][i as usize];
            if prior < LowestPrior {
                //find lowest element
                //lower and isn't a flag meaning this one wasn't initialized with anything
                LowestPrior = prior;
                LowestIndex = i
            }
            i += 1
        }
        if LowestIndex < 0 as libc::c_int {
            //huh, nothing found? (probably nothing set yet, no experience)
            //debug(LOG_ERROR,"sortBaseDefendLoc() - No lowest elem found");
            return 1 as libc::c_int
        }
        if LowestPrior <
               baseDefendLocPrior[nPlayer as usize][SortBound as usize] {
            //swap
            //need to swap? (might be the same elem)
            //priority
            temp = baseDefendLocPrior[nPlayer as usize][SortBound as usize];
            baseDefendLocPrior[nPlayer as usize][SortBound as usize] =
                baseDefendLocPrior[nPlayer as usize][LowestIndex as usize];
            baseDefendLocPrior[nPlayer as usize][LowestIndex as usize] = temp;
            temp =
                baseDefendLocation[nPlayer as
                                       usize][SortBound as
                                                  usize][0 as libc::c_int as
                                                             usize];
            baseDefendLocation[nPlayer as
                                   usize][SortBound as
                                              usize][0 as libc::c_int as
                                                         usize] =
                baseDefendLocation[nPlayer as
                                       usize][LowestIndex as
                                                  usize][0 as libc::c_int as
                                                             usize];
            baseDefendLocation[nPlayer as
                                   usize][LowestIndex as
                                              usize][0 as libc::c_int as
                                                         usize] = temp;
            temp =
                baseDefendLocation[nPlayer as
                                       usize][SortBound as
                                                  usize][1 as libc::c_int as
                                                             usize];
            baseDefendLocation[nPlayer as
                                   usize][SortBound as
                                              usize][1 as libc::c_int as
                                                         usize] =
                baseDefendLocation[nPlayer as
                                       usize][LowestIndex as
                                                  usize][1 as libc::c_int as
                                                             usize];
            baseDefendLocation[nPlayer as
                                   usize][LowestIndex as
                                              usize][1 as libc::c_int as
                                                         usize] = temp
        }
        SortBound -= 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BaseExperienceDebug(mut nPlayer: SDWORD) {
    let mut i: SDWORD = 0;
    printf_console(b"-------------\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 30 as libc::c_int {
        printf_console(b"%d) %d - %d (%d)\x00" as *const u8 as
                           *const libc::c_char, i,
                       baseDefendLocation[nPlayer as
                                              usize][i as
                                                         usize][0 as
                                                                    libc::c_int
                                                                    as usize]
                           >> 7 as libc::c_int,
                       baseDefendLocation[nPlayer as
                                              usize][i as
                                                         usize][1 as
                                                                    libc::c_int
                                                                    as usize]
                           >> 7 as libc::c_int,
                       baseDefendLocPrior[nPlayer as usize][i as usize]);
        i += 1
    }
    printf_console(b"-------------\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn OilExperienceDebug(mut nPlayer: SDWORD) {
    let mut i: SDWORD = 0;
    printf_console(b"-------------\x00" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        printf_console(b"%d) %d - %d (%d)\x00" as *const u8 as
                           *const libc::c_char, i,
                       oilDefendLocation[nPlayer as
                                             usize][i as
                                                        usize][0 as
                                                                   libc::c_int
                                                                   as usize]
                           >> 7 as libc::c_int,
                       oilDefendLocation[nPlayer as
                                             usize][i as
                                                        usize][1 as
                                                                   libc::c_int
                                                                   as usize]
                           >> 7 as libc::c_int,
                       oilDefendLocPrior[nPlayer as usize][i as usize]);
        i += 1
    }
    printf_console(b"-------------\x00" as *const u8 as *const libc::c_char);
}
//x
//y
//x and y are passed by script, find out if this loc is close to
//an already stored loc, if yes then increase its priority
#[no_mangle]
pub unsafe extern "C" fn StoreOilDefendLoc(mut x: SDWORD, mut y: SDWORD,
                                           mut nPlayer: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut found: BOOL = 0;
    index = GetOilDefendLocIndex(x, y, nPlayer);
    if index < 0 as libc::c_int {
        //this one is new
        //find an empty element
        found = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            if oilDefendLocation[nPlayer as
                                     usize][i as
                                                usize][0 as libc::c_int as
                                                           usize] <
                   0 as libc::c_int {
                //not initialized yet
                //addConsoleMessage("Oil defense location - NEW LOCATION.", RIGHT_JUSTIFY);
                oilDefendLocation[nPlayer as
                                      usize][i as
                                                 usize][0 as libc::c_int as
                                                            usize] = x;
                oilDefendLocation[nPlayer as
                                      usize][i as
                                                 usize][1 as libc::c_int as
                                                            usize] = y;
                oilDefendLocPrior[nPlayer as usize][i as usize] =
                    1 as libc::c_int;
                found = 1 as libc::c_int;
                return 1 as libc::c_int
            }
            i += 1
        }
        addConsoleMessage(b"Oil defense location - NO SPACE LEFT.\x00" as
                              *const u8 as *const libc::c_char as *mut STRING,
                          RIGHT_JUSTIFY);
        return 0 as libc::c_int
    } else {
        //not enough space to store
        //this one already stored
        //addConsoleMessage("Oil defense location - INCREASED PRIORITY.",RIGHT_JUSTIFY);
        oilDefendLocPrior[nPlayer as usize][index as usize] +=
            1; //higher the priority
        if oilDefendLocPrior[nPlayer as usize][index as usize] ==
               0x7fffffff as libc::c_int {
            oilDefendLocPrior[nPlayer as usize][index as usize] =
                1 as libc::c_int
        } //start all over
        SortOilDefendLoc(nPlayer); //in world units
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GetOilDefendLocIndex(mut x: SDWORD, mut y: SDWORD,
                                              mut nPlayer: SDWORD) -> SDWORD {
    let mut i: SDWORD = 0;
    let mut range: SDWORD = 0;
    range = (8 as libc::c_int) << 7 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if oilDefendLocation[nPlayer as
                                 usize][i as usize][0 as libc::c_int as usize]
               > 0 as libc::c_int &&
               oilDefendLocation[nPlayer as
                                     usize][i as
                                                usize][1 as libc::c_int as
                                                           usize] >
                   0 as libc::c_int {
            //if this one initialized
            //check if very close to an already stored location
            if dirtySqrt(x, y,
                         oilDefendLocation[nPlayer as
                                               usize][i as
                                                          usize][0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize],
                         oilDefendLocation[nPlayer as
                                               usize][i as
                                                          usize][1 as
                                                                     libc::c_int
                                                                     as
                                                                     usize]) <
                   range as libc::c_uint {
                return i
                //end here
            }
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
//sort the priorities, placing the higher ones at the top
#[no_mangle]
pub unsafe extern "C" fn SortOilDefendLoc(mut nPlayer: SDWORD) -> BOOL {
    let mut i: SDWORD = 0; //nothing sorted yet, point at last elem
    let mut prior: SDWORD = 0;
    let mut temp: SDWORD = 0;
    let mut LowestPrior: SDWORD = 0;
    let mut LowestIndex: SDWORD = 0;
    let mut SortBound: SDWORD = 0;
    SortBound = 100 as libc::c_int - 1 as libc::c_int;
    while SortBound >= 0 as libc::c_int {
        //while didn't reach the top
        LowestPrior = 0x7fffffff as libc::c_int - 1 as libc::c_int;
        LowestIndex = -(1 as libc::c_int);
        //in any case lower the boundry, even if didn't swap
        i = 0 as libc::c_int;
        while i <= SortBound {
            prior = oilDefendLocPrior[nPlayer as usize][i as usize];
            if prior < LowestPrior {
                //find lowest element
                //lower and isn't a flag meaning this one wasn't initialized with anything
                LowestPrior = prior;
                LowestIndex = i
            }
            i += 1
        }
        if LowestIndex < 0 as libc::c_int {
            //huh, nothing found? (probably nothing set yet, no experience)
            //debug(LOG_ERROR,"sortBaseDefendLoc() - No lowest elem found");
            return 1 as libc::c_int
        }
        if LowestPrior <
               oilDefendLocPrior[nPlayer as usize][SortBound as usize] {
            //swap
            //need to swap? (might be the same elem)
            //priority
            temp = oilDefendLocPrior[nPlayer as usize][SortBound as usize];
            oilDefendLocPrior[nPlayer as usize][SortBound as usize] =
                oilDefendLocPrior[nPlayer as usize][LowestIndex as usize];
            oilDefendLocPrior[nPlayer as usize][LowestIndex as usize] = temp;
            temp =
                oilDefendLocation[nPlayer as
                                      usize][SortBound as
                                                 usize][0 as libc::c_int as
                                                            usize];
            oilDefendLocation[nPlayer as
                                  usize][SortBound as
                                             usize][0 as libc::c_int as usize]
                =
                oilDefendLocation[nPlayer as
                                      usize][LowestIndex as
                                                 usize][0 as libc::c_int as
                                                            usize];
            oilDefendLocation[nPlayer as
                                  usize][LowestIndex as
                                             usize][0 as libc::c_int as usize]
                = temp;
            temp =
                oilDefendLocation[nPlayer as
                                      usize][SortBound as
                                                 usize][1 as libc::c_int as
                                                            usize];
            oilDefendLocation[nPlayer as
                                  usize][SortBound as
                                             usize][1 as libc::c_int as usize]
                =
                oilDefendLocation[nPlayer as
                                      usize][LowestIndex as
                                                 usize][1 as libc::c_int as
                                                            usize];
            oilDefendLocation[nPlayer as
                                  usize][LowestIndex as
                                             usize][1 as libc::c_int as usize]
                = temp
        }
        SortBound -= 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CanRememberPlayerBaseLoc(mut lookingPlayer: SDWORD,
                                                  mut enemyPlayer: SDWORD)
 -> BOOL {
    if lookingPlayer < 0 as libc::c_int || enemyPlayer < 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if lookingPlayer >= 8 as libc::c_int || enemyPlayer >= 8 as libc::c_int {
        return 0 as libc::c_int
    }
    if baseLocation[lookingPlayer as
                        usize][enemyPlayer as
                                   usize][0 as libc::c_int as usize] <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if baseLocation[lookingPlayer as
                        usize][enemyPlayer as
                                   usize][1 as libc::c_int as usize] <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CanRememberPlayerBaseDefenseLoc(mut player: SDWORD,
                                                         mut index: SDWORD)
 -> BOOL {
    if player < 0 as libc::c_int { return 0 as libc::c_int }
    if player >= 8 as libc::c_int { return 0 as libc::c_int }
    if index < 0 as libc::c_int || index >= 30 as libc::c_int {
        return 0 as libc::c_int
    }
    if baseDefendLocation[player as
                              usize][index as
                                         usize][0 as libc::c_int as usize] <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    if baseDefendLocation[player as
                              usize][index as
                                         usize][1 as libc::c_int as usize] <=
           0 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CanRememberPlayerOilDefenseLoc(mut player: SDWORD,
                                                        mut index: SDWORD)
 -> BOOL {
    if player < 0 as libc::c_int { return 0 as libc::c_int }
    if player >= 8 as libc::c_int { return 0 as libc::c_int }
    if index < 0 as libc::c_int || index >= 30 as libc::c_int {
        return 0 as libc::c_int
    }
    if oilDefendLocation[player as
                             usize][index as usize][0 as libc::c_int as usize]
           <= 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if oilDefendLocation[player as
                             usize][index as usize][1 as libc::c_int as usize]
           <= 0 as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//x
//y
