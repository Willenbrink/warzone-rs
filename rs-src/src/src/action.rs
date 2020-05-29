use ::libc;
extern "C" {
    pub type _gateway;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn labs(_: libc::c_long) -> libc::c_long;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Supposedly fast lookup sqrt - unfortunately it's probably slower than the FPU sqrt :-( */
    #[no_mangle]
    fn trigIntSqrt(val: UDWORD) -> FRACT;
    #[no_mangle]
    fn audio_QueueTrackMinDelay(iTrack: SDWORD, iMinDelay: UDWORD);
    /* Set up a droid to build a structure - returns true if successful */
    #[no_mangle]
    fn droidStartBuild(psDroid: *mut DROID) -> BOOL;
    /* Set up a droid to build a foundation - returns true if successful */
    #[no_mangle]
    fn droidStartFoundation(psDroid: *mut DROID) -> BOOL;
    /* Sets a droid to start demolishing - returns true if successful */
    #[no_mangle]
    fn droidStartDemolishing(psDroid: *mut DROID) -> BOOL;
    /* Update a construction droid while it is demolishing 
   returns TRUE while demolishing */
    #[no_mangle]
    fn droidUpdateDemolishing(psDroid: *mut DROID) -> BOOL;
    /* Sets a droid to start repairing - returns true if successful */
    #[no_mangle]
    fn droidStartRepair(psDroid: *mut DROID) -> BOOL;
    /* Update a construction droid while it is repairing 
   returns TRUE while repairing */
    #[no_mangle]
    fn droidUpdateRepair(psDroid: *mut DROID) -> BOOL;
    /*Start a Repair Droid working on a damaged droid - returns TRUE if successful*/
    #[no_mangle]
    fn droidStartDroidRepair(psDroid: *mut DROID) -> BOOL;
    /*Updates a Repair Droid working on a damaged droid - returns TRUE whilst repairing*/
    #[no_mangle]
    fn droidUpdateDroidRepair(psRepairDroid: *mut DROID) -> BOOL;
    /*checks a droids current body points to see if need to self repair*/
    #[no_mangle]
    fn droidSelfRepair(psDroid: *mut DROID);
    /* Update a construction droid while it is building 
   returns TRUE while building continues */
    #[no_mangle]
    fn droidUpdateBuild(psDroid: *mut DROID) -> BOOL;
    /* Update a construction droid while it is building a
   foundation. Returns TRUE whilst foundation continues */
    #[no_mangle]
    fn droidUpdateFoundation(psDroid: *mut DROID) -> BOOL;
    /*Start a EW weapon droid working on a low resistance structure*/
    #[no_mangle]
    fn droidStartRestore(psDroid: *mut DROID) -> BOOL;
    /*continue restoring a structure*/
    #[no_mangle]
    fn droidUpdateRestore(psDroid: *mut DROID) -> BOOL;
    /* Remove a droid and free it's memory */
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    /* Burn a barbarian then destroy it */
    #[no_mangle]
    fn droidBurn(psDroid: *mut DROID);
    /* Remove a droid from the apsDroidLists so doesn't update or get drawn etc*/
//returns TRUE if successfully removed from the list
    #[no_mangle]
    fn droidRemove(psDroid: *mut DROID, pList: *mut *mut DROID) -> BOOL;
    /*Deals with building a module - checking if any droid is currently doing this
 - if so, helping to build the current one*/
    #[no_mangle]
    fn setUpBuildModule(psDroid: *mut DROID);
    /*checks to see if an electronic warfare weapon is attached to the droid*/
    #[no_mangle]
    fn electronicDroid(psDroid: *mut DROID) -> BOOL;
    /* Set up a droid to clear a wrecked building feature - returns true if successful */
    #[no_mangle]
    fn droidStartClearing(psDroid: *mut DROID) -> BOOL;
    /* Update a construction droid while it is clearing 
   returns TRUE while continues */
    #[no_mangle]
    fn droidUpdateClearing(psDroid: *mut DROID) -> BOOL;
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    /*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
    #[no_mangle]
    fn vtolEmpty(psDroid: *mut DROID) -> BOOL;
    /*Checks a vtol for being fully armed and fully repaired to see if ready to 
leave reArm pad */
    #[no_mangle]
    fn vtolHappy(psDroid: *mut DROID) -> BOOL;
    // true if a vtol droid currently returning to be rearmed
    #[no_mangle]
    fn vtolRearming(psDroid: *mut DROID) -> BOOL;
    // return whether a droid has a CB sensor on it
    #[no_mangle]
    fn cbSensorDroid(psDroid: *mut DROID) -> BOOL;
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    /* checks that the location is a valid one to build on and sets the outline colour
x and y in tile-coords*/
    #[no_mangle]
    fn validLocation(psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD,
                     player: UDWORD, bCheckBuildQueue: BOOL) -> BOOL;
    /* EW works differently in multiplayer mode compared with single player.*/
    #[no_mangle]
    fn validStructResistance(psStruct: *mut STRUCTURE) -> BOOL;
    // return the nearest rearm pad
// if bClear is true it tries to find the nearest clear rearm pad in
// the same cluster as psTarget
// psTarget can be NULL
    #[no_mangle]
    fn findNearestReArmPad(psDroid: *mut DROID, psTarget: *mut STRUCTURE,
                           bClear: BOOL) -> *mut STRUCTURE;
    // clear a rearm pad for a vtol to land on it
    #[no_mangle]
    fn ensureRearmPadClear(psStruct: *mut STRUCTURE, psDroid: *mut DROID);
    // La!
    #[no_mangle]
    fn IsStatExpansionModule(psStats: *mut STRUCTURE_STATS) -> BOOL;
    /*looks around the given droid to see if there is any building 
wreckage to clear*/
    #[no_mangle]
    fn checkForWreckage(psDroid: *mut DROID) -> *mut FEATURE;
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    // Find the nearest target to a droid
    #[no_mangle]
    fn aiNearestTarget(psDroid: *mut DROID, ppsObj: *mut *mut BASE_OBJECT)
     -> BOOL;
    /*set of rules which determine whether the weapon associated with the object
can fire on the propulsion type of the target*/
    #[no_mangle]
    fn validTarget(psObject: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
    #[no_mangle]
    fn moveDroidTo(psDroid: *mut DROID, x: UDWORD, y: UDWORD) -> BOOL;
    /* Set a target location for a droid to move to  - returns a BOOL based on if there is a path to the destination (TRUE if there is a path)*/
// the droid will not join a formation when it gets to the location
    #[no_mangle]
    fn moveDroidToNoFormation(psDroid: *mut DROID, x: UDWORD, y: UDWORD)
     -> BOOL;
    // move a droid directly to a location (used by vtols only)
    #[no_mangle]
    fn moveDroidToDirect(psDroid: *mut DROID, x: UDWORD, y: UDWORD);
    // Get a droid to turn towards a locaton
    #[no_mangle]
    fn moveTurnDroid(psDroid: *mut DROID, x: UDWORD, y: UDWORD);
    /* Stop a droid */
    #[no_mangle]
    fn moveStopDroid(psDroid: *mut DROID);
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    fn weaponShortHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn weaponLongHit(psStats: *mut WEAPON_STATS, player: UBYTE) -> UDWORD;
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
    // returns a pointer to the current loaded map data
    #[no_mangle]
    fn GetHeightOfMap() -> UDWORD;
    #[no_mangle]
    fn GetWidthOfMap() -> UDWORD;
    /* Fire a weapon at something */
    #[no_mangle]
    fn combFire(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                psTarget: *mut BASE_OBJECT);
    #[no_mangle]
    fn getTileStructure(x: UDWORD, y: UDWORD) -> *mut STRUCTURE;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    /* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
    #[no_mangle]
    fn visibleObject(psViewer: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    // Do visibility check, but with walls completely blocking LOS.
    #[no_mangle]
    fn visibleObjWallBlock(psViewer: *mut BASE_OBJECT,
                           psTarget: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // return the maximum range for a weapon
    #[no_mangle]
    fn proj_GetLongRange(psStats: *mut WEAPON_STATS, dz: SDWORD) -> SDWORD;
    /* Give a droid an order */
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* Check the order state of a droid */
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    /* Give a droid an order with a location target */
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn secondaryGetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         pState: *mut SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    fn intBuildStarted(psDroid: *mut DROID);
    /*checks how long the transporter has been travelling to see if it should
have arrived - returns TRUE when there*/
    #[no_mangle]
    fn updateTransporter(psTransporter: *mut DROID) -> BOOL;
    /*check to see if any research has been completed that enables self repair*/
    #[no_mangle]
    fn selfRepairEnabled(player: UBYTE) -> BOOL;
    #[no_mangle]
    static mut mission: MISSION;
    //returns the x coord for where the Transporter can land
    #[no_mangle]
    fn getLandingX(iPlayer: SDWORD) -> UWORD;
    //returns the y coord for where the Transporter can land
    #[no_mangle]
    fn getLandingY(iPlayer: SDWORD) -> UWORD;
    #[no_mangle]
    fn missionGetReinforcementTime() -> UDWORD;
    #[no_mangle]
    fn missionGetTransporterExit(iPlayer: SDWORD, iX: *mut UWORD,
                                 iY: *mut UWORD);
    #[no_mangle]
    fn getDroidsToSafetyFlag() -> BOOL;
    //checks to see if the player has any droids (except Transporters left)
    #[no_mangle]
    fn missionDroidsRemaining(player: UDWORD) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn sendVtolRearm(psDroid: *mut DROID, psStruct: *mut STRUCTURE,
                     chosen: UBYTE) -> BOOL;
    #[no_mangle]
    fn formationLeave(psFormation: *mut FORMATION, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn StatIsStructure(Stat: *mut BASE_STATS) -> BOOL;
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
}
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
pub type FRACT = libc::c_float;
pub type FRACT_D = libc::c_float;
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
// Size of the entry for vertex id in the imd polygon structure (32bits on pc 16bits on PSX)
// warning.... this is not used on the playstation version !
// the polygon number for the next in the BSP list ... or BSPPOLYID_TERMINATE for no more
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
// information about a formation member
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
// information about a formation
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
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
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
// Feature armour
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
pub const TER_CLIFFFACE: _terrain_type = 8;
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
// data required for any action
pub type DROID_ACTION_DATA = _droid_action_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _droid_action_data {
    pub action: DROID_ACTION,
    pub x: UDWORD,
    pub y: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub psStats: *mut BASE_STATS,
}
pub const DSS_HALT_HOLD: _secondary_state = 64;
// the state of secondary orders
pub type SECONDARY_STATE = _secondary_state;
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
pub const DSS_ALEV_NEVER: _secondary_state = 48;
pub const DSS_ALEV_ATTACKED: _secondary_state = 32;
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const DSS_ARANGE_LONG: _secondary_state = 2;
pub const DSS_ARANGE_SHORT: _secondary_state = 1;
// secondary orders for droids
pub type SECONDARY_ORDER = _secondary_order;
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
pub type FORMATION = _formation;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_0 = 222;
pub type TRIGGER_TYPE = _trigger_type;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
/* Array debug info for a script */
/* Line debugging information for a script */
// Offset in the compiled script that corresponds to
// this line in the original script.
// the trigger/event that starts at this line
/* Different types of triggers */
pub type _trigger_type = libc::c_uint;
// The user defined callback triggers should start with this id
// Event has paused for an interval and will restart in the middle of it's code
pub const TR_CALLBACKSTART: _trigger_type = 5;
// Trigger at repeated intervals
pub const TR_PAUSE: _trigger_type = 4;
// Trigger after a time pause
pub const TR_EVERY: _trigger_type = 3;
// Trigger uses script code
pub const TR_WAIT: _trigger_type = 2;
// Trigger fires when the script is first run
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
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
pub type MISSION = _mission;
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
pub type PLAYER_POWER = _player_power;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
//MISSION_TYPE		type;							//defines which start and end functions to use
//defines which start and end functions to use - see levels_type in levels.h
//the original mapTiles
//the original mapLinePoints
//the original mapWidth
//the original mapHeight
//the gateway list
//the RLE map zones
//the number of map zones
//zone equivalence data
//scroll coords for original map
//original object lists
//struct _proximity_display	*apsProxDisp[MAX_PLAYERS];
//time the mission started
//how long the mission can last
// < 0 = no limit
//time taken for reinforcements to arrive
// < 0 = none allowed
//time the cheating started (mission time-wise!)
//LANDING_ZONE		homeLZ;
//selectedPlayer's LZ x and y
//original view position
/* transporter entry/exit tiles */
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
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
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
pub const CALL_CONSOLE: _scr_callback_types = 58;
pub const CALL_ALLIANCEOFFER: _scr_callback_types = 57;
pub const CALL_PLAYERLEFT: _scr_callback_types = 56;
pub const CALL_UNITTAKEOVER: _scr_callback_types = 55;
pub const CALL_VTOL_OFF_MAP: _scr_callback_types = 54;
pub const CALL_CLUSTER_EMPTY: _scr_callback_types = 53;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub const CALL_OBJECTCLOSE: _scr_callback_types = 48;
pub const CALL_OBJECTOPEN: _scr_callback_types = 47;
pub const CALL_FEATURE_DESTROYED: _scr_callback_types = 46;
pub const CALL_DROID_DESTROYED: _scr_callback_types = 45;
pub const CALL_STRUCT_DESTROYED: _scr_callback_types = 44;
pub const CALL_OBJ_DESTROYED: _scr_callback_types = 43;
pub const CALL_OBJ_SEEN: _scr_callback_types = 42;
pub const CALL_FEATURE_SEEN: _scr_callback_types = 41;
pub const CALL_DROID_SEEN: _scr_callback_types = 40;
pub const CALL_STRUCT_SEEN: _scr_callback_types = 39;
pub const CALL_ATTACKED: _scr_callback_types = 38;
pub const CALL_DROID_ATTACKED: _scr_callback_types = 37;
pub const CALL_STRUCT_ATTACKED: _scr_callback_types = 36;
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_RESEARCHCOMPLETED: _scr_callback_types = 34;
pub const CALL_DESIGN_PROPULSION: _scr_callback_types = 33;
pub const CALL_DESIGN_BODY: _scr_callback_types = 32;
pub const CALL_DESIGN_COMMAND: _scr_callback_types = 31;
pub const CALL_DESIGN_SYSTEM: _scr_callback_types = 30;
pub const CALL_DESIGN_WEAPON: _scr_callback_types = 29;
pub const CALL_DESIGN_QUIT: _scr_callback_types = 28;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const CALL_BUTTON_PRESSED: _scr_callback_types = 26;
pub const CALL_MANULIST: _scr_callback_types = 25;
pub const CALL_MANURUN: _scr_callback_types = 24;
pub const CALL_RESEARCHLIST: _scr_callback_types = 23;
pub const CALL_BUILDGRID: _scr_callback_types = 22;
pub const CALL_BUILDLIST: _scr_callback_types = 21;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
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
#[inline]
unsafe extern "C" fn tileOnMap(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    return (x >= 0 as libc::c_int && x < mapWidth as SDWORD &&
                y >= 0 as libc::c_int && y < mapHeight as SDWORD) as
               libc::c_int;
}
/* Check if a target is at correct range to attack */
/* Check if a target is at correct range to attack */
#[no_mangle]
pub unsafe extern "C" fn actionInAttackRange(mut psDroid: *mut DROID,
                                             mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut radSq: SDWORD = 0;
    let mut rangeSq: SDWORD = 0;
    let mut longRange: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    //if (psDroid->numWeaps == 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    psStats =
        asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                    usize].nStat as isize);
    dx = (*psDroid).x as SDWORD - (*psObj).x as SDWORD;
    dy = (*psDroid).y as SDWORD - (*psObj).y as SDWORD;
    dz = (*psDroid).z as SDWORD - (*psObj).z as SDWORD;
    radSq = dx * dx + dy * dy;
    if (*psDroid).order == DORDER_ATTACKTARGET as libc::c_int &&
           secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) != 0 &&
           state as libc::c_uint ==
               DSS_HALT_HOLD as libc::c_int as libc::c_uint {
        longRange = proj_GetLongRange(psStats, dz);
        rangeSq = longRange * longRange
    } else {
        match (*psDroid).secondaryOrder & 0x3 as libc::c_int as libc::c_uint {
            3 => {
                //if (psStats->shortHit > psStats->longHit)
                if weaponShortHit(psStats, (*psDroid).player) >
                       weaponLongHit(psStats, (*psDroid).player) {
                    rangeSq =
                        (*psStats).shortRange.wrapping_mul((*psStats).shortRange)
                            as SDWORD
                } else {
                    longRange = proj_GetLongRange(psStats, dz);
                    rangeSq = longRange * longRange
                }
            }
            1 => {
                rangeSq =
                    (*psStats).shortRange.wrapping_mul((*psStats).shortRange)
                        as SDWORD
            }
            2 => {
                longRange = proj_GetLongRange(psStats, dz);
                rangeSq = longRange * longRange
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"actionInAttackRange: unknown attack range order\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"action.c\x00" as *const u8 as *const libc::c_char,
                          130 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"actionInAttackRange\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                longRange = proj_GetLongRange(psStats, dz);
                rangeSq = longRange * longRange
            }
        }
    }
    /* check max range */
    if radSq <= rangeSq {
        /* check min range */
        rangeSq =
            (*psStats).minRange.wrapping_mul((*psStats).minRange) as SDWORD;
        if radSq >= rangeSq || proj_Direct(psStats) == 0 {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
    }
    return 0 as libc::c_int;
}
// check if a target is within weapon range
// check if a target is within weapon range
#[no_mangle]
pub unsafe extern "C" fn actionInRange(mut psDroid: *mut DROID,
                                       mut psObj: *mut BASE_OBJECT) -> BOOL {
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut radSq: SDWORD = 0;
    let mut rangeSq: SDWORD = 0;
    let mut longRange: SDWORD = 0;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    //if (psDroid->numWeaps == 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    psStats =
        asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                    usize].nStat as isize);
    dx = (*psDroid).x as SDWORD - (*psObj).x as SDWORD;
    dy = (*psDroid).y as SDWORD - (*psObj).y as SDWORD;
    dz = (*psDroid).z as SDWORD - (*psObj).z as SDWORD;
    radSq = dx * dx + dy * dy;
    longRange = proj_GetLongRange(psStats, dz);
    rangeSq = longRange * longRange;
    /* check max range */
    if radSq <= rangeSq {
        /* check min range */
        rangeSq =
            (*psStats).minRange.wrapping_mul((*psStats).minRange) as SDWORD;
        if radSq >= rangeSq || proj_Direct(psStats) == 0 {
            return 1 as libc::c_int
        } else { return 0 as libc::c_int }
    }
    return 0 as libc::c_int;
}
// check if a target is inside minimum weapon range
// check if a target is inside minimum weapon range
#[no_mangle]
pub unsafe extern "C" fn actionInsideMinRange(mut psDroid: *mut DROID,
                                              mut psObj: *mut BASE_OBJECT)
 -> BOOL {
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut radSq: SDWORD = 0;
    let mut rangeSq: SDWORD = 0;
    let mut minRange: SDWORD = 0;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    //if (psDroid->numWeaps == 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    psStats =
        asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                    usize].nStat as isize);
    dx = (*psDroid).x as SDWORD - (*psObj).x as SDWORD;
    dy = (*psDroid).y as SDWORD - (*psObj).y as SDWORD;
    dz = (*psDroid).z as SDWORD - (*psObj).z as SDWORD;
    radSq = dx * dx + dy * dy;
    minRange = (*psStats).minRange as SDWORD;
    rangeSq = minRange * minRange;
    // check min range
    if radSq <= rangeSq { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
// Realign turret
/* Find a new location by a structure when building */
/*static BOOL actionNewBuildPos(DROID *psDroid, UDWORD *pX, UDWORD *pY)
{
	//STRUCTURE_STATS		*psStructStats;
	BASE_STATS				*psStats;

	// get the structure stats
	switch (psDroid->action)
	{
	case DACTION_MOVETOBUILD:
	case DACTION_FOUNDATION_WANDER:
		if ( psDroid->order == DORDER_BUILD || psDroid->order == DORDER_LINEBUILD )
		{
			//psStructStats = (STRUCTURE_STATS *)psDroid->psTarStats;
			psStats = psDroid->psTarStats;
		}
		else
		{
			ASSERT( psDroid->order == DORDER_HELPBUILD,
				"actionNewBuildPos: invalid order" );
			//psStructStats = ((STRUCTURE *)psDroid->psTarget)->pStructureType;
			psStats = (BASE_STATS *)((STRUCTURE *)psDroid->psTarget)->pStructureType;
		}
		break;
	case DACTION_BUILD:
	case DACTION_BUILDWANDER:
	case DACTION_MOVETODEMOLISH:
	//case DACTION_MOVETOREPAIR:
	case DACTION_MOVETORESTORE:
		//psStructStats = ((STRUCTURE *)psDroid->psTarget)->pStructureType;
		psStats = (BASE_STATS *)((STRUCTURE *)psDroid->psTarget)->pStructureType;
		break;
	case DACTION_MOVETOREPAIR:
	case DACTION_MOVETOREARM:
		psStats = (BASE_STATS *)((STRUCTURE *)psDroid->psActionTarget)->pStructureType;
		break;
	case DACTION_MOVETOCLEAR:
		psStats = (BASE_STATS *)((FEATURE *)psDroid->psTarget)->psStats;
		break;
	default:
		ASSERT( FALSE,
			"actionNewBuildPos: invalid action" );
		return FALSE;
		break;
	}
	//ASSERT( PTRVALID(psStructStats, sizeof(STRUCTURE_STATS)),
	//	"actionNewBuildPos: invalid structure stats pointer" );

	// find a new destination
	//if (getDroidDestination(psStructStats, psDroid->orderX, psDroid->orderY,pX,pY))
	if (psDroid->action == DACTION_MOVETOREARM OR psDroid->action == DACTION_MOVETOREPAIR)
	{
		//use action target
		if (getDroidDestination(psStats, psDroid->actionX, psDroid->actionY, pX, pY))
		{
			return TRUE;
		}
	}
	else
	{
		//use order target
		if (getDroidDestination(psStats, psDroid->orderX, psDroid->orderY, pX, pY))
		{
			return TRUE;
		}
	}

	return FALSE;
}*/
// Realign turret
#[no_mangle]
pub unsafe extern "C" fn actionAlignTurret(mut psObj: *mut BASE_OBJECT) {
    let mut rotation: UDWORD = 0;
    let mut tRot: UWORD = 0;
    let mut tPitch: UWORD = 0;
    let mut nearest: UWORD = 0 as libc::c_int as UWORD;
    //get the maximum rotation this frame
    //rotation = (psDroid->turretRotRate * frameTime) / (4 * GAME_TICKS_PER_SEC);
    rotation =
        (180 as libc::c_int as
             libc::c_uint).wrapping_mul(frameTime).wrapping_div((4 as
                                                                     libc::c_int
                                                                     *
                                                                     1000 as
                                                                         libc::c_int)
                                                                    as
                                                                    libc::c_uint);
    if rotation == 0 as libc::c_int as libc::c_uint {
        rotation = 1 as libc::c_int as UDWORD
    }
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            tRot = (*(psObj as *mut DROID)).turretRotation;
            tPitch = (*(psObj as *mut DROID)).turretPitch
        }
        1 => {
            tRot = (*(psObj as *mut STRUCTURE)).turretRotation;
            tPitch = (*(psObj as *mut STRUCTURE)).turretPitch;
            // now find the nearest 90 degree angle
            nearest =
                ((tRot as libc::c_int + 45 as libc::c_int) / 90 as libc::c_int
                     * 90 as libc::c_int) as UWORD;
            tRot =
                ((tRot as libc::c_int + 360 as libc::c_int -
                      nearest as libc::c_int) % 360 as libc::c_int) as UWORD
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"actionAlignTurret: invalid object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      333 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"actionAlignTurret\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return
        }
    }
    //	DBP1(("rotrate=%d framTime=%d rotation=%d\n",psDroid->turretRotRate,frameTime,rotation));
    //DBP1(("droid=%x turret=%x\n",psDroid,psDroid->turretRotation));
    if rotation > 180 as libc::c_int as libc::c_uint {
        //crop to 180 degrees, no point in turning more than all the way round
        rotation = 180 as libc::c_int as UDWORD
    }
    if (tRot as libc::c_int) < 180 as libc::c_int {
        // +ve angle 0 - 179 degrees
        if tRot as libc::c_uint > rotation {
            tRot = (tRot as libc::c_uint).wrapping_sub(rotation) as UWORD
        } else { tRot = 0 as libc::c_int as UWORD }
    } else if (tRot as libc::c_uint) <
                  (360 as libc::c_int as libc::c_uint).wrapping_sub(rotation)
     {
        tRot = (tRot as libc::c_uint).wrapping_add(rotation) as UWORD
    } else { tRot = 0 as libc::c_int as UWORD }
    tRot = (tRot as libc::c_int % 360 as libc::c_int) as UWORD;
    //angle greater than 180 rotate in opposite direction
    // align the turret pitch
    if (tPitch as libc::c_int) < 180 as libc::c_int {
        // +ve angle 0 - 179 degrees
        if tPitch as libc::c_uint >
               rotation.wrapping_div(2 as libc::c_int as libc::c_uint) {
            tPitch =
                (tPitch as
                     libc::c_uint).wrapping_sub(rotation.wrapping_div(2 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
                    as UWORD
        } else { tPitch = 0 as libc::c_int as UWORD }
    } else if (tPitch as libc::c_int) <
                  360 as libc::c_int - rotation as SDWORD / 2 as libc::c_int {
        tPitch =
            (tPitch as
                 libc::c_uint).wrapping_add(rotation.wrapping_div(2 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UWORD
    } else { tPitch = 0 as libc::c_int as UWORD }
    tPitch = (tPitch as libc::c_int % 360 as libc::c_int) as UWORD;
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            (*(psObj as *mut DROID)).turretRotation = tRot;
            (*(psObj as *mut DROID)).turretPitch = tPitch
        }
        1 => {
            // -ve angle rotate in opposite direction
            // now adjust back to the nearest 90 degree angle
            tRot =
                ((tRot as libc::c_int + nearest as libc::c_int) %
                     360 as libc::c_int) as UWORD;
            (*(psObj as *mut STRUCTURE)).turretRotation = tRot;
            (*(psObj as *mut STRUCTURE)).turretPitch = tPitch
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"actionAlignTurret: invalid object type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      412 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"actionAlignTurret\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return
        }
    };
}
/* Rotate turret toward  target return True if locked on (Droid and Structure) */
/*extern BOOL actionTargetTurret(BASE_OBJECT *psAttacker, BASE_OBJECT *psTarget,
								UWORD *pRotation, UWORD *pPitch, SWORD rotRate,
								SWORD pitchRate, BOOL bDirectFire, BOOL bInvert);*/
//								UDWORD *pRotation, UDWORD *pPitch, SDWORD rotRate,
//								SDWORD pitchRate, BOOL bDirectFire, BOOL bInvert);
/* returns true if on target */
//BOOL actionTargetTurret(BASE_OBJECT *psAttacker, BASE_OBJECT *psTarget, UDWORD *pRotation,
//		UDWORD *pPitch, SDWORD rotRate, SDWORD pitchRate, BOOL bDirectFire, BOOL bInvert)
//BOOL actionTargetTurret(BASE_OBJECT *psAttacker, BASE_OBJECT *psTarget, UWORD *pRotation,
//		UWORD *pPitch, SWORD rotRate, SWORD pitchRate, BOOL bDirectFire, BOOL bInvert)
#[no_mangle]
pub unsafe extern "C" fn actionTargetTurret(mut psAttacker: *mut BASE_OBJECT,
                                            mut psTarget: *mut BASE_OBJECT,
                                            mut pRotation: *mut UWORD,
                                            mut pPitch: *mut UWORD,
                                            mut psWeapStats:
                                                *mut WEAPON_STATS,
                                            mut bInvert: BOOL) -> BOOL {
    let mut tRotation: SWORD = 0;
    let mut tPitch: SWORD = 0;
    let mut rotRate: SWORD = 0;
    let mut pitchRate: SWORD = 0;
    let mut targetRotation: SDWORD = 0;
    let mut targetPitch: SDWORD = 0;
    let mut pitchError: SDWORD = 0;
    let mut rotationError: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut onTarget: BOOL = 0 as libc::c_int;
    let mut fR: FRACT = 0.;
    let mut pitchLowerLimit: SDWORD = 0;
    let mut pitchUpperLimit: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //	iVector	muzzle;
    //these are constants now and can be set up at the start of the function
    rotRate = 180 as libc::c_int as SWORD;
    pitchRate = (180 as libc::c_int / 2 as libc::c_int) as SWORD;
    //added for 22/07/99 upgrade - AB
    if !psWeapStats.is_null() {
        //extra heavy weapons on some structures need to rotate and pitch more slowly
        if (*psWeapStats).weight > 50000 as libc::c_int as libc::c_uint {
            rotRate =
                ((180 as libc::c_int / 2 as libc::c_int) as
                     libc::c_uint).wrapping_sub((100 as libc::c_int as
                                                     libc::c_uint).wrapping_mul((*psWeapStats).weight.wrapping_sub(50000
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)).wrapping_div((*psWeapStats).weight))
                    as SWORD;
            pitchRate = (rotRate as libc::c_int / 2 as libc::c_int) as SWORD
        }
    }
    tRotation = *pRotation as SWORD;
    tPitch = *pPitch as SWORD;
    //set the pitch limits based on the weapon stats of the attacker
    pitchUpperLimit = 0 as libc::c_int;
    pitchLowerLimit = pitchUpperLimit;
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        pitchLowerLimit =
            (*asWeaponStats.offset((*(psAttacker as
                                          *mut STRUCTURE)).asWeaps[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize].nStat
                                       as isize)).minElevation as SDWORD;
        pitchUpperLimit =
            (*asWeaponStats.offset((*(psAttacker as
                                          *mut STRUCTURE)).asWeaps[0 as
                                                                       libc::c_int
                                                                       as
                                                                       usize].nStat
                                       as isize)).maxElevation as SDWORD
    } else if (*psAttacker).type_0 as libc::c_uint ==
                  OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psAttacker as *mut DROID;
        if (*psDroid).droidType as libc::c_uint ==
               DROID_WEAPON as libc::c_int as libc::c_uint ||
               (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
               (*psDroid).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint ||
               (*psDroid).droidType as libc::c_uint ==
                   DROID_CYBORG as libc::c_int as libc::c_uint ||
               (*psDroid).droidType as libc::c_uint ==
                   DROID_CYBORG_SUPER as libc::c_int as libc::c_uint {
            pitchLowerLimit =
                (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                              usize].nStat as
                                           isize)).minElevation as SDWORD;
            pitchUpperLimit =
                (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                              usize].nStat as
                                           isize)).maxElevation as SDWORD
        } else if (*psDroid).droidType as libc::c_uint ==
                      DROID_REPAIR as libc::c_int as libc::c_uint {
            pitchLowerLimit = 30 as libc::c_int;
            pitchUpperLimit = -(15 as libc::c_int)
        }
    }
    //get the maximum rotation this frame
    rotRate =
        (rotRate as
             libc::c_uint).wrapping_mul(frameTime).wrapping_div(1000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
            as SWORD;
    if rotRate as libc::c_int > 180 as libc::c_int {
        //crop to 180 degrees, no point in turning more than all the way round
        rotRate = 180 as libc::c_int as SWORD
    }
    if rotRate as libc::c_int <= 0 as libc::c_int {
        rotRate = 1 as libc::c_int as SWORD
    }
    pitchRate =
        (pitchRate as
             libc::c_uint).wrapping_mul(frameTime).wrapping_div(1000 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
            as SWORD;
    if pitchRate as libc::c_int > 180 as libc::c_int {
        //crop to 180 degrees, no point in turning more than all the way round
        pitchRate = 180 as libc::c_int as SWORD
    }
    if pitchRate as libc::c_int <= 0 as libc::c_int {
        pitchRate = 1 as libc::c_int as SWORD
    }
    /*	if ( (psAttacker->type == OBJ_STRUCTURE) &&
		 (((STRUCTURE *)psAttacker)->pStructureType->type == REF_DEFENSE) &&
		 (asWeaponStats[((STRUCTURE *)psAttacker)->asWeaps[0].nStat].surfaceToAir == SHOOT_IN_AIR) )
	{
		rotRate = 180;
		pitchRate = 180;
	}*/
    //and point the turret at target
    targetRotation =
        calcDirection((*psAttacker).x as UDWORD, (*psAttacker).y as UDWORD,
                      (*psTarget).x as UDWORD, (*psTarget).y as UDWORD);
    //DBP1(("att: rotrate=%d framTime=%d rotation=%d target=%d   startrot=%d ",psAttacker->turretRotRate,frameTime,rotation,targetRotation,tRotation));
    rotationError =
        targetRotation -
            (tRotation as libc::c_int +
                 (*psAttacker).direction as libc::c_int);
    //restrict rotationerror to =/- 180 degrees
    while rotationError > 180 as libc::c_int {
        rotationError -= 360 as libc::c_int
    }
    while rotationError < -(180 as libc::c_int) {
        rotationError += 360 as libc::c_int
    }
    if -rotationError > rotRate as SDWORD {
        // subtract rotation
        if (tRotation as libc::c_int) < rotRate as libc::c_int {
            tRotation =
                (tRotation as libc::c_int + 360 as libc::c_int -
                     rotRate as libc::c_int) as SWORD
        } else {
            tRotation =
                (tRotation as libc::c_int - rotRate as libc::c_int) as SWORD
        }
    } else if rotationError > rotRate as SDWORD {
        // add rotation
        tRotation =
            (tRotation as libc::c_int + rotRate as libc::c_int) as SWORD;
        tRotation = (tRotation as libc::c_int % 360 as libc::c_int) as SWORD
    } else {
        //roughly there so lock on and fire
        if (*psAttacker).direction as SDWORD > targetRotation {
            tRotation =
                (targetRotation + 360 as libc::c_int -
                     (*psAttacker).direction as libc::c_int) as SWORD
        } else {
            tRotation =
                (targetRotation - (*psAttacker).direction as libc::c_int) as
                    SWORD
        }
        onTarget = 1 as libc::c_int
    }
    tRotation = (tRotation as libc::c_int % 360 as libc::c_int) as SWORD;
    if (*psAttacker).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint &&
           vtolDroid(psAttacker as *mut DROID) != 0 {
        // limit the rotation for vtols
        if tRotation as libc::c_int <= 180 as libc::c_int &&
               tRotation as libc::c_int > 45 as libc::c_int {
            tRotation = 45 as libc::c_int as SWORD
        } else if tRotation as libc::c_int > 180 as libc::c_int &&
                      (tRotation as libc::c_int) < 315 as libc::c_int {
            tRotation = 315 as libc::c_int as SWORD
        }
    }
    /* set muzzle pitch if direct fire */
//	if ( asWeaponStats[psAttacker->asWeaps->nStat].direct == TRUE )
    if !psWeapStats.is_null() &&
           (proj_Direct(psWeapStats) != 0 ||
                (*psAttacker).type_0 as libc::c_uint ==
                    OBJ_DROID as libc::c_int as libc::c_uint &&
                    proj_Direct(psWeapStats) == 0 &&
                    actionInsideMinRange(psDroid, (*psDroid).psActionTarget)
                        != 0) {
        // difference between muzzle position and droid origin is unlikely to affect aiming
// particularly as target origin is used
//		calcDroidMuzzleLocation( psAttacker, &muzzle);
        dx =
            (*psTarget).x as libc::c_int -
                (*psAttacker).x as libc::c_int; //muzzle.x;
        dy =
            (*psTarget).y as libc::c_int -
                (*psAttacker).y as libc::c_int; //muzzle.y;
        //		dz = map_Height(psTarget->x, psTarget->y) - psAttacker->z;//muzzle.z;
        dz =
            (*psTarget).z as libc::c_int -
                (*psAttacker).z as libc::c_int; //muzzle.z;
        /* get target distance */
        fR = trigIntSqrt((dx * dx + dy * dy) as UDWORD);
        targetPitch =
            (atan2(dz as libc::c_double, fR as libc::c_double) * 180.0f64 /
                 3.141592654f64) as SDWORD;
        //tPitch = tPitch;
        if tPitch as libc::c_int > 180 as libc::c_int {
            tPitch = (tPitch as libc::c_int - 360 as libc::c_int) as SWORD
        }
        /* invert calculations for bottom-mounted weapons (i.e. for vtols) */
        if bInvert != 0 {
            tPitch = -(tPitch as libc::c_int) as SWORD;
            targetPitch = -targetPitch
        }
        pitchError = targetPitch - tPitch as libc::c_int;
        if pitchError < -(pitchRate as libc::c_int) {
            // move down
            tPitch =
                (tPitch as libc::c_int - pitchRate as libc::c_int) as SWORD;
            onTarget = 0 as libc::c_int
        } else if pitchError > pitchRate as libc::c_int {
            // add rotation
            tPitch =
                (tPitch as libc::c_int + pitchRate as libc::c_int) as SWORD;
            onTarget = 0 as libc::c_int
        } else {
            //roughly there so lock on and fire
            tPitch = targetPitch as SWORD
        }
        /* re-invert result for bottom-mounted weapons (i.e. for vtols) */
        if bInvert != 0 { tPitch = -(tPitch as libc::c_int) as SWORD }
        if (tPitch as libc::c_int) < pitchLowerLimit {
            // move down
            tPitch = pitchLowerLimit as SWORD;
            onTarget = 0 as libc::c_int
        } else if tPitch as libc::c_int > pitchUpperLimit {
            // add rotation
            tPitch = pitchUpperLimit as SWORD;
            onTarget = 0 as libc::c_int
        }
        if (tPitch as libc::c_int) < 0 as libc::c_int {
            tPitch = (tPitch as libc::c_int + 360 as libc::c_int) as SWORD
        }
    }
    *pRotation = tRotation as UWORD;
    *pPitch = tPitch as UWORD;
    return onTarget;
}
// return whether a droid can see a target to fire on it
// return whether a droid can see a target to fire on it
#[no_mangle]
pub unsafe extern "C" fn actionVisibleTarget(mut psDroid: *mut DROID,
                                             mut psTarget: *mut BASE_OBJECT)
 -> BOOL {
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    //if (psDroid->numWeaps == 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint || vtolDroid(psDroid) != 0 {
        return visibleObject(psDroid as *mut BASE_OBJECT, psTarget)
    }
    psStats =
        asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                    usize].nStat as isize);
    if proj_Direct(psStats) != 0 {
        if visibleObjWallBlock(psDroid as *mut BASE_OBJECT, psTarget) != 0 {
            return 1 as libc::c_int
        }
    } else if orderState(psDroid, DORDER_FIRESUPPORT) != 0 {
        if (*psTarget).visible[(*psDroid).player as usize] != 0 {
            return 1 as libc::c_int
        }
    } else if visibleObject(psDroid as *mut BASE_OBJECT, psTarget) != 0 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn actionAddVtolAttackRun(mut psDroid: *mut DROID) {
    let mut fA: FRACT_D = 0.;
    let mut iVNx: SDWORD = 0;
    let mut iVNy: SDWORD = 0;
    let mut iA: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if !(*psDroid).psActionTarget.is_null() {
        psTarget = (*psDroid).psActionTarget
    } else if !(*psDroid).psTarget.is_null() {
        psTarget = (*psDroid).psTarget
    } else { return }
    // indirect can only attack things they can see unless attacking
		// through a sensor droid - see DORDER_FIRESUPPORT
    /* get normal vector from droid to target */
    iVNx = (*psTarget).x as libc::c_int - (*psDroid).x as libc::c_int;
    iVNy = (*psTarget).y as libc::c_int - (*psDroid).y as libc::c_int;
    /* get magnitude of normal vector */
    fA = trigIntSqrt((iVNx * iVNx + iVNy * iVNy) as UDWORD);
    iA = fA as SDWORD;
    /* add waypoint behind target attack length away*/
    iX = (*psTarget).x as libc::c_int + iVNx * 1000 as libc::c_int / iA;
    iY = (*psTarget).y as libc::c_int + iVNy * 1000 as libc::c_int / iA;
    //	orderAddWayPoint( psDroid, iX, iY );
    if iX <= 0 as libc::c_int || iY <= 0 as libc::c_int ||
           iX > (GetWidthOfMap() << 7 as libc::c_int) as SDWORD ||
           iY > (GetHeightOfMap() << 7 as libc::c_int) as SDWORD {
        debug(LOG_NEVER,
              b"*** actionAddVtolAttackRun: run off map! ***\n\x00" as
                  *const u8 as *const libc::c_char);
    } else { moveDroidToDirect(psDroid, iX as UDWORD, iY as UDWORD); };
    /* update attack run count - done in projectile.c now every time a bullet is fired*/
	//psDroid->sMove.iAttackRuns++;
}
#[no_mangle]
pub unsafe extern "C" fn actionUpdateVtolAttack(mut psDroid: *mut DROID) {
    let mut psWeapStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    /* don't do attack runs whilst returning to base */
    if (*psDroid).order == DORDER_RTB as libc::c_int { return }
    //if (psDroid->numWeaps > 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        psWeapStats =
            asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                        usize].nStat as
                                     isize);
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"actionUpdateVtolAttack: invalid weapon stats pointer\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"action.c\x00" as *const u8 as *const libc::c_char,
                  791 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"actionUpdateVtolAttack\x00")).as_ptr(),
                  b"PTRVALID(psWeapStats, sizeof(WEAPON_STATS))\x00" as
                      *const u8 as *const libc::c_char);
        };
    }
    /* order back to base after fixed number of attack runs */
    if !psWeapStats.is_null() {
        //if ( psDroid->sMove.iAttackRuns >= psWeapStats->vtolAttackRuns )
        if vtolEmpty(psDroid) != 0 { moveToRearm(psDroid); return }
    }
    /* circle around target if hovering and not cyborg */
    if (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int &&
           cyborgDroid(psDroid) == 0 {
        actionAddVtolAttackRun(psDroid);
    };
}
#[no_mangle]
pub unsafe extern "C" fn actionUpdateTransporter(mut psDroid: *mut DROID) {
    //check if transporter has arrived
    if updateTransporter(psDroid) != 0 {
        // Got to destination
        (*psDroid).action = DACTION_NONE as libc::c_int;
        return
    }
    //check the target hasn't become one the same player ID - Electronic Warfare
    if !(*psDroid).psActionTarget.is_null() &&
           (*psDroid).player as libc::c_int ==
               (*(*psDroid).psActionTarget).player as libc::c_int {
        (*psDroid).psActionTarget = 0 as *mut _base_object
    }
    /* check for weapon */
	//if ( psDroid->numWeaps > 0 )
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        if (*psDroid).psActionTarget.is_null() {
            aiNearestTarget(psDroid, &mut (*psDroid).psActionTarget);
        }
        if !(*psDroid).psActionTarget.is_null() {
            if visibleObject(psDroid as *mut BASE_OBJECT,
                             (*psDroid).psActionTarget) != 0 {
                /*if (actionTargetTurret((BASE_OBJECT*)psDroid, psDroid->psActionTarget,
						&(psDroid->turretRotation), &(psDroid->turretPitch),
						psDroid->turretRotRate, (SWORD)(psDroid->turretRotRate/2),
						//asWeaponStats[psDroid->asWeaps->nStat].direct))
						proj_Direct(&asWeaponStats[psDroid->asWeaps->nStat]),
						TRUE))*/
                if actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).psActionTarget,
                                      &mut (*psDroid).turretRotation,
                                      &mut (*psDroid).turretPitch,
                                      &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                     as
                                                                     isize),
                                      1 as libc::c_int) != 0 {
                    // In range - fire !!!
                    combFire((*psDroid).asWeaps.as_mut_ptr(),
                             psDroid as *mut BASE_OBJECT,
                             (*psDroid).psActionTarget);
                }
            } else {
                // lost the target
                (*psDroid).psActionTarget = 0 as *mut _base_object
            }
        }
    };
}
// calculate a position for units to pull back to if they
// need to increase the range between them and a target
#[no_mangle]
pub unsafe extern "C" fn actionCalcPullBackPoint(mut psObj: *mut BASE_OBJECT,
                                                 mut psTarget:
                                                     *mut BASE_OBJECT,
                                                 mut px: *mut SDWORD,
                                                 mut py: *mut SDWORD) {
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut len: SDWORD = 0;
    // get the vector from the target to the object
    xdiff = (*psObj).x as SDWORD - (*psTarget).x as SDWORD;
    ydiff = (*psObj).y as SDWORD - (*psTarget).y as SDWORD;
    len =
        sqrt((xdiff * xdiff + ydiff * ydiff) as libc::c_double) as FRACT as
            SDWORD;
    if len == 0 as libc::c_int {
        xdiff = 128 as libc::c_int;
        ydiff = 128 as libc::c_int
    } else {
        xdiff = xdiff * 128 as libc::c_int / len;
        ydiff = ydiff * 128 as libc::c_int / len
    }
    // create the position
    *px = (*psObj).x as SDWORD + xdiff * 10 as libc::c_int;
    *py = (*psObj).y as SDWORD + ydiff * 10 as libc::c_int;
}
// check whether a droid is in the neighboring tile to a build position
// check whether a droid is in the neighboring tile to a build position
#[no_mangle]
pub unsafe extern "C" fn actionReachedBuildPos(mut psDroid: *mut DROID,
                                               mut x: SDWORD, mut y: SDWORD,
                                               mut psStats: *mut BASE_STATS)
 -> BOOL {
    let mut width: SDWORD = 0;
    let mut breadth: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    // do all calculations in half tile units so that
	// the droid moves to within half a tile of the target
	// NOT ANY MORE - JOHN
    dx = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    dy = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    if StatIsStructure(psStats) != 0 {
        width = (*(psStats as *mut STRUCTURE_STATS)).baseWidth as SDWORD;
        breadth = (*(psStats as *mut STRUCTURE_STATS)).baseBreadth as SDWORD
    } else {
        width = (*(psStats as *mut FEATURE_STATS)).baseWidth as SDWORD;
        breadth = (*(psStats as *mut FEATURE_STATS)).baseBreadth as SDWORD
    }
    tx = x >> 7 as libc::c_int;
    ty = y >> 7 as libc::c_int;
    // move the x,y to the top left of the structure
    tx -= width / 2 as libc::c_int;
    ty -= breadth / 2 as libc::c_int;
    if dx == tx - 1 as libc::c_int || dx == tx + width {
        // droid could be at either the left or the right
        if dy >= ty - 1 as libc::c_int && dy <= ty + breadth {
            return 1 as libc::c_int
        }
    } else if dy == ty - 1 as libc::c_int || dy == ty + breadth {
        // droid could be at either the top or the bottom
        if dx >= tx - 1 as libc::c_int && dx <= tx + width {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// check if a droid is on the foundations of a new building
// check if a droid is on the foundations of a new building
#[no_mangle]
pub unsafe extern "C" fn actionDroidOnBuildPos(mut psDroid: *mut DROID,
                                               mut x: SDWORD, mut y: SDWORD,
                                               mut psStats: *mut BASE_STATS)
 -> BOOL {
    let mut width: SDWORD = 0;
    let mut breadth: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    dx = (*psDroid).x as libc::c_int >> 7 as libc::c_int;
    dy = (*psDroid).y as libc::c_int >> 7 as libc::c_int;
    if StatIsStructure(psStats) != 0 {
        width = (*(psStats as *mut STRUCTURE_STATS)).baseWidth as SDWORD;
        breadth = (*(psStats as *mut STRUCTURE_STATS)).baseBreadth as SDWORD
    } else {
        width = (*(psStats as *mut FEATURE_STATS)).baseWidth as SDWORD;
        breadth = (*(psStats as *mut FEATURE_STATS)).baseBreadth as SDWORD
    }
    tx = (x >> 7 as libc::c_int) - width / 2 as libc::c_int;
    ty = (y >> 7 as libc::c_int) - breadth / 2 as libc::c_int;
    if dx >= tx && dx < tx + width && dy >= ty && dy < ty + breadth {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// return the position of a players home base
// return the position of a players home base
#[no_mangle]
pub unsafe extern "C" fn actionHomeBasePos(mut player: SDWORD,
                                           mut px: *mut SDWORD,
                                           mut py: *mut SDWORD) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if player >= 0 as libc::c_int && player < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"actionHomeBasePos: invalide player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player >= 0 as libc::c_int && player < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              985 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"actionHomeBasePos\x00")).as_ptr(),
              b"player >= 0 && player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_HQ as libc::c_int as libc::c_uint {
            *px = (*psStruct).x as SDWORD;
            *py = (*psStruct).y as SDWORD;
            return
        }
        psStruct = (*psStruct).psNext
    }
    *px = getLandingX(player) as SDWORD;
    *py = getLandingY(player) as SDWORD;
}
// tell the action system of a potential location for walls blocking routing
// tell the action system of a potential location for walls blocking routing
#[no_mangle]
pub unsafe extern "C" fn actionRouteBlockingPos(mut psDroid: *mut DROID,
                                                mut tx: SDWORD,
                                                mut ty: SDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut psWall: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if vtolDroid(psDroid) != 0 ||
           (*psDroid).order != DORDER_MOVE as libc::c_int &&
               (*psDroid).order != DORDER_SCOUT as libc::c_int {
        return 0 as libc::c_int
    }
    // see if there is a wall to attack around the location
    psWall = 0 as *mut STRUCTURE;
    i = tx - 1 as libc::c_int;
    's_28:
        while i <= tx + 1 as libc::c_int {
            j = ty - 1 as libc::c_int;
            while j <= ty + 1 as libc::c_int {
                if tileOnMap(i, j) != 0 {
                    psTile = mapTile(i as UDWORD, j as UDWORD);
                    if (*psTile).tileInfoBits as libc::c_int &
                           0x20 as libc::c_int != 0 {
                        psWall = getTileStructure(i as UDWORD, j as UDWORD);
                        if (*psWall).player as libc::c_int !=
                               (*psDroid).player as libc::c_int {
                            break 's_28 ;
                        }
                        psWall = 0 as *mut STRUCTURE
                    }
                }
                j += 1
            }
            i += 1
        }
    if !psWall.is_null() {
        if (*psDroid).order == DORDER_MOVE as libc::c_int {
            (*psDroid).order = DORDER_MOVE_ATTACKWALL as libc::c_int
        } else if (*psDroid).order == DORDER_SCOUT as libc::c_int {
            (*psDroid).order = DORDER_SCOUT_ATTACKWALL as libc::c_int
        }
        (*psDroid).psTarget = psWall as *mut BASE_OBJECT;
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// 10 secs
/* Update the action state for a droid */
// Update the action state for a droid
#[no_mangle]
pub unsafe extern "C" fn actionUpdateDroid(mut psDroid: *mut DROID) {
    //	UDWORD				structX,structY;
    let mut droidX: UDWORD = 0; //, *psObj;
    let mut droidY: UDWORD = 0;
    let mut tlx: UDWORD = 0;
    let mut tly: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStructStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut psTarget: *mut BASE_OBJECT = (*psDroid).psTarget;
    let mut psWeapStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut targetDir: SDWORD = 0;
    let mut dirDiff: SDWORD = 0;
    let mut pbx: SDWORD = 0;
    let mut pby: SDWORD = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut rangeSq: SDWORD = 0;
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut bChaseBloke: BOOL = 0;
    let mut bInvert: BOOL = 0;
    let mut psNextWreck: *mut FEATURE = 0 as *mut FEATURE;
    let mut actionUpdateFunc:
            Option<unsafe extern "C" fn(_: *mut DROID) -> BOOL> = None;
    let mut moveAction: SDWORD = 0;
    let mut bDoHelpBuild: BOOL = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"actionUpdateUnit: invalid propulsion stats pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              1084 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    if ((*psDroid).turretRotation as libc::c_int) < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"turretRotation out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroid).turretRotation as libc::c_int) < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              1086 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
              b"psDroid->turretRotation < 360\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroid).direction as libc::c_int) < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"unit direction out of range\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*psDroid).direction as libc::c_int) < 360 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              1087 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
              b"psDroid->direction < 360\x00" as *const u8 as
                  *const libc::c_char);
    };
    /* check whether turret inverted for actionTargetTurret */
	//if ( psDroid->droidType != DROID_CYBORG &&
    if cyborgDroid(psDroid) == 0 &&
           (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
       {
        bInvert = 1 as libc::c_int
    } else { bInvert = 0 as libc::c_int }
    // clear the target if it has died
    if !(*psDroid).psActionTarget.is_null() &&
           (*(*psDroid).psActionTarget).died != 0 {
        (*psDroid).psActionTarget = 0 as *mut _base_object;
        if (*psDroid).action != DACTION_MOVEFIRE as libc::c_int &&
               (*psDroid).action != DACTION_TRANSPORTIN as libc::c_int &&
               (*psDroid).action != DACTION_TRANSPORTOUT as libc::c_int {
            (*psDroid).action = DACTION_NONE as libc::c_int;
            //if Vtol - return to rearm pad
            if vtolDroid(psDroid) != 0 { moveToRearm(psDroid); }
        }
    }
    //if the droid has been attacked by an EMP weapon, it is temporarily disabled
    if (*psDroid).lastHitWeapon == WSC_EMP as libc::c_int as libc::c_uint {
        if gameTime.wrapping_sub((*psDroid).timeLastHit) >
               10000 as libc::c_int as libc::c_uint {
            //the actionStarted time needs to be adjusted
            (*psDroid).actionStarted =
                ((*psDroid).actionStarted as
                     libc::c_uint).wrapping_add(gameTime.wrapping_sub((*psDroid).timeLastHit))
                    as UDWORD as UDWORD;
            //reset the lastHit parameters
            (*psDroid).timeLastHit = 0 as libc::c_int as UDWORD;
            (*psDroid).lastHitWeapon = 0xffffffff as libc::c_uint
        } else {
            //get out without updating
            return
        }
    }
    //if (psDroid->numWeaps > 0)
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        psWeapStats =
            asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                        usize].nStat as isize)
    } else { psWeapStats = 0 as *mut WEAPON_STATS }
    match (*psDroid).action {
        0 | 26 => {
            // doing nothing
  		//since not doing anything, see if need to self repair
            if selfRepairEnabled((*psDroid).player) != 0 {
                //wait for 1 second to give the repair facility a chance to do the repair work
                if gameTime.wrapping_sub((*psDroid).actionStarted) >
                       1000 as libc::c_int as libc::c_uint {
                    droidSelfRepair(psDroid);
                }
            }
        }
        28 => {
            // don't want to be in a formation for this move
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            // move back to the repair facility if necessary
            if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                    (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                    ||
                    (*psDroid).sMove.Status as libc::c_int ==
                        12 as libc::c_int) &&
                   actionReachedBuildPos(psDroid,
                                         (*(*psDroid).psTarget).x as SDWORD,
                                         (*(*psDroid).psTarget).y as SDWORD,
                                         (*((*psDroid).psTarget as
                                                *mut STRUCTURE)).pStructureType
                                             as *mut BASE_STATS) == 0 {
                moveDroidToNoFormation(psDroid,
                                       (*(*psDroid).psTarget).x as UDWORD,
                                       (*(*psDroid).psTarget).y as UDWORD);
            }
        }
        12 => {
            //if we're moving droids to safety and currently waiting to fly back in, see if time is up
            if (*psDroid).player as libc::c_uint == selectedPlayer &&
                   getDroidsToSafetyFlag() != 0 {
                if (mission.ETA as
                        libc::c_uint).wrapping_sub(gameTime.wrapping_sub(missionGetReinforcementTime()))
                       as SDWORD <= 0 as libc::c_int {
                    if droidRemove(psDroid,
                                   mission.apsDroidLists.as_mut_ptr()) == 0 {
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"actionUpdate: Unable to remove transporter from mission list\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if 0 as libc::c_int != 0 {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"action.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1186 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 18],
                                                            &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
                                  b"FALSE\x00" as *const u8 as
                                      *const libc::c_char);
                        };
                    }
                    addDroid(psDroid, apsDroidLists.as_mut_ptr());
                    //set the x/y up since they were set to INVALID_XY when moved offWorld
                    missionGetTransporterExit(selectedPlayer as SDWORD,
                                              &mut droidX as *mut UDWORD as
                                                  *mut UWORD,
                                              &mut droidY as *mut UDWORD as
                                                  *mut UWORD);
                    (*psDroid).x = droidX as UWORD;
                    (*psDroid).y = droidY as UWORD;
                    //fly Transporter back to get some more droids
                    orderDroidLoc(psDroid, DORDER_TRANSPORTIN,
                                  getLandingX(selectedPlayer as SDWORD) as
                                      UDWORD,
                                  getLandingY(selectedPlayer as SDWORD) as
                                      UDWORD);
                } else if missionDroidsRemaining(selectedPlayer) == 0 {
                    /*if we're currently moving units to safety and waiting to fly
                back in - check there is something to fly back for!*/
                    //the script can call startMission for this callback for offworld missions
                    eventFireCallbackTrigger(CALL_START_NEXT_LEVEL as
                                                 libc::c_int as TRIGGER_TYPE);
                }
            }
        }
        1 => {
            // moving to a location
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                // Got to destination
                (*psDroid).action = DACTION_NONE as libc::c_int
                //if vtol and offworld and empty - 'magic' it back home!
/*			alternatively - lets not - John.
			if (vtolEmpty(psDroid) AND missionIsOffworld())
			{
				//check has reached LZ
				xdiff = (SDWORD)psDroid->x - (SDWORD)psDroid->actionX;
				ydiff = (SDWORD)psDroid->y - (SDWORD)psDroid->actionY;
				if (xdiff*xdiff + ydiff*ydiff <= TILE_UNITS*TILE_UNITS)
				{
					//magic! - take droid out of list and add to one back at home base
					if (droidRemove(psDroid, apsDroidLists))
                    {
                        //only add to mission lists if successfully removed from current droid lists
					    addDroid(psDroid, mission.apsDroidLists);
    					//make sure fully armed etc by time back at home
	    				mendVtol(psDroid);
		    			addConsoleMessage("VTOL MAGIC!",DEFAULT_JUSTIFY);
                    }
                    //else the droid should be dead!
				}
				else
				{
					//re-order the droid back to the LZ
					orderDroidLoc(psDroid, DORDER_MOVE, getLandingX(psDroid->player),
						getLandingY(psDroid->player));
				}
			}*/
            } else if vtolDroid(psDroid) == 0 &&
                          (*psDroid).asWeaps[0 as libc::c_int as usize].nStat
                              > 0 as libc::c_int as libc::c_uint &&
                          (*psWeapStats).rotate as libc::c_int != 0 &&
                          (*psWeapStats).fireOnMove as libc::c_uint !=
                              FOM_NO as libc::c_int as libc::c_uint &&
                          aiNearestTarget(psDroid,
                                          &mut (*psDroid).psActionTarget) != 0
             {
                if secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &mut state) !=
                       0 {
                    if state as libc::c_uint ==
                           DSS_ALEV_ALWAYS as libc::c_int as libc::c_uint {
                        (*psDroid).action = DACTION_MOVEFIRE as libc::c_int
                    }
                } else { (*psDroid).action = DACTION_MOVEFIRE as libc::c_int }
            }
        }
        38 | 39 => {
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                /*		else if(secondaryGetState(psDroid, DSO_HALTTYPE, &state) && (state == DSS_HALT_HOLD))
		{
			psDroid->action = DACTION_NONE;		// hold is set, stop moving.
			moveStopDroid(psDroid);
		}*/
                /*		else if ((psDroid->order == DORDER_SCOUT) &&
				 aiNearestTarget(psDroid, &psObj))
		{
			if (psDroid->numWeaps > 0)
			{
				orderDroidObj(psDroid, DORDER_ATTACK, psObj);
			}
			else
			{
				orderDroid(psDroid, DORDER_STOP);
			}
		}*/
                //else if (psDroid->numWeaps > 0 &&
                // Got to destination
                (*psDroid).action = DACTION_NONE as libc::c_int
            }
        }
        13 | 11 => { actionUpdateTransporter(psDroid); }
        17 => {
            //check if vtol that its armed
            if vtolEmpty(psDroid) != 0 { moveToRearm(psDroid); }
            // firing on something while moving
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                // Got to desitination
                (*psDroid).action = DACTION_NONE as libc::c_int
            } else if (*psDroid).psActionTarget.is_null() ||
                          validTarget(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).psActionTarget) == 0 ||
                          secondaryGetState(psDroid, DSO_ATTACK_LEVEL,
                                            &mut state) != 0 &&
                              state as libc::c_uint !=
                                  DSS_ALEV_ALWAYS as libc::c_int as
                                      libc::c_uint {
                // Target lost
                (*psDroid).action = DACTION_MOVE as libc::c_int
                //if Vtol - return to rearm pad
            /*if (vtolDroid(psDroid))
            {
			    moveToRearm(psDroid);
            }*/
            } else if electronicDroid(psDroid) != 0 &&
                          (*psDroid).player as libc::c_int ==
                              (*(*psDroid).psActionTarget).player as
                                  libc::c_int {
                (*psDroid).psActionTarget = 0 as *mut _base_object;
                (*psDroid).action = DACTION_NONE as libc::c_int
            } else if visibleObject(psDroid as *mut BASE_OBJECT,
                                    (*psDroid).psActionTarget) != 0 {
                //check the target hasn't become one the same player ID - Electronic Warfare
                /*if (actionTargetTurret((BASE_OBJECT*)psDroid, psDroid->psActionTarget,
										&(psDroid->turretRotation), &(psDroid->turretPitch),
										psDroid->turretRotRate, (SWORD)(psDroid->turretRotRate/2),
										//asWeaponStats[psDroid->asWeaps->nStat].direct))
										proj_Direct(&asWeaponStats[psDroid->asWeaps->nStat]),
										bInvert))*/
                if actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).psActionTarget,
                                      &mut (*psDroid).turretRotation,
                                      &mut (*psDroid).turretPitch,
                                      &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                     as
                                                                     isize),
                                      bInvert) != 0 {
                    // In range - fire !!!
                    combFire((*psDroid).asWeaps.as_mut_ptr(),
                             psDroid as *mut BASE_OBJECT,
                             (*psDroid).psActionTarget);
                }
            } else {
                // lost the target
                (*psDroid).action = DACTION_MOVE as libc::c_int;
                (*psDroid).psActionTarget = 0 as *mut _base_object
            }
            //check its a VTOL unit since adding Transporter's into multiPlayer
		/* check vtol attack runs */
		//if ( psPropStats->propulsionType == LIFT )
            if vtolDroid(psDroid) != 0 { actionUpdateVtolAttack(psDroid); }
        }
        6 => {
            if !(*psDroid).psActionTarget.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"actionUpdateUnit: target is NULL while attacking\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !(*psDroid).psActionTarget.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      1372 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
                      b"psDroid->psActionTarget != NULL\x00" as *const u8 as
                          *const libc::c_char);
            };
            // don't wan't formations for this one
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            //check the target hasn't become one the same player ID - Electronic Warfare
            if electronicDroid(psDroid) != 0 &&
                   (*psDroid).player as libc::c_int ==
                       (*(*psDroid).psActionTarget).player as libc::c_int ||
                   validTarget(psDroid as *mut BASE_OBJECT,
                               (*psDroid).psActionTarget) == 0 {
                // ||
                //			(secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &state) && (state != DSS_ALEV_ALWAYS)))
                (*psDroid).psActionTarget = 0 as *mut _base_object;
                (*psDroid).action = DACTION_NONE as libc::c_int
            } else if actionVisibleTarget(psDroid, (*psDroid).psActionTarget)
                          != 0 &&
                          actionInAttackRange(psDroid,
                                              (*psDroid).psActionTarget) != 0
             {
                if (*psWeapStats).rotate == 0 {
                    // no rotating turret - need to check aligned with target
                    targetDir =
                        calcDirection((*psDroid).x as UDWORD,
                                      (*psDroid).y as UDWORD,
                                      (*(*psDroid).psActionTarget).x as
                                          UDWORD,
                                      (*(*psDroid).psActionTarget).y as
                                          UDWORD);
                    dirDiff =
                        labs((targetDir - (*psDroid).direction as SDWORD) as
                                 libc::c_long) as SDWORD
                } else { dirDiff = 0 as libc::c_int }
                if dirDiff > 1 as libc::c_int {
                    if (*psDroid).sMove.Status as libc::c_int !=
                           12 as libc::c_int {
                        (*psDroid).action =
                            DACTION_ROTATETOATTACK as libc::c_int;
                        moveTurnDroid(psDroid,
                                      (*(*psDroid).psActionTarget).x as
                                          UDWORD,
                                      (*(*psDroid).psActionTarget).y as
                                          UDWORD);
                    }
                } else if (*psWeapStats).rotate == 0 ||
                              actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                                 (*psDroid).psActionTarget,
                                                 &mut (*psDroid).turretRotation,
                                                 &mut (*psDroid).turretPitch,
                                                 &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                                as
                                                                                isize),
                                                 bInvert) != 0 {
                    /* In range - fire !!! */
                    combFire((*psDroid).asWeaps.as_mut_ptr(),
                             psDroid as *mut BASE_OBJECT,
                             (*psDroid).psActionTarget);
                }
            } else if ((*psDroid).order == DORDER_ATTACKTARGET as libc::c_int
                           ||
                           (*psDroid).order ==
                               DORDER_FIRESUPPORT as libc::c_int) &&
                          secondaryGetState(psDroid, DSO_HALTTYPE, &mut state)
                              != 0 &&
                          state as libc::c_uint ==
                              DSS_HALT_HOLD as libc::c_int as libc::c_uint ||
                          vtolDroid(psDroid) == 0 &&
                              orderStateObj(psDroid, DORDER_FIRESUPPORT,
                                            &mut psTarget) != 0 &&
                              (*psTarget).type_0 as libc::c_uint ==
                                  OBJ_STRUCTURE as libc::c_int as libc::c_uint
             {
                // don't move if on hold or firesupport for a sensor tower
                (*psDroid).action = DACTION_NONE as libc::c_int
                // holding, cancel the order.
            } else {
                (*psDroid).action = DACTION_MOVETOATTACK as libc::c_int
                // out of range - chase it
            }
        }
        36 => {
            //check if vtol that its armed
            if vtolEmpty(psDroid) != 0 || (*psDroid).psActionTarget.is_null()
                   ||
                   electronicDroid(psDroid) != 0 &&
                       (*psDroid).player as libc::c_int ==
                           (*(*psDroid).psActionTarget).player as libc::c_int
                   ||
                   validTarget(psDroid as *mut BASE_OBJECT,
                               (*psDroid).psActionTarget) == 0 {
                // ||
                //			 (secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &state) && (state != DSS_ALEV_ALWAYS)))
                moveToRearm(psDroid);
            } else {
                if actionVisibleTarget(psDroid, (*psDroid).psActionTarget) !=
                       0 {
                    if actionInRange(psDroid, (*psDroid).psActionTarget) != 0
                       {
                        if (*psDroid).player as libc::c_uint == selectedPlayer
                           {
                            audio_QueueTrackMinDelay(ID_SOUND_COMMENCING_ATTACK_RUN2
                                                         as libc::c_int,
                                                     (3 as libc::c_int *
                                                          1000 as libc::c_int)
                                                         as UDWORD);
                        }
                        if actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                              (*psDroid).psActionTarget,
                                              &mut (*psDroid).turretRotation,
                                              &mut (*psDroid).turretPitch,
                                              &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                             as
                                                                             isize),
                                              bInvert) != 0 {
                            // In range - fire !!!
                            combFire((*psDroid).asWeaps.as_mut_ptr(),
                                     psDroid as *mut BASE_OBJECT,
                                     (*psDroid).psActionTarget);
                        }
                    } else {
                        actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                           (*psDroid).psActionTarget,
                                           &mut (*psDroid).turretRotation,
                                           &mut (*psDroid).turretPitch,
                                           &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                          as
                                                                          isize),
                                           bInvert);
                    }
                }
                /*		else
		{
			// lost the target
			psDroid->action = DACTION_MOVETOATTACK;
			moveDroidTo(psDroid, psDroid->psActionTarget->x, psDroid->psActionTarget->y);
		}*/
                /* check vtol attack runs */
