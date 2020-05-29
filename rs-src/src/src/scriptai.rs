use ::libc;
extern "C" {
    pub type _formation;
    pub type _viewdata;
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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Return the type of a droid from it's template */
    #[no_mangle]
    fn droidTemplateType(psTemplate: *mut DROID_TEMPLATE) -> DROID_TYPE;
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    fn IsPlayerStructureLimitReached(PlayerNumber: UDWORD) -> BOOL;
    #[no_mangle]
    fn validTemplateForFactory(psTemplate: *mut DROID_TEMPLATE,
                               psFactory: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn checkSpecificStructExists(structInc: UDWORD, player: UDWORD) -> BOOL;
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    // add a droid to a group
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // remove a droid from a group
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    //lasSat structure can select a target
    #[no_mangle]
    fn orderStructureObj(player: UDWORD, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn orderDroidStatsLocAdd(psDroid: *mut DROID, order: DROID_ORDER,
                             psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn orderDroidStatsTwoLocAdd(psDroid: *mut DROID, order: DROID_ORDER,
                                psStats: *mut BASE_STATS, x1: UDWORD,
                                y1: UDWORD, x2: UDWORD, y2: UDWORD);
    #[no_mangle]
    fn orderDroidStatsTwoLoc(psDroid: *mut DROID, order: DROID_ORDER,
                             psStats: *mut BASE_STATS, x1: UDWORD, y1: UDWORD,
                             x2: UDWORD, y2: UDWORD);
    //extern void orderGroupBase(DROID_GROUP *psGroup, struct _droid_order_data *psData);
    /* Give a group an order */
    #[no_mangle]
    fn orderGroup(psGroup: *mut DROID_GROUP, order: DROID_ORDER);
    /* Give a group of droids an order */
    #[no_mangle]
    fn orderGroupLoc(psGroup: *mut DROID_GROUP, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    /* Give a group of droids an order */
    #[no_mangle]
    fn orderGroupObj(psGroup: *mut DROID_GROUP, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    /* set the secondary state for a group of droids */
    #[no_mangle]
    fn grpSetSecondary(psGroup: *mut DROID_GROUP, sec: SECONDARY_ORDER,
                       state: SECONDARY_STATE);
    #[no_mangle]
    fn orderDroidStatsLoc(psDroid: *mut DROID, order: DROID_ORDER,
                          psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* **********************************************************************************
 *
 * Support functions for writing instinct functions
 */
    /* Pop a number of values off the stack checking their types
 * This is used by instinct functions to get their parameters
 * The varargs part is a set of INTERP_TYPE, UDWORD * pairs.
 * The value of the parameter is stored in the DWORD pointed to by the UDWORD *
 */
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
    /* Push a value onto the stack without using a value structure */
    #[no_mangle]
    fn stackPushResult(type_0: INTERP_TYPE, data: SDWORD) -> BOOL;
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
    //scroll min and max values
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    // Indirect the cluster ID to an actual cluster number
    #[no_mangle]
    static mut aClusterMap: [UBYTE; 255];
    // information about the cluster
    #[no_mangle]
    static mut aClusterInfo: [UBYTE; 255];
    // initialise iterating a cluster
    #[no_mangle]
    fn clustInitIterate(clusterID: SDWORD);
    // iterate a cluster
    #[no_mangle]
    fn clustIterate() -> *mut BASE_OBJECT;
    #[no_mangle]
    fn NETlogEntry(str: *mut CHAR, a: UDWORD, b: UDWORD) -> BOOL;
    #[no_mangle]
    fn cmdDroidAddDroid(psCommander: *mut DROID, psDroid: *mut DROID);
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    #[no_mangle]
    static mut numResearch: UDWORD;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
    // the list of gateways on the current map
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    /* Give a droid an action with an object target */
    #[no_mangle]
    fn actionDroidObj(psDroid: *mut DROID, action: DROID_ACTION,
                      psObj: *mut BASE_OBJECT);
    /*subtract the power required */
    #[no_mangle]
    fn usePower(player: UDWORD, quantity: UDWORD) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    #[no_mangle]
    fn dirtySqrt(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> UDWORD;
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
pub type CHAR = libc::c_char;
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
pub type BASE_STATS = _base_stats;
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
pub type RESEARCH = research_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/* the 2nd IMD for base plates/turrets*/
// If the research has been suspended then this value contains the number of points generated at the suspension/cancel point
// normally it is null  ... on the PSX if we cancel we lose all research so far... so it's not needed
// Bit flags   ...  see below
//	UBYTE		possible;				/* Flag to specify whether the research is possible - so
//										   can enable topics vis scripts */
//	UBYTE		researched;				/* Flag to specify whether the research is 
//										   complete	*/
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
pub type STRUCTURE = _structure;
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
pub type SECONDARY_STATE = _secondary_state;
pub type DROID_GROUP = _droid_group;
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
pub type INTERP_TYPE = _interp_type;
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
// ID numbers for each user type
pub type _scr_user_types = libc::c_uint;
// maximum possible type - should always be last
// NULL stats
pub const ST_MAXTYPE: _scr_user_types = 34;
//used so we can check for NULL templates
pub const ST_POINTER_S: _scr_user_types = 33;
//used so we can check for NULL objects etc
pub const ST_POINTER_T: _scr_user_types = 32;
// A research topic
//private types for game code - not for use in script
pub const ST_POINTER_O: _scr_user_types = 31;
// A group of droids
pub const ST_RESEARCH: _scr_user_types = 30;
// The name of a game level
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
// text string for display messages in tutorial
pub const ST_SOUND: _scr_user_types = 27;
// ID of a droid
pub const ST_TEXTSTRING: _scr_user_types = 26;
// feature stat type
pub const ST_DROIDID: _scr_user_types = 25;
// structure stat type
pub const ST_FEATURESTAT: _scr_user_types = 24;
/* A structure ID number (don't really 
											   need this since just a number?)*/
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
// Template object
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
// Component types
pub const ST_PROPULSION: _scr_user_types = 14;
// General component
pub const ST_BODY: _scr_user_types = 13;
// General stats type
pub const ST_COMPONENT: _scr_user_types = 12;
// Feature object
pub const ST_BASESTATS: _scr_user_types = 11;
// Structure object
pub const ST_FEATURE: _scr_user_types = 10;
// Droid object
pub const ST_STRUCTURE: _scr_user_types = 9;
// Base object
pub const ST_DROID: _scr_user_types = 8;
// Intelligence message ?? (6)
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
pub type _scr_struct_tar = libc::c_uint;
pub const SCR_ST_DEF_ALL: _scr_struct_tar = 28672;
pub const SCR_ST_DEF_IDF: _scr_struct_tar = 16384;
pub const SCR_ST_DEF_AIR: _scr_struct_tar = 8192;
pub const SCR_ST_DEF_GROUND: _scr_struct_tar = 4096;
pub const SCR_ST_SENSOR: _scr_struct_tar = 2048;
pub const SCR_ST_REARM_PAD: _scr_struct_tar = 1024;
pub const SCR_ST_VTOL_FACTORY: _scr_struct_tar = 512;
pub const SCR_ST_CYBORG_FACTORY: _scr_struct_tar = 256;
pub const SCR_ST_COMMAND_CONTROL: _scr_struct_tar = 128;
pub const SCR_ST_REPAIR_FACILITY: _scr_struct_tar = 64;
pub const SCR_ST_RESEARCH: _scr_struct_tar = 32;
pub const SCR_ST_WALL: _scr_struct_tar = 16;
pub const SCR_ST_RESOURCE_EXTRACTOR: _scr_struct_tar = 8;
pub const SCR_ST_POWER_GEN: _scr_struct_tar = 4;
pub const SCR_ST_FACTORY: _scr_struct_tar = 2;
pub const SCR_ST_HQ: _scr_struct_tar = 1;
pub type _scr_droid_tar = libc::c_uint;
pub const SCR_DT_HOVER: _scr_droid_tar = 65536;
pub const SCR_DT_VTOL: _scr_droid_tar = 32768;
pub const SCR_DT_GROUND: _scr_droid_tar = 30720;
pub const SCR_DT_LEGS: _scr_droid_tar = 16384;
pub const SCR_DT_WHEEL: _scr_droid_tar = 8192;
pub const SCR_DT_HTRACK: _scr_droid_tar = 4096;
pub const SCR_DT_TRACK: _scr_droid_tar = 2048;
pub const SCR_DT_SUPER_HEAVY: _scr_droid_tar = 1024;
pub const SCR_DT_HEAVY: _scr_droid_tar = 512;
pub const SCR_DT_MEDIUM: _scr_droid_tar = 256;
pub const SCR_DT_LIGHT: _scr_droid_tar = 128;
pub const SCR_DT_WEAP_ALL: _scr_droid_tar = 112;
pub const SCR_DT_WEAP_IDF: _scr_droid_tar = 64;
pub const SCR_DT_WEAP_AIR: _scr_droid_tar = 32;
pub const SCR_DT_WEAP_GROUND: _scr_droid_tar = 16;
pub const SCR_DT_REPAIR: _scr_droid_tar = 8;
pub const SCR_DT_CONSTRUCT: _scr_droid_tar = 4;
pub const SCR_DT_SENSOR: _scr_droid_tar = 2;
pub const SCR_DT_COMMAND: _scr_droid_tar = 1;
pub const SCR_TAR_STRUCT: C2RustUnnamed_0 = 0;
// see if one target is preferable to another
pub type TARGET_PREF
    =
    Option<unsafe extern "C" fn(_: *mut *mut BASE_OBJECT, _: *mut BASE_OBJECT)
               -> ()>;
// get target mask function for scrTargetInArea
pub type TARGET_MASK
    =
    Option<unsafe extern "C" fn(_: *mut BASE_OBJECT) -> UDWORD>;
pub const SCR_TAR_DROID: C2RustUnnamed_0 = 1;
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
pub type GATEWAY = _gateway;
// the flags that get reset by the router
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway {
    pub x1: UBYTE,
    pub y1: UBYTE,
    pub x2: UBYTE,
    pub y2: UBYTE,
    pub zone1: UBYTE,
    pub zone2: UBYTE,
    pub psNext: *mut _gateway,
    pub psLinks: *mut GATEWAY_LINK,
    pub zone1Links: UBYTE,
    pub zone2Links: UBYTE,
    pub flags: UBYTE,
    pub dist: SWORD,
    pub est: SWORD,
    pub psOpen: *mut _gateway,
    pub psRoute: *mut _gateway,
}
pub type GATEWAY_LINK = _gateway_link;
/*
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
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
pub const TER_WATER: _terrain_type = 7;
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
// what type of object for scrTargetInArea
pub type C2RustUnnamed_0 = libc::c_uint;
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
/*
 * ScriptAI.h
 *
 * Script functions to support the AI system
 *
 */
// Add a droid to a group
/*
 * ScriptAI.c
 *
 * Script functions to support the AI system
 *
 */
// various script tracing printf's
//#define DEBUG_GROUP0
// script order printf's
//#define DEBUG_GROUP1
// Add a droid to a group
#[no_mangle]
pub unsafe extern "C" fn scrGroupAddDroid() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAdd: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              43 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrGroupAddDroid\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAdd: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              45 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrGroupAddDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGroupAdd: cannot add a command droid to a group\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  53 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrGroupAddDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGroupAdd: cannot add a transporter to a group\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  59 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrGroupAddDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    grpJoin(psGroup, psDroid);
    return 1 as libc::c_int;
}
// Add droids in an area to a group
// Add droids in an area to a group
#[no_mangle]
pub unsafe extern "C" fn scrGroupAddArea() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut player: SDWORD = 0;
    if stackPopParams(6 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x2 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAdd: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              83 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"scrGroupAddArea\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGroupAddArea: invalid player\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  87 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrGroupAddArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).x as SDWORD >= x1 && (*psDroid).x as SDWORD <= x2 &&
               (*psDroid).y as SDWORD >= y1 && (*psDroid).y as SDWORD <= y2 &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            grpJoin(psGroup, psDroid);
        }
        psDroid = (*psDroid).psNext
    }
    return 1 as libc::c_int;
}
// Add groupless droids in an area to a group 
// Add groupless droids in an area to a group
#[no_mangle]
pub unsafe extern "C" fn scrGroupAddAreaNoGroup() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut player: SDWORD = 0;
    if stackPopParams(6 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x2 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAddNoGroup: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              124 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"scrGroupAddAreaNoGroup\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGroupAddAreaNoGroup: invalid player\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  128 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrGroupAddAreaNoGroup\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).x as SDWORD >= x1 && (*psDroid).x as SDWORD <= x2 &&
               (*psDroid).y as SDWORD >= y1 && (*psDroid).y as SDWORD <= y2 &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_COMMAND as libc::c_int as libc::c_uint &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               (*psDroid).psGroup.is_null() {
            grpJoin(psGroup, psDroid);
        }
        psDroid = (*psDroid).psNext
    }
    return 1 as libc::c_int;
}
// Move the droids from one group to another
// Move the droids from one group to another
#[no_mangle]
pub unsafe extern "C" fn scrGroupAddGroup() -> BOOL {
    let mut psTo: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psFrom: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psTo as *mut *mut DROID_GROUP,
                      ST_GROUP as libc::c_int,
                      &mut psFrom as *mut *mut DROID_GROUP) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAddGroup: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              160 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrGroupAddGroup\x00")).as_ptr(),
              b"PTRVALID(psTo, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupAddGroup: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              162 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrGroupAddGroup\x00")).as_ptr(),
              b"PTRVALID(psFrom, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psDroid = (*psFrom).psList;
    while !psDroid.is_null() {
        psNext = (*psDroid).psGrpNext;
        grpJoin(psTo, psDroid);
        psDroid = psNext
    }
    return 1 as libc::c_int;
}
// check if a droid is a member of a group
// check if a droid is a member of a group
#[no_mangle]
pub unsafe extern "C" fn scrGroupMember() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut retval: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupMember: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              187 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"scrGroupMember\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrGroupMember: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              189 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"scrGroupMember\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    if (*psDroid).psGroup == psGroup {
        retval = 1 as libc::c_int
    } else { retval = 0 as libc::c_int }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// return number of idle droids in group.
// returns number of idle droids in a group.
#[no_mangle]
pub unsafe extern "C" fn scrIdleGroup() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    if stackPopParams(1 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrIdleGroup: invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              226 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"scrIdleGroup\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psDroid = (*psGroup).psList;
    while !psDroid.is_null() {
        if (*psDroid).order == DORDER_NONE as libc::c_int ||
               (*psDroid).order == DORDER_GUARD as libc::c_int &&
                   (*psDroid).psTarget.is_null() {
            count = count.wrapping_add(1)
        }
        psDroid = (*psDroid).psGrpNext
    }
    if stackPushResult(VAL_INT, count as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// variables for the group iterator
#[no_mangle]
pub static mut psScrIterateGroup: *mut DROID_GROUP =
    0 as *const DROID_GROUP as *mut DROID_GROUP;
#[no_mangle]
pub static mut psScrIterateGroupDroid: *mut DROID =
    0 as *const DROID as *mut DROID;
// initialise iterating a groups members
// initialise iterating a groups members
#[no_mangle]
pub unsafe extern "C" fn scrInitIterateGroup() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if stackPopParams(1 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrInitGroupIterate: invalid group pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              259 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"scrInitIterateGroup\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psScrIterateGroup = psGroup;
    psScrIterateGroupDroid = (*psGroup).psList;
    return 1 as libc::c_int;
}
// iterate through a groups members
// iterate through a groups members
#[no_mangle]
pub unsafe extern "C" fn scrIterateGroup() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP) == 0 {
        return 0 as libc::c_int
    }
    if psGroup != psScrIterateGroup {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrIterateGroup: invalid group, InitGroupIterate not called?\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  282 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrIterateGroup\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if !psScrIterateGroupDroid.is_null() {
        psDroid = psScrIterateGroupDroid;
        psScrIterateGroupDroid = (*psScrIterateGroupDroid).psGrpNext
    } else { psDroid = 0 as *mut DROID }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psDroid as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// initialise iterating a cluster
// initialise iterating a cluster
#[no_mangle]
pub unsafe extern "C" fn scrInitIterateCluster() -> BOOL {
    let mut clusterID: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut clusterID as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    clustInitIterate(clusterID);
    return 1 as libc::c_int;
}
// iterate a cluster
// iterate a cluster
#[no_mangle]
pub unsafe extern "C" fn scrIterateCluster() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    psObj = clustIterate();
    if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                       psObj as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// remove a droid from a group
// remove a droid from a group
#[no_mangle]
pub unsafe extern "C" fn scrDroidLeaveGroup() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if !(*psDroid).psGroup.is_null() {
        grpLeave((*psDroid).psGroup, psDroid);
    }
    return 1 as libc::c_int;
}
// Give a group an order
// Give a group an order
#[no_mangle]
pub unsafe extern "C" fn scrOrderGroup() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut order: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut order as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderGroup: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              368 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrOrderGroup\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order != DORDER_STOP as libc::c_int &&
           order != DORDER_RETREAT as libc::c_int &&
           order != DORDER_DESTRUCT as libc::c_int &&
           order != DORDER_RTR as libc::c_int &&
           order != DORDER_RTB as libc::c_int &&
           order != DORDER_RUN as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderGroup: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  378 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrOrderGroup\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderGroup(psGroup, order as DROID_ORDER);
    return 1 as libc::c_int;
}
// Give a group an order to a location
// Give a group an order to a location
#[no_mangle]
pub unsafe extern "C" fn scrOrderGroupLoc() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut order: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut order as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderGroupLoc: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              401 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderGroupLoc\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order != DORDER_MOVE as libc::c_int &&
           order != DORDER_SCOUT as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderGroupLoc: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  407 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderGroupLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x > (mapWidth << 7 as libc::c_int) as SDWORD ||
           y < 0 as libc::c_int ||
           y > (mapHeight << 7 as libc::c_int) as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderGroupLoc: Invalid location\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  414 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderGroupLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderGroupLoc(psGroup, order as DROID_ORDER, x as UDWORD, y as UDWORD);
    return 1 as libc::c_int;
}
// Give a group an order to an object
// Give a group an order to an object
#[no_mangle]
pub unsafe extern "C" fn scrOrderGroupObj() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut order: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(3 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut order as *mut SDWORD,
                      ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderGroupObj: Invalid group pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              439 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderGroupObj\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderGroupObj: Invalid object pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              441 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderGroupObj\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if order != DORDER_ATTACK as libc::c_int &&
           order != DORDER_HELPBUILD as libc::c_int &&
           order != DORDER_DEMOLISH as libc::c_int &&
           order != DORDER_REPAIR as libc::c_int &&
           order != DORDER_OBSERVE as libc::c_int &&
           order != DORDER_EMBARK as libc::c_int &&
           order != DORDER_FIRESUPPORT as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderGroupObj: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  452 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderGroupObj\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderGroupObj(psGroup, order as DROID_ORDER, psObj);
    return 1 as libc::c_int;
}
// Give a Droid an order
// Give a droid an order
#[no_mangle]
pub unsafe extern "C" fn scrOrderDroid() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut order: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut order as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnit: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              475 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrOrderDroid\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    if order != DORDER_STOP as libc::c_int &&
           order != DORDER_RETREAT as libc::c_int &&
           order != DORDER_DESTRUCT as libc::c_int &&
           order != DORDER_RTR as libc::c_int &&
           order != DORDER_RTB as libc::c_int &&
           order != DORDER_RUN as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnit: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  489 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrOrderDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderDroid(psDroid, order as DROID_ORDER);
    return 1 as libc::c_int;
}
// Give a Droid an order to a location
// Give a Droid an order to a location
#[no_mangle]
pub unsafe extern "C" fn scrOrderDroidLoc() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut order: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut order as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitLoc: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              511 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderDroidLoc\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    if order != DORDER_MOVE as libc::c_int &&
           order != DORDER_SCOUT as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitLoc: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  521 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderDroidLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x > (mapWidth << 7 as libc::c_int) as SDWORD ||
           y < 0 as libc::c_int ||
           y > (mapHeight << 7 as libc::c_int) as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitLoc: Invalid location\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  528 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderDroidLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderDroidLoc(psDroid, order as DROID_ORDER, x as UDWORD, y as UDWORD);
    return 1 as libc::c_int;
}
// Give a Droid an order to an object
// Give a Droid an order to an object
#[no_mangle]
pub unsafe extern "C" fn scrOrderDroidObj() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut order: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(3 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut order as *mut SDWORD, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitObj: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              551 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderDroidObj\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitObj: Invalid object pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              553 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrOrderDroidObj\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() || psObj.is_null() { return 0 as libc::c_int }
    if order != DORDER_ATTACK as libc::c_int &&
           order != DORDER_HELPBUILD as libc::c_int &&
           order != DORDER_DEMOLISH as libc::c_int &&
           order != DORDER_REPAIR as libc::c_int &&
           order != DORDER_OBSERVE as libc::c_int &&
           order != DORDER_EMBARK as libc::c_int &&
           order != DORDER_FIRESUPPORT as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitObj: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  568 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOrderDroidObj\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    orderDroidObj(psDroid, order as DROID_ORDER, psObj);
    return 1 as libc::c_int;
}
// Give a Droid an order with a stat
// Give a Droid an order with a stat
#[no_mangle]
pub unsafe extern "C" fn scrOrderDroidStatsLoc() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut order: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut statIndex: SDWORD = 0;
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    if stackPopParams(5 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut order as *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut statIndex as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if statIndex < 0 as libc::c_int ||
           statIndex >= numStructureStats as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitStatsLoc: invalid structure stat\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  593 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrOrderDroidStatsLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(statIndex as isize) as *mut BASE_STATS;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitStatsLoc: Invalid Unit pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              599 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrOrderDroidStatsLoc\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitStatsLoc: Invalid object pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              601 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrOrderDroidStatsLoc\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(BASE_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    if x < 0 as libc::c_int || x > mapWidth as SDWORD * 128 as libc::c_int ||
           y < 0 as libc::c_int ||
           y > mapHeight as SDWORD * 128 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitStatsLoc: Invalid location\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  611 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrOrderDroidStatsLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if order != DORDER_BUILD as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrOrderUnitStatsLoc: Invalid order\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  618 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrOrderDroidStatsLoc\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // Don't allow scripts to order structure builds if players structure
	// limit has been reached.
    if IsPlayerStructureLimitReached((*psDroid).player as UDWORD) ==
           0 as libc::c_int {
        orderDroidStatsLoc(psDroid, order as DROID_ORDER, psStats,
                           x as UDWORD, y as UDWORD);
    }
    //#ifdef PSX
//	else {
//		BeepMessage(STR_GAM_MAXSTRUCTSREACHED);
//#endif
//	}
    return 1 as libc::c_int;
}
// set the secondary state for a droid
// set the secondary state for a droid
#[no_mangle]
pub unsafe extern "C" fn scrSetDroidSecondary() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut sec: SDWORD = 0;
    let mut state: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut sec as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut state as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrSetUnitSecondary: invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              649 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"scrSetDroidSecondary\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() { return 0 as libc::c_int }
    secondarySetState(psDroid, sec as SECONDARY_ORDER,
                      state as SECONDARY_STATE);
    return 1 as libc::c_int;
}
// set the secondary state for a droid
// set the secondary state for a droid
#[no_mangle]
pub unsafe extern "C" fn scrSetGroupSecondary() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut sec: SDWORD = 0;
    let mut state: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut sec as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut state as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrSetGroupSecondary: invalid group pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              672 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"scrSetGroupSecondary\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    grpSetSecondary(psGroup, sec as SECONDARY_ORDER,
                    state as SECONDARY_STATE);
    return 1 as libc::c_int;
}
// add a droid to a commander
// add a droid to a commander
#[no_mangle]
pub unsafe extern "C" fn scrCmdDroidAddDroid() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psCommander: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psCommander as *mut *mut DROID,
                      ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    cmdDroidAddDroid(psCommander, psDroid);
    return 1 as libc::c_int;
}
// store the prefered and ignore targets for the target functions
#[no_mangle]
pub static mut scrStructPref: UDWORD = 0;
#[no_mangle]
pub static mut scrStructIgnore: UDWORD = 0;
#[no_mangle]
pub static mut scrDroidPref: UDWORD = 0;
#[no_mangle]
pub static mut scrDroidIgnore: UDWORD = 0;
// reset the structure preferences
// reset the structure preferences
#[no_mangle]
pub unsafe extern "C" fn scrResetStructTargets() -> BOOL {
    scrStructPref = 0 as libc::c_int as UDWORD;
    scrStructIgnore = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
// reset the droid preferences
// reset the droid preferences
#[no_mangle]
pub unsafe extern "C" fn scrResetDroidTargets() -> BOOL {
    scrDroidPref = 0 as libc::c_int as UDWORD;
    scrDroidIgnore = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
// set prefered structure target types
// set prefered structure target types
#[no_mangle]
pub unsafe extern "C" fn scrSetStructTarPref() -> BOOL {
    let mut pref: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut pref as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if SCR_ST_HQ as libc::c_int as libc::c_uint == pref ||
           SCR_ST_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_POWER_GEN as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_WALL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESEARCH as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REPAIR_FACILITY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_COMMAND_CONTROL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_CYBORG_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_VTOL_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REARM_PAD as libc::c_int as libc::c_uint == pref ||
           SCR_ST_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_ALL as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"scrSetStructTarPref: unknown target preference\x00" as
                  *const u8 as *const libc::c_char);
    };
    if SCR_ST_HQ as libc::c_int as libc::c_uint == pref ||
           SCR_ST_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_POWER_GEN as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_WALL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESEARCH as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REPAIR_FACILITY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_COMMAND_CONTROL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_CYBORG_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_VTOL_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REARM_PAD as libc::c_int as libc::c_uint == pref ||
           SCR_ST_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_ALL as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              746 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"scrSetStructTarPref\x00")).as_ptr(),
              b"(SCR_ST_HQ == pref) || (SCR_ST_FACTORY == pref) || (SCR_ST_POWER_GEN == pref) || (SCR_ST_RESOURCE_EXTRACTOR == pref) || (SCR_ST_WALL == pref) || (SCR_ST_RESEARCH == pref) || (SCR_ST_REPAIR_FACILITY == pref) || (SCR_ST_COMMAND_CONTROL == pref) || (SCR_ST_CYBORG_FACTORY == pref) || (SCR_ST_VTOL_FACTORY == pref) || (SCR_ST_REARM_PAD == pref) || (SCR_ST_SENSOR == pref) || (SCR_ST_DEF_GROUND == pref) || (SCR_ST_DEF_AIR == pref) || (SCR_ST_DEF_IDF == pref) || (SCR_ST_DEF_ALL == pref)\x00"
                  as *const u8 as *const libc::c_char);
    };
    scrStructPref |= pref;
    return 1 as libc::c_int;
}
// set structure target ignore types
// set structure target ignore types
#[no_mangle]
pub unsafe extern "C" fn scrSetStructTarIgnore() -> BOOL {
    let mut pref: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut pref as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if SCR_ST_HQ as libc::c_int as libc::c_uint == pref ||
           SCR_ST_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_POWER_GEN as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_WALL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESEARCH as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REPAIR_FACILITY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_COMMAND_CONTROL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_CYBORG_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_VTOL_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REARM_PAD as libc::c_int as libc::c_uint == pref ||
           SCR_ST_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_ALL as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"scrSetStructTarIgnore: unknown ignore target\x00" as *const u8
                  as *const libc::c_char);
    };
    if SCR_ST_HQ as libc::c_int as libc::c_uint == pref ||
           SCR_ST_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_POWER_GEN as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_WALL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_RESEARCH as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REPAIR_FACILITY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_COMMAND_CONTROL as libc::c_int as libc::c_uint == pref ||
           SCR_ST_CYBORG_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_VTOL_FACTORY as libc::c_int as libc::c_uint == pref ||
           SCR_ST_REARM_PAD as libc::c_int as libc::c_uint == pref ||
           SCR_ST_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_ST_DEF_ALL as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              780 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrSetStructTarIgnore\x00")).as_ptr(),
              b"(SCR_ST_HQ == pref) || (SCR_ST_FACTORY == pref) || (SCR_ST_POWER_GEN == pref) || (SCR_ST_RESOURCE_EXTRACTOR == pref) || (SCR_ST_WALL == pref) || (SCR_ST_RESEARCH == pref) || (SCR_ST_REPAIR_FACILITY == pref) || (SCR_ST_COMMAND_CONTROL == pref) || (SCR_ST_CYBORG_FACTORY == pref) || (SCR_ST_VTOL_FACTORY == pref) || (SCR_ST_REARM_PAD == pref) || (SCR_ST_SENSOR == pref) || (SCR_ST_DEF_GROUND == pref) || (SCR_ST_DEF_AIR == pref) || (SCR_ST_DEF_IDF == pref) || (SCR_ST_DEF_ALL == pref)\x00"
                  as *const u8 as *const libc::c_char);
    };
    scrStructIgnore |= pref;
    return 1 as libc::c_int;
}
// set prefered droid target types
// set prefered droid target types
#[no_mangle]
pub unsafe extern "C" fn scrSetDroidTarPref() -> BOOL {
    let mut pref: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut pref as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if SCR_DT_COMMAND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_CONSTRUCT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_REPAIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_ALL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LIGHT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_MEDIUM as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SUPER_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_TRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HTRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WHEEL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LEGS as libc::c_int as libc::c_uint == pref ||
           SCR_DT_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_VTOL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HOVER as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"scrSetUnitTarPref: unknown target preference\x00" as *const u8
                  as *const libc::c_char);
    };
    if SCR_DT_COMMAND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_CONSTRUCT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_REPAIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_ALL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LIGHT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_MEDIUM as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SUPER_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_TRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HTRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WHEEL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LEGS as libc::c_int as libc::c_uint == pref ||
           SCR_DT_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_VTOL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HOVER as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              817 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"scrSetDroidTarPref\x00")).as_ptr(),
              b"(SCR_DT_COMMAND == pref) || (SCR_DT_SENSOR == pref) || (SCR_DT_CONSTRUCT == pref) || (SCR_DT_REPAIR == pref) || (SCR_DT_WEAP_GROUND == pref) || (SCR_DT_WEAP_AIR == pref) || (SCR_DT_WEAP_IDF == pref) || (SCR_DT_WEAP_ALL == pref) || (SCR_DT_LIGHT == pref) || (SCR_DT_MEDIUM == pref) || (SCR_DT_HEAVY == pref) || (SCR_DT_SUPER_HEAVY == pref) || (SCR_DT_TRACK == pref) || (SCR_DT_HTRACK == pref) || (SCR_DT_WHEEL == pref) || (SCR_DT_LEGS == pref) || (SCR_DT_GROUND == pref) || (SCR_DT_VTOL == pref) || (SCR_DT_HOVER == pref)\x00"
                  as *const u8 as *const libc::c_char);
    };
    scrDroidPref |= pref;
    return 1 as libc::c_int;
}
// set droid target ignore types
// set droid target ignore types
#[no_mangle]
pub unsafe extern "C" fn scrSetDroidTarIgnore() -> BOOL {
    let mut pref: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut pref as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if SCR_DT_COMMAND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_CONSTRUCT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_REPAIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_ALL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LIGHT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_MEDIUM as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SUPER_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_TRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HTRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WHEEL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LEGS as libc::c_int as libc::c_uint == pref ||
           SCR_DT_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_VTOL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HOVER as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"scrSetUnitTarIgnore: unknown ignore target\x00" as *const u8
                  as *const libc::c_char);
    };
    if SCR_DT_COMMAND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SENSOR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_CONSTRUCT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_REPAIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_AIR as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_IDF as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WEAP_ALL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LIGHT as libc::c_int as libc::c_uint == pref ||
           SCR_DT_MEDIUM as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_SUPER_HEAVY as libc::c_int as libc::c_uint == pref ||
           SCR_DT_TRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HTRACK as libc::c_int as libc::c_uint == pref ||
           SCR_DT_WHEEL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_LEGS as libc::c_int as libc::c_uint == pref ||
           SCR_DT_GROUND as libc::c_int as libc::c_uint == pref ||
           SCR_DT_VTOL as libc::c_int as libc::c_uint == pref ||
           SCR_DT_HOVER as libc::c_int as libc::c_uint == pref {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              854 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"scrSetDroidTarIgnore\x00")).as_ptr(),
              b"(SCR_DT_COMMAND == pref) || (SCR_DT_SENSOR == pref) || (SCR_DT_CONSTRUCT == pref) || (SCR_DT_REPAIR == pref) || (SCR_DT_WEAP_GROUND == pref) || (SCR_DT_WEAP_AIR == pref) || (SCR_DT_WEAP_IDF == pref) || (SCR_DT_WEAP_ALL == pref) || (SCR_DT_LIGHT == pref) || (SCR_DT_MEDIUM == pref) || (SCR_DT_HEAVY == pref) || (SCR_DT_SUPER_HEAVY == pref) || (SCR_DT_TRACK == pref) || (SCR_DT_HTRACK == pref) || (SCR_DT_WHEEL == pref) || (SCR_DT_LEGS == pref) || (SCR_DT_GROUND == pref) || (SCR_DT_VTOL == pref) || (SCR_DT_HOVER == pref)\x00"
                  as *const u8 as *const libc::c_char);
    };
    scrDroidIgnore |= pref;
    return 1 as libc::c_int;
}
// get the correct type mask for a structure target
#[no_mangle]
pub unsafe extern "C" fn scrStructTargetMask(mut psStruct: *mut STRUCTURE)
 -> UDWORD {
    let mut mask: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    psStats = (*psStruct).pStructureType;
    match (*psStats).type_0 {
        0 => { mask = SCR_ST_HQ as libc::c_int as UDWORD }
        1 | 2 => { mask = SCR_ST_FACTORY as libc::c_int as UDWORD }
        3 | 4 => { mask = SCR_ST_POWER_GEN as libc::c_int as UDWORD }
        5 => { mask = SCR_ST_RESOURCE_EXTRACTOR as libc::c_int as UDWORD }
        6 => {
            //if (psStats->numWeaps == 0 && psStats->pSensor != NULL)
            if (*psStats).psWeapStat.is_null() &&
                   !(*psStats).pSensor.is_null() {
                mask = SCR_ST_SENSOR as libc::c_int as UDWORD
            } else if !(*psStats).psWeapStat.is_null() {
                //else if (psStats->numWeaps > 0)
                //psWStats = psStats->asWeapList[0];
                psWStats = (*psStats).psWeapStat; // ignore walls for now
                if proj_Direct(psWStats) == 0 {
                    mask = SCR_ST_DEF_IDF as libc::c_int as UDWORD
                } else if (*psWStats).surfaceToAir as libc::c_int &
                              0x2 as libc::c_int != 0 {
                    mask = SCR_ST_DEF_AIR as libc::c_int as UDWORD
                } else { mask = SCR_ST_DEF_GROUND as libc::c_int as UDWORD }
            }
        }
        7 | 8 => { mask = 0 as libc::c_int as UDWORD }
        10 | 11 => { mask = SCR_ST_RESEARCH as libc::c_int as UDWORD }
        12 => { mask = SCR_ST_REPAIR_FACILITY as libc::c_int as UDWORD }
        13 => { mask = SCR_ST_COMMAND_CONTROL as libc::c_int as UDWORD }
        16 => { mask = SCR_ST_CYBORG_FACTORY as libc::c_int as UDWORD }
        17 => { mask = SCR_ST_VTOL_FACTORY as libc::c_int as UDWORD }
        19 => { mask = SCR_ST_REARM_PAD as libc::c_int as UDWORD }
        20 => {
            //don't want the assert!
            mask = 0 as libc::c_int as UDWORD
        }
        18 | 14 | 15 | 9 | _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrStructTargetMask: unknown or invalid target structure type\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                      944 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"scrStructTargetMask\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            mask = 0 as libc::c_int as UDWORD
        }
    }
    return mask;
}
// prioritise structure targets
#[no_mangle]
pub unsafe extern "C" fn scrStructTargetPriority(mut ppsTarget:
                                                     *mut *mut STRUCTURE,
                                                 mut psCurr: *mut STRUCTURE) {
    // skip walls unless they are explicitly asked for
    if scrStructPref & SCR_ST_WALL as libc::c_int as libc::c_uint != 0 ||
           (*(*psCurr).pStructureType).type_0 !=
               REF_WALL as libc::c_int as libc::c_uint &&
               (*(*psCurr).pStructureType).type_0 !=
                   REF_WALLCORNER as libc::c_int as libc::c_uint {
        *ppsTarget = psCurr
    };
}
#[no_mangle]
pub unsafe extern "C" fn scrDroidTargetMask(mut psDroid: *mut DROID)
 -> UDWORD {
    let mut mask: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut psBStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut psPStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    // get the turret type
    match (*psDroid).droidType as libc::c_uint {
        4 | 5 | 12 | 0 => {
            //if (psDroid->numWeaps > 0)
            if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                   0 as libc::c_int as libc::c_uint {
                psWStats =
                    asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int
                                                                as
                                                                usize].nStat
                                             as isize);
                if proj_Direct(psWStats) == 0 {
                    mask |= SCR_DT_WEAP_IDF as libc::c_int as libc::c_uint
                } else if (*psWStats).surfaceToAir as libc::c_int &
                              0x2 as libc::c_int != 0 {
                    mask |= SCR_DT_WEAP_AIR as libc::c_int as libc::c_uint
                } else {
                    mask |= SCR_DT_WEAP_GROUND as libc::c_int as libc::c_uint
                }
            } else { mask |= SCR_DT_SENSOR as libc::c_int as libc::c_uint }
        }
        1 | 2 => { mask |= SCR_DT_SENSOR as libc::c_int as libc::c_uint }
        3 | 10 => { mask |= SCR_DT_CONSTRUCT as libc::c_int as libc::c_uint }
        7 => { mask |= SCR_DT_COMMAND as libc::c_int as libc::c_uint }
        8 | 11 => { mask |= SCR_DT_REPAIR as libc::c_int as libc::c_uint }
        6 => { }
        9 | 13 | _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrUnitTargetMask: unknown or invalid target unit type\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                      1023 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"scrDroidTargetMask\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    // get the body type
    psBStats =
        asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                 usize].nStat as libc::c_int
                               as isize);
    match (*psBStats).size as libc::c_int {
        0 => { mask |= SCR_DT_LIGHT as libc::c_int as libc::c_uint }
        1 => { mask |= SCR_DT_MEDIUM as libc::c_int as libc::c_uint }
        2 => { mask |= SCR_DT_HEAVY as libc::c_int as libc::c_uint }
        3 => { mask |= SCR_DT_SUPER_HEAVY as libc::c_int as libc::c_uint }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrUnitTargetMask: unknown or invalid target body size\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                      1045 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"scrDroidTargetMask\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    // get the propulsion type
    psPStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    match (*psPStats).propulsionType as libc::c_int {
        0 => { mask |= SCR_DT_WHEEL as libc::c_int as libc::c_uint }
        1 => { mask |= SCR_DT_TRACK as libc::c_int as libc::c_uint }
        2 | 8 => { mask |= SCR_DT_LEGS as libc::c_int as libc::c_uint }
        3 => { mask |= SCR_DT_HOVER as libc::c_int as libc::c_uint }
        5 => { mask |= SCR_DT_VTOL as libc::c_int as libc::c_uint }
        7 => { mask |= SCR_DT_HTRACK as libc::c_int as libc::c_uint }
        6 | 4 | _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrUnitTargetMask: unknown or invalid target unit propulsion type\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                      1076 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"scrDroidTargetMask\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    return mask;
}
// prioritise droid targets
#[no_mangle]
pub unsafe extern "C" fn scrDroidTargetPriority(mut ppsTarget:
                                                    *mut *mut DROID,
                                                mut psCurr: *mut DROID) {
    // priority to things with weapons
    if (*ppsTarget).is_null() ||
           (**ppsTarget).asWeaps[0 as libc::c_int as usize].nStat ==
               0 as libc::c_int as libc::c_uint {
        *ppsTarget = psCurr
    };
}
// generic find a target in an area given preference data
#[no_mangle]
pub unsafe extern "C" fn scrTargetInArea(mut tarPlayer: SDWORD,
                                         mut visPlayer: SDWORD,
                                         mut tarType: SDWORD,
                                         mut cluster: SDWORD, mut x1: SDWORD,
                                         mut y1: SDWORD, mut x2: SDWORD,
                                         mut y2: SDWORD) -> *mut BASE_OBJECT {
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut temp: SDWORD = 0;
    let mut bVisCheck: BOOL = 0;
    let mut tarMask: UDWORD = 0;
    let mut getTargetMask: TARGET_MASK = None;
    let mut targetPriority: TARGET_PREF = None;
    let mut prefer: UDWORD = 0;
    let mut ignore: UDWORD = 0;
    if tarPlayer < 0 as libc::c_int || tarPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTargetInArea: invalid target player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  1125 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrTargetInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as *mut BASE_OBJECT
    }
    if x1 > x2 { temp = x2; x2 = x1; x1 = temp }
    if y1 > y2 { temp = y2; y2 = y1; y1 = temp }
    // see if a visibility check is required and for which player
    if visPlayer < 0 as libc::c_int || visPlayer >= 8 as libc::c_int {
        bVisCheck = 0 as libc::c_int
    } else { bVisCheck = 1 as libc::c_int }
    bVisCheck = 0 as libc::c_int;
    // see which target type to use
    match tarType {
        0 => {
            getTargetMask =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut STRUCTURE)
                                                   -> UDWORD>,
                                        TARGET_MASK>(Some(scrStructTargetMask
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut STRUCTURE)
                                                                  -> UDWORD));
            targetPriority =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut *mut STRUCTURE,
                                                                    _:
                                                                        *mut STRUCTURE)
                                                   -> ()>,
                                        TARGET_PREF>(Some(scrStructTargetPriority
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut *mut STRUCTURE,
                                                                                   _:
                                                                                       *mut STRUCTURE)
                                                                  -> ()));
            prefer = scrStructPref;
            ignore = scrStructIgnore;
            psCurr = apsStructLists[tarPlayer as usize] as *mut BASE_OBJECT
        }
        1 => {
            getTargetMask =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut DROID)
                                                   -> UDWORD>,
                                        TARGET_MASK>(Some(scrDroidTargetMask
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut DROID)
                                                                  -> UDWORD));
            targetPriority =
                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                        *mut *mut DROID,
                                                                    _:
                                                                        *mut DROID)
                                                   -> ()>,
                                        TARGET_PREF>(Some(scrDroidTargetPriority
                                                              as
                                                              unsafe extern "C" fn(_:
                                                                                       *mut *mut DROID,
                                                                                   _:
                                                                                       *mut DROID)
                                                                  -> ()));
            prefer = scrDroidPref;
            ignore = scrDroidIgnore;
            psCurr = apsDroidLists[tarPlayer as usize] as *mut BASE_OBJECT
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrTargetInArea: invalid target type\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                      1171 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"scrTargetInArea\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut BASE_OBJECT
        }
    }
    psTarget = 0 as *mut BASE_OBJECT;
    while !psCurr.is_null() {
        if (bVisCheck == 0 ||
                (*psCurr).visible[visPlayer as usize] as libc::c_int != 0) &&
               (cluster == 0 as libc::c_int ||
                    (*psCurr).cluster as libc::c_int == cluster) &&
               (*psCurr).x as SDWORD >= x1 && (*psCurr).x as SDWORD <= x2 &&
               (*psCurr).y as SDWORD >= y1 && (*psCurr).y as SDWORD <= y2 {
            tarMask =
                getTargetMask.expect("non-null function pointer")(psCurr);
            if !(tarMask & ignore != 0) {
                if tarMask & prefer != 0 {
                    // got a prefered target - go with that
                    psTarget = psCurr;
                    break ;
                } else {
                    // see if the current target has priority over the previous one
                    targetPriority.expect("non-null function pointer")(&mut psTarget,
                                                                       psCurr);
                }
            }
        }
        // skip any that match ignore
        psCurr = (*psCurr).psNext
    }
    return psTarget;
}
// get a structure target in an area using the preferences
// get a structure target in an area using the preferences
#[no_mangle]
pub unsafe extern "C" fn scrStructTargetInArea() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut tarPlayer: SDWORD = 0;
    let mut visPlayer: SDWORD = 0;
    let mut psTarget: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tarPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut visPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psTarget =
        scrTargetInArea(tarPlayer, visPlayer, SCR_TAR_STRUCT as libc::c_int,
                        0 as libc::c_int, x1, y1, x2, y2) as *mut STRUCTURE;
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psTarget as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// get a structure target on the map using the preferences
// get a structure target on the map using the preferences
#[no_mangle]
pub unsafe extern "C" fn scrStructTargetOnMap() -> BOOL {
    let mut tarPlayer: SDWORD = 0;
    let mut visPlayer: SDWORD = 0;
    let mut psTarget: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tarPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut visPlayer as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psTarget =
        scrTargetInArea(tarPlayer, visPlayer, SCR_TAR_STRUCT as libc::c_int,
                        0 as libc::c_int, scrollMinX * 128 as libc::c_int,
                        scrollMinY * 128 as libc::c_int,
                        scrollMaxX * 128 as libc::c_int,
                        scrollMaxY * 128 as libc::c_int) as *mut STRUCTURE;
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psTarget as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// get a droid target in an area using the preferences
// get a droid target in an area using the preferences
#[no_mangle]
pub unsafe extern "C" fn scrDroidTargetInArea() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut tarPlayer: SDWORD = 0;
    let mut visPlayer: SDWORD = 0;
    let mut psTarget: *mut DROID = 0 as *mut DROID;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tarPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut visPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psTarget =
        scrTargetInArea(tarPlayer, visPlayer, SCR_TAR_DROID as libc::c_int,
                        0 as libc::c_int, x1, y1, x2, y2) as *mut DROID;
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psTarget as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// get a droid target on the map using the preferences
// get a droid target on the map using the preferences
#[no_mangle]
pub unsafe extern "C" fn scrDroidTargetOnMap() -> BOOL {
    let mut tarPlayer: SDWORD = 0;
    let mut visPlayer: SDWORD = 0;
    let mut psTarget: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tarPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut visPlayer as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psTarget =
        scrTargetInArea(tarPlayer, visPlayer, SCR_TAR_DROID as libc::c_int,
                        0 as libc::c_int, scrollMinX * 128 as libc::c_int,
                        scrollMinY * 128 as libc::c_int,
                        scrollMaxX * 128 as libc::c_int,
                        scrollMaxY * 128 as libc::c_int) as *mut DROID;
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psTarget as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// get a target from a cluster using the preferences
// get a target from a cluster using the preferences
#[no_mangle]
pub unsafe extern "C" fn scrTargetInCluster() -> BOOL {
    let mut tarPlayer: SDWORD = 0;
    let mut tarType: SDWORD = 0;
    let mut visPlayer: SDWORD = 0;
    let mut clusterID: SDWORD = 0;
    let mut cluster: SDWORD = 0;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut clusterID as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut visPlayer as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if clusterID < 0 as libc::c_int || clusterID >= 0xff as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTargetInCluster: invalid clusterID\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  1314 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrTargetInCluster\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    cluster = aClusterMap[clusterID as usize] as SDWORD;
    tarPlayer =
        aClusterInfo[cluster as usize] as libc::c_int & 0x7 as libc::c_int;
    tarType =
        if aClusterInfo[cluster as usize] as libc::c_int & 0x8 as libc::c_int
               != 0 {
            SCR_TAR_DROID as libc::c_int
        } else { SCR_TAR_STRUCT as libc::c_int };
    psTarget =
        scrTargetInArea(tarPlayer, visPlayer, tarType, cluster,
                        scrollMinX * 128 as libc::c_int,
                        scrollMinY * 128 as libc::c_int,
                        scrollMaxX * 128 as libc::c_int,
                        scrollMaxY * 128 as libc::c_int);
    if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                       psTarget as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// check a template
// ********************************************************************************************
// ********************************************************************************************
// Additional Skirmish Code for superduper skirmish games may99
// ********************************************************************************************
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn scrSkCanBuildTemplate() -> BOOL {
    let mut current_block: u64;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut player: SDWORD = 0;
    let mut structure: SDWORD = 0;
    let mut templ: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, ST_STRUCTURE as libc::c_int,
                      &mut structure as *mut SDWORD,
                      ST_TEMPLATE as libc::c_int, &mut templ as *mut SDWORD)
           == 0 {
        return 0 as libc::c_int
    }
    psTempl = templ as *mut DROID_TEMPLATE;
    psStructure = structure as *mut STRUCTURE;
    // is factory big enough?
    if !(validTemplateForFactory(psTempl, psStructure) == 0) {
        if !((*asBodyStats.offset((*psTempl).asParts[COMP_BODY as libc::c_int
                                                         as usize] as
                                      isize)).size as libc::c_int >
                 (*((*psStructure).pFunctionality as *mut FACTORY)).capacity
                     as libc::c_int) {
            // is every component from template available?
	// body available.
            if !(*apCompLists[player as
                                  usize][COMP_BODY as libc::c_int as
                                             usize].offset((*psTempl).asParts[COMP_BODY
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  usize]
                                                               as isize) as
                     libc::c_int != 0x1 as libc::c_int) {
                // propulsion method available.
                if !(*apCompLists[player as
                                      usize][COMP_PROPULSION as libc::c_int as
                                                 usize].offset((*psTempl).asParts[COMP_PROPULSION
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      usize]
                                                                   as isize)
                         as libc::c_int != 0x1 as libc::c_int) {
                    // weapon/sensor
                    match droidTemplateType(psTempl) as libc::c_uint {
                        5 => {
                            // cyborg-type thang.. no need to check weapon.
                            current_block = 11459959175219260272;
                        }
                        12 => { current_block = 11459959175219260272; }
                        0 => {
                            if *apCompLists[player as
                                                usize][COMP_WEAPON as
                                                           libc::c_int as
                                                           usize].offset((*psTempl).asWeaps[0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                             as
                                                                             isize)
                                   as libc::c_int != 0x1 as libc::c_int {
                                current_block = 17332787028948588208;
                            } else { current_block = 11459959175219260272; }
                        }
                        1 => {
                            if *apCompLists[player as
                                                usize][COMP_SENSOR as
                                                           libc::c_int as
                                                           usize].offset((*psTempl).asParts[COMP_SENSOR
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                             as
                                                                             isize)
                                   as libc::c_int != 0x1 as libc::c_int {
                                current_block = 17332787028948588208;
                            } else { current_block = 11459959175219260272; }
                        }
                        2 => {
                            if *apCompLists[player as
                                                usize][COMP_ECM as libc::c_int
                                                           as
                                                           usize].offset((*psTempl).asParts[COMP_ECM
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                             as
                                                                             isize)
                                   as libc::c_int != 0x1 as libc::c_int {
                                current_block = 17332787028948588208;
                            } else { current_block = 11459959175219260272; }
                        }
                        8 => {
                            if *apCompLists[player as
                                                usize][COMP_REPAIRUNIT as
                                                           libc::c_int as
                                                           usize].offset((*psTempl).asParts[COMP_REPAIRUNIT
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                usize]
                                                                             as
                                                                             isize)
                                   as libc::c_int != 0x1 as libc::c_int {
                                current_block = 17332787028948588208;
                            } else { current_block = 11459959175219260272; }
                        }
                        7 | 3 => {
                            // Constructor droid
                            current_block = 17731332437551765381;
                        }
                        4 => { current_block = 17731332437551765381; }
                        10 => { current_block = 7529742765864360335; }
                        11 => { current_block = 18293408742361555136; }
                        6 => { current_block = 6404720315412020058; }
                        9 | 13 | _ => {
                            current_block = 13164329089130377634;
                        }
                    }
                    match current_block {
                        17332787028948588208 => { }
                        _ => {
                            match current_block {
                                11459959175219260272 =>
                                // super cyborg-type thang
                                {
                                    if stackPushResult(VAL_BOOL,
                                                       1 as libc::c_int) == 0
                                       {
                                        // yes
                                        return 0 as libc::c_int
                                    }
                                    return 1 as libc::c_int
                                }
                                17731332437551765381 =>
                                // person
                                {
                                    current_block = 7529742765864360335;
                                }
                                _ => { }
                            }
                            match current_block {
                                7529742765864360335 =>
                                // cyborg-construct thang
                                {
                                    current_block = 18293408742361555136;
                                }
                                _ => { }
                            }
                            match current_block {
                                18293408742361555136 =>
                                // cyborg-repair thang
                                {
                                    current_block = 6404720315412020058;
                                }
                                _ => { }
                            }
                            match current_block {
                                6404720315412020058 =>
                                // guess what this is!
                                {
                                }
                                _ => { }
                            }
                            // Default droid
                            debug(LOG_ERROR,
                                  b"scrSkCanBuildTemplate: Unhandled template type\x00"
                                      as *const u8 as *const libc::c_char);
                            abort();
                        }
                    }
                }
            }
        }
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        // no
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// find the human players
// ********************************************************************************************
// locate the enemy
// gives a target location given a player to attack.
#[no_mangle]
pub unsafe extern "C" fn scrSkLocateEnemy() -> BOOL {
    let mut player: SDWORD = 0; //,*x,*y;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // find where the player has some structures..	// factories or hq.
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() &&
              !((*(*psStruct).pStructureType).type_0 ==
                    REF_HQ as libc::c_int as libc::c_uint ||
                    (*(*psStruct).pStructureType).type_0 ==
                        REF_FACTORY as libc::c_int as libc::c_uint ||
                    (*(*psStruct).pStructureType).type_0 ==
                        REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                    (*(*psStruct).pStructureType).type_0 ==
                        REF_VTOL_FACTORY as libc::c_int as libc::c_uint) {
        psStruct = (*psStruct).psNext
    }
    // set the x and y accordingly..
    if !psStruct.is_null() {
        if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                           psStruct as SDWORD) == 0 {
            // success!
            return 0 as libc::c_int
        }
    } else if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                              0 as libc::c_int) == 0 {
        // part success
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn skTopicAvail(mut inc: UWORD, mut player: UDWORD)
 -> BOOL {
    let mut incPR: UDWORD = 0;
    let mut incS: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[player as usize];
    let mut bPRFound: BOOL = 0;
    let mut bStructFound: BOOL = 0;
    //if the topic is possible and has not already been researched - add to list
    if (*pPlayerRes.offset(inc as isize)).ResearchStatus as libc::c_int &
           0x80 as libc::c_int != 0 {
        if (*pPlayerRes.offset(inc as isize)).ResearchStatus as libc::c_int &
               0x4 as libc::c_int == 0 as libc::c_int &&
               (*pPlayerRes.offset(inc as isize)).ResearchStatus as
                   libc::c_int & 0x1 as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        }
    }
    // make sure that the research is not completed  or started by another researchfac
    if (*pPlayerRes.offset(inc as isize)).ResearchStatus as libc::c_int &
           0x4 as libc::c_int == 0 as libc::c_int &&
           (*pPlayerRes.offset(inc as isize)).ResearchStatus as libc::c_int &
               0x1 as libc::c_int == 0 as libc::c_int {
        // Research is not completed  ... also  it has not been started by another researchfac
        //if there aren't any PR's - go to next topic
        if (*asResearch.offset(inc as isize)).numPRRequired == 0 {
            return 0 as libc::c_int
        }
        //check for pre-requisites
        bPRFound = 1 as libc::c_int;
        incPR = 0 as libc::c_int as UDWORD;
        while incPR <
                  (*asResearch.offset(inc as isize)).numPRRequired as
                      libc::c_uint {
            if (*pPlayerRes.offset(*(*asResearch.offset(inc as
                                                            isize)).pPRList.offset(incPR
                                                                                       as
                                                                                       isize)
                                       as isize)).ResearchStatus as
                   libc::c_int & 0x4 as libc::c_int == 0 as libc::c_int {
                //if haven't pre-requisite - quit checking rest
                bPRFound = 0 as libc::c_int;
                break ;
            } else { incPR = incPR.wrapping_add(1) }
        }
        if bPRFound == 0 {
            //if haven't pre-requisites, skip the rest of the checks
            return 0 as libc::c_int
        }
        //check for structure effects
        bStructFound = 1 as libc::c_int;
        incS = 0 as libc::c_int as UDWORD;
        while incS <
                  (*asResearch.offset(inc as isize)).numStructures as
                      libc::c_uint {
            //if (!checkStructureStatus(asStructureStats + asResearch[inc].
			//	pStructList[incS], playerID, SS_BUILT))
            if checkSpecificStructExists(*(*asResearch.offset(inc as
                                                                  isize)).pStructList.offset(incS
                                                                                                 as
                                                                                                 isize)
                                             as UDWORD, player) == 0 {
                //if not built, quit checking
                bStructFound = 0 as libc::c_int;
                break ;
            } else { incS = incS.wrapping_add(1) }
        }
        if bStructFound == 0 {
            //if haven't all structs built, skip to next topic
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Skirmish funcs may99
// choose and do research
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn scrSkDoResearch() -> BOOL {
    let mut structure: SDWORD = 0; //,timeToResearch;//,*x,*y;
    let mut player: SDWORD = 0;
    let mut bias: SDWORD = 0;
    let mut i: UWORD = 0;
    let mut sTemp: [STRING; 128] = [0; 128];
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psResFacilty: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    if stackPopParams(3 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut structure as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut bias as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psBuilding = structure as *mut STRUCTURE;
    psResFacilty = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
    if !(*psResFacilty).psSubject.is_null() {
        // not finshed yet..
        return 1 as libc::c_int
    }
    // choose a topic to complete.
    i = 0 as libc::c_int as UWORD; //set the subject up
    while (i as libc::c_uint) < numResearch {
        if skTopicAvail(i, player as UDWORD) != 0 { break ; }
        i = i.wrapping_add(1)
    }
    if i as libc::c_uint != numResearch {
        pResearch = asResearch.offset(i as libc::c_int as isize);
        pPlayerRes =
            asPlayerResList[player as
                                usize].offset(i as libc::c_int as isize);
        (*psResFacilty).psSubject = pResearch as *mut BASE_STATS;
        if (*pPlayerRes).ResearchStatus as libc::c_int & 0x2 as libc::c_int !=
               0 {
            (*psResFacilty).powerAccrued = (*pResearch).researchPower
            //set up as if all power available for cancelled topics
        } else { (*psResFacilty).powerAccrued = 0 as libc::c_int as UDWORD }
        (*pPlayerRes).ResearchStatus =
            ((*pPlayerRes).ResearchStatus as libc::c_int &
                 !(0x1 as libc::c_int | 0x2 as libc::c_int |
                       0x4 as libc::c_int) | 0x1 as libc::c_int) as UBYTE;
        (*psResFacilty).timeStarted = 0 as libc::c_int as UDWORD;
        (*psResFacilty).timeStartHold = 0 as libc::c_int as UDWORD;
        (*psResFacilty).timeToResearch =
            ((*pResearch).researchPoints as
                 libc::c_uint).wrapping_div((*psResFacilty).researchPoints);
        if (*psResFacilty).timeToResearch == 0 as libc::c_int as libc::c_uint
           {
            (*psResFacilty).timeToResearch = 1 as libc::c_int as UDWORD
        }
        sprintf(sTemp.as_mut_ptr(),
                b"player:%d starts topic: %s\x00" as *const u8 as
                    *const libc::c_char, player,
                (*asResearch.offset(i as isize)).pName);
        NETlogEntry(sTemp.as_mut_ptr(), 0 as libc::c_int as UDWORD,
                    0 as libc::c_int as UDWORD);
    }
    return 1 as libc::c_int;
    /*
	// do it.
	if(i != numResearch)
	{
		researchResult(i,(UBYTE)player,FALSE);

		sprintf(sTemp,"player:%d did topic: %s",player, asResearch[i].pName );
		NETlogEntry(sTemp,0,0);

		SendResearch((UBYTE)player,i );
	}

	// set delay for next topic.


	timeToResearch = (asResearch+i)->researchPoints / ((RESEARCH_FACILITY*)psResearch->pFunctionality)->researchPoints;;

	if (!stackPushResult(VAL_INT, timeToResearch))		// return time to do it..
	{
		return FALSE;
	}
*/
    //	UDWORD				count;
}
// check for vtol availability
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn scrSkVtolEnableCheck() -> BOOL {
    let mut player: SDWORD = 0;
    let mut i: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // vtol factory
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats &&
              (*asStructureStats.offset(i as isize)).type_0 !=
                  REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        i = i.wrapping_add(1)
    }
    if i < numStructureStats &&
           *apStructTypeLists[player as usize].offset(i as isize) as
               libc::c_int == 0x1 as libc::c_int {
        // vtol propulsion
        i = 0 as libc::c_int as UDWORD;
        while i < numPropulsionStats {
            if (*asPropulsionStats.offset(i as isize)).propulsionType as
                   libc::c_int == LIFT as libc::c_int &&
                   *apCompLists[player as
                                    usize][COMP_PROPULSION as libc::c_int as
                                               usize].offset(i as isize) as
                       libc::c_int == 0x1 as libc::c_int {
                if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                    // success!
                    return 0 as libc::c_int
                }
                return 1 as libc::c_int
            }
            i = i.wrapping_add(1)
        }
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        // success!
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// check capacity
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn scrSkGetFactoryCapacity() -> BOOL {
    let mut count: SDWORD = 0 as libc::c_int;
    let mut structure: SDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut structure as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psStructure = structure as *mut STRUCTURE;
    if !psStructure.is_null() && StructIsFactory(psStructure) != 0 {
        count =
            (*((*psStructure).pFunctionality as *mut FACTORY)).capacity as
                SDWORD
    }
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// help/hinder player.
// ********************************************************************************************
#[no_mangle]
pub unsafe extern "C" fn scrSkDifficultyModifier() -> BOOL {
    let mut amount: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psResFacility: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    let mut psStr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // power modifier
    amount =
        game.skDiff[player as usize] as libc::c_int *
            40 as libc::c_int; //(0-20)*25
    if amount > 0 as libc::c_int {
        addPower(player as UDWORD, amount as UDWORD);
    } else {
        usePower(player as UDWORD, (0 as libc::c_int - amount) as UDWORD);
    }
    //research modifier.??
    psStr = apsStructLists[player as usize];
    while !psStr.is_null() {
        //		subtract 0 - 60% off the time to research.
        if (*(*psStr).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
            psResFacility = (*psStr).pFunctionality as *mut RESEARCH_FACILITY;
            /*if(psResFacility->timeToResearch )
			{
				amount = psResFacility->timeToResearch;
				psResFacility->timeToResearch  = amount - ( (amount*3*game.skDiff[player])/100);
			}*/
            //this is not appropriate now the timeToResearch is not used - 10/06/99 so...
            //add 0-60% to the amount required to research
            if !(*psResFacility).psSubject.is_null() {
                pPlayerRes =
                    asPlayerResList[player as
                                        usize].offset((*((*psResFacility).psSubject
                                                             as
                                                             *mut RESEARCH)).ref_0.wrapping_sub(0xb0000
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                          as isize);
                (*pPlayerRes).currentPoints =
                    ((*pPlayerRes).currentPoints as
                         libc::c_uint).wrapping_add(((*((*psResFacility).psSubject
                                                            as
                                                            *mut RESEARCH)).researchPoints
                                                         as libc::c_int *
                                                         3 as libc::c_int *
                                                         game.skDiff[player as
                                                                         usize]
                                                             as libc::c_int /
                                                         100 as libc::c_int)
                                                        as libc::c_uint) as
                        UDWORD as UDWORD
            }
        }
        psStr = (*psStr).psNext
    }
    //free stuff??
    return 1 as libc::c_int;
}
// pick good spots.
// ********************************************************************************************
// return a good place to build a defence, given a starting point
#[no_mangle]
pub unsafe extern "C" fn scrSkDefenseLocation() -> BOOL {
    let mut pX: *mut SDWORD = 0 as *mut SDWORD;
    let mut pY: *mut SDWORD = 0 as *mut SDWORD;
    let mut statIndex: SDWORD = 0;
    let mut statIndex2: SDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut gX: UDWORD = 0;
    let mut gY: UDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut nearestSoFar: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut psGate: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psChosenGate: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psWStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut x1: UDWORD = 0;
    let mut x2: UDWORD = 0;
    let mut x3: UDWORD = 0;
    let mut x4: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut y2: UDWORD = 0;
    let mut y3: UDWORD = 0;
    let mut y4: UDWORD = 0;
    let mut noWater: BOOL = 0;
    if stackPopParams(6 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pX as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pY as *mut *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut statIndex as *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut statIndex2 as *mut SDWORD, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSkDefenseLocation:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  1789 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrSkDefenseLocation\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(statIndex as isize) as *mut BASE_STATS;
    psWStats =
        asStructureStats.offset(statIndex2 as isize) as *mut BASE_STATS;
    // check for wacky coords.
    if !(*pX < 0 as libc::c_int ||
             *pX > (mapWidth << 7 as libc::c_int) as SDWORD ||
             *pY < 0 as libc::c_int ||
             *pY > (mapHeight << 7 as libc::c_int) as SDWORD) {
        x = (*pX >> 7 as libc::c_int) as UDWORD; // change to tile coords.
        y = (*pY >> 7 as libc::c_int) as UDWORD;
        // go down the gateways, find the nearest gateway with >1 empty tiles
        nearestSoFar = 0xffffffff as libc::c_uint;
        psChosenGate = 0 as *mut GATEWAY;
        psGate = psGateways;
        while !psGate.is_null() {
            count = 0 as libc::c_int as UDWORD;
            noWater = 1 as libc::c_int;
            // does it have >1 tile unoccupied.
            if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
                // vert
                gX = (*psGate).x1 as UDWORD;
                gY = (*psGate).y1 as UDWORD;
                while gY <= (*psGate).y2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        count = count.wrapping_add(1)
                    }
                    if terrainTypes[((*mapTile(gX, gY)).texture as libc::c_int
                                         & 0x1ff as libc::c_int) as usize] as
                           libc::c_int == TER_WATER as libc::c_int {
                        noWater = 0 as libc::c_int
                    }
                    gY = gY.wrapping_add(1)
                }
            } else {
                // horiz
                gY = (*psGate).y1 as UDWORD;
                gX = (*psGate).x1 as UDWORD;
                while gX <= (*psGate).x2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        count = count.wrapping_add(1)
                    }
                    if terrainTypes[((*mapTile(gX, gY)).texture as libc::c_int
                                         & 0x1ff as libc::c_int) as usize] as
                           libc::c_int == TER_WATER as libc::c_int {
                        noWater = 0 as libc::c_int
                    }
                    gX = gX.wrapping_add(1)
                }
            }
            if count > 1 as libc::c_int as libc::c_uint && noWater != 0 {
                // ok it's free. Is it the nearest one yet?
                /* Get gateway midpoint */
                gX =
                    (((*psGate).x1 as libc::c_int +
                          (*psGate).x2 as libc::c_int) / 2 as libc::c_int) as
                        UDWORD;
                gY =
                    (((*psGate).y1 as libc::c_int +
                          (*psGate).y2 as libc::c_int) / 2 as libc::c_int) as
                        UDWORD;
                /* Estimate the distance to it */
                dist =
                    dirtySqrt(x as SDWORD, y as SDWORD, gX as SDWORD,
                              gY as SDWORD);
                /* Is it best we've found? */
                if dist < nearestSoFar &&
                       dist < 30 as libc::c_int as libc::c_uint {
                    /* Yes, then keep a record of it */
                    nearestSoFar = dist;
                    psChosenGate = psGate
                }
            }
            psGate = (*psGate).psNext
        }
        if !psChosenGate.is_null() {
            // find an unnocupied tile on that gateway.
            if (*psChosenGate).x1 as libc::c_int ==
                   (*psChosenGate).x2 as libc::c_int {
                // vert
                gX = (*psChosenGate).x1 as UDWORD;
                gY = (*psChosenGate).y1 as UDWORD;
                while gY <= (*psChosenGate).y2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        y = gY;
                        x = gX;
                        break ;
                    } else { gY = gY.wrapping_add(1) }
                }
            } else {
                // horiz
                gY = (*psChosenGate).y1 as UDWORD;
                gX = (*psChosenGate).x1 as UDWORD;
                while gX <= (*psChosenGate).x2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        y = gY;
                        x = gX;
                        break ;
                    } else { gX = gX.wrapping_add(1) }
                }
            }
            // back to world coords and store result.
            *pX =
                (x <<
                     7 as
                         libc::c_int).wrapping_add((128 as libc::c_int /
                                                        2 as libc::c_int) as
                                                       libc::c_uint) as
                    SDWORD; // return centre of tile.
            *pY =
                (y <<
                     7 as
                         libc::c_int).wrapping_add((128 as libc::c_int /
                                                        2 as libc::c_int) as
                                                       libc::c_uint) as
                    SDWORD;
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                // success
                return 0 as libc::c_int
            }
            // order the droid to build two walls, one either side of the gateway.
	// or one in the case of a 2 size gateway.
            x =
                (((*psChosenGate).x1 as libc::c_int +
                      (*psChosenGate).x2 as libc::c_int) / 2 as libc::c_int)
                    as UDWORD;
            y =
                (((*psChosenGate).y1 as libc::c_int +
                      (*psChosenGate).y2 as libc::c_int) / 2 as libc::c_int)
                    as UDWORD;
            x1 =
                ((((*psChosenGate).x1 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            y1 =
                ((((*psChosenGate).y1 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            if (*psChosenGate).x1 as libc::c_int ==
                   (*psChosenGate).x2 as libc::c_int {
                x2 = x1;
                y2 =
                    (y.wrapping_sub(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint);
                x3 = x1;
                y3 =
                    (y.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint)
            } else {
                x2 =
                    (x.wrapping_sub(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint);
                y2 = y1;
                x3 =
                    (x.wrapping_add(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint);
                y3 = y1
            }
            x4 =
                ((((*psChosenGate).x2 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            y4 =
                ((((*psChosenGate).y2 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            // first section.
            if x1 == x2 && y1 == y2 {
                orderDroidStatsLoc(psDroid, DORDER_BUILD, psWStats, x1, y1);
            } else {
                orderDroidStatsTwoLoc(psDroid, DORDER_LINEBUILD, psWStats, x1,
                                      y1, x2, y2);
            }
            // second section
            if x3 == x4 && y3 == y4 {
                orderDroidStatsLocAdd(psDroid, DORDER_BUILD, psWStats, x3,
                                      y3);
            } else {
                orderDroidStatsTwoLocAdd(psDroid, DORDER_LINEBUILD, psWStats,
                                         x3, y3, x4, y4);
            }
            return 1 as libc::c_int
        }
    }
    // we have a gateway.
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        // failed!
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// return a good place to build a defence with a min number of clear tiles
#[no_mangle]
pub unsafe extern "C" fn scrSkDefenseLocationB() -> BOOL {
    let mut pX: *mut SDWORD = 0 as *mut SDWORD;
    let mut pY: *mut SDWORD = 0 as *mut SDWORD;
    let mut statIndex: SDWORD = 0;
    let mut statIndex2: SDWORD = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut gX: UDWORD = 0;
    let mut gY: UDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut nearestSoFar: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut psGate: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psChosenGate: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut psWStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut x1: UDWORD = 0;
    let mut x2: UDWORD = 0;
    let mut x3: UDWORD = 0;
    let mut x4: UDWORD = 0;
    let mut y1: UDWORD = 0;
    let mut y2: UDWORD = 0;
    let mut y3: UDWORD = 0;
    let mut y4: UDWORD = 0;
    let mut noWater: BOOL = 0;
    if stackPopParams(6 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pX as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pY as *mut *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut statIndex as *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut statIndex2 as *mut SDWORD, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrSkDefenseLocationB: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSkDefenseLocationB:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  1990 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrSkDefenseLocationB\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(statIndex as isize) as *mut BASE_STATS;
    psWStats =
        asStructureStats.offset(statIndex2 as isize) as *mut BASE_STATS;
    // check for wacky coords.
    if !(*pX < 0 as libc::c_int ||
             *pX > (mapWidth << 7 as libc::c_int) as SDWORD ||
             *pY < 0 as libc::c_int ||
             *pY > (mapHeight << 7 as libc::c_int) as SDWORD) {
        x = (*pX >> 7 as libc::c_int) as UDWORD; // change to tile coords.
        y = (*pY >> 7 as libc::c_int) as UDWORD;
        // go down the gateways, find the nearest gateway with >1 empty tiles
        nearestSoFar = 0xffffffff as libc::c_uint;
        psChosenGate = 0 as *mut GATEWAY;
        let mut current_block_48: u64;
        psGate = psGateways;
        while !psGate.is_null() {
            count = 0 as libc::c_int as UDWORD;
            noWater = 1 as libc::c_int;
            // does it have >1 tile unoccupied.
            if (*psGate).x1 as libc::c_int == (*psGate).x2 as libc::c_int {
                // vert
                //skip gates that are too short
                if (*psGate).y2 as libc::c_int - (*psGate).y1 as libc::c_int
                       <= 2 as libc::c_int {
                    current_block_48 = 26972500619410423;
                } else {
                    gX = (*psGate).x1 as UDWORD;
                    gY = (*psGate).y1 as UDWORD;
                    while gY <= (*psGate).y2 as libc::c_uint {
                        if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                               (0x1 as libc::c_int | 0x2 as libc::c_int |
                                    0x20 as libc::c_int) == 0 {
                            count = count.wrapping_add(1)
                        }
                        if terrainTypes[((*mapTile(gX, gY)).texture as
                                             libc::c_int &
                                             0x1ff as libc::c_int) as usize]
                               as libc::c_int == TER_WATER as libc::c_int {
                            noWater = 0 as libc::c_int
                        }
                        gY = gY.wrapping_add(1)
                    }
                    current_block_48 = 11763295167351361500;
                }
            } else if (*psGate).x2 as libc::c_int -
                          (*psGate).x1 as libc::c_int <= 2 as libc::c_int {
                current_block_48 = 26972500619410423;
            } else {
                gY = (*psGate).y1 as UDWORD;
                gX = (*psGate).x1 as UDWORD;
                while gX <= (*psGate).x2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        count = count.wrapping_add(1)
                    }
                    if terrainTypes[((*mapTile(gX, gY)).texture as libc::c_int
                                         & 0x1ff as libc::c_int) as usize] as
                           libc::c_int == TER_WATER as libc::c_int {
                        noWater = 0 as libc::c_int
                    }
                    gX = gX.wrapping_add(1)
                }
                current_block_48 = 11763295167351361500;
            }
            match current_block_48 {
                11763295167351361500 => {
                    if count > 2 as libc::c_int as libc::c_uint &&
                           noWater != 0 {
                        // horiz
                        //skip gates that are too short
                        //<NEW> min 2 tiles
                        // ok it's free. Is it the nearest one yet?
			/* Get gateway midpoint */
                        gX =
                            (((*psGate).x1 as libc::c_int +
                                  (*psGate).x2 as libc::c_int) /
                                 2 as libc::c_int) as UDWORD;
                        gY =
                            (((*psGate).y1 as libc::c_int +
                                  (*psGate).y2 as libc::c_int) /
                                 2 as libc::c_int) as UDWORD;
                        dist =
                            dirtySqrt(x as SDWORD, y as SDWORD, gX as SDWORD,
                                      gY as SDWORD);
                        if dist < nearestSoFar &&
                               dist < 30 as libc::c_int as libc::c_uint {
                            /* Estimate the distance to it */
                            /* Yes, then keep a record of it */
                            nearestSoFar = dist;
                            psChosenGate = psGate
                        }
                    }
                }
                _ => { }
            }
            psGate = (*psGate).psNext
        }
        if !psChosenGate.is_null() {
            /* Is it best we've found? */
            // find an unnocupied tile on that gateway.
            if (*psChosenGate).x1 as libc::c_int ==
                   (*psChosenGate).x2 as libc::c_int {
                // vert
                gX = (*psChosenGate).x1 as UDWORD;
                gY = (*psChosenGate).y1 as UDWORD;
                while gY <= (*psChosenGate).y2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        y = gY;
                        x = gX;
                        break ;
                    } else { gY = gY.wrapping_add(1) }
                }
            } else {
                // horiz
                gY = (*psChosenGate).y1 as UDWORD;
                gX = (*psChosenGate).x1 as UDWORD;
                while gX <= (*psChosenGate).x2 as libc::c_uint {
                    if (*mapTile(gX, gY)).tileInfoBits as libc::c_int &
                           (0x1 as libc::c_int | 0x2 as libc::c_int |
                                0x20 as libc::c_int) == 0 {
                        y = gY;
                        x = gX;
                        break ;
                    } else { gX = gX.wrapping_add(1) }
                }
            }
            // back to world coords and store result.
            *pX =
                (x <<
                     7 as
                         libc::c_int).wrapping_add((128 as libc::c_int /
                                                        2 as libc::c_int) as
                                                       libc::c_uint) as
                    SDWORD; // return centre of tile.
            *pY =
                (y <<
                     7 as
                         libc::c_int).wrapping_add((128 as libc::c_int /
                                                        2 as libc::c_int) as
                                                       libc::c_uint) as
                    SDWORD;
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                // success
                return 0 as libc::c_int
            }
            // order the droid to build two walls, one either side of the gateway.
	// or one in the case of a 2 size gateway.
            //find center of the gateway
            x =
                (((*psChosenGate).x1 as libc::c_int +
                      (*psChosenGate).x2 as libc::c_int) / 2 as libc::c_int)
                    as UDWORD;
            y =
                (((*psChosenGate).y1 as libc::c_int +
                      (*psChosenGate).y2 as libc::c_int) / 2 as libc::c_int)
                    as UDWORD;
            //find start pos of the gateway
            x1 =
                ((((*psChosenGate).x1 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            y1 =
                ((((*psChosenGate).y1 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            if (*psChosenGate).x1 as libc::c_int ==
                   (*psChosenGate).x2 as libc::c_int {
                //vert
                x2 = x1; //vert: end x pos of the first section = start x pos
                y2 =
                    (y.wrapping_sub(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as
                                                           libc::c_uint); //start y loc of the first sec
                x3 = x1;
                y3 =
                    (y.wrapping_add(2 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint)
            } else {
                //hor
                x2 =
                    (x.wrapping_sub(1 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as
                                                           libc::c_uint); //<NEW>
                y2 = y1;
                x3 =
                    (x.wrapping_add(2 as libc::c_int as libc::c_uint) <<
                         7 as
                             libc::c_int).wrapping_add((128 as libc::c_int /
                                                            2 as libc::c_int)
                                                           as libc::c_uint);
                y3 = y1
            }
            //end coords of the second section
            x4 =
                ((((*psChosenGate).x2 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            y4 =
                ((((*psChosenGate).y2 as libc::c_int) << 7 as libc::c_int) +
                     128 as libc::c_int / 2 as libc::c_int) as UDWORD;
            //some temp checks
            if x2 < x1 {
                debug(LOG_ERROR,
                      b"scrSkDefenseLocationB: x2 < x1\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            if x3 > x4 {
                debug(LOG_ERROR,
                      b"scrSkDefenseLocationB: x2 < x1\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            if y2 < y1 {
                debug(LOG_ERROR,
                      b"scrSkDefenseLocationB: y2 < y1\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            if y3 > y4 {
                debug(LOG_ERROR,
                      b"scrSkDefenseLocationB: y3 > y4\x00" as *const u8 as
                          *const libc::c_char);
                return 0 as libc::c_int
            }
            // first section.
            if x1 == x2 && y1 == y2 {
                //first sec is 1 tile only: ((2 tile gate) or (3 tile gate and first sec))
                orderDroidStatsLoc(psDroid, DORDER_BUILD, psWStats, x1, y1);
            } else {
                orderDroidStatsTwoLoc(psDroid, DORDER_LINEBUILD, psWStats, x1,
                                      y1, x2, y2);
            }
            // second section
            if x3 == x4 && y3 == y4 {
                orderDroidStatsLocAdd(psDroid, DORDER_BUILD, psWStats, x3,
                                      y3);
            } else {
                orderDroidStatsTwoLocAdd(psDroid, DORDER_LINEBUILD, psWStats,
                                         x3, y3, x4, y4);
            }
            return 1 as libc::c_int
        }
    }
    // we have a gateway.
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        // failed!
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrSkFireLassat() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD,
                      ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if !psObj.is_null() { orderStructureObj(player as UDWORD, psObj); }
    return 1 as libc::c_int;
}
//-----------------------
// New functions
//-----------------------
#[no_mangle]
pub unsafe extern "C" fn scrActionDroidObj() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut action: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(3 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut action as *mut SDWORD,
                      ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        debug(LOG_ERROR,
              b"scrActionDroidObj: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitObj: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              2242 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"scrActionDroidObj\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrOrderUnitObj: Invalid object pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              2244 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"scrActionDroidObj\x00")).as_ptr(),
              b"PTRVALID(psObj, sizeof(BASE_OBJECT))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psDroid.is_null() || psObj.is_null() { return 0 as libc::c_int }
    if action != DACTION_DROIDREPAIR as libc::c_int {
        debug(LOG_ERROR,
              b"scrActionDroidObj: this action is not supported\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    actionDroidObj(psDroid, action as DROID_ACTION, psObj);
    return 1 as libc::c_int;
}
//<script function - improved version
// variables for the group iterator
#[no_mangle]
pub static mut psScrIterateGroupB: [*mut DROID_GROUP; 8] =
    [0 as *const DROID_GROUP as *mut DROID_GROUP; 8];
#[no_mangle]
pub static mut psScrIterateGroupDroidB: [*mut DROID; 8] =
    [0 as *const DROID as *mut DROID; 8];
// initialise iterating a groups members
#[no_mangle]
pub unsafe extern "C" fn scrInitIterateGroupB() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut bucket: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut bucket as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrInitIterateGroupB: stackPopParams failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrInitIterateGroupB: invalid group pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              2280 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"scrInitIterateGroupB\x00")).as_ptr(),
              b"PTRVALID(psGroup, sizeof(DROID_GROUP))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bucket < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"scrInitIterateGroupB: invalid bucket\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bucket < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              2283 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"scrInitIterateGroupB\x00")).as_ptr(),
              b"bucket < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    psScrIterateGroupB[bucket as usize] = psGroup;
    psScrIterateGroupDroidB[bucket as usize] = (*psGroup).psList;
    return 1 as libc::c_int;
}
//script function - improved version
// iterate through a groups members
#[no_mangle]
pub unsafe extern "C" fn scrIterateGroupB() -> BOOL {
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bucket: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut bucket as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrIterateGroupB: stackPopParams failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if bucket < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"scrIterateGroupB: invalid bucket\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bucket < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptai.c\x00" as *const u8 as *const libc::c_char,
              2306 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"scrIterateGroupB\x00")).as_ptr(),
              b"bucket < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if psGroup != psScrIterateGroupB[bucket as usize] {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrIterateGroupB: invalid group, InitGroupIterateB not called?\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptai.c\x00" as *const u8 as *const libc::c_char,
                  2310 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrIterateGroupB\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if !psScrIterateGroupDroidB[bucket as usize].is_null() {
        psDroid = psScrIterateGroupDroidB[bucket as usize];
        psScrIterateGroupDroidB[bucket as usize] =
            (*psScrIterateGroupDroidB[bucket as usize]).psGrpNext
    } else { psDroid = 0 as *mut DROID }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psDroid as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrIterateGroupB: stackPushResult failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// storage definition
/*
typedef struct _skirmishstore {
	SDWORD					index;
	DROID_TEMPLATE			*psTempl;
	struct _skirmishstore	*psNext;
}SKIRMISHSTORE;

#define numSkPiles	8
SKIRMISHSTORE	*skirmishStore[numSkPiles];

// init templates
BOOL skInitSkirmishTemplates(void)
{
	UBYTE i;
	for(i=0;i<numSkPiles;i++)
	{
		skirmishStore[i] = NULL;
	}
	if (!stackPushResult(VAL_BOOL, TRUE))		// success!
	{
		return FALSE;
	}
	return TRUE;
}

// add template
BOOL skAddTemplate(void)
{
	SKIRMISHSTORE *psT;

	SDWORD			stempl,index,pile;
	DROID_TEMPLATE	*psTempl;

	if (!stackPopParams(3,VAL_INT, &pile,VAL_INT, &index, ST_TEMPLATE, &stempl))
	{
		return FALSE;
	}
	psTempl =(DROID_TEMPLATE *)stempl;
//	psT = MALLOC(sizeof(SKIRMISHSTORE));
	HEAP_ALLOC(psTemplateHeap,&psT);
	if ( !psT)
	{
		goto fail;
	}
	psT->index			= index;
	psT->psTempl		= psTempl;
	psT->psNext			= skirmishStore[pile];
	skirmishStore[pile] = psT;

	if (!stackPushResult(VAL_BOOL, TRUE))		// yes
	{
		return FALSE;
	}
	return TRUE;
fail:
	if (!stackPushResult(VAL_BOOL, FALSE))		// no
	{
		return FALSE;
	}
	return TRUE;
}


// get template
BOOL skGetTemplate(void)
{
	SDWORD	index,pile;
	SKIRMISHSTORE *psT;

	if (!stackPopParams(2,VAL_INT, &pile,VAL_INT, &index))
	{
		return FALSE;
	}

	for( psT = skirmishStore[pile]; psT && (psT->index != index); psT=psT->psNext);
	if(!psT)
	{
		DBERROR(("failed to find required skTemplate"));
		return FALSE;
	}

	if (!stackPushResult(ST_TEMPLATE, (UDWORD)(psT->psTempl) ))
	{
		return FALSE;
	}
	return TRUE;
}

// clear templates
BOOL skClearSkirmishTemplates(void)
{
	UBYTE i;
	for(i=0;i<numSkPiles;i++)
	{
		while(skirmishStore[i])
		{
			HEAP_FREE(psTemplateHeap,skirmishStore[i]);
		}
	}

	return TRUE;
}
*/
// ********************************************************************************************
// ********************************************************************************************
// skirmish template storage facility
// ********************************************************************************************
// Give a Droid a build order
/*BOOL scrSkOrderDroidLineBuild(void)
{
	DROID			*psDroid;
	SDWORD			x1,y1,x2,y2, statIndex;
	BASE_STATS		*psStats;

	if (!stackPopParams(6, ST_DROID, &psDroid, ST_STRUCTURESTAT, &statIndex,
						   VAL_INT, &x1, VAL_INT, &y1,VAL_INT, &x2, VAL_INT, &y2))
	{
		return FALSE;
	}

	psStats = (BASE_STATS *)(asStructureStats + statIndex);

	ASSERT( PTRVALID(psDroid, sizeof(DROID)),
		"scrOrderDroidLineBuild: Invalid Unit pointer" );
	ASSERT( PTRVALID(psStats, sizeof(BASE_STATS)),
		"scrOrderDroidLineBuild: Invalid object pointer" );
	if (psDroid == NULL)
	{
		return FALSE;
	}

	if ((x1 < 0) || (x1 > (SDWORD)mapWidth*TILE_UNITS) ||
		(y1 < 0) || (y1 > (SDWORD)mapHeight*TILE_UNITS)||
		(x2 < 0) || (x2 > (SDWORD)mapWidth*TILE_UNITS) ||
		(y2 < 0) || (y2 > (SDWORD)mapHeight*TILE_UNITS) )
	{
		ASSERT( FALSE,
			"scrOrderDroidLineBuild: Invalid location" );
		return FALSE;
	}


	if(IsPlayerStructureLimitReached(psDroid->player) == FALSE)
	{
		orderDroidStatsTwoLoc(psDroid, DORDER_LINEBUILD, psStats, (UDWORD)x1,(UDWORD)y1,(UDWORD)x2,(UDWORD)y2);
	}

	return TRUE;
}
*/
// ********************************************************************************************
