use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* Lookup trig functions */
    #[no_mangle]
    fn trigSin(angle: SDWORD) -> FRACT;
    #[no_mangle]
    fn trigCos(angle: SDWORD) -> FRACT;
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    // Get a droid's name.
    #[no_mangle]
    fn droidGetName(psDroid: *mut DROID) -> *mut STRING;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    //extern POWER_STATS			*asPowerStats;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    fn DrawnInLastFrame(Frame: SDWORD) -> BOOL;
    #[no_mangle]
    fn CheckScrollLimits() -> BOOL;
    #[no_mangle]
    fn getViewDistance() -> UDWORD;
    #[no_mangle]
    fn setViewDistance(dist: UDWORD);
    #[no_mangle]
    static mut player: iView;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn flushConsoleMessages();
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    // Never stops.
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime2: UDWORD;
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    #[no_mangle]
    fn droidOnScreen(psDroid: *mut DROID, tolerance: SDWORD) -> BOOL;
    // print out information about a droid and it's components
    #[no_mangle]
    fn printDroidInfo(psDroid: *mut DROID);
    #[no_mangle]
    fn getDrivingStatus() -> BOOL;
    #[no_mangle]
    fn demoGetStatus() -> BOOL;
    #[no_mangle]
    fn setFindNewTarget();
    // Calculates the maximum height and distance found along a line from any
