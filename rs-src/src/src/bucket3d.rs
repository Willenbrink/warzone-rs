use ::libc;
extern "C" {
    pub type _formation;
    pub type _droid_group;
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
    //Sets all states
    #[no_mangle]
    fn pie_SetDepthBufferStatus(depthMode: DEPTH_MODE);
    /* **************************************************************************/
/*
 * pieBlitFunc.h
 *
 * patch for exisitng ivis rectangle draw functions.
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
 *	Global Variables
 */
/* **************************************************************************/
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_Line(x0: libc::c_int, y0: libc::c_int, x1: libc::c_int,
                y1: libc::c_int, colour: uint32);
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    #[no_mangle]
    fn pie_RotProj(v3d: *mut iVector, v2d: *mut iPoint) -> int32;
    /* **************************************************************************/
/*
 * pieclip.h
 *
 * clipping for all pumpkin image library functions.
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
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    fn renderDroid(psDroid: *mut DROID);
    #[no_mangle]
    fn renderStructure(psStructure: *mut STRUCTURE);
    #[no_mangle]
    fn renderFeature(psFeature: *mut FEATURE);
    #[no_mangle]
    fn renderProximityMsg(psProxDisp: *mut PROXIMITY_DISPLAY);
    #[no_mangle]
    fn drawTerrainTile(i: UDWORD, j: UDWORD);
    //fast version - optimised
    #[no_mangle]
    fn drawTerrainWaterTile(i: UDWORD, j: UDWORD);
    #[no_mangle]
    fn renderProjectile(psCurr: *mut PROJ_OBJECT);
    #[no_mangle]
    fn renderAnimComponent(psObj: *mut COMPONENT_OBJECT);
    #[no_mangle]
    fn renderDeliveryPoint(psPosition: *mut FLAG_POSITION);
    #[no_mangle]
    fn renderShadow(psDroid: *mut DROID, psShadowIMD: *mut iIMDShape);
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut terrainMidX: UDWORD;
    #[no_mangle]
    static mut terrainMidY: UDWORD;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
    //extern iIMDShape	*pAssemblyPointIMDs[NUM_FACTORY_TYPES][MAX_FACTORY];
    #[no_mangle]
    static mut pAssemblyPointIMDs: [[*mut iIMDShape; 5]; 4];
    #[no_mangle]
    fn renderEffect(psEffect: *mut EFFECT);
    #[no_mangle]
    fn renderParticle(psPart: *mut ATPART);
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
pub type DEPTH_MODE = libc::c_uint;
pub const DEPTH_CMP_ALWAYS_WRT_OFF: DEPTH_MODE = 3;
pub const DEPTH_CMP_LEQ_WRT_OFF: DEPTH_MODE = 2;
pub const DEPTH_CMP_ALWAYS_WRT_ON: DEPTH_MODE = 1;
pub const DEPTH_CMP_LEQ_WRT_ON: DEPTH_MODE = 0;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
pub struct SIMPLE_OBJECT {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut SIMPLE_OBJECT,
}
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
/* **************************************************************************/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PROJ_OBJECT {
    pub type_0: OBJECT_TYPE,
    pub id: UDWORD,
    pub x: UWORD,
    pub y: UWORD,
    pub z: UWORD,
    pub direction: UWORD,
    pub pitch: SWORD,
    pub roll: SWORD,
    pub psNext: *mut PROJ_OBJECT,
    pub state: UBYTE,
    pub airTarget: UBYTE,
    pub player: UBYTE,
    pub bVisible: UBYTE,
    pub psWStats: *mut WEAPON_STATS,
    pub psSource: *mut BASE_OBJECT,
    pub psDest: *mut BASE_OBJECT,
    pub startX: UDWORD,
    pub startY: UDWORD,
    pub tarX: UDWORD,
    pub tarY: UDWORD,
    pub vXY: SDWORD,
    pub vZ: SDWORD,
    pub srcHeight: UDWORD,
    pub altChange: SDWORD,
    pub born: UDWORD,
    pub targetRadius: UDWORD,
    pub pInFlightFunc: PROJECTILE_FUNC,
}
pub type PROJECTILE_FUNC
    =
    Option<unsafe extern "C" fn(_: *mut PROJ_OBJECT) -> ()>;
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
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const EFFECT_FIREWORK: C2RustUnnamed_1 = 11;
pub const EFFECT_FIRE: C2RustUnnamed_1 = 10;
pub const EFFECT_DUST_BALL: C2RustUnnamed_1 = 9;
pub const EFFECT_SAT_LASER: C2RustUnnamed_1 = 8;
pub const EFFECT_DESTRUCTION: C2RustUnnamed_1 = 7;
pub const EFFECT_BLOOD: C2RustUnnamed_1 = 6;
pub const EFFECT_WAYPOINT: C2RustUnnamed_1 = 5;
pub const EFFECT_GRAVITON: C2RustUnnamed_1 = 4;
pub const EFFECT_STRUCTURE: C2RustUnnamed_1 = 3;
pub const EFFECT_SMOKE: C2RustUnnamed_1 = 2;
pub const EFFECT_CONSTRUCTION: C2RustUnnamed_1 = 1;
pub const EFFECT_EXPLOSION: C2RustUnnamed_1 = 0;
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
// Feature armour
// + (30 - rand()%60))
// + (20 - rand()%40))
// 1.5 seconds
// 3.5 seconds
/*
	Definition of an 'effect'. This'll need to come down in size
	for the PlayStation?
*/
pub type EFFECT = _effect_def;
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
/* bucket3D.h */
pub type RENDER_TYPE = _render_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_bucket {
    pub i: UDWORD,
    pub j: UDWORD,
    pub depth: SDWORD,
}
pub type TILE_BUCKET = _tile_bucket;
pub type BUCKET_TAG = _bucket_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bucket_tag {
    pub psNextTag: *mut _bucket_tag,
    pub objectType: RENDER_TYPE,
    pub pObject: *mut libc::c_void,
    pub actualZ: SDWORD,
}
pub type ATPART = _atmosParticle;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _atmosParticle {
    pub status: UBYTE,
    pub type_0: UBYTE,
    pub size: UDWORD,
    pub position: PIEVECTORF,
    pub velocity: PIEVECTORF,
    pub imd: *mut iIMDShape,
}
#[no_mangle]
pub static mut tagResource: [BUCKET_TAG; 4000] =
    [BUCKET_TAG{psNextTag: 0 as *const _bucket_tag as *mut _bucket_tag,
                objectType: RENDER_DROID,
                pObject: 0 as *const libc::c_void as *mut libc::c_void,
                actualZ: 0,}; 4000];
#[no_mangle]
pub static mut bucketArray: [*mut BUCKET_TAG; 8000] =
    [0 as *const BUCKET_TAG as *mut BUCKET_TAG; 8000];
#[no_mangle]
pub static mut resourceCounter: UDWORD = 0;
#[no_mangle]
pub static mut zMax: SDWORD = 0;
#[no_mangle]
pub static mut zMin: SDWORD = 0;
#[no_mangle]
pub static mut worldMax: SDWORD = 0;
#[no_mangle]
pub static mut worldMin: SDWORD = 0;
//type of object held
//pointer to the object
//function prototypes
/* reset object list */
/* code */
/* reset object list */
#[no_mangle]
pub unsafe extern "C" fn bucketSetupList() -> BOOL {
    let mut i: UDWORD = 0;
    zMax = 0x80000000 as libc::c_uint as SDWORD;
    zMin = 0x7fffffff as libc::c_int;
    worldMax = 0x80000000 as libc::c_uint as SDWORD;
    worldMin = 0x7fffffff as libc::c_int;
    //reset resource
    resourceCounter = 0 as libc::c_int as UDWORD;
    //reset buckets
    i = 0 as libc::c_int as UDWORD;
    while i < 8000 as libc::c_int as libc::c_uint {
        bucketArray[i as usize] = 0 as *mut BUCKET_TAG;
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* add an object to the current render list */
/* add an object to the current render list */
#[no_mangle]
pub unsafe extern "C" fn bucketAddTypeToList(mut objectType: RENDER_TYPE,
                                             mut pObject: *mut libc::c_void)
 -> BOOL {
    let mut newTag: *mut BUCKET_TAG = 0 as *mut BUCKET_TAG;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut z: SDWORD = 0;
    /*	switch (objectType)
	{
	case RENDER_STRUCTURE:
	case RENDER_DROID:
		return TRUE;
		break;
	case RENDER_EFFECT:
	case RENDER_EXPLOSION:
	case RENDER_GRAVITON:
	case RENDER_SMOKE:
	case RENDER_PROJECTILE:
	case RENDER_PROJECTILE_TRANSPARENT:
	case RENDER_PARTICLE:
	case RENDER_FEATURE:
	case RENDER_PROXMSG:
	case RENDER_SHADOW:
	case RENDER_ANIMATION:
	case RENDER_WATERTILE:
	case RENDER_MIST:
	case RENDER_DELIVPOINT:
	case RENDER_TILE:
		break;
	}*/
    //get next Tag
    newTag =
        &mut *tagResource.as_mut_ptr().offset(resourceCounter as isize) as
            *mut BUCKET_TAG;
    if resourceCounter >= 4000 as libc::c_int as libc::c_uint {
        //		DBPRINTF(("bucket sort too many objects"));
		/* Just get out if there's too much to render already...! */
        return 1 as libc::c_int
    }
    resourceCounter = resourceCounter.wrapping_add(1);
    if resourceCounter <= 4000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"bucketAddTypeToList: too many objects\x00" as *const u8 as
                  *const libc::c_char);
    };
    if resourceCounter <= 4000 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"bucket3d.c\x00" as *const u8 as *const libc::c_char,
              135 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 20],
                                        &[libc::c_char; 20]>(b"bucketAddTypeToList\x00")).as_ptr(),
              b"resourceCounter <= NUM_OBJECTS\x00" as *const u8 as
                  *const libc::c_char);
    };
    //put the object data into the tag
    (*newTag).objectType = objectType;
    (*newTag).pObject = pObject;
    if objectType as libc::c_uint ==
           RENDER_EFFECT as libc::c_int as libc::c_uint &&
           ((*(pObject as *mut EFFECT)).group as libc::c_int ==
                EFFECT_EXPLOSION as libc::c_int ||
                (*(pObject as *mut EFFECT)).group as libc::c_int ==
                    EFFECT_CONSTRUCTION as libc::c_int ||
                (*(pObject as *mut EFFECT)).group as libc::c_int ==
                    EFFECT_SMOKE as libc::c_int ||
                (*(pObject as *mut EFFECT)).group as libc::c_int ==
                    EFFECT_FIREWORK as libc::c_int) {
        z = bucketCalculateZ(objectType, pObject)
    } else if objectType as libc::c_uint ==
                  RENDER_SHADOW as libc::c_int as libc::c_uint {
        z = bucketCalculateZ(objectType, pObject)
    } else if objectType as libc::c_uint ==
                  RENDER_PROJECTILE_TRANSPARENT as libc::c_int as libc::c_uint
     {
        z = bucketCalculateZ(objectType, pObject)
    } else if objectType as libc::c_uint ==
                  RENDER_PROXMSG as libc::c_int as libc::c_uint {
        z = bucketCalculateZ(objectType, pObject)
    } else if objectType as libc::c_uint ==
                  RENDER_WATERTILE as libc::c_int as libc::c_uint {
        z = bucketCalculateZ(objectType, pObject)
    } else {
        //		else if(objectType == RENDER_PARTICLE)
//		{
//			z = bucketCalculateZ(objectType, pObject);
//		}
        /*		else if(objectType == RENDER_PROJECTILE)//rendered solid so sort by state
		{
			z = bucketCalculateZ(objectType, pObject);
		}
*/
        z = bucketCalculateState(objectType, pObject)
    }
    if z < 0 as libc::c_int {
        /* Object will not be render - has been clipped! */
        if objectType as libc::c_uint ==
               RENDER_DROID as libc::c_int as libc::c_uint {
            /* Won't draw selection boxes */
            psDroid = pObject as *mut DROID;
            (*psDroid).sDisplay.frameNumber = 0 as libc::c_int as UDWORD
        } else if objectType as libc::c_uint ==
                      RENDER_STRUCTURE as libc::c_int as libc::c_uint {
            /* Won't draw selection boxes */
            psStructure = pObject as *mut STRUCTURE;
            (*psStructure).sDisplay.frameNumber = 0 as libc::c_int as UDWORD
        }
        return 1 as libc::c_int
    }
    /* Maintain biggest*/
    if z > worldMax { worldMax = z } else if z < worldMin { worldMin = z }
    /* get min and max */
    if z > zMax { zMax = z } else if z < zMin { zMin = z }
    z = z * 8000 as libc::c_int / 32000 as libc::c_int;
    if z >= 8000 as libc::c_int { z = 8000 as libc::c_int - 1 as libc::c_int }
    //add tag to bucketArray
    (*newTag).psNextTag = bucketArray[z as usize];
    (*newTag).actualZ = z;
    bucketArray[z as usize] = newTag;
    return 1 as libc::c_int;
}
/* render Objects in list */
/* render Objects in list */
#[no_mangle]
pub unsafe extern "C" fn bucketRenderCurrentList() -> BOOL {
    let mut z: SDWORD = 0;
    let mut thisTag: *mut BUCKET_TAG = 0 as *mut BUCKET_TAG;
    z = 8000 as libc::c_int - 1 as libc::c_int;
    while z >= 0 as libc::c_int {
        // render from back to front
        thisTag = bucketArray[z as usize];
        while !thisTag.is_null() {
            match (*thisTag).objectType as libc::c_uint {
                16 => { renderParticle((*thisTag).pObject as *mut ATPART); }
                9 => { renderEffect((*thisTag).pObject as *mut EFFECT); }
                0 => { renderDroid((*thisTag).pObject as *mut DROID); }
                6 => {
                    renderShadow((*thisTag).pObject as *mut DROID,
                                 getImdFromIndex(MI_SHADOW as libc::c_int as
                                                     UDWORD));
                }
                1 => {
                    renderStructure((*thisTag).pObject as *mut STRUCTURE);
                }
                2 => { renderFeature((*thisTag).pObject as *mut FEATURE); }
                3 => {
                    renderProximityMsg((*thisTag).pObject as
                                           *mut PROXIMITY_DISPLAY);
                }
                12 => {
                    drawTerrainTile((*((*thisTag).pObject as
                                           *mut TILE_BUCKET)).i,
                                    (*((*thisTag).pObject as
                                           *mut TILE_BUCKET)).j);
                }
                13 => {
                    drawTerrainWaterTile((*((*thisTag).pObject as
                                                *mut TILE_BUCKET)).i,
                                         (*((*thisTag).pObject as
                                                *mut TILE_BUCKET)).j);
                }
                4 | 5 => {
                    renderProjectile((*thisTag).pObject as *mut PROJ_OBJECT);
                }
                7 => {
                    renderAnimComponent((*thisTag).pObject as
                                            *mut COMPONENT_OBJECT);
                }
                15 => {
                    //				case RENDER_GRAVITON:
//					renderGraviton((GRAVITON*)thisTag->pObject);
//				break;
//				case RENDER_SMOKE:
//					renderSmoke((PARTICLE*)thisTag->pObject);
//				break;
                    renderDeliveryPoint((*thisTag).pObject as
                                            *mut FLAG_POSITION);
                }
                14 | _ => { }
            }
            thisTag = (*thisTag).psNextTag
        }
        //reset the bucket array as we go
        bucketArray[z as usize] = 0 as *mut BUCKET_TAG;
        z -= 1
    }
    //	testRender();
    //reset the tag array
    resourceCounter = 0 as libc::c_int as UDWORD;
    //	iV_NumberOut(worldMax,100,100,255);
 //	iV_NumberOut(worldMin,100,200,255);
    zMax = 0x80000000 as libc::c_uint as SDWORD;
    zMin = 0x7fffffff as libc::c_int;
    worldMax = 0x80000000 as libc::c_uint as SDWORD;
    worldMin = 0x7fffffff as libc::c_int;
    return 1 as libc::c_int;
}
/* function prototypes */
#[no_mangle]
pub unsafe extern "C" fn bucketCalculateZ(mut objectType: RENDER_TYPE,
                                          mut pObject: *mut libc::c_void)
 -> SDWORD {
    let mut z: SDWORD = 0 as libc::c_int;
    let mut radius: SDWORD = 0;
    let mut px: SDWORD = 0;
    let mut pz: SDWORD = 0;
    let mut pixel: iPoint = iPoint{x: 0, y: 0,};
    let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut droidSize: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psBStats: *mut BODY_STATS = 0 as *mut BODY_STATS;
    let mut psSimpObj: *mut SIMPLE_OBJECT = 0 as *mut SIMPLE_OBJECT;
    let mut psCompObj: *mut COMPONENT_OBJECT = 0 as *mut COMPONENT_OBJECT;
    let mut pImd: *mut iIMDShape = 0 as *mut iIMDShape;
    pie_MatBegin();
    match objectType as libc::c_uint {
        16 => {
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            position.x =
                (*(pObject as *mut ATPART)).position.x as SDWORD as UDWORD as
                    int32;
            position.y =
                (*(pObject as *mut ATPART)).position.y as SDWORD as UDWORD as
                    int32;
            position.z =
                (*(pObject as *mut ATPART)).position.z as SDWORD as UDWORD as
                    int32;
            position.x =
                ((position.x - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub((position.z
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as SDWORD;
            position.y = position.y;
            /* 16 below is HACK!!! */
            z = pie_RotProj(&mut position, &mut pixel) - 16 as libc::c_int;
            if z > 0 as libc::c_int {
                //particle use the image radius
                radius = (*(*(pObject as *mut ATPART)).imd).radius;
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        4 | 5 => {
            //not depth sorted
            //			((PROJ_OBJECT*)pObject)->psWStats;
			/* these guys should never be added to the list anyway */
            if (*(*(pObject as *mut PROJ_OBJECT)).psWStats).weaponSubClass as
                   libc::c_uint == WSC_FLAME as libc::c_int as libc::c_uint ||
                   (*(*(pObject as *mut PROJ_OBJECT)).psWStats).weaponSubClass
                       as libc::c_uint ==
                       WSC_COMMAND as libc::c_int as libc::c_uint ||
                   (*(*(pObject as *mut PROJ_OBJECT)).psWStats).weaponSubClass
                       as libc::c_uint ==
                       WSC_EMP as libc::c_int as libc::c_uint {
                /* We don't do projectiles from these guys, cos there's an effect instead */
                z = -(1 as libc::c_int)
            } else {
                //the weapon stats holds the reference to which graphic to use
                pImd =
                    (*(*(pObject as
                             *mut PROJ_OBJECT)).psWStats).pInFlightGraphic;
                px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
                pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
                /* Translate */
                pie_TRANSLATE(px, 0 as libc::c_int, -pz);
                psSimpObj = pObject as *mut SIMPLE_OBJECT;
                position.x =
                    (((*psSimpObj).x as libc::c_int - player.p.x) as
                         libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint))
                        as int32;
                position.z =
                    terrainMidY.wrapping_mul(128 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(((*psSimpObj).y
                                                                                 as
                                                                                 libc::c_int
                                                                                 -
                                                                                 player.p.z)
                                                                                as
                                                                                libc::c_uint)
                        as int32;
                position.y = (*psSimpObj).z as int32;
                z = pie_RotProj(&mut position, &mut pixel);
                if z > 0 as libc::c_int {
                    //particle use the image radius
                    radius = (*pImd).radius;
                    radius *= (1 as libc::c_int) << 12 as libc::c_int;
                    radius /= z;
                    if pixel.x + radius < 0 as libc::c_int ||
                           pixel.x - radius >
                               pie_GetVideoBufferWidth() as SDWORD ||
                           pixel.y + radius < 0 as libc::c_int ||
                           pixel.y - radius >
                               pie_GetVideoBufferHeight() as SDWORD {
                        z = -(1 as libc::c_int)
                    }
                }
            }
        }
        1 => {
            //not depth sorted
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            psSimpObj = pObject as *mut SIMPLE_OBJECT;
            position.x =
                (((*psSimpObj).x as libc::c_int - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub(((*psSimpObj).y
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as int32;
            //if((objectType == RENDER_STRUCTURE) AND (((STRUCTURE*)pObject)->
			//	pStructureType->type >= REF_DEFENSE) AND
			//	(((STRUCTURE*)pObject)->pStructureType->type<=REF_TOWER4))
            if objectType as libc::c_uint ==
                   RENDER_STRUCTURE as libc::c_int as libc::c_uint &&
                   ((*(*(pObject as *mut STRUCTURE)).pStructureType).type_0 ==
                        REF_DEFENSE as libc::c_int as libc::c_uint ||
                        (*(*(pObject as
                                 *mut STRUCTURE)).pStructureType).type_0 ==
                            REF_WALL as libc::c_int as libc::c_uint ||
                        (*(*(pObject as
                                 *mut STRUCTURE)).pStructureType).type_0 ==
                            REF_WALLCORNER as libc::c_int as libc::c_uint) {
                position.y =
                    (*psSimpObj).z as libc::c_int + 64 as libc::c_int;
                radius = (*(*(pObject as *mut STRUCTURE)).sDisplay.imd).radius
                //walls guntowers and tank traps clip tightly
            } else {
                position.y = (*psSimpObj).z as int32;
                radius =
                    (*(*(pObject as *mut STRUCTURE)).sDisplay.imd).radius *
                        2 as libc::c_int
                //other building clipping not so close
            }
            z = pie_RotProj(&mut position, &mut pixel);
            if z > 0 as libc::c_int {
                //particle use the image radius
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        2 => {
            //not depth sorted
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            psSimpObj = pObject as *mut SIMPLE_OBJECT;
            position.x =
                (((*psSimpObj).x as libc::c_int - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub(((*psSimpObj).y
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as int32;
            position.y = (*psSimpObj).z as libc::c_int + 2 as libc::c_int;
            z = pie_RotProj(&mut position, &mut pixel);
            if z > 0 as libc::c_int {
                //particle use the image radius
                radius = (*(*(pObject as *mut FEATURE)).sDisplay.imd).radius;
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        7 => {
            //not depth sorted
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            psCompObj = pObject as *mut COMPONENT_OBJECT;
            psSimpObj = (*psCompObj).psParent as *mut SIMPLE_OBJECT;
            position.x =
                (((*psSimpObj).x as libc::c_int - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub(((*psSimpObj).y
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as int32;
            position.y = (*psSimpObj).z as int32;
            /* object offset translation */
            position.x += (*(*psCompObj).psShape).ocen.x;
            position.y += (*(*psCompObj).psShape).ocen.z;
            position.z -= (*(*psCompObj).psShape).ocen.y;
            /* object (animation) translations - ivis z and y flipped */
            pie_TRANSLATE((*psCompObj).position.x, (*psCompObj).position.z,
                          (*psCompObj).position.y);
            /* object (animation) rotations */
            pie_MatRotY(-(*psCompObj).orientation.z);
            pie_MatRotZ(-(*psCompObj).orientation.y);
            pie_MatRotX(-(*psCompObj).orientation.x);
            z = pie_RotProj(&mut position, &mut pixel)
        }
        0 | 6 => {
            psDroid = pObject as *mut DROID;
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            psSimpObj = pObject as *mut SIMPLE_OBJECT;
            position.x =
                (((*psSimpObj).x as libc::c_int - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub(((*psSimpObj).y
                                                                             as
                                                                             libc::c_int
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as int32;
            position.y = (*psSimpObj).z as int32;
            if objectType as libc::c_uint ==
                   RENDER_SHADOW as libc::c_int as libc::c_uint {
                position.y += 4 as libc::c_int
            }
            psBStats =
                asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int
                                                         as usize].nStat as
                                       libc::c_int as isize);
            droidSize = (*(*psBStats).pIMD).radius as UDWORD;
            z =
                (pie_RotProj(&mut position, &mut pixel) as
                     libc::c_uint).wrapping_sub(droidSize.wrapping_mul(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint))
                    as SDWORD;
            if z > 0 as libc::c_int {
                //particle use the image radius
                radius = droidSize as SDWORD;
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        3 => {
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            if (*(pObject as *mut PROXIMITY_DISPLAY)).type_0 as libc::c_uint
                   == POS_PROXDATA as libc::c_int as libc::c_uint {
                position.x =
                    (*((*((*(*(pObject as
                                   *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                              as *mut VIEWDATA)).pData as
                           *mut VIEW_PROXIMITY)).x.wrapping_sub(player.p.x as
                                                                    libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint))
                        as int32;
                position.z =
                    terrainMidY.wrapping_mul(128 as libc::c_int as
                                                 libc::c_uint).wrapping_sub((*((*((*(*(pObject
                                                                                           as
                                                                                           *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                                                                                      as
                                                                                      *mut VIEWDATA)).pData
                                                                                   as
                                                                                   *mut VIEW_PROXIMITY)).y.wrapping_sub(player.p.z
                                                                                                                            as
                                                                                                                            libc::c_uint))
                        as int32;
                position.y =
                    (*((*((*(*(pObject as
                                   *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                              as *mut VIEWDATA)).pData as
                           *mut VIEW_PROXIMITY)).z as int32
            } else if (*(pObject as *mut PROXIMITY_DISPLAY)).type_0 as
                          libc::c_uint ==
                          POS_PROXOBJ as libc::c_int as libc::c_uint {
                position.x =
                    (((*((*(*(pObject as
                                  *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                             as *mut BASE_OBJECT)).x as libc::c_int -
                          player.p.x) as
                         libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint))
                        as int32;
                position.z =
                    terrainMidY.wrapping_mul(128 as libc::c_int as
                                                 libc::c_uint).wrapping_sub(((*((*(*(pObject
                                                                                         as
                                                                                         *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                                                                                    as
                                                                                    *mut BASE_OBJECT)).y
                                                                                 as
                                                                                 libc::c_int
                                                                                 -
                                                                                 player.p.z)
                                                                                as
                                                                                libc::c_uint)
                        as int32;
                position.y =
                    (*((*(*(pObject as
                                *mut PROXIMITY_DISPLAY)).psMessage).pViewData
                           as *mut BASE_OBJECT)).z as int32
            }
            z = pie_RotProj(&mut position, &mut pixel);
            if z > 0 as libc::c_int {
                //particle use the image radius
                pImd =
                    getImdFromIndex(MI_BLIP_ENEMY as libc::c_int as
                                        UDWORD); //use MI_BLIP_ENEMY as all are same radius
                radius = (*pImd).radius;
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        12 => { z = (*(pObject as *mut TILE_BUCKET)).depth }
        13 => { z = (*(pObject as *mut TILE_BUCKET)).depth }
        9 => {
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            position.x =
                (((*(pObject as *mut EFFECT)).position.x -
                      player.p.x as libc::c_float) as SDWORD as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                (terrainMidY.wrapping_mul(128 as libc::c_int as libc::c_uint)
                     as libc::c_float -
                     ((*(pObject as *mut EFFECT)).position.z -
                          player.p.z as libc::c_float)) as SDWORD;
            position.y = (*(pObject as *mut EFFECT)).position.y as SDWORD;
            /* 16 below is HACK!!! */
            z = pie_RotProj(&mut position, &mut pixel) - 16 as libc::c_int;
            if z > 0 as libc::c_int {
                //particle use the image radius
                pImd = (*(pObject as *mut EFFECT)).imd;
                if !pImd.is_null() {
                    radius = (*pImd).radius;
                    radius *= (1 as libc::c_int) << 12 as libc::c_int;
                    radius /= z;
                    if pixel.x + radius < 0 as libc::c_int ||
                           pixel.x - radius >
                               pie_GetVideoBufferWidth() as SDWORD ||
                           pixel.y + radius < 0 as libc::c_int ||
                           pixel.y - radius >
                               pie_GetVideoBufferHeight() as SDWORD {
                        z = -(1 as libc::c_int)
                    }
                }
            }
        }
        15 => {
            px = player.p.x & 128 as libc::c_int - 1 as libc::c_int;
            pz = player.p.z & 128 as libc::c_int - 1 as libc::c_int;
            /* Translate */
            pie_TRANSLATE(px, 0 as libc::c_int, -pz);
            position.x =
                (((*(pObject as *mut FLAG_POSITION)).coords.x - player.p.x) as
                     libc::c_uint).wrapping_sub(terrainMidX.wrapping_mul(128
                                                                             as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_uint))
                    as int32;
            position.z =
                terrainMidY.wrapping_mul(128 as libc::c_int as
                                             libc::c_uint).wrapping_sub(((*(pObject
                                                                                as
                                                                                *mut FLAG_POSITION)).coords.y
                                                                             -
                                                                             player.p.z)
                                                                            as
                                                                            libc::c_uint)
                    as int32;
            position.y = (*(pObject as *mut FLAG_POSITION)).coords.z;
            z = pie_RotProj(&mut position, &mut pixel);
            if z > 0 as libc::c_int {
                //particle use the image radius
                radius =
                    (*pAssemblyPointIMDs[(*(pObject as
                                                *mut FLAG_POSITION)).factoryType
                                             as
                                             usize][(*(pObject as
                                                           *mut FLAG_POSITION)).factoryInc
                                                        as usize]).radius;
                radius *= (1 as libc::c_int) << 12 as libc::c_int;
                radius /= z;
                if pixel.x + radius < 0 as libc::c_int ||
                       pixel.x - radius > pie_GetVideoBufferWidth() as SDWORD
                       || pixel.y + radius < 0 as libc::c_int ||
                       pixel.y - radius > pie_GetVideoBufferHeight() as SDWORD
                   {
                    z = -(1 as libc::c_int)
                }
            }
        }
        _ => { }
    }
    pie_MatEnd();
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn bucketCalculateState(mut objectType: RENDER_TYPE,
                                              mut pObject: *mut libc::c_void)
 -> SDWORD {
    let mut z: SDWORD = 0 as libc::c_int;
    let mut pie: *mut iIMDShape = 0 as *mut iIMDShape;
    if bucketCalculateZ(objectType, pObject) < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    match objectType as libc::c_uint {
        9 => {
            let mut current_block_5: u64;
            match (*(pObject as *mut EFFECT)).group as libc::c_int {
                5 | 0 | 1 => {
                    pie = (*(pObject as *mut EFFECT)).imd;
                    z = 8000 as libc::c_int - (*pie).texpage;
                    current_block_5 = 5399440093318478209;
                }
                2 => {
                    //				renderSmokeEffect(psEffect);
                    current_block_5 = 17170742539656529095;
                }
                4 => { current_block_5 = 17170742539656529095; }
                6 | 3 | 7 | _ => { current_block_5 = 13508210184899974930; }
            }
            match current_block_5 {
                17170742539656529095 =>
                //				renderGravitonEffect(psEffect);
                {
                    current_block_5 = 13508210184899974930;
                }
                _ => { }
            }
            match current_block_5 {
                13508210184899974930 =>
                //				renderBloodEffect(psEffect);
                {
                    z = 8000 as libc::c_int - 42 as libc::c_int
                }
                _ => { }
            }
        }
        0 => {
            pie =
                (*asBodyStats.offset((*(pObject as
                                            *mut DROID)).asBits[COMP_BODY as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nStat
                                         as
                                         isize)).pIMD; //stretch the dummy depth so its right when its compressed into the bucket array
            z = 8000 as libc::c_int - (*pie).texpage
        }
        1 => {
            pie = (*(pObject as *mut STRUCTURE)).sDisplay.imd;
            z = 8000 as libc::c_int - (*pie).texpage
        }
        2 => {
            pie = (*(pObject as *mut FEATURE)).sDisplay.imd;
            z = 8000 as libc::c_int - (*pie).texpage
        }
        3 => { z = 8000 as libc::c_int - 40 as libc::c_int }
        12 => { z = 8000 as libc::c_int - 1 as libc::c_int }
        13 => { z = 8000 as libc::c_int - 1 as libc::c_int }
        4 => {
            pie =
                (*(*(pObject as *mut PROJ_OBJECT)).psWStats).pInFlightGraphic;
            z = 8000 as libc::c_int - (*pie).texpage
        }
        7 => {
            pie = (*(pObject as *mut COMPONENT_OBJECT)).psShape;
            z = 8000 as libc::c_int - (*pie).texpage
        }
        15 => {
            pie =
                pAssemblyPointIMDs[(*(pObject as
                                          *mut FLAG_POSITION)).factoryType as
                                       usize][(*(pObject as
                                                     *mut FLAG_POSITION)).factoryInc
                                                  as usize];
            z = 8000 as libc::c_int - (*pie).texpage
        }
        _ => { }
    }
    z *= 32000 as libc::c_int / 8000 as libc::c_int;
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn testRender() {
    pie_SetDepthBufferStatus(DEPTH_CMP_ALWAYS_WRT_OFF);
    //render test line
    pie_Line(0 as libc::c_int, 0 as libc::c_int,
             pie_GetVideoBufferWidth() as SDWORD, 0 as libc::c_int,
             2 as libc::c_int as uint32);
    pie_Line(pie_GetVideoBufferWidth() as SDWORD, 0 as libc::c_int,
             pie_GetVideoBufferWidth() as SDWORD,
             pie_GetVideoBufferHeight() as SDWORD,
             2 as libc::c_int as uint32);
    pie_Line(pie_GetVideoBufferWidth() as SDWORD,
             pie_GetVideoBufferHeight() as SDWORD, 0 as libc::c_int,
             pie_GetVideoBufferHeight() as SDWORD,
             2 as libc::c_int as uint32);
    pie_Line(0 as libc::c_int, pie_GetVideoBufferHeight() as SDWORD,
             0 as libc::c_int, 0 as libc::c_int, 2 as libc::c_int as uint32);
    /*	pie_Line(320, 200, 320, 100, 2);
	pie_Line(301, 100, 319, 100, 2);
	pie_Line(319, 200, 301, 200, 2);
	pie_Line(300, 100, 300, 200, 2);

	pie_Box(520, 450, 540, 470, 2);
	pie_BoxFillIndex(560, 450, 580, 470, 2);
	pie_UniTransBoxFill(600, 450, 620, 470, 0x0000007f, 0x0f);


	pie_Box(400, 100, 463, 163, 2);

	pie_SetBilinear(FALSE);

	image.texPage = 0;
	image.tu = 0;
	image.tv = 0;
	image.tw = 63;
	image.th = 63;
	dest.x = 400;
	dest.y = 100;
	dest.w = 63;
	dest.h = 63;


	pie_DrawImage(&image, &dest, &style);

	image.texPage = 0;
	image.tu = 0;
	image.tv = 0;
	image.tw = 63;
	image.th = 31;
	dest.x = 500;
	dest.y = 100;
	dest.w = 63;
	dest.h = 31;


	pie_DrawImage(&image, &dest, &style);

	image.texPage = 0;
	image.tu = 0;
	image.tv = 0;
	image.tw = 31;
	image.th = 63;
	dest.x = 400;
	dest.y = 200;
	dest.w = 31;
	dest.h = 63;


	pie_DrawImage(&image, &dest, &style);
*/
    pie_SetDepthBufferStatus(DEPTH_CMP_LEQ_WRT_ON);
}
