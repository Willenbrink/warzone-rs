use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn rand() -> libc::c_int;
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
    fn pie_Draw3DShape(shape: *mut iIMDShape, frame: libc::c_int,
                       team: libc::c_int, colour: UDWORD, specular: UDWORD,
                       pieFlag: libc::c_int, pieData: libc::c_int);
    #[no_mangle]
    fn SetBSPObjectPos(x: SDWORD, y: SDWORD, z: SDWORD);
    #[no_mangle]
    fn SetBSPCameraPos(x: SDWORD, y: SDWORD, z: SDWORD);
    #[no_mangle]
    fn SetBSPObjectRot(Yaw: SDWORD, Pitch: SDWORD);
    #[no_mangle]
    fn droidSetBits(pTemplate: *mut DROID_TEMPLATE, psDroid: *mut DROID);
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
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
    #[no_mangle]
    fn pie_RotProj(v3d: *mut iVector, v2d: *mut iPoint) -> int32;
    //*************************************************************************
    #[no_mangle]
    fn pie_PerspectiveBegin();
    #[no_mangle]
    fn pie_PerspectiveEnd();
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn lightDoFogAndIllumination(brightness: UBYTE, dx: SDWORD, dz: SDWORD,
                                 pSpecular: *mut UDWORD) -> UDWORD;
    /* **************************************************************************/
    #[no_mangle]
    fn GetRealCameraPos(Camera: *mut OBJPOS, Distance: SDWORD,
                        CameraLoc: *mut iVector);
    #[no_mangle]
    fn gamePaused() -> BOOL;
    /*
 * Stats.h
 *
 * Interface to the common stats module
 *
 */
    /* *************************************************************************************
 *
 * Function prototypes and data storage for the stats
 */
    /* The stores for the different stats */
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    //extern POWER_STATS			*asPowerStats;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    //extern ARMOUR_STATS			*asArmourStats;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut asConstructStats: *mut CONSTRUCT_STATS;
    #[no_mangle]
    static mut godMode: BOOL;
    #[no_mangle]
    static mut terrainMidX: UDWORD;
    #[no_mangle]
    static mut terrainMidY: UDWORD;
    #[no_mangle]
    fn calcScreenCoords(psDroid: *mut DROID);
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    fn getCentreX() -> SDWORD;
    #[no_mangle]
    fn getCentreZ() -> SDWORD;
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
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn getStaticTimeValueRange(tickFrequency: UDWORD, requiredRange: UDWORD)
     -> UDWORD;
    /* Power levels are divided by this for power bar display. The extra factor has
been included so that the levels appear the same for the power bar as for the
power values in the buttons */
    // Speed to rotate objects rendered in
    // buttons ( degrees per second )
    //the two types of button used in the object display (bottom bar)
    // Bitmap buffer.
    // Ivis surface definition.
    // I tried to get the PC code working with the above PSX structure but it was having none of it
//  ... sorry about that ... TC
    // Is it in use.
    // Is it initialised.
    // Rotation if button is an IMD.
    // Copy of widget's state so we know if state has changed.
    // Any data we want to attach.
    // Any data we want to attach.
    // Surface to render the button into.
    //	uint8 *Buffer;		// Bitmap buffer.
//	iSurface *Surface;	// Ivis surface definition.
    //extern RENDERED_BUTTON System1Buffers[NUM_OBJECTBUFFERS];
//extern RENDERED_BUTTON System2Buffers[NUM_OBJECTBUFFERS];
    // Power required to manufacture the current item.
    // Set audio IDs for form opening/closing anims.
    // Initialise interface graphics.
    // Free up interface graphics.
    // Intialise button surfaces.
    // Free up button surfaces.
    // Get a free RENDERED_BUTTON structure for an object window button.
    // Clear ( make unused ) all RENDERED_BUTTON structures for the object window.
    // Clear ( make unused ) all RENDERED_BUTTON structures for the topic window.
    // Clear ( make unused ) a RENDERED_BUTTON structure.
    // Clear ( make unused ) a RENDERED_BUTTON structure.
    // Get a free RENDERED_BUTTON structure for a stat window button.
    // Clear ( make unused ) all RENDERED_BUTTON structures for the stat window.
    /*these have been set up for the Transporter - the design screen DOESN'T use them*/
// Clear ( make unused ) *all* RENDERED_BUTTON structures.
    // Clear ( make unused ) a RENDERED_BUTTON structure.
    // Get a free RENDERED_BUTTON structure.
    // callback to update the command droid size label
    // callback to update the command droid experience
    // callback to update the command droid factories
    //callback to display the factory number
    //callback to display the production quantity number for a template
    //void RenderCompositeDroid(UDWORD Index,iVector *Rotation,iVector *Position);
    //iIMDShape *TemplateGetIMD(DROID_TEMPLATE *DroidTemp,UDWORD Player);
//UDWORD TemplateGetIMDIndex(DROID_TEMPLATE *Template,UDWORD Player);
    //SDWORD ResearchGetImage(RESEARCH_FACILITY *Research);
    //iIMDShape *StatGetTemplateIMD(BASE_STATS *Stat,UDWORD Player);
//UDWORD StatGetTemplateIMDIndex(BASE_STATS *Stat,UDWORD Player);
    #[no_mangle]
    fn StatIsComponent(Stat: *mut BASE_STATS) -> SDWORD;
    //iIMDShape *StatGetComponentIMD(BASE_STATS *Stat);
