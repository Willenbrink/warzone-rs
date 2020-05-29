use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
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
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn structPowerToBuild(psStruct: *mut STRUCTURE) -> UDWORD;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    // This dos'nt compile on the PSX.
//typedef enum _titlemode tMode;	// define the type
    #[no_mangle]
    static mut titleMode: tMode;
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    // returns true when no droid on x,y square.
    // true if no droid at x,y
    // returns true if it's a sensible place to put that droid.
    // true if x,y is an ok place
    // returns an x/y coord to place a droid
    //initialises the droid movement model
    /* Looks through the players list of droids to see if any of them are 
building the specified structure - returns TRUE if finds one*/
    /* Looks through the players list of droids to see if any of them are 
demolishing the specified structure - returns TRUE if finds one*/
    /* checks the structure for type and capacity and orders the droid to build
a module if it can - returns TRUE if order is set */
    /*Deals with building a module - checking if any droid is currently doing this
 - if so, helping to build the current one*/
    /*return the name to display for the interface given a DROID structure*/
    /*return the name to display for the interface - we don't know if this is 
a string ID or something the user types in*/
    /* Just returns true if the droid's present body points aren't as high as the original*/
    /* Returns currently active (selected) group */
    /*checks to see if an electronic warfare weapon is attached to the droid*/
    /*checks to see if the droid is currently being repaired by another*/
    //count how many Command Droids exist in the world at any one moment
    /* Set up a droid to clear a wrecked building feature - returns true if successful */
    /* Update a construction droid while it is clearing 
   returns TRUE while continues */
    /*For a given repair droid, check if there are any damaged droids within 
a defined range*/
    //access function
    /*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
    /*Checks a vtol for being fully armed and fully repaired to see if ready to 
leave reArm pad */
    /*this mends the VTOL when it has been returned to home base whilst on an
offworld mission*/
    /*checks if the droid is a VTOL droid and updates the attack runs as required*/
    /*returns a count of the base number of attack runs for the weapon attached to the droid*/
    //assign rearmPad to the VTOL
    //don't use this function any more - the droid checks each frame for this to have died
//look through all droids to see if any are associated with the ReArming Pad
//extern void releaseVTOLPad(STRUCTURE *psReArmPad);
// true if a vtol is waiting to be rearmed by a particular rearm pad
    // true if a vtol droid currently returning to be rearmed
    // true if a droid is currently attacking
    // see if there are any other vtols attacking the same target
