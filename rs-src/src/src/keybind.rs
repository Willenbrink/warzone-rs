use ::libc;
extern "C" {
    pub type _formation;
    pub type _gateway;
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* This returns true if the key is currently depressed */
    #[no_mangle]
    fn keyDown(code: KEY_CODE) -> BOOL;
    /* This returns true if the key went from being up to being down this frame */
    #[no_mangle]
    fn keyPressed(code: KEY_CODE) -> BOOL;
    /* The input buffer can contain normal character codes and these control codes */
    /* Some defines for keys that map into the normal character space */
    /* Return the next key press or 0 if no key in the buffer.
 * The key returned will have been remaped to the correct ascii code for the
 * windows key map.
 * All key presses are buffered up (including windows auto repeat).
 */
    #[no_mangle]
    fn inputGetKey() -> UDWORD;
    #[no_mangle]
    fn inputGetCharKey() -> libc::c_char;
    /* Clear the input buffer */
    #[no_mangle]
    fn inputClearBuffer();
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
    fn strlen(_: *const libc::c_char) -> libc::c_uint;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Return the current frame rate */
    #[no_mangle]
    fn frameGetFrameRate() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    fn audio_GetPreviousQueueTrackPos(iX: *mut SDWORD, iY: *mut SDWORD,
                                      iZ: *mut SDWORD) -> BOOL;
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    #[no_mangle]
    fn assignDroidsToGroup(playerNumber: UDWORD, groupNumber: UDWORD);
    #[no_mangle]
    fn activateGroup(playerNumber: UDWORD, groupNumber: UDWORD) -> BOOL;
    #[no_mangle]
    fn activateGroupAndMove(playerNumber: UDWORD, groupNumber: UDWORD)
     -> BOOL;
    /* Remove a structure and free it's memory */
    #[no_mangle]
    fn destroyStruct(psDel: *mut STRUCTURE) -> BOOL;
    /*Initialise the production list and set up the production player*/
    #[no_mangle]
    fn changeProductionPlayer(player_0: UBYTE);
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    fn moveToggleFormationSpeedLimiting();
    #[no_mangle]
    fn moveFormationSpeedLimitingOn() -> BOOL;
    /* The maximum map size */
    /* The size and contents of the map */
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    /* Return height of x,y */
//extern SDWORD map_Height(UDWORD x, UDWORD y);
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    //scroll min and max values
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn camToggleInfo();
    #[no_mangle]
    fn getRadarTrackingStatus() -> BOOL;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn flushConsoleMessages();
    #[no_mangle]
    fn enableConsoleDisplay(state: BOOL);
    #[no_mangle]
    fn getConsoleDisplayStatus() -> BOOL;
    #[no_mangle]
    fn setConsolePermanence(state: BOOL, bClearOld: BOOL);
    #[no_mangle]
    fn permitNewConsoleMessages(allow: BOOL);
    #[no_mangle]
    fn toggleConsoleDrop();
    // / Print to the ingame console in debug mode only
    #[no_mangle]
    fn console(pFormat: *const libc::c_char, _: ...);
    #[no_mangle]
    fn setInvertMouseStatus(val: BOOL);
    #[no_mangle]
    fn getInvertMouseStatus() -> BOOL;
    #[no_mangle]
    fn setDrawShadows(val: BOOL);
    #[no_mangle]
    fn getDrawShadows() -> BOOL;
    #[no_mangle]
    fn getRadarJumpStatus() -> BOOL;
    #[no_mangle]
    fn setRadarJump(val: BOOL);
    #[no_mangle]
    static mut gameStats: BOOL;
    #[no_mangle]
    static mut godMode: BOOL;
    #[no_mangle]
    static mut RadarZoomLevel: UWORD;
    #[no_mangle]
    fn setDesiredPitch(pitch: SDWORD);
    #[no_mangle]
    fn setShakeStatus(val: BOOL);
    #[no_mangle]
    fn getShakeStatus() -> BOOL;
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    static mut rangeOnScreen: BOOL;
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    #[no_mangle]
    fn toggleReloadBarDisplay();
    #[no_mangle]
    fn toggleEnergyBars();
    #[no_mangle]
    fn doWeDrawRadarBlips() -> BOOL;
    #[no_mangle]
    fn doWeDrawProximitys() -> BOOL;
    #[no_mangle]
    fn setBlipDraw(val: BOOL);
    #[no_mangle]
    fn setProximityDraw(val: BOOL);
    #[no_mangle]
    static mut player: iView;
    #[no_mangle]
    static mut distance: UDWORD;
    #[no_mangle]
    static mut terrainOutline: UDWORD;
    #[no_mangle]
    static mut mouseTileX: SDWORD;
    #[no_mangle]
    static mut mouseTileY: SDWORD;
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    #[no_mangle]
    static mut geoOffset: UDWORD;
    #[no_mangle]
    fn raiseTile(tile3dX: UDWORD, tile3dY: UDWORD);
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    // Never stops.
    /* The time for the last frame */
    #[no_mangle]
    static mut frameTime2: UDWORD;
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    #[no_mangle]
    fn gameTimeResetMod();
    #[no_mangle]
    fn gameTimeGetMod(pMod: *mut FRACT);
    #[no_mangle]
    fn gameTimeSetMod(mod_0: FRACT);
    /* Terrain types that could obscure LOS */
    /*TER_STONE*/
    /* The distance under which visibility is automatic */
    // initialise the visibility stuff
    /* Check which tiles can be seen by an object */
    /* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
    /* Check whether psViewer can see psTarget.
 * struckBlock controls whether structures block LOS
 */
    // Do visibility check, but with walls completely blocking LOS.
    // Find the wall that is blocking LOS to a target (if any)
    // calculate the power at a given distance from a sensor/ecm
    // update the visibility reduction
    // sensorrangedisplay.
    #[no_mangle]
    fn startSensorDisplay();
    #[no_mangle]
    fn stopSensorDisplay();
    //TEST FUNCTION - MAKE EVERYTHING AVAILABLE
    #[no_mangle]
    fn makeAllAvailable();
    #[no_mangle]
    static mut fogStatus: UDWORD;
    //extern void	initLighting( void );
    #[no_mangle]
    fn initLighting(x1: UDWORD, y1: UDWORD, x2: UDWORD, y2: UDWORD);
    /*resets the power levels for all players when power is turned back on*/
    #[no_mangle]
    fn powerCalc(on: BOOL);
    /*Temp function to give all players some power when a new game has been loaded*/
    #[no_mangle]
    fn newGameInitPower();
    /*	Returns the next res. Ext. in the list from the one passed in. returns 1st one
	in list if passed in is NULL and NULL if there's none?
*/
    #[no_mangle]
    fn getRExtractor(psStruct: *mut STRUCTURE) -> *mut STRUCTURE;
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
    //flag used to check for power calculations to be done or not
    #[no_mangle]
    static mut powerCalculated: BOOL;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* Set the map view point to the world coordinates x,y */
    /* Set the map view point to the world coordinates x,y */
    #[no_mangle]
    fn intSetMapPos(x: UDWORD, y: UDWORD);
    /* Sync the interface to an object */
    #[no_mangle]
    fn intObjectSelected(psObj: *mut BASE_OBJECT);
    /* Reset the widget screen to just the reticule */
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    /* Add the options widgets to the widget screen */
    #[no_mangle]
    fn intAddOptions() -> BOOL;
    /*sets which list of structures to use for the interface*/
    #[no_mangle]
    fn interfaceStructList() -> *mut STRUCTURE;
    /*causes a reticule button to start flashing*/
    // stop a reticule button flashing
    //toggles the Power Bar display on and off
    #[no_mangle]
    fn togglePowerBar();
    //displays the Power Bar
    //hides the power bar from the display
//extern void intHidePowerBar(void);
    //hides the power bar from the display - regardless of what player requested!
    /* Add the Proximity message buttons */
    /*Remove a Proximity Button - when the message is deleted*/
    /* Allows us to fool the widgets with a keypress */
    /*Checks to see if there are any research topics to do and flashes the button*/
    // see if a reticule button is enabled
    #[no_mangle]
    fn intCheckReticuleButEnabled(id: UDWORD) -> BOOL;
    #[no_mangle]
    fn setKeyButtonMapping(val: UDWORD);
    //access function for selected object in the interface
    //initialise all the previous obj - particularly useful for when go Off world!
    #[no_mangle]
    fn intGetReopenBuild() -> BOOL;
    #[no_mangle]
    fn intReopenBuild(reopen: BOOL);
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    #[no_mangle]
    fn getWidgetsStatus() -> BOOL;
    // print out information about a droid and it's components
    #[no_mangle]
    fn printDroidInfo(psDroid: *mut DROID);
    /*
 * inGameOp.h
 * In game options screen.n
 * pumpkin Studios. 98.
 */
    // functions	
    #[no_mangle]
    fn intAddInGameOptions() -> BOOL;
    /* Maximum number of effects in the world - need to investigate what this should be */
/* EXTERNAL REFERENCES */
    #[no_mangle]
    fn effectGiveAuxVar(var: UDWORD);
    // naughty
    #[no_mangle]
    fn effectGiveAuxVarSec(var: UDWORD);
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    static mut droidScale: UDWORD;
    //#define RADAR_POSITION_AT_ZOOM
    /* Radar.h */
    #[no_mangle]
    fn resetRadarRedraw();
    #[no_mangle]
    fn SetRadarZoom(ZoomLevel: UWORD);
    //#define RADAR_ROT	1
    #[no_mangle]
    static mut bDrawRadarTerrain: BOOL;
    //radar terrain on/off
    #[no_mangle]
    static mut bEnemyAllyRadarColor: BOOL;
    /* screendump */
    #[no_mangle]
    fn screenDumpToDisk(path: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn attemptCheatCode(pName: *mut STRING) -> BOOL;
    #[no_mangle]
    fn toggleDemoStatus();
    #[no_mangle]
    fn demoGetStatus() -> BOOL;
    #[no_mangle]
    fn demoProcessTilesIn();
    #[no_mangle]
    fn demoProcessTilesOut();
    // ////////////////////////////////////////////////////////////////////////
// variables
    #[no_mangle]
    static mut NetPlay: NETPLAY;
    // leave the game in play.
    #[no_mangle]
    fn NETgetBytesSent() -> UDWORD;
    // return bytes sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetPacketsSent() -> UDWORD;
    // return packets sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetBytesRecvd() -> UDWORD;
    // return bytes sent/recv.  call regularly for good results
    #[no_mangle]
    fn NETgetPacketsRecvd() -> UDWORD;
    //capture
    #[no_mangle]
    fn NETstopAudioCapture() -> BOOL;
    #[no_mangle]
    fn NETstartAudioCapture() -> BOOL;
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn sendTextMessage(pStr: *mut libc::c_char, cast: BOOL) -> BOOL;
    #[no_mangle]
    static mut ingame: MULTIPLAYERINGAME;
    #[no_mangle]
    fn sendAIMessage(pStr: *mut libc::c_char, player_0: SDWORD, to: SDWORD)
     -> BOOL;
    #[no_mangle]
    static mut openchannels: [BOOL; 8];
    #[no_mangle]
    fn getPlayerName(player_0: UDWORD) -> *mut STRING;
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    #[no_mangle]
    fn addTemplateSet(from: UDWORD, to: UDWORD) -> BOOL;
    #[no_mangle]
    fn intAddMultiMenu() -> BOOL;
    #[no_mangle]
    fn atmosInitSystem();
    #[no_mangle]
    fn atmosSetWeatherType(type_0: WT_CLASS);
    #[no_mangle]
    fn atmosGetWeatherType() -> WT_CLASS;
    #[no_mangle]
    fn setRevealStatus(val: BOOL);
    #[no_mangle]
    fn getRevealStatus() -> BOOL;
    /*set validty keys for save game debugging*/
    #[no_mangle]
    fn game_SetValidityKey(keys: UDWORD);
    #[no_mangle]
    fn setDifficultyLevel(lev: DIFFICULTY_LEVEL);
    #[no_mangle]
    fn intRemoveOrder();
    #[no_mangle]
    fn iV_GetTextWidth(String: *mut STRING) -> libc::c_int;
    //fog available
    #[no_mangle]
    fn pie_EnableFog(val: BOOL);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn pie_SetFogColour(colour: UDWORD);
    #[no_mangle]
    fn getLastSubKey() -> KEY_CODE;
    #[no_mangle]
    fn processDebugMappings(val: BOOL);
    #[no_mangle]
    fn getDebugMappingStatus() -> BOOL;
    #[no_mangle]
    fn getMarkerX(code: KEY_CODE) -> UDWORD;
    #[no_mangle]
    fn getMarkerY(code: KEY_CODE) -> UDWORD;
    #[no_mangle]
    fn getMarkerSpin(code: KEY_CODE) -> SDWORD;
    //remove this one below
    #[no_mangle]
    fn keyShowMappings();
    #[no_mangle]
    static mut display3D: BOOL;
    #[no_mangle]
    static mut loopPieCount: SDWORD;
    #[no_mangle]
    static mut loopTileCount: SDWORD;
    #[no_mangle]
    static mut loopPolyCount: SDWORD;
    #[no_mangle]
    static mut loopStateChanges: SDWORD;
    #[no_mangle]
    fn gamePaused() -> BOOL;
    #[no_mangle]
    fn setGamePauseStatus(val: BOOL);
    #[no_mangle]
    fn setAudioPause(state: BOOL);
    #[no_mangle]
    fn setScriptPause(state: BOOL);
    #[no_mangle]
    fn setConsolePause(state: BOOL);
    // Get the value pointer for a variable index
    // Process all the currently active triggers
// Time is the application time at which all the triggers are to be processed
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    /*
 * EvntSave.h
 *
 * Save the state of the event system.
 *
 */
    // Save the state of the event system
    #[no_mangle]
    fn eventSaveState(version: SDWORD, ppBuffer: *mut *mut libc::c_char,
                      pFileSize: *mut UDWORD) -> BOOL;
    #[no_mangle]
    fn eventReset();
    // Load the state of the event system
    #[no_mangle]
    fn eventLoadState(pBuffer: *mut libc::c_char, fileSize: UDWORD,
                      bHashed: BOOL) -> BOOL;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    #[no_mangle]
    static mut mission: MISSION;
    //bCheating = TRUE == start of cheat, bCheating = FALSE == end of cheat
    #[no_mangle]
    fn setMissionCheatTime(bCheating: BOOL);
    // Display all the grid's an object is a member of
    #[no_mangle]
    fn gridDisplayCoverage(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    static mut DirectControl: BOOL;
    #[no_mangle]
    fn StartDriverMode(psOldDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn StopDriverMode();
    #[no_mangle]
    fn driveSelectionChanged();
    #[no_mangle]
    fn setDrivingStatus(val: BOOL);
    #[no_mangle]
    fn getDrivingStatus() -> BOOL;
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    // ---------------------------------------------------------------------
// EXTERNALLY REFERENCED FUNCTIONS
    #[no_mangle]
    fn selDroidSelection(player_0: UDWORD, droidClass: SELECTION_CLASS,
                         droidType: SELECTIONTYPE, bOnScreen: BOOL) -> UDWORD;
    #[no_mangle]
    fn selNextUnassignedUnit();
    #[no_mangle]
    fn selNextSpecifiedBuilding(structType: UDWORD);
    #[no_mangle]
    fn selNextSpecifiedUnit(unitType: UDWORD);
    // select the n'th command droid
    #[no_mangle]
    fn selCommander(n: SDWORD);
    /*
 * ScriptCB.h
 *
 * functions to deal with parameterised script callback triggers.
 *
 */
    //console callback stuff
//---------------------------
    #[no_mangle]
    static mut ConsolePlayer: SDWORD;
    #[no_mangle]
    static mut ConsoleMsg: [libc::c_char; 255];
    #[no_mangle]
    fn LoadPlayerAIExperience(nPlayer: SDWORD, bNotify: BOOL) -> BOOL;
    #[no_mangle]
    fn SavePlayerAIExperience(nPlayer: SDWORD, bNotify: BOOL) -> BOOL;
    #[no_mangle]
    fn SaveAIExperience(bNotify: BOOL) -> BOOL;
    #[no_mangle]
    fn LoadAIExperience(bNotify: BOOL) -> BOOL;
    #[no_mangle]
    fn OilExperienceDebug(nPlayer: SDWORD);
    #[no_mangle]
    fn BaseExperienceDebug(nPlayer: SDWORD);
    #[no_mangle]
    static mut bAllowDebugMode: BOOL;
    #[no_mangle]
    fn shakeStop();
    #[no_mangle]
    static mut ScreenDumpPath: [libc::c_char; 0];
}
pub type UBYTE = libc::c_uchar;
pub type SBYTE = libc::c_schar;
pub type STRING = libc::c_char;
pub type UWORD = libc::c_ushort;
pub type SWORD = libc::c_short;
pub type UDWORD = libc::c_uint;
pub type SDWORD = libc::c_int;
pub type BOOL = libc::c_int;
pub type CHAR = libc::c_char;
pub type DWORD = libc::c_int;
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
// The next free ID
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
/*
if any types are added BEFORE 'COMP_BODY' - then Save/Load Game will have to be 
altered since it loops through the components from 1->MAX_COMP
*/
//COMP_ARMOUR,
//COMP_POWERPLANT,
//COMP_PROGRAM,		//this needs to be removed when save games changes
//ALL components and structures and research topics have a tech level to which they belong
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
/* LOC used for holding locations for Sensors and ECM's*/
/* SIZE used for specifying body size */
//only using KINETIC and HEAT for now
//bullets etc
//WC_EXPLOSIVE,	//rockets etc - classed as WC_KINETIC now to save space in struct _droid AB 25/11/98 
//laser etc
//WC_MISC,		//others we haven't thought of! - classed as WC_HEAT now to save space in struct _droid AB 25/11/98 
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
pub type PROPULSION_TYPE = _propulsion_type;
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
// basic chance to run
//
// orderdef.h 
//
// order releated structures.
// data for barbarians retreating
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
pub type STRUCTURE = _structure;
/* The time the research facility was put on hold*/
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
// Feature armour
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
pub type MAPTILE = _maptile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
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
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
pub type TILE_COORD = _tile_coord;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
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
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_0 = 198;
pub const ID_SOUND_REPORTING: C2RustUnnamed_0 = 175;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_0 = 165;
/* Different types of triggers */
pub type TRIGGER_TYPE = _trigger_type;
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_MISSION_END: _scr_callback_types = 14;
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
pub const STR_GAM_NORTH: _fixed_str_id = 223;
pub const STR_MISC_PAUSED: _fixed_str_id = 13;
pub const STR_GAM_ENERGY: _fixed_str_id = 224;
pub type WT_CLASS = libc::c_uint;
pub const WT_NONE: WT_CLASS = 2;
pub const WT_SNOWING: WT_CLASS = 1;
pub const WT_RAINING: WT_CLASS = 0;
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
pub const CALL_CONSOLE: _scr_callback_types = 58;
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
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
pub const DSS_HALT_HOLD: _secondary_state = 64;
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
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub type WIDGET = _widget;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
/* The common widget data */
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
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
pub const STR_GAM_FORMATION_ON: _fixed_str_id = 436;
pub const STR_GAM_FORMATION_OFF: _fixed_str_id = 437;
pub const STR_GAM_RESNOTFOUND: _fixed_str_id = 244;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_0 = 6;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_0 = 3;
pub type DIFFICULTY_LEVEL = _difficulty_level;
pub type _difficulty_level = libc::c_uint;
pub const DL_KILLER: _difficulty_level = 4;
pub const DL_TOUGH: _difficulty_level = 3;
pub const DL_HARD: _difficulty_level = 2;
pub const DL_NORMAL: _difficulty_level = 1;
pub const DL_EASY: _difficulty_level = 0;
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
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
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_0 = 305;
pub const STR_GAM_BUILD_NO_REOPEN: _fixed_str_id = 435;
pub const STR_GAM_BUILD_REOPEN: _fixed_str_id = 434;
pub const STR_GAM_SPEED_UP: _fixed_str_id = 425;
pub const STR_GAM_NORMAL_SPEED: _fixed_str_id = 427;
pub const STR_GAM_SLOW_DOWN: _fixed_str_id = 426;
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
pub const ID_SOUND_GROUP_9: C2RustUnnamed_0 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_0 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_0 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_0 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_0 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_0 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_0 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_0 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_0 = 166;
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
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_0 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_0 = 4;
pub const ID_SOUND_SELECT: C2RustUnnamed_0 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_0 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_0 = 0;
pub const NO_SOUND: C2RustUnnamed_0 = -1;
pub type _scr_callback_types = libc::c_uint;
pub const CALL_BEACON: _scr_callback_types = 62;
pub const CALL_TRANSPORTER_LANDED_B: _scr_callback_types = 61;
pub const CALL_STRUCTBUILT: _scr_callback_types = 60;
pub const CALL_AI_MSG: _scr_callback_types = 59;
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
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
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
pub const STR_BIND_REOPEN_BUILD: _fixed_str_id = 433;
pub const STR_FE_SUBTITLES: _fixed_str_id = 432;
pub const STR_DORD_VTOL_FACTORY: _fixed_str_id = 431;
pub const STR_DORD_FIREDES: _fixed_str_id = 430;
pub const STR_BIND_RADJUMP: _fixed_str_id = 429;
pub const STR_BIND_RELOAD: _fixed_str_id = 428;
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
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
pub const STR_GAM_POWCON: _fixed_str_id = 226;
pub const STR_GAM_RESREM: _fixed_str_id = 225;
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
// Returns TRUE if drive mode is active.
//
#[inline]
unsafe extern "C" fn driveModeActive() -> BOOL { return DirectControl; }
#[no_mangle]
pub static mut bAllowOtherKeyPresses: BOOL = 1 as libc::c_int;
#[no_mangle]
pub static mut psOldRE: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
#[no_mangle]
pub static mut sTextToSend: [STRING; 255] = [0; 255];
#[no_mangle]
pub static mut fogCol: libc::c_int = 0 as libc::c_int;
/*
	KeyBind.c
	Holds all the functions that can be mapped to a key.
	All functions at the moment must be 'void func(void)'.
	Alex McLean, Pumpkin Studios, EIDOS Interactive.
*/
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleMissionTimer() {
    if mission.cheatTime != 0 {
        setMissionCheatTime(0 as libc::c_int);
    } else { setMissionCheatTime(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleRadarJump() {
    if getRadarJumpStatus() != 0 {
        setRadarJump(0 as libc::c_int);
    } else { setRadarJump(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_NoFaults() {
    audio_QueueTrack(ID_SOUND_NOFAULTS as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDimension() {
    display3D = 0 as libc::c_int;
    intSetMapPos((player.p.x as
                      libc::c_uint).wrapping_add(visibleXTiles.wrapping_div(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                     << 7 as libc::c_int),
                 (player.p.z as
                      libc::c_uint).wrapping_add(visibleYTiles.wrapping_div(2
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint)
                                                     << 7 as libc::c_int));
}
// --------------------------------------------------------------------------
//===================================================
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleSensorDisplay() {
    if rangeOnScreen != 0 { //added this message... Yeah, its lame. :)
        addConsoleMessage(b"Fine, sensor display is OFF!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY); //added this message... Yeah, its lame. :)
    } else {
        addConsoleMessage(b"Lets us see what you see!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    }
    rangeOnScreen = !rangeOnScreen;
    //toggle...  HMm  -Q 5-10-05
}
// --------------- All those keyboard mappable functions */
//===================================================
/* Halves all the heights of the map tiles */
#[no_mangle]
pub unsafe extern "C" fn kf_HalveHeights() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth {
        j = 0 as libc::c_int as UDWORD;
        while j < mapHeight {
            psTile = mapTile(i, j);
            (*psTile).height =
                ((*psTile).height as libc::c_int / 2 as libc::c_int) as UBYTE;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// dirty but necessary
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_FaceNorth() {
    player.r.y = 0 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_FaceSouth() {
    player.r.y =
        65536 as libc::c_int / 360 as libc::c_int * 180 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_FaceEast() {
    player.r.y =
        65536 as libc::c_int / 360 as libc::c_int * 90 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_FaceWest() {
    player.r.y =
        65536 as libc::c_int / 360 as libc::c_int * 270 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); };
}
// --------------------------------------------------------------------------
/* Writes out debug info about all the selected droids */
#[no_mangle]
pub unsafe extern "C" fn kf_DebugDroidInfo() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 { printDroidInfo(psDroid); }
        psDroid = (*psDroid).psNext
    };
}
// --------------------------------------------------------------------------
//
// /* Prints out the date and time of the build of the game */
#[no_mangle]
pub unsafe extern "C" fn kf_BuildInfo() {
    sprintf(ConsoleString.as_mut_ptr(),
            b"Built at %s on %s\x00" as *const u8 as *const libc::c_char,
            b"23:15:58\x00" as *const u8 as *const libc::c_char,
            b"May 29 2020\x00" as *const u8 as *const libc::c_char);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleConsoleDrop() {
    if bInTutorial == 0 { toggleConsoleDrop(); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetKillerLevel() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        setDifficultyLevel(DL_KILLER);
        addConsoleMessage(b"Hard as nails!!!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetEasyLevel() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        setDifficultyLevel(DL_EASY);
        addConsoleMessage(b"Takings thing easy!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_UpThePower() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        (*asPower[selectedPlayer as usize]).currentPower =
            ((*asPower[selectedPlayer as usize]).currentPower as
                 libc::c_uint).wrapping_add(1000 as libc::c_int as
                                                libc::c_uint) as UDWORD as
                UDWORD;
        addConsoleMessage(b"1000 big ones!!!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_MaxPower() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        (*asPower[selectedPlayer as usize]).currentPower =
            0x7fffffff as libc::c_int as UDWORD;
        addConsoleMessage(b"Max Power!!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetNormalLevel() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        setDifficultyLevel(DL_NORMAL);
        addConsoleMessage(b"Back to normality!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetHardLevel() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        setDifficultyLevel(DL_HARD);
        addConsoleMessage(b"Getting tricky!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetToughUnitsLevel() {
    if bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int {
        setDifficultyLevel(DL_TOUGH);
        addConsoleMessage(b"Twice as nice!\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
/* Writes out the frame rate */
#[no_mangle]
pub unsafe extern "C" fn kf_FrameRate() {
    sprintf(ConsoleString.as_mut_ptr(),
            b"FPS %d; PIEs %d; polys %d; Terr. polys %d; States %d\x00" as
                *const u8 as *const libc::c_char, frameGetFrameRate(),
            loopPieCount, loopPolyCount, loopTileCount, loopStateChanges);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    if bMultiPlayer != 0 {
        sprintf(ConsoleString.as_mut_ptr(),
                b"NETWORK:  Bytes: s-%d r-%d  Packets: s-%d r-%d\x00" as
                    *const u8 as *const libc::c_char, NETgetBytesSent(),
                NETgetBytesRecvd(), NETgetPacketsSent(),
                NETgetPacketsRecvd());
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    }
    gameStats = (gameStats == 0) as libc::c_int;
    sprintf(ConsoleString.as_mut_ptr(),
            b"Built at %s on %s\x00" as *const u8 as *const libc::c_char,
            b"23:15:58\x00" as *const u8 as *const libc::c_char,
            b"May 29 2020\x00" as *const u8 as *const libc::c_char);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    //		addConsoleMessage("Game statistics display toggled",DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
// display the total number of objects in the world
#[no_mangle]
pub unsafe extern "C" fn kf_ShowNumObjects() {
    let mut i: SDWORD = 0;
    let mut droids: SDWORD = 0;
    let mut structures: SDWORD = 0;
    let mut features: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    droids = 0 as libc::c_int;
    structures = 0 as libc::c_int;
    features = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() {
            droids += 1 as libc::c_int;
            psDroid = (*psDroid).psNext
        }
        psStruct = apsStructLists[i as usize];
        while !psStruct.is_null() {
            structures += 1 as libc::c_int;
            psStruct = (*psStruct).psNext
        }
        i += 1
    }
    psFeat = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeat.is_null() {
        features += 1 as libc::c_int;
        psFeat = (*psFeat).psNext
    }
    sprintf(ConsoleString.as_mut_ptr(),
            b"Num Droids: %d  Num Structures: %d  Num Features: %d\x00" as
                *const u8 as *const libc::c_char, droids, structures,
            features);
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
/* Toggles radar on off */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleRadar() {
    radarOnScreen = (radarOnScreen == 0) as libc::c_int;
    //		addConsoleMessage("Radar display toggled",DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
/* Toggles the outline around the map tiles */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleOutline() {
    if terrainOutline != 0 {
        terrainOutline = 0 as libc::c_int as UDWORD
    } else { terrainOutline = 1 as libc::c_int as UDWORD };
    //		addConsoleMessage("Tile outline display toggled",DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
/* Toggles infinite power on/off */
#[no_mangle]
pub unsafe extern "C" fn kf_TogglePower() {
    if bMultiPlayer != 0 { return }
    powerCalculated = (powerCalculated == 0) as libc::c_int;
    if powerCalculated != 0 {
        addConsoleMessage(b"Infinite power disabled\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
        powerCalc(1 as libc::c_int);
    } else {
        addConsoleMessage(b"Infinite power enabled\x00" as *const u8 as
                              *const libc::c_char as *mut STRING,
                          DEFAULT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
/* Recalculates the lighting values for a tile */
#[no_mangle]
pub unsafe extern "C" fn kf_RecalcLighting() {
    //initLighting();
    initLighting(0 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD,
                 mapWidth, mapHeight);
    addConsoleMessage(b"Lighting values for all tiles recalculated\x00" as
                          *const u8 as *const libc::c_char as *mut STRING,
                      DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
/* Raises the 3dfx gamma value */
#[no_mangle]
pub unsafe extern "C" fn kf_RaiseGamma() { }
// --------------------------------------------------------------------------
/* Lowers the threedfx gamma value */
#[no_mangle]
pub unsafe extern "C" fn kf_LowerGamma() { }
// --------------------------------------------------------------------------
/* Sends the 3dfx screen buffer to disk */
#[no_mangle]
pub unsafe extern "C" fn kf_ScreenDump() {
    //CONPRINTF(ConsoleString,(ConsoleString,"Screen dump written to working directory : %s", screenDumpToDisk()));
    screenDumpToDisk(ScreenDumpPath.as_mut_ptr());
}
// --------------------------------------------------------------------------
/* Make all functions available */
#[no_mangle]
pub unsafe extern "C" fn kf_AllAvailable() {
    if bMultiPlayer != 0 && NetPlay.bComms != 0 as libc::c_int { return }
    //		addConsoleMessage("All items made available",DEFAULT_JUSTIFY);
    makeAllAvailable();
}
// --------------------------------------------------------------------------
/* Flips the cut of a tile */
#[no_mangle]
pub unsafe extern "C" fn kf_TriFlip() {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    //MAPTILE	*psTile;
//		psTile = mapTile(mouseTileX,mouseTileY);
//		TOGGLE_TRIFLIP(psTile);
//		addConsoleMessage("Triangle flip status toggled",DEFAULT_JUSTIFY);
    pos.x =
        mouseTileX * 128 as libc::c_int +
            128 as libc::c_int / 2 as libc::c_int;
    pos.z =
        mouseTileY * 128 as libc::c_int +
            128 as libc::c_int / 2 as libc::c_int;
    pos.y = map_Height(pos.x as UDWORD, pos.x as UDWORD) as int32;
    effectGiveAuxVar(50 as libc::c_int as UDWORD);
    effectGiveAuxVarSec(10000 as libc::c_int as UDWORD);
    addEffect(&mut pos, EFFECT_FIRE, FIRE_TYPE_LOCALISED, 0 as libc::c_int,
              0 as *mut iIMDShape, 0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleBackgroundFog() {
    static mut bEnabled: BOOL = 1 as libc::c_int; //start in nicks mode
    if bEnabled != 0 {
        //true, so go to false
        bEnabled = 0 as libc::c_int; //clear lowest bit of 3
        fogStatus &= (7 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            pie_EnableFog(0 as libc::c_int);
        }
    } else {
        bEnabled = 1 as libc::c_int;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_EnableFog(1 as libc::c_int);
        }
        fogStatus |= 1 as libc::c_int as libc::c_uint
        //set lowest bit of 3
    }; //start in nicks mode
}
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDistanceFog() {
    static mut bEnabled: BOOL = 1 as libc::c_int;
    if bEnabled != 0 {
        //true, so go to false
        bEnabled = 0 as libc::c_int; //clear middle bit of 3
        fogStatus &= (7 as libc::c_int - 2 as libc::c_int) as libc::c_uint;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            pie_EnableFog(0 as libc::c_int);
        }
    } else {
        bEnabled = 1 as libc::c_int;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_EnableFog(1 as libc::c_int);
        }
        fogStatus |= 2 as libc::c_int as libc::c_uint
        //set lowest bit of 3
    }; //start in nicks mode
}
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleMistFog() {
    static mut bEnabled: BOOL = 1 as libc::c_int;
    if bEnabled != 0 {
        //true, so go to false
        bEnabled = 0 as libc::c_int; //clear highest bit of 3
        fogStatus &= (7 as libc::c_int - 4 as libc::c_int) as libc::c_uint;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            pie_EnableFog(0 as libc::c_int);
        }
    } else {
        bEnabled = 1 as libc::c_int;
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_EnableFog(1 as libc::c_int);
        }
        fogStatus |= 4 as libc::c_int as libc::c_uint
        //set highest bit of 3
    }; //nicks colour Urban
}
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleFogColour() {
    fogCol += 1; //nicks colour Rockies 182,225,236
    if fogCol > 4 as libc::c_int { fogCol = 0 as libc::c_int } //haze
    match fogCol {
        1 => {
            pie_SetFogColour(0xc9920f as libc::c_int as UDWORD); //black
        }
        2 => {
            pie_SetFogColour(0xb6e1ec as libc::c_int as
                                 UDWORD); //nicks colour Arizona
        }
        3 => { pie_SetFogColour(0x101040 as libc::c_int as UDWORD); }
        4 => { pie_SetFogColour(0 as libc::c_int as UDWORD); }
        0 | _ => { pie_SetFogColour(0xb08f5f as libc::c_int as UDWORD); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleFog() {
    static mut fogEnabled: BOOL = 0 as libc::c_int;
    if fogEnabled != 0 {
        fogEnabled = 0 as libc::c_int;
        pie_SetFogStatus(0 as libc::c_int);
        pie_EnableFog(fogEnabled);
        //			addConsoleMessage("Fog Off",DEFAULT_JUSTIFY);
    } else {
        fogEnabled = 1 as libc::c_int;
        pie_EnableFog(fogEnabled);
        //			addConsoleMessage("Fog On",DEFAULT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
/* Toggles fog on/off */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleWidgets() {
    //	 	widgetsOn = !widgetsOn;
    if getWidgetsStatus() != 0 {
        setWidgetsStatus(0 as libc::c_int);
    } else { setWidgetsStatus(1 as libc::c_int); };
    //	addConsoleMessage("Widgets display toggled",DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
/* Toggle camera on/off */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleCamera() {
    if getWarCamStatus() == 0 as libc::c_int {
        shakeStop(); // Ensure screen shake stopped before starting camera mode.
        setDrivingStatus(0 as libc::c_int);
    }
    camToggleStatus();
}
// --------------------------------------------------------------------------
/* Simulates a close down */
/*
void	kf_SimCloseDown( void )
{
#ifndef PSX
  		bScreenClose = TRUE;
		audio_PlayTrack( ID_SOUND_THX_SHUTDOWN );

		closingTimeStart = gameTime;
//		widgetsOn = FALSE;
		spinScene = TRUE;
		radarOnScreen = FALSE;
		screenCloseState = SC_CLOSING_DOWN;
#endif
}
*/
// --------------------------------------------------------------------------
/* Raises the tile under the mouse */
#[no_mangle]
pub unsafe extern "C" fn kf_RaiseTile() {
    raiseTile(mouseTileX as UDWORD, mouseTileY as UDWORD);
}
// --------------------------------------------------------------------------
/* Lowers the tile under the mouse */
#[no_mangle]
pub unsafe extern "C" fn kf_LowerTile() {
    //	lowerTile(mouseTileX,mouseTileY);
    selNextSpecifiedBuilding(REF_FACTORY as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
/* Quick game exit */
#[no_mangle]
pub unsafe extern "C" fn kf_SystemClose() { }
// --------------------------------------------------------------------------
/* Zooms out from display */
#[no_mangle]
pub unsafe extern "C" fn kf_ZoomOut() {
    let mut fraction: FRACT = 0.;
    let mut zoomInterval: FRACT = 0.;
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    zoomInterval = fraction * 1000 as libc::c_int as libc::c_float;
    distance =
        (distance as
             libc::c_uint).wrapping_add(zoomInterval as SDWORD as
                                            libc::c_uint) as UDWORD as UDWORD;
    /* If we're debugging, limit to a bit more */
    if distance > 4500 as libc::c_int as libc::c_uint {
        distance = 4500 as libc::c_int as UDWORD
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_RadarZoomIn() {
    if (RadarZoomLevel as libc::c_int) < 2 as libc::c_int {
        RadarZoomLevel = RadarZoomLevel.wrapping_add(1);
        SetRadarZoom(RadarZoomLevel);
        audio_PlayTrack(ID_SOUND_BUTTON_CLICK_5 as libc::c_int);
    } else {
        // at maximum already
        audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_RadarZoomOut() {
    if RadarZoomLevel != 0 {
        RadarZoomLevel = RadarZoomLevel.wrapping_sub(1);
        SetRadarZoom(RadarZoomLevel);
        audio_PlayTrack(ID_SOUND_BUTTON_CLICK_5 as libc::c_int);
    } else {
        // at minimum already
        audio_PlayTrack(ID_SOUND_BUILD_FAIL as libc::c_int);
    };
}
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
/* Zooms in the map */
#[no_mangle]
pub unsafe extern "C" fn kf_ZoomIn() {
    let mut fraction: FRACT = 0.;
    let mut zoomInterval: FRACT = 0.;
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    zoomInterval = fraction * 1000 as libc::c_int as libc::c_float;
    distance =
        (distance as
             libc::c_uint).wrapping_sub(zoomInterval as SDWORD as
                                            libc::c_uint) as UDWORD as UDWORD;
    if distance < 200 as libc::c_int as libc::c_uint {
        distance = 200 as libc::c_int as UDWORD
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_MaxScrollLimits() {
    scrollMinY = 0 as libc::c_int;
    scrollMinX = scrollMinY;
    scrollMaxX = mapWidth as SDWORD;
    scrollMaxY = mapHeight as SDWORD;
}
// --------------------------------------------------------------------------
// Shrink the screen down
/*
void	kf_ShrinkScreen( void )
{
	// nearest multiple of 8 plus 1
	if (xOffset<73)
	{
		xOffset+=8;
  		distance+=170;
		if (yOffset<200)
		{
			yOffset+=8;
		}
	}
}
*/
// --------------------------------------------------------------------------
// Expand the screen
/*
void	kf_ExpandScreen( void )
{
	if(xOffset)
	{
   		if (distance>MAXDISTANCE)
   		{
   			distance-=170;
   		}
   		xOffset-=8;
   		if(yOffset)
   		{
   			yOffset-=8;
   		}
	}
}
*/
// --------------------------------------------------------------------------
/* Spins the world round left */
#[no_mangle]
pub unsafe extern "C" fn kf_RotateLeft() {
    let mut fraction: FRACT = 0.;
    let mut rotAmount: FRACT = 0.;
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    rotAmount =
        fraction *
            (360 as libc::c_int * (65536 as libc::c_int / 360 as libc::c_int)
                 / 2 as libc::c_int) as libc::c_float;
    player.r.y += rotAmount as SDWORD;
}
// --------------------------------------------------------------------------
/* Spins the world right */
#[no_mangle]
pub unsafe extern "C" fn kf_RotateRight() {
    let mut fraction: FRACT = 0.;
    let mut rotAmount: FRACT = 0.;
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    rotAmount =
        fraction *
            (360 as libc::c_int * (65536 as libc::c_int / 360 as libc::c_int)
                 / 2 as libc::c_int) as libc::c_float;
    player.r.y -= rotAmount as SDWORD;
    if player.r.y < 0 as libc::c_int {
        player.r.y +=
            65536 as libc::c_int / 360 as libc::c_int * 360 as libc::c_int
    };
}
// --------------------------------------------------------------------------
/* Pitches camera back */
#[no_mangle]
pub unsafe extern "C" fn kf_PitchBack() {
    let mut fraction: FRACT = 0.;
    let mut pitchAmount: FRACT = 0.;
    //#ifdef ALEXM
//SDWORD	pitch;
//SDWORD	angConcern;
//#endif
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    pitchAmount =
        fraction *
            (360 as libc::c_int * (65536 as libc::c_int / 360 as libc::c_int)
                 / 2 as libc::c_int) as libc::c_float;
    //#ifdef ALEXM
//	pitch = getSuggestedPitch();
//	angConcern = DEG(360-pitch);
//
//	if(player.r.x < angConcern)
//	{
//#endif
    player.r.x += pitchAmount as SDWORD;
    //#ifdef ALEXM
//	}
//#endif
//#ifdef ALEXM
//	if(getDebugMappingStatus() == FALSE)
//#endif
    //	{
    if player.r.x >
           65536 as libc::c_int / 360 as libc::c_int *
               (360 as libc::c_int + -(14 as libc::c_int)) {
        player.r.x =
            65536 as libc::c_int / 360 as libc::c_int *
                (360 as libc::c_int + -(14 as libc::c_int))
    }
    //	}
    setDesiredPitch(player.r.x / (65536 as libc::c_int / 360 as libc::c_int));
}
// --------------------------------------------------------------------------
/* Pitches camera foward */
#[no_mangle]
pub unsafe extern "C" fn kf_PitchForward() {
    let mut fraction: FRACT = 0.;
    let mut pitchAmount: FRACT = 0.;
    fraction = frameTime2 as FRACT / 1000 as libc::c_int as libc::c_float;
    pitchAmount =
        fraction *
            (360 as libc::c_int * (65536 as libc::c_int / 360 as libc::c_int)
                 / 2 as libc::c_int) as libc::c_float;
    player.r.x -= pitchAmount as SDWORD;
    //#ifdef ALEXM
//	if(getDebugMappingStatus() == FALSE)
//#endif
//	{
    if player.r.x <
           65536 as libc::c_int / 360 as libc::c_int *
               (360 as libc::c_int + -(50 as libc::c_int)) {
        player.r.x =
            65536 as libc::c_int / 360 as libc::c_int *
                (360 as libc::c_int + -(50 as libc::c_int))
    }
    //	}
    setDesiredPitch(player.r.x / (65536 as libc::c_int / 360 as libc::c_int));
}
// --------------------------------------------------------------------------
/* Resets pitch to default */
#[no_mangle]
pub unsafe extern "C" fn kf_ResetPitch() {
    player.r.x =
        65536 as libc::c_int / 360 as libc::c_int *
            (360 as libc::c_int - 20 as libc::c_int);
    distance = 2500 as libc::c_int as UDWORD;
}
// --------------------------------------------------------------------------
/* Dumps all the keyboard mappings to the console display */
#[no_mangle]
pub unsafe extern "C" fn kf_ShowMappings() { keyShowMappings(); }
// --------------------------------------------------------------------------
/*If this is performed twice then it changes the productionPlayer*/
#[no_mangle]
pub unsafe extern "C" fn kf_SelectPlayer() {
    let mut playerNumber: UDWORD = 0;
    let mut prevPlayer: UDWORD = 0;
    if bMultiPlayer != 0 && NetPlay.bComms != 0 as libc::c_int { return }
    //store the current player
    prevPlayer = selectedPlayer;
    playerNumber =
        (getLastSubKey() as
             libc::c_uint).wrapping_sub(KEY_F1 as libc::c_int as
                                            libc::c_uint);
    if playerNumber >= 10 as libc::c_int as libc::c_uint {
        selectedPlayer = 0 as libc::c_int as UDWORD
    } else { selectedPlayer = playerNumber }
    //	godMode = TRUE;
    if prevPlayer == selectedPlayer {
        changeProductionPlayer(selectedPlayer as UBYTE);
    };
}
// --------------------------------------------------------------------------
/* Selects the player's groups 1..9 */
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping(mut groupNumber: UDWORD) {
    let mut bAlreadySelected: BOOL = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut Selected: BOOL = 0;
    bAlreadySelected = 0 as libc::c_int;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        /* Wipe out the ones in the wrong group */
        if (*psDroid).selected as libc::c_int != 0 &&
               (*psDroid).group as libc::c_uint != groupNumber {
            (*psDroid).selected = 0 as libc::c_int as UBYTE
        }
        /* Get the right ones */
        if (*psDroid).group as libc::c_uint == groupNumber {
            if (*psDroid).selected != 0 {
                bAlreadySelected = 1 as libc::c_int
            }
        }
        psDroid = (*psDroid).psNext
    }
    if bAlreadySelected != 0 {
        Selected = activateGroupAndMove(selectedPlayer, groupNumber)
    } else { Selected = activateGroup(selectedPlayer, groupNumber) }
    // Tell the driving system that the selection may have changed.
    driveSelectionChanged();
    /* play group audio but only if they wern't already selected - AM */
    if Selected != 0 && bAlreadySelected == 0 {
        audio_QueueTrack((ID_SOUND_GROUP_0 as libc::c_int as
                              libc::c_uint).wrapping_add(groupNumber) as
                             SDWORD);
        audio_QueueTrack(ID_SOUND_REPORTING as libc::c_int);
        audio_QueueTrack(ID_SOUND_RADIOCLICK_1 as libc::c_int +
                             rand() % 6 as libc::c_int);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_1() {
    assignDroidsToGroup(selectedPlayer, 1 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_1() {
    selCommander(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_1() {
    kf_SelectGrouping(1 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_2() {
    kf_SelectGrouping(2 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_2() {
    selCommander(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_2() {
    assignDroidsToGroup(selectedPlayer, 2 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_3() {
    selCommander(3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_3() {
    kf_SelectGrouping(3 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_3() {
    assignDroidsToGroup(selectedPlayer, 3 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_4() {
    selCommander(4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_4() {
    assignDroidsToGroup(selectedPlayer, 4 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_4() {
    kf_SelectGrouping(4 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_5() {
    selCommander(5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_5() {
    kf_SelectGrouping(5 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_5() {
    assignDroidsToGroup(selectedPlayer, 5 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_6() {
    kf_SelectGrouping(6 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_6() {
    selCommander(6 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_6() {
    assignDroidsToGroup(selectedPlayer, 6 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_7() {
    kf_SelectGrouping(7 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_7() {
    selCommander(7 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_7() {
    assignDroidsToGroup(selectedPlayer, 7 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_8() {
    kf_SelectGrouping(8 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_8() {
    assignDroidsToGroup(selectedPlayer, 8 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_8() {
    selCommander(8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_AssignGrouping_9() {
    assignDroidsToGroup(selectedPlayer, 9 as libc::c_int as UDWORD);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectCommander_9() {
    selCommander(9 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kf_SelectGrouping_9() {
    kf_SelectGrouping(9 as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectMoveGrouping() {
    let mut groupNumber: UDWORD = 0;
    groupNumber =
        (getLastSubKey() as
             libc::c_uint).wrapping_sub(KEY_1 as libc::c_int as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint);
    activateGroupAndMove(selectedPlayer, groupNumber);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDroidInfo() { camToggleInfo(); }
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_addInGameOptions() {
    setWidgetsStatus(1 as libc::c_int);
    intAddInGameOptions();
}
// --------------------------------------------------------------------------
/* Tell the scripts to start a mission*/
#[no_mangle]
pub unsafe extern "C" fn kf_AddMissionOffWorld() {
    if bMultiPlayer != 0 { return }
    game_SetValidityKey(0x4 as libc::c_int as UDWORD);
    eventFireCallbackTrigger(CALL_MISSION_START as libc::c_int as
                                 TRIGGER_TYPE);
}
// --------------------------------------------------------------------------
/* Tell the scripts to end a mission*/
#[no_mangle]
pub unsafe extern "C" fn kf_EndMissionOffWorld() {
    if bMultiPlayer != 0 { return }
    eventFireCallbackTrigger(CALL_MISSION_END as libc::c_int as TRIGGER_TYPE);
}
// --------------------------------------------------------------------------
/* Initialise the player power levels*/
#[no_mangle]
pub unsafe extern "C" fn kf_NewPlayerPower() {
    if bMultiPlayer != 0 { return }
    newGameInitPower();
}
// --------------------------------------------------------------------------
// Display multiplayer guff.
#[no_mangle]
pub unsafe extern "C" fn kf_addMultiMenu() {
    if bMultiPlayer != 0 { intAddMultiMenu(); };
}
// --------------------------------------------------------------------------
// start/stop capturing audio for multiplayer
#[no_mangle]
pub unsafe extern "C" fn kf_multiAudioStart() {
    if bMultiPlayer != 0 &&
           game.bytesPerSec as libc::c_int == 2000 as libc::c_int &&
           NetPlay.bCaptureInUse == 0 {
        // noone else talking.
        NETstartAudioCapture();
    };
}
#[no_mangle]
pub unsafe extern "C" fn kf_multiAudioStop() {
    if bMultiPlayer != 0 &&
           game.bytesPerSec as libc::c_int == 2000 as libc::c_int {
        NETstopAudioCapture();
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToMapMarker() {
    let mut entry: UDWORD = 0;
    if getRadarTrackingStatus() == 0 {
        entry = getLastSubKey() as UDWORD;
        //		CONPRINTF(ConsoleString,(ConsoleString,"Restoring map position %d:%d",getMarkerX(entry),getMarkerY(entry)));
        player.p.x = getMarkerX(entry as KEY_CODE) as int32;
        player.p.z = getMarkerY(entry as KEY_CODE) as int32;
        player.r.y = getMarkerSpin(entry as KEY_CODE);
        /* A fix to stop the camera continuing when marker code is called */
        if getWarCamStatus() != 0 { camToggleStatus(); }
    };
}
// --------------------------------------------------------------------------
/* Raises the G Offset */
#[no_mangle]
pub unsafe extern "C" fn kf_UpGeoOffset() {
    geoOffset = geoOffset.wrapping_add(1);
}
// --------------------------------------------------------------------------
/* Lowers the geoOffset */
#[no_mangle]
pub unsafe extern "C" fn kf_DownGeoOffset() {
    geoOffset = geoOffset.wrapping_sub(1);
}
// --------------------------------------------------------------------------
/* Ups the droid scale */
#[no_mangle]
pub unsafe extern "C" fn kf_UpDroidScale() {
    droidScale = droidScale.wrapping_add(1);
}
// --------------------------------------------------------------------------
/* Lowers the droid scale */
#[no_mangle]
pub unsafe extern "C" fn kf_DownDroidScale() {
    if droidScale > 2 as libc::c_int as libc::c_uint {
        droidScale = droidScale.wrapping_sub(1)
    };
}
// --------------------------------------------------------------------------
/* Toggles the power bar display on and off*/
#[no_mangle]
pub unsafe extern "C" fn kf_TogglePowerBar() { togglePowerBar(); }
// --------------------------------------------------------------------------
/* Toggles whether we process debug key mappings */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDebugMappings() {
    // Prevent cheating in multiplayer when not compiled in debug mode
    if bMultiPlayer != 0 && NetPlay.bComms != 0 as libc::c_int { return }
    if bAllowDebugMode != 0 {
        if getDebugMappingStatus() != 0 {
            processDebugMappings(0 as libc::c_int);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"CHEATS DISABLED!\x00" as *const u8 as
                        *const libc::c_char);
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        } else {
            game_SetValidityKey(0x8 as libc::c_int as UDWORD);
            processDebugMappings(1 as libc::c_int);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"CHEATS ENABLED!\x00" as *const u8 as
                        *const libc::c_char);
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        if bMultiPlayer != 0 {
            sendTextMessage(b"Presses Debug. CHEAT\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            1 as libc::c_int);
        }
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleGodMode() {
    if bMultiPlayer != 0 && NetPlay.bComms != 0 as libc::c_int { return }
    if godMode != 0 {
        godMode = 0 as libc::c_int;
        //		setDifficultyLevel(getDifficultyLevel());
        sprintf(ConsoleString.as_mut_ptr(),
                b"God Mode OFF\x00" as *const u8 as *const libc::c_char);
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        demoProcessTilesOut();
    } else {
        godMode = 1 as libc::c_int;
        //		setModifiers(FRACTCONST(1000,100),FRACTCONST(100,1000));
        sprintf(ConsoleString.as_mut_ptr(),
                b"God Mode ON\x00" as *const u8 as *const libc::c_char);
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        demoProcessTilesIn();
    };
}
// --------------------------------------------------------------------------
/* Aligns the view to north - some people can't handle the world spinning */
#[no_mangle]
pub unsafe extern "C" fn kf_SeekNorth() {
    player.r.y = 0 as libc::c_int;
    if getWarCamStatus() != 0 { camToggleStatus(); }
    sprintf(ConsoleString.as_mut_ptr(),
            strresGetString(psStringRes,
                            STR_GAM_NORTH as libc::c_int as UDWORD));
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_TogglePauseMode() {
    if bMultiPlayer != 0 && NetPlay.bComms != 0 as libc::c_int { return }
    /* Is the game running? */
    if gamePaused() == 0 as libc::c_int {
        /* Then pause it */
        setGamePauseStatus(1 as libc::c_int);
        setConsolePause(1 as libc::c_int);
        setScriptPause(1 as libc::c_int);
        setAudioPause(1 as libc::c_int);
        /* And stop the clock */
        gameTimeStop();
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_MISC_PAUSED as libc::c_int as
                                              UDWORD), CENTRE_JUSTIFY);
    } else {
        /* Else get it going again */
        setGamePauseStatus(0 as libc::c_int);
        setConsolePause(0 as libc::c_int);
        setScriptPause(0 as libc::c_int);
        setAudioPause(0 as libc::c_int);
        /* And start the clock again */
        gameTimeStart();
    };
}
// --------------------------------------------------------------------------
// finish all the research for the selected player
#[no_mangle]
pub unsafe extern "C" fn kf_FinishResearch() {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //for (psCurr=apsStructLists[selectedPlayer]; psCurr; psCurr = psCurr->psNext)
    psCurr = interfaceStructList();
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
            //((RESEARCH_FACILITY *)psCurr->pFunctionality)->timeStarted = 0;
            (*((*psCurr).pFunctionality as
                   *mut RESEARCH_FACILITY)).timeStarted =
                gameTime.wrapping_add(100000 as libc::c_int as libc::c_uint);
            //set power accrued to high value so that will trigger straight away
            (*((*psCurr).pFunctionality as
                   *mut RESEARCH_FACILITY)).powerAccrued =
                10000 as libc::c_int as UDWORD
        }
        psCurr = (*psCurr).psNext
    };
}
// --------------------------------------------------------------------------
//void	kf_ToggleRadarAllign( void )
//{
//	toggleRadarAllignment();
//	addConsoleMessage("Radar allignment toggled",LEFT_JUSTIFY);
//}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleEnergyBars() {
    toggleEnergyBars();
    sprintf(ConsoleString.as_mut_ptr(),
            strresGetString(psStringRes,
                            STR_GAM_ENERGY as libc::c_int as UDWORD));
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleReloadBars() {
    toggleReloadBarDisplay();
    sprintf(ConsoleString.as_mut_ptr(),
            strresGetString(psStringRes,
                            STR_GAM_ENERGY as libc::c_int as UDWORD));
    addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDemoMode() {
    if demoGetStatus() == 0 as libc::c_int {
        /* Switch on demo mode */
        toggleDemoStatus();
        enableConsoleDisplay(1 as libc::c_int);
    } else {
        toggleDemoStatus();
        flushConsoleMessages();
        setConsolePermanence(0 as libc::c_int, 1 as libc::c_int);
        permitNewConsoleMessages(1 as libc::c_int);
        addConsoleMessage(b"Demo Mode OFF - Returning to normal game mode\x00"
                              as *const u8 as *const libc::c_char as
                              *mut STRING, LEFT_JUSTIFY);
        if getWarCamStatus() != 0 { camToggleStatus(); }
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseOptions() {
    //	if(!widgetsOn) widgetsOn = TRUE;
    intResetScreen(1 as libc::c_int);
    setWidgetsStatus(1 as libc::c_int);
    intAddOptions();
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleBlips() {
    if doWeDrawRadarBlips() != 0 {
        setBlipDraw(0 as libc::c_int);
    } else { setBlipDraw(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleProximitys() {
    if doWeDrawProximitys() != 0 {
        setProximityDraw(0 as libc::c_int);
    } else { setProximityDraw(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToResourceExtractor() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE; // face north
    let mut xJump: SDWORD = 0;
    let mut yJump: SDWORD = 0;
    psStruct = getRExtractor(psOldRE);
    if !psStruct.is_null() {
        xJump =
            ((*psStruct).x as
                 libc::c_uint).wrapping_sub(visibleXTiles.wrapping_div(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint).wrapping_mul(128
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint))
                as SDWORD;
        yJump =
            ((*psStruct).y as
                 libc::c_uint).wrapping_sub(visibleYTiles.wrapping_div(2 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_uint).wrapping_mul(128
                                                                                                          as
                                                                                                          libc::c_int
                                                                                                          as
                                                                                                          libc::c_uint))
                as SDWORD;
        player.p.x = xJump;
        player.p.z = yJump;
        player.r.y = 0 as libc::c_int;
        setViewPos(((*psStruct).x as libc::c_int >> 7 as libc::c_int) as
                       UDWORD,
                   ((*psStruct).y as libc::c_int >> 7 as libc::c_int) as
                       UDWORD, 1 as libc::c_int);
        psOldRE = psStruct
    } else {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_RESNOTFOUND as libc::c_int
                                              as UDWORD), LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToRepairUnits() {
    selNextSpecifiedUnit(DROID_REPAIR as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToConstructorUnits() {
    selNextSpecifiedUnit(DROID_CONSTRUCT as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToSensorUnits() {
    selNextSpecifiedUnit(DROID_SENSOR as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToCommandUnits() {
    selNextSpecifiedUnit(DROID_COMMAND as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_JumpToUnassignedUnits() {
    selNextUnassignedUnit();
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleOverlays() {
    /* Make sure they're both the same */
//		radarOnScreen = widgetsOn;
		/* Flip their states */
//		radarOnScreen = !radarOnScreen;
    if getWidgetsStatus() != 0 {
        setWidgetsStatus(0 as libc::c_int);
    } else { setWidgetsStatus(1 as libc::c_int); };
}
//Was commented out.  Re-enabled --Q 5/10/05
#[no_mangle]
pub unsafe extern "C" fn kf_SensorDisplayOn() {
    //	debugToggleSensorDisplay();
    startSensorDisplay();
}
#[no_mangle]
pub unsafe extern "C" fn kf_SensorDisplayOff() { stopSensorDisplay(); }
// --------------------------------------------------------------------------
/*
#define IDRET_OPTIONS		2		// option button
#define IDRET_BUILD			3		// build button
#define IDRET_MANUFACTURE	4		// manufacture button
#define IDRET_RESEARCH		5		// research button
#define IDRET_INTEL_MAP		6		// intelligence map button
#define IDRET_DESIGN		7		// design droids button
#define IDRET_CANCEL		8		// central cancel button
*/
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseCommand() {
    if intCheckReticuleButEnabled(9 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(9 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;

	psWidg = widgGetFromID(psWScreen,IDRET_COMMAND);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseManufacture() {
    if intCheckReticuleButEnabled(4 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(4 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;

	psWidg = widgGetFromID(psWScreen,IDRET_MANUFACTURE);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseResearch() {
    if intCheckReticuleButEnabled(5 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(5 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;
	psWidg = widgGetFromID(psWScreen,IDRET_RESEARCH);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	 */
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseBuild() {
    if intCheckReticuleButEnabled(3 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(3 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;
	psWidg = widgGetFromID(psWScreen,IDRET_BUILD);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseDesign() {
    if intCheckReticuleButEnabled(7 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(7 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;
	psWidg = widgGetFromID(psWScreen,IDRET_DESIGN);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseIntelligence() {
    if intCheckReticuleButEnabled(6 as libc::c_int as UDWORD) != 0 {
        setKeyButtonMapping(6 as libc::c_int as UDWORD);
    };
    /*
WIDGET *psWidg;
W_BUTTON *psButton;
	psWidg = widgGetFromID(psWScreen,IDRET_INTEL_MAP);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ChooseCancel() {
    setKeyButtonMapping(8 as libc::c_int as UDWORD);
    /*
WIDGET *psWidg;
W_BUTTON *psButton;
	psWidg = widgGetFromID(psWScreen,IDRET_CANCEL);
	psButton = (W_BUTTON*)psWidg;
	buttonClicked(psButton,WKEY_PRIMARY);
  */
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleDrivingMode() {
    /* No point unless we're tracking */
    if getWarCamStatus() != 0 {
        if getDrivingStatus() != 0 {
            StopDriverMode();
        } else if driveModeActive() == 0 as libc::c_int &&
                      demoGetStatus() == 0 as libc::c_int && bMultiPlayer == 0
         {
            StartDriverMode(0 as *mut DROID);
        }
    };
}
#[no_mangle]
pub static mut bMovePause: BOOL = 0 as libc::c_int;
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_MovePause() {
    if bMultiPlayer == 0 {
        // can't do it in multiplay
        if bMovePause == 0 {
            /* Then pause it */
            setGamePauseStatus(1 as libc::c_int);
            setConsolePause(1 as libc::c_int);
            setScriptPause(1 as libc::c_int);
            setAudioPause(1 as libc::c_int);
            /* And stop the clock */
            gameTimeStop();
            setWidgetsStatus(0 as libc::c_int);
            radarOnScreen = 0 as libc::c_int;
            bMovePause = 1 as libc::c_int
        } else {
            setWidgetsStatus(1 as libc::c_int);
            radarOnScreen = 1 as libc::c_int;
            /* Else get it going again */
            setGamePauseStatus(0 as libc::c_int);
            setConsolePause(0 as libc::c_int);
            setScriptPause(0 as libc::c_int);
            setAudioPause(0 as libc::c_int);
            /* And start the clock again */
            gameTimeStart();
            bMovePause = 0 as libc::c_int
        }
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_MoveToLastMessagePos() {
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    if audio_GetPreviousQueueTrackPos(&mut iX, &mut iY, &mut iZ) != 0 {
        // Should use requestRadarTrack but the camera gets jammed so use setViewpos - GJ
//		requestRadarTrack( iX, iY );
        setViewPos((iX >> 7 as libc::c_int) as UDWORD,
                   (iY >> 7 as libc::c_int) as UDWORD, 1 as libc::c_int);
    };
}
// --------------------------------------------------------------------------
/* Makes it snow if it's not snowing and stops it if it is */
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleWeather() {
    if atmosGetWeatherType() as libc::c_uint ==
           WT_NONE as libc::c_int as libc::c_uint {
        atmosSetWeatherType(WT_SNOWING);
        addConsoleMessage(b"Oh, the weather outside is frightful... SNOW\x00"
                              as *const u8 as *const libc::c_char as
                              *mut STRING, LEFT_JUSTIFY);
    } else if atmosGetWeatherType() as libc::c_uint ==
                  WT_SNOWING as libc::c_int as libc::c_uint {
        atmosSetWeatherType(WT_RAINING);
        addConsoleMessage(b"Singing in the rain, I\'m singing in the rain... RAIN\x00"
                              as *const u8 as *const libc::c_char as
                              *mut STRING, LEFT_JUSTIFY);
    } else {
        atmosInitSystem();
        atmosSetWeatherType(WT_NONE);
        addConsoleMessage(b"Forecast : Clear skies for all areas... NO WEATHER\x00"
                              as *const u8 as *const libc::c_char as
                              *mut STRING, LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectNextFactory() {
    selNextSpecifiedBuilding(REF_FACTORY as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectNextResearch() {
    selNextSpecifiedBuilding(REF_RESEARCH as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectNextPowerStation() {
    selNextSpecifiedBuilding(REF_POWER_GEN as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectNextCyborgFactory() {
    selNextSpecifiedBuilding(REF_CYBORG_FACTORY as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_KillEnemy() {
    let mut player_0: UDWORD = 0;
    let mut psCDroid: *mut DROID = 0 as *mut DROID;
    let mut psNDroid: *mut DROID = 0 as *mut DROID;
    //STRUCTURE	*psCStruct, *psNStruct;
    player_0 = 0 as libc::c_int as UDWORD;
    while player_0 < 8 as libc::c_int as libc::c_uint && bMultiPlayer == 0 {
        if player_0 != selectedPlayer {
            // wipe out all the droids
            psCDroid = apsDroidLists[player_0 as usize];
            while !psCDroid.is_null() {
                psNDroid = (*psCDroid).psNext;
                destroyDroid(psCDroid);
                psCDroid = psNDroid
            }
            // wipe out all their structures
		  //	for(psCStruct=apsStructLists[player]; psCStruct; psCStruct=psNStruct)
		  //	{
		  //		psNStruct = psCStruct->psNext;
		  //		destroyStruct(psCStruct);
		  //	}
        }
        player_0 = player_0.wrapping_add(1)
    };
}
// kill all the selected objects
#[no_mangle]
pub unsafe extern "C" fn kf_KillSelected() {
    let mut psCDroid: *mut DROID = 0 as *mut DROID;
    let mut psNDroid: *mut DROID = 0 as *mut DROID;
    let mut psCStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if bMultiPlayer != 0 { return }
    psCDroid = apsDroidLists[selectedPlayer as usize];
    while !psCDroid.is_null() {
        psNDroid = (*psCDroid).psNext;
        if (*psCDroid).selected != 0 {
            //		  	removeDroid(psCDroid);
            destroyDroid(psCDroid);
        }
        psCDroid = psNDroid
    }
    psCStruct = apsStructLists[selectedPlayer as usize];
    while !psCStruct.is_null() {
        psNStruct = (*psCStruct).psNext;
        if (*psCStruct).selected != 0 { destroyStruct(psCStruct); }
        psCStruct = psNStruct
    };
}
// --------------------------------------------------------------------------
// display the grid info for all the selected objects
#[no_mangle]
pub unsafe extern "C" fn kf_ShowGridInfo() {
    let mut psCDroid: *mut DROID = 0 as *mut DROID;
    let mut psNDroid: *mut DROID = 0 as *mut DROID;
    let mut psCStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psCDroid = apsDroidLists[selectedPlayer as usize];
    while !psCDroid.is_null() {
        psNDroid = (*psCDroid).psNext;
        if (*psCDroid).selected != 0 {
            gridDisplayCoverage(psCDroid as *mut BASE_OBJECT);
        }
        psCDroid = psNDroid
    }
    psCStruct = apsStructLists[selectedPlayer as usize];
    while !psCStruct.is_null() {
        psNStruct = (*psCStruct).psNext;
        if (*psCStruct).selected != 0 {
            gridDisplayCoverage(psCStruct as *mut BASE_OBJECT);
        }
        psCStruct = psNStruct
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_GiveTemplateSet() {
    addTemplateSet(4 as libc::c_int as UDWORD, 0 as libc::c_int as UDWORD);
    addTemplateSet(4 as libc::c_int as UDWORD, 1 as libc::c_int as UDWORD);
    addTemplateSet(4 as libc::c_int as UDWORD, 2 as libc::c_int as UDWORD);
    addTemplateSet(4 as libc::c_int as UDWORD, 3 as libc::c_int as UDWORD);
}
// --------------------------------------------------------------------------
// Chat message. NOTE THIS FUNCTION CAN DISABLE ALL OTHER KEYPRESSES
#[no_mangle]
pub unsafe extern "C" fn kf_SendTextMessage() {
    let mut ch: CHAR = 0;
    let mut tmp: [STRING; 100] = [0; 100];
    let mut i: SDWORD = 0;
    if bAllowOtherKeyPresses != 0 {
        // just starting.
        bAllowOtherKeyPresses = 0 as libc::c_int;
        strcpy(sTextToSend.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
        inputClearBuffer();
    }
    ch = inputGetKey() as CHAR;
    while ch as libc::c_int != 0 as libc::c_int {
        // in progress
        // Kill if they hit return - it maxes out console or it's more than one line long
        if ch as libc::c_int == 0xd as libc::c_int ||
               strlen(sTextToSend.as_mut_ptr()) >=
                   (255 as libc::c_int - 16 as libc::c_int) as libc::c_uint ||
               iV_GetTextWidth(sTextToSend.as_mut_ptr()) as libc::c_uint >
                   pie_GetVideoBufferWidth().wrapping_sub(64 as libc::c_int as
                                                              libc::c_uint) {
            // sendit
            //	if((ch == INPBUF_CR) || (strlen(sTextToSend)==MAX_TYPING_LENGTH)
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            // don't send empty lines to other players
            if strcmp(sTextToSend.as_mut_ptr(),
                      b"\x00" as *const u8 as *const libc::c_char) == 0 {
                return
            }
            /* process console commands (only if skirmish or multiplayer, not a campaign) */
            if game.type_0 as libc::c_int == 14 as libc::c_int ||
                   bMultiPlayer != 0 {
                if processConsoleCommands(sTextToSend.as_mut_ptr()) != 0 {
                    return
                    //it was a console command, so don't send
                }
            }
            //console callback message
				//--------------------------
            ConsolePlayer = selectedPlayer as SDWORD;
            strcpy(ConsoleMsg.as_mut_ptr(), sTextToSend.as_mut_ptr());
            eventFireCallbackTrigger(CALL_CONSOLE as libc::c_int as
                                         TRIGGER_TYPE);
            if bMultiPlayer != 0 && NetPlay.bComms != 0 {
                sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            } else {
                //show the message we sent on our local console as well (even in skirmish, to see console commands)
					//sprintf(tmp,"%d",selectedPlayer);
                sprintf(tmp.as_mut_ptr(),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        getPlayerName(selectedPlayer)); // seperator
                strcat(tmp.as_mut_ptr(),
                       b" : \x00" as *const u8 as
                           *const libc::c_char); // add message
                strcat(tmp.as_mut_ptr(), sTextToSend.as_mut_ptr());
                addConsoleMessage(tmp.as_mut_ptr(), DEFAULT_JUSTIFY);
                //in skirmish send directly to AIs, for command and chat procesing
                i = 0 as libc::c_int;
                while i < game.maxPlayers as libc::c_int {
                    //don't use MAX_PLAYERS here, although possible
                    if openchannels[i as usize] != 0 &&
                           i as libc::c_uint != selectedPlayer {
                        sendAIMessage(sTextToSend.as_mut_ptr(),
                                      selectedPlayer as SDWORD, i);
                    }
                    i += 1
                }
                if getDebugMappingStatus() != 0 {
                    attemptCheatCode(sTextToSend.as_mut_ptr());
                }
            }
            return
        } else {
            if ch as libc::c_int == 0x8 as libc::c_int {
                // delete
                if sTextToSend[0 as libc::c_int as usize] as libc::c_int !=
                       '\u{0}' as i32 {
                    // cant delete nothing!
                    sTextToSend[strlen(sTextToSend.as_mut_ptr()).wrapping_sub(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                                    as usize] = '\u{0}' as i32 as STRING
                }
            } else if ch as libc::c_int == 0x1b as libc::c_int {
                //abort.
                bAllowOtherKeyPresses = 1 as libc::c_int;
                //	flushConsoleMessages();
                return
            } else {
                // display
                sprintf(sTextToSend.as_mut_ptr(),
                        b"%s%c\x00" as *const u8 as *const libc::c_char,
                        sTextToSend.as_mut_ptr(),
                        inputGetCharKey() as libc::c_int);
            }
        }
        ch = inputGetKey() as CHAR
    }
    // macro store stuff
    if keyPressed(KEY_F1) != 0 {
        if keyDown(KEY_LCTRL) != 0 {
            strcpy(ingame.phrases[0 as libc::c_int as usize].as_mut_ptr(),
                   sTextToSend.as_mut_ptr());
        } else {
            strcpy(sTextToSend.as_mut_ptr(),
                   ingame.phrases[0 as libc::c_int as usize].as_mut_ptr());
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            return
        }
    }
    if keyPressed(KEY_F2) != 0 {
        if keyDown(KEY_LCTRL) != 0 {
            strcpy(ingame.phrases[1 as libc::c_int as usize].as_mut_ptr(),
                   sTextToSend.as_mut_ptr());
        } else {
            strcpy(sTextToSend.as_mut_ptr(),
                   ingame.phrases[1 as libc::c_int as usize].as_mut_ptr());
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            return
        }
    }
    if keyPressed(KEY_F3) != 0 {
        if keyDown(KEY_LCTRL) != 0 {
            strcpy(ingame.phrases[2 as libc::c_int as usize].as_mut_ptr(),
                   sTextToSend.as_mut_ptr());
        } else {
            strcpy(sTextToSend.as_mut_ptr(),
                   ingame.phrases[2 as libc::c_int as usize].as_mut_ptr());
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            return
        }
    }
    if keyPressed(KEY_F4) != 0 {
        if keyDown(KEY_LCTRL) != 0 {
            strcpy(ingame.phrases[3 as libc::c_int as usize].as_mut_ptr(),
                   sTextToSend.as_mut_ptr());
        } else {
            strcpy(sTextToSend.as_mut_ptr(),
                   ingame.phrases[3 as libc::c_int as usize].as_mut_ptr());
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            return
        }
    }
    if keyPressed(KEY_F5) != 0 {
        if keyDown(KEY_LCTRL) != 0 {
            strcpy(ingame.phrases[4 as libc::c_int as usize].as_mut_ptr(),
                   sTextToSend.as_mut_ptr());
        } else {
            strcpy(sTextToSend.as_mut_ptr(),
                   ingame.phrases[4 as libc::c_int as usize].as_mut_ptr());
            bAllowOtherKeyPresses = 1 as libc::c_int;
            //	flushConsoleMessages();
            sendTextMessage(sTextToSend.as_mut_ptr(), 0 as libc::c_int);
            return
        }
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleConsole() {
    if getConsoleDisplayStatus() != 0 {
        enableConsoleDisplay(0 as libc::c_int);
    } else { enableConsoleDisplay(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllOnScreenUnits() {
    selDroidSelection(selectedPlayer, DS_ALL_UNITS, DST_UNUSED,
                      1 as libc::c_int);
    /*
DROID	*psDroid;
UDWORD	dX,dY;

	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		if (DrawnInLastFrame(psDroid->sDisplay.frameNumber)==TRUE)
		{
			dX = psDroid->sDisplay.screenX;
			dY = psDroid->sDisplay.screenY;
			if(dX>0 AND dY>0 AND dX<DISP_WIDTH AND dY<DISP_HEIGHT)
			{
				psDroid->selected = TRUE;
			}
		}
	}
*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllUnits() {
    selDroidSelection(selectedPlayer, DS_ALL_UNITS, DST_UNUSED,
                      0 as libc::c_int);
    /*
DROID	*psDroid;
	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		psDroid->selected = TRUE;
	}
*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllVTOLs() {
    //	kfsf_SelectAllSameProp(LIFT);
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_VTOL, 0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllHovers() {
    //	kfsf_SelectAllSameProp(HOVER);
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_HOVER,
                      0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllWheeled() {
    //	kfsf_SelectAllSameProp(WHEELED);
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_WHEELED,
                      0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllTracked() {
    //	kfsf_SelectAllSameProp(TRACKED);
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_TRACKED,
                      0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllHalfTracked() {
    //	kfsf_SelectAllSameProp(HALF_TRACKED);
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_HALF_TRACKED,
                      0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllDamaged() {
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_ALL_DAMAGED,
                      0 as libc::c_int);
    /*
DROID	*psDroid;
UDWORD	damage;

	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		damage = PERCENT(psDroid->body,psDroid->originalBody);
		if(damage<REPAIRLEV_LOW)
		{
			psDroid->selected = TRUE;
		}
	}
*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllCombatUnits() {
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_ALL_COMBAT,
                      0 as libc::c_int);
    /*
DROID	*psDroid;
	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		if(psDroid->numWeaps)
		{
			psDroid->selected = TRUE;
		}
	}
*/
}
/* Support functions to minimise code size */
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kfsf_SelectAllSameProp(mut propType:
                                                    PROPULSION_TYPE) {
    /*
PROPULSION_STATS	*psPropStats;
DROID	*psDroid;

	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		if(!psDroid->selected)
		{
			psPropStats = asPropulsionStats + psDroid->asBits[COMP_PROPULSION].nStat;
			ASSERT( PTRVALID(psPropStats, sizeof(PROPULSION_STATS)),
					"moveUpdateDroid: invalid propulsion stats pointer" );
			if ( psPropStats->propulsionType == propType )
			{
				psDroid->selected = TRUE;
			}
		}
	}
	*/
}
// --------------------------------------------------------------------------
// this is worst case (size of apsDroidLists[selectedPlayer] squared).
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SelectAllSameType() {
    selDroidSelection(selectedPlayer, DS_BY_TYPE, DST_ALL_SAME,
                      0 as libc::c_int);
    /*
DROID	*psDroid;
//PROPULSION_STATS	*psPropStats;

	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		if(psDroid->selected)
		{
//			psPropStats = asPropulsionStats + psDroid->asBits[COMP_PROPULSION].nStat;
//			kfsf_SelectAllSameProp(psPropStats->propulsionType);	// non optimal - multiple assertion!?
			kfsf_SelectAllSameName(psDroid->aName);
		}
	}
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kfsf_SelectAllSameName(mut droidName: *mut STRING) {
    /*
DROID	*psDroid;
	for(psDroid = apsDroidLists[selectedPlayer]; psDroid; psDroid = psDroid->psNext)
	{
		// already selected - ignore
		if(!psDroid->selected)
		{
			if(!strcmp(droidName,psDroid->aName))
			{
				psDroid->selected = TRUE;
			}
		}
	}
	*/
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRangeShort() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_RANGE, DSS_ARANGE_SHORT);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRangeDefault() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_RANGE, DSS_ARANGE_DEFAULT);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRangeLong() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_RANGE, DSS_ARANGE_LONG);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRetreatMedium() {
    kfsf_SetSelectedDroidsState(DSO_REPAIR_LEVEL, DSS_REPLEV_LOW);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRetreatHeavy() {
    kfsf_SetSelectedDroidsState(DSO_REPAIR_LEVEL, DSS_REPLEV_HIGH);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRetreatNever() {
    kfsf_SetSelectedDroidsState(DSO_REPAIR_LEVEL, DSS_REPLEV_NEVER);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidAttackAtWill() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_LEVEL, DSS_ALEV_ALWAYS);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidAttackReturn() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_LEVEL, DSS_ALEV_ATTACKED);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidAttackCease() {
    kfsf_SetSelectedDroidsState(DSO_ATTACK_LEVEL, DSS_ALEV_NEVER);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidMoveHold() {
    kfsf_SetSelectedDroidsState(DSO_HALTTYPE, DSS_HALT_HOLD);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidMovePursue() {
    kfsf_SetSelectedDroidsState(DSO_HALTTYPE, DSS_HALT_PERSUE);
    // ASK?
}
//not there?
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidMovePatrol() {
    kfsf_SetSelectedDroidsState(DSO_PATROL, DSS_PATROL_SET);
    // ASK
}
// not there?
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidReturnToBase() {
    kfsf_SetSelectedDroidsState(DSO_RETURN_TO_LOC, DSS_RTL_BASE);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidGoForRepair() {
    kfsf_SetSelectedDroidsState(DSO_RETURN_TO_LOC, DSS_RTL_REPAIR);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_SetDroidRecycle() {
    kfsf_SetSelectedDroidsState(DSO_RECYCLE, DSS_RECYCLE_SET);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleVisibility() {
    if getRevealStatus() != 0 {
        setRevealStatus(0 as libc::c_int);
    } else { setRevealStatus(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kfsf_SetSelectedDroidsState(mut sec: SECONDARY_ORDER,
                                                     mut state:
                                                         SECONDARY_STATE) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if (*psDroid).selected != 0 {
            secondarySetState(psDroid, sec, state);
            /* Kick him out of group if he's going for repair */
            // Now done in secondarySetState
            //	if ((sec == DSO_RETURN_TO_LOC) && (state == DSS_RTL_REPAIR))
		   //	{
		   //		psDroid->group = UBYTE_MAX;
		   //		psDroid->selected = FALSE;
		   //	}
        }
        psDroid = (*psDroid).psNext
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_TriggerRayCast() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut found: BOOL = 0;
    let mut psOther: *mut DROID = 0 as *mut DROID;
    found = 0 as libc::c_int;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() && found == 0 {
        if (*psDroid).selected != 0 {
            found = 1 as libc::c_int;
            psOther = psDroid
        }
        psDroid = (*psDroid).psNext
        /* NOP */
    }
    (found) != 0;
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ScatterDroids() {
    // to be written!
    addConsoleMessage(b"Scatter droids - not written yet!\x00" as *const u8 as
                          *const libc::c_char as *mut STRING, LEFT_JUSTIFY);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_CentreOnBase() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut bGotHQ: BOOL = 0;
    let mut xJump: UDWORD = 0 as libc::c_int as UDWORD;
    let mut yJump: UDWORD = 0 as libc::c_int as UDWORD;
    /* Got through our buildings */
    psStruct = apsStructLists[selectedPlayer as usize]; // start
    bGotHQ = 0 as libc::c_int;
    while !psStruct.is_null() && bGotHQ == 0 {
        // iteration
        /* Have we got a HQ? */
        if (*(*psStruct).pStructureType).type_0 ==
               REF_HQ as libc::c_int as libc::c_uint {
            bGotHQ = 1 as libc::c_int;
            xJump =
                ((*psStruct).x as
                     libc::c_uint).wrapping_sub(visibleXTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(128
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint));
            yJump =
                ((*psStruct).y as
                     libc::c_uint).wrapping_sub(visibleYTiles.wrapping_div(2
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(128
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint))
        }
        psStruct = (*psStruct).psNext
    }
    /* If we found it, then jump to it! */
    if bGotHQ != 0 {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_GOHQ as libc::c_int as
                                              UDWORD),
                          LEFT_JUSTIFY); // face north
        player.p.x = xJump as int32;
        player.p.z = yJump as int32;
        player.r.y = 0 as libc::c_int;
        /* A fix to stop the camera continuing when marker code is called */
        if getWarCamStatus() != 0 { camToggleStatus(); }
    } else {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_NOHQ as libc::c_int as
                                              UDWORD), LEFT_JUSTIFY);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleFormationSpeedLimiting() {
    if bMultiPlayer != 0 { return }
    if moveFormationSpeedLimitingOn() != 0 {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_FORMATION_OFF as libc::c_int
                                              as UDWORD), LEFT_JUSTIFY);
    } else {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_FORMATION_ON as libc::c_int
                                              as UDWORD), LEFT_JUSTIFY);
    }
    moveToggleFormationSpeedLimiting();
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_RightOrderMenu() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psGotOne: *mut DROID = 0 as *mut DROID;
    let mut bFound: BOOL = 0;
    // if menu open, then close it!
    if !widgGetFromID(psWScreen, 8000 as libc::c_int as UDWORD).is_null() {
        intRemoveOrder(); // close the screen.
        return
    }
    psDroid = apsDroidLists[selectedPlayer as usize];
    bFound = 0 as libc::c_int;
    while !psDroid.is_null() && bFound == 0 {
        if (*psDroid).selected != 0 {
            // AND droidOnScreen(psDroid,0))
            bFound = 1 as libc::c_int;
            psGotOne = psDroid
        }
        psDroid = (*psDroid).psNext
    }
    if bFound != 0 {
        intResetScreen(1 as libc::c_int);
        intObjectSelected(psGotOne as *mut BASE_OBJECT);
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ScriptTest() {
    let mut pBuffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: UDWORD = 0;
    eventSaveState(1 as libc::c_int, &mut pBuffer, &mut size);
    eventReset();
    eventLoadState(pBuffer, size, 1 as libc::c_int);
    memFreeRelease(pBuffer as *mut libc::c_void);
    pBuffer = 0 as *mut libc::c_char;
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_TriggerShockWave() {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    pos.x =
        mouseTileX * 128 as libc::c_int +
            128 as libc::c_int / 2 as libc::c_int;
    pos.z =
        mouseTileY * 128 as libc::c_int +
            128 as libc::c_int / 2 as libc::c_int;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            64 as libc::c_int;
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SHOCKWAVE,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleMouseInvert() {
    if getInvertMouseStatus() != 0 {
        setInvertMouseStatus(0 as libc::c_int);
    } else { setInvertMouseStatus(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleShakeStatus() {
    if getShakeStatus() != 0 {
        setShakeStatus(0 as libc::c_int);
    } else { setShakeStatus(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleShadows() {
    if getDrawShadows() != 0 {
        setDrawShadows(0 as libc::c_int);
    } else { setDrawShadows(1 as libc::c_int); };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub static mut available_speed: [FRACT; 11] =
    [1 as libc::c_int as libc::c_float / 8 as libc::c_int as libc::c_float,
     1 as libc::c_int as libc::c_float / 4 as libc::c_int as libc::c_float,
     1 as libc::c_int as libc::c_float / 2 as libc::c_int as libc::c_float,
     3 as libc::c_int as libc::c_float / 4 as libc::c_int as libc::c_float,
     1 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float,
     3 as libc::c_int as libc::c_float / 2 as libc::c_int as libc::c_float,
     2 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float,
     5 as libc::c_int as libc::c_float / 2 as libc::c_int as libc::c_float,
     3 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float,
     10 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float,
     20 as libc::c_int as libc::c_float / 1 as libc::c_int as libc::c_float];
#[no_mangle]
pub static mut nb_available_speeds: libc::c_uint =
    11 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn kf_SpeedUp() {
    let mut mod_0: FRACT = 0.;
    if (bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int) &&
           bInTutorial == 0 {
        let mut i: libc::c_int = 0;
        // get the current modifier
        gameTimeGetMod(&mut mod_0);
        i = 1 as libc::c_int;
        while (i as libc::c_uint) < nb_available_speeds {
            if mod_0 < available_speed[i as usize] {
                mod_0 = available_speed[i as usize];
                if mod_0 ==
                       1 as libc::c_int as libc::c_float /
                           1 as libc::c_int as libc::c_float {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_NORMAL_SPEED as
                                                libc::c_int as UDWORD));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_SPEED_UP as libc::c_int as
                                                UDWORD),
                            mod_0 as libc::c_double);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
                gameTimeSetMod(mod_0);
                break ;
            } else { i += 1 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn kf_SlowDown() {
    let mut mod_0: FRACT = 0.;
    if (bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int) &&
           bInTutorial == 0 {
        let mut i: libc::c_int = 0;
        // get the current modifier
        gameTimeGetMod(&mut mod_0);
        i =
            nb_available_speeds.wrapping_sub(2 as libc::c_int as libc::c_uint)
                as libc::c_int;
        while i >= 0 as libc::c_int {
            if mod_0 > available_speed[i as usize] {
                mod_0 = available_speed[i as usize];
                if mod_0 ==
                       1 as libc::c_int as libc::c_float /
                           1 as libc::c_int as libc::c_float {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_NORMAL_SPEED as
                                                libc::c_int as UDWORD));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                } else {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_SLOW_DOWN as libc::c_int
                                                as UDWORD),
                            mod_0 as libc::c_double);
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                }
                gameTimeSetMod(mod_0);
                break ;
            } else { i -= 1 }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn kf_NormalSpeed() {
    if (bMultiPlayer == 0 || NetPlay.bComms == 0 as libc::c_int) &&
           bInTutorial == 0 {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GAM_NORMAL_SPEED as libc::c_int as
                                    UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        gameTimeResetMod();
    };
}
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleReopenBuildMenu() {
    intReopenBuild((intGetReopenBuild() == 0) as libc::c_int);
    if intGetReopenBuild() != 0 {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GAM_BUILD_REOPEN as libc::c_int as
                                    UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    } else {
        sprintf(ConsoleString.as_mut_ptr(),
                strresGetString(psStringRes,
                                STR_GAM_BUILD_NO_REOPEN as libc::c_int as
                                    UDWORD));
        addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
    };
}
//radar terrain
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleRadarAllyEnemy() {
    if bEnemyAllyRadarColor == 1 as libc::c_int {
        bEnemyAllyRadarColor = 0 as libc::c_int;
        resetRadarRedraw();
    } else { bEnemyAllyRadarColor = 1 as libc::c_int; resetRadarRedraw(); };
}
#[no_mangle]
pub unsafe extern "C" fn kf_ToggleRadarTerrain() {
    if bDrawRadarTerrain == 1 as libc::c_int {
        bDrawRadarTerrain = 0 as libc::c_int;
        resetRadarRedraw();
    } else { bDrawRadarTerrain = 1 as libc::c_int; resetRadarRedraw(); };
}
//start in nicks mode
//Returns TRUE if the engine should dofurther text processing, FALSE if just exit
#[no_mangle]
pub unsafe extern "C" fn processConsoleCommands(mut pName: *mut STRING)
 -> BOOL {
    let mut bFound: BOOL = 0 as libc::c_int;
    let mut i: SDWORD = 0;
    if strcmp(pName, b"/loadai\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        LoadAIExperience(1 as libc::c_int);
        return 1 as libc::c_int
    } else {
        if strcmp(pName, b"/saveai\x00" as *const u8 as *const libc::c_char)
               == 0 as libc::c_int {
            SaveAIExperience(1 as libc::c_int);
            return 1 as libc::c_int
        } else {
            if strcmp(pName,
                      b"/maxplayers\x00" as *const u8 as *const libc::c_char)
                   == 0 as libc::c_int {
                console(b"game.maxPlayers: &d\x00" as *const u8 as
                            *const libc::c_char,
                        game.maxPlayers as libc::c_int);
                return 1 as libc::c_int
            } else {
                if strcmp(pName,
                          b"/bd\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int {
                    BaseExperienceDebug(selectedPlayer as SDWORD);
                    return 1 as libc::c_int
                } else {
                    if strcmp(pName,
                              b"/sm\x00" as *const u8 as *const libc::c_char)
                           == 0 as libc::c_int {
                        i = 0 as libc::c_int;
                        while i < 8 as libc::c_int {
                            console(b"%d - %d\x00" as *const u8 as
                                        *const libc::c_char, i,
                                    game.skDiff[i as usize] as libc::c_int);
                            i += 1
                        }
                        return 1 as libc::c_int
                    } else {
                        if strcmp(pName,
                                  b"/od\x00" as *const u8 as
                                      *const libc::c_char) == 0 as libc::c_int
                           {
                            OilExperienceDebug(selectedPlayer as SDWORD);
                            return 1 as libc::c_int
                        } else {
                            let mut tmpStr: [libc::c_char; 255] = [0; 255];
                            /* saveai x */
                            i = 0 as libc::c_int; //"saveai 0"
                            while i < 8 as libc::c_int {
                                sprintf(tmpStr.as_mut_ptr(),
                                        b"/saveai %d\x00" as *const u8 as
                                            *const libc::c_char, i);
                                if strcmp(pName, tmpStr.as_mut_ptr()) ==
                                       0 as libc::c_int {
                                    SavePlayerAIExperience(i,
                                                           1 as libc::c_int);
                                    return 1 as libc::c_int
                                }
                                i += 1
                            }
                            /* loadai x */
                            i = 0 as libc::c_int; //"loadai 0"
                            while i < 8 as libc::c_int {
                                sprintf(tmpStr.as_mut_ptr(),
                                        b"/loadai %d\x00" as *const u8 as
                                            *const libc::c_char, i);
                                if strcmp(pName, tmpStr.as_mut_ptr()) ==
                                       0 as libc::c_int {
                                    LoadPlayerAIExperience(i,
                                                           1 as libc::c_int);
                                    return 1 as libc::c_int
                                }
                                i += 1
                            }
                        }
                    }
                }
            }
        }
    }
    return bFound;
}