// point to the edge of the grid
    #[no_mangle]
    fn getBestPitchToEdgeOfGrid(x: UDWORD, y: UDWORD, direction: UDWORD,
                                pitch: *mut SDWORD);
    #[no_mangle]
    fn selNumSelected(player_0: UDWORD) -> UDWORD;
    #[no_mangle]
    static mut bTrackingTransporter: BOOL;
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
pub type _key_code = libc::c_uint;
pub const KEY_KPENTER: _key_code = 271;
pub const KEY_DELETE: _key_code = 127;
pub const KEY_INSERT: _key_code = 277;
pub const KEY_PAGEDOWN: _key_code = 281;
pub const KEY_DOWNARROW: _key_code = 274;
pub const KEY_END: _key_code = 279;
pub const KEY_RIGHTARROW: _key_code = 275;
pub const KEY_LEFTARROW: _key_code = 276;
pub const KEY_PAGEUP: _key_code = 280;
pub const KEY_UPARROW: _key_code = 273;
pub const KEY_HOME: _key_code = 278;
pub const KEY_RALT: _key_code = 307;
pub const KEY_KP_BACKSLASH: _key_code = 267;
pub const KEY_RCTRL: _key_code = 305;
pub const KEY_F12: _key_code = 293;
pub const KEY_F11: _key_code = 292;
pub const KEY_KP_FULLSTOP: _key_code = 266;
pub const KEY_KP_0: _key_code = 256;
pub const KEY_KP_3: _key_code = 259;
pub const KEY_KP_2: _key_code = 258;
pub const KEY_KP_1: _key_code = 257;
pub const KEY_KP_PLUS: _key_code = 270;
pub const KEY_KP_6: _key_code = 262;
pub const KEY_KP_5: _key_code = 261;
pub const KEY_KP_4: _key_code = 260;
pub const KEY_KP_MINUS: _key_code = 269;
pub const KEY_KP_9: _key_code = 265;
pub const KEY_KP_8: _key_code = 264;
pub const KEY_KP_7: _key_code = 263;
pub const KEY_SCROLLLOCK: _key_code = 302;
pub const KEY_NUMLOCK: _key_code = 300;
pub const KEY_F10: _key_code = 291;
pub const KEY_F9: _key_code = 290;
pub const KEY_F8: _key_code = 289;
pub const KEY_F7: _key_code = 288;
pub const KEY_F6: _key_code = 287;
pub const KEY_F5: _key_code = 286;
pub const KEY_F4: _key_code = 285;
pub const KEY_F3: _key_code = 284;
pub const KEY_F2: _key_code = 283;
pub const KEY_F1: _key_code = 282;
pub const KEY_CAPSLOCK: _key_code = 301;
pub const KEY_SPACE: _key_code = 32;
pub const KEY_LALT: _key_code = 308;
pub const KEY_KP_STAR: _key_code = 268;
pub const KEY_RSHIFT: _key_code = 303;
pub const KEY_FORWARDSLASH: _key_code = 47;
pub const KEY_FULLSTOP: _key_code = 46;
pub const KEY_COMMA: _key_code = 44;
pub const KEY_M: _key_code = 109;
pub const KEY_N: _key_code = 110;
pub const KEY_B: _key_code = 98;
pub const KEY_V: _key_code = 118;
pub const KEY_C: _key_code = 99;
pub const KEY_X: _key_code = 120;
pub const KEY_Z: _key_code = 122;
pub const KEY_BACKSLASH: _key_code = 92;
pub const KEY_LSHIFT: _key_code = 304;
pub const KEY_BACKQUOTE: _key_code = 96;
pub const KEY_QUOTE: _key_code = 39;
pub const KEY_SEMICOLON: _key_code = 59;
pub const KEY_L: _key_code = 108;
pub const KEY_K: _key_code = 107;
pub const KEY_J: _key_code = 106;
pub const KEY_H: _key_code = 104;
pub const KEY_G: _key_code = 103;
pub const KEY_F: _key_code = 102;
pub const KEY_D: _key_code = 100;
pub const KEY_S: _key_code = 115;
pub const KEY_A: _key_code = 97;
pub const KEY_LCTRL: _key_code = 306;
pub const KEY_RETURN: _key_code = 13;
pub const KEY_RBRACE: _key_code = 93;
pub const KEY_LBRACE: _key_code = 91;
pub const KEY_P: _key_code = 112;
pub const KEY_O: _key_code = 111;
pub const KEY_I: _key_code = 105;
pub const KEY_U: _key_code = 117;
pub const KEY_Y: _key_code = 121;
pub const KEY_T: _key_code = 116;
pub const KEY_R: _key_code = 114;
pub const KEY_E: _key_code = 101;
pub const KEY_W: _key_code = 119;
pub const KEY_Q: _key_code = 113;
pub const KEY_TAB: _key_code = 9;
pub const KEY_BACKSPACE: _key_code = 8;
pub const KEY_EQUALS: _key_code = 61;
pub const KEY_MINUS: _key_code = 45;
pub const KEY_0: _key_code = 48;
pub const KEY_9: _key_code = 57;
pub const KEY_8: _key_code = 56;
pub const KEY_7: _key_code = 55;
pub const KEY_6: _key_code = 54;
pub const KEY_5: _key_code = 53;
pub const KEY_4: _key_code = 52;
pub const KEY_3: _key_code = 51;
pub const KEY_2: _key_code = 50;
pub const KEY_1: _key_code = 49;
pub const KEY_ESC: _key_code = 27;
pub type KEY_CODE = _key_code;
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
pub struct PIEVECTORF {
    pub x: FRACT,
    pub y: FRACT,
    pub z: FRACT,
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
pub type _propulsion_type = libc::c_uint;
pub const NUM_PROP_TYPES: _propulsion_type = 9;
pub const JUMP: _propulsion_type = 8;
pub const HALF_TRACKED: _propulsion_type = 7;
pub const PROPELLOR: _propulsion_type = 6;
pub const LIFT: _propulsion_type = 5;
pub const SKI: _propulsion_type = 4;
pub const HOVER: _propulsion_type = 3;
pub const LEGGED: _propulsion_type = 2;
pub const TRACKED: _propulsion_type = 1;
pub const WHEELED: _propulsion_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_stats {
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
    pub maxSpeed: UDWORD,
    pub propulsionType: UBYTE,
}
pub type PROPULSION_STATS = _propulsion_stats;
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
/* The different tracking states */
pub type C2RustUnnamed = libc::c_uint;
pub const CAM_TRACK_LOCATION: C2RustUnnamed = 5;
pub const CAM_TRACK_OBJECT: C2RustUnnamed = 4;
pub const CAM_RESET: C2RustUnnamed = 3;
pub const CAM_TRACKING: C2RustUnnamed = 2;
pub const CAM_REQUEST: C2RustUnnamed = 1;
pub const CAM_INACTIVE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _warcam {
    pub status: UDWORD,
    pub trackClass: UDWORD,
    pub lastUpdate: UDWORD,
    pub oldView: iView,
    pub acceleration: PIEVECTORF,
    pub velocity: PIEVECTORF,
    pub position: PIEVECTORF,
    pub rotation: PIEVECTORF,
    pub rotVel: PIEVECTORF,
    pub rotAccel: PIEVECTORF,
    pub oldDistance: UDWORD,
    pub target: *mut BASE_OBJECT,
}
/* Storage for old viewnagles etc */
pub type WARCAM = _warcam;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
static mut testAngle: UDWORD = 250 as libc::c_int as UDWORD;
/* Holds all the details of our camera */
static mut trackingCamera: WARCAM =
    WARCAM{status: 0,
           trackClass: 0,
           lastUpdate: 0,
           oldView:
               iView{p: iVector{x: 0, y: 0, z: 0,},
                     r: iVector{x: 0, y: 0, z: 0,},},
           acceleration: PIEVECTORF{x: 0., y: 0., z: 0.,},
           velocity: PIEVECTORF{x: 0., y: 0., z: 0.,},
           position: PIEVECTORF{x: 0., y: 0., z: 0.,},
           rotation: PIEVECTORF{x: 0., y: 0., z: 0.,},
           rotVel: PIEVECTORF{x: 0., y: 0., z: 0.,},
           rotAccel: PIEVECTORF{x: 0., y: 0., z: 0.,},
           oldDistance: 0,
           target: 0 as *const BASE_OBJECT as *mut BASE_OBJECT,};
/* Present rotation for the 3d camera logo */
static mut warCamLogoRotation: SDWORD = 0;
/* The fake target that we track when jumping to a new location on the radar */
static mut radarTarget: BASE_OBJECT =
    BASE_OBJECT{type_0: OBJ_DROID,
                id: 0,
                x: 0,
                y: 0,
                z: 0,
                direction: 0,
                pitch: 0,
                roll: 0,
                psNext: 0 as *const _base_object as *mut _base_object,
                sDisplay:
                    SCREEN_DISP_DATA{imd:
                                         0 as *const iIMDShape as
                                             *mut iIMDShape,
                                     frameNumber: 0,
                                     screenX: 0,
                                     screenY: 0,
                                     screenR: 0,},
                player: 0,
                group: 0,
                selected: 0,
                cluster: 0,
                visible: [0; 8],
                died: 0,
                lastEmission: 0,
                inFire: 0,
                burnStart: 0,
                burnDamage: 0,};
/* Do we trun to face when doing a radar jump? */
static mut bRadarAllign: BOOL = 0;
/* How far we track relative to the droids location - direction matters */
#[no_mangle]
pub static mut camDroidXOffset: SDWORD = 0;
#[no_mangle]
pub static mut camDroidYOffset: SDWORD = 0;
#[no_mangle]
pub static mut presAvAngle: SDWORD = 0 as libc::c_int;
/*	These used to be #defines but they're variable now as it may be necessary
	to allow the player	to customise tracking speed? Jim?
*/
#[no_mangle]
pub static mut accelConstant: FRACT = 0.;
#[no_mangle]
pub static mut velocityConstant: FRACT = 0.;
#[no_mangle]
pub static mut rotAccelConstant: FRACT = 0.;
#[no_mangle]
pub static mut rotVelocityConstant: FRACT = 0.;
/* How much info do you want when tracking a droid - this toggles full stat info */
static mut bFullInfo: BOOL = 0 as libc::c_int;
/* Are we requesting a new track to start that is a radar (location) track? */
static mut bRadarTrackingRequested: BOOL = 0 as libc::c_int;
/* World coordinates for a radar track/jump */
//static  SDWORD	 radarX,radarY;
static mut radarX: FRACT = 0.;
static mut radarY: FRACT = 0.;
/*	Where we were up to (pos and rot) last update - allows us to see whether
	we are sufficently near our target to disable further tracking */
static mut oldPosition: iVector = iVector{x: 0, y: 0, z: 0,};
static mut oldRotation: iVector = iVector{x: 0, y: 0, z: 0,};
/* The fraction of a second that the last game frame took */
static mut fraction: FRACT = 0.;
static mut OldViewValid: BOOL = 0;
/* Externally referenced functions */
//-----------------------------------------------------------------------------------
/* Sets the camera to inactive to begin with */
#[no_mangle]
pub unsafe extern "C" fn initWarCam() {
    /* We're not intitially following anything */
    trackingCamera.status = CAM_INACTIVE as libc::c_int as UDWORD;
    /* Set up the default tracking variables */
    accelConstant =
        64 as libc::c_int as libc::c_float /
            10 as libc::c_int as libc::c_float;
    velocityConstant =
        4 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
    rotAccelConstant =
        4 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
    rotVelocityConstant =
        4 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float;
    /* Logo setup */
    warCamLogoRotation = 0 as libc::c_int;
    /* Offset from droid's world coords */
    camDroidXOffset = -(400 as libc::c_int);
    camDroidYOffset = -(400 as libc::c_int);
    OldViewValid = 0 as libc::c_int;
}
//-----------------------------------------------------------------------------------
// Just turn it off.
//
#[no_mangle]
pub unsafe extern "C" fn CancelWarCam() {
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        if bTrackingTransporter != 0 &&
               (*(trackingCamera.target as *mut DROID)).droidType as
                   libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            return
        }
    }
    trackingCamera.status = CAM_INACTIVE as libc::c_int as UDWORD;
}
/* Updates the camera position/angle along with the object movement */
#[no_mangle]
pub unsafe extern "C" fn processWarCam() -> BOOL {
    let mut foundTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Status: BOOL = 1 as libc::c_int;
    /* Get out if the camera isn't active */
    if trackingCamera.status == CAM_INACTIVE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    /* Calculate fraction of a second for last game frame */
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    /* Ensure that the camera only ever flips state within this routine! */
    match trackingCamera.status {
        1 => {
            /* See if we can find the target to follow */
            foundTarget = camFindTarget();
            if !foundTarget.is_null() && (*foundTarget).died == 0 {
                /* We've got one, so store away info */
                camAllignWithTarget(foundTarget);
                /* We're now into tracking status */
                trackingCamera.status = CAM_TRACKING as libc::c_int as UDWORD;
                /* Inform via console */
                if (*foundTarget).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                    if getWarCamStatus() == 0 {
                        sprintf(ConsoleString.as_mut_ptr(),
                                b"WZ/CAM  - %s\x00" as *const u8 as
                                    *const libc::c_char,
                                droidGetName(foundTarget as *mut DROID));
                        addConsoleMessage(ConsoleString.as_mut_ptr(),
                                          DEFAULT_JUSTIFY);
                    }
                }
            } else {
                /* We've requested a track with no droid selected */
//				addConsoleMessage("Droid-CAM V0.1 ERROR - No targets(s) selected",DEFAULT_JUSTIFY);
                trackingCamera.status = CAM_INACTIVE as libc::c_int as UDWORD
            }
        }
        2 => {
            /* Track the droid unless routine comes back false */
            if camTrackCamera() == 0 {
                /*
					Camera track came back false, either because droid died or is
					no longer selected, so reset to old values
				*/
                foundTarget = camFindTarget();
                if !foundTarget.is_null() && (*foundTarget).died == 0 {
                    trackingCamera.status =
                        CAM_REQUEST as libc::c_int as UDWORD
                } else {
                    trackingCamera.status = CAM_RESET as libc::c_int as UDWORD
                }
            }
            processLeaderSelection();
        }
        3 => {
            /* Reset camera to pre-droid tracking status */
            if trackingCamera.target.is_null() ||
                   (*trackingCamera.target).type_0 as libc::c_uint !=
                       OBJ_TARGET as libc::c_int as libc::c_uint {
                camSwitchOff();
            }
            /* Switch to inactive mode */
            trackingCamera.status = CAM_INACTIVE as libc::c_int as UDWORD;
            //			addConsoleMessage("Droid-CAM V0.1 Disabled",DEFAULT_JUSTIFY);
            Status = 0 as libc::c_int
        }
        _ => {
            debug(LOG_ERROR,
                  b"Weirdy status for tracking Camera\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    /* TBR
	flushConsoleMessages();
	CONPRINTF(ConsoleString,(ConsoleString,"Acceleration of movement constant : %.2f",accelConstant));
	CONPRINTF(ConsoleString,(ConsoleString,"Velocity of movement constant : %.2f",velocityConstant));
	CONPRINTF(ConsoleString,(ConsoleString,"Acceleration of rotation constant : %.2f",rotAccelConstant));
	CONPRINTF(ConsoleString,(ConsoleString,"Velocity of rotation constant : %.2f",rotVelocityConstant));
	CONPRINTF(ConsoleString,(ConsoleString,"Tracking droid direction : %d",trackingCamera.droid->direction));
	CONPRINTF(ConsoleString,(ConsoleString,"Tracking droid pitch : %d",trackingCamera.droid->pitch));
	CONPRINTF(ConsoleString,(ConsoleString,"Tracking droid roll : %d",trackingCamera.droid->roll));
	CONPRINTF(ConsoleString,(ConsoleString,"Tracking droid height (z) : %d",trackingCamera.droid->z));
	CONPRINTF(ConsoleString,(ConsoleString,"position.p.y : %d",player.p.y));
	*/
    return Status;
}
//-----------------------------------------------------------------------------------
/* Flips states for camera active */
#[no_mangle]
pub unsafe extern "C" fn setWarCamActive(mut status: BOOL) {
    debug(LOG_NEVER,
          b"setWarCamActive(%d)\n\x00" as *const u8 as *const libc::c_char,
          status);
    /* We're trying to switch it on */
    if status == 1 as libc::c_int {
        /* If it's not inactive then it's already in use - so return */
		/* We're tracking a droid */
        if trackingCamera.status !=
               CAM_INACTIVE as libc::c_int as libc::c_uint {
            if bRadarTrackingRequested != 0 {
                trackingCamera.status = CAM_REQUEST as libc::c_int as UDWORD
            } else { return }
        } else {
            /* Otherwise request the camera to track */
            trackingCamera.status = CAM_REQUEST as libc::c_int as UDWORD
        }
    } else if trackingCamera.status ==
                  CAM_INACTIVE as libc::c_int as libc::c_uint {
        return
    } else {
        /* We trying to switch off */
        /* Is it already off? */
        /* Attempt to set to normal */
        trackingCamera.status = CAM_RESET as libc::c_int as UDWORD
    };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn camFindDroidTarget() -> *mut BASE_OBJECT {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 {
            /* Return the first one found */
            return psDroid as *mut BASE_OBJECT
        }
        psDroid = (*psDroid).psNext
    }
    //printf("camFindUnitTarget : NOT FOUND\n");
	/* We didn't find one */
    return 0 as *mut BASE_OBJECT;
}
/* Attempts to find the target for the camera to track */
#[no_mangle]
pub unsafe extern "C" fn camFindTarget() -> *mut BASE_OBJECT {
    /*	See if we can find a selected droid. If there's more than one
		droid selected for the present player, then we track the oldest
		one. */
    if bRadarTrackingRequested != 0 {
        setUpRadarTarget(radarX as SDWORD, radarY as SDWORD);
        bRadarTrackingRequested = 0 as libc::c_int;
        return &mut radarTarget
    }
    return camFindDroidTarget();
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getTestAngle() -> UDWORD { return testAngle; }
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn updateTestAngle() {
    testAngle =
        (testAngle as
             libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    if testAngle >= 360 as libc::c_int as libc::c_uint {
        testAngle = 0 as libc::c_int as UDWORD
    };
}
//-----------------------------------------------------------------------------------
/* Stores away old viewangle info and sets up new distance and angles */
#[no_mangle]
pub unsafe extern "C" fn camAllignWithTarget(mut psTarget: *mut BASE_OBJECT) {
    /* Store away the target */
    trackingCamera.target = psTarget;
    /* Save away all the view angles */
    trackingCamera.rotation.x = player.r.x as FRACT;
    trackingCamera.oldView.r.x = trackingCamera.rotation.x as int32;
    trackingCamera.rotation.y = player.r.y as FRACT;
    trackingCamera.oldView.r.y = trackingCamera.rotation.y as int32;
    trackingCamera.rotation.z = player.r.z as FRACT;
    trackingCamera.oldView.r.z = trackingCamera.rotation.z as int32;
    /* Store away the old positions and set the start position too */
    trackingCamera.position.x = player.p.x as FRACT;
    trackingCamera.oldView.p.x = trackingCamera.position.x as int32;
    trackingCamera.position.y = player.p.y as FRACT;
    trackingCamera.oldView.p.y = trackingCamera.position.y as int32;
    trackingCamera.position.z = player.p.z as FRACT;
    trackingCamera.oldView.p.z = trackingCamera.position.z as int32;
    //	trackingCamera.rotation.x = player.r.x = DEG(-90);
	/* No initial velocity for moving */
    trackingCamera.velocity.z = 0 as libc::c_int as FRACT;
    trackingCamera.velocity.y = trackingCamera.velocity.z;
    trackingCamera.velocity.x = trackingCamera.velocity.y;
    /* Nor for rotation */
    trackingCamera.rotVel.z = 0 as libc::c_int as FRACT;
    trackingCamera.rotVel.y = trackingCamera.rotVel.z;
    trackingCamera.rotVel.x = trackingCamera.rotVel.y;
    /* No initial acceleration for moving */
    trackingCamera.acceleration.z = 0 as libc::c_int as FRACT;
    trackingCamera.acceleration.y = trackingCamera.acceleration.z;
    trackingCamera.acceleration.x = trackingCamera.acceleration.y;
    /* Nor for rotation */
    trackingCamera.rotAccel.z = 0 as libc::c_int as FRACT;
    trackingCamera.rotAccel.y = trackingCamera.rotAccel.z;
    trackingCamera.rotAccel.x = trackingCamera.rotAccel.y;
    /* Sote the old distance */
    trackingCamera.oldDistance = getViewDistance(); //distance;
    /* Store away when we started */
    trackingCamera.lastUpdate = gameTime2;
    OldViewValid = 1 as libc::c_int;
}
/* Function Prototypes... */
/* Firstly for tracking position */
//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------
							/* How this all works */
/*
Each frame we calculate the new acceleration, velocity and positions for the location
and rotation of the camera. The velocity is obviously based on the acceleration and this
in turn is based on the separation between the two objects. This separation is distance
in the case of location and degrees of arc in the case of rotation.

  Each frame:-

  ACCELERATION	-	A
  VELOCITY		-	V
  POSITION		-	P
  Location of camera	(x1,y1)
  Location of droid		(x2,y2)
  Separation(distance) = D. This is the distance between (x1,y1) and (x2,y2)

  A = c1D - c2V		Where c1 and c2 are two constants to be found (by experiment)
  V = V + A(frameTime/GAME_TICKS_PER_SEC)
  P = P + V(frameTime/GAME_TICKS_PER_SEC)

  Things are the same for the rotation except that D is then the difference in angles
  between the way the camera and droid being tracked are facing. AND.... the two
  constants c1 and c2 will be different as we're dealing with entirely different scales
  and units. Separation in terms of distance could be in the thousands whereas degrees
  cannot exceed 180.

  This all works because acceleration is based on how far apart they are minus some factor
  times the camera's present velocity. This minus factor is what slows it down when the
  separation gets very small. Without this, it would continually oscillate about it's target
  point. The four constants (two each for rotation and position) need to be found
  by trial and error since the metrics of time,space and rotation are entirely warzone
  specific.

  And that's all folks.
*/
//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------
/*	N.B. the code from here on in is not very PSX friendly as there's lots of
	unfriendly little floats - essentially the next 6 functions */
#[no_mangle]
pub unsafe extern "C" fn updateCameraAcceleration(mut update: UBYTE) {
    let mut separation: FRACT = 0.;
    let mut realPos: SDWORD = 0;
    let mut xConcern: SDWORD = 0;
    let mut yConcern: SDWORD = 0;
    let mut zConcern: SDWORD = 0;
    let mut xBehind: SDWORD = 0;
    let mut yBehind: SDWORD = 0;
    let mut bFlying: BOOL = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut multiAngle: UDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    //SDWORD	pitch;
    let mut angle: SDWORD = 0;
    angle = abs(player.r.x / 182 as libc::c_int % 90 as libc::c_int);
    angle = 90 as libc::c_int - angle;
    bFlying = 0 as libc::c_int;
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = trackingCamera.target as *mut DROID;
        psPropStats =
            asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                         libc::c_int as isize);
        if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
           {
            bFlying = 1 as libc::c_int
        }
    }
    /*	This is where we check what it is we're tracking.
		Were we to track a building or location - this is
		where it'd be set up */
    /*	If we're tracking a droid, then we nned to track slightly in front
		of it in order that the droid appears down the screen a bit. This means
		that we need to find an offset point from it relative to it's present
		direction
	*/
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        /* Present direction is important */
        if getNumDroidsSelected() > 2 as libc::c_int as libc::c_uint {
            if (*trackingCamera.target).selected != 0 {
                multiAngle = getAverageTrackAngle(1 as libc::c_int) as UDWORD
            } else {
                multiAngle =
                    getGroupAverageTrackAngle((*trackingCamera.target).group
                                                  as UDWORD, 1 as libc::c_int)
                        as UDWORD
            }
            xBehind =
                camDroidYOffset *
                    *aSinTable.as_mut_ptr().offset((((65536 as libc::c_int /
                                                          360 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_mul(multiAngle)
                                                        as uint16 as
                                                        libc::c_int >>
                                                        4 as libc::c_int) as
                                                       isize) >>
                    12 as libc::c_int;
            yBehind =
                camDroidXOffset *
                    *aSinTable.as_mut_ptr().offset(((((65536 as libc::c_int /
                                                           360 as libc::c_int)
                                                          as
                                                          libc::c_uint).wrapping_mul(multiAngle)
                                                         as uint16 as
                                                         libc::c_int >>
                                                         4 as libc::c_int) +
                                                        1024 as libc::c_int)
                                                       as isize) >>
                    12 as libc::c_int
        } else {
            xBehind =
                camDroidYOffset *
                    *aSinTable.as_mut_ptr().offset(((65536 as libc::c_int /
                                                         360 as libc::c_int *
                                                         (*trackingCamera.target).direction
                                                             as libc::c_int)
                                                        as uint16 as
                                                        libc::c_int >>
                                                        4 as libc::c_int) as
                                                       isize) >>
                    12 as libc::c_int;
            yBehind =
                camDroidXOffset *
                    *aSinTable.as_mut_ptr().offset((((65536 as libc::c_int /
                                                          360 as libc::c_int *
                                                          (*trackingCamera.target).direction
                                                              as libc::c_int)
                                                         as uint16 as
                                                         libc::c_int >>
                                                         4 as libc::c_int) +
                                                        1024 as libc::c_int)
                                                       as isize) >>
                    12 as libc::c_int
        }
    } else {
        /* Irrelevant for normal radar tracking */
        xBehind = 0 as libc::c_int;
        yBehind = 0 as libc::c_int
    }
    /*	Get these new coordinates */
    if getNumDroidsSelected() > 2 as libc::c_int as libc::c_uint &&
           (*trackingCamera.target).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        xConcern =
            (*trackingCamera.target).x as SDWORD; // nb - still NEED to be set
        yConcern = (*trackingCamera.target).z as SDWORD;
        zConcern = (*trackingCamera.target).y as SDWORD;
        if (*trackingCamera.target).selected != 0 {
            getTrackingConcerns(&mut xConcern, &mut yConcern, &mut zConcern);
        } else {
            getGroupTrackingConcerns(&mut xConcern, &mut yConcern,
                                     &mut zConcern,
                                     (*trackingCamera.target).group as UDWORD,
                                     1 as libc::c_int);
        }
        //		getBestPitchToEdgeOfGrid(xConcern,zConcern,360-((getAverageTrackAngle(TRUE)+180)%360),&pitch);
        yConcern += angle * 5 as libc::c_int
    } else {
        xConcern = (*trackingCamera.target).x as SDWORD;
        yConcern = (*trackingCamera.target).z as SDWORD;
        zConcern = (*trackingCamera.target).y as SDWORD
    }
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint &&
           getNumDroidsSelected() <= 2 as libc::c_int as libc::c_uint {
        //		getBestPitchToEdgeOfGrid(trackingCamera.target->x,trackingCamera.target->z,
//			360-((trackingCamera.target->direction+180)%360),&pitch);
        yConcern += angle * 5 as libc::c_int
    }
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        /* Need to update acceleration along x axis */
        realPos =
            xConcern -
                32 as libc::c_int / 2 as libc::c_int * 128 as libc::c_int -
                xBehind;
        separation = realPos as libc::c_float - trackingCamera.position.x;
        if bFlying == 0 {
            trackingCamera.acceleration.x =
                accelConstant * separation -
                    velocityConstant * trackingCamera.velocity.x
        } else {
            trackingCamera.acceleration.x =
                accelConstant * separation * 4 as libc::c_int as libc::c_float
                    -
                    velocityConstant * 2 as libc::c_int as libc::c_float *
                        trackingCamera.velocity.x
        }
    }
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        //		flushConsoleMessages();
//		CONPRINTF(ConsoleString,(ConsoleString,"Attempted height : %d",yConcern));
        /* Need to update acceleration along y axis */
        realPos = yConcern;
        separation = realPos as libc::c_float - trackingCamera.position.y;
        if bFlying != 0 {
            separation = separation / 2 as libc::c_int as libc::c_float
        }
        //		CONPRINTF(ConsoleString,(ConsoleString,"Separation : %f",separation));
//		CONPRINTF(ConsoleString,(ConsoleString,"Distance : %d",distance));
        if bFlying == 0 {
            trackingCamera.acceleration.y =
                accelConstant * separation -
                    velocityConstant * trackingCamera.velocity.y
        } else {
            trackingCamera.acceleration.y =
                accelConstant * separation * 4 as libc::c_int as libc::c_float
                    -
                    velocityConstant * 2 as libc::c_int as libc::c_float *
                        trackingCamera.velocity.y
        }
    }
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        /* Need to update acceleration along z axis */
        realPos =
            zConcern -
                32 as libc::c_int / 2 as libc::c_int * 128 as libc::c_int -
                yBehind;
        separation = realPos as libc::c_float - trackingCamera.position.z;
        if bFlying == 0 {
            trackingCamera.acceleration.z =
                accelConstant * separation -
                    velocityConstant * trackingCamera.velocity.z
        } else {
            trackingCamera.acceleration.z =
                accelConstant * separation * 4 as libc::c_int as libc::c_float
                    -
                    velocityConstant * 2 as libc::c_int as libc::c_float *
                        trackingCamera.velocity.z
        }
    };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn updateCameraVelocity(mut update: UBYTE) {
    //UDWORD	frameTime;
    let mut fraction_0: FRACT = 0.;
    /*	Get the time fraction of a second - the next two lines are present in 4
		of the next six functions. All 4 of these functions are called every frame, so
		it may be an idea to calculate these higher up and store them in a static but
		I've left them in for clarity for now */
    //	frameTime = gameTime - trackingCamera.lastUpdate;
    fraction_0 = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        trackingCamera.velocity.x +=
            trackingCamera.acceleration.x * fraction_0
    }
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        trackingCamera.velocity.y +=
            trackingCamera.acceleration.y * fraction_0
    }
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        trackingCamera.velocity.z +=
            trackingCamera.acceleration.z * fraction_0
    };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn updateCameraPosition(mut update: UBYTE) {
    //UDWORD	frameTime;
    let mut bFlying: BOOL = 0;
    let mut fraction_0: FRACT = 0.;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    bFlying = 0 as libc::c_int;
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = trackingCamera.target as *mut DROID;
        psPropStats =
            asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                         libc::c_int as isize);
        if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
           {
            bFlying = 1 as libc::c_int
        }
    }
    /* See above */