// but still rearming
    /*compares the droid sensor type with the droid weapon type to see if the 
FIRE_SUPPORT order can be assigned*/
    // return whether a droid has a CB sensor on it
    // give a droid from one player to another - used in Electronic Warfare and multiplayer
    /*calculates the electronic resistance of a droid based on its experience level*/
    /*this is called to check the weapon is 'allowed'. Check if VTOL, the weapon is 
direct fire. Also check numVTOLattackRuns for the weapon is not zero - return 
TRUE if valid weapon*/
    /*called when a Template is deleted in the Design screen*/
    // Select a droid and do any necessary housekeeping.
    // De-select a droid and do any necessary housekeeping.
    /*calculate the power cost to repair a droid*/
    #[no_mangle]
    fn powerReqForDroidRepair(psDroid: *mut DROID) -> UWORD;
    #[no_mangle]
    static mut offWorldKeepLists: BOOL;
    #[no_mangle]
    static mut mission: MISSION;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    #[no_mangle]
    fn DroidIsBuilding(Droid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn DroidIsRepairing(Droid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn getDifficultyLevel() -> DIFFICULTY_LEVEL;
    /*
 * PowerCrypt.h
 *
 * Set up a seperate encrypted copy of each players power.
 *
 */
    // set the current power value
    #[no_mangle]
    fn pwrcSetPlayerCryptPower(player: UDWORD, power: UDWORD);
    // check the current power value
    #[no_mangle]
    fn pwrcCheckPlayerCryptPower(player: UDWORD, power: UDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
// win32 functions to POSIX
// WIN32
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
pub struct _resource_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub maxPower: UDWORD,
}
pub type RESOURCE_FUNCTION = _resource_function;
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
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/*
 * messageDef.h
 *
 * Message structure definitions
 */
// Research message
// Campaign message
// Mission Report messages
// Proximity message
pub type _view_type = libc::c_uint;
// full screen view sequence - flic.	extended format
pub const VIEW_TYPES: _view_type = 4;
// proximity view - no view really!
pub const VIEW_RPLX: _view_type = 3;
// full screen view sequence - flic
pub const VIEW_PROX: _view_type = 2;
// research view
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
pub type RESEARCH = research_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
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
pub type _struct_states = libc::c_uint;
pub const SS_BEING_DEMOLISHED: _struct_states = 2;
pub const SS_BUILT: _struct_states = 1;
pub const SS_BEING_BUILT: _struct_states = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_facility {
    pub psSubject: *mut _base_stats,
    pub capacity: UDWORD,
    pub timeStarted: UDWORD,
    pub researchPoints: UDWORD,
    pub timeToResearch: UDWORD,
    pub psBestTopic: *mut _base_stats,
    pub powerAccrued: UDWORD,
    pub timeStartHold: UDWORD,
}
pub type RESEARCH_FACILITY = _research_facility;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _factory {
    pub capacity: UBYTE,
    pub quantity: UBYTE,
    pub loopsPerformed: UBYTE,
    pub productionOutput: UBYTE,
    pub powerAccrued: UDWORD,
    pub psSubject: *mut BASE_STATS,
    pub timeStarted: UDWORD,
    pub timeToBuild: UDWORD,
    pub timeStartHold: UDWORD,
    pub psAssemblyPoint: *mut FLAG_POSITION,
    pub psFormation: *mut _formation,
    pub psCommander: *mut _droid,
    pub secondaryOrder: UDWORD,
}
pub type FACTORY = _factory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _res_extractor {
    pub power: UDWORD,
    pub timeLastUpdated: UDWORD,
    pub active: BOOL,
    pub psPowerGen: *mut _structure,
}
pub type RES_EXTRACTOR = _res_extractor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen {
    pub power: UDWORD,
    pub multiplier: UDWORD,
    pub capacity: UDWORD,
    pub apResExtractors: [*mut _structure; 4],
}
pub type POWER_GEN = _power_gen;
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
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*owning power generator*/
/*pointers to the res ext
																associated with this gen*/
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
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
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
pub const FORCESELECT: _title_mode = 9;
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
pub const GAMEFIND: _title_mode = 10;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
pub const DL_HARD: _difficulty_level = 2;
pub type DIFFICULTY_LEVEL = _difficulty_level;
pub type _difficulty_level = libc::c_uint;
pub const DL_KILLER: _difficulty_level = 4;
pub const DL_TOUGH: _difficulty_level = 3;
pub const DL_NORMAL: _difficulty_level = 1;
pub const DL_EASY: _difficulty_level = 0;
pub type MISSION = _mission;
// 0 intro mode
// 1 single player menu
// 2 multiplayer menu
// 3 options menu	
// 4
// 5  tutorial/fastplay	
// 6  credits
// 7  MULTIPLAYER, select proto
// 8 MULTIPLAYER, select game options
// 9 MULTIPLAYER, Force design screen
// 10 MULTIPLAYER, gamefinder.
// 11 MULTIPLAYER, Limit the multistuff.
// 12 Fire up the game
// 13 reshow the intro
// 14 leaving game
// 15 loading a save game
// 16 keymap editor
// 17 second options menu.
// 18 third options menu.
//		GRAPHICS,					// 5
//		VIDEO,
//	DEMOMODE,					// demo mode. remove for release?
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
pub type TILE_COORD = _tile_coord;
/* The maximum map size */
/* The size and contents of the map */
/* The shift on the y coord when calculating into the map */
/* The number of units accross a tile */
/* The shift on a coordinate to get the tile coordinate */
/* The mask to get internal tile coords from a full coordinate */
/* Shutdown the map module */
/* Create a new map of a specified size */
/* Load the map data */
/* Save the map data */
/* Load map texture info */
/* Save the current map texture info */
/* A post process for the water tiles in map to ensure height integrity */
/* Return a pointer to the tile structure at x,y */
//return psMapTiles + x + (y << mapShift); //width no longer a power of 2
/* Return height of tile at x,y */
//static inline SDWORD map_TileHeight(UDWORD x, UDWORD y)
/*sets the tile height */
//psMapTiles[x + (y << mapShift)].height = height;//width no longer a power of 2
/*increases the tile height by one */
/*static inline void incTileHeight(UDWORD x, UDWORD y)
{
	psMapTiles[x + (y << mapShift)].height++;
}*/
/*decreases the tile height by one */
/*static inline void decTileHeight(UDWORD x, UDWORD y)
{
	psMapTiles[x + (y << mapShift)].height--;
}*/
/* Return whether a tile coordinate is on the map */
/* Return whether a world coordinate is on the map */
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
pub type MAPTILE = _maptile;
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
 * Map.h
 *
 * Definitions for the map structure
 *
 */
// Visibility bits - can also be accessed as a byte (as a whole).
/* The different types of terrain as far as the game is concerned */
/* change these if you change above - maybe wrap up in enumerate? */
/* Flags for whether texture tiles are flipped in X and Y or rotated */
// This bit describes the direction the tile is split into 2 triangles (same as triangleFlip)
// set when the tile has the structure cursor over it
// NASTY - this should be in tileInfoBits but there isn't any room left
// units can drive on this even if there is a structure or feature on it
//#define	BITS_TILE_HIGHLIGHT 0x8
// show small structures - tank traps / bunkers
// bit set temporarily by find path to mark a blocking tile
// bit set to show a gateway on the tile
// bit set to show a tall structure which camera needs to avoid.
/*#ifndef PSX	// Extra tile info bits.... WIN32 only
#define	EXTRA_BITS_SENSOR	0x1
#define	EXTRA_BITS_2		0x2
#define	EXTRA_BITS_3		0x4
#define	EXTRA_BITS_4		0x8
#define	EXTRA_BITS_5		0x10
#define	EXTRA_BITS_6		0x20
#define	EXTRA_BITS_7		0x40
#define	EXTRA_BITS_8		0x80
#endif*/
//#define TILE_HIGHLIGHT(x)		(x->tileInfoBits & BITS_TILE_HIGHLIGHT)
/*
#ifndef PSX		// I've even set them up for you...:-)
#define TILE_IN_SENSORRANGE(x)	(x->tileExtraBits & EXTRA_BITS_SENSOR)
#define TILE_EXTRA_BIT2_SET(x)	(x->tileExtraBits & EXTRA_BITS_2)
#define TILE_EXTRA_BIT3_SET(x)	(x->tileExtraBits & EXTRA_BITS_3)
#define TILE_EXTRA_BIT4_SET(x)	(x->tileExtraBits & EXTRA_BITS_4)
#define TILE_EXTRA_BIT5_SET(x)	(x->tileExtraBits & EXTRA_BITS_5)
#define TILE_EXTRA_BIT6_SET(x)	(x->tileExtraBits & EXTRA_BITS_6)
#define TILE_EXTRA_BIT7_SET(x)	(x->tileExtraBits & EXTRA_BITS_7)
#define TILE_EXTRA_BIT8_SET(x)	(x->tileExtraBits & EXTRA_BITS_8)
#endif
*/
/*
#ifndef PSX	// again, done for you again!
#define SET_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_SENSOR))
#define CLEAR_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_SENSOR)))
#define SET_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_2))
#define CLEAR_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_2)))
#define SET_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_3))
#define CLEAR_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_3)))
#define SET_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_4))
#define CLEAR_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_4)))
#define SET_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_5))
#define CLEAR_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_5)))
#define SET_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_6))
#define CLEAR_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_6)))
#define SET_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_7))
#define CLEAR_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_7)))
#define SET_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_8))
#define CLEAR_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_8)))
#endif
*/
// Multiplier for the tile height
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
pub const DACTION_DROIDREPAIR: _droid_action = 14;
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
//arbitary high value - needs to allow all structures to be built at start of any game
//flag used to check for power calculations to be done or not
#[no_mangle]
pub static mut powerCalculated: BOOL = 0;
//returns the relevant list based on OffWorld or OnWorld for the accruePower function
//static STRUCTURE* powerUpdateStructList(UBYTE player);
#[no_mangle]
pub static mut asPower: [*mut PLAYER_POWER; 8] =
    [0 as *const PLAYER_POWER as *mut PLAYER_POWER; 8];
/*allocate the space for the playerPower*/
/*allocate the space for the playerPower*/
#[no_mangle]
pub unsafe extern "C" fn allocPlayerPower() -> BOOL {
    let mut player: UDWORD = 0;
    //allocate the space for the structure
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        asPower[player as usize] =
            memMallocRelease(::std::mem::size_of::<PLAYER_POWER>() as
                                 libc::c_ulong) as *mut PLAYER_POWER;
        if asPower[player as usize].is_null() {
            debug(LOG_ERROR,
                  b"Out of memory\x00" as *const u8 as *const libc::c_char);
            abort();
        }
        player = player.wrapping_add(1)
    }
    clearPlayerPower();
    powerCalculated = 1 as libc::c_int;
    return 1 as libc::c_int;
}
/*clear the playerPower */
/*clear the playerPower */
#[no_mangle]
pub unsafe extern "C" fn clearPlayerPower() {
    let mut player: UDWORD = 0;
    //check power has been allocated!
    if asPower[0 as libc::c_int as usize].is_null() { return }
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        memset(asPower[player as usize] as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<PLAYER_POWER>() as libc::c_ulong);
        pwrcSetPlayerCryptPower(player, 0 as libc::c_int as UDWORD);
        player = player.wrapping_add(1)
    };
}
/*Free the space used for playerPower */
/*Free the space used for playerPower */
#[no_mangle]
pub unsafe extern "C" fn releasePlayerPower() {
    let mut player: UDWORD = 0;
    //check power has been allocated!
    if asPower[0 as libc::c_int as usize].is_null() { return }
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        if !asPower[player as usize].is_null() {
            memFreeRelease(asPower[player as usize] as *mut libc::c_void);
            asPower[player as usize] = 0 as *mut PLAYER_POWER
        }
        player = player.wrapping_add(1)
    };
}
/*check the available power*/
/*check the current power - if enough return true, else return false */
#[no_mangle]
pub unsafe extern "C" fn checkPower(mut player: UDWORD, mut quantity: UDWORD,
                                    mut playAudio: BOOL) -> BOOL {
    //if not doing a check on the power - just return TRUE
    if powerCalculated == 0 { return 1 as libc::c_int }
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return 0 as libc::c_int
    }
    if (*asPower[player as usize]).currentPower >= quantity {
        return 1 as libc::c_int
    }
    //Not playing the power low message anymore - 6/1/99
	/*else if (player == selectedPlayer)
	{
//#ifdef PSX
//#warning POWER LOW IS DISABLED
//		return TRUE;
//#endif
		if (playAudio && player == selectedPlayer)
		{
			audio_QueueTrack( ID_SOUND_POWER_LOW );
			return FALSE;
		}
	}*/
    return 0 as libc::c_int;
}
/*subtract the power required */
/*check the current power - if enough subtracts the amount
 required to perform the task and returns true, else returns
 false */
