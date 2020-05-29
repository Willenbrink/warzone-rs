use ::libc;
extern "C" {
    pub type _formation;
    /* Misc || X/Open.  */
    /* Set N bytes of S to C.  */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
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
    /* Abort execution and generate a core-dump.  */
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn grpLeave(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    #[no_mangle]
    fn orderUpdateDroid(psDroid: *mut DROID);
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
    //UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
    /* the current amount of power avaialble 
									    to the player*/
    /* the power extracted but not converted */
    /*the last object that received power 
									    before it ran out*/
    /*allocate the space for the playerPower*/
    /*clear the playerPower */
    /*initialise the PlayerPower list */
//extern initPlayerPower(void);
    /*reset the power levels when a power_gen or resource_extractor is destroyed */
    /*Free the space used for playerPower */
    /*check the available power*/
    /*subtract the power required */
    //return the power when a structure/droid is deliberately destroyed
    /* Each Resource Extractor yields EXTRACT_POINTS per second until there are none
   left in the resource. */
//extern void updateExtractedPower(STRUCTURE	*psBuilding);
    /* Update current power based on what was extracted during the last cycle and 
   what Power Generators exist */
    // used in multiplayer to force power levels.
    /*resets the power levels for all players when power is turned back on*/
    /*sets the initial value for the power*/
    /*Temp function to give all players some power when a new game has been loaded*/
    //informs the power array that a Object has been destroyed
    //extern void spreadPower(UBYTE player);
/*accrue the power in the facilities that require it*/
    //returns the relevant list based on OffWorld or OnWorld for the accruePower function
//extern STRUCTURE* powerUpdateStructList(UBYTE player);
    /*inform the players power struct that the last Object to receive power has changed*/
    /*	Returns the next res. Ext. in the list from the one passed in. returns 1st one
	in list if passed in is NULL and NULL if there's none?
*/
    /*checks if the object to be powered next - returns TRUE if power*/
    /*defines which structure types draw power - returns TRUE if use power*/
    /*defines which droid types draw power - returns TRUE if use power*/
    //won't bother with this on PSX unless starts being used too much!
    //this is a check cos there is a problem with the power but not sure where!!
    //flag used to check for power calculations to be done or not
    #[no_mangle]
    static mut powerCalculated: BOOL;
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
    #[no_mangle]
    fn getLastPowered(psStructure: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn updateLastPowered(psObject: *mut BASE_OBJECT, player_0: UBYTE);
    #[no_mangle]
    fn accruePower(psObject: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn usePower(player_0: UDWORD, quantity: UDWORD) -> BOOL;
    #[no_mangle]
    fn checkPower(player_0: UDWORD, quantity: UDWORD, playAudio: BOOL)
     -> BOOL;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    fn map_Height(x: UDWORD, y: UDWORD) -> SWORD;
    #[no_mangle]
    static mut aMapLinePoints: *mut TILE_COORD;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Set the current cursor from a Resource ID
 * This is the same as calling:
 *       frameSetCursor(LoadCursor(MAKEINTRESOURCE(resID)));
 * but with a bit of extra error checking.
 */
    #[no_mangle]
    fn frameSetCursorFromRes(resID: WORD);
    #[no_mangle]
    fn pie_GetVideoBufferWidth() -> UDWORD;
    #[no_mangle]
    fn pie_GetVideoBufferHeight() -> UDWORD;
    #[no_mangle]
    fn audio_PlayObjDynamicTrack(psObj: *mut libc::c_void,
                                 iTrack: libc::c_int,
                                 pUserCallback: AUDIO_CALLBACK) -> BOOL;
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    fn audio_StopAll();
    #[no_mangle]
    fn removeDroidBase(psDel: *mut DROID);
    /*Builds an instance of a Structure - the x/y passed in are in world coords.*/
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player_0: UDWORD, onMission: BOOL) -> *mut DROID;
    /* Same as destroy droid except no graphical effects */
    #[no_mangle]
    fn vanishDroid(psDel: *mut DROID);
    /* Remove a droid from the apsDroidLists so doesn't update or get drawn etc*/
//returns TRUE if successfully removed from the list
    #[no_mangle]
    fn droidRemove(psDroid: *mut DROID, pList: *mut *mut DROID) -> BOOL;
    #[no_mangle]
    fn pickHalfATile(x: *mut UDWORD, y: *mut UDWORD, numIterations: UBYTE)
     -> PICKTILE;
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
    /* Just returns true if the droid's present body points aren't as high as the original*/
    #[no_mangle]
    fn droidIsDamaged(psDroid: *mut DROID) -> BOOL;
    /*returns TRUE if droid type is one of the Cyborg types*/
    #[no_mangle]
    fn cyborgDroid(psDroid: *mut DROID) -> BOOL;
    //stores which player the production list has been set up for
    #[no_mangle]
    static mut productionPlayer: SBYTE;
    #[no_mangle]
    fn CheckHaltOnMaxUnitsReached(psStructure: *mut STRUCTURE) -> BOOL;
    /* Set the type of droid for a factory to build */
    #[no_mangle]
    fn structSetManufacture(psStruct: *mut STRUCTURE,
                            psTempl: *mut DROID_TEMPLATE, quantity: UBYTE)
     -> BOOL;
    // remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
    /* set the current number of structures of each type built */
    #[no_mangle]
    fn setCurrentStructQuantity(displayError: BOOL);
    /*initialises the flag before a new data set is loaded up*/
    #[no_mangle]
    fn initFactoryNumFlag();
    //called at start of missions
    #[no_mangle]
    fn resetFactoryNumFlag();
    /*this is called whenever a structure has finished building*/
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    // Set the command droid that factory production should go to
//struct _command_droid;
    #[no_mangle]
    fn assignFactoryCommandDroid(psStruct: *mut STRUCTURE,
                                 psCommander: *mut _droid);
    // remove all factories from a command droid
    #[no_mangle]
    fn clearCommandDroidFactory(psDroid: *mut DROID);
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    /*this is called when a factory produces a droid. The Template returned is the next 
one to build - if any*/
    #[no_mangle]
    fn factoryProdUpdate(psStructure: *mut STRUCTURE,
                         psTemplate: *mut DROID_TEMPLATE)
     -> *mut DROID_TEMPLATE;
    /*set a factory's production run to hold*/
    #[no_mangle]
    fn holdProduction(psBuilding: *mut STRUCTURE);
    /*Initialise the production list and set up the production player*/
    #[no_mangle]
    fn changeProductionPlayer(player_0: UBYTE);
    /*returns the power cost to build this structure*/
    #[no_mangle]
    fn structPowerToBuild(psStruct: *mut STRUCTURE) -> UDWORD;
    /* The lists of objects allocated */
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    /* General housekeeping for the object system */
    #[no_mangle]
    fn objmemUpdate();
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    /* Remove all droids */
    #[no_mangle]
    fn freeAllDroids();
    /*Removes all droids that may be stored in the mission lists*/
    #[no_mangle]
    fn freeAllMissionDroids();
    /*Removes all droids that may be stored in the limbo lists*/
    #[no_mangle]
    fn freeAllLimboDroids();
    /* Remove all structures */
    #[no_mangle]
    fn freeAllStructs();
    /* Remove all features */
    #[no_mangle]
    fn freeAllFeatures();
    /*Stops a droid dead in its tracks - doesn't allow for any little skidding bits*/
    #[no_mangle]
    fn moveReallyStopDroid(psDroid: *mut DROID);
    /* update body and turret to local slope */
    #[no_mangle]
    fn updateDroidOrientation(psDroid: *mut DROID);
    /* audio callback used to kill movement sounds */
    #[no_mangle]
    fn moveCheckDroidMovingAndVisible(psSample: *mut AUDIO_SAMPLE) -> BOOL;
    /*goes thru' the list passed in reversing the order so the first entry becomes 
the last and the last entry becomes the first!*/
    #[no_mangle]
    fn reverseObjectList(ppsList: *mut *mut BASE_OBJECT);
    #[no_mangle]
    static mut mapWidth: UDWORD;
    #[no_mangle]
    static mut mapHeight: UDWORD;
    #[no_mangle]
    static mut psMapTiles: *mut MAPTILE;
    #[no_mangle]
    fn mapShutdown() -> BOOL;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* Call this to stop the game timer */
    #[no_mangle]
    fn gameTimeStop();
    /* Call this to restart the game timer after a call to gameTimeStop */
    #[no_mangle]
    fn gameTimeStart();
    /* **************************************************************************/
/*
 *	Global ProtoTypes
 */
/* **************************************************************************/
    #[no_mangle]
    fn loadGame(pGameToLoad: *mut STRING, keepObjects: BOOL, freeMem: BOOL,
                UserSaveGame: BOOL) -> BOOL;
    // UserSaveGame is TRUE when the save game is not a new level (User Save Game)
    /*This just loads up the .gam file to determine which level data to set up - split up
so can be called in levLoadData when starting a game from a load save game*/
    #[no_mangle]
    fn loadGameInit(pGameToLoad: *mut STRING) -> BOOL;
    #[no_mangle]
    fn saveGame(aFileName: *mut STRING, saveType: SDWORD) -> BOOL;
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn proj_FreeAllProjectiles();
    #[no_mangle]
    fn releaseAllProxDisp();
    #[no_mangle]
    fn freeMessages();
    //add proximity messages for all untapped VISIBLE oil resources
    #[no_mangle]
    fn addOilResourceProximities();
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /* process the results of a completed research topic */
    #[no_mangle]
    fn researchResult(researchIndex: UDWORD, player_0: UBYTE, bDisplay: BOOL);
    /*puts research facility on hold*/
    #[no_mangle]
    fn holdResearch(psBuilding: *mut STRUCTURE);
    //NOT ANYMORE! 10/08/98 AB
//#ifndef PSX
//#define INCLUDE_PRODSLIDER	// Include quantity slider in manufacture window.
//#endif
    //#ifndef PSX
    //#endif
    #[no_mangle]
    static mut intMode: INTMODE;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* the widget font */
    #[no_mangle]
    static mut WFont: libc::c_int;
    /* Add the reticule widgets to the widget screen */
    #[no_mangle]
    fn intAddReticule() -> BOOL;
    #[no_mangle]
    fn intRemoveReticule();
    /* Tell the interface a research facility has completed a topic */
    #[no_mangle]
    fn intResearchFinished(psBuilding: *mut STRUCTURE);
    /* Tell the interface a factory has completed building ALL droids */
    #[no_mangle]
    fn intManufactureFinished(psBuilding: *mut STRUCTURE);
    /* Reset the widget screen to just the reticule */
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    /* Refresh icons on the interface, without disturbing the layout. i.e. smartreset*/
    #[no_mangle]
    fn intRefreshScreen();
    //displays the Power Bar
    #[no_mangle]
    fn intShowPowerBar();
    //hides the power bar from the display
//extern void intHidePowerBar(void);
    //hides the power bar from the display - regardless of what player requested!
    /* Add the Proximity message buttons */
    /*Remove a Proximity Button - when the message is deleted*/
    /* Allows us to fool the widgets with a keypress */
    /*Checks to see if there are any research topics to do and flashes the button*/
    // see if a reticule button is enabled
    //access function for selected object in the interface
    //initialise all the previous obj - particularly useful for when go Off world!
    #[no_mangle]
    fn intResetPreviousObj();
    #[no_mangle]
    fn widgAddButton(psScreen: *mut W_SCREEN, psInit: *mut W_BUTINIT) -> BOOL;
    #[no_mangle]
    fn intCheckResearchButton();
    #[no_mangle]
    fn widgDelete(psScreen: *mut W_SCREEN, id: UDWORD);
    #[no_mangle]
    fn widgAddLabel(psScreen: *mut W_SCREEN, psInit: *mut W_LABINIT) -> BOOL;
    #[no_mangle]
    fn widgAddForm(psScreen: *mut W_SCREEN, psInit: *mut W_FORMINIT) -> BOOL;
    #[no_mangle]
    fn forceHidePowerBar();
    /* Find a widget in a screen from its ID number */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    #[no_mangle]
    fn widgGetUserData(psScreen: *mut W_SCREEN, id: UDWORD)
     -> *mut libc::c_void;
    #[no_mangle]
    fn widgSetUserData(psScreen: *mut W_SCREEN, id: UDWORD,
                       UserData: *mut libc::c_void);
    #[no_mangle]
    fn setWidgetsStatus(var: BOOL);
    #[no_mangle]
    fn getWidgetsStatus() -> BOOL;
    // after failing a route ... this is the amount of time that the droid goes all defensive untill it can start going aggressive
    // 1.5 sec
    // 4 secs
    //this is how long a droid is disabled for when its been attacked by an EMP weapon
    // 10 secs
    /* Update the action state for a droid */
    #[no_mangle]
    fn actionUpdateDroid(psDroid: *mut DROID);
    #[no_mangle]
    static mut player: iView;
    //mouse states
    #[no_mangle]
    fn pie_SetMouse(ImageFile: *mut IMAGEFILE, ImageID: UWORD);
    // and so's this
    #[no_mangle]
    fn initEffectsSystem();
    #[no_mangle]
    fn addEffect(pos: *mut iVector, group: EFFECT_GROUP, type_0: EFFECT_TYPE,
                 specified: BOOL, imd: *mut iIMDShape, lit: BOOL);
    #[no_mangle]
    fn effectSetLandLightSpec(spec: LAND_LIGHT_SPEC);
    //#define RADAR_POSITION_AT_ZOOM
    /* Radar.h */
    #[no_mangle]
    fn resetRadarRedraw();
    //The Transporter capacity label
    //initialises Transporter variables
    #[no_mangle]
    fn initTransporters();
    /*check to see if the droid can fit on the Transporter - return TRUE if fits*/
    #[no_mangle]
    fn checkTransporterSpace(psTransporter: *mut DROID,
                             psAssigned: *mut DROID) -> BOOL;
    #[no_mangle]
    fn intUpdateTransCapacity(psWidget: *mut _widget,
                              psContext: *mut _w_context);
    /* Remove the Transporter Launch widget from the screen*/
    #[no_mangle]
    fn intRemoveTransporterLaunch();
    /*This is used to display the transporter button and capacity when at the home base ONLY*/
    #[no_mangle]
    fn intAddTransporterLaunch(psDroid: *mut DROID) -> BOOL;
    /* set current transporter (for script callbacks) */
    #[no_mangle]
    fn transporterSetScriptCurrent(psTransporter: *mut DROID);
    /*set the time for the Launch*/
    #[no_mangle]
    fn transporterSetLaunchTime(time: UDWORD);
    #[no_mangle]
    fn flashMissionButton(buttonID: UDWORD);
    #[no_mangle]
    fn stopMissionButtonFlash(buttonID: UDWORD);
    /*checks the order of the droid to see if its currenly flying*/
    #[no_mangle]
    fn transporterFlying(psTransporter: *mut DROID) -> BOOL;
    //initialise the flag to indicate the first transporter has arrived - set in startMission()
    #[no_mangle]
    fn initFirstTransporterFlag();
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    #[no_mangle]
    fn processFrontendSnap(bHideCursor: BOOL);
    #[no_mangle]
    fn displayTextOption(psWidget: *mut _widget, xOffset: UDWORD,
                         yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intSetCurrentCursorPosition(Snap: *mut CURSORSNAP, id: UDWORD);
    #[no_mangle]
    static mut IntImages: *mut IMAGEFILE;
    #[no_mangle]
    fn intOpenPlainForm(psWidget: *mut _widget, xOffset: UDWORD,
                        yOffset: UDWORD, pColours: *mut UDWORD);
    /* Initialise a form widget before running it */
    /* Return the widgets currently displayed by a form */
    /* Return the origin on the form from which button locations are calculated */
    /* Variables for the formGetAllWidgets functions */
    /* Initialise the formGetAllWidgets function */
    /* Repeated calls to this function will return widget lists
 * until all widgets in a form have been returned.
 * When a NULL list is returned, all widgets have been seen.
 */
    /* Get the button state of a click form */
    /* Set the button state of a click form */
    #[no_mangle]
    fn formSetClickState(psForm: *mut W_CLICKFORM, state: UDWORD);
    #[no_mangle]
    fn intDisplayImageHilight(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    #[no_mangle]
    fn intDisplayMissionClock(psWidget: *mut _widget, xOffset: UDWORD,
                              yOffset: UDWORD, pColours: *mut UDWORD);
    // reset the input state
    #[no_mangle]
    fn resetInput();
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut bLoadSaveUp: BOOL;
    #[no_mangle]
    static mut sRequestResult: [STRING; 255];
    #[no_mangle]
    static mut bRequestLoad: BOOL;
    #[no_mangle]
    fn addLoadSave(mode: LOADSAVE_MODE, defaultdir: *mut CHAR,
                   extension: *mut CHAR, title: *mut CHAR) -> BOOL;
    #[no_mangle]
    fn closeLoadSave() -> BOOL;
    #[no_mangle]
    fn runLoadSave(bResetMissionWidgets: BOOL) -> BOOL;
    /* Set the callback table */
    /* Set the type equivalence table */
    /* **********************************************************************************
 *
 * Return stack stuff
 */
    /* **********************************************************************************
 *
 * Compiler functions
 */
    /* Compile a script program */
    /* Free a SCRIPT_CODE structure */
    /* Display the contents of a program in readable form */
    /* Lookup a script variable */
    /* Run a compiled script */
    /* **********************************************************************************
 *
 * Event system functions
 */
    // Whether a context is released when there are no active triggers for it
    // release the context
    // do not release the context
    // reset the event system
    // Initialise the create/release function array - specify the maximum value type
    // a create function for data stored in an INTERP_VAL
    // a release function for data stored in an INTERP_VAL
    // Add a new value create function
    // Add a new value release function
    // Create a new context for a script
    // Copy a context, including variable values
    // Add a new object to the trigger system
// Time is the application time at which all the triggers are to be started
    // Remove a context from the event system
    // Set a global variable value for a context
    // Get the value pointer for a variable index
    // Process all the currently active triggers
// Time is the application time at which all the triggers are to be processed
    // Activate a callback trigger
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    #[no_mangle]
    fn getWarCamStatus() -> BOOL;
    #[no_mangle]
    fn camToggleStatus();
    #[no_mangle]
    fn setPlayerHasLost(val: BOOL);
    #[no_mangle]
    fn testPlayerHasLost() -> BOOL;
    #[no_mangle]
    fn testPlayerHasWon() -> BOOL;
    #[no_mangle]
    fn setPlayerHasWon(val: BOOL);
    #[no_mangle]
    fn setScriptWinLoseVideo(val: UBYTE);
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn sendDroidDisEmbark(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn iV_GetImageWidth(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn iV_GetImageHeight(ImageFile: *mut IMAGEFILE, ID: UWORD) -> UWORD;
    #[no_mangle]
    fn pie_LoadBackDrop(screenType: SCREENTYPE, b3DFX: BOOL);
    //this function is called whenever the map changes - load new level or return from an offWorld map
    #[no_mangle]
    fn environReset();
    #[no_mangle]
    static mut loopMissionState: LOOP_MISSION_STATE;
    // this is set by scrStartMission to say what type of new level is to be started
    #[no_mangle]
    static mut nextMissionType: SDWORD;
    #[no_mangle]
    fn setGameUpdatePause(state: BOOL);
    #[no_mangle]
    fn setAudioPause(state: BOOL);
    #[no_mangle]
    fn setScriptPause(state: BOOL);
    #[no_mangle]
    fn setConsolePause(state: BOOL);
    // free the data for the current mission
    #[no_mangle]
    fn levReleaseMissionData() -> BOOL;
    /* Check which tiles can be seen by an object */
    #[no_mangle]
    fn visTilesUpdate(psObj: *mut BASE_OBJECT, SpreadLoad: BOOL);
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // initialise the cluster system
    #[no_mangle]
    fn clustInitialise();
    /*
 * Gateway.h
 *
 * Interface to routing gateway code.
 *
 */
    // the list of gateways on the current map
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    // the RLE map zones for each tile
    #[no_mangle]
    static mut apRLEZones: *mut *mut UBYTE;
    // the number of map zones
    #[no_mangle]
    static mut gwNumZones: SDWORD;
    // The zone equivalence tables
    #[no_mangle]
    static mut aNumEquiv: *mut UBYTE;
    #[no_mangle]
    static mut apEquivZones: *mut *mut UBYTE;
    #[no_mangle]
    static mut aZoneReachable: *mut UBYTE;
    // Shutdown the gateway system
    #[no_mangle]
    fn gwShutDown();
    #[no_mangle]
    fn selDroidDeselect(player_0: UDWORD) -> UDWORD;
    #[no_mangle]
    fn scoreInitSystem() -> BOOL;
    #[no_mangle]
    fn scoreDataToScreen();
    #[no_mangle]
    fn getDebugMappingStatus() -> BOOL;
    #[no_mangle]
    fn cdAudio_PlayTrack(iTrack: SDWORD) -> BOOL;
    #[no_mangle]
    fn cdAudio_Stop() -> BOOL;
    /*
 * mission.c
 *
 * all the stuff relevant to a mission
 */
    //#include "texture.h"	   // ffs
    #[no_mangle]
    static mut InterfaceSnap: CURSORSNAP;
    //EXTERNALS*************
    #[no_mangle]
    static mut SaveGamePath: [libc::c_char; 0];
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
pub type WORD = libc::c_short;
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
// root of the tree
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
pub type FRACT_D = libc::c_float;
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
// The next free ID
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_research {
    pub currentPoints: UDWORD,
    pub ResearchStatus: UBYTE,
}
pub type PLAYER_RESEARCH = _player_research;
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
/* The time the research facility was put on hold*/
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
pub type PICKTILE = libc::c_uint;
pub const HALF_FREE_TILE: PICKTILE = 2;
pub const FREE_TILE: PICKTILE = 1;
pub const NO_FREE_TILE: PICKTILE = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
pub type PLAYER_POWER = _player_power;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _landing_zone {
    pub x1: UBYTE,
    pub y1: UBYTE,
    pub x2: UBYTE,
    pub y2: UBYTE,
}
pub type LANDING_ZONE = _landing_zone;
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
// Feature armour
/*
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
// flags for the gateway links
// the link is part of the current route - to the previous gateway
// the link is part of the current route - to the next gateway
// the route between the two zones is blocked
// the flags that get reset by the router
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway {
    pub x1: UBYTE,
    pub y1: UBYTE,
    pub x2: UBYTE,
    pub y2: UBYTE,
    pub zone1: UBYTE,
    pub zone2: UBYTE,
    pub psNext: *mut _gateway,
    pub psLinks: *mut GATEWAY_LINK,
    pub zone1Links: UBYTE,
    pub zone2Links: UBYTE,
    pub flags: UBYTE,
    pub dist: SWORD,
    pub est: SWORD,
    pub psOpen: *mut _gateway,
    pub psRoute: *mut _gateway,
}
pub type GATEWAY_LINK = _gateway_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
//storage structure for values that need to be kept between missions
pub type MISSION = _mission;
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
pub type DROID_ORDER = _droid_order;
pub type DROID_GROUP = _droid_group;
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
// off map saving any droids (selectedPlayer) at end into apsLimboDroids
pub const LDS_NONE: _level_type = 10;
pub type GATEWAY = _gateway;
// Previous point in the route
// expand campaign map using droids held in apsLimboDroids
pub const LDS_MKEEP_LIMBO: _level_type = 9;
// off map mission (extra map data)
pub const LDS_MCLEAR: _level_type = 7;
// pause between missions
pub const LDS_MKEEP: _level_type = 6;
pub type LEVEL_TYPE = UDWORD;
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
pub type WIDGET = _widget;
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
/*
 * WidgInt.h
 *
 * Internal widget library definitions
 */
/* Control whether to use malloc for widgets */
/* Control whether the internal widget string heap should be used */
/* Context information to pass into the widget functions */
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
pub type W_CLICKFORM = _w_clickform;
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
pub type WIDGET_AUDIOCALLBACK
    =
    Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type W_LABINIT = _w_labinit;
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
/*
 * Label.h
 *
 * Definitions for the label widget.
 */
/* The widget heaps */
// label states.
// label is hilited
pub type W_LABEL = _w_label;
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
/* The common widget data */
// The current button state
// Text on the label
//	PROP_FONT	*psFont;				// Font for the label
// The tool tip for the button
// mapdata for the start of a campaign
pub const LDS_CAMCHANGE: _level_type = 3;
// the data set for a campaign (no map data)
pub const LDS_CAMSTART: _level_type = 2;
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
pub type _trigger_type = libc::c_uint;
pub const TR_CALLBACKSTART: _trigger_type = 5;
pub const TR_PAUSE: _trigger_type = 4;
pub const TR_EVERY: _trigger_type = 3;
pub const TR_WAIT: _trigger_type = 2;
pub const TR_CODE: _trigger_type = 1;
pub const TR_INIT: _trigger_type = 0;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_2 = 282;
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub type W_FORMINIT = _w_forminit;
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
pub type TAB_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: UDWORD, _: BOOL, _: BOOL, _: UDWORD,
                                _: UDWORD, _: UDWORD, _: UDWORD) -> ()>;
pub const IMAGE_TRANSETA_UP: C2RustUnnamed_1 = 322;
pub const IMAGE_TRANSETA_DOWN: C2RustUnnamed_1 = 480;
pub const STR_INT_TRANSPORTER: _fixed_str_id = 50;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_2 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_2 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_2 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_2 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_2 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_2 = 139;
pub const IMAGE_MISSION_CLOCK_UP: C2RustUnnamed_1 = 462;
pub const IMAGE_MISSION_CLOCK: C2RustUnnamed_1 = 320;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
// off map mission (extra map data)
pub const LDS_EXPAND_LIMBO: _level_type = 8;
// data for changing between levels
pub const LDS_EXPAND: _level_type = 4;
// extra data for expanding a campaign map
pub const LDS_BETWEEN: _level_type = 5;
pub type LOOP_MISSION_STATE = libc::c_uint;
pub const LMS_CLEAROBJECTS: LOOP_MISSION_STATE = 5;
pub const LMS_LOADGAME: LOOP_MISSION_STATE = 4;
pub const LMS_NEWLEVEL: LOOP_MISSION_STATE = 3;
pub const LMS_SAVECONTINUE: LOOP_MISSION_STATE = 2;
pub const LMS_SETUPMISSION: LOOP_MISSION_STATE = 1;
pub const LMS_NORMAL: LOOP_MISSION_STATE = 0;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_2 = 137;
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
pub type W_BUTINIT = _w_butinit;
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
pub const STR_MR_QUIT_TO_MAIN: _fixed_str_id = 254;
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
pub const STR_MR_LOAD_GAME: _fixed_str_id = 253;
pub const STR_MR_SAVE_GAME: _fixed_str_id = 252;
pub const STR_MR_CONTINUE: _fixed_str_id = 255;
pub const STR_MR_OBJECTIVE_FAILED: _fixed_str_id = 251;
pub const STR_MR_OBJECTIVE_ACHIEVED: _fixed_str_id = 250;
pub type SCREENTYPE = _screenType;
pub type _screenType = libc::c_uint;
pub const SCREEN_COVERMOUNT: _screenType = 8;
pub const SCREEN_SLIDE5: _screenType = 7;
pub const SCREEN_SLIDE4: _screenType = 6;
pub const SCREEN_SLIDE3: _screenType = 5;
pub const SCREEN_SLIDE2: _screenType = 4;
pub const SCREEN_SLIDE1: _screenType = 3;
pub const SCREEN_MISSIONEND: _screenType = 2;
pub const SCREEN_CREDITS: _screenType = 1;
pub const SCREEN_RANDOMBDROP: _screenType = 0;
/* **************************************************************************/
/* 
 * loadsave.h
 */
/* **************************************************************************/
/* **************************************************************************/
/*
 *	Global Definitions
 */
/* **************************************************************************/
pub type LOADSAVE_MODE = _loadsave_mode;
pub type _loadsave_mode = libc::c_uint;
pub const SAVE_FORCE: _loadsave_mode = 6;
pub const LOAD_FORCE: _loadsave_mode = 5;
pub const SAVE_INGAME: _loadsave_mode = 4;
pub const LOAD_INGAME: _loadsave_mode = 3;
pub const SAVE_MISSIONEND: _loadsave_mode = 2;
pub const LOAD_MISSIONEND: _loadsave_mode = 1;
pub const LOAD_FRONTEND: _loadsave_mode = 0;
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAME_SAVED: _fixed_str_id = 298;
// Stand alone mission.
pub const GTYPE_SAVE_START: C2RustUnnamed_0 = 3;
pub const IMAGE_CURSOR_DEFAULT: C2RustUnnamed_1 = 232;
pub const CALL_TRANSPORTER_LANDED: _scr_callback_types = 50;
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
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
pub type LAND_LIGHT_SPEC = libc::c_uint;
pub const LL_OUTER: LAND_LIGHT_SPEC = 2;
pub const LL_INNER: LAND_LIGHT_SPEC = 1;
pub const LL_MIDDLE: LAND_LIGHT_SPEC = 0;
pub const CALL_NO_REINFORCEMENTS_LEFT: _scr_callback_types = 52;
pub const CALL_TRANSPORTER_OFFMAP: _scr_callback_types = 49;
pub type C2RustUnnamed_0 = libc::c_uint;
// User saved game - in the middle of a level
// User saved game - at the start of a level.
pub const GTYPE_SAVE_MIDMISSION: C2RustUnnamed_0 = 4;
// Scenario scroll area expansion.
pub const GTYPE_MISSION: C2RustUnnamed_0 = 2;
// Initial scenario state.
pub const GTYPE_SCENARIO_EXPAND: C2RustUnnamed_0 = 1;
pub const GTYPE_SCENARIO_START: C2RustUnnamed_0 = 0;
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
/* **********************************************/
/* Image ID definition file created by Framer. */
/* **********************************************/
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IMAGE_ASCII64: C2RustUnnamed_1 = 483;
pub const IMAGE_ORD_FIREDES_DOWN: C2RustUnnamed_1 = 482;
pub const IMAGE_ORD_FIREDES_UP: C2RustUnnamed_1 = 481;
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
pub const IMAGE_CURSOR_DEST: C2RustUnnamed_1 = 348;
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
pub const IMAGE_LAUNCHDOWN: C2RustUnnamed_1 = 321;
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
pub const IMAGE_CURSOR_NOTPOS: C2RustUnnamed_1 = 242;
pub const IMAGE_CURSOR_PICKUP: C2RustUnnamed_1 = 241;
pub const IMAGE_CURSOR_REPAIR: C2RustUnnamed_1 = 240;
pub const IMAGE_CURSOR_FIX: C2RustUnnamed_1 = 239;
pub const IMAGE_CURSOR_ATTACH: C2RustUnnamed_1 = 238;
pub const IMAGE_CURSOR_BRIDGE: C2RustUnnamed_1 = 237;
pub const IMAGE_CURSOR_EMBARK: C2RustUnnamed_1 = 236;
pub const IMAGE_CURSOR_GUARD: C2RustUnnamed_1 = 235;
pub const IMAGE_CURSOR_MOVE: C2RustUnnamed_1 = 234;
pub const IMAGE_CURSOR_BUILD: C2RustUnnamed_1 = 233;
pub const IMAGE_CURSOR_ECM: C2RustUnnamed_1 = 231;
pub const IMAGE_CURSOR_LOCKON: C2RustUnnamed_1 = 230;
pub const IMAGE_CURSOR_SELECT: C2RustUnnamed_1 = 229;
pub const IMAGE_CURSOR_ATTACK: C2RustUnnamed_1 = 228;
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
pub const CALL_ALL_ONSCREEN_DROIDS_SELECTED: _scr_callback_types = 51;
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
pub type C2RustUnnamed_2 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_2 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_2 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_2 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_2 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_2 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_2 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_2 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_2 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_2 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_2 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_2 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_2 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_2 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_2 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_2 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_2 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_2 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_2 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_2 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_2 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_2 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_2 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_2 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_2 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_2 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_2 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_2 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_2 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_2 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_2 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_2 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_2 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_2 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_2 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_2 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_2 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_2 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_2 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_2 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_2 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_2 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_2 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_2 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_2 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_2 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_2 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_2 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_2 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_2 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_2 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_2 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_2 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_2 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_2 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_2 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_2 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_2 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_2 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_2 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_2 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_2 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_2 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_2 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_2 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_2 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_2 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_2 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_2 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_2 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_2 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_2 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_2 = 283;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_2 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_2 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_2 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_2 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_2 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_2 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_2 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_2 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_2 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_2 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_2 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_2 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_2 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_2 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_2 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_2 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_2 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_2 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_2 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_2 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_2 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_2 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_2 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_2 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_2 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_2 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_2 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_2 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_2 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_2 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_2 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_2 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_2 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_2 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_2 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_2 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_2 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_2 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_2 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_2 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_2 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_2 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_2 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_2 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_2 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_2 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_2 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_2 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_2 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_2 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_2 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_2 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_2 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_2 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_2 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_2 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_2 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_2 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_2 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_2 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_2 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_2 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_2 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_2 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_2 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_2 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_2 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_2 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_2 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_2 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_2 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_2 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_2 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_2 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_2 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_2 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_2 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_2 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_2 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_2 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_2 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_2 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_2 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_2 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_2 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_2 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_2 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_2 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_2 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_2 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_2 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_2 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_2 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_2 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_2 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_2 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_2 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_2 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_2 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_2 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_2 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_2 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_2 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_2 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_2 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_2 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_2 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_2 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_2 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_2 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_2 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_2 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_2 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_2 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_2 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_2 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_2 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_2 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_2 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_2 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_2 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_2 = 160;
pub const ID_GIFT: C2RustUnnamed_2 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_2 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_2 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_2 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_2 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_2 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_2 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_2 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_2 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_2 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_2 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_2 = 145;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_2 = 138;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_2 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_2 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_2 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_2 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_2 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_2 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_2 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_2 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_2 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_2 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_2 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_2 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_2 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_2 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_2 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_2 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_2 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_2 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_2 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_2 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_2 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_2 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_2 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_2 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_2 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_2 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_2 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_2 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_2 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_2 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_2 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_2 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_2 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_2 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_2 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_2 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_2 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_2 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_2 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_2 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_2 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_2 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_2 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_2 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_2 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_2 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_2 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_2 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_2 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_2 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_2 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_2 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_2 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_2 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_2 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_2 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_2 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_2 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_2 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_2 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_2 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_2 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_2 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_2 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_2 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_2 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_2 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_2 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_2 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_2 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_2 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_2 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_2 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_2 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_2 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_2 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_2 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_2 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_2 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_2 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_2 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_2 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_2 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_2 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_2 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_2 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_2 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_2 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_2 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_2 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_2 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_2 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_2 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_2 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_2 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_2 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_2 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_2 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_2 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_2 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_2 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_2 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_2 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_2 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_2 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_2 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_2 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_2 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_2 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_2 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_2 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_2 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_2 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_2 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_2 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_2 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_2 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_2 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_2 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_2 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_2 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_2 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_2 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_2 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_2 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_2 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_2 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_2 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_2 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_2 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_2 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_2 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_2 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_2 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_2 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_2 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_2 = 0;
pub const NO_SOUND: C2RustUnnamed_2 = -1;
/*
 * Levels.h
 *
 * Control the data loading for game levels
 *
 */
// maximum number of WRF/WDG files
// types of level datasets
pub type _level_type = libc::c_uint;
//flags when not got a mission to go back to or when 
//already on one - ****LEAVE AS LAST ONE****
// all data required for a stand alone level
pub const LDS_CAMPAIGN: _level_type = 1;
pub const LDS_COMPLETE: _level_type = 0;
#[no_mangle]
pub static mut mission: MISSION =
    MISSION{type_0: 0,
            psMapTiles: 0 as *const MAPTILE as *mut MAPTILE,
            aMapLinePoints: 0 as *const TILE_COORD as *mut TILE_COORD,
            mapWidth: 0,
            mapHeight: 0,
            psGateways: 0 as *const _gateway as *mut _gateway,
            apRLEZones: 0 as *const *mut UBYTE as *mut *mut UBYTE,
            gwNumZones: 0,
            aNumEquiv: 0 as *const UBYTE as *mut UBYTE,
            apEquivZones: 0 as *const *mut UBYTE as *mut *mut UBYTE,
            aZoneReachable: 0 as *const UBYTE as *mut UBYTE,
            scrollMinX: 0,
            scrollMinY: 0,
            scrollMaxX: 0,
            scrollMaxY: 0,
            apsStructLists: [0 as *const STRUCTURE as *mut STRUCTURE; 8],
            apsDroidLists: [0 as *const DROID as *mut DROID; 8],
            apsFeatureLists: [0 as *const FEATURE as *mut FEATURE; 8],
            apsFlagPosLists:
                [0 as *const FLAG_POSITION as *mut FLAG_POSITION; 8],
            asPower:
                [PLAYER_POWER{currentPower: 0,
                              extractedPower: 0,
                              psLastPowered:
                                  0 as *const _base_object as
                                      *mut _base_object,}; 8],
            startTime: 0,
            time: 0,
            ETA: 0,
            cheatTime: 0,
            homeLZ_X: 0,
            homeLZ_Y: 0,
            playerX: 0,
            playerY: 0,
            iTranspEntryTileX: [0; 8],
            iTranspEntryTileY: [0; 8],
            iTranspExitTileX: [0; 8],
            iTranspExitTileY: [0; 8],};
#[no_mangle]
pub static mut offWorldKeepLists: BOOL = 0;
// Set by scrFlyInTransporter. True if were currenly tracking the transporter.
#[no_mangle]
pub static mut bTrackingTransporter: BOOL = 0 as libc::c_int;
/*lists of droids that are held seperate over several missions. There should
only be selectedPlayer's droids but have possibility for MAX_PLAYERS -
also saves writing out list functions to cater for just one player*/
#[no_mangle]
pub static mut apsLimboDroids: [*mut DROID; 8] =
    [0 as *const DROID as *mut DROID; 8];
/* *********TEST************/
//static  UDWORD      addCount = 0;
//STATICS***************
//Where the Transporter lands for player 0 (sLandingZone[0]), and the rest are
//a list of areas that cannot be built on, used for landing the enemy transporters
static mut sLandingZone: [LANDING_ZONE; 9] =
    [LANDING_ZONE{x1: 0, y1: 0, x2: 0, y2: 0,}; 9];
//flag to indicate when the droids in a Transporter are flown to safety and not the next mission
static mut bDroidsToSafety: BOOL = 0;
/* mission result holder */
static mut g_bMissionResult: BOOL = 0;
// return positions for vtols
#[no_mangle]
pub static mut asVTOLReturnPos: [POINT; 8] = [POINT{x: 0, y: 0,}; 8];
static mut missionCountDown: UBYTE = 0;
//flag to indicate whether the coded mission countdown is played
static mut bPlayCountDown: UBYTE = 0;
//void intRemoveMissionResult			(void);
//void intRemoveMissionResultNoAnim	(void);
//void intRunMissionResult			(void);
//void intProcessMissionResult		(UDWORD id);
#[no_mangle]
pub static mut MissionResUp: BOOL = 0 as libc::c_int;
#[no_mangle]
pub static mut ClosingMissionRes: BOOL = 0 as libc::c_int;
static mut g_iReinforceTime: SDWORD = 0 as libc::c_int;
//static DROID_GROUP	*g_CurrentScriptGroup = NULL;
/* Which campaign are we dealing with? */
static mut camNumber: UDWORD = 1 as libc::c_int as UDWORD;
//static iSprite *pMissionBackDrop; //pointer to backdrop piccy
//returns TRUE if on an off world mission
#[no_mangle]
pub unsafe extern "C" fn missionIsOffworld() -> BOOL {
    return (mission.type_0 == LDS_MKEEP as libc::c_int as libc::c_uint ||
                mission.type_0 == LDS_MCLEAR as libc::c_int as libc::c_uint ||
                mission.type_0 ==
                    LDS_MKEEP_LIMBO as libc::c_int as libc::c_uint) as
               libc::c_int;
}
//returns TRUE if the correct type of mission for reinforcements
#[no_mangle]
pub unsafe extern "C" fn missionForReInforcements() -> BOOL {
    if mission.type_0 == LDS_CAMSTART as libc::c_int as libc::c_uint ||
           missionIsOffworld() != 0 ||
           mission.type_0 == LDS_CAMCHANGE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
//returns TRUE if the correct type of mission and a reinforcement time has been set
#[no_mangle]
pub unsafe extern "C" fn missionCanReEnforce() -> BOOL {
    //	return missionIsOffworld() && (mission.ETA >= 0);
    if mission.ETA >= 0 as libc::c_int {
        //added CAMSTART for when load up a save game on Cam2A - AB 17/12/98
        //if (missionIsOffworld() OR (mission.type == LDS_CAMCHANGE) OR
        //    (mission.type == LDS_CAMSTART))
        if missionForReInforcements() != 0 { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
//returns TRUE if the mission is a Limbo Expand mission
#[no_mangle]
pub unsafe extern "C" fn missionLimboExpand() -> BOOL {
    if mission.type_0 == LDS_EXPAND_LIMBO as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
//mission initialisation game code
#[no_mangle]
pub unsafe extern "C" fn initMission() {
    let mut inc: UDWORD = 0;
    debug(LOG_NEVER,
          b"***Init Mission ***\n\x00" as *const u8 as *const libc::c_char);
    //mission.type = MISSION_NONE;
    mission.type_0 = LDS_NONE as libc::c_int as UDWORD;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        mission.apsStructLists[inc as usize] = 0 as *mut STRUCTURE;
        mission.apsDroidLists[inc as usize] = 0 as *mut DROID;
        mission.apsFeatureLists[inc as usize] = 0 as *mut FEATURE;
        //mission.apsProxDisp[inc] = NULL;
        mission.apsFlagPosLists[inc as usize] = 0 as *mut FLAG_POSITION;
        apsLimboDroids[inc as usize] = 0 as *mut DROID;
        inc = inc.wrapping_add(1)
    }
    offWorldKeepLists = 0 as libc::c_int;
    mission.time = -(1 as libc::c_int);
    // ffs ab
    setMissionCountDown();
    mission.ETA = -(1 as libc::c_int);
    mission.startTime = 0 as libc::c_int as UDWORD;
    mission.psGateways = 0 as *mut _gateway;
    mission.apRLEZones = 0 as *mut *mut UBYTE;
    mission.gwNumZones = 0 as libc::c_int;
    mission.mapHeight = 0 as libc::c_int as UDWORD;
    mission.mapWidth = 0 as libc::c_int as UDWORD;
    mission.aNumEquiv = 0 as *mut UBYTE;
    mission.apEquivZones = 0 as *mut *mut UBYTE;
    mission.aZoneReachable = 0 as *mut UBYTE;
    //init all the landing zones
    inc = 0 as libc::c_int as UDWORD;
    while inc < 9 as libc::c_int as libc::c_uint {
        sLandingZone[inc as usize].y2 = 0 as libc::c_int as UBYTE;
        sLandingZone[inc as usize].x2 = sLandingZone[inc as usize].y2;
        sLandingZone[inc as usize].y1 = sLandingZone[inc as usize].x2;
        sLandingZone[inc as usize].x1 = sLandingZone[inc as usize].y1;
        inc = inc.wrapping_add(1)
    }
    // init the vtol return pos
    memset(asVTOLReturnPos.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<POINT>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_uint));
    bDroidsToSafety = 0 as libc::c_int;
    setPlayCountDown(1 as libc::c_int as UBYTE);
    //start as not cheating!
    mission.cheatTime = 0 as libc::c_int as UDWORD;
}
// reset the vtol landing pos
#[no_mangle]
pub unsafe extern "C" fn resetVTOLLandingPos() {
    memset(asVTOLReturnPos.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (::std::mem::size_of::<POINT>() as
                libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                libc::c_uint));
}
//this is called everytime the game is quit
#[no_mangle]
pub unsafe extern "C" fn releaseMission() {
    /*mission.apsDroidLists may contain some droids that have been transferred
    from one campaign to the next*/
    freeAllMissionDroids();
    /*apsLimboDroids may contain some droids that have been saved
    at the end of one mission and not yet used*/
    freeAllLimboDroids();
}
//called to shut down when mid-mission on an offWorld map
#[no_mangle]
pub unsafe extern "C" fn missionShutDown() -> BOOL {
    let mut inc: UDWORD = 0;
    //if (mission.type == MISSION_OFFKEEP OR mission.type == MISSION_OFFCLEAR)
    if missionIsOffworld() != 0 {
        //mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR)
        //clear out the audio
        audio_StopAll();
        freeAllDroids();
        freeAllStructs();
        freeAllFeatures();
        releaseAllProxDisp();
        gwShutDown();
        inc = 0 as libc::c_int as UDWORD;
        while inc < 8 as libc::c_int as libc::c_uint {
            apsDroidLists[inc as usize] = mission.apsDroidLists[inc as usize];
            mission.apsDroidLists[inc as usize] = 0 as *mut DROID;
            apsStructLists[inc as usize] =
                mission.apsStructLists[inc as usize];
            mission.apsStructLists[inc as usize] = 0 as *mut STRUCTURE;
            apsFeatureLists[inc as usize] =
                mission.apsFeatureLists[inc as usize];
            mission.apsFeatureLists[inc as usize] = 0 as *mut FEATURE;
            //flag positions go with structs
//		FREE(psMapTiles);
//		mapFreeTilesAndStrips();
		//FREE(aMapLinePoints);
            //apsProxDisp[inc] = mission.apsProxDisp[inc];
            apsFlagPosLists[inc as usize] =
                mission.apsFlagPosLists[inc as usize];
            mission.apsFlagPosLists[inc as usize] = 0 as *mut FLAG_POSITION;
            inc = inc.wrapping_add(1)
        }
        psMapTiles = mission.psMapTiles;
        aMapLinePoints = mission.aMapLinePoints;
        mapWidth = mission.mapWidth;
        mapHeight = mission.mapHeight;
        psGateways = mission.psGateways;
        apRLEZones = mission.apRLEZones;
        gwNumZones = mission.gwNumZones;
        aNumEquiv = mission.aNumEquiv;
        apEquivZones = mission.apEquivZones;
        aZoneReachable = mission.aZoneReachable
    }
    // sorry if this breaks something - but it looks like it's what should happen - John
    mission.type_0 = LDS_NONE as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/*on the PC - sets the countdown played flag*/
#[no_mangle]
pub unsafe extern "C" fn setMissionCountDown() {
    let mut timeRemaining: SDWORD = 0;
    timeRemaining =
        (mission.time as
             libc::c_uint).wrapping_sub(gameTime.wrapping_sub(mission.startTime))
            as SDWORD;
    if timeRemaining < 0 as libc::c_int { timeRemaining = 0 as libc::c_int }
    //need to init the countdown played each time the mission time is changed
    missionCountDown =
        (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int |
             0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int)
            as UBYTE;
    if timeRemaining <
           10 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int -
               1 as libc::c_int {
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x10 as libc::c_int)) as
                UBYTE
    }
    if timeRemaining <
           5 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int -
               1 as libc::c_int {
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x8 as libc::c_int)) as UBYTE
    }
    if timeRemaining <
           3 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int -
               1 as libc::c_int {
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x4 as libc::c_int)) as UBYTE
    }
    if timeRemaining <
           2 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int -
               1 as libc::c_int {
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x2 as libc::c_int)) as UBYTE
    }
    if timeRemaining <
           1 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int -
               1 as libc::c_int {
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x1 as libc::c_int)) as UBYTE
    };
}
//BOOL startMission(MISSION_TYPE missionType, STRING *pGame)
#[no_mangle]
pub unsafe extern "C" fn startMission(mut missionType: LEVEL_TYPE,
                                      mut pGame: *mut STRING) -> BOOL {
    let mut loaded: BOOL = 1 as libc::c_int;
    /* Player has (obviously) not failed at the start */
    setPlayerHasLost(0 as libc::c_int);
    setPlayerHasWon(0 as libc::c_int);
    /*and win/lose video is obvioulsy not playing*/
    setScriptWinLoseVideo(0 as libc::c_int as UBYTE);
    //HACKY HACK HACK
    //this inits the flag so that 'reinforcements have arrived' message isn't played for the first transporter load
    initFirstTransporterFlag();
    //	if (missionType == LDS_CAMSTART)
//	{
		//only load up one campaign start
//		if (!DemoStart)
//		{
//			DBERROR(("Unable to load mission"));
//			return FALSE;
//		}
//		DemoStart = FALSE;
//	}
//	else if (missionType == LDS_MKEEP)
//	{
		//only load up one other mission
//		if (!DemoExpand)
//		{
//			DBERROR(("Unable to load mission"));
//			return FALSE;
//		}
//		DemoExpand = FALSE;
//	}
//	else if(missionType == LDS_BETWEEN)
//	{
//		// do nothing.
//	}
//	else
//	{
//		//don't want to load up any other type of mission
//		DBERROR(("Unable to load mission"));
//		return FALSE;
//	}
    //if (mission.type != MISSION_NONE)
    if mission.type_0 != LDS_NONE as libc::c_int as libc::c_uint {
        /*mission type gets set to none when you have returned from a mission
		so don't want to go another mission when already on one! - so ignore*/
        debug(LOG_NEVER,
              b"Already on a mission\x00" as *const u8 as
                  *const libc::c_char);
        return 1 as libc::c_int
    }
    // reset the cluster stuff
    clustInitialise();
    initEffectsSystem();
    //load the game file for all types of mission except a Between Mission
	//if (missionType != MISSION_BETWEEN)
    if missionType != LDS_BETWEEN as libc::c_int as libc::c_uint {
        loadGameInit(pGame);
    }
    //all proximity messages are removed between missions now
    releaseAllProxDisp();
    match missionType {
        2 => {
            if startMissionCampaignStart(pGame) == 0 {
                loaded = 0 as libc::c_int
            }
        }
        6 | 9 => {
            if startMissionOffKeep(pGame) == 0 { loaded = 0 as libc::c_int }
        }
        5 => {
            //do anything?
            if startMissionBetween() == 0 { loaded = 0 as libc::c_int }
        }
        3 => {
            /*if (getCampaignNumber() == 1)
			{
				//play the cam 2 video
				seq_ClearSeqList();
			#ifndef PSX
				seq_AddSeqToList("CAM2\\c002.rpl",NULL,"CAM2\\c002.txa",FALSE);
			#else
				seq_AddSeqToList("CAM2\\C002.STR","1656f");
			#endif
				seq_StartNextFullScreenVideo();
			}
			else
			{
				//play the cam 3 video
				seq_ClearSeqList();
			#ifndef PSX
				seq_AddSeqToList("CAM2\\cam2out.rpl",NULL,NULL,FALSE);
				seq_AddSeqToList("CAM3\\c003.rpl",NULL,"CAM3\\c003.txa",FALSE);
			#else
				seq_AddSeqToList("CAM3\\C003.STR","1656f");
			#endif
				seq_StartNextFullScreenVideo();
			}*/
            if startMissionCampaignChange(pGame) == 0 {
                loaded = 0 as libc::c_int
            }
        }
        4 => {
            if startMissionCampaignExpand(pGame) == 0 {
                loaded = 0 as libc::c_int
            }
        }
        8 => {
            if startMissionCampaignExpandLimbo(pGame) == 0 {
                loaded = 0 as libc::c_int
            }
        }
        7 => {
            if startMissionOffClear(pGame) == 0 { loaded = 0 as libc::c_int }
        }
        _ => {
            //error!
            debug(LOG_ERROR,
                  b"Unknown Mission Type\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    if loaded == 0 {
        debug(LOG_ERROR,
              b"Unable to load mission file\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    mission.type_0 = missionType;
    if missionIsOffworld() != 0 {
        //add what power have got from the home base
        adjustMissionPower();
    }
    if missionCanReEnforce() != 0 {
        //add mission timer - if necessary
        addMissionTimerInterface();
        //add Transporter Timer
        addTransporterTimerInterface();
    }
    scoreInitSystem();
    //add proximity messages for all untapped VISIBLE oil resources
    addOilResourceProximities();
    return 1 as libc::c_int;
}
// initialise the mission stuff for a save game
#[no_mangle]
pub unsafe extern "C" fn startMissionSave(mut missionType: SDWORD) -> BOOL {
    mission.type_0 = missionType as UDWORD;
    return 1 as libc::c_int;
}
/*checks the time has been set and then adds the timer if not already on
the display*/
#[no_mangle]
pub unsafe extern "C" fn addMissionTimerInterface() {
    //don't add if the timer hasn't been set
    if mission.time < 0 as libc::c_int { return }
    //check timer is not already on the screen
    if widgGetFromID(psWScreen, 11000 as libc::c_int as UDWORD).is_null() {
        intAddMissionTimer();
    };
}
/*checks that the timer has been set and that a Transporter exists before
adding the timer button*/
#[no_mangle]
pub unsafe extern "C" fn addTransporterTimerInterface() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut bAddInterface: BOOL = 0 as libc::c_int;
    let mut psForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    //check if reinforcements are allowed
    if mission.ETA >= 0 as libc::c_int {
        //check the player has at least one Transporter back at base
        psTransporter = 0 as *mut DROID;
        psDroid = psTransporter;
        psDroid = mission.apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                psTransporter = psDroid;
                break ;
            } else { psDroid = (*psDroid).psNext }
        }
        if !psDroid.is_null() {
            //don't bother checking for reinforcements - always add it if you've got a Transporter
            //check the player has some reinforcements back at home
            /*psDroid = NULL;
            for (psDroid = mission.apsDroidLists[selectedPlayer]; psDroid !=
                NULL; psDroid = psDroid->psNext)
            {
                if (psDroid->droidType != DROID_TRANSPORTER)
                {
                    break;
                }
            }
            //if reinforcements available
            if (psDroid)*/
            bAddInterface = 1 as libc::c_int;
            //check timer is not already on the screen
            if widgGetFromID(psWScreen,
                             11012 as libc::c_int as UDWORD).is_null() {
                intAddTransporterTimer();
            }
            //set the data for the transporter timer
            widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD,
                            psTransporter as *mut libc::c_void);
            //lock the button if necessary
            if transporterFlying(psTransporter) != 0 {
                //disable the form so can't add any more droids into the transporter
                psForm =
                    widgGetFromID(psWScreen, 11012 as libc::c_int as UDWORD)
                        as *mut W_CLICKFORM;
                if !psForm.is_null() {
                    formSetClickState(psForm, 0x2 as libc::c_int as UDWORD);
                }
            }
        }
    }
    //if criteria not met
    if bAddInterface == 0 {
        //make sure its not there!
        intRemoveTransporterTimer();
    };
}
/* get offscreen point */
#[no_mangle]
pub unsafe extern "C" fn missionGetOffScreenPoint(mut iX: UWORD,
                                                  mut iY: UWORD,
                                                  mut piOffX: *mut UWORD,
                                                  mut piOffY: *mut UWORD,
                                                  mut piOffZ: *mut UWORD) {
    //	UDWORD	iMapWidth  = GetWidthOfMap()  << TILE_SHIFT,
//			iMapHeight = GetHeightOfMap() << TILE_SHIFT;
    let mut iTestX: SDWORD = iX as SDWORD - 600 as libc::c_int;
    let mut iTestY: SDWORD = iY as SDWORD - 600 as libc::c_int;
    if iTestX > 0 as libc::c_int {
        *piOffX = iTestX as UWORD
    } else { *piOffX = (iX as SDWORD - 600 as libc::c_int) as UWORD }
    if iTestY > 0 as libc::c_int {
        *piOffY = iTestY as UWORD
    } else { *piOffY = (iY as SDWORD - 600 as libc::c_int) as UWORD }
    *piOffZ =
        (map_Height(iX as UDWORD, iY as UDWORD) as libc::c_int +
             600 as libc::c_int) as UWORD;
}
/* pick nearest map edge to point */
#[no_mangle]
pub unsafe extern "C" fn missionGetNearestCorner(mut iX: UWORD, mut iY: UWORD,
                                                 mut piOffX: *mut UWORD,
                                                 mut piOffY: *mut UWORD) {
    let mut iMidX: UDWORD =
        ((scrollMinX + scrollMaxX) / 2 as libc::c_int) as UDWORD;
    let mut iMidY: UDWORD =
        ((scrollMinY + scrollMaxY) / 2 as libc::c_int) as UDWORD;
    if ((iX as libc::c_int >> 7 as libc::c_int) as UDWORD) < iMidX {
        *piOffX =
            ((scrollMinX << 7 as libc::c_int) +
                 1 as libc::c_int * 128 as libc::c_int) as UWORD
    } else {
        *piOffX =
            ((scrollMaxX << 7 as libc::c_int) -
                 1 as libc::c_int * 128 as libc::c_int) as UWORD
    }
    if ((iY as libc::c_int >> 7 as libc::c_int) as UDWORD) < iMidY {
        *piOffY =
            ((scrollMinY << 7 as libc::c_int) +
                 1 as libc::c_int * 128 as libc::c_int) as UWORD
    } else {
        *piOffY =
            ((scrollMaxY << 7 as libc::c_int) -
                 1 as libc::c_int * 128 as libc::c_int) as UWORD
    };
}
/* fly in transporters at start of level */
#[no_mangle]
pub unsafe extern "C" fn missionFlyTransportersIn(mut iPlayer: SDWORD,
                                                  mut bTrackTransporter:
                                                      BOOL) {
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut iX: UWORD = 0;
    let mut iY: UWORD = 0;
    let mut iZ: UWORD = 0;
    let mut iLandX: SDWORD = 0;
    let mut iLandY: SDWORD = 0;
    let mut iDx: SDWORD = 0;
    let mut iDy: SDWORD = 0;
    let mut fR: FRACT_D = 0.;
    bTrackingTransporter = bTrackTransporter;
    iLandX = getLandingX(iPlayer) as SDWORD;
    iLandY = getLandingY(iPlayer) as SDWORD;
    missionGetTransporterEntry(iPlayer, &mut iX, &mut iY);
    iZ =
        (map_Height(iX as UDWORD, iY as UDWORD) as libc::c_int +
             600 as libc::c_int) as UWORD;
    psNext = 0 as *mut DROID;
    //get the droids for the mission
    psTransporter = mission.apsDroidLists[iPlayer as usize];
    while !psTransporter.is_null() {
        psNext = (*psTransporter).psNext;
        if (*psTransporter).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            //check that this transporter actually contains some droids
            if !(*psTransporter).psGroup.is_null() &&
                   (*(*psTransporter).psGroup).refCount as libc::c_int >
                       1 as libc::c_int {
                //remove out of stored list and add to current Droid list
                if droidRemove(psTransporter,
                               mission.apsDroidLists.as_mut_ptr()) != 0 {
                    //don't want to add it unless managed to remove it from the previous list
                    addDroid(psTransporter, apsDroidLists.as_mut_ptr());
                }
                /* set start position */
                (*psTransporter).x = iX;
                (*psTransporter).y = iY;
                (*psTransporter).z = iZ;
                /* set start direction */
                iDx = iLandX - iX as libc::c_int;
                iDy = iLandY - iY as libc::c_int;
                fR =
                    atan2(iDx as libc::c_double, iDy as libc::c_double) as
                        FRACT_D;
                if (fR as libc::c_double) < 0.0f64 {
                    fR +=
                        (2 as libc::c_int as libc::c_double * 3.141592654f64)
                            as FRACT_D
                }
                (*psTransporter).direction =
                    (fR as libc::c_double * 180.0f64 / 3.141592654f64) as
                        UWORD;
                // Camera track requested and it's the selected player.
                if bTrackTransporter == 1 as libc::c_int &&
                       iPlayer == selectedPlayer as SDWORD {
                    /* deselect all droids */
                    selDroidDeselect(selectedPlayer);
                    if getWarCamStatus() != 0 { camToggleStatus(); }
                    /* select transporter */
                    (*psTransporter).selected = 1 as libc::c_int as UBYTE;
                    camToggleStatus();
                }
                //little hack to ensure all Transporters are fully repaired by time enter world
                (*psTransporter).body = (*psTransporter).originalBody;
                /* set fly-in order */
                orderDroidLoc(psTransporter, DORDER_TRANSPORTIN,
                              iLandX as UDWORD, iLandY as UDWORD);
                audio_PlayObjDynamicTrack(psTransporter as *mut libc::c_void,
                                          ID_SOUND_BLIMP_FLIGHT as
                                              libc::c_int,
                                          Some(moveCheckDroidMovingAndVisible
                                                   as
                                                   unsafe extern "C" fn(_:
                                                                            *mut AUDIO_SAMPLE)
                                                       -> BOOL));
                break ;
            }
        }
        psTransporter = psNext
    };
}
/*Saves the necessary data when moving from a home base Mission to an OffWorld
mission*/
unsafe extern "C" fn saveMissionData() {
    let mut inc: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStructBeingBuilt: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut bRepairExists: BOOL = 0;
    //clear out the audio
    audio_StopAll();
    //save the mission data
    mission.psMapTiles = psMapTiles;
    mission.aMapLinePoints = aMapLinePoints;
    mission.mapWidth = mapWidth;
    mission.mapHeight = mapHeight;
    mission.scrollMinX = scrollMinX as UDWORD;
    mission.scrollMinY = scrollMinY as UDWORD;
    mission.scrollMaxX = scrollMaxX as UDWORD;
    mission.scrollMaxY = scrollMaxY as UDWORD;
    mission.psGateways = psGateways;
    psGateways = 0 as *mut GATEWAY;
    mission.apRLEZones = apRLEZones;
    apRLEZones = 0 as *mut *mut UBYTE;
    mission.gwNumZones = gwNumZones;
    gwNumZones = 0 as libc::c_int;
    mission.aNumEquiv = aNumEquiv;
    aNumEquiv = 0 as *mut UBYTE;
    mission.apEquivZones = apEquivZones;
    apEquivZones = 0 as *mut *mut UBYTE;
    mission.aZoneReachable = aZoneReachable;
    aZoneReachable = 0 as *mut UBYTE;
    //save the selectedPlayer's LZ
    mission.homeLZ_X = getLandingX(selectedPlayer as SDWORD);
    mission.homeLZ_Y = getLandingY(selectedPlayer as SDWORD);
    bRepairExists = 0 as libc::c_int;
    //set any structures currently being built to completed for the selected player
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        if (*psStruct).status as libc::c_int == SS_BEING_BUILT as libc::c_int
           {
            //find a droid working on it
            psDroid = apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                if orderStateObj(psDroid, DORDER_BUILD,
                                 &mut psStructBeingBuilt as
                                     *mut *mut STRUCTURE as
                                     *mut *mut BASE_OBJECT) != 0 {
                    if psStructBeingBuilt == psStruct {
                        //check there is enough power to complete
                        //inc = psStruct->pStructureType->powerToBuild - psStruct->currentPowerAccrued;
                        inc =
                            structPowerToBuild(psStruct).wrapping_sub((*psStruct).currentPowerAccrued
                                                                          as
                                                                          libc::c_uint);
                        if inc > 0 as libc::c_int as libc::c_uint {
                            //not accrued enough power, so check if there is enough available
                            if checkPower(selectedPlayer, inc,
                                          0 as libc::c_int) != 0 {
                                //enough - so use it and set to complete
                                usePower(selectedPlayer, inc);
                                buildingComplete(psStruct);
                            }
                        } else {
                            //enough power or more than enough! - either way, set to complete
                            buildingComplete(psStruct);
                        }
                        break ;
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        //check if have a completed repair facility on home world
        if (*(*psStruct).pStructureType).type_0 ==
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            bRepairExists = 1 as libc::c_int
        }
        psStruct = (*psStruct).psNext
    }
    //repair all droids back at home base if have a repair facility
    if bRepairExists != 0 {
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if droidIsDamaged(psDroid) != 0 {
                (*psDroid).body = (*psDroid).originalBody
            }
            psDroid = (*psDroid).psNext
        }
    }
    //clear droid orders for all droids except constructors still building
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        if orderStateObj(psDroid, DORDER_BUILD,
                         &mut psStructBeingBuilt as *mut *mut STRUCTURE as
                             *mut *mut BASE_OBJECT) != 0 {
            if (*psStructBeingBuilt).status as libc::c_int ==
                   SS_BUILT as libc::c_int {
                orderDroid(psDroid, DORDER_STOP);
            }
        } else { orderDroid(psDroid, DORDER_STOP); }
        psDroid = (*psDroid).psNext
    }
    //THIS HAPPENS AT THE END OF THE CAMCHANGE MISSION NOW - AB 22/12/98
    //before copy the pointers over check selectedPlayer's mission.droids since
    //there might be some from the previous camapign
    //processPreviousCampDroids();
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        mission.apsStructLists[inc as usize] = apsStructLists[inc as usize];
        mission.apsDroidLists[inc as usize] = apsDroidLists[inc as usize];
        mission.apsFeatureLists[inc as usize] = apsFeatureLists[inc as usize];
        //mission.apsProxDisp[inc] = apsProxDisp[inc];
        mission.apsFlagPosLists[inc as usize] = apsFlagPosLists[inc as usize];
        inc = inc.wrapping_add(1)
    }
    mission.playerX = player.p.x;
    mission.playerY = player.p.z;
    //save the power settings
    saveMissionPower();
    //reset before loading in the new game
	//resetFactoryNumFlag();
    //init before loading in the new game
    initFactoryNumFlag();
    //clear all the effects from the map
    initEffectsSystem();
    resetRadarRedraw();
}
/*
	This routine frees the memory for the offworld mission map (in the call to mapShutdown)

	- so when this routine is called we must still be set to the offworld map data
	i.e. We shoudn't have called SwapMissionPointers()

*/
unsafe extern "C" fn restoreMissionData() {
    let mut inc: UDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    //clear out the audio
    audio_StopAll();
    //clear all the lists
	//clearPlayerPower();
    proj_FreeAllProjectiles();
    freeAllDroids();
    freeAllStructs();
    freeAllFeatures();
    gwShutDown();
    mapShutdown();
    //	FREE(psMapTiles);
//	mapFreeTilesAndStrips();
	//FREE(aMapLinePoints);
	//releaseAllProxDisp();
	//flag positions go with structs
    //restore the game pointers
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        apsDroidLists[inc as usize] = mission.apsDroidLists[inc as usize];
        mission.apsDroidLists[inc as usize] = 0 as *mut DROID;
        psObj = apsDroidLists[inc as usize] as *mut BASE_OBJECT;
        while !psObj.is_null() {
            gridAddObject(psObj);
            //make sure the died flag is not set
            (*psObj).died = 0 as libc::c_int as UDWORD;
            psObj = (*psObj).psNext
        }
        apsStructLists[inc as usize] = mission.apsStructLists[inc as usize];
        mission.apsStructLists[inc as usize] = 0 as *mut STRUCTURE;
        psObj = apsStructLists[inc as usize] as *mut BASE_OBJECT;
        while !psObj.is_null() {
            gridAddObject(psObj);
            psObj = (*psObj).psNext
        }
        apsFeatureLists[inc as usize] = mission.apsFeatureLists[inc as usize];
        mission.apsFeatureLists[inc as usize] = 0 as *mut FEATURE;
        psObj = apsFeatureLists[inc as usize] as *mut BASE_OBJECT;
        while !psObj.is_null() {
            gridAddObject(psObj);
            psObj = (*psObj).psNext
        }
        apsFlagPosLists[inc as usize] = mission.apsFlagPosLists[inc as usize];
        mission.apsFlagPosLists[inc as usize] = 0 as *mut FLAG_POSITION;
        //apsProxDisp[inc] = mission.apsProxDisp[inc];
		//mission.apsProxDisp[inc] = NULL;
		//asPower[inc]->usedPower = mission.usedPower[inc];
		//init the next structure to be powered
        (*asPower[inc as usize]).psLastPowered = 0 as *mut _base_object;
        inc = inc.wrapping_add(1)
    }
    //swap mission data over
    psMapTiles = mission.psMapTiles;
    aMapLinePoints = mission.aMapLinePoints;
    mapWidth = mission.mapWidth;
    mapHeight = mission.mapHeight;
    scrollMinX = mission.scrollMinX as SDWORD;
    scrollMinY = mission.scrollMinY as SDWORD;
    scrollMaxX = mission.scrollMaxX as SDWORD;
    scrollMaxY = mission.scrollMaxY as SDWORD;
    psGateways = mission.psGateways;
    apRLEZones = mission.apRLEZones;
    gwNumZones = mission.gwNumZones;
    aNumEquiv = mission.aNumEquiv;
    apEquivZones = mission.apEquivZones;
    aZoneReachable = mission.aZoneReachable;
    //and clear the mission pointers
    mission.psMapTiles = 0 as *mut MAPTILE;
    mission.aMapLinePoints = 0 as *mut TILE_COORD;
    mission.mapWidth = 0 as libc::c_int as UDWORD;
    mission.mapHeight = 0 as libc::c_int as UDWORD;
    mission.scrollMinX = 0 as libc::c_int as UDWORD;
    mission.scrollMinY = 0 as libc::c_int as UDWORD;
    mission.scrollMaxX = 0 as libc::c_int as UDWORD;
    mission.scrollMaxY = 0 as libc::c_int as UDWORD;
    mission.psGateways = 0 as *mut _gateway;
    mission.apRLEZones = 0 as *mut *mut UBYTE;
    mission.gwNumZones = 0 as libc::c_int;
    mission.aNumEquiv = 0 as *mut UBYTE;
    mission.apEquivZones = 0 as *mut *mut UBYTE;
    mission.aZoneReachable = 0 as *mut UBYTE;
    //reset the current structure lists
    setCurrentStructQuantity(0 as libc::c_int);
    //initPlayerPower();
    initFactoryNumFlag();
    resetFactoryNumFlag();
    //terrain types? - hopefully not! otherwise we have to load in the terrain texture pages.
    //reset the game time
	//gameTimeReset(mission.startTime);
    offWorldKeepLists = 0 as libc::c_int;
    resetRadarRedraw();
    //reset the environ map back to the homebase settings
    environReset();
    //intSetMapPos(mission.playerX, mission.playerY);
}
/*Saves the necessary data when moving from one mission to a limbo expand Mission*/
unsafe extern "C" fn saveMissionLimboData() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    //UDWORD			droidX, droidY;
	//PICKTILE		pickRes;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    //clear out the audio
    audio_StopAll();
    //before copy the pointers over check selectedPlayer's mission.droids since
    //there might be some from the previous camapign
    processPreviousCampDroids();
    //only need to keep the selectedPlayer's droid's separate
	//mission.apsDroidLists[selectedPlayer] = apsDroidLists[selectedPlayer];
    //apsDroidLists[selectedPlayer] = NULL;
    //move droids properly - does all the clean up code
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        if droidRemove(psDroid, apsDroidLists.as_mut_ptr()) != 0 {
            addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
        }
        psDroid = psNext
    }
    apsDroidLists[selectedPlayer as usize] = 0 as *mut DROID;
    //this is happening in a separate function now so can be called once the mission has started
    /*apsDroidLists[selectedPlayer] = apsLimboDroids[selectedPlayer];
    apsLimboDroids[selectedPlayer] = NULL;

    //set up location for each of the droids
    for (psDroid = apsDroidLists[selectedPlayer]; psDroid != NULL; psDroid =
        psDroid->psNext)
    {
		droidX = getLandingX(LIMBO_LANDING) >> TILE_SHIFT;
		droidY = getLandingY(LIMBO_LANDING) >> TILE_SHIFT;
		pickRes = pickHalfATile(&droidX, &droidY,LOOK_FOR_EMPTY_TILE);
		if (pickRes == NO_FREE_TILE )
		{
			ASSERT( FALSE, "saveMissionLimboData: Unable to find a free location" );
		}
		psDroid->x = (UWORD)(droidX << TILE_SHIFT);
		psDroid->y = (UWORD)(droidY << TILE_SHIFT);
		if (pickRes == HALF_FREE_TILE )
		{
			psDroid->x += TILE_UNITS;
			psDroid->y += TILE_UNITS;
		}
		psDroid->z = map_Height(psDroid->x, psDroid->y);
		updateDroidOrientation(psDroid);
		//psDroid->lastTile = mapTile(psDroid->x >> TILE_SHIFT,
		//	psDroid->y >> TILE_SHIFT);
		psDroid->selected = FALSE;
        //this is mainly for VTOLs
        psDroid->psBaseStruct = NULL;
		psDroid->cluster = 0;
		//initialise the movement data
		initDroidMovement(psDroid);
        //make sure the died flag is not set
        psDroid->died = FALSE;

    }*/
    //any selectedPlayer's factories/research need to be put on holdProduction/holdresearch
    psStruct = apsStructLists[selectedPlayer as usize];
    while !psStruct.is_null() {
        if StructIsFactory(psStruct) != 0 {
            holdProduction(psStruct);
        } else if (*(*psStruct).pStructureType).type_0 ==
                      REF_RESEARCH as libc::c_int as libc::c_uint {
            holdResearch(psStruct);
        }
        psStruct = (*psStruct).psNext
    };
}
//this is called via a script function to place the Limbo droids once the mission has started
#[no_mangle]
pub unsafe extern "C" fn placeLimboDroids() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    let mut pickRes: PICKTILE = NO_FREE_TILE;
    //copy the droids across for the selected Player
    psDroid = apsLimboDroids[selectedPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        if droidRemove(psDroid, apsLimboDroids.as_mut_ptr()) != 0 {
            addDroid(psDroid, apsDroidLists.as_mut_ptr());
            //KILL OFF TRANSPORTER - should never be one but....
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                vanishDroid(psDroid);
            } else {
                //set up location for each of the droids
                droidX =
                    (getLandingX(8 as libc::c_int) as libc::c_int >>
                         7 as libc::c_int) as UDWORD;
                droidY =
                    (getLandingY(8 as libc::c_int) as libc::c_int >>
                         7 as libc::c_int) as UDWORD;
                pickRes =
                    pickHalfATile(&mut droidX, &mut droidY,
                                  20 as libc::c_int as UBYTE);
                if pickRes as libc::c_uint ==
                       NO_FREE_TILE as libc::c_int as libc::c_uint {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"placeLimboUnits: Unable to find a free location\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"mission.c\x00" as *const u8 as
                                  *const libc::c_char, 1289 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 17],
                                                        &[libc::c_char; 17]>(b"placeLimboDroids\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
                (*psDroid).x = (droidX << 7 as libc::c_int) as UWORD;
                (*psDroid).y = (droidY << 7 as libc::c_int) as UWORD;
                if pickRes as libc::c_uint ==
                       HALF_FREE_TILE as libc::c_int as libc::c_uint {
                    (*psDroid).x =
                        ((*psDroid).x as libc::c_int + 128 as libc::c_int) as
                            UWORD;
                    (*psDroid).y =
                        ((*psDroid).y as libc::c_int + 128 as libc::c_int) as
                            UWORD
                }
                (*psDroid).z =
                    map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD)
                        as UWORD;
                updateDroidOrientation(psDroid);
                //psDroid->lastTile = mapTile(psDroid->x >> TILE_SHIFT,
		    //	psDroid->y >> TILE_SHIFT);
                (*psDroid).selected = 0 as libc::c_int as UBYTE;
                //this is mainly for VTOLs
                (*psDroid).psBaseStruct = 0 as *mut _structure;
                (*psDroid).cluster = 0 as libc::c_int as UBYTE;
                //initialise the movement data
                initDroidMovement(psDroid);
                //make sure the died flag is not set
                (*psDroid).died = 0 as libc::c_int as UDWORD
            }
        } else {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"placeLimboUnits: Unable to remove unit from Limbo list\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"mission.c\x00" as *const u8 as *const libc::c_char,
                      1313 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"placeLimboDroids\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        psDroid = psNext
    };
}
/*restores the necessary data on completion of a Limbo Expand mission*/
unsafe extern "C" fn restoreMissionLimboData() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    /*the droids stored in the mission droid list need to be added back
    into the current droid list*/
    psDroid = mission.apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        //remove out of stored list and add to current Droid list
        if droidRemove(psDroid, mission.apsDroidLists.as_mut_ptr()) != 0 {
            addDroid(psDroid, apsDroidLists.as_mut_ptr());
            (*psDroid).cluster = 0 as libc::c_int as UBYTE;
            gridAddObject(psDroid as *mut BASE_OBJECT);
            //the location of the droid should be valid!
            orderDroid(psDroid, DORDER_STOP);
        }
        psDroid = psNext
    }
    if mission.apsDroidLists[selectedPlayer as usize].is_null() {
    } else {
        debug(LOG_ERROR,
              b"restoreMissionLimboData: list should be empty\x00" as
                  *const u8 as *const libc::c_char);
    };
    if mission.apsDroidLists[selectedPlayer as usize].is_null() {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              1341 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 24],
                                        &[libc::c_char; 24]>(b"restoreMissionLimboData\x00")).as_ptr(),
              b"mission.apsDroidLists[selectedPlayer] == NULL\x00" as
                  *const u8 as *const libc::c_char);
    };
}
//reset droid orders
/*Saves the necessary data when moving from one campaign to the start of the
next - saves out the list of droids for the selected player*/
unsafe extern "C" fn saveCampaignData() {
    let mut inc: UBYTE = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psSafeDroid: *mut DROID = 0 as *mut DROID;
    let mut psNextSafe: *mut DROID = 0 as *mut DROID;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psCurrNext: *mut DROID = 0 as *mut DROID;
    //if the droids have been moved to safety then get any Transporters that exist
    if getDroidsToSafetyFlag() != 0 {
        //move any Transporters into the mission list
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            psNext = (*psDroid).psNext;
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                /*Now - we want to empty the transporter and make it turn up
                with the first ten out of the previous mission*/
                //we want to make sure they are full
                /*if (calcRemainingCapacity(psDroid))
                //before we move the droid into the mission list - check to see if it's empty
                //if (psDroid->psGroup AND psDroid->psGroup->refCount == 1)
                {
                    //fill it with droids from the mission list
                    for (psSafeDroid = mission.apsDroidLists[selectedPlayer];
                        psSafeDroid != NULL; psSafeDroid = psNextSafe)
                    {
                        psNextSafe = psSafeDroid->psNext;
                        //add to the Transporter, checking for when full
	                    if (checkTransporterSpace(psDroid, psSafeDroid))
	                    {
		                    if (droidRemove(psSafeDroid, mission.apsDroidLists))
                            {
    	                        grpJoin(psDroid->psGroup, psSafeDroid);
                            }
                        }
                        else
                        {
                            //setting this will cause the loop to end
                            psNextSafe = NULL;
                        }
                    }
                }*/
                //empty the transporter into the mission list
                if !(*psDroid).psGroup.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"saveCampaignData: Transporter does not have a group\x00"
                              as *const u8 as *const libc::c_char);
                };
                if !(*psDroid).psGroup.is_null() {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"mission.c\x00" as *const u8 as
                              *const libc::c_char, 1391 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"saveCampaignData\x00")).as_ptr(),
                          b"psDroid->psGroup != NULL\x00" as *const u8 as
                              *const libc::c_char);
                };
                psCurr = (*(*psDroid).psGroup).psList;
                while !psCurr.is_null() && psCurr != psDroid {
                    psCurrNext = (*psCurr).psGrpNext;
                    //remove it from the transporter group
                    grpLeave((*psDroid).psGroup, psCurr);
                    //cam change add droid
                    (*psCurr).x =
                        (512 as libc::c_int * 127 as libc::c_int) as UWORD;
                    (*psCurr).y =
                        (512 as libc::c_int * 127 as libc::c_int) as UWORD;
                    //add it back into current droid lists
                    addDroid(psCurr, mission.apsDroidLists.as_mut_ptr());
                    psCurr = psCurrNext
                }
                //remove the transporter from the current list
                if droidRemove(psDroid, apsDroidLists.as_mut_ptr()) != 0 {
                    //cam change add droid
                    (*psDroid).x =
                        (512 as libc::c_int * 127 as libc::c_int) as UWORD;
                    (*psDroid).y =
                        (512 as libc::c_int * 127 as libc::c_int) as UWORD;
                    addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
                }
            }
            psDroid = psNext
        }
    } else {
        //reserve the droids for selected player for start of next campaign
        mission.apsDroidLists[selectedPlayer as usize] =
            apsDroidLists[selectedPlayer as usize];
        apsDroidLists[selectedPlayer as usize] = 0 as *mut DROID;
        psDroid = mission.apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            //cam change add droid
            (*psDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
            (*psDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
            psDroid = (*psDroid).psNext
        }
    }
    //if the droids have been moved to safety then get any Transporters that exist
    if getDroidsToSafetyFlag() != 0 {
        /*now that every unit for the selected player has been moved into the
        mission list - reverse it and fill the transporter with the first ten units*/
        reverseObjectList(&mut *mission.apsDroidLists.as_mut_ptr().offset(selectedPlayer
                                                                              as
                                                                              isize)
                              as *mut *mut DROID as *mut *mut BASE_OBJECT);
        //find the *first* transporter
        psDroid = mission.apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                //fill it with droids from the mission list
                psSafeDroid = mission.apsDroidLists[selectedPlayer as usize];
                while !psSafeDroid.is_null() {
                    psNextSafe = (*psSafeDroid).psNext;
                    if psSafeDroid != psDroid {
                        //add to the Transporter, checking for when full
                        if checkTransporterSpace(psDroid, psSafeDroid) != 0 {
                            if droidRemove(psSafeDroid,
                                           mission.apsDroidLists.as_mut_ptr())
                                   != 0 {
                                grpJoin((*psDroid).psGroup, psSafeDroid);
                            }
                        } else {
                            //setting this will cause the loop to end
                            psNextSafe = 0 as *mut DROID
                        }
                    }
                    psSafeDroid = psNextSafe
                }
                break ;
            } else { psDroid = (*psDroid).psNext }
        }
    }
    //clear all other droids
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_int) < 8 as libc::c_int {
        psDroid = apsDroidLists[inc as usize];
        while !psDroid.is_null() {
            psNext = (*psDroid).psNext;
            vanishDroid(psDroid);
            psDroid = psNext
        }
        inc = inc.wrapping_add(1)
    }
    //clear out the audio
    audio_StopAll();
    //clear all other memory
    freeAllStructs();
    freeAllFeatures();
}
//start an off world mission - clearing the object lists
unsafe extern "C" fn startMissionOffClear(mut pGame: *mut STRING) -> BOOL {
    saveMissionData();
    //load in the new game clearing the lists
    if loadGame(pGame, (1 as libc::c_int == 0) as libc::c_int,
                (1 as libc::c_int == 0) as libc::c_int, 0 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    //call after everything has been loaded up - done on stageThreeInit
	//gridReset();
    offWorldKeepLists = 0 as libc::c_int;
    intResetPreviousObj();
    //this gets set when the timer is added in scriptFuncs
	//mission.startTime = gameTime;
		// ffs ab
    //the message should have been played at the between stage
    missionCountDown =
        (missionCountDown as libc::c_int & !(0x20 as libc::c_int)) as UBYTE;
    return 1 as libc::c_int;
}
//start an off world mission - keeping the object lists
unsafe extern "C" fn startMissionOffKeep(mut pGame: *mut STRING) -> BOOL {
    saveMissionData();
    //load in the new game clearing the lists
    if loadGame(pGame, (1 as libc::c_int == 0) as libc::c_int,
                (1 as libc::c_int == 0) as libc::c_int, 0 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    //call after everything has been loaded up - done on stageThreeInit
	//gridReset();
    offWorldKeepLists = 1 as libc::c_int;
    intResetPreviousObj();
    //this gets set when the timer is added in scriptFuncs
	//mission.startTime = gameTime;
	// ffs ab
    //the message should have been played at the between stage
    missionCountDown =
        (missionCountDown as libc::c_int & !(0x20 as libc::c_int)) as UBYTE;
    return 1 as libc::c_int;
}
unsafe extern "C" fn startMissionCampaignStart(mut pGame: *mut STRING)
 -> BOOL {
    //clear out all intelligence screen messages
    freeMessages();
    //check no units left with any settings that are invalid
    clearCampaignUnits();
    //load in the new game details
    if loadGame(pGame, (1 as libc::c_int == 0) as libc::c_int,
                1 as libc::c_int, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    //call after everything has been loaded up - done on stageThreeInit
	//gridReset();
    offWorldKeepLists = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn startMissionCampaignChange(mut pGame: *mut STRING)
 -> BOOL {
    //clear out all intelligence screen messages
    freeMessages();
    //check no units left with any settings that are invalid
    clearCampaignUnits();
    //clear out the production run between campaigns
    changeProductionPlayer(selectedPlayer as UBYTE);
    saveCampaignData();
    //load in the new game details
    if loadGame(pGame, (1 as libc::c_int == 0) as libc::c_int,
                (1 as libc::c_int == 0) as libc::c_int, 0 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    offWorldKeepLists = 0 as libc::c_int;
    intResetPreviousObj();
    return 1 as libc::c_int;
}
unsafe extern "C" fn startMissionCampaignExpand(mut pGame: *mut STRING)
 -> BOOL {
    //load in the new game details
    if loadGame(pGame, 1 as libc::c_int,
                (1 as libc::c_int == 0) as libc::c_int, 0 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    //call after everything has been loaded up - done on stageThreeInit
	//gridReset();
    offWorldKeepLists = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn startMissionCampaignExpandLimbo(mut pGame: *mut STRING)
 -> BOOL {
    saveMissionLimboData();
    //load in the new game details
    if loadGame(pGame, 1 as libc::c_int,
                (1 as libc::c_int == 0) as libc::c_int, 0 as libc::c_int) == 0
       {
        return 0 as libc::c_int
    }
    //call after everything has been loaded up - done on stageThreeInit
	//gridReset();
    offWorldKeepLists = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn startMissionBetween() -> BOOL {
    offWorldKeepLists = 0 as libc::c_int;
    return 1 as libc::c_int;
}
//check no units left with any settings that are invalid
unsafe extern "C" fn clearCampaignUnits() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        orderDroid(psDroid, DORDER_STOP);
        (*psDroid).psBaseStruct = 0 as *mut _structure;
        psDroid = (*psDroid).psNext
    };
}
/*This deals with droids at the end of an offworld mission*/
#[no_mangle]
pub unsafe extern "C" fn processMission() {
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    let mut pickRes: PICKTILE = NO_FREE_TILE;
    //and the rest on the mission map  - for now?
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        //reset order - do this to all the droids that are returning from offWorld
        orderDroid(psDroid, DORDER_STOP);
        //remove out of stored list and add to current Droid list
        if droidRemove(psDroid, apsDroidLists.as_mut_ptr()) != 0 {
            addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
            //orderSelectedLoc(psDroid->player, psDroid->x + 3 * TILE_UNITS,
		    //	psDroid->y + 3 * TILE_UNITS);
            droidX = getHomeLandingX();
            droidY = getHomeLandingY();
            swapMissionPointers();
            pickRes =
                pickHalfATile(&mut droidX, &mut droidY,
                              20 as libc::c_int as UBYTE);
            if pickRes as libc::c_uint ==
                   NO_FREE_TILE as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"processMission: Unable to find a free location\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"mission.c\x00" as *const u8 as
                              *const libc::c_char, 1683 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"processMission\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            (*psDroid).x = (droidX << 7 as libc::c_int) as UWORD;
            (*psDroid).y = (droidY << 7 as libc::c_int) as UWORD;
            if pickRes as libc::c_uint ==
                   HALF_FREE_TILE as libc::c_int as libc::c_uint {
                (*psDroid).x =
                    ((*psDroid).x as libc::c_int + 128 as libc::c_int) as
                        UWORD;
                (*psDroid).y =
                    ((*psDroid).y as libc::c_int + 128 as libc::c_int) as
                        UWORD
            }
            (*psDroid).z =
                map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                    UWORD;
            updateDroidOrientation(psDroid);
            swapMissionPointers();
            (*psDroid).selected = 0 as libc::c_int as UBYTE;
            (*psDroid).psBaseStruct = 0 as *mut _structure;
            (*psDroid).cluster = 0 as libc::c_int as UBYTE;
            initDroidMovement(psDroid);
        }
        psDroid = psNext
    };
}
// just remove the droid from the grid cos
	    	//   there is only one grid which gets reset when we go back to the
    		//   campaign map
		    //gridRemoveObject((BASE_OBJECT *)psDroid); - happens in droidRemove()
		    //set the x/y for now
	    	//psDroid->x = getHomeLandingX() + 256;
    		//psDroid->y = getHomeLandingY() + 256;
		    //droidX = getLandingX(psDroid->player) >> TILE_SHIFT; //getHomeLandingX() >> TILE_SHIFT;
	    	//droidY = getLandingY(psDroid->player) >> TILE_SHIFT; //getHomeLandingY() >> TILE_SHIFT;
//swap the droid and map pointers
/*		    psDroid->lastTile = mapTile(psDroid->x >> TILE_SHIFT,
			psDroid->y >> TILE_SHIFT);
*/
	    	//swap the droid and map pointers back again
//this is mainly for VTOLs
//initialise the movement data
/*This deals with droids at the end of an offworld Limbo mission*/
unsafe extern "C" fn processMissionLimbo() {
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut numDroidsAddedToLimboList: UDWORD = 0 as libc::c_int as UDWORD;
    //all droids (for selectedPlayer only) are placed into the limbo list
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        //KILL OFF TRANSPORTER - should never be one but....
        if (*psDroid).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            vanishDroid(psDroid);
        } else if numDroidsAddedToLimboList >=
                      999 as libc::c_int as libc::c_uint {
            // any room in limbo list
            vanishDroid(psDroid);
        } else if droidRemove(psDroid, apsDroidLists.as_mut_ptr()) != 0 {
            //remove out of stored list and add to current Droid list
            //limbo list invalidate XY
            (*psDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
            (*psDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
            addDroid(psDroid, apsLimboDroids.as_mut_ptr());
            //this is mainly for VTOLs
            (*psDroid).psBaseStruct = 0 as *mut _structure;
            (*psDroid).cluster = 0 as libc::c_int as UBYTE;
            orderDroid(psDroid, DORDER_STOP);
            numDroidsAddedToLimboList =
                numDroidsAddedToLimboList.wrapping_add(1)
        }
        psDroid = psNext
    };
}
/*switch the pointers for the map and droid lists so that droid placement
 and orientation can occur on the map they will appear on*/
#[no_mangle]
pub unsafe extern "C" fn swapMissionPointers() {
    let mut pVoid: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ppVoid: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut udwTemp: UDWORD = 0;
    let mut inc: UDWORD = 0;
    //swap psMapTiles
    pVoid = psMapTiles as *mut libc::c_void;
    psMapTiles = mission.psMapTiles;
    mission.psMapTiles = pVoid as *mut MAPTILE;
    //swap map sizes
    udwTemp = mapWidth;
    mapWidth = mission.mapWidth;
    mission.mapWidth = udwTemp;
    udwTemp = mapHeight;
    mapHeight = mission.mapHeight;
    mission.mapHeight = udwTemp;
    //swap gateway zones
    pVoid = psGateways as *mut libc::c_void;
    psGateways = mission.psGateways;
    mission.psGateways = pVoid as *mut _gateway;
    ppVoid = apRLEZones as *mut *mut libc::c_void;
    apRLEZones = mission.apRLEZones;
    mission.apRLEZones = ppVoid as *mut *mut UBYTE;
    udwTemp = gwNumZones as UDWORD;
    gwNumZones = mission.gwNumZones;
    mission.gwNumZones = udwTemp as SDWORD;
    pVoid = aNumEquiv as *mut libc::c_void;
    aNumEquiv = mission.aNumEquiv;
    mission.aNumEquiv = pVoid as *mut UBYTE;
    ppVoid = apEquivZones as *mut *mut libc::c_void;
    apEquivZones = mission.apEquivZones;
    mission.apEquivZones = ppVoid as *mut *mut UBYTE;
    pVoid = aZoneReachable as *mut libc::c_void;
    aZoneReachable = mission.aZoneReachable;
    mission.aZoneReachable = pVoid as *mut UBYTE;
    //swap scroll limits
    udwTemp = scrollMinX as UDWORD;
    scrollMinX = mission.scrollMinX as SDWORD;
    mission.scrollMinX = udwTemp;
    udwTemp = scrollMinY as UDWORD;
    scrollMinY = mission.scrollMinY as SDWORD;
    mission.scrollMinY = udwTemp;
    udwTemp = scrollMaxX as UDWORD;
    scrollMaxX = mission.scrollMaxX as SDWORD;
    mission.scrollMaxX = udwTemp;
    udwTemp = scrollMaxY as UDWORD;
    scrollMaxY = mission.scrollMaxY as SDWORD;
    mission.scrollMaxY = udwTemp;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        pVoid = apsDroidLists[inc as usize] as *mut libc::c_void;
        apsDroidLists[inc as usize] = mission.apsDroidLists[inc as usize];
        mission.apsDroidLists[inc as usize] = pVoid as *mut DROID;
        pVoid = apsStructLists[inc as usize] as *mut libc::c_void;
        apsStructLists[inc as usize] = mission.apsStructLists[inc as usize];
        mission.apsStructLists[inc as usize] = pVoid as *mut STRUCTURE;
        pVoid = apsFeatureLists[inc as usize] as *mut libc::c_void;
        apsFeatureLists[inc as usize] = mission.apsFeatureLists[inc as usize];
        mission.apsFeatureLists[inc as usize] = pVoid as *mut FEATURE;
        pVoid = apsFlagPosLists[inc as usize] as *mut libc::c_void;
        apsFlagPosLists[inc as usize] = mission.apsFlagPosLists[inc as usize];
        mission.apsFlagPosLists[inc as usize] = pVoid as *mut FLAG_POSITION;
        inc = inc.wrapping_add(1)
    };
    /*
	for (inc = 0; inc < MAX_PLAYERS; inc++)
	{
		udwTemp = iTranspEntryTileX[inc];
		iTranspEntryTileX[inc] = mission.iTranspEntryTileX[inc];
		mission.iTranspEntryTileX[inc] = udwTemp;
		udwTemp = iTranspEntryTileY[inc];
		iTranspEntryTileY[inc] = mission.iTranspEntryTileY[inc];
		mission.iTranspEntryTileY[inc] = udwTemp;
		udwTemp = iTranspExitTileX[inc];
		iTranspExitTileX[inc] = mission.iTranspExitTileX[inc];
		mission.iTranspExitTileX[inc] = udwTemp;
		udwTemp = iTranspExitTileY[inc];
		iTranspExitTileY[inc] = mission.iTranspExitTileY[inc];
		mission.iTranspExitTileY[inc] = udwTemp;
	}
*/
	// NOTE: none of the gateway pointers are swapped at the moment
	// which isn't a problem for the current usage - might need to be
	// added later
/* stuff to add
	UDWORD				type;							//defines which start and end functions to use - see levels_type in levels.h
	//struct _proximity_display	*apsProxDisp[MAX_PLAYERS];
	FLAG_POSITION				*apsFlagPosLists[MAX_PLAYERS];
	PLAYER_POWER				asPower[MAX_PLAYERS];

//stuff for save game
	UDWORD				startTime;			//time the mission started
	SDWORD				time;				//how long the mission can last
											// < 0 = no limit
	SDWORD				ETA;				//time taken for reinforcements to arrive
											// < 0 = none allowed
    UWORD               homeLZ_X;           //selectedPlayer's LZ x and y
    UWORD               homeLZ_Y;
	SDWORD				playerX;			//original view position
	SDWORD				playerY;
*/
}
#[no_mangle]
pub unsafe extern "C" fn endMission() {
    if mission.type_0 == LDS_NONE as libc::c_int as libc::c_uint {
        //can't go back any further!!
        debug(LOG_NEVER,
              b"Already returned from mission\x00" as *const u8 as
                  *const libc::c_char);
        return
    }
    match mission.type_0 {
        2 => {
            //any transporters that are flying in need to be emptied
            emptyTransporters(0 as libc::c_int);
            //when loading in a save game mid cam2a or cam3a it is loaded as a camstart
            endMissionCamChange();
        }
        6 => {
            //any transporters that are flying in need to be emptied
            emptyTransporters(1 as libc::c_int);
            endMissionOffKeep();
        }
        4 | 5 => { }
        3 => {
            //any transporters that are flying in need to be emptied
            emptyTransporters(0 as libc::c_int);
            endMissionCamChange();
        }
        8 => {
            /* left in so can skip the mission for testing...*/
            //shouldn't be any transporters on this mission but...who knows?
            endMissionExpandLimbo();
        }
        7 => {
            //any transporters that are flying in need to be emptied
            emptyTransporters(1 as libc::c_int);
            endMissionOffClear();
        }
        9 => {
            //any transporters that are flying in need to be emptied
            emptyTransporters(1 as libc::c_int);
            endMissionOffKeepLimbo();
        }
        _ => {
            //error!
            debug(LOG_ERROR,
                  b"Unknown Mission Type\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
    }
    if missionCanReEnforce() != 0 {
        //mission.type == LDS_MCLEAR OR mission.type == LDS_MKEEP)
        intRemoveMissionTimer();
        intRemoveTransporterTimer();
    }
    //at end of mission always do this
    intRemoveTransporterLaunch();
    //and this...
    //make sure the cheat time is not set for the next mission
    mission.cheatTime = 0 as libc::c_int as UDWORD;
    //reset the bSetPlayCountDown flag
    setPlayCountDown(1 as libc::c_int as UBYTE);
    //mission.type = MISSION_NONE;
    mission.type_0 = LDS_NONE as libc::c_int as UDWORD;
    // reset the transporters
    initTransporters();
}
unsafe extern "C" fn endMissionCamChange() {
    //get any droids remaining from the previous campaign
    processPreviousCampDroids();
}
unsafe extern "C" fn endMissionOffClear() {
    processMission();
    //called in setUpMission now - AB 6/4/98
	//intAddMissionResult(result);// ajl. fire up interface.
    restoreMissionData();
    //reset variables in Droids
    missionResetDroids();
}
unsafe extern "C" fn endMissionOffKeep() {
    processMission();
    //called in setUpMission now - AB 6/4/98
	//intAddMissionResult(result);// ajl. fire up interface.
    restoreMissionData();
    //reset variables in Droids
    missionResetDroids();
}
/*In this case any droids remaining (for selectedPlayer) go into a limbo list
for use in a future mission (expand type) */
unsafe extern "C" fn endMissionOffKeepLimbo() {
    //save any droids left 'alive'
    processMissionLimbo();
    //set the lists back to the home base lists
    restoreMissionData();
    //reset variables in Droids
    missionResetDroids();
}
//This happens MID_MISSION now! but is left here in case the scripts fail but somehow get here...?
/*The selectedPlayer's droids which were separated at the start of the
mission need to merged back into the list*/
unsafe extern "C" fn endMissionExpandLimbo() { restoreMissionLimboData(); }
//this is called mid Limbo mission via the script
#[no_mangle]
pub unsafe extern "C" fn resetLimboMission() {
    //add the units that were moved into the mission list at the start of the mission
    restoreMissionLimboData();
    //set the mission type to plain old expand...
    mission.type_0 = LDS_EXPAND as libc::c_int as UDWORD;
}
/* The AI update routine for all Structures left back at base during a Mission*/
unsafe extern "C" fn aiUpdateMissionStructure(mut psStructure:
                                                  *mut STRUCTURE) {
    let mut pSubject: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut pointsToAdd: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[(*psStructure).player as usize];
    let mut structureMode: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psNewDroid: *mut DROID = 0 as *mut DROID;
    let mut Quantity: UBYTE = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psResFacility: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    let mut psNextTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"aiUpdateMissionStructure: invalid Structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              2047 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"aiUpdateMissionStructure\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psStructure).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"aiUpdateMissionStructure: Structure is not a Factory or Research Facility\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*(*psStructure).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              2053 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"aiUpdateMissionStructure\x00")).as_ptr(),
              b"(psStructure->pStructureType->type == REF_FACTORY OR psStructure->pStructureType->type == REF_CYBORG_FACTORY OR psStructure->pStructureType->type == REF_VTOL_FACTORY OR psStructure->pStructureType->type == REF_RESEARCH)\x00"
                  as *const u8 as *const libc::c_char);
    };
    //only interested if the Structure "does" something!
    if (*psStructure).pFunctionality.is_null() { return }
    //check if any power available
    if (*asPower[(*psStructure).player as usize]).currentPower >
           5 as libc::c_int as libc::c_uint || powerCalculated == 0 {
        //check if this structure is due some power
        if getLastPowered(psStructure as *mut BASE_OBJECT) != 0 {
            //get some power if necessary
            if accruePower(psStructure as *mut BASE_OBJECT) != 0 {
                updateLastPowered(psStructure as *mut BASE_OBJECT,
                                  (*psStructure).player);
            }
        }
    }
    //determine the Subject
    match (*(*psStructure).pStructureType).type_0 {
        10 => {
            pSubject =
                (*((*psStructure).pFunctionality as
                       *mut RESEARCH_FACILITY)).psSubject;
            structureMode = REF_RESEARCH as libc::c_int as UDWORD
        }
        1 | 16 | 17 => {
            pSubject =
                (*((*psStructure).pFunctionality as *mut FACTORY)).psSubject;
            structureMode = REF_FACTORY as libc::c_int as UDWORD;
            //check here to see if the factory's commander has died
            if !(*((*psStructure).pFunctionality as
                       *mut FACTORY)).psCommander.is_null() &&
                   (*(*((*psStructure).pFunctionality as
                            *mut FACTORY)).psCommander).died != 0 {
                //remove the commander from the factory
                assignFactoryCommandDroid(psStructure, 0 as *mut _droid);
            }
        }
        _ => { }
    }
    if !pSubject.is_null() {
        //if subject is research...
        if structureMode == REF_RESEARCH as libc::c_int as libc::c_uint {
            psResFacility =
                (*psStructure).pFunctionality as *mut RESEARCH_FACILITY;
            //if on hold don't do anything
            if (*psResFacility).timeStartHold != 0 { return }
            pPlayerRes =
                pPlayerRes.offset((*pSubject).ref_0.wrapping_sub(0xb0000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                      as isize);
            //check research has not already been completed by another structure
            if (*pPlayerRes).ResearchStatus as libc::c_int &
                   0x4 as libc::c_int == 0 as libc::c_int {
                //check to see if enough power to research has accrued
                if (*psResFacility).powerAccrued <
                       (*(pSubject as *mut RESEARCH)).researchPower {
                    //wait until enough power
                    return
                }
                if (*psResFacility).timeStarted ==
                       0 as libc::c_int as libc::c_uint {
                    //set the time started
                    (*psResFacility).timeStarted = gameTime
                }
                pointsToAdd =
                    (*psResFacility).researchPoints.wrapping_mul(gameTime.wrapping_sub((*psResFacility).timeStarted)).wrapping_div(1000
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint);
                //check if Research is complete
                //if ((pointsToAdd + pPlayerRes->currentPoints) > psResFacility->
				//	timeToResearch)
                if pointsToAdd.wrapping_add((*pPlayerRes).currentPoints) >
                       (*(pSubject as *mut RESEARCH)).researchPoints as
                           libc::c_uint {
                    //store the last topic researched - if its the best
                    if (*psResFacility).psBestTopic.is_null() {
                        (*psResFacility).psBestTopic =
                            (*psResFacility).psSubject
                    } else if (*((*psResFacility).psSubject as
                                     *mut RESEARCH)).researchPoints as
                                  libc::c_int >
                                  (*((*psResFacility).psBestTopic as
                                         *mut RESEARCH)).researchPoints as
                                      libc::c_int {
                        (*psResFacility).psSubject =
                            (*psResFacility).psSubject
                    }
                    (*psResFacility).psSubject = 0 as *mut _base_stats;
                    intResearchFinished(psStructure);
                    researchResult((*pSubject).ref_0.wrapping_sub(0xb0000 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                                   (*psStructure).player, 1 as libc::c_int);
                    //check if this result has enabled another topic
                    intCheckResearchButton();
                }
            } else {
                //cancel this Structure's research since now complete
                (*psResFacility).psSubject = 0 as *mut _base_stats;
                intResearchFinished(psStructure);
            }
        } else if structureMode == REF_FACTORY as libc::c_int as libc::c_uint
         {
            psFactory = (*psStructure).pFunctionality as *mut FACTORY;
            Quantity = (*psFactory).quantity;
            //check for manufacture
            //if on hold don't do anything
            if (*psFactory).timeStartHold != 0 { return }
            //			if (psFactory->timeStarted == ACTION_START_TIME)
//			{
//				// also need to check if a command droid's group is full
//				if ( ( psFactory->psCommander != NULL ) &&
//					 ( grpNumMembers( psFactory->psCommander->psGroup ) >=
//							cmdDroidMaxGroup( psFactory->psCommander ) ) )
//				{
//					return;
//				}
//			}
            if CheckHaltOnMaxUnitsReached(psStructure) == 1 as libc::c_int {
                return
            }
            //check enough power has accrued to build the droid
            if (*psFactory).powerAccrued <
                   (*(pSubject as *mut DROID_TEMPLATE)).powerPoints {
                //wait until enough power
                return
            }
            /*must be enough power so subtract that required to build*/
            if (*psFactory).timeStarted == 0 as libc::c_int as libc::c_uint {
                //set the time started
                (*psFactory).timeStarted = gameTime
            }
            pointsToAdd =
                gameTime.wrapping_sub((*psFactory).timeStarted).wrapping_div(1000
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint);
            //check for manufacture to be complete
            if pointsToAdd > (*psFactory).timeToBuild {
                //build droid - store in mission list
                psNewDroid =
                    buildMissionDroid(pSubject as *mut DROID_TEMPLATE,
                                      (*psStructure).x as UDWORD,
                                      (*psStructure).y as UDWORD,
                                      (*psStructure).player as UDWORD);
                if psNewDroid.is_null() {
                    //if couldn't build then cancel the production
                    Quantity = 0 as libc::c_int as UBYTE;
                    (*psFactory).psSubject = 0 as *mut BASE_STATS;
                    intManufactureFinished(psStructure);
                } else {
                    if (*psStructure).player as libc::c_uint == selectedPlayer
                       {
                        intRefreshScreen();
                        // update the interface.
                    }
                    //store the factory as the droid's baseStructure instead
					//add it to the factory group
					/*if (!psFactory->psGroup)
					{
						//create the factory group
						if (!grpCreate(&psFactory->psGroup))
						{
							DBPRINTF(("missionUpdateStructure: unable to create group\n"));
						}
						else
						{
							grpJoin(psFactory->psGroup, psNewDroid);
						}
					}
					else
					{
						grpJoin(psFactory->psGroup, psNewDroid);
					}*/
                    (*psNewDroid).psBaseStruct = psStructure;
                    //reset the start time
                    (*psFactory).timeStarted = 0 as libc::c_int as UDWORD;
                    (*psFactory).powerAccrued = 0 as libc::c_int as UDWORD;
                    //next bit for productionPlayer only
                    if productionPlayer as libc::c_int ==
                           (*psStructure).player as libc::c_int {
                        psNextTemplate =
                            factoryProdUpdate(psStructure,
                                              pSubject as
                                                  *mut DROID_TEMPLATE);
                        if !psNextTemplate.is_null() {
                            structSetManufacture(psStructure, psNextTemplate,
                                                 Quantity);
                            return
                        } else {
                            //nothing more to manufacture - reset the Subject and Tab on HCI Form
                            (*psFactory).psSubject = 0 as *mut BASE_STATS;
                            intManufactureFinished(psStructure);
                        }
                    } else {
                        //decrement the quantity to manufacture if not set to infinity
                        if Quantity as libc::c_int !=
                               10 as libc::c_int + 1 as libc::c_int {
                            (*psFactory).quantity =
                                (*psFactory).quantity.wrapping_sub(1);
                            Quantity = Quantity.wrapping_sub(1)
                        }
                        // If quantity not 0 then kick of another manufacture.
                        if Quantity != 0 {
                            // Manufacture another.
                            structSetManufacture(psStructure,
                                                 pSubject as
                                                     *mut DROID_TEMPLATE,
                                                 Quantity);
                            //playerNewDroid(psNewDroid);
                            return
                        } else {
                            //when quantity = 0, reset the Subject and Tab on HCI Form
                            (*psFactory).psSubject = 0 as *mut BASE_STATS;
                            intManufactureFinished(psStructure);
                        }
                    }
                }
            }
        }
    };
}
/* The update routine for all Structures left back at base during a Mission*/
#[no_mangle]
pub unsafe extern "C" fn missionStructureUpdate(mut psBuilding:
                                                    *mut STRUCTURE) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureUpdate: Invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              2314 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"missionStructureUpdate\x00")).as_ptr(),
              b"PTRVALID(psBuilding, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //update the manufacture/research of the building
//	if (psBuilding->pStructureType->type == REF_FACTORY OR
//		psBuilding->pStructureType->type == REF_CYBORG_FACTORY OR
//		psBuilding->pStructureType->type == REF_VTOL_FACTORY OR
//		psBuilding->pStructureType->type == REF_RESEARCH)
    if StructIsFactory(psBuilding) != 0 ||
           (*(*psBuilding).pStructureType).type_0 ==
               REF_RESEARCH as libc::c_int as libc::c_uint {
        if (*psBuilding).status as libc::c_int == SS_BUILT as libc::c_int {
            aiUpdateMissionStructure(psBuilding);
        }
    };
}
/* The update routine for all droids left back at home base
Only interested in Transporters at present*/
#[no_mangle]
pub unsafe extern "C" fn missionDroidUpdate(mut psDroid: *mut DROID) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"unitUpdate: Invalid unit pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              2336 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"missionDroidUpdate\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    /*This is required for Transporters that are moved offWorld so the
    saveGame doesn't try to set their position in the map - especially important
    for endCam2 where there isn't a valid map!*/
    if (*psDroid).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        (*psDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
        (*psDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD
    }
    //ignore all droids except Transporters
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint ||
           !(orderState(psDroid, DORDER_TRANSPORTOUT) != 0 ||
                 orderState(psDroid, DORDER_TRANSPORTIN) != 0 ||
                 orderState(psDroid, DORDER_TRANSPORTRETURN) != 0) {
        return
    }
    // NO ai update droid
    // update the droids order
    orderUpdateDroid(psDroid);
    // update the action of the droid
    actionUpdateDroid(psDroid);
    //NO move update
}
//reset variables in Droids such as order and position
unsafe extern "C" fn missionResetDroids() {
    let mut player_0: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    //	UDWORD			mapX, mapY;
    let mut placed: BOOL = 0;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut pickRes: PICKTILE = NO_FREE_TILE;
    player_0 = 0 as libc::c_int as UDWORD;
    while player_0 < 8 as libc::c_int as libc::c_uint {
        psDroid = apsDroidLists[player_0 as usize];
        while !psDroid.is_null() {
            psNext = (*psDroid).psNext;
            //reset order - unless constructor droid that is mid-build
            //if (psDroid->droidType == DROID_CONSTRUCT AND orderStateObj(psDroid,
            if ((*psDroid).droidType as libc::c_uint ==
                    DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
                    (*psDroid).droidType as libc::c_uint ==
                        DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint)
                   &&
                   orderStateObj(psDroid, DORDER_BUILD,
                                 &mut psStruct as *mut *mut STRUCTURE as
                                     *mut *mut BASE_OBJECT) != 0 {
                //need to set the action time to ignore the previous mission time
                (*psDroid).actionStarted = gameTime
            } else { orderDroid(psDroid, DORDER_STOP); }
            //KILL OFF TRANSPORTER
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                vanishDroid(psDroid);
            }
            psDroid = psNext
        }
        player_0 = player_0.wrapping_add(1)
    }
    //don't need to implement this hack now since using pickATile - oh what a wonderful routine...
	//only need to look through the selected players list
	/*need to put all the droids that were built whilst off on the mission into
	a temp list so can add them back into the world one by one so that the
	path blocking routines do not fail*/
	/*for (psDroid = apsDroidLists[selectedPlayer]; psDroid != NULL; psDroid =
		psNext)
	{
		psNext = psDroid->psNext;
		//for all droids that have never left home base
		if (psDroid->x == INVALID_XY AND psDroid->y == INVALID_XY)
		{
			if (droidRemove(psDroid, apsDroidLists))
            {
			    addDroid(psDroid, mission.apsDroidLists);
            }
		}
	}*/
    psDroid = apsDroidLists[selectedPlayer as usize];
    while !psDroid.is_null() {
        //for (psDroid = mission.apsDroidLists[selectedPlayer]; psDroid != NULL;
	//	psDroid = psNext)
        psNext = (*psDroid).psNext;
        //for all droids that have never left home base
        if (*psDroid).x as libc::c_int ==
               512 as libc::c_int * 127 as libc::c_int &&
               (*psDroid).y as libc::c_int ==
                   512 as libc::c_int * 127 as libc::c_int {
            //this is now stored as the baseStruct when the droid is built...
			//find which factory produced the droid
			/*psFactory = NULL;
			for (psStruct = apsStructLists[psDroid->player]; psStruct !=
				NULL; psStruct = psStruct->psNext)
			{
				if(StructIsFactory(psStruct))
				{
					if (((FACTORY *)psStruct->pFunctionality)->psGroup ==
						psDroid->psGroup)
					{
						//found the right factory
						psFactory = (FACTORY *)psStruct->pFunctionality;
						break;
					}
				}
			}*/
            psStruct = (*psDroid).psBaseStruct;
            if !psStruct.is_null() && StructIsFactory(psStruct) != 0 {
                psFactory = (*psStruct).pFunctionality as *mut FACTORY
            }
            placed = 0 as libc::c_int;
            //find a location next to the factory
            if !psStruct.is_null() {
                /*placed = placeDroid(psStruct, &x, &y);
				if (placed)
				{
					psDroid->x = (UWORD)(x << TILE_SHIFT);
					psDroid->y = (UWORD)(y << TILE_SHIFT);
				}
				else
				{
					psStruct = NULL;
				}*/
                //use pickATile now - yipeee!
                //use factory DP if one
                if !(*psFactory).psAssemblyPoint.is_null() {
                    x =
                        ((*(*psFactory).psAssemblyPoint).coords.x >>
                             7 as libc::c_int) as UWORD as UDWORD;
                    y =
                        ((*(*psFactory).psAssemblyPoint).coords.y >>
                             7 as libc::c_int) as UWORD as UDWORD
                } else {
                    x =
                        ((*psStruct).x as libc::c_int >> 7 as libc::c_int) as
                            UWORD as UDWORD;
                    y =
                        ((*psStruct).y as libc::c_int >> 7 as libc::c_int) as
                            UWORD as UDWORD
                }
                pickRes =
                    pickHalfATile(&mut x, &mut y, 20 as libc::c_int as UBYTE);
                if pickRes as libc::c_uint ==
                       NO_FREE_TILE as libc::c_int as libc::c_uint {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"missionResetUnits: Unable to find a free location\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"mission.c\x00" as *const u8 as
                                  *const libc::c_char, 2486 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"missionResetDroids\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    psStruct = 0 as *mut STRUCTURE
                } else {
                    (*psDroid).x = (x << 7 as libc::c_int) as UWORD;
                    (*psDroid).y = (y << 7 as libc::c_int) as UWORD;
                    if pickRes as libc::c_uint ==
                           HALF_FREE_TILE as libc::c_int as libc::c_uint {
                        (*psDroid).x =
                            ((*psDroid).x as libc::c_int + 128 as libc::c_int)
                                as UWORD;
                        (*psDroid).y =
                            ((*psDroid).y as libc::c_int + 128 as libc::c_int)
                                as UWORD
                    }
                    placed = 1 as libc::c_int
                }
            }
            //if couldn't find the factory - hmmm
            if psStruct.is_null() {
                //just stick them near the HQ
                psStruct = apsStructLists[(*psDroid).player as usize];
                while !psStruct.is_null() {
                    if (*(*psStruct).pStructureType).type_0 ==
                           REF_HQ as libc::c_int as libc::c_uint {
                        /*psDroid->x = (UWORD)(psStruct->x + 128);
						psDroid->y = (UWORD)(psStruct->y + 128);
						placed = TRUE;
						break;*/
                        //use pickATile again...
                        x =
                            ((*psStruct).x as libc::c_int >> 7 as libc::c_int)
                                as UWORD as UDWORD;
                        y =
                            ((*psStruct).y as libc::c_int >> 7 as libc::c_int)
                                as UWORD as UDWORD;
                        pickRes =
                            pickHalfATile(&mut x, &mut y,
                                          20 as libc::c_int as UBYTE);
                        if pickRes as libc::c_uint ==
                               NO_FREE_TILE as libc::c_int as libc::c_uint {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"missionResetUnits: Unable to find a free location\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"mission.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      2520 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 19],
                                                                &[libc::c_char; 19]>(b"missionResetDroids\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                            psStruct = 0 as *mut STRUCTURE
                        } else {
                            (*psDroid).x = (x << 7 as libc::c_int) as UWORD;
                            (*psDroid).y = (y << 7 as libc::c_int) as UWORD;
                            if pickRes as libc::c_uint ==
                                   HALF_FREE_TILE as libc::c_int as
                                       libc::c_uint {
                                (*psDroid).x =
                                    ((*psDroid).x as libc::c_int +
                                         128 as libc::c_int) as UWORD;
                                (*psDroid).y =
                                    ((*psDroid).y as libc::c_int +
                                         128 as libc::c_int) as UWORD
                            }
                            placed = 1 as libc::c_int
                        }
                        break ;
                    } else { psStruct = (*psStruct).psNext }
                }
            }
            if placed != 0 {
                //don't need to do this since using pickATile now...
				//add them to the current list
				/*if (droidRemove(psDroid, mission.apsDroidLists))
                {
				    addDroid(psDroid, apsDroidLists);
    				if (psFactory)
	    			{
		    			//order the droid to the factory DP
			    		orderSelectedLoc(psDroid->player,
				    		psFactory->psAssemblyPoint->coords.x,
					    	psFactory->psAssemblyPoint->coords.y);
    				}
	    			else
		    		{
			    		//order them to move to an arbitary new location!
				    	orderSelectedLoc(psDroid->player, psDroid->x + 3 * TILE_UNITS,
					    	psDroid->y + 3 * TILE_UNITS);
				    }*/
				    //do all the things in build droid that never did when it was built!
				    // check the droid is a reasonable distance from the edge of the map
                if (*psDroid).x as libc::c_int <= 128 as libc::c_int ||
                       (*psDroid).x as libc::c_uint >=
                           mapWidth.wrapping_mul(128 as libc::c_int as
                                                     libc::c_uint).wrapping_sub(128
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                       || (*psDroid).y as libc::c_int <= 128 as libc::c_int ||
                       (*psDroid).y as libc::c_uint >=
                           mapHeight.wrapping_mul(128 as libc::c_int as
                                                      libc::c_uint).wrapping_sub(128
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                   {
                    debug(LOG_NEVER,
                          b"missionResetUnits: unit too close to edge of map - removing\x00"
                              as *const u8 as *const libc::c_char);
                    vanishDroid(psDroid);
                } else {
                    // Check there is nothing on the map
				    /*if ( TILE_OCCUPIED(mapTile(x >> TILE_SHIFT, y >> TILE_SHIFT)) )
				    {
					    DBPRINTF(("missionResetDroids: tile occupied\n");
	    `				removeDroid(psDroid, apsDroidLists);
					    droidRelease(psDroid);
					    HEAP_FREE(psDroidHeap, psDroid);
				    }*/
/*				    mapX = psDroid->x >> TILE_SHIFT;
				    mapY = psDroid->y >> TILE_SHIFT;
				    psDroid->lastTile = mapTile(mapX,mapY);
*/
				    //set droid height
                    (*psDroid).z =
                        map_Height((*psDroid).x as UDWORD,
                                   (*psDroid).y as UDWORD) as UWORD;
                    // People always stand upright
				    //if(psDroid->droidType != DROID_PERSON AND psDroid->type != DROID_CYBORG)
                    if (*psDroid).droidType as libc::c_uint !=
                           DROID_PERSON as libc::c_int as libc::c_uint &&
                           cyborgDroid(psDroid) == 0 {
                        updateDroidOrientation(psDroid);
                    }
                    visTilesUpdate(psDroid as *mut BASE_OBJECT,
                                   0 as libc::c_int);
                    //reset the selected flag
                    (*psDroid).selected = 0 as libc::c_int as UBYTE
                }
                //}
            } else {
                //can't put it down so get rid of this droid!!
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"missionResetUnits: can\'t place unit - cancel to continue\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"mission.c\x00" as *const u8 as
                              *const libc::c_char, 2597 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 19],
                                                    &[libc::c_char; 19]>(b"missionResetDroids\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                vanishDroid(psDroid);
            }
        }
        psDroid = (*psDroid).psNext
    };
}
/*unloads the Transporter passed into the mission at the specified x/y
goingHome = TRUE when returning from an off World mission*/
#[no_mangle]
pub unsafe extern "C" fn unloadTransporter(mut psTransporter: *mut DROID,
                                           mut x: UDWORD, mut y: UDWORD,
                                           mut goingHome: BOOL) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut ppCurrentList: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut ppStoredList: *mut *mut DROID = 0 as *mut *mut DROID;
    let mut droidX: UDWORD = 0;
    let mut droidY: UDWORD = 0;
    let mut iX: UWORD = 0;
    let mut iY: UWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if goingHome != 0 {
        ppCurrentList = mission.apsDroidLists.as_mut_ptr();
        ppStoredList = apsDroidLists.as_mut_ptr()
    } else {
        ppCurrentList = apsDroidLists.as_mut_ptr();
        ppStoredList = mission.apsDroidLists.as_mut_ptr()
    }
    //look through the stored list of droids to see if there any transporters
	/*for (psTransporter = ppStoredList[selectedPlayer]; psTransporter != NULL; psTransporter =
		psTransporter->psNext)
	{
		if (psTransporter->droidType == DROID_TRANSPORTER)
		{
			//remove out of stored list and add to current Droid list
			removeDroid(psTransporter, ppStoredList);
			addDroid(psTransporter, ppCurrentList);
			//need to put the Transporter down at a specified location
			psTransporter->x = x;
			psTransporter->y = y;
		}
	}*/
    //unload all the droids from within the current Transporter
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        // If the scripts asked for transporter tracking then clear the "tracking a transporter" flag
		// since the transporters landed and unloaded now.
        if (*psTransporter).player as libc::c_uint == selectedPlayer {
            bTrackingTransporter = 0 as libc::c_int
        }
        // reset the transporter cluster
        (*psTransporter).cluster = 0 as libc::c_int as UBYTE;
        psDroid = (*(*psTransporter).psGroup).psList;
        while !psDroid.is_null() && psDroid != psTransporter {
            psNext = (*psDroid).psGrpNext;
            //add it back into current droid lists
            addDroid(psDroid, ppCurrentList);
            //hack in the starting point!
			/*psDroid->x = x;
			psDroid->y = y;
			if (!goingHome)
			{
				//send the droids off to a starting location
				orderSelectedLoc(psDroid->player, x + 3*TILE_UNITS, y + 3*TILE_UNITS);
			}*/
			//starting point...based around the value passed in
            droidX = x >> 7 as libc::c_int;
            droidY = y >> 7 as libc::c_int;
            if goingHome != 0 {
                //swap the droid and map pointers
                swapMissionPointers();
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
                          b"unloadTransporter: Unable to find a valid location\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"mission.c\x00" as *const u8 as
                              *const libc::c_char, 2676 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"unloadTransporter\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
            }
            (*psDroid).x = (droidX << 7 as libc::c_int) as UWORD;
            (*psDroid).y = (droidY << 7 as libc::c_int) as UWORD;
            (*psDroid).z =
                map_Height((*psDroid).x as UDWORD, (*psDroid).y as UDWORD) as
                    UWORD;
            updateDroidOrientation(psDroid);
            // a commander needs to get it's group back
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_COMMAND as libc::c_int as libc::c_uint {
                if grpCreate(&mut psGroup) != 0 { grpJoin(psGroup, psDroid); }
                clearCommandDroidFactory(psDroid);
            }
            //initialise the movement data
            initDroidMovement(psDroid);
            //reset droid orders
            orderDroid(psDroid, DORDER_STOP);
            gridAddObject(psDroid as *mut BASE_OBJECT);
            (*psDroid).selected = 0 as libc::c_int as UBYTE;
            //this is mainly for VTOLs
            (*psDroid).psBaseStruct = 0 as *mut _structure;
            (*psDroid).cluster = 0 as libc::c_int as UBYTE;
            if goingHome != 0 {
                //swap the droid and map pointers
                swapMissionPointers();
            }
            //inform all other players
            if bMultiPlayer != 0 { sendDroidDisEmbark(psDroid); }
            psDroid = psNext
        }
        /* trigger script callback detailing group about to disembark */
        transporterSetScriptCurrent(psTransporter);
        eventFireCallbackTrigger(CALL_TRANSPORTER_LANDED as libc::c_int as
                                     TRIGGER_TYPE);
        transporterSetScriptCurrent(0 as *mut DROID);
        /* remove droids from transporter group if not already transferred to script group */
        psDroid = (*(*psTransporter).psGroup).psList;
        while !psDroid.is_null() && psDroid != psTransporter {
            psNext = (*psDroid).psGrpNext;
            grpLeave((*psTransporter).psGroup, psDroid);
            psDroid = psNext
        }
    }
    //don't do this in multiPlayer
    if bMultiPlayer == 0 {
        //send all transporter Droids back to home base if off world
        if goingHome == 0 {
            /* stop the camera following the transporter */
            (*psTransporter).selected = 0 as libc::c_int as UBYTE;
            /* send transporter offworld */
            missionGetTransporterExit((*psTransporter).player as SDWORD,
                                      &mut iX, &mut iY);
            orderDroidLoc(psTransporter, DORDER_TRANSPORTRETURN, iX as UDWORD,
                          iY as UDWORD);
            //set the launch time so the transporter doesn't just disappear for CAMSTART/CAMCHANGE
            transporterSetLaunchTime(gameTime);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn missionMoveTransporterOffWorld(mut psTransporter:
                                                            *mut DROID) {
    let mut psForm: *mut W_CLICKFORM = 0 as *mut W_CLICKFORM;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        /* trigger script callback */
        transporterSetScriptCurrent(psTransporter);
        eventFireCallbackTrigger(CALL_TRANSPORTER_OFFMAP as libc::c_int as
                                     TRIGGER_TYPE);
        transporterSetScriptCurrent(0 as *mut DROID);
        //gridRemoveObject( (BASE_OBJECT *) psTransporter ); - these happen in droidRemove()
		//clustRemoveObject( (BASE_OBJECT *) psTransporter );
        if droidRemove(psTransporter, apsDroidLists.as_mut_ptr()) != 0 {
            addDroid(psTransporter, mission.apsDroidLists.as_mut_ptr());
        }
        //stop the droid moving - the moveUpdate happens AFTER the orderUpdate and can cause problems if the droid moves from one tile to another
        moveReallyStopDroid(psTransporter);
        //if offworld mission, then add the timer
		//if (mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR)
        if missionCanReEnforce() != 0 &&
               (*psTransporter).player as libc::c_uint == selectedPlayer {
            addTransporterTimerInterface();
            //set the data for the transporter timer label
            widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD,
                            psTransporter as *mut libc::c_void);
            //make sure the button is enabled
            psForm =
                widgGetFromID(psWScreen, 11012 as libc::c_int as UDWORD) as
                    *mut W_CLICKFORM;
            if !psForm.is_null() {
                formSetClickState(psForm, 0 as libc::c_int as UDWORD);
            }
        }
        //need a callback for when all the selectedPlayers' reinforcements have been delivered
        if (*psTransporter).player as libc::c_uint == selectedPlayer {
            psDroid = 0 as *mut DROID;
            psDroid = mission.apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                if (*psDroid).droidType as libc::c_uint !=
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    break ;
                }
                psDroid = (*psDroid).psNext
            }
            if psDroid.is_null() {
                eventFireCallbackTrigger(CALL_NO_REINFORCEMENTS_LEFT as
                                             libc::c_int as TRIGGER_TYPE);
            }
        }
    } else {
        debug(LOG_NEVER,
              b"missionMoveTransporterOffWorld: droid type not transporter!\n\x00"
                  as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn intReAddMissionTimer() {
    if !widgGetFromID(psWScreen, 11000 as libc::c_int as UDWORD).is_null() {
        let mut UserData: *mut libc::c_void = 0 as *mut libc::c_void;
        // Need to preserve widgets UserData.
        UserData = widgGetUserData(psWScreen, 11010 as libc::c_int as UDWORD);
        intRemoveMissionTimer();
        intAddMissionTimer();
        widgSetUserData(psWScreen, 11010 as libc::c_int as UDWORD, UserData);
    };
}
//add the Mission timer into the top  right hand corner of the screen
unsafe extern "C" fn intAddMissionTimer() -> BOOL {
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
    //check to see if it exists already
    if !widgGetFromID(psWScreen, 11000 as libc::c_int as UDWORD).is_null() {
        return 1 as libc::c_int
    }
    // Add the background
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as
               libc::c_ulong); //TIMER_WIDTH;
    sFormInit.formID = 0 as libc::c_int as UDWORD; //TIMER_HEIGHT;
    sFormInit.id = 11000 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.width =
        iV_GetImageWidth(IntImages,
                         IMAGE_MISSION_CLOCK as libc::c_int as UWORD);
    sFormInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_MISSION_CLOCK as libc::c_int as UWORD);
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
                                                                                                                                                                                                                                                                                                                     libc::c_uint)).wrapping_add(128
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     libc::c_int
                                                                                                                                                                                                                                                                                                                                                     as
                                                                                                                                                                                                                                                                                                                                                     libc::c_uint).wrapping_sub(sFormInit.width
                                                                                                                                                                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                                                                                                                                                                    libc::c_uint)
            as SWORD;
    sFormInit.y = 22 as libc::c_int as SWORD;
    sFormInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_MISSION_CLOCK as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_MISSION_CLOCK_UP as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    sFormInit.pDisplay =
        Some(intDisplayMissionClock as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //add labels for the time display
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as
               libc::c_ulong); //TIMER_WIDTH;
    sLabInit.formID = 11000 as libc::c_int as UDWORD; //TIMER_HEIGHT;
    sLabInit.id = 11001 as libc::c_int as UDWORD;
    sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInit.x = 15 as libc::c_int as SWORD;
    sLabInit.y = 0 as libc::c_int as SWORD;
    sLabInit.width = sFormInit.width;
    sLabInit.height = sFormInit.height;
    sLabInit.pText =
        b"00:00:00\x00" as *const u8 as *const libc::c_char as *mut STRING;
    sLabInit.FontID = WFont;
    sLabInit.pCallback =
        Some(intUpdateMissionTimer as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//add the Transporter timer into the top left hand corner of the screen
unsafe extern "C" fn intAddTransporterTimer() -> BOOL {
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
    //make sure that Transporter Launch button isn't up as well
    intRemoveTransporterLaunch();
    //check to see if it exists already
    if !widgGetFromID(psWScreen, 11012 as libc::c_int as UDWORD).is_null() {
        return 1 as libc::c_int
    }
    // Add the button form - clickable
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 11012 as libc::c_int as UDWORD;
    sFormInit.style = (4 as libc::c_int | 8 as libc::c_int) as UDWORD;
    sFormInit.x = 23 as libc::c_int as SWORD;
    sFormInit.y = 22 as libc::c_int as SWORD;
    sFormInit.width =
        iV_GetImageWidth(IntImages,
                         IMAGE_TRANSETA_UP as libc::c_int as UWORD);
    sFormInit.height =
        iV_GetImageHeight(IntImages,
                          IMAGE_TRANSETA_UP as libc::c_int as UWORD);
    sFormInit.pTip =
        strresGetString(psStringRes,
                        STR_INT_TRANSPORTER as libc::c_int as UDWORD);
    sFormInit.pDisplay =
        Some(intDisplayImageHilight as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    sFormInit.pUserData =
        ((0 as libc::c_int & 0x3ff as libc::c_int) << 20 as libc::c_int |
             (IMAGE_TRANSETA_DOWN as libc::c_int & 0x3ff as libc::c_int) <<
                 10 as libc::c_int |
             IMAGE_TRANSETA_UP as libc::c_int & 0x3ff as libc::c_int) as
            *mut libc::c_void;
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //add labels for the time display
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = 11012 as libc::c_int as UDWORD;
    sLabInit.id = 11010 as libc::c_int as UDWORD;
    sLabInit.style = (0 as libc::c_int | 0x8000 as libc::c_int) as UDWORD;
    sLabInit.x = 4 as libc::c_int as SWORD;
    sLabInit.y = 0 as libc::c_int as SWORD;
    sLabInit.width = 25 as libc::c_int as UWORD;
    sLabInit.height = sFormInit.height;
    sLabInit.FontID = WFont;
    sLabInit.pCallback =
        Some(intUpdateTransporterTimer as
                 unsafe extern "C" fn(_: *mut _widget, _: *mut _w_context)
                     -> ());
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    //add the capacity label
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = 11012 as libc::c_int as UDWORD;
    sLabInit.id = 9500 as libc::c_int as UDWORD;
    sLabInit.style = 0 as libc::c_int as UDWORD;
    sLabInit.x = 65 as libc::c_int as SWORD;
    sLabInit.y = 1 as libc::c_int as SWORD;
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
    return 1 as libc::c_int;
}
/*
//add the Transporter timer into the top left hand corner of the screen
BOOL intAddTransporterTimer(void)
{
#ifndef PSX
	W_FORMINIT		sFormInit;
	W_LABINIT		sLabInit;
	W_BUTINIT		sButInit;

	// Add the background - invisible since the button image caters for this
	memset(&sFormInit, 0, sizeof(W_FORMINIT));
//#ifdef PSX
//	WidgSetOTIndex(OT2D_FARBACK);
//#endif
	sFormInit.formID = 0;
	sFormInit.id = IDTRANSTIMER_FORM;
	sFormInit.style = WFORM_PLAIN | WFORM_INVISIBLE;
	sFormInit.x = TRAN_FORM_X;
	sFormInit.y = TRAN_FORM_Y;
	sFormInit.width = iV_GetImageWidth(IntImages,IMAGE_TRANSETA_UP);//TRAN_FORM_WIDTH;
	sFormInit.height = iV_GetImageHeight(IntImages,IMAGE_TRANSETA_UP);//TRAN_FORM_HEIGHT;
	//sFormInit.pDisplay = intDisplayPlainForm;

	if (!widgAddForm(psWScreen, &sFormInit))
	{
		return FALSE;
	}

//#ifdef PSX
//	WidgSetOTIndex(OT2D_BACK);
//#endif

	//add labels for the time display
	memset(&sLabInit,0,sizeof(W_LABINIT));
	sLabInit.formID = IDTRANSTIMER_FORM;
	sLabInit.id = IDTRANTIMER_DISPLAY;
	sLabInit.style = WLAB_PLAIN | WIDG_HIDDEN;
	sLabInit.x = TRAN_TIMER_X;
	sLabInit.y = TRAN_TIMER_Y;
	sLabInit.width = TRAN_TIMER_WIDTH;
	sLabInit.height = sFormInit.height;//TRAN_TIMER_HEIGHT;
	//sLabInit.pText = "00.00";
	//if (mission.ETA < 0)
	//{
	//	sLabInit.pText = "00:00";
	//}
	//else
	//{
	//	fillTimeDisplay(sLabInit.pText, mission.ETA);
	}
	sLabInit.FontID = WFont;
	sLabInit.pCallback = intUpdateTransporterTimer;
	if (!widgAddLabel(psWScreen, &sLabInit))
	{
		return FALSE;
	}

//#ifdef PSX
//	WidgSetOTIndex(OT2D_FARBACK);
//#endif
	//set up button data
	memset(&sButInit, 0, sizeof(W_BUTINIT));
	sButInit.formID = IDTRANSTIMER_FORM;
	sButInit.id = IDTRANTIMER_BUTTON;
	sButInit.x = 0;//TRAN_FORM_X;
	sButInit.y = 0;//TRAN_FORM_Y;
	sButInit.width = sFormInit.width;
	sButInit.height = sFormInit.height;
	sButInit.FontID = WFont;
	sButInit.style = WBUT_PLAIN;
	//sButInit.pText = "T";
	sButInit.pTip = strresGetString(psStringRes, STR_INT_TRANSPORTER);
	sButInit.pDisplay = intDisplayImageHilight;
	sButInit.pUserData = (void*)PACKDWORD_TRI(0,IMAGE_TRANSETA_DOWN,
		IMAGE_TRANSETA_UP);
//#ifdef PSX
//	AddCursorSnap(&InterfaceSnap,
//					sFormInit.x+sButInit.x+sButInit.width/2,
//					sFormInit.y+sButInit.y+sButInit.height/2,sButInit.formID);
//#endif
	if (!widgAddButton(psWScreen, &sButInit))
	{
		return FALSE;
	}

#endif
	return TRUE;
}
*/
#[no_mangle]
pub unsafe extern "C" fn missionSetReinforcementTime(mut iTime: UDWORD) {
    g_iReinforceTime = iTime as SDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn missionGetReinforcementTime() -> UDWORD {
    return g_iReinforceTime as UDWORD;
}
//fills in a hours(if bHours = TRUE), minutes and seconds display for a given time in 1000th sec
unsafe extern "C" fn fillTimeDisplay(mut psText: *mut STRING,
                                     mut time: UDWORD, mut bHours: BOOL) {
    let mut calcTime: UDWORD = 0;
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    //this is only for the transporter timer - never have hours!
    if time == 99999900 as libc::c_int as libc::c_uint {
        let fresh0 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh0 as isize) = '-' as i32 as UBYTE as STRING;
        let fresh1 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh1 as isize) = '-' as i32 as UBYTE as STRING;
        //seperator
        let fresh2 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh2 as isize) = ':' as i32 as UBYTE as STRING;
        let fresh3 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh3 as isize) = '-' as i32 as UBYTE as STRING;
        let fresh4 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh4 as isize) = '-' as i32 as UBYTE as STRING;
        //terminate the timer text
        *psText.offset(inc as isize) = '\u{0}' as i32 as STRING
    } else {
        if bHours != 0 {
            //hours
            calcTime =
                time.wrapping_div((60 as libc::c_int * 60 as libc::c_int *
                                       1000 as libc::c_int) as libc::c_uint);
            let fresh5 = inc;
            inc = inc.wrapping_add(1);
            *psText.offset(fresh5 as isize) =
                ('0' as i32 as
                     libc::c_uint).wrapping_add(calcTime.wrapping_div(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
                    as UBYTE as STRING;
            let fresh6 = inc;
            inc = inc.wrapping_add(1);
            *psText.offset(fresh6 as isize) =
                ('0' as i32 as
                     libc::c_uint).wrapping_add(calcTime.wrapping_rem(10 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint))
                    as UBYTE as STRING;
            time =
                (time as
                     libc::c_uint).wrapping_sub(calcTime.wrapping_mul((60 as
                                                                           libc::c_int
                                                                           *
                                                                           60
                                                                               as
                                                                               libc::c_int
                                                                           *
                                                                           1000
                                                                               as
                                                                               libc::c_int)
                                                                          as
                                                                          libc::c_uint))
                    as UDWORD as UDWORD;
            //seperator
            let fresh7 = inc;
            inc = inc.wrapping_add(1);
            *psText.offset(fresh7 as isize) = ':' as i32 as UBYTE as STRING
        }
        //minutes
        calcTime =
            time.wrapping_div((60 as libc::c_int * 1000 as libc::c_int) as
                                  libc::c_uint);
        let fresh8 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh8 as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(calcTime.wrapping_div(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        let fresh9 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh9 as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(calcTime.wrapping_rem(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        time =
            (time as
                 libc::c_uint).wrapping_sub(calcTime.wrapping_mul((60 as
                                                                       libc::c_int
                                                                       *
                                                                       1000 as
                                                                           libc::c_int)
                                                                      as
                                                                      libc::c_uint))
                as UDWORD as UDWORD;
        //seperator
        let fresh10 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh10 as isize) = ':' as i32 as UBYTE as STRING;
        //seconds
        calcTime = time.wrapping_div(1000 as libc::c_int as libc::c_uint);
        let fresh11 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh11 as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(calcTime.wrapping_div(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        let fresh12 = inc;
        inc = inc.wrapping_add(1);
        *psText.offset(fresh12 as isize) =
            ('0' as i32 as
                 libc::c_uint).wrapping_add(calcTime.wrapping_rem(10 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint))
                as UBYTE as STRING;
        //terminate the timer text
        *psText.offset(inc as isize) = '\u{0}' as i32 as STRING
    };
}
//update function for the mission timer
unsafe extern "C" fn intUpdateMissionTimer(mut psWidget: *mut _widget,
                                           mut psContext: *mut _w_context) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL; //, calcTime;
    let mut timeElapsed: UDWORD = 0;
    let mut timeRemaining: SDWORD = 0;
    //take into account cheating with the mission timer
    //timeElapsed = gameTime - mission.startTime;
    //if the cheatTime has been set, then don't want the timer to countdown until stop cheating
    if mission.cheatTime != 0 {
        timeElapsed = mission.cheatTime.wrapping_sub(mission.startTime)
    } else { timeElapsed = gameTime.wrapping_sub(mission.startTime) }
    //check not gone over more than one hour - the mission should have been aborted?
	//if (timeElapsed > 60*60*GAME_TICKS_PER_SEC)
	//check not gone over more than 99 mins - the mission should have been aborted?
    //check not gone over more than 5 hours - arbitary number of hours
    if timeElapsed >
           (5 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int *
                1000 as libc::c_int) as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"You\'ve taken too long for this mission!\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"mission.c\x00" as *const u8 as *const libc::c_char,
                  3129 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"intUpdateMissionTimer\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    timeRemaining =
        (mission.time as libc::c_uint).wrapping_sub(timeElapsed) as SDWORD;
    if timeRemaining < 0 as libc::c_int { timeRemaining = 0 as libc::c_int }
    fillTimeDisplay((*Label).aText.as_mut_ptr(), timeRemaining as UDWORD,
                    1 as libc::c_int);
    //make sure its visible
    (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint;
    //make timer flash if time remaining < 5 minutes
    if timeRemaining <
           5 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int {
        flashMissionButton(11000 as libc::c_int as UDWORD);
    }
    //stop timer from flashing when gets to < 4 minutes
    if timeRemaining <
           4 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int {
        stopMissionButtonFlash(11000 as libc::c_int as UDWORD);
    }
    //play audio the first time the timed mission is started
    if timeRemaining != 0 &&
           missionCountDown as libc::c_int & 0x20 as libc::c_int != 0 {
        audio_QueueTrack(ID_SOUND_MISSION_TIMER_ACTIVATED as libc::c_int);
        missionCountDown =
            (missionCountDown as libc::c_int & !(0x20 as libc::c_int)) as
                UBYTE
    }
    //play some audio for mission countdown - start at 10 minutes remaining
    if getPlayCountDown() != 0 &&
           timeRemaining <
               10 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int {
        if timeRemaining <
               10 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int &&
               missionCountDown as libc::c_int & 0x10 as libc::c_int != 0 {
            audio_QueueTrack(ID_SOUND_10_MINUTES_REMAINING as libc::c_int);
            missionCountDown =
                (missionCountDown as libc::c_int & !(0x10 as libc::c_int)) as
                    UBYTE
        } else if timeRemaining <
                      5 as libc::c_int * 60 as libc::c_int *
                          1000 as libc::c_int &&
                      missionCountDown as libc::c_int & 0x8 as libc::c_int !=
                          0 {
            audio_QueueTrack(ID_SOUND_5_MINUTES_REMAINING as libc::c_int);
            missionCountDown =
                (missionCountDown as libc::c_int & !(0x8 as libc::c_int)) as
                    UBYTE
        } else if timeRemaining <
                      3 as libc::c_int * 60 as libc::c_int *
                          1000 as libc::c_int &&
                      missionCountDown as libc::c_int & 0x4 as libc::c_int !=
                          0 {
            audio_QueueTrack(ID_SOUND_3_MINUTES_REMAINING as libc::c_int);
            missionCountDown =
                (missionCountDown as libc::c_int & !(0x4 as libc::c_int)) as
                    UBYTE
        } else if timeRemaining <
                      2 as libc::c_int * 60 as libc::c_int *
                          1000 as libc::c_int &&
                      missionCountDown as libc::c_int & 0x2 as libc::c_int !=
                          0 {
            audio_QueueTrack(ID_SOUND_2_MINUTES_REMAINING as libc::c_int);
            missionCountDown =
                (missionCountDown as libc::c_int & !(0x2 as libc::c_int)) as
                    UBYTE
        } else if timeRemaining <
                      1 as libc::c_int * 60 as libc::c_int *
                          1000 as libc::c_int &&
                      missionCountDown as libc::c_int & 0x1 as libc::c_int !=
                          0 {
            audio_QueueTrack(ID_SOUND_1_MINUTE_REMAINING as libc::c_int);
            missionCountDown =
                (missionCountDown as libc::c_int & !(0x1 as libc::c_int)) as
                    UBYTE
        }
    };
}
//update function for the transporter timer
unsafe extern "C" fn intUpdateTransporterTimer(mut psWidget: *mut _widget,
                                               mut psContext:
                                                   *mut _w_context) {
    let mut Label: *mut W_LABEL = psWidget as *mut W_LABEL;
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut timeRemaining: SDWORD = 0;
    let mut ETA: SDWORD = 0;
    ETA = mission.ETA;
    if ETA < 0 as libc::c_int { ETA = 0 as libc::c_int }
    // Get the object associated with this widget.
    psTransporter = (*Label).pUserData as *mut DROID;
    if !psTransporter.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"intUpdateTransporterTimer: invalid Droid pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"mission.c\x00" as *const u8 as *const libc::c_char,
                  3214 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"intUpdateTransporterTimer\x00")).as_ptr(),
                  b"PTRVALID(psTransporter, sizeof(DROID))\x00" as *const u8
                      as *const libc::c_char);
        };
        if (*psTransporter).action == DACTION_TRANSPORTIN as libc::c_int ||
               (*psTransporter).action ==
                   DACTION_TRANSPORTWAITTOFLYIN as libc::c_int {
            if mission.ETA == 99999900 as libc::c_int {
                timeRemaining = 99999900 as libc::c_int
            } else {
                timeRemaining =
                    (mission.ETA as
                         libc::c_uint).wrapping_sub(gameTime.wrapping_sub(g_iReinforceTime
                                                                              as
                                                                              libc::c_uint))
                        as SDWORD;
                if timeRemaining < 0 as libc::c_int {
                    timeRemaining = 0 as libc::c_int
                }
                if timeRemaining < 10 as libc::c_int * 1000 as libc::c_int {
                    // arrived: tell the transporter to move to the new onworld
				    // location if not already doing so
                    if (*psTransporter).action ==
                           DACTION_TRANSPORTWAITTOFLYIN as libc::c_int {
                        missionFlyTransportersIn(selectedPlayer as SDWORD,
                                                 0 as libc::c_int);
                        eventFireCallbackTrigger(CALL_TRANSPORTER_REINFORCE as
                                                     libc::c_int as
                                                     TRIGGER_TYPE);
                    }
                }
            }
            fillTimeDisplay((*Label).aText.as_mut_ptr(),
                            timeRemaining as UDWORD, 0 as libc::c_int);
        } else {
            fillTimeDisplay((*Label).aText.as_mut_ptr(), ETA as UDWORD,
                            0 as libc::c_int);
        }
    } else if missionCanReEnforce() != 0 {
        // ((mission.type == LDS_MKEEP) || (mission.type == LDS_MCLEAR)) & (mission.ETA >= 0) ) {
        fillTimeDisplay((*Label).aText.as_mut_ptr(), ETA as UDWORD,
                        0 as libc::c_int);
    } else {
        fillTimeDisplay((*Label).aText.as_mut_ptr(),
                        0 as libc::c_int as UDWORD, 0 as libc::c_int);
    }
    //minutes
	/*calcTime = timeRemaining / (60*GAME_TICKS_PER_SEC);
	Label->aText[0] = (UBYTE)('0'+ calcTime / 10);
	Label->aText[1] = (UBYTE)('0'+ calcTime % 10);
	timeElapsed -= calcTime * (60*GAME_TICKS_PER_SEC);
	//seperator
	Label->aText[3] = (UBYTE)(':');
	//seconds
	calcTime = timeRemaining / GAME_TICKS_PER_SEC;
	Label->aText[3] = (UBYTE)('0'+ calcTime / 10);
	Label->aText[4] = (UBYTE)('0'+ calcTime % 10);*/
    (*Label).style &= !(0x8000 as libc::c_int) as libc::c_uint;
}
/* Remove the Mission Timer widgets from the screen*/
#[no_mangle]
pub unsafe extern "C" fn intRemoveMissionTimer() {
    // Check it's up.
    if !widgGetFromID(psWScreen, 11000 as libc::c_int as UDWORD).is_null() {
        //and remove it.
        widgDelete(psWScreen, 11000 as libc::c_int as UDWORD);
    };
}
/* Remove the Transporter Timer widgets from the screen*/
#[no_mangle]
pub unsafe extern "C" fn intRemoveTransporterTimer() {
    //remove main screen
    if !widgGetFromID(psWScreen, 11012 as libc::c_int as UDWORD).is_null() {
        widgDelete(psWScreen, 11012 as libc::c_int as UDWORD);
    };
}
// ////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////
// mission result functions for the interface.
#[no_mangle]
pub unsafe extern "C" fn intDisplayMissionBackDrop(mut psWidget: *mut _widget,
                                                   mut xOffset: UDWORD,
                                                   mut yOffset: UDWORD,
                                                   mut pColours:
                                                       *mut UDWORD) {
    //	iV_DownloadDisplayBuffer(pMissionBackDrop->bmp);
    scoreDataToScreen();
}
#[no_mangle]
pub unsafe extern "C" fn missionResetInGameState() {
    //stop the game if in single player mode
    setMissionPauseState();
    // reset the input state
    resetInput();
    // Add the background
	// get rid of reticule etc..
    intResetScreen(0 as libc::c_int);
    //intHidePowerBar();
    forceHidePowerBar();
    intRemoveReticule();
    intRemoveMissionTimer();
}
unsafe extern "C" fn _intAddMissionResult(mut result: BOOL,
                                          mut bPlaySuccess: BOOL) -> BOOL {
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
    //	UDWORD			fileSize=0;
    missionResetInGameState();
    memset(&mut sFormInit as *mut W_FORMINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_FORMINIT>() as libc::c_ulong);
    // add some funky beats
    cdAudio_PlayTrack(2 as libc::c_int); // 2= frontend music.
    pie_LoadBackDrop(SCREEN_MISSIONEND, 0 as libc::c_int);
    sFormInit.formID = 0 as libc::c_int as UDWORD;
    sFormInit.id = 11013 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x =
        (0 as libc::c_int as
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
        (0 as libc::c_int as
             libc::c_uint).wrapping_add(pie_GetVideoBufferHeight().wrapping_sub(480
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_uint))
            as SWORD;
    sFormInit.width = 640 as libc::c_int as UWORD;
    sFormInit.height = 480 as libc::c_int as UWORD;
    sFormInit.pDisplay =
        Some(intDisplayMissionBackDrop as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //#endif
    // TITLE
    sFormInit.formID = 11013 as libc::c_int as UDWORD; //intDisplayPlainForm;
    sFormInit.id = 11014 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 20 as libc::c_int as SWORD;
    sFormInit.y = 20 as libc::c_int as SWORD;
    sFormInit.width = 600 as libc::c_int as UWORD;
    sFormInit.height = 40 as libc::c_int as UWORD;
    sFormInit.disableChildren = 1 as libc::c_int;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    // removed for more space
//#ifdef PSX
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    //#endif
    // add form
    sFormInit.formID = 11013 as libc::c_int as UDWORD; //intDisplayPlainForm;
    sFormInit.id = 11002 as libc::c_int as UDWORD;
    sFormInit.style = 0 as libc::c_int as UDWORD;
    sFormInit.x = 20 as libc::c_int as SWORD;
    sFormInit.y = 380 as libc::c_int as SWORD;
    sFormInit.width = 600 as libc::c_int as UWORD;
    sFormInit.height = 80 as libc::c_int as UWORD;
    sFormInit.disableChildren = 1 as libc::c_int;
    sFormInit.pDisplay =
        Some(intOpenPlainForm as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    if widgAddForm(psWScreen, &mut sFormInit) == 0 { return 0 as libc::c_int }
    // description of success/fail
    memset(&mut sLabInit as *mut W_LABINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_LABINIT>() as libc::c_ulong);
    sLabInit.formID = 11014 as libc::c_int as UDWORD;
    sLabInit.id = 11004 as libc::c_int as UDWORD;
    sLabInit.style = (0 as libc::c_int | 2 as libc::c_int) as UDWORD;
    sLabInit.x = 0 as libc::c_int as SWORD;
    sLabInit.y = 12 as libc::c_int as SWORD;
    sLabInit.width = 600 as libc::c_int as UWORD;
    sLabInit.height = 16 as libc::c_int as UWORD;
    if result != 0 {
        //don't bother adding the text if haven't played the audio
        if bPlaySuccess != 0 {
            sLabInit.pText =
                strresGetString(psStringRes,
                                STR_MR_OBJECTIVE_ACHIEVED as libc::c_int as
                                    UDWORD)
            //"Objective Achieved";
        }
    } else {
        sLabInit.pText =
            strresGetString(psStringRes,
                            STR_MR_OBJECTIVE_FAILED as libc::c_int as UDWORD)
        //"Objective Failed;
    }
    sLabInit.FontID = WFont;
    //removed for more space
//#ifdef PSX
    if widgAddLabel(psWScreen, &mut sLabInit) == 0 { return 0 as libc::c_int }
    //#endif
	// options.
    memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
    sButInit.formID = 11002 as libc::c_int as UDWORD;
    sButInit.style = (0 as libc::c_int | 64 as libc::c_int) as UDWORD;
    sButInit.width = (600 as libc::c_int - 10 as libc::c_int) as UWORD;
    sButInit.height = 16 as libc::c_int as UWORD;
    sButInit.FontID = WFont;
    sButInit.pTip = 0 as *mut STRING;
    sButInit.pDisplay =
        Some(displayTextOption as
                 unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                      _: *mut UDWORD) -> ());
    //if won or in debug mode
    if result != 0 || getDebugMappingStatus() != 0 {
        //continue
        sButInit.x = 5 as libc::c_int as SWORD;
        // Won the game, so display "Quit to main menu"
        if testPlayerHasWon() != 0 {
            sButInit.id = 11007 as libc::c_int as UDWORD;
            sButInit.y = (35 as libc::c_int - 8 as libc::c_int) as SWORD;
            sButInit.pText =
                strresGetString(psStringRes,
                                STR_MR_QUIT_TO_MAIN as libc::c_int as UDWORD);
            widgAddButton(psWScreen, &mut sButInit);
            intSetCurrentCursorPosition(&mut InterfaceSnap, sButInit.id);
        } else {
            // Finished the mission, so display "Continue Game"
            sButInit.y = 35 as libc::c_int as SWORD; //"Continue Game";
            sButInit.id = 11008 as libc::c_int as UDWORD;
            sButInit.pText =
                strresGetString(psStringRes,
                                STR_MR_CONTINUE as libc::c_int as UDWORD);
            widgAddButton(psWScreen, &mut sButInit);
            intSetCurrentCursorPosition(&mut InterfaceSnap, sButInit.id);
        }
        /* Only add save option if in the game for real, ie, not fastplay.
        And the player hasn't just completed the whole game
        Don't add save option if just lost and in debug mode*/
        if testPlayerHasWon() == 0 &&
               !(testPlayerHasLost() != 0 && getDebugMappingStatus() != 0) {
            //save
            sButInit.id = 11006 as libc::c_int as UDWORD; //"Save Game";
            sButInit.x = 5 as libc::c_int as SWORD;
            sButInit.y = 15 as libc::c_int as SWORD;
            sButInit.pText =
                strresGetString(psStringRes,
                                STR_MR_SAVE_GAME as libc::c_int as UDWORD);
            widgAddButton(psWScreen, &mut sButInit);
            intSetCurrentCursorPosition(&mut InterfaceSnap, sButInit.id);
        }
    } else {
        //load
        sButInit.id = 11005 as libc::c_int as UDWORD; //"Load Saved Game";
        sButInit.x = 5 as libc::c_int as SWORD;
        sButInit.y = 15 as libc::c_int as SWORD;
        sButInit.pText =
            strresGetString(psStringRes,
                            STR_MR_LOAD_GAME as libc::c_int as UDWORD);
        widgAddButton(psWScreen, &mut sButInit);
        intSetCurrentCursorPosition(&mut InterfaceSnap, sButInit.id);
        //quit
        sButInit.id = 11007 as libc::c_int as UDWORD; //"Quit to Main Menu";
        sButInit.x = 5 as libc::c_int as SWORD;
        sButInit.y = 35 as libc::c_int as SWORD;
        sButInit.pText =
            strresGetString(psStringRes,
                            STR_MR_QUIT_TO_MAIN as libc::c_int as UDWORD);
        widgAddButton(psWScreen, &mut sButInit);
    }
    intMode = INT_MISSIONRES;
    MissionResUp = 1 as libc::c_int;
    /* play result audio */
    if result == 1 as libc::c_int && bPlaySuccess != 0 {
        audio_QueueTrack(ID_SOUND_OBJECTIVE_ACCOMPLISHED as libc::c_int);
    }
    return 1 as libc::c_int;
}
//result screen functions
#[no_mangle]
pub unsafe extern "C" fn intAddMissionResult(mut result: BOOL,
                                             mut bPlaySuccess: BOOL) -> BOOL {
    /* save result */
    g_bMissionResult = result;
    return _intAddMissionResult(result, bPlaySuccess);
}
#[no_mangle]
pub unsafe extern "C" fn intRemoveMissionResultNoAnim() {
    widgDelete(psWScreen, 11014 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 11002 as libc::c_int as UDWORD);
    widgDelete(psWScreen, 11013 as libc::c_int as UDWORD);
    cdAudio_Stop();
    MissionResUp = 0 as libc::c_int;
    ClosingMissionRes = 0 as libc::c_int;
    intMode = INT_NORMAL;
    //reset the pauses
    resetMissionPauseState();
    // add back the reticule and power bar.
    intAddReticule();
    intShowPowerBar();
    //	EnableMouseDraw(TRUE);
//	MouseMovement(TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn intRunMissionResult() {
    processFrontendSnap(0 as libc::c_int);
    pie_SetMouse(IntImages, IMAGE_CURSOR_DEFAULT as libc::c_int as UWORD);
    frameSetCursorFromRes(108 as libc::c_int as WORD);
    if bLoadSaveUp != 0 {
        if runLoadSave(0 as libc::c_int) != 0 {
            // check for file name.
            if strlen(sRequestResult.as_mut_ptr()) != 0 {
                debug(LOG_NEVER,
                      b"Returned %s\x00" as *const u8 as *const libc::c_char,
                      sRequestResult.as_mut_ptr());
                if !(bRequestLoad != 0) {
                    saveGame(sRequestResult.as_mut_ptr(),
                             GTYPE_SAVE_START as libc::c_int);
                    addConsoleMessage(strresGetString(psStringRes,
                                                      STR_GAME_SAVED as
                                                          libc::c_int as
                                                          UDWORD),
                                      LEFT_JUSTIFY);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn missionCDCancelPressed() {
    intAddMissionResult(g_bMissionResult, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn missionContineButtonPressed() {
    //SHOULDN'T BE ABLE TO BE ANY OTHER TYPE AT PRESENT!
	// start the next mission if necessary
	// otherwise wait for the Launch button to be pressed
	//if (nextMissionType == MISSION_CAMPSTART OR nextMissionType ==
	//	MISSION_CAMPEXPAND OR nextMissionType == MISSION_BETWEEN)
    if nextMissionType == LDS_CAMSTART as libc::c_int ||
           nextMissionType == LDS_BETWEEN as libc::c_int ||
           nextMissionType == LDS_EXPAND as libc::c_int ||
           nextMissionType == LDS_EXPAND_LIMBO as libc::c_int {
        //if we're moving from cam2-cam3?
        launchMission();
    }
    widgDelete(psWScreen, 11002 as libc::c_int as UDWORD);
    //close option box.
    //if (nextMissionType == MISSION_OFFKEEP OR nextMissionType == MISSION_OFFCLEAR)
	/*if (nextMissionType == MISSION_BETWEEN)
	{
		intRemoveMissionResultNoAnim();
	}*/
//	intRemoveMissionResultNoAnim();
    //just being paranoid here - definately don't want this in the final build
}
#[no_mangle]
pub unsafe extern "C" fn intProcessMissionResult(mut id: UDWORD) {
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
    match id {
        11005 => {
            // throw up some filerequester
            addLoadSave(LOAD_MISSIONEND, SaveGamePath.as_mut_ptr(),
                        b"gam\x00" as *const u8 as *const libc::c_char as
                            *mut CHAR,
                        strresGetString(psStringRes,
                                        STR_MR_LOAD_GAME as libc::c_int as
                                            UDWORD));
        }
        11006 => {
            addLoadSave(SAVE_MISSIONEND, SaveGamePath.as_mut_ptr(),
                        b"gam\x00" as *const u8 as *const libc::c_char as
                            *mut CHAR,
                        strresGetString(psStringRes,
                                        STR_MR_SAVE_GAME as libc::c_int as
                                            UDWORD));
            if widgGetFromID(psWScreen,
                             11007 as libc::c_int as UDWORD).is_null() {
                //Add Quit Button now save has been pressed
                memset(&mut sButInit as *mut W_BUTINIT as *mut libc::c_void,
                       0 as libc::c_int,
                       ::std::mem::size_of::<W_BUTINIT>() as libc::c_ulong);
                sButInit.formID = 11002 as libc::c_int as UDWORD;
                sButInit.style =
                    (0 as libc::c_int | 64 as libc::c_int) as UDWORD;
                sButInit.width =
                    (600 as libc::c_int - 10 as libc::c_int) as UWORD;
                sButInit.height = 16 as libc::c_int as UWORD;
                sButInit.FontID = WFont;
                sButInit.pTip = 0 as *mut STRING;
                sButInit.pDisplay =
                    Some(displayTextOption as
                             unsafe extern "C" fn(_: *mut _widget, _: UDWORD,
                                                  _: UDWORD, _: *mut UDWORD)
                                 -> ());
                sButInit.id = 11007 as libc::c_int as UDWORD;
                sButInit.x = 5 as libc::c_int as SWORD;
                sButInit.y = 55 as libc::c_int as SWORD;
                sButInit.pText =
                    strresGetString(psStringRes,
                                    STR_MR_QUIT_TO_MAIN as libc::c_int as
                                        UDWORD);
                widgAddButton(psWScreen, &mut sButInit);
            }
        }
        11008 => {
            if bLoadSaveUp != 0 {
                closeLoadSave();
                // close save interface if it's up.
            }
            missionContineButtonPressed();
        }
        11007 | _ => { }
    };
}
// end of interface stuff.
// ////////////////////////////////////////////////////////////
// ////////////////////////////////////////////////////////////
/*builds a droid back at the home base whilst on a mission - stored in a list made
available to the transporter interface*/
#[no_mangle]
pub unsafe extern "C" fn buildMissionDroid(mut psTempl: *mut DROID_TEMPLATE,
                                           mut x: UDWORD, mut y: UDWORD,
                                           mut player_0: UDWORD)
 -> *mut DROID {
    let mut psNewDroid: *mut DROID = 0 as *mut DROID;
    psNewDroid =
        buildDroid(psTempl, x << 7 as libc::c_int, y << 7 as libc::c_int,
                   player_0, 1 as libc::c_int);
    if psNewDroid.is_null() { return 0 as *mut DROID }
    //addDroid(psNewDroid, mission.apsBuiltDroids);
    addDroid(psNewDroid, mission.apsDroidLists.as_mut_ptr());
    //set its x/y to impossible values so can detect when return from mission
    (*psNewDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
    (*psNewDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
    //set all the droids to selected from when return
    (*psNewDroid).selected = 1 as libc::c_int as UBYTE;
    return psNewDroid;
}
//this causes the new mission data to be loaded up - only if startMission has been called
#[no_mangle]
pub unsafe extern "C" fn launchMission() {
    //if (mission.type == MISSION_NONE)
    if mission.type_0 == LDS_NONE as libc::c_int as libc::c_uint {
        // tell the loop that a new level has to be loaded up
        loopMissionState = LMS_NEWLEVEL
    } else {
        debug(LOG_NEVER,
              b"Start Mission has not been called\x00" as *const u8 as
                  *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn intCDOK() {
    resetMissionPauseState();
    intAddReticule();
    intShowPowerBar();
    launchMission();
}
#[no_mangle]
pub unsafe extern "C" fn intCDCancel() {
    /* do nothing - dealt with in HCI */
}
//sets up the game to start a new mission
//BOOL setUpMission(MISSION_TYPE type)
#[no_mangle]
pub unsafe extern "C" fn setUpMission(mut type_0: UDWORD) -> BOOL {
    /*#ifndef PSX
	CD_INDEX	CDrequired;
#endif*/
    //MISSION_TYPE	oldMission;
    //close the interface
    intResetScreen(1 as libc::c_int);
    //oldMission = mission.type;
	/*the last mission must have been successful otherwise endgame would have
	been called*/
    endMission();
    //release the level data for the previous mission
    if levReleaseMissionData() == 0 { return 0 as libc::c_int }
    //if (type == MISSION_OFFCLEAR OR type == MISSION_OFFKEEP)
    if type_0 == LDS_CAMSTART as libc::c_int as libc::c_uint {
        //this cannot be called here since we need to be able to save the game at the end of cam1 and cam2
		/*CDrequired = getCDForCampaign( getCampaignNumber() );
		if ( cdspan_CheckCDPresent(CDrequired) )*/
        //another one of those lovely hacks!!
        let mut bPlaySuccess: BOOL = 1 as libc::c_int;
        if getCampaignNumber() == 2 as libc::c_int as libc::c_uint {
            bPlaySuccess = 0 as libc::c_int
        }
        if intAddMissionResult(1 as libc::c_int, bPlaySuccess) == 0 {
            return 0 as libc::c_int
        }
        loopMissionState = LMS_SAVECONTINUE
    } else if type_0 == LDS_MKEEP as libc::c_int as libc::c_uint ||
                  type_0 == LDS_MCLEAR as libc::c_int as libc::c_uint ||
                  type_0 == LDS_MKEEP_LIMBO as libc::c_int as libc::c_uint {
        launchMission();
    } else {
        if getWidgetsStatus() == 0 {
            setWidgetsStatus(1 as libc::c_int);
            intResetScreen(0 as libc::c_int);
        }
        //we don't want the 'mission accomplished' audio/text message at end of cam1
        //give the option of save/continue
        /*else
		{
			if(!getWidgetsStatus())
			{
				setWidgetsStatus(TRUE);
				intResetScreen(FALSE);
			}
			missionResetInGameState();
			addCDChangeInterface( CDrequired, intCDOK, intCDCancel );
			loopMissionState = LMS_SAVECONTINUE;
		}*/
        //give the option of save/continue
        if intAddMissionResult(1 as libc::c_int, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        loopMissionState = LMS_SAVECONTINUE
    }
    //if current mission is 'between' then don't give option to save/continue again
	/*if (oldMission != MISSION_BETWEEN)
	{
		//give the option of save/continue
		if (!intAddMissionResult(TRUE))
		{
			return FALSE;
		}
	}*/
    return 1 as libc::c_int;
}
//save the power settings before loading in the new map data
unsafe extern "C" fn saveMissionPower() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //mission.asPower[inc].initialPower = asPower[inc]->initialPower;
        mission.asPower[inc as usize].extractedPower =
            (*asPower[inc as usize]).extractedPower;
        mission.asPower[inc as usize].currentPower =
            (*asPower[inc as usize]).currentPower;
        inc = inc.wrapping_add(1)
    };
}
//add the power from the home base to the current power levels for the mission map
unsafe extern "C" fn adjustMissionPower() {
    let mut inc: UDWORD = 0;
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        //asPower[inc]->initialPower += mission.asPower[inc].initialPower;
        (*asPower[inc as usize]).extractedPower =
            ((*asPower[inc as usize]).extractedPower as
                 libc::c_uint).wrapping_add(mission.asPower[inc as
                                                                usize].extractedPower)
                as UDWORD as UDWORD;
        (*asPower[inc as usize]).currentPower =
            ((*asPower[inc as usize]).currentPower as
                 libc::c_uint).wrapping_add(mission.asPower[inc as
                                                                usize].currentPower)
                as UDWORD as UDWORD;
        inc = inc.wrapping_add(1)
    };
}
/*sets the appropriate pause states for when the interface is up but the
game needs to be paused*/
#[no_mangle]
pub unsafe extern "C" fn setMissionPauseState() {
    if bMultiPlayer == 0 {
        gameTimeStop();
        setGameUpdatePause(1 as libc::c_int);
        setAudioPause(1 as libc::c_int);
        setScriptPause(1 as libc::c_int);
        setConsolePause(1 as libc::c_int);
    };
}
/*resets the pause states */
#[no_mangle]
pub unsafe extern "C" fn resetMissionPauseState() {
    if bMultiPlayer == 0 {
        setGameUpdatePause(0 as libc::c_int);
        setAudioPause(0 as libc::c_int);
        setScriptPause(0 as libc::c_int);
        setConsolePause(0 as libc::c_int);
        gameTimeStart();
    };
}
//gets the coords for a no go area
#[no_mangle]
pub unsafe extern "C" fn getLandingZone(mut i: SDWORD) -> *mut LANDING_ZONE {
    if i >= 0 as libc::c_int && i < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"getLandingZone out of range.\x00" as *const u8 as
                  *const libc::c_char);
    };
    if i >= 0 as libc::c_int && i < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              3902 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"getLandingZone\x00")).as_ptr(),
              b"(i >= 0) && (i < MAX_NOGO_AREAS)\x00" as *const u8 as
                  *const libc::c_char);
    };
    return &mut *sLandingZone.as_mut_ptr().offset(i as isize) as
               *mut LANDING_ZONE;
}
/*Initialises all the nogo areas to 0 - DOESN'T INIT THE LIMBO AREA because we
have to set this up in the mission BEFORE*/
#[no_mangle]
pub unsafe extern "C" fn initNoGoAreas() {
    let mut i: UBYTE = 0;
    i = 0 as libc::c_int as UBYTE;
    while (i as libc::c_int) < 9 as libc::c_int {
        if i as libc::c_int != 8 as libc::c_int {
            sLandingZone[i as usize].y2 = 0 as libc::c_int as UBYTE;
            sLandingZone[i as usize].x2 = sLandingZone[i as usize].y2;
            sLandingZone[i as usize].y1 = sLandingZone[i as usize].x2;
            sLandingZone[i as usize].x1 = sLandingZone[i as usize].y1
        }
        i = i.wrapping_add(1)
    };
}
//sets the coords for the Transporter to land (for player 0 - selectedPlayer)
#[no_mangle]
pub unsafe extern "C" fn setLandingZone(mut x1: UBYTE, mut y1: UBYTE,
                                        mut x2: UBYTE, mut y2: UBYTE) {
    //quick check that x2 > x1 and y2 > y1
    if (x2 as libc::c_int) < x1 as libc::c_int {
        sLandingZone[0 as libc::c_int as usize].x1 = x2;
        sLandingZone[0 as libc::c_int as usize].x2 = x1
    } else {
        sLandingZone[0 as libc::c_int as usize].x1 = x1;
        sLandingZone[0 as libc::c_int as usize].x2 = x2
    }
    if (y2 as libc::c_int) < y1 as libc::c_int {
        sLandingZone[0 as libc::c_int as usize].y1 = y2;
        sLandingZone[0 as libc::c_int as usize].y2 = y1
    } else {
        sLandingZone[0 as libc::c_int as usize].y1 = y1;
        sLandingZone[0 as libc::c_int as usize].y2 = y2
    }
    addLandingLights((getLandingX(0 as libc::c_int) as libc::c_int +
                          64 as libc::c_int) as UDWORD,
                     (getLandingY(0 as libc::c_int) as libc::c_int +
                          64 as libc::c_int) as UDWORD);
}
//sets the coords for a no go area
#[no_mangle]
pub unsafe extern "C" fn setNoGoArea(mut x1: UBYTE, mut y1: UBYTE,
                                     mut x2: UBYTE, mut y2: UBYTE,
                                     mut area: UBYTE) {
    //quick check that x2 > x1 and y2 > y1
    if (x2 as libc::c_int) < x1 as libc::c_int {
        sLandingZone[area as usize].x1 = x2;
        sLandingZone[area as usize].x2 = x1
    } else {
        sLandingZone[area as usize].x1 = x1;
        sLandingZone[area as usize].x2 = x2
    }
    if (y2 as libc::c_int) < y1 as libc::c_int {
        sLandingZone[area as usize].y1 = y2;
        sLandingZone[area as usize].y2 = y1
    } else {
        sLandingZone[area as usize].y1 = y1;
        sLandingZone[area as usize].y2 = y2
    }
    if area as libc::c_int == 0 as libc::c_int {
        addLandingLights((getLandingX(area as SDWORD) as libc::c_int +
                              64 as libc::c_int) as UDWORD,
                         (getLandingY(area as SDWORD) as libc::c_int +
                              64 as libc::c_int) as UDWORD);
    };
}
//FUNCTIONS**************
unsafe extern "C" fn addLandingLights(mut x: UDWORD, mut y: UDWORD) {
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,}; //middle
    pos.x = x as int32; //outer
    pos.z = y as int32; // inner
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_MIDDLE);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 1 as libc::c_int);
    pos.x = x.wrapping_add(128 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_add(128 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_OUTER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_add(128 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_sub(128 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_OUTER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_sub(128 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_add(128 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_OUTER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_sub(128 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_sub(128 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_OUTER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_add(64 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_add(64 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_INNER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_add(64 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_sub(64 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_INNER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_sub(64 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_add(64 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_INNER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
    pos.x = x.wrapping_sub(64 as libc::c_int as libc::c_uint) as int32;
    pos.z = y.wrapping_sub(64 as libc::c_int as libc::c_uint) as int32;
    pos.y =
        map_Height(pos.x as UDWORD, pos.z as UDWORD) as libc::c_int +
            16 as libc::c_int;
    effectSetLandLightSpec(LL_INNER);
    addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_LAND_LIGHT,
              0 as libc::c_int, 0 as *mut iIMDShape, 0 as libc::c_int);
}
/*	checks the x,y passed in are not within the boundary of any Landing Zone
	x and y in tile coords*/
#[no_mangle]
pub unsafe extern "C" fn withinLandingZone(mut x: UDWORD, mut y: UDWORD)
 -> BOOL {
    let mut inc: UDWORD = 0;
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"withinLandingZone: x coord bigger than mapWidth\x00" as
                  *const u8 as *const libc::c_char);
    };
    if x < mapWidth {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4039 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"withinLandingZone\x00")).as_ptr(),
              b"x < mapWidth\x00" as *const u8 as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"withinLandingZone: y coord bigger than mapHeight\x00" as
                  *const u8 as *const libc::c_char);
    };
    if y < mapHeight {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4040 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"withinLandingZone\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    inc = 0 as libc::c_int as UDWORD;
    while inc < 9 as libc::c_int as libc::c_uint {
        if x >= sLandingZone[inc as usize].x1 as UDWORD &&
               x <= sLandingZone[inc as usize].x2 as UDWORD &&
               (y >= sLandingZone[inc as usize].y1 as UDWORD &&
                    y <= sLandingZone[inc as usize].y2 as UDWORD) {
            return 1 as libc::c_int
        }
        inc = inc.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
//returns the x coord for where the Transporter can land (for player 0)
#[no_mangle]
pub unsafe extern "C" fn getLandingX(mut iPlayer: SDWORD) -> UWORD {
    if iPlayer < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"getLandingX: player %d out of range\x00" as *const u8 as
                  *const libc::c_char, iPlayer);
    };
    if iPlayer < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4057 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"getLandingX\x00")).as_ptr(),
              b"iPlayer<MAX_NOGO_AREAS\x00" as *const u8 as
                  *const libc::c_char);
    };
    return ((sLandingZone[iPlayer as usize].x1 as libc::c_int +
                 (sLandingZone[iPlayer as usize].x2 as libc::c_int -
                      sLandingZone[iPlayer as usize].x1 as libc::c_int) /
                     2 as libc::c_int) << 7 as libc::c_int) as UWORD;
}
//returns the y coord for where the Transporter can land
#[no_mangle]
pub unsafe extern "C" fn getLandingY(mut iPlayer: SDWORD) -> UWORD {
    if iPlayer < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"getLandingY: player %d out of range\x00" as *const u8 as
                  *const libc::c_char, iPlayer);
    };
    if iPlayer < 9 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4065 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"getLandingY\x00")).as_ptr(),
              b"iPlayer<MAX_NOGO_AREAS\x00" as *const u8 as
                  *const libc::c_char);
    };
    return ((sLandingZone[iPlayer as usize].y1 as libc::c_int +
                 (sLandingZone[iPlayer as usize].y2 as libc::c_int -
                      sLandingZone[iPlayer as usize].y1 as libc::c_int) /
                     2 as libc::c_int) << 7 as libc::c_int) as UWORD;
}
//returns the x coord for where the Transporter can land back at home base
unsafe extern "C" fn getHomeLandingX() -> UDWORD {
    //return ((mission.homeLZ.x1 + (mission.homeLZ.x2 -
	//	mission.homeLZ.x1)/2) << TILE_SHIFT);
    return (mission.homeLZ_X as libc::c_int >> 7 as libc::c_int) as UDWORD;
}
//returns the y coord for where the Transporter can land back at home base
unsafe extern "C" fn getHomeLandingY() -> UDWORD {
    //return ((mission.homeLZ.y1 + (mission.homeLZ.y2 -
	//	mission.homeLZ.y1)/2) << TILE_SHIFT);
    return (mission.homeLZ_Y as libc::c_int >> 7 as libc::c_int) as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn missionSetTransporterEntry(mut iPlayer: SDWORD,
                                                    mut iEntryTileX: SDWORD,
                                                    mut iEntryTileY: SDWORD) {
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"missionSetTransporterEntry: player %i too high\x00" as
                  *const u8 as *const libc::c_char, iPlayer);
    };
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4088 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"missionSetTransporterEntry\x00")).as_ptr(),
              b"iPlayer<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if iEntryTileX > scrollMinX && iEntryTileX < scrollMaxX {
        mission.iTranspEntryTileX[iPlayer as usize] = iEntryTileX as UWORD
    } else {
        debug(LOG_NEVER,
              b"missionSetTransporterEntry: entry point x %i outside scroll limits %i->%i\n\x00"
                  as *const u8 as *const libc::c_char, iEntryTileX,
              scrollMinX, scrollMaxX);
        mission.iTranspEntryTileX[iPlayer as usize] =
            (scrollMinX + 1 as libc::c_int) as UWORD
    }
    if iEntryTileY > scrollMinY && iEntryTileY < scrollMaxY {
        mission.iTranspEntryTileY[iPlayer as usize] = iEntryTileY as UWORD
    } else {
        debug(LOG_NEVER,
              b"missionSetTransporterEntry: entry point y %i outside scroll limits %i->%i\n\x00"
                  as *const u8 as *const libc::c_char, iEntryTileY,
              scrollMinY, scrollMaxY);
        mission.iTranspEntryTileY[iPlayer as usize] =
            (scrollMinY + 1 as libc::c_int) as UWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn missionSetTransporterExit(mut iPlayer: SDWORD,
                                                   mut iExitTileX: SDWORD,
                                                   mut iExitTileY: SDWORD) {
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"missionSetTransporterExit: player %i too high\x00" as
                  *const u8 as *const libc::c_char, iPlayer);
    };
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4113 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"missionSetTransporterExit\x00")).as_ptr(),
              b"iPlayer<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    if iExitTileX > scrollMinX && iExitTileX < scrollMaxX {
        mission.iTranspExitTileX[iPlayer as usize] = iExitTileX as UWORD
    } else {
        debug(LOG_NEVER,
              b"missionSetTransporterExit: entry point x %i outside scroll limits %i->%i\n\x00"
                  as *const u8 as *const libc::c_char, iExitTileX, scrollMinX,
              scrollMaxX);
        mission.iTranspExitTileX[iPlayer as usize] =
            (scrollMinX + 1 as libc::c_int) as UWORD
    }
    if iExitTileY > scrollMinY && iExitTileY < scrollMaxY {
        mission.iTranspExitTileY[iPlayer as usize] = iExitTileY as UWORD
    } else {
        debug(LOG_NEVER,
              b"missionSetTransporterExit: entry point y %i outside scroll limits %i->%i\n\x00"
                  as *const u8 as *const libc::c_char, iExitTileY, scrollMinY,
              scrollMaxY);
        mission.iTranspExitTileY[iPlayer as usize] =
            (scrollMinY + 1 as libc::c_int) as UWORD
    };
}
#[no_mangle]
pub unsafe extern "C" fn missionGetTransporterEntry(mut iPlayer: SDWORD,
                                                    mut iX: *mut UWORD,
                                                    mut iY: *mut UWORD) {
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"missionGetTransporterEntry: player %i too high\x00" as
                  *const u8 as *const libc::c_char, iPlayer);
    };
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4138 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"missionGetTransporterEntry\x00")).as_ptr(),
              b"iPlayer<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    *iX =
        ((mission.iTranspEntryTileX[iPlayer as usize] as libc::c_int) <<
             7 as libc::c_int) as UWORD;
    *iY =
        ((mission.iTranspEntryTileY[iPlayer as usize] as libc::c_int) <<
             7 as libc::c_int) as UWORD;
}
#[no_mangle]
pub unsafe extern "C" fn missionGetTransporterExit(mut iPlayer: SDWORD,
                                                   mut iX: *mut UWORD,
                                                   mut iY: *mut UWORD) {
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"missionGetTransporterExit: player %i too high\x00" as
                  *const u8 as *const libc::c_char, iPlayer);
    };
    if iPlayer < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4146 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"missionGetTransporterExit\x00")).as_ptr(),
              b"iPlayer<MAX_PLAYERS\x00" as *const u8 as *const libc::c_char);
    };
    *iX =
        ((mission.iTranspExitTileX[iPlayer as usize] as libc::c_int) <<
             7 as libc::c_int) as UWORD;
    *iY =
        ((mission.iTranspExitTileY[iPlayer as usize] as libc::c_int) <<
             7 as libc::c_int) as UWORD;
}
/*update routine for mission details */
#[no_mangle]
pub unsafe extern "C" fn missionTimerUpdate() {
    //don't bother with the time check if have 'cheated'
    if mission.cheatTime == 0 {
        //Want a mission timer on all types of missions now - AB 26/01/99
	    //only interested in off world missions (so far!) and if timer has been set
        if mission.time >= 0 as libc::c_int {
            //AND (
            //mission.type == LDS_MKEEP OR mission.type == LDS_MKEEP_LIMBO OR
            //mission.type == LDS_MCLEAR OR mission.type == LDS_BETWEEN))
            //check if time is up
            if gameTime.wrapping_sub(mission.startTime) as SDWORD >
                   mission.time {
                //the script can call the end game cos have failed!
                eventFireCallbackTrigger(CALL_MISSION_TIME as libc::c_int as
                                             TRIGGER_TYPE);
            }
        }
    };
}
// Remove any objects left ie walls,structures and droids that are not the selected player.
//
#[no_mangle]
pub unsafe extern "C" fn missionDestroyObjects() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut Player: UBYTE = 0;
    debug(LOG_NEVER,
          b"missionDestroyObjects\n\x00" as *const u8 as *const libc::c_char);
    Player = 0 as libc::c_int as UBYTE;
    while (Player as libc::c_int) < 8 as libc::c_int {
        if Player as libc::c_uint != selectedPlayer {
            psDroid = apsDroidLists[Player as usize];
            while !psDroid.is_null() {
                let mut psNext: *mut DROID = (*psDroid).psNext;
                removeDroidBase(psDroid);
                //				droidRemove(psDroid, apsDroidLists);
//				droidRelease(psDroid);
//				HEAP_FREE(psDroidHeap, psDroid);
                psDroid = psNext
            }
            //clear out the mission lists as well to make sure no Tranporters exist
            apsDroidLists[Player as usize] =
                mission.apsDroidLists[Player as usize];
            psDroid = apsDroidLists[Player as usize];
            while !psDroid.is_null() {
                let mut psNext_0: *mut DROID = (*psDroid).psNext;
                //make sure its died flag is not set since we've swapped the apsDroidList pointers over
                (*psDroid).died = 0 as libc::c_int as UDWORD;
                removeDroidBase(psDroid);
                //				droidRemove(psDroid, apsDroidLists);
//				droidRelease(psDroid);
//				HEAP_FREE(psDroidHeap, psDroid);
                psDroid = psNext_0
            } // Wonderfull hack to ensure objects destroyed above get free'ed up by objmemUpdate.
            mission.apsDroidLists[Player as usize] = 0 as *mut DROID;
            psStruct = apsStructLists[Player as usize];
            while !psStruct.is_null() {
                let mut psNext_1: *mut STRUCTURE = (*psStruct).psNext;
                removeStruct(psStruct, 1 as libc::c_int);
                psStruct = psNext_1
            }
        }
        Player = Player.wrapping_add(1)
    }
    gameTime = gameTime.wrapping_add(1);
    objmemUpdate();
    // Not sure why but we need to call this after freeing up the droids. List house keeping?
}
unsafe extern "C" fn processPreviousCampDroids() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    //UDWORD			droidX, droidY;
    //BOOL            bPlaced;
    //see if any are left
    if !mission.apsDroidLists[selectedPlayer as usize].is_null() {
        psDroid = mission.apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            psNext = (*psDroid).psNext;
            /*else
            {
		        //remove out of stored list and add to current Droid list
		        if (droidRemove(psDroid, mission.apsDroidLists))
                {
		            addDroid(psDroid, apsDroidLists);
		            //set the x/y
		            droidX = getLandingX(psDroid->player) >> TILE_SHIFT;
		            droidY = getLandingY(psDroid->player) >> TILE_SHIFT;
                    bPlaced = pickATileGen(&droidX, &droidY,LOOK_FOR_EMPTY_TILE,normalPAT);
                    if (!bPlaced)
		            {
			            ASSERT( FALSE, "processPreviousCampDroids: Unable to find a free location \
                            cancel to continue" );
                        vanishDroid(psDroid);
		            }
                    else
                    {
    		            psDroid->x = (UWORD)(droidX << TILE_SHIFT);
	    	            psDroid->y = (UWORD)(droidY << TILE_SHIFT);
		                psDroid->z = map_Height(psDroid->x, psDroid->y);
		                updateDroidOrientation(psDroid);
		                //psDroid->lastTile = mapTile(psDroid->x >> TILE_SHIFT,
			            //    psDroid->y >> TILE_SHIFT);

					    psDroid->selected = FALSE;
		                psDroid->cluster = 0;
		                gridAddObject((BASE_OBJECT *)psDroid);
		                //initialise the movement data
		                initDroidMovement(psDroid);
                    }
                }
            }*/
            if droidRemove(psDroid, mission.apsDroidLists.as_mut_ptr()) != 0 {
                addDroid(psDroid, apsDroidLists.as_mut_ptr());
                vanishDroid(psDroid);
            }
            psDroid = psNext
        }
    };
}
//We want to kill off all droids now! - AB 27/01/99
		    //KILL OFF TRANSPORTER
    		//if (psDroid->droidType == DROID_TRANSPORTER)
