use ::libc;
extern "C" {
    pub type _formation;
    /* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
    /*
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    // Set the tile no draw flags for a structure
    #[no_mangle]
    fn setStructTileDraw(psStruct: *mut STRUCTURE);
    /*returns the status of the flag*/
    #[no_mangle]
    fn getSatUplinkExists(player: UDWORD) -> BOOL;
    /*checks if the structure has a Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    /*checks if the structure has a VTOL Counter Battery sensor attached - returns 
TRUE if it has*/
    #[no_mangle]
    fn structVTOLCBSensor(psStruct: *mut STRUCTURE) -> BOOL;
    // Set the tile no draw flags for a structure
    #[no_mangle]
    fn setFeatTileDraw(psFeat: *mut FEATURE);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    /*Access functions for the upgradeable stats of a ECM*/
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
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
    /* cast a ray from x,y (world coords) at angle ray (0-NUM_RAYS) */
    #[no_mangle]
    fn rayCast(x: UDWORD, y: UDWORD, ray: UDWORD, length: UDWORD,
               callback: RAY_CALLBACK);
    /* Distance of a point from a line.
 * NOTE: This is not 100% accurate - it approximates to get the square root
 *
 * This is based on Graphics Gems II setion 1.3
 */
    #[no_mangle]
    fn rayPointDist(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD,
                    px: SDWORD, py: SDWORD) -> SDWORD;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /*Add a messgae to the list */
    #[no_mangle]
    fn addMessage(msgType: UDWORD, proxPos: BOOL, player: UDWORD)
     -> *mut MESSAGE;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    // initialise the grid system to start iterating through units that
// could affect a location (x,y in world coords)
    #[no_mangle]
    fn gridStartIterate(x: SDWORD, y: SDWORD);
    // get the next object that could affect a location,
// should only be called after gridStartIterate
    #[no_mangle]
    fn gridIterate() -> *mut BASE_OBJECT;
    // tell the cluster system that an objects visibility has changed
    #[no_mangle]
    fn clustObjectSeen(psObj: *mut BASE_OBJECT, psViewer: *mut BASE_OBJECT);
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn avInformOfChange(x: SDWORD, y: SDWORD);
    #[no_mangle]
    fn getRevealStatus() -> BOOL;
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
pub type ECM_STATS = _ecm_stats;
pub type WEAPON_STATS = _weapon_stats;
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
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
// Feature armour
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
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
/* The raycast intersection callback.
 * Return FALSE if no more points are required, TRUE otherwise
 */
pub type RAY_CALLBACK
    =
    Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD) -> BOOL>;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
/*
 * messageDef.h
 *
 * Message structure definitions
 */
pub type MESSAGE_TYPE = _message_type;
pub type MSG_VIEWDATA = *mut libc::c_void;
// Research message
// Campaign message
// Mission Report messages
// Proximity message
//base structure for each message
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _message {
    pub type_0: MESSAGE_TYPE,
    pub id: UDWORD,
    pub pViewData: *mut MSG_VIEWDATA,
    pub read: BOOL,
    pub player: UDWORD,
    pub psNext: *mut _message,
}
pub type MESSAGE = _message;
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
//pointer to the next in the list
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
// List of objects that could intersect a ray
static mut apsTestObjects: [*mut BASE_OBJECT; 100] =
    [0 as *const BASE_OBJECT as *mut BASE_OBJECT; 100];
static mut numTestObjects: SDWORD = 0 as libc::c_int;
// list of objects intersecting a ray and the distance to them
static mut apsRayObjects: [*mut BASE_OBJECT; 20] =
    [0 as *const BASE_OBJECT as *mut BASE_OBJECT; 20];