#[no_mangle]
pub unsafe extern "C" fn usePower(mut player: UDWORD, mut quantity: UDWORD)
 -> BOOL {
    //if not doing a check on the power - just return TRUE
    if powerCalculated == 0 { return 1 as libc::c_int }
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return 0 as libc::c_int
    }
    //check there is enough first
    if (*asPower[player as usize]).currentPower >= quantity {
        (*asPower[player as usize]).currentPower =
            ((*asPower[player as usize]).currentPower as
                 libc::c_uint).wrapping_sub(quantity) as UDWORD as UDWORD;
        pwrcSetPlayerCryptPower(player,
                                (*asPower[player as usize]).currentPower);
        return 1 as libc::c_int
    } else {
        if player == selectedPlayer {
            //#ifdef PSX
//#warning POWER LOW IS DISABLED
//		return TRUE;
//#endif
            if titleMode as libc::c_uint ==
                   FORCESELECT as libc::c_int as libc::c_uint {
                //|| (titleMode == DESIGNSCREEN))
                return 0 as libc::c_int
            }
            //Not playing the power low message anymore - 6/1/99
		//audio_QueueTrack( ID_SOUND_POWER_LOW );
        }
    }
    return 0 as libc::c_int;
}
//return the power when a structure/droid is deliberately destroyed
//return the power when a structure/droid is deliberately destroyed
#[no_mangle]
pub unsafe extern "C" fn addPower(mut player: UDWORD, mut quantity: UDWORD) {
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return
    }
    (*asPower[player as usize]).currentPower =
        ((*asPower[player as usize]).currentPower as
             libc::c_uint).wrapping_add(quantity) as UDWORD as UDWORD;
    pwrcSetPlayerCryptPower(player, (*asPower[player as usize]).currentPower);
}
/*resets the power levels for all players when power is turned back on*/
/*resets the power calc flag for all players*/
#[no_mangle]
pub unsafe extern "C" fn powerCalc(mut on: BOOL) {
    if on != 0 {
        powerCalculated = 1 as libc::c_int
    } else { powerCalculated = 0 as libc::c_int };
}
/* Each Resource Extractor yields EXTRACT_POINTS per second until there are none
   left in the resource. */
//extern void updateExtractedPower(STRUCTURE	*psBuilding);
/* Each Resource Extractor yields EXTRACT_POINTS per second until there are none
   left in the resource. */
#[no_mangle]
pub unsafe extern "C" fn updateExtractedPower(mut psBuilding: *mut STRUCTURE)
 -> UDWORD {
    let mut pResExtractor: *mut RES_EXTRACTOR = 0 as *mut RES_EXTRACTOR;
    let mut pointsToAdd: UDWORD = 0;
    let mut extractedPoints: UDWORD = 0;
    let mut timeDiff: UDWORD = 0;
    let mut modifier: UBYTE = 0;
    pResExtractor = (*psBuilding).pFunctionality as *mut RES_EXTRACTOR;
    extractedPoints = 0 as libc::c_int as UDWORD;
    //only extracts points whilst its active ie associated with a power gen
	//and has got some power to extract
    if (*pResExtractor).active != 0 && (*pResExtractor).power != 0 {
        timeDiff = gameTime.wrapping_sub((*pResExtractor).timeLastUpdated);
        //add modifier according to difficulty level
        if getDifficultyLevel() as libc::c_uint ==
               DL_EASY as libc::c_int as libc::c_uint {
            modifier = 110 as libc::c_int as UBYTE
        } else if getDifficultyLevel() as libc::c_uint ==
                      DL_HARD as libc::c_int as libc::c_uint {
            modifier = 90 as libc::c_int as UBYTE
        } else { modifier = 100 as libc::c_int as UBYTE }
        //include modifier as a %
        pointsToAdd =
            ((modifier as libc::c_int * 1 as libc::c_int) as
                 libc::c_uint).wrapping_mul(timeDiff).wrapping_div((1000 as
                                                                        libc::c_int
                                                                        *
                                                                        100 as
                                                                            libc::c_int)
                                                                       as
                                                                       libc::c_uint);
        if pointsToAdd != 0 {
            //lose a lot on rounding this way
            (*pResExtractor).timeLastUpdated = gameTime;
            //pResExtractor->timeLastUpdated = gameTime - (timeDiff - GAME_TICKS_PER_SEC);
            if (*pResExtractor).power > pointsToAdd {
                extractedPoints =
                    (extractedPoints as
                         libc::c_uint).wrapping_add(pointsToAdd) as UDWORD as
                        UDWORD;
                (*pResExtractor).power =
                    ((*pResExtractor).power as
                         libc::c_uint).wrapping_sub(pointsToAdd) as UDWORD as
                        UDWORD
            } else {
                extractedPoints =
                    (extractedPoints as
                         libc::c_uint).wrapping_add((*pResExtractor).power) as
                        UDWORD as UDWORD;
                (*pResExtractor).power = 0 as libc::c_int as UDWORD
            }
            if (*pResExtractor).power == 0 as libc::c_int as libc::c_uint {
                //if not having unlimited power, put the 2 lines below back in
				//set the extractor to be inactive
				//pResExtractor->active = FALSE;
				//break the link between the power gen and the res extractor
				//releaseResExtractor(psBuilding);
                //for now, when the power = 0 set it back to the max level!
                (*pResExtractor).power =
                    (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                              libc::c_int
                                                                              as
                                                                              isize)
                           as *mut RESOURCE_FUNCTION)).maxPower
            }
        }
    }
    return extractedPoints;
}
/*accrue the power in the facilities that require it*/
//static void accruePower(STRUCTURE *psStructure);
//returns the relevant list based on OffWorld or OnWorld
/* Each Resource Extractor yields EXTRACT_POINTS per second until there are none
   left in the resource. */
