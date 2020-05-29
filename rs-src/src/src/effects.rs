use ::libc;
extern "C" {
    pub type _formation;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
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
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    /* Return the resource for a type and ID */
    #[no_mangle]
    fn resGetDataFromHash(pType: *mut STRING, HashedID: UDWORD)
     -> *mut libc::c_void;
    // return the HashedID string for a piece of data
    #[no_mangle]
    fn resGetHashfromData(pType: *mut STRING, pData: *mut libc::c_void,
                          pHash: *mut UDWORD) -> BOOL;
    /* Returns the current frame we're on - used to establish whats on screen */
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    // The size of this buffer
    /* Save the data in the buffer into the given file */
    #[no_mangle]
    fn saveFile(pFileName: *const libc::c_char,
                pFileData: *const libc::c_char, fileSize: UDWORD) -> BOOL;
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Draw3DShape(shape: *mut iIMDShape, frame: libc::c_int,
                       team: libc::c_int, colour: UDWORD, specular: UDWORD,
                       pieFlag: libc::c_int, pieData: libc::c_int);
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatScale(percent: UDWORD);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    #[no_mangle]
    fn audio_PlayStaticTrack(iX: SDWORD, iY: SDWORD, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjStaticTrack(psObj: *mut libc::c_void, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn getCentreZ() -> SDWORD;
    #[no_mangle]
    fn getCentreX() -> SDWORD;
    #[no_mangle]
    static mut terrainMidY: UDWORD;
    #[no_mangle]
    static mut terrainMidX: UDWORD;
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    /* The current tutorial message - there is only ever one at a time. They are displayed 
when called by the script. They are not to be re-displayed*/
//extern MESSAGE		tutorialMessage;
/* The IMD to use for the proximity messages */
    #[no_mangle]
    static mut pProximityMsgIMD: *mut iIMDShape;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    /* add an object to the current render list */
    #[no_mangle]
    fn bucketAddTypeToList(objectType: RENDER_TYPE, object: *mut libc::c_void)
     -> BOOL;
    #[no_mangle]
    fn getCampaignNumber() -> UDWORD;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
    #[no_mangle]
    fn getRandomDebrisImd() -> *mut iIMDShape;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn processLight(psLight: *mut LIGHT);
    #[no_mangle]
    fn lightDoFogAndIllumination(brightness: UBYTE, dx: SDWORD, dz: SDWORD,
                                 pSpecular: *mut UDWORD) -> UDWORD;
    #[no_mangle]
    fn gamePaused() -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn isHumanPlayer(player_0: UDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
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
/*
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
pub type KEY_CODE = _key_code;
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
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
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
pub type STRUCT_STRENGTH = _struct_strength;
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
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
//	SDWORD		XCoordinate,YCoordinate;
pub type MOVE_CONTROL = _move_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
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
pub type WEAPON = _weapon;
pub type BASE_OBJECT = _base_object;
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
// The stats for the weapon type
// When the weapon last fired
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
pub type STRUCTURE = _structure;
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
pub type _render_type = libc::c_uint;
pub const RENDER_PARTICLE: _render_type = 16;
pub const RENDER_DELIVPOINT: _render_type = 15;
pub const RENDER_MIST: _render_type = 14;
pub const RENDER_WATERTILE: _render_type = 13;
pub const RENDER_TILE: _render_type = 12;
pub const RENDER_SMOKE: _render_type = 11;
pub const RENDER_GRAVITON: _render_type = 10;
pub const RENDER_EFFECT: _render_type = 9;
pub const RENDER_EXPLOSION: _render_type = 8;
pub const RENDER_ANIMATION: _render_type = 7;
pub const RENDER_SHADOW: _render_type = 6;
pub const RENDER_PROJECTILE_TRANSPARENT: _render_type = 5;
pub const RENDER_PROJECTILE: _render_type = 4;
pub const RENDER_PROXMSG: _render_type = 3;
pub const RENDER_FEATURE: _render_type = 2;
pub const RENDER_STRUCTURE: _render_type = 1;
pub const RENDER_DROID: _render_type = 0;
/*owning power generator*/
/*pointers to the res ext
																associated with this gen*/
/* bucket3D.h */
pub type RENDER_TYPE = _render_type;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MI_TOO_MANY: C2RustUnnamed_0 = 44;
pub const MI_FIREWORK: C2RustUnnamed_0 = 43;
pub const MI_DEBRIS4: C2RustUnnamed_0 = 42;
pub const MI_DEBRIS3: C2RustUnnamed_0 = 41;
pub const MI_DEBRIS2: C2RustUnnamed_0 = 40;
pub const MI_DEBRIS1: C2RustUnnamed_0 = 39;
pub const MI_DEBRIS0: C2RustUnnamed_0 = 38;
pub const MI_WRECK4: C2RustUnnamed_0 = 37;
pub const MI_WRECK3: C2RustUnnamed_0 = 36;
pub const MI_WRECK2: C2RustUnnamed_0 = 35;
pub const MI_WRECK1: C2RustUnnamed_0 = 34;
pub const MI_WRECK0: C2RustUnnamed_0 = 33;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed_0 = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed_0 = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed_0 = 30;
pub const MI_SHOCK: C2RustUnnamed_0 = 29;
pub const MI_LANDING: C2RustUnnamed_0 = 28;
pub const MI_KICK: C2RustUnnamed_0 = 27;
pub const MI_SPLASH: C2RustUnnamed_0 = 26;
pub const MI_SNOW: C2RustUnnamed_0 = 25;
pub const MI_RAIN: C2RustUnnamed_0 = 24;
pub const MI_MFLARE: C2RustUnnamed_0 = 23;
pub const MI_TESLA: C2RustUnnamed_0 = 22;
pub const MI_FLAME: C2RustUnnamed_0 = 21;
pub const MI_TRAIL: C2RustUnnamed_0 = 20;
pub const MI_BLOOD: C2RustUnnamed_0 = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed_0 = 18;
pub const MI_SHADOW: C2RustUnnamed_0 = 17;
pub const MI_BLIP: C2RustUnnamed_0 = 16;
pub const MI_PLASMA: C2RustUnnamed_0 = 15;
pub const MI_SMALL_STEAM: C2RustUnnamed_0 = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed_0 = 13;
pub const MI_WATER: C2RustUnnamed_0 = 12;
pub const MI_CYBORG_BODY: C2RustUnnamed_0 = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed_0 = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed_0 = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed_0 = 8;
pub const MI_BABA_BODY: C2RustUnnamed_0 = 7;
pub const MI_BABA_ARM: C2RustUnnamed_0 = 6;
pub const MI_BABA_LEGS: C2RustUnnamed_0 = 5;
pub const MI_BABA_HEAD: C2RustUnnamed_0 = 4;
pub const MI_SMALL_SMOKE: C2RustUnnamed_0 = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed_0 = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed_0 = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed_0 = 0;
pub type EFFECT_GROUP = libc::c_uint;
pub const EFFECT_FIREWORK: EFFECT_GROUP = 11;
pub const EFFECT_FIRE: EFFECT_GROUP = 10;
pub const EFFECT_DUST_BALL: EFFECT_GROUP = 9;
pub const EFFECT_SAT_LASER: EFFECT_GROUP = 8;
pub const EFFECT_DESTRUCTION: EFFECT_GROUP = 7;
pub const EFFECT_BLOOD: EFFECT_GROUP = 6;
pub const EFFECT_WAYPOINT: EFFECT_GROUP = 5;
pub const EFFECT_GRAVITON: EFFECT_GROUP = 4;
pub const EFFECT_STRUCTURE: EFFECT_GROUP = 3;
pub const EFFECT_SMOKE: EFFECT_GROUP = 2;
pub const EFFECT_CONSTRUCTION: EFFECT_GROUP = 1;
pub const EFFECT_EXPLOSION: EFFECT_GROUP = 0;
pub type EFFECT_TYPE = libc::c_uint;
pub const FIREWORK_TYPE_LAUNCHER: EFFECT_TYPE = 42;
pub const FIREWORK_TYPE_STARBURST: EFFECT_TYPE = 41;
pub const WAYPOINT_TYPE: EFFECT_TYPE = 40;
pub const SAT_LASER_STANDARD: EFFECT_TYPE = 39;
pub const DESTRUCTION_TYPE_SKYSCRAPER: EFFECT_TYPE = 38;
pub const DESTRUCTION_TYPE_FEATURE: EFFECT_TYPE = 37;
pub const DESTRUCTION_TYPE_WALL_SECTION: EFFECT_TYPE = 36;
pub const DESTRUCTION_TYPE_POWER_STATION: EFFECT_TYPE = 35;
pub const DESTRUCTION_TYPE_STRUCTURE: EFFECT_TYPE = 34;
pub const DESTRUCTION_TYPE_DROID: EFFECT_TYPE = 33;
pub const DUST_TYPE_NORMAL: EFFECT_TYPE = 32;
pub const BLOOD_TYPE_NORMAL: EFFECT_TYPE = 31;
pub const CONSTRUCTION_TYPE_DRIFTING: EFFECT_TYPE = 30;
pub const FIRE_TYPE_SMOKY_BLUE: EFFECT_TYPE = 29;
pub const FIRE_TYPE_SMOKY: EFFECT_TYPE = 28;
pub const FIRE_TYPE_LOCALISED: EFFECT_TYPE = 27;
pub const SMOKE_TYPE_TRAIL: EFFECT_TYPE = 26;
pub const SMOKE_TYPE_STEAM: EFFECT_TYPE = 25;
pub const SMOKE_TYPE_BILLOW: EFFECT_TYPE = 24;
pub const SMOKE_TYPE_DRIFTING_SMALL: EFFECT_TYPE = 23;
pub const SMOKE_TYPE_DRIFTING_HIGH: EFFECT_TYPE = 22;
pub const SMOKE_TYPE_DRIFTING: EFFECT_TYPE = 21;
pub const GRAVITON_TYPE_GIBLET: EFFECT_TYPE = 20;
pub const GRAVITON_TYPE_EMITTING_ST: EFFECT_TYPE = 19;
pub const GRAVITON_TYPE_EMITTING_DR: EFFECT_TYPE = 18;
pub const GRAVITON_TYPE_STANDARD: EFFECT_TYPE = 17;
pub const EXPLOSION_TYPE_SHOCKWAVE: EFFECT_TYPE = 16;
pub const EXPLOSION_TYPE_LAND_LIGHT: EFFECT_TYPE = 15;
pub const EXPLOSION_TYPE_KICKUP: EFFECT_TYPE = 14;
pub const EXPLOSION_TYPE_PLASMA: EFFECT_TYPE = 13;
pub const EXPLOSION_TYPE_FLARE: EFFECT_TYPE = 12;
pub const EXPLOSION_TYPE_DISCOVERY: EFFECT_TYPE = 11;
pub const EXPLOSION_TYPE_TESLA: EFFECT_TYPE = 10;
pub const EXPLOSION_TYPE_LASER: EFFECT_TYPE = 9;
pub const EXPLOSION_TYPE_FLAMETHROWER: EFFECT_TYPE = 8;
pub const EXPLOSION_TYPE_SPECIFIED_FIXME: EFFECT_TYPE = 7;
pub const EXPLOSION_TYPE_SPECIFIED_SOLID: EFFECT_TYPE = 6;
pub const EXPLOSION_TYPE_NOT_FACING: EFFECT_TYPE = 5;
pub const EXPLOSION_TYPE_SPECIFIED: EFFECT_TYPE = 4;
pub const EXPLOSION_TYPE_LARGE: EFFECT_TYPE = 3;
pub const EXPLOSION_TYPE_MEDIUM: EFFECT_TYPE = 2;
pub const EXPLOSION_TYPE_VERY_SMALL: EFFECT_TYPE = 1;
pub const EXPLOSION_TYPE_SMALL: EFFECT_TYPE = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ES_DORMANT: C2RustUnnamed_1 = 2;
pub const ES_ACTIVE: C2RustUnnamed_1 = 1;
pub const ES_INACTIVE: C2RustUnnamed_1 = 0;
pub type LAND_LIGHT_SPEC = libc::c_uint;
pub const LL_OUTER: LAND_LIGHT_SPEC = 2;
pub const LL_INNER: LAND_LIGHT_SPEC = 1;
pub const LL_MIDDLE: LAND_LIGHT_SPEC = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _effect_def {
    pub status: UBYTE,
    pub control: UBYTE,
    pub group: UBYTE,
    pub type_0: UBYTE,
    pub frameNumber: UBYTE,
    pub size: UWORD,
    pub baseScale: UBYTE,
    pub specific: UBYTE,
    pub position: PIEVECTORF,
    pub velocity: PIEVECTORF,
    pub rotation: iVector,
    pub spin: iVector,
    pub birthTime: UDWORD,
    pub lastFrame: UDWORD,
    pub frameDelay: UWORD,
    pub lifeSpan: UWORD,
    pub radius: UWORD,
    pub imd: *mut iIMDShape,
}
// 3.5 seconds
/*
	Definition of an 'effect'. This'll need to come down in size
	for the PlayStation?
*/
pub type EFFECT = _effect_def;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_2 = 296;
pub const ID_SOUND_STEAM: C2RustUnnamed_2 = 297;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_2 = 271;
pub type LIGHT = _light;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _light {
    pub position: iVector,
    pub type_0: UBYTE,
    pub range: UDWORD,
    pub colour: UDWORD,
}
pub const LIGHT_BLUE: _lightcols = 2;
pub const LIGHT_RED: _lightcols = 0;
pub const LIGHT_WHITE: _lightcols = 4;
pub const LIGHT_YELLOW: _lightcols = 3;
pub type FX_SAVEHEADER = _fx_save_header;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fx_save_header {
    pub aFileType: [STRING; 4],
    pub version: UDWORD,
    pub entries: UDWORD,
}
pub type C2RustUnnamed_2 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_2 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_2 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_2 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_2 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_2 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_2 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_2 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_2 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_2 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_2 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_2 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_2 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_2 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_2 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_2 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_2 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_2 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_2 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_2 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_2 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_2 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_2 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_2 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_2 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_2 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_2 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_2 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_2 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_2 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_2 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_2 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_2 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_2 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_2 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_2 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_2 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_2 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_2 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_2 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_2 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_2 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_2 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_2 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_2 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_2 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_2 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_2 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_2 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_2 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_2 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_2 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_2 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_2 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_2 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_2 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_2 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_2 = 298;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_2 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_2 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_2 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_2 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_2 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_2 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_2 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_2 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_2 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_2 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_2 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_2 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_2 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_2 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_2 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_2 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_2 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_2 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_2 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_2 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_2 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_2 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_2 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_2 = 272;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_2 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_2 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_2 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_2 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_2 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_2 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_2 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_2 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_2 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_2 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_2 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_2 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_2 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_2 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_2 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_2 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_2 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_2 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_2 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_2 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_2 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_2 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_2 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_2 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_2 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_2 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_2 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_2 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_2 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_2 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_2 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_2 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_2 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_2 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_2 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_2 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_2 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_2 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_2 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_2 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_2 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_2 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_2 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_2 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_2 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_2 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_2 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_2 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_2 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_2 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_2 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_2 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_2 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_2 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_2 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_2 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_2 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_2 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_2 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_2 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_2 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_2 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_2 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_2 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_2 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_2 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_2 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_2 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_2 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_2 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_2 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_2 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_2 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_2 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_2 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_2 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_2 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_2 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_2 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_2 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_2 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_2 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_2 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_2 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_2 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_2 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_2 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_2 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_2 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_2 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_2 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_2 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_2 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_2 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_2 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_2 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_2 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_2 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_2 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_2 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_2 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_2 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_2 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_2 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_2 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_2 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_2 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_2 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_2 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_2 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_2 = 160;
pub const ID_GIFT: C2RustUnnamed_2 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_2 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_2 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_2 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_2 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_2 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_2 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_2 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_2 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_2 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_2 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_2 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_2 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_2 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_2 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_2 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_2 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_2 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_2 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_2 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_2 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_2 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_2 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_2 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_2 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_2 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_2 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_2 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_2 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_2 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_2 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_2 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_2 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_2 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_2 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_2 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_2 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_2 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_2 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_2 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_2 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_2 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_2 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_2 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_2 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_2 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_2 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_2 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_2 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_2 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_2 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_2 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_2 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_2 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_2 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_2 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_2 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_2 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_2 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_2 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_2 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_2 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_2 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_2 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_2 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_2 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_2 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_2 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_2 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_2 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_2 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_2 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_2 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_2 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_2 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_2 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_2 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_2 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_2 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_2 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_2 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_2 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_2 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_2 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_2 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_2 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_2 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_2 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_2 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_2 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_2 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_2 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_2 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_2 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_2 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_2 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_2 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_2 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_2 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_2 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_2 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_2 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_2 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_2 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_2 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_2 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_2 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_2 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_2 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_2 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_2 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_2 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_2 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_2 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_2 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_2 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_2 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_2 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_2 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_2 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_2 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_2 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_2 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_2 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_2 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_2 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_2 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_2 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_2 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_2 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_2 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_2 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_2 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_2 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_2 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_2 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_2 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_2 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_2 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_2 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_2 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_2 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_2 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_2 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_2 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_2 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_2 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_2 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_2 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_2 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_2 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_2 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_2 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_2 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_2 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_2 = 0;
pub const NO_SOUND: C2RustUnnamed_2 = -1;
pub type _lightcols = libc::c_uint;
pub const LIGHT_GREEN: _lightcols = 1;
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
// what status is the present effect - active/inactive/dormant
// Controls the bits above - essential,flips etc
// what	group is it - explosion, building effect etc....
// what type is it within the group?
// what frame number is the imd on?
// Size in terms of percent of original imd.
// if scaled, what's bottom line?
// how many times has it bounced?
// world coordinates of the effect - floats on the PC.
// movement values per update
// current rotation - only for gravitons
// rotation info for spinning things.
// what time was it introduced into the world?
// when did we last update the frame?
// how many game ticks between each frame?
// what is it's life expectancy?
// Used for area effects
// pointer to the imd the effect uses.
/* Our list of all game world effects */
#[no_mangle]
pub static mut asEffectsList: [EFFECT; 2500] =
    [EFFECT{status: 0,
            control: 0,
            group: 0,
            type_0: 0,
            frameNumber: 0,
            size: 0,
            baseScale: 0,
            specific: 0,
            position: PIEVECTORF{x: 0., y: 0., z: 0.,},
            velocity: PIEVECTORF{x: 0., y: 0., z: 0.,},
            rotation: iVector{x: 0, y: 0, z: 0,},
            spin: iVector{x: 0, y: 0, z: 0,},
            birthTime: 0,
            lastFrame: 0,
            frameDelay: 0,
            lifeSpan: 0,
            radius: 0,
            imd: 0 as *const iIMDShape as *mut iIMDShape,}; 2500];
/* Tick counts for updates on a particular interval */
static mut lastUpdateDroids: [UDWORD; 101] = [0; 101];
static mut lastUpdateStructures: [UDWORD; 103] = [0; 103];
/* Current next slot to use - cyclic */
static mut freeEffect: UDWORD = 0;
static mut numEffects: UDWORD = 0;
static mut activeEffects: UDWORD = 0;
static mut missCount: UDWORD = 0;
static mut skipped: UDWORD = 0;
static mut skippedEffects: UDWORD = 0;
static mut letThrough: UDWORD = 0;
static mut auxVar: UDWORD = 0;
// dirty filthy hack - don't look for what this does.... //FIXME
static mut auxVarSec: UDWORD = 0;
// dirty filthy hack - don't look for what this does.... //FIXME
static mut aeCalls: UDWORD = 0;
static mut specifiedSize: UDWORD = 0;
static mut ellSpec: UDWORD = 0;
/* The fraction of a second that the last game frame took */
static mut fraction: FRACT = 0.;
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn essentialEffect(mut group: EFFECT_GROUP,
                                         mut type_0: EFFECT_TYPE) -> BOOL {
    match group as libc::c_uint {
        10 | 5 | 7 | 8 | 3 => { return 1 as libc::c_int }
        0 => {
            if type_0 as libc::c_uint ==
                   EXPLOSION_TYPE_LAND_LIGHT as libc::c_int as libc::c_uint {
                return 1 as libc::c_int
            } else { return 0 as libc::c_int }
        }
        _ => { return 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn utterlyReject(mut group: EFFECT_GROUP,
                                       mut type_0: EFFECT_TYPE) -> BOOL {
    match group as libc::c_uint {
        6 | 9 | 1 => { return 1 as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
// and so's this
// ----------------------------------------------------------------------------------------
/*	Simply sets the free pointer to the start - actually this isn't necessary
	as it will work just fine anyway. This WOULD be necessary were we to change
	the system so that it seeks FREE slots rather than the oldest one. This is because
	different effects last for different times and the oldest effect may have
	just got going (if it was a long effect).
*/
#[no_mangle]
pub unsafe extern "C" fn initEffectsSystem() {
    let mut i: UDWORD = 0;
    let mut psEffect: *mut EFFECT = 0 as *mut EFFECT;
    /* Set position to first */
    freeEffect = 0 as libc::c_int as UDWORD;
    /* None are active */
    numEffects = 0 as libc::c_int as UDWORD;
    activeEffects = 0 as libc::c_int as UDWORD;
    missCount = 0 as libc::c_int as UDWORD;
    letThrough = 0 as libc::c_int as UDWORD;
    skipped = letThrough;
    i = 0 as libc::c_int as UDWORD;
    while i < 2500 as libc::c_int as libc::c_uint {
        /* Get a pointer - just cos our macro requires it, speeds not an issue here */
        psEffect =
            &mut *asEffectsList.as_mut_ptr().offset(i as isize) as
                *mut EFFECT;
        /* Clear all the control bits */
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        /* All effects are initially inactive */
        asEffectsList[i as usize].status =
            ES_INACTIVE as libc::c_int as UBYTE;
        i = i.wrapping_add(1)
    };
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn effectSetLandLightSpec(mut spec: LAND_LIGHT_SPEC) {
    ellSpec = spec as UDWORD;
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn effectSetSize(mut size: UDWORD) {
    specifiedSize = size;
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn addMultiEffect(mut basePos: *mut iVector,
                                        mut scatter: *mut iVector,
                                        mut group: EFFECT_GROUP,
                                        mut type_0: EFFECT_TYPE,
                                        mut specified: BOOL,
                                        mut imd: *mut iIMDShape,
                                        mut number: UDWORD, mut lit: BOOL,
                                        mut size: UDWORD) {
    let mut i: UDWORD = 0;
    let mut scatPos: iVector = iVector{x: 0, y: 0, z: 0,};
    if number == 0 as libc::c_int as libc::c_uint { return }
    /* Set up the scaling for specified ones */
    specifiedSize = size;
    /* If there's only one, make sure it's in the centre */
    if number == 1 as libc::c_int as libc::c_uint {
        scatPos.x = (*basePos).x;
        scatPos.y = (*basePos).y;
        scatPos.z = (*basePos).z;
        addEffect(&mut scatPos, group, type_0, specified, imd, lit);
    } else {
        /* Fix for jim */
        (*scatter).x /= 10 as libc::c_int;
        (*scatter).y /= 10 as libc::c_int;
        (*scatter).z /= 10 as libc::c_int;
        /* There are multiple effects - so scatter them around according to parameter */
        i = 0 as libc::c_int as UDWORD;
        while i < number {
            scatPos.x =
                (*basePos).x +
                    (if (*scatter).x != 0 {
                         ((*scatter).x) -
                             rand() % (2 as libc::c_int * (*scatter).x)
                     } else { 0 as libc::c_int });
            scatPos.y =
                (*basePos).y +
                    (if (*scatter).y != 0 {
                         ((*scatter).y) -
                             rand() % (2 as libc::c_int * (*scatter).y)
                     } else { 0 as libc::c_int });
            scatPos.z =
                (*basePos).z +
                    (if (*scatter).z != 0 {
                         ((*scatter).z) -
                             rand() % (2 as libc::c_int * (*scatter).z)
                     } else { 0 as libc::c_int });
            addEffect(&mut scatPos, group, type_0, specified, imd, lit);
            i = i.wrapping_add(1)
        }
    };
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getNumActiveEffects() -> UDWORD {
    return activeEffects;
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getMissCount() -> UDWORD { return missCount; }
#[no_mangle]
pub unsafe extern "C" fn getNumSkippedEffects() -> UDWORD {
    return skippedEffects;
}
#[no_mangle]
pub unsafe extern "C" fn getNumEvenEffects() -> UDWORD { return letThrough; }
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub static mut Reject1: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn addEffect(mut pos: *mut iVector,
                                   mut group: EFFECT_GROUP,
                                   mut type_0: EFFECT_TYPE,
                                   mut specified: BOOL,
                                   mut imd: *mut iIMDShape, mut lit: BOOL) {
    let mut essentialCount: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut bSmoke: BOOL = 0;
    aeCalls = aeCalls.wrapping_add(1);
    if gamePaused() != 0 { return }
    /* Quick optimsation to reject every second non-essential effect if it's off grid */
//	if(clipXY((UDWORD)MAKEINT(pos->x),(UDWORD)MAKEINT(pos->z)) == FALSE)
    if clipXY((*pos).x as UDWORD as SDWORD, (*pos).z as UDWORD as SDWORD) ==
           0 as libc::c_int {
        /* 	If effect is essentail - then let it through */
        if essentialEffect(group, type_0) == 0 {
            /* Some we can get rid of right away */
            if utterlyReject(group, type_0) != 0 {
                skipped = skipped.wrapping_add(1);
                return
            }
            /* Smoke gets culled more than most off grid effects */
            if group as libc::c_uint ==
                   EFFECT_SMOKE as libc::c_int as libc::c_uint {
                bSmoke = 1 as libc::c_int
            } else { bSmoke = 0 as libc::c_int }
            /* Others intermittently (50/50 for most and 25/100 for smoke */
            if if bSmoke != 0 {
                   (aeCalls) & 0x3 as libc::c_int as libc::c_uint
               } else { (aeCalls) & 0x1 as libc::c_int as libc::c_uint } != 0
               {
                /* Do one */
                skipped = skipped.wrapping_add(1);
                return
            }
            letThrough = letThrough.wrapping_add(1)
        }
    }
    i = freeEffect;
    essentialCount = 0 as libc::c_int as UDWORD;
    while asEffectsList[i as usize].control as libc::c_int &
              0x8 as libc::c_int != 0 &&
              essentialCount < 2500 as libc::c_int as libc::c_uint {
        /* Check for wrap around */
        if i >= (2500 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
            /* Go back to the first one */
            i = 0 as libc::c_int as UDWORD
        }
        essentialCount = essentialCount.wrapping_add(1);
        missCount = missCount.wrapping_add(1);
        i = i.wrapping_add(1)
    }
    /* Check the list isn't just full of essential effects */
    if essentialCount >= 2500 as libc::c_int as libc::c_uint {
        /* All of the effects are essential!?!? */
        return
    } else { freeEffect = i }
    /* Store away it's position - into FRACTS */
    asEffectsList[freeEffect as usize].position.x = (*pos).x as FRACT;
    asEffectsList[freeEffect as usize].position.y = (*pos).y as FRACT;
    asEffectsList[freeEffect as usize].position.z = (*pos).z as FRACT;
    /* Now, note group and type */
    asEffectsList[freeEffect as usize].group = group as UBYTE;
    asEffectsList[freeEffect as usize].type_0 = type_0 as UBYTE;
    /* Set when it entered the world */
    asEffectsList[freeEffect as usize].lastFrame = gameTime;
    asEffectsList[freeEffect as usize].birthTime =
        asEffectsList[freeEffect as usize].lastFrame;
    if group as libc::c_uint == EFFECT_GRAVITON as libc::c_int as libc::c_uint
           &&
           (type_0 as libc::c_uint ==
                GRAVITON_TYPE_GIBLET as libc::c_int as libc::c_uint ||
                type_0 as libc::c_uint ==
                    GRAVITON_TYPE_EMITTING_DR as libc::c_int as libc::c_uint)
       {
        asEffectsList[freeEffect as usize].frameNumber = lit as UBYTE
    } else {
        /* Starts off on frame zero */
        asEffectsList[freeEffect as usize].frameNumber =
            0 as libc::c_int as UBYTE
    }
    /*
		See what kind of effect it is - the add fucnction is different for each,
		although some things are shared
	*/
    asEffectsList[freeEffect as usize].imd = 0 as *mut iIMDShape;
    if lit != 0 {
        asEffectsList[freeEffect as usize].control =
            (asEffectsList[freeEffect as usize].control as libc::c_int |
                 0x40 as libc::c_int) as UBYTE
    }
    if specified != 0 {
        /* We're specifying what the imd is - override */
        asEffectsList[freeEffect as usize].imd = imd;
        //		}
        asEffectsList[freeEffect as usize].size = specifiedSize as UWORD
    }
    //		if(type == EXPLOSION_TYPE_SPECIFIED_FIXME)
//		{
//			asEffectsList[freeEffect].size = EXPLOSION_SIZE;
//		}
//		else
//		{
    /* Do all the effect type specific stuff */
    match group as libc::c_uint {
        2 => {
            effectSetupSmoke(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                         as
                                                                         isize));
        }
        4 => {
            effectSetupGraviton(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                            as
                                                                            isize));
        }
        0 => {
            effectSetupExplosion(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                             as
                                                                             isize));
        }
        1 => {
            //			effectSetupDust(&asEffectsList[freeEffect]);
            effectSetupConstruction(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                                as
                                                                                isize));
        }
        5 => {
            effectSetupWayPoint(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                            as
                                                                            isize));
        }
        6 => {
            effectSetupBlood(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                         as
                                                                         isize));
        }
        7 => {
            effectSetupDestruction(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                               as
                                                                               isize));
        }
        10 => {
            effectSetupFire(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                        as
                                                                        isize));
        }
        8 => {
            effectSetUpSatLaser(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                            as
                                                                            isize));
        }
        11 => {
            effectSetUpFirework(&mut *asEffectsList.as_mut_ptr().offset(freeEffect
                                                                            as
                                                                            isize));
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy group type for an effect\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"effects.c\x00" as *const u8 as *const libc::c_char,
                      485 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 10],
                                                &[libc::c_char; 10]>(b"addEffect\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    /* Make the effect active */
    asEffectsList[freeEffect as usize].status =
        ES_ACTIVE as libc::c_int as UBYTE;
    /* As of yet, it hasn't bounced (or whatever)... */
    if type_0 as libc::c_uint !=
           EXPLOSION_TYPE_LAND_LIGHT as libc::c_int as libc::c_uint {
        asEffectsList[freeEffect as usize].specific =
            0 as libc::c_int as UBYTE
    }
    /* Looks like we didn't establish an imd for the effect */
	/*
	ASSERT( asEffectsList[freeEffect].imd != NULL OR group == EFFECT_DESTRUCTION OR group == EFFECT_FIRE OR group == EFFECT_SAT_LASER,
		"null effect imd" );
	*/
    /* No more slots available? */
    let fresh0 = freeEffect;
    freeEffect = freeEffect.wrapping_add(1);
    if fresh0 >= (2500 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        /* Go back to the first one */
        freeEffect = 0 as libc::c_int as UDWORD
    };
}
/* Calls all the update functions for each different currently active effect */
#[no_mangle]
pub unsafe extern "C" fn processEffects() {
    let mut i: UDWORD = 0;
    let mut num: UDWORD = 0;
    /* Establish how long the last game frame took */
    fraction = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
    num = 0 as libc::c_int as UDWORD;
    missCount = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 2500 as libc::c_int as libc::c_uint {
        /* Is it active */
        match asEffectsList[i as usize].status as libc::c_int {
            1 => {
                /* The effect is active */
                /* So process it */
                updateEffect(&mut *asEffectsList.as_mut_ptr().offset(i as
                                                                         isize));
                num = num.wrapping_add(1)
            }
            2 | _ => { }
        }
        i = i.wrapping_add(1)
    }
    /* Add any droid effects */
    effectDroidUpdates();
    /* Add any structure effects */
    effectStructureUpdates();
    activeEffects = num;
    skippedEffects = skipped;
}
// ----------------------------------------------------------------------------------------
/*
drawEffects:-
This will either draw all the effects that are on the grid in a oner or
more likely add them to the bucket.
*/
#[no_mangle]
pub unsafe extern "C" fn drawEffects() {
    let mut i: UDWORD = 0;
    /* Reset counter */
    numEffects = 0 as libc::c_int as UDWORD;
    /* Traverse the list */
    i = 0 as libc::c_int as UDWORD;
    while i < 2500 as libc::c_int as libc::c_uint {
        /* Don't bother unless it's active */
        if asEffectsList[i as usize].status as libc::c_int ==
               ES_ACTIVE as libc::c_int {
            /* One more is active */
            numEffects = numEffects.wrapping_add(1);
            /* Is it on the grid */
            if clipXY(asEffectsList[i as usize].position.x as SDWORD as UDWORD
                          as SDWORD,
                      asEffectsList[i as usize].position.z as SDWORD as UDWORD
                          as SDWORD) != 0 {
                /* Add it to the bucket */
                bucketAddTypeToList(RENDER_EFFECT,
                                    &mut *asEffectsList.as_mut_ptr().offset(i
                                                                                as
                                                                                isize)
                                        as *mut EFFECT as *mut libc::c_void);
            }
        }
        i = i.wrapping_add(1)
    };
}
// MASTER Fn
// ----------------------------------------------------------------------------------------
// ---- Update functions - every group type of effect has one of these */
// ----------------------------------------------------------------------------------------
/* The general update function for all effects - calls a specific one for each */
#[no_mangle]
pub unsafe extern "C" fn updateEffect(mut psEffect: *mut EFFECT) {
    /* What type of effect are we dealing with? */
    match (*psEffect).group as libc::c_int {
        0 => { updateExplosion(psEffect); }
        5 => { if gamePaused() == 0 { updateWaypoint(psEffect); } }
        1 => { if gamePaused() == 0 { updateConstruction(psEffect); } }
        2 => { if gamePaused() == 0 { updatePolySmoke(psEffect); } }
        3 => { }
        4 => { if gamePaused() == 0 { updateGraviton(psEffect); } }
        6 => { if gamePaused() == 0 { updateBlood(psEffect); } }
        7 => { if gamePaused() == 0 { updateDestruction(psEffect); } }
        10 => { if gamePaused() == 0 { updateFire(psEffect); } }
        8 => { if gamePaused() == 0 { updateSatLaser(psEffect); } }
        11 => { if gamePaused() == 0 { updateFirework(psEffect); } }
        _ => {
            debug(LOG_ERROR,
                  b"Weirdy class of effect passed to updateEffect\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    };
}
//MASTER Fn
// ----------------------------------------------------------------------------------------
// ALL THE UPDATE FUNCTIONS
// ----------------------------------------------------------------------------------------
/* Update the waypoint effects.*/
#[no_mangle]
pub unsafe extern "C" fn updateWaypoint(mut psEffect: *mut EFFECT) {
    if !(keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
             keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0) {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE
    };
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn updateFirework(mut psEffect: *mut EFFECT) {
    let mut height: UDWORD = 0;
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut radius: UDWORD = 0;
    let mut val: UDWORD = 0;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut dif: UDWORD = 0;
    let mut drop_0: UDWORD = 0;
    /* Move it */
    (*psEffect).position.x += (*psEffect).velocity.x * fraction;
    (*psEffect).position.y += (*psEffect).velocity.y * fraction;
    (*psEffect).position.z += (*psEffect).velocity.z * fraction;
    if (*psEffect).type_0 as libc::c_int ==
           FIREWORK_TYPE_LAUNCHER as libc::c_int {
        height = (*psEffect).position.y as SDWORD as UDWORD;
        if height > (*psEffect).size as libc::c_uint {
            dv.x = (*psEffect).position.x as SDWORD;
            dv.z = (*psEffect).position.z as SDWORD;
            dv.y =
                (*psEffect).position.y as SDWORD +
                    (*psEffect).radius as libc::c_int / 2 as libc::c_int;
            addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            audio_PlayStaticTrack((*psEffect).position.x as SDWORD,
                                  (*psEffect).position.z as SDWORD,
                                  ID_SOUND_EXPLOSION as libc::c_int);
            //			stepHeight = psEffect->radius/15;
//			stepAngle = psEffect->radius/15;
            dif = 0 as libc::c_int as UDWORD;
            while dif <
                      ((*psEffect).radius as libc::c_int * 2 as libc::c_int)
                          as libc::c_uint {
                if dif < (*psEffect).radius as libc::c_uint {
                    drop_0 =
                        ((*psEffect).radius as libc::c_uint).wrapping_sub(dif)
                } else {
                    drop_0 =
                        dif.wrapping_sub((*psEffect).radius as libc::c_uint)
                }
                radius =
                    sqrt((((*psEffect).radius as libc::c_int *
                               (*psEffect).radius as libc::c_int) as
                              libc::c_uint).wrapping_sub(drop_0.wrapping_mul(drop_0))
                             as libc::c_double) as UDWORD;
                //val = getStaticTimeValueRange(720,360);	// grab an angle - 4 seconds cyclic
                val = 0 as libc::c_int as UDWORD;
                while val <= 180 as libc::c_int as libc::c_uint {
                    xDif =
                        radius.wrapping_mul(*aSinTable.as_mut_ptr().offset((((65536
                                                                                  as
                                                                                  libc::c_int
                                                                                  /
                                                                                  360
                                                                                      as
                                                                                      libc::c_int)
                                                                                 as
                                                                                 libc::c_uint).wrapping_mul(val)
                                                                                as
                                                                                uint16
                                                                                as
                                                                                libc::c_int
                                                                                >>
                                                                                4
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               isize)
                                                as libc::c_uint);
                    yDif =
                        radius.wrapping_mul(*aSinTable.as_mut_ptr().offset(((((65536
                                                                                   as
                                                                                   libc::c_int
                                                                                   /
                                                                                   360
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint).wrapping_mul(val)
                                                                                 as
                                                                                 uint16
                                                                                 as
                                                                                 libc::c_int
                                                                                 >>
                                                                                 4
                                                                                     as
                                                                                     libc::c_int)
                                                                                +
                                                                                1024
                                                                                    as
                                                                                    libc::c_int)
                                                                               as
                                                                               isize)
                                                as libc::c_uint);
                    //   			dv.x = dv.x - (2*xDif);
	//   			dv.z = dv.z - (2*yDif);	// buildings are level!
	//			effectGiveAuxVar(100);
	//   			addEffect(&dv,EFFECT_FIREWORK, FIREWORK_TYPE_STARBURST,FALSE,NULL,0);
                    xDif =
                        xDif.wrapping_div(4096 as libc::c_int as
                                              libc::c_uint); // cos it's fixed point
                    yDif =
                        yDif.wrapping_div(4096 as libc::c_int as
                                              libc::c_uint);
                    dv.x =
                        ((*psEffect).position.x as SDWORD as
                             libc::c_uint).wrapping_add(xDif) as int32;
                    dv.z =
                        ((*psEffect).position.z as SDWORD as
                             libc::c_uint).wrapping_add(yDif) as int32;
                    dv.y =
                        ((*psEffect).position.y as SDWORD as
                             libc::c_uint).wrapping_add(dif) as int32;
                    effectGiveAuxVar(100 as libc::c_int as UDWORD);
                    addEffect(&mut dv, EFFECT_FIREWORK,
                              FIREWORK_TYPE_STARBURST, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                    dv.x =
                        ((*psEffect).position.x as SDWORD as
                             libc::c_uint).wrapping_sub(xDif) as int32;
                    dv.z =
                        ((*psEffect).position.z as SDWORD as
                             libc::c_uint).wrapping_sub(yDif) as int32;
                    dv.y =
                        ((*psEffect).position.y as SDWORD as
                             libc::c_uint).wrapping_add(dif) as int32;
                    effectGiveAuxVar(100 as libc::c_int as UDWORD);
                    addEffect(&mut dv, EFFECT_FIREWORK,
                              FIREWORK_TYPE_STARBURST, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                    dv.x =
                        ((*psEffect).position.x as SDWORD as
                             libc::c_uint).wrapping_add(xDif) as int32;
                    dv.z =
                        ((*psEffect).position.z as SDWORD as
                             libc::c_uint).wrapping_sub(yDif) as int32;
                    dv.y =
                        ((*psEffect).position.y as SDWORD as
                             libc::c_uint).wrapping_add(dif) as int32;
                    effectGiveAuxVar(100 as libc::c_int as UDWORD);
                    addEffect(&mut dv, EFFECT_FIREWORK,
                              FIREWORK_TYPE_STARBURST, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                    dv.x =
                        ((*psEffect).position.x as SDWORD as
                             libc::c_uint).wrapping_sub(xDif) as int32;
                    dv.z =
                        ((*psEffect).position.z as SDWORD as
                             libc::c_uint).wrapping_add(yDif) as int32;
                    dv.y =
                        ((*psEffect).position.y as SDWORD as
                             libc::c_uint).wrapping_add(dif) as int32;
                    effectGiveAuxVar(100 as libc::c_int as UDWORD);
                    addEffect(&mut dv, EFFECT_FIREWORK,
                              FIREWORK_TYPE_STARBURST, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                    val =
                        (val as
                             libc::c_uint).wrapping_add(20 as libc::c_int as
                                                            libc::c_uint) as
                            UDWORD as UDWORD
                }
                dif =
                    (dif as
                         libc::c_uint).wrapping_add(20 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD
            }
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE
        } else {
            /* Add an effect at the firework's position */
            dv.x = (*psEffect).position.x as SDWORD;
            dv.y = (*psEffect).position.y as SDWORD;
            dv.z = (*psEffect).position.z as SDWORD;
            /* Add a trail graphic */
            addEffect(&mut dv, EFFECT_SMOKE, SMOKE_TYPE_TRAIL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    } else {
        // must be a startburst
        /* Time to update the frame number on the smoke sprite */
        if gameTime.wrapping_sub((*psEffect).lastFrame) >
               (*psEffect).frameDelay as libc::c_uint {
            /* Store away last frame change time */
            (*psEffect).lastFrame = gameTime;
            /* Are we on the last frame? */
            (*psEffect).frameNumber = (*psEffect).frameNumber.wrapping_add(1);
            if (*psEffect).frameNumber as libc::c_uint >=
                   EffectGetNumFrames(psEffect) {
                /* Does the anim wrap around? */
                if (*psEffect).control as libc::c_int & 0x4 as libc::c_int !=
                       0 {
                    (*psEffect).frameNumber = 0 as libc::c_int as UBYTE
                } else {
                    /* Kill it off */
                    (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                    (*psEffect).control = 0 as libc::c_int as UBYTE;
                    return
                }
            }
        }
        if (*psEffect).control as libc::c_int & 0x4 as libc::c_int != 0 {
            /* Has it overstayed it's welcome? */
            if gameTime.wrapping_sub((*psEffect).birthTime) >
                   (*psEffect).lifeSpan as libc::c_uint {
                /* Kill it */
                (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                (*psEffect).control = 0 as libc::c_int as UBYTE
            }
        }
    };
}
/* If it doesn't get killed by frame number, then by age */
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn updateSatLaser(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut val: UDWORD = 0;
    let mut radius: UDWORD = 0;
    let mut xDif: UDWORD = 0;
    let mut yDif: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut startHeight: UDWORD = 0;
    let mut endHeight: UDWORD = 0;
    let mut pie: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut xPos: UDWORD = 0;
    let mut yPos: UDWORD = 0;
    let mut light: LIGHT =
        LIGHT{position: iVector{x: 0, y: 0, z: 0,},
              type_0: 0,
              range: 0,
              colour: 0,};
    // Do these here cause there used by the lighting code below this if.
    xPos = (*psEffect).position.x as SDWORD as UDWORD;
    startHeight = (*psEffect).position.y as SDWORD as UDWORD;
    endHeight = startHeight.wrapping_add(1064 as libc::c_int as libc::c_uint);
    yPos = (*psEffect).position.z as SDWORD as UDWORD;
    if (*psEffect).baseScale != 0 {
        (*psEffect).baseScale = 0 as libc::c_int as UBYTE;
        pie = getImdFromIndex(MI_FLAME as libc::c_int as UDWORD);
        //printf("%d %d : %d %d : %p\n",xPos,yPos,startHeight,endHeight,pie);
		/* Add some big explosions....! */
        i = 0 as libc::c_int as UDWORD;
        while i < 16 as libc::c_int as libc::c_uint {
            dv.x =
                xPos.wrapping_add((200 as libc::c_int -
                                       rand() % 400 as libc::c_int) as
                                      libc::c_uint) as int32;
            dv.z =
                yPos.wrapping_add((200 as libc::c_int -
                                       rand() % 400 as libc::c_int) as
                                      libc::c_uint) as int32;
            dv.y =
                startHeight.wrapping_add((rand() % 100 as libc::c_int) as
                                             libc::c_uint) as int32;
            addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            i = i.wrapping_add(1)
        }
        /* Add a sound effect */
        audio_PlayStaticTrack((*psEffect).position.x as SDWORD,
                              (*psEffect).position.z as SDWORD,
                              ID_SOUND_EXPLOSION as libc::c_int);
        /* Add a shockwave */
        dv.x = xPos as int32;
        dv.z = yPos as int32;
        dv.y =
            startHeight.wrapping_add(64 as libc::c_int as libc::c_uint) as
                int32;
        addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_SHOCKWAVE,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        /* Now, add the column of light */
        i = startHeight;
        while i < endHeight {
            radius = 80 as libc::c_int as UDWORD;
            /* Add 36 around in a circle..! */
            val = 0 as libc::c_int as UDWORD; // cos it's fixed point
            while val <= 180 as libc::c_int as libc::c_uint {
                xDif =
                    radius.wrapping_mul(*aSinTable.as_mut_ptr().offset((((65536
                                                                              as
                                                                              libc::c_int
                                                                              /
                                                                              360
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             libc::c_uint).wrapping_mul(val)
                                                                            as
                                                                            uint16
                                                                            as
                                                                            libc::c_int
                                                                            >>
                                                                            4
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           isize)
                                            as libc::c_uint);
                yDif =
                    radius.wrapping_mul(*aSinTable.as_mut_ptr().offset(((((65536
                                                                               as
                                                                               libc::c_int
                                                                               /
                                                                               360
                                                                                   as
                                                                                   libc::c_int)
                                                                              as
                                                                              libc::c_uint).wrapping_mul(val)
                                                                             as
                                                                             uint16
                                                                             as
                                                                             libc::c_int
                                                                             >>
                                                                             4
                                                                                 as
                                                                                 libc::c_int)
                                                                            +
                                                                            1024
                                                                                as
                                                                                libc::c_int)
                                                                           as
                                                                           isize)
                                            as libc::c_uint);
                xDif = xDif.wrapping_div(4096 as libc::c_int as libc::c_uint);
                yDif = yDif.wrapping_div(4096 as libc::c_int as libc::c_uint);
                dv.x =
                    xPos.wrapping_add(xDif).wrapping_add(i.wrapping_div(64 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                        as int32;
                dv.z = yPos.wrapping_add(yDif) as int32;
                dv.y = startHeight.wrapping_add(i) as int32;
                effectGiveAuxVar(100 as libc::c_int as UDWORD);
                addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                dv.x =
                    xPos.wrapping_sub(xDif).wrapping_add(i.wrapping_div(64 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                        as int32;
                dv.z = yPos.wrapping_sub(yDif) as int32;
                dv.y = startHeight.wrapping_add(i) as int32;
                effectGiveAuxVar(100 as libc::c_int as UDWORD);
                addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                dv.x =
                    xPos.wrapping_add(xDif).wrapping_add(i.wrapping_div(64 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                        as int32;
                dv.z = yPos.wrapping_sub(yDif) as int32;
                dv.y = startHeight.wrapping_add(i) as int32;
                effectGiveAuxVar(100 as libc::c_int as UDWORD);
                addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                dv.x =
                    xPos.wrapping_sub(xDif).wrapping_add(i.wrapping_div(64 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                        as int32;
                dv.z = yPos.wrapping_add(yDif) as int32;
                dv.y = startHeight.wrapping_add(i) as int32;
                effectGiveAuxVar(100 as libc::c_int as UDWORD);
                addEffect(&mut dv, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                val =
                    (val as
                         libc::c_uint).wrapping_add(30 as libc::c_int as
                                                        libc::c_uint) as
                        UDWORD as UDWORD
            }
            i =
                (i as
                     libc::c_uint).wrapping_add(56 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
    }
    //printf("%d %d\n",gameTime,psEffect->birthTime);
    if gameTime.wrapping_sub((*psEffect).birthTime) <
           1000 as libc::c_int as libc::c_uint {
        light.position.x = xPos as int32;
        light.position.y = startHeight as int32;
        light.position.z = yPos as int32;
        light.range = 800 as libc::c_int as UDWORD;
        light.colour = LIGHT_BLUE as libc::c_int as UDWORD;
        processLight(&mut light);
    } else {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE
    };
}
// ----------------------------------------------------------------------------------------
/* The update function for the explosions */
#[no_mangle]
pub unsafe extern "C" fn updateExplosion(mut psEffect: *mut EFFECT) {
    let mut light: LIGHT =
        LIGHT{position: iVector{x: 0, y: 0, z: 0,},
              type_0: 0,
              range: 0,
              colour: 0,};
    let mut percent: UDWORD = 0;
    let mut range: UDWORD = 0;
    let mut scaling: FRACT = 0.;
    if (*psEffect).control as libc::c_int & 0x40 as libc::c_int != 0 {
        if (*psEffect).lifeSpan != 0 {
            percent =
                gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                             as
                                                                                                             libc::c_uint);
            if percent > 100 as libc::c_int as libc::c_uint {
                percent = 100 as libc::c_int as UDWORD
            } else if percent > 50 as libc::c_int as libc::c_uint {
                percent =
                    (100 as libc::c_int as libc::c_uint).wrapping_sub(percent)
            }
        } else { percent = 100 as libc::c_int as UDWORD }
        range = percent;
        //#endif
        light.position.x = (*psEffect).position.x as SDWORD;
        light.position.y = (*psEffect).position.y as SDWORD;
        light.position.z = (*psEffect).position.z as SDWORD;
        light.range =
            (3 as libc::c_int as
                 libc::c_uint).wrapping_mul(range).wrapping_div(2 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint);
        light.colour = LIGHT_RED as libc::c_int as UDWORD;
        processLight(&mut light);
    }
    //#ifdef DOLIGHTS
    /*
	if(psEffect->type == EXPLOSION_TYPE_LAND_LIGHT)
	{
		light.position.x = MAKEINT(psEffect->position.x);
		light.position.y = MAKEINT(psEffect->position.y);
		light.position.z = MAKEINT(psEffect->position.z);
		light.range = getTimeValueRange(1024,512);
		if(light.range>256) light.range = 512-light.range;
		light.colour = LIGHT_RED;
		processLight(&light);
	}
*/
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_SHOCKWAVE as libc::c_int {
        (*psEffect).size =
            ((*psEffect).size as libc::c_int +
                 (fraction * 1000 as libc::c_int as libc::c_float) as SDWORD)
                as UWORD;
        scaling =
            (*psEffect).size as FRACT / 500 as libc::c_int as libc::c_float;
        (*psEffect).frameNumber =
            (scaling * EffectGetNumFrames(psEffect) as libc::c_float) as
                SDWORD as UBYTE;
        light.position.x = (*psEffect).position.x as SDWORD;
        light.position.y = (*psEffect).position.y as SDWORD;
        light.position.z = (*psEffect).position.z as SDWORD;
        light.range =
            ((*psEffect).size as libc::c_int + 200 as libc::c_int) as UDWORD;
        light.colour = LIGHT_YELLOW as libc::c_int as UDWORD;
        processLight(&mut light);
        if (*psEffect).size as libc::c_int > 500 as libc::c_int ||
               light.range > 600 as libc::c_int as libc::c_uint {
            /* Kill it off */
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        }
    } else if gameTime.wrapping_sub((*psEffect).lastFrame) >
                  (*psEffect).frameDelay as libc::c_uint {
        (*psEffect).lastFrame = gameTime;
        /* Time to update the frame number on the explosion */
        /* Are we on the last frame? */
        (*psEffect).frameNumber = (*psEffect).frameNumber.wrapping_add(1);
        if (*psEffect).frameNumber as libc::c_uint >=
               EffectGetNumFrames(psEffect) {
            if (*psEffect).type_0 as libc::c_int !=
                   EXPLOSION_TYPE_LAND_LIGHT as libc::c_int {
                /* Kill it off */
                (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                (*psEffect).control = 0 as libc::c_int as UBYTE;
                return
            } else { (*psEffect).frameNumber = 0 as libc::c_int as UBYTE }
        }
    }
    if gamePaused() == 0 {
        /* Tesla explosions are the only ones that rise, or indeed move */
        if (*psEffect).type_0 as libc::c_int ==
               EXPLOSION_TYPE_TESLA as libc::c_int {
            (*psEffect).position.y +=
                (*psEffect).velocity.y as SDWORD as libc::c_float * fraction
        }
    };
}
// ----------------------------------------------------------------------------------------
/* The update function for blood */
#[no_mangle]
pub unsafe extern "C" fn updateBlood(mut psEffect: *mut EFFECT) {
    /* Time to update the frame number on the blood */
    if gameTime.wrapping_sub((*psEffect).lastFrame) >
           (*psEffect).frameDelay as libc::c_uint {
        (*psEffect).lastFrame = gameTime;
        /* Are we on the last frame? */
        (*psEffect).frameNumber = (*psEffect).frameNumber.wrapping_add(1);
        if (*psEffect).frameNumber as libc::c_uint >=
               EffectGetNumFrames(psEffect) {
            /* Kill it off */
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        }
    }
    /* Move it about in the world */
    (*psEffect).position.x += (*psEffect).velocity.x * fraction;
    (*psEffect).position.y += (*psEffect).velocity.y * fraction;
    (*psEffect).position.z += (*psEffect).velocity.z * fraction;
}
// ----------------------------------------------------------------------------------------
/* Processes all the drifting smoke
	Handles the smoke puffing out the factory as well */
#[no_mangle]
pub unsafe extern "C" fn updatePolySmoke(mut psEffect: *mut EFFECT) {
    /* Time to update the frame number on the smoke sprite */
    if gameTime.wrapping_sub((*psEffect).lastFrame) >
           (*psEffect).frameDelay as libc::c_uint {
        /* Store away last frame change time */
        (*psEffect).lastFrame = gameTime;
        /* Are we on the last frame? */
        (*psEffect).frameNumber = (*psEffect).frameNumber.wrapping_add(1);
        if (*psEffect).frameNumber as libc::c_uint >=
               EffectGetNumFrames(psEffect) {
            /* Does the anim wrap around? */
            if (*psEffect).control as libc::c_int & 0x4 as libc::c_int != 0 {
                /* Does it change drift direction? */
                if (*psEffect).type_0 as libc::c_int ==
                       SMOKE_TYPE_DRIFTING as libc::c_int {
                    /* Make it change direction */
                    (*psEffect).velocity.x =
                        (rand() % 20 as libc::c_int) as FRACT;
                    (*psEffect).velocity.z =
                        (10 as libc::c_int - rand() % 20 as libc::c_int) as
                            FRACT;
                    (*psEffect).velocity.y =
                        (10 as libc::c_int + rand() % 20 as libc::c_int) as
                            FRACT
                }
                /* Reset the frame */
                (*psEffect).frameNumber = 0 as libc::c_int as UBYTE
            } else {
                /* Kill it off */
                (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                (*psEffect).control = 0 as libc::c_int as UBYTE;
                return
            }
        }
    }
    /* Update position */
    (*psEffect).position.x += (*psEffect).velocity.x * fraction;
    (*psEffect).position.y += (*psEffect).velocity.y * fraction;
    (*psEffect).position.z += (*psEffect).velocity.z * fraction;
    /* If it doesn't get killed by frame number, then by age */
    if (*psEffect).control as libc::c_int & 0x4 as libc::c_int != 0 {
        /* Has it overstayed it's welcome? */
        if gameTime.wrapping_sub((*psEffect).birthTime) >
               (*psEffect).lifeSpan as libc::c_uint {
            /* Kill it */
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE
        }
    };
}
// ----------------------------------------------------------------------------------------
/*
	Gravitons just fly up for a bit and then drop down and are
	killed off when they hit the ground
*/
#[no_mangle]
pub unsafe extern "C" fn updateGraviton(mut psEffect: *mut EFFECT) {
    let mut accel: FRACT = 0.;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut groundHeight: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut light: LIGHT =
        LIGHT{position: iVector{x: 0, y: 0, z: 0,},
              type_0: 0,
              range: 0,
              colour: 0,};
    if (*psEffect).type_0 as libc::c_int !=
           GRAVITON_TYPE_GIBLET as libc::c_int {
        light.position.x = (*psEffect).position.x as SDWORD;
        light.position.y = (*psEffect).position.y as SDWORD;
        light.position.z = (*psEffect).position.z as SDWORD;
        light.range = 128 as libc::c_int as UDWORD;
        light.colour = LIGHT_YELLOW as libc::c_int as UDWORD;
        processLight(&mut light);
    }
    if gamePaused() != 0 {
        /* Only update the lights if it's paused */
        return
    }
    /* Move it about in the world */
    (*psEffect).position.x += (*psEffect).velocity.x * fraction;
    (*psEffect).position.y += (*psEffect).velocity.y * fraction;
    (*psEffect).position.z += (*psEffect).velocity.z * fraction;
    /* If it's bounced/drifted off the map then kill it */
    if ((*psEffect).position.x as SDWORD as
            UDWORD).wrapping_div(128 as libc::c_int as libc::c_uint) >=
           mapWidth ||
           ((*psEffect).position.z as SDWORD as
                UDWORD).wrapping_div(128 as libc::c_int as libc::c_uint) >=
               mapHeight {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        return
    }
    groundHeight =
        map_Height((*psEffect).position.x as SDWORD as UDWORD,
                   (*psEffect).position.z as SDWORD as UDWORD) as UDWORD;
    /* If it's going up and it's still under the landscape, then remove it... */
    if (*psEffect).position.y < groundHeight as libc::c_float &&
           (*psEffect).velocity.y as SDWORD > 0 as libc::c_int {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        return
    }
    /* Does it emit a trail? And is it high enough? */
    if (*psEffect).type_0 as libc::c_int ==
           GRAVITON_TYPE_EMITTING_DR as libc::c_int ||
           (*psEffect).type_0 as libc::c_int ==
               GRAVITON_TYPE_EMITTING_ST as libc::c_int &&
               (*psEffect).position.y >
                   groundHeight.wrapping_add(10 as libc::c_int as
                                                 libc::c_uint) as
                       libc::c_float {
        /* Time to add another trail 'thing'? */
        if gameTime >
               (*psEffect).lastFrame.wrapping_add((*psEffect).frameDelay as
                                                      libc::c_uint) {
            /* Store away last update */
            (*psEffect).lastFrame = gameTime;
            /* Add an effect at the gravitons's position */
            dv.x = (*psEffect).position.x as SDWORD;
            dv.y = (*psEffect).position.y as SDWORD;
            dv.z = (*psEffect).position.z as SDWORD;
            /* Add a trail graphic */
            addEffect(&mut dv, EFFECT_SMOKE, SMOKE_TYPE_TRAIL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    } else if (*psEffect).type_0 as libc::c_int ==
                  GRAVITON_TYPE_GIBLET as libc::c_int &&
                  (*psEffect).position.y >
                      groundHeight.wrapping_add(5 as libc::c_int as
                                                    libc::c_uint) as
                          libc::c_float {
        /* Time to add another trail 'thing'? */
        if gameTime >
               (*psEffect).lastFrame.wrapping_add((*psEffect).frameDelay as
                                                      libc::c_uint) {
            /* Store away last update */
            (*psEffect).lastFrame = gameTime;
            /* Add an effect at the gravitons's position */
            dv.x = (*psEffect).position.x as SDWORD;
            dv.y = (*psEffect).position.y as SDWORD;
            dv.z = (*psEffect).position.z as SDWORD;
            addEffect(&mut dv, EFFECT_BLOOD, BLOOD_TYPE_NORMAL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    }
    /* Spin it round a bit */
    (*psEffect).rotation.x +=
        ((*psEffect).spin.x as FRACT * fraction) as SDWORD;
    (*psEffect).rotation.y +=
        ((*psEffect).spin.y as FRACT * fraction) as SDWORD;
    (*psEffect).rotation.z +=
        ((*psEffect).spin.z as FRACT * fraction) as SDWORD;
    /* Update velocity (and retarding of descent) according to present frame rate */
    accel = -(800 as libc::c_int) as FRACT * fraction;
    (*psEffect).velocity.y += accel;
    /* If it's bounced/drifted off the map then kill it */
    if (*psEffect).position.x as SDWORD <= 128 as libc::c_int ||
           (*psEffect).position.z as SDWORD <= 128 as libc::c_int {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        return
    }
    /* Are we below it? - Hit the ground? */
    if ((*psEffect).position.y as SDWORD) < groundHeight as SDWORD {
        psTile =
            mapTile(((*psEffect).position.x as SDWORD >> 7 as libc::c_int) as
                        UDWORD,
                    ((*psEffect).position.z as SDWORD >> 7 as libc::c_int) as
                        UDWORD);
        if terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_WATER as libc::c_int {
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        } else {
            /* Are we falling - rather than rising? */
            if ((*psEffect).velocity.y as SDWORD) < 0 as libc::c_int {
                /* Has it sufficient energy to keep bouncing? */
                if abs((*psEffect).velocity.y as SDWORD) > 16 as libc::c_int
                       &&
                       (*psEffect).specific as libc::c_int <= 2 as libc::c_int
                   {
                    (*psEffect).specific =
                        (*psEffect).specific.wrapping_add(1);
                    /* Half it's velocity */
                    //				psEffect->velocity.x/=(FRACT)(2);
                    (*psEffect).velocity.y /=
                        -(2 as libc::c_int) as FRACT; // only y gets flipped
                    //				psEffect->velocity.z/=(FRACT)(2);
                    /* Set it at ground level - may have gone through */
                    (*psEffect).position.y = groundHeight as FRACT
                } else {
                    /* Giblets don't blow up when they hit the ground! */
                    if (*psEffect).type_0 as libc::c_int !=
                           GRAVITON_TYPE_GIBLET as libc::c_int {
                        /* Remove the graviton and add an explosion */
                        dv.x = (*psEffect).position.x as SDWORD;
                        dv.y =
                            ((*psEffect).position.y +
                                 10 as libc::c_int as libc::c_float) as
                                SDWORD;
                        dv.z = (*psEffect).position.z as SDWORD;
                        addEffect(&mut dv, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_VERY_SMALL, 0 as libc::c_int,
                                  0 as *mut iIMDShape, 0 as libc::c_int);
                    }
                    (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                    (*psEffect).control = 0 as libc::c_int as UBYTE;
                    return
                }
            }
        }
    };
}
// ----------------------------------------------------------------------------------------
/* updateDestruction
This isn't really an on-screen effect itself - it just spawns other ones....
  */
#[no_mangle]
pub unsafe extern "C" fn updateDestruction(mut psEffect: *mut EFFECT) {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut effectType: UDWORD = 0;
    let mut widthScatter: UDWORD = 0 as libc::c_int as UDWORD;
    let mut breadthScatter: UDWORD = 0 as libc::c_int as UDWORD;
    let mut heightScatter: UDWORD = 0 as libc::c_int as UDWORD;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut light: LIGHT =
        LIGHT{position: iVector{x: 0, y: 0, z: 0,},
              type_0: 0,
              range: 0,
              colour: 0,};
    let mut percent: UDWORD = 0;
    let mut range: UDWORD = 0;
    let mut div: FRACT = 0.;
    let mut height: UDWORD = 0;
    percent =
        gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                     as
                                                                                                     libc::c_uint);
    if percent > 100 as libc::c_int as libc::c_uint {
        percent = 100 as libc::c_int as UDWORD
    }
    range =
        (50 as libc::c_int -
             abs((50 as libc::c_int as libc::c_uint).wrapping_sub(percent) as
                     libc::c_int)) as UDWORD;
    light.position.x = (*psEffect).position.x as SDWORD;
    light.position.y = (*psEffect).position.y as SDWORD;
    light.position.z = (*psEffect).position.z as SDWORD;
    if (*psEffect).type_0 as libc::c_int ==
           DESTRUCTION_TYPE_STRUCTURE as libc::c_int {
        light.range = range.wrapping_mul(10 as libc::c_int as libc::c_uint)
    } else {
        light.range = range.wrapping_mul(4 as libc::c_int as libc::c_uint)
    }
    if (*psEffect).type_0 as libc::c_int ==
           DESTRUCTION_TYPE_POWER_STATION as libc::c_int {
        light.range =
            (light.range as
                 libc::c_uint).wrapping_mul(3 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        light.colour = LIGHT_WHITE as libc::c_int as UDWORD
    } else { light.colour = LIGHT_RED as libc::c_int as UDWORD }
    processLight(&mut light);
    if gameTime >
           (*psEffect).birthTime.wrapping_add((*psEffect).lifeSpan as
                                                  libc::c_uint) {
        /* Kill it - it's too old */
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        return
    }
    if (*psEffect).type_0 as libc::c_int ==
           DESTRUCTION_TYPE_SKYSCRAPER as libc::c_int {
        if gameTime.wrapping_sub((*psEffect).birthTime) >
               (9 as libc::c_int * (*psEffect).lifeSpan as libc::c_int /
                    10 as libc::c_int) as libc::c_uint {
            pos.x = (*psEffect).position.x as SDWORD;
            pos.z = (*psEffect).position.z as SDWORD;
            pos.y = (*psEffect).position.y as SDWORD;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LARGE,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        }
        div =
            gameTime.wrapping_sub((*psEffect).birthTime) as FRACT /
                (*psEffect).lifeSpan as libc::c_int as libc::c_float;
        if div > 1 as libc::c_int as FRACT { div = 1 as libc::c_int as FRACT }
        div = 1 as libc::c_int as FRACT - div;
        height =
            (div * (*(*psEffect).imd).ymax as libc::c_float) as SDWORD as
                UDWORD
    } else { height = 16 as libc::c_int as UDWORD }
    /* Time to add another effect? */
    if gameTime.wrapping_sub((*psEffect).lastFrame) >
           (*psEffect).frameDelay as libc::c_uint {
        (*psEffect).lastFrame = gameTime;
        match (*psEffect).type_0 as libc::c_int {
            38 => {
                widthScatter = 128 as libc::c_int as UDWORD;
                breadthScatter = 128 as libc::c_int as UDWORD;
                heightScatter = 128 as libc::c_int as UDWORD
            }
            35 | 34 => {
                widthScatter =
                    (128 as libc::c_int / 2 as libc::c_int) as UDWORD;
                breadthScatter =
                    (128 as libc::c_int / 2 as libc::c_int) as UDWORD;
                heightScatter =
                    (128 as libc::c_int / 4 as libc::c_int) as UDWORD
            }
            33 | 36 | 37 => {
                widthScatter =
                    (128 as libc::c_int / 6 as libc::c_int) as UDWORD;
                breadthScatter =
                    (128 as libc::c_int / 6 as libc::c_int) as UDWORD;
                heightScatter =
                    (128 as libc::c_int / 6 as libc::c_int) as UDWORD
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Weirdy destruction type effect\x00" as *const u8
                              as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"effects.c\x00" as *const u8 as
                              *const libc::c_char, 1368 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"updateDestruction\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
        /* Find a position to dump it at */
        pos.x =
            ((*psEffect).position.x as SDWORD as
                 libc::c_uint).wrapping_add(widthScatter).wrapping_sub((rand()
                                                                            as
                                                                            libc::c_uint).wrapping_rem((2
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            libc::c_uint).wrapping_mul(widthScatter)))
                as int32;
        pos.z =
            ((*psEffect).position.z as SDWORD as
                 libc::c_uint).wrapping_add(breadthScatter).wrapping_sub((rand()
                                                                              as
                                                                              libc::c_uint).wrapping_rem((2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint).wrapping_mul(breadthScatter)))
                as int32;
        pos.y =
            ((*psEffect).position.y as SDWORD as
                 libc::c_uint).wrapping_add(height).wrapping_add((rand() as
                                                                      libc::c_uint).wrapping_rem(heightScatter))
                as int32;
        if (*psEffect).type_0 as libc::c_int ==
               DESTRUCTION_TYPE_SKYSCRAPER as libc::c_int {
            pos.y =
                ((*psEffect).position.y as SDWORD as
                     libc::c_uint).wrapping_add(height) as int32
        }
        /* Choose an effect */
        effectType = (rand() % 15 as libc::c_int) as UDWORD;
        match effectType {
            0 => {
                addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_DRIFTING,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
            }
            1 | 2 | 3 | 4 | 5 => {
                if (*psEffect).type_0 as libc::c_int ==
                       DESTRUCTION_TYPE_SKYSCRAPER as libc::c_int {
                    addEffect(&mut pos, EFFECT_EXPLOSION,
                              EXPLOSION_TYPE_LARGE, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                } else if (*psEffect).type_0 as libc::c_int ==
                              DESTRUCTION_TYPE_STRUCTURE as libc::c_int {
                    addEffect(&mut pos, EFFECT_EXPLOSION,
                              EXPLOSION_TYPE_MEDIUM, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                } else {
                    addEffect(&mut pos, EFFECT_EXPLOSION,
                              EXPLOSION_TYPE_SMALL, 0 as libc::c_int,
                              0 as *mut iIMDShape, 0 as libc::c_int);
                }
            }
            6 | 7 | 8 | 9 | 10 => {
                if (*psEffect).type_0 as libc::c_int ==
                       DESTRUCTION_TYPE_STRUCTURE as libc::c_int {
                    /* Only structures get the big explosions */
                    //				addEffect(&pos,EFFECT_GRAVITON,GRAVITON_TYPE_EMITTING_ST,TRUE,debrisImds[rand()%MAX_DEBRIS],0);
                    addEffect(&mut pos, EFFECT_GRAVITON,
                              GRAVITON_TYPE_EMITTING_ST, 1 as libc::c_int,
                              getRandomDebrisImd(), 0 as libc::c_int);
                } else {
                    addEffect(&mut pos, EFFECT_GRAVITON,
                              GRAVITON_TYPE_EMITTING_DR, 1 as libc::c_int,
                              getRandomDebrisImd(), 0 as libc::c_int);
                }
            }
            11 => {
                addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_DRIFTING,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
            }
            12 | 13 => {
                addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
            }
            14 => {
                /* Add sound effect, but only if we're less than 3/4 of the way thru' destruction */
                if gameTime <
                       (3 as libc::c_int as
                            libc::c_uint).wrapping_mul((*psEffect).birthTime.wrapping_add((*psEffect).lifeSpan
                                                                                              as
                                                                                              libc::c_uint)).wrapping_div(4
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint)
                   {
                    iX = (*psEffect).position.x as SDWORD;
                    iY = (*psEffect).position.z as SDWORD;
                    audio_PlayStaticTrack(iX, iY,
                                          ID_SOUND_EXPLOSION as libc::c_int);
                }
            }
            _ => { }
        }
    };
}
// ----------------------------------------------------------------------------------------
/*
updateConstruction:-
Moves the construction graphic about - dust cloud or whatever....
*/
#[no_mangle]
pub unsafe extern "C" fn updateConstruction(mut psEffect: *mut EFFECT) {
    /* Time to update the frame number on the construction sprite */
    if gameTime.wrapping_sub((*psEffect).lastFrame) >
           (*psEffect).frameDelay as libc::c_uint {
        (*psEffect).lastFrame = gameTime;
        /* Are we on the last frame? */
        (*psEffect).frameNumber = (*psEffect).frameNumber.wrapping_add(1);
        if (*psEffect).frameNumber as libc::c_uint >=
               EffectGetNumFrames(psEffect) {
            /* Is it a cyclic sprite? */
            if (*psEffect).control as libc::c_int & 0x4 as libc::c_int != 0 {
                (*psEffect).frameNumber = 0 as libc::c_int as UBYTE
            } else {
                (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
                (*psEffect).control = 0 as libc::c_int as UBYTE;
                return
            }
        }
    }
    /* Move it about in the world */
    (*psEffect).position.x += (*psEffect).velocity.x * fraction;
    (*psEffect).position.y += (*psEffect).velocity.y * fraction;
    (*psEffect).position.z += (*psEffect).velocity.z * fraction;
    /* If it doesn't get killed by frame number, then by height */
    if (*psEffect).control as libc::c_int & 0x4 as libc::c_int != 0 {
        /* Has it hit the ground */
        if (*psEffect).position.y as SDWORD as UDWORD <=
               map_Height((*psEffect).position.x as SDWORD as UDWORD,
                          (*psEffect).position.z as SDWORD as UDWORD) as
                   libc::c_uint {
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        }
        if gameTime.wrapping_sub((*psEffect).birthTime) >
               (*psEffect).lifeSpan as libc::c_uint {
            (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
            (*psEffect).control = 0 as libc::c_int as UBYTE;
            return
        }
    };
}
// ----------------------------------------------------------------------------------------
/* Update fire sequences */
#[no_mangle]
pub unsafe extern "C" fn updateFire(mut psEffect: *mut EFFECT) {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut light: LIGHT =
        LIGHT{position: iVector{x: 0, y: 0, z: 0,},
              type_0: 0,
              range: 0,
              colour: 0,};
    let mut percent: UDWORD = 0;
    percent =
        gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                     as
                                                                                                     libc::c_uint);
    if percent > 100 as libc::c_int as libc::c_uint {
        percent = 100 as libc::c_int as UDWORD
    }
    light.position.x = (*psEffect).position.x as SDWORD;
    light.position.y = (*psEffect).position.y as SDWORD;
    light.position.z = (*psEffect).position.z as SDWORD;
    light.range =
        percent.wrapping_mul((*psEffect).radius as
                                 libc::c_uint).wrapping_mul(3 as libc::c_int
                                                                as
                                                                libc::c_uint).wrapping_div(100
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint);
    light.colour = LIGHT_RED as libc::c_int as UDWORD;
    processLight(&mut light);
    /* Time to update the frame number on the construction sprite */
    if gameTime.wrapping_sub((*psEffect).lastFrame) >
           (*psEffect).frameDelay as libc::c_uint {
        (*psEffect).lastFrame = gameTime;
        pos.x =
            (*psEffect).position.x as SDWORD +
                (rand() % (*psEffect).radius as libc::c_int -
                     rand() %
                         (2 as libc::c_int *
                              (*psEffect).radius as libc::c_int));
        pos.z =
            (*psEffect).position.z as SDWORD +
                (rand() % (*psEffect).radius as libc::c_int -
                     rand() %
                         (2 as libc::c_int *
                              (*psEffect).radius as libc::c_int));
        pos.y = map_Height(pos.x as UDWORD, pos.z as UDWORD) as int32;
        if (*psEffect).type_0 as libc::c_int ==
               FIRE_TYPE_SMOKY_BLUE as libc::c_int {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_FLAMETHROWER,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        /*
		pos.x = MAKEINT(psEffect->position.x);
		pos.y = MAKEINT(psEffect->position.y);
		pos.z = MAKEINT(psEffect->position.z);

		scatter.x = psEffect->radius; scatter.y = 0; scatter.z = psEffect->radius;
		addMultiEffect(&pos,&scatter,EFFECT_EXPLOSION,EXPLOSION_TYPE_SMALL,FALSE,NULL,2,0,0);
		*/
        if (*psEffect).type_0 as libc::c_int == FIRE_TYPE_SMOKY as libc::c_int
               ||
               (*psEffect).type_0 as libc::c_int ==
                   FIRE_TYPE_SMOKY_BLUE as libc::c_int {
            pos.x =
                (*psEffect).position.x as SDWORD +
                    (rand() % (*psEffect).radius as libc::c_int /
                         2 as libc::c_int -
                         rand() %
                             (2 as libc::c_int *
                                  (*psEffect).radius as libc::c_int /
                                  2 as libc::c_int));
            pos.z =
                (*psEffect).position.z as SDWORD +
                    (rand() % (*psEffect).radius as libc::c_int /
                         2 as libc::c_int -
                         rand() %
                             (2 as libc::c_int *
                                  (*psEffect).radius as libc::c_int /
                                  2 as libc::c_int));
            pos.y = map_Height(pos.x as UDWORD, pos.z as UDWORD) as int32;
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_DRIFTING_HIGH,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            pos.x =
                (*psEffect).position.x as SDWORD +
                    (rand() % (*psEffect).radius as libc::c_int -
                         rand() %
                             (2 as libc::c_int *
                                  (*psEffect).radius as libc::c_int));
            pos.z =
                (*psEffect).position.z as SDWORD +
                    (rand() % (*psEffect).radius as libc::c_int -
                         rand() %
                             (2 as libc::c_int *
                                  (*psEffect).radius as libc::c_int));
            pos.y = map_Height(pos.x as UDWORD, pos.z as UDWORD) as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
    }
    if gameTime.wrapping_sub((*psEffect).birthTime) >
           (*psEffect).lifeSpan as libc::c_uint {
        (*psEffect).status = ES_INACTIVE as libc::c_int as UBYTE;
        (*psEffect).control = 0 as libc::c_int as UBYTE;
        return
    };
}
//		pos.x = (MAKEINT(psEffect->position.x) + ((rand()%psEffect->radius) - (rand()%(2*psEffect->radius))));
//		pos.z = (MAKEINT(psEffect->position.z) + ((rand()%psEffect->radius) - (rand()%(2*psEffect->radius))));
//		pos.y = map_Height(pos.x,pos.z);
// ----------------------------------------------------------------------------------------
// ALL THE RENDER FUNCTIONS
// ----------------------------------------------------------------------------------------
/*
renderEffect:-
Calls the appropriate render routine for each type of effect
*/
#[no_mangle]
pub unsafe extern "C" fn renderEffect(mut psEffect: *mut EFFECT) {
    /* What type of effect are we dealing with? */
    match (*psEffect).group as libc::c_int {
        5 => { renderWaypointEffect(psEffect); }
        0 => { renderExplosionEffect(psEffect); }
        1 => { renderConstructionEffect(psEffect); }
        2 => { renderSmokeEffect(psEffect); }
        4 => { renderGravitonEffect(psEffect); }
        6 => { renderBloodEffect(psEffect); }
        7 => {
            /*	There is no display func for a destruction effect -
			it merely spawn other effects over time */
            renderDestructionEffect(psEffect);
        }
        10 => { }
        3 | 8 => { }
        11 => { renderFirework(psEffect); }
        _ => {
            debug(LOG_ERROR,
                  b"Weirdy class of effect passed to renderEffect\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
    };
}
// ----------------------------------------------------------------------------------------
/* drawing func for wapypoints . AJL. */
unsafe extern "C" fn renderWaypointEffect(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD centreX, centreZ;
    dv.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Push the indentity matrix */
    dv.y =
        (*psEffect).position.y as SDWORD as UDWORD as
            int32; /* Get the x,z translation components */
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Translate */
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    // set up lighting
//	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    pie_Draw3DShape((*psEffect).imd, 0 as libc::c_int, 0 as libc::c_int,
                    brightness, specular, 0 as libc::c_int, 0 as libc::c_int);
    //	pie_Draw3DShape(psEffect->imd, 0, 0, pie_MAX_BRIGHT_LEVEL, 0, pie_NO_BILINEAR, 0);
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn renderFirework(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD centreX, centreZ;
    /* these don't get rendered */
    if (*psEffect).type_0 as libc::c_int ==
           FIREWORK_TYPE_LAUNCHER as libc::c_int {
        return
    } /* Push the indentity matrix */
    dv.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Get the x,z translation components */
    dv.y =
        (*psEffect).position.y as SDWORD as UDWORD as int32; /* Translate */
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    pie_MatRotY(-player.r.y);
    pie_MatRotX(-player.r.x);
    //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
  //	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    pie_MatScale((*psEffect).size as UDWORD);
    pie_Draw3DShape((*psEffect).imd, (*psEffect).frameNumber as libc::c_int,
                    0 as libc::c_int, brightness, 0 as libc::c_int as UDWORD,
                    0x4 as libc::c_int, 164 as libc::c_int);
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
/* drawing func for blood. */
unsafe extern "C" fn renderBloodEffect(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD centreX, centreZ;
    dv.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Push the indentity matrix */
    dv.y =
        (*psEffect).position.y as SDWORD as UDWORD as
            int32; /* Get the x,z translation components */
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Translate */
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    pie_MatRotY(-player.r.y);
    pie_MatRotX(-player.r.x);
    pie_MatScale((*psEffect).size as UDWORD);
    // set up lighting
  //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
  //	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    pie_Draw3DShape(getImdFromIndex(MI_BLOOD as libc::c_int as UDWORD),
                    (*psEffect).frameNumber as libc::c_int, 0 as libc::c_int,
                    brightness, specular, 0x2 as libc::c_int,
                    128 as libc::c_int);
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn renderDestructionEffect(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut div: FRACT = 0.;
    let mut percent: SDWORD = 0;
    //SDWORD	centreX,centreZ;
    let mut brightness: UDWORD = 0; /* Push the indentity matrix */
    let mut specular: UDWORD = 0; /* Get the x,z translation components */
    if (*psEffect).type_0 as libc::c_int !=
           DESTRUCTION_TYPE_SKYSCRAPER as libc::c_int {
        return
    } /* Translate */
    dv.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; //temporary!
    dv.y = (*psEffect).position.y as SDWORD as UDWORD as int32;
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    div =
        gameTime.wrapping_sub((*psEffect).birthTime) as FRACT /
            (*psEffect).lifeSpan as libc::c_int as libc::c_float;
    if div as libc::c_double > 1.0f64 { div = 1.0f64 as FRACT }
    div = (1.0f64 - div as libc::c_double) as FRACT;
    percent = (div * 256 as libc::c_int as libc::c_float) as SDWORD;
    //get fog value
   //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
  //	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    if gamePaused() == 0 {
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        1 as libc::c_int * 1 as libc::c_int / 2 as libc::c_int
                        -
                        rand() %
                            (2 as libc::c_int *
                                 (65536 as libc::c_int / 360 as libc::c_int *
                                      1 as libc::c_int * 1 as libc::c_int /
                                      2 as libc::c_int)));
    }
    pie_Draw3DShape((*psEffect).imd, 0 as libc::c_int, 0 as libc::c_int,
                    brightness, 0 as libc::c_int as UDWORD,
                    0x20 as libc::c_int, percent);
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn rejectLandLight(mut type_0: LAND_LIGHT_SPEC)
 -> BOOL {
    let mut timeSlice: UDWORD = 0;
    timeSlice = gameTime.wrapping_rem(2000 as libc::c_int as libc::c_uint);
    if timeSlice < 400 as libc::c_int as libc::c_uint {
        if type_0 as libc::c_uint == LL_MIDDLE as libc::c_int as libc::c_uint
           {
            return 0 as libc::c_int
        } else { return 1 as libc::c_int }
        // reject all expect middle
    } else if timeSlice < 800 as libc::c_int as libc::c_uint {
        if type_0 as libc::c_uint == LL_OUTER as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
        // reject only outer
    } else if timeSlice < 1200 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
        //reject none
    } else if timeSlice < 1600 as libc::c_int as libc::c_uint {
        if type_0 as libc::c_uint == LL_OUTER as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
        // reject only outer
    } else if type_0 as libc::c_uint ==
                  LL_MIDDLE as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
// reject all expect middle
// ----------------------------------------------------------------------------------------
// ---- The render functions - every group type of effect has a distinct one
// ----------------------------------------------------------------------------------------
/* Renders the standard explosion effect */
unsafe extern "C" fn renderExplosionEffect(mut psEffect: *mut EFFECT) {
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut percent: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //	SDWORD centreX, centreZ;
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_LAND_LIGHT as libc::c_int {
        if rejectLandLight((*psEffect).specific as LAND_LIGHT_SPEC) != 0 {
            return
        }
    } /* Push the indentity matrix */
    dv.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32; /* Get the x,z translation components */
    dv.y =
        (*psEffect).position.y as SDWORD as UDWORD as int32; /* Translate */
    dv.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    pie_MatBegin();
    pie_TRANSLATE(dv.x, dv.y, dv.z);
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Bit in comments - doesn't quite work yet? */
    if (*psEffect).control as libc::c_int & 0x10 as libc::c_int != 0 {
        /* Always face the viewer! */
/*		TEST_FLIPPED_Y(psEffect) ? iV_MatrixRotateY(-player.r.y+iV_DEG(180)) :*/
        pie_MatRotY(-player.r.y);
        /*		TEST_FLIPPED_X(psEffect) ? iV_MatrixRotateX(-player.r.x+iV_DEG(180)) :*/
        pie_MatRotX(-player.r.x);
    }
    /* Tesla explosions diminish in size */
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_TESLA as libc::c_int {
        percent =
            gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                         as
                                                                                                         libc::c_uint)
                as SDWORD;
        if percent < 0 as libc::c_int { percent = 0 as libc::c_int }
        if percent > 45 as libc::c_int { percent = 45 as libc::c_int }
        pie_MatScale(((*psEffect).size as libc::c_int - percent) as UDWORD);
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_PLASMA as libc::c_int {
        percent =
            gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                         as
                                                                                                         libc::c_uint)
                as SDWORD / 3 as libc::c_int;
        pie_MatScale((0 as libc::c_int + percent) as UDWORD);
    } else { pie_MatScale((*psEffect).size as UDWORD); }
    //get fog value
//	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_PLASMA as libc::c_int {
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, brightness,
                        0 as libc::c_int as UDWORD, 0x4 as libc::c_int,
                        224 as libc::c_int);
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_KICKUP as libc::c_int {
        /* not transparent */
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, 0x2 as libc::c_int as UDWORD,
                        128 as libc::c_int as UDWORD, 0 as libc::c_int,
                        0 as libc::c_int);
    } else {
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, brightness,
                        0 as libc::c_int as UDWORD, 0x4 as libc::c_int,
                        164 as libc::c_int);
    }
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn renderGravitonEffect(mut psEffect: *mut EFFECT) {
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD	centreX,centreZ;
  //	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
  //	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    /* Establish world position */
    vec.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    vec.y = (*psEffect).position.y as SDWORD as UDWORD as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    /* Push matrix */
    pie_MatBegin();
    /* Move to position */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    /* Offset from camera */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Move to camera reference */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    pie_MatRotX((*psEffect).rotation.x);
    pie_MatRotY((*psEffect).rotation.y);
    pie_MatRotZ((*psEffect).rotation.z);
    /* Buildings emitted by gravitons are chunkier */
    if (*psEffect).type_0 as libc::c_int ==
           GRAVITON_TYPE_EMITTING_ST as libc::c_int {
        /* Twice as big - 150 percent */
        pie_MatScale((*psEffect).size as UDWORD);
    } else { pie_MatScale(100 as libc::c_int as UDWORD); }
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    pie_Draw3DShape((*psEffect).imd, (*psEffect).frameNumber as libc::c_int,
                    0 as libc::c_int, brightness, specular, 0 as libc::c_int,
                    0 as libc::c_int);
    /* Pop the matrix */
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
/*
renderConstructionEffect:-
Renders the standard construction effect */
unsafe extern "C" fn renderConstructionEffect(mut psEffect: *mut EFFECT) {
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut null: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut percent: SDWORD = 0;
    let mut translucency: UDWORD = 0;
    let mut size: UDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD centreX, centreZ;
    /* No rotation about arbitrary axis */
    null.z = 0 as libc::c_int;
    null.y = null.z;
    null.x = null.y;
    /* Establish world position */
    vec.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    vec.y = (*psEffect).position.y as SDWORD as UDWORD as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    /* Push matrix */
    pie_MatBegin();
    /* Move to position */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    /* Offset from camera */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Move to camera reference */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Bit in comments doesn't quite work yet? */
    if (*psEffect).control as libc::c_int & 0x10 as libc::c_int != 0 {
        /*		TEST_FLIPPED_Y(psEffect) ? iV_MatrixRotateY(-player.r.y+iV_DEG(180)) :*/
        pie_MatRotY(-player.r.y);
        /*		TEST_FLIPPED_X(psEffect) ? iV_MatrixRotateX(-player.r.x+iV_DEG(180)) :*/
        pie_MatRotX(-player.r.x);
    }
    /* Scale size according to age */
    percent =
        gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                     as
                                                                                                     libc::c_uint)
            as SDWORD;
    if percent < 0 as libc::c_int { percent = 0 as libc::c_int }
    if percent > 100 as libc::c_int { percent = 100 as libc::c_int }
    /* Make imds be transparent on 3dfx */
    if percent < 50 as libc::c_int {
        translucency = (percent * 2 as libc::c_int) as UDWORD
    } else {
        translucency =
            ((100 as libc::c_int - percent) * 2 as libc::c_int) as UDWORD
    }
    translucency =
        (translucency as
             libc::c_uint).wrapping_add(10 as libc::c_int as libc::c_uint) as
            UDWORD as UDWORD;
    size = (2 as libc::c_int as libc::c_uint).wrapping_mul(translucency);
    if size > 90 as libc::c_int as libc::c_uint {
        size = 90 as libc::c_int as UDWORD
    }
    pie_MatScale(size);
    // set up lighting
//	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular);
    pie_Draw3DShape((*psEffect).imd, (*psEffect).frameNumber as libc::c_int,
                    0 as libc::c_int, brightness, specular,
                    0x2 as libc::c_int, translucency as UBYTE as libc::c_int);
    //	pie_Draw3DShape(psEffect->imd, psEffect->frameNumber, 0, pie_MAX_BRIGHT_LEVEL, 0, pie_TRANSLUCENT, (UBYTE)(40+percent));
//	pie_Draw3DShape(psEffect->imd, psEffect->frameNumber, 0, pie_MAX_BRIGHT_LEVEL, 0, pie_TRANSLUCENT, (UBYTE)(130-percent));
    /* Pop the matrix */
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
/*
renderSmokeEffect:-
Renders the standard smoke effect - it is now scaled in real-time as well
*/
unsafe extern "C" fn renderSmokeEffect(mut psEffect: *mut EFFECT) {
    let mut percent: UDWORD = 0;
    let mut transparency: UDWORD = 0 as libc::c_int as UDWORD;
    let mut vec: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut rx: SDWORD = 0;
    let mut rz: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //SDWORD centreX, centreZ;
    /* Establish world position */
    vec.x =
        ((*psEffect).position.x as SDWORD as
             UDWORD).wrapping_sub(player.p.x as
                                      libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    vec.y = (*psEffect).position.y as SDWORD as UDWORD as int32;
    vec.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psEffect).position.z
                                                                     as SDWORD
                                                                     as
                                                                     UDWORD).wrapping_sub(player.p.z
                                                                                              as
                                                                                              libc::c_uint))
            as int32;
    /* Push matrix */
    pie_MatBegin();
    /* Move to position */
    pie_TRANSLATE(vec.x, vec.y, vec.z);
    /* Offset from camera */
    rx = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    rz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Move to camera reference */
    pie_TRANSLATE(rx, 0 as libc::c_int, -rz);
    /* Bit in comments doesn't quite work yet? */
    if (*psEffect).control as libc::c_int & 0x10 as libc::c_int != 0 {
        /* Always face the viewer! */
/*		TEST_FLIPPED_Y(psEffect) ? iV_MatrixRotateY(-player.r.y+iV_DEG(180)) : */
        pie_MatRotY(-player.r.y);
        /*		TEST_FLIPPED_X(psEffect) ? iV_MatrixRotateX(-player.r.x+iV_DEG(180)) : */
        pie_MatRotX(-player.r.x);
    }
    /* Small smoke - used for the droids */
//		if(psEffect->type == SMOKE_TYPE_DRIFTING_SMALL OR psEffect->type == SMOKE_TYPE_TRAIL)
    if (*psEffect).control as libc::c_int & 0x20 as libc::c_int != 0 {
        //test additive
        //Constant alpha
        percent =
            gameTime.wrapping_sub((*psEffect).birthTime).wrapping_mul(100 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint).wrapping_div((*psEffect).lifeSpan
                                                                                                         as
                                                                                                         libc::c_uint)
                as SDWORD as UDWORD;
        pie_MatScale(percent.wrapping_add((*psEffect).baseScale as
                                              libc::c_uint));
        transparency =
            (130 as libc::c_int as
                 libc::c_uint).wrapping_mul((100 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(percent)).wrapping_div(100
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
    }
    // set up lighting
//	centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
//	centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
    brightness =
        lightDoFogAndIllumination(255 as libc::c_int as UBYTE,
                                  getCentreX() -
                                      (*psEffect).position.x as SDWORD,
                                  getCentreZ() -
                                      (*psEffect).position.z as SDWORD,
                                  &mut specular); //JPS smoke strength increased for d3d 12 may 99
    transparency =
        transparency.wrapping_mul(3 as libc::c_int as
                                      libc::c_uint).wrapping_div(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
    /* Make imds be transparent on 3dfx */
    if (*psEffect).type_0 as libc::c_int == SMOKE_TYPE_STEAM as libc::c_int {
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, brightness, specular,
                        0x2 as libc::c_int,
                        128 as libc::c_int as UBYTE as libc::c_int /
                            2 as libc::c_int);
    } else if (*psEffect).type_0 as libc::c_int ==
                  SMOKE_TYPE_TRAIL as libc::c_int {
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, brightness, specular,
                        0x2 as libc::c_int,
                        (2 as libc::c_int as
                             libc::c_uint).wrapping_mul(transparency).wrapping_div(3
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                            as UBYTE as libc::c_int);
    } else {
        pie_Draw3DShape((*psEffect).imd,
                        (*psEffect).frameNumber as libc::c_int,
                        0 as libc::c_int, brightness, specular,
                        0x2 as libc::c_int,
                        transparency as UBYTE as libc::c_int /
                            2 as libc::c_int);
    }
    /* Pop the matrix */
    pie_MatEnd();
}
// ----------------------------------------------------------------------------------------
// ALL THE SETUP FUNCTIONS
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetUpFirework(mut psEffect: *mut EFFECT) {
    let mut camExtra: UDWORD = 0;
    if (*psEffect).type_0 as libc::c_int ==
           FIREWORK_TYPE_LAUNCHER as libc::c_int {
        (*psEffect).velocity.x =
            (200 as libc::c_int - rand() % 400 as libc::c_int) as FRACT;
        (*psEffect).velocity.z =
            (200 as libc::c_int - rand() % 400 as libc::c_int) as FRACT;
        // not actually drawn
        (*psEffect).velocity.y =
            (400 as libc::c_int + rand() % 200 as libc::c_int) as
                FRACT; //height
        (*psEffect).lifeSpan =
            (1000 as libc::c_int * 3 as libc::c_int) as
                UWORD; //height it goes off
        (*psEffect).radius =
            (80 as libc::c_int + rand() % 150 as libc::c_int) as
                UWORD; //height
        camExtra = 0 as libc::c_int as UDWORD;
        if getCampaignNumber() != 1 as libc::c_int as libc::c_uint {
            camExtra =
                (camExtra as
                     libc::c_uint).wrapping_add((rand() % 200 as libc::c_int)
                                                    as libc::c_uint) as UDWORD
                    as UDWORD
        }
        (*psEffect).size =
            (300 as libc::c_int + rand() % 300 as libc::c_int) as UWORD;
        (*psEffect).imd =
            getImdFromIndex(MI_FIREWORK as libc::c_int as UDWORD)
    } else {
        (*psEffect).velocity.x =
            (20 as libc::c_int - rand() % 40 as libc::c_int) as FRACT;
        (*psEffect).velocity.z =
            (20 as libc::c_int - rand() % 40 as libc::c_int) as FRACT;
        (*psEffect).velocity.y =
            (0 as libc::c_int -
                 (20 as libc::c_int + rand() % 40 as libc::c_int)) as FRACT;
        (*psEffect).lifeSpan =
            (1000 as libc::c_int * 4 as libc::c_int) as UWORD;
        /* setup the imds */
        match rand() % 3 as libc::c_int {
            0 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_FIREWORK as libc::c_int as
                                        UDWORD); //size of graphic
                (*psEffect).size = 45 as libc::c_int as UWORD
            }
            1 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_SNOW as libc::c_int as
                                        UDWORD); //size of graphic
                (*psEffect).control =
                    ((*psEffect).control as libc::c_int | 0x4 as libc::c_int)
                        as UBYTE; //size of graphic
                (*psEffect).size = 60 as libc::c_int as UWORD
            }
            _ => {
                (*psEffect).imd =
                    getImdFromIndex(MI_FLAME as libc::c_int as UDWORD);
                (*psEffect).size = 40 as libc::c_int as UWORD
            }
        }
    }
    (*psEffect).frameDelay =
        ((25 as libc::c_int + rand() % 40 as libc::c_int) * 2 as libc::c_int)
            as UWORD;
}
/* There is no render destruction effect! */
// ----------------------------------------------------------------------------------------
// ---- The set up functions - every type has one
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupSmoke(mut psEffect: *mut EFFECT) {
    /* everything except steam drifts about */
    if (*psEffect).type_0 as libc::c_int == SMOKE_TYPE_STEAM as libc::c_int {
        /* Only upwards */
        (*psEffect).velocity.x = 0 as libc::c_int as FRACT;
        (*psEffect).velocity.z = 0 as libc::c_int as FRACT
    } else if (*psEffect).type_0 as libc::c_int ==
                  SMOKE_TYPE_BILLOW as libc::c_int {
        (*psEffect).velocity.x =
            (10 as libc::c_int - rand() % 20 as libc::c_int) as FRACT;
        (*psEffect).velocity.z =
            (10 as libc::c_int - rand() % 20 as libc::c_int) as FRACT
    } else {
        (*psEffect).velocity.x = (rand() % 20 as libc::c_int) as FRACT;
        (*psEffect).velocity.z =
            (10 as libc::c_int - rand() % 20 as libc::c_int) as FRACT
    }
    /* Steam isn't cyclic  - it doesn't grow with time either */
    if (*psEffect).type_0 as libc::c_int != SMOKE_TYPE_STEAM as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x4 as libc::c_int) as
                UBYTE;
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x20 as libc::c_int) as
                UBYTE
    }
    match (*psEffect).type_0 as libc::c_int {
        21 => {
            (*psEffect).imd =
                getImdFromIndex(MI_SMALL_SMOKE as libc::c_int as UDWORD);
            (*psEffect).lifeSpan =
                (6000 as libc::c_int + rand() % 3000 as libc::c_int) as UWORD;
            (*psEffect).velocity.y =
                (35 as libc::c_int + rand() % 30 as libc::c_int) as FRACT;
            (*psEffect).baseScale = 40 as libc::c_int as UBYTE
        }
        22 => {
            (*psEffect).imd =
                getImdFromIndex(MI_SMALL_SMOKE as libc::c_int as UDWORD);
            (*psEffect).lifeSpan =
                (6000 as libc::c_int + rand() % 3000 as libc::c_int) as UWORD;
            (*psEffect).velocity.y =
                (40 as libc::c_int + rand() % 45 as libc::c_int) as FRACT;
            (*psEffect).baseScale = 25 as libc::c_int as UBYTE
        }
        23 => {
            (*psEffect).imd =
                getImdFromIndex(MI_SMALL_SMOKE as libc::c_int as UDWORD);
            (*psEffect).lifeSpan =
                (3000 as libc::c_int + rand() % 3000 as libc::c_int) as UWORD;
            (*psEffect).velocity.y =
                (25 as libc::c_int + rand() % 35 as libc::c_int) as FRACT;
            (*psEffect).baseScale = 17 as libc::c_int as UBYTE
        }
        24 => {
            (*psEffect).imd =
                getImdFromIndex(MI_SMALL_SMOKE as libc::c_int as UDWORD);
            (*psEffect).lifeSpan =
                (3000 as libc::c_int + rand() % 3000 as libc::c_int) as UWORD;
            (*psEffect).velocity.y =
                (10 as libc::c_int + rand() % 20 as libc::c_int) as FRACT;
            (*psEffect).baseScale = 80 as libc::c_int as UBYTE
        }
        25 => {
            (*psEffect).imd =
                getImdFromIndex(MI_SMALL_STEAM as libc::c_int as UDWORD);
            (*psEffect).velocity.y = (rand() % 5 as libc::c_int) as FRACT
        }
        26 => {
            (*psEffect).imd =
                getImdFromIndex(MI_TRAIL as libc::c_int as UDWORD);
            (*psEffect).lifeSpan = 1200 as libc::c_int as UWORD;
            (*psEffect).velocity.y =
                (5 as libc::c_int + rand() % 10 as libc::c_int) as FRACT;
            (*psEffect).baseScale = 25 as libc::c_int as UBYTE
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weird smoke type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"effects.c\x00" as *const u8 as *const libc::c_char,
                      2238 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"effectSetupSmoke\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    /* It always faces you */
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x10 as libc::c_int) as UBYTE;
    (*psEffect).frameDelay =
        (40 as libc::c_int + rand() % 30 as libc::c_int) as UWORD;
    /* Randomly flip gfx for variation */
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x1 as libc::c_int) as UBYTE
    }
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x2 as libc::c_int) as UBYTE
    };
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetUpSatLaser(mut psEffect: *mut EFFECT) {
    /* Does nothing at all..... Runs only for one frame! */
    (*psEffect).baseScale = 1 as libc::c_int as UBYTE;
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupGraviton(mut psEffect: *mut EFFECT) {
    match (*psEffect).type_0 as libc::c_int {
        20 => {
            (*psEffect).velocity.x =
                (50 as libc::c_int - rand() % 100 as libc::c_int) as FRACT;
            (*psEffect).velocity.z =
                (50 as libc::c_int - rand() % 100 as libc::c_int) as FRACT;
            (*psEffect).velocity.y = 12 as libc::c_int as FRACT
        }
        19 => {
            (*psEffect).velocity.x =
                (200 as libc::c_int - rand() % 300 as libc::c_int) as FRACT;
            (*psEffect).velocity.z =
                (200 as libc::c_int - rand() % 300 as libc::c_int) as FRACT;
            (*psEffect).velocity.y =
                5 as libc::c_int as libc::c_float *
                    (300 as libc::c_int + rand() % 100 as libc::c_int) as
                        FRACT / 4 as libc::c_int as libc::c_float;
            (*psEffect).size =
                (120 as libc::c_int + rand() % 30 as libc::c_int) as UWORD
        }
        18 => {
            (*psEffect).velocity.x =
                (200 as libc::c_int - rand() % 300 as libc::c_int) as FRACT /
                    2 as libc::c_int as libc::c_float;
            (*psEffect).velocity.z =
                (200 as libc::c_int - rand() % 300 as libc::c_int) as FRACT /
                    2 as libc::c_int as libc::c_float;
            (*psEffect).velocity.y =
                (300 as libc::c_int + rand() % 100 as libc::c_int) as FRACT
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy type of graviton\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"effects.c\x00" as *const u8 as *const libc::c_char,
                      2286 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"effectSetupGraviton\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    (*psEffect).rotation.x =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 360 as libc::c_int);
    (*psEffect).rotation.z =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 360 as libc::c_int);
    (*psEffect).rotation.y =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 360 as libc::c_int);
    (*psEffect).spin.x =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 100 as libc::c_int + 20 as libc::c_int);
    (*psEffect).spin.z =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 100 as libc::c_int + 20 as libc::c_int);
    (*psEffect).spin.y =
        65536 as libc::c_int / 360 as libc::c_int *
            (rand() % 100 as libc::c_int + 20 as libc::c_int);
    /* Gravitons are essential */
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x8 as libc::c_int) as UBYTE;
    if (*psEffect).type_0 as libc::c_int ==
           GRAVITON_TYPE_GIBLET as libc::c_int {
        (*psEffect).frameDelay =
            (200 as libc::c_int + rand() % 100 as libc::c_int) as UWORD
    } else {
        (*psEffect).frameDelay =
            (100 as libc::c_int + rand() % 50 as libc::c_int) as UWORD
    };
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupExplosion(mut psEffect: *mut EFFECT) {
    /* Get an imd if it's not established */
    if (*psEffect).imd.is_null() {
        match (*psEffect).type_0 as libc::c_int {
            0 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_EXPLOSION_SMALL as libc::c_int as
                                        UDWORD); // change this
                (*psEffect).size =
                    (6 as libc::c_int *
                         (110 as libc::c_int +
                              (30 as libc::c_int -
                                   rand() % 60 as libc::c_int)) /
                         5 as libc::c_int) as UBYTE as UWORD
            }
            1 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_EXPLOSION_SMALL as libc::c_int as
                                        UDWORD); // change this
                (*psEffect).size =
                    (80 as libc::c_int as libc::c_uint).wrapping_add(auxVar)
                        as UBYTE as UWORD
            }
            2 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_EXPLOSION_MEDIUM as libc::c_int as
                                        UDWORD); // Landing lights are permanent and cyclic
                (*psEffect).size =
                    (110 as libc::c_int +
                         (30 as libc::c_int - rand() % 60 as libc::c_int)) as
                        UBYTE as UWORD
            }
            3 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_EXPLOSION_MEDIUM as libc::c_int as
                                        UDWORD); //resGetData("IMD","blbhq.pie");
                (*psEffect).size =
                    ((110 as libc::c_int +
                          (30 as libc::c_int - rand() % 60 as libc::c_int)) as
                         UBYTE as libc::c_int * 2 as libc::c_int) as UWORD
            }
            8 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_FLAME as libc::c_int as UDWORD);
                (*psEffect).size =
                    (80 as libc::c_int as libc::c_uint).wrapping_add(auxVar)
                        as UBYTE as UWORD
            }
            9 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_FLAME as libc::c_int as UDWORD);
                (*psEffect).size =
                    (10 as libc::c_int as libc::c_uint).wrapping_add(auxVar)
                        as UBYTE as UWORD
            }
            11 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_TESLA as libc::c_int as UDWORD);
                (*psEffect).size = 60 as libc::c_int as UWORD
            }
            12 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_MFLARE as libc::c_int as UDWORD);
                (*psEffect).size = 100 as libc::c_int as UWORD
            }
            10 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_TESLA as libc::c_int as UDWORD);
                (*psEffect).size = 100 as libc::c_int as UWORD;
                (*psEffect).velocity.y = 170 as libc::c_int as FRACT
            }
            14 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_KICK as libc::c_int as UDWORD);
                (*psEffect).size = 100 as libc::c_int as UWORD
            }
            13 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_PLASMA as libc::c_int as UDWORD);
                (*psEffect).size = 0 as libc::c_int as UWORD;
                (*psEffect).velocity.y = 0.0f32
            }
            15 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_LANDING as libc::c_int as UDWORD);
                (*psEffect).size = 120 as libc::c_int as UWORD;
                (*psEffect).specific = ellSpec as UBYTE;
                (*psEffect).velocity.y = 0.0f32;
                (*psEffect).control =
                    ((*psEffect).control as libc::c_int | 0x8 as libc::c_int)
                        as UBYTE
            }
            16 => {
                (*psEffect).imd =
                    getImdFromIndex(MI_SHOCK as libc::c_int as UDWORD);
                (*psEffect).size = 50 as libc::c_int as UWORD;
                (*psEffect).velocity.y = 0.0f32
            }
            _ => { }
        }
    }
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_FLAMETHROWER as libc::c_int {
        (*psEffect).frameDelay = 45 as libc::c_int as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_LASER as libc::c_int {
        (*psEffect).frameDelay =
            ((25 as libc::c_int + rand() % 40 as libc::c_int) /
                 2 as libc::c_int) as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_TESLA as libc::c_int {
        (*psEffect).frameDelay = 65 as libc::c_int as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_PLASMA as libc::c_int {
        (*psEffect).frameDelay = 45 as libc::c_int as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  EXPLOSION_TYPE_LAND_LIGHT as libc::c_int {
        (*psEffect).frameDelay = 120 as libc::c_int as UWORD
    } else {
        (*psEffect).frameDelay =
            (25 as libc::c_int + rand() % 40 as libc::c_int) as UWORD
    }
    if (*psEffect).type_0 as libc::c_int ==
           EXPLOSION_TYPE_SHOCKWAVE as libc::c_int {
        (*psEffect).lifeSpan = 1000 as libc::c_int as UWORD
    } else {
        (*psEffect).lifeSpan =
            ((*psEffect).frameDelay as libc::c_int *
                 (*(*psEffect).imd).numFrames as libc::c_int) as UWORD
    }
    if (*psEffect).type_0 as libc::c_int !=
           EXPLOSION_TYPE_NOT_FACING as libc::c_int &&
           (*psEffect).type_0 as libc::c_int !=
               EXPLOSION_TYPE_SHOCKWAVE as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x10 as libc::c_int) as
                UBYTE
    }
    /* Set how long it lasts */
    /* Randomly flip x and y for variation */
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x1 as libc::c_int) as UBYTE
    }
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x2 as libc::c_int) as UBYTE
    };
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupConstruction(mut psEffect: *mut EFFECT) {
    (*psEffect).velocity.x = 0 as libc::c_int as FRACT; //(1-rand()%3);
    (*psEffect).velocity.z = 0 as libc::c_int as FRACT; //(1-rand()%3);
    (*psEffect).velocity.y =
        (0 as libc::c_int - rand() % 3 as libc::c_int) as FRACT;
    (*psEffect).frameDelay =
        (40 as libc::c_int + rand() % 30 as libc::c_int) as UWORD;
    (*psEffect).imd =
        getImdFromIndex(MI_CONSTRUCTION as libc::c_int as UDWORD);
    (*psEffect).lifeSpan = 5000 as libc::c_int as UWORD;
    /* These effects always face you */
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x10 as libc::c_int) as UBYTE;
    /* It's a cyclic anim - dies on age */
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x4 as libc::c_int) as UBYTE;
    /* Randomly flip the construction graphics in x and y for variation */
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x1 as libc::c_int) as UBYTE
    }
    if rand() % 2 as libc::c_int == 0 as libc::c_int {
        (*psEffect).control =
            ((*psEffect).control as libc::c_int | 0x2 as libc::c_int) as UBYTE
    };
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupFire(mut psEffect: *mut EFFECT) {
    (*psEffect).frameDelay =
        300 as libc::c_int as UWORD; // needs to be investigated...
    (*psEffect).radius = auxVar as UWORD; // needs to be investigated
    (*psEffect).lifeSpan = auxVarSec as UWORD;
    (*psEffect).birthTime = gameTime;
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x8 as libc::c_int) as UBYTE;
}
//static void	effectSetupDust			( EFFECT *psEffect );
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupWayPoint(mut psEffect: *mut EFFECT) {
    (*psEffect).imd = pProximityMsgIMD;
    /* These effects musnt make way for others */
    (*psEffect).control =
        ((*psEffect).control as libc::c_int | 0x8 as libc::c_int) as UBYTE;
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupBlood(mut psEffect: *mut EFFECT) {
    (*psEffect).frameDelay = 150 as libc::c_int as UWORD;
    (*psEffect).velocity.y =
        -(20 as libc::c_int + rand() % 20 as libc::c_int) as FRACT;
    (*psEffect).imd = getImdFromIndex(MI_BLOOD as libc::c_int as UDWORD);
    (*psEffect).size =
        (100 as libc::c_int +
             (30 as libc::c_int - rand() % 60 as libc::c_int)) as UBYTE as
            UWORD;
}
// ----------------------------------------------------------------------------------------
unsafe extern "C" fn effectSetupDestruction(mut psEffect: *mut EFFECT) {
    if (*psEffect).type_0 as libc::c_int ==
           DESTRUCTION_TYPE_SKYSCRAPER as libc::c_int {
        (*psEffect).lifeSpan =
            (3 as libc::c_int * 1000 as libc::c_int / 2 as libc::c_int +
                 rand() % 1000 as libc::c_int) as UWORD;
        (*psEffect).frameDelay =
            (200 as libc::c_int / 2 as libc::c_int) as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  DESTRUCTION_TYPE_DROID as libc::c_int {
        /* It's all over quickly for droids */
        (*psEffect).lifeSpan =
            (3 as libc::c_int * 1000 as libc::c_int / 2 as libc::c_int) as
                UWORD;
        (*psEffect).frameDelay = 200 as libc::c_int as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  DESTRUCTION_TYPE_WALL_SECTION as libc::c_int ||
                  (*psEffect).type_0 as libc::c_int ==
                      DESTRUCTION_TYPE_FEATURE as libc::c_int {
        (*psEffect).lifeSpan =
            (7 as libc::c_int * 1000 as libc::c_int / 2 as libc::c_int /
                 4 as libc::c_int) as UWORD;
        (*psEffect).frameDelay =
            (200 as libc::c_int / 2 as libc::c_int) as UWORD
    } else if (*psEffect).type_0 as libc::c_int ==
                  DESTRUCTION_TYPE_POWER_STATION as libc::c_int {
        (*psEffect).lifeSpan =
            (7 as libc::c_int * 1000 as libc::c_int / 2 as libc::c_int /
                 2 as libc::c_int) as UWORD;
        (*psEffect).frameDelay =
            (200 as libc::c_int / 4 as libc::c_int) as UWORD
    } else {
        /* building's destruction is longer */
        (*psEffect).lifeSpan =
            (7 as libc::c_int * 1000 as libc::c_int / 2 as libc::c_int) as
                UWORD;
        (*psEffect).frameDelay =
            (200 as libc::c_int / 2 as libc::c_int) as UWORD
    };
}
// ----------------------------------------------------------------------------------------
//void	initPerimeterSmoke			( EFFECT *psEffect );
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn initPerimeterSmoke(mut pImd: *mut iIMDShape,
                                            mut x: UDWORD, mut y: UDWORD,
                                            mut z: UDWORD) {
    let mut i: SDWORD = 0; //(varEnd-varStart)/FX_PER_EDGE;
    let mut inStart: SDWORD = 0; //(varEnd-varStart)/FX_PER_EDGE;
    let mut inEnd: SDWORD = 0;
    let mut varStart: SDWORD = 0;
    let mut varEnd: SDWORD = 0;
    let mut varStride: SDWORD = 0;
    let mut shift: SDWORD = 0 as libc::c_int;
    let mut base: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    base.x = x as int32;
    base.y = y as int32;
    base.z = z as int32;
    varStart = (*pImd).xmin - 16 as libc::c_int;
    varEnd = (*pImd).xmax + 16 as libc::c_int;
    varStride = 24 as libc::c_int;
    inStart = (*pImd).zmin - 16 as libc::c_int;
    inEnd = (*pImd).zmax + 16 as libc::c_int;
    i = varStart;
    while i < varEnd {
        shift = 16 as libc::c_int - rand() % 32 as libc::c_int;
        pos.x = base.x + i + shift;
        pos.y = base.y;
        pos.z = base.z + inStart + shift;
        if rand() % 6 as libc::c_int == 1 as libc::c_int {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_BILLOW,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        pos.x = base.x + i + shift;
        pos.y = base.y;
        pos.z = base.z + inEnd + shift;
        if rand() % 6 as libc::c_int == 1 as libc::c_int {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_BILLOW,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        i += varStride
    }
    varStart = (*pImd).zmin - 16 as libc::c_int;
    varEnd = (*pImd).zmax + 16 as libc::c_int;
    varStride = 24 as libc::c_int;
    inStart = (*pImd).xmin - 16 as libc::c_int;
    inEnd = (*pImd).xmax + 16 as libc::c_int;
    i = varStart;
    while i < varEnd {
        pos.x = base.x + inStart + shift;
        pos.y = base.y;
        pos.z = base.z + i + shift;
        if rand() % 6 as libc::c_int == 1 as libc::c_int {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_BILLOW,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        pos.x = base.x + inEnd + shift;
        pos.y = base.y;
        pos.z = base.z + i + shift;
        if rand() % 6 as libc::c_int == 1 as libc::c_int {
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            addEffect(&mut pos, EFFECT_SMOKE, SMOKE_TYPE_BILLOW,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        i += varStride
    };
}
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn getNumEffects() -> UDWORD { return numEffects; }
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn EffectGetNumFrames(mut psEffect: *mut EFFECT)
 -> UDWORD {
    return (*(*psEffect).imd).numFrames as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn IMDGetNumFrames(mut Shape: *mut iIMDShape)
 -> UDWORD {
    return (*Shape).numFrames as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn IMDGetAnimInterval(mut Shape: *mut iIMDShape)
 -> UDWORD {
    return (*Shape).animInterval as UDWORD;
}
/* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
#[no_mangle]
pub unsafe extern "C" fn effectGiveAuxVar(mut var: UDWORD) { auxVar = var; }
// naughty
#[no_mangle]
pub unsafe extern "C" fn effectGiveAuxVarSec(mut var: UDWORD) {
    auxVarSec = var;
}
// ----------------------------------------------------------------------------------------
/* Runs all the spot effect stuff for the droids - adding of dust and the like... */
#[no_mangle]
pub unsafe extern "C" fn effectDroidUpdates() {
    let mut i: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut partition: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut xBehind: SDWORD = 0;
    let mut yBehind: SDWORD = 0;
    /* Go through all players */
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        /* Now go through all their droids */
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() {
            /* Gets it's group number */
            partition =
                (*psDroid).id.wrapping_rem(101 as libc::c_int as
                                               libc::c_uint);
            /* Right frame to process? */
            if partition ==
                   frameGetFrameNumber().wrapping_rem(101 as libc::c_int as
                                                          libc::c_uint) &&
                   rand() % 4 as libc::c_int == 0 as libc::c_int {
                /* Sufficent time since last update? - The EQUALS comparison is needed */
                if gameTime >=
                       lastUpdateDroids[partition as
                                            usize].wrapping_add(500 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                   {
                    /* Store away when we last processed this group */
                    lastUpdateDroids[partition as usize] = gameTime;
                    /*	Now add some dust at it's arse end if it's moving or skidding.
						The check that it's not 0 is probably not sufficient.
					*/
                    if (*psDroid).sMove.speed as SDWORD != 0 as libc::c_int {
                        /* Present direction is important */
                        xBehind =
                            50 as libc::c_int *
                                *aSinTable.as_mut_ptr().offset(((65536 as
                                                                     libc::c_int
                                                                     /
                                                                     360 as
                                                                         libc::c_int
                                                                     *
                                                                     (*psDroid).direction
                                                                         as
                                                                         libc::c_int)
                                                                    as uint16
                                                                    as
                                                                    libc::c_int
                                                                    >>
                                                                    4 as
                                                                        libc::c_int)
                                                                   as isize)
                                >> 12 as libc::c_int;
                        yBehind =
                            50 as libc::c_int *
                                *aSinTable.as_mut_ptr().offset((((65536 as
                                                                      libc::c_int
                                                                      /
                                                                      360 as
                                                                          libc::c_int
                                                                      *
                                                                      (*psDroid).direction
                                                                          as
                                                                          libc::c_int)
                                                                     as uint16
                                                                     as
                                                                     libc::c_int
                                                                     >>
                                                                     4 as
                                                                         libc::c_int)
                                                                    +
                                                                    1024 as
                                                                        libc::c_int)
                                                                   as isize)
                                >> 12 as libc::c_int;
                        pos.x = (*psDroid).x as libc::c_int - xBehind;
                        pos.z = (*psDroid).y as libc::c_int - yBehind;
                        pos.y =
                            map_Height(pos.x as UDWORD, pos.z as UDWORD) as
                                int32
                        //						addEffect(&pos,EFFECT_SMOKE,SMOKE_TYPE_TRAIL,FALSE,NULL);
                    }
                }
            }
            psDroid = (*psDroid).psNext
        }
        i = i.wrapping_add(1)
    };
}
// ----------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------
/* Runs all the structure effect stuff - steam puffing out etc */
#[no_mangle]
pub unsafe extern "C" fn effectStructureUpdates() {
    let mut i: UDWORD = 0;
    let mut partition: UDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut eventPos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut capacity: UDWORD = 0;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut active: BOOL = 0;
    /* Go thru' all players */
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psStructure = apsStructLists[i as usize];
        while !psStructure.is_null() {
            /* Find it's group */
            partition =
                (*psStructure).id.wrapping_rem(103 as libc::c_int as
                                                   libc::c_uint);
            /* Is it the right frame? */
            if partition ==
                   frameGetFrameNumber().wrapping_rem(103 as libc::c_int as
                                                          libc::c_uint) {
                /* Is it the right time? */
                if gameTime >
                       lastUpdateStructures[partition as
                                                usize].wrapping_add(1250 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                   {
                    /* Store away the last update time */
                    lastUpdateStructures[partition as usize] = gameTime;
                    // -------------------------------------------------------------------------------
					/* Factories puff out smoke, power stations puff out tesla stuff */
                    if (*(*psStructure).pStructureType).type_0 ==
                           REF_FACTORY as libc::c_int as libc::c_uint ||
                           (*(*psStructure).pStructureType).type_0 ==
                               REF_POWER_GEN as libc::c_int as libc::c_uint {
                        if bMultiPlayer != 0 &&
                               isHumanPlayer((*psStructure).player as UDWORD)
                                   != 0 ||
                               (*psStructure).player as libc::c_int ==
                                   0 as libc::c_int {
                            if (*psStructure).status as libc::c_int ==
                                   SS_BUILT as libc::c_int {
                                if (*psStructure).visible[selectedPlayer as
                                                              usize] != 0 {
                                    /*	We're a factory, so better puff out a bit of steam
							Complete hack with the magic numbers - just for IAN demo
						*/
                                    if (*(*psStructure).pStructureType).type_0
                                           ==
                                           REF_FACTORY as libc::c_int as
                                               libc::c_uint {
                                        if (*(*psStructure).sDisplay.imd).nconnectors
                                               == 1 as libc::c_int {
                                            eventPos.x =
                                                (*psStructure).x as
                                                    libc::c_int +
                                                    (*(*(*psStructure).sDisplay.imd).connectors).x;
                                            eventPos.z =
                                                (*psStructure).y as
                                                    libc::c_int -
                                                    (*(*(*psStructure).sDisplay.imd).connectors).y;
                                            eventPos.y =
                                                (*psStructure).z as
                                                    libc::c_int +
                                                    (*(*(*psStructure).sDisplay.imd).connectors).z;
                                            addEffect(&mut eventPos,
                                                      EFFECT_SMOKE,
                                                      SMOKE_TYPE_STEAM,
                                                      0 as libc::c_int,
                                                      0 as *mut iIMDShape,
                                                      0 as libc::c_int);
                                            if selectedPlayer ==
                                                   (*psStructure).player as
                                                       libc::c_uint {
                                                audio_PlayObjStaticTrack(psStructure
                                                                             as
                                                                             *mut libc::c_void,
                                                                         ID_SOUND_STEAM
                                                                             as
                                                                             libc::c_int);
                                            }
                                        }
                                    } else if (*(*psStructure).pStructureType).type_0
                                                  ==
                                                  REF_POWER_GEN as libc::c_int
                                                      as libc::c_uint {
                                        psPowerGen =
                                            (*psStructure).pFunctionality as
                                                *mut POWER_GEN;
                                        eventPos.x =
                                            (*psStructure).x as int32;
                                        eventPos.z =
                                            (*psStructure).y as int32;
                                        if (*(*psStructure).sDisplay.imd).nconnectors
                                               > 0 as libc::c_int {
                                            eventPos.y =
                                                (*psStructure).z as
                                                    libc::c_int +
                                                    (*(*(*psStructure).sDisplay.imd).connectors).z
                                        } else {
                                            eventPos.y =
                                                (*psStructure).z as int32
                                        }
                                        capacity = (*psPowerGen).capacity;
                                        /*	Work out how many spires it has. This is a particularly unpleasant
								hack and I'm not proud of it, but it needs to done. Honest. AM
							*/
							//if(capacity)
                                        active = 0 as libc::c_int;
                                        i = 0 as libc::c_int as UDWORD;
                                        while i <
                                                  4 as libc::c_int as
                                                      libc::c_uint {
                                            if !(*psPowerGen).apResExtractors[i
                                                                                  as
                                                                                  usize].is_null()
                                                   &&
                                                   (*((*(*psPowerGen).apResExtractors[i
                                                                                          as
                                                                                          usize]).pFunctionality
                                                          as
                                                          *mut RES_EXTRACTOR)).active
                                                       != 0 {
                                                active = 1 as libc::c_int;
                                                break ;
                                            } else { i = i.wrapping_add(1) }
                                        }
                                        eventPos.y =
                                            (*psStructure).z as libc::c_int +
                                                48 as libc::c_int;
                                        addEffect(&mut eventPos,
                                                  EFFECT_EXPLOSION,
                                                  EXPLOSION_TYPE_TESLA,
                                                  0 as libc::c_int,
                                                  0 as *mut iIMDShape,
                                                  0 as libc::c_int);
                                        if selectedPlayer ==
                                               (*psStructure).player as
                                                   libc::c_uint {
                                            audio_PlayObjStaticTrack(psStructure
                                                                         as
                                                                         *mut libc::c_void,
                                                                     ID_SOUND_POWER_SPARK
                                                                         as
                                                                         libc::c_int);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            psStructure = (*psStructure).psNext
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn getFreeEffect() -> UDWORD { return freeEffect; }
/*if(capacity)
							{
								eventPos.y = psStructure->z + 48;
							}*/
							/* Add an effect over the central spire - if
							connected to Res Extractor and it is active*/
							//look through the list to see if any connected Res Extr
/*
							if (((POWER_GEN*)psStructure->pFunctionality)->
								apResExtractors[0] AND ((RES_EXTRACTOR *)((POWER_GEN*)
								psStructure->pFunctionality)->apResExtractors[0]->
								pFunctionality)->active)
							*/
							//if (active)
// ----------------------------------------------------------------------------------------
/* PROTOTYPES */
/* externals */
/* Don't even ask what this fellow does... */
// ----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn effectResetUpdates() {
    let mut i: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 101 as libc::c_int as libc::c_uint {
        lastUpdateDroids[i as usize] = 0 as libc::c_int as UDWORD;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while i < 103 as libc::c_int as libc::c_uint {
        lastUpdateStructures[i as usize] = 0 as libc::c_int as UDWORD;
        i = i.wrapping_add(1)
    };
}
// -----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn fireOnLocation(mut x: UDWORD, mut y: UDWORD)
 -> BOOL {
    let mut i: UDWORD = 0;
    let mut posX: UDWORD = 0;
    let mut posY: UDWORD = 0;
    let mut bOnFire: BOOL = 0;
    i = 0 as libc::c_int as UDWORD;
    bOnFire = 0 as libc::c_int;
    while i < 2500 as libc::c_int as libc::c_uint && bOnFire == 0 {
        if asEffectsList[i as usize].status as libc::c_int ==
               ES_ACTIVE as libc::c_int &&
               asEffectsList[i as usize].group as libc::c_int ==
                   EFFECT_FIRE as libc::c_int {
            posX = asEffectsList[i as usize].position.x as SDWORD as UDWORD;
            posY = asEffectsList[i as usize].position.z as SDWORD as UDWORD;
            if posX == x && posY == y { bOnFire = 1 as libc::c_int }
        }
        i = i.wrapping_add(1)
    }
    return bOnFire;
}
// -----------------------------------------------------------------------------------
/* This will save out the effects data */
#[no_mangle]
pub unsafe extern "C" fn writeFXData(mut pFileName: *mut STRING) -> BOOL {
    let mut pFileData: *mut libc::c_char =
        0 as *mut libc::c_char; // Pointer to the necessary allocated memory
    let mut pFXData: *mut EFFECT =
        0 as *mut EFFECT; // How many bytes we need - depends on compression
    let mut fileSize: UDWORD = 0; // Pointer to the header part of the file
    let mut psHeader: *mut FX_SAVEHEADER =
        0 as *mut FX_SAVEHEADER; // Effectively, how many tiles are there?
    let mut fxEntries: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut imdHashedNumber: UDWORD = 0;
    let mut psOrig: *mut iIMDShape = 0 as *mut iIMDShape;
    /* How many FX do we write out data from? Only write active ones! */
    i = 0 as libc::c_int as UDWORD;
    fxEntries = 0 as libc::c_int as UDWORD;
    while i < 2500 as libc::c_int as libc::c_uint {
        if asEffectsList[i as usize].status as libc::c_int ==
               ES_ACTIVE as libc::c_int {
            fxEntries = fxEntries.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    /* Calculate memory required */
    fileSize =
        (::std::mem::size_of::<_fx_save_header>() as
             libc::c_ulong).wrapping_add(fxEntries.wrapping_mul(::std::mem::size_of::<_effect_def>()
                                                                    as
                                                                    libc::c_ulong));
    /* Try and allocate it - freed up in same function */
    pFileData = memMallocRelease(fileSize) as *mut libc::c_char;
    /* Did we get it? */
    if pFileData.is_null() {
        /* Nope, so do one */
        debug(LOG_ERROR,
              b"Saving FX data : Cannot get the memory! (%d)\x00" as *const u8
                  as *const libc::c_char, fileSize);
        abort();
    }
    /* We got the memory, so put the file header on the file */
    psHeader = pFileData as *mut FX_SAVEHEADER;
    (*psHeader).aFileType[0 as libc::c_int as usize] = 'f' as i32 as STRING;
    (*psHeader).aFileType[1 as libc::c_int as usize] = 'x' as i32 as STRING;
    (*psHeader).aFileType[2 as libc::c_int as usize] = 'd' as i32 as STRING;
    (*psHeader).aFileType[3 as libc::c_int as usize] = 'a' as i32 as STRING;
    (*psHeader).entries = fxEntries;
    /* Write out the version number - unlikely to change for FX data */
    (*psHeader).version = 33 as libc::c_int as UDWORD;
    /* Skip past the header to the raw data area */
    pFXData =
        pFileData.offset(::std::mem::size_of::<_fx_save_header>() as
                             libc::c_ulong as isize) as *mut EFFECT;
    i = 0 as libc::c_int as UDWORD;
    while i < 2500 as libc::c_int as libc::c_uint {
        if asEffectsList[i as usize].status as libc::c_int ==
               ES_ACTIVE as libc::c_int {
            /*
			restore = FALSE;
			// Can't save out the pointer, so hash it first
			if(asEffectsList[i].imd)
			{
				restore = TRUE;
				psTemp = asEffectsList[i].imd;
				psOrig =asEffectsList[i].imd;
				asEffectsList[i].imd = (iIMDShape*)resGetHashfromData("IMD",psOrig,&imdHashedNumber);
			}
			memcpy(pFXData,&asEffectsList[i],sizeof(struct _effect_def));
			if(restore)
			{
				asEffectsList[i].imd = psTemp;
			}
			*/
            memcpy(pFXData as *mut libc::c_void,
                   &mut *asEffectsList.as_mut_ptr().offset(i as isize) as
                       *mut EFFECT as *const libc::c_void,
                   ::std::mem::size_of::<_effect_def>() as libc::c_ulong);
            /* Is there an imd? */
            if !asEffectsList[i as usize].imd.is_null() {
                psOrig = asEffectsList[i as usize].imd;
                resGetHashfromData(b"IMD\x00" as *const u8 as
                                       *const libc::c_char as *mut STRING,
                                   psOrig as *mut libc::c_void,
                                   &mut imdHashedNumber);
                (*pFXData).imd = imdHashedNumber as *mut iIMDShape
            }
            pFXData = pFXData.offset(1)
        }
        i = i.wrapping_add(1)
    }
    /* Have a bash at opening the file to write */
    if saveFile(pFileName, pFileData, fileSize) == 0 {
        debug(LOG_ERROR,
              b"Saving FX data: couldn\'t open file %s\x00" as *const u8 as
                  *const libc::c_char, pFileName);
        return 0 as libc::c_int
    }
    /* And free up the memory we used */
    if !pFileData.is_null() {
        memFreeRelease(pFileData as *mut libc::c_void);
        pFileData = 0 as *mut libc::c_char
    }
    /* Everything is just fine! */
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------
/* This will read in the effects data */
#[no_mangle]
pub unsafe extern "C" fn readFXData(mut pFileData: *mut libc::c_char,
                                    mut fileSize: UDWORD) -> BOOL {
    let mut expectedFileSize: UDWORD = 0;
    let mut psHeader: *mut FX_SAVEHEADER = 0 as *mut FX_SAVEHEADER;
    let mut i: UDWORD = 0;
    let mut pFXData: *mut EFFECT = 0 as *mut EFFECT;
    /* See if we've been given the right file type? */
    psHeader = pFileData as *mut FX_SAVEHEADER;
    if (*psHeader).aFileType[0 as libc::c_int as usize] as libc::c_int !=
           'f' as i32 ||
           (*psHeader).aFileType[1 as libc::c_int as usize] as libc::c_int !=
               'x' as i32 ||
           (*psHeader).aFileType[2 as libc::c_int as usize] as libc::c_int !=
               'd' as i32 ||
           (*psHeader).aFileType[3 as libc::c_int as usize] as libc::c_int !=
               'a' as i32 {
        debug(LOG_ERROR,
              b"Read FX data: Weird file type found? Has header letters - %c %c %c %c\x00"
                  as *const u8 as *const libc::c_char,
              (*psHeader).aFileType[0 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[1 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[2 as libc::c_int as usize] as libc::c_int,
              (*psHeader).aFileType[3 as libc::c_int as usize] as
                  libc::c_int);
        return 0 as libc::c_int
    }
    /* How much data are we expecting? */
    expectedFileSize =
        (::std::mem::size_of::<_fx_save_header>() as
             libc::c_ulong).wrapping_add((*psHeader).entries.wrapping_mul(::std::mem::size_of::<_effect_def>()
                                                                              as
                                                                              libc::c_ulong));
    /* Is that what we've been given? */
    if fileSize != expectedFileSize {
        /* No, so bomb out */
        debug(LOG_ERROR,
              b"Read FX data : Weird file size!\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    /* Skip past the header gubbins - can check version number here too */
    pFXData =
        pFileData.offset(::std::mem::size_of::<_fx_save_header>() as
                             libc::c_ulong as isize) as *mut EFFECT;
    /* Clear out anything that's there already! */
    initEffectsSystem();
    /* For every FX... */
    i = 0 as libc::c_int as UDWORD;
    while i < (*psHeader).entries {
        let fresh1 = pFXData;
        pFXData = pFXData.offset(1);
        memcpy(&mut *asEffectsList.as_mut_ptr().offset(i as isize) as
                   *mut EFFECT as *mut libc::c_void,
               fresh1 as *const libc::c_void,
               ::std::mem::size_of::<_effect_def>() as libc::c_ulong);
        if !asEffectsList[i as usize].imd.is_null() {
            /* Restore the pointer from the hashed ID */
            asEffectsList[i as usize].imd =
                resGetDataFromHash(b"IMD\x00" as *const u8 as
                                       *const libc::c_char as *mut STRING,
                                   asEffectsList[i as usize].imd as UDWORD) as
                    *mut iIMDShape
        }
        i = i.wrapping_add(1)
    }
    /* Ensure free effects kept up to date */
    freeEffect = i;
    /* Hopefully everything's just fine by now */
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn addFireworksEffect() { }
// ----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------