static mut aRayObjDist: [SDWORD; 20] = [0; 20];
static mut numRayObjects: SDWORD = 0 as libc::c_int;
//static BOOL				gotRayObjects = FALSE;
// which object is being considered by the callback
static mut currObj: SDWORD = 0;
// rate to change visibility level
// fractional accumulator of how much to change visibility this frame
static mut visLevelIncAcc: FRACT = 0.;
static mut visLevelDecAcc: FRACT = 0.;
// integer amount to change visiblility this turn
static mut visLevelInc: SDWORD = 0;
static mut visLevelDec: SDWORD = 0;
/* Variables for the visibility callback */
static mut rayPlayer: SDWORD = 0;
// The player the ray is being cast for
static mut startH: SDWORD = 0;
// The height at the view point
static mut currG: SDWORD = 0;
// The current obscuring gradient
static mut lastH: SDWORD = 0;
static mut lastD: SDWORD = 0;
// The last height and distance
static mut rayStart: BOOL = 0;
// Whether this is the first point on the ray
static mut tarDist: SDWORD = 0;
// The distance to the ray target
static mut blockingWall: BOOL = 0;
// Whether walls block line of sight
static mut finalX: SDWORD = 0;
static mut finalY: SDWORD = 0;
// The final tile of the ray cast
static mut numWalls: SDWORD = 0;
// Whether the LOS has hit a wall
static mut wallX: SDWORD = 0;
static mut wallY: SDWORD = 0;
/* Terrain types that could obscure LOS */
/*TER_STONE*/
/* The distance under which visibility is automatic */
// initialise the visibility stuff
// the position of a wall if it is on the LOS
// initialise the visibility stuff
#[no_mangle]
pub unsafe extern "C" fn visInitialise() -> BOOL {
    visLevelIncAcc = 0 as libc::c_int as FRACT;
    visLevelDecAcc = 0 as libc::c_int as FRACT;
    visLevelInc = 0 as libc::c_int;
    visLevelDec = 0 as libc::c_int;
    return 1 as libc::c_int;
}
// update the visibility reduction
// update the visibility change levels
#[no_mangle]
pub unsafe extern "C" fn visUpdateLevel() {
    visLevelIncAcc +=
        frameTime as FRACT *
            ((255 as libc::c_int * 2 as libc::c_int) as libc::c_float /
                 1000 as libc::c_int as libc::c_float);
    visLevelInc = visLevelIncAcc as SDWORD;
    visLevelIncAcc -= visLevelInc as FRACT;
    visLevelDecAcc +=
        frameTime as FRACT *
            (50 as libc::c_int as libc::c_float /
                 1000 as libc::c_int as libc::c_float);
    visLevelDec = visLevelDecAcc as SDWORD;
    visLevelDecAcc -= visLevelDec as FRACT;
}
/* Return the radius a base object covers on the map */
unsafe extern "C" fn visObjRadius(mut psObject: *mut BASE_OBJECT) -> SDWORD {
    let mut radius: SDWORD = 0;
    match (*psObject).type_0 as libc::c_uint {
        0 => { radius = (*(*psObject).sDisplay.imd).radius }
        1 => { radius = (*(*psObject).sDisplay.imd).radius }
        2 => { radius = (*(*psObject).sDisplay.imd).radius }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"visObjRadius: unknown object type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"visibility.c\x00" as *const u8 as *const libc::c_char,
                      137 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"visObjRadius\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            radius = 0 as libc::c_int
        }
    }
    return radius;
}
unsafe extern "C" fn visObjHeight(mut psObject: *mut BASE_OBJECT) -> SDWORD {
    let mut height: SDWORD = 0;
    match (*psObject).type_0 as libc::c_uint {
        0 => { height = 80 as libc::c_int }
        1 => { height = (*(*psObject).sDisplay.imd).ymax }
        2 => { height = (*(*psObject).sDisplay.imd).ymax }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"visObjHeight: unknown object type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"visibility.c\x00" as *const u8 as *const libc::c_char,
                      162 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"visObjHeight\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            height = 0 as libc::c_int
        }
    }
    return height;
}
/* The terrain revealing ray callback */
unsafe extern "C" fn rayTerrainCallback(mut x: SDWORD, mut y: SDWORD,
                                        mut dist: SDWORD) -> BOOL {
    let mut newH: SDWORD = 0; // The new gradient
    let mut newG: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if x >= 0 as libc::c_int && x < (mapWidth as SDWORD) << 7 as libc::c_int
           && y >= 0 as libc::c_int &&
           y < (mapHeight as SDWORD) << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"rayTerrainCallback: coords off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x >= 0 as libc::c_int && x < (mapWidth as SDWORD) << 7 as libc::c_int
           && y >= 0 as libc::c_int &&
           y < (mapHeight as SDWORD) << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"visibility.c\x00" as *const u8 as *const libc::c_char,
              179 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"rayTerrainCallback\x00")).as_ptr(),
              b"x >= 0 && x < ((SDWORD)mapWidth << TILE_SHIFT) && y >= 0 && y < ((SDWORD)mapHeight << TILE_SHIFT)\x00"
                  as *const u8 as *const libc::c_char);
    };
    psTile =
        mapTile((x >> 7 as libc::c_int) as UDWORD,
                (y >> 7 as libc::c_int) as UDWORD);
    /* Not true visibility - done on sensor range */
    if dist == 0 as libc::c_int {
        //Complete hack PD.. John what should happen if dist is 0 ???
        debug(LOG_NEVER,
              b"rayTerrainCallback: dist == 0, will divide by zero\n\x00" as
                  *const u8 as *const libc::c_char);
        dist = 1 as libc::c_int
    }
    newH = (*psTile).height as libc::c_int * 2 as libc::c_int;
    newG = (newH - startH) * 10000 as libc::c_int / dist;
    if newG >= currG {
        currG = newG;
        (*psTile).tileVisBits =
            ((*psTile).tileVisBits as libc::c_int |
                 (1 as libc::c_int) << rayPlayer) as UBYTE;
        if rayPlayer as libc::c_uint == selectedPlayer &&
               bDisplaySensorRange == 0 {
            (*psTile).inRange = 0xff as libc::c_int as UBYTE
        }
        // new - ask alex M
        if selectedPlayer != rayPlayer as libc::c_uint &&
               (bMultiPlayer != 0 &&
                    game.type_0 as libc::c_int == 13 as libc::c_int &&
                    aiCheckAlliances(selectedPlayer, rayPlayer as UDWORD) !=
                        0) {
            (*psTile).tileVisBits =
                ((*psTile).tileVisBits as libc::c_int |
                     (1 as libc::c_int) << selectedPlayer) as UBYTE
        }
        // new - ask Alex M
	/* Not true visibility - done on sensor range */
        if getRevealStatus() != 0 {
            if rayPlayer as UDWORD == selectedPlayer ||
                   bMultiPlayer != 0 &&
                       game.type_0 as libc::c_int == 13 as libc::c_int &&
                       aiCheckAlliances(selectedPlayer, rayPlayer as UDWORD)
                           != 0 {
                // can see opponent moving
                // new - ask AM
                avInformOfChange(x >> 7 as libc::c_int,
                                 y >> 7 as libc::c_int);
                //				SET_TILE_SENSOR(psTile);
            }
        }
    }
    return 1 as libc::c_int;
}
/* The los ray callback */
unsafe extern "C" fn rayLOSCallback(mut x: SDWORD, mut y: SDWORD,
                                    mut dist: SDWORD) -> BOOL {
    let mut newG: SDWORD = 0; // The new gradient
    //	MAPTILE		*psTile;
    let mut distSq: SDWORD = 0;
    let mut tileX: SDWORD = 0;
    let mut tileY: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if x >= 0 as libc::c_int && x < (mapWidth as SDWORD) << 7 as libc::c_int
           && y >= 0 as libc::c_int &&
           y < (mapHeight as SDWORD) << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"rayLOSCallback: coords off map\x00" as *const u8 as
                  *const libc::c_char);
    };
    if x >= 0 as libc::c_int && x < (mapWidth as SDWORD) << 7 as libc::c_int
           && y >= 0 as libc::c_int &&
           y < (mapHeight as SDWORD) << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"visibility.c\x00" as *const u8 as *const libc::c_char,
              246 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"rayLOSCallback\x00")).as_ptr(),
              b"x >= 0 && x < ((SDWORD)mapWidth << TILE_SHIFT) && y >= 0 && y < ((SDWORD)mapHeight << TILE_SHIFT)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /*	if(dist == 0) {	//Complete hack PD.. John what should happen if dist is 0 ???
#ifndef PSX
		DBPRINTF(("rayTerrainCallback: dist == 0, will divide by zero\n"));
