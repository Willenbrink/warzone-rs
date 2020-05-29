use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
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
    fn widgSetButtonState(psScreen: *mut W_SCREEN, id: UDWORD, state: UDWORD);
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    fn widgSetUserData(psScreen: *mut W_SCREEN, id: UDWORD,
                       UserData: *mut libc::c_void);
    #[no_mangle]
    fn widgGetTabs(psScreen: *mut W_SCREEN, id: UDWORD, pMajor: *mut UWORD,
                   pMinor: *mut UWORD);
    #[no_mangle]
    fn widgSetTabs(psScreen: *mut W_SCREEN, id: UDWORD, major: UWORD,
                   minor: UWORD);
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    #[no_mangle]
    fn widgAddBarGraph(psScreen: *mut W_SCREEN, psInit: *mut W_BARINIT)
     -> BOOL;
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayObjDynamicTrack(psObj: *mut libc::c_void,
                                 iTrack: libc::c_int,
                                 pUserCallback: AUDIO_CALLBACK) -> BOOL;
    #[no_mangle]
    fn audio_StopObjTrack(psObj: *mut libc::c_void, iTrack: libc::c_int);
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    fn intIsRefreshing() -> BOOL;
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    #[no_mangle]
    fn numForms(total: UDWORD, perForm: UDWORD) -> UWORD;
    #[no_mangle]
    static mut intMode: INTMODE;
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    #[no_mangle]
    static mut WFont: libc::c_int;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut ClosingTrans: BOOL;
    #[no_mangle]
    static mut ClosingTransCont: BOOL;
    #[no_mangle]
    static mut ClosingTransDroids: BOOL;
    #[no_mangle]
    static mut TopicBuffers: [RENDERED_BUTTON; 20];
    #[no_mangle]
    static mut ObjectBuffers: [RENDERED_BUTTON; 40];
    #[no_mangle]
    static mut StatBuffers: [RENDERED_BUTTON; 80];
    #[no_mangle]
    static mut System0Buffers: [RENDERED_BUTTON; 80];
    // Clear ( make unused ) all RENDERED_BUTTON structures for the object window.
    #[no_mangle]
    fn ClearObjectBuffers();
    // Clear ( make unused ) all RENDERED_BUTTON structures for the topic window.
    #[no_mangle]
    fn ClearTopicBuffers();
    // Clear ( make unused ) a RENDERED_BUTTON structure.
    #[no_mangle]
    fn ClearObjectButtonBuffer(BufferID: SDWORD);
    // Clear ( make unused ) a RENDERED_BUTTON structure.
    #[no_mangle]
    fn ClearTopicButtonBuffer(BufferID: SDWORD);
    #[no_mangle]
    fn RefreshObjectButtons();
    #[no_mangle]
    fn RefreshSystem0Buttons();
    #[no_mangle]
    fn RefreshTopicButtons();
    #[no_mangle]
    fn RefreshStatsButtons();
    // Get a free RENDERED_BUTTON structure for a stat window button.
    #[no_mangle]
    fn GetStatBuffer() -> SDWORD;
    // Clear ( make unused ) all RENDERED_BUTTON structures for the stat window.
    #[no_mangle]
    fn ClearStatBuffers();
    /*these have been set up for the Transporter - the design screen DOESN'T use them*/