/*void updateExtractedPower(STRUCTURE	*psBuilding)
{
	RES_EXTRACTOR		*pResExtractor;
	UDWORD				pointsToAdd;

	pResExtractor = (RES_EXTRACTOR *) psBuilding->pFunctionality;

	//only extracts points whilst its active ie associated with a power gen
	if (pResExtractor->active)
	{
		pointsToAdd = EXTRACT_POINTS * (gameTime - pResExtractor->timeLastUpdated) /
			GAME_TICKS_PER_SEC;

		if (pointsToAdd)
		{
			pResExtractor->timeLastUpdated = gameTime;
			if (pResExtractor->power > pointsToAdd)
			{
				asPower[psBuilding->player]->extractedPower += pointsToAdd;
				pResExtractor->power -= pointsToAdd;
			}
			else
			{
				asPower[psBuilding->player]->extractedPower += pResExtractor->power;
				pResExtractor->power = 0;
			}

			//for now, when the power = 0 set it back to the max level!
			if (pResExtractor->power == 0)
			{
				pResExtractor->power = ((POWER_REG_FUNCTION*)psBuilding->pStructureType->
					asFuncList[0])->maxPower;
			}
		}
	}
}*/
//returns the relevant list based on OffWorld or OnWorld
unsafe extern "C" fn powerStructList(mut player: UBYTE) -> *mut STRUCTURE {
    if offWorldKeepLists != 0 {
        return mission.apsStructLists[player as usize]
    } else { return apsStructLists[player as usize] };
}
/* Update current power based on what was extracted during the last cycle and 
   what Power Generators exist */
//returns the relevant list based on OffWorld or OnWorld for the accruePower function
/*STRUCTURE* powerUpdateStructList(UBYTE player)
{
	static BYTE		first[MAX_PLAYERS] = {TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE};

	if (offWorldKeepLists)
	{
		if (first[player])
		{
			first[player] = FALSE;
			return (mission.apsStructLists[player]);
		}
		else
		{
			first[player] = TRUE;
			return (apsStructLists[player]);
		}
	}
	else
	{
		return (apsStructLists[player]);
	}
}*/
/* Update current power based on what Power Generators exist */
#[no_mangle]
pub unsafe extern "C" fn updatePlayerPower(mut player: UDWORD) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE; //, *psList;
    /*if (offWorldKeepLists)
	{
		psList = mission.apsStructLists[player];
	}
	else
	{
		psList = apsStructLists[player];
	}*/
    //for (psStruct = psList; psStruct != NULL; psStruct = psStruct->psNext)
    psStruct = powerStructList(player as UBYTE);
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            updateCurrentPower((*psStruct).pFunctionality as *mut POWER_GEN,
                               player);
        }
        psStruct = (*psStruct).psNext
    }
    //check that the psLastPowered hasn't died
    if !(*asPower[player as usize]).psLastPowered.is_null() &&
           (*(*asPower[player as usize]).psLastPowered).died != 0 {
        (*asPower[player as usize]).psLastPowered = 0 as *mut _base_object
    };
}
/*Update the capcity and available power if necessary */
//static void availablePowerUpdate(STRUCTURE *psBuilding);
/*looks through the player's list of droids and structures to see what power
has been used*/
//static UDWORD calcPlayerUsedPower(UDWORD player);
/* Updates the current power based on the extracted power and a Power Generator*/
/* Update current power based on what was extracted during the last cycle and
   what Power Generators exist */