#endif
		dist = 1;
	}*/
    distSq = dist * dist;
    if rayStart != 0 {
        rayStart = 0 as libc::c_int
    } else {
        // see if the ray hit an object on the last tile
/*		if (currObj < numRayObjects &&
			distSq > aRayObjDist[currObj])
		{
			lastH += visObjHeight(apsRayObjects[currObj]);
			currObj += 1;
		}*/
        // Calculate the current LOS gradient
        newG = (lastH - startH) * 10000 as libc::c_int / lastD;
        if newG >= currG { currG = newG }
    }
    // See if the ray has reached the target
    if distSq >= tarDist {
        lastD = dist;
        return 0 as libc::c_int
    } else {
        // Store the height at this tile for next time round
	//	psTile = mapTile(x >> TILE_SHIFT, y >> TILE_SHIFT);
	//	lastH = psTile->height * ELEVATION_SCALE;
        tileX = x >> 7 as libc::c_int;
        tileY = y >> 7 as libc::c_int;
        if blockingWall != 0 && !(tileX == finalX && tileY == finalY) {
            psTile =
                mapTile((x >> 7 as libc::c_int) as UDWORD,
                        (y >> 7 as libc::c_int) as UDWORD);
            if (*psTile).tileInfoBits as libc::c_int & 0x20 as libc::c_int !=
                   0 &&
                   (*psTile).tileInfoBits as libc::c_int & 0x8 as libc::c_int
                       == 0 {
                lastH =
                    2 as libc::c_int * 0xff as libc::c_int * 2 as libc::c_int;
                //			return FALSE;
                numWalls += 1 as libc::c_int;
                wallX = x;
                wallY = y
            } else { lastH = map_Height(x as UDWORD, y as UDWORD) as SDWORD }
        } else { lastH = map_Height(x as UDWORD, y as UDWORD) as SDWORD }
        lastD = dist
    }
    return 1 as libc::c_int;
}
//			currG = UBYTE_MAX * ELEVATION_SCALE * GRAD_MUL / lastD;
//SDWORD currRayAng;
//BOOL currRayPending = FALSE;
// Call this once per game cycle to update visible terrain ray angle when spreading processor
// load.
//
#[no_mangle]
pub unsafe extern "C" fn visTilesUpdateLoadSpread() {
    //	if(currRayPending) {
//		currRayAng += VTRAYSTEP;
//		if(currRayAng >= (NUM_RAYS/4)) {
//			currRayAng = 0;	//Pending = FALSE;
//		}
//	}
}
#[no_mangle]
pub unsafe extern "C" fn visTilesPending(mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"visTilesPending : Only implemented for droids\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"visibility.c\x00" as *const u8 as *const libc::c_char,
              345 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"visTilesPending\x00")).as_ptr(),
              b"psObj->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    return (*(psObj as *mut DROID)).updateFlags as libc::c_int &
               0x1 as libc::c_int;
}
/* Check which tiles can be seen by an object */
/* Check which tiles can be seen by an object */
#[no_mangle]
pub unsafe extern "C" fn visTilesUpdate(mut psObj: *mut BASE_OBJECT,
                                        mut SpreadLoad: BOOL) {
    let mut range: SDWORD = 0;
    let mut ray: SDWORD = 0;
    // Get the sensor Range and power
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            // Done whenever a droid is built or moves to a new tile.
            range = (*(psObj as *mut DROID)).sensorRange as SDWORD
        }
        1 => {
            // Only done when structure initialy built.
            if SpreadLoad == 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"visTilesUpdate : Can only spread load for droids\x00"
                          as *const u8 as *const libc::c_char);
            };
            if SpreadLoad == 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"visibility.c\x00" as *const u8 as *const libc::c_char,
                      364 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"visTilesUpdate\x00")).as_ptr(),
                      b"SpreadLoad == FALSE\x00" as *const u8 as
                          *const libc::c_char);
            };
            // can't spread load for structures.
            range = (*(psObj as *mut STRUCTURE)).sensorRange as SDWORD
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"visTilesUpdate: visibility checking is only implemented forunits and structures\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"visibility.c\x00" as *const u8 as *const libc::c_char,
                      370 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"visTilesUpdate\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return
        }
    }
    rayPlayer = (*psObj).player as SDWORD;
    if SpreadLoad != 0 {
        // Just do 4 rays at right angles.
        let mut psDroid: *mut DROID = psObj as *mut DROID;
        let mut currRayAng: SDWORD = 0;
        if (*psDroid).updateFlags as libc::c_int & 0x1 as libc::c_int ==
               0 as libc::c_int {
            (*psDroid).currRayAng = 0 as libc::c_int as UBYTE;
            (*psDroid).updateFlags =
                ((*psDroid).updateFlags as libc::c_int | 0x1 as libc::c_int)
                    as UBYTE
        }
        currRayAng = (*psDroid).currRayAng as SDWORD;
        // Cast the rays from the viewer
        startH = (*psObj).z as libc::c_int + visObjHeight(psObj);
        currG = -(0xff as libc::c_int) * 10000 as libc::c_int;
        rayCast((*psObj).x as UDWORD, (*psObj).y as UDWORD,
                currRayAng as UDWORD, range as UDWORD,
                Some(rayTerrainCallback as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                             -> BOOL));
        startH = (*psObj).z as libc::c_int + visObjHeight(psObj);
        currG = -(0xff as libc::c_int) * 10000 as libc::c_int;
        rayCast((*psObj).x as UDWORD, (*psObj).y as UDWORD,
                ((currRayAng + 360 as libc::c_int / 4 as libc::c_int) %
                     360 as libc::c_int) as UDWORD, range as UDWORD,
                Some(rayTerrainCallback as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                             -> BOOL));
        startH = (*psObj).z as libc::c_int + visObjHeight(psObj);
        currG = -(0xff as libc::c_int) * 10000 as libc::c_int;
        rayCast((*psObj).x as UDWORD, (*psObj).y as UDWORD,
                ((currRayAng + 360 as libc::c_int / 2 as libc::c_int) %
                     360 as libc::c_int) as UDWORD, range as UDWORD,
                Some(rayTerrainCallback as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                             -> BOOL));
        startH = (*psObj).z as libc::c_int + visObjHeight(psObj);
        currG = -(0xff as libc::c_int) * 10000 as libc::c_int;
        rayCast((*psObj).x as UDWORD, (*psObj).y as UDWORD,
                ((currRayAng + 360 as libc::c_int / 2 as libc::c_int +
                      360 as libc::c_int / 4 as libc::c_int) %
                     360 as libc::c_int) as UDWORD, range as UDWORD,
                Some(rayTerrainCallback as
                         unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                             -> BOOL));
        (*psDroid).currRayAng =
            ((*psDroid).currRayAng as libc::c_int +
                 360 as libc::c_int / 120 as libc::c_int) as UBYTE;
        //DBPRINTF(("%p %d\n",psDroid,psDroid->currRayAng);
        if (*psDroid).currRayAng as libc::c_int >=
               360 as libc::c_int / 4 as libc::c_int {
            (*psDroid).currRayAng = 0 as libc::c_int as UBYTE;
            (*psDroid).updateFlags =
                ((*psDroid).updateFlags as libc::c_int &
                     !(0x1 as libc::c_int)) as UBYTE
            //DBPRINTF(("%p done\n",psDroid);
        }
    } else {
        // Do the whole circle.
        ray = 0 as libc::c_int;
        while ray < 360 as libc::c_int {
            // initialise the callback variables
	/*		startH = (mapTile(psObj->x >> TILE_SHIFT,psObj->y >> TILE_SHIFT)->height)
						* ELEVATION_SCALE;*/
            startH = (*psObj).z as libc::c_int + visObjHeight(psObj);
            currG = -(0xff as libc::c_int) * 10000 as libc::c_int;
            // Cast the rays from the viewer
            rayCast((*psObj).x as UDWORD, (*psObj).y as UDWORD, ray as UDWORD,
                    range as UDWORD,
                    Some(rayTerrainCallback as
                             unsafe extern "C" fn(_: SDWORD, _: SDWORD,
                                                  _: SDWORD) -> BOOL));
            ray += 360 as libc::c_int / 80 as libc::c_int
        }
    };
}
/* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
/* Check whether psViewer can see psTarget.
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 * struckBlock controls whether structures block LOS
 */