//		actionUpdateVtolAttack( psDroid );
                /* circle around target if hovering and not cyborg */
                if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
                       ||
                       (*psDroid).sMove.Status as libc::c_int ==
                           8 as libc::c_int ||
                       (*psDroid).sMove.Status as libc::c_int ==
                           12 as libc::c_int {
                    actionAddVtolAttackRun(psDroid);
                } else {
                    // if the vtol is close to the target, go around again
                    xdiff =
                        (*psDroid).x as SDWORD -
                            (*(*psDroid).psActionTarget).x as SDWORD;
                    ydiff =
                        (*psDroid).y as SDWORD -
                            (*(*psDroid).psActionTarget).y as SDWORD;
                    rangeSq = xdiff * xdiff + ydiff * ydiff;
                    if rangeSq < 400 as libc::c_int * 400 as libc::c_int {
                        // don't do another attack run if already moving away from the target
                        xdiff =
                            (*psDroid).sMove.DestinationX -
                                (*(*psDroid).psActionTarget).x as SDWORD;
                        ydiff =
                            (*psDroid).sMove.DestinationY -
                                (*(*psDroid).psActionTarget).y as SDWORD;
                        if xdiff * xdiff + ydiff * ydiff <
                               400 as libc::c_int * 400 as libc::c_int {
                            actionAddVtolAttackRun(psDroid);
                        }
                    } else if rangeSq >
                                  (*psWeapStats).longRange.wrapping_mul((*psWeapStats).longRange)
                                      as SDWORD {
                        // if the vtol is far enough away head for the target again
//			else if (rangeSq > (VTOL_ATTACK_RETURNDIST*VTOL_ATTACK_RETURNDIST))
                        // don't do another attack run if already heading for the target
                        xdiff =
                            (*psDroid).sMove.DestinationX -
                                (*(*psDroid).psActionTarget).x as SDWORD;
                        ydiff =
                            (*psDroid).sMove.DestinationY -
                                (*(*psDroid).psActionTarget).y as SDWORD;
                        if xdiff * xdiff + ydiff * ydiff >
                               400 as libc::c_int * 400 as libc::c_int {
                            moveDroidToDirect(psDroid,
                                              (*(*psDroid).psActionTarget).x
                                                  as UDWORD,
                                              (*(*psDroid).psActionTarget).y
                                                  as UDWORD);
                        }
                    }
                }
            }
        }
        23 => {
            // don't wan't formations for this one
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            // send vtols back to rearm
            if vtolDroid(psDroid) != 0 && vtolEmpty(psDroid) != 0 {
                moveToRearm(psDroid);
            } else if electronicDroid(psDroid) != 0 &&
                          (*psDroid).player as libc::c_int ==
                              (*(*psDroid).psActionTarget).player as
                                  libc::c_int ||
                          validTarget(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).psActionTarget) == 0 {
                //check the target hasn't become one the same player ID - Electronic Warfare
                // ||
                //			(secondaryGetState(psDroid, DSO_ATTACK_LEVEL, &state) && (state != DSS_ALEV_ALWAYS)))
                (*psDroid).psActionTarget = 0 as *mut _base_object;
                (*psDroid).action = DACTION_NONE as libc::c_int
            } else {
                if actionVisibleTarget(psDroid, (*psDroid).psActionTarget) !=
                       0 {
                    if (*psWeapStats).rotate != 0 {
                        /*actionTargetTurret((BASE_OBJECT*)psDroid, psDroid->psActionTarget,
										    &(psDroid->turretRotation), &(psDroid->turretPitch),
										    psDroid->turretRotRate, (SWORD)(psDroid->turretRotRate/2),
										    //asWeaponStats[psDroid->asWeaps->nStat].direct);
										    proj_Direct(&asWeaponStats[psDroid->asWeaps->nStat]),
										    bInvert);*/
                        actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                           (*psDroid).psActionTarget,
                                           &mut (*psDroid).turretRotation,
                                           &mut (*psDroid).turretPitch,
                                           &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr()).nStat
                                                                          as
                                                                          isize),
                                           bInvert);
                    }
                    bChaseBloke = 0 as libc::c_int;
                    if vtolDroid(psDroid) == 0 &&
                           (*(*psDroid).psActionTarget).type_0 as libc::c_uint
                               == OBJ_DROID as libc::c_int as libc::c_uint &&
                           (*((*psDroid).psActionTarget as
                                  *mut DROID)).droidType as libc::c_uint ==
                               DROID_PERSON as libc::c_int as libc::c_uint &&
                           (*psWeapStats).fireOnMove as libc::c_uint !=
                               FOM_NO as libc::c_int as libc::c_uint {
                        bChaseBloke = 1 as libc::c_int
                    }
                    if actionInAttackRange(psDroid, (*psDroid).psActionTarget)
                           != 0 && bChaseBloke == 0 {
                        /* Stop the droid moving any closer */
//				    ASSERT( psDroid->x != 0 && psDroid->y != 0,
//						"moveUpdateUnit: Unit at (0,0)" );
                        /* init vtol attack runs count if necessary */
                        if (*psPropStats).propulsionType as libc::c_int ==
                               LIFT as libc::c_int {
                            (*psDroid).action =
                                DACTION_VTOLATTACK as libc::c_int
                            //actionAddVtolAttackRun( psDroid );
					    //actionUpdateVtolAttack( psDroid );
                        } else {
                            moveStopDroid(psDroid);
                            if (*psWeapStats).rotate != 0 {
                                (*psDroid).action =
                                    DACTION_ATTACK as libc::c_int
                            } else {
                                (*psDroid).action =
                                    DACTION_ROTATETOATTACK as libc::c_int;
                                moveTurnDroid(psDroid,
                                              (*(*psDroid).psActionTarget).x
                                                  as UDWORD,
                                              (*(*psDroid).psActionTarget).y
                                                  as UDWORD);
                            }
                        }
                    } else if actionInRange(psDroid,
                                            (*psDroid).psActionTarget) != 0 {
                        // fire while closing range
                        combFire((*psDroid).asWeaps.as_mut_ptr(),
                                 psDroid as *mut BASE_OBJECT,
                                 (*psDroid).psActionTarget);
                    }
                } else if (*psDroid).turretRotation as libc::c_int !=
                              0 as libc::c_int ||
                              (*psDroid).turretPitch as libc::c_int !=
                                  0 as libc::c_int {
                    actionAlignTurret(psDroid as *mut BASE_OBJECT);
                }
                if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
                        ||
                        (*psDroid).sMove.Status as libc::c_int ==
                            8 as libc::c_int ||
                        (*psDroid).sMove.Status as libc::c_int ==
                            12 as libc::c_int) &&
                       (*psDroid).action != DACTION_ATTACK as libc::c_int {
                    /* Stopped moving but haven't reached the target - possibly move again */
                    if (*psDroid).order == DORDER_ATTACKTARGET as libc::c_int
                           &&
                           secondaryGetState(psDroid, DSO_HALTTYPE,
                                             &mut state) != 0 &&
                           state as libc::c_uint ==
                               DSS_HALT_HOLD as libc::c_int as libc::c_uint {
                        (*psDroid).action = DACTION_NONE as libc::c_int
                        // on hold, give up.
                    } else if actionInsideMinRange(psDroid,
                                                   (*psDroid).psActionTarget)
                                  != 0 {
                        if proj_Direct(psWeapStats) != 0 {
                            // try and extend the range
                            actionCalcPullBackPoint(psDroid as
                                                        *mut BASE_OBJECT,
                                                    (*psDroid).psActionTarget,
                                                    &mut pbx, &mut pby);
                            moveDroidTo(psDroid, pbx as UDWORD,
                                        pby as UDWORD);
                        } else if (*psWeapStats).rotate != 0 {
                            (*psDroid).action = DACTION_ATTACK as libc::c_int
                        } else {
                            (*psDroid).action =
                                DACTION_ROTATETOATTACK as libc::c_int;
                            moveTurnDroid(psDroid,
                                          (*(*psDroid).psActionTarget).x as
                                              UDWORD,
                                          (*(*psDroid).psActionTarget).y as
                                              UDWORD);
                        }
                    } else {
                        // try to close the range
                        moveDroidTo(psDroid,
                                    (*(*psDroid).psActionTarget).x as UDWORD,
                                    (*(*psDroid).psActionTarget).y as UDWORD);
                    }
                }
            }
        }
        9 => {
            // unable to route to target ... don't do anything aggressive until time is up
		// we need to do something defensive at this point ???
		//if (gameTime>psDroid->actionHeight)				// actionHeight is used here for the ending time for this action
            //hmmm, hope this doesn't cause any problems!
            if gameTime > (*psDroid).actionStarted {
                (*psDroid).action = DACTION_NONE as libc::c_int
                // Sulking is over lets get back to the action ... is this all I need to do to get it back into the action?
            }
        }
        24 => {
            //		if (DROID_STOPPED(psDroid))
            if (*psDroid).sMove.Status as libc::c_int != 6 as libc::c_int {
                (*psDroid).action = DACTION_ATTACK as libc::c_int
            }
        }
        18 => {
            // The droid cannot be in a formation
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            // moving to a location to build a structure
            if actionReachedBuildPos(psDroid, (*psDroid).orderX as SDWORD,
                                     (*psDroid).orderY as SDWORD,
                                     (*psDroid).psTarStats) != 0 &&
                   actionDroidOnBuildPos(psDroid, (*psDroid).orderX as SDWORD,
                                         (*psDroid).orderY as SDWORD,
                                         (*psDroid).psTarStats) == 0 {
                moveStopDroid(psDroid);
                /*			xdiff = (SDWORD)psDroid->x - (SDWORD)psDroid->actionX;
			ydiff = (SDWORD)psDroid->y - (SDWORD)psDroid->actionY;
			if ( xdiff*xdiff + ydiff*ydiff >= TILE_UNITS*TILE_UNITS )
			{
				DBP2(("DACTION_MOVETOBUILD: new location\n"));
				// Couldn't reach destination - try and find a new one
				if (actionNewBuildPos(psDroid, &droidX,&droidY))
				{
					psDroid->actionX = droidX;
					psDroid->actionY = droidY;
					moveDroidTo(psDroid, droidX,droidY);
				}
				else
				{
					DBP2(("DACTION_MOVETOBUILD: giving up\n"));
					psDroid->action = DACTION_NONE;
				}
			}
			else*/
                bDoHelpBuild = 0 as libc::c_int;
                // Got to destination - start building
                psStructStats = (*psDroid).psTarStats as *mut STRUCTURE_STATS;
                if (*psDroid).order == DORDER_BUILD as libc::c_int &&
                       (*psDroid).psTarget.is_null() {
                    //					&&  (
//					psStructStats->type != REF_WALL ||
//					psStructStats->type != REF_WALLCORNER))
					//psDroid->order == DORDER_LINEBUILD)
                    // Starting a new structure
					// calculate the top left of the structure
                    tlx =
                        ((*psDroid).orderX as SDWORD -
                             (*psStructStats).baseWidth.wrapping_mul(128 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                 as SDWORD / 2 as libc::c_int) as UDWORD;
                    tly =
                        ((*psDroid).orderY as SDWORD -
                             (*psStructStats).baseBreadth.wrapping_mul(128 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint)
                                 as SDWORD / 2 as libc::c_int) as UDWORD;
                    if IsStatExpansionModule(psStructStats) != 0 {
                        setUpBuildModule(psDroid);
                    } else if (*mapTile(((*psDroid).orderX as libc::c_int >>
                                             7 as libc::c_int) as UDWORD,
                                        ((*psDroid).orderY as libc::c_int >>
                                             7 as libc::c_int) as
                                            UDWORD)).tileInfoBits as
                                  libc::c_int & 0x1 as libc::c_int != 0 {
                        // structure on the build location - see if it is the same type
                        psStruct =
                            getTileStructure(((*psDroid).orderX as libc::c_int
                                                  >> 7 as libc::c_int) as
                                                 UDWORD,
                                             ((*psDroid).orderY as libc::c_int
                                                  >> 7 as libc::c_int) as
                                                 UDWORD);
                        if (*psStruct).pStructureType ==
                               (*psDroid).psTarStats as *mut STRUCTURE_STATS {
                            // same type - do a help build
                            (*psDroid).psTarget =
                                psStruct as *mut BASE_OBJECT;
                            bDoHelpBuild = 1 as libc::c_int
                        } else if ((*(*psStruct).pStructureType).type_0 ==
                                       REF_WALL as libc::c_int as libc::c_uint
                                       ||
                                       (*(*psStruct).pStructureType).type_0 ==
                                           REF_WALLCORNER as libc::c_int as
                                               libc::c_uint) &&
                                      (*((*psDroid).psTarStats as
                                             *mut STRUCTURE_STATS)).type_0 ==
                                          REF_DEFENSE as libc::c_int as
                                              libc::c_uint {
                            // building a gun tower over a wall - OK
                            if droidStartBuild(psDroid) != 0 {
                                (*psDroid).action =
                                    DACTION_BUILD as libc::c_int
                            } else {
                                (*psDroid).action =
                                    DACTION_NONE as libc::c_int
                            }
                        } else {
                            (*psDroid).action = DACTION_NONE as libc::c_int
                        }
                    } else if validLocation((*psDroid).psTarStats as
                                                *mut BASE_STATS,
                                            tlx >> 7 as libc::c_int,
                                            tly >> 7 as libc::c_int,
                                            (*psDroid).player as UDWORD,
                                            0 as libc::c_int) == 0 {
                        (*psDroid).action = DACTION_NONE as libc::c_int
                    } else if droidStartFoundation(psDroid) != 0 {
                        (*psDroid).action =
                            DACTION_BUILD_FOUNDATION as libc::c_int
                    } else { (*psDroid).action = DACTION_NONE as libc::c_int }
                } else if ((*psDroid).order == DORDER_LINEBUILD as libc::c_int
                               ||
                               (*psDroid).order ==
                                   DORDER_BUILD as libc::c_int) &&
                              ((*psStructStats).type_0 ==
                                   REF_WALL as libc::c_int as libc::c_uint ||
                                   (*psStructStats).type_0 ==
                                       REF_WALLCORNER as libc::c_int as
                                           libc::c_uint ||
                                   (*psStructStats).type_0 ==
                                       REF_DEFENSE as libc::c_int as
                                           libc::c_uint) {
                    //					tlx = tlx >> TILE_UNITS;
//					tly = tly >> TILE_UNITS;
                    //need to check if something has already started building here?
					//unless its a module!
                    // building a wall.
                    psTile =
                        mapTile(((*psDroid).orderX as libc::c_int >>
                                     7 as libc::c_int) as UDWORD,
                                ((*psDroid).orderY as libc::c_int >>
                                     7 as libc::c_int) as UDWORD);
                    if (*psDroid).psTarget.is_null() &&
                           ((*psTile).tileInfoBits as libc::c_int &
                                0x1 as libc::c_int != 0 ||
                                (*psTile).tileInfoBits as libc::c_int &
                                    0x2 as libc::c_int != 0) {
                        if (*psTile).tileInfoBits as libc::c_int &
                               0x1 as libc::c_int != 0 {
                            // structure on the build location - see if it is the same type
                            psStruct =
                                getTileStructure(((*psDroid).orderX as
                                                      libc::c_int >>
                                                      7 as libc::c_int) as
                                                     UDWORD,
                                                 ((*psDroid).orderY as
                                                      libc::c_int >>
                                                      7 as libc::c_int) as
                                                     UDWORD);
                            if (*psStruct).pStructureType ==
                                   (*psDroid).psTarStats as
                                       *mut STRUCTURE_STATS {
                                // same type - do a help build
                                (*psDroid).psTarget =
                                    psStruct as *mut BASE_OBJECT;
                                bDoHelpBuild = 1 as libc::c_int
                            } else if ((*(*psStruct).pStructureType).type_0 ==
                                           REF_WALL as libc::c_int as
                                               libc::c_uint ||
                                           (*(*psStruct).pStructureType).type_0
                                               ==
                                               REF_WALLCORNER as libc::c_int
                                                   as libc::c_uint) &&
                                          (*((*psDroid).psTarStats as
                                                 *mut STRUCTURE_STATS)).type_0
                                              ==
                                              REF_DEFENSE as libc::c_int as
                                                  libc::c_uint {
                                // building a gun tower over a wall - OK
                                if droidStartBuild(psDroid) != 0 {
                                    (*psDroid).action =
                                        DACTION_BUILD as libc::c_int
                                } else {
                                    (*psDroid).action =
                                        DACTION_NONE as libc::c_int
                                }
                            } else {
                                (*psDroid).action =
                                    DACTION_NONE as libc::c_int
                            }
                        } else {
                            (*psDroid).action = DACTION_NONE as libc::c_int
                        }
                    } else if droidStartBuild(psDroid) != 0 {
                        (*psDroid).action = DACTION_BUILD as libc::c_int;
                        intBuildStarted(psDroid);
                    } else { (*psDroid).action = DACTION_NONE as libc::c_int }
                } else { bDoHelpBuild = 1 as libc::c_int }
                if bDoHelpBuild != 0 {
                    // continuing a partially built structure (order = helpBuild)
                    if droidStartBuild(psDroid) != 0 {
                        (*psDroid).action = DACTION_BUILD as libc::c_int;
                        intBuildStarted(psDroid);
                    } else { (*psDroid).action = DACTION_NONE as libc::c_int }
                }
            } else if (*psDroid).sMove.Status as libc::c_int ==
                          0 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              8 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              12 as libc::c_int {
                if actionDroidOnBuildPos(psDroid, (*psDroid).orderX as SDWORD,
                                         (*psDroid).orderY as SDWORD,
                                         (*psDroid).psTarStats) != 0 {
                    actionHomeBasePos((*psDroid).player as SDWORD, &mut pbx,
                                      &mut pby);
                    moveDroidToNoFormation(psDroid, pbx as UDWORD,
                                           pby as UDWORD);
                } else {
                    moveDroidToNoFormation(psDroid, (*psDroid).actionX,
                                           (*psDroid).actionY);
                }
            }
        }
        2 => {
            // The droid cannot be in a formation
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                    (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                    ||
                    (*psDroid).sMove.Status as libc::c_int ==
                        12 as libc::c_int) &&
                   actionReachedBuildPos(psDroid, (*psDroid).orderX as SDWORD,
                                         (*psDroid).orderY as SDWORD,
                                         (*psDroid).psTarStats) == 0 {
                //			psDroid->action = DACTION_MOVETOBUILD;
                moveDroidToNoFormation(psDroid, (*psDroid).actionX,
                                       (*psDroid).actionY);
            } else if !((*psDroid).sMove.Status as libc::c_int ==
                            0 as libc::c_int ||
                            (*psDroid).sMove.Status as libc::c_int ==
                                8 as libc::c_int ||
                            (*psDroid).sMove.Status as libc::c_int ==
                                12 as libc::c_int) &&
                          (*psDroid).sMove.Status as libc::c_int !=
                              6 as libc::c_int &&
                          (*psDroid).sMove.Status as libc::c_int !=
                              12 as libc::c_int &&
                          actionReachedBuildPos(psDroid,
                                                (*psDroid).orderX as SDWORD,
                                                (*psDroid).orderY as SDWORD,
                                                (*psDroid).psTarStats) != 0 {
                moveStopDroid(psDroid);
            }
            if droidUpdateBuild(psDroid) != 0 {
                /*			if ( (psDroid->psTarget) && (psDroid->sMove.Status != MOVESHUFFLE) )
			{
		   		moveTurnDroid(psDroid,psDroid->psTarget->x,psDroid->psTarget->y);
			}*/
                actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                   (*psDroid).psActionTarget,
                                   &mut (*psDroid).turretRotation,
                                   &mut (*psDroid).turretPitch,
                                   0 as *mut WEAPON_STATS, 0 as libc::c_int);
            } else { (*psDroid).action = DACTION_NONE as libc::c_int }
        }
        19 | 20 | 31 | 30 => {
            // The droid cannot be in a formation
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            // see if the droid is at the edge of what it is moving to
            if actionReachedBuildPos(psDroid, (*psDroid).actionX as SDWORD,
                                     (*psDroid).actionY as SDWORD,
                                     (*psDroid).psTarStats) != 0 &&
                   actionDroidOnBuildPos(psDroid,
                                         (*psDroid).actionX as SDWORD,
                                         (*psDroid).actionY as SDWORD,
                                         (*psDroid).psTarStats) == 0 {
                moveStopDroid(psDroid);
                // got to the edge - start doing whatever it was meant to do
                match (*psDroid).action {
                    19 => {
                        (*psDroid).action =
                            if droidStartDemolishing(psDroid) != 0 {
                                DACTION_DEMOLISH as libc::c_int
                            } else { DACTION_NONE as libc::c_int }
                    }
                    20 => {
                        (*psDroid).action =
                            if droidStartRepair(psDroid) != 0 {
                                DACTION_REPAIR as libc::c_int
                            } else { DACTION_NONE as libc::c_int }
                    }
                    31 => {
                        (*psDroid).action =
                            if droidStartClearing(psDroid) != 0 {
                                DACTION_CLEARWRECK as libc::c_int
                            } else { DACTION_NONE as libc::c_int }
                    }
                    30 => {
                        (*psDroid).action =
                            if droidStartRestore(psDroid) != 0 {
                                DACTION_RESTORE as libc::c_int
                            } else { DACTION_NONE as libc::c_int }
                    }
                    _ => { }
                }
            } else if (*psDroid).sMove.Status as libc::c_int ==
                          0 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              8 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              12 as libc::c_int {
                if actionDroidOnBuildPos(psDroid,
                                         (*psDroid).actionX as SDWORD,
                                         (*psDroid).actionY as SDWORD,
                                         (*psDroid).psTarStats) != 0 {
                    actionHomeBasePos((*psDroid).player as SDWORD, &mut pbx,
                                      &mut pby);
                    moveDroidToNoFormation(psDroid, pbx as UDWORD,
                                           pby as UDWORD);
                } else {
                    moveDroidToNoFormation(psDroid, (*psDroid).actionX,
                                           (*psDroid).actionY);
                }
            }
        }
        4 | 5 | 16 | 15 => {
            // set up for the specific action
            match (*psDroid).action {
                4 => {
                    moveAction = DACTION_MOVETODEMOLISH as libc::c_int;
                    actionUpdateFunc =
                        Some(droidUpdateDemolishing as
                                 unsafe extern "C" fn(_: *mut DROID) -> BOOL)
                }
                5 => {
                    moveAction = DACTION_MOVETOREPAIR as libc::c_int;
                    actionUpdateFunc =
                        Some(droidUpdateRepair as
                                 unsafe extern "C" fn(_: *mut DROID) -> BOOL)
                }
                16 => {
                    moveAction = DACTION_MOVETOCLEAR as libc::c_int;
                    actionUpdateFunc =
                        Some(droidUpdateClearing as
                                 unsafe extern "C" fn(_: *mut DROID) -> BOOL)
                }
                15 => {
                    moveAction = DACTION_MOVETORESTORE as libc::c_int;
                    actionUpdateFunc =
                        Some(droidUpdateRestore as
                                 unsafe extern "C" fn(_: *mut DROID) -> BOOL)
                }
                _ => { }
            }
            // now do the action update
            if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                    (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                    ||
                    (*psDroid).sMove.Status as libc::c_int ==
                        12 as libc::c_int) &&
                   actionReachedBuildPos(psDroid,
                                         (*psDroid).actionX as SDWORD,
                                         (*psDroid).actionY as SDWORD,
                                         (*psDroid).psTarStats) == 0 {
                moveDroidToNoFormation(psDroid, (*psDroid).actionX,
                                       (*psDroid).actionY);
            } else if !((*psDroid).sMove.Status as libc::c_int ==
                            0 as libc::c_int ||
                            (*psDroid).sMove.Status as libc::c_int ==
                                8 as libc::c_int ||
                            (*psDroid).sMove.Status as libc::c_int ==
                                12 as libc::c_int) &&
                          (*psDroid).sMove.Status as libc::c_int !=
                              6 as libc::c_int &&
                          (*psDroid).sMove.Status as libc::c_int !=
                              12 as libc::c_int &&
                          actionReachedBuildPos(psDroid,
                                                (*psDroid).actionX as SDWORD,
                                                (*psDroid).actionY as SDWORD,
                                                (*psDroid).psTarStats) != 0 {
                moveStopDroid(psDroid);
            } else if actionUpdateFunc.expect("non-null function pointer")(psDroid)
                          != 0 {
                actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                   (*psDroid).psActionTarget,
                                   &mut (*psDroid).turretRotation,
                                   &mut (*psDroid).turretPitch,
                                   0 as *mut WEAPON_STATS, 0 as libc::c_int);
            } else if (*psDroid).action == DACTION_CLEARWRECK as libc::c_int {
                //see if there is any other wreckage in the area
                (*psDroid).action = DACTION_NONE as libc::c_int;
                psNextWreck = checkForWreckage(psDroid);
                if !psNextWreck.is_null() {
                    orderDroidObj(psDroid, DORDER_CLEARWRECK,
                                  psNextWreck as *mut BASE_OBJECT);
                }
            } else { (*psDroid).action = DACTION_NONE as libc::c_int }
        }
        34 => {
            /* moving to rearm pad */
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                (*psDroid).action = DACTION_WAITDURINGREARM as libc::c_int
            }
        }
        27 => {
            // don't want to be in a formation for this move
            if !(*psDroid).sMove.psFormation.is_null() {
                formationLeave((*psDroid).sMove.psFormation,
                               psDroid as *mut BASE_OBJECT);
                (*psDroid).sMove.psFormation = 0 as *mut _formation
            }
            /* moving from front to rear of repair facility or rearm pad */
/*		xdiff = (SDWORD)psDroid->x - ((SDWORD)psDroid->psActionTarget->x + TILE_UNITS);
		ydiff = (SDWORD)psDroid->y - (SDWORD)psDroid->psActionTarget->y;
		if (xdiff*xdiff + ydiff*ydiff < (TILE_UNITS/2)*(TILE_UNITS/2))*/
            if actionReachedBuildPos(psDroid,
                                     (*(*psDroid).psActionTarget).x as SDWORD,
                                     (*(*psDroid).psActionTarget).y as SDWORD,
                                     (*((*psDroid).psActionTarget as
                                            *mut STRUCTURE)).pStructureType as
                                         *mut BASE_STATS) != 0 {
                moveStopDroid(psDroid);
                (*psDroid).action = DACTION_WAITDURINGREPAIR as libc::c_int
            } else if (*psDroid).sMove.Status as libc::c_int ==
                          0 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              8 as libc::c_int ||
                          (*psDroid).sMove.Status as libc::c_int ==
                              12 as libc::c_int {
                moveDroidToNoFormation(psDroid,
                                       (*(*psDroid).psActionTarget).x as
                                           UDWORD,
                                       (*(*psDroid).psActionTarget).y as
                                           UDWORD);
            }
        }
        3 => {
            //building a structure's foundation - flattening the ground for now
            if !(droidUpdateFoundation(psDroid) != 0) {
                psTile =
                    mapTile(((*psDroid).orderX as libc::c_int >>
                                 7 as libc::c_int) as UDWORD,
                            ((*psDroid).orderY as libc::c_int >>
                                 7 as libc::c_int) as UDWORD);
                psStructStats = (*psDroid).psTarStats as *mut STRUCTURE_STATS;
                tlx =
                    ((*psDroid).orderX as SDWORD -
                         (*psStructStats).baseWidth.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                             as SDWORD / 2 as libc::c_int) as UDWORD;
                tly =
                    ((*psDroid).orderY as SDWORD -
                         (*psStructStats).baseBreadth.wrapping_mul(128 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                             as SDWORD / 2 as libc::c_int) as UDWORD;
                if (*psDroid).psTarget.is_null() &&
                       ((*psTile).tileInfoBits as libc::c_int &
                            0x1 as libc::c_int != 0 ||
                            (*psTile).tileInfoBits as libc::c_int &
                                0x2 as libc::c_int != 0) {
                    if (*psTile).tileInfoBits as libc::c_int &
                           0x1 as libc::c_int != 0 {
                        // structure on the build location - see if it is the same type
                        psStruct =
                            getTileStructure(((*psDroid).orderX as libc::c_int
                                                  >> 7 as libc::c_int) as
                                                 UDWORD,
                                             ((*psDroid).orderY as libc::c_int
                                                  >> 7 as libc::c_int) as
                                                 UDWORD);
                        if (*psStruct).pStructureType ==
                               (*psDroid).psTarStats as *mut STRUCTURE_STATS {
                            // same type - do a help build
                            (*psDroid).psTarget = psStruct as *mut BASE_OBJECT
                        } else {
                            (*psDroid).action = DACTION_NONE as libc::c_int
                        }
                    } else if validLocation((*psDroid).psTarStats as
                                                *mut BASE_STATS,
                                            tlx >> 7 as libc::c_int,
                                            tly >> 7 as libc::c_int,
                                            (*psDroid).player as UDWORD,
                                            0 as libc::c_int) == 0 {
                        (*psDroid).action = DACTION_NONE as libc::c_int
                    }
                }
                //ready to start building the structure
                if (*psDroid).action != DACTION_NONE as libc::c_int &&
                       droidStartBuild(psDroid) != 0 {
                    (*psDroid).action = DACTION_BUILD as libc::c_int
                    //add one to current quantity for this player - done in droidStartBuild()
				//asStructLimits[psDroid->player][(STRUCTURE_STATS *)psDroid->
					//psTarStats - asStructureStats].currentQuantity++;
                } else { (*psDroid).action = DACTION_NONE as libc::c_int }
            }
        }
        7 => {
            // align the turret
            actionTargetTurret(psDroid as *mut BASE_OBJECT,
                               (*psDroid).psActionTarget,
                               &mut (*psDroid).turretRotation,
                               &mut (*psDroid).turretPitch,
                               0 as *mut WEAPON_STATS, 0 as libc::c_int);
            if cbSensorDroid(psDroid) != 0 {
                // don't move to the target, just make sure it is visible
			// Anyone commenting this out will get a knee capping from John.
			// You have been warned!!
                (*(*psDroid).psActionTarget).visible[(*psDroid).player as
                                                         usize] =
                    0xff as libc::c_int as UBYTE
            } else {
                // make sure the target is within sensor range
                xdiff =
                    (*psDroid).x as SDWORD -
                        (*(*psDroid).psActionTarget).x as SDWORD;
                ydiff =
                    (*psDroid).y as SDWORD -
                        (*(*psDroid).psActionTarget).y as SDWORD;
                //if change this back - change in MOVETOOBSERVE as well
			//rangeSq = 2 * (SDWORD)psDroid->sensorRange / 3;
                rangeSq = (*psDroid).sensorRange as SDWORD;
                rangeSq = rangeSq * rangeSq;
                if visibleObject(psDroid as *mut BASE_OBJECT,
                                 (*psDroid).psActionTarget) == 0 ||
                       xdiff * xdiff + ydiff * ydiff >= rangeSq {
                    /*				if (secondaryGetState(psDroid, DSO_HALTTYPE, &state) && (state == DSS_HALT_HOLD))
				{
					psDroid->action = DACTION_NONE;						// holding, don't move.
				}
				else*/
                    (*psDroid).action = DACTION_MOVETOOBSERVE as libc::c_int;
                    moveDroidTo(psDroid,
                                (*(*psDroid).psActionTarget).x as UDWORD,
                                (*(*psDroid).psActionTarget).y as UDWORD);
                }
            }
        }
        25 => {
            // align the turret
            actionTargetTurret(psDroid as *mut BASE_OBJECT,
                               (*psDroid).psActionTarget,
                               &mut (*psDroid).turretRotation,
                               &mut (*psDroid).turretPitch,
                               0 as *mut WEAPON_STATS, 0 as libc::c_int);
            if visibleObject(psDroid as *mut BASE_OBJECT,
                             (*psDroid).psActionTarget) != 0 {
                // make sure the target is within sensor range
                xdiff =
                    (*psDroid).x as SDWORD -
                        (*(*psDroid).psActionTarget).x as SDWORD;
                ydiff =
                    (*psDroid).y as SDWORD -
                        (*(*psDroid).psActionTarget).y as SDWORD;
                //if change this back - change in OBSERVE as well
			//rangeSq = 2 * (SDWORD)psDroid->sensorRange / 3;
                rangeSq = (*psDroid).sensorRange as SDWORD;
                rangeSq = rangeSq * rangeSq;
                if xdiff * xdiff + ydiff * ydiff < rangeSq &&
                       !((*psDroid).sMove.Status as libc::c_int ==
                             0 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 8 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 12 as libc::c_int) {
                    (*psDroid).action = DACTION_OBSERVE as libc::c_int;
                    moveStopDroid(psDroid);
                }
            }
            if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                    (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                    ||
                    (*psDroid).sMove.Status as libc::c_int ==
                        12 as libc::c_int) &&
                   (*psDroid).action == DACTION_MOVETOOBSERVE as libc::c_int {
                /*			if (secondaryGetState(psDroid, DSO_HALTTYPE, &state) && (state == DSS_HALT_HOLD))
			{
				psDroid->action = DACTION_NONE;				// on hold, don't go any further.
			}
			else*/
                moveDroidTo(psDroid, (*(*psDroid).psActionTarget).x as UDWORD,
                            (*(*psDroid).psActionTarget).y as UDWORD);
            }
        }
        8 => {
            //can be either a droid or a structure now - AB 7/10/98
            if ((*(*psDroid).psTarget).type_0 as libc::c_uint ==
                    OBJ_DROID as libc::c_int as libc::c_uint ||
                    (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                        OBJ_STRUCTURE as libc::c_int as libc::c_uint) &&
                   (*(*psDroid).psTarget).player as libc::c_int ==
                       (*psDroid).player as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"DACTION_FIRESUPPORT: incorrect target type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if ((*(*psDroid).psTarget).type_0 as libc::c_uint ==
                    OBJ_DROID as libc::c_int as libc::c_uint ||
                    (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                        OBJ_STRUCTURE as libc::c_int as libc::c_uint) &&
                   (*(*psDroid).psTarget).player as libc::c_int ==
                       (*psDroid).player as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2223 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
                      b"(psDroid->psTarget->type == OBJ_DROID OR psDroid->psTarget->type == OBJ_STRUCTURE) && (psDroid->psTarget->player == psDroid->player)\x00"
                          as *const u8 as *const libc::c_char);
            };
            /*		if (orderState(((DROID *)psDroid->psTarget), DORDER_OBSERVE))
		{
			// move to attack
			psDroid->action = DACTION_MOVETOFSUPP_ATTACK;
			psDroid->psActionTarget = psDroid->psTarget->psTarget;
			moveDroidTo(psDroid, psDroid->psActionTarget->x, psDroid->psActionTarget->y);
		}
		else
		{*/
		//Move droids attached to structures and droids now...AB 13/10/98
		//move (indirect weapon)droids attached to a sensor
		//if (psDroid->psTarget->type == OBJ_DROID)
            //don't move VTOL's
			// also don't move closer to sensor towers
            if vtolDroid(psDroid) == 0 &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint !=
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                //move droids to within short range of the sensor now!!!!
                xdiff =
                    (*psDroid).x as SDWORD -
                        (*(*psDroid).psTarget).x as SDWORD;
                ydiff =
                    (*psDroid).y as SDWORD -
                        (*(*psDroid).psTarget).y as SDWORD;
                // make sure the weapon droid is within 2/3 weapon range of the sensor
			    //rangeSq = 2 * proj_GetLongRange(asWeaponStats + psDroid->asWeaps[0].nStat, 0) / 3;
                rangeSq =
                    (*asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int
                                                                  as
                                                                  usize].nStat
                                               as isize)).shortRange as
                        SDWORD;
                rangeSq = rangeSq * rangeSq;
                if xdiff * xdiff + ydiff * ydiff < rangeSq {
                    if !((*psDroid).sMove.Status as libc::c_int ==
                             0 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 8 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 12 as libc::c_int) {
                        moveStopDroid(psDroid);
                    }
                } else {
                    if !((*psDroid).sMove.Status as libc::c_int ==
                             0 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 8 as libc::c_int ||
                             (*psDroid).sMove.Status as libc::c_int ==
                                 12 as libc::c_int) {
                        xdiff =
                            (*(*psDroid).psTarget).x as SDWORD -
                                (*psDroid).sMove.DestinationX;
                        ydiff =
                            (*(*psDroid).psTarget).y as SDWORD -
                                (*psDroid).sMove.DestinationY
                    }
                    if (*psDroid).sMove.Status as libc::c_int ==
                           0 as libc::c_int ||
                           (*psDroid).sMove.Status as libc::c_int ==
                               8 as libc::c_int ||
                           (*psDroid).sMove.Status as libc::c_int ==
                               12 as libc::c_int ||
                           xdiff * xdiff + ydiff * ydiff > rangeSq {
                        if secondaryGetState(psDroid, DSO_HALTTYPE,
                                             &mut state) != 0 &&
                               state as libc::c_uint ==
                                   DSS_HALT_HOLD as libc::c_int as
                                       libc::c_uint {
                            // droid on hold, don't allow moves.
                            (*psDroid).action = DACTION_NONE as libc::c_int
                        } else {
                            // move in range
                            moveDroidTo(psDroid,
                                        (*(*psDroid).psTarget).x as UDWORD,
                                        (*(*psDroid).psTarget).y as UDWORD);
                        }
                    }
                }
            }
        }
        10 => {
            if (*psDroid).actionStarted.wrapping_add(2000 as libc::c_int as
                                                         libc::c_uint) <
                   gameTime {
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_PERSON as libc::c_int as libc::c_uint {
                    droidBurn(psDroid);
                } else { destroyDroid(psDroid); }
            }
        }
        29 => {
            // moving to repair a droid
            xdiff =
                (*psDroid).x as SDWORD -
                    (*(*psDroid).psActionTarget).x as SDWORD;
            ydiff =
                (*psDroid).y as SDWORD -
                    (*(*psDroid).psActionTarget).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff <
                   128 as libc::c_int * 128 as libc::c_int * 4 as libc::c_int
               {
                // Got to destination - start repair
			//rotate turret to point at droid being repaired
			/*if (actionTargetTurret((BASE_OBJECT*)psDroid,
					psDroid->psActionTarget, &(psDroid->turretRotation),
					&(psDroid->turretPitch), psDroid->turretRotRate,
					(SWORD)(psDroid->turretRotRate/2), FALSE, FALSE))*/
                if actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                      (*psDroid).psActionTarget,
                                      &mut (*psDroid).turretRotation,
                                      &mut (*psDroid).turretPitch,
                                      0 as *mut WEAPON_STATS,
                                      0 as libc::c_int) != 0 {
                    if droidStartDroidRepair(psDroid) != 0 {
                        (*psDroid).action = DACTION_DROIDREPAIR as libc::c_int
                    } else { (*psDroid).action = DACTION_NONE as libc::c_int }
                }
            }
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                // Couldn't reach destination - try and find a new one
			/*droidX = psDroid->psTarget->x >> TILE_SHIFT;
			droidY = psDroid->psTarget->y >> TILE_SHIFT;
			if (!pickATile(&droidX, &droidY, LOOK_NEXT_TO_DROID))
			{
				//couldn't get next to the droid, so stop trying
				psDroid->action = DACTION_NONE;
			}
			psDroid->actionX = droidX << TILE_SHIFT;
			psDroid->actionY = droidY << TILE_SHIFT;*/
                (*psDroid).actionX = (*(*psDroid).psActionTarget).x as UDWORD;
                (*psDroid).actionY = (*(*psDroid).psActionTarget).y as UDWORD;
                moveDroidTo(psDroid, (*psDroid).actionX, (*psDroid).actionY);
            }
        }
        14 => {
            /*actionTargetTurret((BASE_OBJECT*)psDroid,
					psDroid->psActionTarget, &(psDroid->turretRotation),
					&(psDroid->turretPitch), psDroid->turretRotRate,
					(SWORD)(psDroid->turretRotRate/2), FALSE, FALSE);*/
            //if not doing self-repair
            if (*psDroid).psActionTarget != psDroid as *mut BASE_OBJECT {
                actionTargetTurret(psDroid as *mut BASE_OBJECT,
                                   (*psDroid).psActionTarget,
                                   &mut (*psDroid).turretRotation,
                                   &mut (*psDroid).turretPitch,
                                   0 as *mut WEAPON_STATS, 0 as libc::c_int);
            }
            //check still next to the damaged droid
            xdiff =
                (*psDroid).x as SDWORD -
                    (*(*psDroid).psActionTarget).x as SDWORD;
            ydiff =
                (*psDroid).y as SDWORD -
                    (*(*psDroid).psActionTarget).y as SDWORD;
            if xdiff * xdiff + ydiff * ydiff >
                   128 as libc::c_int * 128 as libc::c_int * 4 as libc::c_int
               {
                /*once started - don't allow the Repair droid to follow the
			damaged droid for too long*/
			/*if (psDroid->actionPoints)
			{
				if (gameTime - psDroid->actionStarted > KEEP_TRYING_REPAIR)
				{
					addConsoleMessage("Repair Droid has given up!",DEFAULT_JUSTIFY);
					psDroid->action = DACTION_NONE;
					break;
				}
			}*/
			// damaged droid has moved off - follow!
                (*psDroid).actionX = (*(*psDroid).psActionTarget).x as UDWORD;
                (*psDroid).actionY = (*(*psDroid).psActionTarget).y as UDWORD;
                (*psDroid).action = DACTION_MOVETODROIDREPAIR as libc::c_int;
                moveDroidTo(psDroid, (*psDroid).actionX, (*psDroid).actionY);
            } else if droidUpdateDroidRepair(psDroid) == 0 {
                (*psDroid).action = DACTION_NONE as libc::c_int;
                //if the order is RTR then resubmit order so that the unit will go to repair facility point
                if orderState(psDroid, DORDER_RTR) != 0 {
                    orderDroid(psDroid, DORDER_RTR);
                }
            } else if (*((*psDroid).psActionTarget as
                             *mut DROID)).sMove.Status as libc::c_int ==
                          12 as libc::c_int {
                moveStopDroid((*psDroid).psActionTarget as *mut DROID);
            }
        }
        33 => {
            // don't let the target for a repair shuffle
            // wait here for the rearm pad to ask the vtol to move
            if (*psDroid).psActionTarget.is_null() {
                // rearm pad destroyed - move to another
                moveToRearm(psDroid);
            } else if ((*psDroid).sMove.Status as libc::c_int ==
                           0 as libc::c_int ||
                           (*psDroid).sMove.Status as libc::c_int ==
                               8 as libc::c_int ||
                           (*psDroid).sMove.Status as libc::c_int ==
                               12 as libc::c_int) && vtolHappy(psDroid) != 0 {
                // don't actually need to rearm so just sit next to the rearm pad
                (*psDroid).action = DACTION_NONE as libc::c_int
            }
        }
        37 => {
            if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                   (*psDroid).sMove.Status as libc::c_int == 8 as libc::c_int
                   ||
                   (*psDroid).sMove.Status as libc::c_int == 12 as libc::c_int
               {
                (*psDroid).action = DACTION_NONE as libc::c_int
            }
        }
        35 => { }
        32 => {
            if (*psDroid).psActionTarget.is_null() {
                // base destroyed - find another
                moveToRearm(psDroid);
            } else {
                if visibleObject(psDroid as *mut BASE_OBJECT,
                                 (*psDroid).psActionTarget) != 0 {
                    // got close to the rearm pad - now find a clear one
                    psStruct =
                        findNearestReArmPad(psDroid,
                                            (*psDroid).psActionTarget as
                                                *mut STRUCTURE,
                                            1 as libc::c_int);
                    if !psStruct.is_null() {
                        // found a clear landing pad - go for it
                        (*psDroid).psActionTarget =
                            psStruct as *mut BASE_OBJECT
                    }
                    (*psDroid).action = DACTION_WAITFORREARM as libc::c_int
                }
                if (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
                       ||
                       (*psDroid).sMove.Status as libc::c_int ==
                           8 as libc::c_int ||
                       (*psDroid).sMove.Status as libc::c_int ==
                           12 as libc::c_int ||
                       (*psDroid).action ==
                           DACTION_WAITFORREARM as libc::c_int {
                    droidX = (*(*psDroid).psActionTarget).x as UDWORD;
                    droidY = (*(*psDroid).psActionTarget).y as UDWORD;
                    if actionVTOLLandingPos(psDroid, &mut droidX, &mut droidY)
                           == 0 {
                        // totally bunged up - give up
                        debug(LOG_NEVER,
                              b"DACTION_MOVETOREARM: couldn\'t find a clear tile near rearm pad - RTB\n\x00"
                                  as *const u8 as *const libc::c_char);
                        orderDroid(psDroid, DORDER_RTB);
                    } else { moveDroidToDirect(psDroid, droidX, droidY); }
                }
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"actionUpdateUnit: unknown action\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2458 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"actionUpdateDroid\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    if (*psDroid).action != DACTION_MOVEFIRE as libc::c_int &&
           (*psDroid).action != DACTION_ATTACK as libc::c_int &&
           (*psDroid).action != DACTION_MOVETOATTACK as libc::c_int &&
           (*psDroid).action != DACTION_MOVETODROIDREPAIR as libc::c_int &&
           (*psDroid).action != DACTION_DROIDREPAIR as libc::c_int &&
           (*psDroid).action != DACTION_BUILD as libc::c_int &&
           (*psDroid).action != DACTION_OBSERVE as libc::c_int &&
           (*psDroid).action != DACTION_MOVETOOBSERVE as libc::c_int {
        if (*psDroid).turretRotation as libc::c_int != 0 as libc::c_int ||
               (*psDroid).turretPitch as libc::c_int != 0 as libc::c_int {
            actionAlignTurret(psDroid as *mut BASE_OBJECT);
        }
    };
}
/* Overall action function that is called by the specific action functions */
unsafe extern "C" fn actionDroidBase(mut psDroid: *mut DROID,
                                     mut psAction: *mut DROID_ACTION_DATA) {
    let mut state: SECONDARY_STATE = 0 as SECONDARY_STATE;
    let mut pbx: SDWORD = 0;
    let mut pby: SDWORD = 0;
    let mut psWeapStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"actionUnitBase: Invalid Unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              2491 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDroid).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"actionUnitBase: Unit pointer does not reference a unit\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psDroid).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"action.c\x00" as *const u8 as *const libc::c_char,
              2493 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
              b"psDroid->type == OBJ_DROID\x00" as *const u8 as
                  *const libc::c_char);
    };
    let mut current_block_184: u64;
    match (*psAction).action as libc::c_uint {
        0 => {
            // Clear up what ever the droid was doing before if necessary
//		if(!driveModeActive() || !psDroid->selected) {
            if !((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int ||
                     (*psDroid).sMove.Status as libc::c_int ==
                         8 as libc::c_int ||
                     (*psDroid).sMove.Status as libc::c_int ==
                         12 as libc::c_int) {
                moveStopDroid(psDroid);
            }
            (*psDroid).action = DACTION_NONE as libc::c_int;
            (*psDroid).actionX = 0 as libc::c_int as UDWORD;
            (*psDroid).actionY = 0 as libc::c_int as UDWORD;
            (*psDroid).actionStarted = 0 as libc::c_int as UDWORD;
            (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
            //psDroid->actionHeight = 0;
            (*psDroid).powerAccrued = 0 as libc::c_int as UWORD;
            (*psDroid).psActionTarget = 0 as *mut _base_object
        }
        12 => {
            (*psDroid).action = DACTION_TRANSPORTWAITTOFLYIN as libc::c_int
        }
        6 => {
            // can't attack without a weapon
		// or yourself
            if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
                   0 as libc::c_int as libc::c_uint ||
                   (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
                   (*psAction).psObj == psDroid as *mut BASE_OBJECT {
                return
            }
            //check electronic droids only attack structures - not so anymore!
            if electronicDroid(psDroid) != 0 {
                //check for low or zero resistance - just zero resistance!
                if (*(*psAction).psObj).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                       validStructResistance((*psAction).psObj as
                                                 *mut STRUCTURE) == 0 {
                    //structure is low resistance already so don't attack
                    (*psDroid).action = DACTION_NONE as libc::c_int;
                    current_block_184 = 1176253869785344635;
                } else if bMultiPlayer != 0 &&
                              (*(*psAction).psObj).type_0 as libc::c_uint ==
                                  OBJ_DROID as libc::c_int as libc::c_uint &&
                              (*((*psAction).psObj as *mut DROID)).droidType
                                  as libc::c_uint ==
                                  DROID_TRANSPORTER as libc::c_int as
                                      libc::c_uint {
                    (*psDroid).action = DACTION_NONE as libc::c_int;
                    current_block_184 = 1176253869785344635;
                } else { current_block_184 = 17788412896529399552; }
            } else { current_block_184 = 17788412896529399552; }
            match current_block_184 {
                1176253869785344635 => { }
                _ => {
                    //in multiPlayer cannot electronically attack a tranporter
                    //		psDroid->actionX = psAction->psObj->x;
//		psDroid->actionY = psAction->psObj->y;
		// note the droid's current pos so that scout & patrol orders know how far the
		// droid has gone during an attack
		// slightly strange place to store this I know, but I didn't want to add any more to the droid
                    (*psDroid).actionX =
                        (*psDroid).x as
                            UDWORD; // holding, try attack straightaway
                    (*psDroid).actionY = (*psDroid).y as UDWORD;
                    (*psDroid).psActionTarget = (*psAction).psObj;
                    if ((*psDroid).order == DORDER_ATTACKTARGET as libc::c_int
                            ||
                            (*psDroid).order ==
                                DORDER_FIRESUPPORT as libc::c_int) &&
                           secondaryGetState(psDroid, DSO_HALTTYPE,
                                             &mut state) != 0 &&
                           state as libc::c_uint ==
                               DSS_HALT_HOLD as libc::c_int as libc::c_uint ||
                           vtolDroid(psDroid) == 0 &&
                               orderStateObj(psDroid, DORDER_FIRESUPPORT,
                                             &mut psTarget) != 0 &&
                               (*psTarget).type_0 as libc::c_uint ==
                                   OBJ_STRUCTURE as libc::c_int as
                                       libc::c_uint {
                        (*psDroid).action = DACTION_ATTACK as libc::c_int;
                        (*psDroid).psActionTarget = (*psAction).psObj
                    } else if actionInsideMinRange(psDroid, (*psAction).psObj)
                                  != 0 {
                        psWeapStats =
                            &mut *asWeaponStats.offset((*(*psDroid).asWeaps.as_mut_ptr().offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize)).nStat
                                                           as isize) as
                                *mut WEAPON_STATS;
                        if proj_Direct(psWeapStats) == 0 {
                            if (*psWeapStats).rotate != 0 {
                                (*psDroid).action =
                                    DACTION_ATTACK as libc::c_int
                            } else {
                                (*psDroid).action =
                                    DACTION_ROTATETOATTACK as libc::c_int;
                                moveTurnDroid(psDroid,
                                              (*(*psDroid).psActionTarget).x
                                                  as UDWORD,
                                              (*(*psDroid).psActionTarget).y
                                                  as UDWORD);
                            }
                        } else {
                            /* direct fire - try and extend the range */
                            (*psDroid).action =
                                DACTION_MOVETOATTACK as libc::c_int;
                            actionCalcPullBackPoint(psDroid as
                                                        *mut BASE_OBJECT,
                                                    (*psAction).psObj,
                                                    &mut pbx, &mut pby);
                            turnOffMultiMsg(1 as libc::c_int);
                            moveDroidTo(psDroid, pbx as UDWORD,
                                        pby as UDWORD);
                            turnOffMultiMsg(0 as libc::c_int);
                        }
                    } else {
                        (*psDroid).action =
                            DACTION_MOVETOATTACK as libc::c_int;
                        turnOffMultiMsg(1 as libc::c_int);
                        moveDroidTo(psDroid, (*(*psAction).psObj).x as UDWORD,
                                    (*(*psAction).psObj).y as UDWORD);
                        turnOffMultiMsg(0 as libc::c_int);
                    }
                }
            }
        }
        32 => {
            (*psDroid).action = DACTION_MOVETOREARM as libc::c_int;
            (*psDroid).actionX = (*(*psAction).psObj).x as UDWORD;
            (*psDroid).actionY = (*(*psAction).psObj).y as UDWORD;
            (*psDroid).actionStarted = gameTime;
            (*psDroid).psActionTarget = (*psAction).psObj;
            droidX = (*(*psDroid).psActionTarget).x as UDWORD;
            droidY = (*(*psDroid).psActionTarget).y as UDWORD;
            if actionVTOLLandingPos(psDroid, &mut droidX, &mut droidY) == 0 {
                // totally bunged up - give up
                orderDroid(psDroid, DORDER_RTB);
            } else { moveDroidToDirect(psDroid, droidX, droidY); }
        }
        37 => {
            (*psDroid).action = DACTION_CLEARREARMPAD as libc::c_int;
            (*psDroid).psActionTarget = (*psAction).psObj;
            droidX = (*(*psDroid).psActionTarget).x as UDWORD;
            droidY = (*(*psDroid).psActionTarget).y as UDWORD;
            if actionVTOLLandingPos(psDroid, &mut droidX, &mut droidY) == 0 {
                // totally bunged up - give up
                orderDroid(psDroid, DORDER_RTB);
            } else { moveDroidToDirect(psDroid, droidX, droidY); }
        }
        1 | 13 | 11 | 38 | 39 => {
            (*psDroid).action = (*psAction).action as SDWORD;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            (*psDroid).actionStarted = gameTime;
            (*psDroid).psActionTarget = (*psAction).psObj;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        2 => {
            if (*psDroid).order == DORDER_BUILD as libc::c_int ||
                   (*psDroid).order == DORDER_HELPBUILD as libc::c_int ||
                   (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: cannot start build action without a build order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*psDroid).order == DORDER_BUILD as libc::c_int ||
                   (*psDroid).order == DORDER_HELPBUILD as libc::c_int ||
                   (*psDroid).order == DORDER_LINEBUILD as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2664 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"psDroid->order == DORDER_BUILD || psDroid->order == DORDER_HELPBUILD || psDroid->order == DORDER_LINEBUILD\x00"
                          as *const u8 as *const libc::c_char);
            };
            (*psDroid).action = DACTION_MOVETOBUILD as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            if actionDroidOnBuildPos(psDroid, (*psDroid).orderX as SDWORD,
                                     (*psDroid).orderY as SDWORD,
                                     (*psDroid).psTarStats) != 0 {
                actionHomeBasePos((*psDroid).player as SDWORD, &mut pbx,
                                  &mut pby);
                moveDroidToNoFormation(psDroid, pbx as UDWORD, pby as UDWORD);
            } else {
                moveDroidToNoFormation(psDroid, (*psDroid).actionX,
                                       (*psDroid).actionY);
            }
        }
        4 => {
            if (*psDroid).order == DORDER_DEMOLISH as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: cannot start demolish action without a demolish order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*psDroid).order == DORDER_DEMOLISH as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2681 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"psDroid->order == DORDER_DEMOLISH\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).action = DACTION_MOVETODEMOLISH as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: invalid target for demolish order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2686 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"(psDroid->psTarget != NULL) && (psDroid->psTarget->type == OBJ_STRUCTURE)\x00"
                          as *const u8 as *const libc::c_char);
            };
            (*psDroid).psTarStats =
                (*((*psDroid).psTarget as *mut STRUCTURE)).pStructureType as
                    *mut BASE_STATS;
            (*psDroid).psActionTarget = (*psAction).psObj;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        5 => {
            //ASSERT( psDroid->order == DORDER_REPAIR,
		//	"actionDroidBase: cannot start repair action without a repair order" );
            (*psDroid).action = DACTION_MOVETOREPAIR as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            //this needs setting so that automatic repair works
            (*psDroid).psActionTarget = (*psAction).psObj;
            if !(*psDroid).psActionTarget.is_null() &&
                   (*(*psDroid).psActionTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: invalid target for demolish order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !(*psDroid).psActionTarget.is_null() &&
                   (*(*psDroid).psActionTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2700 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"(psDroid->psActionTarget != NULL) && (psDroid->psActionTarget->type == OBJ_STRUCTURE)\x00"
                          as *const u8 as *const libc::c_char);
            };
            (*psDroid).psTarStats =
                (*((*psDroid).psActionTarget as
                       *mut STRUCTURE)).pStructureType as *mut BASE_STATS;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        7 => {
            (*psDroid).psActionTarget = (*psAction).psObj;
            (*psDroid).actionX = (*psDroid).x as UDWORD;
            (*psDroid).actionY = (*psDroid).y as UDWORD;
            if cbSensorDroid(psDroid) != 0 {
                (*psDroid).action = DACTION_OBSERVE as libc::c_int
            } else {
                (*psDroid).action = DACTION_MOVETOOBSERVE as libc::c_int;
                moveDroidTo(psDroid, (*(*psDroid).psActionTarget).x as UDWORD,
                            (*(*psDroid).psActionTarget).y as UDWORD);
            }
        }
        8 => {
            (*psDroid).action = DACTION_FIRESUPPORT as libc::c_int;
            if vtolDroid(psDroid) == 0 &&
                   !(secondaryGetState(psDroid, DSO_HALTTYPE, &mut state) != 0
                         &&
                         state as libc::c_uint ==
                             DSS_HALT_HOLD as libc::c_int as libc::c_uint) &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint !=
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                moveDroidTo(psDroid, (*(*psDroid).psTarget).x as UDWORD,
                            (*(*psDroid).psTarget).y as UDWORD);
                // movetotarget.
            }
        }
        9 => {
            (*psDroid).action = DACTION_SULK as libc::c_int;
            //hmmm, hope this doesn't cause any problems!
		//psDroid->actionStarted = gameTime;			// what is action started used for ? Certainly not used here!
		//psDroid->actionHeight = (UWORD)(gameTime+MIN_SULK_TIME+(rand()%(
		//	MAX_SULK_TIME-MIN_SULK_TIME)));	// actionHeight is used here for the ending time for this action
            (*psDroid).actionStarted =
                gameTime.wrapping_add(1500 as libc::c_int as
                                          libc::c_uint).wrapping_add((rand() %
                                                                          (4000
                                                                               as
                                                                               libc::c_int
                                                                               -
                                                                               1500
                                                                                   as
                                                                                   libc::c_int))
                                                                         as
                                                                         libc::c_uint)
        }
        10 => {
            (*psDroid).action = DACTION_DESTRUCT as libc::c_int;
            (*psDroid).actionStarted = gameTime
        }
        26 => {
            (*psDroid).action = DACTION_WAITFORREPAIR as libc::c_int;
            //set the time so we can tell whether the start the self repair or not
            (*psDroid).actionStarted = gameTime
        }
        27 => {
            (*psDroid).action = (*psAction).action as SDWORD;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            (*psDroid).actionStarted = gameTime;
            (*psDroid).psActionTarget = (*psAction).psObj;
            moveDroidToNoFormation(psDroid, (*psAction).x, (*psAction).y);
        }
        28 => { (*psDroid).action = DACTION_WAITDURINGREPAIR as libc::c_int }
        34 => {
            (*psDroid).action = (*psAction).action as SDWORD;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            (*psDroid).actionStarted = gameTime;
            (*psDroid).psActionTarget = (*psAction).psObj;
            moveDroidToDirect(psDroid, (*psAction).x, (*psAction).y);
            // make sure there arn't any other VTOLs on the rearm pad
            ensureRearmPadClear((*psAction).psObj as *mut STRUCTURE, psDroid);
        }
        14 => {
            //		ASSERT( psDroid->order == DORDER_DROIDREPAIR,
//			"actionDroidBase: cannot start droid repair action without a repair order" );
            (*psDroid).action = DACTION_MOVETODROIDREPAIR as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            (*psDroid).psActionTarget = (*psAction).psObj;
            //initialise the action points
            (*psDroid).actionPoints = 0 as libc::c_int as UDWORD;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        15 => {
            if (*psDroid).order == DORDER_RESTORE as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: cannot start restore action without a restore order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*psDroid).order == DORDER_RESTORE as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2784 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"psDroid->order == DORDER_RESTORE\x00" as *const u8 as
                          *const libc::c_char);
            };
            (*psDroid).action = DACTION_MOVETORESTORE as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: invalid target for restore order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2789 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"(psDroid->psTarget != NULL) && (psDroid->psTarget->type == OBJ_STRUCTURE)\x00"
                          as *const u8 as *const libc::c_char);
            };
            (*psDroid).psTarStats =
                (*((*psDroid).psTarget as *mut STRUCTURE)).pStructureType as
                    *mut BASE_STATS;
            (*psDroid).psActionTarget = (*psAction).psObj;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        16 => {
            if (*psDroid).order == DORDER_CLEARWRECK as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: cannot start clear action without a clear order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*psDroid).order == DORDER_CLEARWRECK as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2796 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"psDroid->order == DORDER_CLEARWRECK\x00" as *const u8
                          as *const libc::c_char);
            };
            (*psDroid).action = DACTION_MOVETOCLEAR as libc::c_int;
            (*psDroid).actionX = (*psAction).x;
            (*psDroid).actionY = (*psAction).y;
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_FEATURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: invalid target for demolish order\x00"
                          as *const u8 as *const libc::c_char);
            };
            if !(*psDroid).psTarget.is_null() &&
                   (*(*psDroid).psTarget).type_0 as libc::c_uint ==
                       OBJ_FEATURE as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2801 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"(psDroid->psTarget != NULL) && (psDroid->psTarget->type == OBJ_FEATURE)\x00"
                          as *const u8 as *const libc::c_char);
            };
            (*psDroid).psTarStats =
                (*((*psDroid).psTarget as *mut FEATURE)).psStats as
                    *mut BASE_STATS;
            (*psDroid).psActionTarget = (*psDroid).psTarget;
            moveDroidTo(psDroid, (*psAction).x, (*psAction).y);
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"actionUnitBase: unknown action\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"action.c\x00" as *const u8 as *const libc::c_char,
                      2807 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"actionDroidBase\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    };
}
/* Give a droid an action */
/* Give a droid an action */
#[no_mangle]
pub unsafe extern "C" fn actionDroid(mut psDroid: *mut DROID,
                                     mut action: DROID_ACTION) {
    let mut sAction: DROID_ACTION_DATA =
        DROID_ACTION_DATA{action: DACTION_NONE,
                          x: 0,
                          y: 0,
                          psObj: 0 as *mut BASE_OBJECT,
                          psStats: 0 as *mut BASE_STATS,};
    memset(&mut sAction as *mut DROID_ACTION_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ACTION_DATA>() as libc::c_ulong);
    sAction.action = action;
    actionDroidBase(psDroid, &mut sAction);
}
/* Give a droid an action with a location target */
/* Give a droid an action with a location target */
#[no_mangle]
pub unsafe extern "C" fn actionDroidLoc(mut psDroid: *mut DROID,
                                        mut action: DROID_ACTION,
                                        mut x: UDWORD, mut y: UDWORD) {
    let mut sAction: DROID_ACTION_DATA =
        DROID_ACTION_DATA{action: DACTION_NONE,
                          x: 0,
                          y: 0,
                          psObj: 0 as *mut BASE_OBJECT,
                          psStats: 0 as *mut BASE_STATS,};
    memset(&mut sAction as *mut DROID_ACTION_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ACTION_DATA>() as libc::c_ulong);
    sAction.action = action;
    sAction.x = x;
    sAction.y = y;
    actionDroidBase(psDroid, &mut sAction);
}
/* Give a droid an action with an object target */
/* Give a droid an action with an object target */
#[no_mangle]
pub unsafe extern "C" fn actionDroidObj(mut psDroid: *mut DROID,
                                        mut action: DROID_ACTION,
                                        mut psObj: *mut BASE_OBJECT) {
    let mut sAction: DROID_ACTION_DATA =
        DROID_ACTION_DATA{action: DACTION_NONE,
                          x: 0,
                          y: 0,
                          psObj: 0 as *mut BASE_OBJECT,
                          psStats: 0 as *mut BASE_STATS,};
    memset(&mut sAction as *mut DROID_ACTION_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ACTION_DATA>() as libc::c_ulong);
    sAction.action = action;
    sAction.psObj = psObj;
    sAction.x = (*psObj).x as UDWORD;
    sAction.y = (*psObj).y as UDWORD;
    actionDroidBase(psDroid, &mut sAction);
}
/* Give a droid an action with an object target and a location */
/* Give a droid an action with an object target and a location */
#[no_mangle]
pub unsafe extern "C" fn actionDroidObjLoc(mut psDroid: *mut DROID,
                                           mut action: DROID_ACTION,
                                           mut psObj: *mut BASE_OBJECT,
                                           mut x: UDWORD, mut y: UDWORD) {
    let mut sAction: DROID_ACTION_DATA =
        DROID_ACTION_DATA{action: DACTION_NONE,
                          x: 0,
                          y: 0,
                          psObj: 0 as *mut BASE_OBJECT,
                          psStats: 0 as *mut BASE_STATS,};
    memset(&mut sAction as *mut DROID_ACTION_DATA as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<DROID_ACTION_DATA>() as libc::c_ulong);
    sAction.action = action;
    sAction.psObj = psObj;
    sAction.x = x;
    sAction.y = y;
    actionDroidBase(psDroid, &mut sAction);
}
/*send the vtol droid back to the nearest rearming pad - if one otherwise
return to base*/
/*send the vtol droid back to the nearest rearming pad - if one otherwise
return to base*/
// IF YOU CHANGE THE ORDER/ACTION RESULTS TELL ALEXL!!!! && recvvtolrearm
#[no_mangle]
pub unsafe extern "C" fn moveToRearm(mut psDroid: *mut DROID) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut chosen: UBYTE = 0 as libc::c_int as UBYTE;
    // IF YOU CHANGE THE ORDER/ACTION RESULTS TELL ALEXL!!!! && recvvtolrearm
    if vtolDroid(psDroid) == 0 { return }
    //if droid is already returning - ignore
    if vtolRearming(psDroid) != 0 { return }
    //get the droid to fly back to a ReArming Pad
	// don't worry about finding a clear one for the minute
    psStruct =
        findNearestReArmPad(psDroid, (*psDroid).psBaseStruct,
                            0 as libc::c_int);
    if !psStruct.is_null() {
        // note a base rearm pad if the vtol doesn't have one
        if (*psDroid).psBaseStruct.is_null() {
            (*psDroid).psBaseStruct = psStruct
        }
        //return to re-arming pad
        if (*psDroid).order == DORDER_NONE as libc::c_int {
            // no order set - use the rearm order to ensure the unit goes back
			// to the landing pad
            orderDroidObj(psDroid, DORDER_REARM,
                          psStruct as *mut BASE_OBJECT);
            chosen = 1 as libc::c_int as UBYTE
        } else {
            actionDroidObj(psDroid, DACTION_MOVETOREARM,
                           psStruct as *mut BASE_OBJECT);
            chosen = 2 as libc::c_int as UBYTE
        }
    } else {
        //return to base un-armed
        orderDroid(psDroid, DORDER_RTB);
        chosen = 3 as libc::c_int as UBYTE
    }
    if bMultiPlayer != 0 { sendVtolRearm(psDroid, psStruct, chosen); };
}
// whether a tile is suitable for a vtol to land on
#[no_mangle]
pub unsafe extern "C" fn vtolLandingTile(mut x: SDWORD, mut y: SDWORD)
 -> BOOL {
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if x < 0 as libc::c_int || x >= mapWidth as SDWORD || y < 0 as libc::c_int
           || y >= mapHeight as SDWORD {
        return 0 as libc::c_int
    }
    psTile = mapTile(x as UDWORD, y as UDWORD);
    if (*psTile).tileInfoBits as libc::c_int & 0x10 as libc::c_int != 0 ||
           (*psTile).tileInfoBits as libc::c_int &
               (0x1 as libc::c_int | 0x2 as libc::c_int | 0x20 as libc::c_int)
               != 0 ||
           terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_CLIFFFACE as libc::c_int ||
           terrainTypes[((*psTile).texture as libc::c_int &
                             0x1ff as libc::c_int) as usize] as libc::c_int ==
               TER_WATER as libc::c_int {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// choose a landing position for a VTOL when it goes to rearm
// choose a landing position for a VTOL when it goes to rearm
// choose a landing position for a VTOL when it goes to rearm
#[no_mangle]
pub unsafe extern "C" fn actionVTOLLandingPos(mut psDroid: *mut DROID,
                                              mut px: *mut UDWORD,
                                              mut py: *mut UDWORD) -> BOOL {
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut passes: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut result: BOOL = 0;
    //	ASSERT( (psDroid->psActionTarget != NULL),
//		"actionVTOLLandingPos: no rearm pad set for the VTOL" );
    /* Initial box dimensions and set iteration count to zero */
//	startX = endX = (SDWORD)psDroid->psActionTarget->x >> TILE_SHIFT;
//	startY = endY = (SDWORD)psDroid->psActionTarget->y >> TILE_SHIFT;
    endX = *px as SDWORD >> 7 as libc::c_int;
    startX = endX;
    endY = *py as SDWORD >> 7 as libc::c_int;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    // set blocking flags for all the other droids
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if (*psCurr).sMove.Status as libc::c_int == 0 as libc::c_int ||
               (*psCurr).sMove.Status as libc::c_int == 8 as libc::c_int ||
               (*psCurr).sMove.Status as libc::c_int == 12 as libc::c_int {
            tx = (*psCurr).x as libc::c_int >> 7 as libc::c_int;
            ty = (*psCurr).y as libc::c_int >> 7 as libc::c_int
        } else {
            tx = (*psCurr).sMove.DestinationX >> 7 as libc::c_int;
            ty = (*psCurr).sMove.DestinationY >> 7 as libc::c_int
        }
        if psCurr != psDroid {
            if tileOnMap(tx, ty) != 0 {
                let ref mut fresh0 =
                    (*mapTile(tx as UDWORD, ty as UDWORD)).tileInfoBits;
                *fresh0 =
                    (*fresh0 as libc::c_int | 0x10 as libc::c_int) as UBYTE
            }
        }
        psCurr = (*psCurr).psNext
    }
    /* Keep going until we get a tile or we exceed distance */
    result = 0 as libc::c_int;
    's_104:
        while passes < 20 as libc::c_int as libc::c_uint {
            /* Process whole box */
            i = startX;
            while i <= endX {
                j = startY;
                while j <= endY {
                    /* Test only perimeter as internal tested previous iteration */
                    if i == startX || i == endX || j == startY || j == endY {
                        /* Good enough? */
                        if vtolLandingTile(i, j) != 0 {
                            /* Set exit conditions and get out NOW */
                            *px =
                                ((i << 7 as libc::c_int) +
                                     128 as libc::c_int / 2 as libc::c_int) as
                                    UDWORD;
                            *py =
                                ((j << 7 as libc::c_int) +
                                     128 as libc::c_int / 2 as libc::c_int) as
                                    UDWORD;
                            result = 1 as libc::c_int;
                            break 's_104 ;
                        }
                    }
                    j += 1
                }
                i += 1
            }
            /* Expand the box out in all directions - off map handled by tileAcceptable */
            startX -= 1;
            startY -= 1;
            endX += 1;
            endY += 1;
            passes = passes.wrapping_add(1)
        }
    // clear blocking flags for all the other droids
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if (*psCurr).sMove.Status as libc::c_int == 0 as libc::c_int ||
               (*psCurr).sMove.Status as libc::c_int == 8 as libc::c_int ||
               (*psCurr).sMove.Status as libc::c_int == 12 as libc::c_int {
            tx = (*psCurr).x as libc::c_int >> 7 as libc::c_int;
            ty = (*psCurr).y as libc::c_int >> 7 as libc::c_int
        } else {
            tx = (*psCurr).sMove.DestinationX >> 7 as libc::c_int;
            ty = (*psCurr).sMove.DestinationY >> 7 as libc::c_int
        }
        if tileOnMap(tx, ty) != 0 {
            let ref mut fresh1 =
                (*mapTile(tx as UDWORD, ty as UDWORD)).tileInfoBits;
            *fresh1 =
                (*fresh1 as libc::c_int & !(0x10 as libc::c_int)) as UBYTE
        }
        psCurr = (*psCurr).psNext
    }
    return result;
}
