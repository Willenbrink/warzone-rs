use ::libc;
extern "C" {
    pub type _formation;
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
    pub type _w_context;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* These two functions return the current position of the mouse */
    #[no_mangle]
    fn mouseX() -> SDWORD;
    #[no_mangle]
    fn mouseY() -> SDWORD;
    /* This returns true if the mouse key is currently depressed */
    #[no_mangle]
    fn mouseDown(code: MOUSE_KEY_CODE) -> BOOL;
    /* This returns true if the mouse key was double clicked */
    #[no_mangle]
    fn mouseDClicked(code: MOUSE_KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being up to being down this frame */
    #[no_mangle]
    fn mousePressed(code: MOUSE_KEY_CODE) -> BOOL;
    /* This returns true if the mouse key went from being down to being up this frame */
    #[no_mangle]
    fn mouseReleased(code: MOUSE_KEY_CODE) -> BOOL;
    /* Check for a mouse drag, return the drag start coords if dragging */
    #[no_mangle]
    fn mouseDrag(code: MOUSE_KEY_CODE, px: *mut UDWORD, py: *mut UDWORD)
     -> BOOL;
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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    #[no_mangle]
    fn GetTickCount() -> DWORD;
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    //leave as last one
    #[no_mangle]
    static mut godMode: BOOL;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    //extern void createAssemblyPoint(STRUCTURE* psStruct);
    /* consider delivery points when selected by player*/
    #[no_mangle]
    fn processDeliveryPoint(player_0: UDWORD, x: UDWORD, y: UDWORD);
    /*returns the status of the flag*/
    #[no_mangle]
    fn getHQExists(player_0: UDWORD) -> BOOL;
    //print some info at the top of the screen dependant on the structure
    #[no_mangle]
    fn printStructureInfo(psStructure: *mut STRUCTURE);
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    //Find the factory associated with the delivery point - returns NULL if none exist
    #[no_mangle]
    fn findDeliveryFactory(psDelPoint: *mut FLAG_POSITION) -> *mut STRUCTURE;
    /*checks the structure passed in is a Las Sat structure which is currently 
selected - returns TRUE if valid*/
    #[no_mangle]
    fn lasSatStructSelected(psStruct: *mut STRUCTURE) -> BOOL;
    /* Return the type of a droid */
    #[no_mangle]
    fn droidType(psDroid: *mut DROID) -> DROID_TYPE;
    #[no_mangle]
    fn getDroidLevelName(psDroid: *mut DROID) -> *mut STRING;
    // Get a droid's name.
    #[no_mangle]
    fn droidGetName(psDroid: *mut DROID) -> *mut STRING;
    /* Just returns true if the droid's present body points aren't as high as the original*/
    #[no_mangle]
    fn droidIsDamaged(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn setSelectedGroup(groupNumber: UDWORD);
    #[no_mangle]
    fn setSelectedCommander(commander: UDWORD);
    //count how many Command Droids exist in the world at any one moment
    /* Set up a droid to clear a wrecked building feature - returns true if successful */
    /* Update a construction droid while it is clearing 
   returns TRUE while continues */
    /*For a given repair droid, check if there are any damaged droids within 
a defined range*/
    //access function
    /*returns TRUE if a VTOL Weapon Droid which has completed all runs*/
    /*Checks a vtol for being fully armed and fully repaired to see if ready to 
leave reArm pad */
    /*this mends the VTOL when it has been returned to home base whilst on an
offworld mission*/
    /*checks if the droid is a VTOL droid and updates the attack runs as required*/
    /*returns a count of the base number of attack runs for the weapon attached to the droid*/
    //assign rearmPad to the VTOL
    //don't use this function any more - the droid checks each frame for this to have died
//look through all droids to see if any are associated with the ReArming Pad
//extern void releaseVTOLPad(STRUCTURE *psReArmPad);
// true if a vtol is waiting to be rearmed by a particular rearm pad
    // true if a vtol droid currently returning to be rearmed
    // true if a droid is currently attacking
    // see if there are any other vtols attacking the same target
// but still rearming
    /*compares the droid sensor type with the droid weapon type to see if the 
FIRE_SUPPORT order can be assigned*/
    // return whether a droid has a CB sensor on it
    // give a droid from one player to another - used in Electronic Warfare and multiplayer
    /*calculates the electronic resistance of a droid based on its experience level*/
    /*this is called to check the weapon is 'allowed'. Check if VTOL, the weapon is 
direct fire. Also check numVTOLattackRuns for the weapon is not zero - return 
TRUE if valid weapon*/
    /*called when a Template is deleted in the Design screen*/
    // Select a droid and do any necessary housekeeping.
    #[no_mangle]
    fn SelectDroid(psDroid: *mut DROID);
    // De-select a droid and do any necessary housekeeping.
    #[no_mangle]
    fn DeSelectDroid(psDroid: *mut DROID);
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
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    /*calculate the power cost to repair a droid*/
    /*power cost for One repair point*/
    /* audio finished callback */
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    #[no_mangle]
    fn droidSensorDroidWeapon(psObj: *mut BASE_OBJECT, psDroid: *mut DROID)
     -> BOOL;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    fn gamePaused() -> BOOL;
    //mouse states
    #[no_mangle]
    fn pie_SetMouse(ImageFile: *mut IMAGEFILE, ImageID: UWORD);
    #[no_mangle]
    static mut iV_SetTransFilter:
           Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD) -> ()>;
    #[no_mangle]
    fn assignSensorTarget(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    static mut mouseTileY: SDWORD;
    #[no_mangle]
    static mut mouseTileX: SDWORD;
    #[no_mangle]
    fn assignDestTarget();
    #[no_mangle]
    static mut selectAttempt: BOOL;
    #[no_mangle]
    fn draw3DScene();
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    static mut intMode: INTMODE;
    // Get current audio ID for hilight.
    // Get current audio ID for clicked.
    #[no_mangle]
    fn getWidgetsStatus() -> BOOL;
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    #[no_mangle]
    fn intRefreshScreen();
    #[no_mangle]
    fn intBuildSelectMode() -> BOOL;
    #[no_mangle]
    fn intConstructorSelected(psDroid: *mut DROID);
    #[no_mangle]
    fn intCommanderSelected(psDroid: *mut DROID);
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    #[no_mangle]
    fn intDemolishSelectMode() -> BOOL;
    #[no_mangle]
    fn intObjectSelected(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn addTransporterInterface(psSelected: *mut DROID, onMission: BOOL);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn init3DBuilding(psStats: *mut BASE_STATS, CallBack: BUILDCALLBACK,
                      UserData: *mut libc::c_void);
    #[no_mangle]
    fn kill3DBuilding();
    #[no_mangle]
    static mut buildSite: HIGHLIGHT;
    #[no_mangle]
    static mut sBuildDetails: BUILDDETAILS;
    #[no_mangle]
    static mut buildState: UDWORD;
    #[no_mangle]
    fn getTileOccupier(x: UDWORD, y: UDWORD) -> *mut BASE_OBJECT;
    #[no_mangle]
    fn getNearestDroid(x: UDWORD, y: UDWORD, bSelected: BOOL) -> *mut DROID;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime2: UDWORD;
    #[no_mangle]
    fn CalcRadarPosition(mX_0: UDWORD, mY_0: UDWORD, PosX: *mut UDWORD,
                         PosY: *mut UDWORD);
    #[no_mangle]
    fn CoordInRadar(x: libc::c_int, y: libc::c_int) -> BOOL;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn setConsolePermanence(state: BOOL, bClearOld: BOOL);
    #[no_mangle]
    fn mouseOverConsoleBox() -> BOOL;
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    /* Give a droid an order with a location and a stat */
    #[no_mangle]
    fn orderDroidStatsLoc(psDroid: *mut DROID, order: DROID_ORDER,
                          psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD);
    /* Give selected droids an order with a location target */
    #[no_mangle]
    fn orderSelectedLoc(player_0: UDWORD, x: UDWORD, y: UDWORD);
    #[no_mangle]
    fn orderSelectedLocAdd(player_0: UDWORD, x: UDWORD, y: UDWORD, add: BOOL);
    /* Give selected droids a new waypoint to add to move*/
//extern void orderSelectedWaypoint(UDWORD player, UDWORD x, UDWORD y);
//extern BOOL orderAddWayPoint(DROID *psDroid ,UDWORD dX,UDWORD dY);
    /* Give selected droids an order with an object target */
    #[no_mangle]
    fn orderSelectedObj(player_0: UDWORD, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn orderSelectedObjAdd(player_0: UDWORD, psObj: *mut BASE_OBJECT,
                           add: BOOL);
    /* add an order with a location and a stat to the droids order list*/
    #[no_mangle]
    fn orderDroidStatsLocAdd(psDroid: *mut DROID, order: DROID_ORDER,
                             psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD);
    // make all the members of a numeric group have the same secondary states
    #[no_mangle]
    fn secondarySetAverageGroupState(player_0: UDWORD, group: UDWORD);
    // do a health check for a droid
    // set the state of a secondary order for a Factory, return FALSE if failed.
    // get the state of a secondary order for a Factory, return FALSE if unsupported
    //lasSat structure can select a target
    #[no_mangle]
    fn orderStructureObj(player_0: UDWORD, psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn keyProcessMappings(bExclude: BOOL);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    static mut InGameOpUp: BOOL;
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn requestRadarTrack(x: SDWORD, y: SDWORD);
    #[no_mangle]
    fn getRadarTrackingStatus() -> BOOL;
    #[no_mangle]
    fn camInformOfRotation(rotation: *mut iVector);
    #[no_mangle]
    fn getNumDroidsSelected() -> UDWORD;
    #[no_mangle]
    fn camAllignWithTarget(psTarget: *mut BASE_OBJECT);
    #[no_mangle]
    fn kf_ZoomOut();
    #[no_mangle]
    fn kf_ZoomIn();
    #[no_mangle]
    fn kf_SendTextMessage();
    #[no_mangle]
    static mut bAllowOtherKeyPresses: BOOL;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    #[no_mangle]
    fn fireOnLocation(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    // the Droid that was selected for a CALL_DROID_SELECTED
    #[no_mangle]
    static mut psCBSelectedDroid: *mut DROID;
    #[no_mangle]
    fn targetGetCurrent() -> *mut BASE_OBJECT;
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    static mut psDrivenDroid: *mut DROID;
    #[no_mangle]
    fn StopDriverMode();
    #[no_mangle]
    fn driveEnableControl();
    #[no_mangle]
    fn driveProcessAquireButton();
    #[no_mangle]
    fn driveDisableTactical();
    #[no_mangle]
    fn driveTacticalActive() -> BOOL;
    #[no_mangle]
    fn driveProcessRadarInput(x: libc::c_int, y: libc::c_int);
    // ---------------------------------------------------------------------
// EXTERNALLY REFERENCED FUNCTIONS
    #[no_mangle]
    fn selDroidSelection(player_0: UDWORD, droidClass: SELECTION_CLASS,
                         droidType_0: SELECTIONTYPE, bOnScreen: BOOL)
     -> UDWORD;
    #[no_mangle]
    fn selNumSelected(player_0: UDWORD) -> UDWORD;
    /*calculates how much space is remaining on the transporter - allows droids to take
up different amount depending on their body size - currently all are set to one!*/
    #[no_mangle]
    fn calcRemainingCapacity(psTransporter: *mut DROID) -> UDWORD;
    //new function added to bring up the RMB order form for Factories as well as droids
    #[no_mangle]
    fn intAddFactoryOrder(psStructure: *mut STRUCTURE);
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    static mut bDisplayMultiJoiningStatus: UBYTE;
    #[no_mangle]
    fn isHumanPlayer(player_0: UDWORD) -> BOOL;
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type WORD = libc::c_short;
pub type DWORD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PALETTEENTRY {
    pub peRed: UBYTE,
    pub peGreen: UBYTE,
    pub peBlue: UBYTE,
    pub peFlags: UBYTE,
}
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
pub type _mouse_key_code = libc::c_uint;
pub const MOUSE_WDN: _mouse_key_code = 5;
pub const MOUSE_WUP: _mouse_key_code = 4;
pub const MOUSE_RMB: _mouse_key_code = 3;
pub const MOUSE_MMB: _mouse_key_code = 2;
pub const MOUSE_LMB: _mouse_key_code = 1;
pub type MOUSE_KEY_CODE = _mouse_key_code;
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
// root of the tree
/*
 * StrRes.h
 *
 * String resource interface functions
 *
 */
/* A string block */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _str_block {
    pub apStrings: *mut *mut STRING,
    pub idStart: UDWORD,
    pub idEnd: UDWORD,
    pub psNext: *mut _str_block,
}
pub type STR_BLOCK = _str_block;
/* A String Resource */
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
pub struct BSPTREENODE {
    pub link: [*mut BSPTREENODE; 2],
    pub Plane: PLANE,
    pub TriSameDir: BSPPOLYID,
    pub TriOppoDir: BSPPOLYID,
}
// The next free ID
// only needed when generating the tree
/* **************************************************************************/
// These 1st three entries can NOT NOW be cast into a iVectorf *   (iVectorf on PC are doubles)
// these values form the plane equation ax+by+cz=d
// a point on the plane - in normal non-fract format
pub type PSBSPTREENODE = *mut BSPTREENODE;
pub type VERTEXID = libc::c_int;
// points to first polygon in the BSP tree entry ... BSP_NextPoly in the iIMDPoly structure will point to the next entry
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// id of the first polygon in the list ... or BSPPOLYID_TERMINATE for none
// only needed when generating the tree
/* **************************************************************************/
/*
 * ivisdef.h
 *
 * type defines for all ivis library functions.
 *
 */
/* **************************************************************************/
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
// now defined for all versions (optional BSP handled on all formats)
//Not really needed I guess, however, see debug.c comments.  -Qamly
// texture animation defines
/* **************************************************************************/
/*
 *	Global Macros
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Type Definitions
 */
/* **************************************************************************/
//*************************************************************************
//
// Basic types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// Simple derived types (now defined in pieTypes.h)
//
//*************************************************************************
//*************************************************************************
//
// screen surface structure
//
//*************************************************************************
// currently uses 4k per structure (!)
//*************************************************************************
//
// texture animation structures
//
//*************************************************************************
//*************************************************************************
//
// imd structures
//
//*************************************************************************
// lets hope this can work as a byte ... that will limit it to 255 polygons in 1 imd
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
pub type _loc = libc::c_uint;
pub const LOC_TURRET: _loc = 1;
pub const LOC_DEFAULT: _loc = 0;
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
pub type WEAPON_STATS = _weapon_stats;
//no capability - droid must stop
//partial capability - droid has 50% chance to hit
//full capability - droid fires normally on move
//works as all of the above together! - new for updates - added 11/06/99 AB
/* Common stats */
// Max distance to target for short range shot
// Max distance to target for long range shot
// Min distance to target for shot
//Use Movement Model now for projectile type - AB 15/6/98
	//BOOL			direct;				// Whether the weapon fires directly or indirectly
// Chance to hit at short range
// Chance to hit at long range
// Time between each weapon fire
// The number of explosions per shot
// The number of rounds per salvo
// Time to reload the round of ammo (salvo fire)
// How much damage the weapon causes
// Basic blast radius of weapon
// Chance to hit in the blast radius
// Damage done in the blast radius
// How long the round burns
// Damage done each burn cycle
// Burn radius of the round
// speed ammo travels at
// how high the ammo travels for indirect fire
// indicates whether the droid has to stop before firing
// the class of weapon - see enum WEAPON_CLASS
// the subclass to which the weapon belongs - see 
// enum WEAPON_SUBCLASS
// which projectile model to use for the bullet
// which type of warhead is associated with the weapon
//Use Movement Model now for projectile type - AB 15/6/98
	//BOOL			homingRound;		// flag to indicate whether homing or not
// used to compare with weight to see if recoils or not
// amount the weapon(turret) can rotate 0 = none
// max amount the turret can be elevated up			
// min amount the turret can be elevated down
// flag to make the (explosion) effect face the player when drawn
// flag to make the inflight effect face the player when drawn
// size of the effect 100 = normal, 50 = half etc
// flag to indicate whether the effect lights up the world
// indicates how good in the air - SHOOT_ON_GROUND, SHOOT_IN_AIR or both
// number of attack runs a VTOL droid can do with this weapon
/* Graphics control stats */
// How long a direct fire weapon is visible
// Measured in 1/100 sec.
// How long a blast radius is visible
/* Graphics used for the weapon */
// The turret mount to use
// The muzzle flash 
// The ammo in flight
// The ammo hitting a target
// The ammo missing a target
// The ammo hitting water
// The trail used for in flight
/* Audio */
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
pub struct _object_position {
    pub type_0: POSITION_TYPE,
    pub frameNumber: UDWORD,
    pub screenX: UDWORD,
    pub screenY: UDWORD,
    pub screenR: UDWORD,
    pub player: UDWORD,
    pub selected: BOOL,
}
pub type OBJECT_POSITION = _object_position;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _power_gen {
    pub power: UDWORD,
    pub multiplier: UDWORD,
    pub capacity: UDWORD,
    pub apResExtractors: [*mut _structure; 4],
}
pub type POWER_GEN = _power_gen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct REPAIR_FACILITY {
    pub power: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub powerAccrued: UDWORD,
    pub psDeliveryPoint: *mut FLAG_POSITION,
    pub currentPtsAdded: UDWORD,
    pub psGroup: *mut _droid_group,
    pub psGrpNext: *mut _droid,
}
pub type STRUCTURE = _structure;
// position to retreat to
// number of units below which might run
// %health value below which to turn and run - FOR GROUPS ONLY
// basic chance to run
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
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_0 = 7;
pub const INT_DESIGN: INTMODE = 7;
pub type INTMODE = libc::c_uint;
pub const INT_MAXMODE: INTMODE = 15;
pub const INT_CDCHANGE: INTMODE = 14;
pub const INT_MULTIMENU: INTMODE = 13;
pub const INT_MISSIONRES: INTMODE = 12;
pub const INT_TRANSPORTER: INTMODE = 11;
pub const INT_INGAMEOP: INTMODE = 10;
pub const INT_ORDER: INTMODE = 9;
pub const INT_INTELMAP: INTMODE = 8;
pub const INT_CMDORDER: INTMODE = 6;
pub const INT_STAT: INTMODE = 5;
pub const INT_OBJECT: INTMODE = 4;
pub const INT_EDITSTAT: INTMODE = 3;
pub const INT_EDIT: INTMODE = 2;
pub const INT_OPTION: INTMODE = 1;
pub const INT_NORMAL: INTMODE = 0;
pub type MOUSE_TARGET = _targets;
pub type _targets = libc::c_uint;
pub const MT_NOTARGET: _targets = 23;
pub const MT_SENSORSTRUCTDAM: _targets = 22;
pub const MT_SENSORSTRUCT: _targets = 21;
pub const MT_CONSTRUCT: _targets = 20;
pub const MT_WRECKFEATURE: _targets = 19;
pub const MT_SENSOR: _targets = 18;
pub const MT_DAMFEATURE: _targets = 17;
pub const MT_ARTIFACT: _targets = 16;
pub const MT_COMMAND: _targets = 15;
pub const MT_ENEMYDROID: _targets = 14;
pub const MT_OWNDROIDDAM: _targets = 13;
pub const MT_OWNDROID: _targets = 12;
pub const MT_TRANDROID: _targets = 11;
pub const MT_ENEMYSTR: _targets = 10;
pub const MT_REPAIRDAM: _targets = 9;
pub const MT_REPAIR: _targets = 8;
pub const MT_OWNSTRINCOMP: _targets = 7;
pub const MT_OWNSTROK: _targets = 6;
pub const MT_OWNSTRDAM: _targets = 5;
pub const MT_TRENCH: _targets = 4;
pub const MT_RIVER: _targets = 3;
pub const MT_BLOCKING: _targets = 2;
pub const MT_RESOURCE: _targets = 1;
pub const MT_TERRAIN: _targets = 0;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_1 = 232;
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_1 = 242;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_1 = 228;
pub type SELECTION_TYPE = _selectionTypes;
pub type _selectionTypes = libc::c_uint;
pub const SC_INVALID: _selectionTypes = 13;
pub const SC_DROID_REPAIR: _selectionTypes = 12;
pub const SC_DROID_DEMOLISH: _selectionTypes = 11;
pub const SC_DROID_TRANSPORTER: _selectionTypes = 10;
pub const SC_DROID_BOMBER: _selectionTypes = 9;
pub const SC_DROID_COMMAND: _selectionTypes = 8;
pub const SC_DROID_RECOVERY: _selectionTypes = 7;
pub const SC_DROID_BRIDGE: _selectionTypes = 6;
pub const SC_DROID_ECM: _selectionTypes = 5;
pub const SC_DROID_SENSOR: _selectionTypes = 4;
pub const SC_DROID_CLOSE: _selectionTypes = 3;
pub const SC_DROID_INDIRECT: _selectionTypes = 2;
pub const SC_DROID_DIRECT: _selectionTypes = 1;
pub const SC_DROID_CONSTRUCT: _selectionTypes = 0;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_1 = 229;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_1 = 240;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_1 = 241;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_1 = 234;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_1 = 230;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_1 = 231;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_1 = 235;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_1 = 239;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_1 = 236;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_1 = 233;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_1 = 237;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_1 = 238;
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_1 = 348;
pub const TER_CLIFFFACE: _terrain_type = 8;
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
pub const ID_SOUND_SELECT: C2RustUnnamed_0 = 2;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dragBox {
    pub x1: UDWORD,
    pub y1: UDWORD,
    pub x2: UDWORD,
    pub y2: UDWORD,
    pub status: UDWORD,
    pub lastTime: UDWORD,
    pub boxColourIndex: UDWORD,
}
pub type SELECTIONTYPE = _selectiontype;
pub type _selectiontype = libc::c_uint;
pub const DST_ALL_SAME: _selectiontype = 8;
pub const DST_ALL_DAMAGED: _selectiontype = 7;
pub const DST_ALL_COMBAT: _selectiontype = 6;
pub const DST_HALF_TRACKED: _selectiontype = 5;
pub const DST_TRACKED: _selectiontype = 4;
pub const DST_WHEELED: _selectiontype = 3;
pub const DST_HOVER: _selectiontype = 2;
pub const DST_VTOL: _selectiontype = 1;
pub const DST_UNUSED: _selectiontype = 0;
pub type SELECTION_CLASS = _selection_class;
pub type _selection_class = libc::c_uint;
pub const DS_BY_TYPE: _selection_class = 1;
pub const DS_ALL_UNITS: _selection_class = 0;
pub type BUILDCALLBACK
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut libc::c_void)
               -> ()>;
pub type HIGHLIGHT = _highlight;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _highlight {
    pub xTL: UWORD,
    pub yTL: UWORD,
    pub xBR: UWORD,
    pub yBR: UWORD,
}
pub type TRIGGER_TYPE = _trigger_type;
// Feature armour
// Top left of box to highlight
// Bottom right of box to highlight
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
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
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
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_0 = 6;
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
// something else.
pub const STR_GAM_DERRICK_BURNING: _fixed_str_id = 384;
pub const STR_GAM_DERRICK: _fixed_str_id = 220;
pub const CALL_DROID_SELECTED: _scr_callback_types = 27;
pub const GT_COMMAND: _group_type = 1;
pub const STR_GAM_DROIDSTATE: _fixed_str_id = 221;
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
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
pub type WIDGET_TYPE = _widget_type;
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
/* The common widget data */
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub type BUILDDETAILS = _build_details;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _build_details {
    pub CallBack: BUILDCALLBACK,
    pub UserData: *mut libc::c_void,
    pub x: UDWORD,
    pub y: UDWORD,
    pub width: UDWORD,
    pub height: UDWORD,
    pub psStats: *mut BASE_STATS,
}
pub type _terrain_type = libc::c_uint;
pub const TER_MAX: _terrain_type = 12;
pub const TER_SLUSH: _terrain_type = 11;
pub const TER_SHEETICE: _terrain_type = 10;
pub const TER_RUBBLE: _terrain_type = 9;
pub const TER_WATER: _terrain_type = 7;
pub const TER_ROAD: _terrain_type = 6;
pub const TER_PINKROCK: _terrain_type = 5;
pub const TER_REDBRUSH: _terrain_type = 4;
pub const TER_GREENMUD: _terrain_type = 3;
pub const TER_BAKEDEARTH: _terrain_type = 2;
pub const TER_SANDYBRUSH: _terrain_type = 1;
pub const TER_SAND: _terrain_type = 0;
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
//this holds the OBJECT_POSITION pointer for a Deliv Point
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
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_0 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_0 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_0 = 3;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_0 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_0 = 0;
pub const NO_SOUND: C2RustUnnamed_0 = -1;
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_1 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_1 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_1 = 481;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_1 = 480;
pub const IMAGE_LAUNCHUP: C2RustUnnamed_1 = 479;
pub const IMAGE_BLUE6: C2RustUnnamed_1 = 478;
pub const IMAGE_BLUE5: C2RustUnnamed_1 = 477;
pub const IMAGE_BLUE4: C2RustUnnamed_1 = 476;
pub const IMAGE_BLUE3: C2RustUnnamed_1 = 475;
pub const IMAGE_BLUE2: C2RustUnnamed_1 = 474;
pub const IMAGE_BLUE1: C2RustUnnamed_1 = 473;
pub const IMAGE_DES_ARMOUR_KINETIC: C2RustUnnamed_1 = 472;
pub const IMAGE_TABHILIGHT_SM: C2RustUnnamed_1 = 471;
pub const IMAGE_TAB1DOWN_SM: C2RustUnnamed_1 = 470;
pub const IMAGE_TAB1SELECTED_SM: C2RustUnnamed_1 = 469;
pub const IMAGE_TAB1_SM: C2RustUnnamed_1 = 468;
pub const IMAGE_TARGET5: C2RustUnnamed_1 = 467;
pub const IMAGE_TARGET4: C2RustUnnamed_1 = 466;
pub const IMAGE_TARGET3: C2RustUnnamed_1 = 465;
pub const IMAGE_TARGET2: C2RustUnnamed_1 = 464;
pub const IMAGE_TARGET1: C2RustUnnamed_1 = 463;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_1 = 462;
pub const IMAGE_RES_GRPDAM: C2RustUnnamed_1 = 461;
pub const IMAGE_RES_GRPACC: C2RustUnnamed_1 = 460;
pub const IMAGE_RES_GRPROF: C2RustUnnamed_1 = 459;
pub const IMAGE_RES_GRPREP: C2RustUnnamed_1 = 458;
pub const IMAGE_RES_GRPUPG: C2RustUnnamed_1 = 457;
pub const IMAGE_ASCII131: C2RustUnnamed_1 = 456;
pub const IMAGE_ASCII161: C2RustUnnamed_1 = 455;
pub const IMAGE_ASCII191: C2RustUnnamed_1 = 454;
pub const IMAGE_ASCII208: C2RustUnnamed_1 = 453;
pub const IMAGE_ASCII207: C2RustUnnamed_1 = 452;
pub const IMAGE_ASCII206: C2RustUnnamed_1 = 451;
pub const IMAGE_ASCII205: C2RustUnnamed_1 = 450;
pub const IMAGE_ASCII204: C2RustUnnamed_1 = 449;
pub const IMAGE_ASCII203: C2RustUnnamed_1 = 448;
pub const IMAGE_ASCII202: C2RustUnnamed_1 = 447;
pub const IMAGE_ASCII201: C2RustUnnamed_1 = 446;
pub const IMAGE_ASCII200: C2RustUnnamed_1 = 445;
pub const IMAGE_ASCII198: C2RustUnnamed_1 = 444;
pub const IMAGE_ASCII197: C2RustUnnamed_1 = 443;
pub const IMAGE_ASCII196: C2RustUnnamed_1 = 442;
pub const IMAGE_ASCII195: C2RustUnnamed_1 = 441;
pub const IMAGE_ASCII194: C2RustUnnamed_1 = 440;
pub const IMAGE_ASCII193: C2RustUnnamed_1 = 439;
pub const IMAGE_ASCII192: C2RustUnnamed_1 = 438;
pub const IMAGE_ASCII210: C2RustUnnamed_1 = 437;
pub const IMAGE_ASCII211: C2RustUnnamed_1 = 436;
pub const IMAGE_ASCII214: C2RustUnnamed_1 = 435;
pub const IMAGE_ASCII213: C2RustUnnamed_1 = 434;
pub const IMAGE_ASCII212: C2RustUnnamed_1 = 433;
pub const IMAGE_ASCII216: C2RustUnnamed_1 = 432;
pub const IMAGE_ASCII220: C2RustUnnamed_1 = 431;
pub const IMAGE_ASCII217: C2RustUnnamed_1 = 430;
pub const IMAGE_ASCII219: C2RustUnnamed_1 = 429;
pub const IMAGE_ASCII218: C2RustUnnamed_1 = 428;
pub const IMAGE_ASCII221: C2RustUnnamed_1 = 427;
pub const IMAGE_ASCII223: C2RustUnnamed_1 = 426;
pub const IMAGE_ASCII248: C2RustUnnamed_1 = 425;
pub const IMAGE_ASCII241: C2RustUnnamed_1 = 424;
pub const IMAGE_ASCII253: C2RustUnnamed_1 = 423;
pub const IMAGE_ASCII252: C2RustUnnamed_1 = 422;
pub const IMAGE_ASCII251: C2RustUnnamed_1 = 421;
pub const IMAGE_ASCII250: C2RustUnnamed_1 = 420;
pub const IMAGE_ASCII249: C2RustUnnamed_1 = 419;
pub const IMAGE_ASCII246: C2RustUnnamed_1 = 418;
pub const IMAGE_ASCII245: C2RustUnnamed_1 = 417;
pub const IMAGE_ASCII244: C2RustUnnamed_1 = 416;
pub const IMAGE_ASCII243: C2RustUnnamed_1 = 415;
pub const IMAGE_ASCII242: C2RustUnnamed_1 = 414;
pub const IMAGE_ASCII239: C2RustUnnamed_1 = 413;
pub const IMAGE_ASCII238: C2RustUnnamed_1 = 412;
pub const IMAGE_ASCII237: C2RustUnnamed_1 = 411;
pub const IMAGE_ASCII236: C2RustUnnamed_1 = 410;
pub const IMAGE_ASCII235: C2RustUnnamed_1 = 409;
pub const IMAGE_ASCII234: C2RustUnnamed_1 = 408;
pub const IMAGE_ASCII233: C2RustUnnamed_1 = 407;
pub const IMAGE_ASCII232: C2RustUnnamed_1 = 406;
pub const IMAGE_ASCII231: C2RustUnnamed_1 = 405;
pub const IMAGE_ASCII230: C2RustUnnamed_1 = 404;
pub const IMAGE_ASCII229: C2RustUnnamed_1 = 403;
pub const IMAGE_ASCII228: C2RustUnnamed_1 = 402;
pub const IMAGE_ASCII227: C2RustUnnamed_1 = 401;
pub const IMAGE_ASCII226: C2RustUnnamed_1 = 400;
pub const IMAGE_ASCII225: C2RustUnnamed_1 = 399;
pub const IMAGE_ASCII224: C2RustUnnamed_1 = 398;
pub const IMAGE_ASCII189: C2RustUnnamed_1 = 397;
pub const IMAGE_ASCII188: C2RustUnnamed_1 = 396;
pub const IMAGE_RES_DEFENCE: C2RustUnnamed_1 = 395;
pub const IMAGE_LEV_7: C2RustUnnamed_1 = 394;
pub const IMAGE_LEV_6: C2RustUnnamed_1 = 393;
pub const IMAGE_LEV_5: C2RustUnnamed_1 = 392;
pub const IMAGE_LEV_4: C2RustUnnamed_1 = 391;
pub const IMAGE_LEV_3: C2RustUnnamed_1 = 390;
pub const IMAGE_LEV_2: C2RustUnnamed_1 = 389;
pub const IMAGE_LEV_1: C2RustUnnamed_1 = 388;
pub const IMAGE_LEV_0: C2RustUnnamed_1 = 387;
pub const IMAGE_CDCHANGE_CANCEL: C2RustUnnamed_1 = 386;
pub const IMAGE_CDCHANGE_OK: C2RustUnnamed_1 = 385;
pub const IMAGE_MULTI_CHAN: C2RustUnnamed_1 = 384;
pub const IMAGE_MULTI_NOCHAN: C2RustUnnamed_1 = 383;
pub const IMAGE_ORD_EMBARKDOWN: C2RustUnnamed_1 = 382;
pub const IMAGE_ORD_EMBARKUP: C2RustUnnamed_1 = 381;
pub const IMAGE_ORD_RTRDOWN: C2RustUnnamed_1 = 380;
pub const IMAGE_ORD_RTRUP: C2RustUnnamed_1 = 379;
pub const IMAGE_ORD_GUARDDOWN: C2RustUnnamed_1 = 378;
pub const IMAGE_ORD_GUARDUP: C2RustUnnamed_1 = 377;
pub const IMAGE_ORD_PERSUEDOWN: C2RustUnnamed_1 = 376;
pub const IMAGE_ORD_PERSUEUP: C2RustUnnamed_1 = 375;
pub const IMAGE_ORD_PATROLDOWN: C2RustUnnamed_1 = 374;
pub const IMAGE_ORD_PATROLUP: C2RustUnnamed_1 = 373;
pub const IMAGE_GN_0: C2RustUnnamed_1 = 372;
pub const IMAGE_GN_1: C2RustUnnamed_1 = 371;
pub const IMAGE_GN_2: C2RustUnnamed_1 = 370;
pub const IMAGE_GN_3: C2RustUnnamed_1 = 369;
pub const IMAGE_GN_4: C2RustUnnamed_1 = 368;
pub const IMAGE_GN_5: C2RustUnnamed_1 = 367;
pub const IMAGE_GN_6: C2RustUnnamed_1 = 366;
pub const IMAGE_GN_7: C2RustUnnamed_1 = 365;
pub const IMAGE_GN_8: C2RustUnnamed_1 = 364;
pub const IMAGE_GN_9: C2RustUnnamed_1 = 363;
pub const IMAGE_GN_STAR: C2RustUnnamed_1 = 362;
pub const IMAGE_VDP_HI: C2RustUnnamed_1 = 361;
pub const IMAGE_VDP_UP: C2RustUnnamed_1 = 360;
pub const IMAGE_VDP_DOWN: C2RustUnnamed_1 = 359;
pub const IMAGE_FDP_HI: C2RustUnnamed_1 = 358;
pub const IMAGE_FDP_UP: C2RustUnnamed_1 = 357;
pub const IMAGE_FDP_DOWN: C2RustUnnamed_1 = 356;
pub const IMAGE_CDP_HI: C2RustUnnamed_1 = 355;
pub const IMAGE_CDP_UP: C2RustUnnamed_1 = 354;
pub const IMAGE_CDP_DOWN: C2RustUnnamed_1 = 353;
pub const IMAGE_LOOP_HI: C2RustUnnamed_1 = 352;
pub const IMAGE_LOOP_DOWN: C2RustUnnamed_1 = 351;
pub const IMAGE_LOOP_UP: C2RustUnnamed_1 = 350;
pub const IMAGE_RES_CYBORGTECH: C2RustUnnamed_1 = 349;
pub const IMAGE_DES_EDITBOXRIGHTH: C2RustUnnamed_1 = 347;
pub const IMAGE_DES_EDITBOXMIDH: C2RustUnnamed_1 = 346;
pub const IMAGE_DES_EDITBOXRIGHT: C2RustUnnamed_1 = 345;
pub const IMAGE_DES_EDITBOXMID: C2RustUnnamed_1 = 344;
pub const IMAGE_DES_POWERBAR_RIGHT: C2RustUnnamed_1 = 343;
pub const IMAGE_DES_POWERBAR_LEFT: C2RustUnnamed_1 = 342;
pub const IMAGE_AUDIO_LASTSAMPLEH: C2RustUnnamed_1 = 341;
pub const IMAGE_AUDIO_LASTSAMPLE: C2RustUnnamed_1 = 340;
pub const IMAGE_DES_BINH: C2RustUnnamed_1 = 339;
pub const IMAGE_DES_BIN: C2RustUnnamed_1 = 338;
pub const IMAGE_INFINITE_HI: C2RustUnnamed_1 = 337;
pub const IMAGE_INFINITE_DOWN: C2RustUnnamed_1 = 336;
pub const IMAGE_INFINITE_UP: C2RustUnnamed_1 = 335;
pub const IMAGE_RETICULE_GREY: C2RustUnnamed_1 = 334;
pub const IMAGE_ORD_FACHILITE: C2RustUnnamed_1 = 333;
pub const IMAGE_ORD_FAC5DOWN: C2RustUnnamed_1 = 332;
pub const IMAGE_ORD_FAC5UP: C2RustUnnamed_1 = 331;
pub const IMAGE_ORD_FAC4DOWN: C2RustUnnamed_1 = 330;
pub const IMAGE_ORD_FAC4UP: C2RustUnnamed_1 = 329;
pub const IMAGE_ORD_FAC3DOWN: C2RustUnnamed_1 = 328;
pub const IMAGE_ORD_FAC3UP: C2RustUnnamed_1 = 327;
pub const IMAGE_ORD_FAC2DOWN: C2RustUnnamed_1 = 326;
pub const IMAGE_ORD_FAC2UP: C2RustUnnamed_1 = 325;
pub const IMAGE_ORD_FAC1DOWN: C2RustUnnamed_1 = 324;
pub const IMAGE_ORD_FAC1UP: C2RustUnnamed_1 = 323;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_1 = 322;
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_1 = 321;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_1 = 320;
pub const IMAGE_NRUTER: C2RustUnnamed_1 = 319;
pub const IMAGE_CMDDROID_EXP: C2RustUnnamed_1 = 318;
pub const IMAGE_MULTI_VIS_HI: C2RustUnnamed_1 = 317;
pub const IMAGE_MULTI_POW_HI: C2RustUnnamed_1 = 316;
pub const IMAGE_MULTI_DRO_HI: C2RustUnnamed_1 = 315;
pub const IMAGE_MULTI_TEK_HI: C2RustUnnamed_1 = 314;
pub const IMAGE_MULTI_TEK: C2RustUnnamed_1 = 313;
pub const IMAGE_MULTI_DRO: C2RustUnnamed_1 = 312;
pub const IMAGE_MULTI_POW: C2RustUnnamed_1 = 311;
pub const IMAGE_MULTI_VIS: C2RustUnnamed_1 = 310;
pub const IMAGE_MULTI_AL_HI: C2RustUnnamed_1 = 309;
pub const IMAGE_MULTI_OFFAL_HI: C2RustUnnamed_1 = 308;
pub const IMAGE_MULTI_NOAL_HI: C2RustUnnamed_1 = 307;
pub const IMAGE_MULTI_AL: C2RustUnnamed_1 = 306;
pub const IMAGE_MULTI_OFFAL: C2RustUnnamed_1 = 305;
pub const IMAGE_MULTI_NOAL: C2RustUnnamed_1 = 304;
pub const IMAGE_RAD_ART3: C2RustUnnamed_1 = 303;
pub const IMAGE_RAD_ART2: C2RustUnnamed_1 = 302;
pub const IMAGE_RAD_ART1: C2RustUnnamed_1 = 301;
pub const IMAGE_RAD_ARTREAD: C2RustUnnamed_1 = 300;
pub const IMAGE_RAD_RES3: C2RustUnnamed_1 = 299;
pub const IMAGE_RAD_RES2: C2RustUnnamed_1 = 298;
pub const IMAGE_RAD_RES1: C2RustUnnamed_1 = 297;
pub const IMAGE_RAD_RESREAD: C2RustUnnamed_1 = 296;
pub const IMAGE_RAD_ENM3: C2RustUnnamed_1 = 295;
pub const IMAGE_RAD_ENM2: C2RustUnnamed_1 = 294;
pub const IMAGE_RAD_ENM1: C2RustUnnamed_1 = 293;
pub const IMAGE_RAD_ENMREAD: C2RustUnnamed_1 = 292;
pub const IMAGE_SLIDER_INFINITY: C2RustUnnamed_1 = 291;
pub const IMAGE_SLIDER_BIGBUT: C2RustUnnamed_1 = 290;
pub const IMAGE_ORD_DESTRUCT2GREY: C2RustUnnamed_1 = 289;
pub const IMAGE_ORD_DESTRUCT2DOWN: C2RustUnnamed_1 = 288;
pub const IMAGE_ORD_DESTRUCT2UP: C2RustUnnamed_1 = 287;
pub const IMAGE_ORD_GOTOHQDOWN: C2RustUnnamed_1 = 286;
pub const IMAGE_ORD_GOTOHQUP: C2RustUnnamed_1 = 285;
pub const IMAGE_ORD_HALTDOWN: C2RustUnnamed_1 = 284;
pub const IMAGE_ORD_HALTUP: C2RustUnnamed_1 = 283;
pub const IMAGE_ORD_HOLDFIREDOWN: C2RustUnnamed_1 = 282;
pub const IMAGE_ORD_HOLDFIREUP: C2RustUnnamed_1 = 281;
pub const IMAGE_ORD_RETFIREDOWN: C2RustUnnamed_1 = 280;
pub const IMAGE_ORD_RETFIREUP: C2RustUnnamed_1 = 279;
pub const IMAGE_ORD_FATWILLDOWN: C2RustUnnamed_1 = 278;
pub const IMAGE_ORD_FATWILLUP: C2RustUnnamed_1 = 277;
pub const IMAGE_ORD_REPAIR3DOWN: C2RustUnnamed_1 = 276;
pub const IMAGE_ORD_REPAIR3UP: C2RustUnnamed_1 = 275;
pub const IMAGE_ORD_REPAIR1DOWN: C2RustUnnamed_1 = 274;
pub const IMAGE_ORD_REPAIR1UP: C2RustUnnamed_1 = 273;
pub const IMAGE_ORD_REPAIR2DOWN: C2RustUnnamed_1 = 272;
pub const IMAGE_ORD_REPAIR2UP: C2RustUnnamed_1 = 271;
pub const IMAGE_ORD_RANGE3DOWN: C2RustUnnamed_1 = 270;
pub const IMAGE_ORD_RANGE3UP: C2RustUnnamed_1 = 269;
pub const IMAGE_ORD_RANGE2DOWN: C2RustUnnamed_1 = 268;
pub const IMAGE_ORD_RANGE2UP: C2RustUnnamed_1 = 267;
pub const IMAGE_ORD_RANGE1DOWN: C2RustUnnamed_1 = 266;
pub const IMAGE_ORD_RANGE1UP: C2RustUnnamed_1 = 265;
pub const IMAGE_ORD_DESTRUCT1DOWN: C2RustUnnamed_1 = 264;
pub const IMAGE_ORD_DESTRUCT1UP: C2RustUnnamed_1 = 263;
pub const IMAGE_RES_QUESTIONMARK: C2RustUnnamed_1 = 262;
pub const IMAGE_RES_STRUCTURETECH: C2RustUnnamed_1 = 261;
pub const IMAGE_RES_SYSTEMTECH: C2RustUnnamed_1 = 260;
pub const IMAGE_RES_POWERTECH: C2RustUnnamed_1 = 259;
pub const IMAGE_RES_COMPUTERTECH: C2RustUnnamed_1 = 258;
pub const IMAGE_RES_WEAPONTECH: C2RustUnnamed_1 = 257;
pub const IMAGE_RES_DROIDTECH: C2RustUnnamed_1 = 256;
pub const IMAGE_QUESTION_MARK: C2RustUnnamed_1 = 255;
pub const IMAGE_RES_MAJOR_ROCKET: C2RustUnnamed_1 = 254;
pub const IMAGE_RES_MINOR_AUTOWEAPONS: C2RustUnnamed_1 = 253;
pub const IMAGE_TRACKS: C2RustUnnamed_1 = 252;
pub const IMAGE_STAR: C2RustUnnamed_1 = 251;
pub const IMAGE_SLIDER_BIG: C2RustUnnamed_1 = 250;
pub const IMAGE_DES_STATBACKMID: C2RustUnnamed_1 = 249;
pub const IMAGE_DES_STATBACKRIGHT: C2RustUnnamed_1 = 248;
pub const IMAGE_DES_STATBACKLEFT: C2RustUnnamed_1 = 247;
pub const IMAGE_SIDETABSEL: C2RustUnnamed_1 = 246;
pub const IMAGE_SIDETABDOWN: C2RustUnnamed_1 = 245;
pub const IMAGE_SIDETABHI: C2RustUnnamed_1 = 244;
pub const IMAGE_SIDETAB: C2RustUnnamed_1 = 243;
pub const IMAGE_ASCII126: C2RustUnnamed_1 = 227;
pub const IMAGE_ASCII125: C2RustUnnamed_1 = 226;
pub const IMAGE_ASCII124: C2RustUnnamed_1 = 225;
pub const IMAGE_ASCII123: C2RustUnnamed_1 = 224;
pub const IMAGE_ASCII122: C2RustUnnamed_1 = 223;
pub const IMAGE_ASCII121: C2RustUnnamed_1 = 222;
pub const IMAGE_ASCII120: C2RustUnnamed_1 = 221;
pub const IMAGE_ASCII119: C2RustUnnamed_1 = 220;
pub const IMAGE_ASCII118: C2RustUnnamed_1 = 219;
pub const IMAGE_ASCII117: C2RustUnnamed_1 = 218;
pub const IMAGE_ASCII116: C2RustUnnamed_1 = 217;
pub const IMAGE_ASCII115: C2RustUnnamed_1 = 216;
pub const IMAGE_ASCII114: C2RustUnnamed_1 = 215;
pub const IMAGE_ASCII113: C2RustUnnamed_1 = 214;
pub const IMAGE_ASCII112: C2RustUnnamed_1 = 213;
pub const IMAGE_ASCII111: C2RustUnnamed_1 = 212;
pub const IMAGE_ASCII110: C2RustUnnamed_1 = 211;
pub const IMAGE_ASCII109: C2RustUnnamed_1 = 210;
pub const IMAGE_ASCII108: C2RustUnnamed_1 = 209;
pub const IMAGE_ASCII107: C2RustUnnamed_1 = 208;
pub const IMAGE_ASCII106: C2RustUnnamed_1 = 207;
pub const IMAGE_ASCII105: C2RustUnnamed_1 = 206;
pub const IMAGE_ASCII104: C2RustUnnamed_1 = 205;
pub const IMAGE_ASCII103: C2RustUnnamed_1 = 204;
pub const IMAGE_ASCII102: C2RustUnnamed_1 = 203;
pub const IMAGE_ASCII101: C2RustUnnamed_1 = 202;
pub const IMAGE_ASCII100: C2RustUnnamed_1 = 201;
pub const IMAGE_ASCII99: C2RustUnnamed_1 = 200;
pub const IMAGE_ASCII98: C2RustUnnamed_1 = 199;
pub const IMAGE_ASCII97: C2RustUnnamed_1 = 198;
pub const IMAGE_ASCII96: C2RustUnnamed_1 = 197;
pub const IMAGE_ASCII95: C2RustUnnamed_1 = 196;
pub const IMAGE_ASCII94: C2RustUnnamed_1 = 195;
pub const IMAGE_ASCII93: C2RustUnnamed_1 = 194;
pub const IMAGE_ASCII92: C2RustUnnamed_1 = 193;
pub const IMAGE_ASCII91: C2RustUnnamed_1 = 192;
pub const IMAGE_ASCII90: C2RustUnnamed_1 = 191;
pub const IMAGE_ASCII89: C2RustUnnamed_1 = 190;
pub const IMAGE_ASCII88: C2RustUnnamed_1 = 189;
pub const IMAGE_ASCII87: C2RustUnnamed_1 = 188;
pub const IMAGE_ASCII86: C2RustUnnamed_1 = 187;
pub const IMAGE_ASCII85: C2RustUnnamed_1 = 186;
pub const IMAGE_ASCII84: C2RustUnnamed_1 = 185;
pub const IMAGE_ASCII83: C2RustUnnamed_1 = 184;
pub const IMAGE_ASCII82: C2RustUnnamed_1 = 183;
pub const IMAGE_ASCII81: C2RustUnnamed_1 = 182;
pub const IMAGE_ASCII80: C2RustUnnamed_1 = 181;
pub const IMAGE_ASCII79: C2RustUnnamed_1 = 180;
pub const IMAGE_ASCII78: C2RustUnnamed_1 = 179;
pub const IMAGE_ASCII77: C2RustUnnamed_1 = 178;
pub const IMAGE_ASCII76: C2RustUnnamed_1 = 177;
pub const IMAGE_ASCII75: C2RustUnnamed_1 = 176;
pub const IMAGE_ASCII74: C2RustUnnamed_1 = 175;
pub const IMAGE_ASCII73: C2RustUnnamed_1 = 174;
pub const IMAGE_ASCII72: C2RustUnnamed_1 = 173;
pub const IMAGE_ASCII71: C2RustUnnamed_1 = 172;
pub const IMAGE_ASCII70: C2RustUnnamed_1 = 171;
pub const IMAGE_ASCII69: C2RustUnnamed_1 = 170;
pub const IMAGE_ASCII68: C2RustUnnamed_1 = 169;
pub const IMAGE_ASCII67: C2RustUnnamed_1 = 168;
pub const IMAGE_ASCII66: C2RustUnnamed_1 = 167;
pub const IMAGE_ASCII65: C2RustUnnamed_1 = 166;
pub const IMAGE_ASTERISK: C2RustUnnamed_1 = 165;
pub const IMAGE_ASCII63: C2RustUnnamed_1 = 164;
pub const IMAGE_ASCII62: C2RustUnnamed_1 = 163;
pub const IMAGE_ASCII61: C2RustUnnamed_1 = 162;
pub const IMAGE_ASCII60: C2RustUnnamed_1 = 161;
pub const IMAGE_ASCII59: C2RustUnnamed_1 = 160;
pub const IMAGE_ASCII58: C2RustUnnamed_1 = 159;
pub const IMAGE_ASCII57: C2RustUnnamed_1 = 158;
pub const IMAGE_ASCII56: C2RustUnnamed_1 = 157;
pub const IMAGE_ASCII55: C2RustUnnamed_1 = 156;
pub const IMAGE_ASCII54: C2RustUnnamed_1 = 155;
pub const IMAGE_ASCII53: C2RustUnnamed_1 = 154;
pub const IMAGE_ASCII52: C2RustUnnamed_1 = 153;
pub const IMAGE_ASCII51: C2RustUnnamed_1 = 152;
pub const IMAGE_ASCII50: C2RustUnnamed_1 = 151;
pub const IMAGE_ASCII49: C2RustUnnamed_1 = 150;
pub const IMAGE_ASCII48: C2RustUnnamed_1 = 149;
pub const IMAGE_ASCII47: C2RustUnnamed_1 = 148;
pub const IMAGE_ASCII46: C2RustUnnamed_1 = 147;
pub const IMAGE_ASCII45: C2RustUnnamed_1 = 146;
pub const IMAGE_ASCII44: C2RustUnnamed_1 = 145;
pub const IMAGE_ASCII43: C2RustUnnamed_1 = 144;
pub const IMAGE_ASCII42: C2RustUnnamed_1 = 143;
pub const IMAGE_ASCII41: C2RustUnnamed_1 = 142;
pub const IMAGE_ASCII40: C2RustUnnamed_1 = 141;
pub const IMAGE_ASCII39: C2RustUnnamed_1 = 140;
pub const IMAGE_ASCII38: C2RustUnnamed_1 = 139;
pub const IMAGE_ASCII37: C2RustUnnamed_1 = 138;
pub const IMAGE_ASCII36: C2RustUnnamed_1 = 137;
pub const IMAGE_ASCII35: C2RustUnnamed_1 = 136;
pub const IMAGE_ASCII34: C2RustUnnamed_1 = 135;
pub const IMAGE_ASCII33: C2RustUnnamed_1 = 134;
pub const IMAGE_INTEL_CAMPAIGNDOWN: C2RustUnnamed_1 = 133;
pub const IMAGE_INTEL_CAMPAIGN: C2RustUnnamed_1 = 132;
pub const IMAGE_INTEL_MISSIONDOWN: C2RustUnnamed_1 = 131;
pub const IMAGE_INTEL_MISSION: C2RustUnnamed_1 = 130;
pub const IMAGE_INTEL_RESEARCHDOWN: C2RustUnnamed_1 = 129;
pub const IMAGE_INTEL_RESEARCH: C2RustUnnamed_1 = 128;
pub const IMAGE_DES_BARYELLOW: C2RustUnnamed_1 = 127;
pub const IMAGE_DES_BARRED: C2RustUnnamed_1 = 126;
pub const IMAGE_DES_BARBLUE: C2RustUnnamed_1 = 125;
pub const IMAGE_DES_BARBACK: C2RustUnnamed_1 = 124;
pub const IMAGE_9: C2RustUnnamed_1 = 123;
pub const IMAGE_8: C2RustUnnamed_1 = 122;
pub const IMAGE_7: C2RustUnnamed_1 = 121;
pub const IMAGE_6: C2RustUnnamed_1 = 120;
pub const IMAGE_5: C2RustUnnamed_1 = 119;
pub const IMAGE_4: C2RustUnnamed_1 = 118;
pub const IMAGE_3: C2RustUnnamed_1 = 117;
pub const IMAGE_2: C2RustUnnamed_1 = 116;
pub const IMAGE_1: C2RustUnnamed_1 = 115;
pub const IMAGE_0: C2RustUnnamed_1 = 114;
pub const IMAGE_RES_MAJOR_HOVER: C2RustUnnamed_1 = 113;
pub const IMAGE_RES_MAJOR_HEAVYWEP: C2RustUnnamed_1 = 112;
pub const IMAGE_RES_MAJOR_ELECTRONIC: C2RustUnnamed_1 = 111;
pub const IMAGE_RES_MAJOR_PLASCRETE: C2RustUnnamed_1 = 110;
pub const IMAGE_RES_MINOR_RADAR: C2RustUnnamed_1 = 109;
pub const IMAGE_PLASCRETE: C2RustUnnamed_1 = 108;
pub const IMAGE_ECM: C2RustUnnamed_1 = 107;
pub const IMAGE_RES_MINOR_PLASSTEEL: C2RustUnnamed_1 = 106;
pub const IMAGE_HOVERCRAFT: C2RustUnnamed_1 = 105;
pub const IMAGE_CANNON: C2RustUnnamed_1 = 104;
pub const IMAGE_ROCKET: C2RustUnnamed_1 = 103;
pub const IMAGE_DES_BODYPOINTS: C2RustUnnamed_1 = 102;
pub const IMAGE_DES_FIRERATE: C2RustUnnamed_1 = 101;
pub const IMAGE_DES_HOVER: C2RustUnnamed_1 = 100;
pub const IMAGE_DES_POWER: C2RustUnnamed_1 = 99;
pub const IMAGE_DES_DAMAGE: C2RustUnnamed_1 = 98;
pub const IMAGE_DES_WEIGHT: C2RustUnnamed_1 = 97;
pub const IMAGE_DES_ROAD: C2RustUnnamed_1 = 96;
pub const IMAGE_DES_CROSSCOUNTRY: C2RustUnnamed_1 = 95;
pub const IMAGE_DES_ARMOUR_EXPLOSIVE: C2RustUnnamed_1 = 94;
pub const IMAGE_DES_RANGE: C2RustUnnamed_1 = 93;
pub const IMAGE_DES_BUILDRATE: C2RustUnnamed_1 = 92;
pub const IMAGE_DES_TABWEAPONDOWN: C2RustUnnamed_1 = 91;
pub const IMAGE_DES_TABWEAPON: C2RustUnnamed_1 = 90;
pub const IMAGE_DES_EXTRAHI: C2RustUnnamed_1 = 89;
pub const IMAGE_DES_STATSCOMP: C2RustUnnamed_1 = 88;
pub const IMAGE_DES_BACK: C2RustUnnamed_1 = 87;
pub const IMAGE_FRAME_HC3: C2RustUnnamed_1 = 86;
pub const IMAGE_FRAME_HC2: C2RustUnnamed_1 = 85;
pub const IMAGE_FRAME_HC1: C2RustUnnamed_1 = 84;
pub const IMAGE_FRAME_HC0: C2RustUnnamed_1 = 83;
pub const IMAGE_FRAME_VC3: C2RustUnnamed_1 = 82;
pub const IMAGE_FRAME_VC2: C2RustUnnamed_1 = 81;
pub const IMAGE_FRAME_VC1: C2RustUnnamed_1 = 80;
pub const IMAGE_FRAME_VC0: C2RustUnnamed_1 = 79;
pub const IMAGE_DES_EDITBOXLEFTH: C2RustUnnamed_1 = 78;
pub const IMAGE_DES_EDITBOXLEFT: C2RustUnnamed_1 = 77;
pub const IMAGE_DES_POWERBACK: C2RustUnnamed_1 = 76;
pub const IMAGE_DES_STATSCURR: C2RustUnnamed_1 = 75;
pub const IMAGE_DES_STATSBACK: C2RustUnnamed_1 = 74;
pub const IMAGE_DES_POWERCURR: C2RustUnnamed_1 = 73;
pub const IMAGE_FRAME_HBH: C2RustUnnamed_1 = 72;
pub const IMAGE_FRAME_HTH: C2RustUnnamed_1 = 71;
pub const IMAGE_FRAME_HB2: C2RustUnnamed_1 = 70;
pub const IMAGE_FRAME_HT2: C2RustUnnamed_1 = 69;
pub const IMAGE_DES_COMMANDDOWN: C2RustUnnamed_1 = 68;
pub const IMAGE_DES_COMMAND: C2RustUnnamed_1 = 67;
pub const IMAGE_DES_SYSTEMSDOWN: C2RustUnnamed_1 = 66;
pub const IMAGE_DES_SYSTEMS: C2RustUnnamed_1 = 65;
pub const IMAGE_DES_WEAPONSDOWN: C2RustUnnamed_1 = 64;
pub const IMAGE_DES_WEAPONS: C2RustUnnamed_1 = 63;
pub const IMAGE_FRAME_VRH: C2RustUnnamed_1 = 62;
pub const IMAGE_FRAME_VLH: C2RustUnnamed_1 = 61;
pub const IMAGE_FRAME_VR2: C2RustUnnamed_1 = 60;
pub const IMAGE_FRAME_VL2: C2RustUnnamed_1 = 59;
pub const IMAGE_DES_HILIGHT: C2RustUnnamed_1 = 58;
pub const IMAGE_DES_PROPULSIONH: C2RustUnnamed_1 = 57;
pub const IMAGE_DES_PROPULSION: C2RustUnnamed_1 = 56;
pub const IMAGE_DES_BODYH: C2RustUnnamed_1 = 55;
pub const IMAGE_DES_BODY: C2RustUnnamed_1 = 54;
pub const IMAGE_DES_TURRETH: C2RustUnnamed_1 = 53;
pub const IMAGE_DES_TURRET: C2RustUnnamed_1 = 52;
pub const IMAGE_FRAME_VR: C2RustUnnamed_1 = 51;
pub const IMAGE_FRAME_VL: C2RustUnnamed_1 = 50;
pub const IMAGE_SLIDER_BACK: C2RustUnnamed_1 = 49;
pub const IMAGE_FRAME_HB: C2RustUnnamed_1 = 48;
pub const IMAGE_FRAME_HT: C2RustUnnamed_1 = 47;
pub const IMAGE_SLIDER_BUT: C2RustUnnamed_1 = 46;
pub const IMAGE_CLOSEHILIGHT: C2RustUnnamed_1 = 45;
pub const IMAGE_CLOSEDOWN: C2RustUnnamed_1 = 44;
pub const IMAGE_CLOSE: C2RustUnnamed_1 = 43;
pub const IMAGE_CANCEL_HILIGHT: C2RustUnnamed_1 = 42;
pub const IMAGE_CANCEL_DOWN: C2RustUnnamed_1 = 41;
pub const IMAGE_CANCEL_UP: C2RustUnnamed_1 = 40;
pub const IMAGE_RETICULE_BUTDOWN: C2RustUnnamed_1 = 39;
pub const IMAGE_RETICULE_HILIGHT: C2RustUnnamed_1 = 38;
pub const IMAGE_MANUFACTURE_DOWN: C2RustUnnamed_1 = 37;
pub const IMAGE_MANUFACTURE_UP: C2RustUnnamed_1 = 36;
pub const IMAGE_RESEARCH_DOWN: C2RustUnnamed_1 = 35;
pub const IMAGE_RESEARCH_UP: C2RustUnnamed_1 = 34;
pub const IMAGE_BUILD_DOWN: C2RustUnnamed_1 = 33;
pub const IMAGE_BUILD_UP: C2RustUnnamed_1 = 32;
pub const IMAGE_DESIGN_DOWN: C2RustUnnamed_1 = 31;
pub const IMAGE_DESIGN_UP: C2RustUnnamed_1 = 30;
pub const IMAGE_COMMANDDROID_DOWN: C2RustUnnamed_1 = 29;
pub const IMAGE_COMMANDDROID_UP: C2RustUnnamed_1 = 28;
pub const IMAGE_INTELMAP_DOWN: C2RustUnnamed_1 = 27;
pub const IMAGE_INTELMAP_UP: C2RustUnnamed_1 = 26;
pub const IMAGE_PBAR_BOTTOM: C2RustUnnamed_1 = 25;
pub const IMAGE_PBAR_TOP: C2RustUnnamed_1 = 24;
pub const IMAGE_FRAME_C3: C2RustUnnamed_1 = 23;
pub const IMAGE_FRAME_C2: C2RustUnnamed_1 = 22;
pub const IMAGE_FRAME_C1: C2RustUnnamed_1 = 21;
pub const IMAGE_FRAME_C0: C2RustUnnamed_1 = 20;
pub const IMAGE_TABHILIGHT: C2RustUnnamed_1 = 19;
pub const IMAGE_TABSELECTED: C2RustUnnamed_1 = 18;
pub const IMAGE_TAB4DOWN: C2RustUnnamed_1 = 17;
pub const IMAGE_TAB3DOWN: C2RustUnnamed_1 = 16;
pub const IMAGE_TAB2DOWN: C2RustUnnamed_1 = 15;
pub const IMAGE_TAB1DOWN: C2RustUnnamed_1 = 14;
pub const IMAGE_TAB4: C2RustUnnamed_1 = 13;
pub const IMAGE_TAB3: C2RustUnnamed_1 = 12;
pub const IMAGE_TAB2: C2RustUnnamed_1 = 11;
pub const IMAGE_TAB1: C2RustUnnamed_1 = 10;
pub const IMAGE_BUTB_HILITE: C2RustUnnamed_1 = 9;
pub const IMAGE_BUT_HILITE: C2RustUnnamed_1 = 8;
pub const IMAGE_BUTB0_DOWN: C2RustUnnamed_1 = 7;
pub const IMAGE_BUTB0_UP: C2RustUnnamed_1 = 6;
pub const IMAGE_BUT0_DOWN: C2RustUnnamed_1 = 5;
pub const IMAGE_BUT0_UP: C2RustUnnamed_1 = 4;
pub const IMAGE_PBAR_REQUIRED: C2RustUnnamed_1 = 3;
pub const IMAGE_PBAR_USED: C2RustUnnamed_1 = 2;
pub const IMAGE_PBAR_AVAIL: C2RustUnnamed_1 = 1;
pub const IMAGE_PBAR_EMPTY: C2RustUnnamed_1 = 0;
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
pub const CALL_GAMEINIT: _scr_callback_types = 5;
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_NORMAL: _group_type = 0;
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
    return psMapTiles.offset(x as
                                 isize).offset(y.wrapping_mul(mapWidth) as
                                                   isize);
}
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
//BOOL driveHasDriven(void);
//BOOL driveModeActive(void);
//BOOL driveIsDriven(DROID *psDroid);
//BOOL driveIsFollower(DROID *psDroid);
#[inline]
unsafe extern "C" fn driveGetDriven() -> *mut DROID { return psDrivenDroid; }
#[no_mangle]
pub static mut dragBox3D: _dragBox =
    _dragBox{x1: 0,
             y1: 0,
             x2: 0,
             y2: 0,
             status: 0,
             lastTime: 0,
             boxColourIndex: 0,};
#[no_mangle]
pub static mut wallDrag: _dragBox =
    _dragBox{x1: 0,
             y1: 0,
             x2: 0,
             y2: 0,
             status: 0,
             lastTime: 0,
             boxColourIndex: 0,};
// control whether the scroll is limited to visible tiles
//#define VISIBLE_SCROLL
#[no_mangle]
pub static mut arnMPointers: [[UDWORD; 13]; 23] =
    [[121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 112 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [114 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 113 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 113 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [116 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [122 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [114 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD],
     [116 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD, 124 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      124 as libc::c_int as UDWORD],
     [121 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 118 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 112 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [115 as libc::c_int as UDWORD, 115 as libc::c_int as UDWORD,
      115 as libc::c_int as UDWORD, 115 as libc::c_int as UDWORD,
      115 as libc::c_int as UDWORD, 115 as libc::c_int as UDWORD,
      115 as libc::c_int as UDWORD, 115 as libc::c_int as UDWORD,
      115 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      115 as libc::c_int as UDWORD],
     [125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD],
     [125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      116 as libc::c_int as UDWORD],
     [121 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 118 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [125 as libc::c_int as UDWORD, 110 as libc::c_int as UDWORD,
      110 as libc::c_int as UDWORD, 110 as libc::c_int as UDWORD,
      110 as libc::c_int as UDWORD, 110 as libc::c_int as UDWORD,
      110 as libc::c_int as UDWORD, 110 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      110 as libc::c_int as UDWORD],
     [123 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      123 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      123 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      123 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      123 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 123 as libc::c_int as UDWORD,
      123 as libc::c_int as UDWORD],
     [122 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 111 as libc::c_int as UDWORD,
      111 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [125 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      116 as libc::c_int as UDWORD],
     [101 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD],
     [125 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 125 as libc::c_int as UDWORD,
      125 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      116 as libc::c_int as UDWORD],
     [114 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD],
     [116 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      119 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 117 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD, 121 as libc::c_int as UDWORD,
      117 as libc::c_int as UDWORD, 122 as libc::c_int as UDWORD,
      122 as libc::c_int as UDWORD, 101 as libc::c_int as UDWORD,
      121 as libc::c_int as UDWORD]];
#[no_mangle]
pub static mut MPointerImageIDs: [UWORD; 25] =
    [IMAGE_CURSOR_DEST as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_ATTACH as libc::c_int as UWORD,
     IMAGE_CURSOR_ATTACK as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_BRIDGE as libc::c_int as UWORD,
     IMAGE_CURSOR_BUILD as libc::c_int as UWORD,
     IMAGE_CURSOR_EMBARK as libc::c_int as UWORD,
     IMAGE_CURSOR_FIX as libc::c_int as UWORD,
     IMAGE_CURSOR_GUARD as libc::c_int as UWORD,
     IMAGE_CURSOR_ECM as libc::c_int as UWORD,
     IMAGE_CURSOR_LOCKON as libc::c_int as UWORD,
     IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD,
     IMAGE_CURSOR_MOVE as libc::c_int as UWORD,
     IMAGE_CURSOR_NOTPOS as libc::c_int as UWORD,
     IMAGE_CURSOR_PICKUP as libc::c_int as UWORD,
     IMAGE_CURSOR_REPAIR as libc::c_int as UWORD,
     IMAGE_CURSOR_SELECT as libc::c_int as UWORD];
#[no_mangle]
pub static mut scroll_speed_accel: UDWORD = 800 as libc::c_int as UDWORD;
//static BOOL ctrlShiftDown(void);
static mut bInvertMouse: BOOL = 1 as libc::c_int;
static mut bDrawShadows: BOOL = 1 as libc::c_int;
/* Mouse x and y - no point checking them more than once per frame */
#[no_mangle]
pub static mut mX: SDWORD = 9999 as libc::c_int;
#[no_mangle]
pub static mut mY: SDWORD = 9999 as libc::c_int;
#[no_mangle]
pub static mut bInstantRadarJump: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut rotActive: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut desiredPitch: SDWORD = 340 as libc::c_int;
#[no_mangle]
pub static mut currentFrame: UDWORD = 0;
static mut StartOfLastFrame: UDWORD = 0;
#[no_mangle]
pub static mut rotX: SDWORD = 0;
#[no_mangle]
pub static mut rotY: SDWORD = 0;
#[no_mangle]
pub static mut worldAngle: UDWORD = 0;
#[no_mangle]
pub static mut rotInitial: UDWORD = 0;
#[no_mangle]
pub static mut rotInitialUp: UDWORD = 0;
#[no_mangle]
pub static mut xMoved: UDWORD = 0;
#[no_mangle]
pub static mut yMoved: UDWORD = 0;
//DROID	*psSelected3D = NULL;
#[no_mangle]
pub static mut gameStats: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut psBuilding: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
#[no_mangle]
pub static mut mouseAtEdge: BOOL = 0 as libc::c_int;
/* Will be used to do diagonal scroll */
#[no_mangle]
pub static mut mouseAtBottom: BOOL = 0;
#[no_mangle]
pub static mut mouseAtTop: BOOL = 0;
#[no_mangle]
pub static mut mouseAtRight: BOOL = 0;
#[no_mangle]
pub static mut mouseAtLeft: BOOL = 0;
#[no_mangle]
pub static mut direction: SDWORD = 0 as libc::c_int;
#[no_mangle]
pub static mut edgeOfMap: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut scrollRefTime: UDWORD = 0;
#[no_mangle]
pub static mut scrollAccel: libc::c_float = 0.;
#[no_mangle]
pub static mut scrollSpeedLeftRight: libc::c_float = 0.;
//use two directions and add them because its simple
#[no_mangle]
pub static mut scrollStepLeftRight: libc::c_float = 0.;
#[no_mangle]
pub static mut scrollSpeedUpDown: libc::c_float = 0.;
#[no_mangle]
pub static mut scrollStepUpDown: libc::c_float = 0.;
#[no_mangle]
pub static mut noDrag3D: BOOL = 0;
#[no_mangle]
pub static mut mouseOverRadar: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut mouseOverConsole: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ignoreOrder: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ignoreRMBC: BOOL = 1 as libc::c_int;
/* Hackety hack hack hack */
/* Has the big blue droid been added to the world? */
#[no_mangle]
pub static mut bigBlueInWorld: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut missionComplete: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut RadarZoomLevel: UWORD = 0 as libc::c_int as UWORD;
#[no_mangle]
pub static mut gammaValue: libc::c_float = 1.0f64 as libc::c_float;
#[no_mangle]
pub static mut psSelectedVtol: *mut DROID = 0 as *const DROID as *mut DROID;
#[no_mangle]
pub static mut psDominantSelected: *mut DROID =
    0 as *const DROID as *mut DROID;
/* Hackety hack hack hack */
#[no_mangle]
pub static mut screenShakeTable: [SDWORD; 100] =
    [-(2 as libc::c_int), -(2 as libc::c_int), -(3 as libc::c_int),
     -(4 as libc::c_int), -(3 as libc::c_int), -(3 as libc::c_int),
     -(5 as libc::c_int), -(4 as libc::c_int), -(4 as libc::c_int),
     -(4 as libc::c_int), -(4 as libc::c_int), -(5 as libc::c_int),
     -(5 as libc::c_int), -(5 as libc::c_int), -(5 as libc::c_int),
     -(7 as libc::c_int), -(5 as libc::c_int), -(6 as libc::c_int),
     -(8 as libc::c_int), -(6 as libc::c_int), -(7 as libc::c_int),
     -(8 as libc::c_int), -(6 as libc::c_int), -(4 as libc::c_int),
     -(8 as libc::c_int), -(7 as libc::c_int), -(7 as libc::c_int),
     -(7 as libc::c_int), -(6 as libc::c_int), -(5 as libc::c_int),
     -(6 as libc::c_int), -(5 as libc::c_int), -(2 as libc::c_int),
     -(5 as libc::c_int), -(6 as libc::c_int), -(3 as libc::c_int),
     -(5 as libc::c_int), -(3 as libc::c_int), -(2 as libc::c_int),
     -(4 as libc::c_int), -(5 as libc::c_int), -(3 as libc::c_int),
     -(2 as libc::c_int), -(0 as libc::c_int), 1 as libc::c_int,
     2 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
     0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int,
     3 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
     2 as libc::c_int, 3 as libc::c_int, 4 as libc::c_int, 4 as libc::c_int,
     2 as libc::c_int, 6 as libc::c_int, 4 as libc::c_int, 5 as libc::c_int,
     3 as libc::c_int, 7 as libc::c_int, 7 as libc::c_int, 3 as libc::c_int,
     6 as libc::c_int, 4 as libc::c_int, 7 as libc::c_int, 9 as libc::c_int,
     10 as libc::c_int, 9 as libc::c_int, 8 as libc::c_int, 6 as libc::c_int,
     4 as libc::c_int, 7 as libc::c_int, 5 as libc::c_int, 5 as libc::c_int,
     4 as libc::c_int, 6 as libc::c_int, 2 as libc::c_int, 4 as libc::c_int,
     5 as libc::c_int, 3 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int,
     1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int,
     -(1 as libc::c_int), -(1 as libc::c_int), -(2 as libc::c_int),
     -(1 as libc::c_int), 1 as libc::c_int, 0 as libc::c_int,
     1 as libc::c_int, 0 as libc::c_int];
#[no_mangle]
pub static mut gamePalette: [PALETTEENTRY; 255] =
    [PALETTEENTRY{peRed: 0, peGreen: 0, peBlue: 0, peFlags: 0,}; 255];
// another game palette (yawn)
static mut bScreenShakeActive: BOOL = 0 as libc::c_int;
static mut screenShakeStarted: UDWORD = 0;
static mut screenShakeLength: UDWORD = 0;
//used to determine is a weapon droid is assigned to a sensor tower or sensor droid
static mut bSensorAssigned: BOOL = 0;
//used to determine if the player has selected a Las Sat structure
static mut bLasSatStruct: BOOL = 0;
#[no_mangle]
pub static mut bShakingPermitted: BOOL = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn setRadarJump(mut val: BOOL) {
    bInstantRadarJump = val;
}
#[no_mangle]
pub unsafe extern "C" fn getRadarJumpStatus() -> BOOL {
    return bInstantRadarJump;
}
//access function for bSensorAssigned variable
#[no_mangle]
pub unsafe extern "C" fn getShakeStatus() -> BOOL {
    return bShakingPermitted;
}
#[no_mangle]
pub unsafe extern "C" fn getInvertMouseStatus() -> BOOL {
    return bInvertMouse;
}
#[no_mangle]
pub unsafe extern "C" fn setInvertMouseStatus(mut val: BOOL) {
    bInvertMouse = val;
}
#[no_mangle]
pub unsafe extern "C" fn getDrawShadows() -> BOOL { return bDrawShadows; }
#[no_mangle]
pub unsafe extern "C" fn setDrawShadows(mut val: BOOL) { bDrawShadows = val; }
#[no_mangle]
pub unsafe extern "C" fn setShakeStatus(mut val: BOOL) {
    bShakingPermitted = val;
}
#[no_mangle]
pub unsafe extern "C" fn shakeStart() {
    if bShakingPermitted != 0 {
        if bScreenShakeActive == 0 {
            bScreenShakeActive = 1 as libc::c_int;
            screenShakeStarted = gameTime;
            screenShakeLength = 1500 as libc::c_int as UDWORD
            //1500;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn shakeStop() {
    bScreenShakeActive = 0 as libc::c_int;
    player.r.z = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn shakeUpdate() {
    let mut screenShakePercentage: UDWORD = 0;
    /* Check if we're shaking the screen or not */
    if bScreenShakeActive != 0 {
        //		screenShakePercentage = (((gameTime-screenShakeStarted)<<8) / screenShakeLength) * 100;
        screenShakePercentage =
            gameTime2.wrapping_sub(screenShakeStarted).wrapping_mul(100 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint).wrapping_div(screenShakeLength);
        //		screenShakePercentage = screenShakePercentage >> 8;
        if screenShakePercentage < 100 as libc::c_int as libc::c_uint {
            player.r.z =
                0 as libc::c_int +
                    65536 as libc::c_int / 360 as libc::c_int *
                        screenShakeTable[screenShakePercentage as usize]
        }
        if gameTime > screenShakeStarted.wrapping_add(screenShakeLength) {
            bScreenShakeActive = 0 as libc::c_int;
            player.r.z = 0 as libc::c_int
        }
    } else if getWarCamStatus() == 0 { player.r.z = 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn LoadLevelGraphics(mut LevelNumber: UBYTE) -> BOOL {
    return 1 as libc::c_int;
}
/* Initialise the display system */
#[no_mangle]
pub unsafe extern "C" fn dispInitialise() -> BOOL {
    /*	UBYTE	*pFileData, *pImageData;
	UDWORD	fileSize, width,height;
	UDWORD i;*/
    /*	Build the transparency table that's
		used for the rgb filter rectangle plotter in ivis */
    iV_SetTransFilter.expect("non-null function pointer")(2 as libc::c_int as
                                                              UDWORD,
                                                          0 as libc::c_int as
                                                              UDWORD); // set the table.
    iV_SetTransFilter.expect("non-null function pointer")(3 as libc::c_int as
                                                              UDWORD,
                                                          1 as libc::c_int as
                                                              UDWORD); // set the other table.
    iV_SetTransFilter.expect("non-null function pointer")(0 as libc::c_int as
                                                              UDWORD,
                                                          2 as libc::c_int as
                                                              UDWORD); // set the other other table.
    iV_SetTransFilter.expect("non-null function pointer")(4 as libc::c_int as
                                                              UDWORD,
                                                          3 as libc::c_int as
                                                              UDWORD); // set the other other table.
    //	screenSetTextColour(0xff,0xff,0xff);
    noDrag3D = 0 as libc::c_int;
    RadarZoomLevel = 0 as libc::c_int as UWORD;
    return 1 as libc::c_int;
}
/* Tidy up after a mode change */
#[no_mangle]
pub unsafe extern "C" fn dispModeChange() -> BOOL { return 1 as libc::c_int; }
#[no_mangle]
pub unsafe extern "C" fn GetMouseOverRadar() -> BOOL {
    return mouseOverRadar;
}
#[no_mangle]
pub unsafe extern "C" fn ClearMouseOverRadar() {
    mouseOverRadar = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetMouseOverRadar() {
    mouseOverRadar = 1 as libc::c_int;
}
#[no_mangle]
pub static mut bRadarDragging: BOOL = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn ProcessRadarInput() {
    let mut PosX: SDWORD = 0;
    let mut PosY: SDWORD = 0;
    let mut x: libc::c_int = mouseX();
    let mut y: libc::c_int = mouseY();
    let mut temp1: UDWORD = 0;
    let mut temp2: UDWORD = 0;
    /* Only allow jump-to-area-of-map if radar is on-screen */
    mouseOverRadar = 0 as libc::c_int;
    if radarOnScreen != 0 && getHQExists(selectedPlayer) != 0 {
        if CoordInRadar(x, y) != 0 {
            mouseOverRadar = 1 as libc::c_int;
            if mousePressed(MOUSE_LMB) != 0 {
                if driveModeActive() != 0 {
                    driveProcessRadarInput(x, y);
                } else {
                    /* If we're tracking a droid, then cancel that */
//					if(getWarCamStatus() == TRUE)
//					{
//						camToggleStatus();
//					}
                    CalcRadarPosition(x as UDWORD, y as UDWORD,
                                      &mut PosX as *mut SDWORD as *mut UDWORD,
                                      &mut PosY as *mut SDWORD as
                                          *mut UDWORD);
                    if mouseOverRadar != 0 {
                        //	requestRadarTrack(PosX*TILE_UNITS,PosY*TILE_UNITS);
						// MARKER
						// Send all droids to that location
                        orderSelectedLoc(selectedPlayer,
                                         (PosX * 128 as libc::c_int +
                                              128 as libc::c_int /
                                                  2 as libc::c_int) as UDWORD,
                                         (PosY * 128 as libc::c_int +
                                              128 as libc::c_int /
                                                  2 as libc::c_int) as
                                             UDWORD);
                    }
                    //				setViewPos(PosX,PosY);
                    CheckScrollLimits();
                    audio_PlayTrack(ID_SOUND_MESSAGEEND as libc::c_int);
                }
            }
            if mouseDrag(MOUSE_RMB, &mut temp1, &mut temp2) != 0 &&
                   rotActive == 0 {
                CalcRadarPosition(x as UDWORD, y as UDWORD,
                                  &mut PosX as *mut SDWORD as *mut UDWORD,
                                  &mut PosY as *mut SDWORD as *mut UDWORD);
                setViewPos(PosX as UDWORD, PosY as UDWORD, 1 as libc::c_int);
                bRadarDragging = 1 as libc::c_int;
                if keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 {
                    player.r.y = 0 as libc::c_int
                }
            } else if mousePressed(MOUSE_RMB) != 0 {
                /*
   				RadarZoomLevel++;
   				if(RadarZoomLevel > MAX_RADARZOOM) {
   					RadarZoomLevel = 0;
   				}
   				SetRadarZoom(RadarZoomLevel);
   				audio_PlayTrack( ID_SOUND_BUTTON_CLICK_5 );
				*/
                CalcRadarPosition(x as UDWORD, y as UDWORD,
                                  &mut PosX as *mut SDWORD as *mut UDWORD,
                                  &mut PosY as *mut SDWORD as *mut UDWORD);
                if bInstantRadarJump != 0 {
                    /* Go instantly */
                    setViewPos(PosX as UDWORD, PosY as UDWORD,
                               1 as libc::c_int);
                } else {
                    /* Pan to it */
                    requestRadarTrack(PosX * 128 as libc::c_int,
                                      PosY * 128 as libc::c_int);
                }
            }
        }
    };
}
// reset the input state
#[no_mangle]
pub unsafe extern "C" fn resetInput() {
    rotActive = 0 as libc::c_int;
    dragBox3D.status = 0 as libc::c_int as UDWORD;
    wallDrag.status = 0 as libc::c_int as UDWORD;
}
/* Process the user input. This just processes the key input and jumping around the radar*/
//BOOL processInput(void)
#[no_mangle]
pub unsafe extern "C" fn processInput() {
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut mOverR: BOOL = 0 as libc::c_int;
    let mut WheelZoomIterator: libc::c_int = 0;
    x = mouseX() as UDWORD;
    y = mouseY() as UDWORD;
    if radarOnScreen != 0 && getHQExists(selectedPlayer) != 0 {
        if CoordInRadar(x as libc::c_int, y as libc::c_int) != 0 {
            mOverR = 1 as libc::c_int
        }
    }
    StartOfLastFrame = currentFrame;
    currentFrame = frameGetFrameNumber();
    psBuilding = 0 as *mut STRUCTURE;
    mouseAtEdge = 0 as libc::c_int;
    ResetMouseBoundryConditions();
    edgeOfMap = 0 as libc::c_int;
    ignoreRMBC = 0 as libc::c_int;
    mX = mouseX();
    mY = mouseY();
    /* Process all of our key mappings */
//	keyProcessMappings();	// done later - see below.
    //	if(mousePressed(MOUSE_LMB) AND getRadarTrackingStatus() AND !mOverR)
    if mousePressed(MOUSE_LMB) != 0 && mOverR == 0 &&
           getRadarTrackingStatus() != 0 {
        camToggleStatus();
    }
    //	if(mousePressed(MOUSE_RMB) AND getRadarTrackingStatus() AND !mOverR)
    if mousePressed(MOUSE_LMB) != 0 && mOverR == 0 &&
           getRadarTrackingStatus() != 0 {
        camToggleStatus();
    }
    if mousePressed(MOUSE_WUP) != 0 {
        WheelZoomIterator = 0 as libc::c_int;
        while WheelZoomIterator < 10 as libc::c_int {
            kf_ZoomIn();
            WheelZoomIterator += 1
        }
    }
    if mousePressed(MOUSE_WDN) != 0 {
        WheelZoomIterator = 0 as libc::c_int;
        while WheelZoomIterator < 10 as libc::c_int {
            kf_ZoomOut();
            WheelZoomIterator += 1
        }
    }
    if intMode as libc::c_uint != INT_DESIGN as libc::c_int as libc::c_uint {
        if bAllowOtherKeyPresses != 0 {
            /* Run all standard mappings */
            keyProcessMappings(0 as libc::c_int);
        } else {
            kf_SendTextMessage();
            // process multiplayer chat message.
        }
    } else {
        /* Only process the function keys */
        keyProcessMappings(1 as libc::c_int);
    }
    /* Allow the user to clear the console if need be */
    mouseOverConsole = mouseOverConsoleBox();
    if mouseOverConsole != 0 && mousePressed(MOUSE_LMB) != 0 {
        setConsolePermanence(0 as libc::c_int, 1 as libc::c_int);
    };
    /*   	if(keyDown(KEY_F1))
	{
		theSun.x+=64;
		initLighting();
	}

	if(keyDown(KEY_F2))
	{
		theSun.x-=64;
		initLighting();
	}

	if(keyDown(KEY_F3))
	{
		theSun.y+=64;
		initLighting();
	}

	if(keyDown(KEY_F4))
	{
		theSun.y-=64;
		initLighting();
	}

	if(keyDown(KEY_F5))
	{
		theSun.z+=64;
		initLighting();
	}

	if(keyDown(KEY_F6))
	{
		theSun.z-=64;
		initLighting();
	}
*/
}
// //don't want to do any of these whilst in the Intelligence Screen
//void processMouseClickInput(void)
//{
//	UDWORD	dragX,dragY,i;
//	SELECTION_TYPE	selection;
//	MOUSE_TARGET	item;
//	BOOL OverRadar = mouseOverRadar;
//
//	/* Have we tried to click on something - only used to signal to display3d.c */
//	if (mouseDown(MOUSE_LMB))
//	{
//		selectAttempt = TRUE;
//	}
//
//	if(mouseDown(MOUSE_RMB) AND	rotActive)
//	{
//  		if(abs(mX-rotX)>8)
//		{
//			if(mX<rotX)
//			{
//				player.r.y = rotInitial + ( ((rotX-mX)/4) * DEG(1) );
//			}
//			else
//			{
//				player.r.y = rotInitial - ( ((mX-rotX)/4) * DEG(1) );
//		   	}
//		}
//	}
//
//	if(mouseReleased(MOUSE_RMB) AND rotActive)
//	{
//		rotActive = FALSE;
//		ignoreRMBC = TRUE;
//	}
//}
#[no_mangle]
pub unsafe extern "C" fn OverRadarAndNotDragging() -> BOOL {
    let mut OverRadar: BOOL = mouseOverRadar;
    if getHQExists(selectedPlayer) == 0 as libc::c_int {
        return 0 as libc::c_int
    }
    if dragBox3D.status == 1 as libc::c_int as libc::c_uint ||
           wallDrag.status == 1 as libc::c_int as libc::c_uint {
        OverRadar = 0 as libc::c_int
    }
    return OverRadar;
}
#[no_mangle]
pub unsafe extern "C" fn CheckFinishedDrag() {
    if driveModeActive() != 0 { return }
    if mouseReleased(MOUSE_LMB) != 0 {
        selectAttempt = 0 as libc::c_int;
        if dragBox3D.status == 1 as libc::c_int as libc::c_uint {
            if wallDrag.status == 1 as libc::c_int as libc::c_uint {
                //if invalid location keep looking for a valid one
                if buildState == 102 as libc::c_int as libc::c_uint ||
                       buildState == 101 as libc::c_int as libc::c_uint {
                    if (*(sBuildDetails.psStats as
                              *mut STRUCTURE_STATS)).type_0 ==
                           REF_WALL as libc::c_int as libc::c_uint ||
                           (*(sBuildDetails.psStats as
                                  *mut STRUCTURE_STATS)).type_0 ==
                               REF_DEFENSE as libc::c_int as libc::c_uint {
                        wallDrag.x2 = mouseTileX as UDWORD;
                        wallDrag.y2 = mouseTileY as UDWORD;
                        wallDrag.status = 2 as libc::c_int as UDWORD
                    }
                }
            }
            /* Only clear if shift isn't down - this is for the drag selection box for units*/
            if keyDown(KEY_LCTRL) == 0 && keyDown(KEY_RCTRL) == 0 &&
                   keyDown(KEY_LSHIFT) == 0 && keyDown(KEY_RSHIFT) == 0 &&
                   wallDrag.status == 0 as libc::c_int as libc::c_uint {
                clearSelection();
            }
            dragBox3D.status = 2 as libc::c_int as UDWORD;
            dragBox3D.x2 = mX as UDWORD;
            dragBox3D.y2 = mY as UDWORD
        } else {
            dragBox3D.status = 0 as libc::c_int as UDWORD;
            wallDrag.status = 0 as libc::c_int as UDWORD
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckStartWallDrag() {
    if driveModeActive() != 0 { return }
    if mousePressed(MOUSE_LMB) != 0 {
        /* Store away the details if we're building */
		// You can start dragging walls from invalid locations so check for
		// BUILD3D_POS or BUILD3D_VALID, used tojust check for BUILD3D_VALID.
        if buildState == 100 as libc::c_int as libc::c_uint ||
               buildState == 102 as libc::c_int as libc::c_uint {
            //if(((STRUCTURE_STATS *)sBuildDetails.psStats)->type >= REF_WALLH AND
			//	((STRUCTURE_STATS *)sBuildDetails.psStats)->type <= REF_WALLV)
            if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint ||
                   (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0
                       == REF_DEFENSE as libc::c_int as libc::c_uint {
                wallDrag.x2 = mouseTileX as UDWORD;
                wallDrag.x1 = wallDrag.x2;
                wallDrag.y2 = mouseTileY as UDWORD;
                wallDrag.y1 = wallDrag.y2;
                wallDrag.status = 3 as libc::c_int as UDWORD;
                debug(LOG_NEVER,
                      b"Start Wall Drag\n\x00" as *const u8 as
                          *const libc::c_char);
            }
        } else if intBuildSelectMode() != 0 {
            //if we were in build select mode
            //uhoh no place to build here
            audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
        }
    };
}
//BOOL CheckFinishedWallDrag(void) - misleading name - sorry Paul!
//this function is called when a location has been chosen to place a structure or a DP
#[no_mangle]
pub unsafe extern "C" fn CheckFinishedFindPosition() -> BOOL {
    let mut OverRadar: BOOL = OverRadarAndNotDragging();
    //	if(driveModeActive()) {
//		return FALSE;
//	}
    //	if (buildState == BUILD3D_VALID)
//	{
		/* Do not let the player position buildings 'under' the radar */
    if mouseReleased(MOUSE_LMB) != 0 && OverRadar == 0 {
        if buildState == 102 as libc::c_int as libc::c_uint {
            if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint ||
                   (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0
                       == REF_DEFENSE as libc::c_int as libc::c_uint {
                let mut dx: libc::c_int = 0;
                let mut dy: libc::c_int = 0;
                wallDrag.x2 = mouseTileX as UDWORD;
                wallDrag.y2 = mouseTileY as UDWORD;
                dx =
                    abs((mouseTileX as libc::c_uint).wrapping_sub(wallDrag.x1)
                            as libc::c_int);
                dy =
                    abs((mouseTileY as libc::c_uint).wrapping_sub(wallDrag.y1)
                            as libc::c_int);
                if dx >= dy {
                    wallDrag.y2 = wallDrag.y1
                } else if dx < dy { wallDrag.x2 = wallDrag.x1 }
                wallDrag.status = 2 as libc::c_int as UDWORD
            }
            debug(LOG_NEVER,
                  b"BUILD3D_FINISHED\n\x00" as *const u8 as
                      *const libc::c_char);
            buildState = 101 as libc::c_int as UDWORD;
            return 1 as libc::c_int
        }
    }
    //	}
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HandleDrag() {
    let mut OverRadar: BOOL = mouseOverRadar;
    let mut dragX: UDWORD = 0;
    let mut dragY: UDWORD = 0;
    if driveModeActive() != 0 {
        if mouseDown(MOUSE_LMB) != 0 {
            if buildState == 102 as libc::c_int as libc::c_uint {
                if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0
                       == REF_WALL as libc::c_int as libc::c_uint ||
                       (*(sBuildDetails.psStats as
                              *mut STRUCTURE_STATS)).type_0 ==
                           REF_DEFENSE as libc::c_int as libc::c_uint {
                    let mut dx: libc::c_int = 0;
                    let mut dy: libc::c_int = 0;
                    wallDrag.x2 = mouseTileX as UDWORD;
                    wallDrag.y2 = mouseTileY as UDWORD;
                    dx =
                        abs((mouseTileX as
                                 libc::c_uint).wrapping_sub(wallDrag.x1) as
                                libc::c_int);
                    dy =
                        abs((mouseTileY as
                                 libc::c_uint).wrapping_sub(wallDrag.y1) as
                                libc::c_int);
                    if dx >= dy {
                        wallDrag.y2 = wallDrag.y1
                    } else if dx < dy { wallDrag.x2 = wallDrag.x1 }
                    wallDrag.status = 1 as libc::c_int as UDWORD
                }
            }
        }
        return
    }
    if mouseDrag(MOUSE_LMB, &mut dragX, &mut dragY) != 0 && OverRadar == 0 &&
           mouseDown(MOUSE_RMB) == 0 {
        dragBox3D.x1 = dragX;
        dragBox3D.x2 = mX as UDWORD;
        dragBox3D.y1 = dragY;
        dragBox3D.y2 = mY as UDWORD;
        if buildState == 102 as libc::c_int as libc::c_uint {
            if (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint ||
                   (*(sBuildDetails.psStats as *mut STRUCTURE_STATS)).type_0
                       == REF_DEFENSE as libc::c_int as libc::c_uint {
                let mut dx_0: libc::c_int = 0;
                let mut dy_0: libc::c_int = 0;
                wallDrag.x2 = mouseTileX as UDWORD;
                wallDrag.y2 = mouseTileY as UDWORD;
                dx_0 =
                    abs((mouseTileX as libc::c_uint).wrapping_sub(wallDrag.x1)
                            as libc::c_int);
                dy_0 =
                    abs((mouseTileY as libc::c_uint).wrapping_sub(wallDrag.y1)
                            as libc::c_int);
                if dx_0 >= dy_0 {
                    wallDrag.y2 = wallDrag.y1
                } else if dx_0 < dy_0 { wallDrag.x2 = wallDrag.x1 }
                wallDrag.status = 1 as libc::c_int as UDWORD
            }
        }
        dragBox3D.status = 1 as libc::c_int as UDWORD
    };
}
static mut CurrentItemUnderMouse: UDWORD = 0;
#[no_mangle]
pub unsafe extern "C" fn getTargetType() -> UDWORD {
    return CurrentItemUnderMouse;
}
//don't want to do any of these whilst in the Intelligence Screen
#[no_mangle]
pub unsafe extern "C" fn processMouseClickInput() {
    let mut i: UDWORD = 0;
    let mut selection: SELECTION_TYPE = SC_DROID_CONSTRUCT;
    let mut item: MOUSE_TARGET = MT_NOTARGET;
    let mut OverRadar: BOOL = OverRadarAndNotDragging();
    // These four functions were embedded in this function but I moved them out for readability. In the
// absense of any comments I had a guess as to there use and named them accordingly PD 28/05/98.
	//ignoreOrder = CheckFinishedWallDrag(); - this name is misleading since called for all Structures AB
    ignoreOrder = CheckFinishedFindPosition();
    CheckStartWallDrag();
    HandleDrag();
    CheckFinishedDrag();
    //
    if mouseReleased(MOUSE_LMB) != 0 && OverRadar == 0 &&
           dragBox3D.status != 2 as libc::c_int as libc::c_uint &&
           ignoreOrder == 0 && mouseOverConsole == 0 &&
           bDisplayMultiJoiningStatus == 0 {
        dealWithLMB();
    }
    if mouseDClicked(MOUSE_LMB) != 0 { dealWithLMBDClick(); }
    if driveModeActive() != 0 && driveTacticalActive() == 0 {
        driveProcessAquireButton();
    } else if driveModeActive() == 0 {
        if mouseReleased(MOUSE_RMB) != 0 && rotActive == 0 && ignoreRMBC == 0
           {
            //		clearSelection();
		//		psSelected3D = NULL;
            dragBox3D.status = 0 as libc::c_int as UDWORD;
            // Pretty sure we wan't set walldrag status here aswell.
            wallDrag.status = 0 as libc::c_int as UDWORD;
            //			printf("Cancel Wall Drag\n");
            bRadarDragging = 0 as libc::c_int;
            dealWithRMB();
            // Why?
            if getWarCamStatus() != 0 { camToggleStatus(); }
        }
        if mouseDrag(MOUSE_RMB, &mut rotX as *mut SDWORD as *mut UDWORD,
                     &mut rotY as *mut SDWORD as *mut UDWORD) == 0 &&
               bRadarDragging != 0 {
            bRadarDragging = 0 as libc::c_int
        }
        /* Right mouse click kills a building placement */
        if mouseReleased(MOUSE_RMB) != 0 &&
               (buildState == 100 as libc::c_int as libc::c_uint ||
                    buildState == 102 as libc::c_int as libc::c_uint) {
            /* Stop the placement */
            kill3DBuilding();
            bRadarDragging = 0 as libc::c_int
        }
        if mouseDrag(MOUSE_RMB, &mut rotX as *mut SDWORD as *mut UDWORD,
                     &mut rotY as *mut SDWORD as *mut UDWORD) != 0 &&
               rotActive == 0 && bRadarDragging == 0 {
            rotInitial = player.r.y as UDWORD;
            rotInitialUp = player.r.x as UDWORD;
            xMoved = 0 as libc::c_int as UDWORD;
            yMoved = 0 as libc::c_int as UDWORD;
            rotActive = 1 as libc::c_int
        }
    }
    selection = establishSelection(selectedPlayer);
    if selection as libc::c_uint <= 13 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Weirdy selection!\x00" as *const u8 as *const libc::c_char);
    };
    if selection as libc::c_uint <= 13 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"display.c\x00" as *const u8 as *const libc::c_char,
              1004 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"processMouseClickInput\x00")).as_ptr(),
              b"selection<=POSSIBLE_SELECTIONS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if selection as libc::c_uint != SC_INVALID as libc::c_int as libc::c_uint
           && gamePaused() == 0 {
        let mut ObjUnderMouse: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
        item = itemUnderMouse(&mut ObjUnderMouse);
        if (item as libc::c_uint) < 23 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Weirdy target!\x00" as *const u8 as *const libc::c_char);
        };
        if (item as libc::c_uint) < 23 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"display.c\x00" as *const u8 as *const libc::c_char,
                  1011 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"processMouseClickInput\x00")).as_ptr(),
                  b"item<POSSIBLE_TARGETS\x00" as *const u8 as
                      *const libc::c_char);
        };
        // alliance override. If in alli then just use the move icon. - but not if its the same player
        //in single player, the genexp script defaults to setting an alliance between player 0 and selectedPlayer
        if !ObjUnderMouse.is_null() &&
               selectedPlayer != (*ObjUnderMouse).player as libc::c_uint &&
               aiCheckAlliances(selectedPlayer,
                                (*ObjUnderMouse).player as UDWORD) != 0 {
            item = MT_NOTARGET
        }
        if item as libc::c_uint != MT_NOTARGET as libc::c_int as libc::c_uint
           {
            // exceptions to the lookup table.
            if ctrlShiftDown() != 0 && !ObjUnderMouse.is_null() &&
                   (*ObjUnderMouse).player as libc::c_uint == selectedPlayer
                   &&
                   (*ObjUnderMouse).type_0 as libc::c_uint ==
                       OBJ_DROID as libc::c_int as libc::c_uint {
                item = MT_OWNDROID
            } else if (keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0) &&
                          !ObjUnderMouse.is_null() &&
                          (*ObjUnderMouse).player as libc::c_uint ==
                              selectedPlayer {
                if selection as libc::c_uint ==
                       SC_DROID_REPAIR as libc::c_int as libc::c_uint {
                    item = MT_OWNDROIDDAM
                } else {
                    // attacking own unit
                    item = MT_ENEMYDROID
                }
            } else if bMultiPlayer != 0 &&
                          item as libc::c_uint ==
                              MT_TRANDROID as libc::c_int as libc::c_uint {
                if cyborgDroidSelected(selectedPlayer) == 0 {
                    item = MT_BLOCKING
                }
            } else if selection as libc::c_uint ==
                          SC_DROID_CONSTRUCT as libc::c_int as libc::c_uint {
                //in multiPlayer can only put cyborgs onto a Transporter
                // We don't allow the build cursor under certain circumstances ....
				// can't build if res extractors arent available.
                if item as libc::c_uint ==
                       MT_RESOURCE as libc::c_int as libc::c_uint {
                    i = 0 as libc::c_int as UDWORD; // find resource stat
                    while i < numStructureStats &&
                              (*asStructureStats.offset(i as isize)).type_0 !=
                                  REF_RESOURCE_EXTRACTOR as libc::c_int as
                                      libc::c_uint {
                        i = i.wrapping_add(1)
                    }
                    if i < numStructureStats &&
                           *apStructTypeLists[selectedPlayer as
                                                  usize].offset(i as isize) as
                               libc::c_int == 0x2 as libc::c_int {
                        // check if you can build it!
                        item = MT_BLOCKING
                        // don't allow build pointer.
                    }
                } else if item as libc::c_uint ==
                              MT_SENSOR as libc::c_int as libc::c_uint {
                    if !ObjUnderMouse.is_null() &&
                           (*ObjUnderMouse).type_0 as libc::c_uint ==
                               OBJ_STRUCTURE as libc::c_int as libc::c_uint {
                        // repair instead of sensor/guard with cons. droids.
                        // check if struct
                        if buildingDamaged(ObjUnderMouse as *mut STRUCTURE) !=
                               0 {
                            item = MT_OWNSTRDAM
                            // replace guard/sense with usual icons.
                        } else { item = MT_OWNSTROK }
                    }
                }
            } else if item as libc::c_uint ==
                          MT_SENSOR as libc::c_int as libc::c_uint &&
                          selection as libc::c_uint ==
                              SC_DROID_INDIRECT as libc::c_int as libc::c_uint
                          &&
                          (keyDown(KEY_LSHIFT) != 0 ||
                               keyDown(KEY_RSHIFT) != 0) {
                selection = SC_DROID_SENSOR
            } else if (item as libc::c_uint ==
                           MT_SENSOR as libc::c_int as libc::c_uint ||
                           item as libc::c_uint ==
                               MT_SENSORSTRUCT as libc::c_int as libc::c_uint
                           ||
                           item as libc::c_uint ==
                               MT_SENSORSTRUCTDAM as libc::c_int as
                                   libc::c_uint) &&
                          selection as libc::c_uint ==
                              SC_DROID_INDIRECT as libc::c_int as libc::c_uint
             {
                if droidSensorDroidWeapon(ObjUnderMouse, psDominantSelected)
                       == 0 {
                    item = MT_BLOCKING
                }
            } else if (item as libc::c_uint ==
                           MT_SENSOR as libc::c_int as libc::c_uint ||
                           item as libc::c_uint ==
                               MT_SENSORSTRUCT as libc::c_int as libc::c_uint
                           ||
                           item as libc::c_uint ==
                               MT_SENSORSTRUCTDAM as libc::c_int as
                                   libc::c_uint) &&
                          selection as libc::c_uint ==
                              SC_DROID_DIRECT as libc::c_int as libc::c_uint
                          &&
                          vtolDroidSelected(selectedPlayer as UBYTE as UDWORD)
                              != 0 {
                // check the type of sensor for indirect weapons
                //check for VTOL droids being assigned to a sensor droid/structure
                // NB. psSelectedVtol was set by vtolDroidSelected - yes I know its horrible, but it
				// only smells as much as the rest of display.c so I don't feel so bad
                if droidSensorDroidWeapon(ObjUnderMouse, psSelectedVtol) != 0
                   {
                    selection = SC_DROID_INDIRECT
                } else { item = MT_BLOCKING }
            } else if item as libc::c_uint ==
                          MT_ARTIFACT as libc::c_int as libc::c_uint &&
                          selection as libc::c_uint ==
                              SC_DROID_DIRECT as libc::c_int as libc::c_uint
                          &&
                          vtolDroidSelected(selectedPlayer as UBYTE as UDWORD)
                              != 0 {
                item = MT_BLOCKING
            }
            //vtols cannot pick up artifacts
            //VTOL's can't be moved to empty terrain
/*			else if (vtolDroidSelected((UBYTE)selectedPlayer)
					AND item == MT_TERRAIN
					AND selection == SC_DROID_DIRECT)
			{
				item = MT_BLOCKING;
			}*/
            //in multiPlayer Transporters can be moved around the terrain - and repaired
            if bMultiPlayer != 0 &&
                   selection as libc::c_uint ==
                       SC_DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
                   (item as libc::c_uint ==
                        MT_TERRAIN as libc::c_int as libc::c_uint ||
                        item as libc::c_uint ==
                            MT_REPAIR as libc::c_int as libc::c_uint) {
                //change to standard droid selection
                selection = SC_DROID_INDIRECT
            } else if item as libc::c_uint ==
                          MT_TERRAIN as libc::c_int as libc::c_uint &&
                          terrainTypes[((*mapTile(mouseTileX as UDWORD,
                                                  mouseTileY as
                                                      UDWORD)).texture as
                                            libc::c_int &
                                            0x1ff as libc::c_int) as usize] as
                              libc::c_int == TER_CLIFFFACE as libc::c_int {
                item = MT_BLOCKING
            }
            pie_SetMouse(IntImages,
                         MPointerImageIDs[arnMPointers[item as
                                                           usize][selection as
                                                                      usize].wrapping_sub(101
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint)
                                              as usize]);
            frameSetCursorFromRes(arnMPointers[item as
                                                   usize][selection as usize]
                                      as WORD);
        }
    } else if bMultiPlayer != 0 && bLasSatStruct != 0 {
        let mut ObjUnderMouse_0: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
        item = itemUnderMouse(&mut ObjUnderMouse_0);
        if (item as libc::c_uint) < 23 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Weirdy target!\x00" as *const u8 as *const libc::c_char);
        };
        if (item as libc::c_uint) < 23 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"display.c\x00" as *const u8 as *const libc::c_char,
                  1156 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"processMouseClickInput\x00")).as_ptr(),
                  b"item<POSSIBLE_TARGETS\x00" as *const u8 as
                      *const libc::c_char);
        };
        if item as libc::c_uint ==
               MT_ENEMYDROID as libc::c_int as libc::c_uint ||
               item as libc::c_uint ==
                   MT_ENEMYSTR as libc::c_int as libc::c_uint ||
               item as libc::c_uint ==
                   MT_DAMFEATURE as libc::c_int as libc::c_uint {
            //exceptions, exceptions...AB 10/06/99
            //display attack cursor
            pie_SetMouse(IntImages,
                         IMAGE_CURSOR_ATTACK as libc::c_int as UWORD);
            frameSetCursorFromRes(111 as libc::c_int as WORD);
        } else {
            //display block cursor
            pie_SetMouse(IntImages,
                         IMAGE_CURSOR_NOTPOS as libc::c_int as UWORD);
            frameSetCursorFromRes(122 as libc::c_int as WORD);
        }
    } else {
        pie_SetMouse(IntImages, IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD);
        frameSetCursorFromRes(108 as libc::c_int as WORD);
    }
    CurrentItemUnderMouse = item as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn scroll() {
    let mut radians: libc::c_float = 0.;
    let mut cosine: libc::c_float = 0.;
    let mut sine: libc::c_float = 0.;
    let mut xDif: SDWORD = 0;
    let mut yDif: SDWORD = 0;
    let mut timeDiff: UDWORD = 0;
    let mut bRetardScroll: BOOL = 0 as libc::c_int;
    if InGameOpUp != 0 || bDisplayMultiJoiningStatus as libc::c_int != 0 {
        // cant scroll when menu up. or when over radar
        return
    }
    if keyDown(KEY_LCTRL) == 0 && keyDown(KEY_RCTRL) == 0 {
        /* Scroll left */
        if keyDown(KEY_LEFTARROW) != 0 || mX < 16 as libc::c_int {
            mouseAtLeft = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_LEFTARROW) == 0 &&
                   mX > 16 as libc::c_int / 2 as libc::c_int {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll right */
        if keyDown(KEY_RIGHTARROW) != 0 ||
               mX >
                   pie_GetVideoBufferWidth().wrapping_sub(16 as libc::c_int as
                                                              libc::c_uint) as
                       SDWORD {
            mouseAtRight = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_RIGHTARROW) == 0 &&
                   mX <
                       pie_GetVideoBufferWidth().wrapping_sub((16 as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint)
                           as SDWORD {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll up */
        if keyDown(KEY_UPARROW) != 0 || mY < 16 as libc::c_int {
            mouseAtBottom = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_UPARROW) == 0 &&
                   mY > 16 as libc::c_int / 2 as libc::c_int {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll down */
        if keyDown(KEY_DOWNARROW) != 0 ||
               mY >
                   pie_GetVideoBufferHeight().wrapping_sub(16 as libc::c_int
                                                               as
                                                               libc::c_uint)
                       as SDWORD {
            mouseAtTop = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_DOWNARROW) == 0 &&
                   mY <
                       pie_GetVideoBufferHeight().wrapping_sub((16 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint)
                           as SDWORD {
                bRetardScroll = 1 as libc::c_int
            }
        }
    } else {
        /* Scroll left */
        if mX < 16 as libc::c_int {
            mouseAtLeft = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_LEFTARROW) == 0 &&
                   mX > 16 as libc::c_int / 2 as libc::c_int {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll right */
        if mX >
               pie_GetVideoBufferWidth().wrapping_sub(16 as libc::c_int as
                                                          libc::c_uint) as
                   SDWORD {
            mouseAtRight = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_RIGHTARROW) == 0 &&
                   mX <
                       pie_GetVideoBufferWidth().wrapping_sub((16 as
                                                                   libc::c_int
                                                                   /
                                                                   2 as
                                                                       libc::c_int)
                                                                  as
                                                                  libc::c_uint)
                           as SDWORD {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll up */
        if mY < 16 as libc::c_int {
            mouseAtBottom = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_UPARROW) == 0 &&
                   mY > 16 as libc::c_int / 2 as libc::c_int {
                bRetardScroll = 1 as libc::c_int
            }
        }
        /* Scroll down */
        if mY >
               pie_GetVideoBufferHeight().wrapping_sub(16 as libc::c_int as
                                                           libc::c_uint) as
                   SDWORD {
            mouseAtTop = 1 as libc::c_int;
            mouseAtEdge = 1 as libc::c_int;
            if keyDown(KEY_DOWNARROW) == 0 &&
                   mY <
                       pie_GetVideoBufferHeight().wrapping_sub((16 as
                                                                    libc::c_int
                                                                    /
                                                                    2 as
                                                                        libc::c_int)
                                                                   as
                                                                   libc::c_uint)
                           as SDWORD {
                bRetardScroll = 1 as libc::c_int
            }
        }
    }
    /* Time to update scroll - change to should be time */
	//scrollAccel = (float)SCROLL_SPEED_ACCEL * (float)(gameTime - scrollRefTime)/(float)GAME_TICKS_PER_SEC;
    timeDiff = (GetTickCount() as libc::c_uint).wrapping_sub(scrollRefTime);
    //WHEN its fixed - you can uncomment it!
	//if(bRetardScroll AND FALSE)	//temp until fixed
	//{
	//	timeDiff/=2;
	//}
	/* Store reference time */
    scrollRefTime = GetTickCount() as UDWORD;
    if timeDiff > (1000 as libc::c_int / 6 as libc::c_int) as libc::c_uint {
        timeDiff = (1000 as libc::c_int / 6 as libc::c_int) as UDWORD
    }
    scrollAccel =
        scroll_speed_accel as libc::c_float * timeDiff as libc::c_float /
            1000 as libc::c_int as libc::c_float;
    if mouseAtLeft != 0 {
        if scrollSpeedLeftRight > 0 as libc::c_int as libc::c_float {
            scrollSpeedLeftRight = 0.0f64 as libc::c_float
        }
        scrollSpeedLeftRight -= scrollAccel;
        if scrollSpeedLeftRight <
               -((800 as libc::c_int as
                      libc::c_uint).wrapping_add(scroll_speed_accel) as
                     libc::c_float) {
            scrollSpeedLeftRight =
                -((800 as libc::c_int as
                       libc::c_uint).wrapping_add(scroll_speed_accel) as
                      libc::c_float)
        }
    } else if mouseAtRight != 0 {
        if scrollSpeedLeftRight < 0 as libc::c_int as libc::c_float {
            scrollSpeedLeftRight = 0.0f64 as libc::c_float
        }
        scrollSpeedLeftRight += scrollAccel;
        if scrollSpeedLeftRight >
               (800 as libc::c_int as
                    libc::c_uint).wrapping_add(scroll_speed_accel) as
                   libc::c_float {
            scrollSpeedLeftRight =
                (800 as libc::c_int as
                     libc::c_uint).wrapping_add(scroll_speed_accel) as
                    libc::c_float
        }
    } else if scrollSpeedLeftRight >
                  2 as libc::c_int as libc::c_float * scrollAccel {
        scrollSpeedLeftRight -=
            2 as libc::c_int as libc::c_float * scrollAccel
    } else if scrollSpeedLeftRight <
                  -(2 as libc::c_int) as libc::c_float * scrollAccel {
        scrollSpeedLeftRight +=
            2 as libc::c_int as libc::c_float * scrollAccel
    } else { scrollSpeedLeftRight = 0.0f64 as libc::c_float }
    if mouseAtBottom != 0 {
        // not at left or right so retard the scroll
        //its at the top??
        if scrollSpeedUpDown < 0 as libc::c_int as libc::c_float {
            scrollSpeedUpDown = 0.0f64 as libc::c_float
        }
        scrollSpeedUpDown += scrollAccel;
        if scrollSpeedUpDown >
               (800 as libc::c_int as
                    libc::c_uint).wrapping_add(scroll_speed_accel) as
                   libc::c_float {
            scrollSpeedUpDown =
                (800 as libc::c_int as
                     libc::c_uint).wrapping_add(scroll_speed_accel) as
                    libc::c_float
        }
    } else if mouseAtTop != 0 {
        //its at the bottom??
        if scrollSpeedUpDown > 0 as libc::c_int as libc::c_float {
            scrollSpeedUpDown = 0.0f64 as libc::c_float
        }
        scrollSpeedUpDown -= scrollAccel;
        if scrollSpeedUpDown <
               -((800 as libc::c_int as
                      libc::c_uint).wrapping_add(scroll_speed_accel) as
                     libc::c_float) {
            scrollSpeedUpDown =
                -((800 as libc::c_int as
                       libc::c_uint).wrapping_add(scroll_speed_accel) as
                      libc::c_float)
        }
    } else if scrollSpeedUpDown > scrollAccel {
        scrollSpeedUpDown -= 2 as libc::c_int as libc::c_float * scrollAccel
    } else if scrollSpeedUpDown < -scrollAccel {
        scrollSpeedUpDown += 2 as libc::c_int as libc::c_float * scrollAccel
    } else { scrollSpeedUpDown = 0.0f64 as libc::c_float }
    // not at top or bottom so retard the scroll
    // scrool speeds updated in proportion to frame time calculate how far to step in each direction
	//scrollStepLeftRight = scrollSpeedLeftRight * (float)(gameTime - scrollRefTime)/(float)GAME_TICKS_PER_SEC;
    scrollStepLeftRight =
        scrollSpeedLeftRight * timeDiff as libc::c_float /
            1000 as libc::c_int as libc::c_float;
    //scrollStepUpDown = scrollSpeedUpDown * (float)(gameTime - scrollRefTime)/(float)GAME_TICKS_PER_SEC;
    scrollStepUpDown =
        scrollSpeedUpDown * timeDiff as libc::c_float /
            1000 as libc::c_int as libc::c_float;
    /* Get angle vector to scroll along */
    worldAngle =
        (player.r.y as
             UDWORD).wrapping_div((65536 as libc::c_int / 360 as libc::c_int)
                                      as
                                      libc::c_uint).wrapping_rem(360 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
    direction =
        (360 as libc::c_int as libc::c_uint).wrapping_sub(worldAngle) as
            SDWORD;
    /* Convert to radians */
    radians =
        3.141592657f64 as libc::c_float / 180 as libc::c_int as libc::c_float
            * direction as libc::c_float;
    cosine = cos(radians as libc::c_double) as libc::c_float;
    sine = sin(radians as libc::c_double) as libc::c_float;
    /* Get x component of movement */
    xDif =
        if cosine * scrollStepLeftRight + sine * scrollStepUpDown >=
               0 as libc::c_int as libc::c_float {
            ((cosine * scrollStepLeftRight + sine * scrollStepUpDown) as
                 libc::c_double + 0.5f64) as SDWORD
        } else {
            ((cosine * scrollStepLeftRight + sine * scrollStepUpDown) as
                 libc::c_double - 0.5f64) as SDWORD
        };
    /* Get y component of movement */
    yDif =
        if sine * scrollStepLeftRight - cosine * scrollStepUpDown >=
               0 as libc::c_int as libc::c_float {
            ((sine * scrollStepLeftRight - cosine * scrollStepUpDown) as
                 libc::c_double + 0.5f64) as SDWORD
        } else {
            ((sine * scrollStepLeftRight - cosine * scrollStepUpDown) as
                 libc::c_double - 0.5f64) as SDWORD
        };
    /* Adjust player's position by these components */
    player.p.x += xDif;
    player.p.z += yDif;
    edgeOfMap = CheckScrollLimits();
    mouseAtLeft = 0 as libc::c_int;
    mouseAtRight = 0 as libc::c_int;
    mouseAtTop = 0 as libc::c_int;
    mouseAtBottom = 0 as libc::c_int;
}
// Check a coordinate is within the scroll limits, uses object type coords ie UWORDs.
// Returns TRUE if edge hit.
//
#[no_mangle]
pub unsafe extern "C" fn CheckObjInScrollLimits(mut xPos: *mut UWORD,
                                                mut zPos: *mut UWORD)
 -> BOOL {
    let mut xp: SDWORD =
        *xPos as SWORD as libc::c_int -
            32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int;
    let mut zp: SDWORD =
        *zPos as SWORD as libc::c_int -
            32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int;
    let mut ret: BOOL = CheckInScrollLimits(&mut xp, &mut zp);
    *xPos =
        (xp + 32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int) as
            UWORD;
    *zPos =
        (zp + 32 as libc::c_int * 128 as libc::c_int / 2 as libc::c_int) as
            UWORD;
    return ret;
}
// Check a coordinate is within the scroll limits, SDWORD version.
// Returns TRUE if edge hit.
//
#[no_mangle]
pub unsafe extern "C" fn CheckInScrollLimits(mut xPos: *mut SDWORD,
                                             mut zPos: *mut SDWORD) -> BOOL {
    let mut EdgeHit: BOOL = 0 as libc::c_int;
    let mut minX: SDWORD = 0;
    let mut minY: SDWORD = 0;
    let mut maxX: SDWORD = 0;
    let mut maxY: SDWORD = 0;
    //always view that little bit more than the scroll limits...
	/*minX = scrollMinX * TILE_UNITS;
	minY = scrollMinY * TILE_UNITS;
	maxX = (((scrollMaxX-1) - visibleXTiles) * TILE_UNITS);
	maxY = (((scrollMaxY-1) - visibleYTiles) * TILE_UNITS);

	if(scrollMinX==0)
	{
		minX = ((0 - visibleXTiles/2) * TILE_UNITS);
	}

	if((UDWORD)scrollMaxX == mapWidth)
	{
		maxX = ((mapWidth-1-(visibleXTiles/2)) * TILE_UNITS);
	}

	if(scrollMinY==0)
	{
		minY = ((0 - visibleYTiles/2) * TILE_UNITS);
	}

	if((UDWORD)scrollMaxY == mapHeight)
	{
		maxY = ((mapHeight-1-(visibleYTiles/2)) * TILE_UNITS);
	}*/
    minX =
        (scrollMinX as
             libc::c_uint).wrapping_sub(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)).wrapping_mul(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    maxX =
        ((scrollMaxX - 1 as libc::c_int) as
             libc::c_uint).wrapping_sub(visibleXTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)).wrapping_mul(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    minY =
        (scrollMinY as
             libc::c_uint).wrapping_sub(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)).wrapping_mul(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    maxY =
        ((scrollMaxY - 1 as libc::c_int) as
             libc::c_uint).wrapping_sub(visibleYTiles.wrapping_div(2 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)).wrapping_mul(128
                                                                                                       as
                                                                                                       libc::c_int
                                                                                                       as
                                                                                                       libc::c_uint)
            as SDWORD;
    //scroll is limited to what can be seen for current campaign
    if *xPos < minX {
        *xPos = minX;
        EdgeHit = 1 as libc::c_int
    } else if *xPos >= maxX { *xPos = maxX; EdgeHit = 1 as libc::c_int }
    if *zPos < minY {
        *zPos = minY;
        EdgeHit = 1 as libc::c_int
    } else if *zPos >= maxY { *zPos = maxY; EdgeHit = 1 as libc::c_int }
    return EdgeHit;
}
// Check the view is within the scroll limits,
// Returns TRUE if edge hit.
//
#[no_mangle]
pub unsafe extern "C" fn CheckScrollLimits() -> BOOL {
    // need to be carefull here, player position uses SDWORD's on PC and SWORDS on PSX.
    let mut xp: SDWORD = player.p.x;
    let mut zp: SDWORD = player.p.z;
    let mut ret: BOOL = CheckInScrollLimits(&mut xp, &mut zp);
    player.p.x = xp;
    player.p.z = zp;
    return ret;
}
/* Do the 3D display */
#[no_mangle]
pub unsafe extern "C" fn displayWorld() {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    shakeUpdate();
    if mouseDown(MOUSE_RMB) != 0 && rotActive != 0 {
        if abs(mX - rotX) > 8 as libc::c_int ||
               xMoved > 8 as libc::c_int as libc::c_uint {
            xMoved =
                (xMoved as
                     libc::c_uint).wrapping_add(abs(mX - rotX) as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if mX < rotX {
                player.r.y =
                    rotInitial.wrapping_add(((rotX - mX) / 2 as libc::c_int *
                                                 (65536 as libc::c_int /
                                                      360 as libc::c_int *
                                                      1 as libc::c_int)) as
                                                libc::c_uint) as int32
            } else {
                player.r.y =
                    rotInitial.wrapping_sub(((mX - rotX) / 2 as libc::c_int *
                                                 (65536 as libc::c_int /
                                                      360 as libc::c_int *
                                                      1 as libc::c_int)) as
                                                libc::c_uint) as int32
            }
        }
        if abs(mY - rotY) > 8 as libc::c_int ||
               yMoved > 8 as libc::c_int as libc::c_uint {
            yMoved =
                (yMoved as
                     libc::c_uint).wrapping_add(abs(mY - rotY) as
                                                    libc::c_uint) as UDWORD as
                    UDWORD;
            if bInvertMouse != 0 {
                if mY < rotY {
                    player.r.x =
                        rotInitialUp.wrapping_add(((rotY - mY) /
                                                       3 as libc::c_int *
                                                       (65536 as libc::c_int /
                                                            360 as libc::c_int
                                                            *
                                                            1 as libc::c_int))
                                                      as libc::c_uint) as
                            int32
                } else {
                    player.r.x =
                        rotInitialUp.wrapping_sub(((mY - rotY) /
                                                       3 as libc::c_int *
                                                       (65536 as libc::c_int /
                                                            360 as libc::c_int
                                                            *
                                                            1 as libc::c_int))
                                                      as libc::c_uint) as
                            int32
                }
            } else if mY < rotY {
                player.r.x =
                    rotInitialUp.wrapping_sub(((rotY - mY) / 3 as libc::c_int
                                                   *
                                                   (65536 as libc::c_int /
                                                        360 as libc::c_int *
                                                        1 as libc::c_int)) as
                                                  libc::c_uint) as int32
            } else {
                player.r.x =
                    rotInitialUp.wrapping_add(((mY - rotY) / 3 as libc::c_int
                                                   *
                                                   (65536 as libc::c_int /
                                                        360 as libc::c_int *
                                                        1 as libc::c_int)) as
                                                  libc::c_uint) as int32
            }
            if player.r.x >
                   65536 as libc::c_int / 360 as libc::c_int *
                       (360 as libc::c_int + -(14 as libc::c_int)) {
                player.r.x =
                    65536 as libc::c_int / 360 as libc::c_int *
                        (360 as libc::c_int + -(14 as libc::c_int))
            }
            if player.r.x <
                   65536 as libc::c_int / 360 as libc::c_int *
                       (360 as libc::c_int + -(50 as libc::c_int)) {
                player.r.x =
                    65536 as libc::c_int / 360 as libc::c_int *
                        (360 as libc::c_int + -(50 as libc::c_int))
            }
            setDesiredPitch(player.r.x /
                                (65536 as libc::c_int / 360 as libc::c_int));
        }
    }
    if mouseReleased(MOUSE_RMB) != 0 && rotActive != 0 {
        rotActive = 0 as libc::c_int;
        yMoved = 0 as libc::c_int as UDWORD;
        xMoved = yMoved;
        ignoreRMBC = 1 as libc::c_int;
        pos.x = player.r.x;
        pos.y = player.r.y;
        pos.z = player.r.z;
        camInformOfRotation(&mut pos);
        bRadarDragging = 0 as libc::c_int
    }
    draw3DScene();
}
#[no_mangle]
pub unsafe extern "C" fn mouseSetMXMY() { mX = mouseX(); mY = mouseY(); }
#[no_mangle]
pub unsafe extern "C" fn mouseInBox(mut x0: SDWORD, mut y0: SDWORD,
                                    mut x1: SDWORD, mut y1: SDWORD) -> BOOL {
    if mX > x0 && mX < x1 {
        if mY > y0 && mY < y1 { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DrawnInLastFrame(mut Frame: SDWORD) -> BOOL {
    if Frame >= StartOfLastFrame as SDWORD { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
/*
	Returns what the mouse was clicked on. Only called if there was a mouse pressed message
	on MOUSE_LMB. We aren't concerned here with setting selection flags - just what it
	actually was
*/
#[no_mangle]
pub unsafe extern "C" fn mouseTarget() -> *mut BASE_OBJECT {
    let mut i: UDWORD = 0;
    let mut psReturn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut dispX: UDWORD = 0;
    let mut dispY: UDWORD = 0;
    let mut dispR: UDWORD = 0;
    if mouseTileX < 0 as libc::c_int || mouseTileY < 0 as libc::c_int ||
           mouseTileX >
               mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                   SDWORD ||
           mouseTileY >
               mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                   SDWORD {
        return 0 as *mut BASE_OBJECT
    }
    /* We haven't found anything yet */
    psReturn = 0 as *mut BASE_OBJECT;
    /* First have a look through the droid lists */
    i = 0 as libc::c_int as UDWORD; // end of checking for droids
    while i < 8 as libc::c_int as libc::c_uint {
        /* Note the !psObject check isn't really necessary as the goto will jump out */
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() && psReturn.is_null() {
            dispX = (*psDroid).sDisplay.screenX;
            dispY = (*psDroid).sDisplay.screenY;
            dispR = (*psDroid).sDisplay.screenR;
            /* Only check droids that're on screen */
            // Has the droid been drawn since the start of the last frame
            if (*psDroid).visible[selectedPlayer as usize] as libc::c_int != 0
                   &&
                   DrawnInLastFrame((*psDroid).sDisplay.frameNumber as SDWORD)
                       == 1 as libc::c_int {
                // 			if(psDroid->sDisplay.frameNumber+1 == currentFrame)
                if mouseInBox(dispX.wrapping_sub(dispR) as SDWORD,
                              dispY.wrapping_sub(dispR) as SDWORD,
                              dispX.wrapping_add(dispR) as SDWORD,
                              dispY.wrapping_add(dispR) as SDWORD) != 0 {
                    /* We HAVE clicked on droid! */
                    psReturn = psDroid as *mut BASE_OBJECT;
                    /* There's no point in checking other object types */
                    return psReturn
                }
            }
            psDroid = (*psDroid).psNext
        }
        i = i.wrapping_add(1)
    }
    /*	Not a droid, so maybe a structure or feature?
		If still NULL after this then nothing */
    psReturn = getTileOccupier(mouseTileX as UDWORD, mouseTileY as UDWORD);
    /* Send the result back - if it's null then we clicked on an area of terrain */
    return psReturn;
}
#[no_mangle]
pub static mut lastangle: UWORD = 0;
// debugging only
// Dummy structure stats used for positioning delivery points.
static mut ReposStats: STRUCTURE_STATS =
    STRUCTURE_STATS{ref_0: 0,
                    pName: 0 as *const STRING as *mut STRING,
                    type_0: 0,
                    techLevel: TECH_LEVEL_ONE,
                    strength: STRENGTH_SOFT,
                    terrainType: 0,
                    baseWidth: 0,
                    baseBreadth: 0,
                    foundationType: 0,
                    buildPoints: 0,
                    height: 0,
                    armourValue: 0,
                    bodyPoints: 0,
                    repairSystem: 0,
                    powerToBuild: 0,
                    resistance: 0,
                    sizeModifier: 0,
                    pIMD: 0 as *const iIMDShape as *mut iIMDShape,
                    pBaseIMD: 0 as *const iIMDShape as *mut iIMDShape,
                    pECM: 0 as *const _ecm_stats as *mut _ecm_stats,
                    pSensor: 0 as *const _sensor_stats as *mut _sensor_stats,
                    psWeapStat:
                        0 as *const _weapon_stats as *mut _weapon_stats,
                    numFuncs: 0,
                    defaultFunc: 0,
                    asFuncList:
                        0 as *const *mut _function as *mut *mut _function,};
static mut ReposValid: BOOL = 0 as libc::c_int;
static mut BVReposValid: BOOL = 0 as libc::c_int;
static mut ReposFlag: *mut FLAG_POSITION =
    0 as *const FLAG_POSITION as *mut FLAG_POSITION;
#[no_mangle]
pub unsafe extern "C" fn StartTacticalScroll(mut driveActive: BOOL) { }
#[no_mangle]
pub unsafe extern "C" fn StartTacticalScrollObj(mut driveActive: BOOL,
                                                mut psObj: *mut BASE_OBJECT) {
}
#[no_mangle]
pub unsafe extern "C" fn CancelTacticalScroll() { }
#[no_mangle]
pub unsafe extern "C" fn displayInitVars() {
    ReposValid = 0 as libc::c_int;
    BVReposValid = 0 as libc::c_int;
}
// Start repositioning a delivery point.
//
#[no_mangle]
pub unsafe extern "C" fn StartDeliveryPosition(mut psLocation:
                                                   *mut OBJECT_POSITION,
                                               mut driveActive: BOOL) {
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    /* clear the selection */
//	clearSelection();
	//clear the Deliv Point if one
    psFlagPos = apsFlagPosLists[selectedPlayer as usize];
    while !psFlagPos.is_null() {
        (*psFlagPos).selected = 0 as libc::c_int;
        psFlagPos = (*psFlagPos).psNext
    }
    //set this object position to be highlighted
    (*psLocation).selected = 1 as libc::c_int;
    if bInTutorial != 0 {
        eventFireCallbackTrigger(CALL_DELIVPOINTMOVED as libc::c_int as
                                     TRIGGER_TYPE);
    }
    // Setup dummy structure stats for positioning a delivery point.
    ReposValid = 0 as libc::c_int; //REF_STRUCTURE_START;
    ReposFlag = 0 as *mut FLAG_POSITION;
    ReposStats.baseWidth = 1 as libc::c_int as UDWORD;
    ReposStats.baseBreadth = 1 as libc::c_int as UDWORD;
    ReposStats.ref_0 = 0 as libc::c_int as UDWORD;
    //set up the buildSite variable for drawing
    buildSite.xTL = (*psLocation).screenX as UWORD;
    buildSite.yTL = (*psLocation).screenY as UWORD;
    buildSite.xBR =
        (buildSite.xTL as libc::c_int - 1 as libc::c_int) as UWORD;
    buildSite.yBR =
        (buildSite.yTL as libc::c_int - 1 as libc::c_int) as UWORD;
    init3DBuilding(&mut ReposStats as *mut STRUCTURE_STATS as *mut BASE_STATS,
                   Some(FinishDeliveryPosition as
                            unsafe extern "C" fn(_: UDWORD, _: UDWORD,
                                                 _: *mut libc::c_void) -> ()),
                   psLocation as *mut libc::c_void);
}
// Finished repositioning a delivery point.
//
#[no_mangle]
pub unsafe extern "C" fn FinishDeliveryPosition(mut xPos: UDWORD,
                                                mut yPos: UDWORD,
                                                mut UserData:
                                                    *mut libc::c_void) {
    //This deals with adding waypoints and moving the primary
    processDeliveryPoint((*(UserData as *mut FLAG_POSITION)).player,
                         xPos << 7 as libc::c_int, yPos << 7 as libc::c_int);
    //deselect it
    (*(UserData as *mut FLAG_POSITION)).selected = 0 as libc::c_int;
    CancelDeliveryRepos();
}
// Cancel repositioning of the delivery point without moveing it.
//
/*void CancelDeliveryRepos(void)
{
	if((ReposValid) && (ReposFlag!=NULL))
	{
		if(driveModeActive())
		{
			DROID *Driven = driveGetDriven();
			if(Driven != NULL) {
				Driven->selected = TRUE;
				camAllignWithTarget(Driven);
			}
			driveEnableControl();
		}
		ReposValid = FALSE;
		ReposFlag = NULL;
		buildState = BUILD3D_NONE;
	}
}*/
// Get the current screen position of the delivery point.
//
/*BOOL GetDeliveryRepos(UDWORD *xPos,UDWORD *yPos)
{
	if((ReposValid) && (ReposFlag!=NULL))
	{
		*xPos = scoord_PC2PSXx(ReposFlag->screenX);
		*yPos = scoord_PC2PSXy(ReposFlag->screenY);
		return TRUE;
	}

	return FALSE;
}*/
// Is there a valid delivery point repositioning going on.
//
#[no_mangle]
pub unsafe extern "C" fn DeliveryReposValid() -> BOOL {
    if driveModeActive() != 0 {
        return (ReposValid != 0 && !ReposFlag.is_null()) as libc::c_int
    } else { return BVReposValid };
}
// Cancel repositioning of the delivery point without moving it.
//
#[no_mangle]
pub unsafe extern "C" fn CancelDeliveryRepos() {
    if ReposValid != 0 && !ReposFlag.is_null() {
        if driveModeActive() != 0 {
            let mut Driven: *mut DROID = driveGetDriven();
            if !Driven.is_null() {
                //				Driven->selected = TRUE;
                SelectDroid(Driven);
                camAllignWithTarget(Driven as *mut BASE_OBJECT);
            }
            driveEnableControl();
        }
        ReposValid = 0 as libc::c_int;
        ReposFlag = 0 as *mut FLAG_POSITION
    }
    BVReposValid = 0 as libc::c_int;
}
// check whether a clicked on droid is in a command group or assigned to a sensor
#[no_mangle]
pub unsafe extern "C" fn droidHasLeader(mut psDroid: *mut DROID) -> BOOL {
    let mut psLeader: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if (*psDroid).droidType as libc::c_uint ==
           DROID_COMMAND as libc::c_int as libc::c_uint ||
           (*psDroid).droidType as libc::c_uint ==
               DROID_SENSOR as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    psLeader = 0 as *mut BASE_OBJECT;
    if !(*psDroid).psGroup.is_null() &&
           (*(*psDroid).psGroup).type_0 as libc::c_int ==
               GT_COMMAND as libc::c_int {
        psLeader = (*(*psDroid).psGroup).psCommander as *mut BASE_OBJECT
    } else {
        //psLeader can be either a droid or a structure
        orderStateObj(psDroid, DORDER_FIRESUPPORT, &mut psLeader);
    }
    if !psLeader.is_null() {
        //		psLeader->selected = TRUE;
        if (*psLeader).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            SelectDroid(psLeader as *mut DROID);
        }
        assignSensorTarget(psLeader);
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// deal with selecting a droid
#[no_mangle]
pub unsafe extern "C" fn dealWithDroidSelect(mut psDroid: *mut DROID,
                                             mut bDragBox: BOOL) {
    let mut psD: *mut DROID = 0 as *mut DROID;
    let mut bGotGroup: BOOL = 0;
    let mut groupNumber: SDWORD = 0 as libc::c_int;
    /*	Toggle selection on and off - allows you drag around a big
		area of droids and then exclude certain individuals */
    if bDragBox == 0 && (*psDroid).selected as libc::c_int == 1 as libc::c_int
       {
        //		psDroid->selected = FALSE;
        DeSelectDroid(psDroid);
        // was "=="
        (*psDroid).group = 0xff as libc::c_int as UBYTE
    } else if keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
                  keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0 ||
                  droidHasLeader(psDroid) == 0 {
        psD = apsDroidLists[selectedPlayer as usize];
        bGotGroup = 0 as libc::c_int;
        while !psD.is_null() && bGotGroup == 0 {
            if (*psD).selected as libc::c_int != 0 &&
                   (*psD).group as libc::c_int != 0xff as libc::c_int {
                bGotGroup = 1 as libc::c_int;
                groupNumber = (*psD).group as SDWORD
            }
            psD = (*psD).psNext
        }
        if bGotGroup != 0 &&
               (keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
                    keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0) {
            (*psDroid).group = groupNumber as UBYTE;
            secondarySetAverageGroupState(selectedPlayer,
                                          groupNumber as UDWORD);
        }
        //		psDroid->selected = TRUE;
        if keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0 {
            /* We only want to select weapon units if ALT is down on a drag */
            if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                   0 as libc::c_int as libc::c_uint {
                SelectDroid(psDroid);
            }
        } else { SelectDroid(psDroid); }
        /*						if(psDroid->droidType == DROID_COMMAND)
		{
			cmdSelectSubDroids(psDroid);
		}*/
//					intObjectSelected((BASE_OBJECT *)psDroid);
        if bInTutorial != 0 {
            psCBSelectedDroid = psDroid;
            eventFireCallbackTrigger(CALL_DROID_SELECTED as libc::c_int as
                                         TRIGGER_TYPE);
            psCBSelectedDroid = 0 as *mut DROID
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FeedbackOrderGiven() {
    static mut LastFrame: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ThisFrame: UDWORD = frameGetFrameNumber();
    // Ensure only played once per game cycle.
    if ThisFrame != LastFrame {
        audio_PlayTrack(ID_SOUND_SELECT as libc::c_int);
        LastFrame = ThisFrame
    };
}
#[no_mangle]
pub unsafe extern "C" fn FeedbackClickedOn() { FeedbackOrderGiven(); }
// check whether the queue order keys are pressed
// check whether the queue order keys are pressed
#[no_mangle]
pub unsafe extern "C" fn ctrlShiftDown() -> BOOL {
    return (keyDown(KEY_LCTRL) != 0 || keyDown(KEY_RCTRL) != 0 ||
                keyDown(KEY_LSHIFT) != 0 || keyDown(KEY_RSHIFT) != 0) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BeepMessage(mut StringID: UDWORD) {
    static mut LastFrame: UDWORD = 0 as libc::c_int as UDWORD;
    let mut ThisFrame: UDWORD = frameGetFrameNumber();
    // Ensure only done once per game cycle.
    if ThisFrame != LastFrame {
        addConsoleMessage(strresGetString(psStringRes, StringID),
                          DEFAULT_JUSTIFY);
        audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
        LastFrame = ThisFrame
    };
}
#[no_mangle]
pub unsafe extern "C" fn AddDerrickBurningMessage() {
    addConsoleMessage(strresGetString(psStringRes,
                                      STR_GAM_DERRICK_BURNING as libc::c_int
                                          as UDWORD), DEFAULT_JUSTIFY);
    audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dealWithLMB() {
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psNearestUnit: *mut DROID = 0 as *mut DROID;
    //BOOL			bWeapDroidSelected;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psSLoop: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psLocation: *mut OBJECT_POSITION = 0 as *mut OBJECT_POSITION;
    let mut i: UDWORD = 0;
    let mut selection: SELECTION_TYPE = SC_DROID_CONSTRUCT;
    /* Don't process if in game options are on screen */
    if InGameOpUp == 1 as libc::c_int ||
           !widgGetFromID(psWScreen, 10500 as libc::c_int as UDWORD).is_null()
       {
        return
    }
    /* What have we clicked on? */
    if driveModeActive() != 0 && driveTacticalActive() == 0 {
        psClickedOn = targetGetCurrent()
        //if(psClickedOn != NULL) {
//printf("%p\n",psClickedOn);
//}
    } else { psClickedOn = mouseTarget() }
    //DBPRINTF(("dealWithLMB() : %p\n",psClickedOn));
	/* If not NULL, then it's a droid or a structure */
    if !psClickedOn.is_null() {
        selection = establishSelection(selectedPlayer);
        /* We've got a droid or a structure */
        if (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            psDroid = psClickedOn as *mut DROID; // end if its a droid
            if (*psDroid).player as libc::c_uint == selectedPlayer {
                if ctrlShiftDown() != 0 {
                    /* We clicked on droid */
                    // select/deselect etc. the droid
                    dealWithDroidSelect(psDroid, 0 as libc::c_int);
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_TRANSPORTER as libc::c_int as libc::c_uint
                 {
                    if selection as libc::c_uint ==
                           SC_INVALID as libc::c_int as libc::c_uint {
                        //in multiPlayer mode we RMB to get the interface up
                        if bMultiPlayer != 0 {
                            (*psDroid).selected = 1 as libc::c_int as UBYTE
                        } else {
                            intResetScreen(0 as libc::c_int);
                            if getWidgetsStatus() == 0 {
                                setWidgetsStatus(1 as libc::c_int);
                            }
                            addTransporterInterface(psDroid,
                                                    0 as libc::c_int);
                        }
                    } else {
                        orderSelectedObj(selectedPlayer, psClickedOn);
                        FeedbackOrderGiven();
                    }
                } else if keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0 {
                    /*
				else if (psDroid->droidType == DROID_SENSOR &&
					selection == SC_DROID_INDIRECT &&
					!(keyDown(KEY_LCTRL) || keyDown(KEY_RCTRL)))
	//				!(keyDown(KEY_LSHIFT) || keyDown(KEY_RSHIFT)))
				{
					orderSelectedObj(selectedPlayer, psClickedOn);
				}
				*/
// Like to get this working on PSX.
                    // try to attack your own unit
                    psCurr = apsDroidLists[selectedPlayer as usize];
                    while !psCurr.is_null() {
                        if psCurr != psDroid &&
                               (*psCurr).selected as libc::c_int != 0 {
                            if (*psCurr).droidType as libc::c_uint ==
                                   DROID_WEAPON as libc::c_int as libc::c_uint
                                   ||
                                   (*psCurr).droidType as libc::c_uint ==
                                       DROID_CYBORG as libc::c_int as
                                           libc::c_uint ||
                                   (*psCurr).droidType as libc::c_uint ==
                                       DROID_CYBORG_SUPER as libc::c_int as
                                           libc::c_uint ||
                                   (*psCurr).droidType as libc::c_uint ==
                                       DROID_COMMAND as libc::c_int as
                                           libc::c_uint {
                                orderDroidObj(psCurr, DORDER_ATTACK,
                                              psClickedOn);
                                FeedbackOrderGiven();
                            } else if (*psCurr).droidType as libc::c_uint ==
                                          DROID_SENSOR as libc::c_int as
                                              libc::c_uint {
                                orderDroidObj(psCurr, DORDER_OBSERVE,
                                              psClickedOn);
                                FeedbackOrderGiven();
                            } else if (*psCurr).droidType as libc::c_uint ==
                                          DROID_REPAIR as libc::c_int as
                                              libc::c_uint ||
                                          (*psCurr).droidType as libc::c_uint
                                              ==
                                              DROID_CYBORG_REPAIR as
                                                  libc::c_int as libc::c_uint
                             {
                                orderDroidObj(psCurr, DORDER_DROIDREPAIR,
                                              psClickedOn);
                                FeedbackOrderGiven();
                            }
                        }
                        psCurr = (*psCurr).psNext
                    }
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_COMMAND as libc::c_int as libc::c_uint &&
                              selection as libc::c_uint !=
                                  SC_INVALID as libc::c_int as libc::c_uint &&
                              selection as libc::c_uint !=
                                  SC_DROID_COMMAND as libc::c_int as
                                      libc::c_uint &&
                              selection as libc::c_uint !=
                                  SC_DROID_CONSTRUCT as libc::c_int as
                                      libc::c_uint &&
                              !(keyDown(KEY_LCTRL) != 0 ||
                                    keyDown(KEY_RCTRL) != 0) &&
                              !(keyDown(KEY_LSHIFT) != 0 ||
                                    keyDown(KEY_RSHIFT) != 0) {
                    turnOffMultiMsg(1 as libc::c_int);
                    orderSelectedObj(selectedPlayer, psClickedOn);
                    FeedbackOrderGiven();
                    clearSelection();
                    assignSensorTarget(psDroid as *mut BASE_OBJECT);
                    dealWithDroidSelect(psDroid, 0 as libc::c_int);
                    turnOffMultiMsg(0 as libc::c_int);
                } else if ((*psDroid).droidType as libc::c_uint ==
                               DROID_CONSTRUCT as libc::c_int as libc::c_uint
                               ||
                               (*psDroid).droidType as libc::c_uint ==
                                   DROID_SENSOR as libc::c_int as
                                       libc::c_uint) &&
                              selection as libc::c_uint ==
                                  SC_DROID_DIRECT as libc::c_int as
                                      libc::c_uint {
                    orderSelectedObj(selectedPlayer, psClickedOn);
                    FeedbackOrderGiven();
                } else if droidIsDamaged(psDroid) != 0 &&
                              repairDroidSelected(selectedPlayer) != 0 {
                    assignDestTarget();
                    orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                        ctrlShiftDown());
                    FeedbackOrderGiven();
                } else {
                    // Clicked on a commander? Will link to it.
                    // Clicked on a construction unit? Will guard it.
                    // Clicked on a damaged unit? Will repair it.
                    // Just plain clicked on?
                    // Display unit info.
					/* We've clicked on one of our own droids */
                    if godMode != 0 {
                        sprintf(ConsoleString.as_mut_ptr(),
                                b"%s - Damage %d%% - Serial ID %d - Kills %d order %d action %d, %s\x00"
                                    as *const u8 as *const libc::c_char,
                                droidGetName(psDroid),
                                (100 as libc::c_int as
                                     libc::c_uint).wrapping_sub((*psDroid).body.wrapping_mul(100
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_div((*psDroid).originalBody)),
                                (*psDroid).id,
                                (*psDroid).numKills as libc::c_int,
                                (*psDroid).order, (*psDroid).action,
                                getDroidLevelName(psDroid));
                        addConsoleMessage(ConsoleString.as_mut_ptr(),
                                          DEFAULT_JUSTIFY);
                        FeedbackClickedOn();
                    } else if (*psDroid).selected == 0 {
                        sprintf(ConsoleString.as_mut_ptr(),
                                strresGetString(psStringRes,
                                                STR_GAM_DROIDSTATE as
                                                    libc::c_int as UDWORD),
                                droidGetName(psDroid),
                                (100 as libc::c_int as
                                     libc::c_uint).wrapping_sub((*psDroid).body.wrapping_mul(100
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_div((*psDroid).originalBody)),
                                (*psDroid).numKills as libc::c_int,
                                getDroidLevelName(psDroid));
                        addConsoleMessage(ConsoleString.as_mut_ptr(),
                                          DEFAULT_JUSTIFY);
                        FeedbackClickedOn();
                    }
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_SENSOR as libc::c_int as libc::c_uint {
                        //bWeapDroidSelected = FALSE;
                        bSensorAssigned = 0 as libc::c_int;
                        psCurr = apsDroidLists[selectedPlayer as usize];
                        while !psCurr.is_null() {
                            //must be indirect weapon droid or VTOL weapon droid
                            if (*psCurr).droidType as libc::c_uint ==
                                   DROID_WEAPON as libc::c_int as libc::c_uint
                                   && (*psCurr).selected as libc::c_int != 0
                                   &&
                                   (*psCurr).asWeaps[0 as libc::c_int as
                                                         usize].nStat >
                                       0 as libc::c_int as libc::c_uint &&
                                   (proj_Direct(asWeaponStats.offset((*psCurr).asWeaps[0
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           usize].nStat
                                                                         as
                                                                         isize))
                                        == 0 || vtolDroid(psCurr) != 0) &&
                                   droidSensorDroidWeapon(psDroid as
                                                              *mut BASE_OBJECT,
                                                          psCurr) != 0 {
                                //bWeapDroidSelected = TRUE;
                                bSensorAssigned = 1 as libc::c_int;
                                orderDroidObj(psCurr, DORDER_FIRESUPPORT,
                                              psDroid as *mut BASE_OBJECT);
                                FeedbackOrderGiven();
                            }
                            psCurr = (*psCurr).psNext
                        }
                        //if(bWeapDroidSelected)
                        if bSensorAssigned != 0 {
                            //assignSensorTarget(psDroid);
                            assignSensorTarget(psDroid as *mut BASE_OBJECT);
                        }
                    }
                    bLasSatStruct = 0 as libc::c_int;
                    if ctrlShiftDown() == 0 {
                        clearSelection();
                        dealWithDroidSelect(psDroid, 0 as libc::c_int);
                    }
                }
            } else {
                //cannot have LasSat struct and Droid selected
                // select/deselect etc. the droid
                /* We've clicked on somebody else's droid */
//				addConsoleMessage("Clicked on another player's droid",DEFAULT_JUSTIFY);
                orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                    ctrlShiftDown());
                //lasSat structure can select a target - in multiPlayer only
                if bMultiPlayer != 0 && bLasSatStruct != 0 {
                    if aiCheckAlliances(selectedPlayer,
                                        (*psClickedOn).player as UDWORD) ==
                           0 as libc::c_int {
                        orderStructureObj(selectedPlayer,
                                          psClickedOn); // end if its a structure
                    }
                }
                FeedbackOrderGiven();
                driveDisableTactical();
            }
        } else if (*psClickedOn).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            //			clearSelection();	// Clear droid selection.
            psStructure = psClickedOn as *mut STRUCTURE;
            if (*psStructure).player as libc::c_uint == selectedPlayer {
                /* We've clicked on our own building */
                //print some info at the top of the screen for the specific structure
                printStructureInfo(psStructure);
                /* Got to be built. Also, you can't 'select' derricks */
                if (*psStructure).status as libc::c_int ==
                       SS_BUILT as libc::c_int &&
                       (*(*psStructure).pStructureType).type_0 !=
                           REF_RESOURCE_EXTRACTOR as libc::c_int as
                               libc::c_uint {
                    //if selected object is an upgradeable structure then don't
					//inform the interface (if not fully upgraded) and a any droid
					//is selected
/*					if (!(((psStructure->pStructureType->type == REF_FACTORY AND
						((FACTORY *)psStructure->pFunctionality)->capacity <
						NUM_FACTORY_MODULES) OR
						(psStructure->pStructureType->type == REF_RESEARCH AND
						((RESEARCH_FACILITY *)psStructure->pFunctionality)->capacity <
						NUM_RESEARCH_MODULES) OR
						(psStructure->pStructureType->type == REF_VTOL_FACTORY AND
						((FACTORY *)psStructure->pFunctionality)->capacity <
						NUM_FACTORY_MODULES)) AND
						//constructorDroidSelected(selectedPlayer)))
						anyDroidSelected(selectedPlayer)))*/
					// now only display interface if nothing selected
                    if anyDroidSelected(selectedPlayer) == 0 {
                        intObjectSelected(psStructure as *mut BASE_OBJECT);
                        FeedbackClickedOn();
                    }
                    // We don't actually wan't to select structures, just inform the interface we've clicked on it,
// might wan't to do this on PC as well as it fixes the problem with the interface locking multiple
// buttons in the object window.
                    if selection as libc::c_uint ==
                           SC_INVALID as libc::c_int as libc::c_uint {
                        /* Clear old building selection(s) - should only be one */
                        psSLoop = apsStructLists[selectedPlayer as usize];
                        while !psSLoop.is_null() {
                            (*psSLoop).selected = 0 as libc::c_int as UBYTE;
                            psSLoop = (*psSLoop).psNext
                        }
                        /* Establish new one */
                        (*psStructure).selected = 1 as libc::c_int as UBYTE
                    }
                    //determine if LasSat structure has been selected
                    bLasSatStruct = 0 as libc::c_int;
                    if lasSatStructSelected(psStructure) != 0 {
                        bLasSatStruct = 1 as libc::c_int
                    }
                } else if (*psStructure).status as libc::c_int ==
                              SS_BUILT as libc::c_int &&
                              (*(*psStructure).pStructureType).type_0 ==
                                  REF_RESOURCE_EXTRACTOR as libc::c_int as
                                      libc::c_uint {
                    if selection as libc::c_uint ==
                           SC_INVALID as libc::c_int as libc::c_uint {
                        /* Clear old building selection(s) - should only be one */
                        psSLoop = apsStructLists[selectedPlayer as usize];
                        while !psSLoop.is_null() {
                            (*psSLoop).selected = 0 as libc::c_int as UBYTE;
                            psSLoop = (*psSLoop).psNext
                        }
                        /* Establish new one */
                        (*psStructure).selected = 1 as libc::c_int as UBYTE
                    }
                }
                if keyDown(KEY_LALT) != 0 || keyDown(KEY_RALT) != 0 {
                    // Like to implement this on PSX as well.
					// try to attack your own structure
                    psCurr = apsDroidLists[selectedPlayer as usize];
                    while !psCurr.is_null() {
                        if (*psCurr).selected != 0 {
                            //if ((psCurr->droidType == DROID_WEAPON) || (psCurr->droidType == DROID_CYBORG) ||
                            if (*psCurr).droidType as libc::c_uint ==
                                   DROID_WEAPON as libc::c_int as libc::c_uint
                                   || cyborgDroid(psCurr) != 0 ||
                                   (*psCurr).droidType as libc::c_uint ==
                                       DROID_COMMAND as libc::c_int as
                                           libc::c_uint {
                                orderDroidObj(psCurr, DORDER_ATTACK,
                                              psClickedOn);
                                FeedbackOrderGiven();
                            } else if (*psCurr).droidType as libc::c_uint ==
                                          DROID_SENSOR as libc::c_int as
                                              libc::c_uint {
                                orderDroidObj(psCurr, DORDER_OBSERVE,
                                              psClickedOn);
                                FeedbackOrderGiven();
                            }
                        }
                        psCurr = (*psCurr).psNext
                    }
                } else {
                    bSensorAssigned = 0 as libc::c_int;
                    orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                        ctrlShiftDown());
                    FeedbackOrderGiven();
                    if bSensorAssigned != 0 {
                        clearSelection();
                        assignSensorTarget(psStructure as *mut BASE_OBJECT);
                    }
                }
            } else {
                /* We've clicked on somebody else's building */
//				addConsoleMessage("Clicked on another player's building",DEFAULT_JUSTIFY);
                orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                    ctrlShiftDown());
                //lasSat structure can select a target - in multiPlayer only
                if bMultiPlayer != 0 && bLasSatStruct != 0 {
                    orderStructureObj(selectedPlayer, psClickedOn);
                }
                FeedbackOrderGiven();
            }
            driveDisableTactical();
        } else if (*psClickedOn).type_0 as libc::c_uint ==
                      OBJ_FEATURE as libc::c_int as libc::c_uint {
            //some features are targetable
            psFeature = psClickedOn as *mut FEATURE;
            //check for constructor droid trying to remove wrecked building first
            if (*(*psFeature).psStats).subType as libc::c_uint ==
                   FEAT_BUILD_WRECK as libc::c_int as libc::c_uint &&
                   !constructorDroidSelected(selectedPlayer).is_null() {
                orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                    ctrlShiftDown());
                FeedbackOrderGiven();
            }
            //go on to check for
            if (*(*psFeature).psStats).damageable != 0 {
                orderSelectedObjAdd(selectedPlayer, psClickedOn,
                                    ctrlShiftDown());
                //lasSat structure can select a target - in multiPlayer only
                if bMultiPlayer != 0 && bLasSatStruct != 0 {
                    orderStructureObj(selectedPlayer, psClickedOn);
                }
                FeedbackOrderGiven();
            }
            //clicking an oil field should start a build..
			//if(psFeature->subType == FEAT_OIL_RESOURCE)
            if (*(*psFeature).psStats).subType as libc::c_uint ==
                   FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                // find any construction droids. and order them to build an oil resource.
                // first find the derrick.
                i = 0 as libc::c_int as UDWORD;
                while i < numStructureStats &&
                          (*asStructureStats.offset(i as isize)).type_0 !=
                              REF_RESOURCE_EXTRACTOR as libc::c_int as
                                  libc::c_uint {
                    i = i.wrapping_add(1)
                }
                if i < numStructureStats &&
                       *apStructTypeLists[selectedPlayer as
                                              usize].offset(i as isize) as
                           libc::c_int == 0x1 as libc::c_int {
                    // dont go any further if no derrick stat found.
                    // for each droid
                    psDroid = apsDroidLists[selectedPlayer as usize];
                    while !psDroid.is_null() {
                        //if((droidType(psDroid) == DROID_CONSTRUCT) && (psDroid->selected))
                        if (droidType(psDroid) as libc::c_uint ==
                                DROID_CONSTRUCT as libc::c_int as libc::c_uint
                                ||
                                droidType(psDroid) as libc::c_uint ==
                                    DROID_CYBORG_CONSTRUCT as libc::c_int as
                                        libc::c_uint) &&
                               (*psDroid).selected as libc::c_int != 0 {
                            if fireOnLocation((*psFeature).x as UDWORD,
                                              (*psFeature).y as UDWORD) == 0 {
                                if ctrlShiftDown() != 0 {
                                    orderDroidStatsLocAdd(psDroid,
                                                          DORDER_BUILD,
                                                          &mut *asStructureStats.offset(i
                                                                                            as
                                                                                            isize)
                                                              as
                                                              *mut STRUCTURE_STATS
                                                              as
                                                              *mut BASE_STATS,
                                                          (*psFeature).x as
                                                              UDWORD,
                                                          (*psFeature).y as
                                                              UDWORD);
                                } else {
                                    orderDroidStatsLoc(psDroid, DORDER_BUILD,
                                                       &mut *asStructureStats.offset(i
                                                                                         as
                                                                                         isize)
                                                           as
                                                           *mut STRUCTURE_STATS
                                                           as *mut BASE_STATS,
                                                       (*psFeature).x as
                                                           UDWORD,
                                                       (*psFeature).y as
                                                           UDWORD);
                                }
                                addConsoleMessage(strresGetString(psStringRes,
                                                                  STR_GAM_DERRICK
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      UDWORD),
                                                  DEFAULT_JUSTIFY);
                                //				"Construction vehicle ordered to build a Derrick.",DEFAULT_JUSTIFY);
                                FeedbackOrderGiven();
                            } else {
                                // can't build because it's burning
                                AddDerrickBurningMessage();
                                break ;
                            }
                        }
                        psDroid = (*psDroid).psNext
                    }
                }
            } else {
                match (*(*psFeature).psStats).subType as libc::c_uint {
                    3 | 10 => {
                        psNearestUnit =
                            getNearestDroid((mouseTileX * 128 as libc::c_int +
                                                 128 as libc::c_int /
                                                     2 as libc::c_int) as
                                                UDWORD,
                                            (mouseTileY * 128 as libc::c_int +
                                                 128 as libc::c_int /
                                                     2 as libc::c_int) as
                                                UDWORD, 1 as libc::c_int);
                        /* If so then find the nearest unit! */
                        if !psNearestUnit.is_null() {
                            // bloody well should be!!!
                            //						orderDroidLoc(psNearestUnit,DORDER_MOVE, mouseTileX*TILE_UNITS+TILE_UNITS/2,mouseTileY*TILE_UNITS+TILE_UNITS/2);
                            orderDroidObj(psNearestUnit, DORDER_RECOVER,
                                          psClickedOn);
                            FeedbackOrderGiven();
                        } else {
                            //						orderSelectedLoc(selectedPlayer, psFeature->x,psFeature->y);	// recover it.
                            orderSelectedObj(selectedPlayer, psClickedOn);
                            FeedbackOrderGiven();
                        }
                    }
                    5 => { }
                    0 => { }
                    1 => { }
                    4 | 6 | _ => { }
                }
            }
            driveDisableTactical();
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy selection from LMB?!\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"display.c\x00" as *const u8 as *const libc::c_char,
                      2508 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"dealWithLMB\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    } else if driveModeActive() == 0 || driveTacticalActive() != 0 {
        /*Check for a Delivery Point or a Proximity Message*/
        psLocation = 0 as *mut OBJECT_POSITION;
        psLocation = checkMouseLoc();
        if !psLocation.is_null() && driveModeActive() == 0 &&
               selNumSelected(selectedPlayer) == 0 {
            match (*psLocation).type_0 as libc::c_uint {
                0 => {
                    if (*psLocation).player == selectedPlayer {
                        StartDeliveryPosition(psLocation, 0 as libc::c_int);
                        /* We've clicked on one of our own DP */
 //					addConsoleMessage("Clicked on your delivery point",DEFAULT_JUSTIFY);
                        //					/* clear the selection */
//					clearSelection();
//
//					//set this object position to be highlighted
//					psLocation->selected = TRUE;
                    }
                }
                _ => {
                    /*case POS_PROX:
				if(psLocation->player == selectedPlayer)
				{
					displayProximityMessage((PROXIMITY_DISPLAY *)psLocation);
				}
				break;*/
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Unknown type from checkMouseLoc\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"display.c\x00" as *const u8 as
                                  *const libc::c_char, 2547 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 12],
                                                        &[libc::c_char; 12]>(b"dealWithLMB\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
        } else {
            /* We've just clicked on an area of terrain. A 'send to' operation */
			/* We need to establish the world coordinates */
/*			if(keyDown(KEY_LALT) || keyDown(KEY_RALT))		// shift clicked a destination, add a waypoint.
			{
				orderSelectedWaypoint(selectedPlayer,mouseTileX*TILE_UNITS+TILE_UNITS/2,mouseTileY*TILE_UNITS+TILE_UNITS/2);
			}*/
            // now changed to use the multiple order stuff
            if ctrlShiftDown() != 0 {
                // shift clicked a destination, add an order
                orderSelectedLocAdd(selectedPlayer,
                                    (mouseTileX * 128 as libc::c_int +
                                         128 as libc::c_int /
                                             2 as libc::c_int) as UDWORD,
                                    (mouseTileY * 128 as libc::c_int +
                                         128 as libc::c_int /
                                             2 as libc::c_int) as UDWORD,
                                    1 as libc::c_int);
            } else {
                // clicked on a destination.
                /* Otherwise send them all */
                orderSelectedLoc(selectedPlayer,
                                 (mouseTileX * 128 as libc::c_int +
                                      128 as libc::c_int / 2 as libc::c_int)
                                     as UDWORD,
                                 (mouseTileY * 128 as libc::c_int +
                                      128 as libc::c_int / 2 as libc::c_int)
                                     as UDWORD);
                if getNumDroidsSelected() != 0 {
                    assignDestTarget();
                    audio_PlayTrack(ID_SOUND_SELECT as libc::c_int);
                }
                (godMode != 0 && mouseTileX >= 0 as libc::c_int &&
                     mouseTileX < mapWidth as SDWORD &&
                     mouseTileY >= 0 as libc::c_int) &&
                    mouseTileY < mapHeight as SDWORD;
            }
            driveDisableTactical();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getRotActive() -> BOOL { return rotActive; }
#[no_mangle]
pub unsafe extern "C" fn getDesiredPitch() -> SDWORD { return desiredPitch; }
#[no_mangle]
pub unsafe extern "C" fn setDesiredPitch(mut pitch: SDWORD) {
    desiredPitch = pitch;
}
//DBPRINTF(("orderSelectedLoc(%d,%d,%d)\n",selectedPlayer, mouseTileX*TILE_UNITS+TILE_UNITS/2,mouseTileY*TILE_UNITS+TILE_UNITS/2));
// process LMB double clicks
#[no_mangle]
pub unsafe extern "C" fn dealWithLMBDClick() {
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    /* What have we clicked on? */
    psClickedOn = mouseTarget();
    /* If not NULL, then it's a droid or a structure */
    if !psClickedOn.is_null() {
        /* We've got a droid or a structure */
        if (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            /* We clicked on droid */
            psDroid = psClickedOn as *mut DROID;
            if (*psDroid).player as libc::c_uint == selectedPlayer {
                /* If we've double clicked on a constructor droid, activate build menu */
				//if (psDroid->droidType == DROID_CONSTRUCT)
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                       (*psDroid).droidType as libc::c_uint ==
                           DROID_CYBORG_CONSTRUCT as libc::c_int as
                               libc::c_uint {
                    intResetScreen(1 as libc::c_int);
                    intConstructorSelected(psDroid);
                } else if (*psDroid).droidType as libc::c_uint ==
                              DROID_COMMAND as libc::c_int as libc::c_uint {
                    intResetScreen(1 as libc::c_int);
                    intCommanderSelected(psDroid);
                } else {
                    /* Otherwise, activate the droid's group (if any) */
//					activateGroup(selectedPlayer,psDroid->group);
					// Now selects all of smae type on screen
                    selDroidSelection(selectedPlayer, DS_BY_TYPE,
                                      DST_ALL_SAME, 1 as libc::c_int);
                }
            }
        }
    };
}
/*This checks to see if the mouse was over a delivery point or a proximity message
  when the mouse button was pressed */
#[no_mangle]
pub unsafe extern "C" fn checkMouseLoc() -> *mut OBJECT_POSITION {
    let mut psReturn: *mut OBJECT_POSITION = 0 as *mut OBJECT_POSITION;
    let mut psPoint: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    //PROXIMITY_DISPLAY	*psProxDisp;
    let mut i: UDWORD = 0;
    let mut dispX: UDWORD = 0;
    let mut dispY: UDWORD = 0;
    let mut dispR: UDWORD = 0;
    // We haven't found anything yet
    psReturn = 0 as *mut OBJECT_POSITION;
    // First have a look through the DeliveryPoint lists
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //new way, handles multiple points.
        psPoint = apsFlagPosLists[i as usize];
        while !psPoint.is_null() {
            dispX = (*psPoint).screenX;
            dispY = (*psPoint).screenY;
            dispR = (*psPoint).screenR;
            if DrawnInLastFrame((*psPoint).frameNumber as SDWORD) ==
                   1 as libc::c_int {
                // Only check DP's that are on screen
                if mouseInBox(dispX.wrapping_sub(dispR) as SDWORD,
                              dispY.wrapping_sub(dispR) as SDWORD,
                              dispX.wrapping_add(dispR) as SDWORD,
                              dispY.wrapping_add(dispR) as SDWORD) != 0 {
                    // We HAVE clicked on DP!
                    psReturn = psPoint as *mut OBJECT_POSITION;
                    //There's no point in checking other object types
                    return psReturn
                }
            }
            psPoint = (*psPoint).psNext
        }
        i = i.wrapping_add(1)
        // old way, only one point allowed.
//
//		//look throught the list of structures to see if there is a factory
//		//and therefore a DP
//		for (psStructure = apsStructLists[i]; psStructure; psStructure = psStructure->psNext)
//		{
//			if (psStructure->pStructureType->type == REF_FACTORY)
//			{
//				psPoint = ((FACTORY *)psStructure->pFunctionality)->psAssemblyPoint;
//				dispX = psPoint->screenX;
//				dispY = psPoint->screenY;
//				dispR = psPoint->screenR;
//				// Only check DP's that are on screen
//				if (DrawnInLastFrame(psPoint->frameNumber)==TRUE)
//				{
//					if (mouseInBox(dispX-dispR, dispY-dispR, dispX+dispR, dispY+dispR))
//					{
//						// We HAVE clicked on DP!
//						psReturn = psPoint;
//						//There's no point in checking other object types
//						return(psReturn);
//					}
//				}
//			}
//		} // end of checking for droids
    }
    //now check for Proximity Message
	/*for(psProxDisp = apsProxDisp[selectedPlayer]; psProxDisp; psProxDisp =
		psProxDisp->psNext)
	{
		dispX = psProxDisp->screenX;
		dispY = psProxDisp->screenY;
		dispR = psProxDisp->screenR;
		// Only check DP's that are on screen
		if (DrawnInLastFrame(psProxDisp->frameNumber)==TRUE)
		{
			if (mouseInBox(dispX-dispR, dispY-dispR, dispX+dispR, dispY+dispR))
			{
				// We HAVE clicked on Proximity Message!
				psReturn = (OBJECT_POSITION *)psProxDisp;
				//There's no point in checking other object types
				return(psReturn);
			}
		}
	}*/
    return 0 as *mut OBJECT_POSITION;
}
#[no_mangle]
pub unsafe extern "C" fn dealWithRMB() {
    let mut psClickedOn: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psSLoop: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psLocation: *mut OBJECT_POSITION = 0 as *mut OBJECT_POSITION;
    //printf("dealWithRMB %d\n",mouseOverRadar);
    if driveModeActive() != 0 { return }
    if mouseOverRadar != 0 { return }
    //RMB will always cancel the selection of the Las Sat struct
    bLasSatStruct = 0 as libc::c_int;
    /* What have we clicked on? */
    psClickedOn = mouseTarget();
    /* If not NULL, then it's a droid or a structure */
    if !psClickedOn.is_null() {
        /* We've got a droid or a structure */
        if (*psClickedOn).type_0 as libc::c_uint ==
               OBJ_DROID as libc::c_int as libc::c_uint {
            psDroid = psClickedOn as *mut DROID; // end if its a droid
            if (*psDroid).player as libc::c_uint == selectedPlayer {
                //			if(radarCheckForHQ(selectedPlayer))	// removed by Jim, well kind of, he asked 19 oct 98
                /* We clicked on droid */
                //ignore RMB on a Transporter - for now?
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    /* We've clicked on one of our own droids */
		//				addGameMessage("Right clicked on own droid",1000,TRUE);
	//					addConsoleMessage("Right click detected on own droid",DEFAULT_JUSTIFY);
                    if (*psDroid).selected as libc::c_int == 1 as libc::c_int
                       {
                        //					psDroid->selected = FALSE;
		//					intObjectSelected(NULL);
                        intObjectSelected(psDroid as *mut BASE_OBJECT);
                    } else {
                        clearSelection();
                        //							psDroid->selected = TRUE;
                        SelectDroid(psDroid);
                        intObjectSelected(psDroid as *mut BASE_OBJECT);
                    }
                } else if bMultiPlayer != 0 {
                    intResetScreen(0 as libc::c_int);
                    if getWidgetsStatus() == 0 {
                        setWidgetsStatus(1 as libc::c_int);
                    }
                    addTransporterInterface(psDroid, 0 as libc::c_int);
                }
            } else if bMultiPlayer != 0 {
                if isHumanPlayer((*psDroid).player as UDWORD) != 0 {
                    sprintf(ConsoleString.as_mut_ptr(),
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            droidGetName(psDroid));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    FeedbackClickedOn();
                }
            }
        } else if (*psClickedOn).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            //it was only just 'for now'!!!
            psStructure =
                psClickedOn as *mut STRUCTURE; // end if its a structure
            if (*psStructure).player as libc::c_uint == selectedPlayer {
                /* We've clicked on our own building */
//				addGameMessage("Right clicked on own building",1000,TRUE);
//				addConsoleMessage("Right clicked on own building",DEFAULT_JUSTIFY);
                if (*psStructure).selected as libc::c_int == 1 as libc::c_int
                   {
                    (*psStructure).selected = 0 as libc::c_int as UBYTE;
                    intObjectSelected(0 as *mut BASE_OBJECT);
                } else {
                    // We don't actually wan't to select structures, just inform the interface weve clicked on it,
// might wan't to do this on PC as well as it fixes the problem with the interface locking multiple
// buttons in the object window.
//					clearSelection();
					/* Clear old building selection(s) - should only be one */
                    psSLoop = apsStructLists[selectedPlayer as usize];
                    while !psSLoop.is_null() {
                        (*psSLoop).selected = 0 as libc::c_int as UBYTE;
                        psSLoop = (*psSLoop).psNext
                    }
                    //					psStructure->selected = TRUE;
					//if a factory go to deliv point rather than setting up the interface
                    if StructIsFactory(psStructure) != 0 {
                        setViewPos(((*(*((*psStructure).pFunctionality as
                                             *mut FACTORY)).psAssemblyPoint).coords.x
                                        >> 7 as libc::c_int) as UDWORD,
                                   ((*(*((*psStructure).pFunctionality as
                                             *mut FACTORY)).psAssemblyPoint).coords.y
                                        >> 7 as libc::c_int) as UDWORD,
                                   1 as libc::c_int);
                        //pop up the order interface for the factory - AB 21/04/99 Patch v1.2->
                        intAddFactoryOrder(psStructure);
                    } else if (*(*psStructure).pStructureType).type_0 ==
                                  REF_REPAIR_FACILITY as libc::c_int as
                                      libc::c_uint {
                        setViewPos(((*(*((*psStructure).pFunctionality as
                                             *mut REPAIR_FACILITY)).psDeliveryPoint).coords.x
                                        >> 7 as libc::c_int) as UDWORD,
                                   ((*(*((*psStructure).pFunctionality as
                                             *mut REPAIR_FACILITY)).psDeliveryPoint).coords.y
                                        >> 7 as libc::c_int) as UDWORD,
                                   1 as libc::c_int);
                    } else {
                        intObjectSelected(psStructure as *mut BASE_OBJECT);
                    }
                }
            }
        } else if (*psClickedOn).type_0 as libc::c_uint !=
                      OBJ_FEATURE as libc::c_int as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Weirdy selection from RMB?!\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"display.c\x00" as *const u8 as *const libc::c_char,
                      2885 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 12],
                                                &[libc::c_char; 12]>(b"dealWithRMB\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    } else {
        /* And if it's not a feature, then we're in trouble! */
        /*Check for a Delivery Point*/
        psLocation = 0 as *mut OBJECT_POSITION;
        psLocation = checkMouseLoc();
        if !psLocation.is_null() {
            match (*psLocation).type_0 as libc::c_uint {
                0 => {
                    if (*psLocation).player == selectedPlayer {
                        //centre the view on the owning Factory
                        psStructure =
                            findDeliveryFactory(psLocation as
                                                    *mut FLAG_POSITION);
                        if !psStructure.is_null() {
                            setViewPos(((*psStructure).x as libc::c_int >>
                                            7 as libc::c_int) as UDWORD,
                                       ((*psStructure).y as libc::c_int >>
                                            7 as libc::c_int) as UDWORD,
                                       1 as libc::c_int);
                        }
                    }
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Unknown type from checkMouseLoc\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"display.c\x00" as *const u8 as
                                  *const libc::c_char, 2910 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 12],
                                                        &[libc::c_char; 12]>(b"dealWithRMB\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
        } else {
            /* We've just clicked on an area of terrain. A 'send to' operation
            for Transporter in multiPlay mode*/
            /*if (bMultiPlayer)
            {
                //there may be more than one Transporter selected
                for (psDroid = apsDroidLists[selectedPlayer]; psDroid != NULL;
                    psDroid = psDroid->psNext)
                {
                    if (psDroid->selected)
                    {
                        if (psDroid->droidType == DROID_TRANSPORTER)
                        {
                            orderDroidLoc(psDroid, DORDER_DISEMBARK, mouseTileX *
                                TILE_UNITS + TILE_UNITS/2, mouseTileY * TILE_UNITS +
                                TILE_UNITS/2);
                        }
                        else
                        {
                            //de-select any other units
                			psDroid->selected = FALSE;
                        }
                    }
                 }
            }
            else*/
            clearSelection();
            intObjectSelected(0 as *mut BASE_OBJECT);
        }
    };
}
// wrapper functions ... for psx
#[no_mangle]
pub unsafe extern "C" fn ResetMouseBoundryConditions() {
    mouseAtLeft = 0 as libc::c_int;
    mouseAtRight = 0 as libc::c_int;
    mouseAtTop = 0 as libc::c_int;
    mouseAtBottom = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn IsMouseAtLeft() -> BOOL { return mouseAtLeft; }
#[no_mangle]
pub unsafe extern "C" fn IsMouseAtRight() -> BOOL { return mouseAtRight; }
#[no_mangle]
pub unsafe extern "C" fn IsMouseAtTop() -> BOOL { return mouseAtTop; }
#[no_mangle]
pub unsafe extern "C" fn IsMouseAtBottom() -> BOOL { return mouseAtBottom; }
// Local prototypes
/* if there is a valid object under the mouse this routine returns not only the type of the object in the
return code, but also a pointer to the BASE_OBJECT) ... well if your going to be "object orientated" you might as well do it right
  - it sets it to null if we don't find anything
*/
unsafe extern "C" fn itemUnderMouse(mut ppObjectUnderMouse:
                                        *mut *mut BASE_OBJECT)
 -> MOUSE_TARGET {
    let mut i: UDWORD = 0;
    let mut retVal: MOUSE_TARGET = MT_TERRAIN;
    let mut psNotDroid: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut dispX: UDWORD = 0;
    let mut dispY: UDWORD = 0;
    let mut dispR: UDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    *ppObjectUnderMouse = 0 as *mut BASE_OBJECT;
    if driveModeActive() == 0 || driveTacticalActive() != 0 {
        if mouseTileX < 0 as libc::c_int || mouseTileY < 0 as libc::c_int ||
               mouseTileX >
                   mapWidth.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                       SDWORD ||
               mouseTileY >
                   mapHeight.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                       SDWORD {
            retVal = MT_BLOCKING;
            return retVal
        }
    }
    //DBPRINTF(("%d %d\n",mouseTileX,mouseTileY);
    /* We haven't found anything yet */
    retVal = MT_NOTARGET;
    if driveModeActive() != 0 && driveTacticalActive() == 0 {
        let mut psObj: *mut BASE_OBJECT = targetGetCurrent();
        if !psObj.is_null() {
            if (*psObj).type_0 as libc::c_uint ==
                   OBJ_DROID as libc::c_int as libc::c_uint {
                psDroid = psObj as *mut DROID;
                if (*psDroid).player as libc::c_uint == selectedPlayer {
                    *ppObjectUnderMouse = psObj;
                    // need to check for command droids here as well
                    if (*psDroid).droidType as libc::c_uint ==
                           DROID_SENSOR as libc::c_int as libc::c_uint {
                        //	DBPRINTF(("MT_SENSOR\n");
                        return MT_SENSOR
                    } else {
                        if (*psDroid).droidType as libc::c_uint ==
                               DROID_TRANSPORTER as libc::c_int as
                                   libc::c_uint {
                            //	DBPRINTF(("MT_TRANDROID\n");
                        //check the transporter is not full
                            if calcRemainingCapacity(psDroid) != 0 {
                                return MT_TRANDROID
                            } else { return MT_BLOCKING }
                        } else {
                            if (*psDroid).droidType as libc::c_uint ==
                                   DROID_CONSTRUCT as libc::c_int as
                                       libc::c_uint ||
                                   (*psDroid).droidType as libc::c_uint ==
                                       DROID_CYBORG_CONSTRUCT as libc::c_int
                                           as libc::c_uint {
                                return MT_CONSTRUCT
                            } else {
                                if (*psDroid).droidType as libc::c_uint ==
                                       DROID_COMMAND as libc::c_int as
                                           libc::c_uint {
                                    //	DBPRINTF(("MT_COMMAND\n");
                                    return MT_COMMAND
                                } else {
                                    if droidIsDamaged(psDroid) != 0 {
                                        //	DBPRINTF(("MT_OWNDROIDDAM\n");
                                        return MT_OWNDROIDDAM
                                    }
                                }
                            }
                        }
                    }
                } else {
                    //	DBPRINTF(("MT_ENEMYDROID\n");
                    return MT_ENEMYDROID
                }
            }
            //			if( (psObj->type == OBJ_DROID) && (psObj->player != selectedPlayer) ) {
//				return MT_ENEMYDROID;
//			}
        }
    } else {
        /* First have a look through the droid lists */
        i = 0 as libc::c_int as UDWORD;
        while i < 8 as libc::c_int as libc::c_uint {
            /* Note the !psObject check isn't really necessary as the goto will jump out */
            psDroid = apsDroidLists[i as usize];
            while !psDroid.is_null() &&
                      retVal as libc::c_uint ==
                          MT_NOTARGET as libc::c_int as libc::c_uint {
                dispX = (*psDroid).sDisplay.screenX;
                dispY = (*psDroid).sDisplay.screenY;
                dispR = (*psDroid).sDisplay.screenR;
                /* Only check droids that're on screen */
                if (*psDroid).sDisplay.frameNumber.wrapping_add(1 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint)
                       == currentFrame &&
                       (*psDroid).visible[selectedPlayer as usize] as
                           libc::c_int != 0 {
                    if mouseInBox(dispX.wrapping_sub(dispR) as SDWORD,
                                  dispY.wrapping_sub(dispR) as SDWORD,
                                  dispX.wrapping_add(dispR) as SDWORD,
                                  dispY.wrapping_add(dispR) as SDWORD) != 0 {
                        /* We HAVE clicked on droid! */
                        if (*psDroid).player as libc::c_uint == selectedPlayer
                           {
                            *ppObjectUnderMouse = psDroid as *mut BASE_OBJECT;
                            // need to check for command droids here as well
                            if (*psDroid).droidType as libc::c_uint ==
                                   DROID_SENSOR as libc::c_int as libc::c_uint
                               {
                                retVal = MT_SENSOR
                            } else if (*psDroid).droidType as libc::c_uint ==
                                          DROID_TRANSPORTER as libc::c_int as
                                              libc::c_uint {
                                //check the transporter is not full
                                if calcRemainingCapacity(psDroid) != 0 {
                                    retVal = MT_TRANDROID
                                } else { retVal = MT_BLOCKING }
                            } else if (*psDroid).droidType as libc::c_uint ==
                                          DROID_CONSTRUCT as libc::c_int as
                                              libc::c_uint ||
                                          (*psDroid).droidType as libc::c_uint
                                              ==
                                              DROID_CYBORG_CONSTRUCT as
                                                  libc::c_int as libc::c_uint
                             {
                                return MT_CONSTRUCT
                            } else {
                                if (*psDroid).droidType as libc::c_uint ==
                                       DROID_COMMAND as libc::c_int as
                                           libc::c_uint {
                                    retVal = MT_COMMAND
                                } else if droidIsDamaged(psDroid) != 0 {
                                    retVal = MT_OWNDROIDDAM
                                } else { retVal = MT_OWNDROID }
                            }
                        } else {
                            *ppObjectUnderMouse = psDroid as *mut BASE_OBJECT;
                            //printf("Enemy Droid %d %d %d\n",dispX,dispY,dispR);
                            retVal = MT_ENEMYDROID
                        }
                        /* There's no point in checking other object types */
                        return retVal
                    }
                }
                psDroid = (*psDroid).psNext
            }
            i = i.wrapping_add(1)
        }
        // end of checking for droids
    }
    /*	Not a droid, so maybe a structure or feature?
		If still NULL after this then nothing */
    if driveModeActive() != 0 && driveTacticalActive() == 0 {
        psNotDroid = targetGetCurrent()
    } else {
        psNotDroid =
            getTileOccupier(mouseTileX as UDWORD, mouseTileY as UDWORD)
    }
    if !psNotDroid.is_null() {
        *ppObjectUnderMouse = psNotDroid;
        if (*psNotDroid).type_0 as libc::c_uint ==
               OBJ_FEATURE as libc::c_int as libc::c_uint {
            if (*(*(psNotDroid as *mut FEATURE)).psStats).subType as
                   libc::c_uint ==
                   FEAT_GEN_ARTE as libc::c_int as libc::c_uint ||
                   (*(*(psNotDroid as *mut FEATURE)).psStats).subType as
                       libc::c_uint ==
                       FEAT_OIL_DRUM as libc::c_int as libc::c_uint {
                retVal = MT_ARTIFACT
            } else if (*(*(psNotDroid as *mut FEATURE)).psStats).damageable !=
                          0 {
                //make damageable features return 'target' mouse pointer
                //printf("Damagable Feature %d %d\n",mouseTileX,mouseTileY);
                retVal = MT_DAMFEATURE
            } else if (*(*(psNotDroid as *mut FEATURE)).psStats).subType as
                          libc::c_uint ==
                          FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                retVal = MT_RESOURCE
            } else if (*(*(psNotDroid as *mut FEATURE)).psStats).subType as
                          libc::c_uint ==
                          FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                retVal = MT_WRECKFEATURE
            } else { retVal = MT_BLOCKING }
        } else if (*psNotDroid).type_0 as libc::c_uint ==
                      OBJ_STRUCTURE as libc::c_int as libc::c_uint {
            psStructure = psNotDroid as *mut STRUCTURE;
            if (*psNotDroid).player as libc::c_uint == selectedPlayer {
                if (*psStructure).status as libc::c_int ==
                       SS_BEING_BUILT as libc::c_int {
                    retVal = MT_OWNSTRINCOMP
                } else if (*(*psStructure).pStructureType).type_0 ==
                              REF_REPAIR_FACILITY as libc::c_int as
                                  libc::c_uint {
                    if buildingDamaged(psStructure) != 0 {
                        retVal = MT_REPAIRDAM
                    } else { retVal = MT_REPAIR }
                } else if !(*(*psStructure).pStructureType).pSensor.is_null()
                              &&
                              (*(*(*psStructure).pStructureType).pSensor).location
                                  == LOC_TURRET as libc::c_int as libc::c_uint
                 {
                    if buildingDamaged(psStructure) != 0 {
                        retVal = MT_SENSORSTRUCT
                    } else { retVal = MT_SENSORSTRUCTDAM }
                } else if buildingDamaged(psStructure) != 0 {
                    retVal = MT_OWNSTRDAM
                } else if (*(*psStructure).pStructureType).type_0 ==
                              REF_FACTORY as libc::c_int as libc::c_uint &&
                              ((*((*psStructure).pFunctionality as
                                      *mut FACTORY)).capacity as libc::c_int)
                                  < 2 as libc::c_int ||
                              (*(*psStructure).pStructureType).type_0 ==
                                  REF_VTOL_FACTORY as libc::c_int as
                                      libc::c_uint &&
                                  ((*((*psStructure).pFunctionality as
                                          *mut FACTORY)).capacity as
                                       libc::c_int) < 2 as libc::c_int ||
                              (*(*psStructure).pStructureType).type_0 ==
                                  REF_POWER_GEN as libc::c_int as libc::c_uint
                                  &&
                                  (*((*psStructure).pFunctionality as
                                         *mut POWER_GEN)).capacity <
                                      4 as libc::c_int as libc::c_uint ||
                              (*(*psStructure).pStructureType).type_0 ==
                                  REF_RESEARCH as libc::c_int as libc::c_uint
                                  &&
                                  (*((*psStructure).pFunctionality as
                                         *mut RESEARCH_FACILITY)).capacity <
                                      4 as libc::c_int as libc::c_uint {
                    retVal = MT_OWNSTRINCOMP
                } else {
                    // repair center.
                    //sensor tower
                    // standard buildings. - check for buildingDamaged BEFORE upgrades
                    // if factory/powgen/research and not upgraded. make build icon available..
                    // standard buildings.
				//else if(buildingDamaged(psStructure))
				//{
				//	retVal = MT_OWNSTRDAM;
				//}
                    /* All the different stages of construction */
                    retVal = MT_OWNSTROK
                }
            } else {
                retVal = MT_ENEMYSTR
                // enemy structure
            }
        }
    }
    /* Send the result back - if it's null then we clicked on an area of terrain */
	/* make unseen objects just look like terrain. */
    if retVal as libc::c_uint == MT_NOTARGET as libc::c_int as libc::c_uint ||
           (*psNotDroid).visible[selectedPlayer as usize] == 0 {
        retVal = MT_TERRAIN
    }
    return retVal;
}
// Indicates the priority given to any given droid
// type in a multiple droid selection, the larger the
// number, the lower the priority. The order of entries
// corresponds to the order of droid types in the DROID_TYPE
// enum in DroidDef.h
//
//#define NUM_DROID_WEIGHTS (10)
#[no_mangle]
pub static mut DroidSelectionWeights: [UBYTE; 13] =
    [3 as libc::c_int as UBYTE, 1 as libc::c_int as UBYTE,
     2 as libc::c_int as UBYTE, 4 as libc::c_int as UBYTE,
     3 as libc::c_int as UBYTE, 3 as libc::c_int as UBYTE,
     9 as libc::c_int as UBYTE, 0 as libc::c_int as UBYTE,
     4 as libc::c_int as UBYTE, 5 as libc::c_int as UBYTE,
     4 as libc::c_int as UBYTE, 4 as libc::c_int as UBYTE,
     3 as libc::c_int as UBYTE];
//BOOL	widgetsOn=TRUE;	//FALSE;
//BOOL	forceWidgetsOn = FALSE;
/* Only deals with one type of droid being selected!!!! */
/*	We'll have to make it assesss which selection is to be dominant in the case
	of multiple selections */
#[no_mangle]
pub unsafe extern "C" fn establishSelection(mut selectedPlayer_0: UDWORD)
 -> SELECTION_TYPE {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psDominant: *mut DROID = 0 as *mut DROID;
    let mut CurrWeight: UBYTE = 0;
    //BOOL			gotWeapon = FALSE;
//DROID			*psWeapDroid = NULL;
    let mut atLeastOne: BOOL = 0;
    let mut selectionClass: SELECTION_TYPE = SC_DROID_CONSTRUCT;
    atLeastOne = 0 as libc::c_int;
    selectionClass = SC_INVALID;
    CurrWeight = 0xff as libc::c_int as UBYTE;
    psDroid = apsDroidLists[selectedPlayer_0 as usize];
    while !psDroid.is_null() {
        // This code dos'nt work, what about the case of a selection of DROID_WEAPON types with a
		// DROID_CONSTRUCT type grouped with them,claims to handle this but dos'nt.
//PD		if(psDroid->selected)
//PD		{
//PD			atLeastOne = TRUE;
//PD			if(psDroid->type == DROID_WEAPON)
//PD			{
//PD				gotWeapon = TRUE;
//PD				psWeapDroid = psDroid;
//PD			}
//PD			if (psDroid->droidType == DROID_COMMAND ||
//PD				psDominant == NULL)
//PD			{
//PD				psDominant = psDroid;
//PD			}
//PD		}
        // This works, uses the DroidSelectionWeights[] table to priorities the different
		// droid types and find the dominant selection.
        if (*psDroid).selected != 0 {
            if ((*psDroid).droidType as libc::c_uint) <
                   13 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"establishSelection : droidType exceeds NUM_DROID_WEIGHTS\x00"
                          as *const u8 as *const libc::c_char);
            };
            if ((*psDroid).droidType as libc::c_uint) <
                   13 as libc::c_int as libc::c_uint {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"display.c\x00" as *const u8 as *const libc::c_char,
                      3334 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"establishSelection\x00")).as_ptr(),
                      b"psDroid->droidType < NUM_DROID_WEIGHTS\x00" as
                          *const u8 as *const libc::c_char);
            };
            atLeastOne = 1 as libc::c_int;
            if (DroidSelectionWeights[(*psDroid).droidType as usize] as
                    libc::c_int) < CurrWeight as libc::c_int {
                CurrWeight =
                    DroidSelectionWeights[(*psDroid).droidType as usize];
                psDominant = psDroid
            }
        }
        psDroid = (*psDroid).psNext
    }
    //	/* Weapon droids in a selection will override all others */
//	if(psWeapDroid)
//	{
//		psDominant = psWeapDroid;
//	}
    //	if(psDominant) {
//		DBPRINTF(("Dominant selection type == %d\n",psDominant->droidType));
//	}
    if atLeastOne != 0 {
        psDominantSelected = psDominant;
        match (*psDominant).droidType as libc::c_uint {
            0 => {
                if proj_Direct(asWeaponStats.offset((*psDominant).asWeaps[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize].nStat
                                                        as isize)) != 0 {
                    selectionClass = SC_DROID_DIRECT
                } else { selectionClass = SC_DROID_INDIRECT }
            }
            4 => { selectionClass = SC_DROID_DIRECT }
            5 | 12 => { selectionClass = SC_DROID_DIRECT }
            6 => {
                //can remove this is NEVER going to select the Transporter to move
            //Never say Never!! cos here we go in multiPlayer!!
                selectionClass = SC_DROID_TRANSPORTER
            }
            1 => { selectionClass = SC_DROID_SENSOR }
            2 => { selectionClass = SC_DROID_ECM }
            3 | 10 => {
                /* Re-written to allow demolish order to be added to the queuing system
			if ((psDominant->psTarget == NULL AND psDominant->psTarStats ==
                (BASE_STATS *) structGetDemolishStat())*/
                if intDemolishSelectMode() != 0 {
                    selectionClass = SC_DROID_DEMOLISH
                    // demolish mode.
                } else {
                    selectionClass = SC_DROID_CONSTRUCT
                    // ordinary mode.
                }
            }
            7 => { selectionClass = SC_DROID_COMMAND }
            8 | 11 => { selectionClass = SC_DROID_REPAIR }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Weirdy droid type on what you\'ve clicked on!!!\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"display.c\x00" as *const u8 as
                              *const libc::c_char, 3415 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"establishSelection\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
        }
    }
    return selectionClass;
}
// acceleration on scrolling. Game Option.
/* Just returns true if the building's present body points aren't 100 percent */
#[no_mangle]
pub unsafe extern "C" fn buildingDamaged(mut psStructure: *mut STRUCTURE)
 -> BOOL {
    //if( PERCENT(psStructure->body , psStructure->baseBodyPoints ) < 100)
    if (((*psStructure).body as libc::c_int * 100 as libc::c_int) as
            libc::c_uint).wrapping_div(structureBody(psStructure)) <
           100 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/*Looks through the list of selected players droids to see if one is a repair droid*/
unsafe extern "C" fn repairDroidSelected(mut player_0: UDWORD) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    psCurr = apsDroidLists[player_0 as usize];
    while !psCurr.is_null() {
        //if (psCurr->selected AND psCurr->droidType == DROID_REPAIR)
        if (*psCurr).selected as libc::c_int != 0 &&
               ((*psCurr).droidType as libc::c_uint ==
                    DROID_REPAIR as libc::c_int as libc::c_uint ||
                    (*psCurr).droidType as libc::c_uint ==
                        DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint) {
            return 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    //didn't find one...
    return 0 as libc::c_int;
}
/*Looks through the list of selected players droids to see if one is a constructor droid*/
#[no_mangle]
pub unsafe extern "C" fn constructorDroidSelected(mut player_0: UDWORD)
 -> *mut DROID {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    psCurr = apsDroidLists[player_0 as usize];
    while !psCurr.is_null() {
        //if (psCurr->selected AND psCurr->droidType == DROID_CONSTRUCT)
        if (*psCurr).selected as libc::c_int != 0 &&
               ((*psCurr).droidType as libc::c_uint ==
                    DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                    (*psCurr).droidType as libc::c_uint ==
                        DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint)
           {
            return psCurr
        }
        psCurr = (*psCurr).psNext
    }
    //didn't find one...
    return 0 as *mut DROID;
}
//static DROID *constructorDroidSelected(UDWORD player);
/*Looks through the list of selected players droids to see if one is a VTOL droid*/
unsafe extern "C" fn vtolDroidSelected(mut player_0: UDWORD) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    psCurr = apsDroidLists[player_0 as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected as libc::c_int != 0 && vtolDroid(psCurr) != 0 {
            // horrible hack to note one of the selected vtols
            psSelectedVtol = psCurr;
            return 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    //didn't find one...
    return 0 as libc::c_int;
}
/*Looks through the list of selected players droids to see if any is selected*/
unsafe extern "C" fn anyDroidSelected(mut player_0: UDWORD) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    psCurr = apsDroidLists[player_0 as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected != 0 { return 1 as libc::c_int }
        psCurr = (*psCurr).psNext
    }
    //didn't find one...
    return 0 as libc::c_int;
}
/*Looks through the list of selected players droids to see if one is a cyborg droid*/
unsafe extern "C" fn cyborgDroidSelected(mut player_0: UDWORD) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    psCurr = apsDroidLists[player_0 as usize];
    while !psCurr.is_null() {
        if (*psCurr).selected as libc::c_int != 0 && cyborgDroid(psCurr) != 0
           {
            return 1 as libc::c_int
        }
        psCurr = (*psCurr).psNext
    }
    //didn't find one...
    return 0 as libc::c_int;
}
/* Clear the selection flag for a player */
#[no_mangle]
pub unsafe extern "C" fn clearSel() {
    let mut psCurrDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //FEATURE			*psFeat;
    let mut psFlagPos: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    psCurrDroid = apsDroidLists[selectedPlayer as usize];
    while !psCurrDroid.is_null() {
        (*psCurrDroid).selected = 0 as libc::c_int as UBYTE;
        psCurrDroid = (*psCurrDroid).psNext
    }
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        (*psStruct).selected = 0 as libc::c_int as UBYTE;
        psStruct = (*psStruct).psNext
    }
    //can a feature ever be selected?
	/*for(psFeat = apsFeatureLists[0]; psFeat;
		psFeat = psFeat->psNext)
	{
		psFeat->selected = FALSE;
	}*/
	//clear the Deliv Point if one
    psFlagPos = apsFlagPosLists[selectedPlayer as usize];
    while !psFlagPos.is_null() {
        (*psFlagPos).selected = 0 as libc::c_int;
        psFlagPos = (*psFlagPos).psNext
    }
    setSelectedGroup(0xff as libc::c_int as UDWORD);
    setSelectedCommander(0xff as libc::c_int as UDWORD);
    intRefreshScreen();
}
// Clear the selection and stop driver mode.
//
#[no_mangle]
pub unsafe extern "C" fn clearSelection() {
    StopDriverMode(); // Cancel driver mode ( if active ).
    clearSel();
}
//access function for bSensorAssigned variable
#[no_mangle]
pub unsafe extern "C" fn setSensorAssigned() {
    bSensorAssigned = 1 as libc::c_int;
}
