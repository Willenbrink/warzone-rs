use ::libc;
extern "C" {
    pub type _w_context;
    pub type _formation;
    /* Time base values for timespec_get.  */
    /* Time used by the program so far (user time + system time).
   The result / CLOCKS_PER_SEC is program time in seconds.  */
    #[no_mangle]
    fn clock() -> clock_t;
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn srand(__seed: libc::c_uint);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    /* Hide a widget */
    #[no_mangle]
    fn widgHide(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Reveal a widget */
    #[no_mangle]
    fn widgReveal(psScreen: *mut W_SCREEN, id: UDWORD);
    /* Get widget structure */
    #[no_mangle]
    fn widgGetFromID(psScreen: *mut W_SCREEN, id: UDWORD) -> *mut WIDGET;
    /* Possible states for a button */
    // Disable (grey out) a button
    // Fix a button down
    // Fix a button down but it is still clickable
    // Make a button flash.
    #[no_mangle]
    fn widgSetButtonFlash(psScreen: *mut W_SCREEN, id: UDWORD);
    #[no_mangle]
    fn widgClearButtonFlash(psScreen: *mut W_SCREEN, id: UDWORD);
    #[no_mangle]
    fn fireOnLocation(x: UDWORD, y: UDWORD) -> BOOL;
    /* Pop a value off the stack */
    #[no_mangle]
    fn stackPop(psVal: *mut INTERP_VAL) -> BOOL;
    #[no_mangle]
    fn stackPushResult(type_0: INTERP_TYPE, data: SDWORD) -> BOOL;
    #[no_mangle]
    fn stackPopParams(numParams: SDWORD, _: ...) -> BOOL;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
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
    /*Builds an instance of a Structure - the x/y passed in are in world coords.*/
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player: UDWORD, onMission: BOOL) -> *mut DROID;
    /* Remove a droid and free it's memory */
    #[no_mangle]
    fn destroyDroid(psDel: *mut DROID);
    /* Same as destroy droid except no graphical effects */
    #[no_mangle]
    fn vanishDroid(psDel: *mut DROID);
    /* Remove a droid from the apsDroidLists so doesn't update or get drawn etc*/
//returns TRUE if successfully removed from the list
    #[no_mangle]
    fn droidRemove(psDroid: *mut DROID, pList: *mut *mut DROID) -> BOOL;
    // finds a droid for the player and sets it to be the current selected droid
    #[no_mangle]
    fn selectDroidByID(id: UDWORD, player: UDWORD) -> BOOL;
    #[no_mangle]
    fn zonedPAT(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn pickATileGenThreat(x: *mut UDWORD, y: *mut UDWORD,
                          numIterations: UBYTE, threatRange: SDWORD,
                          player: SDWORD,
                          function:
                              Option<unsafe extern "C" fn(_: UDWORD,
                                                          _: UDWORD) -> BOOL>)
     -> BOOL;
    #[no_mangle]
    fn getDroidResourceName(pName: *mut STRING) -> BOOL;
    //access function
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    // give a droid from one player to another - used in Electronic Warfare and multiplayer
    #[no_mangle]
    fn giftSingleDroid(psD: *mut DROID, to: UDWORD) -> *mut DROID;
    //holder for all StructureStats
    #[no_mangle]
    static mut asStructureStats: *mut STRUCTURE_STATS;
    #[no_mangle]
    static mut numStructureStats: UDWORD;
    #[no_mangle]
    static mut asStructLimits: [*mut STRUCTURE_LIMITS; 8];
    #[no_mangle]
    fn IsPlayerDroidLimitReached(PlayerNumber: UDWORD) -> BOOL;
    /* Set the type of droid for a factory to build */
    #[no_mangle]
    fn structSetManufacture(psStruct: *mut STRUCTURE,
                            psTempl: *mut DROID_TEMPLATE, quantity: UBYTE)
     -> BOOL;
    //builds a specified structure at a given location
    #[no_mangle]
    fn buildStructure(pStructureType: *mut STRUCTURE_STATS, x: UDWORD,
                      y: UDWORD, player: UDWORD, FromSave: BOOL)
     -> *mut STRUCTURE;
    /* Remove a structure and free it's memory */
    #[no_mangle]
    fn destroyStruct(psDel: *mut STRUCTURE) -> BOOL;
    // remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
    #[no_mangle]
    fn removeStruct(psDel: *mut STRUCTURE, bDestroy: BOOL) -> BOOL;
    /* checks that the location is a valid one to build on and sets the outline colour
x and y in tile-coords*/
    #[no_mangle]
    fn validLocation(psStats: *mut BASE_STATS, x: UDWORD, y: UDWORD,
                     player: UDWORD, bCheckBuildQueue: BOOL) -> BOOL;
    /*check to see if the structure is 'doing' anything  - return TRUE if idle*/
    #[no_mangle]
    fn structureIdle(psBuilding: *mut STRUCTURE) -> BOOL;
    /*checks to see if any structure exists of a specified type with a specified status */
    #[no_mangle]
    fn checkStructureStatus(psStats: *mut STRUCTURE_STATS, player: UDWORD,
                            status: UDWORD) -> BOOL;
    /*sets the point new droids go to - x/y in world coords for a Factory*/
    #[no_mangle]
    fn setAssemblyPoint(psAssemblyPoint: *mut FLAG_POSITION, x: UDWORD,
                        y: UDWORD, player: UDWORD, bCheck: BOOL);
    /*this is called whenever a structure has finished building*/
    #[no_mangle]
    fn buildingComplete(psBuilding: *mut STRUCTURE);
    /*Checks the template type against the factory type - returns FALSE 
if not a good combination!*/
    #[no_mangle]
    fn validTemplateForFactory(psTemplate: *mut DROID_TEMPLATE,
                               psFactory: *mut STRUCTURE) -> BOOL;
    /*Access functions for the upgradeable stats of a structure*/
    #[no_mangle]
    fn structureBody(psStruct: *mut STRUCTURE) -> UDWORD;
    // Is a structure a factory of somekind?
    #[no_mangle]
    fn StructIsFactory(Struct: *mut STRUCTURE) -> BOOL;
    // give a structure from one player to another - used in Electronic Warfare
    #[no_mangle]
    fn giftSingleStructure(psStructure: *mut STRUCTURE, attackPlayer: UBYTE,
                           bFromScript: BOOL) -> *mut STRUCTURE;
    #[no_mangle]
    fn getMaxDroids(PlayerNumber: UDWORD) -> UDWORD;
    /*
 * Feature.h
 *
 * Definitions for the feature structures.
 *
 */
    //they're just not there anymore!!!!! Ye ha!
    /* The statistics for the features */
    #[no_mangle]
    static mut asFeatureStats: *mut FEATURE_STATS;
    /* Create a feature on the map */
    #[no_mangle]
    fn buildFeature(psStats: *mut FEATURE_STATS, x: UDWORD, y: UDWORD,
                    FromSave: BOOL) -> *mut FEATURE;
    // free up a feature with no visual effects
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
    /* Remove a Feature and free it's memory */
    #[no_mangle]
    fn destroyFeature(psDel: *mut FEATURE);
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
    /* add the droid to the Droid Lists */
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
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
    /* Check no alliance has formed*/
    #[no_mangle]
    fn aiCheckAlliances(s1: UDWORD, s2: UDWORD) -> BOOL;
    // set a vtol to be hovering in the air
    #[no_mangle]
    fn moveMakeVtolHover(psDroid: *mut DROID);
    /*
 * Stats.h
 *
 * Interface to the common stats module
 *
 */
    /* *************************************************************************************
 *
 * Function prototypes and data storage for the stats
 */
    /* The stores for the different stats */
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    //extern POWER_STATS			*asPowerStats;
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    //extern ARMOUR_STATS			*asArmourStats;
    //extern PROGRAM_STATS		*asProgramStats;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    //extern UDWORD		numArmourStats;
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    /* What number the ref numbers start at for each type of stat */
    //#define REF_POWER_START			0x030000
    //#define REF_ARMOUR_START		0x070000
    //#define REF_PROGRAM_START		0x090000
    /* The maximum number of refs for a type of stat */
    //stores for each players component states - see below
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    //store for each players Structure states
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    //converts the name read in from Access into the name which is used in the Stat lists (or ignores it)
    #[no_mangle]
    fn getResourceName(pName: *mut STRING) -> BOOL;
    // Pass in a stat and get its name
    #[no_mangle]
    fn getStatName(pStat: *mut libc::c_void) -> *mut STRING;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    /* The widget screen */
    #[no_mangle]
    static mut psWScreen: *mut W_SCREEN;
    /* Which is the currently selected player */
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    /* Reset the widget screen to just the reticule */
    #[no_mangle]
    fn intResetScreen(NoAnim: BOOL);
    #[no_mangle]
    fn cdspan_PlayInGameAudio(szFileName: *mut STRING, iVol: SDWORD);
    /*causes a reticule button to start flashing*/
    #[no_mangle]
    fn flashReticuleButton(buttonID: UDWORD);
    // stop a reticule button flashing
    #[no_mangle]
    fn stopReticuleButtonFlash(buttonID: UDWORD);
    #[no_mangle]
    fn addMessage(msgType: UDWORD, proxPos: BOOL, player: UDWORD)
     -> *mut MESSAGE;
    #[no_mangle]
    fn removeMessage(psDel: *mut MESSAGE, player: UDWORD);
    #[no_mangle]
    fn findMessage(pViewdata: *mut MSG_VIEWDATA, type_0: MESSAGE_TYPE,
                   player: UDWORD) -> *mut MESSAGE;
    // tell the intelligence screen to play this message immediately
    #[no_mangle]
    fn displayImmediateMessage(psMessage: *mut MESSAGE);
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
    /* Get a player to start manufacturing droids */
//extern void playerStartManufacture(UDWORD player);
    /*sets the point new droids go to - x/y in world coords*/
//extern void setAssemblyPoint(UDWORD x, UDWORD y, UDWORD player);
    /* sends players droids to attack a specified x/y */
    #[no_mangle]
    fn attackLocation(x: UDWORD, y: UDWORD, player: UDWORD);
    #[no_mangle]
    fn setViewPos(x: UDWORD, y: UDWORD, Pan: BOOL);
    //extern void	assignSensorTarget( DROID *psDroid );
    #[no_mangle]
    fn assignSensorTarget(psObj: *mut BASE_OBJECT);
    #[no_mangle]
    fn setUnderwaterTile(num: UDWORD);
    #[no_mangle]
    fn setRubbleTile(num: UDWORD);
    #[no_mangle]
    static mut visibleXTiles: UDWORD;
    #[no_mangle]
    static mut visibleYTiles: UDWORD;
    /* The store for the research stats */
    #[no_mangle]
    static mut asResearch: *mut RESEARCH;
    #[no_mangle]
    static mut numResearch: UDWORD;
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /* Default level of sensor, repair and ECM */
    #[no_mangle]
    static mut aDefaultSensor: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultECM: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultRepair: [UDWORD; 8];
    /* process the results of a completed research topic */
    #[no_mangle]
    fn researchResult(researchIndex: UDWORD, player: UBYTE, bDisplay: BOOL);
    /* Sets the 'possible' flag for a player's research so the topic will appear in 
the research list next time the Research Facilty is selected */
    #[no_mangle]
    fn enableResearch(psResearch: *mut RESEARCH, player: UDWORD) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    /*resets the power levels for all players when power is turned back on*/
    #[no_mangle]
    fn powerCalc(on: BOOL);
    /*sets the initial value for the power*/
    #[no_mangle]
    fn setPlayerPower(power: UDWORD, player: UDWORD);
    #[no_mangle]
    static mut asPower: [*mut PLAYER_POWER; 8];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    #[no_mangle]
    fn initConsoleMessages();
    #[no_mangle]
    fn flushConsoleMessages();
    #[no_mangle]
    fn setConsolePermanence(state: BOOL, bClearOld: BOOL);
    #[no_mangle]
    fn permitNewConsoleMessages(allow: BOOL);
    #[no_mangle]
    fn printf_console(pFormat: *const libc::c_char, _: ...);
    #[no_mangle]
    fn skTopicAvail(inc: UWORD, player: UDWORD) -> BOOL;
    #[no_mangle]
    fn dirtySqrt(x1: SDWORD, y1: SDWORD, x2: SDWORD, y2: SDWORD) -> UDWORD;
    /* Check whether psViewer can see psTarget
 * psViewer should be an object that has some form of sensor,
 * currently droids and structures.
 * psTarget can be any type of BASE_OBJECT (e.g. a tree).
 */
    #[no_mangle]
    fn visibleObject(psViewer: *mut BASE_OBJECT, psTarget: *mut BASE_OBJECT)
     -> BOOL;
    // Do visibility check, but with walls completely blocking LOS.
    #[no_mangle]
    fn visibleObjWallBlock(psViewer: *mut BASE_OBJECT,
                           psTarget: *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    static mut psGateways: *mut GATEWAY;
    #[no_mangle]
    fn DeliveryReposValid() -> BOOL;
    #[no_mangle]
    fn setPlayerColour(player: UDWORD, col: UDWORD) -> BOOL;
    #[no_mangle]
    fn getPlayerColour(pl: UDWORD) -> UBYTE;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    //clear the sequence list
    #[no_mangle]
    fn seq_ClearSeqList();
    //add a sequence to the list to be played
    #[no_mangle]
    fn seq_AddSeqToList(pSeqName: *mut STRING, pAudioName: *mut STRING,
                        pTextName: *mut STRING, bLoop: BOOL,
                        PSXSeqNumber: UDWORD);
    //set and check subtitle mode, TRUE subtitles on
    /*returns the next sequence in the list to play*/
    #[no_mangle]
    fn seq_StartNextFullScreenVideo();
    // Check if the map tile at a location blocks a droid
    #[no_mangle]
    fn fpathGroundBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    fn war_GetFog() -> BOOL;
    #[no_mangle]
    fn war_GetPlayAudioCDs() -> BOOL;
    #[no_mangle]
    static mut fogStatus: UDWORD;
    //extern void	initLighting( void );
    #[no_mangle]
    fn initLighting(x1: UDWORD, y1: UDWORD, x2: UDWORD, y2: UDWORD);
    #[no_mangle]
    fn atmosSetWeatherType(type_0: WT_CLASS);
    #[no_mangle]
    fn cdAudio_PlayTrack(iTrack: SDWORD) -> BOOL;
    #[no_mangle]
    fn cdAudio_Stop() -> BOOL;
    #[no_mangle]
    fn cdAudio_Pause() -> BOOL;
    #[no_mangle]
    fn cdAudio_Resume() -> BOOL;
    // ////////////////////////////////////////////////////////////////////////////
// Game Options and stats. 
    #[no_mangle]
    static mut game: MULTIPLAYERGAME;
    // the game description.
    #[no_mangle]
    static mut bMultiPlayer: BOOL;
    #[no_mangle]
    fn isHumanPlayer(player: UDWORD) -> BOOL;
    //to tell if the player is a computer or not.
    #[no_mangle]
    fn myResponsibility(player: UDWORD) -> BOOL;
    #[no_mangle]
    fn SendResearch(player: UBYTE, index: UDWORD) -> BOOL;
    #[no_mangle]
    fn addTemplate(player: UDWORD, psNew: *mut DROID_TEMPLATE) -> BOOL;
    #[no_mangle]
    static mut asRunData: [RUN_DATA; 8];
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    /* 
 *multigifts. h
 *
 * multiplayer game, gifts and deathmatch relevant funcs.
 */
    /* 
 *multigifts. h
 *
 * multiplayer game, gifts and deathmatch relevant funcs.
 */
    #[no_mangle]
    fn requestAlliance(from: UBYTE, to: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn breakAlliance(p1: UBYTE, p2: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn formAlliance(p1: UBYTE, p2: UBYTE, prop: BOOL, allowAudio: BOOL);
    #[no_mangle]
    fn giftRadar(from: UDWORD, to: UDWORD, send: BOOL);
    #[no_mangle]
    fn applyLimitSet();
    //fog available
    #[no_mangle]
    fn pie_EnableFog(val: BOOL);
    //fog currently on
    #[no_mangle]
    fn pie_SetFogStatus(val: BOOL);
    #[no_mangle]
    fn pie_SetFogColour(colour: UDWORD);
    #[no_mangle]
    fn setScriptWinLoseVideo(val: UBYTE);
    #[no_mangle]
    static mut mission: MISSION;
    // return positions for vtols
    #[no_mangle]
    static mut asVTOLReturnPos: [POINT; 8];
    /*on the PC - sets the countdown played flag*/
    #[no_mangle]
    fn setMissionCountDown();
    #[no_mangle]
    fn missionCanReEnforce() -> BOOL;
    //returns TRUE if the mission is a Limbo Expand mission
    #[no_mangle]
    fn missionLimboExpand() -> BOOL;
    //this is called mid Limbo mission via the script
    #[no_mangle]
    fn resetLimboMission();
    #[no_mangle]
    fn unloadTransporter(psTransporter: *mut DROID, x: UDWORD, y: UDWORD,
                         goingHome: BOOL);
    /*checks that the timer has been set and that a Transporter exists before 
adding the timer button*/
    #[no_mangle]
    fn addTransporterTimerInterface();
    #[no_mangle]
    fn intRemoveTransporterTimer();
    /*checks the time has been set and then adds the timer if not already on 
the display*/
    #[no_mangle]
    fn addMissionTimerInterface();
    #[no_mangle]
    fn intRemoveMissionTimer();
    //access functions for bPlayCountDown flag
    #[no_mangle]
    fn setPlayCountDown(set: UBYTE);
    //sets the coords for the Transporter to land
    #[no_mangle]
    fn setLandingZone(x1: UBYTE, y1: UBYTE, x2: UBYTE, y2: UBYTE);
    /*Initialises all the nogo areas to 0*/
    #[no_mangle]
    fn initNoGoAreas();
    //sets the coords for a no go area
    #[no_mangle]
    fn setNoGoArea(x1: UBYTE, y1: UBYTE, x2: UBYTE, y2: UBYTE, area: UBYTE);
    /* fly in transporters at start of level */
    #[no_mangle]
    fn missionFlyTransportersIn(iPlayer: SDWORD, bTrackTransporter: BOOL);
    /*builds a droid back at the home base whilst on a mission - stored in a list made
available to the transporter interface*/
    #[no_mangle]
    fn buildMissionDroid(psTempl: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                         player: UDWORD) -> *mut DROID;
    //This is just a very big number - bigger than a map width/height could ever be!
    #[no_mangle]
    fn missionSetTransporterEntry(iPlayer: SDWORD, iEntryTileX: SDWORD,
                                  iEntryTileY: SDWORD);
    #[no_mangle]
    fn missionSetTransporterExit(iPlayer: SDWORD, iExitTileX: SDWORD,
                                 iExitTileY: SDWORD);
    //access functions for droidsToSafety flag
    #[no_mangle]
    fn setDroidsToSafetyFlag(set: BOOL);
    #[no_mangle]
    fn setCampaignNumber(number: UDWORD);
    //this is called via a script function to place the Limbo droids once the mission has started
    #[no_mangle]
    fn placeLimboDroids();
    #[no_mangle]
    static mut loopMissionState: LOOP_MISSION_STATE;
    // this is set by scrStartMission to say what type of new level is to be started
    #[no_mangle]
    static mut nextMissionType: SDWORD;
    // Number of units in the current list.
    #[no_mangle]
    fn getNumDroids(player: UDWORD) -> UDWORD;
    // the global case
    //#define DEFAULT_LEVEL	"CAM_2A"
    #[no_mangle]
    static mut pLevelName: [libc::c_char; 257];
    /*check to see if the droid can fit on the Transporter - return TRUE if fits*/
    #[no_mangle]
    fn checkTransporterSpace(psTransporter: *mut DROID,
                             psAssigned: *mut DROID) -> BOOL;
    /*calculates how much space is remaining on the transporter - allows droids to take
up different amount depending on their body size - currently all are set to one!*/
    #[no_mangle]
    fn calcRemainingCapacity(psTransporter: *mut DROID) -> UDWORD;
    /* Remove the Transporter Launch widget from the screen*/
    #[no_mangle]
    fn intRemoveTransporterLaunch();
    /*checks the order of the droid to see if its currenly flying*/
    #[no_mangle]
    fn transporterFlying(psTransporter: *mut DROID) -> BOOL;
    #[no_mangle]
    fn SetRadarZoom(ZoomLevel: UWORD);
    // find the level dataset
    #[no_mangle]
    fn levFindDataSet(pName: *mut STRING, ppsDataSet: *mut *mut LEVEL_DATASET)
     -> BOOL;
    #[no_mangle]
    fn proj_SendProjectile(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                           player: SDWORD, tarX: UDWORD, tarY: UDWORD,
                           tarZ: UDWORD, psTarget: *mut BASE_OBJECT,
                           bVisible: BOOL) -> BOOL;
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // reset the visibility for all clusters for a particular player
    #[no_mangle]
    fn clustResetVisibility(player: SDWORD);
    #[no_mangle]
    static mut baseLocation: [[[SDWORD; 2]; 8]; 8];
}
pub type size_t = libc::c_uint;
/* Type of file system IDs.  */
pub type __clock_t = libc::c_long;
/* Returned by `clock'.  */
pub type clock_t = __clock_t;
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
pub type _widget_type = libc::c_uint;
pub const WIDG_SLIDER: _widget_type = 5;
pub const WIDG_BARGRAPH: _widget_type = 4;
pub const WIDG_EDITBOX: _widget_type = 3;
pub const WIDG_BUTTON: _widget_type = 2;
pub const WIDG_LABEL: _widget_type = 1;
pub const WIDG_FORM: _widget_type = 0;
pub type WIDGET_TYPE = _widget_type;
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
pub type WIDGET_DISPLAY
    =
    Option<unsafe extern "C" fn(_: *mut _widget, _: UDWORD, _: UDWORD,
                                _: *mut UDWORD) -> ()>;
pub type WIDGET = _widget;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _w_screen {
    pub psForm: *mut WIDGET,
    pub psFocus: *mut WIDGET,
    pub TipFontID: libc::c_int,
}
/* The screen structure which stores all info for a widget screen */
pub type W_SCREEN = _w_screen;
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
pub struct iTexAnim {
    pub nFrames: libc::c_int,
    pub playbackRate: libc::c_int,
    pub textureWidth: libc::c_int,
    pub textureHeight: libc::c_int,
}
// The root form of the screen
// The widget that has keyboard focus
//	PROP_FONT	*psTipFont;			// The font for tool tips
// ID of the IVIS font to use for tool tips.
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
pub type _interp_type = libc::c_uint;
pub const VAL_USERTYPESTART: _interp_type = 6;
pub const VAL_VOID: _interp_type = 5;
pub const VAL_EVENT: _interp_type = 4;
pub const VAL_TRIGGER: _interp_type = 3;
pub const VAL_STRING: _interp_type = 2;
pub const VAL_INT: _interp_type = 1;
pub const VAL_BOOL: _interp_type = 0;
pub type INTERP_TYPE = _interp_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _interp_val {
    pub type_0: INTERP_TYPE,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub bval: BOOL,
    pub ival: SDWORD,
    pub sval: *mut STRING,
    pub oval: *mut libc::c_void,
    pub pVoid: *mut libc::c_void,
}
pub type INTERP_VAL = _interp_val;
pub type _scr_user_types = libc::c_uint;
pub const ST_MAXTYPE: _scr_user_types = 34;
pub const ST_POINTER_S: _scr_user_types = 33;
pub const ST_POINTER_T: _scr_user_types = 32;
pub const ST_POINTER_O: _scr_user_types = 31;
pub const ST_RESEARCH: _scr_user_types = 30;
pub const ST_GROUP: _scr_user_types = 29;
pub const ST_LEVEL: _scr_user_types = 28;
pub const ST_SOUND: _scr_user_types = 27;
pub const ST_TEXTSTRING: _scr_user_types = 26;
pub const ST_DROIDID: _scr_user_types = 25;
pub const ST_FEATURESTAT: _scr_user_types = 24;
pub const ST_STRUCTURESTAT: _scr_user_types = 23;
pub const ST_STRUCTUREID: _scr_user_types = 22;
pub const ST_TEMPLATE: _scr_user_types = 21;
pub const ST_BRAIN: _scr_user_types = 20;
pub const ST_REPAIR: _scr_user_types = 19;
pub const ST_WEAPON: _scr_user_types = 18;
pub const ST_CONSTRUCT: _scr_user_types = 17;
pub const ST_SENSOR: _scr_user_types = 16;
pub const ST_ECM: _scr_user_types = 15;
pub const ST_PROPULSION: _scr_user_types = 14;
pub const ST_BODY: _scr_user_types = 13;
pub const ST_COMPONENT: _scr_user_types = 12;
pub const ST_BASESTATS: _scr_user_types = 11;
pub const ST_FEATURE: _scr_user_types = 10;
pub const ST_STRUCTURE: _scr_user_types = 9;
pub const ST_DROID: _scr_user_types = 8;
pub const ST_BASEOBJECT: _scr_user_types = 7;
pub const ST_INTMESSAGE: _scr_user_types = 6;
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
pub type BASE_STATS = _base_stats;
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
//UDWORD		dummy;
//	BOOL		drawnThisFrame;		// for sorting - have we drawn the imd already?
// last frame it was drawn
//	UDWORD		animFrame;			// anim Frame
//	SDWORD		bucketDepth;
//	BOOL		onScreen;
//	UDWORD		numTiles;
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
/* LOC used for holding locations for Sensors and ECM's*/
pub type _loc = libc::c_uint;
pub const LOC_TURRET: _loc = 1;
pub const LOC_DEFAULT: _loc = 0;
/* SIZE used for specifying body size */
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
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
//pointer to which flame graphic to use - for VTOLs only at the moment
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _propulsion_stats {
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
    pub maxSpeed: UDWORD,
    pub propulsionType: UBYTE,
}
pub type PROPULSION_STATS = _propulsion_stats;
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
pub type SENSOR_STATS = _sensor_stats;
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
pub type ECM_STATS = _ecm_stats;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_stats {
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
    pub repairPoints: UDWORD,
    pub repairArmour: BOOL,
    pub location: UDWORD,
    pub time: UDWORD,
    pub pMountGraphic: *mut iIMDShape,
}
pub type REPAIR_STATS = _repair_stats;
pub type WEAPON_STATS = _weapon_stats;
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
//line build requires two sets of coords
// maximum number of characters in a droid name
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUM_DIFF_BUILDINGS: C2RustUnnamed_0 = 22;
pub const REF_SAT_UPLINK: C2RustUnnamed_0 = 21;
pub const REF_MISSILE_SILO: C2RustUnnamed_0 = 20;
pub const REF_REARM_PAD: C2RustUnnamed_0 = 19;
pub const REF_LAB: C2RustUnnamed_0 = 18;
pub const REF_VTOL_FACTORY: C2RustUnnamed_0 = 17;
pub const REF_CYBORG_FACTORY: C2RustUnnamed_0 = 16;
pub const REF_DEMOLISH: C2RustUnnamed_0 = 15;
pub const REF_BRIDGE: C2RustUnnamed_0 = 14;
pub const REF_COMMAND_CONTROL: C2RustUnnamed_0 = 13;
pub const REF_REPAIR_FACILITY: C2RustUnnamed_0 = 12;
pub const REF_RESEARCH_MODULE: C2RustUnnamed_0 = 11;
pub const REF_RESEARCH: C2RustUnnamed_0 = 10;
pub const REF_BLASTDOOR: C2RustUnnamed_0 = 9;
pub const REF_WALLCORNER: C2RustUnnamed_0 = 8;
pub const REF_WALL: C2RustUnnamed_0 = 7;
pub const REF_DEFENSE: C2RustUnnamed_0 = 6;
pub const REF_RESOURCE_EXTRACTOR: C2RustUnnamed_0 = 5;
pub const REF_POWER_MODULE: C2RustUnnamed_0 = 4;
pub const REF_POWER_GEN: C2RustUnnamed_0 = 3;
pub const REF_FACTORY_MODULE: C2RustUnnamed_0 = 2;
pub const REF_FACTORY: C2RustUnnamed_0 = 1;
pub const REF_HQ: C2RustUnnamed_0 = 0;
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
/* The time the research facility was put on hold*/
// secondary order state for all units coming out of the factory
                                            // added AB 22/04/99
//these are no longer required - yipee!
	// The group the droids produced by this factory belong to - used for Missions
	//struct _droid_group		*psGroup;
	//struct _droid			*psGrpNext;
/*pointers to the res ext
																associated with this gen*/
/*highest number the limit can be set to */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_limits {
    pub limit: UBYTE,
    pub currentQuantity: UBYTE,
    pub globalLimit: UBYTE,
}
pub type STRUCTURE_LIMITS = _structure_limits;
// multiplayer only. sets the max value selectable (limits changed by player)
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
// Feature armour
/*
 * messageDef.h
 *
 * Message structure definitions
 */
// Research message
// Campaign message
// Mission Report messages
// Proximity message
// research view
// full screen view sequence - flic
// proximity view - no view really!
// full screen view sequence - flic.	extended format
//enemy proximity message
//resource proximity message
//artefact proximity message
// info required to view an object in Intelligence screen
//allows base plates and turrets to be drawn as well
//which windowed flic to display
/*name of audio track to play (for this seq)*/
/* On PSX if type is VIEW_RPL then
											this is used as a number_of_frames_in_the_stream
											count - NOT used on PC*/
//flag data to control video playback 1 = loop till audio finish
//the number of textmessages associated with
//this sequence
//Pointer to text messages - if any
/*name of audio track to play (for this seq)*/
/* On PSX if type is VIEW_RPL then
								this is used as a number_of_frames_in_the_stream
								count - NOT used on PC*/
//info required to view a flic in Intelligence Screen
//STRING		**ppSeqName;
	//UBYTE		numText;	//the number of textmessages associated with this sequence
	//STRING		**ppTextMsg;	//Pointer to text messages - if any
// info required to view a proximity message
//world coords for position of Proximity message
/*ID of the audio track to play - if any */
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
//name ID of the message - used for loading in and identifying
//the type of view
//the number of textmessages associated with this data
//Pointer to text messages - if any
/*the data required to view - either a
							  VIEW_RESEARCH, VIEW_PROXIMITY or VIEW_REPLAY*/
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const ID_MAX_SOUND: C2RustUnnamed_1 = 354;
pub const ID_SOUND_LAS_SAT_COUNTDOWN: C2RustUnnamed_1 = 353;
pub const ID_SOUND_UPLINK: C2RustUnnamed_1 = 352;
pub const ID_SOUND_PLASMA_FLAMER: C2RustUnnamed_1 = 351;
pub const ID_SOUND_LASER_HEAVY: C2RustUnnamed_1 = 350;
pub const ID_SOUND_EMP: C2RustUnnamed_1 = 349;
pub const ID_SOUND_CYBORG_HEAVY: C2RustUnnamed_1 = 348;
pub const ID_SOUND_CYBORG_GROUND: C2RustUnnamed_1 = 347;
pub const ID_SOUND_NEXUS_UNIT_NEUTRALISED: C2RustUnnamed_1 = 346;
pub const ID_SOUND_NEXUS_UNIT_ABSORBED: C2RustUnnamed_1 = 345;
pub const ID_SOUND_NEXUS_SYNAPTIC_LINK: C2RustUnnamed_1 = 344;
pub const ID_SOUND_NEXUS_STRUCTURE_NEUTRALISED: C2RustUnnamed_1 = 343;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_1 = 342;
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_1 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_1 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_1 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_1 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_1 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_1 = 336;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_1 = 335;
pub const ID_SOUND_INCOMING_LASER_SAT_STRIKE: C2RustUnnamed_1 = 334;
pub const ID_SOUND_LASER_SATELLITE_FIRING: C2RustUnnamed_1 = 333;
pub const ID_SOUND_TEAM_GAMMA_RESCUED: C2RustUnnamed_1 = 332;
pub const ID_SOUND_TEAM_BETA_RESCUED: C2RustUnnamed_1 = 331;
pub const ID_SOUND_TEAM_ALPHA_RESCUED: C2RustUnnamed_1 = 330;
pub const ID_SOUND_TEAM_GAMMA_ERADICATED: C2RustUnnamed_1 = 329;
pub const ID_SOUND_TEAM_BETA_ERADICATED: C2RustUnnamed_1 = 328;
pub const ID_SOUND_TEAM_ALPHA_ERADICATED: C2RustUnnamed_1 = 327;
pub const ID_SOUND_ENEMY_TRANSPORT_LANDING: C2RustUnnamed_1 = 326;
pub const ID_SOUND_ENEMY_ESCAPING: C2RustUnnamed_1 = 325;
pub const ID_SOUND_ENEMY_ESCAPED: C2RustUnnamed_1 = 324;
pub const ID_SOUND_OUT_OF_TIME: C2RustUnnamed_1 = 323;
pub const ID_SOUND_GROUP_INFECTED: C2RustUnnamed_1 = 322;
pub const ID_SOUND_STRUCTURE_INFECTED: C2RustUnnamed_1 = 321;
pub const ID_SOUND_OBJECTIVE_DESTROYED: C2RustUnnamed_1 = 320;
pub const ID_SOUND_OBJECTIVE_CAPTURED: C2RustUnnamed_1 = 319;
pub const ID_SOUND_GROUP_CAPTURED: C2RustUnnamed_1 = 318;
pub const ID_SOUND_GROUP_RESCUED: C2RustUnnamed_1 = 317;
pub const ID_SOUND_UNITS_RESCUED: C2RustUnnamed_1 = 316;
pub const ID_SOUND_CIVILIANS_RESCUED: C2RustUnnamed_1 = 315;
pub const ID_SOUND_CIVILIAN_RESCUED: C2RustUnnamed_1 = 314;
pub const ID_SOUND_STRUCTURE_CAPTURED: C2RustUnnamed_1 = 313;
pub const ID_SOUND_MISSILE_NME_DETECTED: C2RustUnnamed_1 = 312;
pub const ID_SOUND_MISSILE_SILO: C2RustUnnamed_1 = 311;
pub const ID_SOUND_SAM_SITE: C2RustUnnamed_1 = 310;
pub const ID_SOUND_NUCLEAR_REACTOR: C2RustUnnamed_1 = 309;
pub const ID_SOUND_NASDA_CENTRAL: C2RustUnnamed_1 = 308;
pub const ID_SOUND_SATELLITE_UPLINK: C2RustUnnamed_1 = 307;
pub const ID_SOUND_LANDING_ZONE: C2RustUnnamed_1 = 306;
pub const ID_SOUND_NOFAULTS: C2RustUnnamed_1 = 305;
pub const ID_SOUND_OF_SILENCE: C2RustUnnamed_1 = 304;
pub const ID_SOUND_BARB_SCREAM3: C2RustUnnamed_1 = 303;
pub const ID_SOUND_BARB_SCREAM2: C2RustUnnamed_1 = 302;
pub const ID_SOUND_BARB_SCREAM: C2RustUnnamed_1 = 301;
pub const ID_SOUND_HELP: C2RustUnnamed_1 = 300;
pub const ID_SOUND_FIRE_ROAR: C2RustUnnamed_1 = 299;
pub const ID_SOUND_ECM_TOWER: C2RustUnnamed_1 = 298;
pub const ID_SOUND_STEAM: C2RustUnnamed_1 = 297;
pub const ID_SOUND_POWER_SPARK: C2RustUnnamed_1 = 296;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_1 = 295;
pub const ID_SOUND_OIL_PUMP_2: C2RustUnnamed_1 = 294;
pub const ID_SOUND_CYBORG_MOVE: C2RustUnnamed_1 = 293;
pub const ID_SOUND_HOVER_STOP: C2RustUnnamed_1 = 292;
pub const ID_SOUND_HOVER_START: C2RustUnnamed_1 = 291;
pub const ID_SOUND_HOVER_MOVE: C2RustUnnamed_1 = 290;
pub const ID_SOUND_TREAD: C2RustUnnamed_1 = 289;
pub const ID_SOUND_VTOL_MOVE: C2RustUnnamed_1 = 288;
pub const ID_SOUND_VTOL_OFF: C2RustUnnamed_1 = 287;
pub const ID_SOUND_VTOL_LAND: C2RustUnnamed_1 = 286;
pub const ID_SOUND_BLIMP_TAKE_OFF: C2RustUnnamed_1 = 285;
pub const ID_SOUND_BLIMP_LAND: C2RustUnnamed_1 = 284;
pub const ID_SOUND_BLIMP_IDLE: C2RustUnnamed_1 = 283;
pub const ID_SOUND_BLIMP_FLIGHT: C2RustUnnamed_1 = 282;
pub const ID_SOUND_CONSTRUCTOR_SHUTDOWN: C2RustUnnamed_1 = 281;
pub const ID_SOUND_CONSTRUCTOR_MOVE: C2RustUnnamed_1 = 280;
pub const ID_SOUND_CONSTRUCTOR_MOVEOFF: C2RustUnnamed_1 = 279;
pub const ID_SOUND_NEXUS_EXPLOSION: C2RustUnnamed_1 = 278;
pub const ID_SOUND_BUILDING_FALL: C2RustUnnamed_1 = 277;
pub const ID_SOUND_BARB_SQUISH: C2RustUnnamed_1 = 276;
pub const ID_SOUND_RICOCHET_3: C2RustUnnamed_1 = 275;
pub const ID_SOUND_RICOCHET_2: C2RustUnnamed_1 = 274;
pub const ID_SOUND_RICOCHET_1: C2RustUnnamed_1 = 273;
pub const ID_SOUND_EXPLOSION_ANTITANK: C2RustUnnamed_1 = 272;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_1 = 271;
pub const ID_SOUND_EXPLOSION_LASER: C2RustUnnamed_1 = 270;
pub const ID_SOUND_EXPLOSION_SMALL: C2RustUnnamed_1 = 269;
pub const ID_SOUND_CONSTRUCTION_4: C2RustUnnamed_1 = 268;
pub const ID_SOUND_CONSTRUCTION_3: C2RustUnnamed_1 = 267;
pub const ID_SOUND_CONSTRUCTION_2: C2RustUnnamed_1 = 266;
pub const ID_SOUND_CONSTRUCTION_1: C2RustUnnamed_1 = 265;
pub const ID_SOUND_CONSTRUCTION_LOOP: C2RustUnnamed_1 = 264;
pub const ID_SOUND_CONSTRUCTION_START: C2RustUnnamed_1 = 263;
pub const ID_SOUND_WELD_2: C2RustUnnamed_1 = 262;
pub const ID_SOUND_WELD_1: C2RustUnnamed_1 = 261;
pub const ID_SOUND_NEXUS_TOWER: C2RustUnnamed_1 = 260;
pub const ID_SOUND_HIVEL_CANNON: C2RustUnnamed_1 = 259;
pub const ID_SOUND_RAPID_CANNON: C2RustUnnamed_1 = 258;
pub const ID_SOUND_ASSAULT_MG: C2RustUnnamed_1 = 257;
pub const ID_SOUND_SPLASH: C2RustUnnamed_1 = 256;
pub const ID_SOUND_BABA_MG_TOWER: C2RustUnnamed_1 = 255;
pub const ID_SOUND_BABA_MG_HEAVY: C2RustUnnamed_1 = 254;
pub const ID_SOUND_BABA_MG_3: C2RustUnnamed_1 = 253;
pub const ID_SOUND_BABA_MG_2: C2RustUnnamed_1 = 252;
pub const ID_SOUND_BABA_MG_1: C2RustUnnamed_1 = 251;
pub const ID_SOUND_HOWITZ_FLIGHT: C2RustUnnamed_1 = 250;
pub const ID_SOUND_MORTAR: C2RustUnnamed_1 = 249;
pub const ID_SOUND_BEAM_LASER: C2RustUnnamed_1 = 248;
pub const ID_SOUND_PULSE_LASER: C2RustUnnamed_1 = 247;
pub const ID_SOUND_FLAME_THROWER: C2RustUnnamed_1 = 246;
pub const ID_SOUND_MEDIUM_CANNON: C2RustUnnamed_1 = 245;
pub const ID_SOUND_SMALL_CANNON: C2RustUnnamed_1 = 244;
pub const ID_SOUND_LARGE_CANNON: C2RustUnnamed_1 = 243;
pub const ID_SOUND_GAUSSGUN: C2RustUnnamed_1 = 242;
pub const ID_SOUND_ROTARY_LASER: C2RustUnnamed_1 = 241;
pub const ID_SOUND_ROCKET: C2RustUnnamed_1 = 240;
pub const ID_SOUND_COLL_ENEMY_DESTROYED: C2RustUnnamed_1 = 239;
pub const ID_SOUND_COLL_INTERCEPT_AND_DESTROY: C2RustUnnamed_1 = 238;
pub const ID_SOUND_COLL_DIE: C2RustUnnamed_1 = 237;
pub const ID_SOUND_COLL_STARTING_ATTACK_RUN: C2RustUnnamed_1 = 236;
pub const ID_SOUND_COLL_ENGAGING: C2RustUnnamed_1 = 235;
pub const ID_SOUND_COLL_ENEMY_DETECTED: C2RustUnnamed_1 = 234;
pub const ID_SOUND_COLL_FIRE: C2RustUnnamed_1 = 233;
pub const ID_SOUND_COLL_ATTACK: C2RustUnnamed_1 = 232;
pub const ID_SOUND_COLL_DESTROYING_BIOLOGICALS: C2RustUnnamed_1 = 231;
pub const ID_SOUND_COLL_CLEANSE_AND_DESTROY: C2RustUnnamed_1 = 230;
pub const ID_SOUND_ABORTING_ATTACK_RUN3: C2RustUnnamed_1 = 229;
pub const ID_SOUND_COMMENCING_ATTACK_RUN3: C2RustUnnamed_1 = 228;
pub const ID_SOUND_LOCKED_ON3: C2RustUnnamed_1 = 227;
pub const ID_SOUND_RETURNING_TO_BASE3: C2RustUnnamed_1 = 226;
pub const ID_SOUND_ON_OUR_WAY3: C2RustUnnamed_1 = 225;
pub const ID_SOUND_ENEMY_LOCATED3: C2RustUnnamed_1 = 224;
pub const ID_SOUND_ABORTING_ATTACK_RUN2: C2RustUnnamed_1 = 223;
pub const ID_SOUND_COMMENCING_ATTACK_RUN2: C2RustUnnamed_1 = 222;
pub const ID_SOUND_LOCKED_ON2: C2RustUnnamed_1 = 221;
pub const ID_SOUND_RETURNING_TO_BASE2: C2RustUnnamed_1 = 220;
pub const ID_SOUND_ON_OUR_WAY2: C2RustUnnamed_1 = 219;
pub const ID_SOUND_ENEMY_LOCATED2: C2RustUnnamed_1 = 218;
pub const ID_SOUND_ABORTING_ATTACK_RUN1: C2RustUnnamed_1 = 217;
pub const ID_SOUND_COMMENCING_ATTACK_RUN1: C2RustUnnamed_1 = 216;
pub const ID_SOUND_LOCKED_ON1: C2RustUnnamed_1 = 215;
pub const ID_SOUND_RETURNING_TO_BASE1: C2RustUnnamed_1 = 214;
pub const ID_SOUND_ON_OUR_WAY1: C2RustUnnamed_1 = 213;
pub const ID_SOUND_ENEMY_LOCATED1: C2RustUnnamed_1 = 212;
pub const ID_SOUND_PREPARE_FOR_DUST_OFF: C2RustUnnamed_1 = 211;
pub const ID_SOUND_GO_GO_GO: C2RustUnnamed_1 = 210;
pub const ID_SOUND_GREEN_LIGHT_IN_2: C2RustUnnamed_1 = 209;
pub const ID_SOUND_GREEN_LIGHT_IN_3: C2RustUnnamed_1 = 208;
pub const ID_SOUND_GREEN_LIGHT_IN_4: C2RustUnnamed_1 = 207;
pub const ID_SOUND_GREEN_LIGHT_IN_5: C2RustUnnamed_1 = 206;
pub const ID_SOUND_ALRIGHT_BOYS: C2RustUnnamed_1 = 205;
pub const ID_SOUND_APPROACHING_LZ: C2RustUnnamed_1 = 204;
pub const ID_SOUND_RADIOCLICK_6: C2RustUnnamed_1 = 203;
pub const ID_SOUND_RADIOCLICK_5: C2RustUnnamed_1 = 202;
pub const ID_SOUND_RADIOCLICK_4: C2RustUnnamed_1 = 201;
pub const ID_SOUND_RADIOCLICK_3: C2RustUnnamed_1 = 200;
pub const ID_SOUND_RADIOCLICK_2: C2RustUnnamed_1 = 199;
pub const ID_SOUND_RADIOCLICK_1: C2RustUnnamed_1 = 198;
pub const ID_SOUND_COM_HEADING_FOR_RALLY_POINT: C2RustUnnamed_1 = 197;
pub const ID_SOUND_COM_RETURNING_FOR_REPAIR: C2RustUnnamed_1 = 196;
pub const ID_SOUND_COM_UNABLE_TO_COMPLY: C2RustUnnamed_1 = 195;
pub const ID_SOUND_COM_NO_ROUTE_AVAILABLE: C2RustUnnamed_1 = 194;
pub const ID_SOUND_COM_ROUTE_OBSTRUCTED: C2RustUnnamed_1 = 193;
pub const ID_SOUND_COM_ENEMY_VTOLS_DETECTED: C2RustUnnamed_1 = 192;
pub const ID_SOUND_COM_ENEMY_BATTERY_DETECTED: C2RustUnnamed_1 = 191;
pub const ID_SOUND_COM_NEXUS_DETECTED: C2RustUnnamed_1 = 190;
pub const ID_SOUND_COM_NEXUS_TURRET_DETECTED: C2RustUnnamed_1 = 189;
pub const ID_SOUND_COM_NEXUS_TOWER_DETECTED: C2RustUnnamed_1 = 188;
pub const ID_SOUND_COM_FRIENDLY_LZ_DETECTED: C2RustUnnamed_1 = 187;
pub const ID_SOUND_COM_ENEMY_LZ_DETECTED: C2RustUnnamed_1 = 186;
pub const ID_SOUND_COM_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_1 = 185;
pub const ID_SOUND_COM_ALLY_DETECTED: C2RustUnnamed_1 = 184;
pub const ID_SOUND_COM_ENEMY_BASE_DETECTED: C2RustUnnamed_1 = 183;
pub const ID_SOUND_COM_ENEMY_DETECTED: C2RustUnnamed_1 = 182;
pub const ID_SOUND_COM_ARTEFACT_DETECTED: C2RustUnnamed_1 = 181;
pub const ID_SOUND_COM_RESOURCE_DETECTED: C2RustUnnamed_1 = 180;
pub const ID_SOUND_COM_SCAV_OUTPOST_DETECTED: C2RustUnnamed_1 = 179;
pub const ID_SOUND_COM_SCAV_BASE_DETECTED: C2RustUnnamed_1 = 178;
pub const ID_SOUND_COM_SCAVS_DETECTED: C2RustUnnamed_1 = 177;
pub const ID_SOUND_COMMANDER: C2RustUnnamed_1 = 176;
pub const ID_SOUND_REPORTING: C2RustUnnamed_1 = 175;
pub const ID_SOUND_GROUP_9: C2RustUnnamed_1 = 174;
pub const ID_SOUND_GROUP_8: C2RustUnnamed_1 = 173;
pub const ID_SOUND_GROUP_7: C2RustUnnamed_1 = 172;
pub const ID_SOUND_GROUP_6: C2RustUnnamed_1 = 171;
pub const ID_SOUND_GROUP_5: C2RustUnnamed_1 = 170;
pub const ID_SOUND_GROUP_4: C2RustUnnamed_1 = 169;
pub const ID_SOUND_GROUP_3: C2RustUnnamed_1 = 168;
pub const ID_SOUND_GROUP_2: C2RustUnnamed_1 = 167;
pub const ID_SOUND_GROUP_1: C2RustUnnamed_1 = 166;
pub const ID_SOUND_GROUP_0: C2RustUnnamed_1 = 165;
pub const ID_SOUND_GROUP: C2RustUnnamed_1 = 164;
pub const ID_UNITS_TRANSFER: C2RustUnnamed_1 = 163;
pub const ID_TECHNOLOGY_TRANSFER: C2RustUnnamed_1 = 162;
pub const ID_SENSOR_DOWNLOAD: C2RustUnnamed_1 = 161;
pub const ID_POWER_TRANSMIT: C2RustUnnamed_1 = 160;
pub const ID_GIFT: C2RustUnnamed_1 = 159;
pub const ID_CLAN_EXIT: C2RustUnnamed_1 = 158;
pub const ID_CLAN_ENTER: C2RustUnnamed_1 = 157;
pub const ID_ALLIANCE_OFF: C2RustUnnamed_1 = 156;
pub const ID_ALLIANCE_BRO: C2RustUnnamed_1 = 155;
pub const ID_ALLIANCE_ACC: C2RustUnnamed_1 = 154;
pub const ID_SOUND_ENTERING_WARZONE: C2RustUnnamed_1 = 153;
pub const ID_SOUND_MISSILE_CODES_CRACKED: C2RustUnnamed_1 = 152;
pub const ID_SOUND_3RD_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 151;
pub const ID_SOUND_2ND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 150;
pub const ID_SOUND_1ST_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 149;
pub const ID_SOUND_MISSILE_CODES_DECIPHERED: C2RustUnnamed_1 = 148;
pub const ID_SOUND_YOU_ARE_DEFEATED: C2RustUnnamed_1 = 147;
pub const ID_SOUND_SYSTEM_FAILURE_IMMINENT: C2RustUnnamed_1 = 146;
pub const ID_SOUND_UNIT_CAPTURED: C2RustUnnamed_1 = 145;
pub const ID_SOUND_1_MINUTE_REMAINING: C2RustUnnamed_1 = 144;
pub const ID_SOUND_2_MINUTES_REMAINING: C2RustUnnamed_1 = 143;
pub const ID_SOUND_3_MINUTES_REMAINING: C2RustUnnamed_1 = 142;
pub const ID_SOUND_5_MINUTES_REMAINING: C2RustUnnamed_1 = 141;
pub const ID_SOUND_10_MINUTES_REMAINING: C2RustUnnamed_1 = 140;
pub const ID_SOUND_MISSION_TIMER_ACTIVATED: C2RustUnnamed_1 = 139;
pub const ID_SOUND_OBJECTIVE_FAILED: C2RustUnnamed_1 = 138;
pub const ID_SOUND_OBJECTIVE_ACCOMPLISHED: C2RustUnnamed_1 = 137;
pub const ID_SOUND_MISSION_SUCCESSFUL: C2RustUnnamed_1 = 136;
pub const ID_SOUND_MISSION_FAILED: C2RustUnnamed_1 = 135;
pub const ID_SOUND_INCOMING_INTELLIGENCE_REPORT: C2RustUnnamed_1 = 134;
pub const ID_SOUND_INCOMING_TRANSMISSION: C2RustUnnamed_1 = 133;
pub const ID_SOUND_TECHNOLOGY_TAKEN: C2RustUnnamed_1 = 132;
pub const ID_SOUND_RESEARCH_STOLEN: C2RustUnnamed_1 = 131;
pub const ID_SOUND_MISSION_RESULTS: C2RustUnnamed_1 = 130;
pub const ID_SOUND_WARZONE_ACTIVE: C2RustUnnamed_1 = 129;
pub const ID_SOUND_WARZONE_PAUSED: C2RustUnnamed_1 = 128;
pub const ID_SOUND_MISSION_UPDATE: C2RustUnnamed_1 = 127;
pub const ID_SOUND_MISSION_OBJECTIVE: C2RustUnnamed_1 = 126;
pub const ID_SOUND_TRANSPORT_UNABLE_TO_LAND: C2RustUnnamed_1 = 125;
pub const ID_SOUND_TRANSPORT_RETURNING_TO_BASE: C2RustUnnamed_1 = 124;
pub const ID_SOUND_LZ_CLEAR: C2RustUnnamed_1 = 123;
pub const ID_SOUND_LZ_COMPROMISED: C2RustUnnamed_1 = 122;
pub const ID_SOUND_TRANSPORT_REPAIRING: C2RustUnnamed_1 = 121;
pub const ID_SOUND_TRANSPORT_UNDER_ATTACK: C2RustUnnamed_1 = 120;
pub const ID_SOUND_TRANSPORT_LANDING: C2RustUnnamed_1 = 119;
pub const ID_SOUND_REINFORCEMENTS_IN_TRANSIT: C2RustUnnamed_1 = 118;
pub const ID_SOUND_REINFORCEMENTS_AVAILABLE: C2RustUnnamed_1 = 117;
pub const ID_SOUND_NO_ROUTE_AVAILABLE: C2RustUnnamed_1 = 116;
pub const ID_SOUND_ROUTE_OBSTRUCTED: C2RustUnnamed_1 = 115;
pub const ID_SOUND_COMMANDER_REPORTING: C2RustUnnamed_1 = 114;
pub const ID_SOUND_GROUP_REPORTING: C2RustUnnamed_1 = 113;
pub const ID_SOUND_ASSIGNED_TO_COMMANDER: C2RustUnnamed_1 = 112;
pub const ID_SOUND_EXCELLENT: C2RustUnnamed_1 = 111;
pub const ID_SOUND_WELL_DONE: C2RustUnnamed_1 = 110;
pub const ID_SOUND_THAT_IS_INCORRECT: C2RustUnnamed_1 = 109;
pub const ID_SOUND_NO: C2RustUnnamed_1 = 108;
pub const ID_SOUND_NOT_POSSIBLE_TRY_AGAIN: C2RustUnnamed_1 = 107;
pub const ID_SOUND_SCATTER: C2RustUnnamed_1 = 106;
pub const ID_SOUND_RECYCLING: C2RustUnnamed_1 = 105;
pub const ID_SOUND_RETURN_TO_LZ: C2RustUnnamed_1 = 104;
pub const ID_SOUND_PATROL: C2RustUnnamed_1 = 103;
pub const ID_SOUND_PURSUE: C2RustUnnamed_1 = 102;
pub const ID_SOUND_GUARD: C2RustUnnamed_1 = 101;
pub const ID_SOUND_HOLD_POSITION: C2RustUnnamed_1 = 100;
pub const ID_SOUND_CEASEFIRE: C2RustUnnamed_1 = 99;
pub const ID_SOUND_RETURN_FIRE: C2RustUnnamed_1 = 98;
pub const ID_SOUND_FIRE_AT_WILL: C2RustUnnamed_1 = 97;
pub const ID_SOUND_NO_RETREAT: C2RustUnnamed_1 = 96;
pub const ID_SOUND_RETREAT_AT_HEAVY_DAMAGE: C2RustUnnamed_1 = 95;
pub const ID_SOUND_RETREAT_AT_MEDIUM_DAMAGE: C2RustUnnamed_1 = 94;
pub const ID_SOUND_OPTIMUM_RANGE: C2RustUnnamed_1 = 93;
pub const ID_SOUND_LONG_RANGE: C2RustUnnamed_1 = 92;
pub const ID_SOUND_SHORT_RANGE: C2RustUnnamed_1 = 91;
pub const ID_SOUND_COMMAND_CONSOLE_ACTIVATED: C2RustUnnamed_1 = 90;
pub const ID_SOUND_INTERCEPTORS_ASSIGNED: C2RustUnnamed_1 = 89;
pub const ID_SOUND_ASSIGNED: C2RustUnnamed_1 = 88;
pub const ID_SOUND_VTOLS_ENGAGING: C2RustUnnamed_1 = 87;
pub const ID_SOUND_REARMING: C2RustUnnamed_1 = 86;
pub const ID_SOUND_INTERCEPTORS_LAUNCHED: C2RustUnnamed_1 = 85;
pub const ID_SOUND_BATTERY_FIRING_COUNTER_ATTACK: C2RustUnnamed_1 = 84;
pub const ID_SOUND_ENEMY_BATTERY_LOCATED: C2RustUnnamed_1 = 83;
pub const ID_SOUND_ASSIGNED_TO_COUNTER_RADAR: C2RustUnnamed_1 = 82;
pub const ID_SOUND_SENSOR_LOCKED_ON: C2RustUnnamed_1 = 81;
pub const ID_SOUND_ASSIGNED_TO_SENSOR: C2RustUnnamed_1 = 80;
pub const ID_SOUND_UNIT_RETURNING_FOR_REPAIR: C2RustUnnamed_1 = 79;
pub const ID_SOUND_UNIT_RETREATING: C2RustUnnamed_1 = 78;
pub const ID_SOUND_UNIT_DESTROYED: C2RustUnnamed_1 = 77;
pub const ID_SOUND_UNIT_UNDER_ATTACK: C2RustUnnamed_1 = 76;
pub const ID_SOUND_LZ2: C2RustUnnamed_1 = 75;
pub const ID_SOUND_LZ1: C2RustUnnamed_1 = 74;
pub const ID_SOUND_ENEMY_LZ: C2RustUnnamed_1 = 73;
pub const ID_SOUND_INCOMING_ENEMY_TRANSPORT: C2RustUnnamed_1 = 72;
pub const ID_SOUND_ENEMY_BASE_ERADICATED: C2RustUnnamed_1 = 71;
pub const ID_SOUND_ENEMY_BASE: C2RustUnnamed_1 = 70;
pub const ID_SOUND_SCAVENGER_BASE_ERADICATED: C2RustUnnamed_1 = 69;
pub const ID_SOUND_SCAVENGER_OUTPOST_ERADICATED: C2RustUnnamed_1 = 68;
pub const ID_SOUND_SCAVENGER_OUTPOST: C2RustUnnamed_1 = 67;
pub const ID_SOUND_SCAVENGER_BASE: C2RustUnnamed_1 = 66;
pub const ID_SOUND_ENEMY_VTOLS_DETECTED: C2RustUnnamed_1 = 65;
pub const ID_SOUND_ENEMY_BATTERY_DETECTED: C2RustUnnamed_1 = 64;
pub const ID_SOUND_NEXUS_UNIT_DETECTED: C2RustUnnamed_1 = 63;
pub const ID_SOUND_NEXUS_TURRET_DETECTED: C2RustUnnamed_1 = 62;
pub const ID_SOUND_NEXUS_TOWER_DETECTED: C2RustUnnamed_1 = 61;
pub const ID_SOUND_FRIENDLY_LZ_DETECTED: C2RustUnnamed_1 = 60;
pub const ID_SOUND_ENEMY_LZ_DETECTED: C2RustUnnamed_1 = 59;
pub const ID_SOUND_ENEMY_TRANSPORT_DETECTED: C2RustUnnamed_1 = 58;
pub const ID_SOUND_ALLY_DETECTED: C2RustUnnamed_1 = 57;
pub const ID_SOUND_ENEMY_BASE_DETECTED: C2RustUnnamed_1 = 56;
pub const ID_SOUND_ENEMY_UNIT_DETECTED: C2RustUnnamed_1 = 55;
pub const ID_SOUND_ARTEFACT_DISC: C2RustUnnamed_1 = 54;
pub const ID_SOUND_POWER_RESOURCE: C2RustUnnamed_1 = 53;
pub const ID_SOUND_SCAVENGER_OUTPOST_DETECTED: C2RustUnnamed_1 = 52;
pub const ID_SOUND_SCAVENGER_BASE_DETECTED: C2RustUnnamed_1 = 51;
pub const ID_SOUND_SCAVENGERS_DETECTED: C2RustUnnamed_1 = 50;
pub const ID_SOUND_UNIT_REPAIRED: C2RustUnnamed_1 = 49;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED_TO: C2RustUnnamed_1 = 48;
pub const ID_SOUND_DELIVERY_POINT_ASSIGNED: C2RustUnnamed_1 = 47;
pub const ID_SOUND_PRODUCTION_CANCELLED: C2RustUnnamed_1 = 46;
pub const ID_SOUND_PRODUCTION_PAUSED: C2RustUnnamed_1 = 45;
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_1 = 44;
pub const ID_SOUND_PRODUCTION_STARTED: C2RustUnnamed_1 = 43;
pub const ID_SOUND_CYBORG_RESEARCH_COMPLETED: C2RustUnnamed_1 = 42;
pub const ID_SOUND_WEAPON_RESEARCH_COMPLETED: C2RustUnnamed_1 = 41;
pub const ID_SOUND_SYSTEMS_RESEARCH_COMPLETED: C2RustUnnamed_1 = 40;
pub const ID_SOUND_VEHICLE_RESEARCH_COMPLETED: C2RustUnnamed_1 = 39;
pub const ID_SOUND_COMPUTER_RESEARCH_COMPLETED: C2RustUnnamed_1 = 38;
pub const ID_SOUND_POWER_RESEARCH_COMPLETED: C2RustUnnamed_1 = 37;
pub const ID_SOUND_STRUCTURE_RESEARCH_COMPLETED: C2RustUnnamed_1 = 36;
pub const ID_SOUND_MAJOR_RESEARCH: C2RustUnnamed_1 = 35;
pub const ID_SOUND_RESEARCH_COMPLETED: C2RustUnnamed_1 = 34;
pub const ID_SOUND_NEW_CYBORG_AVAILABLE: C2RustUnnamed_1 = 33;
pub const ID_SOUND_NEW_COMPONENT_AVAILABLE: C2RustUnnamed_1 = 32;
pub const ID_SOUND_NEW_STRUCTURE_AVAILABLE: C2RustUnnamed_1 = 31;
pub const ID_SOUND_NEW_RESEARCH_PROJ_AVAILABLE: C2RustUnnamed_1 = 30;
pub const ID_SOUND_ARTIFACT_RECOVERED: C2RustUnnamed_1 = 29;
pub const ID_SOUND_ARTIFACT: C2RustUnnamed_1 = 28;
pub const ID_SOUND_RESEARCH_FACILITY_REQUIRED: C2RustUnnamed_1 = 27;
pub const ID_SOUND_POWER_GENERATOR_REQUIRED: C2RustUnnamed_1 = 26;
pub const ID_SOUND_POWER_TRANSFER_IN_PROGRESS: C2RustUnnamed_1 = 25;
pub const ID_SOUND_RESOURCE_DEPLETED: C2RustUnnamed_1 = 24;
pub const ID_SOUND_DERRICK_DESTROYED: C2RustUnnamed_1 = 23;
pub const ID_SOUND_DERRICK_UNDER_ATTACK: C2RustUnnamed_1 = 22;
pub const ID_SOUND_RESOURCE_HERE: C2RustUnnamed_1 = 21;
pub const ID_SOUND_POWER_LOW: C2RustUnnamed_1 = 20;
pub const ID_SOUND_POWER_GENERATOR_DESTROYED: C2RustUnnamed_1 = 19;
pub const ID_SOUND_POWER_GENERATOR_UNDER_ATTACK: C2RustUnnamed_1 = 18;
pub const ID_SOUND_STRUCTURE_DEMOLISHED: C2RustUnnamed_1 = 17;
pub const ID_SOUND_STRUCTURE_REPAIR_IN_PROGRESS: C2RustUnnamed_1 = 16;
pub const ID_SOUND_STRUCTURE_UNDER_ATTACK: C2RustUnnamed_1 = 15;
pub const ID_SOUND_STRUCTURE_COMPLETED: C2RustUnnamed_1 = 14;
pub const ID_SOUND_CONSTRUCTION_STARTED: C2RustUnnamed_1 = 13;
pub const ID_SOUND_DESIGN_COMPLETED: C2RustUnnamed_1 = 12;
pub const ID_SOUND_PROPULSION_SELECTED: C2RustUnnamed_1 = 11;
pub const ID_SOUND_BODY_SELECTED: C2RustUnnamed_1 = 10;
pub const ID_SOUND_TURRET_SELECTED: C2RustUnnamed_1 = 9;
pub const ID_SOUND_GAME_SHUTDOWN: C2RustUnnamed_1 = 8;
pub const ID_SOUND_MESSAGEEND: C2RustUnnamed_1 = 7;
pub const ID_SOUND_BUILD_FAIL: C2RustUnnamed_1 = 6;
pub const ID_SOUND_ZOOM_ON_RADAR: C2RustUnnamed_1 = 5;
pub const FE_AUDIO_MESSAGEEND: C2RustUnnamed_1 = 4;
pub const ID_SOUND_BUTTON_CLICK_5: C2RustUnnamed_1 = 3;
pub const ID_SOUND_SELECT: C2RustUnnamed_1 = 2;
pub const ID_SOUND_WINDOWOPEN: C2RustUnnamed_1 = 1;
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_1 = 0;
pub const NO_SOUND: C2RustUnnamed_1 = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _player_power {
    pub currentPower: UDWORD,
    pub extractedPower: UDWORD,
    pub psLastPowered: *mut _base_object,
}
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub type DROID_GROUP = _droid_group;
pub type LOOP_MISSION_STATE = libc::c_uint;
pub const LMS_CLEAROBJECTS: LOOP_MISSION_STATE = 5;
pub const LMS_LOADGAME: LOOP_MISSION_STATE = 4;
pub const LMS_NEWLEVEL: LOOP_MISSION_STATE = 3;
pub const LMS_SAVECONTINUE: LOOP_MISSION_STATE = 2;
pub const LMS_SETUPMISSION: LOOP_MISSION_STATE = 1;
pub const LMS_NORMAL: LOOP_MISSION_STATE = 0;
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
// the WRF/WDG files needed for a particular level
// the WRF/WDG files needed for a particular level
pub type LEVEL_DATASET = _level_dataset;
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
// type of map
// number of players for the map
// index of WRF/WDG that loads the scenario file
// title for the level
// title for the level
// the WRF/WDG files for the level
// in load order
// LEVEL_DATASET that must be loaded for this level to load
// LEVEL_DATASET used when changing to this level from another
// off map saving any droids (selectedPlayer) at end into apsLimboDroids
pub const LDS_NONE: _level_type = 10;
pub type WT_CLASS = libc::c_uint;
pub const WT_NONE: WT_CLASS = 2;
pub const WT_SNOWING: WT_CLASS = 1;
pub const WT_RAINING: WT_CLASS = 0;
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
 * GatewayDef.h
 *
 * Structure definitions for routing gateways.
 *
 */
pub type GATEWAY_LINK = _gateway_link;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gateway_link {
    pub psGateway: *mut _gateway,
    pub dist: SWORD,
    pub flags: SWORD,
}
// the flags that get reset by the router
pub type GATEWAY = _gateway;
pub const STATUS_DeliveryReposInProgress: gamestatus = 2;
pub const STATUS_BattleMapViewEnabled: gamestatus = 1;
pub const STATUS_ReticuleIsOpen: gamestatus = 0;
pub type gamestatus = libc::c_uint;
// zone to the left/above the gateway
// zone to the right/below the gateway
// array of links to other zones
// number of links
// Data for the gateway router
// open or closed node
// distance so far and estimate to end
// Previous point in the route
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
/*
 * ScriptFuncs.h
 *
 * All the C functions callable from the script code
 *
 */
// not used in scripts, but used in code.
/* *****************************************************************************************/
/*                 Check for objects in areas                                             */
// check for a base object being in range of a point
#[no_mangle]
pub unsafe extern "C" fn objectInRange(mut psList: *mut BASE_OBJECT,
                                       mut x: SDWORD, mut y: SDWORD,
                                       mut range: SDWORD) -> BOOL {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut rangeSq: SDWORD = 0;
    // See if there is a droid in range
    rangeSq = range * range;
    psCurr = psList;
    while !psCurr.is_null() {
        // skip partially build structures
        if !((*psCurr).type_0 as libc::c_uint ==
                 OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                 (*(psCurr as *mut STRUCTURE)).status as libc::c_int !=
                     SS_BUILT as libc::c_int) {
            // skip flying vtols
            if !((*psCurr).type_0 as libc::c_uint ==
                     OBJ_DROID as libc::c_int as libc::c_uint &&
                     vtolDroid(psCurr as *mut DROID) != 0 &&
                     (*(psCurr as *mut DROID)).sMove.Status as libc::c_int !=
                         0 as libc::c_int) {
                xdiff = (*psCurr).x as SDWORD - x;
                ydiff = (*psCurr).y as SDWORD - y;
                if xdiff * xdiff + ydiff * ydiff < rangeSq {
                    return 1 as libc::c_int
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as libc::c_int;
}
// Check for any player object being within a certain range of a position
// -----------------------------------------------------------------------------------------
// Check for any player object being within a certain range of a position
#[no_mangle]
pub unsafe extern "C" fn scrObjectInRange() -> BOOL {
    let mut range: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrObjectInRange: invalid player number\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  136 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrObjectInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        (objectInRange(apsDroidLists[player as usize] as *mut BASE_OBJECT, x,
                       y, range) != 0 ||
             objectInRange(apsStructLists[player as usize] as
                               *mut BASE_OBJECT, x, y, range) != 0) as
            libc::c_int;
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a droid being within a certain range of a position
// -----------------------------------------------------------------------------------------
// Check for a droid being within a certain range of a position
#[no_mangle]
pub unsafe extern "C" fn scrDroidInRange() -> BOOL {
    let mut range: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrUnitInRange: invalid player number\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  165 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrDroidInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        objectInRange(apsDroidLists[player as usize] as *mut BASE_OBJECT, x,
                      y, range);
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a struct being within a certain range of a position
// -----------------------------------------------------------------------------------------
// Check for a struct being within a certain range of a position
#[no_mangle]
pub unsafe extern "C" fn scrStructInRange() -> BOOL {
    let mut range: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructInRange: invalid player number\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  193 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrStructInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        objectInRange(apsStructLists[player as usize] as *mut BASE_OBJECT, x,
                      y, range);
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// return power of a player.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrPlayerPower() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrPlayerPower: invalid player number\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  218 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrPlayerPower\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT,
                       (*asPower[player as usize]).currentPower as SDWORD) ==
           0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// check for a base object being in an area
unsafe extern "C" fn objectInArea(mut psList: *mut BASE_OBJECT,
                                  mut x1: SDWORD, mut y1: SDWORD,
                                  mut x2: SDWORD, mut y2: SDWORD) -> BOOL {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    // See if there is a droid in Area
    psCurr = psList;
    while !psCurr.is_null() {
        // skip partially build structures
        if !((*psCurr).type_0 as libc::c_uint ==
                 OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                 (*(psCurr as *mut STRUCTURE)).status as libc::c_int !=
                     SS_BUILT as libc::c_int) {
            ox = (*psCurr).x as SDWORD;
            oy = (*psCurr).y as SDWORD;
            if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                return 1 as libc::c_int
            }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as libc::c_int;
}
// Check for any player object being within a certain area
// -----------------------------------------------------------------------------------------
// Check for any player object being within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrObjectInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrObjectInArea: invalid player number\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  272 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrObjectInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        (objectInArea(apsDroidLists[player as usize] as *mut BASE_OBJECT, x1,
                      y1, x2, y2) != 0 ||
             objectInArea(apsStructLists[player as usize] as *mut BASE_OBJECT,
                          x1, y1, x2, y2) != 0) as libc::c_int;
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a droid being within a certain area
// -----------------------------------------------------------------------------------------
// Check for a droid being within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrDroidInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrUnitInArea: invalid player number\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  301 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrDroidInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        objectInArea(apsDroidLists[player as usize] as *mut BASE_OBJECT, x1,
                     y1, x2, y2);
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a struct being within a certain Area of a position
// -----------------------------------------------------------------------------------------
// Check for a struct being within a certain Area of a position
#[no_mangle]
pub unsafe extern "C" fn scrStructInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructInArea: invalid player number\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  329 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrStructInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found =
        objectInArea(apsStructLists[player as usize] as *mut BASE_OBJECT, x1,
                     y1, x2, y2);
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// as above, but only visible structures.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSeenStructInArea() -> BOOL {
    let mut walls: BOOL = 0 as libc::c_int;
    let mut found: BOOL = 0 as libc::c_int;
    let mut player: SDWORD = 0;
    let mut enemy: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    // player, enemyplayer, walls, x1,r1,x2,y2
    if stackPopParams(7 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut enemy as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut walls as *mut BOOL, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSeenStructInArea: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  361 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSeenStructInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psCurr = apsStructLists[enemy as usize];
    while !psCurr.is_null() {
        // skip partially build structures
        if !((*psCurr).type_0 as libc::c_uint ==
                 OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                 (*psCurr).status as libc::c_int != SS_BUILT as libc::c_int) {
            // possible skip walls
            if !(walls != 0 &&
                     ((*(*psCurr).pStructureType).type_0 !=
                          REF_WALL as libc::c_int as libc::c_uint ||
                          (*(*psCurr).pStructureType).type_0 !=
                              REF_WALLCORNER as libc::c_int as libc::c_uint))
               {
                ox = (*psCurr).x as SDWORD;
                oy = (*psCurr).y as SDWORD;
                if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                    // structure is in area.
                    if (*psCurr).visible[player as usize] != 0 {
                        found = 1 as libc::c_int
                    }
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a players structures but no walls being within a certain area
// -----------------------------------------------------------------------------------------
// Check for a players structures but no walls being within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrStructButNoWallsInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut found: SDWORD = 0 as libc::c_int;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructButNoWallsInArea: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  415 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrStructButNoWallsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 !=
               REF_WALL as libc::c_int as libc::c_uint &&
               (*(*psStruct).pStructureType).type_0 !=
                   REF_WALLCORNER as libc::c_int as libc::c_uint &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            ox = (*psStruct).x as SDWORD;
            oy = (*psStruct).y as SDWORD;
            if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                found = 1 as libc::c_int;
                break ;
            }
        }
        psStruct = (*psStruct).psNext
    }
    if stackPushResult(VAL_BOOL, found) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// check for the number of base objects in an area
unsafe extern "C" fn numObjectsInArea(mut psList: *mut BASE_OBJECT,
                                      mut x1: SDWORD, mut y1: SDWORD,
                                      mut x2: SDWORD, mut y2: SDWORD)
 -> SDWORD {
    let mut psCurr: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    let mut count: SDWORD = 0;
    // See if there is a droid in Area
    count = 0 as libc::c_int;
    psCurr = psList;
    while !psCurr.is_null() {
        // skip partially build structures
        if !((*psCurr).type_0 as libc::c_uint ==
                 OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
                 (*(psCurr as *mut STRUCTURE)).status as libc::c_int !=
                     SS_BUILT as libc::c_int) {
            ox = (*psCurr).x as SDWORD;
            oy = (*psCurr).y as SDWORD;
            if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                count += 1 as libc::c_int
            }
        }
        psCurr = (*psCurr).psNext
    }
    return count;
}
// Count the number of player objects within a certain area
// -----------------------------------------------------------------------------------------
// Count the number of player objects within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrNumObjectsInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut count: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumObjectsInArea: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  490 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrNumObjectsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count =
        numObjectsInArea(apsDroidLists[player as usize] as *mut BASE_OBJECT,
                         x1, y1, x2, y2) +
            numObjectsInArea(apsStructLists[player as usize] as
                                 *mut BASE_OBJECT, x1, y1, x2, y2);
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Count the number of player droids within a certain area
// -----------------------------------------------------------------------------------------
// Count the number of player droids within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrNumDroidsInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut count: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumUnitInArea: invalid player number\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  520 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrNumDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count =
        numObjectsInArea(apsDroidLists[player as usize] as *mut BASE_OBJECT,
                         x1, y1, x2, y2);
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Count the number of player structures within a certain area
// -----------------------------------------------------------------------------------------
// Count the number of player structures within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut count: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsInArea: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  549 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrNumStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count =
        numObjectsInArea(apsStructLists[player as usize] as *mut BASE_OBJECT,
                         x1, y1, x2, y2);
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Count the number of player structures but not walls within a certain area
// -----------------------------------------------------------------------------------------
// Count the number of player structures but not walls within a certain area
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsButNotWallsInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut count: SDWORD = 0;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsButNotWallsInArea: invalid player number\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  579 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 31],
                                            &[libc::c_char; 31]>(b"scrNumStructsButNotWallsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count = 0 as libc::c_int;
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 !=
               REF_WALL as libc::c_int as libc::c_uint &&
               (*(*psStruct).pStructureType).type_0 !=
                   REF_WALLCORNER as libc::c_int as libc::c_uint &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            ox = (*psStruct).x as SDWORD;
            oy = (*psStruct).y as SDWORD;
            if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                count += 1 as libc::c_int
            }
        }
        psStruct = (*psStruct).psNext
    }
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Count the number of structures in an area of a certain type
// -----------------------------------------------------------------------------------------
// Count the number of structures in an area of a certain type
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsByTypeInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut count: SDWORD = 0;
    let mut ox: SDWORD = 0;
    let mut oy: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut type_0 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsByTypeInArea: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  625 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrNumStructsByTypeInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count = 0 as libc::c_int;
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 == type_0 as UDWORD &&
               (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
            ox = (*psStruct).x as SDWORD;
            oy = (*psStruct).y as SDWORD;
            if ox >= x1 && ox <= x2 && oy >= y1 && oy <= y2 {
                count += 1 as libc::c_int
            }
        }
        psStruct = (*psStruct).psNext
    }
    if stackPushResult(VAL_INT, count) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Check for a droid having seen a certain object
// -----------------------------------------------------------------------------------------
// Check for a droid having seen a certain object
#[no_mangle]
pub unsafe extern "C" fn scrDroidHasSeen() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut seen: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if psObj.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrUnitHasSeen: NULL object\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  669 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrDroidHasSeen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrUnitHasSeen:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  675 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrDroidHasSeen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // See if any droid has seen this object
    seen = 0 as libc::c_int;
    if (*psObj).visible[player as usize] != 0 { seen = 1 as libc::c_int }
    if stackPushResult(VAL_BOOL, seen as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// Check for a droid being within range of a position
#[no_mangle]
pub unsafe extern "C" fn scrDroidInRangeOfPosition() -> BOOL {
    let mut range: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut rangeSquared: SDWORD = 0;
    let mut dx: SDWORD = 0;
    let mut dy: SDWORD = 0;
    let mut dz: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iZ as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrUnitInRangeOfPosition: invalid player number\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  713 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrDroidInRangeOfPosition\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // See if there is a droid in range
    rangeSquared = range * range;
    found = 0 as libc::c_int;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        dx = (*psCurr).x as SDWORD - iX;
        dy = (*psCurr).y as SDWORD - iY;
        dz = (*psCurr).z as SDWORD - iZ;
        if dx * dx + dy * dy + dz * dz < rangeSquared {
            found = 1 as libc::c_int;
            break ;
        } else { psCurr = (*psCurr).psNext }
    }
    if stackPushResult(VAL_BOOL, found as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Enable a component to be researched
// -----------------------------------------------------------------------------------------
// Enable a component to be researched
#[no_mangle]
pub unsafe extern "C" fn scrEnableComponent() -> BOOL {
    let mut player: SDWORD = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPop(&mut sVal) == 0 { return 0 as libc::c_int }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrEnableComponent:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  759 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrEnableComponent\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // enable the appropriate component
    match sVal.type_0 as libc::c_uint {
        13 => {
            *apCompLists[player as
                             usize][COMP_BODY as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        14 => {
            *apCompLists[player as
                             usize][COMP_PROPULSION as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        15 => {
            *apCompLists[player as
                             usize][COMP_ECM as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        16 => {
            *apCompLists[player as
                             usize][COMP_SENSOR as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        17 => {
            *apCompLists[player as
                             usize][COMP_CONSTRUCT as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        18 => {
            *apCompLists[player as
                             usize][COMP_WEAPON as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        19 => {
            *apCompLists[player as
                             usize][COMP_REPAIRUNIT as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        20 => {
            *apCompLists[player as
                             usize][COMP_BRAIN as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x4 as libc::c_int as UBYTE
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrEnableComponent: unknown type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 791 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"scrEnableComponent\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// Make a component available
// -----------------------------------------------------------------------------------------
// Make a component available
#[no_mangle]
pub unsafe extern "C" fn scrMakeComponentAvailable() -> BOOL {
    let mut player: SDWORD = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPop(&mut sVal) == 0 { return 0 as libc::c_int }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrMakeComponentAvailable:player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  816 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrMakeComponentAvailable\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // make the appropriate component available
    match sVal.type_0 as libc::c_uint {
        13 => {
            *apCompLists[player as
                             usize][COMP_BODY as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        14 => {
            *apCompLists[player as
                             usize][COMP_PROPULSION as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        15 => {
            *apCompLists[player as
                             usize][COMP_ECM as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        16 => {
            *apCompLists[player as
                             usize][COMP_SENSOR as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        17 => {
            *apCompLists[player as
                             usize][COMP_CONSTRUCT as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        18 => {
            *apCompLists[player as
                             usize][COMP_WEAPON as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        19 => {
            *apCompLists[player as
                             usize][COMP_REPAIRUNIT as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        20 => {
            *apCompLists[player as
                             usize][COMP_BRAIN as libc::c_int as
                                        usize].offset(sVal.v.ival as isize) =
                0x1 as libc::c_int as UBYTE
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrEnableComponent: unknown type\x00" as *const u8 as
                          *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 848 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"scrMakeComponentAvailable\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// Build a droid
// -----------------------------------------------------------------------------------------
// Add a droid
#[no_mangle]
pub unsafe extern "C" fn scrAddDroidToMissionList() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_TEMPLATE as libc::c_int,
                      &mut psTemplate as *mut *mut DROID_TEMPLATE,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    /*	if ((UBYTE)player == selectedPlayer )
	{
		ASSERT( FALSE, "scrAddDroidToMissionList: can't add own player to list" );
		return FALSE;
	}*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddUnitToMissionList:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  876 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrAddDroidToMissionList\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddUnitToMissionList: Invalid template pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              881 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"scrAddDroidToMissionList\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    // Don't build a new droid if player limit reached, unless it's a transporter.
    if IsPlayerDroidLimitReached(player as UDWORD) != 0 &&
           (*psTemplate).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        debug(LOG_NEVER,
              b"scrAddUnit : Max units reached ,player %d\n\x00" as *const u8
                  as *const libc::c_char, player);
        psDroid = 0 as *mut DROID
    } else {
        psDroid =
            buildMissionDroid(psTemplate, 128 as libc::c_int as UDWORD,
                              128 as libc::c_int as UDWORD, player as UDWORD)
    }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psDroid as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Build a droid
// -----------------------------------------------------------------------------------------
// Add a droid
#[no_mangle]
pub unsafe extern "C" fn scrAddDroid() -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut player: SDWORD = 0;
    //	INTERP_VAL		sVal;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(4 as libc::c_int, ST_TEMPLATE as libc::c_int,
                      &mut psTemplate as *mut *mut DROID_TEMPLATE,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}
	if (sVal.type != ST_TEMPLATE)
	{
		ASSERT( FALSE, "scrAddDroid: type mismatch for object" );
		return FALSE;
	}
	psTemplate = (DROID_TEMPLATE *)sVal.v.ival;
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddUnit:player number is too high\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  928 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"scrAddDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddUnit: Invalid template pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              933 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 12],
                                        &[libc::c_char; 12]>(b"scrAddDroid\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    // Don't build a new droid if player limit reached, unless it's a transporter.
    if IsPlayerDroidLimitReached(player as UDWORD) != 0 &&
           (*psTemplate).droidType as libc::c_uint !=
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        debug(LOG_NEVER,
              b"scrAddUnit : Max units reached ,player %d\n\x00" as *const u8
                  as *const libc::c_char, player);
        psDroid = 0 as *mut DROID
    } else {
        psDroid =
            buildDroid(psTemplate, x as UDWORD, y as UDWORD, player as UDWORD,
                       0 as libc::c_int);
        if !psDroid.is_null() {
            addDroid(psDroid, apsDroidLists.as_mut_ptr());
            if vtolDroid(psDroid) != 0 {
                // vtols start in the air
                moveMakeVtolHover(psDroid);
            }
        }
    }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psDroid as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Add droid to transporter
// -----------------------------------------------------------------------------------------
// Add droid to transporter
#[no_mangle]
pub unsafe extern "C" fn scrAddDroidToTransporter() -> BOOL {
    let mut psTransporter: *mut DROID = 0 as *mut DROID;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psTransporter as *mut *mut DROID,
                      ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 0 as libc::c_int
    }
    if psTransporter.is_null() || psDroid.is_null() {
        //ignore!
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddUnitToTransporter: null unit passed\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  978 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrAddDroidToTransporter\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 1 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddUnitToTransporter: invalid transporter pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              983 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"scrAddDroidToTransporter\x00")).as_ptr(),
              b"PTRVALID(psTransporter, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddUnitToTransporter: invalid unit pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              985 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"scrAddDroidToTransporter\x00")).as_ptr(),
              b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"scrAddUnitToTransporter: invalid transporter type\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psTransporter).droidType as libc::c_uint ==
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              987 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"scrAddDroidToTransporter\x00")).as_ptr(),
              b"psTransporter->droidType == DROID_TRANSPORTER\x00" as
                  *const u8 as *const libc::c_char);
    };
    /* check for space */
    if checkTransporterSpace(psTransporter, psDroid) != 0 {
        if droidRemove(psDroid, mission.apsDroidLists.as_mut_ptr()) != 0 {
            grpJoin((*psTransporter).psGroup, psDroid);
        }
    }
    return 1 as libc::c_int;
}
//check for a building to have been destroyed
// -----------------------------------------------------------------------------------------
//check for a building to have been destroyed
#[no_mangle]
pub unsafe extern "C" fn scrBuildingDestroyed() -> BOOL {
    let mut player: SDWORD = 0;
    let mut structureID: UDWORD = 0;
    //	INTERP_VAL	sVal;
    let mut destroyed: BOOL = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(2 as libc::c_int, ST_STRUCTUREID as libc::c_int,
                      &mut structureID as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_STRUCTUREID)
	{
		ASSERT( FALSE, "scrBuildingDestroyed: type mismatch for object" );
		return FALSE;
	}
	structureID = (UDWORD)sVal.v.ival;
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBuildingDestroyed:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1029 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrBuildingDestroyed\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    destroyed = 1 as libc::c_int;
    //look thru the players list to see if the structure still exists
    psCurr = apsStructLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).id == structureID { destroyed = 0 as libc::c_int }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_BOOL, destroyed as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Enable a structure type to be built
// -----------------------------------------------------------------------------------------
// Enable a structure to be built
#[no_mangle]
pub unsafe extern "C" fn scrEnableStructure() -> BOOL {
    let mut player: SDWORD = 0;
    let mut index: SDWORD = 0;
    //	INTERP_VAL	sVal;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_STRUCTURESTAT)
	{
		ASSERT( FALSE, "scrEnableStructure: type mismatch for object" );
		return FALSE;
	}*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrEnableStructure:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1074 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrEnableStructure\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if index < 0 as libc::c_int || index > numStructureStats as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrEnableStructure:invalid structure stat\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1080 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrEnableStructure\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // enable the appropriate structure
    *apStructTypeLists[player as usize].offset(index as isize) =
        0x1 as libc::c_int as UBYTE;
    return 1 as libc::c_int;
}
// true if structure is available.
// -----------------------------------------------------------------------------------------
// Check if a structure can be built.
// currently PC skirmish only.
#[no_mangle]
pub unsafe extern "C" fn scrIsStructureAvailable() -> BOOL {
    let mut player: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut result: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if *apStructTypeLists[player as usize].offset(index as isize) as
           libc::c_int == 0x1 as libc::c_int {
        result = 1 as libc::c_int
    } else { result = 0 as libc::c_int }
    if stackPushResult(VAL_BOOL, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// add a tutorial message to the Intelligence Display
//extern BOOL scrAddTutorialMessage(void);
//make the droid with the matching id the currently selected droid
// -----------------------------------------------------------------------------------------
//make the droid with the matching id the currently selected droid
#[no_mangle]
pub unsafe extern "C" fn scrSelectDroidByID() -> BOOL {
    let mut player: SDWORD = 0;
    let mut droidID: SDWORD = 0;
    //	INTERP_VAL		sVal;
    let mut selected: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_DROIDID as libc::c_int,
                      &mut droidID as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_DROIDID)
	{
		ASSERT( FALSE, "scrSelectDroidByID: type mismatch for object" );
		return FALSE;
	}
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSelectUnitByID:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1146 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSelectDroidByID\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    selected = 0 as libc::c_int;
    if selectDroidByID(droidID as UDWORD, player as UDWORD) != 0 {
        selected = 1 as libc::c_int
    }
    //store the reult cos might need to check the droid exists before doing anything else
    if stackPushResult(VAL_BOOL, selected as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Pop up a message box with a number value in it
// -----------------------------------------------------------------------------------------
// Pop up a message box with a number value in it
#[no_mangle]
pub unsafe extern "C" fn scrNumMB() -> BOOL {
    let mut val: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	gameTimeStop();
	DBERROR(("scrNumMB: called by script with value: %d", val));
	gameTimeStart();*/
    debug(LOG_NEVER,
          b"scrNumMB: called by script with value: %d\n\x00" as *const u8 as
              *const libc::c_char, val);
    return 1 as libc::c_int;
}
// Do an approximation to a square root
// -----------------------------------------------------------------------------------------
// Do an approximation to a square root
#[no_mangle]
pub unsafe extern "C" fn scrApproxRoot() -> BOOL {
    let mut val1: SDWORD = 0;
    let mut val2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut val2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if val1 < val2 {
        val1 = val2 + (val1 >> 1 as libc::c_int)
    } else { val1 = val1 + (val2 >> 1 as libc::c_int) }
    if stackPushResult(VAL_INT, val1) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Add a reticule button to the interface
// Add a reticule button to the interface
#[no_mangle]
pub unsafe extern "C" fn scrAddReticuleButton() -> BOOL {
    let mut val: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //set the appropriate flag to 'draw' the button
    match val {
        2 => {
            // bit of a hack here to keep compatibility with old scripts
            widgReveal(psWScreen, 9 as libc::c_int as UDWORD);
        }
        9 => { widgReveal(psWScreen, 9 as libc::c_int as UDWORD); }
        3 => { widgReveal(psWScreen, 3 as libc::c_int as UDWORD); }
        4 => { widgReveal(psWScreen, 4 as libc::c_int as UDWORD); }
        5 => { widgReveal(psWScreen, 5 as libc::c_int as UDWORD); }
        6 => { widgReveal(psWScreen, 6 as libc::c_int as UDWORD); }
        7 => { widgReveal(psWScreen, 7 as libc::c_int as UDWORD); }
        8 => { widgReveal(psWScreen, 8 as libc::c_int as UDWORD); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrAddReticuleButton: Invalid reticule Button ID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 1254 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 21],
                                                &[libc::c_char; 21]>(b"scrAddReticuleButton\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
//Remove a reticule button from the interface
// -----------------------------------------------------------------------------------------
//Remove a reticule button from the interface
#[no_mangle]
pub unsafe extern "C" fn scrRemoveReticuleButton() -> BOOL {
    let mut val: SDWORD = 0;
    let mut bReset: BOOL = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bReset as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    if bInTutorial != 0 {
        if bReset != 0 {
            // not always desirable
            intResetScreen(1 as libc::c_int);
        }
    }
    match val {
        2 => {
            // bit of a hack here to keep compatibility with old scripts
            widgHide(psWScreen, 9 as libc::c_int as UDWORD);
        }
        9 => { widgHide(psWScreen, 9 as libc::c_int as UDWORD); }
        3 => { widgHide(psWScreen, 3 as libc::c_int as UDWORD); }
        4 => { widgHide(psWScreen, 4 as libc::c_int as UDWORD); }
        5 => { widgHide(psWScreen, 5 as libc::c_int as UDWORD); }
        6 => { widgHide(psWScreen, 6 as libc::c_int as UDWORD); }
        7 => { widgHide(psWScreen, 7 as libc::c_int as UDWORD); }
        8 => { widgHide(psWScreen, 8 as libc::c_int as UDWORD); }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrAddReticuleButton: Invalid reticule Button ID\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 1312 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 24],
                                                &[libc::c_char; 24]>(b"scrRemoveReticuleButton\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// add a message to the Intelligence Display
// -----------------------------------------------------------------------------------------
// add a message to the Intelligence Display
#[no_mangle]
pub unsafe extern "C" fn scrAddMessage() -> BOOL {
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut msgType: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut playImmediate: BOOL = 0;
    //	INTERP_VAL		sVal;
    let mut psViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    let mut height: UDWORD = 0;
    if stackPopParams(4 as libc::c_int, ST_INTMESSAGE as libc::c_int,
                      &mut psViewData as *mut *mut VIEWDATA,
                      VAL_INT as libc::c_int, &mut msgType as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      VAL_BOOL as libc::c_int,
                      &mut playImmediate as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    /*
	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_INTMESSAGE)
	{
		ASSERT( FALSE, "scrAddMessage: type mismatch for object" );
		return FALSE;
	}
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddMessage:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1350 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrAddMessage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //create the message
    psMessage =
        addMessage(msgType as UDWORD, 0 as libc::c_int, player as UDWORD);
    if !psMessage.is_null() {
        //set the data
        (*psMessage).pViewData = psViewData as *mut MSG_VIEWDATA;
        if msgType == MSG_PROXIMITY as libc::c_int {
            //check the z value is at least the height of the terrain
            height =
                map_Height((*((*psViewData).pData as *mut VIEW_PROXIMITY)).x,
                           (*((*psViewData).pData as *mut VIEW_PROXIMITY)).y)
                    as UDWORD;
            if (*((*psViewData).pData as *mut VIEW_PROXIMITY)).z < height {
                (*((*psViewData).pData as *mut VIEW_PROXIMITY)).z = height
            }
        }
        if playImmediate != 0 {
            //		psCurrentMsg = psMessage;
			//initTextDisplay(psCurrentMsg, WFont, 255);
			//addIntelScreen(TRUE);
	//		addIntelScreen();
            displayImmediateMessage(psMessage);
            stopReticuleButtonFlash(6 as libc::c_int as UDWORD);
        }
    }
    return 1 as libc::c_int;
}
// remove a message from the Intelligence Display
// -----------------------------------------------------------------------------------------
// remove a message from the Intelligence Display
#[no_mangle]
pub unsafe extern "C" fn scrRemoveMessage() -> BOOL {
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut msgType: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    if stackPopParams(3 as libc::c_int, ST_INTMESSAGE as libc::c_int,
                      &mut psViewData as *mut *mut VIEWDATA,
                      VAL_INT as libc::c_int, &mut msgType as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddMessage:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1402 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrRemoveMessage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //find the message
    psMessage =
        findMessage(psViewData as *mut MSG_VIEWDATA, msgType as MESSAGE_TYPE,
                    player as UDWORD);
    if !psMessage.is_null() {
        //delete it
        removeMessage(psMessage, player as UDWORD);
    } else {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrRemoveMessage:cannot find message - %s\x00" as
                      *const u8 as *const libc::c_char, (*psViewData).pName);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1416 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrRemoveMessage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//builds a droid in the specified factory//
// -----------------------------------------------------------------------------------------
// add a tutorial message to the Intelligence Display
/*BOOL scrAddTutorialMessage(void)
{
	SDWORD			player;
	VIEWDATA		*psViewData;


	if (!stackPopParams(2, ST_INTMESSAGE, &psViewData , VAL_INT, &player))
	{
		return FALSE;
	}

	if (player >= MAX_PLAYERS)
	{
		ASSERT( FALSE, "scrAddTutorialMessage:player number is too high" );
		return FALSE;
	}

	//set the data
	tutorialMessage.pViewData = psViewData;
	tutorialMessage.player = player;

	//play the tutorial message immediately
	psCurrentMsg = &tutorialMessage;
	initTextDisplay(psCurrentMsg, WFont, 255);
	addIntelScreen(TRUE);

	return TRUE;
}*/
// -----------------------------------------------------------------------------------------
/*builds a droid in the specified factory*/
#[no_mangle]
pub unsafe extern "C" fn scrBuildDroid() -> BOOL {
    let mut player: SDWORD = 0;
    let mut productionRun: SDWORD = 0;
    //	INTERP_VAL		sVal, sVal2;
    let mut psFactory: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if stackPopParams(4 as libc::c_int, ST_TEMPLATE as libc::c_int,
                      &mut psTemplate as *mut *mut DROID_TEMPLATE,
                      ST_STRUCTURE as libc::c_int,
                      &mut psFactory as *mut *mut STRUCTURE,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      VAL_INT as libc::c_int,
                      &mut productionRun as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if psFactory.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBuildUnit: NULL factory object\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1471 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBuildUnit:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1477 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if productionRun > 0xff as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBuildUnit: production run too high\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1483 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrBuildUnit: Invalid structure pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              1488 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
              b"PTRVALID(psFactory, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psFactory).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psFactory).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psFactory).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"scrBuildUnit: structure is not a factory\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psFactory).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psFactory).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psFactory).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              1492 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
              b"(psFactory->pStructureType->type == REF_FACTORY OR psFactory->pStructureType->type == REF_CYBORG_FACTORY OR psFactory->pStructureType->type == REF_VTOL_FACTORY)\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrBuildUnit: Invalid template pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              1494 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    //check building the right sort of droid for the factory
    if validTemplateForFactory(psTemplate, psFactory) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrBuildUnit: invalid template - %s for factory - %s\x00"
                      as *const u8 as *const libc::c_char,
                  (*psTemplate).aName.as_mut_ptr(),
                  (*(*psFactory).pStructureType).pName);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1501 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrBuildDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    structSetManufacture(psFactory, psTemplate, productionRun as UBYTE);
    return 1 as libc::c_int;
}
// for a specified player, set the assembly point droids go to when built
// -----------------------------------------------------------------------------------------
// for a specified structure, set the assembly point droids go to when built
#[no_mangle]
pub unsafe extern "C" fn scrSetAssemblyPoint() -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(3 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut psBuilding as *mut *mut STRUCTURE,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if psBuilding.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetAssemblyPoint: NULL structure\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1525 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetAssemblyPoint\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*(*psBuilding).pStructureType).type_0 !=
           REF_FACTORY as libc::c_int as libc::c_uint &&
           (*(*psBuilding).pStructureType).type_0 !=
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint &&
           (*(*psBuilding).pStructureType).type_0 !=
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetAssemblyPoint: structure is not a factory\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1533 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetAssemblyPoint\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setAssemblyPoint((*((*psBuilding).pFunctionality as
                            *mut FACTORY)).psAssemblyPoint, x as UDWORD,
                     y as UDWORD, (*psBuilding).player as UDWORD,
                     1 as libc::c_int);
    return 1 as libc::c_int;
}
// test for structure is idle or not
// -----------------------------------------------------------------------------------------
// test for structure is idle or not
#[no_mangle]
pub unsafe extern "C" fn scrStructureIdle() -> BOOL {
    //	INTERP_VAL	sVal;
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut idle: BOOL = 0;
    if stackPopParams(1 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut psBuilding as *mut *mut STRUCTURE) == 0 {
        return 0 as libc::c_int
    }
    //	DBPRINTF(("scrStructureIdle called\n"));
    if psBuilding.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureIdle: NULL structure\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1559 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrStructureIdle\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //test for idle
    idle = 0 as libc::c_int;
    if structureIdle(psBuilding) != 0 { idle = 1 as libc::c_int }
    //	DBPRINTF(("structure %p is %d\n",psBuilding,idle));
    if stackPushResult(VAL_BOOL, idle as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// sends a players droids to a location to attack
// -----------------------------------------------------------------------------------------
// sends a players droids to a location to attack
#[no_mangle]
pub unsafe extern "C" fn scrAttackLocation() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAttackLocation:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1594 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrAttackLocation\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    attackLocation(x as UDWORD, y as UDWORD, player as UDWORD);
    return 1 as libc::c_int;
}
//Destroy a feature
// -----------------------------------------------------------------------------------------
//Destroy a feature
#[no_mangle]
pub unsafe extern "C" fn scrDestroyFeature() -> BOOL {
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    //	INTERP_VAL	sVal;
    if stackPopParams(1 as libc::c_int, ST_FEATURE as libc::c_int,
                      &mut psFeature as *mut *mut FEATURE) == 0 {
        return 0 as libc::c_int
    }
    if psFeature.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrDestroyFeature: Invalid feature pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1618 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrDestroyFeature\x00")).as_ptr(),
                  b"PTRVALID(psFeature, sizeof(FEATURE))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    removeFeature(psFeature);
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// static vars to enum features.
static mut psFeatureStatToFind: [*mut FEATURE_STATS; 8] =
    [0 as *const FEATURE_STATS as *mut FEATURE_STATS; 8];
static mut playerToEnum: [SDWORD; 8] = [0; 8];
static mut getFeatureCount: [SDWORD; 8] =
    [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0];
// enumerate features;
//static	FEATURE			*psCurrEnumFeature[MAX_PLAYERS];
// -----------------------------------------------------------------------------------------
// init enum visible features.
#[no_mangle]
pub unsafe extern "C" fn scrInitGetFeature() -> BOOL {
    let mut player: SDWORD = 0; // find this stat
    let mut iFeat: SDWORD = 0; // that this player can see
    let mut bucket: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, ST_FEATURESTAT as libc::c_int,
                      &mut iFeat as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut bucket as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psFeatureStatToFind[bucket as usize] =
        asFeatureStats.offset(iFeat as isize);
    playerToEnum[bucket as usize] = player;
    //	psCurrEnumFeature[bucket]		= apsFeatureLists[0];
    getFeatureCount[bucket as usize] =
        0 as libc::c_int; // start at the beginning of list.
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// get next visible feature of required type
// notes:	can't rely on just using the feature list, since it may change
//			between calls, Use an index into list instead.
//			Doesn't return Features sharing a tile with a structure.
//			Skirmish Only, dunno if kev uses this?
#[no_mangle]
pub unsafe extern "C" fn scrGetFeature() -> BOOL {
    let mut bucket: SDWORD = 0;
    let mut count: SDWORD = 0;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut bucket as *mut SDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetFeature: Failed to pop player number from stack\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1667 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrGetFeature\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    count = 0 as libc::c_int;
    // go to the correct start point in the feature list.
    psFeat = apsFeatureLists[0 as libc::c_int as usize];
    while !psFeat.is_null() && count < getFeatureCount[bucket as usize] {
        psFeat = (*psFeat).psNext;
        count += 1
    }
    if psFeat.is_null() {
        // no more to find.
        if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                           0 as *mut libc::c_void as SDWORD) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrGetFeature: Failed to push result\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 1682 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"scrGetFeature\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    // check to see if badly called
    if psFeatureStatToFind[bucket as usize].is_null() {
        debug(LOG_NEVER,
              b"invalid feature to find. possibly due to save game\n\x00" as
                  *const u8 as *const libc::c_char);
        if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                           0 as *mut libc::c_void as SDWORD) == 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrGetFeature: Failed to push result\x00" as *const u8
                          as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 1694 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 14],
                                                &[libc::c_char; 14]>(b"scrGetFeature\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    // begin searching the feature list for the required stat.
    while !psFeat.is_null() {
        if (*(*psFeat).psStats).subType as libc::c_uint ==
               (*psFeatureStatToFind[bucket as usize]).subType as libc::c_uint
               &&
               (*psFeat).visible[playerToEnum[bucket as usize] as usize] as
                   libc::c_int != 0 as libc::c_int &&
               (*mapTile(((*psFeat).x as libc::c_int >> 7 as libc::c_int) as
                             UDWORD,
                         ((*psFeat).y as libc::c_int >> 7 as libc::c_int) as
                             UDWORD)).tileInfoBits as libc::c_int &
                   0x1 as libc::c_int == 0 &&
               fireOnLocation((*psFeat).x as UDWORD, (*psFeat).y as UDWORD) ==
                   0 {
            // not burning.
            if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                               psFeat as SDWORD) == 0 {
                //	push result
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrGetFeature: Failed to push result\x00" as
                              *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptfuncs.c\x00" as *const u8 as
                              *const libc::c_char, 1711 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 14],
                                                    &[libc::c_char; 14]>(b"scrGetFeature\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
            getFeatureCount[bucket as usize] += 1;
            return 1 as libc::c_int
        }
        getFeatureCount[bucket as usize] += 1;
        psFeat = (*psFeat).psNext
    }
    // none found
    if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                       0 as *mut libc::c_void as UDWORD as SDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetFeature: Failed to push result\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1725 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrGetFeature\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Add a feature
/*
// -----------------------------------------------------------------------------------------
// enum next visible feature of required type.
// note: wont return features covered by structures (ie oil resources)
// YUK NASTY BUG. CANT RELY ON THE FEATURE LIST BETWEEN CALLS.
BOOL scrGetFeature(void)
{
	SDWORD	bucket;

	if ( !stackPopParams(1,VAL_INT,&bucket) )
	{
		return FALSE;
	}

	while(psCurrEnumFeature[bucket])
	{
		if(	( psCurrEnumFeature[bucket]->psStats->subType == psFeatureStatToFind[bucket]->subType)
			&&
			( psCurrEnumFeature[bucket]->visible[playerToEnum[bucket]]	!= 0)
			&&
			!TILE_HAS_STRUCTURE(mapTile(psCurrEnumFeature[bucket]->x>>TILE_SHIFT,psCurrEnumFeature[bucket]->y>>TILE_SHIFT) )
		   )
		{
			if (!stackPushResult(ST_FEATURE,(UDWORD) psCurrEnumFeature[bucket]))			//	push result
			{
				return FALSE;
			}
			psCurrEnumFeature[bucket] = psCurrEnumFeature[bucket]->psNext;
			return TRUE;
		}

		psCurrEnumFeature[bucket] = psCurrEnumFeature[bucket]->psNext;
	}
	// push NULL, none found;
	if (!stackPushResult(ST_FEATURE, (UDWORD)NULL))
	{
		return FALSE;
	}
	return TRUE;
}
*/
// -----------------------------------------------------------------------------------------
//Add a feature
#[no_mangle]
pub unsafe extern "C" fn scrAddFeature() -> BOOL {
    let mut psStat: *mut FEATURE_STATS = 0 as *mut FEATURE_STATS;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iMapX: SDWORD = 0;
    let mut iMapY: SDWORD = 0;
    let mut iTestX: SDWORD = 0;
    let mut iTestY: SDWORD = 0;
    let mut iFeat: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, ST_FEATURESTAT as libc::c_int,
                      &mut iFeat as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psStat = asFeatureStats.offset(iFeat as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddFeature: Invalid feature pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              1791 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"scrAddFeature\x00")).as_ptr(),
              b"PTRVALID(psStat, sizeof(FEATURE_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psStat.is_null() {
        iMapX = iX >> 7 as libc::c_int;
        iMapY = iY >> 7 as libc::c_int;
        /* check for wrecked feature already on-tile and remove */
        psFeat = apsFeatureLists[0 as libc::c_int as usize];
        while !psFeat.is_null() {
            iTestX = (*psFeat).x as libc::c_int >> 7 as libc::c_int;
            iTestY = (*psFeat).y as libc::c_int >> 7 as libc::c_int;
            if iTestX == iMapX && iTestY == iMapY {
                if (*(*psFeat).psStats).subType as libc::c_uint ==
                       FEAT_BUILD_WRECK as libc::c_int as libc::c_uint {
                    /* remove feature */
                    removeFeature(psFeat);
                    break ;
                } else {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"scrAddFeature: building feature on tile already occupied\n\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"scriptfuncs.c\x00" as *const u8 as
                                  *const libc::c_char, 1815 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 14],
                                                        &[libc::c_char; 14]>(b"scrAddFeature\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            psFeat = (*psFeat).psNext
        }
        psFeat =
            buildFeature(psStat, iX as UDWORD, iY as UDWORD, 0 as libc::c_int)
    }
    if stackPushResult(ST_FEATURE as libc::c_int as INTERP_TYPE,
                       psFeat as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Add a structure
// -----------------------------------------------------------------------------------------
//Add a structure
#[no_mangle]
pub unsafe extern "C" fn scrAddStructure() -> BOOL {
    let mut psStat: *mut STRUCTURE_STATS =
        0 as *mut STRUCTURE_STATS; //, iWidth, iBreadth;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE; //, iW, iB;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iMapX: SDWORD = 0;
    let mut iMapY: SDWORD = 0;
    let mut iStruct: SDWORD = 0;
    let mut iPlayer: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut iStruct as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    psStat = asStructureStats.offset(iStruct as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddStructure: Invalid feature pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              1849 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"scrAddStructure\x00")).as_ptr(),
              b"PTRVALID(psStat, sizeof(STRUCTURE_STATS))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psStat.is_null() {
        /* offset coords so building centre at (iX, iY) */
/*		no longer necessary - buildStruct no longer uses top left
		iX -= psStat->baseWidth*TILE_UNITS/2;
		iY -= psStat->baseBreadth*TILE_UNITS/2;*/
        iMapX = iX >> 7 as libc::c_int;
        iMapY = iY >> 7 as libc::c_int;
        /* check for structure already on-tile */
        if (*mapTile(iMapX as UDWORD, iMapY as UDWORD)).tileInfoBits as
               libc::c_int & 0x1 as libc::c_int != 0 {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"scrAddStructure: tile already occupied by structure\n\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 1865 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 16],
                                                &[libc::c_char; 16]>(b"scrAddStructure\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
        psStruct =
            buildStructure(psStat, iX as UDWORD, iY as UDWORD,
                           iPlayer as UDWORD, 0 as libc::c_int);
        if !psStruct.is_null() {
            (*psStruct).status = SS_BUILT as libc::c_int as UBYTE;
            buildingComplete(psStruct);
            /*
            Apart from this being wrong (iWidth = 0 when psStat->baseWidth = 1
            and you end up in an infinite loop) we don't need to do this here
            since the map is flattened as part of buildStructure

			iWidth   = psStat->baseWidth/2;
			iBreadth = psStat->baseBreadth/2;

			// flatten tiles across building base
			for ( iW=iMapX; iW<=iMapX+(SDWORD)psStat->baseWidth; iW+=iWidth )
			{
				for ( iB=iMapY; iB<=iMapY+(SDWORD)psStat->baseBreadth; iB+=iBreadth )
				{
					setTileHeight(iW, iB, psStruct->z);
				}
			}*/
        }
    }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psStruct as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Destroy a structure
// -----------------------------------------------------------------------------------------
//Destroy a structure
#[no_mangle]
pub unsafe extern "C" fn scrDestroyStructure() -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut psStruct as *mut *mut STRUCTURE) == 0 {
        return 0 as libc::c_int
    }
    if psStruct.is_null() {
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrDestroyStructure: Invalid structure pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  1915 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrDestroyStructure\x00")).as_ptr(),
                  b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as *const u8 as
                      *const libc::c_char);
        };
    }
    removeStruct(psStruct, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
//NEXT 2 FUNCS ONLY USED IN MULTIPLAYER AS FAR AS I KNOW (25 AUG98) alexl.
// static vars to enum structs;
static mut psStructStatToFind: *mut STRUCTURE_STATS =
    0 as *const STRUCTURE_STATS as *mut STRUCTURE_STATS;
static mut playerToEnumStruct: UDWORD = 0;
static mut enumStructCount: UDWORD = 0;
static mut structfindany: BOOL = 0;
// enumerate structures
// init enum visible structures.
#[no_mangle]
pub unsafe extern "C" fn scrInitEnumStruct() -> BOOL {
    let mut player: SDWORD = 0;
    let mut iStat: SDWORD = 0;
    let mut targetplayer: SDWORD = 0;
    let mut any: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut any as *mut SDWORD,
                      ST_STRUCTURESTAT as libc::c_int,
                      &mut iStat as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut targetplayer as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if any == 1 as libc::c_int {
        structfindany = 1 as libc::c_int
    } else { structfindany = 0 as libc::c_int }
    psStructStatToFind = asStructureStats.offset(iStat as isize);
    playerToEnumStruct = player as UDWORD;
    enumStructCount = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrEnumStruct() -> BOOL {
    let mut count: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    // go to the correct start point in the structure list.
    count = 0 as libc::c_int as UDWORD;
    psStruct = apsStructLists[playerToEnumStruct as usize];
    while !psStruct.is_null() && count < enumStructCount {
        psStruct = (*psStruct).psNext;
        count = count.wrapping_add(1)
    }
    if psStruct.is_null() {
        // no more to find.
        if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                           0 as *mut libc::c_void as SDWORD) == 0 {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    while !psStruct.is_null() {
        // find a visible structure of required type.
        //		if(	(structfindany || (psStruct->pStructureType->type == psStructStatToFind->type))
        if (structfindany != 0 ||
                (*(*psStruct).pStructureType).ref_0 ==
                    (*psStructStatToFind).ref_0) &&
               (*psStruct).visible[playerToEnumStruct as usize] as libc::c_int
                   != 0 {
            if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                               psStruct as UDWORD as SDWORD) == 0 {
                //	push result
                return 0 as libc::c_int
            }
            enumStructCount = enumStructCount.wrapping_add(1);
            return 1 as libc::c_int
        }
        enumStructCount = enumStructCount.wrapping_add(1);
        psStruct = (*psStruct).psNext
    }
    // push NULL, none found;
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       0 as *mut libc::c_void as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*looks to see if a structure (specified by type) exists */
// -----------------------------------------------------------------------------------------
/*looks to see if a structure (specified by type) exists and is being built*/
#[no_mangle]
pub unsafe extern "C" fn scrStructureBeingBuilt() -> BOOL {
    //	INTERP_VAL			sVal;
    let mut structInc: UDWORD = 0;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut player: SDWORD = 0;
    let mut beingBuilt: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_STRUCTURESTAT)
	{
		ASSERT( FALSE, "scrStructureBeingBuilt: type mismatch for object" );
		return FALSE;
	}
	psStats = (STRUCTURE_STATS *)(asStructureStats + sVal.v.ival);
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBeingBuilt:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2036 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrStructureBeingBuilt\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(structInc as isize);
    beingBuilt = 0 as libc::c_int;
    if checkStructureStatus(psStats, player as UDWORD,
                            SS_BEING_BUILT as libc::c_int as UDWORD) != 0 {
        beingBuilt = 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, beingBuilt as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* almost the same as above, but only for a specific struct*/
// pc multiplayer only for now.
// -----------------------------------------------------------------------------------------
// multiplayer skirmish only for now.
// returns TRUE if a specific struct is complete. I know it's like the previous func,
#[no_mangle]
pub unsafe extern "C" fn scrStructureComplete() -> BOOL {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut result: BOOL = 0;
    if stackPopParams(1 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut psStruct as *mut *mut STRUCTURE) == 0 {
        return 0 as libc::c_int
    }
    if (*psStruct).status as libc::c_int == SS_BUILT as libc::c_int {
        result = 1 as libc::c_int
    } else { result = 0 as libc::c_int }
    if stackPushResult(VAL_BOOL, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*looks to see if a structure (specified by type) exists and built*/
// -----------------------------------------------------------------------------------------
/*looks to see if a structure (specified by type) exists and built*/
#[no_mangle]
pub unsafe extern "C" fn scrStructureBuilt() -> BOOL {
    //	INTERP_VAL			sVal;
    let mut structInc: UDWORD = 0;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut player: SDWORD = 0;
    let mut built: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    /*	if (!stackPop(&sVal))
	{
		return FALSE;
	}

	if (sVal.type != ST_STRUCTURESTAT)
	{
		ASSERT( FALSE, "scrStructureBuilt: type mismatch for object" );
		return FALSE;
	}
	psStats = (STRUCTURE_STATS *)(asStructureStats + sVal.v.ival);
*/
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuilt:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2115 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrStructureBuilt\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(structInc as isize);
    built = 0 as libc::c_int;
    if checkStructureStatus(psStats, player as UDWORD,
                            SS_BUILT as libc::c_int as UDWORD) != 0 {
        built = 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, built as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*centre theview on an object - can be droid/structure or feature */
// -----------------------------------------------------------------------------------------
/*centre the view on an object - can be droid/structure or feature */
#[no_mangle]
pub unsafe extern "C" fn scrCentreView() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    //	INTERP_VAL	sVal;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psObj.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCentreView: NULL object\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2147 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 14],
                                            &[libc::c_char; 14]>(b"scrCentreView\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //centre the view on the objects x/y
    setViewPos(((*psObj).x as libc::c_int >> 7 as libc::c_int) as UDWORD,
               ((*psObj).y as libc::c_int >> 7 as libc::c_int) as UDWORD,
               0 as libc::c_int);
    return 1 as libc::c_int;
}
/*centre the view on a position */
// -----------------------------------------------------------------------------------------
/*centre the view on a position */
#[no_mangle]
pub unsafe extern "C" fn scrCentreViewPos() -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >= mapWidth as SDWORD * 128 as libc::c_int ||
           y < 0 as libc::c_int ||
           y >= mapHeight as SDWORD * 128 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCenterViewPos: coords off map\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2171 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrCentreViewPos\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //centre the view on the objects x/y
    setViewPos((x >> 7 as libc::c_int) as UDWORD,
               (y >> 7 as libc::c_int) as UDWORD, 0 as libc::c_int);
    return 1 as libc::c_int;
}
// Get a pointer to a structure based on a stat - returns NULL if cannot find one
// -----------------------------------------------------------------------------------------
// Get a pointer to a structure based on a stat - returns NULL if cannot find one
#[no_mangle]
pub unsafe extern "C" fn scrGetStructure() -> BOOL {
    let mut player: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut structType: UDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetStructure:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2197 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrGetStructure\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    structType = (*asStructureStats.offset(index as isize)).ref_0;
    //search the players' list of built structures to see if one exists
    found = 0 as libc::c_int;
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).ref_0 == structType {
            found = 1 as libc::c_int;
            break ;
        } else { psStruct = (*psStruct).psNext }
    }
    //make sure pass NULL back if not got one
    if found == 0 { psStruct = 0 as *mut STRUCTURE }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psStruct as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Get a pointer to a template based on a component stat - returns NULL if cannot find one
// -----------------------------------------------------------------------------------------
// Get a pointer to a template based on a component stat - returns NULL if cannot find one
#[no_mangle]
pub unsafe extern "C" fn scrGetTemplate() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut found: BOOL = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    let mut i: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetTemplate:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2246 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrGetTemplate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if stackPop(&mut sVal) == 0 { return 0 as libc::c_int }
    //search the players' list of templates to see if one exists
    found = 0 as libc::c_int;
    psTemplate = apsDroidTemplates[player as usize];
    while !psTemplate.is_null() {
        match sVal.type_0 as libc::c_uint {
            13 => {
                if (*psTemplate).asParts[COMP_BODY as libc::c_int as usize] ==
                       sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            14 => {
                if (*psTemplate).asParts[COMP_PROPULSION as libc::c_int as
                                             usize] == sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            15 => {
                if (*psTemplate).asParts[COMP_ECM as libc::c_int as usize] ==
                       sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            16 => {
                if (*psTemplate).asParts[COMP_SENSOR as libc::c_int as usize]
                       == sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            17 => {
                if (*psTemplate).asParts[COMP_CONSTRUCT as libc::c_int as
                                             usize] == sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            19 => {
                if (*psTemplate).asParts[COMP_REPAIRUNIT as libc::c_int as
                                             usize] == sVal.v.ival {
                    found = 1 as libc::c_int
                }
            }
            18 => {
                i = 0 as libc::c_int as UDWORD;
                while i < 1 as libc::c_int as libc::c_uint {
                    if (*psTemplate).asWeaps[i as usize] ==
                           sVal.v.ival as UDWORD {
                        found = 1 as libc::c_int;
                        break ;
                    } else { i = i.wrapping_add(1) }
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrGetTemplate: unknown type\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptfuncs.c\x00" as *const u8 as
                              *const libc::c_char, 2309 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"scrGetTemplate\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        if found != 0 { break ; }
        psTemplate = (*psTemplate).psNext
    }
    //make sure pass NULL back if not got one
    if found == 0 { psTemplate = 0 as *mut DROID_TEMPLATE }
    if stackPushResult(ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                       psTemplate as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Get a pointer to a droid based on a component stat - returns NULL if cannot find one
// -----------------------------------------------------------------------------------------
// Get a pointer to a droid based on a component stat - returns NULL if cannot find one
#[no_mangle]
pub unsafe extern "C" fn scrGetDroid() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut found: BOOL = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    let mut i: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetUnit:player number is too high\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2350 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"scrGetDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if stackPop(&mut sVal) == 0 { return 0 as libc::c_int }
    //search the players' list of droid to see if one exists
    found = 0 as libc::c_int;
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        match sVal.type_0 as libc::c_uint {
            13 => {
                if (*psDroid).asBits[COMP_BODY as libc::c_int as usize].nStat
                       as libc::c_uint == sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            14 => {
                if (*psDroid).asBits[COMP_PROPULSION as libc::c_int as
                                         usize].nStat as libc::c_uint ==
                       sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            15 => {
                if (*psDroid).asBits[COMP_ECM as libc::c_int as usize].nStat
                       as libc::c_uint == sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            16 => {
                if (*psDroid).asBits[COMP_SENSOR as libc::c_int as
                                         usize].nStat as libc::c_uint ==
                       sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            17 => {
                if (*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as
                                         usize].nStat as libc::c_uint ==
                       sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            19 => {
                if (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as
                                         usize].nStat as libc::c_uint ==
                       sVal.v.ival as UDWORD {
                    found = 1 as libc::c_int
                }
            }
            18 => {
                i = 0 as libc::c_int as UDWORD;
                while i < 1 as libc::c_int as libc::c_uint {
                    if (*psDroid).asWeaps[i as usize].nStat ==
                           sVal.v.ival as UDWORD {
                        found = 1 as libc::c_int;
                        break ;
                    } else { i = i.wrapping_add(1) }
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"scrGetUnit: unknown type\x00" as *const u8 as
                              *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"scriptfuncs.c\x00" as *const u8 as
                              *const libc::c_char, 2413 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_char; 12]>(b"scrGetDroid\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 0 as libc::c_int
            }
        }
        if found != 0 { break ; }
        psDroid = (*psDroid).psNext
    }
    //make sure pass NULL back if not got one
    if found == 0 { psDroid = 0 as *mut DROID }
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psDroid as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Sets all the scroll params for the map
// -----------------------------------------------------------------------------------------
// Sets all the scroll params for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetScrollParams() -> BOOL {
    let mut minX: SDWORD = 0;
    let mut minY: SDWORD = 0;
    let mut maxX: SDWORD = 0;
    let mut maxY: SDWORD = 0;
    let mut prevMinX: SDWORD = 0;
    let mut prevMinY: SDWORD = 0;
    let mut prevMaxX: SDWORD = 0;
    let mut prevMaxY: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut minX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut minY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut maxX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut maxY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the values entered are valid
    if minX < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Minimum scroll x value %d is less than zero - \x00" as
                      *const u8 as *const libc::c_char, minX);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2452 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if minY < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Minimum scroll y value %d is less than zero - \x00" as
                      *const u8 as *const libc::c_char, minY);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2457 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    if maxX > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll x value %d is greater than mapWidth - \x00"
                      as *const u8 as *const libc::c_char, maxX);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2461 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    if maxX <
           visibleXTiles.wrapping_add(1 as libc::c_int as libc::c_uint) as
               SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll x %d has to be bigger than Visible Width(22) - \x00"
                      as *const u8 as *const libc::c_char, maxX);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2465 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    if maxY > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll y value %d is greater than mapWidth - \x00"
                      as *const u8 as *const libc::c_char, maxY);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2469 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    if maxY <
           visibleYTiles.wrapping_add(1 as libc::c_int as libc::c_uint) as
               SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll y %d has to be bigger than Visible Height(22) - \x00"
                      as *const u8 as *const libc::c_char, maxY);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2473 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetScrollParams\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    prevMinX = scrollMinX;
    prevMinY = scrollMinY;
    prevMaxX = scrollMaxX;
    prevMaxY = scrollMaxY;
    scrollMinX = minX;
    scrollMaxX = maxX;
    scrollMinY = minY;
    scrollMaxY = maxY;
    //when the scroll limits change midgame - need to redo the lighting
    //initLighting(scrollMinX, scrollMinY, scrollMaxX, scrollMaxY);
    initLighting(if prevMinX < scrollMinX { prevMinX } else { scrollMinX } as
                     UDWORD,
                 if prevMinY < scrollMinY { prevMinY } else { scrollMinY } as
                     UDWORD,
                 if prevMaxX < scrollMaxX { prevMaxX } else { scrollMaxX } as
                     UDWORD,
                 if prevMaxY < scrollMaxY { prevMaxY } else { scrollMaxY } as
                     UDWORD);
    return 1 as libc::c_int;
}
// Sets the scroll minX separately for the map
// -----------------------------------------------------------------------------------------
// Sets the scroll minX separately for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetScrollMinX() -> BOOL {
    let mut minX: SDWORD = 0;
    let mut prevMinX: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut minX as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the value entered are valid
    if minX < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Minimum scroll x value %d is less than zero - \x00" as
                      *const u8 as *const libc::c_char, minX);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2510 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetScrollMinX\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    prevMinX = scrollMinX;
    scrollMinX = minX;
    //when the scroll limits change midgame - need to redo the lighting
    //initLighting(scrollMinX, scrollMinY, scrollMaxX, scrollMaxY);
    initLighting(if prevMinX < scrollMinX { prevMinX } else { scrollMinX } as
                     UDWORD, scrollMinY as UDWORD, scrollMaxX as UDWORD,
                 scrollMaxY as UDWORD);
    return 1 as libc::c_int;
}
// Sets the scroll minY separately for the map
// -----------------------------------------------------------------------------------------
// Sets the scroll minY separately for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetScrollMinY() -> BOOL {
    let mut minY: SDWORD = 0;
    let mut prevMinY: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut minY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the value entered are valid
    if minY < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Minimum scroll y value %d is less than zero - \x00" as
                      *const u8 as *const libc::c_char, minY);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2540 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetScrollMinY\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    prevMinY = scrollMinY;
    scrollMinY = minY;
    //when the scroll limits change midgame - need to redo the lighting
    //initLighting(scrollMinX, scrollMinY, scrollMaxX, scrollMaxY);
    initLighting(scrollMinX as UDWORD,
                 if prevMinY < scrollMinY { prevMinY } else { scrollMinY } as
                     UDWORD, scrollMaxX as UDWORD, scrollMaxY as UDWORD);
    return 1 as libc::c_int;
}
// Sets the scroll maxX separately for the map
// -----------------------------------------------------------------------------------------
// Sets the scroll maxX separately for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetScrollMaxX() -> BOOL {
    let mut maxX: SDWORD = 0;
    let mut prevMaxX: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut maxX as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the value entered are valid
    if maxX > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll x value %d is greater than mapWidth - \x00"
                      as *const u8 as *const libc::c_char, maxX);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2571 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetScrollMaxX\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    prevMaxX = scrollMaxX;
    scrollMaxX = maxX;
    //when the scroll limits change midgame - need to redo the lighting
    //initLighting(scrollMinX, scrollMinY, scrollMaxX, scrollMaxY);
    initLighting(scrollMinX as UDWORD, scrollMinY as UDWORD,
                 if prevMaxX < scrollMaxX { prevMaxX } else { scrollMaxX } as
                     UDWORD, scrollMaxY as UDWORD);
    return 1 as libc::c_int;
}
// Sets the scroll maxY separately for the map
// -----------------------------------------------------------------------------------------
// Sets the scroll maxY separately for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetScrollMaxY() -> BOOL {
    let mut maxY: SDWORD = 0;
    let mut prevMaxY: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut maxY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the value entered are valid
    if maxY > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Maximum scroll y value %d is greater than mapWidth - \x00"
                      as *const u8 as *const libc::c_char, maxY);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2602 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetScrollMaxY\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    prevMaxY = scrollMaxY;
    scrollMaxY = maxY;
    //when the scroll limits change midgame - need to redo the lighting
    //initLighting(scrollMinX, scrollMinY, scrollMaxX, scrollMaxY);
    initLighting(scrollMinX as UDWORD, scrollMinY as UDWORD,
                 scrollMaxX as UDWORD,
                 if prevMaxY < scrollMaxY { prevMaxY } else { scrollMaxY } as
                     UDWORD);
    return 1 as libc::c_int;
}
// Sets which sensor will be used as the default for a player
// -----------------------------------------------------------------------------------------
// Sets which sensor will be used as the default for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetDefaultSensor() -> BOOL {
    let mut player: SDWORD = 0;
    let mut sensorInc: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_SENSOR as libc::c_int,
                      &mut sensorInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultSensor:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2632 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultSensor\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check is a valid sensor Inc
    if sensorInc > numSensorStats {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultSensor: Sensor Inc is too high - %d\x00" as
                      *const u8 as *const libc::c_char, sensorInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2639 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultSensor\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check that this sensor is a default sensor
    if (*asSensorStats.offset(sensorInc as isize)).location !=
           LOC_DEFAULT as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultSensor: This sensor is not a default one - %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(&mut *asSensorStats.offset(sensorInc as isize)
                                  as *mut SENSOR_STATS as *mut libc::c_void));
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2648 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultSensor\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //assign since OK!
    aDefaultSensor[player as usize] = sensorInc;
    return 1 as libc::c_int;
}
// Sets which ECM will be used as the default for a player
// -----------------------------------------------------------------------------------------
// Sets which ECM will be used as the default for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetDefaultECM() -> BOOL {
    let mut player: SDWORD = 0;
    let mut ecmInc: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_ECM as libc::c_int,
                      &mut ecmInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultECM:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2672 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetDefaultECM\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check is a valid ecmInc
    if ecmInc > numECMStats {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultECM: ECM Inc is too high - %d\x00" as
                      *const u8 as *const libc::c_char, ecmInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2679 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetDefaultECM\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check that this ecm is a default ecm
    if (*asECMStats.offset(ecmInc as isize)).location !=
           LOC_DEFAULT as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultECM: This ecm is not a default one - %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(&mut *asECMStats.offset(ecmInc as isize) as
                                  *mut ECM_STATS as *mut libc::c_void));
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2687 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetDefaultECM\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //assign since OK!
    aDefaultECM[player as usize] = ecmInc;
    return 1 as libc::c_int;
}
// Sets which RepairUnit will be used as the default for a player
// -----------------------------------------------------------------------------------------
// Sets which RepairUnit will be used as the default for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetDefaultRepair() -> BOOL {
    let mut player: SDWORD = 0;
    let mut repairInc: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_REPAIR as libc::c_int,
                      &mut repairInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultRepair:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2711 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultRepair\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check is a valid repairInc
    if repairInc > numRepairStats {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultRepair: Repair Inc is too high - %d\x00" as
                      *const u8 as *const libc::c_char, repairInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2718 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultRepair\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check that this repair is a default repair
    if (*asRepairStats.offset(repairInc as isize)).location !=
           LOC_DEFAULT as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetDefaultRepair: This repair is not a default one - %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(&mut *asRepairStats.offset(repairInc as isize)
                                  as *mut REPAIR_STATS as *mut libc::c_void));
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2726 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetDefaultRepair\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //assign since OK!
    aDefaultRepair[player as usize] = repairInc;
    return 1 as libc::c_int;
}
// Sets the structure limits for a player
// -----------------------------------------------------------------------------------------
// Sets the structure limits for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetStructureLimits() -> BOOL {
    let mut player: SDWORD = 0;
    let mut limit: SDWORD = 0;
    let mut structInc: UDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    if stackPopParams(3 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut limit as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2751 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrSetStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if structInc > numStructureStats {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: Structure stat is too high - %d\x00"
                      as *const u8 as *const libc::c_char, structInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2757 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrSetStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if limit < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: limit is less than zero - %d\x00"
                      as *const u8 as *const libc::c_char, limit);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2763 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrSetStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if limit > 255 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: limit is too high - %d - must be less than %d\x00"
                      as *const u8 as *const libc::c_char, limit,
                  255 as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2770 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrSetStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStructLimits = asStructLimits[player as usize];
    (*psStructLimits.offset(structInc as isize)).limit = limit as UBYTE;
    (*psStructLimits.offset(structInc as isize)).globalLimit = limit as UBYTE;
    return 1 as libc::c_int;
}
//multiplayer limit handler
// -----------------------------------------------------------------------------------------
// multiplayer limit handler.
#[no_mangle]
pub unsafe extern "C" fn scrApplyLimitSet() -> BOOL {
    applyLimitSet();
    return 1 as libc::c_int;
}
// plays a sound for the specified player - only plays the sound if the
//specified player = selectedPlayer
// -----------------------------------------------------------------------------------------
// plays a sound for the specified player - only plays the sound if the
// specified player = selectedPlayer
#[no_mangle]
pub unsafe extern "C" fn scrPlaySound() -> BOOL {
    let mut player: SDWORD = 0;
    let mut soundID: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_SOUND as libc::c_int,
                      &mut soundID as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrPlaySound:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2808 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 13],
                                            &[libc::c_char; 13]>(b"scrPlaySound\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player == selectedPlayer as SDWORD {
        audio_QueueTrack(soundID);
        if bInTutorial != 0 {
            audio_QueueTrack(ID_SOUND_OF_SILENCE as libc::c_int);
        }
    }
    return 1 as libc::c_int;
}
// plays a sound for the specified player - only plays the sound if the
// specified player = selectedPlayer - saves position
// -----------------------------------------------------------------------------------------
// plays a sound for the specified player - only plays the sound if the
// specified player = selectedPlayer - saves position
#[no_mangle]
pub unsafe extern "C" fn scrPlaySoundPos() -> BOOL {
    let mut player: SDWORD = 0;
    let mut soundID: SDWORD = 0;
    let mut iX: SDWORD = 0;
    let mut iY: SDWORD = 0;
    let mut iZ: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, ST_SOUND as libc::c_int,
                      &mut soundID as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iZ as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrPlaySoundPos:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2838 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrPlaySoundPos\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player == selectedPlayer as SDWORD {
        audio_QueueTrackPos(soundID, iX, iY, iZ);
    }
    return 1 as libc::c_int;
}
// same as above - but it doesn't clear what's there and isn't permanent
// -----------------------------------------------------------------------------------------
/* add a text message to the top of the screen for the selected player*/
#[no_mangle]
pub unsafe extern "C" fn scrShowConsoleText() -> BOOL {
    let mut pText: *mut STRING = 0 as *mut STRING;
    let mut player: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_TEXTSTRING as libc::c_int,
                      &mut pText as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddConsoleText:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2864 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrShowConsoleText\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player == selectedPlayer as SDWORD {
        permitNewConsoleMessages(1 as libc::c_int);
        addConsoleMessage(pText, CENTRE_JUSTIFY);
    }
    return 1 as libc::c_int;
}
/* add a text message tothe top of the screen for the selected player*/
// -----------------------------------------------------------------------------------------
/* add a text message to the top of the screen for the selected player*/
#[no_mangle]
pub unsafe extern "C" fn scrAddConsoleText() -> BOOL {
    let mut pText: *mut STRING = 0 as *mut STRING;
    let mut player: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_TEXTSTRING as libc::c_int,
                      &mut pText as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddConsoleText:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2891 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrAddConsoleText\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player == selectedPlayer as SDWORD {
        permitNewConsoleMessages(1 as libc::c_int);
        setConsolePermanence(1 as libc::c_int, 1 as libc::c_int);
        addConsoleMessage(pText, CENTRE_JUSTIFY);
        permitNewConsoleMessages(0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
/* Adds console text without clearing old */
// -----------------------------------------------------------------------------------------
/* add a text message to the top of the screen for the selected player - without clearing whats there*/
#[no_mangle]
pub unsafe extern "C" fn scrTagConsoleText() -> BOOL {
    let mut pText: *mut STRING = 0 as *mut STRING;
    let mut player: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_TEXTSTRING as libc::c_int,
                      &mut pText as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddConsoleText:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  2922 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrTagConsoleText\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player == selectedPlayer as SDWORD {
        permitNewConsoleMessages(1 as libc::c_int);
        setConsolePermanence(1 as libc::c_int, 0 as libc::c_int);
        addConsoleMessage(pText, CENTRE_JUSTIFY);
        permitNewConsoleMessages(0 as libc::c_int);
    }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrClearConsole() -> BOOL {
    flushConsoleMessages();
    return 1 as libc::c_int;
}
//demo functions for turning the power on
// -----------------------------------------------------------------------------------------
//demo functions for turning the power on
#[no_mangle]
pub unsafe extern "C" fn scrTurnPowerOff() -> BOOL {
    //powerCalculated = FALSE;
    powerCalc(0 as libc::c_int);
    return 1 as libc::c_int;
}
//demo functions for turning the power off
// -----------------------------------------------------------------------------------------
//demo functions for turning the power off
#[no_mangle]
pub unsafe extern "C" fn scrTurnPowerOn() -> BOOL {
    //powerCalculated = TRUE;
    powerCalc(1 as libc::c_int);
    return 1 as libc::c_int;
}
//flags when the tutorial is over so that console messages can be turned on again
// -----------------------------------------------------------------------------------------
//flags when the tutorial is over so that console messages can be turned on again
#[no_mangle]
pub unsafe extern "C" fn scrTutorialEnd() -> BOOL {
    initConsoleMessages();
    return 1 as libc::c_int;
}
//function to play a full-screen video in the middle of the game for the selected player
// -----------------------------------------------------------------------------------------
//function to play a full-screen video in the middle of the game for the selected player
#[no_mangle]
pub unsafe extern "C" fn scrPlayVideo() -> BOOL {
    let mut pVideo: *mut STRING =
        0 as *mut STRING; // Arpzzzzzzzzzzzzzzzlksht!
    let mut pText: *mut STRING = 0 as *mut STRING;
    if stackPopParams(2 as libc::c_int, ST_TEXTSTRING as libc::c_int,
                      &mut pVideo as *mut *mut STRING,
                      ST_TEXTSTRING as libc::c_int,
                      &mut pText as *mut *mut STRING) == 0 {
        return 0 as libc::c_int
    }
    seq_ClearSeqList();
    seq_AddSeqToList(pVideo, 0 as *mut STRING, pText, 0 as libc::c_int,
                     0 as libc::c_int as UDWORD);
    seq_StartNextFullScreenVideo();
    return 1 as libc::c_int;
}
//checks to see if there are any droids for the specified player
// -----------------------------------------------------------------------------------------
//checks to see if there are any droids for the specified player
#[no_mangle]
pub unsafe extern "C" fn scrAnyDroidsLeft() -> BOOL {
    let mut player: SDWORD = 0;
    let mut droidsLeft: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAnyUnitsLeft:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3007 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrAnyDroidsLeft\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check the players list for any droid
    droidsLeft = 1 as libc::c_int;
    if apsDroidLists[player as usize].is_null() {
        droidsLeft = 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, droidsLeft as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//function to call when the game is over, plays a message.
// -----------------------------------------------------------------------------------------
//function to call when the game is over, plays a message then does game over stuff.
//
#[no_mangle]
pub unsafe extern "C" fn scrGameOverMessage() -> BOOL {
    let mut gameOver: BOOL = 0;
    let mut psMessage: *mut MESSAGE = 0 as *mut MESSAGE;
    let mut msgType: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut psViewData: *mut VIEWDATA = 0 as *mut VIEWDATA;
    //UDWORD			height;
    if stackPopParams(4 as libc::c_int, ST_INTMESSAGE as libc::c_int,
                      &mut psViewData as *mut *mut VIEWDATA,
                      VAL_INT as libc::c_int, &mut msgType as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut gameOver as *mut BOOL) ==
           0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGameOverMessage:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3047 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrGameOverMessage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //create the message
    psMessage =
        addMessage(msgType as UDWORD, 0 as libc::c_int, player as UDWORD);
    if msgType != MSG_PROXIMITY as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"scrGameOverMessage: Bad message type (MSG_PROXIMITY)\x00" as
                  *const u8 as *const libc::c_char);
    };
    if msgType != MSG_PROXIMITY as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              3054 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"scrGameOverMessage\x00")).as_ptr(),
              b"msgType != MSG_PROXIMITY\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !psMessage.is_null() {
        //we need to set this here so the VIDEO_QUIT callback is not called
        setScriptWinLoseVideo(if gameOver != 0 {
                                  1 as libc::c_int
                              } else { 2 as libc::c_int } as UBYTE);
        // Can't do this cos won't process windows stuff
        // Wait for the video to finish.
		/*while (loop_GetVideoStatus())
		{
			videoLoop();
		}*/
        (*psMessage).pViewData = psViewData as *mut MSG_VIEWDATA;
        displayImmediateMessage(psMessage);
        stopReticuleButtonFlash(6 as libc::c_int as UDWORD);
    }
    //set the data
    //this now called when the video Quit is processed
	//displayGameOver(gameOver);
    return 1 as libc::c_int;
}
//function to call when the game is over
// -----------------------------------------------------------------------------------------
//function to call when the game is over
#[no_mangle]
pub unsafe extern "C" fn scrGameOver() -> BOOL {
    let mut gameOver: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut gameOver as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    /*this function will only be called with gameOver = TRUE when at the end of
    the game so we'll just hard-code what happens!*/
    //don't want this in multiplayer...
    if bMultiPlayer == 0 {
        if gameOver == 1 as libc::c_int && bInTutorial == 0 {
            //we need to set this here so the VIDEO_QUIT callback is not called
            setScriptWinLoseVideo(1 as libc::c_int as UBYTE);
            seq_ClearSeqList();
            seq_AddSeqToList(b"outro.rpl\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as *mut STRING,
                             b"outro.txa\x00" as *const u8 as
                                 *const libc::c_char as *mut STRING,
                             0 as libc::c_int, 0 as libc::c_int as UDWORD);
            seq_StartNextFullScreenVideo();
        }
    }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrAnyFactoriesLeft() -> BOOL {
    let mut player: SDWORD = 0;
    let mut result: BOOL = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAnyFactorysLeft:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3131 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrAnyFactoriesLeft\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check the players list for any structures
    result = 0 as libc::c_int;
    if !apsStructLists[player as usize].is_null() {
        psCurr = apsStructLists[player as usize];
        while !psCurr.is_null() {
            //			if (psCurr->pStructureType->type	== REF_FACTORY OR
//				psCurr->pStructureType->type == REF_CYBORG_FACTORY OR
//				psCurr->pStructureType->type == REF_VTOL_FACTORY )
            if StructIsFactory(psCurr) != 0 {
                result = 1 as libc::c_int;
                break ;
            } else { psCurr = (*psCurr).psNext }
        }
    }
    if stackPushResult(VAL_BOOL, result as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//checks to see if there are any structures (except walls) for the specified player
// -----------------------------------------------------------------------------------------
//checks to see if there are any structures (except walls) for the specified player
#[no_mangle]
pub unsafe extern "C" fn scrAnyStructButWallsLeft() -> BOOL {
    let mut player: SDWORD = 0;
    let mut structuresLeft: BOOL = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAnyStructuresButWallsLeft:player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3176 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrAnyStructButWallsLeft\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check the players list for any structures
    structuresLeft = 1 as libc::c_int;
    if apsStructLists[player as usize].is_null() {
        structuresLeft = 0 as libc::c_int
    } else {
        structuresLeft = 0 as libc::c_int;
        psCurr = apsStructLists[player as usize];
        while !psCurr.is_null() {
            if (*(*psCurr).pStructureType).type_0 !=
                   REF_WALL as libc::c_int as libc::c_uint &&
                   (*(*psCurr).pStructureType).type_0 !=
                       REF_WALLCORNER as libc::c_int as libc::c_uint {
                structuresLeft = 1 as libc::c_int;
                break ;
            } else { psCurr = (*psCurr).psNext }
        }
    }
    if stackPushResult(VAL_BOOL, structuresLeft as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//defines the background audio to play
// -----------------------------------------------------------------------------------------
//defines the background audio to play
#[no_mangle]
pub unsafe extern "C" fn scrPlayBackgroundAudio() -> BOOL {
    let mut pText: *mut STRING = 0 as *mut STRING;
    let mut iVol: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_TEXTSTRING as libc::c_int,
                      &mut pText as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut iVol as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    cdspan_PlayInGameAudio(pText, iVol);
    return 1 as libc::c_int;
}
// cd audio funcs
// -----------------------------------------------------------------------------------------
//defines the CD audio to play
#[no_mangle]
pub unsafe extern "C" fn scrPlayCDAudio() -> BOOL {
    let mut iTrack: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut iTrack as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if war_GetPlayAudioCDs() != 0 { cdAudio_PlayTrack(iTrack); }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrStopCDAudio() -> BOOL {
    if war_GetPlayAudioCDs() != 0 { cdAudio_Stop(); }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrPauseCDAudio() -> BOOL {
    if war_GetPlayAudioCDs() != 0 { cdAudio_Pause(); }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrResumeCDAudio() -> BOOL {
    if war_GetPlayAudioCDs() != 0 { cdAudio_Resume(); }
    return 1 as libc::c_int;
}
// set the retreat point for a player
// -----------------------------------------------------------------------------------------
// set the retreat point for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetRetreatPoint() -> BOOL {
    let mut player: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatPoint: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3289 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetRetreatPoint\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >= mapWidth as SDWORD * 128 as libc::c_int ||
           y < 0 as libc::c_int ||
           y >= mapHeight as SDWORD * 128 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatPoint: coords off map\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3295 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetRetreatPoint\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    asRunData[player as usize].sPos.x = x;
    asRunData[player as usize].sPos.y = y;
    return 1 as libc::c_int;
}
// set the retreat force level
// -----------------------------------------------------------------------------------------
// set the retreat force level
#[no_mangle]
pub unsafe extern "C" fn scrSetRetreatForce() -> BOOL {
    let mut player: SDWORD = 0;
    let mut level: SDWORD = 0;
    let mut numDroids: SDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut level as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatForce: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3319 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetRetreatForce\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if level > 100 as libc::c_int || level < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatForce: level out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3325 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetRetreatForce\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // count up the current number of droids
    numDroids = 0 as libc::c_int;
    psCurr = apsDroidLists[player as usize];
    while !psCurr.is_null() {
        numDroids += 1 as libc::c_int;
        psCurr = (*psCurr).psNext
    }
    asRunData[player as usize].forceLevel =
        (level * numDroids / 100 as libc::c_int) as UBYTE;
    return 1 as libc::c_int;
}
// set the retreat leadership
// -----------------------------------------------------------------------------------------
// set the retreat leadership
#[no_mangle]
pub unsafe extern "C" fn scrSetRetreatLeadership() -> BOOL {
    let mut player: SDWORD = 0;
    let mut level: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut level as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatLeadership: player out of range\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3354 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrSetRetreatLeadership\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if level > 100 as libc::c_int || level < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatLeadership: level out of range\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3360 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrSetRetreatLeadership\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    asRunData[player as usize].leadership = level as UBYTE;
    return 1 as libc::c_int;
}
// set the retreat point for a group
// -----------------------------------------------------------------------------------------
// set the retreat point for a group
#[no_mangle]
pub unsafe extern "C" fn scrSetGroupRetreatPoint() -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if stackPopParams(3 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >= mapWidth as SDWORD * 128 as libc::c_int ||
           y < 0 as libc::c_int ||
           y >= mapHeight as SDWORD * 128 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatPoint: coords off map\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3384 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrSetGroupRetreatPoint\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    (*psGroup).sRunData.sPos.x = x;
    (*psGroup).sRunData.sPos.y = y;
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSetGroupRetreatForce() -> BOOL {
    let mut level: SDWORD = 0;
    let mut numDroids: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut level as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if level > 100 as libc::c_int || level < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatForce: level out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3408 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrSetGroupRetreatForce\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // count up the current number of droids
    numDroids = 0 as libc::c_int;
    psCurr = (*psGroup).psList;
    while !psCurr.is_null() {
        numDroids += 1 as libc::c_int;
        psCurr = (*psCurr).psGrpNext
    }
    (*psGroup).sRunData.forceLevel =
        (level * numDroids / 100 as libc::c_int) as UBYTE;
    return 1 as libc::c_int;
}
// set the retreat health level
// -----------------------------------------------------------------------------------------
// set the retreat health level
#[no_mangle]
pub unsafe extern "C" fn scrSetRetreatHealth() -> BOOL {
    let mut player: SDWORD = 0;
    let mut health: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut health as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetHealthForce: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3437 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetRetreatHealth\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if health > 100 as libc::c_int || health < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetHealthForce: health out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3443 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetRetreatHealth\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    asRunData[player as usize].healthLevel = health as UBYTE;
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSetGroupRetreatHealth() -> BOOL {
    let mut health: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut health as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if health > 100 as libc::c_int || health < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetGroupRetreatHealth: health out of range\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3465 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrSetGroupRetreatHealth\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    (*psGroup).sRunData.healthLevel = health as UBYTE;
    return 1 as libc::c_int;
}
// set the retreat leadership
// -----------------------------------------------------------------------------------------
// set the retreat leadership
#[no_mangle]
pub unsafe extern "C" fn scrSetGroupRetreatLeadership() -> BOOL {
    let mut level: SDWORD = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_INT as libc::c_int, &mut level as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if level > 100 as libc::c_int || level < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRetreatLeadership: level out of range\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3488 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 29],
                                            &[libc::c_char; 29]>(b"scrSetGroupRetreatLeadership\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    (*psGroup).sRunData.leadership = level as UBYTE;
    return 1 as libc::c_int;
}
//start a Mission
// -----------------------------------------------------------------------------------------
//start a Mission - the missionType is ignored now - gets it from the level data ***********
#[no_mangle]
pub unsafe extern "C" fn scrStartMission() -> BOOL {
    let mut pGame: *mut STRING = 0 as *mut STRING;
    let mut missionType: SDWORD = 0;
    let mut psNewLevel: *mut LEVEL_DATASET = 0 as *mut LEVEL_DATASET;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut missionType as *mut SDWORD,
                      ST_LEVEL as libc::c_int, &mut pGame as *mut *mut STRING)
           == 0 {
        return 0 as libc::c_int
    }
    //if (missionType > MISSION_NONE)
    if missionType > LDS_NONE as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Invalid Mission Type\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3513 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrStartMission\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // check the last mission got finished
	/*if (mission.type != MISSION_NONE)
	{
		DBMB(("scrStartMission: old mission incomplete\n   ending mission with success"));
		endMission(TRUE);
	}*/
    // tell the loop that a new level has to be loaded up - not yet!
	//loopNewLevel = TRUE;
    strcpy(pLevelName.as_mut_ptr(), pGame);
    // find the level dataset
    if levFindDataSet(pGame, &mut psNewLevel) == 0 {
        debug(LOG_ERROR,
              b"scrStartMission: couldn\'t find level data\x00" as *const u8
                  as *const libc::c_char);
        abort();
    }
    //set the mission rolling...
	//nextMissionType = missionType;
    nextMissionType = (*psNewLevel).type_0 as SDWORD;
    //	loopMissionState = LMS_SETUPMISSION;
    loopMissionState = LMS_CLEAROBJECTS;
    /*	if (!setUpMission(missionType))
	{
		ASSERT( FALSE, "Unable to start mission - %s", pGame );
		return FALSE;
	}*/
    return 1 as libc::c_int;
}
//end a mission NO LONGER CALLED FROM SCRIPT
//extern BOOL scrEndMission(void);
//set Snow (enable disable snow)
//end a mission - NO LONGER CALLED FROM SCRIPT
/*BOOL scrEndMission(void)
{
	BOOL	status;

	if (!stackPopParams(1, VAL_BOOL, &status))
	{
		return FALSE;
	}

	endMission(status);
	return TRUE;
}*/
// -----------------------------------------------------------------------------------------
//set Snow (enable disable snow)
#[no_mangle]
pub unsafe extern "C" fn scrSetSnow() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    if bState != 0 {
        atmosSetWeatherType(WT_SNOWING);
    } else { atmosSetWeatherType(WT_NONE); }
    return 1 as libc::c_int;
}
//set Rain (enable disable Rain)
// -----------------------------------------------------------------------------------------
//set Rain (enable disable Rain)
#[no_mangle]
pub unsafe extern "C" fn scrSetRain() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    if bState != 0 {
        atmosSetWeatherType(WT_RAINING);
    } else { atmosSetWeatherType(WT_NONE); }
    return 1 as libc::c_int;
}
//set Background Fog (replace fade out with fog)
// -----------------------------------------------------------------------------------------
//set Background Fog (replace fade out with fog)
#[no_mangle]
pub unsafe extern "C" fn scrSetBackgroundFog() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    //jps 17 feb 99 just set the status let other code worry about fogEnable/reveal
    if bState != 0 {
        //true, so go to false
        //restart fog if it was off
        if fogStatus == 0 as libc::c_int as libc::c_uint && war_GetFog() != 0
               && !(bMultiPlayer != 0 && game.fog != 0) {
            pie_EnableFog(1 as libc::c_int); //clear middle bit of 3
        }
        fogStatus |= 1 as libc::c_int as libc::c_uint
    } else {
        fogStatus &= (7 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
        //disable fog if it longer used
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            pie_EnableFog(0 as libc::c_int);
        }
    }
    /* jps 17 feb 99
	if(getRevealStatus())		// fog'o war enabled
	{
		pie_SetFogStatus(FALSE);
		pie_EnableFog(FALSE);
//		fogStatus = 0;
		return TRUE;
	}

	if (bState)//true, so go to false
	{
		if (war_GetFog())
		{
			//restart fog if it was off
			if (fogStatus == 0)
			{
				pie_EnableFog(TRUE);
			}
			fogStatus |= FOG_BACKGROUND;//set lowest bit of 3
		}
	}
	else
	{
		if (war_GetFog())
		{
			fogStatus &= FOG_FLAGS-FOG_BACKGROUND;//clear middle bit of 3
			//disable fog if it longer used
			if (fogStatus == 0)
			{
				pie_SetFogStatus(FALSE);
				pie_EnableFog(FALSE);
			}
		}
	}
*/
    return 1 as libc::c_int;
}
//set Depth Fog (gradual fog from mid range to edge of world)
// -----------------------------------------------------------------------------------------
//set Depth Fog (gradual fog from mid range to edge of world)
#[no_mangle]
pub unsafe extern "C" fn scrSetDepthFog() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    // ffs am
//jps 17 feb 99 just set the status let other code worry about fogEnable/reveal
    if bState != 0 {
        //true, so go to false
        //restart fog if it was off
        if fogStatus == 0 as libc::c_int as libc::c_uint && war_GetFog() != 0
           {
            pie_EnableFog(1 as libc::c_int); //clear middle bit of 3
        }
        fogStatus |= 2 as libc::c_int as libc::c_uint
    } else {
        fogStatus &= (7 as libc::c_int - 2 as libc::c_int) as libc::c_uint;
        //disable fog if it longer used
        if fogStatus == 0 as libc::c_int as libc::c_uint {
            pie_SetFogStatus(0 as libc::c_int);
            pie_EnableFog(0 as libc::c_int);
        }
    }
    /* jps 17 feb 99	if(getRevealStatus())		// fog'o war enabled
	{
		pie_SetFogStatus(FALSE);
		pie_EnableFog(FALSE);
//		fogStatus = 0;
		return TRUE;
	}

	if (bState)//true, so go to false
	{
		if (war_GetFog())
		{
			//restart fog if it was off
			if (fogStatus == 0)
			{
				pie_EnableFog(TRUE);
			}
			fogStatus |= FOG_DISTANCE;//set lowest bit of 3
		}
	}
	else
	{
		if (war_GetFog())
		{
			fogStatus &= FOG_FLAGS-FOG_DISTANCE;//clear middle bit of 3
			//disable fog if it longer used
			if (fogStatus == 0)
			{
				pie_SetFogStatus(FALSE);
				pie_EnableFog(FALSE);
			}
		}
	}
*/
    return 1 as libc::c_int;
}
//set Mission Fog colour, may be modified by weather effects
// -----------------------------------------------------------------------------------------
//set Mission Fog colour, may be modified by weather effects
#[no_mangle]
pub unsafe extern "C" fn scrSetFogColour() -> BOOL {
    let mut red: SDWORD = 0;
    let mut green: SDWORD = 0;
    let mut blue: SDWORD = 0;
    let mut scrFogColour: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut red as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut green as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut blue as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //	if (pie_GetRenderEngine() == ENGINE_GLIDE)
//	{
    red &= 0xff as libc::c_int;
    green &= 0xff as libc::c_int;
    blue &= 0xff as libc::c_int;
    scrFogColour =
        (red << 16 as libc::c_int) + (green << 8 as libc::c_int) + blue;
    pie_SetFogColour(scrFogColour as UDWORD);
    //	}
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// test function to test variable references
#[no_mangle]
pub unsafe extern "C" fn scrRefTest() -> BOOL {
    let mut Num: SDWORD = 0 as libc::c_int;
    (stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int, Num)) == 0;
    return 0 as libc::c_int;
}
// is <player> human or a computer? (multiplayer)
// -----------------------------------------------------------------------------------------
// is player a human or computer player? (multiplayer only)
#[no_mangle]
pub unsafe extern "C" fn scrIsHumanPlayer() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, isHumanPlayer(player as UDWORD)) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Set an alliance between two players
// -----------------------------------------------------------------------------------------
// Set an alliance between two players
#[no_mangle]
pub unsafe extern "C" fn scrCreateAlliance() -> BOOL {
    let mut player1: SDWORD = 0;
    let mut player2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player1 < 0 as libc::c_int || player1 >= 8 as libc::c_int ||
           player2 < 0 as libc::c_int || player2 >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCreateAlliance: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3829 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrCreateAlliance\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if bMultiPlayer != 0 {
        if game.alliance as libc::c_int == 0 as libc::c_int ||
               player1 >= game.maxPlayers as libc::c_int ||
               player2 >= game.maxPlayers as libc::c_int {
            return 1 as libc::c_int
        }
    }
    formAlliance(player1 as UBYTE, player2 as UBYTE, 1 as libc::c_int,
                 0 as libc::c_int);
    /*
#ifndef PSX
	if(bMultiPlayer)
	{

		if(game.alliance==NO_ALLIANCES || player1 >= game.maxPlayers || player2>=game.maxPlayers)
		{
			return TRUE;
		}

		if(alliances[player1][player2] != ALLIANCE_FORMED)
		{
#ifdef DEBUG
			CONPRINTF(ConsoleString,(ConsoleString,"%d and %d form an alliance.",player1,player2));
#endif
			sendAlliance((UBYTE)player1,(UBYTE)player2,ALLIANCE_FORMED,0);
		}
	}
#endif

	alliances[player1][player2] = ALLIANCE_FORMED;
	alliances[player2][player1] = ALLIANCE_FORMED;
*/
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// offer an alliance
#[no_mangle]
pub unsafe extern "C" fn scrOfferAlliance() -> BOOL {
    let mut player1: SDWORD = 0;
    let mut player2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if game.alliance as libc::c_int == 0 as libc::c_int ||
           player1 < 0 as libc::c_int || player1 >= 8 as libc::c_int ||
           player2 < 0 as libc::c_int || player2 >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCreateAlliance: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3884 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrOfferAlliance\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    requestAlliance(player1 as UBYTE, player2 as UBYTE, 1 as libc::c_int,
                    1 as libc::c_int);
    return 1 as libc::c_int;
}
// Break an alliance between two players
// -----------------------------------------------------------------------------------------
// Break an alliance between two players
#[no_mangle]
pub unsafe extern "C" fn scrBreakAlliance() -> BOOL {
    let mut player1: SDWORD = 0;
    let mut player2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player1 < 0 as libc::c_int || player1 >= 8 as libc::c_int ||
           player2 < 0 as libc::c_int || player2 >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCreateAlliance: player out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  3909 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrBreakAlliance\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /*
if(bMultiPlayer)
	{


		if(alliances[player1][player2] != ALLIANCE_BROKEN)
		{
			CONPRINTF(ConsoleString,(ConsoleString,"%d and %d break alliance.",player1,player2));
			sendAlliance((UBYTE)player1,(UBYTE)player2,ALLIANCE_BROKEN,0);
		}
}
*/
    if bMultiPlayer != 0 {
        if game.alliance as libc::c_int == 0 as libc::c_int ||
               player1 >= game.maxPlayers as libc::c_int ||
               player2 >= game.maxPlayers as libc::c_int {
            return 1 as libc::c_int
        }
        breakAlliance(player1 as UBYTE, player2 as UBYTE, 1 as libc::c_int,
                      1 as libc::c_int);
    } else {
        breakAlliance(player1 as UBYTE, player2 as UBYTE, 0 as libc::c_int,
                      1 as libc::c_int);
    }
    /*
	alliances[player1][player2] = ALLIANCE_BROKEN;
	alliances[player2][player1] = ALLIANCE_BROKEN;
*/
    return 1 as libc::c_int;
}
// push true if an alliance still exists.
// -----------------------------------------------------------------------------------------
// Multiplayer relevant scriptfuncs
// returns true if 2 or more players are in alliance.
#[no_mangle]
pub unsafe extern "C" fn scrAllianceExists() -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            if alliances[i as usize][j as usize] as libc::c_int ==
                   3 as libc::c_int {
                if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                    return 0 as libc::c_int
                }
                return 1 as libc::c_int
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrAllianceExistsBetween() -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut i as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut j as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if alliances[i as usize][j as usize] as libc::c_int == 3 as libc::c_int {
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
        return 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// true if player is allied.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrPlayerInAlliance() -> BOOL {
    let mut player: UDWORD = 0;
    let mut j: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    j = 0 as libc::c_int as UDWORD;
    while j < 8 as libc::c_int as libc::c_uint {
        if alliances[player as usize][j as usize] as libc::c_int ==
               3 as libc::c_int {
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
        j = j.wrapping_add(1)
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// push true if group wins are allowed.
//extern BOOL scrAllianceState(void);
// push true if a single alliance is dominant.
// -----------------------------------------------------------------------------------------
// returns true if a single alliance is dominant.
#[no_mangle]
pub unsafe extern "C" fn scrDominatingAlliance() -> BOOL {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < 8 as libc::c_int as libc::c_uint {
            if isHumanPlayer(j) != 0 && isHumanPlayer(i) != 0 && i != j &&
                   alliances[i as usize][j as usize] as libc::c_int !=
                       3 as libc::c_int {
                if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int
                }
                return 1 as libc::c_int
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
        // -----------------------------------------------------------------------------------------
    }
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// push true if human player is responsible for 'player'
#[no_mangle]
pub unsafe extern "C" fn scrMyResponsibility() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if myResponsibility(player as UDWORD) != 0 {
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/*checks to see if a structure of the type specified exists within the
specified range of an XY location */
// -----------------------------------------------------------------------------------------
/*checks to see if a structure of the type specified exists within the
specified range of an XY location */
#[no_mangle]
pub unsafe extern "C" fn scrStructureBuiltInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut found: BOOL = 0;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut psTarget: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    if stackPopParams(5 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4116 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureBuiltInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >> 7 as libc::c_int > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : invalid X coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4122 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureBuiltInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y < 0 as libc::c_int || y >> 7 as libc::c_int > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : invalid Y coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4127 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureBuiltInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if index < 0 as libc::c_int || index > numStructureStats as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : Invalid structure stat\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4132 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureBuiltInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if range < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : Rnage is less than zero\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4137 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureBuiltInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //now look through the players list of structures to see if this type
	//exists within range
    psTarget =
        &mut *asStructureStats.offset(index as isize) as *mut STRUCTURE_STATS;
    rangeSquared = range * range;
    found = 0 as libc::c_int;
    psCurr = apsStructLists[player as usize];
    while !psCurr.is_null() {
        xdiff = (*psCurr).x as SDWORD - x;
        ydiff = (*psCurr).y as SDWORD - y;
        if xdiff * xdiff + ydiff * ydiff <= rangeSquared {
            if strcmp((*(*psCurr).pStructureType).pName, (*psTarget).pName) ==
                   0 as libc::c_int {
                if (*psCurr).status as libc::c_int == SS_BUILT as libc::c_int
                   {
                    found = 1 as libc::c_int;
                    break ;
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    //make sure pass NULL back if not got one
    if found == 0 { psCurr = 0 as *mut STRUCTURE }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psCurr as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// generate a random number
// -----------------------------------------------------------------------------------------
// generate a random number
#[no_mangle]
pub unsafe extern "C" fn scrRandom() -> BOOL {
    let mut range: SDWORD = 0;
    let mut result: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if range == 0 as libc::c_int {
        result = 0 as libc::c_int
    } else if range > 0 as libc::c_int {
        result = rand() % range
    } else { result = rand() % -range }
    if stackPushResult(VAL_INT, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// randomise the random number seed
// -----------------------------------------------------------------------------------------
// randomise the random number seed
#[no_mangle]
pub unsafe extern "C" fn scrRandomiseSeed() -> BOOL {
    srand(clock() as UDWORD);
    return 1 as libc::c_int;
}
//explicitly enables a research topic
// -----------------------------------------------------------------------------------------
//explicitly enables a research topic
#[no_mangle]
pub unsafe extern "C" fn scrEnableResearch() -> BOOL {
    let mut player: SDWORD = 0;
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    if stackPopParams(2 as libc::c_int, ST_RESEARCH as libc::c_int,
                      &mut psResearch as *mut *mut RESEARCH,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrEnableResearch:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4237 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrEnableResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if enableResearch(psResearch, player as UDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//acts as if the research topic was completed - used to jump into the tree
// -----------------------------------------------------------------------------------------
//acts as if the research topic was completed - used to jump into the tree
#[no_mangle]
pub unsafe extern "C" fn scrCompleteResearch() -> BOOL {
    let mut player: SDWORD = 0; //TODO: fix if needed
    let mut psResearch: *mut RESEARCH = 0 as *mut RESEARCH;
    let mut researchIndex: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_RESEARCH as libc::c_int,
                      &mut psResearch as *mut *mut RESEARCH,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCompleteResearch:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4263 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrCompleteResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if psResearch.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCompleteResearch: no such research topic\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4270 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrCompleteResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    researchIndex =
        psResearch.wrapping_offset_from(asResearch) as libc::c_int as UDWORD;
    if researchIndex > numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrCompleteResearch: invalid research index\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4277 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrCompleteResearch\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    researchResult(researchIndex, player as UBYTE, 0 as libc::c_int);
    if bMultiPlayer != 0 && gameTime > 2 as libc::c_int as libc::c_uint {
        SendResearch(player as UBYTE, researchIndex);
    }
    return 1 as libc::c_int;
}
// start a reticule button flashing
// -----------------------------------------------------------------------------------------
// This routine used to start just a reticule button flashing
//   .. now it starts any button flashing (awaiting implmentation from widget library)
#[no_mangle]
pub unsafe extern "C" fn scrFlashOn() -> BOOL {
    let mut button: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut button as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    // For the time being ... we will perform the old code for the reticule ...
    if button >= 2 as libc::c_int && button <= 8 as libc::c_int {
        flashReticuleButton(button as UDWORD);
        return 1 as libc::c_int
    }
    if !widgGetFromID(psWScreen, button as UDWORD).is_null() {
        widgSetButtonFlash(psWScreen, button as UDWORD);
    }
    return 1 as libc::c_int;
}
// stop a reticule button flashing
// -----------------------------------------------------------------------------------------
// stop a generic button flashing
#[no_mangle]
pub unsafe extern "C" fn scrFlashOff() -> BOOL {
    let mut button: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut button as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if button >= 2 as libc::c_int && button <= 8 as libc::c_int {
        stopReticuleButtonFlash(button as UDWORD);
        return 1 as libc::c_int
    }
    if !widgGetFromID(psWScreen, button as UDWORD).is_null() {
        widgClearButtonFlash(psWScreen, button as UDWORD);
    }
    return 1 as libc::c_int;
}
//set the initial power level settings for a player
// -----------------------------------------------------------------------------------------
//set the initial power level settings for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetPowerLevel() -> BOOL {
    let mut player: SDWORD = 0;
    let mut power: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut power as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetPowerLevel:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4359 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetPowerLevel\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setPlayerPower(power as UDWORD, player as UDWORD);
    return 1 as libc::c_int;
}
//add some power for a player
// -----------------------------------------------------------------------------------------
//add some power for a player
#[no_mangle]
pub unsafe extern "C" fn scrAddPower() -> BOOL {
    let mut player: SDWORD = 0;
    let mut power: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut power as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddPower:player number is too high\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4381 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 12],
                                            &[libc::c_char; 12]>(b"scrAddPower\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    addPower(player as UDWORD, power as UDWORD);
    return 1 as libc::c_int;
}
//set the landing Zone position for the map
// -----------------------------------------------------------------------------------------
/*set the landing Zone position for the map - this is for player 0. Can be
scrapped and replaced by setNoGoAreas, left in for compatibility*/
#[no_mangle]
pub unsafe extern "C" fn scrSetLandingZone() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the values - check against max possible since can set in one mission for the next
	//if (x1 > (SDWORD)mapWidth)
    if x1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLandingZone: x1 is greater than max mapWidth\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4406 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetLandingZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (x2 > (SDWORD)mapWidth)
    if x2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLandingZone: x2 is greater than max mapWidth\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4412 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetLandingZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y1 > (SDWORD)mapHeight)
    if y1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLandingZone: y1 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4418 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetLandingZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y2 > (SDWORD)mapHeight)
    if y2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLandingZone: y2 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4424 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetLandingZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check won't overflow!
    if x1 > 0xff as libc::c_int || y1 > 0xff as libc::c_int ||
           x2 > 0xff as libc::c_int || y2 > 0xff as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLandingZone: one coord is greater than %d\x00" as
                      *const u8 as *const libc::c_char, 0xff as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4430 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetLandingZone\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setLandingZone(x1 as UBYTE, y1 as UBYTE, x2 as UBYTE, y2 as UBYTE);
    return 1 as libc::c_int;
}
/*set the landing Zone position for the Limbo droids*/
/*set the landing Zone position for the Limbo droids and adds the Limbo droids
to the world at the location*/
#[no_mangle]
pub unsafe extern "C" fn scrSetLimboLanding() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //check the values - check against max possible since can set in one mission for the next
	//if (x1 > (SDWORD)mapWidth)
    if x1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLimboLanding: x1 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4454 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetLimboLanding\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (x2 > (SDWORD)mapWidth)
    if x2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLimboLanding: x2 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4460 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetLimboLanding\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y1 > (SDWORD)mapHeight)
    if y1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLimboLanding: y1 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4466 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetLimboLanding\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y2 > (SDWORD)mapHeight)
    if y2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLimboLanding: y2 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4472 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetLimboLanding\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check won't overflow!
    if x1 > 0xff as libc::c_int || y1 > 0xff as libc::c_int ||
           x2 > 0xff as libc::c_int || y2 > 0xff as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetLimboLanding: one coord is greater than %d\x00" as
                      *const u8 as *const libc::c_char, 0xff as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4478 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetLimboLanding\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setNoGoArea(x1 as UBYTE, y1 as UBYTE, x2 as UBYTE, y2 as UBYTE,
                8 as libc::c_int as UBYTE);
    //this calls the Droids from the Limbo list onto the map
    placeLimboDroids();
    return 1 as libc::c_int;
}
//initialises all the no go areas
// -----------------------------------------------------------------------------------------
//initialises all the no go areas
#[no_mangle]
pub unsafe extern "C" fn scrInitAllNoGoAreas() -> BOOL {
    initNoGoAreas();
    return 1 as libc::c_int;
}
//set a no go area for the map - landing zones for the enemy, or player 0
// -----------------------------------------------------------------------------------------
//set a no go area for the map - landing zones for the enemy, or player 0
#[no_mangle]
pub unsafe extern "C" fn scrSetNoGoArea() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut area: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut area as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if area == 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: Cannot set the Limbo Landing area with this function\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4513 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check the values - check against max possible since can set in one mission for the next
	//if (x1 > (SDWORD)mapWidth)
    if x1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: x1 is greater than max mapWidth\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4521 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (x2 > (SDWORD)mapWidth)
    if x2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: x2 is greater than max mapWidth\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4527 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y1 > (SDWORD)mapHeight)
    if y1 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: y1 is greater than max mapHeight\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4533 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //if (y2 > (SDWORD)mapHeight)
    if y2 > 256 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: y2 is greater than max mapHeight\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4539 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //check won't overflow!
    if x1 > 0xff as libc::c_int || y1 > 0xff as libc::c_int ||
           x2 > 0xff as libc::c_int || y2 > 0xff as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: one coord is greater than %d\x00" as
                      *const u8 as *const libc::c_char, 0xff as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4545 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if area >= 9 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetNoGoArea: max num of areas is %d\x00" as *const u8
                      as *const libc::c_char, 9 as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4551 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrSetNoGoArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setNoGoArea(x1 as UBYTE, y1 as UBYTE, x2 as UBYTE, y2 as UBYTE,
                area as UBYTE);
    return 1 as libc::c_int;
}
// set the zoom level for the radar
// -----------------------------------------------------------------------------------------
// set the zoom level for the radar
#[no_mangle]
pub unsafe extern "C" fn scrSetRadarZoom() -> BOOL {
    let mut level: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut level as *mut SDWORD) == 0 {
        return 1 as libc::c_int
    }
    // MAX_RADARZOOM is different on PC and PSX
    if level < 0 as libc::c_int || level > 2 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetRadarZoom: zoom level out of range\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4575 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrSetRadarZoom\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    SetRadarZoom(level as UWORD);
    return 1 as libc::c_int;
}
//set how long an offworld mission can last -1 = no limit
// -----------------------------------------------------------------------------------------
//set how long an offworld mission can last -1 = no limit
#[no_mangle]
pub unsafe extern "C" fn scrSetMissionTime() -> BOOL {
    let mut time: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut time as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    time *= 100 as libc::c_int;
    //check not more than one hour - the mission timers cannot cope at present! - (visually)
	//if (time > 60*60*GAME_TICKS_PER_SEC)
	//check not more than 99 mins - the mission timers cannot cope at present! - (visually)
    //we're allowing up to 5 hours now!
    if time >
           5 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int *
               1000 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"The mission timer cannot be set to more than 99!\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4605 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrSetMissionTime\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        time = -(1 as libc::c_int)
    }
    //store the value
    mission.time = time;
    // ffs ab    ... but shouldn't this be on the psx ?
    setMissionCountDown();
    //add the timer to the interface
    if mission.time >= 0 as libc::c_int {
        mission.startTime = gameTime;
        addMissionTimerInterface();
    } else {
        //make sure its not up if setting to -1
        intRemoveMissionTimer();
        //make sure the cheat time is not set
        mission.cheatTime = 0 as libc::c_int as UDWORD
    }
    return 1 as libc::c_int;
}
// this returns how long is left for the current mission time is 1/100th sec - same units as passed in
// this returns how long is left for the current mission time is 1/100th sec - same units as passed in
#[no_mangle]
pub unsafe extern "C" fn scrMissionTimeRemaining() -> BOOL {
    let mut timeRemaining: SDWORD = 0;
    timeRemaining =
        (mission.time as
             libc::c_uint).wrapping_sub(gameTime.wrapping_sub(mission.startTime))
            as SDWORD;
    if timeRemaining < 0 as libc::c_int {
        timeRemaining = 0 as libc::c_int
    } else { timeRemaining /= 100 as libc::c_int }
    if stackPushResult(VAL_INT, timeRemaining) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//set the time delay for reinforcements for an offworld mission
// -----------------------------------------------------------------------------------------
//set the time delay for reinforcements for an offworld mission
#[no_mangle]
pub unsafe extern "C" fn scrSetReinforcementTime() -> BOOL {
    let mut time: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut time as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    time *= 100 as libc::c_int;
    //check not more than one hour - the mission timers cannot cope at present!
    if time != 99999900 as libc::c_int &&
           time > 60 as libc::c_int * 60 as libc::c_int * 1000 as libc::c_int
       {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"The transport timer cannot be set to more than 1 hour!\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4671 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrSetReinforcementTime\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        time = -(1 as libc::c_int)
    }
    //not interseted in this check any more -  AB 28/01/99
    //quick check of the value - don't check if time has not been set
	/*if (mission.time > 0 AND time != LZ_COMPROMISED_TIME AND time > mission.time)
	{
		DBMB(("scrSetReinforcementTime: reinforcement time greater than mission time!"));
	}*/
	//store the value
    mission.ETA = time;
    //if offworld or campaign change mission, then add the timer
	//if (mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR OR
    //    mission.type == LDS_CAMCHANGE)
    if missionCanReEnforce() != 0 { addTransporterTimerInterface(); }
    //make sure the timer is not there if the reinforcement time has been set to < 0
    if time < 0 as libc::c_int {
        intRemoveTransporterTimer();
        /*only remove the launch if haven't got a transporter droid since the
        scripts set the time to -1 at the between stage if there are not going
        to be reinforcements on the submap  */
        psDroid = apsDroidLists[selectedPlayer as usize];
        while !psDroid.is_null() {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                break ;
            }
            psDroid = (*psDroid).psNext
        }
        //if not found a transporter, can remove the launch button
        if psDroid.is_null() { intRemoveTransporterLaunch(); }
    }
    return 1 as libc::c_int;
}
// Sets all structure limits for a player to a specified value
// -----------------------------------------------------------------------------------------
// Sets all structure limits for a player to a specified value
#[no_mangle]
pub unsafe extern "C" fn scrSetAllStructureLimits() -> BOOL {
    let mut player: SDWORD = 0;
    let mut limit: SDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    let mut i: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut limit as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4734 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrSetAllStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if limit < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: limit is less than zero - %d\x00"
                      as *const u8 as *const libc::c_char, limit);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4740 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrSetAllStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if limit > 255 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: limit is too high - %d - must be less than %d\x00"
                      as *const u8 as *const libc::c_char, limit,
                  255 as libc::c_int);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4747 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrSetAllStructureLimits\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //set all the limits to the value specified
    psStructLimits = asStructLimits[player as usize];
    i = 0 as libc::c_int as UDWORD;
    while i < numStructureStats {
        (*psStructLimits.offset(i as isize)).limit = limit as UBYTE;
        (*psStructLimits.offset(i as isize)).globalLimit = limit as UBYTE;
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
// clear all the console messages
// -----------------------------------------------------------------------------------------
// clear all the console messages
#[no_mangle]
pub unsafe extern "C" fn scrFlushConsoleMessages() -> BOOL {
    flushConsoleMessages();
    return 1 as libc::c_int;
}
// establish the distance between two points in world coordinates - approximate bounded to 11% out
// -----------------------------------------------------------------------------------------
// Establishes the distance between two points - uses an approximation
#[no_mangle]
pub unsafe extern "C" fn scrDistanceTwoPts() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut retVal: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Distance between two points - cannot get parameters\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4784 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrDistanceTwoPts\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Approximate the distance */
    retVal = dirtySqrt(x1, y1, x2, y2) as SDWORD;
    if stackPushResult(VAL_INT, retVal) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Distance between two points - cannot return result\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4793 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 18],
                                            &[libc::c_char; 18]>(b"scrDistanceTwoPts\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// decides if a base object can see another - you can select whether walls matter to line of sight
// -----------------------------------------------------------------------------------------
// Returns whether two objects can see each other
#[no_mangle]
pub unsafe extern "C" fn scrLOSTwoBaseObjects() -> BOOL {
    let mut psSource: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDest: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut bWallsBlock: BOOL = 0;
    let mut retVal: BOOL = 0;
    if stackPopParams(3 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psSource as *mut *mut BASE_OBJECT,
                      ST_BASEOBJECT as libc::c_int,
                      &mut psDest as *mut *mut BASE_OBJECT,
                      VAL_BOOL as libc::c_int, &mut bWallsBlock as *mut BOOL)
           == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : scrLOSTwoBaseObjects - cannot get parameters\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4809 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrLOSTwoBaseObjects\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if bWallsBlock != 0 {
        retVal = visibleObjWallBlock(psSource, psDest)
    } else { retVal = visibleObject(psSource, psDest) }
    if stackPushResult(VAL_BOOL, retVal) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : scrLOSTwoBaseObjects - cannot return result\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4824 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrLOSTwoBaseObjects\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// destroys all structures of a certain type within a certain area and gives a gfx effect if you want it
// -----------------------------------------------------------------------------------------
// Destroys all structures within a certain bounding area.
#[no_mangle]
pub unsafe extern "C" fn scrDestroyStructuresInArea() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut typeRef: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNextS: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psNextF: *mut FEATURE = 0 as *mut FEATURE;
    let mut bVisible: BOOL = 0;
    let mut bTakeFeatures: BOOL = 0;
    let mut sX: SDWORD = 0;
    let mut sY: SDWORD = 0;
    if stackPopParams(8 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut typeRef as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bVisible as *mut BOOL, VAL_BOOL as libc::c_int,
                      &mut bTakeFeatures as *mut BOOL) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : scrDestroyStructuresInArea - Cannot get parameters\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4845 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrDestroyStructuresInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Player number too high in scrDestroyStructuresInArea\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4851 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrDestroyStructuresInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    psStructure = apsStructLists[player as usize];
    while !psStructure.is_null() {
        /* Keep a copy */
        psNextS = (*psStructure).psNext;
        sX = (*psStructure).x as SDWORD;
        sY = (*psStructure).y as SDWORD;
        if (*(*psStructure).pStructureType).type_0 == typeRef {
            if sX >= x1 && sX <= x2 && sY >= y1 && sY <= y2 {
                if bVisible != 0 {
                    destroyStruct(psStructure);
                } else { removeStruct(psStructure, 1 as libc::c_int); }
            }
        }
        psStructure = psNextS
    }
    if bTakeFeatures != 0 {
        psFeature = apsFeatureLists[0 as libc::c_int as usize];
        while !psFeature.is_null() {
            /* Keep a copy */
            psNextF = (*psFeature).psNext;
            sX = (*psFeature).x as SDWORD;
            sY = (*psFeature).y as SDWORD;
            if (*(*psFeature).psStats).subType as libc::c_uint ==
                   FEAT_BUILDING as libc::c_int as libc::c_uint {
                //		(psFeature->psStats->subType != FEAT_OIL_DRUM) AND
		  //		(psFeature->psStats->subType != FEAT_OIL_RESOURCE) )
                if sX >= x1 && sX <= x2 && sY >= y1 && sY <= y2 {
                    if bVisible != 0 {
                        destroyFeature(psFeature);
                    } else { removeFeature(psFeature); }
                }
            }
            psFeature = psNextF
        }
    }
    return 1 as libc::c_int;
}
// Estimates a threat from droids within a certain area
// -----------------------------------------------------------------------------------------
// Returns a value representing the threat from droids in a given area
#[no_mangle]
pub unsafe extern "C" fn scrThreatInArea() -> BOOL {
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut ldThreat: SDWORD = 0;
    let mut mdThreat: SDWORD = 0;
    let mut hdThreat: SDWORD = 0;
    let mut playerLooking: UDWORD = 0;
    let mut playerTarget: UDWORD = 0;
    let mut totalThreat: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut dX: SDWORD = 0;
    let mut dY: SDWORD = 0;
    let mut bVisible: BOOL = 0;
    if stackPopParams(10 as libc::c_int, VAL_INT as libc::c_int,
                      &mut playerLooking as *mut UDWORD,
                      VAL_INT as libc::c_int,
                      &mut playerTarget as *mut UDWORD,
                      VAL_INT as libc::c_int, &mut x1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y1 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x2 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y2 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut ldThreat as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut mdThreat as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut hdThreat as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut bVisible as *mut BOOL) ==
           0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : scrThreatInArea - Cannot get parameters\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4924 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrThreatInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if playerLooking >= 8 as libc::c_int as libc::c_uint ||
           playerTarget >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Player number too high in scrThreatInArea\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4930 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrThreatInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    totalThreat = 0 as libc::c_int;
    psDroid = apsDroidLists[playerTarget as usize];
    while !psDroid.is_null() {
        if !((*psDroid).droidType as libc::c_uint !=
                 DROID_WEAPON as libc::c_int as libc::c_uint &&
                 (*psDroid).droidType as libc::c_uint !=
                     DROID_PERSON as libc::c_int as libc::c_uint &&
                 (*psDroid).droidType as libc::c_uint !=
                     DROID_CYBORG as libc::c_int as libc::c_uint &&
                 (*psDroid).droidType as libc::c_uint !=
                     DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
            dX = (*psDroid).x as SDWORD;
            dY = (*psDroid).y as SDWORD;
            /* Do we care if the droid is visible or not */
            if if bVisible != 0 {
                   (*psDroid).visible[playerLooking as usize] as libc::c_int
               } else { 1 as libc::c_int } != 0 {
                /* Have we found a droid in this area */
                if dX >= x1 && dX <= x2 && dY >= y1 && dY <= y2 {
                    match (*asBodyStats.offset((*psDroid).asBits[COMP_BODY as
                                                                     libc::c_int
                                                                     as
                                                                     usize].nStat
                                                   as libc::c_int as
                                                   isize)).size as libc::c_int
                        {
                        0 => { totalThreat += ldThreat }
                        1 => { totalThreat += mdThreat }
                        2 | 3 => { totalThreat += hdThreat }
                        _ => {
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Weird droid size in threat assessment\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if 0 as libc::c_int != 0 {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"scriptfuncs.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      4967 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 16],
                                                                &[libc::c_char; 16]>(b"scrThreatInArea\x00")).as_ptr(),
                                      b"FALSE\x00" as *const u8 as
                                          *const libc::c_char);
                            };
                        }
                    }
                }
            }
        }
        psDroid = (*psDroid).psNext
    }
    //	DBPRINTF(("scrThreatInArea: returning %d\n", totalThreat));
    if stackPushResult(VAL_INT, totalThreat) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot push result in scrThreatInArea\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4976 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrThreatInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// gets the nearest gateway to a list of points
// -----------------------------------------------------------------------------------------
// returns the nearest gateway bottleneck to a specified point
#[no_mangle]
pub unsafe extern "C" fn scrGetNearestGateway() -> BOOL {
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut gX: SDWORD = 0;
    let mut gY: SDWORD = 0;
    let mut nearestSoFar: UDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut psGateway: *mut GATEWAY = 0 as *mut GATEWAY;
    let mut retX: SDWORD = 0;
    let mut retY: SDWORD = 0;
    let mut rX: *mut SDWORD = 0 as *mut SDWORD;
    let mut rY: *mut SDWORD = 0 as *mut SDWORD;
    let mut success: BOOL = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut rX as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut rY as *mut *mut SDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot get parameters for scrGetNearestGateway\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  4996 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetNearestGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x > mapWidth as SDWORD || y < 0 as libc::c_int
           || y > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Invalid coordinates in getNearestGateway\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5002 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetNearestGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if psGateways.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : No gateways found in getNearestGatway\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5008 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetNearestGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    nearestSoFar = 0xffffffff as libc::c_uint;
    retY = 0 as libc::c_int;
    retX = retY;
    success = 0 as libc::c_int;
    psGateway = psGateways;
    while !psGateway.is_null() {
        /* Get gateway midpoint */
        gX =
            ((*psGateway).x1 as libc::c_int + (*psGateway).x2 as libc::c_int)
                / 2 as libc::c_int;
        gY =
            ((*psGateway).y1 as libc::c_int + (*psGateway).y2 as libc::c_int)
                / 2 as libc::c_int;
        /* Estimate the distance to it */
        dist = dirtySqrt(x, y, gX, gY);
        /* Is it best we've found? */
        if dist < nearestSoFar {
            success = 1 as libc::c_int;
            /* Yes, then keep a record of it */
            nearestSoFar = dist;
            retX = gX;
            retY = gY
        }
        psGateway = (*psGateway).psNext
    }
    *rX = retX;
    *rY = retY;
    if stackPushResult(VAL_BOOL, success) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot return result for stackPushResult\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5040 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetNearestGateway\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Lets the user specify which tile goes under water.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSetWaterTile() -> BOOL {
    let mut tileNum: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tileNum as *mut UDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot get parameter for scrSetWaterTile\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5054 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrSetWaterTile\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if tileNum > 96 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Water tile number too high in scrSetWaterTile\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5061 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrSetWaterTile\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setUnderwaterTile(tileNum);
    return 1 as libc::c_int;
}
// lets the user specify which tile	is used for rubble on skyscraper destruction
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSetRubbleTile() -> BOOL {
    let mut tileNum: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut tileNum as *mut UDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot get parameter for scrSetRubbleTile\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5076 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetRubbleTile\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if tileNum > 96 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Rubble tile number too high in scrSetWaterTile\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5083 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetRubbleTile\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setRubbleTile(tileNum);
    return 1 as libc::c_int;
}
// Tells the game what campaign it's in
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrSetCampaignNumber() -> BOOL {
    let mut campaignNumber: UDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut campaignNumber as *mut UDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot get parameter for scrSetCampaignNumber\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5098 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrSetCampaignNumber\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    setCampaignNumber(campaignNumber);
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrGetUnitCount() -> BOOL {
    return 1 as libc::c_int;
}
// tests whether a structure has a module. If structure is null, then any structure
// -----------------------------------------------------------------------------------------
// Tests whether a structure has a certain module for a player. Tests whether any structure
// has this module if structure is null
#[no_mangle]
pub unsafe extern "C" fn scrTestStructureModule() -> BOOL {
    let mut player: SDWORD = 0;
    let mut refId: SDWORD = 0;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut bFound: BOOL = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, ST_STRUCTURE as libc::c_int,
                      &mut psStructure as *mut *mut STRUCTURE,
                      VAL_INT as libc::c_int, &mut refId as *mut SDWORD) == 0
       {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot get parameters in scrTestStructureModule\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5126 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrTestStructureModule\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Player number too high in scrTestStructureModule\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5132 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrTestStructureModule\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Nothing yet */
    bFound = 0 as libc::c_int;
    /* Check the specified case first */
    if !psStructure.is_null() {
        if structHasModule(psStructure) != 0 { bFound = 1 as libc::c_int }
    } else {
        /* psStructure was NULL - so test the general case */
        // Search them all, but exit if we get one!!
        psStruct = apsStructLists[player as usize];
        bFound = 0 as libc::c_int;
        while !psStruct.is_null() && bFound == 0 {
            if structHasModule(psStruct) != 0 { bFound = 1 as libc::c_int }
            psStruct = (*psStruct).psNext
        }
    }
    /* Send back the result */
    if stackPushResult(VAL_BOOL, bFound) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"SCRIPT : Cannot push result for scrTestStructureModule\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5165 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrTestStructureModule\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Sets an object to be a certain percent damaged
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrForceDamage() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut damagePercent: UDWORD = 0;
    let mut divisor: FRACT = 0.;
    let mut newVal: UDWORD = 0;
    /* OK - let's get the vars */
    if stackPopParams(2 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT,
                      VAL_INT as libc::c_int,
                      &mut damagePercent as *mut UDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Cannot pop params for scrForceDamage\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5187 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrForceDamage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Got to be a percent, so must be less than or equal to 100 */
    if damagePercent > 100 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrForceDamage : You\'re supposed to be passing in a PERCENTAGE VALUE, \t\t\tinstead I got given %d, which is clearly no good, now is it!?\x00"
                      as *const u8 as *const libc::c_char, damagePercent);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5195 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrForceDamage\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    /* Get percentage in range [0.1] */
    divisor = damagePercent as FRACT / 100 as libc::c_int as libc::c_float;
    /* See what we're dealing with */
    match (*psObj).type_0 as libc::c_uint {
        0 => {
            psDroid = psObj as *mut DROID;
            newVal =
                (divisor * (*psDroid).originalBody as libc::c_float) as SDWORD
                    as UDWORD;
            (*psDroid).body = newVal
        }
        1 => {
            psStructure = psObj as *mut STRUCTURE;
            newVal =
                (divisor * structureBody(psStructure) as libc::c_float) as
                    SDWORD as UDWORD;
            (*psStructure).body = newVal as UWORD
        }
        2 => {
            psFeature = psObj as *mut FEATURE;
            /* Some features cannot be damaged */
            if (*(*psFeature).psStats).damageable != 0 {
                newVal =
                    (divisor * (*(*psFeature).psStats).body as libc::c_float)
                        as SDWORD as UDWORD;
                (*psFeature).body = newVal
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Unsupported base object type in scrForceDamage\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 5225 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"scrForceDamage\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
// Kills of a droid without spawning any explosion effects.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrDestroyUnitsInArea() -> BOOL {
    let mut psDroid: *mut DROID =
        0 as *mut DROID; // get a copy cos pointer will be lost
    let mut psNext: *mut DROID = 0 as *mut DROID;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut player: UDWORD = 0;
    let mut count: UDWORD = 0 as libc::c_int as UDWORD;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Cannot get params for scrDestroyUnitsInArea\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5244 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrDestroyUnitsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Invalid player number in scrKillDroidsInArea\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5250 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrDestroyUnitsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        if (*psDroid).x as libc::c_int > x1 &&
               ((*psDroid).x as libc::c_int) < x2 &&
               (*psDroid).y as libc::c_int > y1 &&
               ((*psDroid).y as libc::c_int) < y2 {
            /* then it's inside the area */
            destroyDroid(psDroid);
            count = count.wrapping_add(1)
        }
        psDroid = psNext
    }
    if stackPushResult(VAL_INT, count as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Removes a droid from thr world without all the graphical hoo ha.
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn scrRemoveDroid() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Cannot get vars for scrRemoveDroid!\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5278 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrRemoveDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if !psDroid.is_null() { vanishDroid(psDroid); }
    return 1 as libc::c_int;
}
/*
 * ScriptFuncs.c
 *
 * All the C functions callable from the script code
 *
 */
//used in the set nogoArea and LandingZone functions - use the ones defined in Map.h
//#define MAX_MAP_WIDTH		192
//#define MAX_MAP_HEIGHT		128
// If this is defined then check max number of units not reached before adding more.
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn structHasModule(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut bFound: BOOL = 0;
    /* Fail if the structure isn't built yet */
    if (*psStruct).status as libc::c_int != SS_BUILT as libc::c_int {
        return 0 as libc::c_int
    }
    /* Not found yet */
    bFound = 0 as libc::c_int;
    if psStruct.is_null() {
        if !psStruct.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"structHasModule - Testing for a module from a NULL struct - huh!?\x00"
                      as *const u8 as *const libc::c_char);
        };
        if !psStruct.is_null() {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5307 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"structHasModule\x00")).as_ptr(),
                  b"psStruct!=NULL\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if !psStruct.is_null() {
        /* Grab a stats pointer */
        psStats = (*psStruct).pStructureType;
        if StructIsFactory(psStruct) != 0 ||
               (*psStats).type_0 ==
                   REF_POWER_GEN as libc::c_int as libc::c_uint ||
               (*psStats).type_0 ==
                   REF_RESEARCH as libc::c_int as libc::c_uint {
            match (*psStats).type_0 {
                3 => {
                    if (*((*psStruct).pFunctionality as
                              *mut POWER_GEN)).capacity != 0 {
                        bFound = 1 as libc::c_int
                    }
                }
                1 | 17 => {
                    if (*((*psStruct).pFunctionality as
                              *mut FACTORY)).capacity != 0 {
                        bFound = 1 as libc::c_int
                    }
                }
                10 => {
                    if (*((*psStruct).pFunctionality as
                              *mut RESEARCH_FACILITY)).capacity != 0 {
                        bFound = 1 as libc::c_int
                    }
                }
                _ => { }
            }
        } else {
            /* Wrong type of building - cannot have a module */
            bFound = 0 as libc::c_int
        }
    }
    return bFound;
}
// give a player a template from another player
// -----------------------------------------------------------------------------------------
// give player a template belonging to another.
#[no_mangle]
pub unsafe extern "C" fn scrAddTemplate() -> BOOL {
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut player: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_TEMPLATE as libc::c_int,
                      &mut psTemplate as *mut *mut DROID_TEMPLATE,
                      VAL_INT as libc::c_int, &mut player as *mut UDWORD) == 0
       {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrAddTemplate:player number is too high\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5369 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"scrAddTemplate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrAddTemplate: Invalid template pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              5373 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"scrAddTemplate\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    if addTemplate(player, psTemplate) != 0 {
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
// additional structure check
#[no_mangle]
pub unsafe extern "C" fn structDoubleCheck(mut psStat: *mut BASE_STATS,
                                           mut xx: UDWORD, mut yy: UDWORD)
 -> BOOL {
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut xTL: UDWORD = 0;
    let mut yTL: UDWORD = 0;
    let mut xBR: UDWORD = 0;
    let mut yBR: UDWORD = 0;
    let mut count: UBYTE = 0 as libc::c_int as UBYTE;
    let mut psBuilding: *mut STRUCTURE_STATS = psStat as *mut STRUCTURE_STATS;
    xTL = xx.wrapping_sub(1 as libc::c_int as libc::c_uint);
    yTL = yy.wrapping_sub(1 as libc::c_int as libc::c_uint);
    xBR = xx.wrapping_add((*psBuilding).baseWidth);
    yBR = yy.wrapping_add((*psBuilding).baseBreadth);
    // can you get past it?
    y = yTL; // top
    x = xTL; // bottom
    while x != xBR.wrapping_add(1 as libc::c_int as libc::c_uint) {
        if fpathGroundBlockingTile(x as SDWORD, y as SDWORD) != 0 {
            count = count.wrapping_add(1); // left
            break ; // right
        } else { x = x.wrapping_add(1) }
    }
    y = yBR;
    x = xTL;
    while x != xBR.wrapping_add(1 as libc::c_int as libc::c_uint) {
        if fpathGroundBlockingTile(x as SDWORD, y as SDWORD) != 0 {
            count = count.wrapping_add(1);
            break ;
        } else { x = x.wrapping_add(1) }
    }
    x = xTL;
    y = yTL.wrapping_add(1 as libc::c_int as libc::c_uint);
    while y != yBR {
        if fpathGroundBlockingTile(x as SDWORD, y as SDWORD) != 0 {
            count = count.wrapping_add(1);
            break ;
        } else { y = y.wrapping_add(1) }
    }
    x = xBR;
    y = yTL.wrapping_add(1 as libc::c_int as libc::c_uint);
    while y != yBR {
        if fpathGroundBlockingTile(x as SDWORD, y as SDWORD) != 0 {
            count = count.wrapping_add(1);
            break ;
        } else { y = y.wrapping_add(1) }
    }
    if (count as libc::c_int) < 2 as libc::c_int {
        //no more than one blocking side.
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// find and manipulate a position to build a structure.
// pick a structure location(only used in skirmish game at 27Aug) ajl.
#[no_mangle]
pub unsafe extern "C" fn scrPickStructLocation() -> BOOL {
    let mut pX: *mut SDWORD = 0 as *mut SDWORD;
    let mut pY: *mut SDWORD = 0 as *mut SDWORD;
    let mut index: SDWORD = 0;
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut numIterations: UDWORD = 30 as libc::c_int as UDWORD;
    let mut found: BOOL = 0 as libc::c_int;
    let mut startX: UDWORD = 0;
    let mut startY: UDWORD = 0;
    let mut incX: UDWORD = 0;
    let mut incY: UDWORD = 0;
    let mut x: SDWORD = 0 as libc::c_int;
    let mut y: SDWORD = 0 as libc::c_int;
    let mut player: UDWORD = 0;
    if stackPopParams(4 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pX as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut pY as *mut *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut UDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrPickStructLocation:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5475 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrPickStructLocation\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    // check for wacky coords.
    if !(*pX < 0 as libc::c_int ||
             *pX > (mapWidth << 7 as libc::c_int) as SDWORD ||
             *pY < 0 as libc::c_int ||
             *pY > (mapHeight << 7 as libc::c_int) as SDWORD) {
        psStat =
            &mut *asStructureStats.offset(index as isize) as
                *mut STRUCTURE_STATS; // get stat.
        startX =
            (*pX >> 7 as libc::c_int) as UDWORD; // change to tile coords.
        startY = (*pY >> 7 as libc::c_int) as UDWORD;
        incX = 1 as libc::c_int as UDWORD;
        incY = 1 as libc::c_int as UDWORD;
        while incX < numIterations {
            if found == 0 {
                //top
                y = startY.wrapping_sub(incY) as SDWORD;
                x = startX.wrapping_sub(incX) as SDWORD;
                while x < startX.wrapping_add(incX) as SDWORD {
                    if validLocation(psStat as *mut BASE_STATS, x as UDWORD,
                                     y as UDWORD, player, 0 as libc::c_int) !=
                           0 &&
                           structDoubleCheck(psStat as *mut BASE_STATS,
                                             x as UDWORD, y as UDWORD) != 0 {
                        found = 1 as libc::c_int;
                        break ;
                    } else { x += 1 }
                }
            }
            if found == 0 {
                //right
                x = startX.wrapping_add(incX) as SDWORD;
                y = startY.wrapping_sub(incY) as SDWORD;
                while y < startY.wrapping_add(incY) as SDWORD {
                    if validLocation(psStat as *mut BASE_STATS, x as UDWORD,
                                     y as UDWORD, player, 0 as libc::c_int) !=
                           0 &&
                           structDoubleCheck(psStat as *mut BASE_STATS,
                                             x as UDWORD, y as UDWORD) != 0 {
                        found = 1 as libc::c_int;
                        break ;
                    } else { y += 1 }
                }
            }
            if found == 0 {
                //bot
                y = startY.wrapping_add(incY) as SDWORD;
                x = startX.wrapping_add(incX) as SDWORD;
                while x > startX.wrapping_sub(incX) as SDWORD {
                    if validLocation(psStat as *mut BASE_STATS, x as UDWORD,
                                     y as UDWORD, player, 0 as libc::c_int) !=
                           0 &&
                           structDoubleCheck(psStat as *mut BASE_STATS,
                                             x as UDWORD, y as UDWORD) != 0 {
                        found = 1 as libc::c_int;
                        break ;
                    } else { x -= 1 }
                }
            }
            if found == 0 {
                //left
                x = startX.wrapping_sub(incX) as SDWORD;
                y = startY.wrapping_add(incY) as SDWORD;
                while y > startY.wrapping_sub(incY) as SDWORD {
                    if validLocation(psStat as *mut BASE_STATS, x as UDWORD,
                                     y as UDWORD, player, 0 as libc::c_int) !=
                           0 &&
                           structDoubleCheck(psStat as *mut BASE_STATS,
                                             x as UDWORD, y as UDWORD) != 0 {
                        found = 1 as libc::c_int;
                        break ;
                    } else { y -= 1 }
                }
            }
            if found != 0 { break ; }
            incX = incX.wrapping_add(1);
            incY = incY.wrapping_add(1)
        }
        if found != 0 {
            // did It!
            // back to world coords.
            *pX =
                ((x << 7 as libc::c_int) as
                     libc::c_uint).wrapping_add((*psStat).baseWidth.wrapping_mul((128
                                                                                      as
                                                                                      libc::c_int
                                                                                      /
                                                                                      2
                                                                                          as
                                                                                          libc::c_int)
                                                                                     as
                                                                                     libc::c_uint))
                    as SDWORD;
            *pY =
                ((y << 7 as libc::c_int) as
                     libc::c_uint).wrapping_add((*psStat).baseBreadth.wrapping_mul((128
                                                                                        as
                                                                                        libc::c_int
                                                                                        /
                                                                                        2
                                                                                            as
                                                                                            libc::c_int)
                                                                                       as
                                                                                       libc::c_uint))
                    as SDWORD;
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                // success!
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        // failed!
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Sets the transporter entry and exit points for the map
// -----------------------------------------------------------------------------------------
// Sets the transporter entry and exit points for the map
#[no_mangle]
pub unsafe extern "C" fn scrSetTransporterExit() -> BOOL {
    let mut iPlayer: SDWORD = 0;
    let mut iExitTileX: SDWORD = 0;
    let mut iExitTileY: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut iPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iExitTileX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iExitTileY as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    missionSetTransporterExit(iPlayer, iExitTileX, iExitTileY);
    return 1 as libc::c_int;
}
// Fly transporters in at start of map
// -----------------------------------------------------------------------------------------
// Fly transporters in at start of map
#[no_mangle]
pub unsafe extern "C" fn scrFlyTransporterIn() -> BOOL {
    let mut iPlayer: SDWORD = 0;
    let mut iEntryTileX: SDWORD = 0;
    let mut iEntryTileY: SDWORD = 0;
    let mut bTrackTransporter: BOOL = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut iPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iEntryTileX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut iEntryTileY as *mut SDWORD,
                      VAL_BOOL as libc::c_int,
                      &mut bTrackTransporter as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    missionSetTransporterEntry(iPlayer, iEntryTileX, iEntryTileY);
    missionFlyTransportersIn(iPlayer, bTrackTransporter);
    return 1 as libc::c_int;
}
// -----------------------------------------------------------------------------------------
/*
 ** scrGetGameStatus
 *
 *  FILENAME: C:\Deliverance\SrcPSX\ScriptFuncs.c
 *
 *  PARAMETERS: The parameter passed must be one of the STATUS_ variable
 *
 *  DESCRIPTION: Returns various BOOL options in the game	e.g. If the reticule is open
 *      - You should use the externed variable intMode for other game mode options
 *        e.g. in the intelligence screen or desgin screen)
 *
 *  RETURNS:
 *
 */
#[no_mangle]
pub unsafe extern "C" fn scrGetGameStatus() -> BOOL {
    let mut GameChoice: SDWORD = 0;
    let mut result: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut GameChoice as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    //	DBPRINTF(("getgamestatus choice=%d\n",GameChoice));
    result = 0 as libc::c_int; // the default result is false
    match GameChoice {
        0 => {
            if !widgGetFromID(psWScreen, 1 as libc::c_int as UDWORD).is_null()
               {
                result = 1 as libc::c_int
            }
        }
        1 => {
            //			if (driveTacticalActive()==TRUE) result=TRUE;
            if result == 1 as libc::c_int {
                debug(LOG_NEVER,
                      b"battle map active\x00" as *const u8 as
                          *const libc::c_char);
            } else {
                debug(LOG_NEVER,
                      b"battle map notactive\x00" as *const u8 as
                          *const libc::c_char);
            }
        }
        2 => {
            if DeliveryReposValid() == 1 as libc::c_int {
                result = 1 as libc::c_int
            }
        }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"ScrGetGameStatus. Invalid STATUS_ variable\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"scriptfuncs.c\x00" as *const u8 as
                          *const libc::c_char, 5659 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 17],
                                                &[libc::c_char; 17]>(b"scrGetGameStatus\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
        }
    }
    if stackPushResult(VAL_BOOL, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//get the colour number used by a player
//get the colour number used by a player
#[no_mangle]
pub unsafe extern "C" fn scrGetPlayerColour() -> BOOL {
    let mut player: SDWORD = 0;
    let mut colour: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetPlayerColour:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5682 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrGetPlayerColour\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    colour = getPlayerColour(player as UDWORD) as SDWORD;
    if stackPushResult(VAL_INT, colour) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//set the colour number to use for a player
//set the colour number to use for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetPlayerColour() -> BOOL {
    let mut player: SDWORD = 0;
    let mut colour: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut colour as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetPlayerColour:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5708 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetPlayerColour\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if colour >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetPlayerColour:colour number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5714 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrSetPlayerColour\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //not the end of the world if this doesn't work so don't check the return code
    setPlayerColour(player as UDWORD, colour as UDWORD);
    return 1 as libc::c_int;
}
//set all droids in an area to belong to a different player
//set all droids in an area to belong to a different player - returns the number of droids changed
#[no_mangle]
pub unsafe extern "C" fn scrTakeOverDroidsInArea() -> BOOL {
    let mut fromPlayer: SDWORD = 0;
    let mut toPlayer: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut numChanged: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut fromPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut toPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if fromPlayer >= 8 as libc::c_int || toPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5738 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrTakeOverDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: x1 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5744 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrTakeOverDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: x2 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5750 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrTakeOverDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: y1 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5756 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrTakeOverDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: y2 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5762 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrTakeOverDroidsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    numChanged = 0 as libc::c_int;
    psDroid = apsDroidLists[fromPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        //check if within area specified
        if (*psDroid).x as libc::c_int >= x1 &&
               (*psDroid).x as libc::c_int <= x2 &&
               (*psDroid).y as libc::c_int >= y1 &&
               (*psDroid).y as libc::c_int <= y2 {
            //give the droid away
            if !giftSingleDroid(psDroid, toPlayer as UDWORD).is_null() {
                numChanged += 1
            }
        }
        psDroid = psNext
    }
    if stackPushResult(VAL_INT, numChanged) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*this takes over a single droid and passes a pointer back to the new one*/
/*this takes over a single droid and passes a pointer back to the new one*/
#[no_mangle]
pub unsafe extern "C" fn scrTakeOverSingleDroid() -> BOOL {
    let mut playerToGain: SDWORD = 0;
    let mut psDroidToTake: *mut DROID = 0 as *mut DROID;
    let mut psNewDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroidToTake as *mut *mut DROID,
                      VAL_INT as libc::c_int,
                      &mut playerToGain as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if playerToGain >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverSingleUnit:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5803 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrTakeOverSingleDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if psDroidToTake.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverSingleUnit: Null unit\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5809 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 23],
                                            &[libc::c_char; 23]>(b"scrTakeOverSingleDroid\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrTakeOverSingleUnit: Invalid unit pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              5814 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 23],
                                        &[libc::c_char; 23]>(b"scrTakeOverSingleDroid\x00")).as_ptr(),
              b"PTRVALID(psDroidToTake, sizeof(DROID))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psNewDroid = giftSingleDroid(psDroidToTake, playerToGain as UDWORD);
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       psNewDroid as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// set all droids in an area of a certain experience level or less to belong to
// a different player - returns the number of droids changed
// set all droids in an area of a certain experience level or less to belong to
// a different player - returns the number of droids changed
#[no_mangle]
pub unsafe extern "C" fn scrTakeOverDroidsInAreaExp() -> BOOL {
    let mut fromPlayer: SDWORD = 0;
    let mut toPlayer: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut numChanged: SDWORD = 0;
    let mut level: SDWORD = 0;
    let mut maxUnits: SDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psNext: *mut DROID = 0 as *mut DROID;
    if stackPopParams(8 as libc::c_int, VAL_INT as libc::c_int,
                      &mut fromPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut toPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut level as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut maxUnits as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if fromPlayer >= 8 as libc::c_int || toPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5840 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverDroidsInAreaExp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: x1 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5846 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverDroidsInAreaExp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: x2 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5852 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverDroidsInAreaExp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: y1 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5858 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverDroidsInAreaExp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverUnitsInArea: y2 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5864 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverDroidsInAreaExp\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    numChanged = 0 as libc::c_int;
    psDroid = apsDroidLists[fromPlayer as usize];
    while !psDroid.is_null() {
        psNext = (*psDroid).psNext;
        //check if within area specified
        if (*psDroid).droidType as libc::c_uint !=
               DROID_CONSTRUCT as libc::c_int as libc::c_uint &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_REPAIR as libc::c_int as libc::c_uint &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint &&
               (*psDroid).droidType as libc::c_uint !=
                   DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint &&
               (*psDroid).numKills as SDWORD <= level &&
               (*psDroid).x as libc::c_int >= x1 &&
               (*psDroid).x as libc::c_int <= x2 &&
               (*psDroid).y as libc::c_int >= y1 &&
               (*psDroid).y as libc::c_int <= y2 {
            //give the droid away
            if !giftSingleDroid(psDroid, toPlayer as UDWORD).is_null() {
                numChanged += 1
            }
        }
        if numChanged >= maxUnits { break ; }
        psDroid = psNext
    }
    if stackPushResult(VAL_INT, numChanged) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/*this takes over a single structure and passes a pointer back to the new one*/
/*this takes over a single structure and passes a pointer back to the new one*/
#[no_mangle]
pub unsafe extern "C" fn scrTakeOverSingleStructure() -> BOOL {
    let mut playerToGain: SDWORD = 0;
    let mut psStructToTake: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNewStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut structureInc: UDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut psStructToTake as *mut *mut STRUCTURE,
                      VAL_INT as libc::c_int,
                      &mut playerToGain as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if playerToGain >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverSingleStructure:player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5917 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverSingleStructure\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if psStructToTake.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverSingleStructure: Null structure\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5923 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrTakeOverSingleStructure\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrTakeOverSingleStructure: Invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              5928 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 27],
                                        &[libc::c_char; 27]>(b"scrTakeOverSingleStructure\x00")).as_ptr(),
              b"PTRVALID(psStructToTake, sizeof(STRUCTURE))\x00" as *const u8
                  as *const libc::c_char);
    };
    structureInc =
        (*(*psStructToTake).pStructureType).ref_0.wrapping_sub(0xd0000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
    if playerToGain == selectedPlayer as SDWORD &&
           StructIsFactory(psStructToTake) != 0 &&
           (*asStructLimits[playerToGain as
                                usize].offset(structureInc as
                                                  isize)).currentQuantity as
               libc::c_int >= 5 as libc::c_int {
        debug(LOG_NEVER,
              b"scrTakeOverSingleStructure - factory ignored for selectedPlayer\n\x00"
                  as *const u8 as *const libc::c_char);
        psNewStruct = 0 as *mut STRUCTURE
    } else {
        psNewStruct =
            giftSingleStructure(psStructToTake, playerToGain as UBYTE,
                                1 as libc::c_int);
        if !psNewStruct.is_null() {
            //check the structure limits aren't compromised
            if (*asStructLimits[playerToGain as
                                    usize].offset(structureInc as
                                                      isize)).currentQuantity
                   as libc::c_int >
                   (*asStructLimits[playerToGain as
                                        usize].offset(structureInc as
                                                          isize)).limit as
                       libc::c_int {
                (*asStructLimits[playerToGain as
                                     usize].offset(structureInc as
                                                       isize)).limit =
                    (*asStructLimits[playerToGain as
                                         usize].offset(structureInc as
                                                           isize)).currentQuantity
            }
            //for each structure taken - add graphical effect if the selectedPlayer
            if playerToGain == selectedPlayer as SDWORD {
                assignSensorTarget(psNewStruct as *mut BASE_OBJECT);
            }
        }
    }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psNewStruct as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//set all structures in an area to belong to a different player - returns the number of droids changed
//will not work on factories for the selectedPlayer
//set all structures in an area to belong to a different player - returns the number of droids changed
//will not work on factories for the selectedPlayer
#[no_mangle]
pub unsafe extern "C" fn scrTakeOverStructsInArea() -> BOOL {
    let mut fromPlayer: SDWORD = 0;
    let mut toPlayer: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut numChanged: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNext: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNewStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut structureInc: UDWORD = 0;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut fromPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut toPlayer as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if fromPlayer >= 8 as libc::c_int || toPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverStructsInArea:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5980 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrTakeOverStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverStructsInArea: x1 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5986 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrTakeOverStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverStructsInArea: x2 is greater than max mapWidth\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5992 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrTakeOverStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y1 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverStructsInArea: y1 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  5998 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrTakeOverStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y2 > (256 as libc::c_int) << 7 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrTakeOverStructsInArea: y2 is greater than max mapHeight\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6004 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrTakeOverStructsInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    numChanged = 0 as libc::c_int;
    psStruct = apsStructLists[fromPlayer as usize];
    while !psStruct.is_null() {
        psNext = (*psStruct).psNext;
        //check if within area specified
        if (*psStruct).x as libc::c_int >= x1 &&
               (*psStruct).x as libc::c_int <= x2 &&
               (*psStruct).y as libc::c_int >= y1 &&
               (*psStruct).y as libc::c_int <= y2 {
            //changed this so allows takeOver is have less than 5 factories
            //don't work on factories for the selectedPlayer
            structureInc =
                (*(*psStruct).pStructureType).ref_0.wrapping_sub(0xd0000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
            if toPlayer == selectedPlayer as SDWORD &&
                   StructIsFactory(psStruct) != 0 &&
                   (*asStructLimits[toPlayer as
                                        usize].offset(structureInc as
                                                          isize)).currentQuantity
                       as libc::c_int >= 5 as libc::c_int {
                debug(LOG_NEVER,
                      b"scrTakeOverStructsInArea - factory ignored for selectedPlayer\n\x00"
                          as *const u8 as *const libc::c_char);
            } else {
                //give the structure away
                psNewStruct =
                    giftSingleStructure(psStruct, toPlayer as UBYTE,
                                        1 as libc::c_int);
                if !psNewStruct.is_null() {
                    numChanged += 1;
                    //check the structure limits aren't compromised
                    //structureInc = psNewStruct->pStructureType->ref - REF_STRUCTURE_START;
                    if (*asStructLimits[toPlayer as
                                            usize].offset(structureInc as
                                                              isize)).currentQuantity
                           as libc::c_int >
                           (*asStructLimits[toPlayer as
                                                usize].offset(structureInc as
                                                                  isize)).limit
                               as libc::c_int {
                        (*asStructLimits[toPlayer as
                                             usize].offset(structureInc as
                                                               isize)).limit =
                            (*asStructLimits[toPlayer as
                                                 usize].offset(structureInc as
                                                                   isize)).currentQuantity
                    }
                    //for each structure taken - add graphical effect if the selectedPlayer
                    if toPlayer == selectedPlayer as SDWORD {
                        assignSensorTarget(psNewStruct as *mut BASE_OBJECT);
                    }
                }
            }
        }
        psStruct = psNext
    }
    if stackPushResult(VAL_INT, numChanged) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//set Flag for defining what happens to the droids in a Transporter
//set Flag for defining what happens to the droids in a Transporter
#[no_mangle]
pub unsafe extern "C" fn scrSetDroidsToSafetyFlag() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    setDroidsToSafetyFlag(bState);
    return 1 as libc::c_int;
}
//set Flag for defining whether the coded countDown is called
//set Flag for defining whether the coded countDown is called
#[no_mangle]
pub unsafe extern "C" fn scrSetPlayCountDown() -> BOOL {
    let mut bState: BOOL = 0;
    if stackPopParams(1 as libc::c_int, VAL_BOOL as libc::c_int,
                      &mut bState as *mut BOOL) == 0 {
        return 0 as libc::c_int
    }
    setPlayCountDown(bState as UBYTE);
    return 1 as libc::c_int;
}
//get the number of droids currently onthe map for a player
//get the number of droids currently onthe map for a player
#[no_mangle]
pub unsafe extern "C" fn scrGetDroidCount() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetUnitCount:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6101 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrGetDroidCount\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT, getNumDroids(player as UDWORD) as SDWORD) == 0
       {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// fire a weapon stat at an object
// fire a weapon stat at an object
#[no_mangle]
pub unsafe extern "C" fn scrFireWeaponAtObj() -> BOOL {
    let mut wIndex: SDWORD = 0;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut sWeapon: WEAPON =
        WEAPON{nStat: 0,
               hitPoints: 0,
               ammo: 0,
               lastFired: 0,
               recoilValue: 0,};
    if stackPopParams(2 as libc::c_int, ST_WEAPON as libc::c_int,
                      &mut wIndex as *mut SDWORD,
                      ST_BASEOBJECT as libc::c_int,
                      &mut psTarget as *mut *mut BASE_OBJECT) == 0 {
        return 0 as libc::c_int
    }
    if psTarget.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrFireWeaponAtObj: Null target pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6128 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrFireWeaponAtObj\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    memset(&mut sWeapon as *mut WEAPON as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<WEAPON>() as libc::c_ulong);
    sWeapon.nStat = wIndex as UDWORD;
    // send the projectile using the selectedPlayer so that it can always be seen
    proj_SendProjectile(&mut sWeapon, 0 as *mut BASE_OBJECT,
                        selectedPlayer as SDWORD, (*psTarget).x as UDWORD,
                        (*psTarget).y as UDWORD, (*psTarget).z as UDWORD,
                        psTarget, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// fire a weapon stat at a location
// fire a weapon stat at a location
#[no_mangle]
pub unsafe extern "C" fn scrFireWeaponAtLoc() -> BOOL {
    let mut wIndex: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut sWeapon: WEAPON =
        WEAPON{nStat: 0,
               hitPoints: 0,
               ammo: 0,
               lastFired: 0,
               recoilValue: 0,};
    if stackPopParams(3 as libc::c_int, ST_WEAPON as libc::c_int,
                      &mut wIndex as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    memset(&mut sWeapon as *mut WEAPON as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<WEAPON>() as libc::c_ulong);
    sWeapon.nStat = wIndex as UDWORD;
    // send the projectile using the selectedPlayer so that it can always be seen
    proj_SendProjectile(&mut sWeapon, 0 as *mut BASE_OBJECT,
                        selectedPlayer as SDWORD, x as UDWORD, y as UDWORD,
                        map_Height(x as UDWORD, y as UDWORD) as UDWORD,
                        0 as *mut BASE_OBJECT, 1 as libc::c_int);
    return 1 as libc::c_int;
}
// set the number of kills for a droid
// set the number of kills for a droid
#[no_mangle]
pub unsafe extern "C" fn scrSetDroidKills() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut kills: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut kills as *mut SDWORD) == 0 {
        return 1 as libc::c_int
    }
    if psDroid.is_null() ||
           (*psDroid).type_0 as libc::c_uint !=
               OBJ_DROID as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetUnitKills: NULL/invalid unit pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6175 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"scrSetDroidKills\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    (*psDroid).numKills = kills as UWORD;
    return 1 as libc::c_int;
}
// reset the visibility for a player
// reset the visibility for a player
#[no_mangle]
pub unsafe extern "C" fn scrResetPlayerVisibility() -> BOOL {
    let mut player: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player > 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrResetPlayerVisibility: invalid player\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6197 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrResetPlayerVisibility\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(i == player) {
            psObj = apsDroidLists[i as usize] as *mut BASE_OBJECT;
            while !psObj.is_null() {
                (*psObj).visible[player as usize] = 0 as libc::c_int as UBYTE;
                psObj = (*psObj).psNext
            }
            psObj = apsStructLists[i as usize] as *mut BASE_OBJECT;
            while !psObj.is_null() {
                (*psObj).visible[player as usize] = 0 as libc::c_int as UBYTE;
                psObj = (*psObj).psNext
            }
        }
        i += 1
    }
    psObj = apsFeatureLists[0 as libc::c_int as usize] as *mut BASE_OBJECT;
    while !psObj.is_null() {
        (*psObj).visible[player as usize] = 0 as libc::c_int as UBYTE;
        psObj = (*psObj).psNext
    }
    clustResetVisibility(player);
    return 1 as libc::c_int;
}
// set the vtol return pos for a player
// set the vtol return pos for a player
#[no_mangle]
pub unsafe extern "C" fn scrSetVTOLReturnPos() -> BOOL {
    let mut player: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut tx as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut ty as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetVTOLReturnPos: invalid player\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6242 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"scrSetVTOLReturnPos\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    asVTOLReturnPos[player as usize].x =
        tx * 128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
    asVTOLReturnPos[player as usize].y =
        ty * 128 as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
    return 1 as libc::c_int;
}
//called via the script in a Limbo Expand level to set the level to plain ol' expand
//called via the script in a Limbo Expand level to set the level to plain ol' expand
#[no_mangle]
pub unsafe extern "C" fn scrResetLimboMission() -> BOOL {
    //check currently on a Limbo expand mission
    if missionLimboExpand() == 0 {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrResetLimboMission: current mission type invalid\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6258 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrResetLimboMission\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    //turn it into an expand mission
    resetLimboMission();
    return 1 as libc::c_int;
}
// skirmish function **NOT PSX**
// skirmish only.
#[no_mangle]
pub unsafe extern "C" fn scrIsVtol() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut result: BOOL = 0;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        return 1 as libc::c_int
    }
    if psDroid.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrIsVtol: null droid passed in.\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6283 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 10],
                                            &[libc::c_char; 10]>(b"scrIsVtol\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
    }
    result = vtolDroid(psDroid);
    if stackPushResult(VAL_BOOL, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// init templates for tutorial.
// do the setting up of the template list for the tutorial.
#[no_mangle]
pub unsafe extern "C" fn scrTutorialTemplates() -> BOOL {
    let mut psCurr: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psPrev: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut pName: [STRING; 60] = [0; 60];
    // find ViperLtMGWheels
    strcpy(pName.as_mut_ptr(),
           b"ViperLtMGWheels\x00" as *const u8 as *const libc::c_char);
    if getResourceName(pName.as_mut_ptr()) == 0 {
        debug(LOG_ERROR,
              b"tutorial template setup failed\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    getDroidResourceName(pName.as_mut_ptr());
    psCurr = apsDroidTemplates[selectedPlayer as usize];
    psPrev = 0 as *mut DROID_TEMPLATE;
    while !psCurr.is_null() {
        if strcmp(pName.as_mut_ptr(), (*psCurr).aName.as_mut_ptr()) ==
               0 as libc::c_int {
            if !psPrev.is_null() {
                (*psPrev).psNext = (*psCurr).psNext
            } else {
                apsDroidTemplates[selectedPlayer as usize] = (*psCurr).psNext
            }
            break ;
        } else { psPrev = psCurr; psCurr = (*psCurr).psNext }
    }
    // Delete the template.
    if !psCurr.is_null() {
        heapFree(psTemplateHeap, psCurr as *mut libc::c_void);
    } else {
        debug(LOG_ERROR,
              b"tutorial template setup failed\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    return 1 as libc::c_int;
}
//-----------------------------------------
//New functions
//-----------------------------------------
//-----------------------------------------
//New functions
//-----------------------------------------
//compare two strings (0 means they are different)
#[no_mangle]
pub unsafe extern "C" fn scrStrcmp() -> BOOL {
    let mut ssval1: *mut STRING = 0 as *mut STRING;
    let mut ssval2: *mut STRING = 0 as *mut STRING;
    if stackPopParams(2 as libc::c_int, VAL_STRING as libc::c_int,
                      &mut ssval1 as *mut *mut STRING,
                      VAL_STRING as libc::c_int,
                      &mut ssval2 as *mut *mut STRING) == 0 {
        debug(LOG_ERROR,
              b"scrStrcmp(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, (strcmp(ssval1, ssval2) == 0) as libc::c_int)
           == 0 {
        debug(LOG_ERROR,
              b"scrStrcmp: failed to push result\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* Output a string to console */
#[no_mangle]
pub unsafe extern "C" fn scrConsole() -> BOOL {
    let mut ssval: *mut STRING = 0 as *mut STRING;
    if stackPopParams(1 as libc::c_int, VAL_STRING as libc::c_int,
                      &mut ssval as *mut *mut STRING) == 0 {
        debug(LOG_ERROR,
              b"scrConsole(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    addConsoleMessage(ssval, DEFAULT_JUSTIFY);
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut scrDebug: [BOOL; 8] = [0; 8];
//turn on debug messages
#[no_mangle]
pub unsafe extern "C" fn scrDbgMsgOn() -> BOOL {
    let mut bOn: BOOL = 0;
    let mut player: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bOn as *mut BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrDbgMsgOn(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int || player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrDbgMsgOn(): wrong player number\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    scrDebug[player as usize] = bOn;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrMsg() -> BOOL {
    let mut playerTo: SDWORD = 0;
    let mut playerFrom: SDWORD = 0;
    let mut ssval: *mut STRING = 0 as *mut STRING;
    let mut tmp: [STRING; 255] = [0; 255];
    if stackPopParams(3 as libc::c_int, VAL_STRING as libc::c_int,
                      &mut ssval as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut playerFrom as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut playerTo as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrMsg(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if playerFrom < 0 as libc::c_int || playerFrom >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrMsg(): playerFrom out of range\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if playerTo < 0 as libc::c_int || playerTo >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrMsg(): playerTo out of range\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //sendAIMessage(ssval, playerFrom, playerTo);		//TODO: implement multiplayer messages
    //show the message we sent on our local console as well (even in skirmish, if player plays as this AI)
    if playerFrom as libc::c_uint == selectedPlayer {
        sprintf(tmp.as_mut_ptr(),
                b"[%d-%d] : %s\x00" as *const u8 as *const libc::c_char,
                playerFrom, playerTo, ssval); // add message
        addConsoleMessage(tmp.as_mut_ptr(), RIGHT_JUSTIFY);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrDbg() -> BOOL {
    let mut ssval: *mut STRING = 0 as *mut STRING;
    let mut player: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_STRING as libc::c_int,
                      &mut ssval as *mut *mut STRING, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrDbg(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if scrDebug[player as usize] != 0 {
        let mut sTmp: *mut STRING = 0 as *mut STRING;
        sTmp =
            memMallocRelease(255 as libc::c_int as size_t) as
                *mut libc::c_char;
        sprintf(sTmp, b"%d) %s\x00" as *const u8 as *const libc::c_char,
                player, ssval);
        addConsoleMessage(sTmp, DEFAULT_JUSTIFY);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrDebugFile() -> BOOL {
    let mut ssval: *mut STRING = 0 as *mut STRING;
    if stackPopParams(1 as libc::c_int, VAL_STRING as libc::c_int,
                      &mut ssval as *mut *mut STRING) == 0 {
        debug(LOG_ERROR,
              b"scrDebugFile(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    debug(LOG_SCRIPT, ssval);
    return 1 as libc::c_int;
}
static mut playerToEnumDroid: UDWORD = 0;
static mut playerVisibleDroid: UDWORD = 0;
static mut enumDroidCount: UDWORD = 0;
/* Prepare the droid iteration */
#[no_mangle]
pub unsafe extern "C" fn scrInitEnumDroids() -> BOOL {
    let mut targetplayer: SDWORD = 0;
    let mut playerVisible: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut targetplayer as *mut SDWORD,
                      VAL_INT as libc::c_int,
                      &mut playerVisible as *mut SDWORD) == 0 {
        //DbgMsg("scrInitEnumDroids() - failed to pop params");
        return 0 as libc::c_int
    } //returned 0 droids so far
    playerToEnumDroid = targetplayer as UDWORD;
    playerVisibleDroid = playerVisible as UDWORD;
    enumDroidCount = 0 as libc::c_int as UDWORD;
    return 1 as libc::c_int;
}
/* Get next droid */
#[no_mangle]
pub unsafe extern "C" fn scrEnumDroid() -> BOOL {
    let mut count: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut found: BOOL = 0;
    count = 0 as libc::c_int as UDWORD;
    psDroid = apsDroidLists[playerToEnumDroid as usize];
    while !psDroid.is_null() && count < enumDroidCount {
        psDroid = (*psDroid).psNext;
        count = count.wrapping_add(1)
    }
    //search the players' list of droid to see if one exists and is visible
    found = 0 as libc::c_int;
    while !psDroid.is_null() {
        if (*psDroid).visible[playerVisibleDroid as usize] != 0 {
            if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                               psDroid as UDWORD as SDWORD) == 0 {
                //	push result
                return 0 as libc::c_int
            }
            enumDroidCount = enumDroidCount.wrapping_add(1);
            return 1 as libc::c_int
        }
        enumDroidCount = enumDroidCount.wrapping_add(1);
        psDroid = (*psDroid).psNext
    }
    // push NULLDROID, since didn't find any
    if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                       0 as *mut libc::c_void as UDWORD as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrEnumDroid() - push failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Return the template factory is currently building
#[no_mangle]
pub unsafe extern "C" fn scrFactoryGetTemplate() -> BOOL {
    let mut structure: SDWORD = 0; //Convert
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if stackPopParams(1 as libc::c_int, ST_STRUCTURE as libc::c_int,
                      &mut structure as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate() - stackPopParams failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    psStructure = structure as *mut STRUCTURE;
    if psStructure.is_null() {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate() - NULL factory object\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrFactoryGetTemplate: NULL factory object\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6578 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"scrFactoryGetTemplate\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate: Invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              6583 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrFactoryGetTemplate\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psStructure).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate: structure is not a factory\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*(*psStructure).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*psStructure).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              6587 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrFactoryGetTemplate\x00")).as_ptr(),
              b"(psStructure->pStructureType->type == REF_FACTORY OR psStructure->pStructureType->type == REF_CYBORG_FACTORY OR psStructure->pStructureType->type == REF_VTOL_FACTORY)\x00"
                  as *const u8 as *const libc::c_char);
    };
    if StructIsFactory(psStructure) == 0 {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate: structure not a factory.\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    psTemplate =
        (*((*psStructure).pFunctionality as *mut FACTORY)).psSubject as
            *mut DROID_TEMPLATE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate: Invalid template pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              6598 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"scrFactoryGetTemplate\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    if stackPushResult(ST_TEMPLATE as libc::c_int as INTERP_TYPE,
                       psTemplate as UDWORD as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrFactoryGetTemplate: stackPushResult failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumTemplatesInProduction() -> BOOL {
    let mut player: SDWORD = 0;
    let mut numTemplates: SDWORD = 0 as libc::c_int;
    let mut psTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psList: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psBaseStats: *mut BASE_STATS = 0 as *mut BASE_STATS;
    if stackPopParams(2 as libc::c_int, ST_TEMPLATE as libc::c_int,
                      &mut psTemplate as *mut *mut DROID_TEMPLATE,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumTemplatesInProduction: stackPopParams failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrNumTemplatesInProduction: player number is too high\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumTemplatesInProduction: player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6626 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 28],
                                            &[libc::c_char; 28]>(b"scrNumTemplatesInProduction\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrNumTemplatesInProduction: Invalid template pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              6631 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 28],
                                        &[libc::c_char; 28]>(b"scrNumTemplatesInProduction\x00")).as_ptr(),
              b"PTRVALID(psTemplate, sizeof(DROID_TEMPLATE))\x00" as *const u8
                  as *const libc::c_char);
    };
    psBaseStats = psTemplate as *mut BASE_STATS;
    psList = apsStructLists[player as usize];
    psStruct = psList;
    while !psStruct.is_null() {
        if StructIsFactory(psStruct) != 0 {
            let mut psFactory: *mut FACTORY =
                (*psStruct).pFunctionality as *mut FACTORY;
            //if this is the template currently being worked on
            if psBaseStats == (*psFactory).psSubject { numTemplates += 1 }
        }
        psStruct = (*psStruct).psNext
    }
    if stackPushResult(VAL_INT, numTemplates) == 0 {
        debug(LOG_ERROR,
              b"scrNumTemplatesInProduction: stackPushResult failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Returns number of units based on a component a certain player has
#[no_mangle]
pub unsafe extern "C" fn scrNumDroidsByComponent() -> BOOL {
    let mut player: SDWORD = 0; //cache access
    let mut lookingPlayer: SDWORD = 0;
    let mut comp: SDWORD = 0;
    let mut numFound: UDWORD = 0;
    let mut sVal: INTERP_VAL =
        INTERP_VAL{type_0: VAL_BOOL, v: C2RustUnnamed{bval: 0,},};
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumDroidsByComponent(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrNumDroidsByComponent(): player number is too high\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumDroidsByComponent:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6677 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrNumDroidsByComponent\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if stackPop(&mut sVal) == 0 {
        debug(LOG_ERROR,
              b"scrNumDroidsByComponent(): failed to pop component\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    numFound = 0 as libc::c_int as UDWORD;
    comp = sVal.v.ival;
    //check droids
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).visible[lookingPlayer as usize] != 0 {
            //can see this droid?
            match sVal.type_0 as libc::c_uint {
                13 => {
                    if (*psDroid).asBits[COMP_BODY as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                14 => {
                    if (*psDroid).asBits[COMP_PROPULSION as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                15 => {
                    if (*psDroid).asBits[COMP_ECM as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                16 => {
                    if (*psDroid).asBits[COMP_SENSOR as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                17 => {
                    if (*psDroid).asBits[COMP_CONSTRUCT as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                19 => {
                    if (*psDroid).asBits[COMP_REPAIRUNIT as libc::c_int as
                                             usize].nStat as libc::c_int ==
                           comp {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                18 => {
                    if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat ==
                           comp as libc::c_uint {
                        numFound = numFound.wrapping_add(1)
                    }
                }
                _ => {
                    debug(LOG_ERROR,
                          b"scrNumDroidsByComponent(): unknown component type\x00"
                              as *const u8 as *const libc::c_char);
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"scrNumDroidsByComponent: unknown component type\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"scriptfuncs.c\x00" as *const u8 as
                                  *const libc::c_char, 6743 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 24],
                                                        &[libc::c_char; 24]>(b"scrNumDroidsByComponent\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as libc::c_int
                }
            }
        }
        psDroid = (*psDroid).psNext
    }
    if stackPushResult(VAL_INT, numFound as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumDroidsByComponent(): stackPushResult failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrGetStructureLimit() -> BOOL {
    let mut player: SDWORD = 0;
    let mut limit: SDWORD = 0;
    let mut structInc: UDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetStructureLimit(): stackPopParams failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrGetStructureLimit(): player number is too high\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6773 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetStructureLimit\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if structInc > numStructureStats {
        debug(LOG_ERROR,
              b"scrGetStructureLimit(): tructure stat is too high - %d\x00" as
                  *const u8 as *const libc::c_char, structInc);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: Structure stat is too high - %d\x00"
                      as *const u8 as *const libc::c_char, structInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6779 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrGetStructureLimit\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStructLimits = asStructLimits[player as usize];
    limit = (*psStructLimits.offset(structInc as isize)).limit as SDWORD;
    if stackPushResult(VAL_INT, limit) == 0 {
        debug(LOG_ERROR,
              b"scrGetStructureLimit(): stackPushResult failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Returns TRUE if limit for the passed structurestat is reached, otherwise returns FALSE
#[no_mangle]
pub unsafe extern "C" fn scrStructureLimitReached() -> BOOL {
    let mut player: SDWORD = 0;
    let mut bLimit: BOOL = 0 as libc::c_int;
    let mut structInc: UDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrStructureLimitReached(): stackPopParams failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrStructureLimitReached(): player number is too high\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6811 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureLimitReached\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if structInc > numStructureStats {
        debug(LOG_ERROR,
              b"scrStructureLimitReached(): Structure stat is too high - %d\x00"
                  as *const u8 as *const libc::c_char, structInc);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits: Structure stat is too high - %d\x00"
                      as *const u8 as *const libc::c_char, structInc);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6818 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 25],
                                            &[libc::c_char; 25]>(b"scrStructureLimitReached\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStructLimits = asStructLimits[player as usize];
    if (*psStructLimits.offset(structInc as isize)).currentQuantity as
           libc::c_int >=
           (*psStructLimits.offset(structInc as isize)).limit as libc::c_int {
        bLimit = 1 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, bLimit) == 0 {
        debug(LOG_ERROR,
              b"scrStructureLimitReached(): stackPushResult failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// How many structures of a given type a player has
#[no_mangle]
pub unsafe extern "C" fn scrGetNumStructures() -> BOOL {
    let mut player: SDWORD = 0;
    let mut numStructures: SDWORD = 0;
    let mut structInc: UDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    if stackPopParams(2 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut structInc as *mut UDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrSetStructureLimits: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrSetStructureLimits:player number is too high\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if structInc > numStructureStats {
        debug(LOG_ERROR,
              b"scrSetStructureLimits: Structure stat is too high\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    psStructLimits = asStructLimits[player as usize];
    numStructures =
        (*psStructLimits.offset(structInc as isize)).currentQuantity as
            SDWORD;
    if stackPushResult(VAL_INT, numStructures) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Return player's unit limit
#[no_mangle]
pub unsafe extern "C" fn scrGetUnitLimit() -> BOOL {
    let mut player: SDWORD = 0;
    let mut limit: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetUnitLimit: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrSetStructureLimits:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  6879 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"scrGetUnitLimit\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    limit = getMaxDroids(player as UDWORD) as SDWORD;
    if stackPushResult(VAL_INT, limit) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Return minimum of 2 vals
#[no_mangle]
pub unsafe extern "C" fn scrMin() -> BOOL {
    let mut val1: SDWORD = 0;
    let mut val2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut val2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT, if val2 < val1 { val2 } else { val1 }) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// Return maximum of 2 vals
#[no_mangle]
pub unsafe extern "C" fn scrMax() -> BOOL {
    let mut val1: SDWORD = 0;
    let mut val2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut val1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut val2 as *mut SDWORD) == 0 {
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT, if val1 < val2 { val2 } else { val1 }) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ThreatInRange(mut player: SDWORD, mut range: SDWORD,
                                       mut rangeX: SDWORD, mut rangeY: SDWORD,
                                       mut bVTOLs: BOOL) -> BOOL {
    let mut i: UDWORD = 0;
    let mut structType: UDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    tx = (rangeX >> 7 as libc::c_int) as UDWORD;
    ty = (rangeY >> 7 as libc::c_int) as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if !(alliances[player as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == player as libc::c_uint) {
            //check structures
            psStruct = apsStructLists[i as usize];
            while !psStruct.is_null() {
                if (*psStruct).visible[player as usize] != 0 {
                    //if can see it
                    structType = (*(*psStruct).pStructureType).type_0;
                    match structType {
                        6 | 16 | 1 | 17 | 19 => {
                            //dangerous to get near these structures
                            if range < 0 as libc::c_int ||
                                   (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                              (*psStruct).x as libc::c_int >>
                                                  7 as libc::c_int,
                                              (*psStruct).y as libc::c_int >>
                                                  7 as libc::c_int) <<
                                        7 as libc::c_int) <
                                       range as libc::c_uint {
                                //enemy in range
                                return 1 as libc::c_int
                            }
                        }
                        _ => { }
                    }
                }
                psStruct = (*psStruct).psNext
            }
            //check droids
            psDroid = apsDroidLists[i as usize];
            while !psDroid.is_null() {
                if (*psDroid).visible[player as usize] != 0 {
                    //can see this droid?
                    if !((*psDroid).droidType as libc::c_uint !=
                             DROID_WEAPON as libc::c_int as libc::c_uint &&
                             (*psDroid).droidType as libc::c_uint !=
                                 DROID_PERSON as libc::c_int as libc::c_uint
                             &&
                             (*psDroid).droidType as libc::c_uint !=
                                 DROID_CYBORG as libc::c_int as libc::c_uint
                             &&
                             (*psDroid).droidType as libc::c_uint !=
                                 DROID_CYBORG_SUPER as libc::c_int as
                                     libc::c_uint) {
                        //if VTOLs are excluded, skip them
                        if !(bVTOLs == 0 &&
                                 ((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].nStat
                                                                 as
                                                                 isize)).propulsionType
                                      as libc::c_int == LIFT as libc::c_int ||
                                      (*psDroid).droidType as libc::c_uint ==
                                          DROID_TRANSPORTER as libc::c_int as
                                              libc::c_uint)) {
                            if range < 0 as libc::c_int ||
                                   (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                              (*psDroid).x as libc::c_int >>
                                                  7 as libc::c_int,
                                              (*psDroid).y as libc::c_int >>
                                                  7 as libc::c_int) <<
                                        7 as libc::c_int) <
                                       range as libc::c_uint {
                                //enemy in range
                                return 1 as libc::c_int
                            }
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
//find unrevealed tile closest to pwLooker within the range of wRange
#[no_mangle]
pub unsafe extern "C" fn scrFogTileInRange() -> BOOL {
    let mut pwLookerX: SDWORD = 0;
    let mut pwLookerY: SDWORD = 0;
    let mut tBestX: SDWORD = 0;
    let mut tBestY: SDWORD = 0;
    let mut threadRange: SDWORD = 0;
    let mut wRangeX: SDWORD = 0;
    let mut wRangeY: SDWORD = 0;
    let mut tRangeX: SDWORD = 0;
    let mut tRangeY: SDWORD = 0;
    let mut wRange: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut wDist: UDWORD = 0;
    let mut wBestDist: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut ok: BOOL = 0 as libc::c_int;
    let mut wTileX: *mut SDWORD = 0 as *mut SDWORD;
    let mut wTileY: *mut SDWORD = 0 as *mut SDWORD;
    if stackPopParams(9 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut wTileX as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut wTileY as *mut *mut SDWORD, VAL_INT as libc::c_int,
                      &mut pwLookerX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut pwLookerY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut wRangeX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut wRangeY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut wRange as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut threadRange as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrFogTileInRange: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if pwLookerX < 0 as libc::c_int ||
           pwLookerX > (mapWidth << 7 as libc::c_int) as SDWORD ||
           pwLookerY < 0 as libc::c_int ||
           pwLookerY > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrFogTileInRange: coords off map\x00" as *const u8 as
                  *const libc::c_char); //cache to tile coords, for faster calculations
        return 0 as libc::c_int
    } // change to tile coords.
    tRangeX = wRangeX >> 7 as libc::c_int;
    tRangeY = wRangeY >> 7 as libc::c_int;
    tx = (pwLookerX >> 7 as libc::c_int) as UDWORD;
    ty = (pwLookerY >> 7 as libc::c_int) as UDWORD;
    wBestDist = 99999 as libc::c_int as UDWORD;
    tBestX = -(1 as libc::c_int);
    tBestY = -(1 as libc::c_int);
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth {
        j = 0 as libc::c_int as UDWORD;
        while j < mapHeight {
            psTile = mapTile(i, j);
            if (*psTile).tileVisBits as libc::c_int &
                   (1 as libc::c_int) << player == 0 {
                //not vis
                //within base range
                if wRange <= 0 as libc::c_int ||
                       (dirtySqrt(tRangeX, tRangeY, i as SDWORD, j as SDWORD)
                            << 7 as libc::c_int) < wRange as libc::c_uint {
                    //dist in world units between baseX/baseY and the tile
                    //calc dist between this tile and looker
                    wDist =
                        dirtySqrt(tx as SDWORD, ty as SDWORD, i as SDWORD,
                                  j as SDWORD) << 7 as libc::c_int;
                    if wDist < wBestDist {
                        //tmpX = i;
						//tmpY = j;
						//if(pickATileGen(&tmpX, &tmpY, 4,zonedPAT))	//can reach (don't need many passes)
                        if zonedPAT(i, j) != 0 {
                            //Can reach this tile
                            //if((tmpX == i) && (tmpY == j))	//can't allow to change coords, otherwise might send to the same unrevealed tile next time
															//and units will stuck forever
							//{
                            if threadRange <= 0 as libc::c_int ||
                                   ThreatInRange(player, threadRange,
                                                 (i << 7 as libc::c_int) as
                                                     SDWORD,
                                                 (j << 7 as libc::c_int) as
                                                     SDWORD, 0 as libc::c_int)
                                       == 0 {
                                wBestDist = wDist;
                                tBestX = i as SDWORD;
                                tBestY = j as SDWORD;
                                ok = 1 as libc::c_int
                            }
                        }
                    }
                }
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    if ok != 0 {
        //closer than last one?
        //something found
        *wTileX = tBestX << 7 as libc::c_int;
        *wTileY = tBestY << 7 as libc::c_int;
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            debug(LOG_ERROR,
                  b"scrFogTileInRange: stackPushResult failed (found)\x00" as
                      *const u8 as *const libc::c_char);
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        debug(LOG_ERROR,
              b"scrFogTileInRange: stackPushResult failed (not found)\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrMapRevealedInRange() -> BOOL {
    let mut wRangeX: SDWORD = 0;
    let mut wRangeY: SDWORD = 0;
    let mut tRangeX: SDWORD = 0;
    let mut tRangeY: SDWORD = 0;
    let mut wRange: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut wRangeX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut wRangeY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut wRange as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrMapRevealedInRange: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if wRangeX < 0 as libc::c_int ||
           wRangeX > (mapWidth << 7 as libc::c_int) as SDWORD ||
           wRangeY < 0 as libc::c_int ||
           wRangeY > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrMapRevealedInRange: coords off map\x00" as *const u8 as
                  *const libc::c_char); //cache to tile coords, for faster calculations
        return 0 as libc::c_int
    }
    tRangeX = wRangeX >> 7 as libc::c_int;
    tRangeY = wRangeY >> 7 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < mapWidth {
        j = 0 as libc::c_int as UDWORD;
        while j < mapHeight {
            psTile = mapTile(i, j);
            if (*psTile).tileVisBits as libc::c_int &
                   (1 as libc::c_int) << player != 0 {
                //not vis
                //within range
                if (dirtySqrt(tRangeX, tRangeY, i as SDWORD, j as SDWORD) <<
                        7 as libc::c_int) < wRange as libc::c_uint {
                    //dist in world units between x/y and the tile
                    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                        return 0 as libc::c_int
                    }
                    return 1 as libc::c_int
                }
            }
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    //nothing found
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//return number of reserach topics that are left to be researched
//for a certain technology to become available
#[no_mangle]
pub unsafe extern "C" fn scrNumResearchLeft() -> BOOL {
    let mut psResearch: *mut RESEARCH =
        0 as *mut RESEARCH; //TODO: fix if needed
    let mut player: SDWORD = 0; //init, count the top research topic as 1
    let mut result: SDWORD = 0; //start with first index's PR
    let mut cur: UWORD = 0;
    let mut index: UWORD = 0;
    let mut tempIndex: UWORD = 0;
    let mut top: SWORD = 0;
    let mut Stack: [UWORD; 400] = [0; 400];
    let mut found: BOOL = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, ST_RESEARCH as libc::c_int,
                      &mut psResearch as *mut *mut RESEARCH) == 0 {
        debug(LOG_ERROR,
              b"scrNumResearchLeft(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if psResearch.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumResearchLeft(): no such research topic\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7177 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrNumResearchLeft\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    pPlayerRes = asPlayerResList[player as usize];
    index =
        psResearch.wrapping_offset_from(asResearch) as libc::c_int as UWORD;
    if index as libc::c_uint >= numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumResearchLeft(): invalid research index\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7186 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrNumResearchLeft\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    found = 0 as libc::c_int;
    if beingResearchedByAlly(index as SDWORD, player) != 0 {
        result = 1 as libc::c_int
    } else if (*pPlayerRes.offset(index as isize)).ResearchStatus as
                  libc::c_int & 0x4 as libc::c_int != 0 {
        result = 0 as libc::c_int
    } else if (*pPlayerRes.offset(index as isize)).ResearchStatus as
                  libc::c_int & 0x1 as libc::c_int != 0 {
        result = 1 as libc::c_int
    } else if (*pPlayerRes.offset(index as isize)).ResearchStatus as
                  libc::c_int & 0x80 as libc::c_int != 0 ||
                  (*pPlayerRes.offset(index as isize)).ResearchStatus as
                      libc::c_int & 0x2 as libc::c_int != 0 {
        result = 1 as libc::c_int
    } else if skTopicAvail(index, player as UDWORD) != 0 {
        result = 1 as libc::c_int
    } else {
        result = 1 as libc::c_int;
        top = -(1 as libc::c_int) as SWORD;
        cur = 0 as libc::c_int as UWORD;
        tempIndex = -(1 as libc::c_int) as UWORD;
        loop 
             //do
             {
            if cur as libc::c_int >=
                   (*asResearch.offset(index as isize)).numPRRequired as
                       libc::c_int {
                //this one has no PRs or end of PRs reached
                top = (top as libc::c_int - 2 as libc::c_int) as SWORD;
                if (top as libc::c_int) < -(1 as libc::c_int) {
                    break ;
                    //go to next PR of the last node
                    //end of stack
                } else {
                    index =
                        Stack[(top as libc::c_int + 2 as libc::c_int) as
                                  usize]; //if index = -1, then exit
                    cur =
                        Stack[(top as libc::c_int + 1 as libc::c_int) as
                                  usize]
                }
            } else {
                //end of PRs not reached
                result +=
                    (*asResearch.offset(index as isize)).numPRRequired as
                        libc::c_int; //add num of PRs this topic has
                tempIndex =
                    *(*asResearch.offset(index as
                                             isize)).pPRList.offset(cur as
                                                                        isize); //get cur node's index
                if (*pPlayerRes.offset(tempIndex as isize)).ResearchStatus as
                       libc::c_int & 0x4 as libc::c_int == 0 &&
                       skTopicAvail(index, player as UDWORD) == 0 &&
                       beingResearchedByAlly(index as SDWORD, player) == 0 {
                    //will become available soon anyway
                    if (*asResearch.offset(tempIndex as isize)).numPRRequired
                           as libc::c_int > 0 as libc::c_int {
                        //node has any nodes itself
                        Stack[(top as libc::c_int + 1 as libc::c_int) as
                                  usize] = cur; //so can go back to it further
                        Stack[(top as libc::c_int + 2 as libc::c_int) as
                                  usize] = index; //go 1 level further
                        top =
                            (top as libc::c_int + 2 as libc::c_int) as SWORD;
                        index = tempIndex;
                        cur = -(1 as libc::c_int) as UWORD
                    }
                }
            }
            //decide if has to check its PRs
            cur = cur.wrapping_add(1); //try next node of the main node
            if cur as libc::c_int >=
                   (*asResearch.offset(index as isize)).numPRRequired as
                       libc::c_int &&
                   top as libc::c_int <= -(1 as libc::c_int) {
                break ;
            }
        }
    }
    if stackPushResult(VAL_INT, result) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//check if any of the ally is researching this topic
#[no_mangle]
pub unsafe extern "C" fn beingResearchedByAlly(mut resIndex: SDWORD,
                                               mut player: SDWORD) -> BOOL {
    let mut psOtherStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut i: SDWORD = 0;
    let mut Stat: *mut BASE_STATS = 0 as *mut BASE_STATS;
    Stat = asResearch.offset(resIndex as isize) as *mut BASE_STATS;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if i != player && aiCheckAlliances(player as UDWORD, i as UDWORD) != 0
           {
            //check each research facility to see if they are doing this topic.
            psOtherStruct = apsStructLists[i as usize];
            while !psOtherStruct.is_null() {
                if (*(*psOtherStruct).pStructureType).type_0 ==
                       REF_RESEARCH as libc::c_int as libc::c_uint &&
                       (*psOtherStruct).status as libc::c_int ==
                           SS_BUILT as libc::c_int &&
                       !(*((*psOtherStruct).pFunctionality as
                               *mut RESEARCH_FACILITY)).psSubject.is_null() {
                    if (*(*((*psOtherStruct).pFunctionality as
                                *mut RESEARCH_FACILITY)).psSubject).ref_0 ==
                           (*Stat).ref_0 {
                        return 1 as libc::c_int
                    }
                }
                psOtherStruct = (*psOtherStruct).psNext
            }
        }
        i += 1
    }
    return 0 as libc::c_int;
}
// TRUE if player has completed this research
#[no_mangle]
pub unsafe extern "C" fn scrResearchCompleted() -> BOOL {
    let mut psResearch: *mut RESEARCH =
        0 as *mut RESEARCH; //TODO: fix if needed
    let mut player: SDWORD = 0;
    let mut index: UWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    if stackPopParams(2 as libc::c_int, ST_RESEARCH as libc::c_int,
                      &mut psResearch as *mut *mut RESEARCH,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrResearchCompleted(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if psResearch.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b": no such research topic\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7324 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrResearchCompleted\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    pPlayerRes = asPlayerResList[player as usize];
    index =
        psResearch.wrapping_offset_from(asResearch) as libc::c_int as UWORD;
    if index as libc::c_uint >= numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrResearchCompleted: invalid research index\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7333 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 21],
                                            &[libc::c_char; 21]>(b"scrResearchCompleted\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*pPlayerRes.offset(index as isize)).ResearchStatus as libc::c_int &
           0x4 as libc::c_int != 0 {
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
// TRUE if player has already started researching it
#[no_mangle]
pub unsafe extern "C" fn scrResearchStarted() -> BOOL {
    let mut psResearch: *mut RESEARCH =
        0 as *mut RESEARCH; //TODO: fix if needed
    let mut player: SDWORD = 0;
    let mut index: UWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH = 0 as *mut PLAYER_RESEARCH;
    if stackPopParams(2 as libc::c_int, ST_RESEARCH as libc::c_int,
                      &mut psResearch as *mut *mut RESEARCH,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrResearchStarted(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if psResearch.is_null() {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b": no such research topic\x00" as *const u8 as
                      *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7371 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrResearchStarted\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    pPlayerRes = asPlayerResList[player as usize];
    index =
        psResearch.wrapping_offset_from(asResearch) as libc::c_int as UWORD;
    if index as libc::c_uint >= numResearch {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrResearchCompleted: invalid research index\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7380 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrResearchStarted\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if (*pPlayerRes.offset(index as isize)).ResearchStatus as libc::c_int &
           0x1 as libc::c_int != 0 {
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//returns TRUE if location is dangerous
#[no_mangle]
pub unsafe extern "C" fn scrThreatInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut threat: BOOL = 0;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bVTOLs as *mut BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrThreatInRange(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    threat = ThreatInRange(player, range, rangeX, rangeY, bVTOLs);
    if stackPushResult(VAL_BOOL, threat) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumEnemyWeapObjInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut bVTOLs as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapObjInRange(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[lookingPlayer as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == lookingPlayer) {
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapDroidsInRange(i,
                                                                   lookingPlayer,
                                                                   range,
                                                                   rangeX,
                                                                   rangeY,
                                                                   bVTOLs));
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapStructsInRange(i,
                                                                    lookingPlayer,
                                                                    range,
                                                                    rangeX,
                                                                    rangeY))
        }
        //skip allies and myself
        i += 1
    }
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapObjInRange(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn numPlayerWeapDroidsInRange(mut player: SDWORD,
                                                    mut lookingPlayer: SDWORD,
                                                    mut range: SDWORD,
                                                    mut rangeX: SDWORD,
                                                    mut rangeY: SDWORD,
                                                    mut bVTOLs: BOOL)
 -> UDWORD {
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut numEnemies: UDWORD = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    tx = (rangeX >> 7 as libc::c_int) as UDWORD;
    ty = (rangeY >> 7 as libc::c_int) as UDWORD;
    numEnemies = 0 as libc::c_int as UDWORD;
    //check droids
    psDroid = apsDroidLists[player as usize];
    while !psDroid.is_null() {
        if (*psDroid).visible[lookingPlayer as usize] != 0 {
            //can see this droid?
            if !((*psDroid).droidType as libc::c_uint !=
                     DROID_WEAPON as libc::c_int as libc::c_uint &&
                     (*psDroid).droidType as libc::c_uint !=
                         DROID_PERSON as libc::c_int as libc::c_uint &&
                     (*psDroid).droidType as libc::c_uint !=
                         DROID_CYBORG as libc::c_int as libc::c_uint &&
                     (*psDroid).droidType as libc::c_uint !=
                         DROID_CYBORG_SUPER as libc::c_int as libc::c_uint) {
                //if VTOLs are excluded, skip them
                if !(bVTOLs == 0 &&
                         ((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                           as
                                                                           libc::c_int
                                                                           as
                                                                           usize].nStat
                                                         as
                                                         isize)).propulsionType
                              as libc::c_int == LIFT as libc::c_int ||
                              (*psDroid).droidType as libc::c_uint ==
                                  DROID_TRANSPORTER as libc::c_int as
                                      libc::c_uint)) {
                    if range < 0 as libc::c_int ||
                           (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                      (*psDroid).x as libc::c_int >>
                                          7 as libc::c_int,
                                      (*psDroid).y as libc::c_int >>
                                          7 as libc::c_int) <<
                                7 as libc::c_int) < range as libc::c_uint {
                        //enemy in range
                        numEnemies = numEnemies.wrapping_add(1)
                    }
                }
            }
        }
        psDroid = (*psDroid).psNext
    }
    return numEnemies;
}
#[no_mangle]
pub unsafe extern "C" fn numPlayerWeapStructsInRange(mut player: SDWORD,
                                                     mut lookingPlayer:
                                                         SDWORD,
                                                     mut range: SDWORD,
                                                     mut rangeX: SDWORD,
                                                     mut rangeY: SDWORD)
 -> UDWORD {
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut numEnemies: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    tx = (rangeX >> 7 as libc::c_int) as UDWORD;
    ty = (rangeY >> 7 as libc::c_int) as UDWORD;
    numEnemies = 0 as libc::c_int as UDWORD;
    //check structures
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*psStruct).visible[lookingPlayer as usize] != 0 {
            //if can see it
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_DEFENSE as libc::c_int as libc::c_uint {
                if range < 0 as libc::c_int ||
                       (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                  (*psStruct).x as libc::c_int >>
                                      7 as libc::c_int,
                                  (*psStruct).y as libc::c_int >>
                                      7 as libc::c_int) << 7 as libc::c_int) <
                           range as libc::c_uint {
                    //enemy in range
                    numEnemies = numEnemies.wrapping_add(1)
                }
            }
        }
        psStruct = (*psStruct).psNext
    }
    return numEnemies;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumEnemyWeapDroidsInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut bVTOLs as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapDroidsInRange(): stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[lookingPlayer as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == lookingPlayer) {
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapDroidsInRange(i,
                                                                   lookingPlayer,
                                                                   range,
                                                                   rangeX,
                                                                   rangeY,
                                                                   bVTOLs))
        }
        //skip allies and myself
        i += 1
    }
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapDroidsInRange(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumEnemyWeapStructsInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapStructsInRange(): stack failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[lookingPlayer as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == lookingPlayer) {
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapStructsInRange(i,
                                                                    lookingPlayer,
                                                                    range,
                                                                    rangeX,
                                                                    rangeY))
        }
        //skip allies and myself
        i += 1
    }
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyWeapStructsInRange(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumFriendlyWeapObjInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numFriends: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bVTOLs as *mut BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrNumFriendlyWeapObjInRange(): stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if alliances[player as usize][i as usize] as libc::c_int ==
               3 as libc::c_int || i == player {
            //skip enemies
            numFriends =
                numFriends.wrapping_add(numPlayerWeapDroidsInRange(i, player,
                                                                   range,
                                                                   rangeX,
                                                                   rangeY,
                                                                   bVTOLs));
            numFriends =
                numFriends.wrapping_add(numPlayerWeapStructsInRange(i, player,
                                                                    range,
                                                                    rangeX,
                                                                    rangeY))
        }
        i += 1
    }
    if stackPushResult(VAL_INT, numFriends as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumFriendlyWeapDroidsInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut bVTOLs as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrNumFriendlyWeapDroidsInRange(): stack failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if alliances[lookingPlayer as usize][i as usize] as libc::c_int ==
               3 as libc::c_int || i == lookingPlayer {
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapDroidsInRange(i,
                                                                   lookingPlayer,
                                                                   range,
                                                                   rangeX,
                                                                   rangeY,
                                                                   bVTOLs))
        }
        i += 1
    }
    //numEnemies = numEnemyWeapObjInRange(player, range, rangeX, rangeY, bVTOLs);
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumFriendlyWeapDroidsInRange(): failed to push result\x00"
                  as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumFriendlyWeapStructsInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int) == 0 {
        debug(LOG_ERROR,
              b"scrNumFriendlyWeapStructsInRange(): stack failed\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if alliances[lookingPlayer as usize][i as usize] as libc::c_int ==
               3 as libc::c_int || i == lookingPlayer {
            //skip enemies
            numEnemies =
                numEnemies.wrapping_add(numPlayerWeapStructsInRange(i,
                                                                    lookingPlayer,
                                                                    range,
                                                                    rangeX,
                                                                    rangeY))
        }
        i += 1
    }
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumFriendlyWeapStructsInRange(): failed to push result\x00"
                  as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumPlayerWeapObjInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD,
                      VAL_BOOL as libc::c_int, &mut bVTOLs as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrNumPlayerWeapObjInRange(): stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    numEnemies =
        numEnemies.wrapping_add(numPlayerWeapDroidsInRange(player,
                                                           lookingPlayer,
                                                           range, rangeX,
                                                           rangeY, bVTOLs));
    numEnemies =
        numEnemies.wrapping_add(numPlayerWeapStructsInRange(player,
                                                            lookingPlayer,
                                                            range, rangeX,
                                                            rangeY));
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumPlayerWeapObjInRange(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumEnemyObjInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut numEnemies: UDWORD = 0 as libc::c_int as UDWORD;
    let mut bVTOLs: BOOL = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut rangeY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bVTOLs as *mut BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyObjInRange(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    numEnemies = numEnemyObjInRange(player, range, rangeX, rangeY, bVTOLs);
    if stackPushResult(VAL_INT, numEnemies as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumEnemyObjInRange(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn numEnemyObjInRange(mut player: SDWORD,
                                            mut range: SDWORD,
                                            mut rangeX: SDWORD,
                                            mut rangeY: SDWORD,
                                            mut bVTOLs: BOOL) -> UDWORD {
    let mut i: UDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    let mut numEnemies: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    tx = (rangeX >> 7 as libc::c_int) as UDWORD;
    ty = (rangeY >> 7 as libc::c_int) as UDWORD;
    numEnemies = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        if !(alliances[player as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == player as libc::c_uint) {
            //check structures
            psStruct = apsStructLists[i as usize];
            while !psStruct.is_null() {
                if (*psStruct).visible[player as usize] != 0 {
                    //if can see it
                    //if(psStruct->pStructureType->type == REF_DEFENSE)
				//{
                    if range < 0 as libc::c_int ||
                           (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                      (*psStruct).x as libc::c_int >>
                                          7 as libc::c_int,
                                      (*psStruct).y as libc::c_int >>
                                          7 as libc::c_int) <<
                                7 as libc::c_int) < range as libc::c_uint {
                        //enemy in range
                        numEnemies = numEnemies.wrapping_add(1)
                    }
                }
                psStruct = (*psStruct).psNext
            }
            //check droids
            psDroid = apsDroidLists[i as usize];
            while !psDroid.is_null() {
                if (*psDroid).visible[player as usize] != 0 {
                    //can see this droid?
                    //if VTOLs are excluded, skip them
                    if !(bVTOLs == 0 &&
                             ((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               usize].nStat
                                                             as
                                                             isize)).propulsionType
                                  as libc::c_int == LIFT as libc::c_int ||
                                  (*psDroid).droidType as libc::c_uint ==
                                      DROID_TRANSPORTER as libc::c_int as
                                          libc::c_uint)) {
                        if range < 0 as libc::c_int ||
                               (dirtySqrt(tx as SDWORD, ty as SDWORD,
                                          (*psDroid).x as libc::c_int >>
                                              7 as libc::c_int,
                                          (*psDroid).y as libc::c_int >>
                                              7 as libc::c_int) <<
                                    7 as libc::c_int) < range as libc::c_uint
                           {
                            //enemy in range
                            numEnemies = numEnemies.wrapping_add(1)
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        i = i.wrapping_add(1)
    }
    return numEnemies;
}
/* Similiar to structureBuiltInRange(), but also returns true if structure is not finished */
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsByStatInRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut NumStruct: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut psTarget: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    if stackPopParams(6 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumStructsByStatInRange(): stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7810 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByStatInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >> 7 as libc::c_int > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : invalid X coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7816 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByStatInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y < 0 as libc::c_int || y >> 7 as libc::c_int > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : invalid Y coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7821 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByStatInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if index < 0 as libc::c_int || index > numStructureStats as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : Invalid structure stat\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7826 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByStatInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if range < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : Rnage is less than zero\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7831 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByStatInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    NumStruct = 0 as libc::c_int;
    //now look through the players list of structures to see if this type
	//exists within range
    psTarget =
        &mut *asStructureStats.offset(index as isize) as *mut STRUCTURE_STATS;
    rangeSquared = range * range;
    psCurr = apsStructLists[player as usize];
    while !psCurr.is_null() {
        xdiff = (*psCurr).x as SDWORD - x;
        ydiff = (*psCurr).y as SDWORD - y;
        if xdiff * xdiff + ydiff * ydiff <= rangeSquared {
            if strcmp((*(*psCurr).pStructureType).pName, (*psTarget).pName) ==
                   0 as libc::c_int {
                if (*psCurr).visible[lookingPlayer as usize] != 0 {
                    //can we see it?
                    NumStruct += 1
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_INT, NumStruct) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsByStatInArea() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut x1: SDWORD = 0;
    let mut y1: SDWORD = 0;
    let mut x2: SDWORD = 0;
    let mut y2: SDWORD = 0;
    let mut NumStruct: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    if stackPopParams(7 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut x2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y2 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumStructsByStatInArea: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrNumStructsByStatInArea: player number too high\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7888 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrNumStructsByStatInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if index < 0 as libc::c_int || index > numStructureStats as SDWORD {
        debug(LOG_ERROR,
              b"scrNumStructsByStatInArea: invalid structure stat\x00" as
                  *const u8 as *const libc::c_char);
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrStructureBuiltInRange : Invalid structure stat\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7896 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"scrNumStructsByStatInArea\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    psStats = asStructureStats.offset(index as isize);
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"scrNumStructsByStatInArea: Invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
              7903 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"scrNumStructsByStatInArea\x00")).as_ptr(),
              b"PTRVALID(psStats, sizeof(STRUCTURE_STATS))\x00" as *const u8
                  as *const libc::c_char);
    };
    NumStruct = 0 as libc::c_int;
    psCurr = apsStructLists[player as usize];
    while !psCurr.is_null() {
        if (*psCurr).pStructureType == psStats {
            if (*psCurr).visible[lookingPlayer as usize] != 0 {
                //can we see it?
                if !(((*psCurr).x as libc::c_int) < x1) { //not in bounds
                    if !(((*psCurr).y as libc::c_int) < y1) { //not in bounds
                        if !((*psCurr).x as libc::c_int > x2)
                           { //not in bounds
                            if !((*psCurr).y as libc::c_int > y2) {
                                NumStruct += 1
                            }
                        }
                    }
                }
            }
        } //not in bounds
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_INT, NumStruct) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsByTypeInRange() -> BOOL {
    let mut targetPlayer: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut NumStruct: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int,
                      &mut targetPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut type_0 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumStructsByTypeInRange: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if lookingPlayer >= 8 as libc::c_int || targetPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsByTypeInRange:player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7947 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >> 7 as libc::c_int > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsByTypeInRange : invalid X coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7953 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y < 0 as libc::c_int || y >> 7 as libc::c_int > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsByTypeInRange : invalid Y coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7959 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if range < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsByTypeInRange : Rnage is less than zero\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  7965 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 27],
                                            &[libc::c_char; 27]>(b"scrNumStructsByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    NumStruct = 0 as libc::c_int;
    //now look through the players list of structures to see if this type
	//exists within range
    rangeSquared = range * range;
    psCurr = apsStructLists[targetPlayer as usize];
    while !psCurr.is_null() {
        xdiff = (*psCurr).x as SDWORD - x;
        ydiff = (*psCurr).y as SDWORD - y;
        if xdiff * xdiff + ydiff * ydiff <= rangeSquared {
            if type_0 < 0 as libc::c_int ||
                   (*(*psCurr).pStructureType).type_0 ==
                       type_0 as libc::c_uint {
                if (*psCurr).visible[lookingPlayer as usize] != 0 {
                    //can we see it?
                    NumStruct += 1
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_INT, NumStruct) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumFeatByTypeInRange() -> BOOL {
    let mut lookingPlayer: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut NumFeat: SDWORD = 0;
    let mut psCurr: *mut FEATURE = 0 as *mut FEATURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut type_0 as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumFeatByTypeInRange(): failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if lookingPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumFeatByTypeInRange:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8014 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrNumFeatByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >> 7 as libc::c_int > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumFeatByTypeInRange : invalid X coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8020 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrNumFeatByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y < 0 as libc::c_int || y >> 7 as libc::c_int > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumFeatByTypeInRange : invalid Y coord\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8026 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrNumFeatByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if range < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumFeatByTypeInRange : Rnage is less than zero\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8032 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 24],
                                            &[libc::c_char; 24]>(b"scrNumFeatByTypeInRange\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    NumFeat = 0 as libc::c_int;
    //now look through the players list of structures to see if this type
	//exists within range
    rangeSquared = range * range;
    psCurr = apsFeatureLists[0 as libc::c_int as usize];
    while !psCurr.is_null() {
        xdiff = (*psCurr).x as SDWORD - x;
        ydiff = (*psCurr).y as SDWORD - y;
        if xdiff * xdiff + ydiff * ydiff <= rangeSquared {
            if type_0 < 0 as libc::c_int ||
                   (*(*psCurr).psStats).subType as libc::c_uint ==
                       type_0 as libc::c_uint {
                //like FEAT_OIL_RESOURCE
                if (*psCurr).visible[lookingPlayer as usize] != 0 {
                    //can we see it?
                    NumFeat += 1
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_INT, NumFeat) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//returns num of visible structures of a certain player in range (only visible ones)
#[no_mangle]
pub unsafe extern "C" fn scrNumStructsButNotWallsInRangeVis() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeSquared: SDWORD = 0;
    let mut NumStruct: SDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut player as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumStructsButNotWallsInRangeVis: failed to pop\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int || lookingPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsButNotWallsInRangeVis:player number is too high\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8082 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 35],
                                            &[libc::c_char; 35]>(b"scrNumStructsButNotWallsInRangeVis\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >> 7 as libc::c_int > mapWidth as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsButNotWallsInRangeVis : invalid X coord\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8088 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 35],
                                            &[libc::c_char; 35]>(b"scrNumStructsButNotWallsInRangeVis\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if y < 0 as libc::c_int || y >> 7 as libc::c_int > mapHeight as SDWORD {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsButNotWallsInRangeVis : invalid Y coord\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8093 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 35],
                                            &[libc::c_char; 35]>(b"scrNumStructsButNotWallsInRangeVis\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    if range < 0 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrNumStructsButNotWallsInRangeVis : Rnage is less than zero\x00"
                      as *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8098 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 35],
                                            &[libc::c_char; 35]>(b"scrNumStructsButNotWallsInRangeVis\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    NumStruct = 0 as libc::c_int;
    //now look through the players list of structures
    rangeSquared = range * range;
    psCurr = apsStructLists[player as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 !=
               REF_WALL as libc::c_int as libc::c_uint &&
               (*(*psCurr).pStructureType).type_0 !=
                   REF_WALLCORNER as libc::c_int as libc::c_uint {
            if (*psCurr).visible[lookingPlayer as usize] != 0 {
                //can we see it?
                xdiff = (*psCurr).x as SDWORD - x;
                ydiff = (*psCurr).y as SDWORD - y;
                if xdiff * xdiff + ydiff * ydiff <= rangeSquared {
                    NumStruct += 1
                }
            }
        }
        psCurr = (*psCurr).psNext
    }
    if stackPushResult(VAL_INT, NumStruct) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
// Only returns structure if it is visible
#[no_mangle]
pub unsafe extern "C" fn scrGetStructureVis() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut index: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut structType: UDWORD = 0;
    let mut found: BOOL = 0;
    if stackPopParams(3 as libc::c_int, ST_STRUCTURESTAT as libc::c_int,
                      &mut index as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetStructureVis: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int || lookingPlayer >= 8 as libc::c_int {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"scrGetStructureVis:player number is too high\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"scriptfuncs.c\x00" as *const u8 as *const libc::c_char,
                  8147 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 19],
                                            &[libc::c_char; 19]>(b"scrGetStructureVis\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return 0 as libc::c_int
    }
    structType = (*asStructureStats.offset(index as isize)).ref_0;
    //search the players' list of built structures to see if one exists
    found = 0 as libc::c_int;
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).ref_0 == structType {
            if (*psStruct).visible[lookingPlayer as usize] != 0 {
                found = 1 as libc::c_int;
                break ;
            }
        }
        psStruct = (*psStruct).psNext
    }
    //make sure pass NULL back if not got one
    if found == 0 { psStruct = 0 as *mut STRUCTURE }
    if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                       psStruct as UDWORD as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//returns num of visible structures of a certain player in range
#[no_mangle]
pub unsafe extern "C" fn scrChooseValidLoc() -> BOOL {
    let mut sendY: SDWORD = 0;
    let mut sendX: SDWORD = 0;
    let mut x: *mut SDWORD = 0 as *mut SDWORD;
    let mut y: *mut SDWORD = 0 as *mut SDWORD;
    let mut player: SDWORD = 0;
    let mut threatRange: SDWORD = 0;
    let mut tx: UDWORD = 0;
    let mut ty: UDWORD = 0;
    if stackPopParams(6 as libc::c_int,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut x as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut y as *mut *mut SDWORD, VAL_INT as libc::c_int,
                      &mut sendX as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut sendY as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut threatRange as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrChooseValidLoc: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if sendX < 0 as libc::c_int ||
           sendX > (mapWidth << 7 as libc::c_int) as SDWORD ||
           sendY < 0 as libc::c_int ||
           sendY > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrChooseValidLoc: coords off map\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    tx = (sendX >> 7 as libc::c_int) as UDWORD;
    ty = (sendY >> 7 as libc::c_int) as UDWORD;
    if pickATileGenThreat(&mut tx, &mut ty, 20 as libc::c_int as UBYTE,
                          threatRange, player,
                          Some(zonedPAT as
                                   unsafe extern "C" fn(_: UDWORD, _: UDWORD)
                                       -> BOOL)) != 0 {
        *x = (tx << 7 as libc::c_int) as SDWORD;
        *y = (ty << 7 as libc::c_int) as SDWORD;
        if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//returns closest enemy object
#[no_mangle]
pub unsafe extern "C" fn scrGetClosestEnemy() -> BOOL {
    let mut x: SDWORD = 0; //only military objects?
    let mut y: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut bestDist: UDWORD = 0;
    let mut weaponOnly: BOOL = 0;
    let mut bVTOLs: BOOL = 0;
    let mut bFound: BOOL = 0 as libc::c_int;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut weaponOnly as *mut BOOL, VAL_BOOL as libc::c_int,
                      &mut bVTOLs as *mut BOOL, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetClosestEnemy: stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if x < 0 as libc::c_int || x > (mapWidth << 7 as libc::c_int) as SDWORD ||
           y < 0 as libc::c_int ||
           y > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrGetClosestEnemy: coords off map\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    tx = x >> 7 as libc::c_int;
    ty = y >> 7 as libc::c_int;
    bestDist = 99999 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[player as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == player) {
            //check droids
            psDroid = apsDroidLists[i as usize];
            while !psDroid.is_null() {
                if (*psDroid).visible[player as usize] != 0 {
                    //can see this droid?
                    //if only weapon droids and don't have it, then skip
                    if !(weaponOnly != 0 &&
                             ((*psDroid).droidType as libc::c_uint !=
                                  DROID_WEAPON as libc::c_int as libc::c_uint
                                  &&
                                  (*psDroid).droidType as libc::c_uint !=
                                      DROID_PERSON as libc::c_int as
                                          libc::c_uint &&
                                  (*psDroid).droidType as libc::c_uint !=
                                      DROID_CYBORG as libc::c_int as
                                          libc::c_uint &&
                                  (*psDroid).droidType as libc::c_uint !=
                                      DROID_CYBORG_SUPER as libc::c_int as
                                          libc::c_uint)) {
                        //if VTOLs are excluded, skip them
                        if !(bVTOLs == 0 &&
                                 ((*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   usize].nStat
                                                                 as
                                                                 isize)).propulsionType
                                      as libc::c_int == LIFT as libc::c_int ||
                                      (*psDroid).droidType as libc::c_uint ==
                                          DROID_TRANSPORTER as libc::c_int as
                                              libc::c_uint)) {
                            dist =
                                dirtySqrt(tx, ty,
                                          (*psDroid).x as libc::c_int >>
                                              7 as libc::c_int,
                                          (*psDroid).y as libc::c_int >>
                                              7 as libc::c_int) <<
                                    7 as libc::c_int;
                            if dist < bestDist {
                                if range < 0 as libc::c_int ||
                                       dist < range as libc::c_uint {
                                    //enemy in range
                                    bestDist = dist;
                                    bFound = 1 as libc::c_int;
                                    psObj = psDroid as *mut BASE_OBJECT
                                }
                            }
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
            //check structures
            psStruct = apsStructLists[i as usize];
            while !psStruct.is_null() {
                if (*psStruct).visible[player as usize] != 0 {
                    //if can see it
                    //only need defenses?
                    if !(weaponOnly != 0 &&
                             ((*(*psStruct).pStructureType).type_0 !=
                                  REF_DEFENSE as libc::c_int as libc::c_uint
                                  ||
                                  (*psStruct).status as libc::c_int !=
                                      SS_BUILT as libc::c_int)) {
                        dist =
                            dirtySqrt(tx, ty,
                                      (*psStruct).x as libc::c_int >>
                                          7 as libc::c_int,
                                      (*psStruct).y as libc::c_int >>
                                          7 as libc::c_int) <<
                                7 as libc::c_int;
                        if dist < bestDist {
                            if range < 0 as libc::c_int ||
                                   dist < range as libc::c_uint {
                                //in range
                                bestDist = dist;
                                bFound = 1 as libc::c_int;
                                psObj = psStruct as *mut BASE_OBJECT
                            }
                        }
                    }
                }
                //non-weapon-structures	or not finished
                psStruct = (*psStruct).psNext
            }
        }
        i += 1
    }
    if bFound != 0 {
        if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                           psObj as SDWORD) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(ST_BASEOBJECT as libc::c_int as INTERP_TYPE,
                              0 as *mut libc::c_void as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//How many droids can it still fit?
#[no_mangle]
pub unsafe extern "C" fn scrTransporterCapacity() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        debug(LOG_ERROR,
              b"scrTransporterCapacity(): failed to pop params\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if psDroid.is_null() {
        debug(LOG_ERROR,
              b"scrTransporterCapacity(): NULLOBJECT passed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"scrTransporterCapacity(): passed droid is not a transporter\x00"
                  as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT, calcRemainingCapacity(psDroid) as SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrHasIndirectWeapon(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//is it?
#[no_mangle]
pub unsafe extern "C" fn scrTransporterFlying() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        debug(LOG_ERROR,
              b"scrTransporterFlying(): failed to pop params\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if psDroid.is_null() {
        debug(LOG_ERROR,
              b"scrTransporterFlying(): NULLOBJECT passed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"scrTransporterFlying(): passed droid is not a transporter\x00"
                  as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, transporterFlying(psDroid)) == 0 {
        debug(LOG_ERROR,
              b"scrTransporterFlying(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrUnloadTransporter() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(3 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrUnloadTransporter(): failed to pop params\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if psDroid.is_null() {
        debug(LOG_ERROR,
              b"scrUnloadTransporter(): NULLOBJECT passed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if (*psDroid).droidType as libc::c_uint !=
           DROID_TRANSPORTER as libc::c_int as libc::c_uint {
        debug(LOG_ERROR,
              b"scrUnloadTransporter(): passed droid is not a transporter\x00"
                  as *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    unloadTransporter(psDroid, x as UDWORD, y as UDWORD, 0 as libc::c_int);
    return 1 as libc::c_int;
}
//return true if droid is a member of any group
#[no_mangle]
pub unsafe extern "C" fn scrHasGroup() -> BOOL {
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut retval: BOOL = 0;
    if stackPopParams(1 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID) == 0 {
        debug(LOG_ERROR,
              b"scrHasGroup: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if psDroid.is_null() {
        debug(LOG_ERROR,
              b"scrHasGroup: droid is NULLOBJECT\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if !(*psDroid).psGroup.is_null() {
        retval = 1 as libc::c_int
    } else { retval = 0 as libc::c_int }
    if stackPushResult(VAL_BOOL, retval) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrObjWeaponMaxRange() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        debug(LOG_ERROR,
              b"scrObjWeaponMaxRange: stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //check if valid type
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psObj as *mut DROID;
        if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat !=
               0 as libc::c_int as libc::c_uint {
            psStats =
                asWeaponStats.offset((*psDroid).asWeaps[0 as libc::c_int as
                                                            usize].nStat as
                                         isize);
            if stackPushResult(VAL_INT, (*psStats).longRange as SDWORD) == 0 {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        psStruct = psObj as *mut STRUCTURE;
        if (*psStruct).asWeaps[0 as libc::c_int as usize].nStat !=
               0 as libc::c_int as libc::c_uint {
            psStats =
                asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int as
                                                             usize].nStat as
                                         isize);
            if stackPushResult(VAL_INT, (*psStats).longRange as SDWORD) == 0 {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    }
    if stackPushResult(VAL_INT, -(1 as libc::c_int)) == 0 {
        debug(LOG_ERROR,
              b"scrObjWeaponMaxRange: wrong object type\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrObjHasWeapon() -> BOOL {
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        debug(LOG_ERROR,
              b"scrObjHasWeapon: stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    //check if valid type
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psObj as *mut DROID;
        if (*psDroid).asWeaps[0 as libc::c_int as usize].nStat !=
               0 as libc::c_int as libc::c_uint {
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        psStruct = psObj as *mut STRUCTURE;
        if (*psStruct).asWeaps[0 as libc::c_int as usize].nStat !=
               0 as libc::c_int as libc::c_uint {
            if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int
            }
            return 1 as libc::c_int
        }
    }
    if stackPushResult(VAL_BOOL, 0 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrObjectHasIndirectWeapon() -> BOOL {
    let mut psWeapStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut bIndirect: BOOL = 0;
    let mut psObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    if stackPopParams(1 as libc::c_int, ST_BASEOBJECT as libc::c_int,
                      &mut psObj as *mut *mut BASE_OBJECT) == 0 {
        debug(LOG_ERROR,
              b"scrHasIndirectWeapon(): failed to pop params\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if psObj.is_null() {
        debug(LOG_ERROR,
              b"scrHasIndirectWeapon(): NULLOBJECT passed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    bIndirect = 0 as libc::c_int;
    if (*psObj).type_0 as libc::c_uint ==
           OBJ_DROID as libc::c_int as libc::c_uint {
        if (*(psObj as *mut DROID)).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
            psWeapStats =
                asWeaponStats.offset((*(psObj as
                                            *mut DROID)).asWeaps[0 as
                                                                     libc::c_int
                                                                     as
                                                                     usize].nStat
                                         as isize);
            bIndirect = (proj_Direct(psWeapStats) == 0) as libc::c_int
        }
    } else if (*psObj).type_0 as libc::c_uint ==
                  OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        if (*(psObj as
                  *mut STRUCTURE)).asWeaps[0 as libc::c_int as usize].nStat >
               0 as libc::c_int as libc::c_uint {
            psWeapStats =
                asWeaponStats.offset((*(psObj as
                                            *mut STRUCTURE)).asWeaps[0 as
                                                                         libc::c_int
                                                                         as
                                                                         usize].nStat
                                         as isize);
            bIndirect = (proj_Direct(psWeapStats) == 0) as libc::c_int
        }
    }
    if stackPushResult(VAL_BOOL, bIndirect) == 0 {
        debug(LOG_ERROR,
              b"scrHasIndirectWeapon(): failed to push result\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//returns closest droid by type
#[no_mangle]
pub unsafe extern "C" fn scrGetClosestEnemyDroidByType() -> BOOL {
    let mut x: SDWORD = 0; //only military objects?
    let mut y: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut bestDist: UDWORD = 0;
    let mut bFound: BOOL = 0 as libc::c_int;
    let mut bVTOLs: BOOL = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut foundDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(6 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut type_0 as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut bVTOLs as *mut BOOL, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetClosestEnemyDroidByType: stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if x < 0 as libc::c_int || x > (mapWidth << 7 as libc::c_int) as SDWORD ||
           y < 0 as libc::c_int ||
           y > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrGetClosestEnemyDroidByType: coords off map\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    tx = x >> 7 as libc::c_int;
    ty = y >> 7 as libc::c_int;
    bestDist = 99999 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[player as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == player) {
            //check droids
            psDroid = apsDroidLists[i as usize];
            while !psDroid.is_null() {
                //if VTOLs are excluded, skip them (don't check for transporter this time)
                if !(bVTOLs == 0 &&
                         (*asPropulsionStats.offset((*psDroid).asBits[COMP_PROPULSION
                                                                          as
                                                                          libc::c_int
                                                                          as
                                                                          usize].nStat
                                                        as
                                                        isize)).propulsionType
                             as libc::c_int == LIFT as libc::c_int) {
                    if (*psDroid).visible[player as usize] != 0 {
                        //can see this droid?
                        //skip?
                        if !(type_0 != -(1 as libc::c_int) &&
                                 (*psDroid).droidType as libc::c_uint !=
                                     type_0 as libc::c_uint) {
                            dist =
                                dirtySqrt(tx, ty,
                                          (*psDroid).x as libc::c_int >>
                                              7 as libc::c_int,
                                          (*psDroid).y as libc::c_int >>
                                              7 as libc::c_int) <<
                                    7 as libc::c_int;
                            if dist < bestDist {
                                if dist < range as libc::c_uint {
                                    //enemy in range
                                    bestDist = dist;
                                    bFound = 1 as libc::c_int;
                                    foundDroid = psDroid
                                }
                            }
                        }
                    }
                }
                psDroid = (*psDroid).psNext
            }
        }
        i += 1
    }
    if bFound != 0 {
        if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                           foundDroid as SDWORD) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(ST_DROID as libc::c_int as INTERP_TYPE,
                              0 as *mut libc::c_void as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//returns closest structure by type
#[no_mangle]
pub unsafe extern "C" fn scrGetClosestEnemyStructByType() -> BOOL {
    let mut x: SDWORD = 0; //only military objects?
    let mut y: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut player: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut i: SDWORD = 0;
    let mut type_0: SDWORD = 0;
    let mut dist: SDWORD = 0;
    let mut bestDist: UDWORD = 0;
    let mut bFound: BOOL = 0 as libc::c_int;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut foundStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut x as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut y as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut range as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut type_0 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrGetClosestEnemyStructByType: stack failed\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    //Check coords
    if x < 0 as libc::c_int || x > (mapWidth << 7 as libc::c_int) as SDWORD ||
           y < 0 as libc::c_int ||
           y > (mapHeight << 7 as libc::c_int) as SDWORD {
        debug(LOG_ERROR,
              b"scrGetClosestEnemyStructByType: coords off map\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    tx = x >> 7 as libc::c_int;
    ty = y >> 7 as libc::c_int;
    bestDist = 99999 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if !(alliances[player as usize][i as usize] as libc::c_int ==
                 3 as libc::c_int || i == player) {
            //check structures
            psStruct = apsStructLists[i as usize];
            while !psStruct.is_null() {
                if (*psStruct).visible[player as usize] != 0 {
                    //if can see it
                    //only need defenses?
                    if !(type_0 != -(1 as libc::c_int) &&
                             (*(*psStruct).pStructureType).type_0 !=
                                 type_0 as libc::c_uint) {
                        dist =
                            (dirtySqrt(tx, ty,
                                       (*psStruct).x as libc::c_int >>
                                           7 as libc::c_int,
                                       (*psStruct).y as libc::c_int >>
                                           7 as libc::c_int) <<
                                 7 as libc::c_int) as SDWORD;
                        if (dist as libc::c_uint) < bestDist {
                            if range < 0 as libc::c_int || dist < range {
                                //in range or no range check
                                bestDist = dist as UDWORD;
                                bFound = 1 as libc::c_int;
                                foundStruct = psStruct
                            }
                        }
                    }
                }
                //non-weapon-structures
                psStruct = (*psStruct).psNext
            }
        }
        i += 1
    }
    if bFound != 0 {
        if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                           foundStruct as SDWORD) == 0 {
            return 0 as libc::c_int
        }
    } else if stackPushResult(ST_STRUCTURE as libc::c_int as INTERP_TYPE,
                              0 as *mut libc::c_void as SDWORD) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//Approx point of intersection of a circle and a line with start loc being circle's center point
#[no_mangle]
pub unsafe extern "C" fn scrCirclePerimPoint() -> BOOL {
    let mut basex: SDWORD = 0; //x len (signed!)
    let mut basey: SDWORD = 0; //len
    let mut grx: *mut SDWORD =
        0 as *mut SDWORD; //by what factor is dist > radius?
    let mut gry: *mut SDWORD = 0 as *mut SDWORD;
    let mut radius: SDWORD = 0;
    let mut dist: UDWORD = 0;
    let mut factor: libc::c_float = 0.;
    let mut tempx: libc::c_float = 0.;
    let mut tempy: libc::c_float = 0.;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut basex as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut basey as *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut grx as *mut *mut SDWORD,
                      0x100000 as libc::c_int | VAL_INT as libc::c_int,
                      &mut gry as *mut *mut SDWORD, VAL_INT as libc::c_int,
                      &mut radius as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrCirclePerimPoint(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if radius == 0 as libc::c_int {
        debug(LOG_ERROR,
              b"scrCirclePerimPoint: radius == 0.\x00" as *const u8 as
                  *const libc::c_char);
        return 1 as libc::c_int
    }
    tempx = (*grx - basex) as libc::c_float;
    tempy = (*gry - basey) as libc::c_float;
    dist = dirtySqrt(basex, basey, *grx, *gry);
    factor = dist as libc::c_float / radius as libc::c_float;
    //if point was inside of the circle, don't modify passed parameter
    if factor == 0 as libc::c_int as libc::c_float {
        printf_console(b"scrCirclePerimPoint: division by zero.\x00" as
                           *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    //calc new len
    tempx = tempx / factor;
    tempy = tempy / factor;
    //now add new len to the center coords
    *grx = basex + tempx as SDWORD;
    *gry = basey + tempy as SDWORD;
    return 1 as libc::c_int;
}
//send my vision to AI
#[no_mangle]
pub unsafe extern "C" fn scrGiftRadar() -> BOOL {
    let mut playerFrom: SDWORD = 0;
    let mut playerTo: SDWORD = 0;
    let mut playMsg: BOOL = 0;
    if stackPopParams(3 as libc::c_int, VAL_INT as libc::c_int,
                      &mut playerFrom as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut playerTo as *mut SDWORD, VAL_BOOL as libc::c_int,
                      &mut playMsg as *mut BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrGiftRadar(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if playerFrom >= 8 as libc::c_int || playerTo >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrGiftRadar: player out of range\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    giftRadar(playerFrom as UDWORD, playerTo as UDWORD, 1 as libc::c_int);
    if playMsg != 0 { audio_QueueTrack(ID_SENSOR_DOWNLOAD as libc::c_int); }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrNumAllies() -> BOOL {
    let mut player: SDWORD = 0;
    let mut numAllies: SDWORD = 0;
    let mut i: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumAllies: failed to pop\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"scrNumAllies: player < 0\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if player >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrNumAllies: player index too high\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    numAllies = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if i != player {
            if alliances[i as usize][player as usize] as libc::c_int ==
                   3 as libc::c_int {
                numAllies += 1
            }
        }
        i += 1
    }
    if stackPushResult(VAL_INT, numAllies) == 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
//num aa defenses in range
#[no_mangle]
pub unsafe extern "C" fn scrNumAAinRange() -> BOOL {
    let mut player: SDWORD = 0;
    let mut lookingPlayer: SDWORD = 0;
    let mut range: SDWORD = 0;
    let mut rangeX: SDWORD = 0;
    let mut rangeY: SDWORD = 0;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut numFound: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if stackPopParams(5 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut lookingPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeX as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut rangeY as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut range as *mut SDWORD) == 0
       {
        debug(LOG_ERROR,
              b"scrNumAAinRange(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    tx = rangeX >> 7 as libc::c_int;
    ty = rangeY >> 7 as libc::c_int;
    numFound = 0 as libc::c_int as UDWORD;
    //check structures
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        if (*psStruct).visible[lookingPlayer as usize] != 0 {
            //if can see it
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_DEFENSE as libc::c_int as libc::c_uint &&
                   (*asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int
                                                                  as
                                                                  usize].nStat
                                              as isize)).surfaceToAir as
                       libc::c_int == 0x2 as libc::c_int {
                if range < 0 as libc::c_int ||
                       (dirtySqrt(tx, ty,
                                  (*psStruct).x as libc::c_int >>
                                      7 as libc::c_int,
                                  (*psStruct).y as libc::c_int >>
                                      7 as libc::c_int) << 7 as libc::c_int) <
                           range as libc::c_uint {
                    //enemy in range
                    numFound = numFound.wrapping_add(1)
                }
            }
        }
        psStruct = (*psStruct).psNext
    }
    if stackPushResult(VAL_INT, numFound as SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrNumAAinRange(): failed to push result\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
//select droid
#[no_mangle]
pub unsafe extern "C" fn scrSelectDroid() -> BOOL {
    let mut bSelect: BOOL = 0;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_DROID as libc::c_int,
                      &mut psDroid as *mut *mut DROID,
                      VAL_BOOL as libc::c_int, &mut bSelect as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrSelectDroid(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if psDroid.is_null() {
        debug(LOG_ERROR,
              b"scrSelectDroid(): droid is NULLOBJECT\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    (*psDroid).selected = bSelect as UBYTE;
    return 1 as libc::c_int;
}
//select droid group
#[no_mangle]
pub unsafe extern "C" fn scrSelectGroup() -> BOOL {
    let mut bSelect: BOOL = 0;
    let mut psGroup: *mut DROID_GROUP = 0 as *mut DROID_GROUP;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    if stackPopParams(2 as libc::c_int, ST_GROUP as libc::c_int,
                      &mut psGroup as *mut *mut DROID_GROUP,
                      VAL_BOOL as libc::c_int, &mut bSelect as *mut BOOL) == 0
       {
        debug(LOG_ERROR,
              b"scrSelectGroup(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    psCurr = (*psGroup).psList;
    while !psCurr.is_null() {
        (*psCurr).selected = bSelect as UBYTE;
        psCurr = (*psCurr).psGrpNext
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrModulo() -> BOOL {
    let mut num1: SDWORD = 0;
    let mut num2: SDWORD = 0;
    if stackPopParams(2 as libc::c_int, VAL_INT as libc::c_int,
                      &mut num1 as *mut SDWORD, VAL_INT as libc::c_int,
                      &mut num2 as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrModulo(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_INT, num1 % num2) == 0 {
        debug(LOG_ERROR,
              b"scrModulo(): failed to push result\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scrPlayerLoaded() -> BOOL {
    let mut player: SDWORD = 0;
    if stackPopParams(1 as libc::c_int, VAL_INT as libc::c_int,
                      &mut player as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrPlayerLoaded(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if stackPushResult(VAL_BOOL, game.skDiff[player as usize] as BOOL) == 0 {
        debug(LOG_ERROR,
              b"scrPlayerLoaded(): failed to push result\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *******************************/
		/*		AI Experience Stuff		*/
		/* *******************************/
//Returns enemy base x and y for a certain player
#[no_mangle]
pub unsafe extern "C" fn scrLearnPlayerBaseLoc() -> BOOL {
    let mut playerStoring: SDWORD = 0;
    let mut enemyPlayer: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    if stackPopParams(4 as libc::c_int, VAL_INT as libc::c_int,
                      &mut playerStoring as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut enemyPlayer as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut x as *mut SDWORD,
                      VAL_INT as libc::c_int, &mut y as *mut SDWORD) == 0 {
        debug(LOG_ERROR,
              b"scrLearnPlayerBaseLoc(): stack failed\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    if playerStoring >= 8 as libc::c_int || enemyPlayer >= 8 as libc::c_int {
        debug(LOG_ERROR,
              b"scrLearnPlayerBaseLoc: player index too high.\x00" as
                  *const u8 as *const libc::c_char);
        return 0 as libc::c_int
    }
    if playerStoring < 0 as libc::c_int || enemyPlayer < 0 as libc::c_int {
        debug(LOG_ERROR,
              b"scrLearnPlayerBaseLoc: player index too low.\x00" as *const u8
                  as *const libc::c_char);
        return 0 as libc::c_int
    }
    if x < 0 as libc::c_int || x >= (mapWidth as SDWORD) << 7 as libc::c_int
           || y < 0 as libc::c_int ||
           y >= (mapHeight as SDWORD) << 7 as libc::c_int {
        debug(LOG_ERROR,
              b"scrLearnPlayerBaseLoc: coords off map\x00" as *const u8 as
                  *const libc::c_char);
        return 0 as libc::c_int
    }
    baseLocation[playerStoring as
                     usize][enemyPlayer as usize][0 as libc::c_int as usize] =
        x;
    baseLocation[playerStoring as
                     usize][enemyPlayer as usize][1 as libc::c_int as usize] =
        y;
    //addConsoleMessage("Learned player base.",RIGHT_JUSTIFY);
    if stackPushResult(VAL_BOOL, 1 as libc::c_int) == 0 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