//iIMDShape *StatGetComponentIMD(BASE_STATS *Stat, SDWORD compID);
    #[no_mangle]
    fn StatGetComponentIMD(Stat: *mut BASE_STATS, compID: SDWORD,
                           CompIMD: *mut *mut iIMDShape,
                           MountIMD: *mut *mut iIMDShape) -> BOOL;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
    #[no_mangle]
    fn getRandomDebrisImd() -> *mut iIMDShape;
    /* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
    #[no_mangle]
    fn effectGiveAuxVar(var: UDWORD);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn demoGetStatus() -> BOOL;
    #[no_mangle]
    fn bobTransporterHeight() -> SDWORD;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn objectShimmy(psObj: *mut BASE_OBJECT);
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
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
pub struct iPoint {
    pub x: int32,
    pub y: int32,
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
//*************************************************************************
//
// imd structures
//
//*************************************************************************
pub type BSPPOLYID = uint16;
// only needed when generating the tree
pub type WORLDCOORD = UDWORD;
pub type ANGLE = SWORD;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBJPOS {
    pub x: WORLDCOORD,
    pub y: WORLDCOORD,
    pub z: WORLDCOORD,
    pub pitch: ANGLE,
    pub yaw: ANGLE,
    pub roll: ANGLE,
}
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
pub type SCREEN_DISP_DATA = _screen_disp_data;
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
pub type _object_type = libc::c_uint;
// for the camera tracking
// Comes out of guns, stupid :-)
pub const OBJ_TARGET: _object_type = 4;
// Things like roads, trees, bridges, fires
pub const OBJ_BULLET: _object_type = 3;
// All Buildings
pub const OBJ_FEATURE: _object_type = 2;
// Droids
pub const OBJ_STRUCTURE: _object_type = 1;
pub const OBJ_DROID: _object_type = 0;
pub type OBJECT_TYPE = _object_type;
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
pub type BASE_OBJECT = _base_object;
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
pub type FIREONMOVE = _fireonmove;
pub type _fireonmove = libc::c_uint;
pub const FOM_YES: _fireonmove = 2;
pub const FOM_PARTIAL: _fireonmove = 1;
pub const FOM_NO: _fireonmove = 0;
pub type _prop_side = libc::c_uint;
pub const NUM_PROP_SIDES: _prop_side = 2;
pub const RIGHT_PROP: _prop_side = 1;
pub const LEFT_PROP: _prop_side = 0;
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
pub type SENSOR_STATS = _sensor_stats;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_stats {
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
    pub repairPoints: UDWORD,
    pub repairArmour: BOOL,
    pub location: UDWORD,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type REPAIR_STATS = _repair_stats;
pub type WEAPON_STATS = _weapon_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _construct_stats {
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
    pub constructPoints: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type CONSTRUCT_STATS = _construct_stats;
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
pub type RESEARCH = research_stats;
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
/* the 2nd IMD for base plates/turrets*/
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
pub type DROID = _droid;
pub type STRUCTURE = _structure;
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
pub const MI_FLAME: C2RustUnnamed = 21;
pub const MI_BLIP: C2RustUnnamed = 16;
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
pub const MI_BABA_BODY: C2RustUnnamed = 7;
pub const MI_BABA_ARM: C2RustUnnamed = 6;
pub const MI_BABA_LEGS: C2RustUnnamed = 5;
pub const MI_BABA_HEAD: C2RustUnnamed = 4;
pub const MI_CYBORG_BODY: C2RustUnnamed = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed = 8;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MI_TOO_MANY: C2RustUnnamed = 44;
pub const MI_FIREWORK: C2RustUnnamed = 43;
pub const MI_DEBRIS4: C2RustUnnamed = 42;
pub const MI_DEBRIS3: C2RustUnnamed = 41;
pub const MI_DEBRIS2: C2RustUnnamed = 40;
pub const MI_DEBRIS1: C2RustUnnamed = 39;
pub const MI_DEBRIS0: C2RustUnnamed = 38;
pub const MI_WRECK4: C2RustUnnamed = 37;
pub const MI_WRECK3: C2RustUnnamed = 36;
pub const MI_WRECK2: C2RustUnnamed = 35;
pub const MI_WRECK1: C2RustUnnamed = 34;
pub const MI_WRECK0: C2RustUnnamed = 33;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed = 30;
pub const MI_SHOCK: C2RustUnnamed = 29;
pub const MI_LANDING: C2RustUnnamed = 28;
pub const MI_KICK: C2RustUnnamed = 27;
pub const MI_SPLASH: C2RustUnnamed = 26;
pub const MI_SNOW: C2RustUnnamed = 25;
pub const MI_RAIN: C2RustUnnamed = 24;
pub const MI_MFLARE: C2RustUnnamed = 23;
pub const MI_TESLA: C2RustUnnamed = 22;
pub const MI_TRAIL: C2RustUnnamed = 20;
pub const MI_BLOOD: C2RustUnnamed = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed = 18;
pub const MI_SHADOW: C2RustUnnamed = 17;
pub const MI_PLASMA: C2RustUnnamed = 15;
pub const MI_SMALL_STEAM: C2RustUnnamed = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed = 13;
pub const MI_WATER: C2RustUnnamed = 12;
pub const MI_SMALL_SMOKE: C2RustUnnamed = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed = 0;
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
#[no_mangle]
pub static mut droidScale: UDWORD = 100 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut leftFirst: BOOL = 0;
#[no_mangle]
pub static mut droidLightLevel: SDWORD = 224 as libc::c_int;
#[no_mangle]
pub static mut lightInterval: UDWORD = 15 as libc::c_int as UDWORD;
#[no_mangle]
pub static mut lightLastChanged: UDWORD = 0;
#[no_mangle]
pub static mut lightSpeed: SDWORD = 2 as libc::c_int;
#[no_mangle]
pub static mut PlayerColour: [UBYTE; 8] = [0; 8];
// = {0,1,2,3,4,5,6,7}
// Colour Lookups
// use col = MAX_PLAYERS for anycolour (see multiint.c)
#[no_mangle]
pub unsafe extern "C" fn setPlayerColour(mut player_0: UDWORD,
                                         mut col: UDWORD) -> BOOL {
    if player_0 > 8 as libc::c_int as libc::c_uint ||
           col > 8 as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"setplayercolour: wrong values\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    PlayerColour[player_0 as UBYTE as usize] = col as UBYTE;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getPlayerColour(mut pl: UDWORD) -> UBYTE {
    return PlayerColour[pl as usize];
}
/*
	Header file for component.c 
	Pumpkin Studios, EIDOS Interactive. 
*/
// = {0,1,2,3,4,5,6,7}
#[no_mangle]
pub unsafe extern "C" fn initPlayerColours() {
    let mut i: UBYTE = 0;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 8 as libc::c_int {
        PlayerColour[i as usize] = i;
        i = i.wrapping_add(1)
    };
}
/* Don't know what these might be? */
//#define PROGRAM_IMD(DROID,PLAYER)	(DROID->asProgs[0]->pIMD)
//testing only - remove when decided
#[no_mangle]
pub unsafe extern "C" fn updateLightLevels() {
    if gameTime > lightLastChanged.wrapping_add(lightInterval) {
        droidLightLevel += lightSpeed;
        lightLastChanged = gameTime;
        if droidLightLevel > 255 as libc::c_int ||
               droidLightLevel < 128 as libc::c_int {
            if lightSpeed > 0 as libc::c_int {
                lightSpeed = -lightSpeed
            } else { lightSpeed = -lightSpeed }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn setMatrix(mut Position: *mut iVector,
                                   mut Rotation: *mut iVector,
                                   mut CameraPos: *mut iVector,
                                   mut RotXYZ: BOOL) {
    let mut BSPCameraPos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut Camera: OBJPOS =
        {
            let mut init =
                OBJPOS{x: 0 as libc::c_int as WORLDCOORD,
                       y: 0 as libc::c_int as WORLDCOORD,
                       z: 0 as libc::c_int as WORLDCOORD,
                       pitch: 0,
                       yaw: 0,
                       roll: 0,};
            init
        };
    Camera.pitch = -(45 as libc::c_int) as ANGLE;
    Camera.yaw = 0 as libc::c_int as ANGLE;
    //	Rotation->y=0;
    GetRealCameraPos(&mut Camera, (*Position).z, &mut BSPCameraPos);
    //	SetBSPCameraPos(BSPCameraPos.x,BSPCameraPos.y,BSPCameraPos.z);
	// Fixes BSP drawing in buttons. eg Player HQ.
    SetBSPCameraPos(BSPCameraPos.x, 500 as libc::c_int,
                    BSPCameraPos.z); // For imd button the bsp is sourced at 0,0,0
    SetBSPObjectPos(0 as libc::c_int, 0 as libc::c_int,
                    0 as libc::c_int); // Droid rotation
    SetBSPObjectRot(65536 as libc::c_int / 360 as libc::c_int *
                        -(*Rotation).y, 0 as libc::c_int);
    pie_PerspectiveBegin();
    pie_MatBegin();
    pie_TRANSLATE((*Position).x, (*Position).y, (*Position).z);
    if RotXYZ != 0 {
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).x);
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).y);
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).z);
    } else {
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).y);
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).x);
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        (*Rotation).z);
    };
}
#[no_mangle]
pub unsafe extern "C" fn unsetMatrix() { pie_MatEnd(); pie_PerspectiveEnd(); }
#[no_mangle]
pub unsafe extern "C" fn getComponentDroidRadius(mut psDroid: *mut DROID)
 -> UDWORD {
    return 100 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getComponentDroidTemplateRadius(mut psDroid:
                                                             *mut DROID_TEMPLATE)
 -> UDWORD {
    return 100 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getComponentRadius(mut psComponent: *mut BASE_STATS)
 -> UDWORD {
    let mut ComponentIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut MountIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut compID: SDWORD = 0;
    compID = StatIsComponent(psComponent);
    if compID > 0 as libc::c_int {
        StatGetComponentIMD(psComponent, compID, &mut ComponentIMD,
                            &mut MountIMD);
        //		ComponentIMD = StatGetComponentIMD(psComponent, compID);
        if !ComponentIMD.is_null() {
            return (*ComponentIMD).sradius as UDWORD
        }
    }
    //	DBERROR(("Not a valid component"));
	/* VTOL bombs are only stats allowed to have NULL ComponentIMD */
    if StatIsComponent(psComponent) != COMP_WEAPON as libc::c_int ||
           (*(psComponent as *mut WEAPON_STATS)).weaponSubClass as
               libc::c_uint != WSC_BOMB as libc::c_int as libc::c_uint {
        debug(LOG_NEVER,
              b"ComponentPIE == NULL : File : %s Line : %d\n\x00" as *const u8
                  as *const libc::c_char,
              b"component.c\x00" as *const u8 as *const libc::c_char,
              207 as libc::c_int);
    }
    return 64 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getResearchRadius(mut Stat: *mut BASE_STATS)
 -> UDWORD {
    let mut ResearchIMD: *mut iIMDShape = (*(Stat as *mut RESEARCH)).pIMD;
    if !ResearchIMD.is_null() { return (*ResearchIMD).sradius as UDWORD }
    //	DBERROR(("Null IMD in getResearchRadius()"));
    debug(LOG_NEVER,
          b"ResearchPIE == NULL : File : %s Line : %d\n\x00" as *const u8 as
              *const libc::c_char,
          b"component.c\x00" as *const u8 as *const libc::c_char,
          223 as libc::c_int);
    return 100 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getStructureSize(mut psStructure: *mut STRUCTURE)
 -> UDWORD {
    let mut size: UDWORD = 0;
    //radius based on base plate size
    size = (*(*psStructure).pStructureType).baseWidth;
    if (*(*psStructure).pStructureType).baseBreadth > size {
        size = (*(*psStructure).pStructureType).baseBreadth
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn getStructureRadius(mut psStructure: *mut STRUCTURE)
 -> UDWORD {
    let mut baseImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut Radius: SDWORD = (*(*psStructure).sDisplay.imd).sradius;
    baseImd = (*(*psStructure).pStructureType).pBaseIMD;
    if !baseImd.is_null() {
        if (*baseImd).sradius > Radius { Radius = (*baseImd).sradius }
    }
    return Radius as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn getStructureStatSize(mut Stats: *mut STRUCTURE_STATS)
 -> UDWORD {
    let mut size: UDWORD = 0;
    //radius based on base plate size
    size = (*Stats).baseWidth; // ajl changed 0 to selectedPlayer
    if (*Stats).baseBreadth > size { size = (*Stats).baseBreadth }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn getStructureStatRadius(mut Stats:
                                                    *mut STRUCTURE_STATS,
                                                mut Player: UDWORD)
 -> UDWORD {
    let mut baseImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut Radius: SDWORD = (*(*Stats).pIMD).sradius;
    baseImd = (*Stats).pBaseIMD;
    if !baseImd.is_null() {
        if (*baseImd).sradius > Radius { Radius = (*baseImd).sradius }
    }
    return Radius as UDWORD;
}
unsafe extern "C" fn getStructureHeight(mut psStructure: *mut STRUCTURE)
 -> UDWORD {
    return getStructureStatHeight((*psStructure).pStructureType);
}
#[no_mangle]
pub unsafe extern "C" fn getStructureStatHeight(mut psStat:
                                                    *mut STRUCTURE_STATS)
 -> UDWORD {
    if !(*psStat).pIMD.is_null() {
        return ((*(*psStat).pIMD).ymax - (*(*psStat).pIMD).ymin) as UDWORD
    }
    return 0 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn displayIMDButton(mut IMDShape: *mut iIMDShape,
                                          mut Rotation: *mut iVector,
                                          mut Position: *mut iVector,
                                          mut RotXYZ: BOOL,
                                          mut scale: SDWORD) {
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    pie_SetFogStatus(0 as libc::c_int);
    pie_Draw3DShape(IMDShape, 0 as libc::c_int,
                    getPlayerColour(selectedPlayer) as libc::c_int,
                    255 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                    0x40 as libc::c_int, 0 as libc::c_int);
    unsetMatrix();
}
#[no_mangle]
pub unsafe extern "C" fn displayStructureButton(mut psStructure:
                                                    *mut STRUCTURE,
                                                mut Rotation: *mut iVector,
                                                mut Position: *mut iVector,
                                                mut RotXYZ: BOOL,
                                                mut scale: SDWORD) {
    let mut baseImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut strImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut mountImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut weaponImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut nWeaponStat: UDWORD = 0;
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    /*HACK HACK HACK!
    if its a 'tall thin (ie tower)' structure with something on the top - offset the
    position to show the object on top*/
    if (*(*(*psStructure).pStructureType).pIMD).nconnectors != 0 &&
           scale == 55 as libc::c_int &&
           getStructureHeight(psStructure) >
               100 as libc::c_int as libc::c_uint {
        (*Position).y -= 20 as libc::c_int
    }
    SetBSPObjectPos(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    /* Draw the building's base first */
    baseImd = (*(*psStructure).pStructureType).pBaseIMD;
    if !baseImd.is_null() {
        pie_Draw3DShape(baseImd, 0 as libc::c_int,
                        getPlayerColour(selectedPlayer) as libc::c_int,
                        255 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                        0 as libc::c_int);
        // ajl changed 0 to selectedPlayer
    } // ajl changed 0 to selectedPlayer
    pie_Draw3DShape((*psStructure).sDisplay.imd, 0 as libc::c_int,
                    getPlayerColour(selectedPlayer) as libc::c_int,
                    255 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                    0x40 as libc::c_int, 0 as libc::c_int);
    //and draw the turret
    if (*(*psStructure).sDisplay.imd).nconnectors != 0 {
        weaponImd = 0 as *mut iIMDShape; //weapon is gun ecm or sensor
        mountImd = 0 as *mut iIMDShape;
        strImd = (*psStructure).sDisplay.imd;
        //get an imd to draw on the connector priority is weapon, ECM, sensor
		//check for weapon
		//if (psStructure->numWeaps > 0)
        if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
            nWeaponStat =
                (*psStructure).asWeaps[0 as libc::c_int as usize].nStat;
            weaponImd = (*asWeaponStats.offset(nWeaponStat as isize)).pIMD;
            mountImd =
                (*asWeaponStats.offset(nWeaponStat as isize)).pMountGraphic
        }
        if weaponImd.is_null() {
            //check for ECM
            if !(*(*psStructure).pStructureType).pECM.is_null() {
                weaponImd = (*(*(*psStructure).pStructureType).pECM).pIMD;
                mountImd =
                    (*(*(*psStructure).pStructureType).pECM).pMountGraphic
            }
        }
        if weaponImd.is_null() {
            //check for sensor
            if !(*(*psStructure).pStructureType).pSensor.is_null() {
                weaponImd = (*(*(*psStructure).pStructureType).pSensor).pIMD;
                mountImd =
                    (*(*(*psStructure).pStructureType).pSensor).pMountGraphic
            }
        }
        //draw Weapon/ECM/Sensor for structure
        if !weaponImd.is_null() {
            pie_MatBegin();
            pie_TRANSLATE((*(*strImd).connectors).x,
                          (*(*strImd).connectors).z,
                          (*(*strImd).connectors).y);
            pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                            -((*psStructure).turretRotation as SDWORD));
            if !mountImd.is_null() {
                pie_Draw3DShape(mountImd, 0 as libc::c_int,
                                getPlayerColour(selectedPlayer) as
                                    libc::c_int, 255 as libc::c_int as UDWORD,
                                0 as libc::c_int as UDWORD,
                                0x40 as libc::c_int, 0 as libc::c_int);
                if (*mountImd).nconnectors != 0 {
                    pie_TRANSLATE((*(*mountImd).connectors).x,
                                  (*(*mountImd).connectors).z,
                                  (*(*mountImd).connectors).y);
                }
            }
            pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                            (*psStructure).turretPitch as libc::c_int);
            pie_Draw3DShape(weaponImd, 0 as libc::c_int,
                            getPlayerColour(selectedPlayer) as libc::c_int,
                            255 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                            0 as libc::c_int);
            //we have a droid weapon so do we draw a muzzle flash
            pie_MatEnd();
        }
    }
    unsetMatrix();
}
#[no_mangle]
pub unsafe extern "C" fn displayStructureStatButton(mut Stats:
                                                        *mut STRUCTURE_STATS,
                                                    mut Player: UDWORD,
                                                    mut Rotation:
                                                        *mut iVector,
                                                    mut Position:
                                                        *mut iVector,
                                                    mut RotXYZ: BOOL,
                                                    mut scale: SDWORD) {
    let mut baseImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut strImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut mountImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut weaponImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    //UDWORD			nWeaponStat;
    /*HACK HACK HACK!
    if its a 'tall thin (ie tower)' structure stat with something on the top - offset the
    position to show the object on top*/
    if (*(*Stats).pIMD).nconnectors != 0 && scale == 55 as libc::c_int &&
           getStructureStatHeight(Stats) > 100 as libc::c_int as libc::c_uint
       {
        (*Position).y -= 20 as libc::c_int
    }
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    /* Draw the building's base first */
    baseImd = (*Stats).pBaseIMD;
    if !baseImd.is_null() {
        pie_Draw3DShape(baseImd, 0 as libc::c_int,
                        getPlayerColour(selectedPlayer) as libc::c_int,
                        255 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                        0 as libc::c_int);
        // ajl changed 0 to selectedPlayer
    } // ajl changed 0 to selectedPlayer
    pie_Draw3DShape((*Stats).pIMD, 0 as libc::c_int,
                    getPlayerColour(selectedPlayer) as libc::c_int,
                    255 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                    0x40 as libc::c_int, 0 as libc::c_int);
    //and draw the turret
    if (*(*Stats).pIMD).nconnectors != 0 {
        weaponImd = 0 as *mut iIMDShape; //weapon is gun ecm or sensor
        mountImd = 0 as *mut iIMDShape;
        strImd = (*Stats).pIMD;
        //get an imd to draw on the connector priority is weapon, ECM, sensor
		//check for weapon
		//if (Stats->numWeaps > 0)
        //can only have the one
        if !(*Stats).psWeapStat.is_null() {
            /*nWeaponStat = Stats->defaultWeap;
			weaponImd =  Stats->asWeapList[nWeaponStat]->pIMD;
			mountImd =  Stats->asWeapList[nWeaponStat]->pMountGraphic;*/
            weaponImd = (*(*Stats).psWeapStat).pIMD;
            mountImd = (*(*Stats).psWeapStat).pMountGraphic
        }
        if weaponImd.is_null() {
            //check for ECM
            if !(*Stats).pECM.is_null() {
                weaponImd = (*(*Stats).pECM).pIMD;
                mountImd = (*(*Stats).pECM).pMountGraphic
            }
        }
        if weaponImd.is_null() {
            //check for sensor
            if !(*Stats).pSensor.is_null() {
                weaponImd = (*(*Stats).pSensor).pIMD;
                mountImd = (*(*Stats).pSensor).pMountGraphic
            }
        }
        //draw Weapon/ECM/Sensor for structure
        if !weaponImd.is_null() {
            pie_MatBegin();
            pie_TRANSLATE((*(*strImd).connectors).x,
                          (*(*strImd).connectors).z,
                          (*(*strImd).connectors).y);
            pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                            0 as libc::c_int);
            if !mountImd.is_null() {
                pie_Draw3DShape(mountImd, 0 as libc::c_int,
                                getPlayerColour(selectedPlayer) as
                                    libc::c_int, 255 as libc::c_int as UDWORD,
                                0 as libc::c_int as UDWORD,
                                0x40 as libc::c_int, 0 as libc::c_int);
                if (*mountImd).nconnectors != 0 {
                    pie_TRANSLATE((*(*mountImd).connectors).x,
                                  (*(*mountImd).connectors).z,
                                  (*(*mountImd).connectors).y);
                }
            }
            pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                            0 as libc::c_int);
            pie_Draw3DShape(weaponImd, 0 as libc::c_int,
                            getPlayerColour(selectedPlayer) as libc::c_int,
                            255 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                            0 as libc::c_int);
            //we have a droid weapon so do we draw a muzzle flash
            pie_MatEnd();
        }
    }
    unsetMatrix();
}
// Render a component given a BASE_STATS structure.
//
#[no_mangle]
pub unsafe extern "C" fn displayComponentButton(mut Stat: *mut BASE_STATS,
                                                mut Rotation: *mut iVector,
                                                mut Position: *mut iVector,
                                                mut RotXYZ: BOOL,
                                                mut scale: SDWORD) {
    let mut ComponentIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut MountIMD: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut compID: SDWORD = 0;
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    compID = StatIsComponent(Stat);
    if compID > 0 as libc::c_int {
        StatGetComponentIMD(Stat, compID, &mut ComponentIMD, &mut MountIMD);
        //		ComponentIMD = StatGetComponentIMD(Stat, compID);
    } else { unsetMatrix(); return }
    /* VTOL bombs are only stats allowed to have NULL ComponentIMD */
    if ComponentIMD.is_null() &&
           (StatIsComponent(Stat) != COMP_WEAPON as libc::c_int ||
                (*(Stat as *mut WEAPON_STATS)).weaponSubClass as libc::c_uint
                    != WSC_BOMB as libc::c_int as libc::c_uint) {
        debug(LOG_NEVER,
              b"ComponentPIE == NULL : File : %s Line : %d\n\x00" as *const u8
                  as *const libc::c_char,
              b"component.c\x00" as *const u8 as *const libc::c_char,
              525 as libc::c_int);
        //		DBERROR(("ComponentIMD == NULL"));
    }
    if !MountIMD.is_null() {
        pie_Draw3DShape(MountIMD, 0 as libc::c_int,
                        getPlayerColour(selectedPlayer) as libc::c_int,
                        255 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                        0 as libc::c_int);
        // ajl changed 0 to selectedPlayer
    }
    if !ComponentIMD.is_null() {
        pie_Draw3DShape(ComponentIMD, 0 as libc::c_int,
                        getPlayerColour(selectedPlayer) as libc::c_int,
                        255 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                        0 as libc::c_int);
        // ajl changed 0 to selectedPlayer
    }
    unsetMatrix();
}
// Render a research item given a BASE_STATS structure.
//
#[no_mangle]
pub unsafe extern "C" fn displayResearchButton(mut Stat: *mut BASE_STATS,
                                               mut Rotation: *mut iVector,
                                               mut Position: *mut iVector,
                                               mut RotXYZ: BOOL,
                                               mut scale: SDWORD) {
    let mut ResearchIMD: *mut iIMDShape = (*(Stat as *mut RESEARCH)).pIMD;
    let mut MountIMD: *mut iIMDShape = (*(Stat as *mut RESEARCH)).pIMD2;
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    if !ResearchIMD.is_null() {
        setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
        pie_MatScale(scale as UDWORD);
        if !MountIMD.is_null() {
            pie_Draw3DShape(MountIMD, 0 as libc::c_int,
                            getPlayerColour(selectedPlayer) as libc::c_int,
                            255 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                            0 as libc::c_int);
            // ajl, added colourthing using selectedPlayer
        } //ajl, added colourthing using selectedPlayer
        pie_Draw3DShape(ResearchIMD, 0 as libc::c_int,
                        getPlayerColour(selectedPlayer) as libc::c_int,
                        255 as libc::c_int as UDWORD,
                        0 as libc::c_int as UDWORD, 0x40 as libc::c_int,
                        0 as libc::c_int);
        unsetMatrix();
    };
    //	DBERROR(("Null IMD in displayResearchButton()"));
}
// Render a composite droid given a DROID_TEMPLATE structure.
//
#[no_mangle]
pub unsafe extern "C" fn displayComponentButtonTemplate(mut psTemplate:
                                                            *mut DROID_TEMPLATE,
                                                        mut Rotation:
                                                            *mut iVector,
                                                        mut Position:
                                                            *mut iVector,
                                                        mut RotXYZ: BOOL,
                                                        mut scale: SDWORD) {
    static mut Droid: DROID =
        DROID{type_0: OBJ_DROID,
              id: 0,
              x: 0,
              y: 0,
              z: 0,
              direction: 0,
              pitch: 0,
              roll: 0,
              psNext: 0 as *const _droid as *mut _droid,
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
              burnDamage: 0,
              aName: [0; 60],
              droidType: DROID_WEAPON,
              asBits: [COMPONENT{nStat: 0,}; 8],
              weight: 0,
              baseSpeed: 0,
              sensorRange: 0,
              sensorPower: 0,
              ECMMod: 0,
              originalBody: 0,
              body: 0,
              armour: [0; 2],
              numKills: 0,
              turretRotation: 0,
              turretPitch: 0,
              NameVersion: 0,
              currRayAng: 0,
              resistance: 0,
              asWeaps:
                  [WEAPON{nStat: 0,
                          hitPoints: 0,
                          ammo: 0,
                          lastFired: 0,
                          recoilValue: 0,}; 1],
              psGroup: 0 as *const _droid_group as *mut _droid_group,
              psGrpNext: 0 as *const _droid as *mut _droid,
              psBaseStruct: 0 as *const _structure as *mut _structure,
              listSize: 0,
              asOrderList:
                  [ORDER_LIST{order: 0,
                              psOrderTarget:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              x: 0,
                              y: 0,
                              x2: 0,
                              y2: 0,}; 10],
              order: 0,
              orderX: 0,
              orderY: 0,
              orderX2: 0,
              orderY2: 0,
              lastHitWeapon: 0,
              timeLastHit: 0,
              bTargetted: 0,
              psTarget: 0 as *const _base_object as *mut _base_object,
              psTarStats: 0 as *const _base_stats as *mut _base_stats,
              secondaryOrder: 0,
              lastSync: 0,
              action: 0,
              actionX: 0,
              actionY: 0,
              psActionTarget: 0 as *const _base_object as *mut _base_object,
              actionStarted: 0,
              actionPoints: 0,
              powerAccrued: 0,
              illumination: 0,
              updateFlags: 0,
              sMove:
                  MOVE_CONTROL{Status: 0,
                               Mask: 0,
                               Position: 0,
                               numPoints: 0,
                               asPath: [PATH_POINT{x: 0, y: 0,}; 100],
                               DestinationX: 0,
                               DestinationY: 0,
                               srcX: 0,
                               srcY: 0,
                               targetX: 0,
                               targetY: 0,
                               fx: 0.,
                               fy: 0.,
                               speed: 0.,
                               boundX: 0,
                               boundY: 0,
                               dir: 0,
                               bumpDir: 0,
                               bumpTime: 0,
                               lastBump: 0,
                               pauseTime: 0,
                               bumpX: 0,
                               bumpY: 0,
                               shuffleStart: 0,
                               psFormation:
                                   0 as *const _formation as *mut _formation,
                               iVertSpeed: 0,
                               iAttackRuns: 0,
                               fz: 0.,},
              psCurAnim: 0 as *const ANIM_OBJECT as *mut ANIM_OBJECT,
              iAudioID: 0,}; // Made static to reduce stack usage.
    let mut difference: SDWORD = 0;
    let mut mountRotation: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    /* init to NULL */
    memset(&mut Droid as *mut DROID as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<DROID>() as libc::c_ulong);
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    // Decide how to sort it.
    difference =
        (*Rotation).y % 360 as libc::c_int; // important for psx stuff ...
    if difference > 0 as libc::c_int && difference < 180 as libc::c_int ||
           difference < -(180 as libc::c_int) {
        leftFirst = 0 as libc::c_int
    } else { leftFirst = 1 as libc::c_int }
    droidSetBits(psTemplate, &mut Droid);
    Droid.player = selectedPlayer as UBYTE;
    Droid.z = 0 as libc::c_int as UWORD;
    Droid.y = Droid.z;
    Droid.x = Droid.y;
    //draw multi component object as a button object
    displayCompObj(&mut Droid as *mut DROID as *mut BASE_OBJECT,
                   &mut mountRotation, 1 as libc::c_int);
    unsetMatrix();
}
// Render a composite droid given a DROID structure.
//
#[no_mangle]
pub unsafe extern "C" fn displayComponentButtonObject(mut psDroid: *mut DROID,
                                                      mut Rotation:
                                                          *mut iVector,
                                                      mut Position:
                                                          *mut iVector,
                                                      mut RotXYZ: BOOL,
                                                      mut scale: SDWORD) {
    let mut difference: SDWORD = 0;
    let mut mountRotation: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    let mut TmpCamPos: iVector =
        {
            let mut init =
                iVector{x: 0 as libc::c_int,
                        y: 0 as libc::c_int,
                        z: 0 as libc::c_int,};
            init
        };
    setMatrix(Position, Rotation, &mut TmpCamPos, RotXYZ);
    pie_MatScale(scale as UDWORD);
    // Decide how to sort it.
    difference = (*Rotation).y % 360 as libc::c_int;
    if difference > 0 as libc::c_int && difference < 180 as libc::c_int ||
           difference < -(180 as libc::c_int) {
        leftFirst = 0 as libc::c_int
    } else { leftFirst = 1 as libc::c_int }
    // And render the composite object.
	//draw multi component object as a button object
    displayCompObj(psDroid as *mut BASE_OBJECT, &mut mountRotation,
                   1 as libc::c_int);
    unsetMatrix();
}
/* Assumes matrix context is already set */
#[no_mangle]
pub unsafe extern "C" fn displayComponentObject(mut psObj: *mut BASE_OBJECT) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //iIMDShape	*psShape;
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,}; //,null;
    let mut rotation: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut mountRotation: iVector = iVector{x: 0, y: 0, z: 0,};
    //iPoint		screenCoords;
//SDWORD		dummyZ;
    let mut xShift: int32 = 0;
    let mut zShift: int32 = 0;
    let mut worldAngle: UDWORD = 0;
    let mut difference: SDWORD = 0;
    let mut frame: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut tileX: UDWORD = 0;
    let mut tileY: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psDroid = psObj as *mut DROID;
    psPropStats =
        asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION as
                                                       libc::c_int as
                                                       usize].nStat as
                                     libc::c_int as isize);
    worldAngle =
        (player.r.y as
             UDWORD).wrapping_div((65536 as libc::c_int / 360 as libc::c_int)
                                      as
                                      libc::c_uint).wrapping_rem(360 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
    difference =
        worldAngle.wrapping_sub((*psObj).direction as libc::c_uint) as SDWORD;
    if difference > 0 as libc::c_int && difference < 180 as libc::c_int ||
           difference < -(180 as libc::c_int) {
        leftFirst = 0 as libc::c_int
    } else { leftFirst = 1 as libc::c_int }
    /* Push the matrix */
    pie_MatBegin();
    /* Get internal tile units coordinates */
    xShift = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
    zShift = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
    /* Mask out to tile_units resolution */
    pie_TRANSLATE(xShift, 0 as libc::c_int, -zShift);
    /* Get the real position */
    position.x =
        (((*psDroid).x as libc::c_int - player.p.x) as
             libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint))
            as int32;
    position.z =
        terrainMidY.wrapping_mul(128 as libc::c_int as
                                     libc::c_uint).wrapping_sub(((*psDroid).y
                                                                     as
                                                                     libc::c_int
                                                                     -
                                                                     player.p.z)
                                                                    as
                                                                    libc::c_uint)
            as int32;
    position.y = (*psDroid).z as int32;
    //{
	//	position.y += bobTransporterHeight();
	//}
	//if(psPropStats->propulsionType == LIFT)
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        position.y += bobTransporterHeight()
    }
    /* Get all the pitch,roll,yaw info */
    rotation.y = -((*psDroid).direction as SDWORD);
    rotation.x = (*psDroid).pitch as int32;
    rotation.z = (*psDroid).roll as int32;
    /* Get rotation info for the mounting too (holds the gun */
    mountRotation.y = -((*psDroid).turretRotation as SDWORD);
    mountRotation.x = (*psDroid).turretPitch as int32;
    mountRotation.z = 0 as libc::c_int;
    /* Translate origin */
    pie_TRANSLATE(position.x, position.y, position.z);
    /* Rotate for droid */
    pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int * rotation.y);
    pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int * rotation.x);
    pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int * rotation.z);
    if gameTime.wrapping_sub((*psDroid).timeLastHit) <
           1000 as libc::c_int as libc::c_uint &&
           (*psDroid).lastHitWeapon ==
               WSC_ELECTRONIC as libc::c_int as libc::c_uint {
        objectShimmy(psDroid as *mut BASE_OBJECT);
    }
    if (*psDroid).lastHitWeapon == WSC_EMP as libc::c_int as libc::c_uint &&
           gameTime.wrapping_sub((*psDroid).timeLastHit) <
               10000 as libc::c_int as libc::c_uint {
        let mut position_0: iVector = iVector{x: 0, y: 0, z: 0,};
        //add an effect on the droid
        position_0.x =
            (*psDroid).x as libc::c_int +
                (20 as libc::c_int - rand() % 40 as libc::c_int);
        position_0.y =
            (*psDroid).z as libc::c_int + rand() % 8 as libc::c_int;
        position_0.z =
            (*psDroid).y as libc::c_int +
                (20 as libc::c_int - rand() % 40 as libc::c_int);
        effectGiveAuxVar((90 as libc::c_int + rand() % 20 as libc::c_int) as
                             UDWORD);
        addEffect(&mut position_0, EFFECT_EXPLOSION, EXPLOSION_TYPE_PLASMA,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    }
    if godMode != 0 ||
           (*psDroid).visible[selectedPlayer as usize] as libc::c_int ==
               0xff as libc::c_int || demoGetStatus() != 0 {
        //ingame not button object
        displayCompObj(psObj, &mut mountRotation, 0 as libc::c_int);
    } else {
        // make sure it's not over water.
        tileX = ((*psDroid).x as libc::c_int / 128 as libc::c_int) as UDWORD;
        tileY = ((*psDroid).y as libc::c_int / 128 as libc::c_int) as UDWORD;
        // double check it's on map
        if tileX >= 0 as libc::c_int as libc::c_uint &&
               tileY >= 0 as libc::c_int as libc::c_uint && tileX < mapWidth
               && tileY < mapHeight {
            psTile = mapTile(tileX, tileY); //visible[selectedPlayer];
            if terrainTypes[((*psTile).texture as libc::c_int &
                                 0x1ff as libc::c_int) as usize] as
                   libc::c_int != TER_WATER as libc::c_int {
                frame =
                    gameTime.wrapping_div(200 as libc::c_int as
                                              libc::c_uint).wrapping_add((*psDroid).id)
                        as SDWORD;
                pie_Draw3DShape(getImdFromIndex(MI_BLIP as libc::c_int as
                                                    UDWORD), frame,
                                0 as libc::c_int,
                                255 as libc::c_int as UDWORD,
                                0 as libc::c_int as UDWORD,
                                0x4 as libc::c_int,
                                (*psDroid).visible[selectedPlayer as usize] as
                                    libc::c_int / 2 as libc::c_int);
                // 	pie_Draw3DShape(blipImd, frame, 0, pie_MAX_BRIGHT_LEVEL, 0, pie_TRANSLUCENT, 128);
				/* set up all the screen coords stuff - need to REMOVE FROM THIS LOOP */
            }
        }
    }
    pie_MatEnd();
}
//void displayComponentObject(BASE_OBJECT *psObj);
/* Assumes matrix context is already set */
unsafe extern "C" fn displayCompObj(mut psObj: *mut BASE_OBJECT,
                                    mut mountRotation: *mut iVector,
                                    mut bButton: BOOL) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psShape: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psJet: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut null: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut screenCoords: iPoint = iPoint{x: 0, y: 0,};
    let mut dummyZ: SDWORD = 0;
    let mut iConnector: SDWORD = 0;
    let mut psPropStats: *mut PROPULSION_STATS = 0 as *mut PROPULSION_STATS;
    let mut frame: SDWORD = 0;
    let mut pieFlag: SDWORD = 0;
    let mut iPieData: SDWORD = 0;
    let mut brightness: UDWORD = 0;
    let mut specular: UDWORD = 0;
    //	SDWORD				centreX,centreZ;
    let mut colour: UDWORD = 0;
    let mut bDarkSide: UDWORD = 0 as libc::c_int as UDWORD;
    /* Cast the droid pointer */
    psDroid = psObj as *mut DROID;
    if gameTime.wrapping_sub((*psDroid).timeLastHit) <
           (1000 as libc::c_int / 4 as libc::c_int) as libc::c_uint &&
           (*psDroid).lastHitWeapon ==
               WSC_ELECTRONIC as libc::c_int as libc::c_uint &&
           gamePaused() == 0 {
        colour =
            getPlayerColour((rand() % 8 as libc::c_int) as UDWORD) as UDWORD;
        bDarkSide = 1 as libc::c_int as UDWORD
    } else { colour = getPlayerColour((*psDroid).player as UDWORD) as UDWORD }
    //set pieflag for button object or ingame object
    if bButton != 0 {
        pieFlag = 0x40 as libc::c_int
    } else { pieFlag = 0 as libc::c_int }
    if pieFlag == 0 {
        //		centreX = ( player.p.x + ((visibleXTiles/2)<<TILE_SHIFT) );
   //		centreZ = ( player.p.z + ((visibleYTiles/2)<<TILE_SHIFT) );
        brightness =
            lightDoFogAndIllumination((*psDroid).illumination,
                                      getCentreX() -
                                          (*psDroid).x as libc::c_int,
                                      getCentreZ() -
                                          (*psDroid).y as libc::c_int,
                                      &mut specular)
    } else {
        brightness = 255 as libc::c_int as UDWORD;
        specular = 0 as libc::c_int as UDWORD
    }
    //	/* No auxilliary rotation */
    null.z = 0 as libc::c_int;
    null.y = null.z;
    null.x = null.y;
    /* We've got a z value here _and_ screen coords of origin */
    dummyZ = pie_RotProj(&mut null, &mut screenCoords);
    /* Draw the propulsion and body imds here */
	/* Establish the propulsion - this is more complex if two parts */
