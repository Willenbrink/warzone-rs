use ::libc;
extern "C" {
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
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /*
 * config.h
 * load and save favourites to the registry.
 */
    #[no_mangle]
    fn getWarzoneKeyNumeric(pName: *mut STRING, val: *mut DWORD) -> BOOL;
    #[no_mangle]
    fn openWarzoneKey() -> BOOL;
    #[no_mangle]
    fn closeWarzoneKey() -> BOOL;
    #[no_mangle]
    fn setWarzoneKeyNumeric(pName: *mut STRING, val: DWORD) -> BOOL;
    #[no_mangle]
    fn getWarzoneKeyString(pName: *mut STRING, pString: *mut STRING) -> BOOL;
    #[no_mangle]
    fn setWarzoneKeyString(pName: *mut STRING, pString: *mut STRING) -> BOOL;
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
    fn pie_SetVideoBuffer(width: UDWORD, height: UDWORD) -> BOOL;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn getShakeStatus() -> BOOL;
    #[no_mangle]
    fn getInvertMouseStatus() -> BOOL;
    #[no_mangle]
    fn getDrawShadows() -> BOOL;
    #[no_mangle]
    static mut gammaValue: libc::c_float;
    #[no_mangle]
    static mut scroll_speed_accel: UDWORD;
    #[no_mangle]
    fn setShakeStatus(val: BOOL);
    #[no_mangle]
    fn setDrawShadows(val: BOOL);
    #[no_mangle]
    fn setInvertMouseStatus(val: BOOL);
    #[no_mangle]
    fn pie_SetGammaValue(val: libc::c_float);
    #[no_mangle]
    fn war_SetFog(val: BOOL);
    #[no_mangle]
    fn war_GetFog() -> BOOL;
    #[no_mangle]
    fn war_SetSeqMode(mode: SEQ_MODE);
    #[no_mangle]
    fn war_GetSeqMode() -> SEQ_MODE;
    #[no_mangle]
    fn war_SetPlayAudioCDs(b: BOOL);
    #[no_mangle]
    fn war_GetPlayAudioCDs() -> BOOL;
    #[no_mangle]
    fn war_SetAllowSubtitles(_: BOOL);
    #[no_mangle]
    fn war_GetAllowSubtitles() -> BOOL;
    #[no_mangle]
    fn war_setFullscreen(_: BOOL);
    #[no_mangle]
    fn war_getFullscreen() -> BOOL;
    // = {0,1,2,3,4,5,6,7}
    #[no_mangle]
    fn initPlayerColours();
    #[no_mangle]
    fn setPlayerColour(player: UDWORD, col: UDWORD) -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    //full screen render
//extern BOOL seq_PlayVideo(char* pSeq, char* pAudio);
//extern BOOL seq_StartFullScreenVideo(char* sequenceFile, char* audioFile);//start videos through seqList
    //control
    //text
    //clear the sequence list
    //add a sequence to the list to be played
    /*checks to see if there are any sequences left in the list to play*/
    //set and check subtitle mode, TRUE subtitles on
    #[no_mangle]
    fn seq_SetSubtitles(bNewState: BOOL);
    #[no_mangle]
    fn seq_GetSubtitles() -> BOOL;
    #[no_mangle]
    fn setDifficultyLevel(lev: DIFFICULTY_LEVEL);
    #[no_mangle]
    fn getDifficultyLevel() -> DIFFICULTY_LEVEL;
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    #[no_mangle]
    static mut barMode: UDWORD;
    #[no_mangle]
    static mut gameSpy: GAMESPY;
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
    #[no_mangle]
    fn avSetStatus(var: BOOL);
    #[no_mangle]
    fn mixer_GetCDVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_SetCDVolume(iVol: SDWORD);
    #[no_mangle]
    fn mixer_GetWavVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_SetWavVolume(iVol: SDWORD);
    #[no_mangle]
    fn mixer_Get3dWavVolume() -> SDWORD;
    #[no_mangle]
    fn mixer_Set3dWavVolume(iVol: SDWORD);
    #[no_mangle]
    fn intReopenBuild(reopen: BOOL);
    #[no_mangle]
    fn intGetReopenBuild() -> BOOL;
    //#define RADAR_ROT	1
    #[no_mangle]
    static mut bDrawRadarTerrain: BOOL;
    //radar terrain on/off
    #[no_mangle]
    static mut bEnemyAllyRadarColor: BOOL;
    #[no_mangle]
    static mut sForceName: [libc::c_char; 256];
    #[no_mangle]
    static mut sPlayer: [UBYTE; 128];
    #[no_mangle]
    fn registry_clear();
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
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DWORD = libc::c_int;
/* !WIN32 */
pub type DPID = libc::c_int;
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
/*
 * Treap.h
 *
 * A balanced binary tree implementation
 *
 * Overhead for using the treap is -
 *		Overhead for the heap used by the treap :
 *                  24 bytes + 4 bytes per extension
 *      12 bytes for the root
 *      20 bytes per node
 */
/* Turn on and off the treap debugging */
/* Function type for the object compare
 * return -1 for less
 *         1 for more
 *         0 for equal
 */
pub type TREAP_CMP
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> SDWORD>;
/* The basic elements in the treap node.
 * These are done as macros so that the memory system
 * can use parts of the treap system.
 */
/* The key to sort the node on */
/* Treap priority */
/* The object stored in the treap */
/* The sub trees */
/* The debug info */
/* file the node was created in */
/* line the node was created at */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap_node {
    pub key: UDWORD,
    pub priority: UDWORD,
    pub pObj: *mut libc::c_void,
    pub psLeft: *mut _treap_node,
    pub psRight: *mut _treap_node,
}
pub type TREAP_NODE = _treap_node;
/* Treap data structure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _treap {
    pub cmp: TREAP_CMP,
    pub psNodes: *mut OBJ_HEAP,
    pub psRoot: *mut TREAP_NODE,
}
pub type TREAP = _treap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_res {
    pub psIDTreap: *mut TREAP,
    pub psStrings: *mut STR_BLOCK,
    pub init: UDWORD,
    pub ext: UDWORD,
    pub nextID: UDWORD,
}
pub type STR_RES = _str_res;
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
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GAMESPY {
    pub bGameSpy: BOOL,
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
pub struct GAMESTRUCT {
    pub name: [libc::c_char; 64],
    pub desc: SESSIONDESC,
}
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
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const DL_NORMAL: _difficulty_level = 1;
pub type DIFFICULTY_LEVEL = _difficulty_level;
pub type _difficulty_level = libc::c_uint;
pub const DL_KILLER: _difficulty_level = 4;
pub const DL_TOUGH: _difficulty_level = 3;
pub const DL_HARD: _difficulty_level = 2;
pub const DL_EASY: _difficulty_level = 0;
pub type SEQ_MODE = libc::c_uint;
pub const SEQ_SKIP: SEQ_MODE = 2;
pub const SEQ_SMALL: SEQ_MODE = 1;
pub const SEQ_FULL: SEQ_MODE = 0;
// root of the tree
/*
 * Text.h (was Strings.h)
 *
 * String management functions
 *
 */
//the two defines below are MUTUALLY EXCLUSIVE! don't have both defined...
//#define RESOURCE_NAMES
/* ID numbers for all the fixed strings */
pub type _fixed_str_id = libc::c_uint;
pub const STR_MAX_ID: _fixed_str_id = 442;
pub const STR_SEQ_MINIMAL: _fixed_str_id = 441;
pub const STR_SEQ_WINDOW: _fixed_str_id = 440;
pub const STR_SEQ_FULL: _fixed_str_id = 439;
pub const STR_SEQ_PLAYBACK: _fixed_str_id = 438;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_BIND_NORMAL_SPEED: _fixed_str_id = 424;
pub const STR_BIND_SLOW_DOWN: _fixed_str_id = 423;
pub const STR_BIND_SPEED_UP: _fixed_str_id = 422;
pub const STR_BIND_LEFT: _fixed_str_id = 421;
pub const STR_BIND_RIGHT: _fixed_str_id = 420;
pub const STR_BIND_DOWN: _fixed_str_id = 419;
pub const STR_BIND_UP: _fixed_str_id = 418;
pub const STR_BIND_CONSOLE: _fixed_str_id = 417;
pub const STR_BIND_SELCYBORG: _fixed_str_id = 416;
pub const STR_BIND_SELPOWER: _fixed_str_id = 415;
pub const STR_BIND_SELRESEARCH: _fixed_str_id = 414;
pub const STR_BIND_SELFACTORY: _fixed_str_id = 413;
pub const STR_BIND_CMD9: _fixed_str_id = 412;
pub const STR_BIND_CMD8: _fixed_str_id = 411;
pub const STR_BIND_CMD7: _fixed_str_id = 410;
pub const STR_BIND_CMD6: _fixed_str_id = 409;
pub const STR_BIND_CMD5: _fixed_str_id = 408;
pub const STR_BIND_CMD4: _fixed_str_id = 407;
pub const STR_BIND_CMD3: _fixed_str_id = 406;
pub const STR_BIND_CMD2: _fixed_str_id = 405;
pub const STR_BIND_CMD1: _fixed_str_id = 404;
pub const STR_GAM_COMNOTFOUND: _fixed_str_id = 403;
pub const STR_GAM_SENNOTFOUND: _fixed_str_id = 402;
pub const STR_GAM_CONNOTFOUND: _fixed_str_id = 401;
pub const STR_BIND_COMJ: _fixed_str_id = 400;
pub const STR_BIND_SENJ: _fixed_str_id = 399;
// number of string ID's
//added by alex 26-01-99
pub const STR_BIND_CONJ: _fixed_str_id = 398;
pub const STR_FE_CYAN: _fixed_str_id = 397;
pub const STR_FE_PINK: _fixed_str_id = 396;
pub const STR_FE_BLUE: _fixed_str_id = 395;
pub const STR_FE_RED: _fixed_str_id = 394;
pub const STR_FE_BLACK: _fixed_str_id = 393;
pub const STR_FE_GREY: _fixed_str_id = 392;
pub const STR_FE_ORANGE: _fixed_str_id = 391;
pub const STR_FE_GREEN: _fixed_str_id = 390;
pub const STR_FE_OFF: _fixed_str_id = 389;
pub const STR_FE_ON: _fixed_str_id = 388;
pub const STR_FE_MFLIP: _fixed_str_id = 387;
pub const STR_FE_SSHAKE: _fixed_str_id = 386;
pub const STR_SEL_NO_STRUCTURE: _fixed_str_id = 385;
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_BIND_ASIMIL: _fixed_str_id = 383;
pub const STR_BIND_AWHE: _fixed_str_id = 382;
pub const STR_BIND_AVTOL: _fixed_str_id = 381;
pub const STR_BIND_ALL: _fixed_str_id = 380;
pub const STR_BIND_ATR: _fixed_str_id = 379;
pub const STR_BIND_ASCR: _fixed_str_id = 378;
pub const STR_BIND_RECY: _fixed_str_id = 377;
pub const STR_BIND_AHOV: _fixed_str_id = 376;
pub const STR_BIND_AHTR: _fixed_str_id = 375;
pub const STR_BIND_ABDU: _fixed_str_id = 374;
pub const STR_BIND_ACU: _fixed_str_id = 373;
pub const STR_BIND_NDAM: _fixed_str_id = 372;
pub const STR_BIND_HDAM: _fixed_str_id = 371;
pub const STR_BIND_LDAM: _fixed_str_id = 370;
pub const STR_BIND_SCAT: _fixed_str_id = 369;
pub const STR_BIND_LONGR: _fixed_str_id = 368;
pub const STR_BIND_SENDT: _fixed_str_id = 367;
pub const STR_BIND_DSTOP: _fixed_str_id = 366;
pub const STR_BIND_REPA: _fixed_str_id = 365;
pub const STR_BIND_PATR: _fixed_str_id = 364;
pub const STR_BIND_PURS: _fixed_str_id = 363;
pub const STR_BIND_SHOR: _fixed_str_id = 362;
pub const STR_BIND_SPLIM: _fixed_str_id = 361;
pub const STR_BIND_DEFR: _fixed_str_id = 360;
pub const STR_BIND_RTB: _fixed_str_id = 359;
pub const STR_BIND_FAW: _fixed_str_id = 358;
pub const STR_BIND_ENGAG: _fixed_str_id = 357;
pub const STR_BIND_UNITJ: _fixed_str_id = 356;
pub const STR_BIND_CEASE: _fixed_str_id = 355;
pub const STR_BIND_CENTV: _fixed_str_id = 354;
pub const STR_BIND_OVERL: _fixed_str_id = 353;
pub const STR_BIND_REPJ: _fixed_str_id = 352;
pub const STR_BIND_RESJ: _fixed_str_id = 351;
pub const STR_BIND_ORD: _fixed_str_id = 350;
pub const STR_BIND_PB: _fixed_str_id = 349;
pub const STR_BIND_RR: _fixed_str_id = 348;
pub const STR_BIND_RP: _fixed_str_id = 347;
pub const STR_BIND_RL: _fixed_str_id = 346;
pub const STR_BIND_PF: _fixed_str_id = 345;
pub const STR_BIND_ZOUT: _fixed_str_id = 344;
pub const STR_BIND_ZIN: _fixed_str_id = 343;
pub const STR_BIND_ROUT: _fixed_str_id = 342;
pub const STR_BIND_RIN: _fixed_str_id = 341;
pub const STR_BIND_OPT: _fixed_str_id = 340;
pub const STR_BIND_TRACK: _fixed_str_id = 339;
pub const STR_BIND_NORTH: _fixed_str_id = 338;
pub const STR_BIND_AUDOFF: _fixed_str_id = 337;
pub const STR_BIND_AUDON: _fixed_str_id = 336;
pub const STR_BIND_MULOP: _fixed_str_id = 335;
pub const STR_BIND_GR9: _fixed_str_id = 334;
pub const STR_BIND_GR8: _fixed_str_id = 333;
pub const STR_BIND_GR7: _fixed_str_id = 332;
pub const STR_BIND_GR6: _fixed_str_id = 331;
pub const STR_BIND_GR5: _fixed_str_id = 330;
pub const STR_BIND_GR4: _fixed_str_id = 329;
pub const STR_BIND_GR3: _fixed_str_id = 328;
pub const STR_BIND_GR2: _fixed_str_id = 327;
pub const STR_BIND_GR1: _fixed_str_id = 326;
pub const STR_BIND_AS9: _fixed_str_id = 325;
pub const STR_BIND_AS8: _fixed_str_id = 324;
pub const STR_BIND_AS7: _fixed_str_id = 323;
pub const STR_BIND_AS6: _fixed_str_id = 322;
pub const STR_BIND_AS5: _fixed_str_id = 321;
pub const STR_BIND_AS4: _fixed_str_id = 320;
pub const STR_BIND_AS3: _fixed_str_id = 319;
pub const STR_BIND_AS2: _fixed_str_id = 318;
pub const STR_BIND_AS1: _fixed_str_id = 317;
pub const STR_BIND_PREV: _fixed_str_id = 316;
pub const STR_BIND_PAUSE: _fixed_str_id = 315;
pub const STR_BIND_SHOT: _fixed_str_id = 314;
pub const STR_BIND_BARS: _fixed_str_id = 313;
pub const STR_BIND_TOGCON: _fixed_str_id = 312;
pub const STR_BIND_TOGRAD: _fixed_str_id = 311;
pub const STR_BIND_CHOCOM: _fixed_str_id = 310;
pub const STR_BIND_CHOINT: _fixed_str_id = 309;
pub const STR_BIND_CHODES: _fixed_str_id = 308;
pub const STR_BIND_CHOBUI: _fixed_str_id = 307;
pub const STR_BIND_CHORES: _fixed_str_id = 306;
pub const STR_BIND_CHOMAN: _fixed_str_id = 305;
pub const STR_KM_KEYMAP_SIDE: _fixed_str_id = 304;
// keymap editor.
pub const STR_KM_KEYMAP: _fixed_str_id = 303;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
pub const STR_GAME_RESTART: _fixed_str_id = 301;
pub const STR_DORD_CYBORG_FACTORY: _fixed_str_id = 300;
pub const STR_GAME_LOADED: _fixed_str_id = 299;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
pub const STR_GAME_REPLAY: _fixed_str_id = 297;
pub const STR_CD_CHANGE_1OR2: _fixed_str_id = 296;
pub const STR_CD_CHANGE: _fixed_str_id = 295;
pub const STR_DL_LEVEL_ACE: _fixed_str_id = 294;
pub const STR_DL_LEVEL_SPECIAL: _fixed_str_id = 293;
pub const STR_DL_LEVEL_ELITE: _fixed_str_id = 292;
pub const STR_DL_LEVEL_CRACK: _fixed_str_id = 291;
pub const STR_DL_LEVEL_VETERAN: _fixed_str_id = 290;
pub const STR_DL_LEVEL_PROF: _fixed_str_id = 289;
pub const STR_DL_LEVEL_REGULAR: _fixed_str_id = 288;
pub const STR_DL_LEVEL_TRAINED: _fixed_str_id = 287;
pub const STR_DL_LEVEL_GREEN: _fixed_str_id = 286;
pub const STR_DL_LEVEL_ROOKIE: _fixed_str_id = 285;
pub const STR_MR_LEVEL_ACE: _fixed_str_id = 284;
pub const STR_MR_LEVEL_SPECIAL: _fixed_str_id = 283;
pub const STR_MR_LEVEL_ELITE: _fixed_str_id = 282;
pub const STR_MR_LEVEL_CRACK: _fixed_str_id = 281;
pub const STR_MR_LEVEL_VETERAN: _fixed_str_id = 280;
pub const STR_MR_LEVEL_PROF: _fixed_str_id = 279;
pub const STR_MR_LEVEL_REGULAR: _fixed_str_id = 278;
pub const STR_MR_LEVEL_TRAINED: _fixed_str_id = 277;
pub const STR_MR_LEVEL_GREEN: _fixed_str_id = 276;
pub const STR_MR_LEVEL_ROOKIE: _fixed_str_id = 275;
pub const STR_MR_BABAS_RUN_OVER: _fixed_str_id = 274;
pub const STR_MR_SHOTS_OFF_TARGET: _fixed_str_id = 273;
pub const STR_MR_SHOTS_ON_TARGET: _fixed_str_id = 272;
pub const STR_MR_GAME_TIME: _fixed_str_id = 271;
pub const STR_MR_MISSION_TIME: _fixed_str_id = 270;
pub const STR_MR_ARTEFACTS_FOUND: _fixed_str_id = 269;
pub const STR_MR_STR_NOW: _fixed_str_id = 268;
pub const STR_MR_STR_LOST: _fixed_str_id = 267;
pub const STR_MR_STR_BLOWN_UP: _fixed_str_id = 266;
pub const STR_MR_STR_BUILT: _fixed_str_id = 265;
pub const STR_MR_AV_UNIT_EL: _fixed_str_id = 264;
pub const STR_MR_UNITS_NOW: _fixed_str_id = 263;
pub const STR_MR_UNITS_LOST: _fixed_str_id = 262;
pub const STR_MR_UNITS_KILLED: _fixed_str_id = 261;
pub const STR_MR_UNITS_BUILT: _fixed_str_id = 260;
pub const STR_MR_RANKINGS: _fixed_str_id = 259;
pub const STR_MR_FORCE_INFO: _fixed_str_id = 258;
pub const STR_MR_STRUCTURE_LOSSES: _fixed_str_id = 257;
pub const STR_MR_UNIT_LOSSES: _fixed_str_id = 256;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub const STR_GP_ALLIGN: _fixed_str_id = 249;
pub const STR_GP_CENTERED: _fixed_str_id = 248;
pub const STR_GP_ASSIGNED: _fixed_str_id = 247;
pub const STR_GP_SELECTED: _fixed_str_id = 246;
pub const STR_GAM_REPNOTFOUND: _fixed_str_id = 245;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_GAM_UNITLOST: _fixed_str_id = 222;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const STR_DORD_FACTORY: _fixed_str_id = 219;
pub const STR_DORD_RECYCLE: _fixed_str_id = 218;
pub const STR_DORD_ARMRECYCLE: _fixed_str_id = 217;
pub const STR_DORD_EMBARK: _fixed_str_id = 216;
pub const STR_DORD_RETBASE: _fixed_str_id = 215;
pub const STR_DORD_RETREPAIR: _fixed_str_id = 214;
pub const STR_DORD_HOLDPOS: _fixed_str_id = 213;
pub const STR_DORD_GUARD: _fixed_str_id = 212;
pub const STR_DORD_PERSUE: _fixed_str_id = 211;
pub const STR_DORD_PATROL: _fixed_str_id = 210;
pub const STR_DORD_FIRE3: _fixed_str_id = 209;
pub const STR_DORD_FIRE2: _fixed_str_id = 208;
pub const STR_DORD_FIRE1: _fixed_str_id = 207;
pub const STR_DORD_REPAIR3: _fixed_str_id = 206;
pub const STR_DORD_REPAIR2: _fixed_str_id = 205;
pub const STR_DORD_REPAIR1: _fixed_str_id = 204;
pub const STR_DORD_RANGE3: _fixed_str_id = 203;
pub const STR_DORD_RANGE2: _fixed_str_id = 202;
pub const STR_DORD_RANGE1: _fixed_str_id = 201;
pub const STR_MUL_RESPOND: _fixed_str_id = 200;
pub const STR_MUL_JOINING: _fixed_str_id = 199;
pub const STR_MUL_LEAVE: _fixed_str_id = 198;
pub const STR_HARD: _fixed_str_id = 197;
pub const STR_NORMAL: _fixed_str_id = 196;
pub const STR_EASY: _fixed_str_id = 195;
pub const STR_FE_DIFFICULTY: _fixed_str_id = 194;
pub const STR_FE_TRANSPARENCY: _fixed_str_id = 193;
pub const STR_FE_FOG: _fixed_str_id = 192;
pub const STR_FE_EFFECTS: _fixed_str_id = 191;
pub const STR_FE_TEXTURE: _fixed_str_id = 190;
pub const STR_FE_AUDIO: _fixed_str_id = 189;
pub const STR_FE_GRAPHICS: _fixed_str_id = 188;
pub const STR_FE_GAME: _fixed_str_id = 187;
pub const STR_FE_GLIDE: _fixed_str_id = 186;
pub const STR_FE_OPENGL: _fixed_str_id = 185;
pub const STR_FE_DIRECTX: _fixed_str_id = 184;
pub const STR_FE_SOFTWARE: _fixed_str_id = 183;
pub const STR_FE_VIDEO: _fixed_str_id = 182;
pub const STR_FE_GOODFOG: _fixed_str_id = 181;
pub const STR_FE_CRAPFOG: _fixed_str_id = 180;
pub const STR_FE_CLAN: _fixed_str_id = 179;
pub const STR_FE_MUSIC: _fixed_str_id = 178;
pub const STR_FE_3D_FX: _fixed_str_id = 177;
pub const STR_FE_FX: _fixed_str_id = 176;
pub const STR_FE_GAMMA: _fixed_str_id = 175;
pub const STR_FE_SCROLL: _fixed_str_id = 174;
pub const STR_FE_MOUSE: _fixed_str_id = 173;
pub const STR_FE_SIDEOPTIONS: _fixed_str_id = 172;
pub const STR_FE_SKIRMISH: _fixed_str_id = 171;
pub const STR_FE_FORCEEDIT: _fixed_str_id = 170;
pub const STR_FE_JOIN: _fixed_str_id = 169;
pub const STR_FE_HOST: _fixed_str_id = 168;
pub const STR_FE_SIDEMULTI: _fixed_str_id = 167;
pub const STR_FE_RETURN: _fixed_str_id = 166;
pub const STR_FE_FASTPLAY: _fixed_str_id = 165;
pub const STR_FE_TUT: _fixed_str_id = 164;
pub const STR_FE_LOAD: _fixed_str_id = 163;
pub const STR_FE_NEW: _fixed_str_id = 162;
pub const STR_FE_SIDETUT: _fixed_str_id = 161;
pub const STR_FE_SIDESINGLE2: _fixed_str_id = 160;
pub const STR_FE_SIDESINGLE1: _fixed_str_id = 159;
pub const STR_FE_QUIT: _fixed_str_id = 158;
pub const STR_FE_INTRO: _fixed_str_id = 157;
pub const STR_FE_OPTIONS: _fixed_str_id = 156;
pub const STR_FE_MULTI: _fixed_str_id = 155;
pub const STR_FE_SINGLE: _fixed_str_id = 154;
// FrontEnd STrings
pub const STR_FE_SIDEMAIN: _fixed_str_id = 153;
pub const STR_GAME_RESUME: _fixed_str_id = 152;
//ingame ops.
pub const STR_GAME_QUIT: _fixed_str_id = 151;
pub const STR_COMPATIBLE: _fixed_str_id = 148;
pub const STR_MUL_ARTIF: _fixed_str_id = 147;
pub const STR_GIFT_POW: _fixed_str_id = 146;
pub const STR_GIFT_TEC: _fixed_str_id = 145;
pub const STR_GIFT_DRO: _fixed_str_id = 144;
pub const STR_GIFT_VIS: _fixed_str_id = 143;
pub const STR_ALLI_FRM: _fixed_str_id = 142;
pub const STR_ALLI_BRK: _fixed_str_id = 141;
pub const STR_ALLI_OFF: _fixed_str_id = 140;
pub const STR_ALLI_REQ: _fixed_str_id = 139;
pub const STR_ALLI_POW: _fixed_str_id = 138;
pub const STR_ALLI_DRO: _fixed_str_id = 137;
pub const STR_ALLI_TEC: _fixed_str_id = 136;
pub const STR_ALLI_VIS: _fixed_str_id = 135;
pub const STR_ALLI_STATE: _fixed_str_id = 134;
pub const STR_MUL_FOG_OFF: _fixed_str_id = 133;
pub const STR_MUL_FOG_ON: _fixed_str_id = 132;
pub const STR_MUL_TEAMPLAY: _fixed_str_id = 131;
pub const STR_MUL_SKIRMISH: _fixed_str_id = 130;
pub const STR_MUL_STRLIM: _fixed_str_id = 129;
pub const STR_MUL_COMP_N: _fixed_str_id = 128;
pub const STR_MUL_COMP_Y: _fixed_str_id = 127;
pub const STR_MUL_COMP: _fixed_str_id = 126;
pub const STR_MUL_PLAY: _fixed_str_id = 125;
pub const STR_MUL_PLAYERS: _fixed_str_id = 124;
pub const STR_LABEL_FOG: _fixed_str_id = 123;
pub const STR_LABEL_LIMIT: _fixed_str_id = 122;
pub const STR_LABEL_BASE: _fixed_str_id = 121;
pub const STR_LABEL_TEC: _fixed_str_id = 120;
pub const STR_LABEL_ALLI: _fixed_str_id = 119;
pub const STR_LABEL_TYPE: _fixed_str_id = 118;
pub const STR_GAMES_GAMES: _fixed_str_id = 117;
pub const STR_CON_MORE: _fixed_str_id = 116;
pub const STR_CON_CABLE: _fixed_str_id = 115;
pub const STR_CON_LAN: _fixed_str_id = 114;
pub const STR_CON_INTERNET: _fixed_str_id = 113;
pub const STR_CON_MODEM: _fixed_str_id = 112;
pub const STR_MUL_FRAGLIM: _fixed_str_id = 111;
pub const STR_MUL_TIMELIM: _fixed_str_id = 110;
pub const STR_MUL_NOLIM: _fixed_str_id = 109;
pub const STR_MUL_ALLIANCEY: _fixed_str_id = 108;
pub const STR_MUL_ALLIANCEN: _fixed_str_id = 107;
pub const STR_MUL_GAMEIC: _fixed_str_id = 106;
pub const STR_MUL_MAPIC: _fixed_str_id = 105;
pub const STR_MUL_FORCEIC: _fixed_str_id = 104;
pub const STR_MUL_PLAYERIC: _fixed_str_id = 103;
pub const STR_MUL_CAMPBASE: _fixed_str_id = 102;
pub const STR_MUL_CAMPDEFENCE: _fixed_str_id = 101;
pub const STR_MUL_CAMPCLEAN: _fixed_str_id = 100;
pub const STR_MUL_TECH3: _fixed_str_id = 99;
pub const STR_MUL_TECH2: _fixed_str_id = 98;
pub const STR_MUL_TECH1: _fixed_str_id = 97;
pub const STR_MUL_POWHI: _fixed_str_id = 96;
pub const STR_MUL_POWMED: _fixed_str_id = 95;
pub const STR_MUL_POWLO: _fixed_str_id = 94;
pub const STR_MUL_PING: _fixed_str_id = 93;
pub const STR_MUL_KILLS: _fixed_str_id = 92;
pub const STR_MUL_SCORE: _fixed_str_id = 91;
pub const STR_MUL_ALLIANCES: _fixed_str_id = 90;
pub const STR_MUL_STARTING: _fixed_str_id = 89;
pub const STR_MUL_CHAT: _fixed_str_id = 88;
pub const STR_MUL_SIDEINFO: _fixed_str_id = 87;
pub const STR_MUL_SIDETEMPLATES: _fixed_str_id = 86;
pub const STR_MUL_SIDEFORCE: _fixed_str_id = 85;
pub const STR_MUL_SIDEPLAYERS: _fixed_str_id = 84;
pub const STR_MUL_SIDEGAMES: _fixed_str_id = 83;
pub const STR_MUL_SIDEOPTIONS: _fixed_str_id = 82;
pub const STR_MUL_SIDECONNECTION: _fixed_str_id = 81;
pub const STR_MUL_115200: _fixed_str_id = 80;
pub const STR_MUL_56000: _fixed_str_id = 79;
pub const STR_MUL_19200: _fixed_str_id = 78;
pub const STR_MUL_14400: _fixed_str_id = 77;
pub const STR_MUL_SEARCHING: _fixed_str_id = 76;
pub const STR_MUL_DESIGN: _fixed_str_id = 75;
pub const STR_MUL_SAVE: _fixed_str_id = 74;
pub const STR_MUL_LOAD: _fixed_str_id = 73;
pub const STR_MUL_DEFAULT: _fixed_str_id = 72;
pub const STR_MUL_CLEAR: _fixed_str_id = 71;
pub const STR_MUL_HOST: _fixed_str_id = 70;
//	STR_MUL_RESLO,	
//	STR_MUL_RESMED,		
//	STR_MUL_RESHI,		
//	STR_MUL_HELPON,	
//	STR_MUL_HELPOFF,		
pub const STR_MUL_REFRESH: _fixed_str_id = 69;
pub const STR_MUL_CAMPAIGN: _fixed_str_id = 68;
pub const STR_MUL_ARENA: _fixed_str_id = 67;
pub const STR_MUL_MAXPLAY: _fixed_str_id = 66;
pub const STR_MUL_GAME: _fixed_str_id = 65;
pub const STR_MUL_PLAYER: _fixed_str_id = 64;
pub const STR_MUL_OK: _fixed_str_id = 63;
pub const STR_MUL_CANCEL: _fixed_str_id = 62;
pub const STR_MUL_COM4: _fixed_str_id = 61;
pub const STR_MUL_COM3: _fixed_str_id = 60;
pub const STR_MUL_COM2: _fixed_str_id = 59;
pub const STR_MUL_COM1: _fixed_str_id = 58;
pub const STR_MUL_IPADDR: _fixed_str_id = 57;
// multiplayer strings
pub const STR_MUL_PHONENO: _fixed_str_id = 56;
pub const STR_INT_POWER: _fixed_str_id = 55;
pub const STR_INT_POWERACCRUED: _fixed_str_id = 54;
pub const STR_INT_LOOP: _fixed_str_id = 53;
pub const STR_INT_DPOINT: _fixed_str_id = 52;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const STR_INT_RESCOMPLETED: _fixed_str_id = 49;
pub const STR_INT_MISMESSAGE: _fixed_str_id = 48;
pub const STR_INT_GENMESSAGE: _fixed_str_id = 47;
pub const STR_INT_RESMESSAGE: _fixed_str_id = 46;
pub const STR_INT_PWRUSAGE: _fixed_str_id = 45;
pub const STR_INT_BLDSPEED: _fixed_str_id = 44;
// Other interface strings
pub const STR_INT_BLDPROGRESS: _fixed_str_id = 43;
pub const STR_DES_TEMPBODY: _fixed_str_id = 42;
pub const STR_DES_TEMPPOWER: _fixed_str_id = 41;
pub const STR_DES_TURRET: _fixed_str_id = 40;
pub const STR_DES_PROPULSION: _fixed_str_id = 39;
pub const STR_DES_BODY: _fixed_str_id = 38;
pub const STR_DES_COMMAND: _fixed_str_id = 37;
pub const STR_DES_OTHER: _fixed_str_id = 36;
pub const STR_DES_WEAPONS: _fixed_str_id = 35;
pub const STR_DES_WATER: _fixed_str_id = 34;
pub const STR_DES_OFFROAD: _fixed_str_id = 33;
pub const STR_DES_ROAD: _fixed_str_id = 32;
pub const STR_DES_AIR: _fixed_str_id = 31;
pub const STR_DES_ROF: _fixed_str_id = 30;
pub const STR_DES_DAMAGE: _fixed_str_id = 29;
pub const STR_DES_RANGE: _fixed_str_id = 28;
pub const STR_DES_BUILD_POINTS: _fixed_str_id = 27;
pub const STR_DES_ECM_POWER: _fixed_str_id = 26;
pub const STR_DES_SENSOR_POWER: _fixed_str_id = 25;
pub const STR_DES_SENSOR_RANGE: _fixed_str_id = 24;
pub const STR_DES_POWERUSE: _fixed_str_id = 23;
pub const STR_DES_WEIGHT: _fixed_str_id = 22;
pub const STR_DES_POWER: _fixed_str_id = 21;
pub const STR_DES_ARMOUR_HEAT: _fixed_str_id = 20;
pub const STR_DES_ARMOUR_KIN: _fixed_str_id = 19;
pub const STR_DES_NEW: _fixed_str_id = 18;
pub const STR_DES_DEL: _fixed_str_id = 17;
pub const STR_DES_CANCEL: _fixed_str_id = 16;
pub const STR_DES_STORE: _fixed_str_id = 15;
// Design screen strings
pub const STR_DES_NEWVEH: _fixed_str_id = 14;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_MISC_QUIT: _fixed_str_id = 12;
pub const STR_MISC_SAVEGAME: _fixed_str_id = 11;
pub const STR_MISC_LOADGAME: _fixed_str_id = 10;
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const STR_RET_COMMAND: _fixed_str_id = 8;
pub const STR_RET_CLOSE: _fixed_str_id = 7;
pub const STR_RET_BUILD: _fixed_str_id = 6;
pub const STR_RET_RESEARCH: _fixed_str_id = 5;
pub const STR_RET_DESIGN: _fixed_str_id = 4;
pub const STR_RET_MANUFACTURE: _fixed_str_id = 3;
pub const STR_RET_INTELLIGENCE: _fixed_str_id = 2;
// Reticule strings
pub const STR_RET_OPTIONS: _fixed_str_id = 1;
// The default (id == 0) string
pub const STR_DEFAULT: _fixed_str_id = 0;
/*
 * config.h
 * load and save favourites to the registry.
 */
// from configfile.c
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn loadConfig(mut bResourceAvailable: BOOL) -> BOOL {
    let mut val: DWORD = 0;
    let mut sBuf: [STRING; 255] = [0; 255];
    if openWarzoneKey() == 0 { return 0 as libc::c_int }
    //  options screens.
	// //////////////////////////
    // //////////////////////////
	// subtitles
    if getWarzoneKeyNumeric(b"allowsubtitles\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        war_SetAllowSubtitles(val);
    }
    // //////////////////////////
	// voice vol
    if getWarzoneKeyNumeric(b"voicevol\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        mixer_SetWavVolume(val);
        //was val
    }
    // //////////////////////////
	// fx vol
    if getWarzoneKeyNumeric(b"fxvol\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        mixer_Set3dWavVolume(val);
        //was val
    }
    // //////////////////////////
	// cdvol
    if getWarzoneKeyNumeric(b"cdvol\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        mixer_SetCDVolume(val);
    }
    if getWarzoneKeyNumeric(b"playaudiocds\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        war_SetPlayAudioCDs(val);
    }
    // //////////////////////////
	// gamma
    if getWarzoneKeyNumeric(b"gamma\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        gammaValue =
            val as libc::c_float / 25 as libc::c_int as libc::c_float;
        if (gammaValue as libc::c_double) < 0.5f64 {
            gammaValue = 0.5f64 as libc::c_float
        }
        pie_SetGammaValue(gammaValue);
    } else {
        gammaValue =
            25 as libc::c_int as libc::c_float /
                25 as libc::c_int as libc::c_float;
        pie_SetGammaValue(gammaValue);
        if (gammaValue as libc::c_double) < 0.5f64 {
            gammaValue = 0.5f64 as libc::c_float
        }
        setWarzoneKeyNumeric(b"gamma\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, 25 as libc::c_int);
    }
    // //////////////////////////
	// scroll
    if getWarzoneKeyNumeric(b"scroll\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        scroll_speed_accel = val as UDWORD
    } else {
        scroll_speed_accel = 800 as libc::c_int as UDWORD;
        setWarzoneKeyNumeric(b"scroll\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, 800 as libc::c_int);
    }
    // //////////////////////////
	// screen shake
    if getWarzoneKeyNumeric(b"shake\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        setShakeStatus(val);
    } else {
        setShakeStatus(1 as libc::c_int);
        setWarzoneKeyNumeric(b"shake\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, 1 as libc::c_int);
    }
    // //////////////////////////
	// draw shadows
    if getWarzoneKeyNumeric(b"shadows\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        setDrawShadows(val);
    } else {
        setDrawShadows(1 as libc::c_int);
        setWarzoneKeyNumeric(b"shadows\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             1 as libc::c_int);
    }
    // //////////////////////////
	// invert mouse
    if getWarzoneKeyNumeric(b"mouseflip\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        setInvertMouseStatus(val);
    } else {
        setInvertMouseStatus(1 as libc::c_int);
        setWarzoneKeyNumeric(b"mouseflip\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             1 as libc::c_int);
    }
    // //////////////////////////
	// sequences
    if getWarzoneKeyNumeric(b"sequences\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        war_SetSeqMode(val as SEQ_MODE);
    } else { war_SetSeqMode(SEQ_FULL); }
    // //////////////////////////
	// subtitles
    if getWarzoneKeyNumeric(b"subtitles\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        seq_SetSubtitles(val);
    } else { seq_SetSubtitles(1 as libc::c_int); }
    // //////////////////////////
	// difficulty
    if getWarzoneKeyNumeric(b"difficulty\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        setDifficultyLevel(val as DIFFICULTY_LEVEL);
    } else {
        setDifficultyLevel(DL_NORMAL);
        setWarzoneKeyNumeric(b"difficulty\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             DL_NORMAL as libc::c_int);
    }
    if getWarzoneKeyNumeric(b"barmode\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 &&
           val < 3 as libc::c_int && val >= 0 as libc::c_int {
        barMode = val as UDWORD
    } else {
        barMode = 0 as libc::c_int as UDWORD;
        //		setDifficultyLevel(DL_NORMAL);
        setWarzoneKeyNumeric(b"barmode\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as libc::c_int);
    }
    // //////////////////////////
	// use vis fog
    if getWarzoneKeyNumeric(b"visfog\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        if val != 0 {
            war_SetFog(0 as libc::c_int);
            avSetStatus(1 as libc::c_int);
        } else {
            avSetStatus(0 as libc::c_int);
            war_SetFog(1 as libc::c_int);
        }
    } else {
        avSetStatus(0 as libc::c_int);
        war_SetFog(1 as libc::c_int);
        setWarzoneKeyNumeric(b"visfog\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, 0 as libc::c_int);
    }
    // //////////////////////////
	// favourite colour
    if bMultiPlayer == 0 {
        initPlayerColours(); // clear current maps.
        if getWarzoneKeyNumeric(b"colour\x00" as *const u8 as
                                    *const libc::c_char as *mut STRING,
                                &mut val) != 0 {
            setPlayerColour(0 as libc::c_int as UDWORD, val as UDWORD);
        } else {
            setPlayerColour(0 as libc::c_int as UDWORD,
                            0 as libc::c_int as UDWORD);
            setWarzoneKeyNumeric(b"colour\x00" as *const u8 as
                                     *const libc::c_char as *mut STRING,
                                 0 as libc::c_int);
        }
    }
    // reopen the build menu after placing a structure
    if getWarzoneKeyNumeric(b"reopenBuild\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        intReopenBuild(val);
    } else {
        intReopenBuild(0 as libc::c_int);
        setWarzoneKeyNumeric(b"reopenBuild\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as libc::c_int);
    }
    // the maximum route processing per frame
/*	if(getWarzoneKeyNumeric("maxRoute", &val))
	{
		fpathSetMaxRoute(val);
	}
	else
	{
		fpathSetMaxRoute(FPATH_MAX_ROUTE_INIT);
		setWarzoneKeyNumeric("maxRoute", FPATH_MAX_ROUTE_INIT);
	}*/
    // //////////////////////////
	//	getWarzoneKey("mouse", &val, 0);		// mouse
	//	multitype // alliance // power // base // limits // tech
	// keymaps
    // /////////////////////////
	//  multiplayer stuff.
	// /////////////////////////
    if bResourceAvailable != 0 {
        // game name
        if NetPlay.bLobbyLaunched == 0 && gameSpy.bGameSpy == 0 {
            if getWarzoneKeyString(b"gameName\x00" as *const u8 as
                                       *const libc::c_char as *mut STRING,
                                   &mut sBuf as *mut [STRING; 255] as
                                       *mut libc::c_char) != 0 {
                strcpy(game.name.as_mut_ptr(), sBuf.as_mut_ptr());
            } else {
                strcpy(game.name.as_mut_ptr(),
                       strresGetString(psStringRes,
                                       STR_GAME_NAME as libc::c_int as
                                           UDWORD));
                setWarzoneKeyString(b"gameName\x00" as *const u8 as
                                        *const libc::c_char as *mut STRING,
                                    game.name.as_mut_ptr());
            }
        }
        // player name
        if NetPlay.bLobbyLaunched == 0 && gameSpy.bGameSpy == 0 {
            // name will be set for us.
            if getWarzoneKeyString(b"playerName\x00" as *const u8 as
                                       *const libc::c_char as *mut STRING,
                                   &mut sBuf as *mut [STRING; 255] as
                                       *mut libc::c_char) != 0 {
                strcpy(sPlayer.as_mut_ptr() as *mut STRING,
                       sBuf.as_mut_ptr());
            } else {
                strcpy(sPlayer.as_mut_ptr() as *mut STRING,
                       strresGetString(psStringRes,
                                       STR_PLAYER_NAME as libc::c_int as
                                           UDWORD));
                setWarzoneKeyString(b"playerName\x00" as *const u8 as
                                        *const libc::c_char as *mut STRING,
                                    sPlayer.as_mut_ptr() as *mut STRING);
            }
        }
    }
    // map name
    if getWarzoneKeyString(b"mapName\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,
                           &mut sBuf as *mut [STRING; 255] as
                               *mut libc::c_char) != 0 {
        strcpy(game.map.as_mut_ptr(), sBuf.as_mut_ptr());
    } else {
        strcpy(game.map.as_mut_ptr(),
               b"Rush\x00" as *const u8 as *const libc::c_char);
        setWarzoneKeyString(b"mapName\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, game.map.as_mut_ptr());
    }
    // modem to use.
    if getWarzoneKeyNumeric(b"modemId\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        ingame.modem = val as UDWORD
    } else {
        ingame.modem = 0 as libc::c_int as UDWORD;
        setWarzoneKeyNumeric(b"modemId\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             ingame.modem as DWORD);
    }
    // power
    if getWarzoneKeyNumeric(b"power\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        game.power = val as UDWORD
    } else {
        game.power = 400 as libc::c_int as UDWORD;
        setWarzoneKeyNumeric(b"power\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, game.power as DWORD);
    }
    // fog
    if getWarzoneKeyNumeric(b"fog\x00" as *const u8 as *const libc::c_char as
                                *mut STRING, &mut val) != 0 {
        game.fog = val
    } else {
        game.fog = 1 as libc::c_int;
        setWarzoneKeyNumeric(b"fog\x00" as *const u8 as *const libc::c_char as
                                 *mut STRING, game.fog);
    }
    //type
    if getWarzoneKeyNumeric(b"type\x00" as *const u8 as *const libc::c_char as
                                *mut STRING, &mut val) != 0 {
        game.type_0 = val as UBYTE
    } else {
        game.type_0 = 12 as libc::c_int as UBYTE;
        setWarzoneKeyNumeric(b"type\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, game.type_0 as DWORD);
    }
    //base
    if getWarzoneKeyNumeric(b"base\x00" as *const u8 as *const libc::c_char as
                                *mut STRING, &mut val) != 0 {
        game.base = val as UBYTE
    } else {
        game.base = 1 as libc::c_int as UBYTE;
        setWarzoneKeyNumeric(b"base\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, game.base as DWORD);
    }
    //limit
    if getWarzoneKeyNumeric(b"limit\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        game.limit = val as UBYTE
    } else {
        game.limit = 0 as libc::c_int as UBYTE;
        setWarzoneKeyNumeric(b"limit\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING, game.limit as DWORD);
    }
    //maxplay
    if getWarzoneKeyNumeric(b"maxPlay\x00" as *const u8 as *const libc::c_char
                                as *mut STRING, &mut val) != 0 {
        game.maxPlayers = val as UBYTE
    } else {
        game.maxPlayers = 4 as libc::c_int as UBYTE;
        setWarzoneKeyNumeric(b"maxPlay\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.maxPlayers as DWORD);
    }
    //compplay
    if getWarzoneKeyNumeric(b"compPlay\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        game.bComputerPlayers = val
    } else {
        game.bComputerPlayers = 0 as libc::c_int;
        setWarzoneKeyNumeric(b"compPlay\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.bComputerPlayers);
    }
    //alliance
    if getWarzoneKeyNumeric(b"alliance\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        game.alliance = val as UBYTE
    } else {
        game.alliance = 0 as libc::c_int as UBYTE;
        setWarzoneKeyNumeric(b"alliance\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.alliance as DWORD);
    }
    // force name
    if getWarzoneKeyString(b"forceName\x00" as *const u8 as
                               *const libc::c_char as *mut STRING,
                           &mut sBuf as *mut [STRING; 255] as
                               *mut libc::c_char) != 0 {
        strcpy(sForceName.as_mut_ptr(), sBuf.as_mut_ptr());
    } else {
        strcpy(sForceName.as_mut_ptr(),
               b"Default\x00" as *const u8 as *const libc::c_char);
        setWarzoneKeyString(b"forceName\x00" as *const u8 as
                                *const libc::c_char as *mut STRING,
                            sForceName.as_mut_ptr());
    }
    // favourite phrases
    if getWarzoneKeyString(b"phrase0\x00" as *const u8 as *const libc::c_char
                               as *mut STRING,
                           ingame.phrases[0 as libc::c_int as
                                              usize].as_mut_ptr()) != 0 {
        getWarzoneKeyString(b"phrase1\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[1 as libc::c_int as
                                               usize].as_mut_ptr());
        getWarzoneKeyString(b"phrase2\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[2 as libc::c_int as
                                               usize].as_mut_ptr());
        getWarzoneKeyString(b"phrase3\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[3 as libc::c_int as
                                               usize].as_mut_ptr());
        getWarzoneKeyString(b"phrase4\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[4 as libc::c_int as
                                               usize].as_mut_ptr());
    } else {
        memset(&mut ingame.phrases as *mut [[CHAR; 255]; 5] as
                   *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<[[CHAR; 255]; 5]>() as libc::c_ulong);
        setWarzoneKeyString(b"phrase0\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[0 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase1\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[1 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase2\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[2 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase3\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[3 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase4\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[4 as libc::c_int as
                                               usize].as_mut_ptr());
    }
    // enemy/allies radar view
    if getWarzoneKeyNumeric(b"radarObjectMode\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        bEnemyAllyRadarColor = val
    } else {
        bEnemyAllyRadarColor = 0 as libc::c_int;
        setWarzoneKeyNumeric(b"radarObjectMode\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             bEnemyAllyRadarColor);
    }
    // no-terrain radar view
    if getWarzoneKeyNumeric(b"radarTerrainMode\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        bDrawRadarTerrain = val
    } else {
        bDrawRadarTerrain = 1 as libc::c_int;
        setWarzoneKeyNumeric(b"radarTerrainMode\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             bDrawRadarTerrain);
    }
    return closeWarzoneKey();
}
#[no_mangle]
pub unsafe extern "C" fn loadRenderMode() -> BOOL {
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut val: DWORD = 0;
    let mut w: libc::c_uint = 0;
    let mut h: libc::c_uint = 0;
    if openWarzoneKey() == 0 { return 0 as libc::c_int }
    if getWarzoneKeyNumeric(b"fullscreen\x00" as *const u8 as
                                *const libc::c_char as *mut STRING, &mut val)
           != 0 {
        war_setFullscreen(val);
    }
    // now load the desired res..
	// note that we only do this if we havent changed renderer..
    if getWarzoneKeyString(b"resolution\x00" as *const u8 as
                               *const libc::c_char as *mut STRING,
                           str.as_mut_ptr()) != 0 &&
           sscanf(str.as_mut_ptr(),
                  b"%ix%i\x00" as *const u8 as *const libc::c_char,
                  &mut w as *mut libc::c_uint, &mut h as *mut libc::c_uint) ==
               2 as libc::c_int {
        pie_SetVideoBuffer(w, h);
    }
    return closeWarzoneKey();
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn saveConfig() -> BOOL {
    debug(LOG_WZ,
          b"Writing prefs to registry\n\x00" as *const u8 as
              *const libc::c_char);
    if openWarzoneKey() == 0 { return 0 as libc::c_int }
    // //////////////////////////
	// voicevol, fxvol and cdvol
    setWarzoneKeyNumeric(b"voicevol\x00" as *const u8 as *const libc::c_char
                             as *mut STRING, mixer_GetWavVolume());
    setWarzoneKeyNumeric(b"fxvol\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, mixer_Get3dWavVolume());
    setWarzoneKeyNumeric(b"cdvol\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, mixer_GetCDVolume());
    setWarzoneKeyNumeric(b"playaudiocds\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,
                         war_GetPlayAudioCDs());
    // res.
    let mut buf: [libc::c_char; 32] = [0; 32];
    sprintf(buf.as_mut_ptr(),
            b"%ix%i\x00" as *const u8 as *const libc::c_char,
            pie_GetVideoBufferWidth(), pie_GetVideoBufferHeight());
    setWarzoneKeyString(b"resolution\x00" as *const u8 as *const libc::c_char
                            as *mut STRING, buf.as_mut_ptr());
    setWarzoneKeyNumeric(b"fullscreen\x00" as *const u8 as *const libc::c_char
                             as *mut STRING, war_getFullscreen());
    // dont save out the cheat mode.
    if getDifficultyLevel() as libc::c_uint ==
           DL_KILLER as libc::c_int as libc::c_uint ||
           getDifficultyLevel() as libc::c_uint ==
               DL_TOUGH as libc::c_int as libc::c_uint {
        setDifficultyLevel(DL_NORMAL); // gamma
    } // scroll
    setWarzoneKeyNumeric(b"allowSubtitles\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,
                         war_GetAllowSubtitles()); // level
    setWarzoneKeyNumeric(b"gamma\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,
                         (gammaValue * 25 as libc::c_int as libc::c_float) as
                             DWORD); //energybars
    setWarzoneKeyNumeric(b"scroll\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,
                         scroll_speed_accel as DWORD); // fogtype
    setWarzoneKeyNumeric(b"difficulty\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,
                         getDifficultyLevel() as DWORD); // screenshake
    setWarzoneKeyNumeric(b"barmode\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, barMode as DWORD); // flipmouse
    setWarzoneKeyNumeric(b"visfog\x00" as *const u8 as *const libc::c_char as
                             *mut STRING,
                         (war_GetFog() == 0) as libc::c_int); // shadows
    setWarzoneKeyNumeric(b"shake\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, getShakeStatus()); // sequences
    setWarzoneKeyNumeric(b"mouseflip\x00" as *const u8 as *const libc::c_char
                             as *mut STRING,
                         getInvertMouseStatus()); // subtitles
    setWarzoneKeyNumeric(b"shadows\x00" as *const u8 as *const libc::c_char as
                             *mut STRING, getDrawShadows()); // build menu
    setWarzoneKeyNumeric(b"sequences\x00" as *const u8 as *const libc::c_char
                             as *mut STRING, war_GetSeqMode() as DWORD);
    setWarzoneKeyNumeric(b"subtitles\x00" as *const u8 as *const libc::c_char
                             as *mut STRING, seq_GetSubtitles());
    setWarzoneKeyNumeric(b"reopenBuild\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,
                         intGetReopenBuild());
    //	setWarzoneKeyNumeric("maxRoute",(DWORD)(fpathGetMaxRoute()));			// maximum routing
    setWarzoneKeyNumeric(b"radarObjectMode\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,
                         bEnemyAllyRadarColor); // enemy/allies radar view
    setWarzoneKeyNumeric(b"radarTerrainMode\x00" as *const u8 as
                             *const libc::c_char as *mut STRING,
                         bDrawRadarTerrain);
    if bMultiPlayer == 0 {
        setWarzoneKeyNumeric(b"colour\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING,
                             getPlayerColour(0 as libc::c_int as UDWORD) as
                                 DWORD);
        // favourite colour.
    } else {
        debug(LOG_NEVER,
              b"Writing multiplay prefs to registry\n\x00" as *const u8 as
                  *const libc::c_char);
        if NetPlay.bHost != 0 && ingame.localJoiningInProgress != 0 {
            setWarzoneKeyString(b"gameName\x00" as *const u8 as
                                    *const libc::c_char as *mut STRING,
                                game.name.as_mut_ptr());
            //  last hosted game
        } //  map name
        setWarzoneKeyString(b"mapName\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            game.map.as_mut_ptr()); // power
        setWarzoneKeyNumeric(b"power\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING,
                             game.power as DWORD); // game type
        setWarzoneKeyNumeric(b"type\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING,
                             game.type_0 as DWORD); // size of base
        setWarzoneKeyNumeric(b"base\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING,
                             game.base as DWORD); // fog 'o war
        setWarzoneKeyNumeric(b"fog\x00" as *const u8 as *const libc::c_char as
                                 *mut STRING, game.fog); // limits
        setWarzoneKeyNumeric(b"limit\x00" as *const u8 as *const libc::c_char
                                 as *mut STRING,
                             game.limit as DWORD); // max no of players
        setWarzoneKeyNumeric(b"maxPlay\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.maxPlayers as DWORD); // allow pc players
        setWarzoneKeyNumeric(b"compPlay\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.bComputerPlayers); // allow alliances
        setWarzoneKeyNumeric(b"alliance\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             game.alliance as DWORD); // modem to use.
        setWarzoneKeyNumeric(b"modemId\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             ingame.modem as DWORD); // force
        setWarzoneKeyString(b"forceName\x00" as *const u8 as
                                *const libc::c_char as *mut STRING,
                            sForceName.as_mut_ptr()); // player name
        setWarzoneKeyString(b"playerName\x00" as *const u8 as
                                *const libc::c_char as *mut STRING,
                            sPlayer.as_mut_ptr() as *mut STRING); // phrases
        setWarzoneKeyString(b"phrase0\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[0 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase1\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[1 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase2\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[2 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase3\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[3 as libc::c_int as
                                               usize].as_mut_ptr());
        setWarzoneKeyString(b"phrase4\x00" as *const u8 as *const libc::c_char
                                as *mut STRING,
                            ingame.phrases[4 as libc::c_int as
                                               usize].as_mut_ptr());
    }
    return closeWarzoneKey();
}
#[no_mangle]
pub unsafe extern "C" fn closeConfig() { registry_clear(); }
