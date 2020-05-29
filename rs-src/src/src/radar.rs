use ::libc;
extern "C" {
    pub type _formation;
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
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    //*************************************************************************
    #[no_mangle]
    static mut colours: [uint8; 0];
    #[no_mangle]
    static mut palShades: [uint8; 4096];
    #[no_mangle]
    fn pal_GetNearestColour(r: uint8, g: uint8, b: uint8) -> uint8;
    #[no_mangle]
    fn pie_GetGamePal() -> *mut iColour;
    #[no_mangle]
    fn pie_TransBoxFill(x0: SDWORD, y0: SDWORD, x1: SDWORD, y1: SDWORD);
    #[no_mangle]
    fn pie_InitRadar() -> BOOL;
    #[no_mangle]
    fn pie_ShutdownRadar() -> BOOL;
    #[no_mangle]
    fn pie_DownLoadRadar(buffer: *mut libc::c_uchar, texPageID: UDWORD);
    #[no_mangle]
    fn pie_RenderRadar(Image: *mut IMAGEDEF, Bmp: *mut iBitmap,
                       Modulus: UDWORD, x: libc::c_int, y: libc::c_int);
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    /* The list of proximity displays allocated */
    #[no_mangle]
    static mut apsProxDisp: [*mut PROXIMITY_DISPLAY; 8];
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut distance: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    //scroll min and max values
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    #[no_mangle]
    static mut godMode: BOOL;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn DrawEnableLocks(Enable: BOOL);
    #[no_mangle]
    fn RenderWindowFrame(Frame: *mut IMAGEFRAME, x: UDWORD, y: UDWORD,
                         Width: UDWORD, Height: UDWORD);
    #[no_mangle]
    static mut FrameRadar: IMAGEFRAME;
    #[no_mangle]
    fn gamePaused() -> BOOL;
    #[no_mangle]
    fn gameUpdatePaused() -> BOOL;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn getCampaignNumber() -> UDWORD;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn pie_DrawViewingWindow(v: *mut iVector, x1: UDWORD, y1: UDWORD,
                             x2: UDWORD, y2: UDWORD, colour: UDWORD);
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
//*************************************************************************
//
// Simple derived types
//
//*************************************************************************
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iColour {
    pub r: uint8,
    pub g: uint8,
    pub b: uint8,
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
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
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
/* LOC used for holding locations for Sensors and ECM's*/
/* SIZE used for specifying body size */
//only using KINETIC and HEAT for now
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
// weapon subclasses used to define which weapons are affected by weapon upgrade functions
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
// used to define which projectile model to use for the weapon
//used to modify the damage to a propuslion type (or structure) based on weapon
/* common stats - N.B. system points for a body
	 * are the limit for all other components
	 */
//UDWORD		configuration;		// Body shape
	//UDWORD		armourMult;			// How configuration affects body shape
									// Actually could be a fractional value
									// store x 10
// How big the body is - affects how hit
//UDWORD		bodyPoints;			// How much damage the body can take before destruction
// The number of weapon slots on the body
// A measure of how much protection the armour provides
// cross-ref with the weapon types
// A measure of how much energy the power plant outputs
//list of IMDs to use for propulsion unit - up to numPropulsionStats
//pointer to which flame graphic to use - for VTOLs only at the moment
/* Common stats */
// Program capacity
//UDWORD		AICap;				// AI capacity
	//UDWORD		AISpeed;			// AI Learning Speed
//weapon stats associated with this brain - for Command Droids
//defines the left and right sides for propulsion IMDs
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
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
pub type MOVE_CONTROL = _move_control;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_group {
    pub type_0: SWORD,
    pub refCount: SWORD,
    pub psList: *mut DROID,
    pub psCommander: *mut DROID,
    pub sRunData: RUN_DATA,
}
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
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
pub type _position_type = libc::c_uint;
pub const POS_TEMPDELIVERY: _position_type = 3;
pub const POS_PROXOBJ: _position_type = 2;
pub const POS_PROXDATA: _position_type = 1;
pub const POS_DELIVERY: _position_type = 0;
pub type POSITION_TYPE = _position_type;
pub type STRUCTURE = _structure;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/*
 * messageDef.h
 *
 * Message structure definitions
 */
pub type MESSAGE_TYPE = _message_type;
pub type _prox_type = libc::c_uint;
pub const PROX_TYPES: _prox_type = 3;
pub const PROX_ARTEFACT: _prox_type = 2;
pub const PROX_RESOURCE: _prox_type = 1;
pub const PROX_ENEMY: _prox_type = 0;
pub type PROX_TYPE = _prox_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _view_proximity {
    pub x: UDWORD,
    pub y: UDWORD,
    pub z: UDWORD,
    pub proxType: PROX_TYPE,
    pub audioID: SDWORD,
}
pub type VIEW_PROXIMITY = _view_proximity;
pub type VIEWDATA = _viewdata;
pub type MSG_VIEWDATA = *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _proximity_display {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
    pub psMessage: *mut MESSAGE,
    pub radarX: UDWORD,
    pub radarY: UDWORD,
    pub timeLastDrawn: UDWORD,
    pub strobe: UDWORD,
    pub buttonID: UDWORD,
    pub psNext: *mut _proximity_display,
}
pub type PROXIMITY_DISPLAY = _proximity_display;
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
// Research message
// Campaign message
// Mission Report messages
// Proximity message
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFRAME {
    pub OffsetX0: SWORD,
    pub OffsetY0: SWORD,
    pub OffsetX1: SWORD,
    pub OffsetY1: SWORD,
    pub TopLeft: SWORD,
    pub TopRight: SWORD,
    pub BottomLeft: SWORD,
    pub BottomRight: SWORD,
    pub TopEdge: SWORD,
    pub TopType: SWORD,
    pub RightEdge: SWORD,
    pub RightType: SWORD,
    pub BottomEdge: SWORD,
    pub BottomType: SWORD,
    pub LeftEdge: SWORD,
    pub LeftType: SWORD,
    pub FRect: [FRAMERECT; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FRAMERECT {
    pub Type: UWORD,
    pub TLXOffset: SWORD,
    pub TLYOffset: SWORD,
    pub BRXOffset: SWORD,
    pub BRYOffset: SWORD,
    pub ColourIndex: UBYTE,
}
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
// If defined then draw a box to show the viewing area.
//#define	RADAR_TRIANGLE_TOPLEFT	// If defined then put direction triangle in top left
								// otherwise put in view box and scale it by zoom level.
static mut sweep: UDWORD = 0;
static mut colBlack: UBYTE = 0;
static mut colWhite: UBYTE = 0;
static mut colRadarBorder: UBYTE = 0;
static mut colGrey: UBYTE = 0;
static mut colRadarEnemy: UBYTE = 0;
static mut colRadarAlly: UBYTE = 0;
static mut colRadarMe: UBYTE = 0;
#[no_mangle]
pub static mut bDrawRadarTerrain: BOOL = 1 as libc::c_int;
//radar terrain on/off
#[no_mangle]
pub static mut bEnemyAllyRadarColor: BOOL = 0 as libc::c_int;
//enemy/ally radar color
// colours for each clan on the radar map.
static mut clanColours: [[UDWORD; 8]; 3] =
    [[81 as libc::c_int as UDWORD, 243 as libc::c_int as UDWORD,
      231 as libc::c_int as UDWORD, 1 as libc::c_int as UDWORD,
      182 as libc::c_int as UDWORD, 187 as libc::c_int as UDWORD,
      207 as libc::c_int as UDWORD, 195 as libc::c_int as UDWORD],
     [81 as libc::c_int as UDWORD, 243 as libc::c_int as UDWORD,
      231 as libc::c_int as UDWORD, 1 as libc::c_int as UDWORD,
      182 as libc::c_int as UDWORD, 187 as libc::c_int as UDWORD,
      207 as libc::c_int as UDWORD, 195 as libc::c_int as UDWORD],
     [81 as libc::c_int as UDWORD, 243 as libc::c_int as UDWORD,
      231 as libc::c_int as UDWORD, 1 as libc::c_int as UDWORD,
      182 as libc::c_int as UDWORD, 187 as libc::c_int as UDWORD,
      207 as libc::c_int as UDWORD, 195 as libc::c_int as UDWORD]];
static mut flashColours: [[UDWORD; 8]; 3] =
    [[165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      255 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD],
     [165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      255 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD],
     [165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      255 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD,
      165 as libc::c_int as UDWORD, 165 as libc::c_int as UDWORD]];
static mut tileColours: [UBYTE; 100] = [0; 100];
static mut radarStrobe: BOOL = 0;
static mut radarStrobeX: UDWORD = 0;
static mut radarStrobeY: UDWORD = 0;
static mut radarStrobeIndex: UDWORD = 0;
static mut sweepStrobeIndex: UDWORD = 0;
static mut radarBuffer: *mut UBYTE = 0 as *const UBYTE as *mut UBYTE;
static mut RadarScrollX: SDWORD = 0;
static mut RadarScrollY: SDWORD = 0;
static mut RadarWidth: SDWORD = 0;
static mut RadarHeight: SDWORD = 0;
static mut RadVisWidth: SDWORD = 0;
static mut RadVisHeight: SDWORD = 0;
static mut RadarOffsetX: SDWORD = 0;
static mut RadarOffsetY: SDWORD = 0;
static mut RadarRedraw: BOOL = 0;
static mut RadarZoom: UWORD = 0;
static mut RadarImage: IMAGEDEF =
    IMAGEDEF{TPageID: 0,
             PalID: 0,
             Tu: 0,
             Tv: 0,
             Width: 0,
             Height: 0,
             XOffset: 0,
             YOffset: 0,};
static mut RadarMapOriginX: SDWORD = 0;
static mut RadarMapOriginY: SDWORD = 0;
static mut RadarMapWidth: SDWORD = 0;
static mut RadarMapHeight: SDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn radarInitVars() {
    sweep = 0 as libc::c_int as UDWORD;
    radarStrobe = 1 as libc::c_int;
    RadarScrollX = 0 as libc::c_int;
    RadarScrollY = 0 as libc::c_int;
    RadarWidth = 128 as libc::c_int;
    RadarHeight = 128 as libc::c_int;
    RadarRedraw = 1 as libc::c_int;
    RadarOffsetX = 0 as libc::c_int;
    RadarOffsetY = 0 as libc::c_int;
    RadarZoom = 0 as libc::c_int as UWORD;
}
//#define RADAR_POSITION_AT_ZOOM
/* Radar.h */
//called for when a new mission is started
#[no_mangle]
pub unsafe extern "C" fn resetRadarRedraw() {
    RadarRedraw = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn InitRadar() -> BOOL {
    radarBuffer =
        memMallocRelease((128 as libc::c_int * 128 as libc::c_int) as size_t)
            as *mut UBYTE;
    if radarBuffer.is_null() { return 0 as libc::c_int }
    memset(radarBuffer as *mut libc::c_void, 0 as libc::c_int,
           (128 as libc::c_int * 128 as libc::c_int) as libc::c_uint);
    // Set up an image structure for the radar bitmap so we can draw
// it useing iV_DrawImageDef().
    RadarImage.TPageID =
        31 as libc::c_int as
            UWORD; // 3dfx only,radar is hard coded to texture page 31 - sort this out?
    RadarImage.Tu = 0 as libc::c_int as UWORD;
    RadarImage.Tv = 0 as libc::c_int as UWORD;
    RadarImage.Width = RadarWidth as UWORD;
    RadarImage.Height = RadarHeight as UWORD;
    RadarImage.XOffset = 0 as libc::c_int as SWORD;
    RadarImage.YOffset = 0 as libc::c_int as SWORD;
    colRadarBorder = *colours.as_mut_ptr().offset(7 as libc::c_int as isize);
    colBlack = 0 as libc::c_int as UBYTE;
    colGrey = *colours.as_mut_ptr().offset(8 as libc::c_int as isize);
    colWhite = *colours.as_mut_ptr().offset(15 as libc::c_int as isize);
    //for enemy/ally radar color mode
    colRadarAlly = *colours.as_mut_ptr().offset(14 as libc::c_int as isize);
    colRadarEnemy = *colours.as_mut_ptr().offset(12 as libc::c_int as isize);
    colRadarMe = *colours.as_mut_ptr().offset(15 as libc::c_int as isize);
    pie_InitRadar();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ShutdownRadar() -> BOOL {
    pie_ShutdownRadar();
    memFreeRelease(radarBuffer as *mut libc::c_void);
    radarBuffer = 0 as *mut UBYTE;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetRadarZoom(mut ZoomLevel: UWORD) {
    if ZoomLevel as libc::c_int <= 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"SetRadarZoom: Max radar zoom exceeded\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ZoomLevel as libc::c_int <= 2 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"radar.c\x00" as *const u8 as *const libc::c_char,
              178 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"SetRadarZoom\x00")).as_ptr(),
              b"ZoomLevel <= MAX_RADARZOOM\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ZoomLevel as libc::c_int != RadarZoom as libc::c_int {
        RadarZoom = ZoomLevel;
        RadarRedraw = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetRadarZoom() -> UDWORD {
    return RadarZoom as UDWORD;
}
// Given a position within the radar, return a world coordinate.
//
#[no_mangle]
pub unsafe extern "C" fn CalcRadarPosition(mut mX: UDWORD, mut mY: UDWORD,
                                           mut PosX: *mut UDWORD,
                                           mut PosY: *mut UDWORD) {
    let mut boxSizeH: UWORD = 0;
    let mut boxSizeV: UWORD = 0;
    let mut sPosX: SDWORD = 0;
    let mut sPosY: SDWORD = 0;
    let mut Xoffset: SDWORD = 0;
    let mut Yoffset: SDWORD = 0;
    CalcRadarPixelSize(&mut boxSizeH, &mut boxSizeV);
    CalcRadarScroll(boxSizeH, boxSizeV);
    //*PosX = ((mX-RADTLX-RadarOffsetX)/boxSizeH)+RadarScrollX+RadarMapOriginX;
	//*PosY = ((mY-RADTLY-RadarOffsetY)/boxSizeV)+RadarScrollY+RadarMapOriginY;
    // Calculate where on the radar we clicked
    Xoffset =
        mX.wrapping_sub(((23 as libc::c_int + 132 as libc::c_int +
                              6 as libc::c_int) as
                             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint)).wrapping_add(320
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_int
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_uint).wrapping_add(6
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                 as
                                                                                                                                                                                                 libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                     libc::c_uint))).wrapping_sub(RadarOffsetX
                                                                                                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                                                                                                      libc::c_uint)
            as SDWORD;
    // we need to check for negative values (previously this meant that sPosX/Y were becoming huge)
    if Xoffset < 0 as libc::c_int { Xoffset = 0 as libc::c_int }
    Yoffset =
        mY.wrapping_sub((324 as libc::c_int as
                             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)).wrapping_add(1
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)).wrapping_sub(RadarOffsetY
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint)
            as SDWORD;
    if Yoffset < 0 as libc::c_int { Yoffset = 0 as libc::c_int }
    sPosX =
        Xoffset / boxSizeH as libc::c_int + RadarScrollX + RadarMapOriginX;
    sPosY =
        Yoffset / boxSizeV as libc::c_int + RadarScrollY + RadarMapOriginY;
    if sPosX < scrollMinX {
        sPosX = scrollMinX
    } else if sPosX > scrollMaxX { sPosX = scrollMaxX }
    if sPosY < scrollMinY {
        sPosY = scrollMinY
    } else if sPosY > scrollMaxY { sPosY = scrollMaxY }
    *PosX = sPosX as UDWORD;
    *PosY = sPosY as UDWORD;
}
//given a world pos, return a radar pos..
// ajl did this, so don't blame paul when it barfs...
#[no_mangle]
pub unsafe extern "C" fn worldPosToRadarPos(mut wX: UDWORD, mut wY: UDWORD,
                                            mut rX: *mut SDWORD,
                                            mut rY: *mut SDWORD) {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut boxSizeH: UWORD = 0;
    let mut boxSizeV: UWORD = 0;
    CalcRadarPixelSize(&mut boxSizeH, &mut boxSizeV);
    CalcRadarScroll(boxSizeH, boxSizeV);
    x =
        wX.wrapping_sub(RadarScrollX as
                            libc::c_uint).wrapping_sub(RadarMapOriginX as
                                                           libc::c_uint).wrapping_mul(boxSizeH
                                                                                          as
                                                                                          libc::c_uint).wrapping_add(RadarOffsetX
                                                                                                                         as
                                                                                                                         libc::c_uint)
            as SDWORD;
    y =
        wY.wrapping_sub(RadarScrollY as
                            libc::c_uint).wrapping_sub(RadarMapOriginY as
                                                           libc::c_uint).wrapping_mul(boxSizeV
                                                                                          as
                                                                                          libc::c_uint).wrapping_add(RadarOffsetY
                                                                                                                         as
                                                                                                                         libc::c_uint)
            as SDWORD;
    *rX = x;
    *rY = y;
}
// Kick of a new radar strobe.
//
#[no_mangle]
pub unsafe extern "C" fn SetRadarStrobe(mut x: UDWORD, mut y: UDWORD) {
    radarStrobe = 1 as libc::c_int;
    radarStrobeX = x;
    radarStrobeY = y;
    radarStrobeIndex = 0 as libc::c_int as UDWORD;
}
// Calculate the radar pixel sizes.
//
unsafe extern "C" fn CalcRadarPixelSize(mut SizeH: *mut UWORD,
                                        mut SizeV: *mut UWORD) {
    let mut Size: UWORD =
        ((1 as libc::c_int) << RadarZoom as libc::c_int) as UWORD;
    *SizeH = Size;
    *SizeV = Size;
    //#ifdef FORCEPIXELSIZE
//#ifndef PSX
//	*SizeH = 2;
//	*SizeV = 2;
//#else
//	*SizeH = 2;
//	*SizeV = 2;
//#endif
//#else
//	UWORD boxSizeH,boxSizeV;
//
//	boxSizeH = (UWORD)(RadarWidth/mapWidth);
//	boxSizeV = (UWORD)(RadarHeight/mapHeight);
//
// // Ensure boxSizeH and V are always 1 or greater and equal to each other.
//	if((boxSizeH == 0) || (boxSizeV ==0)) {
//		boxSizeH = boxSizeV = 1;
//	} else if(boxSizeH > boxSizeV) {
//		boxSizeV = boxSizeH;
//	} else {
//		boxSizeH = boxSizeV;
//	}
//
//	*SizeH = boxSizeH;
//	*SizeV = boxSizeV;
//#endif
}
// Calculate the radar scroll positions from the current player position.
//
unsafe extern "C" fn CalcRadarScroll(mut boxSizeH: UWORD,
                                     mut boxSizeV: UWORD) {
    let mut viewX: SDWORD = 0;
    let mut viewY: SDWORD = 0;
    let mut PrevRadarOffsetX: SDWORD = RadarOffsetX;
    let mut PrevRadarOffsetY: SDWORD = RadarOffsetY;
    let mut PrevRadarScrollX: SDWORD = RadarScrollX;
    let mut PrevRadarScrollY: SDWORD = RadarScrollY;
    let mut PrevRadarMapOriginX: SDWORD = RadarMapOriginX;
    let mut PrevRadarMapOriginY: SDWORD = RadarMapOriginY;
    let mut PrevRadarMapWidth: SDWORD = RadarMapWidth;
    let mut PrevRadarMapHeight: SDWORD = RadarMapHeight;
    let mut BorderX: SDWORD = 0;
    let mut BorderY: SDWORD = 0;
    RadarMapOriginX = scrollMinX;
    RadarMapOriginY = scrollMinY;
    RadarMapWidth = scrollMaxX - scrollMinX;
    RadarMapHeight = scrollMaxY - scrollMinY;
    RadVisWidth = RadarWidth;
    RadVisHeight = RadarHeight;
    if RadarMapWidth < RadVisWidth / boxSizeH as libc::c_int {
        RadVisWidth = RadarMapWidth * boxSizeH as libc::c_int;
        RadarOffsetX = (RadarWidth - RadVisWidth) / 2 as libc::c_int
    } else { RadarOffsetX = 0 as libc::c_int }
    if RadarMapHeight < RadVisHeight / boxSizeV as libc::c_int {
        RadVisHeight = RadarMapHeight * boxSizeV as libc::c_int;
        RadarOffsetY = (RadarHeight - RadVisHeight) / 2 as libc::c_int
    } else { RadarOffsetY = 0 as libc::c_int }
    BorderX =
        (RadVisWidth as
             libc::c_uint).wrapping_sub(visibleXTiles.wrapping_mul(boxSizeH as
                                                                       libc::c_uint)).wrapping_div(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    BorderY =
        (RadVisHeight as
             libc::c_uint).wrapping_sub(visibleYTiles.wrapping_mul(boxSizeV as
                                                                       libc::c_uint)).wrapping_div(2
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    BorderX /= boxSizeH as libc::c_int;
    BorderY /= boxSizeV as libc::c_int;
    if BorderX > 16 as libc::c_int {
        BorderX = 16 as libc::c_int
    } else if BorderX < 0 as libc::c_int { BorderX = 0 as libc::c_int }
    if BorderY > 16 as libc::c_int {
        BorderY = 16 as libc::c_int
    } else if BorderY < 0 as libc::c_int { BorderY = 0 as libc::c_int }
    viewX = player.p.x / 128 as libc::c_int - RadarScrollX - RadarMapOriginX;
    viewY = player.p.z / 128 as libc::c_int - RadarScrollY - RadarMapOriginY;
    if viewX < BorderX { RadarScrollX += viewX - BorderX }
    viewX =
        (viewX as libc::c_uint).wrapping_add(visibleXTiles) as SDWORD as
            SDWORD;
    if viewX > RadVisWidth / boxSizeH as libc::c_int - BorderX {
        RadarScrollX +=
            viewX - RadVisWidth / boxSizeH as libc::c_int + BorderX
    }
    if viewY < BorderY { RadarScrollY += viewY - BorderY }
    viewY =
        (viewY as libc::c_uint).wrapping_add(visibleYTiles) as SDWORD as
            SDWORD;
    if viewY > RadVisHeight / boxSizeV as libc::c_int - BorderY {
        RadarScrollY +=
            viewY - RadVisHeight / boxSizeV as libc::c_int + BorderY
    }
    if RadarScrollX < 0 as libc::c_int {
        RadarScrollX = 0 as libc::c_int
    } else if RadarScrollX >
                  RadarMapWidth - RadVisWidth / boxSizeH as libc::c_int {
        RadarScrollX = RadarMapWidth - RadVisWidth / boxSizeH as libc::c_int
    }
    if RadarScrollY < 0 as libc::c_int {
        RadarScrollY = 0 as libc::c_int
    } else if RadarScrollY >
                  RadarMapHeight - RadVisHeight / boxSizeV as libc::c_int {
        RadarScrollY = RadarMapHeight - RadVisHeight / boxSizeV as libc::c_int
    }
    if PrevRadarOffsetX != RadarOffsetX || PrevRadarOffsetY != RadarOffsetY ||
           PrevRadarScrollX != RadarScrollX ||
           PrevRadarScrollY != RadarScrollY ||
           PrevRadarMapOriginX != RadarMapOriginX ||
           PrevRadarMapOriginY != RadarMapOriginY ||
           PrevRadarMapWidth != RadarMapWidth ||
           PrevRadarMapHeight != RadarMapHeight {
        RadarRedraw = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn drawRadar() {
    let mut boxSizeH: UWORD = 0;
    let mut boxSizeV: UWORD = 0;
    CalcRadarPixelSize(&mut boxSizeH, &mut boxSizeV);
    CalcRadarScroll(boxSizeH, boxSizeV);
    if RadarRedraw != 0 {
        if RadVisWidth != RadarWidth || RadVisHeight != RadarHeight {
            ClearRadar(radarBuffer, 128 as libc::c_int as UDWORD, boxSizeH,
                       boxSizeV);
        }
    }
    DrawRadarTiles(radarBuffer, 128 as libc::c_int as UDWORD, boxSizeH,
                   boxSizeV);
    DrawRadarObjects(radarBuffer, 128 as libc::c_int as UDWORD, boxSizeH,
                     boxSizeV);
    pie_DownLoadRadar(radarBuffer, 31 as libc::c_int as UDWORD);
    pie_TransBoxFill(((23 as libc::c_int + 132 as libc::c_int +
                           6 as libc::c_int) as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint).wrapping_div(2
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)).wrapping_add(320
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint).wrapping_add(6
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                  libc::c_uint))
                         as SDWORD,
                     (324 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_add(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint)
                         as SDWORD,
                     ((23 as libc::c_int + 132 as libc::c_int +
                           6 as libc::c_int) as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint).wrapping_div(2
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)).wrapping_add(320
                                                                                                                                                               as
                                                                                                                                                               libc::c_int
                                                                                                                                                               as
                                                                                                                                                               libc::c_uint).wrapping_add(6
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                             as
                                                                                                                                                                                                                             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                  libc::c_uint)).wrapping_add(128
                                                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                                                                                  libc::c_uint)
                         as SDWORD,
                     (324 as libc::c_int as
                          libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint)).wrapping_add(1
                                                                                                                                 as
                                                                                                                                 libc::c_int
                                                                                                                                 as
                                                                                                                                 libc::c_uint).wrapping_add(128
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint)
                         as SDWORD);
    //iV_DrawSemiTransImageDef(&RadarImage,radarBuffer,RadarWidth,RADTLX,RADTLY,192);
    pie_RenderRadar(&mut RadarImage, radarBuffer, RadarWidth as UDWORD,
                    ((23 as libc::c_int + 132 as libc::c_int +
                          6 as libc::c_int) as
                         libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_div(2
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint)).wrapping_add(320
                                                                                                                                                              as
                                                                                                                                                              libc::c_int
                                                                                                                                                              as
                                                                                                                                                              libc::c_uint).wrapping_add(6
                                                                                                                                                                                             as
                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                             as
                                                                                                                                                                                             libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                                                                                  libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                                                 libc::c_uint))
                        as libc::c_int,
                    (324 as libc::c_int as
                         libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)).wrapping_add(1
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)
                        as libc::c_int);
    DrawRadarExtras(boxSizeH, boxSizeV);
    UpdateRadar(boxSizeH, boxSizeV);
    RadarRedraw = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn downloadAtStartOfFrame() {
    pie_DownLoadRadar(radarBuffer, 31 as libc::c_int as UDWORD);
}
unsafe extern "C" fn UpdateRadar(mut boxSizeH: UWORD, mut boxSizeV: UWORD) {
    if gamePaused() == 0 {
        sweep =
            (sweep as libc::c_uint).wrapping_add(boxSizeV as libc::c_uint) as
                UDWORD as UDWORD
    }
    if sweep >= RadarHeight as UDWORD { sweep = 0 as libc::c_int as UDWORD }
    if gamePaused() == 0 {
        let fresh0 = sweepStrobeIndex;
        sweepStrobeIndex = sweepStrobeIndex.wrapping_add(1);
        if fresh0 >= 10 as libc::c_int as libc::c_uint {
            sweepStrobeIndex = 0 as libc::c_int as UDWORD
        }
    };
}
// Clear the radar buffer.
//
unsafe extern "C" fn ClearRadar(mut screen: *mut UBYTE, mut Modulus: UDWORD,
                                mut boxSizeH: UWORD, mut boxSizeV: UWORD) {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut Scr: *mut UBYTE = 0 as *mut UBYTE;
    let mut WScr: *mut UBYTE = 0 as *mut UBYTE;
    let mut RadWidth: SDWORD = 0;
    let mut RadHeight: SDWORD = 0;
    RadWidth = RadarWidth;
    RadHeight = RadarHeight;
    Scr = screen;
    i = 0 as libc::c_int;
    while i < RadWidth {
        WScr = Scr;
        j = 0 as libc::c_int;
        while j < RadHeight {
            *WScr = colBlack;
            WScr = WScr.offset(1);
            j += 1
        }
        Scr = Scr.offset(Modulus as isize);
        i += 1
    };
}
// Draw the map tiles on the radar.
//
unsafe extern "C" fn DrawRadarTiles(mut screen: *mut UBYTE,
                                    mut Modulus: UDWORD, mut boxSizeH: UWORD,
                                    mut boxSizeV: UWORD) {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut EndY: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut WTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut Scr: *mut UBYTE = 0 as *mut UBYTE;
    let mut WScr: *mut UBYTE = 0 as *mut UBYTE;
    let mut c: UDWORD = 0;
    let mut d: UDWORD = 0;
    let mut Ptr: *mut UBYTE = 0 as *mut UBYTE;
    let mut WPtr: *mut UBYTE = 0 as *mut UBYTE;
    let mut SizeH: UWORD = 0;
    let mut SizeV: UWORD = 0;
    let mut SweepPos: SDWORD = 0;
    let mut VisWidth: SDWORD = 0;
    let mut VisHeight: SDWORD = 0;
    let mut OffsetX: SDWORD = 0;
    let mut OffsetY: SDWORD = 0;
    let mut ShadeDiv: UBYTE = 0 as libc::c_int as UBYTE;
    SizeH = boxSizeH;
    SizeV = boxSizeV;
    VisWidth = RadVisWidth;
    VisHeight = RadVisHeight;
    SweepPos = sweep.wrapping_sub(RadarOffsetY as libc::c_uint) as SDWORD;
    OffsetX = RadarOffsetX;
    OffsetY = RadarOffsetY;
    if SizeV as libc::c_int != 0 as libc::c_int &&
           SizeV as libc::c_int != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Zero pixel size\x00" as *const u8 as *const libc::c_char);
    };
    if SizeV as libc::c_int != 0 as libc::c_int &&
           SizeV as libc::c_int != 0 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"radar.c\x00" as *const u8 as *const libc::c_char,
              539 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"DrawRadarTiles\x00")).as_ptr(),
              b"(SizeV!=0) && (SizeV!=0)\x00" as *const u8 as
                  *const libc::c_char);
    };
    SweepPos = SweepPos & !(SizeV as libc::c_int - 1 as libc::c_int);
    /* Get pointer to very first tile */
    psTile =
        psMapTiles.offset(RadarScrollX as
                              isize).offset((RadarScrollY as
                                                 libc::c_uint).wrapping_mul(mapWidth)
                                                as isize);
    psTile =
        psTile.offset((RadarMapOriginX as
                           libc::c_uint).wrapping_add((RadarMapOriginY as
                                                           libc::c_uint).wrapping_mul(mapWidth))
                          as isize);
    Scr =
        screen.offset(OffsetX as
                          isize).offset((OffsetY as
                                             libc::c_uint).wrapping_mul(Modulus)
                                            as isize);
    ShadeDiv = 4 as libc::c_int as UBYTE;
    if RadarRedraw != 0 {
        EndY = VisHeight
    } else {
        if SweepPos < 0 as libc::c_int || SweepPos >= VisHeight { return }
        EndY = 1 as libc::c_int;
        Scr =
            Scr.offset((SweepPos as libc::c_uint).wrapping_mul(Modulus) as
                           isize);
        psTile =
            psTile.offset(((SweepPos / SizeV as libc::c_int) as
                               libc::c_uint).wrapping_mul(mapWidth) as isize)
    }
    if SizeH as libc::c_int == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < EndY {
            WScr = Scr;
            WTile = psTile;
            j = 0 as libc::c_int;
            while j < VisWidth {
                if (*WTile).tileVisBits as libc::c_int &
                       (1 as libc::c_int) << selectedPlayer != 0 ||
                       godMode != 0 {
                    if bDrawRadarTerrain != 0 {
                        //draw radar terrain on/off feature
                        let mut i_0: libc::c_int =
                            tileColours[((*WTile).texture as libc::c_int &
                                             0x1ff as libc::c_int) as usize]
                                as libc::c_int * 16 as libc::c_int;
                        let mut j_0: libc::c_int =
                            (*WTile).illumination as libc::c_int >>
                                ShadeDiv as libc::c_int;
                        *WScr = palShades[(i_0 + j_0) as usize]
                    } else { *WScr = colBlack }
                } else { *WScr = colBlack }
                /* Next pixel, next tile */
                WScr = WScr.offset(1);
                WTile = WTile.offset(1);
                j += SizeV as libc::c_int
            }
            Scr = Scr.offset(Modulus as isize);
            psTile = psTile.offset(mapWidth as isize);
            i += SizeH as libc::c_int
        }
    } else {
        i = 0 as libc::c_int;
        while i < EndY {
            WTile = psTile;
            j = 0 as libc::c_int;
            while j < VisWidth {
                /* Only draw if discovered or in GOD mode */
                if (*WTile).tileVisBits as libc::c_int &
                       (1 as libc::c_int) << selectedPlayer != 0 ||
                       godMode != 0 {
                    let mut Val: UBYTE =
                        tileColours[((*WTile).texture as libc::c_int &
                                         0x1ff as libc::c_int) as usize];
                    Val =
                        palShades[(Val as libc::c_int * 16 as libc::c_int +
                                       ((*WTile).illumination as libc::c_int
                                            >> ShadeDiv as libc::c_int)) as
                                      usize];
                    Ptr =
                        Scr.offset(j as
                                       isize).offset((i as
                                                          libc::c_uint).wrapping_mul(Modulus)
                                                         as isize);
                    c = 0 as libc::c_int as UDWORD;
                    while c < SizeV as libc::c_uint {
                        WPtr = Ptr;
                        d = 0 as libc::c_int as UDWORD;
                        while d < SizeH as libc::c_uint {
                            if bDrawRadarTerrain != 0 {
                                //radar terrain
                                *WPtr = Val
                            } else { *WPtr = colBlack } //colGrey;
                            WPtr = WPtr.offset(1);
                            d = d.wrapping_add(1)
                        }
                        Ptr = Ptr.offset(Modulus as isize);
                        c = c.wrapping_add(1)
                    }
                } else {
                    Ptr =
                        Scr.offset(j as
                                       isize).offset((i as
                                                          libc::c_uint).wrapping_mul(Modulus)
                                                         as isize);
                    c = 0 as libc::c_int as UDWORD;
                    while c < SizeV as libc::c_uint {
                        WPtr = Ptr;
                        d = 0 as libc::c_int as UDWORD;
                        while d < SizeH as libc::c_uint {
                            *WPtr = colBlack;
                            WPtr = WPtr.offset(1);
                            d = d.wrapping_add(1)
                        }
                        Ptr = Ptr.offset(Modulus as isize);
                        c = c.wrapping_add(1)
                    }
                }
                WTile = WTile.offset(1);
                j += SizeH as libc::c_int
            }
            psTile = psTile.offset(mapWidth as isize);
            i += SizeV as libc::c_int
        }
    };
}
// Draw the droids and structure positions on the radar.
//
unsafe extern "C" fn DrawRadarObjects(mut screen: *mut UBYTE,
                                      mut Modulus: UDWORD,
                                      mut boxSizeH: UWORD,
                                      mut boxSizeV: UWORD) {
    let mut c: SDWORD = 0;
    let mut d: SDWORD = 0;
    let mut clan: UBYTE = 0;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psProxDisp: *mut PROXIMITY_DISPLAY = 0 as *mut PROXIMITY_DISPLAY;
    let mut psViewProx: *mut VIEW_PROXIMITY = 0 as *mut VIEW_PROXIMITY;
    let mut Ptr: *mut UBYTE = 0 as *mut UBYTE;
    let mut WPtr: *mut UBYTE = 0 as *mut UBYTE;
    let mut SizeH: SDWORD = 0;
    let mut SizeV: SDWORD = 0;
    let mut bw: SDWORD = 0;
    let mut bh: SDWORD = 0;
    let mut SSizeH: SDWORD = 0;
    let mut SSizeV: SDWORD = 0;
    let mut SweepPos: SDWORD = 0;
    let mut VisWidth: SDWORD = 0;
    let mut VisHeight: SDWORD = 0;
    let mut OffsetX: SDWORD = 0;
    let mut OffsetY: SDWORD = 0;
    let mut playerCol: UBYTE = 0;
    let mut col: UBYTE = 0;
    let mut flashCol: UBYTE = 0;
    let mut camNum: UBYTE = 0;
    SizeH = boxSizeH as SDWORD;
    SizeV = boxSizeV as SDWORD;
    VisWidth = RadVisWidth;
    VisHeight = RadVisHeight;
    //	SweepPos = sweep - RadarOffsetY;
    OffsetX = RadarOffsetX;
    OffsetY = RadarOffsetY;
    SweepPos = sweep.wrapping_sub(RadarOffsetY as libc::c_uint) as SDWORD;
    if SweepPos < 0 as libc::c_int || SweepPos >= RadVisHeight { return }
    camNum =
        getCampaignNumber().wrapping_sub(1 as libc::c_int as libc::c_uint) as
            UBYTE;
    /* Show droids on map - go through all players */
    clan = 0 as libc::c_int as UBYTE;
    while (clan as libc::c_int) < 8 as libc::c_int {
        //draw enemies in red, allies in yellow, if bEnemyAllyRadarColor is TRUE
        if bEnemyAllyRadarColor != 0 {
            if clan as libc::c_uint == selectedPlayer {
                playerCol = colRadarMe
                // grey
            } else {
                //enemy or ally (red or yellow)
                playerCol =
                    if aiCheckAlliances(selectedPlayer, clan as UDWORD) != 0 {
                        colRadarAlly as libc::c_int
                    } else { colRadarEnemy as libc::c_int } as UBYTE
            }
        } else {
            //original 8-color mode
            playerCol =
                clanColours[camNum as
                                usize][getPlayerColour(clan as UDWORD) as
                                           usize] as UBYTE
        }
        flashCol =
            flashColours[camNum as
                             usize][getPlayerColour(clan as UDWORD) as usize]
                as UBYTE;
        /* Go through all droids */
        psDroid = apsDroidLists[clan as usize];
        while !psDroid.is_null() {
            if (*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0
                   || godMode != 0 ||
                   bMultiPlayer != 0 &&
                       game.type_0 as libc::c_int == 13 as libc::c_int &&
                       aiCheckAlliances(selectedPlayer,
                                        (*psDroid).player as UDWORD) != 0 {
                x =
                    (*psDroid).x as libc::c_int / 128 as libc::c_int -
                        RadarScrollX;
                y =
                    (*psDroid).y as libc::c_int / 128 as libc::c_int -
                        RadarScrollY;
                x -= RadarMapOriginX;
                y -= RadarMapOriginY;
                x *= boxSizeH as libc::c_int;
                y *= boxSizeV as libc::c_int;
                if 1 as libc::c_int != 0 || RadarRedraw != 0 {
                    if x < VisWidth && y < VisHeight && x >= 0 as libc::c_int
                           && y >= 0 as libc::c_int {
                        Ptr =
                            screen.offset(x as
                                              isize).offset((y as
                                                                 libc::c_uint).wrapping_mul(Modulus)
                                                                as
                                                                isize).offset(OffsetX
                                                                                  as
                                                                                  isize).offset((OffsetY
                                                                                                     as
                                                                                                     libc::c_uint).wrapping_mul(Modulus)
                                                                                                    as
                                                                                                    isize);
                        //
                        if clan as libc::c_uint == selectedPlayer &&
                               gameTime.wrapping_sub((*psDroid).timeLastHit) <
                                   (1000 as libc::c_int * 2 as libc::c_int) as
                                       libc::c_uint {
                            /*
							if(gameTime%250<125)
							{
								col = playerCol;
							}
							else
							{
								col = 165;//COL_RED;
							}
							*/
                            col = flashCol
                        } else { col = playerCol }
                        c = 0 as libc::c_int;
                        while c < SizeV {
                            WPtr = Ptr;
                            d = 0 as libc::c_int;
                            while d < SizeH {
                                *WPtr = col;
                                WPtr = WPtr.offset(1);
                                d += 1
                            }
                            Ptr = Ptr.offset(Modulus as isize);
                            c += 1
                        }
                    }
                }
            }
            psDroid = (*psDroid).psNext
        }
        clan = clan.wrapping_add(1)
    }
    /* Do the same for structures */
    clan = 0 as libc::c_int as UBYTE;
    while (clan as libc::c_int) < 8 as libc::c_int {
        //draw enemies in red, allies in yellow, if bEnemyAllyRadarColor is TRUE
        if bEnemyAllyRadarColor != 0 {
            if clan as libc::c_uint == selectedPlayer {
                playerCol = colRadarMe
                //grey
            } else {
                //enemy or ally (red or yellow)
                playerCol =
                    if aiCheckAlliances(selectedPlayer, clan as UDWORD) != 0 {
                        colRadarAlly as libc::c_int
                    } else { colRadarEnemy as libc::c_int } as UBYTE
            }
        } else {
            //original 8-color mode 
            playerCol =
                clanColours[camNum as
                                usize][getPlayerColour(clan as UDWORD) as
                                           usize] as UBYTE
        }
        flashCol =
            flashColours[camNum as
                             usize][getPlayerColour(clan as UDWORD) as usize]
                as UBYTE;
        /* Go through all structures */
        psStruct = apsStructLists[clan as usize];
        while !psStruct.is_null() {
            if (*psStruct).visible[selectedPlayer as usize] as libc::c_int !=
                   0 || godMode != 0 ||
                   bMultiPlayer != 0 &&
                       game.type_0 as libc::c_int == 13 as libc::c_int &&
                       aiCheckAlliances(selectedPlayer,
                                        (*psStruct).player as UDWORD) != 0 {
                x =
                    (*psStruct).x as libc::c_int / 128 as libc::c_int -
                        RadarScrollX;
                y =
                    (*psStruct).y as libc::c_int / 128 as libc::c_int -
                        RadarScrollY;
                x -= RadarMapOriginX;
                y -= RadarMapOriginY;
                x *= boxSizeH as libc::c_int;
                y *= boxSizeV as libc::c_int;
                //				}
                bw =
                    (*(*psStruct).pStructureType).baseWidth as UWORD as
                        SDWORD;
                bh =
                    (*(*psStruct).pStructureType).baseBreadth as UWORD as
                        SDWORD;
                x -= bw >> 1 as libc::c_int;
                y -= bh >> 1 as libc::c_int;
                x = x & !(boxSizeH as libc::c_int - 1 as libc::c_int);
                y = y & !(boxSizeV as libc::c_int - 1 as libc::c_int);
                SSizeH = boxSizeH as SWORD as libc::c_int * bh;
                SSizeV = boxSizeV as SWORD as libc::c_int * bw;
                if x < 0 as libc::c_int { SSizeH += x; x = 0 as libc::c_int }
                if y < 0 as libc::c_int { SSizeV += y; y = 0 as libc::c_int }
                if x + SSizeH > VisWidth { SSizeH -= x + SSizeH - VisWidth }
                if y + SSizeV > VisHeight { SSizeV -= y + SSizeV - VisHeight }
                if SSizeV > 0 as libc::c_int && SSizeH > 0 as libc::c_int {
                    Ptr =
                        screen.offset(x as
                                          isize).offset((y as
                                                             libc::c_uint).wrapping_mul(Modulus)
                                                            as
                                                            isize).offset(OffsetX
                                                                              as
                                                                              isize).offset((OffsetY
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_mul(Modulus)
                                                                                                as
                                                                                                isize);
                    if clan as libc::c_uint == selectedPlayer &&
                           gameTime.wrapping_sub((*psStruct).timeLastHit) <
                               (1000 as libc::c_int * 2 as libc::c_int) as
                                   libc::c_uint {
                        // Get structures tile size.
                        // Need to offset for the structures top left corner.
                        //				if( ((y >= sweep) && (y <= sweep+(bh*boxSizeV))) || (RadarRedraw) ) {
                        // Clip the structure box.
                        // And draw it.
                        /*
							if(gameTime%250<125)
							{
								col = playerCol;
							}
							else
							{
								col = 165;//COL_RED;
						   	}
							*/
                        col = flashCol
                    } else { col = playerCol }
                    c = 0 as libc::c_int;
                    while c < SSizeV {
                        WPtr = Ptr;
                        d = 0 as libc::c_int;
                        while d < SSizeH {
                            *WPtr = col;
                            WPtr = WPtr.offset(1);
                            d += 1
                        }
                        Ptr = Ptr.offset(Modulus as isize);
                        c += 1
                    }
                    //store the radar coords
                    (*psStruct).radarX = (x + OffsetX) as UWORD;
                    (*psStruct).radarY = (y + OffsetY) as UWORD
                }
            }
            psStruct = (*psStruct).psNext
        }
        clan = clan.wrapping_add(1)
    }
    //now set up coords for Proximity Messages - but only for selectedPlayer
    psProxDisp = apsProxDisp[selectedPlayer as usize];
    while !psProxDisp.is_null() {
        if (*psProxDisp).type_0 as libc::c_uint ==
               POS_PROXDATA as libc::c_int as libc::c_uint {
            psViewProx =
                (*((*(*psProxDisp).psMessage).pViewData as
                       *mut VIEWDATA)).pData as *mut VIEW_PROXIMITY;
            x =
                (*psViewProx).x.wrapping_div(128 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(RadarScrollX
                                                                                as
                                                                                libc::c_uint)
                    as SDWORD;
            y =
                (*psViewProx).y.wrapping_div(128 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(RadarScrollY
                                                                                as
                                                                                libc::c_uint)
                    as SDWORD
        } else if (*psProxDisp).type_0 as libc::c_uint ==
                      POS_PROXOBJ as libc::c_int as libc::c_uint {
            x =
                (*((*(*psProxDisp).psMessage).pViewData as
                       *mut BASE_OBJECT)).x as libc::c_int /
                    128 as libc::c_int - RadarScrollX;
            y =
                (*((*(*psProxDisp).psMessage).pViewData as
                       *mut BASE_OBJECT)).y as libc::c_int /
                    128 as libc::c_int - RadarScrollY
        }
        x -= RadarMapOriginX;
        y -= RadarMapOriginY;
        x *= boxSizeH as libc::c_int;
        y *= boxSizeV as libc::c_int;
        (*psProxDisp).radarX = 0 as libc::c_int as UDWORD;
        (*psProxDisp).radarY = 0 as libc::c_int as UDWORD;
        if x < VisWidth && y < VisHeight && x >= 0 as libc::c_int &&
               y >= 0 as libc::c_int {
            //store the coords
            (*psProxDisp).radarX = (x + OffsetX) as UDWORD;
            (*psProxDisp).radarY = (y + OffsetY) as UDWORD
        }
        psProxDisp = (*psProxDisp).psNext
    };
}
// Rotate an array of 2d vectors about a given angle, also translates them after rotating.
//
#[no_mangle]
pub unsafe extern "C" fn RotateVector2D(mut Vector: *mut iVector,
                                        mut TVector: *mut iVector,
                                        mut Pos: *mut iVector,
                                        mut Angle: libc::c_int,
                                        mut Count: libc::c_int) {
    let mut Cos: libc::c_int =
        *aSinTable.as_mut_ptr().offset(((Angle as uint16 as libc::c_int >>
                                             4 as libc::c_int) +
                                            1024 as libc::c_int) as isize);
    let mut Sin: libc::c_int =
        *aSinTable.as_mut_ptr().offset((Angle as uint16 as libc::c_int >>
                                            4 as libc::c_int) as isize);
    let mut ox: libc::c_int = 0 as libc::c_int;
    let mut oy: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut Vec: *mut iVector = Vector;
    let mut TVec: *mut iVector = TVector;
    if !Pos.is_null() { ox = (*Pos).x; oy = (*Pos).y }
    i = 0 as libc::c_int;
    while i < Count {
        (*TVec).x =
            ((*Vec).x * Cos + (*Vec).y * Sin >> 12 as libc::c_int) + ox;
        (*TVec).y =
            ((*Vec).y * Cos - (*Vec).x * Sin >> 12 as libc::c_int) + oy;
        Vec = Vec.offset(1);
        TVec = TVec.offset(1);
        i += 1
    };
}
// Returns the world position which corresponds to the center of the radar view rectangle.
//
#[no_mangle]
pub unsafe extern "C" fn GetRadarPlayerPos(mut XPos: *mut UDWORD,
                                           mut YPos: *mut UDWORD) {
    *XPos =
        (player.p.x as
             libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint));
    *YPos =
        (player.p.z as
             libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint).wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn getDistanceAdjust() -> SDWORD {
    let mut origDistance: UDWORD = 0;
    let mut dif: SDWORD = 0;
    origDistance = 4500 as libc::c_int as UDWORD;
    dif = origDistance.wrapping_sub(distance) as SDWORD;
    if dif < 0 as libc::c_int { dif = 0 as libc::c_int }
    dif /= 100 as libc::c_int;
    return dif;
}
#[no_mangle]
pub unsafe extern "C" fn getLengthAdjust() -> SDWORD {
    let mut pitch: SDWORD = 0;
    let mut lookingDown: UDWORD = 0;
    let mut lookingFar: UDWORD = 0;
    let mut dif: SDWORD = 0;
    pitch =
        360 as libc::c_int -
            player.r.x / (65536 as libc::c_int / 360 as libc::c_int);
    // Max at
    lookingDown = (0 as libc::c_int - -(50 as libc::c_int)) as UDWORD;
    lookingFar = (0 as libc::c_int - -(14 as libc::c_int)) as UDWORD;
    dif = (pitch as libc::c_uint).wrapping_sub(lookingFar) as SDWORD;
    if dif < 0 as libc::c_int { dif = 0 as libc::c_int }
    if dif as libc::c_uint > lookingDown.wrapping_sub(lookingFar) {
        dif = lookingDown.wrapping_sub(lookingFar) as SDWORD
    }
    return dif / 2 as libc::c_int;
}
/* Draws a Myth/FF7 style viewing window */
#[no_mangle]
pub unsafe extern "C" fn drawViewingWindow(mut x: UDWORD, mut y: UDWORD,
                                           mut boxSizeH: UDWORD,
                                           mut boxSizeV: UDWORD) {
    let mut v: [iVector; 4] = [iVector{x: 0, y: 0, z: 0,}; 4];
    let mut tv: [iVector; 4] = [iVector{x: 0, y: 0, z: 0,}; 4];
    let mut centre: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut shortX: UDWORD = 0;
    let mut longX: UDWORD = 0;
    let mut yDrop: UDWORD = 0;
    let mut yDropVar: UDWORD = 0;
    let mut dif: SDWORD = getDistanceAdjust();
    let mut dif2: SDWORD = getLengthAdjust();
    let mut colour: UDWORD = 0 as libc::c_int as UDWORD;
    let mut camNumber: UDWORD = 0;
    shortX =
        visibleXTiles.wrapping_div(4 as libc::c_int as
                                       libc::c_uint).wrapping_sub((dif /
                                                                       3 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint).wrapping_mul(boxSizeH);
    longX =
        visibleXTiles.wrapping_div(2 as libc::c_int as
                                       libc::c_uint).wrapping_sub((dif /
                                                                       2 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint).wrapping_mul(boxSizeH);
    yDropVar =
        visibleYTiles.wrapping_div(2 as libc::c_int as
                                       libc::c_uint).wrapping_sub((dif2 /
                                                                       3 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint).wrapping_mul(boxSizeV);
    yDrop =
        visibleYTiles.wrapping_div(2 as libc::c_int as
                                       libc::c_uint).wrapping_sub((dif2 /
                                                                       3 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint).wrapping_mul(boxSizeV);
    v[0 as libc::c_int as usize].x = longX.wrapping_neg() as int32;
    v[0 as libc::c_int as usize].y = yDropVar.wrapping_neg() as int32;
    v[1 as libc::c_int as usize].x = longX as int32;
    v[1 as libc::c_int as usize].y = yDropVar.wrapping_neg() as int32;
    v[2 as libc::c_int as usize].x = shortX.wrapping_neg() as int32;
    v[2 as libc::c_int as usize].y = yDrop as int32;
    v[3 as libc::c_int as usize].x = shortX as int32;
    v[3 as libc::c_int as usize].y = yDrop as int32;
    centre.x =
        ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_add(320
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint).wrapping_add(6
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                     libc::c_uint)).wrapping_add(x).wrapping_add(visibleXTiles.wrapping_mul(boxSizeH).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                                                                       libc::c_uint))
            as int32;
    centre.y =
        (324 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)).wrapping_add(1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_add(y).wrapping_add(visibleYTiles.wrapping_mul(boxSizeV).wrapping_div(2
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_uint))
            as int32;
    RotateVector2D(v.as_mut_ptr(), tv.as_mut_ptr(), &mut centre, player.r.y,
                   4 as libc::c_int);
    //	iV_Line(tv[0].x,tv[0].y,tv[1].x,tv[1].y,colWhite);
  //	iV_Line(tv[1].x,tv[1].y,tv[3].x,tv[3].y,colWhite);
  //	iV_Line(tv[3].x,tv[3].y,tv[2].x,tv[2].y,colWhite);
  //	iV_Line(tv[2].x,tv[2].y,tv[0].x,tv[0].y,colWhite);
    camNumber = getCampaignNumber(); //white
    match camNumber {
        1 => { colour = 0x3fffffff as libc::c_int as UDWORD }
        2 => { colour = 0x3fffffff as libc::c_int as UDWORD }
        3 => { //white
            colour = 0x3f3fff3f as libc::c_int as UDWORD
        }
        _ => { }
    } //green?
    /* Send the four points to the draw routine and the clip box params */
    pie_DrawViewingWindow(tv.as_mut_ptr(),
                          ((23 as libc::c_int + 132 as libc::c_int +
                                6 as libc::c_int) as
                               libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint).wrapping_div(2
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)).wrapping_add(320
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint).wrapping_add(6
                                                                                                                                                                                                   as
                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                   as
                                                                                                                                                                                                   libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                       libc::c_uint)),
                          (324 as libc::c_int as
                               libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint)).wrapping_add(1
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint),
                          ((23 as libc::c_int + 132 as libc::c_int +
                                6 as libc::c_int) as
                               libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_uint).wrapping_div(2
                                                                                                                                    as
                                                                                                                                    libc::c_int
                                                                                                                                    as
                                                                                                                                    libc::c_uint)).wrapping_add(320
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint).wrapping_add(6
                                                                                                                                                                                                   as
                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                   as
                                                                                                                                                                                                   libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  libc::c_int
                                                                                                                                                                                                                                  as
                                                                                                                                                                                                                                  libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                       libc::c_uint)).wrapping_add(128
                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                                                                                                       libc::c_uint),
                          (324 as libc::c_int as
                               libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint)).wrapping_add(1
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint).wrapping_add(128
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_int
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_uint),
                          colour);
}
unsafe extern "C" fn DrawRadarExtras(mut boxSizeH: UWORD,
                                     mut boxSizeV: UWORD) {
    //	UDWORD	viewX,viewY;
    let mut viewX: SDWORD = 0;
    let mut viewY: SDWORD = 0;
    let mut offsetX: SDWORD = 0;
    let mut offsetY: SDWORD = 0;
    viewX =
        (player.p.x / 128 as libc::c_int - RadarScrollX - RadarMapOriginX) *
            boxSizeH as libc::c_int;
    offsetY = viewX;
    offsetX = offsetY;
    viewY =
        (player.p.z / 128 as libc::c_int - RadarScrollY - RadarMapOriginY) *
            boxSizeV as libc::c_int;
    viewX += RadarOffsetX;
    viewY += RadarOffsetY;
    viewX = viewX & !(boxSizeH as libc::c_int - 1 as libc::c_int);
    viewY = viewY & !(boxSizeV as libc::c_int - 1 as libc::c_int);
    //don't update the strobe whilst the game is paused
    (gameUpdatePaused()) == 0;
    drawViewingWindow(viewX as UDWORD, viewY as UDWORD, boxSizeH as UDWORD,
                      boxSizeV as UDWORD);
    DrawEnableLocks(0 as libc::c_int);
    RenderWindowFrame(&mut FrameRadar,
                      ((23 as libc::c_int + 132 as libc::c_int +
                            6 as libc::c_int) as
                           libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_div(2
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint)).wrapping_add(320
                                                                                                                                                                as
                                                                                                                                                                libc::c_int
                                                                                                                                                                as
                                                                                                                                                                libc::c_uint).wrapping_add(6
                                                                                                                                                                                               as
                                                                                                                                                                                               libc::c_int
                                                                                                                                                                                               as
                                                                                                                                                                                               libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              libc::c_int
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                   libc::c_uint)).wrapping_sub(1
                                                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                                                                                                                                   libc::c_uint),
                      (324 as libc::c_int as
                           libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  libc::c_uint)).wrapping_add(1
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint).wrapping_sub(1
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_int
                                                                                                                                                                 as
                                                                                                                                                                 libc::c_uint),
                      (128 as libc::c_int + 2 as libc::c_int) as UDWORD,
                      (128 as libc::c_int + 2 as libc::c_int) as UDWORD);
    DrawEnableLocks(1 as libc::c_int);
    // Draw the radar border.
}
// Does a screen coordinate lie within the radar area?
//
#[no_mangle]
pub unsafe extern "C" fn CoordInRadar(mut x: libc::c_int, mut y: libc::c_int)
 -> BOOL {
    if x as libc::c_uint >=
           ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
                libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint)).wrapping_add(320
                                                                                                                                                     as
                                                                                                                                                     libc::c_int
                                                                                                                                                     as
                                                                                                                                                     libc::c_uint).wrapping_add(6
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_int
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                         libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                                        libc::c_uint)).wrapping_sub(1
                                                                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                                                                                                                                                                                        as
                                                                                                                                                                                                                                                                                                                                                        libc::c_uint)
           &&
           (x as libc::c_uint) <
               ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_uint).wrapping_div(2
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_uint)).wrapping_add(320
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint).wrapping_add(6
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       libc::c_int
                                                                                                                                                                                                                       as
                                                                                                                                                                                                                       libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                             libc::c_int
                                                                                                                                                                                                                                                                                             as
                                                                                                                                                                                                                                                                                             libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                            libc::c_uint)).wrapping_add(128
                                                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                                                            libc::c_int
                                                                                                                                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                                                                                                                                            libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                           libc::c_int
                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                           libc::c_uint)
           &&
           y as libc::c_uint >=
               (324 as libc::c_int as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)).wrapping_add(1
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint).wrapping_sub(1
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint)
           &&
           (y as libc::c_uint) <
               (324 as libc::c_int as
                    libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)).wrapping_add(1
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint).wrapping_add(128
                                                                                                                                                          as
                                                                                                                                                          libc::c_int
                                                                                                                                                          as
                                                                                                                                                          libc::c_uint).wrapping_add(1
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         libc::c_uint)
       {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn calcRadarColour(mut tileBitmap: *mut UBYTE,
                                         mut tileNumber: UDWORD) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut penNumber: UBYTE = 0;
    let mut fRed: UBYTE = 0;
    let mut fGreen: UBYTE = 0;
    let mut fBlue: UBYTE = 0;
    let mut red: UBYTE = 0;
    let mut green: UBYTE = 0;
    let mut blue: UBYTE = 0;
    let mut tRed: UDWORD = 0;
    let mut tGreen: UDWORD = 0;
    let mut tBlue: UDWORD = 0;
    let mut psPalette: *mut iColour = 0 as *mut iColour;
    tBlue = 0 as libc::c_int as UDWORD;
    tGreen = tBlue;
    tRed = tGreen;
    //this routine only checks 64 pixels
	//offset half a step at the start
    tileBitmap =
        tileBitmap.offset((128 as libc::c_int /
                               (2 as libc::c_int * 8 as libc::c_int) *
                               128 as libc::c_int) as isize);
    tileBitmap =
        tileBitmap.offset((128 as libc::c_int /
                               (2 as libc::c_int * 8 as libc::c_int)) as
                              isize);
    psPalette = pie_GetGamePal();
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            /* Get pixel colour index */
            penNumber =
                *tileBitmap.offset(j.wrapping_mul((128 as libc::c_int /
                                                       8 as libc::c_int) as
                                                      libc::c_uint) as
                                       isize); //stepping across a few steps
            /* Get the r,g,b components */
            red = (*psPalette.offset(penNumber as isize)).r;
            green = (*psPalette.offset(penNumber as isize)).g;
            blue = (*psPalette.offset(penNumber as isize)).b;
            /* Add them to totals */
            tRed =
                (tRed as libc::c_uint).wrapping_add(red as libc::c_uint) as
                    UDWORD as UDWORD;
            tGreen =
                (tGreen as libc::c_uint).wrapping_add(green as libc::c_uint)
                    as UDWORD as UDWORD;
            tBlue =
                (tBlue as libc::c_uint).wrapping_add(blue as libc::c_uint) as
                    UDWORD as UDWORD;
            j = j.wrapping_add(1)
        }
        //step down a few lines
        tileBitmap =
            tileBitmap.offset((128 as libc::c_int / 8 as libc::c_int *
                                   128 as libc::c_int) as isize);
        i = i.wrapping_add(1)
    }
    /* Get average of each component */
    fRed =
        tRed.wrapping_div((8 as libc::c_int * 8 as libc::c_int) as
                              libc::c_uint) as UBYTE;
    fGreen =
        tGreen.wrapping_div((8 as libc::c_int * 8 as libc::c_int) as
                                libc::c_uint) as UBYTE;
    fBlue =
        tBlue.wrapping_div((8 as libc::c_int * 8 as libc::c_int) as
                               libc::c_uint) as UBYTE;
    tileColours[tileNumber as usize] =
        pal_GetNearestColour(fRed, fGreen, fBlue);
}