/*
	if(leftFirst)
	{
		psShape = getLeftPropulsionIMD(psDroid);
		if(psShape!=NULL)
		{
			iV_PIEDraw(psShape,psDroid->player);
		}
	}
	else
	{
		psShape = getRightPropulsionIMD(psDroid);
		if(psShape!=NULL)
		{
			iV_PIEDraw(psShape,psDroid->player);
		}
	}
*/
//	pie_MatScale(droidScale);
    /* set default components transparent */
    if (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as
           libc::c_int == 0 as libc::c_int {
        pieFlag |= 0x2 as libc::c_int;
        iPieData = 128 as libc::c_int
    } else { iPieData = 0 as libc::c_int }
    psShape =
        if leftFirst != 0 {
            getLeftPropulsionIMD(psDroid)
        } else { getRightPropulsionIMD(psDroid) };
    if !psShape.is_null() {
        pie_Draw3DShape(psShape, 0 as libc::c_int, colour as libc::c_int,
                        brightness, specular, pieFlag, iPieData);
    }
    /* set default components transparent */
    if (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat as
           libc::c_int == 0 as libc::c_int {
        pieFlag |= 0x2 as libc::c_int;
        iPieData = 128 as libc::c_int
    } else { pieFlag &= !(0x2 as libc::c_int); iPieData = 0 as libc::c_int }
    /* Get the body graphic now*/
    psShape =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as
                                 isize)).pIMD;
    if !psShape.is_null() {
        // FIXME
        if (*psDroid).droidType as libc::c_uint ==
               DROID_PERSON as libc::c_int as libc::c_uint {
            /* draw body if not animating */
            if (*psDroid).psCurAnim.is_null() ||
                   (*(*psDroid).psCurAnim).bVisible == 0 as libc::c_int {
                // FIXME - hideous....!!!!
                pie_MatScale(75 as libc::c_int as UDWORD);
                pie_Draw3DShape(psShape, 0 as libc::c_int,
                                (*psDroid).player as libc::c_int -
                                    6 as libc::c_int, brightness, specular,
                                pieFlag, iPieData);
            }
        } else if cyborgDroid(psDroid) != 0 {
            //else if( psDroid->droidType == DROID_CYBORG)
            /* draw body if cyborg not animating */
            if (*psDroid).psCurAnim.is_null() ||
                   (*(*psDroid).psCurAnim).bVisible == 0 as libc::c_int {
                pie_Draw3DShape(psShape, 0 as libc::c_int,
                                colour as libc::c_int, brightness, specular,
                                pieFlag, iPieData);
            }
        } else {
            pie_Draw3DShape(psShape, 0 as libc::c_int, colour as libc::c_int,
                            brightness, specular, pieFlag, iPieData);
        }
    }
    /* get propulsion stats */
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
              b"component.c\x00" as *const u8 as *const libc::c_char,
              912 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"displayCompObj\x00")).as_ptr(),
              b"PTRVALID(psPropStats, sizeof(PROPULSION_STATS))\x00" as
                  *const u8 as *const libc::c_char);
    };
    /* render vtol jet if flying - horrible hack - GJ */
    if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int &&
           cyborgDroid(psDroid) == 0 && bButton == 0 {
        /* show flame if above ground */
        if (*psDroid).sMove.Status as libc::c_int != 0 as libc::c_int {
            /* draw flame if found  */
            /* GJ TODO: add flame-finding code here */
            psJet =
                (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as
                                                           libc::c_int as
                                                           usize].nStat as
                                         isize)).pFlameIMD;
            if !psJet.is_null() {
                pie_Draw3DShape(psJet,
                                getStaticTimeValueRange(100 as libc::c_int as
                                                            UDWORD,
                                                        (*psJet).numFrames as
                                                            UDWORD) as
                                    libc::c_int, colour as libc::c_int,
                                brightness, specular, 0x4 as libc::c_int,
                                200 as libc::c_int);
            }
        }
    }
    //don't change the screen coords of an object if drawing it in a button
    if bButton == 0 {
        /* set up all the screen coords stuff - need to REMOVE FROM THIS LOOP */
        calcScreenCoords(psDroid);
    }
    /* set default components transparent */
    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint &&
           (*psDroid).asBits[COMP_SENSOR as libc::c_int as usize].nStat as
               libc::c_int == 0 as libc::c_int &&
           (*psDroid).asBits[COMP_ECM as libc::c_int as usize].nStat as
               libc::c_int == 0 as libc::c_int &&
           (*psDroid).asBits[COMP_BRAIN as libc::c_int as usize].nStat as
               libc::c_int == 0 as libc::c_int &&
           (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as usize].nStat as
               libc::c_int == 0 as libc::c_int &&
           (*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as usize].nStat as
               libc::c_int == 0 as libc::c_int {
        pieFlag |= 0x2 as libc::c_int;
        iPieData = 128 as libc::c_int
    } else { pieFlag &= !(0x2 as libc::c_int); iPieData = 0 as libc::c_int }
    /* Indenting here is only to show new matrix context */
    psShape =
        (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                   usize].nStat as
                                 isize)).pIMD; // end of illustrative indentation - see above
    pie_MatBegin();
    if (*psShape).nconnectors != 0 {
        /* vtol weapons attach to connector 2 (underneath);
			 * all others to connector 1 */
        if (*psPropStats).propulsionType as libc::c_int == LIFT as libc::c_int
               &&
               (*psDroid).droidType as libc::c_uint ==
                   DROID_WEAPON as libc::c_int as libc::c_uint {
            iConnector = 1 as libc::c_int
        } else { iConnector = 0 as libc::c_int }
        /* Now we need to move for the mount point */
        pie_TRANSLATE((*(*psShape).connectors.offset(iConnector as isize)).x,
                      (*(*psShape).connectors.offset(iConnector as isize)).z,
                      (*(*psShape).connectors.offset(iConnector as isize)).y);
        /* Rotate the turret */
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*mountRotation).y);
        //dont pitch the turret
