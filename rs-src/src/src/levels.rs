use ::libc;
extern "C" {
    /* Set a block heap to use for all memory allocation rather than standard malloc/free */
    #[no_mangle]
    fn memSetBlockHeap(psHeap: *mut _block_heap);
    /* Get the current block heap */
    #[no_mangle]
    fn memGetBlockHeap() -> *mut _block_heap;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn blkReset(psHeap: *mut BLOCK_HEAP);
    #[no_mangle]
    fn resToLower(pStr: *mut STRING);
    /* Copyright (C) 1991-2019 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <http://www.gnu.org/licenses/>.  */
    /*
 *	ISO C99 Standard: 7.20 General utilities	<stdlib.h>
 */
    /* Get size_t, wchar_t and NULL from <stddef.h>.  */
    /* XPG requires a few symbols from <sys/wait.h> being defined.  */
    /* Define the macros <sys/wait.h> also would define this way.  */
    /* X/Open or XPG7 and <sys/wait.h> not included.  */
    /* _FloatN API tests for enablement.  */
    /* Returned by `div'.  */
    /* Quotient.  */
    /* Remainder.  */
    /* Returned by `ldiv'.  */
    /* Quotient.  */
    /* Remainder.  */
    /* Returned by `lldiv'.  */
    /* Quotient.  */
    /* Remainder.  */
    /* The largest number rand will return (same as INT_MAX).  */
    /* We define these the same for all machines.
   Changes from this to the outside world should be done in `_exit'.  */
    /* Failing exit status.  */
    /* Successful exit status.  */
    /* Maximum length of a multibyte character in the current locale.  */
    /* Convert a string to a floating-point number.  */
    /* Convert a string to an integer.  */
    /* Convert a string to a long integer.  */
    /* Convert a string to a long long integer.  */
    /* Convert a string to a floating-point number.  */
    /* Likewise for `float' and `long double' sizes of floating-point numbers.  */
    /* Likewise for '_FloatN' and '_FloatNx'.  */
    /* Convert a string to a long integer.  */
    /* Convert a string to an unsigned long integer.  */
    /* Convert a string to a quadword integer.  */
    /* Convert a string to an unsigned quadword integer.  */
    /* Use misc.  */
    /* Convert a string to a quadword integer.  */
    /* Convert a string to an unsigned quadword integer.  */
    /* ISO C99 or use MISC.  */
    /* Convert a floating-point number to a string.  */
    /* Parallel versions of the functions above which take the locale to
   use as an additional parameter.  These are GNU extensions inspired
   by the POSIX.1-2008 extended locale API.  */
    /* GNU */
    /* Optimizing and Inlining.  */
    /* Convert N to base 64 using the digits "./0-9A-Za-z", least-significant
   digit first.  Returns a pointer to static storage overwritten by the
   next call.  */
    /* Read a number from a string S in base 64 as above.  */
    /* Use misc || extended X/Open.  */
    /* we need int32_t... */
    /* These are the functions that actually do things.  The `random', `srandom',
   `initstate' and `setstate' functions are those from BSD Unices.
   The `rand' and `srand' functions are required by the ANSI standard.
   We provide both interfaces to the same random number generator.  */
/* Return a random long integer between 0 and RAND_MAX inclusive.  */
    /* Seed the random number generator with the given number.  */
    /* Initialize the random number generator to use state buffer STATEBUF,
   of length STATELEN, and seed it with SEED.  Optimal lengths are 8, 16,
   32, 64, 128 and 256, the bigger the better; values less than 8 will
   cause an error and values greater than 256 will be rounded down.  */
    /* Switch the random number generator to state buffer STATEBUF,
   which should have been previously initialized by `initstate'.  */
    /* Reentrant versions of the `random' family of functions.
   These functions all use the following data structure to contain
   state, rather than global state variables.  */
    /* Front pointer.  */
    /* Rear pointer.  */
    /* Array of state values.  */
    /* Type of random number generator.  */
    /* Degree of random number generator.  */
    /* Distance between front and rear.  */
    /* Pointer behind state table.  */
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    /* Seed the random number generator with the given number.  */
    /* Reentrant interface according to POSIX.1.  */
    /* System V style 48-bit random number generator functions.  */
    /* Return non-negative, double-precision floating-point value in [0.0,1.0).  */
    /* Return non-negative, long integer in [0,2^31).  */
    /* Return signed, long integers in [-2^31,2^31).  */
    /* Seed random number generator.  */
    /* Data structure for communication with thread safe versions.  This
   type is to be regarded as opaque.  It's only exported because users
   have to allocate objects of this type.  */
    /* Current state.  */
    /* Old state.  */
    /* Additive const. in congruential formula.  */
    /* Flag for initializing.  */
    /* Factor in congruential
						   formula.  */
    /* Return non-negative, double-precision floating-point value in [0.0,1.0).  */
    /* Return non-negative, long integer in [0,2^31).  */
    /* Return signed, long integers in [-2^31,2^31).  */
    /* Seed random number generator.  */
    /* Use misc.  */
    /* Use misc or X/Open.  */
    /* Allocate SIZE bytes of memory.  */
    /* Allocate NMEMB elements of SIZE bytes each, all initialized to 0.  */
    /* Re-allocate the previously allocated block
   in PTR, making the new block SIZE bytes long.  */
/* __attribute_malloc__ is not used, because if realloc returns
   the same pointer that was passed to it, aliasing needs to be allowed
   between objects pointed by the old and new pointers.  */
    /* Re-allocate the previously allocated block in PTR, making the new
   block large enough for NMEMB elements of SIZE bytes each.  */
/* __attribute_malloc__ is not used, because if reallocarray returns
   the same pointer that was passed to it, aliasing needs to be allowed
   between objects pointed by the old and new pointers.  */
    /* Free a block allocated by `malloc', `realloc' or `calloc'.  */
    /* Use misc.  */
    /* Allocate SIZE bytes on a page boundary.  The storage cannot be freed.  */
    /* Allocate memory of SIZE bytes with an alignment of ALIGNMENT.  */
    /* ISO C variant of aligned allocation.  */
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn resLoad(pResFile: *mut STRING, blockID: SDWORD,
               pLoadBuffer: *mut libc::c_char, bufferSize: SDWORD,
               psMemHeap: *mut _block_heap) -> BOOL;
    #[no_mangle]
    fn resReleaseBlockData(blockID: SDWORD);
    #[no_mangle]
    fn stageOneInitialise() -> BOOL;
    #[no_mangle]
    fn stageOneShutDown() -> BOOL;
    #[no_mangle]
    fn stageTwoInitialise() -> BOOL;
    #[no_mangle]
    fn stageTwoShutDown() -> BOOL;
    #[no_mangle]
    fn stageThreeInitialise() -> BOOL;
    #[no_mangle]
    fn stageThreeShutDown() -> BOOL;
    #[no_mangle]
    fn gameReset() -> BOOL;
    #[no_mangle]
    fn newMapInitialise() -> BOOL;
    // Reset the game between campaigns
    #[no_mangle]
    fn campaignReset() -> BOOL;
    // Reset the game when loading a save game
    #[no_mangle]
    fn saveGameReset() -> BOOL;
    #[no_mangle]
    fn rebuildSearchPath(mode: searchPathMode, force: BOOL) -> BOOL;
    // the block heap for the game data
    #[no_mangle]
    static mut psGameHeap: *mut BLOCK_HEAP;
    // the block heap for the campaign map
    #[no_mangle]
    static mut psMapHeap: *mut BLOCK_HEAP;
    // the block heap for the pre WRF data
    #[no_mangle]
    static mut psMissionHeap: *mut BLOCK_HEAP;
    #[no_mangle]
    static mut DisplayBuffer: *mut libc::c_char;
    #[no_mangle]
    static mut displayBufferSize: UDWORD;
    #[no_mangle]
    fn startMissionSave(missionType: SDWORD) -> BOOL;
    #[no_mangle]
    fn startMission(missionType: UDWORD, pGame: *mut STRING) -> BOOL;
    // the lexer function
    #[no_mangle]
    fn lev_lex() -> libc::c_int;
    /* Set the current input buffer for the lexer */
    #[no_mangle]
    fn levSetInputBuffer(pBuffer: *mut libc::c_char, size: UDWORD);
    #[no_mangle]
    fn levGetErrorData(pLine: *mut libc::c_int,
                       ppText: *mut *mut libc::c_char);
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn loadGame(pGameToLoad: *mut STRING, keepObjects: BOOL, freeMem: BOOL,
                UserSaveGame: BOOL) -> BOOL;
    #[no_mangle]
    fn loadMissionExtras(pGameToLoad: *mut STRING, levelType: SWORD) -> BOOL;
    // load the script state given a .gam name
    #[no_mangle]
    fn loadScriptState(pFileName: *mut STRING) -> BOOL;
    #[no_mangle]
    fn dataSetSaveFlag();
    #[no_mangle]
    fn dataClearSaveFlag();
    //*************************************************************************
    #[no_mangle]
    fn iV_Reset(bResetPal: libc::c_int);
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
}
pub type size_t = libc::c_uint;
pub type UBYTE = libc::c_uchar;
pub type STRING = libc::c_char;
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
pub type BLOCK_HEAP_MEM = _block_heap_mem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap_mem {
    pub size: SDWORD,
    pub pFree: *mut UBYTE,
    pub pMem: *mut UBYTE,
    pub pLastAllocated: *mut UBYTE,
    pub psNext: *mut _block_heap_mem,
}
pub type BLOCK_HEAP = _block_heap;
pub type searchPathMode = libc::c_uint;
pub const mod_multiplay: searchPathMode = 2;
pub const mod_campaign: searchPathMode = 1;
pub const mod_clean: searchPathMode = 0;
// types of level datasets
pub type _level_type = libc::c_uint;
//flags when not got a mission to go back to or when 
//already on one - ****LEAVE AS LAST ONE****
// off map saving any droids (selectedPlayer) at end into apsLimboDroids
pub const LDS_NONE: _level_type = 10;
// expand campaign map using droids held in apsLimboDroids
pub const LDS_MKEEP_LIMBO: _level_type = 9;
// off map mission (extra map data)
pub const LDS_EXPAND_LIMBO: _level_type = 8;
// off map mission (extra map data)
pub const LDS_MCLEAR: _level_type = 7;
// pause between missions
pub const LDS_MKEEP: _level_type = 6;
// extra data for expanding a campaign map
pub const LDS_BETWEEN: _level_type = 5;
// data for changing between levels
pub const LDS_EXPAND: _level_type = 4;
// mapdata for the start of a campaign
pub const LDS_CAMCHANGE: _level_type = 3;
// the data set for a campaign (no map data)
pub const LDS_CAMSTART: _level_type = 2;
// all data required for a stand alone level
pub const LDS_CAMPAIGN: _level_type = 1;
pub const LDS_COMPLETE: _level_type = 0;
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
// dataset token received
pub const LP_WAITDATA: C2RustUnnamed_0 = 6;
// data token received
pub const LP_GAME: C2RustUnnamed_0 = 8;
// defining level data, waiting for data token
pub const LP_DATA: C2RustUnnamed_0 = 7;
// an identifier
pub const LTK_STRING: _token_type = 272;
// type token received
pub const LP_DATASET: C2RustUnnamed_0 = 5;
// level token received
pub const LP_LEVELDONE: C2RustUnnamed_0 = 2;
// no input received
pub const LP_LEVEL: C2RustUnnamed_0 = 1;
// miss_clear key word
pub const LTK_IDENT: _token_type = 271;
// data key word
pub const LTK_GAME: _token_type = 260;
// type key word
pub const LTK_DATA: _token_type = 259;
// camchange key word
pub const LTK_DATASET: _token_type = 264;
// players token received
pub const LP_TYPE: C2RustUnnamed_0 = 4;
// defined a level waiting for players/type/data
pub const LP_PLAYERS: C2RustUnnamed_0 = 3;
// a quoted string
pub const LTK_INTEGER: _token_type = 273;
// players key word
pub const LTK_TYPE: _token_type = 258;
// level key word
pub const LTK_PLAYERS: _token_type = 257;
// miss_keep key word
pub const LTK_MKEEP_LIMBO: _token_type = 269;
// expand key word
pub const LTK_EXPAND_LIMBO: _token_type = 266;
// miss_keep Limbo key word
pub const LTK_MCLEAR: _token_type = 270;
// dataset key word
pub const LTK_EXPAND: _token_type = 265;
// camstart key word
pub const LTK_CAMCHANGE: _token_type = 263;
// between key word
pub const LTK_MKEEP: _token_type = 268;
// expand Limbo key word
pub const LTK_BETWEEN: _token_type = 267;
// campaign key word
pub const LTK_CAMSTART: _token_type = 262;
// game key word
pub const LTK_CAMPAIGN: _token_type = 261;
pub const LTK_LEVEL: _token_type = 256;
pub const LP_START: C2RustUnnamed_0 = 0;
pub type TRIGGER_TYPE = _trigger_type;
/* The type of function called to access an object or in-game variable */
/* The possible storage types for a variable */
// Public variable
// Private variable
// A value stored in an objects data space.
// An external value accessed by function call
// A local variable
/* Variable debugging info for a script */
/* Array info for a script */
// the base index of the array values
// the array data type
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
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed = 4;
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed = 3;
/*
 * Internal definitions for the level system
 *
 */
