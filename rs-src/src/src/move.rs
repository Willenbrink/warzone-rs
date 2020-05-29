use ::libc;
extern "C" {
    #[no_mangle]
    fn atan(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn labs(_: libc::c_long) -> libc::c_long;
    /* Lookup trig functions */
    #[no_mangle]
    fn trigSin(angle: SDWORD) -> FRACT;
    #[no_mangle]
    fn trigCos(angle: SDWORD) -> FRACT;
    #[no_mangle]
    fn trigInvCos(val: FRACT) -> FRACT;
    /* Supposedly fast lookup sqrt - unfortunately it's probably slower than the FPU sqrt :-( */
    #[no_mangle]
    fn trigIntSqrt(val: UDWORD) -> FRACT;
    #[no_mangle]
    fn animObj_SetDoneFunc(psObj: *mut ANIM_OBJECT,
                           pDoneFunc: ANIMOBJDONEFUNC);
    /* uwCycles=0 for infinite looping */
    #[no_mangle]
    fn animObj_Add(pParentObj: *mut libc::c_void, iAnimID: libc::c_int,
                   udwStartDelay: UDWORD, uwCycles: UWORD)
     -> *mut ANIM_OBJECT;
    #[no_mangle]
    fn animObj_Remove(ppsObj: *mut *mut ANIM_OBJECT, iAnimID: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayObjDynamicTrack(psObj: *mut libc::c_void,
                                 iTrack: libc::c_int,
                                 pUserCallback: AUDIO_CALLBACK) -> BOOL;
    /* Store for the objects near the droid currently being updated */
    #[no_mangle]
    static mut asDroidNaybors: [NAYBOR_INFO; 100];
    #[no_mangle]
    static mut numNaybors: UDWORD;
    // refresh the naybor list
// this only does anything if the naybor list is out of date
    #[no_mangle]
    fn droidGetNaybors(psDroid: *mut DROID);
    /* Calculate the speed of a droid over a terrain */
    #[no_mangle]
    fn calcDroidSpeed(baseSpeed_0: UDWORD, terrainType: UDWORD,
                      propIndex: UDWORD) -> UDWORD;
    /* Remove a droid and free it's memory */
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    // free up a feature with no visual effects
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    /* Return the difference in directions */
    #[no_mangle]
    fn dirDiff(start: SDWORD, end: SDWORD) -> UDWORD;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asPropulsionTypes: *mut PROPULSION_TYPES;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    /* Check which tiles can be seen by an object */
    #[no_mangle]
    fn visTilesUpdate(psObj: *mut BASE_OBJECT, SpreadLoad: BOOL);
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
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
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
    // Find a route for an object to a location
    #[no_mangle]
    fn fpathRoute(psObj: *mut BASE_OBJECT, psMoveCntl: *mut MOVE_CONTROL,
                  targetX: SDWORD, targetY: SDWORD) -> FPATH_RETVAL;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
    // Check if the map tile at a location blocks a droid
    #[no_mangle]
    fn fpathGroundBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    fn fpathLiftSlideBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
    /* set the correct blocking tile function */
    #[no_mangle]
    fn fpathSetBlockingTile(ubPropulsionType: UBYTE);
    /* set pointer for current fpath object - GJ hack */
    #[no_mangle]
    fn fpathSetCurrentObject(psDroid: *mut BASE_OBJECT);
    /* set direct path to position */
    #[no_mangle]
    fn fpathSetDirectRoute(psObj: *mut BASE_OBJECT, targetX: SDWORD,
                           targetY: SDWORD);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    // Create a new formation
    #[no_mangle]
    fn formationNew(ppsFormation: *mut *mut FORMATION, type_0: FORMATION_TYPE,
                    x: SDWORD, y: SDWORD, dir: SDWORD) -> BOOL;
    // Try and find a formation near to a location
    #[no_mangle]
    fn formationFind(ppsFormation: *mut *mut FORMATION, x: SDWORD, y: SDWORD)
     -> BOOL;
    // Associate a unit with a formation
    #[no_mangle]
    fn formationJoin(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT);
    // Remove a unit from a formation
    #[no_mangle]
    fn formationLeave(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT);
    // get a target position to move into a formation
    #[no_mangle]
    fn formationGetPos(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT,
                       pX: *mut SDWORD, pY: *mut SDWORD, bCheckLOS: BOOL)
     -> BOOL;
    // See if a unit is a member of a formation (i.e. it has a position assigned)
    #[no_mangle]
    fn formationMember(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT)
     -> BOOL;
    /* Give a droid an action */
    #[no_mangle]
    fn actionDroid(psDroid: *mut DROID, action: DROID_ACTION);
    // choose a landing position for a VTOL when it goes to rearm
    #[no_mangle]
    fn actionVTOLLandingPos(psDroid: *mut DROID, px: *mut UDWORD,
                            py: *mut UDWORD) -> BOOL;
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    // reset the astar counters
    #[no_mangle]
    fn astarResetCounters();
    // Check los between two tiles
    #[no_mangle]
    fn fpathTileLOS(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> BOOL;
    // move an object within the grid
// oldX,oldY are the old position of the object in world coords
    #[no_mangle]
    fn gridMoveObject(psObj: *mut BASE_OBJECT, oldX: SDWORD, oldY: SDWORD);
    #[no_mangle]
    static mut godMode: BOOL;
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    #[no_mangle]
    fn scoreUpdateVar(var: DATA_INDEX);
    #[no_mangle]
    fn driveSetDroidMove(psDroid: *mut DROID);
    #[no_mangle]
    fn driveGetMoveSpeed() -> SDWORD;
    #[no_mangle]
    fn driveGetMoveDir() -> SDWORD;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // true when more than 1 player.
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn SendDroidMove(psDroid: *mut DROID, x: UDWORD, y: UDWORD,
                     bFormation: BOOL) -> BOOL;
    // deathmatch stuff
    #[no_mangle]
    fn addOilDrum(count: UDWORD) -> BOOL;
    #[no_mangle]
    fn giftPower(from: UDWORD, to: UDWORD, send: BOOL);
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
/* SIZE used for specifying body size */
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
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
pub struct _body_stats {
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
    pub size: UBYTE,
    pub weaponSlots: UDWORD,
    pub armourValue: [UDWORD; 2],
    pub powerOutput: UDWORD,
    pub ppIMDList: *mut *mut iIMDShape,
    pub pFlameIMD: *mut iIMDShape,
}
pub type BODY_STATS = _body_stats;
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
//pointer to which flame graphic to use - for VTOLs only at the moment
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
pub type WEAPON_STATS = _weapon_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_types {
    pub powerRatioMult: UWORD,
    pub travel: UDWORD,
    pub startID: SWORD,
    pub idleID: SWORD,
    pub moveOffID: SWORD,
    pub moveID: SWORD,
    pub hissID: SWORD,
    pub shutDownID: SWORD,
}
pub type PROPULSION_TYPES = _propulsion_types;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
//sound to play when this prop type shuts down
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
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
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
pub type BASE_OBJECT = _base_object;
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
pub type MOVE_CONTROL = _move_control;
// position relative to center
// orientation of line
// first member in the 'linked list' of members
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
pub type ALuint = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AUDIO_SAMPLE {
    pub iTrack: SDWORD,
    pub iSample: ALuint,
    pub x: SDWORD,
    pub y: SDWORD,
    pub z: SDWORD,
    pub iLoops: SDWORD,
    pub bRemove: BOOL,
    pub pCallback: AUDIO_CALLBACK,
    pub psObj: *mut libc::c_void,
    pub psPrev: *mut AUDIO_SAMPLE,
    pub psNext: *mut AUDIO_SAMPLE,
}
pub type AUDIO_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL>;
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
// Feature armour
/* The range for neighbouring objects */
//used to stop structures being built too near the edge and droids being placed down - pickATile
/* Info stored for each droid neighbour */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _naybor_info {
    pub psObj: *mut BASE_OBJECT,
    pub distSqr: UDWORD,
}
pub type NAYBOR_INFO = _naybor_info;
// The square of the distance to the object
//UDWORD			dist;			// The distance to the object
/*
 * Action.h
 *
 * Function prototypes for setting the action of a droid
 *
 */
// What a droid is currently doing
// Not necessarily the same as it's order as the AI may get a droid to do
// something else whilst carrying out an order
pub type DROID_ACTION = _droid_action;
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
pub const DACTION_DROIDREPAIR: _droid_action = 14;
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
pub const FPR_WAIT: _fpath_retval = 2;
// not doing anything
// 1 moving to a location
// building a structure
// 3 building a foundation for a structure
// demolishing a structure
// 5 repairing a structure
// attacking something
// 7 observing something
// attacking something visible by a sensor droid
// 9 refuse to do anything aggresive for a fixed time
// self destruct
// 11 move transporter offworld
// wait for timer to move reinforcements in
// 13 move transporter onworld
// repairing a droid
// 15 restore resistance points of a structure
// clearing building wreckage
// The states below are used by the action system
	// but should not be given as an action
// 17
// moving to a new building location
// 19 moving to a new demolition location
// moving to a new repair location
// 21 moving around while building
// moving around while building the foundation
// 23 moving to a target to attack
// rotating to a target to attack
// 25 moving to be able to see a target
// waiting to be repaired by a facility
// 27 move to repair facility repair point
// waiting to be repaired by a facility
// 29 moving to a new location next to droid to be repaired
// moving to a low resistance structure
// 31 moving to a building wreck location
// (32)moving to a rearming pad - VTOLS
// (33)waiting for rearm - VTOLS
// (34)move to rearm point - VTOLS - this actually moves them onto the pad
// (35)waiting during rearm process- VTOLS
// (36) a VTOL droid doing attack runs
// (37) a VTOL droid being told to get off a rearm pad
// (38) used by scout/patrol order when returning to route
// (39) used by firesupport order when sensor retreats
// return values for routing
pub type FPATH_RETVAL = _fpath_retval;
pub type _fpath_retval = libc::c_uint;
pub const FPR_RESCHEDULE: _fpath_retval = 3;
pub const FPR_FAILED: _fpath_retval = 1;
pub const FPR_OK: _fpath_retval = 0;
// found a route
// failed to find a route
// route was too long to calculate this frame
// routing will continue on succeeding frames
// didn't try to route because too much time has been
// spent routing this frame
// information about a formation
pub type FORMATION = _formation;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
/*
 * Formation.h
 *
 * Control units moving in formation.
 *
 */
pub type FORMATION_TYPE = _formation_type;
pub type _formation_type = libc::c_uint;
pub const FT_COLUMN: _formation_type = 1;
pub const FT_LINE: _formation_type = 0;
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
pub const NO_SOUND: C2RustUnnamed = -1;
pub const DORDER_NONE: _droid_order = 0;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed = 293;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed = 282;
pub const ID_SOUND_TREAD: C2RustUnnamed = 289;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed = 284;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed = 285;
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
pub const TER_WATER: _terrain_type = 7;
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
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
// --------------------------------------------------------------------
pub type DATA_INDEX = data_index;
pub type data_index = libc::c_uint;
pub const WD_BARBARIANS_MOWED_DOWN: data_index = 10;
pub const WD_SHOTS_OFF_TARGET: data_index = 9;
pub const WD_SHOTS_ON_TARGET: data_index = 8;
pub const WD_MISSION_STARTED: data_index = 7;
pub const WD_ARTEFACTS_FOUND: data_index = 6;
pub const WD_STR_LOST: data_index = 5;
pub const WD_STR_KILLED: data_index = 4;
pub const WD_STR_BUILT: data_index = 3;
pub const WD_UNITS_LOST: data_index = 2;
pub const WD_UNITS_KILLED: data_index = 1;
pub const WD_UNITS_BUILT: data_index = 0;
pub const DORDER_RUNBURN: _droid_order = 29;
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
pub type C2RustUnnamed = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed = 350;
pub const ID_SOUND_EMP: C2RustUnnamed = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed = 301;
pub const ID_SOUND_HELP: C2RustUnnamed = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed = 294;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed = 290;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed = 286;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed = 283;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed = 160;
pub const ID_GIFT: C2RustUnnamed = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed = 109;
pub const ID_SOUND_NO: C2RustUnnamed = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed = 0;
pub type _droid_order = libc::c_uint;
pub const DORDER_RTR_SPECIFIED: _droid_order = 37;
pub const DORDER_LEAVEMAP: _droid_order = 36;
pub const DORDER_RECOVER: _droid_order = 35;
pub const DORDER_SCOUT_ATTACKWALL: _droid_order = 34;
pub const DORDER_MOVE_ATTACKWALL: _droid_order = 33;
pub const DORDER_REARM: _droid_order = 32;
pub const DORDER_PATROL: _droid_order = 31;
pub const DORDER_CLEARWRECK: _droid_order = 30;
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
/* Return whether a world coordinate is on the map */
#[inline]
unsafe extern "C" fn worldOnMap(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    return (x >= 0 as libc::c_int &&
                x < (mapWidth as SDWORD) << 7 as libc::c_int &&
                y >= 0 as libc::c_int &&
                y < (mapHeight as SDWORD) << 7 as libc::c_int) as libc::c_int;
}
/* The current base speed for this frame and averages for the last few seconds */
#[no_mangle]
pub static mut baseSpeed: FRACT = 0.;
#[no_mangle]
pub static mut baseTimes: [UDWORD; 10] = [0; 10];
/* The current base turn rate */
#[no_mangle]
pub static mut baseTurn: FRACT = 0.;
// The next object that should get the router when a lot of units are
// in a MOVEROUTE state
#[no_mangle]
pub static mut psNextRouteDroid: *mut DROID = 0 as *const DROID as *mut DROID;
//typedef enum MOVESOUNDTYPE	{ MOVESOUNDSTART, MOVESOUNDIDLE, MOVESOUNDMOVEOFF,
//								MOVESOUNDMOVE, MOVESOUNDSTOPHISS, MOVESOUNDSHUTDOWN };
static mut g_bFormationSpeedLimitingOn: BOOL = 1 as libc::c_int;
/* Initialise the movement system */
#[no_mangle]
pub unsafe extern "C" fn moveInitialise() -> BOOL {
    let mut i: UDWORD = 0;
    // Initialise the base speed counters
    baseSpeed =
        1 as libc::c_int as libc::c_float /
            25 as libc::c_int as libc::c_float;
    baseTurn =
        1 as libc::c_int as libc::c_float /
            25 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        baseTimes[i as usize] =
            (1000 as libc::c_int / 25 as libc::c_int) as UDWORD;
        i = i.wrapping_add(1)
    }
    psNextRouteDroid = 0 as *mut DROID;
    return 1 as libc::c_int;
}
/* Update the base speed for all movement */
#[no_mangle]
pub unsafe extern "C" fn moveUpdateBaseSpeed() {
    //	UDWORD	totalTime=0, numFrames=0, i;
    let mut totalTime: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    // Update the list of frame times
    i = 0 as libc::c_int as UDWORD;
    while i < (10 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        baseTimes[i as usize] =
            baseTimes[i.wrapping_add(1 as libc::c_int as libc::c_uint) as
                          usize];
        i = i.wrapping_add(1)
    }
    baseTimes[(10 as libc::c_int - 1 as libc::c_int) as usize] = frameTime;
    // Add up the time for the last few frames
    i = 0 as libc::c_int as UDWORD;
    while i < 10 as libc::c_int as libc::c_uint {
        totalTime =
            (totalTime as libc::c_uint).wrapping_add(baseTimes[i as usize]) as
                UDWORD as UDWORD;
        i = i.wrapping_add(1)
    }
    // Set the base speed
	// here is the original calculation before the fract stuff
	// baseSpeed = (totalTime * BASE_SPEED) / (GAME_TICKS_PER_SEC * BASE_FRAMES);
    baseSpeed =
        1 as libc::c_int as FRACT * totalTime as FRACT /
            (1000 as libc::c_int as FRACT * 10 as libc::c_int as FRACT);
    // Set the base turn rate
    baseTurn =
        1 as libc::c_int as FRACT * totalTime as FRACT /
            (1000 as libc::c_int as FRACT * 10 as libc::c_int as FRACT);
    // reset the astar counters
    astarResetCounters();
    // check the waiting droid pointer
    if !psNextRouteDroid.is_null() {
        if (*psNextRouteDroid).died != 0 ||
               (*psNextRouteDroid).sMove.Status as libc::c_int !=
                   7 as libc::c_int &&
                   (*psNextRouteDroid).sMove.Status as libc::c_int !=
                       13 as libc::c_int {
            psNextRouteDroid = 0 as *mut DROID
        }
    };
}
/* Set a target location for a droid to move to */
// Now returns a BOOL based on the success of the routing
// returns TRUE if the routing was successful ... if FALSE then the calling code should not try to route here again for a while
#[no_mangle]
pub unsafe extern "C" fn _moveDroidToBase(mut psDroid: *mut DROID,
                                          mut x: UDWORD, mut y: UDWORD,
                                          mut bFormation: BOOL) -> BOOL {
    let mut retVal: FPATH_RETVAL = FPR_OK;
    let mut fmx1: SDWORD = 0;
    let mut fmy1: SDWORD = 0;
    let mut fmx2: SDWORD = 0;
    let mut fmy2: SDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUnitTo: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              370 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"_moveDroidToBase\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bMultiPlayer != 0 &&
           (*psDroid).sMove.Status as libc::c_int != 11 as libc::c_int {
        if SendDroidMove(psDroid, x, y, bFormation) == 0 as libc::c_int {
            // dont make the move since we'll recv it anyway
            return 0 as libc::c_int
        }
    }
    //	DBPRINTF(("movedroidto (%d,%d) -> (%d,%d)\n",psDroid->x,psDroid->y,x,y);
    //in multiPlayer make Transporter move like the vtols
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
           game.maxPlayers as libc::c_int == 0 as libc::c_int {
        fpathSetDirectRoute(psDroid as *mut BASE_OBJECT, x as SDWORD,
                            y as SDWORD);
        (*psDroid).sMove.Status = 1 as libc::c_int as UBYTE;
        (*psDroid).sMove.Position = 0 as libc::c_int as UBYTE;
        (*psDroid).sMove.psFormation = 0 as *mut _formation;
        return 1 as libc::c_int
    } else {
        if vtolDroid(psDroid) != 0 ||
               game.maxPlayers as libc::c_int > 0 as libc::c_int &&
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            fpathSetDirectRoute(psDroid as *mut BASE_OBJECT, x as SDWORD,
                                y as SDWORD);
            retVal = FPR_OK
        } else {
            retVal =
                fpathRoute(psDroid as *mut BASE_OBJECT, &mut (*psDroid).sMove,
                           x as SDWORD, y as SDWORD)
        }
    }
    // ----
	// ----
    /* check formations */
    if retVal as libc::c_uint == FPR_OK as libc::c_int as libc::c_uint {
        // bit of a hack this - john
		// if astar doesn't have a complete route, it returns a route to the nearest clear tile.
		// the location of the clear tile is in DestinationX,DestinationY.
		// reset x,y to this position so the formation gets set up correctly
        x = (*psDroid).sMove.DestinationX as UDWORD;
        y = (*psDroid).sMove.DestinationY as UDWORD;
        (*psDroid).sMove.Status = 1 as libc::c_int as UBYTE;
        (*psDroid).sMove.Position = 0 as libc::c_int as UBYTE;
        (*psDroid).sMove.fx = (*psDroid).x as FRACT;
        (*psDroid).sMove.fy = (*psDroid).y as FRACT;
        (*psDroid).sMove.fz = (*psDroid).z as FRACT;
        //		DBPRINTF(("moveDroidTo: form %p id %d end\n",psDroid->sMove.psFormation, psDroid->id));
        if psDroid == psNextRouteDroid { psNextRouteDroid = 0 as *mut DROID }
        if !(*psDroid).sMove.psFormation.is_null() {
            formationLeave((*psDroid).sMove.psFormation,
                           psDroid as *mut BASE_OBJECT);
            (*psDroid).sMove.psFormation = 0 as *mut _formation
        }
        if bFormation != 0 {
            // reset the next route droid
            //		DBPRINTF(("moveDroidTo: form %p id %d\n",psDroid->sMove.psFormation, psDroid->id));
            // leave any old formation
            // join a formation if it exists at the destination
            if formationFind(&mut (*psDroid).sMove.psFormation, x as SDWORD,
                             y as SDWORD) != 0 {
                formationJoin((*psDroid).sMove.psFormation,
                              psDroid as *mut BASE_OBJECT);
            } else {
                // align the formation with the last path of the route
                fmx2 =
                    (*psDroid).sMove.asPath[((*psDroid).sMove.numPoints as
                                                 libc::c_int -
                                                 1 as libc::c_int) as usize].x
                        as SDWORD;
                fmy2 =
                    (*psDroid).sMove.asPath[((*psDroid).sMove.numPoints as
                                                 libc::c_int -
                                                 1 as libc::c_int) as usize].y
                        as SDWORD;
                fmx2 =
                    (fmx2 << 7 as libc::c_int) +
                        128 as libc::c_int / 2 as libc::c_int;
                fmy2 =
                    (fmy2 << 7 as libc::c_int) +
                        128 as libc::c_int / 2 as libc::c_int;
                if (*psDroid).sMove.numPoints as libc::c_int ==
                       1 as libc::c_int {
                    fmx1 = (*psDroid).x as SDWORD;
                    fmy1 = (*psDroid).y as SDWORD
                } else {
                    fmx1 =
                        (*psDroid).sMove.asPath[((*psDroid).sMove.numPoints as
                                                     libc::c_int -
                                                     2 as libc::c_int) as
                                                    usize].x as SDWORD;
                    fmy1 =
                        (*psDroid).sMove.asPath[((*psDroid).sMove.numPoints as
                                                     libc::c_int -
                                                     2 as libc::c_int) as
                                                    usize].y as SDWORD;
                    fmx1 =
                        (fmx1 << 7 as libc::c_int) +
                            128 as libc::c_int / 2 as libc::c_int;
                    fmy1 =
                        (fmy1 << 7 as libc::c_int) +
                            128 as libc::c_int / 2 as libc::c_int
                }
                // no formation so create a new one
                if formationNew(&mut (*psDroid).sMove.psFormation, FT_LINE,
                                x as SDWORD, y as SDWORD,
                                calcDirection(fmx1 as UDWORD, fmy1 as UDWORD,
                                              fmx2 as UDWORD, fmy2 as UDWORD))
                       != 0 {
                    formationJoin((*psDroid).sMove.psFormation,
                                  psDroid as *mut BASE_OBJECT);
                }
            }
        }
    } else if retVal as libc::c_uint ==
                  FPR_RESCHEDULE as libc::c_int as libc::c_uint {
        // maxed out routing time this frame - do it next time
        (*psDroid).sMove.DestinationX = x as SDWORD;
        (*psDroid).sMove.DestinationY = y as SDWORD;
        if (*psDroid).sMove.Status as libc::c_int != 7 as libc::c_int &&
               (*psDroid).sMove.Status as libc::c_int != 13 as libc::c_int {
            (*psDroid).sMove.Status = 7 as libc::c_int as UBYTE;
            //			psDroid->sMove.bumpTime = gameTime + REROUTE_BASETIME + REROUTE_RNDTIME - (rand()%REROUTE_RNDTIME);
            (*psDroid).sMove.bumpTime = gameTime
        }
    } else if retVal as libc::c_uint ==
                  FPR_WAIT as libc::c_int as libc::c_uint {
        // note when the unit first tried to route
        // reset the next route droid
        if psDroid == psNextRouteDroid { psNextRouteDroid = 0 as *mut DROID }
        // the route will be calculated over a number of frames
        (*psDroid).sMove.Status = 11 as libc::c_int as UBYTE;
        (*psDroid).sMove.DestinationX = x as SDWORD;
        (*psDroid).sMove.DestinationY = y as SDWORD
    } else {
        // if (retVal == FPR_FAILED)
        (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE;
        actionDroid(psDroid, DACTION_SULK);
        //		DBPRINTF(("mdt: FALSE\n");
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Shame about this but the find path code uses too much stack space
// so we can't safely run it in the dcache.
//
#[no_mangle]
pub unsafe extern "C" fn moveDroidToBase(mut psDroid: *mut DROID,
                                         mut x: UDWORD, mut y: UDWORD,
                                         mut bFormation: BOOL) -> BOOL {
    return _moveDroidToBase(psDroid, x, y, bFormation);
}
// move a droid to a location, joining a formation
#[no_mangle]
pub unsafe extern "C" fn moveDroidTo(mut psDroid: *mut DROID, mut x: UDWORD,
                                     mut y: UDWORD) -> BOOL {
    return moveDroidToBase(psDroid, x, y, 1 as libc::c_int);
}
// move a droid to a location, not joining a formation
#[no_mangle]
pub unsafe extern "C" fn moveDroidToNoFormation(mut psDroid: *mut DROID,
                                                mut x: UDWORD, mut y: UDWORD)
 -> BOOL {
    return moveDroidToBase(psDroid, x, y, 0 as libc::c_int);
}
// move a droid directly to a location (used by vtols only)
#[no_mangle]
pub unsafe extern "C" fn moveDroidToDirect(mut psDroid: *mut DROID,
                                           mut x: UDWORD, mut y: UDWORD) {
    if 1 as libc::c_int != 0 && vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUnitToDirect: only valid for a vtol unit\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 && vtolDroid(psDroid) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              562 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"moveDroidToDirect\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID)) && vtolDroid(psDroid)\x00" as
                  *const u8 as *const libc::c_char);
    };
    fpathSetDirectRoute(psDroid as *mut BASE_OBJECT, x as SDWORD,
                        y as SDWORD);
    (*psDroid).sMove.Status = 1 as libc::c_int as UBYTE;
    (*psDroid).sMove.Position = 0 as libc::c_int as UBYTE;
    // leave any old formation
    if !(*psDroid).sMove.psFormation.is_null() {
        formationLeave((*psDroid).sMove.psFormation,
                       psDroid as *mut BASE_OBJECT);
        (*psDroid).sMove.psFormation = 0 as *mut _formation
    };
}
// Get a droid to turn towards a locaton
#[no_mangle]
pub unsafe extern "C" fn moveTurnDroid(mut psDroid: *mut DROID, mut x: UDWORD,
                                       mut y: UDWORD) {
    let mut moveDir: SDWORD =
        calcDirection((*psDroid).x as UDWORD, (*psDroid).y as UDWORD, x, y);
    if (*psDroid).direction as libc::c_uint != moveDir as UDWORD {
        (*psDroid).sMove.targetX = x as SDWORD;
        (*psDroid).sMove.targetY = y as SDWORD;
        (*psDroid).sMove.Status = 6 as libc::c_int as UBYTE
    };
}
// get the difference in direction
#[no_mangle]
pub unsafe extern "C" fn moveDirDiff(mut start: SDWORD, mut end: SDWORD)
 -> SDWORD {
    let mut retval: SDWORD = 0;
    let mut diff: SDWORD = 0;
    diff = end - start;
    if diff > 0 as libc::c_int {
        if diff < 180 as libc::c_int {
            retval = diff
        } else { retval = diff - 360 as libc::c_int }
    } else if diff > -(180 as libc::c_int) {
        retval = diff
    } else { retval = 360 as libc::c_int + diff }
    return retval;
}
// initialise a droid for a shuffle move
/*void moveShuffleInit(DROID *psDroid, SDWORD mx, SDWORD my)
{
	// set up the move state
	psDroid->sMove.Status = MOVESHUFFLE;
	psDroid->sMove.shuffleX = (SWORD)mx;
	psDroid->sMove.shuffleY = (SWORD)my;
	psDroid->sMove.srcX = (SDWORD)psDroid->x;
	psDroid->sMove.srcY = (SDWORD)psDroid->y;
	psDroid->sMove.targetX = (SDWORD)psDroid->x + mx;
	psDroid->sMove.targetY = (SDWORD)psDroid->y + my;
	psDroid->sMove.DestinationX = psDroid->sMove.targetX;
	psDroid->sMove.DestinationY = psDroid->sMove.targetY;
	psDroid->sMove.numPoints = 0;
	psDroid->sMove.Position = 0;
	psDroid->sMove.fx = MAKEFRACT(psDroid->x);
	psDroid->sMove.fy = MAKEFRACT(psDroid->y);
	psDroid->sMove.bumpTime = 0;
	moveCalcBoundary(psDroid);

	if (psDroid->sMove.psFormation != NULL)
	{
		formationLeave(psDroid->sMove.psFormation, (BASE_OBJECT *)psDroid);
		psDroid->sMove.psFormation = NULL;
	}
}*/
// Tell a droid to move out the way for a shuffle
#[no_mangle]
pub unsafe extern "C" fn moveShuffleDroid(mut psDroid: *mut DROID,
                                          mut shuffleStart: UDWORD,
                                          mut sx: SDWORD, mut sy: SDWORD) {
    let mut shuffleDir: FRACT = 0.;
    let mut droidDir: FRACT = 0.;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut mx: SDWORD = 0;
    let mut my: SDWORD = 0;
    let mut shuffleMag: SDWORD = 0;
    let mut diff: SDWORD = 0;
    let mut frontClear: BOOL = 1 as libc::c_int;
    let mut leftClear: BOOL = 1 as libc::c_int;
    let mut rightClear: BOOL = 1 as libc::c_int;
    let mut lvx: SDWORD = 0;
    let mut lvy: SDWORD = 0;
    let mut rvx: SDWORD = 0;
    let mut rvy: SDWORD = 0;
    let mut svx: SDWORD = 0;
    let mut svy: SDWORD = 0;
    let mut shuffleMove: SDWORD = 0;
    let mut tarX: SDWORD = 0;
    let mut tarY: SDWORD = 0;
    shuffleDir = vectorToAngle(sx as FRACT, sy as FRACT);
    shuffleMag =
        sqrt((sx * sx + sy * sy) as libc::c_double) as FRACT as SDWORD;
    if shuffleMag == 0 as libc::c_int { return }
    shuffleMove = 2 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int;
    /*	if (vtolDroid(psDroid))
	{
		shuffleMove /= 4;
	}*/
    // calculate the possible movement vectors
    lvx = -sy * shuffleMove / shuffleMag; // sx * SHUFFLE_MOVE / shuffleMag;
    lvy = sx * shuffleMove / shuffleMag; // sy * SHUFFLE_MOVE / shuffleMag;
    rvx = sy * shuffleMove / shuffleMag;
    rvy = -sx * shuffleMove / shuffleMag;
    svx = lvy;
    svy = rvx;
    // check for blocking tiles
    if fpathBlockingTile.expect("non-null function pointer")((*psDroid).x as
                                                                 SDWORD + lvx
                                                                 >>
                                                                 7 as
                                                                     libc::c_int,
                                                             (*psDroid).y as
                                                                 SDWORD + lvy
                                                                 >>
                                                                 7 as
                                                                     libc::c_int)
           != 0 {
        leftClear = 0 as libc::c_int
    } else if fpathBlockingTile.expect("non-null function pointer")((*psDroid).x
                                                                        as
                                                                        SDWORD
                                                                        + rvx
                                                                        >>
                                                                        7 as
                                                                            libc::c_int,
                                                                    (*psDroid).y
                                                                        as
                                                                        SDWORD
                                                                        + rvy
                                                                        >>
                                                                        7 as
                                                                            libc::c_int)
                  != 0 {
        rightClear = 0 as libc::c_int
    } else if fpathBlockingTile.expect("non-null function pointer")((*psDroid).x
                                                                        as
                                                                        SDWORD
                                                                        + svx
                                                                        >>
                                                                        7 as
                                                                            libc::c_int,
                                                                    (*psDroid).y
                                                                        as
                                                                        SDWORD
                                                                        + svy
                                                                        >>
                                                                        7 as
                                                                            libc::c_int)
                  != 0 {
        frontClear = 0 as libc::c_int
    }
    // find any droids that could block the shuffle
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if psCurr != psDroid {
            xdiff = (*psCurr).x as SDWORD - (*psDroid).x as SDWORD;
            ydiff = (*psCurr).y as SDWORD - (*psDroid).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   3 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int *
                       (3 as libc::c_int * 128 as libc::c_int /
                            2 as libc::c_int) {
                droidDir = vectorToAngle(xdiff as FRACT, ydiff as FRACT);
                diff = moveDirDiff(shuffleDir as SDWORD, droidDir as SDWORD);
                if diff > -(135 as libc::c_int) && diff < -(45 as libc::c_int)
                   {
                    leftClear = 0 as libc::c_int
                } else if diff > 45 as libc::c_int &&
                              diff < 135 as libc::c_int {
                    rightClear = 0 as libc::c_int
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    // calculate a target
    if leftClear != 0 {
        mx = lvx;
        my = lvy
    } else if rightClear != 0 {
        mx = rvx;
        my = rvy
    } else if frontClear != 0 {
        mx = svx;
        my = svy
    } else {
        // nowhere to shuffle to, quit
        return
    }
    // check the location for vtols
    tarX = (*psDroid).x as SDWORD + mx;
    tarY = (*psDroid).y as SDWORD + my;
    if vtolDroid(psDroid) != 0 {
        actionVTOLLandingPos(psDroid, &mut tarX as *mut SDWORD as *mut UDWORD,
                             &mut tarY as *mut SDWORD as *mut UDWORD);
    }
    // set up the move state
    if (*psDroid).sMove.Status as libc::c_int == 7 as libc::c_int {
        (*psDroid).sMove.Status = 13 as libc::c_int as UBYTE
    } else { (*psDroid).sMove.Status = 12 as libc::c_int as UBYTE }
    //	psDroid->sMove.shuffleX = (SWORD)sx;
//	psDroid->sMove.shuffleY = (SWORD)sy;
    (*psDroid).sMove.shuffleStart = shuffleStart;
    (*psDroid).sMove.srcX = (*psDroid).x as SDWORD;
    (*psDroid).sMove.srcY = (*psDroid).y as SDWORD;
    //	psDroid->sMove.targetX = (SDWORD)psDroid->x + mx;
//	psDroid->sMove.targetY = (SDWORD)psDroid->y + my;
    (*psDroid).sMove.targetX = tarX;
    (*psDroid).sMove.targetY = tarY;
    // setting the Destination could overwrite a MOVEROUTE's destination
	// it is not actually needed for a shuffle anyway
//	psDroid->sMove.DestinationX = psDroid->sMove.targetX;
//	psDroid->sMove.DestinationY = psDroid->sMove.targetY;
//	psDroid->sMove.bumpTime = 0;
    (*psDroid).sMove.numPoints = 0 as libc::c_int as UBYTE;
    (*psDroid).sMove.Position = 0 as libc::c_int as UBYTE;
    (*psDroid).sMove.fx = (*psDroid).x as FRACT;
    (*psDroid).sMove.fy = (*psDroid).y as FRACT;
    (*psDroid).sMove.fz = (*psDroid).z as FRACT;
    moveCalcBoundary(psDroid);
    if !(*psDroid).sMove.psFormation.is_null() {
        formationLeave((*psDroid).sMove.psFormation,
                       psDroid as *mut BASE_OBJECT);
        (*psDroid).sMove.psFormation = 0 as *mut _formation
    };
}
/* Stop a droid */
#[no_mangle]
pub unsafe extern "C" fn moveStopDroid(mut psDroid: *mut DROID) {
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveStopUnit: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              801 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"moveStopDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              805 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"moveStopDroid\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int {
        (*psDroid).sMove.Status = 8 as libc::c_int as UBYTE
    } else { (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE };
}
/*Stops a droid dead in its tracks - doesn't allow for any little skidding bits*/
#[no_mangle]
pub unsafe extern "C" fn moveReallyStopDroid(mut psDroid: *mut DROID) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveReallyStopUnit: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              821 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"moveReallyStopDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE;
    (*psDroid).sMove.speed = 0 as libc::c_int as FRACT;
}
/* Get pitch and roll from direction and tile data - NOT VERY PSX FRIENDLY */
#[no_mangle]
pub unsafe extern "C" fn updateDroidOrientation(mut psDroid: *mut DROID) {
    let mut hx0: SDWORD = 0;
    let mut hx1: SDWORD = 0;
    let mut hy0: SDWORD = 0;
    let mut hy1: SDWORD = 0;
    let mut w: SDWORD = 0;
    let mut newPitch: SDWORD = 0;
    let mut dPitch: SDWORD = 0;
    let mut pitchLimit: SDWORD = 0;
    let mut dx: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut direction: libc::c_double = 0.;
    let mut pitch: libc::c_double = 0.;
    let mut roll: libc::c_double = 0.;
    if ((*psDroid).x as libc::c_uint) < mapWidth << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: x coordinate bigger than map width\x00" as
                  *const u8 as *const libc::c_char);
    };
    if ((*psDroid).x as libc::c_uint) < mapWidth << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              839 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"updateDroidOrientation\x00")).as_ptr(),
              b"psDroid->x < (mapWidth << TILE_SHIFT)\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroid).y as libc::c_uint) < mapHeight << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"mapHeight: y coordinate bigger than map height\x00" as
                  *const u8 as *const libc::c_char);
    };
    if ((*psDroid).y as libc::c_uint) < mapHeight << 7 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              841 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"updateDroidOrientation\x00")).as_ptr(),
              b"psDroid->y < (mapHeight<< TILE_SHIFT)\x00" as *const u8 as
                  *const libc::c_char);
    };
    //if(psDroid->droidType == DROID_PERSON OR psDroid->droidType == DROID_CYBORG OR
    if (*psDroid).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint ||
           cyborgDroid(psDroid) != 0 ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        /* These guys always stand upright */
        return
    }
    w = 20 as libc::c_int;
    hx0 =
        map_Height(((*psDroid).x as libc::c_int + w) as UDWORD,
                   (*psDroid).y as UDWORD) as SDWORD;
    hx1 =
        map_Height(((*psDroid).x as libc::c_int - w) as UDWORD,
                   (*psDroid).y as UDWORD) as SDWORD;
    hy0 =
        map_Height((*psDroid).x as UDWORD,
                   ((*psDroid).y as libc::c_int + w) as UDWORD) as SDWORD;
    hy1 =
        map_Height((*psDroid).x as UDWORD,
                   ((*psDroid).y as libc::c_int - w) as UDWORD) as SDWORD;
    //update height in case were in the bottom of a trough
    if (hx0 + hx1) / 2 as libc::c_int > (*psDroid).z as SDWORD {
        (*psDroid).z = ((hx0 + hx1) / 2 as libc::c_int) as UWORD
    }
    if (hy0 + hy1) / 2 as libc::c_int > (*psDroid).z as SDWORD {
        (*psDroid).z = ((hy0 + hy1) / 2 as libc::c_int) as UWORD
    }
    dx = (hx0 - hx1) as libc::c_double / (w as libc::c_double * 2.0f64);
    dy = (hy0 - hy1) as libc::c_double / (w as libc::c_double * 2.0f64);
    //dx is atan of angle of elevation along x axis
	//dx is atan of angle of elevation along y axis
	//body
    direction =
        3.141592654f64 * (*psDroid).direction as libc::c_int as libc::c_double
            / 180.0f64;
    pitch = sin(direction) * dx + cos(direction) * dy;
    pitch = atan(pitch);
    newPitch =
        (pitch * 180 as libc::c_int as libc::c_double / 3.141592654f64) as
            SDWORD;
    //limit the rate the front comes down to simulate momentum
    pitchLimit =
        (150 as libc::c_int as
             libc::c_uint).wrapping_mul(frameTime).wrapping_div(1000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
            as SDWORD;
    dPitch = newPitch - (*psDroid).pitch as libc::c_int;
    if dPitch < 0 as libc::c_int { dPitch += 360 as libc::c_int }
    if dPitch > 180 as libc::c_int && dPitch < 360 as libc::c_int - pitchLimit
       {
        (*psDroid).pitch =
            ((*psDroid).pitch as libc::c_int - pitchLimit) as SWORD
    } else { (*psDroid).pitch = newPitch as SWORD }
    roll = cos(direction) * dx - sin(direction) * dy;
    roll = atan(roll);
    (*psDroid).roll =
        (roll * 180 as libc::c_int as libc::c_double / 3.141592654f64) as
            UWORD as SWORD;
}
/* Turn a vector into an angle - returns a FRACT (!) */
/* Calculate the normalised vector between a droid and a point */
/*void moveCalcVector(DROID *psDroid, UDWORD x, UDWORD y, FRACT *pVX, FRACT *pVY)
{
	SDWORD	dx,dy, mag;
	FRACT	root;

	// Calc the basic vector
	dx = (SDWORD)x - (SDWORD)psDroid->x;
	dy = (SDWORD)y - (SDWORD)psDroid->y;

	// normalise
	mag = dx*dx + dy*dy;
	root = fSQRT(MAKEFRACT(mag));
	*pVX = FRACTdiv(dx, root);
	*pVY = FRACTdiv(dy, root);
}*/
/* Turn a vector into an angle - returns a FRACT (!) */
unsafe extern "C" fn vectorToAngle(mut vx: FRACT, mut vy: FRACT) -> FRACT {
    let mut angle: FRACT = 0.; // Angle in degrees (0->360)
    angle =
        (360 as libc::c_int as libc::c_double *
             atan2(-vy as libc::c_double, vx as libc::c_double) /
             3.141592654f64 / 2 as libc::c_int as libc::c_double) as
            libc::c_float;
    angle += (360 as libc::c_int / 4 as libc::c_int) as libc::c_float;
    if angle < 0 as libc::c_int as libc::c_float {
        angle += 360 as libc::c_int as libc::c_float
    }
    if angle >= 360.0f32 { angle -= 360.0f32 }
    return angle;
}
/* Turn an angle into a vector */
unsafe extern "C" fn angleToVector(mut angle: SDWORD, mut pX: *mut FRACT,
                                   mut pY: *mut FRACT) {
    *pX = trigSin(angle);
    *pY = trigCos(angle);
}
/* Calculate the change in direction given a target angle and turn rate */
unsafe extern "C" fn moveCalcTurn(mut pCurr: *mut FRACT, mut target: FRACT,
                                  mut rate: UDWORD) {
    let mut diff: FRACT = 0.;
    let mut change: FRACT = 0.;
    //Ugh.  If your gonna ONLY use this variable in "DEBUG", then
    if target < 360 as libc::c_int as FRACT &&
           target >= 0 as libc::c_int as FRACT {
    } else {
        debug(LOG_ERROR,
              b"moveCalcTurn: target out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if target < 360 as libc::c_int as FRACT &&
           target >= 0 as libc::c_int as FRACT {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              971 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"moveCalcTurn\x00")).as_ptr(),
              b"target < MAKEFRACT(360) && target >= MAKEFRACT(0)\x00" as
                  *const u8 as *const libc::c_char);
    };
    if *pCurr < 360 as libc::c_int as FRACT &&
           *pCurr >= 0 as libc::c_int as FRACT {
    } else {
        debug(LOG_ERROR,
              b"moveCalcTurn: cur ang out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if *pCurr < 360 as libc::c_int as FRACT &&
           *pCurr >= 0 as libc::c_int as FRACT {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              974 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"moveCalcTurn\x00")).as_ptr(),
              b"(*pCurr) < MAKEFRACT(360) && (*pCurr) >= MAKEFRACT(0)\x00" as
                  *const u8 as *const libc::c_char);
    };
    // calculate the difference in the angles
    diff = target - *pCurr;
    // calculate the change in direction
    //	change = FRACTmul( baseTurn, MAKEFRACT(rate) );
    change =
        baseTurn *
            rate as
                libc::c_float; // constant rate so we can use a normal mult
    if change <
           1 as libc::c_int as libc::c_float /
               1 as libc::c_int as libc::c_float {
        change =
            1 as libc::c_int as libc::c_float /
                1 as libc::c_int as
                    libc::c_float // that integer angle to turn per frame is less than 1
        // HACK to solve issue of when framerate so high
    }
    if diff >= 0 as libc::c_int as libc::c_float && diff < change ||
           diff < 0 as libc::c_int as libc::c_float && diff > -change {
        // got to the target direction
        *pCurr = target
    } else if diff > 0 as libc::c_int as libc::c_float {
        // Target dir is greater than current
        if diff < (360 as libc::c_int / 2 as libc::c_int) as FRACT {
            // Simple case - just increase towards target
            *pCurr += change
        } else {
            // decrease to target, but could go over 0 boundary */
            *pCurr -= change
        }
    } else if diff > -(360 as libc::c_int / 2 as libc::c_int) as FRACT {
        // Simple case - just decrease towards target
        *pCurr -= change
    } else {
        // increase to target, but could go over 0 boundary
        *pCurr += change
    }
    if *pCurr < 0 as libc::c_int as libc::c_float {
        *pCurr += 360 as libc::c_int as FRACT
    }
    if *pCurr >= 360 as libc::c_int as FRACT {
        *pCurr -= 360 as libc::c_int as FRACT
    };
    //Don't forget that if you don't define the variable, then we error out.
}
/* Get the next target point from the route */
unsafe extern "C" fn moveNextTarget(mut psDroid: *mut DROID) -> BOOL {
    let mut srcX: UDWORD = 0;
    let mut srcY: UDWORD = 0;
    let mut tarX: UDWORD = 0;
    let mut tarY: UDWORD = 0;
    // See if there is anything left in the move list
//	if (psDroid->sMove.MovementList[psDroid->sMove.Position].XCoordinate == -1)
    if (*psDroid).sMove.Position as libc::c_int ==
           (*psDroid).sMove.numPoints as libc::c_int {
        return 0 as libc::c_int
    }
    /*	tarX = (psDroid->sMove.MovementList[psDroid->sMove.Position].XCoordinate
				<< TILE_SHIFT) + TILE_UNITS/2;
	tarY = (psDroid->sMove.MovementList[psDroid->sMove.Position].YCoordinate
				<< TILE_SHIFT) + TILE_UNITS/2;*/
    tarX =
        ((((*psDroid).sMove.asPath[(*psDroid).sMove.Position as usize].x as
               libc::c_int) << 7 as libc::c_int) +
             128 as libc::c_int / 2 as libc::c_int) as UDWORD;
    tarY =
        ((((*psDroid).sMove.asPath[(*psDroid).sMove.Position as usize].y as
               libc::c_int) << 7 as libc::c_int) +
             128 as libc::c_int / 2 as libc::c_int) as UDWORD;
    if (*psDroid).sMove.Position as libc::c_int == 0 as libc::c_int {
        (*psDroid).sMove.srcX = (*psDroid).x as SDWORD;
        (*psDroid).sMove.srcY = (*psDroid).y as SDWORD
    } else {
        srcX =
            ((((*psDroid).sMove.asPath[((*psDroid).sMove.Position as
                                            libc::c_int - 1 as libc::c_int) as
                                           usize].x as libc::c_int) <<
                  7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int)
                as UDWORD;
        srcY =
            ((((*psDroid).sMove.asPath[((*psDroid).sMove.Position as
                                            libc::c_int - 1 as libc::c_int) as
                                           usize].y as libc::c_int) <<
                  7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int)
                as UDWORD;
        (*psDroid).sMove.srcX = srcX as SDWORD;
        (*psDroid).sMove.srcY = srcY as SDWORD
    }
    (*psDroid).sMove.targetX = tarX as SDWORD;
    (*psDroid).sMove.targetY = tarY as SDWORD;
    (*psDroid).sMove.Position = (*psDroid).sMove.Position.wrapping_add(1);
    return 1 as libc::c_int;
}
/* Look at the next target point from the route */
unsafe extern "C" fn movePeekNextTarget(mut psDroid: *mut DROID,
                                        mut pX: *mut SDWORD,
                                        mut pY: *mut SDWORD) {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    // See if there is anything left in the move list
    if (*psDroid).sMove.Position as libc::c_int ==
           (*psDroid).sMove.numPoints as libc::c_int {
        // No points left - fudge one to continue the same direction
        xdiff = (*psDroid).sMove.targetX - (*psDroid).sMove.srcX;
        ydiff = (*psDroid).sMove.targetY - (*psDroid).sMove.srcY;
        *pX = (*psDroid).sMove.targetX + xdiff;
        *pY = (*psDroid).sMove.targetY + ydiff
    } else {
        /*		*pX = (psDroid->sMove.MovementList[psDroid->sMove.Position].XCoordinate
					<< TILE_SHIFT) + TILE_UNITS/2;
		*pY = (psDroid->sMove.MovementList[psDroid->sMove.Position].YCoordinate
					<< TILE_SHIFT) + TILE_UNITS/2;*/
        *pX =
            (((*psDroid).sMove.asPath[(*psDroid).sMove.Position as usize].x as
                  libc::c_int) << 7 as libc::c_int) +
                128 as libc::c_int / 2 as libc::c_int;
        *pY =
            (((*psDroid).sMove.asPath[(*psDroid).sMove.Position as usize].y as
                  libc::c_int) << 7 as libc::c_int) +
                128 as libc::c_int / 2 as libc::c_int
    };
}
// hack to get the box in the 2D display
//	QUAD	sBox;
/* Get a direction vector to avoid anything in front of a droid */
/*void moveObstacleVector(DROID *psDroid, FRACT *pX, FRACT *pY)
{
	FRACT	wsin,wcos, lsin,lcos, x,y;
	SDWORD	dir, diff, obstDir, target;
	UDWORD	distSqr, i;
	POINT	sPos;
	BASE_OBJECT		*psObst;

	// Calculate the obstacle box
	dir = (SDWORD)psDroid->direction;
	wsin = FRACTmul(MAKEFRACT(HITBOX_WIDTH/2), trigSin(dir));
	wcos = FRACTmul(MAKEFRACT(HITBOX_WIDTH/2), trigCos(dir));
	lsin = FRACTmul(MAKEFRACT(HITBOX_LENGTH), trigSin(dir));
	lcos = FRACTmul(MAKEFRACT(HITBOX_LENGTH), trigCos(dir));

	x = MAKEFRACT(psDroid->x);
	y = MAKEFRACT(psDroid->y);
	sBox.coords[0].x = MAKEINT(x - wcos);
	sBox.coords[0].y = MAKEINT(y + wsin);
	sBox.coords[1].x = MAKEINT(x + wcos);
	sBox.coords[1].y = MAKEINT(y - wsin);
	sBox.coords[3].x = MAKEINT(MAKEFRACT(sBox.coords[0].x) + lsin);
	sBox.coords[3].y = MAKEINT(MAKEFRACT(sBox.coords[0].y) + lcos);
	sBox.coords[2].x = MAKEINT(MAKEFRACT(sBox.coords[1].x) + lsin);
	sBox.coords[2].y = MAKEINT(MAKEFRACT(sBox.coords[1].y) + lcos);

	// Find the nearest obstacle in the box
	distSqr = UDWORD_MAX;
	psObst = NULL;
	for(i=0; i<numNaybors; i++)
	{
		sPos.x = (SDWORD)asDroidNaybors[i].psObj->x;
		sPos.y = (SDWORD)asDroidNaybors[i].psObj->y;
		if (inQuad(&sPos, &sBox) && asDroidNaybors[i].distSqr < distSqr)
		{
			psObst = asDroidNaybors[i].psObj;
			distSqr = asDroidNaybors[i].distSqr;
		}
	}

	if (psObst)
	{
//		moveCalcVector(psDroid, psObst->x, psObst->y, pX,pY);
		// got an obstacle - turn the droid away from it
		obstDir = (SDWORD)calcDirection(psDroid->x, psDroid->y, psObst->x,psObst->y);
		diff = directionDiff(obstDir, dir);
		ASSERT( diff > -90 && diff < 90, "big diff" );
		if (diff < 0)
		{
			target = dir + HITBOX_ANGLE + diff;
		}
		else
		{
			target = dir - HITBOX_ANGLE + diff;
		}

		angleToVector(MAKEFRACT(target), pX,pY);
	}
	else
	{
		// no obstacle - no control vector
		*pX = MAKEFRACT(0);
		*pY = MAKEFRACT(0);
	}
}*/
static mut mvPersRad: libc::c_int = 20 as libc::c_int;
static mut mvCybRad: libc::c_int = 30 as libc::c_int;
static mut mvSmRad: libc::c_int = 40 as libc::c_int;
static mut mvMedRad: libc::c_int = 50 as libc::c_int;
static mut mvLgRad: libc::c_int = 60 as libc::c_int;
// Get the radius of a base object for collision
unsafe extern "C" fn moveObjRadius(mut psObj: *mut BASE_OBJECT) -> SDWORD {
    let mut radius: SDWORD = 0;
    let mut psBdyStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            if (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint {
                radius = mvPersRad
            } else if cyborgDroid(psObj as *mut DROID) != 0 {
                radius = mvCybRad
            } else {
                radius = (*(*psObj).sDisplay.imd).radius;
                psBdyStats =
                    asBodyStats.offset((*(psObj as
                                              *mut DROID)).asBits[COMP_BODY as
                                                                      libc::c_int
                                                                      as
                                                                      usize].nStat
                                           as libc::c_int as isize);
                match (*psBdyStats).size as libc::c_int {
                    1 => { radius = mvMedRad }
                    2 => { radius = mvLgRad }
                    3 => { radius = 130 as libc::c_int }
                    0 | _ => { radius = mvSmRad }
                }
            }
        }
        1 => {
            //else if ( ((DROID *)psObj)->droidType == DROID_CYBORG )
            //		radius = psObj->sDisplay.imd->visRadius;
            radius = (*(*psObj).sDisplay.imd).radius / 2 as libc::c_int
        }
        2 => {
            //		radius = psObj->sDisplay.imd->visRadius;
            radius = (*(*psObj).sDisplay.imd).radius / 2 as libc::c_int
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"moveObjRadius: unknown object type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      1240 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"moveObjRadius\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            radius = 0 as libc::c_int
        }
    }
    return radius;
}
// Find any object in the naybors list that the droid has hit
/*BOOL moveFindObstacle(DROID *psDroid, FRACT mx, FRACT my, BASE_OBJECT **ppsObst)
{
	SDWORD		i, droidR, rad, radSq;
	SDWORD		objR;
	FRACT		xdiff,ydiff, distSq;
	NAYBOR_INFO	*psInfo;
	SDWORD		distSq1;


	droidR = moveObjRadius((BASE_OBJECT *)psDroid);

	droidGetNaybors(psDroid);

	for(i=0; i<(SDWORD)numNaybors; i++)
	{
		psInfo = asDroidNaybors + i;
		if (psInfo->psObj->type == OBJ_DROID &&
			((DROID *)psInfo->psObj)->droidType != DROID_PERSON)
		{
			// ignore droids
			continue;
		}
		objR = moveObjRadius(psInfo->psObj);
		rad = droidR + objR;
		radSq = rad*rad;

		xdiff = MAKEFRACT(psDroid->x) + mx - MAKEFRACT(psInfo->psObj->x);
		ydiff = MAKEFRACT(psDroid->y) + my - MAKEFRACT(psInfo->psObj->y);
#ifndef PSX

		distSq = FRACTmul(xdiff,xdiff) + FRACTmul(ydiff,ydiff);
		distSq1 = MAKEINT(distSq);
#else
		x1 = MAKEINT (xdiff);
		y1 = MAKEINT (ydiff);
		distSq1 = (x1*x1)+(y1*y1);
#endif

		if (radSq > (distSq1))
		{
			if ((psDroid->droidType != DROID_PERSON) &&
				(psInfo->psObj->type == OBJ_DROID) &&
				((DROID *)psInfo->psObj)->droidType == DROID_PERSON &&
				psDroid->player != psInfo->psObj->player)
			{	// run over a bloke - kill him

				destroyDroid((DROID*)psInfo->psObj);
				continue;
			}
			else
			{
				*ppsObst = psInfo->psObj;
				// note the bump time and position if necessary
				if (psDroid->sMove.bumpTime == 0)
				{
					psDroid->sMove.bumpTime = gameTime;
					psDroid->sMove.bumpX = psDroid->x;
					psDroid->sMove.bumpY = psDroid->y;
					psDroid->sMove.bumpDir = (SDWORD)psDroid->direction;
				}
				return TRUE;
			}
		}
		else if (psInfo->distSqr > OBJ_MAXRADIUS*OBJ_MAXRADIUS)
		{
			// object is too far away to be hit
			break;
		}
	}

	return FALSE;
}*/
// see if a Droid has run over a person
#[no_mangle]
pub unsafe extern "C" fn moveCheckSquished(mut psDroid: *mut DROID,
                                           mut mx: FRACT, mut my: FRACT) {
    let mut i: SDWORD = 0;
    let mut droidR: SDWORD = 0;
    let mut rad: SDWORD = 0;
    let mut radSq: SDWORD = 0;
    let mut objR: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut psInfo: *mut NAYBOR_INFO = 0 as *mut NAYBOR_INFO;
    droidR = moveObjRadius(psDroid as *mut BASE_OBJECT);
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        psInfo = asDroidNaybors.as_mut_ptr().offset(i as isize);
        if !((*(*psInfo).psObj).type_0 as libc::c_uint !=
                 OBJ_DROID as libc::c_int as libc::c_uint ||
                 (*((*psInfo).psObj as *mut DROID)).droidType as libc::c_uint
                     != DROID_PERSON as libc::c_int as libc::c_uint) {
            if (*(*psInfo).psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*((*psInfo).psObj as *mut DROID)).droidType as
                       libc::c_uint ==
                       DROID_PERSON as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"squished - eerk\x00" as *const u8 as
                          *const libc::c_char);
            };
            if (*(*psInfo).psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint &&
                   (*((*psInfo).psObj as *mut DROID)).droidType as
                       libc::c_uint ==
                       DROID_PERSON as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      1346 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"moveCheckSquished\x00")).as_ptr(),
                      b"psInfo->psObj->type == OBJ_DROID && ((DROID *)psInfo->psObj)->droidType == DROID_PERSON\x00"
                          as *const u8 as *const libc::c_char);
            };
            objR = moveObjRadius((*psInfo).psObj);
            rad = droidR + objR;
            radSq = rad * rad;
            xdiff =
                (*psDroid).x as SDWORD + mx as SDWORD -
                    (*(*psInfo).psObj).x as SDWORD;
            ydiff =
                (*psDroid).y as SDWORD + my as SDWORD -
                    (*(*psInfo).psObj).y as SDWORD;
            distSq = xdiff * xdiff + ydiff * ydiff;
            if 2 as libc::c_int * radSq / 3 as libc::c_int > distSq {
                if (*psDroid).player as libc::c_int !=
                       (*(*psInfo).psObj).player as libc::c_int &&
                       aiCheckAlliances((*psDroid).player as UDWORD,
                                        (*(*psInfo).psObj).player as UDWORD)
                           == 0 {
                    // run over a bloke - kill him
                    destroyDroid((*psInfo).psObj as *mut DROID);
                    scoreUpdateVar(WD_BARBARIANS_MOWED_DOWN);
                }
            } else if (*psInfo).distSqr >
                          (128 as libc::c_int * 4 as libc::c_int *
                               (128 as libc::c_int * 4 as libc::c_int)) as
                              libc::c_uint {
                break ;
            }
        }
        // ignore everything but people
        i += 1
    };
}
// See if the droid has been stopped long enough to give up on the move
#[no_mangle]
pub unsafe extern "C" fn moveBlocked(mut psDroid: *mut DROID) -> BOOL {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut diffSq: SDWORD = 0;
    let mut blockTime: UDWORD = 0;
    if (*psDroid).sMove.bumpTime == 0 as libc::c_int as libc::c_uint ||
           (*psDroid).sMove.bumpTime > gameTime ||
           (*psDroid).sMove.Status as libc::c_int == 7 as libc::c_int ||
           (*psDroid).sMove.Status as libc::c_int == 13 as libc::c_int {
        // no bump - can't be blocked
        return 0 as libc::c_int
    }
    // See if the block can be cancelled
    if dirDiff((*psDroid).direction as SDWORD,
               (*psDroid).sMove.bumpDir as SDWORD) >
           90 as libc::c_int as libc::c_uint {
        // Move on, clear the bump
        (*psDroid).sMove.bumpTime = 0 as libc::c_int as UDWORD;
        (*psDroid).sMove.lastBump = 0 as libc::c_int as UWORD;
        return 0 as libc::c_int
    }
    xdiff = (*psDroid).x as SDWORD - (*psDroid).sMove.bumpX as SDWORD;
    ydiff = (*psDroid).y as SDWORD - (*psDroid).sMove.bumpY as SDWORD;
    diffSq = xdiff * xdiff + ydiff * ydiff;
    if diffSq > 64 as libc::c_int * 64 as libc::c_int {
        // Move on, clear the bump
        (*psDroid).sMove.bumpTime = 0 as libc::c_int as UDWORD;
        (*psDroid).sMove.lastBump = 0 as libc::c_int as UWORD;
        return 0 as libc::c_int
    }
    if (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int {
        blockTime = 2000 as libc::c_int as UDWORD
    } else { blockTime = 6000 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub((*psDroid).sMove.bumpTime) > blockTime {
        // Stopped long enough - blocked
        (*psDroid).sMove.bumpTime = 0 as libc::c_int as UDWORD;
        (*psDroid).sMove.lastBump = 0 as libc::c_int as UWORD;
        // if the unit cannot see the next way point - reroute it's got stuck
        if (bMultiPlayer != 0 ||
                (*psDroid).player as libc::c_uint == selectedPlayer) &&
               (*psDroid).sMove.Position as libc::c_int !=
                   (*psDroid).sMove.numPoints as libc::c_int &&
               fpathTileLOS((*psDroid).x as SDWORD >> 7 as libc::c_int,
                            (*psDroid).y as SDWORD >> 7 as libc::c_int,
                            (*psDroid).sMove.DestinationX >> 7 as libc::c_int,
                            (*psDroid).sMove.DestinationY >> 7 as libc::c_int)
                   == 0 {
            moveDroidTo(psDroid, (*psDroid).sMove.DestinationX as UDWORD,
                        (*psDroid).sMove.DestinationY as UDWORD);
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// See if an object is on a droids target
#[no_mangle]
pub unsafe extern "C" fn moveObjOnTarget(mut psDroid: *mut DROID,
                                         mut psObst: *mut BASE_OBJECT)
 -> BOOL {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut radius: SDWORD = 0;
    /*	xdiff = (SDWORD)psObst->x - (psDroid->sMove.DestinationX << TILE_SHIFT) -
				TILE_UNITS/2;
	ydiff = (SDWORD)psObst->y - (psDroid->sMove.DestinationY << TILE_SHIFT) -
				TILE_UNITS/2;*/
    xdiff = (*psObst).x as SDWORD - (*psDroid).sMove.DestinationX;
    ydiff = (*psObst).y as SDWORD - (*psDroid).sMove.DestinationY;
    radius = moveObjRadius(psObst) * 2 as libc::c_int;
    if xdiff * xdiff + ydiff * ydiff < radius * radius {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
    /*	SDWORD	absX, absY;
	SDWORD	tarX, tarY, tarMag;
	SDWORD	obstX, obstY, obstMag;
	FRACT	distRatio, dot;

	// Calculate the vectors to the target and obstruction
	tarX = (SDWORD)psDroid->x - psDroid->sMove.targetX;
	tarY = (SDWORD)psDroid->y - psDroid->sMove.targetY;
	absX = labs(tarX); absY = labs(tarY);
	tarMag = absX > absY ? absX + absY/2 : absY + absX/2;
	obstX = (SDWORD)psDroid->x - (SDWORD)psObst->x;
	obstY = (SDWORD)psDroid->y - (SDWORD)psObst->y;
	absX = labs(obstX); absY = labs(obstY);
	obstMag = absX > absY ? absX + absY/2 : absY + absX/2;

	// If the difference in distance is too big, obstruction cannot be
	// on target
	distRatio = FRACTdiv(MAKEFRACT(obstMag), MAKEFRACT(tarMag));
	if (distRatio < BLOCK_MINRATIO || distRatio > BLOCK_MAXRATIO)
	{
		return FALSE;
	}

	// Do the dot product to see if the vectors are the same direction
	dot = FRACTmul(FRACTdiv(MAKEFRACT(tarX), MAKEFRACT(tarMag)),
				   FRACTdiv(MAKEFRACT(obstX), MAKEFRACT(obstMag))) +
		  FRACTmul(FRACTdiv(MAKEFRACT(tarY), MAKEFRACT(tarMag)),
				   FRACTdiv(MAKEFRACT(obstY), MAKEFRACT(obstMag)));
	if (dot > BLOCK_DOTVAL)
	{
		return TRUE;
	}

	return FALSE;*/
}
// Calculate the actual movement to slide around
#[no_mangle]
pub unsafe extern "C" fn moveCalcSlideVector(mut psDroid: *mut DROID,
                                             mut objX: SDWORD,
                                             mut objY: SDWORD,
                                             mut pMx: *mut FRACT,
                                             mut pMy: *mut FRACT) {
    let mut obstX: SDWORD = 0;
    let mut obstY: SDWORD = 0;
    let mut absX: SDWORD = 0;
    let mut absY: SDWORD = 0;
    let mut dirX: SDWORD = 0;
    let mut dirY: SDWORD = 0;
    let mut dirMag: SDWORD = 0;
    let mut mx: FRACT = 0.;
    let mut my: FRACT = 0.;
    let mut unitX: FRACT = 0.;
    let mut unitY: FRACT = 0.;
    let mut dotRes: FRACT = 0.;
    mx = *pMx;
    my = *pMy;
    // Calculate the vector to the obstruction
    obstX = (*psDroid).x as SDWORD - objX;
    obstY = (*psDroid).y as SDWORD - objY;
    // if the target dir is the same, don't need to slide
    if obstX as libc::c_float * mx + obstY as libc::c_float * my >=
           0 as libc::c_int as libc::c_float {
        return
    }
    // Choose the tangent vector to this on the same side as the target
//	tarX = psDriod->sMove.targetX - (SDWORD)psDroid->x;
//	tarY = psDriod->sMove.targetY - (SDWORD)psDroid->y;
//	dotRes = FRACTmul(MAKEFRACT(obstY),*pMx);
//	dotRes -= FRACTmul(MAKEFRACT(obstX),*pMy);
//	dotRes = obstY * mx - obstX * my;
    dotRes = obstY as FRACT * mx - obstX as FRACT * my;
    if dotRes >= 0 as libc::c_int as libc::c_float {
        dirX = obstY;
        dirY = -obstX
    } else {
        dirX = -obstY;
        dirY = obstX;
        dotRes = dirX as FRACT * *pMx + dirY as FRACT * *pMy
    }
    absX = labs(dirX as libc::c_long) as SDWORD;
    absY = labs(dirY as libc::c_long) as SDWORD;
    dirMag =
        if absX > absY {
            (absX) + absY / 2 as libc::c_int
        } else { (absY) + absX / 2 as libc::c_int };
    // Calculate the component of the movement in the direction of the tangent vector
    unitX = dirX as FRACT / dirMag as FRACT;
    unitY = dirY as FRACT / dirMag as FRACT;
    dotRes = dotRes / dirMag as FRACT;
    *pMx = unitX * dotRes;
    *pMy = unitY * dotRes;
}
// see if a droid has run into a blocking tile
#[no_mangle]
pub unsafe extern "C" fn moveCalcBlockingSlide(mut psDroid: *mut DROID,
                                               mut pmx: *mut FRACT,
                                               mut pmy: *mut FRACT,
                                               mut tarDir: SDWORD,
                                               mut pSlideDir: *mut SDWORD) {
    let mut mx: FRACT = *pmx; // current tile x,y and new tile x,y
    let mut my: FRACT = *pmy;
    let mut nx: FRACT = 0.;
    let mut ny: FRACT = 0.;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut ntx: SDWORD = 0;
    let mut nty: SDWORD = 0;
    let mut blkCX: SDWORD = 0;
    let mut blkCY: SDWORD = 0;
    let mut horizX: SDWORD = 0;
    let mut horizY: SDWORD = 0;
    let mut vertX: SDWORD = 0;
    let mut vertY: SDWORD = 0;
    let mut intx: SDWORD = 0;
    let mut inty: SDWORD = 0;
    let mut jumpx: SDWORD = 0;
    let mut jumpy: SDWORD = 0;
    let mut bJumped: SDWORD = 0 as libc::c_int;
    //	FRACT	mag, rad, temp;
    let mut radx: FRACT = 0.;
    let mut rady: FRACT = 0.;
    let mut blocked: BOOL = 0;
    let mut slideDir: SDWORD = 0;
    blocked = 0 as libc::c_int;
    radx = 0 as libc::c_int as FRACT;
    rady = 0 as libc::c_int as FRACT;
    // calculate the new coords and see if they are on a different tile
    tx = (*psDroid).sMove.fx as SDWORD >> 7 as libc::c_int;
    ty = (*psDroid).sMove.fy as SDWORD >> 7 as libc::c_int;
    nx = (*psDroid).sMove.fx + mx;
    ny = (*psDroid).sMove.fy + my;
    ntx = nx as SDWORD >> 7 as libc::c_int;
    nty = ny as SDWORD >> 7 as libc::c_int;
    // is the new tile blocking?
    if fpathBlockingTile.expect("non-null function pointer")(ntx, nty) != 0 {
        blocked = 1 as libc::c_int
    }
    // now test ahead of the droid
/*	if (!blocked)
	{
		rad = MKF(moveObjRadius((BASE_OBJECT *)psDroid));
		mag = fSQRT(mx*mx + my*my);

		if (mag==0)
		{
			*pmx = MKF(0);
			*pmy = MKF(0);
			return;
		}

		radx = (rad * mx) / mag;
		rady = (rad * my) / mag;

		nx = psDroid->sMove.fx + radx;
		ny = psDroid->sMove.fy + rady;
		tx = MAKEINT(nx) >> TILE_SHIFT;
		ty = MAKEINT(ny) >> TILE_SHIFT;
		nx += mx;
		ny += my;
		ntx = MAKEINT(nx) >> TILE_SHIFT;
		nty = MAKEINT(ny) >> TILE_SHIFT;

		// is the new tile blocking?
		if (fpathBlockingTile(ntx,nty))
		{
			blocked = TRUE;
		}
	}

	// now test one side of the droid
	if (!blocked)
	{
		nx = psDroid->sMove.fx - rady;
		ny = psDroid->sMove.fy + radx;
		tx = MAKEINT(nx) >> TILE_SHIFT;
		ty = MAKEINT(ny) >> TILE_SHIFT;
		nx += mx;
		ny += my;
		ntx = MAKEINT(nx) >> TILE_SHIFT;
		nty = MAKEINT(ny) >> TILE_SHIFT;

		// is the new tile blocking?
		if (fpathBlockingTile(ntx,nty))
		{
			blocked = TRUE;
			temp = radx;
			radx = -rady;
			rady = radx;
		}
	}

	// now test the other side of the droid
	if (!blocked)
	{
		nx = psDroid->sMove.fx + rady;
		ny = psDroid->sMove.fy - radx;
		tx = MAKEINT(nx) >> TILE_SHIFT;
		ty = MAKEINT(ny) >> TILE_SHIFT;
		nx += mx;
		ny += my;
		ntx = MAKEINT(nx) >> TILE_SHIFT;
		nty = MAKEINT(ny) >> TILE_SHIFT;

		// is the new tile blocking?
		if (fpathBlockingTile(ntx,nty))
		{
			blocked = TRUE;
			temp = radx;
			radx = rady;
			rady = -radx;
		}
	}*/
    blkCX = (ntx << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    blkCY = (nty << 7 as libc::c_int) + 128 as libc::c_int / 2 as libc::c_int;
    // is the new tile blocking?
    if blocked == 0 {
        // not blocking, don't change the move vector
        return
    }
    // if the droid is shuffling - just stop
    if (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int {
        (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
    }
    // note the bump time and position if necessary
    if vtolDroid(psDroid) == 0 &&
           (*psDroid).sMove.bumpTime == 0 as libc::c_int as libc::c_uint {
        (*psDroid).sMove.bumpTime = gameTime;
        (*psDroid).sMove.lastBump = 0 as libc::c_int as UWORD;
        (*psDroid).sMove.pauseTime = 0 as libc::c_int as UWORD;
        (*psDroid).sMove.bumpX = (*psDroid).x;
        (*psDroid).sMove.bumpY = (*psDroid).y;
        (*psDroid).sMove.bumpDir = (*psDroid).direction as SWORD
    }
    if tx != ntx && ty != nty {
        // moved diagonally
        // figure out where the other two possible blocking tiles are
        horizX =
            if mx < 0 as libc::c_int as libc::c_float {
                (ntx) + 1 as libc::c_int
            } else { (ntx) - 1 as libc::c_int };
        horizY = nty;
        vertX = ntx;
        vertY =
            if my < 0 as libc::c_int as libc::c_float {
                (nty) + 1 as libc::c_int
            } else { (nty) - 1 as libc::c_int };
        if fpathBlockingTile.expect("non-null function pointer")(horizX,
                                                                 horizY) != 0
               &&
               fpathBlockingTile.expect("non-null function pointer")(vertX,
                                                                     vertY) !=
                   0 {
            //		*pmx = MAKEFRACT(0);
//		*pmy = MAKEFRACT(0);
            // in a corner - choose an arbitrary slide
            if rand() % 2 as libc::c_int == 0 as libc::c_int {
                *pmx = 0 as libc::c_int as FRACT;
                *pmy = -*pmy
            } else { *pmx = -*pmx; *pmy = 0 as libc::c_int as FRACT }
        } else if fpathBlockingTile.expect("non-null function pointer")(horizX,
                                                                        horizY)
                      != 0 {
            *pmy = 0 as libc::c_int as FRACT
        } else if fpathBlockingTile.expect("non-null function pointer")(vertX,
                                                                        vertY)
                      != 0 {
            *pmx = 0 as libc::c_int as FRACT
        } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
    } else if tx != ntx {
        // moved horizontally - see which half of the tile were in
        if (*psDroid).y as libc::c_int & 0x7f as libc::c_int >
               128 as libc::c_int / 2 as libc::c_int {
            // top half
            if fpathBlockingTile.expect("non-null function pointer")(ntx,
                                                                     nty +
                                                                         1 as
                                                                             libc::c_int)
                   != 0 {
                *pmx = 0 as libc::c_int as FRACT
            } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
        } else if fpathBlockingTile.expect("non-null function pointer")(ntx,
                                                                        nty -
                                                                            1
                                                                                as
                                                                                libc::c_int)
                      != 0 {
            *pmx = 0 as libc::c_int as FRACT
        } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
    } else if ty != nty {
        // bottom half
        // moved vertically
        if (*psDroid).x as libc::c_int & 0x7f as libc::c_int >
               128 as libc::c_int / 2 as libc::c_int {
            // top half
            if fpathBlockingTile.expect("non-null function pointer")(ntx +
                                                                         1 as
                                                                             libc::c_int,
                                                                     nty) != 0
               {
                *pmy = 0 as libc::c_int as FRACT
            } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
        } else if fpathBlockingTile.expect("non-null function pointer")(ntx -
                                                                            1
                                                                                as
                                                                                libc::c_int,
                                                                        nty)
                      != 0 {
            *pmy = 0 as libc::c_int as FRACT
        } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
    } else {
        // bottom half
        // if (tx == ntx && ty == nty)
        // on a blocking tile - see if we need to jump off
        intx = (*psDroid).sMove.fx as SDWORD & 0x7f as libc::c_int;
        inty = (*psDroid).sMove.fy as SDWORD & 0x7f as libc::c_int;
        jumpx = (*psDroid).x as SDWORD;
        jumpy = (*psDroid).y as SDWORD;
        bJumped = 0 as libc::c_int;
        if intx < 128 as libc::c_int / 2 as libc::c_int {
            if inty < 128 as libc::c_int / 2 as libc::c_int {
                // top left
                if mx < 0 as libc::c_int as libc::c_float &&
                       fpathBlockingTile.expect("non-null function pointer")(tx
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int,
                                                                             ty)
                           != 0 {
                    bJumped = 1 as libc::c_int;
                    jumpy =
                        (jumpy & !(0x7f as libc::c_int)) - 1 as libc::c_int
                }
                if my < 0 as libc::c_int as libc::c_float &&
                       fpathBlockingTile.expect("non-null function pointer")(tx,
                                                                             ty
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                           != 0 {
                    bJumped = 1 as libc::c_int;
                    jumpx =
                        (jumpx & !(0x7f as libc::c_int)) - 1 as libc::c_int
                }
            } else {
                // bottom left
                if mx < 0 as libc::c_int as libc::c_float &&
                       fpathBlockingTile.expect("non-null function pointer")(tx
                                                                                 -
                                                                                 1
                                                                                     as
                                                                                     libc::c_int,
                                                                             ty)
                           != 0 {
                    bJumped = 1 as libc::c_int;
                    jumpy =
                        (jumpy & !(0x7f as libc::c_int)) + 128 as libc::c_int
                }
                if my >= 0 as libc::c_int as libc::c_float &&
                       fpathBlockingTile.expect("non-null function pointer")(tx,
                                                                             ty
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     libc::c_int)
                           != 0 {
                    bJumped = 1 as libc::c_int;
                    jumpx =
                        (jumpx & !(0x7f as libc::c_int)) - 1 as libc::c_int
                }
            }
        } else if inty < 128 as libc::c_int / 2 as libc::c_int {
            // top right
            if mx >= 0 as libc::c_int as libc::c_float &&
                   fpathBlockingTile.expect("non-null function pointer")(tx +
                                                                             1
                                                                                 as
                                                                                 libc::c_int,
                                                                         ty)
                       != 0 {
                bJumped = 1 as libc::c_int;
                jumpy = (jumpy & !(0x7f as libc::c_int)) - 1 as libc::c_int
            }
            if my < 0 as libc::c_int as libc::c_float &&
                   fpathBlockingTile.expect("non-null function pointer")(tx,
                                                                         ty -
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                       != 0 {
                bJumped = 1 as libc::c_int;
                jumpx = (jumpx & !(0x7f as libc::c_int)) + 128 as libc::c_int
            }
        } else {
            // bottom right
            if mx >= 0 as libc::c_int as libc::c_float &&
                   fpathBlockingTile.expect("non-null function pointer")(tx +
                                                                             1
                                                                                 as
                                                                                 libc::c_int,
                                                                         ty)
                       != 0 {
                bJumped = 1 as libc::c_int;
                jumpy = (jumpy & !(0x7f as libc::c_int)) + 128 as libc::c_int
            }
            if my >= 0 as libc::c_int as libc::c_float &&
                   fpathBlockingTile.expect("non-null function pointer")(tx,
                                                                         ty +
                                                                             1
                                                                                 as
                                                                                 libc::c_int)
                       != 0 {
                bJumped = 1 as libc::c_int;
                jumpx = (jumpx & !(0x7f as libc::c_int)) + 128 as libc::c_int
            }
        }
        if bJumped != 0 {
            (*psDroid).x = (jumpx - radx as SDWORD) as SWORD as UWORD;
            (*psDroid).y = (jumpy - rady as SDWORD) as SWORD as UWORD;
            (*psDroid).sMove.fx = jumpx as FRACT;
            (*psDroid).sMove.fy = jumpy as FRACT;
            *pmx = 0 as libc::c_int as FRACT;
            *pmy = 0 as libc::c_int as FRACT
        } else { moveCalcSlideVector(psDroid, blkCX, blkCY, pmx, pmy); }
    }
    slideDir = vectorToAngle(*pmx, *pmy) as SDWORD;
    if ntx != tx {
        /*		jumpx = MAKEINT(nx - mx);
		jumpy = MAKEINT(ny - my);
		intx = jumpx & TILE_MASK;
		inty = jumpy & TILE_MASK;
		bJumped = FALSE;*/
        // hit a horizontal block
        if (tarDir < 90 as libc::c_int || tarDir > 270 as libc::c_int) &&
               (slideDir >= 90 as libc::c_int &&
                    slideDir <= 270 as libc::c_int) {
            slideDir = tarDir
        } else if tarDir >= 90 as libc::c_int && tarDir <= 270 as libc::c_int
                      &&
                      (slideDir < 90 as libc::c_int ||
                           slideDir > 270 as libc::c_int) {
            slideDir = tarDir
        }
    }
    if nty != ty {
        // hit a vertical block
        if tarDir < 180 as libc::c_int && slideDir >= 180 as libc::c_int {
            slideDir = tarDir
        } else if tarDir >= 180 as libc::c_int &&
                      slideDir < 180 as libc::c_int {
            slideDir = tarDir
        }
    }
    *pSlideDir = slideDir;
}
// see if a droid has run into another droid
// Only consider stationery droids
#[no_mangle]
pub unsafe extern "C" fn moveCalcDroidSlide(mut psDroid: *mut DROID,
                                            mut pmx: *mut FRACT,
                                            mut pmy: *mut FRACT) {
    let mut i: SDWORD = 0;
    let mut droidR: SDWORD = 0;
    let mut rad: SDWORD = 0;
    let mut radSq: SDWORD = 0;
    let mut objR: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut psInfo: *mut NAYBOR_INFO = 0 as *mut NAYBOR_INFO;
    let mut psObst: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bLegs: BOOL = 0;
    bLegs = 0 as libc::c_int;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint ||
           cyborgDroid(psDroid) != 0 {
        bLegs = 1 as libc::c_int
    }
    droidR = moveObjRadius(psDroid as *mut BASE_OBJECT);
    psObst = 0 as *mut BASE_OBJECT;
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        psInfo = asDroidNaybors.as_mut_ptr().offset(i as isize);
        if (*(*psInfo).psObj).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            if !((*((*psInfo).psObj as *mut DROID)).droidType as libc::c_uint
                     == DROID_TRANSPORTER as libc::c_int as libc::c_uint) {
                if !(bLegs != 0 &&
                         (*((*psInfo).psObj as *mut DROID)).droidType as
                             libc::c_uint !=
                             DROID_PERSON as libc::c_int as libc::c_uint &&
                         cyborgDroid((*psInfo).psObj as *mut DROID) == 0) {
                    if !(bLegs == 0 &&
                             (*((*psInfo).psObj as *mut DROID)).droidType as
                                 libc::c_uint ==
                                 DROID_PERSON as libc::c_int as libc::c_uint)
                       {
                        objR = moveObjRadius((*psInfo).psObj);
                        rad = droidR + objR;
                        radSq = rad * rad;
                        xdiff =
                            ((*psDroid).sMove.fx + *pmx) as SDWORD -
                                (*(*psInfo).psObj).x as SDWORD;
                        ydiff =
                            ((*psDroid).sMove.fy + *pmy) as SDWORD -
                                (*(*psInfo).psObj).y as SDWORD;
                        distSq = xdiff * xdiff + ydiff * ydiff;
                        if !(xdiff as FRACT * *pmx + ydiff as FRACT * *pmy >=
                                 0 as libc::c_int as libc::c_float) {
                            if radSq > distSq {
                                if !psObst.is_null() ||
                                       aiCheckAlliances((*(*psInfo).psObj).player
                                                            as UDWORD,
                                                        (*psDroid).player as
                                                            UDWORD) == 0 {
                                    // hit more than one droid - stop
                                    *pmx = 0 as libc::c_int as FRACT;
                                    *pmy = 0 as libc::c_int as FRACT;
                                    psObst = 0 as *mut BASE_OBJECT;
                                    break ;
                                } else {
                                    //				if (((DROID *)psInfo->psObj)->sMove.Status == MOVEINACTIVE)
                                    psObst = (*psInfo).psObj;
                                    // note the bump time and position if necessary
                                    if (*psDroid).sMove.bumpTime ==
                                           0 as libc::c_int as libc::c_uint {
                                        (*psDroid).sMove.bumpTime = gameTime;
                                        (*psDroid).sMove.lastBump =
                                            0 as libc::c_int as UWORD;
                                        (*psDroid).sMove.pauseTime =
                                            0 as libc::c_int as UWORD;
                                        (*psDroid).sMove.bumpX = (*psDroid).x;
                                        (*psDroid).sMove.bumpY = (*psDroid).y;
                                        (*psDroid).sMove.bumpDir =
                                            (*psDroid).direction as SWORD
                                    } else {
                                        (*psDroid).sMove.lastBump =
                                            gameTime.wrapping_sub((*psDroid).sMove.bumpTime)
                                                as UWORD
                                    }
                                    // tell inactive droids to get out the way
                                    if (*psObst).type_0 as libc::c_uint ==
                                           OBJ_DROID as libc::c_int as
                                               libc::c_uint &&
                                           aiCheckAlliances((*psObst).player
                                                                as UDWORD,
                                                            (*psDroid).player
                                                                as UDWORD) !=
                                               0 &&
                                           ((*(psObst as
                                                   *mut DROID)).sMove.Status
                                                as libc::c_int ==
                                                0 as libc::c_int ||
                                                (*(psObst as
                                                       *mut DROID)).sMove.Status
                                                    as libc::c_int ==
                                                    7 as libc::c_int) {
                                        if (*psDroid).sMove.Status as
                                               libc::c_int ==
                                               12 as libc::c_int {
                                            moveShuffleDroid(psObst as
                                                                 *mut DROID,
                                                             (*psDroid).sMove.shuffleStart,
                                                             (*psDroid).sMove.targetX
                                                                 -
                                                                 (*psDroid).x
                                                                     as
                                                                     SDWORD,
                                                             (*psDroid).sMove.targetY
                                                                 -
                                                                 (*psDroid).y
                                                                     as
                                                                     SDWORD);
                                        } else {
                                            moveShuffleDroid(psObst as
                                                                 *mut DROID,
                                                             gameTime,
                                                             (*psDroid).sMove.targetX
                                                                 -
                                                                 (*psDroid).x
                                                                     as
                                                                     SDWORD,
                                                             (*psDroid).sMove.targetY
                                                                 -
                                                                 (*psDroid).y
                                                                     as
                                                                     SDWORD);
                                        }
                                    }
                                }
                            } else if (*psInfo).distSqr >
                                          (128 as libc::c_int *
                                               4 as libc::c_int *
                                               (128 as libc::c_int *
                                                    4 as libc::c_int)) as
                                              libc::c_uint {
                                break ;
                            }
                        }
                    }
                }
            }
        }
        // object behind
        i += 1
    }
    if !psObst.is_null() {
        // Try to slide round it
        moveCalcSlideVector(psDroid, (*psObst).x as SDWORD,
                            (*psObst).y as SDWORD, pmx, pmy);
    };
}
// Get the movement vector for an object
/*void moveGetObstMove(BASE_OBJECT *psObj, FRACT *pX, FRACT *pY)
{
	switch (psObj->type)
	{
	case OBJ_DROID:
		if (((DROID *)psObj)->sMove.Status == MOVEPOINTTOPOINT)
		{
			*pX=((DROID *)psObj)->sMove.dx;
			*pY=((DROID *)psObj)->sMove.dy;
		}
		else
		{
			*pX=(FRACT)0;
			*pY=(FRACT)0;
		}
		break;
	case OBJ_STRUCTURE:
		*pX=(FRACT)0;
		*pY=(FRACT)0;
		break;
	case OBJ_FEATURE:
		*pX=(FRACT)0;
		*pY=(FRACT)0;
		break;
	default:
		ASSERT( FALSE,"moveGetObstMove: unknown object type" );
		*pX = (FRACT)0;
		*pY = (FRACT)0;
		break;
	}
}*/
// Get a charged particle vector from an object
/*BOOL moveGetObstVector(DROID *psDroid, BASE_OBJECT *psObj, FRACT *pX, FRACT *pY)
{
	SDWORD		xdiff,ydiff, absX,absY, mag;
	FRACT		normX,normY;
	FRACT		mx,my;//, omx,omy;
	FRACT		avoidX, avoidY, resMag;
//	SDWORD		obstRad, obstBound, obstRange;

	xdiff = (SDWORD)psDroid->x - (SDWORD)psObj->x;
	ydiff = (SDWORD)psDroid->y - (SDWORD)psObj->y;
	mx = psDroid->sMove.dx;
	my = psDroid->sMove.dy;

	if (xdiff* (*pX) + ydiff* (*pY) >= 0 ||
		moveObjOnTarget(psDroid,psObj))
	{
		return FALSE;
	}
//	moveGetObstMove(psObj, &omx,&omy);
//	obstRad = moveObjRadius(psObj);
//	obstBound = 3 * obstRad;
//	obstRange = obstBound - obstRad;

	// Calculate the normalised vector from the obstacle to the droid
	absX = labs(xdiff);
	absY = labs(ydiff);
	mag = absX > absY ? absX + absY/2 : absY + absX/2;
	normX = FRACTdiv(MAKEFRACT(xdiff), MAKEFRACT(mag));
	normY = FRACTdiv(MAKEFRACT(ydiff), MAKEFRACT(mag));
	DBP3(("mag %d\n", mag));

	// Create the avoid vector
	if (FRACTmul(*pX, normY) + FRACTmul(*pY,-normX) < 0)
	{
		DBP3(("First perp\n"));
		avoidX = -normY;
		avoidY = normX;
	}
	else
	{
		DBP3(("Second perp\n"));
		avoidX = normY;
		avoidY = -normX;
	}


//	if (mag > obstBound)
//	{
//		DBP3(("mag > obstBound\n"));
		*pX = *pX * (float)mag / AVOID_DIST +
			  avoidX * (AVOID_DIST - (float)mag)/AVOID_DIST;
		*pY = *pY * (float)mag / AVOID_DIST +
			  avoidY * (AVOID_DIST - (float)mag)/AVOID_DIST;

		resMag = FRACTmul(*pX, *pX) + FRACTmul(*pY,*pY);
		resMag = fSQRT(resMag);
		*pX = FRACTdiv((*pX),resMag);
		*pY = FRACTdiv((*pY),resMag);
//	}
//	else
//	{
//		DBP3(("mag < obstBound\n"));
//		*pX = *pX * (float)mag / AVOID_DIST +
//			  normX * (float)(obstBound - mag)/obstRange +
//			  avoidX * (mag - obstRad)/obstBound;
//		*pY = *pY * (float)mag / AVOID_DIST +
//			  normY * (float)(obstBound - mag)/obstRange +
//			  avoidY * (mag - obstRad)/obstBound;
//	}

	return TRUE;
}*/
// Get the distance to a tile if it is on the map
#[no_mangle]
pub unsafe extern "C" fn moveGetTileObst(mut cx: SDWORD, mut cy: SDWORD,
                                         mut ox: SDWORD, mut oy: SDWORD,
                                         mut pDist: *mut SDWORD) -> BOOL {
    let mut absx: SDWORD = 0;
    let mut absy: SDWORD = 0;
    if cx + ox < 0 as libc::c_int || cx + ox >= mapWidth as SDWORD ||
           cy + oy < 0 as libc::c_int || cy + oy >= mapHeight as SDWORD {
        return 0 as libc::c_int
    }
    //	if (TERRAIN_TYPE(mapTile(cx+ox, cy+oy)) != TER_CLIFFFACE)
    if fpathBlockingTile.expect("non-null function pointer")(cx + ox, cy + oy)
           == 0 {
        return 0 as libc::c_int
    }
    absx =
        (labs(ox as libc::c_long) * 128 as libc::c_int as libc::c_long) as
            SDWORD;
    absy =
        (labs(oy as libc::c_long) * 128 as libc::c_int as libc::c_long) as
            SDWORD;
    *pDist =
        if absx > absy {
            (absx) + absy / 2 as libc::c_int
        } else { (absy) + absx / 2 as libc::c_int };
    return 1 as libc::c_int;
}
// Get a charged particle vector from all nearby objects
#[no_mangle]
pub unsafe extern "C" fn moveGetObstVector2(mut psDroid: *mut DROID,
                                            mut pX: *mut FRACT,
                                            mut pY: *mut FRACT) {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut absX: SDWORD = 0;
    let mut absY: SDWORD = 0;
    let mut mag: SDWORD = 0;
    let mut minMag: SDWORD = 0;
    let mut ox: FRACT = 0.;
    let mut oy: FRACT = 0.;
    let mut normX: FRACT = 0.;
    let mut normY: FRACT = 0.;
    let mut ratio: FRACT = 0.;
    let mut avoidX: FRACT = 0.;
    let mut avoidY: FRACT = 0.;
    let mut resMag: FRACT = 0.;
    let mut i: SDWORD = 0;
    let mut size: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut mapX: SDWORD = 0;
    let mut mapY: SDWORD = 0;
    let mut DivTop: FRACT = 0.;
    let mut DivBot: FRACT = 0.;
    normY = 0 as libc::c_int as FRACT;
    normX = normY;
    size = 0 as libc::c_int;
    minMag = 128 as libc::c_int * 2 as libc::c_int;
    droidGetNaybors(psDroid);
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        if !(asDroidNaybors[i as usize].distSqr >
                 (128 as libc::c_int * 2 as libc::c_int *
                      (128 as libc::c_int * 2 as libc::c_int)) as
                     libc::c_uint) {
            psObj = asDroidNaybors[i as usize].psObj;
            if !(moveObjOnTarget(psDroid, psObj) != 0) {
                xdiff = (*psDroid).x as SDWORD - (*psObj).x as SDWORD;
                ydiff = (*psDroid).y as SDWORD - (*psObj).y as SDWORD;
                if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY >=
                         0 as libc::c_int as libc::c_float) {
                    // Calculate the vector from the obstacle to the droid
                    absX = labs(xdiff as libc::c_long) as SDWORD;
                    absY = labs(ydiff as libc::c_long) as SDWORD;
                    mag =
                        if absX > absY {
                            (absX) + absY / 2 as libc::c_int
                        } else { (absY) + absX / 2 as libc::c_int };
                    ox = xdiff as FRACT / mag as FRACT;
                    oy = ydiff as FRACT / mag as FRACT;
                    // Add the obstacle vector to the total, biased for distance
//		ratio = FRACTdiv(MAKEFRACT(AVOID_DIST*AVOID_DIST - mag*mag), MAKEFRACT(AVOID_DIST*AVOID_DIST));
                    if !(mag >= 128 as libc::c_int * 2 as libc::c_int) {
                        DivTop =
                            (128 as libc::c_int * 2 as libc::c_int *
                                 (128 as libc::c_int * 2 as libc::c_int) -
                                 mag * mag) as FRACT; //AVOID_DIST - mag;
                        DivBot =
                            (128 as libc::c_int * 2 as libc::c_int *
                                 (128 as libc::c_int * 2 as libc::c_int)) as
                                FRACT;
                        ratio = DivTop / DivBot;
                        normX += ox * ratio;
                        normY += oy * ratio;
                        size += 1 as libc::c_int;
                        if minMag > mag { minMag = mag }
                    }
                }
            }
        }
        // dont consider if it is EQUAL !!
        i += 1
        // not worth considering if it is right on the border ... leads to divide by zero errors
    }
    // now scan the tiles for TER_CLIFFFACE
    mapX = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    mapY = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    ydiff = -(2 as libc::c_int);
    while ydiff <= 2 as libc::c_int {
        xdiff = -(2 as libc::c_int);
        while xdiff <= 2 as libc::c_int {
            if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY <=
                     0 as libc::c_int as libc::c_float) {
                if moveGetTileObst(mapX, mapY, xdiff, ydiff, &mut mag) != 0 {
                    if !(mag >= 128 as libc::c_int * 2 as libc::c_int) {
                        ox =
                            (-xdiff * 128 as libc::c_int) as FRACT /
                                mag as FRACT;
                        oy =
                            (-ydiff * 128 as libc::c_int) as FRACT /
                                mag as FRACT;
                        // Add the obstacle vector to the total, biased for distance
//old				ratio = FRACTdiv(MAKEFRACT(AVOID_DIST*AVOID_DIST - mag*mag), MAKEFRACT(AVOID_DIST*AVOID_DIST));
                        DivTop =
                            (128 as libc::c_int * 2 as libc::c_int *
                                 (128 as libc::c_int * 2 as libc::c_int) -
                                 mag * mag) as FRACT; //AVOID_DIST - mag;
                        DivBot =
                            (128 as libc::c_int * 2 as libc::c_int *
                                 (128 as libc::c_int * 2 as libc::c_int)) as
                                FRACT;
                        ratio = DivTop / DivBot;
                        normX += ox * ratio;
                        normY += oy * ratio;
                        size += 1 as libc::c_int;
                        if minMag > mag { minMag = mag }
                    }
                }
            }
            // dont consider if it is EQUAL !!
            xdiff += 1
            // not worth considering if it is right on the border ... leads to divide by zero errors
        }
        ydiff += 1
    }
    if size != 0 as libc::c_int {
        /*		normX = FRACTdiv(normX, MAKEFRACT(size));
		normY = FRACTdiv(normY, MAKEFRACT(size));*/
        resMag =
            sqrt((normX * normX + normY * normY) as libc::c_double) as FRACT;
        normX = normX / resMag;
        normY = normY / resMag;
        mag = minMag;
        // Create the avoid vector
        if *pX * normY + *pY * -normX < 0 as libc::c_int as libc::c_float {
            avoidX = -normY;
            avoidY = normX
        } else { avoidX = normY; avoidY = -normX }
        *pX =
            *pX * mag as libc::c_float /
                (128 as libc::c_int * 2 as libc::c_int) as libc::c_float +
                avoidX *
                    ((128 as libc::c_int * 2 as libc::c_int) as libc::c_float
                         - mag as libc::c_float) /
                    (128 as libc::c_int * 2 as libc::c_int) as libc::c_float;
        *pY =
            *pY * mag as libc::c_float /
                (128 as libc::c_int * 2 as libc::c_int) as libc::c_float +
                avoidY *
                    ((128 as libc::c_int * 2 as libc::c_int) as libc::c_float
                         - mag as libc::c_float) /
                    (128 as libc::c_int * 2 as libc::c_int) as libc::c_float;
        resMag = *pX * *pX + *pY * *pY;
        resMag = sqrt(resMag as libc::c_double) as FRACT;
        *pX = *pX / resMag;
        *pY = *pY / resMag
    };
}
// get an obstacle avoidance vector
#[no_mangle]
pub unsafe extern "C" fn moveGetObstVector3(mut psDroid: *mut DROID,
                                            mut pX: *mut FRACT,
                                            mut pY: *mut FRACT) {
    let mut i: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut numObst: SDWORD = 0;
    let mut dirTot: SDWORD = 0;
    let mut distTot: SDWORD = 0;
    let mut ox: FRACT = 0.;
    let mut oy: FRACT = 0.;
    let mut ratio: FRACT = 0.;
    let mut avoidX: FRACT = 0.;
    let mut avoidY: FRACT = 0.;
    let mut mapX: SDWORD = 0;
    let mut mapY: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut td: SDWORD = 0;
    numObst = 0 as libc::c_int;
    dirTot = 0 as libc::c_int;
    distTot = 0 as libc::c_int;
    droidGetNaybors(psDroid);
    // scan the neighbours for obstacles
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        if !(asDroidNaybors[i as usize].distSqr >=
                 (128 as libc::c_int * 2 as libc::c_int *
                      (128 as libc::c_int * 2 as libc::c_int)) as
                     libc::c_uint) {
            psObj = asDroidNaybors[i as usize].psObj;
            if !(moveObjOnTarget(psDroid, psObj) != 0) {
                xdiff = (*psDroid).x as SDWORD - (*psObj).x as SDWORD;
                ydiff = (*psDroid).y as SDWORD - (*psObj).y as SDWORD;
                if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY >=
                         0 as libc::c_int as libc::c_float) {
                    dirTot +=
                        calcDirection((*psObj).x as UDWORD,
                                      (*psObj).y as UDWORD,
                                      (*psDroid).x as UDWORD,
                                      (*psDroid).y as UDWORD);
                    distTot =
                        (distTot as
                             libc::c_uint).wrapping_add(((128 as libc::c_int *
                                                              2 as libc::c_int
                                                              *
                                                              (128 as
                                                                   libc::c_int
                                                                   *
                                                                   2 as
                                                                       libc::c_int))
                                                             as
                                                             libc::c_uint).wrapping_sub(asDroidNaybors[i
                                                                                                           as
                                                                                                           usize].distSqr))
                            as SDWORD as SDWORD;
                    numObst += 1 as libc::c_int
                }
            }
        }
        // object behind
        i += 1
    }
    // now scan for blocking tiles
    mapX = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    mapY = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    ydiff = -(2 as libc::c_int);
    while ydiff <= 2 as libc::c_int {
        xdiff = -(2 as libc::c_int);
        while xdiff <= 2 as libc::c_int {
            if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY <=
                     0 as libc::c_int as libc::c_float) {
                if fpathBlockingTile.expect("non-null function pointer")(mapX
                                                                             +
                                                                             xdiff,
                                                                         mapY
                                                                             +
                                                                             ydiff)
                       != 0 {
                    tx = xdiff << 7 as libc::c_int;
                    ty = ydiff << 7 as libc::c_int;
                    td = tx * tx + ty * ty;
                    if td <
                           128 as libc::c_int * 2 as libc::c_int *
                               (128 as libc::c_int * 2 as libc::c_int) {
                        dirTot +=
                            calcDirection(((*psDroid).x as libc::c_int + tx)
                                              as UDWORD,
                                          ((*psDroid).y as libc::c_int + ty)
                                              as UDWORD,
                                          (*psDroid).x as UDWORD,
                                          (*psDroid).y as UDWORD);
                        distTot +=
                            128 as libc::c_int * 2 as libc::c_int *
                                (128 as libc::c_int * 2 as libc::c_int) - td;
                        numObst += 1 as libc::c_int
                    }
                }
            }
            // object behind
            xdiff += 1
        }
        ydiff += 1
    }
    if numObst > 0 as libc::c_int {
        dirTot /= numObst;
        distTot /= numObst;
        // Create the avoid vector
//		dirTot = (SDWORD)adjustDirection(dirTot, 180);
        angleToVector(dirTot, &mut ox, &mut oy);
        if *pX * oy + *pY * -ox < 0 as libc::c_int as libc::c_float {
            avoidX = -oy;
            avoidY = ox
        } else { avoidX = oy; avoidY = -ox }
        // combine the avoid vector and the target vector
        ratio =
            distTot as FRACT /
                (128 as libc::c_int * 2 as libc::c_int *
                     (128 as libc::c_int * 2 as libc::c_int)) as FRACT;
        *pX =
            *pX * (1 as libc::c_int as libc::c_float - ratio) +
                avoidX * ratio;
        *pY =
            *pY * (1 as libc::c_int as libc::c_float - ratio) + avoidY * ratio
    };
}
/* arrow colours */
// get an obstacle avoidance vector
#[no_mangle]
pub unsafe extern "C" fn moveGetObstVector4(mut psDroid: *mut DROID,
                                            mut pX: *mut FRACT,
                                            mut pY: *mut FRACT) {
    let mut i: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut absx: SDWORD = 0;
    let mut absy: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut numObst: SDWORD = 0;
    let mut distTot: SDWORD = 0;
    let mut dirX: FRACT = 0.;
    let mut dirY: FRACT = 0.;
    let mut omag: FRACT = 0.;
    let mut ox: FRACT = 0.;
    let mut oy: FRACT = 0.;
    let mut ratio: FRACT = 0.;
    let mut avoidX: FRACT = 0.;
    let mut avoidY: FRACT = 0.;
    let mut mapX: SDWORD = 0;
    let mut mapY: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut td: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              2509 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"moveGetObstVector4\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    numObst = 0 as libc::c_int;
    dirX = 0 as libc::c_int as FRACT;
    dirY = 0 as libc::c_int as FRACT;
    distTot = 0 as libc::c_int;
    droidGetNaybors(psDroid);
    // scan the neighbours for obstacles
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        psObj = asDroidNaybors[i as usize].psObj;
        if !((*psObj).type_0 as libc::c_uint !=
                 OBJ_DROID as libc::c_int as libc::c_uint ||
                 asDroidNaybors[i as usize].distSqr >=
                     (128 as libc::c_int * 2 as libc::c_int *
                          (128 as libc::c_int * 2 as libc::c_int)) as
                         libc::c_uint) {
            // vtol droids only avoid each other and don't affect ground droids
            if !(vtolDroid(psDroid) != 0 &&
                     ((*psObj).type_0 as libc::c_uint !=
                          OBJ_DROID as libc::c_int as libc::c_uint ||
                          vtolDroid(psObj as *mut DROID) == 0) ||
                     vtolDroid(psDroid) == 0 &&
                         (*psObj).type_0 as libc::c_uint ==
                             OBJ_DROID as libc::c_int as libc::c_uint &&
                         vtolDroid(psObj as *mut DROID) != 0) {
                if !((*(psObj as *mut DROID)).droidType as libc::c_uint ==
                         DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
                         (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                             DROID_PERSON as libc::c_int as libc::c_uint &&
                             (*psObj).player as libc::c_int !=
                                 (*psDroid).player as libc::c_int) {
                    xdiff = (*psObj).x as SDWORD - (*psDroid).x as SDWORD;
                    ydiff = (*psObj).y as SDWORD - (*psDroid).y as SDWORD;
                    if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY <
                             0 as libc::c_int as libc::c_float) {
                        absx = labs(xdiff as libc::c_long) as SDWORD;
                        absy = labs(ydiff as libc::c_long) as SDWORD;
                        dist =
                            if absx > absy {
                                (absx) + absy / 2 as libc::c_int
                            } else { (absx / 2 as libc::c_int) + absy };
                        if dist != 0 as libc::c_int {
                            dirX += xdiff as FRACT / (dist * dist) as FRACT;
                            dirY += ydiff as FRACT / (dist * dist) as FRACT;
                            distTot += dist * dist;
                            numObst += 1 as libc::c_int
                        } else {
                            dirX += xdiff as FRACT;
                            dirY += ydiff as FRACT;
                            numObst += 1 as libc::c_int
                        }
                    }
                }
            }
        }
        // object behind
        i += 1
    }
    // now scan for blocking tiles
    mapX = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    mapY = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    ydiff = -(2 as libc::c_int);
    while ydiff <= 2 as libc::c_int {
        xdiff = -(2 as libc::c_int);
        while xdiff <= 2 as libc::c_int {
            if !(xdiff as FRACT * *pX + ydiff as FRACT * *pY <=
                     0 as libc::c_int as libc::c_float) {
                if fpathBlockingTile.expect("non-null function pointer")(mapX
                                                                             +
                                                                             xdiff,
                                                                         mapY
                                                                             +
                                                                             ydiff)
                       != 0 {
                    tx = xdiff << 7 as libc::c_int;
                    ty = ydiff << 7 as libc::c_int;
                    td = tx * tx + ty * ty;
                    if td <
                           128 as libc::c_int * 2 as libc::c_int *
                               (128 as libc::c_int * 2 as libc::c_int) {
                        absx = labs(tx as libc::c_long) as SDWORD;
                        absy = labs(ty as libc::c_long) as SDWORD;
                        dist =
                            if absx > absy {
                                (absx) + absy / 2 as libc::c_int
                            } else { (absx / 2 as libc::c_int) + absy };
                        if dist != 0 as libc::c_int {
                            dirX += tx as FRACT / (dist * dist) as FRACT;
                            dirY += ty as FRACT / (dist * dist) as FRACT;
                            distTot += dist * dist;
                            numObst += 1 as libc::c_int
                        }
                    }
                }
            }
            // object behind
            xdiff += 1
        }
        ydiff += 1
    }
    if numObst > 0 as libc::c_int {
        distTot /= numObst;
        // Create the avoid vector
        if dirX == 0 as libc::c_int as FRACT &&
               dirY == 0 as libc::c_int as FRACT {
            avoidX = 0 as libc::c_int as FRACT;
            avoidY = 0 as libc::c_int as FRACT;
            distTot =
                128 as libc::c_int * 2 as libc::c_int *
                    (128 as libc::c_int * 2 as libc::c_int)
        } else {
            omag =
                sqrt((dirX * dirX + dirY * dirY) as libc::c_double) as FRACT;
            ox = dirX / omag;
            oy = dirY / omag;
            if *pX * oy + *pY * -ox < 0 as libc::c_int as libc::c_float {
                avoidX = -oy;
                avoidY = ox
            } else { avoidX = oy; avoidY = -ox }
        }
        // combine the avoid vector and the target vector
        ratio =
            distTot as FRACT /
                (128 as libc::c_int * 2 as libc::c_int *
                     (128 as libc::c_int * 2 as libc::c_int)) as FRACT;
        if ratio > 1 as libc::c_int as FRACT {
            ratio = 1 as libc::c_int as FRACT
        }
        *pX =
            *pX * ratio +
                avoidX * (1 as libc::c_int as libc::c_float - ratio);
        *pY =
            *pY * ratio + avoidY * (1 as libc::c_int as libc::c_float - ratio)
    };
}
/*void moveUpdateRepulsiveVector( FRACT fVObstX, FRACT fVObstY, FRACT fVTarX, FRACT fVTarY,
					FRACT *pfVRepX, FRACT *pfVRepY, FRACT *pfDistTot )
{
	FRACT	fDistObjSq, fDistObj, fDistTar;
	FRACT	fDot, fCos, fCross, fAngle, fScale;

	fDistTar   = fSQRT( Fmul( fVTarX, fVTarX ) + Fmul( fVTarY, fVTarY ) );

	fDistObjSq = Fmul( fVObstX, fVObstX ) + Fmul( fVObstY, fVObstY );
	fDistObj   = fSQRT( fDistObjSq );

	fDot = Fmul(fVObstX,fVTarX) + Fmul(fVObstY,fVTarY);
	fCos = Fdiv( fDot, Fmul( fDistTar, fDistObj ) );
	fAngle = trigInvCos( fCos );

	// scale by angle to obstacle (zero repulsion for objects directly behind)
	fScale = Fdiv((MKF(180) - fAngle), MKF(180));

	// scale by distance to obstacle squared
	fScale = Fdiv( fScale, fDistObjSq );

	// decide which avoidance perpendicular to use
	fCross = FRACTmul(fVTarX, fVObstY) + FRACTmul(fVTarY,-fVObstX);
	if ( fCross < 0 )
	{
		*pfVRepX += Fmul( -fVObstY, fScale );
		*pfVRepY += Fmul(  fVObstX, fScale );
	}
	else
	{
		*pfVRepX += Fmul(  fVObstY, fScale );
		*pfVRepY += Fmul( -fVObstX, fScale );
	}

	*pfDistTot += fDistObjSq;
}*/
#[no_mangle]
pub unsafe extern "C" fn moveUpdateRepulsiveVector(mut fVObstX: FRACT,
                                                   mut fVObstY: FRACT,
                                                   mut fVTarX: FRACT,
                                                   mut fVTarY: FRACT,
                                                   mut pfVRepX: *mut FRACT,
                                                   mut pfVRepY: *mut FRACT,
                                                   mut pfDistTot:
                                                       *mut FRACT) {
    let mut fDistObjSq: FRACT = 0.;
    let mut fDistObj: FRACT = 0.;
    let mut fDot: FRACT = 0.;
    let mut fCross: FRACT = 0.;
    // ignore obstacles behind
    fDot = fVObstX * fVTarX + fVObstY * fVTarY;
    if fDot <= 0 as libc::c_int as libc::c_float { return }
    fDistObjSq = fVObstX * fVObstX + fVObstY * fVObstY;
    fDistObj = sqrt(fDistObjSq as libc::c_double) as FRACT;
    // decide which avoidance perpendicular to use
    fCross = fVTarX * fVObstY + fVTarY * -fVObstX;
    if fCross < 0 as libc::c_int as libc::c_float {
        *pfVRepX += -fVObstY / fDistObj;
        *pfVRepY += fVObstX / fDistObj
    } else { *pfVRepX += fVObstY / fDistObj; *pfVRepY += -fVObstX / fDistObj }
    *pfDistTot += fDistObjSq;
}
// get an obstacle avoidance vector
#[no_mangle]
pub unsafe extern "C" fn moveGetObstVector5(mut psDroid: *mut DROID,
                                            mut pX: *mut FRACT,
                                            mut pY: *mut FRACT) {
    let mut i: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut numObst: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut dirX: FRACT = 0.;
    let mut dirY: FRACT = 0.;
    let mut fDx: FRACT = 0.;
    let mut fDy: FRACT = 0.;
    let mut fDistTot: FRACT = 0.;
    let mut omag: FRACT = 0.;
    let mut ratio: FRACT = 0.;
    let mut avoidX: FRACT = 0.;
    let mut avoidY: FRACT = 0.;
    let mut mapX: SDWORD = 0;
    let mut mapY: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut td: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              2767 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"moveGetObstVector5\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    numObst = 0 as libc::c_int;
    dirX = 0 as libc::c_int as FRACT;
    dirY = 0 as libc::c_int as FRACT;
    fDistTot = 0 as libc::c_int as FRACT;
    /* if not flying check ground objects */
    if (*psPropStats).propulsionType as libc::c_int != LIFT as libc::c_int {
        droidGetNaybors(psDroid);
        // scan the neighbours for obstacles
        i = 0 as libc::c_int;
        while i < numNaybors as SDWORD {
            psObj = asDroidNaybors[i as usize].psObj;
            if !((*psObj).type_0 as libc::c_uint !=
                     OBJ_DROID as libc::c_int as libc::c_uint ||
                     asDroidNaybors[i as usize].distSqr >=
                         (128 as libc::c_int * 2 as libc::c_int *
                              (128 as libc::c_int * 2 as libc::c_int)) as
                             libc::c_uint) {
                if !((*(psObj as *mut DROID)).droidType as libc::c_uint ==
                         DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
                         (*(psObj as *mut DROID)).droidType as libc::c_uint ==
                             DROID_PERSON as libc::c_int as libc::c_uint &&
                             (*psObj).player as libc::c_int !=
                                 (*psDroid).player as libc::c_int) {
                    xdiff = (*psObj).x as SDWORD - (*psDroid).x as SDWORD;
                    ydiff = (*psObj).y as SDWORD - (*psDroid).y as SDWORD;
                    fDx = xdiff as FRACT;
                    fDy = ydiff as FRACT;
                    /* stop droids shagging an inactive droid which is
			 * sitting on their target */
/*			if ( psObj->type == OBJ_STRUCTURE || psObj->type == OBJ_FEATURE ||
				 (psObj->type == OBJ_DROID &&
				  ((DROID *) psObj)->sMove.Status == MOVEINACTIVE) )
			{
				SDWORD	iDx = (SDWORD)psObj->x - psDroid->sMove.targetX,
						iDy = (SDWORD)psObj->y - psDroid->sMove.targetY,
						iRad = moveObjRadius(psObj)*2;

				if ( iDx*iDx + iDy*iDy < iRad*iRad )
				{
					moveStopDroid( psDroid );
					return;
				}
			}*/
                    moveUpdateRepulsiveVector(fDx, fDy, *pX, *pY, &mut dirX,
                                              &mut dirY, &mut fDistTot);
                    numObst += 1
                }
            }
            // don't avoid people on the other side - run over them
            i += 1
        }
    }
    // now scan for blocking tiles
    mapX = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    mapY = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    ydiff = -(2 as libc::c_int);
    while ydiff <= 2 as libc::c_int {
        xdiff = -(2 as libc::c_int);
        while xdiff <= 2 as libc::c_int {
            if (xdiff != 0 as libc::c_int || ydiff != 0 as libc::c_int) &&
                   fpathBlockingTile.expect("non-null function pointer")(mapX
                                                                             +
                                                                             xdiff,
                                                                         mapY
                                                                             +
                                                                             ydiff)
                       != 0 {
                tx = xdiff << 7 as libc::c_int;
                ty = ydiff << 7 as libc::c_int;
                td = tx * tx + ty * ty;
                fDx = tx as FRACT;
                fDy = ty as FRACT;
                if td <
                       128 as libc::c_int * 2 as libc::c_int *
                           (128 as libc::c_int * 2 as libc::c_int) {
                    moveUpdateRepulsiveVector(fDx, fDy, *pX, *pY, &mut dirX,
                                              &mut dirY, &mut fDistTot);
                    numObst += 1
                }
            }
            xdiff += 1
        }
        ydiff += 1
    }
    if numObst > 0 as libc::c_int {
        fDistTot /= numObst as libc::c_float;
        // Create the avoid vector
        if dirX == 0 as libc::c_int as FRACT &&
               dirY == 0 as libc::c_int as FRACT {
            avoidX = 0 as libc::c_int as FRACT;
            avoidY = 0 as libc::c_int as FRACT;
            fDistTot =
                (128 as libc::c_int * 2 as libc::c_int *
                     (128 as libc::c_int * 2 as libc::c_int)) as FRACT
        } else {
            omag =
                sqrt((dirX * dirX + dirY * dirY) as libc::c_double) as FRACT;
            avoidX = dirX / omag;
            avoidY = dirY / omag
        }
        // combine the avoid vector and the target vector
        ratio =
            fDistTot /
                (128 as libc::c_int * 2 as libc::c_int *
                     (128 as libc::c_int * 2 as libc::c_int)) as FRACT;
        if ratio > 1 as libc::c_int as FRACT {
            ratio = 1 as libc::c_int as FRACT
        }
        *pX =
            *pX * ratio +
                avoidX * (1 as libc::c_int as libc::c_float - ratio);
        *pY =
            *pY * ratio + avoidY * (1 as libc::c_int as libc::c_float - ratio)
    };
}
/* Get a direction for a droid to avoid obstacles etc. */
// This routine smells ...
unsafe extern "C" fn moveGetDirection(mut psDroid: *mut DROID,
                                      mut pX: *mut FRACT,
                                      mut pY: *mut FRACT) {
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut mag: SDWORD = 0;
    let mut root: FRACT = 0.;
    let mut bNoVector: BOOL = 0;
    let mut ndx: SDWORD = 0;
    let mut ndy: SDWORD = 0;
    let mut ntx: SDWORD = 0;
    let mut nty: SDWORD = 0;
    let mut nmag: SDWORD = 0;
    let mut nroot: FRACT = 0.;
    tx = (*psDroid).sMove.targetX;
    ty = (*psDroid).sMove.targetY;
    // Calc the basic vector
    dx = tx - (*psDroid).x as SDWORD;
    dy = ty - (*psDroid).y as SDWORD;
    // If the droid is getting close to the way point start to phase in the next target
    mag = dx * dx + dy * dy;
    bNoVector = 1 as libc::c_int;
    // fade in the next target point if we arn't at the end of the waypoints
    if (*psDroid).sMove.Position as libc::c_int !=
           (*psDroid).sMove.numPoints as libc::c_int &&
           mag <
               128 as libc::c_int * 2 as libc::c_int *
                   (128 as libc::c_int * 2 as libc::c_int) {
        // find the next target
        movePeekNextTarget(psDroid, &mut ntx, &mut nty);
        ndx = ntx - (*psDroid).x as SDWORD;
        ndy = nty - (*psDroid).y as SDWORD;
        nmag = ndx * ndx + ndy * ndy;
        if mag != 0 as libc::c_int && nmag != 0 as libc::c_int {
            // Get the size of the vectors
            root = sqrt(mag as FRACT as libc::c_double) as FRACT;
            nroot = sqrt(nmag as FRACT as libc::c_double) as FRACT;
            // Split the proportion of the vectors based on how close to the point they are
            ndx =
                ndx *
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int) - mag) /
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int));
            ndy =
                ndy *
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int) - mag) /
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int));
            dx =
                dx * mag /
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int));
            dy =
                dy * mag /
                    (128 as libc::c_int * 2 as libc::c_int *
                         (128 as libc::c_int * 2 as libc::c_int));
            // Calculate the normalised result
            *pX = dx as FRACT / root + ndx as FRACT / nroot;
            *pY = dy as FRACT / root + ndy as FRACT / nroot;
            bNoVector = 0 as libc::c_int
        }
    }
    if bNoVector != 0 {
        root = sqrt(mag as FRACT as libc::c_double) as FRACT;
        *pX = dx as FRACT / root;
        *pY = dy as FRACT / root
    }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        moveGetObstVector4(psDroid, pX, pY);
    };
}
// Calculate the boundary vector
// Calculate the boundary vector
#[no_mangle]
pub unsafe extern "C" fn moveCalcBoundary(mut psDroid: *mut DROID) {
    let mut absX: SDWORD = 0;
    let mut absY: SDWORD = 0;
    let mut prevX: SDWORD = 0;
    let mut prevY: SDWORD = 0;
    let mut prevMag: SDWORD = 0;
    let mut nTarX: SDWORD = 0;
    let mut nTarY: SDWORD = 0;
    let mut nextX: SDWORD = 0;
    let mut nextY: SDWORD = 0;
    let mut nextMag: SDWORD = 0;
    let mut sumX: SDWORD = 0;
    let mut sumY: SDWORD = 0;
    // No points left - simple case for the bound vector
    if (*psDroid).sMove.Position as libc::c_int ==
           (*psDroid).sMove.numPoints as libc::c_int {
        (*psDroid).sMove.boundX =
            ((*psDroid).sMove.srcX - (*psDroid).sMove.targetX) as SWORD;
        (*psDroid).sMove.boundY =
            ((*psDroid).sMove.srcY - (*psDroid).sMove.targetY) as SWORD;
        return
    }
    // Calculate the vector back along the current path
    prevX = (*psDroid).sMove.srcX - (*psDroid).sMove.targetX;
    prevY = (*psDroid).sMove.srcY - (*psDroid).sMove.targetY;
    absX = labs(prevX as libc::c_long) as SDWORD;
    absY = labs(prevY as libc::c_long) as SDWORD;
    prevMag =
        if absX > absY {
            (absX) + absY / 2 as libc::c_int
        } else { (absY) + absX / 2 as libc::c_int };
    //	prevMag = sqrt(prevX*prevX + prevY*prevY);
    // Calculate the vector to the next target
    movePeekNextTarget(psDroid, &mut nTarX, &mut nTarY);
    nextX = nTarX - (*psDroid).sMove.targetX;
    nextY = nTarY - (*psDroid).sMove.targetY;
    absX = labs(nextX as libc::c_long) as SDWORD;
    absY = labs(nextY as libc::c_long) as SDWORD;
    nextMag =
        if absX > absY {
            (absX) + absY / 2 as libc::c_int
        } else { (absY) + absX / 2 as libc::c_int };
    //	nextMag = sqrt(nextX*nextX + nextY*nextY);
    if prevMag != 0 as libc::c_int && nextMag == 0 as libc::c_int {
        // don't bother mixing the boundaries - cos there isn't a next vector anyway
        (*psDroid).sMove.boundX =
            ((*psDroid).sMove.srcX - (*psDroid).sMove.targetX) as SWORD;
        (*psDroid).sMove.boundY =
            ((*psDroid).sMove.srcY - (*psDroid).sMove.targetY) as SWORD;
        return
    } else {
        if prevMag == 0 as libc::c_int || nextMag == 0 as libc::c_int {
            (*psDroid).sMove.boundX = 0 as libc::c_int as SWORD;
            (*psDroid).sMove.boundY = 0 as libc::c_int as SWORD;
            return
        }
    }
    // Calculate the vector between the two
    sumX =
        prevX * 1000 as libc::c_int / prevMag +
            nextX * 1000 as libc::c_int / nextMag;
    sumY =
        prevY * 1000 as libc::c_int / prevMag +
            nextY * 1000 as libc::c_int / nextMag;
    // Rotate by 90 degrees one way and see if it is the same side as the src vector
	// if not rotate 90 the other.
    if prevX * sumY - prevY * sumX < 0 as libc::c_int {
        (*psDroid).sMove.boundX = -sumY as SWORD;
        (*psDroid).sMove.boundY = sumX as SWORD
    } else {
        (*psDroid).sMove.boundX = sumY as SWORD;
        (*psDroid).sMove.boundY = -sumX as SWORD
    };
}
// Check if a droid has got to a way point
#[no_mangle]
pub unsafe extern "C" fn moveReachedWayPoint(mut psDroid: *mut DROID)
 -> BOOL {
    let mut droidX: SDWORD = 0;
    let mut droidY: SDWORD = 0;
    let mut iRange: SDWORD = 0;
    // Calculate the vector to the droid
    droidX = (*psDroid).x as SDWORD - (*psDroid).sMove.targetX;
    droidY = (*psDroid).y as SDWORD - (*psDroid).sMove.targetY;
    // see if this is a formation end point
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
           !(*psDroid).sMove.psFormation.is_null() &&
               formationMember((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT) != 0 ||
           vtolDroid(psDroid) != 0 &&
               (*psDroid).sMove.numPoints as libc::c_int ==
                   (*psDroid).sMove.Position as libc::c_int {
        //							 && (psDroid->action != DACTION_VTOLATTACK)) )
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            iRange = 128 as libc::c_int / 4 as libc::c_int
        } else { iRange = 128 as libc::c_int / 4 as libc::c_int }
        if droidX * droidX + droidY * droidY < iRange * iRange {
            return 1 as libc::c_int
        }
    } else if (*psDroid).sMove.boundX as libc::c_int * droidX +
                  (*psDroid).sMove.boundY as libc::c_int * droidY <=
                  0 as libc::c_int &&
                  fpathTileLOS((*psDroid).x as SDWORD >> 7 as libc::c_int,
                               (*psDroid).y as SDWORD >> 7 as libc::c_int,
                               (*psDroid).sMove.targetX >> 7 as libc::c_int,
                               (*psDroid).sMove.targetY >> 7 as libc::c_int)
                      != 0 {
        // if the dot product is -ve the droid has got past the way point
		// but only move onto the next way point if we can see the previous one
		// (this helps units that have got nudged off course).
        //		DBPRINTF(("Waypoint %d\n", psDroid->sMove.Position));
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moveToggleFormationSpeedLimiting() {
    g_bFormationSpeedLimitingOn =
        (g_bFormationSpeedLimitingOn == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moveSetFormationSpeedLimiting(mut bVal: BOOL) {
    g_bFormationSpeedLimitingOn = bVal;
}
#[no_mangle]
pub unsafe extern "C" fn moveFormationSpeedLimitingOn() -> BOOL {
    return g_bFormationSpeedLimitingOn;
}
// Calculate the new speed for a droid
#[no_mangle]
pub unsafe extern "C" fn moveCalcDroidSpeed(mut psDroid: *mut DROID)
 -> SDWORD {
    let mut mapX: UDWORD = 0; //, tarSpeed;
    let mut mapY: UDWORD = 0;
    let mut damLevel: UDWORD = 0;
    let mut speed: SDWORD = 0;
    let mut pitch: SDWORD = 0;
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    mapX = ((*psDroid).x as libc::c_int >> 7 as libc::c_int) as UDWORD;
    mapY = ((*psDroid).y as libc::c_int >> 7 as libc::c_int) as UDWORD;
    speed =
        calcDroidSpeed((*psDroid).baseSpeed,
                       terrainTypes[((*mapTile(mapX, mapY)).texture as
                                         libc::c_int & 0x1ff as libc::c_int)
                                        as usize] as UDWORD,
                       (*psDroid).asBits[COMP_PROPULSION as libc::c_int as
                                             usize].nStat as UDWORD) as
            SDWORD;
    /*	if ( vtolDroid(psDroid) &&
		 ((asBodyStats + psDroid->asBits[COMP_BODY].nStat)->size == SIZE_HEAVY) )
	{
		speed /= 2;
	}*/
    pitch = (*psDroid).pitch as SDWORD;
    if pitch > 60 as libc::c_int {
        pitch = 60 as libc::c_int
    } else if pitch < -(60 as libc::c_int) { pitch = -(60 as libc::c_int) }
    // now offset the speed for the slope of the droid
    speed = (60 as libc::c_int - pitch) * speed / 60 as libc::c_int;
    //#ifdef PSX
//	pitch=0;		// hack for the demo
//#endif
    // slow down damaged droids
    damLevel =
        (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                         libc::c_uint).wrapping_div((*psDroid).originalBody);
    if damLevel < 25 as libc::c_int as libc::c_uint {
        speed = 2 as libc::c_int * speed / 3 as libc::c_int
    }
    // stop droids that have just fired a no fire while moving weapon
	//if (psDroid->numWeaps > 0 && psDroid->asWeaps[0].lastFired + FOM_MOVEPAUSE > gameTime)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint &&
           (*psDroid).asWeaps[0 as libc::c_int as
                                  usize].lastFired.wrapping_add(1500 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
               > gameTime {
        psWStats =
            asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                        usize].nStat as
                                     isize);
        if (*psWStats).fireOnMove as libc::c_uint ==
               FOM_NO as libc::c_int as libc::c_uint {
            speed = 0 as libc::c_int
        }
    }
    /* adjust speed for formation */
    if vtolDroid(psDroid) == 0 && moveFormationSpeedLimitingOn() != 0 &&
           !(*psDroid).sMove.psFormation.is_null() {
        let mut FrmSpeed: SDWORD =
            (*(*psDroid).sMove.psFormation).iSpeed as SDWORD;
        if speed > FrmSpeed { speed = FrmSpeed }
    }
    // slow down shuffling VTOLs
    if vtolDroid(psDroid) != 0 &&
           (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int &&
           speed > 60 as libc::c_int {
        speed = 60 as libc::c_int
    }
    //	/* adjust speed for formation */
//	if ( moveFormationSpeedLimitingOn() &&
//		 psDroid->sMove.psFormation &&
//		 speed > (SDWORD)psDroid->sMove.psFormation->iSpeed )
//	{
//		speed = psDroid->sMove.psFormation->iSpeed;
//	}
    //#if(1)
//	if(psDroid->selected) {
//		printf("%d : %d : %d\n",driveGetSpeed(),psDroid->baseSpeed,speed);
//	}
//#endif
    return speed;
}
#[no_mangle]
pub unsafe extern "C" fn moveDroidStopped(mut psDroid: *mut DROID,
                                          mut speed: SDWORD) -> BOOL {
    if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
            (*psDroid).sMove.Status as libc::c_int == 7 as libc::c_int) &&
           speed == 0 as libc::c_int &&
           (*psDroid).sMove.speed == 0 as libc::c_int as FRACT {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn moveUpdateDroidDirection(mut psDroid: *mut DROID,
                                                  mut pSpeed: *mut SDWORD,
                                                  mut direction: SDWORD,
                                                  mut iSpinAngle: SDWORD,
                                                  mut iSpinSpeed: SDWORD,
                                                  mut iTurnSpeed: SDWORD,
                                                  mut pDroidDir: *mut SDWORD,
                                                  mut pfSpeed: *mut FRACT) {
    let mut adiff: SDWORD = 0;
    let mut temp: FRACT = 0.;
    *pfSpeed = *pSpeed as FRACT;
    *pDroidDir = (*psDroid).direction as SDWORD;
    // don't move if in MOVEPAUSE state
    if (*psDroid).sMove.Status as libc::c_int == 3 as libc::c_int { return }
    temp = *pDroidDir as FRACT;
    adiff = labs((direction - *pDroidDir) as libc::c_long) as SDWORD;
    if adiff > 360 as libc::c_int / 2 as libc::c_int {
        adiff = 360 as libc::c_int - adiff
    }
    if adiff > iSpinAngle {
        // large change in direction, spin on the spot
        moveCalcTurn(&mut temp, direction as FRACT, iSpinSpeed as UDWORD);
        *pSpeed = 0 as libc::c_int
    } else {
        // small change in direction, turn while moving
        moveCalcTurn(&mut temp, direction as FRACT, iTurnSpeed as UDWORD);
    }
    *pDroidDir = temp as SDWORD;
}
// Calculate current speed perpendicular to droids direction
#[no_mangle]
pub unsafe extern "C" fn moveCalcPerpSpeed(mut psDroid: *mut DROID,
                                           mut iDroidDir: SDWORD,
                                           mut iSkidDecel: SDWORD) -> FRACT {
    let mut adiff: SDWORD = 0;
    let mut perpSpeed: FRACT = 0.;
    adiff =
        labs((iDroidDir - (*psDroid).sMove.dir as libc::c_int) as
                 libc::c_long) as SDWORD;
    perpSpeed = (*psDroid).sMove.speed * trigSin(adiff);
    // decelerate the perpendicular speed
    perpSpeed -= iSkidDecel as libc::c_float * baseSpeed;
    if perpSpeed < 0 as libc::c_int as FRACT {
        perpSpeed = 0 as libc::c_int as FRACT
    }
    return perpSpeed;
}
#[no_mangle]
pub unsafe extern "C" fn moveCombineNormalAndPerpSpeeds(mut psDroid:
                                                            *mut DROID,
                                                        mut fNormalSpeed:
                                                            FRACT,
                                                        mut fPerpSpeed: FRACT,
                                                        mut iDroidDir:
                                                            SDWORD) {
    let mut finalDir: SDWORD = 0;
    let mut adiff: SDWORD = 0;
    let mut finalSpeed: FRACT = 0.;
    /* set current direction */
    (*psDroid).direction = iDroidDir as UWORD;
    /* set normal speed and direction if perpendicular speed is zero */
    if fPerpSpeed == 0 as libc::c_int as FRACT {
        (*psDroid).sMove.speed = fNormalSpeed;
        (*psDroid).sMove.dir = iDroidDir as SWORD;
        return
    }
    finalSpeed =
        sqrt((fNormalSpeed * fNormalSpeed + fPerpSpeed * fPerpSpeed) as
                 libc::c_double) as FRACT;
    // calculate the angle between the droid facing and movement direction
    finalDir = trigInvCos(fNormalSpeed / finalSpeed) as SDWORD;
    // choose the finalDir on the same side as the old movement direction
    adiff =
        labs((iDroidDir - (*psDroid).sMove.dir as libc::c_int) as
                 libc::c_long) as SDWORD;
    if adiff < 360 as libc::c_int / 2 as libc::c_int {
        if iDroidDir > (*psDroid).sMove.dir as libc::c_int {
            finalDir = iDroidDir - finalDir
        } else { finalDir = iDroidDir + finalDir }
    } else if iDroidDir > (*psDroid).sMove.dir as libc::c_int {
        finalDir = iDroidDir + finalDir;
        if finalDir >= 360 as libc::c_int { finalDir -= 360 as libc::c_int }
    } else {
        finalDir = iDroidDir - finalDir;
        if finalDir < 0 as libc::c_int { finalDir += 360 as libc::c_int }
    }
    (*psDroid).sMove.dir = finalDir as SWORD;
    (*psDroid).sMove.speed = finalSpeed;
}
// Calculate the current speed in the droids normal direction
#[no_mangle]
pub unsafe extern "C" fn moveCalcNormalSpeed(mut psDroid: *mut DROID,
                                             mut fSpeed: FRACT,
                                             mut iDroidDir: SDWORD,
                                             mut iAccel: SDWORD,
                                             mut iDecel: SDWORD) -> FRACT {
    let mut adiff: SDWORD = 0;
    let mut normalSpeed: FRACT = 0.;
    adiff =
        labs((iDroidDir - (*psDroid).sMove.dir as libc::c_int) as
                 libc::c_long) as SDWORD;
    normalSpeed = (*psDroid).sMove.speed * trigCos(adiff);
    if normalSpeed < fSpeed {
        // accelerate
        normalSpeed += iAccel as libc::c_float * baseSpeed;
        if normalSpeed > fSpeed { normalSpeed = fSpeed }
    } else {
        // decelerate
        normalSpeed -= iDecel as libc::c_float * baseSpeed;
        if normalSpeed < fSpeed { normalSpeed = fSpeed }
    }
    return normalSpeed;
}
#[no_mangle]
pub unsafe extern "C" fn moveGetDroidPosDiffs(mut psDroid: *mut DROID,
                                              mut pDX: *mut FRACT,
                                              mut pDY: *mut FRACT) {
    let mut move_0: FRACT = 0.;
    move_0 = (*psDroid).sMove.speed * baseSpeed;
    *pDX = move_0 * trigSin((*psDroid).sMove.dir as SDWORD);
    *pDY = move_0 * trigCos((*psDroid).sMove.dir as SDWORD);
}
// see if the droid is close to the final way point
#[no_mangle]
pub unsafe extern "C" fn moveCheckFinalWaypoint(mut psDroid: *mut DROID,
                                                mut pSpeed: *mut SDWORD) {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut distSq: SDWORD = 0;
    let mut minEndSpeed: SDWORD =
        (*psDroid).baseSpeed.wrapping_div(3 as libc::c_int as libc::c_uint) as
            SDWORD;
    if minEndSpeed > 60 as libc::c_int { minEndSpeed = 60 as libc::c_int }
    // don't do this for VTOLs doing attack runs
    if vtolDroid(psDroid) != 0 &&
           (*psDroid).action == DACTION_VTOLATTACK as libc::c_int {
        return
    }
    if *pSpeed > minEndSpeed &&
           (*psDroid).sMove.Status as libc::c_int != 12 as libc::c_int &&
           (*psDroid).sMove.Position as libc::c_int ==
               (*psDroid).sMove.numPoints as libc::c_int {
        xdiff = (*psDroid).x as SDWORD - (*psDroid).sMove.targetX;
        ydiff = (*psDroid).y as SDWORD - (*psDroid).sMove.targetY;
        distSq = xdiff * xdiff + ydiff * ydiff;
        if distSq <
               3 as libc::c_int * 128 as libc::c_int *
                   (3 as libc::c_int * 128 as libc::c_int) {
            *pSpeed =
                (300 as libc::c_int - minEndSpeed) * distSq /
                    (3 as libc::c_int * 128 as libc::c_int *
                         (3 as libc::c_int * 128 as libc::c_int)) +
                    minEndSpeed
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn moveUpdateDroidPos(mut psDroid: *mut DROID,
                                            mut dx: FRACT, mut dy: FRACT) {
    let mut iX: SDWORD = 0 as libc::c_int;
    let mut iY: SDWORD = 0 as libc::c_int;
    if (*psDroid).sMove.Status as libc::c_int == 3 as libc::c_int {
        // don't actually move if the move is paused
        return
    }
    (*psDroid).sMove.fx += dx;
    (*psDroid).sMove.fy += dy;
    //	psDroid->sMove.dx = dx;
//	psDroid->sMove.dy = dy;
    //#ifdef PSX
//	psDroid->x = (UDWORD)MAKEINT(psDroid->sMove.fx);
//	psDroid->y = (UDWORD)MAKEINT(psDroid->sMove.fy);
//
//	if ( psDroid->x > 0x80000000)
//	{
//		DBPRINTF(("Droid off edge of map ... fixing (a)\n"));
//		psDroid->x=1;
//	}
//	else
//	{
//		if ( psDroid->x > mapWidth*TILE_UNITS )
//		{
//			DBPRINTF(("Droid off edge of map ... fixing (b)\n"));
//			psDroid->x= mapWidth*TILE_UNITS-1;
//
//		}
//	}
//
//
//	if ( psDroid->y > 0x80000000)
//	{
//		DBPRINTF(("Droid off edge of map ... fixing (c)\n"));
//		psDroid->y=1;
//	}
//	else
//	{
//		if ( psDroid->y > mapHeight*TILE_UNITS )
//		{
//			DBPRINTF(("Droid off edge of map ... fixing (d)\n"));
//			psDroid->y= mapHeight*TILE_UNITS-1;
//
//		}
//	}
//#else
    iX = (*psDroid).sMove.fx as SDWORD;
    iY = (*psDroid).sMove.fy as SDWORD;
    /* impact if about to go off map else update coordinates */
    if worldOnMap(iX, iY) == 0 as libc::c_int {
        if !((*psDroid).droidType as libc::c_uint ==
                 DROID_TRANSPORTER as libc::c_int as libc::c_uint) {
            /* dreadful last-ditch crash-avoiding hack - sort this! - GJ */
            debug(LOG_NEVER,
                  b"**** droid about to go off map - fixed ****\n\x00" as
                      *const u8 as *const libc::c_char);
            destroyDroid(psDroid);
        }
    } else { (*psDroid).x = iX as UWORD; (*psDroid).y = iY as UWORD }
    // lovely hack to keep transporters just on the map
	// two weeks to go and the hacks just get better !!!
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if (*psDroid).x as libc::c_int == 0 as libc::c_int {
            (*psDroid).x = 1 as libc::c_int as UWORD
        }
        if (*psDroid).y as libc::c_int == 0 as libc::c_int {
            (*psDroid).y = 1 as libc::c_int as UWORD
        }
    };
    //#endif
}
/* Update a tracked droids position and speed given target values */
#[no_mangle]
pub unsafe extern "C" fn moveUpdateGroundModel(mut psDroid: *mut DROID,
                                               mut speed: SDWORD,
                                               mut direction: SDWORD) {
    let mut fPerpSpeed: FRACT = 0.;
    let mut fNormalSpeed: FRACT = 0.;
    let mut dx: FRACT = 0.;
    let mut dy: FRACT = 0.;
    let mut fSpeed: FRACT = 0.;
    let mut bx: FRACT = 0.;
    let mut by: FRACT = 0.;
    let mut iDroidDir: SDWORD = 0;
    let mut slideDir: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut spinSpeed: SDWORD = 0;
    let mut turnSpeed: SDWORD = 0;
    let mut skidDecel: SDWORD = 0;
    // constants for the different propulsion types
    static mut hvrSkid: SDWORD = 120 as libc::c_int; //0.75f;
    static mut whlSkid: SDWORD = 350 as libc::c_int; //1.0f;
    static mut trkSkid: SDWORD = 600 as libc::c_int; //1.0f;
    static mut hvrTurn: FRACT =
        3 as libc::c_int as libc::c_float /
            4 as libc::c_int as libc::c_float; //HOVER_SKID_DECEL;
    static mut whlTurn: FRACT =
        1 as libc::c_int as libc::c_float /
            1 as libc::c_int as libc::c_float; //WHEELED_SKID_DECEL;
    static mut trkTurn: FRACT =
        1 as libc::c_int as libc::c_float /
            1 as libc::c_int as libc::c_float; //TRACKED_SKID_DECEL;
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    match (*psPropStats).propulsionType as libc::c_int {
        3 => {
            spinSpeed =
                ((*psDroid).baseSpeed as libc::c_float * hvrTurn) as SDWORD;
            turnSpeed =
                ((*psDroid).baseSpeed.wrapping_div(3 as libc::c_int as
                                                       libc::c_uint) as
                     libc::c_float * hvrTurn) as SDWORD;
            skidDecel = hvrSkid
        }
        0 => {
            spinSpeed =
                ((*psDroid).baseSpeed as libc::c_float * hvrTurn) as SDWORD;
            turnSpeed =
                ((*psDroid).baseSpeed.wrapping_div(3 as libc::c_int as
                                                       libc::c_uint) as
                     libc::c_float * whlTurn) as SDWORD;
            skidDecel = whlSkid
        }
        1 | _ => {
            spinSpeed =
                ((*psDroid).baseSpeed as libc::c_float * hvrTurn) as SDWORD;
            turnSpeed =
                ((*psDroid).baseSpeed.wrapping_div(3 as libc::c_int as
                                                       libc::c_uint) as
                     libc::c_float * trkTurn) as SDWORD;
            skidDecel = trkSkid
        }
    }
    // nothing to do if the droid is stopped
    if moveDroidStopped(psDroid, speed) == 1 as libc::c_int { return }
    // update the naybors list
    droidGetNaybors(psDroid);
    moveCheckFinalWaypoint(psDroid, &mut speed);
    //	moveUpdateDroidDirection( psDroid, &speed, direction, TRACKED_SPIN_ANGLE,
//				TRACKED_SPIN_SPEED, TRACKED_TURN_SPEED, &iDroidDir, &fSpeed );
    moveUpdateDroidDirection(psDroid, &mut speed, direction,
                             360 as libc::c_int / 8 as libc::c_int, spinSpeed,
                             turnSpeed, &mut iDroidDir, &mut fSpeed);
    fNormalSpeed =
        moveCalcNormalSpeed(psDroid, fSpeed, iDroidDir, 250 as libc::c_int,
                            800 as libc::c_int);
    fPerpSpeed = moveCalcPerpSpeed(psDroid, iDroidDir, skidDecel);
    moveCombineNormalAndPerpSpeeds(psDroid, fNormalSpeed, fPerpSpeed,
                                   iDroidDir);
    //	if (psDroid->direction != psDroid->sMove.dir)
/*	if (fPerpSpeed > 0)
	{
		DBPRINTF(("droid %d direction %d total dir %d perpspeed %f\n",
			psDroid->id, psDroid->direction, psDroid->sMove.dir, fPerpSpeed));
	}*/
    moveGetDroidPosDiffs(psDroid, &mut dx, &mut dy);
    moveCheckSquished(psDroid, dx, dy);
    moveCalcDroidSlide(psDroid, &mut dx, &mut dy);
    bx = dx;
    by = dy;
    moveCalcBlockingSlide(psDroid, &mut bx, &mut by, direction,
                          &mut slideDir);
    if bx != dx || by != dy {
        moveUpdateDroidDirection(psDroid, &mut speed, slideDir,
                                 360 as libc::c_int / 8 as libc::c_int,
                                 (*psDroid).baseSpeed as SDWORD,
                                 (*psDroid).baseSpeed.wrapping_div(3 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                                     as SDWORD, &mut iDroidDir, &mut fSpeed);
        (*psDroid).direction = iDroidDir as UWORD
    }
    moveUpdateDroidPos(psDroid, bx, by);
    //set the droid height here so other routines can use it
    (*psDroid).z =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
            UWORD; //jps 21july96
    updateDroidOrientation(psDroid);
}
/* Update a persons position and speed given target values */
#[no_mangle]
pub unsafe extern "C" fn moveUpdatePersonModel(mut psDroid: *mut DROID,
                                               mut speed: SDWORD,
                                               mut direction: SDWORD) {
    let mut fPerpSpeed: FRACT = 0.;
    let mut fNormalSpeed: FRACT = 0.;
    let mut dx: FRACT = 0.;
    let mut dy: FRACT = 0.;
    let mut fSpeed: FRACT = 0.;
    let mut iDroidDir: SDWORD = 0;
    let mut slideDir: SDWORD = 0;
    //	BASE_OBJECT		*psObst;
    let mut bRet: BOOL = 0;
    // nothing to do if the droid is stopped
    if moveDroidStopped(psDroid, speed) == 1 as libc::c_int {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_PERSON as libc::c_int as libc::c_uint &&
               (*psDroid).order != DORDER_RUNBURN as libc::c_int &&
               ((*psDroid).action == DACTION_ATTACK as libc::c_int ||
                    (*psDroid).action ==
                        DACTION_ROTATETOATTACK as libc::c_int) {
            /* remove previous anim */
            if !(*psDroid).psCurAnim.is_null() &&
                   (*(*(*psDroid).psCurAnim).psAnim).uwID as libc::c_int !=
                       1 as libc::c_int {
                bRet =
                    animObj_Remove(&mut (*psDroid).psCurAnim,
                                   (*(*(*psDroid).psCurAnim).psAnim).uwID as
                                       libc::c_int);
                if bRet == 1 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"moveUpdatePersonModel: animObj_Remove failed\x00"
                              as *const u8 as *const libc::c_char);
                };
                if bRet == 1 as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"move.c\x00" as *const u8 as *const libc::c_char,
                          3663 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 22],
                                                    &[libc::c_char; 22]>(b"moveUpdatePersonModel\x00")).as_ptr(),
                          b"bRet == TRUE\x00" as *const u8 as
                              *const libc::c_char);
                };
                (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
            }
            /* add firing anim */
            if (*psDroid).psCurAnim.is_null() {
                (*psDroid).psCurAnim =
                    animObj_Add(psDroid as *mut libc::c_void,
                                1 as libc::c_int, 0 as libc::c_int as UDWORD,
                                0 as libc::c_int as UWORD)
            } else { (*(*psDroid).psCurAnim).bVisible = 1 as libc::c_int }
            return
        }
        /* don't show move animations if inactive */
        if !(*psDroid).psCurAnim.is_null() {
            //			DBPRINTF(("droid anim stopped %p\n",psDroid);
//DBPRINTF(("vis 1 off %p\n",psDroid);
            // On the pc we make the animation non-visible
//
// on the psx we remove the animation totally, and reallocate it when it is next needed
//    ... this is so we can allow the playstation to use far fewer animation entries
            (*(*psDroid).psCurAnim).bVisible = 0 as libc::c_int
        }
        return
    }
    // update the naybors list
    droidGetNaybors(psDroid);
    moveUpdateDroidDirection(psDroid, &mut speed, direction,
                             360 as libc::c_int / 8 as libc::c_int,
                             500 as libc::c_int, 250 as libc::c_int,
                             &mut iDroidDir, &mut fSpeed);
    fNormalSpeed =
        moveCalcNormalSpeed(psDroid, fSpeed, iDroidDir, 250 as libc::c_int,
                            450 as libc::c_int);
    /* people don't skid at the moment so set zero perpendicular speed */
    fPerpSpeed = 0 as libc::c_int as FRACT;
    moveCombineNormalAndPerpSpeeds(psDroid, fNormalSpeed, fPerpSpeed,
                                   iDroidDir);
    moveGetDroidPosDiffs(psDroid, &mut dx, &mut dy);
    /*	if (moveFindObstacle(psDroid, dx,dy, &psObst))
	{
		moveCalcSlideVector(psDroid, (SDWORD)psObst->x, (SDWORD)psObst->y, &dx, &dy);
	}*/
    moveCalcDroidSlide(psDroid, &mut dx, &mut dy);
    moveCalcBlockingSlide(psDroid, &mut dx, &mut dy, direction,
                          &mut slideDir);
    moveUpdateDroidPos(psDroid, dx, dy);
    //set the droid height here so other routines can use it
    (*psDroid).z =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
            UWORD; //jps 21july96
    (*psDroid).sMove.fz = (*psDroid).z as FRACT;
    /* update anim if moving and not on fire */
    if (*psDroid).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint &&
           speed != 0 as libc::c_int &&
           (*psDroid).order != DORDER_RUNBURN as libc::c_int {
        /* remove previous anim */
        if !(*psDroid).psCurAnim.is_null() &&
               ((*(*(*psDroid).psCurAnim).psAnim).uwID as libc::c_int !=
                    2 as libc::c_int ||
                    (*(*(*psDroid).psCurAnim).psAnim).uwID as libc::c_int !=
                        2 as libc::c_int) {
            bRet =
                animObj_Remove(&mut (*psDroid).psCurAnim,
                               (*(*(*psDroid).psCurAnim).psAnim).uwID as
                                   libc::c_int);
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"moveUpdatePersonModel: animObj_Remove failed\x00" as
                          *const u8 as *const libc::c_char);
            };
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      3743 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"moveUpdatePersonModel\x00")).as_ptr(),
                      b"bRet == TRUE\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
        }
        /* if no anim currently attached, get one */
        if (*psDroid).psCurAnim.is_null() {
            // Only add the animation if the droid is on screen, saves memory and time.
            if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) != 0 {
                debug(LOG_NEVER,
                      b"Added person run anim\n\x00" as *const u8 as
                          *const libc::c_char);
                (*psDroid).psCurAnim =
                    animObj_Add(psDroid as *mut BASE_OBJECT as
                                    *mut libc::c_void, 2 as libc::c_int,
                                0 as libc::c_int as UDWORD,
                                0 as libc::c_int as UWORD)
            }
        } else if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) == 0
         {
            bRet =
                animObj_Remove(&mut (*psDroid).psCurAnim,
                               (*(*(*psDroid).psCurAnim).psAnim).uwID as
                                   libc::c_int);
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"moveUpdatePersonModel : animObj_Remove failed\x00" as
                          *const u8 as *const libc::c_char);
            };
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      3760 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"moveUpdatePersonModel\x00")).as_ptr(),
                      b"bRet == TRUE\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT;
            debug(LOG_NEVER,
                  b"Removed person run anim\n\x00" as *const u8 as
                      *const libc::c_char);
        }
    }
    // If the droid went off screen then remove the animation, saves memory and time.
    /* show anim */
    if !(*psDroid).psCurAnim.is_null() {
        (*(*psDroid).psCurAnim).bVisible = 1 as libc::c_int
    };
}
/* primitive 'bang-bang' vtol height controller */
#[no_mangle]
pub unsafe extern "C" fn moveAdjustVtolHeight(mut psDroid: *mut DROID,
                                              mut iMapHeight: UDWORD) {
    let mut iMinHeight: UDWORD = 0;
    let mut iMaxHeight: UDWORD = 0;
    let mut iLevelHeight: UDWORD = 0;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        iMinHeight = (2 as libc::c_int * 250 as libc::c_int) as UDWORD;
        iLevelHeight = (2 as libc::c_int * 300 as libc::c_int) as UDWORD;
        iMaxHeight = (2 as libc::c_int * 350 as libc::c_int) as UDWORD
    } else {
        iMinHeight = 250 as libc::c_int as UDWORD;
        iLevelHeight = 300 as libc::c_int as UDWORD;
        iMaxHeight = 350 as libc::c_int as UDWORD
    }
    if (*psDroid).z as libc::c_uint >= iMapHeight.wrapping_add(iMaxHeight) {
        (*psDroid).sMove.iVertSpeed =
            -if (*psDroid).baseSpeed as SDWORD / 4 as libc::c_int >
                    60 as libc::c_int {
                 ((*psDroid).baseSpeed as SDWORD) / 4 as libc::c_int
             } else { 60 as libc::c_int } as SWORD
    } else if ((*psDroid).z as libc::c_uint) <
                  iMapHeight.wrapping_add(iMinHeight) {
        (*psDroid).sMove.iVertSpeed =
            if (*psDroid).baseSpeed as SDWORD / 4 as libc::c_int >
                   60 as libc::c_int {
                ((*psDroid).baseSpeed as SDWORD) / 4 as libc::c_int
            } else { 60 as libc::c_int } as SWORD
    } else if ((*psDroid).z as libc::c_uint) < iLevelHeight &&
                  ((*psDroid).sMove.iVertSpeed as libc::c_int) <
                      0 as libc::c_int {
        (*psDroid).sMove.iVertSpeed = 0 as libc::c_int as SWORD
    } else if (*psDroid).z as libc::c_uint > iLevelHeight &&
                  (*psDroid).sMove.iVertSpeed as libc::c_int >
                      0 as libc::c_int {
        (*psDroid).sMove.iVertSpeed = 0 as libc::c_int as SWORD
    };
}
// set a vtol to be hovering in the air
// set a vtol to be hovering in the air
#[no_mangle]
pub unsafe extern "C" fn moveMakeVtolHover(mut psDroid: *mut DROID) {
    (*psDroid).sMove.Status = 8 as libc::c_int as UBYTE;
    (*psDroid).z =
        (map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
             libc::c_int + 300 as libc::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn moveUpdateVtolModel(mut psDroid: *mut DROID,
                                             mut speed: SDWORD,
                                             mut direction: SDWORD) {
    let mut fPerpSpeed: FRACT = 0.;
    let mut fNormalSpeed: FRACT = 0.;
    let mut dx: FRACT = 0.;
    let mut dy: FRACT = 0.;
    let mut fSpeed: FRACT = 0.;
    let mut iDroidDir: SDWORD = 0;
    let mut iMapZ: SDWORD = 0;
    let mut iRoll: SDWORD = 0;
    let mut slideDir: SDWORD = 0;
    let mut iSpinSpeed: SDWORD = 0;
    let mut iTurnSpeed: SDWORD = 0;
    //	SDWORD iDZ, iDroidZ;
    let mut fDZ: FRACT = 0.;
    let mut fDroidZ: FRACT = 0.;
    let mut fMapZ: FRACT = 0.;
    // nothing to do if the droid is stopped
    if moveDroidStopped(psDroid, speed) == 1 as libc::c_int { return }
    // update the naybors list
    droidGetNaybors(psDroid);
    moveCheckFinalWaypoint(psDroid, &mut speed);
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        moveUpdateDroidDirection(psDroid, &mut speed, direction,
                                 360 as libc::c_int, 100 as libc::c_int,
                                 100 as libc::c_int, &mut iDroidDir,
                                 &mut fSpeed);
    } else {
        iSpinSpeed =
            if (*psDroid).baseSpeed.wrapping_div(2 as libc::c_int as
                                                     libc::c_uint) >
                   100 as libc::c_int as libc::c_uint {
                (*psDroid).baseSpeed.wrapping_div(2 as libc::c_int as
                                                      libc::c_uint)
            } else { 100 as libc::c_int as libc::c_uint } as SDWORD;
        iTurnSpeed =
            if (*psDroid).baseSpeed.wrapping_div(8 as libc::c_int as
                                                     libc::c_uint) >
                   100 as libc::c_int as libc::c_uint {
                (*psDroid).baseSpeed.wrapping_div(8 as libc::c_int as
                                                      libc::c_uint)
            } else { 100 as libc::c_int as libc::c_uint } as SDWORD;
        moveUpdateDroidDirection(psDroid, &mut speed, direction,
                                 360 as libc::c_int, iSpinSpeed, iTurnSpeed,
                                 &mut iDroidDir, &mut fSpeed);
    }
    fNormalSpeed =
        moveCalcNormalSpeed(psDroid, fSpeed, iDroidDir, 200 as libc::c_int,
                            200 as libc::c_int);
    fPerpSpeed = moveCalcPerpSpeed(psDroid, iDroidDir, 600 as libc::c_int);
    moveCombineNormalAndPerpSpeeds(psDroid, fNormalSpeed, fPerpSpeed,
                                   iDroidDir);
    moveGetDroidPosDiffs(psDroid, &mut dx, &mut dy);
    /* set slide blocking tile for map edge */
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        fpathBlockingTile =
            Some(fpathLiftSlideBlockingTile as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
        moveCalcBlockingSlide(psDroid, &mut dx, &mut dy, direction,
                              &mut slideDir);
        fpathBlockingTile =
            Some(fpathGroundBlockingTile as
                     unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL)
    }
    moveUpdateDroidPos(psDroid, dx, dy);
    /* update vtol orientation */
    iRoll =
        ((*psDroid).sMove.dir as libc::c_int - (*psDroid).direction as SDWORD)
            / 3 as libc::c_int;
    if iRoll < 0 as libc::c_int { iRoll += 360 as libc::c_int }
    (*psDroid).roll = iRoll as UWORD as SWORD;
    iMapZ =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as SDWORD;
    /* do vertical movement */
    fDZ =
        ((*psDroid).sMove.iVertSpeed as libc::c_int * frameTime as SDWORD) as
            FRACT / 1000 as libc::c_int as libc::c_float;
    fDroidZ = (*psDroid).sMove.fz;
    fMapZ =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as FRACT;
    if fDroidZ + fDZ < 0 as libc::c_int as libc::c_float {
        (*psDroid).sMove.fz = 0 as libc::c_int as FRACT
    } else if fDroidZ + fDZ < fMapZ {
        (*psDroid).sMove.fz = fMapZ
    } else { (*psDroid).sMove.fz = (*psDroid).sMove.fz + fDZ }
    (*psDroid).z = (*psDroid).sMove.fz as UWORD;
    moveAdjustVtolHeight(psDroid, iMapZ as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn moveGetStatusStr(mut status: UBYTE,
                                          mut szStr: *mut libc::c_char) {
    match status as libc::c_int {
        0 => {
            strcpy(szStr,
                   b"MOVEINACTIVE\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            strcpy(szStr,
                   b"MOVENAVIGATE\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            strcpy(szStr,
                   b"MOVETURN\x00" as *const u8 as *const libc::c_char);
        }
        3 => {
            strcpy(szStr,
                   b"MOVEPAUSE\x00" as *const u8 as *const libc::c_char);
        }
        4 => {
            strcpy(szStr,
                   b"MOVEPOINTTOPOINT\x00" as *const u8 as
                       *const libc::c_char);
        }
        5 => {
            strcpy(szStr,
                   b"MOVETURNSTOP\x00" as *const u8 as *const libc::c_char);
        }
        6 => {
            strcpy(szStr,
                   b"MOVETURNTOTARGET\x00" as *const u8 as
                       *const libc::c_char);
        }
        7 => {
            strcpy(szStr,
                   b"MOVEROUTE\x00" as *const u8 as *const libc::c_char);
        }
        8 => {
            strcpy(szStr,
                   b"MOVEHOVER\x00" as *const u8 as *const libc::c_char);
        }
        9 => {
            strcpy(szStr,
                   b"MOVEDRIVE\x00" as *const u8 as *const libc::c_char);
        }
        10 => {
            strcpy(szStr,
                   b"MOVEDRIVEFOLLOW\x00" as *const u8 as
                       *const libc::c_char);
        }
        _ => { strcpy(szStr, b"\x00" as *const u8 as *const libc::c_char); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn moveCyborgLaunchAnimDone(mut psObj:
                                                      *mut ANIM_OBJECT) {
    let mut psDroid: *mut DROID = (*psObj).psParent as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveCyborgLaunchAnimDone: invalid cyborg pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              3961 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"moveCyborgLaunchAnimDone\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* raise cyborg a little bit so flying - terrible hack - GJ */
    (*psDroid).z = (*psDroid).z.wrapping_add(1);
    (*psDroid).sMove.iVertSpeed =
        ((*psDroid).baseSpeed as SDWORD / 2 as libc::c_int) as SWORD;
    (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT;
}
#[no_mangle]
pub unsafe extern "C" fn moveCyborgTouchDownAnimDone(mut psObj:
                                                         *mut ANIM_OBJECT) {
    let mut psDroid: *mut DROID = (*psObj).psParent as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveCyborgTouchDownAnimDone: invalid cyborg pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              3976 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"moveCyborgTouchDownAnimDone\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT;
    (*psDroid).z =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn moveUpdateJumpCyborgModel(mut psDroid: *mut DROID,
                                                   mut speed: SDWORD,
                                                   mut direction: SDWORD) {
    let mut fPerpSpeed: FRACT = 0.;
    let mut fNormalSpeed: FRACT = 0.;
    let mut dx: FRACT = 0.;
    let mut dy: FRACT = 0.;
    let mut fSpeed: FRACT = 0.;
    let mut iDroidDir: SDWORD = 0;
    // nothing to do if the droid is stopped
    if moveDroidStopped(psDroid, speed) == 1 as libc::c_int { return }
    // update the naybors list
    droidGetNaybors(psDroid);
    moveUpdateDroidDirection(psDroid, &mut speed, direction,
                             360 as libc::c_int,
                             (*psDroid).baseSpeed as SDWORD,
                             (*psDroid).baseSpeed.wrapping_div(3 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
                                 as SDWORD, &mut iDroidDir, &mut fSpeed);
    fNormalSpeed =
        moveCalcNormalSpeed(psDroid, fSpeed, iDroidDir, 200 as libc::c_int,
                            200 as libc::c_int);
    fPerpSpeed = 0 as libc::c_int as FRACT;
    moveCombineNormalAndPerpSpeeds(psDroid, fNormalSpeed, fPerpSpeed,
                                   iDroidDir);
    moveGetDroidPosDiffs(psDroid, &mut dx, &mut dy);
    moveUpdateDroidPos(psDroid, dx, dy);
}
#[no_mangle]
pub unsafe extern "C" fn moveUpdateCyborgModel(mut psDroid: *mut DROID,
                                               mut moveSpeed: SDWORD,
                                               mut moveDir: SDWORD,
                                               mut oldStatus: UBYTE) {
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut psObj: *mut BASE_OBJECT = psDroid as *mut BASE_OBJECT;
    let mut iMapZ: UDWORD =
        map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as UDWORD;
    let mut iDist: SDWORD = 0;
    let mut iDx: SDWORD = 0;
    let mut iDy: SDWORD = 0;
    let mut iDz: SDWORD = 0;
    let mut iDroidZ: SDWORD = 0;
    let mut bRet: BOOL = 0;
    // nothing to do if the droid is stopped
    if moveDroidStopped(psDroid, moveSpeed) == 1 as libc::c_int {
        if !(*psDroid).psCurAnim.is_null() {
            if animObj_Remove(&mut (*psDroid).psCurAnim,
                              (*(*psDroid).psCurAnim).uwID as libc::c_int) ==
                   0 as libc::c_int {
                debug(LOG_NEVER,
                      b"moveUpdateCyborgModel: couldn\'t remove walk anim\n\x00"
                          as *const u8 as *const libc::c_char);
            }
            (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
        }
        return
    }
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateCyborgModel: invalid propulsion stats pointer\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4037 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"moveUpdateCyborgModel\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    /* do vertical movement */
    if (*psPropStats).propulsionType as libc::c_int == JUMP as libc::c_int {
        iDz =
            (*psDroid).sMove.iVertSpeed as libc::c_int * frameTime as SDWORD /
                1000 as libc::c_int;
        iDroidZ = (*psDroid).z as SDWORD;
        if iDroidZ + iDz < iMapZ as SDWORD {
            (*psDroid).sMove.iVertSpeed = 0 as libc::c_int as SWORD;
            (*psDroid).z = iMapZ as UWORD
        } else { (*psDroid).z = ((*psDroid).z as libc::c_int + iDz) as UWORD }
        if (*psDroid).z as libc::c_uint >=
               iMapZ.wrapping_add(200 as libc::c_int as libc::c_uint) &&
               (*psDroid).sMove.iVertSpeed as libc::c_int > 0 as libc::c_int {
            (*psDroid).sMove.iVertSpeed =
                -((*psDroid).baseSpeed as SDWORD / 2 as libc::c_int) as SWORD
        }
        (*psDroid).sMove.fz = (*psDroid).z as FRACT
    }
    /* calculate move distance */
    iDx = (*psDroid).sMove.DestinationX - (*psDroid).x as SDWORD;
    iDy = (*psDroid).sMove.DestinationY - (*psDroid).y as SDWORD;
    iDz = (*psDroid).z as SDWORD - iMapZ as SDWORD;
    iDist = trigIntSqrt((iDx * iDx + iDy * iDy) as UDWORD) as SDWORD;
    /* set jumping cyborg walking short distances */
    if (*psPropStats).propulsionType as libc::c_int != JUMP as libc::c_int ||
           (*psDroid).sMove.iVertSpeed as libc::c_int == 0 as libc::c_int &&
               iDist < 500 as libc::c_int {
        if (*psDroid).psCurAnim.is_null() {
            // Only add the animation if the droid is on screen, saves memory and time.
            if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) != 0 {
                //DBPRINTF(("Added cyborg run anim\n"));
                //What about my new cyborg droids?????!!!!!!!
				/*if ( psDroid->droidType == DROID_CYBORG )
				{
					psDroid->psCurAnim = animObj_Add( psObj, ID_ANIM_CYBORG_RUN, 0, 0 );
				}
				else if ( psDroid->droidType == DROID_CYBORG_SUPER )
				{
					psDroid->psCurAnim = animObj_Add( psObj, ID_ANIM_SUPERCYBORG_RUN, 0, 0 );
				}*/
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
                    (*psDroid).psCurAnim =
                        animObj_Add(psObj as *mut libc::c_void,
                                    10 as libc::c_int,
                                    0 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UWORD)
                } else if cyborgDroid(psDroid) != 0 {
                    (*psDroid).psCurAnim =
                        animObj_Add(psObj as *mut libc::c_void,
                                    6 as libc::c_int,
                                    0 as libc::c_int as UDWORD,
                                    0 as libc::c_int as UWORD)
                }
            }
        } else if clipXY((*psDroid).x as SDWORD, (*psDroid).y as SDWORD) == 0
         {
            bRet =
                animObj_Remove(&mut (*psDroid).psCurAnim,
                               (*(*(*psDroid).psCurAnim).psAnim).uwID as
                                   libc::c_int);
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"moveUpdateCyborgModel : animObj_Remove failed\x00" as
                          *const u8 as *const libc::c_char);
            };
            if bRet == 1 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      4105 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"moveUpdateCyborgModel\x00")).as_ptr(),
                      b"bRet == TRUE\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).psCurAnim = 0 as *mut ANIM_OBJECT
            // If the droid went off screen then remove the animation, saves memory and time.
            //DBPRINTF(("Removed cyborg run anim\n"));
        }
        /* use baba person movement */
        moveUpdatePersonModel(psDroid, moveSpeed, moveDir);
    } else {
        /* jumping cyborg: remove walking animation if present */
        if !(*psDroid).psCurAnim.is_null() {
            if ((*(*psDroid).psCurAnim).uwID as libc::c_int ==
                    6 as libc::c_int ||
                    (*(*psDroid).psCurAnim).uwID as libc::c_int ==
                        10 as libc::c_int ||
                    (*(*psDroid).psCurAnim).uwID as libc::c_int ==
                        9 as libc::c_int) &&
                   animObj_Remove(&mut (*psDroid).psCurAnim,
                                  (*(*psDroid).psCurAnim).uwID as libc::c_int)
                       == 0 as libc::c_int {
                debug(LOG_NEVER,
                      b"moveUpdateCyborgModel: couldn\'t remove walk anim\n\x00"
                          as *const u8 as *const libc::c_char);
            }
        }
        /* add jumping or landing anim */
        if oldStatus as libc::c_int == 4 as libc::c_int &&
               (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int {
            (*psDroid).psCurAnim =
                animObj_Add(psObj as *mut libc::c_void, 8 as libc::c_int,
                            0 as libc::c_int as UDWORD,
                            1 as libc::c_int as UWORD);
            animObj_SetDoneFunc((*psDroid).psCurAnim,
                                Some(moveCyborgTouchDownAnimDone as
                                         unsafe extern "C" fn(_:
                                                                  *mut ANIM_OBJECT)
                                             -> ()));
        } else if (*psDroid).sMove.Status as libc::c_int == 4 as libc::c_int {
            if (*psDroid).z as libc::c_uint == iMapZ {
                if (*psDroid).sMove.iVertSpeed as libc::c_int ==
                       0 as libc::c_int {
                    (*psDroid).psCurAnim =
                        animObj_Add(psObj as *mut libc::c_void,
                                    7 as libc::c_int,
                                    0 as libc::c_int as UDWORD,
                                    1 as libc::c_int as UWORD);
                    animObj_SetDoneFunc((*psDroid).psCurAnim,
                                        Some(moveCyborgLaunchAnimDone as
                                                 unsafe extern "C" fn(_:
                                                                          *mut ANIM_OBJECT)
                                                     -> ()));
                } else {
                    (*psDroid).psCurAnim =
                        animObj_Add(psObj as *mut libc::c_void,
                                    8 as libc::c_int,
                                    0 as libc::c_int as UDWORD,
                                    1 as libc::c_int as UWORD);
                    animObj_SetDoneFunc((*psDroid).psCurAnim,
                                        Some(moveCyborgTouchDownAnimDone as
                                                 unsafe extern "C" fn(_:
                                                                          *mut ANIM_OBJECT)
                                                     -> ()));
                }
            } else { moveUpdateJumpCyborgModel(psDroid, moveSpeed, moveDir); }
        }
    }
    (*psDroid).pitch = 0 as libc::c_int as SWORD;
    (*psDroid).roll = 0 as libc::c_int as SWORD;
}
#[no_mangle]
pub unsafe extern "C" fn moveDescending(mut psDroid: *mut DROID,
                                        mut iMapHeight: UDWORD) -> BOOL {
    if (*psDroid).z as libc::c_uint > iMapHeight {
        /* descending */
        (*psDroid).sMove.iVertSpeed =
            -if (*psDroid).baseSpeed as SDWORD / 4 as libc::c_int >
                    60 as libc::c_int {
                 ((*psDroid).baseSpeed as SDWORD) / 4 as libc::c_int
             } else { 60 as libc::c_int } as SWORD;
        (*psDroid).sMove.speed = 0 as libc::c_int as FRACT;
        //		psDroid->sMove.Speed = 0;
        /* return TRUE to show still descending */
        return 1 as libc::c_int
    } else {
        /* on floor - stop */
        (*psDroid).sMove.iVertSpeed = 0 as libc::c_int as SWORD;
        /* conform to terrain */
        updateDroidOrientation(psDroid);
        /* return FALSE to show stopped descending */
        return 0 as libc::c_int
    };
}
/* Frame update for the movement of a tracked droid */
/* update body and turret to local slope */
/* audio callback used to kill movement sounds */
#[no_mangle]
pub unsafe extern "C" fn moveCheckDroidMovingAndVisible(mut psSample:
                                                            *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveCheckUnitMovingAndVisible: audio sample pointer invalid\n\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4194 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 31],
                                        &[libc::c_char; 31]>(b"moveCheckDroidMovingAndVisible\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psSample).psObj.is_null() {
        return 0 as libc::c_int
    } else {
        psDroid = (*psSample).psObj as *mut DROID;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"moveCheckUnitMovingAndVisible: unit pointer invalid\n\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"move.c\x00" as *const u8 as *const libc::c_char,
                  4204 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 31],
                                            &[libc::c_char; 31]>(b"moveCheckDroidMovingAndVisible\x00")).as_ptr(),
                  b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    /* check for dead, not moving or invisible to player */
    if (*psDroid).died != 0 ||
           moveDroidStopped(psDroid, 0 as libc::c_int) != 0 ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               (*psDroid).order == DORDER_NONE as libc::c_int ||
           !((*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0
                 || godMode != 0) {
        (*psDroid).iAudioID = NO_SOUND as libc::c_int;
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn movePlayDroidMoveAudio(mut psDroid: *mut DROID) {
    let mut iAudioID: SDWORD = NO_SOUND as libc::c_int;
    let mut psPropType: *mut PROPULSION_TYPES = 0 as *mut PROPULSION_TYPES;
    let mut iPropType: UBYTE = 0 as libc::c_int as UBYTE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"movePlayUnitMoveAudio: unit pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4230 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"movePlayDroidMoveAudio\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psDroid.is_null() &&
           ((*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0 ||
                godMode != 0) {
        iPropType =
            (*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                             libc::c_int as
                                                             usize].nStat as
                                           isize)).propulsionType;
        psPropType =
            &mut *asPropulsionTypes.offset(iPropType as isize) as
                *mut PROPULSION_TYPES;
        /* play specific wheeled and transporter or stats-specified noises */
        if iPropType as libc::c_int == WHEELED as libc::c_int &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint {
            iAudioID = ID_SOUND_TREAD as libc::c_int
        } else if (*psDroid).droidType as libc::c_uint ==
                      DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            iAudioID = ID_SOUND_BLIMP_FLIGHT as libc::c_int
        } else if iPropType as libc::c_int == LEGGED as libc::c_int &&
                      cyborgDroid(psDroid) != 0 {
            iAudioID = ID_SOUND_CYBORG_MOVE as libc::c_int
        } else { iAudioID = (*psPropType).moveID as SDWORD }
        if iAudioID != NO_SOUND as libc::c_int {
            if audio_PlayObjDynamicTrack(psDroid as *mut libc::c_void,
                                         iAudioID,
                                         Some(moveCheckDroidMovingAndVisible
                                                  as
                                                  unsafe extern "C" fn(_:
                                                                           *mut AUDIO_SAMPLE)
                                                      -> BOOL)) != 0 {
                (*psDroid).iAudioID = iAudioID
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn moveDroidStartCallback(mut psSample:
                                                    *mut AUDIO_SAMPLE)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUnitStartCallback: audio sample pointer invalid\n\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4275 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"moveDroidStartCallback\x00")).as_ptr(),
              b"PTRVALID(psSample, sizeof(AUDIO_SAMPLE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psSample).psObj.is_null() {
        return 0 as libc::c_int
    } else {
        psDroid = (*psSample).psObj as *mut DROID;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"moveDroidStartCallback: unit pointer invalid\n\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"move.c\x00" as *const u8 as *const libc::c_char,
                  4285 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"moveDroidStartCallback\x00")).as_ptr(),
                  b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    if !psDroid.is_null() { movePlayDroidMoveAudio(psDroid); }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn movePlayAudio(mut psDroid: *mut DROID,
                                       mut bStarted: BOOL,
                                       mut bStoppedBefore: BOOL,
                                       mut iMoveSpeed: SDWORD) {
    let mut propType: UBYTE = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut psPropType: *mut PROPULSION_TYPES = 0 as *mut PROPULSION_TYPES;
    let mut bStoppedNow: BOOL = 0;
    let mut iAudioID: SDWORD = NO_SOUND as libc::c_int;
    let mut pAudioCallback: AUDIO_CALLBACK = None;
    //else if ( iPropType == LEGGED && psDroid->droidType == DROID_CYBORG )
    /* get prop stats */
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4310 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"movePlayAudio\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    propType = (*psPropStats).propulsionType;
    psPropType =
        &mut *asPropulsionTypes.offset(propType as isize) as
            *mut PROPULSION_TYPES;
    /* get current droid motion status */
    bStoppedNow = moveDroidStopped(psDroid, iMoveSpeed);
    if bStarted != 0 {
        /* play start audio */
        if propType as libc::c_int == WHEELED as libc::c_int &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
               (*psPropType).startID as libc::c_int == NO_SOUND as libc::c_int
           {
            movePlayDroidMoveAudio(psDroid);
            return
        } else {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                iAudioID = ID_SOUND_BLIMP_TAKE_OFF as libc::c_int
            } else { iAudioID = (*psPropType).startID as SDWORD }
        }
        pAudioCallback =
            Some(moveDroidStartCallback as
                     unsafe extern "C" fn(_: *mut AUDIO_SAMPLE) -> BOOL)
    } else if bStoppedBefore == 0 && bStoppedNow != 0 &&
                  (*psPropType).shutDownID as libc::c_int !=
                      NO_SOUND as libc::c_int {
        /* play stop audio */
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            iAudioID = ID_SOUND_BLIMP_LAND as libc::c_int
        } else if propType as libc::c_int != WHEELED as libc::c_int ||
                      (*psDroid).droidType as libc::c_uint ==
                          DROID_CONSTRUCT as libc::c_int as libc::c_uint {
            iAudioID = (*psPropType).shutDownID as SDWORD
        }
    } else if bStoppedBefore == 0 && bStoppedNow == 0 &&
                  (*psDroid).iAudioID == NO_SOUND as libc::c_int {
        /* play move audio */
        movePlayDroidMoveAudio(psDroid);
        return
    }
    if iAudioID != NO_SOUND as libc::c_int &&
           ((*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0 ||
                godMode != 0) {
        if audio_PlayObjDynamicTrack(psDroid as *mut libc::c_void, iAudioID,
                                     pAudioCallback) != 0 {
            (*psDroid).iAudioID = iAudioID
        }
    };
}
// called when a droid moves to a new tile.
// use to pick up oil, etc..
unsafe extern "C" fn checkLocalFeatures(mut psDroid: *mut DROID) {
    let mut i: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    // only do for players droids.
    if (*psDroid).player as libc::c_uint != selectedPlayer {
        return
    } // update naybor list.
    droidGetNaybors(psDroid);
    // scan the neighbours
    i = 0 as libc::c_int;
    while i < numNaybors as SDWORD {
        psObj = asDroidNaybors[i as usize].psObj;
        if !((*psObj).type_0 as libc::c_uint !=
                 OBJ_FEATURE as libc::c_int as libc::c_uint ||
                 (*(*(psObj as *mut FEATURE)).psStats).subType as libc::c_uint
                     != FEAT_OIL_DRUM as libc::c_int as libc::c_uint ||
                 asDroidNaybors[i as usize].distSqr >=
                     (128 as libc::c_int * 3 as libc::c_int / 2 as libc::c_int
                          *
                          (128 as libc::c_int * 3 as libc::c_int /
                               2 as libc::c_int)) as libc::c_uint) {
            if bMultiPlayer != 0 &&
                   (*psObj).player as libc::c_int == 99 as libc::c_int {
                // remove artifact+ send multiplay info.
                giftPower(99 as libc::c_int as UDWORD, selectedPlayer,
                          1 as libc::c_int); // give power and tell everyone.
                addOilDrum(1 as libc::c_int as UDWORD);
            } else { addPower(selectedPlayer, 100 as libc::c_int as UDWORD); }
            removeFeature(psObj as *mut FEATURE);
        }
        i += 1
    };
}
// object too far away to worry about
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
/* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
// the droid will not join a formation when it gets to the location
// move a droid directly to a location (used by vtols only)
// Get a droid to turn towards a locaton
/* Stop a droid */
/*Stops a droid dead in its tracks - doesn't allow for any little skidding bits*/
/* Get a droid to do a frame's worth of moving */
//static UDWORD LastMoveFrame;
/* Frame update for the movement of a tracked droid */
#[no_mangle]
pub unsafe extern "C" fn moveUpdateDroid(mut psDroid: *mut DROID) {
    //	SDWORD		xdiff,ydiff, obstX,obstY;
//	UDWORD		mapX,mapY, tarSpeed;
//	FRACT		newX,newY;
//	FRACT		speed;
//	FRACT		dangle;
//	BASE_OBJECT	*psObst;
    let mut tx: FRACT = 0.; //adiff, dx,dy, mx,my;
    let mut ty: FRACT =
        0.; // thats DROID angle and TARGET angle - not some bizzare pun :-)
    let mut tangle: FRACT = 0.;
    // doesn't matter - they're still shit names...! :-)
    let mut fx: SDWORD = 0;
    let mut fy: SDWORD = 0;
    let mut oldx: UDWORD = 0;
    let mut oldy: UDWORD = 0;
    let mut iZ: UDWORD = 0;
    let mut oldStatus: UBYTE = (*psDroid).sMove.Status;
    let mut moveSpeed: SDWORD = 0;
    let mut moveDir: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut bStarted: BOOL = 0 as libc::c_int;
    let mut bStopped: BOOL = 0;
    //	UDWORD				landX,landY;
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"moveUpdateUnit: unit at (0,0)" );
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"moveUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"move.c\x00" as *const u8 as *const libc::c_char,
              4455 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"moveUpdateDroid\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    //	if(driveModeActive()) {
//		driveUpdateDroid(psDroid);
//	}
    //if the droid has been attacked by an EMP weapon, it is temporarily disabled
    if (*psDroid).lastHitWeapon == WSC_EMP as libc::c_int as libc::c_uint {
        if gameTime.wrapping_sub((*psDroid).timeLastHit) <
               10000 as libc::c_int as libc::c_uint {
            //get out without updating
            return
        }
    }
    /* save current motion status of droid */
    bStopped = moveDroidStopped(psDroid, 0 as libc::c_int);
    fpathSetBlockingTile((*psPropStats).propulsionType);
    fpathSetCurrentObject(psDroid as *mut BASE_OBJECT);
    moveSpeed = 0 as libc::c_int;
    moveDir = (*psDroid).direction as SDWORD;
    /* get droid height */
    iZ = map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as UDWORD;
    let mut current_block_146: u64;
    match (*psDroid).sMove.Status as libc::c_int {
        0 => {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_PERSON as libc::c_int as libc::c_uint &&
                   !(*psDroid).psCurAnim.is_null() &&
                   (*(*(*psDroid).psCurAnim).psAnim).uwID as libc::c_int ==
                       2 as libc::c_int {
                (*(*psDroid).psCurAnim).bVisible = 0 as libc::c_int
            }
        }
        7 | 13 | 12 => {
            // deal with both waiting for a route (MOVEROUTE) and the droid shuffle (MOVESHUFFLE)
		// here because droids waiting for a route need to shuffle out of the way (MOVEROUTESHUFFLE)
		// of those that have already got a route
            if (*psDroid).sMove.Status as libc::c_int == 7 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 13 as libc::c_int
               {
                // see if this droid started waiting for a route before the previous one
			// and note it to be the next droid to route.
			// selectedPlayer always gets precidence in single player
                if psNextRouteDroid.is_null() {
                    psNextRouteDroid = psDroid
                } else if bMultiPlayer != 0 &&
                              (*psNextRouteDroid).sMove.bumpTime >
                                  (*psDroid).sMove.bumpTime {
                    psNextRouteDroid = psDroid
                } else if (*psDroid).player as libc::c_uint == selectedPlayer
                              &&
                              ((*psNextRouteDroid).player as libc::c_uint !=
                                   selectedPlayer ||
                                   (*psNextRouteDroid).sMove.bumpTime >
                                       (*psDroid).sMove.bumpTime) {
                    psNextRouteDroid = psDroid
                } else if (*psDroid).player as libc::c_uint != selectedPlayer
                              &&
                              (*psNextRouteDroid).player as libc::c_uint !=
                                  selectedPlayer &&
                              (*psNextRouteDroid).sMove.bumpTime >
                                  (*psDroid).sMove.bumpTime {
                    psNextRouteDroid = psDroid
                }
            }
            if (*psDroid).sMove.Status as libc::c_int == 7 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 13 as libc::c_int
               {
                //			 (gameTime >= psDroid->sMove.bumpTime) )
                (*psDroid).sMove.fx = (*psDroid).x as FRACT;
                (*psDroid).sMove.fy = (*psDroid).y as FRACT;
                (*psDroid).sMove.fz = (*psDroid).z as FRACT;
                //			psDroid->sMove.bumpTime = 0;
                turnOffMultiMsg(1 as libc::c_int);
                moveDroidTo(psDroid, (*psDroid).sMove.DestinationX as UDWORD,
                            (*psDroid).sMove.DestinationY as UDWORD);
                fpathSetBlockingTile((*psPropStats).propulsionType);
                turnOffMultiMsg(0 as libc::c_int);
            } else if (*psDroid).sMove.Status as libc::c_int ==
                          12 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              13 as libc::c_int {
                if moveReachedWayPoint(psDroid) != 0 ||
                       (*psDroid).sMove.shuffleStart.wrapping_add(10000 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                           < gameTime {
                    if (*psDroid).sMove.Status as libc::c_int ==
                           13 as libc::c_int {
                        (*psDroid).sMove.Status = 7 as libc::c_int as UBYTE
                    } else if (*psPropStats).propulsionType as libc::c_int ==
                                  LIFT as libc::c_int {
                        (*psDroid).sMove.Status = 8 as libc::c_int as UBYTE
                    } else {
                        (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
                    }
                } else {
                    // Calculate a target vector
                    moveGetDirection(psDroid, &mut tx, &mut ty);
                    // Turn the droid if necessary
                    tangle = vectorToAngle(tx, ty);
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                        debug(LOG_NEVER,
                              b"a) dir %g,%g (%g)\n\x00" as *const u8 as
                                  *const libc::c_char, tx as libc::c_double,
                              ty as libc::c_double, tangle as libc::c_double);
                    }
                    moveSpeed = moveCalcDroidSpeed(psDroid);
                    moveDir = tangle as SDWORD
                }
            }
        }
        11 => {
            moveDroidTo(psDroid, (*psDroid).sMove.DestinationX as UDWORD,
                        (*psDroid).sMove.DestinationY as UDWORD);
            fpathSetBlockingTile((*psPropStats).propulsionType);
        }
        1 => {
            // Get the next control point
            if moveNextTarget(psDroid) == 0 {
                // No more waypoints - finish
                if (*psPropStats).propulsionType as libc::c_int ==
                       LIFT as libc::c_int {
                    (*psDroid).sMove.Status = 8 as libc::c_int as UBYTE
                } else { (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE }
            } else {
                // Calculate the direction vector
//		psDroid->direction = calcDirection(psDroid->x,psDroid->y, tarX,tarY);
//		moveCalcVector(psDroid, tarX,tarY, &psDroid->sMove.dx,&psDroid->sMove.dy)
                (*psDroid).sMove.fx = (*psDroid).x as FRACT;
                (*psDroid).sMove.fy = (*psDroid).y as FRACT;
                (*psDroid).sMove.fz = (*psDroid).z as FRACT;
                moveCalcBoundary(psDroid);
                if vtolDroid(psDroid) != 0 {
                    (*psDroid).pitch = 0 as libc::c_int as SWORD
                }
                (*psDroid).sMove.Status = 4 as libc::c_int as UBYTE;
                (*psDroid).sMove.bumpTime = 0 as libc::c_int as UDWORD;
                /* save started status for movePlayAudio */
                if (*psDroid).sMove.speed == 0 as libc::c_int as FRACT {
                    bStarted = 1 as libc::c_int
                }
            }
        }
        4 | 3 => {
            // moving between two way points
            // See if the target point has been reached
            if moveReachedWayPoint(psDroid) != 0 {
                // Got there - move onto the next waypoint
                if moveNextTarget(psDroid) == 0 {
                    // No more waypoints - finish
//				psDroid->sMove.Status = MOVEINACTIVE;
                    if (*psPropStats).propulsionType as libc::c_int ==
                           LIFT as libc::c_int {
                        (*psDroid).sMove.Status = 8 as libc::c_int as UBYTE
                    } else {
                        (*psDroid).sMove.Status = 2 as libc::c_int as UBYTE
                    }
                    current_block_146 = 8880031775101799352;
                } else {
                    moveCalcBoundary(psDroid);
                    current_block_146 = 15734707049249739970;
                }
            } else { current_block_146 = 15734707049249739970; }
            match current_block_146 {
                8880031775101799352 => { }
                _ => {
                    if !(*psDroid).sMove.psFormation.is_null() &&
                           (*psDroid).sMove.Position as libc::c_int ==
                               (*psDroid).sMove.numPoints as libc::c_int {
                        if vtolDroid(psDroid) != 0 {
                            // vtols have to use the ground blocking tile when they are going to land
                            fpathBlockingTile =
                                Some(fpathGroundBlockingTile as
                                         unsafe extern "C" fn(_: SDWORD,
                                                              _: SDWORD)
                                             -> BOOL)
                        }
                        if formationGetPos((*psDroid).sMove.psFormation,
                                           psDroid as *mut BASE_OBJECT,
                                           &mut fx, &mut fy, 1 as libc::c_int)
                               != 0 {
                            (*psDroid).sMove.targetX = fx;
                            (*psDroid).sMove.targetY = fy;
                            moveCalcBoundary(psDroid);
                        }
                        /*if (vtolDroid(psDroid))
			{
				// reset to the normal blocking tile
				fpathBlockingTile = fpathLiftBlockingTile;
			}*/
                    }
                    //		DebugP=FALSE;
//		if ( psDroid->droidType == DROID_TRANSPORTER ) DebugP=TRUE;
                    // Calculate a target vector
                    moveGetDirection(psDroid, &mut tx, &mut ty);
                    // Turn the droid if necessary
		// calculate the difference in the angles
//		dangle = (float) psDroid->direction;
                    tangle = vectorToAngle(tx, ty);
                    moveSpeed = moveCalcDroidSpeed(psDroid);
                    moveDir = tangle as SDWORD;
                    //if ( psDroid->droidType == DROID_TRANSPORTER )
//{
//			DBPRINTF(("dir %d,%d ($%x=%d)\n",tx,ty,tangle,moveDir));
//	}
                    if (*psDroid).sMove.bumpTime !=
                           0 as libc::c_int as libc::c_uint &&
                           ((*psDroid).sMove.pauseTime as
                                libc::c_uint).wrapping_add((*psDroid).sMove.bumpTime).wrapping_add(1500
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                               < gameTime {
                        if (*psDroid).sMove.Status as libc::c_int ==
                               4 as libc::c_int {
                            (*psDroid).sMove.Status =
                                3 as libc::c_int as UBYTE
                        } else {
                            (*psDroid).sMove.Status =
                                4 as libc::c_int as UBYTE
                        }
                        (*psDroid).sMove.pauseTime =
                            gameTime.wrapping_sub((*psDroid).sMove.bumpTime)
                                as UWORD
                    }
                    if (*psDroid).sMove.Status as libc::c_int ==
                           3 as libc::c_int &&
                           (*psDroid).sMove.bumpTime !=
                               0 as libc::c_int as libc::c_uint &&
                           (*psDroid).sMove.lastBump as libc::c_int >
                               (*psDroid).sMove.pauseTime as libc::c_int &&
                           ((*psDroid).sMove.lastBump as
                                libc::c_uint).wrapping_add((*psDroid).sMove.bumpTime).wrapping_add(500
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
                               < gameTime {
                        (*psDroid).sMove.Status = 4 as libc::c_int as UBYTE
                    }
                }
            }
        }
        2 => {
            // Turn the droid to it's final facing
            if !(*psDroid).sMove.psFormation.is_null() &&
                   (*(*psDroid).sMove.psFormation).refCount as libc::c_int >
                       1 as libc::c_int &&
                   (*psDroid).direction as libc::c_uint !=
                       (*(*psDroid).sMove.psFormation).dir as UDWORD {
                moveSpeed = 0 as libc::c_int;
                moveDir = (*(*psDroid).sMove.psFormation).dir as SDWORD
            } else if (*psPropStats).propulsionType as libc::c_int ==
                          LIFT as libc::c_int {
                (*psDroid).sMove.Status = 4 as libc::c_int as UBYTE
            } else { (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE }
        }
        6 => {
            moveSpeed = 0 as libc::c_int;
            moveDir =
                calcDirection((*psDroid).x as UDWORD, (*psDroid).y as UDWORD,
                              (*psDroid).sMove.targetX as UDWORD,
                              (*psDroid).sMove.targetY as UDWORD);
            if (*psDroid).direction as libc::c_uint == moveDir as UDWORD {
                if (*psPropStats).propulsionType as libc::c_int ==
                       LIFT as libc::c_int {
                    (*psDroid).sMove.Status = 4 as libc::c_int as UBYTE
                } else { (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE }
            }
        }
        8 => {
            /* change vtols to attack run mode if target found - but not if no ammo*/
		/*if ( psDroid->droidType != DROID_CYBORG && psDroid->psTarget != NULL &&
			!vtolEmpty(psDroid))
		{
			psDroid->sMove.Status = MOVEPOINTTOPOINT;
			break;
		}*/
            /*		moveGetDirection(psDroid, &tx,&ty);
		tangle = vectorToAngle(tx,ty);
		moveSpeed = moveCalcDroidSpeed(psDroid);
		moveDir = MAKEINT(tangle);*/
            /* descend if no orders or actions or cyborg at target */
/*		if ( (psDroid->droidType == DROID_CYBORG) ||
			 ((psDroid->action == DACTION_NONE) && (psDroid->order == DORDER_NONE)) ||
			 ((psDroid->action == DACTION_NONE) && (psDroid->order == DORDER_TRANSPORTIN)) ||
			 ((psDroid->action == DACTION_NONE) && (psDroid->order == DORDER_GUARD)) ||
			 (psDroid->action == DACTION_MOVE) || (psDroid->action == DACTION_WAITFORREARM) ||
			 (psDroid->action == DACTION_WAITDURINGREARM)  || (psDroid->action == DACTION_FIRESUPPORT))*/
            if moveDescending(psDroid, iZ) == 0 as libc::c_int {
                /* reset move state */
                (*psDroid).sMove.Status = 0 as libc::c_int as UBYTE
            }
            /*			else
			{
				// see if the landing position is clear
				landX = psDroid->x;
				landY = psDroid->y;
				actionVTOLLandingPos(psDroid, &landX,&landY);
				if ((SDWORD)(landX >> TILE_SHIFT) != (psDroid->x >> TILE_SHIFT) &&
					(SDWORD)(landY >> TILE_SHIFT) != (psDroid->y >> TILE_SHIFT))
				{
					moveDroidToDirect(psDroid, landX,landY);
				}
			}*/
        }
        9 => {
            // Driven around by the player.
            driveSetDroidMove(psDroid); //MAKEINT(psDroid->sMove.speed);
            moveSpeed = driveGetMoveSpeed(); //psDroid->sMove.dir;
            moveDir = driveGetMoveDir()
        }
        10 => { }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"moveUpdateUnit: unknown move state\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"move.c\x00" as *const u8 as *const libc::c_char,
                      4895 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"moveUpdateDroid\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    // Update the movement model for the droid
    oldx = (*psDroid).x as UDWORD;
    oldy = (*psDroid).y as UDWORD;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_PERSON as libc::c_int as libc::c_uint {
        moveUpdatePersonModel(psDroid, moveSpeed, moveDir);
    } else if cyborgDroid(psDroid) != 0 {
        moveUpdateCyborgModel(psDroid, moveSpeed, moveDir, oldStatus);
    } else if (*psPropStats).propulsionType as libc::c_int ==
                  LIFT as libc::c_int {
        moveUpdateVtolModel(psDroid, moveSpeed, moveDir);
    } else { moveUpdateGroundModel(psDroid, moveSpeed, moveDir); }
    if oldx as SDWORD >> 7 as libc::c_int !=
           (*psDroid).x as libc::c_int >> 7 as libc::c_int ||
           oldy as SDWORD >> 7 as libc::c_int !=
               (*psDroid).y as libc::c_int >> 7 as libc::c_int {
        visTilesUpdate(psDroid as *mut BASE_OBJECT, 0 as libc::c_int);
        gridMoveObject(psDroid as *mut BASE_OBJECT, oldx as SDWORD,
                       oldy as SDWORD);
        //else if ( psDroid->droidType == DROID_CYBORG )
        // object moved from one tile to next, check to see if droid is near stuff.(oil)
        checkLocalFeatures(psDroid);
    }
    // See if it's got blocked
    if (*psPropStats).propulsionType as libc::c_int != LIFT as libc::c_int &&
           moveBlocked(psDroid) != 0 {
        (*psDroid).sMove.Status = 2 as libc::c_int as UBYTE
    }
    //	// If were in drive mode and the droid is a follower then stop it when it gets within
//	// range of the driver.
//	if(driveIsFollower(psDroid)) {
//		if(DoFollowRangeCheck) {
// //DBPRINTF(("%d\n",gameTime);
//			if(driveInDriverRange(psDroid)) {
//				psDroid->sMove.Status = MOVEINACTIVE;
// //				ClearFollowRangeCheck = TRUE;
//			} else {
//				AllInRange = FALSE;
//			}
//		}
//	}
    // reset the blocking tile function and current object
    fpathBlockingTile =
        Some(fpathGroundBlockingTile as
                 unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL);
    fpathSetCurrentObject(0 as *mut BASE_OBJECT);
    //	ASSERT( psDroid->x != 0 && psDroid->y != 0,
//		"moveUpdateUnit (end): unit at (0,0)" );
    /* If it's sitting in water then it's got to go with the flow! */
    if terrainTypes[((*mapTile(((*psDroid).x as libc::c_int /
                                    128 as libc::c_int) as UDWORD,
                               ((*psDroid).y as libc::c_int /
                                    128 as libc::c_int) as UDWORD)).texture as
                         libc::c_int & 0x1ff as libc::c_int) as usize] as
           libc::c_int == TER_WATER as libc::c_int {
        updateDroidOrientation(psDroid);
    }
    if (*psDroid).inFire != 0 &&
           (*psDroid).type_0 as libc::c_uint !=
               DROID_PERSON as libc::c_int as libc::c_uint &&
           (*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0 {
        pos.x =
            (*psDroid).x as libc::c_int +
                (18 as libc::c_int - rand() % 36 as libc::c_int);
        pos.z =
            (*psDroid).y as libc::c_int +
                (18 as libc::c_int - rand() % 36 as libc::c_int);
        //		pos.y = map_Height(pos.x,pos.z) + (psDroid->sDisplay.imd->ymax/3);
        pos.y =
            (*psDroid).z as libc::c_int +
                (*(*psDroid).sDisplay.imd).ymax / 3 as libc::c_int;
        addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SMALL,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    }
    movePlayAudio(psDroid, bStarted, bStopped, moveSpeed);
}
