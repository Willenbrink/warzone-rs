use ::libc;
extern "C" {
    pub type _formation;
    /* This returns true if the mouse key is currently depressed */
    #[no_mangle]
    fn mouseDown(code: MOUSE_KEY_CODE) -> BOOL;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
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
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
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
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    //Value is stored for easy access to this structure stat
    #[no_mangle]
    static mut factoryModuleStat: UDWORD;
    #[no_mangle]
    static mut powerModuleStat: UDWORD;
    #[no_mangle]
    static mut researchModuleStat: UDWORD;
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    //builds a specified structure at a given location
    #[no_mangle]
    fn buildStructure(pStructureType: *mut STRUCTURE_STATS, x: UDWORD,
                      y: UDWORD, player: UDWORD, FromSave: BOOL)
     -> *mut STRUCTURE;
    // remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
    /*this is called whenever a structure has finished building*/
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
    #[no_mangle]
    fn moveDroidTo(psDroid: *mut DROID, x: UDWORD, y: UDWORD) -> BOOL;
    /* Stop a droid */
    #[no_mangle]
    fn moveStopDroid(psDroid: *mut DROID);
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn DrawnInLastFrame(Frame: SDWORD) -> BOOL;
    //extern BOOL	forceWidgetsOn;
    #[no_mangle]
    fn mouseTarget() -> *mut BASE_OBJECT;
    /* Give a droid an order with a location target */
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn getTileStructure(x: UDWORD, y: UDWORD) -> *mut STRUCTURE;
    // move an object within the grid
// oldX,oldY are the old position of the object in world coords
    #[no_mangle]
    fn gridMoveObject(psObj: *mut BASE_OBJECT, oldX: SDWORD, oldY: SDWORD);
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    //choose one. 
    #[no_mangle]
    fn NETsend(msg: *mut NETMSG, player: DPID, guarantee: BOOL) -> BOOL;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    // leave the game in play.
    #[no_mangle]
    fn NETgetBytesSent() -> UDWORD;
    // return bytes sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetPacketsSent() -> UDWORD;
    // return packets sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetBytesRecvd() -> UDWORD;
    // return bytes sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetPacketsRecvd() -> UDWORD;
    // return packets sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetRecentBytesSent() -> UDWORD;
    // more immediate functions.
    #[no_mangle]
    fn NETgetRecentPacketsSent() -> UDWORD;
    #[no_mangle]
    fn NETgetRecentBytesRecvd() -> UDWORD;
    // return one of the four flags(dword) about the game.
    #[no_mangle]
    fn NETsetGameFlags(flag: UDWORD, value: DWORD) -> BOOL;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // true when more than 1 player.
    #[no_mangle]
    static mut player2dpid: [DWORD; 8];
    // if ping is bigger than this, then worry and panic.
    // how many points to allocate for res levels???
    // ////////////////////////////////////////////////////////////////////////////
// macros For handling net messages, just copy things in & out of the msg buffer
/*
#define NetAdd(m,pos,thing) \
	memcpy(&(m.body[pos]),&(thing),sizeof(thing)) 

#define NetAdd2(m,pos,thing) \
	memcpy( &((*m).body[pos]), &(thing), sizeof(thing)) 

#define NetAddSt(m,pos,stri) \
	strcpy(&(m.body[pos]),stri)

#define NetGet(m,pos,thing) \
	memcpy(&(thing),&(m->body[pos]),sizeof(thing))

#define NetGetSt(m,pos,stri) \
	strcpy(stri,&(m->body[pos]))
*/
// ////////////////////////////////////////////////////////////////////////////
// functions
    #[no_mangle]
    fn IdToPointer(id: UDWORD, player: UDWORD) -> *mut BASE_OBJECT;
    #[no_mangle]
    fn IdToStruct(id: UDWORD, player: UDWORD) -> *mut STRUCTURE;
    #[no_mangle]
    fn IdToDroid(id: UDWORD, player: UDWORD, psDroid: *mut *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    // This dos'nt compile on the PSX.
//typedef enum _titlemode tMode;	// define the type
    #[no_mangle]
    static mut titleMode: tMode;
    // form disk
    #[no_mangle]
    fn getMultiStats(player: UDWORD, bLocal: BOOL) -> PLAYERSTATS;
    // get from net
    #[no_mangle]
    fn setMultiStats(playerDPID: DWORD, plStats: PLAYERSTATS, bLocal: BOOL)
     -> BOOL;
    // used in multiplayer to force power levels.
    #[no_mangle]
    fn setPower(player: UDWORD, avail: UDWORD);
    /*resets the power levels for all players when power is turned back on*/
    #[no_mangle]
    fn powerCalc(on: BOOL);
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
pub type DPID = libc::c_int;
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
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
pub type PSBSPTREENODE = *mut BSPTREENODE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
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
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
// currently uses 4k per structure (!)
//*************************************************************************
//
// texture animation structures
//
//*************************************************************************
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
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
pub type VERTEXID = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _base_stats {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
}
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
pub type BASE_STATS = _base_stats;
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
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
//WSC_ARTILLARY,
//WSC_CLOSECOMBAT,
// used to define which projectile model to use for the weapon
pub type MOVEMENT_MODEL = _movement_model;
pub type _weapon_effect = libc::c_uint;
pub const WE_NUMEFFECTS: _weapon_effect = 6;
pub const WE_ANTI_AIRCRAFT: _weapon_effect = 5;
pub const WE_FLAMER: _weapon_effect = 4;
pub const WE_ARTILLERY_ROUND: _weapon_effect = 3;
pub const WE_BUNKER_BUSTER: _weapon_effect = 2;
pub const WE_ANTI_TANK: _weapon_effect = 1;
pub const WE_ANTI_PERSONNEL: _weapon_effect = 0;
//used to modify the damage to a propuslion type (or structure) based on weapon
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
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
/* Common stats */
// Max speed for the droid
// Type of propulsion used - index 
// into PropulsionTable
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
pub struct _power_gen {
    pub power: UDWORD,
    pub multiplier: UDWORD,
    pub capacity: UDWORD,
    pub apResExtractors: [*mut _structure; 4],
}
pub type POWER_GEN = _power_gen;
pub type STRUCTURE = _structure;
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
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*pointers to the res ext
																associated with this gen*/
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SESSIONDESC {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub host: [libc::c_char; 16],
    pub dwMaxPlayers: DWORD,
    pub dwCurrentPlayers: DWORD,
    pub dwUser1: DWORD,
    pub dwUser2: DWORD,
    pub dwUser3: DWORD,
    pub dwUser4: DWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYER {
    pub dpid: DPID,
    pub name: [libc::c_char; 64],
    pub bHost: BOOL,
    pub bSpectator: BOOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETPLAY {
    pub games: [GAMESTRUCT; 12],
    pub players: [PLAYER; 8],
    pub playercount: UDWORD,
    pub dpidPlayer: DPID,
    pub bComms: BOOL,
    pub bHost: BOOL,
    pub bLobbyLaunched: BOOL,
    pub bSpectator: BOOL,
    pub bEncryptAllPackets: BOOL,
    pub cryptKey: [UDWORD; 4],
    pub bCaptureInUse: BOOL,
    pub bAllowCaptureRecord: BOOL,
    pub bAllowCapturePlay: BOOL,
}
pub type _msgtype = libc::c_uint;
pub const NET_SET_TEAMS: _msgtype = 50;
pub const NET_BEACONMSG: _msgtype = 49;
pub const NET_TEAMS_ON: _msgtype = 48;
pub const NET_AITEXTMSG: _msgtype = 47;
pub const NET_REQUESTMAP: _msgtype = 46;
pub const NET_LASSAT: _msgtype = 45;
pub const NET_RESEARCHSTATUS: _msgtype = 44;
pub const NET_DROIDDISEMBARK: _msgtype = 43;
pub const NET_DROIDEMBARK: _msgtype = 42;
pub const NET_SECONDARY_ALL: _msgtype = 41;
pub const NET_WHITEBOARD: _msgtype = 40;
pub const NET_VTOLREARM: _msgtype = 39;
pub const NET_VTOL: _msgtype = 38;
pub const NET_DESTROYXTRA: _msgtype = 37;
pub const NET_SCORESUBMIT: _msgtype = 36;
pub const NET_DMATCHWIN: _msgtype = 35;
pub const NET_ARTIFACTS: _msgtype = 34;
pub const NET_COLOURREQUEST: _msgtype = 33;
pub const NET_DEMOLISH: _msgtype = 32;
pub const NET_GIFT: _msgtype = 31;
pub const NET_ALLIANCE: _msgtype = 30;
pub const NET_FIREUP: _msgtype = 29;
pub const NET_SECONDARY: _msgtype = 28;
pub const NET_KICK: _msgtype = 27;
pub const NET_OPTIONS: _msgtype = 26;
pub const NET_PLAYERRESPONDING: _msgtype = 25;
pub const NET_FEATURES: _msgtype = 24;
pub const NET_WHOLEDROID: _msgtype = 23;
pub const NET_STRUCT: _msgtype = 22;
pub const NET_REQUESTPLAYER: _msgtype = 21;
pub const NET_PLAYERCOMPLETE: _msgtype = 20;
pub const NET_REQUESTDROID: _msgtype = 19;
pub const NET_LEAVING: _msgtype = 18;
pub const NET_TEXTMSG: _msgtype = 17;
pub const NET_RESEARCH: _msgtype = 16;
pub const NET_BUILDFINISHED: _msgtype = 15;
pub const NET_STRUCTDEST: _msgtype = 14;
pub const NET_BUILD: _msgtype = 13;
pub const NET_VERSION: _msgtype = 12;
pub const NET_CHECK_POWER: _msgtype = 11;
pub const NET_CHECK_STRUCT: _msgtype = 10;
pub const NET_CHECK_DROID: _msgtype = 9;
pub const NET_PING: _msgtype = 8;
pub const NET_FEATUREDEST: _msgtype = 7;
pub const NET_TEMPLATEDEST: _msgtype = 6;
pub const NET_TEMPLATE: _msgtype = 5;
pub const NET_GROUPORDER: _msgtype = 4;
pub const NET_DROIDMOVE: _msgtype = 3;
pub const NET_DROIDDEST: _msgtype = 2;
pub const NET_DROIDINFO: _msgtype = 1;
pub const NET_DROID: _msgtype = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MULTIPLAYERINGAME {
    pub PingTimes: [UDWORD; 8],
    pub localOptionsReceived: BOOL,
    pub localJoiningInProgress: BOOL,
    pub JoiningInProgress: [BOOL; 8],
    pub bHostSetup: BOOL,
    pub startTime: UDWORD,
    pub modem: UDWORD,
    pub numStructureLimits: UDWORD,
    pub pStructureLimits: *mut UBYTE,
    pub skScores: [[UDWORD; 2]; 8],
    pub phrases: [[CHAR; 255]; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PLAYERSTATS {
    pub played: DWORD,
    pub wins: DWORD,
    pub loses: DWORD,
    pub totalKills: DWORD,
    pub totalScore: SDWORD,
    pub recentKills: DWORD,
    pub recentScore: SDWORD,
    pub killsToAdd: DWORD,
    pub scoreToAdd: SDWORD,
}
pub const MULTILIMIT: _title_mode = 11;
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
pub const GAMEFIND: _title_mode = 10;
pub const FORCESELECT: _title_mode = 9;
pub const MULTIOPTION: _title_mode = 8;
pub const PROTOCOL: _title_mode = 7;
pub const CREDITS: _title_mode = 6;
pub const TUTORIAL: _title_mode = 5;
pub const GAME: _title_mode = 4;
pub const OPTIONS: _title_mode = 3;
pub const MULTI: _title_mode = 2;
pub const SINGLE: _title_mode = 1;
pub const TITLE: _title_mode = 0;
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
 * Power.h
 *
 * Definitions for the Power Functionality.
 *
 */
// free power on collection of oildrum.
//% used to determine the power cost of repairing a droid
                                         //definately DON'T WANT the brackets round 1/2 - it will equate to zero!
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
// how often to update global score.
// maximum time before doing a dirty fix.
static mut PingSend: [UDWORD; 8] = [0; 8];
//stores the time the ping was called.
// ////////////////////////////////////////////////////////////////////////////
// test traffic level.
unsafe extern "C" fn okToSend() -> BOOL {
    //update checks	& go no further if any exceeded.
    if NETgetRecentBytesSent().wrapping_add(NETgetRecentBytesRecvd()) >=
           game.bytesPerSec as libc::c_uint {
        return 0 as libc::c_int
    }
    if NETgetRecentPacketsSent() >= game.packetsPerSec as libc::c_uint {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*
 * MultiSync.c
 *
 * synching issues
 * This file handles the constant backstream of net info, checking that recvd info
 * is concurrent with the local world, and correcting as required. Magic happens here.
 *
 * All conflicts due to non-guaranteed messaging are detected/resolved here.
 *
 * Alex Lee, pumpkin Studios, bath.
 */
// ////////////////////////////////////////////////////////////////////////////
// function definitions
// ////////////////////////////////////////////////////////////////////////////
// Droid checking info. keep position and damage in sync.
#[no_mangle]
pub unsafe extern "C" fn sendCheck() -> BOOL {
    let mut i: UDWORD = 0; // update stats.
    NETgetBytesSent();
    NETgetBytesRecvd();
    NETgetPacketsSent();
    NETgetPacketsRecvd();
    // dont send checks till all players are present.
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 && ingame.JoiningInProgress[i as usize] != 0
           {
            return 1 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    // send Checks. note each send has it's own send criteria, so might not send anything.
    if okToSend() != 0 { sendPing(); }
    if okToSend() != 0 { sendStructureCheck(); }
    if okToSend() != 0 { sendPowerCheck(0 as libc::c_int); }
    if okToSend() != 0 { sendScoreCheck(); }
    if okToSend() != 0 { sendDroidCheck(); }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// pick a droid to send, NULL otherwise.
unsafe extern "C" fn pickADroid() -> *mut DROID {
    let mut pD: *mut DROID = 0 as *mut DROID; // current droid we're checking
    let mut i: UDWORD = 0; // how far down the playerlist to go.
    static mut droidnum: UDWORD =
        0 as libc::c_int as UDWORD; // current player we're checking
    static mut player: UDWORD = 0 as libc::c_int as UDWORD;
    static mut maxtrys: UDWORD = 0 as libc::c_int as UDWORD;
    if myResponsibility(player) == 0 {
        // dont send stuff that's not our problem.
        player = player.wrapping_add(1); // next player next time.
        player =
            player.wrapping_rem(8 as libc::c_int as
                                    libc::c_uint); // get the droid to send to everyone
        droidnum = 0 as libc::c_int as UDWORD; // CRITERIA FOR PICKADROID
        if maxtrys < 8 as libc::c_int as libc::c_uint {
            maxtrys = maxtrys.wrapping_add(1);
            return pickADroid()
        } else {
            maxtrys = 0 as libc::c_int as UDWORD;
            return 0 as *mut DROID
        }
    }
    pD = apsDroidLists[player as usize];
    i = 0 as libc::c_int as UDWORD;
    while i < droidnum && !pD.is_null() {
        //
        pD = (*pD).psNext;
        i = i.wrapping_add(1)
    }
    if pD.is_null() {
        // droid is no longer there or list end.
        player = player.wrapping_add(1); // next player next time.
        player = player.wrapping_rem(8 as libc::c_int as libc::c_uint);
        droidnum = 0 as libc::c_int as UDWORD;
        if maxtrys < 8 as libc::c_int as libc::c_uint {
            maxtrys = maxtrys.wrapping_add(1);
            return pickADroid()
        } else {
            maxtrys = 0 as libc::c_int as UDWORD;
            return 0 as *mut DROID
        }
    }
    droidnum = droidnum.wrapping_add(1);
    maxtrys = 0 as libc::c_int as UDWORD;
    return pD;
}
// ///////////////////////////////////////////////////////////////////////////
// send a droid info packet.
unsafe extern "C" fn sendDroidCheck() -> BOOL {
    let mut pD: *mut DROID = 0 as *mut DROID; // last time a struct was sent.
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut i: UDWORD = 0 as libc::c_int as UDWORD;
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    static mut lastSent: UDWORD = 0 as libc::c_int as UDWORD;
    let mut toSend: UDWORD = 4 as libc::c_int as UDWORD;
    if lastSent > gameTime { lastSent = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastSent) < 1000 as libc::c_int as libc::c_uint {
        // only send a struct send if not done recently.
        return 1 as libc::c_int
    }
    lastSent = gameTime;
    msg.size = 0 as libc::c_int as libc::c_ushort;
    match game.bytesPerSec as libc::c_int {
        2000 => { toSend = 6 as libc::c_int as UDWORD }
        1201 | 1202 | 1000 | 1200 | _ => {
            toSend = 4 as libc::c_int as UDWORD
        }
    }
    while count < toSend {
        // send x droids.
        pD = pickADroid();
        if !pD.is_null() { packageCheck(i, &mut msg, pD); }
        i = msg.size as UDWORD;
        count = count.wrapping_add(1 as libc::c_int as libc::c_uint)
    }
    msg.type_0 = NET_CHECK_DROID as libc::c_int as libc::c_uchar;
    NETbcast(&mut msg, 0 as libc::c_int);
    return 1 as libc::c_int;
}
//power
// ////////////////////////////////////////////////////////////////////////////
// Send a Single Droid Check message
unsafe extern "C" fn packageCheck(mut i: UDWORD, mut pMsg: *mut NETMSG,
                                  mut pD: *mut DROID) {
    //	UDWORD packtemp;
    let mut numkills: UWORD = 0; //
    (*pMsg).body[i.wrapping_add(0 as libc::c_int as libc::c_uint) as usize] =
        (*pD).player as CHAR; // order being executed
    (*pMsg).body[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
        (*pD).order as CHAR; // droid id
    memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(2 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // damage points
    memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(6 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).secondaryOrder as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // direction
    memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(10 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).body as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); //fraction move pos
    memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(14 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pD).direction as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as
               libc::c_ulong); //non fractional move pos
    if (*pD).order == DORDER_ATTACK as libc::c_int ||
           (*pD).order == DORDER_MOVE as libc::c_int ||
           (*pD).order == DORDER_RTB as libc::c_int ||
           (*pD).order == DORDER_RTR as libc::c_int {
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(16 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).sMove.fx as *mut FRACT as *const libc::c_void,
               ::std::mem::size_of::<FRACT>() as libc::c_ulong);
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(20 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).sMove.fy as *mut FRACT as *const libc::c_void,
               ::std::mem::size_of::<FRACT>() as libc::c_ulong);
    } else {
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(16 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).x as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(20 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).y as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    }
    if (*pD).order == DORDER_ATTACK as libc::c_int {
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(24 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*(*pD).psTarget).id as *mut UDWORD as
                   *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        // target id
    } else if (*pD).order == DORDER_MOVE as libc::c_int {
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(24 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).orderX as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as
                   libc::c_ulong); // droid kills
        memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(26 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pD).orderY as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    }
    numkills = (*pD).numKills;
    memcpy(&mut *(*pMsg).body.as_mut_ptr().offset(i.wrapping_add(28 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut numkills as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    (*pMsg).size = ((*pMsg).size as libc::c_int + 30 as libc::c_int) as UWORD;
}
// send/recv  check info
// ////////////////////////////////////////////////////////////////////////////
// receive a check and update the local world state accordingly
#[no_mangle]
pub unsafe extern "C" fn recvDroidCheck(mut m: *mut NETMSG) -> BOOL {
    let mut fx: FRACT = 0 as libc::c_int as FRACT; //,dir;
    let mut fy: FRACT = 0 as libc::c_int as FRACT;
    let mut ref_0: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut x: UDWORD = 0 as libc::c_int as UDWORD;
    let mut y: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bod: UDWORD = 0;
    let mut target: UDWORD = 0 as libc::c_int as UDWORD;
    let mut dir: UWORD = 0;
    let mut numkills: UWORD = 0;
    let mut ord: DROID_ORDER = DORDER_NONE;
    let mut onscreen: BOOL = 0;
    let mut pD: *mut DROID = 0 as *mut DROID;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut i: UDWORD = 0;
    let mut state: UDWORD = 0;
    let mut tx: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ty: UDWORD = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < (*m).size as libc::c_uint {
        // obtain information about remote droid.
        player =
            (*m).body[i.wrapping_add(0 as libc::c_int as libc::c_uint) as
                          usize] as UDWORD;
        // done this droid, move on to next.
        ord =
            (*m).body[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                          usize] as DROID_ORDER; // droid id.
        memcpy(&mut ref_0 as *mut UDWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as
                   libc::c_ulong); // Damage update.
        memcpy(&mut state as *mut UDWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(6 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut bod as *mut UDWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut dir as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(14 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        if ord as libc::c_uint == DORDER_ATTACK as libc::c_int as libc::c_uint
               ||
               ord as libc::c_uint ==
                   DORDER_MOVE as libc::c_int as libc::c_uint ||
               ord as libc::c_uint ==
                   DORDER_RTB as libc::c_int as libc::c_uint ||
               ord as libc::c_uint ==
                   DORDER_RTR as libc::c_int as libc::c_uint {
            // detailed position info mode
            memcpy(&mut fx as *mut FRACT as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(16 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<FRACT>() as
                       libc::c_ulong); // dont pack since could be sending fractional vals anyway.
            memcpy(&mut fy as *mut FRACT as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(20 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<FRACT>() as libc::c_ulong);
        } else {
            memcpy(&mut x as *mut UDWORD as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(16 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            memcpy(&mut y as *mut UDWORD as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(20 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        }
        if ord as libc::c_uint == DORDER_ATTACK as libc::c_int as libc::c_uint
           {
            memcpy(&mut target as *mut UDWORD as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(24 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        } else if ord as libc::c_uint ==
                      DORDER_MOVE as libc::c_int as libc::c_uint {
            memcpy(&mut tx as *mut UDWORD as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(24 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            memcpy(&mut ty as *mut UDWORD as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(26 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                           as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        }
        memcpy(&mut numkills as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(i.wrapping_add(28 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                       as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        i = i.wrapping_add(30 as libc::c_int as libc::c_uint);
        if IdToDroid(ref_0, player, &mut pD) == 0 {
            // ////////////////////////////////////
            // find the droid in question
            NETlogEntry(b"Recvd Unknown droid info. val=player\x00" as
                            *const u8 as *const libc::c_char as *mut CHAR,
                        0 as libc::c_int as UDWORD, player);
            debug(LOG_NEVER,
                  b"Received Checking Info for an unknown (As yet) droid player:%d ref:%d\n\x00"
                      as *const u8 as *const libc::c_char, player, ref_0);
            return 1 as libc::c_int
            //Recvd checking info for an unknown droid
        }
        if target != 0 {
            psTarget = IdToPointer(target, 99 as libc::c_int as UDWORD)
            // get the target in question.
        }
        if DrawnInLastFrame((*pD).sDisplay.frameNumber as SDWORD) != 0 &&
               (*pD).sDisplay.screenX < pie_GetVideoBufferWidth() &&
               (*pD).sDisplay.screenY < pie_GetVideoBufferHeight() {
            // ////////////////////////////////////
		// decide how to sync it.
            // check for onscreen
            if (*pD).visible[selectedPlayer as usize] != 0 {
                onscreen = 1 as libc::c_int
                // onscreen and visible
            } else {
                onscreen = 0 as libc::c_int
                // onscreen, but not visible.
            }
        } else {
            onscreen = 0 as libc::c_int
            // not onscreen.
        }
        if ingame.PingTimes[player as usize] >
               2000 as libc::c_int as libc::c_uint {
            // if it's a big ping then don't do a smooth move.
            onscreen = 0 as libc::c_int
        }
        if onscreen != 0 || vtolDroid(pD) != 0 {
            onscreenUpdate(pD, bod, x, y, fx, fy, dir, ord);
        } else { offscreenUpdate(pD, bod, x, y, fx, fy, dir, ord); }
        if abs(x.wrapping_sub((*pD).x as libc::c_uint) as libc::c_int) <
               128 as libc::c_int * 2 as libc::c_int ||
               abs(y.wrapping_sub((*pD).y as libc::c_uint) as libc::c_int) <
                   128 as libc::c_int * 2 as libc::c_int {
            (*pD).lastSync = gameTime
            //		if( pD->lastSync > gameTime)pD->lastSync =0;
//		if(  (gameTime - pD->lastSync) > SYNC_PANIC )	// if it's been a while then jump it.
//		{
//			onscreen = FALSE;
//		}
            // ////////////////////////////////////
		// / now do the update.
            // ////////////////////////////////////
		// now make note of how accurate the world model is for this droid.	// if droid is close then remember.
            // note we did a reasonable job.
        }
        if vtolDroid(pD) == 0 {
            highLevelDroidUpdate(pD, x, y, state, ord as UDWORD, psTarget,
                                 numkills as UDWORD);
        }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////
		// now update the higher level stuff
// send/recv Ping information
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// higher order droid updating. Works mainly at the order level. comes after the main sync.
unsafe extern "C" fn highLevelDroidUpdate(mut psDroid: *mut DROID,
                                          mut x: UDWORD, mut y: UDWORD,
                                          mut state: UDWORD,
                                          mut order: UDWORD,
                                          mut psTarget: *mut BASE_OBJECT,
                                          mut numKills: UDWORD) {
    // update kill rating.
    (*psDroid).numKills = numKills as UWORD;
    // remote droid is attacking, not here tho!
    if order == DORDER_ATTACK as libc::c_int as libc::c_uint &&
           (*psDroid).order != DORDER_ATTACK as libc::c_int &&
           !psTarget.is_null() {
        turnOffMultiMsg(1 as libc::c_int);
        orderDroidObj(psDroid, DORDER_ATTACK, psTarget);
        turnOffMultiMsg(0 as libc::c_int);
    }
    // secondary orders.
    if (*psDroid).secondaryOrder != state {
        (*psDroid).secondaryOrder = state
    }
    // see how well the sync worked, optionally update.
	// offscreen updates will make this ok each time.
    if (*psDroid).order == DORDER_NONE as libc::c_int &&
           order == DORDER_NONE as libc::c_int as libc::c_uint {
        if abs(x.wrapping_sub((*psDroid).x as libc::c_uint) as libc::c_int) >
               128 as libc::c_int * 2 as libc::c_int ||
               abs(y.wrapping_sub((*psDroid).y as libc::c_uint) as
                       libc::c_int) > 128 as libc::c_int * 2 as libc::c_int {
            turnOffMultiMsg(1 as libc::c_int);
            orderDroidLoc(psDroid, DORDER_MOVE, x, y);
            turnOffMultiMsg(0 as libc::c_int);
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// droid on screen needs modifying
unsafe extern "C" fn onscreenUpdate(mut psDroid: *mut DROID, mut dam: UDWORD,
                                    mut x: UDWORD, mut y: UDWORD,
                                    mut fx: FRACT, mut fy: FRACT,
                                    mut dir: UWORD, mut order: DROID_ORDER) {
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bMouseOver: BOOL = 0 as libc::c_int;
    psClickedOn = mouseTarget();
    if !psClickedOn.is_null() &&
           (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
        if (*psClickedOn).id == (*psDroid).id && mouseDown(MOUSE_RMB) != 0 {
            bMouseOver = 1 as libc::c_int
            // override, so you dont see the updates.
        }
    }
    if bMouseOver == 0 {
        (*psDroid).body = dam
        // update damage
    };
}
// what it should be doing
// ////////////////////////////////////////////////////////////////////////////
// droid offscreen needs modyfying.
unsafe extern "C" fn offscreenUpdate(mut psDroid: *mut DROID, mut dam: UDWORD,
                                     mut x: UDWORD, mut y: UDWORD,
                                     mut fx: FRACT, mut fy: FRACT,
                                     mut dir: UWORD, mut order: DROID_ORDER) {
    let mut oldx: UDWORD = 0;
    let mut oldy: UDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    // stage one, update the droids position & info, LOW LEVEL STUFF.
    if order as libc::c_uint == DORDER_ATTACK as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint == DORDER_MOVE as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint == DORDER_RTB as libc::c_int as libc::c_uint
           ||
           order as libc::c_uint == DORDER_RTR as libc::c_int as libc::c_uint
       {
        // move order
        // calculate difference between remote and local
        xdiff =
            (*psDroid).x as libc::c_int -
                fx as UWORD as libc::c_int; //update x
        ydiff =
            (*psDroid).y as libc::c_int -
                fy as UWORD as libc::c_int; //update y
        distSq = xdiff * xdiff + ydiff * ydiff; //update move progress
        if distSq >
               2 as libc::c_int * 128 as libc::c_int *
                   (2 as libc::c_int * 128 as libc::c_int) {
            if fx as UDWORD != 0 as libc::c_int as libc::c_uint &&
                   fy as UDWORD != 0 as libc::c_int as libc::c_uint {
                oldx = (*psDroid).x as UDWORD; // update rotation
                oldy = (*psDroid).y as UDWORD;
                (*psDroid).sMove.fx = fx;
                (*psDroid).sMove.fy = fy;
                (*psDroid).x = fx as UWORD;
                (*psDroid).y = fy as UWORD;
                gridMoveObject(psDroid as *mut BASE_OBJECT, oldx as SDWORD,
                               oldy as SDWORD);
                (*psDroid).direction =
                    (dir as libc::c_int % 360 as libc::c_int) as UWORD;
                // reroute the droid.
                turnOffMultiMsg(1 as libc::c_int);
                moveDroidTo(psDroid, (*psDroid).sMove.DestinationX as UDWORD,
                            (*psDroid).sMove.DestinationY as UDWORD);
                turnOffMultiMsg(0 as libc::c_int);
            }
        }
    } else {
        oldx = (*psDroid).x as UDWORD;
        oldy = (*psDroid).y as UDWORD;
        // if more than  2 squares, jump it.
        // update rotation
        (*psDroid).x = x as UWORD; //update x
        (*psDroid).y = y as UWORD; //update y
        gridMoveObject(psDroid as *mut BASE_OBJECT, oldx as SDWORD,
                       oldy as SDWORD); // update damage
        (*psDroid).direction =
            (dir as libc::c_int % 360 as libc::c_int) as UWORD
    }
    (*psDroid).body = dam;
    // stop droid if remote droid has stopped.
    if (order as libc::c_uint == DORDER_NONE as libc::c_int as libc::c_uint ||
            order as libc::c_uint ==
                DORDER_GUARD as libc::c_int as libc::c_uint) &&
           !((*psDroid).order == DORDER_NONE as libc::c_int ||
                 (*psDroid).order == DORDER_GUARD as libc::c_int) {
        turnOffMultiMsg(1 as libc::c_int);
        moveStopDroid(psDroid);
        turnOffMultiMsg(0 as libc::c_int);
    }
    // snap droid(if on ground)  to terrain level at x,y.
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"offscreenUpdate: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"multisync.c\x00" as *const u8 as *const libc::c_char,
              588 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"offscreenUpdate\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psPropStats).propulsionType as libc::c_int != LIFT as libc::c_int {
        // if not airborne.
        (*psDroid).z =
            map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                UWORD
    };
}
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Structure Checking, to ensure smoke and stuff is consistent across machines.
// this func is recursive!
unsafe extern "C" fn pickAStructure() -> *mut STRUCTURE {
    static mut player: UDWORD =
        0 as libc::c_int as UDWORD; // player currently checking.
    static mut snum: UDWORD =
        0 as libc::c_int as UDWORD; // structure index for this player.
    let mut pS: *mut STRUCTURE =
        0 as *mut STRUCTURE; // don't loop forever if failing/.
    static mut maxtrys: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    if myResponsibility(player) == 0 {
        // dont send stuff that's not our problem.
        player = player.wrapping_add(1); // next player next time.
        player =
            player.wrapping_rem(8 as libc::c_int as
                                    libc::c_uint); // find the strucutre
        snum = 0 as libc::c_int as UDWORD;
        if maxtrys < 8 as libc::c_int as libc::c_uint {
            maxtrys = maxtrys.wrapping_add(1);
            return pickAStructure()
        } else {
            maxtrys = 0 as libc::c_int as UDWORD;
            return 0 as *mut STRUCTURE
        }
    }
    pS = apsStructLists[player as usize];
    i = 0 as libc::c_int as UDWORD;
    while i < snum && !pS.is_null() {
        pS = (*pS).psNext;
        i = i.wrapping_add(1)
    }
    if pS.is_null() {
        // last structure or no structures at all
        player = player.wrapping_add(1); // go onto the next player
        player =
            player.wrapping_rem(8 as libc::c_int as
                                    libc::c_uint); // next structure next time
        snum = 0 as libc::c_int as UDWORD;
        if maxtrys < 8 as libc::c_int as libc::c_uint {
            maxtrys = maxtrys.wrapping_add(1);
            return pickAStructure()
        } else {
            maxtrys = 0 as libc::c_int as UDWORD;
            return 0 as *mut STRUCTURE
        }
    }
    snum = snum.wrapping_add(1);
    maxtrys = 0 as libc::c_int as UDWORD;
    return pS;
}
//score
// ////////////////////////////////////////////////////////////////////////
// Send structure information.
unsafe extern "C" fn sendStructureCheck() -> BOOL {
    static mut lastSent: UDWORD =
        0 as libc::c_int as UDWORD; // last time a struct was sent.
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut capacity: UBYTE = 0;
    if lastSent > gameTime { lastSent = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastSent) < 700 as libc::c_int as libc::c_uint {
        // only send a struct send if not done recently.
        return 1 as libc::c_int
    }
    lastSent = gameTime;
    pS = pickAStructure();
    if !pS.is_null() && (*pS).status as libc::c_int == SS_BUILT as libc::c_int
       {
        // only send info about complete buildings.
        m.body[0 as libc::c_int as usize] =
            (*pS).player as libc::c_char; // send struct details
        memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); // damage
        memcpy(&mut *m.body.as_mut_ptr().offset(5 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).body as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as
                   libc::c_ulong); // building type.
        memcpy(&mut *m.body.as_mut_ptr().offset(7 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*(*pS).pStructureType).ref_0 as *mut UDWORD as
                   *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong); //position
        memcpy(&mut *m.body.as_mut_ptr().offset(11 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).x as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(13 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).y as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(15 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).z as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut *m.body.as_mut_ptr().offset(17 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*pS).direction as *mut UWORD as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        m.type_0 = NET_CHECK_STRUCT as libc::c_int as libc::c_uchar;
        m.size = 19 as libc::c_int as libc::c_ushort;
        if (*(*pS).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
            capacity =
                (*((*pS).pFunctionality as *mut RESEARCH_FACILITY)).capacity
                    as UBYTE;
            memcpy(&mut *m.body.as_mut_ptr().offset(19 as libc::c_int as
                                                        isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut capacity as *mut UBYTE as *const libc::c_void,
                   ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
            m.size =
                (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort
        }
        if (*(*pS).pStructureType).type_0 ==
               REF_FACTORY as libc::c_int as libc::c_uint ||
               (*(*pS).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
            capacity = (*((*pS).pFunctionality as *mut FACTORY)).capacity;
            memcpy(&mut *m.body.as_mut_ptr().offset(19 as libc::c_int as
                                                        isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut capacity as *mut UBYTE as *const libc::c_void,
                   ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
            m.size =
                (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort
        }
        if (*(*pS).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint {
            capacity =
                (*((*pS).pFunctionality as *mut POWER_GEN)).capacity as UBYTE;
            memcpy(&mut *m.body.as_mut_ptr().offset(19 as libc::c_int as
                                                        isize) as
                       *mut libc::c_char as *mut libc::c_void,
                   &mut capacity as *mut UBYTE as *const libc::c_void,
                   ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
            m.size =
                (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort
        }
        NETbcast(&mut m, 0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
// functionality.
// receive checking info about a structure and update local world state
#[no_mangle]
pub unsafe extern "C" fn recvStructureCheck(mut m: *mut NETMSG) -> BOOL {
    let mut x: UWORD = 0; // Damage update.
    let mut y: UWORD = 0;
    let mut z: UWORD = 0;
    let mut dir: UWORD = 0;
    let mut ref_0: UDWORD = 0;
    let mut type_0: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut i: UBYTE = 0;
    let mut player: UBYTE = 0;
    let mut cap: UBYTE = 0;
    let mut pS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    player = (*m).body[0 as libc::c_int as usize] as UBYTE;
    memcpy(&mut ref_0 as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pS = IdToStruct(ref_0, player as UDWORD);
    if !pS.is_null() {
        memcpy(&mut (*pS).body as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(5 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut (*pS).direction as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(17 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    } else {
        // structure wasn't found, create it.
        memcpy(&mut type_0 as *mut UDWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(7 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        memcpy(&mut x as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(11 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut y as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(13 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut z as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(15 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        memcpy(&mut dir as *mut UWORD as *mut libc::c_void,
               &mut *(*m).body.as_mut_ptr().offset(17 as libc::c_int as isize)
                   as *mut libc::c_char as *const libc::c_void,
               ::std::mem::size_of::<UWORD>() as libc::c_ulong);
        NETlogEntry(b"scheck:structure check failed, adding struct. val=type\x00"
                        as *const u8 as *const libc::c_char as *mut CHAR,
                    0 as libc::c_int as UDWORD,
                    type_0.wrapping_sub(0xd0000 as libc::c_int as
                                            libc::c_uint));
        i = 0 as libc::c_int as UBYTE;
        while (i as libc::c_uint) < numStructureStats &&
                  (*asStructureStats.offset(i as isize)).ref_0 != type_0 {
            i = i.wrapping_add(1)
        }
        psStats =
            &mut *asStructureStats.offset(i as isize) as *mut STRUCTURE_STATS;
        // check for similar buildings, to avoid overlaps
        if (*mapTile((x as libc::c_int >> 7 as libc::c_int) as UDWORD,
                     (y as libc::c_int >> 7 as libc::c_int) as
                         UDWORD)).tileInfoBits as libc::c_int &
               0x1 as libc::c_int != 0 {
            NETlogEntry(b"scheck:Tile has structure val=player\x00" as
                            *const u8 as *const libc::c_char as *mut CHAR,
                        0 as libc::c_int as UDWORD, player as UDWORD);
            // else remove local copy. with a bang (make it look like an explosion, itll update next time around).
			// ?? dunno if we should do this!
	//		return TRUE;					// structure exists already there....
            pS =
                getTileStructure((x as libc::c_int >> 7 as libc::c_int) as
                                     UDWORD,
                                 (y as libc::c_int >> 7 as libc::c_int) as
                                     UDWORD);
            if !pS.is_null() && (*(*pS).pStructureType).type_0 == type_0 &&
                   (*pS).player as libc::c_int == player as libc::c_int {
                (*pS).direction = dir;
                (*pS).id = ref_0;
                if (*pS).status as libc::c_int != SS_BUILT as libc::c_int {
                    (*pS).status = SS_BUILT as libc::c_int as UBYTE;
                    buildingComplete(pS);
                }
                NETlogEntry(b"scheck: fixed?\x00" as *const u8 as
                                *const libc::c_char as *mut CHAR,
                            0 as libc::c_int as UDWORD, player as UDWORD);
            } else if (*(*pS).pStructureType).type_0 ==
                          REF_WALL as libc::c_int as libc::c_uint {
                if (*psStats).type_0 ==
                       REF_WALLCORNER as libc::c_int as libc::c_uint {
                    NETlogEntry(b"scheck: fixed wall->cornerwall\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut CHAR, 0 as libc::c_int as UDWORD,
                                0 as libc::c_int as UDWORD);
                    removeStruct(pS, 1 as libc::c_int);
                    // if correct type && player then complete & modify
                    // wall becoming a cornerwall
                    powerCalc(0 as libc::c_int); // turn off power
                    pS =
                        buildStructure(psStats, x as UDWORD, y as UDWORD,
                                       player as UDWORD,
                                       1 as libc::c_int); //turn on power
                    powerCalc(1 as libc::c_int);
                    if !pS.is_null() {
                        (*pS).id = ref_0
                    } else {
                        NETlogEntry(b"scheck: failed to upgrade wall!\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut CHAR, 0 as libc::c_int as UDWORD,
                                    player as UDWORD);
                        return 0 as libc::c_int
                    }
                }
            } else {
                NETlogEntry(b"scheck:Tile did not have correct type or player val=player\x00"
                                as *const u8 as *const libc::c_char as
                                *mut CHAR, 0 as libc::c_int as UDWORD,
                            player as UDWORD);
                return 0 as libc::c_int
            }
        } else {
            NETlogEntry(b"scheck: didn\'t find structure at all, building it\x00"
                            as *const u8 as *const libc::c_char as *mut CHAR,
                        0 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD);
            //turn on power
            powerCalc(0 as libc::c_int);
            pS =
                buildStructure(psStats, x as UDWORD, y as UDWORD,
                               player as UDWORD, 1 as libc::c_int);
            powerCalc(1 as libc::c_int);
        }
    }
    if !pS.is_null() {
        if (*pS).status as libc::c_int != SS_BUILT as libc::c_int {
            //			buildFlatten(psStats, x,y,z);
//			levelGround(x,y,z,
//				((STRUCTURE_STATS *)psStats)->baseWidth,
//				((STRUCTURE_STATS *)psStats)->baseBreadth);
            // turn off power
            // check its finished
            (*pS).direction = dir;
            (*pS).id = ref_0;
            (*pS).status = SS_BUILT as libc::c_int as UBYTE;
            buildingComplete(pS);
        }
        if (*m).size as libc::c_int > 19 as libc::c_int {
            // capacity
            memcpy(&mut i as *mut UBYTE as *mut libc::c_void,
                   &mut *(*m).body.as_mut_ptr().offset(19 as libc::c_int as
                                                           isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
            match (*(*pS).pStructureType).type_0 {
                10 => {
                    // current capacity
                    cap =
                        (*((*pS).pFunctionality as
                               *mut RESEARCH_FACILITY)).capacity as UBYTE
                }
                1 | 17 => {
                    //			case REF_CYBORG_FACTORY:
                    cap = (*((*pS).pFunctionality as *mut FACTORY)).capacity
                }
                3 => {
                    cap =
                        (*((*pS).pFunctionality as *mut POWER_GEN)).capacity
                            as UBYTE
                }
                _ => {
                    NETlogEntry(b"unknown upgrade error in recv struct check\x00"
                                    as *const u8 as *const libc::c_char as
                                    *mut CHAR, 0 as libc::c_int as UDWORD,
                                0 as libc::c_int as UDWORD);
                    cap = 0 as libc::c_int as UBYTE
                }
            }
            if cap as libc::c_int != i as libc::c_int {
                // compare and upgrade
                match (*(*pS).pStructureType).type_0 {
                    10 => {
                        //for (j = 0; (j<numStructureStats) && (asStructureStats[j].type != REF_RESEARCH_MODULE);j++);
                        j = researchModuleStat
                    }
                    1 | 17 => {
                        //			case REF_CYBORG_FACTORY:
					//for (j = 0; (j<numStructureStats) && (asStructureStats[j].type != REF_FACTORY_MODULE);j++);
                        j = factoryModuleStat
                    }
                    3 => {
                        //for (j = 0; (j<numStructureStats) && (asStructureStats[j].type != REF_POWER_MODULE);j++);
                        j = powerModuleStat
                    }
                    _ => {
                        j = 0 as libc::c_int as UDWORD;
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Unknown Upgrade in structure checking!\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"multisync.c\x00" as *const u8 as
                                      *const libc::c_char, 880 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 19],
                                                            &[libc::c_char; 19]>(b"recvStructureCheck\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                        return 1 as libc::c_int
                    }
                }
                i = (i as libc::c_int - cap as libc::c_int) as UBYTE;
                while i as libc::c_int > 0 as libc::c_int {
                    buildStructure(&mut *asStructureStats.offset(j as isize),
                                   (*pS).x as UDWORD, (*pS).y as UDWORD,
                                   (*pS).player as UDWORD, 0 as libc::c_int);
                    if !pS.is_null() &&
                           (*pS).status as libc::c_int !=
                               SS_BUILT as libc::c_int {
                        // check its finished again.
                        (*pS).id = ref_0;
                        (*pS).status = SS_BUILT as libc::c_int as UBYTE;
                        buildingComplete(pS);
                    }
                    i = (i as libc::c_int - 1 as libc::c_int) as UBYTE
                }
            }
        }
    }
    return 1 as libc::c_int;
}
//Structure
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Power Checking. Send a power level check every now and again.
#[no_mangle]
pub unsafe extern "C" fn sendPowerCheck(mut now: BOOL) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    static mut lastsent: UDWORD = 0 as libc::c_int as UDWORD;
    if now == 0 {
        if lastsent > gameTime { lastsent = 0 as libc::c_int as UDWORD }
        if gameTime.wrapping_sub(lastsent) <
               10000 as libc::c_int as libc::c_uint {
            // only send if not done recently.
            return 1 as libc::c_int
        }
    }
    lastsent = gameTime;
    // ok send a power check.
    m.body[0 as libc::c_int as usize] = selectedPlayer as libc::c_char;
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (**asPower.as_mut_ptr().offset(selectedPlayer as
                                                   isize)).currentPower as
               *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_CHECK_POWER as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn recvPowerCheck(mut pMsg: *mut NETMSG) -> BOOL {
    let mut player: UDWORD = 0;
    let mut b: UDWORD = 0;
    player = (*pMsg).body[0 as libc::c_int as usize] as UDWORD;
    memcpy(&mut b as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    setPower(player, b);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Score
#[no_mangle]
pub unsafe extern "C" fn sendScoreCheck() -> BOOL {
    static mut lastsent: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut stats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if lastsent > gameTime { lastsent = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastsent) < 25000 as libc::c_int as libc::c_uint
       {
        // only send if not done recently.
        return 1 as libc::c_int
    }
    lastsent = gameTime;
    // syncronise scores.
    // update local score
    stats =
        getMultiStats(selectedPlayer,
                      1 as libc::c_int); // add recently scored points.
    stats.recentKills += stats.killsToAdd; // store local version.
    stats.totalKills += stats.killsToAdd; // send score to the ether
    stats.recentScore += stats.scoreToAdd;
    stats.totalScore += stats.scoreToAdd;
    stats.killsToAdd = 0 as libc::c_int;
    stats.scoreToAdd = 0 as libc::c_int;
    setMultiStats(player2dpid[selectedPlayer as usize], stats,
                  1 as libc::c_int);
    setMultiStats(player2dpid[selectedPlayer as usize], stats,
                  0 as libc::c_int);
    // broadcast any changes in other players, but not in FRONTEND!!!
    if titleMode as libc::c_uint != MULTIOPTION as libc::c_int as libc::c_uint
           &&
           titleMode as libc::c_uint !=
               MULTILIMIT as libc::c_int as libc::c_uint {
        m.size = 0 as libc::c_int as libc::c_ushort; // terminate msg.
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            if isHumanPlayer(i) != 0 && i != selectedPlayer {
                stats = getMultiStats(i, 1 as libc::c_int);
                if stats.killsToAdd != 0 || stats.scoreToAdd != 0 {
                    m.body[m.size as usize] = i as UBYTE as libc::c_char;
                    m.size =
                        (m.size as libc::c_int + 1 as libc::c_int) as
                            libc::c_ushort;
                    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize)
                               as *mut libc::c_char as *mut libc::c_void,
                           &mut stats.killsToAdd as *mut DWORD as
                               *const libc::c_void,
                           ::std::mem::size_of::<DWORD>() as libc::c_ulong);
                    m.size =
                        (m.size as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<DWORD>()
                                                            as libc::c_ulong)
                            as libc::c_ushort as libc::c_ushort;
                    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize)
                               as *mut libc::c_char as *mut libc::c_void,
                           &mut stats.scoreToAdd as *mut SDWORD as
                               *const libc::c_void,
                           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
                    m.size =
                        (m.size as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>()
                                                            as libc::c_ulong)
                            as libc::c_ushort as libc::c_ushort
                }
            }
            i = i.wrapping_add(1)
        }
        if m.size as libc::c_int != 0 as libc::c_int {
            m.body[m.size as usize] = 99 as libc::c_int as libc::c_char;
            m.size =
                (m.size as libc::c_int + 1 as libc::c_int) as libc::c_ushort;
            m.type_0 = NET_SCORESUBMIT as libc::c_int as libc::c_uchar;
            NETbcast(&mut m, 0 as libc::c_int);
        }
    }
    // get global versions of scores.
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 {
            setMultiStats(player2dpid[i as usize],
                          getMultiStats(i, 0 as libc::c_int),
                          1 as libc::c_int);
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn recvScoreSubmission(mut pMsg: *mut NETMSG) -> BOOL {
    let mut player: UDWORD = 0 as libc::c_int as UDWORD;
    let mut kil: UDWORD = 0;
    let mut index: UDWORD = 0;
    let mut sco: SDWORD = 0;
    let mut stats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    // process msg, add to next score addition.
    index = 0 as libc::c_int as UDWORD;
    while player != 99 as libc::c_int as libc::c_uint {
        player = (*pMsg).body[index as usize] as UDWORD;
        index =
            (index as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        if player != 99 as libc::c_int as libc::c_uint {
            memcpy(&mut kil as *mut UDWORD as *mut libc::c_void,
                   &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
            index =
                (index as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                    as libc::c_ulong) as
                    UDWORD as UDWORD;
            memcpy(&mut sco as *mut SDWORD as *mut libc::c_void,
                   &mut *(*pMsg).body.as_mut_ptr().offset(index as isize) as
                       *mut libc::c_char as *const libc::c_void,
                   ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
            index =
                (index as
                     libc::c_uint).wrapping_add(::std::mem::size_of::<SDWORD>()
                                                    as libc::c_ulong) as
                    UDWORD as UDWORD;
            // do the update.
            if player == selectedPlayer {
                stats = getMultiStats(player, 1 as libc::c_int);
                stats.killsToAdd =
                    (stats.killsToAdd as libc::c_uint).wrapping_add(kil) as
                        DWORD as DWORD;
                stats.scoreToAdd += sco;
                setMultiStats(player2dpid[player as usize], stats,
                              1 as libc::c_int);
                // store local version.
            }
        }
    }
    return 1 as libc::c_int;
}
//droids
// ////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////
// Pings
#[no_mangle]
pub unsafe extern "C" fn averagePing() -> UDWORD {
    let mut i: UDWORD = 0; // last time we sent a ping.
    let mut count: UDWORD = 0; // last time we updated average.
    let mut total: UDWORD = 0;
    count = 0 as libc::c_int as UDWORD;
    total = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 {
            total =
                (total as
                     libc::c_uint).wrapping_add(ingame.PingTimes[i as usize])
                    as UDWORD as UDWORD;
            count = count.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    return total.wrapping_div(count);
}
#[no_mangle]
pub unsafe extern "C" fn sendPing() -> BOOL {
    let mut ping: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut i: UDWORD = 0;
    static mut lastPing: UDWORD = 0 as libc::c_int as UDWORD;
    static mut lastav: UDWORD = 0 as libc::c_int as UDWORD;
    // only ping every so often.
    if lastPing > gameTime { lastPing = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastPing) < 12000 as libc::c_int as libc::c_uint
       {
        return 1 as libc::c_int
    }
    lastPing = gameTime;
    // / if host, also update the average ping stat for joiners.
    if NetPlay.bHost != 0 {
        if lastav > gameTime { lastav = 0 as libc::c_int as UDWORD }
        if gameTime.wrapping_sub(lastav) >
               45000 as libc::c_int as libc::c_uint {
            NETsetGameFlags(2 as libc::c_int as UDWORD,
                            averagePing() as DWORD);
            lastav = gameTime
        }
    }
    // before we send the ping, if any player failed to respond to the last one
	// we should re-enumerate the players.
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 && PingSend[i as usize] != 0 &&
               ingame.PingTimes[i as usize] != 0 && i != selectedPlayer {
            //		CONPRINTF(ConsoleString,(ConsoleString,strresGetString(psStringRes,STR_MUL_RESPOND),getPlayerName(i) ));
            ingame.PingTimes[i as usize] = 2000 as libc::c_int as UDWORD
        } else if isHumanPlayer(i) == 0 && PingSend[i as usize] != 0 &&
                      ingame.PingTimes[i as usize] != 0 && i != selectedPlayer
         {
            ingame.PingTimes[i as usize] = 0 as libc::c_int as UDWORD
        }
        i = i.wrapping_add(1)
    }
    ping.body[0 as libc::c_int as usize] = selectedPlayer as libc::c_char;
    ping.body[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char;
    ping.size = 2 as libc::c_int as libc::c_ushort;
    ping.type_0 = NET_PING as libc::c_int as libc::c_uchar;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        PingSend[i as usize] = gameTime2;
        i = i.wrapping_add(1)
        //clock();
    }
    return NETbcast(&mut ping, 0 as libc::c_int);
}
// accept and process incoming ping messages.
#[no_mangle]
pub unsafe extern "C" fn recvPing(mut ping: *mut NETMSG) -> BOOL {
    let mut reply: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    if (*ping).body[1 as libc::c_int as usize] as libc::c_int ==
           1 as libc::c_int {
        // if this is a new ping
        reply.body[0 as libc::c_int as usize] =
            selectedPlayer as libc::c_char;
        reply.body[1 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        reply.size = 2 as libc::c_int as libc::c_ushort;
        reply.type_0 = NET_PING as libc::c_int as libc::c_uchar;
        NETsend(&mut reply,
                player2dpid[(*ping).body[0 as libc::c_int as usize] as
                                libc::c_int as usize], 0 as libc::c_int);
        // reply to it
    } else {
        // else it's returned, so store it.
        ingame.PingTimes[(*ping).body[0 as libc::c_int as usize] as
                             libc::c_int as usize] =
            gameTime2.wrapping_sub(PingSend[(*ping).body[0 as libc::c_int as
                                                             usize] as
                                                libc::c_int as
                                                usize]).wrapping_div(2 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint);
        PingSend[(*ping).body[0 as libc::c_int as usize] as libc::c_int as
                     usize] = 0 as libc::c_int as UDWORD
        // note we've recvd it!
    }
    return 1 as libc::c_int;
}