// return values from the lexer
pub type _token_type = libc::c_uint;
// a number
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
pub type C2RustUnnamed = libc::c_uint;
// User saved game - in the middle of a level
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed = 0;
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
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
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
// modes for the parser
pub type C2RustUnnamed_0 = libc::c_uint;
static mut currentLevelName: [libc::c_char; 32] = [0; 32];
// game token received
// the current level descriptions
#[no_mangle]
pub static mut psLevels: *mut LEVEL_DATASET =
    0 as *const LEVEL_DATASET as *mut LEVEL_DATASET;
// the currently loaded data set
#[no_mangle]
pub static mut psBaseData: *mut LEVEL_DATASET =
    0 as *const LEVEL_DATASET as *mut LEVEL_DATASET;
#[no_mangle]
pub static mut psCurrLevel: *mut LEVEL_DATASET =
    0 as *const LEVEL_DATASET as *mut LEVEL_DATASET;
// dummy level data for single WRF loads
#[no_mangle]
pub static mut sSingleWRF: LEVEL_DATASET =
    LEVEL_DATASET{type_0: 0,
                  players: 0,
                  game: 0,
                  pName: 0 as *const STRING as *mut STRING,
                  dataDir: 0,
                  apDataFiles: [0 as *const STRING as *mut STRING; 9],
                  psBaseData:
                      0 as *const _level_dataset as *mut _level_dataset,
                  psChange: 0 as *const _level_dataset as *mut _level_dataset,
                  psNext: 0 as *const _level_dataset as *mut _level_dataset,};