// Clear ( make unused ) *all* RENDERED_BUTTON structures.
    #[no_mangle]
    fn ClearSystem0Buffers();
    // Get a free RENDERED_BUTTON structure.
    #[no_mangle]
    fn GetSystem0Buffer() -> SDWORD;
    #[no_mangle]
    fn intDisplayStatusButton(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectButton(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayObjectForm(psWidget: *mut _widget, xOffset: UDWORD,
                            yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intClosePlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                           yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayTab(psWidget: *mut _widget, TabType: UDWORD,
                     Position: UDWORD, Number: UDWORD, Selected: BOOL,
                     Hilight: BOOL, x: UDWORD, y: UDWORD, Width: UDWORD,
                     Height: UDWORD);
    // A few useful defined tabs.
    #[no_mangle]
    static mut StandardTab: TABDEF;
    #[no_mangle]
    static mut SmallTab: TABDEF;
    /* Remove a droid from the apsDroidLists so doesn't update or get drawn etc*/
//returns TRUE if successfully removed from the list
    #[no_mangle]
    fn droidRemove(psDroid: *mut DROID, pList: *mut *mut DROID) -> BOOL;
    // Get a droid's name.
    #[no_mangle]
    fn droidGetName(psDroid: *mut DROID) -> *mut STRING;
    #[no_mangle]
    fn zonedPAT(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn pickATileGen(x: *mut UDWORD, y: *mut UDWORD, numIterations: UBYTE,
                    function:
                        Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                   -> BOOL>) -> BOOL;
    //initialises the droid movement model
    #[no_mangle]
    fn initDroidMovement(psDroid: *mut DROID);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    // Widget callback to display a contents button for the Transporter
    #[no_mangle]
    fn intDisplayTransportButton(psWidget: *mut _widget, xOffset: UDWORD,
                                 yOffset: UDWORD, pColours: *mut UDWORD);
    /* Set the button state of a click form */
    #[no_mangle]
    fn formSetClickState(psForm: *mut W_CLICKFORM, state: UDWORD);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    #[no_mangle]
    fn FindATransporter() -> *mut DROID;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /*
 * Mission.h
 *
 * Mission defines for the game
 *
 */
    /*the number of areas that can be defined to prevent buildings being placed - 
used for Transporter Landing Zones 0-7 are for players, 8 = LIMBO_LANDING*/
    // Set by scrFlyInTransporter. True if were currenly tracking the transporter.
    // return positions for vtols
    //this is called everytime the game is quit
    /*on the PC - sets the countdown played flag*/
    //extern BOOL startMission(MISSION_TYPE missionType, STRING *pGame);
    // initialise the mission stuff for a save game
    //sets up the game to start a new mission
//extern BOOL setUpMission(MISSION_TYPE type);
    //this causes the new mission data to be loaded up
    /* The update routine for all Structures left back at base during a Mission*/
    /* The update routine for all droids left back at home base
Only interested in Transporters at present*/
    //returns TRUE if the mission is a Limbo Expand mission
    //this is called mid Limbo mission via the script
    // mission results.
    //timer display for transporter timer
    //position defines
    // status of the mission result screens.
    /*sets the appropriate pause states for when the interface is up but the 
game needs to be paused*/
    /*resets the pause states */
    //returns the x coord for where the Transporter can land
    //returns the y coord for where the Transporter can land
    /*checks that the timer has been set and that a Transporter exists before 
adding the timer button*/
    /*update routine for mission details */
    /*checks the time has been set and then adds the timer if not already on 
the display*/
    //access functions for bPlayCountDown flag
    /*	checks the x,y passed in are not within the boundary of the Landing Zone
	x and y in tile coords */
    //sets the coords for the Transporter to land
    /*Initialises all the nogo areas to 0*/
    //sets the coords for a no go area
    /* fly in transporters at start of level */
    /* move transporter offworld */
    /* pick nearest map edge to point */
    /*builds a droid back at the home base whilst on a mission - stored in a list made
available to the transporter interface*/
    //This is just a very big number - bigger than a map width/height could ever be!
    //access functions for droidsToSafety flag
    #[no_mangle]
    fn getDroidsToSafetyFlag() -> BOOL;
    //checks to see if the player has any droids (except Transporters left)
    #[no_mangle]
    fn missionDroidsRemaining(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn missionIsOffworld() -> BOOL;
    #[no_mangle]
    fn missionForReInforcements() -> BOOL;
    #[no_mangle]
    fn missionGetTransporterExit(iPlayer: SDWORD, iX: *mut UWORD,
                                 iY: *mut UWORD);
    #[no_mangle]
    fn missionSetReinforcementTime(iTime: UDWORD);
    #[no_mangle]
    fn getLandingX(iPlayer: SDWORD) -> UWORD;
    #[no_mangle]
    fn getLandingY(iPlayer: SDWORD) -> UWORD;
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
    /* Frame update for the movement of a tracked droid */
    /* update body and turret to local slope */
    #[no_mangle]
    fn updateDroidOrientation(psDroid: *mut DROID);
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    /* Give a droid an action */
    #[no_mangle]
    fn actionDroid(psDroid: *mut DROID, action: DROID_ACTION);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    static mut aSinTable: [SDWORD; 0];
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn sendDroidEmbark(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn intSetCurrentCursorPosition(Snap: *mut CURSORSNAP, id: UDWORD);
    /*
 * transporter.c
 *
 * code to deal with loading/unloading, interface and flight
*/
    // FIXME Direct iVis implementation include!
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _block_heap {
    pub init: SDWORD,
    pub ext: SDWORD,
    pub psBlocks: *mut BLOCK_HEAP_MEM,
    pub psNext: *mut _block_heap,
}
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
// size of block
// pointer to the start of the free memory section
// pointer to the base of the memory block
// The start of the last allocated block (so that it can be freed by blkSpecialFree
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
// root of the tree
/*
 * WidgBase.h
 *
 * Definitions for the basic widget types.
 */
/* The different base types of widget */
/* Button colours */
// Colour for button text
// Colour for button background
// Colour for button border
// 2nd border colour
// Hilite colour
/* The display function prototype */
/* The optional user callback function */
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_context {
    pub psScreen: *mut W_SCREEN,
    pub psForm: *mut _w_form,
    pub xOffset: SDWORD,
    pub yOffset: SDWORD,
    pub mx: SDWORD,
    pub my: SDWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_form {
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
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
}
pub type WIDGET = _widget;
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
pub type W_SCREEN = _w_screen;
// ID of the IVIS font to use for tool tips.
/* The common widget data */
/* The screen structure which stores all info for a widget screen */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type WIDGET_AUDIOCALLBACK
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_colourdef {
    pub red: UBYTE,
    pub green: UBYTE,
    pub blue: UBYTE,
    pub alpha: UBYTE,
}
pub type W_COLOURDEF = _w_colourdef;
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_forminit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub disableChildren: BOOL,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub numMajor: UWORD,
    pub aNumMinors: [UWORD; 9],
    pub pTip: *mut STRING,
    pub apMajorTips: [*mut STRING; 9],
    pub apMinorTips: [[*mut STRING; 5]; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
pub type W_FORMINIT = _w_forminit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_labinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
pub type W_LABINIT = _w_labinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_butinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub pText: *mut STRING,
    pub pTip: *mut STRING,
    pub FontID: libc::c_int,
}
pub type W_BUTINIT = _w_butinit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_barinit {
    pub formID: UDWORD,
    pub majorID: UWORD,
    pub minorID: UWORD,
    pub id: UDWORD,
    pub style: UDWORD,
    pub x: SWORD,
    pub y: SWORD,
    pub width: UWORD,
    pub height: UWORD,
    pub pDisplay: WIDGET_DISPLAY,
    pub pCallback: WIDGET_CALLBACK,
    pub pUserData: *mut libc::c_void,
    pub UserData: UDWORD,
    pub orientation: UWORD,
    pub size: UWORD,
    pub minorSize: UWORD,
    pub iRange: UWORD,
    pub sCol: W_COLOURDEF,
    pub sMinorCol: W_COLOURDEF,
    pub pTip: *mut STRING,
}
pub type W_BARINIT = _w_barinit;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iClip {
    pub left: libc::c_int,
    pub top: libc::c_int,
    pub right: libc::c_int,
    pub bottom: libc::c_int,
}
pub type iBitmap = uint8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSprite {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub bmp: *mut iBitmap,
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
// Droids
// All Buildings
// Things like roads, trees, bridges, fires
// Comes out of guns, stupid :-)
// for the camera tracking
//ALL components and structures and research topics have a tech level to which they belong
pub type TECH_LEVEL = _tech_level;
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
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
/* common stats - N.B. system points for a body
	 * are the limit for all other components
	 */
//UDWORD		configuration;		// Body shape
	//UDWORD		armourMult;			// How configuration affects body shape
									// Actually could be a fractional value
									// store x 10
// How big the body is - affects how hit
//UDWORD		bodyPoints;			// How much damage the body can take before destruction
// The number of weapon slots on the body
// A measure of how much protection the armour provides
// cross-ref with the weapon types
// A measure of how much energy the power plant outputs
//list of IMDs to use for propulsion unit - up to numPropulsionStats
//pointer to which flame graphic to use - for VTOLs only at the moment
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iSurface {
    pub usr: libc::c_int,
    pub flags: uint32,
    pub xcentre: libc::c_int,
    pub ycentre: libc::c_int,
    pub xpshift: libc::c_int,
    pub ypshift: libc::c_int,
    pub clip: iClip,
    pub buffer: *mut uint8,
    pub scantable: [int32; 1024],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub size: int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEHEADER {
    pub Type: [UBYTE; 4],
    pub Version: UWORD,
    pub NumImages: UWORD,
    pub BitDepth: UWORD,
    pub NumTPages: UWORD,
    pub TPageFiles: [[UBYTE; 16]; 16],
    pub PalFile: [UBYTE; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEDEF {
    pub TPageID: UWORD,
    pub PalID: UWORD,
    pub Tu: UWORD,
    pub Tv: UWORD,
    pub Width: UWORD,
    pub Height: UWORD,
    pub XOffset: SWORD,
    pub YOffset: SWORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAGEFILE {
    pub Header: IMAGEHEADER,
    pub TexturePages: *mut iSprite,
    pub NumCluts: UWORD,
    pub TPageIDs: [UWORD; 16],
    pub ClutIDs: [UWORD; 48],
    pub ImageDefs: *mut IMAGEDEF,
}
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
/* * unsigned 32-bit integer */
pub type ALuint = libc::c_uint;
/* **************************************************************************/
/* **************************************************************************/
/* defines */
/* **************************************************************************/
/* **************************************************************************/
/* enums */
/* **************************************************************************/
/* forward definitions
 */
/* **************************************************************************/
/* typedefs
 */
/* **************************************************************************/
/* structs */
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
//line build requires two sets of coords
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
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
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
pub type INTMODE = libc::c_uint;
pub const INT_MAXMODE: INTMODE = 15;
pub const INT_CDCHANGE: INTMODE = 14;
pub const INT_MULTIMENU: INTMODE = 13;
pub const INT_MISSIONRES: INTMODE = 12;
pub const INT_TRANSPORTER: INTMODE = 11;
pub const INT_INGAMEOP: INTMODE = 10;
pub const INT_ORDER: INTMODE = 9;
pub const INT_INTELMAP: INTMODE = 8;
pub const INT_DESIGN: INTMODE = 7;
pub const INT_CMDORDER: INTMODE = 6;
pub const INT_STAT: INTMODE = 5;
pub const INT_OBJECT: INTMODE = 4;
pub const INT_EDITSTAT: INTMODE = 3;
pub const INT_EDIT: INTMODE = 2;
pub const INT_OPTION: INTMODE = 1;
pub const INT_NORMAL: INTMODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_minortab {
    pub psWidgets: *mut WIDGET,
    pub pTip: *mut STRING,
}
pub type W_MINORTAB = _w_minortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_majortab {
    pub lastMinor: UWORD,
    pub numMinor: UWORD,
    pub asMinor: [W_MINORTAB; 5],
    pub pTip: *mut STRING,
}
pub type W_MAJORTAB = _w_majortab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_tabform {
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
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
    pub majorPos: UWORD,
    pub minorPos: UWORD,
    pub majorSize: UWORD,
    pub minorSize: UWORD,
    pub tabMajorThickness: UWORD,
    pub tabMinorThickness: UWORD,
    pub tabMajorGap: UWORD,
    pub tabMinorGap: UWORD,
    pub tabVertOffset: SWORD,
    pub tabHorzOffset: SWORD,
    pub majorOffset: SWORD,
    pub minorOffset: SWORD,
    pub majorT: UWORD,
    pub minorT: UWORD,
    pub state: UWORD,
    pub tabHiLite: UWORD,
    pub numMajor: UWORD,
    pub asMajor: [W_MAJORTAB; 9],
    pub pTabDisplay: TAB_DISPLAY,
    pub pFormDisplay: WIDGET_DISPLAY,
}
pub type W_TABFORM = _w_tabform;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_clickform {
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
    pub disableChildren: BOOL,
    pub Ax0: UWORD,
    pub Ay0: UWORD,
    pub Ax1: UWORD,
    pub Ay1: UWORD,
    pub animCount: UDWORD,
    pub startTime: UDWORD,
    pub aColours: [UDWORD; 8],
    pub psLastHiLite: *mut WIDGET,
    pub psWidgets: *mut WIDGET,
    pub state: UDWORD,
    pub pTip: *mut STRING,
    pub HilightAudioID: SWORD,
    pub ClickedAudioID: SWORD,
    pub AudioCallback: WIDGET_AUDIOCALLBACK,
}
pub type W_CLICKFORM = _w_clickform;
// Feature armour
/*
 * Label.h
 *
 * Definitions for the label widget.
 */
/* The widget heaps */
// label states.
// label is hilited
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_label {
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
    pub state: UDWORD,
    pub aText: [STRING; 80],
    pub FontID: libc::c_int,
    pub pTip: *mut STRING,
}
pub type W_LABEL = _w_label;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDEF {
    pub MajorUp: SWORD,
    pub MajorDown: SWORD,
    pub MajorHilight: SWORD,
    pub MajorSelected: SWORD,
    pub MinorUp: SWORD,
    pub MinorDown: SWORD,
    pub MinorHilight: SWORD,
    pub MinorSelected: SWORD,
}
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed = 479;
pub const IMAGE_BLUE6: C2RustUnnamed = 478;
pub const IMAGE_BLUE5: C2RustUnnamed = 477;
pub const IMAGE_BLUE4: C2RustUnnamed = 476;
pub const IMAGE_BLUE3: C2RustUnnamed = 475;
pub const IMAGE_BLUE2: C2RustUnnamed = 474;
pub const IMAGE_BLUE1: C2RustUnnamed = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed = 468;
pub const IMAGE_TARGET5: C2RustUnnamed = 467;
pub const IMAGE_TARGET4: C2RustUnnamed = 466;
pub const IMAGE_TARGET3: C2RustUnnamed = 465;
pub const IMAGE_TARGET2: C2RustUnnamed = 464;
pub const IMAGE_TARGET1: C2RustUnnamed = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed = 457;
pub const IMAGE_ASCII131: C2RustUnnamed = 456;
pub const IMAGE_ASCII161: C2RustUnnamed = 455;
pub const IMAGE_ASCII191: C2RustUnnamed = 454;
pub const IMAGE_ASCII208: C2RustUnnamed = 453;
pub const IMAGE_ASCII207: C2RustUnnamed = 452;
pub const IMAGE_ASCII206: C2RustUnnamed = 451;
pub const IMAGE_ASCII205: C2RustUnnamed = 450;
pub const IMAGE_ASCII204: C2RustUnnamed = 449;
pub const IMAGE_ASCII203: C2RustUnnamed = 448;
pub const IMAGE_ASCII202: C2RustUnnamed = 447;
pub const IMAGE_ASCII201: C2RustUnnamed = 446;
pub const IMAGE_ASCII200: C2RustUnnamed = 445;
pub const IMAGE_ASCII198: C2RustUnnamed = 444;
pub const IMAGE_ASCII197: C2RustUnnamed = 443;
pub const IMAGE_ASCII196: C2RustUnnamed = 442;
pub const IMAGE_ASCII195: C2RustUnnamed = 441;
pub const IMAGE_ASCII194: C2RustUnnamed = 440;
pub const IMAGE_ASCII193: C2RustUnnamed = 439;
pub const IMAGE_ASCII192: C2RustUnnamed = 438;
pub const IMAGE_ASCII210: C2RustUnnamed = 437;
pub const IMAGE_ASCII211: C2RustUnnamed = 436;
pub const IMAGE_ASCII214: C2RustUnnamed = 435;
pub const IMAGE_ASCII213: C2RustUnnamed = 434;
pub const IMAGE_ASCII212: C2RustUnnamed = 433;
pub const IMAGE_ASCII216: C2RustUnnamed = 432;
pub const IMAGE_ASCII220: C2RustUnnamed = 431;
pub const IMAGE_ASCII217: C2RustUnnamed = 430;
pub const IMAGE_ASCII219: C2RustUnnamed = 429;
pub const IMAGE_ASCII218: C2RustUnnamed = 428;
pub const IMAGE_ASCII221: C2RustUnnamed = 427;
pub const IMAGE_ASCII223: C2RustUnnamed = 426;
pub const IMAGE_ASCII248: C2RustUnnamed = 425;
pub const IMAGE_ASCII241: C2RustUnnamed = 424;
pub const IMAGE_ASCII253: C2RustUnnamed = 423;
pub const IMAGE_ASCII252: C2RustUnnamed = 422;
pub const IMAGE_ASCII251: C2RustUnnamed = 421;
pub const IMAGE_ASCII250: C2RustUnnamed = 420;
pub const IMAGE_ASCII249: C2RustUnnamed = 419;
pub const IMAGE_ASCII246: C2RustUnnamed = 418;
pub const IMAGE_ASCII245: C2RustUnnamed = 417;
pub const IMAGE_ASCII244: C2RustUnnamed = 416;
pub const IMAGE_ASCII243: C2RustUnnamed = 415;
pub const IMAGE_ASCII242: C2RustUnnamed = 414;
pub const IMAGE_ASCII239: C2RustUnnamed = 413;
pub const IMAGE_ASCII238: C2RustUnnamed = 412;
pub const IMAGE_ASCII237: C2RustUnnamed = 411;
pub const IMAGE_ASCII236: C2RustUnnamed = 410;
pub const IMAGE_ASCII235: C2RustUnnamed = 409;
pub const IMAGE_ASCII234: C2RustUnnamed = 408;
pub const IMAGE_ASCII233: C2RustUnnamed = 407;
pub const IMAGE_ASCII232: C2RustUnnamed = 406;
pub const IMAGE_ASCII231: C2RustUnnamed = 405;
pub const IMAGE_ASCII230: C2RustUnnamed = 404;
pub const IMAGE_ASCII229: C2RustUnnamed = 403;
pub const IMAGE_ASCII228: C2RustUnnamed = 402;
pub const IMAGE_ASCII227: C2RustUnnamed = 401;
pub const IMAGE_ASCII226: C2RustUnnamed = 400;
pub const IMAGE_ASCII225: C2RustUnnamed = 399;
pub const IMAGE_ASCII224: C2RustUnnamed = 398;
pub const IMAGE_ASCII189: C2RustUnnamed = 397;
pub const IMAGE_ASCII188: C2RustUnnamed = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed = 395;
pub const IMAGE_LEV_7: C2RustUnnamed = 394;
pub const IMAGE_LEV_6: C2RustUnnamed = 393;
pub const IMAGE_LEV_5: C2RustUnnamed = 392;
pub const IMAGE_LEV_4: C2RustUnnamed = 391;
pub const IMAGE_LEV_3: C2RustUnnamed = 390;
pub const IMAGE_LEV_2: C2RustUnnamed = 389;
pub const IMAGE_LEV_1: C2RustUnnamed = 388;
pub const IMAGE_LEV_0: C2RustUnnamed = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed = 373;
pub const IMAGE_GN_0: C2RustUnnamed = 372;
pub const IMAGE_GN_1: C2RustUnnamed = 371;
pub const IMAGE_GN_2: C2RustUnnamed = 370;
pub const IMAGE_GN_3: C2RustUnnamed = 369;
pub const IMAGE_GN_4: C2RustUnnamed = 368;
pub const IMAGE_GN_5: C2RustUnnamed = 367;
pub const IMAGE_GN_6: C2RustUnnamed = 366;
pub const IMAGE_GN_7: C2RustUnnamed = 365;
pub const IMAGE_GN_8: C2RustUnnamed = 364;
pub const IMAGE_GN_9: C2RustUnnamed = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed = 349;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed = 348;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed = 320;
pub const IMAGE_NRUTER: C2RustUnnamed = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed = 253;
pub const IMAGE_TRACKS: C2RustUnnamed = 252;
pub const IMAGE_STAR: C2RustUnnamed = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed = 243;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed = 233;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed = 232;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed = 228;
pub const IMAGE_ASCII126: C2RustUnnamed = 227;
pub const IMAGE_ASCII125: C2RustUnnamed = 226;
pub const IMAGE_ASCII124: C2RustUnnamed = 225;
pub const IMAGE_ASCII123: C2RustUnnamed = 224;
pub const IMAGE_ASCII122: C2RustUnnamed = 223;
pub const IMAGE_ASCII121: C2RustUnnamed = 222;
pub const IMAGE_ASCII120: C2RustUnnamed = 221;
pub const IMAGE_ASCII119: C2RustUnnamed = 220;
pub const IMAGE_ASCII118: C2RustUnnamed = 219;
pub const IMAGE_ASCII117: C2RustUnnamed = 218;
pub const IMAGE_ASCII116: C2RustUnnamed = 217;
pub const IMAGE_ASCII115: C2RustUnnamed = 216;
pub const IMAGE_ASCII114: C2RustUnnamed = 215;
pub const IMAGE_ASCII113: C2RustUnnamed = 214;
pub const IMAGE_ASCII112: C2RustUnnamed = 213;
pub const IMAGE_ASCII111: C2RustUnnamed = 212;
pub const IMAGE_ASCII110: C2RustUnnamed = 211;
pub const IMAGE_ASCII109: C2RustUnnamed = 210;
pub const IMAGE_ASCII108: C2RustUnnamed = 209;
pub const IMAGE_ASCII107: C2RustUnnamed = 208;
pub const IMAGE_ASCII106: C2RustUnnamed = 207;
pub const IMAGE_ASCII105: C2RustUnnamed = 206;
pub const IMAGE_ASCII104: C2RustUnnamed = 205;
pub const IMAGE_ASCII103: C2RustUnnamed = 204;
pub const IMAGE_ASCII102: C2RustUnnamed = 203;
pub const IMAGE_ASCII101: C2RustUnnamed = 202;
pub const IMAGE_ASCII100: C2RustUnnamed = 201;
pub const IMAGE_ASCII99: C2RustUnnamed = 200;
pub const IMAGE_ASCII98: C2RustUnnamed = 199;
pub const IMAGE_ASCII97: C2RustUnnamed = 198;
pub const IMAGE_ASCII96: C2RustUnnamed = 197;
pub const IMAGE_ASCII95: C2RustUnnamed = 196;
pub const IMAGE_ASCII94: C2RustUnnamed = 195;
pub const IMAGE_ASCII93: C2RustUnnamed = 194;
pub const IMAGE_ASCII92: C2RustUnnamed = 193;
pub const IMAGE_ASCII91: C2RustUnnamed = 192;
pub const IMAGE_ASCII90: C2RustUnnamed = 191;
pub const IMAGE_ASCII89: C2RustUnnamed = 190;
pub const IMAGE_ASCII88: C2RustUnnamed = 189;
pub const IMAGE_ASCII87: C2RustUnnamed = 188;
pub const IMAGE_ASCII86: C2RustUnnamed = 187;
pub const IMAGE_ASCII85: C2RustUnnamed = 186;
pub const IMAGE_ASCII84: C2RustUnnamed = 185;
pub const IMAGE_ASCII83: C2RustUnnamed = 184;
pub const IMAGE_ASCII82: C2RustUnnamed = 183;
pub const IMAGE_ASCII81: C2RustUnnamed = 182;
pub const IMAGE_ASCII80: C2RustUnnamed = 181;
pub const IMAGE_ASCII79: C2RustUnnamed = 180;
pub const IMAGE_ASCII78: C2RustUnnamed = 179;
pub const IMAGE_ASCII77: C2RustUnnamed = 178;
pub const IMAGE_ASCII76: C2RustUnnamed = 177;
pub const IMAGE_ASCII75: C2RustUnnamed = 176;
pub const IMAGE_ASCII74: C2RustUnnamed = 175;
pub const IMAGE_ASCII73: C2RustUnnamed = 174;
pub const IMAGE_ASCII72: C2RustUnnamed = 173;
pub const IMAGE_ASCII71: C2RustUnnamed = 172;
pub const IMAGE_ASCII70: C2RustUnnamed = 171;
pub const IMAGE_ASCII69: C2RustUnnamed = 170;
pub const IMAGE_ASCII68: C2RustUnnamed = 169;
pub const IMAGE_ASCII67: C2RustUnnamed = 168;
pub const IMAGE_ASCII66: C2RustUnnamed = 167;
pub const IMAGE_ASCII65: C2RustUnnamed = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed = 165;
pub const IMAGE_ASCII63: C2RustUnnamed = 164;
pub const IMAGE_ASCII62: C2RustUnnamed = 163;
pub const IMAGE_ASCII61: C2RustUnnamed = 162;
pub const IMAGE_ASCII60: C2RustUnnamed = 161;
pub const IMAGE_ASCII59: C2RustUnnamed = 160;
pub const IMAGE_ASCII58: C2RustUnnamed = 159;
pub const IMAGE_ASCII57: C2RustUnnamed = 158;
pub const IMAGE_ASCII56: C2RustUnnamed = 157;
pub const IMAGE_ASCII55: C2RustUnnamed = 156;
pub const IMAGE_ASCII54: C2RustUnnamed = 155;
pub const IMAGE_ASCII53: C2RustUnnamed = 154;
pub const IMAGE_ASCII52: C2RustUnnamed = 153;
pub const IMAGE_ASCII51: C2RustUnnamed = 152;
pub const IMAGE_ASCII50: C2RustUnnamed = 151;
pub const IMAGE_ASCII49: C2RustUnnamed = 150;
pub const IMAGE_ASCII48: C2RustUnnamed = 149;
pub const IMAGE_ASCII47: C2RustUnnamed = 148;
pub const IMAGE_ASCII46: C2RustUnnamed = 147;
pub const IMAGE_ASCII45: C2RustUnnamed = 146;
pub const IMAGE_ASCII44: C2RustUnnamed = 145;
pub const IMAGE_ASCII43: C2RustUnnamed = 144;
pub const IMAGE_ASCII42: C2RustUnnamed = 143;
pub const IMAGE_ASCII41: C2RustUnnamed = 142;
pub const IMAGE_ASCII40: C2RustUnnamed = 141;
pub const IMAGE_ASCII39: C2RustUnnamed = 140;
pub const IMAGE_ASCII38: C2RustUnnamed = 139;
pub const IMAGE_ASCII37: C2RustUnnamed = 138;
pub const IMAGE_ASCII36: C2RustUnnamed = 137;
pub const IMAGE_ASCII35: C2RustUnnamed = 136;
pub const IMAGE_ASCII34: C2RustUnnamed = 135;
pub const IMAGE_ASCII33: C2RustUnnamed = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed = 124;
pub const IMAGE_9: C2RustUnnamed = 123;
pub const IMAGE_8: C2RustUnnamed = 122;
pub const IMAGE_7: C2RustUnnamed = 121;
pub const IMAGE_6: C2RustUnnamed = 120;
pub const IMAGE_5: C2RustUnnamed = 119;
pub const IMAGE_4: C2RustUnnamed = 118;
pub const IMAGE_3: C2RustUnnamed = 117;
pub const IMAGE_2: C2RustUnnamed = 116;
pub const IMAGE_1: C2RustUnnamed = 115;
pub const IMAGE_0: C2RustUnnamed = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed = 108;
pub const IMAGE_ECM: C2RustUnnamed = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed = 105;
pub const IMAGE_CANNON: C2RustUnnamed = 104;
pub const IMAGE_ROCKET: C2RustUnnamed = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed = 44;
pub const IMAGE_CLOSE: C2RustUnnamed = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed = 14;
pub const IMAGE_TAB4: C2RustUnnamed = 13;
pub const IMAGE_TAB3: C2RustUnnamed = 12;
pub const IMAGE_TAB2: C2RustUnnamed = 11;
pub const IMAGE_TAB1: C2RustUnnamed = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BUTTON_SURFACE {
    pub Buffer: *mut uint8,
    pub Surface: *mut iSurface,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RENDERED_BUTTON {
    pub InUse: BOOL,
    pub Initialised: BOOL,
    pub ImdRotation: SDWORD,
    pub State: UDWORD,
    pub Data: *mut libc::c_void,
    pub Data2: *mut libc::c_void,
    pub ButSurf: *mut BUTTON_SURFACE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CURSORSNAP {
    pub MaxSnaps: UWORD,
    pub NumSnaps: SWORD,
    pub CurrentSnap: SWORD,
    pub NewCurrentFormID: UDWORD,
    pub NewCurrentID: UDWORD,
    pub SnapCoords: *mut SNAPCOORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPCOORD {
    pub FormID: UDWORD,
    pub ID: UDWORD,
    pub SnapX: SWORD,
    pub SnapY: SWORD,
    pub Bias: *mut SNAPBIAS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SNAPBIAS {
    pub NDxBias: SWORD,
    pub NDyBias: SWORD,
    pub UDxBias: SWORD,
    pub UDyBias: SWORD,
    pub RDxBias: SWORD,
    pub RDyBias: SWORD,
    pub DDxBias: SWORD,
    pub DDyBias: SWORD,
    pub LDxBias: SWORD,
    pub LDyBias: SWORD,
}
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
/*
 * Power.h
 *
 * Definitions for the Power Functionality.
 *
 */
// free power on collection of oildrum.
//% used to determine the power cost of repairing a droid
                                         //definately DON'T WANT the brackets round 1/2 - it will equate to zero!
//used to multiply all repair calculations by to avaoid rounding errors
//OLD PLAYER_POWER
//typedef struct _player_power
//{
//	UDWORD		availablePower;		/* quantity that can be used from the Generators */
//	UDWORD		extractedPower;		/* quantity being extracted but not converted 
//									   by a Generator */
//	SDWORD		capacity;			/* the spare capacity of the generators */
//	SDWORD		usedPower;			/* quantity currently being used */
//} PLAYER_POWER;
//NEW PLAYER_POWER
pub type PLAYER_POWER = _player_power;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
// Miscellaneous strings
pub const STR_MISC_CLOSE: _fixed_str_id = 9;
pub const STR_INT_TRANSLAUNCH: _fixed_str_id = 51;
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub const DACTION_TRANSPORTOUT: _droid_action = 11;
pub type DROID_GROUP = _droid_group;
pub const DORDER_NONE: _droid_order = 0;
pub const DORDER_DISEMBARK: _droid_order = 17;
pub const DORDER_MOVE: _droid_order = 2;
pub const DORDER_TRANSPORTRETURN: _droid_order = 24;
pub const DORDER_TRANSPORTIN: _droid_order = 23;
pub const DORDER_TRANSPORTOUT: _droid_order = 22;
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
pub const DORDER_RECYCLE: _droid_order = 21;
pub const DORDER_BUILDMODULE: _droid_order = 20;
pub const DORDER_COMMAND: _droid_order = 19;
pub const DORDER_ATTACKTARGET: _droid_order = 18;
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
pub const DORDER_STOP: _droid_order = 1;
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
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_0 = 119;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_0 = 285;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_0 = 282;
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
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_0 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_0 = 283;
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
/* the widget screen */
/* Static variables */
//static	UDWORD			transID;
static mut psCurrTransporter: *mut DROID = 0 as *const DROID as *mut DROID;
static mut g_psCurScriptTransporter: *mut DROID =
    0 as *const DROID as *mut DROID;
static mut onMission: BOOL = 0;
static mut g_iLaunchTime: UDWORD = 0 as libc::c_int as UDWORD;
//used for audio message for reinforcements
static mut bFirstTransporter: BOOL = 0;
//the tab positions of the DroidsAvail window
static mut objMajor: UWORD = 0 as libc::c_int as UWORD;
static mut objMinor: UWORD = 0 as libc::c_int as UWORD;
#[no_mangle]
pub unsafe extern "C" fn GetCurrTransporter() -> *mut DROID {
    return psCurrTransporter;
}
//The Transporter capacity label
//initialises Transporter variables
//initialises Transporter variables
#[no_mangle]
pub unsafe extern "C" fn initTransporters() {
    onMission = 0 as libc::c_int;
    psCurrTransporter = 0 as *mut DROID;
}
// Refresh the transporter screen.
// Call to refresh the transporter screen, ie when a droids boards it.
//
#[no_mangle]
pub unsafe extern "C" fn intRefreshTransporter() -> BOOL {
    //printf("intRefreshTransporter\n");
    return _intRefreshTransporter();
}
unsafe extern "C" fn _intRefreshTransporter() -> BOOL {
    // Is the transporter screen up?
    if intMode as libc::c_uint ==
           INT_TRANSPORTER as libc::c_int as libc::c_uint &&
           !widgGetFromID(psWScreen, 9000 as libc::c_int as UDWORD).is_null()
       {
        let mut Ret: BOOL = 0;
        // Refresh it by re-adding it.
        Ret = intAddTransporter(psCurrTransporter, onMission);
        intMode = INT_TRANSPORTER;
        return Ret
    }
    return 1 as libc::c_int;
}
/*Add the Transporter Interface*/
#[no_mangle]
pub unsafe extern "C" fn intAddTransporter(mut psSelected: *mut DROID,
                                           mut offWorld: BOOL) -> BOOL {
    return _intAddTransporter(psSelected, offWorld);
}
/*Add the Transporter Interface*/
unsafe extern "C" fn _intAddTransporter(mut psSelected: *mut DROID,
                                        mut offWorld: BOOL) -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut Animate: BOOL = 1 as libc::c_int;
    onMission = offWorld;
    psCurrTransporter = psSelected;
    /*if transporter has died - close the interface - this can only happen in
    multiPlayer where the transporter can be killed*/
    if bMultiPlayer != 0 {
        if !psCurrTransporter.is_null() && (*psCurrTransporter).died != 0 &&
               (*psCurrTransporter).died != 1 as libc::c_int as libc::c_uint {
            intRemoveTransNoAnim();
            return 1 as libc::c_int
        }
    }
    // Add the main Transporter form
	// Is the form already up?
    if !widgGetFromID(psWScreen, 9000 as libc::c_int as UDWORD).is_null() {
        intRemoveTransNoAnim();
        Animate = 0 as libc::c_int
    }
    if intIsRefreshing() != 0 { Animate = 0 as libc::c_int }
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 9000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint))
            as SWORD;
    sFormInit.y =
        (324 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
            as SWORD;
    sFormInit.width = 320 as libc::c_int as UWORD;
    sFormInit.height = 115 as libc::c_int as UWORD;
    // If the window was closed then do open animation.
    if Animate != 0 {
        sFormInit.pDisplay =
            Some(intOpenPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.disableChildren = 1 as libc::c_int
    } else {
        // otherwise just recreate it.
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the close button */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 9000 as libc::c_int as UDWORD;
    sButInit.id = 9002 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (320 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 0 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_CLOSEHILIGHT as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_CLOSE as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    if intAddTransButtonForm() == 0 { return 0 as libc::c_int }
    // Add the Transporter Contents form (and buttons)
    if intAddTransporterContents() == 0 { return 0 as libc::c_int }
    //if on a mission - add the Droids back at home base form
    if onMission != 0 {
        if intAddDroidsAvailForm() == 0 { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
/* *********TEST************/
//static  UDWORD      addCount = 0;
//static  UDWORD      removeCount = 0;
/*functions */
// Add the main Transporter Contents Interface
unsafe extern "C" fn intAddTransporterContents() -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut sButFInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut Animate: BOOL = 1 as libc::c_int;
    let mut AlreadyUp: BOOL = 0 as libc::c_int;
    // Is the form already up?
    if !widgGetFromID(psWScreen, 9003 as libc::c_int as UDWORD).is_null() {
        intRemoveTransContentNoAnim();
        Animate = 0 as libc::c_int;
        AlreadyUp = 1 as libc::c_int
    }
    if intIsRefreshing() != 0 { Animate = 0 as libc::c_int }
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 9003 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 23 as libc::c_int as SWORD;
    sFormInit.y =
        (45 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
            as SWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
    // If the window was closed then do open animation.
    if Animate != 0 {
        sFormInit.pDisplay =
            Some(intOpenPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.disableChildren = 1 as libc::c_int
    } else {
        // otherwise just recreate it.
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the close button */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 9003 as libc::c_int as UDWORD;
    sButInit.id = 9005 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (132 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 0 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_CLOSEHILIGHT as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_CLOSE as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    //	Now done further down.
//	if (!intAddTransContentsForm())
//	{
//		return FALSE;
//	}
    //add the capacity label - if not yet on the mission
	/*if (!onMission)
	{
		memset(&sLabInit,0,sizeof(W_LABINIT));
		sLabInit.formID = IDTRANS_CONTENTFORM;
		sLabInit.id = IDTRANS_CAPACITY;
		sLabInit.style = WLAB_PLAIN | WIDG_HIDDEN;
	#ifndef PSX
		sLabInit.x = STAT_SLDX + STAT_SLDWIDTH + 4;
		sLabInit.y = STAT_SLDY + 3;
	#else
		sLabInit.x = STAT_SLDX + STAT_SLDWIDTH + 4;
		sLabInit.y = STAT_SLDY - 4;
	#endif
		sLabInit.width = 16;
		sLabInit.height = 16;
		sLabInit.pText = "10";
		sLabInit.FontID = WFont;
		sLabInit.pCallback = intUpdateTransCapacity;
		if (!widgAddLabel(psWScreen, &sLabInit))
		{
			return FALSE;
		}
	}*/
    //add the Launch button - if on a mission, or all the time on the PSX
    if onMission != 0 {
        memset(&mut sButFInit as *mut W_FORMINIT as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
        sButFInit.formID = 9003 as libc::c_int as UDWORD;
        sButFInit.id = 9010 as libc::c_int as UDWORD;
        //		sButFInit.style = WBUT_PLAIN;
        sButFInit.style = (4 as libc::c_int | 8 as libc::c_int) as UDWORD;
        sButFInit.x = 2 as libc::c_int as SWORD;
        sButFInit.y = (4 as libc::c_int - 1 as libc::c_int) as UWORD as SWORD;
        sButFInit.width =
            iV_GetImageWidth(IntImages,
                             IMAGE_LAUNCHUP as libc::c_int as UWORD);
        sButFInit.height =
            iV_GetImageHeight(IntImages,
                              IMAGE_LAUNCHUP as libc::c_int as UWORD);
        sButFInit.pTip =
            strresGetString(psStringRes,
                            STR_INT_TRANSLAUNCH as libc::c_int as UDWORD);
        //sButInit.pText = "Launch";
//		sButFInit.FontID = WFont;
        sButFInit.pDisplay =
            Some(intDisplayImageHilight as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sButFInit.pUserData =
            ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
                 (IMAGE_LAUNCHDOWN as libc::c_int & 0x3ff as libc::c_int) <<
                     10 as libc::c_int |
                 IMAGE_LAUNCHUP as libc::c_int & 0x3ff as libc::c_int) as
                *mut libc::c_void;
        if widgAddForm(psWScreen, &mut sButFInit) == 0 {
            return 0 as libc::c_int
        }
        if AlreadyUp == 0 {
            intSetCurrentCursorPosition(&mut InterfaceSnap, sButFInit.id);
        }
    }
    if intAddTransContentsForm() == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*This is used to display the transporter button and capacity when at the home base ONLY*/
/*This is used to display the transporter button and capacity when at the home base ONLY*/
#[no_mangle]
pub unsafe extern "C" fn intAddTransporterLaunch(mut psDroid: *mut DROID)
 -> BOOL {
    //W_BUTINIT		sButInit;
    let mut sButInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,}; //needs to be a clickable form now
    let mut sLabInit: W_LABINIT =
        W_LABINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut capacity: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if bMultiPlayer != 0 { return 1 as libc::c_int }
    //do this first so that if the interface is already up it syncs with this transporter
	//set up the static transporter
    psCurrTransporter = psDroid;
    //check the button is not already up
    if !widgGetFromID(psWScreen, 9010 as libc::c_int as UDWORD).is_null() {
        return 1 as libc::c_int
    }
    memset(&mut sButInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sButInit.formID = 0 as libc::c_int as UDWORD;
    sButInit.id = 9010 as libc::c_int as UDWORD;
    sButInit.style = (4 as libc::c_int | 8 as libc::c_int) as UDWORD;
    sButInit.x = 23 as libc::c_int as SWORD;
    sButInit.y = 22 as libc::c_int as SWORD;
    sButInit.width =
        (10 as libc::c_int +
             iV_GetImageWidth(IntImages,
                              IMAGE_LAUNCHUP as libc::c_int as UWORD) as
                 libc::c_int) as UWORD;
    sButInit.height =
        iV_GetImageHeight(IntImages, IMAGE_LAUNCHUP as libc::c_int as UWORD);
    sButInit.pTip =
        strresGetString(psStringRes,
                        STR_INT_TRANSLAUNCH as libc::c_int as UDWORD);
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_LAUNCHDOWN as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_LAUNCHUP as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddForm(psWScreen, &mut sButInit) == 0 { return 0 as libc::c_int }
    //add the capacity label
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = 9010 as libc::c_int as UDWORD;
    sLabInit.id = 9500 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = (sButInit.x as libc::c_int + 20 as libc::c_int) as SWORD;
    sLabInit.y = 0 as libc::c_int as SWORD;
    sLabInit.width = 16 as libc::c_int as UWORD;
    sLabInit.height = 16 as libc::c_int as UWORD;
    sLabInit.pText =
        b"00/10\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInit.FontID = WFont;
    sLabInit.pCallback =
        Some(intUpdateTransCapacity as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    //when full flash the transporter button
    if !psCurrTransporter.is_null() && !(*psCurrTransporter).psGroup.is_null()
       {
        capacity = 10 as libc::c_int as UDWORD;
        psCurr = (*(*psCurrTransporter).psGroup).psList;
        while !psCurr.is_null() {
            psNext = (*psCurr).psGrpNext;
            if psCurr != psCurrTransporter {
                capacity =
                    (capacity as
                         libc::c_uint).wrapping_sub(transporterSpaceRequired(psCurr))
                        as UDWORD as UDWORD
            }
            psCurr = psNext
        }
        if capacity <= 0 as libc::c_int as libc::c_uint {
            flashMissionButton(9010 as libc::c_int as UDWORD);
        }
    }
    return 1 as libc::c_int;
}
/* Remove the Transporter Launch widget from the screen*/
/* Remove the Transporter Launch widget from the screen*/
#[no_mangle]
pub unsafe extern "C" fn intRemoveTransporterLaunch() {
    if !widgGetFromID(psWScreen, 9010 as libc::c_int as UDWORD).is_null() {
        widgDelete(psWScreen, 9010 as libc::c_int as UDWORD);
    };
}
/* Add the Transporter Button form */
unsafe extern "C" fn intAddTransButtonForm() -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sBFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sBFormInit2: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut numButtons: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut BufferID: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    /* Add the button form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 9000 as libc::c_int as UDWORD;
    sFormInit.id = 9001 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.width = 316 as libc::c_int as UWORD;
    sFormInit.height = 112 as libc::c_int as UWORD;
    sFormInit.x = 2 as libc::c_int as SWORD;
    sFormInit.y = 6 as libc::c_int as SWORD;
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    numButtons = 0 as libc::c_int as UDWORD;
    /*work out the number of buttons */
    psDroid = transInterfaceDroidList();
    while !psDroid.is_null() {
        //only interested in Transporter droids
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               ((*psDroid).action != DACTION_TRANSPORTOUT as libc::c_int &&
                    (*psDroid).action != DACTION_TRANSPORTIN as libc::c_int) {
            //set the first Transporter to be the current one if not already set
            if psCurrTransporter.is_null() { psCurrTransporter = psDroid }
            numButtons = numButtons.wrapping_add(1)
        }
        psDroid = (*psDroid).psNext
    }
    //set the number of tabs required
    sFormInit.numMajor =
        numForms(((60 as libc::c_int + 2 as libc::c_int) as
                      libc::c_uint).wrapping_mul(numButtons),
                 (316 as libc::c_int - 2 as libc::c_int) as UDWORD);
    //set minor tabs to 1
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        &mut StandardTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the transporter and status buttons */
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    memset(&mut sBFormInit2 as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 9001 as libc::c_int as UDWORD;
    sBFormInit.id = 9100 as libc::c_int as UDWORD;
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.style = 4 as libc::c_int as UDWORD;
    sBFormInit.x = 2 as libc::c_int as SWORD;
    sBFormInit.y = 42 as libc::c_int as SWORD;
    sBFormInit.width = 60 as libc::c_int as UWORD;
    sBFormInit.height = 46 as libc::c_int as UWORD;
    memcpy(&mut sBFormInit2 as *mut W_FORMINIT as *mut libc::c_void,
           &mut sBFormInit as *mut W_FORMINIT as *const libc::c_void,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit2.id = 9200 as libc::c_int as UDWORD;
    sBFormInit2.y = 0 as libc::c_int as SWORD;
    ClearObjectBuffers();
    ClearTopicBuffers();
    //add each button
	//transID = 0;
    psDroid = transInterfaceDroidList();
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               ((*psDroid).action != DACTION_TRANSPORTOUT as libc::c_int &&
                    (*psDroid).action != DACTION_TRANSPORTIN as libc::c_int) {
            /* Set the tip and add the button */
//			sBFormInit.pTip = psDroid->pName;
            sBFormInit.pTip = droidGetName(psDroid);
            BufferID =
                sBFormInit.id.wrapping_sub(9100 as libc::c_int as
                                               libc::c_uint) as SDWORD;
            if BufferID < 5 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"BufferID > NUM_TOPICBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID < 5 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 650 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTransButtonForm\x00")).as_ptr(),
                      b"BufferID < NUM_TOPICBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            ClearTopicButtonBuffer(BufferID);
            TopicBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            TopicBuffers[BufferID as usize].Data =
                psDroid as *mut libc::c_void;
            sBFormInit.pUserData =
                &mut *TopicBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit.pDisplay =
                Some(intDisplayObjectButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
                return 0 as libc::c_int
            }
            /* if the current droid matches psCurrTransporter lock the button */
            if psDroid == psCurrTransporter {
                //transID = sBFormInit.id;
				//widgSetButtonState(psWScreen, transID, WBUT_LOCK);
                widgSetButtonState(psWScreen, sBFormInit.id,
                                   0x2 as libc::c_int as UDWORD);
                widgSetTabs(psWScreen, 9001 as libc::c_int as UDWORD,
                            sBFormInit.majorID, 0 as libc::c_int as UWORD);
            }
            //now do status button
            sBFormInit2.pTip = 0 as *mut STRING;
            BufferID =
                sBFormInit2.id.wrapping_sub(9200 as libc::c_int as
                                                libc::c_uint).wrapping_mul(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_add(1
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint)
                    as SDWORD;
            if BufferID < 10 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"BufferID > NUM_OBJECTBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID < 10 as libc::c_int * 4 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 676 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTransButtonForm\x00")).as_ptr(),
                      b"BufferID < NUM_OBJECTBUFFERS\x00" as *const u8 as
                          *const libc::c_char);
            };
            ClearObjectButtonBuffer(BufferID);
            ObjectBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            sBFormInit2.pUserData =
                &mut *ObjectBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit2.pDisplay =
                Some(intDisplayStatusButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit2) == 0 {
                return 0 as libc::c_int
            }
            /* Update the init struct for the next buttons */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit.id < 9199 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many Transporter buttons\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBFormInit.id < 9199 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 690 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTransButtonForm\x00")).as_ptr(),
                      b"sBFormInit.id < IDTRANS_END\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBFormInit.x =
                (sBFormInit.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 316 as libc::c_int {
                sBFormInit.x = 2 as libc::c_int as SWORD;
                sBFormInit.majorID =
                    (sBFormInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
            sBFormInit2.id =
                (sBFormInit2.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit2.id < 9299 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many Transporter status buttons\x00" as *const u8
                          as *const libc::c_char);
            };
            if sBFormInit2.id < 9299 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 700 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddTransButtonForm\x00")).as_ptr(),
                      b"sBFormInit2.id < IDTRANS_STATEND\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBFormInit2.x =
                (sBFormInit2.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit2.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 316 as libc::c_int {
                sBFormInit2.x = 2 as libc::c_int as SWORD;
                sBFormInit2.majorID =
                    (sBFormInit2.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
        }
        psDroid = (*psDroid).psNext
    }
    return 1 as libc::c_int;
}
/* Add the Transporter Contents form */
unsafe extern "C" fn intAddTransContentsForm() -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sBFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut numButtons: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut BufferID: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    /* Add the contents form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 9003 as libc::c_int as UDWORD;
    sFormInit.id = 9004 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
    sFormInit.x = 0 as libc::c_int as SWORD;
    sFormInit.y = 18 as libc::c_int as SWORD;
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = 26 as libc::c_int as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    numButtons = 10 as libc::c_int as UDWORD;
    //set the number of tabs required
	//sFormInit.numMajor = numForms((OBJ_BUTWIDTH + OBJ_GAP) * numButtons,
	//							  OBJ_WIDTH - OBJ_GAP);
    sFormInit.numMajor = 1 as libc::c_int as UWORD;
    //set minor tabs to 1
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        &mut StandardTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the transporter contents buttons */
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 9004 as libc::c_int as UDWORD;
    sBFormInit.id = 9300 as libc::c_int as UDWORD;
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.style = 4 as libc::c_int as UDWORD;
    sBFormInit.x = 2 as libc::c_int as SWORD;
    sBFormInit.y =
        (42 as libc::c_int - 46 as libc::c_int - 2 as libc::c_int) as SWORD;
    sBFormInit.width = 60 as libc::c_int as UWORD;
    sBFormInit.height = 46 as libc::c_int as UWORD;
    ClearStatBuffers();
    //add each button
	//transID = 0;
    if !psCurrTransporter.is_null() {
        psDroid = (*(*psCurrTransporter).psGroup).psList;
        while !psDroid.is_null() && psDroid != psCurrTransporter {
            psNext = (*psDroid).psGrpNext;
            /* Set the tip and add the button */
//			sBFormInit.pTip = psDroid->pName;
            sBFormInit.pTip = droidGetName(psDroid);
            BufferID = GetStatBuffer();
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Unable to acquire stat buffer.\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 789 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"intAddTransContentsForm\x00")).as_ptr(),
                      b"BufferID >= 0\x00" as *const u8 as
                          *const libc::c_char);
            };
            StatBuffers[BufferID as usize].InUse = 1 as libc::c_int;
            StatBuffers[BufferID as usize].Data =
                psDroid as *mut libc::c_void;
            sBFormInit.pUserData =
                &mut *StatBuffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit.pDisplay =
                Some(intDisplayTransportButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
                return 0 as libc::c_int
            }
            //			intSetCurrentCursorPosition(&InterfaceSnap,sBFormInit.id);
            /* Update the init struct for the next button */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit.id < 9399 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many Transporter Droid buttons\x00" as *const u8
                          as *const libc::c_char);
            };
            if sBFormInit.id < 9399 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 803 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"intAddTransContentsForm\x00")).as_ptr(),
                      b"sBFormInit.id < IDTRANS_CONTEND\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBFormInit.x =
                (sBFormInit.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 132 as libc::c_int {
                sBFormInit.x = 2 as libc::c_int as SWORD;
                sBFormInit.y =
                    (sBFormInit.y as libc::c_int +
                         (46 as libc::c_int + 2 as libc::c_int)) as SWORD
            }
            if sBFormInit.y as libc::c_int + 46 as libc::c_int +
                   2 as libc::c_int > 273 as libc::c_int {
                sBFormInit.y = 42 as libc::c_int as SWORD;
                sBFormInit.majorID =
                    (sBFormInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
            psDroid = psNext
        }
    }
    return 1 as libc::c_int;
}
/* Add the Droids back at home form */
unsafe extern "C" fn intAddDroidsAvailForm() -> BOOL {
    let mut sFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sButInit: W_BUTINIT =
        W_BUTINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  pText: 0 as *mut STRING,
                  pTip: 0 as *mut STRING,
                  FontID: 0,};
    let mut sBFormInit: W_FORMINIT =
        W_FORMINIT{formID: 0,
                   majorID: 0,
                   minorID: 0,
                   id: 0,
                   style: 0,
                   x: 0,
                   y: 0,
                   width: 0,
                   height: 0,
                   pDisplay: None,
                   pCallback: None,
                   pUserData: 0 as *mut libc::c_void,
                   UserData: 0,
                   disableChildren: 0,
                   majorPos: 0,
                   minorPos: 0,
                   majorSize: 0,
                   minorSize: 0,
                   majorOffset: 0,
                   minorOffset: 0,
                   tabVertOffset: 0,
                   tabHorzOffset: 0,
                   tabMajorThickness: 0,
                   tabMinorThickness: 0,
                   tabMajorGap: 0,
                   tabMinorGap: 0,
                   numMajor: 0,
                   aNumMinors: [0; 9],
                   pTip: 0 as *mut STRING,
                   apMajorTips: [0 as *mut STRING; 9],
                   apMinorTips: [[0 as *mut STRING; 5]; 9],
                   pTabDisplay: None,
                   pFormDisplay: None,};
    let mut sBarInit: W_BARINIT =
        W_BARINIT{formID: 0,
                  majorID: 0,
                  minorID: 0,
                  id: 0,
                  style: 0,
                  x: 0,
                  y: 0,
                  width: 0,
                  height: 0,
                  pDisplay: None,
                  pCallback: None,
                  pUserData: 0 as *mut libc::c_void,
                  UserData: 0,
                  orientation: 0,
                  size: 0,
                  minorSize: 0,
                  iRange: 0,
                  sCol: W_COLOURDEF{red: 0, green: 0, blue: 0, alpha: 0,},
                  sMinorCol:
                      W_COLOURDEF{red: 0, green: 0, blue: 0, alpha: 0,},
                  pTip: 0 as *mut STRING,};
    //W_LABINIT		sLabInit;
    let mut numButtons: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut butPerForm: UDWORD = 0;
    let mut BufferID: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Animate: BOOL = 1 as libc::c_int;
    // Is the form already up?
    if !widgGetFromID(psWScreen, 9006 as libc::c_int as UDWORD).is_null() {
        intRemoveTransDroidsAvailNoAnim();
        Animate = 0 as libc::c_int
    }
    if intIsRefreshing() != 0 { Animate = 0 as libc::c_int }
    /* Add the droids available form */
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 9006 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
    sFormInit.x =
        ((23 as libc::c_int + 132 as libc::c_int + 6 as libc::c_int) as
             libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                  as
                                                                                                                  libc::c_int
                                                                                                                  as
                                                                                                                  libc::c_uint)).wrapping_add(320
                                                                                                                                                  as
                                                                                                                                                  libc::c_int
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint).wrapping_add(6
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                 as
                                                                                                                                                                                 libc::c_uint).wrapping_add(1
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_int
                                                                                                                                                                                                                as
                                                                                                                                                                                                                libc::c_uint).wrapping_add(pie_GetVideoBufferWidth().wrapping_sub(640
                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                      libc::c_int
                                                                                                                                                                                                                                                                                      as
                                                                                                                                                                                                                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                     libc::c_uint))
            as SWORD;
    sFormInit.y =
        (45 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint))
            as SWORD;
    // If the window was closed then do open animation.
    if Animate != 0 {
        sFormInit.pDisplay =
            Some(intOpenPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        sFormInit.disableChildren = 1 as libc::c_int
    } else {
        // otherwise just recreate it.
        sFormInit.pDisplay =
            Some(intDisplayPlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ())
    }
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the close button */
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 9006 as libc::c_int as UDWORD;
    sButInit.id = 9008 as libc::c_int as UDWORD;
    sButInit.style = 0 as libc::c_int as UDWORD;
    sButInit.x = (132 as libc::c_int - 15 as libc::c_int) as SWORD;
    sButInit.y = 0 as libc::c_int as SWORD;
    sButInit.width = 15 as libc::c_int as UWORD;
    sButInit.height = 15 as libc::c_int as UWORD;
    sButInit.pTip =
        strresGetString(psStringRes, STR_MISC_CLOSE as libc::c_int as UDWORD);
    sButInit.FontID = WFont;
    sButInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sButInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_CLOSEHILIGHT as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_CLOSE as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddButton(psWScreen, &mut sButInit) == 0 {
        return 0 as libc::c_int
    }
    //now add the tabbed droids available form
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 9006 as libc::c_int as UDWORD;
    sFormInit.id = 9007 as libc::c_int as UDWORD;
    sFormInit.style = 1 as libc::c_int as UDWORD;
    sFormInit.width = 132 as libc::c_int as UWORD;
    sFormInit.height = 273 as libc::c_int as UWORD;
    sFormInit.x = 0 as libc::c_int as SWORD;
    sFormInit.y = 18 as libc::c_int as SWORD;
    sFormInit.majorPos = 1 as libc::c_int as UWORD;
    sFormInit.minorPos = 0 as libc::c_int as UWORD;
    sFormInit.majorSize = (26 as libc::c_int / 2 as libc::c_int) as UWORD;
    sFormInit.majorOffset = 2 as libc::c_int as SWORD;
    sFormInit.tabVertOffset = (11 as libc::c_int / 2 as libc::c_int) as SWORD;
    sFormInit.tabMajorThickness = 11 as libc::c_int as UWORD;
    sFormInit.tabMajorGap = 2 as libc::c_int as UWORD;
    //calc num buttons
    numButtons = 0 as libc::c_int as UDWORD;
    //look through the list of droids that were built before the mission
    psDroid = mission.apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        //ignore any Transporters!
        if (*psDroid).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            numButtons = numButtons.wrapping_add(1)
        }
        //quit when reached max can cope with
        if numButtons == 80 as libc::c_int as libc::c_uint { break ; }
        psDroid = (*psDroid).psNext
    }
    butPerForm =
        ((132 as libc::c_int - 2 as libc::c_int) /
             (60 as libc::c_int + 2 as libc::c_int) *
             ((273 as libc::c_int - 2 as libc::c_int) /
                  (46 as libc::c_int + 2 as libc::c_int))) as UDWORD;
    sFormInit.numMajor = numForms(numButtons, butPerForm);
    //set minor tabs to 1
    i = 0 as libc::c_int as UDWORD;
    while i < sFormInit.numMajor as libc::c_uint {
        sFormInit.aNumMinors[i as usize] = 1 as libc::c_int as UWORD;
        i = i.wrapping_add(1)
    }
    sFormInit.pFormDisplay =
        Some(intDisplayObjectForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData = &mut SmallTab as *mut TABDEF as *mut libc::c_void;
    sFormInit.pTabDisplay =
        Some(intDisplayTab as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                      _: UDWORD, _: UDWORD, _: UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    /* Add the droids available buttons */
    memset(&mut sBFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sBFormInit.formID = 9007 as libc::c_int as UDWORD;
    sBFormInit.id = 9400 as libc::c_int as UDWORD;
    sBFormInit.majorID = 0 as libc::c_int as UWORD;
    sBFormInit.minorID = 0 as libc::c_int as UWORD;
    sBFormInit.style = 4 as libc::c_int as UDWORD;
    sBFormInit.x = 2 as libc::c_int as SWORD;
    sBFormInit.y = 0 as libc::c_int as SWORD;
    sBFormInit.width = 60 as libc::c_int as UWORD;
    sBFormInit.height = 46 as libc::c_int as UWORD;
    ClearSystem0Buffers();
    /* Add the state of repair bar for each droid*/
    memset(&mut sBarInit as *mut W_BARINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BARINIT>() as libc::c_ulong);
    sBarInit.id = 9600 as libc::c_int as UDWORD;
    sBarInit.style = 0 as libc::c_int as UDWORD;
    sBarInit.orientation = 0x1 as libc::c_int as UWORD;
    sBarInit.x = 3 as libc::c_int as SWORD;
    sBarInit.y =
        (46 as libc::c_int - 4 as libc::c_int - 3 as libc::c_int) as SWORD;
    sBarInit.width = (60 as libc::c_int - 8 as libc::c_int) as UWORD;
    sBarInit.height = 4 as libc::c_int as UWORD;
    sBarInit.size = 50 as libc::c_int as UWORD;
    sBarInit.sCol.red = 255 as libc::c_int as UBYTE;
    sBarInit.sCol.green = 235 as libc::c_int as UBYTE;
    sBarInit.sCol.blue = 19 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.red = 0x55 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.green = 0 as libc::c_int as UBYTE;
    sBarInit.sMinorCol.blue = 0 as libc::c_int as UBYTE;
    //add each button
	//add droids built whilst on the mission
	/*for (psDroid = mission.apsBuiltDroids[selectedPlayer]; psDroid != NULL;
		psDroid = psDroid->psNext)
	{
		//don't add Transporter Droids!
		if (psDroid->droidType != DROID_TRANSPORTER)
		{
			// Set the tip and add the button
			sBFormInit.pTip = psDroid->pName;
			BufferID = GetSystem0Buffer();
			ASSERT( BufferID >= 0,"Unable to acquire stat buffer." );
			RENDERBUTTON_INUSE(&System0Buffers[BufferID]);
			System0Buffers[BufferID].Data = (void*)psDroid;
			sBFormInit.pUserData = (void*)&System0Buffers[BufferID];
			sBFormInit.pDisplay = intDisplayTransportButton;

			if (!widgAddForm(psWScreen, &sBFormInit))
			{
				return FALSE;
			}

			// Update the init struct for the next button
			sBFormInit.id += 1;
			ASSERT( sBFormInit.id < IDTRANS_DROIDEND,"Too many Droids Built buttons" );

			sBFormInit.x += OBJ_BUTWIDTH + OBJ_GAP;
			if (sBFormInit.x + OBJ_BUTWIDTH + OBJ_GAP > TRANSDROID_TABWIDTH)
			{
				sBFormInit.x = OBJ_STARTX;
				sBFormInit.y += OBJ_BUTHEIGHT + OBJ_GAP;
			}

			if (sBFormInit.y + OBJ_BUTHEIGHT + OBJ_GAP > TRANSDROID_TABHEIGHT)
			{
				sBFormInit.y = OBJ_STARTY;
				sBFormInit.majorID += 1;
			}
		}
	}*/
	//add droids built before the mission
    psDroid = mission.apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        //stop adding the buttons once MAX_DROIDS has been reached
        if sBFormInit.id ==
               (9400 as libc::c_int + 80 as libc::c_int) as libc::c_uint {
            break ;
        }
        //don't add Transporter Droids!
        if (*psDroid).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            /* Set the tip and add the button */
//			sBFormInit.pTip = psDroid->pName;
            sBFormInit.pTip = droidGetName(psDroid);
            BufferID = GetSystem0Buffer();
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Unable to acquire stat buffer.\x00" as *const u8 as
                          *const libc::c_char);
            };
            if BufferID >= 0 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 1050 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddDroidsAvailForm\x00")).as_ptr(),
                      b"BufferID >= 0\x00" as *const u8 as
                          *const libc::c_char);
            };
            System0Buffers[BufferID as usize].InUse = 1 as libc::c_int;
            System0Buffers[BufferID as usize].Data =
                psDroid as *mut libc::c_void;
            sBFormInit.pUserData =
                &mut *System0Buffers.as_mut_ptr().offset(BufferID as isize) as
                    *mut RENDERED_BUTTON as *mut libc::c_void;
            sBFormInit.pDisplay =
                Some(intDisplayTransportButton as
                         unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                              _: UDWORD, _: *mut UDWORD)
                             -> ());
            if widgAddForm(psWScreen, &mut sBFormInit) == 0 {
                return 0 as libc::c_int
            }
            // Snap to the first button in the form.
            if sBFormInit.id == 9400 as libc::c_int as libc::c_uint {
                intSetCurrentCursorPosition(&mut InterfaceSnap,
                                            sBFormInit.id);
            }
            //add bar to indicate stare of repair
            sBarInit.size =
                (*psDroid).body.wrapping_mul(100 as libc::c_int as
                                                 libc::c_uint).wrapping_div((*psDroid).originalBody)
                    as UWORD;
            if sBarInit.size as libc::c_int > 100 as libc::c_int {
                sBarInit.size = 100 as libc::c_int as UWORD
            }
            sBarInit.formID = sBFormInit.id;
            //sBarInit.iRange = TBAR_MAX_REPAIR;
            if widgAddBarGraph(psWScreen, &mut sBarInit) == 0 {
                return 0 as libc::c_int
            }
            /* Update the init struct for the next button */
            sBFormInit.id =
                (sBFormInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if sBFormInit.id < 9499 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Too many Droids Built buttons\x00" as *const u8 as
                          *const libc::c_char);
            };
            if sBFormInit.id < 9499 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 1082 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 22],
                                                &[libc::c_char; 22]>(b"intAddDroidsAvailForm\x00")).as_ptr(),
                      b"sBFormInit.id < IDTRANS_DROIDEND\x00" as *const u8 as
                          *const libc::c_char);
            };
            sBFormInit.x =
                (sBFormInit.x as libc::c_int +
                     (60 as libc::c_int + 2 as libc::c_int)) as SWORD;
            if sBFormInit.x as libc::c_int + 60 as libc::c_int +
                   2 as libc::c_int > 132 as libc::c_int {
                sBFormInit.x = 2 as libc::c_int as SWORD;
                sBFormInit.y =
                    (sBFormInit.y as libc::c_int +
                         (46 as libc::c_int + 2 as libc::c_int)) as SWORD
            }
            if sBFormInit.y as libc::c_int + 46 as libc::c_int +
                   2 as libc::c_int > 273 as libc::c_int {
                sBFormInit.y = 0 as libc::c_int as SWORD;
                sBFormInit.majorID =
                    (sBFormInit.majorID as libc::c_int + 1 as libc::c_int) as
                        UWORD
            }
            //and bar
            sBarInit.id =
                (sBarInit.id as
                     libc::c_uint).wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as UDWORD as
                    UDWORD
        }
        psDroid = (*psDroid).psNext
    }
    //reset which tab we were on
    if objMajor as libc::c_int >
           (sFormInit.numMajor as libc::c_int - 1 as libc::c_int) as UWORD as
               libc::c_int {
        //set to last if have lost a tab
        widgSetTabs(psWScreen, 9007 as libc::c_int as UDWORD,
                    (sFormInit.numMajor as libc::c_int - 1 as libc::c_int) as
                        UWORD, objMinor);
    } else {
        //set to same tab we were on previously
        widgSetTabs(psWScreen, 9007 as libc::c_int as UDWORD, objMajor,
                    objMinor);
    }
    return 1 as libc::c_int;
}
/*calculates how much space is remaining on the transporter - allows droids to take
up different amount depending on their body size - currently all are set to one!*/
/*calculates how much space is remaining on the transporter - allows droids to take
up different amount depending on their body size - currently all are set to one!*/
#[no_mangle]
pub unsafe extern "C" fn calcRemainingCapacity(mut psTransporter: *mut DROID)
 -> UDWORD {
    let mut capacity: SDWORD = 10 as libc::c_int;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    // If it's dead, and it really is dead then just return 0.
    if (*psTransporter).died != 0 &&
           (*psTransporter).died != 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as UDWORD
    }
    psDroid = (*(*psTransporter).psGroup).psList;
    while !psDroid.is_null() && psDroid != psTransporter {
        psNext = (*psDroid).psGrpNext;
        capacity =
            (capacity as
                 libc::c_uint).wrapping_sub(transporterSpaceRequired(psDroid))
                as SDWORD as SDWORD;
        psDroid = psNext
    }
    if capacity < 0 as libc::c_int { capacity = 0 as libc::c_int }
    return capacity as UDWORD;
}
// Order all selected droids to embark all avaialable transporters.
// Order all selected droids to embark all avaialable transporters.
//
#[no_mangle]
pub unsafe extern "C" fn OrderDroidsToEmbark() -> BOOL {
    let mut NumTransporters: UWORD = 0 as libc::c_int as UWORD;
    let mut CurrentTransporter: UWORD = 0;
    let mut psTransporters: [*mut DROID; 8] = [0 as *mut DROID; 8];
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Ok_0: BOOL = 0 as libc::c_int;
    // First build a list of transporters.
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            psTransporters[NumTransporters as usize] = psDroid;
            NumTransporters = NumTransporters.wrapping_add(1);
            if NumTransporters as libc::c_int <= 8 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"MAX_TRANSPORTERS Exceeded\x00" as *const u8 as
                          *const libc::c_char);
            };
            if NumTransporters as libc::c_int <= 8 as libc::c_int {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 1162 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"OrderDroidsToEmbark\x00")).as_ptr(),
                      b"NumTransporters <= MAX_TRANSPORTERS\x00" as *const u8
                          as *const libc::c_char);
            };
        }
        psDroid = (*psDroid).psNext
    }
    // Now order any selected droids to embark them.
    if NumTransporters != 0 {
        CurrentTransporter = 0 as libc::c_int as UWORD;
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).selected as libc::c_int != 0 &&
                   (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                orderDroidObj(psDroid, DORDER_EMBARK,
                              psTransporters[CurrentTransporter as usize] as
                                  *mut BASE_OBJECT);
                // Step through the available transporters.
                CurrentTransporter = CurrentTransporter.wrapping_add(1);
                if CurrentTransporter as libc::c_int >=
                       NumTransporters as libc::c_int {
                    CurrentTransporter = 0 as libc::c_int as UWORD
                }
                Ok_0 = 1 as libc::c_int
            }
            psDroid = (*psDroid).psNext
        }
    }
    return 0 as libc::c_int;
}
// Order a single droid to embark any available transporters.
// Order a single droid to embark any available transporters.
//
#[no_mangle]
pub unsafe extern "C" fn OrderDroidToEmbark(mut psDroid: *mut DROID) -> BOOL {
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    psTransporter = FindATransporter();
    if !psTransporter.is_null() {
        orderDroidObj(psDroid, DORDER_EMBARK,
                      psTransporter as *mut BASE_OBJECT);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn intSetTransCapacityLabel(mut Label:
                                                      *mut libc::c_char) {
    let mut capacity: UDWORD = 10 as libc::c_int as UDWORD;
    if !psCurrTransporter.is_null() {
        capacity = calcRemainingCapacity(psCurrTransporter);
        //NOT ANY MORE!
		//Label->style &= ~WIDG_HIDDEN;
		//if nothing on the Transporter, need to remove the Launch Button
		/*if (capacity == TRANSPORTER_CAPACITY)
		{
			widgHide(psWScreen, IDTRANS_LAUNCH);
		}
		else
		{
			widgReveal(psWScreen, IDTRANS_LAUNCH);
		}*/
        capacity = (10 as libc::c_int as libc::c_uint).wrapping_sub(capacity);
        *Label.offset(0 as libc::c_int as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(capacity.wrapping_div(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as libc::c_char;
        *Label.offset(1 as libc::c_int as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(capacity.wrapping_rem(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as libc::c_char
    };
}
//		for (psDroid = psCurrTransporter->psGroup->psList; psDroid != NULL AND psDroid !=
//				; psDroid = psNext)
//		{
//			psNext = psDroid->psGrpNext;
//			//switch on body size
//			capacity -= transporterSpaceRequired(psDroid);
//		}
		//change round the way the remaining capacity is displayed - show 0/10 when empty now
/*updates the capacity of the current Transporter*/
#[no_mangle]
pub unsafe extern "C" fn intUpdateTransCapacity(mut psWidget: *mut _widget,
                                                mut psContext:
                                                    *mut _w_context) {
    //DROID		*psDroid, *psNext;
//	UDWORD		capacity = TRANSPORTER_CAPACITY;
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    intSetTransCapacityLabel((*Label).aText.as_mut_ptr());
    //	if (psCurrTransporter)
//	{
//		capacity = calcRemainingCapacity(psCurrTransporter);
// //		for (psDroid = psCurrTransporter->psGroup->psList; psDroid != NULL AND psDroid !=
// //				; psDroid = psNext)
// //		{
// //			psNext = psDroid->psGrpNext;
// //			//switch on body size
// //			capacity -= transporterSpaceRequired(psDroid);
// //		}
//		//change round the way the remaining capacity is displayed - show 0/10 when empty now
//		capacity = TRANSPORTER_CAPACITY - capacity;
//
//		Label->aText[0] = (UBYTE)('0'+capacity / 10);
//		Label->aText[1] = (UBYTE)('0'+capacity % 10);
//		//NOT ANY MORE!
//		//Label->style &= ~WIDG_HIDDEN;
//		//if nothing on the Transporter, need to remove the Launch Button
//		/*if (capacity == TRANSPORTER_CAPACITY)
//		{
//			widgHide(psWScreen, IDTRANS_LAUNCH);
//		}
//		else
//		{
//			widgReveal(psWScreen, IDTRANS_LAUNCH);
//		}*/
//	}
}
/* Process return codes from the Transporter Screen*/
/* Process return codes from the Transporter Screen*/
#[no_mangle]
pub unsafe extern "C" fn intProcessTransporter(mut id: UDWORD) {
    _intProcessTransporter(id);
}
unsafe extern "C" fn _intProcessTransporter(mut id: UDWORD) {
    if id >= 9100 as libc::c_int as libc::c_uint &&
           id <= 9199 as libc::c_int as libc::c_uint {
        /* A Transporter button has been pressed */
        setCurrentTransporter(id);
        /*refresh the Contents list */
        intAddTransporterContents();
    } else if id >= 9300 as libc::c_int as libc::c_uint &&
                  id <= 9399 as libc::c_int as libc::c_uint {
        //got to have a current transporter for this to work - and can't be flying
        if !psCurrTransporter.is_null() &&
               transporterFlying(psCurrTransporter) == 0 {
            transporterRemoveDroid(id);
            /*refresh the Contents list */
            intAddTransporterContents();
            if onMission != 0 {
                /*refresh the Avail list */
                intAddDroidsAvailForm();
            }
        }
    } else if id == 9002 as libc::c_int as libc::c_uint {
        intRemoveTransContent();
        intRemoveTrans();
        psCurrTransporter = 0 as *mut DROID
    } else if id == 9005 as libc::c_int as libc::c_uint {
        intRemoveTransContent();
    } else if id == 9008 as libc::c_int as libc::c_uint {
        intRemoveTransDroidsAvail();
    } else if id >= 9400 as libc::c_int as libc::c_uint &&
                  id <= 9499 as libc::c_int as libc::c_uint {
        //got to have a current transporter for this to work - and can't be flying
        if !psCurrTransporter.is_null() &&
               transporterFlying(psCurrTransporter) == 0 {
            intTransporterAddDroid(id);
            /*don't need to explicitly refresh here since intRefreshScreen()
            is called by intTransporterAddDroid()*/
			/*refresh the Contents list */
			//intAddTransporterContents();
			/*refresh the Avail list */
			//intAddDroidsAvailForm();
        }
    } else if id == 9001 as libc::c_int as libc::c_uint {
        // Process form tab clicks.
        //If tab clicked on Transporter screen then refresh rendered buttons.
        RefreshObjectButtons();
        RefreshTopicButtons();
    } else if id == 9004 as libc::c_int as libc::c_uint {
        //If tab clicked on Transporter Contents screen then refresh rendered buttons.
        RefreshStatsButtons();
    } else if id == 9007 as libc::c_int as libc::c_uint {
        //If tab clicked on Droids Available screen then refresh rendered buttons.
        RefreshSystem0Buttons();
    };
    /*
	else if (id == IDTRANS_LAUNCH)
	{
		processLaunchTransporter();
		//launch the Transporter
		if (psCurrTransporter)
		{
			launchTransporter(psCurrTransporter);
			//set the data for the transporter timer
			widgSetUserData(psWScreen, IDTRANTIMER_DISPLAY, (void*)psCurrTransporter);
		}
	}
	*/
}
/* Remove the Transporter widgets from the screen */
/* Remove the Transporter widgets from the screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveTrans() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 9000 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    (*Form).display =
        Some(intClosePlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    (*Form).disableChildren = 1 as libc::c_int;
    (*Form).pUserData = 0 as *mut libc::c_void;
    ClosingTrans = 1 as libc::c_int;
    intRemoveTransContent();
    intRemoveTransDroidsAvail();
    intMode = INT_NORMAL;
}
/* Remove the Transporter Content widgets from the screen w/o animation!*/
#[no_mangle]
pub unsafe extern "C" fn intRemoveTransNoAnim() {
    //remove main screen
    widgDelete(psWScreen, 9000 as libc::c_int as UDWORD);
    intRemoveTransContentNoAnim();
    intRemoveTransDroidsAvailNoAnim();
    intMode = INT_NORMAL;
}
/* Remove the Transporter Content widgets from the screen */
#[no_mangle]
pub unsafe extern "C" fn intRemoveTransContent() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 9003 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).disableChildren = 1 as libc::c_int;
        (*Form).pUserData = 0 as *mut libc::c_void;
        ClosingTransCont = 1 as libc::c_int
    };
}
//static void intUpdateTransCapacity(struct _widget *psWidget, struct _w_context *psContext);
/* Remove the Transporter Content widgets from the screen w/o animation!*/
unsafe extern "C" fn intRemoveTransContentNoAnim() {
    //remove main screen
    widgDelete(psWScreen, 9003 as libc::c_int as UDWORD);
}
/* Remove the Transporter Droids Avail widgets from the screen */
unsafe extern "C" fn intRemoveTransDroidsAvail() {
    let mut Form: *mut W_TABFORM = 0 as *mut W_TABFORM;
    // Start the window close animation.
    Form =
        widgGetFromID(psWScreen, 9006 as libc::c_int as UDWORD) as
            *mut W_TABFORM; // Used to signal when the close anim has finished.
    if !Form.is_null() {
        (*Form).display =
            Some(intClosePlainForm as
                     unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                          _: UDWORD, _: *mut UDWORD) -> ());
        (*Form).disableChildren = 1 as libc::c_int;
        (*Form).pUserData = 0 as *mut libc::c_void;
        ClosingTransDroids = 1 as libc::c_int;
        //remember which tab we were on
        widgGetTabs(psWScreen, 9007 as libc::c_int as UDWORD, &mut objMajor,
                    &mut objMinor);
    };
}
/* Remove the Transporter Droids Avail widgets from the screen w/o animation!*/
unsafe extern "C" fn intRemoveTransDroidsAvailNoAnim() {
    if !widgGetFromID(psWScreen, 9006 as libc::c_int as UDWORD).is_null() {
        //remember which tab we were on
        widgGetTabs(psWScreen, 9007 as libc::c_int as UDWORD, &mut objMajor,
                    &mut objMinor);
        //remove main screen
        widgDelete(psWScreen, 9006 as libc::c_int as UDWORD);
    };
}
/*sets psCurrTransporter */
unsafe extern "C" fn setCurrentTransporter(mut id: UDWORD) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut currID: UDWORD = 0;
    psCurrTransporter = 0 as *mut DROID;
    currID = 9100 as libc::c_int as UDWORD;
    //loop thru all the droids to find the selected one
    psDroid = transInterfaceDroidList();
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
               ((*psDroid).action != DACTION_TRANSPORTOUT as libc::c_int &&
                    (*psDroid).action != DACTION_TRANSPORTIN as libc::c_int) {
            if currID == id { break ; }
            currID = currID.wrapping_add(1)
        }
        psDroid = (*psDroid).psNext
    }
    if !psDroid.is_null() {
        psCurrTransporter = psDroid;
        //set the data for the transporter timer
        widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD,
                        psCurrTransporter as *mut libc::c_void);
    };
}
/*removes a droid from the group associated with the transporter*/
unsafe extern "C" fn transporterRemoveDroid(mut id: UDWORD) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut currID: UDWORD = 0;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if !psCurrTransporter.is_null() {
    } else {
        debug(LOG_ERROR,
              b"transporterRemoveUnit:can\'t remove units\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psCurrTransporter.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1501 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"transporterRemoveDroid\x00")).as_ptr(),
              b"psCurrTransporter != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    currID = 9300 as libc::c_int as UDWORD;
    psDroid = (*(*psCurrTransporter).psGroup).psList;
    while !psDroid.is_null() && psDroid != psCurrTransporter {
        psNext = (*psDroid).psGrpNext;
        if currID == id { break ; }
        currID = currID.wrapping_add(1);
        psDroid = psNext
    }
    if !psDroid.is_null() {
        /*if we're offWorld we can't pick a tile without swapping the map
        pointers - can't be bothered so just do this...*/
        if onMission != 0 {
            (*psDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
            (*psDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD
        } else {
            if bMultiPlayer != 0 {
                //set the units next to the transporter's current location
                droidX =
                    ((*psCurrTransporter).x as libc::c_int >>
                         7 as libc::c_int) as UDWORD;
                droidY =
                    ((*psCurrTransporter).y as libc::c_int >>
                         7 as libc::c_int) as UDWORD
            } else {
                //pick a tile because save games won't remember where the droid was when it was loaded
                droidX =
                    (getLandingX(0 as libc::c_int) as libc::c_int >>
                         7 as libc::c_int) as UDWORD;
                droidY =
                    (getLandingY(0 as libc::c_int) as libc::c_int >>
                         7 as libc::c_int) as UDWORD
            }
            if pickATileGen(&mut droidX, &mut droidY,
                            20 as libc::c_int as UBYTE,
                            Some(zonedPAT as
                                     unsafe extern "C" fn(_: UDWORD,
                                                          _: UDWORD) -> BOOL))
                   == 0 {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"transporterRemoveUnit: Unable to find a valid location\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"transporter.c\x00" as *const u8 as
                              *const libc::c_char, 1539 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"transporterRemoveDroid\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            (*psDroid).x = (droidX << 7 as libc::c_int) as UWORD;
            (*psDroid).y = (droidY << 7 as libc::c_int) as UWORD;
            (*psDroid).z =
                map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                    UWORD;
            updateDroidOrientation(psDroid);
            //initialise the movement data
            initDroidMovement(psDroid);
            //reset droid orders
            orderDroid(psDroid, DORDER_STOP);
            gridAddObject(psDroid as *mut BASE_OBJECT);
            (*psDroid).cluster = 0 as libc::c_int as UBYTE
        }
        grpLeave((*psDroid).psGroup, psDroid);
        //add it back into apsDroidLists
        if onMission != 0 {
            //addDroid(psDroid, mission.apsBuiltDroids);
            addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
        } else { addDroid(psDroid, apsDroidLists.as_mut_ptr()); }
        if (*psDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint {
            if grpCreate(&mut psGroup) != 0 { grpJoin(psGroup, psDroid); }
        }
        (*psDroid).selected = 1 as libc::c_int as UBYTE;
        if calcRemainingCapacity(psCurrTransporter) != 0 {
            //make sure the button isn't flashing
            stopMissionButtonFlash(9010 as libc::c_int as UDWORD);
        }
    };
}
/*adds a droid to the current transporter via the interface*/
unsafe extern "C" fn intTransporterAddDroid(mut id: UDWORD) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut currID: UDWORD = 0;
    if !psCurrTransporter.is_null() {
    } else {
        debug(LOG_ERROR,
              b"intTransporterAddUnit:can\'t remove units\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psCurrTransporter.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1589 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"intTransporterAddDroid\x00")).as_ptr(),
              b"psCurrTransporter != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    currID = 9400 as libc::c_int as UDWORD;
    psDroid = transInterfaceDroidList();
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        if (*psDroid).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            if currID == id { break ; }
            currID = currID.wrapping_add(1)
        }
        psDroid = psNext
    }
    if !psDroid.is_null() {
        transporterAddDroid(psCurrTransporter, psDroid);
    };
}
/*Adds a droid to the transporter, removing it from the world*/
/*Adds a droid to the transporter, removing it from the world */
#[no_mangle]
pub unsafe extern "C" fn transporterAddDroid(mut psTransporter: *mut DROID,
                                             mut psDroidToAdd: *mut DROID) {
    let mut bDroidRemoved: BOOL = 0;
    /* check for space */
    if checkTransporterSpace(psTransporter, psDroidToAdd) == 0 { return }
    if onMission != 0 {
        bDroidRemoved =
            droidRemove(psDroidToAdd, mission.apsDroidLists.as_mut_ptr())
    } else {
        bDroidRemoved = droidRemove(psDroidToAdd, apsDroidLists.as_mut_ptr());
        //inform all other players
        if bMultiPlayer != 0 { sendDroidEmbark(psDroidToAdd); }
    }
    if bDroidRemoved != 0 {
        grpJoin((*psTransporter).psGroup, psDroidToAdd);
    };
    //this is called by droidRemove
	//intRefreshScreen();
}
/*check to see if the droid can fit on the Transporter - return TRUE if fits*/
/*check to see if the droid can fit on the Transporter - return TRUE if fits*/
#[no_mangle]
pub unsafe extern "C" fn checkTransporterSpace(mut psTransporter: *mut DROID,
                                               mut psAssigned: *mut DROID)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut capacity: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"checkTransporterSpace: Invalid droid pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1651 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"checkTransporterSpace\x00")).as_ptr(),
              b"PTRVALID(psTransporter, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"checkTransporterSpace: Invalid droid pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1653 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"checkTransporterSpace\x00")).as_ptr(),
              b"PTRVALID(psAssigned, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"checkTransporterSpace: Droid is not a Transporter\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1655 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"checkTransporterSpace\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    if !(*psTransporter).psGroup.is_null() {
    } else {
        debug(LOG_ERROR,
              b"checkTransporterSpace: tranporter doesn\'t have a group\x00"
                  as *const u8 as *const libc::c_char);
    };
    if !(*psTransporter).psGroup.is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1657 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"checkTransporterSpace\x00")).as_ptr(),
              b"psTransporter->psGroup != NULL\x00" as *const u8 as
                  *const libc::c_char);
    };
    //work out how much space is currently left
    capacity = 10 as libc::c_int as UDWORD;
    psDroid = (*(*psTransporter).psGroup).psList;
    while !psDroid.is_null() && psDroid != psTransporter {
        psNext = (*psDroid).psGrpNext;
        capacity =
            (capacity as
                 libc::c_uint).wrapping_sub(transporterSpaceRequired(psDroid))
                as UDWORD as UDWORD;
        psDroid = psNext
    }
    if capacity >= transporterSpaceRequired(psAssigned) {
        //when full flash the transporter button
        if capacity.wrapping_sub(transporterSpaceRequired(psAssigned)) ==
               0 as libc::c_int as libc::c_uint {
            flashMissionButton(9010 as libc::c_int as UDWORD);
        }
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*returns the space the droid occupies on a transporter based on the body size*/
unsafe extern "C" fn transporterSpaceRequired(mut psDroid: *mut DROID)
 -> UDWORD {
    let mut size: UDWORD = 0;
    match (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as libc::c_int as
                                                     usize].nStat as
                                   libc::c_int as isize)).size as libc::c_int
        {
        0 => { size = 1 as libc::c_int as UDWORD }
        1 => { size = 1 as libc::c_int as UDWORD }
        2 | 3 => { size = 1 as libc::c_int as UDWORD }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"transporterSpaceRequired: Unknown Droid size\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 1700 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"transporterSpaceRequired\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            size = 0 as libc::c_int as UDWORD
        }
    }
    return size;
}
/*sets which list of droids to use for the transporter interface*/
unsafe extern "C" fn transInterfaceDroidList() -> *mut DROID {
    if onMission != 0 {
        return mission.apsDroidLists[selectedPlayer as usize]
    } else { return apsDroidLists[selectedPlayer as usize] };
}
/* get time transporter launch button was pressed */
#[no_mangle]
pub unsafe extern "C" fn transporterGetLaunchTime() -> UDWORD {
    return g_iLaunchTime;
}
/*set the time for the Launch*/
#[no_mangle]
pub unsafe extern "C" fn transporterSetLaunchTime(mut time: UDWORD) {
    g_iLaunchTime = time;
}
/*launches the defined transporter to the offworld map*/
/*launches the defined transporter to the offworld map*/
#[no_mangle]
pub unsafe extern "C" fn launchTransporter(mut psTransporter: *mut DROID)
 -> BOOL {
    let mut iX: UWORD = 0;
    let mut iY: UWORD = 0;
    //close the interface
    intResetScreen(1 as libc::c_int);
    // Hmmm...Only do this if were at our home base about to go off world.
//	//deselect all droids/structs etc
//	clearSelection();
    //this launches the mission if on homebase when the button is pressed
    if onMission == 0 {
        //deselect all droids/structs etc
		//clearSelection(); - we're deselecting 3 lines below!?
        //automatic from the script call to startMission now AB 12/05/97
		//launchMission();
        //tell the transporter to move to the new offworld location
        missionGetTransporterExit((*psTransporter).player as SDWORD, &mut iX,
                                  &mut iY);
        orderDroidLoc(psTransporter, DORDER_TRANSPORTOUT, iX as UDWORD,
                      iY as UDWORD);
        //g_iLaunchTime = gameTime;
        transporterSetLaunchTime(gameTime);
    } else {
        //otherwise just launches the Transporter
        if (*psTransporter).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"launchTransporter: Invalid Transporter Droid\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"transporter.c\x00" as *const u8 as
                          *const libc::c_char, 1783 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"launchTransporter\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        //remove out of stored list and add to current Droid list
		//removeDroid(psTransporter, mission.apsDroidLists);
		//addDroid(psTransporter, apsDroidLists);
		//need to put the Transporter down at a specified location
		//psTransporter->x = getLandingX(psTransporter->player);
		//psTransporter->y = getLandingY(psTransporter->player);
		//unloadTransporter(psTransporter, psTransporter->x, psTransporter->y, FALSE);
        orderDroid(psTransporter, DORDER_TRANSPORTIN);
        /* set action transporter waits for timer */
        actionDroid(psTransporter, DACTION_TRANSPORTWAITTOFLYIN);
        missionSetReinforcementTime(gameTime);
    }
    return 1 as libc::c_int;
}
/*checks how long the transporter has been travelling to see if it should
have arrived - returns TRUE when there*/
/*checks how long the transporter has been travelling to see if it should
have arrived - returns TRUE when there*/
#[no_mangle]
pub unsafe extern "C" fn updateTransporter(mut psTransporter: *mut DROID)
 -> BOOL {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"updateTransporter: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              1812 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"updateTransporter\x00")).as_ptr(),
              b"PTRVALID(psTransporter, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"updateTransporter: Invalid droid type\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"transporter.c\x00" as *const u8 as *const libc::c_char,
                  1817 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"updateTransporter\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 1 as libc::c_int
    }
    //if not moving to mission site, exit
    if (*psTransporter).action != DACTION_TRANSPORTOUT as libc::c_int &&
           (*psTransporter).action != DACTION_TRANSPORTIN as libc::c_int {
        return 1 as libc::c_int
    }
    /*if the transporter (selectedPlayer only) is moving droids to safety and
    all remaining droids are destoyed then we need to flag the end of mission
    as long as we're not flying out*/
    if (*psTransporter).player as libc::c_uint == selectedPlayer &&
           getDroidsToSafetyFlag() != 0 &&
           (*psTransporter).action != DACTION_TRANSPORTOUT as libc::c_int {
        //if there aren't any droids left...
        if missionDroidsRemaining(selectedPlayer) == 0 {
            // Set the Transporter to have arrived at its destination
            (*psTransporter).action = DACTION_NONE as libc::c_int;
            //the script can call startMission for this callback for offworld missions
            eventFireCallbackTrigger(CALL_START_NEXT_LEVEL as libc::c_int as
                                         TRIGGER_TYPE);
            // clear order
            (*psTransporter).order = DORDER_NONE as libc::c_int;
            (*psTransporter).psTarget = 0 as *mut _base_object;
            (*psTransporter).psTarStats = 0 as *mut _base_stats;
            return 1 as libc::c_int
        }
    }
    // moving to a location
    // if we're coming back for more droids then we want the transporter to
    // fly to edge of map before turning round again
    if (*psTransporter).sMove.Status as libc::c_int == 0 as libc::c_int ||
           (*psTransporter).sMove.Status as libc::c_int == 8 as libc::c_int ||
           (*psTransporter).action == DACTION_TRANSPORTOUT as libc::c_int &&
               missionIsOffworld() == 0 &&
               gameTime >
                   transporterGetLaunchTime().wrapping_add((4 as libc::c_int *
                                                                1000 as
                                                                    libc::c_int)
                                                               as
                                                               libc::c_uint)
               && getDroidsToSafetyFlag() == 0 {
        audio_StopObjTrack(psTransporter as *mut libc::c_void,
                           ID_SOUND_BLIMP_FLIGHT as libc::c_int);
        if (*psTransporter).action == DACTION_TRANSPORTIN as libc::c_int {
            /* !!!! GJ Hack - should be landing audio !!!! */
            audio_PlayObjDynamicTrack(psTransporter as *mut libc::c_void,
                                      ID_SOUND_BLIMP_TAKE_OFF as libc::c_int,
                                      None);
        }
        //DON@T PLAY AUDIO FOR THE FIRST TRANSPORTER LOAD...AB 9/2/99
        //show if selectedPlayer's transporter
		//if ( onMission && psTransporter->action == DACTION_TRANSPORTIN AND
        //    psTransporter->player == selectedPlayer)
        //changed onMission to missionForReInforcements() to cater for cam2A/cam3A - AB 4/2/99
        if bFirstTransporter == 0 && missionForReInforcements() != 0 &&
               (*psTransporter).action == DACTION_TRANSPORTIN as libc::c_int
               && (*psTransporter).player as libc::c_uint == selectedPlayer {
            //play reinforcements have arrived message
            audio_QueueTrackPos(ID_SOUND_TRANSPORT_LANDING as libc::c_int,
                                (*psTransporter).x as SDWORD,
                                (*psTransporter).y as SDWORD,
                                (*psTransporter).z as SDWORD);
            addConsoleMessage(strresGetString(psStringRes,
                                              STR_GAM_REINF as libc::c_int as
                                                  UDWORD), LEFT_JUSTIFY);
            //reset the data for the transporter timer
            widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD,
                            0 as *mut libc::c_void);
            return 1 as libc::c_int
        }
        // Got to destination
        (*psTransporter).action = DACTION_NONE as libc::c_int;
        //reset the flag to trigger the audio message
        bFirstTransporter = 0 as libc::c_int;
        return 1 as libc::c_int
    }
    //not arrived yet...
    return 0 as libc::c_int;
}
//process the launch transporter button click
//process the launch transporter button click
#[no_mangle]
pub unsafe extern "C" fn processLaunchTransporter() {
    let mut capacity: UDWORD = 10 as libc::c_int as UDWORD;
    let mut psForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    //launch the Transporter
    if !psCurrTransporter.is_null() {
        //check there is something on the transporter
        capacity = calcRemainingCapacity(psCurrTransporter);
        if capacity != 10 as libc::c_int as libc::c_uint {
            //make sure the button doesn't flash once launched
            stopMissionButtonFlash(9010 as libc::c_int as UDWORD);
            //disable the form so can't add any more droids into the transporter
            psForm =
                widgGetFromID(psWScreen, 9010 as libc::c_int as UDWORD) as
                    *mut W_CLICKFORM;
            if !psForm.is_null() {
                formSetClickState(psForm, 0x2 as libc::c_int as UDWORD);
            }
            //disable the form so can't add any more droids into the transporter
            psForm =
                widgGetFromID(psWScreen, 11012 as libc::c_int as UDWORD) as
                    *mut W_CLICKFORM;
            if !psForm.is_null() {
                formSetClickState(psForm, 0x2 as libc::c_int as UDWORD);
            }
            launchTransporter(psCurrTransporter);
            //set the data for the transporter timer
            widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD,
                            psCurrTransporter as *mut libc::c_void);
            eventFireCallbackTrigger(CALL_LAUNCH_TRANSPORTER as libc::c_int as
                                         TRIGGER_TYPE);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bobTransporterHeight() -> SDWORD {
    let mut val: SDWORD = 0;
    let mut angle: UDWORD = 0;
    // Because 4320/12 = 360 degrees
	// this gives us a bob frequency of 4.32 seconds.
	// we scale amplitude to 10 (world coordinate metric).
	// we need to use 360 degrees and not 180, as otherwise
	// it will not 'bounce' off the top _and_ bottom of
	// it's movemment arc.
    angle = gameTime.wrapping_rem(4320 as libc::c_int as libc::c_uint);
    val = angle.wrapping_div(12 as libc::c_int as libc::c_uint) as SDWORD;
    val =
        10 as libc::c_int *
            *aSinTable.as_mut_ptr().offset(((65536 as libc::c_int /
                                                 360 as libc::c_int * val) as
                                                uint16 as libc::c_int >>
                                                4 as libc::c_int) as isize);
    return val / 4096 as libc::c_int;
}
/*causes one of the mission buttons (Launch Button or Mission Timer) to start flashing*/
#[no_mangle]
pub unsafe extern "C" fn flashMissionButton(mut buttonID: UDWORD) {
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    //get the button from the id
    psForm = widgGetFromID(psWScreen, buttonID) as *mut W_TABFORM;
    if !psForm.is_null() {
        match buttonID {
            9010 => {
                (*psForm).pUserData =
                    ((1 as libc::c_int & 0x3ff as libc::c_int) <<
                         20 as libc::c_int |
                         (IMAGE_LAUNCHDOWN as libc::c_int &
                              0x3ff as libc::c_int) << 10 as libc::c_int |
                         IMAGE_LAUNCHUP as libc::c_int & 0x3ff as libc::c_int)
                        as *mut libc::c_void
            }
            11000 => {
                (*psForm).pUserData =
                    ((1 as libc::c_int & 0x3ff as libc::c_int) <<
                         20 as libc::c_int |
                         (IMAGE_MISSION_CLOCK as libc::c_int &
                              0x3ff as libc::c_int) << 10 as libc::c_int |
                         IMAGE_MISSION_CLOCK_UP as libc::c_int &
                             0x3ff as libc::c_int) as *mut libc::c_void
            }
            _ => {
                //do nothing other than in debug
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"flashMissionButton: Unknown button ID\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"transporter.c\x00" as *const u8 as
                              *const libc::c_char, 1978 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"flashMissionButton\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    };
}
/*stops one of the mission buttons (Launch Button or Mission Timer) flashing*/
#[no_mangle]
pub unsafe extern "C" fn stopMissionButtonFlash(mut buttonID: UDWORD) {
    let mut psForm: *mut W_TABFORM = 0 as *mut W_TABFORM;
    //get the button from the id
    psForm = widgGetFromID(psWScreen, buttonID) as *mut W_TABFORM;
    if !psForm.is_null() {
        match buttonID {
            9010 => {
                (*psForm).pUserData =
                    ((0 as libc::c_int & 0x3ff as libc::c_int) <<
                         20 as libc::c_int |
                         (IMAGE_LAUNCHDOWN as libc::c_int &
                              0x3ff as libc::c_int) << 10 as libc::c_int |
                         IMAGE_LAUNCHUP as libc::c_int & 0x3ff as libc::c_int)
                        as *mut libc::c_void
            }
            11000 => {
                (*psForm).pUserData =
                    ((0 as libc::c_int & 0x3ff as libc::c_int) <<
                         20 as libc::c_int |
                         (IMAGE_MISSION_CLOCK as libc::c_int &
                              0x3ff as libc::c_int) << 10 as libc::c_int |
                         IMAGE_MISSION_CLOCK_UP as libc::c_int &
                             0x3ff as libc::c_int) as *mut libc::c_void
            }
            _ => {
                //do nothing other than in debug
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"stopMissionButtonFlash: Unknown button ID\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"transporter.c\x00" as *const u8 as
                              *const libc::c_char, 2005 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 23],
                                                    &[libc::c_char; 23]>(b"stopMissionButtonFlash\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    };
}
/* set current transporter (for script callbacks) */
/* set current transporter (for script callbacks) */
#[no_mangle]
pub unsafe extern "C" fn transporterSetScriptCurrent(mut psTransporter:
                                                         *mut DROID) {
    g_psCurScriptTransporter = psTransporter;
}
/* get current transporter (for script callbacks) */
/* get current transporter (for script callbacks) */
#[no_mangle]
pub unsafe extern "C" fn transporterGetScriptCurrent() -> *mut DROID {
    return g_psCurScriptTransporter;
}
/* check whether transporter on mission */
//extern BOOL transporterOnMission( void );
/*called when a Transporter has arrived back at the LZ when sending droids to safety*/
/* check whether transporter on mission - Never used!?*/
/*BOOL transporterOnMission( void )
{
	return onMission;
}*/
/*called when a Transporter has arrived back at the LZ when sending droids to safety*/
#[no_mangle]
pub unsafe extern "C" fn resetTransporter(mut psTransporter: *mut DROID) {
    let mut psForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    //enable the form so can add more droids into the transporter
    psForm =
        widgGetFromID(psWScreen, 9010 as libc::c_int as UDWORD) as
            *mut W_CLICKFORM;
    if !psForm.is_null() {
        formSetClickState(psForm, 0 as libc::c_int as UDWORD);
    };
}
/*checks the order of the droid to see if its currenly flying*/
/*checks the order of the droid to see if its currently flying*/
#[no_mangle]
pub unsafe extern "C" fn transporterFlying(mut psTransporter: *mut DROID)
 -> BOOL {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"transporterFlying: Invalid droid pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              2047 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"transporterFlying\x00")).as_ptr(),
              b"PTRVALID(psTransporter, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"transporterFlying: Droid is not a Transporter\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"transporter.c\x00" as *const u8 as *const libc::c_char,
              2049 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"transporterFlying\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psTransporter).order == DORDER_TRANSPORTOUT as libc::c_int ||
           (*psTransporter).order == DORDER_TRANSPORTIN as libc::c_int ||
           (*psTransporter).order == DORDER_TRANSPORTRETURN as libc::c_int ||
           bMultiPlayer != 0 &&
               (*psTransporter).order == DORDER_MOVE as libc::c_int ||
           bMultiPlayer != 0 &&
               (*psTransporter).order == DORDER_DISEMBARK as libc::c_int ||
           bMultiPlayer != 0 &&
               (*psTransporter).order == DORDER_NONE as libc::c_int &&
               (*psTransporter).sMove.iVertSpeed as libc::c_int !=
                   0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
//initialise the flag to indicate the first transporter has arrived - set in startMission()
//initialise the flag to indicate the first transporter has arrived - set in startMission()
#[no_mangle]
pub unsafe extern "C" fn initFirstTransporterFlag() {
    bFirstTransporter = 1 as libc::c_int;
}
