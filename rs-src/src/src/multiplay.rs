use ::libc;
extern "C" {
    pub type _formation;
    /* Write formatted output to S.  */
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
 *	ISO C99 Standard: 7.21 String handling	<string.h>
 */
    /* Get size_t and NULL from <stddef.h>.  */
    /* Tell the caller that we provide correct C++ prototypes.  */
    /* Copy N bytes of SRC to DEST.  */
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    /* Copy SRC to DEST.  */
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Append SRC onto DEST.  */
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    /* Append no more than N characters from SRC onto DEST.  */
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_uint)
     -> *mut libc::c_char;
    /* Compare S1 and S2.  */
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    /* Compare N characters of S1 and S2.  */
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_uint) -> libc::c_int;
    /* Return the length of S.  */
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
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
    /* Use misc.  */
    /* Use extended X/Open || misc. */
    /* Return a random integer between 0 and RAND_MAX inclusive.  */
    #[no_mangle]
    fn rand() -> libc::c_int;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
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
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    /*cancels the production run for the factory and returns any power that was 
accrued but not used*/
    #[no_mangle]
    fn cancelProduction(psBuilding: *mut STRUCTURE);
    // free up a feature with no visual effects
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
    // the memory heap for templates
    #[no_mangle]
    static mut psTemplateHeap: *mut OBJ_HEAP;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
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
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /* process the results of a completed research topic */
    #[no_mangle]
    fn researchResult(researchIndex: UDWORD, player_0: UBYTE, bDisplay: BOOL);
    /* sets the status of the topic to cancelled and stores the current research
   points accquired */
    #[no_mangle]
    fn cancelResearch(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn CancelAllResearch(pl: UDWORD);
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn flushConsoleMessages();
    /*subtract the power required */
    #[no_mangle]
    fn usePower(player_0: UDWORD, quantity: UDWORD) -> BOOL;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn cmdDroidUpdateKills(psKiller: *mut DROID);
    #[no_mangle]
    fn setPlayerHasLost(val: BOOL);
    #[no_mangle]
    fn testPlayerHasWon() -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    // This dos'nt compile on the PSX.
//typedef enum _titlemode tMode;	// define the type
    #[no_mangle]
    static mut titleMode: tMode;
    // shutdown the level system
    #[no_mangle]
    fn levShutDown();
    #[no_mangle]
    fn levInitialise() -> BOOL;
    #[no_mangle]
    fn selDroidDeselect(player_0: UDWORD) -> UDWORD;
    #[no_mangle]
    fn buildMapList() -> BOOL;
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn requestRadarTrack(x: SDWORD, y: SDWORD);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime: UDWORD;
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    #[no_mangle]
    static mut MultiMsgPlayerTo: SDWORD;
    #[no_mangle]
    static mut MultiMsgPlayerFrom: SDWORD;
    //Last console message
    #[no_mangle]
    static mut MultiplayMsg: [libc::c_char; 255];
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    //choose one. 
    #[no_mangle]
    fn NETsend(msg: *mut NETMSG, player_0: DPID, guarantee: BOOL) -> BOOL;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    // broadcast to everyone, possibly guaranteed
    #[no_mangle]
    fn NETrecv(msg: *mut NETMSG) -> BOOL;
    // recv a message if possible
    #[no_mangle]
    fn NETsendFile(newFile: BOOL, fileName: *mut CHAR, player_0: DPID)
     -> UCHAR;
    // send file chunk.
    #[no_mangle]
    fn NETrecvFile(pMsg: *mut NETMSG) -> UCHAR;
    // check for spectator status.
    // from net audio.
    #[no_mangle]
    fn NETprocessAudioCapture() -> BOOL;
    #[no_mangle]
    fn NETplayIncomingAudio(pMsg: *mut NETMSG);
    #[no_mangle]
    fn addTemplate(player_0: UDWORD, psNew: *mut DROID_TEMPLATE) -> BOOL;
    // syncing.
    #[no_mangle]
    fn sendCheck() -> BOOL;
    /*
 * Multijoin.h
 *
 * Alex Lee, pumpkin studios,
 * multijoin caters for all the player comings and goings of each player
 */
    #[no_mangle]
    fn sendVersionCheck() -> BOOL;
    #[no_mangle]
    fn recvVersionCheck(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn MultiPlayerLeave(dp: DPID) -> BOOL;
    /*
 * 	MultiRecv.h
 *
 * header for multiplay files that recv netmsgs.
 * this is a little lower level than multiplay.h
 * so we don't include it in other warzone stuff
 * to avoid a load of warnings
 */
    #[no_mangle]
    fn recvDroid(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDroidInfo(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDestroyDroid(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvGroupOrder(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDroidMove(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn receiveWholeDroid(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDestroyStructure(pMsg: *mut NETMSG) -> BOOL;
    //extern BOOL RecvBuild				(NETMSG *pMsg);
    #[no_mangle]
    fn recvBuildStarted(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvBuildFinished(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDemolishFinished(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvPing(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvRequestDroid(pMsg: *mut NETMSG) -> BOOL;
    //extern BOOL recvDroidWaypoint		(NETMSG *pMsg);
    #[no_mangle]
    fn recvDroidSecondary(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDroidSecondaryAll(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDroidEmbark(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvDroidDisEmbark(pMsg: *mut NETMSG) -> BOOL;
    //extern BOOL recvCommandDroid		(NETMSG *pMsg);
    #[no_mangle]
    fn recvDroidCheck(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvStructureCheck(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvPowerCheck(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvAlliance(pMsg: *mut NETMSG, allowAudio: BOOL) -> BOOL;
    //extern BOOL multiPlayerRequest		(NETMSG *pMsg);
    #[no_mangle]
    fn recvColourRequest(pMsg: *mut NETMSG) -> BOOL;
    //extern BOOL sendWholeStructure		(STRUCTURE *pS, DPID dest);
    #[no_mangle]
    fn recvOptions(pMsg: *mut NETMSG);
    #[no_mangle]
    fn recvScoreSubmission(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvHappyVtol(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvVtolRearm(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvLasSat(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvGift(pMsg: *mut NETMSG) -> BOOL;
    #[no_mangle]
    fn recvMultiPlayerRandomArtifacts(pMsg: *mut NETMSG);
    #[no_mangle]
    fn processMultiPlayerArtifacts();
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
pub type UCHAR = libc::c_uchar;
pub type DWORD = libc::c_int;
pub type LPVOID = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POINT {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
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
 * Input.h
 *
 * Prototypes for the keyboard and mouse input funcitons.
 */
/* Check the header files have been included from frame.h if they
 * are used outside of the framework library.
 */
/* The defines for all the key codes */
pub type _key_code = libc::c_uint;
pub const KEY_KPENTER: _key_code = 271;
pub const KEY_DELETE: _key_code = 127;
pub const KEY_INSERT: _key_code = 277;
pub const KEY_PAGEDOWN: _key_code = 281;
pub const KEY_DOWNARROW: _key_code = 274;
pub const KEY_END: _key_code = 279;
pub const KEY_RIGHTARROW: _key_code = 275;
pub const KEY_LEFTARROW: _key_code = 276;
pub const KEY_PAGEUP: _key_code = 280;
pub const KEY_UPARROW: _key_code = 273;
pub const KEY_HOME: _key_code = 278;
pub const KEY_RALT: _key_code = 307;
pub const KEY_KP_BACKSLASH: _key_code = 267;
pub const KEY_RCTRL: _key_code = 305;
pub const KEY_F12: _key_code = 293;
pub const KEY_F11: _key_code = 292;
pub const KEY_KP_FULLSTOP: _key_code = 266;
pub const KEY_KP_0: _key_code = 256;
pub const KEY_KP_3: _key_code = 259;
pub const KEY_KP_2: _key_code = 258;
pub const KEY_KP_1: _key_code = 257;
pub const KEY_KP_PLUS: _key_code = 270;
pub const KEY_KP_6: _key_code = 262;
pub const KEY_KP_5: _key_code = 261;
pub const KEY_KP_4: _key_code = 260;
pub const KEY_KP_MINUS: _key_code = 269;
pub const KEY_KP_9: _key_code = 265;
pub const KEY_KP_8: _key_code = 264;
pub const KEY_KP_7: _key_code = 263;
pub const KEY_SCROLLLOCK: _key_code = 302;
pub const KEY_NUMLOCK: _key_code = 300;
pub const KEY_F10: _key_code = 291;
pub const KEY_F9: _key_code = 290;
pub const KEY_F8: _key_code = 289;
pub const KEY_F7: _key_code = 288;
pub const KEY_F6: _key_code = 287;
pub const KEY_F5: _key_code = 286;
pub const KEY_F4: _key_code = 285;
pub const KEY_F3: _key_code = 284;
pub const KEY_F2: _key_code = 283;
pub const KEY_F1: _key_code = 282;
pub const KEY_CAPSLOCK: _key_code = 301;
pub const KEY_SPACE: _key_code = 32;
pub const KEY_LALT: _key_code = 308;
pub const KEY_KP_STAR: _key_code = 268;
pub const KEY_RSHIFT: _key_code = 303;
pub const KEY_FORWARDSLASH: _key_code = 47;
pub const KEY_FULLSTOP: _key_code = 46;
pub const KEY_COMMA: _key_code = 44;
pub const KEY_M: _key_code = 109;
pub const KEY_N: _key_code = 110;
pub const KEY_B: _key_code = 98;
pub const KEY_V: _key_code = 118;
pub const KEY_C: _key_code = 99;
pub const KEY_X: _key_code = 120;
pub const KEY_Z: _key_code = 122;
pub const KEY_BACKSLASH: _key_code = 92;
pub const KEY_LSHIFT: _key_code = 304;
pub const KEY_BACKQUOTE: _key_code = 96;
pub const KEY_QUOTE: _key_code = 39;
pub const KEY_SEMICOLON: _key_code = 59;
pub const KEY_L: _key_code = 108;
pub const KEY_K: _key_code = 107;
pub const KEY_J: _key_code = 106;
pub const KEY_H: _key_code = 104;
pub const KEY_G: _key_code = 103;
pub const KEY_F: _key_code = 102;
pub const KEY_D: _key_code = 100;
pub const KEY_S: _key_code = 115;
pub const KEY_A: _key_code = 97;
pub const KEY_LCTRL: _key_code = 306;
pub const KEY_RETURN: _key_code = 13;
pub const KEY_RBRACE: _key_code = 93;
pub const KEY_LBRACE: _key_code = 91;
pub const KEY_P: _key_code = 112;
pub const KEY_O: _key_code = 111;
pub const KEY_I: _key_code = 105;
pub const KEY_U: _key_code = 117;
pub const KEY_Y: _key_code = 121;
pub const KEY_T: _key_code = 116;
pub const KEY_R: _key_code = 114;
pub const KEY_E: _key_code = 101;
pub const KEY_W: _key_code = 119;
pub const KEY_Q: _key_code = 113;
pub const KEY_TAB: _key_code = 9;
pub const KEY_BACKSPACE: _key_code = 8;
pub const KEY_EQUALS: _key_code = 61;
pub const KEY_MINUS: _key_code = 45;
pub const KEY_0: _key_code = 48;
pub const KEY_9: _key_code = 57;
pub const KEY_8: _key_code = 56;
pub const KEY_7: _key_code = 55;
pub const KEY_6: _key_code = 54;
pub const KEY_5: _key_code = 53;
pub const KEY_4: _key_code = 52;
pub const KEY_3: _key_code = 51;
pub const KEY_2: _key_code = 50;
pub const KEY_1: _key_code = 49;
pub const KEY_ESC: _key_code = 27;
pub type KEY_CODE = _key_code;
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
pub type FRACT = libc::c_float;
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
pub struct iView {
    pub p: iVector,
    pub r: iVector,
}
// root of the tree
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
/* Only needed for Alex's movement update ? */
//	UDWORD	timeStarted;
//	UDWORD	arrivalTime;
//	UDWORD	pathStarted;
//	BOOL	startedMoving;
//	UDWORD	lastTime;
//	BOOL	speedChange;
/*
 * messageDef.h
 *
 * Message structure definitions
 */
// Research message
// Campaign message
// Mission Report messages
// Proximity message
pub type _view_type = libc::c_uint;
// full screen view sequence - flic.	extended format
pub const VIEW_TYPES: _view_type = 4;
// proximity view - no view really!
pub const VIEW_RPLX: _view_type = 3;
// full screen view sequence - flic
pub const VIEW_PROX: _view_type = 2;
// research view
pub const VIEW_RPL: _view_type = 1;
pub const VIEW_RES: _view_type = 0;
pub type RESEARCH = research_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
/* the 2nd IMD for base plates/turrets*/
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
//this is sizeof(FACTORY) the largest at present 11-2-99 - increased AB 22-04-99
pub type FUNCTIONALITY = [UBYTE; 40];
//this structure is used to hold the permenant stats for each type of building
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
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
/*
 * StructureDef.h
 *
 * Structure definitions for structures
 *
 */
// Used to indicate any kind of building when calling intGotoNextStructureType()
/* Defines for indexing an appropriate IMD object given a buildings purpose. */
pub type C2RustUnnamed = libc::c_uint;
//need to keep a count of how many types for IMD loading
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
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed = 20;
pub const REF_REARM_PAD: C2RustUnnamed = 19;
pub const REF_LAB: C2RustUnnamed = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed = 17;
//the demolish structure type - should only be one stat with this type
pub const REF_CYBORG_FACTORY: C2RustUnnamed = 16;
pub const REF_DEMOLISH: C2RustUnnamed = 15;
//control centre for command droids
pub const REF_BRIDGE: C2RustUnnamed = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed = 11;
pub const REF_RESEARCH: C2RustUnnamed = 10;
//corner wall - no gun
pub const REF_BLASTDOOR: C2RustUnnamed = 9;
pub const REF_WALLCORNER: C2RustUnnamed = 8;
pub const REF_WALL: C2RustUnnamed = 7;
pub const REF_DEFENSE: C2RustUnnamed = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed = 5;
pub const REF_POWER_MODULE: C2RustUnnamed = 4;
//draw as factory 2	
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
pub type STRUCTURE = _structure;
//Delivery Points NOT wayPoints
//proximity messages that are data generated
//proximity messages that are in game generated
//SAVE ONLY delivery point for factory currently assigned to commander
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
/* the subject the structure is working on*/
/* Number of upgrade modules added*/
/* The time the building started on the subject*/
/* Research Points produced per research cycle*/
/* Time taken to research the topic*/
/* The topic with the most research points 
										   that was last performed*/
/* used to keep track of power before 
										   researching a topic*/
/* The time the research facility was put on hold*/
/* The max size of body the factory 
											   can produce*/
/* The number of droids to produce OR for 
											   selectedPlayer, how many loops to perform*/
/* how many times the loop has been performed*/
//struct _propulsion_types*	propulsionType;		
	//UBYTE				propulsionType;		/* The type of propulsion the facility 
	//										   can produce*/
/* Droid Build Points Produced Per 
											   Build Cycle*/
/* used to keep track of power before building a droid*/
/* the subject the structure is working on */
/* The time the building started on the subject*/
/* Time taken to build one droid */
/* The time the factory was put on hold*/
/* Place for the new droids to assemble at */
// formation for the droids that are produced
// command droid to produce droids for (if any)
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
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
pub const STR_GAME_NAME: _fixed_str_id = 150;
pub const STR_PLAYER_NAME: _fixed_str_id = 149;
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
pub type _title_mode = libc::c_uint;
pub const GAME3: _title_mode = 18;
pub const GAME2: _title_mode = 17;
pub const KEYMAP: _title_mode = 16;
pub const LOADSAVEGAME: _title_mode = 15;
pub const QUIT: _title_mode = 14;
pub const SHOWINTRO: _title_mode = 13;
pub const STARTGAME: _title_mode = 12;
pub const MULTILIMIT: _title_mode = 11;
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
/* 
 * Frontend.h
 */
// determines which option screen to use. when in GS_TITLE_SCREEN mode.
pub type tMode = _title_mode;
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
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_0 = 222;
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
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
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
 * Interp.h
 *
 * Script interpreter definitions
 */
/* The possible value types for scripts */
// Basic types
//	VAL_FLOAT,
// events and triggers
/* Type used by the compiler for functions that do not return a value */
// user defined types should start with this id
// flag to specify a variable reference rather than simple value
/* A value consists of its type and value */
// VAL_BOOL
// VAL_INT
//		float		fval;		// VAL_FLOAT
// VAL_STRING
// VAL_OBJECT
// VAL_VOIDPTR
// maximum number of equivalent types for a type
// type equivalences
// the type that the others are equivalent to
// number of equivalent types
// the equivalent types
/* Opcodes for the script interpreter */
// Push value onto stack
// Push a pointer to a variable onto the stack
// Pop value from stack
// Push the value of a global variable onto the stack
// Pop a value from the stack into a global variable
// Push the value of a global array variable onto the stack
// Pop a value from the stack into a global array variable
// Call the 'C' function pointed to by the next value
// Call the variable access 'C' function pointed to by the next value
// Jump to a different location in the script
// Jump if the top stack value is true
// Jump if the top stack value is false
// Call a binary maths/boolean operator
// Call a unary maths/boolean operator
// End the program
// temporarily pause the current event
// The following operations are secondary data to OP_BINARYOP and OP_UNARYOP
// Maths operators
// Boolean operators
//String cancatenation
// Comparison operators
//custom (in-script) function call
//local var
//variable of object type (pointer)
/* How far the opcode is shifted up a UDWORD to allow other data to be
 * stored in the same UDWORD
 */
// maximum sizes for arrays
/* The mask for the number of array elements stored in the data part of an opcode */
/* The type of function called by an OP_CALL */
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
pub type TRIGGER_TYPE = _trigger_type;
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
pub struct GAMESPY {
    pub bGameSpy: BOOL,
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
 * Multiplay.c
 *
 * Alex Lee, Sep97, Pumpkin Studios
 *
 * Contains the day to day networking stuff, and received message handler.
 */
// for templates.
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// globals.
#[no_mangle]
pub static mut bMultiPlayer: BOOL = 0 as libc::c_int;
// true when more than 1 player.
#[no_mangle]
pub static mut sForceName: [STRING; 256] =
    [68, 101, 102, 97, 117, 108, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub static mut player2dpid: [DWORD; 8] =
    [0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int];
//stores dpids of each player. FILTHY HACK (ASSUMES 8playerS)
//UDWORD						arenaPlayersReceived=0;
#[no_mangle]
pub static mut openchannels: [BOOL; 8] =
    [1 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub static mut bDisplayMultiJoiningStatus: UBYTE = 0;
#[no_mangle]
pub static mut gameSpy: GAMESPY = GAMESPY{bGameSpy: 0,};
#[no_mangle]
pub static mut game: MULTIPLAYERGAME =
    MULTIPLAYERGAME{type_0: 0,
                    map: [0; 128],
                    version: [0; 8],
                    maxPlayers: 0,
                    name: [0; 128],
                    bComputerPlayers: 0,
                    fog: 0,
                    power: 0,
                    base: 0,
                    alliance: 0,
                    limit: 0,
                    bytesPerSec: 0,
                    packetsPerSec: 0,
                    encryptKey: 0,
                    skDiff: [0; 8],};
//info to describe game.
#[no_mangle]
pub static mut ingame: MULTIPLAYERINGAME =
    MULTIPLAYERINGAME{PingTimes: [0; 8],
                      localOptionsReceived: 0,
                      localJoiningInProgress: 0,
                      JoiningInProgress: [0; 8],
                      bHostSetup: 0,
                      startTime: 0,
                      modem: 0,
                      numStructureLimits: 0,
                      pStructureLimits: 0 as *const UBYTE as *mut UBYTE,
                      skScores: [[0; 2]; 8],
                      phrases: [[0; 255]; 5],};
#[no_mangle]
pub static mut bSendingMap: BOOL = 0 as libc::c_int;
// map broadcasting.
#[no_mangle]
pub static mut tempString: [STRING; 12] = [0; 12];
#[no_mangle]
pub static mut msgStr: [[libc::c_char; 255]; 100] = [[0; 255]; 100];
#[no_mangle]
pub static mut msgPlFrom: [SDWORD; 100] = [0; 100];
#[no_mangle]
pub static mut msgPlTo: [SDWORD; 100] = [0; 100];
#[no_mangle]
pub static mut callbackType: [SDWORD; 100] = [0; 100];
#[no_mangle]
pub static mut locx: [SDWORD; 100] = [0; 100];
#[no_mangle]
pub static mut locy: [SDWORD; 100] = [0; 100];
#[no_mangle]
pub static mut msgStackPos: SDWORD = -(1 as libc::c_int);
//send AI message
// for templates.
//list of possible research items.
// ////////////////////////////////////////////////////////////////////////////
// Local Prototypes
// ////////////////////////////////////////////////////////////////////////////
// temporarily disable multiplayer mode.
#[no_mangle]
pub unsafe extern "C" fn turnOffMultiMsg(mut bDoit: BOOL) -> BOOL {
    static mut bTemp: BOOL = 0;
    if bDoit != 0 {
        // turn off msgs.
        if bTemp == 1 as libc::c_int {
            debug(LOG_NEVER,
                  b"\nturnoffmultimsg: multiple calls to turn off msging.\n\x00"
                      as *const u8 as *const libc::c_char);
        }
        if bMultiPlayer != 0 {
            bMultiPlayer = 0 as libc::c_int;
            bTemp = 1 as libc::c_int
        }
    } else if bTemp != 0 {
        bMultiPlayer = 1 as libc::c_int;
        bTemp = 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// turn on msgs.
// ////////////////////////////////////////////////////////////////////////////
// throw a pary when you win!
#[no_mangle]
pub unsafe extern "C" fn multiplayerWinSequence(mut firstCall: BOOL) -> BOOL {
    static mut pos: iVector =
        iVector{x: 0,
                y: 0,
                z: 0,}; // pan the camera to home if not already doing so
    let mut pos2: iVector = iVector{x: 0, y: 0, z: 0,};
    static mut last: UDWORD = 0 as libc::c_int as UDWORD;
    let mut fraction: FRACT = 0.;
    let mut rotAmount: FRACT = 0.;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if firstCall != 0 {
        pos = cameraToHome(selectedPlayer, 1 as libc::c_int);
        last = 0 as libc::c_int as UDWORD;
        // stop all research
        CancelAllResearch(selectedPlayer);
        // stop all manufacture.
        psStruct = apsStructLists[selectedPlayer as usize];
        while !psStruct.is_null() {
            if StructIsFactory(psStruct) != 0 {
                if !(*((*psStruct).pFunctionality as
                           *mut FACTORY)).psSubject.is_null() {
                    //check if active
                    cancelProduction(psStruct);
                }
            }
            psStruct = (*psStruct).psNext
        }
    }
    // rotate world
    if getWarCamStatus() == 0 {
        fraction = frameTime as FRACT / 1000 as libc::c_int as libc::c_float;
        rotAmount =
            fraction *
                (360 as libc::c_int *
                     (65536 as libc::c_int / 360 as libc::c_int) /
                     2 as libc::c_int) as libc::c_float /
                12 as libc::c_int as libc::c_float;
        player.r.y += rotAmount as SDWORD
    }
    if last > gameTime { last = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(last) < 500 as libc::c_int as libc::c_uint {
        // only  if not done recently.
        return 1 as libc::c_int
    }
    last = gameTime;
    if rand() % 3 as libc::c_int == 0 as libc::c_int {
        pos2 = pos;
        pos2.x +=
            rand() % ((8 as libc::c_int) << 7 as libc::c_int) -
                ((4 as libc::c_int) << 7 as libc::c_int);
        pos2.z +=
            rand() % ((8 as libc::c_int) << 7 as libc::c_int) -
                ((4 as libc::c_int) << 7 as libc::c_int);
        if pos2.x < 0 as libc::c_int { pos2.x = 128 as libc::c_int }
        if pos2.x as libc::c_uint > mapWidth << 7 as libc::c_int {
            pos2.x = (mapWidth << 7 as libc::c_int) as int32
        }
        if pos2.z < 0 as libc::c_int { pos2.z = 128 as libc::c_int }
        if pos2.z as libc::c_uint > mapHeight << 7 as libc::c_int {
            pos2.z = (mapHeight << 7 as libc::c_int) as int32
        }
        addEffect(&mut pos2, EFFECT_FIREWORK, FIREWORK_TYPE_LAUNCHER,
                  0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
        // throw up some fire works.
    }
    // show the score..
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// MultiPlayer main game loop code.
#[no_mangle]
pub unsafe extern "C" fn multiPlayerLoop() -> BOOL {
    let mut i: UDWORD = 0; // send some checking info if possible
    let mut joinCount: UBYTE = 0; // process artifacts
    sendCheck();
    processMultiPlayerArtifacts();
    //	if( (game.type == DMATCH) && !ingame.localJoiningInProgress)
//	{
//		deathmatchCheck();
//	}
    //	if (game.type != DMATCH)
//	{
    joinCount = 0 as libc::c_int as UBYTE; // someone is still joining! say So
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if isHumanPlayer(i) != 0 && ingame.JoiningInProgress[i as usize] != 0
           {
            joinCount = joinCount.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if joinCount != 0 {
        setWidgetsStatus(0 as libc::c_int);
        bDisplayMultiJoiningStatus = joinCount;
        // deselect anything selected.
        selDroidDeselect(selectedPlayer);
        if keyPressed(KEY_ESC) != 0 {
            // check for cancel
            bDisplayMultiJoiningStatus = 0 as libc::c_int as UBYTE;
            setWidgetsStatus(1 as libc::c_int);
            setPlayerHasLost(1 as libc::c_int);
        }
    } else if bDisplayMultiJoiningStatus != 0 {
        sendVersionCheck();
        bDisplayMultiJoiningStatus = 0 as libc::c_int as UBYTE;
        setWidgetsStatus(1 as libc::c_int);
    }
    //everyone is in the game now!
    //	}
    // process network audio
    if game.bytesPerSec as libc::c_int == 2000 as libc::c_int {
        NETprocessAudioCapture();
        // manage the capture buffer
    } // get queued messages
    recvMessage();
    // if player has won then process the win effects...
    if testPlayerHasWon() != 0 { multiplayerWinSequence(0 as libc::c_int); }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// quikie functions.
// to get droids ...
#[no_mangle]
pub unsafe extern "C" fn IdToDroid(mut id: UDWORD, mut player_0: UDWORD,
                                   mut psDroid: *mut *mut DROID) -> BOOL {
    let mut i: UDWORD = 0;
    let mut d: *mut DROID = 0 as *mut DROID;
    if player_0 == 99 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            // find the droid to order form them all
            d = apsDroidLists[i as usize];
            while !d.is_null() && (*d).id != id { d = (*d).psNext }
            if !d.is_null() { *psDroid = d; return 1 as libc::c_int }
            i = i.wrapping_add(1)
        }
        return 0 as libc::c_int
    } else {
        // find the droid, given player
        d = apsDroidLists[player_0 as usize];
        while !d.is_null() && (*d).id != id { d = (*d).psNext }
        if !d.is_null() { *psDroid = d; return 1 as libc::c_int }
        return 0 as libc::c_int
    };
}
// ////////////////////////////////////////////////////////////////////////////
// find a structure
#[no_mangle]
pub unsafe extern "C" fn IdToStruct(mut id: UDWORD, mut player_0: UDWORD)
 -> *mut STRUCTURE {
    let mut psStr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut i: UDWORD = 0;
    if player_0 == 99 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            psStr = apsStructLists[i as usize];
            while !psStr.is_null() && (*psStr).id != id {
                psStr = (*psStr).psNext
            }
            if !psStr.is_null() { return psStr }
            i = i.wrapping_add(1)
        }
    } else {
        psStr = apsStructLists[player_0 as usize];
        while !psStr.is_null() && (*psStr).id != id {
            psStr = (*psStr).psNext
        }
    }
    return psStr;
}
// ////////////////////////////////////////////////////////////////////////////
// find a feature
#[no_mangle]
pub unsafe extern "C" fn IdToFeature(mut id: UDWORD, mut player_0: UDWORD)
 -> *mut FEATURE {
    let mut psF: *mut FEATURE = 0 as *mut FEATURE;
    let mut i: UDWORD = 0;
    if player_0 == 99 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            psF = apsFeatureLists[i as usize];
            while !psF.is_null() && (*psF).id != id { psF = (*psF).psNext }
            if !psF.is_null() { return psF }
            i = i.wrapping_add(1)
        }
    } else {
        psF = apsFeatureLists[player_0 as usize];
        while !psF.is_null() && (*psF).id != id { psF = (*psF).psNext }
    }
    return psF;
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn IdToTemplate(mut tempId: UDWORD,
                                      mut player_0: UDWORD)
 -> *mut DROID_TEMPLATE {
    let mut psTempl: *mut DROID_TEMPLATE =
        0 as *mut DROID_TEMPLATE; // follow templates
    let mut i: UDWORD = 0;
    if player_0 != 99 as libc::c_int as libc::c_uint {
        psTempl = apsDroidTemplates[player_0 as usize];
        while !psTempl.is_null() && (*psTempl).multiPlayerID != tempId {
            psTempl = (*psTempl).psNext
        }
    } else {
        // REALLY DANGEROUS!!! ID's are NOT assumed to be unique for TEMPLATES.
        debug(LOG_NEVER,
              b"Really Dodgy Check performed for a template\x00" as *const u8
                  as *const libc::c_char); // follow templates
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            psTempl = apsDroidTemplates[i as usize];
            while !psTempl.is_null() && (*psTempl).multiPlayerID != tempId {
                psTempl = (*psTempl).psNext
            }
            if !psTempl.is_null() { return psTempl }
            i = i.wrapping_add(1)
        }
    }
    return psTempl;
}
// the same as above, but only checks names in similarity.
#[no_mangle]
pub unsafe extern "C" fn NameToTemplate(mut sName: *mut CHAR,
                                        mut player_0: UDWORD)
 -> *mut DROID_TEMPLATE {
    let mut psTempl: *mut DROID_TEMPLATE =
        0 as *mut DROID_TEMPLATE; // follow templates
    psTempl = apsDroidTemplates[player_0 as usize];
    while !psTempl.is_null() &&
              strcmp((*psTempl).aName.as_mut_ptr(), sName) != 0 as libc::c_int
          {
        psTempl = (*psTempl).psNext
    }
    return psTempl;
}
//#define MULTI_SKIRMISHA			20
//#define MULTI_CAMPAIGNA			17
// limit options for dmatch.
// campaign subtypes
// game templates are stored in player x.
// this ping is kickin'.
// this ping is crusin'.
// this ping is crawlin'.
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
// ///////////////////////////////////////////////////////////////////////////////
//  Returns a pointer to base object, given an id and optionally a player.
#[no_mangle]
pub unsafe extern "C" fn IdToPointer(mut id: UDWORD, mut player_0: UDWORD)
 -> *mut BASE_OBJECT {
    let mut pD: *mut DROID = 0 as *mut DROID;
    let mut pS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pF: *mut FEATURE = 0 as *mut FEATURE;
    // droids.
    if IdToDroid(id, player_0, &mut pD) != 0 { return pD as *mut BASE_OBJECT }
    // structures
    pS = IdToStruct(id, player_0);
    if !pS.is_null() { return pS as *mut BASE_OBJECT }
    // features
    pF = IdToFeature(id, player_0);
    if !pF.is_null() { return pF as *mut BASE_OBJECT }
    return 0 as *mut BASE_OBJECT;
}
// ////////////////////////////////////////////////////////////////////////////
// return a players name.
#[no_mangle]
pub unsafe extern "C" fn getPlayerName(mut player_0: UDWORD) -> *mut STRING {
    let mut i: UDWORD = 0;
    //	STRING tempString[2];
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if player2dpid[player_0 as usize] == NetPlay.players[i as usize].dpid
           {
            if strcmp(NetPlay.players[i as usize].name.as_mut_ptr(),
                      b"\x00" as *const u8 as *const libc::c_char) ==
                   0 as libc::c_int {
                // make up a name for this player.
                match getPlayerColour(player_0) as libc::c_int {
                    0 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_GREEN as libc::c_int as
                                                   UDWORD));
                    }
                    1 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_ORANGE as libc::c_int as
                                                   UDWORD));
                    }
                    2 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_GREY as libc::c_int as
                                                   UDWORD));
                    }
                    3 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_BLACK as libc::c_int as
                                                   UDWORD));
                    }
                    4 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_RED as libc::c_int as
                                                   UDWORD));
                    }
                    5 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_BLUE as libc::c_int as
                                                   UDWORD));
                    }
                    6 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_PINK as libc::c_int as
                                                   UDWORD));
                    }
                    7 => {
                        strcpy(tempString.as_mut_ptr(),
                               strresGetString(psStringRes,
                                               STR_FE_CYAN as libc::c_int as
                                                   UDWORD));
                    }
                    _ => { }
                }
                //				sprintf(tempString,"%d",player);
                return tempString.as_mut_ptr()
            }
            return &mut (*NetPlay.players.as_mut_ptr().offset(i as
                                                                  isize)).name
                       as *mut [libc::c_char; 64] as *mut STRING
        }
        i = i.wrapping_add(1)
    }
    return NetPlay.players[0 as libc::c_int as usize].name.as_mut_ptr();
}
// ////////////////////////////////////////////////////////////////////////////
// to determine human/computer players and responsibilities of each..
#[no_mangle]
pub unsafe extern "C" fn isHumanPlayer(mut player_0: UDWORD) -> BOOL {
    let mut val: BOOL =
        (player2dpid[player_0 as usize] != 0 as libc::c_int) as libc::c_int;
    return val;
}
// has player responsibility for player2
// returns player responsible for 'player'
#[no_mangle]
pub unsafe extern "C" fn whosResponsible(mut player_0: UDWORD) -> UDWORD {
    let mut c: UDWORD = 0;
    let mut i: SDWORD = 0;
    c = 99 as libc::c_int as UDWORD;
    if isHumanPlayer(player_0) != 0 {
        c = player_0
    } else if player_0 == selectedPlayer {
        c = player_0
    } else {
        // crawl down array to find a responsible fellow,
        i = player_0 as SDWORD;
        while i >= 0 as libc::c_int {
            if isHumanPlayer(i as UDWORD) != 0 { c = i as UDWORD }
            i -= 1
        }
        // else crawl up to find a responsible fellow
        if c == 99 as libc::c_int as libc::c_uint {
            i = player_0 as SDWORD;
            while i < 8 as libc::c_int {
                if isHumanPlayer(i as UDWORD) != 0 { c = i as UDWORD }
                i += 1
            }
        }
    }
    if c == 99 as libc::c_int as libc::c_uint {
        debug(LOG_NEVER,
              b"failed to find a player for %d \n\x00" as *const u8 as
                  *const libc::c_char, player_0);
    }
    return c;
}
//to tell if the player is a computer or not.
// determine if human
//returns true if selected player is responsible for 'player'
#[no_mangle]
pub unsafe extern "C" fn myResponsibility(mut player_0: UDWORD) -> BOOL {
    if whosResponsible(player_0) == selectedPlayer {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// this pc has comms responsibility
//returns true if 'player' is responsible for 'playerinquestion'
#[no_mangle]
pub unsafe extern "C" fn responsibleFor(mut player_0: UDWORD,
                                        mut playerinquestion: UDWORD)
 -> BOOL {
    if whosResponsible(playerinquestion) == player_0 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// returns player responsible for 'player'
// ////////////////////////////////////////////////////////////////////////////
// probably temporary. Places the camera on the players 1st droid or struct.
#[no_mangle]
pub unsafe extern "C" fn cameraToHome(mut player_0: UDWORD, mut scroll: BOOL)
 -> iVector {
    let mut res: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psBuilding = apsStructLists[player_0 as usize];
    while !psBuilding.is_null() &&
              (*(*psBuilding).pStructureType).type_0 !=
                  REF_HQ as libc::c_int as libc::c_uint {
        psBuilding = (*psBuilding).psNext
    }
    if !psBuilding.is_null() {
        x = ((*psBuilding).x as libc::c_int >> 7 as libc::c_int) as UDWORD;
        y = ((*psBuilding).y as libc::c_int >> 7 as libc::c_int) as UDWORD
    } else if !apsDroidLists[player_0 as usize].is_null() {
        // or first droid
        x =
            ((*apsDroidLists[player_0 as usize]).x as libc::c_int >>
                 7 as libc::c_int) as UDWORD;
        y =
            ((*apsDroidLists[player_0 as usize]).y as libc::c_int >>
                 7 as libc::c_int) as UDWORD
    } else if !apsStructLists[player_0 as usize].is_null() {
        // center on first struct
        x =
            ((*apsStructLists[player_0 as usize]).x as libc::c_int >>
                 7 as libc::c_int) as UDWORD;
        y =
            ((*apsStructLists[player_0 as usize]).y as libc::c_int >>
                 7 as libc::c_int) as UDWORD
    } else {
        //or map center.
        x = mapWidth.wrapping_div(2 as libc::c_int as libc::c_uint);
        y = mapHeight.wrapping_div(2 as libc::c_int as libc::c_uint)
    }
    if scroll != 0 {
        requestRadarTrack((x << 7 as libc::c_int) as SDWORD,
                          (y << 7 as libc::c_int) as SDWORD);
    } else { setViewPos(x, y, 1 as libc::c_int); }
    res.x = (x << 7 as libc::c_int) as int32;
    res.y = map_TileHeight(x, y) as int32;
    res.z = (y << 7 as libc::c_int) as int32;
    return res;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Required by the net library. It's the system message handler..
#[no_mangle]
pub unsafe extern "C" fn DirectPlaySystemMessageHandler(mut mg: LPVOID)
 -> BOOL {
    return 1 as libc::c_int;
}
// for loop.c						
// interpret DP messages
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Recv Messages. Get a message and dispatch to relevant function.
#[no_mangle]
pub unsafe extern "C" fn recvMessage() -> BOOL {
    let mut msg: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut dp: DPID = 0;
    let mut a: UDWORD = 0;
    while NETrecv(&mut msg) == 1 as libc::c_int {
        // for all incoming messages.
        // messages only in game.
        if ingame.localJoiningInProgress == 0 {
            match msg.type_0 as libc::c_int {
                255 => { recvAudioMsg(&mut msg); }
                0 => {
                    // new droid of known type
                    recvDroid(&mut msg);
                }
                1 => {
                    //droid update info
                    recvDroidInfo(&mut msg);
                }
                2 => {
                    // droid destroy
                    recvDestroyDroid(&mut msg); // a generic destroy, complete wiht killer info.
                }
                37 => { recvDestroyExtra(&mut msg); }
                3 => {
                    // move a droid to x,y command.
                    recvDroidMove(&mut msg);
                }
                4 => {
                    // an order for more than 1 droid.
                    recvGroupOrder(&mut msg);
                }
                9 => {
                    // droid damage and position checks
                    recvDroidCheck(&mut msg);
                }
                10 => {
                    // structure damage checks.
                    recvStructureCheck(&mut msg);
                }
                11 => {
                    // Power level syncing.
                    recvPowerCheck(&mut msg);
                }
                17 => {
                    // simple text message
                    recvTextMessage(&mut msg);
                }
                47 => {
                    //multiplayer AI text message
                    recvTextMessageAI(&mut msg);
                }
                13 => {
                    //case NET_BEACONMSG:					//beacon (blip) message
			//	recvBeacon(&msg);
			//	break;
                    // a build order has been sent.
                    recvBuildStarted(&mut msg);
                }
                15 => {
                    // a building is complete
                    recvBuildFinished(&mut msg);
                }
                14 => {
                    // structure destroy
                    recvDestroyStructure(&mut msg);
                }
                28 => {
                    //			case NET_WAYPOINT:					// add waypoint to droids.
//				recvDroidWaypoint(&msg);
//				break;
                    // set a droids secondary order level.
                    recvDroidSecondary(&mut msg);
                }
                41 => {
                    // set a droids secondary order level.
                    recvDroidSecondaryAll(&mut msg); //droid has embarked on a Transporter
                }
                42 => {
                    recvDroidEmbark(&mut msg); //droid has disembarked from a Transporter
                }
                43 => { recvDroidDisEmbark(&mut msg); }
                19 => {
                    // player requires a droid that they dont have.
                    recvRequestDroid(&mut msg);
                }
                31 => {
                    //			case NET_REQUESTPLAYER:				// a new player requires information
//				multiPlayerRequest(&msg);
//				break;
                    // an alliance gift from one player to another.
                    recvGift(&mut msg);
                }
                36 => {
                    //  a score update from another player
                    recvScoreSubmission(&mut msg);
                }
                38 => {
                    //			case NET_DMATCHWIN:					// someone has won!.
//				recvdMatchWinner(&msg);
//				break;
                    recvHappyVtol(&mut msg);
                }
                39 => { recvVtolRearm(&mut msg); }
                45 => { recvLasSat(&mut msg); }
                _ => { }
            }
        }
        match msg.type_0 as libc::c_int {
            5 => {
                // new template
                recvTemplate(&mut msg);
            }
            6 => {
                // template destroy
                recvDestroyTemplate(&mut msg);
            }
            7 => {
                // feature destroy
                recvDestroyFeature(&mut msg);
            }
            8 => {
                // diagnostic ping msg.
                recvPing(&mut msg);
            }
            32 => {
                // structure demolished.
                recvDemolishFinished(&mut msg);
            }
            16 => {
                // some research has been done.
                recvResearch(&mut msg);
            }
            18 => {
                // player leaving nicely
                memcpy(&mut dp as *mut DPID as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<DPID>() as libc::c_ulong);
                MultiPlayerLeave(dp);
            }
            23 => {
                // a complete droid description has arrived.
                receiveWholeDroid(&mut msg);
            }
            26 => { recvOptions(&mut msg); }
            12 => { recvVersionCheck(&mut msg); }
            25 => {
                // remote player is now playing
                memcpy(&mut a as *mut UDWORD as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<UDWORD>() as
                           libc::c_ulong); // player is with us!
                ingame.JoiningInProgress[a as usize] = 0 as libc::c_int
            }
            33 => { recvColourRequest(&mut msg); }
            34 => { recvMultiPlayerRandomArtifacts(&mut msg); }
            30 => { recvAlliance(&mut msg, 1 as libc::c_int); }
            27 => {
                memcpy(&mut dp as *mut DPID as *mut libc::c_void,
                       &mut *msg.body.as_mut_ptr().offset(0 as libc::c_int as
                                                              isize) as
                           *mut libc::c_char as *const libc::c_void,
                       ::std::mem::size_of::<DPID>() as libc::c_ulong);
                if NetPlay.dpidPlayer == dp {
                    // we've been told to leave.
                    setPlayerHasLost(1 as libc::c_int);
                }
            }
            29 => { }
            44 => { recvResearchStatus(&mut msg); }
            _ => { }
        }
    }
    return 1 as libc::c_int;
}
/*
		// messages only during setup.
		if(ingame.localJoiningInProgress)
		{
			switch(msg.type)
			{
			case NET_PLAYERCOMPLETE:			// don't care about this!
				arenaPlayersReceived++;

				if (arenaPlayersReceived == MAX_PLAYERS)
				{
					ProcessDroidOrders();						// set droid orders....
					bMultiPlayer = TRUE;						// reinit mulitplay messaging

					useTheForce(FALSE);							// use the force.
					playerResponding();							// say hi.
					addDMatchDroid(1);							// each player adds a newdroid point.
				}
				else
				{
					// request the next player
					NetAdd(msg,0,NetPlay.dpidPlayer);				//our id
					NetAdd(msg,4,arenaPlayersReceived);				//player we require.
					msg.size = 8;
					msg.type = NET_REQUESTPLAYER;
					NETbcast(msg,TRUE);
				}
				break;

			case NET_FEATURES:								// feature set
				recvFeatures(&msg);							// **biggest message of them all!**
				break;

//			case NET_STRUCT:								// structure set.
//				receiveWholeStructure(&msg);
//				break;

			default:
				break;
			}
		}
*/
		// messages usable all the time
// process an incoming message
// ////////////////////////////////////////////////////////////////////////////
// Research Stuff. Nat games only send the result of research procedures.
#[no_mangle]
pub unsafe extern "C" fn SendResearch(mut player_0: UBYTE, mut index: UDWORD)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // player researching
    let mut i: UBYTE = 0; // reference into topic.
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player_0 as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut index as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_RESEARCH as libc::c_int as libc::c_uchar;
    // teamplay games share research....
    if game.type_0 as libc::c_int == 13 as libc::c_int ||
           game.type_0 as libc::c_int == 14 as libc::c_int {
        pPlayerRes =
            asPlayerResList[player_0 as
                                usize]; // do the research for that player
        pPlayerRes = pPlayerRes.offset(index as isize);
        i = 0 as libc::c_int as UBYTE;
        while (i as libc::c_int) < 8 as libc::c_int {
            if alliances[i as usize][player_0 as usize] as libc::c_int ==
                   3 as libc::c_int {
                pPlayerRes = asPlayerResList[i as usize];
                pPlayerRes = pPlayerRes.offset(index as isize);
                if (*pPlayerRes).ResearchStatus as libc::c_int &
                       0x4 as libc::c_int == 0 as libc::c_int {
                    (*pPlayerRes).ResearchStatus =
                        ((*pPlayerRes).ResearchStatus as libc::c_int &
                             !(0x1 as libc::c_int | 0x2 as libc::c_int |
                                   0x4 as libc::c_int) | 0x4 as libc::c_int)
                            as UBYTE;
                    researchResult(index, i, 0 as libc::c_int);
                }
            }
            i = i.wrapping_add(1)
        }
    }
    return NETbcast(&mut m, 0 as libc::c_int);
}
// send/recv Research issues
// recv a research topic that is now complete.
#[no_mangle]
pub unsafe extern "C" fn recvResearch(mut m: *mut NETMSG) -> BOOL {
    let mut player_0: UBYTE = 0; // get the index
    let mut i: UBYTE = 0;
    let mut index: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    player_0 = (*m).body[0 as libc::c_int as usize] as UBYTE;
    memcpy(&mut index as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pPlayerRes = asPlayerResList[player_0 as usize];
    pPlayerRes = pPlayerRes.offset(index as isize);
    if (*pPlayerRes).ResearchStatus as libc::c_int & 0x4 as libc::c_int ==
           0 as libc::c_int {
        (*pPlayerRes).ResearchStatus =
            ((*pPlayerRes).ResearchStatus as libc::c_int &
                 !(0x1 as libc::c_int | 0x2 as libc::c_int |
                       0x4 as libc::c_int) | 0x4 as libc::c_int) as UBYTE;
        researchResult(index, player_0, 0 as libc::c_int);
        //take off the power if available.
        pResearch = asResearch.offset(index as isize);
        usePower(player_0 as UDWORD, (*pResearch).researchPower);
    }
    // teamplay games share research....
    if game.type_0 as libc::c_int == 13 as libc::c_int ||
           game.type_0 as libc::c_int == 14 as libc::c_int {
        i = 0 as libc::c_int as UBYTE; // do the research for that player
        while (i as libc::c_int) < 8 as libc::c_int {
            if alliances[i as usize][player_0 as usize] as libc::c_int ==
                   3 as libc::c_int {
                pPlayerRes = asPlayerResList[i as usize];
                pPlayerRes = pPlayerRes.offset(index as isize);
                if (*pPlayerRes).ResearchStatus as libc::c_int &
                       0x4 as libc::c_int == 0 as libc::c_int {
                    (*pPlayerRes).ResearchStatus =
                        ((*pPlayerRes).ResearchStatus as libc::c_int &
                             !(0x1 as libc::c_int | 0x2 as libc::c_int |
                                   0x4 as libc::c_int) | 0x4 as libc::c_int)
                            as UBYTE;
                    researchResult(index, i, 0 as libc::c_int);
                }
            }
            i = i.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
}
//score check only(frontend)
// allow game to request pings.
// multijoin
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// New research stuff, so you can see what others are up to!
// inform others, I'm researching this.
#[no_mangle]
pub unsafe extern "C" fn sendReseachStatus(mut psBuilding: *mut STRUCTURE,
                                           mut index: UDWORD,
                                           mut player_0: UBYTE,
                                           mut bStart: BOOL) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // player researching
    let mut nil: UDWORD = 0 as libc::c_int as UDWORD; // start stop..
    let mut start: UWORD = bStart as UBYTE as UWORD;
    if myResponsibility(player_0 as UDWORD) == 0 ||
           gameTime < 5 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut player_0 as *mut UBYTE as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut start as *mut UWORD as *const libc::c_void,
           ::std::mem::size_of::<UWORD>() as libc::c_ulong);
    if !psBuilding.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psBuilding).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        // res lab.
    } else {
        memcpy(&mut *m.body.as_mut_ptr().offset(2 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut nil as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        // res lab.
    } // topic.
    memcpy(&mut *m.body.as_mut_ptr().offset(6 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut index as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as
               libc::c_ulong); // player researching
    m.size = 10 as libc::c_int as libc::c_ushort; // start stop..
    m.type_0 = NET_RESEARCHSTATUS as libc::c_int as libc::c_uchar; // res lab.
    return NETbcast(&mut m, 0 as libc::c_int); // topic.
}
#[no_mangle]
pub unsafe extern "C" fn recvResearchStatus(mut pMsg: *mut NETMSG) -> BOOL {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    let mut psResFacilty: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    let mut pResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut player_0: UBYTE = 0;
    let mut bStart: UBYTE = 0;
    let mut index: UDWORD = 0;
    let mut buildingId: UDWORD = 0;
    memcpy(&mut player_0 as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut bStart as *mut UBYTE as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(1 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UBYTE>() as libc::c_ulong);
    memcpy(&mut buildingId as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(2 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    memcpy(&mut index as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(6 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    pPlayerRes = asPlayerResList[player_0 as usize].offset(index as isize);
    // psBuilding may be null if finishing.
    if bStart != 0 {
        //starting research
        psBuilding = IdToStruct(buildingId, player_0 as UDWORD);
        if !psBuilding.is_null() {
            // set that facility to research
            psResFacilty =
                (*psBuilding).pFunctionality as
                    *mut RESEARCH_FACILITY; //set the subject up
            if !(*psResFacilty).psSubject.is_null() {
                cancelResearch(psBuilding);
            }
            pResearch = asResearch.offset(index as isize);
            (*psResFacilty).psSubject = pResearch as *mut BASE_STATS;
            if (*pPlayerRes).ResearchStatus as libc::c_int &
                   0x2 as libc::c_int != 0 {
                (*psResFacilty).powerAccrued = (*pResearch).researchPower
                //set up as if all power available for cancelled topics
            } else {
                (*psResFacilty).powerAccrued = 0 as libc::c_int as UDWORD
            }
            (*pPlayerRes).ResearchStatus =
                ((*pPlayerRes).ResearchStatus as libc::c_int &
                     !(0x1 as libc::c_int | 0x2 as libc::c_int |
                           0x4 as libc::c_int) | 0x1 as libc::c_int) as UBYTE;
            (*psResFacilty).timeStarted = 0 as libc::c_int as UDWORD;
            (*psResFacilty).timeStartHold = 0 as libc::c_int as UDWORD;
            (*psResFacilty).timeToResearch =
                ((*pResearch).researchPoints as
                     libc::c_uint).wrapping_div((*psResFacilty).researchPoints);
            if (*psResFacilty).timeToResearch ==
                   0 as libc::c_int as libc::c_uint {
                (*psResFacilty).timeToResearch = 1 as libc::c_int as UDWORD
            }
        }
    } else {
        // finished/cancelled research
        if buildingId == 0 as libc::c_int as libc::c_uint {
            // find the centre doing this research.(set building
            // go through the structs to find the centre that's doing this topic)
            psBuilding = apsStructLists[player_0 as usize];
            while !psBuilding.is_null() {
                if (*(*psBuilding).pStructureType).type_0 ==
                       REF_RESEARCH as libc::c_int as libc::c_uint &&
                       (*psBuilding).status as libc::c_int ==
                           SS_BUILT as libc::c_int &&
                       !(*((*psBuilding).pFunctionality as
                               *mut RESEARCH_FACILITY)).psSubject.is_null() &&
                       (*(*((*psBuilding).pFunctionality as
                                *mut RESEARCH_FACILITY)).psSubject).ref_0.wrapping_sub(0xb0000
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)
                           == index {
                    break ;
                }
                psBuilding = (*psBuilding).psNext
            }
        } else { psBuilding = IdToStruct(buildingId, player_0 as UDWORD) }
        if (*pPlayerRes).ResearchStatus as libc::c_int & 0x4 as libc::c_int !=
               0 {
            return 1 as libc::c_int
        }
        // stop that facility doing any research.
        if !psBuilding.is_null() { cancelResearch(psBuilding); }
    }
    return 1 as libc::c_int;
}
// send a destruct feature message.      
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Text Messaging between players. proceed string with players to send to.
// eg "123 hi there" sends "hi there" to players 1,2 and 3.
#[no_mangle]
pub unsafe extern "C" fn sendTextMessage(mut pStr: *mut libc::c_char,
                                         mut all: BOOL) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut normal: BOOL = 1 as libc::c_int;
    let mut sendto: [BOOL; 8] = [0; 8];
    let mut i: UDWORD = 0;
    let mut display: [CHAR; 255] = [0; 255];
    let mut bEncrypting: BOOL = 0;
    if ingame.localOptionsReceived == 0 { return 1 as libc::c_int }
    bEncrypting = NetPlay.bEncryptAllPackets;
    NetPlay.bEncryptAllPackets = 0 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        sendto[i as usize] = 0 as libc::c_int;
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as UDWORD;
    while *pStr.offset(i as isize) as libc::c_int >= '0' as i32 &&
              *pStr.offset(i as isize) as libc::c_int <= '8' as i32 {
        // for each numeric char encountered..
        sendto[(*pStr.offset(i as isize) as libc::c_int - '0' as i32) as
                   usize] = 1 as libc::c_int; // copy message in.
        normal = 0 as libc::c_int; // package the message up and send it.
        i = i.wrapping_add(1)
    }
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut NetPlay.dpidPlayer as *mut DPID as *const libc::c_void,
           ::std::mem::size_of::<DPID>() as libc::c_ulong);
    memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut *pStr.offset(i as isize) as *mut libc::c_char as
               *const libc::c_void,
           strlen(&mut *pStr.offset(i as
                                        isize)).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint));
    m.size =
        strlen(&mut *pStr.offset(i as
                                     isize)).wrapping_add(5 as libc::c_int as
                                                              libc::c_uint) as
            UWORD;
    m.type_0 = NET_TEXTMSG as libc::c_int as libc::c_uchar;
    if all != 0 {
        NETbcast(&mut m, 0 as libc::c_int);
    } else if normal != 0 {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            if openchannels[i as usize] != 0 {
                if isHumanPlayer(i) != 0 {
                    NETsend(&mut m, player2dpid[i as usize],
                            0 as libc::c_int);
                } else {
                    //also send to AIs now (non-humans), needed for AI
                    sendAIMessage(&mut *m.body.as_mut_ptr().offset(4 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                                  selectedPlayer as SDWORD, i as SDWORD);
                }
            }
            i = i.wrapping_add(1)
        }
    } else {
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            if sendto[i as usize] != 0 {
                if isHumanPlayer(i) != 0 {
                    NETsend(&mut m, player2dpid[i as usize],
                            0 as libc::c_int);
                } else {
                    //also send to AIs now (non-humans), needed for AI
                    sendAIMessage(&mut *m.body.as_mut_ptr().offset(4 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                                  selectedPlayer as SDWORD,
                                  i as SDWORD); //findplayer
                }
            } // name
            i = i.wrapping_add(1)
        }
    } // seperator
    NetPlay.bEncryptAllPackets = bEncrypting; // add message
    i = 0 as libc::c_int as UDWORD; // display
    while NetPlay.players[i as usize].dpid != NetPlay.dpidPlayer {
        i = i.wrapping_add(1)
    }
    strcpy(display.as_mut_ptr(),
           NetPlay.players[i as usize].name.as_mut_ptr());
    strcat(display.as_mut_ptr(),
           b" : \x00" as *const u8 as *const libc::c_char);
    strcat(display.as_mut_ptr(), pStr);
    addConsoleMessage(display.as_mut_ptr(), DEFAULT_JUSTIFY);
    return 1 as libc::c_int;
}
// send a text message
// send/recv a text message
//AI multiplayer message, send from a certain player index to another player index
#[no_mangle]
pub unsafe extern "C" fn sendAIMessage(mut pStr: *mut libc::c_char,
                                       mut player_0: SDWORD, mut to: SDWORD)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut sendPlayer: SDWORD = 0;
    let mut bEncrypting: BOOL = 0;
    //check if this is one of the local players, don't need net send then
    if to as libc::c_uint == selectedPlayer ||
           myResponsibility(to as UDWORD) != 0 {
        //(the only) human on this machine or AI on this machine
        //Just show him the message
        displayAIMessage(pStr, player_0, to);
        if msgStackPush(CALL_AI_MSG as libc::c_int, player_0, to, pStr,
                        -(1 as libc::c_int), -(1 as libc::c_int)) == 0 {
            debug(LOG_ERROR,
                  b"sendAIMessage() - msgStackPush - stack failed\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
    } else {
        //Received a console message from a player callback
		//store and call later
		//-------------------------------------------------
        //not a local player (use multiplayer mode)
        if ingame.localOptionsReceived == 0 {
            return 1 as libc::c_int
        } //save the actual sender
        bEncrypting = NetPlay.bEncryptAllPackets;
        NetPlay.bEncryptAllPackets = 0 as libc::c_int;
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut player_0 as *mut SDWORD as *const libc::c_void,
               ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
        //save the actual player that is to get this msg on the source machine (source can host many AIs)
        memcpy(&mut *m.body.as_mut_ptr().offset(4 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut to as *mut SDWORD as *const libc::c_void,
               ::std::mem::size_of::<SDWORD>() as
                   libc::c_ulong); //save the actual receiver (might not be the same as the one we are actually sending to, in case of AIs)
        memcpy(&mut *m.body.as_mut_ptr().offset(8 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut *pStr.offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *const libc::c_void,
               strlen(&mut *pStr.offset(0 as libc::c_int as
                                            isize)).wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)); // copy message in.
        m.size =
            strlen(&mut *pStr.offset(0 as libc::c_int as
                                         isize)).wrapping_add(9 as libc::c_int
                                                                  as
                                                                  libc::c_uint)
                as UWORD; // package the message up and send it.
        m.type_0 = NET_AITEXTMSG as libc::c_int as libc::c_uchar; //new type
        //find machine that is hosting this human or AI
        sendPlayer = whosResponsible(to as UDWORD) as SDWORD;
        if sendPlayer >= 8 as libc::c_int {
            debug(LOG_ERROR,
                  b"sendAIMessage() - sendPlayer >= MAX_PLAYERS\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
        if isHumanPlayer(sendPlayer as UDWORD) == 0 {
            //NETsend can't send to non-humans
            debug(LOG_ERROR,
                  b"sendAIMessage() - player is not human.\x00" as *const u8
                      as
                      *const libc::c_char); //send to the player who is hosting 'to' player (might be himself if human and not AI)
            return 0 as libc::c_int
        }
        NETsend(&mut m, player2dpid[sendPlayer as usize], 0 as libc::c_int);
        NetPlay.bEncryptAllPackets = bEncrypting
    }
    return 1 as libc::c_int;
}
//send AI message
#[no_mangle]
pub unsafe extern "C" fn displayAIMessage(mut pStr: *mut STRING,
                                          mut from: SDWORD, mut to: SDWORD) {
    let mut tmp: [STRING; 255] = [0; 255];
    if isHumanPlayer(to as UDWORD) != 0 {
        //display text only if receiver is the (human) host machine itself
        //addConsoleMessage(pStr,DEFAULT_JUSTIFY);
		//console("%d: %s", from, pStr);
        sprintf(tmp.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                from); // seperator
        strcat(tmp.as_mut_ptr(),
               b" : \x00" as *const u8 as *const libc::c_char); // add message
        strcat(tmp.as_mut_ptr(), pStr);
        addConsoleMessage(tmp.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
//AI multiplayer message
// Write a message to the console.
#[no_mangle]
pub unsafe extern "C" fn recvTextMessage(mut pMsg: *mut NETMSG) -> BOOL {
    let mut dpid: DPID = 0; //console callback - player who sent the message
    let mut i: UDWORD = 0; //findplayer
    let mut msg: [STRING; 255] = [0; 255];
    let mut player_0: UDWORD = 0;
    let mut j: UDWORD = 0;
    memcpy(&mut dpid as *mut DPID as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<DPID>() as libc::c_ulong);
    i = 0 as libc::c_int as UDWORD;
    while NetPlay.players[i as usize].dpid != dpid { i = i.wrapping_add(1) }
    //console callback - find real number of the player
    j = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if dpid == player2dpid[j as usize] {
            player_0 = j;
            break ;
        } else { j = j.wrapping_add(1) }
    }
    //sprintf(msg, "%d", i);
    strcpy(msg.as_mut_ptr(),
           NetPlay.players[i as usize].name.as_mut_ptr()); // seperator
    strcat(msg.as_mut_ptr(), b" : \x00" as *const u8 as *const libc::c_char);
    //strcat(msg, &(pMsg->body[4]));					// add message
    strncat(msg.as_mut_ptr(),
            &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize),
            255 as libc::c_int as libc::c_uint); // add message
    addConsoleMessage(&mut msg as *mut [STRING; 255] as *mut libc::c_char,
                      DEFAULT_JUSTIFY);
    //multiplayer message callback
	//Received a console message from a player, save
	//----------------------------------------------
    MultiMsgPlayerFrom = player_0 as SDWORD;
    MultiMsgPlayerTo = selectedPlayer as SDWORD;
    strcpy(MultiplayMsg.as_mut_ptr(),
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize));
    eventFireCallbackTrigger(CALL_AI_MSG as libc::c_int as TRIGGER_TYPE);
    // make some noise!
    if titleMode as libc::c_uint == MULTIOPTION as libc::c_int as libc::c_uint
           ||
           titleMode as libc::c_uint ==
               MULTILIMIT as libc::c_int as libc::c_uint {
        audio_PlayTrack(FE_AUDIO_MESSAGEEND as libc::c_int);
    } else if ingame.localJoiningInProgress == 0 {
        audio_PlayTrack(ID_SOUND_MESSAGEEND as libc::c_int);
    }
    return 1 as libc::c_int;
}
//AI multiplayer message - received message from AI (from scripts)
#[no_mangle]
pub unsafe extern "C" fn recvTextMessageAI(mut pMsg: *mut NETMSG) -> BOOL {
    let mut sender: SDWORD = 0; // name
    let mut receiver: SDWORD = 0;
    let mut msg: [STRING; 255] = [0; 255];
    memcpy(&mut sender as *mut SDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    memcpy(&mut receiver as *mut SDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(4 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<SDWORD>() as libc::c_ulong);
    strcpy(msg.as_mut_ptr(),
           &mut *(*pMsg).body.as_mut_ptr().offset(8 as libc::c_int as isize));
    strcat(msg.as_mut_ptr(),
           NetPlay.players[sender as usize].name.as_mut_ptr());
    //Display the message and make the script callback
    displayAIMessage(msg.as_mut_ptr(), sender, receiver);
    //Received a console message from a player callback
	//store and call later
	//-------------------------------------------------
    if msgStackPush(CALL_AI_MSG as libc::c_int, sender, receiver,
                    msg.as_mut_ptr(), -(1 as libc::c_int),
                    -(1 as libc::c_int)) == 0 {
        debug(LOG_ERROR,
              b"recvTextMessageAI() - msgStackPush - stack failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Templates
// send a newly created template to other players
#[no_mangle]
pub unsafe extern "C" fn sendTemplate(mut pTempl: *mut DROID_TEMPLATE)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; //player to attach template to
    if pTempl.is_null() { return 1 as libc::c_int } //the template itself
    m.body[0 as libc::c_int as usize] =
        selectedPlayer as UBYTE as libc::c_char;
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           pTempl as *const libc::c_void,
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    m.size =
        (::std::mem::size_of::<DROID_TEMPLATE>() as
             libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_uint) as
            UWORD;
    m.type_0 = NET_TEMPLATE as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
//extern BOOL receiveWholeStructure	(NETMSG *pMsg);
// send/recv Template information
// receive a template created by another player
#[no_mangle]
pub unsafe extern "C" fn recvTemplate(mut m: *mut NETMSG) -> BOOL {
    let mut player_0: UBYTE = 0;
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut t: DROID_TEMPLATE =
        DROID_TEMPLATE{ref_0: 0,
                       pName: 0 as *mut STRING,
                       aName: [0; 60],
                       NameVersion: 0,
                       asParts: [0; 8],
                       buildPoints: 0,
                       powerPoints: 0,
                       storeCount: 0,
                       numWeaps: 0,
                       asWeaps: [0; 1],
                       droidType: DROID_WEAPON,
                       multiPlayerID: 0,
                       psNext: 0 as *mut _droid_template,};
    player_0 = (*m).body[0 as libc::c_int as usize] as UBYTE;
    if (player_0 as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"recvtemplate: invalid player size: %d\x00" as *const u8 as
                  *const libc::c_char, player_0 as libc::c_int);
    };
    if (player_0 as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"multiplay.c\x00" as *const u8 as *const libc::c_char,
              1372 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"recvTemplate\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if ((*m).size as libc::c_uint) <
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong {
        return 1 as libc::c_int
    }
    memcpy(&mut t as *mut DROID_TEMPLATE as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    psTempl = IdToTemplate(t.multiPlayerID, player_0 as UDWORD);
    if !psTempl.is_null() {
        // already exists.
        t.psNext = (*psTempl).psNext;
        memcpy(psTempl as *mut libc::c_void,
               &mut t as *mut DROID_TEMPLATE as *const libc::c_void,
               ::std::mem::size_of::<DROID_TEMPLATE>() as libc::c_ulong);
    } else {
        addTemplate(player_0 as UDWORD, &mut t);
        (*apsDroidTemplates[player_0 as usize]).ref_0 =
            0xc0000 as libc::c_int as UDWORD
        // templates are the odd one out!
        /*		if (!HEAP_ALLOC(psTemplateHeap, &psTempl))
		{
			return FALSE;
		}
		memcpy(psTempl, &t, sizeof(DROID_TEMPLATE));
		psTempl->ref = REF_TEMPLATE_START;					// templates are the odd one out!
		if (apsDroidTemplates[player])						// Add it to the list
		{
			for(psCurr = apsDroidTemplates[player];
				psCurr->psNext != NULL;
				psCurr = psCurr->psNext
				);

			psCurr->psNext = psTempl;
			psTempl->psNext = NULL;
		}
		else
		{
			apsDroidTemplates[player]=psTempl;
			psTempl->psNext = NULL;
		}
*/
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// inform others that you no longer have a template
#[no_mangle]
pub unsafe extern "C" fn SendDestroyTemplate(mut t: *mut DROID_TEMPLATE)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],}; // send player number
    m.body[0 as libc::c_int as usize] = selectedPlayer as libc::c_char;
    // send id of template to destroy
    memcpy(&mut *m.body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*t).multiPlayerID as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size = 5 as libc::c_int as libc::c_ushort;
    m.type_0 = NET_TEMPLATEDEST as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 0 as libc::c_int);
}
// send/recv Template destruction info
// acknowledge another player no longer has a template
#[no_mangle]
pub unsafe extern "C" fn recvDestroyTemplate(mut m: *mut NETMSG) -> BOOL {
    let mut player_0: UDWORD = 0; // decode the message
    let mut targetref: UDWORD = 0; // first find it.
    let mut psTempl: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psTempPrev: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    player_0 = (*m).body[0 as libc::c_int as usize] as UDWORD;
    memcpy(&mut targetref as *mut UDWORD as *mut libc::c_void,
           &mut *(*m).body.as_mut_ptr().offset(1 as libc::c_int as isize) as
               *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    psTempPrev = 0 as *mut DROID_TEMPLATE;
    psTempl = apsDroidTemplates[player_0 as usize];
    while !psTempl.is_null() {
        if (*psTempl).multiPlayerID == targetref { break ; }
        psTempPrev = psTempl;
        psTempl = (*psTempl).psNext
    }
    if !psTempl.is_null() {
        // if we found itthen delete it.
        if !psTempPrev.is_null() {
            // Update list pointers.
            (*psTempPrev).psNext = (*psTempl).psNext
            // It's down the list somewhere ?
        } else {
            apsDroidTemplates[player_0 as usize] = (*psTempl).psNext
            // It's at the root ?
        }
        heapFree(psTemplateHeap, psTempl as *mut libc::c_void);
        // Delete the template.
    }
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////
// Features
// send a destruct feature message.
#[no_mangle]
pub unsafe extern "C" fn SendDestroyFeature(mut pF: *mut FEATURE) -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    // only send the destruction if the feature is our responsibility
    memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
               *mut libc::c_char as *mut libc::c_void,
           &mut (*pF).id as *mut UDWORD as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    m.size =
        ::std::mem::size_of::<UDWORD>() as libc::c_ulong as libc::c_ushort;
    m.type_0 = NET_FEATUREDEST as libc::c_int as libc::c_uchar;
    return NETbcast(&mut m, 1 as libc::c_int);
}
// send a destruct feature message.
// process a destroy feature msg.
#[no_mangle]
pub unsafe extern "C" fn recvDestroyFeature(mut pMsg: *mut NETMSG) -> BOOL {
    let mut pF: *mut FEATURE = 0 as *mut FEATURE; // get feature id
    let mut id: UDWORD = 0;
    memcpy(&mut id as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    //	for(pF = apsFeatureLists[0]; pF && (pF->id  != id);	pF = pF->psNext);	// find the feature
    pF = IdToFeature(id, 99 as libc::c_int as UDWORD);
    if pF.is_null() {
        // if already a gonner.
        return 0 as libc::c_int
        // feature wasnt found anyway.
    } // remove, don't broadcast.
    bMultiPlayer = 0 as libc::c_int;
    removeFeature(pF);
    bMultiPlayer = 1 as libc::c_int;
    return 1 as libc::c_int;
}
// process a destroy feature msg.
// ////////////////////////////////////////////////////////////////////////////
// a generic destroy function, with killer info included.
#[no_mangle]
pub unsafe extern "C" fn sendDestroyExtra(mut psKilled: *mut BASE_OBJECT,
                                          mut psKiller: *mut BASE_OBJECT)
 -> BOOL {
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut n: UDWORD = 0 as libc::c_int as UDWORD;
    /*	if(psKilled != NULL)
	{
		NetAdd(m,4,psKilled->id);	// id of thing killed
	}
	else
	{
		NetAdd(m,4,n);
	}
*/
    if !psKiller.is_null() {
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut (*psKiller).id as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
        // id of killer.
    } else {
        memcpy(&mut *m.body.as_mut_ptr().offset(0 as libc::c_int as isize) as
                   *mut libc::c_char as *mut libc::c_void,
               &mut n as *mut UDWORD as *const libc::c_void,
               ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    }
    m.type_0 = NET_DESTROYXTRA as libc::c_int as libc::c_uchar;
    m.size = 4 as libc::c_int as libc::c_ushort;
    return NETbcast(&mut m, 0 as libc::c_int);
}
// ////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub unsafe extern "C" fn recvDestroyExtra(mut pMsg: *mut NETMSG) -> BOOL {
    //	BASE_OBJECT	*psKilled;
//	UDWORD		 killedId;
    let mut psSrc: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut srcId: UDWORD = 0;
    let mut psKiller: *mut DROID = 0 as *mut DROID;
    /*
	NetGet(pMsg,0,killedId);					// remove as normal
	if(killedId !=0)
	{
		psKilled = IdToPointer(killedId,ANYPLAYER);
		if(psKilled)
		{
			switch(psKilled->type)
			{
			case OBJ_DROID:
				recvDestroyDroid(pMsg);
				break;
			case OBJ_STRUCTURE:
				recvDestroyStructure(pMsg);
				break;
			case OBJ_FEATURE:
				recvDestroyFeature(pMsg);
				break;
			}
		}
	}
*/
    memcpy(&mut srcId as *mut UDWORD as *mut libc::c_void,
           &mut *(*pMsg).body.as_mut_ptr().offset(0 as libc::c_int as isize)
               as *mut libc::c_char as *const libc::c_void,
           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
    if srcId != 0 as libc::c_int as libc::c_uint {
        psSrc = IdToPointer(srcId, 99 as libc::c_int as UDWORD);
        if !psSrc.is_null() &&
               (*psSrc).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
            // process extra bits.
            psKiller = psSrc as *mut DROID;
            if !psKiller.is_null() {
                (*psKiller).numKills = (*psKiller).numKills.wrapping_add(1)
            }
            cmdDroidUpdateKills(psKiller);
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Network Audio packet processor.
#[no_mangle]
pub unsafe extern "C" fn recvAudioMsg(mut pMsg: *mut NETMSG) -> BOOL {
    NETplayIncomingAudio(pMsg);
    return 1 as libc::c_int;
}
// ////////////////////////////////////////////////////////////////////////////
// Network File packet processor.
#[no_mangle]
pub unsafe extern "C" fn recvMapFileRequested(mut pMsg: *mut NETMSG) -> BOOL {
    let mut mapStr: [libc::c_char; 256] = [0; 256];
    let mut mapName: [libc::c_char; 256] = [0; 256];
    let mut fixedname: [libc::c_char; 256] = [0; 256];
    //	pMsg;
    // another player is requesting the map
    if NetPlay.bHost == 0 { return 1 as libc::c_int }
    //#ifndef DISABLEMAPSEND
	// start sending the map to the other players.
    if bSendingMap == 0 {
        memset(mapStr.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               256 as libc::c_int as libc::c_uint);
        memset(mapName.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               256 as libc::c_int as libc::c_uint);
        memset(fixedname.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               256 as libc::c_int as libc::c_uint);
        bSendingMap = 1 as libc::c_int;
        addConsoleMessage(b"SENDING MAP!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
        addConsoleMessage(b"FIX FOR LINUX!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
        strcpy(mapName.as_mut_ptr(), game.map.as_mut_ptr());
        // chop off the -T1
        mapName[strlen(game.map.as_mut_ptr()).wrapping_sub(3 as libc::c_int as
                                                               libc::c_uint)
                    as usize] =
            0 as libc::c_int as libc::c_char; // chop off the -T1 etc..
        // chop off the sk- if required.
        if strncmp(mapName.as_mut_ptr(),
                   b"Sk-\x00" as *const u8 as *const libc::c_char,
                   3 as libc::c_int as libc::c_uint) == 0 as libc::c_int {
            strcpy(mapStr.as_mut_ptr(),
                   &mut *mapName.as_mut_ptr().offset(3 as libc::c_int as
                                                         isize)); //.wdg
            strcpy(mapName.as_mut_ptr(),
                   mapStr.as_mut_ptr()); //We know maps are in /maps dir...now. fix for linux -Q
        }
        sprintf(mapStr.as_mut_ptr(),
                b"%dc-%s\x00" as *const u8 as *const libc::c_char,
                game.maxPlayers as libc::c_int, mapName.as_mut_ptr());
        strcat(mapStr.as_mut_ptr(),
               b".wz\x00" as *const u8 as *const libc::c_char);
        sprintf(fixedname.as_mut_ptr(),
                b"maps\\%s\x00" as *const u8 as *const libc::c_char,
                mapStr.as_mut_ptr());
        memcpy(mapStr.as_mut_ptr() as *mut libc::c_void,
               fixedname.as_mut_ptr() as *const libc::c_void,
               256 as libc::c_int as libc::c_uint);
        NETsendFile(1 as libc::c_int, mapStr.as_mut_ptr(), 0 as libc::c_int);
    }
    //#endif
    return 1 as libc::c_int;
}
// continue sending the map
#[no_mangle]
pub unsafe extern "C" fn sendMap() -> UBYTE {
    let mut done: UBYTE = 0;
    static mut lastCall: UDWORD = 0;
    if lastCall > gameTime { lastCall = 0 as libc::c_int as UDWORD }
    if gameTime.wrapping_sub(lastCall) < 200 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as UBYTE
    }
    lastCall = gameTime;
    done =
        NETsendFile(0 as libc::c_int, game.map.as_mut_ptr(),
                    0 as libc::c_int);
    if done as libc::c_int == 100 as libc::c_int {
        addConsoleMessage(b"MAP SENT!\x00" as *const u8 as *const libc::c_char
                              as *mut STRING, DEFAULT_JUSTIFY);
        bSendingMap = 0 as libc::c_int
    }
    return done;
}
// another player is broadcasting a map, recv a chunk
#[no_mangle]
pub unsafe extern "C" fn recvMapFileData(mut pMsg: *mut NETMSG) -> BOOL {
    let mut done: UBYTE = 0; //send
    done = NETrecvFile(pMsg);
    if done as libc::c_int == 100 as libc::c_int {
        addConsoleMessage(b"MAP DOWNLOADED!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
        sendTextMessage(b"MAP DOWNLOADED\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int);
        // clear out the old level list.
        levShutDown();
        levInitialise();
        if buildMapList() == 0 { return 0 as libc::c_int }
    } else {
        flushConsoleMessages();
        sprintf(ConsoleString.as_mut_ptr(),
                b"MAP:%d%%\x00" as *const u8 as *const libc::c_char,
                done as libc::c_int);
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    }
    return 1 as libc::c_int;
}
//------------------------------------------------------------------------------------------------//
/* multiplayer message stack */
#[no_mangle]
pub unsafe extern "C" fn msgStackReset() {
    msgStackPos = -(1 as libc::c_int);
    //Beginning of the stack
}
//make AI process a message
/* for multiplayer message stack */
#[no_mangle]
pub unsafe extern "C" fn msgStackPush(mut CBtype: SDWORD, mut plFrom: SDWORD,
                                      mut plTo: SDWORD, mut tStr: *mut STRING,
                                      mut x: SDWORD, mut y: SDWORD)
 -> UDWORD {
    if msgStackPos >= 100 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackPush() - stack full\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int as UDWORD
    }
    //make point to the last valid element
    msgStackPos += 1;
    //remember values
    msgPlFrom[msgStackPos as usize] = plFrom;
    msgPlTo[msgStackPos as usize] = plTo;
    callbackType[msgStackPos as usize] = CBtype;
    locx[msgStackPos as usize] = x;
    locy[msgStackPos as usize] = y;
    strcpy(msgStr[msgStackPos as usize].as_mut_ptr(), tStr);
    return 1 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn isMsgStackEmpty() -> BOOL {
    if msgStackPos == -(1 as libc::c_int) { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetFrom(mut psVal: *mut SDWORD) -> BOOL {
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackGetFrom: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    *psVal = msgPlFrom[0 as libc::c_int as usize];
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetTo(mut psVal: *mut SDWORD) -> BOOL {
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackGetTo: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    *psVal = msgPlTo[0 as libc::c_int as usize];
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetCallbackType(mut psVal: *mut SDWORD)
 -> BOOL {
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackGetCallbackType: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    *psVal = callbackType[0 as libc::c_int as usize];
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetXY(mut psValx: *mut SDWORD,
                                       mut psValy: *mut SDWORD) -> BOOL {
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackGetXY: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    *psValx = locx[0 as libc::c_int as usize];
    *psValy = locy[0 as libc::c_int as usize];
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetMsg(mut psVal: *mut STRING) -> BOOL {
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackGetMsg: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    strcpy(psVal, msgStr[0 as libc::c_int as usize].as_mut_ptr());
    //*psVal = msgPlTo[msgStackPos];
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackSort() -> BOOL {
    let mut i: SDWORD = 0;
    //go through all-1 elements (bottom-top)
    i = 0 as libc::c_int;
    while i < msgStackPos {
        msgPlFrom[i as usize] = msgPlFrom[(i + 1 as libc::c_int) as usize];
        msgPlTo[i as usize] = msgPlTo[(i + 1 as libc::c_int) as usize];
        callbackType[i as usize] =
            callbackType[(i + 1 as libc::c_int) as usize];
        locx[i as usize] = locx[(i + 1 as libc::c_int) as usize];
        locy[i as usize] = locy[(i + 1 as libc::c_int) as usize];
        strcpy(msgStr[i as usize].as_mut_ptr(),
               msgStr[(i + 1 as libc::c_int) as usize].as_mut_ptr());
        i += 1
    }
    //erase top element
    msgPlFrom[msgStackPos as usize] =
        -(2 as libc::c_int); //since removed the top element
    msgPlTo[msgStackPos as usize] = -(2 as libc::c_int);
    callbackType[msgStackPos as usize] = -(2 as libc::c_int);
    locx[msgStackPos as usize] = -(2 as libc::c_int);
    locy[msgStackPos as usize] = -(2 as libc::c_int);
    strcpy(msgStr[msgStackPos as usize].as_mut_ptr(),
           b"ERROR STRING!!!!!!!!\x00" as *const u8 as *const libc::c_char);
    msgStackPos -= 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackPop() -> BOOL {
    if msgStackPos < 0 as libc::c_int || msgStackPos >= 100 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackPop: wrong msgStackPos index\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return msgStackSort();
    //move allelements 1 pos lower
}
#[no_mangle]
pub unsafe extern "C" fn msgStackGetCount() -> SDWORD {
    return msgStackPos + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn msgStackFireTop() -> BOOL {
    let mut _callbackType: SDWORD = 0;
    let mut msg: [STRING; 255] = [0; 255];
    if msgStackPos < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"msgStackFireTop: msgStackPos < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if msgStackGetCallbackType(&mut _callbackType) == 0 {
        return 0 as libc::c_int
    }
    match _callbackType {
        59 => {
            /*
		case CALL_BEACON:

			if(!msgStackGetXY(&beaconX, &beaconY))
				return FALSE;

			if(!msgStackGetFrom(&MultiMsgPlayerFrom))
				return FALSE;

			if(!msgStackGetTo(&MultiMsgPlayerTo))
				return FALSE;

			if(!msgStackGetMsg(msg))
				return FALSE;

			strcpy(MultiplayMsg, msg);

			eventFireCallbackTrigger(CALL_BEACON);
			break;
*/
            if msgStackGetFrom(&mut MultiMsgPlayerFrom) == 0 {
                return 0 as libc::c_int
            }
            if msgStackGetTo(&mut MultiMsgPlayerTo) == 0 {
                return 0 as libc::c_int
            }
            if msgStackGetMsg(msg.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            strcpy(MultiplayMsg.as_mut_ptr(), msg.as_mut_ptr());
            eventFireCallbackTrigger(CALL_AI_MSG as libc::c_int as
                                         TRIGGER_TYPE);
        }
        _ => {
            debug(LOG_ERROR,
                  b"msgStackFireTop: unknown callback type\x00" as *const u8
                      as *const libc::c_char);
            return 0 as libc::c_int
        }
    }
    if msgStackPop() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