//BOOL visibleObjectBlock(BASE_OBJECT *psViewer, BASE_OBJECT *psTarget, BOOL structBlock)
#[no_mangle]
pub unsafe extern "C" fn visibleObject(mut psViewer: *mut BASE_OBJECT,
                                       mut psTarget: *mut BASE_OBJECT)
 -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut ray: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut senPower: UDWORD = 0;
    let mut ecmPower_0: UDWORD = 0;
    //	SDWORD		x1,y1, x2,y2;
    let mut tarG: SDWORD = 0;
    let mut top: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    /* Get the sensor Range and power */
    match (*psViewer).type_0 as libc::c_uint {
        0 => {
            range = (*(psViewer as *mut DROID)).sensorRange as SDWORD;
            senPower = (*(psViewer as *mut DROID)).sensorPower;
            if (*(psViewer as *mut DROID)).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                range = 3 as libc::c_int * range / 2 as libc::c_int
            }
        }
        1 => {
            psStruct = psViewer as *mut STRUCTURE;
            // a structure that is being built cannot see anything
            if (*psStruct).status as libc::c_int != SS_BUILT as libc::c_int {
                return 0 as libc::c_int
            }
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint ||
                   (*(*psStruct).pStructureType).type_0 ==
                       REF_WALLCORNER as libc::c_int as libc::c_uint {
                return 0 as libc::c_int
            }
            if (structCBSensor(psViewer as *mut STRUCTURE) != 0 ||
                    structVTOLCBSensor(psViewer as *mut STRUCTURE) != 0) &&
                   (*(psViewer as *mut STRUCTURE)).psTarget == psTarget {
                // if a unit is targetted by a counter battery sensor
			// it is automatically seen
//			lastSensorPower = senPower;
                return 1 as libc::c_int
            }
            range = (*(psViewer as *mut STRUCTURE)).sensorRange as SDWORD;
            senPower = (*(psViewer as *mut STRUCTURE)).sensorPower as UDWORD;
            // increase the sensor range for AA sites
		// AA sites are defensive structures that can only shoot in the air
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_DEFENSE as libc::c_int as libc::c_uint &&
                   (*asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int
                                                                  as
                                                                  usize].nStat
                                              as isize)).surfaceToAir as
                       libc::c_int == 0x2 as libc::c_int {
                range = 3 as libc::c_int * range / 2 as libc::c_int
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"visibleObject: visibility checking is only implemented forunits and structures\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"visibility.c\x00" as *const u8 as *const libc::c_char,
                      503 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"visibleObject\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    /* Get the target's ecm power (if it has one)
	 * or that of a nearby ECM droid.
	 */
    match (*psTarget).type_0 as libc::c_uint {
        0 => { ecmPower_0 = (*(psTarget as *mut DROID)).ECMMod }
        1 => {
            ecmPower_0 = (*(psTarget as *mut STRUCTURE)).ecmPower as UDWORD;
            range = 4 as libc::c_int * range / 3 as libc::c_int
        }
        _ => {
            /* No ecm so zero power */
            ecmPower_0 = 0 as libc::c_int as UDWORD
        }
    }
    // fix to see units and structures of your ally in teamplay mode
	/*
	if(bMultiPlayer && game.type == TEAMPLAY && aiCheckAlliances(psViewer->player,psTarget->player))
	{
		if( (psViewer->type == OBJ_DROID) OR (psViewer->type == OBJ_STRUCTURE) )
			{
				if( (psTarget->type == OBJ_DROID) OR (psTarget->type == OBJ_STRUCTURE) )
				{
					return(TRUE);
				}
			}
	}
	*/
    /* First see if the target is in sensor range */
    x = (*psViewer).x as SDWORD;
    xdiff = x - (*psTarget).x as SDWORD;
    if xdiff < 0 as libc::c_int { xdiff = -xdiff }
    if xdiff > range {
        // too far away, reject
        return 0 as libc::c_int
    }
    y = (*psViewer).y as SDWORD;
    ydiff = y - (*psTarget).y as SDWORD;
    if ydiff < 0 as libc::c_int { ydiff = -ydiff }
    if ydiff > range {
        // too far away, reject
        return 0 as libc::c_int
    }
    rangeSquared = xdiff * xdiff + ydiff * ydiff;
    if rangeSquared > range * range {
        /* Out of sensor range */
        return 0 as libc::c_int
    }
    //	if (rangeSquared > BASE_VISIBILITY*BASE_VISIBILITY)
//	{
		/* Not automatically seen so have to check against ecm */
//		sensorPower = visCalcPower(psViewer->x,psViewer->y, psTarget->x,psTarget->y,
//									sensorPower, sensorRange);
//		lastSensorPower = senPower;
		// ecm power was already calculated in processVisiblity
/*		if (sensorPower < ecmPower)
		{
			return FALSE;
		}*/
//	}
//	else
//	{
//		lastSensorPower = MAX_SENSOR_POWER;	// NOTE this was "lastSensorPower == 0" which I assume was wrong (PD)
//	}
    if rangeSquared == 0 as libc::c_int {
        // Should never be on top of each other, but ...
        return 1 as libc::c_int
    }
    // initialise the callback variables
//	startH = mapTile(x>>TILE_SHIFT,y>>TILE_SHIFT)->height * ELEVATION_SCALE;
    startH = (*psViewer).z as SDWORD;
    startH += visObjHeight(psViewer);
    currG = -(0xff as libc::c_int) * 10000 as libc::c_int * 2 as libc::c_int;
    tarDist = rangeSquared;
    rayStart = 1 as libc::c_int;
    currObj = 0 as libc::c_int;
    ray =
        360 as libc::c_int - 1 as libc::c_int -
            calcDirection((*psViewer).x as UDWORD, (*psViewer).y as UDWORD,
                          (*psTarget).x as UDWORD, (*psTarget).y as UDWORD);
    finalX = (*psTarget).x as libc::c_int >> 7 as libc::c_int;
    finalY = (*psTarget).y as libc::c_int >> 7 as libc::c_int;
    /*	if (structBlock)
	{
		// Get the objects that might intersect the rays for this quadrant
		if (ray < NUM_RAYS/2)
		{
			x1 = x - xDiff;
			x2 = x;
		}
		else
		{
			x1 = x;
			x2 = x + xDiff;
		}
		if (ray < NUM_RAYS/4 || ray > 3*NUM_RAYS/4)
		{
			y1 = y;
			y2 = y + yDiff;
		}
		else
		{
			y1 = y - yDiff;
			y2 = y;
		}
		visGetTestObjects(x1,y1, x2,y2, psViewer, psTarget);

		// Get the objects that actually intersect the ray
		visGetRayObjects(x,y, (SDWORD)psTarget->x,(SDWORD)psTarget->y);
	}
	else*/
    // don't check for any objects intersecting the ray
    numRayObjects = 0 as libc::c_int;
    // Cast a ray from the viewer to the target
    rayCast(x as UDWORD, y as UDWORD, ray as UDWORD, range as UDWORD,
            Some(rayLOSCallback as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD, _: SDWORD)
                         -> BOOL));
    // See if the target can be seen
    top = (*psTarget).z as SDWORD + visObjHeight(psTarget) - startH;
    //	tarG = (top*top) * GRAD_MUL / rangeSquared;