// return values from the lexer
#[no_mangle]
pub static mut pLevToken: *mut STRING = 0 as *const STRING as *mut STRING;
#[no_mangle]
pub static mut levVal: SDWORD = 0;
#[no_mangle]
pub static mut levelLoadType: SDWORD = 0;
/*// the current data file to parse
static UBYTE	*pDataFile;
static SDWORD	dataFileSize;

// the current position in the data file
static UBYTE	*pDataPtr;
static SDWORD	levLine;

// the token buffer
#define TOKEN_MAX	255
static STRING	aTokenBuff[TOKEN_MAX];
*/
// initialise the level system
#[no_mangle]
pub unsafe extern "C" fn levInitialise() -> BOOL {
    psLevels = 0 as *mut LEVEL_DATASET;
    psBaseData = 0 as *mut LEVEL_DATASET;
    psCurrLevel = 0 as *mut LEVEL_DATASET;
    return 1 as libc::c_int;
}
//get the type of level currently being loaded of GTYPE type
#[no_mangle]
pub unsafe extern "C" fn getLevelLoadType() -> SDWORD {
    return levelLoadType;
}
// shutdown the level system
// shutdown the level system
#[no_mangle]
pub unsafe extern "C" fn levShutDown() {
    let mut psNext: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut i: SDWORD = 0;
    while !psLevels.is_null() {
        memFreeRelease((*psLevels).pName as *mut libc::c_void);
        (*psLevels).pName = 0 as *mut STRING;
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if !(*psLevels).apDataFiles[i as usize].is_null() {
                memFreeRelease((*psLevels).apDataFiles[i as usize] as
                                   *mut libc::c_void);
                (*psLevels).apDataFiles[i as usize] = 0 as *mut STRING
            }
            i += 1
        }
        psNext = (*psLevels).psNext;
        memFreeRelease(psLevels as *mut libc::c_void);
        psLevels = 0 as *mut LEVEL_DATASET;
        psLevels = psNext
    };
}
// return values from the lexer
// error report function for the level parser
// error report function for the level parser
#[no_mangle]
pub unsafe extern "C" fn levError(mut pError: *mut STRING) {
    let mut pText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: libc::c_int = 0;
    levGetErrorData(&mut line, &mut pText);
    debug(LOG_ERROR,
          b"Level File parse error:\n%s at line %d text %s\n\x00" as *const u8
              as *const libc::c_char, pError, line, pText);
}
// find the level dataset
// find the level dataset
#[no_mangle]
pub unsafe extern "C" fn levFindDataSet(mut pName: *mut STRING,
                                        mut ppsDataSet:
                                            *mut *mut LEVEL_DATASET) -> BOOL {
    let mut psNewLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    psNewLevel = psLevels;
    while !psNewLevel.is_null() {
        if !(*psNewLevel).pName.is_null() {
            if strcmp((*psNewLevel).pName, pName) == 0 as libc::c_int {
                *ppsDataSet = psNewLevel;
                return 1 as libc::c_int
            }
        }
        psNewLevel = (*psNewLevel).psNext
    }
    return 0 as libc::c_int;
}
// the current level descriptions
// parse a level description data file
// parse a level description data file
#[no_mangle]
pub unsafe extern "C" fn levParse(mut pBuffer: *mut libc::c_char,
                                  mut size: SDWORD, mut datadir: libc::c_int)
 -> BOOL {
    let mut token: SDWORD = 0;
    let mut state: SDWORD = 0;
    let mut currData: SDWORD = 0 as libc::c_int;
    let mut psDataSet: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut psFoundData: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    levSetInputBuffer(pBuffer, size as UDWORD);
    state = LP_START as libc::c_int;
    token = lev_lex();
    while token != 0 as libc::c_int {
        match token {
            256 | 261 | 262 | 263 | 265 | 267 | 268 | 270 | 266 | 269 => {
                if state == LP_START as libc::c_int ||
                       state == LP_WAITDATA as libc::c_int {
                    // start a new level data set
                    psDataSet =
                        memMallocRelease(::std::mem::size_of::<LEVEL_DATASET>()
                                             as libc::c_ulong) as
                            *mut LEVEL_DATASET;
                    if psDataSet.is_null() {
                        levError(b"Out of memory\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    memset(psDataSet as *mut libc::c_void, 0 as libc::c_int,
                           ::std::mem::size_of::<LEVEL_DATASET>() as
                               libc::c_ulong);
                    (*psDataSet).players = 1 as libc::c_int as SWORD;
                    (*psDataSet).game = -(1 as libc::c_int) as SWORD;
                    (*psDataSet).dataDir = datadir;
                    let mut psPrev: *mut LEVEL_DATASET =
                        0 as *mut LEVEL_DATASET;
                    let mut psCurr: *mut LEVEL_DATASET =
                        0 as *mut LEVEL_DATASET;
                    psPrev = 0 as *mut LEVEL_DATASET;
                    psCurr = psLevels;
                    while !psCurr.is_null() {
                        psPrev = psCurr;
                        psCurr = (*psCurr).psNext
                    }
                    if !psPrev.is_null() {
                        (*psPrev).psNext = psDataSet
                    } else {
                        (*psDataSet).psNext = 0 as *mut _level_dataset;
                        psLevels = psDataSet
                    }
                    currData = 0 as libc::c_int;
                    // set the dataset type
                    match token {
                        256 => {
                            (*psDataSet).type_0 =
                                LDS_COMPLETE as libc::c_int as SWORD
                        }
                        261 => {
                            (*psDataSet).type_0 =
                                LDS_CAMPAIGN as libc::c_int as SWORD
                        }
                        262 => {
                            (*psDataSet).type_0 =
                                LDS_CAMSTART as libc::c_int as SWORD
                        }
                        267 => {
                            (*psDataSet).type_0 =
                                LDS_BETWEEN as libc::c_int as SWORD
                        }
                        268 => {
                            (*psDataSet).type_0 =
                                LDS_MKEEP as libc::c_int as SWORD
                        }
                        263 => {
                            (*psDataSet).type_0 =
                                LDS_CAMCHANGE as libc::c_int as SWORD
                        }
                        265 => {
                            (*psDataSet).type_0 =
                                LDS_EXPAND as libc::c_int as SWORD
                        }
                        270 => {
                            (*psDataSet).type_0 =
                                LDS_MCLEAR as libc::c_int as SWORD
                        }
                        266 => {
                            (*psDataSet).type_0 =
                                LDS_EXPAND_LIMBO as libc::c_int as SWORD
                        }
                        269 => {
                            (*psDataSet).type_0 =
                                LDS_MKEEP_LIMBO as libc::c_int as SWORD
                        }
                        _ => {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"eh?\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"levels.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      230 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 9],
                                                                &[libc::c_char; 9]>(b"levParse\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                        }
                    }
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
                state = LP_LEVEL as libc::c_int
            }
            257 => {
                if state == LP_LEVELDONE as libc::c_int &&
                       ((*psDataSet).type_0 as libc::c_int ==
                            LDS_COMPLETE as libc::c_int ||
                            (*psDataSet).type_0 as libc::c_int >=
                                10 as libc::c_int) {
                    state = LP_PLAYERS as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            258 => {
                if state == LP_LEVELDONE as libc::c_int &&
                       (*psDataSet).type_0 as libc::c_int ==
                           LDS_COMPLETE as libc::c_int {
                    state = LP_TYPE as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            273 => {
                if state == LP_PLAYERS as libc::c_int {
                    (*psDataSet).players = levVal as SWORD
                } else if state == LP_TYPE as libc::c_int {
                    if levVal < 10 as libc::c_int {
                        levError(b"invalid type number\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    (*psDataSet).type_0 = levVal as SWORD
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
                state = LP_LEVELDONE as libc::c_int
            }
            264 => {
                if state == LP_LEVELDONE as libc::c_int &&
                       (*psDataSet).type_0 as libc::c_int !=
                           LDS_COMPLETE as libc::c_int {
                    state = LP_DATASET as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            259 => {
                if state == LP_WAITDATA as libc::c_int {
                    state = LP_DATA as libc::c_int
                } else if state == LP_LEVELDONE as libc::c_int {
                    if (*psDataSet).type_0 as libc::c_int ==
                           LDS_CAMSTART as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_MKEEP as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_CAMCHANGE as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_EXPAND as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_MCLEAR as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_EXPAND_LIMBO as libc::c_int ||
                           (*psDataSet).type_0 as libc::c_int ==
                               LDS_MKEEP_LIMBO as libc::c_int {
                        levError(b"Missing dataset command\x00" as *const u8
                                     as *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    state = LP_DATA as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            260 => {
                if (state == LP_WAITDATA as libc::c_int ||
                        state == LP_LEVELDONE as libc::c_int) &&
                       (*psDataSet).game as libc::c_int == -(1 as libc::c_int)
                       &&
                       (*psDataSet).type_0 as libc::c_int !=
                           LDS_CAMPAIGN as libc::c_int {
                    state = LP_GAME as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            271 => {
                if state == LP_LEVEL as libc::c_int {
                    if (*psDataSet).type_0 as libc::c_int ==
                           LDS_CAMCHANGE as libc::c_int {
                        // campaign change dataset - need to find the full data set
                        if levFindDataSet(pLevToken, &mut psFoundData) == 0 {
                            levError(b"Cannot find full data set for camchange\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut STRING);
                            return 0 as libc::c_int
                        }
                        if (*psFoundData).type_0 as libc::c_int !=
                               LDS_CAMSTART as libc::c_int {
                            levError(b"Invalid data set name for cam change\x00"
                                         as *const u8 as *const libc::c_char
                                         as *mut STRING);
                            return 0 as libc::c_int
                        }
                        (*psFoundData).psChange = psDataSet
                    }
                    // store the level name
                    (*psDataSet).pName =
                        memMallocRelease(strlen(pLevToken).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                            as *mut STRING;
                    if (*psDataSet).pName.is_null() {
                        levError(b"Out of memory\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    strcpy((*psDataSet).pName, pLevToken);
                    state = LP_LEVELDONE as libc::c_int
                } else if state == LP_DATASET as libc::c_int {
                    // find the dataset
                    if levFindDataSet(pLevToken, &mut (*psDataSet).psBaseData)
                           == 0 {
                        levError(b"Unknown dataset\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    state = LP_WAITDATA as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            272 => {
                if state == LP_DATA as libc::c_int ||
                       state == LP_GAME as libc::c_int {
                    if currData >= 9 as libc::c_int {
                        levError(b"Too many data files\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    // note the game index if necessary
                    if state == LP_GAME as libc::c_int {
                        (*psDataSet).game = currData as SWORD
                    }
                    // store the data name
                    (*psDataSet).apDataFiles[currData as usize] =
                        memMallocRelease(strlen(pLevToken).wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                            as *mut STRING;
                    if (*psDataSet).apDataFiles[currData as usize].is_null() {
                        levError(b"Out of memory\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING);
                        return 0 as libc::c_int
                    }
                    resToLower(pLevToken);
                    strcpy((*psDataSet).apDataFiles[currData as usize],
                           pLevToken);
                    currData += 1 as libc::c_int;
                    state = LP_WAITDATA as libc::c_int
                } else {
                    levError(b"Syntax Error\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING);
                    return 0 as libc::c_int
                }
            }
            _ => {
                levError(b"Unexpected token\x00" as *const u8 as
                             *const libc::c_char as *mut STRING);
            }
        }
        // get the next token
        token = lev_lex()
    }
    if state != LP_WAITDATA as libc::c_int || currData == 0 as libc::c_int {
        levError(b"Unexpected end of file\x00" as *const u8 as
                     *const libc::c_char as *mut STRING);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// free the data for the current mission
// free the data for the current mission
#[no_mangle]
pub unsafe extern "C" fn levReleaseMissionData() -> BOOL {
    let mut i: SDWORD = 0;
    // release old data if any was loaded
    if !psCurrLevel.is_null() {
        if stageThreeShutDown() == 0 { return 0 as libc::c_int }
        if ((*psCurrLevel).type_0 as libc::c_int ==
                LDS_COMPLETE as libc::c_int ||
                (*psCurrLevel).type_0 as libc::c_int >= 10 as libc::c_int) &&
               (*psCurrLevel).game as libc::c_int == -(1 as libc::c_int) {
            blkReset(psMissionHeap);
        }
        // free up the old data
        i = 9 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if i == (*psCurrLevel).game as libc::c_int {
                blkReset(psMissionHeap);
                if (*psCurrLevel).psBaseData.is_null() {
                    if stageTwoShutDown() == 0 { return 0 as libc::c_int }
                }
            } else {
                // if (psCurrLevel->apDataFiles[i])
                resReleaseBlockData(i + 9 as libc::c_int);
            }
            i -= 1
        }
        if (*psCurrLevel).type_0 as libc::c_int == LDS_BETWEEN as libc::c_int
           {
            blkReset(psMissionHeap);
        }
    }
    return 1 as libc::c_int;
}
// free the currently loaded dataset
// free the currently loaded dataset
#[no_mangle]
pub unsafe extern "C" fn levReleaseAll() -> BOOL {
    let mut i: SDWORD = 0;
    // release old data if any was loaded
    if !psCurrLevel.is_null() {
        if levReleaseMissionData() == 0 { return 0 as libc::c_int }
        // release the game data
        if !(*psCurrLevel).psBaseData.is_null() {
            if stageTwoShutDown() == 0 { return 0 as libc::c_int }
        }
        if !(*psCurrLevel).psBaseData.is_null() {
            i = 9 as libc::c_int - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if !(*(*psCurrLevel).psBaseData).apDataFiles[i as
                                                                 usize].is_null()
                   {
                    resReleaseBlockData(i);
                }
                i -= 1
            }
        }
        if stageOneShutDown() == 0 { return 0 as libc::c_int }
        blkReset(psGameHeap);
    }
    psCurrLevel = 0 as *mut LEVEL_DATASET;
    return 1 as libc::c_int;
}
// load up a single wrf file
#[no_mangle]
pub unsafe extern "C" fn levLoadSingleWRF(mut pName: *mut STRING) -> BOOL {
    // free the old data
    levReleaseAll();
    // create the dummy level data
    memset(&mut sSingleWRF as *mut LEVEL_DATASET as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<LEVEL_DATASET>() as libc::c_ulong);
    sSingleWRF.pName = pName;
    // load up the WRF
    if stageOneInitialise() == 0 { return 0 as libc::c_int }
    blkReset(psGameHeap);
    memSetBlockHeap(psGameHeap);
    // load the data
    debug(LOG_WZ,
          b"levLoadSingleWRF: Loading %s ...\x00" as *const u8 as
              *const libc::c_char, pName);
    if resLoad(pName, 0 as libc::c_int, DisplayBuffer,
               displayBufferSize as SDWORD, psGameHeap) == 0 {
        return 0 as libc::c_int
    }
    blkReset(psMissionHeap);
    memSetBlockHeap(psMissionHeap);
    if stageThreeInitialise() == 0 { return 0 as libc::c_int }
    psCurrLevel = &mut sSingleWRF;
    return 1 as libc::c_int;
}
// load up the base data set for a level (used by savegames)
// load up the base data set for a level (used by savegames)
#[no_mangle]
pub unsafe extern "C" fn levLoadBaseData(mut pName: *mut STRING) -> BOOL {
    let mut psNewLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut psBaseData_0: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut i: SDWORD = 0;
    debug(LOG_WZ,
          b"Loading base data for level %s\x00" as *const u8 as
              *const libc::c_char, pName);
    // find the level dataset
    if levFindDataSet(pName, &mut psNewLevel) == 0 {
        debug(LOG_ERROR,
              b"levLoadBaseData: couldn\'t find level data\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    if (*psNewLevel).type_0 as libc::c_int != LDS_CAMSTART as libc::c_int &&
           (*psNewLevel).type_0 as libc::c_int != LDS_MKEEP as libc::c_int &&
           (*psNewLevel).type_0 as libc::c_int != LDS_EXPAND as libc::c_int &&
           (*psNewLevel).type_0 as libc::c_int != LDS_MCLEAR as libc::c_int &&
           (*psNewLevel).type_0 as libc::c_int !=
               LDS_EXPAND_LIMBO as libc::c_int &&
           (*psNewLevel).type_0 as libc::c_int !=
               LDS_MKEEP_LIMBO as libc::c_int {
        debug(LOG_ERROR,
              b"levLoadBaseData: incorect level type\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    // clear all the old data
    levReleaseAll();
    // basic game data is loaded in the game heap
    memSetBlockHeap(psGameHeap);
    // initialise
    blkReset(psGameHeap);
    if stageOneInitialise() == 0 { return 0 as libc::c_int }
    // load up the base dataset
    psBaseData_0 = (*psNewLevel).psBaseData;
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if !(*psBaseData_0).apDataFiles[i as usize].is_null() {
            // load the data
            debug(LOG_WZ,
                  b"levLoadBaseData: Loading %s\x00" as *const u8 as
                      *const libc::c_char,
                  (*psBaseData_0).apDataFiles[i as usize]);
            if resLoad((*psBaseData_0).apDataFiles[i as usize], i,
                       DisplayBuffer, displayBufferSize as SDWORD, psGameHeap)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        i += 1
    }
    psCurrLevel = psNewLevel;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLevelName() -> *mut libc::c_char {
    return currentLevelName.as_mut_ptr();
}
// load up the data for a level
// load up the data for a level
#[no_mangle]
pub unsafe extern "C" fn levLoadData(mut pName: *mut STRING,
                                     mut pSaveName: *mut STRING,
                                     mut saveType: SDWORD) -> BOOL {
    let mut psNewLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut psBaseData_0: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut psChangeLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    let mut i: SDWORD = 0;
    let mut psCurrHeap: *mut BLOCK_HEAP = 0 as *mut BLOCK_HEAP;
    let mut bCamChangeSaveGame: BOOL = 0;
    debug(LOG_WZ, b"Loading level %s\x00" as *const u8 as *const libc::c_char,
          pName);
    // reset fog
//	fogStatus = 0;
//	pie_EnableFog(FALSE);//removed, always set by script or save game
    levelLoadType = saveType;
    // find the level dataset
    if levFindDataSet(pName, &mut psNewLevel) == 0 {
        debug(LOG_NEVER,
              b"levLoadData: dataset %s not found - trying to load as WRF\x00"
                  as *const u8 as *const libc::c_char, pName);
        return levLoadSingleWRF(pName)
    }
    /* Keep a copy of the present level name */
    strcpy(currentLevelName.as_mut_ptr(), pName);
    bCamChangeSaveGame = 0 as libc::c_int;
    if !pSaveName.is_null() && saveType == GTYPE_SAVE_START as libc::c_int {
        if !(*psNewLevel).psChange.is_null() {
            bCamChangeSaveGame = 1 as libc::c_int
        }
    }
    // select the change dataset if there is one
    psChangeLevel = 0 as *mut LEVEL_DATASET;
    if !(*psNewLevel).psChange.is_null() && !psCurrLevel.is_null() ||
           bCamChangeSaveGame != 0 {
        //store the level name
        debug(LOG_WZ,
              b"levLoadData: Found CAMCHANGE dataset\n\x00" as *const u8 as
                  *const libc::c_char);
        psChangeLevel = psNewLevel;
        psNewLevel = (*psNewLevel).psChange
    }
    // ensure the correct dataset is loaded
    if (*psNewLevel).type_0 as libc::c_int == LDS_CAMPAIGN as libc::c_int {
        debug(LOG_ERROR,
              b"levLoadData: Cannot load a campaign dataset (%s)\x00" as
                  *const u8 as *const libc::c_char, (*psNewLevel).pName);
        return 0 as libc::c_int
    } else {
        if !psCurrLevel.is_null() {
            if (*psCurrLevel).psBaseData != (*psNewLevel).psBaseData ||
                   ((*psCurrLevel).type_0 as libc::c_int) <
                       LDS_NONE as libc::c_int &&
                       (*psNewLevel).type_0 as libc::c_int >=
                           LDS_NONE as libc::c_int ||
                   (*psCurrLevel).type_0 as libc::c_int >=
                       LDS_NONE as libc::c_int &&
                       ((*psNewLevel).type_0 as libc::c_int) <
                           LDS_NONE as libc::c_int {
                // there is a dataset loaded but it isn't the correct one
                debug(LOG_WZ,
                      b"levLoadData: Incorrect base dataset loaded - levReleaseAll()\n\x00"
                          as *const u8 as *const libc::c_char);
                levReleaseAll();
                // this sets psCurrLevel to NULL
            }
        }
        // setup the correct dataset to load if necessary
        if psCurrLevel.is_null() {
            if !(*psNewLevel).psBaseData.is_null() {
                debug(LOG_NEVER,
                      b"levLoadData: Setting base dataset to load: %s\n\x00"
                          as *const u8 as *const libc::c_char,
                      (*(*psNewLevel).psBaseData).pName);
            }
            psBaseData_0 = (*psNewLevel).psBaseData
        } else {
            debug(LOG_NEVER,
                  b"levLoadData: No base dataset to load\n\x00" as *const u8
                      as *const libc::c_char);
            psBaseData_0 = 0 as *mut LEVEL_DATASET
        }
    }
    rebuildSearchPath((*psNewLevel).dataDir as searchPathMode,
                      0 as libc::c_int);
    // reset the old mission data if necessary
    if !psCurrLevel.is_null() {
        debug(LOG_NEVER,
              b"levLoadData: reseting old mission data\n\x00" as *const u8 as
                  *const libc::c_char);
        if gameReset() == 0 { return 0 as libc::c_int }
        if levReleaseMissionData() == 0 { return 0 as libc::c_int }
    }
    // need to free the current map and droids etc for a save game
    if psBaseData_0.is_null() && !pSaveName.is_null() {
        if saveGameReset() == 0 { return 0 as libc::c_int }
    }
    // basic game data is loaded in the game heap
    debug(LOG_NEVER,
          b"levLoadData: Setting game heap\n\x00" as *const u8 as
              *const libc::c_char);
    memSetBlockHeap(psGameHeap);
    // initialise if necessary
    if (*psNewLevel).type_0 as libc::c_int == LDS_COMPLETE as libc::c_int ||
           !psBaseData_0.is_null() {
        debug(LOG_NEVER,
              b"levLoadData: reset game heap\n\x00" as *const u8 as
                  *const libc::c_char);
        blkReset(psGameHeap);
        if stageOneInitialise() == 0 { return 0 as libc::c_int }
    }
    // load up a base dataset if necessary
    if !psBaseData_0.is_null() {
        debug(LOG_NEVER,
              b"levLoadData: loading base dataset %s\n\x00" as *const u8 as
                  *const libc::c_char, (*psBaseData_0).pName);
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if !(*psBaseData_0).apDataFiles[i as usize].is_null() {
                // load the data
                debug(LOG_WZ,
                      b"levLoadData: Loading %s ...\x00" as *const u8 as
                          *const libc::c_char,
                      (*psBaseData_0).apDataFiles[i as usize]);
                if resLoad((*psBaseData_0).apDataFiles[i as usize], i,
                           DisplayBuffer, displayBufferSize as SDWORD,
                           psGameHeap) == 0 {
                    return 0 as libc::c_int
                }
            }
            i += 1
        }
    }
    if (*psNewLevel).type_0 as libc::c_int == LDS_CAMCHANGE as libc::c_int {
        if campaignReset() == 0 { return 0 as libc::c_int }
    }
    if (*psNewLevel).game as libc::c_int == -(1 as libc::c_int) {
        //no .gam file to load - BETWEEN missions (for Editor games only)
        if (*psNewLevel).type_0 as libc::c_int == LDS_BETWEEN as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"levLoadData: only BETWEEN missions do not need a .gam file\x00"
                      as *const u8 as *const libc::c_char);
        };
        if (*psNewLevel).type_0 as libc::c_int == LDS_BETWEEN as libc::c_int {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"levels.c\x00" as *const u8 as *const libc::c_char,
                  793 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"levLoadData\x00")).as_ptr(),
                  b"psNewLevel->type == LDS_BETWEEN\x00" as *const u8 as
                      *const libc::c_char);
        };
        debug(LOG_NEVER,
              b"levLoadData: no .gam file for level: BETWEEN mission\n\x00" as
                  *const u8 as *const libc::c_char);
        if !pSaveName.is_null() {
            if !psBaseData_0.is_null() {
                if stageTwoInitialise() == 0 { return 0 as libc::c_int }
            }
            debug(LOG_NEVER,
                  b"levLoadData: setting map heap\n\x00" as *const u8 as
                      *const libc::c_char);
            blkReset(psMapHeap);
            memSetBlockHeap(psMapHeap);
            //set the mission type before the saveGame data is loaded
            if saveType == GTYPE_SAVE_MIDMISSION as libc::c_int {
                debug(LOG_NEVER,
                      b"levLoadData: init mission stuff\n\x00" as *const u8 as
                          *const libc::c_char);
                if startMissionSave((*psNewLevel).type_0 as SDWORD) == 0 {
                    return 0 as libc::c_int
                }
                debug(LOG_NEVER,
                      b"levLoadData: dataSetSaveFlag\n\x00" as *const u8 as
                          *const libc::c_char);
                dataSetSaveFlag();
            }
            debug(LOG_NEVER,
                  b"levLoadData: loading savegame: %s\n\x00" as *const u8 as
                      *const libc::c_char, pSaveName);
            if loadGame(pSaveName, 0 as libc::c_int, 1 as libc::c_int,
                        1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            if newMapInitialise() == 0 { return 0 as libc::c_int }
        }
        if pSaveName.is_null() || saveType == GTYPE_SAVE_START as libc::c_int
           {
            debug(LOG_NEVER,
                  b"levLoadData: start mission - no .gam\n\x00" as *const u8
                      as *const libc::c_char);
            if startMission((*psNewLevel).type_0 as UDWORD, 0 as *mut STRING)
                   == 0 {
                return 0 as libc::c_int
            }
        }
        debug(LOG_NEVER,
              b"levLoadData: setting mission heap\n\x00" as *const u8 as
                  *const libc::c_char);
        blkReset(psMissionHeap);
        memSetBlockHeap(psMissionHeap);
    }
    //we need to load up the save game data here for a camchange
    if bCamChangeSaveGame != 0 {
        debug(LOG_NEVER,
              b"levLoadData: no .gam file for level: BETWEEN mission\n\x00" as
                  *const u8 as *const libc::c_char);
        if !pSaveName.is_null() {
            if !psBaseData_0.is_null() {
                if stageTwoInitialise() == 0 { return 0 as libc::c_int }
            }
            debug(LOG_NEVER,
                  b"levLoadData: setting map heap\n\x00" as *const u8 as
                      *const libc::c_char);
            blkReset(psMapHeap);
            memSetBlockHeap(psMapHeap);
            debug(LOG_NEVER,
                  b"levLoadData: loading savegame: %s\n\x00" as *const u8 as
                      *const libc::c_char, pSaveName);
            if loadGame(pSaveName, 0 as libc::c_int, 1 as libc::c_int,
                        1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            if campaignReset() == 0 { return 0 as libc::c_int }
            //we now need to go to the next level
            //psNewLevel = psChangeLevel;
            //psChangeLevel = NULL;
            //stageTwoShutDown??
            //block_reset??
        }
    }
    // load the new data
    debug(LOG_NEVER,
          b"levLoadData: loading mission dataset: %s\n\x00" as *const u8 as
              *const libc::c_char, (*psNewLevel).pName);
    psCurrHeap = memGetBlockHeap();
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        if (*psNewLevel).game as libc::c_int == i {
            // do some more initialising if necessary
            if (*psNewLevel).type_0 as libc::c_int ==
                   LDS_COMPLETE as libc::c_int ||
                   (*psNewLevel).type_0 as libc::c_int >= 10 as libc::c_int ||
                   !psBaseData_0.is_null() && bCamChangeSaveGame == 0 {
                iV_Reset(0 as
                             libc::c_int); //unload font, to avoid crash on 8th load... ajl 15/sep/99
                if stageTwoInitialise() == 0 { return 0 as libc::c_int }
                debug(LOG_NEVER,
                      b"levLoadData: setting map heap\n\x00" as *const u8 as
                          *const libc::c_char);
                blkReset(psMapHeap);
                memSetBlockHeap(psMapHeap);
                psCurrHeap = psMapHeap
            }
            // missions with a seperate map have to use the mission heap now
            if ((*psNewLevel).type_0 as libc::c_int ==
                    LDS_MKEEP as libc::c_int ||
                    (*psNewLevel).type_0 as libc::c_int ==
                        LDS_MCLEAR as libc::c_int ||
                    (*psNewLevel).type_0 as libc::c_int ==
                        LDS_MKEEP_LIMBO as libc::c_int) && pSaveName.is_null()
               {
                debug(LOG_NEVER,
                      b"levLoadData: setting mission heap\n\x00" as *const u8
                          as *const libc::c_char);
                blkReset(psMissionHeap);
                memSetBlockHeap(psMissionHeap);
                psCurrHeap = psMissionHeap
            }
            // load a savegame if there is one - but not if already done so
            if !pSaveName.is_null() && bCamChangeSaveGame == 0 {
                // make sure the map gets loaded into the right heap
                debug(LOG_NEVER,
                      b"levLoadData: setting map heap\n\x00" as *const u8 as
                          *const libc::c_char);
                blkReset(psMapHeap);
                memSetBlockHeap(psMapHeap);
                psCurrHeap = psMapHeap;
                /*				if (saveType == GTYPE_SAVE_START)
				{
					// do not load any more data
					break;
				}*/
                if saveType == GTYPE_SAVE_MIDMISSION as libc::c_int {
                    debug(LOG_NEVER,
                          b"levLoadData: init mission stuff\n\x00" as
                              *const u8 as *const libc::c_char);
                    if startMissionSave((*psNewLevel).type_0 as SDWORD) == 0 {
                        return 0 as libc::c_int
                    }
                    debug(LOG_NEVER,
                          b"levLoadData: dataSetSaveFlag\n\x00" as *const u8
                              as *const libc::c_char);
                    dataSetSaveFlag();
                }
                debug(LOG_NEVER,
                      b"levLoadData: loading save game %s\n\x00" as *const u8
                          as *const libc::c_char, pSaveName);
                if loadGame(pSaveName, 0 as libc::c_int, 1 as libc::c_int,
                            1 as libc::c_int) == 0 {
                    return 0 as libc::c_int
                }
            }
            if pSaveName.is_null() ||
                   saveType == GTYPE_SAVE_START as libc::c_int {
                //set the mission type before the saveGame data is loaded
                // load the game
                debug(LOG_WZ,
                      b"Loading scenario file %s\x00" as *const u8 as
                          *const libc::c_char,
                      (*psNewLevel).apDataFiles[i as usize]);
                match (*psNewLevel).type_0 as libc::c_int {
                    0 | 2 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_COMPLETE / LDS_CAMSTART\x00"
                                  as *const u8 as *const libc::c_char);
                        //if (!startMission(MISSION_CAMPSTART, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_CAMSTART as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    5 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_BETWEEN\x00" as *const u8 as
                                  *const libc::c_char);
                        if startMission(LDS_BETWEEN as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    6 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_MKEEP\x00" as *const u8 as
                                  *const libc::c_char);
                        //if (!startMission(MISSION_OFFKEEP, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_MKEEP as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    3 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_CAMCHANGE\x00" as *const u8
                                  as *const libc::c_char);
                        //if (!startMission(MISSION_CAMPSTART, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_CAMCHANGE as libc::c_int as
                                            UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    4 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_EXPAND\x00" as *const u8 as
                                  *const libc::c_char);
                        //if (!startMission(MISSION_CAMPEXPAND, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_EXPAND as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    8 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_LIMBO\x00" as *const u8 as
                                  *const libc::c_char);
                        //if (!startMission(MISSION_CAMPEXPAND, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_EXPAND_LIMBO as libc::c_int as
                                            UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    7 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_MCLEAR\x00" as *const u8 as
                                  *const libc::c_char);
                        //if (!startMission(MISSION_OFFCLEAR, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_MCLEAR as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    9 => {
                        debug(LOG_WZ,
                              b"levLoadData: LDS_MKEEP_LIMBO\x00" as *const u8
                                  as *const libc::c_char);
                        debug(LOG_NEVER,
                              b"MKEEP_LIMBO\n\x00" as *const u8 as
                                  *const libc::c_char);
                        //if (!startMission(MISSION_OFFKEEP, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_MKEEP_LIMBO as libc::c_int as
                                            UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                    _ => {
                        if (*psNewLevel).type_0 as libc::c_int >=
                               10 as libc::c_int {
                        } else {
                            debug(LOG_ERROR,
                                  b"levLoadData: Unexpected mission type\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if (*psNewLevel).type_0 as libc::c_int >=
                               10 as libc::c_int {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"levels.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1036 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 12],
                                                            &[libc::c_char; 12]>(b"levLoadData\x00")).as_ptr(),
                                  b"psNewLevel->type >= MULTI_TYPE_START\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        debug(LOG_WZ,
                              b"levLoadData: default (MULTIPLAYER)\x00" as
                                  *const u8 as *const libc::c_char);
                        //if (!startMission(MISSION_CAMPSTART, psNewLevel->apDataFiles[i]))
                        if startMission(LDS_CAMSTART as libc::c_int as UDWORD,
                                        (*psNewLevel).apDataFiles[i as usize])
                               == 0 {
                            return 0 as libc::c_int
                        }
                    }
                }
            }
            // set the view position if necessary
            if !pSaveName.is_null() ||
                   (*psNewLevel).type_0 as libc::c_int !=
                       LDS_BETWEEN as libc::c_int &&
                       (*psNewLevel).type_0 as libc::c_int !=
                           LDS_EXPAND as libc::c_int &&
                       (*psNewLevel).type_0 as libc::c_int !=
                           LDS_EXPAND_LIMBO as libc::c_int {
                if newMapInitialise() == 0 { return 0 as libc::c_int }
            }
            // set the mission heap now if it isn't already being used
            if memGetBlockHeap() != psMissionHeap {
                debug(LOG_NEVER,
                      b"levLoadData: setting mission heap\n\x00" as *const u8
                          as *const libc::c_char);
                blkReset(psMissionHeap);
                memSetBlockHeap(psMissionHeap);
            }
            psCurrHeap = psMissionHeap
        } else if !(*psNewLevel).apDataFiles[i as usize].is_null() {
            // load the data
            debug(LOG_WZ,
                  b"levLoadData: Loading %s\x00" as *const u8 as
                      *const libc::c_char,
                  (*psNewLevel).apDataFiles[i as usize]);
            if resLoad((*psNewLevel).apDataFiles[i as usize],
                       i + 9 as libc::c_int, DisplayBuffer,
                       displayBufferSize as SDWORD, psCurrHeap) == 0 {
                return 0 as libc::c_int
            }
        }
        i += 1
    }
    dataClearSaveFlag();
    // set the mission heap now if it isn't already being used
    if memGetBlockHeap() != psMissionHeap {
        debug(LOG_NEVER,
              b"levLoadData: setting mission heap\n\x00" as *const u8 as
                  *const libc::c_char);
        blkReset(psMissionHeap);
        memSetBlockHeap(psMissionHeap);
        psCurrHeap = psMissionHeap
    }
    //if (pSaveName != NULL && saveType == GTYPE_SAVE_MIDMISSION)
    if !pSaveName.is_null() {
        //load MidMission Extras
        if loadMissionExtras(pSaveName, (*psNewLevel).type_0) == 0 {
            return 0 as libc::c_int
        }
    }
    if !pSaveName.is_null() &&
           saveType == GTYPE_SAVE_MIDMISSION as libc::c_int {
        //load script stuff
		// load the event system state here for a save game
        debug(LOG_NEVER,
              b"levLoadData: loading script system state\n\x00" as *const u8
                  as *const libc::c_char);
        if loadScriptState(pSaveName) == 0 { return 0 as libc::c_int }
    }
    if stageThreeInitialise() == 0 { return 0 as libc::c_int }
    //want to test with release build too
//#ifdef DEBUG
    //this enables us to to start cam2/cam3 without going via a save game and get the extra droids
    //in from the script-controlled Transporters
    if pSaveName.is_null() &&
           (*psNewLevel).type_0 as libc::c_int == LDS_CAMSTART as libc::c_int
       {
        eventFireCallbackTrigger(CALL_NO_REINFORCEMENTS_LEFT as libc::c_int as
                                     TRIGGER_TYPE);
    }
    //#endif
    //restore the level name for comparisons on next mission load up
    if psChangeLevel.is_null() {
        psCurrLevel = psNewLevel
    } else { psCurrLevel = psChangeLevel }
    return 1 as libc::c_int;
}