//access functions for droidsToSafety flag - so we don't have to end the mission when a Transporter fly's off world
#[no_mangle]
pub unsafe extern "C" fn setDroidsToSafetyFlag(mut set: BOOL) {
    bDroidsToSafety = set;
}
#[no_mangle]
pub unsafe extern "C" fn getDroidsToSafetyFlag() -> BOOL {
    return bDroidsToSafety;
}
//access functions for bPlayCountDown flag - TRUE = play coded mission count down
#[no_mangle]
pub unsafe extern "C" fn setPlayCountDown(mut set: UBYTE) {
    bPlayCountDown = set;
}
#[no_mangle]
pub unsafe extern "C" fn getPlayCountDown() -> BOOL {
    return bPlayCountDown as BOOL;
}
//checks to see if the player has any droids (except Transporters left)
#[no_mangle]
pub unsafe extern "C" fn missionDroidsRemaining(mut player_0: UDWORD)
 -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bDroidsRemaining: BOOL = 0 as libc::c_int;
    psDroid = apsDroidLists[player_0 as usize];
    while !psDroid.is_null() {
        if (*psDroid).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            bDroidsRemaining = 1 as libc::c_int;
            break ;
        } else { psDroid = (*psDroid).psNext }
    }
    return bDroidsRemaining;
}
/*called when a Transporter gets to the edge of the world and the droids are
being flown to safety. The droids inside the Transporter are placed into the
mission list for later use*/
#[no_mangle]
pub unsafe extern "C" fn moveDroidsToSafety(mut psTransporter: *mut DROID) {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"moveUnitsToSafety: unit not a Transporter\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4337 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"moveDroidsToSafety\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    //move droids out of Transporter into mission list
    psDroid = (*(*psTransporter).psGroup).psList;
    while !psDroid.is_null() && psDroid != psTransporter {
        psNext = (*psDroid).psGrpNext;
        grpLeave((*psTransporter).psGroup, psDroid);
        //cam change add droid
        (*psDroid).x = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
        (*psDroid).y = (512 as libc::c_int * 127 as libc::c_int) as UWORD;
        addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
        psDroid = psNext
    }
    //move the transporter into the mission list also
    if droidRemove(psTransporter, apsDroidLists.as_mut_ptr()) != 0 {
        //cam change add droid - done in missionDroidUpdate()
		//psDroid->x = INVALID_XY;
		//psDroid->y = INVALID_XY;
        addDroid(psTransporter, mission.apsDroidLists.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn clearMissionWidgets() {
    //remove any widgets that are up due to the missions
    if mission.time > 0 as libc::c_int { intRemoveMissionTimer(); }
    if missionCanReEnforce() != 0 { intRemoveTransporterTimer(); }
    intRemoveTransporterLaunch();
}
#[no_mangle]
pub unsafe extern "C" fn resetMissionWidgets() {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    //add back any widgets that should be up due to the missions
    if mission.time > 0 as libc::c_int {
        intAddMissionTimer();
        //make sure its not flashing when added
        stopMissionButtonFlash(11000 as libc::c_int as UDWORD);
    }
    if missionCanReEnforce() != 0 {
        addTransporterTimerInterface();
    } else if missionForReInforcements() == 0 {
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                intAddTransporterLaunch(psDroid);
                break ;
            } else { psDroid = (*psDroid).psNext }
        }
        //if not a reinforceable mission and a transporter exists, then add the launch button
    //if (!missionCanReEnforce())
    //check not a typical reinforcement mission
        /*if we got to the end without adding a transporter - there might be
        one sitting in the mission list which is waiting to come back in*/
        if psDroid.is_null() {
            psDroid = mission.apsDroidLists[selectedPlayer as usize];
            while !psDroid.is_null() {
                if (*psDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint &&
                       (*psDroid).action ==
                           DACTION_TRANSPORTWAITTOFLYIN as libc::c_int {
                    intAddTransporterLaunch(psDroid);
                    break ;
                } else { psDroid = (*psDroid).psNext }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn setCampaignNumber(mut number: UDWORD) {
    if number < 4 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Campaign Number too high!\x00" as *const u8 as
                  *const libc::c_char);
    };
    if number < 4 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"mission.c\x00" as *const u8 as *const libc::c_char,
              4427 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"setCampaignNumber\x00")).as_ptr(),
              b"number<4\x00" as *const u8 as *const libc::c_char);
    };
    camNumber = number;
}
#[no_mangle]
pub unsafe extern "C" fn getCampaignNumber() -> UDWORD { return camNumber; }
/*deals with any selectedPlayer's transporters that are flying in when the
mission ends. bOffWorld is TRUE if the Mission is currenly offWorld*/
unsafe extern "C" fn emptyTransporters(mut bOffWorld: BOOL) {
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut psNextTrans: *mut DROID = 0 as *mut DROID;
    //see if there are any Transporters in the world
    psTransporter = apsDroidLists[selectedPlayer as usize];
    while !psTransporter.is_null() {
        psNextTrans = (*psTransporter).psNext;
        if (*psTransporter).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            //if flying in, empty the contents
            if orderState(psTransporter, DORDER_TRANSPORTIN) != 0 {
                /* if we're offWorld, all we need to do is put the droids into the apsDroidList
                and processMission() will assign them a location etc */
                if bOffWorld != 0 {
                    psDroid = (*(*psTransporter).psGroup).psList;
                    while !psDroid.is_null() && psDroid != psTransporter {
                        psNext = (*psDroid).psGrpNext;
                        //take it out of the Transporter group
                        grpLeave((*psTransporter).psGroup, psDroid);
                        //add it back into current droid lists
                        addDroid(psDroid, apsDroidLists.as_mut_ptr());
                        psDroid = psNext
                    }
                } else {
                    /* we're not offWorld so add to mission.apsDroidList to be
                processed by the endMission function */
                    psDroid = (*(*psTransporter).psGroup).psList;
                    while !psDroid.is_null() && psDroid != psTransporter {
                        psNext = (*psDroid).psGrpNext;
                        //take it out of the Transporter group
                        grpLeave((*psTransporter).psGroup, psDroid);
                        //add it back into current droid lists
                        addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
                        psDroid = psNext
                    }
                }
                //now kill off the Transporter
                vanishDroid(psDroid);
            }
        }
        psTransporter = psNextTrans
    }
    //deal with any transporters that are waiting to come over
    psTransporter = mission.apsDroidLists[selectedPlayer as usize];
    while !psTransporter.is_null() {
        if (*psTransporter).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            //for each droid within the transporter...
            psDroid = (*(*psTransporter).psGroup).psList;
            while !psDroid.is_null() && psDroid != psTransporter {
                psNext = (*psDroid).psGrpNext;
                //take it out of the Transporter group
                grpLeave((*psTransporter).psGroup, psDroid);
                //add it back into mission droid lists
                addDroid(psDroid, mission.apsDroidLists.as_mut_ptr());
                psDroid = psNext
            }
        }
        psTransporter = (*psTransporter).psNext
        //don't need to destory the transporter here - it is dealt with by the endMission process
    };
}
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
//checks to see if the player has any droids (except Transporters left)
/*called when a Transporter gets to the edge of the world and the droids are 
being flown to safety. The droids inside the Transporter are placed into the 
mission list for later use*/
//called when ESC is pressed
//resets if return to game after an ESC
// reset the vtol landing pos
//this is called via a script function to place the Limbo droids once the mission has started
//bCheating = TRUE == start of cheat, bCheating = FALSE == end of cheat
/*bCheating = TRUE == start of cheat
bCheating = FALSE == end of cheat */
#[no_mangle]
pub unsafe extern "C" fn setMissionCheatTime(mut bCheating: BOOL) {
    if bCheating != 0 {
        mission.cheatTime = gameTime
    } else {
        //adjust the mission start time for the duration of the cheat!
        mission.startTime =
            (mission.startTime as
                 libc::c_uint).wrapping_add(gameTime.wrapping_sub(mission.cheatTime))
                as UDWORD as UDWORD;
        mission.cheatTime = 0 as libc::c_int as UDWORD
    };
}