//	if (top < 0)
//	{
//		tarG = - tarG;
//	}
    tarG = top * 10000 as libc::c_int / lastD;
    return (tarG >= currG) as libc::c_int;
}
// Do visibility check, but with walls completely blocking LOS.
// Do visibility check, but with walls completely blocking LOS.
#[no_mangle]
pub unsafe extern "C" fn visibleObjWallBlock(mut psViewer: *mut BASE_OBJECT,
                                             mut psTarget: *mut BASE_OBJECT)
 -> BOOL {
    let mut result: BOOL = 0;
    blockingWall = 1 as libc::c_int;
    result = visibleObject(psViewer, psTarget);
    blockingWall = 0 as libc::c_int;
    return result;
}
// Find the wall that is blocking LOS to a target (if any)
// Find the wall that is blocking LOS to a target (if any)
#[no_mangle]
pub unsafe extern "C" fn visGetBlockingWall(mut psViewer: *mut BASE_OBJECT,
                                            mut psTarget: *mut BASE_OBJECT,
                                            mut ppsWall: *mut *mut STRUCTURE)
 -> BOOL {
    let mut tileX: SDWORD = 0;
    let mut tileY: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psWall: *mut STRUCTURE = 0 as *mut STRUCTURE;
    blockingWall = 1 as libc::c_int;
    numWalls = 0 as libc::c_int;
    visibleObject(psViewer, psTarget);
    blockingWall = 0 as libc::c_int;
    // see if there was a wall in the way
    psWall = 0 as *mut STRUCTURE;
    if numWalls == 1 as libc::c_int {
        tileX = wallX >> 7 as libc::c_int;
        tileY = wallY >> 7 as libc::c_int;
        player = 0 as libc::c_int;
        's_41:
            while player < 8 as libc::c_int {
                psCurr = apsStructLists[player as usize];
                while !psCurr.is_null() {
                    if (*psCurr).x as libc::c_int >> 7 as libc::c_int == tileX
                           &&
                           (*psCurr).y as libc::c_int >> 7 as libc::c_int ==
                               tileY {
                        psWall = psCurr;
                        break 's_41 ;
                    } else { psCurr = (*psCurr).psNext }
                }
                player += 1 as libc::c_int
            }
    }
    *ppsWall = psWall;
    return (psWall != 0 as *mut libc::c_void as *mut STRUCTURE) as
               libc::c_int;
}
/* Check whether psViewer can see psTarget.
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
/*BOOL visibleObject(BASE_OBJECT *psViewer, BASE_OBJECT *psTarget)
{
	BOOL	structBlock;

	switch (psTarget->type)
	{
	case OBJ_DROID:
		structBlock = FALSE;
//		structBlock = TRUE;
		break;
	default:
		structBlock = FALSE;
		break;
	}

	return visibleObjectBlock(psViewer,psTarget, structBlock);
}*/
/*BOOL	blockingTile(UDWORD x, UDWORD y)
{
	TILE	*psTile;

	// get a pointer to the tile
	psTile = mapTile(x,y);

	// Is it anything other than grass or sand?
	if (psTile->type != TER_GRASS AND psTile->type!=TER_SAND)
		return(TRUE);
	else
		return(FALSE);
}*/
//#ifndef PSX
//
//void processVisibility(BASE_OBJECT *psObj)
//{
//	processVis(psObj);
//}
//
//#else
//
//
//void processVisibility(BASE_OBJECT *psObj)
//{
//	static BASE_OBJECT *psTmpObj;
//
//	// Stack in the DCache.
//	psTmpObj = psObj;
//	SetSpDCache();
//	processVis(psTmpObj);
//	SetSpNormal();
//}
//
//#endif
//
//
/* Find out what can see this object */
#[no_mangle]
pub unsafe extern "C" fn processVisibility(mut psObj: *mut BASE_OBJECT) {
    //	DROID		*psCount;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut i: UDWORD = 0;
    let mut maxPower: UDWORD = 0;
    let mut ecmPoints: UDWORD = 0;
    //	UDWORD		currPower;
    let mut psECMStats: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut prevVis: [BOOL; 8] = [0; 8];
    let mut currVis: [BOOL; 8] = [0; 8];
    //	SDWORD		maxSensor[MAX_PLAYERS];
    let mut visLevel: SDWORD = 0;
    //	SDWORD		powerRatio;
    let mut psViewer: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    //	BOOL		changed;
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut player: UDWORD = 0;
    let mut ally: UDWORD = 0;
    // calculate the ecm power for the object based on other ECM's in the area
    maxPower = 0 as libc::c_int as UDWORD;
    // set the current ecm power
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            psDroid = psObj as *mut DROID;
            psECMStats =
                asECMStats.offset((*psDroid).asBits[COMP_ECM as libc::c_int as
                                                        usize].nStat as
                                      libc::c_int as isize);
            ecmPoints = ecmPower(psECMStats, (*psDroid).player);
            //if (psECMStats->power < maxPower)
            if ecmPoints < maxPower {
                (*psDroid).ECMMod = maxPower
            } else {
                //psDroid->ECMMod = psECMStats->power;
                (*psDroid).ECMMod = ecmPoints;
                maxPower = (*psDroid).ECMMod
            }
        }
        1 => {
            psBuilding = psObj as *mut STRUCTURE;
            psECMStats = (*(*psBuilding).pStructureType).pECM;
            if !psECMStats.is_null() && (*psECMStats).power > maxPower {
                (*psBuilding).ecmPower = (*psECMStats).power as UWORD
            } else {
                (*psBuilding).ecmPower = maxPower as UWORD;
                maxPower = (*psBuilding).ecmPower as UDWORD
            }
        }
        2 | _ => { }
    }
    // initialise the visibility array
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        prevVis[i as usize] =
            ((*psObj).visible[i as usize] as libc::c_int != 0 as libc::c_int)
                as libc::c_int;
        i = i.wrapping_add(1)
    }
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        memset(currVis.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<BOOL>() as
                    libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                    libc::c_uint));
        // one can trivially see oneself
        currVis[(*psObj).player as usize] = 1 as libc::c_int
    } else {
        memcpy(currVis.as_mut_ptr() as *mut libc::c_void,
               prevVis.as_mut_ptr() as *const libc::c_void,
               (::std::mem::size_of::<BOOL>() as
                    libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                    libc::c_uint));
    }
    // get all the objects from the grid the droid is in
    gridStartIterate((*psObj).x as SDWORD, (*psObj).y as SDWORD);
    // Fix for ally vis
    if bMultiPlayer != 0 && game.type_0 as libc::c_int == 13 as libc::c_int {
        player = 0 as libc::c_int as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            if player != (*psObj).player as libc::c_uint {
                if aiCheckAlliances(player, (*psObj).player as UDWORD) != 0 {
                    currVis[player as usize] = 1 as libc::c_int
                }
            }
            player = player.wrapping_add(1)
        }
    }
    //if a player has a SAT_UPLINK structure, they can see everything!
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        if getSatUplinkExists(player) != 0 {
            currVis[player as usize] = 1 as libc::c_int;
            if (*psObj).visible[player as usize] as libc::c_int ==
                   0 as libc::c_int {
                (*psObj).visible[player as usize] = 1 as libc::c_int as UBYTE
            }
        }
        player = player.wrapping_add(1)
    }
    psViewer = gridIterate();
    while !psViewer.is_null() {
        // If we've got ranged line of sight...
        if (*psViewer).type_0 as libc::c_uint !=
               OBJ_FEATURE as libc::c_int as libc::c_uint &&
               currVis[(*psViewer).player as usize] == 0 &&
               visibleObject(psViewer, psObj) != 0 {
            // Tell system that this side can see this object
            currVis[(*psViewer).player as usize] = 1 as libc::c_int;
            if prevVis[(*psViewer).player as usize] == 0 {
                if (*psObj).visible[(*psViewer).player as usize] as
                       libc::c_int == 0 as libc::c_int {
                    (*psObj).visible[(*psViewer).player as usize] =
                        1 as libc::c_int as UBYTE
                }
                clustObjectSeen(psObj, psViewer);
            }
        }
        psViewer = gridIterate()
    }
    // jiggle the visibility for team play
    if bMultiPlayer != 0 && game.type_0 as libc::c_int == 13 as libc::c_int {
        player = 0 as libc::c_int as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            ally = 0 as libc::c_int as UDWORD;
            while ally < 8 as libc::c_int as libc::c_uint {
                if currVis[player as usize] != 0 &&
                       aiCheckAlliances(player, ally) != 0 {
                    currVis[ally as usize] = 1 as libc::c_int
                }
                ally = ally.wrapping_add(1)
            }
            player = player.wrapping_add(1)
        }
    }
    // update the visibility levels
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if i == (*psObj).player as libc::c_uint {
            (*psObj).visible[i as usize] = 0xff as libc::c_int as UBYTE
        } else {
            visLevel = 0 as libc::c_int;
            if currVis[i as usize] != 0 { visLevel = 0xff as libc::c_int }
            if visLevel < (*psObj).visible[i as usize] as libc::c_int &&
                   (*psObj).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                if (*psObj).visible[i as usize] as libc::c_int <= visLevelDec
                   {
                    (*psObj).visible[i as usize] = 0 as libc::c_int as UBYTE
                } else {
                    (*psObj).visible[i as usize] =
                        ((*psObj).visible[i as usize] as libc::c_int -
                             visLevelDec) as UBYTE
                }
            } else if visLevel > (*psObj).visible[i as usize] as libc::c_int {
                if (*psObj).visible[i as usize] as libc::c_int + visLevelInc
                       >= 0xff as libc::c_int {
                    (*psObj).visible[i as usize] =
                        0xff as libc::c_int as UBYTE
                } else {
                    (*psObj).visible[i as usize] =
                        ((*psObj).visible[i as usize] as libc::c_int +
                             visLevelInc) as UBYTE
                }
            }
        }
        i = i.wrapping_add(1)
    }
    // if a structure has just become visible set the tile flags
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           prevVis[selectedPlayer as usize] == 0 &&
           (*psObj).visible[selectedPlayer as usize] as libc::c_int != 0 {
        setStructTileDraw(psObj as *mut STRUCTURE);
    }
    /* Make sure all tiles under a feature/structure become visible when you see it */
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if ((*psObj).type_0 as libc::c_uint ==
                OBJ_STRUCTURE as libc::c_int as libc::c_uint ||
                (*psObj).type_0 as libc::c_uint ==
                    OBJ_FEATURE as libc::c_int as libc::c_uint) &&
               (prevVis[i as usize] == 0 &&
                    (*psObj).visible[i as usize] as libc::c_int != 0) {
            setUnderTilesVis(psObj, i);
        }
        i = i.wrapping_add(1)
    }
    // if a feature has just become visible set the tile flags
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint &&
           prevVis[selectedPlayer as usize] == 0 &&
           (*psObj).visible[selectedPlayer as usize] as libc::c_int != 0 {
        setFeatTileDraw(psObj as *mut FEATURE);
        /*if this is an oil resource we want to add a proximity message for
		the selected Player - if there isn't an Resource Extractor on it*/
        if (*(*(psObj as *mut FEATURE)).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            if (*mapTile(((*psObj).x as libc::c_int >> 7 as libc::c_int) as
                             UDWORD,
                         ((*psObj).y as libc::c_int >> 7 as libc::c_int) as
                             UDWORD)).tileInfoBits as libc::c_int &
                   0x1 as libc::c_int == 0 {
                psMessage =
                    addMessage(MSG_PROXIMITY as libc::c_int as UDWORD,
                               1 as libc::c_int, selectedPlayer);
                if !psMessage.is_null() {
                    (*psMessage).pViewData = psObj as *mut MSG_VIEWDATA
                }
                if bInTutorial == 0 {
                    //play message to indicate been seen
                    audio_QueueTrackPos(ID_SOUND_RESOURCE_HERE as libc::c_int,
                                        (*psObj).x as SDWORD,
                                        (*psObj).y as SDWORD,
                                        (*psObj).z as SDWORD);
                }
            }
        }
        if (*(*(psObj as *mut FEATURE)).psStats).subType as libc::c_uint ==
               FEAT_GEN_ARTE as libc::c_int as libc::c_uint {
            psMessage =
                addMessage(MSG_PROXIMITY as libc::c_int as UDWORD,
                           1 as libc::c_int, selectedPlayer);
            if !psMessage.is_null() {
                (*psMessage).pViewData = psObj as *mut MSG_VIEWDATA
            }
            if bInTutorial == 0 {
                //play message to indicate been seen
                audio_QueueTrackPos(ID_SOUND_ARTEFACT_DISC as libc::c_int,
                                    (*psObj).x as SDWORD,
                                    (*psObj).y as SDWORD,
                                    (*psObj).z as SDWORD);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn setUnderTilesVis(mut psObj: *mut BASE_OBJECT,
                                          mut player: UDWORD) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStats: *mut FEATURE_STATS = 0 as *mut FEATURE_STATS;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_FEATURE as libc::c_int as libc::c_uint {
        psFeature = psObj as *mut FEATURE;
        psStats = (*psFeature).psStats;
        width = (*psStats).baseWidth as UDWORD;
        breadth = (*psStats).baseBreadth as UDWORD;
        mapX =
            ((*psFeature).x as
                 libc::c_uint).wrapping_sub(width.wrapping_mul(128 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_div(2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint))
                >> 7 as libc::c_int;
        mapY =
            ((*psFeature).y as
                 libc::c_uint).wrapping_sub(breadth.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_div(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint))
                >> 7 as libc::c_int
    } else {
        /* Must be a structure */
        psStructure = psObj as *mut STRUCTURE;
        width = (*(*psStructure).pStructureType).baseWidth;
        breadth = (*(*psStructure).pStructureType).baseBreadth;
        mapX =
            ((*psStructure).x as
                 libc::c_uint).wrapping_sub(width.wrapping_mul(128 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint).wrapping_div(2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint))
                >> 7 as libc::c_int;
        mapY =
            ((*psStructure).y as
                 libc::c_uint).wrapping_sub(breadth.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_div(2
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint))
                >> 7 as libc::c_int
    }
    i = 0 as libc::c_int as UDWORD;
    while i < width {
        j = 0 as libc::c_int as UDWORD;
        while j < breadth {
            /* Slow fade up */
            if getRevealStatus() != 0 {
                if player == selectedPlayer {
                    avInformOfChange(mapX.wrapping_add(i) as SDWORD,
                                     mapY.wrapping_add(j) as SDWORD);
                }
            }
            psTile = mapTile(mapX.wrapping_add(i), mapY.wrapping_add(j));
            (*psTile).tileVisBits =
                ((*psTile).tileVisBits as libc::c_int |
                     (1 as libc::c_int) << player) as UBYTE;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// percentage of power over which objects start to be visible
// maximum possible sensor power
// the last sensor power to see an object - used by process visibility
// to avoid having to recalculate it
//static SDWORD			lastSensorPower;
// whether a side has seen a unit on another side yet
/* Get all the objects that might intersect a ray.
 * Do not include psSource in the list as the ray comes from it
 */
/* Get all the objects that might intersect a ray.
 * Do not include psSource in the list as the ray comes from it
 */
#[no_mangle]
pub unsafe extern "C" fn visGetTestObjects(mut x1: SDWORD, mut y1: SDWORD,
                                           mut x2: SDWORD, mut y2: SDWORD,
                                           mut psSource: *mut BASE_OBJECT,
                                           mut psTarget: *mut BASE_OBJECT) {
    let mut player: SDWORD = 0; //, bx1,by1, bx2,by2;
    let mut radius: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    // find the structures
    numTestObjects = 0 as libc::c_int;
    player = 0 as libc::c_int;
    while player < 8 as libc::c_int {
        psObj = apsStructLists[player as usize] as *mut BASE_OBJECT;
        while !psObj.is_null() {
            radius = visObjRadius(psObj);
            if (*psObj).x as SDWORD + radius >= x1 &&
                   (*psObj).x as SDWORD - radius <= x2 &&
                   (*psObj).y as SDWORD + radius >= y1 &&
                   (*psObj).y as SDWORD - radius <= y2 && psObj != psSource &&
                   psObj != psTarget && numTestObjects < 100 as libc::c_int {
                let fresh0 = numTestObjects;
                numTestObjects = numTestObjects + 1;
                apsTestObjects[fresh0 as usize] = psObj
            }
            psObj = (*psObj).psNext
        }
        player += 1
    }
    // find the features
    psObj = apsFeatureLists[0 as libc::c_int as usize] as *mut BASE_OBJECT;
    while !psObj.is_null() {
        radius = visObjRadius(psObj);
        if (*psObj).x as SDWORD + radius >= x1 &&
               (*psObj).x as SDWORD - radius <= x2 &&
               (*psObj).y as SDWORD + radius >= y1 &&
               (*psObj).y as SDWORD - radius <= y2 && psObj != psSource &&
               psObj != psTarget && numTestObjects < 100 as libc::c_int {
            let fresh1 = numTestObjects;
            numTestObjects = numTestObjects + 1;
            apsTestObjects[fresh1 as usize] = psObj
        }
        psObj = (*psObj).psNext
    };
}
/* Get the objects in the apsTestObjects array that intersect a ray */
/* Get the objects in the apsTestObjects array that intersect a ray */
#[no_mangle]
pub unsafe extern "C" fn visGetRayObjects(mut x1: SDWORD, mut y1: SDWORD,
                                          mut x2: SDWORD, mut y2: SDWORD) {
    let mut apsObjs: [*mut BASE_OBJECT; 20] = [0 as *mut BASE_OBJECT; 20];
    let mut aObjDist: [SDWORD; 20] = [0; 20];
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut furthest: SDWORD = 0 as libc::c_int;
    // find the objects that intersect the ray
    numRayObjects = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numTestObjects {
        psObj = apsTestObjects[i as usize];
        x = (*psObj).x as SDWORD;
        y = (*psObj).y as SDWORD;
        dist = rayPointDist(x1, y1, x2, y2, x, y);
        if dist < visObjRadius(psObj) && numRayObjects < 20 as libc::c_int {
            // object intersects ray, calc squared distance
            xdiff = x - x1;
            ydiff = y - y1;
            aObjDist[numRayObjects as usize] =
                xdiff * xdiff + ydiff * ydiff - dist * dist;
            apsObjs[numRayObjects as usize] = psObj;
            numRayObjects += 1 as libc::c_int
        }
        i += 1
    }
    // reorder the objects on distance
    i = 0 as libc::c_int;
    while i < numRayObjects {
        dist = 0x7fffffff as libc::c_int;
        furthest = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < numRayObjects {
            if aObjDist[j as usize] < dist {
                furthest = j;
                dist = aObjDist[j as usize]
            }
            j += 1
        }
        if furthest != -(1 as libc::c_int) {
        } else {
            debug(LOG_ERROR,
                  b"visGetRayObjects: reordering failed\x00" as *const u8 as
                      *const libc::c_char);
        };
        if furthest != -(1 as libc::c_int) {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"visibility.c\x00" as *const u8 as *const libc::c_char,
                  1156 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"visGetRayObjects\x00")).as_ptr(),
                  b"furthest != -1\x00" as *const u8 as *const libc::c_char);
        };
        apsRayObjects[i as usize] = apsObjs[furthest as usize];
        aRayObjDist[i as usize] = aObjDist[furthest as usize];
        aObjDist[furthest as usize] = 0x7fffffff as libc::c_int;
        i += 1
    };
}
// calculate the power at a given distance from a sensor/ecm
/*UDWORD visCalcPower(UDWORD x1,UDWORD y1, UDWORD x2,UDWORD y2, UDWORD power, UDWORD range)
{
	SDWORD	xdiff,ydiff;
	SDWORD	distSq,rangeSq, powerBoost;
	UDWORD	finalPower;
//	SDWORD	dist, absx,absy;

	xdiff = (SDWORD)x1 - (SDWORD)x2;
	ydiff = (SDWORD)y1 - (SDWORD)y2;
	distSq = xdiff*xdiff + ydiff*ydiff;
	rangeSq = (SDWORD)(range*range);
//	absx = abs(xdiff);
//	absy = abs(ydiff);
//	dist = absx > absy ? absx + absy/2 : absx/2 + absy;

//	if (dist >= range)
//	{
//		finalPower = 0;
//	}
//	else
//	{
//		finalPower = power - power * dist / range;
//	}

	if (distSq > rangeSq)
	{
		finalPower = 0;
	}
	else
	{
		// increase the power -> will be bigger than power for some of range
//		powerBoost = 3 * power / 2;
		powerBoost = power;
		finalPower = (UDWORD)(powerBoost - powerBoost * distSq / rangeSq);
		// bring the power lower than max power
		if (finalPower > power)
		{
			finalPower = power;
		}
	}

	return finalPower;
}*/
// //////////////////////////////////////////////////////////////////
// alexl's sensor range.
#[no_mangle]
pub static mut bDisplaySensorRange: BOOL = 0;
#[no_mangle]
pub unsafe extern "C" fn startSensorDisplay() {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut x: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //	SDWORD		range;
//	SDWORD		ray;
    // clear each sensor bit.
    psTile = psMapTiles;
    x = 0 as libc::c_int as UDWORD;
    while x < mapWidth.wrapping_mul(mapHeight) as SDWORD as libc::c_uint {
        (*psTile).inRange = 0 as libc::c_int as UBYTE;
        psTile = psTile.offset(1 as libc::c_int as isize);
        x =
            (x as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    // process the sensor range of all droids/structs.
    // units.
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        //		range = psDroid->sensorRange;
//		for(ray=0; ray < NUM_RAYS; ray += NUM_RAYS/80)
//		{
//			startH = psDroid->z + visObjHeight((BASE_OBJECT*)psDroid);// initialise the callback variables //rayTerrainCallback
//			currG = -UBYTE_MAX * GRAD_MUL;	// Cast the rays from the viewer
        visTilesUpdate(psDroid as *mut BASE_OBJECT, 0 as libc::c_int);
        psDroid = (*psDroid).psNext
        //			rayCast(psDroid->x,psDroid->y,ray, range, rayTerrainCallback);
//		}
    }
    // structs.
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 !=
               REF_WALL as libc::c_int as libc::c_uint &&
               (*(*psStruct).pStructureType).type_0 !=
                   REF_WALLCORNER as libc::c_int as libc::c_uint {
            visTilesUpdate(psStruct as *mut BASE_OBJECT, 0 as libc::c_int);
        }
        psStruct = (*psStruct).psNext
    }
    // set the display flag for the tiledraw er.
    bDisplaySensorRange = 1 as libc::c_int;
}
// sensorrangedisplay.
#[no_mangle]
pub unsafe extern "C" fn stopSensorDisplay() {
    // set the display flag off.
    bDisplaySensorRange = 0 as libc::c_int;
}
