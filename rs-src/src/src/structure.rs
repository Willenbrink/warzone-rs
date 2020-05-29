use ::libc;
extern "C" {
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_uint)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    #[no_mangle]
    fn rand() -> libc::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memMallocRelease(Size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memFreeRelease(pMemToFree: *mut libc::c_void);
    /* Return an object to the heap */
    #[no_mangle]
    fn heapFree(psHeap: *mut OBJ_HEAP, pObject: *mut libc::c_void) -> BOOL;
    #[no_mangle]
    static mut numPropulsionStats: UDWORD;
    #[no_mangle]
    static mut numBodyStats: UDWORD;
    #[no_mangle]
    static mut asBodyStats: *mut BODY_STATS;
    #[no_mangle]
    fn setFeatTileDraw(psFeat: *mut FEATURE);
    #[no_mangle]
    static mut numRepairStats: UDWORD;
    #[no_mangle]
    static mut apCompLists: [[*mut UBYTE; 9]; 8];
    #[no_mangle]
    fn removeStructureFromList(psStructToRemove: *mut STRUCTURE,
                               pList: *mut *mut STRUCTURE);
    #[no_mangle]
    static mut asPropulsionStats: *mut PROPULSION_STATS;
    #[no_mangle]
    static mut apStructTypeLists: [*mut UBYTE; 8];
    #[no_mangle]
    fn removeStructFunc(psDel: *mut FUNCTIONALITY);
    #[no_mangle]
    fn aiChooseTarget(psObj: *mut BASE_OBJECT,
                      ppsTarget: *mut *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn aiChooseSensorTarget(psObj: *mut BASE_OBJECT,
                            ppsTarget: *mut *mut BASE_OBJECT) -> BOOL;
    #[no_mangle]
    fn addDroid(psDroidToAdd: *mut DROID, pList: *mut *mut DROID);
    #[no_mangle]
    fn removeFlagPosition(psDel: *mut FLAG_POSITION);
    #[no_mangle]
    fn repairPoints(psStats: *mut REPAIR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    static mut asRepairStats: *mut REPAIR_STATS;
    #[no_mangle]
    fn createStruct(player: UDWORD, ppsNew: *mut *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn sensorRange(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn sensorPower(psStats: *mut SENSOR_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn ecmPower(psStats: *mut ECM_STATS, player: UBYTE) -> UDWORD;
    #[no_mangle]
    fn structureProductionUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureResearchUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structurePowerUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn structureRepairUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    fn createFlagPosition(ppsNew: *mut *mut FLAG_POSITION, player: UDWORD)
     -> BOOL;
    #[no_mangle]
    fn addFlagPosition(psFlagPosToAdd: *mut FLAG_POSITION);
    #[no_mangle]
    static mut apsFlagPosLists: [*mut FLAG_POSITION; 8];
    #[no_mangle]
    static mut apsDroidLists: [*mut DROID; 8];
    #[no_mangle]
    fn createStructFunc(ppsNew: *mut *mut FUNCTIONALITY) -> BOOL;
    #[no_mangle]
    fn structureReArmUpgrade(psBuilding: *mut STRUCTURE);
    #[no_mangle]
    static mut psStructHeap: *mut OBJ_HEAP;
    #[no_mangle]
    fn addStructure(psStructToAdd: *mut STRUCTURE);
    #[no_mangle]
    static mut apsFeatureLists: [*mut FEATURE; 8];
    #[no_mangle]
    fn removeFeature(psDel: *mut FEATURE);
    #[no_mangle]
    fn buildFeature(psStats: *mut FEATURE_STATS, x: UDWORD, y: UDWORD,
                    FromSave: BOOL) -> *mut FEATURE;
    #[no_mangle]
    static mut asFeatureStats: *mut FEATURE_STATS;
    #[no_mangle]
    static mut oilResFeature: UDWORD;
    #[no_mangle]
    fn killStruct(psDel: *mut STRUCTURE);
    #[no_mangle]
    static mut apsStructLists: [*mut STRUCTURE; 8];
    #[no_mangle]
    fn getWeaponEffect(pWeaponEffect: *mut STRING) -> UBYTE;
    #[no_mangle]
    static mut numFunctions: UDWORD;
    #[no_mangle]
    static mut asFunctions: *mut *mut FUNCTION;
    #[no_mangle]
    static mut numWeaponStats: UDWORD;
    #[no_mangle]
    static mut asWeaponStats: *mut WEAPON_STATS;
    #[no_mangle]
    fn numCR(pFileBuffer: *mut libc::c_char, fileSize: UDWORD) -> UDWORD;
    #[no_mangle]
    fn allocateName(ppStore: *mut *mut STRING, pName: *mut STRING) -> BOOL;
    #[no_mangle]
    fn setTechLevel(psStats: *mut BASE_STATS, pLevel: *mut STRING) -> BOOL;
    #[no_mangle]
    fn getStatName(pStat: *mut libc::c_void) -> *mut STRING;
    #[no_mangle]
    static mut asECMStats: *mut ECM_STATS;
    #[no_mangle]
    static mut numECMStats: UDWORD;
    #[no_mangle]
    fn getResourceName(pName: *mut STRING) -> BOOL;
    #[no_mangle]
    static mut asSensorStats: *mut SENSOR_STATS;
    #[no_mangle]
    static mut numSensorStats: UDWORD;
    #[no_mangle]
    fn getName(pNameID: *mut STRING) -> *mut STRING;
    #[no_mangle]
    fn repairPowerPoint(psDroid: *mut DROID) -> UWORD;
    #[no_mangle]
    fn powerReqForDroidRepair(psDroid: *mut DROID) -> UWORD;
    #[no_mangle]
    fn droidResistance(psDroid: *mut DROID) -> SWORD;
    #[no_mangle]
    fn giftSingleDroid(psD: *mut DROID, to: UDWORD) -> *mut DROID;
    #[no_mangle]
    fn vtolReadyToRearm(psDroid: *mut DROID, psStruct: *mut STRUCTURE)
     -> BOOL;
    #[no_mangle]
    fn vtolHappy(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn vtolDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn noDroid(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    fn droidTemplateType(psTemplate: *mut DROID_TEMPLATE) -> DROID_TYPE;
    #[no_mangle]
    fn recycleDroid(psDel: *mut DROID);
    #[no_mangle]
    fn idfDroid(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn templateIsIDF(psTemplate: *mut DROID_TEMPLATE) -> BOOL;
    #[no_mangle]
    fn buildDroid(pTemplate: *mut DROID_TEMPLATE, x: UDWORD, y: UDWORD,
                  player: UDWORD, onMission: BOOL) -> *mut DROID;
    #[no_mangle]
    fn audio_QueueTrackPos(iTrack: SDWORD, iX: SDWORD, iY: SDWORD,
                           iZ: SDWORD);
    #[no_mangle]
    fn audio_QueueTrack(iTrack: SDWORD);
    #[no_mangle]
    fn audio_PlayTrack(iTrack: libc::c_int);
    #[no_mangle]
    fn audio_PlayObjStaticTrack(psObj: *mut libc::c_void, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn audio_PlayStaticTrack(iX: SDWORD, iY: SDWORD, iTrack: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn animObj_Remove(ppsObj: *mut *mut ANIM_OBJECT, iAnimID: libc::c_int)
     -> BOOL;
    #[no_mangle]
    fn animObj_Add(pParentObj: *mut libc::c_void, iAnimID: libc::c_int,
                   udwStartDelay: UDWORD, uwCycles: UWORD)
     -> *mut ANIM_OBJECT;
    #[no_mangle]
    fn frameGetFrameNumber() -> UDWORD;
    #[no_mangle]
    fn resGetData(pType: *mut STRING, pID: *mut STRING) -> *mut libc::c_void;
    /* Get the string from an ID number */
    #[no_mangle]
    fn strresGetString(psRes: *mut STR_RES, id: UDWORD) -> *mut STRING;
    /* Set a tile to be visible for a player */
    /*#ifndef PSX
#define SET_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits | (1<<p))
#define CLEAR_TILE_DOOR(p,t) t->tileDoorBits = (UBYTE) (t->tileDoorBits & (~(1<<p))) // check logic
// Is there a door here for the player?
#define TEST_TILE_DOOR(p,t) ( (t->tileDoorBits) & (1<<p) )
#endif*/
/* Arbitrary maximum number of terrain textures - used in look up table for terrain type */
    #[no_mangle]
    static mut terrainTypes: [UBYTE; 255];
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
    /* returns the max and min height of a tile by looking at the four corners
   in tile coords */
    #[no_mangle]
    fn getTileMaxMin(x: UDWORD, y: UDWORD, pMax: *mut UDWORD,
                     pMin: *mut UDWORD);
    //scroll min and max values
    #[no_mangle]
    static mut scrollMinX: SDWORD;
    #[no_mangle]
    static mut scrollMaxX: SDWORD;
    #[no_mangle]
    static mut scrollMinY: SDWORD;
    #[no_mangle]
    static mut scrollMaxY: SDWORD;
    // The maximum time for one frame (stops the clock running away when debugging)
// changed to /6 by ajl. if this needs to go back to ticks/10 then tell me. 
    /* The current time in the game world */
    #[no_mangle]
    static mut gameTime: UDWORD;
    /* Check which tiles can be seen by an object */
    #[no_mangle]
    fn visTilesUpdate(psObj: *mut BASE_OBJECT, SpreadLoad: BOOL);
    #[no_mangle]
    fn processVisibility(psCurr: *mut BASE_OBJECT);
    #[no_mangle]
    fn setUnderTilesVis(psObj: *mut BASE_OBJECT, player: UDWORD);
    //List of pointers to arrays of PLAYER_RESEARCH[numResearch] for each player
    #[no_mangle]
    static mut asPlayerResList: [*mut PLAYER_RESEARCH; 8];
    /* Default level of sensor, repair and ECM */
    #[no_mangle]
    static mut aDefaultSensor: [UDWORD; 8];
    #[no_mangle]
    static mut aDefaultRepair: [UDWORD; 8];
    /* process the results of a completed research topic */
    #[no_mangle]
    fn researchResult(researchIndex: UDWORD, player: UBYTE, bDisplay: BOOL);
    /* sets the status of the topic to cancelled and stores the current research
   points accquired */
    #[no_mangle]
    fn cancelResearch(psBuilding: *mut STRUCTURE);
    /*find the last research topic of importance that the losing player did and 
'give' the results to the reward player*/
    #[no_mangle]
    fn researchReward(losingPlayer: UBYTE, rewardPlayer: UBYTE);
    /*check to see if any research has been completed that enables self repair*/
    #[no_mangle]
    fn selfRepairEnabled(player: UBYTE) -> BOOL;
    /*puts research facility on hold*/
    #[no_mangle]
    fn holdResearch(psBuilding: *mut STRUCTURE);
    /*release a research facility from hold*/
    #[no_mangle]
    fn releaseResearch(psBuilding: *mut STRUCTURE);
    /*checks the stat to see if its of type wall or defence*/
    #[no_mangle]
    fn wallDefenceStruct(psStats: *mut STRUCTURE_STATS) -> BOOL;
    //arbitary value!
    //value gets set to colour used for drawing
    #[no_mangle]
    static mut outlineColour3D: UDWORD;
    #[no_mangle]
    static mut outlineOK: UDWORD;
    #[no_mangle]
    static mut outlineColour: UDWORD;
    #[no_mangle]
    static mut outlineNotOK: UDWORD;
    //Buffer to hold the 3D view for the Intelligence Screen
    /*Message View Buffer width and height - MAXIMUM Sizes! - only need to be
as big as Pie View in Research Msg now*/
    //DISP_WIDTH//640
    //DISP_HEIGHT//480
    /* pointer to hold the imd to use for a new template in the design screen */
    /* Initialise the in game interface */
    /* Shut down the in game interface */
    /* Return codes for the widget interface */
    // no key clicks have been intercepted
    // key clicks have been intercepted
    //INT_FULLSCREENPAUSE,	// The widget interface is full screen and
							// the rest of the game should pause
	//INT_INTELPAUSE,			// The Intelligence Map is up and all update
							// routines should pause - hopefully!
    //The 3DView of the intelligence screen is up
    // and we don't want scroll (or update!)
    // The game should quit
    /* Run the widgets for the in game interface */
    /* Display the widgets for the in game interface */
    /* Add the reticule widgets to the widget screen */
    /* Set the map view point to the world coordinates x,y */
    /* Set the map view point to the world coordinates x,y */
    /* Tell the interface when an object is created
 * - it may have to be added to a screen
 */
    /* Tell the interface a construction droid has finished building */
    /* Tell the interface a construction droid has started building*/
    /* Tell the interface a research facility has completed a topic */
    /* Tell the interface a factory has completed building ALL droids */
    #[no_mangle]
    fn intManufactureFinished(psBuilding: *mut STRUCTURE);
    /* Sync the interface to an object */
    // add the construction interface if a constructor droid is selected
    // add the construction interface if a constructor droid is selected
    //sets up the Intelligence Screen as far as the interface is concerned
//extern void addIntelScreen(BOOL playImmediate);
    // update shadow...
    /* Reset the widget screen to just the reticule */
    /* Refresh icons on the interface, without disturbing the layout. i.e. smartreset*/
    #[no_mangle]
    fn intRefreshScreen();
    #[no_mangle]
    static mut selectedPlayer: UDWORD;
    #[no_mangle]
    fn intCheckResearchButton();
    #[no_mangle]
    fn intResearchFinished(psBuilding: *mut STRUCTURE);
    /*check the available power*/
    #[no_mangle]
    fn checkPower(player: UDWORD, quantity: UDWORD, playAudio: BOOL) -> BOOL;
    //return the power when a structure/droid is deliberately destroyed
    #[no_mangle]
    fn addPower(player: UDWORD, quantity: UDWORD);
    /*resets the power levels for all players when power is turned back on*/
    #[no_mangle]
    fn powerCalc(on: BOOL);
    //informs the power array that a Object has been destroyed
    #[no_mangle]
    fn powerDestroyObject(psObject: *mut BASE_OBJECT);
    //extern void spreadPower(UBYTE player);
/*accrue the power in the facilities that require it*/
    #[no_mangle]
    fn accruePower(psObject: *mut BASE_OBJECT) -> BOOL;
    //returns the relevant list based on OffWorld or OnWorld for the accruePower function
//extern STRUCTURE* powerUpdateStructList(UBYTE player);
    /*inform the players power struct that the last Object to receive power has changed*/
    #[no_mangle]
    fn updateLastPowered(psObject: *mut BASE_OBJECT, player: UBYTE);
    /*checks if the object to be powered next - returns TRUE if power*/
    #[no_mangle]
    fn getLastPowered(psStructure: *mut BASE_OBJECT) -> BOOL;
    /*defines which structure types draw power - returns TRUE if use power*/
    #[no_mangle]
    fn structUsesPower(psStruct: *mut STRUCTURE) -> BOOL;
    //flag used to check for power calculations to be done or not
    #[no_mangle]
    static mut powerCalculated: BOOL;
    #[no_mangle]
    fn getImdFromIndex(index: UDWORD) -> *mut iIMDShape;
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
    fn effectSetSize(size: UDWORD);
    /* Fire a weapon at something */
    #[no_mangle]
    fn combFire(psWeap: *mut WEAPON, psAttacker: *mut BASE_OBJECT,
                psTarget: *mut BASE_OBJECT);
    #[no_mangle]
    fn kill3DBuilding();
    #[no_mangle]
    static mut sBuildDetails: BUILDDETAILS;
    /*returns true if the build state is not equal to BUILD3D_NONE*/
    #[no_mangle]
    fn tryingToGetLocation() -> BOOL;
    #[no_mangle]
    static mut radarOnScreen: BOOL;
    #[no_mangle]
    fn clipXY(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    static mut mouseTileX: SDWORD;
    #[no_mangle]
    static mut mouseTileY: SDWORD;
    #[no_mangle]
    fn getTileStructure(x: UDWORD, y: UDWORD) -> *mut STRUCTURE;
    #[no_mangle]
    fn getTileFeature(x: UDWORD, y: UDWORD) -> *mut FEATURE;
    #[no_mangle]
    fn calcDirection(x0: UDWORD, y0: UDWORD, x1: UDWORD, y1: UDWORD)
     -> SDWORD;
    /* **************************************************************************/
/*
 *	Global Variables
 */
/* **************************************************************************/
    #[no_mangle]
    static mut psMatrix: *mut SDMATRIX;
    //*************************************************************************
    #[no_mangle]
    fn pie_MatBegin();
    #[no_mangle]
    fn pie_MatEnd();
    #[no_mangle]
    fn pie_TRANSLATE(x: libc::c_int, y: libc::c_int, z: libc::c_int);
    #[no_mangle]
    fn pie_MatRotX(x: libc::c_int);
    #[no_mangle]
    fn pie_MatRotY(y: libc::c_int);
    #[no_mangle]
    fn pie_MatRotZ(z: libc::c_int);
    /* Give a droid an order */
    #[no_mangle]
    fn orderDroid(psDroid: *mut DROID, order: DROID_ORDER);
    /* Check the order state of a droid */
    #[no_mangle]
    fn orderState(psDroid: *mut DROID, order: DROID_ORDER) -> BOOL;
    /* Give a droid an order with a location target */
    #[no_mangle]
    fn orderDroidLoc(psDroid: *mut DROID, order: DROID_ORDER, x: UDWORD,
                     y: UDWORD);
    /* Give a droid an order with an object target */
    #[no_mangle]
    fn orderDroidObj(psDroid: *mut DROID, order: DROID_ORDER,
                     psObj: *mut BASE_OBJECT);
    /* Get the state of a droid order with an object */
    #[no_mangle]
    fn orderStateObj(psDroid: *mut DROID, order: DROID_ORDER,
                     ppsObj: *mut *mut BASE_OBJECT) -> BOOL;
    // set the state of a secondary order, return FALSE if failed.
    #[no_mangle]
    fn secondarySetState(psDroid: *mut DROID, sec: SECONDARY_ORDER,
                         State: SECONDARY_STATE) -> BOOL;
    #[no_mangle]
    fn eventFireCallbackTrigger(callback: TRIGGER_TYPE);
    #[no_mangle]
    static mut psScrCBNewDroid: *mut DROID;
    #[no_mangle]
    static mut psScrCBNewDroidFact: *mut STRUCTURE;
    // remove all the members from a formation and release it
    #[no_mangle]
    fn formationReset(psFormation: *mut FORMATION);
    /* The string resource object */
    #[no_mangle]
    static mut psStringRes: *mut STR_RES;
    /* Give a droid an action with a location target */
    #[no_mangle]
    fn actionDroidLoc(psDroid: *mut DROID, action: DROID_ACTION, x: UDWORD,
                      y: UDWORD);
    /* Give a droid an action with an object target */
    #[no_mangle]
    fn actionDroidObj(psDroid: *mut DROID, action: DROID_ACTION,
                      psObj: *mut BASE_OBJECT);
    /* Give a droid an action with an object target and a location */
    #[no_mangle]
    fn actionDroidObjLoc(psDroid: *mut DROID, action: DROID_ACTION,
                         psObj: *mut BASE_OBJECT, x: UDWORD, y: UDWORD);
    /* Rotate turret toward  target return True if locked on (Droid and Structure) */
/*extern BOOL actionTargetTurret(BASE_OBJECT *psAttacker, BASE_OBJECT *psTarget,
								UWORD *pRotation, UWORD *pPitch, SWORD rotRate,
								SWORD pitchRate, BOOL bDirectFire, BOOL bInvert);*/
//								UDWORD *pRotation, UDWORD *pPitch, SDWORD rotRate,
//								SDWORD pitchRate, BOOL bDirectFire, BOOL bInvert);
    #[no_mangle]
    fn actionTargetTurret(psAttacker: *mut BASE_OBJECT,
                          psTarget: *mut BASE_OBJECT, pRotation: *mut UWORD,
                          pPitch: *mut UWORD, psWeapStats: *mut WEAPON_STATS,
                          bInvert: BOOL) -> BOOL;
    // Realign turret
    #[no_mangle]
    fn actionAlignTurret(psObj: *mut BASE_OBJECT);
    /*send the vtol droid back to the nearest rearming pad - if one otherwise
return to base*/
    #[no_mangle]
    fn moveToRearm(psDroid: *mut DROID);
    // choose a landing position for a VTOL when it goes to rearm
    #[no_mangle]
    fn actionVTOLLandingPos(psDroid: *mut DROID, px: *mut UDWORD,
                            py: *mut UDWORD) -> BOOL;
    // create a new group
    #[no_mangle]
    fn grpCreate(ppsGroup: *mut *mut DROID_GROUP) -> BOOL;
    // add a droid to a group
    #[no_mangle]
    fn grpJoin(psGroup: *mut DROID_GROUP, psDroid: *mut DROID);
    // count the members of a group
    #[no_mangle]
    fn grpNumMembers(psGroup: *mut DROID_GROUP) -> SDWORD;
    #[no_mangle]
    static mut fpathBlockingTile:
           Option<unsafe extern "C" fn(_: SDWORD, _: SDWORD) -> BOOL>;
    // Check if the map tile at a location blocks a droid
    #[no_mangle]
    fn fpathGroundBlockingTile(x: SDWORD, y: SDWORD) -> BOOL;
    #[no_mangle]
    static mut mission: MISSION;
    #[no_mangle]
    fn missionIsOffworld() -> BOOL;
    //returns TRUE if the mission is a Limbo Expand mission
    #[no_mangle]
    fn missionLimboExpand() -> BOOL;
    /*	checks the x,y passed in are not within the boundary of the Landing Zone
	x and y in tile coords */
    #[no_mangle]
    fn withinLandingZone(x: UDWORD, y: UDWORD) -> BOOL;
    #[no_mangle]
    static mut ConsoleString: [libc::c_char; 255];
    #[no_mangle]
    fn addConsoleMessage(messageText: *mut STRING,
                         jusType: CONSOLE_TEXT_JUSTIFICATION) -> BOOL;
    // add a droid to a command group
    #[no_mangle]
    fn cmdDroidAddDroid(psCommander: *mut DROID, psDroid: *mut DROID);
    // get the maximum group size for a command droid
    #[no_mangle]
    fn cmdDroidMaxGroup(psCommander: *mut DROID) -> SDWORD;
    // add an object to the grid system
    #[no_mangle]
    fn gridAddObject(psObj: *mut BASE_OBJECT);
    // remove an object from the grid system
    #[no_mangle]
    fn gridRemoveObject(psObj: *mut BASE_OBJECT);
    // return whether a weapon is direct or indirect
    #[no_mangle]
    fn proj_Direct(psStats: *mut WEAPON_STATS) -> BOOL;
    // tell the cluster system about a new structure
    #[no_mangle]
    fn clustNewStruct(psStruct: *mut STRUCTURE);
    // update all objects from a list belonging to a specific cluster
    #[no_mangle]
    fn clustUpdateCluster(psList: *mut BASE_OBJECT, cluster: SDWORD);
    // remove an object from the cluster system
    #[no_mangle]
    fn clustRemoveObject(psObj: *mut BASE_OBJECT);
    // tell the cluster system that an object has been attacked
    #[no_mangle]
    fn clustObjectAttacked(psObj: *mut BASE_OBJECT);
    //void RenderCompositeDroid(UDWORD Index,iVector *Rotation,iVector *Position);
    //iIMDShape *TemplateGetIMD(DROID_TEMPLATE *DroidTemp,UDWORD Player);
//UDWORD TemplateGetIMDIndex(DROID_TEMPLATE *Template,UDWORD Player);
    //SDWORD ResearchGetImage(RESEARCH_FACILITY *Research);
    #[no_mangle]
    fn StatIsFeature(Stat: *mut BASE_STATS) -> BOOL;
    #[no_mangle]
    fn StatIsStructure(Stat: *mut BASE_STATS) -> BOOL;
    #[no_mangle]
    fn shakeStart();
    #[no_mangle]
    static mut wallDrag: _dragBox;
    // check whether the queue order keys are pressed
    #[no_mangle]
    fn ctrlShiftDown() -> BOOL;
    #[no_mangle]
    fn modifyForDifficultyLevel(basicVal: SDWORD, IsPlayer: BOOL) -> SDWORD;
    // whether the tutorial is active
    #[no_mangle]
    static mut bInTutorial: BOOL;
    #[no_mangle]
    fn getDebugMappingStatus() -> BOOL;
    #[no_mangle]
    fn avInformOfChange(x: SDWORD, y: SDWORD);
    #[no_mangle]
    fn getRevealStatus() -> BOOL;
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
    //send AI message
    #[no_mangle]
    fn turnOffMultiMsg(bDoit: BOOL) -> BOOL;
    #[no_mangle]
    fn SendDestroyStructure(s: *mut STRUCTURE) -> BOOL;
    #[no_mangle]
    fn sendDroidSecondaryAll(psDroid: *mut DROID) -> BOOL;
    #[no_mangle]
    fn sendHappyVtol(psDroid: *mut DROID) -> BOOL;
    // send to player, possibly guaranteed
    #[no_mangle]
    fn NETbcast(msg: *mut NETMSG, guarantee: BOOL) -> BOOL;
    // Number of units in the current list.
    #[no_mangle]
    fn getNumDroids(player: UDWORD) -> UDWORD;
    // Number of units on transporters.
    #[no_mangle]
    fn getNumTransporterDroids(player: UDWORD) -> UDWORD;
    // Number of units in the mission list.
    #[no_mangle]
    fn getNumMissionDroids(player: UDWORD) -> UDWORD;
    #[no_mangle]
    fn getNumCommandDroids(player: UDWORD) -> UDWORD;
    #[no_mangle]
    fn getNumConstructorDroids(player: UDWORD) -> UDWORD;
    // increase the droid counts - used by update factory to keep the counts in sync
    #[no_mangle]
    fn incNumDroids(player: UDWORD);
    #[no_mangle]
    fn incNumCommandDroids(player: UDWORD);
    #[no_mangle]
    fn incNumConstructorDroids(player: UDWORD);
    #[no_mangle]
    fn scoreUpdateVar(var: DATA_INDEX);
    // Look up the zone for a coordinate
    #[no_mangle]
    fn gwGetZone(x: SDWORD, y: SDWORD) -> SDWORD;
    // see if a zone is reachable
    #[no_mangle]
    fn gwZoneReachable(zone: SDWORD) -> BOOL;
}
pub type size_t = libc::c_uint;
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
// The next free ID
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
pub type _loc = libc::c_uint;
pub const LOC_TURRET: _loc = 1;
pub const LOC_DEFAULT: _loc = 0;
pub type _size = libc::c_uint;
pub const SIZE_SUPER_HEAVY: _size = 3;
pub const SIZE_HEAVY: _size = 2;
pub const SIZE_MEDIUM: _size = 1;
pub const SIZE_LIGHT: _size = 0;
pub type _weapon_class = libc::c_uint;
pub const NUM_WEAPON_CLASS: _weapon_class = 2;
pub const WC_HEAT: _weapon_class = 1;
pub const WC_KINETIC: _weapon_class = 0;
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
pub type WEAPON_SUBCLASS = _weapon_subclass;
pub type _movement_model = libc::c_uint;
pub const NUM_MOVEMENT_MODEL: _movement_model = 6;
pub const MM_SWEEP: _movement_model = 5;
pub const MM_ERRATICDIRECT: _movement_model = 4;
pub const MM_HOMINGINDIRECT: _movement_model = 3;
pub const MM_HOMINGDIRECT: _movement_model = 2;
pub const MM_INDIRECT: _movement_model = 1;
pub const MM_DIRECT: _movement_model = 0;
pub type MOVEMENT_MODEL = _movement_model;
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
pub type FUNCTION_TYPES = libc::c_uint;
pub const NUMFUNCTIONS: FUNCTION_TYPES = 20;
pub const REARM_UPGRADE_TYPE: FUNCTION_TYPES = 19;
pub const REARM_TYPE: FUNCTION_TYPES = 18;
pub const DROIDCONST_UPGRADE_TYPE: FUNCTION_TYPES = 17;
pub const DROIDSENSOR_UPGRADE_TYPE: FUNCTION_TYPES = 16;
pub const DROIDBODY_UPGRADE_TYPE: FUNCTION_TYPES = 15;
pub const DROIDECM_UPGRADE_TYPE: FUNCTION_TYPES = 14;
pub const DROIDREPAIR_UPGRADE_TYPE: FUNCTION_TYPES = 13;
pub const REPAIR_UPGRADE_TYPE: FUNCTION_TYPES = 12;
pub const POWER_UPGRADE_TYPE: FUNCTION_TYPES = 11;
pub const WALLDEFENCE_UPGRADE_TYPE: FUNCTION_TYPES = 10;
pub const STRUCTURE_UPGRADE_TYPE: FUNCTION_TYPES = 9;
pub const WALL_TYPE: FUNCTION_TYPES = 8;
pub const WEAPON_UPGRADE_TYPE: FUNCTION_TYPES = 7;
pub const REPAIR_DROID_TYPE: FUNCTION_TYPES = 6;
pub const RESOURCE_TYPE: FUNCTION_TYPES = 5;
pub const POWER_GEN_TYPE: FUNCTION_TYPES = 4;
pub const RESEARCH_UPGRADE_TYPE: FUNCTION_TYPES = 3;
pub const RESEARCH_TYPE: FUNCTION_TYPES = 2;
pub const PRODUCTION_UPGRADE_TYPE: FUNCTION_TYPES = 1;
pub const PRODUCTION_TYPE: FUNCTION_TYPES = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
}
pub type FUNCTION = _function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repair_droid_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub repairPoints: UDWORD,
}
pub type REPAIR_DROID_FUNCTION = _repair_droid_function;
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
pub struct _wall_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub pStructName: *mut STRING,
    pub pCornerStat: *mut _structure_stats,
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
pub type WALL_FUNCTION = _wall_function;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _resource_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub maxPower: UDWORD,
}
pub type RESOURCE_FUNCTION = _resource_function;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _research_function {
    pub ref_0: UDWORD,
    pub pName: *mut STRING,
    pub type_0: UBYTE,
    pub researchPoints: UDWORD,
}
pub type RESEARCH_FUNCTION = _research_function;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _formation {
    pub refCount: SWORD,
    pub size: SWORD,
    pub rankDist: SWORD,
    pub dir: SWORD,
    pub x: SDWORD,
    pub y: SDWORD,
    pub asLines: [F_LINE; 4],
    pub numLines: SWORD,
    pub maxRank: UBYTE,
    pub free: SBYTE,
    pub asMembers: [F_MEMBER; 20],
    pub iSpeed: UDWORD,
    pub psNext: *mut _formation,
}
// information about a formation member
pub type F_MEMBER = _f_member;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_member {
    pub line: SBYTE,
    pub next: SBYTE,
    pub dist: SWORD,
    pub psObj: *mut BASE_OBJECT,
}
pub type BASE_OBJECT = _base_object;
// which line this member is on
// the next member on this line
// distance along the line
// the member unit
/*
 * FormationDef.h
 *
 */
// maximum number of lines in a formation
// maximum number of unit members of a formation (cannot be more that 128)
// information about a formation line
// a linked list of the formation members on this line is maintained
// using their index in the asMembers array.  (-1 == 'NULL')
// (cuts down the memory use over proper pointers)
pub type F_LINE = _f_line;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _f_line {
    pub xoffset: SWORD,
    pub yoffset: SWORD,
    pub dir: SWORD,
    pub member: SBYTE,
}
// position relative to center
// orientation of line
// first member in the 'linked list' of members
/*
 * MoveDef.h
 *
 * Structure definitions for movement structures.
 *
 */
//	SDWORD		XCoordinate,YCoordinate;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _weapon {
    pub nStat: UDWORD,
    pub hitPoints: UDWORD,
    pub ammo: UDWORD,
    pub lastFired: UDWORD,
    pub recoilValue: UDWORD,
}
// Inactive, Navigating or moving point to point status
// Mask used for the creation of this path	
//	SBYTE	Direction;					// Direction object should be moving (0-7) 0=Up,1=Up-Right
//	SDWORD	Speed;						// Speed at which object moves along the movement list
// Position in asPath
// number of points in asPath
//	PATH_POINT	MovementList[TRAVELSIZE];
// Pointer to list of block X,Y coordinates.
// When initialised list is terminated by (0xffff,0xffff)
										// Values prefixed by 0x8000 are pixel coordinates instead of
										// block coordinates
// DestinationX,Y should match objects current X,Y
//		location for this movement to be complete.
//   	UDWORD	Direction3D;				// *** not necessary
//	UDWORD	TargetDir;					// *** not necessary Direction the object should be facing
//	SDWORD	Gradient;					// Gradient of line
//	SDWORD	DeltaX;						// Distance from start to end position of current movement X
//	SDWORD	DeltaY;						// Distance from start to end position of current movement Y
//	SDWORD	XStep;						// Adjustment to the characters X position each movement
//	SDWORD	YStep;						// Adjustment to the characters Y position each movement
//	SDWORD	DestPixelX;					// Pixel coordinate destination for pixel movement (NOT the final X)
//	SDWORD	DestPixelY;					// Pixel coordiante destination for pixel movement (NOT the final Y)
/* Stuff for John's movement update */
// droid location as a fract
//	FRACT	dx,dy;						// x and y change for current direction
	// NOTE: this is supposed to replace Speed
// Speed of motion
// Vector for the end of path boundary
// direction of motion (not the direction the droid is facing)
// direction at last bump
// time of first bump with something
// time of last bump with a droid - relative to bumpTime
// when MOVEPAUSE started - relative to bumpTime
// position of last bump
// when a shuffle started
// formation the droid is currently a member of
/* vtol movement - GJ */
// added for vtol movement
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
pub type WEAPON = _weapon;
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
pub type RUN_DATA = _run_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _run_data {
    pub sPos: POINT,
    pub forceLevel: UBYTE,
    pub healthLevel: UBYTE,
    pub leadership: UBYTE,
}
// The stats for the weapon type
// When the weapon last fired
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
pub type STRUCTSTRENGTH_MODIFIER = UWORD;
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
pub struct _res_extractor {
    pub power: UDWORD,
    pub timeLastUpdated: UDWORD,
    pub active: BOOL,
    pub psPowerGen: *mut _structure,
}
pub type RES_EXTRACTOR = _res_extractor;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rearm_pad {
    pub reArmPoints: UDWORD,
    pub timeStarted: UDWORD,
    pub psObj: *mut BASE_OBJECT,
    pub currentPtsAdded: UDWORD,
}
pub type REARM_PAD = _rearm_pad;
pub type STRUCTURE = _structure;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_limits {
    pub limit: UBYTE,
    pub currentQuantity: UBYTE,
    pub globalLimit: UBYTE,
}
pub type STRUCTURE_LIMITS = _structure_limits;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _production_run {
    pub quantity: UBYTE,
    pub built: UBYTE,
    pub psTemplate: *mut _droid_template,
}
pub type PRODUCTION_RUN = _production_run;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _structure_upgrade {
    pub armour: UWORD,
    pub body: UWORD,
    pub resistance: UWORD,
}
pub type STRUCTURE_UPGRADE = _structure_upgrade;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wallDefence_upgrade {
    pub armour: UWORD,
    pub body: UWORD,
}
pub type WALLDEFENCE_UPGRADE = _wallDefence_upgrade;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _upgrade {
    pub modifier: UWORD,
}
pub type UPGRADE = _upgrade;
pub type RESEARCH_UPGRADE = UPGRADE;
pub type PRODUCTION_UPGRADE = UPGRADE;
pub type REPAIR_FACILITY_UPGRADE = UPGRADE;
pub type POWER_UPGRADE = UPGRADE;
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
//this is used for module graphics - factory and vtol factory
//number to build
//number built on current run
//template to build
/* structure stats which can be upgraded by research*/
/* wall/Defence structure stats which can be upgraded by research*/
//% to increase the stat by
pub type REARM_UPGRADE = UPGRADE;
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
pub type CONSOLE_TEXT_JUSTIFICATION = libc::c_uint;
pub const DEFAULT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 3;
pub const CENTRE_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 2;
pub const RIGHT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 1;
pub const LEFT_JUSTIFY: CONSOLE_TEXT_JUSTIFICATION = 0;
pub const STR_GAM_MAXUNITSREACHED: _fixed_str_id = 302;
/* The common structure elements for all objects */
/*time the feature was created - valid for 
									  wrecked droids and structures */
/* current body points */
// how much to scale the graphic by - for variation - spice of life 'n all that
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
/* Information stored with each tile */
// The name is now changed to MAPTILE to allow correct compilation on the PlayStation
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
pub type BUILDCALLBACK
    =
    Option<unsafe extern "C" fn(_: UDWORD, _: UDWORD, _: *mut libc::c_void)
               -> ()>;
pub const ID_SOUND_EXPLOSION: C2RustUnnamed_1 = 271;
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
pub const ID_SOUND_WINDOWCLOSE: C2RustUnnamed_1 = 0;
pub type HIGHLIGHT = _highlight;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _highlight {
    pub xTL: UWORD,
    pub yTL: UWORD,
    pub xBR: UWORD,
    pub yBR: UWORD,
}
pub const DORDER_BUILD: _droid_order = 4;
pub const TER_CLIFFFACE: _terrain_type = 8;
pub const TER_WATER: _terrain_type = 7;
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
pub type DROID_GROUP = _droid_group;
pub const DSS_HALT_GUARD: _secondary_state = 128;
pub const DSS_ALEV_ALWAYS: _secondary_state = 16;
pub const DSS_REPLEV_NEVER: _secondary_state = 12;
pub const DSS_ARANGE_DEFAULT: _secondary_state = 3;
pub const ID_SOUND_POWER_HUM: C2RustUnnamed_1 = 295;
pub const MI_PLASMA: C2RustUnnamed_0 = 15;
pub const DACTION_NONE: _droid_action = 0;
pub const DACTION_WAITDURINGREARM: _droid_action = 35;
pub const DACTION_MOVETOREARMPOINT: _droid_action = 34;
pub const MI_FLAME: C2RustUnnamed_0 = 21;
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
//this holds the OBJECT_POSITION pointer for a Deliv Point
// Top left of box to highlight
// Bottom right of box to highlight
// list of droids in the group
// the command droid of a command group
// where the group should retreat to
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
pub const DORDER_ATTACK: _droid_order = 3;
pub const DORDER_MOVE: _droid_order = 2;
pub const DORDER_STOP: _droid_order = 1;
pub const DORDER_NONE: _droid_order = 0;
pub const GT_COMMAND: _group_type = 1;
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
pub const DSS_HALT_HOLD: _secondary_state = 64;
pub const DSS_ALEV_NEVER: _secondary_state = 48;
pub const DSS_ALEV_ATTACKED: _secondary_state = 32;
pub const DSS_REPLEV_HIGH: _secondary_state = 8;
pub const DSS_REPLEV_LOW: _secondary_state = 4;
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
pub const DACTION_WAITDURINGREPAIR: _droid_action = 28;
// assign production to a command droid - state is the factory number
// remove production from a command droid
// patrol between current pos and next move target
// what to do when stopped
// return to various locations
// command droid controlling IDF structures
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
pub const CALL_NEWDROID: _scr_callback_types = 35;
pub const CALL_DROIDBUILT: _scr_callback_types = 8;
// Trigger fires when the script is first run
// Trigger uses script code
// Trigger after a time pause
// Trigger at repeated intervals
// Event has paused for an interval and will restart in the middle of it's code
// The user defined callback triggers should start with this id
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
//UDWORD		initialPower;	    HAVEN'T FOUND A USE FOR IT YET! AB 26/8/98	
									    /* what the initial power level is - set 
									    in script not sure if will need it, but 
									    keeping for now*/
/* the current amount of power avaialble 
									    to the player*/
/* the power extracted but not converted */
/*the last object that received power 
									    before it ran out*/
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
/* Store a map coordinate and it's associated tile, used by mapCalcLine */
pub type TILE_COORD = _tile_coord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tile_coord {
    pub x: UDWORD,
    pub y: UDWORD,
    pub psTile: *mut MAPTILE,
}
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
pub const DACTION_WAITFORREARM: _droid_action = 33;
pub const DACTION_MOVETOREARM: _droid_action = 32;
pub const DACTION_MOVETOCLEAR: _droid_action = 31;
pub const DACTION_MOVETORESTORE: _droid_action = 30;
pub const DACTION_MOVETODROIDREPAIR: _droid_action = 29;
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
pub const DACTION_TRANSPORTIN: _droid_action = 13;
pub const DACTION_TRANSPORTWAITTOFLYIN: _droid_action = 12;
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
pub const ID_SOUND_DROID_COMPLETED: C2RustUnnamed_1 = 44;
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
// information about a formation
pub type FORMATION = _formation;
pub const CALL_FACTORY_BUILT: _scr_callback_types = 12;
pub const CALL_RESEARCH_BUILT: _scr_callback_types = 11;
pub const CALL_RESEX_BUILT: _scr_callback_types = 10;
pub const CALL_POWERGEN_BUILT: _scr_callback_types = 9;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDMATRIX {
    pub a: SDWORD,
    pub b: SDWORD,
    pub c: SDWORD,
    pub d: SDWORD,
    pub e: SDWORD,
    pub f: SDWORD,
    pub g: SDWORD,
    pub h: SDWORD,
    pub i: SDWORD,
    pub j: SDWORD,
    pub k: SDWORD,
    pub l: SDWORD,
}
pub const STR_GAM_POWCON: _fixed_str_id = 226;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NETMSG {
    pub size: libc::c_ushort,
    pub paddedBytes: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub destination: libc::c_uchar,
    pub body: [libc::c_char; 8000],
}
pub const NET_GIFT: _msgtype = 31;
pub const CALL_ELECTRONIC_TAKEOVER: _scr_callback_types = 20;
pub const STR_GAM_ELECDAM: _fixed_str_id = 228;
pub const ID_SOUND_NEXUS_STRUCTURE_ABSORBED: C2RustUnnamed_1 = 342;
pub const ID_SOUND_NEXUS_DEFENCES_ABSORBED: C2RustUnnamed_1 = 335;
pub const STR_GAM_REWREPN: _fixed_str_id = 243;
pub const STR_GAM_REWREPA: _fixed_str_id = 242;
pub const STR_GAM_REWELEC: _fixed_str_id = 237;
pub const STR_GAM_REWNOWT: _fixed_str_id = 241;
pub const STR_GAM_REWWEAP: _fixed_str_id = 240;
pub const STR_GAM_REWBODY: _fixed_str_id = 239;
pub const STR_GAM_REWPROP: _fixed_str_id = 238;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MI_TOO_MANY: C2RustUnnamed_0 = 44;
pub const MI_FIREWORK: C2RustUnnamed_0 = 43;
pub const MI_DEBRIS4: C2RustUnnamed_0 = 42;
pub const MI_DEBRIS3: C2RustUnnamed_0 = 41;
pub const MI_DEBRIS2: C2RustUnnamed_0 = 40;
pub const MI_DEBRIS1: C2RustUnnamed_0 = 39;
pub const MI_DEBRIS0: C2RustUnnamed_0 = 38;
pub const MI_WRECK4: C2RustUnnamed_0 = 37;
pub const MI_WRECK3: C2RustUnnamed_0 = 36;
pub const MI_WRECK2: C2RustUnnamed_0 = 35;
pub const MI_WRECK1: C2RustUnnamed_0 = 34;
pub const MI_WRECK0: C2RustUnnamed_0 = 33;
pub const MI_BLIP_ARTEFACT: C2RustUnnamed_0 = 32;
pub const MI_BLIP_RESOURCE: C2RustUnnamed_0 = 31;
pub const MI_BLIP_ENEMY: C2RustUnnamed_0 = 30;
pub const MI_SHOCK: C2RustUnnamed_0 = 29;
pub const MI_LANDING: C2RustUnnamed_0 = 28;
pub const MI_KICK: C2RustUnnamed_0 = 27;
pub const MI_SPLASH: C2RustUnnamed_0 = 26;
pub const MI_SNOW: C2RustUnnamed_0 = 25;
pub const MI_RAIN: C2RustUnnamed_0 = 24;
pub const MI_MFLARE: C2RustUnnamed_0 = 23;
pub const MI_TESLA: C2RustUnnamed_0 = 22;
pub const MI_TRAIL: C2RustUnnamed_0 = 20;
pub const MI_BLOOD: C2RustUnnamed_0 = 19;
pub const MI_TRANSPORTER_SHADOW: C2RustUnnamed_0 = 18;
pub const MI_SHADOW: C2RustUnnamed_0 = 17;
pub const MI_BLIP: C2RustUnnamed_0 = 16;
pub const MI_SMALL_STEAM: C2RustUnnamed_0 = 14;
pub const MI_DROID_DAMAGE: C2RustUnnamed_0 = 13;
pub const MI_WATER: C2RustUnnamed_0 = 12;
pub const MI_CYBORG_BODY: C2RustUnnamed_0 = 11;
pub const MI_CYBORG_ARM: C2RustUnnamed_0 = 10;
pub const MI_CYBORG_LEGS: C2RustUnnamed_0 = 9;
pub const MI_CYBORG_HEAD: C2RustUnnamed_0 = 8;
pub const MI_BABA_BODY: C2RustUnnamed_0 = 7;
pub const MI_BABA_ARM: C2RustUnnamed_0 = 6;
pub const MI_BABA_LEGS: C2RustUnnamed_0 = 5;
pub const MI_BABA_HEAD: C2RustUnnamed_0 = 4;
pub const MI_SMALL_SMOKE: C2RustUnnamed_0 = 3;
pub const MI_CONSTRUCTION: C2RustUnnamed_0 = 2;
pub const MI_EXPLOSION_MEDIUM: C2RustUnnamed_0 = 1;
pub const MI_EXPLOSION_SMALL: C2RustUnnamed_0 = 0;
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
pub const ID_SOUND_NEXUS_RESEARCH_ABSORBED: C2RustUnnamed_1 = 341;
pub const ID_SOUND_NEXUS_PRODUCTION_COMPLETED: C2RustUnnamed_1 = 340;
pub const ID_SOUND_NEXUS_LAUGH3: C2RustUnnamed_1 = 339;
pub const ID_SOUND_NEXUS_LAUGH2: C2RustUnnamed_1 = 338;
pub const ID_SOUND_NEXUS_LAUGH1: C2RustUnnamed_1 = 337;
pub const ID_SOUND_NEXUS_DEFENCES_NEUTRALISED: C2RustUnnamed_1 = 336;
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
pub const NO_SOUND: C2RustUnnamed_1 = -1;
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
pub const CALL_MISSION_TIME: _scr_callback_types = 19;
pub const CALL_TRANSPORTER_REINFORCE: _scr_callback_types = 18;
pub const CALL_START_NEXT_LEVEL: _scr_callback_types = 17;
pub const CALL_LAUNCH_TRANSPORTER: _scr_callback_types = 16;
pub const CALL_VIDEO_QUIT: _scr_callback_types = 15;
pub const CALL_MISSION_END: _scr_callback_types = 14;
pub const CALL_MISSION_START: _scr_callback_types = 13;
pub const CALL_DROIDDESIGNED: _scr_callback_types = 7;
pub const CALL_DELIVPOINTMOVED: _scr_callback_types = 6;
pub const CALL_GAMEINIT: _scr_callback_types = 5;
// number of units using the formation
// maximum length of the lines
// seperation between the ranks
// direction of the formation
// position of the front of the formation
// the lines that make up a formation
// the units that have a position allocated in the formation
// formation speed (currently speed of slowest member) - GJ - sorry.
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
pub const STR_GAM_JOINING: _fixed_str_id = 236;
pub const STR_GAM_GAMEOVER: _fixed_str_id = 235;
pub const STR_GAM_REINF: _fixed_str_id = 234;
pub const STR_GAM_UNITSEL: _fixed_str_id = 233;
pub const STR_GAM_RESREWARD: _fixed_str_id = 232;
pub const STR_GAM_NOHQ: _fixed_str_id = 231;
pub const STR_GAM_GOHQ: _fixed_str_id = 230;
pub const STR_GAM_STRREST: _fixed_str_id = 229;
pub const STR_GAM_STRDAM: _fixed_str_id = 227;
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
pub type _group_type = libc::c_uint;
pub const GT_TRANSPORTER: _group_type = 2;
pub const GT_NORMAL: _group_type = 0;
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
/*sets the tile height */
#[inline]
unsafe extern "C" fn setTileHeight(mut x: UDWORD, mut y: UDWORD,
                                   mut height: UDWORD) {
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
              308 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"setTileHeight\x00")).as_ptr(),
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
              310 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"setTileHeight\x00")).as_ptr(),
              b"y < mapHeight\x00" as *const u8 as *const libc::c_char);
    };
    //psMapTiles[x + (y << mapShift)].height = height;//width no longer a power of 2
    (*psMapTiles.offset(x.wrapping_add(y.wrapping_mul(mapWidth)) as
                            isize)).height =
        height.wrapping_div(2 as libc::c_int as libc::c_uint) as UBYTE;
}
/* Return whether a world coordinate is on the map */
#[inline]
unsafe extern "C" fn worldOnMap(mut x: SDWORD, mut y: SDWORD) -> BOOL {
    return (x >= 0 as libc::c_int &&
                x < (mapWidth as SDWORD) << 7 as libc::c_int &&
                y >= 0 as libc::c_int &&
                y < (mapHeight as SDWORD) << 7 as libc::c_int) as libc::c_int;
}
//holds the IMD pointers for the modules - ONLY VALID for player0
#[no_mangle]
pub static mut factoryModuleIMDs: [[*mut iIMDShape; 2]; 2] =
    [[0 as *const iIMDShape as *mut iIMDShape; 2]; 2];
#[no_mangle]
pub static mut researchModuleIMDs: [*mut iIMDShape; 4] =
    [0 as *const iIMDShape as *mut iIMDShape; 4];
#[no_mangle]
pub static mut powerModuleIMDs: [*mut iIMDShape; 4] =
    [0 as *const iIMDShape as *mut iIMDShape; 4];
//Value is stored for easy access to this structure stat
#[no_mangle]
pub static mut factoryModuleStat: UDWORD = 0;
#[no_mangle]
pub static mut powerModuleStat: UDWORD = 0;
#[no_mangle]
pub static mut researchModuleStat: UDWORD = 0;
//holder for all StructureStats
#[no_mangle]
pub static mut asStructureStats: *mut STRUCTURE_STATS =
    0 as *const STRUCTURE_STATS as *mut STRUCTURE_STATS;
#[no_mangle]
pub static mut numStructureStats: UDWORD = 0;
//holder for the limits of each structure per map
#[no_mangle]
pub static mut asStructLimits: [*mut STRUCTURE_LIMITS; 8] =
    [0 as *const STRUCTURE_LIMITS as *mut STRUCTURE_LIMITS; 8];
//holds the upgrades attained through research for structure stats
#[no_mangle]
pub static mut asStructureUpgrade: [STRUCTURE_UPGRADE; 8] =
    [STRUCTURE_UPGRADE{armour: 0, body: 0, resistance: 0,}; 8];
#[no_mangle]
pub static mut asWallDefenceUpgrade: [WALLDEFENCE_UPGRADE; 8] =
    [WALLDEFENCE_UPGRADE{armour: 0, body: 0,}; 8];
//holds the upgrades for the functionality of structures through research
#[no_mangle]
pub static mut asResearchUpgrade: [RESEARCH_UPGRADE; 8] =
    [RESEARCH_UPGRADE{modifier: 0,}; 8];
#[no_mangle]
pub static mut asPowerUpgrade: [POWER_UPGRADE; 8] =
    [RESEARCH_UPGRADE{modifier: 0,}; 8];
#[no_mangle]
pub static mut asRepairFacUpgrade: [REPAIR_FACILITY_UPGRADE; 8] =
    [RESEARCH_UPGRADE{modifier: 0,}; 8];
#[no_mangle]
pub static mut asProductionUpgrade: [[PRODUCTION_UPGRADE; 3]; 8] =
    [[RESEARCH_UPGRADE{modifier: 0,}; 3]; 8];
#[no_mangle]
pub static mut asReArmUpgrade: [REARM_UPGRADE; 8] =
    [RESEARCH_UPGRADE{modifier: 0,}; 8];
//used to hold the modifiers cross refd by weapon effect and structureStrength
#[no_mangle]
pub static mut asStructStrengthModifier: [[STRUCTSTRENGTH_MODIFIER; 4]; 6] =
    [[0; 4]; 6];
// //the three different types of factory (currently) - FACTORY, CYBORG_FACTORY, VTOL_FACTORY
//#define NUM_FACTORY_TYPES	3
//#define FACTORY_FLAG		0
//#define CYBORG_FLAG		1
//#define VTOL_FLAG			2
//specifies which numbers have been allocated for the assembly points for the factories
//UBYTE				factoryNumFlag[MAX_PLAYERS][NUM_FACTORY_TYPES];
#[no_mangle]
pub static mut factoryNumFlag: [[UBYTE; 4]; 8] = [[0; 4]; 8];
//the list of what to build - only for selectedPlayer
#[no_mangle]
pub static mut asProductionRun: [[[PRODUCTION_RUN; 20]; 5]; 3] =
    [[[PRODUCTION_RUN{quantity: 0,
                      built: 0,
                      psTemplate:
                          0 as *const _droid_template as
                              *mut _droid_template,}; 20]; 5]; 3];
//stores which player the production list has been set up for
#[no_mangle]
pub static mut productionPlayer: SBYTE = 0;
/* destroy building construction droid stat pointer */
static mut g_psStatDestroyStruct: *mut STRUCTURE_STATS =
    0 as *const STRUCTURE_STATS as *mut STRUCTURE_STATS;
// store the last time a structure was hit for a side
// this controls when the CALL_STRUCT_ATTACKED is made
//UDWORD	aLastStructHit[MAX_PLAYERS];
// the structure that was last hit
#[no_mangle]
pub static mut psLastStructHit: *mut STRUCTURE =
    0 as *const STRUCTURE as *mut STRUCTURE;
//flag for oil derrick anim
//static	UBYTE	powerGenExists[MAX_PLAYERS];
//flag for drawing radar
static mut hqExists: [UBYTE; 8] = [0; 8];
//flag for drawing all sat uplink sees
static mut satUplinkExists: [UBYTE; 8] = [0; 8];
//flag for when the player has one built - either completely or partially
static mut lasSatExists: [UBYTE; 8] = [0; 8];
// last time the maximum units message was displayed
static mut lastMaxUnitMessage: UDWORD = 0;
//used to flag when the Factory is ready to start building
//distance that VTOLs can be away from the reArm pad
//Value is stored for easy access to this structure stat
// the structure that was last hit
//stores which player the production list has been set up for
//holder for all StructureStats
//holds the upgrades attained through research for structure stats
//holds the upgrades for the functionality of structures through research
//used to hold the modifiers cross refd by weapon effect and structureStrength
/*Load the Structure Strength Modifiers from the file exported from Access*/
/* Set the type of droid for a factory to build */
//temp test function for creating structures at the start of the game
// Set the tile no draw flags for a structure
//builds a specified structure at a given location
/* The main update routine for all Structures */
/* Release all resources associated with a structure */
/* Remove a structure and free it's memory */
// remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
//fills the list with Structures that can be built
/* checks that the location is a valid one to build on and sets the outline colour
x and y in tile-coords*/
/* for a new structure, find a location along an edge which the droid can get
to and return this as the destination for the droid */
//extern BOOL getDroidDestination(STRUCTURE_STATS *psPositionStats, UDWORD structX, 
//	UDWORD structY, UDWORD * pDroidX, UDWORD *pDroidY);
/*for a structure or feature, find a location along an edge which the droid can get
to and return this as the destination for the droid*/
/* check along the width of a structure for an empty space */
/* check along the length of a structure for an empty space */
//initialise the structure limits structure
/* set the current number of structures of each type built */
/* get a stat inc based on the name */
/*check to see if the structure is 'doing' anything  - return TRUE if idle*/
/*checks to see if any structure exists of a specified type with a specified status */
/*sets the point new droids go to - x/y in world coords for a Factory*/
//extern void createAssemblyPoint(STRUCTURE* psStruct);
/* consider delivery points when selected by player*/
/*called when a structure has been built - checks through the list of callbacks 
for the scripts*/
/*initialises the flag before a new data set is loaded up*/
//called at start of missions
/* get demolish stat */
/*find a location near to the factory to start the droid of*/
/*sets the flag to indicate a Power Generator Exists - so do Oil Derrick anim*/
//extern void setPowerGenExists(BOOL state, UDWORD player);
/*returns teh status of the flag*/
//extern BOOL getPowerGenExists(UDWORD player);
/*sets the flag to indicate a HQ Exists - so draw Radar*/
/*returns the status of the flag*/
/*sets the flag to indicate a SatUplink Exists - so draw everything!*/
/*returns the status of the flag*/
/*sets the flag to indicate a Las Sat Exists - ONLY EVER WANT ONE*/
/*returns the status of the flag*/
/*this is called whenever a structure has finished building*/
// these functions are used in game.c inplace of  building complete
/*Looks through the players list of structures to see if a HQ exists - will look
through the list of structures at Home Base when on an offWorld mission map*/
// Set the command droid that factory production should go to
//struct _command_droid;
// remove all factories from a command droid
/*for a given structure, return a pointer to its module stat */
/*called when a Res extractor is destroyed or runs out of power or is disconnected
adjusts the owning Power Gen so that it can link to a different Res Extractor if one
is available*/
/*called when a Power Gen is destroyed or is disconnected
adjusts the associated Res Extractors so that they can link to different Power 
Gens if any are available*/
//print some info at the top of the screen dependant on the structure
/*Checks the template type against the factory type - returns FALSE 
if not a good combination!*/
/*calculates the damage caused to the resistance levels of structures*/
//extern BOOL electronicDamage(STRUCTURE *psStructure, UDWORD damage, UBYTE attackPlayer);
//electronic damage can be targetted at droids as well as structures now - AB 5/11/98
/* EW works differently in multiplayer mode compared with single player.*/
/*checks to see if a specific structure type exists -as opposed to a structure 
stat type*/
/*Access functions for the upgradeable stats of a structure*/
/*this returns the Base Body points of a structure - regardless of upgrade*/
// Is a structure a factory of somekind?
// Is a flag a factory delivery point?
// Find a factories corresonding delivery point.
//Find the factory associated with the delivery point - returns NULL if none exist
/*this is called when a factory produces a droid. The Template returned is the next 
one to build - if any*/
//increment the production run for this type
//returns the quantity of a specific template in the production list
/*returns the quantity of a specific template in the production list that 
have already been built*/
//looks through a players production list to see if a command droid is being built
//check that delivery points haven't been put down in invalid location
//adjust the loop quantity for this factory
/*cancels the production run for the factory and returns any power that was 
accrued but not used*/
/*set a factory's production run to hold*/
/*release a factory's production run from hold*/
/*This function is called after a game is loaded so that any resource extractors 
that are active are initialised for when to start*/
// Count number of factories assignable to a command droid.
/*Used for determining how much of the structure to draw as being built or demolished*/
/*compares the structure sensor type with the droid weapon type to see if the 
FIRE_SUPPORT order can be assigned*/
/*checks if the structure has a Counter Battery sensor attached - returns 
TRUE if it has*/
/*checks if the structure has a Standard Turret sensor attached - returns 
TRUE if it has*/
/*checks if the structure has a VTOL Intercept sensor attached - returns 
TRUE if it has*/
/*checks if the structure has a VTOL Counter Battery sensor attached - returns 
TRUE if it has*/
// return the nearest rearm pad
// if bClear is true it tries to find the nearest clear rearm pad in
// the same cluster as psTarget
// psTarget can be NULL
// check whether a rearm pad is clear
// clear a rearm pad for a vtol to land on it
// return whether a rearm pad has a vtol on it
/* Just returns true if the structure's present body points aren't as high as the original*/
// give a structure from one player to another - used in Electronic Warfare
/*Initialise the production list and set up the production player*/
// La!
/*checks that the structure stats have loaded up as expected - must be done after 
all StructureStats parts have been loaded*/
/*returns the power cost to build this structure*/
// check whether a factory of a certain number and type exists
// remove a structure from a game without any visible effects
// (for example used to change the type of wall at a location)
//BOOL removeStruct(STRUCTURE *psDel);
//#ifdef DEMO
//BOOL demoStructs(void);
//BOOL createStructureStat(STRUCTURE_STATS *psBuilding, STRUCTURE_STATS *psNewStructure,
//						 UDWORD ref, STRING *pName, UDWORD type);
//void printAvailStructs(void);
//#endif
/* New function from Alex M */
/* Tells you if a point is inside the footprint of a building */
#[no_mangle]
pub unsafe extern "C" fn ptInStructure(mut psStruct: *mut STRUCTURE,
                                       mut x: UDWORD, mut y: UDWORD) -> BOOL {
    let mut tlX: UDWORD = 0;
    let mut tlY: UDWORD = 0;
    let mut brX: UDWORD = 0;
    let mut brY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut height: UDWORD = 0;
    width =
        (*(*psStruct).pStructureType).baseWidth.wrapping_mul(128 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint);
    height =
        (*(*psStruct).pStructureType).baseBreadth.wrapping_mul(128 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
    tlX =
        ((*psStruct).x as
             libc::c_uint).wrapping_sub(width.wrapping_div(2 as libc::c_int as
                                                               libc::c_uint));
    tlY =
        ((*psStruct).y as
             libc::c_uint).wrapping_sub(height.wrapping_div(2 as libc::c_int
                                                                as
                                                                libc::c_uint));
    brX =
        ((*psStruct).x as
             libc::c_uint).wrapping_add(width.wrapping_div(2 as libc::c_int as
                                                               libc::c_uint));
    brY =
        ((*psStruct).y as
             libc::c_uint).wrapping_add(height.wrapping_div(2 as libc::c_int
                                                                as
                                                                libc::c_uint));
    if x > tlX && x < brX {
        if y > tlY && y < brY { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
/*
Check to see if the stats is some kind of expansion module

... this replaces the thousands of occurance that is spread through out the code

 ... There were a couple of places where it skipping around a routine if the stat was a expansion module
	(loadSaveStructureV7 & 9) this code seemed suspect, and to clarify it we replaced it with the routine below
... the module stuff seemed to work though ...     TJC (& AB) 8-DEC-98
*/
#[no_mangle]
pub unsafe extern "C" fn IsStatExpansionModule(mut psStats:
                                                   *mut STRUCTURE_STATS)
 -> BOOL {
    // If the stat is any of the 3 expansion types ... then return TRUE
    if (*psStats).type_0 == REF_POWER_MODULE as libc::c_int as libc::c_uint ||
           (*psStats).type_0 ==
               REF_FACTORY_MODULE as libc::c_int as libc::c_uint ||
           (*psStats).type_0 ==
               REF_RESEARCH_MODULE as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn structureInitVars() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        factoryModuleIMDs[i as usize][0 as libc::c_int as usize] =
            0 as *mut iIMDShape;
        factoryModuleIMDs[i as usize][1 as libc::c_int as usize] =
            0 as *mut iIMDShape;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        researchModuleIMDs[i as usize] = 0 as *mut iIMDShape;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        powerModuleIMDs[i as usize] = 0 as *mut iIMDShape;
        i += 1
    }
    asStructureStats = 0 as *mut STRUCTURE_STATS;
    numStructureStats = 0 as libc::c_int as UDWORD;
    factoryModuleStat = 0 as libc::c_int as UDWORD;
    powerModuleStat = 0 as libc::c_int as UDWORD;
    researchModuleStat = 0 as libc::c_int as UDWORD;
    lastMaxUnitMessage = 0 as libc::c_int as UDWORD;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        asStructLimits[i as usize] = 0 as *mut STRUCTURE_LIMITS;
        //		aLastStructHit[i] = 0;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            factoryNumFlag[i as usize][j as usize] =
                0 as libc::c_int as UBYTE;
            j += 1
        }
        i += 1
    }
    //for (j=0; j < NUM_FACTORY_TYPES; j++)
    /*for (i = 0; i < MAX_PLAYERS; i++)
	{
		powerGenExists[i] = FALSE;
	}*/
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        hqExists[i as usize] = 0 as libc::c_int as UBYTE;
        satUplinkExists[i as usize] = 0 as libc::c_int as UBYTE;
        lasSatExists[i as usize] = 0 as libc::c_int as UBYTE;
        i += 1
    }
    //initialise the selectedPlayer's production run
    memset(&mut asProductionRun as *mut [[[PRODUCTION_RUN; 20]; 5]; 3] as
               *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<PRODUCTION_RUN>() as
                libc::c_ulong).wrapping_mul(3 as libc::c_int as
                                                libc::c_uint).wrapping_mul(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(20
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint));
    //set up at beginning of game which player will have a production list
    productionPlayer = selectedPlayer as SBYTE;
}
/*Initialise the production list and set up the production player*/
#[no_mangle]
pub unsafe extern "C" fn changeProductionPlayer(mut player: UBYTE) {
    //clear the production run
    memset(&mut asProductionRun as *mut [[[PRODUCTION_RUN; 20]; 5]; 3] as
               *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<PRODUCTION_RUN>() as
                libc::c_ulong).wrapping_mul(3 as libc::c_int as
                                                libc::c_uint).wrapping_mul(5
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_mul(20
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint));
    //set this player to have the production list
    productionPlayer = player as SBYTE;
}
/*initialises the flag before a new data set is loaded up*/
#[no_mangle]
pub unsafe extern "C" fn initFactoryNumFlag() {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //initialise the flag
		//for (j=0; j < NUM_FACTORY_TYPES; j++)
        j = 0 as libc::c_int as UDWORD;
        while j < 4 as libc::c_int as libc::c_uint {
            factoryNumFlag[i as usize][j as usize] =
                0 as libc::c_int as UBYTE;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
//called at start of missions
#[no_mangle]
pub unsafe extern "C" fn resetFactoryNumFlag() {
    let mut i: UDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut mask: UBYTE = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        //initialise the flag
		//factoryNumFlag[i] = (UBYTE)0;
        //look throu the list of structures to see which have been used
        psStruct = apsStructLists[i as usize];
        while !psStruct.is_null() {
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_FACTORY as libc::c_int as libc::c_uint {
                psFactory = (*psStruct).pFunctionality as *mut FACTORY;
                if !(*psFactory).psAssemblyPoint.is_null() {
                    mask =
                        ((1 as libc::c_int) <<
                             (*(*psFactory).psAssemblyPoint).factoryInc as
                                 libc::c_int) as UBYTE;
                    factoryNumFlag[i as usize][0 as libc::c_int as usize] =
                        (factoryNumFlag[i as usize][0 as libc::c_int as usize]
                             as libc::c_int | mask as libc::c_int) as UBYTE
                }
            }
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint {
                psFactory = (*psStruct).pFunctionality as *mut FACTORY;
                if !(*psFactory).psAssemblyPoint.is_null() {
                    mask =
                        ((1 as libc::c_int) <<
                             (*(*psFactory).psAssemblyPoint).factoryInc as
                                 libc::c_int) as UBYTE;
                    factoryNumFlag[i as usize][1 as libc::c_int as usize] =
                        (factoryNumFlag[i as usize][1 as libc::c_int as usize]
                             as libc::c_int | mask as libc::c_int) as UBYTE
                }
            }
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
                psFactory = (*psStruct).pFunctionality as *mut FACTORY;
                if !(*psFactory).psAssemblyPoint.is_null() {
                    mask =
                        ((1 as libc::c_int) <<
                             (*(*psFactory).psAssemblyPoint).factoryInc as
                                 libc::c_int) as UBYTE;
                    factoryNumFlag[i as usize][2 as libc::c_int as usize] =
                        (factoryNumFlag[i as usize][2 as libc::c_int as usize]
                             as libc::c_int | mask as libc::c_int) as UBYTE
                }
            }
            psStruct = (*psStruct).psNext
        }
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureType(mut pStructure: *mut STRUCTURE_STATS,
                                       mut pType: *mut libc::c_char) {
    if strcmp(pType, b"HQ\x00" as *const u8 as *const libc::c_char) == 0 {
        (*pStructure).type_0 = REF_HQ as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"FACTORY\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_FACTORY as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"FACTORY MODULE\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_FACTORY_MODULE as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"RESEARCH\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_RESEARCH as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"RESEARCH MODULE\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_RESEARCH_MODULE as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"POWER GENERATOR\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_POWER_GEN as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"POWER MODULE\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 = REF_POWER_MODULE as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"RESOURCE EXTRACTOR\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 =
            REF_RESOURCE_EXTRACTOR as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"DEFENSE\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_DEFENSE as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"WALL\x00" as *const u8 as *const libc::c_char) == 0 {
        (*pStructure).type_0 = REF_WALL as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"CORNER WALL\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 = REF_WALLCORNER as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"REPAIR FACILITY\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_REPAIR_FACILITY as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"COMMAND RELAY\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 = REF_COMMAND_CONTROL as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"DEMOLISH\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_DEMOLISH as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType,
              b"CYBORG FACTORY\x00" as *const u8 as *const libc::c_char) == 0
       {
        (*pStructure).type_0 = REF_CYBORG_FACTORY as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"VTOL FACTORY\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 = REF_VTOL_FACTORY as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"LAB\x00" as *const u8 as *const libc::c_char) == 0 {
        (*pStructure).type_0 = REF_LAB as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"DOOR\x00" as *const u8 as *const libc::c_char) == 0 {
        (*pStructure).type_0 = REF_BLASTDOOR as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"REARM PAD\x00" as *const u8 as *const libc::c_char) ==
           0 {
        (*pStructure).type_0 = REF_REARM_PAD as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"MISSILE SILO\x00" as *const u8 as *const libc::c_char)
           == 0 {
        (*pStructure).type_0 = REF_MISSILE_SILO as libc::c_int as UDWORD;
        return
    }
    if strcmp(pType, b"SAT UPLINK\x00" as *const u8 as *const libc::c_char) ==
           0 {
        (*pStructure).type_0 = REF_SAT_UPLINK as libc::c_int as UDWORD;
        return
    }
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Unknown Structure Type\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              492 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"structureType\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn getStructName(mut psStruct: *mut STRUCTURE_STATS)
 -> *mut libc::c_char {
    return getName((*psStruct).pName);
}
/*returns the structure strength based on the string name passed in */
#[no_mangle]
pub unsafe extern "C" fn getStructStrength(mut pStrength: *mut STRING)
 -> UBYTE {
    if strcmp(pStrength, b"SOFT\x00" as *const u8 as *const libc::c_char) == 0
       {
        return STRENGTH_SOFT as libc::c_int as UBYTE
    } else if strcmp(pStrength,
                     b"MEDIUM\x00" as *const u8 as *const libc::c_char) == 0 {
        return STRENGTH_MEDIUM as libc::c_int as UBYTE
    } else if strcmp(pStrength,
                     b"HARD\x00" as *const u8 as *const libc::c_char) == 0 {
        return STRENGTH_HARD as libc::c_int as UBYTE
    } else if strcmp(pStrength,
                     b"BUNKER\x00" as *const u8 as *const libc::c_char) == 0 {
        return STRENGTH_BUNKER as libc::c_int as UBYTE
    } else {
        return (NUM_STRUCT_STRENGTH as libc::c_int + 1 as libc::c_int) as
                   UBYTE
    };
}
#[no_mangle]
pub unsafe extern "C" fn initModulePIEs(mut PIEName: *mut libc::c_char,
                                        mut i: UDWORD,
                                        mut psStructure:
                                            *mut STRUCTURE_STATS) {
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut charNum: [STRING; 2] = [0; 2];
    let mut length: UDWORD = 0;
    let mut module: UDWORD = 0 as libc::c_int as UDWORD;
    strcpy(GfxFile.as_mut_ptr(), PIEName);
    //need to work out the IMD's for the modules - HACK!
    if (*psStructure).type_0 ==
           REF_FACTORY_MODULE as libc::c_int as libc::c_uint {
        length =
            strlen(GfxFile.as_mut_ptr()).wrapping_sub(5 as libc::c_int as
                                                          libc::c_uint);
        module = 1 as libc::c_int as UDWORD;
        while module < (2 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
            sprintf(charNum.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char, module);
            GfxFile[length as usize] = *charNum.as_mut_ptr();
            factoryModuleIMDs[module.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
                                  usize][0 as libc::c_int as usize] =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if factoryModuleIMDs[module.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint) as
                                     usize][0 as libc::c_int as
                                                usize].is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the PIE for factory module %d - %s\x00" as
                          *const u8 as *const libc::c_char, module,
                      GfxFile.as_mut_ptr());
                abort();
                // FALSE;
            }
            module = module.wrapping_add(1)
        }
        //store the stat for easy access later on
        factoryModuleStat = i
    }
    if (*psStructure).type_0 ==
           REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        length =
            strlen(GfxFile.as_mut_ptr()).wrapping_sub(5 as libc::c_int as
                                                          libc::c_uint);
        module = 1 as libc::c_int as UDWORD;
        while module < (2 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
            sprintf(charNum.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char, module);
            GfxFile[length as usize] = *charNum.as_mut_ptr();
            factoryModuleIMDs[module.wrapping_sub(1 as libc::c_int as
                                                      libc::c_uint) as
                                  usize][1 as libc::c_int as usize] =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, GfxFile.as_mut_ptr()) as
                    *mut iIMDShape;
            if factoryModuleIMDs[module.wrapping_sub(1 as libc::c_int as
                                                         libc::c_uint) as
                                     usize][1 as libc::c_int as
                                                usize].is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the PIE for vtol factory module %d - %s\x00"
                          as *const u8 as *const libc::c_char, module,
                      GfxFile.as_mut_ptr());
                abort();
                // FALSE;
            }
            module = module.wrapping_add(1)
        }
    }
    // Setup the PIE's for the research modules.
    if (*psStructure).type_0 ==
           REF_RESEARCH_MODULE as libc::c_int as libc::c_uint {
        length =
            strlen(GfxFile.as_mut_ptr()).wrapping_sub(5 as libc::c_int as
                                                          libc::c_uint);
        GfxFile[length as usize] = '4' as i32 as STRING;
        researchModuleIMDs[0 as libc::c_int as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, GfxFile.as_mut_ptr()) as
                *mut iIMDShape;
        if researchModuleIMDs[0 as libc::c_int as usize].is_null() {
            debug(LOG_ERROR,
                  b"Cannot find the PIE for research module %d - %s\x00" as
                      *const u8 as *const libc::c_char, module,
                  GfxFile.as_mut_ptr());
            abort();
            // FALSE;
        }
        researchModuleIMDs[1 as libc::c_int as usize] =
            researchModuleIMDs[0 as libc::c_int as usize];
        researchModuleIMDs[2 as libc::c_int as usize] =
            researchModuleIMDs[0 as libc::c_int as usize];
        researchModuleIMDs[3 as libc::c_int as usize] =
            researchModuleIMDs[0 as libc::c_int as usize];
        //store the stat for easy access later on
        researchModuleStat = i
    }
    // Setup the PIE's for the power modules.
    if (*psStructure).type_0 ==
           REF_POWER_MODULE as libc::c_int as libc::c_uint {
        length =
            strlen(GfxFile.as_mut_ptr()).wrapping_sub(5 as libc::c_int as
                                                          libc::c_uint);
        GfxFile[length as usize] = '4' as i32 as STRING;
        powerModuleIMDs[0 as libc::c_int as usize] =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, GfxFile.as_mut_ptr()) as
                *mut iIMDShape;
        if powerModuleIMDs[0 as libc::c_int as usize].is_null() {
            debug(LOG_ERROR,
                  b"Cannot find the PIE for power module %d - %s\x00" as
                      *const u8 as *const libc::c_char, module,
                  GfxFile.as_mut_ptr());
            abort();
            // FALSE;
        }
        powerModuleIMDs[1 as libc::c_int as usize] =
            powerModuleIMDs[0 as libc::c_int as usize];
        powerModuleIMDs[2 as libc::c_int as usize] =
            powerModuleIMDs[0 as libc::c_int as usize];
        powerModuleIMDs[3 as libc::c_int as usize] =
            powerModuleIMDs[0 as libc::c_int as usize];
        //store the stat for easy access later on
        powerModuleStat = i
    };
}
/* load the Structure stats from the Access database */
#[no_mangle]
pub unsafe extern "C" fn loadStructureStats(mut pStructData:
                                                *mut libc::c_char,
                                            mut bufferSize: UDWORD) -> BOOL {
    let mut pData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumStructures: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut numWeaps: UDWORD = 0;
    let mut weapSlots: UDWORD = 0;
    let mut StructureName: [STRING; 60] = [0; 60];
    let mut foundation: [STRING; 60] = [0; 60];
    let mut type_0: [STRING; 60] = [0; 60];
    let mut techLevel: [STRING; 60] = [0; 60];
    let mut strength: [STRING; 60] = [0; 60];
    let mut GfxFile: [STRING; 60] = [0; 60];
    let mut baseIMD: [STRING; 60] = [0; 60];
    let mut ecmType: [STRING; 60] = [0; 60];
    let mut sensorType: [STRING; 60] = [0; 60];
    let mut psStructure: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut pStartStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut pECMType: *mut ECM_STATS = 0 as *mut ECM_STATS;
    let mut pSensorType: *mut SENSOR_STATS = 0 as *mut SENSOR_STATS;
    let mut module: UDWORD = 0;
    //UDWORD				length, module;
	//STRING				charNum[2];
    let mut iID: UDWORD = 0;
    let mut dummyVal: UDWORD = 0;
    //initialise the module IMD structs
    module = 0 as libc::c_int as UDWORD;
    while module < 2 as libc::c_int as libc::c_uint {
        factoryModuleIMDs[module as usize][0 as libc::c_int as usize] =
            0 as *mut iIMDShape;
        factoryModuleIMDs[module as usize][1 as libc::c_int as usize] =
            0 as *mut iIMDShape;
        module = module.wrapping_add(1)
    }
    module = 0 as libc::c_int as UDWORD;
    while module < 4 as libc::c_int as libc::c_uint {
        researchModuleIMDs[module as usize] = 0 as *mut iIMDShape;
        module = module.wrapping_add(1)
    }
    module = 0 as libc::c_int as UDWORD;
    while module < 4 as libc::c_int as libc::c_uint {
        powerModuleIMDs[module as usize] = 0 as *mut iIMDShape;
        module = module.wrapping_add(1)
    }
    //keep the start so we release it at the end
    pData = pStructData;
    NumStructures = numCR(pStructData, bufferSize);
    //#ifdef DEMO
//	asStructureStats = (STRUCTURE_STATS *)MALLOC(sizeof(STRUCTURE_STATS)*
//		(NumStructures + NUM_DEMO_STRUCTS));
//	//numStructureStats is added to in in demoStructs()
//#else
    asStructureStats =
        memMallocRelease((::std::mem::size_of::<STRUCTURE_STATS>() as
                              libc::c_ulong).wrapping_mul(NumStructures)) as
            *mut STRUCTURE_STATS;
    //#endif
    numStructureStats = NumStructures;
    if asStructureStats.is_null() {
        debug(LOG_ERROR,
              b"Structure Stats - Out of memory\x00" as *const u8 as
                  *const libc::c_char);
        abort();
    }
    //save the starting address
    pStartStats = asStructureStats;
    //get the start of the structure_stats storage
    psStructure = asStructureStats;
    i = 0 as libc::c_int as UDWORD;
    while i < NumStructures {
        memset(psStructure as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<STRUCTURE_STATS>() as libc::c_ulong);
        //read the data into the storage - the data is delimeted using comma's
        GfxFile[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        StructureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        type_0[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        strength[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        foundation[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        ecmType[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sensorType[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        baseIMD[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pStructData,
               b"%[^\',\'],%[^\',\'],%[^\',\'],%[^\',\'],%d,%d,%d,%[^\',\'],\t\t\t%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%[^\',\'],%[^\',\'],%d,%[^\',\'],%[^\',\'],\t\t\t%d,%d\x00"
                   as *const u8 as *const libc::c_char,
               StructureName.as_mut_ptr(), type_0.as_mut_ptr(),
               techLevel.as_mut_ptr(), strength.as_mut_ptr(),
               &mut (*psStructure).terrainType as *mut UDWORD,
               &mut (*psStructure).baseWidth as *mut UDWORD,
               &mut (*psStructure).baseBreadth as *mut UDWORD,
               foundation.as_mut_ptr(),
               &mut (*psStructure).buildPoints as *mut UDWORD,
               &mut (*psStructure).height as *mut UDWORD,
               &mut (*psStructure).armourValue as *mut UDWORD,
               &mut (*psStructure).bodyPoints as *mut UDWORD,
               &mut (*psStructure).repairSystem as *mut UDWORD,
               &mut (*psStructure).powerToBuild as *mut UDWORD,
               &mut dummyVal as *mut UDWORD,
               &mut (*psStructure).resistance as *mut UDWORD,
               &mut dummyVal as *mut UDWORD,
               &mut (*psStructure).sizeModifier as *mut UDWORD,
               ecmType.as_mut_ptr(), sensorType.as_mut_ptr(),
               &mut weapSlots as *mut UDWORD, GfxFile.as_mut_ptr(),
               baseIMD.as_mut_ptr(),
               &mut (*psStructure).numFuncs as *mut UDWORD,
               &mut numWeaps as *mut UDWORD);
        //		DBPRINTF(("%s: height %d\n", StructureName, psStructure->height));
        //#ifdef FILTER_WALLS
//		if(strcmp(type,"WALL") == 0) {
//			DBPRINTF(("Filtered out WALL : %s\n",StructureName));
//			pStructData = strchr(pStructData,'\n') + 1;
//			continue;
//		}
//		if(strcmp(type,"CORNER WALL") == 0) {
//			DBPRINTF(("Filtered out CORNER WALL : %s\n",StructureName));
//			pStructData = strchr(pStructData,'\n') + 1;
//			continue;
//		}
//#endif
        //allocate storage for the name
 		/*psStructure->pName = (STRING *)MALLOC((strlen(StructureName))+1);
		if (psStructure->pName == NULL)
		{
			DBERROR(("Structure Stats Name - Out of memory"));
			return FALSE;
		}
		strcpy(psStructure->pName,StructureName);*/
        if allocateName(&mut (*psStructure).pName, StructureName.as_mut_ptr())
               == 0 {
            return 0 as libc::c_int
        }
        (*psStructure).ref_0 =
            (0xd0000 as libc::c_int as libc::c_uint).wrapping_add(i);
        //determine the structure type
        structureType(psStructure, type_0.as_mut_ptr());
        //determine the tech level
        if setTechLevel(psStructure as *mut BASE_STATS,
                        techLevel.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        //set the struct strength
        (*psStructure).strength =
            getStructStrength(strength.as_mut_ptr()) as
                STRUCT_STRENGTH; //->pName));
        if (*psStructure).strength as libc::c_uint ==
               (NUM_STRUCT_STRENGTH as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadStructureStats: Unknown structure strength for %s\x00"
                      as *const u8 as *const libc::c_char,
                  getStatName(psStructure as *mut libc::c_void));
            abort();
        }
        //get the ecm stats pointer
        if strcmp(ecmType.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStructure).pECM = 0 as *mut _ecm_stats
        } else {
            pECMType = asECMStats;
            if getResourceName(ecmType.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            inc = 0 as libc::c_int as UDWORD;
            while inc < numECMStats {
                //compare the names
                if strcmp(ecmType.as_mut_ptr(), (*pECMType).pName) == 0 {
                    (*psStructure).pECM = pECMType;
                    break ;
                } else {
                    pECMType = pECMType.offset(1);
                    inc = inc.wrapping_add(1)
                }
            }
        }
        //get the sensor stats pointer
        if strcmp(sensorType.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) == 0 {
            (*psStructure).pSensor = 0 as *mut _sensor_stats
        } else {
            if getResourceName(sensorType.as_mut_ptr()) == 0 {
                return 0 as libc::c_int
            }
            pSensorType = asSensorStats;
            inc = 0 as libc::c_int as UDWORD;
            while inc < numSensorStats {
                //compare the names
                if strcmp(sensorType.as_mut_ptr(), (*pSensorType).pName) == 0
                   {
                    (*psStructure).pSensor = pSensorType;
                    break ;
                } else {
                    pSensorType = pSensorType.offset(1);
                    inc = inc.wrapping_add(1)
                }
            }
            //check not allocating a turret sensor if have weapons attached
            if !(*psStructure).pSensor.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"loadStructureStats: should have a sensor attached to %s!\x00"
                          as *const u8 as *const libc::c_char,
                      StructureName.as_mut_ptr());
            };
            if !(*psStructure).pSensor.is_null() {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      968 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 19],
                                                &[libc::c_char; 19]>(b"loadStructureStats\x00")).as_ptr(),
                      b"psStructure->pSensor != NULL\x00" as *const u8 as
                          *const libc::c_char);
            };
            //if (psStructure->pSensor->location == LOC_TURRET AND psStructure->numWeaps)
            if (*(*psStructure).pSensor).location ==
                   LOC_TURRET as libc::c_int as libc::c_uint && numWeaps != 0
               {
                debug(LOG_ERROR,
                      b"loadStructureStats: a Turret Sensor and weapon \t\t\t\t\thave been assigned to %s\x00"
                          as *const u8 as *const libc::c_char,
                      StructureName.as_mut_ptr());
                abort();
            }
        }
        //get the IMD for the structure
        (*psStructure).pIMD =
            resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                           *mut STRING, GfxFile.as_mut_ptr()) as
                *mut iIMDShape;
        if (*psStructure).pIMD.is_null() {
            debug(LOG_ERROR,
                  b"Cannot find the structure PIE for record %s\x00" as
                      *const u8 as *const libc::c_char,
                  getStructName(psStructure));
            abort();
        }
        if strcmp(baseIMD.as_mut_ptr(),
                  b"0\x00" as *const u8 as *const libc::c_char) != 0 {
            (*psStructure).pBaseIMD =
                resGetData(b"IMD\x00" as *const u8 as *const libc::c_char as
                               *mut STRING, baseIMD.as_mut_ptr()) as
                    *mut iIMDShape;
            if (*psStructure).pIMD.is_null() {
                debug(LOG_ERROR,
                      b"Cannot find the structure base PIE for record %s\x00"
                          as *const u8 as *const libc::c_char,
                      getStructName(psStructure));
                abort();
            }
        } else { (*psStructure).pBaseIMD = 0 as *mut iIMDShape }
        initModulePIEs(GfxFile.as_mut_ptr(), i, psStructure);
        //allocate storage for the weapons - if any
		//psStructure->defaultWeap = -1;
		//check haven't allocated more than allowed
		//if (psStructure->numWeaps > psStructure->weaponSlots)
        //Only having one weapon per structure now...AB 24/01/99
        if weapSlots > 1 as libc::c_int as libc::c_uint ||
               numWeaps > weapSlots {
            //DBERROR(("Allocated more weapons than allowed in Access DB for Structure"));
            debug(LOG_ERROR,
                  b"Allocated more weapons than allowed for Structure\x00" as
                      *const u8 as *const libc::c_char);
            abort();
        }
        //Don't need to allocate space since thereis only one possible pointer now! AB 24/01/99
		//if (psStructure->numWeaps > 0)
        /*if (numWeaps > 0)
		{
			//psStructure->asWeapList = (WEAPON_STATS **)MALLOC(psStructure->weaponSlots*
			//	sizeof(WEAPON_STATS*));
			psStructure->asWeapList = (WEAPON_STATS **)MALLOC(numWeaps *
                sizeof(WEAPON_STATS*));
			if (psStructure->asWeapList == NULL)
			{
				DBERROR(("Out of memory assigning structure weapons"));
				return FALSE;
			}
		}*/
		//allocate storage for the functions - if any
        (*psStructure).defaultFunc = -(1 as libc::c_int);
        if (*psStructure).numFuncs > 0 as libc::c_int as libc::c_uint {
            (*psStructure).asFuncList =
                memMallocRelease((*psStructure).numFuncs.wrapping_mul(::std::mem::size_of::<*mut FUNCTION>()
                                                                          as
                                                                          libc::c_ulong))
                    as *mut *mut FUNCTION;
            if (*psStructure).asFuncList.is_null() {
                debug(LOG_ERROR,
                      b"Out of memory assigning structure Functions\x00" as
                          *const u8 as *const libc::c_char);
                abort();
            }
        }
        //increment the pointer to the start of the next record
        pStructData =
            strchr(pStructData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        //increment the list to the start of the next storage block
        psStructure = psStructure.offset(1);
        i = i.wrapping_add(1)
    }
    //	FREE(pData);
    asStructureStats = pStartStats;
    /* get global dummy stat pointer - GJ */
	/*strcpy(StructureName, "A00DemolishStructure");
	iID = getStructStatFromName( StructureName );*/
    iID = 0 as libc::c_int as UDWORD;
    while iID < numStructureStats {
        if (*asStructureStats.offset(iID as isize)).type_0 ==
               REF_DEMOLISH as libc::c_int as libc::c_uint {
            break ;
        }
        iID = iID.wrapping_add(1)
    }
    //if ( iID == -1 )
    if iID > numStructureStats {
        debug(LOG_ERROR,
              b"intAddObjectStats: destroy structure stat not found\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    g_psStatDestroyStruct = asStructureStats.offset(iID as isize);
    //#ifdef DEMO
//	if (!demoStructs())
//	{
//		return FALSE;
//	}
//#endif
    //allocate the structureLimits structure
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        asStructLimits[player as usize] =
            memMallocRelease((::std::mem::size_of::<STRUCTURE_LIMITS>() as
                                  libc::c_ulong).wrapping_mul(numStructureStats))
                as *mut STRUCTURE_LIMITS;
        if asStructLimits[player as usize].is_null() {
            debug(LOG_ERROR,
                  b"Unable to allocate structure limits\x00" as *const u8 as
                      *const libc::c_char);
            abort();
        }
        player = player.wrapping_add(1)
    }
    initStructLimits();
    //initialise the structure upgrade arrays
    memset(asStructureUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<STRUCTURE_UPGRADE>()
                                               as libc::c_ulong));
    memset(asWallDefenceUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<WALLDEFENCE_UPGRADE>()
                                               as libc::c_ulong));
    memset(asResearchUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<RESEARCH_UPGRADE>()
                                               as libc::c_ulong));
    memset(asPowerUpgrade.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<POWER_UPGRADE>()
                                               as libc::c_ulong));
    memset(asRepairFacUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<REPAIR_FACILITY_UPGRADE>()
                                               as libc::c_ulong));
    memset(asProductionUpgrade.as_mut_ptr() as *mut libc::c_void,
           0 as libc::c_int,
           ((8 as libc::c_int * 3 as libc::c_int) as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<PRODUCTION_UPGRADE>()
                                               as libc::c_ulong));
    memset(asReArmUpgrade.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           (8 as libc::c_int as
                libc::c_uint).wrapping_mul(::std::mem::size_of::<REARM_UPGRADE>()
                                               as libc::c_ulong));
    return 1 as libc::c_int;
}
//initialise the structure limits structure
#[no_mangle]
pub unsafe extern "C" fn initStructLimits() {
    let mut i: UDWORD = 0;
    let mut player: UDWORD = 0;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        psStructLimits = asStructLimits[player as usize];
        i = 0 as libc::c_int as UDWORD;
        while i < numStructureStats {
            (*psStructLimits.offset(i as isize)).limit =
                255 as libc::c_int as UBYTE;
            (*psStructLimits.offset(i as isize)).currentQuantity =
                0 as libc::c_int as UBYTE;
            (*psStructLimits.offset(i as isize)).globalLimit =
                255 as libc::c_int as UBYTE;
            i = i.wrapping_add(1)
        }
        player = player.wrapping_add(1)
    };
}
/* set the current number of structures of each type built */
#[no_mangle]
pub unsafe extern "C" fn setCurrentStructQuantity(mut displayError: BOOL) {
    let mut player: UDWORD = 0;
    let mut inc: UDWORD = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStructLimits: *mut STRUCTURE_LIMITS =
        0 as *mut STRUCTURE_LIMITS;
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        psStructLimits = asStructLimits[player as usize];
        /*		//also check the mission structures lists

		for (psCurr = mission.apsStructLists[player]; psCurr != NULL; psCurr =
			psCurr->psNext)
		{
			inc = psCurr->pStructureType - asStructureStats;
			psStructLimits[inc].currentQuantity++;
			if (displayError)
			{
				//check quantity never exceeds the limit
				if (psStructLimits[inc].currentQuantity > psStructLimits[inc].limit)
				{
					ASSERT( FALSE, "There appears to be too many %s on this map!",
						getStructName(&asStructureStats[inc] ) );
				}
			}
		}
*/
        inc = 0 as libc::c_int as UDWORD;
        while inc < numStructureStats {
            (*psStructLimits.offset(inc as isize)).currentQuantity =
                0 as libc::c_int as UBYTE;
            inc = inc.wrapping_add(1)
        }
        psCurr = apsStructLists[player as usize];
        while !psCurr.is_null() {
            inc =
                (*psCurr).pStructureType.wrapping_offset_from(asStructureStats)
                    as libc::c_int as UDWORD;
            let ref mut fresh0 =
                (*psStructLimits.offset(inc as isize)).currentQuantity;
            *fresh0 = (*fresh0).wrapping_add(1);
            if displayError != 0 {
                //initialise the current quantity for all structures
                //check quantity never exceeds the limit
                if (*psStructLimits.offset(inc as isize)).currentQuantity as
                       libc::c_int >
                       (*psStructLimits.offset(inc as isize)).limit as
                           libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"There appears to be too many %s on this map!\x00"
                                  as *const u8 as *const libc::c_char,
                              getStructName(&mut *asStructureStats.offset(inc
                                                                              as
                                                                              isize)));
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 1157 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 25],
                                                        &[libc::c_char; 25]>(b"setCurrentStructQuantity\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            psCurr = (*psCurr).psNext
        }
        player = player.wrapping_add(1)
    };
}
//Load the weapons assigned to Structure in the Access database
#[no_mangle]
pub unsafe extern "C" fn loadStructureWeapons(mut pWeaponData:
                                                  *mut libc::c_char,
                                              mut bufferSize: UDWORD)
 -> BOOL {
    let mut pStartWeaponData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut incS: UDWORD = 0;
    let mut incW: UDWORD = 0;
    let mut StructureName: [STRING; 60] = [0; 60];
    let mut WeaponName: [STRING; 60] = [0; 60];
    let mut pStructure: *mut STRUCTURE_STATS = asStructureStats;
    let mut pWeapon: *mut WEAPON_STATS = asWeaponStats;
    let mut weaponFound: BOOL = 0;
    let mut structureFound: BOOL = 0;
    pStartWeaponData = pWeaponData;
    NumToAlloc = numCR(pWeaponData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        //read the data into the storage - the data is delimeted using comma's
        StructureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        WeaponName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pWeaponData,
               b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as
                   *const libc::c_char, StructureName.as_mut_ptr(),
               WeaponName.as_mut_ptr());
        if getResourceName(StructureName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        if getResourceName(WeaponName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        structureFound = 0 as libc::c_int;
        weaponFound = structureFound;
        //loop through each Structure_Stat to compare the name
        incS = 0 as libc::c_int as UDWORD;
        while incS < numStructureStats {
            if strcmp(StructureName.as_mut_ptr(),
                      (*pStructure.offset(incS as isize)).pName) == 0 {
                //Structure found, so loop through each weapon
                structureFound = 1 as libc::c_int;
                incW = 0 as libc::c_int as UDWORD;
                while incW < numWeaponStats {
                    if strcmp(WeaponName.as_mut_ptr(),
                              (*pWeapon.offset(incW as isize)).pName) == 0 {
                        weaponFound = 1 as libc::c_int;
                        //weapon found alloc this weapon to the current Structure
						//pStructure[incS].defaultWeap++;
						//pStructure[incS].asWeapList[pStructure[incS].defaultWeap] =
						//	&pWeapon[incW];
						//check not allocating more than allowed
						//if (pStructure[incS].defaultWeap >
						//				(SDWORD)pStructure[incS].weaponSlots)
                        //see if we have already allocated one
                        if !(*pStructure.offset(incS as
                                                    isize)).psWeapStat.is_null()
                           {
                            debug(LOG_ERROR,
                                  b"Trying to allocate more weapons than allowed for Structure\x00"
                                      as *const u8 as *const libc::c_char);
                            abort();
                        }
                        let ref mut fresh1 =
                            (*pStructure.offset(incS as isize)).psWeapStat;
                        *fresh1 =
                            &mut *pWeapon.offset(incW as isize) as
                                *mut WEAPON_STATS;
                        break ;
                    } else { incW = incW.wrapping_add(1) }
                }
                //if weapon not found - error
                if weaponFound == 0 {
                    debug(LOG_ERROR,
                          b"Unable to find stats for weapon %s for structure %s\x00"
                              as *const u8 as *const libc::c_char,
                          WeaponName.as_mut_ptr(),
                          StructureName.as_mut_ptr());
                    abort();
                }
            }
            incS = incS.wrapping_add(1)
        }
        //if structure not found - error
        if structureFound == 0 {
            debug(LOG_ERROR,
                  b"Unable to find stats for structure %s\x00" as *const u8 as
                      *const libc::c_char, StructureName.as_mut_ptr());
            abort();
        }
        //increment the pointer to the start of the next record
        pWeaponData =
            strchr(pWeaponData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartWeaponData);
    return 1 as libc::c_int;
}
//Load the programs assigned to Droids in the Access database
#[no_mangle]
pub unsafe extern "C" fn loadStructureFunctions(mut pFunctionData:
                                                    *mut libc::c_char,
                                                mut bufferSize: UDWORD)
 -> BOOL {
    let mut pStartFunctionData: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut NumToAlloc: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut incS: UDWORD = 0;
    let mut incF: UDWORD = 0;
    let mut StructureName: [STRING; 60] = [0; 60];
    let mut FunctionName: [STRING; 60] = [0; 60];
    let mut pStructure: *mut STRUCTURE_STATS = asStructureStats;
    let mut pFunction: *mut FUNCTION = 0 as *mut FUNCTION;
    let mut pStartFunctions: *mut *mut FUNCTION = asFunctions;
    let mut functionFound: BOOL = 0;
    let mut structureFound: BOOL = 0;
    pStartFunctionData = pFunctionData;
    NumToAlloc = numCR(pFunctionData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumToAlloc {
        //read the data into the storage - the data is delimeted using comma's
        StructureName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        FunctionName[0 as libc::c_int as usize] = '\u{0}' as i32 as STRING;
        sscanf(pFunctionData,
               b"%[^\',\'],%[^\',\'],%*d\x00" as *const u8 as
                   *const libc::c_char, StructureName.as_mut_ptr(),
               FunctionName.as_mut_ptr());
        structureFound = 0 as libc::c_int;
        functionFound = structureFound;
        if getResourceName(StructureName.as_mut_ptr()) == 0 {
            return 0 as libc::c_int
        }
        /*if (!getResourceName(FunctionName))
		{
			return FALSE;
		}*/
        //loop through each Structure_Stat to compare the name
        incS = 0 as libc::c_int as UDWORD;
        while incS < numStructureStats {
            if strcmp(StructureName.as_mut_ptr(),
                      (*pStructure.offset(incS as isize)).pName) == 0 {
                //Structure found, so loop through each Function
                structureFound = 1 as libc::c_int;
                pStartFunctions = asFunctions;
                incF = 0 as libc::c_int as UDWORD;
                while incF < numFunctions {
                    pFunction = *pStartFunctions;
                    if strcmp(FunctionName.as_mut_ptr(), (*pFunction).pName)
                           == 0 {
                        //function found alloc this function to the current Structure
                        functionFound = 1 as libc::c_int;
                        let ref mut fresh2 =
                            (*pStructure.offset(incS as isize)).defaultFunc;
                        *fresh2 += 1;
                        //check not allocating more than allowed
                        if (*pStructure.offset(incS as isize)).defaultFunc >
                               (*pStructure.offset(incS as isize)).numFuncs as
                                   SDWORD {
                            debug(LOG_ERROR,
                                  b"Trying to allocate more functions than allowed for Structure\x00"
                                      as *const u8 as *const libc::c_char);
                            abort();
                        }
                        let ref mut fresh3 =
                            *(*pStructure.offset(incS as
                                                     isize)).asFuncList.offset((*pStructure.offset(incS
                                                                                                       as
                                                                                                       isize)).defaultFunc
                                                                                   as
                                                                                   isize);
                        *fresh3 = pFunction;
                        break ;
                    } else {
                        pStartFunctions = pStartFunctions.offset(1);
                        incF = incF.wrapping_add(1)
                    }
                }
                //if function not found - error
                if functionFound == 0 {
                    debug(LOG_ERROR,
                          b"Unable to find stats for function %s for structure %s\x00"
                              as *const u8 as *const libc::c_char,
                          FunctionName.as_mut_ptr(),
                          StructureName.as_mut_ptr());
                    abort();
                }
            }
            incS = incS.wrapping_add(1)
        }
        //if structure not found - error
        if structureFound == 0 {
            debug(LOG_ERROR,
                  b"Unable to find stats for structure %s\x00" as *const u8 as
                      *const libc::c_char, StructureName.as_mut_ptr());
            abort();
        }
        //increment the pointer to the start of the next record
        pFunctionData =
            strchr(pFunctionData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    //	FREE(pStartFunctionData);
    /* *************************************************************************/
	//Wall Function requires a structure stat so can allocate it now
    pStartFunctions = asFunctions;
    incF = 0 as libc::c_int as UDWORD;
    while incF < numFunctions {
        pFunction = *pStartFunctions;
        if (*pFunction).type_0 as libc::c_int == WALL_TYPE as libc::c_int {
            //loop through all structures to find the stat
            pStructure = asStructureStats;
            let ref mut fresh4 =
                (*(pFunction as *mut WALL_FUNCTION)).pCornerStat;
            *fresh4 = 0 as *mut _structure_stats;
            i = 0 as libc::c_int as UDWORD;
            while i < numStructureStats {
                //compare the names
                if strcmp((*(pFunction as *mut WALL_FUNCTION)).pStructName,
                          (*pStructure).pName) == 0 {
                    let ref mut fresh5 =
                        (*(pFunction as *mut WALL_FUNCTION)).pCornerStat;
                    *fresh5 = pStructure;
                    break ;
                } else {
                    pStructure = pStructure.offset(1);
                    i = i.wrapping_add(1)
                }
            }
            //if haven't found the STRUCTURE STAT, then problem
            if (*(pFunction as *mut WALL_FUNCTION)).pCornerStat.is_null() {
                debug(LOG_ERROR,
                      b"Unknown Corner Wall stat for function %s\x00" as
                          *const u8 as *const libc::c_char,
                      (*pFunction).pName);
                abort();
            }
        }
        pStartFunctions = pStartFunctions.offset(1);
        incF = incF.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*Load the Structure Strength Modifiers from the file exported from Access*/
#[no_mangle]
pub unsafe extern "C" fn loadStructureStrengthModifiers(mut pStrengthModData:
                                                            *mut libc::c_char,
                                                        mut bufferSize:
                                                            UDWORD) -> BOOL {
    let mut strengthInc: STRUCT_STRENGTH = STRENGTH_SOFT;
    let mut effectInc: WEAPON_EFFECT = WE_ANTI_PERSONNEL;
    let mut NumRecords: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut modifier: UDWORD = 0;
    let mut weaponEffectName: [STRING; 60] = [0; 60];
    let mut strengthName: [STRING; 60] = [0; 60];
    //memset(asStructStrengthModifier, 0, (sizeof(STRUCTSTRENGTH_MODIFIER) *
	//	WE_NUMEFFECTS * NUM_STRUCT_STRENGTH));
	//initialise to 100%
    i = 0 as libc::c_int as UDWORD;
    while i < WE_NUMEFFECTS as libc::c_int as libc::c_uint {
        j = 0 as libc::c_int as UDWORD;
        while j < NUM_STRUCT_STRENGTH as libc::c_int as libc::c_uint {
            asStructStrengthModifier[i as usize][j as usize] =
                100 as libc::c_int as STRUCTSTRENGTH_MODIFIER;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    NumRecords = numCR(pStrengthModData, bufferSize);
    i = 0 as libc::c_int as UDWORD;
    while i < NumRecords {
        //read the data into the storage - the data is delimeted using comma's
        sscanf(pStrengthModData,
               b"%[^\',\'],%[^\',\'],%d\x00" as *const u8 as
                   *const libc::c_char, weaponEffectName.as_mut_ptr(),
               strengthName.as_mut_ptr(), &mut modifier as *mut UDWORD);
        //get the weapon effect inc
        effectInc =
            getWeaponEffect(weaponEffectName.as_mut_ptr()) as WEAPON_EFFECT;
        if effectInc as libc::c_uint ==
               (WE_NUMEFFECTS as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadStructureStrengthModifiers: Invalid Weapon Effect - %s\x00"
                      as *const u8 as *const libc::c_char,
                  weaponEffectName.as_mut_ptr());
            abort();
        }
        //get the propulsion inc
        strengthInc =
            getStructStrength(strengthName.as_mut_ptr()) as STRUCT_STRENGTH;
        if strengthInc as libc::c_uint ==
               (NUM_STRUCT_STRENGTH as libc::c_int + 1 as libc::c_int) as
                   libc::c_uint {
            debug(LOG_ERROR,
                  b"loadStructureStrengthModifiers: Invalid Strength type - %s\x00"
                      as *const u8 as *const libc::c_char,
                  strengthName.as_mut_ptr());
            abort();
        }
        if modifier > 0xffff as libc::c_int as libc::c_uint {
            debug(LOG_ERROR,
                  b"loadStructureStrengthModifiers: modifier for effect %s, strength %s is too large\x00"
                      as *const u8 as *const libc::c_char,
                  weaponEffectName.as_mut_ptr(), strengthName.as_mut_ptr());
            abort();
        }
        //store in the appropriate index
        asStructStrengthModifier[effectInc as usize][strengthInc as usize] =
            modifier as UWORD;
        //increment the pointer to the start of the next record
        pStrengthModData =
            strchr(pStrengthModData,
                   '\n' as i32).offset(1 as libc::c_int as isize);
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn structureStatsShutDown() -> BOOL {
    let mut inc: UDWORD = 0;
    let mut pStructure: *mut STRUCTURE_STATS = asStructureStats;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numStructureStats {
        //#ifndef RESOURCE_NAMES
        /*if (pStructure->numWeaps > 0)
		{
			FREE(pStructure->asWeapList);
		}*/
        if (*pStructure).numFuncs > 0 as libc::c_int as libc::c_uint {
            memFreeRelease((*pStructure).asFuncList as *mut libc::c_void);
            (*pStructure).asFuncList = 0 as *mut *mut _function
        }
        inc = inc.wrapping_add(1);
        pStructure = pStructure.offset(1)
    }
    if numStructureStats != 0 {
        memFreeRelease(asStructureStats as *mut libc::c_void);
        asStructureStats = 0 as *mut STRUCTURE_STATS
    }
    //free up the structLimits structure
    inc = 0 as libc::c_int as UDWORD;
    while inc < 8 as libc::c_int as libc::c_uint {
        if !asStructLimits[inc as usize].is_null() {
            memFreeRelease(asStructLimits[inc as usize] as *mut libc::c_void);
            asStructLimits[inc as usize] = 0 as *mut STRUCTURE_LIMITS
        }
        inc = inc.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* Do damage to a Structure.
 * Returns TRUE if the Structure is destroyed
 */
#[no_mangle]
pub unsafe extern "C" fn structureDamage(mut psStructure: *mut STRUCTURE,
                                         mut damage: UDWORD,
                                         mut weaponClass: UDWORD,
                                         mut weaponSubClass: UDWORD) -> BOOL {
    let mut penDamage: UDWORD = 0;
    let mut armourDamage: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureDamage: Invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              1535 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"structureDamage\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //EMP cannons do not work on Structures
    if weaponSubClass == WSC_EMP as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    //	if(selectedPlayer==0)
    if (*psStructure).player as libc::c_uint != selectedPlayer {
        // Player inflicting damage on enemy.
        damage =
            modifyForDifficultyLevel(damage as SDWORD, 1 as libc::c_int) as
                UDWORD
    } else {
        // Enemy inflicting damage on player.
        damage =
            modifyForDifficultyLevel(damage as SDWORD, 0 as libc::c_int) as
                UDWORD
    }
    //store the time it was hit
    (*psStructure).timeLastHit = gameTime;
    // tell the cluster system it has been attacked
    clustObjectAttacked(psStructure as *mut BASE_OBJECT);
    if damage > (*psStructure).armour as libc::c_uint {
        /* Damage has penetrated - reduce armour and body points */
        penDamage =
            damage.wrapping_sub((*psStructure).armour as libc::c_uint);
        if penDamage >= (*psStructure).body as libc::c_uint {
            //		DBP1(("penetrated: %d, armour: %d\n", penDamage, armourDamage));
            /* structure destroyed */
            return destroyStruct(psStructure)
        } else {
            (*psStructure).body =
                ((*psStructure).body as libc::c_int -
                     penDamage as UWORD as libc::c_int) as UWORD
        }
        armourDamage =
            damage.wrapping_div(10 as libc::c_int as
                                    libc::c_uint).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint)
    } else {
        /* Do damage to armour */
        /* Damage didn't penetrate - only reduce armour */
        armourDamage =
            damage.wrapping_div(50 as libc::c_int as
                                    libc::c_uint).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint);
        //		DBP1(("armour: %d\n", armourDamage));
        if (*psStructure).body as libc::c_int == 1 as libc::c_int {
            return destroyStruct(psStructure)
            /* Do one point of damage to body */
            //			return TRUE;
        } else {
            (*psStructure).body =
                ((*psStructure).body as libc::c_int - 1 as libc::c_int) as
                    UWORD
        }
    }
    /* Actually reduce the Structure's armour */
    //only overwrite if the last weapon to hit was not an EMP - need the time value for this
    if (*psStructure).lastHitWeapon != WSC_EMP as libc::c_int as libc::c_uint
       {
        (*psStructure).timeLastHit = gameTime;
        (*psStructure).lastHitWeapon = weaponSubClass
    }
    return 0 as libc::c_int;
}
/* Set the type of droid for a factory to build */
#[no_mangle]
pub unsafe extern "C" fn structSetManufacture(mut psStruct: *mut STRUCTURE,
                                              mut psTempl:
                                                  *mut DROID_TEMPLATE,
                                              mut quantity: UBYTE) -> BOOL {
    let mut psFact: *mut FACTORY = 0 as *mut FACTORY;
    if 1 as libc::c_int != 0 &&
           (*psStruct).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           ((*(*psStruct).pStructureType).type_0 ==
                REF_FACTORY as libc::c_int as libc::c_uint ||
                (*(*psStruct).pStructureType).type_0 ==
                    REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                (*(*psStruct).pStructureType).type_0 ==
                    REF_VTOL_FACTORY as libc::c_int as libc::c_uint) {
    } else {
        debug(LOG_ERROR,
              b"structSetManufacture: invalid Factory pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 &&
           (*psStruct).type_0 as libc::c_uint ==
               OBJ_STRUCTURE as libc::c_int as libc::c_uint &&
           ((*(*psStruct).pStructureType).type_0 ==
                REF_FACTORY as libc::c_int as libc::c_uint ||
                (*(*psStruct).pStructureType).type_0 ==
                    REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                (*(*psStruct).pStructureType).type_0 ==
                    REF_VTOL_FACTORY as libc::c_int as libc::c_uint) {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              1639 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"structSetManufacture\x00")).as_ptr(),
              b"PTRVALID(psStruct, sizeof(STRUCTURE)) && psStruct->type == OBJ_STRUCTURE && (psStruct->pStructureType->type == REF_FACTORY OR psStruct->pStructureType->type == REF_CYBORG_FACTORY OR psStruct->pStructureType->type == REF_VTOL_FACTORY)\x00"
                  as *const u8 as *const libc::c_char);
    };
    /* psTempl might be NULL if the build is being cancelled in the middle */
    if psTempl.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structSetManufacture: invalid Template pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if psTempl.is_null() || 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              1642 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 21],
                                        &[libc::c_char; 21]>(b"structSetManufacture\x00")).as_ptr(),
              b"psTempl == NULL || PTRVALID(psTempl, sizeof(DROID_TEMPLATE))\x00"
                  as *const u8 as *const libc::c_char);
    };
    //assign it to the Factory
    psFact = (*psStruct).pFunctionality as *mut FACTORY;
    (*psFact).psSubject = psTempl as *mut BASE_STATS;
    //set up the start time and build time
    if !psTempl.is_null() {
        //only use this for non selectedPlayer
        if (*psStruct).player as libc::c_uint != selectedPlayer {
            //set quantity to produce
            (*psFact).quantity = quantity
        } //gameTime;
        (*psFact).timeStarted = 0 as libc::c_int as UDWORD;
        (*psFact).powerAccrued = 0 as libc::c_int as UDWORD;
        (*psFact).timeStartHold = 0 as libc::c_int as UDWORD;
        (*psFact).timeToBuild =
            (*psTempl).buildPoints.wrapping_div((*psFact).productionOutput as
                                                    libc::c_uint);
        //check for zero build time - usually caused by 'silly' data!
        if (*psFact).timeToBuild == 0 as libc::c_int as libc::c_uint {
            //set to 1/1000th sec - ie very fast!
            (*psFact).timeToBuild = 1 as libc::c_int as UDWORD
        }
    }
    return 1 as libc::c_int;
}
/* ****************************************************************************/
/*
 * All this wall type code is horrible, but I really can't think of a better way to do it.
 *        John.
 */
// look at where other walls are to decide what type of wall to build
unsafe extern "C" fn structWallScan(mut aWallPresent: *mut [BOOL; 5],
                                    mut x: SDWORD, mut y: SDWORD) -> SDWORD {
    let mut cx: SDWORD = 0;
    let mut cy: SDWORD = 0;
    let mut numWalls: SDWORD = 0;
    numWalls = 0 as libc::c_int;
    cx = x - 1 as libc::c_int;
    while cx <= x + 1 as libc::c_int {
        cy = y - 1 as libc::c_int;
        while cy <= y + 1 as libc::c_int {
            // do not look at walls diagonally from this wall
            if (cx == x && cy != y || cx != x && cy == y) &&
                   (*aWallPresent.offset(cx as isize))[cy as usize] != 0 {
                numWalls += 1 as libc::c_int
            }
            cy += 1 as libc::c_int
        }
        cx += 1 as libc::c_int
    }
    // Now decide what type of wall is needed
    if numWalls == 1 as libc::c_int {
        if (*aWallPresent.offset((x - 1 as libc::c_int) as isize))[y as usize]
               != 0 ||
               (*aWallPresent.offset((x + 1 as libc::c_int) as
                                         isize))[y as usize] != 0 {
            // there is a wall horizontally adjacent
            return 0 as libc::c_int
        } else { return 1 as libc::c_int }
    } else {
        if numWalls == 2 as libc::c_int {
            if (*aWallPresent.offset((x - 1 as libc::c_int) as
                                         isize))[y as usize] != 0 &&
                   (*aWallPresent.offset((x + 1 as libc::c_int) as
                                             isize))[y as usize] != 0 {
                // there is a wall horizontally adjacent
                return 0 as libc::c_int
            } else if (*aWallPresent.offset(x as
                                                isize))[(y - 1 as libc::c_int)
                                                            as usize] != 0 &&
                          (*aWallPresent.offset(x as
                                                    isize))[(y +
                                                                 1 as
                                                                     libc::c_int)
                                                                as usize] != 0
             {
                // there is a wall vertically adjacent
                return 1 as libc::c_int
            } else { return 2 as libc::c_int }
        } else {
            if numWalls > 2 as libc::c_int {
                // definately need a corner wall
                return 2 as libc::c_int
            }
        }
    }
    return 1 as libc::c_int;
}
// Choose a type of wall for a location - and update any neighbouring walls
#[no_mangle]
pub unsafe extern "C" fn structChooseWallType(mut player: UDWORD,
                                              mut mapX: UDWORD,
                                              mut mapY: UDWORD) -> SDWORD {
    let mut aWallPresent: [[BOOL; 5]; 5] = [[0; 5]; 5];
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut x: SDWORD = 0;
    let mut y: SDWORD = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut apsStructs: [[*mut STRUCTURE; 5]; 5] =
        [[0 as *mut STRUCTURE; 5]; 5];
    let mut nayborType: SDWORD = 0;
    let mut scanType: SDWORD = 0;
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut sx: UDWORD = 0;
    let mut sy: UDWORD = 0;
    let mut oldBuildPoints: UDWORD = 0;
    // scan around the location looking for walls
    memset(aWallPresent.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<[[BOOL; 5]; 5]>() as libc::c_ulong);
    psStruct = apsStructLists[player as usize];
    while !psStruct.is_null() {
        xdiff =
            mapX as SDWORD - ((*psStruct).x as SDWORD >> 7 as libc::c_int);
        ydiff =
            mapY as SDWORD - ((*psStruct).y as SDWORD >> 7 as libc::c_int);
        if xdiff >= -(2 as libc::c_int) && xdiff <= 2 as libc::c_int &&
               ydiff >= -(2 as libc::c_int) && ydiff <= 2 as libc::c_int &&
               ((*(*psStruct).pStructureType).type_0 ==
                    REF_WALL as libc::c_int as libc::c_uint ||
                    (*(*psStruct).pStructureType).type_0 ==
                        REF_WALLCORNER as libc::c_int as libc::c_uint ||
                    (*(*psStruct).pStructureType).type_0 ==
                        REF_DEFENSE as libc::c_int as libc::c_uint) {
            aWallPresent[(xdiff + 2 as libc::c_int) as
                             usize][(ydiff + 2 as libc::c_int) as usize] =
                1 as libc::c_int;
            apsStructs[(xdiff + 2 as libc::c_int) as
                           usize][(ydiff + 2 as libc::c_int) as usize] =
                psStruct
        }
        psStruct = (*psStruct).psNext
    }
    // add in the wall about to be built
    aWallPresent[2 as libc::c_int as usize][2 as libc::c_int as usize] =
        1 as libc::c_int;
    // now make sure that all the walls around this one are OK
    x = 1 as libc::c_int;
    while x <= 3 as libc::c_int {
        y = 1 as libc::c_int;
        while y <= 3 as libc::c_int {
            // do not look at walls diagonally from this wall
            if (x == 2 as libc::c_int && y != 2 as libc::c_int ||
                    x != 2 as libc::c_int && y == 2 as libc::c_int) &&
                   aWallPresent[x as usize][y as usize] != 0 {
                // figure out what type the wall currently is
                psStruct = apsStructs[x as usize][y as usize];
                if (*(*psStruct).pStructureType).type_0 ==
                       REF_WALL as libc::c_int as libc::c_uint {
                    if (*psStruct).direction as libc::c_int ==
                           90 as libc::c_int {
                        nayborType = 1 as libc::c_int
                    } else { nayborType = 0 as libc::c_int }
                    // see what type the wall should be
                    scanType =
                        structWallScan(aWallPresent.as_mut_ptr(), x, y);
                    if nayborType != scanType {
                        // Got to change the wall
                        if scanType == 2 as libc::c_int {
                            // change to a corner
                            if (**(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      isize)).type_0
                                   as libc::c_int == WALL_TYPE as libc::c_int
                               {
                                /* Still being built - so save and load build points */
                                if (*psStruct).status as libc::c_int ==
                                       SS_BEING_BUILT as libc::c_int {
                                    oldBuildPoints =
                                        (*psStruct).currentBuildPts as UDWORD;
                                    psStats =
                                        (*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                               as
                                               *mut WALL_FUNCTION)).pCornerStat;
                                    sx = (*psStruct).x as UDWORD;
                                    sy = (*psStruct).y as UDWORD;
                                    removeStruct(psStruct, 1 as libc::c_int);
                                    powerCalc(0 as libc::c_int);
                                    psStruct =
                                        buildStructure(psStats, sx, sy,
                                                       player,
                                                       1 as libc::c_int);
                                    powerCalc(1 as libc::c_int);
                                    if !psStruct.is_null() {
                                        (*psStruct).status =
                                            SS_BEING_BUILT as libc::c_int as
                                                UBYTE;
                                        (*psStruct).currentBuildPts =
                                            oldBuildPoints as SWORD
                                    }
                                } else {
                                    psStats =
                                        (*(*(*(*psStruct).pStructureType).asFuncList.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                               as
                                               *mut WALL_FUNCTION)).pCornerStat;
                                    sx = (*psStruct).x as UDWORD;
                                    sy = (*psStruct).y as UDWORD;
                                    removeStruct(psStruct, 1 as libc::c_int);
                                    powerCalc(0 as libc::c_int);
                                    psStruct =
                                        buildStructure(psStats, sx, sy,
                                                       player,
                                                       1 as libc::c_int);
                                    powerCalc(1 as libc::c_int);
                                    if !psStruct.is_null() {
                                        (*psStruct).status =
                                            SS_BUILT as libc::c_int as UBYTE;
                                        buildingComplete(psStruct);
                                    }
                                }
                            }
                        } else if scanType == 0 as libc::c_int {
                            // change to a horizontal wall
                            (*psStruct).direction = 0 as libc::c_int as UWORD
                        } else {
                            // change to a vertical wall
                            (*psStruct).direction = 90 as libc::c_int as UWORD
                        }
                    }
                }
            }
            // do not need to adjust anything apart from walls
            y += 1 as libc::c_int
        }
        x += 1 as libc::c_int
    }
    // finally return the type for this wall
    return structWallScan(aWallPresent.as_mut_ptr(), 2 as libc::c_int,
                          2 as libc::c_int);
}
// Set the tile no draw flags for a structure
#[no_mangle]
pub unsafe extern "C" fn setStructTileDraw(mut psStruct: *mut STRUCTURE) {
    let mut pStructureType: *mut STRUCTURE_STATS = (*psStruct).pStructureType;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    mapX =
        (((*psStruct).x as libc::c_int >> 7 as libc::c_int) as
             libc::c_uint).wrapping_sub((*pStructureType).baseWidth.wrapping_div(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint));
    mapY =
        (((*psStruct).y as libc::c_int >> 7 as libc::c_int) as
             libc::c_uint).wrapping_sub((*pStructureType).baseBreadth.wrapping_div(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint));
    width = 0 as libc::c_int as UDWORD;
    while width < (*pStructureType).baseWidth {
        breadth = 0 as libc::c_int as UDWORD;
        while breadth < (*pStructureType).baseBreadth {
            if !(*pStructureType).pBaseIMD.is_null() {
                (*mapTile(mapX.wrapping_add(width),
                          mapY.wrapping_add(breadth))).tileInfoBits =
                    ((*mapTile(mapX.wrapping_add(width),
                               mapY.wrapping_add(breadth))).tileInfoBits as
                         libc::c_int | 0x4 as libc::c_int) as UBYTE
            }
            breadth = breadth.wrapping_add(1)
        }
        width = width.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn buildFlatten(mut pStructureType:
                                          *mut STRUCTURE_STATS,
                                      mut atx: UDWORD, mut aty: UDWORD,
                                      mut h: UDWORD) {
    let mut x: UDWORD = atx >> 7 as libc::c_int;
    let mut y: UDWORD = aty >> 7 as libc::c_int;
    let mut width: UBYTE = 0;
    let mut breadth: UBYTE = 0;
    //STRUCTURE           *psStruct;
    breadth = 0 as libc::c_int as UBYTE;
    while breadth as libc::c_int <=
              (*pStructureType).baseBreadth as UBYTE as libc::c_int {
        width = 0 as libc::c_int as UBYTE;
        while width as libc::c_int <=
                  (*pStructureType).baseWidth as UBYTE as libc::c_int {
            //			while((x+width-1)<0)width++;
//			while((y+breadth-1)<0)breadth++;
            if (*pStructureType).type_0 !=
                   REF_WALL as libc::c_int as libc::c_uint &&
                   (*pStructureType).type_0 !=
                       REF_WALLCORNER as libc::c_int as libc::c_uint {
                /*&&*/
                /*				(pStructureType->type != REF_DEFENSE))*/
                setTileHeight(x.wrapping_add(width as libc::c_uint),
                              y.wrapping_add(breadth as libc::c_uint),
                              h); //-1
                if (*mapTile(x.wrapping_add(width as libc::c_uint),
                             y.wrapping_add(breadth as
                                                libc::c_uint))).tileInfoBits
                       as libc::c_int & 0x2 as libc::c_int != 0 {
                    (*getTileFeature(x.wrapping_add(width as libc::c_uint),
                                     y.wrapping_add(breadth as
                                                        libc::c_uint))).z =
                        h as UWORD
                }
            }
            width = width.wrapping_add(1)
        }
        breadth = breadth.wrapping_add(1)
    };
}
// We need to raise features on raised tiles to the new height
/*Builds an instance of a Structure - the x/y passed in are in world coords. */
#[no_mangle]
pub unsafe extern "C" fn buildStructure(mut pStructureType:
                                            *mut STRUCTURE_STATS,
                                        mut x: UDWORD, mut y: UDWORD,
                                        mut player: UDWORD,
                                        mut FromSave: BOOL)
 -> *mut STRUCTURE {
    let mut psBuilding: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeature: *mut FEATURE = 0 as *mut FEATURE;
    let mut psFNext: *mut FEATURE = 0 as *mut FEATURE;
    //MESSAGE		*psMessage;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut mapH: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut weapon: UDWORD = 0;
    let mut capacity: UDWORD = 0;
    let mut bodyDiff: UDWORD = 0 as libc::c_int as UDWORD;
    let mut wallType: SDWORD = 0 as libc::c_int;
    let mut preScrollMinX: SDWORD = 0 as libc::c_int;
    let mut preScrollMinY: SDWORD = 0 as libc::c_int;
    let mut preScrollMaxX: SDWORD = 0 as libc::c_int;
    let mut preScrollMaxY: SDWORD = 0 as libc::c_int;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut bUpgraded: BOOL = 0;
    //	BOOL bTemp=FALSE;
    let mut min: UDWORD = 0;
    let mut max: UDWORD = 0;
    //#ifdef FILTER_WALLS
//		if( (pStructureType->type == REF_WALL) || (pStructureType->type == REF_WALLCORNER) ) {
//			DBPRINTF(("Filtered out WALL\n"));
//			return NULL;
//		}
//#endif
    if IsStatExpansionModule(pStructureType) == 0 as libc::c_int {
        //this happends gradually now - AB 5/1/99
	    /*subtract the power required to build
	    if (!usePower(player, pStructureType->powerToBuild))
	    {
		    return NULL;
	    }*/
        //some prelim tests...
        max =
            pStructureType.wrapping_offset_from(asStructureStats) as
                libc::c_int as UDWORD;
        if max > numStructureStats {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"buildStructure:Invalid structure type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      1964 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut STRUCTURE
        }
        if player == selectedPlayer {
            //return psBuilding;
            //don't allow more than interface limits of certain structures
            if (*pStructureType).type_0 ==
                   REF_FACTORY as libc::c_int as libc::c_uint ||
                   (*pStructureType).type_0 ==
                       REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
                   (*pStructureType).type_0 ==
                       REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
                //NEVER EVER EVER WANT MORE THAN 5 FACTORIES
                if (*asStructLimits[selectedPlayer as
                                        usize].offset(max as
                                                          isize)).currentQuantity
                       as libc::c_int > 5 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"buildStructure: trying to build too many factories\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 1979 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut STRUCTURE
                }
            }
            if (*pStructureType).type_0 ==
                   REF_RESEARCH as libc::c_int as libc::c_uint {
                //can only cope with MAX_OBJECTS research facilities
                if (*asStructLimits[selectedPlayer as
                                        usize].offset(max as
                                                          isize)).currentQuantity
                       as libc::c_int > 15 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"buildStructure: trying to build too many research facilities\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 1989 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut STRUCTURE
                }
            }
            //HARD_CODE don't ever want more than one Las Sat structure
            if !(*pStructureType).psWeapStat.is_null() &&
                   (*(*pStructureType).psWeapStat).weaponSubClass as
                       libc::c_uint ==
                       WSC_LAS_SAT as libc::c_int as libc::c_uint &&
                   getLasSatExists(selectedPlayer) != 0 {
                return 0 as *mut STRUCTURE
            }
            //HARD_CODE don't ever want more than one Sat Uplink structure
            if (*pStructureType).type_0 ==
                   REF_SAT_UPLINK as libc::c_int as libc::c_uint {
                if (*asStructLimits[selectedPlayer as
                                        usize].offset(max as
                                                          isize)).currentQuantity
                       as libc::c_int > 0 as libc::c_int {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"buildStructure: trying to build too many Sat Uplinks\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 2005 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                    return 0 as *mut STRUCTURE
                }
            }
        }
        x =
            if (*pStructureType).baseWidth.wrapping_rem(2 as libc::c_int as
                                                            libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint {
                (x) & !(0x7f as libc::c_int) as libc::c_uint
            } else {
                (x &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_add((128 as libc::c_int /
                                                         2 as libc::c_int) as
                                                        libc::c_uint)
            };
        y =
            if (*pStructureType).baseBreadth.wrapping_rem(2 as libc::c_int as
                                                              libc::c_uint) ==
                   0 as libc::c_int as libc::c_uint {
                (y) & !(0x7f as libc::c_int) as libc::c_uint
            } else {
                (y &
                     !(0x7f as libc::c_int) as
                         libc::c_uint).wrapping_add((128 as libc::c_int /
                                                         2 as libc::c_int) as
                                                        libc::c_uint)
            };
        if (x >> 7 as libc::c_int) < 3 as libc::c_int as libc::c_uint ||
               x >> 7 as libc::c_int >
                   mapWidth.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                       SDWORD as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"buildStructure: x coord too near edge\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      2019 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut STRUCTURE
        }
        if (y >> 7 as libc::c_int) < 3 as libc::c_int as libc::c_uint ||
               y >> 7 as libc::c_int >
                   mapHeight.wrapping_sub(3 as libc::c_int as libc::c_uint) as
                       SDWORD as libc::c_uint {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"buildStructure: y coord too near edge\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      2025 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 15],
                                                &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut STRUCTURE
        }
        if FromSave == 0 &&
               (*pStructureType).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint {
            wallType =
                structChooseWallType(player, x >> 7 as libc::c_int,
                                     y >> 7 as libc::c_int);
            if wallType == 2 as libc::c_int {
                if (**(*pStructureType).asFuncList.offset(0 as libc::c_int as
                                                              isize)).type_0
                       as libc::c_int == WALL_TYPE as libc::c_int {
                    pStructureType =
                        (*(*(*pStructureType).asFuncList.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                               as *mut WALL_FUNCTION)).pCornerStat
                }
            }
        }
        if (*pStructureType).type_0 ==
               REF_DEFENSE as libc::c_int as libc::c_uint &&
               (*mapTile(x >> 7 as libc::c_int,
                         y >> 7 as libc::c_int)).tileInfoBits as libc::c_int &
                   0x20 as libc::c_int != 0 {
            // snap the coords to a tile
            //check not trying to build too near the edge
            /*		if (!FromSave && pStructureType->type == REF_DEFENSE)
		{
			// get any walls next to a defensive structure to adjust
			structChooseWallType(player, x >> TILE_SHIFT, y >> TILE_SHIFT);
		}*/
            // building a gun tower over a wall, replace it
            psBuilding =
                getTileStructure(x >> 7 as libc::c_int,
                                 y >> 7 as libc::c_int);
            if !psBuilding.is_null() {
                removeStruct(psBuilding, 1 as libc::c_int);
            }
        }
        if createStruct(player, &mut psBuilding) == 0 {
            return 0 as *mut STRUCTURE
        }
        (*psBuilding).psCurAnim = 0 as *mut ANIM_OBJECT;
        (*psBuilding).pStructureType = pStructureType;
        (*psBuilding).x = x as UWORD;
        (*psBuilding).y = y as UWORD;
        mapX =
            (x >>
                 7 as
                     libc::c_int).wrapping_sub((*pStructureType).baseWidth.wrapping_div(2
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            libc::c_uint));
        mapY =
            (y >>
                 7 as
                     libc::c_int).wrapping_sub((*pStructureType).baseBreadth.wrapping_div(2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              libc::c_uint));
        (*psBuilding).sDisplay.imd = (*pStructureType).pIMD;
        mapH = buildFoundation(pStructureType, x, y) as UDWORD;
        width = 0 as libc::c_int as UDWORD;
        while width < (*pStructureType).baseWidth {
            breadth = 0 as libc::c_int as UDWORD;
            while breadth < (*pStructureType).baseBreadth {
                //try and create the Structure
                //DBPRINTF(("create structure called\n");
//if (PTRVALID(psBuilding,sizeof(STRUCTURE)) != TRUE) DBPRINTF(("...and its not valid\n");
                /* GJ HACK! - add anim to deriks */
		/*if ( pStructureType->type == REF_RESOURCE_EXTRACTOR )
		{
			// add anim now if loading from save file
			if ( FromSave == TRUE )
			{
				psBuilding->psCurAnim = animObj_Add( psBuilding, ID_ANIM_DERIK, 0, 0 );
			}
		}*/
                //fill in other details
                //copy the name across for now
		//psBuilding->pName = pStructureType->pName;
//		if(FromSave==TRUE) {
                /*		} else {
			psBuilding->x = (UWORD)((x & (~TILE_MASK)) + pStructureType->
				baseWidth * TILE_UNITS / 2);
			psBuilding->y = (UWORD)((y & (~TILE_MASK)) + pStructureType->
				baseBreadth * TILE_UNITS / 2);
		}*/
                //This needs to be done before the functionality bit...
		//load into the map data and structure list if not an upgrade
//		if(FromSave==TRUE) {
                /*		} else {
			mapX = x >> TILE_SHIFT;
			mapY = y >> TILE_SHIFT;
		}*/
                //set up the imd to use for the display
                //		psBuilding->sDisplay.animFrame = 0;
                //DBPRINTF(("%d\n",psBuilding->sDisplay.imd->ymax);
                //mapH = buildFoundation(pStructureType, mapX*TILE_UNITS,mapY*TILE_UNITS);
                //psor				mapTile(mapX+width, mapY+breadth)->psObject = (BASE_OBJECT*) psBuilding;
                psTile =
                    mapTile(mapX.wrapping_add(width),
                            mapY.wrapping_add(breadth));
                //				if ((pStructureType->type != REF_WALL) &&
//					(pStructureType->type != REF_WALLCORNER) &&
//					(pStructureType->type != REF_DEFENSE))
//				{
//					setTileHeight(mapX+width, mapY+breadth, mapH);
//				}
                if (*pStructureType).type_0 ==
                       REF_WALLCORNER as libc::c_int as libc::c_uint ||
                       (*pStructureType).type_0 ==
                           REF_WALL as libc::c_int as libc::c_uint {
                    if (*mapTile(mapX.wrapping_add(width),
                                 mapY.wrapping_add(breadth))).tileInfoBits as
                           libc::c_int & 0x1 as libc::c_int != 0 {
                        if (*(*getTileStructure(mapX.wrapping_add(width),
                                                mapY.wrapping_add(breadth))).pStructureType).type_0
                               ==
                               REF_WALLCORNER as libc::c_int as libc::c_uint {
                            return 0 as *mut STRUCTURE
                            // don't really think this should be done here, but dont know otherwise.alexl
                            // dont build.
                        }
                    }
                } else {
                    // end of dodgy stuff
                    if (*mapTile(mapX.wrapping_add(width),
                                 mapY.wrapping_add(breadth))).tileInfoBits as
                           libc::c_int & 0x1 as libc::c_int == 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"buildStructure - structure - %d already found at %d, %d\x00"
                                  as *const u8 as *const libc::c_char,
                              (*psBuilding).id, mapX.wrapping_add(width),
                              mapY.wrapping_add(breadth));
                    };
                    if (*mapTile(mapX.wrapping_add(width),
                                 mapY.wrapping_add(breadth))).tileInfoBits as
                           libc::c_int & 0x1 as libc::c_int == 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 2134 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 15],
                                                        &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                              b"!(TILE_HAS_STRUCTURE(mapTile(mapX+width,mapY+breadth)))\x00"
                                  as *const u8 as *const libc::c_char);
                    };
                }
                (*psTile).tileInfoBits =
                    ((*psTile).tileInfoBits as libc::c_int &
                         !(0x1 as libc::c_int | 0x2 as libc::c_int |
                               0x20 as libc::c_int) | 0x1 as libc::c_int) as
                        UBYTE;
                if (*(*psBuilding).sDisplay.imd).ymax > 200 as libc::c_int {
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x80 as libc::c_int) as UBYTE
                }
                if (*pStructureType).type_0 ==
                       REF_WALL as libc::c_int as libc::c_uint ||
                       (*pStructureType).type_0 ==
                           REF_WALLCORNER as libc::c_int as libc::c_uint {
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x20 as libc::c_int) as UBYTE
                }
                if (*pStructureType).height ==
                       1 as libc::c_int as libc::c_uint {
                    (*psTile).tileInfoBits =
                        ((*psTile).tileInfoBits as libc::c_int |
                             0x8 as libc::c_int) as UBYTE
                }
                breadth = breadth.wrapping_add(1)
            }
            width = width.wrapping_add(1)
        }
        if (*pStructureType).type_0 !=
               REF_DEFENSE as libc::c_int as libc::c_uint {
            buildFlatten(pStructureType, mapX << 7 as libc::c_int,
                         mapY << 7 as libc::c_int, mapH);
            (*psBuilding).z = mapH as UWORD
            // if it's a tall structure then flag it in the map.
            /* DEFENSIVE structures are pulled to the terrain */
            //(UWORD)(map_TileHeight(mapX,mapY)+1);//jps 18july97 - shift it up a wee bit - am
        } else {
            /* Set it at the higher coord */
            getTileMaxMin(mapX, mapY, &mut max, &mut min);
            (*psBuilding).z = max as UWORD
            // Got to be - don't change!!!! ALEXM
        }
        (*psBuilding).turretRotation = 0 as libc::c_int as UWORD;
        (*psBuilding).turretPitch = 0 as libc::c_int as UWORD;
        (*psBuilding).targetted = 0 as libc::c_int as UBYTE;
        (*psBuilding).psTarget = 0 as *mut BASE_OBJECT;
        (*psBuilding).lastEmission = 0 as libc::c_int as UDWORD;
        (*psBuilding).timeLastHit = 0 as libc::c_int as UDWORD;
        (*psBuilding).lastHitWeapon = 0xffffffff as libc::c_uint;
        (*psBuilding).inFire = 0 as libc::c_int;
        (*psBuilding).burnStart = 0 as libc::c_int as UDWORD;
        (*psBuilding).burnDamage = 0 as libc::c_int as UDWORD;
        (*psBuilding).direction = 0 as libc::c_int as UWORD;
        (*psBuilding).pitch = 0 as libc::c_int as SWORD;
        (*psBuilding).roll = 0 as libc::c_int as SWORD;
        (*psBuilding).selected = 0 as libc::c_int as UBYTE;
        (*psBuilding).status = SS_BEING_BUILT as libc::c_int as UBYTE;
        (*psBuilding).currentBuildPts = 0 as libc::c_int as SWORD;
        (*psBuilding).currentPowerAccrued = 0 as libc::c_int as SWORD;
        (*psBuilding).cluster = 0 as libc::c_int as UBYTE;
        if FromSave == 0 &&
               (*pStructureType).type_0 ==
                   REF_WALL as libc::c_int as libc::c_uint &&
               wallType == 1 as libc::c_int {
            (*psBuilding).direction = 90 as libc::c_int as UWORD
        }
        if !(*(*psBuilding).pStructureType).pSensor.is_null() {
            //set up the rest of the data
		//psBuilding->z = pStructureType->height;
            //psBuilding->turretRotRate = 360;
            // these three will come out and into stats
		//psBuilding->lastEventTime = 0;
		//psBuilding->eventFrame = 0;
		//psBuilding->eventInterval = (3*GAME_TICKS_PER_SEC);
            //psBuilding->emissionInterval = 100;
            // no such weapon
            //psBuilding->damage = structureDamage;
            //psBuilding->heightScale = (FRACT)0;
            //psBuilding->sensorRange = 2 * BASE_VISIBILITY;
		//psBuilding->sensorPower = 100;
            // rotate a wall if necessary
            //set up the sensor stats
            //psBuilding->sensorRange = psBuilding->pStructureType->pSensor->range;
			//psBuilding->sensorPower = psBuilding->pStructureType->pSensor->power;
            (*psBuilding).sensorRange =
                sensorRange((*(*psBuilding).pStructureType).pSensor,
                            player as UBYTE) as UWORD;
            (*psBuilding).sensorPower =
                sensorPower((*(*psBuilding).pStructureType).pSensor,
                            player as UBYTE) as UWORD
            //psBuilding->sensorRange = 6 * BASE_VISIBILITY;
			//psBuilding->sensorPower = 200;
        } else {
            //give them the default sensor for droids if not
			//psBuilding->sensorRange = (asSensorStats + aDefaultSensor[player])->range;
			//psBuilding->sensorPower = (asSensorStats + aDefaultSensor[player])->power;
            (*psBuilding).sensorRange =
                sensorRange(asSensorStats.offset(aDefaultSensor[player as
                                                                    usize] as
                                                     isize), player as UBYTE)
                    as UWORD;
            (*psBuilding).sensorPower =
                sensorPower(asSensorStats.offset(aDefaultSensor[player as
                                                                    usize] as
                                                     isize), player as UBYTE)
                    as UWORD
        }
        if !(*(*psBuilding).pStructureType).pECM.is_null() {
            //set up the ecm stat
            //psBuilding->ecmPower = psBuilding->pStructureType->pECM->power;
            (*psBuilding).ecmPower =
                ecmPower((*(*psBuilding).pStructureType).pECM,
                         (*psBuilding).player) as UWORD
        } else { (*psBuilding).ecmPower = 0 as libc::c_int as UWORD }
        memset((*psBuilding).asWeaps.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<WEAPON>() as libc::c_ulong);
        if !(*pStructureType).psWeapStat.is_null() {
            weapon = 0 as libc::c_int as UDWORD;
            memset((*psBuilding).asWeaps.as_mut_ptr().offset(weapon as isize)
                       as *mut libc::c_void, 0 as libc::c_int,
                   ::std::mem::size_of::<WEAPON>() as libc::c_ulong);
            (*psBuilding).asWeaps[weapon as usize].lastFired =
                0 as libc::c_int as UDWORD;
            /* Store the weapons */
		//psBuilding->numWeaps = (UWORD)pStructureType->numWeaps;
//        psBuilding->asWeaps[0].nStat = 0;
            //for(weapon=0; weapon < pStructureType->numWeaps; weapon++)
        //can only have the one weapon now - AB 24/01/99
            //in multiPlayer make the Las-Sats require re-loading from the start
            if bMultiPlayer != 0 &&
                   (*(*pStructureType).psWeapStat).weaponSubClass as
                       libc::c_uint ==
                       WSC_LAS_SAT as libc::c_int as libc::c_uint {
                (*psBuilding).asWeaps[weapon as usize].lastFired = gameTime
            }
            //psBuilding->asWeaps[weapon].nStat =	pStructureType->
			//	asWeapList[weapon] - asWeaponStats;
            (*psBuilding).asWeaps[weapon as usize].nStat =
                (*pStructureType).psWeapStat.wrapping_offset_from(asWeaponStats)
                    as libc::c_int as UDWORD;
            (*psBuilding).asWeaps[weapon as usize].hitPoints =
                (*asWeaponStats.offset((*psBuilding).asWeaps[weapon as
                                                                 usize].nStat
                                           as isize)).hitPoints;
            (*psBuilding).asWeaps[weapon as usize].ammo =
                (*asWeaponStats.offset((*psBuilding).asWeaps[weapon as
                                                                 usize].nStat
                                           as isize)).numRounds as UDWORD;
            (*psBuilding).asWeaps[weapon as usize].recoilValue =
                0 as libc::c_int as UDWORD
        }
        (*psBuilding).armour =
            structureArmour(pStructureType, player as UBYTE) as UWORD;
        (*psBuilding).resistance =
            structureResistance(pStructureType, player as UBYTE) as UWORD as
                SWORD;
        (*psBuilding).lastResistance = 0 as libc::c_int as UDWORD;
        memset((*psBuilding).visible.as_mut_ptr() as *mut libc::c_void,
               0 as libc::c_int,
               (::std::mem::size_of::<UBYTE>() as
                    libc::c_ulong).wrapping_mul(8 as libc::c_int as
                                                    libc::c_uint));
        (*psBuilding).visible[player as usize] = 0xff as libc::c_int as UBYTE;
        if player == selectedPlayer { setStructTileDraw(psBuilding); }
        visTilesUpdate(psBuilding as *mut BASE_OBJECT, 0 as libc::c_int);
        if FromSave != 0 && player == selectedPlayer &&
               missionLimboExpand() != 0 {
            //psBuilding->baseBodyPoints = psBuilding->body;
            //repair not upgradeable at present!
//		psBuilding->repair = (UWORD)pStructureType->repairSystem;
            //psBuilding->body = pStructureType->bodyPoints;
		//psBuilding->baseBodyPoints = psBuilding->body;
		//check for upgrades - work backwards since want the most recent
		/*for (upgrade = numBodyUpgrades-1; upgrade >= 0; upgrade --)
		{
			if (apBodyUpgrades[psBuilding->player][upgrade].available)
			{
				bodyUpgrade(asFunctions[apBodyUpgrades[psBuilding->player][
					upgrade].functionInc], psBuilding);
				break;
			}
		}*/
		//psBuilding->armour = pStructureType->armourValue;
		//check for upgrades - work backwards since want the most recent
		/*for (upgrade = numArmourUpgrades-1; upgrade >= 0; upgrade --)
		{
			if (apArmourUpgrades[psBuilding->player][upgrade].available)
			{
				armourUpgrade(asFunctions[apArmourUpgrades[psBuilding->player][
					upgrade].functionInc], psBuilding);
				break;
			}
		}*/
		//psBuilding->resistance = pStructureType->resistance;
		//check for upgrades - work backwards since want the most recent
		/*for (upgrade = numResistanceUpgrades-1; upgrade >= 0; upgrade --)
		{
			if (apResistanceUpgrades[psBuilding->player][upgrade].available)
			{
				resistanceUpgrade(asFunctions[apResistanceUpgrades[psBuilding->player][
					upgrade].functionInc], psBuilding);
				break;
			}
		}*/
		//psBuilding->repair = pStructureType->repairSystem;
		//check for upgrades - work backwards since want the most recent
		/*for (upgrade = numRepairUpgrades-1; upgrade >= 0; upgrade --)
		{
			if (apRepairUpgrades[psBuilding->player][upgrade].available)
			{
				repairUpgrade(asFunctions[apRepairUpgrades[psBuilding->player][
					upgrade].functionInc], psBuilding);
				break;
			}
		}*/
            //		//set up the imd to use for the display
//		psBuilding->sDisplay.imd = pStructureType->pIMD;
//#ifndef PSX
//		psBuilding->sDisplay.animFrame = 0;
//#endif
            //do the visiblilty stiff before setFunctionality - so placement of DP's can work
//		memset(psBuilding->visible, 0, sizeof(psBuilding->visible));
		/* Not sure about above line - it concerns me, so... */
            /* Structure is trivially visible to the builder (owner) */
            //		if(FromSave)
//		{
			// If we're coming in from a save game, then make buildings visible
			// if the tile they're on is visible.
//		if(TEST_TILE_VISIBLE(selectedPlayer,mapTile(psBuilding->x>>TILE_SHIFT,psBuilding->y>>TILE_SHIFT)))
//		{
//			psBuilding->visible[selectedPlayer] = UBYTE_MAX;
//		}
 //		}
            // Reveal any tiles that can be seen by the structure
            /*if we're coming from a SAVEGAME and we're on an Expand_Limbo mission,
        any factories that were built previously for the selectedPlayer will
        have DP's in an invalid location - the scroll limits will have been
        changed to not include them. This is the only HACK I can think of to
        enable them to be loaded up. So here goes...*/
            //save the current values
            preScrollMinX = scrollMinX;
            preScrollMinY = scrollMinY;
            preScrollMaxX = scrollMaxX;
            preScrollMaxY = scrollMaxY;
            //set the current values to mapWidth/mapHeight
            scrollMinX = 0 as libc::c_int;
            scrollMinY = 0 as libc::c_int;
            scrollMaxX = mapWidth as SDWORD;
            scrollMaxY = mapHeight as SDWORD
        }
        if setFunctionality(psBuilding, (*pStructureType).type_0) == 0 {
            removeStructFromMap(psBuilding);
            heapFree(psStructHeap, psBuilding as *mut libc::c_void);
            //set the functionality dependant on the type of structure
            //better reset these if you couldn't build the structure!
            if FromSave != 0 && player == selectedPlayer &&
                   missionLimboExpand() != 0 {
                //reset the current values
                scrollMinX = preScrollMinX;
                scrollMinY = preScrollMinY;
                scrollMaxX = preScrollMaxX;
                scrollMaxY = preScrollMaxY
            }
            return 0 as *mut STRUCTURE
        }
        if FromSave != 0 && player == selectedPlayer &&
               missionLimboExpand() != 0 {
            //reset the scroll values if adjusted
            //reset the current values
            scrollMinX = preScrollMinX;
            scrollMinY = preScrollMinY;
            scrollMaxX = preScrollMaxX;
            scrollMaxY = preScrollMaxY
        }
        (*psBuilding).body = structureBody(psBuilding) as UWORD;
        addStructure(psBuilding);
        gridAddObject(psBuilding as *mut BASE_OBJECT);
        clustNewStruct(psBuilding);
        if (*pStructureType).type_0 ==
               REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            //add the structure to the list - this enables it to be drawn whilst being built
            /*
//		memset(psBuilding->visible, 0, sizeof(psBuilding->visible));
		// Not sure about above line - it concerns me, so...
		memset(psBuilding->visible, 0, (sizeof(UBYTE) * MAX_PLAYERS) );

		// Structure is trivially visible to the builder (owner)
		psBuilding->visible[player] = UBYTE_MAX;

		if (player == selectedPlayer)
		{
			setStructTileDraw(psBuilding);
		}

//		if(FromSave)
//		{
			// If we're coming in from a save game, then make buildings visible
			// if the tile they're on is visible.
//		if(TEST_TILE_VISIBLE(selectedPlayer,mapTile(psBuilding->x>>TILE_SHIFT,psBuilding->y>>TILE_SHIFT)))
//		{
//			psBuilding->visible[selectedPlayer] = UBYTE_MAX;
//		}
 //		}

		// Reveal any tiles that can be seen by the structure
		visTilesUpdate((BASE_OBJECT *)psBuilding,FALSE);
*/
            //if resource extractor - need to remove oil feature and prox Msg
            //find the resource at this point
            psFeature = apsFeatureLists[0 as libc::c_int as usize];
            while !psFeature.is_null() {
                psFNext = (*psFeature).psNext;
                if (*(*psFeature).psStats).subType as libc::c_uint ==
                       FEAT_OIL_RESOURCE as libc::c_int as libc::c_uint {
                    if (*psFeature).x as libc::c_int ==
                           (*psBuilding).x as libc::c_int &&
                           (*psFeature).y as libc::c_int ==
                               (*psBuilding).y as libc::c_int {
                        //	//see if there is a proximity message FOR THE SELECTED PLAYER at this location
					//	psMessage = findMessage((MSG_VIEWDATA *)psFeature,
					//		MSG_PROXIMITY, selectedPlayer);
					//	if (psMessage)
					//	{
					//		removeMessage(psMessage, selectedPlayer);
					//	}
                        //remove it from the map
                        turnOffMultiMsg(1 as
                                            libc::c_int); // dont send this one!
                        removeFeature(psFeature);
                        turnOffMultiMsg(0 as libc::c_int);
                        //set the map to hold the resource extractor again
                        (*mapTile(((*psFeature).x as libc::c_int >>
                                       7 as libc::c_int) as UDWORD,
                                  ((*psFeature).y as libc::c_int >>
                                       7 as libc::c_int) as
                                      UDWORD)).tileInfoBits =
                            ((*mapTile(((*psFeature).x as libc::c_int >>
                                            7 as libc::c_int) as UDWORD,
                                       ((*psFeature).y as libc::c_int >>
                                            7 as libc::c_int) as
                                           UDWORD)).tileInfoBits as
                                 libc::c_int &
                                 !(0x1 as libc::c_int | 0x2 as libc::c_int |
                                       0x20 as libc::c_int) |
                                 0x1 as libc::c_int) as UBYTE;
                        // if it's a tall structure then flag it in the map.
                        if (*(*psBuilding).sDisplay.imd).ymax >
                               200 as libc::c_int {
                            (*mapTile(((*psFeature).x as libc::c_int >>
                                           7 as libc::c_int) as UDWORD,
                                      ((*psFeature).y as libc::c_int >>
                                           7 as libc::c_int) as
                                          UDWORD)).tileInfoBits =
                                ((*mapTile(((*psFeature).x as libc::c_int >>
                                                7 as libc::c_int) as UDWORD,
                                           ((*psFeature).y as libc::c_int >>
                                                7 as libc::c_int) as
                                               UDWORD)).tileInfoBits as
                                     libc::c_int | 0x80 as libc::c_int) as
                                    UBYTE
                        }
                    }
                }
                psFeature = psFNext
            }
        }
    } else {
        //its an upgrade
        bUpgraded = 0 as libc::c_int;
        //return psBuilding;
        psBuilding =
            getTileStructure(x >> 7 as libc::c_int, y >> 7 as libc::c_int);
        if psBuilding.is_null() {
            //don't create the Structure use existing one
//psor		psBuilding = (STRUCTURE *)mapTile(x >> TILE_SHIFT, y >> TILE_SHIFT)->psObject;
            //debug( LOG_ERROR, "No owning structure for this module - %s", pStructureType->pName );
            debug(LOG_ERROR,
                  b"No owning structure for this module - %s\x00" as *const u8
                      as *const libc::c_char, getStructName(pStructureType));
            abort();
        }
        if (*pStructureType).type_0 ==
               REF_FACTORY_MODULE as libc::c_int as libc::c_uint {
            if (*(*psBuilding).pStructureType).type_0 !=
                   REF_FACTORY as libc::c_int as libc::c_uint &&
                   (*(*psBuilding).pStructureType).type_0 !=
                       REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
                return 0 as *mut STRUCTURE
            }
            //increment the capacity and output for the owning structure
            if ((*((*psBuilding).pFunctionality as *mut FACTORY)).capacity as
                    libc::c_int) < SIZE_SUPER_HEAVY as libc::c_int {
                //this happends gradually now - AB 5/1/99
				/*subtract the power required to build BEFORE do anything*/
				/*if (!usePower(player, pStructureType->powerToBuild))
				{
					return NULL;
				}*/
				//store the % difference in body points before upgrading
                bodyDiff =
                    (((*psBuilding).body as libc::c_int * 100 as libc::c_int)
                         as
                         libc::c_uint).wrapping_div(structureBody(psBuilding));
                let ref mut fresh6 =
                    (*((*psBuilding).pFunctionality as
                           *mut FACTORY)).capacity;
                *fresh6 = (*fresh6).wrapping_add(1);
                bUpgraded = 1 as libc::c_int;
                //put any production on hold
                holdProduction(psBuilding);
                //quick check not trying to add too much
                if ((*((*psBuilding).pFunctionality as
                           *mut FACTORY)).productionOutput as libc::c_int +
                        (*(*(*pStructureType).asFuncList.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                               as *mut PRODUCTION_FUNCTION)).productionOutput
                            as libc::c_int) < 0xff as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"building factory module - productionOutput too big\x00"
                              as *const u8 as *const libc::c_char);
                };
                if ((*((*psBuilding).pFunctionality as
                           *mut FACTORY)).productionOutput as libc::c_int +
                        (*(*(*pStructureType).asFuncList.offset(0 as
                                                                    libc::c_int
                                                                    as isize)
                               as *mut PRODUCTION_FUNCTION)).productionOutput
                            as libc::c_int) < 0xff as libc::c_int {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 2526 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 15],
                                                    &[libc::c_char; 15]>(b"buildStructure\x00")).as_ptr(),
                          b"((FACTORY*)psBuilding->pFunctionality)->productionOutput + ((PRODUCTION_FUNCTION*)pStructureType->asFuncList[0])-> productionOutput < UBYTE_MAX\x00"
                              as *const u8 as *const libc::c_char);
                };
                let ref mut fresh7 =
                    (*((*psBuilding).pFunctionality as
                           *mut FACTORY)).productionOutput;
                *fresh7 =
                    (*fresh7 as libc::c_int +
                         (*(*(*pStructureType).asFuncList.offset(0 as
                                                                     libc::c_int
                                                                     as isize)
                                as *mut PRODUCTION_FUNCTION)).productionOutput
                             as libc::c_int) as UBYTE;
                //need to change which IMD is used for player 0
				//Need to do a check its not Barbarian really!
                if bMultiPlayer != 0 &&
                       isHumanPlayer((*psBuilding).player as UDWORD) != 0 ||
                       bMultiPlayer != 0 &&
                           game.type_0 as libc::c_int == 14 as libc::c_int &&
                           ((*psBuilding).player as libc::c_int) <
                               game.maxPlayers as libc::c_int ||
                       bMultiPlayer == 0 {
                    //|| (psBuilding->player ==0) )
//#else
//				if (psBuilding->player == 0)
                    capacity =
                        (*((*psBuilding).pFunctionality as
                               *mut FACTORY)).capacity as UDWORD;
                    if capacity < 2 as libc::c_int as libc::c_uint {
                        if (*(*psBuilding).pStructureType).type_0 ==
                               REF_FACTORY as libc::c_int as libc::c_uint {
                            (*psBuilding).sDisplay.imd =
                                factoryModuleIMDs[capacity.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                      as
                                                      usize][0 as libc::c_int
                                                                 as usize]
                        } else {
                            (*psBuilding).sDisplay.imd =
                                factoryModuleIMDs[capacity.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                      as
                                                      usize][1 as libc::c_int
                                                                 as usize]
                        }
                    } else if (*(*psBuilding).pStructureType).type_0 ==
                                  REF_FACTORY as libc::c_int as libc::c_uint {
                        (*psBuilding).sDisplay.imd =
                            factoryModuleIMDs[(2 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  usize][0 as libc::c_int as
                                                             usize]
                    } else {
                        (*psBuilding).sDisplay.imd =
                            factoryModuleIMDs[(2 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  usize][1 as libc::c_int as
                                                             usize]
                    }
                }
            }
        }
        if (*pStructureType).type_0 ==
               REF_RESEARCH_MODULE as libc::c_int as libc::c_uint {
            if (*(*psBuilding).pStructureType).type_0 !=
                   REF_RESEARCH as libc::c_int as libc::c_uint {
                return 0 as *mut STRUCTURE
            }
            //increment the capacity and research points for the owning structure
            if (*((*psBuilding).pFunctionality as
                      *mut RESEARCH_FACILITY)).capacity <
                   4 as libc::c_int as libc::c_uint {
                //this happends gradually now - AB 5/1/99
				/*subtract the power required to build BEFORE do anything*/
				/*if (!usePower(player, pStructureType->powerToBuild))
				{
					return NULL;
				}*/
                //store the % difference in body points before upgrading
                bodyDiff =
                    (((*psBuilding).body as libc::c_int * 100 as libc::c_int)
                         as
                         libc::c_uint).wrapping_div(structureBody(psBuilding));
                //add all the research modules in one go AB 24/06/98
				//((RESEARCH_FACILITY*)psBuilding->pFunctionality)->capacity++;
                (*((*psBuilding).pFunctionality as
                       *mut RESEARCH_FACILITY)).capacity =
                    4 as libc::c_int as UDWORD;
                let ref mut fresh8 =
                    (*((*psBuilding).pFunctionality as
                           *mut RESEARCH_FACILITY)).researchPoints;
                *fresh8 =
                    (*fresh8 as
                         libc::c_uint).wrapping_add((*(*(*pStructureType).asFuncList.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                           as
                                                           *mut RESEARCH_FUNCTION)).researchPoints)
                        as UDWORD as UDWORD;
                bUpgraded = 1 as libc::c_int;
                //cancel any research - put on hold now
                if !(*((*psBuilding).pFunctionality as
                           *mut RESEARCH_FACILITY)).psSubject.is_null() {
                    //cancel the topic
                    //cancelResearch(psBuilding, ((RESEARCH *)((RESEARCH_FACILITY *)psBuilding->
                      //  pFunctionality)->psSubject) - asResearch, psBuilding->player);
                    holdResearch(psBuilding);
                }
                //need to change which IMD is used for player 0
				//Need to do a check its not Barbarian really!
                if bMultiPlayer != 0 &&
                       isHumanPlayer((*psBuilding).player as UDWORD) != 0 ||
                       bMultiPlayer != 0 &&
                           game.type_0 as libc::c_int == 14 as libc::c_int &&
                           ((*psBuilding).player as libc::c_int) <
                               game.maxPlayers as libc::c_int ||
                       bMultiPlayer == 0 {
                    capacity =
                        (*((*psBuilding).pFunctionality as
                               *mut RESEARCH_FACILITY)).capacity;
                    if capacity < 4 as libc::c_int as libc::c_uint {
                        (*psBuilding).sDisplay.imd =
                            researchModuleIMDs[capacity.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                   as usize]
                    } else {
                        (*psBuilding).sDisplay.imd =
                            researchModuleIMDs[(4 as libc::c_int -
                                                    1 as libc::c_int) as
                                                   usize]
                    }
                }
            }
        }
        if (*pStructureType).type_0 ==
               REF_POWER_MODULE as libc::c_int as libc::c_uint {
            if (*(*psBuilding).pStructureType).type_0 !=
                   REF_POWER_GEN as libc::c_int as libc::c_uint {
                return 0 as *mut STRUCTURE
            }
            //increment the capacity and research points for the owning structure
            if (*((*psBuilding).pFunctionality as *mut POWER_GEN)).capacity <
                   4 as libc::c_int as libc::c_uint {
                //this happends gradually now - AB 5/1/99
				/*subtract the power required to build BEFORE do anything*/
				/*if (!usePower(player, pStructureType->powerToBuild))
				{
					return NULL;
				}*/
				//store the % difference in body points before upgrading
                bodyDiff =
                    (((*psBuilding).body as libc::c_int * 100 as libc::c_int)
                         as
                         libc::c_uint).wrapping_div(structureBody(psBuilding));
                //increment the power output, multiplier and capacity
				//add all the research modules in one go AB 24/06/98
				//((POWER_GEN*)psBuilding->pFunctionality)->capacity++;
                (*((*psBuilding).pFunctionality as *mut POWER_GEN)).capacity =
                    4 as libc::c_int as UDWORD;
                let ref mut fresh9 =
                    (*((*psBuilding).pFunctionality as *mut POWER_GEN)).power;
                *fresh9 =
                    (*fresh9 as
                         libc::c_uint).wrapping_add((*(*(*pStructureType).asFuncList.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                           as
                                                           *mut POWER_GEN_FUNCTION)).powerOutput)
                        as UDWORD as UDWORD;
                let ref mut fresh10 =
                    (*((*psBuilding).pFunctionality as
                           *mut POWER_GEN)).multiplier;
                *fresh10 =
                    (*fresh10 as
                         libc::c_uint).wrapping_add((*(*(*pStructureType).asFuncList.offset(0
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize)
                                                           as
                                                           *mut POWER_GEN_FUNCTION)).powerMultiplier)
                        as UDWORD as UDWORD;
                bUpgraded = 1 as libc::c_int;
                //need to change which IMD is used for player 0
				//Need to do a check its not Barbarian really!
                if bMultiPlayer != 0 &&
                       isHumanPlayer((*psBuilding).player as UDWORD) != 0 ||
                       bMultiPlayer != 0 &&
                           game.type_0 as libc::c_int == 14 as libc::c_int &&
                           ((*psBuilding).player as libc::c_int) <
                               game.maxPlayers as libc::c_int ||
                       bMultiPlayer == 0 {
                    //#else
//				if (psBuilding->player == 0)
                    capacity =
                        (*((*psBuilding).pFunctionality as
                               *mut POWER_GEN)).capacity;
                    if capacity < 4 as libc::c_int as libc::c_uint {
                        (*psBuilding).sDisplay.imd =
                            powerModuleIMDs[capacity.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                as usize]
                    } else {
                        (*psBuilding).sDisplay.imd =
                            powerModuleIMDs[(4 as libc::c_int -
                                                 1 as libc::c_int) as usize]
                    }
                }
                //need to inform any res Extr associated that not digging until complete
                releasePowerGen(psBuilding);
            }
        }
        if bUpgraded != 0 {
            //calculate the new body points of the owning structure
			//psBuilding->body = (UWORD)(psBuilding->body  + pStructureType->bodyPoints);
            (*psBuilding).body =
                structureBody(psBuilding).wrapping_mul(bodyDiff).wrapping_div(100
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  libc::c_uint)
                    as UWORD;
            //and the base body points
			//psBuilding->baseBodyPoints += pStructureType->bodyPoints;
			//initialise the build points
            (*psBuilding).currentBuildPts = 0 as libc::c_int as SWORD;
            (*psBuilding).currentPowerAccrued = 0 as libc::c_int as SWORD;
            //start building again
            (*psBuilding).status = SS_BEING_BUILT as libc::c_int as UBYTE;
            if (*psBuilding).player as libc::c_uint == selectedPlayer {
                intRefreshScreen();
            }
            //inform power system that won't be needing power until built
            powerDestroyObject(psBuilding as *mut BASE_OBJECT);
        }
    }
    if (*pStructureType).type_0 != REF_WALL as libc::c_int as libc::c_uint &&
           (*pStructureType).type_0 !=
               REF_WALLCORNER as libc::c_int as libc::c_uint {
        if player == selectedPlayer { scoreUpdateVar(WD_STR_BUILT); }
    }
    /* why is this necessary - it makes tiles under the structure visible */
    setUnderTilesVis(psBuilding as *mut BASE_OBJECT, player);
    return psBuilding;
}
/*
void createAssemblyPoint(STRUCTURE* psStruct)
{
	FACTORY* psFactory;
	UDWORD	x, y;

		psFactory = (FACTORY *)psStruct->pFunctionality;
		addFlagPosition(psFactory->psAssemblyPoint);
		switch(psStruct->pStructureType->type)
		{
		case REF_FACTORY:
			setFlagPositionInc(psFactory, psStruct->player, FACTORY_FLAG);
			break;
		case REF_CYBORG_FACTORY:
			setFlagPositionInc(psFactory, psStruct->player, CYBORG_FLAG);
			break;
		case REF_VTOL_FACTORY:
			setFlagPositionInc(psFactory, psStruct->player, VTOL_FLAG);
			break;
		default:
			ASSERT( FALSE, "setFunctionality: Invalid factory type" );
		}
		//initialise the assembly point position
		x = psStruct->x+256 >> TILE_SHIFT;
		y = psStruct->y+256 >> TILE_SHIFT;
		// Belt and braces - shouldn't be able to build too near edge
		getNearestBestValidTile(&x,&y);
		setAssemblyPoint( psFactory->psAssemblyPoint, x << TILE_SHIFT,
			y << TILE_SHIFT );

}
*/
unsafe extern "C" fn setFunctionality(mut psBuilding: *mut STRUCTURE,
                                      mut functionType: UDWORD) -> BOOL {
    //SDWORD					upgrade;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY; //end of switch
    let mut psResFac: *mut RESEARCH_FACILITY = 0 as *mut RESEARCH_FACILITY;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut psRepairFac: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut pFuncRepair: *mut REPAIR_DROID_FUNCTION =
        0 as *mut REPAIR_DROID_FUNCTION;
    let mut psReArmPad: *mut REARM_PAD = 0 as *mut REARM_PAD;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    (*psBuilding).pFunctionality = 0 as *mut FUNCTIONALITY;
    match functionType {
        1 | 16 | 17 => {
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //allocate the necessary space
			/*psBuilding->pFunctionality = (FUNCTIONALITY *) MALLOC(sizeof(FACTORY));
			if (psBuilding->pFunctionality == NULL)
			{
				debug( LOG_ERROR, "Out of memory" );
				abort();
				return FALSE;
			}*/
			//allocate the necessary space
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            psFactory = (*psBuilding).pFunctionality as *mut FACTORY;
            (*psFactory).capacity =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut PRODUCTION_FUNCTION)).capacity as UBYTE;
            //psFactory->propulsionType = ((PRODUCTION_FUNCTION*)psBuilding->
			//	pStructureType->asFuncList[0])->propulsionType;
            (*psFactory).productionOutput =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut PRODUCTION_FUNCTION)).productionOutput as
                    UBYTE;
            (*psFactory).psSubject = 0 as *mut BASE_STATS;
            //default the secondary order - AB 22/04/99
            (*psFactory).secondaryOrder =
                (DSS_ARANGE_DEFAULT as libc::c_int |
                     DSS_REPLEV_NEVER as libc::c_int |
                     DSS_ALEV_ALWAYS as libc::c_int |
                     DSS_HALT_GUARD as libc::c_int) as UDWORD;
            //add the assembly point to the factory
            if createFlagPosition(&mut (*psFactory).psAssemblyPoint,
                                  (*psBuilding).player as UDWORD) == 0 {
                return 0 as libc::c_int
            }
            //			psFactory->psAssemblyPoint->primary= TRUE;		// set prim. point.
            addFlagPosition((*psFactory).psAssemblyPoint);
            match functionType {
                1 => {
                    setFlagPositionInc(psFactory as *mut libc::c_void,
                                       (*psBuilding).player as UDWORD,
                                       0 as libc::c_int as UBYTE);
                }
                16 => {
                    setFlagPositionInc(psFactory as *mut libc::c_void,
                                       (*psBuilding).player as UDWORD,
                                       1 as libc::c_int as UBYTE);
                }
                17 => {
                    setFlagPositionInc(psFactory as *mut libc::c_void,
                                       (*psBuilding).player as UDWORD,
                                       2 as libc::c_int as UBYTE);
                }
                _ => {
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"setFunctionality: Invalid factory type\x00" as
                                  *const u8 as *const libc::c_char);
                    };
                    if 0 as libc::c_int != 0 {
                    } else {
                        debug(LOG_ERROR,
                              b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"structure.c\x00" as *const u8 as
                                  *const libc::c_char, 2831 as libc::c_int,
                              (*::std::mem::transmute::<&[u8; 17],
                                                        &[libc::c_char; 17]>(b"setFunctionality\x00")).as_ptr(),
                              b"FALSE\x00" as *const u8 as
                                  *const libc::c_char);
                    };
                }
            }
            //initialise the assembly point position
            x =
                ((*psBuilding).x as libc::c_int + 256 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            y =
                ((*psBuilding).y as libc::c_int + 256 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            // Belt and braces - shouldn't be able to build too near edge
			//getNearestBestValidTile(&x,&y);
            setAssemblyPoint((*psFactory).psAssemblyPoint,
                             x << 7 as libc::c_int, y << 7 as libc::c_int,
                             (*psBuilding).player as UDWORD,
                             1 as libc::c_int);
            (*psFactory).psFormation = 0 as *mut _formation;
            //this isn't used anymore-  AB 11/02/99
			//Set the factory group
			/*if (!grpCreate(&psFactory->psGroup))
			{
				DBPRINTF(("setFunctionality: unable to create group\n"));
			}
			else
			{
				//add the factory as a null droid
				grpJoin(psFactory->psGroup, NULL);
			}*/
			//check for upgrades - work backwards since want the most recent
			/*for (upgrade = numProductionUpgrades-1; upgrade >= 0; upgrade --)
			{
				if (apProductionUpgrades[psBuilding->player][upgrade].available)
				{
					productionUpgrade(asFunctions[apProductionUpgrades[
						psBuilding->player][upgrade].functionInc], psBuilding);
					break;
				}
			}*/
            structureProductionUpgrade(psBuilding);
        }
        10 => {
            /*SET IN BUILD STRUCTURE case REF_FACTORY_MODULE:
		{
			// Find the owner structure if any
//psor			psOwner = (STRUCTURE *)mapTile(psBuilding->x >> TILE_SHIFT, psBuilding->
//psor				y >> TILE_SHIFT)->psObject;
			psOwner = getTileStructure(psBuilding->x>>TILE_SHIFT,psBuilding->y>>TILE_SHIFT);
			// Check we've got the right owner type
			if (psOwner && psOwner->type != OBJ_STRUCTURE && psOwner->
				pStructureType->type != REF_FACTORY)
			{
				return FALSE;
			}

			//increment the capacity and output for the owning structure
			if (((FACTORY*)psOwner->pFunctionality)->capacity < SIZE_SUPER_HEAVY)
			{
				((FACTORY*)psOwner->pFunctionality)->capacity++;
			}
			((FACTORY*)psOwner->pFunctionality)->productionOutput += ((PRODUCTION_FUNCTION*)
				psBuilding->pStructureType->asFuncList[0])->productionOutput;
			break;
		}*/
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //allocate the necessary space
			/*psBuilding->pFunctionality = (FUNCTIONALITY *) MALLOC(sizeof(RESEARCH_FACILITY));
			if (psBuilding->pFunctionality == NULL)
			{
				debug( LOG_ERROR, "Out of memory" );
				abort();
				return FALSE;
			}*/
			//try and create the Structure
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            psResFac = (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
            (*psResFac).researchPoints =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut RESEARCH_FUNCTION)).researchPoints;
            //check for upgrades - work backwards since want the most recent
			/*for (upgrade = numResearchUpgrades-1; upgrade >= 0; upgrade --)
			{
				if (apResearchUpgrades[psBuilding->player][upgrade].available)
				{
					researchUpgrade(asFunctions[apResearchUpgrades[
						psBuilding->player][upgrade].functionInc], psBuilding);
					break;
				}
			}*/
            structureResearchUpgrade(psBuilding);
        }
        3 => {
            /*SET IN BUILD STRUCTURE case REF_RESEARCH_MODULE:
		{
			// Find the owner structure if any
//psor			psOwner = (STRUCTURE *)mapTile(psBuilding->x >> TILE_SHIFT, psBuilding->
//psor				y >> TILE_SHIFT)->psObject;
			psOwner = getTileStructure(psBuilding->x>>TILE_SHIFT,psBuilding->y>>TILE_SHIFT);
			// Check we've got the right owner type
			if (psOwner && psOwner->type != OBJ_STRUCTURE && psOwner->
				pStructureType->type != REF_RESEARCH)
			{
				return FALSE;
			}

			//increment the research points for the owning structure
			if (((RESEARCH_FACILITY*)psOwner->pFunctionality)->capacity < 4)
			{
				((RESEARCH_FACILITY*)psOwner->pFunctionality)->capacity++;
			}
			((RESEARCH_FACILITY*)psOwner->pFunctionality)->researchPoints += ((
				RESEARCH_UPGRADE_FUNCTION*)psBuilding->pStructureType->asFuncList[0])->
				researchPoints;
			break;
		}*/
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //allocate the necessary space
			/*psBuilding->pFunctionality = (FUNCTIONALITY *) MALLOC(sizeof(POWER_GEN));
			if (psBuilding->pFunctionality == NULL)
			{
				DBERROR(("Out of memory"));
				return FALSE;
			}*/
			//try and create the Structure
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            psPowerGen = (*psBuilding).pFunctionality as *mut POWER_GEN;
            (*psPowerGen).power =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut POWER_GEN_FUNCTION)).powerOutput;
            (*psPowerGen).multiplier =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut POWER_GEN_FUNCTION)).powerMultiplier;
            (*psPowerGen).capacity = 0 as libc::c_int as UDWORD;
            //check for upgrades
            structurePowerUpgrade(psBuilding);
        }
        5 => {
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //allocate the necessary space
			/*psBuilding->pFunctionality = (FUNCTIONALITY *) MALLOC(sizeof(RES_EXTRACTOR));
			if (psBuilding->pFunctionality == NULL)
			{
				DBERROR(("Out of memory"));
				return FALSE;
			}*/
			//try and create the Structure
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            (*((*psBuilding).pFunctionality as *mut RES_EXTRACTOR)).power =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut RESOURCE_FUNCTION)).maxPower;
            //set the structure going
			//((RES_EXTRACTOR*)psBuilding->pFunctionality)->timeLastUpdated = gameTime;
			//set the structure to inactive
            (*((*psBuilding).pFunctionality as *mut RES_EXTRACTOR)).active =
                0 as libc::c_int;
            let ref mut fresh11 =
                (*((*psBuilding).pFunctionality as
                       *mut RES_EXTRACTOR)).psPowerGen;
            *fresh11 = 0 as *mut _structure
        }
        0 => {
            //this just checks that a function has been assigned
            radarOnScreen = 1 as libc::c_int;
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
        }
        7 => {
            //this structure must have a function assigned to the stat - this is just a check!
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
        }
        12 => {
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //allocate the necessary space
			/*psBuilding->pFunctionality = (FUNCTIONALITY *) MALLOC(sizeof(REPAIR_FACILITY));
			if (psBuilding->pFunctionality == NULL)
			{
				debug( LOG_ERROR, "Out of memory" );
				abort();
				return FALSE;
			}*/
			//try and create the Structure
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            psRepairFac =
                (*psBuilding).pFunctionality as *mut REPAIR_FACILITY;
            pFuncRepair =
                *(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                    as *mut REPAIR_DROID_FUNCTION;
            (*psRepairFac).power = (*pFuncRepair).repairPoints;
            (*psRepairFac).psObj = 0 as *mut BASE_OBJECT;
            if grpCreate(&mut (*((*psBuilding).pFunctionality as
                                     *mut REPAIR_FACILITY)).psGroup) == 0 {
                debug(LOG_NEVER,
                      b"setFunctionality: couldn\'t create repair facility group\x00"
                          as *const u8 as *const libc::c_char);
            } else {
                /* add null droid */
                grpJoin((*psRepairFac).psGroup, 0 as *mut DROID);
            }
            //check for upgrades
            structureRepairUpgrade(psBuilding);
            // add an assembly point
            if createFlagPosition(&mut (*psRepairFac).psDeliveryPoint,
                                  (*psBuilding).player as UDWORD) == 0 {
                return 0 as libc::c_int
            }
            addFlagPosition((*psRepairFac).psDeliveryPoint);
            setFlagPositionInc(psRepairFac as *mut libc::c_void,
                               (*psBuilding).player as UDWORD,
                               3 as libc::c_int as UBYTE);
            //initialise the assembly point position
            x =
                ((*psBuilding).x as libc::c_int + 256 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            y =
                ((*psBuilding).y as libc::c_int + 256 as libc::c_int >>
                     7 as libc::c_int) as UDWORD;
            // Belt and braces - shouldn't be able to build too near edge
			//getNearestBestValidTile(&x,&y);
            setAssemblyPoint((*psRepairFac).psDeliveryPoint,
                             x << 7 as libc::c_int, y << 7 as libc::c_int,
                             (*psBuilding).player as UDWORD,
                             1 as libc::c_int);
        }
        19 => {
            //this structure must have a function assigned to the stat
            if (*(*psBuilding).pStructureType).numFuncs ==
                   0 as libc::c_int as libc::c_uint {
                debug(LOG_ERROR,
                      b"There must be a function assigned to this building - %s\x00"
                          as *const u8 as *const libc::c_char,
                      getName((*(*psBuilding).pStructureType).pName));
                abort();
            }
            //try and create the Structure
            if createStructFunc(&mut (*psBuilding).pFunctionality) == 0 {
                debug(LOG_ERROR,
                      b"Out of memory\x00" as *const u8 as
                          *const libc::c_char);
                abort();
            }
            //initialise the memory
            memset((*psBuilding).pFunctionality as *mut libc::c_void,
                   0 as libc::c_int,
                   ::std::mem::size_of::<FUNCTIONALITY>() as libc::c_ulong);
            psReArmPad = (*psBuilding).pFunctionality as *mut REARM_PAD;
            (*psReArmPad).reArmPoints =
                (*(*(*(*psBuilding).pStructureType).asFuncList.offset(0 as
                                                                          libc::c_int
                                                                          as
                                                                          isize)
                       as *mut REARM_PAD)).reArmPoints;
            //check for upgrades
            structureReArmUpgrade(psBuilding);
        }
        _ => { }
    }
    return 1 as libc::c_int;
}
/*
//this is surplus to requirements now!
// Using pickATile - find the nearest best tile to the one we've specified.
void	getNearestBestValidTile(UDWORD *x, UDWORD *y)
{
BOOL	gotPos;
UBYTE	iterations;

	// PickAtile - will succeed even for 0 iterations if tile is valid
	iterations = 0;
	gotPos = 0;
	// Keep going until we get it
	while(!gotPos)
	{
		// Have another go...
		gotPos = pickATileGen(x,y,iterations++,normalPAT);
	}
}
*/
// Set the command droid that factory production should go to
#[no_mangle]
pub unsafe extern "C" fn assignFactoryCommandDroid(mut psStruct:
                                                       *mut STRUCTURE,
                                                   mut psCommander:
                                                       *mut DROID) {
    let mut psFact: *mut FACTORY = 0 as *mut FACTORY;
    let mut psFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut psNext: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut psPrev: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut factoryInc: SDWORD = 0;
    let mut typeFlag: SDWORD = 0;
    if StructIsFactory(psStruct) != 0 {
    } else {
        debug(LOG_ERROR,
              b"assignFactoryCommandUnit: structure not a factory\x00" as
                  *const u8 as *const libc::c_char);
    };
    if StructIsFactory(psStruct) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              3202 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"assignFactoryCommandDroid\x00")).as_ptr(),
              b"StructIsFactory(psStruct)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFact = (*psStruct).pFunctionality as *mut FACTORY;
    match (*(*psStruct).pStructureType).type_0 {
        1 => { typeFlag = 0 as libc::c_int }
        17 => { typeFlag = 2 as libc::c_int }
        16 => { typeFlag = 1 as libc::c_int }
        _ => {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"assignfactorycommandUnit: unknown factory type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      3218 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 26],
                                                &[libc::c_char; 26]>(b"assignFactoryCommandDroid\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            typeFlag = 0 as libc::c_int
        }
    }
    // removing a commander from a factory
    if !(*psFact).psCommander.is_null() {
        if typeFlag == 0 as libc::c_int {
            secondarySetState((*psFact).psCommander, DSO_CLEAR_PRODUCTION,
                              ((1 as libc::c_int) <<
                                   (*(*psFact).psAssemblyPoint).factoryInc as
                                       libc::c_int + 9 as libc::c_int) as
                                  SECONDARY_STATE);
        } else if typeFlag == 1 as libc::c_int {
            secondarySetState((*psFact).psCommander, DSO_CLEAR_PRODUCTION,
                              ((1 as libc::c_int) <<
                                   (*(*psFact).psAssemblyPoint).factoryInc as
                                       libc::c_int +
                                       (9 as libc::c_int + 5 as libc::c_int))
                                  as SECONDARY_STATE);
        } else {
            secondarySetState((*psFact).psCommander, DSO_CLEAR_PRODUCTION,
                              ((1 as libc::c_int) <<
                                   (*(*psFact).psAssemblyPoint).factoryInc as
                                       libc::c_int + 24 as libc::c_int) as
                                  SECONDARY_STATE);
        }
        (*psFact).psCommander = 0 as *mut _droid;
        if missionIsOffworld() == 0 {
            addFlagPosition((*psFact).psAssemblyPoint);
            // add the assembly point back into the list
        } else {
            (*(*psFact).psAssemblyPoint).psNext =
                mission.apsFlagPosLists[(*(*psFact).psAssemblyPoint).player as
                                            usize];
            mission.apsFlagPosLists[(*(*psFact).psAssemblyPoint).player as
                                        usize] = (*psFact).psAssemblyPoint
        }
    }
    if !psCommander.is_null() {
        // adding a commander to a factory
/*		if ( psFact->psCommander != NULL )			// have to remove the factory from any previous commander
		{
			if (typeFlag == FACTORY_FLAG)
			{
				secondarySetState(psFact->psCommander, DSO_CLEAR_PRODUCTION,
						1 << ( psFact->psAssemblyPoint->factoryInc + DSS_ASSPROD_SHIFT) );
			}
			else if (typeFlag == CYBORG_FLAG)
			{
				secondarySetState(psFact->psCommander, DSO_CLEAR_PRODUCTION,
						1 << ( psFact->psAssemblyPoint->factoryInc + DSS_ASSPROD_CYBORG_SHIFT) );
			}
			else
			{
				secondarySetState(psFact->psCommander, DSO_CLEAR_PRODUCTION,
						1 << ( psFact->psAssemblyPoint->factoryInc + DSS_ASSPROD_VTOL_SHIFT) );
			}

			psFact->psCommander = NULL;				// remove commander from a factory
			addFlagPosition(psFact->psAssemblyPoint);// add the assembly point back into the list (only temporaryily.)
		}*/
        if missionIsOffworld() == 0 {
        } else {
            debug(LOG_ERROR,
                  b"assignFactoryCommandDroid: cannot assign a commander to a factory when off world\x00"
                      as *const u8 as *const libc::c_char);
        };
        if missionIsOffworld() == 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  3280 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 26],
                                            &[libc::c_char; 26]>(b"assignFactoryCommandDroid\x00")).as_ptr(),
                  b"!missionIsOffworld()\x00" as *const u8 as
                      *const libc::c_char);
        };
        factoryInc = (*(*psFact).psAssemblyPoint).factoryInc as SDWORD;
        psPrev = 0 as *mut FLAG_POSITION;
        psFlag = apsFlagPosLists[(*psStruct).player as usize];
        while !psFlag.is_null() {
            psNext = (*psFlag).psNext;
            if (*psFlag).factoryInc as libc::c_int == factoryInc &&
                   (*psFlag).factoryType as libc::c_int == typeFlag {
                if psFlag != (*psFact).psAssemblyPoint {
                    removeFlagPosition(psFlag);
                } else {
                    // need to keep the assembly point(s) for the factory
					// but remove it(the primary) from the list so it doesn't get
					// displayed
                    if psPrev.is_null() {
                        apsFlagPosLists[(*psStruct).player as usize] =
                            (*psFlag).psNext
                    } else { (*psPrev).psNext = (*psFlag).psNext }
                    (*psFlag).psNext = 0 as *mut _flag_position
                }
            } else { psPrev = psFlag }
            psFlag = psNext
        }
        //		psFact->psAssemblyPoint->factorySub = 0;			// reset factory count of subpoints.
        (*psFact).psCommander = psCommander
    };
}
// remove all factories from a command droid
#[no_mangle]
pub unsafe extern "C" fn clearCommandDroidFactory(mut psDroid: *mut DROID) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    psCurr = apsStructLists[selectedPlayer as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_FACTORY as libc::c_int as libc::c_uint ||
               (*(*psCurr).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
               (*(*psCurr).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
            if (*((*psCurr).pFunctionality as *mut FACTORY)).psCommander ==
                   psDroid {
                assignFactoryCommandDroid(psCurr, 0 as *mut _droid);
            }
        }
        psCurr = (*psCurr).psNext
    }
    psCurr = mission.apsStructLists[selectedPlayer as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_FACTORY as libc::c_int as libc::c_uint ||
               (*(*psCurr).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
               (*(*psCurr).pStructureType).type_0 ==
                   REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
            if (*((*psCurr).pFunctionality as *mut FACTORY)).psCommander ==
                   psDroid {
                assignFactoryCommandDroid(psCurr, 0 as *mut _droid);
            }
        }
        psCurr = (*psCurr).psNext
    };
}
/* Check that a tile is vacant for a droid to be placed */
unsafe extern "C" fn structClearTile(mut x: UWORD, mut y: UWORD) -> BOOL {
    let mut player: UDWORD = 0;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    /* Check for a structure */
//psor	if (mapTile(x,y)->psObject != NULL)
    if fpathBlockingTile.expect("non-null function pointer")(x as SDWORD,
                                                             y as SDWORD) != 0
       {
        return 0 as libc::c_int
    }
    /* Check for a droid */
    player = 0 as libc::c_int as UDWORD;
    while player < 8 as libc::c_int as libc::c_uint {
        psCurr = apsDroidLists[player as usize];
        while !psCurr.is_null() {
            if (*psCurr).x as libc::c_int >> 7 as libc::c_int ==
                   x as libc::c_int &&
                   (*psCurr).y as libc::c_int >> 7 as libc::c_int ==
                       y as libc::c_int {
                return 0 as libc::c_int
            }
            psCurr = (*psCurr).psNext
        }
        player = player.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*find a location near to a structure to start the droid of*/
#[no_mangle]
pub unsafe extern "C" fn placeDroid(mut psStructure: *mut STRUCTURE,
                                    mut droidX: *mut UDWORD,
                                    mut droidY: *mut UDWORD) -> BOOL {
    let mut sx: SWORD = 0;
    let mut sy: SWORD = 0;
    let mut xmin: SWORD = 0;
    let mut xmax: SWORD = 0;
    let mut ymin: SWORD = 0;
    let mut ymax: SWORD = 0;
    let mut x: SWORD = 0;
    let mut y: SWORD = 0;
    let mut xmid: SWORD = 0;
    let mut placed: BOOL = 0;
    /* Get the tile coords for the top left of the structure */
    sx =
        ((*psStructure).x as
             libc::c_uint).wrapping_sub((*(*psStructure).pStructureType).baseWidth.wrapping_mul(128
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint))
            as SWORD;
    sx = (sx as libc::c_int >> 7 as libc::c_int) as SWORD;
    sy =
        ((*psStructure).y as
             libc::c_uint).wrapping_sub((*(*psStructure).pStructureType).baseBreadth.wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_uint))
            as SWORD;
    sy = (sy as libc::c_int >> 7 as libc::c_int) as SWORD;
    /* Find the four corners of the square */
    xmin = (sx as libc::c_int - 1 as libc::c_int) as SWORD;
    xmax =
        (sx as
             libc::c_uint).wrapping_add((*(*psStructure).pStructureType).baseWidth)
            as SWORD;
    xmid =
        (sx as
             libc::c_uint).wrapping_add((*(*psStructure).pStructureType).baseWidth.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint).wrapping_div(2
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint))
            as SWORD;
    ymin = (sy as libc::c_int - 1 as libc::c_int) as SWORD;
    ymax =
        (sy as
             libc::c_uint).wrapping_add((*(*psStructure).pStructureType).baseBreadth)
            as SWORD;
    if (xmin as libc::c_int) < 0 as libc::c_int {
        xmin = 0 as libc::c_int as SWORD
    }
    if xmax as libc::c_int > mapWidth as SDWORD { xmax = mapWidth as SWORD }
    if (ymin as libc::c_int) < 0 as libc::c_int {
        ymin = 0 as libc::c_int as SWORD
    }
    if ymax as libc::c_int > mapHeight as SDWORD { ymax = mapHeight as SWORD }
    /* Look for a clear location for the droid across the bottom */
	/* start in the middle */
    placed = 0 as libc::c_int;
    y = ymax;
    /* middle to right */
    x = xmid;
    while (x as libc::c_int) < xmax as libc::c_int {
        if structClearTile(x as UWORD, y as UWORD) != 0 {
            placed = 1 as libc::c_int;
            break ;
        } else { x += 1 }
    }
    /* left to middle */
    if placed == 0 {
        x = xmin;
        while (x as libc::c_int) < xmid as libc::c_int {
            if structClearTile(x as UWORD, y as UWORD) != 0 {
                placed = 1 as libc::c_int;
                break ;
            } else { x += 1 }
        }
    }
    /* across the top */
    if placed == 0 {
        y = ymin;
        x = xmin;
        while (x as libc::c_int) < xmax as libc::c_int {
            if structClearTile(x as UWORD, y as UWORD) != 0 {
                placed = 1 as libc::c_int;
                break ;
            } else { x += 1 }
        }
    }
    /* the left */
    if placed == 0 {
        x = xmin;
        y = ymin;
        while (y as libc::c_int) < ymax as libc::c_int {
            if structClearTile(x as UWORD, y as UWORD) != 0 {
                placed = 1 as libc::c_int;
                break ;
            } else { y += 1 }
        }
    }
    /* the right */
    if placed == 0 {
        x = xmax;
        y = ymin;
        while (y as libc::c_int) < ymax as libc::c_int {
            if structClearTile(x as UWORD, y as UWORD) != 0 {
                placed = 1 as libc::c_int;
                break ;
            } else { y += 1 }
        }
    }
    *droidX = x as UDWORD;
    *droidY = y as UDWORD;
    return placed;
}
/* Place a newly manufactured droid next to a factory  and then send if off
to the assembly point*/
unsafe extern "C" fn structPlaceDroid(mut psStructure: *mut STRUCTURE,
                                      mut psTempl: *mut DROID_TEMPLATE,
                                      mut ppsDroid: *mut *mut DROID) {
    let mut x: UDWORD = 0; //bTemp = FALSE;
    let mut y: UDWORD = 0;
    let mut placed: BOOL = 0;
    let mut psNewDroid: *mut DROID = 0 as *mut DROID;
    let mut psFact: *mut FACTORY = 0 as *mut FACTORY;
    let mut apx: SDWORD = 0;
    let mut apy: SDWORD = 0;
    let mut psFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    //	UDWORD			i;
    let mut iVecEffect: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut factoryType: UBYTE = 0;
    let mut assignCommander: BOOL = 0;
    //STRUCTURE       *psReArmPad;
    //	DBPRINTF(("Called structPlaceDroid.\n");
    /* set blocking tile before placing droid */
//	fpathSetBlockingTile( asPropulsionStats[psTempl->asParts[COMP_PROPULSION]].propulsionType );
    placed = placeDroid(psStructure, &mut x, &mut y);
    if placed != 0 {
        //create a droid near to the structure
        psNewDroid =
            buildDroid(psTempl, x << 7 as libc::c_int, y << 7 as libc::c_int,
                       (*psStructure).player as UDWORD, 0 as libc::c_int);
        if psNewDroid.is_null() { *ppsDroid = 0 as *mut DROID; return }
        //set the droids order to that of the factory - AB 22/04/99
        (*psNewDroid).secondaryOrder =
            (*((*psStructure).pFunctionality as *mut FACTORY)).secondaryOrder;
        if bMultiPlayer != 0 { sendDroidSecondaryAll(psNewDroid); }
        if (*psStructure).visible[selectedPlayer as usize] != 0 {
            /* add smoke effect to cover the droid's emergence from the factory */
            iVecEffect.x = (*psNewDroid).x as int32;
            iVecEffect.y =
                map_Height((*psNewDroid).x as UDWORD,
                           (*psNewDroid).y as UDWORD) as libc::c_int +
                    20 as libc::c_int;
            iVecEffect.z = (*psNewDroid).y as int32;
            addEffect(&mut iVecEffect, EFFECT_CONSTRUCTION,
                      CONSTRUCTION_TYPE_DRIFTING, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            iVecEffect.x = (*psNewDroid).x as libc::c_int - 30 as libc::c_int;
            iVecEffect.z = (*psNewDroid).y as libc::c_int - 30 as libc::c_int;
            addEffect(&mut iVecEffect, EFFECT_CONSTRUCTION,
                      CONSTRUCTION_TYPE_DRIFTING, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            iVecEffect.z = (*psNewDroid).y as libc::c_int + 30 as libc::c_int;
            addEffect(&mut iVecEffect, EFFECT_CONSTRUCTION,
                      CONSTRUCTION_TYPE_DRIFTING, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            iVecEffect.x = (*psNewDroid).x as libc::c_int + 30 as libc::c_int;
            addEffect(&mut iVecEffect, EFFECT_CONSTRUCTION,
                      CONSTRUCTION_TYPE_DRIFTING, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            iVecEffect.z = (*psNewDroid).y as libc::c_int - 30 as libc::c_int;
            addEffect(&mut iVecEffect, EFFECT_CONSTRUCTION,
                      CONSTRUCTION_TYPE_DRIFTING, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
        }
        /* add the droid to the list */
        addDroid(psNewDroid, apsDroidLists.as_mut_ptr());
        *ppsDroid = psNewDroid;
        if (*psNewDroid).player as libc::c_uint == selectedPlayer {
            audio_QueueTrack(ID_SOUND_DROID_COMPLETED as libc::c_int);
            intRefreshScreen();
            // update any interface implications.
        }
        // update the droid counts
        incNumDroids((*psNewDroid).player as UDWORD);
        match (*psNewDroid).droidType as libc::c_uint {
            7 => { incNumCommandDroids((*psNewDroid).player as UDWORD); }
            3 | 10 => {
                incNumConstructorDroids((*psNewDroid).player as UDWORD);
            }
            _ => { }
        }
        psFact = (*psStructure).pFunctionality as *mut FACTORY;
        apx = (*(*psFact).psAssemblyPoint).coords.x;
        apy = (*(*psFact).psAssemblyPoint).coords.y;
        //		orderDroidLoc(psNewDroid, DORDER_MOVE, apx,apy);		// factory waypoints handle this now.
        //add it to the factory group
/*		if (psNewDroid->droidType != DROID_COMMAND)
		{
			if (!psFact->psGroup)
			{
				//create the factory group
				if (!grpCreate(&psFact->psGroup))
				{
					DBPRINTF(("structPlaceDroid: unable to create group\n"));
				}
				else
				{
					grpJoin(psFact->psGroup, psNewDroid);
				}
			}
			else
			{
				grpJoin(psFact->psGroup, psNewDroid);
			}
		}*/
        // if we've built a command droid - make sure that it isn't assigned to another commander
        assignCommander = 0 as libc::c_int;
        if (*psNewDroid).droidType as libc::c_uint ==
               DROID_COMMAND as libc::c_int as libc::c_uint &&
               !(*psFact).psCommander.is_null() {
            assignFactoryCommandDroid(psStructure, 0 as *mut _droid);
            assignCommander = 1 as libc::c_int
        }
        if !(*psFact).psCommander.is_null() {
            if idfDroid(psNewDroid) != 0 || vtolDroid(psNewDroid) != 0 {
                orderDroidObj(psNewDroid, DORDER_FIRESUPPORT,
                              (*psFact).psCommander as *mut BASE_OBJECT);
                moveToRearm(psNewDroid);
            } else {
                cmdDroidAddDroid((*psFact).psCommander, psNewDroid);
                /*				orderDroidLoc(psNewDroid, DORDER_MOVE,
								psFact->psCommander->x,
								psFact->psCommander->y);*/
            }
        } else {
            //check flag against factory type
            factoryType = 0 as libc::c_int as UBYTE;
            if (*(*psStructure).pStructureType).type_0 ==
                   REF_CYBORG_FACTORY as libc::c_int as libc::c_uint {
                factoryType = 1 as libc::c_int as UBYTE
            } else if (*(*psStructure).pStructureType).type_0 ==
                          REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
                factoryType = 2 as libc::c_int as UBYTE
            }
            //if vtol droid - send it to ReArm Pad if one exists
            placed = 0 as libc::c_int;
            if vtolDroid(psNewDroid) != 0 {
                moveToRearm(psNewDroid);
                /*                psReArmPad = findNearestReArmPad(psNewDroid, NULL, FALSE);
                if (psReArmPad)
                {
                    orderDroidLoc(psNewDroid, DORDER_MOVE, psReArmPad->x, psReArmPad->y);
                    placed = TRUE;
                    //assign rearmPad to the VTOL
                    assignVTOLPad(psNewDroid, psReArmPad);
                }*/
            }
            if placed == 0 {
                //find flag in question.
                psFlag =
                    apsFlagPosLists[(*(*psFact).psAssemblyPoint).player as
                                        usize];
                while !((*psFlag).factoryInc as libc::c_int ==
                            (*(*psFact).psAssemblyPoint).factoryInc as
                                libc::c_int &&
                            (*psFlag).factoryType as libc::c_int ==
                                factoryType as libc::c_int) {
                    psFlag = (*psFlag).psNext
                }
                if (*psNewDroid).droidType as libc::c_uint ==
                       DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                    let mut droidX: UDWORD = (*psFlag).coords.x as UDWORD;
                    let mut droidY: UDWORD = (*psFlag).coords.y as UDWORD;
                    /*			    // check and process factory waypoints for this droid.
			    // order droid to each in turn.
			    for(i = 0; i <= psFact->psAssemblyPoint->factorySub; i++)
			    {
				    //find flag in question.
				    for(psFlag = apsFlagPosLists[psFact->psAssemblyPoint->player];
					    !( (psFlag->factoryInc == psFact->psAssemblyPoint->factoryInc)	// correct fact.
					      &&(psFlag->factoryType == factoryType)						// correct type
					      &&(psFlag->factorySub == i));									// correct flag.
					    psFlag = psFlag->psNext);

#ifndef PSX
					    if(bMultiPlayer)
					    {
						    bMultiPlayer = FALSE;
						    bTemp = TRUE;
						    SendSingleDroidWaypoint(psNewDroid,psFlag->coords.x,psFlag->coords.y);
					    }
					    else
					    {
						    bTemp = FALSE;
					    }
					    orderAddWayPoint(psNewDroid ,psFlag->coords.x,psFlag->coords.y);
					    if(bTemp)
					    {
						    bMultiPlayer = TRUE;
					    }
#else
    					// psx version, only one delivery point, no waypoint, so just order droid.
	    				orderDroidLoc(psNewDroid,DORDER_MOVE,psFlag->coords.x,psFlag->coords.y);
//#endif
               }
			*/
                    //find a suitable location near the delivery point
                    actionVTOLLandingPos(psNewDroid, &mut droidX,
                                         &mut droidY);
                    actionDroidLoc(psNewDroid, DACTION_MOVE, droidX, droidY);
                } else {
                    orderDroidLoc(psNewDroid, DORDER_MOVE,
                                  (*psFlag).coords.x as UDWORD,
                                  (*psFlag).coords.y as UDWORD);
                }
            }
        }
        if assignCommander != 0 {
            assignFactoryCommandDroid(psStructure, psNewDroid);
        }
        /*

		// remove any out of date formation
		if (psFact->psFormation &&
			(psFact->psFormation->x != apx || psFact->psFormation->y != apy))
		{
			formationLeave(psFact->psFormation, (BASE_OBJECT *)psStructure);
			psFact->psFormation = NULL;
		}
		// setup the formation for the droid if necessary
		if (psFact->psFormation == NULL)
		{
			if (formationNew(&psFact->psFormation, FT_LINE, apx,apy,
					(SDWORD)calcDirection((SDWORD)psStructure->x,(SDWORD)psStructure->y, apx,apy)))
			{
				formationJoin(psFact->psFormation, (BASE_OBJECT *)psStructure);
			}
		}
		// add the droid to the formation
		if (psFact->psFormation)
		{
			formationJoin(psFact->psFormation, (BASE_OBJECT *)psNewDroid);
			psNewDroid->sMove.psFormation = psFact->psFormation;
		}
*/
        if (*psNewDroid).player as libc::c_uint == selectedPlayer {
            eventFireCallbackTrigger(CALL_DROIDBUILT as libc::c_int as
                                         TRIGGER_TYPE);
        }
        psScrCBNewDroid = psNewDroid;
        psScrCBNewDroidFact = psStructure;
        eventFireCallbackTrigger(CALL_NEWDROID as libc::c_int as
                                     TRIGGER_TYPE);
        psScrCBNewDroid = 0 as *mut DROID;
        psScrCBNewDroidFact = 0 as *mut STRUCTURE
    } else { *ppsDroid = 0 as *mut DROID };
}
unsafe extern "C" fn IsFactoryCommanderGroupFull(mut psFactory: *mut FACTORY)
 -> BOOL {
    let mut DroidsInGroup: SDWORD = 0;
    // If we don't have a commander return FALSE (group not full)
    if (*psFactory).psCommander.is_null() { return 0 as libc::c_int }
    // allow any number of IDF droids
    if templateIsIDF((*psFactory).psSubject as *mut DROID_TEMPLATE) != 0 {
        return 0 as libc::c_int
    }
    // Get the number of droids in the commanders group
    DroidsInGroup =
        if !(*(*psFactory).psCommander).psGroup.is_null() {
            grpNumMembers((*(*psFactory).psCommander).psGroup)
        } else { 0 as libc::c_int };
    // if the number in group is less than the maximum allowed then return FALSE (group not full)
    if DroidsInGroup < cmdDroidMaxGroup((*psFactory).psCommander) {
        return 0 as libc::c_int
    }
    // the number in group has reached the maximum
    return 1 as libc::c_int;
}
// Disallow manufacture of units once these limits are reached,
// dos'nt mean that these numbers can't be exceeded if units are
// put down in the editor or by the scripts.
static mut MaxDroidsAllowedPerPlayer: [UWORD; 8] =
    [100 as libc::c_int as UWORD, 999 as libc::c_int as UWORD,
     999 as libc::c_int as UWORD, 999 as libc::c_int as UWORD,
     999 as libc::c_int as UWORD, 999 as libc::c_int as UWORD,
     999 as libc::c_int as UWORD, 999 as libc::c_int as UWORD];
static mut MaxDroidsAllowedPerPlayerMultiPlayer: [UWORD; 8] =
    [300 as libc::c_int as UWORD, 300 as libc::c_int as UWORD,
     300 as libc::c_int as UWORD, 300 as libc::c_int as UWORD,
     300 as libc::c_int as UWORD, 300 as libc::c_int as UWORD,
     300 as libc::c_int as UWORD, 300 as libc::c_int as UWORD];
//static UWORD MaxDroidsAllowedPerPlayerMultiPlayer[MAX_PLAYERS]={10,10,10,10,10,10,10,10};
#[no_mangle]
pub unsafe extern "C" fn getMaxStructures(mut PlayerNumber: UDWORD)
 -> UDWORD {
    // PC currently doesn't limit number of structures a player can build, so just
	// return an absurdly large number.
    return 99999 as libc::c_int as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn IsPlayerStructureLimitReached(mut PlayerNumber:
                                                           UDWORD) -> BOOL {
    // PC currently doesn't limit number of structures a player can build.
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getMaxDroids(mut PlayerNumber: UDWORD) -> UDWORD {
    return if bMultiPlayer != 0 {
               MaxDroidsAllowedPerPlayerMultiPlayer[PlayerNumber as usize] as
                   libc::c_int
           } else {
               MaxDroidsAllowedPerPlayer[PlayerNumber as usize] as libc::c_int
           } as UDWORD;
}
#[no_mangle]
pub unsafe extern "C" fn IsPlayerDroidLimitReached(mut PlayerNumber: UDWORD)
 -> BOOL {
    if getNumDroids(PlayerNumber).wrapping_add(getNumMissionDroids(PlayerNumber)).wrapping_add(getNumTransporterDroids(PlayerNumber))
           >=
           (if bMultiPlayer != 0 {
                MaxDroidsAllowedPerPlayerMultiPlayer[PlayerNumber as usize] as
                    libc::c_int
            } else {
                MaxDroidsAllowedPerPlayer[PlayerNumber as usize] as
                    libc::c_int
            }) as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn maxDroidsByTypeReached(mut psStructure:
                                                    *mut STRUCTURE) -> BOOL {
    let mut psFact: *mut FACTORY =
        (*psStructure).pFunctionality as *mut FACTORY;
    if droidTemplateType((*psFact).psSubject as *mut DROID_TEMPLATE) as
           libc::c_uint == DROID_COMMAND as libc::c_int as libc::c_uint &&
           getNumCommandDroids((*psStructure).player as UDWORD) >=
               10 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    if (droidTemplateType((*psFact).psSubject as *mut DROID_TEMPLATE) as
            libc::c_uint == DROID_CONSTRUCT as libc::c_int as libc::c_uint ||
            droidTemplateType((*psFact).psSubject as *mut DROID_TEMPLATE) as
                libc::c_uint ==
                DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint) &&
           getNumConstructorDroids((*psStructure).player as UDWORD) >=
               15 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Check for max number of units reached and halt production.
//
#[no_mangle]
pub unsafe extern "C" fn CheckHaltOnMaxUnitsReached(mut psStructure:
                                                        *mut STRUCTURE)
 -> BOOL {
    // if the players that owns the factory has reached his (or hers) droid limit
	// then put production on hold & return - we need a message to be displayed here !!!!!!!
    if IsPlayerDroidLimitReached((*psStructure).player as UDWORD) != 0 ||
           maxDroidsByTypeReached(psStructure) != 0 {
        if (*psStructure).player as libc::c_uint == selectedPlayer &&
               lastMaxUnitMessage.wrapping_add(20000 as libc::c_int as
                                                   libc::c_uint) < gameTime {
            addConsoleMessage(strresGetString(psStringRes,
                                              STR_GAM_MAXUNITSREACHED as
                                                  libc::c_int as UDWORD),
                              DEFAULT_JUSTIFY);
            lastMaxUnitMessage = gameTime
        }
        /*
#ifdef DEBUG
		else
		{
			addConsoleMessage("DEBUG:OPPONENT HAS REACHED DROID LIMIT",DEFAULT_JUSTIFY);
		}
#endif*/
        //		holdProduction(psStructure);
        return 1 as libc::c_int
    } //, iPower;
    return 0 as libc::c_int; //bFinishRepair;
}
#[no_mangle]
pub unsafe extern "C" fn aiUpdateStructure(mut psStructure: *mut STRUCTURE) {
    let mut pSubject: *mut BASE_STATS = 0 as *mut BASE_STATS;
    let mut pointsToAdd: UDWORD = 0;
    let mut pPlayerRes: *mut PLAYER_RESEARCH =
        asPlayerResList[(*psStructure).player as usize];
    let mut structureMode: UDWORD = 0 as libc::c_int as UDWORD;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut psChosenObj: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut Quantity: UBYTE = 0;
    let mut iDt: SDWORD = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psRepairFac: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut psResFacility: *mut RESEARCH_FACILITY =
        0 as *mut RESEARCH_FACILITY;
    let mut psReArmPad: *mut REARM_PAD = 0 as *mut REARM_PAD;
    let mut iVecEffect: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut bFinishAction: BOOL = 0;
    let mut psWStats: *mut WEAPON_STATS = 0 as *mut WEAPON_STATS;
    let mut psTarget: *mut BASE_OBJECT = 0 as *mut BASE_OBJECT;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut mindist: SDWORD = 0;
    let mut currdist: SDWORD = 0;
    let mut psNextTemplate: *mut DROID_TEMPLATE = 0 as *mut DROID_TEMPLATE;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"aiUpdateStructure: invalid Structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              3908 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"aiUpdateStructure\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if !(*psStructure).psTarget.is_null() &&
           (*(*psStructure).psTarget).died != 0 {
        (*psStructure).psTarget = 0 as *mut BASE_OBJECT
    }
    // Will go out into a building EVENT stats/text file
	/* Spin round yer sensors! */
	//if (psStructure->numWeaps == 0)
    if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint &&
           (*(*psStructure).pStructureType).type_0 !=
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
        // ////
// - radar should rotate every three seconds ... 'cause we timed it at Heathrow !
// gameTime is in milliseconds - one rotation every 3 seconds = 1 rotation event 3000 millisecs
        (*psStructure).turretRotation =
            gameTime.wrapping_mul(360 as libc::c_int as
                                      libc::c_uint).wrapping_div(3000 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint).wrapping_rem(360
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                as UWORD;
        (*psStructure).turretPitch = 0 as libc::c_int as UWORD
    }
    structUpdateRecoil(psStructure);
    /* See if there is an enemy to attack */
	//if (psStructure->numWeaps > 0) - don't bother looking for a target if Las Sat weapon
    if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint &&
           (*asWeaponStats.offset((*psStructure).asWeaps[0 as libc::c_int as
                                                             usize].nStat as
                                      isize)).weaponSubClass as libc::c_uint
               != WSC_LAS_SAT as libc::c_int as libc::c_uint {
        if (*psStructure).id.wrapping_rem(20 as libc::c_int as libc::c_uint)
               ==
               frameGetFrameNumber().wrapping_rem(20 as libc::c_int as
                                                      libc::c_uint) {
            if aiChooseTarget(psStructure as *mut BASE_OBJECT,
                              &mut psChosenObj) != 0 {
                (*psStructure).psTarget = psChosenObj
            } else {
                (*psStructure).psTarget = 0 as *mut BASE_OBJECT;
                psChosenObj = 0 as *mut BASE_OBJECT
            }
        } else { psChosenObj = (*psStructure).psTarget }
        if !psChosenObj.is_null() {
            // get the weapon stat to see if there is a visible turret to rotate
            psWStats =
                asWeaponStats.offset((*psStructure).asWeaps[0 as libc::c_int
                                                                as
                                                                usize].nStat
                                         as isize);
            //if were going to shoot at something move the turret first then fire when locked on
            if (*psWStats).pMountGraphic.is_null() {
                //no turret so lock on whatever
                (*psStructure).turretRotation =
                    calcDirection((*psStructure).x as UDWORD,
                                  (*psStructure).y as UDWORD,
                                  (*psChosenObj).x as UDWORD,
                                  (*psChosenObj).y as UDWORD) as UWORD;
                combFire((*psStructure).asWeaps.as_mut_ptr(),
                         psStructure as *mut BASE_OBJECT, psChosenObj);
            } else if actionTargetTurret(psStructure as *mut BASE_OBJECT,
                                         psChosenObj,
                                         &mut (*psStructure).turretRotation,
                                         &mut (*psStructure).turretPitch,
                                         psWStats, 0 as libc::c_int) != 0 {
                combFire((*psStructure).asWeaps.as_mut_ptr(),
                         psStructure as *mut BASE_OBJECT, psChosenObj);
            }
        } else if (*psStructure).turretRotation as libc::c_int %
                      90 as libc::c_int != 0 as libc::c_int ||
                      (*psStructure).turretPitch as libc::c_int !=
                          0 as libc::c_int {
            actionAlignTurret(psStructure as *mut BASE_OBJECT);
        }
    } else if !(*(*psStructure).pStructureType).pSensor.is_null() {
        if structStandardSensor(psStructure) != 0 ||
               structVTOLSensor(psStructure) != 0 {
            if (*psStructure).id.wrapping_rem(20 as libc::c_int as
                                                  libc::c_uint) ==
                   frameGetFrameNumber().wrapping_rem(20 as libc::c_int as
                                                          libc::c_uint) {
                if aiChooseSensorTarget(psStructure as *mut BASE_OBJECT,
                                        &mut psChosenObj) != 0 {
                    (*psStructure).psTarget = psChosenObj
                } else { (*psStructure).psTarget = 0 as *mut BASE_OBJECT }
            }
            psChosenObj = (*psStructure).psTarget
        } else { psChosenObj = (*psStructure).psTarget }
        /*else if(actionTargetTurret((BASE_OBJECT*)psStructure, psChosenObj,
									&(psStructure->turretRotation),
									&(psStructure->turretPitch),psStructure->turretRotRate,
									(UWORD)(psStructure->turretRotRate/2),
									proj_Direct(psWStats),   FALSE))*/
        // realign the turret
        /* See if there is an enemy to attack for Sensor Towers that have weapon droids attached*/
        // you can always see anything that a CB sensor is targeting
		// Anyone commenting this out again will get a knee capping from John.
		// You have been warned!!
        if (structCBSensor(psStructure) != 0 ||
                structVTOLCBSensor(psStructure) != 0) &&
               !(*psStructure).psTarget.is_null() {
            (*(*psStructure).psTarget).visible[(*psStructure).player as usize]
                = 0xff as libc::c_int as UBYTE
        }
    }
    //only interested if the Structure "does" something!
    if (*psStructure).pFunctionality.is_null() { return }
    //check if any power available
    if structUsesPower(psStructure) != 0 {
        //	    if ((asPower[psStructure->player]->currentPower > POWER_PER_CYCLE) OR
//		    (!powerCalculated))
        if checkPower((*psStructure).player as UDWORD,
                      5 as libc::c_int as UDWORD, 0 as libc::c_int) != 0 {
            //check if this structure is due some power
            if getLastPowered(psStructure as *mut BASE_OBJECT) != 0 {
                //DBPRINTF(("pStructureType  %d\n",psStructure->pStructureType->type));
//if(psStructure->pStructureType->type == REF_FACTORY) {
//	FACTORY *Fact = psStructure->pFunctionality;
//	DBPRINTF(("Subject %p\n",Fact->psSubject));
//	DBPRINTF(("Accrued %d\n",Fact->powerAccrued));
//	DBPRINTF(("Started %d\n",Fact->timeStarted));
//	DBPRINTF(("Start Hold %d\n",Fact->timeStartHold));
//	if(Fact->psSubject) {
//		DBPRINTF(("Required %d\n",((DROID_TEMPLATE*)Fact->psSubject)->powerPoints ));
//	}
//}
                //get some power if necessary
                if accruePower(psStructure as *mut BASE_OBJECT) != 0 {
                    updateLastPowered(psStructure as *mut BASE_OBJECT,
                                      (*psStructure).player);
                }
            }
        }
    }
    /* Process the functionality according to type
	 * determine the subject stats (for research or manufacture)
	 * or base object (for repair) or update power levels for resourceExtractor
	 */
    match (*(*psStructure).pStructureType).type_0 {
        10 => {
            /*case REF_RESOURCE_EXTRACTOR:
			updateExtractedPower(psStructure);
			break;*/
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
        12 => {
            psRepairFac =
                (*psStructure).pFunctionality as *mut REPAIR_FACILITY;
            psChosenObj = (*psRepairFac).psObj;
            structureMode = REF_REPAIR_FACILITY as libc::c_int as UDWORD;
            psDroid = psChosenObj as *mut DROID;
            // skip droids that are doing anything else
            if !psDroid.is_null() &&
                   (orderState(psDroid, DORDER_RTR) == 0 ||
                        (*psDroid).psTarget !=
                            psStructure as *mut BASE_OBJECT) {
                //				if(psDroid->psGroup != NULL)
//				{
//					grpLeave(psRepairFac->psGroup, (DROID *)psChosenObj);
//				}
                psChosenObj = 0 as *mut BASE_OBJECT;
                psDroid = 0 as *mut DROID;
                (*psRepairFac).psObj = 0 as *mut BASE_OBJECT
            }
            /* select next droid if none being repaired */
            if psChosenObj.is_null() {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"aiUpdateStructure: invalid repair facility group pointer\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 4120 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 18],
                                                    &[libc::c_char; 18]>(b"aiUpdateStructure\x00")).as_ptr(),
                          b"PTRVALID( psRepairFac->psGroup, sizeof(DROID_GROUP) )\x00"
                              as *const u8 as *const libc::c_char);
                };
                // get droid next in repair queue
/*				changed this just to scan for the first droid waiting for repair
				cos using groups here causes all sorts of problems elsewhere. John
				while ( (psRepairFac->psGroup != NULL) &&
						(psRepairFac->psGroup->psList != NULL) )
				{
					psDroid = psRepairFac->psGroup->psList;
					iPower = (psDroid->originalBody - psDroid->body) *
								REPAIR_FACILITY_COST_PERCENT / 100;

					// only move to repair if droid still waiting else remove from group
					if ( psDroid->order != DORDER_RTR ||
						 !checkPower( psStructure->player, iPower, TRUE ) )
					{
						grpLeave( psRepairFac->psGroup, psDroid );
						// return droid back to front of repair facility
						orderDroidLoc( psDroid, DORDER_MOVE, psStructure->x,
						psStructure->y +
						(psStructure->pStructureType->baseBreadth<<TILE_SHIFT)/2 + 2*TILE_UNITS );
					}
					else
					{
						break;
					}
				}*/
                mindist = 0x7fffffff as libc::c_int;
                psDroid = apsDroidLists[(*psStructure).player as usize];
                while !psDroid.is_null() {
                    if orderStateObj(psDroid, DORDER_RTR, &mut psTarget) != 0
                           && psTarget == psStructure as *mut BASE_OBJECT &&
                           (*psDroid).action ==
                               DACTION_WAITFORREPAIR as libc::c_int {
                        xdiff =
                            (*psDroid).x as SDWORD -
                                (*psStructure).x as SDWORD;
                        ydiff =
                            (*psDroid).y as SDWORD -
                                (*psStructure).y as SDWORD;
                        currdist = xdiff * xdiff + ydiff * ydiff;
                        if currdist < mindist {
                            mindist = currdist;
                            psChosenObj = psDroid as *mut BASE_OBJECT
                        }
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = psChosenObj as *mut DROID
            }
            // send the droid to be repaired
            if !psDroid.is_null() &&
                   (*psDroid).action == DACTION_WAITFORREPAIR as libc::c_int {
                /* set chosen object */
                psChosenObj = psDroid as *mut BASE_OBJECT;
                (*psRepairFac).psObj = psDroid as *mut BASE_OBJECT;
                /* move droid to repair point at rear of facility */
/*				actionDroidObjLoc( psDroid, DACTION_MOVETOREPAIRPOINT,
						(BASE_OBJECT *) psStructure, psStructure->x,
						psStructure->y -
						(psStructure->pStructureType->baseBreadth<<TILE_SHIFT)/2 -
						TILE_UNITS/2 );*/
                actionDroidObjLoc(psDroid, DACTION_MOVETOREPAIRPOINT,
                                  psStructure as *mut BASE_OBJECT,
                                  (*psStructure).x as UDWORD,
                                  (*psStructure).y as UDWORD);
                /* reset repair started */
                (*psRepairFac).timeStarted = 0 as libc::c_int as UDWORD;
                (*psRepairFac).currentPtsAdded = 0 as libc::c_int as UDWORD
            }
        }
        19 => {
            psReArmPad = (*psStructure).pFunctionality as *mut REARM_PAD;
            psChosenObj = (*psReArmPad).psObj;
            structureMode = REF_REARM_PAD as libc::c_int as UDWORD;
            psDroid = 0 as *mut DROID;
            /* select next droid if none being rearmed*/
            if psChosenObj.is_null() {
                psDroid = apsDroidLists[(*psStructure).player as usize];
                while !psDroid.is_null() {
                    // move next droid waiting on ground to rearm pad
                    if vtolReadyToRearm(psDroid, psStructure) != 0 &&
                           (psChosenObj.is_null() ||
                                (*(psChosenObj as *mut DROID)).actionStarted >
                                    (*psDroid).actionStarted) {
                        psChosenObj = psDroid as *mut BASE_OBJECT
                    }
                    psDroid = (*psDroid).psNext
                }
                psDroid = psChosenObj as *mut DROID;
                if !psDroid.is_null() {
                    actionDroidObj(psDroid, DACTION_MOVETOREARMPOINT,
                                   psStructure as *mut BASE_OBJECT);
                }
            } else {
                psDroid = psChosenObj as *mut DROID;
                if ((*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
                        ||
                        (*psDroid).sMove.Status as libc::c_int ==
                            8 as libc::c_int) &&
                       (*psDroid).action ==
                           DACTION_WAITFORREARM as libc::c_int {
                    actionDroidObj(psDroid, DACTION_MOVETOREARMPOINT,
                                   psStructure as *mut BASE_OBJECT);
                }
            }
            // if found a droid to rearm assign it to the rearm pad
            if !psDroid.is_null() {
                /* set chosen object */
                psChosenObj = psDroid as *mut BASE_OBJECT;
                (*psReArmPad).psObj = psChosenObj;
                if (*psDroid).action ==
                       DACTION_MOVETOREARMPOINT as libc::c_int {
                    /* reset rearm started */
                    (*psReArmPad).timeStarted = 0 as libc::c_int as UDWORD;
                    (*psReArmPad).currentPtsAdded = 0 as libc::c_int as UDWORD
                }
            }
        }
        _ => { }
    }
    /* check subject stats (for research or manufacture) */
    if !pSubject.is_null() {
        //if subject is research...
        if structureMode == REF_RESEARCH as libc::c_int as libc::c_uint {
            psResFacility =
                (*psStructure).pFunctionality as *mut RESEARCH_FACILITY;
            //if on hold don't do anything
            if (*psResFacility).timeStartHold != 0 { return }
            //electronic warfare affects the functionality of some structures in multiPlayer
            if bMultiPlayer != 0 {
                if ((*psStructure).resistance as libc::c_int) <
                       structureResistance((*psStructure).pStructureType,
                                           (*psStructure).player) as SWORD as
                           libc::c_int {
                    return
                }
            }
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
                // don't update if not responsible (106)
                if bMultiPlayer != 0 &&
                       myResponsibility((*psStructure).player as UDWORD) == 0
                   {
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
                    /*Done in research Result now - AB 31/1/98
					gameTimeStop();
					audio_PlayTrack( ID_SOUND_MAJOR_RESEARCH );
					gameTimeStart();*/
					//pPlayerRes->researched = RESEARCHED;
                    if bMultiPlayer != 0 {
                        SendResearch((*psStructure).player,
                                     (*pSubject).ref_0.wrapping_sub(0xb0000 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint));
                    }
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
            //electronic warfare affects the functionality of some structures in multiPlayer
            if bMultiPlayer != 0 {
                if ((*psStructure).resistance as libc::c_int) <
                       structureResistance((*psStructure).pStructureType,
                                           (*psStructure).player) as SWORD as
                           libc::c_int {
                    return
                }
            }
            if (*psFactory).timeStarted == 0 as libc::c_int as libc::c_uint {
                /*if (Quantity == NON_STOP_PRODUCTION)
				{
					if (!checkPower(psStructure->player,
						((DROID_TEMPLATE *)pSubject)->powerPoints, FALSE))
					{
						return;
					}
				}*/
                // also need to check if a command droid's group is full
                // If the factory commanders group is full - return
                if IsFactoryCommanderGroupFull(psFactory) == 1 as libc::c_int
                   {
                    return
                }
                if CheckHaltOnMaxUnitsReached(psStructure) == 1 as libc::c_int
                   {
                    return
                }
                /*subtract the power required to build*/
				/*if (usePower(psStructure->player, ((DROID_TEMPLATE *)pSubject)->
					powerPoints))
				{
					//set the time started
					psFactory->timeStarted = gameTime;
				}
				else
				{
					if(psStructure->player == selectedPlayer)
					{
						addConsoleMessage("Droid build: No Power",
							DEFAULT_JUSTIFY);
					}
					//not got enough power so cancel
					psFactory->quantity = 0;
					psFactory->psSubject = NULL;
					intManufactureFinished(psStructure);
					return;
				}*/
            }
            //check enough power has accrued to build the droid
            if (*psFactory).powerAccrued <
                   (*(pSubject as *mut DROID_TEMPLATE)).powerPoints {
                //wait until enough power
                return
            }
            /*must be enough power so subtract that required to build*/
            if (*psFactory).timeStarted == 0 as libc::c_int as libc::c_uint {
                /*if (usePower(psStructure->player, ((DROID_TEMPLATE *)pSubject)->
					powerPoints))
				{
					//set the time started
					psFactory->timeStarted = gameTime;
				}
				else
				{
					//This shouldn't happen so cancel!
					psFactory->quantity = 0;
					psFactory->psSubject = NULL;
					intManufactureFinished(psStructure);
					return;
				}*/
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
            if pointsToAdd > (*psFactory).timeToBuild &&
                   IsFactoryCommanderGroupFull(psFactory) == 0 &&
                   CheckHaltOnMaxUnitsReached(psStructure) == 0 {
                /* Place the droid on the map */
                structPlaceDroid(psStructure, pSubject as *mut DROID_TEMPLATE,
                                 &mut psDroid);
                //reset the start time
                (*psFactory).timeStarted = 0 as libc::c_int as UDWORD;
                (*psFactory).powerAccrued = 0 as libc::c_int as UDWORD;
                //next bit for productionPlayer only
                if productionPlayer as libc::c_int ==
                       (*psStructure).player as libc::c_int {
                    psNextTemplate =
                        factoryProdUpdate(psStructure,
                                          pSubject as *mut DROID_TEMPLATE);
                    if !psNextTemplate.is_null() {
                        structSetManufacture(psStructure, psNextTemplate,
                                             Quantity);
                        return
                    } else {
                        //nothing more to manufacture - reset the Subject and Tab on HCI Form
                        intManufactureFinished(psStructure);
                        (*psFactory).psSubject = 0 as *mut BASE_STATS
                    }
                } else {
                    //decrement the quantity to manufacture if not set to infinity
                    if Quantity as libc::c_int != 0 &&
                           Quantity as libc::c_int !=
                               10 as libc::c_int + 1 as libc::c_int {
                        (*psFactory).quantity =
                            (*psFactory).quantity.wrapping_sub(1);
                        Quantity = Quantity.wrapping_sub(1)
                    }
                    // If quantity not 0 then kick of another manufacture.
                    if Quantity != 0 {
                        // Check enough power to continue production run.
						/*if (!checkPower(psStructure->player, ((DROID_TEMPLATE *)pSubject)->
							powerPoints, (Quantity != NON_STOP_PRODUCTION)))
						{
							// If not then stop the run. - only if not set to NON_STOP PRODUCTION
							if (Quantity != NON_STOP_PRODUCTION)
							{
								psFactory->quantity = 0;
								psFactory->psSubject = NULL;
								intManufactureFinished(psStructure);
								//playerNewDroid(psDroid);
							}
							return;
						}*/
                        // Manufacture another.
                        structSetManufacture(psStructure,
                                             pSubject as *mut DROID_TEMPLATE,
                                             Quantity);
                        //playerNewDroid(psDroid);
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
    /* check base object (for repair / rearm) */
    if !psChosenObj.is_null() {
        if structureMode == REF_REPAIR_FACILITY as libc::c_int as libc::c_uint
           {
            let mut powerCost: UDWORD = 0; //, iPower;
            psDroid = psChosenObj as *mut DROID;
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"aiUpdateStructure: invalid droid pointer\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      4540 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"aiUpdateStructure\x00")).as_ptr(),
                      b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                          *const libc::c_char);
            };
            psRepairFac =
                (*psStructure).pFunctionality as *mut REPAIR_FACILITY;
            if (*psDroid).action == DACTION_WAITDURINGREPAIR as libc::c_int &&
                   actionTargetTurret(psStructure as *mut BASE_OBJECT,
                                      psChosenObj,
                                      &mut (*psStructure).turretRotation,
                                      &mut (*psStructure).turretPitch,
                                      0 as *mut WEAPON_STATS,
                                      0 as libc::c_int) != 0 {
                //Do repair gradually as power comes in
//				POWER REQUIRMENTS REMOVED - AB  22/09/98 - and back again 07/01/99
                /*iPower = powerReqForDroidRepair(psDroid);
				//check to see if enough power to research has accrued
				if (psRepairFac->powerAccrued < iPower)
				{
					//wait until enough power
					return;
				}*/
                //check droid is not healthy
                if (*psDroid).body < (*psDroid).originalBody {
                    //if in multiPlayer, and a Transporter - make sure its on the ground before repairing
                    if bMultiPlayer != 0 &&
                           (*psDroid).droidType as libc::c_uint ==
                               DROID_TRANSPORTER as libc::c_int as
                                   libc::c_uint {
                        if !((*psDroid).sMove.Status as libc::c_int ==
                                 0 as libc::c_int &&
                                 (*psDroid).sMove.iVertSpeed as libc::c_int ==
                                     0 as libc::c_int) {
                            return
                        }
                    }
                    //don't do anything if the resistance is low in multiplayer
                    if bMultiPlayer != 0 {
                        if ((*psStructure).resistance as libc::c_int) <
                               structureResistance((*psStructure).pStructureType,
                                                   (*psStructure).player) as
                                   SWORD as libc::c_int {
                            return
                        }
                    }
                    //check if enough power to do any
                    //powerCost = (powerReqForDroidRepair(psDroid) -
                      //  psDroid->powerAccrued) / POWER_FACTOR;
                    powerCost = powerReqForDroidRepair(psDroid) as UDWORD;
                    if powerCost > (*psDroid).powerAccrued as libc::c_uint {
                        powerCost =
                            powerCost.wrapping_sub((*psDroid).powerAccrued as
                                                       libc::c_uint).wrapping_div(100
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
                    } else { powerCost = 0 as libc::c_int as UDWORD }
                    //if the power cost is 0 (due to rounding) then do for free!
                    if powerCost != 0 {
                        if (*psDroid).powerAccrued == 0 {
                            //reset the actionStarted time and actionPoints added so the correct
                            //amount of points are added when there is more power
                            (*psRepairFac).timeStarted = gameTime;
                            //init so repair points to add won't be huge when start up again
                            (*psRepairFac).currentPtsAdded =
                                0 as libc::c_int as UDWORD;
                            return
                        }
                    }
                    if (*psRepairFac).timeStarted ==
                           0 as libc::c_int as libc::c_uint {
                        //must be enough power so subtract that required to research
					    /*if (usePower(psStructure->player, iPower))
					    {
						    //set the time started
						    psRepairFac->timeStarted = gameTime;
					    }
					    else
					    {
						    //This shouldn't happen so cancel!
						    psRepairFac->psObj = NULL;
						    return;
					    }*/
					    //set the time started
                        (*psRepairFac).timeStarted = gameTime;
                        //reset the points added
                        (*psRepairFac).currentPtsAdded =
                            0 as libc::c_int as UDWORD
                    }
                    /* do repairing */
                    iDt =
                        gameTime.wrapping_sub((*psRepairFac).timeStarted) as
                            SDWORD;
                    //- this was a bit exponential ...
                    pointsToAdd =
                        (iDt as
                             libc::c_uint).wrapping_mul((*psRepairFac).power).wrapping_div(1000
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               libc::c_uint).wrapping_sub((*psRepairFac).currentPtsAdded);
                    //bFinishRepair = FALSE;
                    bFinishAction = 0 as libc::c_int;
                    //do some repair
                    if pointsToAdd != 0 {
                        //just add the points if the power cost is negligable
                        //if these points would make the droid healthy again then just add
                        if powerCost == 0 ||
                               (*psDroid).body.wrapping_add(pointsToAdd) >=
                                   (*psDroid).originalBody {
                            //anothe HACK but sorts out all the rounding errors when values get small
                            (*psDroid).body =
                                ((*psDroid).body as
                                     libc::c_uint).wrapping_add(pointsToAdd)
                                    as UDWORD as UDWORD;
                            (*psRepairFac).currentPtsAdded =
                                ((*psRepairFac).currentPtsAdded as
                                     libc::c_uint).wrapping_add(pointsToAdd)
                                    as UDWORD as UDWORD
                        } else {
                            //see if we have enough power to do this amount of repair
                            powerCost =
                                pointsToAdd.wrapping_mul(repairPowerPoint(psDroid)
                                                             as libc::c_uint);
                            if powerCost <=
                                   (*psDroid).powerAccrued as libc::c_uint {
                                (*psDroid).body =
                                    ((*psDroid).body as
                                         libc::c_uint).wrapping_add(pointsToAdd)
                                        as UDWORD as UDWORD;
                                (*psRepairFac).currentPtsAdded =
                                    ((*psRepairFac).currentPtsAdded as
                                         libc::c_uint).wrapping_add(pointsToAdd)
                                        as UDWORD as UDWORD;
                                //subtract the power cost for these points
                                (*psDroid).powerAccrued =
                                    ((*psDroid).powerAccrued as
                                         libc::c_uint).wrapping_sub(powerCost)
                                        as UWORD as UWORD
                            } else {
                                /*reset the actionStarted time and actionPoints added so the correct
                                amount of points are added when there is more power*/
                                (*psRepairFac).timeStarted = gameTime;
                                (*psRepairFac).currentPtsAdded =
                                    0 as libc::c_int as UDWORD
                            }
                        }
                    }
                }
                if (*psDroid).body >= (*psDroid).originalBody {
                    debug(LOG_NEVER,
                          b"aiUpdateStructure: repair completed\n\x00" as
                              *const u8 as *const libc::c_char);
                    (*psRepairFac).psObj = 0 as *mut BASE_OBJECT;
                    /* set droid points to max */
                    (*psDroid).body = (*psDroid).originalBody;
                    //reset the power accrued
                    (*psDroid).powerAccrued = 0 as libc::c_int as UWORD;
                    /* flag repair finished */
			 //		bFinishRepair = TRUE;
                    // if completely repaired reset order
                    secondarySetState(psDroid, DSO_RETURN_TO_LOC,
                                      0 as SECONDARY_STATE);
                    if !(*psDroid).psGroup.is_null() &&
                           (*(*psDroid).psGroup).type_0 as libc::c_int ==
                               GT_COMMAND as libc::c_int &&
                           (*psDroid).droidType as libc::c_uint !=
                               DROID_COMMAND as libc::c_int as libc::c_uint {
                        // return a droid to it's command group
                        let mut psCommander: *mut DROID =
                            (*(*psDroid).psGroup).psCommander;
                        //						orderDroidLoc(psDroid, DORDER_MOVE, psCommander->x, psCommander->y);
                        orderDroidObj(psDroid, DORDER_GUARD,
                                      psCommander as *mut BASE_OBJECT);
                    } else if !(*psRepairFac).psDeliveryPoint.is_null() {
                        // move the droid out the way
                        orderDroidLoc(psDroid, DORDER_MOVE,
                                      (*(*psRepairFac).psDeliveryPoint).coords.x
                                          as UDWORD,
                                      (*(*psRepairFac).psDeliveryPoint).coords.y
                                          as UDWORD);
                        /*						orderDroidLoc( psDroid, DORDER_MOVE, psStructure->x,
							psStructure->y +
							(psStructure->pStructureType->baseBreadth<<TILE_SHIFT)/2 + 2*TILE_UNITS );*/
//						moveShuffleDroid(psDroid, -TILE_UNITS, 0);
                    }
                }
                /* check whether repair finished */
				/*if ( bFinishRepair == TRUE )
				{
					// release droid from structure
					((REPAIR_FACILITY*)psStructure->pFunctionality)->psObj = NULL;
#ifndef PSX
					// This is very new Alex M - Ask John????
			   //		if(psDroid->psOldCommander AND !psDroid->psOldCommander->died)
			   //		{
			   //			cmdDroidAddDroid(psDroid->psOldCommander, psDroid);
			   //		}
					// This is very new Alex M - Ask John????
#endif
				}
				else*/
                if (*psStructure).visible[selectedPlayer as usize] as
                       libc::c_int != 0 &&
                       (*psDroid).visible[selectedPlayer as usize] as
                           libc::c_int != 0 {
                    /* add plasma repair effect whilst being repaired */
                    iVecEffect.x =
                        (*psDroid).x as libc::c_int +
                            (10 as libc::c_int - rand() % 20 as libc::c_int);
                    iVecEffect.y =
                        (*psDroid).z as libc::c_int +
                            (10 as libc::c_int - rand() % 20 as libc::c_int);
                    iVecEffect.z =
                        (*psDroid).y as libc::c_int +
                            (10 as libc::c_int - rand() % 20 as libc::c_int);
                    effectSetSize(100 as libc::c_int as UDWORD);
                    addEffect(&mut iVecEffect, EFFECT_EXPLOSION,
                              EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                              getImdFromIndex(MI_FLAME as libc::c_int as
                                                  UDWORD), 0 as libc::c_int);
                }
            }
        } else if structureMode ==
                      REF_REARM_PAD as libc::c_int as libc::c_uint {
            psReArmPad = (*psStructure).pFunctionality as *mut REARM_PAD;
            psDroid = psChosenObj as *mut DROID;
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"aiUpdateStructure: invalid droid pointer\x00" as
                          *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      4747 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"aiUpdateStructure\x00")).as_ptr(),
                      b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                          *const libc::c_char);
            };
            if vtolDroid(psDroid) != 0 {
            } else {
                debug(LOG_ERROR,
                      b"aiUpdateStructure: invalid droid type\x00" as
                          *const u8 as *const libc::c_char);
            };
            if vtolDroid(psDroid) != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      4748 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"aiUpdateStructure\x00")).as_ptr(),
                      b"vtolDroid(psDroid)\x00" as *const u8 as
                          *const libc::c_char);
            };
            //check for rearming
            // //check for full vtol droid being assigned
			//else if (!vtolEmpty(psDroid))
			//check for fully armed and repaired vtol droid being assigned
/*			else if (vtolHappy(psDroid))
			{
				// release droid from structure
				psReArmPad->psObj = NULL;
			}*/
			/*else if (psDroid->action == DACTION_NONE)
			{
				// release droid from structure
				psReArmPad->psObj = NULL;
				// halt droid if no order set else assume was waiting for rearm
				if ( psDroid->order == DORDER_NONE )
				{
					actionDroid(psDroid, DACTION_NONE );
				}
				else
				{
					actionDroidObj(psDroid, DACTION_MOVETOREARM, (BASE_OBJECT *)psStructure);
				}
			}*/
            if (*psDroid).died != 0 ||
                   (*psDroid).action !=
                       DACTION_MOVETOREARMPOINT as libc::c_int &&
                       (*psDroid).action !=
                           DACTION_WAITDURINGREARM as libc::c_int {
                (*psReArmPad).psObj = 0 as *mut BASE_OBJECT;
                return
            }
            if (*psDroid).action == DACTION_WAITDURINGREARM as libc::c_int &&
                   (*psDroid).sMove.Status as libc::c_int == 0 as libc::c_int
               {
                if (*psReArmPad).timeStarted ==
                       0 as libc::c_int as libc::c_uint {
                    //check hasn't died whilst waiting to be rearmed
			// also clear out any previously repaired droid
                    //if waiting to be rearmed
                    //set the time started
                    (*psReArmPad).timeStarted = gameTime
                }
                bFinishAction = 0 as libc::c_int;
                if bMultiPlayer == 0 ||
                       myResponsibility((*psDroid).player as UDWORD) != 0 {
                    /* check whether rearm finished */
                    // dont rearm on remote pcs.
                    /* do rearming */
                    if (*psDroid).sMove.iAttackRuns as libc::c_int !=
                           0 as libc::c_int {
                        let mut pointsRequired: UDWORD = 0;
                        //amount required is a factor of the droids' weight
                        pointsRequired =
                            (*psDroid).weight.wrapping_div(10 as libc::c_int
                                                               as
                                                               libc::c_uint);
                        pointsToAdd =
                            (*psReArmPad).reArmPoints.wrapping_mul(gameTime.wrapping_sub((*psReArmPad).timeStarted)).wrapping_div(1000
                                                                                                                                      as
                                                                                                                                      libc::c_int
                                                                                                                                      as
                                                                                                                                      libc::c_uint);
                        //if ((SDWORD)(psDroid->sMove.iAttackRuns - pointsToAdd) <= 0)
                        if pointsToAdd >= pointsRequired {
                            /* set rearm value to no runs made */
                            (*psDroid).sMove.iAttackRuns =
                                0 as libc::c_int as UWORD;
                            //reset ammo and lastTimeFired
                            (*psDroid).asWeaps[0 as libc::c_int as usize].ammo
                                =
                                (*asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize].nStat
                                                           as
                                                           isize)).numRounds
                                    as UDWORD;
                            (*psDroid).asWeaps[0 as libc::c_int as
                                                   usize].lastFired =
                                0 as libc::c_int as UDWORD
                        } else if pointsToAdd >=
                                      pointsRequired.wrapping_div((*psDroid).sMove.iAttackRuns
                                                                      as
                                                                      libc::c_uint)
                         {
                            (*psDroid).sMove.iAttackRuns =
                                (*psDroid).sMove.iAttackRuns.wrapping_sub(1)
                        }
                    }
                }
                if (*psDroid).body < (*psDroid).originalBody {
                    pointsToAdd =
                        (10 as libc::c_int as
                             libc::c_uint).wrapping_mul(gameTime.wrapping_sub((*psReArmPad).timeStarted)).wrapping_div(1000
                                                                                                                           as
                                                                                                                           libc::c_int
                                                                                                                           as
                                                                                                                           libc::c_uint);
                    /* do repairing */
                    //this was exponential...
					//psDroid->body += pointsToAdd;
                    if pointsToAdd.wrapping_sub((*psReArmPad).currentPtsAdded)
                           > 0 as libc::c_int as libc::c_uint {
                        (*psDroid).body =
                            ((*psDroid).body as
                                 libc::c_uint).wrapping_add(pointsToAdd.wrapping_sub((*psReArmPad).currentPtsAdded))
                                as UDWORD as UDWORD;
                        (*psReArmPad).currentPtsAdded = pointsToAdd
                    }
                    if (*psDroid).body >= (*psDroid).originalBody {
                        /* set droid points to max */
                        (*psDroid).body = (*psDroid).originalBody
                    }
                }
                if vtolHappy(psDroid) != 0 {
                    //check for fully armed and fully repaired
                    /*if ((psDroid->psGroup != NULL) &&
						(psDroid->psGroup->type == GT_COMMAND) &&
						(psDroid->droidType != DROID_COMMAND))
					{
						// return a droid to it's command group
						DROID	*psCommander = psDroid->psGroup->psCommander;
						orderDroidLoc(psDroid, DORDER_MOVE, psCommander->x, psCommander->y);

					}
					else
					{
						//don't use an order since could be in the middle of a FIRE_SUPPORT order
						actionDroid(psDroid, DACTION_NONE);
						//if not on fire support duty return back to previous place
						if (psDroid->order != DORDER_FIRESUPPORT)
						{
							//don't remove target
							//psDroid->psTarget = NULL;
							secondarySetState(psDroid, DSO_RETURN_TO_LOC, 0);
						}
					}*/
                    /*move the droid off the pad - don't want it to follow Commander - it
                    just has to sit next to the rearm pad until it gets another target
                    By moving the vtol to the rearm pad it will actually move it off it!*/
                    //if the droid no longer has a target move it back to its base ReArm Pad
                    /*  Totally ditch all this now - John
						The action just gets set to DACTION_NONE - the droid's order can
						then catch this and tell it what to do if anything.
						The droid will also now just sit on the pad until another droid arrives
                    if (psDroid->psTarget == NULL OR (psDroid->order ==
                        DORDER_FIRESUPPORT AND psDroid->psActionTarget == NULL))
                    {
                        if (psDroid->psBaseStruct)
                        {
                            actionDroidLoc(psDroid, DACTION_MOVE,
                                psDroid->psBaseStruct->x, psDroid->psBaseStruct->y);
                        }
                        else
                        {
                            //just move it off
                            actionDroidLoc(psDroid, DACTION_MOVE, psStructure->x,
                                psStructure->y);
                        }
                    }
                    else
                    {
                        //just move it off
                        actionDroidLoc(psDroid, DACTION_MOVE, psStructure->x,
                            psStructure->y);
                    }*/
                    if bMultiPlayer != 0 { sendHappyVtol(psDroid); }
                    //clear the rearm pad
                    (*psDroid).action = DACTION_NONE as libc::c_int;
                    bFinishAction = 1 as libc::c_int;
                    (*psReArmPad).psObj = 0 as *mut BASE_OBJECT
                }
            }
        }
    };
}
/* Decides whether a structure should emit smoke when it's damaged */
#[no_mangle]
pub unsafe extern "C" fn canSmoke(mut psStruct: *mut STRUCTURE) -> BOOL {
    if (*(*psStruct).pStructureType).type_0 ==
           REF_WALL as libc::c_int as libc::c_uint ||
           (*(*psStruct).pStructureType).type_0 ==
               REF_WALLCORNER as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
/* The main update routine for all Structures */
#[no_mangle]
pub unsafe extern "C" fn structureUpdate(mut psBuilding: *mut STRUCTURE) {
    let mut widthScatter: UDWORD = 0;
    let mut breadthScatter: UDWORD = 0;
    let mut percentDamage: UDWORD = 0;
    let mut emissionInterval: UDWORD = 0;
    let mut iPointsToAdd: UDWORD = 0;
    let mut iPointsRequired: UDWORD = 0;
    let mut dv: iVector = iVector{x: 0, y: 0, z: 0,};
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
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              4960 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 16],
                                        &[libc::c_char; 16]>(b"structureUpdate\x00")).as_ptr(),
              b"PTRVALID(psBuilding, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    //update the manufacture/research of the building once complete
    if (*psBuilding).status as libc::c_int == SS_BUILT as libc::c_int {
        aiUpdateStructure(psBuilding);
    }
    if (*psBuilding).status as libc::c_int != SS_BUILT as libc::c_int {
        if (*psBuilding).selected != 0 {
            (*psBuilding).selected = 0 as libc::c_int as UBYTE
        }
    }
    //--
    /* Only add smoke if they're visible and they can 'burn' */
    if (*psBuilding).visible[selectedPlayer as usize] as libc::c_int != 0 &&
           canSmoke(psBuilding) != 0 {
        //percentDamage = (100 - PERCENT(psBuilding->body, psBuilding->
		//	baseBodyPoints));
        percentDamage =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_sub((((*psBuilding).body as
                                                  libc::c_int *
                                                  100 as libc::c_int) as
                                                 libc::c_uint).wrapping_div(structureBody(psBuilding)));
        // Is there any damage?
        if percentDamage != 0 as libc::c_int as libc::c_uint {
            if percentDamage >= 100 as libc::c_int as libc::c_uint {
                percentDamage = 99 as libc::c_int as UDWORD
            }
            emissionInterval =
                (100 as libc::c_int as
                     libc::c_uint).wrapping_sub(percentDamage).wrapping_add(10
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_uint).wrapping_div(10
                                                                                                               as
                                                                                                               libc::c_int
                                                                                                               as
                                                                                                               libc::c_uint).wrapping_mul(400
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              libc::c_uint);
            //if(gameTime > (psBuilding->lastEmission+psBuilding->emissionInterval))
            if gameTime >
                   (*psBuilding).lastEmission.wrapping_add(emissionInterval) {
                widthScatter =
                    (*(*psBuilding).pStructureType).baseWidth.wrapping_mul(128
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint).wrapping_div(2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_uint).wrapping_div(3
                                                                                                                                             as
                                                                                                                                             libc::c_int
                                                                                                                                             as
                                                                                                                                             libc::c_uint);
                breadthScatter =
                    (*(*psBuilding).pStructureType).baseBreadth.wrapping_mul(128
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_div(2
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint).wrapping_div(3
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_uint);
                dv.x =
                    ((*psBuilding).x as
                         libc::c_uint).wrapping_add(widthScatter).wrapping_sub((rand()
                                                                                    as
                                                                                    libc::c_uint).wrapping_rem((2
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint).wrapping_mul(widthScatter)))
                        as int32;
                dv.z =
                    ((*psBuilding).y as
                         libc::c_uint).wrapping_add(breadthScatter).wrapping_sub((rand()
                                                                                      as
                                                                                      libc::c_uint).wrapping_rem((2
                                                                                                                      as
                                                                                                                      libc::c_int
                                                                                                                      as
                                                                                                                      libc::c_uint).wrapping_mul(breadthScatter)))
                        as int32;
                dv.y = (*psBuilding).z as int32;
                dv.y +=
                    (*(*psBuilding).sDisplay.imd).ymax * 3 as libc::c_int /
                        4 as libc::c_int;
                addEffect(&mut dv, EFFECT_SMOKE, SMOKE_TYPE_DRIFTING_HIGH,
                          0 as libc::c_int, 0 as *mut iIMDShape,
                          0 as libc::c_int);
                (*psBuilding).lastEmission = gameTime
                //psBuilding->emissionInterval = CALC_STRUCTURE_SMOKE_INTERVAL(percentDamage);
            }
        }
    }
    if (*psBuilding).id.wrapping_rem(10 as libc::c_int as libc::c_uint) ==
           frameGetFrameNumber().wrapping_rem(10 as libc::c_int as
                                                  libc::c_uint) {
        processVisibility(psBuilding as *mut BASE_OBJECT);
    }
    /* Update the fire damage data */
    if (*psBuilding).inFire & 0x1 as libc::c_int != 0 {
        /* Still in a fire, reset the fire flag to see if we get out this turn */
        (*psBuilding).inFire = 0 as libc::c_int
    } else {
        /* The fire flag has not been set so we must be out of the fire */
        (*psBuilding).burnStart = 0 as libc::c_int as UDWORD;
        (*psBuilding).burnDamage = 0 as libc::c_int as UDWORD
    }
    //check the resistance level of the structure
	//if (psBuilding->resistance < (SDWORD)psBuilding->pStructureType->resistance)
    iPointsRequired =
        structureResistance((*psBuilding).pStructureType,
                            (*psBuilding).player);
    if ((*psBuilding).resistance as libc::c_int) <
           iPointsRequired as SWORD as libc::c_int {
        //start the resistance increase
        if (*psBuilding).lastResistance == 0 as libc::c_int as libc::c_uint {
            (*psBuilding).lastResistance = gameTime
        }
        //increase over time if low
        if gameTime.wrapping_sub((*psBuilding).lastResistance) >
               2000 as libc::c_int as libc::c_uint {
            (*psBuilding).resistance += 1;
            //in multiplayer, certain structures do not function whilst low resistance
            if bMultiPlayer != 0 { resetResistanceLag(psBuilding); }
            (*psBuilding).lastResistance = gameTime;
            //once the resistance is back up reset the last time increased
            if (*psBuilding).resistance as libc::c_int >=
                   iPointsRequired as SWORD as libc::c_int {
                (*psBuilding).lastResistance = 0 as libc::c_int as UDWORD
            }
        }
    } else {
        //if selfrepair has been researched then check the health level of the
        //structure once resistance is fully up
        iPointsRequired = structureBody(psBuilding);
        if selfRepairEnabled((*psBuilding).player) != 0 &&
               ((*psBuilding).body as libc::c_int) <
                   iPointsRequired as SWORD as libc::c_int {
            //start the self repair off
            if (*psBuilding).lastResistance ==
                   0 as libc::c_int as libc::c_uint {
                (*psBuilding).lastResistance = gameTime
            }
            /*since self repair, then add half repair points depending on the time delay
            for the stat*/
            iPointsToAdd =
                repairPoints(asRepairStats.offset(aDefaultRepair[(*psBuilding).player
                                                                     as usize]
                                                      as isize),
                             (*psBuilding).player).wrapping_div(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint).wrapping_mul(gameTime.wrapping_sub((*psBuilding).lastResistance).wrapping_div((*asRepairStats.offset(aDefaultRepair[(*psBuilding).player
                                                                                                                                                                                                          as
                                                                                                                                                                                                          usize]
                                                                                                                                                                                           as
                                                                                                                                                                                           isize)).time));
            //add the blue flashing effect for multiPlayer
            if bMultiPlayer != 0 &&
                   rand() % 10 as libc::c_int == 0 as libc::c_int {
                let mut position: iVector = iVector{x: 0, y: 0, z: 0,};
                let mut point: *mut iVector = 0 as *mut iVector;
                let mut realY: SDWORD = 0;
                let mut pointIndex: UDWORD = 0;
                pointIndex =
                    (rand() %
                         ((*(*psBuilding).sDisplay.imd).npoints -
                              1 as libc::c_int)) as UDWORD;
                point =
                    &mut *(*(*psBuilding).sDisplay.imd).points.offset(pointIndex
                                                                          as
                                                                          isize)
                        as *mut iVector;
                position.x = (*psBuilding).x as libc::c_int + (*point).x;
                realY =
                    (structHeightScale(psBuilding) *
                         (*point).y as libc::c_float) as SDWORD;
                position.y = (*psBuilding).z as libc::c_int + realY;
                position.z = (*psBuilding).y as libc::c_int - (*point).z;
                effectSetSize(30 as libc::c_int as UDWORD);
                addEffect(&mut position, EFFECT_EXPLOSION,
                          EXPLOSION_TYPE_SPECIFIED, 1 as libc::c_int,
                          getImdFromIndex(MI_PLASMA as libc::c_int as UDWORD),
                          0 as libc::c_int);
            }
            if iPointsToAdd != 0 {
                (*psBuilding).body =
                    ((*psBuilding).body as
                         libc::c_uint).wrapping_add(iPointsToAdd) as UWORD;
                (*psBuilding).lastResistance = gameTime;
                if (*psBuilding).body as libc::c_uint > iPointsRequired {
                    (*psBuilding).body = iPointsRequired as UWORD;
                    (*psBuilding).lastResistance = 0 as libc::c_int as UDWORD
                }
            }
        }
    };
}
/* Release all resources associated with a structure */
#[no_mangle]
pub unsafe extern "C" fn structureRelease(mut psBuilding: *mut STRUCTURE) {
    let mut psAssemblyPoint: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if !(*psBuilding).pFunctionality.is_null() {
        psAssemblyPoint = 0 as *mut FLAG_POSITION;
        if StructIsFactory(psBuilding) != 0 {
            // free up factory stuff
            if !(*((*psBuilding).pFunctionality as
                       *mut FACTORY)).psFormation.is_null() {
                formationReset((*((*psBuilding).pFunctionality as
                                      *mut FACTORY)).psFormation);
            }
            //not used anymore - AB 11/02/99
			//remove from the group it is in
			/*if (((FACTORY *)psBuilding->pFunctionality)->psGroup != NULL)
			{
				grpLeave(((FACTORY *)psBuilding->pFunctionality)->psGroup, NULL);
			}*/
            psAssemblyPoint =
                (*((*psBuilding).pFunctionality as
                       *mut FACTORY)).psAssemblyPoint;
            // remove any commander from the factory
            if !(*((*psBuilding).pFunctionality as
                       *mut FACTORY)).psCommander.is_null() {
                assignFactoryCommandDroid(psBuilding, 0 as *mut _droid);
            }
        } else if (*(*psBuilding).pStructureType).type_0 ==
                      REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            // free up repair fac stuff
            psAssemblyPoint =
                (*((*psBuilding).pFunctionality as
                       *mut REPAIR_FACILITY)).psDeliveryPoint
        }
        // remove any assembly points
        if !psAssemblyPoint.is_null() { removeFlagPosition(psAssemblyPoint); }
        //free up the space used by the functionality array
        removeStructFunc((*psBuilding).pFunctionality);
    }
    // remove the object from the grid
    gridRemoveObject(psBuilding as *mut BASE_OBJECT);
}
/*
fills the list with Structure that can be built. There is a limit on how many can
be built at any one time. Pass back the number available.
There is now a limit of how many of each type of structure are allowed per mission
*/
// PhilStructureList()  -- ha ha ha
#[no_mangle]
pub unsafe extern "C" fn fillStructureList(mut ppList:
                                               *mut *mut STRUCTURE_STATS,
                                           mut selectedPlayer_0: UDWORD,
                                           mut limit: UDWORD) -> UDWORD {
    let mut inc: UDWORD = 0;
    let mut count: UDWORD = 0;
    let mut researchModule: BOOL = 0;
    let mut factoryModule: BOOL = 0;
    let mut powerModule: BOOL = 0;
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psBuilding: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //check to see if able to build research/factory modules
    powerModule = 0 as libc::c_int;
    factoryModule = powerModule;
    researchModule = factoryModule;
    //if currently on a mission can't build factory/research/power/derricks
	//if (mission.type != LDS_MKEEP AND mission.type != LDS_MCLEAR)
    if missionIsOffworld() == 0 {
        psCurr = apsStructLists[selectedPlayer_0 as usize];
        while !psCurr.is_null() {
            if (*(*psCurr).pStructureType).type_0 ==
                   REF_RESEARCH as libc::c_int as libc::c_uint &&
                   (*psCurr).status as libc::c_int == SS_BUILT as libc::c_int
               {
                researchModule = 1 as libc::c_int
            } else if (*(*psCurr).pStructureType).type_0 ==
                          REF_FACTORY as libc::c_int as libc::c_uint &&
                          (*psCurr).status as libc::c_int ==
                              SS_BUILT as libc::c_int {
                factoryModule = 1 as libc::c_int
            } else if (*(*psCurr).pStructureType).type_0 ==
                          REF_POWER_GEN as libc::c_int as libc::c_uint &&
                          (*psCurr).status as libc::c_int ==
                              SS_BUILT as libc::c_int {
                powerModule = 1 as libc::c_int
            }
            psCurr = (*psCurr).psNext
        }
    }
    count = 0 as libc::c_int as UDWORD;
    let mut current_block_22: u64;
    //set the list of Structures to build
    inc = 0 as libc::c_int as UDWORD;
    while inc < numStructureStats {
        //if the structure is flagged as available, add it to the list
        if *apStructTypeLists[selectedPlayer_0 as usize].offset(inc as isize)
               as libc::c_int & 0x1 as libc::c_int != 0 {
            /*DBPRINTF(("Structure %d is avail - current = %d limit = %d\n",inc,
			asStructLimits[selectedPlayer][inc].currentQuantity ,
					asStructLimits[selectedPlayer][inc].limit));*/
            //check not built the maximum allowed already
            if ((*asStructLimits[selectedPlayer_0 as
                                     usize].offset(inc as
                                                       isize)).currentQuantity
                    as libc::c_int) <
                   (*asStructLimits[selectedPlayer_0 as
                                        usize].offset(inc as isize)).limit as
                       libc::c_int {
                psBuilding = asStructureStats.offset(inc as isize);
                //don't want corner wall to appear in list
                if !((*psBuilding).type_0 ==
                         REF_WALLCORNER as libc::c_int as libc::c_uint) {
                    // Remove the demolish stat from the list for tutorial
// tjc 4-dec-98  ...
                    if bInTutorial != 0 {
                        if (*psBuilding).type_0 ==
                               REF_DEMOLISH as libc::c_int as libc::c_uint {
                            current_block_22 = 5948590327928692120;
                        } else { current_block_22 = 4068382217303356765; }
                    } else { current_block_22 = 4068382217303356765; }
                    match current_block_22 {
                        5948590327928692120 => { }
                        _ =>
                        //can't build list when offworld
				//if (mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR)
                        {
                            if missionIsOffworld() != 0 {
                                if (*psBuilding).type_0 ==
                                       REF_FACTORY as libc::c_int as
                                           libc::c_uint ||
                                       (*psBuilding).type_0 ==
                                           REF_POWER_GEN as libc::c_int as
                                               libc::c_uint ||
                                       (*psBuilding).type_0 ==
                                           REF_RESOURCE_EXTRACTOR as
                                               libc::c_int as libc::c_uint ||
                                       (*psBuilding).type_0 ==
                                           REF_RESEARCH as libc::c_int as
                                               libc::c_uint ||
                                       (*psBuilding).type_0 ==
                                           REF_CYBORG_FACTORY as libc::c_int
                                               as libc::c_uint ||
                                       (*psBuilding).type_0 ==
                                           REF_VTOL_FACTORY as libc::c_int as
                                               libc::c_uint {
                                    current_block_22 = 5948590327928692120;
                                } else {
                                    current_block_22 = 5689316957504528238;
                                }
                            } else { current_block_22 = 5689316957504528238; }
                            match current_block_22 {
                                5948590327928692120 => { }
                                _ => {
                                    if (*psBuilding).type_0 ==
                                           REF_RESEARCH_MODULE as libc::c_int
                                               as libc::c_uint {
                                        //don't add to list if Research Facility not presently built
                                        if researchModule == 0 {
                                            current_block_22 =
                                                5948590327928692120;
                                        } else {
                                            current_block_22 =
                                                15090052786889560393;
                                        }
                                    } else if (*psBuilding).type_0 ==
                                                  REF_FACTORY_MODULE as
                                                      libc::c_int as
                                                      libc::c_uint {
                                        //don't add to list if Factory not presently built
                                        if factoryModule == 0 {
                                            current_block_22 =
                                                5948590327928692120;
                                        } else {
                                            current_block_22 =
                                                15090052786889560393;
                                        }
                                    } else if (*psBuilding).type_0 ==
                                                  REF_POWER_MODULE as
                                                      libc::c_int as
                                                      libc::c_uint {
                                        //don't add to list if Power Gen not presently built
                                        if powerModule == 0 {
                                            current_block_22 =
                                                5948590327928692120;
                                        } else {
                                            current_block_22 =
                                                15090052786889560393;
                                        }
                                    } else {
                                        current_block_22 =
                                            15090052786889560393;
                                    }
                                    match current_block_22 {
                                        5948590327928692120 => { }
                                        _ =>
                                        //paranoid check!!
                                        {
                                            if (*psBuilding).type_0 ==
                                                   REF_FACTORY as libc::c_int
                                                       as libc::c_uint ||
                                                   (*psBuilding).type_0 ==
                                                       REF_CYBORG_FACTORY as
                                                           libc::c_int as
                                                           libc::c_uint ||
                                                   (*psBuilding).type_0 ==
                                                       REF_VTOL_FACTORY as
                                                           libc::c_int as
                                                           libc::c_uint {
                                                //NEVER EVER EVER WANT MORE THAN 5 FACTORIES
                                                if (*asStructLimits[selectedPlayer_0
                                                                        as
                                                                        usize].offset(inc
                                                                                          as
                                                                                          isize)).currentQuantity
                                                       as libc::c_int >=
                                                       5 as libc::c_int {
                                                    current_block_22 =
                                                        5948590327928692120;
                                                } else {
                                                    current_block_22 =
                                                        12997042908615822766;
                                                }
                                            } else {
                                                current_block_22 =
                                                    12997042908615822766;
                                            }
                                            match current_block_22 {
                                                5948590327928692120 => { }
                                                _ =>
                                                //HARD_CODE don't ever want more than one Las Sat structure
                                                {
                                                    if !(!(*psBuilding).psWeapStat.is_null()
                                                             &&
                                                             (*(*psBuilding).psWeapStat).weaponSubClass
                                                                 as
                                                                 libc::c_uint
                                                                 ==
                                                                 WSC_LAS_SAT
                                                                     as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint
                                                             &&
                                                             getLasSatExists(selectedPlayer_0)
                                                                 != 0) {
                                                        //HARD_CODE don't ever want more than one Sat Uplink structure
                                                        if (*psBuilding).type_0
                                                               ==
                                                               REF_SAT_UPLINK
                                                                   as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                           {
                                                            if (*asStructLimits[selectedPlayer_0
                                                                                    as
                                                                                    usize].offset(inc
                                                                                                      as
                                                                                                      isize)).currentQuantity
                                                                   as
                                                                   libc::c_int
                                                                   >=
                                                                   1 as
                                                                       libc::c_int
                                                               {
                                                                current_block_22
                                                                    =
                                                                    5948590327928692120;
                                                            } else {
                                                                current_block_22
                                                                    =
                                                                    14220266465818359136;
                                                            }
                                                        } else {
                                                            current_block_22 =
                                                                14220266465818359136;
                                                        }
                                                        match current_block_22
                                                            {
                                                            5948590327928692120
                                                            => {
                                                            }
                                                            _ => {
                                                                let fresh12 =
                                                                    count;
                                                                count =
                                                                    count.wrapping_add(1);
                                                                let ref mut fresh13 =
                                                                    *ppList.offset(fresh12
                                                                                       as
                                                                                       isize);
                                                                *fresh13 =
                                                                    psBuilding;
                                                                //count++;
				//check haven't reached limit
                                                                if count ==
                                                                       limit {
                                                                    return count
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        inc = inc.wrapping_add(1)
    }
    return count;
}
/* checks that the location is a valid one to build on and sets the outline colour
x and y in tile-coords*/
#[no_mangle]
pub unsafe extern "C" fn validLocation(mut psStats: *mut BASE_STATS,
                                       mut x: UDWORD, mut y: UDWORD,
                                       mut player: UDWORD,
                                       mut bCheckBuildQueue: BOOL) -> BOOL {
    let mut current_block: u64;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut psBuilding: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut valid: BOOL = 1 as libc::c_int;
    //UDWORD				i, j, min, max;
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut min: UDWORD = 0;
    let mut max: UDWORD = 0;
    let mut site: HIGHLIGHT = HIGHLIGHT{xTL: 0, yTL: 0, xBR: 0, yBR: 0,};
    let mut psCurrFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    psBuilding = psStats as *mut STRUCTURE_STATS;
    // initialise the buildsite structure
	// gets rid of the nasty bug when the GLOBAL buildSite was
	// used in here
    // Now now...we can quite easily hack this a bit more...
    if (*psStats).ref_0 >= 0xd0000 as libc::c_int as libc::c_uint &&
           (*psStats).ref_0 <
               (0xd0000 as libc::c_int + 0x10000 as libc::c_int) as
                   libc::c_uint {
        site.xTL = x as UWORD;
        site.yTL = y as UWORD;
        site.xBR =
            x.wrapping_add((*psBuilding).baseWidth).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                as UWORD;
        site.yBR =
            y.wrapping_add((*psBuilding).baseBreadth).wrapping_sub(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                as UWORD;
        // increase the size of a repair facility
        if (*psBuilding).type_0 ==
               REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            site.xTL = (site.xTL as libc::c_int - 1 as libc::c_int) as UWORD;
            site.yTL = (site.yTL as libc::c_int - 1 as libc::c_int) as UWORD;
            site.xBR = (site.xBR as libc::c_int + 1 as libc::c_int) as UWORD;
            site.yBR = (site.yBR as libc::c_int + 1 as libc::c_int) as UWORD
        }
        //if we're dragging the wall/defense we need to check along the current dragged size
        if wallDrag.status != 0 as libc::c_int as libc::c_uint {
            if (*psBuilding).type_0 == REF_WALL as libc::c_int as libc::c_uint
                   ||
                   (*psBuilding).type_0 ==
                       REF_DEFENSE as libc::c_int as libc::c_uint {
                let mut dx: UWORD = 0;
                let mut dy: UWORD = 0;
                wallDrag.x2 = mouseTileX as UDWORD;
                wallDrag.y2 = mouseTileY as UDWORD;
                dx =
                    abs((mouseTileX as libc::c_uint).wrapping_sub(wallDrag.x1)
                            as libc::c_int) as UWORD;
                dy =
                    abs((mouseTileY as libc::c_uint).wrapping_sub(wallDrag.y1)
                            as libc::c_int) as UWORD;
                if dx as libc::c_int >= dy as libc::c_int {
                    //build in x direction
                    site.xTL = wallDrag.x1 as UWORD;
                    site.xBR = wallDrag.x2 as UWORD;
                    site.yTL = wallDrag.y1 as UWORD;
                    if dy as libc::c_int > 0 as libc::c_int {
                        site.yBR =
                            (site.yTL as libc::c_int + 1 as libc::c_int) as
                                UWORD
                    } else { site.yBR = site.yTL }
                    if site.xTL as libc::c_int > site.xBR as libc::c_int {
                        dx = site.xBR;
                        site.xBR = site.xTL;
                        site.xTL = dx
                    }
                } else if (dx as libc::c_int) < dy as libc::c_int {
                    //build in y direction
                    site.yTL = wallDrag.y1 as UWORD;
                    site.yBR = wallDrag.y2 as UWORD;
                    site.xTL = wallDrag.x1 as UWORD;
                    if dx as libc::c_int > 0 as libc::c_int {
                        site.xBR =
                            (site.xTL as libc::c_int + 1 as libc::c_int) as
                                UWORD
                    } else { site.xBR = site.xTL }
                    if site.yTL as libc::c_int > site.yBR as libc::c_int {
                        dy = site.yBR;
                        site.yBR = site.yTL;
                        site.yTL = dy
                    }
                }
            }
        }
    } else {
        // If the stat's not a structure then assume it's size is 1x1.
        site.xTL = x as UWORD;
        site.yTL = y as UWORD;
        site.xBR = x as UWORD;
        site.yBR = y as UWORD
    }
    // This is now handled below by checking against scroll limits. PD,17/01/99.
//	/* Hackety hack hack hack!! */
//	/* Can't build too near the edge - problem solved - at least for now */
//	if(x<=2 OR y<=2 OR x>=mapWidth-5 OR y>=mapHeight-5)
//	{
//#ifdef PSX
//		SetHilightColourNotOK();
//#else
//		outlineColour = outlineNotOK;
//		outlineColour3D = outlineNotOK3D;
//#endif
//		return(FALSE);
//	}
    i = site.xTL as SDWORD;
    's_280:
        loop  {
            if !(i <= site.xBR as libc::c_int && valid != 0) {
                current_block = 17441561948628420366;
                break ;
            }
            j = site.yTL as SDWORD;
            while j <= site.yBR as libc::c_int && valid != 0 {
                //do this after the off-map check
/*#ifdef DEBUG
			if( gwZoneReachable(gwGetZone(i,j)) == FALSE)
			{
				// Can't ever drive there
				valid = FALSE;
				goto failed;
			}
#endif*/
                // Can't build outside of scroll limits.
                if i < scrollMinX + 2 as libc::c_int ||
                       i > scrollMaxX - 5 as libc::c_int ||
                       j < scrollMinY + 2 as libc::c_int ||
                       j > scrollMaxY - 5 as libc::c_int {
                    valid = 0 as libc::c_int;
                    current_block = 18417685841872033065;
                    break 's_280 ;
                } else if i <= 2 as libc::c_int || j <= 2 as libc::c_int ||
                              i >= mapWidth as SDWORD - 2 as libc::c_int ||
                              j >= mapHeight as SDWORD - 2 as libc::c_int {
                    valid = 0 as libc::c_int;
                    current_block = 18417685841872033065;
                    break 's_280 ;
                } else {
                    // check i or j off map.
                    /*God Awful HACK!! - AB 30/04/99 - gets round a problem with
			UrbanDuel map where an oil derrick cannot be built - when the
			map has been edited this hack can be removed*/
                    if (*psBuilding).type_0 !=
                           REF_RESOURCE_EXTRACTOR as libc::c_int as
                               libc::c_uint {
                        if gwZoneReachable(gwGetZone(i, j)) ==
                               0 as libc::c_int {
                            // Can't ever drive there
                            valid = 0 as libc::c_int;
                            current_block = 18417685841872033065;
                            break 's_280 ;
                        }
                    }
                    //don't check tile is visible for placement of a delivery point
                    if (*psStats).ref_0 >=
                           0xd0000 as libc::c_int as libc::c_uint &&
                           (*psStats).ref_0 <
                               (0xd0000 as libc::c_int +
                                    0x10000 as libc::c_int) as libc::c_uint {
                        //allow us to do so in debug mode!
                        if getDebugMappingStatus() == 0 && bMultiPlayer == 0 {
                            // Can't build where we haven't been yet.
                            if (*mapTile(i as UDWORD,
                                         j as UDWORD)).tileVisBits as
                                   libc::c_int & (1 as libc::c_int) << player
                                   == 0 {
                                valid = 0 as libc::c_int;
                                current_block = 18417685841872033065;
                                break 's_280 ;
                            }
                        }
                    }
                    j += 1
                }
            }
            i += 1
        }
    match current_block {
        17441561948628420366 =>
        // cant place on top of a delivery point...
        {
            psCurrFlag = apsFlagPosLists[selectedPlayer as usize];
            loop  {
                if psCurrFlag.is_null() {
                    current_block = 17711149709958600598;
                    break ;
                }
                // Need to check for whole building site, not just one tiles worth.	PD,17/01/99
		// How about a simple point in rect test - not a wacky loop test - John.
		// Hmm, you seem to have introduced a dirty smelly GOTO along the way too - Alex.
		// Anyone else care to comment?
		// It may be dirty and smelly, but it stops you going through about a hundred other
		// if's before the function quits, and if you care to check your K&R this is a perfectly valid
		// use of a goto - go on someone else pitch in !! - John.
		// K&R notwithstanding, aint no place for gotos in my code, hack-boy - Alex.
/*		for (i = site.xTL; i <= site.xBR AND valid; i++) {
			for (j = site.yTL; j <= site.yBR AND valid; j++) {
				if( ((psCurrFlag->coords.x)>>TILE_SHIFT == (SDWORD)i) &&
					((psCurrFlag->coords.y)>>TILE_SHIFT == (SDWORD)j) ) */
                i = (*psCurrFlag).coords.x >> 7 as libc::c_int;
                j = (*psCurrFlag).coords.y >> 7 as libc::c_int;
                if i >= site.xTL as libc::c_int &&
                       i <= site.xBR as libc::c_int &&
                       j >= site.yTL as libc::c_int &&
                       j <= site.yBR as libc::c_int {
                    valid = 0 as libc::c_int;
                    current_block = 18417685841872033065;
                    break ;
                } else { psCurrFlag = (*psCurrFlag).psNext }
            }
            match current_block {
                18417685841872033065 => { }
                _ =>
                // can't build next to a repair facility
                {
                    psStruct = apsStructLists[player as usize];
                    loop  {
                        if psStruct.is_null() {
                            current_block = 7301440000599063274;
                            break ;
                        }
                        if (*(*psStruct).pStructureType).type_0 ==
                               REF_REPAIR_FACILITY as libc::c_int as
                                   libc::c_uint {
                            // get the top left of the struct
                            i =
                                ((*psStruct).x as libc::c_int >>
                                     7 as libc::c_int) - 1 as libc::c_int;
                            j =
                                ((*psStruct).y as libc::c_int >>
                                     7 as libc::c_int) - 1 as libc::c_int;
                            // see if the x extents overlap
                            if site.xTL as libc::c_int >= i &&
                                   site.xTL as libc::c_int <=
                                       i + 2 as libc::c_int ||
                                   site.xBR as libc::c_int >= i &&
                                       site.xBR as libc::c_int <=
                                           i + 2 as libc::c_int {
                                // now see if y extents overlap
                                if site.yTL as libc::c_int >= j &&
                                       site.yTL as libc::c_int <=
                                           j + 2 as libc::c_int ||
                                       site.yBR as libc::c_int >= j &&
                                           site.yBR as libc::c_int <=
                                               j + 2 as libc::c_int {
                                    valid = 0 as libc::c_int;
                                    current_block = 18417685841872033065;
                                    break ;
                                }
                            }
                        }
                        psStruct = (*psStruct).psNext
                    }
                    match current_block {
                        18417685841872033065 => { }
                        _ => {
                            if (*psStats).ref_0 >=
                                   0xd0000 as libc::c_int as libc::c_uint &&
                                   (*psStats).ref_0 <
                                       (0xd0000 as libc::c_int +
                                            0x10000 as libc::c_int) as
                                           libc::c_uint {
                                //		Now done at top of function.
//		psBuilding = (STRUCTURE_STATS *)psStats;
//
//		// initialise the buildsite structure
//		// gets rid of the nasty bug when the GLOBAL buildSite was
//		// used in here
//		site.xTL = (UWORD)x;
//		site.yTL = (UWORD)y;
//		site.xBR = (UWORD)(x + psBuilding->baseWidth - 1);
//		site.yBR = (UWORD)(y + psBuilding->baseBreadth - 1);
                                match (*psBuilding).type_0 {
                                    0 | 1 | 18 | 10 | 3 | 7 | 8 | 6 | 12 | 13
                                    | 16 | 17 | 9 | 19 | 20 => {
                                        /*need to check each tile the structure will sit on is not water*/
                                        i = site.xTL as SDWORD;
                                        while i <= site.xBR as libc::c_int &&
                                                  valid != 0 {
                                            j = site.yTL as SDWORD;
                                            while j <= site.yBR as libc::c_int
                                                      && valid != 0 {
                                                psTile =
                                                    mapTile(i as UDWORD,
                                                            j as UDWORD);
                                                if terrainTypes[((*psTile).texture
                                                                     as
                                                                     libc::c_int
                                                                     &
                                                                     0x1ff as
                                                                         libc::c_int)
                                                                    as usize]
                                                       as libc::c_int ==
                                                       TER_WATER as
                                                           libc::c_int ||
                                                       terrainTypes[((*psTile).texture
                                                                         as
                                                                         libc::c_int
                                                                         &
                                                                         0x1ff
                                                                             as
                                                                             libc::c_int)
                                                                        as
                                                                        usize]
                                                           as libc::c_int ==
                                                           TER_CLIFFFACE as
                                                               libc::c_int {
                                                    valid = 0 as libc::c_int
                                                }
                                                j += 1
                                            }
                                            i += 1
                                        }
                                        //don't bother checking if already found a problem
                                        if valid != 0 {
                                            //check not within landing zone
                                            i = site.xTL as SDWORD;
                                            while i <= site.xBR as libc::c_int
                                                      && valid != 0 {
                                                j = site.yTL as SDWORD;
                                                while j <=
                                                          site.yBR as
                                                              libc::c_int &&
                                                          valid != 0 {
                                                    if withinLandingZone(i as
                                                                             UDWORD,
                                                                         j as
                                                                             UDWORD)
                                                           != 0 {
                                                        valid =
                                                            0 as libc::c_int
                                                    }
                                                    j += 1
                                                }
                                                i += 1
                                            }
                                        }
                                        // special droid/max-min test for repair facility
                                        if valid != 0 &&
                                               (*psBuilding).type_0 ==
                                                   REF_REPAIR_FACILITY as
                                                       libc::c_int as
                                                       libc::c_uint {
                                            getTileMaxMin(x, y, &mut max,
                                                          &mut min);
                                            if max.wrapping_sub(min) >
                                                   50 as libc::c_int as
                                                       libc::c_uint {
                                                valid = 0 as libc::c_int
                                            }
                                            if valid != 0 &&
                                                   noDroid(x, y) == 0 {
                                                valid = 0 as libc::c_int
                                            }
                                        }
                                        if valid != 0 &&
                                               (*psBuilding).type_0 !=
                                                   REF_REPAIR_FACILITY as
                                                       libc::c_int as
                                                       libc::c_uint {
                                            i = site.xTL as SDWORD;
                                            while i <= site.xBR as libc::c_int
                                                      && valid != 0 {
                                                j = site.yTL as SDWORD;
                                                while j <=
                                                          site.yBR as
                                                              libc::c_int &&
                                                          valid != 0 {
                                                    // This really needs to check to see if the droid that's in the way is the droid that wants to build
// in which case it should'nt invalidate the location.
                                                    if noDroid(i as UDWORD,
                                                               j as UDWORD) ==
                                                           0 as libc::c_int {
                                                        valid =
                                                            0 as libc::c_int
                                                    }
                                                    j += 1
                                                    //		if(TERRAIN_TYPE(mapTile(i,j)) == TER_WATER)
					//		{
					//			valid = FALSE;
					//		}
                                                }
                                                i += 1
                                            }
                                        }
                                        //walls/defensive structures can be built along any ground
                                        if valid != 0 &&
                                               !((*psBuilding).type_0 ==
                                                     REF_REPAIR_FACILITY as
                                                         libc::c_int as
                                                         libc::c_uint ||
                                                     (*psBuilding).type_0 ==
                                                         REF_DEFENSE as
                                                             libc::c_int as
                                                             libc::c_uint ||
                                                     (*psBuilding).type_0 ==
                                                         REF_WALL as
                                                             libc::c_int as
                                                             libc::c_uint) {
                                            /*cannot build on ground that is too steep*/
                                            min = 0 as libc::c_int as UDWORD;
                                            max = 0 as libc::c_int as UDWORD;
                                            i = site.xTL as SDWORD;
                                            while i <= site.xBR as libc::c_int
                                                      && valid != 0 {
                                                j = site.yTL as SDWORD;
                                                while j <=
                                                          site.yBR as
                                                              libc::c_int &&
                                                          valid != 0 {
                                                    getTileMaxMin(i as UDWORD,
                                                                  j as UDWORD,
                                                                  &mut max,
                                                                  &mut min);
                                                    if max.wrapping_sub(min) >
                                                           50 as libc::c_int
                                                               as libc::c_uint
                                                       {
                                                        valid =
                                                            0 as libc::c_int
                                                    }
                                                    j += 1
                                                }
                                                i += 1
                                            }
                                        }
                                        //don't bother checking if already found a problem
                                        if valid != 0 {
                                            //on PC - defence structures can be built next to anything now- AB 22/09/98
                    //and the Missile_Silo (special case) - AB 01/03/99
                                            if !((*psBuilding).type_0 ==
                                                     REF_DEFENSE as
                                                         libc::c_int as
                                                         libc::c_uint ||
                                                     (*psBuilding).type_0 ==
                                                         REF_WALL as
                                                             libc::c_int as
                                                             libc::c_uint ||
                                                     (*psBuilding).type_0 ==
                                                         REF_WALLCORNER as
                                                             libc::c_int as
                                                             libc::c_uint ||
                                                     (*psBuilding).type_0 ==
                                                         REF_MISSILE_SILO as
                                                             libc::c_int as
                                                             libc::c_uint) {
                                                /*need to check there is one tile between buildings*/
                                                i =
                                                    (site.xTL as libc::c_int -
                                                         1 as libc::c_int) as
                                                        UWORD as SDWORD;
                                                while i <=
                                                          (site.xBR as
                                                               libc::c_int +
                                                               1 as
                                                                   libc::c_int)
                                                              as UWORD as
                                                              libc::c_int {
                                                    j =
                                                        (site.yTL as
                                                             libc::c_int -
                                                             1 as libc::c_int)
                                                            as UWORD as
                                                            SDWORD;
                                                    while j <=
                                                              (site.yBR as
                                                                   libc::c_int
                                                                   +
                                                                   1 as
                                                                       libc::c_int)
                                                                  as UWORD as
                                                                  libc::c_int
                                                          {
                                                        //skip the actual area the structure will cover
                                                        if i <
                                                               site.xTL as
                                                                   libc::c_int
                                                               ||
                                                               i >
                                                                   site.xBR as
                                                                       libc::c_int
                                                               ||
                                                               j <
                                                                   site.yTL as
                                                                       libc::c_int
                                                               ||
                                                               j >
                                                                   site.yBR as
                                                                       libc::c_int
                                                           {
                                                            if (*mapTile(i as
                                                                             UDWORD,
                                                                         j as
                                                                             UDWORD)).tileInfoBits
                                                                   as
                                                                   libc::c_int
                                                                   &
                                                                   0x1 as
                                                                       libc::c_int
                                                                   != 0 {
                                                                // // On Playstation, to try and reduce the number of structures on screen.. defence
// // structures can only be build next to walls, otherwise there has to be a gap.
//#ifdef PSX
//										psStruct = getTileStructure(i,j);
//										if (psStruct)
//                                        {
//										    //defence structures can be built next to a wall, but nothing else.
//										    if (psBuilding->type == REF_DEFENSE)
//										    {
//												if (!(psStruct->pStructureType->type == REF_WALL OR
//													psStruct->pStructureType->type == REF_WALLCORNER))
//												{
//													valid = FALSE;
//												}
//    										}
//                                            //and walls can be built next to corner walls and defensive structures
//	    									else if (psBuilding->type == REF_WALL)
//                                            {
//												if (!(psStruct->pStructureType->type == REF_DEFENSE OR
//													psStruct->pStructureType->type == REF_WALLCORNER))
//												{
//													valid = FALSE;
//												}
//                                            }
//                                            else
//		    								{
//			    								valid = FALSE;
//				    						}
//                                        }
//#else
                                                                psStruct =
                                                                    getTileStructure(i
                                                                                         as
                                                                                         UDWORD,
                                                                                     j
                                                                                         as
                                                                                         UDWORD);
                                                                if !psStruct.is_null()
                                                                   {
                                                                    //#endif
                                                                    //you can build anything next to a defensive structure
                                                                    if (*(*psStruct).pStructureType).type_0
                                                                           !=
                                                                           REF_DEFENSE
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                           &&
                                                                           (*(*psStruct).pStructureType).type_0
                                                                               !=
                                                                               REF_WALL
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           &&
                                                                           (*(*psStruct).pStructureType).type_0
                                                                               !=
                                                                               REF_WALLCORNER
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                       {
                                                                        //Walls can be built next to walls and defenses - AB 03/03/99
                                                                        if (*psBuilding).type_0
                                                                               ==
                                                                               REF_WALL
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_uint
                                                                           {
                                                                            if !((*(*psStruct).pStructureType).type_0
                                                                                     ==
                                                                                     REF_WALL
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint
                                                                                     ||
                                                                                     (*(*psStruct).pStructureType).type_0
                                                                                         ==
                                                                                         REF_WALLCORNER
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_uint)
                                                                               {
                                                                                valid
                                                                                    =
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                            }
                                                                        } else {
                                                                            valid
                                                                                =
                                                                                0
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                    } else if bMultiPlayer
                                                                                  !=
                                                                                  0
                                                                                  &&
                                                                                  game.type_0
                                                                                      as
                                                                                      libc::c_int
                                                                                      ==
                                                                                      14
                                                                                          as
                                                                                          libc::c_int
                                                                                  &&
                                                                                  isHumanPlayer(player)
                                                                                      ==
                                                                                      0
                                                                     {
                                                                        valid
                                                                            =
                                                                            0
                                                                                as
                                                                                libc::c_int
                                                                    }
                                                                }
                                                            }
                                                            // is a defense.
                                                            // skirmish players don't build defensives next to each other.(route hack)
                                                            //cannot build within one tile of a oil resource
                                                            if (*mapTile(i as
                                                                             UDWORD,
                                                                         j as
                                                                             UDWORD)).tileInfoBits
                                                                   as
                                                                   libc::c_int
                                                                   &
                                                                   0x2 as
                                                                       libc::c_int
                                                                   != 0 {
                                                                psFeat =
                                                                    getTileFeature(i
                                                                                       as
                                                                                       UDWORD,
                                                                                   j
                                                                                       as
                                                                                       UDWORD);
                                                                if !psFeat.is_null()
                                                                       &&
                                                                       (*(*psFeat).psStats).subType
                                                                           as
                                                                           libc::c_uint
                                                                           ==
                                                                           FEAT_OIL_RESOURCE
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint
                                                                   {
                                                                    valid =
                                                                        0 as
                                                                            libc::c_int
                                                                }
                                                            }
                                                        }
                                                        j += 1
                                                    }
                                                    i += 1
                                                }
                                            }
                                        }
                                        //don't bother checking if already found a problem
                                        if valid != 0 {
                                            /*need to check each tile the structure will sit on*/
                                            i = site.xTL as SDWORD;
                                            while i <= site.xBR as libc::c_int
                                                      && valid != 0 {
                                                j = site.yTL as SDWORD;
                                                while j <=
                                                          site.yBR as
                                                              libc::c_int &&
                                                          valid != 0 {
                                                    psTile =
                                                        mapTile(i as UDWORD,
                                                                j as UDWORD);
                                                    if (*psTile).tileInfoBits
                                                           as libc::c_int &
                                                           (0x1 as libc::c_int
                                                                |
                                                                0x2 as
                                                                    libc::c_int
                                                                |
                                                                0x20 as
                                                                    libc::c_int)
                                                           != 0 {
                                                        if ((*psBuilding).type_0
                                                                ==
                                                                REF_DEFENSE as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint
                                                                ||
                                                                (*psBuilding).type_0
                                                                    ==
                                                                    REF_WALL
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                                               &&
                                                               (*psTile).tileInfoBits
                                                                   as
                                                                   libc::c_int
                                                                   &
                                                                   0x20 as
                                                                       libc::c_int
                                                                   != 0 {
                                                            psStruct =
                                                                getTileStructure(i
                                                                                     as
                                                                                     UDWORD,
                                                                                 j
                                                                                     as
                                                                                     UDWORD);
                                                            if !psStruct.is_null()
                                                                   &&
                                                                   (*psStruct).player
                                                                       as
                                                                       libc::c_uint
                                                                       !=
                                                                       player
                                                               {
                                                                valid =
                                                                    0 as
                                                                        libc::c_int
                                                            }
                                                        } else {
                                                            valid =
                                                                0 as
                                                                    libc::c_int
                                                        }
                                                    }
                                                    j += 1
                                                }
                                                i += 1
                                            }
                                        }
                                    }
                                    2 => {
                                        valid = 0 as libc::c_int;
                                        if (*mapTile(x, y)).tileInfoBits as
                                               libc::c_int &
                                               0x1 as libc::c_int != 0 {
                                            psStruct = getTileStructure(x, y);
                                            if !psStruct.is_null() &&
                                                   ((*(*psStruct).pStructureType).type_0
                                                        ==
                                                        REF_FACTORY as
                                                            libc::c_int as
                                                            libc::c_uint ||
                                                        (*(*psStruct).pStructureType).type_0
                                                            ==
                                                            REF_VTOL_FACTORY
                                                                as libc::c_int
                                                                as
                                                                libc::c_uint)
                                                   &&
                                                   (*psStruct).status as
                                                       libc::c_int ==
                                                       SS_BUILT as libc::c_int
                                               {
                                                valid = 1 as libc::c_int
                                            }
                                        }
                                    }
                                    11 => {
                                        valid = 0 as libc::c_int;
                                        //check that there is a research facility at the location
                                        if (*mapTile(x, y)).tileInfoBits as
                                               libc::c_int &
                                               0x1 as libc::c_int != 0 {
                                            psStruct = getTileStructure(x, y);
                                            if !psStruct.is_null() &&
                                                   (*(*psStruct).pStructureType).type_0
                                                       ==
                                                       REF_RESEARCH as
                                                           libc::c_int as
                                                           libc::c_uint &&
                                                   (*psStruct).status as
                                                       libc::c_int ==
                                                       SS_BUILT as libc::c_int
                                               {
                                                valid = 1 as libc::c_int
                                            }
                                        }
                                    }
                                    4 => {
                                        valid = 0 as libc::c_int;
                                        if (*mapTile(x, y)).tileInfoBits as
                                               libc::c_int &
                                               0x1 as libc::c_int != 0 {
                                            psStruct = getTileStructure(x, y);
                                            if !psStruct.is_null() &&
                                                   (*(*psStruct).pStructureType).type_0
                                                       ==
                                                       REF_POWER_GEN as
                                                           libc::c_int as
                                                           libc::c_uint &&
                                                   (*psStruct).status as
                                                       libc::c_int ==
                                                       SS_BUILT as libc::c_int
                                               {
                                                valid = 1 as libc::c_int
                                            }
                                        }
                                    }
                                    5 => {
                                        valid = 0 as libc::c_int;
                                        //check that there is a oil resource at the location
                                        if (*mapTile(x, y)).tileInfoBits as
                                               libc::c_int &
                                               0x2 as libc::c_int != 0 {
                                            psFeat = getTileFeature(x, y);
                                            if !psFeat.is_null() &&
                                                   (*(*psFeat).psStats).subType
                                                       as libc::c_uint ==
                                                       FEAT_OIL_RESOURCE as
                                                           libc::c_int as
                                                           libc::c_uint {
                                                valid = 1 as libc::c_int
                                            }
                                        }
                                    }
                                    _ => { valid = 1 as libc::c_int }
                                }
                                //if setting up a build queue need to check against future sites as well - AB 4/5/99
                                if ctrlShiftDown() != 0 &&
                                       player == selectedPlayer &&
                                       bCheckBuildQueue != 0 {
                                    let mut psDroid: *mut DROID =
                                        0 as *mut DROID;
                                    let mut order: SDWORD = 0;
                                    let mut left: SDWORD = 0;
                                    let mut right: SDWORD = 0;
                                    let mut up: SDWORD = 0;
                                    let mut down: SDWORD = 0;
                                    let mut size: SDWORD = 0;
                                    let mut validCombi: BOOL = 0;
                                    //defense and missile silo's can be built next to anything so don't need to check
                                    if !((*psBuilding).type_0 ==
                                             REF_DEFENSE as libc::c_int as
                                                 libc::c_uint ||
                                             (*psBuilding).type_0 ==
                                                 REF_MISSILE_SILO as
                                                     libc::c_int as
                                                     libc::c_uint) {
                                        psDroid =
                                            apsDroidLists[player as usize];
                                        while !psDroid.is_null() {
                                            //once its invalid stop checking
                                            if valid == 0 as libc::c_int {
                                                break ;
                                            }
                                            if (*psDroid).droidType as
                                                   libc::c_uint ==
                                                   DROID_CONSTRUCT as
                                                       libc::c_int as
                                                       libc::c_uint ||
                                                   (*psDroid).droidType as
                                                       libc::c_uint ==
                                                       DROID_CYBORG_CONSTRUCT
                                                           as libc::c_int as
                                                           libc::c_uint {
                                                //look thru' the list of orders to see if more building sites
                                                order = 0 as libc::c_int;
                                                while order <
                                                          (*psDroid).listSize
                                                      {
                                                    if (*psDroid).asOrderList[order
                                                                                  as
                                                                                  usize].order
                                                           ==
                                                           DORDER_BUILD as
                                                               libc::c_int {
                                                        validCombi =
                                                            0 as libc::c_int;
                                                        if (*((*psDroid).asOrderList[order
                                                                                         as
                                                                                         usize].psOrderTarget
                                                                  as
                                                                  *mut STRUCTURE_STATS)).type_0
                                                               ==
                                                               REF_DEFENSE as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint
                                                               ||
                                                               (*((*psDroid).asOrderList[order
                                                                                             as
                                                                                             usize].psOrderTarget
                                                                      as
                                                                      *mut STRUCTURE_STATS)).type_0
                                                                   ==
                                                                   REF_MISSILE_SILO
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                           {
                                                            validCombi =
                                                                1 as
                                                                    libc::c_int
                                                        }
                                                        //walls can be built next to walls and defence
                                                        if ((*psBuilding).type_0
                                                                ==
                                                                REF_WALL as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint
                                                                ||
                                                                (*psBuilding).type_0
                                                                    ==
                                                                    REF_WALLCORNER
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)
                                                               &&
                                                               ((*((*psDroid).asOrderList[order
                                                                                              as
                                                                                              usize].psOrderTarget
                                                                       as
                                                                       *mut STRUCTURE_STATS)).type_0
                                                                    ==
                                                                    REF_WALL
                                                                        as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint
                                                                    ||
                                                                    (*((*psDroid).asOrderList[order
                                                                                                  as
                                                                                                  usize].psOrderTarget
                                                                           as
                                                                           *mut STRUCTURE_STATS)).type_0
                                                                        ==
                                                                        REF_WALLCORNER
                                                                            as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                           {
                                                            validCombi =
                                                                1 as
                                                                    libc::c_int
                                                        }
                                                        //don't bother checking if valid combination of building types
                                                        if validCombi == 0 {
                                                            /*need to check there is one tile between buildings*/
                                    //check if any corner is within the build site
                                                            size =
                                                                (*((*psDroid).asOrderList[order
                                                                                              as
                                                                                              usize].psOrderTarget
                                                                       as
                                                                       *mut STRUCTURE_STATS)).baseWidth
                                                                    as SDWORD;
                                                            left =
                                                                ((*psDroid).asOrderList[order
                                                                                            as
                                                                                            usize].x
                                                                     as
                                                                     libc::c_int
                                                                     >>
                                                                     7 as
                                                                         libc::c_int)
                                                                    -
                                                                    size /
                                                                        2 as
                                                                            libc::c_int;
                                                            right =
                                                                left + size;
                                                            size =
                                                                (*((*psDroid).asOrderList[order
                                                                                              as
                                                                                              usize].psOrderTarget
                                                                       as
                                                                       *mut STRUCTURE_STATS)).baseBreadth
                                                                    as SDWORD;
                                                            up =
                                                                ((*psDroid).asOrderList[order
                                                                                            as
                                                                                            usize].y
                                                                     as
                                                                     libc::c_int
                                                                     >>
                                                                     7 as
                                                                         libc::c_int)
                                                                    -
                                                                    size /
                                                                        2 as
                                                                            libc::c_int;
                                                            down = up + size;
                                                            // increase the size of a repair facility
                                                            if (*((*psDroid).asOrderList[order
                                                                                             as
                                                                                             usize].psOrderTarget
                                                                      as
                                                                      *mut STRUCTURE_STATS)).type_0
                                                                   ==
                                                                   REF_REPAIR_FACILITY
                                                                       as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint
                                                               {
                                                                left -=
                                                                    1 as
                                                                        libc::c_int;
                                                                up -=
                                                                    1 as
                                                                        libc::c_int;
                                                                right +=
                                                                    1 as
                                                                        libc::c_int;
                                                                down +=
                                                                    1 as
                                                                        libc::c_int
                                                            }
                                                            if left >
                                                                   site.xTL as
                                                                       libc::c_int
                                                                       -
                                                                       1 as
                                                                           libc::c_int
                                                                   &&
                                                                   left <=
                                                                       site.xBR
                                                                           as
                                                                           libc::c_int
                                                                           +
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                   &&
                                                                   (up >
                                                                        site.yTL
                                                                            as
                                                                            libc::c_int
                                                                            -
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                        &&
                                                                        up <=
                                                                            site.yBR
                                                                                as
                                                                                libc::c_int
                                                                                +
                                                                                1
                                                                                    as
                                                                                    libc::c_int)
                                                                   ||
                                                                   right >
                                                                       site.xTL
                                                                           as
                                                                           libc::c_int
                                                                           -
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                       &&
                                                                       right
                                                                           <=
                                                                           site.xBR
                                                                               as
                                                                               libc::c_int
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                       &&
                                                                       (up >
                                                                            site.yTL
                                                                                as
                                                                                libc::c_int
                                                                                -
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            &&
                                                                            up
                                                                                <=
                                                                                site.yBR
                                                                                    as
                                                                                    libc::c_int
                                                                                    +
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                                   ||
                                                                   left >
                                                                       site.xTL
                                                                           as
                                                                           libc::c_int
                                                                           -
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                       &&
                                                                       left <=
                                                                           site.xBR
                                                                               as
                                                                               libc::c_int
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                       &&
                                                                       (down >
                                                                            site.yTL
                                                                                as
                                                                                libc::c_int
                                                                                -
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            &&
                                                                            down
                                                                                <=
                                                                                site.yBR
                                                                                    as
                                                                                    libc::c_int
                                                                                    +
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                                   ||
                                                                   right >
                                                                       site.xTL
                                                                           as
                                                                           libc::c_int
                                                                           -
                                                                           1
                                                                               as
                                                                               libc::c_int
                                                                       &&
                                                                       right
                                                                           <=
                                                                           site.xBR
                                                                               as
                                                                               libc::c_int
                                                                               +
                                                                               1
                                                                                   as
                                                                                   libc::c_int
                                                                       &&
                                                                       (down >
                                                                            site.yTL
                                                                                as
                                                                                libc::c_int
                                                                                -
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                            &&
                                                                            down
                                                                                <=
                                                                                site.yBR
                                                                                    as
                                                                                    libc::c_int
                                                                                    +
                                                                                    1
                                                                                        as
                                                                                        libc::c_int)
                                                               {
                                                                valid =
                                                                    0 as
                                                                        libc::c_int;
                                                                break ;
                                                            }
                                                        }
                                                    }
                                                    order += 1
                                                }
                                            }
                                            psDroid = (*psDroid).psNext
                                        }
                                    }
                                }
                            } else {
                                // not positioning a structure
                                valid = 1 as libc::c_int;
                                if fpathGroundBlockingTile(x as SDWORD,
                                                           y as SDWORD) != 0 {
                                    valid = 0 as libc::c_int
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if valid == 0 {
        // Only set the hilight colour if it's the selected player.
        if player == selectedPlayer {
            outlineColour = outlineNotOK;
            outlineColour3D = 14 as libc::c_int as UDWORD
        }
        return 0 as libc::c_int
    }
    // Only set the hilight colour if it's the selected player.
    if player == selectedPlayer {
        outlineColour = outlineOK;
        outlineColour3D = 255 as libc::c_int as UDWORD
    }
    return 1 as libc::c_int;
}
/*
for a new structure, find a location along an edge which the droid can get
to and return this as the destination for the droid.
*/
//BOOL getDroidDestination(STRUCTURE_STATS *psBuildingStats, UDWORD structX,
#[no_mangle]
pub unsafe extern "C" fn getDroidDestination(mut psStats: *mut BASE_STATS,
                                             mut structX: UDWORD,
                                             mut structY: UDWORD,
                                             mut pDroidX: *mut UDWORD,
                                             mut pDroidY: *mut UDWORD)
 -> BOOL {
    let mut start: UWORD = 0;
    let mut structTileX: UDWORD = 0;
    let mut structTileY: UDWORD = 0;
    let mut width: UDWORD = 0 as libc::c_int as UDWORD;
    let mut breadth: UDWORD = 0 as libc::c_int as UDWORD;
    /*	UDWORD	numIterations;
	UDWORD	desiredX,desiredY;
	BOOL	gotTarget;
	UDWORD	xMod,yMod;
	SDWORD	xVar,yVar;*/
    if StatIsStructure(psStats) != 0 {
        width = (*(psStats as *mut STRUCTURE_STATS)).baseWidth;
        breadth = (*(psStats as *mut STRUCTURE_STATS)).baseBreadth
    } else if StatIsFeature(psStats) != 0 {
        width = (*(psStats as *mut FEATURE_STATS)).baseWidth as UDWORD;
        breadth = (*(psStats as *mut FEATURE_STATS)).baseBreadth as UDWORD
    }
    // -----------------------------------------------------------
	/*
	flushConsoleMessages();

	numIterations = max(width,breadth);
	xMod = (width);
	yMod = (breadth);

	gotTarget = FALSE;
	while(!gotTarget)
	{
   	   	xVar = xMod - rand()%((xMod*2)+1);
		yVar = yMod - rand()%((yMod*2)+1);
		desiredX = (structX>>TILE_SHIFT) + xVar;
		desiredY = (structY>>TILE_SHIFT) + yVar;
		if(desiredX <=0) desiredX = 1;
		if(desiredY <=0) desiredY = 1;
		if(desiredX >= mapWidth-1) desiredX = mapWidth-1;
		if(desiredY >= mapHeight-1) desiredY = mapHeight-1;



   		DBCONPRINTF(ConsoleString,(ConsoleString,"Desired X : %d",desiredX));
		DBCONPRINTF(ConsoleString,(ConsoleString,"Desired Y : %d",desiredY));
		gotTarget = unfussyPickATile(&desiredX,&desiredY,numIterations);
		numIterations++;
	}

	*pDroidX = desiredX<<TILE_SHIFT;
	*pDroidY = desiredY<<TILE_SHIFT;
	DBCONPRINTF(ConsoleString,(ConsoleString,"Given X : %d",desiredX));
	DBCONPRINTF(ConsoleString,(ConsoleString,"Given Y : %d",desiredY));

	return(TRUE);
	*/
    // -----------------------------------------------------------
    //get a random starting place 0=top left
	//start = (UWORD)(rand() % ((psBuildingStats->baseWidth + psBuildingStats->
	//	baseBreadth) * 2));
    start =
        (rand() as
             libc::c_uint).wrapping_rem(width.wrapping_add(breadth).wrapping_mul(2
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint))
            as UWORD;
    //search in a clockwise direction around the structure from the starting point
	//if (start == 0 OR start < psBuildingStats->baseWidth)
    if start as libc::c_int == 0 as libc::c_int ||
           (start as libc::c_uint) < width {
        //top side first
        structTileX = structX >> 7 as libc::c_int;
        structTileY =
            (structY >>
                 7 as
                     libc::c_int).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint);
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        //structTileX += psBuildingStats->baseWidth;
        structTileX =
            (structTileX as libc::c_uint).wrapping_add(width) as UDWORD as
                UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX = structX >> 7 as libc::c_int;
        //structTileY += psBuildingStats->baseBreadth;
        structTileY =
            (structTileY as libc::c_uint).wrapping_add(breadth) as UDWORD as
                UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY = structY >> 7 as libc::c_int;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
    } else if start as libc::c_uint == width ||
                  (start as libc::c_uint) < width.wrapping_add(breadth) {
        //else if (start == psBuildingStats->baseWidth OR start < (psBuildingStats->
	//	baseWidth + psBuildingStats->baseBreadth))
        //right side first
		//structTileX = (structX >> TILE_SHIFT) + psBuildingStats->baseWidth;
        structTileX = (structX >> 7 as libc::c_int).wrapping_add(width);
        structTileY = structY >> 7 as libc::c_int;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX = structX >> 7 as libc::c_int;
        //structTileY += psBuildingStats->baseBreadth;
        structTileY =
            (structTileY as libc::c_uint).wrapping_add(breadth) as UDWORD as
                UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY = structY >> 7 as libc::c_int;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
    } else if start as libc::c_uint == width.wrapping_add(breadth) ||
                  (start as libc::c_uint) < width.wrapping_mul(breadth) {
        //else if (start == (psBuildingStats->baseWidth + psBuildingStats->
	//	baseBreadth) OR start < (psBuildingStats->baseWidth *
	//	psBuildingStats->baseBreadth))
        //bottom first
        structTileX = structX >> 7 as libc::c_int;
        //structTileY = (structY >> TILE_SHIFT) + psBuildingStats->baseBreadth;
        structTileY = (structY >> 7 as libc::c_int).wrapping_add(breadth);
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY = structY >> 7 as libc::c_int;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        //structTileX += psBuildingStats->baseWidth;
        structTileX =
            (structTileX as libc::c_uint).wrapping_add(width) as UDWORD as
                UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
    } else {
        //left side first
        structTileX =
            (structX >>
                 7 as
                     libc::c_int).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint);
        structTileY = structY >> 7 as libc::c_int;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX =
            (structTileX as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
        //structTileX += psBuildingStats->baseWidth;
        structTileX =
            (structTileX as libc::c_uint).wrapping_add(width) as UDWORD as
                UDWORD;
        structTileY =
            (structTileY as
                 libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                as UDWORD as UDWORD;
        //if (checkLength(psBuildingStats->baseBreadth, structTileX, structTileY,
        if checkLength(breadth, structTileX, structTileY, pDroidX, pDroidY) !=
               0 {
            return 1 as libc::c_int
        }
        structTileX = structX >> 7 as libc::c_int;
        //structTileY += psBuildingStats->baseBreadth;
        structTileY =
            (structTileY as libc::c_uint).wrapping_add(breadth) as UDWORD as
                UDWORD;
        //if (checkWidth(psBuildingStats->baseWidth, structTileX, structTileY,
        if checkWidth(width, structTileX, structTileY, pDroidX, pDroidY) != 0
           {
            return 1 as libc::c_int
        }
    }
    //not found a valid location so return FALSE
    return 0 as libc::c_int;
}
/* check along the width of a structure for an empty space */
#[no_mangle]
pub unsafe extern "C" fn checkWidth(mut maxRange: UDWORD, mut x: UDWORD,
                                    mut y: UDWORD, mut pDroidX: *mut UDWORD,
                                    mut pDroidY: *mut UDWORD) -> BOOL {
    let mut side: UDWORD = 0;
    side = 0 as libc::c_int as UDWORD;
    while side < maxRange {
        //psor		if (mapTile(x + side, y)->psObject == NULL)
        if x.wrapping_add(side) < mapWidth && y < mapHeight {
            if (*mapTile(x.wrapping_add(side), y)).tileInfoBits as libc::c_int
                   &
                   (0x1 as libc::c_int | 0x2 as libc::c_int |
                        0x20 as libc::c_int) == 0 {
                *pDroidX = x.wrapping_add(side) << 7 as libc::c_int;
                *pDroidY = y << 7 as libc::c_int;
                if worldOnMap(*pDroidX as SDWORD, *pDroidY as SDWORD) != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkWidth : Insane droid position\x00" as
                              *const u8 as *const libc::c_char);
                };
                if worldOnMap(*pDroidX as SDWORD, *pDroidY as SDWORD) != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 6227 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 11],
                                                    &[libc::c_char; 11]>(b"checkWidth\x00")).as_ptr(),
                          b"worldOnMap(*pDroidX,*pDroidY)\x00" as *const u8 as
                              *const libc::c_char);
                };
                return 1 as libc::c_int
            }
        }
        side = side.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
/* check along the length of a structure for an empty space */
#[no_mangle]
pub unsafe extern "C" fn checkLength(mut maxRange: UDWORD, mut x: UDWORD,
                                     mut y: UDWORD, mut pDroidX: *mut UDWORD,
                                     mut pDroidY: *mut UDWORD) -> BOOL {
    let mut side: UDWORD = 0;
    side = 0 as libc::c_int as UDWORD;
    while side < maxRange {
        //psor		if (mapTile(x, y + side)->psObject == NULL)
        if y.wrapping_add(side) < mapHeight && x < mapWidth {
            if (*mapTile(x, y.wrapping_add(side))).tileInfoBits as libc::c_int
                   &
                   (0x1 as libc::c_int | 0x2 as libc::c_int |
                        0x20 as libc::c_int) == 0 {
                *pDroidX = x << 7 as libc::c_int;
                *pDroidY = y.wrapping_add(side) << 7 as libc::c_int;
                if worldOnMap(*pDroidX as SDWORD, *pDroidY as SDWORD) != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkHeight : Insane droid position\x00" as
                              *const u8 as *const libc::c_char);
                };
                if worldOnMap(*pDroidX as SDWORD, *pDroidY as SDWORD) != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 6252 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 12],
                                                    &[libc::c_char; 12]>(b"checkLength\x00")).as_ptr(),
                          b"worldOnMap(*pDroidX,*pDroidY)\x00" as *const u8 as
                              *const libc::c_char);
                };
                return 1 as libc::c_int
            }
        }
        side = side.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
//remove a structure from the map
unsafe extern "C" fn removeStructFromMap(mut psStruct: *mut STRUCTURE) {
    let mut i: UDWORD = 0;
    let mut j: UDWORD = 0;
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    /* set tiles drawing */
    mapX =
        ((*psStruct).x as
             libc::c_uint).wrapping_sub((*(*psStruct).pStructureType).baseWidth.wrapping_mul(128
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_div(2
                                                                                                                                as
                                                                                                                                libc::c_int
                                                                                                                                as
                                                                                                                                libc::c_uint))
            >> 7 as libc::c_int;
    mapY =
        ((*psStruct).y as
             libc::c_uint).wrapping_sub((*(*psStruct).pStructureType).baseBreadth.wrapping_mul(128
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint).wrapping_div(2
                                                                                                                                  as
                                                                                                                                  libc::c_int
                                                                                                                                  as
                                                                                                                                  libc::c_uint))
            >> 7 as libc::c_int;
    i = 0 as libc::c_int as UDWORD;
    while i < (*(*psStruct).pStructureType).baseWidth {
        j = 0 as libc::c_int as UDWORD;
        while j < (*(*psStruct).pStructureType).baseBreadth {
            psTile = mapTile(mapX.wrapping_add(i), mapY.wrapping_add(j));
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
            (*psTile).tileInfoBits =
                ((*psTile).tileInfoBits as libc::c_int &
                     !(0x8 as libc::c_int)) as UBYTE;
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
}
// remove a structure from a game without any visible effects
// bDestroy = TRUE if the object is to be destroyed
// (for example used to change the type of wall at a location)
#[no_mangle]
pub unsafe extern "C" fn removeStruct(mut psDel: *mut STRUCTURE,
                                      mut bDestroy: BOOL) -> BOOL {
    //UDWORD		i,j;
    let mut resourceFound: BOOL = 0 as libc::c_int;
    let mut mask: UBYTE = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut cluster: SDWORD = 0;
    //UDWORD		mapX, mapY;
    let mut psAssemblyPoint: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyStruct: invalid structure pointer\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              6300 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"removeStruct\x00")).as_ptr(),
              b"PTRVALID(psDel, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bDestroy != 0 { removeStructFromMap(psDel); }
    //tell the power system its gone
    powerDestroyObject(psDel as *mut BASE_OBJECT);
    if bDestroy != 0 {
        //if the structure is a resource extractor, need to put the resource back in the map
	    /*ONLY IF ANY POWER LEFT - HACK HACK HACK!!!! OIL POOLS NEED TO KNOW
	    HOW MUCH IS THERE AND NOT RES EXTRACTORS */
        if (*(*psDel).pStructureType).type_0 ==
               REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            if (*((*psDel).pFunctionality as *mut RES_EXTRACTOR)).power != 0 {
                //clear the tile nodraw attribute so that will get set up when building the feature
                (*mapTile(((*psDel).x as libc::c_int >> 7 as libc::c_int) as
                              UDWORD,
                          ((*psDel).y as libc::c_int >> 7 as libc::c_int) as
                              UDWORD)).tileInfoBits =
                    ((*mapTile(((*psDel).x as libc::c_int >> 7 as libc::c_int)
                                   as UDWORD,
                               ((*psDel).y as libc::c_int >> 7 as libc::c_int)
                                   as UDWORD)).tileInfoBits as libc::c_int &
                         !(0x4 as libc::c_int)) as UBYTE;
                buildFeature(&mut *asFeatureStats.offset(oilResFeature as
                                                             isize),
                             (*psDel).x as UDWORD, (*psDel).y as UDWORD,
                             0 as libc::c_int);
                resourceFound = 1 as libc::c_int
            }
            //		else
    //		{
    //			addConsoleMessage("removeStruct - NO OIL LEFT IN POOL",
    //				DEFAULT_JUSTIFY);
    //		}
        }
    }
    if (*(*psDel).pStructureType).type_0 ==
           REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
        //tell associated Power Gen
        releaseResExtractor(psDel);
    }
    //Player only gets the power back if demolish structure themselves!******
	//put back the power required to maintain this structure (=power to build)
	//returnPower(psDel->player, psDel->pStructureType->powerToBuild);
    /*if (psDel->pStructureType->type == REF_POWER_GEN OR psDel->
		pStructureType->type == REF_RESOURCE_EXTRACTOR OR
		psDel->pStructureType->type == REF_HQ)
	{
		resetPlayerPower(psDel->player, psDel);
	}*/
    if (*(*psDel).pStructureType).type_0 ==
           REF_POWER_GEN as libc::c_int as libc::c_uint {
        //tell associated Res Extractors
        releasePowerGen(psDel);
    }
    //the droid check for this each frame - consistent with the rest of the game
    /*if (psDel->pStructureType->type == REF_REARM_PAD)
    {
        //tell associated VTOL droids
        releaseVTOLPad(psDel);
    }*/
    //check for a research topic currently under way
    if (*(*psDel).pStructureType).type_0 ==
           REF_RESEARCH as libc::c_int as libc::c_uint {
        if !(*((*psDel).pFunctionality as
                   *mut RESEARCH_FACILITY)).psSubject.is_null() {
            //cancel the topic
            cancelResearch(psDel);
        }
    }
    //subtract one from the structLimits list so can build another - don't allow to go less than zero!
    if (*asStructLimits[(*psDel).player as
                            usize].offset((*psDel).pStructureType.wrapping_offset_from(asStructureStats)
                                              as libc::c_int as
                                              isize)).currentQuantity != 0 {
        let ref mut fresh14 =
            (*asStructLimits[(*psDel).player as
                                 usize].offset((*psDel).pStructureType.wrapping_offset_from(asStructureStats)
                                                   as libc::c_int as
                                                   isize)).currentQuantity;
        *fresh14 = (*fresh14).wrapping_sub(1)
    }
    //if it is a factory - need to reset the factoryNumFlag
    if StructIsFactory(psDel) != 0 {
        psFactory = (*psDel).pFunctionality as *mut FACTORY;
        //not used anymore - 11/02/99
		/*if ( psFactory->psGroup != NULL )
		{
			grpLeave(psFactory->psGroup, NULL);
		}
		psFactory->psGroup = NULL;*/
        //need to initialise the production run as well
        cancelProduction(psDel);
        psAssemblyPoint = (*psFactory).psAssemblyPoint
    } else if (*(*psDel).pStructureType).type_0 ==
                  REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
        psAssemblyPoint =
            (*((*psDel).pFunctionality as
                   *mut REPAIR_FACILITY)).psDeliveryPoint
    }
    if !psAssemblyPoint.is_null() {
        mask =
            ((1 as libc::c_int) <<
                 (*psAssemblyPoint).factoryInc as libc::c_int) as UBYTE;
        factoryNumFlag[(*psDel).player as
                           usize][(*psAssemblyPoint).factoryType as usize] =
            (factoryNumFlag[(*psDel).player as
                                usize][(*psAssemblyPoint).factoryType as
                                           usize] as libc::c_int ^
                 mask as libc::c_int) as UBYTE;
        //need to cancel the repositioning of the DP if selectedPlayer and currently moving
        if (*psDel).player as libc::c_uint == selectedPlayer {
            //if currently trying to place a DP
            if tryingToGetLocation() != 0 {
                //need to check if this factory's DP is trying to be re-positioned
                if psAssemblyPoint ==
                       sBuildDetails.UserData as *mut FLAG_POSITION {
                    kill3DBuilding();
                }
            }
        }
    }
    // remove the structure from the grid
    gridRemoveObject(psDel as *mut BASE_OBJECT);
    // remove the structure from the cluster
    cluster = (*psDel).cluster as SDWORD;
    clustRemoveObject(psDel as *mut BASE_OBJECT);
    if bDestroy != 0 { killStruct(psDel); }
    /* remove animation if present */
    if !(*psDel).psCurAnim.is_null() {
        animObj_Remove(&mut (*psDel).psCurAnim,
                       (*(*(*psDel).psCurAnim).psAnim).uwID as libc::c_int);
        (*psDel).psCurAnim = 0 as *mut ANIM_OBJECT
    }
    clustUpdateCluster(apsStructLists[(*psDel).player as usize] as
                           *mut BASE_OBJECT, cluster);
    if (*psDel).player as libc::c_uint == selectedPlayer {
        intRefreshScreen();
    }
    return resourceFound;
}
/* Remove a structure and free it's memory */
#[no_mangle]
pub unsafe extern "C" fn destroyStruct(mut psDel: *mut STRUCTURE) -> BOOL {
    let mut mapX: UDWORD = 0;
    let mut mapY: UDWORD = 0;
    let mut width: UDWORD = 0;
    let mut breadth: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut widthScatter: UDWORD = 0;
    let mut breadthScatter: UDWORD = 0;
    let mut heightScatter: UDWORD = 0;
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut resourceFound: BOOL = 0 as libc::c_int;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    let mut bMinor: BOOL = 0;
    bMinor = 0 as libc::c_int;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"destroyStruct: invalid structure pointer\n\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              6464 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"destroyStruct\x00")).as_ptr(),
              b"PTRVALID(psDel, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if bMultiPlayer != 0 {
        //		if(!myResponsibility(psDel->player) )
//			&& (psDel->body != 0)) // can blow up utterly shagged structs.
//		{
//			return FALSE; // cant blow this up!
//		}
        SendDestroyStructure(psDel);
    }
    ((*(*psDel).pStructureType).type_0) ==
        REF_HQ as libc::c_int as libc::c_uint;
    //---------------------------------------
 	/* Only add if visible */
    if (*psDel).visible[selectedPlayer as usize] != 0 {
        /* Firstly, are we dealing with a wall section */
        if (*(*psDel).pStructureType).type_0 ==
               REF_WALL as libc::c_int as libc::c_uint ||
               (*(*psDel).pStructureType).type_0 ==
                   REF_WALLCORNER as libc::c_int as libc::c_uint {
            bMinor = 1 as libc::c_int
        }
        //---------------------------------------  Do we add immediate explosions?
		/* Set off some explosions, but not for walls */
		/* First Explosions */
        widthScatter = 128 as libc::c_int as UDWORD;
        breadthScatter = 128 as libc::c_int as UDWORD;
        heightScatter = 128 as libc::c_int as UDWORD;
        i = 0 as libc::c_int as UDWORD;
        while i <
                  (if bMinor != 0 {
                       2 as libc::c_int
                   } else { 4 as libc::c_int }) as UDWORD {
            // only add two for walls - gets crazy otherwise
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
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_MEDIUM,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            i = i.wrapping_add(1)
        }
        /* Get coordinates for everybody! */
        pos.x = (*psDel).x as int32;
        pos.z = (*psDel).y as int32;
        pos.y =
            map_Height(pos.x as UWORD as UDWORD, pos.z as UWORD as UDWORD) as
                int32;
        //--------------------------------------- Do we add a fire?
		// Set off a fire, provide dimensions for the fire
        if bMinor != 0 {
            effectGiveAuxVar(((*(*psDel).pStructureType).baseWidth <<
                                  7 as
                                      libc::c_int).wrapping_div(4 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint));
        } else {
            effectGiveAuxVar(((*(*psDel).pStructureType).baseWidth <<
                                  7 as
                                      libc::c_int).wrapping_div(3 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_uint));
        }
        if bMinor != 0 {
            // walls
            /* Give a duration */
            effectGiveAuxVarSec(1000 as libc::c_int as UDWORD);
            addEffect(&mut pos, EFFECT_FIRE, FIRE_TYPE_LOCALISED,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else if (*(*psDel).pStructureType).type_0 ==
                      REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
            /* Normal fire - no smoke */
            // oil resources
            /* Oil resources burn AND puff out smoke AND for longer*/
            effectGiveAuxVarSec(60000 as libc::c_int as UDWORD);
            addEffect(&mut pos, EFFECT_FIRE, FIRE_TYPE_SMOKY,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        } else {
            // everything else
            /* Give a duration */
            effectGiveAuxVarSec(10000 as libc::c_int as UDWORD);
            addEffect(&mut pos, EFFECT_FIRE, FIRE_TYPE_LOCALISED,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
        }
        //--------------------------------------- Do we add a destruction seq, and if so, which?
		/* Power stations have their own desctruction sequence */
        if (*(*psDel).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint {
            addEffect(&mut pos, EFFECT_DESTRUCTION,
                      DESTRUCTION_TYPE_POWER_STATION, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
            //	addEffect(&pos,EFFECT_SAT_LASER,SAT_LASER_STANDARD,FALSE,NULL,0);
            pos.y += 64 as libc::c_int;
            addEffect(&mut pos, EFFECT_EXPLOSION, EXPLOSION_TYPE_SHOCKWAVE,
                      0 as libc::c_int, 0 as *mut iIMDShape,
                      0 as libc::c_int);
            // give some power back to the player.
			//addPower(psDel->player,psDel->pStructureType->powerToBuild);
            addPower((*psDel).player as UDWORD, structPowerToBuild(psDel));
            //if it had a module attached, need to add the power for the base struct as well
            if (*((*psDel).pFunctionality as *mut POWER_GEN)).capacity != 0 {
                addPower((*psDel).player as UDWORD,
                         (*(*psDel).pStructureType).powerToBuild);
            }
        } else if bMinor != 0 {
            addEffect(&mut pos, EFFECT_DESTRUCTION,
                      DESTRUCTION_TYPE_WALL_SECTION, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
        } else {
            /* As do wall sections */
            // and everything else goes here.....
            addEffect(&mut pos, EFFECT_DESTRUCTION,
                      DESTRUCTION_TYPE_STRUCTURE, 0 as libc::c_int,
                      0 as *mut iIMDShape, 0 as libc::c_int);
        }
        //		addEffect(&pos,EFFECT_EXPLOSION,EXPLOSION_TYPE_SKYSCRAPER,TRUE,psDel->sDisplay.imd,0);
        //--------------------------------------- Start an earthquake...!
		/* shake the screen if we're near enough */
        if clipXY(pos.x, pos.z) != 0 {
            shakeStart();
            //attemptScreenShake();
        }
        //--------------------------------------- And finally, add a boom sound!!!!
		/* and add a sound effect */
        audio_PlayStaticTrack((*psDel).x as SDWORD, (*psDel).y as SDWORD,
                              ID_SOUND_EXPLOSION as libc::c_int);
    }
    //---------------------------------------------------------------------------------------
    resourceFound = removeStruct(psDel, 1 as libc::c_int);
    //once a struct is destroyed - it leaves a wrecked struct FEATURE in its place
	// Wall's don't leave wrecked features
    if (*psDel).visible[selectedPlayer as usize] != 0 {
        if resourceFound == 0 &&
               !((*(*psDel).pStructureType).type_0 ==
                     REF_WALL as libc::c_int as libc::c_uint) &&
               !((*(*psDel).pStructureType).type_0 ==
                     REF_WALLCORNER as libc::c_int as libc::c_uint) {
            mapX =
                ((*psDel).x as
                     libc::c_uint).wrapping_sub((*(*psDel).pStructureType).baseWidth.wrapping_mul(128
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_uint).wrapping_div(2
                                                                                                                                     as
                                                                                                                                     libc::c_int
                                                                                                                                     as
                                                                                                                                     libc::c_uint))
                    >> 7 as libc::c_int;
            mapY =
                ((*psDel).y as
                     libc::c_uint).wrapping_sub((*(*psDel).pStructureType).baseBreadth.wrapping_mul(128
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint).wrapping_div(2
                                                                                                                                       as
                                                                                                                                       libc::c_int
                                                                                                                                       as
                                                                                                                                       libc::c_uint))
                    >> 7 as libc::c_int;
            width = 0 as libc::c_int as UDWORD;
            while width < (*(*psDel).pStructureType).baseWidth {
                breadth = 0 as libc::c_int as UDWORD;
                while breadth < (*(*psDel).pStructureType).baseBreadth {
                    psTile =
                        mapTile(mapX.wrapping_add(width),
                                mapY.wrapping_add(breadth));
                    if (*psTile).tileVisBits as libc::c_int &
                           (1 as libc::c_int) << selectedPlayer != 0 {
                        (*psTile).illumination =
                            ((*psTile).illumination as libc::c_int /
                                 2 as libc::c_int) as UBYTE;
                        if (*psTile).bMaxed as libc::c_int != 0 &&
                               (*psTile).level as libc::c_int !=
                                   0xff as libc::c_int {
                            //	if(!TILE_OCCUPIED(psTile))
					   //	{
					   //	buildFeature((asFeatureStats + structFeature),
					   //			(mapX+width) << TILE_SHIFT, (mapY+breadth) << TILE_SHIFT, FALSE);
					   //	}
                            //only do one's already seen
                            (*psTile).level =
                                ((*psTile).level as libc::c_int /
                                     2 as libc::c_int) as UBYTE
                        }
                    }
                    breadth = breadth.wrapping_add(1)
                }
                width = width.wrapping_add(1)
            }
        }
    }
    /* remove animation if present */
    if !(*psDel).psCurAnim.is_null() {
        animObj_Remove(&mut (*psDel).psCurAnim,
                       (*(*(*psDel).psCurAnim).psAnim).uwID as libc::c_int);
        (*psDel).psCurAnim = 0 as *mut ANIM_OBJECT
    }
    // updates score stats only if not wall
    if (*(*psDel).pStructureType).type_0 !=
           REF_WALL as libc::c_int as libc::c_uint &&
           (*(*psDel).pStructureType).type_0 !=
               REF_WALLCORNER as libc::c_int as libc::c_uint {
        if (*psDel).player as libc::c_uint == selectedPlayer {
            scoreUpdateVar(WD_STR_LOST);
        } else { scoreUpdateVar(WD_STR_KILLED); }
    }
    return 1 as libc::c_int;
}
/* For now all this does is work out what height the terrain needs to be set to
   An actual foundation structure may end up being placed down
   The x and y passed in are the CENTRE of the structure*/
#[no_mangle]
pub unsafe extern "C" fn buildFoundation(mut psStructStats:
                                             *mut STRUCTURE_STATS,
                                         mut x: UDWORD, mut y: UDWORD)
 -> SWORD {
    let mut width: UDWORD = 0; //, foundationMin, foundationMax;
    let mut breadth: UDWORD = 0;
    //UDWORD	max, min, startX, startY;
    let mut startX: UDWORD = 0;
    let mut startY: UDWORD = 0;
    let mut height: SWORD = 0;
    let mut foundationMin: SWORD = 0;
    let mut foundationMax: SWORD = 0;
    startX =
        (x >>
             7 as
                 libc::c_int).wrapping_sub((*psStructStats).baseWidth.wrapping_div(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint));
    startY =
        (y >>
             7 as
                 libc::c_int).wrapping_sub((*psStructStats).baseBreadth.wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint));
    //check the terrain is the correct type return -1 if not
    //shouldn't need to do this but doesn't take long hey?!
	/*check if there is a structure next to the new one - return the height of the
	  structure if found*/
    breadth = 0 as libc::c_int as UDWORD;
    while breadth <= (*psStructStats).baseBreadth {
        width = 0 as libc::c_int as UDWORD;
        while width <= (*psStructStats).baseWidth {
            //psor			if (mapTile(startX+width, startY+breadth)->psObject && mapTile(startX+width,
//psor				startY+breadth)->psObject->type == OBJ_STRUCTURE)
            if (*mapTile(startX.wrapping_add(width),
                         startY.wrapping_add(breadth))).tileInfoBits as
                   libc::c_int & 0x1 as libc::c_int != 0 {
                return map_TileHeight(startX.wrapping_add(width),
                                      startY.wrapping_add(breadth))
                //				return ((SWORD)mapTile(startX+width, startY+breadth)->height);
            }
            width = width.wrapping_add(1)
        }
        breadth = breadth.wrapping_add(1)
    }
    //may also have to check that overlapping terrain can be set to the average height
	//eg water - don't want it to 'flow' into the structure if this effect is coded!
    startX =
        (x >>
             7 as
                 libc::c_int).wrapping_sub((*psStructStats).baseWidth.wrapping_div(2
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint));
    startY =
        (y >>
             7 as
                 libc::c_int).wrapping_sub((*psStructStats).baseBreadth.wrapping_div(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint));
    //initialise the starting values so they get set in loop
    foundationMin = (255 as libc::c_int * 2 as libc::c_int) as SWORD;
    foundationMax = 0 as libc::c_int as SWORD;
    breadth = 0 as libc::c_int as UDWORD;
    while breadth <= (*psStructStats).baseBreadth {
        width = 0 as libc::c_int as UDWORD;
        while width <= (*psStructStats).baseWidth {
            /*getTileMaxMin(startX + width, startY + breadth, &max, &min);
			if (foundationMin > min)
			{
				foundationMin = min;
			}
			if (foundationMax < max)
			{
				foundationMax = max;
			}*/
            height =
                map_TileHeight(startX.wrapping_add(width),
                               startY.wrapping_add(breadth));
            if foundationMin as libc::c_int > height as libc::c_int {
                foundationMin = height
            }
            if (foundationMax as libc::c_int) < height as libc::c_int {
                foundationMax = height
            }
            width = width.wrapping_add(1)
        }
        breadth = breadth.wrapping_add(1)
    }
    //return the average of max/min height
    return ((foundationMin as libc::c_int + foundationMax as libc::c_int) /
                2 as libc::c_int) as SWORD;
}
/* gets a structure stat from its name - relies on the name being unique (or it will
   return the first one it finds!! */
#[no_mangle]
pub unsafe extern "C" fn getStructStatFromName(mut pName: *mut STRING)
 -> SDWORD {
    let mut inc: UDWORD = 0;
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    inc = 0 as libc::c_int as UDWORD;
    while inc < numStructureStats {
        psStat =
            &mut *asStructureStats.offset(inc as isize) as
                *mut STRUCTURE_STATS;
        if strcmp((*psStat).pName, pName) == 0 { return inc as SDWORD }
        inc = inc.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
/*check to see if the structure is 'doing' anything  - return TRUE if idle*/
#[no_mangle]
pub unsafe extern "C" fn structureIdle(mut psBuilding: *mut STRUCTURE)
 -> BOOL {
    let mut pSubject: *mut BASE_STATS = 0 as *mut BASE_STATS;
    if !(*psBuilding).pFunctionality.is_null() {
        //determine the Subject
        match (*(*psBuilding).pStructureType).type_0 {
            10 => {
                pSubject =
                    (*((*psBuilding).pFunctionality as
                           *mut RESEARCH_FACILITY)).psSubject
            }
            1 | 16 | 17 => {
                pSubject =
                    (*((*psBuilding).pFunctionality as
                           *mut FACTORY)).psSubject
            }
            _ => { }
        }
        if !pSubject.is_null() { return 0 as libc::c_int }
    }
    return 1 as libc::c_int;
}
/*checks to see if any structure exists of a specified type with a specified status */
#[no_mangle]
pub unsafe extern "C" fn checkStructureStatus(mut psStats:
                                                  *mut STRUCTURE_STATS,
                                              mut player: UDWORD,
                                              mut status: UDWORD) -> BOOL {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut found: BOOL = 0 as libc::c_int;
    psStructure = apsStructLists[player as usize];
    while !psStructure.is_null() {
        if (*(*psStructure).pStructureType).type_0 == (*psStats).type_0 {
            //need to check if THIS instance of the type has the correct status
            if (*psStructure).status as libc::c_uint == status {
                found = 1 as libc::c_int;
                break ;
            }
        }
        psStructure = (*psStructure).psNext
    }
    return found;
}
/*checks to see if a specific structure type exists -as opposed to a structure
stat type*/
#[no_mangle]
pub unsafe extern "C" fn checkSpecificStructExists(mut structInc: UDWORD,
                                                   mut player: UDWORD)
 -> BOOL {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut found: BOOL = 0 as libc::c_int;
    if structInc < numStructureStats {
    } else {
        debug(LOG_ERROR,
              b"checkSpecificStructExists: invalid structure inc\x00" as
                  *const u8 as *const libc::c_char);
    };
    if structInc < numStructureStats {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              6832 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"checkSpecificStructExists\x00")).as_ptr(),
              b"structInc < numStructureStats\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStructure = apsStructLists[player as usize];
    while !psStructure.is_null() {
        if (*psStructure).status as libc::c_int == SS_BUILT as libc::c_int {
            if (*(*psStructure).pStructureType).ref_0.wrapping_sub(0xd0000 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint)
                   == structInc {
                found = 1 as libc::c_int;
                break ;
            }
        }
        psStructure = (*psStructure).psNext
    }
    return found;
}
//This function not used
/* return true if within range of a building */
/*BOOL validAssemblyPoint(UDWORD x, UDWORD y,UDWORD player,FLAG_POSITION *psCurr,BOOL bNew)
{
	return TRUE;
}*/
/*
	STRUCTURE		*psStruct;
	DWORD			xd,yd;
	FLAG_POSITION	*psFlag;

	// valid if within x squares of another ass. point, or x squares within factory.
	// close to the factory....
	for(psStruct = apsStructLists[player];psStruct; psStruct=psStruct->psNext)
	{
		if ((psStruct->pStructureType->type == REF_FACTORY OR
			psStruct->pStructureType->type == REF_CYBORG_FACTORY OR
			psStruct->pStructureType->type == REF_VTOL_FACTORY )
			&& ((FACTORY *)psStruct->pFunctionality)->psAssemblyPoint->factoryInc == psCurr->factoryInc)
		{
			xd = (psStruct->x - x);
			yd = (psStruct->y - y);
			if( ((xd*xd)+(yd*yd)) < (DWORD)(ASSEMBLY_RANGE*ASSEMBLY_RANGE))
			{
				return TRUE;
			}
		}
	}

	// check the previous point.
	if(bNew)
	{
		psFlag  = psCurr;
	}
	else
	{
		if(psCurr->factorySub == 0 )		// don't check first point.
		{
			return FALSE;
		}
		for(psFlag = apsFlagPosLists[player];
		psFlag->factorySub != psCurr->factorySub-1;
		psFlag= psFlag->psNext);
	}

	//check it.
	xd = (psFlag->coords.x - x);
	yd = (psFlag->coords.y - y);
	if( ((xd*xd)+(yd*yd)) < (DWORD)(ASSEMBLY_RANGE*ASSEMBLY_RANGE))
	{
		return TRUE;
	}

	return FALSE;
#endif
*/
/*finds a suitable position for the assembly point based on one passed in*/
unsafe extern "C" fn findAssemblyPointPosition(mut pX: *mut UDWORD,
                                               mut pY: *mut UDWORD,
                                               mut player: UDWORD) {
    //check if valid location
	/*if (TILE_OCCUPIED(mapTile(*pX,*pY)))
	{
		if (!pickATileGen(pX, pY, LOOK_FOR_EMPTY_TILE,normalPAT))
		{
			ASSERT( FALSE, "findAssemblyPointPosition: Unable to find a free location" );
		}
	}
	else
	{
		//check its not blocking or anything like that
		if (!sensiblePlace(*pX,*pY))
		{
			if (!pickATileGen(pX, pY, LOOK_FOR_EMPTY_TILE,normalPAT))
			{
				ASSERT( FALSE, "findAssemblyPointPosition: Unable to find a free location" );
			}
		}
	}*/
    //set up a dummy stat pointer
    let mut sStats: STRUCTURE_STATS =
        STRUCTURE_STATS{ref_0: 0,
                        pName: 0 as *mut STRING,
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
                        pIMD: 0 as *mut iIMDShape,
                        pBaseIMD: 0 as *mut iIMDShape,
                        pECM: 0 as *mut _ecm_stats,
                        pSensor: 0 as *mut _sensor_stats,
                        psWeapStat: 0 as *mut _weapon_stats,
                        numFuncs: 0,
                        defaultFunc: 0,
                        asFuncList: 0 as *mut *mut _function,};
    let mut passes: UDWORD = 0 as libc::c_int as UDWORD;
    let mut i: SDWORD = 0;
    let mut j: SDWORD = 0;
    let mut startX: SDWORD = 0;
    let mut endX: SDWORD = 0;
    let mut startY: SDWORD = 0;
    let mut endY: SDWORD = 0;
    sStats.ref_0 = 0 as libc::c_int as UDWORD;
    sStats.baseWidth = 1 as libc::c_int as UDWORD;
    sStats.baseBreadth = 1 as libc::c_int as UDWORD;
    /* Initial box dimensions and set iteration count to zero */
    endX = *pX as SDWORD;
    startX = endX;
    endY = *pY as SDWORD;
    startY = endY;
    passes = 0 as libc::c_int as UDWORD;
    //if the value passed in is not a valid location - find one!
    if validLocation(&mut sStats as *mut STRUCTURE_STATS as *mut BASE_STATS,
                     *pX, *pY, player, 0 as libc::c_int) == 0 {
        /* Keep going until we get a tile or we exceed distance */
        while passes < 20 as libc::c_int as libc::c_uint {
            /* Process whole box */
            i = startX;
            while i <= endX {
                j = startY;
                while j <= endY {
                    /* Test only perimeter as internal tested previous iteration */
                    if i == startX || i == endX || j == startY || j == endY {
                        /* Good enough? */
                        if validLocation(&mut sStats as *mut STRUCTURE_STATS
                                             as *mut BASE_STATS, i as UDWORD,
                                         j as UDWORD, player,
                                         0 as libc::c_int) != 0 {
                            /* Set exit conditions and get out NOW */
                            *pX = i as UDWORD;
                            *pY = j as UDWORD;
                            //jump out of the loop
                            return
                        }
                    }
                    j += 1
                }
                i += 1
            }
            /* Expand the box out in all directions - off map handled by validLocation() */
            startX -= 1;
            startY -= 1;
            endX += 1;
            endY += 1;
            passes = passes.wrapping_add(1)
        }
    } else {
        //the first location was valid
        return
    }
    /* If we got this far, then we failed - passed in values will be unchanged */
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"findAssemblyPointPosition: unable to find a valid location!\x00"
                  as *const u8 as *const libc::c_char);
    };
    if 0 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              6979 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 26],
                                        &[libc::c_char; 26]>(b"findAssemblyPointPosition\x00")).as_ptr(),
              b"FALSE\x00" as *const u8 as *const libc::c_char);
    };
}
/*sets the point new droids go to - x/y in world coords for a Factory
bCheck is set to TRUE for initial placement of the Assembly Point*/
#[no_mangle]
pub unsafe extern "C" fn setAssemblyPoint(mut psAssemblyPoint:
                                              *mut FLAG_POSITION,
                                          mut x: UDWORD, mut y: UDWORD,
                                          mut player: UDWORD,
                                          mut bCheck: BOOL) {
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"setAssemblyPoint: invalid AssemblyPoint pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              6988 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"setAssemblyPoint\x00")).as_ptr(),
              b"PTRVALID(psAssemblyPoint, sizeof(FLAG_POSITION))\x00" as
                  *const u8 as *const libc::c_char);
    };
    //check its valid
    x = x >> 7 as libc::c_int;
    y = y >> 7 as libc::c_int;
    if bCheck != 0 { findAssemblyPointPosition(&mut x, &mut y, player); }
    //add half a tile so the centre is in the middle of the tile
    x =
        (x <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    y =
        (y <<
             7 as
                 libc::c_int).wrapping_add((128 as libc::c_int /
                                                2 as libc::c_int) as
                                               libc::c_uint);
    (*psAssemblyPoint).coords.x = x as int32;
    (*psAssemblyPoint).coords.y = y as int32;
    // Deliv Point sits at the height of the tile it's centre is on + arbitary amount!
    (*psAssemblyPoint).coords.z =
        map_Height(x as UWORD as UDWORD, y as UWORD as UDWORD) as libc::c_int
            + 10 as libc::c_int;
}
/*sets the factory Inc for the Assembly Point*/
unsafe extern "C" fn setFlagPositionInc(mut pFunctionality: *mut libc::c_void,
                                        mut player: UDWORD,
                                        mut factoryType: UBYTE) {
    let mut inc: UBYTE = 0;
    let mut mask: UBYTE = 1 as libc::c_int as UBYTE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    //that uses the variable.
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"setFlagPositionInc: invalid player number\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              7019 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"setFlagPositionInc\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    //find the first vacant slot
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_int) < 5 as libc::c_int {
        if factoryNumFlag[player as usize][factoryType as usize] as
               libc::c_int & mask as libc::c_int != mask as libc::c_int {
            break ;
        }
        mask = ((mask as libc::c_int) << 1 as libc::c_int) as UBYTE;
        inc = inc.wrapping_add(1)
    }
    if inc as libc::c_int >= 5 as libc::c_int {
        //this may happen now with electronic warfare
        inc = 1 as libc::c_int as UBYTE
    }
    if factoryType as libc::c_int == 3 as libc::c_int {
        psRepair = pFunctionality as *mut REPAIR_FACILITY;
        //		factoryNumFlag[player][factoryType] |= mask;
        (*(*psRepair).psDeliveryPoint).factoryInc = 0 as libc::c_int as UBYTE;
        (*(*psRepair).psDeliveryPoint).factoryType = factoryType
    } else {
        psFactory = pFunctionality as *mut FACTORY;
        (*(*psFactory).psAssemblyPoint).factoryInc = inc;
        (*(*psFactory).psAssemblyPoint).factoryType = factoryType;
        factoryNumFlag[player as usize][factoryType as usize] =
            (factoryNumFlag[player as usize][factoryType as usize] as
                 libc::c_int | mask as libc::c_int) as UBYTE
    };
}
//		psRepair->psAssemblyPoint->factoryInc = inc;
/* called from order.c.. delivery/assembly point handler*/
/*now called from display.c */
#[no_mangle]
pub unsafe extern "C" fn processDeliveryPoint(mut player: UDWORD,
                                              mut x: UDWORD, mut y: UDWORD) {
    let mut psCurrFlag: *mut FLAG_POSITION =
        0 as *mut FLAG_POSITION; //,*psFlag;//,*psNewFlag
    //	STRUCTURE		*psStruct;
//	FACTORY			*psFactory = NULL;
//	UBYTE			factoryFlag = 0;
    //	if(bInTutorial)
//	{
//		eventFireCallbackTrigger(CALL_DELIVPOINTMOVED);
// /	}
    psCurrFlag = apsFlagPosLists[player as usize];
    while !psCurrFlag.is_null() {
        // must be selected and have a valid pos.
        if (*psCurrFlag).selected != 0 {
            //find the factory belonging to this flag.
/*			for(psStruct=apsStructLists[player];psStruct;psStruct=psStruct->psNext)
			{
				if (((psStruct->pStructureType->type == REF_FACTORY AND
					psCurrFlag->factoryType == FACTORY_FLAG)
					OR
					(psStruct->pStructureType->type == REF_CYBORG_FACTORY AND
					psCurrFlag->factoryType == CYBORG_FLAG)
					OR
					(psStruct->pStructureType->type == REF_VTOL_FACTORY AND
					psCurrFlag->factoryType == VTOL_FLAG)) &&
					((FACTORY *)psStruct->pFunctionality)->psAssemblyPoint->
					factoryInc == psCurrFlag->factoryInc)
				{
					factoryFlag = FACTORY_FLAG;
					if (psStruct->pStructureType->type == REF_CYBORG_FACTORY)
					{
						factoryFlag = CYBORG_FLAG;
					}
					else if (psStruct->pStructureType->type == REF_VTOL_FACTORY)
					{
						factoryFlag = VTOL_FLAG;
					}
					psFactory = (FACTORY *)psStruct->pFunctionality;
					break;
				}
			}*/
            //add additional assembly point
	/*		if( (keyDown(KEY_LCTRL) || keyDown(KEY_RCTRL))
				&&	validAssemblyPoint(x,y,player,psCurrFlag,TRUE) )
			{
				psFactory->psAssemblyPoint->primary = FALSE;			//disable old priamry

				if(createFlagPosition(&psNewFlag, psCurrFlag->player))
				{
					addFlagPosition(psNewFlag);								//create

					psNewFlag->factoryInc = psCurrFlag->factoryInc;			// factory to bind to.
					psNewFlag->factoryType = factoryFlag;
					psNewFlag->primary	  = TRUE;							// set primary.
					psNewFlag->factorySub = (UBYTE)(psFactory->psAssemblyPoint->factorySub + 1);// number of fact points.
					setAssemblyPoint(psNewFlag,x,y);
					psFactory->psAssemblyPoint = psNewFlag;					//update factory to point to new point.
				}
				return;
			}
			else
			{
*/
            //this is not the right function to call anymore - and since there are no
            //intermediate DPs is no longer necessary
			//if(validAssemblyPoint(x,y,player,psCurrFlag,FALSE))
            // move existing point
                /*we do need to check if its valid since the mouse click can
                be processed the frame after the interface has been up*/
				//setAssemblyPoint(psCurrFlag, x, y, player, FALSE);
            setAssemblyPoint(psCurrFlag, x, y, player, 1 as libc::c_int);
            //deselect once moved
            (*psCurrFlag).selected = 0 as libc::c_int;
            return
        }
        psCurrFlag = (*psCurrFlag).psNext
    };
}
//will want to break if more than one can be selected?
/*called when a structure has been built - checks through the list of callbacks
for the scripts*/
#[no_mangle]
pub unsafe extern "C" fn structureCompletedCallback(mut psStructType:
                                                        *mut STRUCTURE_STATS) {
    if (*psStructType).type_0 == REF_POWER_GEN as libc::c_int as libc::c_uint
       {
        eventFireCallbackTrigger(CALL_POWERGEN_BUILT as libc::c_int as
                                     TRIGGER_TYPE);
    }
    if (*psStructType).type_0 ==
           REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
        eventFireCallbackTrigger(CALL_RESEX_BUILT as libc::c_int as
                                     TRIGGER_TYPE);
    }
    if (*psStructType).type_0 == REF_RESEARCH as libc::c_int as libc::c_uint {
        eventFireCallbackTrigger(CALL_RESEARCH_BUILT as libc::c_int as
                                     TRIGGER_TYPE);
    }
    if (*psStructType).type_0 == REF_FACTORY as libc::c_int as libc::c_uint ||
           (*psStructType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*psStructType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        eventFireCallbackTrigger(CALL_FACTORY_BUILT as libc::c_int as
                                     TRIGGER_TYPE);
    };
}
#[no_mangle]
pub unsafe extern "C" fn structGetDemolishStat() -> *mut STRUCTURE_STATS {
    if g_psStatDestroyStruct.is_null() {
        debug(LOG_ERROR,
              b"structGetDemolishStat: stat not initialised1\n\x00" as
                  *const u8 as *const libc::c_char);
        abort();
    }
    return g_psStatDestroyStruct;
}
/*sets the flag to indicate a Power Generator Exists - so do Oil Derrick anim*/
/*void setPowerGenExists(BOOL state, UDWORD player)
{
	powerGenExists[player] = (UBYTE)state;
}*/
/*returns the status of the flag*/
/*BOOL getPowerGenExists(UDWORD player)
{
	return powerGenExists[player];
}*/
/*sets the flag to indicate a HQ Exists - so draw Radar*/
#[no_mangle]
pub unsafe extern "C" fn setHQExists(mut state: BOOL, mut player: UDWORD) {
    hqExists[player as usize] = state as UBYTE;
}
/*returns the status of the flag*/
#[no_mangle]
pub unsafe extern "C" fn getHQExists(mut player: UDWORD) -> BOOL {
    //#ifndef PSX
//	if(bMultiPlayer && game.type == DMATCH)
//	{
//		return TRUE;
//	}
//#endif
    return hqExists[player as usize] as BOOL;
}
/*sets the flag to indicate a SatUplink Exists - so draw everything!*/
#[no_mangle]
pub unsafe extern "C" fn setSatUplinkExists(mut state: BOOL,
                                            mut player: UDWORD) {
    satUplinkExists[player as usize] = state as UBYTE;
}
/*returns the status of the flag*/
#[no_mangle]
pub unsafe extern "C" fn getSatUplinkExists(mut player: UDWORD) -> BOOL {
    return satUplinkExists[player as usize] as BOOL;
}
/*sets the flag to indicate a Las Sat Exists - ONLY EVER WANT ONE*/
#[no_mangle]
pub unsafe extern "C" fn setLasSatExists(mut state: BOOL,
                                         mut player: UDWORD) {
    lasSatExists[player as usize] = state as UBYTE;
}
/*returns the status of the flag*/
#[no_mangle]
pub unsafe extern "C" fn getLasSatExists(mut player: UDWORD) -> BOOL {
    return lasSatExists[player as usize] as BOOL;
}
/* calculate muzzle tip location in 3d world */
#[no_mangle]
pub unsafe extern "C" fn calcStructureMuzzleLocation(mut psStructure:
                                                         *mut STRUCTURE,
                                                     mut muzzle: *mut iVector)
 -> BOOL {
    let mut barrel: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut psShape: *mut iIMDShape = 0 as *mut iIMDShape;
    let mut psWeaponImd: *mut iIMDShape = 0 as *mut iIMDShape;
    psShape = (*(*psStructure).pStructureType).pIMD;
    //if (psStructure->numWeaps > 0)
    if (*psStructure).asWeaps[0 as libc::c_int as usize].nStat >
           0 as libc::c_int as libc::c_uint {
        psWeaponImd =
            (*asWeaponStats.offset((*psStructure).asWeaps[0 as libc::c_int as
                                                              usize].nStat as
                                       isize)).pIMD
    } else { psWeaponImd = 0 as *mut iIMDShape }
    if !psShape.is_null() && (*psShape).nconnectors != 0 {
        // This code has not been translated to the PSX Yet !!!!                                     (sorry)
        pie_MatBegin();
        pie_TRANSLATE((*psStructure).x as libc::c_int,
                      -((*psStructure).z as SDWORD),
                      (*psStructure).y as libc::c_int);
        //matrix = the center of droid
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*psStructure).direction as SDWORD);
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*psStructure).pitch as libc::c_int);
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        -((*psStructure).roll as SDWORD));
        //		pie_TRANSLATE(100,0,0);			//	(left,-height,forward)
        pie_TRANSLATE((*(*psShape).connectors).x, -(*(*psShape).connectors).z,
                      -(*(*psShape).connectors).y); //note y and z flipped
        //matrix = the gun and turret mount on the body
        pie_MatRotY(65536 as libc::c_int / 360 as libc::c_int *
                        (*psStructure).turretRotation as
                            SDWORD); //+ve anticlockwise
        pie_MatRotX(65536 as libc::c_int / 360 as libc::c_int *
                        (*psStructure).turretPitch as libc::c_int); //+ve up
        pie_MatRotZ(65536 as libc::c_int / 360 as libc::c_int *
                        0 as libc::c_int);
        //matrix = the muzzle mount on turret
        if !psWeaponImd.is_null() && (*psWeaponImd).nconnectors != 0 {
            barrel.x = (*(*psWeaponImd).connectors).x;
            barrel.y = -(*(*psWeaponImd).connectors).y;
            barrel.z = -(*(*psWeaponImd).connectors).z
        } else {
            barrel.x = 0 as libc::c_int;
            barrel.y = 0 as libc::c_int;
            barrel.z = 0 as libc::c_int
        }
        (*muzzle).x =
            barrel.x * (*psMatrix).a + barrel.z * (*psMatrix).d +
                barrel.y * (*psMatrix).g + (*psMatrix).j;
        (*muzzle).z =
            barrel.x * (*psMatrix).b + barrel.z * (*psMatrix).e +
                barrel.y * (*psMatrix).h + (*psMatrix).k;
        (*muzzle).y =
            barrel.x * (*psMatrix).c + barrel.z * (*psMatrix).f +
                barrel.y * (*psMatrix).i + (*psMatrix).l;
        (*muzzle).x >>= 12 as libc::c_int;
        (*muzzle).z >>= 12 as libc::c_int;
        (*muzzle).y >>= 12 as libc::c_int;
        (*muzzle).z = -(*muzzle).z;
        pie_MatEnd();
    } else {
        (*muzzle).x = (*psStructure).x as int32;
        (*muzzle).y = (*psStructure).y as int32;
        (*muzzle).z =
            (*psStructure).z as libc::c_int +
                (*(*psStructure).sDisplay.imd).ymax
    }
    return 1 as libc::c_int;
}
/*Looks through the list of structures to see if there are any inactive
resource extractors*/
#[no_mangle]
pub unsafe extern "C" fn checkForResExtractors(mut psBuilding:
                                                   *mut STRUCTURE) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut psResExtractor: *mut RES_EXTRACTOR = 0 as *mut RES_EXTRACTOR;
    let mut i: UDWORD = 0;
    let mut slot: SDWORD = 0;
    if (*(*psBuilding).pStructureType).type_0 !=
           REF_POWER_GEN as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"checkForResExtractors: invalid structure type\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7361 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 22],
                                            &[libc::c_char; 22]>(b"checkForResExtractors\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    psPowerGen = (*psBuilding).pFunctionality as *mut POWER_GEN;
    //count the number of allocated slots
    slot = 0 as libc::c_int; //-1;
    //for (i=0; i < (NUM_POWER_MODULES + 1); i++)
    i = 0 as libc::c_int as UDWORD;
    while i < 4 as libc::c_int as libc::c_uint {
        if !(*psPowerGen).apResExtractors[i as usize].is_null() {
            slot += 1;
            //make sure the derrrick is active if any oil left
            psResExtractor =
                (*(*psPowerGen).apResExtractors[i as usize]).pFunctionality as
                    *mut RES_EXTRACTOR;
            if (*psResExtractor).power != 0 {
                (*psResExtractor).active = 1 as libc::c_int
            }
        }
        i = i.wrapping_add(1)
    }
    psResExtractor = 0 as *mut RES_EXTRACTOR;
    //each Power Gen can cope with 4 Extractors now - 9/6/98 AB
	//check capacity against number of filled slots
	//if (slot < (SDWORD)psPowerGen->capacity)
    if slot < 4 as libc::c_int {
        psCurr = apsStructLists[(*psBuilding).player as usize];
        while !psCurr.is_null() {
            if (*(*psCurr).pStructureType).type_0 ==
                   REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
                psResExtractor =
                    (*psCurr).pFunctionality as *mut RES_EXTRACTOR;
                //check not connected and power left and built!
                if (*((*psCurr).pFunctionality as *mut RES_EXTRACTOR)).active
                       == 0 &&
                       (*psCurr).status as libc::c_int ==
                           SS_BUILT as libc::c_int &&
                       (*((*psCurr).pFunctionality as
                              *mut RES_EXTRACTOR)).power != 0 {
                    //assign the extractor to the power generator - use first vacant slot
					//for (i = 0; i < (NUM_POWER_MODULES+1); i++)
                    i = 0 as libc::c_int as UDWORD;
                    while i < 4 as libc::c_int as libc::c_uint {
                        if (*psPowerGen).apResExtractors[i as usize].is_null()
                           {
                            (*psPowerGen).apResExtractors[i as usize] =
                                psCurr;
                            break ;
                        } else { i = i.wrapping_add(1) }
                    }
                    //set the owning power gen up for the resource extractor
                    (*psResExtractor).psPowerGen = psBuilding;
                    //set the res Extr to active
                    (*psResExtractor).active = 1 as libc::c_int;
                    (*psResExtractor).timeLastUpdated = gameTime;
                    slot += 1;
                    //each Power Gen can cope with 4 Extractors now - 9/6/98 AB
					//check to see if any more vacant slots
					//if (slot >= (SDWORD)psPowerGen->capacity)
                    if slot >= 4 as libc::c_int { break ; }
                }
            }
            psCurr = (*psCurr).psNext
        }
    };
}
/*Looks through the list of structures to see if there are any Power Gens
with available slots for the new Res Ext*/
#[no_mangle]
pub unsafe extern "C" fn checkForPowerGen(mut psBuilding: *mut STRUCTURE) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut i: UDWORD = 0;
    let mut slot: SDWORD = 0;
    let mut psPG: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut psRE: *mut RES_EXTRACTOR = 0 as *mut RES_EXTRACTOR;
    if (*(*psBuilding).pStructureType).type_0 !=
           REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"checkForPowerGen: invalid structure type\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7443 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"checkForPowerGen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    psRE = (*psBuilding).pFunctionality as *mut RES_EXTRACTOR;
    if (*psRE).active != 0 { return }
    //loop thru the current structures
    psCurr = apsStructLists[(*psBuilding).player as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint &&
               (*psCurr).status as libc::c_int == SS_BUILT as libc::c_int {
            psPG = (*psCurr).pFunctionality as *mut POWER_GEN;
            //check capacity against number of filled slots
            slot = 0 as libc::c_int; //-1;
            //for (i=0; i < (NUM_POWER_MODULES + 1); i++)
            i = 0 as libc::c_int as UDWORD;
            while i < 4 as libc::c_int as libc::c_uint {
                if !(*psPG).apResExtractors[i as usize].is_null() {
                    slot += 1
                }
                i = i.wrapping_add(1)
            }
            //each Power Gen can cope with 4 Extractors now - 9/6/98 AB
			//each Power Gen can cope with 4 extractors
			//if ((SDWORD)psPG->capacity > slot)
            if slot < 4 as libc::c_int {
                //find the first vacant slot
				//for (i=0; i < (NUM_POWER_MODULES + 1); i++)
                i = 0 as libc::c_int as UDWORD;
                while i < 4 as libc::c_int as libc::c_uint {
                    if (*psPG).apResExtractors[i as usize].is_null() {
                        (*psPG).apResExtractors[i as usize] = psBuilding;
                        (*psRE).psPowerGen = psCurr;
                        (*psRE).active = 1 as libc::c_int;
                        (*psRE).timeLastUpdated = gameTime;
                        return
                    }
                    i = i.wrapping_add(1)
                }
            }
        }
        psCurr = (*psCurr).psNext
    };
}
/*initialise the slot the Resource Extractor filled in the owning Power Gen*/
unsafe extern "C" fn informPowerGen(mut psStruct: *mut STRUCTURE) {
    let mut i: UDWORD = 0;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    if (*(*psStruct).pStructureType).type_0 !=
           REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"informPowerGen: invalid structure type\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7500 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 15],
                                            &[libc::c_char; 15]>(b"informPowerGen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    //get the owning power generator
    psPowerGen =
        (*(*((*psStruct).pFunctionality as
                 *mut RES_EXTRACTOR)).psPowerGen).pFunctionality as
            *mut POWER_GEN;
    if !psPowerGen.is_null() {
        //for (i=0; i < NUM_POWER_MODULES + 1; i++)
        i = 0 as libc::c_int as UDWORD;
        while i < 4 as libc::c_int as libc::c_uint {
            if (*psPowerGen).apResExtractors[i as usize] == psStruct {
                //initialise the 'slot'
                (*psPowerGen).apResExtractors[i as usize] =
                    0 as *mut _structure;
                break ;
            } else { i = i.wrapping_add(1) }
        }
    };
}
/*called when a Res extractor is destroyed or runs out of power or is disconnected
adjusts the owning Power Gen so that it can link to a different Res Extractor if one
is available*/
#[no_mangle]
pub unsafe extern "C" fn releaseResExtractor(mut psRelease: *mut STRUCTURE) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    if (*(*psRelease).pStructureType).type_0 !=
           REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"releaseResExtractor:Invalid structure type\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7531 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 20],
                                            &[libc::c_char; 20]>(b"releaseResExtractor\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    //tell associated Power Gen
    if !(*((*psRelease).pFunctionality as
               *mut RES_EXTRACTOR)).psPowerGen.is_null() {
        informPowerGen(psRelease);
    }
    let ref mut fresh15 =
        (*((*psRelease).pFunctionality as *mut RES_EXTRACTOR)).psPowerGen;
    *fresh15 = 0 as *mut _structure;
    //there may be spare resource extractors
    psCurr = apsStructLists[(*psRelease).player as usize];
    while !psCurr.is_null() {
        //check not connected and power left and built!
        if (*(*psCurr).pStructureType).type_0 ==
               REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint &&
               psCurr != psRelease &&
               (*((*psCurr).pFunctionality as *mut RES_EXTRACTOR)).active == 0
               &&
               (*((*psCurr).pFunctionality as *mut RES_EXTRACTOR)).power != 0
               && (*psCurr).status as libc::c_int == SS_BUILT as libc::c_int {
            checkForPowerGen(psCurr);
        }
        psCurr = (*psCurr).psNext
    };
}
/*called when a Power Gen is destroyed or is disconnected
adjusts the associated Res Extractors so that they can link to different Power
Gens if any are available*/
#[no_mangle]
pub unsafe extern "C" fn releasePowerGen(mut psRelease: *mut STRUCTURE) {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    let mut i: UDWORD = 0;
    if (*(*psRelease).pStructureType).type_0 !=
           REF_POWER_GEN as libc::c_int as libc::c_uint {
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"releasePowerGen:Invalid structure type\x00" as *const u8
                      as *const libc::c_char);
        };
        if 0 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7569 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 16],
                                            &[libc::c_char; 16]>(b"releasePowerGen\x00")).as_ptr(),
                  b"FALSE\x00" as *const u8 as *const libc::c_char);
        };
        return
    }
    psPowerGen = (*psRelease).pFunctionality as *mut POWER_GEN;
    //go through list of res extractors, setting them to inactive
	//for (i=0; i < (NUM_POWER_MODULES + 1); i++)
    i = 0 as libc::c_int as UDWORD;
    while i < 4 as libc::c_int as libc::c_uint {
        if !(*psPowerGen).apResExtractors[i as usize].is_null() {
            (*((*(*psPowerGen).apResExtractors[i as usize]).pFunctionality as
                   *mut RES_EXTRACTOR)).active = 0 as libc::c_int;
            let ref mut fresh16 =
                (*((*(*psPowerGen).apResExtractors[i as usize]).pFunctionality
                       as *mut RES_EXTRACTOR)).psPowerGen;
            *fresh16 = 0 as *mut _structure;
            (*psPowerGen).apResExtractors[i as usize] = 0 as *mut _structure
        }
        i = i.wrapping_add(1)
    }
    //may have a power gen with spare capacity
    psCurr = apsStructLists[(*psRelease).player as usize];
    while !psCurr.is_null() {
        if (*(*psCurr).pStructureType).type_0 ==
               REF_POWER_GEN as libc::c_int as libc::c_uint &&
               psCurr != psRelease &&
               (*psCurr).status as libc::c_int == SS_BUILT as libc::c_int {
            checkForResExtractors(psCurr);
        }
        psCurr = (*psCurr).psNext
    };
}
/*this is called whenever a structure has finished building*/
#[no_mangle]
pub unsafe extern "C" fn buildingComplete(mut psBuilding: *mut STRUCTURE) {
    (*psBuilding).currentBuildPts =
        (*(*psBuilding).pStructureType).buildPoints as SWORD;
    (*psBuilding).status = SS_BUILT as libc::c_int as UBYTE;
    match (*(*psBuilding).pStructureType).type_0 {
        3 => {
            checkForResExtractors(psBuilding);
            if selectedPlayer == (*psBuilding).player as libc::c_uint {
                audio_PlayObjStaticTrack(psBuilding as *mut libc::c_void,
                                         ID_SOUND_POWER_HUM as libc::c_int);
            }
        }
        5 => {
            checkForPowerGen(psBuilding);
            /* GJ HACK! - add anim to deriks */
            if (*psBuilding).psCurAnim.is_null() {
                (*psBuilding).psCurAnim =
                    animObj_Add(psBuilding as *mut libc::c_void,
                                5 as libc::c_int, 0 as libc::c_int as UDWORD,
                                0 as libc::c_int as UWORD)
            }
        }
        10 => {
            intCheckResearchButton();
            //this deals with researc facilities that are upgraded whilst mid-research
            releaseResearch(psBuilding);
        }
        1 | 16 | 17 => {
            //this deals with factories that are upgraded whilst mid-production
            releaseProduction(psBuilding);
        }
        21 => { revealAll((*psBuilding).player); }
        _ => { }
    };
}
/*Looks through the players list of structures to see if a HQ exists - will look
through the list of structures at Home Base when on an offWorld mission map*/
#[no_mangle]
pub unsafe extern "C" fn radarCheckForHQ(mut player: UDWORD) -> BOOL {
    //	BOOL		bPlayerHasHQ = FALSE;
	//STRUCTURE	*psStructure;
    return getHQExists(player);
    //if on a mission, need to check mission structs
	/*if (mission.type == LDS_MKEEP OR mission.type == LDS_MCLEAR)
	{
		for (psStructure = mission.apsStructLists[player]; psStructure AND
			!bPlayerHasHQ; psStructure = psStructure->psNext)
		{
			if (psStructure->pStructureType->type == REF_HQ AND psStructure->
				status == SS_BUILT)
			{
				bPlayerHasHQ = TRUE;
			}
		}
	}
	else
	{
		for (psStructure = apsStructLists[player]; psStructure AND
			!bPlayerHasHQ; psStructure = psStructure->psNext)
		{
			if (psStructure->pStructureType->type == REF_HQ AND psStructure->
				status == SS_BUILT)
			{
				bPlayerHasHQ = TRUE;
			}
		}
	}
	return bPlayerHasHQ;*/
}
/*for a given structure, return a pointer to its module stat */
#[no_mangle]
pub unsafe extern "C" fn getModuleStat(mut psStruct: *mut STRUCTURE)
 -> *mut STRUCTURE_STATS {
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    //UDWORD				i;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"getModuleStat: Invalid structure pointer\x00" as *const u8 as
                  *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              7689 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 14],
                                        &[libc::c_char; 14]>(b"getModuleStat\x00")).as_ptr(),
              b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStat = 0 as *mut STRUCTURE_STATS;
    match (*(*psStruct).pStructureType).type_0 {
        3 => {
            /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
			REF_POWER_MODULE);i++)
		{
			//keep looking for the Power Module stat...
		}*/
            psStat =
                &mut *asStructureStats.offset(powerModuleStat as isize) as
                    *mut STRUCTURE_STATS
        }
        1 | 17 => {
            /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
			REF_FACTORY_MODULE);i++)
		{
			//keep looking for the Factory Module stat...
		}*/
            psStat =
                &mut *asStructureStats.offset(factoryModuleStat as isize) as
                    *mut STRUCTURE_STATS
        }
        10 => {
            /*for (i = 0; (i < numStructureStats) && (asStructureStats[i].type !=
			REF_RESEARCH_MODULE);i++)
		{
			//keep looking for the Research Module stat...
		}*/
            psStat =
                &mut *asStructureStats.offset(researchModuleStat as isize) as
                    *mut STRUCTURE_STATS
        }
        _ => { }
    }
    return psStat;
}
//print some info at the top of the screen dependant on the structure
#[no_mangle]
pub unsafe extern "C" fn printStructureInfo(mut psStructure: *mut STRUCTURE) {
    let mut numConnected: UBYTE = 0;
    let mut i: UBYTE = 0;
    let mut psPowerGen: *mut POWER_GEN = 0 as *mut POWER_GEN;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"printStructureInfo: Invalid Structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              7735 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"printStructureInfo\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    match (*(*psStructure).pStructureType).type_0 {
        5 => {
            /*#ifdef DEBUG
		CONPRINTF(ConsoleString,(ConsoleString,"%s - Resource Remaining %d - Unique ID %d",
			getStatName(psStructure->pStructureType), ((RES_EXTRACTOR *)psStructure->
			pFunctionality)->power,psStructure->id));
#else
		CONPRINTF(ConsoleString,(ConsoleString,strresGetString(psStringRes,STR_GAM_RESREM),
			getStatName(psStructure->pStructureType), ((RES_EXTRACTOR *)psStructure->
			pFunctionality)->power));
#endif*/
            sprintf(ConsoleString.as_mut_ptr(),
                    getStatName((*psStructure).pStructureType as
                                    *mut libc::c_void));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        3 => {
            psPowerGen = (*psStructure).pFunctionality as *mut POWER_GEN;
            numConnected = 0 as libc::c_int as UBYTE;
            i = 0 as libc::c_int as UBYTE;
            while (i as libc::c_int) < 4 as libc::c_int {
                if !(*psPowerGen).apResExtractors[i as usize].is_null() {
                    numConnected = numConnected.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
            sprintf(ConsoleString.as_mut_ptr(),
                    strresGetString(psStringRes,
                                    STR_GAM_POWCON as libc::c_int as UDWORD),
                    getStatName((*psStructure).pStructureType as
                                    *mut libc::c_void),
                    numConnected as libc::c_int, 4 as libc::c_int);
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        _ => {
            sprintf(ConsoleString.as_mut_ptr(),
                    getStatName((*psStructure).pStructureType as
                                    *mut libc::c_void),
                    (100 as libc::c_int as
                         libc::c_uint).wrapping_sub((((*psStructure).body as
                                                          libc::c_int *
                                                          100 as libc::c_int)
                                                         as
                                                         libc::c_uint).wrapping_div(structureBody(psStructure))));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
    };
}
/*Checks the template type against the factory type - returns FALSE
if not a good combination!*/
#[no_mangle]
pub unsafe extern "C" fn validTemplateForFactory(mut psTemplate:
                                                     *mut DROID_TEMPLATE,
                                                 mut psFactory:
                                                     *mut STRUCTURE) -> BOOL {
    //not in multiPlayer! - AB 26/5/99
    if bMultiPlayer == 0 {
        //ignore Transporter Droids
        if (*psTemplate).droidType as libc::c_uint ==
               DROID_TRANSPORTER as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
    }
    //check if droid is a cyborg
    if (*psTemplate).droidType as libc::c_uint ==
           DROID_CYBORG as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_SUPER as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint ||
           (*psTemplate).droidType as libc::c_uint ==
               DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint {
        if (*(*psFactory).pStructureType).type_0 !=
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
    } else if (*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION
                                                                   as
                                                                   libc::c_int
                                                                   as usize]
                                             as isize)).propulsionType as
                  libc::c_int == LIFT as libc::c_int {
        if (*(*psFactory).pStructureType).type_0 !=
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
            return 0 as libc::c_int
        }
    }
    //check for VTOL droid
    //check if cyborg factory
    if (*(*psFactory).pStructureType).type_0 ==
           REF_CYBORG_FACTORY as libc::c_int as libc::c_uint {
        //if (psTemplate->droidType != DROID_CYBORG)
        if !((*psTemplate).droidType as libc::c_uint ==
                 DROID_CYBORG as libc::c_int as libc::c_uint ||
                 (*psTemplate).droidType as libc::c_uint ==
                     DROID_CYBORG_SUPER as libc::c_int as libc::c_uint ||
                 (*psTemplate).droidType as libc::c_uint ==
                     DROID_CYBORG_CONSTRUCT as libc::c_int as libc::c_uint ||
                 (*psTemplate).droidType as libc::c_uint ==
                     DROID_CYBORG_REPAIR as libc::c_int as libc::c_uint) {
            return 0 as libc::c_int
        }
    } else if (*(*psFactory).pStructureType).type_0 ==
                  REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        if (*asPropulsionStats.offset((*psTemplate).asParts[COMP_PROPULSION as
                                                                libc::c_int as
                                                                usize] as
                                          isize)).propulsionType as
               libc::c_int != LIFT as libc::c_int {
            return 0 as libc::c_int
        }
    }
    //check if vtol factory
    //got through all the tests...
    return 1 as libc::c_int;
}
/*calculates the damage caused to the resistance levels of structures - returns
TRUE when captured*/
//BOOL electronicDamage(STRUCTURE *psStructure, UDWORD damage, UBYTE attackPlayer)
//electronic damage can be targetted at droids as well as structures now - AB 5/11/98
#[no_mangle]
pub unsafe extern "C" fn electronicDamage(mut psTarget: *mut BASE_OBJECT,
                                          mut damage: UDWORD,
                                          mut attackPlayer: UBYTE) -> BOOL {
    let mut psStructure: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut bCompleted: BOOL = 1 as libc::c_int;
    let mut m: NETMSG =
        NETMSG{size: 0,
               paddedBytes: 0,
               type_0: 0,
               destination: 0,
               body: [0; 8000],};
    let mut pos: iVector = iVector{x: 0, y: 0, z: 0,};
    let mut i: UDWORD = 0;
    if (attackPlayer as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"electronicDamage: invalid player id\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (attackPlayer as libc::c_int) < 8 as libc::c_int {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              7868 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"electronicDamage\x00")).as_ptr(),
              b"attackPlayer < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    //structure electronic damage
    if (*psTarget).type_0 as libc::c_uint ==
           OBJ_STRUCTURE as libc::c_int as libc::c_uint {
        psStructure = psTarget as *mut STRUCTURE;
        bCompleted = 0 as libc::c_int;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"electronicDamage: Invalid Structure pointer\x00" as
                      *const u8 as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7877 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"electronicDamage\x00")).as_ptr(),
                  b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8
                      as *const libc::c_char);
        };
        if (*(*psStructure).pStructureType).resistance !=
               0 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"electronicDamage: invalid structure for EW\x00" as
                      *const u8 as *const libc::c_char);
        };
        if (*(*psStructure).pStructureType).resistance !=
               0 as libc::c_int as libc::c_uint {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7880 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"electronicDamage\x00")).as_ptr(),
                  b"psStructure->pStructureType->resistance != 0\x00" as
                      *const u8 as *const libc::c_char);
        };
        //return FALSE;
        if ((*psStructure).resistance as libc::c_int) < 0 as libc::c_int {
            bCompleted = 1 as libc::c_int
            //if resistance is already less than 0 don't do any more
            //return TRUE;
        } else {
            //store the time it was hit
            (*psStructure).timeLastHit = gameTime;
            (*psStructure).lastHitWeapon =
                WSC_ELECTRONIC as libc::c_int as UDWORD;
            // tell the cluster system it has been attacked
            clustObjectAttacked(psStructure as *mut BASE_OBJECT);
            (*psStructure).resistance =
                ((*psStructure).resistance as
                     libc::c_uint).wrapping_sub(damage) as SWORD;
            if ((*psStructure).resistance as libc::c_int) < 0 as libc::c_int {
                //add a console message for the selected Player
                if (*psStructure).player as libc::c_uint == selectedPlayer {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_ELECDAM as libc::c_int as
                                                UDWORD),
                            getStatName((*psStructure).pStructureType as
                                            *mut libc::c_void));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    //tell the scripts if selectedPlayer has lost a structure
                    eventFireCallbackTrigger(CALL_ELECTRONIC_TAKEOVER as
                                                 libc::c_int as TRIGGER_TYPE);
                }
                //give a reward to the attacking player
    		    //researchReward(psStructure->player, attackPlayer);
	    	    //electronicReward(psStructure, attackPlayer);
		        //return TRUE;
                bCompleted = 1 as libc::c_int;
                //give the structure to the attacking player
                giftSingleStructure(psStructure, attackPlayer,
                                    0 as libc::c_int);
            }
        }
    } else if (*psTarget).type_0 as libc::c_uint ==
                  OBJ_DROID as libc::c_int as libc::c_uint {
        psDroid = psTarget as *mut DROID;
        bCompleted = 0 as libc::c_int;
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"electronicDamage: Invalid Droid pointer\x00" as *const u8
                      as *const libc::c_char);
        };
        if 1 as libc::c_int != 0 {
        } else {
            debug(LOG_ERROR,
                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                      *const libc::c_char,
                  b"structure.c\x00" as *const u8 as *const libc::c_char,
                  7929 as libc::c_int,
                  (*::std::mem::transmute::<&[u8; 17],
                                            &[libc::c_char; 17]>(b"electronicDamage\x00")).as_ptr(),
                  b"PTRVALID(psDroid, sizeof(DROID))\x00" as *const u8 as
                      *const libc::c_char);
        };
        //droid electronic damage
        /* called in droidDamage now - AB 17/06/99
#ifndef PSX
		psDroid->timeLastHit = gameTime;
		psDroid->lastHitWeapon = WSC_ELECTRONIC;
#endif
        */
        if bMultiPlayer != 0 {
            if (*psDroid).droidType as libc::c_uint ==
                   DROID_TRANSPORTER as libc::c_int as libc::c_uint {
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"electronicDamage: Cannot attack a Transporter in multiPlayer\x00"
                              as *const u8 as *const libc::c_char);
                };
                if 0 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 7936 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 17],
                                                    &[libc::c_char; 17]>(b"electronicDamage\x00")).as_ptr(),
                          b"FALSE\x00" as *const u8 as *const libc::c_char);
                };
                return 1 as libc::c_int
            }
        }
        if (*psDroid).resistance as libc::c_int == 0 as libc::c_int {
            //in multiPlayer cannot attack a Transporter with EW
            //need to set the current resistance level since not been previously attacked (by EW)
            (*psDroid).resistance = droidResistance(psDroid)
        }
        if ((*psDroid).resistance as libc::c_int) < 0 as libc::c_int {
            bCompleted = 1 as libc::c_int
        } else {
            // tell the cluster system it has been attacked
            clustObjectAttacked(psDroid as *mut BASE_OBJECT);
            (*psDroid).resistance =
                ((*psDroid).resistance as libc::c_uint).wrapping_sub(damage)
                    as SWORD;
            if (*psDroid).resistance as libc::c_int <= 0 as libc::c_int {
                //add a console message for the selected Player
                if (*psDroid).player as libc::c_uint == selectedPlayer {
                    sprintf(ConsoleString.as_mut_ptr(),
                            strresGetString(psStringRes,
                                            STR_GAM_ELECDAM as libc::c_int as
                                                UDWORD));
                    addConsoleMessage(ConsoleString.as_mut_ptr(),
                                      DEFAULT_JUSTIFY);
                    //tell the scripts if selectedPlayer has lost a droid
                    eventFireCallbackTrigger(CALL_ELECTRONIC_TAKEOVER as
                                                 libc::c_int as TRIGGER_TYPE);
                }
                bCompleted = 1 as libc::c_int;
                //give the droid to the attacking player
                if (*psDroid).visible[selectedPlayer as usize] != 0 {
                    i = 0 as libc::c_int as UDWORD;
                    while i < 5 as libc::c_int as libc::c_uint {
                        pos.x =
                            (*psDroid).x as libc::c_int +
                                (30 as libc::c_int -
                                     rand() % 60 as libc::c_int);
                        pos.z =
                            (*psDroid).y as libc::c_int +
                                (30 as libc::c_int -
                                     rand() % 60 as libc::c_int);
                        pos.y =
                            (*psDroid).z as libc::c_int +
                                rand() % 8 as libc::c_int;
                        effectGiveAuxVar(80 as libc::c_int as UDWORD);
                        addEffect(&mut pos, EFFECT_EXPLOSION,
                                  EXPLOSION_TYPE_FLAMETHROWER,
                                  0 as libc::c_int, 0 as *mut iIMDShape,
                                  0 as libc::c_int);
                        i = i.wrapping_add(1)
                    }
                }
                giftSingleDroid(psDroid, attackPlayer as UDWORD);
                // tell the world!
                if bMultiPlayer != 0 {
                    m.body[0 as libc::c_int as usize] =
                        2 as libc::c_int as libc::c_char;
                    m.body[1 as libc::c_int as usize] =
                        (*psDroid).player as libc::c_char;
                    m.body[2 as libc::c_int as usize] =
                        attackPlayer as libc::c_char;
                    m.type_0 = NET_GIFT as libc::c_int as libc::c_uchar;
                    m.size = 3 as libc::c_int as libc::c_ushort;
                    memcpy(&mut *m.body.as_mut_ptr().offset(m.size as isize)
                               as *mut libc::c_char as *mut libc::c_void,
                           &mut (*psDroid).id as *mut UDWORD as
                               *const libc::c_void,
                           ::std::mem::size_of::<UDWORD>() as libc::c_ulong);
                    m.size =
                        (m.size as
                             libc::c_uint).wrapping_add(::std::mem::size_of::<UDWORD>()
                                                            as libc::c_ulong)
                            as libc::c_ushort as libc::c_ushort;
                    NETbcast(&mut m, 1 as libc::c_int);
                    //send it
                }
                //check to see if droid limit reached, if so recycle.
				// don't check for transporter/mission coz multiplayer only issue.
                if getNumDroids(attackPlayer as
                                    UDWORD).wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) >
                       getMaxDroids(attackPlayer as UDWORD) {
                    recycleDroid(psDroid);
                }
            }
        }
    }
    return bCompleted;
}
/* EW works differently in multiplayer mode compared with single player.*/
#[no_mangle]
pub unsafe extern "C" fn validStructResistance(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    let mut bTarget: BOOL = 0 as libc::c_int;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"invalidStructResistance: invalid structure pointer\x00" as
                  *const u8 as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8029 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 22],
                                        &[libc::c_char; 22]>(b"validStructResistance\x00")).as_ptr(),
              b"PTRVALID(psStruct, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*(*psStruct).pStructureType).resistance !=
           0 as libc::c_int as libc::c_uint {
        /*certain structures will only provide rewards in multiplayer so
        before they can become valid targets their resistance must be at least
        half the base value*/
        if bMultiPlayer != 0 {
            match (*(*psStruct).pStructureType).type_0 {
                10 | 1 | 17 | 16 | 0 | 12 => {
                    if (*psStruct).resistance as libc::c_int >=
                           structureResistance((*psStruct).pStructureType,
                                               (*psStruct).player).wrapping_div(2
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                               as SDWORD {
                        bTarget = 1 as libc::c_int
                    }
                }
                _ => { bTarget = 1 as libc::c_int }
            }
        } else { bTarget = 1 as libc::c_int }
    }
    return bTarget;
}
/*Access functions for the upgradeable stats of a structure*/
#[no_mangle]
pub unsafe extern "C" fn structureBody(mut psStructure: *mut STRUCTURE)
 -> UDWORD {
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS; //, i;
    let mut player: UBYTE = 0;
    psStats = (*psStructure).pStructureType;
    player = (*psStructure).player;
    match (*psStats).type_0 {
        6 | 7 | 8 | 9 => {
            //wall/defence structures
            return (*psStats).bodyPoints.wrapping_add((*psStats).bodyPoints.wrapping_mul(asWallDefenceUpgrade[player
                                                                                                                  as
                                                                                                                  usize].body
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(100
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint))
        }
        _ => {
            //all other structures
            return structureBaseBody(psStructure).wrapping_add(structureBaseBody(psStructure).wrapping_mul(asStructureUpgrade[player
                                                                                                                                  as
                                                                                                                                  usize].body
                                                                                                               as
                                                                                                               libc::c_uint).wrapping_div(100
                                                                                                                                              as
                                                                                                                                              libc::c_int
                                                                                                                                              as
                                                                                                                                              libc::c_uint))
        }
    };
}
/*this returns the Base Body points of a structure - regardless of upgrade*/
#[no_mangle]
pub unsafe extern "C" fn structureBaseBody(mut psStructure: *mut STRUCTURE)
 -> UDWORD {
    let mut psStats: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut player: UBYTE = 0;
    let mut capacity: UBYTE = 0;
    let mut body: UDWORD = 0;
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"structureBaseBody: invalid structure pointer\x00" as *const u8
                  as *const libc::c_char);
    };
    if 1 as libc::c_int != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8109 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"structureBaseBody\x00")).as_ptr(),
              b"PTRVALID(psStructure, sizeof(STRUCTURE))\x00" as *const u8 as
                  *const libc::c_char);
    };
    psStats = (*psStructure).pStructureType;
    player = (*psStructure).player;
    match (*psStats).type_0 {
        1 | 17 => {
            //modules may be attached
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"structureBaseBody: invalid structure functionality pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      8120 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"structureBaseBody\x00")).as_ptr(),
                      b"PTRVALID(psStructure->pFunctionality, sizeof(FUNCTIONALITY))\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*((*psStructure).pFunctionality as *mut FACTORY)).capacity as
                   libc::c_int > 0 as libc::c_int {
                body = 0 as libc::c_int as UDWORD;
                capacity =
                    (*((*psStructure).pFunctionality as
                           *mut FACTORY)).capacity;
                while capacity != 0 {
                    body =
                        (body as
                             libc::c_uint).wrapping_add((*asStructureStats.offset(factoryModuleStat
                                                                                      as
                                                                                      isize)).bodyPoints)
                            as UDWORD as UDWORD;
                    capacity = capacity.wrapping_sub(1)
                }
                //add on the default for the factory
                body =
                    (body as libc::c_uint).wrapping_add((*psStats).bodyPoints)
                        as UDWORD as UDWORD;
                return body
            } else {
                //no modules
                return (*psStats).bodyPoints
            }
        }
        10 => {
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"structureBaseBody: invalid structure functionality pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      8142 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"structureBaseBody\x00")).as_ptr(),
                      b"PTRVALID(psStructure->pFunctionality, sizeof(FUNCTIONALITY))\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*((*psStructure).pFunctionality as
                      *mut RESEARCH_FACILITY)).capacity >
                   0 as libc::c_int as libc::c_uint {
                body = 0 as libc::c_int as UDWORD;
                body =
                    (*asStructureStats.offset(researchModuleStat as
                                                  isize)).bodyPoints;
                //add on the default for the factory
                body =
                    (body as libc::c_uint).wrapping_add((*psStats).bodyPoints)
                        as UDWORD as UDWORD;
                return body
            } else {
                //no modules
                return (*psStats).bodyPoints
            }
        }
        3 => {
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"structureBaseBody: invalid structure functionality pointer\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 1 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      8159 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 18],
                                                &[libc::c_char; 18]>(b"structureBaseBody\x00")).as_ptr(),
                      b"PTRVALID(psStructure->pFunctionality, sizeof(FUNCTIONALITY))\x00"
                          as *const u8 as *const libc::c_char);
            };
            if (*((*psStructure).pFunctionality as *mut POWER_GEN)).capacity >
                   0 as libc::c_int as libc::c_uint {
                body = 0 as libc::c_int as UDWORD;
                body =
                    (*asStructureStats.offset(powerModuleStat as
                                                  isize)).bodyPoints;
                //add on the default for the factory
                body =
                    (body as libc::c_uint).wrapping_add((*psStats).bodyPoints)
                        as UDWORD as UDWORD;
                return body
            } else {
                //no modules
                return (*psStats).bodyPoints
            }
        }
        _ => {
            //all other structures
            return (*psStats).bodyPoints
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureArmour(mut psStats: *mut STRUCTURE_STATS,
                                         mut player: UBYTE) -> UDWORD {
    match (*psStats).type_0 {
        6 | 7 | 8 | 9 => {
            return (*psStats).armourValue.wrapping_add((*psStats).armourValue.wrapping_mul(asWallDefenceUpgrade[player
                                                                                                                    as
                                                                                                                    usize].armour
                                                                                               as
                                                                                               libc::c_uint).wrapping_div(100
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint))
        }
        _ => {
            return (*psStats).armourValue.wrapping_add((*psStats).armourValue.wrapping_mul(asStructureUpgrade[player
                                                                                                                  as
                                                                                                                  usize].armour
                                                                                               as
                                                                                               libc::c_uint).wrapping_div(100
                                                                                                                              as
                                                                                                                              libc::c_int
                                                                                                                              as
                                                                                                                              libc::c_uint))
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn structureResistance(mut psStats:
                                                 *mut STRUCTURE_STATS,
                                             mut player: UBYTE) -> UDWORD {
    match (*psStats).type_0 {
        7 | 8 | 9 => {
            //defense type can now be upgradede
	//case REF_DEFENSE:
            //not upgradeable
            return (*psStats).resistance
        }
        _ => {
            return (*psStats).resistance.wrapping_add((*psStats).resistance.wrapping_mul(asStructureUpgrade[player
                                                                                                                as
                                                                                                                usize].resistance
                                                                                             as
                                                                                             libc::c_uint).wrapping_div(100
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint))
        }
    };
}
//access function for sensor stats
#[no_mangle]
pub unsafe extern "C" fn structureSensorRange(mut psStats:
                                                  *mut STRUCTURE_STATS)
 -> UDWORD {
    if !(*psStats).pSensor.is_null() {
        return (*(*psStats).pSensor).range
    } else { return 0 as libc::c_int as UDWORD };
}
#[no_mangle]
pub unsafe extern "C" fn structureSensorPower(mut psStats:
                                                  *mut STRUCTURE_STATS)
 -> UDWORD {
    if !(*psStats).pSensor.is_null() {
        return (*(*psStats).pSensor).power
    } else { return 0 as libc::c_int as UDWORD };
}
/*gives the attacking player a reward based on the type of structure that has
been attacked*/
unsafe extern "C" fn electronicReward(mut psStructure: *mut STRUCTURE,
                                      mut attackPlayer: UBYTE) -> BOOL {
    let mut bRewarded: BOOL = 0 as libc::c_int;
    match (*(*psStructure).pStructureType).type_0 {
        10 => {
            researchReward((*psStructure).player, attackPlayer);
            bRewarded = 1 as libc::c_int
        }
        1 | 17 | 16 => {
            factoryReward((*psStructure).player, attackPlayer);
            bRewarded = 1 as libc::c_int
        }
        0 => {
            hqReward((*psStructure).player, attackPlayer);
            if attackPlayer as libc::c_uint == selectedPlayer {
                addConsoleMessage(strresGetString(psStringRes,
                                                  STR_GAM_REWELEC as
                                                      libc::c_int as UDWORD),
                                  DEFAULT_JUSTIFY);
            }
            bRewarded = 1 as libc::c_int
        }
        12 => {
            repairFacilityReward((*psStructure).player, attackPlayer);
            bRewarded = 1 as libc::c_int
        }
        _ => {
            //ASSERT( FALSE, "electronic Reward for a building not catered for - CANCEL will continue the game!" );
            bRewarded = 0 as libc::c_int
        }
    }
    return bRewarded;
}
/*find the 'best' prop/body/weapon component the losing player has and
'give' it to the reward player*/
unsafe extern "C" fn factoryReward(mut losingPlayer: UBYTE,
                                   mut rewardPlayer: UBYTE) {
    let mut inc: UDWORD = 0;
    let mut comp: UDWORD = 0 as libc::c_int as UDWORD;
    //search through the propulsions first
    inc = 0 as libc::c_int as UDWORD;
    while inc < numPropulsionStats {
        if *apCompLists[losingPlayer as
                            usize][COMP_PROPULSION as libc::c_int as
                                       usize].offset(inc as isize) as
               libc::c_int == 0x1 as libc::c_int &&
               *apCompLists[rewardPlayer as
                                usize][COMP_PROPULSION as libc::c_int as
                                           usize].offset(inc as isize) as
                   libc::c_int != 0x1 as libc::c_int {
            if (*asPropulsionStats.offset(inc as isize)).buildPower >
                   (*asPropulsionStats.offset(comp as isize)).buildPower {
                comp = inc
            }
        }
        inc = inc.wrapping_add(1)
    }
    if comp != 0 as libc::c_int as libc::c_uint {
        *apCompLists[rewardPlayer as
                         usize][COMP_PROPULSION as libc::c_int as
                                    usize].offset(comp as isize) =
            0x1 as libc::c_int as UBYTE;
        if rewardPlayer as libc::c_uint == selectedPlayer {
            //addConsoleMessage(strresGetString(psStringRes,STR_GAM_REWPROP), DEFAULT_JUSTIFY);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"%s :- %s\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_GAM_REWPROP as libc::c_int as UDWORD),
                    getName((*asPropulsionStats.offset(comp as
                                                           isize)).pName));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        return
    }
    //haven't found a propulsion - look for a body
    inc = 0 as libc::c_int as UDWORD;
    while inc < numBodyStats {
        if *apCompLists[losingPlayer as
                            usize][COMP_BODY as libc::c_int as
                                       usize].offset(inc as isize) as
               libc::c_int == 0x1 as libc::c_int &&
               *apCompLists[rewardPlayer as
                                usize][COMP_BODY as libc::c_int as
                                           usize].offset(inc as isize) as
                   libc::c_int != 0x1 as libc::c_int {
            if (*asBodyStats.offset(inc as isize)).buildPower >
                   (*asBodyStats.offset(comp as isize)).buildPower {
                comp = inc
            }
        }
        inc = inc.wrapping_add(1)
    }
    if comp != 0 as libc::c_int as libc::c_uint {
        *apCompLists[rewardPlayer as
                         usize][COMP_BODY as libc::c_int as
                                    usize].offset(comp as isize) =
            0x1 as libc::c_int as UBYTE;
        if rewardPlayer as libc::c_uint == selectedPlayer {
            //addConsoleMessage(strresGetString(psStringRes,STR_GAM_REWBODY), DEFAULT_JUSTIFY);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"%s :- %s\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_GAM_REWBODY as libc::c_int as UDWORD),
                    getName((*asBodyStats.offset(comp as isize)).pName));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        return
    }
    //haven't found a body - look for a weapon
    inc = 0 as libc::c_int as UDWORD;
    while inc < numWeaponStats {
        if *apCompLists[losingPlayer as
                            usize][COMP_WEAPON as libc::c_int as
                                       usize].offset(inc as isize) as
               libc::c_int == 0x1 as libc::c_int &&
               *apCompLists[rewardPlayer as
                                usize][COMP_WEAPON as libc::c_int as
                                           usize].offset(inc as isize) as
                   libc::c_int != 0x1 as libc::c_int {
            if (*asWeaponStats.offset(inc as isize)).buildPower >
                   (*asWeaponStats.offset(comp as isize)).buildPower {
                comp = inc
            }
        }
        inc = inc.wrapping_add(1)
    }
    if comp != 0 as libc::c_int as libc::c_uint {
        *apCompLists[rewardPlayer as
                         usize][COMP_WEAPON as libc::c_int as
                                    usize].offset(comp as isize) =
            0x1 as libc::c_int as UBYTE;
        if rewardPlayer as libc::c_uint == selectedPlayer {
            //addConsoleMessage(strresGetString(psStringRes,STR_GAM_REWWEAP), DEFAULT_JUSTIFY);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"%s :- %s\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_GAM_REWWEAP as libc::c_int as UDWORD),
                    getName((*asWeaponStats.offset(comp as isize)).pName));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        return
    }
    //losing Player hasn't got anything better so don't gain anything!
    if rewardPlayer as libc::c_uint == selectedPlayer {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_REWNOWT as libc::c_int as
                                              UDWORD), DEFAULT_JUSTIFY);
    };
}
/*find the 'best' repair component the losing player has and
'give' it to the reward player*/
unsafe extern "C" fn repairFacilityReward(mut losingPlayer: UBYTE,
                                          mut rewardPlayer: UBYTE) {
    let mut inc: UDWORD = 0;
    let mut comp: UDWORD = 0 as libc::c_int as UDWORD;
    //search through the repair stats
    inc = 0 as libc::c_int as UDWORD;
    while inc < numRepairStats {
        if *apCompLists[losingPlayer as
                            usize][COMP_REPAIRUNIT as libc::c_int as
                                       usize].offset(inc as isize) as
               libc::c_int == 0x1 as libc::c_int &&
               *apCompLists[rewardPlayer as
                                usize][COMP_REPAIRUNIT as libc::c_int as
                                           usize].offset(inc as isize) as
                   libc::c_int != 0x1 as libc::c_int {
            if (*asRepairStats.offset(inc as isize)).buildPower >
                   (*asRepairStats.offset(comp as isize)).buildPower {
                comp = inc
            }
        }
        inc = inc.wrapping_add(1)
    }
    if comp != 0 as libc::c_int as libc::c_uint {
        *apCompLists[rewardPlayer as
                         usize][COMP_REPAIRUNIT as libc::c_int as
                                    usize].offset(comp as isize) =
            0x1 as libc::c_int as UBYTE;
        if rewardPlayer as libc::c_uint == selectedPlayer {
            //addConsoleMessage(strresGetString(psStringRes,STR_GAM_REWREPA), DEFAULT_JUSTIFY);
            sprintf(ConsoleString.as_mut_ptr(),
                    b"%s :- %s\x00" as *const u8 as *const libc::c_char,
                    strresGetString(psStringRes,
                                    STR_GAM_REWREPA as libc::c_int as UDWORD),
                    getName((*asRepairStats.offset(comp as isize)).pName));
            addConsoleMessage(ConsoleString.as_mut_ptr(), DEFAULT_JUSTIFY);
        }
        return
    }
    if rewardPlayer as libc::c_uint == selectedPlayer {
        addConsoleMessage(strresGetString(psStringRes,
                                          STR_GAM_REWREPN as libc::c_int as
                                              UDWORD), DEFAULT_JUSTIFY);
    };
}
/*makes the losing players tiles/structures/features visible to the reward player*/
#[no_mangle]
pub unsafe extern "C" fn hqReward(mut losingPlayer: UBYTE,
                                  mut rewardPlayer: UBYTE) {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFeat: *mut FEATURE = 0 as *mut FEATURE;
    let mut psDroid: *mut DROID = 0 as *mut DROID;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut i: UDWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //tiles
    x = 0 as libc::c_int as UDWORD;
    while x < mapWidth {
        y = 0 as libc::c_int as UDWORD;
        while y < mapHeight {
            psTile = mapTile(x, y);
            if (*psTile).tileVisBits as libc::c_int &
                   (1 as libc::c_int) << losingPlayer as libc::c_int != 0 {
                (*psTile).tileVisBits =
                    ((*psTile).tileVisBits as libc::c_int |
                         (1 as libc::c_int) << rewardPlayer as libc::c_int) as
                        UBYTE;
                if getRevealStatus() != 0 {
                    if rewardPlayer as libc::c_uint == selectedPlayer {
                        avInformOfChange(x as SDWORD, y as SDWORD);
                    }
                }
            }
            y = y.wrapping_add(1)
        }
        x = x.wrapping_add(1)
    }
    //struct
    i = 0 as libc::c_int as UDWORD;
    while i < 8 as libc::c_int as libc::c_uint {
        psStruct = apsStructLists[i as usize];
        while !psStruct.is_null() {
            if (*psStruct).visible[losingPlayer as usize] as libc::c_int != 0
                   && (*psStruct).died == 0 {
                (*psStruct).visible[rewardPlayer as usize] =
                    (*psStruct).visible[losingPlayer as usize];
                if rewardPlayer as libc::c_uint == selectedPlayer {
                    setStructTileDraw(psStruct);
                }
            }
            psStruct = (*psStruct).psNext
        }
        //feature
        psFeat = apsFeatureLists[i as usize];
        while !psFeat.is_null() {
            if (*psFeat).visible[losingPlayer as usize] != 0 {
                (*psFeat).visible[rewardPlayer as usize] =
                    (*psFeat).visible[losingPlayer as usize];
                if rewardPlayer as libc::c_uint == selectedPlayer {
                    setFeatTileDraw(psFeat);
                }
            }
            psFeat = (*psFeat).psNext
        }
        //droids.
        psDroid = apsDroidLists[i as usize];
        while !psDroid.is_null() {
            if (*psDroid).visible[losingPlayer as usize] as libc::c_int != 0
                   ||
                   (*psDroid).player as libc::c_int ==
                       losingPlayer as libc::c_int {
                (*psDroid).visible[rewardPlayer as usize] =
                    0xff as libc::c_int as UBYTE
            }
            psDroid = (*psDroid).psNext
        }
        i = i.wrapping_add(1)
    };
}
// Return TRUE if structure is a factory of any type.
//
#[no_mangle]
pub unsafe extern "C" fn StructIsFactory(mut Struct: *mut STRUCTURE) -> BOOL {
    if (*(*Struct).pStructureType).type_0 ==
           REF_FACTORY as libc::c_int as libc::c_uint ||
           (*(*Struct).pStructureType).type_0 ==
               REF_CYBORG_FACTORY as libc::c_int as libc::c_uint ||
           (*(*Struct).pStructureType).type_0 ==
               REF_VTOL_FACTORY as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Return true if flag is a delivery point for a factory.
//
#[no_mangle]
pub unsafe extern "C" fn FlagIsFactory(mut psCurrFlag: *mut FLAG_POSITION)
 -> BOOL {
    if (*psCurrFlag).factoryType as libc::c_int == 0 as libc::c_int ||
           (*psCurrFlag).factoryType as libc::c_int == 1 as libc::c_int ||
           (*psCurrFlag).factoryType as libc::c_int == 2 as libc::c_int {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// Find a structure's delivery point , only if it's a factory.
// Returns NULL if not found or the structure isn't a factory.
//
#[no_mangle]
pub unsafe extern "C" fn FindFactoryDelivery(mut Struct: *mut STRUCTURE)
 -> *mut FLAG_POSITION {
    let mut psCurrFlag: *mut FLAG_POSITION = 0 as *mut FLAG_POSITION;
    if StructIsFactory(Struct) != 0 {
        // Find the factories delivery point.
        psCurrFlag = apsFlagPosLists[selectedPlayer as usize];
        while !psCurrFlag.is_null() {
            if FlagIsFactory(psCurrFlag) != 0 &&
                   (*(*((*Struct).pFunctionality as
                            *mut FACTORY)).psAssemblyPoint).factoryInc as
                       libc::c_int == (*psCurrFlag).factoryInc as libc::c_int
                   &&
                   (*(*((*Struct).pFunctionality as
                            *mut FACTORY)).psAssemblyPoint).factoryType as
                       libc::c_int == (*psCurrFlag).factoryType as libc::c_int
               {
                return psCurrFlag
            }
            psCurrFlag = (*psCurrFlag).psNext
        }
    }
    return 0 as *mut FLAG_POSITION;
}
//Find the factory associated with the delivery point - returns NULL if none exist
#[no_mangle]
pub unsafe extern "C" fn findDeliveryFactory(mut psDelPoint:
                                                 *mut FLAG_POSITION)
 -> *mut STRUCTURE {
    let mut psCurr: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    psCurr = apsStructLists[(*psDelPoint).player as usize];
    while !psCurr.is_null() {
        if StructIsFactory(psCurr) != 0 {
            psFactory = (*psCurr).pFunctionality as *mut FACTORY;
            if (*(*psFactory).psAssemblyPoint).factoryInc as libc::c_int ==
                   (*psDelPoint).factoryInc as libc::c_int &&
                   (*(*psFactory).psAssemblyPoint).factoryType as libc::c_int
                       == (*psDelPoint).factoryType as libc::c_int {
                return psCurr
            }
        } else if (*(*psCurr).pStructureType).type_0 ==
                      REF_REPAIR_FACILITY as libc::c_int as libc::c_uint {
            psRepair = (*psCurr).pFunctionality as *mut REPAIR_FACILITY;
            if (*psRepair).psDeliveryPoint == psDelPoint { return psCurr }
        }
        psCurr = (*psCurr).psNext
    }
    return 0 as *mut STRUCTURE;
}
/*cancels the production run for the factory and returns any power that was
accrued but not used*/
#[no_mangle]
pub unsafe extern "C" fn cancelProduction(mut psBuilding: *mut STRUCTURE) {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"cancelProduction: structure not a factory\x00" as *const u8 as
                  *const libc::c_char);
    };
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8576 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 17],
                                        &[libc::c_char; 17]>(b"cancelProduction\x00")).as_ptr(),
              b"StructIsFactory(psBuilding)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psBuilding).pFunctionality as *mut FACTORY;
    //check its the correct factory
    if (*psBuilding).player as libc::c_int == productionPlayer as libc::c_int
           && !(*psFactory).psSubject.is_null() {
        //clear the production run for this factory
        memset(asProductionRun[(*(*psFactory).psAssemblyPoint).factoryType as
                                   usize][(*(*psFactory).psAssemblyPoint).factoryInc
                                              as usize].as_mut_ptr() as
                   *mut libc::c_void, 0 as libc::c_int,
               (::std::mem::size_of::<PRODUCTION_RUN>() as
                    libc::c_ulong).wrapping_mul(20 as libc::c_int as
                                                    libc::c_uint));
        //return any accrued power
        if (*psFactory).powerAccrued != 0 {
            addPower((*psBuilding).player as UDWORD,
                     (*psFactory).powerAccrued);
        }
        //clear the factories subject and quantity
        (*psFactory).psSubject = 0 as *mut BASE_STATS;
        (*psFactory).quantity = 0 as libc::c_int as UBYTE;
        //tell the interface
        intManufactureFinished(psBuilding);
    };
}
/*set a factory's production run to hold*/
#[no_mangle]
pub unsafe extern "C" fn holdProduction(mut psBuilding: *mut STRUCTURE) {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"holdProduction: structure not a factory\x00" as *const u8 as
                  *const libc::c_char);
    };
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8607 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 15],
                                        &[libc::c_char; 15]>(b"holdProduction\x00")).as_ptr(),
              b"StructIsFactory(psBuilding)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psBuilding).pFunctionality as *mut FACTORY;
    if !(*psFactory).psSubject.is_null() {
        //set the time the factory was put on hold
        (*psFactory).timeStartHold = gameTime;
        //play audio to indicate on hold
        if (*psBuilding).player as libc::c_uint == selectedPlayer {
            audio_PlayTrack(ID_SOUND_WINDOWCLOSE as libc::c_int);
        }
    };
}
/*release a factory's production run from hold*/
#[no_mangle]
pub unsafe extern "C" fn releaseProduction(mut psBuilding: *mut STRUCTURE) {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"releaseProduction: structure not a factory\x00" as *const u8
                  as *const libc::c_char);
    };
    if StructIsFactory(psBuilding) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8630 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"releaseProduction\x00")).as_ptr(),
              b"StructIsFactory(psBuilding)\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psBuilding).pFunctionality as *mut FACTORY;
    if !(*psFactory).psSubject.is_null() && (*psFactory).timeStartHold != 0 {
        //adjust the start time for the current subject
        if (*psFactory).timeStarted != 0 as libc::c_int as libc::c_uint {
            (*psFactory).timeStarted =
                ((*psFactory).timeStarted as
                     libc::c_uint).wrapping_add(gameTime.wrapping_sub((*psFactory).timeStartHold))
                    as UDWORD as UDWORD
        }
        (*psFactory).timeStartHold = 0 as libc::c_int as UDWORD
    };
}
/*this is called when a factory produces a droid. The Template returned is the next
one to build - if any*/
#[no_mangle]
pub unsafe extern "C" fn factoryProdUpdate(mut psStructure: *mut STRUCTURE,
                                           mut psTemplate:
                                               *mut DROID_TEMPLATE)
 -> *mut DROID_TEMPLATE {
    let mut inc: UDWORD = 0;
    let mut factoryType: UDWORD = 0;
    let mut factoryInc: UDWORD = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"factoryProdUpdate: called for incorrect player\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8653 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"factoryProdUpdate\x00")).as_ptr(),
              b"psStructure->player == productionPlayer\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psStructure).pFunctionality as *mut FACTORY;
    factoryType = (*(*psFactory).psAssemblyPoint).factoryType as UDWORD;
    factoryInc = (*(*psFactory).psAssemblyPoint).factoryInc as UDWORD;
    if !psTemplate.is_null() {
        //find the entry in the array for this template
        inc = 0 as libc::c_int as UDWORD;
        while inc < 20 as libc::c_int as libc::c_uint {
            if asProductionRun[factoryType as
                                   usize][factoryInc as
                                              usize][inc as usize].psTemplate
                   == psTemplate {
                asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as usize].built =
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].built.wrapping_add(1);
                if (asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as usize].built
                        as libc::c_int) <
                       asProductionRun[factoryType as
                                           usize][factoryInc as
                                                      usize][inc as
                                                                 usize].quantity
                           as libc::c_int {
                    return psTemplate
                }
                break ;
            } else { inc = inc.wrapping_add(1) }
        }
    }
    //find the next template to build - this just looks for the first uncompleted run
    inc = 0 as libc::c_int as UDWORD;
    while inc < 20 as libc::c_int as libc::c_uint {
        if (asProductionRun[factoryType as
                                usize][factoryInc as
                                           usize][inc as usize].built as
                libc::c_int) <
               asProductionRun[factoryType as
                                   usize][factoryInc as
                                              usize][inc as usize].quantity as
                   libc::c_int {
            return asProductionRun[factoryType as
                                       usize][factoryInc as
                                                  usize][inc as
                                                             usize].psTemplate
        }
        inc = inc.wrapping_add(1)
    }
    /*If you've got here there's nothing left to build unless factory is
      on loop production*/
    if (*psFactory).quantity != 0 {
        //reduce the loop count if not infinite
        if (*psFactory).quantity as libc::c_int != 9 as libc::c_int {
            (*psFactory).quantity = (*psFactory).quantity.wrapping_sub(1)
        }
        //need to reset the quantity built for each entry in the production list
        inc = 0 as libc::c_int as UDWORD;
        while inc < 20 as libc::c_int as libc::c_uint {
            if !asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as
                                                          usize].psTemplate.is_null()
               {
                asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as usize].built =
                    0 as libc::c_int as UBYTE
            }
            inc = inc.wrapping_add(1)
        }
        //get the first to build again
        return factoryProdUpdate(psStructure, 0 as *mut DROID_TEMPLATE)
    }
    //if got to here then nothing left to produce so clear the array
    memset(asProductionRun[factoryType as
                               usize][factoryInc as usize].as_mut_ptr() as
               *mut libc::c_void, 0 as libc::c_int,
           (::std::mem::size_of::<PRODUCTION_RUN>() as
                libc::c_ulong).wrapping_mul(20 as libc::c_int as
                                                libc::c_uint));
    return 0 as *mut DROID_TEMPLATE;
}
//adjust the production run for this template type
#[no_mangle]
pub unsafe extern "C" fn factoryProdAdjust(mut psStructure: *mut STRUCTURE,
                                           mut psTemplate:
                                               *mut DROID_TEMPLATE,
                                           mut add: BOOL) {
    let mut spare: SDWORD = -(1 as libc::c_int);
    let mut inc: UDWORD = 0;
    let mut factoryType: UDWORD = 0;
    let mut factoryInc: UDWORD = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut bAssigned: BOOL = 0 as libc::c_int;
    let mut bCheckForCancel: BOOL = 0 as libc::c_int;
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"factoryProdAdjust: called for incorrect player\x00" as
                  *const u8 as *const libc::c_char);
    };
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8721 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"factoryProdAdjust\x00")).as_ptr(),
              b"psStructure->player == productionPlayer\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psStructure).pFunctionality as *mut FACTORY;
    factoryType = (*(*psFactory).psAssemblyPoint).factoryType as UDWORD;
    factoryInc = (*(*psFactory).psAssemblyPoint).factoryInc as UDWORD;
    //see if the template is already in the list
    inc = 0 as libc::c_int as UDWORD;
    while inc < 20 as libc::c_int as libc::c_uint {
        if asProductionRun[factoryType as
                               usize][factoryInc as
                                          usize][inc as usize].psTemplate ==
               psTemplate {
            //adjust the prod run
            if add != 0 {
                asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as usize].quantity =
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].quantity.wrapping_add(1);
                if asProductionRun[factoryType as
                                       usize][factoryInc as
                                                  usize][inc as
                                                             usize].quantity
                       as libc::c_int > 9 as libc::c_int {
                    //#ifdef PSX	// Don't wrap around, just max out.
//					asProductionRun[factoryType][factoryInc][inc].quantity = MAX_IN_RUN;
//#else
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].quantity
                        = 0 as libc::c_int as UBYTE;
                    //#endif
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].psTemplate
                        = 0 as *mut _droid_template;
                    bCheckForCancel = 1 as libc::c_int;
                    if (*psFactory).psSubject == psTemplate as *mut BASE_STATS
                       {
                        addPower((*psStructure).player as UDWORD,
                                 (*psFactory).powerAccrued);
                        //initialise the template
                        //add power back if we were working on this one
                        //set the factory's power accrued back to zero
                        (*psFactory).powerAccrued = 0 as libc::c_int as UDWORD
                    }
                }
            } else if asProductionRun[factoryType as
                                          usize][factoryInc as
                                                     usize][inc as
                                                                usize].quantity
                          as libc::c_int == 0 as libc::c_int {
                //#ifdef PSX	// Don't wrap around
//#else
                asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as usize].quantity =
                    9 as libc::c_int as UBYTE
                //#endif
            } else {
                asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as usize].quantity =
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].quantity.wrapping_sub(1);
                //add power back if we were working on this one
                if (*psFactory).psSubject == psTemplate as *mut BASE_STATS {
                    addPower((*psStructure).player as UDWORD,
                             (*psFactory).powerAccrued);
                    //set the factory's power accrued back to zero
                    (*psFactory).powerAccrued = 0 as libc::c_int as UDWORD
                }
                if asProductionRun[factoryType as
                                       usize][factoryInc as
                                                  usize][inc as
                                                             usize].quantity
                       as libc::c_int == 0 as libc::c_int {
                    //#ifdef PSX	// Don't wrap around
//#else
						//initialise the template
                    asProductionRun[factoryType as
                                        usize][factoryInc as
                                                   usize][inc as
                                                              usize].psTemplate
                        = 0 as *mut _droid_template;
                    bCheckForCancel = 1 as libc::c_int
                    //#endif
                }
            }
            bAssigned = 1 as libc::c_int;
            break ;
        } else {
            //check to see if any empty slots
            if spare == -(1 as libc::c_int) &&
                   asProductionRun[factoryType as
                                       usize][factoryInc as
                                                  usize][inc as
                                                             usize].quantity
                       as libc::c_int == 0 as libc::c_int {
                spare = inc as SDWORD
            }
            inc = inc.wrapping_add(1)
        }
    }
    if bAssigned == 0 && spare != -(1 as libc::c_int) {
        //start off a new template
        asProductionRun[factoryType as
                            usize][factoryInc as
                                       usize][spare as usize].psTemplate =
            psTemplate;
        if add != 0 {
            asProductionRun[factoryType as
                                usize][factoryInc as
                                           usize][spare as usize].quantity =
                1 as libc::c_int as UBYTE
        } else {
            //#ifdef PSX	// Don't wrap around.
//#else
			//wrap around to max value
            asProductionRun[factoryType as
                                usize][factoryInc as
                                           usize][spare as usize].quantity =
                9 as libc::c_int as UBYTE
            //#endif
        }
    }
    //if nothing is allocated then the current factory may have been cancelled
    if bCheckForCancel != 0 {
        inc = 0 as libc::c_int as UDWORD;
        while inc < 20 as libc::c_int as libc::c_uint {
            if !asProductionRun[factoryType as
                                    usize][factoryInc as
                                               usize][inc as
                                                          usize].psTemplate.is_null()
               {
                break ;
            }
            inc = inc.wrapping_add(1)
        }
        if inc == 20 as libc::c_int as libc::c_uint {
            //must have cancelled eveything - so tell the struct
            (*psFactory).quantity = 0 as libc::c_int as UBYTE
            //all the power is returned when the quantity is decreased now (see above addPower())- AB 8/2/99
			//if started the subject then return half power, else return all accrued
			/*if (psFactory->timeStarted == ACTION_START_TIME)
			{
				addPower(psStructure->player, psFactory->powerAccrued);
			}
			else
			{
				addPower(psStructure->player, ((DROID_TEMPLATE *)psFactory->
					psSubject)->powerPoints / 2);
			}*/
			//psFactory->psSubject = NULL;
			//intManufactureFinished(psStructure);
        }
    };
}
//returns the quantity of a specific template in the production list
#[no_mangle]
pub unsafe extern "C" fn getProductionQuantity(mut psStructure:
                                                   *mut STRUCTURE,
                                               mut psTemplate:
                                                   *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut inc: UDWORD = 0;
    let mut factoryType: UDWORD = 0;
    let mut factoryInc: UDWORD = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    //ASSERT( psStructure->player == selectedPlayer,
	//	"getProductionQuantity: should only be called for selectedPlayer" );
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
        psFactory = (*psStructure).pFunctionality as *mut FACTORY;
        factoryType = (*(*psFactory).psAssemblyPoint).factoryType as UDWORD;
        factoryInc = (*(*psFactory).psAssemblyPoint).factoryInc as UDWORD;
        //see if the template is in the list
        inc = 0 as libc::c_int as UDWORD;
        while inc < 20 as libc::c_int as libc::c_uint {
            if asProductionRun[factoryType as
                                   usize][factoryInc as
                                              usize][inc as usize].psTemplate
                   == psTemplate {
                return asProductionRun[factoryType as
                                           usize][factoryInc as
                                                      usize][inc as
                                                                 usize].quantity
                           as UDWORD
            }
            inc = inc.wrapping_add(1)
        }
    }
    //not in the list so none being produced
    return 0 as libc::c_int as UDWORD;
}
/*returns the quantity of a specific template in the production list that
have already been built*/
#[no_mangle]
pub unsafe extern "C" fn getProductionBuilt(mut psStructure: *mut STRUCTURE,
                                            mut psTemplate:
                                                *mut DROID_TEMPLATE)
 -> UDWORD {
    let mut inc: UDWORD = 0;
    let mut factoryType: UDWORD = 0;
    let mut factoryInc: UDWORD = 0;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    //ASSERT( psStructure->player == selectedPlayer,
	//	"getProductionBuilt: should only be called for selectedPlayer" );
    if (*psStructure).player as libc::c_int == productionPlayer as libc::c_int
       {
        psFactory = (*psStructure).pFunctionality as *mut FACTORY;
        factoryType = (*(*psFactory).psAssemblyPoint).factoryType as UDWORD;
        factoryInc = (*(*psFactory).psAssemblyPoint).factoryInc as UDWORD;
        //see if the template is in the list
        inc = 0 as libc::c_int as UDWORD;
        while inc < 20 as libc::c_int as libc::c_uint {
            if asProductionRun[factoryType as
                                   usize][factoryInc as
                                              usize][inc as usize].psTemplate
                   == psTemplate {
                return asProductionRun[factoryType as
                                           usize][factoryInc as
                                                      usize][inc as
                                                                 usize].built
                           as UDWORD
            }
            inc = inc.wrapping_add(1)
        }
    }
    //not in the list so none being produced
    return 0 as libc::c_int as UDWORD;
}
/*looks through a players production list to see how many command droids
are being built*/
#[no_mangle]
pub unsafe extern "C" fn checkProductionForCommand(mut player: UBYTE)
 -> UBYTE {
    let mut factoryInc: UBYTE = 0;
    let mut inc: UBYTE = 0;
    let mut factoryType: UBYTE = 0;
    let mut mask: UBYTE = 1 as libc::c_int as UBYTE;
    let mut quantity: UBYTE = 0 as libc::c_int as UBYTE;
    //ASSERT( player == selectedPlayer,
	//	"checkProductionForCommand: should only be called for selectedPlayer" );
    if player as libc::c_int == productionPlayer as libc::c_int {
        //assumes Cyborg or VTOL droids are not Command types!
        factoryType = 0 as libc::c_int as UBYTE;
        factoryInc = 0 as libc::c_int as UBYTE;
        while (factoryInc as libc::c_int) < 5 as libc::c_int {
            //check to see if there is a factory
            if factoryNumFlag[player as usize][factoryType as usize] as
                   libc::c_int & mask as libc::c_int == mask as libc::c_int {
                inc = 0 as libc::c_int as UBYTE;
                while (inc as libc::c_int) < 20 as libc::c_int {
                    if !asProductionRun[factoryType as
                                            usize][factoryInc as
                                                       usize][inc as
                                                                  usize].psTemplate.is_null()
                       {
                        if (*asProductionRun[factoryType as
                                                 usize][factoryInc as
                                                            usize][inc as
                                                                       usize].psTemplate).droidType
                               as libc::c_uint ==
                               DROID_COMMAND as libc::c_int as libc::c_uint {
                            quantity =
                                (quantity as libc::c_int +
                                     (asProductionRun[factoryType as
                                                          usize][factoryInc as
                                                                     usize][inc
                                                                                as
                                                                                usize].quantity
                                          as libc::c_int -
                                          asProductionRun[factoryType as
                                                              usize][factoryInc
                                                                         as
                                                                         usize][inc
                                                                                    as
                                                                                    usize].built
                                              as libc::c_int)) as UBYTE
                        }
                    }
                    inc = inc.wrapping_add(1)
                }
            }
            mask = ((mask as libc::c_int) << 1 as libc::c_int) as UBYTE;
            factoryInc = factoryInc.wrapping_add(1)
        }
    }
    return quantity;
}
// Count number of factories assignable to a command droid.
//
#[no_mangle]
pub unsafe extern "C" fn countAssignableFactories(mut player: UBYTE,
                                                  mut factoryType: UWORD)
 -> UWORD {
    let mut factoryInc: UWORD = 0;
    let mut mask: UBYTE = 1 as libc::c_int as UBYTE;
    let mut quantity: UBYTE = 0 as libc::c_int as UBYTE;
    if player as libc::c_uint == selectedPlayer {
    } else {
        debug(LOG_ERROR,
              b"countAssignableFactories: should only be called for selectedPlayer\x00"
                  as *const u8 as *const libc::c_char);
    };
    if player as libc::c_uint == selectedPlayer {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8953 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 25],
                                        &[libc::c_char; 25]>(b"countAssignableFactories\x00")).as_ptr(),
              b"player == selectedPlayer\x00" as *const u8 as
                  *const libc::c_char);
    };
    factoryInc = 0 as libc::c_int as UWORD;
    while (factoryInc as libc::c_int) < 5 as libc::c_int {
        //check to see if there is a factory
        if factoryNumFlag[player as usize][factoryType as usize] as
               libc::c_int & mask as libc::c_int == mask as libc::c_int {
            quantity = quantity.wrapping_add(1)
        }
        mask = ((mask as libc::c_int) << 1 as libc::c_int) as UBYTE;
        factoryInc = factoryInc.wrapping_add(1)
    }
    return quantity as UWORD;
}
// check whether a factory of a certain number and type exists
#[no_mangle]
pub unsafe extern "C" fn checkFactoryExists(mut player: UDWORD,
                                            mut factoryType: UDWORD,
                                            mut inc: UDWORD) -> BOOL {
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"checkFactoryExists: invalid player\x00" as *const u8 as
                  *const libc::c_char);
    };
    if player < 8 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8972 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"checkFactoryExists\x00")).as_ptr(),
              b"player < MAX_PLAYERS\x00" as *const u8 as
                  *const libc::c_char);
    };
    if factoryType < 3 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"checkFactoryExists: invalid factoryType\x00" as *const u8 as
                  *const libc::c_char);
    };
    if factoryType < 3 as libc::c_int as libc::c_uint {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              8974 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 19],
                                        &[libc::c_char; 19]>(b"checkFactoryExists\x00")).as_ptr(),
              b"factoryType < NUM_FACTORY_TYPES\x00" as *const u8 as
                  *const libc::c_char);
    };
    return (factoryNumFlag[player as usize][factoryType as usize] as
                libc::c_int & (1 as libc::c_int) << inc != 0 as libc::c_int)
               as libc::c_int;
}
//check that delivery points haven't been put down in invalid location
#[no_mangle]
pub unsafe extern "C" fn checkDeliveryPoints(mut version: UDWORD) {
    let mut inc: UBYTE = 0;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    let mut psRepair: *mut REPAIR_FACILITY = 0 as *mut REPAIR_FACILITY;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    //find any factories
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_int) < 8 as libc::c_int {
        //don't bother checking selectedPlayer's - causes problems when try and use
        //validLocation since it finds that the DP is on itself! And validLocation
        //will have been called to put in down in the first place
        if inc as libc::c_uint != selectedPlayer {
            psStruct = apsStructLists[inc as usize];
            while !psStruct.is_null() {
                if StructIsFactory(psStruct) != 0 {
                    //check the DP
                    psFactory = (*psStruct).pFunctionality as *mut FACTORY;
                    if (*psFactory).psAssemblyPoint.is_null() {
                        //need to add one
                        if !(*psFactory).psAssemblyPoint.is_null() {
                        } else {
                            debug(LOG_ERROR,
                                  b"checkDeliveryPoints: no delivery point for factory\x00"
                                      as *const u8 as *const libc::c_char);
                        };
                        if !(*psFactory).psAssemblyPoint.is_null() {
                        } else {
                            debug(LOG_ERROR,
                                  b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                                      *const u8 as *const libc::c_char,
                                  b"structure.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  9007 as libc::c_int,
                                  (*::std::mem::transmute::<&[u8; 20],
                                                            &[libc::c_char; 20]>(b"checkDeliveryPoints\x00")).as_ptr(),
                                  b"psFactory->psAssemblyPoint != NULL\x00" as
                                      *const u8 as *const libc::c_char);
                        };
                    } else {
                        setAssemblyPoint((*psFactory).psAssemblyPoint,
                                         (*(*psFactory).psAssemblyPoint).coords.x
                                             as UDWORD,
                                         (*(*psFactory).psAssemblyPoint).coords.y
                                             as UDWORD, inc as UDWORD,
                                         1 as libc::c_int);
                    }
                } else if (*(*psStruct).pStructureType).type_0 ==
                              REF_REPAIR_FACILITY as libc::c_int as
                                  libc::c_uint {
                    psRepair =
                        (*psStruct).pFunctionality as *mut REPAIR_FACILITY;
                    if (*psRepair).psDeliveryPoint.is_null() {
                        //need to add one
                        if version >= 19 as libc::c_int as libc::c_uint {
                            if !(*psRepair).psDeliveryPoint.is_null() {
                            } else {
                                debug(LOG_ERROR,
                                      b"checkDeliveryPoints: no delivery point for repair facility\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                            if !(*psRepair).psDeliveryPoint.is_null() {
                            } else {
                                debug(LOG_ERROR,
                                      b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"structure.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      9023 as libc::c_int,
                                      (*::std::mem::transmute::<&[u8; 20],
                                                                &[libc::c_char; 20]>(b"checkDeliveryPoints\x00")).as_ptr(),
                                      b"psRepair->psDeliveryPoint != NULL\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                            };
                        } else {
                            // add an assembly point
                            if createFlagPosition(&mut (*psRepair).psDeliveryPoint,
                                                  (*psStruct).player as
                                                      UDWORD) == 0 {
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"checkDeliveryPoints: unable to create new delivery point for repair facility\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                                };
                                if 0 as libc::c_int != 0 {
                                } else {
                                    debug(LOG_ERROR,
                                          b"Assert in Warzone: %s:%d : %s (%s)\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"structure.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          9030 as libc::c_int,
                                          (*::std::mem::transmute::<&[u8; 20],
                                                                    &[libc::c_char; 20]>(b"checkDeliveryPoints\x00")).as_ptr(),
                                          b"FALSE\x00" as *const u8 as
                                              *const libc::c_char);
                                };
                                return
                            }
                            addFlagPosition((*psRepair).psDeliveryPoint);
                            setFlagPositionInc(psRepair as *mut libc::c_void,
                                               (*psStruct).player as UDWORD,
                                               3 as libc::c_int as UBYTE);
                            //initialise the assembly point position
                            x =
                                ((*psStruct).x as libc::c_int +
                                     256 as libc::c_int >> 7 as libc::c_int)
                                    as UDWORD;
                            y =
                                ((*psStruct).y as libc::c_int +
                                     256 as libc::c_int >> 7 as libc::c_int)
                                    as UDWORD;
                            // Belt and braces - shouldn't be able to build too near edge
						    //getNearestBestValidTile(&x,&y);
                            setAssemblyPoint((*psRepair).psDeliveryPoint,
                                             x << 7 as libc::c_int,
                                             y << 7 as libc::c_int,
                                             inc as UDWORD, 1 as libc::c_int);
                        }
                    } else {
                        //check existing one
                        setAssemblyPoint((*psRepair).psDeliveryPoint,
                                         (*(*psRepair).psDeliveryPoint).coords.x
                                             as UDWORD,
                                         (*(*psRepair).psDeliveryPoint).coords.y
                                             as UDWORD, inc as UDWORD,
                                         1 as libc::c_int);
                    }
                }
                psStruct = (*psStruct).psNext
            }
        }
        inc = inc.wrapping_add(1)
    };
}
//adjust the loop quantity for this factory
#[no_mangle]
pub unsafe extern "C" fn factoryLoopAdjust(mut psStruct: *mut STRUCTURE,
                                           mut add: BOOL) {
    let mut psFactory: *mut FACTORY = 0 as *mut FACTORY;
    if StructIsFactory(psStruct) != 0 {
    } else {
        debug(LOG_ERROR,
              b"factoryLoopAdjust: structure is not a factory\x00" as
                  *const u8 as *const libc::c_char);
    };
    if StructIsFactory(psStruct) != 0 {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              9062 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"factoryLoopAdjust\x00")).as_ptr(),
              b"StructIsFactory(psStruct)\x00" as *const u8 as
                  *const libc::c_char);
    };
    if (*psStruct).player as libc::c_uint == selectedPlayer {
    } else {
        debug(LOG_ERROR,
              b"factoryLoopAdjust: should only be called for selectedPlayer\x00"
                  as *const u8 as *const libc::c_char);
    };
    if (*psStruct).player as libc::c_uint == selectedPlayer {
    } else {
        debug(LOG_ERROR,
              b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8 as
                  *const libc::c_char,
              b"structure.c\x00" as *const u8 as *const libc::c_char,
              9064 as libc::c_int,
              (*::std::mem::transmute::<&[u8; 18],
                                        &[libc::c_char; 18]>(b"factoryLoopAdjust\x00")).as_ptr(),
              b"psStruct->player == selectedPlayer\x00" as *const u8 as
                  *const libc::c_char);
    };
    psFactory = (*psStruct).pFunctionality as *mut FACTORY;
    if add != 0 {
        //check for wrapping to infinite production
        if (*psFactory).quantity as libc::c_int == 9 as libc::c_int {
            (*psFactory).quantity = 0 as libc::c_int as UBYTE
        } else {
            //increment the count
            (*psFactory).quantity = (*psFactory).quantity.wrapping_add(1);
            //check for limit - this caters for when on infinite production and want to wrap around
            if (*psFactory).quantity as libc::c_int > 9 as libc::c_int {
                (*psFactory).quantity = 9 as libc::c_int as UBYTE
            }
        }
    } else if (*psFactory).quantity as libc::c_int == 0 as libc::c_int {
        (*psFactory).quantity = 9 as libc::c_int as UBYTE
    } else { (*psFactory).quantity = (*psFactory).quantity.wrapping_sub(1) };
}
//decrement the count
/*This function is called after a game is loaded so that any resource extractors
that are active are initialised for when to start*/
#[no_mangle]
pub unsafe extern "C" fn checkResExtractorsActive() {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut inc: UBYTE = 0;
    inc = 0 as libc::c_int as UBYTE;
    while (inc as libc::c_int) < 8 as libc::c_int {
        psStruct = apsStructLists[inc as usize];
        while !psStruct.is_null() {
            if (*(*psStruct).pStructureType).type_0 ==
                   REF_RESOURCE_EXTRACTOR as libc::c_int as libc::c_uint {
                if (*((*psStruct).pFunctionality as
                          *mut RES_EXTRACTOR)).active != 0 {
                    (*((*psStruct).pFunctionality as
                           *mut RES_EXTRACTOR)).timeLastUpdated = gameTime
                }
            }
            psStruct = (*psStruct).psNext
        }
        inc = inc.wrapping_add(1)
    };
}
/*Used for determining how much of the structure to draw as being built or demolished*/
#[no_mangle]
pub unsafe extern "C" fn structHeightScale(mut psStruct: *mut STRUCTURE)
 -> FRACT {
    let mut retVal: FRACT = 0.;
    retVal =
        (*psStruct).currentBuildPts as FRACT /
            (*(*psStruct).pStructureType).buildPoints as libc::c_float;
    if retVal < 0.05f32 { retVal = 0.05f32 }
    return retVal;
}
/*compares the structure sensor type with the droid weapon type to see if the
FIRE_SUPPORT order can be assigned*/
#[no_mangle]
pub unsafe extern "C" fn structSensorDroidWeapon(mut psStruct: *mut STRUCTURE,
                                                 mut psDroid: *mut DROID)
 -> BOOL {
    //Standard Sensor Tower + indirect weapon droid (non VTOL)
	//else if (structStandardSensor(psStruct) AND (psDroid->numWeaps AND
    if structStandardSensor(psStruct) != 0 &&
           ((*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                0 as libc::c_int as libc::c_uint &&
                proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                        libc::c_int
                                                                        as
                                                                        usize].nStat
                                                     as isize)) == 0) &&
           vtolDroid(psDroid) == 0 {
        return 1 as libc::c_int
    } else {
        //CB Sensor Tower + indirect weapon droid (non VTOL)
	//if (structCBSensor(psStruct) AND (psDroid->numWeaps AND
        if structCBSensor(psStruct) != 0 &&
               ((*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                    0 as libc::c_int as libc::c_uint &&
                    proj_Direct(asWeaponStats.offset((*psDroid).asWeaps[0 as
                                                                            libc::c_int
                                                                            as
                                                                            usize].nStat
                                                         as isize)) == 0) &&
               vtolDroid(psDroid) == 0 {
            return 1 as libc::c_int
        } else {
            //VTOL Intercept Sensor Tower + any weapon VTOL droid
	//else if (structVTOLSensor(psStruct) AND psDroid->numWeaps AND
            if structVTOLSensor(psStruct) != 0 &&
                   (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                       0 as libc::c_int as libc::c_uint &&
                   vtolDroid(psDroid) != 0 {
                return 1 as libc::c_int
            } else {
                //VTOL CB Sensor Tower + any weapon VTOL droid
	//else if (structVTOLCBSensor(psStruct) AND psDroid->numWeaps AND
                if structVTOLCBSensor(psStruct) != 0 &&
                       (*psDroid).asWeaps[0 as libc::c_int as usize].nStat >
                           0 as libc::c_int as libc::c_uint &&
                       vtolDroid(psDroid) != 0 {
                    return 1 as libc::c_int
                }
            }
        }
    }
    //case not matched
    return 0 as libc::c_int;
}
/*checks if the structure has a Counter Battery sensor attached - returns
TRUE if it has*/
#[no_mangle]
pub unsafe extern "C" fn structCBSensor(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if !(*(*psStruct).pStructureType).pSensor.is_null() {
        //Super Sensor works as any type
        if ((*(*(*psStruct).pStructureType).pSensor).type_0 as libc::c_uint ==
                INDIRECT_CB_SENSOR as libc::c_int as libc::c_uint ||
                (*(*(*psStruct).pStructureType).pSensor).type_0 as
                    libc::c_uint ==
                    SUPER_SENSOR as libc::c_int as libc::c_uint) &&
               (*(*(*psStruct).pStructureType).pSensor).location ==
                   LOC_TURRET as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*checks if the structure has a Standard Turret sensor attached - returns
TRUE if it has*/
#[no_mangle]
pub unsafe extern "C" fn structStandardSensor(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if !(*(*psStruct).pStructureType).pSensor.is_null() {
        /*Super Sensor works as any type*/
        if ((*(*(*psStruct).pStructureType).pSensor).type_0 as libc::c_uint ==
                STANDARD_SENSOR as libc::c_int as libc::c_uint ||
                (*(*(*psStruct).pStructureType).pSensor).type_0 as
                    libc::c_uint ==
                    SUPER_SENSOR as libc::c_int as libc::c_uint) &&
               (*(*(*psStruct).pStructureType).pSensor).location ==
                   LOC_TURRET as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*checks if the structure has a VTOL Intercept sensor attached - returns
TRUE if it has*/
#[no_mangle]
pub unsafe extern "C" fn structVTOLSensor(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if !(*(*psStruct).pStructureType).pSensor.is_null() {
        //Super Sensor works as any type
        if ((*(*(*psStruct).pStructureType).pSensor).type_0 as libc::c_uint ==
                VTOL_INTERCEPT_SENSOR as libc::c_int as libc::c_uint ||
                (*(*(*psStruct).pStructureType).pSensor).type_0 as
                    libc::c_uint ==
                    SUPER_SENSOR as libc::c_int as libc::c_uint) &&
               (*(*(*psStruct).pStructureType).pSensor).location ==
                   LOC_TURRET as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
/*checks if the structure has a VTOL Counter Battery sensor attached - returns
TRUE if it has*/
#[no_mangle]
pub unsafe extern "C" fn structVTOLCBSensor(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if !(*(*psStruct).pStructureType).pSensor.is_null() {
        //Super Sensor works as any type
        if ((*(*(*psStruct).pStructureType).pSensor).type_0 as libc::c_uint ==
                VTOL_CB_SENSOR as libc::c_int as libc::c_uint ||
                (*(*(*psStruct).pStructureType).pSensor).type_0 as
                    libc::c_uint ==
                    SUPER_SENSOR as libc::c_int as libc::c_uint) &&
               (*(*(*psStruct).pStructureType).pSensor).location ==
                   LOC_TURRET as libc::c_int as libc::c_uint {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// check whether a rearm pad is clear
#[no_mangle]
pub unsafe extern "C" fn clearRearmPad(mut psStruct: *mut STRUCTURE) -> BOOL {
    if (*(*psStruct).pStructureType).type_0 ==
           REF_REARM_PAD as libc::c_int as libc::c_uint &&
           ((*((*psStruct).pFunctionality as *mut REARM_PAD)).psObj.is_null()
                ||
                vtolHappy((*((*psStruct).pFunctionality as
                                 *mut REARM_PAD)).psObj as *mut DROID) != 0) {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
// return the nearest rearm pad
// if bClear is true it tries to find the nearest clear rearm pad in
// the same cluster as psTarget
// psTarget can be NULL
#[no_mangle]
pub unsafe extern "C" fn findNearestReArmPad(mut psDroid: *mut DROID,
                                             mut psTarget: *mut STRUCTURE,
                                             mut bClear: BOOL)
 -> *mut STRUCTURE {
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psNearest: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psTotallyClear: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut xdiff: SDWORD = 0;
    let mut ydiff: SDWORD = 0;
    let mut mindist: SDWORD = 0;
    let mut currdist: SDWORD = 0;
    let mut totallyDist: SDWORD = 0;
    let mut cx: SDWORD = 0;
    let mut cy: SDWORD = 0;
    if !psTarget.is_null() {
        cx = (*psTarget).x as SDWORD;
        cy = (*psTarget).y as SDWORD
    } else { cx = (*psDroid).x as SDWORD; cy = (*psDroid).y as SDWORD }
    mindist = 0x7fffffff as libc::c_int;
    totallyDist = 0x7fffffff as libc::c_int;
    psNearest = 0 as *mut STRUCTURE;
    psTotallyClear = 0 as *mut STRUCTURE;
    psStruct = apsStructLists[(*psDroid).player as usize];
    while !psStruct.is_null() {
        if (*(*psStruct).pStructureType).type_0 ==
               REF_REARM_PAD as libc::c_int as libc::c_uint &&
               (psTarget.is_null() ||
                    (*psTarget).cluster as libc::c_int ==
                        (*psStruct).cluster as libc::c_int) &&
               (bClear == 0 || clearRearmPad(psStruct) != 0) {
            xdiff = (*psStruct).x as SDWORD - cx;
            ydiff = (*psStruct).y as SDWORD - cy;
            currdist = xdiff * xdiff + ydiff * ydiff;
            if bClear != 0 && vtolOnRearmPad(psStruct, psDroid) == 0 {
                if currdist < totallyDist {
                    totallyDist = currdist;
                    psTotallyClear = psStruct
                }
            } else if currdist < mindist {
                mindist = currdist;
                psNearest = psStruct
            }
        }
        psStruct = (*psStruct).psNext
    }
    if bClear != 0 && !psTotallyClear.is_null() { psNearest = psTotallyClear }
    return psNearest;
}
// clear a rearm pad for a droid to land on it
#[no_mangle]
pub unsafe extern "C" fn ensureRearmPadClear(mut psStruct: *mut STRUCTURE,
                                             mut psDroid: *mut DROID) {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    tx = (*psStruct).x as libc::c_int >> 7 as libc::c_int;
    ty = (*psStruct).y as libc::c_int >> 7 as libc::c_int;
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if psCurr != psDroid &&
               (*psCurr).x as libc::c_int >> 7 as libc::c_int == tx &&
               (*psCurr).y as libc::c_int >> 7 as libc::c_int == ty &&
               vtolDroid(psCurr) != 0 {
            actionDroidObj(psCurr, DACTION_CLEARREARMPAD,
                           psStruct as *mut BASE_OBJECT);
        }
        psCurr = (*psCurr).psNext
    };
}
// return whether a rearm pad has a vtol on it
#[no_mangle]
pub unsafe extern "C" fn vtolOnRearmPad(mut psStruct: *mut STRUCTURE,
                                        mut psDroid: *mut DROID) -> BOOL {
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut tx: SDWORD = 0;
    let mut ty: SDWORD = 0;
    let mut found: BOOL = 0;
    tx = (*psStruct).x as libc::c_int >> 7 as libc::c_int;
    ty = (*psStruct).y as libc::c_int >> 7 as libc::c_int;
    found = 0 as libc::c_int;
    psCurr = apsDroidLists[(*psDroid).player as usize];
    while !psCurr.is_null() {
        if psCurr != psDroid &&
               (*psCurr).x as libc::c_int >> 7 as libc::c_int == tx &&
               (*psCurr).y as libc::c_int >> 7 as libc::c_int == ty {
            found = 1 as libc::c_int;
            break ;
        } else { psCurr = (*psCurr).psNext }
    }
    return found;
}
/* Just returns true if the structure's present body points aren't as high as the original*/
#[no_mangle]
pub unsafe extern "C" fn structIsDamaged(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if ((*psStruct).body as libc::c_uint) < structureBody(psStruct) {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
// give a structure from one player to another - used in Electronic Warfare
//returns pointer to the new structure
#[no_mangle]
pub unsafe extern "C" fn giftSingleStructure(mut psStructure: *mut STRUCTURE,
                                             mut attackPlayer: UBYTE,
                                             mut bFromScript: BOOL)
 -> *mut STRUCTURE {
    let mut psNewStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psStruct: *mut STRUCTURE = 0 as *mut STRUCTURE;
    let mut psCurr: *mut DROID = 0 as *mut DROID;
    let mut psType: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut psModule: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut x: UDWORD = 0;
    let mut y: UDWORD = 0;
    let mut capacity: UBYTE = 0 as libc::c_int as UBYTE;
    let mut originalPlayer: UBYTE = 0;
    let mut buildPoints: SWORD = 0 as libc::c_int as SWORD;
    let mut i: SWORD = 0;
    let mut bPowerOn: BOOL = 0;
    let mut direction: UWORD = 0;
    //this is not the case for EW in multiPlayer mode
    if bMultiPlayer == 0 {
        //added 'selectedPlayer == 0' to be able to test the game by changing player...
        //in this version of Warzone, the attack Player can NEVER be the selectedPlayer (unless from the script)
        if bFromScript == 0 &&
               selectedPlayer == 0 as libc::c_int as libc::c_uint &&
               attackPlayer as libc::c_uint == selectedPlayer {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"giftSingleStructure: EW attack by selectedPlayer on a structure\x00"
                          as *const u8 as *const libc::c_char);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      9409 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"giftSingleStructure\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as *mut STRUCTURE
        }
    }
    //don't want the hassle in multiplayer either
    //and now we do! - AB 13/05/99
    if bMultiPlayer != 0 {
        //certain structures give specific results - the rest swap sides!
        if electronicReward(psStructure, attackPlayer) == 0 {
            originalPlayer = (*psStructure).player;
            //tell the system the structure no longer exists
            removeStruct(psStructure, 0 as libc::c_int);
            // remove structure from one list
            removeStructureFromList(psStructure, apsStructLists.as_mut_ptr());
            (*psStructure).selected = 0 as libc::c_int as UBYTE;
            // change player id
            (*psStructure).player = attackPlayer;
            //restore the resistance value
            (*psStructure).resistance =
                structureResistance((*psStructure).pStructureType,
                                    (*psStructure).player) as UWORD as SWORD;
            // add to other list.
            addStructure(psStructure);
            //check through the 'attackPlayer' players list of droids to see if any are targetting it
            psCurr = apsDroidLists[attackPlayer as usize];
            while !psCurr.is_null() {
                if (*psCurr).psTarget == psStructure as *mut BASE_OBJECT ||
                       (*psCurr).psActionTarget ==
                           psStructure as *mut BASE_OBJECT {
                    orderDroid(psCurr, DORDER_STOP);
                }
                //check through order list
                i = 0 as libc::c_int as SWORD;
                while (i as libc::c_int) < (*psCurr).listSize {
                    if (*psCurr).asOrderList[i as usize].psOrderTarget ==
                           psStructure as *mut BASE_OBJECT as
                               *mut libc::c_void {
                        // move the rest of the list down
                        memmove(&mut *(*psCurr).asOrderList.as_mut_ptr().offset(i
                                                                                    as
                                                                                    isize)
                                    as *mut ORDER_LIST as *mut libc::c_void,
                                (&mut *(*psCurr).asOrderList.as_mut_ptr().offset(i
                                                                                     as
                                                                                     isize)
                                     as
                                     *mut ORDER_LIST).offset(1 as libc::c_int
                                                                 as isize) as
                                    *const libc::c_void,
                                (((*psCurr).listSize - i as libc::c_int) as
                                     libc::c_uint).wrapping_mul(::std::mem::size_of::<ORDER_LIST>()
                                                                    as
                                                                    libc::c_ulong));
                        //adjust list size
                        (*psCurr).listSize -= 1 as libc::c_int;
                        //initialise the empty last slot
                        memset((*psCurr).asOrderList.as_mut_ptr().offset((*psCurr).listSize
                                                                             as
                                                                             isize)
                                   as *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<ORDER_LIST>() as
                                   libc::c_ulong);
                    }
                    i += 1
                }
                psCurr = (*psCurr).psNext
            }
            //check through the 'attackPlayer' players list of structures to see if any are targetting it
            psStruct = apsStructLists[attackPlayer as usize];
            while !psStruct.is_null() {
                if (*psStruct).psTarget == psStructure as *mut BASE_OBJECT {
                    (*psStruct).psTarget = 0 as *mut BASE_OBJECT
                }
                psStruct = (*psStruct).psNext
            }
            //add back into cluster system
            clustNewStruct(psStructure);
            //add back into the grid system
            gridAddObject(psStructure as *mut BASE_OBJECT);
            if (*psStructure).status as libc::c_int == SS_BUILT as libc::c_int
               {
                buildingComplete(psStructure);
            }
            //since the structure isn't being rebuilt, the visibility code needs to be adjusted
            if attackPlayer as libc::c_uint == selectedPlayer {
                //make sure this structure is visible to selectedPlayer
                (*psStructure).visible[selectedPlayer as usize] =
                    0xff as libc::c_int as UBYTE;
                //make sure the tiles don't get drawn
                setStructTileDraw(psStructure);
            }
        }
        //ASSERT( FALSE,
        //    "giftSingleStructure: EW attack in multiplayer" );
        return 0 as *mut STRUCTURE
    }
    //save info about the structure
    psType = (*psStructure).pStructureType;
    x = (*psStructure).x as UDWORD;
    y = (*psStructure).y as UDWORD;
    direction = (*psStructure).direction;
    originalPlayer = (*psStructure).player;
    //save how complete the build process is
    if (*psStructure).status as libc::c_int == SS_BEING_BUILT as libc::c_int {
        buildPoints = (*psStructure).currentBuildPts
    }
    //check module not attached
    psModule = getModuleStat(psStructure);
    if !psModule.is_null() {
        match (*psType).type_0 {
            3 => {
                capacity =
                    (*((*psStructure).pFunctionality as
                           *mut POWER_GEN)).capacity as UBYTE
                //no default case cos don't care!
            }
            10 => {
                capacity =
                    (*((*psStructure).pFunctionality as
                           *mut RESEARCH_FACILITY)).capacity as UBYTE
            }
            1 | 17 => {
                capacity =
                    (*((*psStructure).pFunctionality as
                           *mut FACTORY)).capacity
            }
            _ => { }
        }
    }
    //get rid of the structure
    removeStruct(psStructure, 1 as libc::c_int);
    //make sure power is not used to build
    bPowerOn = powerCalculated;
    powerCalculated = 0 as libc::c_int;
    //build a new one for the attacking player - set last element to TRUE so it doesn't adjust x/y
    psNewStruct =
        buildStructure(psType, x, y, attackPlayer as UDWORD,
                       1 as libc::c_int);
    if !psNewStruct.is_null() {
        (*psNewStruct).direction = direction;
        if capacity != 0 {
            match (*psType).type_0 {
                3 | 10 => {
                    //build the module for powerGen and research
                    buildStructure(psModule, (*psNewStruct).x as UDWORD,
                                   (*psNewStruct).y as UDWORD,
                                   attackPlayer as UDWORD, 0 as libc::c_int);
                }
                1 | 17 => {
                    //build the appropriate number of modules
                    while capacity != 0 {
                        buildStructure(psModule, (*psNewStruct).x as UDWORD,
                                       (*psNewStruct).y as UDWORD,
                                       attackPlayer as UDWORD,
                                       0 as libc::c_int);
                        capacity = capacity.wrapping_sub(1)
                    }
                }
                _ => { }
            }
        }
        if buildPoints != 0 {
            (*psNewStruct).status = SS_BEING_BUILT as libc::c_int as UBYTE;
            (*psNewStruct).currentBuildPts = buildPoints
        } else {
            (*psNewStruct).status = SS_BUILT as libc::c_int as UBYTE;
            buildingComplete(psNewStruct);
        }
        if bMultiPlayer == 0 {
            //inform selectedPlayer that takeover has happened
            if originalPlayer as libc::c_uint == selectedPlayer {
                if wallDefenceStruct((*psNewStruct).pStructureType) != 0 {
                    audio_QueueTrackPos(ID_SOUND_NEXUS_DEFENCES_ABSORBED as
                                            libc::c_int,
                                        (*psNewStruct).x as SDWORD,
                                        (*psNewStruct).y as SDWORD,
                                        (*psNewStruct).z as SDWORD);
                } else {
                    audio_QueueTrackPos(ID_SOUND_NEXUS_STRUCTURE_ABSORBED as
                                            libc::c_int,
                                        (*psNewStruct).x as SDWORD,
                                        (*psNewStruct).y as SDWORD,
                                        (*psNewStruct).z as SDWORD);
                }
                //make sure this structure is visible to selectedPlayer if the structure used to be selectedPlayers'
                (*psNewStruct).visible[selectedPlayer as usize] =
                    0xff as libc::c_int as UBYTE;
                //make sure the tiles don't get drawn
                setStructTileDraw(psNewStruct);
            }
        }
    }
    powerCalculated = bPowerOn;
    return psNewStruct;
}
//static void	getNearestBestValidTile(UDWORD *x, UDWORD *y);
/* Code to have the structure's weapon assembly rock back upon firing */
unsafe extern "C" fn structUpdateRecoil(mut psStruct: *mut STRUCTURE) {
    let mut percent: UDWORD = 0;
    let mut recoil: UDWORD = 0;
    let mut fraction: FRACT = 0.;
    /* Check it's actually got a weapon */
    if (*psStruct).asWeaps[0 as libc::c_int as usize].nStat ==
           0 as libc::c_int as libc::c_uint {
        return
    }
    /* We have a weapon */
    if gameTime >
           (*psStruct).asWeaps[0 as libc::c_int as
                                   usize].lastFired.wrapping_add((1000 as
                                                                      libc::c_int
                                                                      /
                                                                      4 as
                                                                          libc::c_int)
                                                                     as
                                                                     libc::c_uint)
       {
        /* Recoil effect is over */
        (*psStruct).asWeaps[0 as libc::c_int as usize].recoilValue =
            0 as libc::c_int as UDWORD;
        return
    }
    /* Where should the assembly be? */
    percent =
        gameTime.wrapping_sub((*psStruct).asWeaps[0 as libc::c_int as
                                                      usize].lastFired).wrapping_mul(100
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint).wrapping_div((1000
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         /
                                                                                                                         4
                                                                                                                             as
                                                                                                                             libc::c_int)
                                                                                                                        as
                                                                                                                        libc::c_uint);
    /* Outward journey */
    if percent >= 50 as libc::c_int as libc::c_uint {
        recoil =
            (100 as libc::c_int as
                 libc::c_uint).wrapping_sub(percent).wrapping_div(5 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
    } else {
        /* Return journey */
        recoil = percent.wrapping_div(5 as libc::c_int as libc::c_uint)
    }
    fraction =
        (*asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int as
                                                       usize].nStat as
                                   isize)).recoilValue as FRACT /
            100 as libc::c_int as FRACT;
    recoil = (recoil as FRACT * fraction) as SDWORD as UDWORD;
    /* Put it into the weapon data */
    (*psStruct).asWeaps[0 as libc::c_int as usize].recoilValue = recoil;
}
/*checks that the structure stats have loaded up as expected - must be done after
all StructureStats parts have been loaded*/
#[no_mangle]
pub unsafe extern "C" fn checkStructureStats() -> BOOL {
    let mut structInc: UDWORD = 0;
    let mut inc: UDWORD = 0;
    structInc = 0 as libc::c_int as UDWORD;
    while structInc < numStructureStats {
        if (*asStructureStats.offset(structInc as isize)).numFuncs !=
               0 as libc::c_int as libc::c_uint {
            inc = 0 as libc::c_int as UDWORD;
            while inc <
                      (*asStructureStats.offset(structInc as isize)).numFuncs
                  {
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"checkStructureStats:                     Invalid function for structure %s\x00"
                              as *const u8 as *const libc::c_char,
                          (*asStructureStats.offset(structInc as
                                                        isize)).pName);
                };
                if 1 as libc::c_int != 0 {
                } else {
                    debug(LOG_ERROR,
                          b"Assert in Warzone: %s:%d : %s (%s)\x00" as
                              *const u8 as *const libc::c_char,
                          b"structure.c\x00" as *const u8 as
                              *const libc::c_char, 9672 as libc::c_int,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"checkStructureStats\x00")).as_ptr(),
                          b"PTRVALID(asStructureStats[structInc].asFuncList[inc], sizeof(FUNCTION *))\x00"
                              as *const u8 as *const libc::c_char);
                };
                inc = inc.wrapping_add(1)
            }
        } else if !(*asStructureStats.offset(structInc as
                                                 isize)).asFuncList.is_null()
         {
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"checkStructureStats:Invalid functions attached to structure %s\x00"
                          as *const u8 as *const libc::c_char,
                      (*asStructureStats.offset(structInc as isize)).pName);
            };
            if 0 as libc::c_int != 0 {
            } else {
                debug(LOG_ERROR,
                      b"Assert in Warzone: %s:%d : %s (%s)\x00" as *const u8
                          as *const libc::c_char,
                      b"structure.c\x00" as *const u8 as *const libc::c_char,
                      9682 as libc::c_int,
                      (*::std::mem::transmute::<&[u8; 20],
                                                &[libc::c_char; 20]>(b"checkStructureStats\x00")).as_ptr(),
                      b"FALSE\x00" as *const u8 as *const libc::c_char);
            };
            return 0 as libc::c_int
        }
        structInc = structInc.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/*returns the power cost to build this structure*/
#[no_mangle]
pub unsafe extern "C" fn structPowerToBuild(mut psStruct: *mut STRUCTURE)
 -> UDWORD {
    let mut psStat: *mut STRUCTURE_STATS = 0 as *mut STRUCTURE_STATS;
    let mut capacity: UBYTE = 0;
    //see if this structure can have a module attached
    psStat = getModuleStat(psStruct);
    if !psStat.is_null() {
        capacity = 0 as libc::c_int as UBYTE;
        //are we building the module or the base structure?
        match (*(*psStruct).pStructureType).type_0 {
            3 => {
                capacity =
                    (*((*psStruct).pFunctionality as *mut POWER_GEN)).capacity
                        as UBYTE
                //no default case cos don't care!
            }
            10 => {
                capacity =
                    (*((*psStruct).pFunctionality as
                           *mut RESEARCH_FACILITY)).capacity as UBYTE
            }
            1 | 17 => {
                capacity =
                    (*((*psStruct).pFunctionality as *mut FACTORY)).capacity
            }
            _ => { }
        }
        if capacity != 0 {
            //return the cost to build the module
            return (*psStat).powerToBuild
        } else {
            //no module attached so building the base structure
            return (*(*psStruct).pStructureType).powerToBuild
        }
    } else {
        //not a module structure, so power cost is the one associated with the stat
        return (*(*psStruct).pStructureType).powerToBuild
    };
}
//for MULTIPLAYER ONLY
//this adjusts the time the relevant action started if the building is attacked by EW weapon
unsafe extern "C" fn resetResistanceLag(mut psBuilding: *mut STRUCTURE) {
    if bMultiPlayer != 0 {
        let mut current_block_11: u64;
        match (*(*psBuilding).pStructureType).type_0 {
            10 => {
                let mut psResFacility: *mut RESEARCH_FACILITY =
                    (*psBuilding).pFunctionality as *mut RESEARCH_FACILITY;
                //if working on a topic
                if !(*psResFacility).psSubject.is_null() {
                    //adjust the start time for the current subject
                    if (*psResFacility).timeStarted !=
                           0 as libc::c_int as libc::c_uint {
                        (*psResFacility).timeStarted =
                            ((*psResFacility).timeStarted as
                                 libc::c_uint).wrapping_add(gameTime.wrapping_sub((*psBuilding).lastResistance))
                                as UDWORD as UDWORD
                    }
                }
                current_block_11 = 13698511929906709940;
                /*
            case REF_REPAIR_FACILITY: - this is catered for in the aiUpdateStructure function
            case REF_REARM_PAD: - this structure is taken over completely
            */
            //default: //do nothing
            }
            1 | 17 | 16 => { current_block_11 = 13698511929906709940; }
            _ => { current_block_11 = 12039483399334584727; }
        }
        match current_block_11 {
            13698511929906709940 => {
                let mut psFactory: *mut FACTORY =
                    (*psBuilding).pFunctionality as *mut FACTORY;
                //if working on a unit
                if !(*psFactory).psSubject.is_null() {
                    //adjust the start time for the current subject
                    if (*psFactory).timeStarted !=
                           0 as libc::c_int as libc::c_uint {
                        (*psFactory).timeStarted =
                            ((*psFactory).timeStarted as
                                 libc::c_uint).wrapping_add(gameTime.wrapping_sub((*psBuilding).lastResistance))
                                as UDWORD as UDWORD
                    }
                }
            }
            _ => { }
        }
    };
}
/*reveals all the terrain in the map*/
unsafe extern "C" fn revealAll(mut player: UBYTE) {
    let mut i: UWORD = 0;
    let mut j: UWORD = 0;
    let mut psTile: *mut MAPTILE = 0 as *mut MAPTILE;
    //reveal all tiles
    i = 0 as libc::c_int as UWORD;
    while (i as libc::c_uint) < mapWidth {
        j = 0 as libc::c_int as UWORD;
        while (j as libc::c_uint) < mapHeight {
            psTile = mapTile(i as UDWORD, j as UDWORD);
            (*psTile).tileVisBits =
                ((*psTile).tileVisBits as libc::c_int |
                     (1 as libc::c_int) << player as libc::c_int) as UBYTE;
            avInformOfChange(i as SDWORD, j as SDWORD);
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    };
    //the objects gets revealed in processVisibility()
}
/*checks the structure passed in is a Las Sat structure which is currently 
selected - returns TRUE if valid*/
/*checks the structure passed in is a Las Sat structure which is currently
selected - returns TRUE if valid*/
#[no_mangle]
pub unsafe extern "C" fn lasSatStructSelected(mut psStruct: *mut STRUCTURE)
 -> BOOL {
    if ((*psStruct).selected as libc::c_int != 0 ||
            bMultiPlayer != 0 &&
                isHumanPlayer((*psStruct).player as UDWORD) == 0) &&
           (*psStruct).asWeaps[0 as libc::c_int as usize].nStat != 0 &&
           (*asWeaponStats.offset((*psStruct).asWeaps[0 as libc::c_int as
                                                          usize].nStat as
                                      isize)).weaponSubClass as libc::c_uint
               == WSC_LAS_SAT as libc::c_int as libc::c_uint {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* ******************* demo stuff ************************/
/*
#ifdef DEMO

void printAvailStructs(void)
{
	UDWORD	stNum;

	for (stNum = 0; stNum<numStructureStats; stNum++)
	{
		if (apStructTypeLists[0][stNum] == AVAILABLE)
		{
			DBPRINTF(("%s available\n", asStructureStats[stNum].pName));
		}
	}
}

//function to create the structures necessary for the demo. Allocates the IMD models
BOOL demoStructs(void)
{
	UDWORD				count;
	STRUCTURE_STATS		*psBuilding = asStructureStats;
	STRUCTURE_STATS		*psNewStructure = asStructureStats + numStructureStats;

	//get the stats for a wall
	for (count=0; count < numStructureStats; count++, psBuilding++)
	{
		if (psBuilding->type == REF_WALL)
		{
			//create Corner Wall 1
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats, "Corner Wall 1", REF_CORNER1))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbcr1.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Corner Wall 2
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 1, "Corner Wall 2", REF_CORNER2))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbcr2.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Corner Wall 3
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 2, "Corner Wall 3", REF_CORNER3))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbcr3.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//Create Corner Wall 4
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 3, "Corner Wall 4", REF_CORNER4))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbcr4.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Gate1
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 4, "Gate 1", REF_GATE1))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbgt1.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Gate2
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 5, "Gate 2", REF_GATE2))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbgt2.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Gate3
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 6, "Gate 3", REF_GATE3))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbgt3.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Gate4
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 7, "Gate 4", REF_GATE4))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbgt4.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			break;
		}
	}

	psBuilding = asStructureStats;
	//get the stats for a defense structure
	for (count=0; count < numStructureStats; count++, psBuilding++)
	{
		if (strcmp(psBuilding->pName, "Gun Tower") == 0)
		{
			//create Corner Tower 1
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 8, "Corner Tower 1", REF_TOWER1))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbtw1.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Corner Tower 2
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 9, "Corner Tower 2", REF_TOWER2))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbtw2.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
		psNewStructure++;

			//create Corner Tower 3
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 10, "Corner Tower 3", REF_TOWER3))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbtw3.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;

			//create Corner Tower 4
			if (!createStructureStat(psBuilding, psNewStructure, REF_STRUCTURE_START +
				numStructureStats + 11, "Corner Tower 4", REF_TOWER4))
			{
				return FALSE;
			}
			//set the IMD for the structure
			psNewStructure->pIMD = (iIMDShape *) resGetData("IMD", "blbrbtw4.imd");
			if (psNewStructure->pIMD == NULL)
			{
				DBERROR(("Cannot find the structure IMD for record %s",
					psNewStructure->pName));
				return FALSE;
			}
			psNewStructure++;
			break;
		}
	}

	numStructureStats += NUM_DEMO_STRUCTS;

	return TRUE;
}

BOOL createStructureStat(STRUCTURE_STATS *psBuilding, STRUCTURE_STATS *psNewStructure,
						 UDWORD ref, STRING *pName, UDWORD type)
{
	UDWORD		i;

	memcpy(psNewStructure, psBuilding, sizeof(STRUCTURE_STATS));
	//change the name, ref and type
	psNewStructure->ref = ref;
	psNewStructure->type = type;

	//allocate storage for the name
	psNewStructure->pName = (STRING *) MALLOC(MAX_NAME_SIZE);
	if (psNewStructure->pName == NULL)
	{
		DBERROR(("Structure Stats Name - Out of memory"));
		return FALSE;
	}
	strcpy(psNewStructure->pName, pName);

	//allocate storage for the weapons and functions - if any
	if (psNewStructure->numWeaps > 0)
	{
		psNewStructure->asWeapList = (WEAPON_STATS **)MALLOC(psNewStructure->
			weaponSlots * sizeof(WEAPON_STATS*));
		if (psNewStructure->asWeapList == NULL)
		{
			DBERROR(("Out of memory assigning structure weapons"));
			return FALSE;
		}
		//copy the stats across
		for (i = 0; i < psNewStructure->numWeaps; i++)
		{
			psNewStructure->asWeapList[i] = psBuilding->asWeapList[i];
		}
	}
	//allocate storage for the functions - if any
	if (psNewStructure->numFuncs > 0)
	{
		psNewStructure->asFuncList = (FUNCTION **)MALLOC(psNewStructure->
			numFuncs * sizeof(FUNCTION*));
		if (psNewStructure->asFuncList == NULL)
		{
			DBERROR(("Out of memory assigning structure Functions"));
			return FALSE;
		}
		//copy the stats across
		for (i = 0; i < psNewStructure->numFuncs; i++)
		{
			psNewStructure->asFuncList[i] = psBuilding->asFuncList[i];
		}
	}
	return TRUE;
}

#endif
  */
