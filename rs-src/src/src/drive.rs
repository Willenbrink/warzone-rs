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
    /* Return the absolute value of X.  */
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being down to being up this frame */
    #[no_mangle]
    fn mouseReleased(code: MOUSE_KEY_CODE) -> BOOL;
    // De-select a droid and do any necessary housekeeping.
    #[no_mangle]
    fn DeSelectDroid(psDroid: *mut DROID);
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    /*
 * Move.h
 *
 * Interface for the unit movement system
 *
 */
    /* The base movement speed */
    // The next object that should get the router when a lot of units are
// in a MOVEROUTE state
    /* Initialise the movement system */
    /* Update the base speed for all movement */
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
    #[no_mangle]
    fn moveDroidTo(psDroid: *mut DROID, x: UDWORD, y: UDWORD) -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn moveCalcDroidSpeed(psDroid: *mut DROID) -> SDWORD;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* Give a droid an order with a location target */
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    /* Give selected droids an order with a location target */
    #[no_mangle]
    fn orderSelectedLoc(player: UDWORD, x: UDWORD, y: UDWORD);
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    fn StartTacticalScroll(driveActive: BOOL);
    #[no_mangle]
    fn StartTacticalScrollObj(driveActive: BOOL, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn CancelTacticalScroll();
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn intAddReticule() -> BOOL;
    #[no_mangle]
    fn intRemoveReticule();
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    #[no_mangle]
    fn intGotoNextDroidType(CurrDroid: *mut DROID, droidType: UDWORD,
                            AllowGroup: BOOL) -> *mut DROID;
    #[no_mangle]
    fn camAllignWithTarget(psTarget: *mut BASE_OBJECT);
    #[no_mangle]
    fn CalcRadarPosition(mX: UDWORD, mY: UDWORD, PosX: *mut UDWORD,
                         PosY: *mut UDWORD);
    #[no_mangle]
    fn DroidIsBuilding(Droid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn DroidGoingToBuild(Droid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn DroidIsDemolishing(Droid: *mut DROID) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn SendDroidInfo(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD, psObj: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn targetAquireNearestObjView(psObj: *mut BASE_OBJECT, TargetType: UWORD)
     -> *mut BASE_OBJECT;
    #[no_mangle]
    static mut DirectControl: BOOL;
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
/*
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
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
pub type STRUCT_STRENGTH = _struct_strength;
pub type _struct_strength = libc::c_uint;
pub const NUM_STRUCT_STRENGTH: _struct_strength = 4;
pub const STRENGTH_BUNKER: _struct_strength = 3;
pub const STRENGTH_HARD: _struct_strength = 2;
pub const STRENGTH_MEDIUM: _struct_strength = 1;
pub const STRENGTH_SOFT: _struct_strength = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _formation {
    pub refCount: SWORD,
    pub size: SWORD,
    pub rankDist: SWORD,
    pub dir: SWORD,
    pub x: SDWORD,
    pub y: SDWORD,
    pub asLines: [F_LINE; 4],
    pub numLines: SWORD,
    pub maxRank: UBYTE,
    pub free: SBYTE,
    pub asMembers: [F_MEMBER; 20],
    pub iSpeed: UDWORD,
    pub psNext: *mut _formation,
}
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
// information about a formation member
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
}
pub type BASE_OBJECT = _base_object;
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
/*
 * FormationDef.h
 *
 */
// maximum number of lines in a formation
// maximum number of unit members of a formation (cannot be more that 128)
// information about a formation line
// a linked list of the formation members on this line is maintained
// using their index in the asMembers array.  (-1 == 'NULL')
// (cuts down the memory use over proper pointers)
pub type F_LINE = _f_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_line {
    pub xoffset: SWORD,
    pub yoffset: SWORD,
    pub dir: SWORD,
    pub member: SBYTE,
}
// position relative to center
// orientation of line
// first member in the 'linked list' of members
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
//	SDWORD		XCoordinate,YCoordinate;
pub type MOVE_CONTROL = _move_control;
// Inactive, Navigating or moving point to point status
// Mask used for the creation of this path	
//	SBYTE	Direction;					// Direction object should be moving (0-7) 0=Up,1=Up-Right
//	SDWORD	Speed;						// Speed at which object moves along the movement list
// Position in asPath
// number of points in asPath
//	PATH_POINT	MovementList[TRAVELSIZE];
// Pointer to list of block X,Y coordinates.
// When initialised list is terminated by (0xffff,0xffff)
										// Values prefixed by 0x8000 are pixel coordinates instead of
										// block coordinates
// DestinationX,Y should match objects current X,Y
//		location for this movement to be complete.
//   	UDWORD	Direction3D;				// *** not necessary
//	UDWORD	TargetDir;					// *** not necessary Direction the object should be facing
//	SDWORD	Gradient;					// Gradient of line
//	SDWORD	DeltaX;						// Distance from start to end position of current movement X
//	SDWORD	DeltaY;						// Distance from start to end position of current movement Y
//	SDWORD	XStep;						// Adjustment to the characters X position each movement
//	SDWORD	YStep;						// Adjustment to the characters Y position each movement
//	SDWORD	DestPixelX;					// Pixel coordinate destination for pixel movement (NOT the final X)
//	SDWORD	DestPixelY;					// Pixel coordiante destination for pixel movement (NOT the final Y)
/* Stuff for John's movement update */
// droid location as a fract
//	FRACT	dx,dy;						// x and y change for current direction
	// NOTE: this is supposed to replace Speed
// Speed of motion
// Vector for the end of path boundary
// direction of motion (not the direction the droid is facing)
// direction at last bump
// time of first bump with something
// time of last bump with a droid - relative to bumpTime
// when MOVEPAUSE started - relative to bumpTime
// position of last bump
// when a shuffle started
// formation the droid is currently a member of
/* vtol movement - GJ */
// added for vtol movement
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
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
pub type DROID = _droid;
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
/*
 * Order.h
 *
 * Function prototypes for giving droids orders
 *
 */
//turn off the build queue availability until desired release date!
//#define DISABLE_BUILD_QUEUE
// The droid orders
pub type DROID_ORDER = _droid_order;
pub type _secondary_order = libc::c_uint;
pub const DSO_ASSIGN_VTOL_PRODUCTION: _secondary_order = 11;
pub const DSO_FIRE_DESIGNATOR: _secondary_order = 10;
pub const DSO_RETURN_TO_LOC: _secondary_order = 9;
pub const DSO_HALTTYPE: _secondary_order = 8;
pub const DSO_PATROL: _secondary_order = 7;
pub const DSO_RECYCLE: _secondary_order = 6;
pub const DSO_CLEAR_PRODUCTION: _secondary_order = 5;
pub const DSO_ASSIGN_CYBORG_PRODUCTION: _secondary_order = 4;
pub const DSO_ASSIGN_PRODUCTION: _secondary_order = 3;
pub const DSO_ATTACK_LEVEL: _secondary_order = 2;
pub const DSO_REPAIR_LEVEL: _secondary_order = 1;
pub const DSO_ATTACK_RANGE: _secondary_order = 0;
// no order set
// stop the current order
// 2 - move to a location
// attack an enemy
// 4 - build a structure
// help to build a structure
// 6 - build a number of structures in a row (walls + bridges)
// demolish a structure
// 8 - repair a structure
// keep a target in sensor view
// 10 - attack whatever the linked sensor droid attacks
// return to the players retreat position
// 12 - self destruct
// return to base
// 14 - return to repair at any repair facility
// run away after moral failure
// 16 - board a transporter
// get off a transporter
// 18 - a suggestion to attack something
// i.e. the target was chosen because the droid could see it
// a command droid issuing orders to it's group
// 20 - build a module (power, research or factory)
// return to factory to be recycled
// 22 - offworld transporter order
// onworld transporter order
// 24 - transporter return after unloading
// guard a structure
// 26 - repair a droid
// restore resistance points for a structure
// 28 - same as move, but stop if an enemy is seen
// run away on fire
// 30 - constructor droid to clear up building wreckage
// move between two way points
// 32 - order a vtol to rearming pad
// move to a location taking out a blocking wall on the way
// 34 - scout to a location taking out a blocking wall on the way
// pick up an artifact
// 36 - vtol flying off the map
// return to repair at a specified repair center
// secondary orders for droids
pub type SECONDARY_ORDER = _secondary_order;
pub type _secondary_state = libc::c_uint;
pub const DSS_VTOLPROD_END: _secondary_state = 268435456;
pub const DSS_VTOLPROD_START: _secondary_state = 16777216;
pub const DSS_FIREDES_SET: _secondary_state = 8388608;
pub const DSS_PATROL_SET: _secondary_state = 4194304;
pub const DSS_RTL_TRANSPORT: _secondary_state = 2097152;
pub const DSS_RTL_BASE: _secondary_state = 1048576;
pub const DSS_RTL_REPAIR: _secondary_state = 524288;
pub const DSS_ASSPROD_END: _secondary_state = 262144;
pub const DSS_ASSPROD_MID: _secondary_state = 8192;
pub const DSS_ASSPROD_START: _secondary_state = 512;
pub const DSS_RECYCLE_SET: _secondary_state = 256;
pub const DSS_HALT_PERSUE: _secondary_state = 192;
pub const DSS_HALT_GUARD: _secondary_state = 128;
pub const DSS_HALT_HOLD: _secondary_state = 64;
pub const DSS_ALEV_NEVER: _secondary_state = 48;
pub const DSS_ALEV_ATTACKED: _secondary_state = 32;
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const DSS_ARANGE_LONG: _secondary_state = 2;
pub const DSS_ARANGE_SHORT: _secondary_state = 1;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
// the state of secondary orders
pub type SECONDARY_STATE = _secondary_state;
pub type DROID_GROUP = _droid_group;
pub const CONTROLMODE_DRIVE: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub const CONTROLMODE_POINTNCLICK: C2RustUnnamed = 0;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
//
// Drive.c
//
// Routines for player driving units about the map.
//
// FIXME Direct iVis implementation include!
// all the bollox needed for script callbacks
// Enable driving noises.
static mut DrivingAudioTrack: SWORD = -(1 as libc::c_int) as SWORD;
// How close followers need to get to the driver before they can stop.
// Damping value for analogue turning.
// Start to orbit if idle for 60 seconds.
#[no_mangle]
pub static mut psDrivenDroid: *mut DROID = 0 as *const DROID as *mut DROID;
// The droid that's being driven.
static mut bDriveMode: BOOL = 0 as libc::c_int;
static mut driveDir: SDWORD = 0;
// Driven droid's direction.
static mut driveSpeed: SDWORD = 0;
// Driven droid's speed.
static mut driveBumpTime: libc::c_int = 0;
// Time that followers get a kick up the ass.
static mut DoFollowRangeCheck: BOOL = 1 as libc::c_int;
static mut AllInRange: BOOL = 1 as libc::c_int;
static mut ClearFollowRangeCheck: BOOL = 0 as libc::c_int;
static mut DriveControlEnabled: BOOL = 0 as libc::c_int;
static mut DriveInterfaceEnabled: BOOL = 0 as libc::c_int;
static mut IdleTime: UDWORD = 0;
static mut TacticalActive: BOOL = 0 as libc::c_int;
static mut WasDriving: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ControlMode: UWORD = CONTROLMODE_DRIVE as libc::c_int as UWORD;
#[no_mangle]
pub static mut TargetFeatures: BOOL = 0 as libc::c_int;
// Intialise drive statics, call with TRUE if coming from frontend, FALSE if
// coming from a mission.
//
#[no_mangle]
pub unsafe extern "C" fn driveInitVars(mut Restart: BOOL) {
    if WasDriving != 0 && Restart == 0 {
        debug(LOG_NEVER,
              b"driveInitVars: WasDriving\n\x00" as *const u8 as
                  *const libc::c_char); //FALSE;
        DrivingAudioTrack = -(1 as libc::c_int) as SWORD; //TRUE;
        psDrivenDroid = 0 as *mut DROID; //FALSE;
        DoFollowRangeCheck = 1 as libc::c_int;
        ClearFollowRangeCheck = 0 as libc::c_int;
        bDriveMode = 0 as libc::c_int;
        DriveControlEnabled = 1 as libc::c_int;
        DriveInterfaceEnabled = 0 as libc::c_int;
        TacticalActive = 0 as libc::c_int;
        ControlMode = CONTROLMODE_DRIVE as libc::c_int as UWORD;
        TargetFeatures = 0 as libc::c_int
    } else {
        debug(LOG_NEVER,
              b"driveInitVars: Driving\n\x00" as *const u8 as
                  *const libc::c_char);
        DrivingAudioTrack = -(1 as libc::c_int) as SWORD;
        psDrivenDroid = 0 as *mut DROID;
        DoFollowRangeCheck = 1 as libc::c_int;
        ClearFollowRangeCheck = 0 as libc::c_int;
        bDriveMode = 0 as libc::c_int;
        DriveControlEnabled = 1 as libc::c_int;
        DriveInterfaceEnabled = 0 as libc::c_int;
        TacticalActive = 0 as libc::c_int;
        ControlMode = CONTROLMODE_DRIVE as libc::c_int as UWORD;
        TargetFeatures = 0 as libc::c_int;
        WasDriving = 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn controlModeGet() -> UWORD { return ControlMode; }
#[no_mangle]
pub unsafe extern "C" fn controlModeSet(mut Mode: UWORD) {
    ControlMode = Mode;
}
#[no_mangle]
pub unsafe extern "C" fn setDrivingStatus(mut val: BOOL) { bDriveMode = val; }
#[no_mangle]
pub unsafe extern "C" fn getDrivingStatus() -> BOOL { return bDriveMode; }
// Start droid driving mode.
//
#[no_mangle]
pub unsafe extern "C" fn StartDriverMode(mut psOldDroid: *mut DROID) -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psLastDriven: *mut DROID = 0 as *mut DROID;
    //	DBPRINTF(("StartDriveMode\n"));
    IdleTime = gameTime;
    psLastDriven = psDrivenDroid;
    psDrivenDroid = 0 as *mut DROID;
    // Find a selected droid and make that the driven one.
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 {
            if psDrivenDroid.is_null() && psDroid != psOldDroid {
                // The first droid found becomes the driven droid.
                !(DroidIsBuilding(psDroid) != 0 ||
                      DroidGoingToBuild(psDroid) != 0);
                psDrivenDroid = psDroid;
                debug(LOG_NEVER,
                      b"New driven droid\n\x00" as *const u8 as
                          *const libc::c_char);
            } else { (psDroid) != psDrivenDroid; }
        }
        psDroid = (*psDroid).psNext
    }
    // If that failed then find any droid and make it the driven one.
    if psDrivenDroid.is_null() {
        psLastDriven = 0 as *mut DROID;
        psDrivenDroid =
            intGotoNextDroidType(0 as *mut DROID,
                                 DROID_ANY as libc::c_int as UDWORD,
                                 1 as libc::c_int);
        //		DBPRINTF(("Selected a new driven droid : %p\n",psDrivenDroid));
        if psDrivenDroid == psOldDroid {
            psDrivenDroid =
                intGotoNextDroidType(0 as *mut DROID,
                                     DROID_ANY as libc::c_int as UDWORD,
                                     1 as libc::c_int)
        }
        if psDrivenDroid == psOldDroid { psDrivenDroid = 0 as *mut DROID }
        if psDrivenDroid.is_null() {
            psDrivenDroid =
                intGotoNextDroidType(0 as *mut DROID,
                                     DROID_TRANSPORTER as libc::c_int as
                                         UDWORD, 1 as libc::c_int)
        }
    }
    if !psDrivenDroid.is_null() {
        driveDir =
            (*psDrivenDroid).direction as libc::c_int % 360 as libc::c_int;
        driveSpeed = 0 as libc::c_int;
        driveBumpTime = gameTime as libc::c_int;
        setDrivingStatus(1 as libc::c_int);
        if DriveInterfaceEnabled != 0 {
            debug(LOG_NEVER,
                  b"Interface enabled1 ! Disabling drive control\n\x00" as
                      *const u8 as *const libc::c_char);
            DriveControlEnabled = 0 as libc::c_int
        } else { DriveControlEnabled = 1 as libc::c_int }
        if psLastDriven != psDrivenDroid {
            debug(LOG_NEVER,
                  b"camAllignWithTarget\n\x00" as *const u8 as
                      *const libc::c_char);
            camAllignWithTarget(psDrivenDroid as *mut BASE_OBJECT);
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn StopEngineNoise() {
    DrivingAudioTrack = -(1 as libc::c_int) as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn ChangeDriver() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if !psDrivenDroid.is_null() {
        debug(LOG_NEVER,
              b"Driver Changed\n\x00" as *const u8 as *const libc::c_char);
        // If it's the same droid then try again
        // If it failed then try for a transporter.
        //		audio_StopObjTrack(psDrivenDroid,ID_SOUND_SMALL_DROID_RUN);
        //		psDrivenDroid = NULL;
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).sMove.Status as libc::c_int == 9 as libc::c_int {
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Tried to control a transporter\x00" as *const u8
                              as *const libc::c_char);
                };
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"drive.c\x00" as *const u8 as *const libc::c_char,
                          275 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 13],
                                                    &[libc::c_char; 13]>(b"ChangeDriver\x00")).as_ptr(),
                          b"(psDroid->droidType != DROID_TRANSPORTER)\x00" as
                              *const u8 as *const libc::c_char);
                };
                secondarySetState(psDroid, DSO_HALTTYPE, DSS_HALT_GUARD);
                (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
            }
            psDroid = (*psDroid).psNext
        }
    };
    //	#ifdef PSX
//	MouseMovement(TRUE);
//	#endif
//	setDrivingStatus(FALSE);
//	DriveControlEnabled = FALSE;
}
// Stop droid driving mode.
//
#[no_mangle]
pub unsafe extern "C" fn StopDriverMode() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if !psDrivenDroid.is_null() {
        debug(LOG_NEVER,
              b"Drive mode canceled\n\x00" as *const u8 as
                  *const libc::c_char);
        //		audio_StopObjTrack(psDrivenDroid,ID_SOUND_SMALL_DROID_RUN);
        psDrivenDroid = 0 as *mut DROID;
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).sMove.Status as libc::c_int == 9 as libc::c_int {
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Tried to control a transporter\x00" as *const u8
                              as *const libc::c_char);
                };
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"drive.c\x00" as *const u8 as *const libc::c_char,
                          305 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"StopDriverMode\x00")).as_ptr(),
                          b"(psDroid->droidType != DROID_TRANSPORTER)\x00" as
                              *const u8 as *const libc::c_char);
                };
                secondarySetState(psDroid, DSO_HALTTYPE, DSS_HALT_GUARD);
                (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
            }
            psDroid = (*psDroid).psNext
        }
    }
    setDrivingStatus(0 as libc::c_int);
    DriveControlEnabled = 0 as libc::c_int;
}
// Returns TRUE if we have a driven droid.
//
//INLINED BOOL driveHasDriven(void)
//INLINED {
//INLINED 	return (DirectControl) && (psDrivenDroid != NULL) ? TRUE : FALSE;
//INLINED }
// Returns TRUE if drive mode is active.
//
//INLINED BOOL driveModeActive(void)
//INLINED {
//INLINED //DBPRINTF(("%d\n",DirectControl);
//INLINED #ifdef DRIVEFIXED
//INLINED 	return DirectControl;
//INLINED #else
//INLINED 	return psDrivenDroid != NULL ? TRUE : FALSE;
//INLINED #endif
//INLINED }
// Return TRUE if the specified droid is the driven droid.
//
//INLINED BOOL driveIsDriven(DROID *psDroid)
//INLINED {
//INLINED #ifdef DRIVEFIXED
//INLINED 	return (DirectControl) && (psDrivenDroid != NULL) && (psDroid == psDrivenDroid) ? TRUE : FALSE;
//INLINED #else
//INLINED 	return (psDrivenDroid != NULL) && (psDroid == psDrivenDroid) ? TRUE : FALSE;
//INLINED #endif
//INLINED }
//INLINED BOOL driveIsFollower(DROID *psDroid)
//INLINED {
//INLINED #ifdef DRIVEFIXED
//INLINED 	return (DirectControl) && (psDrivenDroid != NULL) && (psDroid != psDrivenDroid) && psDroid->selected ? TRUE : FALSE;
//INLINED #else
//INLINED 	return (psDrivenDroid != NULL) && (psDroid != psDrivenDroid) && psDroid->selected ? TRUE : FALSE;
//INLINED #endif
//INLINED }
//INLINED DROID *driveGetDriven(void)
//INLINED {
//INLINED 	return psDrivenDroid;
//INLINED }
// Call this whenever a droid gets killed or removed.
// returns TRUE if ok, returns FALSE if resulted in driving mode being stopped, ie could'nt find
// a selected droid to drive.
//
#[no_mangle]
pub unsafe extern "C" fn driveDroidKilled(mut psDroid: *mut DROID) -> BOOL {
    //	DROID *NewDroid;
    if driveModeActive() != 0 {
        if psDroid == psDrivenDroid {
            ChangeDriver();
            //		StopDriverMode();
            psDrivenDroid = 0 as *mut DROID;
            //			psDroid->selected = FALSE;
            DeSelectDroid(psDroid);
            if StartDriverMode(psDroid) == 0 {
                //			NewDroid = intGotoNextDroidType(NULL,DROID_ANY,TRUE);
	//
	//			DBPRINTF(("Droid Killed %p new %p\n",psDroid,NewDroid));
	//
	//			if(NewDroid == psDroid) {
	//				NewDroid = intGotoNextDroidType(NULL,DROID_ANY,TRUE);
	//				DBPRINTF(("Droid Killed %p new %p\n",psDroid,NewDroid));
	//			}
	//
	//			if((!StartDriverMode()) || (NewDroid == psDroid)) {
	//#ifdef PSX
	//				// Failed to find a droid to track!
	//				DBPRINTF(("No droid to drive!\n"));
	//
	//		DBPRINTF(("no droids, find a structure\n");
	//				if(StartObjectOrbit((BASE_OBJECT*)intFindAStructure())) {
	//				} else {
	//		DBPRINTF(("no structures or droids, should be game over!\n");
	//				}
	//#endif
                return 0 as libc::c_int
                //			}
            }
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn driveDroidIsBusy(mut psDroid: *mut DROID) -> BOOL {
    if DroidIsBuilding(psDroid) != 0 || DroidGoingToBuild(psDroid) != 0 ||
           DroidIsDemolishing(psDroid) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Call whenever the selections change to re initialise drive mode.
//
#[no_mangle]
pub unsafe extern "C" fn driveSelectionChanged() {
    if driveModeActive() != 0 {
        if !psDrivenDroid.is_null() {
            //		StopDriverMode();
            ChangeDriver();
            StartDriverMode(0 as *mut DROID);
            driveTacticalSelectionChanged();
        }
    };
}
// Cycle to next droid in group and make it the driver.
//
#[no_mangle]
pub unsafe extern "C" fn driveNextDriver() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //	BOOL Found = FALSE;
    //DBPRINTF(("driveNextDriver %p\n",psDrivenDroid);
    // Start from the current driven droid.
    psDroid = psDrivenDroid;
    while !psDroid.is_null() {
        if (*psDroid).selected as libc::c_int != 0 && psDroid != psDrivenDroid
           {
            // Found one so make it the driven droid.
//			psDrivenDroid->sMove.Status = MOVEINACTIVE;
//			psDroid->sMove.Status = MOVEDRIVE;
            psDrivenDroid = psDroid;
            camAllignWithTarget(psDroid as *mut BASE_OBJECT);
            driveTacticalSelectionChanged();
            return
        }
        psDroid = (*psDroid).psNext
    }
    //	if(!Found) {
		// Not found so start at the begining.
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() && psDroid != psDrivenDroid {
        if (*psDroid).selected != 0 {
            // Found one so make it the driven droid.
//				psDrivenDroid->sMove.Status = MOVEINACTIVE;
//				psDroid->sMove.Status = MOVEDRIVE;
            psDrivenDroid = psDroid;
            camAllignWithTarget(psDroid as *mut BASE_OBJECT);
            driveTacticalSelectionChanged();
            return
        }
        psDroid = (*psDroid).psNext
    };
    //	}
	// No other selected droids found. Oh well...
}
// PC Version...
unsafe extern "C" fn driveControl(mut psDroid: *mut DROID) -> BOOL {
    let mut Input: BOOL = 0 as libc::c_int;
    let mut MaxSpeed: SDWORD = moveCalcDroidSpeed(psDroid);
    if DriveControlEnabled == 0 { return 0 as libc::c_int }
    if keyPressed(KEY_N) != 0 { driveNextDriver(); }
    if keyDown(KEY_LEFTARROW) != 0 {
        driveDir += 4 as libc::c_int;
        Input = 1 as libc::c_int
    } else if keyDown(KEY_RIGHTARROW) != 0 {
        driveDir -= 4 as libc::c_int;
        if driveDir < 0 as libc::c_int { driveDir += 360 as libc::c_int }
        Input = 1 as libc::c_int
    }
    driveDir = driveDir % 360 as libc::c_int;
    if keyDown(KEY_UPARROW) != 0 {
        if driveSpeed >= 0 as libc::c_int {
            driveSpeed += 16 as libc::c_int;
            if driveSpeed > MaxSpeed { driveSpeed = MaxSpeed }
        } else {
            driveSpeed += 32 as libc::c_int;
            if driveSpeed > 0 as libc::c_int { driveSpeed = 0 as libc::c_int }
        }
        Input = 1 as libc::c_int
    } else if keyDown(KEY_DOWNARROW) != 0 {
        if driveSpeed <= 0 as libc::c_int {
            driveSpeed -= 16 as libc::c_int;
            if driveSpeed < -MaxSpeed { driveSpeed = -MaxSpeed }
        } else {
            driveSpeed -= 32 as libc::c_int;
            if driveSpeed < 0 as libc::c_int { driveSpeed = 0 as libc::c_int }
        }
        Input = 1 as libc::c_int
    } else if driveSpeed > 0 as libc::c_int {
        driveSpeed -= 16 as libc::c_int;
        if driveSpeed < 0 as libc::c_int { driveSpeed = 0 as libc::c_int }
    } else {
        driveSpeed += 16 as libc::c_int;
        if driveSpeed > 0 as libc::c_int { driveSpeed = 0 as libc::c_int }
    }
    return Input;
}
unsafe extern "C" fn driveInDriverRange(mut psDroid: *mut DROID) -> BOOL {
    if abs((*psDroid).x as libc::c_int - (*psDrivenDroid).x as libc::c_int) <
           256 as libc::c_int &&
           abs((*psDroid).y as libc::c_int -
                   (*psDrivenDroid).y as libc::c_int) < 256 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn driveMoveFollower(mut psDroid: *mut DROID) {
    if (driveBumpTime as libc::c_uint) < gameTime {
        // Update the driven droid's followers.
        if driveInDriverRange(psDroid) == 0 {
            //psDroid->secondaryOrder&=~DSS_MOVEHOLD_SET;		// Remove secondary order ... this stops the droid from jumping back to GUARD mode ... see order.c #111 - tjc
            secondarySetState(psDroid, DSO_HALTTYPE, DSS_HALT_GUARD);
            // if the droid is currently guarding we need to change the order to a move
            if (*psDroid).order == DORDER_GUARD as libc::c_int {
                orderDroidLoc(psDroid, DORDER_MOVE,
                              (*psDrivenDroid).x as UDWORD,
                              (*psDrivenDroid).y as UDWORD);
            } else {
                // otherwise we just adjust its move-to location
                moveDroidTo(psDroid, (*psDrivenDroid).x as UDWORD,
                            (*psDrivenDroid).y as UDWORD);
            }
        }
    }
    // Stop it when it gets within range of the driver.
    if DoFollowRangeCheck != 0 {
        if driveInDriverRange(psDroid) != 0 {
            (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
        } else { AllInRange = 0 as libc::c_int }
    };
}
unsafe extern "C" fn driveMoveCommandFollowers(mut psDroid: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psGroup: *mut DROID_GROUP = (*psDroid).psGroup;
    psCurr = (*psGroup).psList;
    while !psCurr.is_null() {
        driveMoveFollower(psCurr);
        psCurr = (*psCurr).psGrpNext
    };
}
// Call once per frame.
//
#[no_mangle]
pub unsafe extern "C" fn driveUpdate() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    AllInRange = 1 as libc::c_int;
    if DirectControl != 0 {
        if !psDrivenDroid.is_null() {
            if bMultiPlayer != 0 && (driveBumpTime as libc::c_uint) < gameTime
               {
                // send latest info about driven droid.
                SendDroidInfo(psDrivenDroid, DORDER_MOVE,
                              (*psDrivenDroid).x as UDWORD,
                              (*psDrivenDroid).y as UDWORD,
                              0 as *mut BASE_OBJECT);
            }
            //TO BE DONE:
		//clear the order on taking over the droid, to stop attacks..
		//send some sort of message when droids stopo and get inrange.
            // Check the driven droid is still selected
            if (*psDrivenDroid).selected as libc::c_int == 0 as libc::c_int {
                // if it's not then reset the driving system.
                driveSelectionChanged();
                return
            }
            // Update the driven droid.
            if driveControl(psDrivenDroid) != 0 {
                // If control did something then force the droid's move status.
                if (*psDrivenDroid).sMove.Status as libc::c_int !=
                       9 as libc::c_int {
                    (*psDrivenDroid).sMove.Status = 9 as libc::c_int as UBYTE;
                    if (*psDrivenDroid).droidType as libc::c_uint !=
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"Tried to control a transporter\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if (*psDrivenDroid).droidType as libc::c_uint !=
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"drive.c\x00" as *const u8 as
                                  *const libc::c_char, 655 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 12],
                                                        &[libc::c_char; 12]>(b"driveUpdate\x00")).as_ptr(),
                              b"(psDrivenDroid->droidType != DROID_TRANSPORTER)\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    driveDir =
                        (*psDrivenDroid).direction as libc::c_int %
                            360 as libc::c_int
                }
                DoFollowRangeCheck = 1 as libc::c_int
            }
            // Is the driven droid under user control?
            if (*psDrivenDroid).sMove.Status as libc::c_int ==
                   9 as libc::c_int {
                // Is it a command droid
                if (*psDrivenDroid).droidType as libc::c_uint ==
                       DROID_COMMAND as libc::c_int as libc::c_uint &&
                       !(*psDrivenDroid).psGroup.is_null() {
                    driveMoveCommandFollowers(psDrivenDroid);
                }
                psDroid = apsDroidLists[selectedPlayer as usize];
                while !psDroid.is_null() {
                    psPropStats =
                        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       usize].nStat
                                                     as libc::c_int as isize);
                    if (*psDroid).selected as libc::c_int != 0 &&
                           psDroid != psDrivenDroid &&
                           (*psDroid).droidType as libc::c_uint !=
                               DROID_TRANSPORTER as libc::c_int as
                                   libc::c_uint &&
                           ((*psPropStats).propulsionType as libc::c_int !=
                                LIFT as libc::c_int ||
                                cyborgDroid(psDroid) != 0) {
                        // Send new orders to it's followers.
                        driveMoveFollower(psDroid);
                    }
                    psDroid = (*psDroid).psNext
                }
            }
            if AllInRange != 0 { DoFollowRangeCheck = 0 as libc::c_int }
            if (driveBumpTime as libc::c_uint) < gameTime {
                // Send next order in 1 second.
                driveBumpTime =
                    gameTime.wrapping_add(1000 as libc::c_int as libc::c_uint)
                        as libc::c_int
            }
        } else {
            // Start driving
//DBPRINTF(("StartDriveMode\n");
            (StartDriverMode(0 as *mut DROID)) == 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn driveGetMoveSpeed() -> SDWORD { return driveSpeed; }
#[no_mangle]
pub unsafe extern "C" fn driveGetMoveDir() -> SDWORD { return driveDir; }
#[no_mangle]
pub unsafe extern "C" fn driveSetDroidMove(mut psDroid: *mut DROID) {
    //	psDroid->sMove.speed = MAKEFRACT(driveSpeed);
//	psDroid->sMove.dir = driveDir;
    (*psDroid).direction = driveDir as UWORD;
}
// Dissable user control of droid, ie when interface is up.
#[no_mangle]
pub unsafe extern "C" fn driveDisableControl() {
    DriveControlEnabled = 0 as libc::c_int;
}
// Enable user control of droid, ie when interface is down.
//
#[no_mangle]
pub unsafe extern "C" fn driveEnableControl() {
    DriveControlEnabled = 1 as libc::c_int;
}
// Return TRUE if drive control is enabled.
//
#[no_mangle]
pub unsafe extern "C" fn driveControlEnabled() -> BOOL {
    return DriveControlEnabled;
}
// Bring up the reticule.
//
#[no_mangle]
pub unsafe extern "C" fn driveEnableInterface(mut AddReticule: BOOL) {
    if AddReticule != 0 { intAddReticule(); }
    DriveInterfaceEnabled = 1 as libc::c_int;
}
// Get rid of the reticule.
//
#[no_mangle]
pub unsafe extern "C" fn driveDisableInterface() {
    intResetScreen(0 as libc::c_int);
    intRemoveReticule();
    DriveInterfaceEnabled = 0 as libc::c_int;
}
// Get rid of the reticule.
//
#[no_mangle]
pub unsafe extern "C" fn driveDisableInterface2() {
    DriveInterfaceEnabled = 0 as libc::c_int;
}
// Return TRUE if the reticule is up.
//
#[no_mangle]
pub unsafe extern "C" fn driveInterfaceEnabled() -> BOOL {
    return DriveInterfaceEnabled;
}
// Check for and process a user request for a new target.
//
#[no_mangle]
pub unsafe extern "C" fn driveProcessAquireButton() {
    if mouseReleased(MOUSE_RMB) != 0 || keyPressed(KEY_S) != 0 {
        let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
        //		psObj = targetAquireNext(TARGET_TYPE_ANY);
//		psObj = targetAquireNearestObj(targetGetCrosshair(),TARGET_TYPE_ANY);
        psObj =
            targetAquireNearestObjView(psDrivenDroid as *mut BASE_OBJECT,
                                       0xffff as libc::c_int as UWORD)
    };
}
// Start structure placement for drive mode.
//
#[no_mangle]
pub unsafe extern "C" fn driveStartBuild() {
    intRemoveReticule();
    DriveInterfaceEnabled = 0 as libc::c_int;
    //	driveDisableInterface();
    driveEnableControl();
}
#[no_mangle]
pub unsafe extern "C" fn driveStartDemolish() {
    intRemoveReticule();
    DriveInterfaceEnabled = 0 as libc::c_int;
    driveEnableControl();
}
// Stop structure placement for drive mode.
//
#[no_mangle]
pub unsafe extern "C" fn driveStopBuild() { }
// Return TRUE if all the conditions for allowing user control of the droid are met.
//
#[no_mangle]
pub unsafe extern "C" fn driveAllowControl() -> BOOL {
    if TacticalActive != 0 || DriveInterfaceEnabled != 0 ||
           DriveControlEnabled == 0 {
        //		if( TacticalActive )	DBPRINTF(("TacticalActive\n");
//		if( DriveInterfaceEnabled )	DBPRINTF(("DriveInterfaceEnabled\n");
//		if( DeliveryReposValid() ) DBPRINTF(("DeliveryReposValid\n");
//		if( (camGetMode() == CAMMODE_ORBIT) ) DBPRINTF(("CAMMODE_ORBIT\n");
//		if( (!DriveControlEnabled) ) DBPRINTF(("!DriveControlEnabled\n");
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Enable Tactical order mode.
//
#[no_mangle]
pub unsafe extern "C" fn driveEnableTactical() {
    //	MouseMovement(TRUE);
//	SetMouseRange(0,RADTLX,RADTLY,RADTLX+RADWIDTH,RADTLY+RADHEIGHT);
//	SetMousePos(0,RADTLX+RADWIDTH/2,RADTLY+RADHEIGHT/2);
    StartTacticalScroll(1 as libc::c_int);
    TacticalActive = 1 as libc::c_int;
    debug(LOG_NEVER,
          b"Tactical Mode Activated\n\x00" as *const u8 as
              *const libc::c_char);
}
// Disable Tactical order mode.
//
#[no_mangle]
pub unsafe extern "C" fn driveDisableTactical() {
    if driveModeActive() != 0 && TacticalActive != 0 {
        CancelTacticalScroll();
        //	MouseMovement(FALSE);
	//	SetMouseRange(0,16,16,639-16,479-16);
	//	SetMousePos(0,320,160);
        TacticalActive = 0 as libc::c_int;
        debug(LOG_NEVER,
              b"Tactical Mode Canceled\n\x00" as *const u8 as
                  *const libc::c_char);
    };
}
// Return TRUE if Tactical order mode is active.
//
#[no_mangle]
pub unsafe extern "C" fn driveTacticalActive() -> BOOL {
    return TacticalActive;
}
#[no_mangle]
pub unsafe extern "C" fn driveTacticalSelectionChanged() {
    if TacticalActive != 0 && !psDrivenDroid.is_null() {
        StartTacticalScrollObj(1 as libc::c_int,
                               psDrivenDroid as *mut BASE_OBJECT);
        debug(LOG_NEVER,
              b"driveTacticalSelectionChanged\n\x00" as *const u8 as
                  *const libc::c_char);
    };
}
// Player clicked in the radar window.
//
#[no_mangle]
pub unsafe extern "C" fn driveProcessRadarInput(mut x: libc::c_int,
                                                mut y: libc::c_int) {
    let mut PosX: SDWORD = 0;
    let mut PosY: SDWORD = 0;
    // when drive mode is active, clicking on the radar orders all selected droids
	// to move to this position.
    CalcRadarPosition(x as UDWORD, y as UDWORD,
                      &mut PosX as *mut SDWORD as *mut UDWORD,
                      &mut PosY as *mut SDWORD as *mut UDWORD);
    orderSelectedLoc(selectedPlayer, (PosX * 128 as libc::c_int) as UDWORD,
                     (PosY * 128 as libc::c_int) as UDWORD);
}