/*void updatePlayerPower(UDWORD player)
{
	STRUCTURE		*psStruct;

	if (asPower[player]->extractedPower == 0)
	{
		return;
	}

	//may need to order the structures so that the Power Gen with the highest
	//multiplier is used first so that the player gets maximum power output. For now
	//all multiplier are the same

	for (psStruct = apsStructLists[player]; psStruct != NULL AND
		asPower[player]->extractedPower != 0; psStruct = psStruct->psNext)
	{
		if (psStruct->pStructureType->type == REF_POWER_GEN AND psStruct->
			status == SS_BUILT)
		{
			updateCurrentPower((POWER_GEN *)psStruct->pFunctionality, player);
		}
	}
}*/
/* Updates the current power based on the extracted power and a Power Generator*/
unsafe extern "C" fn updateCurrentPower(mut psPowerGen: *mut POWER_GEN,
                                        mut player: UDWORD) {
    let mut power: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut extractedPower: UDWORD = 0;
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return
    }
    //each power gen can cope with its associated resource extractors
    extractedPower = 0 as libc::c_int as UDWORD;
    //for (i=0; i < (NUM_POWER_MODULES + 1); i++)
	//each Power Gen can cope with 4 extractors now - 9/6/98 AB
    i = 0 as libc::c_int as UDWORD;
    while i < 4 as libc::c_int as libc::c_uint {
        if !(*psPowerGen).apResExtractors[i as usize].is_null() {
            //check not died
            if (*(*psPowerGen).apResExtractors[i as usize]).died != 0 {
                (*psPowerGen).apResExtractors[i as usize] =
                    0 as *mut _structure
            } else {
                extractedPower =
                    (extractedPower as
                         libc::c_uint).wrapping_add(updateExtractedPower((*psPowerGen).apResExtractors[i
                                                                                                           as
                                                                                                           usize]))
                        as UDWORD as UDWORD
            }
        }
        i = i.wrapping_add(1)
    }
    (*asPower[player as usize]).extractedPower =
        ((*asPower[player as usize]).extractedPower as
             libc::c_uint).wrapping_add(extractedPower) as UDWORD as UDWORD;
    power =
        (*asPower[player as
                      usize]).extractedPower.wrapping_mul((*psPowerGen).multiplier).wrapping_div(100
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint);
    if power != 0 {
        (*asPower[player as usize]).currentPower =
            ((*asPower[player as usize]).currentPower as
                 libc::c_uint).wrapping_add(power) as UDWORD as UDWORD;
        (*asPower[player as usize]).extractedPower =
            0 as libc::c_int as UDWORD;
        pwrcSetPlayerCryptPower(player,
                                (*asPower[player as usize]).currentPower);
    };
}
// used in multiplayer to force power levels.
/* Updates the current power based on the extracted power and a Power Generator*/
/*void updateCurrentPower(POWER_GEN *psPowerGen, UDWORD player)
{
	UDWORD		power;

	//each power gen can cope with a set amount of power
	power = (asPower[player]->extractedPower * psPowerGen->multiplier) / 100;
	if (power)
	{
		if (power > psPowerGen->power)
		{
			power = psPowerGen->power;
			asPower[player]->extractedPower -= ((power / psPowerGen->multiplier)/100);
		}
		else
		{
			//no power left over
			asPower[player]->extractedPower = 0;
		}
		if (power)
		{
			asPower[player]->currentPower += power;
		}
	}
}*/
// only used in multiplayer games.
#[no_mangle]
pub unsafe extern "C" fn setPower(mut player: UDWORD, mut avail: UDWORD) {
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return
    }
    (*asPower[player as usize]).currentPower = avail;
    pwrcSetPlayerCryptPower(player, (*asPower[player as usize]).currentPower);
}
/*sets the initial value for the power*/
/*sets the initial value for the power*/
#[no_mangle]
pub unsafe extern "C" fn setPlayerPower(mut power: UDWORD,
                                        mut player: UDWORD) {
    if pwrcCheckPlayerCryptPower(player,
                                 (*asPower[player as usize]).currentPower) ==
           0 {
        return
    }
    //asPower[player]->initialPower = power;
    (*asPower[player as usize]).currentPower = power;
    pwrcSetPlayerCryptPower(player, (*asPower[player as usize]).currentPower);
}
/*Temp function to give all players some power when a new game has been loaded*/
/*Temp function to give all players some power when a new game has been loaded*/
#[no_mangle]
pub unsafe extern "C" fn newGameInitPower() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //setPlayerPower(400, inc);
        //add as opposed to set
        addPower(inc, 400 as libc::c_int as UDWORD);
        inc = inc.wrapping_add(1)
    };
}
//extern void spreadPower(UBYTE player);
/*accrue the power in the facilities that require it*/
/*this keeps track of which object was the last to receive power. Power is
passed through the object lists each cycle whilst there is some*/
/*void spreadPower(UBYTE player)
{
	STRUCTURE	*psLastPowered = NULL, *psStartStruct;
	UDWORD		lastPower = asPower[player]->currentPower;
	UBYTE		first = TRUE, warning = 0;

	psStartStruct = asPower[player]->psLastPowered;
	//if (!psStartStruct OR psStartStruct->died)
	//if (!psStartStruct)
	//{
	//	psStartStruct = powerUpdateStructList(player);
	//}

	//got to have the minimum power
	while ((asPower[player]->currentPower > POWER_PER_CYCLE) OR (!powerCalculated))
	{
		//little test to see if we're looping indefinately!
		warning++;
		if (warning > 1000)
		{
			ASSERT( FALSE,
				"spreading power round more than 1000 buildings for player %d?",
				player );
			warning = 0;
		}

		//if haven't used any power and been through the whole list of
		//structures then jump out
		if (!first)
		{
			if (psStartStruct == asPower[player]->psLastPowered)
			{
				//back to the start struct
				if (lastPower == asPower[player]->currentPower)
				{
					return;
				}
				lastPower = asPower[player]->currentPower;
			}
		}
		first = FALSE;
		//determine which object is to receive the power
		//check for last in list or nothing available
		if (asPower[player]->psLastPowered == NULL)
		{
			//back to top of list
			psNextPowered = powerUpdateStructList(player);
			psStartStruct = psNextPowered;
			warning = 0;
		}
		else
		{
			//get the next in the list
			psNextPowered = asPower[player]->psLastPowered->psNext;
			//check for last in list
			if (psNextPowered == NULL)
			{
				psNextPowered = powerUpdateStructList(player);
				warning = 0;
			}
		}
		if (psNextPowered)
		{
			accruePower(psNextPowered);
			asPower[player]->psLastPowered = psNextPowered;
		}
	}
}*/
/*accrue the power in the facilities that require it - returns TRUE if use some power*/
#[no_mangle]
pub unsafe extern "C" fn accruePower(mut psObject: *mut BASE_OBJECT) -> BOOL {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psResearch: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    let mut psRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut powerDiff: SDWORD = 0;
    let mut count: UDWORD = 0;
    let mut bPowerUsed: BOOL = 0 as libc::c_int;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psTarget: *mut DROID = 0 as *mut DROID;
    match (*psObject).type_0 as libc::c_uint {
        1 => {
            psStructure = psObject as *mut STRUCTURE;
            //see if it needs power
            match (*(*psStructure).pStructureType).type_0 {
                1 | 16 | 17 => {
                    psFactory = (*psStructure).pFunctionality as *mut FACTORY;
                    //check the factory is not on hold
                    if !((*psFactory).timeStartHold != 0) {
                        //check the factory is active
                        if !(*psFactory).psSubject.is_null() {
                            //check needs power
                            powerDiff =
                                (*((*psFactory).psSubject as
                                       *mut DROID_TEMPLATE)).powerPoints.wrapping_sub((*psFactory).powerAccrued)
                                    as SDWORD;
                            //if equal then don't need power
                            if powerDiff != 0 {
                                if 5 as libc::c_int >= powerDiff {
                                    usePower((*psStructure).player as UDWORD,
                                             powerDiff as UDWORD);
                                    (*psFactory).powerAccrued =
                                        ((*psFactory).powerAccrued as
                                             libc::c_uint).wrapping_add(powerDiff
                                                                            as
                                                                            libc::c_uint)
                                            as UDWORD as UDWORD;
                                    bPowerUsed = 1 as libc::c_int
                                } else if powerDiff > 5 as libc::c_int {
                                    usePower((*psStructure).player as UDWORD,
                                             5 as libc::c_int as UDWORD);
                                    (*psFactory).powerAccrued =
                                        ((*psFactory).powerAccrued as
                                             libc::c_uint).wrapping_add(5 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                            as UDWORD as UDWORD;
                                    bPowerUsed = 1 as libc::c_int
                                }
                            }
                        }
                    }
                }
                10 => {
                    //check the structure is active
                    psResearch =
                        (*psStructure).pFunctionality as
                            *mut RESEARCH_FACILITY;
                    //check the research facility is not on hold
                    if !((*psResearch).timeStartHold != 0) {
                        if !(*psResearch).psSubject.is_null() {
                            //check the research hasn't been cancelled
                            count =
                                (*((*psResearch).psSubject as
                                       *mut RESEARCH)).ref_0.wrapping_sub(0xb0000
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint);
                            if (*asPlayerResList[selectedPlayer as
                                                     usize].offset(count as
                                                                       isize)).ResearchStatus
                                   as libc::c_int & 0x2 as libc::c_int ==
                                   0 as libc::c_int {
                                //check needs power
                                powerDiff =
                                    (*((*psResearch).psSubject as
                                           *mut RESEARCH)).researchPower.wrapping_sub((*psResearch).powerAccrued)
                                        as SDWORD;
                                //if equal then don't need power
                                if powerDiff != 0 {
                                    //use the power if appropriate
                                    if 5 as libc::c_int >= powerDiff {
                                        usePower((*psStructure).player as
                                                     UDWORD,
                                                 powerDiff as UDWORD);
                                        (*psResearch).powerAccrued =
                                            ((*psResearch).powerAccrued as
                                                 libc::c_uint).wrapping_add(powerDiff
                                                                                as
                                                                                libc::c_uint)
                                                as UDWORD as UDWORD;
                                        bPowerUsed = 1 as libc::c_int
                                    } else if powerDiff > 5 as libc::c_int {
                                        usePower((*psStructure).player as
                                                     UDWORD,
                                                 5 as libc::c_int as UDWORD);
                                        (*psResearch).powerAccrued =
                                            ((*psResearch).powerAccrued as
                                                 libc::c_uint).wrapping_add(5
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                as UDWORD as UDWORD;
                                        bPowerUsed = 1 as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }
                12 => {
                    //POWER REQUIRMENTS REMOVED - AB  22/09/98 - BACK IN - AB 07/01/99
                    psRepair =
                        (*psStructure).pFunctionality as *mut REPAIR_FACILITY;
                    psDroid = (*psRepair).psObj as *mut DROID;
                    //check the droid hasn't died in the meantime
                    if !(*psRepair).psObj.is_null() &&
                           (*(*psRepair).psObj).died != 0 {
                        (*psRepair).psObj = 0 as *mut BASE_OBJECT
                    }
                    if !(*psRepair).psObj.is_null() {
                        //check if need power
                        powerDiff =
                            powerReqForDroidRepair(psDroid) as libc::c_int -
                                (*psDroid).powerAccrued as libc::c_int;
                        //if equal then don't need power
                        if powerDiff > 0 as libc::c_int {
                            powerDiff /= 100 as libc::c_int;
                            if 5 as libc::c_int >= powerDiff {
                                usePower((*psStructure).player as UDWORD,
                                         powerDiff as UDWORD);
                                //the unit accrues the power so more than one thing can be working on it
                                (*psDroid).powerAccrued =
                                    ((*psDroid).powerAccrued as libc::c_int +
                                         powerDiff * 100 as libc::c_int) as
                                        UWORD;
                                bPowerUsed = 1 as libc::c_int
                            } else if powerDiff > 5 as libc::c_int {
                                usePower((*psStructure).player as UDWORD,
                                         5 as libc::c_int as UDWORD);
                                (*psDroid).powerAccrued =
                                    ((*psDroid).powerAccrued as libc::c_int +
                                         5 as libc::c_int *
                                             100 as libc::c_int) as UWORD;
                                bPowerUsed = 1 as libc::c_int
                            }
                        }
                    }
                }
                _ => {
                    //no need for power
                    bPowerUsed = 0 as libc::c_int
                }
            }
        }
        0 => {
            psDroid = psObject as *mut DROID;
            match (*psDroid).droidType as libc::c_uint {
                3 | 10 => {
                    //check trying to build something (and that hasn't been blown up)
                    if DroidIsBuilding(psDroid) != 0 &&
                           !(*psDroid).psTarget.is_null() &&
                           (*(*psDroid).psTarget).died == 0 {
                        //powerDiff = ((STRUCTURE *)psDroid->psTarget)->pStructureType->
                //    powerToBuild - ((STRUCTURE *)psDroid->psTarget)->
                //    currentPowerAccrued;
                        powerDiff =
                            structPowerToBuild((*psDroid).psTarget as
                                                   *mut STRUCTURE).wrapping_sub((*((*psDroid).psTarget
                                                                                       as
                                                                                       *mut STRUCTURE)).currentPowerAccrued
                                                                                    as
                                                                                    libc::c_uint)
                                as SDWORD;
                        //if equal then don't need power
                        if powerDiff != 0 {
                            if 5 as libc::c_int >= powerDiff {
                                usePower((*psDroid).player as UDWORD,
                                         powerDiff as UDWORD);
                                let ref mut fresh0 =
                                    (*((*psDroid).psTarget as
                                           *mut STRUCTURE)).currentPowerAccrued;
                                *fresh0 =
                                    (*fresh0 as libc::c_int + powerDiff) as
                                        SWORD;
                                bPowerUsed = 1 as libc::c_int
                            } else if powerDiff > 5 as libc::c_int {
                                usePower((*psDroid).player as UDWORD,
                                         5 as libc::c_int as UDWORD);
                                let ref mut fresh1 =
                                    (*((*psDroid).psTarget as
                                           *mut STRUCTURE)).currentPowerAccrued;
                                *fresh1 =
                                    (*fresh1 as libc::c_int +
                                         5 as libc::c_int) as SWORD;
                                bPowerUsed = 1 as libc::c_int
                            }
                        }
                    }
                }
                8 | 11 => {
                    //check trying to repair something
                    psTarget = 0 as *mut DROID;
                    if DroidIsRepairing(psDroid) != 0 {
                        psTarget = (*psDroid).psTarget as *mut DROID
                    } else if orderState(psDroid, DORDER_GUARD) != 0 &&
                                  (*psDroid).action ==
                                      DACTION_DROIDREPAIR as libc::c_int {
                        psTarget = (*psDroid).psActionTarget as *mut DROID
                    }
                    //might have guard order but action of repair
                    //check the droid hasn't died in the meantime
                    if !psTarget.is_null() && (*psTarget).died != 0 {
                        (*psDroid).psTarget = 0 as *mut _base_object;
                        psTarget = 0 as *mut DROID
                    }
                    if !psTarget.is_null() {
                        powerDiff =
                            powerReqForDroidRepair(psTarget) as libc::c_int -
                                (*psTarget).powerAccrued as libc::c_int;
                        //if equal then don't need power
                        if powerDiff > 0 as libc::c_int {
                            powerDiff /= 100 as libc::c_int;
                            if 5 as libc::c_int >= powerDiff {
                                usePower((*psDroid).player as UDWORD,
                                         powerDiff as UDWORD);
                                //the unit accrues the power so more than one thing can be working on it
                                (*psTarget).powerAccrued =
                                    ((*psTarget).powerAccrued as libc::c_int +
                                         powerDiff * 100 as libc::c_int) as
                                        UWORD;
                                bPowerUsed = 1 as libc::c_int
                            } else if powerDiff > 5 as libc::c_int {
                                usePower((*psDroid).player as UDWORD,
                                         5 as libc::c_int as UDWORD);
                                (*psTarget).powerAccrued =
                                    ((*psTarget).powerAccrued as libc::c_int +
                                         5 as libc::c_int *
                                             100 as libc::c_int) as UWORD;
                                bPowerUsed = 1 as libc::c_int
                            }
                        }
                    }
                }
                _ => {
                    //no need for power
                    bPowerUsed = 0 as libc::c_int
                }
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"accruePower: Invalid object type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"power.c\x00" as *const u8 as *const libc::c_char,
                      811 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"accruePower\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return bPowerUsed;
}
//informs the power array that a Object has been destroyed
//informs the power array that a object has been destroyed
#[no_mangle]
pub unsafe extern "C" fn powerDestroyObject(mut psObject: *mut BASE_OBJECT) {
    //check that this wasn't the last object that received the power
    if (*asPower[(*psObject).player as usize]).psLastPowered == psObject {
        updateLastPowered(0 as *mut BASE_OBJECT, (*psObject).player);
    };
}
/*checks if the object to be powered next - returns TRUE if power*/
/*checks if the Object to be powered next - returns TRUE if power*/
#[no_mangle]
pub unsafe extern "C" fn getLastPowered(mut psObject: *mut BASE_OBJECT)
 -> BOOL {
    if !psObject.is_null() {
    } else {
        debug(LOG_ERROR,
              b"getLastPowered - invalid object\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psObject.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"power.c\x00" as *const u8 as *const libc::c_char,
              831 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"getLastPowered\x00")).as_ptr(),
              b"psObject != NULL\x00" as *const u8 as *const libc::c_char);
    };
    if (*asPower[(*psObject).player as usize]).psLastPowered.is_null() {
        return 1 as libc::c_int
    }
    /*if we've got round to the last object again, by setting to NULL will
	enable the next object to get some power*/
    if (*asPower[(*psObject).player as usize]).psLastPowered == psObject {
        (*asPower[(*psObject).player as usize]).psLastPowered =
            0 as *mut _base_object
    }
    return 0 as libc::c_int;
}
//returns the relevant list based on OffWorld or OnWorld for the accruePower function
//extern STRUCTURE* powerUpdateStructList(UBYTE player);
/*inform the players power struct that the last Object to receive power has changed*/
/*inform the players power struct that the last object to receive power has changed*/
#[no_mangle]
pub unsafe extern "C" fn updateLastPowered(mut psObject: *mut BASE_OBJECT,
                                           mut player: UBYTE) {
    (*asPower[player as usize]).psLastPowered = psObject;
}
/*	Returns the next res. Ext. in the list from the one passed in. returns 1st one
	in list if passed in is NULL and NULL if there's none?
*/
#[no_mangle]
pub unsafe extern "C" fn getRExtractor(mut psStruct: *mut STRUCTURE)
 -> *mut STRUCTURE {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFirst: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut bGonePastIt: BOOL = 0;
    psCurr = apsStructLists[selectedPlayer as usize];
    psFirst = 0 as *mut STRUCTURE;
    bGonePastIt = 0 as libc::c_int;
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            if psFirst.is_null() { psFirst = psCurr }
            if psStruct.is_null() {
                return psCurr
            } else {
                if psCurr != psStruct && bGonePastIt != 0 { return psCurr }
            }
            if psCurr == psStruct { bGonePastIt = 1 as libc::c_int }
        }
        psCurr = (*psCurr).psNext
    }
    return psFirst;
}
/*defines which structure types draw power - returns TRUE if use power*/
/*defines which structure types draw power - returns TRUE if use power*/
#[no_mangle]
pub unsafe extern "C" fn structUsesPower(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    let mut bUsesPower: BOOL = 0 as libc::c_int;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structUsesPower: Invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"power.c\x00" as *const u8 as *const libc::c_char,
              895 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"structUsesPower\x00")).as_ptr(),
              b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    match (*(*psStruct).pStructureType).type_0 {
        1 | 16 | 17 | 10 | 12 => { bUsesPower = 1 as libc::c_int }
        _ => { bUsesPower = 0 as libc::c_int }
    }
    return bUsesPower;
}
/*defines which droid types draw power - returns TRUE if use power*/
/*defines which droid types draw power - returns TRUE if use power*/
#[no_mangle]
pub unsafe extern "C" fn droidUsesPower(mut psDroid: *mut DROID) -> BOOL {
    let mut bUsesPower: BOOL = 0 as libc::c_int;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitUsesPower: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"power.c\x00" as *const u8 as *const libc::c_char,
              920 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"droidUsesPower\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    match (*psDroid).droidType as libc::c_uint {
        3 | 8 | 10 | 11 => { bUsesPower = 1 as libc::c_int }
        _ => { bUsesPower = 0 as libc::c_int }
    }
    return bUsesPower;
}
//won't bother with this on PSX unless starts being used too much!
//this is a check cos there is a problem with the power but not sure where!!
//won't bother with this on PSX unless starts being used too much!
//this is a check cos there is a problem with the power but not sure where!!
#[no_mangle]
pub unsafe extern "C" fn powerCheck(mut bBeforePowerUsed: BOOL,
                                    mut player: UBYTE) {
    static mut psLastPowered: *mut BASE_OBJECT =
        0 as *const BASE_OBJECT as *mut BASE_OBJECT;
    static mut bPowerBefore: BOOL = 0 as libc::c_int;
    if bBeforePowerUsed != 0 {
        //set what the lastPowered object is before using any power
        psLastPowered = (*asPower[player as usize]).psLastPowered;
        bPowerBefore = 0 as libc::c_int;
        //check that there is power available at start of loop
        if (*asPower[player as usize]).currentPower >
               5 as libc::c_int as libc::c_uint {
            bPowerBefore = 1 as libc::c_int
        }
    } else if !psLastPowered.is_null() &&
                  psLastPowered == (*asPower[player as usize]).psLastPowered
                  && bPowerBefore != 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"powerCheck: trouble at mill!\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"power.c\x00" as *const u8 as *const libc::c_char,
                  966 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 11],
                                            &[libc::c_char; 11]>(b"powerCheck\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        /*check to see if we've been thru the whole list of structures and
        droids and not reset the lastPowered object in the power structure and
        there was some power at the start of the loop to use*/
        //initialise so something can have some power next cycle
        (*asPower[player as usize]).psLastPowered = 0 as *mut _base_object
    };
}
/*resets the power levels for all players when power is turned back on*/
/*void powerCalc(BOOL on)
{
	UDWORD		inc;

	if (on)
	{
		powerCalculated = TRUE;
		for (inc=0; inc < MAX_PLAYERS; inc++)
		{
			resetPlayerPower(inc, NULL);
		}
	}
	else
	{
		powerCalculated = FALSE;
	}
}*/
/*looks through the player's list of droids and structures to see what power
has been used*/
/*UDWORD calcPlayerUsedPower(UDWORD player)
{
	DROID		*psDroid;
	STRUCTURE	*psStruct;
	UDWORD		used = 0;

	for (psDroid = apsDroidLists[player]; psDroid != NULL; psDroid = psDroid->
		psNext)
	{
		used += psDroid->power;
	}
	for (psStruct = apsStructLists[player]; psStruct != NULL; psStruct =
		psStruct->psNext)
	{
		used += psStruct->pStructureType->powerToBuild;
	}
	return used;
}*/
/*reset the power levels when a power_gen or resource_extractor is destroyed */
/*BOOL resetPlayerPower(UDWORD player, STRUCTURE *psStruct)
{
	STRUCTURE	*psBuilding;
	UDWORD		usedPower;


	asPower[player]->extractedPower = 0;
	asPower[player]->capacity = 0;

	//total up the resources
	for (psBuilding = apsStructLists[player]; psBuilding != NULL; psBuilding =
		psBuilding->psNext)
	{
		if (psStruct != psBuilding)
		{
			if (psBuilding->pStructureType->type == REF_RESOURCE_EXTRACTOR OR
				psBuilding->pStructureType->type == REF_HQ)
			{
				extractedPowerUpdate(psBuilding);
			}
		}
	}

	asPower[player]->availablePower = 0;

	// add multiplayer allowances if a multiplayer game.
#ifndef PSX
	if(bMultiPlayer && (game.dmatch || game.base == CAMP_CLEAN))
	{
		asPower[player]->availablePower = game.power+RESIDUAL_POW;
	}
#endif

	//calculate the total available and capacity
	for (psBuilding = apsStructLists[player]; psBuilding != NULL; psBuilding =
		psBuilding->psNext)
	{
		if (psStruct != psBuilding)
		{
			if (psBuilding->pStructureType->type == REF_POWER_GEN OR
				psBuilding->pStructureType->type == REF_HQ)
			{
				capacityUpdate(psBuilding);
			}
		}
	}

	//adjust for the power already used
	usedPower = calcPlayerUsedPower(player);
	if ((SDWORD)asPower[player]->availablePower < usedPower)
	{
		asPower[player]->usedPower = asPower[player]->availablePower -
			usedPower;
		asPower[player]->availablePower = 0;
		if (player == selectedPlayer)
		{
			audio_QueueTrack( ID_SOUND_POWER_LOW );
		}
	}
	else
	{
		asPower[player]->availablePower -= usedPower;
		asPower[player]->usedPower = usedPower;
	}

	return TRUE;
}*/
/*update the generator capacity if necessary. This function is called whenever
  a Power Generator or Power Module is built or HQ is built*/
/*void capacityUpdate(STRUCTURE * psBuilding)
{
	UDWORD		power = 0;

	if (psBuilding->pStructureType->type == REF_POWER_GEN)
	{
		//only add on module power if this is what has been built BIT OF A HACK REALLY!
		if (((POWER_GEN*)psBuilding->pFunctionality)->power != ((POWER_GEN_FUNCTION*)
			psBuilding->pStructureType->asFuncList[0])->powerOutput)
		{
			//subtract the power output without modules
			asPower[psBuilding->player]->capacity -= ((POWER_GEN_FUNCTION*)
				psBuilding->pStructureType->asFuncList[0])->powerOutput;
		}
		//asPower[psBuilding->player]->capacity += (((POWER_GEN *)psBuilding->
		//	pFunctionality)->power * ((POWER_GEN *)psBuilding->pFunctionality)->
		//	multiplier);

		power = (((POWER_GEN *)psBuilding->pFunctionality)->power *
			((POWER_GEN *)psBuilding->pFunctionality)->multiplier);
	}
	else
	{
		if (psBuilding->pStructureType->type == REF_HQ)
		{
			power = ((HQ_FUNCTION *)psBuilding->pStructureType->asFuncList[0])->power;
		}
	}
	if (power)
	{
		asPower[psBuilding->player]->capacity += power;

		//now check if there is any spare extracted power
		if (asPower[psBuilding->player]->extractedPower != 0)
		{
			availablePowerUpdate(psBuilding);
		}
	}
}*/
/*Update the extracted power if necessary */
/*void extractedPowerUpdate(STRUCTURE *psBuilding)
{
	STRUCTURE	*psStruct;
	UDWORD		power = 0;

	if (psBuilding->pStructureType->type == REF_RESOURCE_EXTRACTOR)
	{
		power = ((RES_EXTRACTOR *)psBuilding->pFunctionality)->power;
	}
	if (psBuilding->pStructureType->type == REF_HQ)
	{
		power = ((HQ_FUNCTION *)psBuilding->pStructureType->asFuncList[0])->power;
	}
	if (power)
	{
		asPower[psBuilding->player]->extractedPower += power;

		//increase in extracted power might lead to more power being available.
		if (asPower[psBuilding->player]->capacity != 0)
		{
			for (psStruct = apsStructLists[psBuilding->player]; psStruct != NULL AND
				asPower[psBuilding->player]->extractedPower != 0; psStruct =
				psStruct->psNext)
			{
				if (psStruct->pStructureType->type == REF_POWER_GEN OR
					psStruct->pStructureType->type == REF_HQ)
				{
					availablePowerUpdate(psStruct);
				}
			}
		}
	}
}*/
/*Update the available power if necessary */
/*void availablePowerUpdate(STRUCTURE *psBuilding)
{
	UDWORD			power;

	//check its the correct type of building
	if (psBuilding->pStructureType->type == REF_POWER_GEN OR
		psBuilding->pStructureType->type == REF_HQ)
	{
		if ((SDWORD)(asPower[psBuilding->player]->extractedPower) > asPower[psBuilding->
			player]->capacity)
		{
			power = asPower[psBuilding->player]->capacity;
			asPower[psBuilding->player]->extractedPower -= power;
		}
		else
		{
			power = asPower[psBuilding->player]->extractedPower;
			asPower[psBuilding->player]->extractedPower = 0;
		}

		//for power gens need to use the multiplier
		//if (psBuilding->pStructureType->type == REF_POWER_GEN)
		//{
		//	power = power * ((POWER_GEN *)psBuilding->pFunctionality)->multiplier;
		//}

		asPower[psBuilding->player]->capacity -= power;
		asPower[psBuilding->player]->availablePower += power ;
	}
}*/
//return the power when a structure/droid is destroyed
/*void returnPower(UDWORD player, UDWORD quantity)
{
	if (asPower[player]->usedPower < 0)
	{
		asPower[player]->usedPower += quantity;
	}
	else
	{
		asPower[player]->usedPower -= quantity;
		asPower[player]->availablePower += quantity;
	}
}*/
/*check the available power - if enough subtracts the amount
 required to perform the task and returns true, else returns
 false */
/*BOOL usePower(UDWORD player, UDWORD quantity)
{
	//if not doing a check on the power - just use the power and return TRUE
	if (!powerCalculated)
	{
		if (asPower[player]->usedPower >= 0)
		{
			asPower[player]->usedPower += quantity;
		}
		else
		{
			asPower[player]->usedPower -= quantity;
		}

		return TRUE;
	}

	//check there is enough first
	if (asPower[player]->availablePower >= quantity)
	{
		asPower[player]->availablePower -= quantity;
		asPower[player]->usedPower += quantity;
		return TRUE;
	}
	else if (player == selectedPlayer)
	{
#ifdef PSX
#warning POWER LOW IS DISABLED
		return TRUE;
#endif

		if(titleMode == FORCESELECT) //|| (titleMode == DESIGNSCREEN))
		{
			return FALSE;
		}

		if(player == selectedPlayer)
		{
			audio_QueueTrack( ID_SOUND_POWER_LOW );
		}

		return FALSE;
	}
}*/
/*check the available power - if enough return true, else return false */
/*BOOL checkPower(UDWORD player, UDWORD quantity, BOOL playAudio)
{
	//if not doing a check on the power - just return TRUE
	if (!powerCalculated)
	{
		return TRUE;
	}

	if (asPower[player]->availablePower >= quantity)
	{
		return TRUE;
	}
	else if (player == selectedPlayer)
	{
#ifdef PSX
#warning POWER LOW IS DISABLED
		return TRUE;
#endif
		if (playAudio && player == selectedPlayer)
		{
			audio_QueueTrack( ID_SOUND_POWER_LOW );
			return FALSE;
		}
	}
	return FALSE;
}*/
/*initialise the PlayerPower based on what structures are available*/
/*BOOL initPlayerPower(void)
{
	UDWORD		player, usedPower;
	STRUCTURE	*psBuilding;

	//total up the resources
	for (player = 0; player < MAX_PLAYERS; player++)
	{
		asPower[player]->extractedPower = 0;
		for (psBuilding = apsStructLists[player]; psBuilding != NULL; psBuilding =
			psBuilding->psNext)
		{
			if (psBuilding->pStructureType->type == REF_RESOURCE_EXTRACTOR OR
				psBuilding->pStructureType->type == REF_HQ)
			{
				extractedPowerUpdate(psBuilding);
			}
		}
	}

	//calculate the total available and capacity
	for (player = 0; player < MAX_PLAYERS; player++)
	{
		asPower[player]->availablePower = 0;
		asPower[player]->capacity = 0;
		for (psBuilding = apsStructLists[player]; psBuilding != NULL; psBuilding =
			psBuilding->psNext)
		{
			if (psBuilding->pStructureType->type == REF_POWER_GEN OR
				psBuilding->pStructureType->type == REF_HQ)
			{
				capacityUpdate(psBuilding);
			}
		}
	}

	//adjust for the power already used
	for (player = 0; player < MAX_PLAYERS; player++)
	{
		usedPower = calcPlayerUsedPower(player);
		if (usedPower > 0)
		{
			if ((SDWORD)asPower[player]->availablePower < usedPower)
			{
				asPower[player]->usedPower = asPower[player]->availablePower -
					usedPower;
				asPower[player]->availablePower = 0;
//				audio_PlayTrack( ID_SOUND_POWER_LOW );
			}
			else
			{
				asPower[player]->availablePower -= usedPower;
				asPower[player]->usedPower = usedPower;
			}
		}
	}

	return TRUE;
}*/
