use ::libc;
extern "C" {
    pub type _formation;
    /* Read formatted input from S.  */
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    #[no_mangle]
    fn audio_PlayStaticTrack(iX: SDWORD, iY: SDWORD, iTrack: libc::c_int)
     -> BOOL;
    /*
 * Map.h
 *
 * Definitions for the map structure
 *
 */
    // Visibility bits - can also be accessed as a byte (as a whole).
    /* The different types of terrain as far as the game is concerned */
    /* change these if you change above - maybe wrap up in enumerate? */
    /* Flags for whether texture tiles are flipped in X and Y or rotated */
    // This bit describes the direction the tile is split into 2 triangles (same as triangleFlip)
    // set when the tile has the structure cursor over it
    // NASTY - this should be in tileInfoBits but there isn't any room left
    // units can drive on this even if there is a structure or feature on it
    //#define	BITS_TILE_HIGHLIGHT 0x8
    // show small structures - tank traps / bunkers
    // bit set temporarily by find path to mark a blocking tile
    // bit set to show a gateway on the tile
    // bit set to show a tall structure which camera needs to avoid.
    /*#ifndef PSX	// Extra tile info bits.... WIN32 only
#define	EXTRA_BITS_SENSOR	0x1
#define	EXTRA_BITS_2		0x2
#define	EXTRA_BITS_3		0x4
#define	EXTRA_BITS_4		0x8
#define	EXTRA_BITS_5		0x10
#define	EXTRA_BITS_6		0x20
#define	EXTRA_BITS_7		0x40
#define	EXTRA_BITS_8		0x80
#endif*/
    //#define TILE_HIGHLIGHT(x)		(x->tileInfoBits & BITS_TILE_HIGHLIGHT)
    /*
#ifndef PSX		// I've even set them up for you...:-)
#define TILE_IN_SENSORRANGE(x)	(x->tileExtraBits & EXTRA_BITS_SENSOR)
#define TILE_EXTRA_BIT2_SET(x)	(x->tileExtraBits & EXTRA_BITS_2)
#define TILE_EXTRA_BIT3_SET(x)	(x->tileExtraBits & EXTRA_BITS_3)
#define TILE_EXTRA_BIT4_SET(x)	(x->tileExtraBits & EXTRA_BITS_4)
#define TILE_EXTRA_BIT5_SET(x)	(x->tileExtraBits & EXTRA_BITS_5)
#define TILE_EXTRA_BIT6_SET(x)	(x->tileExtraBits & EXTRA_BITS_6)
#define TILE_EXTRA_BIT7_SET(x)	(x->tileExtraBits & EXTRA_BITS_7)
#define TILE_EXTRA_BIT8_SET(x)	(x->tileExtraBits & EXTRA_BITS_8)
#endif
*/
    /*
#ifndef PSX	// again, done for you again!
#define SET_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_SENSOR))
#define CLEAR_TILE_SENSOR(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_SENSOR)))
#define SET_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_2))
#define CLEAR_TILE_BIT2(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_2)))
#define SET_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_3))
#define CLEAR_TILE_BIT3(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_3)))
#define SET_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_4))
#define CLEAR_TILE_BIT4(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_4)))
#define SET_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_5))
#define CLEAR_TILE_BIT5(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_5)))
#define SET_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_6))
#define CLEAR_TILE_BIT6(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_6)))
#define SET_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_7))
#define CLEAR_TILE_BIT7(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_7)))
#define SET_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits | EXTRA_BITS_8))
#define CLEAR_TILE_BIT8(x)	(x->tileExtraBits = (UBYTE)((x)->tileExtraBits & (~EXTRA_BITS_8)))
#endif
*/
// Multiplier for the tile height
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
    /* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* The maximum number of refs for a type of stat */
    //stores for each players component states - see below
    //store for each players Structure states
    //flags to fill apCompLists and apStructTypeLists
    //this item can be used to design droids
    //the player does not know about this item
    //this item has been found, but is unresearched
    /* ******************************************************************************
*		Allocate stats functions
*******************************************************************************/
/* Allocate Weapon stats */
    /*Allocate Armour stats*/
//extern BOOL statsAllocArmour(UDWORD numEntries);
    /*Allocate Body stats*/
    /*Allocate Brain stats*/
    /*Allocate Power stats*/
//extern BOOL statsAllocPower(UDWORD numEntries);
    /*Allocate Propulsion stats*/
    /*Allocate Sensor stats*/
    /*Allocate Ecm Stats*/
    /*Allocate Repair Stats*/
    /*Allocate Program Stats*/
    /*Allocate Construct Stats*/
    /* ******************************************************************************
*		Load stats functions
*******************************************************************************/
/* Return the number of newlines in a file buffer */
    /*Load the weapon stats from the file exported from Access*/
    /*Load the armour stats from the file exported from Access*/
//extern BOOL loadArmourStats(void);
    /*Load the body stats from the file exported from Access*/
    /*Load the brain stats from the file exported from Access*/
    /*Load the power stats from the file exported from Access*/
//extern BOOL loadPowerStats(void);
    /*Load the propulsion stats from the file exported from Access*/
    /*Load the sensor stats from the file exported from Access*/
    /*Load the ecm stats from the file exported from Access*/
    /*Load the repair stats from the file exported from Access*/
    /*Load the program stats from the file exported from Access*/
    /*Load the construct stats from the file exported from Access*/
    /*Load the Propulsion Types from the file exported from Access*/
    /*Load the propulsion sounds from the file exported from Access*/
    /*Load the Terrain Table from the file exported from Access*/
    /*Load the Special Ability stats from the file exported from Access*/
    /* load the IMDs to use for each body-propulsion combination */
    /*Load the weapon sounds from the file exported from Access*/
    /*Load the Weapon Effect Modifiers from the file exported from Access*/
    /* ******************************************************************************
*		Set stats functions
*******************************************************************************/
/* Set the stats for a particular weapon type
 * The function uses the ref number in the stats structure to
 * index the correct array entry
 */
    /*Set the stats for a particular armour type*/
//extern void statsSetArmour(ARMOUR_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular body type*/
    /*Set the stats for a particular brain type*/
    /*Set the stats for a particular power type*/
//extern void statsSetPower(POWER_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular propulsion type*/
    /*Set the stats for a particular sensor type*/
    /*Set the stats for a particular ecm type*/
    /*Set the stats for a particular repair type*/
    /*Set the stats for a particular program type*/
//extern void statsSetProgram(PROGRAM_STATS	*psStats, UDWORD index);
    /*Set the stats for a particular construct type*/
    /* ******************************************************************************
*		Get stats functions
*******************************************************************************/
    //extern ARMOUR_STATS *statsGetArmour(UDWORD ref);
    //extern POWER_STATS *statsGetPower(UDWORD ref);
    //extern PROGRAM_STATS *statsGetProgram(UDWORD ref);
    /* ******************************************************************************
*		Generic stats functions
*******************************************************************************/
/*Load the stats from the Access database*/
//extern BOOL loadStats(void);
    /*calls the STATS_DEALLOC macro for each set of stats*/
    /*Deallocate the stats passed in as parameter */
    //return the type of stat this ref refers to!
    //return the REF_START value of this type of stat
    /*Returns the component type based on the string - used for reading in data */
    //get the component Inc for a stat based on the name
    //get the component Inc for a stat based on the Resource name held in Names.txt
    /*sets the tech level for the stat passed in */
    /*returns the weapon sub class based on the string name passed in */
    /*either gets the name associated with the resource (if one) or allocates space and copies pName*/
    //converts the name read in from Access into the name which is used in the Stat lists (or ignores it)
    /*return the name to display for the interface - valid for OBJECTS and STATS*/
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    #[no_mangle]
    fn getTileMaxMin(x: UDWORD, y: UDWORD, pMax: *mut UDWORD,
                     pMin: *mut UDWORD);
    /*
 * ObjMem.h
 *
 * Routines for managing object's memory
 *
 */
    //the died flag for a droid is set to this when it gets added to the non-current list
    /* The memory heaps for the different object types */
    // the memory heap for templates
    /* The lists of objects allocated */
    /* The list of destroyed objects */
    /* Initialise the object heaps */
    /* Release the object heaps */
    /* General housekeeping for the object system */
    /* Create a new droid */
    /* add the droid to the Droid Lists */
    /*destroy a droid */
    /* Remove all droids */
    /*Remove a single Droid from its list*/
    /*Removes all droids that may be stored in the mission lists*/
    /*Removes all droids that may be stored in the limbo lists*/
    /* Create a new structure */
    /* add the structure to the Structure Lists */
    /* Destroy a structure */
    /* Remove all structures */
    /*Remove a single Structure from a list*/
    /* Create a new Feature */
    /* add the feature to the Feature Lists */
    #[no_mangle]
    fn addFeature(psFeatureToAdd: *mut FEATURE);
    #[no_mangle]
    fn createFeature(ppsNew: *mut *mut FEATURE) -> BOOL;
    /* Destroy a feature */
    #[no_mangle]
    fn killFeature(psDel: *mut FEATURE);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    #[no_mangle]
    fn allocateName(ppStore: *mut *mut STRING, pName: *mut STRING) -> BOOL;
    #[no_mangle]
    fn intRefreshScreen();
    /*
 * Message.h
 *
 * Functions for the messages shown in the Intelligence Map View
 */
    /* The lists of messages allocated */
    /* The current tutorial message - there is only ever one at a time. They are displayed 
when called by the script. They are not to be re-displayed*/
//extern MESSAGE		tutorialMessage;
/* The IMD to use for the proximity messages */
    /* The list of proximity displays allocated */
    //allocates the viewdata heap
    /* Initialise the message heaps */
    /* Release the message heaps */
    //destroys the viewdata heap
    /*Add a messgae to the list */
    /*remove a message */
    #[no_mangle]
    fn removeMessage(psDel: *mut MESSAGE, player: UDWORD);
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* Remove all Messages*/
    /* removes all the proximity displays */
    /*load the view data for the messages from the file exported from the world editor*/
    /*get the view data that contains the text message pointer passed in */
    /* Release the viewdata memory */
    //extern void storeProximityScreenCoords(MESSAGE *psMessage, SDWORD x, SDWORD y);
    /* Looks through the players list of messages to find one with the same viewData 
pointer and which is the same type of message - used in scriptFuncs */
    #[no_mangle]
    fn findMessage(pViewdata: *mut MSG_VIEWDATA, type_0: MESSAGE_TYPE,
                   player: UDWORD) -> *mut MESSAGE;
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
    fn shakeStart();
    #[no_mangle]
    fn getRandomWreckageImd() -> *mut iIMDShape;
    #[no_mangle]
    fn processVisibility(psCurr: *mut BASE_OBJECT);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn initPerimeterSmoke(pImd: *mut iIMDShape, x: UDWORD, y: UDWORD,
                          z: UDWORD);
    #[no_mangle]
    fn getTileFeature(x: UDWORD, y: UDWORD) -> *mut FEATURE;
    #[no_mangle]
    fn scoreUpdateVar(var: DATA_INDEX);
    // the game description.
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn SendDestroyFeature(pF: *mut FEATURE) -> BOOL;
    #[no_mangle]
    fn getRevealStatus() -> BOOL;
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // remove an object from the grid system
    #[no_mangle]
    fn gridRemoveObject(psObj: *mut BASE_OBJECT);
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
    // see if a zone is reachable
    #[no_mangle]
    fn gwZoneReachable(zone: SDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
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
pub type _droid_type = libc::c_uint;
pub const DROID_ANY: _droid_type = 13;
pub const DROID_CYBORG_SUPER: _droid_type = 12;
pub const DROID_CYBORG_REPAIR: _droid_type = 11;
pub const DROID_CYBORG_CONSTRUCT: _droid_type = 10;
pub const DROID_DEFAULT: _droid_type = 9;
pub const DROID_REPAIR: _droid_type = 8;
pub const DROID_COMMAND: _droid_type = 7;
pub const DROID_TRANSPORTER: _droid_type = 6;
pub const DROID_CYBORG: _droid_type = 5;
pub const DROID_PERSON: _droid_type = 4;
pub const DROID_CONSTRUCT: _droid_type = 3;
pub const DROID_ECM: _droid_type = 2;
pub const DROID_SENSOR: _droid_type = 1;
pub const DROID_WEAPON: _droid_type = 0;
pub type DROID_TYPE = _droid_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _component {
    pub nStat: UBYTE,
}
pub type COMPONENT = _component;
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
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
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
// Weapon droid
// Sensor droid
// ECM droid
// Constructor droid
// person
// cyborg-type thang
// guess what this is!
// Command droid
// Repair droid
// Default droid
//cyborg constructor droid - new for update 28/5/99
//cyborg repair droid - new for update 28/5/99
//cyborg repair droid - new for update 7/6/99
// Any droid, Only used as a parameter for intGotoNextDroidType(droidtype).
//	UDWORD					nStat;
// Allowing a maximum of 255 stats per file
//UDWORD					hitPoints; NOT USED?
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
// maximum number of queued orders
//struct _base_object	*psObj;
//this needs to cope with objects and stats
//line build requires two sets of coords
// maximum number of characters in a droid name
// FOR DROID TEMPLATES
	// On the PC the pName entry in STATS_BASE is redundant and can be assumed to be NULL,
	// on the PSX the NameHash entry is used. If it is database generated template, the hashed version of the short name of the template is stored. If it is a user generated template NULL is stored.
/* basic stats */
// on the PC this contains the full editable ascii name of the template
	// on the PSX this is not used, the full name is NON-EDITABLE and is generated from the template components e.g. Viper Mk I
// Version number used in name (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied to droid structure
/* The droid components.  This array is indexed by COMPONENT_TYPE
	 * so the ECM would be accessed using asParts[COMP_ECM].
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/*total build points required to manufacture
												  the droid*/
/*total power points required to build/maintain
												  the droid */
/*used to load in weaps and progs*/
/* The weapon systems */
/* Number of weapons*/
/* weapon indices */
/* The programs */
	//UDWORD			numProgs;					/* Number of programs*/
	//UDWORD			asProgs[DROID_MAXPROGS];	/* program indices*/
// The type of droid
//#ifndef PSX
// multiplayer unique descriptor(cant use id's for templates)
// used for save games as well now - AB 29/10/98
//#endif
/* Pointer to next template*/
pub type DROID = _droid;
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
/* The common structure elements for all objects */
//Ascii name of the droid - This is generated from the droid template and can not be changed by the game player after creation.
//	UBYTE 		NameVersion;			// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
// The type of droid
/* Possibly a set of function pointers here to be called when
	 * damage is done to a component, power is low ...
	 */
//	DROID_TEMPLATE	*pTemplate;		//defines the droids components
/* Holds the specifics for the component parts - allows damage
	 * per part to be calculated. Indexed by COMPONENT_TYPE.
	 * Weapons and Programs need to be dealt with separately.
	 * COMP_BRAIN is an index into the asCommandDroids array NOT asBrainStats
	 */
/* The other droid data.  These are all derived from the components
	 * but stored here for easy access
	 */
//the base speed dependant on propulsion type
//the original body points
// the current body points
//UDWORD		power;
//tjc	UDWORD		imdNum;
//UWORD		turretRotRate; THIS IS A CONSTANT
//*
// Version number used for generating on-the-fly names (e.g. Viper Mk "I" would be stored as 1 - Viper Mk "X" as 10)  - copied from droid template
//	UDWORD		numKills;
//used in Electronic Warfare
//	SDWORD		activeWeapon;		// The currently selected weapon
	//UDWORD		numWeaps;
//SDWORD		activeProg;			// The currently running program
	//UDWORD		numProgs;
	//PROGRAM		asProgs[DROID_MAXPROGS];
// The group the droid belongs to
//a structure that this droid might be associated with
                                                    //for vtols its the rearming pad
// queued orders
/* Order data */
// 	struct _base_object	*psLastAttacker;
// secondary order data
// multiplayer synchronisation value.
/* Action data */
// Action target object
// Game time action started
// number of points done by action since start
//UWORD				actionHeight;		// height to level the ground to for foundation,
											// possibly use it for other data as well? Yup! - powerAccrued!
// renamed the above variable since this is what its used for now!
//UBYTE				tileNumber;			// tile number for foundation NOT USED ANYMORE
/* Movement control data */
//	void				*lastTile;
	/* AI data */
//	AI_DATA				sAI;
	/* anim data */
/*
 * FeatureDef.h
 *
 * Structure definitions for features
 *
 */
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
/* The type of feature */
// Graphic for the feature
/*The width of the base in tiles*/
/*The breadth of the base in tiles*/
//done in script files now
	/* component type activated if a FEAT_GEN_ARTE */
	//UDWORD			compType;			// type of component activated
	//UDWORD			compIndex;			// index of component
/* Flag to indicated whether the tile needs to be drawn
										   TRUE = draw tile */
/* Flag to indicate whether the feature allows the LOS
										   TRUE = can see through the feature */
/* Flag to indicate whether the feature is visible at
										   the start of the mission */
// Whether the feature can be blown up
// Number of body points
// Feature armour
pub type FEATURE = _feature;
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
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
//base structure for each message
pub type MESSAGE = _message;
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
pub type MSG_VIEWDATA = *mut libc::c_void;
pub type MESSAGE_TYPE = _message_type;
pub type _message_type = libc::c_uint;
pub const MSG_TYPES: _message_type = 4;
pub const MSG_PROXIMITY: _message_type = 3;
pub const MSG_MISSION: _message_type = 2;
pub const MSG_CAMPAIGN: _message_type = 1;
pub const MSG_RESEARCH: _message_type = 0;
//The type of message
//ID number of the message
//VIEWDATA		*pViewData;				//Pointer to view data - if any - should be some!
//Pointer to view data - if any - should be some!
//flag to indicate whether message has been read
//which player this message belongs to
//pointer to the next in the list
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
pub const ID_SOUND_EXPLOSION: C2RustUnnamed = 271;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed = 277;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
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
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed = 278;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed = 272;
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
pub const NO_SOUND: C2RustUnnamed = -1;
/* The mask to get internal tile coords from a full coordinate */
/* Shutdown the map module */
/* Create a new map of a specified size */
/* Load the map data */
/* Save the map data */
/* Load map texture info */
/* Save the current map texture info */
/* A post process for the water tiles in map to ensure height integrity */
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
/* Return height of tile at x,y */
//static inline SDWORD map_TileHeight(UDWORD x, UDWORD y)
#[inline]
unsafe extern "C" fn map_TileHeight(mut x: UDWORD, mut y: UDWORD) -> SWORD {
    if x >= mapWidth || y >= mapHeight { return 0 as libc::c_int as SWORD }
    return ((*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                                    isize)).height as libc::c_int *
                2 as libc::c_int) as SWORD;
}
/*
 * Feature.c
 *
 * Load feature stats
 */
/* The statistics for the features */
#[no_mangle]
pub static mut asFeatureStats: *mut FEATURE_STATS =
    0 as *const FEATURE_STATS as *mut FEATURE_STATS;
#[no_mangle]
pub static mut numFeatureStats: UDWORD = 0;
//Value is stored for easy access to this feature in destroyDroid()/destroyStruct()
//UDWORD			droidFeature;
#[no_mangle]
pub static mut structFeature: UDWORD = 0;
#[no_mangle]
pub static mut oilResFeature: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn featureInitVars() {
    asFeatureStats = 0 as *mut FEATURE_STATS;
    numFeatureStats = 0 as libc::c_int as UDWORD;
    //droidFeature = 0;
    structFeature = 0 as libc::c_int as UDWORD;
    oilResFeature = 0 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn featureType(mut psFeature: *mut FEATURE_STATS,
                                     mut pType: *mut libc::c_char) {
    if strcmp(pType, b"HOVER WRECK\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*psFeature).subType = FEAT_HOVER;
        return
    }
    if strcmp(pType, b"TANK WRECK\x00" as *const u8 as *const libc::c_char) ==
           0 {
        (*psFeature).subType = FEAT_TANK;
        return
    }
    if strcmp(pType,
              b"GENERIC ARTEFACT\x00" as *const u8 as *const libc::c_char) ==
           0 {
        (*psFeature).subType = FEAT_GEN_ARTE;
        return
    }
    if strcmp(pType, b"OIL RESOURCE\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*psFeature).subType = FEAT_OIL_RESOURCE;
        return
    }
    if strcmp(pType, b"BOULDER\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*psFeature).subType = FEAT_BOULDER;
        return
    }
    if strcmp(pType, b"VEHICLE\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*psFeature).subType = FEAT_VEHICLE;
        return
    }
    if strcmp(pType, b"DROID WRECK\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*psFeature).subType = FEAT_DROID;
        return
    }
    if strcmp(pType,
              b"BUILDING WRECK\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*psFeature).subType = FEAT_BUILD_WRECK;
        return
    }
    if strcmp(pType, b"BUILDING\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*psFeature).subType = FEAT_BUILDING;
        return
    }
    if strcmp(pType, b"OIL DRUM\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*psFeature).subType = FEAT_OIL_DRUM;
        return
    }
    if strcmp(pType, b"TREE\x00" as *const u8 as *const libc::c_char) == 0 {
        (*psFeature).subType = FEAT_TREE;
        return
    }
    if strcmp(pType, b"SKYSCRAPER\x00" as *const u8 as *const libc::c_char) ==
           0 {
        (*psFeature).subType = FEAT_SKYSCRAPER;
        return
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Unknown Feature Type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"feature.c\x00" as *const u8 as *const libc::c_char,
              135 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"featureType\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
}
/* Load the feature stats */
#[no_mangle]
pub unsafe extern "C" fn loadFeatureStats(mut pFeatureData: *mut libc::c_char,
                                          mut bufferSize: UDWORD) -> BOOL {
    let mut pData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psFeature: *mut FEATURE_STATS = 0 as *mut FEATURE_STATS;
    let mut i: UDWORD = 0;
    let mut featureName: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut type_0: [STRING; 60] = [0; 60];
    //compName[MAX_NAME_SIZE], compType[MAX_NAME_SIZE];
    //keep the start so we release it at the end
    pData = pFeatureData;
    numFeatureStats = numCR(pFeatureData, bufferSize);
    asFeatureStats =
        memMallocRelease((::std::mem::size_of::<FEATURE_STATS>() as
                              libc::c_ulong).wrapping_mul(numFeatureStats)) as
            *mut FEATURE_STATS;
    if asFeatureStats.is_null() {
        debug(LOG_ERROR,
              b"Feature Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    psFeature = asFeatureStats;
    i = 0 as libc::c_int as UDWORD;
    while i < numFeatureStats {
        let mut Width: UDWORD = 0;
        let mut Breadth: UDWORD = 0;
        memset(psFeature as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<FEATURE_STATS>() as libc::c_ulong);
        featureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        type_0[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        //read the data into the storage - the data is delimeted using comma's
        /*		sscanf(pFeatureData,"%[^','],%d,%d,%d,%d,%d,%[^','],%[^','],%[^','],%[^','],%d,%d,%d",
			&featureName, &psFeature->baseWidth, &psFeature->baseBreadth,
			&psFeature->damageable, &psFeature->armour, &psFeature->body,
			&GfxFile, &type, &compType, &compName,
			&psFeature->tileDraw, &psFeature->allowLOS, &psFeature->visibleAtStart);*/
        sscanf(pFeatureData,
               b"%[^\',\'],%d,%d,%d,%d,%d,%[^\',\'],%[^\',\'],%d,%d,%d\x00" as
                   *const u8 as *const libc::c_char, featureName.as_mut_ptr(),
               &mut Width as *mut UDWORD, &mut Breadth as *mut UDWORD,
               &mut (*psFeature).damageable as *mut BOOL,
               &mut (*psFeature).armour as *mut UDWORD,
               &mut (*psFeature).body as *mut UDWORD, GfxFile.as_mut_ptr(),
               type_0.as_mut_ptr(), &mut (*psFeature).tileDraw as *mut BOOL,
               &mut (*psFeature).allowLOS as *mut BOOL,
               &mut (*psFeature).visibleAtStart as *mut BOOL);
        // These are now only 16 bits wide - so we need to copy them
        (*psFeature).baseWidth = Width as UWORD;
        (*psFeature).baseBreadth = Breadth as UWORD;
        if allocateName(&mut (*psFeature).pName, featureName.as_mut_ptr()) ==
               0 {
            return 0 as libc::c_int
        }
        //determine the feature type
        featureType(psFeature, type_0.as_mut_ptr());
        //need to know which is the wrecked droid and wrecked structure for later use
		//the last stat of each type is used
		/*if (psFeature->subType == FEAT_DROID)
		{
			droidFeature = i;
		}
		else */
        if (*psFeature).subType as libc::c_uint ==
               FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
            structFeature = i
        } else if (*psFeature).subType as libc::c_uint ==
                      FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
            oilResFeature = i
        }
        //and the oil resource - assumes only one!
        //get the IMD for the feature
        (*psFeature).psImd =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, GfxFile.as_mut_ptr()) as
                *mut iIMDShape;
        if (*psFeature).psImd.is_null() {
            debug(LOG_ERROR,
                  b"Cannot find the feature PIE for record %s\x00" as
                      *const u8 as *const libc::c_char,
                  getName((*psFeature).pName));
            abort();
        }
        //sort out the component - if any
		/*if (strcmp(compType, "0"))
		{
			psFeature->compType = componentType(compType);
			psFeature->compIndex = getCompFromName(psFeature->compType, compName);
		}*/
        (*psFeature).ref_0 =
            (0x100000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //increment the pointer to the start of the next record
        pFeatureData =
            strchr(pFeatureData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        //increment the list to the start of the next storage block
        psFeature = psFeature.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    return 1 as libc::c_int;
    /* Allocate the stats Array */
/*	numFeatureStats = 19;
	asFeatureStats = (FEATURE_STATS *)MALLOC(sizeof(FEATURE_STATS) * numFeatureStats);
	if (!asFeatureStats)
	{
		DBERROR(("Out of memory"));
		return FALSE;
	}
	memset(asFeatureStats, 0, sizeof(FEATURE_STATS) * numFeatureStats);

	// Create some simple stats
	ref = REF_FEATURE_START;
	psStats = asFeatureStats;
	psStats->pName = "Mesa Feature";
	psStats->ref = ref ++;
	psStats->subType = FEAT_MESA;
	psStats->baseWidth = 6;
	psStats->baseBreadth = 4;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mimesa1.imd");

	psStats++;
	psStats->pName = "Mesa Feature 2";
	psStats->ref = ref ++;
	psStats->subType = FEAT_MESA2;
	psStats->baseWidth = 5;
	psStats->baseBreadth = 4;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mimesa2.imd");

	psStats++;
	psStats->pName = "Cliff";
	psStats->ref = ref ++;
	psStats->subType = FEAT_CLIFF;
	psStats->baseWidth = 7;
	psStats->baseBreadth = 7;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "micliff.imd");

	psStats++;
	psStats->pName = "Stack";
	psStats->ref = ref ++;
	psStats->subType = FEAT_STACK;
	psStats->baseWidth = 2;
	psStats->baseBreadth = 2;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mistack.imd");

	psStats++;
	psStats->pName = "Wrecked Building";
	psStats->ref = ref ++;
	psStats->subType = FEAT_BUILD_WRECK1;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miwreck.imd");

	psStats++;
	psStats->pName = "Wrecked Hovercraft";
	psStats->ref = ref ++;
	psStats->subType = FEAT_HOVER;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miarthov.imd");

	// Find the hover component for it
	psStats->compType = COMP_PROPULSION;
	for(comp=0; comp<numPropulsionStats; comp++)
	{
		if (strcmp(asPropulsionStats[comp].pName, "Hover Propulsion") == 0)
		{
			psStats->compIndex = comp;
		}
	}

	psStats++;
	psStats->pName = "Wrecked Tank";
	psStats->ref = ref ++;
	psStats->subType = FEAT_TANK;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miartecm.imd");

	// Find the ecm component for it
	psStats->compType = COMP_ECM;
	for(comp=0; comp<numECMStats; comp++)
	{
		if (strcmp(asECMStats[comp].pName, "Heavy ECM #1") == 0)
		{
			psStats->compIndex = comp;
		}
	}

	psStats++;
	psStats->pName = "Generic Artefact";
	psStats->ref = ref ++;
	psStats->subType = FEAT_GEN_ARTE;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "miartgen.imd");

	psStats++;
	psStats->pName = "Oil Resource";
	psStats->ref = ref ++;
	psStats->subType = FEAT_OIL_RESOURCE;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "mislick.imd");

	psStats++;
	psStats->pName = "Boulder 1";
	psStats->ref = ref ++;
	psStats->subType = FEAT_BOULDER1;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mibould1.imd");

	psStats++;
	psStats->pName = "Boulder 2";
	psStats->ref = ref ++;
	psStats->subType = FEAT_BOULDER2;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mibould2.imd");

	psStats++;
	psStats->pName = "Boulder 3";
	psStats->ref = ref ++;
	psStats->subType = FEAT_BOULDER3;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mibould3.imd");

	psStats++;
	psStats->pName = "Futuristic Car";
	psStats->ref = ref ++;
	psStats->subType = FEAT_FUTCAR;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mifutcar.imd");

	psStats++;
	psStats->pName = "Fururistic Van";
	psStats->ref = ref ++;
	psStats->subType = FEAT_FUTVAN;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "mifutvan.imd");

	psStats++;
	psStats->pName = "Wrecked Droid Hub";
	psStats->ref = ref ++;
	psStats->subType = FEAT_DROID;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "drwreck.imd");
	//Value is stored for easy access to this feature in destroyDroid()
	droidFeature = psStats - asFeatureStats;

	psStats++;
	//Value is stored for easy access to this feature in destroyStruct
	structFeature = psStats - asFeatureStats;
	psStats->pName = "Wrecked Building1";
	psStats->ref = ref++;
	psStats->subType = FEAT_BUILD_WRECK1;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miwrek1.imd");

	psStats++;
	psStats->pName = "Wrecked Building2";
	psStats->ref = ref++;
	psStats->subType = FEAT_BUILD_WRECK2;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miwrek2.imd");

	psStats++;
	psStats->pName = "Wrecked Building3";
	psStats->ref = ref++;
	psStats->subType = FEAT_BUILD_WRECK3;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miwrek3.imd");

	psStats++;
	psStats->pName = "Wrecked Building4";
	psStats->ref = ref++;
	psStats->subType = FEAT_BUILD_WRECK4;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = FALSE;
	psStats->allowLOS = TRUE;
	psStats->psImd = resGetData("IMD", "miwrek4.imd");

	// These are test features for the LOS code
	psStats++;
	psStats->pName = "Cube 1,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_1_1.imd");

	psStats++;
	psStats->pName = "Cube 1,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_1_3.imd");

	psStats++;
	psStats->pName = "Cube 2,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 2;
	psStats->baseBreadth = 2;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_2_1.imd");

	psStats++;
	psStats->pName = "Cube 2,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 2;
	psStats->baseBreadth = 2;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_2_3.imd");

	psStats++;
	psStats->pName = "Cube 3,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 3;
	psStats->baseBreadth = 3;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_3_1.imd");

	psStats++;
	psStats->pName = "Cube 3,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 3;
	psStats->baseBreadth = 3;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cube_3_3.imd");

	psStats++;
	psStats->pName = "Cyl 1,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_1_1.imd");

	psStats++;
	psStats->pName = "Cyl 1,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 1;
	psStats->baseBreadth = 1;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_1_3.imd");

	psStats++;
	psStats->pName = "Cyl 2,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 2;
	psStats->baseBreadth = 2;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_2_1.imd");

	psStats++;
	psStats->pName = "Cyl 2,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 2;
	psStats->baseBreadth = 2;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_2_3.imd");

	psStats++;
	psStats->pName = "Cyl 3,1";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 3;
	psStats->baseBreadth = 3;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_3_1.imd");

	psStats++;
	psStats->pName = "Cyl 3,3";
	psStats->ref = ref++;
	psStats->subType = FEAT_LOS_OBJ;
	psStats->baseWidth = 3;
	psStats->baseBreadth = 3;
	psStats->tileDraw = TRUE;
	psStats->allowLOS = FALSE;
	psStats->psImd = resGetData("IMD", "cyl_3_3.imd");

	psStats++;
	ASSERT( psStats - asFeatureStats == (SDWORD)numFeatureStats,
		"loadFeatureStats: incorrect number of features" );

	return TRUE;*/
}
/* Release the feature stats memory */
#[no_mangle]
pub unsafe extern "C" fn featureStatsShutDown() {
    if numFeatureStats != 0 {
        memFreeRelease(asFeatureStats as *mut libc::c_void);
        asFeatureStats = 0 as *mut FEATURE_STATS
    };
}
/* Deal with damage to a feature */
#[no_mangle]
pub unsafe extern "C" fn featureDamage(mut psFeature: *mut FEATURE,
                                       mut damage: UDWORD,
                                       mut weaponClass: UDWORD,
                                       mut weaponSubClass: UDWORD) -> BOOL {
    let mut penDamage: UDWORD = 0;
    /* this is ignored for features */
	//(void)weaponClass;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"featureDamage: Invalid feature pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"feature.c\x00" as *const u8 as *const libc::c_char,
              634 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"featureDamage\x00")).as_ptr(),
              b"PTRVALID(psFeature, sizeof(FEATURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //EMP cannons do not work on Features
    if weaponSubClass == WSC_EMP as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    if damage > (*(*psFeature).psStats).armour {
        /* Damage has penetrated - reduce body points */
        penDamage = damage.wrapping_sub((*(*psFeature).psStats).armour);
        if penDamage >= (*psFeature).body {
            /* feature destroyed */
            destroyFeature(psFeature);
            return 1 as libc::c_int
        } else {
            (*psFeature).body =
                ((*psFeature).body as libc::c_uint).wrapping_sub(penDamage) as
                    UDWORD as UDWORD
        }
    } else if (*psFeature).body == 1 as libc::c_int as libc::c_uint {
        destroyFeature(psFeature);
        return 1 as libc::c_int
    } else {
        (*psFeature).body =
            ((*psFeature).body as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD
    }
    /* Do one point of damage to body */
    //	if(psFeature->sDisplay.imd->ymax > 300)
//	{
    (*psFeature).timeLastHit = gameTime;
    //	}
    return 0 as libc::c_int;
}
// Set the tile no draw flags for a structure
#[no_mangle]
pub unsafe extern "C" fn setFeatTileDraw(mut psFeat: *mut FEATURE) {
    let mut psStats: *mut FEATURE_STATS = (*psFeat).psStats;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    mapX =
        (((*psFeat).x as libc::c_int >> 7 as libc::c_int) -
             (*psStats).baseWidth as libc::c_int / 2 as libc::c_int) as
            UDWORD;
    mapY =
        (((*psFeat).y as libc::c_int >> 7 as libc::c_int) -
             (*psStats).baseBreadth as libc::c_int / 2 as libc::c_int) as
            UDWORD;
    if (*psStats).tileDraw == 0 {
        width = 0 as libc::c_int as UDWORD;
        while width < (*psStats).baseWidth as libc::c_uint {
            breadth = 0 as libc::c_int as UDWORD;
            while breadth < (*psStats).baseBreadth as libc::c_uint {
                (*mapTile(mapX.wrapping_add(width),
                          mapY.wrapping_add(breadth))).tileInfoBits =
                    ((*mapTile(mapX.wrapping_add(width),
                               mapY.wrapping_add(breadth))).tileInfoBits as
                         libc::c_int | 0x4 as libc::c_int) as UBYTE;
                breadth = breadth.wrapping_add(1)
            }
            width = width.wrapping_add(1)
        }
    };
}
/* Create a feature on the map */
#[no_mangle]
pub unsafe extern "C" fn buildFeature(mut psStats: *mut FEATURE_STATS,
                                      mut x: UDWORD, mut y: UDWORD,
                                      mut FromSave: BOOL) -> *mut FEATURE {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut foundationMin: UDWORD = 0;
    let mut foundationMax: UDWORD = 0;
    let mut height: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut startX: UDWORD = 0;
    let mut startY: UDWORD = 0;
    let mut max: UDWORD = 0;
    let mut min: UDWORD = 0;
    let mut i: SDWORD = 0;
    let mut vis: UBYTE = 0;
    //try and create the Feature
    if createFeature(&mut psFeature) == 0 { return 0 as *mut FEATURE }
    //add the feature to the list - this enables it to be drawn whilst being built
    addFeature(psFeature);
    // get the terrain average height
    startX = x >> 7 as libc::c_int;
    startY = y >> 7 as libc::c_int;
    foundationMin = (255 as libc::c_int * 2 as libc::c_int) as UDWORD;
    foundationMax = 0 as libc::c_int as UDWORD;
    breadth = 0 as libc::c_int as UDWORD;
    while breadth < (*psStats).baseBreadth as libc::c_uint {
        width = 0 as libc::c_int as UDWORD;
        while width < (*psStats).baseWidth as libc::c_uint {
            getTileMaxMin(startX.wrapping_add(width),
                          startY.wrapping_add(breadth), &mut max, &mut min);
            if foundationMin > min { foundationMin = min }
            if foundationMax < max { foundationMax = max }
            width = width.wrapping_add(1)
        }
        breadth = breadth.wrapping_add(1)
    }
    //return the average of max/min height
    height =
        foundationMin.wrapping_add(foundationMax).wrapping_div(2 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
    // check you can reach an oil resource
    if (*psStats).subType as libc::c_uint ==
           FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint &&
           gwZoneReachable(gwGetZone(startX as SDWORD, startY as SDWORD)) == 0
       {
        debug(LOG_NEVER,
              b"Oil resource at (%d,%d) is unreachable\x00" as *const u8 as
                  *const libc::c_char, startX, startY);
    }
    if FromSave == 1 as libc::c_int {
        (*psFeature).x = x as UWORD;
        (*psFeature).y = y as UWORD
    } else {
        (*psFeature).x =
            (x &
                 !(0x7f as libc::c_int) as
                     libc::c_uint).wrapping_add(((*psStats).baseWidth as
                                                     libc::c_int *
                                                     128 as libc::c_int /
                                                     2 as libc::c_int) as
                                                    libc::c_uint) as UWORD;
        (*psFeature).y =
            (y &
                 !(0x7f as libc::c_int) as
                     libc::c_uint).wrapping_add(((*psStats).baseBreadth as
                                                     libc::c_int *
                                                     128 as libc::c_int /
                                                     2 as libc::c_int) as
                                                    libc::c_uint) as UWORD
    }
    /* Dump down the building wrecks at random angles - still looks shit though */
    if (*psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
        (*psFeature).direction = (rand() % 360 as libc::c_int) as UWORD;
        (*psFeature).gfxScaling =
            (80 as libc::c_int +
                 (10 as libc::c_int - rand() % 20 as libc::c_int)) as UWORD
        // put into define
    } else if (*psStats).subType as libc::c_uint ==
                  FEAT_TREE as libc::c_int as libc::c_uint {
        (*psFeature).direction = (rand() % 360 as libc::c_int) as UWORD;
        (*psFeature).gfxScaling =
            (100 as libc::c_int +
                 (14 as libc::c_int - rand() % 28 as libc::c_int)) as UWORD
    } else {
        (*psFeature).direction = 0 as libc::c_int as UWORD;
        (*psFeature).gfxScaling = 100 as libc::c_int as UWORD
        // but irrelevant anyway, cos it's not scaled
    }
    //psFeature->damage = featureDamage;
    (*psFeature).selected = 0 as libc::c_int as UBYTE;
    (*psFeature).psStats = psStats;
    //psFeature->subType = psStats->subType;
    (*psFeature).body =
        (*psStats).body; //set the player out of range to avoid targeting confusions
    (*psFeature).player = (8 as libc::c_int + 1 as libc::c_int) as UBYTE;
    (*psFeature).bTargetted = 0 as libc::c_int;
    (*psFeature).timeLastHit = 0 as libc::c_int as UDWORD;
    if getRevealStatus() != 0 {
        vis = 0 as libc::c_int as UBYTE
    } else if (*psStats).visibleAtStart != 0 {
        vis = 0xff as libc::c_int as UBYTE;
        setFeatTileDraw(psFeature);
    } else { vis = 0 as libc::c_int as UBYTE }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        (*psFeature).visible[i as usize] = 0 as libc::c_int as UBYTE;
        i += 1
        //vis;
    }
    //load into the map data
    if FromSave != 0 {
        mapX =
            (x >>
                 7 as
                     libc::c_int).wrapping_sub(((*psStats).baseWidth as
                                                    libc::c_int /
                                                    2 as libc::c_int) as
                                                   libc::c_uint);
        mapY =
            (y >>
                 7 as
                     libc::c_int).wrapping_sub(((*psStats).baseBreadth as
                                                    libc::c_int /
                                                    2 as libc::c_int) as
                                                   libc::c_uint)
    } else { mapX = x >> 7 as libc::c_int; mapY = y >> 7 as libc::c_int }
    // set up the imd for the feature
    if (*(*psFeature).psStats).subType as libc::c_uint ==
           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
        //		psFeature->sDisplay.imd = wreckageImds[rand()%MAX_WRECKAGE];
        (*psFeature).sDisplay.imd = getRandomWreckageImd()
    } else {
        (*psFeature).sDisplay.imd = (*psStats).psImd
        //DBPRINTF(("%d %d\n",psStats->psImd->ymin,psStats->psImd->ymax);
    }
    // make sure we have an imd.
    width = 0 as libc::c_int as UDWORD;
    while width <= (*psStats).baseWidth as libc::c_uint {
        breadth = 0 as libc::c_int as UDWORD;
        while breadth <= (*psStats).baseBreadth as libc::c_uint {
            //check not outside of map - for load save game
            if mapX.wrapping_add(width) < mapWidth {
            } else {
                debug(LOG_ERROR,
                      b"x coord bigger than map width - %s, id = %d\x00" as
                          *const u8 as *const libc::c_char,
                      getName((*(*psFeature).psStats).pName),
                      (*psFeature).id);
            };
            if mapX.wrapping_add(width) < mapWidth {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"feature.c\x00" as *const u8 as *const libc::c_char,
                      850 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"buildFeature\x00")).as_ptr(),
                      b"(mapX+width) < mapWidth\x00" as *const u8 as
                          *const libc::c_char);
            };
            if mapY.wrapping_add(breadth) < mapHeight {
            } else {
                debug(LOG_ERROR,
                      b"y coord bigger than map height - %s, id = %d\x00" as
                          *const u8 as *const libc::c_char,
                      getName((*(*psFeature).psStats).pName),
                      (*psFeature).id);
            };
            if mapY.wrapping_add(breadth) < mapHeight {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"feature.c\x00" as *const u8 as *const libc::c_char,
                      853 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 13],
                                                &[libc::c_char; 13]>(b"buildFeature\x00")).as_ptr(),
                      b"(mapY+breadth) < mapHeight\x00" as *const u8 as
                          *const libc::c_char);
            };
            psTile =
                mapTile(mapX.wrapping_add(width), mapY.wrapping_add(breadth));
            if width != (*psStats).baseWidth as libc::c_uint &&
                   breadth != (*psStats).baseBreadth as libc::c_uint {
                if (*mapTile(mapX.wrapping_add(width),
                             mapY.wrapping_add(breadth))).tileInfoBits as
                       libc::c_int & 0x2 as libc::c_int == 0 {
                } else {
                    debug(LOG_ERROR,
                          b"buildFeature - feature- %d already found at %d, %d\x00"
                              as *const u8 as *const libc::c_char,
                          (*psFeature).id, mapX.wrapping_add(width),
                          mapY.wrapping_add(breadth));
                };
                if (*mapTile(mapX.wrapping_add(width),
                             mapY.wrapping_add(breadth))).tileInfoBits as
                       libc::c_int & 0x2 as libc::c_int == 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"feature.c\x00" as *const u8 as
                              *const libc::c_char, 860 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 13],
                                                    &[libc::c_char; 13]>(b"buildFeature\x00")).as_ptr(),
                          b"!(TILE_HAS_FEATURE(mapTile(mapX+width,mapY+breadth)))\x00"
                              as *const u8 as *const libc::c_char);
                };
                (*psTile).tileInfoBits =
                    ((*psTile).tileInfoBits as libc::c_int &
                         !(0x1 as libc::c_int | 0x2 as libc::c_int |
                               0x20 as libc::c_int) | 0x2 as libc::c_int) as
                        UBYTE;
                // if it's a tall feature then flag it in the map.
                if (*(*psFeature).sDisplay.imd).ymax > 200 as libc::c_int {
                    //#ifdef PSX
//DBPRINTF(("Tall feature %d, (%d x %d)\n",psFeature->sDisplay.imd->ymax,psStats->baseWidth,psStats->baseBreadth));
//#endif
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x80 as libc::c_int) as UBYTE
                }
                if (*psStats).subType as libc::c_uint ==
                       FEAT_GEN_ARTE as libc::c_int as libc::c_uint ||
                       (*psStats).subType as libc::c_uint ==
                           FEAT_OIL_DRUM as libc::c_int as libc::c_uint ||
                       (*psStats).subType as libc::c_uint ==
                           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    // they're there - just can see me
                    (*psTile).texture =
                        ((*psTile).texture as libc::c_int |
                             0x200 as libc::c_int) as UWORD
                }
            }
            if (*psStats).tileDraw == 0 && FromSave == 0 as libc::c_int {
                (*psTile).height =
                    height.wrapping_div(2 as libc::c_int as libc::c_uint) as
                        UBYTE
                // This sets the gourad shading to give 'as the artist drew it' levels
                //				psTile->illumination = ILLUMINATION_NONE;		// set the tile so that there is no illumination connecting to feature ... this value works on both psx & pc
            } //jps 18july97
            breadth = breadth.wrapping_add(1)
        }
        width = width.wrapping_add(1)
    }
    (*psFeature).z = map_TileHeight(mapX, mapY) as UWORD;
    //store the time it was built for removing wrecked droids/structures
    (*psFeature).startTime = gameTime;
    //	// set up the imd for the feature