//	frameTime = gameTime - trackingCamera.lastUpdate;
    fraction_0 = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        /* Need to update position along x axis */
        trackingCamera.position.x += trackingCamera.velocity.x * fraction_0
    }
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        //		if(trackingCamera.target->type == OBJ_TARGET)
//		{
			/* Need to update position along y axis */
//		if(!bFlying)
//		{
        trackingCamera.position.y += trackingCamera.velocity.y * fraction_0
        //		}
//		else
//		{
//			trackingCamera.position.y += MAKEINT(((FRACT)trackingCamera.velocity.y * fraction));
//		}
//		}
    }
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        /* Need to update position along z axis */
        trackingCamera.position.z += trackingCamera.velocity.z * fraction_0
    };
}
/* And now, rotation */
//-----------------------------------------------------------------------------------
/* Calculate the acceleration that the camera spins around at */
#[no_mangle]
pub unsafe extern "C" fn updateCameraRotationAcceleration(mut update: UBYTE) {
    let mut worldAngle: SDWORD = 0;
    let mut separation: FRACT = 0.;
    let mut xConcern: SDWORD = 0;
    let mut yConcern: SDWORD = 0;
    let mut zConcern: SDWORD = 0;
    let mut bTooLow: BOOL = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut droidHeight: UDWORD = 0;
    let mut mapHeight_0: UDWORD = 0;
    let mut difHeight: UDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut pitch: SDWORD = 0;
    let mut bGotFlying: BOOL = 0 as libc::c_int;
    let mut xPos: SDWORD = 0;
    let mut yPos: SDWORD = 0;
    let mut zPos: SDWORD = 0;
    bTooLow = 0 as libc::c_int;
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = trackingCamera.target as *mut DROID;
        psPropStats =
            asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                         libc::c_int as isize);
        if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
           {
            bGotFlying = 1 as libc::c_int;
            droidHeight = (*psDroid).z as UDWORD;
            mapHeight_0 =
                map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                    UDWORD;
            difHeight =
                abs(droidHeight.wrapping_sub(mapHeight_0) as libc::c_int) as
                    UDWORD;
            if difHeight < 16 as libc::c_int as libc::c_uint {
                bTooLow = 1 as libc::c_int
            }
        }
    }
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        /* Presently only y rotation being calculated - but same idea for other axes */
		/* Check what we're tracking */
        if getNumDroidsSelected() > 2 as libc::c_int as libc::c_uint &&
               (*trackingCamera.target).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
            if (*trackingCamera.target).selected != 0 {
                yConcern =
                    65536 as libc::c_int / 360 as libc::c_int *
                        getAverageTrackAngle(0 as libc::c_int)
                //DEG(trackingCamera.target->direction);
            } else {
                yConcern =
                    65536 as libc::c_int / 360 as libc::c_int *
                        getGroupAverageTrackAngle((*trackingCamera.target).group
                                                      as UDWORD,
                                                  0 as libc::c_int)
                //DEG(trackingCamera.target->direction);
            }
        } else {
            yConcern =
                65536 as libc::c_int / 360 as libc::c_int *
                    (*trackingCamera.target).direction as libc::c_int
        }
        yConcern +=
            65536 as libc::c_int / 360 as libc::c_int * 180 as libc::c_int;
        while trackingCamera.rotation.y < 0 as libc::c_int as libc::c_float {
            trackingCamera.rotation.y +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        /* Which way are we facing? */
        worldAngle = trackingCamera.rotation.y as SDWORD;
        separation = (yConcern - worldAngle) as FRACT;
        if separation <
               (65536 as libc::c_int / 360 as libc::c_int *
                    -(180 as libc::c_int)) as libc::c_float {
            separation +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        } else if separation >
                      (65536 as libc::c_int / 360 as libc::c_int *
                           180 as libc::c_int) as libc::c_float {
            separation -=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        /* Make new acceleration */
        trackingCamera.rotAccel.y =
            rotAccelConstant * separation -
                rotVelocityConstant * trackingCamera.rotVel.y
    }
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        if (*trackingCamera.target).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint && bGotFlying == 0 {
            psDroid = trackingCamera.target as *mut DROID;
            getTrackingConcerns(&mut xPos, &mut yPos, &mut zPos);
            if (*trackingCamera.target).selected != 0 {
                getBestPitchToEdgeOfGrid(xPos as UDWORD, zPos as UDWORD,
                                         (360 as libc::c_int -
                                              (getAverageTrackAngle(1 as
                                                                        libc::c_int)
                                                   + 180 as libc::c_int) %
                                                  360 as libc::c_int) as
                                             UDWORD, &mut pitch);
            } else {
                getBestPitchToEdgeOfGrid(xPos as UDWORD, zPos as UDWORD,
                                         (360 as libc::c_int -
                                              (getGroupAverageTrackAngle((*trackingCamera.target).group
                                                                             as
                                                                             UDWORD,
                                                                         1 as
                                                                             libc::c_int)
                                                   + 180 as libc::c_int) %
                                                  360 as libc::c_int) as
                                             UDWORD, &mut pitch);
            }
            if pitch < 14 as libc::c_int { pitch = 14 as libc::c_int }
            xConcern = 65536 as libc::c_int / 360 as libc::c_int * -pitch
        } else {
            xConcern =
                65536 as libc::c_int / 360 as libc::c_int *
                    (*trackingCamera.target).pitch as libc::c_int;
            xConcern +=
                65536 as libc::c_int / 360 as libc::c_int *
                    -(16 as libc::c_int)
        }
        //xConcern = DEG(trackingCamera.target->pitch);
	   //	if(xConcern>DEG(MINCAMROTX))
	   //	{
	   //		xConcern = DEG(MINCAMROTX);
	   //	}
        while trackingCamera.rotation.x < 0 as libc::c_int as libc::c_float {
            trackingCamera.rotation.x +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        worldAngle = trackingCamera.rotation.x as SDWORD;
        separation = (xConcern - worldAngle) as FRACT;
        while separation < 0 as libc::c_int as libc::c_float {
            separation +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        while separation >
                  (65536 as libc::c_int / 360 as libc::c_int *
                       360 as libc::c_int) as libc::c_float {
            separation -=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        if separation <
               (65536 as libc::c_int / 360 as libc::c_int *
                    -(180 as libc::c_int)) as libc::c_float {
            separation +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        } else if separation >
                      (65536 as libc::c_int / 360 as libc::c_int *
                           180 as libc::c_int) as libc::c_float {
            separation -=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        /* Make new acceleration */
        trackingCamera.rotAccel.x =
            rotAccelConstant * separation -
                rotVelocityConstant * trackingCamera.rotVel.x
    }
    /* This looks a bit arse - looks like a flight sim */
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        if bTooLow != 0 {
            zConcern = 0 as libc::c_int
        } else {
            zConcern =
                65536 as libc::c_int / 360 as libc::c_int *
                    (*trackingCamera.target).roll as libc::c_int
        }
        while trackingCamera.rotation.z < 0 as libc::c_int as libc::c_float {
            trackingCamera.rotation.z +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        worldAngle = trackingCamera.rotation.z as SDWORD;
        separation = (zConcern - worldAngle) as FRACT;
        if separation <
               (65536 as libc::c_int / 360 as libc::c_int *
                    -(180 as libc::c_int)) as libc::c_float {
            separation +=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        } else if separation >
                      (65536 as libc::c_int / 360 as libc::c_int *
                           180 as libc::c_int) as libc::c_float {
            separation -=
                (65536 as libc::c_int / 360 as libc::c_int *
                     360 as libc::c_int) as libc::c_float
        }
        /* Make new acceleration */
        trackingCamera.rotAccel.z =
            rotAccelConstant / 1 as libc::c_int as libc::c_float * separation
                - rotVelocityConstant * trackingCamera.rotVel.z
    };
}
//-----------------------------------------------------------------------------------
/*	Calculate the velocity that the camera spins around at - just add previously
	calculated acceleration */
#[no_mangle]
pub unsafe extern "C" fn updateCameraRotationVelocity(mut update: UBYTE) {
    //UDWORD	frameTime;
    let mut fraction_0: FRACT = 0.;
    //	frameTime = gameTime - trackingCamera.lastUpdate;
    fraction_0 = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        trackingCamera.rotVel.y += trackingCamera.rotAccel.y * fraction_0
    }
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        trackingCamera.rotVel.x += trackingCamera.rotAccel.x * fraction_0
    }
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        trackingCamera.rotVel.z += trackingCamera.rotAccel.z * fraction_0
    };
}
//-----------------------------------------------------------------------------------
/* Move the camera around by adding the velocity */
#[no_mangle]
pub unsafe extern "C" fn updateCameraRotationPosition(mut update: UBYTE) {
    //UDWORD	frameTime;
    let mut fraction_0: FRACT = 0.;
    //	frameTime = gameTime - trackingCamera.lastUpdate;
    fraction_0 = frameTime2 as FRACT / 1000 as libc::c_int as FRACT;
    if update as libc::c_int & 0x2 as libc::c_int != 0 {
        trackingCamera.rotation.y += trackingCamera.rotVel.y * fraction_0
    }
    if update as libc::c_int & 0x1 as libc::c_int != 0 {
        trackingCamera.rotation.x += trackingCamera.rotVel.x * fraction_0
    }
    if update as libc::c_int & 0x4 as libc::c_int != 0 {
        trackingCamera.rotation.z += trackingCamera.rotVel.z * fraction_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn nearEnough() -> BOOL {
    let mut retVal: BOOL = 0 as libc::c_int;
    let mut xPos: SDWORD = 0;
    let mut yPos: SDWORD = 0;
    xPos =
        player.p.x +
            32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int;
    yPos =
        player.p.z +
            32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int;
    if abs(xPos - (*trackingCamera.target).x as libc::c_int) <=
           256 as libc::c_int &&
           abs(yPos - (*trackingCamera.target).y as libc::c_int) <=
               256 as libc::c_int {
        retVal = 1 as libc::c_int
    }
    return retVal;
}
/* Updates the viewpoint according to the object being tracked */
#[no_mangle]
pub unsafe extern "C" fn camTrackCamera() -> BOOL {
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bFlying: BOOL = 0;
    bFlying = 0 as libc::c_int;
    /* Most importantly - see if the target we're tracking is dead! */
    if (*trackingCamera.target).died != 0 {
        setFindNewTarget();
        return 0 as libc::c_int
    }
    /*	Cancel tracking if it's no longer selected.
		This may not be desirable? 	*/
    ((*trackingCamera.target).type_0 as libc::c_uint) ==
        OBJ_DROID as libc::c_int as libc::c_uint;
    /* Update the acceleration,velocity and position of the camera for movement */
    updateCameraAcceleration((0x1 as libc::c_int + 0x2 as libc::c_int +
                                  0x4 as libc::c_int) as UBYTE);
    updateCameraVelocity((0x1 as libc::c_int + 0x2 as libc::c_int +
                              0x4 as libc::c_int) as UBYTE);
    updateCameraPosition((0x1 as libc::c_int + 0x2 as libc::c_int +
                              0x4 as libc::c_int) as UBYTE);
    /* Update the acceleration,velocity and rotation of the camera for rotation */
	/*	You can track roll as well (z axis) but it makes you ill and looks
		like a flight sim, so for now just pitch and orientation */
    if (*trackingCamera.target).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = trackingCamera.target as *mut DROID;
        psPropStats =
            asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                           libc::c_int as
                                                           usize].nStat as
                                         libc::c_int as isize);
        if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
           {
            bFlying = 1 as libc::c_int
        }
    }
    /*
	bIsBuilding = FALSE;
	if(trackingCamera.target->type == OBJ_DROID)
	{
		psDroid= (DROID*)trackingCamera.target;
		if(DroidIsBuilding(psDroid))
		{
			bIsBuilding = TRUE;
		}
	}
*/
    if bRadarAllign != 0 ||
           (*trackingCamera.target).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        if bFlying != 0 {
            updateCameraRotationAcceleration((0x1 as libc::c_int +
                                                  0x2 as libc::c_int +
                                                  0x4 as libc::c_int) as
                                                 UBYTE);
        } else {
            updateCameraRotationAcceleration((0x1 as libc::c_int +
                                                  0x2 as libc::c_int) as
                                                 UBYTE);
        }
    }
    if bFlying != 0 {
        updateCameraRotationVelocity((0x1 as libc::c_int + 0x2 as libc::c_int
                                          + 0x4 as libc::c_int) as UBYTE);
        updateCameraRotationPosition((0x1 as libc::c_int + 0x2 as libc::c_int
                                          + 0x4 as libc::c_int) as UBYTE);
    } else {
        /*
	else if(bIsBuilding)
	{
		updateCameraRotationVelocity(CAM_X_ONLY);
	}
	*/
        updateCameraRotationVelocity((0x1 as libc::c_int + 0x2 as libc::c_int)
                                         as UBYTE);
        updateCameraRotationPosition((0x1 as libc::c_int + 0x2 as libc::c_int)
                                         as UBYTE);
    }
    /* Record the old positions for comparison */
    oldPosition.x = player.p.x;
    oldPosition.y = player.p.y;
    oldPosition.z = player.p.z;
    /* Update the position that's now stored in trackingCamera.position (iVector) */
    player.p.x = trackingCamera.position.x as int32;
    player.p.y = trackingCamera.position.y as int32;
    player.p.z = trackingCamera.position.z as int32;
    /* Record the old positions for comparison */
    oldRotation.x = player.r.x;
    oldRotation.y = player.r.y;
    oldRotation.z = player.r.z;
    /* Update the rotations that're now stored in trackingCamera.rotation (iVector) */
    player.r.x = trackingCamera.rotation.x as int32;
    /*if(!bIsBuilding)*/
    player.r.y = trackingCamera.rotation.y as int32;
    player.r.z = trackingCamera.rotation.z as int32;
    /* There's a minimum for this - especially when John's VTOL code lets them land vertically on cliffs */
    if player.r.x >
           65536 as libc::c_int / 360 as libc::c_int *
               (360 as libc::c_int + -(14 as libc::c_int)) {
        player.r.x =
            65536 as libc::c_int / 360 as libc::c_int *
                (360 as libc::c_int + -(14 as libc::c_int))
    }
    /*
	if(bIsBuilding)
	{
		player.r.y+=DEG(1);
	}
	*/
	/* Clip the position to the edge of the map */
    CheckScrollLimits();
    /* Store away our last update as acceleration and velocity are all fn()/dt */
    trackingCamera.lastUpdate = gameTime2;
    if bFullInfo != 0 {
        flushConsoleMessages();
        if (*trackingCamera.target).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            printDroidInfo(trackingCamera.target as *mut DROID);
        }
    }
    /* Switch off if we're jumping to a new location and we've got there */
    if getRadarTrackingStatus() != 0 {
        /*	This will ensure we come to a rest and terminate the tracking
			routine once we're close enough
		*/
        if getRotationMagnitude() < 10000 as libc::c_int as libc::c_uint {
            if nearEnough() != 0 &&
                   getPositionMagnitude() < 60 as libc::c_int as libc::c_uint
               {
                camToggleStatus();
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn processLeaderSelection() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psPresent: *mut DROID = 0 as *mut DROID;
    let mut psNew: *mut DROID = 0 as *mut DROID;
    let mut leaderClass: UDWORD = 0;
    let mut bSuccess: BOOL = 0;
    let mut dif: UDWORD = 0;
    let mut bestSoFar: UDWORD = 0;
    if demoGetStatus() != 0 { return }
    if getWarCamStatus() != 0 {
        /* Only do if we're tracking a droid */
        if (*trackingCamera.target).type_0 as libc::c_uint !=
               OBJ_DROID as libc::c_int as libc::c_uint {
            return
        }
    } else { return }
    /* Don't do if we're driving?! */
    if getDrivingStatus() != 0 { return }
    psPresent = trackingCamera.target as *mut DROID;
    if keyPressed(KEY_LEFTARROW) != 0 {
        leaderClass = 1 as libc::c_int as UDWORD
    } else if keyPressed(KEY_RIGHTARROW) != 0 {
        leaderClass = 2 as libc::c_int as UDWORD
    } else if keyPressed(KEY_UPARROW) != 0 {
        leaderClass = 3 as libc::c_int as UDWORD
    } else if keyPressed(KEY_DOWNARROW) != 0 {
        leaderClass = 4 as libc::c_int as UDWORD
    } else { leaderClass = 5 as libc::c_int as UDWORD }
    bSuccess = 0 as libc::c_int;
    bestSoFar = 0xffffffff as libc::c_uint;
    match leaderClass {
        1 => {
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                /* Is it even on the sscreen? */
                if DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD)
                       != 0 && (*psDroid).selected as libc::c_int != 0 &&
                       psDroid != psPresent {
                    if (*psDroid).sDisplay.screenX <
                           (*psPresent).sDisplay.screenX {
                        dif =
                            (*psPresent).sDisplay.screenX.wrapping_sub((*psDroid).sDisplay.screenX);
                        if dif < bestSoFar {
                            bestSoFar = dif;
                            bSuccess = 1 as libc::c_int;
                            psNew = psDroid
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        2 => {
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                /* Is it even on the sscreen? */
                if DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD)
                       != 0 && (*psDroid).selected as libc::c_int != 0 &&
                       psDroid != psPresent {
                    if (*psDroid).sDisplay.screenX >
                           (*psPresent).sDisplay.screenX {
                        dif =
                            (*psDroid).sDisplay.screenX.wrapping_sub((*psPresent).sDisplay.screenX);
                        if dif < bestSoFar {
                            bestSoFar = dif;
                            bSuccess = 1 as libc::c_int;
                            psNew = psDroid
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        3 => {
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                /* Is it even on the sscreen? */
                if DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD)
                       != 0 && (*psDroid).selected as libc::c_int != 0 &&
                       psDroid != psPresent {
                    if (*psDroid).sDisplay.screenY <
                           (*psPresent).sDisplay.screenY {
                        dif =
                            (*psPresent).sDisplay.screenY.wrapping_sub((*psDroid).sDisplay.screenY);
                        if dif < bestSoFar {
                            bestSoFar = dif;
                            bSuccess = 1 as libc::c_int;
                            psNew = psDroid
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        4 => {
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                /* Is it even on the sscreen? */
                if DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD)
                       != 0 && (*psDroid).selected as libc::c_int != 0 &&
                       psDroid != psPresent {
                    if (*psDroid).sDisplay.screenY >
                           (*psPresent).sDisplay.screenY {
                        dif =
                            (*psDroid).sDisplay.screenY.wrapping_sub((*psPresent).sDisplay.screenY);
                        if dif < bestSoFar {
                            bestSoFar = dif;
                            bSuccess = 1 as libc::c_int;
                            psNew = psDroid
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        5 | _ => { }
    }
    if bSuccess != 0 { camAllignWithTarget(psNew as *mut BASE_OBJECT); };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getTrackingDroid() -> *mut DROID {
    if getWarCamStatus() == 0 { return 0 as *mut DROID }
    if trackingCamera.status != CAM_TRACKING as libc::c_int as libc::c_uint {
        return 0 as *mut DROID
    }
    if (*trackingCamera.target).type_0 as libc::c_uint !=
           OBJ_DROID as libc::c_int as libc::c_uint {
        return 0 as *mut DROID
    }
    return trackingCamera.target as *mut DROID;
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getGroupAverageTrackAngle(mut groupNumber: UDWORD,
                                                   mut bCheckOnScreen: BOOL)
 -> SDWORD {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut xShift: FRACT = 0.;
    let mut yShift: FRACT = 0.;
    let mut xTotal: FRACT = 0.;
    let mut yTotal: FRACT = 0.;
    let mut averageAngleFloat: FRACT = 0 as libc::c_int as FRACT;
    let mut droidCount: SDWORD = 0;
    let mut averageAngle: SDWORD = 0;
    let mut retVal: SDWORD = 0;
    /* Initialise all the stuff */
    droidCount = 0 as libc::c_int;
    averageAngle = 0 as libc::c_int;
    /* Set totals to zero */
    yTotal = 0.0f32;
    xTotal = yTotal;
    /* Got thru' all droids */
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        /* Is he worth considering? */
        if (*psDroid).group as libc::c_uint == groupNumber {
            if if bCheckOnScreen != 0 {
                   droidOnScreen(psDroid,
                                 pie_GetVideoBufferWidth().wrapping_div(6 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                     as SDWORD)
               } else { 1 as libc::c_int } != 0 {
                droidCount += 1;
                averageAngle += (*psDroid).direction as libc::c_int;
                xShift = trigSin((*psDroid).direction as SDWORD);
                yShift = trigCos((*psDroid).direction as SDWORD);
                xTotal += xShift;
                yTotal += yShift
            }
            /*
			if(bCheckOnScreen)
			{
				if(droidOnScreen(psDroid,DISP_WIDTH/6))
				{
					droidCount++;
					averageAngle+=psDroid->direction;
					xShift = trigSin(psDroid->direction);
					yShift = trigCos(psDroid->direction);
					xTotal += xShift;
					yTotal += yShift;

				}
			}
			else
			{
					droidCount++;
					averageAngle+=psDroid->direction;
					xShift = trigSin(psDroid->direction);
					yShift = trigCos(psDroid->direction);
					xTotal += xShift;
					yTotal += yShift;


			}
			*/
        } //retVal;
        psDroid = (*psDroid).psNext
    }
    if droidCount != 0 {
        retVal = averageAngle / droidCount;
        averageAngleFloat =
            (atan2(xTotal as libc::c_double, yTotal as libc::c_double) *
                 180.0f64 / 3.141592654f64) as FRACT
    } else { retVal = 0 as libc::c_int }
    presAvAngle = averageAngleFloat as SDWORD;
    return presAvAngle;
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getAverageTrackAngle(mut bCheckOnScreen: BOOL)
 -> SDWORD {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut xShift: FRACT = 0.;
    let mut yShift: FRACT = 0.;
    let mut xTotal: FRACT = 0.;
    let mut yTotal: FRACT = 0.;
    let mut averageAngleFloat: FRACT = 0 as libc::c_int as FRACT;
    let mut droidCount: SDWORD = 0;
    let mut averageAngle: SDWORD = 0;
    let mut retVal: SDWORD = 0;
    /* Initialise all the stuff */
    droidCount = 0 as libc::c_int;
    averageAngle = 0 as libc::c_int;
    /* Set totals to zero */
    yTotal = 0.0f32;
    xTotal = yTotal;
    /* Got thru' all droids */
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        /* Is he worth selecting? */
        if (*psDroid).selected != 0 {
            if if bCheckOnScreen != 0 {
                   droidOnScreen(psDroid,
                                 pie_GetVideoBufferWidth().wrapping_div(6 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                     as SDWORD)
               } else { 1 as libc::c_int } != 0 {
                droidCount += 1;
                averageAngle += (*psDroid).direction as libc::c_int;
                xShift = trigSin((*psDroid).direction as SDWORD);
                yShift = trigCos((*psDroid).direction as SDWORD);
                xTotal += xShift;
                yTotal += yShift
            }
            /*
			if(bCheckOnScreen)
			{
				if(droidOnScreen(psDroid,DISP_WIDTH/6))
				{
			 		droidCount++;
					averageAngle+=psDroid->direction;
					xShift = trigSin(psDroid->direction);
					yShift = trigCos(psDroid->direction);
					xTotal += xShift;
					yTotal += yShift;

				}
			}
			else
			{
					droidCount++;
					averageAngle+=psDroid->direction;
					xShift = trigSin(psDroid->direction);
					yShift = trigCos(psDroid->direction);
					xTotal += xShift;
					yTotal += yShift;


			}
			*/
        } //retVal;
        psDroid = (*psDroid).psNext
    }
    if droidCount != 0 {
        retVal = averageAngle / droidCount;
        averageAngleFloat =
            (atan2(xTotal as libc::c_double, yTotal as libc::c_double) *
                 180.0f64 / 3.141592654f64) as libc::c_float
    } else { retVal = 0 as libc::c_int }
    presAvAngle = averageAngleFloat as SDWORD;
    return presAvAngle;
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getPresAngle() -> SDWORD { return presAvAngle; }
//-----------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getNumDroidsSelected() -> UDWORD {
    return selNumSelected(selectedPlayer);
    /*
DROID	*psDroid;
UDWORD	count;

	for(psDroid = apsDroidLists[selectedPlayer],count = 0;
		psDroid; psDroid = psDroid->psNext)
	{
		if(psDroid->selected)
		{
			count++;
		}
	}
	return(count);
*/
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getTrackingConcerns(mut x: *mut SDWORD,
                                             mut y: *mut SDWORD,
                                             mut z: *mut SDWORD) {
    let mut xTotals: SDWORD = 0; // note the flip
    let mut yTotals: SDWORD = 0;
    let mut zTotals: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut count: UDWORD = 0;
    zTotals = 0 as libc::c_int;
    yTotals = zTotals;
    xTotals = yTotals;
    count = 0 as libc::c_int as UDWORD;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 {
            if droidOnScreen(psDroid,
                             pie_GetVideoBufferWidth().wrapping_div(4 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                 as SDWORD) != 0 {
                count = count.wrapping_add(1);
                xTotals += (*psDroid).x as libc::c_int;
                yTotals += (*psDroid).z as libc::c_int;
                zTotals += (*psDroid).y as libc::c_int
            }
        }
        psDroid = (*psDroid).psNext
    }
    if count != 0 {
        // necessary!!!!!!!
        *x = (xTotals as libc::c_uint).wrapping_div(count) as SDWORD;
        *y = (yTotals as libc::c_uint).wrapping_div(count) as SDWORD;
        *z = (zTotals as libc::c_uint).wrapping_div(count) as SDWORD
    };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getGroupTrackingConcerns(mut x: *mut SDWORD,
                                                  mut y: *mut SDWORD,
                                                  mut z: *mut SDWORD,
                                                  mut groupNumber: UDWORD,
                                                  mut bOnScreen: BOOL) {
    let mut xTotals: SDWORD = 0;
    let mut yTotals: SDWORD = 0;
    let mut zTotals: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut count: UDWORD = 0;
    zTotals = 0 as libc::c_int;
    yTotals = zTotals;
    xTotals = yTotals;
    count = 0 as libc::c_int as UDWORD;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).group as libc::c_uint == groupNumber {
            if if bOnScreen != 0 {
                   droidOnScreen(psDroid,
                                 pie_GetVideoBufferWidth().wrapping_div(4 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                     as SDWORD)
               } else { 1 as libc::c_int } != 0 {
                //					if(droidOnScreen(psDroid,DISP_WIDTH/4))
//					{
                count = count.wrapping_add(1);
                xTotals += (*psDroid).x as libc::c_int;
                //					}
                yTotals += (*psDroid).z as libc::c_int; // note the flip
                zTotals += (*psDroid).y as libc::c_int
            }
            //				else
//				{
//						count++;
//						xTotals+=psDroid->x;
//						yTotals+=psDroid->z;	// note the flip
//						zTotals+=psDroid->y;
//				}
        }
        psDroid = (*psDroid).psNext
    }
    if count != 0 {
        // necessary!!!!!!!
        *x = (xTotals as libc::c_uint).wrapping_div(count) as SDWORD;
        *y = (yTotals as libc::c_uint).wrapping_div(count) as SDWORD;
        *z = (zTotals as libc::c_uint).wrapping_div(count) as SDWORD
    };
}
//-----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn camSetOldView(mut x: libc::c_int, mut y: libc::c_int,
                                       mut z: libc::c_int,
                                       mut rx: libc::c_int,
                                       mut ry: libc::c_int,
                                       mut dist: libc::c_int) {
    //DBPRINTF(("camSetOldView(%d %d %d %d %d %d)\n",x,y,z,rx,ry,dist));
//DBPRINTF(("%d %d %d %d %d %d\n",player.p.x,player.p.y,player.r.x,player.r.y,getViewDistance()));
//	trackingCamera.oldView.p.x = x;
//	trackingCamera.oldView.p.y = y;
//	trackingCamera.oldView.p.z = z;
//	trackingCamera.oldView.r.x = rx;
//	trackingCamera.oldView.r.y = ry;
//	trackingCamera.oldDistance = dist;
}
/* Static function that switches off tracking - and might not be desirable? - Jim?*/
#[no_mangle]
pub unsafe extern "C" fn camSwitchOff() {
    /* Restore the angles */
    //	player.r.x = trackingCamera.oldView.r.x;
    player.r.z = trackingCamera.oldView.r.z;
    /* And height */
	/* Is this desirable??? */
//	player.p.y = trackingCamera.oldView.p.y;
    /* Restore distance */
    setViewDistance(trackingCamera.oldDistance);
}
//-----------------------------------------------------------------------------------
/* Returns whether or not the tracking camera is active */
#[no_mangle]
pub unsafe extern "C" fn getWarCamStatus() -> BOOL {
    /* Is it switched off? */
    if trackingCamera.status == CAM_INACTIVE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else {
        /* Tracking is ON */
        return 1 as libc::c_int
    };
}
//-----------------------------------------------------------------------------------
/* Flips the status of tracking to the opposite of what it presently is */
#[no_mangle]
pub unsafe extern "C" fn camToggleStatus() {
    /* If it's off */
    if trackingCamera.status == CAM_INACTIVE as libc::c_int as libc::c_uint {
        /* Switch it on */
        setWarCamActive(1 as libc::c_int);
    } else {
        /* Otherwise, switch it off */
        setWarCamActive(0 as libc::c_int);
        //		if(getDrivingStatus())
//		{
//			StopDriverMode();
//		}
    };
}
/*	Flips on/off whether we print out full info about the droid being tracked.
	If ON then this info is permanent on screen and realtime updating */
#[no_mangle]
pub unsafe extern "C" fn camToggleInfo() {
    bFullInfo = (bFullInfo == 0) as libc::c_int;
}
//void	setUpRadarTarget			( SDWORD x, SDWORD y );
/* Sets up the dummy target for the camera */
//void	setUpRadarTarget(SDWORD x, SDWORD y)
#[no_mangle]
pub unsafe extern "C" fn setUpRadarTarget(mut x: SDWORD, mut y: SDWORD) {
    radarTarget.x = x as UWORD;
    radarTarget.y = y as UWORD;
    if x < 0 as libc::c_int || y < 0 as libc::c_int ||
           x >
               mapWidth.wrapping_sub(1 as libc::c_int as
                                         libc::c_uint).wrapping_mul(128 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                   as SDWORD ||
           y >
               mapHeight.wrapping_sub(1 as libc::c_int as
                                          libc::c_uint).wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                   as SDWORD {
        radarTarget.z = (128 as libc::c_int * 2 as libc::c_int) as UWORD
    } else { radarTarget.z = map_Height(x as UDWORD, y as UDWORD) as UWORD }
    radarTarget.direction =
        calcDirection(player.p.x as UDWORD, player.p.z as UDWORD, x as UDWORD,
                      y as UDWORD) as UWORD;
    radarTarget.pitch = 0 as libc::c_int as SWORD;
    radarTarget.roll = 0 as libc::c_int as SWORD;
    radarTarget.type_0 = OBJ_TARGET;
    radarTarget.died = 0 as libc::c_int as UDWORD;
}
/* Informs the tracking camera that we want to start tracking to a new radar target */
#[no_mangle]
pub unsafe extern "C" fn requestRadarTrack(mut x: SDWORD, mut y: SDWORD) {
    /*
	ASSERT( x<mapWidth*TILE_UNITS,"Weirdy x coordinate for tracking" );
	ASSERT( y<mapHeight*TILE_UNITS,"Weirdy y coordinate for tracking" );
*/
    radarX = x as SWORD as FRACT;
    radarY = y as SWORD as FRACT;
    bRadarTrackingRequested = 1 as libc::c_int;
    trackingCamera.status = CAM_REQUEST as libc::c_int as UDWORD;
    processWarCam();
    // 	setWarCamActive(TRUE);
}
/* Returns whether we're presently tracking to a new _location_ */
#[no_mangle]
pub unsafe extern "C" fn getRadarTrackingStatus() -> BOOL {
    let mut retVal: BOOL = 0;
    if trackingCamera.status == CAM_INACTIVE as libc::c_int as libc::c_uint {
        retVal = 0 as libc::c_int
    } else if !trackingCamera.target.is_null() &&
                  (*trackingCamera.target).type_0 as libc::c_uint ==
                      OBJ_TARGET as libc::c_int as libc::c_uint {
        retVal = 1 as libc::c_int
    } else { retVal = 0 as libc::c_int }
    return retVal;
}
//if(/*trackingCamera.target && */trackingCamera.target->type == OBJ_TARGET)
        //if you know why the above check was commented out please tell me AB 19/11/98
/* Displays a spinning MTV style logo in the top right of the screen */
#[no_mangle]
pub unsafe extern "C" fn dispWarCamLogo() {
    //iVector		dv;
//
//	if(gamePaused())
//	{
//		/* get out if we're paused */
//		return;
//	}
//
//	warCamLogoRotation += MAKEINT( (MAKEFRACT(LOGO_ROT_SPEED) * fraction) );
//	dv.x = 280;
//	dv.y = 165;
//	dv.z = 1000;
//	iV_MatrixBegin();							/* Push the indentity matrix */
//	iV_TRANSLATE(dv.x,dv.y,dv.z);
//	scaleMatrix(15);
//	iV_MatrixRotateY(warCamLogoRotation);
//	iV_MatrixRotateX(player.r.x);
//
//	pie_Draw3DShape(cameraImd, 0, 0, pie_MAX_BRIGHT_LEVEL, 0, pie_BUTTON, 0);
//	iV_MatrixEnd();
}
#[no_mangle]
pub unsafe extern "C" fn toggleRadarAllignment() {
    bRadarAllign = (bRadarAllign == 0) as libc::c_int;
}
/* Returns how far away we are from our goal in a radar track */
#[no_mangle]
pub unsafe extern "C" fn getPositionMagnitude() -> UDWORD {
    let mut dif: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut val: UDWORD = 0;
    dif.x = abs(player.p.x - oldPosition.x);
    dif.y = abs(player.p.y - oldPosition.y);
    dif.z = abs(player.p.z - oldPosition.z);
    val = (dif.x * dif.x + dif.y * dif.y + dif.z * dif.z) as UDWORD;
    return val;
}
/* Rteurns how far away we are from our goal in rotation */
#[no_mangle]
pub unsafe extern "C" fn getRotationMagnitude() -> UDWORD {
    let mut dif: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut val: UDWORD = 0;
    dif.x = abs(player.r.x - oldRotation.x);
    dif.y = abs(player.r.y - oldRotation.y);
    dif.z = abs(player.r.z - oldRotation.z);
    val = (dif.x * dif.x + dif.y * dif.y + dif.z * dif.z) as UDWORD;
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn camInformOfRotation(mut rotation: *mut iVector) {
    trackingCamera.rotation.x = (*rotation).x as FRACT;
    trackingCamera.rotation.y = (*rotation).y as FRACT;
    trackingCamera.rotation.z = (*rotation).z as FRACT;
}
