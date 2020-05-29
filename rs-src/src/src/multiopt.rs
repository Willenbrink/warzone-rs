use ::libc;
extern "C" {
    pub type _formation;
    /* The optional user callback function */
    pub type _w_context;
    /* Allocate an object from a heap
 * Returns a pointer to the object if successful
 */
    #[no_mangle]
    fn heapAlloc(psHeap: *mut OBJ_HEAP, ppObject: *mut *mut libc::c_void)
     -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
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
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /*
 * droid.h
 *
 * Definitions for the droid object.
 *
 */
    // world->screen check - alex
    // percentage of body points remaining at which to repair droid automatically.
    // ditto, but this will repair much sooner..
    /*defines the % to decrease the illumination of a tile when building - gets set 
back when building is destroyed*/
//#define FOUNDATION_ILLUMIN		50
    //storage
    #[no_mangle]
    static mut apsDroidTemplates: [*mut DROID_TEMPLATE; 8];
    // remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    // the memory heap for templates
    #[no_mangle]
    static mut psTemplateHeap: *mut OBJ_HEAP;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    /*destroy a droid */
    #[no_mangle]
    fn killDroid(psDel: *mut DROID);
    /*
 * AI.h
 *
 * Definitions for the AI system structures
 *
 */
    // states of alliance between players
    // for setting values only.
    //alliance possibilities for games.
    //#define GROUP_WINS		2
    // alliances
    #[no_mangle]
    static mut alliances: [[UBYTE; 8]; 8];
    /*returns the current type of save game being loaded*/
    #[no_mangle]
    fn getSaveGameType() -> UDWORD;
    #[no_mangle]
    fn freeMessages();
    //flag used to check for power calculations to be done or not
    #[no_mangle]
    static mut powerCalculated: BOOL;
    /* Set the current position of a slider bar */
    #[no_mangle]
    fn widgSetSliderPos(psScreen: *mut W_SCREEN, id: UDWORD, pos: UWORD);
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
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
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    #[no_mangle]
    fn NETinit(bFirstCall: BOOL) -> BOOL;
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    #[no_mangle]
    fn NETclose() -> HRESULT;
    #[no_mangle]
    fn NETshutdown() -> HRESULT;
    #[no_mangle]
    fn NETsetGameFlags(flag: UDWORD, value: DWORD) -> BOOL;
    #[no_mangle]
    fn NETfindGame(asynchronously: BOOL) -> BOOL;
    #[no_mangle]
    fn NETjoinGame(gameNumber: UDWORD, playername: LPSTR) -> BOOL;
    #[no_mangle]
    fn NEThostGame(SessionName: LPSTR, PlayerName: LPSTR, one: DWORD,
                   two: DWORD, three: DWORD, four: DWORD, plyrs: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn NETplayerInfo() -> UDWORD;
    #[no_mangle]
    fn NETshutdownAudioCapture() -> BOOL;
    #[no_mangle]
    fn NETinitAudioCapture() -> BOOL;
    #[no_mangle]
    fn NETshutdownAudioPlayback() -> BOOL;
    // encryption
    #[no_mangle]
    fn NETsetKey(c1: UDWORD, c2: UDWORD, c3: UDWORD, c4: UDWORD) -> BOOL;
    #[no_mangle]
    fn NEThashVal(value: UDWORD) -> UCHAR;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* Add the reticule widgets to the widget screen */
    #[no_mangle]
    fn intAddReticule() -> BOOL;
    #[no_mangle]
    fn intRemoveReticule();
    /*
	Header file for component.c 
	Pumpkin Studios, EIDOS Interactive. 
*/
    #[no_mangle]
    static mut PlayerColour: [UBYTE; 8];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    // true when more than 1 player.
    #[no_mangle]
    static mut player2dpid: [DWORD; 8];
    // note this is of type DPID, not DWORD
    #[no_mangle]
    static mut openchannels: [BOOL; 8];
    #[no_mangle]
    fn getPlayerName(player: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn cameraToHome(player: UDWORD, scroll: BOOL) -> iVector;
    // A Player has joined the game.
    #[no_mangle]
    fn setupNewPlayer(dpid: DPID, player: UDWORD);
    // stuff to do when player joins.
    //extern BOOL UpdateClient				(DPID dest, UDWORD playerToSend);// send info about another player
    #[no_mangle]
    fn clearPlayer(player: UDWORD, quietly: BOOL, removeOil: BOOL);
    #[no_mangle]
    static mut bForceEditorLoaded: BOOL;
    // the current level descriptions
    #[no_mangle]
    static mut psLevels: *mut LEVEL_DATASET;
    #[no_mangle]
    static mut Force: FORCE;
    // add a droid (templ) to force
    #[no_mangle]
    fn useTheForce(bAddTempl: BOOL);
    #[no_mangle]
    fn loadForce(name: *mut libc::c_char) -> BOOL;
    // stat defs
    #[no_mangle]
    fn saveMultiStats(sFName: *mut STRING, sPlayerName: *mut STRING,
                      playerStats: *mut PLAYERSTATS) -> BOOL;
    // to disk 
    #[no_mangle]
    fn loadMultiStats(sPlayerName: *mut STRING, playerStats: *mut PLAYERSTATS)
     -> BOOL;
    // form disk
    #[no_mangle]
    fn getMultiStats(player: UDWORD, bLocal: BOOL) -> PLAYERSTATS;
    // get from net
    #[no_mangle]
    fn setMultiStats(playerDPID: DWORD, plStats: PLAYERSTATS, bLocal: BOOL)
     -> BOOL;
    #[no_mangle]
    fn updateMultiStatsGames();
    #[no_mangle]
    static mut sForceName: [libc::c_char; 256];
    #[no_mangle]
    static mut sPlayer: [libc::c_char; 128];
    // players (mid) box
    #[no_mangle]
    fn loadMapPreview();
    // deathmatch stuff
    #[no_mangle]
    fn addOilDrum(count: UDWORD) -> BOOL;
    /*
 * MultiOpt.c
 *
 * Alex Lee,97/98, Pumpkin Studios
 *
 * Routines for setting the game options and starting the init process.
 */
    // ////////////////////////////////////////////////////////////////////////////
// GUID for warzone lobby and MPATH stuff.  i hate this stuff.
    //Not really (going to be) used. -Qamly
    // ////////////////////////////////////////////////////////////////////////////
// External Variables
    #[no_mangle]
    static mut MultiForcesPath: [libc::c_char; 255];
    #[no_mangle]
    static mut buildTime: [libc::c_char; 8];
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type HRESULT = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type UCHAR = libc::c_uchar;
pub type DWORD = libc::c_int;
pub type LPSTR = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
/* !WIN32 */
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
/* Parse the res file */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
/*
 * Block.h
 *
 * Routines to allocate memory from one large block.
 * Any memory allocated is only available to be reallocated after
 * the whole block has been reset.
 */
// control whether the debugging block malloc is used
/* *********************************************************************************/
/*                    type definitions                                            */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
/*
 * Heap.h
 *
 * Interface to the heap memory routines.
 *
 * Overhead of using the heap is :
 *			24 bytes for the initial block
 *           4 bytes for the extension blocks
 *
 */
/* Include Mem.h to get the DEBUG_MALLOC #define - this controls whether
  * normal or debugging memory management is used.
  */
/* structure used to store the list of free heap objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _free_object {
    pub psNext: *mut _free_object,
}
pub type FREE_OBJECT = _free_object;
/* structure used to store the extra space allocated for the heap */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _heap_extension {
    pub pMemory: *mut UBYTE,
    pub psNext: *mut _heap_extension,
}
pub type HEAP_EXTENSION = _heap_extension;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obj_heap {
    pub objSize: UDWORD,
    pub initAlloc: UDWORD,
    pub extAlloc: UDWORD,
    pub psBlkHeap: *mut _block_heap,
    pub psFree: *mut FREE_OBJECT,
    pub pMemory: *mut UBYTE,
    pub psExt: *mut HEAP_EXTENSION,
}
pub type OBJ_HEAP = _obj_heap;
// Extension memory for the heap
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
/*To generate and supply power to other structures*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub powerOutput: UDWORD,
    pub powerMultiplier: UDWORD,
    pub criticalMassChance: UDWORD,
    pub criticalMassRadius: UDWORD,
    pub criticalMassDamage: UDWORD,
    pub radiationDecayTime: UDWORD,
}
pub type POWER_GEN_FUNCTION = _power_gen_function;
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
/*How long the radiation lasts n time cycles*/
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
/*To manufacture droids designed previously*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _production_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub capacity: UWORD,
    pub productionOutput: UWORD,
}
pub type PRODUCTION_FUNCTION = _production_function;
/*Droid Build Points Produced Per 
												  Build Cycle*/
//struct _propulsion_types*		propulsionType;		
	//UBYTE					propulsionType;		/*The type of propulsion the facility 
	//											  can produce*/
/*To research topics available*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub researchPoints: UDWORD,
}
pub type RESEARCH_FUNCTION = _research_function;
/*The number of research points added per 
									  research cycle*/
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
 * game.h	  
 *
 * load and save game routines.
 * Very likely to ALL change! All the headers are separately defined at the 
 * moment - they probably don't need to be - if no difference make into one. 
 * Also the struct defintions throughout the game could be re-ordered to contain 
 * the variables required for saving so that don't need to create a load more here!
 */
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
//the version number used in save games
//#define VERSION_1				1				//demo save games
//#define VERSION_2				2				//names saved for components/structures/features
//#define VERSION_3				3				//changes to SAVE_GAME
//#define VERSION_4				4				//changes to SAVE_GAME
//#define VERSION_5				5				//different types of save game added
//#define VERSION_6				6				//level name added to a user save game
//string ID names saved for comps an objects
//research status saved out for user saved games
//power and experience saved out for user saved games
//includes gateways and zones in .map file.
//newstyle save game with extending structure checked in 13 Nov.
//mission and order stuff checked in 20 Nov.
//odds and ends to 24 Nov. and hashed scripts
//
//
//beta save game
//objId and new struct stats included
//droid name savegame validity stamps
//alliances, colours, radar zoom
//MAX_NAME_ SIZE replaced by MAX_SAVE_NAME_SIZE
//timeStartHold saved out for research facilities
//asRundData
//limbo droids and camstart droids saved properly, no cluster save
//reinforceTime, droid move, droid resistance
//limbo droid, research module hold fixed, cleaned by Alex
//reArm pads
//unit not the "d" word, experience and repair facility currentPtsAdded
//rearm pads currentPtsAdded saved
//mission scroll limits saved
//script external variables saved
//mission cheat time saved
//factory secondary order saved
//skirmish save 
//used in the loadGame
pub type C2RustUnnamed_0 = libc::c_uint;
// User saved game - in the middle of a level
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed_0 = 4;
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed_0 = 3;
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed_0 = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed_0 = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed_0 = 0;
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _widget {
    pub formID: UDWORD,
    pub id: UDWORD,
    pub type_0: WIDGET_TYPE,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub display: WIDGET_DISPLAY,
    pub callback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub psNext: *mut _widget,
}
pub type WIDGET_CALLBACK
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context) -> ()>;
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
/* The common widget data */
/* ID of the widgets base form. */
/* The user set ID number for the widget */
/* This is returned when e.g. a button is pressed */
/* The widget type */
/* The style of the widget */
/* The location of the widget */
/* The size of the widget */
/* Display the widget */
/* User callback (if any) */
/* Pointer to a user data block (if any) */
/* User data (if any) */
/* Pointer to the next widget in the screen list */
/* The base widget data type */
pub type WIDGET = _widget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
/* The common widget data */
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
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
pub type FORCE_MEMBER = _forcemember;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _forcemember {
    pub pTempl: *mut DROID_TEMPLATE,
    pub psNext: *mut _forcemember,
}
pub type FORCE = _force;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _force {
    pub pForceTemplates: *mut DROID_TEMPLATE,
    pub pMembers: *mut FORCE_MEMBER,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _level_dataset {
    pub type_0: SWORD,
    pub players: SWORD,
    pub game: SWORD,
    pub pName: *mut STRING,
    pub dataDir: libc::c_int,
    pub apDataFiles: [*mut STRING; 9],
    pub psBaseData: *mut _level_dataset,
    pub psChange: *mut _level_dataset,
    pub psNext: *mut _level_dataset,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
// pointer to template to use for this droid
// Pointer to next template
// the WRF/WDG files needed for a particular level
// the WRF/WDG files needed for a particular level
pub type LEVEL_DATASET = _level_dataset;
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
// ////////////////////////////////////////////////////////////////////////////
// Local Functions
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// send complete game info set!
// dpid == 0 for no new players.
#[no_mangle]
pub unsafe extern "C" fn sendOptions(mut dest: DPID, mut play: UDWORD) {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; //add dpid array
    let mut checkval: UBYTE = 0; // exe's hash val. DONT SEND THE VAL ITSELF!
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut game as *mut MULTIPLAYERGAME as *const libc::c_void,
           ::std::mem::size_of::<MULTIPLAYERGAME>() as libc::c_ulong);
    m.size =
        ::std::mem::size_of::<MULTIPLAYERGAME>() as libc::c_ulong as
            libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player2dpid as *mut [DWORD; 8] as *const libc::c_void,
           ::std::mem::size_of::<[DWORD; 8]>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[DWORD; 8]>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut ingame.JoiningInProgress as *mut [BOOL; 8] as
               *const libc::c_void,
           ::std::mem::size_of::<[BOOL; 8]>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[BOOL; 8]>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    checkval = NEThashVal(NetPlay.cryptKey[0 as libc::c_int as usize]);
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut checkval as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut dest as *mut DPID as *const libc::c_void,
           ::std::mem::size_of::<DPID>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<DPID>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut play as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut PlayerColour as *mut [UBYTE; 8] as *const libc::c_void,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UBYTE; 8]>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut alliances as *mut [[UBYTE; 8]; 8] as *const libc::c_void,
           ::std::mem::size_of::<[[UBYTE; 8]; 8]>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[[UBYTE; 8]; 8]>()
                                            as libc::c_ulong) as
            libc::c_ushort as libc::c_ushort;
    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut ingame.numStructureLimits as *mut UDWORD as
               *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        (m.size as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as libc::c_ushort
            as libc::c_ushort;
    if ingame.numStructureLimits != 0 {
        memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               ingame.pStructureLimits as *const libc::c_void,
               ingame.numStructureLimits.wrapping_mul((::std::mem::size_of::<UBYTE>()
                                                           as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                                                           as
                                                                                           libc::c_ulong)));
        m.size =
            (m.size as
                 libc::c_uint).wrapping_add(ingame.numStructureLimits.wrapping_mul((::std::mem::size_of::<UBYTE>()
                                                                                        as
                                                                                        libc::c_ulong).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                                                                                        as
                                                                                                                        libc::c_ulong)))
                as UWORD
    }
    //
	// now add the wdg files that are being used.
	//
    m.type_0 = NET_OPTIONS as libc::c_int as libc::c_uchar; // send it.
    NETbcast(&mut m, 1 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn checkGameWdg(mut nm: *mut CHAR) -> BOOL {
    let mut lev: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    //
	// now check the wdg files that are being used.
	//
    // game.map must be available in xxx list.
    lev = psLevels;
    while !lev.is_null() {
        if strcmp((*lev).pName, nm) == 0 as libc::c_int {
            return 1 as libc::c_int
        }
        lev = (*lev).psNext
    }
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// options for a game. (usually recvd in frontend)
#[no_mangle]
pub unsafe extern "C" fn recvOptions(mut pMsg: *mut NETMSG) {
    let mut pos: UDWORD = 0 as libc::c_int as UDWORD; // get details.
    let mut play: UDWORD = 0;
    let mut id: UDWORD = 0;
    let mut newPl: DPID = 0;
    let mut checkval: UBYTE = 0;
    memcpy(&mut game as *mut MULTIPLAYERGAME as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<MULTIPLAYERGAME>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<MULTIPLAYERGAME>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    if strncmp(game.version.as_mut_ptr() as *mut CHAR, buildTime.as_mut_ptr(),
               8 as libc::c_int as libc::c_uint) != 0 as libc::c_int {
        debug(LOG_ERROR,
              b"Host is running a different version of Warzone2100.\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    if ingame.numStructureLimits != 0 {
        // free old limits.
        ingame.numStructureLimits = 0 as libc::c_int as UDWORD;
        memFreeRelease(ingame.pStructureLimits as *mut libc::c_void);
        ingame.pStructureLimits = 0 as *mut UBYTE
    }
    memcpy(&mut player2dpid as *mut [DWORD; 8] as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[DWORD; 8]>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[DWORD; 8]>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut ingame.JoiningInProgress as *mut [BOOL; 8] as
               *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[BOOL; 8]>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[BOOL; 8]>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut checkval as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UBYTE>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    /*
	// This was set to a fixed value in earlier versions of post-Pumpkin code.
	// Commenting out to avoid confusion. Should probably be removed. - Per
	if(checkval != NEThashVal(NetPlay.cryptKey[0]))
	{

		DBERROR(("Host Binary is different from this one. Cheating?"));
	}
*/
    memcpy(&mut newPl as *mut DPID as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<DPID>() as
               libc::c_ulong); // malloc some room
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<DPID>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut play as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut PlayerColour as *mut [UBYTE; 8] as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[UBYTE; 8]>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[UBYTE; 8]>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut alliances as *mut [[UBYTE; 8]; 8] as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<[[UBYTE; 8]; 8]>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<[[UBYTE; 8]; 8]>()
                                            as libc::c_ulong) as UDWORD as
            UDWORD;
    memcpy(&mut ingame.numStructureLimits as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pos =
        (pos as
             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>() as
                                            libc::c_ulong) as UDWORD as
            UDWORD;
    if ingame.numStructureLimits != 0 {
        ingame.pStructureLimits =
            memMallocRelease(ingame.numStructureLimits.wrapping_mul((::std::mem::size_of::<UDWORD>()
                                                                         as
                                                                         libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                                         as
                                                                                                         libc::c_ulong)))
                as *mut UBYTE;
        memcpy(ingame.pStructureLimits as *mut libc::c_void,
               &mut *(*pMsg).body.as_mut_ptr().offset(pos as isize) as
                   *mut libc::c_char as *const libc::c_void,
               ingame.numStructureLimits.wrapping_mul((::std::mem::size_of::<UDWORD>()
                                                           as
                                                           libc::c_ulong).wrapping_add(::std::mem::size_of::<UBYTE>()
                                                                                           as
                                                                                           libc::c_ulong)));
    }
    // process
    if newPl != 0 as libc::c_int {
        if newPl == NetPlay.dpidPlayer {
            // it's us thats new
            selectedPlayer = play; // select player
            // turn off any power requirements.
            NETplayerInfo(); // get player info
            powerCalculated = 0 as libc::c_int
        } else {
            // someone else is joining.
            setupNewPlayer(newPl, play);
        }
    }
    // do the skirmish slider settings if they are up,
    id = 0 as libc::c_int as UDWORD;
    while id < 8 as libc::c_int as libc::c_uint {
        if !widgGetFromID(psWScreen,
                          (10313 as libc::c_int as
                               libc::c_uint).wrapping_add(id)).is_null() {
            widgSetSliderPos(psWScreen,
                             (10313 as libc::c_int as
                                  libc::c_uint).wrapping_add(id),
                             game.skDiff[id as usize] as UWORD);
        }
        id = id.wrapping_add(1)
    }
    if checkGameWdg(game.map.as_mut_ptr()) == 0 {
        // request the map from the host. NET_REQUESTMAP
        let mut m: NETMSG =
            NETMSG{size: 0,
                   paddedBytes: 0,
                   type_0: 0,
                   destination: 0,
                   body: [0; 8000],};
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut NetPlay.dpidPlayer as *mut DPID as *const libc::c_void,
               ::std::mem::size_of::<DPID>() as libc::c_ulong);
        m.type_0 = NET_REQUESTMAP as libc::c_int as libc::c_uchar;
        m.size = 4 as libc::c_int as libc::c_ushort;
        NETbcast(&mut m, 1 as libc::c_int);
        addConsoleMessage(b"MAP REQUESTED!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
    } else { loadMapPreview(); };
}
// ////////////////////////////////////////////////////////////////////////////
// Host Campaign.
#[no_mangle]
pub unsafe extern "C" fn hostCampaign(mut sGame: *mut STRING,
                                      mut sPlayer_0: *mut STRING) -> BOOL {
    let mut playerStats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    let mut pl: UDWORD = 0;
    let mut numpl: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    freeMessages();
    if NetPlay.bLobbyLaunched == 0 {
        NEThostGame(sGame, sPlayer_0, game.type_0 as DWORD, 0 as libc::c_int,
                    0 as libc::c_int, 0 as libc::c_int,
                    game.maxPlayers as UDWORD);
        // temporary stuff
    } else {
        NETsetGameFlags(1 as libc::c_int as UDWORD, game.type_0 as DWORD);
        // 2 is average ping
        NETsetGameFlags(3 as libc::c_int as UDWORD,
                        0 as libc::c_int); //pick a player
        NETsetGameFlags(4 as libc::c_int as UDWORD,
                        0 as libc::c_int); // add ourselves to the array.
    } // enable messages
    i = 0 as libc::c_int as UDWORD; // stats stuff
    while i < 8 as libc::c_int as libc::c_uint {
        player2dpid[i as usize] = 0 as libc::c_int;
        i = i.wrapping_add(1)
    }
    pl = (rand() % game.maxPlayers as libc::c_int) as UDWORD;
    player2dpid[pl as usize] = NetPlay.dpidPlayer;
    selectedPlayer = pl;
    ingame.localJoiningInProgress = 1 as libc::c_int;
    ingame.JoiningInProgress[selectedPlayer as usize] = 1 as libc::c_int;
    bMultiPlayer = 1 as libc::c_int;
    loadMultiStats(sPlayer_0, &mut playerStats);
    setMultiStats(NetPlay.dpidPlayer, playerStats, 0 as libc::c_int);
    setMultiStats(NetPlay.dpidPlayer, playerStats, 1 as libc::c_int);
    if NetPlay.bComms == 0 {
        NETplayerInfo();
        strcpy(NetPlay.players[0 as libc::c_int as usize].name.as_mut_ptr(),
               sPlayer_0);
        numpl = 1 as libc::c_int as UDWORD
    } else { numpl = NETplayerInfo() }
    // may be more than one player already. check and resolve!
    if numpl > 1 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            if NetPlay.players[j as usize].dpid != 0 &&
                   NetPlay.players[j as usize].dpid != NetPlay.dpidPlayer {
                i = 0 as libc::c_int as UDWORD;
                while player2dpid[i as usize] != 0 as libc::c_int {
                    i = i.wrapping_add(1)
                }
                player2dpid[i as usize] = NetPlay.players[j as usize].dpid
            }
            j = j.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Join Campaign
#[no_mangle]
pub unsafe extern "C" fn joinCampaign(mut gameNumber: UDWORD,
                                      mut sPlayer_0: *mut STRING) -> BOOL {
    let mut playerStats: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,};
    if ingame.localJoiningInProgress == 0 {
        //		game.type = CAMPAIGN;
        if NetPlay.bLobbyLaunched == 0 {
            NETjoinGame(gameNumber, sPlayer_0);
            // join
        }
        ingame.localJoiningInProgress = 1 as libc::c_int;
        loadMultiStats(sPlayer_0, &mut playerStats);
        setMultiStats(NetPlay.dpidPlayer, playerStats, 0 as libc::c_int);
        setMultiStats(NetPlay.dpidPlayer, playerStats, 1 as libc::c_int);
        return 0 as libc::c_int
    }
    bMultiPlayer = 1 as libc::c_int;
    return 1 as libc::c_int;
}
//BOOL		hostArena			(STRING *sGame,		STRING *sPlayer);
//BOOL		joinArena			(UDWORD gameNumber, STRING *playername);
// ////////////////////////////////////////////////////////////////////////////
// HostArena
/*
BOOL hostArena(STRING *sGame, STRING *sPlayer)
{
	PLAYERSTATS playerStats;
	UDWORD		numpl,i,j,pl;

	game.type = DMATCH;

	freeMessages();
	if(!NetPlay.bLobbyLaunched)
	{
		NEThostGame(sGame,sPlayer,DMATCH,0,0,0,game.maxPlayers);			// temporary stuff
	}
	else
	{
		NETsetGameFlags(1,DMATCH);
	// 2 is average ping
		NETsetGameFlags(3,0);
		NETsetGameFlags(4,0);
	}

	bMultiPlayer = TRUE;

	for(i=0;i<MAX_PLAYERS;i++)
	{
		player2dpid[i] =0;
	}

	//pick a player
#if 0
	pl =0;
#else
	pl = rand()%game.maxPlayers;
#endif

	player2dpid[pl] = NetPlay.dpidPlayer;							// add ourselves to the array.
	selectedPlayer = pl;

	ingame.localJoiningInProgress = TRUE;
	ingame.JoiningInProgress[selectedPlayer] = TRUE;

	loadMultiStats(sPlayer,&playerStats);						// load player stats!
	setMultiStats(NetPlay.dpidPlayer,playerStats,FALSE);
	setMultiStats(NetPlay.dpidPlayer,playerStats,TRUE);

	numpl = NETplayerInfo(NULL);

	// may be more than one player already. check and resolve!
	if(numpl >1)
	{
		for(j = 0;j<MAX_PLAYERS;j++)
		{
			if(NetPlay.players[j].dpid && (NetPlay.players[j].dpid != NetPlay.dpidPlayer))
			{
				for(i = 0;player2dpid[i] != 0;i++);
				player2dpid[i] = NetPlay.players[j].dpid;
				ingame.JoiningInProgress[i] = TRUE;
			}
		}
	}

	return TRUE;
}

// ////////////////////////////////////////////////////////////////////////////
// JoinArena.
BOOL joinArena(UDWORD gameNumber, STRING *playerName)
{
	PLAYERSTATS	playerStats;

	if(!ingame.localJoiningInProgress)
	{
		game.type = DMATCH;
		if(!NetPlay.bLobbyLaunched)
		{
			NETjoinGame(NetPlay.games[gameNumber].desc.guidInstance,playerName);	// join
		}
		ingame.localJoiningInProgress	= TRUE;

		loadMultiStats(playerName,&playerStats);
		setMultiStats(NetPlay.dpidPlayer,playerStats,FALSE);
		setMultiStats(NetPlay.dpidPlayer,playerStats,TRUE);
		return FALSE;
	}

	bMultiPlayer = TRUE;
	return TRUE;
}
*/
// ////////////////////////////////////////////////////////////////////////////
// Lobby launched. fires the correct routine when the game was lobby launched.
#[no_mangle]
pub unsafe extern "C" fn LobbyLaunched() -> BOOL {
    let mut i: UDWORD = 0;
    let mut pl: PLAYERSTATS =
        {
            let mut init =
                PLAYERSTATS{played: 0 as libc::c_int,
                            wins: 0,
                            loses: 0,
                            totalKills: 0,
                            totalScore: 0,
                            recentKills: 0,
                            recentScore: 0,
                            killsToAdd: 0,
                            scoreToAdd: 0,};
            init
        };
    // set the player info as soon as possible to avoid screwy scores appearing elsewhere.
    NETplayerInfo();
    NETfindGame(1 as libc::c_int);
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint &&
              NetPlay.players[i as usize].dpid != NetPlay.dpidPlayer {
        i = i.wrapping_add(1)
    }
    if loadMultiStats(NetPlay.players[i as usize].name.as_mut_ptr(), &mut pl)
           == 0 {
        return 0 as libc::c_int
        // cheating was detected, so fail.
    }
    setMultiStats(NetPlay.dpidPlayer, pl, 0 as libc::c_int);
    setMultiStats(NetPlay.dpidPlayer, pl, 1 as libc::c_int);
    // setup text boxes on multiplay screen.
    strcpy(sPlayer.as_mut_ptr() as *mut STRING,
           NetPlay.players[i as usize].name.as_mut_ptr());
    strcpy(game.name.as_mut_ptr(),
           NetPlay.games[0 as libc::c_int as usize].name.as_mut_ptr());
    return 1 as libc::c_int;
}
// for Init.c
//only once.
// ////////////////////////////////////////////////////////////////////////////
// Init and shutdown routines
#[no_mangle]
pub unsafe extern "C" fn lobbyInitialise() -> BOOL {
    if NETinit(1 as libc::c_int) == 0 {
        // initialise, may change guid.
        return 0 as libc::c_int
    }
    // setup the encryption key
    // RODZ : hashing the file is no more an option.
	// hash the file to get the key.and catch out the exe patchers.
//#ifdef DEBUG
    NETsetKey(0xdaf456 as libc::c_int as UDWORD,
              0xb72a5 as libc::c_int as UDWORD,
              0x114d0 as libc::c_int as UDWORD,
              0x2a17 as libc::c_int as UDWORD);
    //#else
//	NETsetKey(NEThashFile("warzone.exe"), 0xb72a5, 0x114d0, 0x2a7);
//#endif
    if NetPlay.bLobbyLaunched != 0 {
        // now check for lobby launching..
        if LobbyLaunched() == 0 { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn multiInitialise() -> BOOL {
    // NET AUDIO CAPTURE
    NETinitAudioCapture();
    //Disabled for now.  (returns FALSE always anyway) --Qamly
    return 1 as libc::c_int;
    // use the menus dumbass.
}
//only once.
// ////////////////////////////////////////////////////////////////////////////
// say goodbye to everyone else
#[no_mangle]
pub unsafe extern "C" fn sendLeavingMsg() -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut bHost: UBYTE = NetPlay.bHost as UBYTE;
    // send a leaving message, This resolves a problem with tcpip which
	// occasionally doesn't automatically notice a player leaving.
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut *player2dpid.as_mut_ptr().offset(selectedPlayer as isize) as
               *mut DWORD as *const libc::c_void,
           ::std::mem::size_of::<DWORD>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut bHost as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_LEAVING as libc::c_int as libc::c_uchar;
    NETbcast(&mut m, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// for Init.c
// ////////////////////////////////////////////////////////////////////////////
// called in Init.c to shutdown the whole netgame gubbins.
#[no_mangle]
pub unsafe extern "C" fn multiShutdown() -> BOOL {
    let mut pF: *mut FORCE_MEMBER = 0 as *mut FORCE_MEMBER;
    debug(LOG_MAIN,
          b"shutting down audio capture\x00" as *const u8 as
              *const libc::c_char);
    NETshutdownAudioCapture();
    debug(LOG_MAIN,
          b"shutting down audio playback\x00" as *const u8 as
              *const libc::c_char);
    NETshutdownAudioPlayback();
    // shut down netplay lib.
    debug(LOG_MAIN,
          b"shutting down networking\x00" as *const u8 as
              *const libc::c_char);
    NETshutdown();
    // clear any force we may have.
    debug(LOG_MAIN,
          b"free game data (forces)\x00" as *const u8 as *const libc::c_char);
    while !Force.pMembers.is_null() {
        pF = Force.pMembers;
        Force.pMembers = (*pF).psNext;
        memFreeRelease(pF as *mut libc::c_void);
        pF = 0 as *mut FORCE_MEMBER
    }
    debug(LOG_MAIN,
          b"free game data (structure limits)\x00" as *const u8 as
              *const libc::c_char);
    if ingame.numStructureLimits != 0 {
        ingame.numStructureLimits = 0 as libc::c_int as UDWORD;
        memFreeRelease(ingame.pStructureLimits as *mut libc::c_void);
        ingame.pStructureLimits = 0 as *mut UBYTE
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// copy tempates from one player to another.
#[no_mangle]
pub unsafe extern "C" fn addTemplate(mut player: UDWORD,
                                     mut psNew: *mut DROID_TEMPLATE) -> BOOL {
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if heapAlloc(psTemplateHeap,
                 &mut psTempl as *mut *mut DROID_TEMPLATE as *mut libc::c_void
                     as *mut *mut libc::c_void) == 0 {
        return 0 as libc::c_int
    }
    memcpy(psTempl as *mut libc::c_void, psNew as *const libc::c_void,
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    (*psTempl).pName =
        &mut (*psTempl).aName as *mut [STRING; 60] as *mut CHAR;
    strncpy((*psTempl).aName.as_mut_ptr(), (*psNew).aName.as_mut_ptr(),
            60 as libc::c_int as libc::c_uint);
    *(*psTempl).pName.offset((60 as libc::c_int - 1 as libc::c_int) as isize)
        = 0 as libc::c_int as STRING;
    (*psTempl).psNext = apsDroidTemplates[player as usize];
    apsDroidTemplates[player as usize] = psTempl;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addTemplateSet(mut from: UDWORD, mut to: UDWORD)
 -> BOOL {
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if from == to { return 1 as libc::c_int }
    psCurr = apsDroidTemplates[from as usize];
    while !psCurr.is_null() {
        addTemplate(to, psCurr);
        psCurr = (*psCurr).psNext
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn copyTemplateSet(mut from: UDWORD, mut to: UDWORD)
 -> BOOL {
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if from == to { return 1 as libc::c_int }
    while !apsDroidTemplates[to as usize].is_null() {
        // clear the old template out.
        psTempl = (*apsDroidTemplates[to as usize]).psNext;
        heapFree(psTemplateHeap,
                 apsDroidTemplates[to as usize] as *mut libc::c_void);
        apsDroidTemplates[to as usize] = psTempl
    }
    return addTemplateSet(from, to);
}
// Startup. mulitopt
// ////////////////////////////////////////////////////////////////////////////
// setup templates
#[no_mangle]
pub unsafe extern "C" fn multiTemplateSetup() -> BOOL {
    let mut player: UDWORD = 0;
    let mut pcPlayer: UDWORD = 0 as libc::c_int as UDWORD;
    let mut sTemp: [CHAR; 256] = [0; 256];
    if game.type_0 as libc::c_int == 12 as libc::c_int &&
           game.base as libc::c_int == 2 as libc::c_int {
        strcpy(sTemp.as_mut_ptr(), MultiForcesPath.as_mut_ptr());
        strcat(sTemp.as_mut_ptr(), sForceName.as_mut_ptr());
        strcat(sTemp.as_mut_ptr(),
               b".for\x00" as *const u8 as *const libc::c_char);
        loadForce(sTemp.as_mut_ptr());
        //		useTheForce(TRUE);
    }
    match game.type_0 as libc::c_int {
        13 | 12 => {
            //	case DMATCH:
//		for(player=0;player<MAX_PLAYERS;player++)
//		{
//			 copyTemplateSet(DEATHMATCHTEMPLATES,player);
//		}
//		break;
            player = 0 as libc::c_int as UDWORD;
            while player < game.maxPlayers as libc::c_uint {
                copyTemplateSet(5 as libc::c_int as UDWORD, player);
                player = player.wrapping_add(1)
            }
        }
        14 => {
            // create the pc player list in deathmatch set.
            addTemplateSet(5 as libc::c_int as UDWORD,
                           4 as libc::c_int as UDWORD);
            addTemplateSet(6 as libc::c_int as UDWORD,
                           4 as libc::c_int as UDWORD);
            addTemplateSet(2 as libc::c_int as UDWORD,
                           4 as libc::c_int as UDWORD);
            //choose which way to do this.
            if isHumanPlayer(5 as libc::c_int as UDWORD) != 0 {
                //pc first
                player = 0 as libc::c_int as UDWORD;
                while player < game.maxPlayers as libc::c_uint {
                    if isHumanPlayer(player) == 0 {
                        copyTemplateSet(4 as libc::c_int as UDWORD, player);
                    }
                    player = player.wrapping_add(1)
                }
                //now players.
                player = 0 as libc::c_int as UDWORD;
                while player < game.maxPlayers as libc::c_uint {
                    if isHumanPlayer(player) != 0 {
                        copyTemplateSet(5 as libc::c_int as UDWORD, player);
                    }
                    player = player.wrapping_add(1)
                }
            } else {
                // ensure a copy of pc templates to a pc player.
                if isHumanPlayer(4 as libc::c_int as UDWORD) != 0 {
                    player = 0 as libc::c_int as UDWORD;
                    while player < 8 as libc::c_int as libc::c_uint &&
                              isHumanPlayer(player) != 0 {
                        player = player.wrapping_add(1)
                    }
                    if isHumanPlayer(player) == 0 {
                        pcPlayer = player;
                        copyTemplateSet(4 as libc::c_int as UDWORD, pcPlayer);
                    }
                } else { pcPlayer = 4 as libc::c_int as UDWORD }
                //players first
                player = 0 as libc::c_int as UDWORD;
                while player < game.maxPlayers as libc::c_uint {
                    if isHumanPlayer(player) != 0 {
                        copyTemplateSet(5 as libc::c_int as UDWORD, player);
                    }
                    player = player.wrapping_add(1)
                }
                //now pc
                player = 0 as libc::c_int as UDWORD;
                while player < game.maxPlayers as libc::c_uint {
                    if isHumanPlayer(player) == 0 {
                        copyTemplateSet(pcPlayer, player);
                    }
                    player = player.wrapping_add(1)
                }
            }
        }
        _ => { }
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// remove structures from map before campaign play.
#[no_mangle]
pub unsafe extern "C" fn cleanMap(mut player: UDWORD) -> BOOL {
    let mut psD: *mut DROID = 0 as *mut DROID;
    let mut psD2: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut firstFact: BOOL = 0;
    let mut firstRes: BOOL = 0;
    bMultiPlayer = 0 as libc::c_int;
    firstFact = 1 as libc::c_int;
    firstRes = 1 as libc::c_int;
    // reverse so we always remove the last object. re-reverse afterwards.
//	reverseObjectList((BASE_OBJECT**)&apsStructLists[player]);
    match game.base as libc::c_int {
        0 => {
            //clean map
            while !apsStructLists[player as usize].is_null() {
                //strip away structures.
                removeStruct(apsStructLists[player as usize],
                             1 as
                                 libc::c_int); // remove all but construction droids.
            }
            psD = apsDroidLists[player as usize];
            while !psD.is_null() {
                psD2 = (*psD).psNext;
                //if(psD->droidType != DROID_CONSTRUCT)
                if !((*psD).droidType as libc::c_uint ==
                         DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                         (*psD).droidType as libc::c_uint ==
                             DROID_CYBORG_CONSTRUCT as libc::c_int as
                                 libc::c_uint) {
                    killDroid(psD);
                }
                psD = psD2
            }
        }
        1 => {
            //just structs, no walls
            psStruct = apsStructLists[player as usize];
            while !psStruct.is_null() {
                if (*(*psStruct).pStructureType).type_0 ==
                       REF_WALL as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_WALLCORNER as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_DEFENSE as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_BLASTDOOR as libc::c_int as libc::c_uint ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_CYBORG_FACTORY as libc::c_int as libc::c_uint
                       ||
                       (*(*psStruct).pStructureType).type_0 ==
                           REF_COMMAND_CONTROL as libc::c_int as libc::c_uint
                   {
                    removeStruct(psStruct, 1 as libc::c_int);
                    psStruct = apsStructLists[player as usize]
                    //restart,(list may have changed).
                } else if (*(*psStruct).pStructureType).type_0 ==
                              REF_FACTORY as libc::c_int as libc::c_uint ||
                              (*(*psStruct).pStructureType).type_0 ==
                                  REF_RESEARCH as libc::c_int as libc::c_uint
                              ||
                              (*(*psStruct).pStructureType).type_0 ==
                                  REF_POWER_GEN as libc::c_int as libc::c_uint
                 {
                    if (*(*psStruct).pStructureType).type_0 ==
                           REF_FACTORY as libc::c_int as libc::c_uint {
                        if firstFact == 1 as libc::c_int {
                            firstFact = 0 as libc::c_int;
                            removeStruct(psStruct, 1 as libc::c_int);
                            psStruct = apsStructLists[player as usize]
                        } else {
                            // don't delete, just rejig!
                            if (*((*psStruct).pFunctionality as
                                      *mut FACTORY)).capacity as libc::c_int
                                   != 0 as libc::c_int {
                                (*((*psStruct).pFunctionality as
                                       *mut FACTORY)).capacity =
                                    0 as libc::c_int as UBYTE;
                                (*((*psStruct).pFunctionality as
                                       *mut FACTORY)).productionOutput =
                                    (*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)
                                           as
                                           *mut PRODUCTION_FUNCTION)).productionOutput
                                        as UBYTE;
                                (*psStruct).sDisplay.imd =
                                    (*(*psStruct).pStructureType).pIMD;
                                (*psStruct).body =
                                    structureBody(psStruct) as UWORD
                            }
                            psStruct = (*psStruct).psNext
                        }
                    } else if (*(*psStruct).pStructureType).type_0 ==
                                  REF_RESEARCH as libc::c_int as libc::c_uint
                     {
                        if firstRes == 1 as libc::c_int {
                            firstRes = 0 as libc::c_int;
                            removeStruct(psStruct, 1 as libc::c_int);
                            psStruct = apsStructLists[player as usize]
                        } else {
                            if (*((*psStruct).pFunctionality as
                                      *mut RESEARCH_FACILITY)).capacity !=
                                   0 as libc::c_int as libc::c_uint {
                                // downgrade research
                                (*((*psStruct).pFunctionality as
                                       *mut RESEARCH_FACILITY)).capacity =
                                    0 as libc::c_int as UDWORD;
                                (*((*psStruct).pFunctionality as
                                       *mut RESEARCH_FACILITY)).researchPoints
                                    =
                                    (*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize)
                                           as
                                           *mut RESEARCH_FUNCTION)).researchPoints;
                                (*psStruct).sDisplay.imd =
                                    (*(*psStruct).pStructureType).pIMD;
                                (*psStruct).body =
                                    structureBody(psStruct) as UWORD
                            }
                            psStruct = (*psStruct).psNext
                        }
                    } else if (*(*psStruct).pStructureType).type_0 ==
                                  REF_POWER_GEN as libc::c_int as libc::c_uint
                     {
                        if (*((*psStruct).pFunctionality as
                                  *mut POWER_GEN)).capacity !=
                               0 as libc::c_int as libc::c_uint {
                            // downgrade powergen.
                            (*((*psStruct).pFunctionality as
                                   *mut POWER_GEN)).capacity =
                                0 as libc::c_int as UDWORD;
                            (*((*psStruct).pFunctionality as
                                   *mut POWER_GEN)).power =
                                (*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize)
                                       as
                                       *mut POWER_GEN_FUNCTION)).powerOutput;
                            let ref mut fresh0 =
                                (*((*psStruct).pFunctionality as
                                       *mut POWER_GEN)).multiplier;
                            *fresh0 =
                                (*fresh0 as
                                     libc::c_uint).wrapping_add((*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        isize)
                                                                       as
                                                                       *mut POWER_GEN_FUNCTION)).powerMultiplier)
                                    as UDWORD as UDWORD;
                            (*psStruct).sDisplay.imd =
                                (*(*psStruct).pStructureType).pIMD;
                            (*psStruct).body =
                                structureBody(psStruct) as UWORD
                        }
                        psStruct = (*psStruct).psNext
                    }
                } else { psStruct = (*psStruct).psNext }
            }
        }
        2 => { }
        _ => {
            debug(LOG_ERROR,
                  b"Unknown Campaign Style\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    // rerev list to get back to normal.
//	reverseObjectList((BASE_OBJECT**)&apsStructLists[player]);
    bMultiPlayer = 1 as libc::c_int;
    return 1 as libc::c_int;
}
//static BOOL dMatchInit			(VOID);
// ////////////////////////////////////////////////////////////////////////////
// setup a deathmatch game.
/*
static BOOL dMatchInit()
{
	UDWORD	i;
	NETMSG	msg;
	UDWORD	player;
	BOOL	resourceFound = FALSE;
	CHAR	sTemp[256];

	turnOffMultiMsg(TRUE);

	for(i =0 ; i<MAX_PLAYERS;i++)
	{
		setPower(i,LEV_HI);							// set deathmatch power to hi.
	}

	strcpy(sTemp, "multiplay/forces/");
	strcat(sTemp, sForceName);
	strcat(sTemp,".for");
	loadForce( sTemp);


	freeMessages();									// CLEAR MESSAGES

	if(NetPlay.bHost)								// it's new.
	{

	}
	else
	{
		for(player=0;player<MAX_PLAYERS;player++)			// remove droids.
		{
			while(apsDroidLists[player])
			{
				killDroid(apsDroidLists[player]);
			}

			while(apsFeatureLists[player])
			{
				removeFeature(apsFeatureLists[player]);
			}

			while(apsStructLists[player])
			{
				removeStruct(apsStructLists[player]);
			}
		}

	}
	turnOffMultiMsg(FALSE);

	if(NetPlay.bHost)
	{
														// dont do anything.
	}
	else
	{
		NetAdd(msg,0,NetPlay.dpidPlayer);				// start to request info.
		NetAdd(msg,4,arenaPlayersReceived);				// player we require.
		msg.size = 8;
		msg.type = NET_REQUESTPLAYER;
		NETbcast(msg,TRUE);
	}

	return TRUE;
}
*/
// ////////////////////////////////////////////////////////////////////////////
// setup a campaign game
unsafe extern "C" fn campInit() -> BOOL {
    let mut player: UDWORD = 0;
    let mut newPlayerArray: [UBYTE; 8] = [0; 8];
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    // if this is from a savegame, stop here!
    if getSaveGameType() == GTYPE_SAVE_START as libc::c_int as libc::c_uint ||
           getSaveGameType() ==
               GTYPE_SAVE_MIDMISSION as libc::c_int as libc::c_uint {
        // these two lines are the biggest hack in the world.
		// the reticule seems to get detached from 'reticuleup'
		// this forces it back in sync...
        intRemoveReticule();
        intAddReticule();
        return 1 as libc::c_int
    }
    // for each player, if it's a skirmish then assign a player or clear it off.
    if game.type_0 as libc::c_int == 14 as libc::c_int {
        memset(newPlayerArray.as_mut_ptr() as *mut libc::c_void,
               1 as libc::c_int, 8 as libc::c_int as libc::c_uint);
        j = 0 as libc::c_int as UDWORD;
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            if game.skDiff[i as usize] as libc::c_int == 0 as libc::c_int {
                // no player.
                // find a non human player and strip the lot.
                while isHumanPlayer(j) != 0 &&
                          j < 8 as libc::c_int as libc::c_uint {
                    j = j.wrapping_add(1)
                }
                if j != 8 as libc::c_int as libc::c_uint {
                    clearPlayer(j, 1 as libc::c_int, 0 as libc::c_int);
                    newPlayerArray[j as usize] = 0 as libc::c_int as UBYTE;
                    j = j.wrapping_add(1)
                    // dont do this one again.
                }
            } else if !(game.skDiff[i as usize] as libc::c_int ==
                            0xff as libc::c_int) {
                newPlayerArray[j as usize] =
                    game.skDiff[i as usize]; // skirmish player.
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        memcpy(game.skDiff.as_mut_ptr() as *mut libc::c_void,
               newPlayerArray.as_mut_ptr() as *const libc::c_void,
               8 as libc::c_int as libc::c_uint);
    }
    player = 0 as libc::c_int as UDWORD;
    while player < game.maxPlayers as libc::c_uint {
        // clean up only to the player limit for this map..
        if isHumanPlayer(player) == 0 &&
               game.type_0 as libc::c_int != 14 as libc::c_int {
            // strip away unused players
            clearPlayer(player, 1 as libc::c_int, 1 as libc::c_int);
        }
        cleanMap(player);
        player = player.wrapping_add(1)
    }
    // optionally remove other computer players.
    if (game.type_0 as libc::c_int == 13 as libc::c_int ||
            game.type_0 as libc::c_int == 12 as libc::c_int) &&
           game.bComputerPlayers == 0 ||
           game.type_0 as libc::c_int == 14 as libc::c_int {
        player = game.maxPlayers as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            clearPlayer(player, 1 as libc::c_int, 0 as libc::c_int);
            player = player.wrapping_add(1)
        }
    }
    // add free research gifts..
    if NetPlay.bHost != 0 {
        addOilDrum(NetPlay.playercount.wrapping_mul(2 as libc::c_int as
                                                        libc::c_uint));
        // add some free power.
    } // say howdy!
    playerResponding();
    if game.type_0 as libc::c_int == 12 as libc::c_int &&
           game.base as libc::c_int == 2 as libc::c_int {
        useTheForce(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
//extern BOOL hostArena			(STRING *sGame, STRING *sPlayer);
//extern BOOL joinArena			(UDWORD gameNumber, STRING *playername);
// ////////////////////////////////////////////////////////////////////////////
// say hi to everyone else....
#[no_mangle]
pub unsafe extern "C" fn playerResponding() {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // no longer joining.
    let mut i: UDWORD = 0;
    ingame.startTime = gameTime;
    ingame.localJoiningInProgress = 0 as libc::c_int;
    ingame.JoiningInProgress[selectedPlayer as usize] = 0 as libc::c_int;
    //	arenaPlayersReceived	= 0;						// clear rcvd list.
    cameraToHome(selectedPlayer,
                 0 as libc::c_int); // home the camera to the player.
    memcpy(&mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut selectedPlayer as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); // tell the world we're here.
    msg.size =
        ::std::mem::size_of::<UDWORD>() as libc::c_ulong as libc::c_ushort;
    msg.type_0 = NET_PLAYERRESPONDING as libc::c_int as libc::c_uchar;
    NETbcast(&mut msg, 1 as libc::c_int);
    // set the key from the lowest available dpid.
    i = 0 as libc::c_int as UDWORD;
    while player2dpid[i as usize] == 0 && i < 8 as libc::c_int as libc::c_uint
          {
        i = i.wrapping_add(1)
    }
    NETsetKey(0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
              0 as libc::c_int as UDWORD, player2dpid[i as usize] as UDWORD);
    NetPlay.bEncryptAllPackets = 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
//called when the game finally gets fired up.
#[no_mangle]
pub unsafe extern "C" fn multiGameInit() -> BOOL {
    let mut player: UDWORD = 0;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        openchannels[player as usize] = 1 as libc::c_int;
        player = player.wrapping_add(1)
        //open comms to this player.
    }
    //	if(game.type == DMATCH)
//	{
//		dMatchInit();
//		if(NetPlay.bHost)
//		{
//			addDMatchDroid(1);							// each player adds a newdroid point.
//			playerResponding();							// say hi, only if host, clients wait until all info recvd.
//		}
//	}
//	else
//	{
    campInit();
    //	}
    return 1 as libc::c_int;
}
// every game
// //////////////////////////////
// at the end of every game.
#[no_mangle]
pub unsafe extern "C" fn multiGameShutdown() -> BOOL {
    let mut st: PLAYERSTATS =
        PLAYERSTATS{played: 0,
                    wins: 0,
                    loses: 0,
                    totalKills: 0,
                    totalScore: 0,
                    recentKills: 0,
                    recentScore: 0,
                    killsToAdd: 0,
                    scoreToAdd: 0,}; // say goodbye
    sendLeavingMsg(); // update games played.
    updateMultiStatsGames(); // save stats
    st = getMultiStats(selectedPlayer, 1 as libc::c_int); // close game.
    saveMultiStats(getPlayerName(selectedPlayer),
                   getPlayerName(selectedPlayer), &mut st); // clean up
    NETclose(); //dont attempt a host
    if ingame.numStructureLimits != 0 {
        ingame.numStructureLimits =
            0 as libc::c_int as
                UDWORD; //revert back to main menu, not multioptions.
        memFreeRelease(ingame.pStructureLimits as
                           *mut libc::c_void); //back to single player mode
        ingame.pStructureLimits = 0 as *mut UBYTE
    } //back to use player 0 (single player friendly)
    ingame.localJoiningInProgress = 0 as libc::c_int; // pull security.
    ingame.localOptionsReceived = 0 as libc::c_int;
    ingame.bHostSetup = 0 as libc::c_int;
    NetPlay.bLobbyLaunched = 0 as libc::c_int;
    NetPlay.bHost = 0 as libc::c_int;
    bMultiPlayer = 0 as libc::c_int;
    selectedPlayer = 0 as libc::c_int as UDWORD;
    bForceEditorLoaded = 0 as libc::c_int;
    NetPlay.bEncryptAllPackets = 0 as libc::c_int;
    return 1 as libc::c_int;
}