//	if(psFeature->psStats->subType==FEAT_BUILD_WRECK)
//	{
//		psFeature->sDisplay.imd = wreckageImds[rand()%MAX_WRECKAGE];
//	}
//	else
//	{
//		psFeature->sDisplay.imd = psStats->psImd;
//DBPRINTF(("%d %d\n",psStats->psImd->ymin,psStats->psImd->ymax);
// 	}
    // add the feature to the grid
    gridAddObject(psFeature as *mut BASE_OBJECT);
    return psFeature;
}
/* Release the resources associated with a feature */
#[no_mangle]
pub unsafe extern "C" fn featureRelease(mut psFeature: *mut FEATURE) {
    gridRemoveObject(psFeature as *mut BASE_OBJECT);
}
/* Update routine for features */
#[no_mangle]
pub unsafe extern "C" fn featureUpdate(mut psFeat: *mut FEATURE) {
    //	if(getRevealStatus())
   //	{
		// update the visibility for the feature
    processVisibility(psFeat as *mut BASE_OBJECT);
    //	}
    match (*(*psFeat).psStats).subType as libc::c_uint {
        8 | 0 => {
            destroyFeature(psFeat); // get rid of the now!!!
        }
        _ => { }
    };
}
//		//kill off wrecked droids and structures after 'so' long
//		if ((gameTime - psFeat->startTime) > WRECK_LIFETIME)
//		{
// free up a feature with no visual effects
#[no_mangle]
pub unsafe extern "C" fn removeFeature(mut psDel: *mut FEATURE) {
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    //	iVector		dv;
//	UWORD		uwFlameCycles, uwFlameAnims, i;
//	UDWORD		x, y, udwFlameDelay;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"removeFeature: invalid feature pointer\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"feature.c\x00" as *const u8 as *const libc::c_char,
              955 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"removeFeature\x00")).as_ptr(),
              b"PTRVALID(psDel, sizeof(FEATURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psDel).died != 0 {
        // feature has already been killed, quit
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"removeFeature: feature already dead\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"feature.c\x00" as *const u8 as *const libc::c_char,
                  961 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"removeFeature\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    if bMultiPlayer != 0 && ingame.localJoiningInProgress == 0 {
        SendDestroyFeature(psDel);
        // inform other players of destruction
    }
    //remove from the map data
    mapX =
        ((*psDel).x as libc::c_int -
             (*(*psDel).psStats).baseWidth as libc::c_int * 128 as libc::c_int
                 / 2 as libc::c_int >> 7 as libc::c_int) as UDWORD;
    mapY =
        ((*psDel).y as libc::c_int -
             (*(*psDel).psStats).baseBreadth as libc::c_int *
                 128 as libc::c_int / 2 as libc::c_int >> 7 as libc::c_int) as
            UDWORD;
    width = 0 as libc::c_int as UDWORD;
    while width < (*(*psDel).psStats).baseWidth as libc::c_uint {
        breadth = 0 as libc::c_int as UDWORD;
        while breadth < (*(*psDel).psStats).baseBreadth as libc::c_uint {
            //psor		mapTile(mapX+width, mapY+breadth)->psObject = NULL;
            psTile =
                mapTile(mapX.wrapping_add(width), mapY.wrapping_add(breadth));
            //	psTile->tileInfoBits = (UBYTE)(psTile->tileInfoBits & BITS_STRUCTURE_MASK);
			/* Don't need to worry about clearing structure bits - they should not be there! */
            (*psTile).tileInfoBits =
                ((*psTile).tileInfoBits as libc::c_int &
                     !(0x1 as libc::c_int | 0x2 as libc::c_int |
                           0x20 as libc::c_int)) as UBYTE;
            (*psTile).tileInfoBits =
                ((*psTile).tileInfoBits as libc::c_int &
                     !(0x4 as libc::c_int)) as UBYTE;
            (*psTile).tileInfoBits =
                ((*psTile).tileInfoBits as libc::c_int &
                     !(0x80 as libc::c_int)) as UBYTE;
            (*psTile).texture =
                ((*psTile).texture as libc::c_int & !(0x200 as libc::c_int))
                    as UWORD;
            breadth = breadth.wrapping_add(1)
        }
        width = width.wrapping_add(1)
    }
    if (*(*psDel).psStats).subType as libc::c_uint ==
           FEAT_GEN_ARTE as libc::c_int as libc::c_uint {
        pos.x = (*psDel).x as int32;
        pos.z = (*psDel).y as int32;
        pos.y = map_Height(pos.x as UDWORD, pos.z as UDWORD) as int32;
        addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_DISCOVERY,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        scoreUpdateVar(WD_ARTEFACTS_FOUND);
        intRefreshScreen();
    }
    if (*(*psDel).psStats).subType as libc::c_uint ==
           FEAT_GEN_ARTE as libc::c_int as libc::c_uint ||
           (*(*psDel).psStats).subType as libc::c_uint ==
               FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
        // have to check all players cos if you cheat you'll get em.
        player = 0 as libc::c_int as UDWORD;
        while player < 8 as libc::c_int as libc::c_uint {
            //see if there is a proximity message FOR THE SELECTED PLAYER at this location
            psMessage =
                findMessage(psDel as *mut MSG_VIEWDATA, MSG_PROXIMITY,
                            selectedPlayer);
            while !psMessage.is_null() {
                removeMessage(psMessage, selectedPlayer);
                psMessage =
                    findMessage(psDel as *mut MSG_VIEWDATA, MSG_PROXIMITY,
                                player)
            }
            player = player.wrapping_add(1)
        }
    }
    // remove the feature from the grid
    gridRemoveObject(psDel as *mut BASE_OBJECT);
    killFeature(psDel);
    //once a feature of type FEAT_BUILDING is destroyed - it leaves a wrecked
	//struct FEATURE in its place - ?!
    if (*(*psDel).psStats).subType as libc::c_uint ==
           FEAT_BUILDING as libc::c_int as libc::c_uint {
        mapX =
            ((*psDel).x as libc::c_int -
                 (*(*psDel).psStats).baseWidth as libc::c_int *
                     128 as libc::c_int / 2 as libc::c_int >>
                 7 as libc::c_int) as UDWORD;
        mapY =
            ((*psDel).y as libc::c_int -
                 (*(*psDel).psStats).baseBreadth as libc::c_int *
                     128 as libc::c_int / 2 as libc::c_int >>
                 7 as libc::c_int) as UDWORD
        /*for (width = 0; width < psDel->psStats->baseWidth; width++)
		{
			for (breadth = 0; breadth < psDel->psStats->baseBreadth; breadth++)
			{
				buildFeature((asFeatureStats + structFeature),
						(mapX+width) << TILE_SHIFT, (mapY+breadth) << TILE_SHIFT, FALSE);
			}
		}*/
        //		buildFeature((asFeatureStats + structFeature), mapX << TILE_SHIFT,
//			mapY << TILE_SHIFT, FALSE);
    };
}
/* Remove a Feature and free it's memory */
#[no_mangle]
pub unsafe extern "C" fn destroyFeature(mut psDel: *mut FEATURE) {
    let mut widthScatter: UDWORD = 0;
    let mut breadthScatter: UDWORD = 0;
    let mut heightScatter: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut explosionSize: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut texture: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyFeature: invalid feature pointer\n\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"feature.c\x00" as *const u8 as *const libc::c_char,
              1055 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"destroyFeature\x00")).as_ptr(),
              b"PTRVALID(psDel, sizeof(FEATURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //---------------------------------------------------------------------------------------
 	/* Only add if visible and damageable*/
    if (*psDel).visible[selectedPlayer as usize] as libc::c_int != 0 &&
           (*(*psDel).psStats).damageable != 0 {
        /* Set off a destruction effect */
		/* First Explosions */
        widthScatter = (128 as libc::c_int / 2 as libc::c_int) as UDWORD;
        breadthScatter = (128 as libc::c_int / 2 as libc::c_int) as UDWORD;
        heightScatter = (128 as libc::c_int / 4 as libc::c_int) as UDWORD;
        //set which explosion to use based on size of feature
        if ((*(*psDel).psStats).baseWidth as libc::c_int) < 2 as libc::c_int
               &&
               ((*(*psDel).psStats).baseBreadth as libc::c_int) <
                   2 as libc::c_int {
            explosionSize = EXPLOSION_TYPE_SMALL as libc::c_int as UDWORD
        } else if ((*(*psDel).psStats).baseWidth as libc::c_int) <
                      3 as libc::c_int &&
                      ((*(*psDel).psStats).baseBreadth as libc::c_int) <
                          3 as libc::c_int {
            explosionSize = EXPLOSION_TYPE_MEDIUM as libc::c_int as UDWORD
        } else {
            explosionSize = EXPLOSION_TYPE_LARGE as libc::c_int as UDWORD
        }
        i = 0 as libc::c_int as UDWORD;
        while i < 4 as libc::c_int as libc::c_uint {
            pos.x =
                ((*psDel).x as
                     libc::c_uint).wrapping_add(widthScatter).wrapping_sub((rand()
                                                                                as
                                                                                libc::c_uint).wrapping_rem((2
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint).wrapping_mul(widthScatter)))
                    as int32;
            pos.z =
                ((*psDel).y as
                     libc::c_uint).wrapping_add(breadthScatter).wrapping_sub((rand()
                                                                                  as
                                                                                  libc::c_uint).wrapping_rem((2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint).wrapping_mul(breadthScatter)))
                    as int32;
            pos.y =
                (((*psDel).z as libc::c_int + 32 as libc::c_int) as
                     libc::c_uint).wrapping_add((rand() as
                                                     libc::c_uint).wrapping_rem(heightScatter))
                    as int32;
            addEffect(&mut pos, EFFECT_EXPLOSION,
                      explosionSize as EFFECT_TYPE, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            i = i.wrapping_add(1)
        }
        //	  	if(psDel->sDisplay.imd->ymax>300)	// WARNING - STATS CHANGE NEEDED!!!!!!!!!!!
        if (*(*psDel).psStats).subType as libc::c_uint ==
               FEAT_SKYSCRAPER as libc::c_int as libc::c_uint {
            pos.x = (*psDel).x as int32;
            pos.z = (*psDel).y as int32;
            pos.y = (*psDel).z as int32;
            addEffect(&mut pos, EFFECT_DESTRUCTION,
                      DESTRUCTION_TYPE_SKYSCRAPER, 1 as libc::c_int,
                      (*psDel).sDisplay.imd, 0 as libc::c_int);
            initPerimeterSmoke((*psDel).sDisplay.imd, pos.x as UDWORD,
                               pos.y as UDWORD, pos.z as UDWORD);
            // ----- Flip all the tiles under the skyscraper to a rubble tile
			// smoke effect should disguise this happening
            mapX =
                ((*psDel).x as libc::c_int -
                     (*(*psDel).psStats).baseWidth as libc::c_int *
                         128 as libc::c_int / 2 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            mapY =
                ((*psDel).y as libc::c_int -
                     (*(*psDel).psStats).baseBreadth as libc::c_int *
                         128 as libc::c_int / 2 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            //			if(psDel->sDisplay.imd->ymax>300)
            if (*(*psDel).psStats).subType as libc::c_uint ==
                   FEAT_SKYSCRAPER as libc::c_int as libc::c_uint {
                width = 0 as libc::c_int as UDWORD;
                while width < (*(*psDel).psStats).baseWidth as libc::c_uint {
                    breadth = 0 as libc::c_int as UDWORD;
                    while breadth <
                              (*(*psDel).psStats).baseBreadth as libc::c_uint
                          {
                        psTile =
                            mapTile(mapX.wrapping_add(width),
                                    mapY.wrapping_add(breadth));
                        // stops water texture chnaging for underwateer festures
                        if terrainTypes[((*psTile).texture as libc::c_int &
                                             0x1ff as libc::c_int) as usize]
                               as libc::c_int != TER_WATER as libc::c_int {
                            if terrainTypes[((*psTile).texture as libc::c_int
                                                 & 0x1ff as libc::c_int) as
                                                usize] as libc::c_int !=
                                   TER_CLIFFFACE as libc::c_int {
                                /* Clear feature bits */
                                (*mapTile(mapX.wrapping_add(width),
                                          mapY.wrapping_add(breadth))).tileInfoBits
                                    =
                                    ((*mapTile(mapX.wrapping_add(width),
                                               mapY.wrapping_add(breadth))).tileInfoBits
                                         as libc::c_int &
                                         !(0x1 as libc::c_int |
                                               0x2 as libc::c_int |
                                               0x20 as libc::c_int)) as
                                        UBYTE; //getRubbleTileNum();
                                texture =
                                    ((*psTile).texture as libc::c_int &
                                         !(0x1ff as libc::c_int)) as UDWORD;
                                texture |= 54 as libc::c_int as libc::c_uint;
                                (*psTile).texture = texture as UWORD
                            } else {
                                /* This remains a blocking tile */
                                (*mapTile(mapX.wrapping_add(width),
                                          mapY.wrapping_add(breadth))).tileInfoBits
                                    =
                                    ((*mapTile(mapX.wrapping_add(width),
                                               mapY.wrapping_add(breadth))).tileInfoBits
                                         as libc::c_int &
                                         !(0x1 as libc::c_int |
                                               0x2 as libc::c_int |
                                               0x20 as libc::c_int)) as
                                        UBYTE; //getRubbleTileNum();
                                texture =
                                    ((*psTile).texture as libc::c_int &
                                         !(0x1ff as libc::c_int)) as UDWORD;
                                texture |= 67 as libc::c_int as libc::c_uint;
                                (*psTile).texture = texture as UWORD
                            }
                        }
                        breadth = breadth.wrapping_add(1)
                    }
                    width = width.wrapping_add(1)
                }
                // -------
                shakeStart();
            }
        }
        /* Then a sequence of effects */
        pos.x = (*psDel).x as int32;
        pos.z = (*psDel).y as int32;
        pos.y = map_Height(pos.x as UDWORD, pos.z as UDWORD) as int32;
        addEffect(&mut pos, EFFECT_DESTRUCTION, DESTRUCTION_TYPE_FEATURE,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        //play sound
		// ffs gj
        if (*(*psDel).psStats).subType as libc::c_uint ==
               FEAT_SKYSCRAPER as libc::c_int as libc::c_uint {
            audio_PlayStaticTrack((*psDel).x as SDWORD, (*psDel).y as SDWORD,
                                  ID_SOUND_BUILDING_FALL as libc::c_int);
        } else {
            audio_PlayStaticTrack((*psDel).x as SDWORD, (*psDel).y as SDWORD,
                                  ID_SOUND_EXPLOSION as libc::c_int);
        }
    }
    //---------------------------------------------------------------------------------------
    removeFeature(psDel);
}
#[no_mangle]
pub unsafe extern "C" fn getFeatureStatFromName(mut pName: *mut STRING)
 -> SDWORD {
    let mut inc: UDWORD = 0;
    let mut psStat: *mut FEATURE_STATS = 0 as *mut FEATURE_STATS;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numFeatureStats {
        psStat =
            &mut *asFeatureStats.offset(inc as isize) as *mut FEATURE_STATS;
        if strcmp((*psStat).pName, pName) == 0 { return inc as SDWORD }
        inc = inc.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
/*
 * Feature.h
 *
 * Definitions for the feature structures.
 *
 */
//they're just not there anymore!!!!! Ye ha!
/* The statistics for the features */
//Value is stored for easy access to this feature in destroyDroid()/destroyStruct()
//extern UDWORD			droidFeature;
/* Load the feature stats */
/* Release the feature stats memory */
// Set the tile no draw flags for a structure
/* Create a feature on the map */
/* Release the resources associated with a feature */
/* Update routine for features */
// free up a feature with no visual effects
/* Remove a Feature and free it's memory */
/* get a feature stat id from its name */
/*looks around the given droid to see if there is any building 
wreckage to clear*/
/*looks around the given droid to see if there is any building
wreckage to clear*/
#[no_mangle]
pub unsafe extern "C" fn checkForWreckage(mut psDroid: *mut DROID)
 -> *mut FEATURE {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut startX: UDWORD = 0;
    let mut startY: UDWORD = 0;
    let mut incX: UDWORD = 0;
    let mut incY: UDWORD = 0;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    startX = ((*psDroid).x as libc::c_int >> 7 as libc::c_int) as UDWORD;
    startY = ((*psDroid).y as libc::c_int >> 7 as libc::c_int) as UDWORD;
    //look around the droid - max 2 tiles distance
    incX = 1 as libc::c_int as UDWORD;
    incY = 1 as libc::c_int as UDWORD;
    while incX < 3 as libc::c_int as libc::c_uint {
        /* across the top */
        y = startY.wrapping_sub(incY) as SDWORD;
        x = startX.wrapping_sub(incX) as SDWORD;
        while x < startX.wrapping_add(incX) as SDWORD {
            if (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits as
                   libc::c_int & 0x2 as libc::c_int != 0 {
                psFeature = getTileFeature(x as UDWORD, y as UDWORD);
                if !psFeature.is_null() &&
                       (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    return psFeature
                }
            }
            x += 1
        }
        /* the right */
        x = startX.wrapping_add(incX) as SDWORD;
        y = startY.wrapping_sub(incY) as SDWORD;
        while y < startY.wrapping_add(incY) as SDWORD {
            if (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits as
                   libc::c_int & 0x2 as libc::c_int != 0 {
                psFeature = getTileFeature(x as UDWORD, y as UDWORD);
                if !psFeature.is_null() &&
                       (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    return psFeature
                }
            }
            y += 1
        }
        /* across the bottom*/
        y = startY.wrapping_add(incY) as SDWORD;
        x = startX.wrapping_add(incX) as SDWORD;
        while x > startX.wrapping_sub(incX) as SDWORD {
            if (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits as
                   libc::c_int & 0x2 as libc::c_int != 0 {
                psFeature = getTileFeature(x as UDWORD, y as UDWORD);
                if !psFeature.is_null() &&
                       (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    return psFeature
                }
            }
            x -= 1
        }
        /* the left */
        x = startX.wrapping_sub(incX) as SDWORD;
        y = startY.wrapping_add(incY) as SDWORD;
        while y > startY.wrapping_sub(incY) as SDWORD {
            if (*mapTile(x as UDWORD, y as UDWORD)).tileInfoBits as
                   libc::c_int & 0x2 as libc::c_int != 0 {
                psFeature = getTileFeature(x as UDWORD, y as UDWORD);
                if !psFeature.is_null() &&
                       (*(*psFeature).psStats).subType as libc::c_uint ==
                           FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    return psFeature
                }
            }
            y -= 1
        }
        incX = incX.wrapping_add(1);
        incY = incY.wrapping_add(1)
    }
    return 0 as *mut FEATURE;
}