//			pie_MatRotZ(DEG(mountRotation->z));
        /* vtol weapons inverted */
        if iConnector == 1 as libc::c_int {
            pie_MatRotZ(65536 as libc::c_int / 2 as libc::c_int);
            //this might affect gun rotation
        }
        //SEPERATE Mount IMDs now...
		/*	Get the mounting graphic - we've already moved to the right position
			Allegedly - all droids will have a mount graphic so this shouldn't
			fall on it's arse......*/
		//psShape = MOUNT_IMD(psDroid,psDroid->player);
		/* Draw it */
        match (*psDroid).droidType as libc::c_uint {
            9 | 6 | 5 | 12 | 0 | 7 => {
                // command droids have a weapon to store all the graphics
                /*	Get the mounting graphic - we've already moved to the right position
				Allegedly - all droids will have a mount graphic so this shouldn't
				fall on it's arse......*/
				/* Double check that the weapon droid actually has any */
				//if(psDroid->numWeaps)
                if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                       0 as libc::c_int as libc::c_uint ||
                       (*psDroid).droidType as libc::c_uint ==
                           DROID_DEFAULT as libc::c_int as libc::c_uint {
                    psShape =
                        (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].nStat
                                                   as isize)).pMountGraphic;
                    /* Draw it */
					//if(psDroid->numWeaps) already done this check above?!
                    pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                  (*psDroid).asWeaps[0 as libc::c_int as
                                                         usize].recoilValue.wrapping_div(3
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint)
                                      as libc::c_int);
                    if !psShape.is_null() {
                        pie_Draw3DShape(psShape, 0 as libc::c_int,
                                        colour as libc::c_int, brightness,
                                        specular, pieFlag, iPieData);
                    }
                    //if(psDroid->numWeaps) already done this check above?!
                    pie_TRANSLATE(0 as libc::c_int, 0 as libc::c_int,
                                  (*psDroid).asWeaps[0 as libc::c_int as
                                                         usize].recoilValue as
                                      libc::c_int);
                    /* translate for weapon mount point if cyborg */
					//if( psDroid->droidType == DROID_CYBORG &&
                    if cyborgDroid(psDroid) != 0 && !psShape.is_null() &&
                           (*psShape).nconnectors != 0 {
                        pie_TRANSLATE((*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).x,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).z,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).y);
                    }
                    /* vtol weapons inverted */
                    if iConnector == 1 as libc::c_int {
                        //pitch the barrel down
                        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int
                                        * -(*mountRotation).x);
                    } else {
                        //pitch the barrel up
                        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int
                                        * (*mountRotation).x);
                    }
                    /* Get the weapon (gun?) graphic */
                    psShape =
                        (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                      libc::c_int
                                                                      as
                                                                      usize].nStat
                                                   as isize)).pIMD;
                    /* Draw it */
                    if !psShape.is_null() {
                        pie_Draw3DShape(psShape, 0 as libc::c_int,
                                        colour as libc::c_int, brightness,
                                        specular, pieFlag, iPieData);
                    }
                    //we have a droid weapon so do we draw a muzzle flash
                    if !psShape.is_null() && (*psShape).nconnectors != 0 {
                        /* Now we need to move to the end fo the barrel */
                        pie_TRANSLATE((*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).x,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).z,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).y);
                        //and draw the muzzle flash
						//animate for the duration of the flash only
                        psShape =
                            (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                          libc::c_int
                                                                          as
                                                                          usize].nStat
                                                       as
                                                       isize)).pMuzzleGraphic;
                        if !psShape.is_null() {
                            //assume no clan colours formuzzle effects
                            if (*psShape).numFrames as libc::c_int ==
                                   0 as libc::c_int ||
                                   (*psShape).animInterval as libc::c_int <=
                                       0 as libc::c_int {
                                //no anim so display one frame for a fixed time
                                if gameTime <
                                       (*(*psDroid).asWeaps.as_mut_ptr()).lastFired.wrapping_add((1000
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      /
                                                                                                      10
                                                                                                          as
                                                                                                          libc::c_int)
                                                                                                     as
                                                                                                     libc::c_uint)
                                   {
                                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    brightness,
                                                    0 as libc::c_int as
                                                        UDWORD,
                                                    pieFlag |
                                                        0x4 as libc::c_int,
                                                    128 as libc::c_int);
                                    //muzzle flash
                                    //								pie_Draw3DShape(psShape, 0, 0, brightness, 0, pieFlag, 0);//muzzle flash
                                }
                            } else {
                                frame =
                                    gameTime.wrapping_sub((*(*psDroid).asWeaps.as_mut_ptr()).lastFired).wrapping_div((*psShape).animInterval
                                                                                                                         as
                                                                                                                         libc::c_uint)
                                        as SDWORD;
                                if frame < (*psShape).numFrames as libc::c_int
                                   {
                                    pie_Draw3DShape(psShape, frame,
                                                    0 as libc::c_int,
                                                    brightness,
                                                    0 as libc::c_int as
                                                        UDWORD,
                                                    pieFlag |
                                                        0x4 as libc::c_int,
                                                    128 as libc::c_int);
                                    //muzzle flash
                                    //								pie_Draw3DShape(psShape, frame, 0, brightness, 0, pieFlag, 0);//muzzle flash
                                }
                            }
                        }
                    }
                }
            }
            1 => {
                /*	Get the mounting graphic - we've already moved to the right position
				Allegedly - all droids will have a mount graphic so this shouldn't
				fall on it's arse......*/
                psShape =
                    (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                                 libc::c_int
                                                                 as
                                                                 usize].nStat
                                               as isize)).pMountGraphic;
                /* Draw it */
                if !psShape.is_null() {
                    //					pie_MatRotY(DEG(getStaticTimeValueRange(7920,360)));
//					pie_MatRotY(DEG(psDroid->turretRotation));
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
                /* Get the sensor graphic, assuming it's there */
                psShape =
                    (*asSensorStats.offset((*psDroid).asBits[COMP_SENSOR as
                                                                 libc::c_int
                                                                 as
                                                                 usize].nStat
                                               as isize)).pIMD;
                /* Draw it */
                if !psShape.is_null() {
                    //					pie_MatRotY(DEG(getStaticTimeValueRange(7920,360)));
//					pie_MatRotY(DEG(psDroid->turretRotation));
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
            }
            3 | 10 => {
                /*	Get the mounting graphic - we've already moved to the right position
				Allegedly - all droids will have a mount graphic so this shouldn't
				fall on it's arse......*/
                psShape =
                    (*asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                                  as isize)).pMountGraphic;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
                /* translate for construct mount point if cyborg */
                if cyborgDroid(psDroid) != 0 && !psShape.is_null() &&
                       (*psShape).nconnectors != 0 {
                    pie_TRANSLATE((*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).x,
                                  (*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).z,
                                  (*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).y);
                }
                /* Get the construct graphic assuming it's there */
                psShape =
                    (*asConstructStats.offset((*psDroid).asBits[COMP_CONSTRUCT
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                                  as isize)).pIMD;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
            }
            2 => {
                /*	Get the mounting graphic - we've already moved to the right position
				Allegedly - all droids will have a mount graphic so this shouldn't
				fall on it's arse......*/
                psShape =
                    (*asECMStats.offset((*psDroid).asBits[COMP_ECM as
                                                              libc::c_int as
                                                              usize].nStat as
                                            isize)).pMountGraphic;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
                /* Get the ECM graphic assuming it's there.... */
                psShape =
                    (*asECMStats.offset((*psDroid).asBits[COMP_ECM as
                                                              libc::c_int as
                                                              usize].nStat as
                                            isize)).pIMD;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
            }
            8 | 11 => {
                /*	Get the mounting graphic - we've already moved to the right position
				Allegedly - all droids will have a mount graphic so this shouldn't
				fall on it's arse......*/
                psShape =
                    (*asRepairStats.offset((*psDroid).asBits[COMP_REPAIRUNIT
                                                                 as
                                                                 libc::c_int
                                                                 as
                                                                 usize].nStat
                                               as isize)).pMountGraphic;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                }
                /* translate for construct mount point if cyborg */
                if cyborgDroid(psDroid) != 0 && !psShape.is_null() &&
                       (*psShape).nconnectors != 0 {
                    pie_TRANSLATE((*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).x,
                                  (*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).z,
                                  (*(*psShape).connectors.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).y);
                }
                /* Get the Repair graphic assuming it's there.... */
                psShape =
                    (*asRepairStats.offset((*psDroid).asBits[COMP_REPAIRUNIT
                                                                 as
                                                                 libc::c_int
                                                                 as
                                                                 usize].nStat
                                               as isize)).pIMD;
                /* Draw it */
                if !psShape.is_null() {
                    pie_Draw3DShape(psShape, 0 as libc::c_int,
                                    colour as libc::c_int, brightness,
                                    specular, pieFlag, iPieData);
                    if (*psShape).nconnectors != 0 &&
                           (*psDroid).action ==
                               DACTION_DROIDREPAIR as libc::c_int {
                        pie_TRANSLATE((*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).x,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).z,
                                      (*(*psShape).connectors.offset(0 as
                                                                         libc::c_int
                                                                         as
                                                                         isize)).y);
                        pie_TRANSLATE(0 as libc::c_int, -(20 as libc::c_int),
                                      0 as libc::c_int);
                        psShape =
                            getImdFromIndex(MI_FLAME as libc::c_int as
                                                UDWORD);
                        /* Rotate for droid */
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        * (*psDroid).direction as SDWORD);
                        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int
                                        * -((*psDroid).pitch as libc::c_int));
                        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int
                                        * -((*psDroid).roll as libc::c_int));
                        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int
                                        * -(*mountRotation).y);
                        pie_MatRotY(-player.r.y);
                        pie_MatRotX(-player.r.x);
                        /* Dither on software */
                        pie_Draw3DShape(psShape,
                                        getStaticTimeValueRange(100 as
                                                                    libc::c_int
                                                                    as UDWORD,
                                                                (*psShape).numFrames
                                                                    as UDWORD)
                                            as libc::c_int, 0 as libc::c_int,
                                        brightness,
                                        0 as libc::c_int as UDWORD,
                                        0x4 as libc::c_int,
                                        140 as libc::c_int);
                        /* Dither off software */
                        pie_MatRotX(player.r.x);
                        pie_MatRotY(player.r.y);
                    }
                }
            }
            4 => { }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Whoa! Weirdy type of droid found in drawComponentObject!!!\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"component.c\x00" as *const u8 as
                              *const libc::c_char, 1223 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"displayCompObj\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    }
    /*	We've also got a handle on the psShape here for the weapon which has a connector to point to
			muzzle flash attachment points - just grab it from psShape->connectors->[x|y|z] */
    /* Pop Matrix */
    pie_MatEnd();
    /* set default components transparent */
    if (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as
           libc::c_int == 0 as libc::c_int {
        pieFlag |= 0x2 as libc::c_int;
        iPieData = 128 as libc::c_int
    } else { pieFlag &= !(0x2 as libc::c_int); iPieData = 0 as libc::c_int }
    psShape =
        if leftFirst != 0 {
            getRightPropulsionIMD(psDroid)
        } else { getLeftPropulsionIMD(psDroid) };
    if !psShape.is_null() {
        pie_Draw3DShape(psShape, 0 as libc::c_int, colour as libc::c_int,
                        brightness, specular, pieFlag, iPieData);
    };
    /*
	if(leftFirst)
	{
		psShape = getRightPropulsionIMD(psDroid);
		if(psShape!=NULL)
		{
			iV_PIEDraw(psShape,psDroid->player);
		}
	}
	else
	{
		psShape = getLeftPropulsionIMD(psDroid);
		if(psShape!=NULL)
		{
			iV_PIEDraw(psShape,psDroid->player);
		}
	}
	*/
}
#[no_mangle]
pub unsafe extern "C" fn destroyFXDroid(mut psDroid: *mut DROID) {
    let mut i: UDWORD = 0;
    let mut psImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut widthScatter: SDWORD = 0;
    let mut breadthScatter: SDWORD = 0;
    let mut heightScatter: SDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    widthScatter = 128 as libc::c_int / 4 as libc::c_int;
    breadthScatter = 128 as libc::c_int / 4 as libc::c_int;
    heightScatter = 128 as libc::c_int / 5 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < 5 as libc::c_int as libc::c_uint {
        pos.x =
            (*psDroid).x as libc::c_int + widthScatter -
                rand() % (2 as libc::c_int * widthScatter);
        pos.z =
            (*psDroid).y as libc::c_int + breadthScatter -
                rand() % (2 as libc::c_int * breadthScatter);
        pos.y =
            (*psDroid).z as libc::c_int + 16 as libc::c_int + heightScatter;
        match i {
            0 => {
                match (*psDroid).droidType as libc::c_uint {
                    9 | 5 | 12 | 10 | 11 | 0 | 7 => {
                        //if(psDroid->numWeaps)
                        if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat
                               > 0 as libc::c_int as libc::c_uint {
                            psImd =
                                (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize].nStat
                                                           as
                                                           isize)).pMountGraphic
                        } else {
                            //					psImd = debrisImds[rand()%MAX_DEBRIS];
                            psImd = getRandomDebrisImd()
                        }
                    }
                    _ => {
                        //				psImd = debrisImds[rand()%MAX_DEBRIS];
                        psImd = getRandomDebrisImd()
                    }
                }
            }
            1 => {
                match (*psDroid).droidType as libc::c_uint {
                    9 | 5 | 12 | 10 | 11 | 0 | 7 => {
                        //if(psDroid->numWeaps)
                        if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat
                               > 0 as libc::c_int as libc::c_uint {
                            psImd =
                                (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize].nStat
                                                           as isize)).pIMD
                        } else {
                            //					psImd = debrisImds[rand()%MAX_DEBRIS];
                            psImd = getRandomDebrisImd()
                        }
                    }
                    _ => {
                        //				psImd = debrisImds[rand()%MAX_DEBRIS];
                        psImd = getRandomDebrisImd()
                    }
                }
            }
            2 | 3 | 4 => {
                //			psImd = debrisImds[rand()%MAX_DEBRIS];
                psImd = getRandomDebrisImd()
            }
            _ => { }
        }
        if !psImd.is_null() {
            addEffect(&mut pos, EFFECT_GRAVITON, GRAVITON_TYPE_EMITTING_DR,
                      1 as libc::c_int, psImd,
                      getPlayerColour((*psDroid).player as UDWORD) as BOOL);
        } else {
            //			addEffect(&pos,EFFECT_GRAVITON,GRAVITON_TYPE_EMITTING_DR,TRUE,debrisImds[rand()%MAX_DEBRIS],0);
            addEffect(&mut pos, EFFECT_GRAVITON, GRAVITON_TYPE_EMITTING_DR,
                      1 as libc::c_int, getRandomDebrisImd(),
                      0 as libc::c_int);
        }
        i = i.wrapping_add(1)
    };
}
//void addBodyPartEffect(iVector *position,iIMDShape *psShape)
//{
//	velocity.x = 1-rand()%3;
//	velocity.z = 1-rand()%3;
//	velocity.y = 4+rand()%7;
//	addEffect(position,EFFECT_GRAVITON,GRAVITON_TYPE_GIBLET,TRUE,psShape,0);
//}
#[no_mangle]
pub unsafe extern "C" fn compPersonToBits(mut psDroid: *mut DROID) {
    let mut position: iVector =
        iVector{x: 0, y: 0, z: 0,}; //,rotation,velocity;
    let mut headImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut legsImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut armImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut bodyImd: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut groundHeight: UDWORD = 0;
    let mut col: UDWORD = 0;
    if (*psDroid).visible[selectedPlayer as usize] == 0 {
        /* We can't see the person or cyborg - so get out */
        return
    }
    /* get bits pointers according to whether baba or cyborg*/
	//if ( psDroid->droidType == DROID_CYBORG )
    if cyborgDroid(psDroid) != 0 {
        headImd = getImdFromIndex(MI_CYBORG_HEAD as libc::c_int as UDWORD);
        legsImd = getImdFromIndex(MI_CYBORG_LEGS as libc::c_int as UDWORD);
        armImd = getImdFromIndex(MI_CYBORG_ARM as libc::c_int as UDWORD);
        bodyImd = getImdFromIndex(MI_CYBORG_BODY as libc::c_int as UDWORD)
    } else {
        headImd = getImdFromIndex(MI_BABA_HEAD as libc::c_int as UDWORD);
        legsImd = getImdFromIndex(MI_BABA_LEGS as libc::c_int as UDWORD);
        armImd = getImdFromIndex(MI_BABA_ARM as libc::c_int as UDWORD);
        bodyImd = getImdFromIndex(MI_BABA_BODY as libc::c_int as UDWORD)
    }
    /* Get where he's at */
    position.x = (*psDroid).x as int32;
    position.y = (*psDroid).z as libc::c_int + 1 as libc::c_int;
    groundHeight = (*psDroid).z as UDWORD;
    position.z = (*psDroid).y as int32;
    /* Tell about player colour */
    col = getPlayerColour((*psDroid).player as UDWORD) as UDWORD;
    addEffect(&mut position, EFFECT_GRAVITON, GRAVITON_TYPE_GIBLET,
              1 as libc::c_int, headImd, col as BOOL);
    addEffect(&mut position, EFFECT_GRAVITON, GRAVITON_TYPE_GIBLET,
              1 as libc::c_int, legsImd, col as BOOL);
    addEffect(&mut position, EFFECT_GRAVITON, GRAVITON_TYPE_GIBLET,
              1 as libc::c_int, armImd, col as BOOL);
    addEffect(&mut position, EFFECT_GRAVITON, GRAVITON_TYPE_GIBLET,
              1 as libc::c_int, bodyImd, col as BOOL);
}
unsafe extern "C" fn getLeftPropulsionIMD(mut psDroid: *mut DROID)
 -> *mut iIMDShape {
    let mut bodyStat: UDWORD = 0;
    let mut propStat: UDWORD = 0;
    let mut imd: *mut *mut iIMDShape = 0 as *mut *mut iIMDShape;
    bodyStat =
        (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat as UDWORD;
    propStat =
        (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as
            UDWORD;
    imd = (*asBodyStats.offset(bodyStat as isize)).ppIMDList;
    imd =
        imd.offset(propStat.wrapping_mul(NUM_PROP_SIDES as libc::c_int as
                                             libc::c_uint).wrapping_add(LEFT_PROP
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                       as isize);
    return *imd;
}
unsafe extern "C" fn getRightPropulsionIMD(mut psDroid: *mut DROID)
 -> *mut iIMDShape {
    let mut bodyStat: UDWORD = 0;
    let mut propStat: UDWORD = 0;
    let mut imd: *mut *mut iIMDShape = 0 as *mut *mut iIMDShape;
    bodyStat =
        (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat as UDWORD;
    propStat =
        (*psDroid).asBits[COMP_PROPULSION as libc::c_int as usize].nStat as
            UDWORD;
    imd = (*asBodyStats.offset(bodyStat as isize)).ppIMDList;
    imd =
        imd.offset(propStat.wrapping_mul(NUM_PROP_SIDES as libc::c_int as
                                             libc::c_uint).wrapping_add(RIGHT_PROP
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                       as isize);
    return *imd;
}
#[no_mangle]
pub unsafe extern "C" fn rescaleButtonObject(mut radius: SDWORD,
                                             mut baseScale: SDWORD,
                                             mut baseRadius: SDWORD)
 -> SDWORD {
    let mut newScale: SDWORD = 0;
    newScale = 100 as libc::c_int * baseRadius;
    newScale /= radius;
    if baseScale > 0 as libc::c_int {
        newScale += baseScale;
        newScale /= 2 as libc::c_int
    }
    return newScale;
}
